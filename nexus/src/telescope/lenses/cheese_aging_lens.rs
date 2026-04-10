use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 치즈 숙성 렌즈 — 발효/숙성 파라미터 n=6 수렴
///
/// 치즈 숙성(aging)의 n=6 연결:
///   주요 숙성 단계 = 3~6 → [tau-1, n]
///   스위스 치즈 기포 생성 = CO₂ → 기포 크기 n 분포
///   파르미지아노 숙성 기간 = 12~36개월 → [sigma, sigma·tau-tau]
///   유산균 최적 온도 = 37°C ≈ sigma·tau/phi... = 6·6+1 = 37
///   pH 숙성 커브: 최적 pH ≈ 6 = n (소프트 치즈)
///   린드버그 규칙: 수분/단백질 = 2/1 = phi
pub struct CheeseAgingLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;
const PH_OPT: f64 = 6.0;          // 최적 pH = n
const TEMP_LAB: f64 = 37.0;       // 유산균 최적 온도 = n²+1
const AGING_MIN_MO: f64 = 12.0;   // 최소 숙성 = sigma 개월

impl Lens for CheeseAgingLens {
    fn name(&self) -> &str { "CheeseAgingLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let stds: Vec<f64> = (0..d).map(|j| {
            let m = means[j];
            let var = (0..n).map(|i| (data[i * d + j] - m).powi(2)).sum::<f64>() / n as f64;
            var.sqrt()
        }).collect();

        // 1. 최적 pH=6 공명
        let ph_score = means.iter().filter(|&&m| {
            [PH_OPT, N6].iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count() as f64 / d.max(1) as f64;

        // 2. 숙성 기간 sigma 공명
        let aging_score = means.iter().filter(|&&m| {
            [AGING_MIN_MO, SIGMA, N6 * PHI].iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count() as f64 / d.max(1) as f64;

        // 3. 지수적 변화 감지 (숙성 = 느린 발효)
        let monotone_change: f64 = {
            let mut total = 0.0;
            for j in 0..d {
                let col: Vec<f64> = (0..n).map(|i| data[i * d + j]).collect();
                let change = col.windows(2)
                    .map(|w| (w[1] - w[0]).abs())
                    .sum::<f64>() / (n - 1) as f64;
                total += change;
            }
            let mean_change = total / d as f64;
            // 변화율이 작고 꾸준하면 좋은 숙성
            (1.0 - (mean_change / means.iter().map(|m| m.abs()).sum::<f64>() * d as f64).min(1.0)).max(0.0)
        };

        // 4. n=6 상수 공명
        let all_targets = [N6, TAU, SIGMA, PHI, SOPFR, PH_OPT];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        // 5. n=6 차원 공명
        let n6_dim = (1.0 - (d as f64 - N6).abs() / N6 * 2.0).max(0.0);

        // 6. 균일성 (좋은 치즈 = 균일한 숙성)
        let mean_std = stds.iter().sum::<f64>() / d.max(1) as f64;
        let mean_val = means.iter().map(|m| m.abs()).sum::<f64>() / d.max(1) as f64;
        let uniformity = (1.0 - mean_std / mean_val.max(1e-12)).clamp(0.0, 1.0);

        let cheese_score = ph_score      * 0.20
            + aging_score    * 0.20
            + n6_dim         * 0.20
            + uniformity     * 0.20
            + n6_resonance   * 0.10
            + monotone_change * 0.10;

        let mut r = HashMap::new();
        r.insert("ph_score".to_string(),       vec![ph_score]);
        r.insert("aging_score".to_string(),    vec![aging_score]);
        r.insert("monotone_change".to_string(),vec![monotone_change]);
        r.insert("n6_dim".to_string(),         vec![n6_dim]);
        r.insert("uniformity".to_string(),     vec![uniformity]);
        r.insert("n6_resonance".to_string(),   vec![n6_resonance]);
        r.insert("cheese_score".to_string(),   vec![cheese_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cheese_기본() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d { 0 => N6, 1 => SIGMA, 2 => PH_OPT, 3 => TAU, 4 => PHI, _ => SOPFR }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = CheeseAgingLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("cheese_score"));
        assert!(r["cheese_score"][0] >= 0.0 && r["cheese_score"][0] <= 1.0);
    }

    #[test]
    fn test_cheese_최소입력_거부() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let r = CheeseAgingLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
