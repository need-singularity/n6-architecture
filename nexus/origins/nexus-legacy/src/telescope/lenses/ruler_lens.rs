use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// RulerLens (직교): SVD effective rank approximation.
///
/// Algorithm:
///   1. Compute covariance matrix from data
///   2. Eigenvalue decomposition via power iteration (top-k eigenvalues)
///   3. Effective rank = exp(Shannon entropy of normalized eigenvalues)
///   4. Reports effective rank and explained variance ratio
pub struct RulerLens;

impl Lens for RulerLens {
    fn name(&self) -> &str {
        "RulerLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 3 || d == 0 {
            return HashMap::new();
        }

        // Compute mean-centered data
        let mut means = vec![0.0; d];
        for i in 0..n {
            for j in 0..d {
                means[j] += data[i * d + j];
            }
        }
        for m in means.iter_mut() {
            *m /= n as f64;
        }

        // Covariance matrix (d x d)
        let dd = d.min(50); // limit for large d
        let mut cov = vec![0.0; dd * dd];
        for i in 0..n {
            for r in 0..dd {
                let xr = data[i * d + r] - means[r];
                for c in r..dd {
                    let xc = data[i * d + c] - means[c];
                    cov[r * dd + c] += xr * xc;
                }
            }
        }
        // Symmetrize and normalize
        for r in 0..dd {
            for c in r..dd {
                cov[r * dd + c] /= (n - 1).max(1) as f64;
                cov[c * dd + r] = cov[r * dd + c];
            }
        }

        // Power iteration to find top eigenvalues
        let num_eig = dd.min(10);
        let mut eigenvalues: Vec<f64> = Vec::new();

        let mut deflated_cov = cov.clone();
        for _ in 0..num_eig {
            let eig = power_iteration(&deflated_cov, dd, 100);
            if eig.0 < 1e-15 {
                break;
            }
            eigenvalues.push(eig.0);

            // Deflate: C = C - lambda * v * v^T
            for r in 0..dd {
                for c in 0..dd {
                    deflated_cov[r * dd + c] -= eig.0 * eig.1[r] * eig.1[c];
                }
            }
        }

        if eigenvalues.is_empty() {
            return HashMap::new();
        }

        // Effective rank = exp(entropy of normalized eigenvalues)
        let total: f64 = eigenvalues.iter().sum();
        let effective_rank = if total > 1e-15 {
            let entropy: f64 = eigenvalues
                .iter()
                .filter(|&&e| e > 1e-30)
                .map(|&e| {
                    let p = e / total;
                    -p * p.ln()
                })
                .sum();
            entropy.exp()
        } else {
            1.0
        };

        // Explained variance ratio (first eigenvalue / total)
        let explained_ratio = if total > 1e-15 {
            eigenvalues[0] / total
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert("effective_rank".to_string(), vec![effective_rank]);
        result.insert("explained_variance_ratio".to_string(), vec![explained_ratio]);
        result
    }
}

/// Power iteration: returns (eigenvalue, eigenvector) for largest eigenvalue.
fn power_iteration(matrix: &[f64], d: usize, max_iter: usize) -> (f64, Vec<f64>) {
    let mut v: Vec<f64> = (0..d).map(|i| if i == 0 { 1.0 } else { 0.0 }).collect();
    let mut eigenvalue = 0.0;

    for _ in 0..max_iter {
        // w = M * v
        let mut w = vec![0.0; d];
        for r in 0..d {
            for c in 0..d {
                w[r] += matrix[r * d + c] * v[c];
            }
        }

        // Normalize
        let norm = w.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm < 1e-15 {
            return (0.0, v);
        }

        let new_eigenvalue = norm;
        for i in 0..d {
            w[i] /= norm;
        }

        let converged = (new_eigenvalue - eigenvalue).abs() < 1e-10;
        eigenvalue = new_eigenvalue;
        v = w;

        if converged {
            break;
        }
    }

    (eigenvalue, v)
}
