use std::path::PathBuf;

use oratos_audit::audit_pages;
use oratos_core::{Category, Severity};
use oratos_html::load_pages;
use oratos_html::LoadOptions;

fn fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}

#[tokio::test]
async fn broken_site_has_errors() {
    let target = fixture("testdata/broken_site");
    let pages = load_pages(target.to_str().unwrap(), &LoadOptions::default())
        .await
        .unwrap();
    let report = audit_pages(target.to_str().unwrap(), &pages);
    assert!(report
        .findings
        .iter()
        .any(|f| f.rule_id.contains("missing-h1") || f.rule_id.contains("alt")));
}

#[tokio::test]
async fn good_site_scores_higher_than_broken() {
    let broken_path = fixture("testdata/broken_site");
    let good_path = fixture("testdata/good_site");
    let broken = load_pages(broken_path.to_str().unwrap(), &LoadOptions::default())
        .await
        .unwrap();
    let good = load_pages(good_path.to_str().unwrap(), &LoadOptions::default())
        .await
        .unwrap();
    let broken_report = audit_pages(broken_path.to_str().unwrap(), &broken);
    let good_report = audit_pages(good_path.to_str().unwrap(), &good);
    assert!(good_report.scores.overall > broken_report.scores.overall);
}

#[tokio::test]
async fn seo_rule_emits_expected_findings() {
    let target = fixture("testdata/broken_site");
    let pages = load_pages(target.to_str().unwrap(), &LoadOptions::default())
        .await
        .unwrap();
    let report = audit_pages(target.to_str().unwrap(), &pages);

    assert!(report
        .findings
        .iter()
        .any(|f| f.rule_id == "seo.missing-h1"));
    assert!(report
        .findings
        .iter()
        .any(|f| f.rule_id == "seo.broken-internal-link"));
    assert!(report
        .findings
        .iter()
        .any(|f| f.rule_id == "seo.image-missing-dimensions"));
}

#[tokio::test]
async fn image_missing_dimensions_affects_seo_score() {
    let target = fixture("testdata/broken_site");
    let pages = load_pages(target.to_str().unwrap(), &LoadOptions::default())
        .await
        .unwrap();
    let report = audit_pages(target.to_str().unwrap(), &pages);

    let finding = report
        .findings
        .iter()
        .find(|f| f.rule_id == "seo.image-missing-dimensions")
        .expect("seo.image-missing-dimensions finding");
    assert_eq!(finding.category, Category::Seo);

    let page = &report.pages[0];
    assert!(page.scores.seo < 100.0);
}

#[tokio::test]
async fn accessibility_rule_emits_expected_findings() {
    let target = fixture("testdata/broken_site");
    let pages = load_pages(target.to_str().unwrap(), &LoadOptions::default())
        .await
        .unwrap();
    let report = audit_pages(target.to_str().unwrap(), &pages);

    assert!(report
        .findings
        .iter()
        .any(|f| f.rule_id == "a11y.missing-html-lang"));
    assert!(report
        .findings
        .iter()
        .any(|f| f.rule_id == "a11y.image-missing-alt"));
}

#[tokio::test]
async fn structured_data_rule_emits_expected_findings() {
    let target = fixture("testdata/broken_site");
    let pages = load_pages(target.to_str().unwrap(), &LoadOptions::default())
        .await
        .unwrap();
    let report = audit_pages(target.to_str().unwrap(), &pages);

    assert!(report
        .findings
        .iter()
        .any(|f| f.rule_id == "structured.missing-json-ld"));
    assert!(report
        .findings
        .iter()
        .any(|f| f.rule_id == "structured.missing-webpage"));
}

#[tokio::test]
async fn llm_rule_emits_expected_findings() {
    let target = fixture("testdata/broken_site");
    let pages = load_pages(target.to_str().unwrap(), &LoadOptions::default())
        .await
        .unwrap();
    let report = audit_pages(target.to_str().unwrap(), &pages);

    assert!(report
        .findings
        .iter()
        .any(|f| f.rule_id == "llm.weak-title-description"));
    assert!(report
        .findings
        .iter()
        .any(|f| f.rule_id == "llm.little-extractable-text"));
}

#[tokio::test]
async fn broken_link_not_confused_by_suffix_collision() {
    let target = fixture("testdata/suffix_collision_site");
    let pages = load_pages(target.to_str().unwrap(), &LoadOptions::default())
        .await
        .unwrap();
    let report = audit_pages(target.to_str().unwrap(), &pages);

    assert!(report
        .findings
        .iter()
        .any(|f| { f.rule_id == "seo.broken-internal-link" && f.message.contains("about.html") }));
}

#[tokio::test]
async fn site_level_llms_rule_is_not_duplicated_per_page() {
    let target = fixture("testdata/good_site");
    let pages = load_pages(target.to_str().unwrap(), &LoadOptions::default())
        .await
        .unwrap();
    let report = audit_pages(target.to_str().unwrap(), &pages);

    let llms_missing: Vec<_> = report
        .findings
        .iter()
        .filter(|f| f.rule_id == "llm.missing-llms-txt")
        .collect();
    assert_eq!(llms_missing.len(), 1);
    assert_eq!(llms_missing[0].severity, Severity::Info);
}
