# LLM providers (v0.3 preview)

Oratos v0.2 includes a **preview** LLM abstraction in `oratos::generate` (`LlmProvider` trait). It is **not** wired to the CLI yet.

## Planned configuration

```toml
[llm]
provider = "ollama"
base_url = "http://127.0.0.1:11434"
model = "llama3"
```

## Planned commands (v0.3)

- `oratos review alt` — suggest alt text for review (no auto-write)
- `oratos summarize` — draft page summaries for `llms.txt` assistance

## Design principles

- LLM features remain **optional**.
- Oratos never silently rewrites HTML files.
- Human review is required for generated copy.

See [roadmap.md](roadmap.md).
