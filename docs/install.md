# Install Oratos

Oratos is a single native CLI binary. **You do not need Rust** for normal use.

Current release: **v0.3.1**. Full install matrix and ecosystem wrappers are summarized below. See also [CI/CD](ci.md) and [Phoenix](phoenix.md).

## Quick install (recommended)

### Linux / macOS

```bash
curl -fsSL https://raw.githubusercontent.com/latentmeta/oratos/main/scripts/install.sh | sh
```

Installs to `~/.local/bin` by default. Override with:

```bash
ORATOS_VERSION=v0.3.1 ORATOS_INSTALL_DIR=/usr/local/bin \
  curl -fsSL https://raw.githubusercontent.com/latentmeta/oratos/main/scripts/install.sh | sh
```

### cargo-binstall

If you already have [cargo-binstall](https://github.com/cargo-bins/cargo-binstall):

```bash
cargo binstall oratos
```

### GitHub Releases (manual)

Download the asset for your platform from [Releases](https://github.com/latentmeta/oratos/releases), verify against `SHA256SUMS`, extract, and put `oratos` (or `oratos.exe`) on your `PATH`.

| Platform | Asset |
|----------|--------|
| Linux x86_64 | `oratos-v0.3.1-linux-x86_64.tar.gz` |
| Linux aarch64 | `oratos-v0.3.1-linux-aarch64.tar.gz` |
| macOS Apple Silicon | `oratos-v0.3.1-macos-aarch64.tar.gz` |
| macOS Intel | `oratos-v0.3.1-macos-x86_64.tar.gz` |
| Windows x86_64 | `oratos-v0.3.1-windows-x86_64.zip` |

## macOS

**Homebrew** (when the tap is published):

```bash
brew install latentmeta/tap/oratos
```

Until then, use `install.sh` or download a macOS release asset.

## Windows

**Scoop** (when the bucket is published):

```powershell
scoop bucket add latentmeta https://github.com/latentmeta/scoop-bucket
scoop install oratos
```

Until then, download `oratos-v0.3.1-windows-x86_64.zip` from [Releases](https://github.com/latentmeta/oratos/releases), extract `oratos.exe`, and add it to `PATH`.

## Version managers

### mise (preferred)

```bash
mise use -g github:latentmeta/oratos
# or pin a version:
mise use -g github:latentmeta/oratos@0.3.1
```

In `mise.toml`:

```toml
[tools]
"github:latentmeta/oratos" = "latest"
```

If multiple assets match, narrow with `matching`:

```toml
[tools]
"github:latentmeta/oratos" = { version = "latest", matching = "oratos-" }
```

### asdf

```bash
asdf plugin add oratos https://github.com/latentmeta/asdf-oratos.git
# or from this monorepo: asdf plugin add oratos "${PWD}/packaging/asdf-oratos"
asdf install oratos 0.3.1
asdf global oratos 0.3.1
```

Plugin sources live under [`packaging/asdf-oratos`](../packaging/asdf-oratos).

## GitHub Actions

```yaml
- uses: latentmeta/oratos/.github/actions/setup-oratos@v0.3.1
  with:
    version: "0.3.1"   # omit for latest
- run: oratos audit ./dist --fail-under 85
```

Or copy [`.github/workflows/oratos-audit-example.yml`](../.github/workflows/oratos-audit-example.yml).

## Python (PyPI)

```bash
pip install oratos==0.3.1
oratos audit ./dist
```

Wheels ship the native binary (Ruff-style; no Python runtime dependency beyond the installer). See [`pyproject.toml`](../pyproject.toml).

## Node (npm)

```bash
npm install --save-dev oratos@0.3.1
npx oratos audit ./dist --fail-under 85
```

The package downloads the matching GitHub Release binary on `postinstall`. See [`packaging/npm`](../packaging/npm).

## Elixir / Mix (Hex)

```elixir
# mix.exs
defp deps do
  [
    {:oratos, "~> 0.3.1", only: [:dev, :test], runtime: false}
    # {:oratos, path: "packaging/hex", only: [:dev, :test], runtime: false}
  ]
end
```

```bash
mix deps.get
mix oratos.audit ./priv/static --fail-under 85
```

The Hex package **manages the Oratos CLI binary** (download into `priv/bin`); it is not an in-process BEAM HTML auditor. See [`packaging/hex`](../packaging/hex) and [Phoenix workflows](phoenix.md).

## Rust contributors

```bash
cargo install oratos --version 0.3.1
# from a checkout:
cargo install --path crates/oratos
```

## Verify

```bash
oratos --version
oratos audit examples/static_site
```
