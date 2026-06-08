# ADR 0003: SARIF report shape

## Status

Accepted

## Context

CI systems and GitHub Code Scanning expect valid SARIF 2.1.0 with deduplicated rule metadata.

## Decision

- Emit SARIF version `2.1.0`.
- Deduplicate `tool.driver.rules` by `rule_id`.
- Omit `locations` on results when no location is available.
- Map Oratos severities to SARIF levels (error, warning, note).

## Consequences

See tests in `oratos-report/src/sarif.rs`. Future rules should always set locations when a selector is known.
