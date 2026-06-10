# ADR 0006: oratos.toml configuration

## Status

Accepted (v0.2.0, 2026-06-08)

## Context

CLI-only flags are insufficient for projects that want stable ignore lists and crawl defaults in repo.

## Decision

- File name: `oratos.toml` at project or site root (discovered upward).
- Sections: `[audit]`, `[crawl]` (v0.2); `[llm]` planned v0.3.
- CLI flags override file values.
- TOML format via the `toml` crate in `oratos::core`.

## Consequences

See [docs/configuration.md](../configuration.md) and [examples/oratos.toml.example](../../examples/oratos.toml.example).
