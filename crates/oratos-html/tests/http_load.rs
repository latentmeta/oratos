use oratos_html::{load_pages, LoadOptions};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const REMOTE_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head><title>Remote Page</title></head>
<body><main><h1>Hello</h1><p>Content served by the mock HTTP server for integration testing.</p></main>
</body></html>"#;

#[tokio::test]
async fn load_pages_fetches_html_over_http() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/page.html"))
        .respond_with(ResponseTemplate::new(200).set_body_string(REMOTE_HTML))
        .mount(&server)
        .await;

    let url = format!("{}/page.html", server.uri());
    let pages = load_pages(&url, &LoadOptions::default())
        .await
        .expect("load_pages");

    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].title.as_deref(), Some("Remote Page"));
    assert_eq!(pages[0].url_or_path, url);
    assert!(pages[0].has_main_landmark);
}

#[tokio::test]
async fn load_pages_strips_url_fragment_before_fetch() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(REMOTE_HTML))
        .mount(&server)
        .await;

    let url = format!("{}/#section", server.uri());
    let pages = load_pages(&url, &LoadOptions::default())
        .await
        .expect("load_pages");

    assert_eq!(pages.len(), 1);
    assert!(!pages[0].url_or_path.contains('#'));
}
