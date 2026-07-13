# Oratos

[![crates.io](https://img.shields.io/crates/v/oratos.svg)](https://crates.io/crates/oratos)
[![docs.rs](https://docs.rs/oratos/badge.svg)](https://docs.rs/oratos)
[![CI](https://github.com/latentmeta/oratos/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/latentmeta/oratos/actions/workflows/ci.yml)
[![Code Quality](https://github.com/latentmeta/oratos/actions/workflows/code-quality.yml/badge.svg?branch=main)](https://github.com/latentmeta/oratos/actions/workflows/code-quality.yml)
[![Coverage Status](https://coveralls.io/repos/github/latentmeta/oratos/badge.svg?branch=main)](https://coveralls.io/github/latentmeta/oratos?branch=main)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/latentmeta/oratos/blob/main/LICENSE)
[![MSRV](https://img.shields.io/badge/MSRV-1.74-blue)](https://github.com/latentmeta/oratos/blob/main/Cargo.toml)

**Website visibility intelligence** for SEO, accessibility, structured metadata, and AI readiness.

Oratos audits and improves websites without owning your workflow. It works on local HTML directories, static exports, and live URLs — useful in CI/CD like Credo, Sobelow, Ruff, or Lighthouse CI.

**v0.3.1** ships as a single Rust crate (CLI + library modules) with multi-ecosystem install paths that do **not** require Rust. API docs on [docs.rs](https://docs.rs/oratos) include this README.



## Install

**No Rust required.** Pick your environment — full details in [docs/install.md](https://github.com/latentmeta/oratos/blob/main/docs/install.md).

### Linux / macOS

```bash
curl -fsSL https://raw.githubusercontent.com/latentmeta/oratos/main/scripts/install.sh | sh
```

### Windows

Download `oratos-v*-windows-x86_64.zip` from [Releases](https://github.com/latentmeta/oratos/releases), extract `oratos.exe`, and add it to your `PATH`. Or use Scoop when available: `scoop install oratos`.

### Version managers

```bash
# mise
mise use -g github:latentmeta/oratos

# asdf
asdf plugin add oratos https://github.com/latentmeta/asdf-oratos.git
asdf install oratos latest
```

### CI / language ecosystems

```yaml
# GitHub Actions
- uses: latentmeta/oratos/.github/actions/setup-oratos@main
```

```bash
pip install oratos          # PyPI
npm install -D oratos       # npm
```

```elixir
# mix.exs — only: [:dev, :test], runtime: false
{:oratos, "~> 0.3", only: [:dev, :test], runtime: false}
# then: mix oratos.audit ./priv/static
```

### Rust contributors

```bash
cargo install oratos
# or from a checkout:
cargo install --path crates/oratos
```

Prebuilt binaries for Linux (x86_64/aarch64), macOS (Intel/Apple Silicon), and Windows are published on every `v*` tag with `SHA256SUMS`.

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

```text
oratos audit <target> [--format console|json|markdown|html|sarif] [--output PATH] [--fail-under SCORE] [--strict] [--crawl] [--changed-only] [--config PATH]
oratos generate llms <target> [--output PATH]
oratos generate metadata <target> [--output PATH]
oratos prompt html <file-or-url> [--output PATH]
oratos prompt phoenix <file-or-url> [--output PATH]
```

`<target>` may be a directory (`./priv/static`, `./dist`), a single HTML file, or a URL (`https://example.com`).

**URL audits:** by default a URL fetches one page. Enable multi-page crawl with `--crawl` or `[crawl] enabled = true` in `oratos.toml` (see [Configuration](https://github.com/latentmeta/oratos/blob/main/docs/configuration.md)).

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

See [docs/phoenix.md](https://github.com/latentmeta/oratos/blob/main/docs/phoenix.md) and [docs/ci.md](https://github.com/latentmeta/oratos/blob/main/docs/ci.md).

## Library

Use Oratos from Rust without shelling out to the CLI:

```toml
[dependencies]
oratos = "0.3"
```

```rust,no_run
use oratos::{audit_pages, load_pages, LoadOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pages = load_pages("examples/static_site", &LoadOptions::default()).await?;
    let report = audit_pages("examples/static_site", &pages);
    println!("overall score: {:.1}", report.scores.overall);
    Ok(())
}
```

See [docs/architecture.md](https://github.com/latentmeta/oratos/blob/main/docs/architecture.md) for module layout. Upgrading from v0.2 split crates? See [v0.3.0 release notes](https://github.com/latentmeta/oratos/blob/main/release-notes-v0.3.0.md#upgrade-from-v020). Latest patch notes: [v0.3.1](https://github.com/latentmeta/oratos/blob/main/release-notes-v0.3.1.md).

## Documentation

- [Install](https://github.com/latentmeta/oratos/blob/main/docs/install.md)
- [Architecture](https://github.com/latentmeta/oratos/blob/main/docs/architecture.md)
- [Configuration](https://github.com/latentmeta/oratos/blob/main/docs/configuration.md)
- [Reports](https://github.com/latentmeta/oratos/blob/main/docs/reports.md) — includes [JSON schema](https://github.com/latentmeta/oratos/blob/main/docs/json-schema.md)
- [Rule catalog](https://github.com/latentmeta/oratos/blob/main/docs/rules.md)
- [Scoring](https://github.com/latentmeta/oratos/blob/main/docs/scoring.md)
- [CI/CD](https://github.com/latentmeta/oratos/blob/main/docs/ci.md)
- [Phoenix workflows](https://github.com/latentmeta/oratos/blob/main/docs/phoenix.md)
- [llms.txt](https://github.com/latentmeta/oratos/blob/main/docs/llms-txt.md)
- [Remediation prompts](https://github.com/latentmeta/oratos/blob/main/docs/remediation-prompts.md)
- [Tutorials](https://github.com/latentmeta/oratos/tree/main/docs/tutorials)
- [Publishing to crates.io](https://github.com/latentmeta/oratos/blob/main/docs/publishing.md)
- [Roadmap](https://github.com/latentmeta/oratos/blob/main/docs/roadmap.md)
- [Release notes (v0.3.1)](https://github.com/latentmeta/oratos/blob/main/release-notes-v0.3.1.md)
- [pre-commit](https://github.com/latentmeta/oratos/blob/main/docs/integrations/pre-commit.md) · [Node/Python](https://github.com/latentmeta/oratos/blob/main/docs/integrations/node-python.md)

## Development

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

## License

MIT — see the [LICENSE file](https://github.com/latentmeta/oratos/blob/main/LICENSE).
