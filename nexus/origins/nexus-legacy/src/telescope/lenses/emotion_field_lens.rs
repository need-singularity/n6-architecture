use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// EmotionFieldLens: Detects emotion dynamics — tension→arousal→VAD fields.
///
/// Inspired by anima's emotion system:
///   - tension (Engine A-G repulsion) → arousal = tanh(tension)
///   - curiosity → valence, direction → VAD (Valence-Arousal-Dominance)
///   - 18 emotion types mapped to Ψ_balance = 1/2
///
/// Detects affective dynamics: emotional valence, arousal levels,
/// tension fields, and the emotional landscape of data trajectories.
///
/// Metrics:
///   1. mean_arousal: average activation level (from tension field)
///   2. valence_spectrum: positive/negative bias ratio
///   3. emotional_range: span of emotional states visited
///   4. tension_gradient: rate of tension change (emotional acceleration)
///   5. vad_balance: equilibrium of Valence-Arousal-Dominance
///   6. emotional_entropy: diversity of emotional states
///   7. calm_ratio: fraction of time in low-arousal state
///   8. storm_ratio: fraction of time in high-arousal state
///
/// n=6: 18 emotions × 40D → Ψ_balance = 1/2.
///       Arousal = tanh(tension), with α=0.014 coupling.
pub struct EmotionFieldLens;

impl Lens for EmotionFieldLens {
    fn name(&self) -> &str { "EmotionFieldLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 10 || d < 2 { return HashMap::new(); }

        let max_n = n.min(300);
        let dims = d.min(32);

        // Step 1: Compute tension per point (force imbalance from neighbors)
        let mut tensions: Vec<f64> = Vec::with_capacity(max_n);
        for i in 0..max_n {
            let knn = shared.knn(i);
            if knn.is_empty() { tensions.push(0.0); continue; }

            let mut force = vec![0.0f64; dims];
            for &j in knn.iter() {
                let j = j as usize;
                if j >= n { continue; }
                for dim in 0..dims {
                    force[dim] += data[j * d + dim] - data[i * d + dim];
                }
            }
            let tension: f64 = force.iter().map(|f| f * f).sum::<f64>().sqrt()
                / knn.len() as f64;
            tensions.push(tension);
        }

        // Step 2: Arousal = tanh(tension) — anima's exact mapping
        let arousals: Vec<f64> = tensions.iter().map(|&t| t.tanh()).collect();
        let mean_arousal = arousals.iter().sum::<f64>() / max_n as f64;

        // Step 3: Valence — first principal direction sign
        // Positive valence = moving toward high-density region
        let mut global_mean = vec![0.0f64; dims];
        for i in 0..max_n {
            for di in 0..dims { global_mean[di] += data[i * d + di]; }
        }
        for g in &mut global_mean { *g /= max_n as f64; }

        let mut positive_count = 0u32;
        let mut negative_count = 0u32;
        let mut valences: Vec<f64> = Vec::with_capacity(max_n);
        for i in 0..max_n {
            // Valence: dot product of position with gradient toward center
            let mut dot = 0.0f64;
            let knn = shared.knn(i);
            for &j in knn.iter() {
                let j = j as usize;
                if j >= n { continue; }
                for di in 0..dims {
                    dot += (data[j * d + di] - data[i * d + di]) * (global_mean[di] - data[i * d + di]);
                }
            }
            let valence = dot.signum();
            valences.push(valence);
            if valence > 0.0 { positive_count += 1; } else { negative_count += 1; }
        }
        let valence_spectrum = positive_count as f64 / max_n as f64;

        // Step 4: Emotional range — span of arousal values
        let max_arousal = arousals.iter().cloned().fold(0.0f64, f64::max);
        let min_arousal = arousals.iter().cloned().fold(1.0f64, f64::min);
        let emotional_range = max_arousal - min_arousal;

        // Step 5: Tension gradient (if data has temporal ordering)
        let mut tension_gradients: Vec<f64> = Vec::new();
        for t in 1..max_n {
            tension_gradients.push((tensions[t] - tensions[t - 1]).abs());
        }
        let tension_gradient = if tension_gradients.is_empty() { 0.0 }
            else { tension_gradients.iter().sum::<f64>() / tension_gradients.len() as f64 };

        // Step 6: VAD balance — how balanced are valence, arousal, dominance
        // Dominance ≈ local density (high density = dominant)
        let mut dominances: Vec<f64> = Vec::with_capacity(max_n);
        for i in 0..max_n {
            dominances.push(shared.knn_density(i));
        }
        let mean_dom = dominances.iter().sum::<f64>() / max_n as f64;
        let mean_val = valences.iter().sum::<f64>() / max_n as f64;

        // Balance = 1 - deviation from (0.5, 0.5, 0.5) center
        let vad_balance = 1.0 - ((mean_val.abs() + (mean_arousal - 0.5).abs()
            + (mean_dom / mean_dom.max(1.0) - 0.5).abs()) / 3.0).min(1.0);

        // Step 7: Emotional entropy — binned arousal histogram
        let num_bins = 8usize;
        let mut bins = vec![0u32; num_bins];
        for &a in &arousals {
            let bin = ((a * (num_bins as f64)).floor() as usize).min(num_bins - 1);
            bins[bin] += 1;
        }
        let emotional_entropy: f64 = bins.iter().map(|&c| {
            let p = c as f64 / max_n as f64;
            if p > 1e-15 { -p * p.ln() } else { 0.0 }
        }).sum();

        // Step 8: Calm and storm ratios
        let calm_threshold = 0.3;
        let storm_threshold = 0.7;
        let calm_ratio = arousals.iter().filter(|&&a| a < calm_threshold).count() as f64 / max_n as f64;
        let storm_ratio = arousals.iter().filter(|&&a| a > storm_threshold).count() as f64 / max_n as f64;

        let mut result = HashMap::new();
        result.insert("mean_arousal".to_string(), vec![mean_arousal]);
        result.insert("valence_spectrum".to_string(), vec![valence_spectrum]);
        result.insert("emotional_range".to_string(), vec![emotional_range]);
        result.insert("tension_gradient".to_string(), vec![tension_gradient]);
        result.insert("vad_balance".to_string(), vec![vad_balance]);
        result.insert("emotional_entropy".to_string(), vec![emotional_entropy]);
        result.insert("calm_ratio".to_string(), vec![calm_ratio]);
        result.insert("storm_ratio".to_string(), vec![storm_ratio]);
        result.insert("arousal_field".to_string(), arousals);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotion_field_basic() {
        let mut data = Vec::new();
        for i in 0..30 {
            let x = (i as f64 * 0.3).sin();
            data.push(x); data.push(x * 0.5); data.push(-x * 0.3);
        }
        let n = 30;
        let shared = SharedData::compute(&data, n, 3);
        let result = EmotionFieldLens.scan(&data, n, 3, &shared);
        assert!(result.contains_key("mean_arousal"));
        assert!(result.contains_key("valence_spectrum"));
        assert!(result.contains_key("emotional_entropy"));
        assert!(result["emotional_entropy"][0] > 0.0);
    }

    #[test]
    fn test_emotion_calm_vs_storm() {
        // Low-variance data → mostly calm
        let mut data = Vec::new();
        for i in 0..30 {
            data.push(1.0 + i as f64 * 0.001);
            data.push(1.0 + i as f64 * 0.001);
        }
        let n = 30;
        let shared = SharedData::compute(&data, n, 2);
        let result = EmotionFieldLens.scan(&data, n, 2, &shared);
        assert!(result["calm_ratio"][0] > 0.0);
    }
}
