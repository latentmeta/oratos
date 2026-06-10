# Oratos architecture

Oratos is a Rust crate built around a single idea: extract a page model from HTML, run deterministic rules to produce findings, then render those findings in formats that work in terminals and CI.

## Crate layout

The `oratos` package (`crates/oratos`) contains the CLI binary and library modules:

| Module | Role |
|--------|------|
| `oratos::core` | Shared models (audit report, findings, scores, config) |
| `oratos::html` | HTML loading (directory/file/URL) and extraction |
| `oratos::audit` | Deterministic rules that emit normalized findings |
| `oratos::report` | Console/JSON/Markdown/HTML/SARIF formatters |
| `oratos::generate` | `llms.txt`, metadata/JSON-LD recommendations, remediation prompts |

## Data flow

1. **Load** HTML pages from a directory, single file, or URL.
2. **Extract** a page model (title, description, headings, links, images, JSON-LD, approximated main text).
3. **Audit** with deterministic rules to produce findings.
4. **Score** (0–100 per category, plus overall).
5. **Render / generate** reports and artifacts.

See [Scoring](scoring.md) for exact weights and penalty math.
