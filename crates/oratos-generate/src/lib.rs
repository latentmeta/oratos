//! Recommendation and prompt generation for Oratos.

mod llm;
mod llms;
mod metadata;
mod prompt;
mod prompt_phoenix;

pub use llm::{LlmProvider, LlmProviderConfig, OllamaProvider, OpenAiCompatibleProvider};
pub use llms::generate_llms_txt;
pub use metadata::{generate_metadata_recommendations, MetadataRecommendation};
pub use prompt::generate_html_remediation_prompt;
pub use prompt_phoenix::generate_phoenix_remediation_prompt;
