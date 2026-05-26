# CI/CD

Oratos is designed to run as a quality gate in CI.

## Fail thresholds

Fail the job when the overall score drops below a threshold:

```bash
oratos audit ./priv/static --fail-under 85
```

Fail the job when any warnings or errors exist:

```bash
oratos audit ./priv/static --strict
```

## SARIF in CI

```bash
oratos audit ./priv/static --format sarif --output reports/oratos.sarif
```

