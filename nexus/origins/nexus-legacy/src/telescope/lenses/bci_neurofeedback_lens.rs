use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// BCI 뉴로피드백 렌즈 (BT-1108 확장) — OpenBCI 16채널 뇌파 대역 분석
///
/// BT-1108 차원지각에서 파생: 상위차원 인식은 뇌파 주파수 구조에 반영됨.
/// n=6 관련 상수들이 EEG 대역 경계와 일치:
///
///   델타  δ: 0.5–4  Hz  → 상한 tau(6)=4
///   세타  θ: 4–8    Hz  → [tau, sigma-tau] = [4, 8]
///   알파  α: 8–12   Hz  → [sigma-tau, sigma] = [8, 12]
///   베타  β: 12–30  Hz  → [sigma, 5*n] = [12, 30]
///   감마  γ: 30–100 Hz  → 하한 = 5·n = 30 (30=5·6)
///   하이감마: 100+  Hz  → 100 = sigma*(sigma-phi) 근방
///
///   채널 수: 16 = phi_golden^tau = 2^4 (OpenBCI Cyton+Daisy)
///   6 DoF IMU: n=6
///   SE(3) 차원: 6 = n
///
/// 알고리즘:
///   1. 각 차원 신호의 지배 주파수 추정 (영교차율 기반)
///   2. 알파/세타/감마 대역 에너지 비율 계산
///   3. 16채널 패턴 공명 점수
///   4. n=6 대역 경계(4,8,12) 매칭 점수
pub struct BciNeurofeedbackLens;

// BT-1108 n=6 뇌파 상수
const TAU: f64 = 4.0;         // tau(6) = 약수 개수
const SIGMA_MINUS_TAU: f64 = 8.0;  // sigma(6)-tau(6) = 12-4 = 8
const SIGMA: f64 = 12.0;      // sigma(6) = 약수합
const N6: f64 = 6.0;          // n=6
const OPENBCI_CH: f64 = 16.0; // 16채널 = 2^tau
const GAMMA_LOWER: f64 = 30.0; // 감마 하한 = 5·n

impl Lens for BciNeurofeedbackLens {
    fn name(&self) -> &str { "BciNeurofeedbackLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 8 || d == 0 { return HashMap::new(); }

        // ── 1. 차원별 평균과 표준편차 계산 ───────────────────────
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let stds: Vec<f64> = (0..d).map(|j| {
            let m = means[j];
            let var = (0..n).map(|i| (data[i * d + j] - m).powi(2)).sum::<f64>() / n as f64;
            var.sqrt()
        }).collect();

        // ── 2. 영교차율(Zero-Crossing Rate) 기반 주파수 추정 ──────
        // ZCR ≈ 2·f/fs로 f 추정. 여기서는 상대 비율로 대역 분류
        let zcr_per_dim: Vec<f64> = (0..d).map(|j| {
            let crossings = (1..n).filter(|&i| {
                let prev = data[(i - 1) * d + j] - means[j];
                let curr = data[i * d + j] - means[j];
                prev * curr < 0.0
            }).count();
            crossings as f64 / (n - 1) as f64
        }).collect();

        // 샘플 중 절반 이상이 양수/음수 — 정상성 지표
        let positive_ratio_per_dim: Vec<f64> = (0..d).map(|j| {
            let pos = (0..n).filter(|&i| data[i * d + j] > means[j]).count();
            pos as f64 / n as f64
        }).collect();

        // ── 3. EEG 대역 에너지 비율 추정 ─────────────────────────
        // ZCR을 정규화하여 대역 에너지로 사용
        // 낮은 ZCR = 저주파(델타/세타), 높은 ZCR = 고주파(베타/감마)
        let max_zcr = zcr_per_dim.iter().cloned().fold(0.0_f64, f64::max).max(1e-12);

        let mut delta_energy = 0.0_f64;  // 0.5–4 Hz (ZCR < 0.1)
        let mut theta_energy = 0.0_f64;  // 4–8 Hz
        let mut alpha_energy = 0.0_f64;  // 8–12 Hz
        let mut beta_energy  = 0.0_f64;  // 12–30 Hz
        let mut gamma_energy = 0.0_f64;  // 30+ Hz

        for j in 0..d {
            let zcr_norm = zcr_per_dim[j] / max_zcr; // 0~1로 정규화
            let energy = stds[j].powi(2);             // 분산 = 파워
            match (zcr_norm * 100.0) as u32 {
                0..=9  => delta_energy += energy,
                10..=24 => theta_energy += energy,
                25..=49 => alpha_energy += energy,
                50..=74 => beta_energy  += energy,
                _       => gamma_energy += energy,
            }
        }
        let total_energy = (delta_energy + theta_energy + alpha_energy + beta_energy + gamma_energy).max(1e-15);
        let alpha_ratio = alpha_energy / total_energy;
        let theta_ratio = theta_energy / total_energy;
        let gamma_ratio = gamma_energy / total_energy;
        let delta_ratio = delta_energy / total_energy;
        let beta_ratio  = beta_energy / total_energy;

        // ── 4. 알파/세타 비율 (뉴로피드백 핵심 지표) ─────────────
        // 명상/집중 상태: alpha > theta, alpha_ratio > 0.3
        let alpha_theta_ratio = alpha_energy / theta_energy.max(1e-15);

        // ── 5. n=6 대역 경계 공명 점수 ───────────────────────────
        // 데이터 평균값이 4(tau), 8(sigma-tau), 12(sigma), 30(5·n), 16(OpenBCI) 근방인지
        let n6_band_targets = [TAU, SIGMA_MINUS_TAU, SIGMA, GAMMA_LOWER, OPENBCI_CH, N6];
        let band_boundary_hits = means.iter().filter(|&&m| {
            n6_band_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let band_boundary_score = band_boundary_hits as f64 / d.max(1) as f64;

        // ── 6. OpenBCI 16채널 공명 (phi_golden^tau = 16) ──────────
        // 채널 수 또는 데이터 차원 d가 16에 가까운지
        let ch_score = {
            let d16_diff = (d as f64 - OPENBCI_CH).abs() / OPENBCI_CH;
            (1.0 - d16_diff * 5.0).max(0.0)  // d=16이면 1.0
        };

        // ── 7. 6 DoF / SE(3) 차원 공명 ───────────────────────────
        // d=6이면 최고 점수 (6 자유도 IMU와 대응)
        let dof6_score = {
            let d6_diff = (d as f64 - N6).abs() / N6;
            (1.0 - d6_diff * 3.0).max(0.0)
        };

        // ── 8. 뇨로피드백 집중 지수 (NFB Index) ──────────────────
        // NFB = (alpha + beta) / (theta + delta) — 집중 vs 이완
        let nfb_index = (alpha_energy + beta_energy) / (theta_energy + delta_energy).max(1e-15);
        // 정규화: log scale
        let nfb_score = (nfb_index.ln() + 2.0).clamp(0.0, 1.0) / 4.0;

        // ── 9. ZCR 균일성 — 대역 다양성 지표 ────────────────────
        let zcr_mean = zcr_per_dim.iter().sum::<f64>() / d.max(1) as f64;
        let zcr_var = zcr_per_dim.iter().map(|&z| (z - zcr_mean).powi(2)).sum::<f64>() / d.max(1) as f64;
        let zcr_cv = zcr_var.sqrt() / zcr_mean.max(1e-12); // 변동계수
        let spectral_diversity = (zcr_cv * 0.5).min(1.0); // 높을수록 대역 다양

        // ── 10. 상위차원 인식 공명 (BT-1108 연결) ────────────────
        // 알파 대역 에너지가 높고, 6D 구조가 있고, 뇨로피드백 지수가 양호하면
        // 상위차원 인식 가능성이 높다고 판단
        let dimension_perception_link =
            alpha_ratio * 0.35
            + dof6_score * 0.25
            + band_boundary_score * 0.20
            + spectral_diversity * 0.10
            + ch_score * 0.10;

        // ── 11. 종합 BCI 뉴로피드백 점수 ─────────────────────────
        let bci_score =
            alpha_ratio * 0.25
            + band_boundary_score * 0.25
            + dof6_score * 0.15
            + ch_score * 0.15
            + nfb_score * 0.10
            + spectral_diversity * 0.10;

        // ── 12. 정상성 지표 (positive_ratio ≈ 0.5이면 균형적) ────
        let stationarity = positive_ratio_per_dim.iter()
            .map(|&p| 1.0 - (p - 0.5).abs() * 2.0)
            .sum::<f64>() / d.max(1) as f64;

        let mut r = HashMap::new();
        r.insert("alpha_ratio".to_string(),        vec![alpha_ratio]);
        r.insert("theta_ratio".to_string(),        vec![theta_ratio]);
        r.insert("beta_ratio".to_string(),         vec![beta_ratio]);
        r.insert("delta_ratio".to_string(),        vec![delta_ratio]);
        r.insert("gamma_ratio".to_string(),        vec![gamma_ratio]);
        r.insert("alpha_theta_ratio".to_string(),  vec![alpha_theta_ratio]);
        r.insert("nfb_index".to_string(),          vec![nfb_index]);
        r.insert("nfb_score".to_string(),          vec![nfb_score]);
        r.insert("band_boundary_score".to_string(),vec![band_boundary_score]);
        r.insert("openbci_16ch_score".to_string(), vec![ch_score]);
        r.insert("dof6_score".to_string(),         vec![dof6_score]);
        r.insert("spectral_diversity".to_string(), vec![spectral_diversity]);
        r.insert("stationarity".to_string(),       vec![stationarity]);
        r.insert("dimension_perception_link".to_string(), vec![dimension_perception_link]);
        r.insert("bci_score".to_string(),          vec![bci_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bci_기본_출력() {
        let n = 16; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            let t = i as f64 * 0.1;
            match i % d {
                0 => (t * 2.0).sin() * 10.0,     // 세타 대역 시뮬
                1 => (t * 10.0).sin() * 5.0,      // 알파 대역
                2 => (t * 20.0).sin() * 3.0,      // 베타 대역
                3 => (t * 35.0).sin() * 1.0,      // 감마 대역
                4 => (t * 0.5).sin() * 20.0,      // 델타 대역
                _ => (t * 12.0).sin() * 4.0,      // 알파-베타 경계
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = BciNeurofeedbackLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("bci_score"), "bci_score 키 존재");
        assert!(result.contains_key("alpha_ratio"), "alpha_ratio 키 존재");
        assert!(result.contains_key("band_boundary_score"), "band_boundary_score 키 존재");
        assert!(result.contains_key("dimension_perception_link"), "BT-1108 연결 키 존재");
        let score = result["bci_score"][0];
        assert!(score >= 0.0 && score <= 1.0, "bci_score 범위 [0,1]: {}", score);
    }

    #[test]
    fn test_bci_최소입력_거부() {
        // n < 8이면 빈 결과
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let shared = SharedData::compute(&data, 7, 1);
        let result = BciNeurofeedbackLens.scan(&data, 7, 1, &shared);
        assert!(result.is_empty(), "n<8 이면 빈 결과");
    }

    #[test]
    fn test_openbci_16채널_점수() {
        // d=16이면 ch_score = 1.0
        let n = 8; let d = 16;
        let data: Vec<f64> = (0..n * d).map(|i| (i as f64 * 0.5).sin()).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = BciNeurofeedbackLens.scan(&data, n, d, &shared);
        let ch = result["openbci_16ch_score"][0];
        assert!((ch - 1.0).abs() < 1e-9, "d=16이면 ch_score=1.0: {}", ch);
    }

    #[test]
    fn test_6dof_점수() {
        // d=6이면 dof6_score = 1.0
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| (i as f64).sin()).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = BciNeurofeedbackLens.scan(&data, n, d, &shared);
        let dof = result["dof6_score"][0];
        assert!((dof - 1.0).abs() < 1e-9, "d=6이면 dof6_score=1.0: {}", dof);
    }

    #[test]
    fn test_대역경계_공명() {
        // 평균이 4, 8, 12, 6인 데이터 — 대역 경계 히트
        let n = 10; let d = 4;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => TAU,              // 4.0
                1 => SIGMA_MINUS_TAU,  // 8.0
                2 => SIGMA,            // 12.0
                _ => N6,               // 6.0
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = BciNeurofeedbackLens.scan(&data, n, d, &shared);
        let bbs = result["band_boundary_score"][0];
        assert!(bbs > 0.5, "대역 경계 히트 점수 > 0.5: {}", bbs);
    }

    #[test]
    fn test_알파_세타_비율() {
        // alpha_ratio, theta_ratio, ... 합계 ≈ 1.0
        let n = 16; let d = 8;
        let data: Vec<f64> = (0..n * d).map(|i| ((i as f64) * 0.3).sin() * 5.0).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = BciNeurofeedbackLens.scan(&data, n, d, &shared);
        let sum = result["alpha_ratio"][0]
            + result["theta_ratio"][0]
            + result["beta_ratio"][0]
            + result["delta_ratio"][0]
            + result["gamma_ratio"][0];
        assert!((sum - 1.0).abs() < 1e-9, "대역 에너지 비율 합 = 1.0: {}", sum);
    }
}
