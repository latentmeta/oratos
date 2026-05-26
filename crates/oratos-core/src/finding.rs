use serde::{Deserialize, Serialize};

use crate::location::Location;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Seo,
    Accessibility,
    StructuredData,
    LlmReadiness,
    PerformanceHint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    pub rule_id: String,
    pub severity: Severity,
    pub category: Category,
    pub message: String,
    pub location: Option<Location>,
    pub recommendation: Option<String>,
    pub docs_url: Option<String>,
}

impl Finding {
    pub fn new(
        rule_id: impl Into<String>,
        severity: Severity,
        category: Category,
        message: impl Into<String>,
    ) -> Self {
        Self {
            rule_id: rule_id.into(),
            severity,
            category,
            message: message.into(),
            location: None,
            recommendation: None,
            docs_url: None,
        }
    }

    pub fn with_location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    pub fn with_recommendation(mut self, recommendation: impl Into<String>) -> Self {
        self.recommendation = Some(recommendation.into());
        self
    }

    pub fn with_docs_url(mut self, url: impl Into<String>) -> Self {
        self.docs_url = Some(url.into());
        self
    }
}
