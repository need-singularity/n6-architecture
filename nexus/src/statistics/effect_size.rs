/// Effect size calculation — Cohen's d and classification.

use super::significance;

/// Compute Cohen's d effect size between two groups.
///
/// d = (mean1 - mean2) / pooled_std
///
/// Uses the pooled standard deviation (unbiased).
pub fn cohens_d(group1: &[f64], group2: &[f64]) -> f64 {
    if group1.len() < 2 || group2.len() < 2 {
        return 0.0;
    }

    let mean1 = significance::mean(group1);
    let mean2 = significance::mean(group2);
    let var1 = significance::variance(group1, mean1);
    let var2 = significance::variance(group2, mean2);

    let n1 = group1.len() as f64;
    let n2 = group2.len() as f64;

    // Pooled standard deviation
    let pooled_var = ((n1 - 1.0) * var1 + (n2 - 1.0) * var2) / (n1 + n2 - 2.0);
    let pooled_std = pooled_var.sqrt();

    if pooled_std < 1e-15 {
        return 0.0;
    }

    (mean1 - mean2) / pooled_std
}

/// Classify effect size according to Cohen's conventions.
///
/// - |d| < 0.2: "negligible"
/// - 0.2 <= |d| < 0.5: "small"
/// - 0.5 <= |d| < 0.8: "medium"
/// - |d| >= 0.8: "large"
pub fn classify_effect(d: f64) -> &'static str {
    let abs_d = d.abs();
    if abs_d < 0.2 {
        "negligible"
    } else if abs_d < 0.5 {
        "small"
    } else if abs_d < 0.8 {
        "medium"
    } else {
        "large"
    }
}

/// Compute the common language effect size (CLES).
///
/// Probability that a random value from group1 exceeds a random value from group2.
/// Uses the formula: CLES = Phi(d / sqrt(2)), where Phi is the standard normal CDF.
pub fn common_language_effect(d: f64) -> f64 {
    // Approximate using the normal CDF
    let z = d / std::f64::consts::SQRT_2;
    // Simple normal CDF approximation
    0.5 * (1.0 + erf_approx(z / std::f64::consts::SQRT_2))
}

/// Simple error function approximation (Abramowitz & Stegun 7.1.28).
fn erf_approx(x: f64) -> f64 {
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cohens_d_zero() {
        let data = vec![6.0, 12.0, 24.0];
        let d = cohens_d(&data, &data);
        assert!((d).abs() < 1e-10, "d={}", d);
    }

    #[test]
    fn test_cohens_d_large() {
        let a = vec![100.0, 101.0, 99.0, 100.5, 99.5];
        let b = vec![0.0, 1.0, -1.0, 0.5, -0.5];
        let d = cohens_d(&a, &b);
        assert!(d.abs() > 0.8, "d={}", d);
        assert_eq!(classify_effect(d), "large");
    }

    #[test]
    fn test_classify_effect() {
        assert_eq!(classify_effect(0.0), "negligible");
        assert_eq!(classify_effect(0.1), "negligible");
        assert_eq!(classify_effect(0.3), "small");
        assert_eq!(classify_effect(0.6), "medium");
        assert_eq!(classify_effect(1.0), "large");
        assert_eq!(classify_effect(-1.5), "large");
    }

    #[test]
    fn test_common_language_effect() {
        // d=0 should give ~50% (equal groups)
        let cles = common_language_effect(0.0);
        assert!((cles - 0.5).abs() < 0.01, "cles={}", cles);

        // Large positive d should give > 50%
        let cles_large = common_language_effect(2.0);
        assert!(cles_large > 0.8, "cles={}", cles_large);
    }

    #[test]
    fn test_insufficient_data() {
        let d = cohens_d(&[1.0], &[2.0]);
        assert!((d).abs() < 1e-10);
    }
}
