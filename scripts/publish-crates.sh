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

PUBLISH_DELAY="${PUBLISH_DELAY:-90}"

if [[ "${1:-}" == "--dry-run" ]]; then
  echo "Dry run: validate workspace and package the first crate (oratos-core)."
  echo "Dependent crates cannot be packaged until oratos-core exists on crates.io."
  cargo check --workspace --all-targets
  cargo publish -p oratos-core --dry-run --allow-dirty
  echo "Workspace check and oratos-core package validation passed."
  exit 0
fi

start="${PUBLISH_FROM:-}"
publishing=false
if [[ -z "${start}" ]]; then
  publishing=true
fi

for crate in "${CRATES[@]}"; do
  if [[ -n "${start}" && "${crate}" != "${start}" ]]; then
    continue
  fi
  if [[ -n "${start}" && "${crate}" == "${start}" ]]; then
    publishing=true
    start=""
  fi
  if [[ "${publishing}" != true ]]; then
    continue
  fi
  echo "==> ${crate}"
  cargo publish -p "$crate"
  echo "Waiting ${PUBLISH_DELAY}s for crates.io to index ${crate}..."
  sleep "${PUBLISH_DELAY}"
done

echo "Done."
