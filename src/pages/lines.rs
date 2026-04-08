use leptos::prelude::*;
use crate::models::Line;

#[server(GetLines, "/api")]
pub async fn get_lines() -> Result<Vec<Line>, ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query_as!(
        Line,
        "SELECT id, body, sequence, chapter_id FROM line ORDER BY chapter_id, sequence"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(GetLine, "/api")]
pub async fn get_line(id: i64) -> Result<Option<Line>, ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query_as!(
        Line,
        "SELECT id, body, sequence, chapter_id FROM line WHERE id = ?",
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(CreateLine, "/api")]
pub async fn create_line(body: String, sequence: i64, chapter_id: i64) -> Result<i64, ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    let result = sqlx::query!(
        "INSERT INTO line (body, sequence, chapter_id) VALUES (?, ?, ?)",
        body, sequence, chapter_id
    )
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(result.last_insert_rowid())
}

#[server(UpdateLine, "/api")]
pub async fn update_line(id: i64, body: String, sequence: i64, chapter_id: i64) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query!(
        "UPDATE line SET body = ?, sequence = ?, chapter_id = ? WHERE id = ?",
        body, sequence, chapter_id, id
    )
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[server(DeleteLine, "/api")]
pub async fn delete_line(id: i64) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query!("DELETE FROM line WHERE id = ?", id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[component]
pub fn LinesPage() -> impl IntoView {
    let lines = Resource::new(|| (), |_| get_lines());
    let delete_action = ServerAction::<DeleteLine>::new();

    Effect::new(move |_| {
        if delete_action.value().read().is_some() {
            lines.refetch();
        }
    });

    view! {
        <div class="page">
            <div class="page-header">
                <h1>"Lines"</h1>
                <a href="/lines/new" class="btn btn-primary">"+ New Line"</a>
            </div>
            <Suspense fallback=|| view! { <p class="loading">"Loading..."</p> }>
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
                                    <th>"Chapter"</th>
                                    <th>"Actions"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {list.into_iter().map(|line| {
                                    let id = line.id;
                                    view! {
                                        <tr>
                                            <td>{line.id}</td>
                                            <td>{line.sequence}</td>
                                            <td class="text-cell">{line.body.clone()}</td>
                                            <td>
                                                <a href={format!("/chapters/{}", line.chapter_id)} class="link">
                                                    {format!("Chapter {}", line.chapter_id)}
                                                </a>
                                            </td>
                                            <td class="actions">
                                                <a href={format!("/lines/{}", id)} class="btn btn-sm">"View"</a>
                                                " "
                                                <a href={format!("/lines/{}/edit", id)} class="btn btn-sm">"Edit"</a>
                                                " "
                                                <ActionForm action=delete_action attr:class="inline-form">
                                                    <input type="hidden" name="id" value={id}/>
                                                    <button type="submit" class="btn btn-sm btn-danger"
                                                        onclick="return confirm('Delete this line?')"
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

#[component]
pub fn NewLinePage() -> impl IntoView {
    let query = leptos_router::hooks::use_query_map();
    let prefill_chapter = move || {
        query.read().get("chapter_id").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0)
    };

    let create_action = ServerAction::<CreateLine>::new();
    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        if let Some(Ok(_)) = create_action.value().read().as_ref() {
            navigate("/lines", Default::default());
        }
    });

    let chapters = Resource::new(|| (), |_| crate::pages::chapters::get_chapters());

    view! {
        <div class="page">
            <div class="page-header">
                <a href="/lines" class="back-link">"← Lines"</a>
                <h1>"New Line"</h1>
            </div>
            <ActionForm action=create_action attr:class="form-card">
                <div class="form-group">
                    <label for="body">"Body"</label>
                    <textarea id="body" name="body" class="form-control" rows="4" required></textarea>
                </div>
                <div class="form-group">
                    <label for="sequence">"Sequence"</label>
                    <input type="number" id="sequence" name="sequence" class="form-control" value="0" required/>
                </div>
                <div class="form-group">
                    <label for="chapter_id">"Chapter"</label>
                    <Suspense fallback=|| view! { <select class="form-control"><option>"Loading..."</option></select> }>
                        {move || chapters.get().map(|result| {
                            let prefill = prefill_chapter();
                            match result {
                                Err(_) => view! { <input type="number" name="chapter_id" class="form-control" value={prefill}/> }.into_any(),
                                Ok(list) => view! {
                                    <select id="chapter_id" name="chapter_id" class="form-control" required>
                                        <option value="">"-- Select Chapter --"</option>
                                        {list.into_iter().map(|c| {
                                            let selected = c.id == prefill;
                                            view! {
                                                <option value={c.id} selected={selected}>
                                                    {format!("Chapter {}", c.id)}
                                                </option>
                                            }
                                        }).collect_view()}
                                    </select>
                                }.into_any(),
                            }
                        })}
                    </Suspense>
                </div>
                <div class="form-actions">
                    <button type="submit" class="btn btn-primary">"Create Line"</button>
                    <a href="/lines" class="btn btn-secondary">"Cancel"</a>
                </div>
            </ActionForm>
        </div>
    }
}

#[component]
pub fn LinePage() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    let id = move || params.read().get("id").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
    let line = Resource::new(id, |id| get_line(id));

    view! {
        <div class="page">
            <div class="page-header">
                <a href="/lines" class="back-link">"← Lines"</a>
                <h1>"Line Detail"</h1>
            </div>
            <Suspense fallback=|| view! { <p class="loading">"Loading..."</p> }>
                {move || line.get().map(|result| match result {
                    Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                    Ok(None) => view! { <p class="error">"Line not found."</p> }.into_any(),
                    Ok(Some(l)) => {
                        let edit_href = format!("/lines/{}/edit", l.id);
                        let chap_href = format!("/chapters/{}", l.chapter_id);
                        view! {
                            <div class="detail-card">
                                <dl>
                                    <dt>"ID"</dt><dd>{l.id}</dd>
                                    <dt>"Sequence"</dt><dd>{l.sequence}</dd>
                                    <dt>"Chapter"</dt><dd><a href={chap_href} class="link">{format!("Chapter {}", l.chapter_id)}</a></dd>
                                    <dt>"Body"</dt><dd class="text-cell">{l.body}</dd>
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
pub fn EditLinePage() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    let id = move || params.read().get("id").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
    let line = Resource::new(id, |id| get_line(id));
    let chapters = Resource::new(|| (), |_| crate::pages::chapters::get_chapters());
    let update_action = ServerAction::<UpdateLine>::new();
    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        if let Some(Ok(_)) = update_action.value().read().as_ref() {
            navigate("/lines", Default::default());
        }
    });

    view! {
        <div class="page">
            <div class="page-header">
                <a href="/lines" class="back-link">"← Lines"</a>
                <h1>"Edit Line"</h1>
            </div>
            <Suspense fallback=|| view! { <p class="loading">"Loading..."</p> }>
                {move || {
                    let line_data = line.get();
                    let chapters_data = chapters.get();
                    match (line_data, chapters_data) {
                        (Some(Ok(Some(l))), Some(Ok(chapter_list))) => {
                            let l = l.clone();
                            let chapter_list = chapter_list.clone();
                            let lid = l.id;
                            view! {
                                <ActionForm action=update_action attr:class="form-card">
                                    <input type="hidden" name="id" value={lid}/>
                                    <div class="form-group">
                                        <label for="body">"Body"</label>
                                        <textarea id="body" name="body" class="form-control" rows="4" required>
                                            {l.body.clone()}
                                        </textarea>
                                    </div>
                                    <div class="form-group">
                                        <label for="sequence">"Sequence"</label>
                                        <input type="number" id="sequence" name="sequence" class="form-control" value={l.sequence} required/>
                                    </div>
                                    <div class="form-group">
                                        <label for="chapter_id">"Chapter"</label>
                                        <select id="chapter_id" name="chapter_id" class="form-control" required>
                                            {chapter_list.into_iter().map(|c| {
                                                let selected = c.id == l.chapter_id;
                                                view! {
                                                    <option value={c.id} selected={selected}>
                                                        {format!("Chapter {}", c.id)}
                                                    </option>
                                                }
                                            }).collect_view()}
                                        </select>
                                    </div>
                                    <div class="form-actions">
                                        <button type="submit" class="btn btn-primary">"Save Changes"</button>
                                        <a href="/lines" class="btn btn-secondary">"Cancel"</a>
                                    </div>
                                </ActionForm>
                            }.into_any()
                        }
                        _ => view! { <p class="loading">"Loading..."</p> }.into_any(),
                    }
                }}
            </Suspense>
        </div>
    }
}
