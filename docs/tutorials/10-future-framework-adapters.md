# 10: Future framework adapters

## Mental model

Framework adapters should translate Oratos findings into framework-native guidance, not reimplement auditing logic.

## Why it matters

Teams want “how do I apply this in my stack?” answers (Phoenix, Next.js, etc.) without losing the benefits of a deterministic core.

## How Oratos models it

The core of Oratos is framework-agnostic:

- HTML extraction → page model
- deterministic rules → normalized findings
- report / generation layers

Adapters should focus on:

- mapping recommendations into framework conventions (components, helpers, libraries)
- generating implementation prompts or snippets

## Implementation notes

The crate boundaries in the workspace are designed to keep this separation clean.

## Tests

Adapters should be tested with fixture outputs (static exports) rather than requiring the framework at runtime.

## Limitations

Framework knowledge is inherently version-specific and will likely live in separate crates or packages.

## Future improvements

First-class Phoenix guidance and later Node/Python/Elixir wrappers.

