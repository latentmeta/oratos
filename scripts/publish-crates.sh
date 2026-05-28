#!/usr/bin/env bash
# Publish Oratos workspace crates to crates.io in dependency order.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

CRATES=(
  oratos-core
  oratos-html
  oratos-report
  oratos-audit
  oratos-generate
  oratos
)

if [[ "${1:-}" == "--dry-run" ]]; then
  echo "Dry run: validate workspace and package the first crate (oratos-core)."
  echo "Dependent crates cannot be packaged until oratos-core exists on crates.io."
  cargo check --workspace --all-targets
  cargo publish -p oratos-core --dry-run --allow-dirty
  echo "Workspace check and oratos-core package validation passed."
  exit 0
fi

for crate in "${CRATES[@]}"; do
  echo "==> ${crate}"
  cargo publish -p "$crate"
  echo "Waiting for crates.io to index ${crate}..."
  sleep 45
done

echo "Done."
