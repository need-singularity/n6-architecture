use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 벌집 육각형 렌즈 — 벌집 구조의 n=6 최적화 스캐너
///
/// 벌집(honeycomb)은 정육각형 단위 격자 — 면적 대비 둘레가 최소.
/// n=6 연결:
///   정육각형 꼭짓점 수 = 6 = n
///   최근접 이웃 수 = 6 = n
///   벌집 각도 = 120° = sigma·tau·phi = 12·4·2·... 아니라 360°/3 = 2·60°
///   왁스 세포 벽 두께 비율 ≈ 1/phi = 0.5
///   벌 여왕 산란 세포 군집 = 6계층 = n
pub struct BeekeepingHoneycombLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const DEG120: f64 = 120.0;   // 벌집 내각
const DEG60: f64 = 60.0;     // 벌집 격자 각

impl Lens for BeekeepingHoneycombLens {
    fn name(&self) -> &str { "BeekeepingHoneycombLens" }
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

        // 1. 6각형 구조 공명
        let hex_targets = [N6, DEG60, DEG120, 0.5, PHI];
        let hex_hits = means.iter().filter(|&&m| {
            hex_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.08)
        }).count();
        let hex_score = hex_hits as f64 / d.max(1) as f64;

        // 2. 최적 포장 밀도 공명 — 육각 격자 π√3/6 ≈ 0.9069
        let hex_packing: f64 = std::f64::consts::PI * 3.0_f64.sqrt() / 6.0;
        let packing_score = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - hex_packing) / hex_packing).abs() < 0.08
        }).count() as f64 / d.max(1) as f64;

        // 3. 6-주기 패턴 점수 (벌집의 주기성)
        let period6: f64 = {
            let seg = n / 6;
            if seg == 0 { 0.0 } else {
                let mut total = 0.0;
                for j in 0..d {
                    let seg_means: Vec<f64> = (0..6).map(|s| {
                        let start = s * seg;
                        let end = (start + seg).min(n);
                        (start..end).map(|i| data[i * d + j]).sum::<f64>() / (end - start) as f64
                    }).collect();
                    let mean_s = seg_means.iter().sum::<f64>() / 6.0;
                    let var_s = seg_means.iter().map(|&m| (m - mean_s).powi(2)).sum::<f64>() / 6.0;
                    total += (1.0 - var_s.sqrt() / (mean_s.abs().max(1e-12))).max(0.0);
                }
                total / d as f64
            }
        };

        // 4. n=6 차원 공명
        let n6_dim = (1.0 - (d as f64 - N6).abs() / N6 * 2.0).max(0.0);

        // 5. 균일성 (벌집 세포 크기 균일)
        let mean_std = stds.iter().sum::<f64>() / d.max(1) as f64;
        let mean_val = means.iter().map(|m| m.abs()).sum::<f64>() / d.max(1) as f64;
        let cell_uniformity = (1.0 - mean_std / mean_val.max(1e-12)).clamp(0.0, 1.0);

        // 6. 전체 n=6 공명
        let all_targets = [N6, TAU, SIGMA, PHI, DEG60, hex_packing];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        let honeycomb_score = hex_score      * 0.25
            + period6         * 0.20
            + n6_dim          * 0.20
            + cell_uniformity * 0.15
            + packing_score   * 0.10
            + n6_resonance    * 0.10;

        let mut r = HashMap::new();
        r.insert("hex_score".to_string(),        vec![hex_score]);
        r.insert("packing_score".to_string(),    vec![packing_score]);
        r.insert("period6".to_string(),          vec![period6]);
        r.insert("n6_dim".to_string(),           vec![n6_dim]);
        r.insert("cell_uniformity".to_string(),  vec![cell_uniformity]);
        r.insert("n6_resonance".to_string(),     vec![n6_resonance]);
        r.insert("honeycomb_score".to_string(),  vec![honeycomb_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_honeycomb_기본() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d { 0 => N6, 1 => DEG60, 2 => DEG120, 3 => PHI, 4 => TAU, _ => 0.5 }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = BeekeepingHoneycombLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("honeycomb_score"));
        assert!(r["honeycomb_score"][0] >= 0.0 && r["honeycomb_score"][0] <= 1.0);
    }

    #[test]
    fn test_honeycomb_n6_dim() {
        let n = 8; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| i as f64 + 1.0).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = BeekeepingHoneycombLens.scan(&data, n, d, &shared);
        assert!((r["n6_dim"][0] - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_honeycomb_최소입력_거부() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let r = BeekeepingHoneycombLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
