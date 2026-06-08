//! Core types shared across Oratos crates.

mod audit;
mod config;
mod finding;
mod location;
mod page;
mod score;

pub use audit::{AuditReport, AuditTarget, PageAudit, TargetKind};
pub use config::{apply_ignore_rules, AuditConfig, ConfigError, CrawlConfig, OratosConfig};
pub use finding::{Category, Finding, Severity};
pub use location::Location;
pub use page::PageRef;
pub use score::{CategoryScores, ScoreBreakdown};
