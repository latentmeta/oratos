//! Phoenix / Mix-oriented remediation prompts (v0.4 foundation).

use oratos_core::AuditReport;
use oratos_html::HtmlPage;

/// Generate a remediation prompt tuned for Phoenix static export layouts (`priv/static`).
pub fn generate_phoenix_remediation_prompt(
    page: &HtmlPage,
    report: Option<&AuditReport>,
) -> String {
    let mut prompt = String::from(
        "# Phoenix HTML remediation task\n\n\
         You are updating a **Phoenix static export** HTML file (typically under `priv/static/`). \
         Apply SEO, accessibility, structured data, and LLM-readiness fixes without changing visible content.\n\n\
         ## Phoenix context\n\n\
         - Prefer `<main>` wrapping primary LiveView-rendered content.\n\
         - Keep asset paths compatible with `phx-track-static` and `/assets` URLs.\n\
         - Do not modify HEEx source; only the exported HTML artifact.\n\
         - After fixes, the page should pass `oratos audit` with fewer findings.\n\n",
    );

    prompt.push_str("## Page\n\n");
    prompt.push_str(&format!("- **Path:** `{}`\n", page.url_or_path));
    if let Some(title) = &page.title {
        prompt.push_str(&format!("- **Title:** {title}\n"));
    }

    if let Some(report) = report {
        if let Some(page_audit) = report
            .pages
            .iter()
            .find(|p| p.page.url_or_path == page.url_or_path)
        {
            prompt.push_str("\n## Oratos findings\n\n");
            for f in &page_audit.findings {
                prompt.push_str(&format!("- `{}` — {}\n", f.rule_id, f.message));
            }
        }
    }

    prompt.push_str("\n## Output\n\nReturn the full updated HTML and a unified diff.\n");
    prompt
}

#[cfg(test)]
mod tests {
    use super::*;
    use oratos_html::parse_html;

    #[test]
    fn phoenix_prompt_mentions_priv_static() {
        let page = parse_html(
            "/priv/static/index.html",
            "<html><body></body></html>",
            true,
        );
        let prompt = generate_phoenix_remediation_prompt(&page, None);
        assert!(prompt.contains("priv/static"));
    }

    #[test]
    fn phoenix_prompt_includes_findings_from_report() {
        use oratos_core::{
            AuditReport, AuditTarget, Category, Finding, PageAudit, PageRef, Severity, TargetKind,
        };

        let page = parse_html(
            "/priv/static/index.html",
            "<html><head><title>T</title></head><body></body></html>",
            true,
        );
        let finding = Finding::new(
            "seo.missing-title",
            Severity::Error,
            Category::Seo,
            "missing",
        );
        let report = AuditReport::new(
            AuditTarget {
                path_or_url: ".".into(),
                kind: TargetKind::Directory,
            },
            vec![PageAudit {
                page: PageRef {
                    url_or_path: page.url_or_path.clone(),
                    title: page.title.clone(),
                },
                findings: vec![finding],
                scores: oratos_core::CategoryScores::from_findings(&[]),
            }],
        );
        let prompt = generate_phoenix_remediation_prompt(&page, Some(&report));
        assert!(prompt.contains("seo.missing-title"));
        assert!(prompt.contains("Oratos findings"));
    }
}
