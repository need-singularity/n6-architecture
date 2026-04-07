use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// BrainMapLens: 뇌지도/의식지도 — 데이터를 뇌 영역 유사 네트워크로 매핑.
///
/// 뇌 네트워크 구조를 참조하여 데이터의 기능적 연결성을 분석:
///   1. 기능 영역 분할: 차원들을 n=6 기능 영역으로 클러스터링
///   2. 장거리 연결: 비인접 영역 간 강한 상관 (Global Workspace 지표)
///   3. 소세계성(Small-world): 높은 클러스터링 + 짧은 경로 길이
///   4. 허브 구조: 연결 중심성 분포 (scale-free 여부)
///   5. 좌우 대칭: φ=2 반구 구조
///
/// n=6: n=6 기능 영역, σ=12 연결 매트릭스, J₂=24 최대 노드
pub struct BrainMapLens;

impl Lens for BrainMapLens {
    fn name(&self) -> &str {
        "BrainMapLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 6 {
            return HashMap::new();
        }

        let dims = d.min(24);

        // Correlation matrix between dimensions (= functional connectivity)
        let mut means = vec![0.0_f64; dims];
        let mut stds = vec![0.0_f64; dims];
        for di in 0..dims {
            let m: f64 = (0..n).map(|r| data[r * d + di]).sum::<f64>() / n as f64;
            means[di] = m;
            let v: f64 = (0..n).map(|r| (data[r * d + di] - m).powi(2)).sum::<f64>() / n as f64;
            stds[di] = v.sqrt().max(1e-12);
        }

        let mut corr = vec![vec![0.0_f64; dims]; dims];
        for i in 0..dims {
            corr[i][i] = 1.0;
            for j in (i + 1)..dims {
                let mut cov = 0.0_f64;
                for r in 0..n {
                    cov += (data[r * d + i] - means[i]) * (data[r * d + j] - means[j]);
                }
                cov /= n as f64;
                let c = cov / (stds[i] * stds[j]);
                corr[i][j] = c;
                corr[j][i] = c;
            }
        }

        // Adjacency: connected if |corr| > 0.3
        let adj_threshold = 0.3;
        let mut adj = vec![vec![false; dims]; dims];
        let mut degrees = vec![0u32; dims];
        for i in 0..dims {
            for j in (i + 1)..dims {
                if corr[i][j].abs() > adj_threshold {
                    adj[i][j] = true;
                    adj[j][i] = true;
                    degrees[i] += 1;
                    degrees[j] += 1;
                }
            }
        }

        // 1. Functional regions: greedy clustering
        let mut assigned = vec![false; dims];
        let mut regions: Vec<Vec<usize>> = Vec::new();
        for seed in 0..dims {
            if assigned[seed] { continue; }
            let mut region = vec![seed];
            assigned[seed] = true;
            for cand in (seed + 1)..dims {
                if assigned[cand] { continue; }
                if adj[seed][cand] {
                    region.push(cand);
                    assigned[cand] = true;
                }
            }
            regions.push(region);
        }
        let num_regions = regions.len();

        // 2. Long-range connections (non-adjacent regions connected)
        let mut long_range = 0u32;
        for ri in 0..regions.len() {
            for rj in (ri + 2)..regions.len() {
                // Skip adjacent regions
                for &ni in &regions[ri] {
                    for &nj in &regions[rj] {
                        if corr[ni][nj].abs() > 0.5 {
                            long_range += 1;
                        }
                    }
                }
            }
        }

        // 3. Small-world metrics
        // Clustering coefficient
        let mut total_cc = 0.0_f64;
        let mut cc_count = 0u32;
        for i in 0..dims {
            let neighbors: Vec<usize> = (0..dims).filter(|&j| j != i && adj[i][j]).collect();
            let k = neighbors.len();
            if k < 2 { continue; }
            let mut triangles = 0u32;
            for ni in 0..neighbors.len() {
                for nj in (ni + 1)..neighbors.len() {
                    if adj[neighbors[ni]][neighbors[nj]] {
                        triangles += 1;
                    }
                }
            }
            let possible = (k * (k - 1) / 2) as f64;
            total_cc += triangles as f64 / possible;
            cc_count += 1;
        }
        let clustering_coeff = if cc_count > 0 { total_cc / cc_count as f64 } else { 0.0 };

        // Average path length (BFS, approximate)
        let mut total_path = 0.0_f64;
        let mut path_count = 0u32;
        for start in 0..dims.min(12) {
            let mut visited = vec![false; dims];
            let mut queue = std::collections::VecDeque::new();
            visited[start] = true;
            queue.push_back((start, 0u32));
            while let Some((node, dist)) = queue.pop_front() {
                if dist > 0 {
                    total_path += dist as f64;
                    path_count += 1;
                }
                for j in 0..dims {
                    if !visited[j] && adj[node][j] {
                        visited[j] = true;
                        queue.push_back((j, dist + 1));
                    }
                }
            }
        }
        let avg_path = if path_count > 0 { total_path / path_count as f64 } else { dims as f64 };

        // Small-world index: high CC / short path
        let random_cc = degrees.iter().map(|&k| k as f64).sum::<f64>() / (dims * dims) as f64;
        let small_world = if avg_path > 1e-12 && random_cc > 1e-12 {
            (clustering_coeff / random_cc.max(1e-6)) / avg_path
        } else {
            0.0
        };

        // 4. Hub detection: degree centrality
        let max_degree = *degrees.iter().max().unwrap_or(&0) as f64;
        let mean_degree = degrees.iter().sum::<u32>() as f64 / dims.max(1) as f64;
        let hub_ratio = if mean_degree > 1e-12 {
            max_degree / mean_degree
        } else {
            0.0
        };

        // 5. Bilateral symmetry (φ=2)
        let half = dims / 2;
        let mut symmetry = 0.0_f64;
        for i in 0..half {
            let j = dims - 1 - i;
            symmetry += corr[i][j].abs();
        }
        let bilateral_symmetry = if half > 0 { symmetry / half as f64 } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("num_regions".into(), vec![num_regions as f64]);
        result.insert("long_range_connections".into(), vec![long_range as f64]);
        result.insert("clustering_coefficient".into(), vec![clustering_coeff]);
        result.insert("avg_path_length".into(), vec![avg_path]);
        result.insert("small_world_index".into(), vec![small_world]);
        result.insert("hub_ratio".into(), vec![hub_ratio]);
        result.insert("bilateral_symmetry".into(), vec![bilateral_symmetry]);
        result.insert("mean_degree".into(), vec![mean_degree]);
        result
    }
}
