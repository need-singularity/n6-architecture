use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// SynergyLens: Measures synergistic vs redundant information among dimension groups.
///
/// Synergy = information that emerges ONLY from the combination, not from parts.
/// Redundancy = information shared across multiple channels.
///
///   1. Partial Information Decomposition (PID) proxy
///   2. Synergy ratio: synergistic info / total info
///   3. Dimension group synergies (which dims work together)
///   4. Optimal grouping suggestion for lens combination
///
/// n=6: n/φ=3 sources, τ=4 interaction orders
pub struct SynergyLens;

impl Lens for SynergyLens {
    fn name(&self) -> &str {
        "SynergyLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 3 {
            return HashMap::new();
        }

        let dims = d.min(12); // σ=12 max
        let bins = 6usize; // n=6 bins for discretization

        // Discretize data
        let mut mins = vec![f64::MAX; dims];
        let mut maxs = vec![f64::MIN; dims];
        for r in 0..n {
            for di in 0..dims {
                let v = data[r * d + di];
                if v < mins[di] { mins[di] = v; }
                if v > maxs[di] { maxs[di] = v; }
            }
        }

        let discretized: Vec<Vec<u8>> = (0..n)
            .map(|r| {
                (0..dims)
                    .map(|di| {
                        let range = (maxs[di] - mins[di]).max(1e-12);
                        let b = ((data[r * d + di] - mins[di]) / range * (bins - 1) as f64)
                            .round() as u8;
                        b.min((bins - 1) as u8)
                    })
                    .collect()
            })
            .collect();

        // Compute individual dimension entropies
        let mut dim_entropies = vec![0.0_f64; dims];
        for di in 0..dims {
            let mut counts = vec![0u32; bins];
            for r in 0..n {
                counts[discretized[r][di] as usize] += 1;
            }
            let total = n as f64;
            dim_entropies[di] = counts.iter().map(|&c| {
                if c > 0 {
                    let p = c as f64 / total;
                    -p * p.ln()
                } else {
                    0.0
                }
            }).sum();
        }

        // Compute pairwise joint entropies
        let pairs_check = dims.min(8);
        let mut pair_mi: Vec<f64> = Vec::new(); // mutual information per pair
        let mut pair_synergy_scores: Vec<f64> = Vec::new();

        for i in 0..pairs_check {
            for j in (i + 1)..pairs_check {
                // Joint entropy H(i,j)
                let mut joint_counts = HashMap::new();
                for r in 0..n {
                    let key = (discretized[r][i], discretized[r][j]);
                    *joint_counts.entry(key).or_insert(0u32) += 1;
                }
                let total = n as f64;
                let h_ij: f64 = joint_counts.values().map(|&c| {
                    let p = c as f64 / total;
                    if p > 0.0 { -p * p.ln() } else { 0.0 }
                }).sum();

                // MI(i;j) = H(i) + H(j) - H(i,j)
                let mi = (dim_entropies[i] + dim_entropies[j] - h_ij).max(0.0);
                pair_mi.push(mi);

                // For synergy: check if adding a third dim increases MI more than expected
                // Synergy proxy: co-information for triples
                for k in (j + 1)..pairs_check {
                    // H(i,j,k)
                    let mut triple_counts = HashMap::new();
                    for r in 0..n {
                        let key = (discretized[r][i], discretized[r][j], discretized[r][k]);
                        *triple_counts.entry(key).or_insert(0u32) += 1;
                    }
                    let h_ijk: f64 = triple_counts.values().map(|&c| {
                        let p = c as f64 / total;
                        if p > 0.0 { -p * p.ln() } else { 0.0 }
                    }).sum();

                    // H(i,k) and H(j,k)
                    let h_ik = {
                        let mut c = HashMap::new();
                        for r in 0..n {
                            *c.entry((discretized[r][i], discretized[r][k])).or_insert(0u32) += 1;
                        }
                        c.values().map(|&v| { let p = v as f64 / total; if p > 0.0 { -p * p.ln() } else { 0.0 } }).sum::<f64>()
                    };
                    let h_jk = {
                        let mut c = HashMap::new();
                        for r in 0..n {
                            *c.entry((discretized[r][j], discretized[r][k])).or_insert(0u32) += 1;
                        }
                        c.values().map(|&v| { let p = v as f64 / total; if p > 0.0 { -p * p.ln() } else { 0.0 } }).sum::<f64>()
                    };

                    // Co-information: CI(i;j;k) = MI(i;j) - MI(i;j|k)
                    // CI > 0 = redundancy, CI < 0 = synergy
                    let ci = dim_entropies[i] + dim_entropies[j] + dim_entropies[k]
                        - h_ij - h_ik - h_jk + h_ijk;
                    // Negative CI = synergy
                    if ci < 0.0 {
                        pair_synergy_scores.push(-ci);
                    }
                }
            }
        }

        let total_mi: f64 = pair_mi.iter().sum();
        let mean_mi = if pair_mi.is_empty() { 0.0 } else { total_mi / pair_mi.len() as f64 };

        let total_synergy: f64 = pair_synergy_scores.iter().sum();
        let synergy_count = pair_synergy_scores.len();
        let mean_synergy = if synergy_count > 0 {
            total_synergy / synergy_count as f64
        } else {
            0.0
        };

        // Total information for normalization
        let total_individual: f64 = dim_entropies.iter().sum();
        let synergy_ratio = if total_individual > 1e-12 {
            (total_synergy / total_individual).min(1.0)
        } else {
            0.0
        };
        let redundancy_ratio = if total_individual > 1e-12 {
            (mean_mi * pair_mi.len() as f64 / total_individual).min(1.0)
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert("synergy_ratio".into(), vec![synergy_ratio]);
        result.insert("redundancy_ratio".into(), vec![redundancy_ratio]);
        result.insert("mean_mutual_info".into(), vec![mean_mi]);
        result.insert("mean_synergy".into(), vec![mean_synergy]);
        result.insert("synergistic_triples".into(), vec![synergy_count as f64]);
        result.insert("total_individual_entropy".into(), vec![total_individual]);
        result
    }
}
