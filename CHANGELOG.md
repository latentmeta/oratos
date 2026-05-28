# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2026-05-27

### Added

- Workspace crates: `oratos-core`, `oratos-html`, `oratos-audit`, `oratos-generate`, `oratos-report`, `oratos-cli`
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
