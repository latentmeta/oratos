//! Git-based changed-only filtering for local audits.

use std::collections::HashSet;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};

/// Paths of changed files under `site_root` (relative to repo root), or None if git unavailable.
pub fn changed_files(site_root: &Path) -> Result<Option<HashSet<String>>> {
    let output = Command::new("git")
        .args([
            "diff",
            "--name-only",
            "HEAD",
            "--",
            site_root.to_str().unwrap_or("."),
        ])
        .output()
        .context("failed to run git")?;

    if !output.status.success() {
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut set = HashSet::new();
    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        set.insert(line.replace('\\', "/"));
    }
    Ok(Some(set))
}

pub fn page_matches_changed(page_path: &str, changed: &HashSet<String>, site_root: &Path) -> bool {
    let normalized = page_path.replace('\\', "/");
    if changed.contains(&normalized) {
        return true;
    }
    if let Ok(rel) = Path::new(&normalized)
        .strip_prefix(site_root)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
    {
        if changed.contains(&rel) {
            return true;
        }
    }
    for c in changed {
        if normalized.ends_with(c) || c.ends_with(&normalized) {
            return true;
        }
    }
    false
}

pub fn filter_pages_by_changed(
    pages: Vec<oratos::HtmlPage>,
    site_root: &Path,
) -> Result<Vec<oratos::HtmlPage>> {
    let Some(changed) = changed_files(site_root)? else {
        return Ok(pages);
    };
    if changed.is_empty() {
        return Ok(pages);
    }
    Ok(pages
        .into_iter()
        .filter(|p| page_matches_changed(&p.url_or_path, &changed, site_root))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn page_matches_changed_exact_and_suffix() {
        let root = Path::new("site");
        let mut changed = HashSet::new();
        changed.insert("site/index.html".into());
        assert!(page_matches_changed("site/index.html", &changed, root));
        assert!(page_matches_changed("/abs/site/index.html", &changed, root));
        assert!(!page_matches_changed("site/other.html", &changed, root));
    }

    #[test]
    fn filter_pages_passes_through_when_git_unavailable() {
        let pages = vec![oratos::parse_html(
            "index.html",
            "<html><body><p>x</p></body></html>",
            true,
        )];
        // Non-git path under /tmp usually returns None from changed_files -> all pages kept
        let root = Path::new("/tmp/oratos-non-git-test-dir");
        let filtered = filter_pages_by_changed(pages.clone(), root).unwrap();
        assert_eq!(filtered.len(), pages.len());
    }
}
