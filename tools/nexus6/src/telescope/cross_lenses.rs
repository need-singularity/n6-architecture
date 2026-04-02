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
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_project_lens_count() {
        let entries = cross_project_lens_entries();
        assert_eq!(entries.len(), 40, "Must have exactly 40 cross-project lenses");
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
