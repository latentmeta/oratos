//! End-to-end audit of pages loaded over HTTP (mock server).

use oratos::audit::audit_pages;
use oratos::core::TargetKind;
use oratos::html::{load_pages, LoadOptions};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const BAD_SEO_HTML: &str = r#"<!DOCTYPE html>
<html>
<head></head>
<body><p>Short.</p></body>
</html>"#;

#[tokio::test]
async fn audit_pages_loaded_from_http_url() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/audit-me.html"))
        .respond_with(ResponseTemplate::new(200).set_body_string(BAD_SEO_HTML))
        .mount(&server)
        .await;

    let url = format!("{}/audit-me.html", server.uri());
    let pages = load_pages(&url, &LoadOptions::default())
        .await
        .expect("load_pages");
    let report = audit_pages(&url, &pages);

    assert_eq!(report.target.kind, TargetKind::Url);
    assert_eq!(report.pages.len(), 1);
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.rule_id == "seo.missing-title"),
        "expected SEO findings from fetched HTML"
    );
}
