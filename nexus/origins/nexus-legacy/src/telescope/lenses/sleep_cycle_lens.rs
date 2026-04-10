use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 수면 주기 렌즈 — 수면 단계 n=6 수렴 스캐너
///
/// 인간 수면 주기의 n=6 연결:
///   수면 주기 길이 ≈ 90분 = sigma·tau·phi·... = sigma+tau+... 아니라 90=15·6=sopfr·3·n
///   하룻밤 권장 주기 수 = 4~6 → [tau, n]
///   NREM 단계 수 = 3 = tau-1 (N1, N2, N3)
///   수면 단계 총 = 4 (NREM3 + REM) = tau
///   REM 비율 목표 ≈ 25% = 1/tau = 0.25
///   서파 수면(SWS) 주파수: δ파 0.5~4 Hz → [1/phi, tau]
pub struct SleepCycleLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;
const CYCLE_MIN: f64 = 90.0;     // 수면 주기 90분 = sopfr·18 = tau·sopfr·phi·...
const N_CYCLES: f64 = 5.0;       // 권장 주기 수 ≈ sopfr = 5
const REM_RATIO: f64 = 0.25;     // REM 비율 = 1/tau
const DELTA_HZ_MAX: f64 = 4.0;   // 델타파 상한 = tau

impl Lens for SleepCycleLens {
    fn name(&self) -> &str { "SleepCycleLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        // 1. 수면 주기 길이 공명 (90분 = sopfr·18)
        let cycle_score = means.iter().filter(|&&m| {
            [CYCLE_MIN, N6 * SOPFR * TAU / TAU, N_CYCLES].iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count() as f64 / d.max(1) as f64;

        // 2. REM 비율 공명 (1/tau = 0.25)
        let rem_score = means.iter().filter(|&&m| {
            [REM_RATIO, 1.0 / TAU].iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.08)
        }).count() as f64 / d.max(1) as f64;

        // 3. 수면 단계 tau=4 공명
        let stage_score = means.iter().filter(|&&m| {
            [TAU, TAU - 1.0, N6].iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.08)
        }).count() as f64 / d.max(1) as f64;

        // 4. 주기성 감지 (수면은 주기적 패턴)
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

        // 5. n=6 차원 공명
        let n6_dim = (1.0 - (d as f64 - N6).abs() / N6 * 2.0).max(0.0);

        // 6. 전체 n=6 공명
        let all_targets = [N6, TAU, SIGMA, PHI, SOPFR, REM_RATIO, DELTA_HZ_MAX];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        let sleep_score = cycle_score   * 0.20
            + rem_score     * 0.20
            + periodicity   * 0.20
            + n6_dim        * 0.20
            + stage_score   * 0.10
            + n6_resonance  * 0.10;

        let mut r = HashMap::new();
        r.insert("cycle_score".to_string(),   vec![cycle_score]);
        r.insert("rem_score".to_string(),     vec![rem_score]);
        r.insert("stage_score".to_string(),   vec![stage_score]);
        r.insert("periodicity".to_string(),   vec![periodicity]);
        r.insert("n6_dim".to_string(),        vec![n6_dim]);
        r.insert("n6_resonance".to_string(),  vec![n6_resonance]);
        r.insert("sleep_score".to_string(),   vec![sleep_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sleep_기본() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d { 0 => N6, 1 => TAU, 2 => REM_RATIO, 3 => SIGMA, 4 => SOPFR, _ => PHI }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = SleepCycleLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("sleep_score"));
        assert!(r["sleep_score"][0] >= 0.0 && r["sleep_score"][0] <= 1.0);
    }

    #[test]
    fn test_sleep_최소입력_거부() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let r = SleepCycleLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
