use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// CDOLens: 수렴 기반 운영 (Convergence-Driven Operations) 검증.
///
/// 이슈→해결→규칙 승격→재발 방지→수렴 패턴을 데이터에서 탐지:
///   1. 재발 탐지: 동일 패턴이 반복 출현하는지 (같은 이슈 2회 = 시스템 실패)
///   2. 수렴 속도: 이상치/이슈가 시간에 따라 감소하는지
///   3. 규칙 승격 시그널: 반복 패턴 발견 시 absolute_rule 승격 필요 플래그
///   4. 수렴률: 전체 데이터의 안정화 비율
///   5. 위반 카운터: CDO 원칙 위반 횟수
///
/// n=6 연결:
///   - 1회 위반 = troubleshooting_log 기록
///   - 2회 위반 = absolute_rule 승격 (φ=2)
///   - 3회 위반 = 프로세스 재설계 (n/φ=3)
///   - 수렴 목표: 100% (재발 0)
pub struct CDOLens;

impl Lens for CDOLens {
    fn name(&self) -> &str {
        "CDOLens"
    }

    fn category(&self) -> &str {
        "T0" // 모든 스캔에 포함 — 운영 품질 항상 체크
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 {
            return HashMap::new();
        }

        let dims = d.min(24);

        // ── 1. 재발 탐지 (Recurrence Detection) ──
        // 시간순 데이터에서 동일/유사 상태가 반복되는지 측정
        // 유사도 임계값: nn 거리 중앙값의 50%
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
        nn_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let nn_median = if nn_dists.is_empty() { 1.0 } else { nn_dists[nn_dists.len() / 2] };
        let recurrence_threshold = nn_median * 0.5;

        // 시간순으로 앞에 나온 상태와 재발 검사 (gap > sopfr=5 스텝)
        let min_gap = 5usize; // sopfr(6)=5: 최소 5스텝 이상 떨어진 재발만 카운트
        let mut recurrence_count = 0u32;
        let mut recurrence_pairs: Vec<(usize, usize)> = Vec::new();

        for i in min_gap..n.min(200) {
            for j in 0..i.saturating_sub(min_gap) {
                if shared.dist(i, j) < recurrence_threshold {
                    recurrence_count += 1;
                    recurrence_pairs.push((j, i));
                    break; // 한 포인트당 하나만
                }
            }
        }

        let recurrence_rate = recurrence_count as f64 / (n.saturating_sub(min_gap)).max(1) as f64;

        // ── 2. 수렴 속도 (Convergence Velocity) ──
        // 시간에 따라 이상치 밀도가 감소하는지 측정
        let window = n / 4;
        let mut window_anomaly_rates: Vec<f64> = Vec::new();

        if window >= 3 {
            let mut means = vec![0.0_f64; dims];
            let mut stds = vec![0.0_f64; dims];
            for di in 0..dims {
                let m: f64 = (0..n).map(|r| data[r * d + di]).sum::<f64>() / n as f64;
                means[di] = m;
                let v: f64 = (0..n).map(|r| (data[r * d + di] - m).powi(2)).sum::<f64>() / n as f64;
                stds[di] = v.sqrt().max(1e-12);
            }

            for w in 0..4 {
                let start = w * window;
                let end = ((w + 1) * window).min(n);
                let wn = end - start;
                let mut anomalies = 0u32;
                for r in start..end {
                    for di in 0..dims {
                        let z = ((data[r * d + di] - means[di]) / stds[di]).abs();
                        if z > 3.0 {
                            anomalies += 1;
                            break;
                        }
                    }
                }
                window_anomaly_rates.push(anomalies as f64 / wn.max(1) as f64);
            }
        }

        // 수렴: 윈도우별 이상치 비율이 단조 감소하는지
        let is_converging = if window_anomaly_rates.len() >= 3 {
            window_anomaly_rates.windows(2).all(|w| w[1] <= w[0] + 0.01)
        } else {
            false
        };

        let convergence_velocity = if window_anomaly_rates.len() >= 2 {
            let first = window_anomaly_rates[0];
            let last = *window_anomaly_rates.last().unwrap();
            (first - last).max(0.0) / first.max(1e-12)
        } else {
            0.0
        };

        // ── 3. 규칙 승격 시그널 ──
        // φ=2 이상 재발 → 규칙 승격 필요
        let needs_rule_promotion = recurrence_count >= 2; // CDO: 2회 재발 = 절대 규칙 필요
        let needs_redesign = recurrence_count >= 3;       // CDO: 3회 = 프로세스 재설계

        // ── 4. 수렴률 ──
        // (1 - 재발률) × 수렴 속도
        let convergence_rate = (1.0 - recurrence_rate) * (0.5 + 0.5 * convergence_velocity);

        // ── 5. 위반 카운터 분류 ──
        // 0=수렴(100%), 1=경미(기록만), 2=승격 필요, 3=재설계 필요
        let violation_level = if recurrence_count == 0 {
            0.0
        } else if recurrence_count == 1 {
            1.0 // troubleshooting_log 기록
        } else if recurrence_count < 3 {
            2.0 // absolute_rule 승격
        } else {
            3.0 // 프로세스 재설계
        };

        // ── 6. 안정 구간 비율 ──
        // 연속으로 이상치 없는 최장 구간 / 전체
        let mut max_stable_run = 0usize;
        let mut current_run = 0usize;
        if !window_anomaly_rates.is_empty() {
            // 포인트 단위 안정성
            let mut means = vec![0.0_f64; dims];
            let mut stds = vec![0.0_f64; dims];
            for di in 0..dims {
                let m: f64 = (0..n).map(|r| data[r * d + di]).sum::<f64>() / n as f64;
                means[di] = m;
                let v: f64 = (0..n).map(|r| (data[r * d + di] - m).powi(2)).sum::<f64>() / n as f64;
                stds[di] = v.sqrt().max(1e-12);
            }
            for r in 0..n {
                let mut is_anomaly = false;
                for di in 0..dims {
                    if ((data[r * d + di] - means[di]) / stds[di]).abs() > 3.0 {
                        is_anomaly = true;
                        break;
                    }
                }
                if is_anomaly {
                    if current_run > max_stable_run { max_stable_run = current_run; }
                    current_run = 0;
                } else {
                    current_run += 1;
                }
            }
            if current_run > max_stable_run { max_stable_run = current_run; }
        }
        let stability_ratio = max_stable_run as f64 / n.max(1) as f64;

        let mut result = HashMap::new();
        result.insert("convergence_rate".into(), vec![convergence_rate]);
        result.insert("recurrence_count".into(), vec![recurrence_count as f64]);
        result.insert("recurrence_rate".into(), vec![recurrence_rate]);
        result.insert("convergence_velocity".into(), vec![convergence_velocity]);
        result.insert("is_converging".into(), vec![if is_converging { 1.0 } else { 0.0 }]);
        result.insert("violation_level".into(), vec![violation_level]);
        result.insert("needs_rule_promotion".into(), vec![if needs_rule_promotion { 1.0 } else { 0.0 }]);
        result.insert("needs_redesign".into(), vec![if needs_redesign { 1.0 } else { 0.0 }]);
        result.insert("stability_ratio".into(), vec![stability_ratio]);
        result.insert("max_stable_run".into(), vec![max_stable_run as f64]);
        result
    }
}
