//! Lens Calibration Engine
//!
//! Determines each lens's accuracy/reliability by testing against known datasets
//! with ground-truth patterns. Produces hit_rates for consensus weighting.

use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

// ── n=6 constants (magic number elimination) ──────────────────────────
const N: f64 = 6.0; // the perfect number
const SIGMA: f64 = 12.0; // σ(6) = sum of divisors
const PHI: f64 = 2.0; // φ(6) = Euler totient
const TAU: f64 = 4.0; // τ(6) = number of divisors
const J2: f64 = 24.0; // J₂(6) = Jordan totient
const SOPFR: f64 = 5.0; // sopfr(6) = 2+3
const SIGMA_MINUS_PHI: f64 = 10.0; // σ-φ = 10
const SIGMA_MINUS_TAU: f64 = 8.0; // σ-τ = 8

// ── Tier thresholds ───────────────────────────────────────────────────
const T0_THRESHOLD: f64 = 0.8; // high reliability
const T1_THRESHOLD: f64 = 0.5; // moderate reliability

/// A ground-truth pattern that a lens should detect.
#[derive(Debug, Clone)]
pub struct KnownPattern {
    /// Metric name the lens should produce (key in LensResult)
    pub metric_name: String,
    /// Expected values for that metric
    pub expected_values: Vec<f64>,
    /// Acceptable relative tolerance (0.0..1.0)
    pub tolerance: f64,
}

/// A calibration dataset with known ground-truth patterns.
#[derive(Debug, Clone)]
pub struct CalibrationDataset {
    /// Human-readable name
    pub name: String,
    /// Row-major data (n*d elements)
    pub data: Vec<f64>,
    /// Number of data points
    pub n: usize,
    /// Dimensionality
    pub d: usize,
    /// Ground-truth patterns this dataset contains
    pub known_patterns: Vec<KnownPattern>,
}

/// Calibration result for a single lens.
#[derive(Debug, Clone)]
pub struct CalibrationResult {
    /// Name of the calibrated lens
    pub lens_name: String,
    /// Overall hit rate (0.0..1.0) — fraction of patterns correctly detected
    pub hit_rate: f64,
    /// False positive rate — metrics produced that don't match any known pattern
    pub false_positive_rate: f64,
    /// False negative rate — known patterns the lens missed entirely
    pub false_negative_rate: f64,
    /// Average scan latency in milliseconds (wall clock)
    pub latency_ms: f64,
    /// How well the lens's numeric outputs align with n=6 constants
    pub n6_alignment: f64,
}

/// Aggregated calibration report for all lenses.
#[derive(Debug, Clone)]
pub struct CalibrationReport {
    /// Per-lens results
    pub results: Vec<CalibrationResult>,
    /// Total lenses attempted
    pub total_lenses: usize,
    /// Lenses that produced at least one result
    pub calibrated_lenses: usize,
    /// Mean hit rate across all lenses
    pub mean_hit_rate: f64,
    /// Count of lenses in each tier: "T0", "T1", "T2"
    pub tier_distribution: HashMap<String, usize>,
}

/// Assign a tier label based on hit_rate.
///
/// - T0: hit_rate >= 0.8 (high reliability, σ-φ/σ threshold)
/// - T1: hit_rate >= 0.5 (moderate reliability)
/// - T2: below 0.5
pub fn tier_assignment(result: &CalibrationResult) -> &'static str {
    if result.hit_rate >= T0_THRESHOLD {
        "T0"
    } else if result.hit_rate >= T1_THRESHOLD {
        "T1"
    } else {
        "T2"
    }
}

/// Check if a single lens output matches a known pattern within tolerance.
///
/// Returns true if the lens produced a metric whose values are close enough
/// to the expected values (element-wise relative difference < tolerance).
fn pattern_matches(lens_result: &LensResult, pattern: &KnownPattern) -> bool {
    let values = match lens_result.get(&pattern.metric_name) {
        Some(v) if !v.is_empty() => v,
        _ => return false,
    };

    // If expected is empty, just check that the lens produced *something*
    if pattern.expected_values.is_empty() {
        return true;
    }

    // Compare element-wise up to the shorter length
    let len = values.len().min(pattern.expected_values.len());
    if len == 0 {
        return false;
    }

    let mut matches = 0usize;
    for i in 0..len {
        let expected = pattern.expected_values[i];
        let actual = values[i];
        let diff = (actual - expected).abs();
        let scale = expected.abs().max(1e-12);
        if diff / scale <= pattern.tolerance {
            matches += 1;
        }
    }

    // Require at least half of compared elements to match
    matches * 2 >= len
}

/// Measure how well a set of numeric values aligns with n=6 constants.
///
/// For each value, compute the minimum relative distance to the nearest
/// n=6 constant. Return 1.0 - mean_distance (clamped to [0, 1]).
fn compute_n6_alignment(lens_result: &LensResult) -> f64 {
    const ATTRACTORS: [f64; 8] = [N, SIGMA, PHI, TAU, J2, SOPFR, SIGMA_MINUS_PHI, SIGMA_MINUS_TAU];

    let mut total_distance = 0.0;
    let mut count = 0usize;

    for values in lens_result.values() {
        for &v in values {
            if !v.is_finite() || v.abs() < 1e-15 {
                continue;
            }
            let min_rel = ATTRACTORS
                .iter()
                .map(|&a| {
                    let diff = (v - a).abs();
                    let scale = a.abs().max(v.abs()).max(1e-12);
                    diff / scale
                })
                .fold(f64::MAX, f64::min);
            total_distance += min_rel.min(1.0);
            count += 1;
        }
    }

    if count == 0 {
        return 0.0;
    }

    let mean_dist = total_distance / count as f64;
    (1.0 - mean_dist).max(0.0)
}

/// Calibrate a single lens against multiple datasets.
///
/// Runs the lens on each dataset, compares outputs to known patterns,
/// and computes accuracy metrics.
pub fn calibrate_lens(
    lens: &dyn Lens,
    datasets: &[CalibrationDataset],
) -> CalibrationResult {
    if datasets.is_empty() {
        return CalibrationResult {
            lens_name: lens.name().to_string(),
            hit_rate: 0.0,
            false_positive_rate: 1.0,
            false_negative_rate: 1.0,
            latency_ms: 0.0,
            n6_alignment: 0.0,
        };
    }

    let mut total_patterns = 0usize;
    let mut true_positives = 0usize;
    let mut false_negatives = 0usize;
    let mut total_metrics_produced = 0usize;
    let mut total_metrics_matched = 0usize;
    let mut total_latency_ns = 0u128;
    let mut n6_sum = 0.0f64;
    let mut n6_count = 0usize;

    for dataset in datasets {
        let shared = SharedData::compute(&dataset.data, dataset.n, dataset.d);

        let start = std::time::Instant::now();
        let result = lens.scan(&dataset.data, dataset.n, dataset.d, &shared);
        let elapsed = start.elapsed();
        total_latency_ns += elapsed.as_nanos();

        // Count matches against known patterns
        for pattern in &dataset.known_patterns {
            total_patterns += 1;
            if pattern_matches(&result, pattern) {
                true_positives += 1;
            } else {
                false_negatives += 1;
            }
        }

        // Count false positives: metrics the lens produced that don't correspond
        // to any known pattern in this dataset
        let known_names: Vec<&str> = dataset
            .known_patterns
            .iter()
            .map(|p| p.metric_name.as_str())
            .collect();

        for (metric_name, values) in &result {
            if !values.is_empty() {
                total_metrics_produced += 1;
                if known_names.contains(&metric_name.as_str()) {
                    total_metrics_matched += 1;
                }
            }
        }

        // n=6 alignment for this scan
        let alignment = compute_n6_alignment(&result);
        n6_sum += alignment;
        n6_count += 1;
    }

    let hit_rate = if total_patterns > 0 {
        true_positives as f64 / total_patterns as f64
    } else {
        0.0
    };

    let false_positive_rate = if total_metrics_produced > 0 {
        let fp = total_metrics_produced.saturating_sub(total_metrics_matched);
        fp as f64 / total_metrics_produced as f64
    } else {
        0.0
    };

    let false_negative_rate = if total_patterns > 0 {
        false_negatives as f64 / total_patterns as f64
    } else {
        0.0
    };

    let latency_ms = total_latency_ns as f64 / 1_000_000.0 / datasets.len().max(1) as f64;

    let n6_alignment = if n6_count > 0 {
        n6_sum / n6_count as f64
    } else {
        0.0
    };

    CalibrationResult {
        lens_name: lens.name().to_string(),
        hit_rate,
        false_positive_rate,
        false_negative_rate,
        latency_ms,
        n6_alignment,
    }
}

/// Calibrate all lenses and produce an aggregated report.
pub fn calibrate_all(
    lenses: &[Box<dyn Lens>],
    datasets: &[CalibrationDataset],
) -> CalibrationReport {
    let mut results = Vec::with_capacity(lenses.len());

    for lens in lenses {
        let cr = calibrate_lens(lens.as_ref(), datasets);
        results.push(cr);
    }

    let total_lenses = results.len();
    let calibrated_lenses = results.iter().filter(|r| r.hit_rate > 0.0).count();

    let mean_hit_rate = if total_lenses > 0 {
        results.iter().map(|r| r.hit_rate).sum::<f64>() / total_lenses as f64
    } else {
        0.0
    };

    let mut tier_distribution = HashMap::new();
    for r in &results {
        let tier = tier_assignment(r);
        *tier_distribution.entry(tier.to_string()).or_insert(0) += 1;
    }

    CalibrationReport {
        results,
        total_lenses,
        calibrated_lenses,
        mean_hit_rate,
        tier_distribution,
    }
}

/// Extract a hit_rates map from a calibration report, suitable for
/// passing to `weighted_consensus()`.
pub fn update_hit_rates(report: &CalibrationReport) -> HashMap<String, f64> {
    let mut hit_rates = HashMap::with_capacity(report.results.len());
    for r in &report.results {
        hit_rates.insert(r.lens_name.clone(), r.hit_rate);
    }
    hit_rates
}

/// Generate synthetic calibration datasets with known n=6 patterns.
///
/// Each dataset is crafted so that certain metrics (periodic at σ=12,
/// clustered around J₂=24, etc.) are ground-truth detectable.
pub fn generate_synthetic_datasets() -> Vec<CalibrationDataset> {
    let mut datasets = Vec::new();

    // ── Dataset 1: Periodic σ=12 signal ───────────────────────────────
    // 24 points in 2D with period-12 oscillation in x-dimension
    {
        let n = J2 as usize; // 24 points = J₂
        let d = PHI as usize; // 2 dimensions = φ
        let mut data = Vec::with_capacity(n * d);
        for i in 0..n {
            let t = i as f64;
            let x = (t * std::f64::consts::TAU / SIGMA).sin() * SIGMA; // period σ=12
            let y = t * PHI; // linear ramp
            data.push(x);
            data.push(y);
        }
        datasets.push(CalibrationDataset {
            name: "periodic_sigma12".to_string(),
            data,
            n,
            d,
            known_patterns: vec![
                KnownPattern {
                    metric_name: "period".to_string(),
                    expected_values: vec![SIGMA], // period = 12
                    tolerance: 0.2,               // 20% tolerance
                },
                KnownPattern {
                    metric_name: "dominant_frequency".to_string(),
                    expected_values: vec![1.0 / SIGMA], // 1/12
                    tolerance: 0.3,
                },
            ],
        });
    }

    // ── Dataset 2: Cluster centers at n=6 constants ───────────────────
    // 6 clusters centered at (n, σ), (φ, τ), (J₂, sopfr) with small spread
    {
        let centers: [(f64, f64); 3] = [
            (N, SIGMA),
            (PHI, TAU),
            (J2, SOPFR),
        ];
        let points_per_cluster = N as usize; // n=6 points per cluster
        let n = points_per_cluster * centers.len();
        let d = PHI as usize; // 2D
        let mut data = Vec::with_capacity(n * d);
        let mut rng: u64 = 6; // seed = n

        for &(cx, cy) in &centers {
            for _ in 0..points_per_cluster {
                rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                let rx = ((rng >> 33) as f64) / (u32::MAX as f64) - 0.5;
                rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                let ry = ((rng >> 33) as f64) / (u32::MAX as f64) - 0.5;
                data.push(cx + rx * 0.5);
                data.push(cy + ry * 0.5);
            }
        }
        datasets.push(CalibrationDataset {
            name: "n6_clusters".to_string(),
            data,
            n,
            d,
            known_patterns: vec![
                KnownPattern {
                    metric_name: "cluster_count".to_string(),
                    expected_values: vec![3.0], // n/φ = 3 clusters
                    tolerance: 0.1,
                },
                KnownPattern {
                    metric_name: "cluster_centers".to_string(),
                    expected_values: vec![N, PHI, J2], // x-coords of centers
                    tolerance: 0.3,
                },
            ],
        });
    }

    // ── Dataset 3: Linear ramp with σ-τ=8 slope ──────────────────────
    {
        let n = SIGMA as usize; // 12 points
        let d = PHI as usize; // 2D
        let mut data = Vec::with_capacity(n * d);
        for i in 0..n {
            let x = i as f64;
            let y = SIGMA_MINUS_TAU * x; // slope = σ-τ = 8
            data.push(x);
            data.push(y);
        }
        datasets.push(CalibrationDataset {
            name: "linear_sigma_minus_tau".to_string(),
            data,
            n,
            d,
            known_patterns: vec![
                KnownPattern {
                    metric_name: "slope".to_string(),
                    expected_values: vec![SIGMA_MINUS_TAU],
                    tolerance: 0.15,
                },
                KnownPattern {
                    metric_name: "correlation".to_string(),
                    expected_values: vec![1.0], // perfect linear
                    tolerance: 0.05,
                },
            ],
        });
    }

    // ── Dataset 4: Uniform noise (no pattern — tests false-positive resistance) ─
    {
        let n = J2 as usize; // 24 points
        let d = TAU as usize; // 4 dimensions = τ
        let mut data = Vec::with_capacity(n * d);
        let mut rng: u64 = 42;
        for _ in 0..(n * d) {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let v = ((rng >> 33) as f64) / (u32::MAX as f64);
            data.push(v * 100.0); // uniform [0, 100]
        }
        datasets.push(CalibrationDataset {
            name: "uniform_noise".to_string(),
            data,
            n,
            d,
            known_patterns: vec![], // no ground-truth patterns — everything is false positive
        });
    }

    // ── Dataset 5: n=6 attractor convergence ──────────────────────────
    // Points that spiral toward (6, 12, 24) in 3D
    {
        let n = J2 as usize; // 24 steps
        let d = 3; // 3D (n/φ dimensions)
        let mut data = Vec::with_capacity(n * d);
        for i in 0..n {
            let t = i as f64 / (n as f64 - 1.0);
            // Converge from random-ish start toward (N, SIGMA, J2)
            let x = N + (SIGMA_MINUS_PHI - N) * (1.0 - t) + (1.0 - t) * 2.0 * ((i as f64) * 0.7).sin();
            let y = SIGMA + (SOPFR - SIGMA) * (1.0 - t) + (1.0 - t) * 1.5 * ((i as f64) * 1.3).cos();
            let z = J2 + (TAU - J2) * (1.0 - t) + (1.0 - t) * 3.0 * ((i as f64) * 0.5).sin();
            data.push(x);
            data.push(y);
            data.push(z);
        }
        datasets.push(CalibrationDataset {
            name: "n6_attractor_convergence".to_string(),
            data,
            n,
            d,
            known_patterns: vec![
                KnownPattern {
                    metric_name: "attractor".to_string(),
                    expected_values: vec![N, SIGMA, J2],
                    tolerance: 0.15,
                },
                KnownPattern {
                    metric_name: "convergence_detected".to_string(),
                    expected_values: vec![1.0], // boolean flag
                    tolerance: 0.5,
                },
            ],
        });
    }

    datasets
}

// ═══════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;

    /// A trivial test lens that always reports a "period" metric equal to σ=12.
    struct FixedPeriodLens;

    impl Lens for FixedPeriodLens {
        fn name(&self) -> &str {
            "FixedPeriodLens"
        }
        fn category(&self) -> &str {
            "T0"
        }
        fn scan(&self, _data: &[f64], _n: usize, _d: usize, _shared: &SharedData) -> LensResult {
            let mut result = LensResult::new();
            result.insert("period".to_string(), vec![SIGMA]); // always says period=12
            result.insert("slope".to_string(), vec![SIGMA_MINUS_TAU]); // always says slope=8
            result
        }
    }

    /// A null lens that produces nothing.
    struct NullLens;

    impl Lens for NullLens {
        fn name(&self) -> &str {
            "NullLens"
        }
        fn category(&self) -> &str {
            "T2"
        }
        fn scan(&self, _data: &[f64], _n: usize, _d: usize, _shared: &SharedData) -> LensResult {
            LensResult::new()
        }
    }

    /// A noisy lens that produces metrics with wrong values.
    struct NoisyLens;

    impl Lens for NoisyLens {
        fn name(&self) -> &str {
            "NoisyLens"
        }
        fn category(&self) -> &str {
            "T2"
        }
        fn scan(&self, _data: &[f64], _n: usize, _d: usize, _shared: &SharedData) -> LensResult {
            let mut result = LensResult::new();
            result.insert("period".to_string(), vec![999.0]); // wildly wrong
            result.insert("bogus_metric".to_string(), vec![42.0]); // false positive
            result
        }
    }

    #[test]
    fn test_generate_synthetic_datasets() {
        let datasets = generate_synthetic_datasets();
        assert_eq!(datasets.len(), 5, "should generate 5 datasets");

        // Check dataset sizes match n=6 constants
        assert_eq!(datasets[0].n, J2 as usize, "periodic dataset: J₂=24 points");
        assert_eq!(datasets[0].d, PHI as usize, "periodic dataset: φ=2 dims");
        assert_eq!(datasets[2].n, SIGMA as usize, "linear dataset: σ=12 points");
        assert_eq!(datasets[3].d, TAU as usize, "noise dataset: τ=4 dims");

        // Verify data lengths
        for ds in &datasets {
            assert_eq!(ds.data.len(), ds.n * ds.d, "data length mismatch for {}", ds.name);
        }
    }

    #[test]
    fn test_calibrate_fixed_lens() {
        let datasets = generate_synthetic_datasets();
        let lens = FixedPeriodLens;
        let result = calibrate_lens(&lens, &datasets);

        assert_eq!(result.lens_name, "FixedPeriodLens");
        // The fixed lens reports period=12 and slope=8, which should match
        // datasets 0 (period=12) and 2 (slope=8)
        assert!(result.hit_rate > 0.0, "should detect at least some patterns");
        assert!(result.latency_ms >= 0.0, "latency must be non-negative");
        assert!(result.n6_alignment > 0.0, "n6 alignment should be positive for n=6 values");
    }

    #[test]
    fn test_calibrate_null_lens() {
        let datasets = generate_synthetic_datasets();
        let lens = NullLens;
        let result = calibrate_lens(&lens, &datasets);

        assert_eq!(result.lens_name, "NullLens");
        assert_eq!(result.hit_rate, 0.0, "null lens should hit nothing");
        assert_eq!(result.false_negative_rate, 1.0, "null lens misses everything");
        assert_eq!(result.false_positive_rate, 0.0, "null lens produces nothing");
    }

    #[test]
    fn test_calibrate_noisy_lens() {
        let datasets = generate_synthetic_datasets();
        let lens = NoisyLens;
        let result = calibrate_lens(&lens, &datasets);

        assert_eq!(result.lens_name, "NoisyLens");
        // Noisy lens: period=999 won't match expected 12, and produces bogus_metric
        assert!(result.hit_rate < 0.5, "noisy lens should have low hit rate");
        assert!(result.false_positive_rate > 0.0, "noisy lens should have false positives");
    }

    #[test]
    fn test_calibrate_all_and_tiers() {
        let datasets = generate_synthetic_datasets();
        let lenses: Vec<Box<dyn Lens>> = vec![
            Box::new(FixedPeriodLens),
            Box::new(NullLens),
            Box::new(NoisyLens),
        ];

        let report = calibrate_all(&lenses, &datasets);

        assert_eq!(report.total_lenses, 3);
        assert!(report.calibrated_lenses <= 3);
        assert!(report.mean_hit_rate >= 0.0 && report.mean_hit_rate <= 1.0);

        // Check tier distribution covers all lenses
        let total_tiered: usize = report.tier_distribution.values().sum();
        assert_eq!(total_tiered, 3, "all lenses should be tiered");

        // Verify tier assignments individually
        for r in &report.results {
            let tier = tier_assignment(r);
            assert!(
                tier == "T0" || tier == "T1" || tier == "T2",
                "tier must be T0/T1/T2"
            );
        }
    }

    #[test]
    fn test_update_hit_rates() {
        let datasets = generate_synthetic_datasets();
        let lenses: Vec<Box<dyn Lens>> = vec![
            Box::new(FixedPeriodLens),
            Box::new(NullLens),
        ];

        let report = calibrate_all(&lenses, &datasets);
        let hit_rates = update_hit_rates(&report);

        assert_eq!(hit_rates.len(), 2);
        assert!(hit_rates.contains_key("FixedPeriodLens"));
        assert!(hit_rates.contains_key("NullLens"));
        assert_eq!(*hit_rates.get("NullLens").unwrap(), 0.0);
    }

    #[test]
    fn test_pattern_matching_tolerance() {
        let mut result = LensResult::new();
        result.insert("metric_a".to_string(), vec![12.5]);

        // Within 10% of 12.0 -> should match
        let pattern_match = KnownPattern {
            metric_name: "metric_a".to_string(),
            expected_values: vec![12.0],
            tolerance: 0.1,
        };
        assert!(pattern_matches(&result, &pattern_match));

        // Within 1% of 12.0 -> 12.5 is ~4% off, should NOT match
        let pattern_tight = KnownPattern {
            metric_name: "metric_a".to_string(),
            expected_values: vec![12.0],
            tolerance: 0.01,
        };
        assert!(!pattern_matches(&result, &pattern_tight));
    }

    #[test]
    fn test_n6_alignment_perfect() {
        let mut result = LensResult::new();
        // All values are exact n=6 constants
        result.insert("a".to_string(), vec![N, SIGMA, J2]);
        result.insert("b".to_string(), vec![PHI, TAU, SOPFR]);
        let alignment = compute_n6_alignment(&result);
        assert!(
            (alignment - 1.0).abs() < 1e-10,
            "perfect n=6 values should give alignment=1.0, got {}",
            alignment
        );
    }

    #[test]
    fn test_tier_assignment_boundaries() {
        let make = |hr: f64| CalibrationResult {
            lens_name: "test".to_string(),
            hit_rate: hr,
            false_positive_rate: 0.0,
            false_negative_rate: 0.0,
            latency_ms: 0.0,
            n6_alignment: 0.0,
        };
        assert_eq!(tier_assignment(&make(1.0)), "T0");
        assert_eq!(tier_assignment(&make(0.8)), "T0");
        assert_eq!(tier_assignment(&make(0.79)), "T1");
        assert_eq!(tier_assignment(&make(0.5)), "T1");
        assert_eq!(tier_assignment(&make(0.49)), "T2");
        assert_eq!(tier_assignment(&make(0.0)), "T2");
    }
}
