# 07: CI/CD quality gates

## Mental model

Treat website visibility like a lint pass: deterministic checks that can fail builds when quality drops.

## Why it matters

Visibility regressions often ship silently: missing titles, broken internal links, or missing alt text. CI gates catch them early.

## How Oratos models it

Oratos exposes two primary CI controls:

- `--fail-under <score>`: fail if overall score is below the threshold.
- `--strict`: fail if any warnings or errors exist.

It also supports SARIF output for CI tooling.

## Implementation notes

Oratos prints reports to stdout by default; use `--output` to write artifacts in CI.

## Tests

JSON and Markdown report outputs have tests asserting expected shape/content.

## Limitations

v0.1.0 scoring is intentionally simple and will evolve.

## Future improvements

Config files, rule ignores, and changed-only auditing modes.

