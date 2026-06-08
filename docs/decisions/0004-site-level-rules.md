# ADR 0004: Site-level vs page-level rules

## Status

Accepted (v0.1); refine in v0.2+

## Context

Some checks apply once per site (`llms.txt`), others per page (title, alt text).

## Decision

- **v0.1:** Site-level `llm.missing-llms-txt` is emitted in `audit.rs` after page rules, once per audit.
- **v0.2+:** Introduce explicit `SiteRule` vs `PageRule` traits in `oratos-audit` (see [#26](https://github.com/latentmeta/oratos/issues/26)).

## Consequences

Avoid duplicating site-level findings per page. New site rules must not run inside per-page category loops.
