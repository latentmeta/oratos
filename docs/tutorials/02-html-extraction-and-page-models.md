# 02: HTML extraction and page models

## Mental model

Oratos parses HTML into an `HtmlPage` struct: title, meta tags, headings, links, images, JSON-LD blocks, and approximated main text.

## Why it matters

Rules operate on a stable page model, not raw DOM noise, so the same checks work for local files and fetched URLs.

## Try it

Audit a minimal fixture:

```bash
oratos audit testdata/minimal_site --format json --output /tmp/minimal.json
```

Inspect `pages[0].findings` in the JSON output.

## Implementation notes

- Crate: `oratos::html` (`parse_html`, `load_pages`).
- Invalid HTML is tolerated where possible (see `tolerates_invalid_html` test).

## Tests

```bash
cargo test -p oratos
```

## Limitations

Main-text extraction is heuristic (not a browser layout engine).

## Future improvements

Optional lazy `PageSource` for full HTML (see ADR 0005 / #26).
