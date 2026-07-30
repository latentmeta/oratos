# Publishing the Oratos Hex package (maintainers)

Consumer docs live in [README.md](README.md) (published to HexDocs).

## Automated

Pushing a `v*` tag runs [`.github/workflows/release.yml`](../../.github/workflows/release.yml). After GitHub Release assets are uploaded, **Publish to Hex**:

1. Syncs `@version` in `mix.exs` from the tag
2. Runs `mix hex.publish --yes`

Requires repository secret `HEX_API_KEY`.

## Manual

```bash
cd packaging/hex
# bump @version in mix.exs if needed
mix local.hex --force
mix deps.get
HEX_API_KEY=... mix hex.publish --yes
```
