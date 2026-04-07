use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, mean_var};
use crate::verifier::n6_check;

/// RatioLens: Detect n=6 constant ratios between feature statistics.
///
/// Computes all pairwise ratios of feature means, variances, and ranges,
/// then checks each ratio against the n=6 constant database.
pub struct RatioLens;

impl Lens for RatioLens {
    fn name(&self) -> &str { "RatioLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 3 || d < 2 { return HashMap::new(); }

        let (means, vars) = mean_var(data, n, d);

        // Compute ranges per feature
        let mut ranges = Vec::with_capacity(d);
        for j in 0..d {
            let mut lo = f64::INFINITY;
            let mut hi = f64::NEG_INFINITY;
            for i in 0..n {
                let v = data[i * d + j];
                if v < lo { lo = v; }
                if v > hi { hi = v; }
            }
            ranges.push(hi - lo);
        }

        let dd = d.min(12); // Cap at sigma=12
        let mut n6_ratio_matches = 0;
        let mut total_ratios = 0;
        let mut matched_ratios = Vec::new();

        // Check mean ratios
        for i in 0..dd {
            for j in (i + 1)..dd.min(d) {
                if means[j].abs() > 1e-12 {
                    let ratio = means[i] / means[j];
                    total_ratios += 1;
                    let (_, q) = n6_check::n6_match(ratio.abs());
                    if q >= 0.8 {
                        n6_ratio_matches += 1;
                        matched_ratios.push(ratio.abs());
                    }
                }
                if means[i].abs() > 1e-12 {
                    let ratio = means[j] / means[i];
                    total_ratios += 1;
                    let (_, q) = n6_check::n6_match(ratio.abs());
                    if q >= 0.8 {
                        n6_ratio_matches += 1;
                        matched_ratios.push(ratio.abs());
                    }
                }
            }
        }

        // Check variance ratios (F-statistic like)
        for i in 0..dd {
            for j in (i + 1)..dd.min(d) {
                if vars[j] > 1e-12 {
                    let ratio = vars[i] / vars[j];
                    total_ratios += 1;
                    let (_, q) = n6_check::n6_match(ratio);
                    if q >= 0.8 {
                        n6_ratio_matches += 1;
                        matched_ratios.push(ratio);
                    }
                }
            }
        }

        let ratio_match_rate = if total_ratios > 0 {
            n6_ratio_matches as f64 / total_ratios as f64
        } else { 0.0 };

        // Also check individual means against n6
        let mut mean_n6_matches = 0;
        for j in 0..d {
            let (_, q) = n6_check::n6_match(means[j].abs());
            if q >= 0.8 { mean_n6_matches += 1; }
        }

        let mut result = HashMap::new();
        result.insert("n6_ratio_matches".to_string(), vec![n6_ratio_matches as f64]);
        result.insert("total_ratios_checked".to_string(), vec![total_ratios as f64]);
        result.insert("ratio_match_rate".to_string(), vec![ratio_match_rate]);
        result.insert("mean_n6_matches".to_string(), vec![mean_n6_matches as f64]);
        result.insert("matched_ratios".to_string(), matched_ratios);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ratio_lens_n6_data() {
        // Features with means 6, 12 -> ratio = 2 = phi
        let mut data = Vec::new();
        for i in 0..20 {
            let noise = (i as f64 * 0.1).sin() * 0.01;
            data.push(6.0 + noise);
            data.push(12.0 + noise);
        }
        let n = 20;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let result = RatioLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("n6_ratio_matches"));
        assert!(result["n6_ratio_matches"][0] >= 1.0, "Should detect ratio=2=phi");
    }
}
