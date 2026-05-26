# ADR 0001: Project skeleton

## Status

Accepted

## Context

Oratos v0.1.0 requires a Rust workspace with a CLI binary and shared crates for HTML parsing, auditing, reporting, and generation.

## Decision

Use a Cargo workspace with six crates:

- `oratos-core` — shared types (findings, scores, audit report)
- `oratos-html` — HTML load and extract
- `oratos-audit` — deterministic rules
- `oratos-generate` — llms.txt, metadata recommendations, remediation prompts
- `oratos-report` — console, JSON, Markdown, HTML, SARIF
- `oratos-cli` — `oratos` binary

## Consequences

Clear boundaries for testing and future framework adapters without coupling the CLI to rule internals.
