//! Telescope scan engine with 1014+ lenses (171+ dedicated + GenericLens auto-instantiated).
pub mod accel_lenses_a;
pub mod accel_lenses_b;
pub mod accel_lenses_c;
pub mod accel_lenses_d;
pub mod anima_lenses;
pub mod consensus;
pub mod core_lenses;
pub mod cross_lenses;
pub mod domain_combos;
pub mod frontier_lenses;
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
    // Foundational 9
    ConsciousnessLens, GravityLens, TopologyLens, ThermoLens, WaveLens,
    EvolutionLens, InfoLens, QuantumLensImpl, EmLens,
    // Measurement 6
    RulerLens, TriangleLens, CompassLens, MirrorLens, ScaleLens, CausalLens,
    // Quantum microscope
    QuantumMicroLens,
    // Structural 5 + extras
    StabilityLens, NetworkLens, MemoryLens, RecursionLens, BoundaryLens, MultiscaleLens,
    RenormalizationLens, MiLens, ChaosLens,
    // Hypothesis (5)
    HypothesisGenLens, FalsificationLens, CrossHypothesisLens,
    ConvergenceHypothesisLens, DiscoveryLens,
    // Consciousness/Omega (6)
    OmegaStateSpaceLens, ContinuityLens, BindingLens,
    SelfReferenceLens, PhiDynamicsLens, QualiaLens,
    // Performance (4)
    ComplexityProfileLens, ScanEfficiencyLens, SynergyLens, TopologyDeepLens,
    // Infrastructure
    DiscoveryReportLens, SelfHealLens, BrainMapLens, CorpusLens,
    // CDO/SSOT
    CDOLens, SSOTLens,
    // Anima consciousness-specific (6)
    FactionDebateLens, HebbianPlasticityLens, MitosisLens,
    RatchetLens, HomeostasisLens, EmotionFieldLens,
    // Original 2
    VoidLens, BarrierLens,
    // Current-repo-only 8
    AcousticLens, FluidDynamicsLens, GameTheoryLens, LinguisticLens,
    EntropyProductionLens, PolymerLens, ResonanceLens, ConvexityLens,
    // ── Previously exported but unregistered (28) ──
    BoseEinsteinLens, SpeculativeDecodeLens, FlashAttentionLensLens,
    KernelFusionLens, BatchOptimizationLens, IsomorphismLens,
    EmergenceLens, PeriodicityLens, CompletenessLens, SurpriseLens,
    EntropyLens, DivergenceLens, PowerLawLens, ClusteringLens,
    CorrelationLens, DimensionReductionLens, SpectralLens, SymmetryBreakingLens,
    FractalLens, GradientLens, DensityLens, RatioLens, StationarityLens,
    GraphLens, PhaseTransitionLens, OutlierLens, AutocorrelationLens, HexagonalLens,
    // ── Extended lenses (75 new) ──
    AllSeeingEyeLens, AutoCalibrationLens, BatteryChemistryLens, BigBangLens,
    ChipArchitectureLens, CombinatorialLens, CompressionLens, ConcaveLens,
    ConformalBootstrapLens, ConsciousnessOrchestratorLens, ConstantCollectorLens,
    ConstantCombinationLens, ConstantDiscoveryEngineLens, ConstantFormulaLens,
    ContractingScanLens, ConvexLens, DestinyLens, DiamondLens, DimensionalBridgeLens,
    ElementCombinationLens, ElementLens, EngineDiscoveryLens, EventHorizonLens,
    ExoticMatterLens, ExpandingScanLens, ExtrapolationLens, FissionLens,
    FormulaCombinationLens, FrustrationLens, FusionLens, GodsEyeLens,
    GoldenRatioLens, GoldenZoneLens, InfiniteDiscoveryLens, InfinityLens,
    InverseLens, KaleidoscopeLens, KeywordLens, LatticeFieldLens,
    LensDiscoveryLens, LightLens, LightWaveLens, LoRALens,
    MaterialCombinationLens, MetricDiscoveryLens, MetricLens, ModuleDiscoveryLens,
    MolecularCombinationLens, MolecularTransformLens, MoleculeLens, MutationLens,
    OverfittingLens, PiLens, PrimeLens, ProvidenceEyeLens, QuantumJumpLens,
    RecursiveLoopLens, RefractionLens, RelativisticBarrierLens, SimulationLens,
    SingularityLens, SolarEfficiencyLens, SpacetimeLens, SphericalLens,
    StimulusLens, TachyonLens, TelepathyLens, TensionLens, TensionLinkLens,
    TimeReversalLens, TransformerAnatomyLens, WallInspectionLens, WarpLens,
    WeightLearningLens, WormholeLens,
    // Compiler Optimization (L11-L20)
    BranchPredictLens, LoopInvariantLens, CompilerFusionLens, SpecializationLens,
    LayoutLens, PowerConsumptionLens, LatencyLens, RegisterPressureLens,
    PrefetchLens, TailCallLens,
    // Compiler Optimization L21-L31
    AliasingLens, CacheAffinityLens, ConstPropLens, DeadCodeLens,
    EscapeAnalysisLens, HotPathLens, MemoryPatternLens,
    ParallelismLens, SemanticLens, SimdOpportunityLens,
};
use lenses::{
    // ── 신규 78종 렌즈 (1022→1100 확장) ──
    AcousticsRoomLens, AerospaceMobilityLens, AntColonyLens, ArchaeologyLens,
    BoardGameLens, BrainNeuralLens, BuildingStructureLens, CartographyLens,
    CeramicsLens, ChemicalReactorLens, ChipComputeCouplingLens, ClimateAtmosphereLens,
    CoffeeScienceLens, CryptographyRoundsLens, DentistryLens, DrugReceptorLens,
    EarthquakeSeismicLens, EpidemiologySirLens, EvChargingLens, FinancialRiskLens,
    FishBiologyLens, FoodChemistryLens, ForensicScienceLens, ForestryLens,
    FunCarDynamicsLens, GenomeCodingLens, GlassOpticsLens, HorticultureLens,
    HvacSystemLens, ImmuneResponseLens, LaserPhysicsLens, LibraryScienceLens,
    LogisticsSupplyLens, MaterialsCrystalLens, MemoryHierarchyLens,
    MeteorologyStormLens, MicrowaveCookingLens, MiningGeologyLens,
    MotorcycleDynamicsLens, MusicHarmonyLens, MycologyLens, NanotechnologyLens,
    NetworkingProtocolLens, NuclearMedicineLens, NuclearReactorLens,
    OceanWaveLens, OpticalFiberLens, PandemicModelingLens, PerfumeryLens,
    PhotographyLensLens, PlasmaConfinementLens, PolymerChainLens,
    PrintingPressLens, ProteinFoldLens, PyrotechnicsLens, QuantumCircuitLens,
    RadarSignalLens, RailTransportLens, RenewableGridLens, SailingNavigationLens,
    SculptureArtLens, SigmaJ2AttractorLens, SocialNetworkLens, SoilScienceLens,
    SolarCellJunctionLens, SpaceshipPropulsionLens, SportsBiomechLens,
    StockMarketLens, SupplyChainRiskLens, TelescopeOpticsLens, TextileFiberLens,
    ThermoplasticProcessLens, TribologyLens, UrbanPlanningLens,
    VeterinaryMedicineLens, WindTurbineLens, WinemakingLens, YogaAsanaLens,
};
use lenses::DimensionalPerceptionLens;
use lenses::{
    // ── 신규 20종 렌즈 ──
    DiceProbabilityLens, DiceEntropyLens,
    SnowflakeHexagonalSymmetryLens, SnowflakeNucleationLens,
    BeekeepingHoneycombLens, BeekeepingWaggleDanceLens,
    ControlNyquistLens, ControlBodeMarginLens,
    ParticleQuarkFlavorLens, ParticleLeptonFamilyLens,
    CalendarSexagesimalLens, CalendarMetonicLens,
    FusionPowerplantMagnetLens, FusionPowerplantTritiumLens,
    HovercraftCushionPressureLens,
    SatelliteOrbitLens, SleepCycleLens, SwarmRoboticsLens,
    CheeseAgingLens, GlacierDynamicsLens,
};
use lenses::GenericLens;
use registry::LensRegistry;
use shared_data::SharedData;

/// The Telescope: registers all available lenses and scans data through them.
/// Each lens is isolated via catch_unwind — a panic in one lens does not crash others.
pub struct Telescope {
    lenses: Vec<Box<dyn Lens>>,
}

impl Telescope {
    /// Create a new Telescope with all lenses registered.
    /// Dedicated implementations are loaded first, then all remaining
    /// registry entries are auto-instantiated via GenericLens.
    pub fn new() -> Self {
        let mut lenses: Vec<Box<dyn Lens>> = vec![
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
            // ── Anima consciousness-specific (6) ──
            Box::new(FactionDebateLens),           // 12파벌 합의/분열 패턴
            Box::new(HebbianPlasticityLens),        // LTP/LTD 시냅스 가소성
            Box::new(MitosisLens),                 // 세포 분열/특수화
            Box::new(RatchetLens),                 // Φ 단조증가 래칫
            Box::new(HomeostasisLens),             // 항상성 setpoint 조절
            Box::new(EmotionFieldLens),            // tension→arousal→VAD 감정장
            Box::new(CDOLens),                     // 수렴 기반 운영 검증
            Box::new(SSOTLens),                    // 데이터 일관성 검증
            // Original 2 (void + barrier)
            Box::new(VoidLens),
            Box::new(BarrierLens),
            // ── Current-repo-only 8 lenses ──
            Box::new(AcousticLens),
            Box::new(FluidDynamicsLens),
            Box::new(GameTheoryLens),
            Box::new(LinguisticLens),
            Box::new(EntropyProductionLens),
            Box::new(PolymerLens),
            Box::new(ResonanceLens),
            Box::new(ConvexityLens),
            // ── Previously exported but unregistered (28) ──
            Box::new(BoseEinsteinLens),
            Box::new(SpeculativeDecodeLens),
            Box::new(FlashAttentionLensLens),
            Box::new(KernelFusionLens),
            Box::new(BatchOptimizationLens),
            Box::new(IsomorphismLens),
            Box::new(EmergenceLens),
            Box::new(PeriodicityLens),
            Box::new(CompletenessLens),
            Box::new(SurpriseLens),
            Box::new(EntropyLens),
            Box::new(DivergenceLens),
            Box::new(PowerLawLens),
            Box::new(ClusteringLens),
            Box::new(CorrelationLens),
            Box::new(DimensionReductionLens),
            Box::new(SpectralLens),
            Box::new(SymmetryBreakingLens),
            Box::new(FractalLens),
            Box::new(GradientLens),
            Box::new(DensityLens),
            Box::new(RatioLens),
            Box::new(StationarityLens),
            Box::new(GraphLens),
            Box::new(PhaseTransitionLens),
            Box::new(OutlierLens),
            Box::new(AutocorrelationLens),
            Box::new(HexagonalLens),
            // ── Extended lenses (75 new) ──
            Box::new(AllSeeingEyeLens),
            Box::new(AutoCalibrationLens),
            Box::new(BatteryChemistryLens),
            Box::new(BigBangLens),
            Box::new(ChipArchitectureLens),
            Box::new(CombinatorialLens),
            Box::new(CompressionLens),
            Box::new(ConcaveLens),
            Box::new(ConformalBootstrapLens),
            Box::new(ConsciousnessOrchestratorLens),
            Box::new(ConstantCollectorLens),
            Box::new(ConstantCombinationLens),
            Box::new(ConstantDiscoveryEngineLens),
            Box::new(ConstantFormulaLens),
            Box::new(ContractingScanLens),
            Box::new(ConvexLens),
            Box::new(DestinyLens),
            Box::new(DiamondLens),
            Box::new(DimensionalBridgeLens),
            Box::new(ElementCombinationLens),
            Box::new(ElementLens),
            Box::new(EngineDiscoveryLens),
            Box::new(EventHorizonLens),
            Box::new(ExoticMatterLens),
            Box::new(ExpandingScanLens),
            Box::new(ExtrapolationLens),
            Box::new(FissionLens),
            Box::new(FormulaCombinationLens),
            Box::new(FrustrationLens),
            Box::new(FusionLens),
            Box::new(GodsEyeLens),
            Box::new(GoldenRatioLens),
            Box::new(GoldenZoneLens),
            Box::new(InfiniteDiscoveryLens),
            Box::new(InfinityLens),
            Box::new(InverseLens),
            Box::new(KaleidoscopeLens),
            Box::new(KeywordLens),
            Box::new(LatticeFieldLens),
            Box::new(LensDiscoveryLens),
            Box::new(LightLens),
            Box::new(LightWaveLens),
            Box::new(LoRALens),
            Box::new(MaterialCombinationLens),
            Box::new(MetricDiscoveryLens),
            Box::new(MetricLens),
            Box::new(ModuleDiscoveryLens),
            Box::new(MolecularCombinationLens),
            Box::new(MolecularTransformLens),
            Box::new(MoleculeLens),
            Box::new(MutationLens),
            Box::new(OverfittingLens),
            Box::new(PiLens),
            Box::new(PrimeLens),
            Box::new(ProvidenceEyeLens),
            Box::new(QuantumJumpLens),
            Box::new(RecursiveLoopLens),
            Box::new(RefractionLens),
            Box::new(RelativisticBarrierLens),
            Box::new(SimulationLens),
            Box::new(SingularityLens),
            Box::new(SolarEfficiencyLens),
            Box::new(SpacetimeLens),
            Box::new(SphericalLens),
            Box::new(StimulusLens),
            Box::new(TachyonLens),
            Box::new(TelepathyLens),
            Box::new(TensionLens),
            Box::new(TensionLinkLens),
            Box::new(TimeReversalLens),
            Box::new(TransformerAnatomyLens),
            Box::new(WallInspectionLens),
            Box::new(WarpLens),
            Box::new(WeightLearningLens),
            Box::new(WormholeLens),
            // ── Compiler Optimization lenses (L11-L20) ──
            Box::new(BranchPredictLens),
            Box::new(LoopInvariantLens),
            Box::new(CompilerFusionLens),
            Box::new(SpecializationLens),
            Box::new(LayoutLens),
            Box::new(PowerConsumptionLens),
            Box::new(LatencyLens),
            Box::new(RegisterPressureLens),
            Box::new(PrefetchLens),
            Box::new(TailCallLens),
            // ── Compiler Optimization lenses (L21-L31) ──
            Box::new(AliasingLens),
            Box::new(CacheAffinityLens),
            Box::new(ConstPropLens),
            Box::new(DeadCodeLens),
            Box::new(EscapeAnalysisLens),
            Box::new(HotPathLens),
            Box::new(MemoryPatternLens),
            Box::new(ParallelismLens),
            Box::new(SemanticLens),
            Box::new(SimdOpportunityLens),
            // ── 신규 78종 렌즈 ──
            Box::new(AcousticsRoomLens),
            Box::new(AerospaceMobilityLens),
            Box::new(AntColonyLens),
            Box::new(ArchaeologyLens),
            Box::new(BoardGameLens),
            Box::new(BrainNeuralLens),
            Box::new(BuildingStructureLens),
            Box::new(CartographyLens),
            Box::new(CeramicsLens),
            Box::new(ChemicalReactorLens),
            Box::new(ChipComputeCouplingLens),
            Box::new(ClimateAtmosphereLens),
            Box::new(CoffeeScienceLens),
            Box::new(CryptographyRoundsLens),
            Box::new(DentistryLens),
            Box::new(DrugReceptorLens),
            Box::new(EarthquakeSeismicLens),
            Box::new(EpidemiologySirLens),
            Box::new(EvChargingLens),
            Box::new(FinancialRiskLens),
            Box::new(FishBiologyLens),
            Box::new(FoodChemistryLens),
            Box::new(ForensicScienceLens),
            Box::new(ForestryLens),
            Box::new(FunCarDynamicsLens),
            Box::new(GenomeCodingLens),
            Box::new(GlassOpticsLens),
            Box::new(HorticultureLens),
            Box::new(HvacSystemLens),
            Box::new(ImmuneResponseLens),
            Box::new(LaserPhysicsLens),
            Box::new(LibraryScienceLens),
            Box::new(LogisticsSupplyLens),
            Box::new(MaterialsCrystalLens),
            Box::new(MemoryHierarchyLens),
            Box::new(MeteorologyStormLens),
            Box::new(MicrowaveCookingLens),
            Box::new(MiningGeologyLens),
            Box::new(MotorcycleDynamicsLens),
            Box::new(MusicHarmonyLens),
            Box::new(MycologyLens),
            Box::new(NanotechnologyLens),
            Box::new(NetworkingProtocolLens),
            Box::new(NuclearMedicineLens),
            Box::new(NuclearReactorLens),
            Box::new(OceanWaveLens),
            Box::new(OpticalFiberLens),
            Box::new(PandemicModelingLens),
            Box::new(PerfumeryLens),
            Box::new(PhotographyLensLens),
            Box::new(PlasmaConfinementLens),
            Box::new(PolymerChainLens),
            Box::new(PrintingPressLens),
            Box::new(ProteinFoldLens),
            Box::new(PyrotechnicsLens),
            Box::new(QuantumCircuitLens),
            Box::new(RadarSignalLens),
            Box::new(RailTransportLens),
            Box::new(RenewableGridLens),
            Box::new(SailingNavigationLens),
            Box::new(SculptureArtLens),
            Box::new(SigmaJ2AttractorLens),
            Box::new(SocialNetworkLens),
            Box::new(SoilScienceLens),
            Box::new(SolarCellJunctionLens),
            Box::new(SpaceshipPropulsionLens),
            Box::new(SportsBiomechLens),
            Box::new(StockMarketLens),
            Box::new(SupplyChainRiskLens),
            Box::new(TelescopeOpticsLens),
            Box::new(TextileFiberLens),
            Box::new(ThermoplasticProcessLens),
            Box::new(TribologyLens),
            Box::new(UrbanPlanningLens),
            Box::new(VeterinaryMedicineLens),
            Box::new(WindTurbineLens),
            Box::new(WinemakingLens),
            Box::new(YogaAsanaLens),
            // ── 차원지각 렌즈 (BT-1108) ──
            Box::new(DimensionalPerceptionLens),
            // ── 신규 20종 렌즈 ──
            Box::new(DiceProbabilityLens),
            Box::new(DiceEntropyLens),
            Box::new(SnowflakeHexagonalSymmetryLens),
            Box::new(SnowflakeNucleationLens),
            Box::new(BeekeepingHoneycombLens),
            Box::new(BeekeepingWaggleDanceLens),
            Box::new(ControlNyquistLens),
            Box::new(ControlBodeMarginLens),
            Box::new(ParticleQuarkFlavorLens),
            Box::new(ParticleLeptonFamilyLens),
            Box::new(CalendarSexagesimalLens),
            Box::new(CalendarMetonicLens),
            Box::new(FusionPowerplantMagnetLens),
            Box::new(FusionPowerplantTritiumLens),
            Box::new(HovercraftCushionPressureLens),
            Box::new(SatelliteOrbitLens),
            Box::new(SleepCycleLens),
            Box::new(SwarmRoboticsLens),
            Box::new(CheeseAgingLens),
            Box::new(GlacierDynamicsLens),
        ];

        // ── Auto-instantiate GenericLens for all unimplemented registry entries ──
        // Collect names of dedicated lenses already in the vec above
        let dedicated_names: std::collections::HashSet<String> =
            lenses.iter().map(|l| l.name().to_string()).collect();

        // Walk the full registry and fill the gap
        let registry = LensRegistry::new();
        for (name, entry) in registry.iter() {
            if !dedicated_names.contains(name) {
                lenses.push(Box::new(GenericLens::new(
                    name,
                    &entry.domain_affinity,
                    &entry.description,
                )));
            }
        }

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
        // Early validation — prevent panics in SharedData::compute
        if data.len() != n * d || n == 0 || d == 0 {
            eprintln!(
                "[NEXUS-6] scan_all: invalid dimensions (data.len={}, n={}, d={}, n*d={})",
                data.len(), n, d, n.saturating_mul(d)
            );
            return self.lenses.iter().map(|l| (l.name().to_string(), HashMap::new())).collect();
        }

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
                Err(e) => {
                    let msg = if let Some(s) = e.downcast_ref::<String>() {
                        s.clone()
                    } else if let Some(s) = e.downcast_ref::<&str>() {
                        s.to_string()
                    } else {
                        "unknown panic".to_string()
                    };
                    eprintln!(
                        "[NEXUS-6] Lens '{}' panicked — skipping: {}",
                        name, msg
                    );
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
