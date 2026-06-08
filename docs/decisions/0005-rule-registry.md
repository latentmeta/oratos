# ADR 0005: Per-rule registry (planned)

## Status

Proposed (v0.2 architecture)

## Context

Category-sized rule structs (`SeoRules`, `A11yRules`) block per-rule configuration, ignores, and testing isolation.

## Decision

Refactor toward one implementation per `rule_id` registered in a central table, each implementing a shared `Rule` trait with metadata (id, default severity, category).

## Consequences

Enables `ignore_rules` in config to map cleanly to registry entries and future severity overrides. Large refactor; schedule early in v0.2.x or v0.3.
