# ADR 0002: Scoring weights and penalties

## Status

Accepted

## Context

Oratos reports 0–100 scores per category plus an overall score for CI gates.

## Decision

- Category weights for overall: SEO 30%, Accessibility 25%, Structured Data 25%, LLM Readiness 20%.
- Per-finding penalties within a category: error −10, warning −5, info −1.
- `Category::PerformanceHint` findings (e.g. image dimensions) map to SEO for scoring; they do not affect a separate performance score in v0.1–0.2.

## Consequences

Documented in [docs/scoring.md](../scoring.md). Custom weights are deferred to a future config section.
