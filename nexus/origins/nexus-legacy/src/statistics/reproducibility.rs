/// Reproducibility assessment for experimental results.

use super::significance;

/// Reproducibility report.
#[derive(Debug, Clone)]
pub struct ReproducibilityReport {
    pub n_results: usize,
    pub mean: f64,
    pub std: f64,
    pub cv: f64,
    pub is_reproducible: bool,
    pub consistency_score: f64,
    pub outliers: Vec<usize>,
}

/// Assess reproducibility of a set of repeated measurements.
///
/// A result is considered reproducible if the coefficient of variation (CV)
/// is below the given threshold (default: 0.10 = 10%).
pub fn assess_reproducibility(results: &[f64], cv_threshold: f64) -> ReproducibilityReport {
    if results.is_empty() {
        return ReproducibilityReport {
            n_results: 0,
            mean: 0.0,
            std: 0.0,
            cv: f64::INFINITY,
            is_reproducible: false,
            consistency_score: 0.0,
            outliers: vec![],
        };
    }

    let m = significance::mean(results);
    let s = significance::std_dev(results);
    let cv = if m.abs() > 1e-15 { s / m.abs() } else { f64::INFINITY };

    // Detect outliers using IQR method
    let outliers = detect_outliers(results);

    // Consistency score: 1.0 - (CV / threshold), clamped to [0, 1]
    let consistency_score = (1.0 - cv / cv_threshold).max(0.0).min(1.0);

    ReproducibilityReport {
        n_results: results.len(),
        mean: m,
        std: s,
        cv,
        is_reproducible: cv < cv_threshold && outliers.is_empty(),
        consistency_score,
        outliers,
    }
}

/// Detect outliers using 1.5 * IQR method.
///
/// Returns indices of outlier values.
fn detect_outliers(data: &[f64]) -> Vec<usize> {
    if data.len() < 4 {
        return vec![];
    }

    let mut sorted: Vec<f64> = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let n = sorted.len();
    let q1 = sorted[n / 4];
    let q3 = sorted[3 * n / 4];
    let iqr = q3 - q1;

    let lower = q1 - 1.5 * iqr;
    let upper = q3 + 1.5 * iqr;

    data.iter()
        .enumerate()
        .filter(|(_, &v)| v < lower || v > upper)
        .map(|(i, _)| i)
        .collect()
}

/// Check if a set of n=6 match ratios is consistently high.
///
/// For n=6 verification, we want repeated experiments to consistently
/// find high EXACT ratios.
pub fn n6_consistency(exact_ratios: &[f64]) -> f64 {
    if exact_ratios.is_empty() {
        return 0.0;
    }

    let m = significance::mean(exact_ratios);
    let s = significance::std_dev(exact_ratios);
    let cv = if m.abs() > 1e-15 { s / m.abs() } else { 1.0 };

    // Score: high mean + low variance = good
    // Penalize high CV
    (m * (1.0 - cv.min(1.0))).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reproducible_data() {
        let data = vec![6.0, 6.01, 5.99, 6.0, 6.02, 5.98];
        let report = assess_reproducibility(&data, 0.10);
        assert!(report.is_reproducible);
        assert!(report.cv < 0.01);
        assert!(report.consistency_score > 0.9);
    }

    #[test]
    fn test_unreproducible_data() {
        let data = vec![6.0, 100.0, 0.1, 50.0, 6.0];
        let report = assess_reproducibility(&data, 0.10);
        assert!(!report.is_reproducible);
        assert!(report.cv > 0.5);
    }

    #[test]
    fn test_empty_data() {
        let report = assess_reproducibility(&[], 0.10);
        assert!(!report.is_reproducible);
        assert_eq!(report.n_results, 0);
    }

    #[test]
    fn test_outlier_detection() {
        let data = vec![6.0, 6.1, 5.9, 6.0, 100.0, 6.05, 5.95, 6.0];
        let outliers = detect_outliers(&data);
        // 100.0 should be detected as outlier
        assert!(outliers.contains(&4), "outliers: {:?}", outliers);
    }

    #[test]
    fn test_n6_consistency_high() {
        let ratios = vec![0.95, 0.93, 0.96, 0.94, 0.95];
        let score = n6_consistency(&ratios);
        assert!(score > 0.8, "score={}", score);
    }

    #[test]
    fn test_n6_consistency_low() {
        let ratios = vec![0.1, 0.9, 0.3, 0.8, 0.2];
        let score = n6_consistency(&ratios);
        assert!(score < 0.5, "score={}", score);
    }
}
