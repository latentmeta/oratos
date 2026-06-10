# Release roadmap

High-level plan from v0.1.0 through v0.5.0. See [CHANGELOG.md](../CHANGELOG.md) for shipped versions.

## v0.1.0 (shipped 2026-05-27)

CLI audit, reports, generators, crates.io, GitHub releases, CI with coverage gate.

## v0.2.0 (shipped 2026-06-08)

- `oratos.toml` configuration
- Rule ignores, changed-only audits
- HTTP crawl, robots.txt, sitemap discovery
- Richer JSON-LD hints (Article, Organization)
- `oratos prompt phoenix`

## v0.3.0 (shipped 2026-06-10)

- Single `oratos` crate (library modules + CLI)
- Petrify-style CI/CD workflows, `cargo deny`, docs.rs README
- Windows site-root path fixes; `scraper` 0.27

## v0.4.0 (planned)

- LLM provider trait wired to CLI (`[llm]` config)
- Ollama and OpenAI-compatible backends
- Alt-text / caption review workflows (no auto-write)

## v0.6.0 (planned)

- Mix task / Hex package wrapper
- `phoenix_seo` recommendation generator
- Expanded Phoenix tutorials
- Official GitHub Action
- pre-commit hook examples
- Node/Python thin wrappers
