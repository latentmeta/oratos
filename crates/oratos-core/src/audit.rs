use serde::{Deserialize, Serialize};

use crate::finding::Finding;
use crate::page::PageRef;
use crate::score::CategoryScores;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTarget {
    pub path_or_url: String,
    pub kind: TargetKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetKind {
    Directory,
    Url,
    File,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageAudit {
    pub page: PageRef,
    pub findings: Vec<Finding>,
    pub scores: CategoryScores,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditReport {
    pub version: String,
    pub target: AuditTarget,
    pub pages: Vec<PageAudit>,
    pub findings: Vec<Finding>,
    pub scores: CategoryScores,
    pub page_count: usize,
}

impl AuditReport {
    pub fn new(target: AuditTarget, pages: Vec<PageAudit>) -> Self {
        let findings: Vec<Finding> = pages.iter().flat_map(|p| p.findings.clone()).collect();
        let scores = CategoryScores::from_findings(&findings);
        let page_count = pages.len();

        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            target,
            pages,
            findings,
            scores,
            page_count,
        }
    }
}
