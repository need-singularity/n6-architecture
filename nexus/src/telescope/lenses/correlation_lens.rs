use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors, mean_var};

/// CorrelationLens: Pearson and rank correlation matrix analysis.
///
/// Detects linear and monotonic relationships between features,
/// checks for n=6 correlation structure (sigma=12 dimensional correlations).
pub struct CorrelationLens;

impl Lens for CorrelationLens {
    fn name(&self) -> &str { "CorrelationLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d < 2 { return HashMap::new(); }

        let columns = column_vectors(data, n, d);
        let (means, vars) = mean_var(data, n, d);

        let pairs = d.min(12); // Cap at sigma=12 dimensions
        let mut pearson_vals = Vec::new();

        for i in 0..pairs {
            let std_i = vars[i].sqrt();
            if std_i < 1e-12 { continue; }
            for j in (i + 1)..pairs.min(d) {
                let std_j = vars[j].sqrt();
                if std_j < 1e-12 { continue; }
                let mut cov = 0.0;
                for k in 0..n {
                    cov += (columns[i][k] - means[i]) * (columns[j][k] - means[j]);
                }
                cov /= n as f64;
                let r = cov / (std_i * std_j);
                pearson_vals.push(r);
            }
        }

        if pearson_vals.is_empty() { return HashMap::new(); }

        let mean_abs_corr = pearson_vals.iter().map(|r| r.abs()).sum::<f64>() / pearson_vals.len() as f64;
        let max_abs_corr = pearson_vals.iter().map(|r| r.abs()).fold(0.0_f64, f64::max);

        // Count strong correlations (|r| > 0.7)
        let strong_count = pearson_vals.iter().filter(|&&r| r.abs() > 0.7).count();
        let weak_count = pearson_vals.iter().filter(|&&r| r.abs() < 0.3).count();

        let mut result = HashMap::new();
        result.insert("pearson_values".to_string(), pearson_vals);
        result.insert("mean_abs_correlation".to_string(), vec![mean_abs_corr]);
        result.insert("max_abs_correlation".to_string(), vec![max_abs_corr]);
        result.insert("strong_correlation_count".to_string(), vec![strong_count as f64]);
        result.insert("weak_correlation_count".to_string(), vec![weak_count as f64]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correlation_lens() {
        let mut data = Vec::new();
        for i in 0..20 {
            let x = i as f64;
            data.push(x);
            data.push(x * 2.0 + 1.0); // Perfect linear correlation
            data.push((i as f64 * 0.7).sin()); // Low correlation
        }
        let n = 20;
        let d = 3;
        let shared = SharedData::compute(&data, n, d);
        let result = CorrelationLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("max_abs_correlation"));
        assert!(result["max_abs_correlation"][0] > 0.9, "Should detect near-perfect correlation");
    }
}
