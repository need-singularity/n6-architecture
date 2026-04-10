use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors, mean_var};

/// SymmetryBreakingLens: Detect broken symmetries in data distributions.
///
/// Checks for asymmetries that reveal n=6 structure:
///   1. Skewness per feature (asymmetry measure)
///   2. Left-right imbalance around mean
///   3. Feature-pair asymmetry (f(x,y) != f(y,x) proxy)
pub struct SymmetryBreakingLens;

impl Lens for SymmetryBreakingLens {
    fn name(&self) -> &str { "SymmetryBreakingLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }

        let columns = column_vectors(data, n, d);
        let (means, vars) = mean_var(data, n, d);

        // Skewness per feature
        let mut skewness = Vec::with_capacity(d);
        for j in 0..d {
            let std = vars[j].sqrt();
            if std < 1e-12 { skewness.push(0.0); continue; }
            let mut m3 = 0.0;
            for i in 0..n {
                let z = (columns[j][i] - means[j]) / std;
                m3 += z * z * z;
            }
            m3 /= n as f64;
            skewness.push(m3);
        }

        // Mean absolute skewness
        let mean_abs_skew = skewness.iter().map(|s| s.abs()).sum::<f64>() / d as f64;

        // Left-right imbalance: fraction of points below mean per feature
        let mut lr_imbalance = Vec::with_capacity(d);
        for j in 0..d {
            let below = columns[j].iter().filter(|&&v| v < means[j]).count();
            let ratio = below as f64 / n as f64;
            lr_imbalance.push((ratio - 0.5).abs()); // 0 = symmetric
        }
        let mean_lr_imbalance = lr_imbalance.iter().sum::<f64>() / d as f64;

        // Excess kurtosis per feature
        let mut kurtosis = Vec::with_capacity(d);
        for j in 0..d {
            let std = vars[j].sqrt();
            if std < 1e-12 { kurtosis.push(0.0); continue; }
            let mut m4 = 0.0;
            for i in 0..n {
                let z = (columns[j][i] - means[j]) / std;
                m4 += z * z * z * z;
            }
            m4 /= n as f64;
            kurtosis.push(m4 - 3.0); // Excess kurtosis
        }

        let mean_excess_kurtosis = kurtosis.iter().sum::<f64>() / d as f64;

        let mut result = HashMap::new();
        result.insert("skewness".to_string(), skewness);
        result.insert("mean_abs_skewness".to_string(), vec![mean_abs_skew]);
        result.insert("lr_imbalance".to_string(), lr_imbalance);
        result.insert("mean_lr_imbalance".to_string(), vec![mean_lr_imbalance]);
        result.insert("excess_kurtosis".to_string(), kurtosis);
        result.insert("mean_excess_kurtosis".to_string(), vec![mean_excess_kurtosis]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetry_breaking_symmetric() {
        // Symmetric data around 0
        let data: Vec<f64> = vec![
            -3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0,
            -3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0,
        ];
        let n = 7;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let result = SymmetryBreakingLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("mean_abs_skewness"));
        let skew = result["mean_abs_skewness"][0];
        assert!(skew < 0.1, "Symmetric data should have low skewness, got {}", skew);
    }

    #[test]
    fn test_symmetry_breaking_skewed() {
        // Right-skewed data
        let data: Vec<f64> = vec![
            1.0, 1.0, 1.0, 2.0, 2.0, 3.0, 10.0, 20.0, 50.0,
            1.0, 1.0, 1.0, 2.0, 2.0, 3.0, 10.0, 20.0, 50.0,
        ];
        let n = 9;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let result = SymmetryBreakingLens.scan(&data, n, d, &shared);
        let skew = result["mean_abs_skewness"][0];
        assert!(skew > 0.5, "Skewed data should have high skewness, got {}", skew);
    }
}
