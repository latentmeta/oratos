use oratos_core::{AuditReport, Severity};

pub fn format_html(report: &AuditReport) -> String {
    let mut findings_html = String::new();
    for page in &report.pages {
        if page.findings.is_empty() {
            continue;
        }
        findings_html.push_str(&format!(
            "<section class=\"page\"><h2>{}</h2><ul>\n",
            html_escape(&page.page.url_or_path)
        ));
        for f in &page.findings {
            let class = match f.severity {
                Severity::Error => "error",
                Severity::Warning => "warning",
                Severity::Info => "info",
            };
            findings_html.push_str(&format!(
                "<li class=\"{class}\"><code>{}</code> — {}</li>\n",
                html_escape(&f.rule_id),
                html_escape(&f.message)
            ));
            if let Some(rec) = &f.recommendation {
                findings_html.push_str(&format!(
                    "<li class=\"recommendation\">→ {}</li>\n",
                    html_escape(rec)
                ));
            }
        }
        findings_html.push_str("</ul></section>\n");
    }

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Oratos Audit Report</title>
  <style>
    body {{ font-family: system-ui, sans-serif; max-width: 960px; margin: 2rem auto; padding: 0 1rem; }}
    h1 {{ border-bottom: 2px solid #333; }}
    .scores {{ display: grid; grid-template-columns: repeat(auto-fill, minmax(140px, 1fr)); gap: 1rem; }}
    .score {{ background: #f4f4f5; padding: 1rem; border-radius: 8px; text-align: center; }}
    .score .value {{ font-size: 2rem; font-weight: bold; }}
    .error {{ color: #b91c1c; }}
    .warning {{ color: #b45309; }}
    .info {{ color: #1d4ed8; }}
    .recommendation {{ color: #4b5563; font-style: italic; list-style: none; margin-left: 1.5rem; }}
    section.page {{ margin-top: 2rem; }}
  </style>
</head>
<body>
  <h1>Oratos Audit Report</h1>
  <p><strong>Target:</strong> {target}</p>
  <p><strong>Pages:</strong> {page_count}</p>
  <div class="scores">
    <div class="score"><div class="value">{overall:.0}</div>Overall</div>
    <div class="score"><div class="value">{seo:.0}</div>SEO</div>
    <div class="score"><div class="value">{a11y:.0}</div>Accessibility</div>
    <div class="score"><div class="value">{data:.0}</div>Structured Data</div>
    <div class="score"><div class="value">{llm:.0}</div>LLM Readiness</div>
  </div>
  <h2>Findings</h2>
  {findings}
</body>
</html>"#,
        target = html_escape(&report.target.path_or_url),
        page_count = report.page_count,
        overall = report.scores.overall,
        seo = report.scores.seo,
        a11y = report.scores.accessibility,
        data = report.scores.structured_data,
        llm = report.scores.llm_readiness,
        findings = if findings_html.is_empty() {
            "<p>No findings.</p>".to_string()
        } else {
            findings_html
        },
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
