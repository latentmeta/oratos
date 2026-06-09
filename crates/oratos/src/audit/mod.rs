//! Deterministic audit rules for website visibility.

mod rules;
mod run;
mod target;

pub use run::audit_pages;
pub use rules::Rule;
pub use target::resolve_target;
