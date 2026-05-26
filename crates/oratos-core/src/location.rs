use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    pub page: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,
}

impl Location {
    pub fn page(page: impl Into<String>) -> Self {
        Self {
            page: page.into(),
            selector: None,
            line: None,
        }
    }

    pub fn with_selector(mut self, selector: impl Into<String>) -> Self {
        self.selector = Some(selector.into());
        self
    }
}
