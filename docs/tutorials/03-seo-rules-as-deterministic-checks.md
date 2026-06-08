# 03: SEO rules as deterministic checks

## Mental model

Each SEO rule maps to a `rule_id` (for example `seo.missing-title`) with a severity and recommendation.

## Why it matters

Deterministic rules are fast, repeatable, and ideal for CI — unlike subjective “SEO scores” from opaque services.

## Try it

```bash
oratos audit testdata/broken_site --format console
```

Compare scores with the good site:

```bash
oratos audit testdata/good_site --format console
```

Suppress a rule via config (v0.2):

```toml
[audit]
ignore_rules = ["seo.missing-twitter-card"]
```

## Implementation notes

- Rules live in `crates/oratos-audit/src/rules.rs` (`SeoRules`).
- Full catalog: [rules.md](../rules.md).

## Tests

Per-rule tests: `crates/oratos-audit/tests/rule_cases.rs`.

## Limitations

Open Graph / Twitter checks are presence-based, not pixel-level preview validation.

## Future improvements

Per-rule registry and configurable severities (v0.2 architecture).
