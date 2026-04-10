use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 눈결정 육각 대칭 렌즈 — 6회 회전대칭 수렴 스캐너
///
/// 눈결정의 6회 대칭(C6v)은 물 분자 수소결합 각도(104.5°≈180°/tau+phi)에서 비롯.
/// n=6 상수 연결:
///   6회 대칭축 = n=6
///   결정 격자 각도 = 60° = 360°/n
///   격자 벡터 수 = 6 = n (정육각형 최근접 이웃)
///   눈결정 수지상 = 6가지 주가지 = n
pub struct SnowflakeHexagonalSymmetryLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const DEG60: f64 = 60.0;       // 60° = 360°/6
const SQRT3_2: f64 = 0.866_025_403_784_439; // cos(30°) = √3/2

impl Lens for SnowflakeHexagonalSymmetryLens {
    fn name(&self) -> &str { "SnowflakeHexagonalSymmetryLens" }
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

        // 1. 6회 대칭 점수 — 데이터를 n개 구간으로 나눠 6등분 주기성 검사
        let period6_score = {
            let seg = n / 6;
            if seg == 0 { 0.0 } else {
                let mut score = 0.0;
                let mut count = 0;
                for j in 0..d {
                    let col: Vec<f64> = (0..n).map(|i| data[i * d + j]).collect();
                    // 6 세그먼트 평균
                    let seg_means: Vec<f64> = (0..6).map(|s| {
                        let start = s * seg;
                        let end = (start + seg).min(n);
                        col[start..end].iter().sum::<f64>() / (end - start) as f64
                    }).collect();
                    let overall_mean = seg_means.iter().sum::<f64>() / 6.0;
                    let var: f64 = seg_means.iter().map(|&m| (m - overall_mean).powi(2)).sum::<f64>() / 6.0;
                    // 낮은 분산 = 높은 대칭성
                    let sym = (1.0 - var.sqrt() / (overall_mean.abs().max(1e-12))).max(0.0);
                    score += sym;
                    count += 1;
                }
                if count > 0 { score / count as f64 } else { 0.0 }
            }
        };

        // 2. 60° 격자 공명 (cos60=0.5, cos30=√3/2)
        let lattice_targets = [DEG60, SQRT3_2, 0.5, N6, TAU, SIGMA];
        let lattice_hits = means.iter().filter(|&&m| {
            lattice_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.08)
        }).count();
        let lattice_score = lattice_hits as f64 / d.max(1) as f64;

        // 3. n=6 차원 공명
        let n6_dim = (1.0 - (d as f64 - N6).abs() / N6 * 2.0).max(0.0);

        // 4. 균일성 (눈결정 균일한 성장)
        let mean_std = stds.iter().sum::<f64>() / d.max(1) as f64;
        let mean_val = means.iter().map(|m| m.abs()).sum::<f64>() / d.max(1) as f64;
        let uniformity = (1.0 - mean_std / mean_val.max(1e-12)).clamp(0.0, 1.0);

        // 5. n=6 상수 전체 공명
        let all_targets = [N6, TAU, SIGMA, PHI, DEG60, SQRT3_2, 0.5];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        let snowflake_score = period6_score  * 0.30
            + lattice_score   * 0.20
            + n6_dim          * 0.20
            + uniformity      * 0.15
            + n6_resonance    * 0.15;

        let mut r = HashMap::new();
        r.insert("period6_score".to_string(),   vec![period6_score]);
        r.insert("lattice_score".to_string(),   vec![lattice_score]);
        r.insert("n6_dim".to_string(),          vec![n6_dim]);
        r.insert("uniformity".to_string(),      vec![uniformity]);
        r.insert("n6_resonance".to_string(),    vec![n6_resonance]);
        r.insert("lattice_hits".to_string(),    vec![lattice_hits as f64]);
        r.insert("snowflake_score".to_string(), vec![snowflake_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snowflake_hex_기본() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| (i as f64 * std::f64::consts::PI / 3.0).cos()).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = SnowflakeHexagonalSymmetryLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("snowflake_score"));
        assert!(r["snowflake_score"][0] >= 0.0 && r["snowflake_score"][0] <= 1.0);
    }

    #[test]
    fn test_snowflake_n6_dim() {
        let n = 8; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| (i as f64).sin()).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = SnowflakeHexagonalSymmetryLens.scan(&data, n, d, &shared);
        assert!((r["n6_dim"][0] - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_snowflake_최소입력_거부() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let r = SnowflakeHexagonalSymmetryLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
