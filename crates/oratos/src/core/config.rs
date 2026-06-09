//! Project configuration (`oratos.toml`) for v0.2+.

use std::path::{Path, PathBuf};

use serde::Deserialize;

/// Top-level Oratos configuration file.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct OratosConfig {
    #[serde(default)]
    pub audit: AuditConfig,
    #[serde(default)]
    pub crawl: CrawlConfig,
}

/// Audit and reporting defaults.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AuditConfig {
    pub fail_under: Option<f64>,
    pub strict: Option<bool>,
    pub format: Option<String>,
    /// Rule IDs to suppress in reports and exit-code checks (e.g. `seo.missing-twitter-card`).
    #[serde(default)]
    pub ignore_rules: Vec<String>,
    /// When true, only audit HTML files changed in git (local targets).
    #[serde(default)]
    pub changed_only: bool,
}

/// HTTP crawl and discovery options (URL targets).
#[derive(Debug, Clone, Deserialize)]
pub struct CrawlConfig {
    /// Enable same-origin HTTP crawl (off by default for backward compatibility).
    #[serde(default)]
    pub enabled: bool,
    /// Maximum pages to fetch when crawling from a URL seed.
    #[serde(default = "default_max_pages")]
    pub max_pages: usize,
    /// Maximum link depth from the seed URL.
    #[serde(default = "default_max_depth")]
    pub max_depth: usize,
    /// Honor robots.txt disallow rules when crawling.
    #[serde(default = "default_true")]
    pub respect_robots: bool,
    /// Discover URLs from `sitemap.xml` at the site root when crawling.
    #[serde(default = "default_true")]
    pub use_sitemap: bool,
}

fn default_max_pages() -> usize {
    25
}

fn default_max_depth() -> usize {
    2
}

fn default_true() -> bool {
    true
}

impl Default for CrawlConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_pages: default_max_pages(),
            max_depth: default_max_depth(),
            respect_robots: true,
            use_sitemap: true,
        }
    }
}

impl OratosConfig {
    /// Load `oratos.toml` from `path` if it exists.
    pub fn load(path: &Path) -> Result<Option<Self>, ConfigError> {
        if !path.is_file() {
            return Ok(None);
        }
        let raw = std::fs::read_to_string(path).map_err(|e| ConfigError::Read {
            path: path.display().to_string(),
            source: e,
        })?;
        let cfg: OratosConfig = toml::from_str(&raw).map_err(|e| ConfigError::Parse {
            path: path.display().to_string(),
            source: e,
        })?;
        Ok(Some(cfg))
    }

    /// Search for `oratos.toml` starting at `start` and walking parent directories.
    pub fn discover(start: &Path) -> Result<Option<(PathBuf, Self)>, ConfigError> {
        let mut dir = if start.is_file() {
            start.parent().unwrap_or(start).to_path_buf()
        } else {
            start.to_path_buf()
        };
        loop {
            let candidate = dir.join("oratos.toml");
            if let Some(cfg) = Self::load(&candidate)? {
                return Ok(Some((candidate, cfg)));
            }
            if !dir.pop() {
                break;
            }
        }
        Ok(None)
    }
}

/// Remove ignored rule findings from a report (pages and top-level site findings).
pub fn apply_ignore_rules(report: &mut crate::AuditReport, ignore: &[String]) {
    if ignore.is_empty() {
        return;
    }
    for page in &mut report.pages {
        page.findings
            .retain(|f| !ignore.iter().any(|id| id == &f.rule_id));
        page.scores = crate::CategoryScores::from_findings(&page.findings);
    }
    report
        .findings
        .retain(|f| !ignore.iter().any(|id| id == &f.rule_id));
    report.scores = crate::CategoryScores::from_findings(&report.findings);
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to read config {path}: {source}")]
    Read {
        path: String,
        source: std::io::Error,
    },
    #[error("failed to parse config {path}: {source}")]
    Parse {
        path: String,
        source: toml::de::Error,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_ignore_rules_removes_findings() {
        use crate::core::{AuditReport, AuditTarget, Category, Finding, Severity, TargetKind};

        let mut report = AuditReport::new(
            AuditTarget {
                path_or_url: ".".into(),
                kind: TargetKind::Directory,
            },
            vec![],
        );
        report.findings.push(Finding::new(
            "seo.missing-title",
            Severity::Error,
            Category::Seo,
            "x",
        ));
        apply_ignore_rules(&mut report, &["seo.missing-title".to_string()]);
        assert!(report.findings.is_empty());
    }

    #[test]
    fn load_returns_none_for_missing_file() {
        assert!(OratosConfig::load(Path::new("/nonexistent/oratos.toml"))
            .unwrap()
            .is_none());
    }

    #[test]
    fn load_and_discover_from_temp_dir() {
        let dir = tempfile::tempdir().unwrap();
        let cfg_path = dir.path().join("oratos.toml");
        std::fs::write(
            &cfg_path,
            r#"
[audit]
fail_under = 90.0
"#,
        )
        .unwrap();
        let loaded = OratosConfig::load(&cfg_path).unwrap().unwrap();
        assert_eq!(loaded.audit.fail_under, Some(90.0));

        let nested = dir.path().join("nested");
        std::fs::create_dir(&nested).unwrap();
        let discovered = OratosConfig::discover(&nested).unwrap().unwrap();
        assert_eq!(discovered.1.audit.fail_under, Some(90.0));
    }

    #[test]
    fn load_rejects_invalid_toml() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("oratos.toml");
        std::fs::write(&path, "not = [valid").unwrap();
        assert!(OratosConfig::load(&path).is_err());
    }

    #[test]
    fn parses_minimal_config() {
        let raw = r#"
[audit]
fail_under = 85.0
ignore_rules = ["seo.missing-twitter-card"]
changed_only = true

[crawl]
max_pages = 10
"#;
        let cfg: OratosConfig = toml::from_str(raw).unwrap();
        assert_eq!(cfg.audit.fail_under, Some(85.0));
        assert_eq!(cfg.audit.ignore_rules.len(), 1);
        assert!(cfg.audit.changed_only);
        assert_eq!(cfg.crawl.max_pages, 10);
    }
}
