# Publishing to crates.io

Oratos is published as a **single crate** on [crates.io](https://crates.io/crates/oratos). It ships both the `oratos` CLI binary and the library modules (`oratos::core`, `oratos::html`, etc.).

## Prerequisites

1. [crates.io](https://crates.io) account and API token:
   ```bash
   cargo login
   ```
2. Verify you can publish the `oratos` crate name (reserve on crates.io if needed).
3. All changes committed; version bumped in `[workspace.package]` and `Cargo.lock` updated.

## Dry run (recommended)

From the repository root:

```bash
./scripts/publish-crates.sh --dry-run
```

Or:

```bash
cargo publish -p oratos --dry-run --allow-dirty
```

## Publish

**Manual:**

```bash
./scripts/publish-crates.sh
```

**Automated:** pushing a `v*` tag runs [`.github/workflows/release.yml`](../.github/workflows/release.yml), which:

1. Builds multi-arch binaries and creates a GitHub Release (`SHA256SUMS`)
2. Publishes the Rust crate to crates.io (`CRATES_IO_TOKEN`)
3. Publishes the Mix wrapper from [`packaging/hex`](../packaging/hex) to Hex (`HEX_API_KEY`)

PyPI wheels are published by [`.github/workflows/publish-pypi.yml`](../.github/workflows/publish-pypi.yml) on the same tags.

## Hex.pm

The Mix wrapper in [`packaging/hex`](../packaging/hex) is published as `:oratos` on Hex when `HEX_API_KEY` is set. Create a key at https://hex.pm/dashboard/keys and add it as a repository secret.

Consumers:

```elixir
{:oratos, "~> 0.3.1", only: [:dev, :test], runtime: false}
```

Then `mix oratos.audit ./priv/static`. See [packaging/hex/README.md](../packaging/hex/README.md).

## After publish

Users can install the CLI with:

```bash
cargo install oratos
```

The library is available for programmatic use, e.g. `oratos = "0.3"` with `use oratos::{audit_pages, load_pages, ...}`.

## Version bumps

1. Update `version` in root `Cargo.toml` under `[workspace.package]`.
2. Run `cargo update -w` so the lockfile stays aligned.
3. Update `CHANGELOG.md` and release notes.
4. Tag `v*` and push (GitHub release workflow builds binaries).

## Notes

- Dev-only deps (`insta`, `wiremock`, etc.) are not published.
- `cargo install oratos` installs the `oratos` binary from the `oratos` package.
