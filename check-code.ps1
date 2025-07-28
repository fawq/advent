# Sync uv if any changes
uv sync

# Rebuild mixed workspaces
Get-ChildItem -Path mixed/ -Recurse -Depth 0 | Where-Object { $_.PSIsContainer } | ForEach-Object { 
    cargo run --bin stub_gen 
}
Get-ChildItem -Path mixed/ -Recurse -Depth 0 | Where-Object { $_.PSIsContainer } | ForEach-Object { 
    maturin develop --release --uv -m mixed/$_/Cargo.toml 
}

# Check all rust code (also mixed workspaces)
cargo --config win_config/win_rust_config.toml check
cargo --config win_config/win_rust_config.toml fmt
cargo --config win_config/win_rust_config.toml test
cargo --config win_config/win_rust_config.toml clippy

# Check python code (also mixed workspaces)
uv run mypy .
uv run ruff check --fix
uv run pytest