use serde::{Deserialize, Serialize};

use crate::location::Location;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
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
        let rule_id = rule_id.into();
        debug_assert!(
            is_valid_rule_id(&rule_id),
            "rule_id should use dot-separated kebab-case segments (e.g. seo.title-too-short)"
        );
        Self {
            rule_id,
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

fn is_valid_rule_id(rule_id: &str) -> bool {
    let mut parts = rule_id.split('.');
    let Some(prefix) = parts.next() else {
        return false;
    };
    if prefix.is_empty()
        || !prefix
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
    {
        return false;
    }
    let Some(suffix) = parts.next() else {
        return true;
    };
    if parts.next().is_some() {
        return false;
    }
    !suffix.is_empty()
        && suffix
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}
