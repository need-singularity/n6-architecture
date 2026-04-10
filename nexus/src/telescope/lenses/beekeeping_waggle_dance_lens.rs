use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 꿀벌 8자 춤 렌즈 — waggle dance 방향 정보 n=6 수렴
///
/// 꿀벌의 8자 춤(waggle dance)으로 방향/거리 정보 전달.
/// n=6 연결:
///   1초 waggle = 약 1km 거리 (정보율 = 1초·n km)
///   무리 결정 정족수 = 약 6마리 (n=6 합의)
///   방향 각도 정밀도 = 15° = 360°/24 ≈ tau·phi+7 → sigma-1=11 근방
///   8자 주기 = 1~6초 → [1, n]
///   분봉 정찰 방향 후보 = 6방향 = n
pub struct BeekeepingWaggleDanceLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;
const DEG15: f64 = 15.0;    // 방향 정밀도
const DEG45: f64 = 45.0;    // 8자 기울기 (π/4)

impl Lens for BeekeepingWaggleDanceLens {
    fn name(&self) -> &str { "BeekeepingWaggleDanceLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        // 1. waggle 지속시간 공명 (1~6초 범위)
        let waggle_targets = [1.0, PHI, TAU - 1.0, TAU, SOPFR, N6];
        let waggle_hits = means.iter().filter(|&&m| {
            waggle_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.08)
        }).count();
        let waggle_score = waggle_hits as f64 / d.max(1) as f64;

        // 2. 방향 정보 공명 (15°, 45° 기준)
        let dir_targets = [DEG15, DEG45, N6, SIGMA];
        let dir_hits = means.iter().filter(|&&m| {
            dir_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.08)
        }).count();
        let direction_score = dir_hits as f64 / d.max(1) as f64;

        // 3. 8자 주기성 — 라그-2 자기상관 (8자 = 주기 2)
        let period2_score: f64 = {
            let mut total = 0.0;
            for j in 0..d {
                let col: Vec<f64> = (0..n).map(|i| data[i * d + j]).collect();
                let m = means[j];
                if n < 4 { continue; }
                let ac2: f64 = (2..n).map(|i| (col[i] - m) * (col[i - 2] - m)).sum::<f64>();
                let norm2: f64 = (0..n).map(|i| (col[i] - m).powi(2)).sum::<f64>().max(1e-15);
                total += (ac2 / norm2).clamp(-1.0, 1.0);
            }
            ((total / d as f64 + 1.0) / 2.0).clamp(0.0, 1.0)
        };

        // 4. n=6 합의 정족수 공명
        let n6_dim = (1.0 - (d as f64 - N6).abs() / N6 * 2.0).max(0.0);

        // 5. 전체 n=6 공명
        let all_targets = [N6, TAU, SIGMA, PHI, SOPFR, DEG15];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        let waggle_total = waggle_score    * 0.25
            + direction_score  * 0.20
            + period2_score    * 0.20
            + n6_dim           * 0.20
            + n6_resonance     * 0.15;

        let mut r = HashMap::new();
        r.insert("waggle_score".to_string(),    vec![waggle_score]);
        r.insert("direction_score".to_string(), vec![direction_score]);
        r.insert("period2_score".to_string(),   vec![period2_score]);
        r.insert("n6_dim".to_string(),          vec![n6_dim]);
        r.insert("n6_resonance".to_string(),    vec![n6_resonance]);
        r.insert("waggle_total".to_string(),    vec![waggle_total]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waggle_기본() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d { 0 => N6, 1 => TAU, 2 => PHI, 3 => SOPFR, 4 => DEG15, _ => 1.0 }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = BeekeepingWaggleDanceLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("waggle_total"));
        assert!(r["waggle_total"][0] >= 0.0 && r["waggle_total"][0] <= 1.0);
    }

    #[test]
    fn test_waggle_최소입력_거부() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let r = BeekeepingWaggleDanceLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
