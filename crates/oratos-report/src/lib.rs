//! Report formatters for Oratos audit results.

mod console;
mod html;
mod json;
mod markdown;
mod sarif;

pub use console::format_console;
pub use html::format_html;
pub use json::format_json;
pub use markdown::format_markdown;
pub use sarif::format_sarif;

use oratos_core::AuditReport;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    Console,
    Json,
    Markdown,
    Html,
    Sarif,
}

impl ReportFormat {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "console" => Some(Self::Console),
            "json" => Some(Self::Json),
            "markdown" | "md" => Some(Self::Markdown),
            "html" => Some(Self::Html),
            "sarif" => Some(Self::Sarif),
            _ => None,
        }
    }

    pub fn render(&self, report: &AuditReport) -> String {
        match self {
            Self::Console => format_console(report),
            Self::Json => format_json(report),
            Self::Markdown => format_markdown(report),
            Self::Html => format_html(report),
            Self::Sarif => format_sarif(report),
        }
    }
}
