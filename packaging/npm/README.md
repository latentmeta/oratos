# `@oratos` npm package

Thin Node wrapper that downloads the Oratos CLI from GitHub Releases on `postinstall`.

```bash
npm install --save-dev oratos
npx oratos audit ./dist --fail-under 85
```

No Rust toolchain required. Override version with `ORATOS_VERSION=v0.3.0 npm install`.
