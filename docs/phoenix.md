# Phoenix / Mix workflows

Oratos audits **rendered HTML**. It does not depend on Phoenix at runtime and it does not replace Phoenix SEO libraries.

Install without Rust: [docs/install.md](install.md).

## Hex dependency (recommended)

Add the Oratos Mix wrapper (manages the CLI binary):

```elixir
# mix.exs — audit tooling only; not shipped in production
defp deps do
  [
    {:oratos, "~> 0.3.1", only: [:dev, :test], runtime: false}
    # {:oratos, path: "../oratos/packaging/hex", only: [:dev, :test], runtime: false}
  ]
end
```

```bash
mix deps.get
mix assets.deploy && mix phx.digest && mix phoenix.prerender
mix oratos.audit ./priv/static --fail-under 85
```

Full Mix/HexDocs guide (aliases, config, CI, vs `phoenix_seo`): [hex.pm/packages/oratos](https://hex.pm/packages/oratos) · source [`packaging/hex/README.md`](../packaging/hex/README.md).

## Static export / PhoenixPrerender

When `oratos` is already on your `PATH`:

```bash
mix assets.deploy
mix phx.digest
mix phoenix.prerender
oratos audit ./priv/static --fail-under 85
```

Notes:

- `mix phoenix.prerender` (or `mix phx.prerender`) is provided by the `phoenix_prerender` package.
- Oratos only needs the generated HTML directory (often under `priv/static`).

## Phoenix remediation prompt

```bash
oratos prompt phoenix priv/static/index.html
# or via Mix:
mix oratos prompt phoenix priv/static/index.html --output tmp/fix.md
```

## Mix aliases (PATH-based)

Zero Hex dependency if the binary is installed via Homebrew / `install.sh` / CI action:

```elixir
aliases: [
  "seo.audit": ["cmd oratos audit ./priv/static --fail-under 85"],
  "seo.report": ["cmd oratos audit ./priv/static --format html --output reports/oratos.html"],
  "seo.prompt": ["cmd oratos prompt html ./priv/static/index.html --output reports/remediate.prompt.md"]
]
```

## GitHub Actions recipe

```yaml
- uses: actions/checkout@v4
- uses: latentmeta/oratos/.github/actions/setup-oratos@main
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
