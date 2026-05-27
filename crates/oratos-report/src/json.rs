use oratos_core::AuditReport;

/// Stable JSON report schema for CI and tooling integration.
pub fn format_json(report: &AuditReport) -> String {
    serde_json::to_string_pretty(report).expect("audit report serializes to JSON")
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use oratos_core::{AuditReport, AuditTarget, TargetKind};

    #[test]
    fn json_has_expected_top_level_keys() {
        let report = AuditReport::new(
            AuditTarget {
                path_or_url: "./test".into(),
                kind: TargetKind::Directory,
            },
            vec![],
        );
        let json: serde_json::Value = serde_json::from_str(&format_json(&report)).unwrap();
        assert!(json.get("core_version").is_some());
        assert!(json.get("target").is_some());
        assert!(json.get("pages").is_some());
        assert!(json.get("findings").is_some());
        assert!(json.get("scores").is_some());
        assert!(json.get("page_count").is_some());
    }

    #[test]
    fn json_output_is_stable_snapshot() {
        let report = AuditReport::new(
            AuditTarget {
                path_or_url: "./test".into(),
                kind: TargetKind::Directory,
            },
            vec![],
        );
        assert_snapshot!("empty_report_json", format_json(&report));
    }
}
