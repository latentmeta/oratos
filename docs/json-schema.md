# JSON report schema (stable)

Oratos JSON output serializes `AuditReport` from `oratos::core`.

Top-level keys:

- `core_version` (string)
- `target` (object)
  - `path_or_url` (string)
  - `kind` (`directory` | `file` | `url` | `missing`)
- `pages` (array of page audits)
- `findings` (array of flattened findings)
- `scores` (object: `seo`, `accessibility`, `structured_data`, `llm_readiness`, `overall`)
- `page_count` (number)

Finding object keys:

- `rule_id` (string)
- `severity` (`info` | `warning` | `error`)
- `category` (`seo` | `accessibility` | `structured_data` | `llm_readiness` | `performance_hint`)
- `message` (string)
- `location` (optional object)
- `recommendation` (optional string)
- `docs_url` (optional string)

Stability guarantee:

- The `oratos::report` module includes an insta snapshot test for empty-report JSON shape.
- Additive fields may be introduced in minor releases, but existing fields should not be removed or renamed without a major-version migration note.
