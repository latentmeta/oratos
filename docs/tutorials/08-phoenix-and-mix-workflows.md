# 08: Phoenix and Mix workflows

## Mental model

Oratos is workflow-independent. For Phoenix, that means it should run against the *output* (rendered HTML) produced by your build/export step.

## Why it matters

Phoenix projects can generate HTML through templates, LiveView, and prerender tools. A good audit tool should work without depending on Phoenix internals.

## How Oratos models it

Run Oratos against the directory that contains your built/prerendered HTML (often `./priv/static`).

## Implementation notes

Example build pipeline:

```bash
mix assets.deploy
mix phx.digest
mix phoenix.prerender
oratos audit ./priv/static --fail-under 85
```

Mix aliases can wrap Oratos for convenience:

```elixir
aliases: [
  "seo.audit": ["cmd oratos audit ./priv/static --fail-under 85"],
  "seo.report": ["cmd oratos audit ./priv/static --format html --output reports/oratos.html"],
  "seo.prompt": ["cmd oratos prompt html ./priv/static/index.html --output reports/remediate.prompt.md"]
]
```

## Tests

The repository includes a small `testdata/phoenix_export/` fixture to keep Phoenix workflows in mind.

## Limitations

Oratos does not (yet) generate Phoenix-specific patches or recommend specific `phoenix_seo` usage patterns.

## Future improvements

Phoenix-specific guidance prompts that suggest how to express recommended metadata in Phoenix components and libraries.

