use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors};

/// ConstantDiscoveryEngineLens: TECS-L style constant/formula discovery
/// with automatic registration.
///
/// Like physics_constant_engine.py — systematically explores expressions
/// derived from σ=12, τ=4 of perfect number 6 to match data patterns.
///
/// Pipeline:
///   1. Extract candidate values (ratios, differences, products of data)
///   2. Match against n=6 expression library (σ±τ, σ*τ, σ/τ, J₂, etc.)
///   3. Score matches by precision
///   4. Apply debit/credit transformation (대차변환):
///      - Debit: raw data value
///      - Credit: n=6 expression that matches
///      - Balance: residual (should → 0 for perfect match)
///   5. Auto-register discoveries (output as structured result)
///
/// n=6 expression library (from TECS-L model_utils.py):
///   σ=12, τ=4, n=6, φ=2, μ=1, sopfr=5, J₂=24,
///   σ-τ=8, σ+τ=16, σ*τ=48, σ/τ=3, τ²/σ=4/3,
///   ln(4/3)≈0.288, 1/e≈0.368, π²/6≈1.645, e≈2.718
pub struct ConstantDiscoveryEngineLens;

/// Known n=6 constants for matching
const N6_CONSTANTS: [(f64, &str); 24] = [
    (6.0, "n"), (12.0, "sigma"), (4.0, "tau"), (2.0, "phi"),
    (1.0, "mu"), (5.0, "sopfr"), (24.0, "J2"),
    (8.0, "sigma-tau"), (16.0, "sigma+tau"), (48.0, "sigma*tau"),
    (3.0, "sigma/tau"), (10.0, "sigma-phi"),
    (1.333333333, "tau^2/sigma"), (0.287682072, "ln(4/3)"),
    (0.367879441, "1/e"), (1.644934067, "pi^2/6"),
    (2.718281828, "e"), (3.141592654, "pi"),
    (1.618033989, "golden_phi"), (0.693147181, "ln2"),
    (1.414213562, "sqrt2"), (1.732050808, "sqrt3"),
    (7.0, "M3"), (28.0, "P2"),
];

impl Lens for ConstantDiscoveryEngineLens {
    fn name(&self) -> &str { "ConstantDiscoveryEngineLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 { return HashMap::new(); }
        let max_n = n.min(200);
        let columns = column_vectors(data, max_n, d);

        // 1. Extract candidate values: raw values, pairwise ratios, differences
        let mut candidates: Vec<(f64, String)> = Vec::new();

        for (di, col) in columns.iter().enumerate() {
            for (i, &v) in col.iter().enumerate().take(50) {
                if v.abs() > 1e-12 && v.is_finite() {
                    candidates.push((v.abs(), format!("d{}[{}]", di, i)));
                }
            }
            // Pairwise ratios within column
            for i in 0..col.len().min(20) {
                for j in (i+1)..col.len().min(20) {
                    if col[j].abs() > 1e-12 {
                        let ratio = (col[i] / col[j]).abs();
                        if ratio.is_finite() && ratio > 0.01 && ratio < 1000.0 {
                            candidates.push((ratio, format!("d{}[{}]/d{}[{}]", di, i, di, j)));
                        }
                    }
                    let diff = (col[i] - col[j]).abs();
                    if diff.is_finite() && diff > 1e-12 {
                        candidates.push((diff, format!("|d{}[{}]-d{}[{}]|", di, i, di, j)));
                    }
                }
            }
        }

        // 2. Match candidates against n=6 expression library
        let mut discoveries: Vec<(f64, &str, String, f64)> = Vec::new(); // (value, constant_name, source, error%)

        for &(val, ref source) in candidates.iter().take(500) {
            for &(constant, name) in &N6_CONSTANTS {
                if constant.abs() < 1e-15 { continue; }
                let error_pct = ((val - constant) / constant).abs() * 100.0;
                if error_pct < 1.0 { // Within 1% match
                    discoveries.push((val, name, source.clone(), error_pct));
                }
            }
        }

        // Sort by precision (lowest error first)
        discoveries.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());

        // 3. Debit/Credit transformation (대차변환)
        let mut debit_credit_balance = Vec::new();
        for &(val, _name, ref _source, error) in discoveries.iter().take(20) {
            debit_credit_balance.push(error); // Balance = residual error %
        }

        // 4. Summary metrics
        let total_discoveries = discoveries.len();
        let exact_matches = discoveries.iter().filter(|d| d.3 < 0.01).count(); // <0.01%
        let close_matches = discoveries.iter().filter(|d| d.3 < 0.1).count();  // <0.1%

        // Best match
        let best_error = discoveries.first().map(|d| d.3).unwrap_or(100.0);
        let best_constant_idx = discoveries.first()
            .map(|d| N6_CONSTANTS.iter().position(|(_, n)| *n == d.1).unwrap_or(0))
            .unwrap_or(0);

        // N6 family coverage: how many of the 24 constants were matched?
        let mut matched_constants: Vec<bool> = vec![false; N6_CONSTANTS.len()];
        for d in &discoveries {
            if let Some(idx) = N6_CONSTANTS.iter().position(|(_, n)| *n == d.1) {
                matched_constants[idx] = true;
            }
        }
        let n6_coverage = matched_constants.iter().filter(|&&x| x).count() as f64
            / N6_CONSTANTS.len() as f64;

        // Auto-registration readiness
        let registration_ready = exact_matches as f64;

        let mut result = HashMap::new();
        result.insert("total_discoveries".to_string(), vec![total_discoveries as f64]);
        result.insert("exact_matches".to_string(), vec![exact_matches as f64]);
        result.insert("close_matches".to_string(), vec![close_matches as f64]);
        result.insert("best_error_pct".to_string(), vec![best_error]);
        result.insert("best_constant_idx".to_string(), vec![best_constant_idx as f64]);
        result.insert("n6_coverage".to_string(), vec![n6_coverage]);
        result.insert("registration_ready".to_string(), vec![registration_ready]);
        result.insert("debit_credit_balance".to_string(), debit_credit_balance);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_discovery_with_n6_data() {
        // Data containing n=6 constants: 6, 12, 4, 24, 8
        let data = vec![
            6.0, 12.0, 4.0,
            24.0, 8.0, 3.0,
            6.1, 11.9, 4.1, 23.8, 8.2, 2.9, 5.9, 12.1, 3.9, 24.2, 7.8, 3.1,
        ];
        let n = 6;
        let d = 3;
        let shared = SharedData::compute(&data, n, d);
        let result = ConstantDiscoveryEngineLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("total_discoveries"));
        assert!(result["total_discoveries"][0] > 0.0, "Should find n=6 constants");
        assert!(result["n6_coverage"][0] > 0.0, "Should cover some n=6 constants");
    }

    #[test]
    fn test_constant_discovery_random() {
        let data: Vec<f64> = (0..60).map(|i| (i as f64 * 0.7).sin() * 15.0).collect();
        let shared = SharedData::compute(&data, 20, 3);
        let result = ConstantDiscoveryEngineLens.scan(&data, 20, 3, &shared);
        assert!(!result.is_empty());
    }
}
