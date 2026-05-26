use std::path::Path;

use oratos_core::{AuditTarget, TargetKind};

pub fn resolve_target(target: &str) -> AuditTarget {
    let kind = if target.starts_with("http://") || target.starts_with("https://") {
        TargetKind::Url
    } else if Path::new(target).is_file() {
        TargetKind::File
    } else {
        TargetKind::Directory
    };

    AuditTarget {
        path_or_url: target.to_string(),
        kind,
    }
}
