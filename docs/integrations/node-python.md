# Node and Python wrappers

Oratos is a native CLI. Language packages **ship or download that binary** — they do not reimplement audits.

See [Install](../install.md) for the full matrix.

## Python (PyPI)

```bash
pip install oratos
oratos audit ./dist --format json --output reports/oratos.json
oratos audit ./dist --strict --fail-under 85
```

Wheels are built with [maturin](https://www.maturin.rs/) (`bindings = "bin"`) from the root [`pyproject.toml`](../../pyproject.toml). Publish via [`.github/workflows/publish-pypi.yml`](../../.github/workflows/publish-pypi.yml).

```python
import subprocess
import sys

def run_oratos_audit(target: str = "./dist") -> int:
    return subprocess.call(
        ["oratos", "audit", target, "--format", "json", "--fail-under", "85"],
    )

if __name__ == "__main__":
    sys.exit(run_oratos_audit())
```

## Node (npm)

```bash
npm install --save-dev oratos
npx oratos audit ./dist --fail-under 85
```

Package sources: [`packaging/npm`](../../packaging/npm). On `postinstall`, the matching GitHub Release asset is downloaded into `vendor/`.

```json
{
  "scripts": {
    "audit:seo": "oratos audit ./dist --format json --output reports/oratos.json",
    "audit:seo:strict": "oratos audit ./dist --strict --fail-under 85"
  }
}
```

## Notes

- Prefer release binaries / language wrappers in CI; reserve `cargo install oratos` for Rust contributors.
- Override npm binary version with `ORATOS_VERSION=v0.3.0`.
