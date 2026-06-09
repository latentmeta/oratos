use crate::core::{AuditReport, Category, Severity};

pub fn format_console(report: &AuditReport) -> String {
    let mut out = String::new();
    out.push_str(&format!("Oratos Audit Report v{}\n", report.core_version));
    out.push_str(&format!("Target: {}\n", report.target.path_or_url));
    out.push_str(&format!("Pages: {}\n\n", report.page_count));

    out.push_str("Scores:\n");
    out.push_str(&format!(
        "  Overall:         {:.1}\n",
        report.scores.overall
    ));
    out.push_str(&format!("  SEO:             {:.1}\n", report.scores.seo));
    out.push_str(&format!(
        "  Accessibility:   {:.1}\n",
        report.scores.accessibility
    ));
    out.push_str(&format!(
        "  Structured Data: {:.1}\n",
        report.scores.structured_data
    ));
    out.push_str(&format!(
        "  LLM Readiness:   {:.1}\n\n",
        report.scores.llm_readiness
    ));

    let errors = report
        .findings
        .iter()
        .filter(|f| f.severity == Severity::Error)
        .count();
    let warnings = report
        .findings
        .iter()
        .filter(|f| f.severity == Severity::Warning)
        .count();
    let infos = report
        .findings
        .iter()
        .filter(|f| f.severity == Severity::Info)
        .count();

    out.push_str(&format!(
        "Findings: {errors} errors, {warnings} warnings, {infos} info\n\n"
    ));

    for page in &report.pages {
        if page.findings.is_empty() {
            continue;
        }
        out.push_str(&format!("── {} ──\n", page.page.url_or_path));
        for finding in &page.findings {
            let sev = match finding.severity {
                Severity::Error => "ERROR",
                Severity::Warning => "WARN ",
                Severity::Info => "INFO ",
            };
            let cat = category_label(finding.category);
            out.push_str(&format!(
                "  [{sev}] [{cat}] {}: {}\n",
                finding.rule_id, finding.message
            ));
            if let Some(rec) = &finding.recommendation {
                out.push_str(&format!("         → {rec}\n"));
            }
        }
        out.push('\n');
    }

    out
}

fn category_label(cat: Category) -> &'static str {
    match cat {
        Category::Seo => "SEO",
        Category::Accessibility => "A11Y",
        Category::StructuredData => "DATA",
        Category::LlmReadiness => "LLM ",
        Category::PerformanceHint => "PERF",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        AuditReport, AuditTarget, Category, Finding, PageAudit, PageRef, Severity, TargetKind,
    };
    use insta::assert_snapshot;

    fn sample_report() -> AuditReport {
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
        AuditReport::new(
            AuditTarget {
                path_or_url: "./examples/static_site".into(),
                kind: TargetKind::Directory,
            },
            vec![page],
        )
    }

    #[test]
    fn console_output_snapshot() {
        assert_snapshot!("sample_console_report", format_console(&sample_report()));
    }
}
