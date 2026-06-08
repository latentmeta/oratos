use std::collections::HashSet;
use std::path::Path;

use oratos_core::{Category, Finding, Location, Severity};
use oratos_html::HtmlPage;

pub trait Rule {
    fn id(&self) -> &'static str;
    fn check(&self, page: &HtmlPage, ctx: &AuditContext) -> Vec<Finding>;
}

pub struct AuditContext {
    pub site_root: Option<String>,
    pub known_paths: HashSet<String>,
    pub has_llms_txt: bool,
}

/// Canonical path keys for exact membership checks (no suffix heuristics).
pub fn build_known_paths(pages: &[HtmlPage]) -> HashSet<String> {
    pages
        .iter()
        .map(|p| normalize_path_key(Path::new(&p.url_or_path)))
        .collect()
}

fn normalize_path_key(path: &Path) -> String {
    path.canonicalize()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| path.to_string_lossy().replace('\\', "/"))
}

pub fn all_rules() -> Vec<Box<dyn Rule + Send + Sync>> {
    vec![
        Box::new(SeoRules),
        Box::new(A11yRules),
        Box::new(StructuredDataRules),
        Box::new(LlmReadinessRules),
    ]
}

struct SeoRules;
struct A11yRules;
struct StructuredDataRules;
struct LlmReadinessRules;

impl Rule for SeoRules {
    fn id(&self) -> &'static str {
        "seo"
    }

    fn check(&self, page: &HtmlPage, ctx: &AuditContext) -> Vec<Finding> {
        let mut findings = Vec::new();
        let loc = || Location::page(&page.url_or_path);

        // Title
        match &page.title {
            None => findings.push(
                Finding::new(
                    "seo.missing-title",
                    Severity::Error,
                    Category::Seo,
                    "Page is missing a <title> element.",
                )
                .with_recommendation("Add a unique, descriptive <title> between 30–60 characters.")
                .with_docs_url("https://developers.google.com/search/docs/appearance/title-link"),
            ),
            Some(t) if t.len() < 15 => findings.push(
                Finding::new(
                    "seo.title-too-short",
                    Severity::Warning,
                    Category::Seo,
                    format!("Title is too short ({len} chars).", len = t.len()),
                )
                .with_recommendation(
                    "Expand the title to at least 30 characters while staying under 60.",
                ),
            ),
            Some(t) if t.len() > 60 => findings.push(Finding::new(
                "seo.title-too-long",
                Severity::Warning,
                Category::Seo,
                format!(
                    "Title may be truncated in search results ({len} chars).",
                    len = t.len()
                ),
            )),
            _ => {}
        }

        // Meta description
        match &page.meta_description {
            None => findings.push(
                Finding::new(
                    "seo.missing-meta-description",
                    Severity::Warning,
                    Category::Seo,
                    "Page is missing a meta description.",
                )
                .with_recommendation(
                    "Add <meta name=\"description\" content=\"...\"> with 70–160 characters.",
                ),
            ),
            Some(d) if d.len() < 50 => findings.push(Finding::new(
                "seo.meta-description-too-short",
                Severity::Warning,
                Category::Seo,
                format!(
                    "Meta description is too short ({len} chars).",
                    len = d.len()
                ),
            )),
            Some(d) if d.len() > 160 => findings.push(Finding::new(
                "seo.meta-description-too-long",
                Severity::Warning,
                Category::Seo,
                format!("Meta description is too long ({len} chars).", len = d.len()),
            )),
            _ => {}
        }

        // Canonical
        if page.canonical_urls.is_empty() {
            findings.push(
                Finding::new(
                    "seo.missing-canonical",
                    Severity::Warning,
                    Category::Seo,
                    "Page is missing a canonical URL.",
                )
                .with_recommendation(
                    "Add <link rel=\"canonical\" href=\"...\"> pointing to the preferred URL.",
                ),
            );
        } else if page.canonical_urls.len() > 1 {
            findings.push(
                Finding::new(
                    "seo.multiple-canonical",
                    Severity::Error,
                    Category::Seo,
                    "Page has multiple canonical URLs.",
                )
                .with_recommendation("Keep exactly one canonical link per page."),
            );
        }

        // Headings
        let h1s: Vec<_> = page.headings.iter().filter(|h| h.level == 1).collect();
        if h1s.is_empty() {
            findings.push(
                Finding::new(
                    "seo.missing-h1",
                    Severity::Error,
                    Category::Seo,
                    "Page is missing an <h1> heading.",
                )
                .with_recommendation("Add a single primary <h1> that describes the page topic."),
            );
        } else if h1s.len() > 1 {
            findings.push(
                Finding::new(
                    "seo.multiple-h1",
                    Severity::Warning,
                    Category::Seo,
                    format!("Page has {} <h1> elements.", h1s.len()),
                )
                .with_recommendation("Use a single <h1> per page; use <h2>–<h6> for subsections."),
            );
        }

        if heading_hierarchy_skip(&page.headings) {
            findings.push(
                Finding::new(
                    "seo.heading-hierarchy-skip",
                    Severity::Warning,
                    Category::Seo,
                    "Heading levels skip one or more levels (e.g. h1 → h3).",
                )
                .with_recommendation("Use sequential heading levels without skipping."),
            );
        }

        // Open Graph
        if page.open_graph.title.is_none() {
            findings.push(Finding::new(
                "seo.missing-og-title",
                Severity::Warning,
                Category::Seo,
                "Missing Open Graph title (og:title).",
            ));
        }
        if page.open_graph.description.is_none() {
            findings.push(Finding::new(
                "seo.missing-og-description",
                Severity::Warning,
                Category::Seo,
                "Missing Open Graph description (og:description).",
            ));
        }
        if page.open_graph.image.is_none() {
            findings.push(Finding::new(
                "seo.missing-og-image",
                Severity::Warning,
                Category::Seo,
                "Missing Open Graph image (og:image).",
            ));
        }

        // Twitter card
        if page.twitter_card.card.is_none() && page.twitter_card.title.is_none() {
            findings.push(Finding::new(
                "seo.missing-twitter-card",
                Severity::Info,
                Category::Seo,
                "Missing Twitter card metadata.",
            ));
        }

        // Broken internal links
        for link in &page.links {
            if link.is_internal && !link.href.is_empty() {
                let resolved = resolve_local_target(&page.url_or_path, &link.href, ctx);
                if resolved.is_some() && !resolved.unwrap() {
                    findings.push(
                        Finding::new(
                            "seo.broken-internal-link",
                            Severity::Error,
                            Category::Seo,
                            format!("Broken internal link: {}", link.href),
                        )
                        .with_location(loc().with_selector(format!("a[href='{}']", link.href))),
                    );
                }
            }
        }

        // Image dimensions
        for img in &page.images {
            if !img.src.is_empty() && img.width.is_none() && img.height.is_none() {
                findings.push(
                    Finding::new(
                        "seo.image-missing-dimensions",
                        Severity::Info,
                        Category::Seo,
                        format!("Image missing width/height attributes: {}", img.src),
                    )
                    .with_recommendation("Add width and height to reduce layout shift."),
                );
            }
        }
        findings
    }
}

impl Rule for A11yRules {
    fn id(&self) -> &'static str {
        "accessibility"
    }

    fn check(&self, page: &HtmlPage, _ctx: &AuditContext) -> Vec<Finding> {
        let mut findings = Vec::new();

        if page.language.is_none() {
            findings.push(
                Finding::new(
                    "a11y.missing-html-lang",
                    Severity::Error,
                    Category::Accessibility,
                    "Missing lang attribute on <html>.",
                )
                .with_recommendation(
                    "Add lang=\"en\" (or the page language) to the <html> element.",
                ),
            );
        }

        for img in &page.images {
            if img.alt.is_none() {
                findings.push(
                    Finding::new(
                        "a11y.image-missing-alt",
                        Severity::Error,
                        Category::Accessibility,
                        format!("Image missing alt text: {}", img.src),
                    )
                    .with_recommendation(
                        "Add descriptive alt text, or alt=\"\" for decorative images.",
                    ),
                );
            } else if img.alt.as_deref() == Some("") && likely_meaningful_image(img) {
                findings.push(
                    Finding::new(
                        "a11y.image-empty-alt",
                        Severity::Warning,
                        Category::Accessibility,
                        format!("Image has empty alt but may be meaningful: {}", img.src),
                    )
                    .with_recommendation(
                        "Provide descriptive alt text if the image conveys information.",
                    ),
                );
            }
        }

        for link in &page.links {
            if link.text.trim().is_empty() {
                findings.push(
                    Finding::new(
                        "a11y.link-empty-text",
                        Severity::Warning,
                        Category::Accessibility,
                        format!("Link has no visible text: {}", link.href),
                    )
                    .with_recommendation("Add visible link text or aria-label."),
                );
            }
        }

        if !page.has_main_landmark {
            findings.push(
                Finding::new(
                    "a11y.missing-main-landmark",
                    Severity::Warning,
                    Category::Accessibility,
                    "Page is missing a <main> landmark or role=\"main\".",
                )
                .with_recommendation("Wrap primary content in <main>."),
            );
        }

        for form in &page.forms {
            for input in &form.inputs_without_label {
                findings.push(
                    Finding::new(
                        "a11y.input-without-label",
                        Severity::Warning,
                        Category::Accessibility,
                        format!("Form input without associated label: {input}"),
                    )
                    .with_recommendation("Associate each input with a <label for=\"...\">."),
                );
            }
        }

        if heading_hierarchy_skip(&page.headings) {
            findings.push(Finding::new(
                "a11y.heading-hierarchy",
                Severity::Warning,
                Category::Accessibility,
                "Heading hierarchy skips levels.",
            ));
        }

        findings
    }
}

impl Rule for StructuredDataRules {
    fn id(&self) -> &'static str {
        "structured-data"
    }

    fn check(&self, page: &HtmlPage, _ctx: &AuditContext) -> Vec<Finding> {
        let mut findings = Vec::new();

        if page.json_ld_blocks.is_empty() {
            findings.push(
                Finding::new(
                    "structured.missing-json-ld",
                    Severity::Warning,
                    Category::StructuredData,
                    "Page has no JSON-LD structured data.",
                )
                .with_recommendation(
                    "Add a WebPage JSON-LD block in <script type=\"application/ld+json\">.",
                ),
            );
        }

        for (i, block) in page.json_ld_blocks.iter().enumerate() {
            if !block.valid_json {
                findings.push(
                    Finding::new(
                        "structured.invalid-json-ld",
                        Severity::Error,
                        Category::StructuredData,
                        format!("JSON-LD block #{i} contains invalid JSON."),
                    )
                    .with_recommendation("Fix JSON syntax in the structured data script."),
                );
            }
        }

        let all_types: Vec<_> = page
            .json_ld_blocks
            .iter()
            .flat_map(|b| b.types.iter().cloned())
            .collect();

        if page.json_ld_blocks.iter().all(|b| b.valid_json)
            && !all_types.iter().any(|t| t == "WebPage")
        {
            findings.push(Finding::new(
                "structured.missing-webpage",
                Severity::Info,
                Category::StructuredData,
                "No WebPage JSON-LD type detected; consider adding one.",
            ));
        }

        if page.breadcrumbs_detected && !all_types.iter().any(|t| t == "BreadcrumbList") {
            findings.push(Finding::new(
                "structured.missing-breadcrumb-list",
                Severity::Info,
                Category::StructuredData,
                "Breadcrumbs detected but no BreadcrumbList JSON-LD found.",
            ));
        }

        if page.json_ld_blocks.iter().all(|b| b.valid_json)
            && page.url_or_path.contains("/blog")
            && !all_types
                .iter()
                .any(|t| t == "Article" || t == "BlogPosting")
        {
            findings.push(Finding::new(
                "structured.missing-article",
                Severity::Info,
                Category::StructuredData,
                "Blog-like URL without Article or BlogPosting JSON-LD.",
            ));
        }

        let is_site_root = page.url_or_path.ends_with("/index.html")
            || page.url_or_path.ends_with("/index.htm")
            || page.url_or_path.ends_with('/');
        if page.json_ld_blocks.iter().all(|b| b.valid_json)
            && is_site_root
            && !all_types
                .iter()
                .any(|t| t == "Organization" || t == "WebSite")
        {
            findings.push(Finding::new(
                "structured.missing-organization",
                Severity::Info,
                Category::StructuredData,
                "Site root page without Organization or WebSite JSON-LD.",
            ));
        }

        let important_images: Vec<_> = page
            .images
            .iter()
            .filter(|img| !img.src.is_empty() && likely_meaningful_image(img))
            .collect();
        if !important_images.is_empty() && !all_types.iter().any(|t| t == "ImageObject") {
            findings.push(
                Finding::new(
                    "structured.missing-image-object",
                    Severity::Info,
                    Category::StructuredData,
                    "Important images present but no ImageObject JSON-LD recommendation applied.",
                )
                .with_recommendation("Add ImageObject structured data for key images."),
            );
        }

        findings
    }
}

impl Rule for LlmReadinessRules {
    fn id(&self) -> &'static str {
        "llm-readiness"
    }

    fn check(&self, page: &HtmlPage, _ctx: &AuditContext) -> Vec<Finding> {
        let mut findings = Vec::new();

        let weak_title = page.title.as_ref().map(|t| t.len() < 15).unwrap_or(true);
        let weak_desc = page
            .meta_description
            .as_ref()
            .map(|d| d.len() < 50)
            .unwrap_or(true);
        if weak_title || weak_desc {
            findings.push(Finding::new(
                "llm.weak-title-description",
                Severity::Warning,
                Category::LlmReadiness,
                "Page has a weak or missing title/description for LLM summarization.",
            ));
        }

        if page.headings.iter().all(|h| h.level != 1) {
            findings.push(Finding::new(
                "llm.missing-primary-heading",
                Severity::Warning,
                Category::LlmReadiness,
                "Page lacks a clear primary heading (<h1>).",
            ));
        }

        for img in &page.images {
            if likely_meaningful_image(img)
                && img.alt.as_ref().map(|a| a.is_empty()).unwrap_or(true)
                && !img.has_caption
            {
                findings.push(Finding::new(
                    "llm.image-lacks-description",
                    Severity::Warning,
                    Category::LlmReadiness,
                    format!("Important image lacks alt text or caption: {}", img.src),
                ));
            }
        }

        if page.main_text.len() < 100 {
            findings.push(Finding::new(
                "llm.little-extractable-text",
                Severity::Warning,
                Category::LlmReadiness,
                format!(
                    "Page has little extractable text ({} chars).",
                    page.main_text.len()
                ),
            ));
        }

        let has_summary_candidate = page.meta_description.is_some()
            || page.headings.len() >= 2
            || page.main_text.len() > 200;
        if !has_summary_candidate {
            findings.push(Finding::new(
                "llm.no-summary-candidate",
                Severity::Info,
                Category::LlmReadiness,
                "Page has no clear structured summary candidate for LLMs.",
            ));
        }

        findings
    }
}

fn heading_hierarchy_skip(headings: &[oratos_html::Heading]) -> bool {
    let mut prev: Option<u8> = None;
    for h in headings {
        if let Some(p) = prev {
            if h.level > p + 1 {
                return true;
            }
        }
        prev = Some(h.level);
    }
    false
}

fn likely_meaningful_image(img: &oratos_html::ImageInfo) -> bool {
    let src_lower = img.src.to_lowercase();
    !src_lower.contains("spacer")
        && !src_lower.contains("pixel")
        && !src_lower.contains("tracking")
        && !src_lower.ends_with(".svg")
}

fn resolve_local_target(page_path: &str, href: &str, ctx: &AuditContext) -> Option<bool> {
    if href.starts_with("http://") || href.starts_with("https://") {
        return None;
    }
    let Some(root) = &ctx.site_root else {
        return None;
    };
    let page_dir = std::path::Path::new(page_path)
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from(root));

    if let Some(resolved) = oratos_html::resolve_internal_path(&page_dir, href) {
        return Some(resolved.exists());
    }

    let target = if href.starts_with('/') {
        std::path::PathBuf::from(root).join(href.trim_start_matches('/'))
    } else {
        page_dir.join(href)
    };

    if target.exists() {
        return Some(true);
    }

    let normalized = normalize_path_key(&target);
    Some(ctx.known_paths.contains(&normalized))
}
