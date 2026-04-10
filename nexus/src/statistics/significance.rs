/// Statistical significance testing — pure Rust, no external crates.

/// Result of a significance test.
#[derive(Debug, Clone)]
pub struct SignificanceTest {
    pub p_value: f64,
    pub corrected_p: f64,
    pub significant: bool,
    pub method: String,
}

/// Perform a two-sample t-test approximation between observed and expected values.
///
/// Uses Welch's t-test approximation. The p-value is estimated using a
/// simple normal approximation for large enough samples.
pub fn test_significance(observed: &[f64], expected: &[f64]) -> SignificanceTest {
    if observed.is_empty() || expected.is_empty() {
        return SignificanceTest {
            p_value: 1.0,
            corrected_p: 1.0,
            significant: false,
            method: "t-test (insufficient data)".to_string(),
        };
    }

    let n1 = observed.len() as f64;
    let n2 = expected.len() as f64;

    let mean1 = mean(observed);
    let mean2 = mean(expected);
    let var1 = variance(observed, mean1);
    let var2 = variance(expected, mean2);

    let se = (var1 / n1 + var2 / n2).sqrt();

    let t_stat = if se > 1e-15 {
        (mean1 - mean2) / se
    } else {
        0.0
    };

    // Approximate p-value using standard normal (valid for large n)
    let p_value = 2.0 * standard_normal_cdf(-t_stat.abs());

    SignificanceTest {
        p_value,
        corrected_p: p_value, // no correction for single test
        significant: p_value < 0.05,
        method: "Welch's t-test (normal approx)".to_string(),
    }
}

/// Apply Bonferroni correction to a set of p-values.
///
/// Each p-value is multiplied by the number of tests.
/// Results are capped at 1.0.
pub fn bonferroni_correction(p_values: &[f64]) -> Vec<f64> {
    let m = p_values.len() as f64;
    p_values.iter().map(|&p| (p * m).min(1.0)).collect()
}

/// Apply Benjamini-Hochberg False Discovery Rate control.
///
/// Returns a boolean mask indicating which hypotheses are rejected at the given alpha level.
pub fn false_discovery_rate(p_values: &[f64], alpha: f64) -> Vec<bool> {
    let m = p_values.len();
    if m == 0 {
        return vec![];
    }

    // Sort p-values with their original indices
    let mut indexed: Vec<(usize, f64)> = p_values.iter().copied().enumerate().collect();
    indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    let mut rejected = vec![false; m];

    // Find the largest k such that P_(k) <= k/m * alpha
    let mut max_k = 0;
    for (rank, &(_, p)) in indexed.iter().enumerate() {
        let threshold = (rank + 1) as f64 / m as f64 * alpha;
        if p <= threshold {
            max_k = rank + 1;
        }
    }

    // Reject all hypotheses with rank <= max_k
    for (rank, &(orig_idx, _)) in indexed.iter().enumerate() {
        if rank < max_k {
            rejected[orig_idx] = true;
        }
    }

    rejected
}

/// Compute mean of a slice.
pub fn mean(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    data.iter().sum::<f64>() / data.len() as f64
}

/// Compute sample variance.
pub fn variance(data: &[f64], mean_val: f64) -> f64 {
    if data.len() < 2 {
        return 0.0;
    }
    let ss: f64 = data.iter().map(|&x| (x - mean_val).powi(2)).sum();
    ss / (data.len() as f64 - 1.0)
}

/// Standard deviation.
pub fn std_dev(data: &[f64]) -> f64 {
    let m = mean(data);
    variance(data, m).sqrt()
}

/// Approximate standard normal CDF using Abramowitz & Stegun formula 7.1.26.
fn standard_normal_cdf(x: f64) -> f64 {
    if x < -8.0 {
        return 0.0;
    }
    if x > 8.0 {
        return 1.0;
    }

    let t = 1.0 / (1.0 + 0.2316419 * x.abs());
    let d = 0.3989422804014327; // 1/sqrt(2*pi)
    let p = d * (-x * x / 2.0).exp();
    let c = t * (0.319381530 + t * (-0.356563782 + t * (1.781477937 + t * (-1.821255978 + t * 1.330274429))));

    if x >= 0.0 {
        1.0 - p * c
    } else {
        p * c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_significance_same_data() {
        let data = vec![6.0, 12.0, 24.0, 4.0, 2.0];
        let result = test_significance(&data, &data);
        // Same data should have p-value close to 1.0 (not significant)
        assert!(result.p_value > 0.5, "p={}", result.p_value);
        assert!(!result.significant);
    }

    #[test]
    fn test_significance_different_data() {
        let a = vec![100.0, 101.0, 99.0, 100.5, 99.5];
        let b = vec![0.0, 1.0, -1.0, 0.5, -0.5];
        let result = test_significance(&a, &b);
        assert!(result.p_value < 0.01, "p={}", result.p_value);
        assert!(result.significant);
    }

    #[test]
    fn test_bonferroni() {
        let p_values = vec![0.01, 0.04, 0.03, 0.20];
        let corrected = bonferroni_correction(&p_values);
        assert_eq!(corrected.len(), 4);
        assert!((corrected[0] - 0.04).abs() < 1e-10); // 0.01 * 4
        assert!((corrected[1] - 0.16).abs() < 1e-10); // 0.04 * 4
        assert!((corrected[3] - 0.80).abs() < 1e-10); // 0.20 * 4
    }

    #[test]
    fn test_fdr_basic() {
        let p_values = vec![0.001, 0.04, 0.03, 0.20, 0.50];
        let rejected = false_discovery_rate(&p_values, 0.05);
        // The smallest p-value (0.001) should definitely be rejected
        assert!(rejected[0]);
        // The largest (0.50) should not
        assert!(!rejected[4]);
    }

    #[test]
    fn test_mean_and_variance() {
        let data = vec![2.0, 4.0, 6.0];
        assert!((mean(&data) - 4.0).abs() < 1e-10);
        assert!((variance(&data, 4.0) - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_empty_data() {
        let result = test_significance(&[], &[1.0]);
        assert!(!result.significant);
        assert!((result.p_value - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_normal_cdf_symmetry() {
        let cdf_0 = standard_normal_cdf(0.0);
        assert!((cdf_0 - 0.5).abs() < 0.001);

        let cdf_pos = standard_normal_cdf(1.96);
        assert!((cdf_pos - 0.975).abs() < 0.005);
    }
}
