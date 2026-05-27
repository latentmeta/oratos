# Oratos architecture

Oratos is a small Rust workspace built around a single idea: extract a page model from HTML, run deterministic rules to produce findings, then render those findings in formats that work in terminals and CI.

## Workspace crates

- `crates/oratos-core`: shared models (audit report, findings, scores)
- `crates/oratos-html`: HTML loading (directory/file/URL) and extraction
- `crates/oratos-audit`: deterministic rules that emit normalized findings
- `crates/oratos-report`: console/JSON/Markdown/HTML/SARIF formatters
- `crates/oratos-generate`: `llms.txt`, metadata/JSON-LD recommendations, remediation prompts
- `crates/oratos-cli`: the `oratos` binary

## Data flow

1. **Load** HTML pages from a directory, single file, or URL.
2. **Extract** a page model (title, description, headings, links, images, JSON-LD, approximated main text).
3. **Audit** with deterministic rules to produce findings.
4. **Score** (0–100 per category, plus overall).
5. **Render / generate** reports and artifacts.

See [Scoring](scoring.md) for exact weights and penalty math.

