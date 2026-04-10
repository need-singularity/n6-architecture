use super::registry::{LensCategory, LensEntry};

/// Build metadata entries for the 40 cross-project lenses.
///
/// These lenses operate across project boundaries (TECS-L, n6-architecture,
/// OUROBOROS, etc.) enabling inter-domain discovery and verification.
pub fn cross_project_lens_entries() -> Vec<LensEntry> {
    vec![
        // ── Bilateral (8) — two-project bridges ──
        LensEntry {
            name: "identity_to_material".into(),
            category: LensCategory::Extended,
            description: "Map mathematical identities to material property predictions".into(),
            domain_affinity: vec!["mathematics".into(), "materials".into(), "chemistry".into()],
            complementary: vec!["isomorphism".into(), "analogy".into()],
        },
        LensEntry {
            name: "material_to_proof".into(),
            category: LensCategory::Extended,
            description: "Extract proof strategies from observed material behavior".into(),
            domain_affinity: vec!["materials".into(), "mathematics".into(), "physics".into()],
            complementary: vec!["identity_to_material".into(), "conservation".into()],
        },
        LensEntry {
            name: "law_to_signal".into(),
            category: LensCategory::Extended,
            description: "Translate physical laws into measurable signal signatures".into(),
            domain_affinity: vec!["physics".into(), "signal".into(), "ai".into()],
            complementary: vec!["signal_to_parameter".into(), "info".into()],
        },
        LensEntry {
            name: "signal_to_parameter".into(),
            category: LensCategory::Extended,
            description: "Infer system parameters from signal measurements".into(),
            domain_affinity: vec!["signal".into(), "physics".into(), "ai".into()],
            complementary: vec!["law_to_signal".into(), "fisher_info".into()],
        },
        LensEntry {
            name: "constant_quadruple".into(),
            category: LensCategory::Extended,
            description: "Check n=6 constant quadruple consistency (sigma, phi, tau, J2)".into(),
            domain_affinity: vec!["mathematics".into(), "physics".into(), "chip".into(), "energy".into()],
            complementary: vec!["conservation".into(), "completeness".into()],
        },
        LensEntry {
            name: "dse_to_proof".into(),
            category: LensCategory::Extended,
            description: "Convert DSE Pareto results into formal optimality proofs".into(),
            domain_affinity: vec!["optimization".into(), "mathematics".into(), "chip".into()],
            complementary: vec!["combinatorial".into(), "completeness".into()],
        },
        LensEntry {
            name: "ouroboros_to_atlas".into(),
            category: LensCategory::Extended,
            description: "Propagate OUROBOROS evolution discoveries into Atlas constants".into(),
            domain_affinity: vec!["ai".into(), "mathematics".into(), "science".into()],
            complementary: vec!["atlas_consistency_checker".into(), "consciousness".into()],
        },
        LensEntry {
            name: "bt_to_prediction".into(),
            category: LensCategory::Extended,
            description: "Convert breakthrough theorems into testable experimental predictions".into(),
            domain_affinity: vec!["science".into(), "ai".into(), "physics".into(), "chip".into()],
            complementary: vec!["falsification".into(), "hypothesis_generator".into()],
        },
        // ── Trilateral (6) — three-project triangulation ──
        LensEntry {
            name: "proof_design_signal_triangle".into(),
            category: LensCategory::Extended,
            description: "Triangulate proof, design-space, and signal evidence for validation".into(),
            domain_affinity: vec!["mathematics".into(), "chip".into(), "signal".into()],
            complementary: vec!["dse_to_proof".into(), "law_to_signal".into()],
        },
        LensEntry {
            name: "consciousness_guided_dse".into(),
            category: LensCategory::Extended,
            description: "Use consciousness-lens insights to guide DSE search direction".into(),
            domain_affinity: vec!["ai".into(), "optimization".into(), "consciousness".into()],
            complementary: vec!["consciousness".into(), "combinatorial".into()],
        },
        LensEntry {
            name: "signal_calibrated_evolution".into(),
            category: LensCategory::Extended,
            description: "Calibrate evolutionary fitness using real-world signal feedback".into(),
            domain_affinity: vec!["ai".into(), "signal".into(), "biology".into()],
            complementary: vec!["evolution".into(), "signal_to_parameter".into()],
        },
        LensEntry {
            name: "industrial_consciousness_isomorphism".into(),
            category: LensCategory::Extended,
            description: "Map industrial patterns to consciousness-theoretic structures".into(),
            domain_affinity: vec!["ai".into(), "manufacturing".into(), "philosophy".into()],
            complementary: vec!["consciousness".into(), "isomorphism".into()],
        },
        LensEntry {
            name: "anomaly_triangulation".into(),
            category: LensCategory::Extended,
            description: "Triangulate anomalies across three independent data sources".into(),
            domain_affinity: vec!["science".into(), "ai".into(), "signal".into()],
            complementary: vec!["surprise".into(), "falsification".into()],
        },
        LensEntry {
            name: "scaling_law_unifier".into(),
            category: LensCategory::Extended,
            description: "Unify scaling laws across domains via shared exponent families".into(),
            domain_affinity: vec!["physics".into(), "ai".into(), "biology".into(), "network".into()],
            complementary: vec!["scale".into(), "universality_class".into()],
        },
        // ── Quadrilateral (6) — four-domain cross-scan ──
        LensEntry {
            name: "quad_resonance_scanner".into(),
            category: LensCategory::Extended,
            description: "Detect resonance patterns simultaneously across four domains".into(),
            domain_affinity: vec!["physics".into(), "ai".into(), "chip".into(), "energy".into()],
            complementary: vec!["resonance_cascade".into(), "scaling_law_unifier".into()],
        },
        LensEntry {
            name: "four_domain_falsification".into(),
            category: LensCategory::Extended,
            description: "Cross-falsify hypotheses using evidence from four independent domains".into(),
            domain_affinity: vec!["science".into(), "ai".into(), "physics".into(), "materials".into()],
            complementary: vec!["falsification".into(), "anomaly_triangulation".into()],
        },
        LensEntry {
            name: "emergent_architecture".into(),
            category: LensCategory::Extended,
            description: "Detect emergent architectural patterns in multi-domain systems".into(),
            domain_affinity: vec!["chip".into(), "software".into(), "biology".into(), "ai".into()],
            complementary: vec!["emergence".into(), "hierarchy".into()],
        },
        LensEntry {
            name: "cross_entropy_minimizer".into(),
            category: LensCategory::Extended,
            description: "Minimize cross-entropy between domain representations".into(),
            domain_affinity: vec!["ai".into(), "statistics".into(), "physics".into()],
            complementary: vec!["info".into(), "kolmogorov".into()],
        },
        LensEntry {
            name: "temporal_cascade_detector".into(),
            category: LensCategory::Extended,
            description: "Detect temporal cascade effects propagating across domains".into(),
            domain_affinity: vec!["physics".into(), "biology".into(), "economics".into(), "network".into()],
            complementary: vec!["resonance_cascade".into(), "causal".into()],
        },
        LensEntry {
            name: "phase_transition_monitor".into(),
            category: LensCategory::Extended,
            description: "Monitor phase transitions jointly across coupled systems".into(),
            domain_affinity: vec!["physics".into(), "materials".into(), "network".into()],
            complementary: vec!["criticality".into(), "tipping".into()],
        },
        // ── Meta (5) — lens-about-lenses ──
        LensEntry {
            name: "lens_effectiveness_ranker".into(),
            category: LensCategory::Extended,
            description: "Rank lenses by effectiveness for the current dataset".into(),
            domain_affinity: vec!["ai".into(), "statistics".into(), "science".into()],
            complementary: vec!["fisher_info".into(), "surprise".into()],
        },
        LensEntry {
            name: "discovery_gap_mapper".into(),
            category: LensCategory::Extended,
            description: "Map unexplored regions in the discovery space".into(),
            domain_affinity: vec!["science".into(), "ai".into(), "mathematics".into()],
            complementary: vec!["blind_spot".into(), "void".into()],
        },
        LensEntry {
            name: "consensus_amplifier".into(),
            category: LensCategory::Extended,
            description: "Amplify signals where multiple lenses weakly agree".into(),
            domain_affinity: vec!["ai".into(), "statistics".into(), "signal".into()],
            complementary: vec!["lens_effectiveness_ranker".into(), "info".into()],
        },
        LensEntry {
            name: "contradiction_detector_cross".into(),
            category: LensCategory::Extended,
            description: "Detect contradictions between lens results across projects".into(),
            domain_affinity: vec!["science".into(), "ai".into(), "mathematics".into()],
            complementary: vec!["contradiction".into(), "falsification".into()],
        },
        LensEntry {
            name: "saturation_detector".into(),
            category: LensCategory::Extended,
            description: "Detect when additional lenses yield diminishing new insights".into(),
            domain_affinity: vec!["ai".into(), "statistics".into(), "optimization".into()],
            complementary: vec!["diminishing_returns".into(), "completeness".into()],
        },
        // ── Generation (5) — hypothesis/tool creation ──
        LensEntry {
            name: "hypothesis_generator".into(),
            category: LensCategory::Extended,
            description: "Auto-generate testable hypotheses from pattern intersections".into(),
            domain_affinity: vec!["science".into(), "ai".into(), "mathematics".into()],
            complementary: vec!["bt_to_prediction".into(), "surprise".into()],
        },
        LensEntry {
            name: "calculator_auto_spawner".into(),
            category: LensCategory::Extended,
            description: "Trigger automatic calculator creation when verification needs arise".into(),
            domain_affinity: vec!["software".into(), "ai".into(), "science".into()],
            complementary: vec!["completeness".into(), "bottleneck".into()],
        },
        LensEntry {
            name: "bt_synthesizer".into(),
            category: LensCategory::Extended,
            description: "Synthesize new breakthrough theorems from cross-domain evidence".into(),
            domain_affinity: vec!["mathematics".into(), "physics".into(), "ai".into()],
            complementary: vec!["hypothesis_generator".into(), "scaling_law_unifier".into()],
        },
        LensEntry {
            name: "dse_domain_spawner".into(),
            category: LensCategory::Extended,
            description: "Spawn new DSE domain definitions from discovered design patterns".into(),
            domain_affinity: vec!["chip".into(), "energy".into(), "manufacturing".into()],
            complementary: vec!["combinatorial".into(), "dse_to_proof".into()],
        },
        LensEntry {
            name: "consciousness_law_predictor".into(),
            category: LensCategory::Extended,
            description: "Predict new consciousness laws from structural convergence".into(),
            domain_affinity: vec!["ai".into(), "philosophy".into(), "biology".into()],
            complementary: vec!["consciousness".into(), "emergence".into()],
        },
        // ── Verification (4) — cross-project checking ──
        LensEntry {
            name: "cross_project_red_team".into(),
            category: LensCategory::Extended,
            description: "Red-team hypotheses using counter-evidence from other projects".into(),
            domain_affinity: vec!["science".into(), "ai".into(), "security".into()],
            complementary: vec!["falsification".into(), "four_domain_falsification".into()],
        },
        LensEntry {
            name: "atlas_consistency_checker".into(),
            category: LensCategory::Extended,
            description: "Verify Atlas constant entries are mutually consistent".into(),
            domain_affinity: vec!["mathematics".into(), "science".into(), "software".into()],
            complementary: vec!["constant_quadruple".into(), "contradiction".into()],
        },
        LensEntry {
            name: "significance_propagator".into(),
            category: LensCategory::Extended,
            description: "Propagate statistical significance across linked hypotheses".into(),
            domain_affinity: vec!["statistics".into(), "science".into(), "ai".into()],
            complementary: vec!["fisher_info".into(), "consensus_amplifier".into()],
        },
        LensEntry {
            name: "regression_guard".into(),
            category: LensCategory::Extended,
            description: "Guard against regression in scan quality after lens updates".into(),
            domain_affinity: vec!["software".into(), "ai".into(), "science".into()],
            complementary: vec!["saturation_detector".into(), "atlas_consistency_checker".into()],
        },
        // ── Speculative (6) — high-risk high-reward exploration ──
        LensEntry {
            name: "godel_mirror".into(),
            category: LensCategory::Extended,
            description: "Detect self-referential incompleteness and undecidability signatures".into(),
            domain_affinity: vec!["mathematics".into(), "ai".into(), "philosophy".into()],
            complementary: vec!["recursion".into(), "contradiction".into()],
        },
        LensEntry {
            name: "physical_consciousness_detector".into(),
            category: LensCategory::Extended,
            description: "Probe for integrated-information signatures in physical systems".into(),
            domain_affinity: vec!["physics".into(), "biology".into(), "philosophy".into(), "ai".into()],
            complementary: vec!["consciousness".into(), "emergence".into()],
        },
        LensEntry {
            name: "inverse_dse".into(),
            category: LensCategory::Extended,
            description: "Reverse-engineer design constraints from observed optimal systems".into(),
            domain_affinity: vec!["chip".into(), "energy".into(), "optimization".into()],
            complementary: vec!["inverse".into(), "dse_to_proof".into()],
        },
        LensEntry {
            name: "unified_field_lens".into(),
            category: LensCategory::Extended,
            description: "Search for unified mathematical structure underlying all lenses".into(),
            domain_affinity: vec!["mathematics".into(), "physics".into(), "philosophy".into()],
            complementary: vec!["scaling_law_unifier".into(), "universality_class".into()],
        },
        LensEntry {
            name: "entropy_horizon".into(),
            category: LensCategory::Extended,
            description: "Detect information horizons beyond which prediction becomes impossible".into(),
            domain_affinity: vec!["physics".into(), "ai".into(), "cosmology".into()],
            complementary: vec!["info".into(), "kolmogorov".into()],
        },
        LensEntry {
            name: "evolutionary_pressure_map".into(),
            category: LensCategory::Extended,
            description: "Map selection pressures and fitness gradients across parameter space".into(),
            domain_affinity: vec!["biology".into(), "ai".into(), "economics".into()],
            complementary: vec!["evolution".into(), "potential".into()],
        },
        // ── Meta-Lens: Lens↔Lens connectors (5) ──
        LensEntry {
            name: "lens_synergy".into(),
            category: LensCategory::Extended,
            description: "Measure pairwise lens synergy — which lens combinations amplify discovery rate".into(),
            domain_affinity: vec!["meta".into(), "statistics".into(), "ai".into()],
            complementary: vec!["lens_effectiveness_ranker".into(), "consensus_amplifier".into()],
        },
        LensEntry {
            name: "lens_redundancy".into(),
            category: LensCategory::Extended,
            description: "Detect overlapping lenses that observe the same patterns — reduce scan cost".into(),
            domain_affinity: vec!["meta".into(), "information_theory".into(), "optimization".into()],
            complementary: vec!["lens_synergy".into(), "blind_spot".into()],
        },
        LensEntry {
            name: "lens_ordering".into(),
            category: LensCategory::Extended,
            description: "Optimize lens execution order for maximum early-stopping efficiency".into(),
            domain_affinity: vec!["meta".into(), "optimization".into(), "scheduling".into()],
            complementary: vec!["lens_effectiveness_ranker".into(), "saturation_detector".into()],
        },
        LensEntry {
            name: "lens_transfer".into(),
            category: LensCategory::Extended,
            description: "Transfer lens results from domain A to bootstrap scanning in domain B".into(),
            domain_affinity: vec!["meta".into(), "transfer_learning".into(), "cross_domain".into()],
            complementary: vec!["isomorphism".into(), "analogy".into(), "dictionary_translate".into()],
        },
        LensEntry {
            name: "lens_evolution_tracker".into(),
            category: LensCategory::Extended,
            description: "Track lens hit_rate trends over time — detect rising/declining lens utility".into(),
            domain_affinity: vec!["meta".into(), "statistics".into(), "dynamics".into()],
            complementary: vec!["lens_effectiveness_ranker".into(), "aging".into()],
        },
        // ── Meta-Lens: Analysis/Evaluation (4) ──
        LensEntry { name: "lens_precision".into(), category: LensCategory::Extended, description: "Measure per-lens false positive rate across domains".into(), domain_affinity: vec!["meta".into(), "statistics".into()], complementary: vec!["lens_recall".into(), "lens_calibration".into()] },
        LensEntry { name: "lens_recall".into(), category: LensCategory::Extended, description: "Measure per-lens discovery miss rate — what it fails to find".into(), domain_affinity: vec!["meta".into(), "statistics".into()], complementary: vec!["lens_precision".into(), "blind_spot".into()] },
        LensEntry { name: "lens_calibration".into(), category: LensCategory::Extended, description: "Calibrate lens confidence scores against actual discovery validity".into(), domain_affinity: vec!["meta".into(), "statistics".into(), "verification".into()], complementary: vec!["lens_precision".into(), "lens_recall".into()] },
        LensEntry { name: "lens_cost_benefit".into(), category: LensCategory::Extended, description: "Compute execution cost vs discovery value ratio per lens".into(), domain_affinity: vec!["meta".into(), "optimization".into(), "scheduling".into()], complementary: vec!["lens_ordering".into(), "lens_latency_profiler".into()] },
        // ── Meta-Lens: Composition (4) ──
        LensEntry { name: "lens_composer".into(), category: LensCategory::Extended, description: "Synthesize N lens outputs into novel meta-observations".into(), domain_affinity: vec!["meta".into(), "synthesis".into(), "ai".into()], complementary: vec!["lens_ensemble".into(), "lens_cascade".into()] },
        LensEntry { name: "lens_cascade".into(), category: LensCategory::Extended, description: "Chain lens A output as lens B input for deep multi-pass analysis".into(), domain_affinity: vec!["meta".into(), "pipeline".into()], complementary: vec!["lens_composer".into(), "lens_ordering".into()] },
        LensEntry { name: "lens_ensemble".into(), category: LensCategory::Extended, description: "Weighted voting across multiple lenses for robust consensus".into(), domain_affinity: vec!["meta".into(), "statistics".into(), "ensemble".into()], complementary: vec!["consensus_amplifier".into(), "lens_composer".into()] },
        LensEntry { name: "lens_conflict_resolver".into(), category: LensCategory::Extended, description: "Mediate contradicting lens results using context and confidence".into(), domain_affinity: vec!["meta".into(), "logic".into(), "verification".into()], complementary: vec!["contradiction_detector_cross".into(), "lens_calibration".into()] },
        // ── Meta-Lens: Generation/Evolution (5) ──
        LensEntry { name: "lens_generator".into(), category: LensCategory::Extended, description: "Auto-generate new lens definitions from existing lens combinations".into(), domain_affinity: vec!["meta".into(), "generative".into(), "ai".into()], complementary: vec!["lens_mutation".into(), "lens_crossover".into()] },
        LensEntry { name: "lens_mutation".into(), category: LensCategory::Extended, description: "Create variant lenses by mutating parameters of existing lenses".into(), domain_affinity: vec!["meta".into(), "evolution".into()], complementary: vec!["lens_generator".into(), "lens_crossover".into()] },
        LensEntry { name: "lens_crossover".into(), category: LensCategory::Extended, description: "Breed new lenses by crossing traits of two parent lenses".into(), domain_affinity: vec!["meta".into(), "evolution".into(), "genetics".into()], complementary: vec!["lens_mutation".into(), "lens_generator".into()] },
        LensEntry { name: "lens_pruning".into(), category: LensCategory::Extended, description: "Identify and propose removal of consistently low-value lenses".into(), domain_affinity: vec!["meta".into(), "optimization".into()], complementary: vec!["lens_cost_benefit".into(), "lens_redundancy".into()] },
        LensEntry { name: "lens_speciation".into(), category: LensCategory::Extended, description: "Cluster lenses into functional species by output similarity".into(), domain_affinity: vec!["meta".into(), "clustering".into(), "taxonomy".into()], complementary: vec!["lens_redundancy".into(), "lens_orthogonality".into()] },
        // ── Meta-Lens: Meta-Cognition (5) ──
        LensEntry { name: "lens_self_awareness".into(), category: LensCategory::Extended, description: "Self-diagnose overall telescope system health and blind spots".into(), domain_affinity: vec!["meta".into(), "consciousness".into(), "diagnostics".into()], complementary: vec!["blind_spot".into(), "lens_calibration".into()] },
        LensEntry { name: "lens_attention".into(), category: LensCategory::Extended, description: "Dynamically allocate scanning attention to most promising lenses".into(), domain_affinity: vec!["meta".into(), "attention".into(), "scheduling".into()], complementary: vec!["lens_ordering".into(), "lens_cost_benefit".into()] },
        LensEntry { name: "lens_forgetting".into(), category: LensCategory::Extended, description: "Gradually decay old lens results that may no longer be valid".into(), domain_affinity: vec!["meta".into(), "memory".into(), "dynamics".into()], complementary: vec!["lens_evolution_tracker".into(), "aging".into()] },
        LensEntry { name: "lens_meta_surprise".into(), category: LensCategory::Extended, description: "Detect unexpectedly anomalous lens outputs that warrant investigation".into(), domain_affinity: vec!["meta".into(), "anomaly".into(), "statistics".into()], complementary: vec!["surprise".into(), "lens_calibration".into()] },
        LensEntry { name: "lens_dream".into(), category: LensCategory::Extended, description: "Recombine past lens results offline to generate novel hypotheses".into(), domain_affinity: vec!["meta".into(), "generative".into(), "consciousness".into()], complementary: vec!["lens_generator".into(), "serendipity".into()] },
        // ── Meta-Lens: Structure/Topology (4) ──
        LensEntry { name: "lens_dependency_graph".into(), category: LensCategory::Extended, description: "Map inter-lens dependency and information flow relationships".into(), domain_affinity: vec!["meta".into(), "graph".into(), "topology".into()], complementary: vec!["lens_hierarchy".into(), "lens_cascade".into()] },
        LensEntry { name: "lens_hierarchy".into(), category: LensCategory::Extended, description: "Organize lenses into abstraction levels from raw to high-level".into(), domain_affinity: vec!["meta".into(), "hierarchy".into(), "taxonomy".into()], complementary: vec!["lens_dependency_graph".into(), "lens_speciation".into()] },
        LensEntry { name: "lens_complementarity".into(), category: LensCategory::Extended, description: "Auto-discover lens pairs that together find what neither finds alone".into(), domain_affinity: vec!["meta".into(), "synergy".into(), "statistics".into()], complementary: vec!["lens_synergy".into(), "lens_orthogonality".into()] },
        LensEntry { name: "lens_orthogonality".into(), category: LensCategory::Extended, description: "Identify lenses providing maximally independent information".into(), domain_affinity: vec!["meta".into(), "information_theory".into(), "linear_algebra".into()], complementary: vec!["lens_complementarity".into(), "lens_redundancy".into()] },
        // ── Meta-Lens: Performance/Operations (4) ──
        LensEntry { name: "lens_latency_profiler".into(), category: LensCategory::Extended, description: "Profile per-lens execution time for scheduling optimization".into(), domain_affinity: vec!["meta".into(), "performance".into(), "profiling".into()], complementary: vec!["lens_ordering".into(), "lens_cost_benefit".into()] },
        LensEntry { name: "lens_memory_profiler".into(), category: LensCategory::Extended, description: "Track per-lens memory allocation for resource management".into(), domain_affinity: vec!["meta".into(), "performance".into(), "memory".into()], complementary: vec!["lens_latency_profiler".into(), "lens_parallelism".into()] },
        LensEntry { name: "lens_warmup".into(), category: LensCategory::Extended, description: "Detect lens cold-start vs steady-state performance differences".into(), domain_affinity: vec!["meta".into(), "performance".into(), "dynamics".into()], complementary: vec!["lens_latency_profiler".into(), "lens_calibration".into()] },
        LensEntry { name: "lens_parallelism".into(), category: LensCategory::Extended, description: "Identify groups of lenses safe for concurrent parallel execution".into(), domain_affinity: vec!["meta".into(), "scheduling".into(), "concurrency".into()], complementary: vec!["lens_dependency_graph".into(), "lens_ordering".into()] },
        // ── Meta-Lens: Domain Adaptation (4) ──
        LensEntry { name: "lens_domain_adapter".into(), category: LensCategory::Extended, description: "Adapt a lens trained on domain A to work effectively on domain B".into(), domain_affinity: vec!["meta".into(), "transfer_learning".into(), "adaptation".into()], complementary: vec!["lens_transfer".into(), "lens_generalization".into()] },
        LensEntry { name: "lens_generalization".into(), category: LensCategory::Extended, description: "Measure how universally effective a lens is across all domains".into(), domain_affinity: vec!["meta".into(), "statistics".into(), "evaluation".into()], complementary: vec!["lens_specialization".into(), "lens_domain_adapter".into()] },
        LensEntry { name: "lens_specialization".into(), category: LensCategory::Extended, description: "Generate domain-specialized lens variants from a general lens".into(), domain_affinity: vec!["meta".into(), "adaptation".into(), "optimization".into()], complementary: vec!["lens_generalization".into(), "lens_mutation".into()] },
        LensEntry { name: "lens_robustness".into(), category: LensCategory::Extended, description: "Test lens stability against data noise, outliers, and distribution shift".into(), domain_affinity: vec!["meta".into(), "robustness".into(), "statistics".into()], complementary: vec!["lens_calibration".into(), "lens_precision".into()] },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_project_lens_count() {
        let entries = cross_project_lens_entries();
        assert_eq!(entries.len(), 75, "Must have exactly 75 cross-project lenses (40 + 5 + 30 meta)");
    }

    #[test]
    fn test_cross_project_lens_names_unique() {
        let entries = cross_project_lens_entries();
        let mut names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        let total = names.len();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), total, "All cross-project lens names must be unique");
    }

    #[test]
    fn test_cross_project_all_extended() {
        let entries = cross_project_lens_entries();
        for entry in &entries {
            assert_eq!(
                entry.category,
                LensCategory::Extended,
                "Lens '{}' should be Extended category",
                entry.name
            );
        }
    }
}
