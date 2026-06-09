use crate::core::{AuditReport, Severity};

pub fn format_markdown(report: &AuditReport) -> String {
    let mut out = String::new();
    out.push_str("# Oratos Audit Report\n\n");
    out.push_str(&format!("**Core Version:** {}\n\n", report.core_version));
    out.push_str(&format!("**Target:** `{}`\n\n", report.target.path_or_url));
    out.push_str(&format!("**Pages audited:** {}\n\n", report.page_count));

    out.push_str("## Scores\n\n");
    out.push_str("| Category | Score |\n");
    out.push_str("|----------|-------|\n");
    out.push_str(&format!("| Overall | {:.1} |\n", report.scores.overall));
    out.push_str(&format!("| SEO | {:.1} |\n", report.scores.seo));
    out.push_str(&format!(
        "| Accessibility | {:.1} |\n",
        report.scores.accessibility
    ));
    out.push_str(&format!(
        "| Structured Data | {:.1} |\n",
        report.scores.structured_data
    ));
    out.push_str(&format!(
        "| LLM Readiness | {:.1} |\n\n",
        report.scores.llm_readiness
    ));

    out.push_str("## Findings\n\n");
    if report.findings.is_empty() {
        out.push_str("_No findings._\n");
        return out;
    }

    for page in &report.pages {
        if page.findings.is_empty() {
            continue;
        }
        out.push_str(&format!("### {}\n\n", page.page.url_or_path));
        for f in &page.findings {
            let badge = match f.severity {
                Severity::Error => "Error",
                Severity::Warning => "Warning",
                Severity::Info => "Info",
            };
            out.push_str(&format!(
                "- **{badge}** `{rule}` — {msg}\n",
                rule = f.rule_id,
                msg = f.message
            ));
            if let Some(rec) = &f.recommendation {
                out.push_str(&format!("  - Recommendation: {rec}\n"));
            }
        }
        out.push('\n');
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{AuditReport, AuditTarget, TargetKind};

    #[test]
    fn markdown_contains_title() {
        let report = AuditReport::new(
            AuditTarget {
                path_or_url: "./test".into(),
                kind: TargetKind::Directory,
            },
            vec![],
        );
        let md = format_markdown(&report);
        assert!(md.contains("# Oratos Audit Report"));
        assert!(md.contains("## Scores"));
    }

    #[test]
    fn markdown_output_snapshot() {
        use insta::assert_snapshot;
        use crate::core::{
            AuditReport, AuditTarget, Category, Finding, PageAudit, PageRef, Severity, TargetKind,
        };

        let finding = Finding::new(
            "seo.missing-title",
            Severity::Error,
            Category::Seo,
            "Page is missing a title.",
        );
        let page = PageAudit {
            page: PageRef::new("/index.html"),
            findings: vec![finding],
            scores: crate::core::CategoryScores::from_findings(&[]),
        };
        let report = AuditReport::new(
            AuditTarget {
                path_or_url: "./examples/static_site".into(),
                kind: TargetKind::Directory,
            },
            vec![page],
        );
        assert_snapshot!("sample_markdown_report", format_markdown(&report));
    }
}
