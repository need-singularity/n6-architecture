use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, shannon_entropy, mean_var};

/// ContractingScanLens: Multi-scale contracting scan.
/// Starts from global and zooms in. Measures coarse/fine structure, zoom gradient,
/// detail emergence, fractal consistency.
pub struct ContractingScanLens;

impl Lens for ContractingScanLens {
    fn name(&self) -> &str { "ContractingScanLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        let signal: Vec<f64> = (0..max_n).map(|i| data[i * d]).collect();

        // Define contracting scales: full -> half -> quarter -> ...
        let mut scales = Vec::new();
        let mut size = max_n;
        while size >= 4 {
            scales.push(size);
            size /= 2;
        }
        if scales.is_empty() { scales.push(max_n); }

        // At each scale, compute entropy and variance of downsampled signal
        let mut entropies = Vec::new();
        let mut variances = Vec::new();

        for &sz in &scales {
            // Downsample by averaging bins
            let bin_size = max_n / sz;
            let downsampled: Vec<f64> = (0..sz).map(|i| {
                let start = i * bin_size;
                let end = (start + bin_size).min(max_n);
                if end > start {
                    signal[start..end].iter().sum::<f64>() / (end - start) as f64
                } else {
                    0.0
                }
            }).collect();

            let ent = shannon_entropy(&downsampled, 10);
            entropies.push(ent);

            let (_, vars) = mean_var(&downsampled, sz, 1);
            variances.push(vars[0]);
        }

        // Coarse structure: entropy at largest scale
        let coarse_structure = entropies[0];

        // Fine structure: entropy at smallest scale
        let fine_structure = *entropies.last().unwrap_or(&0.0);

        // Zoom gradient: average change in entropy per scale step
        let zoom_gradient = if entropies.len() >= 2 {
            let total_change: f64 = entropies.windows(2)
                .map(|w| (w[1] - w[0]).abs())
                .sum();
            total_change / (entropies.len() - 1) as f64
        } else {
            0.0
        };

        // Detail emergence: at what scale new details appear (biggest entropy jump)
        let mut max_jump = 0.0f64;
        let mut emergence_idx = 0;
        for i in 1..entropies.len() {
            let jump = (entropies[i] - entropies[i - 1]).abs();
            if jump > max_jump {
                max_jump = jump;
                emergence_idx = i;
            }
        }
        let detail_emergence = if emergence_idx < scales.len() { scales[emergence_idx] as f64 } else { max_n as f64 };

        // Fractal consistency: self-similarity across scales
        // Compare variance ratios between consecutive scales
        let mut var_ratios = Vec::new();
        for i in 1..variances.len() {
            if variances[i - 1] > 1e-12 {
                var_ratios.push(variances[i] / variances[i - 1]);
            }
        }
        let fractal_consistency = if var_ratios.len() >= 2 {
            let mean_ratio = var_ratios.iter().sum::<f64>() / var_ratios.len() as f64;
            let var_of_ratios = var_ratios.iter()
                .map(|&r| (r - mean_ratio).powi(2))
                .sum::<f64>() / var_ratios.len() as f64;
            let cv = if mean_ratio.abs() > 1e-12 { var_of_ratios.sqrt() / mean_ratio.abs() } else { 1.0 };
            1.0 / (1.0 + cv) // high = consistent ratios = fractal
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert("coarse_structure".to_string(), vec![coarse_structure]);
        result.insert("fine_structure".to_string(), vec![fine_structure]);
        result.insert("zoom_gradient".to_string(), vec![zoom_gradient]);
        result.insert("detail_emergence".to_string(), vec![detail_emergence]);
        result.insert("fractal_consistency".to_string(), vec![fractal_consistency]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let data: Vec<f64> = (0..32).map(|i| (i as f64 * 0.3).sin() + (i as f64 * 1.7).cos() * 0.3).collect();
        let shared = SharedData::compute(&data, 32, 1);
        let result = ContractingScanLens.scan(&data, 32, 1, &shared);
        assert!(!result.is_empty());
        assert!(result["coarse_structure"][0] >= 0.0);
        assert!(result["detail_emergence"][0] >= 1.0);
    }
}
