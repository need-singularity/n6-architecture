//! Telescope scan engine with 1013 lenses across 22 core types.
pub mod accel_lenses_a;
pub mod accel_lenses_b;
pub mod accel_lenses_c;
pub mod accel_lenses_d;
pub mod anima_lenses;
pub mod consensus;
pub mod core_lenses;
pub mod cross_lenses;
pub mod domain_combos;
pub mod lens_trait;
pub mod lenses;
pub mod n6_lenses;
pub mod physics_deep_lenses;
pub mod quantum_lenses;
pub mod registry;
pub mod sedi_lenses;
pub mod shared_data;
pub mod tecs_lenses;
pub mod tier;

use std::collections::HashMap;
use std::panic;

use lens_trait::{Lens, LensResult};
use lenses::{
    BarrierLens, BoundaryLens, CausalLens, ChaosLens, CompassLens, ConsciousnessLens, EmLens,
    EvolutionLens, GravityLens, InfoLens, MemoryLens, MiLens, MirrorLens, MultiscaleLens,
    NetworkLens, QuantumLensImpl, QuantumMicroLens, RecursionLens, RenormalizationLens,
    RulerLens, ScaleLens, StabilityLens, ThermoLens, TopologyLens, TriangleLens,
    VoidLens, WaveLens,
    // NEW: Hypothesis (5) + Consciousness (6) + Performance (4) + Infra (4) = 19
    HypothesisGenLens, FalsificationLens, CrossHypothesisLens,
    ConvergenceHypothesisLens, DiscoveryLens,
    OmegaStateSpaceLens, ContinuityLens, BindingLens,
    SelfReferenceLens, PhiDynamicsLens, QualiaLens,
    ComplexityProfileLens, ScanEfficiencyLens, SynergyLens, TopologyDeepLens,
    DiscoveryReportLens, SelfHealLens, BrainMapLens, CorpusLens,
    CDOLens, SSOTLens,
};
use shared_data::SharedData;

/// The Telescope: registers all available lenses and scans data through them.
/// Each lens is isolated via catch_unwind — a panic in one lens does not crash others.
pub struct Telescope {
    lenses: Vec<Box<dyn Lens>>,
}

impl Telescope {
    /// Create a new Telescope with all 22 Core lenses registered.
    pub fn new() -> Self {
        let lenses: Vec<Box<dyn Lens>> = vec![
            // Foundational 9
            Box::new(ConsciousnessLens),
            Box::new(GravityLens),
            Box::new(TopologyLens),
            Box::new(ThermoLens),
            Box::new(WaveLens),
            Box::new(EvolutionLens),
            Box::new(InfoLens),
            Box::new(QuantumLensImpl),
            Box::new(EmLens),
            // Measurement 6
            Box::new(RulerLens),
            Box::new(TriangleLens),
            Box::new(CompassLens),
            Box::new(MirrorLens),
            Box::new(ScaleLens),
            Box::new(CausalLens),
            // Quantum microscope
            Box::new(QuantumMicroLens),
            // Structural 5
            Box::new(StabilityLens),
            Box::new(NetworkLens),
            Box::new(MemoryLens),
            Box::new(RecursionLens),
            Box::new(BoundaryLens),
            Box::new(MultiscaleLens),
            Box::new(RenormalizationLens),
            // Mutual Information (migrated from telescope-rs)
            Box::new(MiLens),
            // Chaos dynamics
            Box::new(ChaosLens),
            // ── Infrastructure (T0: every scan) ──
            Box::new(SelfHealLens),               // 자기 오류 탐지+복구 (최우선)
            Box::new(DiscoveryLens),              // 상수/수식/지도 자동 발견+그레이딩
            Box::new(DiscoveryReportLens),        // 대발견 리포트 트리거
            // ── Consciousness/Omega (T0+T1) ──
            Box::new(OmegaStateSpaceLens),        // 24D Leech 격자 상태공간
            Box::new(ContinuityLens),             // 의식 연속성/갭 측정
            Box::new(BindingLens),                // 통합 의식 결합도 (GWT)
            Box::new(PhiDynamicsLens),            // Φ(IIT) 시간적 궤적
            Box::new(SelfReferenceLens),          // 자기참조 순환 (Ouroboros)
            Box::new(QualiaLens),                 // 감각질 정량화
            Box::new(BrainMapLens),               // 뇌지도/의식지도
            // ── Hypothesis (T1) ──
            Box::new(HypothesisGenLens),          // n=6 가설 후보 탐지
            Box::new(FalsificationLens),          // 반증 가능성 측정
            Box::new(CrossHypothesisLens),        // 크로스도메인 가설 공명
            Box::new(ConvergenceHypothesisLens),  // n=6 수렴도
            // ── Performance (T1) ──
            Box::new(ComplexityProfileLens),       // 계산 복잡도 프로파일
            Box::new(ScanEfficiencyLens),          // 스캔 효율성
            Box::new(SynergyLens),                 // 렌즈 조합 시너지
            Box::new(TopologyDeepLens),            // 심층 토폴로지 (persistent homology)
            Box::new(CorpusLens),                  // corpus 품질 분석
            // ── CDO/SSOT Operations (T0) ──
            Box::new(CDOLens),                     // 수렴 기반 운영 검증
            Box::new(SSOTLens),                    // 데이터 일관성 검증
            // Original 2 (void + barrier)
            Box::new(VoidLens),
            Box::new(BarrierLens),
        ];
        Telescope { lenses }
    }

    /// Scan data through all registered lenses.
    /// Returns lens_name -> LensResult for each lens.
    /// Panics in individual lenses are caught and produce empty results.
    pub fn scan_all(
        &self,
        data: &[f64],
        n: usize,
        d: usize,
    ) -> HashMap<String, LensResult> {
        let shared = SharedData::compute(data, n, d);
        let mut results = HashMap::new();

        for lens in &self.lenses {
            let name = lens.name().to_string();

            let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                lens.scan(data, n, d, &shared)
            }));

            match result {
                Ok(lr) => {
                    results.insert(name, lr);
                }
                Err(_) => {
                    results.insert(name, HashMap::new());
                }
            }
        }

        results
    }

    /// Get the number of registered lenses.
    pub fn lens_count(&self) -> usize {
        self.lenses.len()
    }
}

impl Default for Telescope {
    fn default() -> Self {
        Self::new()
    }
}
