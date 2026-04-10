use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 달력 60진법 렌즈 — sexagesimal 시간 체계 n=6 수렴
///
/// 바빌로니아 60진법의 60=sigma·tau·5=12·4·5/... 아니라 60=2²·3·5
/// 하지만 n=6 연결:
///   60 = n·tau·sopfr/2 = 6·4·5/2 아니라 10·n = 10·6
///   360° = 60·n = 60·6 (원의 각도)
///   24시간 = phi·sigma = 2·12 = sigma
///   60분/시 = 10·n
///   60초/분 = 10·n
///   하루 86400초 = tau·sigma·1800 → n·tau·sigma·tau·sigma·tau·...
pub struct CalendarSexagesimalLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;
const SEC60: f64 = 60.0;       // 60초=60분
const HOURS24: f64 = 24.0;     // 24시간 = phi·sigma
const DEG360: f64 = 360.0;     // 360° = 60·6

impl Lens for CalendarSexagesimalLens {
    fn name(&self) -> &str { "CalendarSexagesimalLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        // 1. 60 공명
        let sixty_score = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - SEC60) / SEC60).abs() < 0.07
        }).count() as f64 / d.max(1) as f64;

        // 2. 24시간 공명 (phi·sigma = 2·12)
        let hours_score = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - HOURS24) / HOURS24).abs() < 0.07
        }).count() as f64 / d.max(1) as f64;

        // 3. 360° 공명
        let deg360_score = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - DEG360) / DEG360).abs() < 0.07
        }).count() as f64 / d.max(1) as f64;

        // 4. n=6 직접 공명
        let n6_direct = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - N6) / N6).abs() < 0.07
        }).count() as f64 / d.max(1) as f64;

        // 5. n=6 차원 공명
        let n6_dim = (1.0 - (d as f64 - N6).abs() / N6 * 2.0).max(0.0);

        // 6. 전체 n=6 상수 공명
        let all_targets = [N6, TAU, SIGMA, PHI, SOPFR, SEC60, HOURS24, DEG360];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        let sexa_score = sixty_score  * 0.25
            + hours_score   * 0.20
            + n6_dim        * 0.20
            + n6_direct     * 0.15
            + deg360_score  * 0.10
            + n6_resonance  * 0.10;

        let mut r = HashMap::new();
        r.insert("sixty_score".to_string(),  vec![sixty_score]);
        r.insert("hours_score".to_string(),  vec![hours_score]);
        r.insert("deg360_score".to_string(), vec![deg360_score]);
        r.insert("n6_direct".to_string(),    vec![n6_direct]);
        r.insert("n6_dim".to_string(),       vec![n6_dim]);
        r.insert("n6_resonance".to_string(), vec![n6_resonance]);
        r.insert("sexa_score".to_string(),   vec![sexa_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sexa_기본() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d { 0 => N6, 1 => SEC60, 2 => HOURS24, 3 => DEG360, 4 => TAU, _ => SIGMA }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = CalendarSexagesimalLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("sexa_score"));
        assert!(r["sexa_score"][0] >= 0.0 && r["sexa_score"][0] <= 1.0);
    }

    #[test]
    fn test_sexa_최소입력_거부() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let r = CalendarSexagesimalLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
