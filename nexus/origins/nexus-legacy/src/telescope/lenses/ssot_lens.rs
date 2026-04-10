use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// SSOTLens: Single Source of Truth 검증 — 데이터 중복/불일치 탐지.
///
/// 동일 데이터가 여러 곳에서 참조될 때 불일치를 감지:
///   1. 중복 탐지: 동일/유사한 데이터 포인트가 여러 번 출현
///   2. 불일치 탐지: 유사하지만 미세하게 다른 복사본 (drift/desync)
///   3. 원본 식별: 가장 "대표적인" 데이터 포인트 (centroid 근접)
///   4. 동기화 품질: 복사본 간 편차 정도
///   5. SSOT 점수: 1.0 = 완전 일관 (중복 없거나 완전 동기), 0.0 = 심각한 불일치
///
/// n=6 연결:
///   - 중복 2곳 이상 = JSON 원본 1개 + 마커 자동 생성 규칙 적용
///   - SSOT 위반 = 하드코딩 감지 → 원본 참조로 교체 필요
///   - σ=12 bin 해상도로 일관성 측정
pub struct SSOTLens;

impl Lens for SSOTLens {
    fn name(&self) -> &str {
        "SSOTLens"
    }

    fn category(&self) -> &str {
        "T0" // 모든 스캔에 포함 — 데이터 일관성 항상 체크
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 {
            return HashMap::new();
        }

        let dims = d.min(24);

        // ── 1. 정확한 중복 탐지 (Exact Duplicates) ──
        let mut exact_dupes = 0u32;
        let check = n.min(200);
        for i in 0..check {
            for j in (i + 1)..check {
                let mut identical = true;
                for di in 0..dims {
                    if (data[i * d + di] - data[j * d + di]).abs() > 1e-15 {
                        identical = false;
                        break;
                    }
                }
                if identical {
                    exact_dupes += 1;
                }
            }
        }

        // ── 2. 근사 중복 (Near Duplicates = 불일치 복사본) ──
        // 거리가 매우 가깝지만 0이 아닌 쌍 = drift/desync된 복사본
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
        nn_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Near-duplicate threshold: 5th percentile of NN distances
        let near_threshold = if nn_dists.len() >= 20 {
            nn_dists[nn_dists.len() / 20]
        } else if !nn_dists.is_empty() {
            nn_dists[0] * 2.0
        } else {
            1.0
        };

        let mut near_dupes = 0u32;
        let mut max_drift = 0.0_f64;
        let mut drift_sum = 0.0_f64;
        for i in 0..check {
            for j in (i + 1)..check {
                let dist = shared.dist(i, j);
                if dist > 1e-15 && dist < near_threshold {
                    near_dupes += 1;
                    if dist > max_drift {
                        max_drift = dist;
                    }
                    drift_sum += dist;
                }
            }
        }
        let avg_drift = if near_dupes > 0 {
            drift_sum / near_dupes as f64
        } else {
            0.0
        };

        // ── 3. 차원별 일관성 (Dimension Consistency) ──
        // 각 차원에서 동일 bin에 속하는 비율 (σ=12 bins)
        let bins = 12usize;
        let mut inconsistent_dims = 0u32;

        for di in 0..dims {
            let mut col: Vec<f64> = (0..n)
                .map(|r| data[r * d + di])
                .filter(|v| v.is_finite())
                .collect();
            if col.len() < 2 { continue; }

            let min_v = col.iter().cloned().fold(f64::MAX, f64::min);
            let max_v = col.iter().cloned().fold(f64::MIN, f64::max);
            let range = (max_v - min_v).max(1e-12);

            // 빈 분포
            let mut bin_counts = vec![0u32; bins];
            for &v in &col {
                let idx = ((v - min_v) / range * (bins - 1) as f64).round() as usize;
                bin_counts[idx.min(bins - 1)] += 1;
            }

            // 최대 빈이 전체의 80% 미만 = 분산된 값 = 불일관
            let max_bin = *bin_counts.iter().max().unwrap_or(&0) as f64;
            let total = col.len() as f64;
            if max_bin / total < 0.8 {
                // 여러 빈에 분산 = 잠재적 불일치
                // 하지만 자연스러운 분포일 수도 있으므로 가중치 낮게
            }

            // 이봉분포(bimodal) 탐지 = 두 개의 "원본"이 혼재
            let mut peaks = 0u32;
            for i in 1..bins.saturating_sub(1) {
                if bin_counts[i] > bin_counts[i - 1] && bin_counts[i] > bin_counts[i + 1] {
                    peaks += 1;
                }
            }
            if peaks >= 2 {
                inconsistent_dims += 1; // 이봉 = 두 소스 혼재 가능
            }
        }

        // ── 4. 클러스터 간 불일치 (Source Divergence) ──
        // 데이터를 φ=2 그룹으로 나누고 그룹 간 중심점 차이 측정
        let mut sorted_by_dist: Vec<(usize, f64)> = (0..n.min(check))
            .map(|i| {
                let mut centroid_dist = 0.0_f64;
                let mut means = vec![0.0_f64; dims];
                for di in 0..dims {
                    means[di] = (0..n).map(|r| data[r * d + di]).sum::<f64>() / n as f64;
                }
                for di in 0..dims {
                    centroid_dist += (data[i * d + di] - means[di]).powi(2);
                }
                (i, centroid_dist.sqrt())
            })
            .collect();
        sorted_by_dist.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        let half = sorted_by_dist.len() / 2;
        let group_a: Vec<usize> = sorted_by_dist[..half].iter().map(|x| x.0).collect();
        let group_b: Vec<usize> = sorted_by_dist[half..].iter().map(|x| x.0).collect();

        let mut source_divergence = 0.0_f64;
        if !group_a.is_empty() && !group_b.is_empty() {
            for di in 0..dims {
                let mean_a: f64 = group_a.iter().map(|&r| data[r * d + di]).sum::<f64>()
                    / group_a.len() as f64;
                let mean_b: f64 = group_b.iter().map(|&r| data[r * d + di]).sum::<f64>()
                    / group_b.len() as f64;
                source_divergence += (mean_a - mean_b).powi(2);
            }
            source_divergence = source_divergence.sqrt() / dims as f64;
        }

        // ── 5. SSOT 점수 계산 ──
        let max_pairs = (check * (check - 1) / 2).max(1) as f64;
        let exact_dupe_ratio = exact_dupes as f64 / max_pairs;
        let near_dupe_ratio = near_dupes as f64 / max_pairs;
        let inconsistency_ratio = inconsistent_dims as f64 / dims.max(1) as f64;

        // SSOT score: 1.0 = 완전 일관, 0.0 = 심각한 불일치
        // 정확한 중복은 괜찮을 수 있음 (같은 원본), 근사 중복이 문제 (desync)
        let ssot_score = (1.0
            - near_dupe_ratio * 5.0      // desync된 복사본이 가장 나쁨
            - inconsistency_ratio * 3.0   // 이봉분포 (두 소스 혼재)
            - exact_dupe_ratio * 1.0)     // 정확 중복은 덜 심각
            .max(0.0)
            .min(1.0);

        // SSOT 위반 플래그
        let ssot_violation = near_dupe_ratio > 0.05 || inconsistency_ratio > 0.3;

        // 하드코딩 시그널: 특정 값이 비정상적으로 많이 반복
        let mut hardcoded_signal = 0.0_f64;
        for di in 0..dims {
            let mut val_counts: HashMap<u64, u32> = HashMap::new();
            for r in 0..n {
                let v = data[r * d + di];
                if v.is_finite() {
                    let key = (v * 1e6).round() as u64;
                    *val_counts.entry(key).or_insert(0) += 1;
                }
            }
            let max_count = val_counts.values().max().copied().unwrap_or(0);
            if max_count as f64 > n as f64 * 0.5 {
                hardcoded_signal += 1.0; // 50% 이상 같은 값 = 하드코딩 의심
            }
        }
        hardcoded_signal /= dims.max(1) as f64;

        let mut result = HashMap::new();
        result.insert("ssot_score".into(), vec![ssot_score]);
        result.insert("exact_duplicates".into(), vec![exact_dupes as f64]);
        result.insert("near_duplicates".into(), vec![near_dupes as f64]);
        result.insert("max_drift".into(), vec![max_drift]);
        result.insert("avg_drift".into(), vec![avg_drift]);
        result.insert("inconsistent_dims".into(), vec![inconsistent_dims as f64]);
        result.insert("source_divergence".into(), vec![source_divergence]);
        result.insert("ssot_violation".into(), vec![if ssot_violation { 1.0 } else { 0.0 }]);
        result.insert("hardcoded_signal".into(), vec![hardcoded_signal]);
        result.insert("exact_dupe_ratio".into(), vec![exact_dupe_ratio]);
        result.insert("near_dupe_ratio".into(), vec![near_dupe_ratio]);
        result
    }
}
