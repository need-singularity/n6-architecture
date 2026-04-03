use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// InverseLens: Reverse-engineer causes from observed effects.
///
/// Algorithm:
///   1. Pseudo-inverse reconstruction: treat each dimension as a "response" variable
///      and compute how well the remaining dimensions predict it (R² analogue).
///      Dimensions with high reconstructability are "effects"; low = "causes".
///   2. Transfer entropy approximation: for each ordered pair (dim_i → dim_j),
///      estimate directional information flow via conditional histogram MI.
///      Net flow asymmetry reveals causal direction.
///   3. Causal depth: rank points by how many neighbors they "explain" — points
///      whose local neighborhood has low residual variance are causal hubs.
pub struct UinverseLens;

impl Lens for UinverseLens {
    fn name(&self) -> &str {
        "UinverseLens"
    }

    fn category(&self) -> &str {
        "causal"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 3 || d < 2 {
            return HashMap::new();
        }

        // --- 1. Reconstructability per dimension (R² analogue) ---
        // For each target dimension t, do a simple mean-regression from the
        // average of all other dimensions, then compute R² = 1 - SS_res / SS_tot.
        let mut means = vec![0.0f64; d];
        for i in 0..n {
            let row = i * d;
            for j in 0..d {
                means[j] += data[row + j];
            }
        }
        let inv_n = 1.0 / n as f64;
        for j in 0..d {
            means[j] *= inv_n;
        }

        let mut reconstructability = Vec::with_capacity(d);
        for t in 0..d {
            // For each point, compute predictor = mean of all dims except t
            let mut ss_res = 0.0;
            let mut ss_tot = 0.0;
            let mut pred_vals = Vec::with_capacity(n);
            let mut target_vals = Vec::with_capacity(n);

            for i in 0..n {
                let row = i * d;
                let target = data[row + t];
                let mut sum_others = 0.0;
                for j in 0..d {
                    if j != t {
                        sum_others += data[row + j];
                    }
                }
                let predictor = sum_others / (d - 1) as f64;
                pred_vals.push(predictor);
                target_vals.push(target);
            }

            // Linear regression: target = a * predictor + b
            let pred_mean: f64 = pred_vals.iter().sum::<f64>() * inv_n;
            let tgt_mean = means[t];

            let mut cov = 0.0;
            let mut var_pred = 0.0;
            for i in 0..n {
                let dp = pred_vals[i] - pred_mean;
                let dt = target_vals[i] - tgt_mean;
                cov += dp * dt;
                var_pred += dp * dp;
            }

            let a = if var_pred.abs() > 1e-15 { cov / var_pred } else { 0.0 };
            let b = tgt_mean - a * pred_mean;

            for i in 0..n {
                let predicted = a * pred_vals[i] + b;
                let residual = target_vals[i] - predicted;
                ss_res += residual * residual;
                let diff = target_vals[i] - tgt_mean;
                ss_tot += diff * diff;
            }

            let r2 = if ss_tot > 1e-15 { 1.0 - ss_res / ss_tot } else { 0.0 };
            reconstructability.push(r2.max(0.0));
        }

        // --- 2. Transfer entropy approximation (dimension-level causal flow) ---
        // For each pair (i→j), compute TE ≈ MI(j_future, i_past | j_past)
        // We approximate with lagged MI: treat point order as temporal order.
        // TE(i→j) ≈ MI(dim_i[0..n-1], dim_j[1..n]) - MI(dim_j[0..n-1], dim_j[1..n])
        let n_bins = 8usize;
        let max_dim_pairs = d.min(8); // limit computation for high-d

        let mut net_flow = vec![0.0f64; d]; // positive = net cause, negative = net effect
        let mut max_te = 0.0f64;

        for di in 0..max_dim_pairs {
            for dj in 0..max_dim_pairs {
                if di == dj { continue; }

                // Extract lagged columns
                let lag_n = n - 1;
                if lag_n < 4 { continue; }

                let mut col_i_past = Vec::with_capacity(lag_n);
                let mut col_j_past = Vec::with_capacity(lag_n);
                let mut col_j_future = Vec::with_capacity(lag_n);

                for k in 0..lag_n {
                    col_i_past.push(data[k * d + di]);
                    col_j_past.push(data[k * d + dj]);
                    col_j_future.push(data[(k + 1) * d + dj]);
                }

                let mi_ij = binned_mi(&col_i_past, &col_j_future, n_bins);
                let mi_jj = binned_mi(&col_j_past, &col_j_future, n_bins);

                // TE(i→j) ≈ mi_ij - mi_jj (simplified)
                let te = (mi_ij - mi_jj).max(0.0);
                net_flow[di] += te; // di is a cause
                net_flow[dj] -= te; // dj is an effect
                if te > max_te { max_te = te; }
            }
        }

        // Normalize net_flow to [-1, 1]
        let flow_max = net_flow.iter().map(|x| x.abs()).fold(0.0f64, f64::max);
        if flow_max > 1e-15 {
            for v in net_flow.iter_mut() {
                *v /= flow_max;
            }
        }

        // --- 3. Causal depth per point ---
        // A point is a "causal hub" if its local neighborhood has low residual
        // variance after subtracting the hub's influence (distance-weighted mean).
        let k = shared.knn_k.min(n - 1).max(1);
        let mut causal_depth = Vec::with_capacity(n);

        for i in 0..n {
            let row_i = &data[i * d..(i + 1) * d];

            // Gather neighbor distances and compute distance-weighted prediction
            let neighbors = shared.knn(i);
            let actual_k = neighbors.len().min(k);
            if actual_k == 0 {
                causal_depth.push(0.0);
                continue;
            }

            // Compute residual: how much variance in neighbors is NOT explained by point i
            let mut total_var = 0.0;
            let mut explained_var = 0.0;

            for dim in 0..d {
                let mut neigh_vals = Vec::with_capacity(actual_k);
                for idx in 0..actual_k {
                    let j = neighbors[idx] as usize;
                    neigh_vals.push(data[j * d + dim]);
                }

                let neigh_mean: f64 = neigh_vals.iter().sum::<f64>() / actual_k as f64;
                let var: f64 = neigh_vals.iter().map(|v| (v - neigh_mean).powi(2)).sum::<f64>()
                    / actual_k as f64;
                total_var += var;

                // Explained component: how close is hub value to neighbor mean
                let deviation = (row_i[dim] - neigh_mean).powi(2);
                explained_var += deviation;
            }

            // Causal depth = ratio of structure (low residual = hub explains neighbors)
            let depth = if total_var + explained_var > 1e-15 {
                total_var / (total_var + explained_var)
            } else {
                0.5
            };
            causal_depth.push(depth);
        }

        // Summary statistics
        let mean_depth = causal_depth.iter().sum::<f64>() / n as f64;
        let mean_recon = reconstructability.iter().sum::<f64>() / d as f64;

        // Identify cause vs effect dimensions
        let mut cause_dims: Vec<f64> = Vec::new();
        let mut effect_dims: Vec<f64> = Vec::new();
        for dim in 0..d.min(max_dim_pairs) {
            if net_flow[dim] > 0.1 {
                cause_dims.push(dim as f64);
            } else if net_flow[dim] < -0.1 {
                effect_dims.push(dim as f64);
            }
        }

        let mut result = HashMap::new();
        result.insert("reconstructability".to_string(), reconstructability);
        result.insert("net_causal_flow".to_string(), net_flow);
        result.insert("causal_depth".to_string(), causal_depth);
        result.insert("cause_dimensions".to_string(), cause_dims);
        result.insert("effect_dimensions".to_string(), effect_dims);
        result.insert("mean_causal_depth".to_string(), vec![mean_depth]);
        result.insert("mean_reconstructability".to_string(), vec![mean_recon]);
        result.insert("max_transfer_entropy".to_string(), vec![max_te]);
        result
    }
}

/// Binned mutual information between two f64 slices.
fn binned_mi(a: &[f64], b: &[f64], n_bins: usize) -> f64 {
    let n = a.len().min(b.len());
    if n < 2 || n_bins < 2 {
        return 0.0;
    }

    let (a_min, a_max) = a.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| {
        (lo.min(v), hi.max(v))
    });
    let (b_min, b_max) = b.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &v| {
        (lo.min(v), hi.max(v))
    });

    let a_range = (a_max - a_min).max(1e-12);
    let b_range = (b_max - b_min).max(1e-12);
    let a_scale = (n_bins - 1) as f64 / a_range;
    let b_scale = (n_bins - 1) as f64 / b_range;

    let mut joint = vec![0u32; n_bins * n_bins];
    for i in 0..n {
        let ai = ((a[i] - a_min) * a_scale) as usize;
        let bi = ((b[i] - b_min) * b_scale) as usize;
        let ai = ai.min(n_bins - 1);
        let bi = bi.min(n_bins - 1);
        joint[ai * n_bins + bi] += 1;
    }

    let mut pa = vec![0u32; n_bins];
    let mut pb = vec![0u32; n_bins];
    for ai in 0..n_bins {
        for bi in 0..n_bins {
            let c = joint[ai * n_bins + bi];
            pa[ai] += c;
            pb[bi] += c;
        }
    }

    let n_f = n as f64;
    let mut mi = 0.0;
    for ai in 0..n_bins {
        if pa[ai] == 0 { continue; }
        let p_a = pa[ai] as f64 / n_f;
        for bi in 0..n_bins {
            let c = joint[ai * n_bins + bi];
            if c == 0 || pb[bi] == 0 { continue; }
            let p_ab = c as f64 / n_f;
            let p_b = pb[bi] as f64 / n_f;
            mi += p_ab * (p_ab / (p_a * p_b)).ln();
        }
    }
    mi.max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    /// Test with a simple causal structure: dim0 = cause, dim1 = 2*dim0 + noise (effect).
    #[test]
    fn test_inverse_lens_causal_structure() {
        let n = 20;
        let d = 3;
        // dim0 = linear ramp (cause), dim1 = 2*dim0 (effect), dim2 = constant
        let mut data = Vec::with_capacity(n * d);
        for i in 0..n {
            let x = i as f64 * 0.5;
            data.push(x);           // dim0: cause
            data.push(2.0 * x);     // dim1: effect (perfectly determined)
            data.push(1.0);         // dim2: constant
        }

        let shared = SharedData::compute(&data, n, d);
        let lens = UinverseLens;
        let result = lens.scan(&data, n, d, &shared);

        // Must return non-empty results
        assert!(!result.is_empty(), "scan() must return non-empty HashMap");
        assert!(result.contains_key("reconstructability"));
        assert!(result.contains_key("causal_depth"));
        assert!(result.contains_key("net_causal_flow"));

        let recon = &result["reconstructability"];
        assert_eq!(recon.len(), d);
        // dim1 (2*dim0) should be highly reconstructable
        assert!(recon[1] > 0.5, "effect dimension should have high R², got {}", recon[1]);

        let depth = &result["causal_depth"];
        assert_eq!(depth.len(), n);
        // All depths should be in [0, 1]
        for &v in depth {
            assert!(v >= 0.0 && v <= 1.0, "causal depth out of range: {v}");
        }
    }

    /// Test with random-ish scattered points to ensure no panics and non-empty output.
    #[test]
    fn test_inverse_lens_scattered_data() {
        let n = 15;
        let d = 4;
        let mut data = Vec::with_capacity(n * d);
        for i in 0..n {
            for j in 0..d {
                // Deterministic pseudo-scatter using trig
                let val = ((i * 7 + j * 13) as f64 * 0.37).sin() * 10.0;
                data.push(val);
            }
        }

        let shared = SharedData::compute(&data, n, d);
        let lens = UinverseLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "scan() must return non-empty HashMap");
        assert!(result.contains_key("mean_causal_depth"));
        assert!(result.contains_key("mean_reconstructability"));
        assert!(result.contains_key("max_transfer_entropy"));

        let mean_depth = result["mean_causal_depth"][0];
        assert!(mean_depth > 0.0 && mean_depth <= 1.0, "mean depth should be in (0,1], got {mean_depth}");

        let mean_recon = result["mean_reconstructability"][0];
        assert!(mean_recon >= 0.0, "mean reconstructability should be >= 0, got {mean_recon}");

        // Verify flow vector has correct length
        let flow = &result["net_causal_flow"];
        assert_eq!(flow.len(), d);
    }
}
