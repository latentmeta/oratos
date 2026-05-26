//! Deterministic audit rules for website visibility.

mod audit;
mod rules;
mod target;

pub use audit::audit_pages;
pub use rules::Rule;
pub use target::resolve_target;
