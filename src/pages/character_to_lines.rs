use leptos::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CharacterToLineView {
    pub character_id: i64,
    pub line_id: i64,
    pub character_name: String,
    pub line_body: String,
}

#[server(GetCharacterToLines, "/api")]
pub async fn get_character_to_lines() -> Result<Vec<CharacterToLineView>, ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    let rows = sqlx::query!(
        r#"SELECT ctl.character_id, ctl.line_id, c.name as character_name, l.body as line_body
           FROM character_to_line ctl
           JOIN character c ON c.id = ctl.character_id
           JOIN line l ON l.id = ctl.line_id
           ORDER BY ctl.character_id, ctl.line_id"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(rows.into_iter().map(|r| CharacterToLineView {
        character_id: r.character_id,
        line_id: r.line_id,
        character_name: r.character_name,
        line_body: r.line_body,
    }).collect())
}

#[server(CreateCharacterToLine, "/api")]
pub async fn create_character_to_line(character_id: i64, line_id: i64) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query!(
        "INSERT OR IGNORE INTO character_to_line (character_id, line_id) VALUES (?, ?)",
        character_id, line_id
    )
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[server(DeleteCharacterToLine, "/api")]
pub async fn delete_character_to_line(character_id: i64, line_id: i64) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query!(
        "DELETE FROM character_to_line WHERE character_id = ? AND line_id = ?",
        character_id, line_id
    )
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[component]
pub fn CharacterToLinesPage() -> impl IntoView {
    let mappings = Resource::new(|| (), |_| get_character_to_lines());
    let delete_action = ServerAction::<DeleteCharacterToLine>::new();

    Effect::new(move |_| {
        if delete_action.value().read().is_some() {
            mappings.refetch();
        }
    });

    view! {
        <div class="page">
            <div class="page-header">
                <h1>"Character ↔ Line Mappings"</h1>
                <a href="/character-lines/new" class="btn btn-primary">"+ New Mapping"</a>
            </div>
            <Suspense fallback=|| view! { <p class="loading">"Loading..."</p> }>
                {move || mappings.get().map(|result| match result {
                    Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                    Ok(list) if list.is_empty() => view! { <p class="empty">"No mappings yet."</p> }.into_any(),
                    Ok(list) => view! {
                        <table class="data-table">
                            <thead>
                                <tr>
                                    <th>"Character"</th>
                                    <th>"Line"</th>
                                    <th>"Actions"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {list.into_iter().map(|m| {
                                    let cid = m.character_id;
                                    let lid = m.line_id;
                                    view! {
                                        <tr>
                                            <td>
                                                <a href={format!("/characters/{}", cid)} class="link">
                                                    {m.character_name.clone()}
                                                </a>
                                            </td>
                                            <td>
                                                <a href={format!("/lines/{}", lid)} class="link">
                                                    {format!("Line {} – {}", lid, &m.line_body.chars().take(40).collect::<String>())}
                                                </a>
                                            </td>
                                            <td>
                                                <ActionForm action=delete_action attr:class="inline-form">
                                                    <input type="hidden" name="character_id" value={cid}/>
                                                    <input type="hidden" name="line_id" value={lid}/>
                                                    <button type="submit" class="btn btn-sm btn-danger"
                                                        onclick="return confirm('Remove this mapping?')">"Remove"</button>
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
pub fn NewCharacterToLinePage() -> impl IntoView {
    let create_action = ServerAction::<CreateCharacterToLine>::new();
    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        if let Some(Ok(_)) = create_action.value().read().as_ref() {
            navigate("/character-lines", Default::default());
        }
    });

    let characters = Resource::new(|| (), |_| crate::pages::characters::get_characters());
    let lines = Resource::new(|| (), |_| crate::pages::lines::get_lines());

    view! {
        <div class="page">
            <div class="page-header">
                <a href="/character-lines" class="back-link">"← Mappings"</a>
                <h1>"New Character ↔ Line Mapping"</h1>
            </div>
            <ActionForm action=create_action attr:class="form-card">
                <Suspense fallback=|| view! { <p class="loading">"Loading..."</p> }>
                    <div class="form-group">
                        <label for="character_id">"Character"</label>
                        {move || characters.get().map(|result| match result {
                            Ok(list) => view! {
                                <select id="character_id" name="character_id" class="form-control" required>
                                    <option value="">"-- Select Character --"</option>
                                    {list.into_iter().map(|c| view! {
                                        <option value={c.id}>{c.name.clone()}</option>
                                    }).collect_view()}
                                </select>
                            }.into_any(),
                            Err(_) => view! { <input type="number" name="character_id" class="form-control" required/> }.into_any(),
                        })}
                    </div>
                    <div class="form-group">
                        <label for="line_id">"Line"</label>
                        {move || lines.get().map(|result| match result {
                            Ok(list) => view! {
                                <select id="line_id" name="line_id" class="form-control" required>
                                    <option value="">"-- Select Line --"</option>
                                    {list.into_iter().map(|l| view! {
                                        <option value={l.id}>
                                            {format!("[Ch.{}] #{} – {}", l.chapter_id, l.sequence,
                                                l.body.chars().take(40).collect::<String>())}
                                        </option>
                                    }).collect_view()}
                                </select>
                            }.into_any(),
                            Err(_) => view! { <input type="number" name="line_id" class="form-control" required/> }.into_any(),
                        })}
                    </div>
                </Suspense>
                <div class="form-actions">
                    <button type="submit" class="btn btn-primary">"Create Mapping"</button>
                    <a href="/character-lines" class="btn btn-secondary">"Cancel"</a>
                </div>
            </ActionForm>
        </div>
    }
}
