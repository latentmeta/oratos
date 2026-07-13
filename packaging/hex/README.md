# Oratos Hex package

Mix wrapper that **downloads and runs** the Oratos CLI from GitHub Releases. It does not audit HTML inside the BEAM.

## Install

```elixir
# mix.exs
defp deps do
  [
    {:oratos, "~> 0.3"}
  ]
end
```

Local development against this monorepo:

```elixir
{:oratos, path: "packaging/hex"}
```

```bash
mix deps.get
mix oratos.audit ./priv/static --fail-under 85
mix oratos --version
```

## Publish

### Automated

Pushing a `v*` tag runs [`.github/workflows/release.yml`](../../.github/workflows/release.yml). After GitHub Release assets are uploaded, the **Publish to Hex** job:

1. Syncs `@version` in `mix.exs` from the tag (e.g. `v0.3.0` → `0.3.0`)
2. Runs `mix hex.publish --yes`

Requires repository secret **`HEX_API_KEY`** ([Hex dashboard](https://hex.pm/dashboard/keys) → New key with API access). If the secret is missing, the job skips publish and exits 0.

### Manual

```bash
cd packaging/hex
# bump @version in mix.exs if needed
mix local.hex --force
mix deps.get
HEX_API_KEY=... mix hex.publish --yes
```

## Configuration

```elixir
# config/config.exs
config :oratos,
  version: "0.3.0",
  # path: "/usr/local/bin/oratos",  # use an existing binary instead of downloading
  prefer_path: false
```

## Notes

- Requires network access on first use to fetch the release asset (or set `:path`).
- Unix: `tar` must be available. Windows: `unzip` must be available for zip assets.
- Publish runs **after** the GitHub Release so consumers can download the matching CLI binary.
