use oratos_html::HtmlPage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataRecommendation {
    pub page: String,
    pub title: Option<String>,
    pub meta_description: Option<String>,
    pub canonical_url: Option<String>,
    pub open_graph: OpenGraphRecommendation,
    pub twitter_card: TwitterRecommendation,
    pub json_ld: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenGraphRecommendation {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TwitterRecommendation {
    pub card: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}

pub fn generate_metadata_recommendations(pages: &[HtmlPage]) -> Vec<MetadataRecommendation> {
    pages.iter().map(recommend_for_page).collect()
}

fn recommend_for_page(page: &HtmlPage) -> MetadataRecommendation {
    let title = page.title.clone().or_else(|| {
        page.headings
            .iter()
            .find(|h| h.level == 1)
            .map(|h| h.text.clone())
    });

    let meta_description = page
        .meta_description
        .clone()
        .or_else(|| Some(truncate(&page.main_text, 160)));

    let canonical_url = page
        .canonical_urls
        .first()
        .cloned()
        .or_else(|| Some(page.url_or_path.clone()));

    let og_title = page.open_graph.title.clone().or_else(|| title.clone());
    let og_description = page
        .open_graph
        .description
        .clone()
        .or_else(|| meta_description.clone());
    let og_image = page.open_graph.image.clone().or_else(|| {
        page.images
            .first()
            .map(|i| i.src.clone())
            .filter(|s| !s.is_empty())
    });

    let twitter_title = page.twitter_card.title.clone().or_else(|| title.clone());
    let twitter_description = page
        .twitter_card
        .description
        .clone()
        .or_else(|| meta_description.clone());

    let mut json_ld = Vec::new();
    json_ld.push(webpage_json_ld(
        page,
        &title,
        &meta_description,
        &canonical_url,
    ));
    if let Some(org) = organization_json_ld(page) {
        json_ld.push(org);
    }
    if page.breadcrumbs_detected {
        json_ld.push(breadcrumb_list_json_ld(page));
    }
    for img in page.images.iter().filter(|i| !i.src.is_empty()).take(3) {
        json_ld.push(image_object_json_ld(img));
    }

    let canonical_for_og = canonical_url.clone();
    MetadataRecommendation {
        page: page.url_or_path.clone(),
        title,
        meta_description,
        canonical_url,
        open_graph: OpenGraphRecommendation {
            title: og_title,
            description: og_description,
            image: og_image.clone(),
            url: canonical_for_og,
        },
        twitter_card: TwitterRecommendation {
            card: Some("summary_large_image".to_string()),
            title: twitter_title,
            description: twitter_description,
            image: page.twitter_card.image.clone().or(og_image),
        },
        json_ld,
    }
}

fn webpage_json_ld(
    page: &HtmlPage,
    title: &Option<String>,
    description: &Option<String>,
    url: &Option<String>,
) -> serde_json::Value {
    serde_json::json!({
        "@context": "https://schema.org",
        "@type": "WebPage",
        "name": title,
        "description": description,
        "url": url,
        "inLanguage": page.language
    })
}

fn organization_json_ld(page: &HtmlPage) -> Option<serde_json::Value> {
    if !page.url_or_path.contains("index") && page.url_or_path != "/" {
        return None;
    }
    Some(serde_json::json!({
        "@context": "https://schema.org",
        "@type": "Organization",
        "name": page.title,
        "url": page.canonical_urls.first().unwrap_or(&page.url_or_path)
    }))
}

fn breadcrumb_list_json_ld(page: &HtmlPage) -> serde_json::Value {
    let items: Vec<_> = page
        .headings
        .iter()
        .take(5)
        .enumerate()
        .map(|(i, h)| {
            serde_json::json!({
                "@type": "ListItem",
                "position": i + 1,
                "name": h.text
            })
        })
        .collect();

    serde_json::json!({
        "@context": "https://schema.org",
        "@type": "BreadcrumbList",
        "itemListElement": items
    })
}

fn image_object_json_ld(img: &oratos_html::ImageInfo) -> serde_json::Value {
    serde_json::json!({
        "@context": "https://schema.org",
        "@type": "ImageObject",
        "contentUrl": img.src,
        "description": img.alt
    })
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        let end = char_boundary_before(s, max.saturating_sub(1));
        format!("{}…", &s[..end])
    }
}

fn char_boundary_before(s: &str, max: usize) -> usize {
    if max >= s.len() {
        return s.len();
    }
    let mut boundary = 0usize;
    for (idx, _) in s.char_indices() {
        if idx > max {
            break;
        }
        boundary = idx;
    }
    boundary
}

#[cfg(test)]
mod tests {
    use super::truncate;

    #[test]
    fn truncate_handles_utf8_boundaries() {
        let s = "Page — metadata with multibyte text";
        let truncated = truncate(s, 10);
        assert!(truncated.ends_with('…'));
    }
}
