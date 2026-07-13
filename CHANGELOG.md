# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- Multi-ecosystem install without Rust: [`scripts/install.sh`](scripts/install.sh), expanded release matrix (linux/macOS x86_64+aarch64, Windows) + `SHA256SUMS`
- Packaging: Homebrew formula, Scoop manifest, asdf plugin, npm wrapper, Hex/Mix package, PyPI maturin config
- Composite action [`.github/actions/setup-oratos`](.github/actions/setup-oratos)
- [docs/install.md](docs/install.md); README Install section leads with no-Rust paths
- Release workflow publishes Hex package (`packaging/hex`) via `HEX_API_KEY` after GitHub Release assets are up

## [0.3.0] - 2026-06-10

### Added

- **Single `oratos` crate** — library modules (`oratos::core`, `oratos::html`, `oratos::audit`, `oratos::report`, `oratos::generate`) plus the `oratos` CLI binary
- Petrify-style CI/CD workflows: CI, code quality, dependencies, release, test setup
- `cargo deny` policy ([`deny.toml`](deny.toml))
- Cross-platform site-root path helpers (`is_site_root_path`) for Windows backslash paths
- docs.rs crate documentation includes the full [README](README.md)

### Changed

- **Breaking:** former workspace crates (`oratos-core`, `oratos-html`, `oratos-audit`, `oratos-generate`, `oratos-report`) are no longer published separately — use `oratos = "0.3"` and `use oratos::{audit_pages, load_pages, ...}`
- `scraper` upgraded to 0.27 (drops unmaintained `fxhash` transitive dependency)
- Release workflow publishes to crates.io on `v*` tag push (`CRATES_IO_TOKEN`); Windows assets ship as `.zip`
- Report `core_version` is now `0.3.0`

### Fixed

- Windows: `structured.missing-organization` and other site-root rules failed on `\index.html` paths
- `cargo publish --dry-run` failed when rustdoc included README from outside the crate package

## [0.2.0] - 2026-06-08

### Added

- `oratos.toml` configuration (`[audit]`, `[crawl]`) with CLI override
- `--config`, `--crawl`, `--changed-only` audit flags
- Rule ignore list via `ignore_rules` in config
- HTTP crawl with robots.txt and sitemap.xml discovery (URL targets, `[crawl] enabled = true`)
- `oratos prompt phoenix` for Phoenix static export workflows
- JSON-LD hints: `structured.missing-article`, `structured.missing-organization`
- LLM provider trait preview in `oratos::generate` (not wired to CLI yet)
- ADRs 0002–0006, [roadmap](docs/roadmap.md), integration docs (pre-commit, Node/Python, GitHub Action example)

### Changed

- Workspace version 0.2.0; report headers use `core_version` 0.2.0

## [0.1.0] - 2026-05-27

### Added

- Workspace crates: `oratos-core`, `oratos-html`, `oratos-audit`, `oratos-generate`, `oratos-report`, `oratos` (CLI)
- `oratos audit` for local directories, single HTML files, and URLs
- Report formats: console, JSON, Markdown, HTML, SARIF
- `oratos generate llms` and `oratos generate metadata`
- `oratos prompt html` remediation prompts
- CI flags: `--fail-under`, `--strict`, `--output`
- Documentation: architecture, reports, scoring, JSON schema, rule catalog, tutorials (outlines)

### Fixed

- UTF-8-safe text truncation in generators
- Performance-hint category scoring for image dimension findings
- Site-level `llm.missing-llms-txt` (fires once per site, not per page)
- SARIF driver rules deduplication and empty location omission
- Internal link resolution for relative and root-relative paths
