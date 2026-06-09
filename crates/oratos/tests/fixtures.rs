use std::path::PathBuf;

use oratos::html::{load_pages, LoadOptions};

fn fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}

#[tokio::test]
async fn loads_minimal_site() {
    let target = fixture("testdata/minimal_site");
    let pages = load_pages(target.to_str().unwrap(), &LoadOptions::default())
        .await
        .unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].title.as_deref(), Some("Minimal"));
}

#[tokio::test]
async fn loads_directory_with_multiple_pages() {
    let target = fixture("testdata/good_site");
    let pages = load_pages(target.to_str().unwrap(), &LoadOptions::default())
        .await
        .unwrap();
    assert!(pages.len() >= 2);
}

#[tokio::test]
async fn tolerates_invalid_html() {
    let html = "<html><head><title>X</title><body><p>Unclosed";
    let page = oratos::html::parse_html("/x.html", html, true);
    assert_eq!(page.title.as_deref(), Some("X"));
}
