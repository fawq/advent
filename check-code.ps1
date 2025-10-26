$projectRoot = Get-Location
$env:PYO3_PYTHON = "$projectRoot\.venv\Scripts\python.exe"


# Rebuild mixed workspaces
Get-ChildItem -Path mixed/ -Recurse -Depth 0 | Where-Object { $_.PSIsContainer } | ForEach-Object { 
    cargo run --bin stub_gen
}

# Sync uv if any changes
uv sync

# Rebuild mixed workspaces (not needed for now)
# Get-ChildItem -Path mixed/ -Recurse -Depth 0 | Where-Object { $_.PSIsContainer } | ForEach-Object { 
#     maturin develop --release --uv -m mixed/$_/Cargo.toml
# }

# Check all rust code (also mixed workspaces)
cargo check
cargo fmt
cargo test
cargo clippy

# Check python code (also mixed workspaces)
uv run mypy .
uv run stubtest algo --ignore-missing-stub --ignore-disjoint-bases
uv run ruff check --fix
uv run pytest