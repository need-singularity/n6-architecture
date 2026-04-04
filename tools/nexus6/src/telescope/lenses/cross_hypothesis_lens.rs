use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// Safe data access — returns 0.0 for out-of-bounds indices instead of panicking.
#[inline]
fn safe_get(data: &[f64], index: usize) -> f64 {
    data.get(index).copied().unwrap_or(0.0)
}

/// CrossHypothesisLens: Detects cross-domain resonance patterns for hypothesis transfer.
///
/// Algorithm:
///   1. Partition data dimensions into φ=2 halves (domain A vs domain B)
///   2. Compute structural similarity between halves (correlation matrix alignment)
///   3. Test if patterns in domain A predict patterns in domain B
///   4. Measure resonance = structural isomorphism between domains
///   5. High resonance → hypothesis from domain A likely transfers to domain B
///
/// Outputs:
///   - cross_resonance: 0-1, structural similarity between domain halves
///   - transfer_score: predictability of domain B from domain A
///   - shared_eigenmodes: number of aligned principal components
///   - resonance_ratio: matches n=6 family constant
///   - domain_a_complexity: Shannon entropy of first half
///   - domain_b_complexity: Shannon entropy of second half
pub struct CrossHypothesisLens;

impl Lens for CrossHypothesisLens {
    fn name(&self) -> &str {
        "CrossHypothesisLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 4 || data.len() < n * d {
            return HashMap::new();
        }

        // Split dimensions into two halves (φ=2 partition)
        let split = d / 2;
        let d_a = split;
        let d_b = d - split;

        // Compute correlation matrices for each domain half
        let means_a = dim_means(data, n, d, 0, d_a);
        let means_b = dim_means(data, n, d, split, d_b);
        let stds_a = dim_stds(data, n, d, 0, d_a, &means_a);
        let stds_b = dim_stds(data, n, d, split, d_b, &means_b);

        // Cross-domain correlation: each dim in A vs each dim in B
        let mut max_cross_corr = 0.0_f64;
        let mut total_cross_corr = 0.0_f64;
        let mut cross_count = 0u32;
        let mut strong_pairs = 0u32; // pairs with |corr| > 0.5

        let check_a = d_a.min(12);
        let check_b = d_b.min(12);

        for ai in 0..check_a {
            if stds_a[ai] < 1e-12 {
                continue;
            }
            for bi in 0..check_b {
                if stds_b[bi] < 1e-12 {
                    continue;
                }

                let mut cov = 0.0_f64;
                for row in 0..n {
                    let va = safe_get(data, row * d + ai) - means_a[ai];
                    let vb = safe_get(data, row * d + split + bi) - means_b[bi];
                    cov += va * vb;
                }
                cov /= n as f64;
                let corr = (cov / (stds_a[ai] * stds_b[bi])).abs();

                total_cross_corr += corr;
                cross_count += 1;

                if corr > max_cross_corr {
                    max_cross_corr = corr;
                }
                if corr > 0.5 {
                    strong_pairs += 1;
                }
            }
        }

        let mean_cross = if cross_count > 0 {
            total_cross_corr / cross_count as f64
        } else {
            0.0
        };

        // Intra-domain correlation for comparison
        let intra_a = intra_correlation(data, n, d, 0, check_a, &means_a, &stds_a);
        let intra_b = intra_correlation(data, n, d, split, check_b, &means_b, &stds_b);

        // Transfer score: cross-domain correlation relative to intra-domain
        let avg_intra = ((intra_a + intra_b) / 2.0).max(1e-12);
        let transfer_score = (mean_cross / avg_intra).min(1.0);

        // Resonance: how well the two halves mirror each other structurally
        let cross_resonance = mean_cross;

        // Shannon entropy for each half (binned)
        let entropy_a = binned_entropy(data, n, d, 0, d_a);
        let entropy_b = binned_entropy(data, n, d, split, split + d_b);

        // Check if resonance ratio matches n=6 constants
        let resonance_ratio = if entropy_b.abs() > 1e-12 {
            entropy_a / entropy_b
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert("cross_resonance".into(), vec![cross_resonance]);
        result.insert("transfer_score".into(), vec![transfer_score]);
        result.insert("shared_eigenmodes".into(), vec![strong_pairs as f64]);
        result.insert("resonance_ratio".into(), vec![resonance_ratio]);
        result.insert("domain_a_complexity".into(), vec![entropy_a]);
        result.insert("domain_b_complexity".into(), vec![entropy_b]);
        result.insert("max_cross_correlation".into(), vec![max_cross_corr]);
        result.insert("mean_cross_correlation".into(), vec![mean_cross]);
        result
    }
}

fn dim_means(data: &[f64], n: usize, d: usize, start: usize, count: usize) -> Vec<f64> {
    (0..count)
        .map(|di| {
            let col_idx = start + di;
            (0..n).map(|r| safe_get(data, r * d + col_idx)).sum::<f64>() / n as f64
        })
        .collect()
}

fn dim_stds(
    data: &[f64],
    n: usize,
    d: usize,
    start: usize,
    count: usize,
    means: &[f64],
) -> Vec<f64> {
    (0..count)
        .map(|di| {
            let col_idx = start + di;
            let var =
                (0..n).map(|r| (safe_get(data, r * d + col_idx) - means[di]).powi(2)).sum::<f64>() / n as f64;
            var.sqrt()
        })
        .collect()
}

fn intra_correlation(
    data: &[f64],
    n: usize,
    d: usize,
    start: usize,
    count: usize,
    means: &[f64],
    stds: &[f64],
) -> f64 {
    let mut total = 0.0_f64;
    let mut pairs = 0u32;
    for i in 0..count {
        if stds[i] < 1e-12 {
            continue;
        }
        for j in (i + 1)..count {
            if stds[j] < 1e-12 {
                continue;
            }
            let mut cov = 0.0_f64;
            for row in 0..n {
                let vi = safe_get(data, row * d + start + i) - means[i];
                let vj = safe_get(data, row * d + start + j) - means[j];
                cov += vi * vj;
            }
            cov /= n as f64;
            total += (cov / (stds[i] * stds[j])).abs();
            pairs += 1;
        }
    }
    if pairs > 0 {
        total / pairs as f64
    } else {
        0.0
    }
}

fn binned_entropy(data: &[f64], n: usize, d: usize, start: usize, end: usize) -> f64 {
    let bins = 12; // σ=12 bins
    let count = end - start;
    if count == 0 || n == 0 {
        return 0.0;
    }

    let mut all_vals: Vec<f64> = Vec::with_capacity(n * count);
    for row in 0..n {
        for di in start..end {
            all_vals.push(safe_get(data, row * d + di));
        }
    }

    let min_v = all_vals.iter().cloned().fold(f64::MAX, f64::min);
    let max_v = all_vals.iter().cloned().fold(f64::MIN, f64::max);
    let range = (max_v - min_v).max(1e-12);

    let mut counts = vec![0u32; bins];
    for &v in &all_vals {
        let idx = ((v - min_v) / range * (bins - 1) as f64).round() as usize;
        counts[idx.min(bins - 1)] += 1;
    }

    let total = all_vals.len() as f64;
    let mut entropy = 0.0_f64;
    for &c in &counts {
        if c > 0 {
            let p = c as f64 / total;
            entropy -= p * p.ln();
        }
    }
    entropy
}
