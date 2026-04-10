use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// MemoryLens: autocorrelation + echo state properties.
///
/// Algorithm:
///   1. Compute autocorrelation function for each dimension
///   2. Memory length = lag where autocorrelation drops below 1/e
///   3. Echo index = ratio of long-range to short-range correlation
///   4. Reports memory length and echo index
pub struct MemoryLens;

impl Lens for MemoryLens {
    fn name(&self) -> &str {
        "MemoryLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 {
            return HashMap::new();
        }

        let max_lag = (n / 3).max(1);
        let threshold = 1.0 / std::f64::consts::E; // 1/e

        let mut memory_lengths: Vec<f64> = Vec::with_capacity(d);
        let mut echo_indices: Vec<f64> = Vec::with_capacity(d);

        for dim in 0..d.min(10) {
            let signal: Vec<f64> = (0..n).map(|i| data[i * d + dim]).collect();
            let mean = signal.iter().sum::<f64>() / n as f64;
            let var: f64 = signal.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / n as f64;

            if var < 1e-15 {
                memory_lengths.push(0.0);
                echo_indices.push(0.0);
                continue;
            }

            // Autocorrelation at each lag
            let mut acf: Vec<f64> = Vec::with_capacity(max_lag);
            let mut memory_len = max_lag as f64;
            let mut found = false;

            for lag in 1..=max_lag {
                let effective = n - lag;
                let corr: f64 = (0..effective)
                    .map(|t| (signal[t] - mean) * (signal[t + lag] - mean))
                    .sum::<f64>()
                    / (effective as f64 * var);

                acf.push(corr);

                if !found && corr.abs() < threshold {
                    memory_len = lag as f64;
                    found = true;
                }
            }

            memory_lengths.push(memory_len);

            // Echo index: ratio of mean |acf| at long range to short range
            let short_range = acf.iter().take(max_lag / 3).map(|x| x.abs()).sum::<f64>()
                / (max_lag / 3).max(1) as f64;
            let long_range = acf.iter().skip(max_lag * 2 / 3).map(|x| x.abs()).sum::<f64>()
                / (max_lag / 3).max(1) as f64;

            let echo = if short_range > 1e-15 {
                long_range / short_range
            } else {
                0.0
            };
            echo_indices.push(echo);
        }

        let mean_memory = memory_lengths.iter().sum::<f64>() / memory_lengths.len() as f64;
        let mean_echo = echo_indices.iter().sum::<f64>() / echo_indices.len() as f64;

        let mut result = HashMap::new();
        result.insert("mean_memory_length".to_string(), vec![mean_memory]);
        result.insert("mean_echo_index".to_string(), vec![mean_echo]);
        result
    }
}
