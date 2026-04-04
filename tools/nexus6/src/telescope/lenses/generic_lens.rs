use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{
    self, SharedData, shannon_entropy, cosine_sim, norm, mean_var, col_min_max, min_max,
};

/// Strategy for computing metrics, selected based on lens category/domain.
#[derive(Debug, Clone, Copy)]
enum MetricStrategy {
    /// topology/network: connectivity, clustering, degree
    Topology,
    /// quantum/physics: entropy, coherence, energy
    Quantum,
    /// biology/evolution: diversity, fitness, adaptation
    Biology,
    /// math/geometry: curvature, symmetry, dimension
    Math,
    /// consciousness/phi: integration, differentiation, complexity
    Consciousness,
    /// signal/wave: frequency, amplitude, phase
    Signal,
    /// engineering/material: stress, efficiency, yield
    Engineering,
    /// information/entropy: mutual info, redundancy, compression
    Information,
    /// default: basic stats
    Stats,
}

/// GenericLens — auto-generated lens for registry entries without dedicated implementations.
/// Maps lens name/category keywords to a metric computation strategy.
/// This allows all registry entries to produce scan results.
pub struct GenericLens {
    name: String,
    strategy: MetricStrategy,
}

impl GenericLens {
    /// Create a GenericLens from registry entry name and domain affinities.
    pub fn new(name: &str, domains: &[String], description: &str) -> Self {
        let strategy = Self::pick_strategy(name, domains, description);
        Self {
            name: name.to_string(),
            strategy,
        }
    }

    fn pick_strategy(name: &str, domains: &[String], description: &str) -> MetricStrategy {
        let haystack = format!(
            "{} {} {}",
            name.to_lowercase(),
            domains.join(" ").to_lowercase(),
            description.to_lowercase()
        );

        // Order matters: more specific first
        if haystack.contains("conscious")
            || haystack.contains("phi")
            || haystack.contains("qualia")
            || haystack.contains("binding")
            || haystack.contains("awareness")
        {
            MetricStrategy::Consciousness
        } else if haystack.contains("topolog")
            || haystack.contains("network")
            || haystack.contains("graph")
            || haystack.contains("connect")
            || haystack.contains("cluster")
            || haystack.contains("percolat")
        {
            MetricStrategy::Topology
        } else if haystack.contains("quantum")
            || haystack.contains("qubit")
            || haystack.contains("superposit")
            || haystack.contains("entangle")
            || haystack.contains("tunneling")
            || haystack.contains("boson")
            || haystack.contains("fermion")
            || haystack.contains("fock")
            || haystack.contains("feynman")
            || haystack.contains("renormaliz")
            || haystack.contains("vacuum")
            || haystack.contains("anomaly_qft")
            || haystack.contains("path_integral")
        {
            MetricStrategy::Quantum
        } else if haystack.contains("biolog")
            || haystack.contains("evolut")
            || haystack.contains("genetic")
            || haystack.contains("mutation")
            || haystack.contains("fitness")
            || haystack.contains("species")
            || haystack.contains("molecul")
            || haystack.contains("protein")
            || haystack.contains("drug")
            || haystack.contains("brain")
            || haystack.contains("neuron")
            || haystack.contains("neuro")
        {
            MetricStrategy::Biology
        } else if haystack.contains("signal")
            || haystack.contains("wave")
            || haystack.contains("frequenc")
            || haystack.contains("spectrum")
            || haystack.contains("acoustic")
            || haystack.contains("oscillat")
            || haystack.contains("resonan")
            || haystack.contains("fourier")
            || haystack.contains("filter")
        {
            MetricStrategy::Signal
        } else if haystack.contains("math")
            || haystack.contains("geometr")
            || haystack.contains("algebra")
            || haystack.contains("number_theor")
            || haystack.contains("prime")
            || haystack.contains("symmetr")
            || haystack.contains("curvatur")
            || haystack.contains("manifold")
            || haystack.contains("tensor")
            || haystack.contains("cohomolog")
        {
            MetricStrategy::Math
        } else if haystack.contains("engineer")
            || haystack.contains("material")
            || haystack.contains("chip")
            || haystack.contains("circuit")
            || haystack.contains("battery")
            || haystack.contains("solar")
            || haystack.contains("semiconduc")
            || haystack.contains("thermal")
            || haystack.contains("aerodynam")
        {
            MetricStrategy::Engineering
        } else if haystack.contains("inform")
            || haystack.contains("entropy")
            || haystack.contains("compress")
            || haystack.contains("coding")
            || haystack.contains("mutual")
            || haystack.contains("redundan")
        {
            MetricStrategy::Information
        } else if haystack.contains("physic")
            || haystack.contains("energy")
            || haystack.contains("force")
            || haystack.contains("field")
            || haystack.contains("particle")
            || haystack.contains("relativi")
            || haystack.contains("spacetime")
            || haystack.contains("gravit")
            || haystack.contains("thermo")
        {
            MetricStrategy::Quantum // physics shares quantum strategy
        } else {
            MetricStrategy::Stats
        }
    }
}

impl Lens for GenericLens {
    fn name(&self) -> &str {
        &self.name
    }

    fn category(&self) -> &str {
        "T2" // generic lenses run at tier 2 (after dedicated lenses)
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 2 || d == 0 {
            return HashMap::new();
        }
        match self.strategy {
            MetricStrategy::Topology => scan_topology(data, n, d, shared),
            MetricStrategy::Quantum => scan_quantum(data, n, d, shared),
            MetricStrategy::Biology => scan_biology(data, n, d, shared),
            MetricStrategy::Math => scan_math(data, n, d, shared),
            MetricStrategy::Consciousness => scan_consciousness(data, n, d, shared),
            MetricStrategy::Signal => scan_signal(data, n, d, shared),
            MetricStrategy::Engineering => scan_engineering(data, n, d, shared),
            MetricStrategy::Information => scan_information(data, n, d, shared),
            MetricStrategy::Stats => scan_stats(data, n, d),
        }
    }
}

// ---------------------------------------------------------------------------
// Strategy implementations — each produces 2-4 named metrics
// ---------------------------------------------------------------------------

/// Topology: connectivity, avg_degree, clustering_coeff
fn scan_topology(data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
    let mut result = HashMap::new();

    // Connectivity: fraction of point pairs within median distance
    let n_pairs = n * (n - 1) / 2;
    if n_pairs == 0 {
        return result;
    }
    let dists: Vec<f64> = (0..n_pairs).map(|idx| shared.distance_matrix[idx]).collect();
    let mut sorted_d = dists.clone();
    sorted_d.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let median = sorted_d[sorted_d.len() / 2];
    let connected = dists.iter().filter(|&&d| d <= median).count();
    let connectivity = connected as f64 / n_pairs as f64;
    result.insert("connectivity".into(), vec![connectivity]);

    // Average degree at median threshold
    let avg_degree = (2.0 * connected as f64) / n as f64;
    result.insert("avg_degree".into(), vec![avg_degree]);

    // Clustering coefficient (approximate via k-NN)
    let k = shared.knn_k;
    if k >= 2 {
        let mut total_cc = 0.0;
        for i in 0..n {
            let neighbors = shared.knn(i);
            let mut triangles = 0u32;
            let mut possible = 0u32;
            let nn: Vec<usize> = neighbors.iter().map(|&x| x as usize).collect();
            for a in 0..nn.len() {
                for b in (a + 1)..nn.len() {
                    possible += 1;
                    let na = nn[a];
                    let nb = nn[b];
                    let dist_ab = if na != nb { shared.dist(na, nb) } else { 0.0 };
                    if dist_ab <= median {
                        triangles += 1;
                    }
                }
            }
            if possible > 0 {
                total_cc += triangles as f64 / possible as f64;
            }
        }
        result.insert("clustering_coeff".into(), vec![total_cc / n as f64]);
    }

    result
}

/// Quantum: entropy, coherence, energy spread
fn scan_quantum(data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
    let mut result = HashMap::new();

    // Entropy across all values
    let entropy = shannon_entropy(data, 32.min(n * d));
    result.insert("entropy".into(), vec![entropy]);

    // Coherence: average absolute cosine similarity between consecutive rows
    if n >= 2 {
        let mut coh_sum = 0.0;
        let pairs = (n - 1).min(64);
        for i in 0..pairs {
            let a = &data[i * d..(i + 1) * d];
            let b = &data[(i + 1) * d..(i + 2) * d];
            coh_sum += cosine_sim(a, b).abs();
        }
        result.insert("coherence".into(), vec![coh_sum / pairs as f64]);
    }

    // Energy spread: variance of row norms
    let norms: Vec<f64> = (0..n)
        .map(|i| norm(&data[i * d..(i + 1) * d]))
        .collect();
    let mean_norm = norms.iter().sum::<f64>() / n as f64;
    let var_norm = norms.iter().map(|x| (x - mean_norm).powi(2)).sum::<f64>() / n as f64;
    result.insert("energy_spread".into(), vec![var_norm.sqrt()]);

    result
}

/// Biology: diversity, fitness proxy, adaptation rate
fn scan_biology(data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
    let mut result = HashMap::new();

    // Diversity: average pairwise distance
    let n_pairs = n * (n - 1) / 2;
    if n_pairs > 0 {
        let avg_dist =
            shared.distance_matrix.iter().take(n_pairs).sum::<f64>() / n_pairs as f64;
        result.insert("diversity".into(), vec![avg_dist]);
    }

    // Fitness proxy: row norms (higher = "fitter")
    let norms: Vec<f64> = (0..n)
        .map(|i| norm(&data[i * d..(i + 1) * d]))
        .collect();
    let max_norm = norms.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let mean_norm = norms.iter().sum::<f64>() / n as f64;
    result.insert("fitness_max".into(), vec![max_norm]);
    result.insert("fitness_mean".into(), vec![mean_norm]);

    // Adaptation: entropy of the norm distribution
    let ent = shannon_entropy(&norms, 16.min(n));
    result.insert("adaptation_entropy".into(), vec![ent]);

    result
}

/// Math: curvature proxy, dimension estimate, symmetry score
fn scan_math(data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
    let mut result = HashMap::new();

    // Curvature proxy: variance of k-NN distances
    if shared.knn_k > 0 && n >= 2 {
        let knn_dists: Vec<f64> = (0..n)
            .map(|i| {
                let neighbors = shared.knn(i);
                let last = neighbors[shared.knn_k - 1] as usize;
                if last != i { shared.dist(i, last) } else { 0.0 }
            })
            .collect();
        let mean_d = knn_dists.iter().sum::<f64>() / n as f64;
        let var_d = knn_dists.iter().map(|x| (x - mean_d).powi(2)).sum::<f64>() / n as f64;
        result.insert("curvature_proxy".into(), vec![var_d]);
    }

    // Effective dimension via participation ratio of singular values (approximate)
    // Use variance explained by each feature
    let (means, vars) = mean_var(data, n, d);
    let total_var: f64 = vars.iter().sum();
    if total_var > 1e-12 {
        let pr: f64 = total_var.powi(2) / vars.iter().map(|v| v.powi(2)).sum::<f64>();
        result.insert("effective_dim".into(), vec![pr]);
    }

    // Symmetry score: how close mean is to midpoint of range per feature
    let mut sym_scores = Vec::with_capacity(d);
    for j in 0..d {
        let (lo, hi) = col_min_max(data, n, d, j);
        let mid = (lo + hi) / 2.0;
        let range = (hi - lo).max(1e-12);
        let sym = 1.0 - ((means[j] - mid) / range).abs();
        sym_scores.push(sym.max(0.0));
    }
    let avg_sym = sym_scores.iter().sum::<f64>() / d as f64;
    result.insert("symmetry_score".into(), vec![avg_sym]);

    result
}

/// Consciousness: integration, differentiation, complexity
fn scan_consciousness(data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
    let mut result = HashMap::new();

    // Integration: average MI across dimension pairs
    if d >= 2 {
        let mut mi_sum = 0.0;
        let mut count = 0;
        let max_d = d.min(32);
        for di in 0..max_d {
            for dj in (di + 1)..max_d {
                mi_sum += shared.mi(di, dj);
                count += 1;
            }
        }
        if count > 0 {
            result.insert("integration".into(), vec![mi_sum / count as f64]);
        }
    }

    // Differentiation: entropy of point-wise variance
    let (_, vars) = mean_var(data, n, d);
    let ent = shannon_entropy(&vars, 16.min(d));
    result.insert("differentiation".into(), vec![ent]);

    // Complexity: product of integration and differentiation (Tononi-style)
    if let (Some(integ), Some(diff)) = (
        result.get("integration").and_then(|v| v.first()),
        result.get("differentiation").and_then(|v| v.first()),
    ) {
        result.insert("complexity".into(), vec![integ * diff]);
    }

    result
}

/// Signal: dominant frequency proxy, amplitude, phase spread
fn scan_signal(data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
    let mut result = HashMap::new();

    // Amplitude: RMS of all values
    let rms = (data.iter().map(|x| x * x).sum::<f64>() / data.len() as f64).sqrt();
    result.insert("amplitude_rms".into(), vec![rms]);

    // Zero-crossing rate per row (frequency proxy)
    if d >= 2 {
        let mut total_zcr = 0.0;
        let rows = n.min(64);
        for i in 0..rows {
            let row = &data[i * d..(i + 1) * d];
            let zc: usize = row.windows(2).filter(|w| w[0].signum() != w[1].signum()).count();
            total_zcr += zc as f64 / (d - 1) as f64;
        }
        result.insert("zero_crossing_rate".into(), vec![total_zcr / rows as f64]);
    }

    // Autocorrelation at lag 1 (average across rows)
    if d >= 3 {
        let mut total_ac = 0.0;
        let rows = n.min(64);
        for i in 0..rows {
            let row = &data[i * d..(i + 1) * d];
            let mean = row.iter().sum::<f64>() / d as f64;
            let var: f64 = row.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / d as f64;
            if var > 1e-12 {
                let cov: f64 = row.windows(2).map(|w| (w[0] - mean) * (w[1] - mean)).sum::<f64>()
                    / (d - 1) as f64;
                total_ac += cov / var;
            }
        }
        result.insert("autocorr_lag1".into(), vec![total_ac / rows as f64]);
    }

    result
}

/// Engineering: efficiency proxy, stress concentration, utilization
fn scan_engineering(data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
    let mut result = HashMap::new();

    // Utilization: fraction of values above mean
    let global_mean = data.iter().sum::<f64>() / data.len() as f64;
    let above = data.iter().filter(|&&x| x > global_mean).count();
    result.insert("utilization".into(), vec![above as f64 / data.len() as f64]);

    // Stress concentration: max/mean ratio of row norms
    let norms: Vec<f64> = (0..n)
        .map(|i| norm(&data[i * d..(i + 1) * d]))
        .collect();
    let mean_norm = norms.iter().sum::<f64>() / n as f64;
    let max_norm = norms.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    if mean_norm > 1e-12 {
        result.insert("stress_concentration".into(), vec![max_norm / mean_norm]);
    }

    // Efficiency: 1 - coefficient of variation (lower spread = more efficient)
    let var_norm = norms.iter().map(|x| (x - mean_norm).powi(2)).sum::<f64>() / n as f64;
    let cv = if mean_norm > 1e-12 { var_norm.sqrt() / mean_norm } else { 0.0 };
    result.insert("efficiency_proxy".into(), vec![(1.0 - cv).max(0.0)]);

    result
}

/// Information: mutual info summary, redundancy, compression ratio
fn scan_information(data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
    let mut result = HashMap::new();

    // Average MI across feature pairs
    if d >= 2 {
        let max_d = d.min(32);
        let mut mi_sum = 0.0;
        let mut count = 0;
        for di in 0..max_d {
            for dj in (di + 1)..max_d {
                mi_sum += shared.mi(di, dj);
                count += 1;
            }
        }
        if count > 0 {
            let avg_mi = mi_sum / count as f64;
            result.insert("avg_mutual_info".into(), vec![avg_mi]);

            // Redundancy: avg_mi / max possible entropy
            let max_ent = (n as f64).ln();
            if max_ent > 1e-12 {
                result.insert("redundancy".into(), vec![avg_mi / max_ent]);
            }
        }
    }

    // Compression ratio: ratio of unique values to total
    let total = data.len();
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let unique = 1 + sorted.windows(2).filter(|w| (w[1] - w[0]).abs() > 1e-10).count();
    result.insert(
        "compression_ratio".into(),
        vec![unique as f64 / total as f64],
    );

    result
}

/// Stats: mean, std, range, skewness, kurtosis
fn scan_stats(data: &[f64], n: usize, d: usize) -> LensResult {
    let mut result = HashMap::new();
    let total = (n * d) as f64;
    if total < 1.0 {
        return result;
    }

    let mean = data.iter().sum::<f64>() / total;
    let var = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / total;
    let std = var.sqrt();

    result.insert("mean".into(), vec![mean]);
    result.insert("std".into(), vec![std]);

    let (lo, hi) = min_max(data);
    result.insert("range".into(), vec![hi - lo]);

    // Skewness
    if std > 1e-12 {
        let skew = data.iter().map(|x| ((x - mean) / std).powi(3)).sum::<f64>() / total;
        result.insert("skewness".into(), vec![skew]);

        // Kurtosis (excess)
        let kurt =
            data.iter().map(|x| ((x - mean) / std).powi(4)).sum::<f64>() / total - 3.0;
        result.insert("kurtosis".into(), vec![kurt]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    fn test_data() -> (Vec<f64>, usize, usize) {
        let n = 8;
        let d = 4;
        let data: Vec<f64> = (0..n * d).map(|i| (i as f64 * 0.3).sin()).collect();
        (data, n, d)
    }

    #[test]
    fn test_generic_lens_stats() {
        let (data, n, d) = test_data();
        let shared = SharedData::compute(&data, n, d);
        let lens = GenericLens::new("test_stats", &[], "some random lens");
        let result = lens.scan(&data, n, d, &shared);
        assert!(result.contains_key("mean"));
        assert!(result.contains_key("std"));
        assert!(result.contains_key("range"));
    }

    #[test]
    fn test_generic_lens_topology() {
        let (data, n, d) = test_data();
        let shared = SharedData::compute(&data, n, d);
        let lens = GenericLens::new(
            "test_topo",
            &["network".to_string()],
            "connectivity analysis",
        );
        let result = lens.scan(&data, n, d, &shared);
        assert!(result.contains_key("connectivity"));
        assert!(result.contains_key("avg_degree"));
    }

    #[test]
    fn test_generic_lens_quantum() {
        let (data, n, d) = test_data();
        let shared = SharedData::compute(&data, n, d);
        let lens = GenericLens::new(
            "wave_function",
            &["quantum".to_string()],
            "quantum state",
        );
        let result = lens.scan(&data, n, d, &shared);
        assert!(result.contains_key("entropy"));
        assert!(result.contains_key("coherence"));
    }

    #[test]
    fn test_generic_lens_consciousness() {
        let (data, n, d) = test_data();
        let shared = SharedData::compute(&data, n, d);
        let lens = GenericLens::new(
            "qualia_test",
            &["consciousness".to_string()],
            "awareness",
        );
        let result = lens.scan(&data, n, d, &shared);
        assert!(result.contains_key("integration"));
        assert!(result.contains_key("differentiation"));
    }

    #[test]
    fn test_all_strategies_no_panic() {
        let (data, n, d) = test_data();
        let shared = SharedData::compute(&data, n, d);

        let cases = vec![
            ("topo", vec!["network".into()], "graph"),
            ("quant", vec!["quantum".into()], "qubit"),
            ("bio", vec!["biology".into()], "evolution"),
            ("math", vec!["mathematics".into()], "geometry"),
            ("consc", vec!["consciousness".into()], "phi"),
            ("sig", vec!["signal".into()], "wave"),
            ("eng", vec!["engineering".into()], "chip"),
            ("info", vec!["information".into()], "entropy"),
            ("other", vec![], "misc"),
        ];

        for (name, domains, desc) in cases {
            let lens = GenericLens::new(name, &domains, desc);
            let result = lens.scan(&data, n, d, &shared);
            assert!(
                !result.is_empty(),
                "Strategy for '{}' produced empty result",
                name
            );
        }
    }
}
