#!/usr/bin/env bash
# Publish the oratos crate to crates.io.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if [[ "${1:-}" == "--dry-run" ]]; then
  echo "Dry run: validate workspace and package oratos."
  cargo check --workspace --all-targets
  cargo publish -p oratos --dry-run --allow-dirty
  echo "Workspace check and oratos package validation passed."
  exit 0
fi

echo "==> oratos"
cargo publish -p oratos

echo "Done."
