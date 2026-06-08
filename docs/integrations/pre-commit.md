# pre-commit integration (v0.5)

Run Oratos on changed HTML before each commit.

## Hook example

Add to `.pre-commit-config.yaml`:

```yaml
repos:
  - repo: local
    hooks:
      - id: oratos-audit
        name: oratos audit (changed HTML)
        entry: oratos audit ./priv/static --changed-only --strict
        language: system
        pass_filenames: false
        files: \.(html|htm)$
```

Install [pre-commit](https://pre-commit.com/) and run `pre-commit install`.

## Notes

- Requires `oratos` on `PATH` (`cargo install oratos`).
- `--changed-only` uses `git diff HEAD` under the audit target directory.
- For full-site gates, use CI instead (see [ci.md](../ci.md)).
