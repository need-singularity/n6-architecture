use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, mean_var};

/// OutlierLens: Statistical outlier detection via z-score and IQR methods.
///
/// Identifies anomalous data points using multiple detection methods,
/// reports fraction of outliers and their properties.
pub struct OutlierLens;

impl Lens for OutlierLens {
    fn name(&self) -> &str { "OutlierLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }

        let (means, vars) = mean_var(data, n, d);

        // Z-score method: |z| > 3
        let mut z_outlier_count = 0;
        let mut z_scores_max = Vec::with_capacity(n);
        for i in 0..n {
            let mut max_z = 0.0_f64;
            for j in 0..d {
                let std = vars[j].sqrt();
                if std > 1e-12 {
                    let z = ((data[i * d + j] - means[j]) / std).abs();
                    max_z = max_z.max(z);
                }
            }
            z_scores_max.push(max_z);
            if max_z > 3.0 { z_outlier_count += 1; }
        }

        // IQR method per dimension
        let mut iqr_outlier_flags = vec![false; n];
        for j in 0..d {
            let mut col: Vec<f64> = (0..n).map(|i| data[i * d + j]).collect();
            col.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let q1 = col[n / 4];
            let q3 = col[3 * n / 4];
            let iqr = q3 - q1;
            let lower = q1 - 1.5 * iqr;
            let upper = q3 + 1.5 * iqr;
            for i in 0..n {
                let v = data[i * d + j];
                if v < lower || v > upper {
                    iqr_outlier_flags[i] = true;
                }
            }
        }
        let iqr_outlier_count = iqr_outlier_flags.iter().filter(|&&f| f).count();

        let z_fraction = z_outlier_count as f64 / n as f64;
        let iqr_fraction = iqr_outlier_count as f64 / n as f64;

        let mut result = HashMap::new();
        result.insert("z_outlier_count".to_string(), vec![z_outlier_count as f64]);
        result.insert("z_outlier_fraction".to_string(), vec![z_fraction]);
        result.insert("iqr_outlier_count".to_string(), vec![iqr_outlier_count as f64]);
        result.insert("iqr_outlier_fraction".to_string(), vec![iqr_fraction]);
        result.insert("z_scores_max".to_string(), z_scores_max);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outlier_lens_with_outlier() {
        // 9 points: 8 clustered + 1 far outlier
        let data = vec![
            1.0, 1.0,
            1.1, 1.0,
            1.0, 1.1,
            1.1, 1.1,
            0.9, 0.9,
            0.9, 1.1,
            1.1, 0.9,
            1.0, 1.05,
            100.0, 100.0, // outlier
        ];
        let n = 9;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let result = OutlierLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("z_outlier_count"));
        // IQR method should catch the extreme outlier
        assert!(result["iqr_outlier_count"][0] >= 1.0, "Should detect at least 1 IQR outlier");
    }

    #[test]
    fn test_outlier_lens_no_outlier() {
        let data: Vec<f64> = (0..20).map(|i| (i as f64 * 0.5).sin()).collect();
        let n = 10;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let result = OutlierLens.scan(&data, n, d, &shared);
        assert!(result["z_outlier_fraction"][0] < 0.3, "Smooth data should have few outliers");
    }
}
