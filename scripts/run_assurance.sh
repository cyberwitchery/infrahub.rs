#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo-llvm-cov >/dev/null 2>&1; then
  echo "cargo-llvm-cov is required (cargo install cargo-llvm-cov)" >&2
  exit 1
fi

if [[ "${INFRAHUB_DOCS:-1}" == "1" ]]; then
  echo "building docs"
  cargo doc --workspace --all-features --no-deps --lib
else
  echo "skipping docs (INFRAHUB_DOCS=0)"
fi

echo "running coverage"
cargo llvm-cov --workspace --all-features --ignore-filename-regex 'src/bin/infrahub-codegen.rs' --fail-under-lines 80

if [[ -n "${INFRAHUB_TOKEN:-}" ]]; then
  echo "running client smoke tests"
  ./scripts/run_smoke.sh
else
  echo "skipping smoke tests (INFRAHUB_TOKEN not set)"
fi

echo "running static analysis"
./scripts/run_static.sh
