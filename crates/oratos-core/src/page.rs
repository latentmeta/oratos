use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageRef {
    pub url_or_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl PageRef {
    pub fn new(url_or_path: impl Into<String>) -> Self {
        Self {
            url_or_path: url_or_path.into(),
            title: None,
        }
    }
}
