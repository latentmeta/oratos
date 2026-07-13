# Release roadmap

High-level plan from v0.1.0 onward. See [CHANGELOG.md](../CHANGELOG.md) for shipped versions and [install.md](install.md) for multi-ecosystem distribution.

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

## v0.3.1 (shipped 2026-07-13)

- Multi-ecosystem install without Rust (`install.sh`, expanded release matrix + `SHA256SUMS`)
- Packaging: Homebrew / Scoop / asdf / npm / Hex / PyPI
- `setup-oratos` GitHub Action; Hex publish on `v*` tags
- Security dependency upgrades (`quick-xml` 0.41, etc.)

## v0.4.0 (planned)

- LLM provider trait wired to CLI (`[llm]` config)
- Ollama and OpenAI-compatible backends
- Alt-text / caption review workflows (no auto-write)

## v0.5.0 (planned)

- `phoenix_seo` recommendation generator
- Expanded Phoenix tutorials
- Official pre-commit hook repo (pip-based)
- winget package
