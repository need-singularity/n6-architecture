use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// InfinityLens: Detect infinite/divergent behavior, convergence/divergence analysis.
///
/// n=6 connection: ζ(2)=π²/6, harmonic series diverges, perfect numbers are finite
/// sums of divisors. 6 is the bridge between finite and infinite structure.
pub struct InfinityLens;

impl Lens for InfinityLens {
    fn name(&self) -> &str {
        "InfinityLens"
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

        // Use first dimension as sequence
        let seq: Vec<f64> = (0..max_n).map(|i| data[i * d]).collect();

        // 1. Divergence rate: fit log|a_n| vs log(n) to detect growth rate.
        //    Slope > 0 => divergent, slope < 0 => convergent.
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;
        let mut sum_xx = 0.0;
        let mut valid = 0usize;
        for i in 1..seq.len() {
            let abs_val = seq[i].abs();
            if abs_val > 1e-15 {
                let x = (i as f64).ln();
                let y = abs_val.ln();
                sum_x += x;
                sum_y += y;
                sum_xy += x * y;
                sum_xx += x * x;
                valid += 1;
            }
        }
        let divergence_rate = if valid > 1 {
            let nf = valid as f64;
            let denom = nf * sum_xx - sum_x * sum_x;
            if denom.abs() > 1e-15 {
                (nf * sum_xy - sum_x * sum_y) / denom
            } else {
                0.0
            }
        } else {
            0.0
        };
        result.insert("divergence_rate".to_string(), vec![divergence_rate]);

        // 2. Convergence score: ratio test — average |a_{n+1}/a_n|.
        //    < 1 means convergent, > 1 means divergent.
        let mut ratio_sum = 0.0;
        let mut ratio_count = 0usize;
        for i in 0..seq.len() - 1 {
            if seq[i].abs() > 1e-15 {
                let ratio = (seq[i + 1] / seq[i]).abs();
                if ratio.is_finite() {
                    ratio_sum += ratio;
                    ratio_count += 1;
                }
            }
        }
        let avg_ratio = if ratio_count > 0 {
            ratio_sum / ratio_count as f64
        } else {
            1.0
        };
        // convergence_score: 1.0 = strongly convergent, 0.0 = divergent
        let convergence_score = (1.0 - avg_ratio).max(0.0).min(1.0);
        result.insert("convergence_score".to_string(), vec![convergence_score]);

        // 3. Series sum ratio: compare partial sums S_n/S_{n/2} to detect behavior.
        //    For convergent series this ratio → 1, for divergent it grows.
        if seq.len() >= 4 {
            let half = seq.len() / 2;
            let s_half: f64 = seq[..half].iter().sum();
            let s_full: f64 = seq.iter().sum();
            let series_sum_ratio = if s_half.abs() > 1e-15 {
                (s_full / s_half).abs()
            } else {
                0.0
            };
            result.insert("series_sum_ratio".to_string(), vec![series_sum_ratio]);
        }

        // 4. Tail heaviness: fraction of total absolute sum in last 1/6 of data (n=6).
        let tail_start = max_n - max_n / 6;
        let total_abs: f64 = seq.iter().map(|x| x.abs()).sum();
        if total_abs > 1e-15 {
            let tail_abs: f64 = seq[tail_start..].iter().map(|x| x.abs()).sum();
            let tail_heaviness = tail_abs / total_abs;
            result.insert("tail_heaviness".to_string(), vec![tail_heaviness]);
        }

        // 5. Zeno fraction: fraction of data values in interval [mean-ε, mean+ε]
        //    for decreasing ε. High concentration = convergence to a point.
        let mean = seq.iter().sum::<f64>() / seq.len() as f64;
        let max_dev = seq.iter().map(|&x| (x - mean).abs()).fold(0.0_f64, f64::max);
        if max_dev > 1e-15 {
            let eps = max_dev / 6.0; // n=6 subdivision
            let in_band = seq.iter().filter(|&&x| (x - mean).abs() <= eps).count();
            let zeno_fraction = in_band as f64 / seq.len() as f64;
            result.insert("zeno_fraction".to_string(), vec![zeno_fraction]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    #[test]
    fn test_infinity_lens_convergent() {
        // Geometric series with ratio 0.5: convergent
        let n = 20;
        let mut data = Vec::with_capacity(n);
        for i in 0..n {
            data.push(0.5_f64.powi(i as i32));
        }
        let shared = SharedData::compute(&data, n, 1);
        let result = InfinityLens.scan(&data, n, 1, &shared);
        assert!(!result.is_empty());

        let conv = result.get("convergence_score").unwrap()[0];
        assert!(conv > 0.3, "Geometric(0.5) should show convergence, got {}", conv);

        let div_rate = result.get("divergence_rate").unwrap()[0];
        assert!(div_rate < 0.0, "Converging series should have negative divergence rate, got {}", div_rate);
    }

    #[test]
    fn test_infinity_lens_divergent() {
        // Exponential growth: divergent
        let n = 20;
        let mut data = Vec::with_capacity(n);
        for i in 0..n {
            data.push(2.0_f64.powi(i as i32));
        }
        let shared = SharedData::compute(&data, n, 1);
        let result = InfinityLens.scan(&data, n, 1, &shared);
        assert!(!result.is_empty());

        let conv = result.get("convergence_score").unwrap()[0];
        assert!(conv < 0.1, "Exponential growth should not be convergent, got {}", conv);
    }

    #[test]
    fn test_infinity_lens_small_n() {
        let data = vec![1.0, 2.0, 3.0];
        let shared = SharedData::compute(&data, 3, 1);
        let result = InfinityLens.scan(&data, 3, 1, &shared);
        assert!(result.is_empty());
    }
}
