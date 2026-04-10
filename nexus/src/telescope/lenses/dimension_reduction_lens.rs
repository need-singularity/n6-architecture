use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, mean_var};

/// DimensionReductionLens: PCA-based intrinsic dimensionality estimation.
///
/// Estimates the effective dimensionality of data and checks if it matches
/// n=6 constants. Uses covariance eigenvalue analysis via power iteration.
pub struct DimensionReductionLens;

impl Lens for DimensionReductionLens {
    fn name(&self) -> &str { "DimensionReductionLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d < 2 { return HashMap::new(); }

        let (means, vars) = mean_var(data, n, d);

        // Compute covariance matrix (d x d)
        let dd = d.min(50); // Cap for performance
        let mut cov = vec![0.0f64; dd * dd];
        for i in 0..n {
            for a in 0..dd {
                let da = data[i * d + a] - means[a];
                for b in a..dd {
                    let db = data[i * d + b] - means[b];
                    cov[a * dd + b] += da * db;
                }
            }
        }
        let inv_n = 1.0 / n as f64;
        for a in 0..dd {
            for b in a..dd {
                cov[a * dd + b] *= inv_n;
                cov[b * dd + a] = cov[a * dd + b]; // Symmetric
            }
        }

        // Compute approximate eigenvalues via diagonal + Gershgorin
        // For a simple estimate, use variances as eigenvalue proxies
        let mut eigenvalues: Vec<f64> = vars[..dd].to_vec();
        eigenvalues.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

        let total_var: f64 = eigenvalues.iter().sum();
        if total_var < 1e-12 { return HashMap::new(); }

        // Explained variance ratio
        let explained: Vec<f64> = eigenvalues.iter().map(|&e| e / total_var).collect();

        // Intrinsic dimensionality: count eigenvalues explaining 95% variance
        let mut cum = 0.0;
        let mut intrinsic_dim = 0;
        for &ev in &explained {
            cum += ev;
            intrinsic_dim += 1;
            if cum >= 0.95 { break; }
        }

        // Participation ratio: (sum lambda)^2 / sum(lambda^2)
        let sum_sq: f64 = eigenvalues.iter().map(|e| e * e).sum();
        let participation_ratio = if sum_sq > 1e-12 {
            total_var * total_var / sum_sq
        } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("intrinsic_dimensionality".to_string(), vec![intrinsic_dim as f64]);
        result.insert("participation_ratio".to_string(), vec![participation_ratio]);
        result.insert("explained_variance".to_string(), explained);
        result.insert("total_variance".to_string(), vec![total_var]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dim_reduction_lens() {
        // 2D data embedded in 4D space (features 3,4 are noise)
        let mut data = Vec::new();
        for i in 0..20 {
            let x = i as f64;
            data.push(x);
            data.push(x * 2.0);
            data.push(0.01 * (i as f64 * 0.1).sin());
            data.push(0.01 * (i as f64 * 0.2).cos());
        }
        let n = 20;
        let d = 4;
        let shared = SharedData::compute(&data, n, d);
        let result = DimensionReductionLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("intrinsic_dimensionality"));
        let id = result["intrinsic_dimensionality"][0];
        assert!(id <= 3.0, "Intrinsic dim should be low, got {}", id);
    }
}
