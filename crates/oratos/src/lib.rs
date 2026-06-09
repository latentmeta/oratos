#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../README.md"))]

pub mod audit;
pub mod core;
pub mod generate;
pub mod html;
pub mod report;

pub use audit::{audit_pages, resolve_target, Rule};
pub use core::{
    apply_ignore_rules, AuditConfig, AuditReport, AuditTarget, Category, CategoryScores,
    ConfigError, CrawlConfig, Finding, Location, OratosConfig, PageAudit, PageRef,
    ScoreBreakdown, Severity, TargetKind,
};
pub use generate::{
    generate_html_remediation_prompt, generate_llms_txt, generate_metadata_recommendations,
    generate_phoenix_remediation_prompt, LlmProvider, LlmProviderConfig, MetadataRecommendation,
    OllamaProvider, OpenAiCompatibleProvider,
};
pub use html::{
    disallowed_paths, is_disallowed, load_pages, normalize_url, parse_html, resolve_internal_path,
    urls_from_sitemap_xml, CrawlOptions, Heading, HtmlPage, ImageInfo, JsonLdBlock, LinkInfo,
    LoadOptions,
};
pub use report::{
    format_console, format_html, format_json, format_markdown, format_sarif, ReportFormat,
};
