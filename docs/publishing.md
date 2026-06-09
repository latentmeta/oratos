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

**Automated:** pushing a `v*` tag runs [`.github/workflows/release.yml`](../.github/workflows/release.yml), which builds release binaries and publishes to crates.io when `CRATES_IO_TOKEN` is configured as a repository secret.

## After publish

Users can install the CLI with:

```bash
cargo install oratos
```

The library is available for programmatic use, e.g. `oratos = "0.2"` with `use oratos::{audit_pages, load_pages, ...}`.

## Version bumps

1. Update `version` in root `Cargo.toml` under `[workspace.package]`.
2. Run `cargo update -w` so the lockfile stays aligned.
3. Update `CHANGELOG.md` and release notes.
4. Tag `v*` and push (GitHub release workflow builds binaries).

## Notes

- Dev-only deps (`insta`, `wiremock`, etc.) are not published.
- `cargo install oratos` installs the `oratos` binary from the `oratos` package.
