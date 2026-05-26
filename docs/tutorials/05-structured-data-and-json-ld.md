# 05: Structured data and JSON-LD

## Mental model

Structured data is a machine-readable summary of what a page *is* (WebPage, Article, Organization, etc.), not just what it says.

## Why it matters

Structured data can improve how pages appear in rich results and helps downstream systems (including LLM tools) disambiguate entities.

## How Oratos models it

Oratos extracts JSON-LD blocks and checks for:

- missing JSON-LD
- invalid JSON syntax in JSON-LD
- missing “WebPage” as a recommended baseline type
- BreadcrumbList and ImageObject recommendations when signals are present

Oratos can generate recommendation JSON-LD for:

- `WebPage`
- `Organization`
- `BreadcrumbList`
- `ImageObject`

## Implementation notes

v0.1.0 validates JSON syntax only. It does not perform full Schema.org validation.

## Tests

The `testdata/good_site` fixture contains a minimal WebPage JSON-LD example.

## Limitations

Recommendations are generic; they cannot infer business-specific fields without additional context.

## Future improvements

More schema types (Article, Event, Person) and stronger validation.

