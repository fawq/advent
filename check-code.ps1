#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"
[Console]::OutputEncoding = [System.Text.UTF8Encoding]::UTF8

$projectRoot = Get-Location
$env:PYO3_PYTHON = "$projectRoot\.venv\Scripts\python.exe"

function Run($Description, $ScriptBlock) {
    Write-Host "[ RUN ] $Description... " -NoNewline

    # Temporarily disable PowerShell error trapping for native stderr output
    $oldErrPref = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'

    # Execute and capture output
    $output = & $ScriptBlock 2>&1 | Out-String
    $exitCode = $LASTEXITCODE

    # Restore original error preference
    $ErrorActionPreference = $oldErrPref

    if ($exitCode -eq 0) {
        Write-Host "OK"
    }
    else {
        Write-Host "FAILED"
        Write-Host "  ↳ Step: $Description"
        Write-Host "----- Command Output -----"
        Write-Host $output
        Write-Host "--------------------------"
        exit $exitCode
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
# Run "stubtest" { uv run stubtest algo --ignore-missing-stub --ignore-disjoint-bases } # Reason: https://github.com/Jij-Inc/pyo3-stub-gen?tab=readme-ov-file#known-limitation-nested-submodules
Run "mypy" { uv run mypy . }
Run "ruff" { uv run ruff check --fix }
Run "pytest" { uv run pytest }

Write-Host "`nAll tasks completed successfully."
