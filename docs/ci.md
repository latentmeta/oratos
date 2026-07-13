# CI/CD

Oratos is designed to run as a quality gate in CI. **Prefer installing a release binary** — do not require Rust in consumer pipelines.

Install options: [install.md](install.md).

## Official setup action

```yaml
- uses: actions/checkout@v4
- uses: latentmeta/oratos/.github/actions/setup-oratos@main
  with:
    version: "0.3.0"   # omit for latest
- run: oratos audit ./priv/static --fail-under 85
```

Example consumer workflow: [`.github/workflows/oratos-audit-example.yml`](../.github/workflows/oratos-audit-example.yml).

## Oratos project CI workflows

| Workflow | File | Purpose |
|----------|------|---------|
| **CI** | [`.github/workflows/ci.yml`](../.github/workflows/ci.yml) | Format, Clippy, cross-platform tests, coverage (90% line gate), `cargo audit`, release builds, crates.io dry-run |
| **Code Quality** | [`.github/workflows/code-quality.yml`](../.github/workflows/code-quality.yml) | Format, Clippy, `cargo doc` (warnings denied), `cargo deny` |
| **Dependencies** | [`.github/workflows/dependencies.yml`](../.github/workflows/dependencies.yml) | Weekly security audit; optional PRs for dependency updates |
| **Release** | [`.github/workflows/release.yml`](../.github/workflows/release.yml) | Multi-arch binaries + `SHA256SUMS`, GitHub Release, crates.io publish |
| **PyPI** | [`.github/workflows/publish-pypi.yml`](../.github/workflows/publish-pypi.yml) | maturin wheels / sdist |
| **Test Setup** | [`.github/workflows/test-setup.yml`](../.github/workflows/test-setup.yml) | Smoke test after workflow changes |

## Fail thresholds

Fail the job when the overall score drops below a threshold:

```bash
oratos audit ./priv/static --fail-under 85
```

Fail the job when any warnings or errors exist:

```bash
oratos audit ./priv/static --strict
```

## SARIF in CI

```bash
oratos audit ./priv/static --format sarif --output reports/oratos.sarif
```

Upload the report as a workflow artifact (or feed it to GitHub Code Scanning when configured):

```yaml
- uses: latentmeta/oratos/.github/actions/setup-oratos@main
- name: Oratos audit
  run: oratos audit ./priv/static --format sarif --output oratos.sarif

- name: Upload Oratos SARIF
  uses: actions/upload-artifact@v4
  with:
    name: oratos-sarif
    path: oratos.sarif
    if-no-files-found: error
```

For human-readable PR review, add a JSON or HTML report the same way:

```yaml
- name: Oratos HTML report
  run: oratos audit ./priv/static --format html --output oratos.html

- name: Upload Oratos HTML report
  uses: actions/upload-artifact@v4
  with:
    name: oratos-report
    path: oratos.html
```

## Coverage gate

The CI workflow enforces **90% line coverage** with `cargo-llvm-cov`:

```yaml
- name: Install cargo-llvm-cov
  uses: taiki-e/install-action@cargo-llvm-cov

- name: Coverage (minimum 90%)
  run: cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info --ignore-filename-regex 'crates/oratos/src/(main|changed)\.rs' --fail-under-lines 90
```

The CLI entrypoint files (`main.rs`, `changed.rs`) are excluded from this aggregate metric because they are exercised through process-level integration tests (`assert_cmd`) rather than in-process unit coverage.

## Release secrets

Tag pushes (`v*`) trigger the release workflow. Configure:

- `CRATES_IO_TOKEN` — API token for `cargo publish` (repository secret)
- PyPI publish uses Trusted Publishing (`id-token: write`) when the `pypi` environment is configured; otherwise the `publish-pypi` job may be skipped or fail until set up
