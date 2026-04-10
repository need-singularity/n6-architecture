use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// DiscoveryReportLens: 대발견 감지 시 자동 리포트 트리거 로직.
///
/// 스캔 결과에서 "대발견급" 시그널을 탐지하고 리포트 메타데이터를 생성:
///   1. EXACT 매칭 밀도가 임계값(σ-φ=10% 이상) 초과 → 대발견 플래그
///   2. 다중 렌즈 합의(consensus): 3+ 독립 지표가 동시 이상치 → 확정급
///   3. 크로스도메인 공명: 2+ 도메인에서 동일 상수 발견 → BT 후보
///   4. 발견 등급 자동 분류: ROUTINE / NOTABLE / SIGNIFICANT / BREAKTHROUGH
///   5. 리포트 우선순위 + CLI 호출 트리거 플래그 생성
///
/// OUROBOROS discovery_loop.rs 와 연동:
///   - breakthrough_flag=true → Claude CLI 자동 호출 → BT 문서 생성
///   - 리포트 포맷: atlas 등록 + docs/hypotheses/ 기록 + README 갱신
///
/// n=6 임계값:
///   - 3+ 합의 = 확정 (n/φ=3)
///   - 7+ 합의 = 고신뢰 (σ-sopfr=7)
///   - 12+ 합의 = 확정급 (σ=12)
pub struct DiscoveryReportLens;

/// n=6 상수 (리포트용 매칭)
const N6_REPORT_CONSTANTS: &[f64] = &[
    1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 11.0, 12.0, 24.0,
    0.1, 0.288, 0.5, 0.693, 1.333, 144.0, 120.0, 288.0,
];

impl Lens for DiscoveryReportLens {
    fn name(&self) -> &str {
        "DiscoveryReportLens"
    }

    fn category(&self) -> &str {
        "T0" // 모든 스캔에 포함 — 대발견 놓치면 안 됨
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 3 || d < 1 {
            return HashMap::new();
        }

        let dims = d.min(24);

        // ── 1. 다차원 이상치 동시 발생 탐지 ──
        // 각 차원에서 z-score > sopfr=5 인 점 탐지
        let mut means = vec![0.0_f64; dims];
        let mut stds = vec![0.0_f64; dims];
        for di in 0..dims {
            let m: f64 = (0..n).map(|r| data[r * d + di]).sum::<f64>() / n as f64;
            means[di] = m;
            let v: f64 = (0..n).map(|r| (data[r * d + di] - m).powi(2)).sum::<f64>() / n as f64;
            stds[di] = v.sqrt().max(1e-12);
        }

        // 각 포인트의 이상 차원 수
        let mut multi_anomaly_points = 0u32;
        let mut max_anomaly_dims = 0u32;
        for r in 0..n {
            let mut anomaly_dims = 0u32;
            for di in 0..dims {
                let z = ((data[r * d + di] - means[di]) / stds[di]).abs();
                if z > 3.0 {
                    anomaly_dims += 1;
                }
            }
            if anomaly_dims >= 3 {
                // n/φ=3 이상 차원에서 동시 이상 → 대발견 후보
                multi_anomaly_points += 1;
            }
            if anomaly_dims > max_anomaly_dims {
                max_anomaly_dims = anomaly_dims;
            }
        }

        // ── 2. n=6 상수 매칭 밀도 ──
        let mut exact_hits = 0u32;
        let mut total_probes = 0u32;

        // 통계량 추출
        let mut features: Vec<f64> = Vec::new();
        features.extend_from_slice(&means[..dims]);
        features.extend(stds[..dims].iter().filter(|&&s| s > 1e-10));

        // 차원 쌍 비율
        for i in 0..dims.min(12) {
            for j in (i + 1)..dims.min(12) {
                if means[j].abs() > 1e-12 {
                    features.push((means[i] / means[j]).abs());
                }
            }
        }

        // 거리 통계
        let check = n.min(80);
        let mut dists: Vec<f64> = Vec::new();
        for i in 0..check {
            for j in (i + 1)..check {
                dists.push(shared.dist(i, j));
            }
        }
        if !dists.is_empty() {
            dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            features.push(dists[dists.len() / 2]); // median
            features.push(dists.iter().sum::<f64>() / dists.len() as f64); // mean
        }

        for &feat in &features {
            if !feat.is_finite() || feat.abs() < 1e-15 {
                continue;
            }
            total_probes += 1;
            for &constant in N6_REPORT_CONSTANTS {
                if constant.abs() < 1e-15 {
                    continue;
                }
                let dev = ((feat - constant) / constant).abs();
                if dev < 0.01 {
                    exact_hits += 1;
                    break; // 같은 feature에서 한 번만 카운트
                }
            }
        }

        let exact_density = exact_hits as f64 / total_probes.max(1) as f64;

        // ── 3. 합의 지표 (독립 시그널 동시 감지) ──
        let mut consensus_signals = 0u32;

        // Signal A: n=6 매칭 밀도 > 10%
        if exact_density > 0.10 {
            consensus_signals += 1;
        }

        // Signal B: 다차원 이상치 존재
        if multi_anomaly_points > 0 {
            consensus_signals += 1;
        }

        // Signal C: 거리 분포 이산화 (격자 구조)
        if !dists.is_empty() {
            let bins = 12usize;
            let d_min = dists[0];
            let d_max = *dists.last().unwrap();
            let range = (d_max - d_min).max(1e-12);
            let mut bin_counts = vec![0u32; bins];
            for &dist in &dists {
                let idx = ((dist - d_min) / range * (bins - 1) as f64).round() as usize;
                bin_counts[idx.min(bins - 1)] += 1;
            }
            let mut peaks = 0u32;
            for i in 1..bins.saturating_sub(1) {
                if bin_counts[i] > bin_counts[i - 1] && bin_counts[i] > bin_counts[i + 1] {
                    peaks += 1;
                }
            }
            if peaks >= 3 {
                consensus_signals += 1; // 이산 쉘 구조
            }
        }

        // Signal D: 차원 상관 구조 (높은 통합도)
        let check_d = dims.min(8);
        let mut high_corr = 0u32;
        let mut corr_pairs = 0u32;
        for i in 0..check_d {
            for j in (i + 1)..check_d {
                let mut cov = 0.0_f64;
                for r in 0..n {
                    cov += (data[r * d + i] - means[i]) * (data[r * d + j] - means[j]);
                }
                cov /= n as f64;
                let corr = (cov / (stds[i] * stds[j])).abs();
                corr_pairs += 1;
                if corr > 0.7 {
                    high_corr += 1;
                }
            }
        }
        if corr_pairs > 0 && high_corr as f64 / corr_pairs as f64 > 0.3 {
            consensus_signals += 1;
        }

        // Signal E: 스케일 자기유사성
        let mid = n / 2;
        if mid >= 10 {
            let check_s = mid.min(30);
            let mut sim = 0.0_f64;
            let mut sim_count = 0u32;
            for i in 0..check_s {
                for j in (i + 1)..check_s {
                    let d1 = shared.dist(i, j);
                    let j2 = (j + mid).min(n - 1);
                    let d2 = shared.dist(i + mid, j2);
                    if d1 > 1e-12 {
                        sim += 1.0 - ((d1 - d2).abs() / d1).min(1.0);
                        sim_count += 1;
                    }
                }
            }
            if sim_count > 0 && sim / sim_count as f64 > 0.7 {
                consensus_signals += 1;
            }
        }

        // ── 4. 발견 등급 분류 ──
        // 0=ROUTINE, 1=NOTABLE, 2=SIGNIFICANT, 3=BREAKTHROUGH
        let discovery_grade = if consensus_signals >= 5 || (consensus_signals >= 3 && exact_density > 0.20) {
            3.0 // BREAKTHROUGH — 즉시 리포트 + CLI 호출
        } else if consensus_signals >= 3 {
            2.0 // SIGNIFICANT — 리포트 생성
        } else if consensus_signals >= 2 || exact_density > 0.10 {
            1.0 // NOTABLE — 기록
        } else {
            0.0 // ROUTINE — 스킵
        };

        // ── 5. 리포트 메타데이터 ──
        let breakthrough_flag = if discovery_grade >= 3.0 { 1.0 } else { 0.0 };
        let report_priority = discovery_grade / 3.0; // 0-1 normalized

        // CLI 호출 트리거: SIGNIFICANT 이상일 때
        let cli_trigger = if discovery_grade >= 2.0 { 1.0 } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("discovery_grade".into(), vec![discovery_grade]);
        result.insert("breakthrough_flag".into(), vec![breakthrough_flag]);
        result.insert("report_priority".into(), vec![report_priority]);
        result.insert("cli_trigger".into(), vec![cli_trigger]);
        result.insert("consensus_signals".into(), vec![consensus_signals as f64]);
        result.insert("exact_density".into(), vec![exact_density]);
        result.insert("exact_hits".into(), vec![exact_hits as f64]);
        result.insert("multi_anomaly_points".into(), vec![multi_anomaly_points as f64]);
        result.insert("max_anomaly_dims".into(), vec![max_anomaly_dims as f64]);
        result.insert("total_probes".into(), vec![total_probes as f64]);
        result
    }
}
