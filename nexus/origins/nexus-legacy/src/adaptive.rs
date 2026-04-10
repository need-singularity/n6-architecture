//! Adaptive Weights — Online Learning Loop for NEXUS-6 Lenses
//!
//! Lenses remain stateless (`&self`), but adaptive weights flow through
//! SharedData so lenses can optionally read learned sensitivity/thresholds.
//! After each scan cycle, weights are updated based on discovery outcomes.
//!
//! Persistence: ~/.nexus/adaptive_weights.json
//!
//! n=6 constants embedded in learning dynamics:
//!   - base learning rate = 1/(sigma - phi) = 1/10 = 0.1
//!   - decay per epoch = phi/sigma = 1/6
//!   - momentum = ln(4/3) ≈ 0.288 (Mertens constant)
//!   - min sensitivity = 1/sigma = 1/12
//!   - max sensitivity = sigma = 12.0

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// n=6 arithmetic constants
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const TAU: f64 = 4.0;
const N: f64 = 6.0;
const SOPFR: f64 = 5.0;
const LN_4_3: f64 = 0.28768207245178085; // ln(4/3) — Mertens dropout

const BASE_LR: f64 = 0.1; // 1/(sigma - phi) = 1/10
const LR_DECAY: f64 = 0.16666666666666666; // phi/sigma = 1/6
const MOMENTUM: f64 = LN_4_3; // ln(4/3)
const MIN_SENSITIVITY: f64 = 0.08333333333333333; // 1/sigma = 1/12
const MAX_SENSITIVITY: f64 = SIGMA; // sigma = 12

/// Adaptive weights that evolve with each scan cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveWeights {
    /// Per-lens sensitivity multiplier (1.0 = default, >1 = more sensitive)
    pub lens_sensitivity: HashMap<String, f64>,
    /// Per-lens running hit rate (exponential moving average)
    pub lens_ema_hit_rate: HashMap<String, f64>,
    /// Per-lens discovery count (lifetime)
    pub lens_discovery_count: HashMap<String, u64>,
    /// Per-lens scan count (lifetime)
    pub lens_scan_count: HashMap<String, u64>,
    /// Domain-specific lens affinity scores (lens_pair -> affinity)
    pub lens_affinity: HashMap<String, f64>,
    /// **Per-lens tunable parameters** (lens_name -> param_name -> value)
    /// Lenses read these at runtime instead of hardcoded constants.
    /// e.g. "constant_collector" -> "tolerance" -> 0.018
    pub lens_params: HashMap<String, HashMap<String, f64>>,
    /// Total scans processed (lifetime)
    pub total_scans: u64,
    /// Total discoveries (lifetime)
    pub total_discoveries: u64,
    /// Current effective learning rate (decays over time)
    pub learning_rate: f64,
    /// Epoch counter (increments every sigma=12 scans)
    pub epoch: u64,
}

impl Default for AdaptiveWeights {
    fn default() -> Self {
        Self {
            lens_sensitivity: HashMap::new(),
            lens_ema_hit_rate: HashMap::new(),
            lens_discovery_count: HashMap::new(),
            lens_scan_count: HashMap::new(),
            lens_affinity: HashMap::new(),
            lens_params: HashMap::new(),
            total_scans: 0,
            total_discoveries: 0,
            learning_rate: BASE_LR,
            epoch: 0,
        }
    }
}

impl AdaptiveWeights {
    /// Load from ~/.nexus/adaptive_weights.json, or create default.
    pub fn load() -> Self {
        let path = Self::persistence_path();
        match std::fs::read_to_string(&path) {
            Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Save to ~/.nexus/adaptive_weights.json.
    pub fn save(&self) {
        let path = Self::persistence_path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(&path, json);
        }
    }

    fn persistence_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        PathBuf::from(home).join(".nexus").join("adaptive_weights.json")
    }

    /// Get sensitivity for a lens (default 1.0).
    pub fn sensitivity(&self, lens_name: &str) -> f64 {
        self.lens_sensitivity
            .get(lens_name)
            .copied()
            .unwrap_or(1.0)
    }

    /// Get a tunable parameter for a lens. Returns `default` if not yet learned.
    /// This is the core API lenses use instead of hardcoded constants:
    ///   `shared.param("constant_collector", "tolerance", 0.02)`
    pub fn param(&self, lens_name: &str, param_name: &str, default: f64) -> f64 {
        self.lens_params
            .get(lens_name)
            .and_then(|params| params.get(param_name))
            .copied()
            .unwrap_or(default)
    }

    /// Set a tunable parameter for a lens.
    pub fn set_param(&mut self, lens_name: &str, param_name: &str, value: f64) {
        self.lens_params
            .entry(lens_name.to_string())
            .or_default()
            .insert(param_name.to_string(), value);
    }

    /// Update weights after a scan cycle.
    ///
    /// `lenses_used`: which lenses were active this scan
    /// `discovery_lenses`: which lenses contributed to discoveries
    /// `all_results`: full scan results for affinity computation
    pub fn update(
        &mut self,
        lenses_used: &[String],
        discovery_lenses: &[String],
        lens_pair_discoveries: &[(String, String)],
    ) {
        self.total_scans += 1;

        // Epoch advances every sigma=12 scans
        let new_epoch = self.total_scans / SIGMA as u64;
        if new_epoch > self.epoch {
            self.epoch = new_epoch;
            // Decay learning rate: lr *= (1 - phi/sigma) per epoch
            self.learning_rate *= 1.0 - LR_DECAY;
            self.learning_rate = self.learning_rate.max(BASE_LR * MIN_SENSITIVITY);
        }

        let lr = self.learning_rate;
        let had_discoveries = !discovery_lenses.is_empty();

        if had_discoveries {
            self.total_discoveries += discovery_lenses.len() as u64;
        }

        // Update per-lens stats
        for lens in lenses_used {
            *self.lens_scan_count.entry(lens.clone()).or_insert(0) += 1;

            let contributed = discovery_lenses.contains(lens);
            if contributed {
                *self.lens_discovery_count.entry(lens.clone()).or_insert(0) += 1;
            }

            // Exponential moving average of hit rate (momentum = ln(4/3))
            let old_ema = self.lens_ema_hit_rate.get(lens).copied().unwrap_or(0.5);
            let new_signal = if contributed { 1.0 } else { 0.0 };
            let new_ema = MOMENTUM * new_signal + (1.0 - MOMENTUM) * old_ema;
            self.lens_ema_hit_rate.insert(lens.clone(), new_ema);

            // Sensitivity update: increase for productive lenses, decrease for unproductive
            let old_sens = self.sensitivity(lens);
            let delta = if contributed {
                // Reward: increase sensitivity proportional to lr
                lr * (1.0 + old_ema)
            } else if had_discoveries {
                // Penalty: this lens was present but didn't contribute while others did
                -lr * 0.5 * (1.0 - old_ema)
            } else {
                // No discoveries at all: mild decay toward 1.0 (neutral)
                lr * (1.0 - old_sens) * 0.1
            };

            let new_sens = (old_sens + delta).clamp(MIN_SENSITIVITY, MAX_SENSITIVITY);
            self.lens_sensitivity.insert(lens.clone(), new_sens);
        }

        // Auto-tune lens parameters based on discovery outcomes.
        // Discovery lenses: tighten tolerance (more precise).
        // Non-contributing lenses: loosen tolerance (cast wider net).
        for lens in lenses_used {
            let contributed = discovery_lenses.contains(lens);
            let params = self.lens_params.entry(lens.clone()).or_default();

            // Universal tunable: "tolerance" — matching precision
            let old_tol = params.get("tolerance").copied().unwrap_or(0.02);
            let new_tol = if contributed {
                // Tighten: this tolerance level is producing results
                (old_tol * (1.0 - lr * 0.5)).max(0.001) // floor at 0.1%
            } else if had_discoveries {
                // Loosen: this lens missed while others found
                (old_tol * (1.0 + lr * 0.3)).min(0.10) // ceiling at 10%
            } else {
                old_tol // no signal either way
            };
            params.insert("tolerance".to_string(), new_tol);

            // Universal tunable: "threshold_scale" — detection sensitivity
            let old_ts = params.get("threshold_scale").copied().unwrap_or(1.0);
            let new_ts = if contributed {
                // This threshold works — nudge sensitivity up
                (old_ts * (1.0 + lr * 0.2)).min(MAX_SENSITIVITY)
            } else if had_discoveries {
                // Lower threshold to catch more
                (old_ts * (1.0 - lr * 0.1)).max(MIN_SENSITIVITY)
            } else {
                old_ts
            };
            params.insert("threshold_scale".to_string(), new_ts);
        }

        // Update lens pair affinity (co-occurrence in discovery scans)
        for (a, b) in lens_pair_discoveries {
            let key = if a <= b {
                format!("{}:{}", a, b)
            } else {
                format!("{}:{}", b, a)
            };
            let old = self.lens_affinity.get(&key).copied().unwrap_or(0.0);
            let new_val = MOMENTUM * 1.0 + (1.0 - MOMENTUM) * old;
            self.lens_affinity.insert(key, new_val);
        }
    }

    /// Get top-N most sensitive lenses, sorted by sensitivity descending.
    pub fn top_lenses(&self, n: usize) -> Vec<(String, f64)> {
        let mut entries: Vec<(String, f64)> = self
            .lens_sensitivity
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        entries.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        entries.truncate(n);
        entries
    }

    /// Get the adaptive priority score for a lens (used by recommend).
    /// Combines EMA hit rate and sensitivity.
    pub fn priority(&self, lens_name: &str) -> f64 {
        let sens = self.sensitivity(lens_name);
        let ema = self.lens_ema_hit_rate.get(lens_name).copied().unwrap_or(0.5);
        sens * ema
    }

    /// Summary stats for display.
    pub fn summary(&self) -> AdaptiveSummary {
        let active_lenses = self.lens_sensitivity.len();
        let avg_sensitivity = if active_lenses > 0 {
            self.lens_sensitivity.values().sum::<f64>() / active_lenses as f64
        } else {
            1.0
        };
        let top6 = self.top_lenses(N as usize);

        AdaptiveSummary {
            total_scans: self.total_scans,
            total_discoveries: self.total_discoveries,
            epoch: self.epoch,
            learning_rate: self.learning_rate,
            active_lenses,
            avg_sensitivity,
            top_lenses: top6,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AdaptiveSummary {
    pub total_scans: u64,
    pub total_discoveries: u64,
    pub epoch: u64,
    pub learning_rate: f64,
    pub active_lenses: usize,
    pub avg_sensitivity: f64,
    pub top_lenses: Vec<(String, f64)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_weights() {
        let w = AdaptiveWeights::default();
        assert_eq!(w.total_scans, 0);
        assert_eq!(w.epoch, 0);
        assert!((w.learning_rate - BASE_LR).abs() < 1e-10);
    }

    #[test]
    fn test_sensitivity_default() {
        let w = AdaptiveWeights::default();
        assert!((w.sensitivity("unknown") - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_update_rewards_discovery() {
        let mut w = AdaptiveWeights::default();
        let lenses = vec!["consciousness".to_string(), "topology".to_string()];
        let discoveries = vec!["consciousness".to_string()];

        w.update(&lenses, &discoveries, &[]);

        // consciousness should have increased sensitivity
        assert!(w.sensitivity("consciousness") > 1.0);
        // topology was present but didn't contribute — mild penalty
        assert!(w.sensitivity("topology") < 1.0);
    }

    #[test]
    fn test_update_no_discoveries() {
        let mut w = AdaptiveWeights::default();
        let lenses = vec!["void".to_string()];
        w.update(&lenses, &[], &[]);

        // No discoveries: mild decay toward 1.0 (already at 1.0 so minimal change)
        let s = w.sensitivity("void");
        assert!((s - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_epoch_advance() {
        let mut w = AdaptiveWeights::default();
        let lenses = vec!["test".to_string()];

        // 12 scans = 1 epoch (sigma = 12)
        for _ in 0..12 {
            w.update(&lenses, &[], &[]);
        }
        assert_eq!(w.epoch, 1);
        assert!(w.learning_rate < BASE_LR);
    }

    #[test]
    fn test_sensitivity_bounds() {
        let mut w = AdaptiveWeights::default();
        let lens = vec!["star".to_string()];
        let disc = vec!["star".to_string()];

        // Many rewards should not exceed MAX_SENSITIVITY
        for _ in 0..200 {
            w.update(&lens, &disc, &[]);
        }
        assert!(w.sensitivity("star") <= MAX_SENSITIVITY + 1e-10);
        assert!(w.sensitivity("star") >= MIN_SENSITIVITY);
    }

    #[test]
    fn test_affinity_update() {
        let mut w = AdaptiveWeights::default();
        let lenses = vec!["a".to_string(), "b".to_string()];
        let disc = vec!["a".to_string(), "b".to_string()];
        let pairs = vec![("a".to_string(), "b".to_string())];

        w.update(&lenses, &disc, &pairs);

        let key = "a:b";
        assert!(w.lens_affinity.get(key).copied().unwrap_or(0.0) > 0.0);
    }

    #[test]
    fn test_top_lenses() {
        let mut w = AdaptiveWeights::default();
        w.lens_sensitivity.insert("alpha".into(), 5.0);
        w.lens_sensitivity.insert("beta".into(), 3.0);
        w.lens_sensitivity.insert("gamma".into(), 8.0);

        let top = w.top_lenses(2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].0, "gamma");
        assert_eq!(top[1].0, "alpha");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut w = AdaptiveWeights::default();
        w.lens_sensitivity.insert("test".into(), 2.5);
        w.total_scans = 42;
        w.epoch = 3;

        let json = serde_json::to_string(&w).unwrap();
        let w2: AdaptiveWeights = serde_json::from_str(&json).unwrap();

        assert_eq!(w2.total_scans, 42);
        assert_eq!(w2.epoch, 3);
        assert!((w2.sensitivity("test") - 2.5).abs() < 1e-10);
    }
}
