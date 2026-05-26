# 02: HTML extraction and page models

## Mental model

HTML is both presentation and metadata. Auditing starts by extracting a “page model” that represents what automated systems can read.

## Why it matters

Rules become simpler and more testable when they operate on extracted fields instead of raw DOM queries.

## How Oratos models it

`oratos-html` loads HTML from:

- local directories (`.html` / `.htm`)
- single files
- URLs

It extracts:

- title, meta description, canonical URLs, language
- headings (h1–h6)
- links and images (including alt text and dimensions when present)
- Open Graph and Twitter card tags
- JSON-LD blocks (with basic JSON validity and detected `@type` values)
- an approximated main text
- basic landmark/form signals

## Implementation notes

Extraction uses the `scraper` crate and favors deterministic parsing over perfect rendering.

## Tests

See `crates/oratos-html/tests/fixtures.rs` and `testdata/*` fixtures.

## Limitations

No JS execution; no CSS layout knowledge; limited link resolution.

## Future improvements

Sitemap-aware crawling, richer landmark detection, and more robust internal link resolution.

