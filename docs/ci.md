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

Upload the report as a workflow artifact (or feed it to GitHub Code Scanning when configured):

```yaml
- name: Oratos audit
  run: |
    cargo run --release -p oratos-cli -- \
      audit ./priv/static \
      --format sarif \
      --output oratos.sarif

- name: Upload Oratos SARIF
  uses: actions/upload-artifact@v4
  with:
    name: oratos-sarif
    path: oratos.sarif
    if-no-files-found: error
```

For human-readable PR review, add a JSON or HTML report the same way:

```yaml
- name: Oratos HTML report
  run: cargo run --release -p oratos-cli -- audit ./priv/static --format html --output oratos.html

- name: Upload Oratos HTML report
  uses: actions/upload-artifact@v4
  with:
    name: oratos-report
    path: oratos.html
```

