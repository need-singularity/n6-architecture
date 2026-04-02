use super::registry::{LensCategory, LensEntry};

/// Build metadata entries for the 55 accelerated engineering lenses (Part C).
///
/// These lenses cover network depth, systems engineering, music/rhythm,
/// chemistry/molecular, economics/game theory, linguistics/semiotics,
/// materials/electronics/textiles, medicine/pharmacology, deep ecology,
/// and hardware architecture domains.
pub fn accel_engineering_lens_entries() -> Vec<LensEntry> {
    vec![
        // ── Network Deep (5) ──
        LensEntry {
            name: "small_world".into(),
            category: LensCategory::Extended,
            description: "Detect small-world topology — high clustering with short path lengths".into(),
            domain_affinity: vec!["network".into(), "social".into(), "biology".into(), "ai".into()],
            complementary: vec!["network".into(), "community_detection".into()],
        },
        LensEntry {
            name: "preferential_attachment".into(),
            category: LensCategory::Extended,
            description: "Identify preferential attachment (rich-get-richer) growth dynamics".into(),
            domain_affinity: vec!["network".into(), "social".into(), "economics".into()],
            complementary: vec!["power_law".into(), "small_world".into()],
        },
        LensEntry {
            name: "community_detection".into(),
            category: LensCategory::Extended,
            description: "Partition graphs into densely connected communities via modularity optimization".into(),
            domain_affinity: vec!["network".into(), "social".into(), "biology".into()],
            complementary: vec!["small_world".into(), "centrality_measure".into()],
        },
        LensEntry {
            name: "centrality_measure".into(),
            category: LensCategory::Extended,
            description: "Rank nodes by betweenness, eigenvector, and closeness centrality".into(),
            domain_affinity: vec!["network".into(), "social".into(), "infrastructure".into()],
            complementary: vec!["community_detection".into(), "bottleneck".into()],
        },
        LensEntry {
            name: "graph_spectral".into(),
            category: LensCategory::Extended,
            description: "Analyze graph Laplacian spectrum for connectivity and clustering structure".into(),
            domain_affinity: vec!["network".into(), "mathematics".into(), "physics".into()],
            complementary: vec!["spectral_gap".into(), "centrality_measure".into()],
        },

        // ── Systems Engineering (5) ──
        LensEntry {
            name: "feedback_loop".into(),
            category: LensCategory::Extended,
            description: "Detect positive and negative feedback loops in control systems".into(),
            domain_affinity: vec!["engineering".into(), "biology".into(), "economics".into(), "climate".into()],
            complementary: vec!["pid_control".into(), "stability".into()],
        },
        LensEntry {
            name: "pid_control".into(),
            category: LensCategory::Extended,
            description: "Analyze proportional-integral-derivative control response and tuning".into(),
            domain_affinity: vec!["engineering".into(), "robotics".into(), "manufacturing".into()],
            complementary: vec!["feedback_loop".into(), "fault_tolerance".into()],
        },
        LensEntry {
            name: "queueing_theory".into(),
            category: LensCategory::Extended,
            description: "Model arrival/service rates, queue lengths, and waiting time distributions".into(),
            domain_affinity: vec!["network".into(), "manufacturing".into(), "chip".into(), "infrastructure".into()],
            complementary: vec!["bottleneck".into(), "reliability_engineering".into()],
        },
        LensEntry {
            name: "reliability_engineering".into(),
            category: LensCategory::Extended,
            description: "Compute MTBF, failure rates, and system reliability via redundancy analysis".into(),
            domain_affinity: vec!["engineering".into(), "chip".into(), "energy".into(), "aerospace".into()],
            complementary: vec!["fault_tolerance".into(), "aging".into()],
        },
        LensEntry {
            name: "fault_tolerance".into(),
            category: LensCategory::Extended,
            description: "Evaluate graceful degradation and Byzantine fault tolerance capacity".into(),
            domain_affinity: vec!["engineering".into(), "chip".into(), "network".into(), "blockchain".into()],
            complementary: vec!["reliability_engineering".into(), "feedback_loop".into()],
        },

        // ── Music / Rhythm (5) ──
        LensEntry {
            name: "harmonic_series_lens".into(),
            category: LensCategory::Extended,
            description: "Analyze overtone structure and harmonic series relationships in signals".into(),
            domain_affinity: vec!["audio".into(), "music".into(), "physics".into(), "signal".into()],
            complementary: vec!["wave".into(), "tonal_gravity".into()],
        },
        LensEntry {
            name: "rhythmic_entrainment".into(),
            category: LensCategory::Extended,
            description: "Detect phase-locking and entrainment between oscillatory rhythmic patterns".into(),
            domain_affinity: vec!["music".into(), "neuroscience".into(), "biology".into()],
            complementary: vec!["polyrhythm".into(), "harmonic_series_lens".into()],
        },
        LensEntry {
            name: "polyrhythm".into(),
            category: LensCategory::Extended,
            description: "Identify polyrhythmic layering and cross-rhythmic interference patterns".into(),
            domain_affinity: vec!["music".into(), "signal".into(), "physics".into()],
            complementary: vec!["rhythmic_entrainment".into(), "wave".into()],
        },
        LensEntry {
            name: "tonal_gravity".into(),
            category: LensCategory::Extended,
            description: "Map tonal tension/resolution fields and harmonic attraction basins".into(),
            domain_affinity: vec!["music".into(), "audio".into(), "physics".into()],
            complementary: vec!["harmonic_series_lens".into(), "potential".into()],
        },
        LensEntry {
            name: "melodic_contour".into(),
            category: LensCategory::Extended,
            description: "Extract pitch contour shapes and melodic arc classification".into(),
            domain_affinity: vec!["music".into(), "audio".into(), "linguistics".into()],
            complementary: vec!["tonal_gravity".into(), "narrative".into()],
        },

        // ── Chemistry / Molecular (6) ──
        LensEntry {
            name: "molecular_dynamics_lens".into(),
            category: LensCategory::Extended,
            description: "Simulate atomic trajectories and extract thermodynamic ensemble averages".into(),
            domain_affinity: vec!["chemistry".into(), "materials".into(), "biology".into(), "physics".into()],
            complementary: vec!["thermo".into(), "reaction_kinetics".into()],
        },
        LensEntry {
            name: "reaction_kinetics".into(),
            category: LensCategory::Extended,
            description: "Model reaction rate laws, order determination, and rate constant fitting".into(),
            domain_affinity: vec!["chemistry".into(), "biology".into(), "materials".into()],
            complementary: vec!["activation_energy_lens".into(), "catalysis".into()],
        },
        LensEntry {
            name: "activation_energy_lens".into(),
            category: LensCategory::Extended,
            description: "Measure Arrhenius activation barriers and transition state energetics".into(),
            domain_affinity: vec!["chemistry".into(), "materials".into(), "energy".into()],
            complementary: vec!["barrier".into(), "reaction_kinetics".into()],
        },
        LensEntry {
            name: "chirality_synthesis".into(),
            category: LensCategory::Extended,
            description: "Analyze enantioselective synthesis pathways and chiral resolution efficiency".into(),
            domain_affinity: vec!["chemistry".into(), "biology".into(), "pharma".into()],
            complementary: vec!["chirality".into(), "molecular_dynamics_lens".into()],
        },
        LensEntry {
            name: "spherification".into(),
            category: LensCategory::Extended,
            description: "Detect sphere-forming encapsulation and droplet gelation dynamics".into(),
            domain_affinity: vec!["chemistry".into(), "food_science".into(), "materials".into()],
            complementary: vec!["gelation".into(), "interface".into()],
        },
        LensEntry {
            name: "gelation".into(),
            category: LensCategory::Extended,
            description: "Identify sol-gel transitions, network percolation in polymer crosslinking".into(),
            domain_affinity: vec!["chemistry".into(), "materials".into(), "biology".into()],
            complementary: vec!["percolation".into(), "spherification".into()],
        },

        // ── Economics / Game Theory (5) ──
        LensEntry {
            name: "nash_equilibrium".into(),
            category: LensCategory::Extended,
            description: "Find Nash equilibria and dominant strategies in multi-agent interactions".into(),
            domain_affinity: vec!["economics".into(), "ai".into(), "network".into(), "security".into()],
            complementary: vec!["mechanism_design".into(), "auction_theory".into()],
        },
        LensEntry {
            name: "auction_theory".into(),
            category: LensCategory::Extended,
            description: "Analyze bidding strategies, revenue equivalence, and auction format efficiency".into(),
            domain_affinity: vec!["economics".into(), "finance".into(), "network".into()],
            complementary: vec!["nash_equilibrium".into(), "mechanism_design".into()],
        },
        LensEntry {
            name: "mechanism_design".into(),
            category: LensCategory::Extended,
            description: "Design incentive-compatible mechanisms achieving desired social outcomes".into(),
            domain_affinity: vec!["economics".into(), "ai".into(), "blockchain".into()],
            complementary: vec!["nash_equilibrium".into(), "behavioral_economics".into()],
        },
        LensEntry {
            name: "market_microstructure".into(),
            category: LensCategory::Extended,
            description: "Analyze order flow, spread dynamics, and price formation at tick level".into(),
            domain_affinity: vec!["finance".into(), "economics".into(), "network".into()],
            complementary: vec!["auction_theory".into(), "queueing_theory".into()],
        },
        LensEntry {
            name: "behavioral_economics".into(),
            category: LensCategory::Extended,
            description: "Detect cognitive biases, loss aversion, and bounded rationality in decisions".into(),
            domain_affinity: vec!["economics".into(), "psychology".into(), "ai".into()],
            complementary: vec!["mechanism_design".into(), "narrative".into()],
        },

        // ── Linguistics / Semiotics (6) ──
        LensEntry {
            name: "syntax_tree".into(),
            category: LensCategory::Extended,
            description: "Parse hierarchical syntactic structure and constituency/dependency trees".into(),
            domain_affinity: vec!["linguistics".into(), "ai".into(), "software".into()],
            complementary: vec!["semantic_field".into(), "hierarchy".into()],
        },
        LensEntry {
            name: "semantic_field".into(),
            category: LensCategory::Extended,
            description: "Map semantic proximity, word fields, and conceptual clustering in language".into(),
            domain_affinity: vec!["linguistics".into(), "ai".into(), "education".into()],
            complementary: vec!["syntax_tree".into(), "pragmatic_context".into()],
        },
        LensEntry {
            name: "pragmatic_context".into(),
            category: LensCategory::Extended,
            description: "Analyze utterance meaning in context — implicature, presupposition, deixis".into(),
            domain_affinity: vec!["linguistics".into(), "ai".into(), "social".into()],
            complementary: vec!["semantic_field".into(), "code_switching".into()],
        },
        LensEntry {
            name: "morphological_decompose".into(),
            category: LensCategory::Extended,
            description: "Decompose words into morphemes — stems, prefixes, suffixes, inflections".into(),
            domain_affinity: vec!["linguistics".into(), "ai".into(), "education".into()],
            complementary: vec!["syntax_tree".into(), "signifier_signified".into()],
        },
        LensEntry {
            name: "signifier_signified".into(),
            category: LensCategory::Extended,
            description: "Analyze sign-referent relationships and semiotic arbitrariness".into(),
            domain_affinity: vec!["linguistics".into(), "philosophy".into(), "ai".into()],
            complementary: vec!["morphological_decompose".into(), "analogy".into()],
        },
        LensEntry {
            name: "code_switching".into(),
            category: LensCategory::Extended,
            description: "Detect code-switching between registers, languages, or symbolic systems".into(),
            domain_affinity: vec!["linguistics".into(), "social".into(), "ai".into()],
            complementary: vec!["pragmatic_context".into(), "narrative".into()],
        },

        // ── Materials / Electronics / Textiles (9) ──
        LensEntry {
            name: "crystal_structure_lens".into(),
            category: LensCategory::Extended,
            description: "Classify crystal systems, space groups, and unit cell symmetries".into(),
            domain_affinity: vec!["materials".into(), "chemistry".into(), "physics".into()],
            complementary: vec!["mirror".into(), "grain_boundary_lens".into()],
        },
        LensEntry {
            name: "grain_boundary_lens".into(),
            category: LensCategory::Extended,
            description: "Analyze grain boundary character, misorientation angles, and segregation".into(),
            domain_affinity: vec!["materials".into(), "chip".into(), "energy".into()],
            complementary: vec!["crystal_structure_lens".into(), "defect".into()],
        },
        LensEntry {
            name: "dislocation_dynamics".into(),
            category: LensCategory::Extended,
            description: "Track dislocation motion, pile-up, and strain hardening mechanisms".into(),
            domain_affinity: vec!["materials".into(), "manufacturing".into(), "physics".into()],
            complementary: vec!["grain_boundary_lens".into(), "defect".into()],
        },
        LensEntry {
            name: "phase_transformation".into(),
            category: LensCategory::Extended,
            description: "Detect solid-state phase transformations — martensitic, diffusive, order-disorder".into(),
            domain_affinity: vec!["materials".into(), "chemistry".into(), "energy".into()],
            complementary: vec!["thermo".into(), "crystal_structure_lens".into()],
        },
        LensEntry {
            name: "circuit_topology".into(),
            category: LensCategory::Extended,
            description: "Analyze circuit graph topology — loops, cutsets, and network theorems".into(),
            domain_affinity: vec!["electronics".into(), "chip".into(), "energy".into()],
            complementary: vec!["impedance_matching".into(), "network".into()],
        },
        LensEntry {
            name: "impedance_matching".into(),
            category: LensCategory::Extended,
            description: "Optimize impedance matching for maximum power transfer and minimal reflection".into(),
            domain_affinity: vec!["electronics".into(), "audio".into(), "energy".into(), "signal".into()],
            complementary: vec!["circuit_topology".into(), "signal_integrity_lens".into()],
        },
        LensEntry {
            name: "signal_integrity_lens".into(),
            category: LensCategory::Extended,
            description: "Evaluate signal integrity — crosstalk, jitter, eye diagram, and ISI".into(),
            domain_affinity: vec!["electronics".into(), "chip".into(), "network".into()],
            complementary: vec!["impedance_matching".into(), "circuit_topology".into()],
        },
        LensEntry {
            name: "weave_pattern".into(),
            category: LensCategory::Extended,
            description: "Classify textile weave structures — plain, twill, satin, and composite layups".into(),
            domain_affinity: vec!["textiles".into(), "materials".into(), "manufacturing".into()],
            complementary: vec!["fiber_tension".into(), "topology".into()],
        },
        LensEntry {
            name: "fiber_tension".into(),
            category: LensCategory::Extended,
            description: "Model fiber stress-strain behavior, crimp, and tensile load distribution".into(),
            domain_affinity: vec!["textiles".into(), "materials".into(), "manufacturing".into()],
            complementary: vec!["weave_pattern".into(), "dislocation_dynamics".into()],
        },

        // ── Medicine / Pharmacology (4) ──
        LensEntry {
            name: "pharmacokinetics".into(),
            category: LensCategory::Extended,
            description: "Model ADME — absorption, distribution, metabolism, excretion drug kinetics".into(),
            domain_affinity: vec!["pharma".into(), "biology".into(), "chemistry".into()],
            complementary: vec!["dose_response".into(), "reaction_kinetics".into()],
        },
        LensEntry {
            name: "dose_response".into(),
            category: LensCategory::Extended,
            description: "Fit dose-response curves — EC50, Hill coefficient, therapeutic window".into(),
            domain_affinity: vec!["pharma".into(), "biology".into(), "chemistry".into()],
            complementary: vec!["pharmacokinetics".into(), "biomarker_detection".into()],
        },
        LensEntry {
            name: "biomarker_detection".into(),
            category: LensCategory::Extended,
            description: "Identify and validate biomarkers for disease diagnosis and drug response".into(),
            domain_affinity: vec!["biology".into(), "pharma".into(), "ai".into()],
            complementary: vec!["dose_response".into(), "clinical_trial".into()],
        },
        LensEntry {
            name: "clinical_trial".into(),
            category: LensCategory::Extended,
            description: "Analyze clinical trial design — randomization, blinding, endpoint statistics".into(),
            domain_affinity: vec!["pharma".into(), "statistics".into(), "biology".into()],
            complementary: vec!["biomarker_detection".into(), "false_discovery_rate".into()],
        },

        // ── Deep Ecology (5) ──
        LensEntry {
            name: "trophic_cascade".into(),
            category: LensCategory::Extended,
            description: "Trace top-down trophic cascades through food web energy flow".into(),
            domain_affinity: vec!["ecology".into(), "biology".into(), "climate".into()],
            complementary: vec!["keystone_species".into(), "hierarchy".into()],
        },
        LensEntry {
            name: "keystone_species".into(),
            category: LensCategory::Extended,
            description: "Identify keystone elements whose removal triggers disproportionate collapse".into(),
            domain_affinity: vec!["ecology".into(), "network".into(), "economics".into()],
            complementary: vec!["trophic_cascade".into(), "centrality_measure".into()],
        },
        LensEntry {
            name: "island_biogeography".into(),
            category: LensCategory::Extended,
            description: "Apply island biogeography theory — area-species, immigration-extinction balance".into(),
            domain_affinity: vec!["ecology".into(), "biology".into(), "network".into()],
            complementary: vec!["carrying_capacity".into(), "niche".into()],
        },
        LensEntry {
            name: "succession_deep".into(),
            category: LensCategory::Extended,
            description: "Model ecological succession stages — pioneer to climax community dynamics".into(),
            domain_affinity: vec!["ecology".into(), "biology".into(), "climate".into()],
            complementary: vec!["ecosystem_resilience".into(), "evolution".into()],
        },
        LensEntry {
            name: "ecosystem_resilience".into(),
            category: LensCategory::Extended,
            description: "Measure ecosystem resilience — recovery rate, resistance, and adaptive capacity".into(),
            domain_affinity: vec!["ecology".into(), "climate".into(), "biology".into()],
            complementary: vec!["succession_deep".into(), "stability".into()],
        },

        // ── Hardware Architecture (5) ──
        LensEntry {
            name: "systolic_array".into(),
            category: LensCategory::Extended,
            description: "Analyze systolic array data flow — PE utilization, latency, and throughput".into(),
            domain_affinity: vec!["chip".into(), "ai".into(), "manufacturing".into()],
            complementary: vec!["dataflow_architecture".into(), "near_memory_compute".into()],
        },
        LensEntry {
            name: "dataflow_architecture".into(),
            category: LensCategory::Extended,
            description: "Model dataflow graph execution — token firing, pipeline balance, throughput".into(),
            domain_affinity: vec!["chip".into(), "ai".into(), "software".into()],
            complementary: vec!["systolic_array".into(), "bottleneck".into()],
        },
        LensEntry {
            name: "near_memory_compute".into(),
            category: LensCategory::Extended,
            description: "Evaluate processing-in/near-memory efficiency — bandwidth savings, energy reduction".into(),
            domain_affinity: vec!["chip".into(), "ai".into(), "energy".into()],
            complementary: vec!["dataflow_architecture".into(), "photonic_compute".into()],
        },
        LensEntry {
            name: "photonic_compute".into(),
            category: LensCategory::Extended,
            description: "Analyze photonic computing elements — MZI meshes, optical energy, latency".into(),
            domain_affinity: vec!["chip".into(), "physics".into(), "energy".into(), "ai".into()],
            complementary: vec!["near_memory_compute".into(), "neuromorphic_chip".into()],
        },
        LensEntry {
            name: "neuromorphic_chip".into(),
            category: LensCategory::Extended,
            description: "Model neuromorphic spike-based computation — event-driven, energy per spike".into(),
            domain_affinity: vec!["chip".into(), "ai".into(), "neuroscience".into(), "energy".into()],
            complementary: vec!["photonic_compute".into(), "systolic_array".into()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_accel_engineering_lens_count() {
        let entries = accel_engineering_lens_entries();
        assert_eq!(entries.len(), 55, "Must have exactly 55 accel engineering lenses (Part C)");
    }

    #[test]
    fn test_accel_engineering_lens_names_unique() {
        let entries = accel_engineering_lens_entries();
        let names: HashSet<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names.len(), entries.len(), "All accel engineering lens names must be unique");
    }

    #[test]
    fn test_accel_engineering_all_extended() {
        let entries = accel_engineering_lens_entries();
        for entry in &entries {
            assert_eq!(
                entry.category,
                super::super::registry::LensCategory::Extended,
                "Lens '{}' should be Extended category",
                entry.name
            );
        }
    }

    #[test]
    fn test_accel_engineering_no_empty_fields() {
        let entries = accel_engineering_lens_entries();
        for entry in &entries {
            assert!(!entry.name.is_empty(), "Lens name must not be empty");
            assert!(!entry.description.is_empty(), "Lens '{}' description must not be empty", entry.name);
            assert!(!entry.domain_affinity.is_empty(), "Lens '{}' must have at least one domain affinity", entry.name);
            assert!(!entry.complementary.is_empty(), "Lens '{}' must have at least one complementary lens", entry.name);
        }
    }
}
