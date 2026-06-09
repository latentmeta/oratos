use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HtmlPage {
    pub url_or_path: String,
    pub source: String,
    pub title: Option<String>,
    pub meta_description: Option<String>,
    pub canonical_urls: Vec<String>,
    pub language: Option<String>,
    pub headings: Vec<Heading>,
    pub links: Vec<LinkInfo>,
    pub images: Vec<ImageInfo>,
    pub open_graph: OpenGraphTags,
    pub twitter_card: TwitterCardTags,
    pub json_ld_blocks: Vec<JsonLdBlock>,
    pub main_text: String,
    pub has_main_landmark: bool,
    pub forms: Vec<FormInfo>,
    pub breadcrumbs_detected: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Heading {
    pub level: u8,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkInfo {
    pub href: String,
    pub text: String,
    pub is_internal: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageInfo {
    pub src: String,
    pub alt: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub has_caption: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct OpenGraphTags {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub url: Option<String>,
    pub type_: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TwitterCardTags {
    pub card: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonLdBlock {
    pub raw: String,
    pub valid_json: bool,
    pub types: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormInfo {
    pub id: Option<String>,
    pub inputs_without_label: Vec<String>,
}

impl HtmlPage {
    pub fn display_name(&self) -> &str {
        self.title
            .as_deref()
            .filter(|t| !t.is_empty())
            .unwrap_or(&self.url_or_path)
    }
}
