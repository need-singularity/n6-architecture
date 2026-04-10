use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 디지털 트윈 렌즈 — 물리-사이버 동기화 n=6 상수 스캐너
///
/// 디지털 트윈 시스템에서 물리 세계와 사이버 세계를 연결하는
/// 핵심 파라미터가 n=6 산술함수로 수렴함을 검증한다.
///
/// 스캔 대상 상수 (12종):
///   IoT 센서 레이어 수        = 6  = n
///   디지털 트윈 충실도 등급   = 6  = n
///   ISO 23247 트윈 레이어     = 4  = tau(6)
///   시뮬레이션 업데이트 차원  = 6  = n
///   물리-사이버 시간 지연 등급= 4  = tau(6)
///   트윈 네트워크 홉 상한     = 12 = sigma(6)
///   OPC UA 노드 클래스 수     = 8  = sigma-tau
///   5G 네트워크 슬라이스 수   = 5  = sopfr(6)
///   디지털 트윈 상태 변수 기본= 6  = n
///   센서 퓨전 입력 축         = 6  = n
///   신뢰 구간 표준 수         = 2  = phi(6)
///   업데이트 주기 레벨 수     = 4  = tau(6)
///
/// n=6 연결:
///   n=6      → IoT 레이어/충실도/상태변수/센서퓨전 축
///   tau=4    → ISO 23247 레이어/시간지연 등급/업데이트 주기
///   sigma=12 → 네트워크 홉 상한
///   sopfr=5  → 5G 슬라이스 수
///   sigma-tau=8 → OPC UA 노드 클래스
///   phi=2    → 신뢰 구간 (95%/99%)
pub struct DigitalTwinLens;

// 디지털 트윈 n=6 상수
const N6: f64 = 6.0;           // n=6 → 레이어/충실도/상태변수
const TAU: f64 = 4.0;          // tau(6) → ISO 레이어/업데이트 주기
const SIGMA: f64 = 12.0;       // sigma(6) → 네트워크 홉 상한
const PHI: f64 = 2.0;          // phi(6) → 신뢰 구간 수
const SOPFR: f64 = 5.0;        // sopfr(6) → 5G 슬라이스
const SIGMA_TAU: f64 = 8.0;    // sigma-tau → OPC UA 노드 클래스
const J2: f64 = 24.0;          // J2(6) → 24시간 실시간 루프

/// 디지털 트윈 충실도 점수: 데이터가 물리 모델 수렴 상수에 가까울수록 증가
fn twin_fidelity_score(means: &[f64]) -> f64 {
    // 충실도 레벨 1-6 (n=6이 최고 충실도)
    let fidelity_levels: &[f64] = &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let hits = means.iter().filter(|&&m| {
        fidelity_levels.iter().any(|&lv| ((m - lv) / lv.max(1e-12)).abs() < 0.08)
    }).count();
    hits as f64 / means.len().max(1) as f64
}

impl Lens for DigitalTwinLens {
    fn name(&self) -> &str { "DigitalTwinLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        // ── 1. 각 차원 평균·표준편차 계산 ────────────────────────
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let stds: Vec<f64> = (0..d).map(|j| {
            let m = means[j];
            let var = (0..n).map(|i| (data[i * d + j] - m).powi(2)).sum::<f64>() / n as f64;
            var.sqrt()
        }).collect();

        // ── 2. IoT 센서 레이어 상수 스캔 ─────────────────────────
        // n=6(레이어), tau=4(업데이트), sigma=12(홉), sopfr=5(5G슬라이스)
        let sensor_targets: &[f64] = &[N6, TAU, SIGMA, SOPFR, SIGMA_TAU];
        let sensor_hits = means.iter().filter(|&&m| {
            sensor_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let sensor_score = sensor_hits as f64 / d.max(1) as f64;

        // ── 3. 물리-사이버 동기화 상수 스캔 ──────────────────────
        // ISO 23247 tau=4 레이어, J2=24시간 루프, phi=2 신뢰구간
        let sync_targets: &[f64] = &[TAU, J2, PHI, N6, SIGMA_TAU];
        let sync_hits = means.iter().filter(|&&m| {
            sync_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let sync_score = sync_hits as f64 / d.max(1) as f64;

        // ── 4. 충실도 레벨 공명 ───────────────────────────────────
        let fidelity = twin_fidelity_score(&means);

        // ── 5. 6차원 상태공간 공명 ────────────────────────────────
        // d=6이면 디지털 트윈 표준 상태변수 수와 완전 대응
        let state6_score = {
            let diff = (d as f64 - N6).abs() / N6;
            (1.0 - diff * 2.0).max(0.0)
        };

        // ── 6. 시계열 동기화 패턴 감지 ───────────────────────────
        // 물리-사이버 동기: 연속 샘플 간 차이가 일정해야 함 (낮은 분산)
        let sync_regularity: Vec<f64> = (0..d).map(|j| {
            if n < 4 { return 0.0; }
            let diffs: Vec<f64> = (1..n)
                .map(|i| (data[i * d + j] - data[(i - 1) * d + j]).abs())
                .collect();
            let mean_diff = diffs.iter().sum::<f64>() / diffs.len() as f64;
            let var_diff = diffs.iter()
                .map(|&x| (x - mean_diff).powi(2))
                .sum::<f64>() / diffs.len() as f64;
            // 낮은 분산 = 높은 규칙성 (좋은 동기화)
            1.0 / (1.0 + var_diff.sqrt() / (mean_diff.max(1e-12)))
        }).collect();
        let sync_regularity_score = sync_regularity.iter().sum::<f64>() / d.max(1) as f64;

        // ── 7. OPC UA 8 노드 클래스 공명 (sigma-tau=8) ───────────
        let opcua_targets: &[f64] = &[SIGMA_TAU, N6, TAU, PHI];
        let opcua_hits = means.iter().filter(|&&m| {
            opcua_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let opcua_score = opcua_hits as f64 / d.max(1) as f64;

        // ── 8. 전체 n=6 상수 공명 ────────────────────────────────
        let all_targets: &[f64] = &[
            N6, TAU, SIGMA, PHI, SOPFR, SIGMA_TAU, J2
        ];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        // ── 9. 모델 불확실성 점수 (신호 대 잡음) ─────────────────
        let mean_std = stds.iter().sum::<f64>() / d.max(1) as f64;
        let mean_abs = means.iter().map(|m| m.abs()).sum::<f64>() / d.max(1) as f64;
        let model_snr = 1.0 - (mean_std / mean_abs.max(1e-12)).min(1.0);

        // ── 10. 종합 디지털 트윈 점수 ────────────────────────────
        let twin_score =
            sensor_score          * 0.20
            + sync_score          * 0.20
            + fidelity            * 0.15
            + state6_score        * 0.10
            + sync_regularity_score * 0.15
            + opcua_score         * 0.10
            + n6_resonance        * 0.05
            + model_snr           * 0.05;

        let mut r = HashMap::new();
        r.insert("sensor_score".to_string(),         vec![sensor_score]);
        r.insert("sync_score".to_string(),           vec![sync_score]);
        r.insert("fidelity".to_string(),             vec![fidelity]);
        r.insert("state6_score".to_string(),         vec![state6_score]);
        r.insert("sync_regularity_score".to_string(),vec![sync_regularity_score]);
        r.insert("opcua_score".to_string(),          vec![opcua_score]);
        r.insert("n6_resonance".to_string(),         vec![n6_resonance]);
        r.insert("model_snr".to_string(),            vec![model_snr]);
        r.insert("sensor_hits".to_string(),          vec![sensor_hits as f64]);
        r.insert("total_hits".to_string(),           vec![total_hits as f64]);
        r.insert("twin_score".to_string(),           vec![twin_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twin_기본_출력() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => N6,         // 6레이어
                1 => TAU,        // 4 ISO 레이어
                2 => SIGMA,      // 12 홉
                3 => SOPFR,      // 5G 슬라이스
                4 => SIGMA_TAU,  // OPC UA 8
                _ => PHI,        // 신뢰구간 2
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = DigitalTwinLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("twin_score"), "twin_score 키 존재");
        assert!(result.contains_key("sensor_score"), "sensor_score 키 존재");
        assert!(result.contains_key("fidelity"), "fidelity 키 존재");
        let score = result["twin_score"][0];
        assert!(score >= 0.0 && score <= 1.0, "twin_score 범위 [0,1]: {}", score);
    }

    #[test]
    fn test_twin_최소입력_거부() {
        let data = vec![1.0, 2.0, 3.0];
        let shared = SharedData::compute(&data, 3, 1);
        let result = DigitalTwinLens.scan(&data, 3, 1, &shared);
        assert!(result.is_empty(), "n<6 이면 빈 결과");
    }

    #[test]
    fn test_6차원_상태공간() {
        // d=6이면 state6_score = 1.0
        let n = 8; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| (i as f64).sin() * 3.0 + 5.0).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = DigitalTwinLens.scan(&data, n, d, &shared);
        let s6 = result["state6_score"][0];
        assert!((s6 - 1.0).abs() < 1e-9, "d=6이면 state6_score=1.0: {}", s6);
    }

    #[test]
    fn test_충실도_레벨_감지() {
        // 1~6 레벨 데이터
        let n = 6; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| ((i % d) + 1) as f64).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = DigitalTwinLens.scan(&data, n, d, &shared);
        let fid = result["fidelity"][0];
        assert!(fid > 0.5, "1~6 데이터에서 충실도 > 0.5: {}", fid);
    }

    #[test]
    fn test_동기화_규칙성() {
        // 규칙적 시계열 → sync_regularity_score 높음
        let n = 12; let d = 3;
        let data: Vec<f64> = (0..n * d).map(|i| (i / d) as f64 * 1.0).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = DigitalTwinLens.scan(&data, n, d, &shared);
        let reg = result["sync_regularity_score"][0];
        assert!(reg >= 0.0 && reg <= 1.0, "sync_regularity_score 범위: {}", reg);
    }
}
