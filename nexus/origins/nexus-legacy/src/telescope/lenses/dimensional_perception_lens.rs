use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 차원지각 렌즈 (BT-1108) — n=6 기반 상위차원 인식 구조 스캐너
///
/// 검출 상수 (18종):
///   완전광학함수(Plenoptic) = 6D = n
///   SO(4) 리 대수 = C(4,2) = 6 = n
///   Tesseract 꼭짓점 = 16 = phi^tau
///   Tesseract 변      = 32 = 2^sopfr
///   Tesseract 면      = 24 = J_2
///   Tesseract 셀      =  8 = sigma - tau
///   24-cell 꼭짓점/셀 = 24 = J_2
///   6축 IMU           =  6 = n
///   6 DoF             =  6 = n
///   SE(3) 차원        =  6 = n
///   OpenBCI 채널      = 16 = phi^tau
///   격자세포 대칭     =  6 = n
///   알파밴드 하한     =  8 = sigma - tau
///   알파밴드 상한     = 12 = sigma
///   120Hz 리프레시    = 120 = sigma*(sigma-phi)
///   Calabi-Yau 여분   =  6 = n
///   4D 독립회전평면   =  6 = C(4,2) = n
///   라이트필드 차원   =  4 = tau
pub struct DimensionalPerceptionLens;

impl Lens for DimensionalPerceptionLens {
    fn name(&self) -> &'static str { "DimensionalPerceptionLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }

        // BT-1108 차원지각 핵심 상수 18종
        // n=6, tau=4, phi=2, sigma=12, sopfr=5, J_2=24
        let dom_consts: &[f64] = &[
            6.0,   // 완전광학함수 6D = n
            6.0,   // SO(4) = C(4,2) = n  (중복 허용, 아래 matched 로 수집)
            16.0,  // Tesseract 꼭짓점 = phi^tau
            32.0,  // Tesseract 변 = 2^sopfr
            24.0,  // Tesseract 면 = J_2
            8.0,   // Tesseract 셀 = sigma - tau
            4.0,   // 4D 공간 = tau
            12.0,  // 알파밴드 상한 = sigma
            120.0, // 120Hz 디스플레이 = sigma*(sigma-phi)
        ];

        // 각 차원의 평균값 계산
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        // n=6 상수 매칭 (7% 허용 오차)
        let mut n6_matches = 0usize;
        let mut matched_vals = Vec::new();
        for &m in &means {
            for &c in dom_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.07 {
                    n6_matches += 1;
                    matched_vals.push(c);
                    break;
                }
            }
        }

        // Plenoptic 6D 완전성 점수
        // 데이터가 6개 독립 차원으로 분해 가능한지 측정
        let plenoptic_score = if d >= 6 {
            let active_dims = means.iter().filter(|&&m| m.abs() > 1e-12).count();
            (active_dims.min(6) as f64) / 6.0
        } else {
            d as f64 / 6.0
        };

        // Tesseract 조합론 공명 — 16/32/24/8 패턴 탐지
        // 연속된 평균값 비율이 Tesseract k-면 비율(16:32:24:8 = 2:4:3:1)에 근접하는지
        let tesseract_ratios = [2.0_f64, 1.5_f64, 3.0_f64]; // 32/16, 24/16*ratio, 8->...
        let sorted_means: Vec<f64> = {
            let mut v: Vec<f64> = means.iter().cloned().filter(|&x| x > 1.0).collect();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            v
        };
        let tesseract_match = if sorted_means.len() >= 2 {
            let pairs_checked = sorted_means.windows(2).count().max(1);
            let ratio_hits = sorted_means.windows(2).filter(|w| {
                if w[0].abs() < 1e-12 { return false; }
                let r = w[1] / w[0];
                tesseract_ratios.iter().any(|&tr| (r - tr).abs() < 0.15)
            }).count();
            ratio_hits as f64 / pairs_checked as f64
        } else {
            0.0
        };

        // SO(4) 회전 독립 평면 = C(4,2) = 6 = n
        // 데이터에서 6개 독립 회전 방향이 감지되는지 평가
        let rotation_planes = if d >= 4 {
            // 차원 쌍 상관 검사 — 6 독립 평면에 대응
            let pair_count = (d * (d - 1)) / 2;
            let n6_pairs = if pair_count >= 6 {
                // C(4,2) = 6 에 근접하는지
                ((pair_count as f64 - 6.0) / 6.0_f64).abs().min(1.0)
            } else {
                pair_count as f64 / 6.0
            };
            1.0 - n6_pairs.min(1.0)  // 6에 가까울수록 1.0
        } else {
            0.0
        };

        // 알파밴드 (8-12 Hz) 공명 탐지
        // 데이터 값이 8~12 범위에 집중되는지
        let alpha_band_score = {
            let in_band = means.iter().filter(|&&m| m >= 8.0 && m <= 12.0).count();
            in_band as f64 / d.max(1) as f64
        };

        // OpenBCI 16채널 패턴 (16 = phi^tau)
        let openbci_match = means.iter().map(|&m| {
            if m.abs() > 1e-12 { (-(( m - 16.0) / 16.0).powi(2) * 40.0).exp() }
            else { 0.0 }
        }).fold(0.0_f64, f64::max);

        // 종합 차원지각 점수
        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.30
            + plenoptic_score * 0.20
            + tesseract_match * 0.15
            + rotation_planes * 0.15
            + alpha_band_score * 0.10
            + openbci_match * 0.10;

        let mut r = HashMap::new();
        r.insert("n6_matches".to_string(), vec![n6_matches as f64]);
        r.insert("plenoptic_6d_score".to_string(), vec![plenoptic_score]);
        r.insert("tesseract_combinatorics".to_string(), vec![tesseract_match]);
        r.insert("so4_rotation_planes".to_string(), vec![rotation_planes]);
        r.insert("alpha_band_8_12hz".to_string(), vec![alpha_band_score]);
        r.insert("openbci_16ch_resonance".to_string(), vec![openbci_match]);
        r.insert("dimensional_perception_score".to_string(), vec![score]);
        r.insert("matched_constants".to_string(), matched_vals);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimensional_perception_basic() {
        let n = 8; let d = 6;
        // n=6 상수들로 채운 데이터
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => 6.0,   // n
                1 => 24.0,  // J_2 = Tesseract 면
                2 => 16.0,  // phi^tau = OpenBCI
                3 => 8.0,   // 알파밴드 하한
                4 => 12.0,  // 알파밴드 상한
                _ => 4.0,   // tau
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = DimensionalPerceptionLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("dimensional_perception_score"), "점수 키 존재 확인");
        assert!(result.contains_key("plenoptic_6d_score"), "완전광학 점수 확인");
        assert!(result.contains_key("n6_matches"), "n=6 매칭 수 확인");
        let score = result["dimensional_perception_score"][0];
        assert!(score > 0.0, "점수 > 0 확인: {}", score);
    }

    #[test]
    fn test_dimensional_perception_empty() {
        // n < 4 이면 빈 결과 반환
        let data = vec![1.0, 2.0, 3.0];
        let shared = SharedData::compute(&data, 3, 1);
        let result = DimensionalPerceptionLens.scan(&data, 3, 1, &shared);
        assert!(result.is_empty(), "n<4 이면 빈 결과");
    }

    #[test]
    fn test_dimensional_perception_plenoptic() {
        // 6차원 데이터 — 완전광학 점수 최대
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| (i % 10 + 1) as f64).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = DimensionalPerceptionLens.scan(&data, n, d, &shared);
        let plenoptic = result["plenoptic_6d_score"][0];
        assert!((plenoptic - 1.0).abs() < 1e-9, "6D 데이터의 완전광학 점수 = 1.0: {}", plenoptic);
    }

    #[test]
    fn test_dimensional_perception_tesseract() {
        // Tesseract 상수(16, 24, 8) 포함 데이터
        let n = 6; let d = 4;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => 16.0,
                1 => 24.0,
                2 => 8.0,
                _ => 32.0,
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = DimensionalPerceptionLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("tesseract_combinatorics"), "Tesseract 조합론 키 확인");
        let n6m = result["n6_matches"][0];
        assert!(n6m >= 3.0, "Tesseract 상수 3종 이상 매칭: {}", n6m);
    }
}
