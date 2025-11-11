#!/usr/bin/env bash
set -euo pipefail

# Helper function for running commands with status messages
run() {
    local description="$1"
    shift
    echo -n "[ RUN ] $description... "
    if "$@" >/dev/null 2>&1; then
        echo "OK"
    else
        echo "FAILED"
        echo "  ↳ Command: $*"
        exit 1
    fi
}

echo "=== Rebuild mixed workspaces ==="
run "Generating stubs for mixed workspaces" \
    find mixed/ -maxdepth 1 -mindepth 1 -type d -exec cargo run --bin stub_gen \;

echo "=== Syncing Python environment ==="
run "Running uv sync" uv sync

# If needed later:
# run "Rebuilding mixed with maturin" \
#     find mixed/ -maxdepth 1 -mindepth 1 -type d -exec maturin develop --release --uv -m {}/Cargo.toml \;

echo "=== Rust checks ==="
run "cargo check" cargo check
run "cargo fmt" cargo fmt --check
run "cargo test" cargo test
run "cargo clippy" cargo clippy --all-targets --all-features -- -D warnings

echo "=== Python checks ==="
# run "stubtest" uv run stubtest algo --ignore-missing-stub --ignore-disjoint-bases # Reason: https://github.com/Jij-Inc/pyo3-stub-gen?tab=readme-ov-file#known-limitation-nested-submodules
run "mypy" uv run mypy .
run "ruff" uv run ruff check --fix
run "pytest" uv run pytest

echo
echo "All tasks completed successfully."
