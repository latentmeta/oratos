# Oratos

**Website visibility intelligence** for SEO, accessibility, structured metadata, and AI readiness.

Oratos audits and improves websites without owning your workflow. It works on local HTML directories, static exports, and live URLs — useful in CI/CD like Credo, Sobelow, Ruff, or Lighthouse CI.


[![Coverage Status](https://coveralls.io/repos/github/latentmeta/oratos/badge.svg?branch=main)](https://coveralls.io/latentmeta/oratos/ex_data_sketch?branch=main)



## Install

```bash
cargo install oratos
```

From a git checkout:

```bash
cargo install --path crates/oratos-cli
```

Or build from source:

```bash
cargo build --release
./target/release/oratos --help
```

## Quick start

```bash
# Audit a static site directory
oratos audit examples/static_site

# JSON report for tooling
oratos audit examples/static_site --format json --output report.json

# CI gate: fail if overall score is below 85
oratos audit ./priv/static --fail-under 85

# Generate llms.txt draft
oratos generate llms examples/static_site

# LLM remediation prompt for a page
oratos prompt html examples/static_site/index.html
```

## CLI

```
oratos audit <target> [--format console|json|markdown|html|sarif] [--output PATH] [--fail-under SCORE] [--strict]
oratos generate llms <target> [--output PATH]
oratos generate metadata <target> [--output PATH]
oratos prompt html <file-or-url> [--output PATH]
```

`<target>` may be a directory (`./priv/static`, `./dist`), a single HTML file, or a URL (`https://example.com`).

## What Oratos checks

- **SEO** — title, meta description, canonical, headings, Open Graph, Twitter cards, internal links
- **Accessibility** — alt text, `lang`, landmarks, form labels, link text
- **Structured data** — JSON-LD presence and syntax, WebPage/BreadcrumbList/ImageObject hints
- **LLM readiness** — `llms.txt`, extractable text, summaries, image descriptions

Scores are 0–100 per category. Overall score uses weights: SEO 30%, Accessibility 25%, Structured Data 25%, LLM Readiness 20%. Penalties: error −10, warning −5, info −1 (per category).

## Phoenix / Mix

Oratos does not replace Phoenix SEO libraries — it audits rendered HTML after prerendering or static export:

```elixir
# mix.exs
aliases: [
  "seo.audit": ["cmd oratos audit ./priv/static --fail-under 85"],
  "seo.report": ["cmd oratos audit ./priv/static --format html --output reports/oratos.html"]
]
```

See [docs/phoenix.md](docs/phoenix.md) and [docs/ci.md](docs/ci.md).

## Documentation

- [Architecture](docs/architecture.md)
- [Configuration](docs/configuration.md)
- [Reports](docs/reports.md) — includes [JSON schema](docs/json-schema.md)
- [Rule catalog](docs/rules.md)
- [Scoring](docs/scoring.md)
- [CI/CD](docs/ci.md)
- [Phoenix workflows](docs/phoenix.md)
- [llms.txt](docs/llms-txt.md)
- [Remediation prompts](docs/remediation-prompts.md)
- [Tutorials](docs/tutorials/)
- [Publishing to crates.io](docs/publishing.md)

## Development

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

## License

MIT — see [LICENSE](LICENSE).
