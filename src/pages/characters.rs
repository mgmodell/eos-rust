use leptos::prelude::*;
use crate::models::Character;

#[server(GetCharacters, "/api")]
pub async fn get_characters() -> Result<Vec<Character>, ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query_as!(Character, "SELECT id, name FROM character ORDER BY id")
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(GetCharacter, "/api")]
pub async fn get_character(id: i64) -> Result<Option<Character>, ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query_as!(Character, "SELECT id, name FROM character WHERE id = ?", id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(CreateCharacter, "/api")]
pub async fn create_character(name: String) -> Result<i64, ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    let result = sqlx::query!("INSERT INTO character (name) VALUES (?)", name)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(result.last_insert_rowid())
}

#[server(UpdateCharacter, "/api")]
pub async fn update_character(id: i64, name: String) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query!("UPDATE character SET name = ? WHERE id = ?", name, id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[server(DeleteCharacter, "/api")]
pub async fn delete_character(id: i64) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query!("DELETE FROM character WHERE id = ?", id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[component]
pub fn CharactersPage() -> impl IntoView {
    let characters = Resource::new(|| (), |_| get_characters());
    let delete_action = ServerAction::<DeleteCharacter>::new();

    Effect::new(move |_| {
        if delete_action.value().read().is_some() {
            characters.refetch();
        }
    });

    view! {
        <div class="page">
            <div class="page-header">
                <h1>"Characters"</h1>
                <a href="/characters/new" class="btn btn-primary">"+ New Character"</a>
            </div>
            <Suspense fallback=|| view! { <p class="loading">"Loading..."</p> }>
                {move || characters.get().map(|result| match result {
                    Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                    Ok(list) if list.is_empty() => view! { <p class="empty">"No characters yet."</p> }.into_any(),
                    Ok(list) => view! {
                        <table class="data-table">
                            <thead>
                                <tr><th>"ID"</th><th>"Name"</th><th>"Actions"</th></tr>
                            </thead>
                            <tbody>
                                {list.into_iter().map(|c| {
                                    let id = c.id;
                                    view! {
                                        <tr>
                                            <td>{c.id}</td>
                                            <td>
                                                <a href={format!("/characters/{}", id)} class="link">{c.name.clone()}</a>
                                            </td>
                                            <td class="actions">
                                                <a href={format!("/characters/{}", id)} class="btn btn-sm">"View"</a>
                                                " "
                                                <a href={format!("/characters/{}/edit", id)} class="btn btn-sm">"Edit"</a>
                                                " "
                                                <ActionForm action=delete_action attr:class="inline-form">
                                                    <input type="hidden" name="id" value={id}/>
                                                    <button type="submit" class="btn btn-sm btn-danger"
                                                        onclick="return confirm('Delete this character?')">"Delete"</button>
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
pub fn NewCharacterPage() -> impl IntoView {
    let create_action = ServerAction::<CreateCharacter>::new();
    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        if let Some(Ok(_)) = create_action.value().read().as_ref() {
            navigate("/characters", Default::default());
        }
    });

    view! {
        <div class="page">
            <div class="page-header">
                <a href="/characters" class="back-link">"← Characters"</a>
                <h1>"New Character"</h1>
            </div>
            <ActionForm action=create_action attr:class="form-card">
                <div class="form-group">
                    <label for="name">"Name"</label>
                    <input type="text" id="name" name="name" class="form-control" required/>
                </div>
                <div class="form-actions">
                    <button type="submit" class="btn btn-primary">"Create Character"</button>
                    <a href="/characters" class="btn btn-secondary">"Cancel"</a>
                </div>
            </ActionForm>
        </div>
    }
}

#[component]
pub fn CharacterPage() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    let id = move || params.read().get("id").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
    let character = Resource::new(id, |id| get_character(id));

    view! {
        <div class="page">
            <div class="page-header">
                <a href="/characters" class="back-link">"← Characters"</a>
                <h1>"Character Detail"</h1>
            </div>
            <Suspense fallback=|| view! { <p class="loading">"Loading..."</p> }>
                {move || character.get().map(|result| match result {
                    Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                    Ok(None) => view! { <p class="error">"Character not found."</p> }.into_any(),
                    Ok(Some(c)) => {
                        let edit_href = format!("/characters/{}/edit", c.id);
                        view! {
                            <div class="detail-card">
                                <dl>
                                    <dt>"ID"</dt><dd>{c.id}</dd>
                                    <dt>"Name"</dt><dd>{c.name.clone()}</dd>
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
pub fn EditCharacterPage() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    let id = move || params.read().get("id").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
    let character = Resource::new(id, |id| get_character(id));
    let update_action = ServerAction::<UpdateCharacter>::new();
    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        if let Some(Ok(_)) = update_action.value().read().as_ref() {
            navigate("/characters", Default::default());
        }
    });

    view! {
        <div class="page">
            <div class="page-header">
                <a href="/characters" class="back-link">"← Characters"</a>
                <h1>"Edit Character"</h1>
            </div>
            <Suspense fallback=|| view! { <p class="loading">"Loading..."</p> }>
                {move || character.get().map(|result| match result {
                    Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                    Ok(None) => view! { <p class="error">"Character not found."</p> }.into_any(),
                    Ok(Some(c)) => {
                        let cid = c.id;
                        view! {
                            <ActionForm action=update_action attr:class="form-card">
                                <input type="hidden" name="id" value={cid}/>
                                <div class="form-group">
                                    <label for="name">"Name"</label>
                                    <input type="text" id="name" name="name" class="form-control" value={c.name.clone()} required/>
                                </div>
                                <div class="form-actions">
                                    <button type="submit" class="btn btn-primary">"Save Changes"</button>
                                    <a href="/characters" class="btn btn-secondary">"Cancel"</a>
                                </div>
                            </ActionForm>
                        }.into_any()
                    }
                })}
            </Suspense>
        </div>
    }
}
