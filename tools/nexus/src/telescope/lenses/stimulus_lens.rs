use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// StimulusLens: Neural stimulus detection.
/// Detects activation bursts, response latency, habituation, and sensitization.
pub struct StimulusLens;

impl Lens for StimulusLens {
    fn name(&self) -> &str { "StimulusLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        // Extract first-dimension signal
        let signal: Vec<f64> = (0..max_n).map(|i| data[i * d]).collect();
        let mean = signal.iter().sum::<f64>() / max_n as f64;
        let std_dev = (signal.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / max_n as f64).sqrt().max(1e-12);

        // Stimulus strength: fraction of points exceeding 2 std deviations
        let threshold = mean + 2.0 * std_dev;
        let burst_count = signal.iter().filter(|&&x| x > threshold).count();
        let stimulus_strength = burst_count as f64 / max_n as f64;

        // Response latency: average gap between consecutive bursts
        let burst_indices: Vec<usize> = (0..max_n).filter(|&i| signal[i] > threshold).collect();
        let response_latency = if burst_indices.len() >= 2 {
            let gaps: Vec<f64> = burst_indices.windows(2).map(|w| (w[1] - w[0]) as f64).collect();
            gaps.iter().sum::<f64>() / gaps.len() as f64
        } else {
            max_n as f64
        };

        // Habituation rate: decreasing response to repeated stimulus
        // Compare burst intensity in first half vs second half
        let mid = max_n / 2;
        let first_half_energy: f64 = signal[..mid].iter().map(|&x| (x - mean).abs()).sum::<f64>() / mid as f64;
        let second_half_energy: f64 = signal[mid..max_n].iter().map(|&x| (x - mean).abs()).sum::<f64>() / (max_n - mid) as f64;
        let habituation_rate = if first_half_energy > 1e-12 {
            ((first_half_energy - second_half_energy) / first_half_energy).clamp(-1.0, 1.0)
        } else {
            0.0
        };

        // Sensitization: increasing variance over windows
        let win = (max_n / 4).max(2);
        let mut window_vars = Vec::new();
        for start in (0..max_n).step_by(win) {
            let end = (start + win).min(max_n);
            if end - start < 2 { break; }
            let w = &signal[start..end];
            let wm = w.iter().sum::<f64>() / w.len() as f64;
            let wv = w.iter().map(|&x| (x - wm).powi(2)).sum::<f64>() / w.len() as f64;
            window_vars.push(wv);
        }
        let sensitization = if window_vars.len() >= 2 {
            let last = *window_vars.last().unwrap();
            let first = window_vars[0].max(1e-12);
            (last / first - 1.0).clamp(-1.0, 10.0)
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert("stimulus_strength".to_string(), vec![stimulus_strength]);
        result.insert("response_latency".to_string(), vec![response_latency]);
        result.insert("habituation_rate".to_string(), vec![habituation_rate]);
        result.insert("sensitization".to_string(), vec![sensitization]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let data: Vec<f64> = (0..30).map(|i| if i % 10 == 5 { 10.0 } else { (i as f64 * 0.1).sin() }).collect();
        let shared = SharedData::compute(&data, 30, 1);
        let result = StimulusLens.scan(&data, 30, 1, &shared);
        assert!(!result.is_empty());
        assert!(result.contains_key("stimulus_strength"));
    }
}
