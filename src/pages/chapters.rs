use leptos::prelude::*;
use crate::models::Chapter;

#[server(GetChapters, "/api")]
pub async fn get_chapters() -> Result<Vec<Chapter>, ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    let chapters = sqlx::query_as!(Chapter, "SELECT id FROM chapter ORDER BY id")
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(chapters)
}

#[server(CreateChapter, "/api")]
pub async fn create_chapter() -> Result<i64, ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    let result = sqlx::query!("INSERT INTO chapter DEFAULT VALUES")
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(result.last_insert_rowid())
}

#[server(DeleteChapter, "/api")]
pub async fn delete_chapter(id: i64) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query!("DELETE FROM chapter WHERE id = ?", id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[component]
pub fn ChaptersPage() -> impl IntoView {
    let chapters = Resource::new(|| (), |_| get_chapters());
    let create_action = ServerAction::<CreateChapter>::new();
    let delete_action = ServerAction::<DeleteChapter>::new();

    Effect::new(move |_| {
        if create_action.value().read().is_some() {
            chapters.refetch();
        }
    });
    Effect::new(move |_| {
        if delete_action.value().read().is_some() {
            chapters.refetch();
        }
    });

    view! {
        <div class="page">
            <h1>"Chapters"</h1>
            <ActionForm action=create_action attr:class="inline-form">
                <button type="submit" class="btn btn-primary">"+ New Chapter"</button>
            </ActionForm>
            <Suspense fallback=|| view! { <p class="loading">"Loading..."</p> }>
                {move || chapters.get().map(|result| match result {
                    Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                    Ok(list) => view! {
                        <table class="data-table">
                            <thead>
                                <tr>
                                    <th>"ID"</th>
                                    <th>"Actions"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {list.into_iter().map(|chapter| {
                                    let id = chapter.id;
                                    view! {
                                        <tr>
                                            <td>
                                                <a href={format!("/chapters/{}", id)} class="link">
                                                    {format!("Chapter {}", id)}
                                                </a>
                                            </td>
                                            <td>
                                                <ActionForm action=delete_action attr:class="inline-form">
                                                    <input type="hidden" name="id" value={id}/>
                                                    <button type="submit" class="btn btn-danger"
                                                        onclick="return confirm('Delete chapter?')"
                                                    >"Delete"</button>
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

#[server(GetChapterLines, "/api")]
pub async fn get_chapter_lines(chapter_id: i64) -> Result<Vec<crate::models::Line>, ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    let lines = sqlx::query_as!(
        crate::models::Line,
        "SELECT id, body, sequence, chapter_id FROM line WHERE chapter_id = ? ORDER BY sequence",
        chapter_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(lines)
}

#[component]
pub fn ChapterPage() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    let id = move || {
        params.read().get("id").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0)
    };

    let lines = Resource::new(id, |id| get_chapter_lines(id));

    view! {
        <div class="page">
            <div class="page-header">
                <a href="/chapters" class="back-link">"← Chapters"</a>
                <h1>"Chapter " {id}</h1>
            </div>

            <section>
                <h2>"Lines in this Chapter"</h2>
                <a href={move || format!("/lines/new?chapter_id={}", id())} class="btn btn-primary">"+ Add Line"</a>
                <Suspense fallback=|| view! { <p class="loading">"Loading lines..."</p> }>
                    {move || lines.get().map(|result| match result {
                        Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                        Ok(list) if list.is_empty() => view! { <p class="empty">"No lines yet."</p> }.into_any(),
                        Ok(list) => view! {
                            <table class="data-table">
                                <thead>
                                    <tr>
                                        <th>"ID"</th>
                                        <th>"Sequence"</th>
                                        <th>"Body"</th>
                                        <th>"Actions"</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {list.into_iter().map(|line| view! {
                                        <tr>
                                            <td>{line.id}</td>
                                            <td>{line.sequence}</td>
                                            <td class="text-cell">{line.body.clone()}</td>
                                            <td>
                                                <a href={format!("/lines/{}", line.id)} class="btn btn-sm">"View"</a>
                                                " "
                                                <a href={format!("/lines/{}/edit", line.id)} class="btn btn-sm">"Edit"</a>
                                            </td>
                                        </tr>
                                    }).collect_view()}
                                </tbody>
                            </table>
                        }.into_any(),
                    })}
                </Suspense>
            </section>
        </div>
    }
}
