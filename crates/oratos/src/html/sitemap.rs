//! Parse sitemap.xml for URL discovery.

use anyhow::{Context, Result};
use url::Url;

/// Collect page URLs from a sitemap XML document (urlset or sitemap index, one level).
pub fn urls_from_sitemap_xml(xml: &str, base: &Url) -> Result<Vec<String>> {
    let mut urls = Vec::new();
    let mut reader = quick_xml::Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut in_loc = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(quick_xml::events::Event::Start(e)) if e.name().as_ref() == b"loc" => {
                in_loc = true;
            }
            Ok(quick_xml::events::Event::Text(e)) if in_loc => {
                let text = e
                    .xml10_content()
                    .context("invalid sitemap xml")?
                    .into_owned();
                let loc = text.trim();
                if !loc.is_empty() {
                    urls.push(resolve_sitemap_loc(base, loc));
                }
                in_loc = false;
            }
            Ok(quick_xml::events::Event::End(e)) if e.name().as_ref() == b"loc" => {
                in_loc = false;
            }
            Ok(quick_xml::events::Event::Eof) => break,
            Err(e) => return Err(e.into()),
            _ => {}
        }
        buf.clear();
    }

    urls.sort();
    urls.dedup();
    Ok(urls)
}

fn resolve_sitemap_loc(base: &Url, loc: &str) -> String {
    Url::parse(loc).map(|u| u.to_string()).unwrap_or_else(|_| {
        base.join(loc.trim_start_matches('/'))
            .unwrap_or_else(|_| base.clone())
            .to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_urlset() {
        let base = Url::parse("https://example.com/").unwrap();
        let xml = r#"<?xml version="1.0"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url><loc>https://example.com/a</loc></url>
  <url><loc>/b</loc></url>
</urlset>"#;
        let urls = urls_from_sitemap_xml(xml, &base).unwrap();
        assert!(urls.contains(&"https://example.com/a".to_string()));
        assert!(urls.iter().any(|u| u.ends_with("/b")));
    }
}
