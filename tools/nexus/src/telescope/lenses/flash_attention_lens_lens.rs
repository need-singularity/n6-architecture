use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// FlashAttentionLens: Detect memory-efficient attention patterns amenable to flash computation.
///
/// Algorithm:
///   1. Build a soft attention matrix A[i][j] = exp(-dist(i,j)^2 / bandwidth) for all pairs
///   2. Row-normalize to get stochastic attention weights
///   3. Measure block-sparsity: partition points into blocks of size ~sqrt(n),
///      compute fraction of attention mass within each block (tiling efficiency)
///   4. Compute causal mask compatibility: how much attention mass lies in the
///      lower-triangular half (amenable to causal flash attention)
///   5. Measure attention entropy per row: low entropy = sharp attention = flash-friendly
///   6. Output: tiling_efficiency, causal_ratio, mean_attention_entropy, flash_score
pub struct UflashUattentionUlensLens;

impl Lens for UflashUattentionUlensLens {
    fn name(&self) -> &str {
        "FlashAttentionLens"
    }

    fn category(&self) -> &str {
        "AI"
    }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 3 {
            return HashMap::new();
        }

        // Step 1: Compute bandwidth as median squared distance
        let pair_count = n * (n - 1) / 2;
        let mut all_dists_sq: Vec<f64> = Vec::with_capacity(pair_count);
        for i in 0..n {
            for j in (i + 1)..n {
                let d = shared.dist(i, j);
                all_dists_sq.push(d * d);
            }
        }
        all_dists_sq.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let bandwidth = all_dists_sq[pair_count / 2].max(1e-12);

        // Step 2: Build row-normalized soft attention matrix
        // A[i][j] = exp(-dist(i,j)^2 / bandwidth), A[i][i] = 1.0
        let mut attn: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
        for i in 0..n {
            attn[i][i] = 1.0; // self-attention
            for j in (i + 1)..n {
                let d = shared.dist(i, j);
                let w = (-d * d / bandwidth).exp();
                attn[i][j] = w;
                attn[j][i] = w;
            }
            // Row-normalize
            let row_sum: f64 = attn[i].iter().sum();
            if row_sum > 1e-12 {
                for j in 0..n {
                    attn[i][j] /= row_sum;
                }
            }
        }

        // Step 3: Block-sparsity / tiling efficiency
        // Partition into blocks of size ~sqrt(n)
        let block_size = (n as f64).sqrt().ceil() as usize;
        let num_blocks = (n + block_size - 1) / block_size;
        let mut intra_block_mass = 0.0;
        let mut total_mass = 0.0;

        for i in 0..n {
            let block_i = i / block_size;
            for j in 0..n {
                let block_j = j / block_size;
                let w = attn[i][j];
                total_mass += w;
                // A tile is a (block_row, block_col) pair; "intra" = same block diagonal tile
                if block_i == block_j {
                    intra_block_mass += w;
                }
            }
        }
        // Also count near-diagonal block tiles (|block_i - block_j| <= 1)
        let mut near_diag_mass = 0.0;
        for i in 0..n {
            let block_i = i / block_size;
            for j in 0..n {
                let block_j = j / block_size;
                if (block_i as isize - block_j as isize).unsigned_abs() <= 1 {
                    near_diag_mass += attn[i][j];
                }
            }
        }
        let tiling_efficiency = if total_mass > 1e-12 {
            near_diag_mass / total_mass
        } else {
            0.0
        };

        // Step 4: Causal mask compatibility
        // Fraction of attention in lower-triangular half (i >= j)
        let mut causal_mass = 0.0;
        for i in 0..n {
            for j in 0..=i {
                causal_mass += attn[i][j];
            }
        }
        let causal_ratio = if total_mass > 1e-12 {
            causal_mass / total_mass
        } else {
            0.0
        };

        // Step 5: Mean attention entropy per row
        let mut entropy_sum = 0.0;
        for i in 0..n {
            let mut h = 0.0;
            for j in 0..n {
                let p = attn[i][j];
                if p > 1e-15 {
                    h -= p * p.ln();
                }
            }
            entropy_sum += h;
        }
        let mean_entropy = entropy_sum / n as f64;
        // Normalize entropy: max possible = ln(n)
        let max_entropy = (n as f64).ln();
        let normalized_entropy = if max_entropy > 1e-12 {
            mean_entropy / max_entropy
        } else {
            0.0
        };

        // Step 6: Flash score
        // High tiling efficiency + low entropy + moderate causal ratio = flash-friendly
        // flash_score in [0, 1]: higher = more amenable to flash attention
        let sparsity_score = 1.0 - normalized_entropy; // sharp attention = good
        let flash_score = 0.4 * tiling_efficiency + 0.3 * sparsity_score + 0.3 * causal_ratio;

        // Block sparsity ratio: how concentrated is attention in diagonal blocks
        let block_sparsity = if total_mass > 1e-12 {
            intra_block_mass / total_mass
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert("flash_score".to_string(), vec![flash_score]);
        result.insert(
            "tiling_efficiency".to_string(),
            vec![tiling_efficiency],
        );
        result.insert("causal_ratio".to_string(), vec![causal_ratio]);
        result.insert(
            "attention_entropy".to_string(),
            vec![mean_entropy, normalized_entropy],
        );
        result.insert("block_sparsity".to_string(), vec![block_sparsity]);
        result.insert(
            "block_config".to_string(),
            vec![block_size as f64, num_blocks as f64],
        );
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    fn make_shared(data: &[f64], n: usize, d: usize) -> SharedData {
        SharedData::compute(data, n, d)
    }

    #[test]
    fn test_clustered_data_high_tiling() {
        // Two tight clusters far apart -> high block sparsity, sharp attention
        let mut data = Vec::new();
        // Cluster 1: near origin
        for i in 0..5 {
            data.push(i as f64 * 0.1);
            data.push(i as f64 * 0.1);
        }
        // Cluster 2: far away
        for i in 0..5 {
            data.push(100.0 + i as f64 * 0.1);
            data.push(100.0 + i as f64 * 0.1);
        }
        let n = 10;
        let d = 2;
        let shared = make_shared(&data, n, d);
        let lens = UflashUattentionUlensLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "result must not be empty");
        assert!(result.contains_key("flash_score"));
        assert!(result.contains_key("tiling_efficiency"));
        assert!(result.contains_key("causal_ratio"));
        assert!(result.contains_key("attention_entropy"));
        assert!(result.contains_key("block_sparsity"));

        let flash_score = result["flash_score"][0];
        assert!(flash_score > 0.0 && flash_score <= 1.0, "flash_score={flash_score}");

        // Clustered data should have high block sparsity
        let block_sparsity = result["block_sparsity"][0];
        assert!(block_sparsity > 0.3, "block_sparsity={block_sparsity} should be high for clustered data");
    }

    #[test]
    fn test_uniform_spread_data() {
        // Evenly spread points -> more uniform attention, lower sparsity
        let mut data = Vec::new();
        for i in 0..8 {
            data.push(i as f64 * 10.0);
            data.push((i as f64 * 7.0) % 30.0);
            data.push(i as f64 * 3.0);
        }
        let n = 8;
        let d = 3;
        let shared = make_shared(&data, n, d);
        let lens = UflashUattentionUlensLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "result must not be empty");
        let keys = ["flash_score", "tiling_efficiency", "causal_ratio", "attention_entropy", "block_config"];
        for key in &keys {
            assert!(result.contains_key(*key), "missing key: {key}");
        }

        // Entropy should be meaningful (not zero, not max)
        let normalized_entropy = result["attention_entropy"][1];
        assert!(normalized_entropy > 0.0, "normalized_entropy={normalized_entropy} should be > 0");

        // Causal ratio should be roughly 0.5+ (lower triangle has >= half the mass)
        let causal_ratio = result["causal_ratio"][0];
        assert!(causal_ratio >= 0.4, "causal_ratio={causal_ratio} should be >= 0.4");
    }
}
