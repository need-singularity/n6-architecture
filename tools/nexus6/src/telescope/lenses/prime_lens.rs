use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// PrimeLens: Detect prime-number-like patterns — gap distributions, density, sieve structure.
///
/// n=6 connection: 6 = 2·3 (product of first two primes), all primes >3 are 6k±1,
/// prime gaps are often multiples of 6.
pub struct PrimeLens;

/// Check if an integer is prime (for small values used in analysis).
fn is_prime(val: u64) -> bool {
    if val < 2 {
        return false;
    }
    if val < 4 {
        return true;
    }
    if val % 2 == 0 || val % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= val {
        if val % i == 0 || val % (i + 2) == 0 {
            return false;
        }
        i += 6; // 6k±1 sieve — n=6 connection
    }
    true
}

impl Lens for PrimeLens {
    fn name(&self) -> &str {
        "PrimeLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 {
            return HashMap::new();
        }

        let max_n = n.min(200);
        let mut result = HashMap::new();

        // Use first dimension, sorted for gap analysis
        let mut vals: Vec<f64> = (0..max_n).map(|i| data[i * d]).collect();
        vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // 1. Prime density: fraction of rounded absolute values that are prime.
        let mut prime_count = 0usize;
        let mut total_checked = 0usize;
        for &v in &vals {
            let abs_int = v.abs().round() as u64;
            if abs_int >= 2 && abs_int <= 10_000 {
                if is_prime(abs_int) {
                    prime_count += 1;
                }
                total_checked += 1;
            }
        }
        let prime_density = if total_checked > 0 {
            prime_count as f64 / total_checked as f64
        } else {
            0.0
        };
        result.insert("prime_density".to_string(), vec![prime_density]);

        // 2. Gap variance: compute gaps between sorted values, compare variance
        //    to what prime gaps would look like (variance / mean^2).
        let mut gaps = Vec::with_capacity(vals.len());
        for i in 1..vals.len() {
            let gap = vals[i] - vals[i - 1];
            if gap > 1e-15 {
                gaps.push(gap);
            }
        }
        if gaps.len() >= 2 {
            let mean_gap = gaps.iter().sum::<f64>() / gaps.len() as f64;
            let var_gap = gaps.iter().map(|&g| (g - mean_gap).powi(2)).sum::<f64>() / gaps.len() as f64;
            let gap_variance = if mean_gap > 1e-15 {
                var_gap / (mean_gap * mean_gap) // coefficient of variation squared
            } else {
                0.0
            };
            result.insert("gap_variance".to_string(), vec![gap_variance]);
        }

        // 3. Twin fraction: fraction of gaps that are "twin" (very close pairs).
        //    Twin primes differ by 2; here we define "twin" as gap < median_gap * 0.5.
        if gaps.len() >= 4 {
            let mut sorted_gaps = gaps.clone();
            sorted_gaps.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let median_gap = sorted_gaps[sorted_gaps.len() / 2];
            if median_gap > 1e-15 {
                let twins = gaps.iter().filter(|&&g| g < median_gap * 0.5).count();
                let twin_fraction = twins as f64 / gaps.len() as f64;
                result.insert("twin_fraction".to_string(), vec![twin_fraction]);
            }
        }

        // 4. Prime staircase fit: how well cumulative count fits x/ln(x).
        //    Use sorted data as "x" values and index as π(x).
        //    Compute R² of index vs x/ln(x).
        if vals.len() >= 6 {
            let mut sum_y = 0.0;
            let mut sum_pred = 0.0;
            let mut ss_res = 0.0;
            let mut count = 0usize;
            for (i, &v) in vals.iter().enumerate() {
                if v > 2.0 {
                    let y = (i + 1) as f64;
                    let pred = v / v.ln();
                    sum_y += y;
                    sum_pred += pred;
                    count += 1;
                }
            }
            if count >= 3 {
                let mean_y = sum_y / count as f64;
                let mut ss_tot = 0.0;
                let mut idx = 0;
                for (i, &v) in vals.iter().enumerate() {
                    if v > 2.0 {
                        let y = (i + 1) as f64;
                        // Scale prediction to match actual range
                        let pred = v / v.ln();
                        let scale = mean_y / (sum_pred / count as f64);
                        let pred_scaled = pred * scale;
                        ss_res += (y - pred_scaled).powi(2);
                        ss_tot += (y - mean_y).powi(2);
                        let _ = idx;
                    }
                }
                let staircase_fit = if ss_tot > 1e-15 {
                    (1.0 - ss_res / ss_tot).max(0.0)
                } else {
                    0.0
                };
                result.insert("prime_staircase_fit".to_string(), vec![staircase_fit]);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    #[test]
    fn test_prime_lens_with_primes() {
        // Data consisting of prime numbers — should have high prime_density
        let data: Vec<f64> = vec![2.0, 3.0, 5.0, 7.0, 11.0, 13.0, 17.0, 19.0, 23.0, 29.0];
        let n = data.len();
        let shared = SharedData::compute(&data, n, 1);
        let result = PrimeLens.scan(&data, n, 1, &shared);
        assert!(!result.is_empty());

        let density = result.get("prime_density").unwrap()[0];
        assert!(density > 0.9, "All-prime data should have high density, got {}", density);
    }

    #[test]
    fn test_prime_lens_with_composites() {
        // Data of composite numbers — lower prime density
        let data: Vec<f64> = vec![4.0, 6.0, 8.0, 9.0, 10.0, 12.0, 14.0, 15.0, 16.0, 18.0];
        let n = data.len();
        let shared = SharedData::compute(&data, n, 1);
        let result = PrimeLens.scan(&data, n, 1, &shared);
        assert!(!result.is_empty());

        let density = result.get("prime_density").unwrap()[0];
        assert!(density < 0.2, "Composite data should have low prime density, got {}", density);
    }

    #[test]
    fn test_prime_lens_small_n() {
        let data = vec![2.0, 3.0, 5.0];
        let shared = SharedData::compute(&data, 3, 1);
        let result = PrimeLens.scan(&data, 3, 1, &shared);
        assert!(result.is_empty());
    }
}
