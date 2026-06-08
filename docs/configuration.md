# Configuration

Oratos **v0.2.0** supports an optional project file `oratos.toml`. CLI flags override file values.

## Discovery

Oratos searches for `oratos.toml` starting at the audit target (file or directory) and walks up parent directories. Use `--config path/to/oratos.toml` to load a specific file.

Copy [examples/oratos.toml.example](../examples/oratos.toml.example) to your site root as `oratos.toml`.

## Audit section

```toml
[audit]
fail_under = 85.0
strict = false
format = "console"   # console | json | markdown | html | sarif
ignore_rules = ["seo.missing-twitter-card"]
changed_only = false
```

| Key | Description |
|-----|-------------|
| `fail_under` | Exit non-zero if overall score is below threshold (0–100) |
| `strict` | Exit non-zero on any warning or error |
| `format` | Default report format |
| `ignore_rules` | Suppress findings with these `rule_id`s |
| `changed_only` | Only audit HTML files changed in `git diff HEAD` (directory targets) |

CLI equivalents: `--fail-under`, `--strict`, `--format`, `--changed-only`.

## Crawl section (URL targets)

```toml
[crawl]
enabled = true
max_pages = 25
max_depth = 2
respect_robots = true
use_sitemap = true
```

| Key | Description |
|-----|-------------|
| `enabled` | When `true`, fetch multiple same-origin pages from a URL seed |
| `max_pages` | Maximum pages to fetch |
| `max_depth` | Link depth from seed |
| `respect_robots` | Skip paths disallowed in `/robots.txt` |
| `use_sitemap` | Seed URLs from `/sitemap.xml` |

CLI: `--crawl` enables crawl for a URL audit (uses `[crawl]` limits from config when present).

**v0.1.x behavior:** without `oratos.toml` and without `--crawl`, a URL target still audits **one page** only.

## Future (v0.3+)

LLM provider settings, custom scoring weights, and per-rule severity overrides are planned; see [docs/roadmap.md](roadmap.md).
