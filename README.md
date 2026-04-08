# eos-rust

> **Non-linear / non-deterministic story presenter**
>
> A full-stack web application for managing the building blocks of a branching story: chapters, lines of text, narrative threads, inter-chapter dependencies, and which characters appear in which lines.

![Home page](https://github.com/user-attachments/assets/1f144e80-251e-47c5-b10e-f43df0492f94)

---

## Table of contents

1. [Prerequisites](#prerequisites)
2. [Installation](#installation)
3. [Database setup](#database-setup)
4. [Running in development](#running-in-development)
5. [Building for production](#building-for-production)
6. [Usage guide](#usage-guide)
7. [Project structure](#project-structure)
8. [Development notes](#development-notes)
9. [End-to-end tests](#end-to-end-tests)

---

## Prerequisites

| Tool | Minimum version | Notes |
|------|----------------|-------|
| [Rust](https://rustup.rs/) | stable (1.75+) | Install via `rustup` |
| [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) | 0.3 | See below |
| [wasm32 target](https://rustwasm.github.io/) | – | Added automatically below |
| [Node.js](https://nodejs.org/) | 18+ | Required by cargo-leptos for CSS/JS tooling |
| [SQLite](https://www.sqlite.org/) | 3.x | Usually pre-installed on macOS/Linux; on Windows install the DLLs |

---

## Installation

### 1. Clone the repository

```bash
git clone https://github.com/mgmodell/eos-rust.git
cd eos-rust
```

### 2. Install the WASM compilation target

```bash
rustup target add wasm32-unknown-unknown
```

### 3. Install cargo-leptos

```bash
cargo install cargo-leptos --locked
```

> `cargo-leptos` orchestrates the dual compilation (server binary + WASM bundle) and the hot-reload dev server.

---

## Database setup

EOS uses **SQLite** as its database. No external database server is required — the database file is created automatically on first run.

The schema is applied automatically via [sqlx migrations](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli#usage) when the server starts. You do **not** need to run any manual migration commands.

The six tables that are created are:

| Table | Key columns |
|-------|------------|
| `chapter` | `id` |
| `line` | `id`, `body`, `sequence`, `chapter_id` |
| `thread` | `id`, `name` |
| `chapter_depends_on_chapter` | `parent_chapter_id`, `child_chapter_id`, `thread_id` |
| `character` | `id`, `name` |
| `character_to_line` | `character_id`, `line_id` |

The database file (`eos.db`) is created in the working directory from which you start the server and is excluded from version control by `.gitignore`.

---

## Running in development

```bash
cargo leptos serve
```

This command:

1. Compiles the **server binary** (Axum + SSR) with the `ssr` feature.
2. Compiles the **WASM bundle** (client-side hydration) with the `hydrate` feature.
3. Starts the server at **http://127.0.0.1:3000** with hot-reload on file changes (CSS, Rust source, assets).

The reload port for the WebSocket watch connection is **3001** (configured in `Cargo.toml` under `[package.metadata.leptos]`).

### Environment variables

| Variable | Default | Description |
|----------|---------|-------------|
| `LEPTOS_SITE_ADDR` | `127.0.0.1:3000` | Bind address for the HTTP server |
| `LEPTOS_ENV` | `DEV` | Set to `PROD` for production mode |
| `DATABASE_URL` | `sqlite:eos.db` | SQLite connection string (only needed when regenerating the `.sqlx` query cache) |

---

## Building for production

```bash
cargo leptos build --release
```

This produces:

- **`target/server/release/eos-rust`** — the self-contained server binary.
- **`target/site/`** — all static assets (WASM bundle, JS loader, CSS, public files).

To deploy, copy the binary and the `target/site/` directory to your server, preserving their relative paths, then run:

```bash
LEPTOS_ENV=PROD ./eos-rust
```

The server will create `eos.db` in the current directory on first launch and apply all migrations automatically.

---

## Usage guide

Open **http://127.0.0.1:3000** in your browser. The home page shows a grid of all six entities. The persistent navigation bar at the top links to each entity's list.

### Chapters

Chapters are the top-level containers for story content.

| Action | How |
|--------|-----|
| Create | Click **"+ New Chapter"** on the Chapters list page — chapters have no fields other than their auto-assigned ID. |
| View | Click a chapter's name link to see its detail page and the lines it contains. |
| Delete | Click **"Delete"** on the list row and confirm the prompt. Deleting a chapter cascades to its lines and dependencies. |

### Lines

Lines are individual pieces of story text that belong to a chapter.

| Action | How |
|--------|-----|
| Create | Click **"+ New Line"** → fill in **Body** (text content), **Sequence** (ordering within the chapter), and select a **Chapter**. |
| View | Click a line's **"View"** button to see its full detail. |
| Edit | Click **"Edit"** to modify the body, sequence, or chapter assignment. |
| Delete | Click **"Delete"** on the list row. |

### Threads

Threads are named narrative paths through the story (e.g. "Main Storyline", "Villain's Perspective").

| Action | How |
|--------|-----|
| Create | Click **"+ New Thread"** → enter a **Name**. |
| View | Click a thread's name to see its detail page. |
| Edit | Click **"Edit"** to rename the thread. |
| Delete | Click **"Delete"** on the list row. |

### Dependencies (Chapter → Chapter)

A dependency records that within a specific thread, one chapter must come before another.

| Action | How |
|--------|-----|
| Create | Click **"+ New Dependency"** → choose a **Parent Chapter**, **Child Chapter**, and the **Thread** they belong to. |
| Delete | Click **"Delete"** on the list row. |

> Dependencies do not have an edit page because the three FK columns form the composite primary key. To change a dependency, delete it and create a new one.

### Characters

Characters are the people (or entities) that appear in the story.

| Action | How |
|--------|-----|
| Create | Click **"+ New Character"** → enter a **Name**. |
| View | Click a character's name. |
| Edit | Click **"Edit"** to rename the character. |
| Delete | Click **"Delete"** on the list row. |

### Character ↔ Line mappings

These mappings record which characters appear in which lines.

| Action | How |
|--------|-----|
| Create | Click **"+ New Mapping"** → select a **Character** and a **Line** from the dropdowns. |
| Delete | Click **"Remove"** on the list row. |

---

## Project structure

```
eos-rust/
├── migrations/                   # SQL migrations (run automatically on startup)
│   └── 20240101000001_create_tables.sql
├── public/                       # Static assets copied into target/site/
├── src/
│   ├── app.rs                    # Root App component, router, navigation bar
│   ├── lib.rs                    # WASM hydration entry point
│   ├── main.rs                   # Axum server entry point, DB pool setup
│   ├── models.rs                 # Shared data model structs (Serialize + FromRow)
│   └── pages/
│       ├── mod.rs
│       ├── chapters.rs           # Chapter CRUD server functions + components
│       ├── lines.rs              # Line CRUD server functions + components
│       ├── threads.rs            # Thread CRUD server functions + components
│       ├── dependencies.rs       # ChapterDependsOnChapter CRUD
│       ├── characters.rs         # Character CRUD server functions + components
│       └── character_to_lines.rs # CharacterToLine CRUD server functions + components
├── style/
│   └── main.scss                 # Application stylesheet (compiled by cargo-leptos)
├── end2end/                      # Playwright end-to-end tests
│   └── tests/example.spec.ts
├── .sqlx/                        # sqlx offline query cache (committed to repo)
└── Cargo.toml                    # Workspace + cargo-leptos metadata
```

**Feature flags** (set automatically by `cargo-leptos`):

| Feature | Used when | Enables |
|---------|-----------|---------|
| `ssr` | Server binary | Axum, Tokio, sqlx, leptos_axum |
| `hydrate` | WASM bundle | wasm-bindgen, console_error_panic_hook |

---

## Development notes

### Modifying SQL queries

This project uses **sqlx offline mode** so the WASM build can succeed without a live database. The `.sqlx/` directory (committed to the repository) contains a cache of all prepared query metadata.

After adding or modifying any `sqlx::query!` / `sqlx::query_as!` call, regenerate the cache:

```bash
DATABASE_URL="sqlite:eos.db" cargo sqlx prepare -- --features ssr
```

Then commit the updated `.sqlx/` files.

### Hot reload

`cargo leptos serve` watches all Rust source files, SCSS, and public assets. Saving a file triggers an incremental recompile and the browser refreshes automatically via the WebSocket on port 3001.

### Checking both targets

To verify a change compiles for both the server and the WASM client without doing a full build:

```bash
# Server (SSR)
cargo check --features ssr

# WASM client
cargo check --target wasm32-unknown-unknown --features hydrate
```

---

## End-to-end tests

End-to-end tests use [Playwright](https://playwright.dev/) and live in `end2end/`.

```bash
# Install Playwright and its browsers (first time only)
cd end2end
npm install
npx playwright install

# Start the dev server in one terminal
cd ..
cargo leptos serve

# Run the tests in another terminal
cd end2end
npx playwright test
```

Or use the cargo-leptos shortcut (starts the server automatically):

```bash
cargo leptos end-to-end
```
