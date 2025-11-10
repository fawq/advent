#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

$projectRoot = Get-Location
$env:PYO3_PYTHON = "$projectRoot\.venv\Scripts\python.exe"


function Run($Description, $Command) {
    Write-Host "[ RUN ] $Description... " -NoNewline
    try {
        & $Command | Out-Null
        Write-Host "OK"
    }
    catch {
        Write-Host "FAILED"
        Write-Host "  ↳ Command: $Command"
        exit 1
    }
}

Write-Host "=== Rebuild mixed workspaces ==="
Run "Generating stubs for mixed workspaces" {
    Get-ChildItem mixed -Directory | ForEach-Object {
        cargo run --bin stub_gen
    }
}

Write-Host "=== Syncing Python environment ==="
Run "Running uv sync" { uv sync }

# If needed later:
# Run "Rebuilding mixed with maturin" {
#     Get-ChildItem mixed -Directory | ForEach-Object {
#         maturin develop --release --uv -m "$($_.FullName)/Cargo.toml"
#     }
# }

Write-Host "=== Rust checks ==="
Run "cargo check" { cargo check }
Run "cargo fmt" { cargo fmt --check }
Run "cargo test" { cargo test }
Run "cargo clippy" { cargo clippy --all-targets --all-features -- -D warnings }

Write-Host "=== Python checks ==="
Run "mypy" { uv run mypy . }
Run "stubtest" { uv run stubtest algo --ignore-missing-stub --ignore-disjoint-bases }
Run "ruff" { uv run ruff check --fix }
Run "pytest" { uv run pytest }

Write-Host "`n✅ All tasks completed successfully."
