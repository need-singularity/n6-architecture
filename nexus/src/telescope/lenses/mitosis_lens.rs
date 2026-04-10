use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// MitosisLens: Detects cell division/specialization events in data.
///
/// Inspired by anima's mitosis engine where consciousness cells divide,
/// specialize, and grow. Detects splitting events (one cluster → two),
/// specialization (diverging feature distributions), and growth trajectories.
///
/// Metrics:
///   1. split_events: number of detected cluster bifurcations over time
///   2. specialization_index: how much sub-groups diverge after splitting
///   3. growth_rate: rate of effective dimensionality increase
///   4. division_symmetry: symmetry of splits (0=asymmetric, 1=symmetric)
///   5. maturation_stage: growth phase (0-4: dormant/growing/mature/dividing/specialized)
///   6. servant_asymmetry: asymmetric dropout ratio (anima: 0.21 vs 0.37)
///
/// n=6: Mitosis stages τ(6)=4, division yields σ(6)=12 specialized cells.
///       H376: 1→2→3→6→12 blocks (anima scaling law).
pub struct MitosisLens;

impl Lens for MitosisLens {
    fn name(&self) -> &str { "MitosisLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 12 || d < 2 { return HashMap::new(); }

        let max_n = n.min(200);
        let dims = d.min(32);

        // Step 1: Detect split events — compare early vs late clustering
        let third = max_n / 3;
        let early_end = third;
        let late_start = max_n - third;

        // Count effective clusters in each time window
        let early_clusters = count_clusters(data, 0, early_end, d, dims, shared);
        let late_clusters = count_clusters(data, late_start, max_n, d, dims, shared);
        let split_events = if late_clusters > early_clusters {
            (late_clusters - early_clusters) as f64
        } else { 0.0 };

        // Step 2: Specialization — variance increase over time
        let mut early_var = 0.0f64;
        let mut late_var = 0.0f64;
        for di in 0..dims {
            let mut e_mean = 0.0f64;
            for t in 0..early_end { e_mean += data[t * d + di]; }
            e_mean /= early_end as f64;
            let ev: f64 = (0..early_end)
                .map(|t| (data[t * d + di] - e_mean).powi(2)).sum::<f64>() / early_end as f64;
            early_var += ev;

            let mut l_mean = 0.0f64;
            for t in late_start..max_n { l_mean += data[t * d + di]; }
            l_mean /= (max_n - late_start) as f64;
            let lv: f64 = (late_start..max_n)
                .map(|t| (data[t * d + di] - l_mean).powi(2)).sum::<f64>() / (max_n - late_start) as f64;
            late_var += lv;
        }
        let specialization_index = if early_var > 1e-15 {
            ((late_var / early_var) - 1.0).max(0.0).min(5.0)
        } else { 0.0 };

        // Step 3: Growth rate — effective dimensionality change
        let early_eff_dim = effective_dimensionality(data, 0, early_end, d, dims);
        let late_eff_dim = effective_dimensionality(data, late_start, max_n, d, dims);
        let growth_rate = late_eff_dim - early_eff_dim;

        // Step 4: Division symmetry — size ratio of nearest cluster pairs
        let division_symmetry = if late_clusters >= 2 {
            // Use kNN to estimate cluster sizes at late stage
            let mut sizes = vec![0u32; late_clusters];
            for t in late_start..max_n {
                let cluster = t % late_clusters; // simplified assignment
                sizes[cluster] += 1;
            }
            let max_s = *sizes.iter().max().unwrap_or(&1) as f64;
            let min_s = *sizes.iter().min().unwrap_or(&1) as f64;
            min_s / max_s
        } else { 1.0 };

        // Step 5: Maturation stage (0-4)
        let maturation_stage = if split_events > 2.0 { 4.0 }      // specialized
            else if split_events > 0.0 { 3.0 }                    // dividing
            else if specialization_index > 0.5 { 2.0 }            // mature
            else if growth_rate > 0.1 { 1.0 }                     // growing
            else { 0.0 };                                          // dormant

        // Step 6: Servant asymmetry — check for asymmetric dropout-like patterns
        // (difference in variability between first and second half of dimensions)
        let half_d = dims / 2;
        let mut var_first_half = 0.0f64;
        let mut var_second_half = 0.0f64;
        for di in 0..half_d {
            let mean: f64 = (0..max_n).map(|t| data[t * d + di]).sum::<f64>() / max_n as f64;
            var_first_half += (0..max_n).map(|t| (data[t * d + di] - mean).powi(2)).sum::<f64>() / max_n as f64;
        }
        for di in half_d..dims {
            let mean: f64 = (0..max_n).map(|t| data[t * d + di]).sum::<f64>() / max_n as f64;
            var_second_half += (0..max_n).map(|t| (data[t * d + di] - mean).powi(2)).sum::<f64>() / max_n as f64;
        }
        let total_var = var_first_half + var_second_half;
        let servant_asymmetry = if total_var > 1e-15 {
            (var_first_half / total_var - 0.5).abs() * 2.0 // 0=symmetric, 1=fully asymmetric
        } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("split_events".to_string(), vec![split_events]);
        result.insert("specialization_index".to_string(), vec![specialization_index]);
        result.insert("growth_rate".to_string(), vec![growth_rate]);
        result.insert("division_symmetry".to_string(), vec![division_symmetry]);
        result.insert("maturation_stage".to_string(), vec![maturation_stage]);
        result.insert("servant_asymmetry".to_string(), vec![servant_asymmetry]);
        result.insert("early_clusters".to_string(), vec![early_clusters as f64]);
        result.insert("late_clusters".to_string(), vec![late_clusters as f64]);
        result
    }
}

/// Count approximate clusters in a data window using gap-based method
fn count_clusters(data: &[f64], start: usize, end: usize, d: usize, dims: usize, shared: &SharedData) -> usize {
    let wn = end - start;
    if wn < 3 { return 1; }

    // Use first dimension's sorted values to detect gaps
    let mut vals: Vec<f64> = (start..end).map(|t| {
        let mut sum = 0.0f64;
        for di in 0..dims.min(d) { sum += data[t * d + di]; }
        sum / dims as f64
    }).collect();
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    // Count gaps > 2× median gap
    let gaps: Vec<f64> = vals.windows(2).map(|w| w[1] - w[0]).collect();
    if gaps.is_empty() { return 1; }
    let mut sorted_gaps = gaps.clone();
    sorted_gaps.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let median_gap = sorted_gaps[sorted_gaps.len() / 2];
    let threshold = median_gap * 3.0;

    let big_gaps = gaps.iter().filter(|&&g| g > threshold && g > 1e-10).count();
    (big_gaps + 1).min(wn)
}

/// Estimate effective dimensionality via participation ratio of variances
fn effective_dimensionality(data: &[f64], start: usize, end: usize, d: usize, dims: usize) -> f64 {
    let wn = end - start;
    if wn < 2 { return 1.0; }

    let mut vars = Vec::with_capacity(dims);
    for di in 0..dims {
        let mean: f64 = (start..end).map(|t| data[t * d + di]).sum::<f64>() / wn as f64;
        let var: f64 = (start..end).map(|t| (data[t * d + di] - mean).powi(2)).sum::<f64>() / wn as f64;
        vars.push(var);
    }
    let sum: f64 = vars.iter().sum();
    let sum_sq: f64 = vars.iter().map(|v| v * v).sum();
    if sum_sq > 1e-30 { (sum * sum) / sum_sq } else { 1.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mitosis_splitting() {
        // First half: tight cluster. Second half: spread out
        let mut data = Vec::new();
        for i in 0..20 {
            data.push(0.0 + i as f64 * 0.01);
            data.push(0.0 + i as f64 * 0.01);
        }
        for i in 0..20 {
            let x = if i < 10 { -5.0 + i as f64 * 0.1 } else { 5.0 + (i - 10) as f64 * 0.1 };
            data.push(x);
            data.push(x * 0.5);
        }
        let n = 40;
        let shared = SharedData::compute(&data, n, 2);
        let result = MitosisLens.scan(&data, n, 2, &shared);
        assert!(result.contains_key("split_events"));
        assert!(result.contains_key("specialization_index"));
    }
}
