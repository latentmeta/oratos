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
4. Publishes `@latentmeta/oratos` to npm (`NPM_TOKEN`)

PyPI wheels are published by [`.github/workflows/publish-pypi.yml`](../.github/workflows/publish-pypi.yml) on `v*` tags (and via **Actions → Publish PyPI wheels → Run workflow**).

## PyPI (`oratos`)

Wheels / sdist are built with [maturin](https://www.maturin.rs/) (`bindings = "bin"`) from the root [`pyproject.toml`](../pyproject.toml). The CLI binary is inside the wheel — no Rust on the installer machine.

Publishing uses **Trusted Publishing** (OIDC). There is no `PYPI_API_TOKEN` secret; the workflow has `id-token: write` and uses the GitHub Environment named **`pypi`**.

### One-time Trusted Publisher setup

1. Create / sign in to a [PyPI](https://pypi.org) account (and enable 2FA).
2. Open [Publishing settings](https://pypi.org/manage/account/publishing/) (or project publishing once it exists).
3. Under **pending publishers** (first upload) or the project’s publishers, add:
   - **Owner:** `latentmeta`
   - **Repository:** `oratos`
   - **Workflow name:** `publish-pypi.yml`
   - **Environment name:** `pypi`
4. Confirm the GitHub repo already has Environment **`pypi`**  
   (`Settings → Environments` on `latentmeta/oratos` — already created for Oratos).

Do **not** use a long-lived PyPI API token unless Trusted Publishing is unavailable.

### First publish (or republish after fixing Trusted Publishing)

After the pending publisher is saved on PyPI:

```bash
# from GitHub UI: Actions → "Publish PyPI wheels" → Run workflow
# or:
gh workflow run publish-pypi.yml --repo latentmeta/oratos
```

Or push a new `v*` tag. The publish job fails loudly if OIDC / publisher config is wrong (it no longer uses `continue-on-error`).

The sdist must ship `LICENSE` (`license-files` + `[tool.maturin] include` in `pyproject.toml`). Without it PyPI returns `400 License-File LICENSE does not exist`. Republishes use `skip-existing: true` so already-uploaded wheels are skipped.

### Consumers

```bash
pip install oratos
oratos audit ./dist --fail-under 85
```

## npm (`@latentmeta/oratos`)

Package sources: [`packaging/npm`](../packaging/npm). Scope: **`@latentmeta/oratos`** (public).

### Create a publish token

1. Open the [latentmeta npm org](https://www.npmjs.com/org/latentmeta) → **Access Tokens** (or [your tokens](https://www.npmjs.com/settings/~/tokens)).
2. **Generate New Token** → **Granular Access Token**.
3. Required settings:
   - Packages / scopes: **read and write** on **`@latentmeta`**
   - Organizations: **read and write** on **`latentmeta`**
   - Security: **bypass 2FA** (automation) **on**
4. Copy the token once (it starts with `npm_`). Store it only in a password manager or GitHub Actions secrets — never commit it.

Add it as repository secret `NPM_TOKEN` on `latentmeta/oratos`:

```bash
gh secret set NPM_TOKEN --repo latentmeta/oratos
# paste the token when prompted
```

On each `v*` tag, the release workflow runs `npm publish --access public` from `packaging/npm` using `NODE_AUTH_TOKEN` ← `NPM_TOKEN`.

### Manual publish

`export NPM_TOKEN=…` alone is **not** enough. `npm publish` does not read that env var unless `.npmrc` references it. Without `.npmrc`, npm keeps using an old `npm login` session and you get:

```text
403 Forbidden - Two-factor authentication or granular access token with
bypass 2fa enabled is required to publish packages.
```

Correct one-off publish:

```bash
cd packaging/npm

export NPM_TOKEN='npm_…'   # granular token with bypass 2FA

# npm only picks up NPM_TOKEN via .npmrc — do not commit this file
printf '%s\n' '//registry.npmjs.org/:_authToken=${NPM_TOKEN}' > .npmrc

npm whoami
npm publish --access public
rm .npmrc
```

Alternative without an env var:

```bash
cd packaging/npm
npm config set //registry.npmjs.org/:_authToken "npm_…"
npm publish --access public
npm config delete //registry.npmjs.org/:_authToken
```

### If a token was exposed

If a token appears in chat, screenshots, CI logs, or a committed `.npmrc`:

1. **Revoke** it immediately on npm.
2. Create a new granular token (same settings as above).
3. Update `NPM_TOKEN` on GitHub with `gh secret set NPM_TOKEN --repo latentmeta/oratos`.

### Consumers

```bash
npm install --save-dev @latentmeta/oratos
npx oratos audit ./dist --fail-under 85
```

## Hex.pm

The Mix wrapper in [`packaging/hex`](../packaging/hex) is published as `:oratos` on Hex when `HEX_API_KEY` is set. Create a key at https://hex.pm/dashboard/keys and add it as a repository secret.

Consumers:

```elixir
{:oratos, "~> 0.3.2", only: [:dev, :test], runtime: false}
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
