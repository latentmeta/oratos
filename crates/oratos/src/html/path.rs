//! Platform-agnostic path helpers for page URLs and local file paths.

/// Normalize path separators to forward slashes for cross-platform checks.
pub fn normalize_path_separators(path: &str) -> String {
    path.replace('\\', "/")
}

/// Whether a page path refers to a site root document (`index.html`, `/`, etc.).
pub fn is_site_root_path(url_or_path: &str) -> bool {
    let p = normalize_path_separators(url_or_path);
    p.ends_with("/index.html")
        || p.ends_with("/index.htm")
        || p.ends_with('/')
        || p == "index.html"
        || p == "index.htm"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn site_root_unix_style() {
        assert!(is_site_root_path("/tmp/site/index.html"));
        assert!(is_site_root_path("https://example.com/"));
    }

    #[test]
    fn site_root_windows_style() {
        assert!(is_site_root_path(r"C:\Users\Temp\site\index.html"));
        assert!(is_site_root_path(r"C:\site\index.htm"));
    }

    #[test]
    fn site_root_rejects_nested_pages() {
        assert!(!is_site_root_path("/blog/post.html"));
        assert!(!is_site_root_path(r"C:\site\blog\post.html"));
    }
}
