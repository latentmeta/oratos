# 07: CI/CD quality gates

## Mental model

Treat website visibility like a lint pass: deterministic checks that can fail builds when quality drops.

## Why it matters

Visibility regressions often ship silently: missing titles, broken internal links, or missing alt text. CI gates catch them early.

## Try it locally

```bash
oratos audit examples/static_site --fail-under 85
echo "exit code: $?"
```

Strict mode (fail on any warning or error):

```bash
oratos audit examples/static_site --strict
```

SARIF for artifacts:

```bash
oratos audit examples/static_site --format sarif --output oratos.sarif
```

## GitHub Actions

Copy [.github/workflows/oratos-audit-example.yml](../../.github/workflows/oratos-audit-example.yml) or see [ci.md](../ci.md).

Project config (v0.2) in `oratos.toml`:

```toml
[audit]
fail_under = 85.0
strict = false
ignore_rules = ["seo.missing-twitter-card"]
```

## How Oratos models it

- `--fail-under <score>`: fail if overall score is below the threshold.
- `--strict`: fail if any warnings or errors exist.
- `ignore_rules` in config suppresses findings before exit-code checks.

## Tests

```bash
cargo test -p oratos-cli
cargo test -p oratos-report
```

## Limitations

Scoring is heuristic; tune thresholds per project.

## Future improvements

Changed-only PR audits (`--changed-only`) and official GitHub Action (v0.5).
