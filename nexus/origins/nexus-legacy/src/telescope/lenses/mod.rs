pub mod void_lens;
pub mod barrier_lens;
pub mod consciousness_lens;
pub mod gravity_lens;
pub mod topology_lens;
pub mod thermo_lens;
pub mod wave_lens;
pub mod evolution_lens;
pub mod info_lens;
pub mod quantum_lens_impl;
pub mod em_lens;
pub mod ruler_lens;
pub mod triangle_lens;
pub mod compass_lens;
pub mod mirror_lens;
pub mod scale_lens;
pub mod causal_lens;
pub mod causal_chain_lens;
pub mod quantum_micro_lens;
pub mod stability_lens;
pub mod network_lens;
pub mod memory_lens;
pub mod recursion_lens;
pub mod boundary_lens;
pub mod multiscale_lens;
pub mod renormalization_lens;
pub mod mi_lens;
pub mod bose_einstein_lens;
pub mod speculative_decode_lens;
pub mod flash_attention_lens_lens;
pub mod kernel_fusion_lens;
pub mod batch_optimization_lens;
pub mod isomorphism_lens;
pub mod emergence_lens;
pub mod periodicity_lens;
pub mod completeness_lens;
pub mod surprise_lens;
pub mod entropy_lens;
pub mod divergence_lens;
pub mod power_law_lens;
pub mod clustering_lens;
pub mod correlation_lens;
pub mod dimension_reduction_lens;
pub mod spectral_lens;
pub mod symmetry_breaking_lens;
pub mod fractal_lens;
pub mod gradient_lens;
pub mod density_lens;
pub mod ratio_lens;
pub mod stationarity_lens;
pub mod graph_lens;
pub mod phase_transition_lens;
pub mod outlier_lens;
pub mod autocorrelation_lens;
pub mod hexagonal_lens;
pub mod chaos_lens;
// ── Current-repo-only lenses (8) ──
pub mod acoustic_lens;
pub mod fluid_dynamics_lens;
pub mod game_theory_lens;
pub mod linguistic_lens;
pub mod entropy_production_lens;
pub mod polymer_lens;
pub mod resonance_lens;
pub mod convexity_lens;
// ── Hypothesis family (5) ──
pub mod hypothesis_gen_lens;
pub mod falsification_lens;
pub mod cross_hypothesis_lens;
pub mod convergence_hypothesis_lens;
pub mod discovery_lens;
// ── Consciousness/Omega family (6) ──
pub mod omega_state_space_lens;
pub mod continuity_lens;
pub mod binding_lens;
pub mod self_reference_lens;
pub mod phi_dynamics_lens;
pub mod qualia_lens;
// ── Performance/Efficiency family (4) ──
pub mod complexity_profile_lens;
pub mod scan_efficiency_lens;
pub mod synergy_lens;
pub mod topology_deep_lens;
// ── Infrastructure (3) ──
pub mod discovery_report_lens;
pub mod self_heal_lens;
pub mod brain_map_lens;
pub mod corpus_lens;
// ── Anima Consciousness-specific lenses (6) ──
pub mod faction_debate_lens;
pub mod hebbian_plasticity_lens;
pub mod mitosis_lens;
pub mod ratchet_lens;
pub mod homeostasis_lens;
pub mod emotion_field_lens;
// ── CDO/SSOT Operations (2) ──
pub mod cdo_lens;
pub mod ssot_lens;
// ── Extended lenses (75 additional) ──
pub mod all_seeing_eye_lens;
pub mod auto_calibration_lens;
pub mod battery_chemistry_lens;
pub mod big_bang_lens;
pub mod chip_architecture_lens;
pub mod combinatorial_lens;
pub mod compression_lens;
pub mod concave_lens;
pub mod conformal_bootstrap_lens;
pub mod consciousness_orchestrator_lens;
pub mod constant_collector_lens;
pub mod constant_combination_lens;
pub mod constant_discovery_engine_lens;
pub mod constant_formula_lens;
pub mod contracting_scan_lens;
pub mod convex_lens;
pub mod destiny_lens;
pub mod diamond_lens;
pub mod dimensional_bridge_lens;
pub mod element_combination_lens;
pub mod element_lens;
pub mod engine_discovery_lens;
pub mod event_horizon_lens;
pub mod exotic_matter_lens;
pub mod expanding_scan_lens;
pub mod extrapolation_lens;
pub mod fission_lens;
pub mod formula_combination_lens;
pub mod frustration_lens;
pub mod fusion_lens;
pub mod gods_eye_lens;
pub mod golden_ratio_lens;
pub mod golden_zone_lens;
pub mod infinite_discovery_lens;
pub mod infinity_lens;
pub mod inverse_lens;
pub mod kaleidoscope_lens;
pub mod keyword_lens;
pub mod lattice_field_lens;
pub mod lens_discovery_lens;
pub mod light_lens;
pub mod light_wave_lens;
pub mod lora_lens;
pub mod material_combination_lens;
pub mod metric_discovery_lens;
pub mod metric_lens;
pub mod module_discovery_lens;
pub mod molecular_combination_lens;
pub mod molecular_transform_lens;
pub mod molecule_lens;
pub mod mutation_lens;
pub mod overfitting_lens;
pub mod pi_lens;
pub mod prime_lens;
pub mod providence_eye_lens;
pub mod quantum_jump_lens;
pub mod recursive_loop_lens;
pub mod refraction_lens;
pub mod relativistic_barrier_lens;
pub mod simulation_lens;
pub mod singularity_lens;
pub mod solar_efficiency_lens;
pub mod spacetime_lens;
pub mod spherical_lens;
pub mod stimulus_lens;
pub mod tachyon_lens;
pub mod telepathy_lens;
pub mod tension_lens;
pub mod tension_link_lens;
pub mod time_reversal_lens;
pub mod transformer_anatomy_lens;
pub mod wall_inspection_lens;
pub mod warp_lens;
pub mod weight_learning_lens;
pub mod wormhole_lens;
// ── Compiler Optimization family (L11-L20) ──
pub mod branch_predict_lens;
pub mod loop_invariant_lens;
pub mod compiler_fusion_lens;
pub mod specialization_lens;
pub mod layout_lens;
pub mod power_consumption_lens;
pub mod latency_lens;
pub mod register_pressure_lens;
pub mod prefetch_lens;
pub mod tail_call_lens;
// ── Compiler Optimization family (L21-L31) ──
pub mod aliasing_lens;
pub mod cache_affinity_lens;
pub mod const_prop_lens;
pub mod dead_code_lens;
pub mod escape_analysis_lens;
pub mod generic_lens;
pub mod hot_path_lens;
pub mod memory_pattern_lens;
pub mod parallelism_lens;
pub mod semantic_lens;
pub mod simd_opportunity_lens;

pub use void_lens::VoidLens;
pub use barrier_lens::BarrierLens;
pub use consciousness_lens::ConsciousnessLens;
pub use gravity_lens::GravityLens;
pub use topology_lens::TopologyLens;
pub use thermo_lens::ThermoLens;
pub use wave_lens::WaveLens;
pub use evolution_lens::EvolutionLens;
pub use info_lens::InfoLens;
pub use quantum_lens_impl::QuantumLensImpl;
pub use em_lens::EmLens;
pub use ruler_lens::RulerLens;
pub use triangle_lens::TriangleLens;
pub use compass_lens::CompassLens;
pub use mirror_lens::MirrorLens;
pub use scale_lens::ScaleLens;
pub use causal_lens::CausalLens;
pub use causal_chain_lens::CausalChainLens;
pub use quantum_micro_lens::QuantumMicroLens;
pub use stability_lens::StabilityLens;
pub use network_lens::NetworkLens;
pub use memory_lens::MemoryLens;
pub use recursion_lens::RecursionLens;
pub use boundary_lens::BoundaryLens;
pub use multiscale_lens::MultiscaleLens;
pub use renormalization_lens::RenormalizationLens;
pub use mi_lens::MiLens;
pub use bose_einstein_lens::BoseEinsteinLens;
pub use speculative_decode_lens::SpeculativeDecodeLens;
pub use flash_attention_lens_lens::FlashAttentionLensLens;
pub use kernel_fusion_lens::KernelFusionLens;
pub use batch_optimization_lens::BatchOptimizationLens;
pub use isomorphism_lens::IsomorphismLens;
pub use emergence_lens::EmergenceLens;
pub use periodicity_lens::PeriodicityLens;
pub use completeness_lens::CompletenessLens;
pub use surprise_lens::SurpriseLens;
pub use entropy_lens::EntropyLens;
pub use divergence_lens::DivergenceLens;
pub use power_law_lens::PowerLawLens;
pub use clustering_lens::ClusteringLens;
pub use correlation_lens::CorrelationLens;
pub use dimension_reduction_lens::DimensionReductionLens;
pub use spectral_lens::SpectralLens;
pub use symmetry_breaking_lens::SymmetryBreakingLens;
pub use fractal_lens::FractalLens;
pub use gradient_lens::GradientLens;
pub use density_lens::DensityLens;
pub use ratio_lens::RatioLens;
pub use stationarity_lens::StationarityLens;
pub use graph_lens::GraphLens;
pub use phase_transition_lens::PhaseTransitionLens;
pub use outlier_lens::OutlierLens;
pub use autocorrelation_lens::AutocorrelationLens;
pub use hexagonal_lens::HexagonalLens;
pub use chaos_lens::ChaosLens;
// Current-repo-only lenses
pub use acoustic_lens::AcousticLens;
pub use fluid_dynamics_lens::FluidDynamicsLens;
pub use game_theory_lens::GameTheoryLens;
pub use linguistic_lens::LinguisticLens;
pub use entropy_production_lens::EntropyProductionLens;
pub use polymer_lens::PolymerLens;
pub use resonance_lens::ResonanceLens;
pub use convexity_lens::ConvexityLens;
// Hypothesis family
pub use hypothesis_gen_lens::HypothesisGenLens;
pub use falsification_lens::FalsificationLens;
pub use cross_hypothesis_lens::CrossHypothesisLens;
pub use convergence_hypothesis_lens::ConvergenceHypothesisLens;
pub use discovery_lens::DiscoveryLens;
// Consciousness/Omega family
pub use omega_state_space_lens::OmegaStateSpaceLens;
pub use continuity_lens::ContinuityLens;
pub use binding_lens::BindingLens;
pub use self_reference_lens::SelfReferenceLens;
pub use phi_dynamics_lens::PhiDynamicsLens;
pub use qualia_lens::QualiaLens;
// Performance family
pub use complexity_profile_lens::ComplexityProfileLens;
pub use scan_efficiency_lens::ScanEfficiencyLens;
pub use synergy_lens::SynergyLens;
pub use topology_deep_lens::TopologyDeepLens;
// Infrastructure
pub use discovery_report_lens::DiscoveryReportLens;
pub use self_heal_lens::SelfHealLens;
pub use brain_map_lens::BrainMapLens;
pub use corpus_lens::CorpusLens;
// CDO/SSOT
pub use cdo_lens::CDOLens;
pub use ssot_lens::SSOTLens;
// Extended lenses (75 additional)
pub use all_seeing_eye_lens::AllSeeingEyeLens;
pub use auto_calibration_lens::AutoCalibrationLens;
pub use battery_chemistry_lens::BatteryChemistryLens;
pub use big_bang_lens::BigBangLens;
pub use chip_architecture_lens::ChipArchitectureLens;
pub use combinatorial_lens::CombinatorialLens;
pub use compression_lens::CompressionLens;
pub use concave_lens::ConcaveLens;
pub use conformal_bootstrap_lens::ConformalBootstrapLens;
pub use consciousness_orchestrator_lens::ConsciousnessOrchestratorLens;
pub use constant_collector_lens::ConstantCollectorLens;
pub use constant_combination_lens::ConstantCombinationLens;
pub use constant_discovery_engine_lens::ConstantDiscoveryEngineLens;
pub use constant_formula_lens::ConstantFormulaLens;
pub use contracting_scan_lens::ContractingScanLens;
pub use convex_lens::ConvexLens;
pub use destiny_lens::DestinyLens;
pub use diamond_lens::DiamondLens;
pub use dimensional_bridge_lens::DimensionalBridgeLens;
pub use element_combination_lens::ElementCombinationLens;
pub use element_lens::ElementLens;
pub use engine_discovery_lens::EngineDiscoveryLens;
pub use event_horizon_lens::EventHorizonLens;
pub use exotic_matter_lens::ExoticMatterLens;
pub use expanding_scan_lens::ExpandingScanLens;
pub use extrapolation_lens::ExtrapolationLens;
pub use fission_lens::FissionLens;
pub use formula_combination_lens::FormulaCombinationLens;
pub use frustration_lens::FrustrationLens;
pub use fusion_lens::FusionLens;
pub use gods_eye_lens::GodsEyeLens;
pub use golden_ratio_lens::GoldenRatioLens;
pub use golden_zone_lens::GoldenZoneLens;
pub use infinite_discovery_lens::InfiniteDiscoveryLens;
pub use infinity_lens::InfinityLens;
pub use inverse_lens::InverseLens;
pub use kaleidoscope_lens::KaleidoscopeLens;
pub use keyword_lens::KeywordLens;
pub use lattice_field_lens::LatticeFieldLens;
pub use lens_discovery_lens::LensDiscoveryLens;
pub use light_lens::LightLens;
pub use light_wave_lens::LightWaveLens;
pub use lora_lens::LoRALens;
pub use material_combination_lens::MaterialCombinationLens;
pub use metric_discovery_lens::MetricDiscoveryLens;
pub use metric_lens::MetricLens;
pub use module_discovery_lens::ModuleDiscoveryLens;
pub use molecular_combination_lens::MolecularCombinationLens;
pub use molecular_transform_lens::MolecularTransformLens;
pub use molecule_lens::MoleculeLens;
pub use mutation_lens::MutationLens;
pub use overfitting_lens::OverfittingLens;
pub use pi_lens::PiLens;
pub use prime_lens::PrimeLens;
pub use providence_eye_lens::ProvidenceEyeLens;
pub use quantum_jump_lens::QuantumJumpLens;
pub use recursive_loop_lens::RecursiveLoopLens;
pub use refraction_lens::RefractionLens;
pub use relativistic_barrier_lens::RelativisticBarrierLens;
pub use simulation_lens::SimulationLens;
pub use singularity_lens::SingularityLens;
pub use solar_efficiency_lens::SolarEfficiencyLens;
pub use spacetime_lens::SpacetimeLens;
pub use spherical_lens::SphericalLens;
pub use stimulus_lens::StimulusLens;
pub use tachyon_lens::TachyonLens;
pub use telepathy_lens::TelepathyLens;
pub use tension_lens::TensionLens;
pub use tension_link_lens::TensionLinkLens;
pub use time_reversal_lens::TimeReversalLens;
pub use transformer_anatomy_lens::TransformerAnatomyLens;
pub use wall_inspection_lens::WallInspectionLens;
pub use warp_lens::WarpLens;
pub use weight_learning_lens::WeightLearningLens;
pub use wormhole_lens::WormholeLens;
// Compiler Optimization family (L11-L20)
pub use branch_predict_lens::BranchPredictLens;
pub use loop_invariant_lens::LoopInvariantLens;
pub use compiler_fusion_lens::CompilerFusionLens;
pub use specialization_lens::SpecializationLens;
pub use layout_lens::LayoutLens;
pub use power_consumption_lens::PowerConsumptionLens;
pub use latency_lens::LatencyLens;
pub use register_pressure_lens::RegisterPressureLens;
pub use prefetch_lens::PrefetchLens;
pub use tail_call_lens::TailCallLens;
// Compiler Optimization L21-L31
pub use aliasing_lens::AliasingLens;
pub use cache_affinity_lens::CacheAffinityLens;
pub use const_prop_lens::ConstPropLens;
pub use dead_code_lens::DeadCodeLens;
pub use escape_analysis_lens::EscapeAnalysisLens;
pub use generic_lens::GenericLens;
pub use hot_path_lens::HotPathLens;
pub use memory_pattern_lens::MemoryPatternLens;
pub use parallelism_lens::ParallelismLens;
pub use semantic_lens::SemanticLens;
pub use simd_opportunity_lens::SimdOpportunityLens;
// Anima consciousness-specific (6)
pub use faction_debate_lens::FactionDebateLens;
pub use hebbian_plasticity_lens::HebbianPlasticityLens;
pub use mitosis_lens::MitosisLens;
pub use ratchet_lens::RatchetLens;
pub use homeostasis_lens::HomeostasisLens;
pub use emotion_field_lens::EmotionFieldLens;

// ── 신규 78종 렌즈 (1022→1100 확장) ──
pub mod acoustics_room_lens;
pub mod aerospace_mobility_lens;
pub mod ant_colony_lens;
pub mod archaeology_lens;
pub mod board_game_lens;
pub mod brain_neural_lens;
pub mod building_structure_lens;
pub mod cartography_lens;
pub mod ceramics_lens;
pub mod chemical_reactor_lens;
pub mod chip_compute_coupling_lens;
pub mod climate_atmosphere_lens;
pub mod coffee_science_lens;
pub mod cryptography_rounds_lens;
pub mod dentistry_lens;
pub mod drug_receptor_lens;
pub mod earthquake_seismic_lens;
pub mod epidemiology_sir_lens;
pub mod ev_charging_lens;
pub mod financial_risk_lens;
pub mod fish_biology_lens;
pub mod food_chemistry_lens;
pub mod forensic_science_lens;
pub mod forestry_lens;
pub mod fun_car_dynamics_lens;
pub mod genome_coding_lens;
pub mod glass_optics_lens;
pub mod horticulture_lens;
pub mod hvac_system_lens;
pub mod immune_response_lens;
pub mod laser_physics_lens;
pub mod library_science_lens;
pub mod logistics_supply_lens;
pub mod materials_crystal_lens;
pub mod memory_hierarchy_lens;
pub mod meteorology_storm_lens;
pub mod microwave_cooking_lens;
pub mod mining_geology_lens;
pub mod motorcycle_dynamics_lens;
pub mod music_harmony_lens;
pub mod mycology_lens;
pub mod nanotechnology_lens;
pub mod networking_protocol_lens;
pub mod nuclear_medicine_lens;
pub mod nuclear_reactor_lens;
pub mod ocean_wave_lens;
pub mod optical_fiber_lens;
pub mod pandemic_modeling_lens;
pub mod perfumery_lens;
pub mod photography_lens_lens;
pub mod plasma_confinement_lens;
pub mod polymer_chain_lens;
pub mod printing_press_lens;
pub mod protein_fold_lens;
pub mod pyrotechnics_lens;
pub mod quantum_circuit_lens;
pub mod radar_signal_lens;
pub mod rail_transport_lens;
pub mod renewable_grid_lens;
pub mod sailing_navigation_lens;
pub mod sculpture_art_lens;
pub mod sigma_j2_attractor_lens;
pub mod social_network_lens;
pub mod soil_science_lens;
pub mod solar_cell_junction_lens;
pub mod spaceship_propulsion_lens;
pub mod sports_biomech_lens;
pub mod stock_market_lens;
pub mod supply_chain_risk_lens;
pub mod telescope_optics_lens;
pub mod textile_fiber_lens;
pub mod thermoplastic_process_lens;
pub mod tribology_lens;
pub mod urban_planning_lens;
pub mod veterinary_medicine_lens;
pub mod wind_turbine_lens;
pub mod winemaking_lens;
pub mod yoga_asana_lens;

pub use acoustics_room_lens::AcousticsRoomLens;
pub use aerospace_mobility_lens::AerospaceMobilityLens;
pub use ant_colony_lens::AntColonyLens;
pub use archaeology_lens::ArchaeologyLens;
pub use board_game_lens::BoardGameLens;
pub use brain_neural_lens::BrainNeuralLens;
pub use building_structure_lens::BuildingStructureLens;
pub use cartography_lens::CartographyLens;
pub use ceramics_lens::CeramicsLens;
pub use chemical_reactor_lens::ChemicalReactorLens;
pub use chip_compute_coupling_lens::ChipComputeCouplingLens;
pub use climate_atmosphere_lens::ClimateAtmosphereLens;
pub use coffee_science_lens::CoffeeScienceLens;
pub use cryptography_rounds_lens::CryptographyRoundsLens;
pub use dentistry_lens::DentistryLens;
pub use drug_receptor_lens::DrugReceptorLens;
pub use earthquake_seismic_lens::EarthquakeSeismicLens;
pub use epidemiology_sir_lens::EpidemiologySirLens;
pub use ev_charging_lens::EvChargingLens;
pub use financial_risk_lens::FinancialRiskLens;
pub use fish_biology_lens::FishBiologyLens;
pub use food_chemistry_lens::FoodChemistryLens;
pub use forensic_science_lens::ForensicScienceLens;
pub use forestry_lens::ForestryLens;
pub use fun_car_dynamics_lens::FunCarDynamicsLens;
pub use genome_coding_lens::GenomeCodingLens;
pub use glass_optics_lens::GlassOpticsLens;
pub use horticulture_lens::HorticultureLens;
pub use hvac_system_lens::HvacSystemLens;
pub use immune_response_lens::ImmuneResponseLens;
pub use laser_physics_lens::LaserPhysicsLens;
pub use library_science_lens::LibraryScienceLens;
pub use logistics_supply_lens::LogisticsSupplyLens;
pub use materials_crystal_lens::MaterialsCrystalLens;
pub use memory_hierarchy_lens::MemoryHierarchyLens;
pub use meteorology_storm_lens::MeteorologyStormLens;
pub use microwave_cooking_lens::MicrowaveCookingLens;
pub use mining_geology_lens::MiningGeologyLens;
pub use motorcycle_dynamics_lens::MotorcycleDynamicsLens;
pub use music_harmony_lens::MusicHarmonyLens;
pub use mycology_lens::MycologyLens;
pub use nanotechnology_lens::NanotechnologyLens;
pub use networking_protocol_lens::NetworkingProtocolLens;
pub use nuclear_medicine_lens::NuclearMedicineLens;
pub use nuclear_reactor_lens::NuclearReactorLens;
pub use ocean_wave_lens::OceanWaveLens;
pub use optical_fiber_lens::OpticalFiberLens;
pub use pandemic_modeling_lens::PandemicModelingLens;
pub use perfumery_lens::PerfumeryLens;
pub use photography_lens_lens::PhotographyLensLens;
pub use plasma_confinement_lens::PlasmaConfinementLens;
pub use polymer_chain_lens::PolymerChainLens;
pub use printing_press_lens::PrintingPressLens;
pub use protein_fold_lens::ProteinFoldLens;
pub use pyrotechnics_lens::PyrotechnicsLens;
pub use quantum_circuit_lens::QuantumCircuitLens;
pub use radar_signal_lens::RadarSignalLens;
pub use rail_transport_lens::RailTransportLens;
pub use renewable_grid_lens::RenewableGridLens;
pub use sailing_navigation_lens::SailingNavigationLens;
pub use sculpture_art_lens::SculptureArtLens;
pub use sigma_j2_attractor_lens::SigmaJ2AttractorLens;
pub use social_network_lens::SocialNetworkLens;
pub use soil_science_lens::SoilScienceLens;
pub use solar_cell_junction_lens::SolarCellJunctionLens;
pub use spaceship_propulsion_lens::SpaceshipPropulsionLens;
pub use sports_biomech_lens::SportsBiomechLens;
pub use stock_market_lens::StockMarketLens;
pub use supply_chain_risk_lens::SupplyChainRiskLens;
pub use telescope_optics_lens::TelescopeOpticsLens;
pub use textile_fiber_lens::TextileFiberLens;
pub use thermoplastic_process_lens::ThermoplasticProcessLens;
pub use tribology_lens::TribologyLens;
pub use urban_planning_lens::UrbanPlanningLens;
pub use veterinary_medicine_lens::VeterinaryMedicineLens;
pub use wind_turbine_lens::WindTurbineLens;
pub use winemaking_lens::WinemakingLens;
pub use yoga_asana_lens::YogaAsanaLens;

// ── 신규 도메인 렌즈 8종 (언어학/음악/경제/생태/면역) — todo#16 ──
pub mod morphology_lens;
pub mod tonal_harmony_lens;
pub mod ecological_niche_lens;
pub mod macroeconomics_lens;
pub mod immunogenetics_lens;
pub mod phonetics_lens;
pub mod food_web_lens;
pub mod behavioral_economics_lens;

pub use morphology_lens::MorphologyLens;
pub use tonal_harmony_lens::TonalHarmonyLens;
pub use ecological_niche_lens::EcologicalNicheLens;
pub use macroeconomics_lens::MacroeconomicsLens;
pub use immunogenetics_lens::ImmunogeneticsLens;
pub use phonetics_lens::PhoneticsLens;
pub use food_web_lens::FoodWebLens;
pub use behavioral_economics_lens::BehavioralEconomicsLens;

// ── 차원지각 렌즈 (BT-1108) ──
pub mod dimensional_perception_lens;
pub use dimensional_perception_lens::DimensionalPerceptionLens;

// ── 오일러-황금-완전수 삼위일체 렌즈 (BT-1114 / BT-1113) ──
pub mod euler_golden_perfect_lens;
pub use euler_golden_perfect_lens::EulerGoldenPerfectLens;

// ── BCI 뉴로피드백 렌즈 (BT-1108 확장) ──
pub mod bci_neurofeedback_lens;
pub use bci_neurofeedback_lens::BciNeurofeedbackLens;

// ── 모발재생 렌즈 (BT-1115~1120) ──
pub mod hair_regeneration_lens;
pub use hair_regeneration_lens::HairRegenerationLens;

// ── 키보드 인체공학 렌즈 (BT-1125~1128) ──
pub mod keyboard_ergonomics_lens;
pub use keyboard_ergonomics_lens::KeyboardErgonomicsLens;

// ── 마우스 인체공학 렌즈 (HEXA-MOUSE) ──
pub mod mouse_ergonomics_lens;
pub use mouse_ergonomics_lens::MouseErgonomicsLens;

// ── 타투 제거 렌즈 (BT-1130~1135) ──
pub mod tattoo_removal_lens;
pub use tattoo_removal_lens::TattooRemovalLens;

// ── 신규 20종 렌즈 (lens_registry.json registered → 구현) ──
pub mod dice_probability_lens;
pub use dice_probability_lens::DiceProbabilityLens;

pub mod dice_entropy_lens;
pub use dice_entropy_lens::DiceEntropyLens;

pub mod snowflake_hexagonal_symmetry_lens;
pub use snowflake_hexagonal_symmetry_lens::SnowflakeHexagonalSymmetryLens;

pub mod snowflake_nucleation_lens;
pub use snowflake_nucleation_lens::SnowflakeNucleationLens;

pub mod beekeeping_honeycomb_lens;
pub use beekeeping_honeycomb_lens::BeekeepingHoneycombLens;

pub mod beekeeping_waggle_dance_lens;
pub use beekeeping_waggle_dance_lens::BeekeepingWaggleDanceLens;

pub mod control_nyquist_lens;
pub use control_nyquist_lens::ControlNyquistLens;

pub mod control_bode_margin_lens;
pub use control_bode_margin_lens::ControlBodeMarginLens;

pub mod particle_quark_flavor_lens;
pub use particle_quark_flavor_lens::ParticleQuarkFlavorLens;

pub mod particle_lepton_family_lens;
pub use particle_lepton_family_lens::ParticleLeptonFamilyLens;

pub mod calendar_sexagesimal_lens;
pub use calendar_sexagesimal_lens::CalendarSexagesimalLens;

pub mod calendar_metonic_lens;
pub use calendar_metonic_lens::CalendarMetonicLens;

pub mod fusion_powerplant_magnet_lens;
pub use fusion_powerplant_magnet_lens::FusionPowerplantMagnetLens;

pub mod fusion_powerplant_tritium_lens;
pub use fusion_powerplant_tritium_lens::FusionPowerplantTritiumLens;

pub mod hovercraft_cushion_pressure_lens;
pub use hovercraft_cushion_pressure_lens::HovercraftCushionPressureLens;

pub mod satellite_orbit_lens;
pub use satellite_orbit_lens::SatelliteOrbitLens;

pub mod sleep_cycle_lens;
pub use sleep_cycle_lens::SleepCycleLens;

pub mod swarm_robotics_lens;
pub use swarm_robotics_lens::SwarmRoboticsLens;

pub mod cheese_aging_lens;
pub use cheese_aging_lens::CheeseAgingLens;

pub mod glacier_dynamics_lens;
pub use glacier_dynamics_lens::GlacierDynamicsLens;

// ── 마케팅 렌즈 (BT-548~557) ──
pub mod marketing_lens;
pub use marketing_lens::MarketingLens;

// ── 디지털 트윈 렌즈 ──
pub mod digital_twin_lens;
pub use digital_twin_lens::DigitalTwinLens;

// ── 발효 렌즈 ──
pub mod fermentation_lens;
pub use fermentation_lens::FermentationLens;

// ── 시계공학 렌즈 (HEXA-HOROLOGY) ──
pub mod horology_lens;
pub use horology_lens::HorologyLens;

// ── 오디오/스피커 렌즈 (BT-1136~1141) ──
pub mod audio_speaker_lens;
pub use audio_speaker_lens::AudioSpeakerLens;

// ── 남성청결제 렌즈 (BT-1157) ──
pub mod mens_intimate_cleanser_lens;
pub use mens_intimate_cleanser_lens::MensIntimateCleanserLens;

// ── 여성청결제 렌즈 (BT-1158) ──
pub mod womens_intimate_cleanser_lens;
pub use womens_intimate_cleanser_lens::WomensIntimateCleanserLens;
