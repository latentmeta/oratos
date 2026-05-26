# Phoenix / Mix workflows

Oratos audits **rendered HTML**. It does not depend on Phoenix at runtime and it does not replace Phoenix SEO libraries.

## Static export / PhoenixPrerender

One common approach is:

```bash
mix assets.deploy
mix phx.digest
mix phoenix.prerender
oratos audit ./priv/static --fail-under 85
```

Notes:

- `mix phoenix.prerender` (or `mix phx.prerender`) is provided by the `phoenix_prerender` package.
- Oratos only needs the generated HTML directory (often under `priv/static`).

## Mix aliases

```elixir
aliases: [
  "seo.audit": ["cmd oratos audit ./priv/static --fail-under 85"],
  "seo.report": ["cmd oratos audit ./priv/static --format html --output reports/oratos.html"],
  "seo.prompt": ["cmd oratos prompt html ./priv/static/index.html --output reports/remediate.prompt.md"]
]
```

## GitHub Actions recipe

```yaml
- name: Build Phoenix assets
  run: mix assets.deploy
- name: Digest assets
  run: mix phx.digest
- name: Generate static pages
  run: mix phoenix.prerender
- name: Run Oratos audit
  run: oratos audit ./priv/static --fail-under 85 --format sarif --output reports/oratos.sarif
```

## Relationship to `phoenix_seo`

- `phoenix_seo` helps Phoenix applications express SEO metadata at render time.
- Oratos audits and scores the final HTML output, generates recommendations, produces CI reports, and generates remediation prompts.

