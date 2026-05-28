# Publishing to crates.io

Oratos is published as a **workspace of six crates**. The user-facing binary is the [`oratos`](https://crates.io/crates/oratos) package (directory `crates/oratos-cli`).

## Prerequisites

1. [crates.io](https://crates.io) account and API token:
   ```bash
   cargo login
   ```
2. Verify you can publish the `oratos` crate name (reserve on crates.io if needed).
3. All changes committed; version bumped in `[workspace.package]` and `Cargo.lock` updated.

## Publish order

Internal path dependencies require publishing **dependencies first**:

| Order | Package | Purpose |
|------|---------|---------|
| 1 | `oratos-core` | Shared types and scoring |
| 2 | `oratos-html` | HTML loading |
| 3 | `oratos-report` | Report formatters |
| 4 | `oratos-audit` | Audit rules |
| 5 | `oratos-generate` | Generators and prompts |
| 6 | `oratos` | CLI binary (`oratos` command) |

## Dry run (recommended)

From the repository root:

```bash
./scripts/publish-crates.sh --dry-run
```

This runs `cargo package --no-verify` for each crate (upload is skipped; verification against crates.io is skipped until dependencies are published).

Or validate the first crate end-to-end:

```bash
cargo publish -p oratos-core --dry-run --allow-dirty
```

## Publish

```bash
./scripts/publish-crates.sh
```

Wait ~30–60 seconds between publishes so crates.io indexes each version before dependents resolve it.

## After publish

Users can install the CLI with:

```bash
cargo install oratos
```

Library crates are available for programmatic use, e.g. `oratos-audit = "0.1"`.

## Version bumps

1. Update `version` in root `Cargo.toml` under `[workspace.package]`.
2. Run `cargo update -w` so lockfile and workspace dependency versions stay aligned.
3. Update `CHANGELOG.md` and release notes.
4. Tag `v*` and push (GitHub release workflow builds binaries).

## Notes

- Workspace internal deps use `{ path = "...", version = "0.1.0" }` so published crates resolve from crates.io.
- Dev-only crates (`insta`, `wiremock`, etc.) are not published.
- `cargo install oratos` installs the `oratos` binary from the `oratos` package.
