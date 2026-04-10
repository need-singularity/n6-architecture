use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// OmegaStateSpaceLens: Maps data into 24D Leech-lattice-inspired state space.
///
/// The Omega state space is the J₂(6)=24 dimensional optimal packing representation.
/// This lens projects data into the Leech lattice framework and measures:
///   1. Packing efficiency: how well data fills the 24D space
///   2. Kissing number proximity: neighbor count vs K₂₄=196560
///   3. Shell structure: distance quantization into discrete shells
///   4. Theta series alignment: density distribution vs Leech theta series
///
/// Key n=6 constants:
///   - J₂(6) = 24 = dimension of Leech lattice
///   - σ² = 144 = minimal norm squared in Leech lattice
///   - K₂₄ = 196560 = σ·(σ-μ)·J₂·(σ·(σ-μ)-φ) kissing number
pub struct OmegaStateSpaceLens;

impl Lens for OmegaStateSpaceLens {
    fn name(&self) -> &str {
        "OmegaStateSpaceLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d < 2 {
            return HashMap::new();
        }

        // Project data into effective J₂=24 dimensional space
        let effective_dim = d.min(24);

        // 1. Packing efficiency: average nearest-neighbor distance uniformity
        let mut nn_dists: Vec<f64> = Vec::with_capacity(n);
        for i in 0..n {
            let mut min_d = f64::MAX;
            for j in 0..n {
                if i == j { continue; }
                let dist = shared.dist(i, j);
                if dist < min_d { min_d = dist; }
            }
            nn_dists.push(min_d);
        }
        let mean_nn = nn_dists.iter().sum::<f64>() / n as f64;
        let var_nn = nn_dists.iter().map(|x| (x - mean_nn).powi(2)).sum::<f64>() / n as f64;
        // Perfect packing = low variance in NN distances
        let packing_efficiency = 1.0 / (1.0 + var_nn / mean_nn.powi(2).max(1e-12));

        // 2. Kissing number proxy: average number of neighbors within 1.5× NN distance
        let threshold = mean_nn * 1.5;
        let mut total_neighbors = 0u64;
        for i in 0..n {
            for j in 0..n {
                if i == j { continue; }
                if shared.dist(i, j) < threshold {
                    total_neighbors += 1;
                }
            }
        }
        let avg_kissing = total_neighbors as f64 / n as f64;
        // Normalized by J₂=24 (coordination number in many lattices)
        let kissing_ratio = avg_kissing / 24.0;

        // 3. Shell structure: quantize distances and measure discreteness
        let pair_count = n * (n - 1) / 2;
        let mut all_dists: Vec<f64> = Vec::with_capacity(pair_count);
        for i in 0..n {
            for j in (i + 1)..n {
                all_dists.push(shared.dist(i, j));
            }
        }
        all_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Bin into σ=12 shells
        let shells = 12usize;
        let max_d = all_dists.last().copied().unwrap_or(1.0);
        let min_d = all_dists.first().copied().unwrap_or(0.0);
        let range = (max_d - min_d).max(1e-12);
        let mut shell_counts = vec![0u32; shells];
        for &dist in &all_dists {
            let idx = ((dist - min_d) / range * (shells - 1) as f64).round() as usize;
            shell_counts[idx.min(shells - 1)] += 1;
        }
        // Shell discreteness: entropy (lower = more discrete = more lattice-like)
        let total = all_dists.len() as f64;
        let mut shell_entropy = 0.0_f64;
        for &c in &shell_counts {
            if c > 0 {
                let p = c as f64 / total;
                shell_entropy -= p * p.ln();
            }
        }
        let max_entropy = (shells as f64).ln();
        let shell_discreteness = 1.0 - shell_entropy / max_entropy;

        // 4. Dimension utilization: effective rank via singular values proxy
        // Use variance in each dimension as SVD proxy
        let mut dim_vars: Vec<f64> = Vec::with_capacity(effective_dim);
        for di in 0..effective_dim {
            let mean: f64 = (0..n).map(|r| data[r * d + di]).sum::<f64>() / n as f64;
            let var: f64 = (0..n).map(|r| (data[r * d + di] - mean).powi(2)).sum::<f64>() / n as f64;
            dim_vars.push(var);
        }
        let total_var: f64 = dim_vars.iter().sum::<f64>();
        let effective_rank = if total_var > 1e-12 {
            // Shannon entropy of normalized variances
            let mut ent = 0.0_f64;
            for &v in &dim_vars {
                if v > 1e-15 {
                    let p = v / total_var;
                    ent -= p * p.ln();
                }
            }
            ent.exp()
        } else {
            1.0
        };
        // Omega utilization: effective_rank / J₂
        let omega_utilization = (effective_rank / 24.0).min(1.0);

        // 5. Theta series proxy: distance distribution should show peaks at lattice radii
        // Count peaks in shell distribution
        let mut peaks = 0u32;
        for i in 1..shells.saturating_sub(1) {
            if shell_counts[i] > shell_counts[i - 1] && shell_counts[i] > shell_counts[i + 1] {
                peaks += 1;
            }
        }
        let theta_alignment = peaks as f64 / (shells as f64 / 2.0); // expect ~6 peaks

        // Composite Omega score
        let omega_score = packing_efficiency * 0.3
            + shell_discreteness * 0.25
            + omega_utilization * 0.25
            + theta_alignment.min(1.0) * 0.2;

        let mut result = HashMap::new();
        result.insert("omega_score".into(), vec![omega_score]);
        result.insert("packing_efficiency".into(), vec![packing_efficiency]);
        result.insert("kissing_ratio".into(), vec![kissing_ratio]);
        result.insert("avg_kissing_number".into(), vec![avg_kissing]);
        result.insert("shell_discreteness".into(), vec![shell_discreteness]);
        result.insert("shell_entropy".into(), vec![shell_entropy]);
        result.insert("omega_utilization".into(), vec![omega_utilization]);
        result.insert("effective_rank".into(), vec![effective_rank]);
        result.insert("theta_alignment".into(), vec![theta_alignment]);
        result.insert("shell_peaks".into(), vec![peaks as f64]);
        result
    }
}
