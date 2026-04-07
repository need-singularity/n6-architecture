use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// CorpusLens: Corpus 품질/일관성/완전성 분석.
///
/// 학습 corpus나 데이터셋의 구조적 품질을 측정:
///   1. 커버리지: 특징 공간의 얼마나 넓은 영역을 커버하는가
///   2. 밸런스: 클래스/영역별 데이터 분포 균형
///   3. 일관성: 동일 영역 내 데이터 일관성 (노이즈 수준)
///   4. 완전성: 빈 영역(hole) 탐지
///   5. 다양성: 고유한 패턴의 풍부도
///
/// corpus 변경/추가 시 NEXUS-6 스캔 필수 — 이 렌즈가 자동 실행
pub struct CorpusLens;

impl Lens for CorpusLens {
    fn name(&self) -> &str {
        "CorpusLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 {
            return HashMap::new();
        }

        let dims = d.min(24);

        // 1. Coverage: volume of convex hull proxy (range product)
        let mut ranges = Vec::with_capacity(dims);
        for di in 0..dims {
            let mut min_v = f64::MAX;
            let mut max_v = f64::MIN;
            for r in 0..n {
                let v = data[r * d + di];
                if v.is_finite() {
                    if v < min_v { min_v = v; }
                    if v > max_v { max_v = v; }
                }
            }
            ranges.push(max_v - min_v);
        }
        // Log-volume (avoid overflow)
        let log_volume: f64 = ranges.iter().map(|&r| (r.max(1e-12)).ln()).sum();
        let coverage = log_volume / dims as f64; // normalized per dim

        // 2. Balance: bin each dimension and check uniformity
        let bins = 6usize; // n=6 bins
        let mut overall_balance = 0.0_f64;
        for di in 0..dims {
            let range = ranges[di].max(1e-12);
            let min_v = {
                let mut m = f64::MAX;
                for r in 0..n { let v = data[r * d + di]; if v.is_finite() && v < m { m = v; } }
                m
            };
            let mut bin_counts = vec![0u32; bins];
            for r in 0..n {
                let v = data[r * d + di];
                if !v.is_finite() { continue; }
                let idx = ((v - min_v) / range * (bins - 1) as f64).round() as usize;
                bin_counts[idx.min(bins - 1)] += 1;
            }
            // Chi-square uniformity test proxy
            let expected = n as f64 / bins as f64;
            let chi2: f64 = bin_counts.iter()
                .map(|&c| (c as f64 - expected).powi(2) / expected)
                .sum();
            // Normalize: chi2=0 → perfect balance, high → imbalanced
            let dim_balance = 1.0 / (1.0 + chi2 / bins as f64);
            overall_balance += dim_balance;
        }
        overall_balance /= dims as f64;

        // 3. Consistency: average intra-cluster variance (k=n/φ=3 clusters)
        let k = 3usize;
        // Simple k-means initialization: percentile splits on first dim
        let mut sorted_rows: Vec<(usize, f64)> = (0..n)
            .map(|r| (r, data[r * d]))
            .collect();
        sorted_rows.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        let chunk = n / k;
        let mut cluster_vars = Vec::new();
        for ci in 0..k {
            let start = ci * chunk;
            let end = if ci == k - 1 { n } else { (ci + 1) * chunk };
            let members: Vec<usize> = sorted_rows[start..end].iter().map(|&(r, _)| r).collect();
            if members.len() < 2 { continue; }

            let mut c_means = vec![0.0_f64; dims];
            for &r in &members {
                for di in 0..dims {
                    c_means[di] += data[r * d + di];
                }
            }
            for m in &mut c_means { *m /= members.len() as f64; }

            let mut var: f64 = 0.0;
            for &r in &members {
                for di in 0..dims {
                    var += (data[r * d + di] - c_means[di]).powi(2);
                }
            }
            var /= (members.len() * dims) as f64;
            cluster_vars.push(var);
        }
        let consistency = if cluster_vars.is_empty() {
            0.0
        } else {
            let mean_var = cluster_vars.iter().sum::<f64>() / cluster_vars.len() as f64;
            1.0 / (1.0 + mean_var)
        };

        // 4. Completeness: detect holes (empty bins in 2D projections)
        let mut hole_count = 0u32;
        let mut total_bins_2d = 0u32;
        let check_pairs = dims.min(6);
        for i in 0..check_pairs {
            for j in (i + 1)..check_pairs {
                let mut grid = vec![vec![false; bins]; bins];
                let ri = ranges[i].max(1e-12);
                let rj = ranges[j].max(1e-12);
                let mi = {
                    let mut m = f64::MAX;
                    for r in 0..n { let v = data[r * d + i]; if v.is_finite() && v < m { m = v; } }
                    m
                };
                let mj = {
                    let mut m = f64::MAX;
                    for r in 0..n { let v = data[r * d + j]; if v.is_finite() && v < m { m = v; } }
                    m
                };
                for r in 0..n {
                    let bi = ((data[r * d + i] - mi) / ri * (bins - 1) as f64).round() as usize;
                    let bj = ((data[r * d + j] - mj) / rj * (bins - 1) as f64).round() as usize;
                    grid[bi.min(bins - 1)][bj.min(bins - 1)] = true;
                }
                for row in &grid {
                    for &cell in row {
                        total_bins_2d += 1;
                        if !cell { hole_count += 1; }
                    }
                }
            }
        }
        let completeness = if total_bins_2d > 0 {
            1.0 - hole_count as f64 / total_bins_2d as f64
        } else {
            0.0
        };

        // 5. Diversity: unique nearest-neighbor distances entropy
        let check = n.min(100);
        let mut nn_dists: Vec<f64> = Vec::with_capacity(check);
        for i in 0..check {
            let mut min_d = f64::MAX;
            for j in 0..n {
                if i == j { continue; }
                let dist = shared.dist(i, j);
                if dist < min_d { min_d = dist; }
            }
            nn_dists.push(min_d);
        }
        let diversity = if nn_dists.is_empty() {
            0.0
        } else {
            nn_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let d_range = (nn_dists.last().unwrap() - nn_dists[0]).max(1e-12);
            let mut d_bins = vec![0u32; bins];
            for &dist in &nn_dists {
                let idx = ((dist - nn_dists[0]) / d_range * (bins - 1) as f64).round() as usize;
                d_bins[idx.min(bins - 1)] += 1;
            }
            let total = nn_dists.len() as f64;
            let entropy: f64 = d_bins.iter().map(|&c| {
                if c > 0 { let p = c as f64 / total; -p * p.ln() } else { 0.0 }
            }).sum();
            entropy / (bins as f64).ln() // normalized
        };

        // Corpus quality score
        let quality = coverage.tanh() * 0.15 // use tanh to bound
            + overall_balance * 0.25
            + consistency * 0.2
            + completeness * 0.25
            + diversity * 0.15;

        let mut result = HashMap::new();
        result.insert("corpus_quality".into(), vec![quality]);
        result.insert("coverage".into(), vec![coverage]);
        result.insert("balance".into(), vec![overall_balance]);
        result.insert("consistency".into(), vec![consistency]);
        result.insert("completeness".into(), vec![completeness]);
        result.insert("diversity".into(), vec![diversity]);
        result.insert("hole_ratio".into(), vec![if total_bins_2d > 0 { hole_count as f64 / total_bins_2d as f64 } else { 0.0 }]);
        result
    }
}
