# 03: SEO rules as deterministic checks

## Mental model

SEO audits are mostly audits of *extractable signals*: titles, descriptions, canonicals, and headings.

## Why it matters

Search engines and link unfurlers rely on these signals to understand and present your pages.

## How Oratos models it

Oratos emits findings in the `Seo` category for rules such as:

- missing / too-short / too-long titles
- missing / too-short / too-long meta descriptions
- missing / multiple canonicals
- missing / multiple `<h1>`
- heading hierarchy skips
- missing Open Graph and Twitter card metadata

## Implementation notes

Rules run per page and return a list of normalized findings (`rule_id`, severity, category, message, recommendation).

## Tests

Audit tests use the `testdata/broken_site` and `testdata/good_site` fixtures.

## Limitations

These checks are heuristics. They cannot tell whether a given title is “good,” only whether it is missing or obviously problematic.

## Future improvements

Sitemap integration and richer page-type detection.

