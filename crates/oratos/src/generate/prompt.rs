use crate::core::{AuditReport, Finding, Severity};
use crate::html::HtmlPage;

/// Generate an LLM remediation prompt for a single HTML page.
pub fn generate_html_remediation_prompt(page: &HtmlPage, report: Option<&AuditReport>) -> String {
    let page_findings: Vec<&Finding> = report
        .map(|r| {
            r.pages
                .iter()
                .find(|p| p.page.url_or_path == page.url_or_path)
                .map(|p| p.findings.iter().collect())
                .unwrap_or_default()
        })
        .unwrap_or_default();

    let mut prompt = String::new();

    prompt.push_str("# HTML Remediation Task\n\n");
    prompt.push_str(
        "You are updating HTML for better SEO, accessibility, structured metadata, and LLM readiness. \
         Apply only the changes described below.\n\n",
    );

    prompt.push_str("## Page context\n\n");
    prompt.push_str(&format!("- **URL/path:** `{}`\n", page.url_or_path));
    if let Some(title) = &page.title {
        prompt.push_str(&format!("- **Current title:** {title}\n"));
    }
    if let Some(desc) = &page.meta_description {
        prompt.push_str(&format!("- **Current meta description:** {desc}\n"));
    }
    if let Some(lang) = &page.language {
        prompt.push_str(&format!("- **Language:** {lang}\n"));
    }
    if !page.headings.is_empty() {
        prompt.push_str("- **Headings:**\n");
        for h in &page.headings {
            prompt.push_str(&format!("  - h{}: {}\n", h.level, h.text));
        }
    }
    prompt.push_str(&format!(
        "- **Extractable text length:** {} characters\n\n",
        page.main_text.len()
    ));

    prompt.push_str("## Detected issues\n\n");
    if page_findings.is_empty() {
        prompt.push_str("_No audit findings for this page. Apply general best-practice improvements only where clearly beneficial._\n\n");
    } else {
        for f in &page_findings {
            let sev = match f.severity {
                Severity::Error => "ERROR",
                Severity::Warning => "WARNING",
                Severity::Info => "INFO",
            };
            prompt.push_str(&format!("- **[{sev}]** `{}`: {}\n", f.rule_id, f.message));
            if let Some(rec) = &f.recommendation {
                prompt.push_str(&format!("  - Required change: {rec}\n"));
            }
        }
        prompt.push('\n');
    }

    prompt.push_str("## Required changes\n\n");
    prompt.push_str(
        "1. Fix all ERROR-level issues.\n\
         2. Address WARNING-level issues where the fix is unambiguous.\n\
         3. Add or improve `<title>`, meta description, canonical URL, Open Graph, and Twitter card tags.\n\
         4. Add valid JSON-LD (`WebPage`, and `BreadcrumbList` / `ImageObject` where appropriate).\n\
         5. Improve `alt` text only when sufficiently inferable from context.\n\
         6. Ensure a single `<h1>` and logical heading hierarchy.\n\
         7. Add `<main>` landmark if missing.\n\
         8. Associate form inputs with labels where detectable.\n\n",
    );

    prompt.push_str("## Preservation rules (mandatory)\n\n");
    prompt.push_str(
        "- **Preserve all visible content.** Do not remove text, links, images, credits, dates, captions, or attribution.\n\
         - Do not change visual design unless required for semantic correctness.\n\
         - Keep changes minimal and reviewable.\n\
         - Mark uncertain generated text with an HTML comment: `<!-- REVIEW: ... -->`.\n\n",
    );

    prompt.push_str("## Accessibility rules\n\n");
    prompt.push_str(
        "- Set `lang` on `<html>` if missing.\n\
         - Every meaningful `<img>` must have descriptive `alt` (or `alt=\"\"` if decorative).\n\
         - Links and buttons must have accessible names.\n\
         - Use semantic landmarks (`<main>`, `<nav>`, etc.).\n\n",
    );

    prompt.push_str("## Structured data rules\n\n");
    prompt.push_str(
        "- Add JSON-LD in `<script type=\"application/ld+json\">` blocks.\n\
         - Prefer `@type: WebPage` for content pages.\n\
         - Add `BreadcrumbList` when breadcrumbs are present.\n\
         - Add `ImageObject` for key images.\n\
         - Ensure JSON is valid.\n\n",
    );

    prompt.push_str("## Expected output format\n\n");
    prompt.push_str(
        "1. Return the **complete updated HTML** file.\n\
         2. Also return a **unified diff** against the original.\n\
         3. **Explain each non-trivial change** in a short changelog after the diff.\n\
         4. Flag any text you generated that needs human review.\n\n",
    );

    prompt.push_str("## Original HTML\n\n");
    prompt.push_str("```html\n");
    prompt.push_str(&page.source);
    prompt.push_str("\n```\n");

    prompt
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html::parse_html;

    #[test]
    fn prompt_includes_preservation_rules() {
        let page = parse_html("/test.html", "<html><body><p>Hello</p></body></html>", true);
        let prompt = generate_html_remediation_prompt(&page, None);
        assert!(prompt.contains("Preserve all visible content"));
        assert!(prompt.contains("unified diff"));
    }
}
