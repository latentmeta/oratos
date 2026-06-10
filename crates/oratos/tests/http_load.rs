use oratos::html::{load_pages, CrawlOptions, LoadOptions};
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

const CRAWL_INDEX: &str = r#"<!DOCTYPE html>
<html lang="en"><head><title>Home</title></head>
<body><main><h1>Home</h1><a href="/about.html">About</a></main></body></html>"#;

const CRAWL_ABOUT: &str = r#"<!DOCTYPE html>
<html lang="en"><head><title>About</title></head>
<body><main><h1>About page</h1></main></body></html>"#;

#[tokio::test]
async fn load_pages_crawls_same_origin_links() {
    let server = MockServer::start().await;
    for p in ["/", "/index.html", "/about.html"] {
        let body = if p.contains("about") {
            CRAWL_ABOUT
        } else {
            CRAWL_INDEX
        };
        Mock::given(method("GET"))
            .and(path(p))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .mount(&server)
            .await;
    }

    let options = LoadOptions {
        crawl: Some(CrawlOptions {
            max_pages: 5,
            max_depth: 2,
            respect_robots: false,
            use_sitemap: false,
        }),
        ..Default::default()
    };
    let pages = load_pages(&server.uri(), &options)
        .await
        .expect("crawl load");

    assert!(pages.len() >= 2);
    assert!(pages.iter().any(|p| p.title.as_deref() == Some("About")));
}
