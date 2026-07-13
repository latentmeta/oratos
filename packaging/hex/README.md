# Oratos Hex package

Mix wrapper that **downloads and runs** the Oratos CLI from GitHub Releases. It does not audit HTML inside the BEAM.

## Install

```elixir
# mix.exs
defp deps do
  [
    {:oratos, path: "packaging/hex"} # during development in this monorepo
    # {:oratos, "~> 0.3"}            # after publishing to Hex
  ]
end
```

```bash
mix deps.get
mix oratos.audit ./priv/static --fail-under 85
mix oratos --version
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
