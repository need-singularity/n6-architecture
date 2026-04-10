use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 군집 로봇 렌즈 — 분산 협조 로봇 n=6 수렴 스캐너
///
/// 군집 로봇(swarm robotics) 알고리즘의 n=6 연결:
///   PSO 이웃 크기 = 6 = n (정육각형 격자)
///   최소 군집 합의 = tau = 4 (쿼럼)
///   정보 전파 홉 수 = 6 = n
///   로봇 간 통신 반경 = sigma/tau = 3체 거리
///   6자유도 이동 = n
///   Boids 규칙 수 = 3 = tau-1
pub struct SwarmRoboticsLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;
const QUORUM: f64 = 4.0;      // 합의 쿼럼 = tau
const BOIDS_RULES: f64 = 3.0; // Boids 규칙 = tau-1
const COMM_RADIUS: f64 = 3.0; // 통신 반경 단위

impl Lens for SwarmRoboticsLens {
    fn name(&self) -> &str { "SwarmRoboticsLens" }
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

        // 1. n=6 이웃/정보홉 공명
        let swarm_targets = [N6, QUORUM, BOIDS_RULES, COMM_RADIUS];
        let swarm_hits = means.iter().filter(|&&m| {
            swarm_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.08)
        }).count();
        let swarm_score = swarm_hits as f64 / d.max(1) as f64;

        // 2. 군집 응집도 — 평균간 분산으로 측정 (낮을수록 응집)
        let mean_val = means.iter().map(|m| m.abs()).sum::<f64>() / d.max(1) as f64;
        let mean_std = stds.iter().sum::<f64>() / d.max(1) as f64;
        let cohesion = (1.0 - mean_std / mean_val.max(1e-12)).clamp(0.0, 1.0);

        // 3. 공간 분리도 — 분산이 너무 작으면 충돌 위험
        let separation = (mean_std / mean_val.max(1e-12)).min(1.0);

        // 4. 정렬 — 라그-1 자기상관 (이웃 방향 정렬)
        let alignment: f64 = {
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
        let all_targets = [N6, TAU, SIGMA, PHI, SOPFR, QUORUM, BOIDS_RULES];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        let swarm_total = swarm_score  * 0.25
            + cohesion     * 0.20
            + alignment    * 0.20
            + n6_dim       * 0.15
            + separation   * 0.10
            + n6_resonance * 0.10;

        let mut r = HashMap::new();
        r.insert("swarm_score".to_string(),   vec![swarm_score]);
        r.insert("cohesion".to_string(),      vec![cohesion]);
        r.insert("separation".to_string(),    vec![separation]);
        r.insert("alignment".to_string(),     vec![alignment]);
        r.insert("n6_dim".to_string(),        vec![n6_dim]);
        r.insert("n6_resonance".to_string(),  vec![n6_resonance]);
        r.insert("swarm_total".to_string(),   vec![swarm_total]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swarm_기본() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d { 0 => N6, 1 => QUORUM, 2 => BOIDS_RULES, 3 => TAU, 4 => PHI, _ => SIGMA }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = SwarmRoboticsLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("swarm_total"));
        assert!(r["swarm_total"][0] >= 0.0 && r["swarm_total"][0] <= 1.0);
    }

    #[test]
    fn test_swarm_최소입력_거부() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let r = SwarmRoboticsLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
