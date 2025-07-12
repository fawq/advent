#!/bin/bash

set -xe

# Sync uv if any changes
uv sync

# Rebuild mixed workspaces
find mixed/ -maxdepth 1 -mindepth 1 -type d -exec maturin develop --release --uv -m {}/Cargo.toml \;

# Check all rust code (also mixed workspaces)
cargo check
cargo fmt
cargo test
cargo clippy

# Check python code (also mixed workspaces)
uv run mypy .
uv run ruff check
uv run pytest