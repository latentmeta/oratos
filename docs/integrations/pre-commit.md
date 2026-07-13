# pre-commit integration

Run Oratos on changed HTML before each commit.

## Hook example (binary on PATH)

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

Install Oratos first ([install.md](../install.md)), then [pre-commit](https://pre-commit.com/) and run `pre-commit install`.

## Hook example (PyPI)

Once `pip install oratos` works for your platform:

```yaml
repos:
  - repo: local
    hooks:
      - id: oratos-audit
        name: oratos audit
        entry: oratos audit ./priv/static --changed-only --strict
        language: python
        additional_dependencies: ["oratos==0.3.1"]
        pass_filenames: false
        files: \.(html|htm)$
```

## Notes

- `--changed-only` uses `git diff HEAD` under the audit target directory.
- For full-site gates, use CI instead (see [ci.md](../ci.md)).
