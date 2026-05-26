# 04: Accessibility basics for developers

## Mental model

Accessibility checks ask a simple question: “Can a user with a different input or output modality still understand and operate this page?”

## Why it matters

Many accessibility issues are also discoverability issues: missing labels, missing headings, or missing landmarks reduce clarity for humans and machines.

## How Oratos models it

Oratos v0.1.0 includes basic checks for:

- missing `lang` on `<html>`
- missing or suspicious `alt` text on images
- empty link text
- missing main landmark (`<main>` or `role="main"`)
- inputs without labels (best-effort detection)
- heading hierarchy issues

## Implementation notes

These checks are intentionally lightweight and deterministic. Oratos is not a full WCAG engine.

## Tests

Fixture sites under `testdata/` cover missing alt and missing landmarks.

## Limitations

Without browser rendering, some accessible-name checks are not possible in v0.1.0.

## Future improvements

Richer accessible-name detection and deeper form/label analysis.

