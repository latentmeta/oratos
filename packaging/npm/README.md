# `@latentmeta/oratos`

Thin Node wrapper that downloads the Oratos CLI from GitHub Releases on `postinstall`.

```bash
npm install --save-dev @latentmeta/oratos
npx oratos audit ./dist --fail-under 85
```

No Rust toolchain required. Override version with `ORATOS_VERSION=v0.3.1 npm install`.

Published from CI on `v*` tags via the `NPM_TOKEN` repository secret (npm **Automation** token for the `latentmeta` org).
