use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    StaticSegment, ParamSegment,
};

use crate::pages::{
    chapters::{ChaptersPage, ChapterPage},
    lines::{LinesPage, NewLinePage, LinePage, EditLinePage},
    threads::{ThreadsPage, NewThreadPage, ThreadPage, EditThreadPage},
    dependencies::{DependenciesPage, NewDependencyPage},
    characters::{CharactersPage, NewCharacterPage, CharacterPage, EditCharacterPage},
    character_to_lines::{CharacterToLinesPage, NewCharacterToLinePage},
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/eos-rust.css"/>
        <Title text="EOS – Story Manager"/>
        <Router>
            <nav class="navbar">
                <A href="/" attr:class="nav-brand">"📖 EOS"</A>
                <div class="nav-links">
                    <A href="/chapters" attr:class="nav-link">"Chapters"</A>
                    <A href="/lines" attr:class="nav-link">"Lines"</A>
                    <A href="/threads" attr:class="nav-link">"Threads"</A>
                    <A href="/dependencies" attr:class="nav-link">"Dependencies"</A>
                    <A href="/characters" attr:class="nav-link">"Characters"</A>
                    <A href="/character-lines" attr:class="nav-link">"Char ↔ Line"</A>
                </div>
            </nav>
            <main>
                <Routes fallback=|| view! { <div class="page"><h1>"404 – Page Not Found"</h1></div> }>
                    <Route path=StaticSegment("") view=HomePage/>

                    <Route path=StaticSegment("chapters") view=ChaptersPage/>
                    <Route path=(StaticSegment("chapters"), ParamSegment("id")) view=ChapterPage/>

                    <Route path=StaticSegment("lines") view=LinesPage/>
                    <Route path=(StaticSegment("lines"), StaticSegment("new")) view=NewLinePage/>
                    <Route path=(StaticSegment("lines"), ParamSegment("id")) view=LinePage/>
                    <Route path=(StaticSegment("lines"), ParamSegment("id"), StaticSegment("edit")) view=EditLinePage/>

                    <Route path=StaticSegment("threads") view=ThreadsPage/>
                    <Route path=(StaticSegment("threads"), StaticSegment("new")) view=NewThreadPage/>
                    <Route path=(StaticSegment("threads"), ParamSegment("id")) view=ThreadPage/>
                    <Route path=(StaticSegment("threads"), ParamSegment("id"), StaticSegment("edit")) view=EditThreadPage/>

                    <Route path=StaticSegment("dependencies") view=DependenciesPage/>
                    <Route path=(StaticSegment("dependencies"), StaticSegment("new")) view=NewDependencyPage/>

                    <Route path=StaticSegment("characters") view=CharactersPage/>
                    <Route path=(StaticSegment("characters"), StaticSegment("new")) view=NewCharacterPage/>
                    <Route path=(StaticSegment("characters"), ParamSegment("id")) view=CharacterPage/>
                    <Route path=(StaticSegment("characters"), ParamSegment("id"), StaticSegment("edit")) view=EditCharacterPage/>

                    <Route path=StaticSegment("character-lines") view=CharacterToLinesPage/>
                    <Route path=(StaticSegment("character-lines"), StaticSegment("new")) view=NewCharacterToLinePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="page home-page">
            <h1>"📖 EOS – Non-linear Story Manager"</h1>
            <p class="subtitle">"Manage your story's chapters, lines, characters and narrative threads."</p>
            <div class="entity-grid">
                <a href="/chapters" class="entity-card">
                    <div class="entity-icon">"📄"</div>
                    <div class="entity-name">"Chapters"</div>
                    <div class="entity-desc">"Story chapters and their content"</div>
                </a>
                <a href="/lines" class="entity-card">
                    <div class="entity-icon">"✍️"</div>
                    <div class="entity-name">"Lines"</div>
                    <div class="entity-desc">"Individual story lines with text and sequence"</div>
                </a>
                <a href="/threads" class="entity-card">
                    <div class="entity-icon">"🧵"</div>
                    <div class="entity-name">"Threads"</div>
                    <div class="entity-desc">"Narrative threads connecting chapters"</div>
                </a>
                <a href="/dependencies" class="entity-card">
                    <div class="entity-icon">"🔗"</div>
                    <div class="entity-name">"Dependencies"</div>
                    <div class="entity-desc">"Chapter ordering within threads"</div>
                </a>
                <a href="/characters" class="entity-card">
                    <div class="entity-icon">"🎭"</div>
                    <div class="entity-name">"Characters"</div>
                    <div class="entity-desc">"Story characters and their details"</div>
                </a>
                <a href="/character-lines" class="entity-card">
                    <div class="entity-icon">"🗣️"</div>
                    <div class="entity-name">"Char ↔ Line"</div>
                    <div class="entity-desc">"Which characters appear in which lines"</div>
                </a>
            </div>
        </div>
    }
}
