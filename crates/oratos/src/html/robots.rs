//! Minimal robots.txt parsing (Disallow paths).

/// Paths or prefixes disallowed for a given user-agent block (simplified: merges all Disallow).
pub fn disallowed_paths(robots_txt: &str) -> Vec<String> {
    let mut paths = Vec::new();
    for line in robots_txt.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if let Some(rest) = line.strip_prefix("Disallow:") {
            let path = rest.trim();
            if !path.is_empty() {
                paths.push(path.to_string());
            }
        }
    }
    paths
}

/// Returns true if `url_path` should not be fetched per robots disallow rules.
pub fn is_disallowed(url_path: &str, disallowed: &[String]) -> bool {
    disallowed
        .iter()
        .any(|d| url_path.starts_with(d) || (d != "/" && url_path == d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_disallow() {
        let txt = "User-agent: *\nDisallow: /private/\nDisallow: /admin\n";
        let paths = disallowed_paths(txt);
        assert_eq!(paths.len(), 2);
        assert!(is_disallowed("/private/page", &paths));
        assert!(!is_disallowed("/public", &paths));
    }
}
