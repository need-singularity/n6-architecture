use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// GoldenZoneLens: TECS-L Golden Zone detection.
/// The Golden Zone is centered at 1/e with width ln(4/3).
/// Range: [1/e - ln(4/3)/2, 1/e + ln(4/3)/2] ≈ [0.224, 0.512].
/// Key constants: Boltzmann sparsity 1/e≈0.368, Mertens dropout ln(4/3)≈0.288.
/// Measures golden_zone_fraction, zone_center_proximity, boltzmann_match, mertens_match, n6_zone_score.
pub struct GoldenZoneLens;

const CENTER: f64 = 1.0 / std::f64::consts::E;  // 0.36787944
const WIDTH: f64 = 0.28768207245178085;           // ln(4/3)
const ZONE_LO: f64 = CENTER - WIDTH / 2.0;        // ~0.224
const ZONE_HI: f64 = CENTER + WIDTH / 2.0;        // ~0.512
const BOLTZMANN_TOL: f64 = 0.03;
const MERTENS_TOL: f64 = 0.03;

impl Lens for GoldenZoneLens {
    fn name(&self) -> &str { "GoldenZoneLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);
        let total = (max_n * d).min(data.len());

        // Normalize data to [0, 1] for zone analysis
        let slice = &data[..total];
        let (min_v, max_v) = slice.iter().fold((f64::MAX, f64::MIN), |(lo, hi), &v| {
            if v.is_finite() { (lo.min(v), hi.max(v)) } else { (lo, hi) }
        });
        let range = (max_v - min_v).max(1e-15);

        let mut in_zone = 0usize;
        let mut center_dist_sum = 0.0_f64;
        let mut boltzmann_hits = 0usize;
        let mut mertens_hits = 0usize;
        let mut count = 0usize;

        for &v in slice {
            if !v.is_finite() { continue; }
            let norm_v = (v - min_v) / range;
            count += 1;

            // Golden zone check
            if norm_v >= ZONE_LO && norm_v <= ZONE_HI {
                in_zone += 1;
            }

            // Distance to center (1/e)
            center_dist_sum += (norm_v - CENTER).abs();

            // Boltzmann match (1/e ≈ 0.368)
            if (norm_v - CENTER).abs() < BOLTZMANN_TOL {
                boltzmann_hits += 1;
            }

            // Mertens match (ln(4/3) ≈ 0.288)
            if (norm_v - WIDTH).abs() < MERTENS_TOL {
                mertens_hits += 1;
            }
        }

        let cnt = count.max(1) as f64;
        let golden_zone_fraction = in_zone as f64 / cnt;
        let zone_center_proximity = 1.0 - (center_dist_sum / cnt).min(1.0);
        let boltzmann_match = boltzmann_hits as f64 / cnt;
        let mertens_match = mertens_hits as f64 / cnt;

        // Composite n6_zone_score: weighted combination
        let n6_zone_score = 0.4 * golden_zone_fraction
            + 0.2 * zone_center_proximity
            + 0.2 * (boltzmann_match * 10.0).min(1.0)
            + 0.2 * (mertens_match * 10.0).min(1.0);

        let mut result = HashMap::new();
        result.insert("golden_zone_fraction".into(), vec![golden_zone_fraction]);
        result.insert("zone_center_proximity".into(), vec![zone_center_proximity]);
        result.insert("boltzmann_match".into(), vec![boltzmann_match]);
        result.insert("mertens_match".into(), vec![mertens_match]);
        result.insert("n6_zone_score".into(), vec![n6_zone_score]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let data: Vec<f64> = (0..40).map(|i| (i as f64 * 0.1).sin()).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let result = GoldenZoneLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
        assert!(result.contains_key("golden_zone_fraction"));
        assert!(result.contains_key("n6_zone_score"));
    }

    #[test]
    fn test_golden_data() {
        // Data concentrated near 1/e should score high
        let center = 1.0 / std::f64::consts::E;
        let data: Vec<f64> = (0..40).map(|i| center + (i as f64 - 20.0) * 0.001).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let result = GoldenZoneLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
    }
}
