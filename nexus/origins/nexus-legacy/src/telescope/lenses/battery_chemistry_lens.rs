use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, mean_var, column_vectors};

/// n=6 electrochemical constants (BT-27, BT-43, BT-57, BT-80~84)
const N: f64 = 6.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const TAU: f64 = 4.0;
const J2: f64 = 24.0;
const SOPFR: f64 = 5.0;

/// Key battery/electrochemistry constants
const BATTERY_CONSTANTS: &[(f64, &str)] = &[
    (6.0, "CN=n (octahedral)"),
    (12.0, "sigma (cell count)"),
    (24.0, "J2 (total electrons LiC6)"),
    (4.0, "tau (tetrahedral CN)"),
    (96.0, "sigma*(sigma-tau) (Tesla 96S)"),
    (192.0, "sigma*2^tau (192S)"),
    (3.7, "nominal_voltage_LiCoO2"),
    (4.2, "charge_cutoff_V"),
    (2.5, "discharge_cutoff_V"),
    (1.333, "tau_sq/sigma (4/3 SQ bandgap)"),
];

/// Coordination number target = 6 (octahedral universality, BT-43)
const CN_TARGET: f64 = 6.0;

/// BatteryChemistryLens: Detect electrochemical patterns in data.
///
/// Algorithm:
///   1. Compute feature statistics and check against battery constants
///   2. Detect coordination number = 6 patterns (nearest-neighbor count)
///   3. Identify voltage ratio patterns (charge/discharge cycles)
///   4. Detect capacity ladder structure (n→σ→J₂ progression)
///   5. Reports cn6_score, voltage_pattern, capacity_ladder, n6_battery_matches
pub struct BatteryChemistryLens;

impl Lens for BatteryChemistryLens {
    fn name(&self) -> &str {
        "BatteryChemistryLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 3 || d == 0 {
            return HashMap::new();
        }

        let (means, vars) = mean_var(data, n, d);
        let columns = column_vectors(data, n, d);

        // 1. Coordination number detection via k-NN
        //    CN=6 means each point has ~6 equidistant nearest neighbors
        let k = 6.min(n.saturating_sub(1));
        let mut cn6_scores = Vec::with_capacity(n);
        for i in 0..n {
            if k == 0 {
                cn6_scores.push(0.0);
                continue;
            }
            // Get distances to all other points
            let mut dists: Vec<f64> = (0..n)
                .filter(|&j| j != i)
                .map(|j| shared.dist(i, j))
                .collect();
            dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

            if dists.len() < k {
                cn6_scores.push(0.0);
                continue;
            }

            // Measure how uniform the first 6 neighbor distances are
            let knn_dists = &dists[..k];
            let mean_d = knn_dists.iter().sum::<f64>() / k as f64;
            if mean_d < 1e-12 {
                cn6_scores.push(1.0); // all at same distance = perfect CN
                continue;
            }
            let variance = knn_dists.iter()
                .map(|&dd| (dd - mean_d).powi(2))
                .sum::<f64>() / k as f64;
            let cv = variance.sqrt() / mean_d; // coefficient of variation
            // Low CV = uniform distances = good CN=6 structure
            cn6_scores.push((-cv * 5.0).exp());
        }
        let avg_cn6_score = cn6_scores.iter().sum::<f64>() / n as f64;

        // 2. Voltage pattern detection: look for cyclic patterns in features
        //    Battery charge/discharge = oscillation between bounds
        let mut voltage_pattern_score: f64 = 0.0;
        for col in &columns {
            if col.len() < 4 { continue; }
            // Count sign changes in first differences (oscillation)
            let mut sign_changes = 0;
            for w in col.windows(3) {
                let d1 = w[1] - w[0];
                let d2 = w[2] - w[1];
                if d1 * d2 < 0.0 {
                    sign_changes += 1;
                }
            }
            let osc_ratio = sign_changes as f64 / (col.len() - 2) as f64;
            voltage_pattern_score = voltage_pattern_score.max(osc_ratio);
        }

        // 3. Capacity ladder detection: check if sorted feature means
        //    follow n→σ→J₂ (6→12→24) doubling pattern
        let mut sorted_means: Vec<f64> = means.iter().filter(|&&m| m > 0.1).cloned().collect();
        sorted_means.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let mut ladder_score = 0.0;
        if sorted_means.len() >= 2 {
            let mut ratio_2_count = 0;
            let mut total_ratios = 0;
            for w in sorted_means.windows(2) {
                if w[0] > 1e-12 {
                    let ratio = w[1] / w[0];
                    total_ratios += 1;
                    // Check for doubling (φ=2) which is the battery cell ladder pattern
                    if (ratio - PHI).abs() < 0.2 || (ratio - 3.0).abs() < 0.3 {
                        ratio_2_count += 1;
                    }
                }
            }
            if total_ratios > 0 {
                ladder_score = ratio_2_count as f64 / total_ratios as f64;
            }
        }

        // 4. n=6 constant matching in feature means
        let mut n6_match_count = 0;
        let mut matched = Vec::new();
        for &m in &means {
            for &(c, _name) in BATTERY_CONSTANTS {
                let tol = if c.abs() > 1.0 { 0.05 } else { 0.15 };
                if c.abs() < 1e-12 { continue; }
                if ((m - c) / c).abs() < tol {
                    n6_match_count += 1;
                    matched.push(c);
                    break;
                }
            }
        }

        // 5. Octahedral regularity: check if knn_k ≈ 6
        let effective_cn = shared.knn_k as f64;
        let cn_proximity = (-(effective_cn - CN_TARGET).powi(2) / 4.0).exp();

        // Combined battery score
        let battery_score = avg_cn6_score * 0.3
            + voltage_pattern_score * 0.2
            + ladder_score * 0.2
            + cn_proximity * 0.15
            + (n6_match_count as f64 / d.max(1) as f64) * 0.15;

        let mut result = HashMap::new();
        result.insert("cn6_score".to_string(), vec![avg_cn6_score]);
        result.insert("voltage_pattern".to_string(), vec![voltage_pattern_score]);
        result.insert("capacity_ladder".to_string(), vec![ladder_score]);
        result.insert("cn_proximity".to_string(), vec![cn_proximity]);
        result.insert("n6_battery_matches".to_string(), vec![n6_match_count as f64]);
        result.insert("battery_score".to_string(), vec![battery_score]);
        result.insert("matched_constants".to_string(), matched);
        result.insert("cn6_per_point".to_string(), cn6_scores);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_shared(data: &[f64], n: usize, d: usize) -> SharedData {
        SharedData::compute(data, n, d)
    }

    #[test]
    fn test_battery_cn6_detection() {
        // Create 7 points: 1 center + 6 vertices of a regular octahedron
        // This should give high CN=6 score for the center point
        let mut data = vec![0.0, 0.0, 0.0]; // center
        let offsets = [
            [1.0, 0.0, 0.0], [-1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0], [0.0, -1.0, 0.0],
            [0.0, 0.0, 1.0], [0.0, 0.0, -1.0],
        ];
        for o in &offsets {
            data.extend_from_slice(o);
        }
        let n = 7;
        let d = 3;
        let shared = make_shared(&data, n, d);
        let result = BatteryChemistryLens.scan(&data, n, d, &shared);

        assert!(result.contains_key("cn6_score"));
        let cn6 = result["cn6_score"][0];
        assert!(cn6 > 0.3, "Octahedral structure should give decent CN=6 score, got {cn6}");
    }

    #[test]
    fn test_battery_voltage_oscillation() {
        // Simulate charge-discharge cycles: oscillating signal
        let n = 20;
        let d = 2;
        let mut data = Vec::with_capacity(n * d);
        for i in 0..n {
            let cycle = (i as f64 * std::f64::consts::PI / 3.0).sin();
            data.push(3.0 + cycle * 0.6); // voltage oscillating 2.4-3.6
            data.push(12.0);              // cell count = σ
        }
        let shared = make_shared(&data, n, d);
        let result = BatteryChemistryLens.scan(&data, n, d, &shared);

        assert!(result.contains_key("voltage_pattern"));
        let vp = result["voltage_pattern"][0];
        assert!(vp > 0.15, "Oscillating signal should show voltage pattern, got {vp}");
    }

    #[test]
    fn test_battery_capacity_ladder() {
        // Features with means 6, 12, 24 (n→σ→J₂ ladder)
        let n = 15;
        let d = 3;
        let mut data = Vec::with_capacity(n * d);
        for i in 0..n {
            let noise = (i as f64 * 0.1).sin() * 0.05;
            data.push(6.0 + noise);
            data.push(12.0 + noise);
            data.push(24.0 + noise);
        }
        let shared = make_shared(&data, n, d);
        let result = BatteryChemistryLens.scan(&data, n, d, &shared);

        assert!(result.contains_key("n6_battery_matches"));
        let matches = result["n6_battery_matches"][0];
        assert!(matches >= 2.0, "Should match n=6 battery constants, got {matches}");
    }
}
