use leptos::prelude::*;
use crate::models::ChapterDependsOnChapter;

#[server(GetDependencies, "/api")]
pub async fn get_dependencies() -> Result<Vec<ChapterDependsOnChapter>, ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query_as!(
        ChapterDependsOnChapter,
        "SELECT parent_chapter_id, child_chapter_id, thread_id FROM chapter_depends_on_chapter ORDER BY thread_id, parent_chapter_id"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(CreateDependency, "/api")]
pub async fn create_dependency(
    parent_chapter_id: i64,
    child_chapter_id: i64,
    thread_id: i64,
) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query!(
        "INSERT OR IGNORE INTO chapter_depends_on_chapter (parent_chapter_id, child_chapter_id, thread_id) VALUES (?, ?, ?)",
        parent_chapter_id, child_chapter_id, thread_id
    )
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[server(DeleteDependency, "/api")]
pub async fn delete_dependency(
    parent_chapter_id: i64,
    child_chapter_id: i64,
    thread_id: i64,
) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query!(
        "DELETE FROM chapter_depends_on_chapter WHERE parent_chapter_id = ? AND child_chapter_id = ? AND thread_id = ?",
        parent_chapter_id, child_chapter_id, thread_id
    )
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[component]
pub fn DependenciesPage() -> impl IntoView {
    let deps = Resource::new(|| (), |_| get_dependencies());
    let delete_action = ServerAction::<DeleteDependency>::new();

    Effect::new(move |_| {
        if delete_action.value().read().is_some() {
            deps.refetch();
        }
    });

    view! {
        <div class="page">
            <div class="page-header">
                <h1>"Chapter Dependencies"</h1>
                <a href="/dependencies/new" class="btn btn-primary">"+ New Dependency"</a>
            </div>
            <Suspense fallback=|| view! { <p class="loading">"Loading..."</p> }>
                {move || deps.get().map(|result| match result {
                    Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                    Ok(list) if list.is_empty() => view! { <p class="empty">"No dependencies yet."</p> }.into_any(),
                    Ok(list) => view! {
                        <table class="data-table">
                            <thead>
                                <tr>
                                    <th>"Parent Chapter"</th>
                                    <th>"Child Chapter"</th>
                                    <th>"Thread"</th>
                                    <th>"Actions"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {list.into_iter().map(|d| {
                                    let p = d.parent_chapter_id;
                                    let c = d.child_chapter_id;
                                    let t = d.thread_id;
                                    view! {
                                        <tr>
                                            <td>
                                                <a href={format!("/chapters/{}", p)} class="link">
                                                    {format!("Chapter {}", p)}
                                                </a>
                                            </td>
                                            <td>
                                                <a href={format!("/chapters/{}", c)} class="link">
                                                    {format!("Chapter {}", c)}
                                                </a>
                                            </td>
                                            <td>
                                                <a href={format!("/threads/{}", t)} class="link">
                                                    {format!("Thread {}", t)}
                                                </a>
                                            </td>
                                            <td>
                                                <ActionForm action=delete_action attr:class="inline-form">
                                                    <input type="hidden" name="parent_chapter_id" value={p}/>
                                                    <input type="hidden" name="child_chapter_id" value={c}/>
                                                    <input type="hidden" name="thread_id" value={t}/>
                                                    <button type="submit" class="btn btn-sm btn-danger"
                                                        onclick="return confirm('Delete this dependency?')">"Delete"</button>
                                                </ActionForm>
                                            </td>
                                        </tr>
                                    }
                                }).collect_view()}
                            </tbody>
                        </table>
                    }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

#[component]
pub fn NewDependencyPage() -> impl IntoView {
    let create_action = ServerAction::<CreateDependency>::new();
    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        if let Some(Ok(_)) = create_action.value().read().as_ref() {
            navigate("/dependencies", Default::default());
        }
    });

    let chapters = Resource::new(|| (), |_| crate::pages::chapters::get_chapters());
    let threads = Resource::new(|| (), |_| crate::pages::threads::get_threads());

    view! {
        <div class="page">
            <div class="page-header">
                <a href="/dependencies" class="back-link">"← Dependencies"</a>
                <h1>"New Chapter Dependency"</h1>
            </div>
            <ActionForm action=create_action attr:class="form-card">
                <Suspense fallback=|| view! { <p class="loading">"Loading..."</p> }>
                    <div class="form-group">
                        <label for="parent_chapter_id">"Parent Chapter"</label>
                        {move || chapters.get().map(|result| match result {
                            Ok(list) => view! {
                                <select id="parent_chapter_id" name="parent_chapter_id" class="form-control" required>
                                    <option value="">"-- Select Parent Chapter --"</option>
                                    {list.into_iter().map(|c| view! {
                                        <option value={c.id}>{format!("Chapter {}", c.id)}</option>
                                    }).collect_view()}
                                </select>
                            }.into_any(),
                            Err(_) => view! { <input type="number" name="parent_chapter_id" class="form-control" required/> }.into_any(),
                        })}
                    </div>
                    <div class="form-group">
                        <label for="child_chapter_id">"Child Chapter"</label>
                        {move || chapters.get().map(|result| match result {
                            Ok(list) => view! {
                                <select id="child_chapter_id" name="child_chapter_id" class="form-control" required>
                                    <option value="">"-- Select Child Chapter --"</option>
                                    {list.into_iter().map(|c| view! {
                                        <option value={c.id}>{format!("Chapter {}", c.id)}</option>
                                    }).collect_view()}
                                </select>
                            }.into_any(),
                            Err(_) => view! { <input type="number" name="child_chapter_id" class="form-control" required/> }.into_any(),
                        })}
                    </div>
                    <div class="form-group">
                        <label for="thread_id">"Thread"</label>
                        {move || threads.get().map(|result| match result {
                            Ok(list) => view! {
                                <select id="thread_id" name="thread_id" class="form-control" required>
                                    <option value="">"-- Select Thread --"</option>
                                    {list.into_iter().map(|t| view! {
                                        <option value={t.id}>{t.name.clone()}</option>
                                    }).collect_view()}
                                </select>
                            }.into_any(),
                            Err(_) => view! { <input type="number" name="thread_id" class="form-control" required/> }.into_any(),
                        })}
                    </div>
                </Suspense>
                <div class="form-actions">
                    <button type="submit" class="btn btn-primary">"Create Dependency"</button>
                    <a href="/dependencies" class="btn btn-secondary">"Cancel"</a>
                </div>
            </ActionForm>
        </div>
    }
}
