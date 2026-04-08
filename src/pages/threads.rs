use leptos::prelude::*;
use crate::models::Thread;

#[server(GetThreads, "/api")]
pub async fn get_threads() -> Result<Vec<Thread>, ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query_as!(Thread, "SELECT id, name FROM thread ORDER BY id")
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(GetThread, "/api")]
pub async fn get_thread(id: i64) -> Result<Option<Thread>, ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query_as!(Thread, "SELECT id, name FROM thread WHERE id = ?", id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(CreateThread, "/api")]
pub async fn create_thread(name: String) -> Result<i64, ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    let result = sqlx::query!("INSERT INTO thread (name) VALUES (?)", name)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(result.last_insert_rowid())
}

#[server(UpdateThread, "/api")]
pub async fn update_thread(id: i64, name: String) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query!("UPDATE thread SET name = ? WHERE id = ?", name, id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[server(DeleteThread, "/api")]
pub async fn delete_thread(id: i64) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query!("DELETE FROM thread WHERE id = ?", id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[component]
pub fn ThreadsPage() -> impl IntoView {
    let threads = Resource::new(|| (), |_| get_threads());
    let delete_action = ServerAction::<DeleteThread>::new();

    Effect::new(move |_| {
        if delete_action.value().read().is_some() {
            threads.refetch();
        }
    });

    view! {
        <div class="page">
            <div class="page-header">
                <h1>"Threads"</h1>
                <a href="/threads/new" class="btn btn-primary">"+ New Thread"</a>
            </div>
            <Suspense fallback=|| view! { <p class="loading">"Loading..."</p> }>
                {move || threads.get().map(|result| match result {
                    Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                    Ok(list) if list.is_empty() => view! { <p class="empty">"No threads yet."</p> }.into_any(),
                    Ok(list) => view! {
                        <table class="data-table">
                            <thead>
                                <tr><th>"ID"</th><th>"Name"</th><th>"Actions"</th></tr>
                            </thead>
                            <tbody>
                                {list.into_iter().map(|t| {
                                    let id = t.id;
                                    view! {
                                        <tr>
                                            <td>{t.id}</td>
                                            <td>
                                                <a href={format!("/threads/{}", id)} class="link">{t.name.clone()}</a>
                                            </td>
                                            <td class="actions">
                                                <a href={format!("/threads/{}", id)} class="btn btn-sm">"View"</a>
                                                " "
                                                <a href={format!("/threads/{}/edit", id)} class="btn btn-sm">"Edit"</a>
                                                " "
                                                <ActionForm action=delete_action attr:class="inline-form">
                                                    <input type="hidden" name="id" value={id}/>
                                                    <button type="submit" class="btn btn-sm btn-danger"
                                                        onclick="return confirm('Delete this thread?')">"Delete"</button>
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
pub fn NewThreadPage() -> impl IntoView {
    let create_action = ServerAction::<CreateThread>::new();
    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        if let Some(Ok(_)) = create_action.value().read().as_ref() {
            navigate("/threads", Default::default());
        }
    });

    view! {
        <div class="page">
            <div class="page-header">
                <a href="/threads" class="back-link">"← Threads"</a>
                <h1>"New Thread"</h1>
            </div>
            <ActionForm action=create_action attr:class="form-card">
                <div class="form-group">
                    <label for="name">"Name"</label>
                    <input type="text" id="name" name="name" class="form-control" required/>
                </div>
                <div class="form-actions">
                    <button type="submit" class="btn btn-primary">"Create Thread"</button>
                    <a href="/threads" class="btn btn-secondary">"Cancel"</a>
                </div>
            </ActionForm>
        </div>
    }
}

#[component]
pub fn ThreadPage() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    let id = move || params.read().get("id").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
    let thread = Resource::new(id, |id| get_thread(id));

    view! {
        <div class="page">
            <div class="page-header">
                <a href="/threads" class="back-link">"← Threads"</a>
                <h1>"Thread Detail"</h1>
            </div>
            <Suspense fallback=|| view! { <p class="loading">"Loading..."</p> }>
                {move || thread.get().map(|result| match result {
                    Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                    Ok(None) => view! { <p class="error">"Thread not found."</p> }.into_any(),
                    Ok(Some(t)) => {
                        let edit_href = format!("/threads/{}/edit", t.id);
                        view! {
                            <div class="detail-card">
                                <dl>
                                    <dt>"ID"</dt><dd>{t.id}</dd>
                                    <dt>"Name"</dt><dd>{t.name.clone()}</dd>
                                </dl>
                                <div class="form-actions">
                                    <a href={edit_href} class="btn btn-primary">"Edit"</a>
                                </div>
                            </div>
                        }.into_any()
                    }
                })}
            </Suspense>
        </div>
    }
}

#[component]
pub fn EditThreadPage() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    let id = move || params.read().get("id").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
    let thread = Resource::new(id, |id| get_thread(id));
    let update_action = ServerAction::<UpdateThread>::new();
    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        if let Some(Ok(_)) = update_action.value().read().as_ref() {
            navigate("/threads", Default::default());
        }
    });

    view! {
        <div class="page">
            <div class="page-header">
                <a href="/threads" class="back-link">"← Threads"</a>
                <h1>"Edit Thread"</h1>
            </div>
            <Suspense fallback=|| view! { <p class="loading">"Loading..."</p> }>
                {move || thread.get().map(|result| match result {
                    Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                    Ok(None) => view! { <p class="error">"Thread not found."</p> }.into_any(),
                    Ok(Some(t)) => {
                        let tid = t.id;
                        view! {
                            <ActionForm action=update_action attr:class="form-card">
                                <input type="hidden" name="id" value={tid}/>
                                <div class="form-group">
                                    <label for="name">"Name"</label>
                                    <input type="text" id="name" name="name" class="form-control" value={t.name.clone()} required/>
                                </div>
                                <div class="form-actions">
                                    <button type="submit" class="btn btn-primary">"Save Changes"</button>
                                    <a href="/threads" class="btn btn-secondary">"Cancel"</a>
                                </div>
                            </ActionForm>
                        }.into_any()
                    }
                })}
            </Suspense>
        </div>
    }
}
