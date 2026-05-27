use std::path::Path;

use oratos_core::{AuditTarget, TargetKind};

pub fn resolve_target(target: &str) -> AuditTarget {
    let kind = if target.starts_with("http://") || target.starts_with("https://") {
        TargetKind::Url
    } else {
        let path = Path::new(target);
        if path.is_file() {
            TargetKind::File
        } else if path.is_dir() {
            TargetKind::Directory
        } else {
            TargetKind::Missing
        }
    };

    AuditTarget {
        path_or_url: target.to_string(),
        kind,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_target_marks_missing_paths() {
        let target = resolve_target("./this-path-should-not-exist-oratos");
        assert_eq!(target.kind, TargetKind::Missing);
    }
}
