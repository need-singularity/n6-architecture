use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, mean_var, shannon_entropy};

/// n=6 solar constants (BT-30, BT-63)
const SIGMA: f64 = 12.0;
const SIGMA_SQ: f64 = 144.0;  // σ² = 144 cells
const SOPFR: f64 = 5.0;
const N: f64 = 6.0;
const TAU: f64 = 4.0;
const PHI: f64 = 2.0;
const J2: f64 = 24.0;

/// Shockley-Queisser bandgap = 4/3 eV (BT-30, BT-111)
const SQ_BANDGAP: f64 = 4.0 / 3.0;

/// Solar panel cell counts (BT-63): 60, 72, 120, 144
const CELL_COUNTS: &[(f64, &str)] = &[
    (60.0, "sigma*sopfr"),
    (72.0, "sigma*n"),
    (120.0, "sigma*(sigma-phi)"),
    (144.0, "sigma_sq"),
];

/// Key solar/photovoltaic constants
const SOLAR_CONSTANTS: &[(f64, &str)] = &[
    (SQ_BANDGAP, "SQ_bandgap_eV"),
    (0.33, "SQ_efficiency_limit"),
    (60.0, "sigma*sopfr"),
    (72.0, "sigma*n"),
    (120.0, "sigma*(sigma-phi)"),
    (144.0, "sigma_sq"),
    (6.0, "n"),
    (12.0, "sigma"),
    (24.0, "J2"),
    (48.0, "sigma*tau"),
    (1.12, "Si_bandgap_eV"),
    (1.5, "CdTe_bandgap_eV"),
    (25.7, "V_thermal_mV_at_300K"),
];

const REL_TOL: f64 = 0.05;

/// SolarEfficiencyLens: Detect photovoltaic patterns in data.
///
/// Algorithm:
///   1. Check feature values against SQ bandgap = 4/3 eV proximity
///   2. Detect cell count patterns (60, 72, 120, 144 = n=6 family)
///   3. Measure efficiency-like ratios (output/input bounded by SQ limit)
///   4. Check for σ² = 144 structural patterns
///   5. Reports sq_proximity, cell_count_matches, efficiency_pattern, solar_score
pub struct SolarEfficiencyLens;

impl Lens for SolarEfficiencyLens {
    fn name(&self) -> &str {
        "SolarEfficiencyLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 3 || d == 0 {
            return HashMap::new();
        }

        let (means, vars) = mean_var(data, n, d);

        // 1. SQ bandgap proximity: check if any feature mean ≈ 4/3
        let mut sq_proximities = Vec::with_capacity(d);
        let mut best_sq = f64::MAX;
        for &m in &means {
            let dist = (m - SQ_BANDGAP).abs();
            sq_proximities.push((-dist * 3.0).exp());
            if dist < best_sq {
                best_sq = dist;
            }
        }
        let sq_proximity = (-best_sq * 3.0).exp();

        // Also check ratio features: any ratio of features ≈ 4/3?
        let mut ratio_43_count = 0;
        for i in 0..d {
            for j in 0..d {
                if i == j { continue; }
                if means[j].abs() > 1e-12 {
                    let ratio = means[i] / means[j];
                    if (ratio - SQ_BANDGAP).abs() < 0.1 {
                        ratio_43_count += 1;
                    }
                }
            }
        }

        // 2. Cell count matching: check individual data values and n*d
        let mut cell_matches = 0;
        let mut matched_cells = Vec::new();
        // Check feature means
        for &m in &means {
            for &(cc, _) in CELL_COUNTS {
                if cc > 0.0 && ((m - cc) / cc).abs() < REL_TOL {
                    cell_matches += 1;
                    matched_cells.push(cc);
                    break;
                }
            }
        }
        // Check n*d product
        let nd = (n * d) as f64;
        for &(cc, _) in CELL_COUNTS {
            if cc > 0.0 && ((nd - cc) / cc).abs() < REL_TOL {
                cell_matches += 1;
                matched_cells.push(cc);
                break;
            }
        }

        // 3. Efficiency pattern: look for values in [0, 1] range
        //    that cluster near the SQ limit ~0.33
        let mut efficiency_vals = Vec::new();
        for i in 0..n {
            for j in 0..d {
                let v = data[i * d + j];
                if v >= 0.0 && v <= 1.0 {
                    efficiency_vals.push(v);
                }
            }
        }
        let efficiency_pattern = if !efficiency_vals.is_empty() {
            let near_sq = efficiency_vals.iter()
                .filter(|&&v| (v - 0.333).abs() < 0.1)
                .count();
            near_sq as f64 / efficiency_vals.len() as f64
        } else {
            0.0
        };

        // 4. σ² = 144 structural detection
        //    Check if data has 144-like periodicity or structure
        let sigma_sq_match = means.iter()
            .any(|&m| ((m - SIGMA_SQ) / SIGMA_SQ).abs() < REL_TOL);

        // 5. General n=6 solar constant matching
        let mut n6_solar_matches = 0;
        for &m in &means {
            for &(c, _) in SOLAR_CONSTANTS {
                if c.abs() < 1e-12 { continue; }
                let tol = if c.abs() > 1.0 { REL_TOL } else { 0.15 };
                if ((m - c) / c).abs() < tol {
                    n6_solar_matches += 1;
                    break;
                }
            }
        }

        // 6. Variance structure: solar panels have highly regular structure
        //    Low variance = uniform cell output = good solar pattern
        let total_var: f64 = vars.iter().sum();
        let total_mean: f64 = means.iter().map(|m| m.abs()).sum::<f64>().max(1e-12);
        let uniformity = (-total_var / total_mean).exp();

        // Combined solar score
        let solar_score = sq_proximity * 0.25
            + (cell_matches as f64 / (d + 1).max(1) as f64) * 0.25
            + efficiency_pattern * 0.2
            + uniformity * 0.15
            + (n6_solar_matches as f64 / d.max(1) as f64) * 0.15;

        let mut result = HashMap::new();
        result.insert("sq_proximity".to_string(), vec![sq_proximity]);
        result.insert("sq_bandgap_target".to_string(), vec![SQ_BANDGAP]);
        result.insert("cell_count_matches".to_string(), vec![cell_matches as f64]);
        result.insert("matched_cell_counts".to_string(), matched_cells);
        result.insert("efficiency_pattern".to_string(), vec![efficiency_pattern]);
        result.insert("ratio_43_count".to_string(), vec![ratio_43_count as f64]);
        result.insert("sigma_sq_match".to_string(), vec![if sigma_sq_match { 1.0 } else { 0.0 }]);
        result.insert("n6_solar_matches".to_string(), vec![n6_solar_matches as f64]);
        result.insert("uniformity".to_string(), vec![uniformity]);
        result.insert("solar_score".to_string(), vec![solar_score]);
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
    fn test_solar_sq_bandgap_detection() {
        // Feature mean near 4/3 = 1.333 eV
        let n = 15;
        let d = 2;
        let mut data = Vec::with_capacity(n * d);
        for i in 0..n {
            let noise = (i as f64 * 0.1).sin() * 0.01;
            data.push(1.333 + noise);  // ≈ SQ bandgap
            data.push(0.33 + noise * 0.1); // ≈ SQ efficiency
        }
        let shared = make_shared(&data, n, d);
        let result = SolarEfficiencyLens.scan(&data, n, d, &shared);

        assert!(result.contains_key("sq_proximity"));
        let sq = result["sq_proximity"][0];
        assert!(sq > 0.8, "Should detect SQ bandgap proximity, got {sq}");
    }

    #[test]
    fn test_solar_cell_count_matching() {
        // Features with means at solar cell counts: 60, 144
        let n = 10;
        let d = 3;
        let mut data = Vec::with_capacity(n * d);
        for _i in 0..n {
            data.push(60.0);
            data.push(72.0);
            data.push(144.0);
        }
        let shared = make_shared(&data, n, d);
        let result = SolarEfficiencyLens.scan(&data, n, d, &shared);

        let matches = result["cell_count_matches"][0];
        assert!(matches >= 2.0, "Should match cell counts, got {matches}");
    }

    #[test]
    fn test_solar_ratio_43() {
        // Two features with ratio 4/3
        let n = 10;
        let d = 2;
        let mut data = Vec::with_capacity(n * d);
        for _i in 0..n {
            data.push(3.0);
            data.push(4.0); // ratio = 4/3
        }
        let shared = make_shared(&data, n, d);
        let result = SolarEfficiencyLens.scan(&data, n, d, &shared);

        let r43 = result["ratio_43_count"][0];
        assert!(r43 >= 1.0, "Should detect 4/3 ratio, got {r43}");
    }
}
