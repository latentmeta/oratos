# `@latentmeta/oratos`

Thin Node wrapper that downloads the Oratos CLI from GitHub Releases on `postinstall`.

```bash
npm install --save-dev @latentmeta/oratos
npx oratos audit ./dist --fail-under 85
```

No Rust toolchain required. Override version with `ORATOS_VERSION=v0.3.1 npm install`.

## Publishing (maintainers)

CI publishes on `v*` tags via repository secret `NPM_TOKEN`.

**Important:** `export NPM_TOKEN=…` alone does not authenticate `npm publish`. You need a temporary `.npmrc` that references the token (never commit it), or an auth token set via `npm config`. Full steps, token settings (granular + bypass 2FA), and the common `403` / 2FA error are documented in [docs/publishing.md](../../docs/publishing.md#npm-latentmetaoratos).
