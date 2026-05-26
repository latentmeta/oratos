use serde::{Deserialize, Serialize};

use crate::finding::{Category, Finding, Severity};

/// Category scores are 0–100. Higher is better.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CategoryScores {
    pub seo: f64,
    pub accessibility: f64,
    pub structured_data: f64,
    pub llm_readiness: f64,
    pub overall: f64,
}

impl CategoryScores {
    pub fn from_findings(findings: &[Finding]) -> Self {
        let seo = score_category(findings, Category::Seo);
        let accessibility = score_category(findings, Category::Accessibility);
        let structured_data = score_category(findings, Category::StructuredData);
        let llm_readiness = score_category(findings, Category::LlmReadiness);

        // Weighted overall: SEO 30%, A11y 25%, Structured 25%, LLM 20%
        let overall =
            seo * 0.30 + accessibility * 0.25 + structured_data * 0.25 + llm_readiness * 0.20;

        Self {
            seo,
            accessibility,
            structured_data,
            llm_readiness,
            overall,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreBreakdown {
    pub scores: CategoryScores,
    pub penalty_weights: PenaltyWeights,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PenaltyWeights {
    pub error: u32,
    pub warning: u32,
    pub info: u32,
}

impl Default for PenaltyWeights {
    fn default() -> Self {
        Self {
            error: 10,
            warning: 5,
            info: 1,
        }
    }
}

fn score_category(findings: &[Finding], category: Category) -> f64 {
    let weights = PenaltyWeights::default();
    let penalty: u32 = findings
        .iter()
        .filter(|f| f.category == category)
        .map(|f| match f.severity {
            Severity::Error => weights.error,
            Severity::Warning => weights.warning,
            Severity::Info => weights.info,
        })
        .sum();

    (100.0_f64 - f64::from(penalty)).clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::finding::Finding;

    #[test]
    fn perfect_score_with_no_findings() {
        let scores = CategoryScores::from_findings(&[]);
        assert!((scores.overall - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn errors_reduce_score() {
        let findings = vec![Finding::new(
            "test",
            Severity::Error,
            Category::Seo,
            "test error",
        )];
        let scores = CategoryScores::from_findings(&findings);
        assert!((scores.seo - 90.0).abs() < f64::EPSILON);
    }
}
