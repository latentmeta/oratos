use std::collections::BTreeMap;

use crate::core::{AuditReport, Severity};

pub fn format_sarif(report: &AuditReport) -> String {
    let results: Vec<serde_json::Value> = report
        .findings
        .iter()
        .map(|f| {
            let level = match f.severity {
                Severity::Error => "error",
                Severity::Warning => "warning",
                Severity::Info => "note",
            };
            let mut result = serde_json::json!({
                "ruleId": f.rule_id,
                "level": level,
                "message": {
                    "text": f.message
                }
            });
            if let Some(loc) = &f.location {
                result["locations"] = serde_json::json!([{
                    "physicalLocation": {
                        "artifactLocation": {
                            "uri": loc.page
                        }
                    }
                }]);
            }
            result
        })
        .collect();

    let mut rule_index = BTreeMap::<String, (String, Option<String>)>::new();
    for f in &report.findings {
        rule_index
            .entry(f.rule_id.clone())
            .or_insert((f.message.clone(), f.docs_url.clone()));
    }

    let rules: Vec<serde_json::Value> = rule_index
        .into_iter()
        .map(|(rule_id, (message, docs_url))| {
            serde_json::json!({
                "id": rule_id,
                "shortDescription": {
                    "text": message
                },
                "helpUri": docs_url
            })
        })
        .collect();

    let sarif = serde_json::json!({
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "oratos",
                    "version": report.core_version,
                    "informationUri": "https://github.com/latentmeta/oratos",
                    "rules": rules
                }
            },
            "results": results
        }]
    });

    serde_json::to_string_pretty(&sarif).expect("SARIF serializes")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        AuditReport, AuditTarget, Category, Finding, PageAudit, PageRef, Severity, TargetKind,
    };

    #[test]
    fn sarif_deduplicates_rules_by_id() {
        let finding = Finding::new(
            "seo.missing-title",
            Severity::Error,
            Category::Seo,
            "Missing title",
        );
        let page = PageAudit {
            page: PageRef::new("/index.html"),
            findings: vec![finding.clone(), finding],
            scores: crate::core::CategoryScores::from_findings(&[]),
        };
        let report = AuditReport::new(
            AuditTarget {
                path_or_url: ".".to_string(),
                kind: TargetKind::Directory,
            },
            vec![page],
        );

        let sarif: serde_json::Value = serde_json::from_str(&format_sarif(&report)).unwrap();
        let rules = sarif["runs"][0]["tool"]["driver"]["rules"]
            .as_array()
            .unwrap();
        assert_eq!(rules.len(), 1);
    }

    #[test]
    fn sarif_omits_locations_when_absent() {
        let finding = Finding::new(
            "a11y.missing-html-lang",
            Severity::Error,
            Category::Accessibility,
            "Missing lang",
        );
        let page = PageAudit {
            page: PageRef::new("/index.html"),
            findings: vec![finding],
            scores: crate::core::CategoryScores::from_findings(&[]),
        };
        let report = AuditReport::new(
            AuditTarget {
                path_or_url: ".".to_string(),
                kind: TargetKind::Directory,
            },
            vec![page],
        );

        let sarif: serde_json::Value = serde_json::from_str(&format_sarif(&report)).unwrap();
        let result = &sarif["runs"][0]["results"][0];
        assert!(result.get("locations").is_none());
    }

    #[test]
    fn sarif_has_required_schema_fields() {
        let finding = Finding::new(
            "seo.missing-title",
            Severity::Error,
            Category::Seo,
            "Missing title",
        );
        let page = PageAudit {
            page: PageRef::new("/index.html"),
            findings: vec![finding.clone(), finding],
            scores: crate::core::CategoryScores::from_findings(&[]),
        };
        let report = AuditReport::new(
            AuditTarget {
                path_or_url: ".".to_string(),
                kind: TargetKind::Directory,
            },
            vec![page],
        );

        let sarif: serde_json::Value = serde_json::from_str(&format_sarif(&report)).unwrap();
        assert_eq!(sarif["version"], "2.1.0");
        assert!(sarif["runs"][0]["tool"]["driver"]["rules"].is_array());
        assert!(sarif["runs"][0]["results"].is_array());
        let rules = sarif["runs"][0]["tool"]["driver"]["rules"]
            .as_array()
            .unwrap();
        assert_eq!(rules.len(), 1);
    }
}
