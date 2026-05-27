use std::collections::HashSet;

use scraper::{ElementRef, Html, Selector};

use crate::page::{
    FormInfo, Heading, HtmlPage, ImageInfo, JsonLdBlock, LinkInfo, OpenGraphTags, TwitterCardTags,
};

pub fn parse_html(url_or_path: &str, source: &str, base_is_local: bool) -> HtmlPage {
    let document = Html::parse_document(source);

    let title = select_text(&document, "title");
    let meta_description = meta_content(&document, "description");
    let canonical_urls = select_attr_all(&document, "link[rel='canonical']", "href");
    let language = select_attr(&document, "html", "lang");

    let headings = extract_headings(&document);
    let links = extract_links(&document, base_is_local);
    let images = extract_images(&document);
    let open_graph = extract_open_graph(&document);
    let twitter_card = extract_twitter(&document);
    let json_ld_blocks = extract_json_ld(&document);
    let main_text = extract_main_text(&document);
    let has_main_landmark = has_element(&document, "main, [role='main']");
    let forms = extract_forms(&document);
    let breadcrumbs_detected = has_element(
        &document,
        "nav[aria-label*='breadcrumb' i], .breadcrumb, [itemtype*='BreadcrumbList']",
    );

    HtmlPage {
        url_or_path: url_or_path.to_string(),
        source: source.to_string(),
        title,
        meta_description,
        canonical_urls,
        language,
        headings,
        links,
        images,
        open_graph,
        twitter_card,
        json_ld_blocks,
        main_text,
        has_main_landmark,
        forms,
        breadcrumbs_detected,
    }
}

fn select_text(document: &Html, selector: &str) -> Option<String> {
    let sel = Selector::parse(selector).ok()?;
    document
        .select(&sel)
        .next()
        .map(|el| normalize_text(&el.text().collect::<String>()))
        .filter(|s| !s.is_empty())
}

fn select_attr(document: &Html, selector: &str, attr: &str) -> Option<String> {
    let sel = Selector::parse(selector).ok()?;
    document
        .select(&sel)
        .next()
        .and_then(|el| el.value().attr(attr).map(str::to_string))
        .filter(|s| !s.is_empty())
}

fn select_attr_all(document: &Html, selector: &str, attr: &str) -> Vec<String> {
    let Ok(sel) = Selector::parse(selector) else {
        return Vec::new();
    };
    document
        .select(&sel)
        .filter_map(|el| el.value().attr(attr).map(str::to_string))
        .collect()
}

fn meta_content(document: &Html, name: &str) -> Option<String> {
    let selector = format!("meta[name='{name}'], meta[property='{name}']");
    let Ok(sel) = Selector::parse(&selector) else {
        return None;
    };
    document
        .select(&sel)
        .next()
        .and_then(|el| {
            el.value()
                .attr("content")
                .or_else(|| el.value().attr("value"))
                .map(str::to_string)
        })
        .filter(|s| !s.is_empty())
}

fn extract_headings(document: &Html) -> Vec<Heading> {
    let Ok(sel) = Selector::parse("h1, h2, h3, h4, h5, h6") else {
        return Vec::new();
    };
    document
        .select(&sel)
        .filter_map(|el| {
            let tag = el.value().name();
            let level = tag.chars().nth(1)?.to_digit(10)? as u8;
            let text = normalize_text(&el.text().collect::<String>());
            if text.is_empty() {
                return None;
            }
            Some(Heading { level, text })
        })
        .collect()
}

fn extract_links(document: &Html, base_is_local: bool) -> Vec<LinkInfo> {
    let Ok(sel) = Selector::parse("a[href]") else {
        return Vec::new();
    };
    document
        .select(&sel)
        .filter_map(|el| {
            let href = el.value().attr("href")?.to_string();
            if href.starts_with('#') || href.starts_with("javascript:") {
                return None;
            }
            let text = normalize_text(&el.text().collect::<String>());
            let is_internal = is_internal_link(&href, base_is_local);
            Some(LinkInfo {
                href,
                text,
                is_internal,
            })
        })
        .collect()
}

fn is_internal_link(href: &str, base_is_local: bool) -> bool {
    if href.starts_with("http://") || href.starts_with("https://") {
        return false;
    }
    // Local audits: any non-http(s) href is site-relative.
    // Remote audits: only relative/protocol-relative paths count as internal.
    base_is_local || !href.starts_with("http")
}

fn extract_images(document: &Html) -> Vec<ImageInfo> {
    let Ok(sel) = Selector::parse("img") else {
        return Vec::new();
    };
    let caption_sel = Selector::parse("figcaption").ok();
    let has_figcaption = caption_sel
        .as_ref()
        .map(|s| document.select(s).next().is_some())
        .unwrap_or(false);

    document
        .select(&sel)
        .map(|el| {
            let src = el
                .value()
                .attr("src")
                .or_else(|| el.value().attr("data-src"))
                .unwrap_or("")
                .to_string();
            let alt = el.value().attr("alt").map(str::to_string);
            let width = el.value().attr("width").map(str::to_string);
            let height = el.value().attr("height").map(str::to_string);
            let in_figure = el
                .parent()
                .and_then(ElementRef::wrap)
                .map(|p| p.value().name() == "figure")
                .unwrap_or(false);
            ImageInfo {
                src,
                alt,
                width,
                height,
                has_caption: has_figcaption && in_figure,
            }
        })
        .collect()
}

fn extract_open_graph(document: &Html) -> OpenGraphTags {
    OpenGraphTags {
        title: meta_content(document, "og:title"),
        description: meta_content(document, "og:description"),
        image: meta_content(document, "og:image"),
        url: meta_content(document, "og:url"),
        type_: meta_content(document, "og:type"),
    }
}

fn extract_twitter(document: &Html) -> TwitterCardTags {
    TwitterCardTags {
        card: meta_content(document, "twitter:card"),
        title: meta_content(document, "twitter:title"),
        description: meta_content(document, "twitter:description"),
        image: meta_content(document, "twitter:image"),
    }
}

fn extract_json_ld(document: &Html) -> Vec<JsonLdBlock> {
    let Ok(sel) = Selector::parse("script[type='application/ld+json']") else {
        return Vec::new();
    };
    document
        .select(&sel)
        .map(|el| {
            let raw = el.text().collect::<String>().trim().to_string();
            let valid_json = serde_json::from_str::<serde_json::Value>(&raw).is_ok();
            let types = if valid_json {
                json_ld_types(&raw)
            } else {
                Vec::new()
            };
            JsonLdBlock {
                raw,
                valid_json,
                types,
            }
        })
        .filter(|b| !b.raw.is_empty())
        .collect()
}

fn json_ld_types(raw: &str) -> Vec<String> {
    let mut types = HashSet::new();
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(raw) {
        collect_types(&value, &mut types);
    }
    let mut v: Vec<_> = types.into_iter().collect();
    v.sort();
    v
}

fn collect_types(value: &serde_json::Value, types: &mut HashSet<String>) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::String(t)) = map.get("@type") {
                types.insert(t.clone());
            }
            for v in map.values() {
                collect_types(v, types);
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr {
                collect_types(v, types);
            }
        }
        _ => {}
    }
}

fn extract_main_text(document: &Html) -> String {
    if let Some(text) = text_from_selector(document, "main, [role='main'], article") {
        if text.len() > 50 {
            return text;
        }
    }
    text_from_selector(document, "body").unwrap_or_default()
}

fn text_from_selector(document: &Html, selector: &str) -> Option<String> {
    let sel = Selector::parse(selector).ok()?;
    let text: String = document.select(&sel).flat_map(|el| el.text()).collect();
    let normalized = normalize_text(&text);
    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
}

fn has_element(document: &Html, selector: &str) -> bool {
    Selector::parse(selector)
        .ok()
        .map(|sel| document.select(&sel).next().is_some())
        .unwrap_or(false)
}

fn extract_forms(document: &Html) -> Vec<FormInfo> {
    let Ok(form_sel) = Selector::parse("form") else {
        return Vec::new();
    };
    let input_sel = Selector::parse("input, textarea, select").ok();

    document
        .select(&form_sel)
        .map(|form| {
            let id = form.value().attr("id").map(str::to_string);
            let mut inputs_without_label = Vec::new();
            if let Some(ref input_sel) = input_sel {
                for input in form.select(input_sel) {
                    let input_type = input.value().attr("type").unwrap_or("text");
                    if matches!(input_type, "hidden" | "submit" | "button" | "reset") {
                        continue;
                    }
                    let id_attr = input.value().attr("id");
                    let has_label = id_attr.is_some_and(|id| form_has_label_for(&form, id))
                        || is_inside_label(&input);

                    if !has_label {
                        let name = input
                            .value()
                            .attr("name")
                            .or_else(|| input.value().attr("id"))
                            .unwrap_or("unnamed")
                            .to_string();
                        inputs_without_label.push(name);
                    }
                }
            }
            FormInfo {
                id,
                inputs_without_label,
            }
        })
        .collect()
}

fn form_has_label_for(form: &ElementRef<'_>, input_id: &str) -> bool {
    let Ok(label_sel) = Selector::parse("label[for]") else {
        return false;
    };
    form.select(&label_sel)
        .any(|label| label.value().attr("for") == Some(input_id))
}

fn is_inside_label(el: &ElementRef<'_>) -> bool {
    let mut current = el.parent();
    while let Some(node) = current {
        if let Some(parent_el) = ElementRef::wrap(node) {
            if parent_el.value().name() == "label" {
                return true;
            }
            current = parent_el.parent();
        } else {
            break;
        }
    }
    false
}

fn normalize_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_title_and_headings() {
        let html = r#"<!DOCTYPE html>
        <html lang="en">
        <head><title>Test Page</title>
        <meta name="description" content="A test page.">
        <link rel="canonical" href="https://example.com/">
        </head>
        <body>
        <h1>Hello</h1>
        <h2>World</h2>
        <p>Some content here for extraction.</p>
        </body></html>"#;
        let page = parse_html("/test.html", html, true);
        assert_eq!(page.title.as_deref(), Some("Test Page"));
        assert_eq!(page.meta_description.as_deref(), Some("A test page."));
        assert_eq!(page.headings.len(), 2);
        assert_eq!(page.language.as_deref(), Some("en"));
    }

    #[test]
    fn is_internal_link_classifies_hrefs() {
        assert!(!is_internal_link("https://example.com/", true));
        assert!(!is_internal_link("http://example.com/", false));

        assert!(is_internal_link("about.html", true));
        assert!(is_internal_link("/docs", true));

        assert!(is_internal_link("about.html", false));
        // Non-http(s) schemes are treated as internal on remote audits (no absolute URL).
        assert!(is_internal_link("ftp://files.example.com/x", false));
    }

    #[test]
    fn form_label_detection_handles_special_characters_in_id() {
        let html = r#"<!DOCTYPE html>
        <html lang="en"><body>
        <form>
          <label for="user's-name">Name</label>
          <input id="user's-name" name="name" type="text">
        </form>
        </body></html>"#;
        let page = parse_html("/test.html", html, true);
        assert!(page.forms[0].inputs_without_label.is_empty());
    }
}
