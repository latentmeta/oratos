# 09: Remediation prompts

## Mental model

An audit is useful only if it can be acted on. Oratos generates prompts that turn deterministic findings into reviewable HTML changes.

## Why it matters

Teams often want to use an LLM to speed up fixes, but they need strong constraints to avoid accidental content loss or design regressions.

## How Oratos models it

`oratos prompt html <file-or-url>` generates a prompt that includes:

- page context
- detected issues
- required changes
- preservation rules (must preserve all visible content)
- accessibility and structured data rules
- expected output format (including a unified diff)

## Implementation notes

Oratos does not call any LLM provider in v0.1.0. It only generates a prompt you can copy into your tool of choice.

## Tests

Prompt generation has unit tests that assert key requirements are included (preservation and unified-diff instructions).

## Limitations

Some fixes require human judgment (e.g., writing accurate alt text without enough context).

## Future improvements

Framework-specific remediation prompts and automated “human review required” surfacing.

