# 01: Why website visibility matters

## Mental model

“Visibility” is the overlap between what you intend a page to communicate and what automated systems can reliably extract: search crawlers, accessibility tools, link unfurlers, and LLM-based agents.

## Why it matters

If a page’s title, description, headings, or structured data are missing or inconsistent, the same content will be harder to discover, summarize, and navigate.

## How Oratos models it

Oratos models each page as a set of extractable signals (metadata, headings, links, images, text) and produces normalized findings across categories.

## Implementation notes

- The CLI loads HTML pages, runs deterministic checks, and renders reports.
- Oratos never rewrites user files in v0.1.0.

## Tests

The repository includes fixture sites under `testdata/` and unit/integration tests for parsing and auditing.

## Limitations

v0.1.0 does not execute JavaScript and does not attempt full WCAG compliance.

## Future improvements

Configurable scoring, sitemap support, and framework adapters.

