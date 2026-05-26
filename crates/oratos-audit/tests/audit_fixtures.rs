use std::path::PathBuf;

use oratos_audit::audit_pages;
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
