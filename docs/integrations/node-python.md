# Node and Python wrappers (v0.5)

Thin wrappers call the `oratos` CLI. No separate protocol in v0.5 — install the Rust binary first.

## Node (npm script)

```json
{
  "scripts": {
    "audit:seo": "oratos audit ./dist --format json --output reports/oratos.json",
    "audit:seo:strict": "oratos audit ./dist --strict --fail-under 85"
  }
}
```

## Python

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

## Future

Dedicated `@oratos/cli` and `oratos-py` packages may wrap the library crates directly after the API stabilizes in v0.3+.
