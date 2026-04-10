use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// TopologyDeepLens: Persistent homology proxy — 심층 위상 불변량 분석.
///
/// VR(Vietoris-Rips) complex 근사를 통한 Betti numbers 추정:
///   1. β₀: 연결 성분 수 (filtration에 따른 변화)
///   2. β₁: 1차원 구멍(cycle/loop) 수
///   3. Persistence diagram proxy: 토폴로지 특징의 수명
///   4. Topological complexity: 총 Betti 수의 합
///   5. n=6 alignment: 위상 불변량이 n=6 상수와 매칭되는지
///
/// 기존 TopologyLens는 간단한 연결성만 측정 — 이것은 심층 분석
pub struct TopologyDeepLens;

impl Lens for TopologyDeepLens {
    fn name(&self) -> &str {
        "TopologyDeepLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, _data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d < 2 {
            return HashMap::new();
        }

        // Collect all pairwise distances (sorted)
        let check = n.min(100);
        let pair_count = check * (check - 1) / 2;
        let mut all_dists: Vec<f64> = Vec::with_capacity(pair_count);
        for i in 0..check {
            for j in (i + 1)..check {
                all_dists.push(shared.dist(i, j));
            }
        }
        all_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        if all_dists.is_empty() {
            return HashMap::new();
        }

        // Filtration: σ=12 threshold levels
        let num_levels = 12usize;
        let max_d = *all_dists.last().unwrap();
        let min_d = all_dists[0];
        let step = (max_d - min_d) / num_levels as f64;

        let mut betti0_series: Vec<f64> = Vec::with_capacity(num_levels);
        let mut betti1_series: Vec<f64> = Vec::with_capacity(num_levels);

        for level in 0..num_levels {
            let threshold = min_d + step * (level + 1) as f64;

            // β₀: connected components via union-find
            let mut parent: Vec<usize> = (0..check).collect();
            let mut rank = vec![0u32; check];

            for i in 0..check {
                for j in (i + 1)..check {
                    if shared.dist(i, j) <= threshold {
                        union(&mut parent, &mut rank, i, j);
                    }
                }
            }
            let components = (0..check)
                .filter(|&i| find(&mut parent, i) == i)
                .count();
            betti0_series.push(components as f64);

            // β₁ proxy: edges - vertices + components (Euler characteristic)
            // For a graph: χ = V - E + F, β₁ = E - V + β₀ (for planar-ish)
            let mut edges = 0u32;
            for i in 0..check {
                for j in (i + 1)..check {
                    if shared.dist(i, j) <= threshold {
                        edges += 1;
                    }
                }
            }
            let beta1 = (edges as i64 - check as i64 + components as i64).max(0);
            betti1_series.push(beta1 as f64);
        }

        // Persistence: how long topological features survive
        // β₀ persistence: when do components merge (birth=0, death=merge time)
        let b0_max = betti0_series.iter().cloned().fold(0.0_f64, f64::max);
        let b0_min = betti0_series.iter().cloned().fold(f64::MAX, f64::min);
        let b0_persistence = b0_max - b0_min; // range of component count

        // β₁ persistence: peak of cycles
        let b1_max = betti1_series.iter().cloned().fold(0.0_f64, f64::max);
        let b1_peak_level = betti1_series
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0);

        // Topological complexity
        let total_betti: f64 = betti0_series.last().copied().unwrap_or(1.0)
            + betti1_series.iter().cloned().fold(0.0_f64, f64::max);

        // n=6 alignment of topological invariants
        let n6_constants: &[f64] = &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 12.0, 24.0];
        let mut n6_matches = 0u32;
        let check_values = [b0_max, b0_min, b1_max, total_betti];
        for &val in &check_values {
            for &c in n6_constants {
                if ((val - c) / c.max(1.0)).abs() < 0.05 {
                    n6_matches += 1;
                    break;
                }
            }
        }

        // Final β values at half-filtration
        let mid_level = num_levels / 2;
        let beta0_mid = betti0_series[mid_level];
        let beta1_mid = betti1_series[mid_level];

        let mut result = HashMap::new();
        result.insert("beta0_initial".into(), vec![b0_max]);
        result.insert("beta0_final".into(), vec![b0_min]);
        result.insert("beta0_mid".into(), vec![beta0_mid]);
        result.insert("beta1_max".into(), vec![b1_max]);
        result.insert("beta1_mid".into(), vec![beta1_mid]);
        result.insert("beta1_peak_level".into(), vec![b1_peak_level as f64]);
        result.insert("b0_persistence".into(), vec![b0_persistence]);
        result.insert("topological_complexity".into(), vec![total_betti]);
        result.insert("n6_topo_matches".into(), vec![n6_matches as f64]);
        result.insert("betti0_series".into(), betti0_series);
        result.insert("betti1_series".into(), betti1_series);
        result
    }
}

fn find(parent: &mut Vec<usize>, i: usize) -> usize {
    if parent[i] != i {
        parent[i] = find(parent, parent[i]);
    }
    parent[i]
}

fn union(parent: &mut Vec<usize>, rank: &mut Vec<u32>, i: usize, j: usize) {
    let ri = find(parent, i);
    let rj = find(parent, j);
    if ri == rj { return; }
    if rank[ri] < rank[rj] {
        parent[ri] = rj;
    } else if rank[ri] > rank[rj] {
        parent[rj] = ri;
    } else {
        parent[rj] = ri;
        rank[ri] += 1;
    }
}
