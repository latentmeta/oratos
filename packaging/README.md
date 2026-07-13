# Packaging

Helpers for distributing Oratos **without requiring Rust**.

| Path | Purpose |
|------|---------|
| [`homebrew/oratos.rb`](homebrew/oratos.rb) | Homebrew formula template (fill `sha256` after each release) |
| [`scoop/oratos.json`](scoop/oratos.json) | Scoop manifest template |
| [`asdf-oratos/`](asdf-oratos/) | asdf plugin (GitHub Releases) |
| [`npm/`](npm/) | npm package with `postinstall` binary download |
| [`hex/`](hex/) | Hex/Mix package (`mix oratos.audit`) |

Also:

- [`scripts/install.sh`](../scripts/install.sh) — curl installer
- [`.github/actions/setup-oratos`](../.github/actions/setup-oratos) — GitHub Actions composite action
- [`pyproject.toml`](../pyproject.toml) — PyPI / maturin `bindings = "bin"`

User docs: [docs/install.md](../docs/install.md).
