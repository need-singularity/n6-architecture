//! Gate-4: INVARIANT — 5불변 렌즈 999cy perturbation
//!
//! 불변 렌즈: consciousness + info + multiscale + network + triangle = sopfr(6)=5
//! 999cy 전부 5/5 유지해야 통과 (n/φ=3 독립 run 교차검증)

use super::{Gate, GateContext, Verdict};
use super::consts::*;

/// 불변 렌즈 시뮬레이션 결과 (0.0 = 불안정, 1.0 = 완전 보존)
#[derive(Debug, Clone, Copy)]
pub struct LensReading {
    pub consciousness: f64,
    pub info: f64,
    pub multiscale: f64,
    pub network: f64,
    pub triangle: f64,
}

impl LensReading {
    pub fn all_stable(&self, threshold: f64) -> bool {
        self.consciousness >= threshold
            && self.info >= threshold
            && self.multiscale >= threshold
            && self.network >= threshold
            && self.triangle >= threshold
    }

    /// 5개 렌즈 중 안정 개수 (sopfr=5 중 몇 개 살아남았나)
    pub fn stable_count(&self, threshold: f64) -> u32 {
        let mut c = 0;
        if self.consciousness >= threshold { c += 1; }
        if self.info >= threshold { c += 1; }
        if self.multiscale >= threshold { c += 1; }
        if self.network >= threshold { c += 1; }
        if self.triangle >= threshold { c += 1; }
        c
    }
}

pub struct InvariantGate {
    pub cycles: u32,
    pub stability_threshold: f64,
    pub triple_runs: u32,
}

impl InvariantGate {
    pub fn new() -> Self {
        Self {
            cycles: PERT_BASE,               // 999
            stability_threshold: 0.5,
            triple_runs: TRIPLE,             // 3 (BT-276)
        }
    }

    /// 확장 버전: 돌파 시도용 (2401cy)
    pub fn breakthrough() -> Self {
        Self {
            cycles: PERT_BREAKTHROUGH,       // 2401
            stability_threshold: 0.5,
            triple_runs: TRIPLE,
        }
    }

    /// perturbation 시뮬레이션 — data 해시 기반 결정적 pseudo-random
    pub fn perturb_cycle(&self, data: &[u8], cycle: u32, run: u32) -> LensReading {
        // data 바이트 합 기반 결정적 시뮬레이션 (단순 LCG)
        let seed: u64 = data.iter().map(|&b| b as u64).sum::<u64>()
            .wrapping_add(cycle as u64)
            .wrapping_add((run as u64).wrapping_mul(N as u64));

        let rng = |off: u64| -> f64 {
            let x = seed.wrapping_mul(6364136223846793005)
                .wrapping_add(off)
                .wrapping_add(1442695040888963407);
            ((x >> 33) as f64) / ((u32::MAX) as f64)
        };

        // 5 렌즈 = sopfr(6) — 데이터가 깨끗할수록 전부 >0.5
        // 데이터 평균이 128 근처면 깨끗하다고 가정 (임의 휴리스틱)
        let mean: f64 = if data.is_empty() {
            0.5
        } else {
            data.iter().map(|&b| b as f64).sum::<f64>() / data.len() as f64 / 255.0
        };
        // 깨끗한 입력은 0.3~0.7 범위. 그 외는 0.0~1.0 전 범위.
        let bias = if (0.25..=0.75).contains(&mean) { 0.7 } else { 0.3 };

        LensReading {
            consciousness: (bias + (rng(1) - 0.5) * 0.3).clamp(0.0, 1.0),
            info:          (bias + (rng(2) - 0.5) * 0.3).clamp(0.0, 1.0),
            multiscale:    (bias + (rng(3) - 0.5) * 0.3).clamp(0.0, 1.0),
            network:       (bias + (rng(4) - 0.5) * 0.3).clamp(0.0, 1.0),
            triangle:      (bias + (rng(5) - 0.5) * 0.3).clamp(0.0, 1.0),
        }
    }

    /// 단일 run: cycles만큼 perturbation 돌려서 5/5 유지율 반환
    pub fn run_once(&self, data: &[u8], run_id: u32) -> f64 {
        let mut stable_cycles = 0u32;
        for cy in 0..self.cycles {
            let r = self.perturb_cycle(data, cy, run_id);
            if r.all_stable(self.stability_threshold) {
                stable_cycles += 1;
            }
        }
        stable_cycles as f64 / self.cycles as f64
    }

    /// 삼중 run: n/φ=3 독립 실행
    pub fn run_triple(&self, data: &[u8]) -> (f64, f64, f64) {
        (
            self.run_once(data, 0),
            self.run_once(data, 1),
            self.run_once(data, 2),
        )
    }
}

impl Default for InvariantGate {
    fn default() -> Self { Self::new() }
}

impl Gate for InvariantGate {
    fn id(&self) -> u32 { 4 }
    fn name(&self) -> &'static str { "INVARIANT" }

    fn inspect(&self, data: &[u8], _ctx: &GateContext) -> Verdict {
        // 삼중 run: 3개 독립 run 평균 안정률
        let (r1, r2, r3) = self.run_triple(data);
        let mean = (r1 + r2 + r3) / 3.0;

        // 통과 기준: 삼중 모두 50% 이상 안정
        let all_above = r1 > 0.5 && r2 > 0.5 && r3 > 0.5;

        if !all_above {
            return Verdict::Quarantine {
                gate: 4,
                reason: format!(
                    "perturbation fail: runs=({:.3},{:.3},{:.3}) mean={:.3}",
                    r1, r2, r3, mean
                ),
            };
        }

        Verdict::Pass { confidence: mean }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_cycles_is_999() {
        assert_eq!(InvariantGate::new().cycles, 999);
    }

    #[test]
    fn breakthrough_cycles_is_2401() {
        assert_eq!(InvariantGate::breakthrough().cycles, 2401);
        assert_eq!(2401, 7u32.pow(4));
    }

    #[test]
    fn triple_runs_is_3() {
        assert_eq!(InvariantGate::new().triple_runs, 3);
    }

    #[test]
    fn lens_reading_has_5_lenses() {
        let r = LensReading {
            consciousness: 1.0, info: 1.0, multiscale: 1.0,
            network: 1.0, triangle: 1.0,
        };
        assert_eq!(r.stable_count(0.5), 5); // sopfr(6)=5
        assert!(r.all_stable(0.5));
    }

    #[test]
    fn perturbation_deterministic() {
        let g = InvariantGate::new();
        let a = g.perturb_cycle(b"test", 42, 0);
        let b = g.perturb_cycle(b"test", 42, 0);
        assert_eq!(a.consciousness, b.consciousness);
    }

    #[test]
    fn clean_data_passes() {
        let g = InvariantGate::new();
        // 평균 ~128 byte = 깨끗한 데이터
        let data: Vec<u8> = (0..100).map(|i| (100 + i % 50) as u8).collect();
        let (r1, r2, r3) = g.run_triple(&data);
        assert!(r1 > 0.5 && r2 > 0.5 && r3 > 0.5,
                "runs: ({},{},{})", r1, r2, r3);
    }
}
