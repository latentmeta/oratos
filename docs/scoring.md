# Scoring model

Oratos computes deterministic category scores from audit findings.

## Category scores

Each scored category starts at `100` and applies severity penalties per finding:

- `error`: `-10`
- `warning`: `-5`
- `info`: `-1`

Category score formula:

`score = clamp(100 - sum(penalties), 0, 100)`

Scored categories in v0.1.0:

- SEO
- Accessibility
- Structured Data
- LLM Readiness

`PerformanceHint` is currently informational and not a separately weighted category.

## Overall score

Overall is a weighted sum of category scores:

- SEO: `30%`
- Accessibility: `25%`
- Structured Data: `25%`
- LLM Readiness: `20%`

Formula:

`overall = seo * 0.30 + accessibility * 0.25 + structured_data * 0.25 + llm_readiness * 0.20`

## Notes

- Scores are deterministic for the same input HTML.
- Scores are bounded to `0..=100`.
- Site-level findings (for example missing `llms.txt`) affect report-level scoring once per audit.
