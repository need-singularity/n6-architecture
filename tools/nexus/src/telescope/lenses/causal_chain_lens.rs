use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// CausalChainLens: reality_map 인과 사슬 전용 렌즈.
///
/// reality_map의 edges를 분석하기 위한 전용 렌즈:
///   1. 데이터의 time-lagged 인과 경로(chain) 탐색
///   2. 경로 강도(chain strength) 평가 — 전이적 인과 관계
///   3. 인과 루프(cycle) 탐지 — 피드백 구조 발견
///   4. n=6 상수와의 공명 패턴 확인
///
/// CausalLens가 쌍별(pairwise) 인과 강도를 측정하는 반면,
/// CausalChainLens는 다단계 경로와 루프를 추적한다.
///
/// 출력 메트릭:
///   - max_chain_length: 최장 인과 경로 길이
///   - chain_strength: 최장 경로의 누적 인과 강도
///   - loop_count: 탐지된 인과 루프 수
///   - loop_n6_resonance: 루프 길이의 n=6 상수 일치도
///   - mean_path_strength: 모든 인과 경로의 평균 강도
///   - hub_dim: 인과 그래프에서 허브(최다 연결) 차원
pub struct CausalChainLens;

/// n=6 상수 목록 — 루프 길이 공명 검사용
const N6_CONSTANTS: &[(usize, &str)] = &[
    (2, "phi"), (3, "n/phi"), (4, "tau"), (5, "sopfr"),
    (6, "n"), (8, "sigma-tau"), (10, "sigma-phi"), (12, "sigma"),
    (24, "J2"),
];

impl Lens for CausalChainLens {
    fn name(&self) -> &str {
        "CausalChainLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 8 || d < 2 {
            return HashMap::new();
        }

        let max_lag = (n / 4).max(1).min(8);
        let dims = d.min(12); // sigma=12 상한

        // 1단계: 쌍별 인과 행렬 구축 (방향 그래프)
        let mut causal_matrix = vec![vec![0.0_f64; dims]; dims];
        let threshold = 0.15;

        for di in 0..dims {
            for dj in 0..dims {
                if di == dj {
                    continue;
                }

                let x: Vec<f64> = (0..n).map(|r| data[r * d + di]).collect();
                let y: Vec<f64> = (0..n).map(|r| data[r * d + dj]).collect();

                let auto_corr = lagged_corr(&y, &y, 1, n).abs();

                let mut max_cross = 0.0_f64;
                for lag in 1..=max_lag {
                    let cross = lagged_corr(&x, &y, lag, n).abs();
                    if cross > max_cross {
                        max_cross = cross;
                    }
                }

                let strength = (max_cross - auto_corr).max(0.0);
                if strength > threshold {
                    causal_matrix[di][dj] = strength;
                }
            }
        }

        // 2단계: 인과 경로 탐색 (DFS, 최장 경로)
        let mut max_chain_length = 0_usize;
        let mut max_chain_strength = 0.0_f64;
        let mut all_path_strengths: Vec<f64> = Vec::new();

        for start in 0..dims {
            let mut visited = vec![false; dims];
            visited[start] = true;
            dfs_chain(
                &causal_matrix, dims, start, 0, 1.0,
                &mut visited,
                &mut max_chain_length, &mut max_chain_strength,
                &mut all_path_strengths,
            );
        }

        // 3단계: 인과 루프 탐지
        let mut loop_count = 0_usize;
        let mut loop_lengths: Vec<usize> = Vec::new();

        for start in 0..dims {
            detect_loops(
                &causal_matrix, dims, start, start, 0,
                &mut vec![false; dims],
                &mut loop_count, &mut loop_lengths,
            );
        }
        // 중복 제거 (방향 루프 A->B->A는 B->A->B와 동일)
        loop_count /= 2_usize.max(1);

        // 4단계: 루프 길이와 n=6 공명
        let mut n6_resonance = 0.0_f64;
        if !loop_lengths.is_empty() {
            let mut matches = 0;
            for &len in &loop_lengths {
                for &(c, _) in N6_CONSTANTS {
                    if len == c {
                        matches += 1;
                        break;
                    }
                }
            }
            n6_resonance = matches as f64 / loop_lengths.len() as f64;
        }

        // 5단계: 허브 차원 (최다 아웃+인 연결)
        let mut connectivity = vec![0_usize; dims];
        for di in 0..dims {
            for dj in 0..dims {
                if causal_matrix[di][dj] > 0.0 {
                    connectivity[di] += 1;
                    connectivity[dj] += 1;
                }
            }
        }
        let hub_dim = connectivity
            .iter()
            .enumerate()
            .max_by_key(|(_, &c)| c)
            .map(|(i, _)| i)
            .unwrap_or(0);

        let mean_path_strength = if all_path_strengths.is_empty() {
            0.0
        } else {
            all_path_strengths.iter().sum::<f64>() / all_path_strengths.len() as f64
        };

        let mut result = HashMap::new();
        result.insert("max_chain_length".to_string(), vec![max_chain_length as f64]);
        result.insert("chain_strength".to_string(), vec![max_chain_strength]);
        result.insert("loop_count".to_string(), vec![loop_count as f64]);
        result.insert("loop_n6_resonance".to_string(), vec![n6_resonance]);
        result.insert("mean_path_strength".to_string(), vec![mean_path_strength]);
        result.insert("hub_dim".to_string(), vec![hub_dim as f64]);
        result.insert(
            "causal_edge_count".to_string(),
            vec![causal_matrix.iter().flatten().filter(|&&v| v > 0.0).count() as f64],
        );
        result.insert(
            "loop_lengths".to_string(),
            loop_lengths.iter().map(|&l| l as f64).collect(),
        );
        result
    }
}

/// DFS 인과 경로 탐색 (백트래킹)
fn dfs_chain(
    matrix: &[Vec<f64>], dims: usize,
    cur: usize, depth: usize, cumul: f64,
    visited: &mut Vec<bool>,
    max_len: &mut usize, max_str: &mut f64,
    all_strengths: &mut Vec<f64>,
) {
    if depth > 0 {
        all_strengths.push(cumul);
        if depth > *max_len || (depth == *max_len && cumul > *max_str) {
            *max_len = depth;
            *max_str = cumul;
        }
    }
    if depth >= dims { return; }

    for next in 0..dims {
        if !visited[next] && matrix[cur][next] > 0.0 {
            visited[next] = true;
            dfs_chain(matrix, dims, next, depth + 1, cumul * matrix[cur][next],
                      visited, max_len, max_str, all_strengths);
            visited[next] = false;
        }
    }
}

/// 인과 루프(사이클) 탐지
fn detect_loops(
    matrix: &[Vec<f64>], dims: usize,
    start: usize, cur: usize, depth: usize,
    visited: &mut Vec<bool>,
    loop_count: &mut usize, loop_lengths: &mut Vec<usize>,
) {
    if depth > 0 && cur == start && depth >= 2 {
        *loop_count += 1;
        loop_lengths.push(depth);
        return;
    }
    if depth >= dims.min(8) { return; } // 루프 길이 상한

    for next in 0..dims {
        if matrix[cur][next] > 0.0 {
            if next == start && depth >= 1 {
                detect_loops(matrix, dims, start, next, depth + 1, visited, loop_count, loop_lengths);
            } else if !visited[next] {
                visited[next] = true;
                detect_loops(matrix, dims, start, next, depth + 1, visited, loop_count, loop_lengths);
                visited[next] = false;
            }
        }
    }
}

/// 시차 상관 계수 (lagged cross-correlation)
fn lagged_corr(x: &[f64], y: &[f64], lag: usize, n: usize) -> f64 {
    if lag >= n { return 0.0; }
    let eff = n - lag;
    if eff < 2 { return 0.0; }

    let x_s = &x[..eff];
    let y_s = &y[lag..lag + eff];

    let mx = x_s.iter().sum::<f64>() / eff as f64;
    let my = y_s.iter().sum::<f64>() / eff as f64;

    let mut cov = 0.0;
    let mut vx = 0.0;
    let mut vy = 0.0;

    for i in 0..eff {
        let dx = x_s[i] - mx;
        let dy = y_s[i] - my;
        cov += dx * dy;
        vx += dx * dx;
        vy += dy * dy;
    }

    let denom = (vx * vy).sqrt();
    if denom < 1e-15 { 0.0 } else { cov / denom }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_causal_chain_basic() {
        // 인과 체인 테스트: dim0 -> dim1 -> dim2 (시차 1)
        let n = 20;
        let d = 3;
        let mut data = vec![0.0; n * d];
        for t in 0..n {
            data[t * d + 0] = t as f64;
            data[t * d + 1] = if t > 0 { (t - 1) as f64 * 0.9 } else { 0.0 };
            data[t * d + 2] = if t > 1 { (t - 2) as f64 * 0.8 } else { 0.0 };
        }
        let shared = SharedData::compute(&data, n, d);
        let result = CausalChainLens.scan(&data, n, d, &shared);

        assert!(result.contains_key("max_chain_length"));
        assert!(result.contains_key("loop_count"));
        assert!(result.contains_key("causal_edge_count"));
        assert!(result["max_chain_length"][0] >= 1.0);
    }

    #[test]
    fn test_causal_chain_loop_detect() {
        let n = 30;
        let d = 4;
        let mut data = vec![0.0; n * d];
        for t in 0..n {
            let phase = t as f64 * 0.3;
            data[t * d + 0] = phase.sin();
            data[t * d + 1] = (phase + 0.5).sin();
            data[t * d + 2] = (phase + 1.0).sin();
            data[t * d + 3] = (phase + 1.5).sin();
        }
        let shared = SharedData::compute(&data, n, d);
        let result = CausalChainLens.scan(&data, n, d, &shared);

        assert!(result.contains_key("loop_count"));
        assert!(result.contains_key("loop_n6_resonance"));
    }
}
