# Remediation prompts

Oratos can generate an LLM remediation prompt for a page:

```bash
oratos prompt html ./priv/static/index.html --output remediate.prompt.md
```

The generated prompt includes:

- page context
- detected issues
- preservation rules (must preserve all visible content)
- accessibility and structured data requirements
- an expected output format (including a unified diff)

