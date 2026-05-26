use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use url::Url;
use walkdir::WalkDir;

use crate::extract::parse_html;
use crate::page::HtmlPage;

#[derive(Debug, Clone, Default)]
pub struct LoadOptions {
    pub base_url: Option<String>,
}

pub async fn load_pages(target: &str, options: &LoadOptions) -> Result<Vec<HtmlPage>> {
    if target.starts_with("http://") || target.starts_with("https://") {
        load_url(target).await.map(|p| vec![p])
    } else {
        let path = Path::new(target);
        if path.is_file() {
            load_file(path).map(|p| vec![p])
        } else if path.is_dir() {
            load_directory(path, options)
        } else {
            anyhow::bail!("target not found: {target}");
        }
    }
}

pub fn load_file(path: &Path) -> Result<HtmlPage> {
    let source = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    let url_or_path = path.to_string_lossy().to_string();
    Ok(parse_html(&url_or_path, &source, true))
}

fn load_directory(dir: &Path, options: &LoadOptions) -> Result<Vec<HtmlPage>> {
    let mut pages = Vec::new();
    for entry in WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && is_html_file(path) {
            let mut page = load_file(path)?;
            if let Some(base) = &options.base_url {
                page.url_or_path = file_to_url(path, dir, base);
            }
            pages.push(page);
        }
    }
    pages.sort_by(|a, b| a.url_or_path.cmp(&b.url_or_path));
    Ok(pages)
}

async fn load_url(url: &str) -> Result<HtmlPage> {
    let parsed = Url::parse(url).context("invalid URL")?;
    let client = reqwest::Client::builder()
        .user_agent("oratos/0.1.0 (+https://github.com/latentmeta/oratos)")
        .build()?;
    let response = client
        .get(parsed.clone())
        .send()
        .await
        .context("HTTP request failed")?;
    let source = response
        .text()
        .await
        .context("failed to read response body")?;
    Ok(parse_html(url, &source, false))
}

fn is_html_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| matches!(ext.to_lowercase().as_str(), "html" | "htm"))
}

fn file_to_url(path: &Path, base_dir: &Path, base_url: &str) -> String {
    let relative = path
        .strip_prefix(base_dir)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");
    let base = base_url.trim_end_matches('/');
    format!("{base}/{relative}")
}

pub fn resolve_internal_path(page_dir: &Path, href: &str) -> Option<PathBuf> {
    if href.starts_with("http://") || href.starts_with("https://") || href.starts_with('#') {
        return None;
    }
    let joined = page_dir.join(href.trim_start_matches('/'));
    if joined.exists() {
        Some(joined.canonicalize().unwrap_or(joined))
    } else {
        None
    }
}

pub fn normalize_url(url: &str) -> Result<String> {
    let parsed = Url::parse(url)?;
    let mut normalized = parsed.clone();
    normalized.set_fragment(None);
    Ok(normalized.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_url_fragments() {
        let n = normalize_url("https://example.com/page#section").unwrap();
        assert_eq!(n, "https://example.com/page");
    }
}
