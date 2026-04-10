use serde::{Deserialize, Serialize};

/// Verification grade based on composite score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Grade {
    S, // >= 0.9
    A, // >= 0.7
    B, // >= 0.5
    C, // >= 0.3
    D, // <  0.3
}

impl Grade {
    pub fn from_score(score: f64) -> Self {
        if score >= 0.9 {
            Grade::S
        } else if score >= 0.7 {
            Grade::A
        } else if score >= 0.5 {
            Grade::B
        } else if score >= 0.3 {
            Grade::C
        } else {
            Grade::D
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Grade::S => "S",
            Grade::A => "A",
            Grade::B => "B",
            Grade::C => "C",
            Grade::D => "D",
        }
    }
}

/// Result of a verification pass.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub score: f64,
    pub grade: Grade,
    /// Per-component breakdown
    pub breakdown: Breakdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breakdown {
    pub lens_consensus: f64,
    pub cross_validation: f64,
    pub physical_check: f64,
    pub graph_bonus: f64,
    pub novelty: f64,
    pub n6_exact: f64,
}

/// Compute a verification score from six input dimensions (each 0.0..=1.0).
///
/// ```text
/// score = lens_consensus   * 0.25
///       + cross_validation * 0.20
///       + physical_check   * 0.25
///       + graph_bonus      * 0.15
///       + novelty          * 0.05
///       + n6_exact         * 0.10
/// ```
pub fn verify(
    lens_consensus: f64,
    cross_validation: f64,
    physical_check: f64,
    graph_bonus: f64,
    novelty: f64,
    n6_exact: f64,
) -> VerificationResult {
    let score = lens_consensus * 0.25
        + cross_validation * 0.20
        + physical_check * 0.25
        + graph_bonus * 0.15
        + novelty * 0.05
        + n6_exact * 0.10;

    let grade = Grade::from_score(score);

    VerificationResult {
        score,
        grade,
        breakdown: Breakdown {
            lens_consensus,
            cross_validation,
            physical_check,
            graph_bonus,
            novelty,
            n6_exact,
        },
    }
}
