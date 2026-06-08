//! LLM provider abstraction (v0.3 foundation).

use anyhow::Result;

/// Optional LLM backend for captioning, alt-text review, and summaries.
pub trait LlmProvider: Send + Sync {
    fn id(&self) -> &'static str;

    /// Complete a prompt and return model text (no streaming in v0.3 preview).
    fn complete(&self, prompt: &str) -> Result<String>;
}

/// Configuration for an LLM provider (stored in future `oratos.toml` `[llm]` section).
#[derive(Debug, Clone)]
pub struct LlmProviderConfig {
    pub provider: String,
    pub base_url: Option<String>,
    pub model: Option<String>,
    pub api_key_env: Option<String>,
}

/// Placeholder OpenAI-compatible provider (not wired to CLI in v0.3 preview).
pub struct OpenAiCompatibleProvider {
    pub base_url: String,
    pub model: String,
}

impl LlmProvider for OpenAiCompatibleProvider {
    fn id(&self) -> &'static str {
        "openai-compatible"
    }

    fn complete(&self, _prompt: &str) -> Result<String> {
        anyhow::bail!(
            "LLM provider support is preview-only in v0.3; configure in a future release"
        )
    }
}

/// Placeholder Ollama provider.
pub struct OllamaProvider {
    pub base_url: String,
    pub model: String,
}

impl LlmProvider for OllamaProvider {
    fn id(&self) -> &'static str {
        "ollama"
    }

    fn complete(&self, _prompt: &str) -> Result<String> {
        anyhow::bail!("Ollama provider is preview-only in v0.3")
    }
}
