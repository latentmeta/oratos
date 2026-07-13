# asdf-oratos

asdf plugin that installs Oratos from [GitHub Releases](https://github.com/latentmeta/oratos/releases).

## Usage

```bash
asdf plugin add oratos https://github.com/latentmeta/asdf-oratos.git
# or from this monorepo while developing:
# asdf plugin add oratos "${PWD}/packaging/asdf-oratos"

asdf install oratos 0.3.0
asdf global oratos 0.3.0
oratos --version
```

No Rust toolchain required.
