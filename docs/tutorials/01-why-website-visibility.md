# 01: Why website visibility matters

## Mental model

“Visibility” is the overlap between what you intend a page to communicate and what automated systems can reliably extract: search crawlers, accessibility tools, link unfurlers, and LLM-based agents.

## Why it matters

If a page’s title, description, headings, or structured data are missing or inconsistent, the same content will be harder to discover, summarize, and navigate.

## Try it

From the repository root:

```bash
cargo install --path crates/oratos
oratos audit examples/static_site
```

You should see category scores and per-page findings with stable `rule_id`s (for example `seo.missing-canonical`).

## How Oratos models it

Oratos models each page as a set of extractable signals (metadata, headings, links, images, text) and produces normalized findings across categories.

## Implementation notes

- The CLI loads HTML pages, runs deterministic checks, and renders reports.
- Oratos never rewrites user files automatically — use `oratos prompt html` for LLM-assisted fixes you review.

## Tests

```bash
cargo test -p oratos
cargo test -p oratos
```

Fixture sites live under `testdata/` and `examples/static_site/`.

## Limitations

- No JavaScript execution (static HTML only).
- Not a full WCAG compliance engine.
- URL audits fetch one page unless crawl is enabled (v0.2+).

## Future improvements

See [roadmap.md](../roadmap.md) for v0.3+ LLM workflows and framework adapters.
