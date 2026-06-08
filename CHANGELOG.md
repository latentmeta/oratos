# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-06-08

### Added

- `oratos.toml` configuration (`[audit]`, `[crawl]`) with CLI override
- `--config`, `--crawl`, `--changed-only` audit flags
- Rule ignore list via `ignore_rules` in config
- HTTP crawl with robots.txt and sitemap.xml discovery (URL targets, `[crawl] enabled = true`)
- `oratos prompt phoenix` for Phoenix static export workflows
- JSON-LD hints: `structured.missing-article`, `structured.missing-organization`
- LLM provider trait preview in `oratos-generate` (not wired to CLI yet)
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
