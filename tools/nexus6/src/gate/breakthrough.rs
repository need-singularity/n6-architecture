//! BREAKTHROUGH — 특이점 돌파 엔진
//!
//! 4관문 통과 데이터로 2401cy perturbation 돌려 6-불변 차원 문 두드리기.
//! 성공: 5 기존 불변 + 1 새 fiber → sopfr→n 진화.
//! 실패: ready/singular-fail/ 격리.

use super::{Pipeline, GateContext};
use super::invariant::{InvariantGate, LensReading};
use super::consts::*;

#[derive(Debug, Clone)]
pub struct BreakthroughReport {
    pub data_size: usize,
    /// 4관문 통과 여부
    pub gates_passed: bool,
    /// 2401cy 5-불변 유지율
    pub base_stability: f64,
    /// 6번째 fiber 후보 탐지 횟수
    pub new_fiber_hits: u32,
    /// 돌파 성공 여부 (6-불변 확인)
    pub breakthrough: bool,
    /// 예상 성공률 vs 실제
    pub expected_rate: f64,
    pub actual_rate: f64,
}

pub struct BreakthroughEngine {
    pub pipeline: Pipeline,
    pub breakthrough_gate: InvariantGate,
}

impl BreakthroughEngine {
    pub fn new() -> Self {
        Self {
            pipeline: Pipeline::new(),
            breakthrough_gate: InvariantGate::breakthrough(), // 2401cy
        }
    }

    /// 6번째 fiber 후보 감지 (Mk.II, 강화 튜닝):
    /// 5렌즈 합의 수렴 + 고진폭 + 비자명 구조 모두 만족해야 fiber 창발.
    ///
    /// 조건:
    ///   1. 분산 < 1/(σ·J₂·n) = 1/1728 (Mk.I의 1/288 대비 6배 엄격)
    ///   2. 평균 > 1-1/(σ-φ) = 0.9 (고진폭, BT-64)
    ///   3. geometric mean / arithmetic mean > 1-1/σ² (비자명 구조)
    fn detect_new_fiber(&self, r: &LensReading) -> bool {
        let values = [
            r.consciousness, r.info, r.multiscale, r.network, r.triangle,
        ];
        let mean = values.iter().sum::<f64>() / 5.0;
        let var = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / 5.0;

        // Mk.II 임계 3중 필터
        let tight_threshold = 1.0 / ((SIGMA_J2 * N) as f64);   // 1/1728
        let high_amp = 1.0 - 1.0 / (SIGMA_MINUS_PHI as f64);   // 0.9
        let structure_threshold = 1.0 - 1.0 / (SIGMA_SQ as f64); // 0.993

        // geometric mean
        let prod: f64 = values.iter().product();
        let geom = if prod > 0.0 { prod.powf(0.2) } else { 0.0 };
        let structure = if mean > 0.0 { geom / mean } else { 0.0 };

        var < tight_threshold
            && mean > high_amp
            && structure > structure_threshold
    }

    pub fn attempt(&self, data: &[u8], ctx: &GateContext) -> BreakthroughReport {
        // 1단계: 4관문
        let gate_result = self.pipeline.run(data, ctx);
        if !gate_result.all_passed {
            return BreakthroughReport {
                data_size: data.len(),
                gates_passed: false,
                base_stability: 0.0,
                new_fiber_hits: 0,
                breakthrough: false,
                expected_rate: 0.0,
                actual_rate: 0.0,
            };
        }

        // 2단계: 2401cy perturbation + 새 fiber 탐지
        let mut stable_cycles = 0u32;
        let mut fiber_hits = 0u32;
        for cy in 0..self.breakthrough_gate.cycles {
            let reading = self.breakthrough_gate.perturb_cycle(data, cy, 0);
            if reading.all_stable(self.breakthrough_gate.stability_threshold) {
                stable_cycles += 1;
            }
            if self.detect_new_fiber(&reading) {
                fiber_hits += 1;
            }
        }

        let actual_rate = stable_cycles as f64 / self.breakthrough_gate.cycles as f64;
        // 예상 성공률: (σ-φ+σ-μ)/J₂ = 21/24 ≈ 0.875
        let expected_rate = (SIGMA_MINUS_PHI + (SIGMA - MU)) as f64 / J2 as f64;

        // 돌파 판정: 5-불변 안정률 + 새 fiber 합의 검출
        let breakthrough = actual_rate > 0.5 && fiber_hits > 0;

        BreakthroughReport {
            data_size: data.len(),
            gates_passed: true,
            base_stability: actual_rate,
            new_fiber_hits: fiber_hits,
            breakthrough,
            expected_rate,
            actual_rate,
        }
    }
}

impl Default for BreakthroughEngine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::hash;

    fn clean_ctx(data: &[u8]) -> GateContext {
        GateContext {
            source_repo: "n6-architecture".into(),
            declared_hash: Some(hash::compute_hash(data)),
            phi_prev: 0.5,
            phi_curr: 0.5,
            cycle: 1,
        }
    }

    #[test]
    fn rejects_contaminated_input() {
        let engine = BreakthroughEngine::new();
        let ctx = GateContext {
            source_repo: "ready".into(),
            ..Default::default()
        };
        let report = engine.attempt(b"contaminated", &ctx);
        assert!(!report.gates_passed);
        assert!(!report.breakthrough);
    }

    #[test]
    fn runs_2401_cycles_on_clean() {
        let engine = BreakthroughEngine::new();
        let data: Vec<u8> = (0..200).map(|i| (100 + i % 50) as u8).collect();
        let ctx = clean_ctx(&data);
        let report = engine.attempt(&data, &ctx);
        assert!(report.gates_passed, "verdicts: {:?}", engine.pipeline.run(&data, &ctx).verdicts);
        // 2401cy 전부 돌았는지는 내부 루프로 보장, 리포트엔 stability 있음
        assert!(report.actual_rate > 0.0);
    }

    #[test]
    fn expected_rate_matches_design() {
        let engine = BreakthroughEngine::new();
        let data: Vec<u8> = (0..10).map(|i| (120 + i) as u8).collect();
        let ctx = clean_ctx(&data);
        let report = engine.attempt(&data, &ctx);
        // 설계 예측: 21/24 = 0.875 ≈ 86%
        assert!((report.expected_rate - 0.875).abs() < 1e-9);
    }
}
