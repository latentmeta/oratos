# 04: Accessibility basics for developers

## Mental model

Oratos checks a practical subset of accessibility: `lang`, alt text, landmarks, form labels, link text, and heading hierarchy.

## Why it matters

These issues block real users and often correlate with poor machine extraction (empty link text, missing headings).

## Try it

```bash
oratos audit testdata/broken_site --format console | grep A11Y
```

Fix findings in HTML, then re-run the audit to confirm scores improve.

## Implementation notes

- Rules in `A11yRules` (`a11y.*` prefix).
- Decorative images should use `alt=""`; meaningful images need descriptive alt text.

## Tests

```bash
cargo test -p oratos --test rule_cases -- a11y
```

## Limitations

Not a replacement for axe-core or manual screen-reader testing.

## Future improvements

Additional ARIA pattern checks in later versions.
