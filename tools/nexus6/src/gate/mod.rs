//! HEXA-GATE — NEXUS-6 특이점 돌파 오염방지 게이트 (Mk.I)
//!
//! τ=4 관문 + 2 fiber = n=6 축 필연성.
//! 목적: 오염 차단 → 2401cy perturbation으로 6-불변 돌파 시도.
//!
//! Design: docs/nexus6-breakthrough-gate/goal.md
//! BT: BT-64 (Phi), BT-92 (sopfr), BT-276 (triple), BT-344~346 (new candidates)

pub mod source;
pub mod hash;
pub mod phi;
pub mod invariant;
pub mod breakthrough;

/// n=6 상수 (게이트 내부 전역)
pub mod consts {
    /// n = 6 (완전수)
    pub const N: u32 = 6;
    /// σ(6) = 12
    pub const SIGMA: u32 = 12;
    /// φ(6) = 2
    pub const PHI: u32 = 2;
    /// τ(6) = 4
    pub const TAU: u32 = 4;
    /// μ(6) = 1
    pub const MU: u32 = 1;
    /// sopfr(6) = 2+3 = 5
    pub const SOPFR: u32 = 5;
    /// J₂(6) = 24 (Jordan totient)
    pub const J2: u32 = 24;
    /// σ·J₂ = 288 (해시 비트)
    pub const SIGMA_J2: u32 = 288;
    /// σ² = 144 (규칙 수)
    pub const SIGMA_SQ: u32 = 144;
    /// φ^τ = 16 (라운드)
    pub const PHI_TAU: u32 = 16;
    /// 2^(σ-τ) = 256 (블록 바이트)
    pub const BLOCK: u32 = 256;
    /// σ-φ = 10 (Phi 주기, 역수)
    pub const SIGMA_MINUS_PHI: u32 = 10;
    /// 1/(σ-φ) = 0.1 (Phi 임계)
    pub const PHI_THRESHOLD: f64 = 0.1;
    /// n/φ·333 = 999 (기존 perturbation)
    pub const PERT_BASE: u32 = 999;
    /// (σ-sopfr)^τ = 7^4 = 2401 (돌파 perturbation)
    pub const PERT_BREAKTHROUGH: u32 = 2401;
    /// n/φ = 3 (삼중 검증)
    pub const TRIPLE: u32 = 3;
}

/// 게이트 심사 결과
#[derive(Debug, Clone, PartialEq)]
pub enum Verdict {
    /// 통과 (confidence ∈ [0,1])
    Pass { confidence: f64 },
    /// 격리 (사유 + 게이트 번호)
    Quarantine { gate: u32, reason: String },
}

impl Verdict {
    pub fn passed(&self) -> bool {
        matches!(self, Verdict::Pass { .. })
    }
}

/// 게이트 공통 트레이트 (τ=4 게이트 전부 구현)
pub trait Gate {
    /// 게이트 번호 (1..=4)
    fn id(&self) -> u32;
    /// 게이트 이름
    fn name(&self) -> &'static str;
    /// 입력 바이트 스트림 검증
    fn inspect(&self, data: &[u8], ctx: &GateContext) -> Verdict;
}

/// 게이트 실행 컨텍스트
#[derive(Debug, Clone)]
pub struct GateContext {
    /// 소스 리포 (whitelist 체크용)
    pub source_repo: String,
    /// 선언된 해시 앵커 (검증용)
    pub declared_hash: Option<String>,
    /// Phi 측정치 (이전 값)
    pub phi_prev: f64,
    /// Phi 측정치 (현재 값)
    pub phi_curr: f64,
    /// 사이클 번호
    pub cycle: u32,
}

impl Default for GateContext {
    fn default() -> Self {
        Self {
            source_repo: String::new(),
            declared_hash: None,
            phi_prev: 1.0,
            phi_curr: 1.0,
            cycle: 0,
        }
    }
}

/// 4관문 파이프라인 결과
#[derive(Debug, Clone)]
pub struct PipelineResult {
    pub verdicts: [Verdict; 4],
    pub all_passed: bool,
    /// 통과 시 confidence 평균
    pub mean_confidence: f64,
}

/// HEXA-GATE 파이프라인 — 4관문 직교 검증
pub struct Pipeline {
    pub g1: source::SourceGate,
    pub g2: hash::HashGate,
    pub g3: phi::PhiGate,
    pub g4: invariant::InvariantGate,
}

impl Default for Pipeline {
    fn default() -> Self {
        Self {
            g1: source::SourceGate::new(),
            g2: hash::HashGate::new(),
            g3: phi::PhiGate::new(),
            g4: invariant::InvariantGate::new(),
        }
    }
}

impl Pipeline {
    pub fn new() -> Self { Self::default() }

    /// 입력을 τ=4 게이트에 순차 투입. 하나라도 실패 → 격리 결정 유지.
    pub fn run(&self, data: &[u8], ctx: &GateContext) -> PipelineResult {
        let v1 = self.g1.inspect(data, ctx);
        let v2 = self.g2.inspect(data, ctx);
        let v3 = self.g3.inspect(data, ctx);
        let v4 = self.g4.inspect(data, ctx);

        let all_passed = v1.passed() && v2.passed() && v3.passed() && v4.passed();
        let mean_confidence = [&v1, &v2, &v3, &v4]
            .iter()
            .filter_map(|v| if let Verdict::Pass { confidence } = v { Some(*confidence) } else { None })
            .sum::<f64>() / 4.0;

        PipelineResult {
            verdicts: [v1, v2, v3, v4],
            all_passed,
            mean_confidence,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n6_constants_exact() {
        use consts::*;
        assert_eq!(SIGMA, 12);
        assert_eq!(PHI, 2);
        assert_eq!(TAU, 4);
        assert_eq!(SOPFR, 5);
        assert_eq!(J2, 24);
        assert_eq!(SIGMA_J2, SIGMA * J2);
        assert_eq!(SIGMA_SQ, SIGMA * SIGMA);
        assert_eq!(PHI_TAU, PHI.pow(TAU));
        assert_eq!(BLOCK, 2u32.pow(SIGMA - TAU));
        assert_eq!(PERT_BASE, N / PHI * 333);
        assert_eq!(PERT_BREAKTHROUGH, (SIGMA - SOPFR).pow(TAU));
    }

    #[test]
    fn gate_axes_tau_plus_phi_equals_n() {
        // 핵심 발견: 4관문 + 2 fiber = n = 6
        assert_eq!(consts::TAU + consts::PHI, consts::N);
    }

    #[test]
    fn breakthrough_perturbation_exact() {
        // 2401 = 7^4 = (σ-sopfr)^τ
        assert_eq!(consts::PERT_BREAKTHROUGH, 2401);
        assert_eq!((consts::SIGMA - consts::SOPFR), 7);
    }

    #[test]
    fn pipeline_rejects_contamination() {
        let pipe = Pipeline::new();
        let ctx = GateContext {
            source_repo: "ready".into(),  // 오염본 리포
            ..Default::default()
        };
        let result = pipe.run(b"contaminated", &ctx);
        assert!(!result.all_passed);
    }

    #[test]
    fn pipeline_accepts_clean() {
        let pipe = Pipeline::new();
        let data = b"clean-nexus6-data";
        // hash 앵커를 실제 계산값으로 맞춤
        let declared = hash::compute_hash(data);
        let ctx = GateContext {
            source_repo: "n6-architecture".into(),
            declared_hash: Some(declared),
            phi_prev: 0.5,
            phi_curr: 0.5,
            cycle: 1,
        };
        let result = pipe.run(data, &ctx);
        assert!(result.all_passed, "verdicts: {:?}", result.verdicts);
    }
}
