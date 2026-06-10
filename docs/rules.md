# Rule catalog

Each finding uses a stable `rule_id` you can grep in reports or silence in future config. Severity and category are fixed per rule in v0.1.0.

## SEO (`seo.*`)

| Rule ID | Severity | Summary |
|---------|----------|---------|
| `seo.missing-title` | Error | No `<title>` element |
| `seo.title-too-short` | Warning | Title shorter than recommended |
| `seo.title-too-long` | Warning | Title longer than recommended |
| `seo.missing-meta-description` | Warning | No meta description |
| `seo.meta-description-too-short` | Warning | Meta description too short |
| `seo.meta-description-too-long` | Warning | Meta description too long |
| `seo.missing-canonical` | Warning | No canonical link |
| `seo.multiple-canonical` | Error | More than one canonical |
| `seo.missing-h1` | Error | No top-level `<h1>` |
| `seo.multiple-h1` | Warning | Multiple `<h1>` elements |
| `seo.heading-hierarchy-skip` | Warning | Heading levels skip (e.g. h1 → h3) |
| `seo.missing-og-title` | Warning | Missing `og:title` |
| `seo.missing-og-description` | Warning | Missing `og:description` |
| `seo.missing-og-image` | Warning | Missing `og:image` |
| `seo.missing-twitter-card` | Warning | Missing Twitter card meta |
| `seo.broken-internal-link` | Error | Internal `href` does not resolve to a known page |
| `seo.image-missing-dimensions` | Info | Image without `width`/`height` (performance hint) |

## Accessibility (`a11y.*`)

| Rule ID | Severity | Summary |
|---------|----------|---------|
| `a11y.missing-html-lang` | Error | `<html>` missing `lang` |
| `a11y.image-missing-alt` | Error | Image without `alt` |
| `a11y.image-empty-alt` | Warning | Empty `alt` on likely meaningful image |
| `a11y.link-empty-text` | Warning | Link with no visible text |
| `a11y.missing-main-landmark` | Warning | No `<main>` or `role="main"` |
| `a11y.input-without-label` | Warning | Form control without associated label |
| `a11y.heading-hierarchy` | Warning | Heading level skip (accessibility category) |

## Structured data (`structured.*`)

| Rule ID | Severity | Summary |
|---------|----------|---------|
| `structured.missing-json-ld` | Warning | No JSON-LD blocks |
| `structured.invalid-json-ld` | Error | JSON-LD is not valid JSON |
| `structured.missing-webpage` | Info | No `WebPage` type in JSON-LD |
| `structured.missing-breadcrumb-list` | Info | No `BreadcrumbList` in JSON-LD |
| `structured.missing-image-object` | Info | No `ImageObject` when images are present |
| `structured.missing-article` | Info | Blog-like URL without Article/BlogPosting JSON-LD |
| `structured.missing-organization` | Info | Site root without Organization/WebSite JSON-LD |

## LLM readiness (`llm.*`)

| Rule ID | Severity | Summary |
|---------|----------|---------|
| `llm.missing-llms-txt` | Warning | Site root has no `llms.txt` (once per site) |
| `llm.weak-title-description` | Warning | Title/description weak for extraction |
| `llm.missing-primary-heading` | Warning | No clear primary heading |
| `llm.image-lacks-description` | Warning | Image without useful alt/title |
| `llm.little-extractable-text` | Warning | Very little main-body text |
| `llm.no-summary-candidate` | Info | No obvious summary paragraph |

Implementation: `crates/oratos/src/audit/rules.rs`.
