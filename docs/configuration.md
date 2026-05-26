# Configuration

Oratos v0.1.0 intentionally does not have a config file. Configuration is done via CLI flags so it stays workflow-independent and CI-friendly.

## Audit flags

- `--format console|json|markdown|html|sarif`
- `--output <path>`
- `--fail-under <score>` (0–100, compares against overall score)
- `--strict` (fail when any warnings or errors are present)

## Future configuration

Future versions may add a config file for rule ignores, scoring weights, sitemap support, and changed-only mode.

