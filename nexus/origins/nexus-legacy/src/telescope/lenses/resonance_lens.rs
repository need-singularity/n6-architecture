use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ResonanceLens: Detect resonance and coupling patterns between dimensions.
///
/// Measures how strongly different dimensions of the data oscillate
/// in sync (phase-locked) or at integer frequency ratios.
/// n=6 connection: perfect number divisors 1,2,3,6 give the fundamental
/// resonance ratios; BT-74 shows 95/5 cross-domain resonance.
pub struct ResonanceLens;

impl Lens for ResonanceLens {
    fn name(&self) -> &str { "ResonanceLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 8 || d < 2 { return HashMap::new(); }

        let dd = d.min(12); // sigma=12 max dimensions to check

        // Compute cross-correlation between dimension pairs
        let mut max_cross_corr = 0.0_f64;
        let mut mean_cross_corr = 0.0;
        let mut n6_ratio_count = 0u32;
        let mut pair_count = 0u32;

        for di in 0..dd {
            for dj in (di + 1)..dd {
                let col_i: Vec<f64> = (0..n).map(|r| data[r * d + di]).collect();
                let col_j: Vec<f64> = (0..n).map(|r| data[r * d + dj]).collect();

                let mi = col_i.iter().sum::<f64>() / n as f64;
                let mj = col_j.iter().sum::<f64>() / n as f64;
                let vi: f64 = col_i.iter().map(|x| (x - mi).powi(2)).sum::<f64>() / n as f64;
                let vj: f64 = col_j.iter().map(|x| (x - mj).powi(2)).sum::<f64>() / n as f64;

                if vi < 1e-12 || vj < 1e-12 { continue; }

                // Cross-correlation at lag 0
                let cov: f64 = col_i.iter().zip(col_j.iter())
                    .map(|(a, b)| (a - mi) * (b - mj))
                    .sum::<f64>() / n as f64;
                let corr = cov / (vi.sqrt() * vj.sqrt());
                let abs_corr = corr.abs();

                max_cross_corr = max_cross_corr.max(abs_corr);
                mean_cross_corr += abs_corr;
                pair_count += 1;

                // Check MI ratio for integer resonance (from shared data)
                let mi_val = shared.mi(di, dj);
                let mi_ii = shared.mi(di, di);
                if mi_ii > 1e-12 {
                    let ratio = mi_val / mi_ii;
                    // Check if ratio is close to a divisor of 6: 1/6, 1/3, 1/2, 1
                    let n6_divisor_ratios = [1.0 / 6.0, 1.0 / 3.0, 0.5, 1.0];
                    for &target in &n6_divisor_ratios {
                        if (ratio - target).abs() < 0.1 {
                            n6_ratio_count += 1;
                            break;
                        }
                    }
                }
            }
        }

        if pair_count == 0 { return HashMap::new(); }
        mean_cross_corr /= pair_count as f64;

        // Resonance strength: fraction of pairs with strong correlation
        let resonance_strength = max_cross_corr;

        // Phase coherence: how consistent are correlations across pairs
        let coherence = if max_cross_corr > 1e-12 {
            mean_cross_corr / max_cross_corr
        } else {
            0.0
        };

        // n=6 resonance ratio: fraction of pairs matching divisor ratios
        let n6_resonance = n6_ratio_count as f64 / pair_count as f64;

        let mut result = HashMap::new();
        result.insert("max_cross_correlation".to_string(), vec![max_cross_corr]);
        result.insert("mean_cross_correlation".to_string(), vec![mean_cross_corr]);
        result.insert("resonance_strength".to_string(), vec![resonance_strength]);
        result.insert("phase_coherence".to_string(), vec![coherence]);
        result.insert("n6_resonance_ratio".to_string(), vec![n6_resonance]);
        result.insert("resonant_pair_count".to_string(), vec![n6_ratio_count as f64]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resonance_correlated() {
        // Two perfectly correlated dimensions
        let n = 20;
        let d = 2;
        let data: Vec<f64> = (0..n).flat_map(|i| {
            let x = i as f64;
            vec![x, x * 2.0] // perfectly correlated
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = ResonanceLens.scan(&data, n, d, &shared);
        assert!(result["max_cross_correlation"][0] > 0.9);
    }

    #[test]
    fn test_resonance_independent() {
        // Two dimensions with different patterns
        let n = 20;
        let d = 2;
        let data: Vec<f64> = (0..n).flat_map(|i| {
            vec![i as f64, ((i * 7 + 3) % 11) as f64]
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = ResonanceLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("resonance_strength"));
    }
}
