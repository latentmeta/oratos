# Oratos for Elixir & Phoenix

**Website visibility audits** for SEO, accessibility, structured data (JSON-LD), and LLM readiness — run from Mix against the HTML you actually ship.

Oratos is a **native CLI** managed by this Hex package. On first use it downloads a prebuilt binary from [GitHub Releases](https://github.com/latentmeta/oratos/releases). **No Rust toolchain** is required.

> This package does **not** audit HTML inside the BEAM, and it does **not** replace [`phoenix_seo`](https://hex.pm/packages/phoenix_seo) or your templates. Use those to *emit* metadata; use Oratos to *score the rendered output* (like Credo/Sobelow for visibility quality).

## Install

```elixir
# mix.exs — audit tooling only; omit from production releases
defp deps do
  [
    {:oratos, "~> 0.3.1", only: [:dev, :test], runtime: false}
  ]
end
```

```bash
mix deps.get
```

First `mix oratos.audit` (or `mix oratos --version`) downloads the CLI into the package `priv/bin/` directory.

## Phoenix workflow

Oratos audits **rendered HTML**, typically under `priv/static` after assets + prerender:

```bash
mix assets.deploy
mix phx.digest
mix phoenix.prerender   # or your static export step
mix oratos.audit ./priv/static --fail-under 85
```

If you skip prerender and only have LiveView HTML, Oratos can still audit any directory of `.html` files you produce (e.g. a `mix` task that writes snapshots). The CLI needs files on disk (or a URL with `--crawl`).

### Mix aliases

```elixir
# mix.exs
def project do
  [
    # ...
    aliases: aliases()
  ]
end

defp aliases do
  [
    # One string per Mix invocation so flags are args, not separate tasks
    "seo.audit": "oratos.audit priv/static --fail-under 85",
    "seo.report": "oratos.audit priv/static --format html --output reports/oratos.html",
    "seo.strict": "oratos.audit priv/static --strict --fail-under 85"
  ]
end
```

```bash
mix seo.audit
mix seo.report
```

## Mix tasks

### `mix oratos.audit`

Runs `oratos audit`. Prefer this in apps and CI.

```bash
# defaults to priv/static (or dist if present)
mix oratos.audit

mix oratos.audit ./priv/static --fail-under 85
mix oratos.audit ./priv/static --strict
mix oratos.audit ./priv/static --format sarif --output reports/oratos.sarif
mix oratos.audit ./priv/static --format json --output reports/oratos.json
mix oratos.audit ./priv/static --changed-only
mix oratos.audit ./priv/static --config oratos.toml
```

Useful flags:

| Flag | Purpose |
|------|---------|
| `--fail-under SCORE` | Non-zero exit if overall score &lt; SCORE (CI gate) |
| `--strict` | Fail on any warning or error finding |
| `--format console\|json\|markdown\|html\|sarif` | Report format |
| `--output PATH` | Write report to a file |
| `--changed-only` | Only HTML changed in `git diff HEAD` |
| `--config PATH` | Load `oratos.toml` (ignore rules, crawl, defaults) |
| `--crawl` | Multi-page crawl when the target is a URL |

### `mix oratos`

Forwards arguments to the CLI (downloads binary if needed):

```bash
mix oratos --version
mix oratos --help
mix oratos generate llms ./priv/static --output priv/static/llms.txt
mix oratos prompt phoenix priv/static/index.html --output tmp/remediate.md
mix oratos prompt html priv/static/about.html
```

## What Oratos checks

Scores are 0–100 per category (weighted overall: SEO 30%, Accessibility 25%, Structured Data 25%, LLM 20%).

- **SEO** — title, meta description, canonical, headings, Open Graph, Twitter cards, internal links
- **Accessibility** — `lang`, alt text, landmarks, form labels, link text
- **Structured data** — JSON-LD presence/validity, WebPage / breadcrumbs / Article / Organization hints
- **LLM readiness** — `llms.txt`, extractable text, summaries, image descriptions

Findings use stable `rule_id`s (see the [rule catalog](https://github.com/latentmeta/oratos/blob/main/docs/rules.md)).

## Configuration (`oratos.toml`)

Optional project file at the site root (or pass `--config`):

```toml
[audit]
fail_under = 85
strict = false
ignore_rules = ["seo.missing-twitter-card"]
changed_only = false

[crawl]
enabled = false
max_pages = 50
```

CLI flags override file values. Full schema: [configuration docs](https://github.com/latentmeta/oratos/blob/main/docs/configuration.md).

### Application config (this Hex package)

Controls **how the CLI binary is obtained**, not audit rules:

```elixir
# config/config.exs
config :oratos,
  # CLI release to download (without leading "v")
  version: "0.3.1",
  # Use an existing binary instead of downloading:
  # path: "/usr/local/bin/oratos",
  # Prefer `oratos` on PATH when present:
  prefer_path: false
```

## CI (GitHub Actions)

```yaml
- uses: actions/checkout@v4

- uses: erlef/setup-beam@v1
  with:
    elixir-version: "1.17"
    otp-version: "27"

- run: mix deps.get
- run: mix assets.deploy && mix phx.digest && mix phoenix.prerender

# Option A — Mix task (downloads CLI into deps/oratos/priv)
- run: mix oratos.audit ./priv/static --fail-under 85 --format sarif --output oratos.sarif

# Option B — install CLI on PATH (no Hex download step)
# - uses: latentmeta/oratos/.github/actions/setup-oratos@v0.3.1
# - run: oratos audit ./priv/static --fail-under 85

- uses: actions/upload-artifact@v4
  with:
    name: oratos-sarif
    path: oratos.sarif
```

## Relationship to `phoenix_seo`

| Tool | Role |
|------|------|
| **phoenix_seo** (and friends) | Help your app *render* titles, meta, JSON-LD |
| **Oratos** | Audit the *exported HTML* and gate CI on scores/findings |

Typical loop: improve metadata in Phoenix → prerender/export → `mix oratos.audit` → fix findings (or generate a remediation prompt with `mix oratos prompt phoenix …`).

## Programmatic access

```elixir
{:ok, path} = Oratos.ensure_binary()
System.cmd(path, ["audit", "priv/static", "--format", "json"], stderr_to_stdout: true)
```

## How the binary is obtained

This Hex package is a thin Mix wrapper. The first time you run `mix oratos` or
`mix oratos.audit`, it downloads a prebuilt `oratos` executable from GitHub
Releases into `deps/oratos/priv/bin/` (cached for later runs).

Fetch and unpack use [Req](https://hex.pm/packages/req) (`Req.Tar` / `Req.ZIP`) —
no `curl`, `tar`, or `unzip`. You only need **outbound HTTPS to GitHub** on that
first run (plus **git** if you use `--changed-only`, which diffs against `HEAD`).

Skip the download by pointing at a CLI you already installed (Homebrew,
`install.sh`, etc.):

```elixir
config :oratos, path: "/opt/homebrew/bin/oratos"
# or: prefer_path: true  # use `oratos` from PATH when available
```

## Further reading

- [Install matrix](https://github.com/latentmeta/oratos/blob/main/docs/install.md) (Homebrew, mise, pip, npm, …)
- [Phoenix workflows](https://github.com/latentmeta/oratos/blob/main/docs/phoenix.md)
- [CI/CD](https://github.com/latentmeta/oratos/blob/main/docs/ci.md)
- [Scoring](https://github.com/latentmeta/oratos/blob/main/docs/scoring.md)
- [GitHub](https://github.com/latentmeta/oratos) · [crates.io CLI](https://crates.io/crates/oratos)
