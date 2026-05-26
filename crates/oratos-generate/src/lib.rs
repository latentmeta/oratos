//! Recommendation and prompt generation for Oratos.

mod llms;
mod metadata;
mod prompt;

pub use llms::generate_llms_txt;
pub use metadata::{generate_metadata_recommendations, MetadataRecommendation};
pub use prompt::generate_html_remediation_prompt;
