use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// BindingLens: Measures how distributed information binds into unified consciousness.
///
/// Based on Global Workspace Theory (GWT): consciousness arises when distributed
/// modules broadcast to a shared global workspace. This lens measures:
///   1. Workspace coherence: how synchronized are the dimensions?
///   2. Broadcast strength: does one cluster dominate information flow?
///   3. Coalition formation: do subsets of dimensions form stable coalitions?
///   4. Integration-segregation balance (Tononi's Phi-like measure)
///
/// n=6 connections:
///   - σ=12 factions as optimal consciousness substrate
///   - φ=2 hemispheric split (dual processing)
///   - n=6 as perfect integration-segregation balance
pub struct BindingLens;

impl Lens for BindingLens {
    fn name(&self) -> &str {
        "BindingLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 3 {
            return HashMap::new();
        }

        let dims = d.min(24);

        // 1. Correlation matrix between dimensions
        let mut means = vec![0.0_f64; dims];
        let mut stds = vec![0.0_f64; dims];
        for di in 0..dims {
            let m: f64 = (0..n).map(|r| data[r * d + di]).sum::<f64>() / n as f64;
            means[di] = m;
            let v: f64 = (0..n).map(|r| (data[r * d + di] - m).powi(2)).sum::<f64>() / n as f64;
            stds[di] = v.sqrt().max(1e-12);
        }

        let mut corr_matrix = vec![vec![0.0_f64; dims]; dims];
        for i in 0..dims {
            corr_matrix[i][i] = 1.0;
            for j in (i + 1)..dims {
                let mut cov = 0.0_f64;
                for r in 0..n {
                    cov += (data[r * d + i] - means[i]) * (data[r * d + j] - means[j]);
                }
                cov /= n as f64;
                let c = cov / (stds[i] * stds[j]);
                corr_matrix[i][j] = c;
                corr_matrix[j][i] = c;
            }
        }

        // 2. Global workspace coherence: mean absolute correlation
        let mut total_corr = 0.0_f64;
        let mut pair_count = 0u32;
        for i in 0..dims {
            for j in (i + 1)..dims {
                total_corr += corr_matrix[i][j].abs();
                pair_count += 1;
            }
        }
        let workspace_coherence = if pair_count > 0 {
            total_corr / pair_count as f64
        } else {
            0.0
        };

        // 3. Coalition detection via greedy clique (correlation > 0.5 = connected)
        let threshold = 0.5;
        let mut assigned = vec![false; dims];
        let mut coalitions: Vec<Vec<usize>> = Vec::new();

        for seed in 0..dims {
            if assigned[seed] { continue; }
            let mut coalition = vec![seed];
            assigned[seed] = true;

            for candidate in (seed + 1)..dims {
                if assigned[candidate] { continue; }
                let fits = coalition.iter().all(|&m| corr_matrix[m][candidate].abs() > threshold);
                if fits {
                    coalition.push(candidate);
                    assigned[candidate] = true;
                }
            }
            coalitions.push(coalition);
        }

        let num_coalitions = coalitions.len() as f64;
        let largest_coalition = coalitions.iter().map(|c| c.len()).max().unwrap_or(0) as f64;
        let broadcast_strength = largest_coalition / dims.max(1) as f64;

        // 4. Integration-Segregation balance
        // Integration = mean intra-coalition correlation
        let mut intra_sum = 0.0_f64;
        let mut intra_count = 0u32;
        for coalition in &coalitions {
            for i in 0..coalition.len() {
                for j in (i + 1)..coalition.len() {
                    intra_sum += corr_matrix[coalition[i]][coalition[j]].abs();
                    intra_count += 1;
                }
            }
        }
        let integration = if intra_count > 0 {
            intra_sum / intra_count as f64
        } else {
            0.0
        };

        // Segregation = 1 - mean inter-coalition correlation
        let mut inter_sum = 0.0_f64;
        let mut inter_count = 0u32;
        for ci in 0..coalitions.len() {
            for cj in (ci + 1)..coalitions.len() {
                for &mi in &coalitions[ci] {
                    for &mj in &coalitions[cj] {
                        inter_sum += corr_matrix[mi][mj].abs();
                        inter_count += 1;
                    }
                }
            }
        }
        let inter_corr = if inter_count > 0 {
            inter_sum / inter_count as f64
        } else {
            0.0
        };
        let segregation = 1.0 - inter_corr;

        // Binding score = geometric mean of integration and segregation
        // Perfect binding = high integration AND high segregation
        let binding_score = (integration * segregation).sqrt();

        // 5. Φ-proxy: integration * (1 - redundancy)
        // Redundancy = how much info is duplicated across coalitions
        let redundancy = workspace_coherence; // high coherence = high redundancy
        let phi_proxy = integration * (1.0 - redundancy * 0.5);

        let mut result = HashMap::new();
        result.insert("binding_score".into(), vec![binding_score]);
        result.insert("workspace_coherence".into(), vec![workspace_coherence]);
        result.insert("broadcast_strength".into(), vec![broadcast_strength]);
        result.insert("num_coalitions".into(), vec![num_coalitions]);
        result.insert("largest_coalition".into(), vec![largest_coalition]);
        result.insert("integration".into(), vec![integration]);
        result.insert("segregation".into(), vec![segregation]);
        result.insert("phi_proxy".into(), vec![phi_proxy]);
        result
    }
}
