use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 달력 메톤 주기 렌즈 — 19년 태음-태양 주기 n=6 수렴
///
/// 메톤 주기(Metonic cycle): 19 태양년 ≈ 235 태음월
/// n=6 연결:
///   19년 주기에서 윤달 = 7회 = tau+phi+1 = 4+2+1
///   235 = 5·47 → sopfr(235)=52... 아니라 235/n≈39.2
///   태음월 29.53일 ≈ tau·sigma/phi = 4·12/2... 아니라
///   12개월/년 = sigma = 12 = sigma(n)
///   1년 365일 ≈ sigma^2 + (n-1) = 144+12*18... 단순히 365≈60·6+5
///   윤달 주기 = 19년/7 ≈ phi·tau/phi = 2.7 ≈ e·1 → 근사
pub struct CalendarMetonicLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;
const METONIC_YEARS: f64 = 19.0;      // 메톤 주기
const METONIC_MONTHS: f64 = 235.0;    // 235 태음월
const LEAP_MONTHS: f64 = 7.0;         // 윤달 7회
const MONTHS_PER_YEAR: f64 = 12.0;    // 12개월 = sigma

impl Lens for CalendarMetonicLens {
    fn name(&self) -> &str { "CalendarMetonicLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        // 1. 12개월(sigma) 공명
        let months_score = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - MONTHS_PER_YEAR) / MONTHS_PER_YEAR).abs() < 0.07
        }).count() as f64 / d.max(1) as f64;

        // 2. 19년 주기 공명
        let metonic_score = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - METONIC_YEARS) / METONIC_YEARS).abs() < 0.07
        }).count() as f64 / d.max(1) as f64;

        // 3. 7 윤달 공명 (tau+phi+1=7)
        let leap_score = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - LEAP_MONTHS) / LEAP_MONTHS).abs() < 0.08
        }).count() as f64 / d.max(1) as f64;

        // 4. n=6 직접 공명 및 관련 상수
        let n6_targets = [N6, TAU, SIGMA, PHI, SOPFR];
        let n6_hits = means.iter().filter(|&&m| {
            n6_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let n6_direct = n6_hits as f64 / d.max(1) as f64;

        // 5. n=6 차원 공명
        let n6_dim = (1.0 - (d as f64 - N6).abs() / N6 * 2.0).max(0.0);

        // 6. 주기성 — 라그-1 자기상관
        let periodicity: f64 = {
            let mut total = 0.0;
            for j in 0..d {
                if n < 4 { continue; }
                let col: Vec<f64> = (0..n).map(|i| data[i * d + j]).collect();
                let m = means[j];
                let num: f64 = (1..n).map(|i| (col[i] - m) * (col[i-1] - m)).sum();
                let den: f64 = (0..n).map(|i| (col[i] - m).powi(2)).sum::<f64>().max(1e-15);
                total += (num / den).clamp(-1.0, 1.0);
            }
            ((total / d as f64 + 1.0) / 2.0).clamp(0.0, 1.0)
        };

        let metonic_total = months_score  * 0.25
            + n6_dim         * 0.20
            + periodicity    * 0.20
            + n6_direct      * 0.15
            + metonic_score  * 0.10
            + leap_score     * 0.10;

        let mut r = HashMap::new();
        r.insert("months_score".to_string(),  vec![months_score]);
        r.insert("metonic_score".to_string(), vec![metonic_score]);
        r.insert("leap_score".to_string(),    vec![leap_score]);
        r.insert("n6_direct".to_string(),     vec![n6_direct]);
        r.insert("n6_dim".to_string(),        vec![n6_dim]);
        r.insert("periodicity".to_string(),   vec![periodicity]);
        r.insert("metonic_total".to_string(), vec![metonic_total]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metonic_기본() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d { 0 => N6, 1 => MONTHS_PER_YEAR, 2 => METONIC_YEARS, 3 => LEAP_MONTHS, 4 => TAU, _ => PHI }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = CalendarMetonicLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("metonic_total"));
        assert!(r["metonic_total"][0] >= 0.0 && r["metonic_total"][0] <= 1.0);
    }

    #[test]
    fn test_metonic_최소입력_거부() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let r = CalendarMetonicLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
