use oratos_core::{AuditReport, Severity};

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
            serde_json::json!({
                "ruleId": f.rule_id,
                "level": level,
                "message": {
                    "text": f.message
                },
                "locations": f.location.as_ref().map(|loc| {
                    serde_json::json!([{
                        "physicalLocation": {
                            "artifactLocation": {
                                "uri": loc.page
                            }
                        }
                    }])
                }).unwrap_or(serde_json::Value::Array(vec![]))
            })
        })
        .collect();

    let rules: Vec<serde_json::Value> = report
        .findings
        .iter()
        .map(|f| {
            serde_json::json!({
                "id": f.rule_id,
                "shortDescription": {
                    "text": f.message
                },
                "helpUri": f.docs_url
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
                    "version": report.version,
                    "informationUri": "https://github.com/latentmeta/oratos",
                    "rules": rules
                }
            },
            "results": results
        }]
    });

    serde_json::to_string_pretty(&sarif).expect("SARIF serializes")
}
