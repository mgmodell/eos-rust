#!/usr/bin/env bash
# .devcontainer/post-create.sh
# Runs once after the devcontainer is created. Sets up all tools needed to
# develop, build and test eos-rust inside VS Code.

set -euo pipefail

echo "==> Installing system dependencies..."
sudo apt-get update -qq
sudo apt-get install -y --no-install-recommends \
    libsqlite3-dev \
    sqlite3

echo "==> Adding wasm32 compilation target..."
rustup target add wasm32-unknown-unknown

echo "==> Installing cargo-leptos..."
cargo install cargo-leptos --locked

echo "==> Installing sqlx-cli (SQLite only, for offline query cache)..."
cargo install sqlx-cli --no-default-features --features sqlite --locked

echo "==> Installing end-to-end test dependencies (Playwright)..."
cd end2end
npm install
cd ..

echo ""
echo "✅  Dev environment ready!"
echo "   Run the app:  cargo leptos serve"
echo "   Regen sqlx:   cargo sqlx prepare -- --features ssr"
