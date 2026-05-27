# Reports

Oratos supports five report formats in v0.1.0:

- Console
- JSON
- Markdown
- HTML
- SARIF

## Console

The console report is optimized for local iteration and CI logs:

```bash
oratos audit ./dist --format console
```

## JSON

The JSON output is a stable `AuditReport` model intended for CI and tooling integrations:

```bash
oratos audit ./dist --format json --output report.json
```

See [`docs/json-schema.md`](json-schema.md) for the documented stable shape.

## Markdown / HTML

Use Markdown for PR comments and HTML for human-friendly browsing:

```bash
oratos audit ./dist --format markdown --output report.md
oratos audit ./dist --format html --output report.html
```

## SARIF

SARIF is useful for CI pipelines and code scanning tools:

```bash
oratos audit ./dist --format sarif --output report.sarif
```

### GitHub code scanning (optional)

If you want to upload SARIF to GitHub code scanning, you can add a step using `github/codeql-action/upload-sarif@v4`.

