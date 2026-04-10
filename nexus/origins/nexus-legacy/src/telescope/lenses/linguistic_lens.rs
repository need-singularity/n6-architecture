use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// LinguisticLens: Detect Zipf's law and rank-frequency patterns.
///
/// Zipf's law states frequency ~ 1/rank^alpha with alpha ~ 1.
/// This lens ranks data values by frequency and measures the
/// power-law exponent. n=6 connection: natural language has
/// ~sigma=12 phoneme classes, vocabulary scales as 2^(n=6)=64
/// basic morphemes, and Heaps' law exponent ~ ln(4/3)=0.288.
pub struct LinguisticLens;

impl Lens for LinguisticLens {
    fn name(&self) -> &str { "LinguisticLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        // Flatten all values and bin them to create a "vocabulary"
        let total = n * d;
        let mut values: Vec<f64> = data[..total].to_vec();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Create frequency distribution via binning
        let n_bins = (total as f64).sqrt().ceil() as usize;
        let n_bins = n_bins.max(6).min(100); // at least n=6 bins
        let (lo, hi) = (values[0], values[values.len() - 1]);
        let range = (hi - lo).max(1e-12);
        let bin_width = range / n_bins as f64;

        let mut counts = vec![0usize; n_bins];
        for &v in &values {
            let bin = ((v - lo) / bin_width) as usize;
            counts[bin.min(n_bins - 1)] += 1;
        }

        // Sort counts descending (rank-frequency)
        let mut freq: Vec<usize> = counts.into_iter().filter(|&c| c > 0).collect();
        freq.sort_unstable_by(|a, b| b.cmp(a));

        if freq.len() < 3 { return HashMap::new(); }

        // Fit Zipf exponent: log(freq) = -alpha * log(rank) + const
        let log_rank: Vec<f64> = (0..freq.len()).map(|i| ((i + 1) as f64).ln()).collect();
        let log_freq: Vec<f64> = freq.iter().map(|&f| (f as f64).ln()).collect();

        let (alpha, r2) = linear_reg(&log_rank, &log_freq);
        let zipf_exponent = -alpha; // Zipf alpha is positive

        // Match to Zipf's law (alpha ~ 1.0 = mu=1)
        let zipf_match = (-(zipf_exponent - 1.0).abs() * 6.0).exp();

        // Vocabulary richness: unique bins / total bins
        let vocab_richness = freq.len() as f64 / n_bins as f64;

        // Entropy rate (bits per symbol)
        let total_count: usize = freq.iter().sum();
        let entropy: f64 = freq.iter()
            .map(|&f| {
                let p = f as f64 / total_count as f64;
                if p > 0.0 { -p * p.log2() } else { 0.0 }
            })
            .sum();

        // n=6 vocabulary check: does vocab size match n=6 constants?
        let vocab_size = freq.len() as f64;
        let n6_constants = [6.0, 12.0, 24.0, 4.0, 2.0, 8.0, 10.0];
        let n6_vocab_match = n6_constants.iter()
            .map(|&c| (-(vocab_size - c).abs() / c).exp())
            .fold(0.0_f64, f64::max);

        let mut result = HashMap::new();
        result.insert("zipf_exponent".to_string(), vec![zipf_exponent]);
        result.insert("zipf_r2".to_string(), vec![r2]);
        result.insert("zipf_match".to_string(), vec![zipf_match]);
        result.insert("vocab_richness".to_string(), vec![vocab_richness]);
        result.insert("entropy_rate_bits".to_string(), vec![entropy]);
        result.insert("n6_vocab_match".to_string(), vec![n6_vocab_match]);
        result
    }
}

fn linear_reg(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len().min(y.len());
    if n < 2 { return (0.0, 0.0); }
    let nf = n as f64;
    let sx: f64 = x[..n].iter().sum();
    let sy: f64 = y[..n].iter().sum();
    let sxy: f64 = x[..n].iter().zip(y[..n].iter()).map(|(a, b)| a * b).sum();
    let sx2: f64 = x[..n].iter().map(|a| a * a).sum();
    let sy2: f64 = y[..n].iter().map(|a| a * a).sum();
    let denom = nf * sx2 - sx * sx;
    if denom.abs() < 1e-15 { return (0.0, 0.0); }
    let slope = (nf * sxy - sx * sy) / denom;
    let r_num = nf * sxy - sx * sy;
    let r_denom = ((nf * sx2 - sx * sx) * (nf * sy2 - sy * sy)).sqrt();
    let r2 = if r_denom > 1e-15 { (r_num / r_denom).powi(2) } else { 0.0 };
    (slope, r2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linguistic_zipf() {
        // Generate Zipf-distributed data: many low values, few high
        let n = 100;
        let d = 1;
        let mut data = Vec::with_capacity(n);
        for i in 1..=n {
            data.push(1.0 / i as f64);
        }
        let shared = SharedData::compute(&data, n, d);
        let result = LinguisticLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("zipf_exponent"));
        assert!(result["vocab_richness"][0] > 0.0);
    }

    #[test]
    fn test_linguistic_uniform() {
        // Uniform data: very different from Zipf
        let n = 30;
        let d = 1;
        let data: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = LinguisticLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("zipf_exponent"));
    }
}
