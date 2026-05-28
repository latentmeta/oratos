# Oratos v0.1.0

First public release of **Oratos** - website visibility intelligence for SEO, accessibility, structured metadata, and AI readiness. Audit static HTML, local exports, or live URLs from the CLI and wire it into CI like Lighthouse CI or Credo.

## Highlights

- **`oratos audit`** - crawl a directory, single HTML file, or URL; score pages and emit findings with stable `rule_id`s
- **Five report formats** - console, JSON, Markdown, HTML, SARIF
- **`oratos generate`** - draft `llms.txt` and metadata / JSON-LD recommendations
- **`oratos prompt html`** - LLM remediation prompts that preserve visible content
- **CI-friendly** - `--fail-under`, `--strict`, `--output` for quality gates and artifacts

## Install

**From source (today):**

```bash
cargo install oratos
```

Or from source:

```bash
git clone https://github.com/latentmeta/oratos
cd oratos
cargo install --path crates/oratos-cli
```

**Prebuilt binaries** (when the release workflow completes):

| Platform | Asset |
|----------|--------|
| Linux x86_64 | `oratos-v0.1.0-linux-x86_64.tar.gz` |
| macOS Apple Silicon | `oratos-v0.1.0-macos-aarch64.tar.gz` |
| Windows x86_64 | `oratos-v0.1.0-windows-x86_64.tar.gz` |

Extract and run `oratos` (or `oratos.exe` on Windows).

## Quick start

```bash
oratos audit examples/static_site
oratos audit examples/static_site --format json --output report.json
oratos audit ./priv/static --fail-under 85 --strict
oratos generate llms examples/static_site
oratos prompt html examples/static_site/index.html
```

## What it checks

| Category | Examples |
|----------|----------|
| **SEO** | Title, meta description, canonical, headings, Open Graph, Twitter cards, broken internal links |
| **Accessibility** | `lang`, alt text, landmarks, form labels, link text, heading hierarchy |
| **Structured data** | JSON-LD presence/validity, WebPage, BreadcrumbList, ImageObject hints |
| **LLM readiness** | `llms.txt` (site-level), extractable text, summaries, image descriptions |

Scoring is 0–100 per category with weighted overall score (SEO 30%, A11y 25%, Structured Data 25%, LLM 20%). See [docs/scoring.md](https://github.com/latentmeta/oratos/blob/main/docs/scoring.md).

## CLI surface

```
oratos audit <target> [--format console|json|markdown|html|sarif] [--output PATH] [--fail-under SCORE] [--strict]
oratos generate llms <target> [--output PATH]
oratos generate metadata <target> [--output PATH]
oratos prompt html <file-or-url> [--output PATH]
```

## Workspace crates

- `oratos-core` - models, scoring, findings
- `oratos-html` - load and parse HTML
- `oratos-audit` - rule engine (~30 rules; [full catalog](https://github.com/latentmeta/oratos/blob/main/docs/rules.md))
- `oratos-generate` - llms.txt, metadata, prompts
- `oratos-report` - formatters
- `oratos` - CLI package (`oratos` binary; sources in `crates/oratos-cli`)

## Documentation

- [Architecture](https://github.com/latentmeta/oratos/blob/main/docs/architecture.md)
- [Reports & JSON schema](https://github.com/latentmeta/oratos/blob/main/docs/json-schema.md)
- [CI/CD](https://github.com/latentmeta/oratos/blob/main/docs/ci.md)
- [Phoenix / Mix](https://github.com/latentmeta/oratos/blob/main/docs/phoenix.md)
- [Tutorials](https://github.com/latentmeta/oratos/tree/main/docs/tutorials) (outlines in v0.1.0)

## Notable fixes in this release

- UTF-8-safe text truncation in generators
- Correct scoring for performance-hint findings (e.g. image dimensions)
- Site-level `llm.missing-llms-txt` (once per site, not per page)
- SARIF: deduplicated driver rules, omit empty locations
- Internal link resolution for relative and root-relative paths

## Known limitations (v0.1.0)

- Install via `cargo install oratos` or release binaries; see [docs/publishing.md](https://github.com/latentmeta/oratos/blob/main/docs/publishing.md) for crate layout
- URL audits fetch a single page (no multi-page crawl over HTTP)
- Tutorials are outlines; rule silencing/config is not yet exposed in CLI
- macOS release builds target **Apple Silicon** (`aarch64`); Intel Mac users should build from source

## Full changelog

See [CHANGELOG.md](https://github.com/latentmeta/oratos/blob/main/CHANGELOG.md).

---

**License:** MIT
