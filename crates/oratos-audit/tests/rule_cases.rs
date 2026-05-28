//! Per-rule regression tests: minimal HTML in, expected rule_id out.

use std::path::PathBuf;

use oratos_audit::audit_pages;
use oratos_html::{load_pages, parse_html, LoadOptions};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture(rel: &str) -> PathBuf {
    workspace_root().join(rel)
}

fn audit_html_in_temp_dir(html: &str, page_name: &str) -> oratos_core::AuditReport {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join(page_name);
    let url_or_path = path.to_string_lossy().to_string();
    let page = parse_html(&url_or_path, html, true);
    let target = dir.path().to_string_lossy().to_string();
    let _guard = dir;
    audit_pages(&target, std::slice::from_ref(&page))
}

fn has_rule(report: &oratos_core::AuditReport, rule_id: &str) -> bool {
    report.findings.iter().any(|f| f.rule_id == rule_id)
}

fn assert_rule(html: &str, rule_id: &str) {
    let report = audit_html_in_temp_dir(html, "index.html");
    assert!(
        has_rule(&report, rule_id),
        "expected rule {rule_id} for html snippet"
    );
}

fn assert_no_rule(html: &str, rule_id: &str) {
    let report = audit_html_in_temp_dir(html, "index.html");
    assert!(!has_rule(&report, rule_id), "did not expect rule {rule_id}");
}

const BASE_HEAD: &str = r#"<!DOCTYPE html><html lang="en"><head>"#;
const BASE_TAIL: &str = r#"</head><body><main><h1>Title</h1><p>Enough body text here for extraction and LLM checks with multiple sentences included.</p><img src="x.jpg" alt="x" width="1" height="1"><a href="x">link</a></main></body></html>"#;

/// Head markup that clears most SEO / structured / LLM checks; tweak per test.
const GOOD_HEAD: &str = r#"<title>Good Page Title For Testing Purposes</title><meta name="description" content="Desc long enough for tests and validation purposes here with plenty of characters."><link rel="canonical" href="https://example.com/"><meta property="og:title" content="t"><meta property="og:description" content="d"><meta property="og:image" content="https://example.com/i.jpg"><meta name="twitter:card" content="summary"><script type="application/ld+json">{"@context":"https://schema.org","@type":"WebPage","name":"Test"}</script>"#;

fn wrap_head_extra(extra: &str) -> String {
    format!("{BASE_HEAD}{extra}{BASE_TAIL}")
}

fn good_page() -> String {
    wrap_head_extra(GOOD_HEAD)
}

#[test]
fn seo_missing_title() {
    let html = wrap_head_extra(
        r#"<meta name="description" content="Desc long enough for tests and validation purposes here."><link rel="canonical" href="https://example.com/"><meta property="og:title" content="t"><meta property="og:description" content="d"><meta property="og:image" content="https://example.com/i.jpg"><meta name="twitter:card" content="summary">"#,
    );
    assert_rule(&html, "seo.missing-title");
}

#[test]
fn seo_title_too_short() {
    let html = wrap_head_extra(
        r#"<title>Hi</title><meta name="description" content="Desc long enough for tests and validation purposes here."><link rel="canonical" href="https://example.com/"><meta property="og:title" content="t"><meta property="og:description" content="d"><meta property="og:image" content="https://example.com/i.jpg"><meta name="twitter:card" content="summary">"#,
    );
    assert_rule(&html, "seo.title-too-short");
}

#[test]
fn seo_missing_meta_description() {
    let html = wrap_head_extra(
        r#"<title>Good Page Title For Testing Purposes</title><link rel="canonical" href="https://example.com/"><meta property="og:title" content="t"><meta property="og:description" content="d"><meta property="og:image" content="https://example.com/i.jpg"><meta name="twitter:card" content="summary">"#,
    );
    assert_rule(&html, "seo.missing-meta-description");
}

#[test]
fn seo_missing_h1() {
    let html = wrap_head_extra(
        r#"<title>Good Page Title For Testing Purposes</title><meta name="description" content="Desc long enough for tests and validation purposes here."><link rel="canonical" href="https://example.com/"><meta property="og:title" content="t"><meta property="og:description" content="d"><meta property="og:image" content="https://example.com/i.jpg"><meta name="twitter:card" content="summary">"#,
    )
    .replace("<h1>Title</h1>", "<h2>Sub</h2>");
    assert_rule(&html, "seo.missing-h1");
}

#[test]
fn seo_heading_hierarchy_skip() {
    let html = wrap_head_extra(
        r#"<title>Good Page Title For Testing Purposes</title><meta name="description" content="Desc long enough for tests and validation purposes here."><link rel="canonical" href="https://example.com/"><meta property="og:title" content="t"><meta property="og:description" content="d"><meta property="og:image" content="https://example.com/i.jpg"><meta name="twitter:card" content="summary">"#,
    )
    .replace("<h1>Title</h1>", "<h1>Title</h1><h3>Skip</h3>");
    assert_rule(&html, "seo.heading-hierarchy-skip");
}

#[test]
fn a11y_missing_html_lang() {
    let html = wrap_head_extra(
        r#"<title>Good Page Title For Testing Purposes</title><meta name="description" content="Desc long enough for tests and validation purposes here."><link rel="canonical" href="https://example.com/"><meta property="og:title" content="t"><meta property="og:description" content="d"><meta property="og:image" content="https://example.com/i.jpg"><meta name="twitter:card" content="summary">"#,
    )
    .replace(" lang=\"en\"", "");
    assert_rule(&html, "a11y.missing-html-lang");
}

#[test]
fn a11y_image_missing_alt() {
    let html = wrap_head_extra(
        r#"<title>Good Page Title For Testing Purposes</title><meta name="description" content="Desc long enough for tests and validation purposes here."><link rel="canonical" href="https://example.com/"><meta property="og:title" content="t"><meta property="og:description" content="d"><meta property="og:image" content="https://example.com/i.jpg"><meta name="twitter:card" content="summary">"#,
    )
    .replace(r#"alt="x""#, "");
    assert_rule(&html, "a11y.image-missing-alt");
}

#[test]
fn structured_missing_json_ld() {
    let html = wrap_head_extra(
        r#"<title>Good Page Title For Testing Purposes</title><meta name="description" content="Desc long enough for tests and validation purposes here."><link rel="canonical" href="https://example.com/"><meta property="og:title" content="t"><meta property="og:description" content="d"><meta property="og:image" content="https://example.com/i.jpg"><meta name="twitter:card" content="summary">"#,
    );
    assert_rule(&html, "structured.missing-json-ld");
}

#[test]
fn structured_invalid_json_ld() {
    let html = wrap_head_extra(
        r#"<title>Good Page Title For Testing Purposes</title><meta name="description" content="Desc long enough for tests and validation purposes here."><link rel="canonical" href="https://example.com/"><meta property="og:title" content="t"><meta property="og:description" content="d"><meta property="og:image" content="https://example.com/i.jpg"><meta name="twitter:card" content="summary"><script type="application/ld+json">{not json}</script>"#,
    );
    assert_rule(&html, "structured.invalid-json-ld");
}

#[test]
fn llm_little_extractable_text() {
    let html = wrap_head_extra(
        r#"<title>Good Page Title For Testing Purposes</title><meta name="description" content="Desc long enough for tests and validation purposes here."><link rel="canonical" href="https://example.com/"><meta property="og:title" content="t"><meta property="og:description" content="d"><meta property="og:image" content="https://example.com/i.jpg"><meta name="twitter:card" content="summary">"#,
    )
    .replace(
        "Enough body text here for extraction and LLM checks with multiple sentences included.",
        "Short.",
    );
    assert_rule(&html, "llm.little-extractable-text");
}

#[test]
fn seo_broken_internal_link() {
    let html = wrap_head_extra(
        r#"<title>Good Page Title For Testing Purposes</title><meta name="description" content="Desc long enough for tests and validation purposes here."><link rel="canonical" href="https://example.com/"><meta property="og:title" content="t"><meta property="og:description" content="d"><meta property="og:image" content="https://example.com/i.jpg"><meta name="twitter:card" content="summary">"#,
    )
    .replace(r#"<a href="x">link</a>"#, r#"<a href="missing.html">broken</a>"#);
    assert_rule(&html, "seo.broken-internal-link");
}

#[test]
fn seo_image_missing_dimensions() {
    let html = wrap_head_extra(
        r#"<title>Good Page Title For Testing Purposes</title><meta name="description" content="Desc long enough for tests and validation purposes here."><link rel="canonical" href="https://example.com/"><meta property="og:title" content="t"><meta property="og:description" content="d"><meta property="og:image" content="https://example.com/i.jpg"><meta name="twitter:card" content="summary">"#,
    )
    .replace(r#"width="1" height="1""#, "");
    assert_rule(&html, "seo.image-missing-dimensions");
}

#[tokio::test]
async fn internal_links_relative_and_absolute_resolution() {
    let target = fixture("testdata/link_resolution_site");
    let pages = load_pages(target.to_str().unwrap(), &LoadOptions::default())
        .await
        .unwrap();
    let report = audit_pages(target.to_str().unwrap(), &pages);
    assert!(
        !has_rule(&report, "seo.broken-internal-link"),
        "valid relative and root-relative links should not be reported broken"
    );
}

#[tokio::test]
async fn url_target_kind_is_url() {
    use oratos_audit::resolve_target;
    use oratos_core::TargetKind;
    let t = resolve_target("https://example.com/page");
    assert_eq!(t.kind, TargetKind::Url);
}

#[test]
fn llm_missing_llms_txt_site_level() {
    let dir = tempfile::tempdir().unwrap();
    let html = r#"<!DOCTYPE html><html lang="en"><head><title>Site</title></head><body><main><h1>Hi</h1><p>Text long enough for checks here with multiple words included.</p></main></body></html>"#;
    let path = dir.path().join("index.html");
    let target = dir.path().to_str().unwrap().to_string();
    let page = parse_html(&path.to_string_lossy(), html, true);
    let _guard = dir;
    let report = audit_pages(&target, std::slice::from_ref(&page));
    assert_eq!(
        report
            .findings
            .iter()
            .filter(|f| f.rule_id == "llm.missing-llms-txt")
            .count(),
        1
    );
}

#[test]
fn well_formed_page_avoids_common_seo_errors() {
    let html = good_page();
    assert_no_rule(&html, "seo.missing-title");
    assert_no_rule(&html, "seo.missing-h1");
}

#[test]
fn seo_title_too_long() {
    let long_title = "A".repeat(61);
    let html = good_page().replace(
        "<title>Good Page Title For Testing Purposes</title>",
        &format!("<title>{long_title}</title>"),
    );
    assert_rule(&html, "seo.title-too-long");
}

#[test]
fn seo_meta_description_too_short() {
    let html = good_page().replace(
        r#"content="Desc long enough for tests and validation purposes here with plenty of characters.""#,
        r#"content="Too short meta description here.""#,
    );
    assert_rule(&html, "seo.meta-description-too-short");
}

#[test]
fn seo_meta_description_too_long() {
    let long = "x".repeat(170);
    let html = good_page().replace(
        r#"content="Desc long enough for tests and validation purposes here with plenty of characters.""#,
        &format!(r#"content="{long}""#),
    );
    assert_rule(&html, "seo.meta-description-too-long");
}

#[test]
fn seo_missing_canonical() {
    let html = good_page().replace(r#"<link rel="canonical" href="https://example.com/">"#, "");
    assert_rule(&html, "seo.missing-canonical");
}

#[test]
fn seo_multiple_canonical() {
    let html = good_page().replace(
        r#"<link rel="canonical" href="https://example.com/">"#,
        r#"<link rel="canonical" href="https://example.com/a"><link rel="canonical" href="https://example.com/b">"#,
    );
    assert_rule(&html, "seo.multiple-canonical");
}

#[test]
fn seo_multiple_h1() {
    let html = good_page().replace("<h1>Title</h1>", "<h1>One</h1><h1>Two</h1>");
    assert_rule(&html, "seo.multiple-h1");
}

#[test]
fn seo_missing_og_title() {
    let html = good_page().replace(r#"<meta property="og:title" content="t">"#, "");
    assert_rule(&html, "seo.missing-og-title");
}

#[test]
fn seo_missing_og_description() {
    let html = good_page().replace(r#"<meta property="og:description" content="d">"#, "");
    assert_rule(&html, "seo.missing-og-description");
}

#[test]
fn seo_missing_og_image() {
    let html = good_page().replace(
        r#"<meta property="og:image" content="https://example.com/i.jpg">"#,
        "",
    );
    assert_rule(&html, "seo.missing-og-image");
}

#[test]
fn seo_missing_twitter_card() {
    let html = good_page().replace(r#"<meta name="twitter:card" content="summary">"#, "");
    assert_rule(&html, "seo.missing-twitter-card");
}

#[test]
fn a11y_image_empty_alt_on_meaningful_image() {
    let html = good_page().replace(
        r#"<img src="x.jpg" alt="x" width="1" height="1">"#,
        r#"<img src="hero.jpg" alt="" width="1" height="1">"#,
    );
    assert_rule(&html, "a11y.image-empty-alt");
}

#[test]
fn a11y_link_empty_text() {
    let html = good_page().replace(
        r#"<a href="x">link</a>"#,
        r#"<a href="https://example.com/x"></a>"#,
    );
    assert_rule(&html, "a11y.link-empty-text");
}

#[test]
fn a11y_missing_main_landmark() {
    let html = good_page()
        .replace("<main>", "<div>")
        .replace("</main>", "</div>");
    assert_rule(&html, "a11y.missing-main-landmark");
}

#[test]
fn a11y_input_without_label() {
    let html = good_page().replace(
        "</main>",
        r#"<form><input type="text" id="email" name="email"></form></main>"#,
    );
    assert_rule(&html, "a11y.input-without-label");
}

#[test]
fn a11y_heading_hierarchy() {
    let html = good_page().replace("<h1>Title</h1>", "<h1>Title</h1><h3>Skip</h3>");
    assert_rule(&html, "a11y.heading-hierarchy");
}

#[test]
fn structured_missing_webpage_type() {
    let html = good_page().replace(
        r#"<script type="application/ld+json">{"@context":"https://schema.org","@type":"WebPage","name":"Test"}</script>"#,
        r#"<script type="application/ld+json">{"@context":"https://schema.org","@type":"Organization","name":"Acme"}</script>"#,
    );
    assert_rule(&html, "structured.missing-webpage");
}

#[test]
fn structured_missing_breadcrumb_list() {
    let html = good_page().replace(
        "<main>",
        r#"<nav class="breadcrumb">Home / About</nav><main>"#,
    );
    assert_rule(&html, "structured.missing-breadcrumb-list");
}

#[test]
fn structured_missing_image_object() {
    let html = good_page().replace(
        r#"<img src="x.jpg" alt="x" width="1" height="1">"#,
        r#"<img src="hero.jpg" alt="Product" width="1" height="1">"#,
    );
    assert_rule(&html, "structured.missing-image-object");
}

#[test]
fn llm_weak_title_description() {
    let html = good_page().replace(
        "<title>Good Page Title For Testing Purposes</title>",
        "<title>Short</title>",
    );
    assert_rule(&html, "llm.weak-title-description");
}

#[test]
fn llm_missing_primary_heading() {
    let html = good_page().replace("<h1>Title</h1>", "<h2>Subtitle</h2>");
    assert_rule(&html, "llm.missing-primary-heading");
}

#[test]
fn llm_image_lacks_description() {
    let html = good_page().replace(
        r#"<img src="x.jpg" alt="x" width="1" height="1">"#,
        r#"<img src="hero.jpg" width="1" height="1">"#,
    );
    assert_rule(&html, "llm.image-lacks-description");
}

#[test]
fn llm_no_summary_candidate() {
    let html = r#"<!DOCTYPE html><html lang="en"><head><title>Good Page Title For Testing Purposes</title></head><body><main><h1>Only</h1><p>Brief.</p></main></body></html>"#;
    assert_rule(html, "llm.no-summary-candidate");
}
