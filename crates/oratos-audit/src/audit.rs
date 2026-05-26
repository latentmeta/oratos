use std::path::Path;

use oratos_core::{AuditReport, CategoryScores, PageAudit, PageRef};
use oratos_html::HtmlPage;

use crate::rules::{all_rules, AuditContext};
use crate::target::resolve_target;

pub fn audit_pages(target: &str, pages: &[HtmlPage]) -> AuditReport {
    let audit_target = resolve_target(target);
    let site_root = if audit_target.kind == oratos_core::TargetKind::Directory {
        Some(target.to_string())
    } else {
        None
    };

    let known_paths: Vec<String> = pages
        .iter()
        .map(|p| p.url_or_path.replace('\\', "/"))
        .collect();

    let has_llms_txt = site_root
        .as_ref()
        .map(|root| Path::new(root).join("llms.txt").exists())
        .unwrap_or(false);

    let ctx = AuditContext {
        site_root,
        known_paths,
        has_llms_txt,
    };

    let rules = all_rules();
    let page_audits: Vec<PageAudit> = pages
        .iter()
        .map(|page| {
            let mut findings = Vec::new();
            for rule in &rules {
                findings.extend(rule.check(page, &ctx));
            }
            let scores = CategoryScores::from_findings(&findings);
            PageAudit {
                page: PageRef {
                    url_or_path: page.url_or_path.clone(),
                    title: page.title.clone(),
                },
                findings,
                scores,
            }
        })
        .collect();

    AuditReport::new(audit_target, page_audits)
}
