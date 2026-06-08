use std::collections::{HashSet, VecDeque};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use url::Url;
use walkdir::WalkDir;

use crate::extract::parse_html;
use crate::page::HtmlPage;
use crate::robots::{disallowed_paths, is_disallowed};
use crate::sitemap::urls_from_sitemap_xml;

#[derive(Debug, Clone, Default)]
pub struct LoadOptions {
    pub base_url: Option<String>,
    /// When set on URL targets, crawl same-origin HTML pages up to limits.
    pub crawl: Option<CrawlOptions>,
}

#[derive(Debug, Clone)]
pub struct CrawlOptions {
    pub max_pages: usize,
    pub max_depth: usize,
    pub respect_robots: bool,
    pub use_sitemap: bool,
}

pub async fn load_pages(target: &str, options: &LoadOptions) -> Result<Vec<HtmlPage>> {
    if target.starts_with("http://") || target.starts_with("https://") {
        if let Some(crawl) = &options.crawl {
            return load_url_crawl(target, crawl).await;
        }
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

fn load_file(path: &Path) -> Result<HtmlPage> {
    let source = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    let url_or_path = path.to_string_lossy().to_string();
    Ok(parse_html(&url_or_path, &source, true))
}

fn load_directory(dir: &Path, options: &LoadOptions) -> Result<Vec<HtmlPage>> {
    let mut pages = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
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
    let (source, normalized) = fetch_url(url).await?;
    Ok(parse_html(&normalized, &source, false))
}

async fn fetch_url(url: &str) -> Result<(String, String)> {
    let normalized_url = normalize_url(url).unwrap_or_else(|_| url.to_string());
    let parsed = Url::parse(&normalized_url).context("invalid URL")?;
    let client = http_client()?;
    let response = client
        .get(parsed.clone())
        .send()
        .await
        .context("HTTP request failed")?;
    let source = response
        .text()
        .await
        .context("failed to read response body")?;
    Ok((source, normalized_url))
}

async fn load_url_crawl(seed: &str, crawl: &CrawlOptions) -> Result<Vec<HtmlPage>> {
    let seed_url = normalize_url(seed).unwrap_or_else(|_| seed.to_string());
    let base = Url::parse(&seed_url).context("invalid seed URL")?;
    let client = http_client()?;

    let mut disallowed = Vec::new();
    if crawl.respect_robots {
        let robots_url = base.join("/robots.txt").unwrap_or_else(|_| base.clone());
        if let Ok(resp) = client.get(robots_url.clone()).send().await {
            if let Ok(body) = resp.text().await {
                disallowed = disallowed_paths(&body);
            }
        }
    }

    let mut queue: VecDeque<(String, u32)> = VecDeque::new();
    let mut seeds = vec![seed_url.clone()];

    if crawl.use_sitemap {
        let sitemap_url = base.join("/sitemap.xml").unwrap_or_else(|_| base.clone());
        if let Ok(resp) = client.get(sitemap_url).send().await {
            if let Ok(xml) = resp.text().await {
                if let Ok(mut from_map) = urls_from_sitemap_xml(&xml, &base) {
                    seeds.append(&mut from_map);
                }
            }
        }
    }

    for s in seeds {
        queue.push_back((s, 0));
    }

    let mut seen = HashSet::new();
    let mut pages = Vec::new();

    while let Some((url, depth)) = queue.pop_front() {
        if pages.len() >= crawl.max_pages {
            break;
        }
        if !seen.insert(url.clone()) {
            continue;
        }
        let parsed = match Url::parse(&url) {
            Ok(u) => u,
            Err(_) => continue,
        };
        if parsed.origin() != base.origin() {
            continue;
        }
        let path = parsed.path();
        if crawl.respect_robots && is_disallowed(path, &disallowed) {
            continue;
        }

        let (source, normalized) = match fetch_url(&url).await {
            Ok(v) => v,
            Err(_) => continue,
        };
        let page = parse_html(&normalized, &source, false);
        if depth < crawl.max_depth as u32 {
            for link in &page.links {
                if link.is_internal || same_origin(&base, &link.href) {
                    if let Some(next) = resolve_crawl_link(&base, &url, &link.href) {
                        queue.push_back((next, depth + 1));
                    }
                }
            }
        }
        pages.push(page);
    }

    pages.sort_by(|a, b| a.url_or_path.cmp(&b.url_or_path));
    Ok(pages)
}

fn same_origin(base: &Url, href: &str) -> bool {
    if href.starts_with("http://") || href.starts_with("https://") {
        Url::parse(href)
            .map(|u| u.origin() == base.origin())
            .unwrap_or(false)
    } else {
        !href.starts_with('#') && !href.starts_with("mailto:") && !href.starts_with("tel:")
    }
}

fn resolve_crawl_link(base: &Url, current: &str, href: &str) -> Option<String> {
    if href.starts_with('#') || href.is_empty() {
        return None;
    }
    let current_url = Url::parse(current).ok()?;
    let joined = current_url.join(href).ok()?;
    if joined.origin() != base.origin() {
        return None;
    }
    let path = joined.path().to_lowercase();
    if path.ends_with(".html")
        || path.ends_with(".htm")
        || path.ends_with('/')
        || !path.contains('.')
    {
        Some(joined.to_string())
    } else {
        None
    }
}

fn http_client() -> Result<reqwest::Client> {
    reqwest::Client::builder()
        .user_agent("oratos/0.2.0 (+https://github.com/latentmeta/oratos)")
        .build()
        .context("failed to build HTTP client")
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
