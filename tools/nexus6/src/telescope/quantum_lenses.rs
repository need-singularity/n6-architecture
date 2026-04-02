use super::registry::{LensCategory, LensEntry};

/// Build metadata entries for 30 quantum-mechanics lenses + 8 topology-evolution lenses = 38 total.
///
/// Groups:
/// - Fundamental QM (8): wave function, Born rule, uncertainty, superposition, tunneling, Pauli, quantum numbers, selection rules
/// - Quantum Information (6): channel, error correction, teleportation, Bell inequality, QKD, no-cloning
/// - Many-Body Quantum (6): Fock space, fermion/boson statistics, phonon, Cooper pair, quasiparticle
/// - Quantum Field Theory (5): Feynman diagrams, renormalization, vacuum, anomaly, path integral
/// - Quantum Computing (5): qubit gate, circuit, annealing, variational, supremacy
/// - Topology Evolution (8): genus tracking, Betti evolution, Morse theory, cobordism flow, handle attachment, surgery transition, persistent homology drift, topological phase transition
pub fn quantum_topology_lens_entries() -> Vec<LensEntry> {
    vec![
        // ══════════════════════════════════════════
        // Fundamental Quantum Mechanics (8)
        // ══════════════════════════════════════════
        LensEntry {
            name: "wave_function".into(),
            category: LensCategory::Extended,
            description: "Analyze wave function amplitude and phase distribution patterns".into(),
            domain_affinity: vec!["quantum".into(), "physics".into(), "chemistry".into()],
            complementary: vec!["born_probability".into(), "superposition".into()],
        },
        LensEntry {
            name: "born_probability".into(),
            category: LensCategory::Extended,
            description: "Map |ψ|² probability distributions and localization patterns".into(),
            domain_affinity: vec!["quantum".into(), "statistics".into(), "physics".into()],
            complementary: vec!["wave_function".into(), "uncertainty_principle".into()],
        },
        LensEntry {
            name: "uncertainty_principle".into(),
            category: LensCategory::Extended,
            description: "Measure conjugate variable uncertainty products (ΔxΔp ≥ ℏ/2)".into(),
            domain_affinity: vec!["quantum".into(), "information_theory".into(), "physics".into()],
            complementary: vec!["wave_function".into(), "born_probability".into()],
        },
        LensEntry {
            name: "superposition".into(),
            category: LensCategory::Extended,
            description: "Identify superposition states and interference pattern signatures".into(),
            domain_affinity: vec!["quantum".into(), "optics".into(), "computing".into()],
            complementary: vec!["wave_function".into(), "quantum_tunneling_lens".into()],
        },
        LensEntry {
            name: "quantum_tunneling_lens".into(),
            category: LensCategory::Extended,
            description: "Detect barrier penetration probability via WKB approximation".into(),
            domain_affinity: vec!["quantum".into(), "chemistry".into(), "materials".into()],
            complementary: vec!["superposition".into(), "wave_function".into()],
        },
        LensEntry {
            name: "pauli_exclusion".into(),
            category: LensCategory::Extended,
            description: "Detect exclusion principle compliance and violation patterns in state occupancy".into(),
            domain_affinity: vec!["quantum".into(), "chemistry".into(), "condensed_matter".into()],
            complementary: vec!["fermion_statistics".into(), "quantum_number".into()],
        },
        LensEntry {
            name: "quantum_number".into(),
            category: LensCategory::Extended,
            description: "Map quantum number (n,l,m,s) structure and degeneracy patterns".into(),
            domain_affinity: vec!["quantum".into(), "spectroscopy".into(), "chemistry".into()],
            complementary: vec!["selection_rule".into(), "pauli_exclusion".into()],
        },
        LensEntry {
            name: "selection_rule".into(),
            category: LensCategory::Extended,
            description: "Detect allowed and forbidden transitions via symmetry selection rules".into(),
            domain_affinity: vec!["quantum".into(), "spectroscopy".into(), "group_theory".into()],
            complementary: vec!["quantum_number".into(), "gauge_symmetry".into()],
        },
        // ══════════════════════════════════════════
        // Quantum Information (6)
        // ══════════════════════════════════════════
        LensEntry {
            name: "quantum_channel".into(),
            category: LensCategory::Extended,
            description: "Analyze quantum channel capacity, noise models, and decoherence rates".into(),
            domain_affinity: vec!["quantum_info".into(), "communication".into(), "cryptography".into()],
            complementary: vec!["quantum_error_correct".into(), "channel_capacity".into()],
        },
        LensEntry {
            name: "quantum_error_correct".into(),
            category: LensCategory::Extended,
            description: "Detect quantum error correction code structures (Steane, surface, toric)".into(),
            domain_affinity: vec!["quantum_info".into(), "coding_theory".into(), "topology".into()],
            complementary: vec!["quantum_channel".into(), "topological_order".into()],
        },
        LensEntry {
            name: "quantum_teleportation".into(),
            category: LensCategory::Extended,
            description: "Identify state transfer protocol patterns and entanglement resource usage".into(),
            domain_affinity: vec!["quantum_info".into(), "communication".into(), "entanglement".into()],
            complementary: vec!["bell_inequality".into(), "entanglement_witness".into()],
        },
        LensEntry {
            name: "bell_inequality".into(),
            category: LensCategory::Extended,
            description: "Measure Bell inequality violation magnitude and nonlocality bounds".into(),
            domain_affinity: vec!["quantum".into(), "foundations".into(), "experiment".into()],
            complementary: vec!["entanglement_witness".into(), "quantum_teleportation".into()],
        },
        LensEntry {
            name: "quantum_key_dist".into(),
            category: LensCategory::Extended,
            description: "Analyze QKD protocol security bounds and eavesdropping detection".into(),
            domain_affinity: vec!["quantum_info".into(), "cryptography".into(), "security".into()],
            complementary: vec!["quantum_channel".into(), "bell_inequality".into()],
        },
        LensEntry {
            name: "no_cloning".into(),
            category: LensCategory::Extended,
            description: "Detect no-cloning theorem boundary conditions and approximate cloning fidelity".into(),
            domain_affinity: vec!["quantum_info".into(), "foundations".into(), "computing".into()],
            complementary: vec!["quantum_teleportation".into(), "quantum_error_correct".into()],
        },
        // ══════════════════════════════════════════
        // Many-Body Quantum (6)
        // ══════════════════════════════════════════
        LensEntry {
            name: "fock_space".into(),
            category: LensCategory::Extended,
            description: "Analyze occupation number representation and creation/annihilation patterns".into(),
            domain_affinity: vec!["quantum".into(), "condensed_matter".into(), "particle".into()],
            complementary: vec!["fermion_statistics".into(), "boson_statistics".into()],
        },
        LensEntry {
            name: "fermion_statistics".into(),
            category: LensCategory::Extended,
            description: "Detect Fermi-Dirac distribution patterns and exclusion effects".into(),
            domain_affinity: vec!["quantum".into(), "condensed_matter".into(), "chemistry".into()],
            complementary: vec!["pauli_exclusion".into(), "fock_space".into()],
        },
        LensEntry {
            name: "boson_statistics".into(),
            category: LensCategory::Extended,
            description: "Detect Bose-Einstein distribution patterns and condensation signatures".into(),
            domain_affinity: vec!["quantum".into(), "condensed_matter".into(), "optics".into()],
            complementary: vec!["bose_einstein".into(), "fock_space".into()],
        },
        LensEntry {
            name: "phonon_spectrum".into(),
            category: LensCategory::Extended,
            description: "Analyze lattice vibration spectra and phonon dispersion relations".into(),
            domain_affinity: vec!["condensed_matter".into(), "materials".into(), "thermal".into()],
            complementary: vec!["crystal_structure_lens".into(), "boltzmann_distribution".into()],
        },
        LensEntry {
            name: "cooper_pair".into(),
            category: LensCategory::Extended,
            description: "Detect Cooper pair formation conditions and BCS gap structure".into(),
            domain_affinity: vec!["superconductor".into(), "condensed_matter".into(), "quantum".into()],
            complementary: vec!["phonon_spectrum".into(), "bose_einstein".into()],
        },
        LensEntry {
            name: "quasiparticle".into(),
            category: LensCategory::Extended,
            description: "Identify quasiparticles (polaron, exciton, magnon, plasmon) in data".into(),
            domain_affinity: vec!["condensed_matter".into(), "materials".into(), "optics".into()],
            complementary: vec!["phonon_spectrum".into(), "fock_space".into()],
        },
        // ══════════════════════════════════════════
        // Quantum Field Theory (5)
        // ══════════════════════════════════════════
        LensEntry {
            name: "feynman_diagram".into(),
            category: LensCategory::Extended,
            description: "Detect interaction vertex structures and perturbative expansion patterns".into(),
            domain_affinity: vec!["particle".into(), "qft".into(), "high_energy".into()],
            complementary: vec!["renormalization_qft".into(), "path_integral".into()],
        },
        LensEntry {
            name: "renormalization_qft".into(),
            category: LensCategory::Extended,
            description: "Track QFT renormalization group flow and running coupling constants".into(),
            domain_affinity: vec!["qft".into(), "particle".into(), "critical_phenomena".into()],
            complementary: vec!["feynman_diagram".into(), "running_coupling".into()],
        },
        LensEntry {
            name: "vacuum_fluctuation".into(),
            category: LensCategory::Extended,
            description: "Detect zero-point energy signatures and Casimir-like effects".into(),
            domain_affinity: vec!["qft".into(), "quantum".into(), "cosmology".into()],
            complementary: vec!["casimir_force".into(), "path_integral".into()],
        },
        LensEntry {
            name: "anomaly_qft".into(),
            category: LensCategory::Extended,
            description: "Detect quantum anomalies (chiral, trace, gravitational) breaking classical symmetry".into(),
            domain_affinity: vec!["qft".into(), "particle".into(), "topology".into()],
            complementary: vec!["gauge_symmetry".into(), "spontaneous_symmetry".into()],
        },
        LensEntry {
            name: "path_integral".into(),
            category: LensCategory::Extended,
            description: "Identify dominant paths in path integral formulation and saddle-point structure".into(),
            domain_affinity: vec!["qft".into(), "quantum".into(), "statistical_mechanics".into()],
            complementary: vec!["feynman_diagram".into(), "hamiltonian_flow".into()],
        },
        // ══════════════════════════════════════════
        // Quantum Computing (5)
        // ══════════════════════════════════════════
        LensEntry {
            name: "qubit_gate".into(),
            category: LensCategory::Extended,
            description: "Measure quantum gate fidelity, error rates, and topological protection".into(),
            domain_affinity: vec!["quantum_computing".into(), "error_correction".into(), "hardware".into()],
            complementary: vec!["quantum_circuit".into(), "quantum_error_correct".into()],
        },
        LensEntry {
            name: "quantum_circuit".into(),
            category: LensCategory::Extended,
            description: "Analyze circuit depth, width, and gate-count optimization opportunities".into(),
            domain_affinity: vec!["quantum_computing".into(), "optimization".into(), "compilation".into()],
            complementary: vec!["qubit_gate".into(), "quantum_annealing_lens".into()],
        },
        LensEntry {
            name: "quantum_annealing_lens".into(),
            category: LensCategory::Extended,
            description: "Map quantum annealing energy landscape and tunneling shortcuts".into(),
            domain_affinity: vec!["quantum_computing".into(), "optimization".into(), "condensed_matter".into()],
            complementary: vec!["quantum_tunneling_lens".into(), "spin_glass".into()],
        },
        LensEntry {
            name: "variational_quantum".into(),
            category: LensCategory::Extended,
            description: "Analyze VQE/QAOA variational parameter landscapes and barren plateaus".into(),
            domain_affinity: vec!["quantum_computing".into(), "chemistry".into(), "optimization".into()],
            complementary: vec!["quantum_circuit".into(), "loss_landscape".into()],
        },
        LensEntry {
            name: "quantum_supremacy".into(),
            category: LensCategory::Extended,
            description: "Measure quantum advantage boundary — where quantum beats classical".into(),
            domain_affinity: vec!["quantum_computing".into(), "complexity".into(), "benchmarking".into()],
            complementary: vec!["quantum_circuit".into(), "algorithmic_complexity".into()],
        },
        // ══════════════════════════════════════════
        // Topology Evolution (8)
        // ══════════════════════════════════════════
        LensEntry {
            name: "topology_evolution".into(),
            category: LensCategory::Extended,
            description: "Track topological invariant changes over time (genus, Betti number transitions: sphere→torus→point)".into(),
            domain_affinity: vec!["topology".into(), "dynamics".into(), "physics".into(), "biology".into()],
            complementary: vec!["betti_evolution".into(), "morse_critical".into()],
        },
        LensEntry {
            name: "betti_evolution".into(),
            category: LensCategory::Extended,
            description: "Track Betti number time series — birth/death of holes across scales".into(),
            domain_affinity: vec!["topology".into(), "tda".into(), "dynamics".into()],
            complementary: vec!["topology_evolution".into(), "persistent_homology_drift".into()],
        },
        LensEntry {
            name: "morse_critical".into(),
            category: LensCategory::Extended,
            description: "Detect Morse theory critical points where topology changes (index 0,1,2,3)".into(),
            domain_affinity: vec!["topology".into(), "differential_geometry".into(), "physics".into()],
            complementary: vec!["topology_evolution".into(), "saddle".into()],
        },
        LensEntry {
            name: "cobordism_flow".into(),
            category: LensCategory::Extended,
            description: "Track cobordism between manifolds — how one surface evolves into another".into(),
            domain_affinity: vec!["topology".into(), "qft".into(), "geometry".into()],
            complementary: vec!["topology_evolution".into(), "cobordism_ring".into()],
        },
        LensEntry {
            name: "handle_attachment".into(),
            category: LensCategory::Extended,
            description: "Detect handle attachment/detachment events (genus change by ±1)".into(),
            domain_affinity: vec!["topology".into(), "surgery".into(), "dynamics".into()],
            complementary: vec!["topology_evolution".into(), "surgery_transition".into()],
        },
        LensEntry {
            name: "surgery_transition".into(),
            category: LensCategory::Extended,
            description: "Identify topological surgery events — cutting and regluing manifolds".into(),
            domain_affinity: vec!["topology".into(), "surgery_theory".into(), "geometry".into()],
            complementary: vec!["handle_attachment".into(), "cobordism_flow".into()],
        },
        LensEntry {
            name: "persistent_homology_drift".into(),
            category: LensCategory::Extended,
            description: "Track persistent homology barcode drift over parameter/time evolution".into(),
            domain_affinity: vec!["tda".into(), "topology".into(), "data_science".into()],
            complementary: vec!["betti_evolution".into(), "persistence_tda".into()],
        },
        LensEntry {
            name: "topological_phase_transition".into(),
            category: LensCategory::Extended,
            description: "Detect phase transitions that change topological invariants (not just order parameter)".into(),
            domain_affinity: vec!["topology".into(), "condensed_matter".into(), "quantum".into()],
            complementary: vec!["topology_evolution".into(), "topological_order".into()],
        },
        // ══════════════════════════════════════════
        // Lens Discovery Engine (3) — 렌즈 자동 발견
        // ══════════════════════════════════════════
        LensEntry {
            name: "lens_discovery_engine".into(),
            category: LensCategory::Extended,
            description: "Auto-discover new lens candidates by analyzing gaps in current lens coverage and data residuals".into(),
            domain_affinity: vec!["meta".into(), "generative".into(), "ai".into(), "discovery".into()],
            complementary: vec!["lens_generator".into(), "discovery_gap_mapper".into()],
        },
        LensEntry {
            name: "lens_hypothesis_miner".into(),
            category: LensCategory::Extended,
            description: "Mine scientific literature and BT theorems for undiscovered lens concepts".into(),
            domain_affinity: vec!["meta".into(), "nlp".into(), "knowledge_graph".into()],
            complementary: vec!["lens_discovery_engine".into(), "hypothesis_generator".into()],
        },
        LensEntry {
            name: "lens_validator".into(),
            category: LensCategory::Extended,
            description: "Validate newly proposed lenses — test discrimination power, uniqueness, and calibration".into(),
            domain_affinity: vec!["meta".into(), "statistics".into(), "verification".into()],
            complementary: vec!["lens_discovery_engine".into(), "lens_calibration".into()],
        },
        // ══════════════════════════════════════════
        // OUROBOROS Cycle / Emergence (5) — 수렴→좁아짐→재확장→창발
        // ══════════════════════════════════════════
        LensEntry {
            name: "ouroboros_contraction".into(),
            category: LensCategory::Extended,
            description: "Detect system contraction phase — convergence, dimensionality reduction, information compression".into(),
            domain_affinity: vec!["dynamics".into(), "consciousness".into(), "thermodynamics".into()],
            complementary: vec!["ouroboros_expansion".into(), "ouroboros_bottleneck".into()],
        },
        LensEntry {
            name: "ouroboros_bottleneck".into(),
            category: LensCategory::Extended,
            description: "Identify the minimal state (bottleneck) where system is maximally compressed before re-expansion".into(),
            domain_affinity: vec!["dynamics".into(), "information_theory".into(), "topology".into()],
            complementary: vec!["ouroboros_contraction".into(), "ouroboros_expansion".into()],
        },
        LensEntry {
            name: "ouroboros_expansion".into(),
            category: LensCategory::Extended,
            description: "Detect re-expansion phase — new dimensions, symmetry breaking, diversification after bottleneck".into(),
            domain_affinity: vec!["dynamics".into(), "evolution".into(), "cosmology".into()],
            complementary: vec!["ouroboros_bottleneck".into(), "ouroboros_emergence".into()],
        },
        LensEntry {
            name: "ouroboros_emergence".into(),
            category: LensCategory::Extended,
            description: "Detect novel emergent properties that appear only after contraction→expansion cycle".into(),
            domain_affinity: vec!["emergence".into(), "complexity".into(), "consciousness".into()],
            complementary: vec!["ouroboros_expansion".into(), "ouroboros_cycle_count".into()],
        },
        LensEntry {
            name: "ouroboros_cycle_count".into(),
            category: LensCategory::Extended,
            description: "Count and characterize repeated contraction→expansion cycles and their increasing complexity".into(),
            domain_affinity: vec!["dynamics".into(), "recursion".into(), "evolution".into()],
            complementary: vec!["ouroboros_emergence".into(), "ouroboros_contraction".into()],
        },
        // ══════════════════════════════════════════
        // Cell / Cell Division (6) — 세포/세포분열
        // ══════════════════════════════════════════
        LensEntry {
            name: "cell_division".into(),
            category: LensCategory::Extended,
            description: "Detect mitosis/meiosis-like splitting patterns — one entity dividing into two with information partitioning".into(),
            domain_affinity: vec!["biology".into(), "dynamics".into(), "replication".into()],
            complementary: vec!["cell_cycle".into(), "cell_differentiation".into()],
        },
        LensEntry {
            name: "cell_cycle".into(),
            category: LensCategory::Extended,
            description: "Identify G1→S→G2→M cell cycle phases and checkpoint-like gating mechanisms".into(),
            domain_affinity: vec!["biology".into(), "dynamics".into(), "scheduling".into()],
            complementary: vec!["cell_division".into(), "cell_apoptosis".into()],
        },
        LensEntry {
            name: "cell_differentiation".into(),
            category: LensCategory::Extended,
            description: "Track cell fate decision — identical units becoming specialized (stem→lineage)".into(),
            domain_affinity: vec!["biology".into(), "development".into(), "ai".into()],
            complementary: vec!["cell_division".into(), "morphogenesis".into()],
        },
        LensEntry {
            name: "cell_apoptosis".into(),
            category: LensCategory::Extended,
            description: "Detect programmed death patterns — controlled removal for system health".into(),
            domain_affinity: vec!["biology".into(), "dynamics".into(), "optimization".into()],
            complementary: vec!["cell_cycle".into(), "lens_pruning".into()],
        },
        LensEntry {
            name: "cell_signaling".into(),
            category: LensCategory::Extended,
            description: "Trace inter-cell communication cascades (ligand→receptor→pathway→response)".into(),
            domain_affinity: vec!["biology".into(), "network".into(), "signal".into()],
            complementary: vec!["cell_differentiation".into(), "cell_membrane".into()],
        },
        LensEntry {
            name: "cell_membrane".into(),
            category: LensCategory::Extended,
            description: "Analyze boundary permeability — what passes through, what is blocked, selective transport".into(),
            domain_affinity: vec!["biology".into(), "boundary".into(), "transport".into()],
            complementary: vec!["cell_signaling".into(), "boundary".into()],
        },
        // ══════════════════════════════════════════
        // Singularity (4) — 특이점
        // ══════════════════════════════════════════
        LensEntry {
            name: "singularity_detect".into(),
            category: LensCategory::Extended,
            description: "Detect mathematical/physical singularities — points where quantities diverge to infinity".into(),
            domain_affinity: vec!["mathematics".into(), "physics".into(), "cosmology".into()],
            complementary: vec!["singularity_classify".into(), "singularity_resolve".into()],
        },
        LensEntry {
            name: "singularity_classify".into(),
            category: LensCategory::Extended,
            description: "Classify singularity type: removable, pole, essential, curvature, naked".into(),
            domain_affinity: vec!["mathematics".into(), "physics".into(), "analysis".into()],
            complementary: vec!["singularity_detect".into(), "singularity_resolve".into()],
        },
        LensEntry {
            name: "singularity_resolve".into(),
            category: LensCategory::Extended,
            description: "Find regularization or resolution strategies for detected singularities".into(),
            domain_affinity: vec!["mathematics".into(), "physics".into(), "renormalization".into()],
            complementary: vec!["singularity_classify".into(), "renormalization_qft".into()],
        },
        LensEntry {
            name: "singularity_approach".into(),
            category: LensCategory::Extended,
            description: "Measure rate and trajectory of approach to singularity — finite-time blowup detection".into(),
            domain_affinity: vec!["dynamics".into(), "physics".into(), "mathematics".into()],
            complementary: vec!["singularity_detect".into(), "tipping".into()],
        },
        // ══════════════════════════════════════════
        // Black Hole (4) — 블랙홀
        // ══════════════════════════════════════════
        LensEntry {
            name: "black_hole_lens".into(),
            category: LensCategory::Extended,
            description: "Detect information-trapping regions — data enters but cannot escape (event horizon analogy)".into(),
            domain_affinity: vec!["cosmology".into(), "information_theory".into(), "physics".into()],
            complementary: vec!["hawking_radiation".into(), "event_horizon".into()],
        },
        LensEntry {
            name: "event_horizon".into(),
            category: LensCategory::Extended,
            description: "Identify point-of-no-return boundaries in parameter/state space".into(),
            domain_affinity: vec!["cosmology".into(), "dynamics".into(), "topology".into()],
            complementary: vec!["black_hole_lens".into(), "singularity_detect".into()],
        },
        LensEntry {
            name: "hawking_radiation".into(),
            category: LensCategory::Extended,
            description: "Detect information leakage from trapped regions — slow escape via quantum effects".into(),
            domain_affinity: vec!["cosmology".into(), "quantum".into(), "information_theory".into()],
            complementary: vec!["black_hole_lens".into(), "vacuum_fluctuation".into()],
        },
        LensEntry {
            name: "information_paradox".into(),
            category: LensCategory::Extended,
            description: "Detect apparent information loss and unitarity violation in system evolution".into(),
            domain_affinity: vec!["quantum".into(), "information_theory".into(), "foundations".into()],
            complementary: vec!["hawking_radiation".into(), "no_cloning".into()],
        },
        // ══════════════════════════════════════════
        // Antimatter (3) — 반물질
        // ══════════════════════════════════════════
        LensEntry {
            name: "antimatter_symmetry".into(),
            category: LensCategory::Extended,
            description: "Detect matter-antimatter asymmetry (CP violation) and baryon asymmetry patterns".into(),
            domain_affinity: vec!["particle".into(), "cosmology".into(), "symmetry".into()],
            complementary: vec!["antimatter_annihilation".into(), "cp_violation".into()],
        },
        LensEntry {
            name: "antimatter_annihilation".into(),
            category: LensCategory::Extended,
            description: "Detect mutual annihilation events — two complementary entities producing pure energy/signal".into(),
            domain_affinity: vec!["particle".into(), "physics".into(), "dynamics".into()],
            complementary: vec!["antimatter_symmetry".into(), "antimatter_creation".into()],
        },
        LensEntry {
            name: "antimatter_creation".into(),
            category: LensCategory::Extended,
            description: "Detect pair production — energy spontaneously creating complementary entity pairs".into(),
            domain_affinity: vec!["particle".into(), "quantum".into(), "cosmology".into()],
            complementary: vec!["antimatter_annihilation".into(), "vacuum_fluctuation".into()],
        },
        // ══════════════════════════════════════════
        // Time (5) — 시간
        // ══════════════════════════════════════════
        LensEntry {
            name: "arrow_of_time".into(),
            category: LensCategory::Extended,
            description: "Detect time-irreversibility and entropy increase direction in system evolution".into(),
            domain_affinity: vec!["physics".into(), "thermodynamics".into(), "information_theory".into()],
            complementary: vec!["time_crystal".into(), "time_reversal".into()],
        },
        LensEntry {
            name: "time_crystal".into(),
            category: LensCategory::Extended,
            description: "Detect spontaneous time-translation symmetry breaking — periodic motion in ground state".into(),
            domain_affinity: vec!["condensed_matter".into(), "quantum".into(), "dynamics".into()],
            complementary: vec!["arrow_of_time".into(), "time_dilation_lens".into()],
        },
        LensEntry {
            name: "time_reversal".into(),
            category: LensCategory::Extended,
            description: "Test T-symmetry: does the system behave identically when time is reversed?".into(),
            domain_affinity: vec!["physics".into(), "quantum".into(), "symmetry".into()],
            complementary: vec!["arrow_of_time".into(), "cp_violation".into()],
        },
        LensEntry {
            name: "time_dilation_lens".into(),
            category: LensCategory::Extended,
            description: "Detect relativistic time dilation effects — rate differences between reference frames".into(),
            domain_affinity: vec!["relativity".into(), "cosmology".into(), "physics".into()],
            complementary: vec!["arrow_of_time".into(), "time_crystal".into()],
        },
        LensEntry {
            name: "temporal_entanglement".into(),
            category: LensCategory::Extended,
            description: "Detect non-local temporal correlations — future-past quantum correlations beyond classical causality".into(),
            domain_affinity: vec!["quantum".into(), "time".into(), "foundations".into()],
            complementary: vec!["bell_inequality".into(), "arrow_of_time".into()],
        },
        // ══════════════════════════════════════════
        // Programming / Software Engineering (12)
        // ══════════════════════════════════════════
        LensEntry { name: "design_pattern".into(), category: LensCategory::Extended, description: "Detect GoF/SOLID design patterns and architectural motifs in system structure".into(), domain_affinity: vec!["software".into(), "architecture".into(), "ai".into()], complementary: vec!["code_smell".into(), "refactoring_opportunity".into()] },
        LensEntry { name: "code_smell".into(), category: LensCategory::Extended, description: "Identify anti-patterns, code smells, and structural dysfunction indicators".into(), domain_affinity: vec!["software".into(), "quality".into(), "maintenance".into()], complementary: vec!["design_pattern".into(), "technical_debt".into()] },
        LensEntry { name: "refactoring_opportunity".into(), category: LensCategory::Extended, description: "Detect refactoring opportunities — duplication, long methods, god objects".into(), domain_affinity: vec!["software".into(), "optimization".into(), "quality".into()], complementary: vec!["code_smell".into(), "coupling_cohesion".into()] },
        LensEntry { name: "dependency_graph_sw".into(), category: LensCategory::Extended, description: "Analyze software dependency graph — circular deps, fragile base class, diamond inheritance".into(), domain_affinity: vec!["software".into(), "graph".into(), "architecture".into()], complementary: vec!["coupling_cohesion".into(), "api_surface".into()] },
        LensEntry { name: "cyclomatic_complexity".into(), category: LensCategory::Extended, description: "Measure cyclomatic/cognitive complexity and control flow branching depth".into(), domain_affinity: vec!["software".into(), "complexity".into(), "testing".into()], complementary: vec!["code_smell".into(), "refactoring_opportunity".into()] },
        LensEntry { name: "coupling_cohesion".into(), category: LensCategory::Extended, description: "Measure module coupling (low=good) and cohesion (high=good) balance".into(), domain_affinity: vec!["software".into(), "architecture".into(), "modularity".into()], complementary: vec!["dependency_graph_sw".into(), "design_pattern".into()] },
        LensEntry { name: "technical_debt".into(), category: LensCategory::Extended, description: "Quantify accumulated technical debt — shortcuts, TODOs, deprecated patterns".into(), domain_affinity: vec!["software".into(), "economics".into(), "maintenance".into()], complementary: vec!["code_smell".into(), "refactoring_opportunity".into()] },
        LensEntry { name: "api_surface".into(), category: LensCategory::Extended, description: "Analyze API surface area — exposed vs internal, stability, backward compatibility".into(), domain_affinity: vec!["software".into(), "interface".into(), "design".into()], complementary: vec!["coupling_cohesion".into(), "type_system_lens".into()] },
        LensEntry { name: "type_system_lens".into(), category: LensCategory::Extended, description: "Analyze type system structure — generics depth, trait bounds, type-level computation".into(), domain_affinity: vec!["software".into(), "type_theory".into(), "verification".into()], complementary: vec!["api_surface".into(), "design_pattern".into()] },
        LensEntry { name: "concurrency_pattern".into(), category: LensCategory::Extended, description: "Detect concurrency patterns — lock-free, actor, CSP, fork-join, async/await".into(), domain_affinity: vec!["software".into(), "concurrency".into(), "performance".into()], complementary: vec!["memory_model_sw".into(), "design_pattern".into()] },
        LensEntry { name: "memory_model_sw".into(), category: LensCategory::Extended, description: "Analyze memory model — ownership, borrowing, aliasing, cache locality patterns".into(), domain_affinity: vec!["software".into(), "performance".into(), "safety".into()], complementary: vec!["concurrency_pattern".into(), "compiler_optimization_lens".into()] },
        LensEntry { name: "compiler_optimization_lens".into(), category: LensCategory::Extended, description: "Detect compiler optimization opportunities — inlining, vectorization, dead code".into(), domain_affinity: vec!["software".into(), "performance".into(), "compilation".into()], complementary: vec!["memory_model_sw".into(), "cyclomatic_complexity".into()] },
        // ══════════════════════════════════════════
        // Fusion / Plasma (10)
        // ══════════════════════════════════════════
        LensEntry { name: "plasma_confinement".into(), category: LensCategory::Extended, description: "Measure plasma confinement time and energy confinement scaling laws".into(), domain_affinity: vec!["fusion".into(), "plasma".into(), "energy".into()], complementary: vec!["lawson_criterion".into(), "tokamak_stability".into()] },
        LensEntry { name: "lawson_criterion".into(), category: LensCategory::Extended, description: "Evaluate Lawson triple product nTτ for fusion ignition feasibility".into(), domain_affinity: vec!["fusion".into(), "plasma".into(), "physics".into()], complementary: vec!["plasma_confinement".into(), "ignition_condition".into()] },
        LensEntry { name: "tokamak_stability".into(), category: LensCategory::Extended, description: "Analyze MHD stability — kink, ballooning, tearing modes in toroidal geometry".into(), domain_affinity: vec!["fusion".into(), "plasma".into(), "stability".into()], complementary: vec!["instability_mode".into(), "magnetic_topology".into()] },
        LensEntry { name: "magnetic_topology".into(), category: LensCategory::Extended, description: "Map magnetic field line topology — flux surfaces, islands, stochastic regions".into(), domain_affinity: vec!["fusion".into(), "topology".into(), "electromagnetism".into()], complementary: vec!["tokamak_stability".into(), "plasma_confinement".into()] },
        LensEntry { name: "tritium_breeding".into(), category: LensCategory::Extended, description: "Evaluate tritium breeding ratio and lithium blanket neutronics".into(), domain_affinity: vec!["fusion".into(), "nuclear".into(), "materials".into()], complementary: vec!["neutron_economy".into(), "fusion_cross_section".into()] },
        LensEntry { name: "divertor_heat".into(), category: LensCategory::Extended, description: "Analyze divertor heat flux management and plasma-wall interaction".into(), domain_affinity: vec!["fusion".into(), "thermal".into(), "materials".into()], complementary: vec!["tokamak_stability".into(), "plasma_confinement".into()] },
        LensEntry { name: "instability_mode".into(), category: LensCategory::Extended, description: "Classify plasma instability modes — ELMs, disruptions, sawtooth, NTMs".into(), domain_affinity: vec!["fusion".into(), "plasma".into(), "dynamics".into()], complementary: vec!["tokamak_stability".into(), "magnetic_topology".into()] },
        LensEntry { name: "ignition_condition".into(), category: LensCategory::Extended, description: "Determine proximity to self-sustaining fusion ignition threshold".into(), domain_affinity: vec!["fusion".into(), "physics".into(), "energy".into()], complementary: vec!["lawson_criterion".into(), "energy_gain_q".into()] },
        LensEntry { name: "energy_gain_q".into(), category: LensCategory::Extended, description: "Calculate fusion energy gain factor Q = P_fusion / P_input".into(), domain_affinity: vec!["fusion".into(), "energy".into(), "physics".into()], complementary: vec!["ignition_condition".into(), "lawson_criterion".into()] },
        LensEntry { name: "fusion_cross_section".into(), category: LensCategory::Extended, description: "Analyze fusion reaction cross-sections — D-T, D-D, D-He3 energy dependence".into(), domain_affinity: vec!["fusion".into(), "nuclear".into(), "particle".into()], complementary: vec!["energy_gain_q".into(), "tritium_breeding".into()] },
        // ══════════════════════════════════════════
        // Topological Passable Walls (6)
        // ══════════════════════════════════════════
        LensEntry { name: "topological_tunnel".into(), category: LensCategory::Extended, description: "Detect topological tunnels — passages through genus holes connecting otherwise separated regions".into(), domain_affinity: vec!["topology".into(), "geometry".into(), "physics".into()], complementary: vec!["wormhole".into(), "portal".into()] },
        LensEntry { name: "wormhole".into(), category: LensCategory::Extended, description: "Identify wormhole-like shortcuts — distant points connected through folded space/parameter manifold".into(), domain_affinity: vec!["cosmology".into(), "topology".into(), "optimization".into()], complementary: vec!["topological_tunnel".into(), "portal".into()] },
        LensEntry { name: "portal".into(), category: LensCategory::Extended, description: "Detect portal connections between distinct topological spaces or parameter regimes".into(), domain_affinity: vec!["topology".into(), "dynamics".into(), "cross_domain".into()], complementary: vec!["wormhole".into(), "topological_tunnel".into()] },
        LensEntry { name: "membrane_permeability".into(), category: LensCategory::Extended, description: "Measure selective permeability of boundaries — what passes through, what is blocked".into(), domain_affinity: vec!["biology".into(), "topology".into(), "transport".into()], complementary: vec!["barrier_transparency".into(), "domain_wall_crossing".into()] },
        LensEntry { name: "domain_wall_crossing".into(), category: LensCategory::Extended, description: "Calculate energy cost and probability of crossing domain walls between phases".into(), domain_affinity: vec!["physics".into(), "topology".into(), "dynamics".into()], complementary: vec!["membrane_permeability".into(), "barrier_transparency".into()] },
        LensEntry { name: "barrier_transparency".into(), category: LensCategory::Extended, description: "Generalized barrier transparency — quantum tunneling extended to any barrier type".into(), domain_affinity: vec!["quantum".into(), "topology".into(), "optimization".into()], complementary: vec!["quantum_tunneling_lens".into(), "domain_wall_crossing".into()] },
        // ══════════════════════════════════════════
        // Scan Type Discovery (8) — 스캔 자동 판별
        // ══════════════════════════════════════════
        LensEntry { name: "scan_type_detector".into(), category: LensCategory::Extended, description: "Analyze data characteristics to auto-select optimal scan Tier and lens combination".into(), domain_affinity: vec!["meta".into(), "classification".into(), "automation".into()], complementary: vec!["domain_identifier".into(), "scan_depth_advisor".into()] },
        LensEntry { name: "domain_identifier".into(), category: LensCategory::Extended, description: "Auto-identify which domain the data belongs to (physics, biology, software, etc.)".into(), domain_affinity: vec!["meta".into(), "classification".into(), "nlp".into()], complementary: vec!["scan_type_detector".into(), "data_shape_classifier".into()] },
        LensEntry { name: "data_shape_classifier".into(), category: LensCategory::Extended, description: "Classify data shape — time series, graph, matrix, tensor, text, image".into(), domain_affinity: vec!["meta".into(), "data_science".into(), "preprocessing".into()], complementary: vec!["domain_identifier".into(), "noise_floor_estimator".into()] },
        LensEntry { name: "noise_floor_estimator".into(), category: LensCategory::Extended, description: "Estimate noise level to calibrate lens sensitivity thresholds".into(), domain_affinity: vec!["meta".into(), "signal".into(), "statistics".into()], complementary: vec!["signal_richness".into(), "scan_depth_advisor".into()] },
        LensEntry { name: "signal_richness".into(), category: LensCategory::Extended, description: "Measure signal richness — determine if data warrants full 775-lens scan or Tier 0 suffices".into(), domain_affinity: vec!["meta".into(), "information_theory".into(), "efficiency".into()], complementary: vec!["noise_floor_estimator".into(), "scan_depth_advisor".into()] },
        LensEntry { name: "scan_depth_advisor".into(), category: LensCategory::Extended, description: "Recommend scan depth: Tier 0 (3 lenses), Tier 1 (30), Tier 2 (775), or Tier 3 (auto evolve)".into(), domain_affinity: vec!["meta".into(), "optimization".into(), "scheduling".into()], complementary: vec!["scan_type_detector".into(), "signal_richness".into()] },
        LensEntry { name: "prior_scan_matcher".into(), category: LensCategory::Extended, description: "Match current data against previous scan history to reuse proven lens combinations".into(), domain_affinity: vec!["meta".into(), "memory".into(), "transfer_learning".into()], complementary: vec!["lens_transfer".into(), "scan_type_detector".into()] },
        LensEntry { name: "anomaly_pre_screen".into(), category: LensCategory::Extended, description: "Quick pre-screening for anomalies to identify focus regions before full scan".into(), domain_affinity: vec!["meta".into(), "anomaly".into(), "efficiency".into()], complementary: vec!["scan_depth_advisor".into(), "noise_floor_estimator".into()] },
        // ══════════════════════════════════════════
        // Auto Evolution (6) — 렌즈 자동 진화
        // ══════════════════════════════════════════
        LensEntry { name: "auto_evolve_trigger".into(), category: LensCategory::Extended, description: "Detect when evolution should start — saturation, stagnation, or coverage gap".into(), domain_affinity: vec!["meta".into(), "dynamics".into(), "automation".into()], complementary: vec!["evolution_direction".into(), "saturation_detector".into()] },
        LensEntry { name: "evolution_direction".into(), category: LensCategory::Extended, description: "Suggest which axis to evolve along — new domain, deeper precision, or wider coverage".into(), domain_affinity: vec!["meta".into(), "optimization".into(), "strategy".into()], complementary: vec!["auto_evolve_trigger".into(), "fitness_evaluator".into()] },
        LensEntry { name: "fitness_evaluator".into(), category: LensCategory::Extended, description: "Auto-evaluate lens/hypothesis fitness — discovery rate, uniqueness, reliability".into(), domain_affinity: vec!["meta".into(), "evolution".into(), "statistics".into()], complementary: vec!["selection_pressure_lens".into(), "lens_effectiveness_ranker".into()] },
        LensEntry { name: "selection_pressure_lens".into(), category: LensCategory::Extended, description: "Measure selection pressure on lenses — which survive, which get pruned".into(), domain_affinity: vec!["meta".into(), "evolution".into(), "dynamics".into()], complementary: vec!["fitness_evaluator".into(), "diversity_monitor".into()] },
        LensEntry { name: "diversity_monitor".into(), category: LensCategory::Extended, description: "Monitor lens population diversity — prevent convergence to monoculture".into(), domain_affinity: vec!["meta".into(), "ecology".into(), "evolution".into()], complementary: vec!["selection_pressure_lens".into(), "co_evolution_tracker".into()] },
        LensEntry { name: "co_evolution_tracker".into(), category: LensCategory::Extended, description: "Track lens↔discovery co-evolution — how lenses and findings shape each other".into(), domain_affinity: vec!["meta".into(), "dynamics".into(), "evolution".into()], complementary: vec!["diversity_monitor".into(), "lens_evolution_tracker".into()] },
        // ══════════════════════════════════════════
        // Auto Discovery → Auto Register Engine (5)
        // ══════════════════════════════════════════
        LensEntry { name: "auto_register_gate".into(), category: LensCategory::Extended, description: "Gating function: should a newly forged lens be auto-registered or held for review?".into(), domain_affinity: vec!["meta".into(), "automation".into(), "safety".into()], complementary: vec!["auto_register_pipeline".into(), "lens_validator".into()] },
        LensEntry { name: "auto_register_pipeline".into(), category: LensCategory::Extended, description: "Full pipeline: forge → validate → register → calibrate → activate — zero human intervention".into(), domain_affinity: vec!["meta".into(), "automation".into(), "pipeline".into()], complementary: vec!["auto_register_gate".into(), "auto_calibrate".into()] },
        LensEntry { name: "auto_calibrate".into(), category: LensCategory::Extended, description: "Auto-calibrate newly registered lens against existing lenses for consistency".into(), domain_affinity: vec!["meta".into(), "calibration".into(), "statistics".into()], complementary: vec!["lens_calibration".into(), "auto_register_pipeline".into()] },
        LensEntry { name: "auto_deprecate".into(), category: LensCategory::Extended, description: "Auto-mark underperforming lenses as deprecated based on sustained low hit_rate".into(), domain_affinity: vec!["meta".into(), "lifecycle".into(), "optimization".into()], complementary: vec!["lens_pruning".into(), "fitness_evaluator".into()] },
        LensEntry { name: "registry_health_check".into(), category: LensCategory::Extended, description: "Periodic health check of entire registry — duplicates, orphans, stale lenses, coverage gaps".into(), domain_affinity: vec!["meta".into(), "maintenance".into(), "diagnostics".into()], complementary: vec!["lens_redundancy".into(), "lens_self_awareness".into()] },
        // ══════════════════════════════════════════
        // Microscope (10) — 미시 세계 탐색
        // ══════════════════════════════════════════
        LensEntry { name: "atomic_force_microscope".into(), category: LensCategory::Extended, description: "Atomic-level surface topography and force mapping at nanoscale resolution".into(), domain_affinity: vec!["materials".into(), "nanotechnology".into(), "physics".into()], complementary: vec!["electron_microscope".into(), "cryo_em".into()] },
        LensEntry { name: "electron_microscope".into(), category: LensCategory::Extended, description: "Electron beam imaging for nanostructure analysis beyond optical diffraction limit".into(), domain_affinity: vec!["materials".into(), "biology".into(), "nanotechnology".into()], complementary: vec!["atomic_force_microscope".into(), "cryo_em".into()] },
        LensEntry { name: "optical_microscope".into(), category: LensCategory::Extended, description: "Optical diffraction-limited structure detection and brightfield/darkfield analysis".into(), domain_affinity: vec!["biology".into(), "materials".into(), "optics".into()], complementary: vec!["fluorescence_microscope".into(), "confocal_microscope".into()] },
        LensEntry { name: "fluorescence_microscope".into(), category: LensCategory::Extended, description: "Fluorescence marker-based selective structure tracking and co-localization".into(), domain_affinity: vec!["biology".into(), "chemistry".into(), "medicine".into()], complementary: vec!["optical_microscope".into(), "confocal_microscope".into()] },
        LensEntry { name: "confocal_microscope".into(), category: LensCategory::Extended, description: "Focal plane slicing for 3D reconstruction of volumetric structures".into(), domain_affinity: vec!["biology".into(), "materials".into(), "imaging".into()], complementary: vec!["fluorescence_microscope".into(), "cryo_em".into()] },
        LensEntry { name: "cryo_em".into(), category: LensCategory::Extended, description: "Cryo-electron microscopy for near-atomic 3D protein/molecular structure determination".into(), domain_affinity: vec!["biology".into(), "chemistry".into(), "structural_biology".into()], complementary: vec!["electron_microscope".into(), "xray_crystallography".into()] },
        LensEntry { name: "xray_crystallography".into(), category: LensCategory::Extended, description: "X-ray diffraction pattern analysis for crystal lattice structure determination".into(), domain_affinity: vec!["chemistry".into(), "materials".into(), "structural_biology".into()], complementary: vec!["cryo_em".into(), "nmr_spectroscopy".into()] },
        LensEntry { name: "nmr_spectroscopy".into(), category: LensCategory::Extended, description: "Nuclear magnetic resonance for molecular structure, dynamics, and environment probing".into(), domain_affinity: vec!["chemistry".into(), "biology".into(), "medicine".into()], complementary: vec!["xray_crystallography".into(), "raman_spectroscopy".into()] },
        LensEntry { name: "mass_spectrometry".into(), category: LensCategory::Extended, description: "Mass-to-charge ratio analysis for molecular weight and composition identification".into(), domain_affinity: vec!["chemistry".into(), "biology".into(), "forensics".into()], complementary: vec!["nmr_spectroscopy".into(), "raman_spectroscopy".into()] },
        LensEntry { name: "raman_spectroscopy".into(), category: LensCategory::Extended, description: "Raman scattering analysis for molecular vibration modes and chemical bond structure".into(), domain_affinity: vec!["chemistry".into(), "materials".into(), "physics".into()], complementary: vec!["nmr_spectroscopy".into(), "mass_spectrometry".into()] },
        // ══════════════════════════════════════════
        // Fast/Combo Scan (5)
        // ══════════════════════════════════════════
        LensEntry { name: "fast_full_scan".into(), category: LensCategory::Extended, description: "Fast full scan — top 100 lenses by hit_rate only, 10x faster than 804-lens full scan".into(), domain_affinity: vec!["meta".into(), "performance".into(), "screening".into()], complementary: vec!["adaptive_combo".into(), "parallel_tier_scan".into()] },
        LensEntry { name: "adaptive_combo".into(), category: LensCategory::Extended, description: "Dynamically adjust lens combination in real-time based on intermediate results".into(), domain_affinity: vec!["meta".into(), "optimization".into(), "adaptive".into()], complementary: vec!["fast_full_scan".into(), "scan_depth_advisor".into()] },
        LensEntry { name: "parallel_tier_scan".into(), category: LensCategory::Extended, description: "Run Tier 0/1/2 simultaneously — fast results first, deep results follow".into(), domain_affinity: vec!["meta".into(), "concurrency".into(), "performance".into()], complementary: vec!["fast_full_scan".into(), "incremental_scan".into()] },
        LensEntry { name: "incremental_scan".into(), category: LensCategory::Extended, description: "Cache previous scan results — only re-scan changed/new data regions".into(), domain_affinity: vec!["meta".into(), "caching".into(), "efficiency".into()], complementary: vec!["parallel_tier_scan".into(), "prior_scan_matcher".into()] },
        LensEntry { name: "batch_domain_scan".into(), category: LensCategory::Extended, description: "Batch scan multiple domains simultaneously with shared computation".into(), domain_affinity: vec!["meta".into(), "scheduling".into(), "cross_domain".into()], complementary: vec!["parallel_tier_scan".into(), "domain_identifier".into()] },
        // ══════════════════════════════════════════
        // Optimal Range (5)
        // ══════════════════════════════════════════
        LensEntry { name: "optimal_range_finder".into(), category: LensCategory::Extended, description: "Auto-search parameter optimal range where n=6 patterns are strongest".into(), domain_affinity: vec!["optimization".into(), "search".into(), "verification".into()], complementary: vec!["sensitivity_band".into(), "sweet_spot_detector".into()] },
        LensEntry { name: "sensitivity_band".into(), category: LensCategory::Extended, description: "Map sensitivity bands — where small input changes cause large output shifts".into(), domain_affinity: vec!["analysis".into(), "dynamics".into(), "control".into()], complementary: vec!["optimal_range_finder".into(), "tolerance_analyzer".into()] },
        LensEntry { name: "operating_envelope".into(), category: LensCategory::Extended, description: "Map safe operating envelope — boundary of stable/valid parameter space".into(), domain_affinity: vec!["engineering".into(), "safety".into(), "control".into()], complementary: vec!["sensitivity_band".into(), "tolerance_analyzer".into()] },
        LensEntry { name: "sweet_spot_detector".into(), category: LensCategory::Extended, description: "Find multi-objective sweet spot — best tradeoff of performance/cost/stability".into(), domain_affinity: vec!["optimization".into(), "pareto".into(), "decision".into()], complementary: vec!["optimal_range_finder".into(), "pareto_optimizer".into()] },
        LensEntry { name: "tolerance_analyzer".into(), category: LensCategory::Extended, description: "Determine tolerance margins — how far from optimal before degradation".into(), domain_affinity: vec!["engineering".into(), "robustness".into(), "quality".into()], complementary: vec!["sensitivity_band".into(), "operating_envelope".into()] },
        // ══════════════════════════════════════════
        // Discovery-Driven Evolved Scan (6)
        // ══════════════════════════════════════════
        LensEntry { name: "discovery_reactive_scan".into(), category: LensCategory::Extended, description: "Scan that reacts to discoveries — finding X triggers deeper scan in X's neighborhood".into(), domain_affinity: vec!["meta".into(), "adaptive".into(), "discovery".into()], complementary: vec!["discovery_chain_scan".into(), "adaptive_combo".into()] },
        LensEntry { name: "discovery_chain_scan".into(), category: LensCategory::Extended, description: "Chain scan — each discovery triggers next scan with context from previous discovery".into(), domain_affinity: vec!["meta".into(), "pipeline".into(), "discovery".into()], complementary: vec!["discovery_reactive_scan".into(), "discovery_branch_scan".into()] },
        LensEntry { name: "discovery_branch_scan".into(), category: LensCategory::Extended, description: "Branch scan — one discovery forks into parallel deep-dives in multiple directions".into(), domain_affinity: vec!["meta".into(), "concurrency".into(), "exploration".into()], complementary: vec!["discovery_chain_scan".into(), "discovery_weight_update".into()] },
        LensEntry { name: "discovery_weight_update".into(), category: LensCategory::Extended, description: "Auto-update lens weights based on what each discovery reveals about lens effectiveness".into(), domain_affinity: vec!["meta".into(), "learning".into(), "adaptation".into()], complementary: vec!["lens_effectiveness_ranker".into(), "discovery_reactive_scan".into()] },
        LensEntry { name: "discovery_saturation_pivot".into(), category: LensCategory::Extended, description: "When current scan saturates, auto-pivot to unexplored lens/domain combinations".into(), domain_affinity: vec!["meta".into(), "strategy".into(), "exploration".into()], complementary: vec!["saturation_detector".into(), "discovery_gap_mapper".into()] },
        LensEntry { name: "discovery_amplification_scan".into(), category: LensCategory::Extended, description: "Amplify weak signals from initial discovery by focusing more lenses on the same pattern".into(), domain_affinity: vec!["meta".into(), "signal".into(), "verification".into()], complementary: vec!["consensus_amplifier".into(), "discovery_reactive_scan".into()] },
        // ══════════════════════════════════════════
        // Weight / Priority (4)
        // ══════════════════════════════════════════
        LensEntry { name: "lens_weight_calculator".into(), category: LensCategory::Extended, description: "Calculate dynamic weight for each lens based on domain, history, and context".into(), domain_affinity: vec!["meta".into(), "statistics".into(), "scheduling".into()], complementary: vec!["discovery_weight_update".into(), "lens_effectiveness_ranker".into()] },
        LensEntry { name: "priority_queue_lens".into(), category: LensCategory::Extended, description: "Priority queue ordering — highest expected-value lenses execute first".into(), domain_affinity: vec!["meta".into(), "scheduling".into(), "optimization".into()], complementary: vec!["lens_weight_calculator".into(), "lens_ordering".into()] },
        LensEntry { name: "bayesian_weight_updater".into(), category: LensCategory::Extended, description: "Bayesian update of lens weights — prior × likelihood from each scan result".into(), domain_affinity: vec!["meta".into(), "bayesian".into(), "statistics".into()], complementary: vec!["lens_weight_calculator".into(), "lens_calibration".into()] },
        LensEntry { name: "context_weight_modifier".into(), category: LensCategory::Extended, description: "Modify lens weights based on context — domain, data type, recent discoveries".into(), domain_affinity: vec!["meta".into(), "context".into(), "adaptation".into()], complementary: vec!["lens_weight_calculator".into(), "domain_identifier".into()] },
        // ══════════════════════════════════════════
        // Fractal (8)
        // ══════════════════════════════════════════
        LensEntry { name: "fractal_dimension".into(), category: LensCategory::Extended, description: "Measure fractal (Hausdorff) dimension of data structures and patterns".into(), domain_affinity: vec!["mathematics".into(), "complexity".into(), "geometry".into()], complementary: vec!["fractal_self_similarity".into(), "multifractal_spectrum".into()] },
        LensEntry { name: "fractal_self_similarity".into(), category: LensCategory::Extended, description: "Detect self-similar patterns recurring across multiple scales".into(), domain_affinity: vec!["mathematics".into(), "scaling".into(), "pattern".into()], complementary: vec!["fractal_dimension".into(), "fractal_branching".into()] },
        LensEntry { name: "fractal_branching".into(), category: LensCategory::Extended, description: "Analyze branching topology in fractal-like hierarchical structures".into(), domain_affinity: vec!["biology".into(), "network".into(), "morphology".into()], complementary: vec!["fractal_self_similarity".into(), "fractal_boundary".into()] },
        LensEntry { name: "fractal_boundary".into(), category: LensCategory::Extended, description: "Characterize fractal boundary complexity and coastline-type irregularity".into(), domain_affinity: vec!["geometry".into(), "complexity".into(), "topology".into()], complementary: vec!["fractal_dimension".into(), "fractal_compression".into()] },
        LensEntry { name: "fractal_compression".into(), category: LensCategory::Extended, description: "Exploit self-similarity for data compression via iterated function systems".into(), domain_affinity: vec!["information".into(), "compression".into(), "signal".into()], complementary: vec!["fractal_self_similarity".into(), "fractal_dimension".into()] },
        LensEntry { name: "mandelbrot_set".into(), category: LensCategory::Extended, description: "Apply Mandelbrot iteration analysis to detect divergence/convergence boundaries".into(), domain_affinity: vec!["mathematics".into(), "dynamics".into(), "complexity".into()], complementary: vec!["julia_set".into(), "fractal_boundary".into()] },
        LensEntry { name: "julia_set".into(), category: LensCategory::Extended, description: "Map Julia set structure for parameter-dependent dynamical behavior".into(), domain_affinity: vec!["mathematics".into(), "dynamics".into(), "chaos".into()], complementary: vec!["mandelbrot_set".into(), "fractal_dimension".into()] },
        LensEntry { name: "multifractal_spectrum".into(), category: LensCategory::Extended, description: "Compute full multifractal spectrum revealing heterogeneous scaling exponents".into(), domain_affinity: vec!["statistics".into(), "scaling".into(), "turbulence".into()], complementary: vec!["fractal_dimension".into(), "fractal_self_similarity".into()] },
        // ══════════════════════════════════════════
        // Geometry (10)
        // ══════════════════════════════════════════
        LensEntry { name: "euclidean_geometry".into(), category: LensCategory::Extended, description: "Analyze flat-space metric properties — distances, angles, areas".into(), domain_affinity: vec!["mathematics".into(), "geometry".into(), "engineering".into()], complementary: vec!["hyperbolic_geometry".into(), "spherical_geometry".into()] },
        LensEntry { name: "hyperbolic_geometry".into(), category: LensCategory::Extended, description: "Detect negative-curvature structures and exponential growth patterns".into(), domain_affinity: vec!["mathematics".into(), "geometry".into(), "network".into()], complementary: vec!["euclidean_geometry".into(), "riemannian_curvature".into()] },
        LensEntry { name: "spherical_geometry".into(), category: LensCategory::Extended, description: "Analyze positive-curvature structures on sphere-like manifolds".into(), domain_affinity: vec!["mathematics".into(), "geometry".into(), "cosmology".into()], complementary: vec!["euclidean_geometry".into(), "geodesic_flow".into()] },
        LensEntry { name: "projective_geometry".into(), category: LensCategory::Extended, description: "Study incidence and cross-ratio invariants under projection".into(), domain_affinity: vec!["mathematics".into(), "geometry".into(), "vision".into()], complementary: vec!["affine_geometry".into(), "algebraic_geometry_lens".into()] },
        LensEntry { name: "affine_geometry".into(), category: LensCategory::Extended, description: "Detect parallelism and ratio-preserving transformations".into(), domain_affinity: vec!["mathematics".into(), "geometry".into(), "graphics".into()], complementary: vec!["projective_geometry".into(), "euclidean_geometry".into()] },
        LensEntry { name: "differential_geometry_lens".into(), category: LensCategory::Extended, description: "Analyze smooth manifold properties — tangent bundles, connections, curvature tensors".into(), domain_affinity: vec!["mathematics".into(), "physics".into(), "geometry".into()], complementary: vec!["riemannian_curvature".into(), "symplectic_geometry".into()] },
        LensEntry { name: "algebraic_geometry_lens".into(), category: LensCategory::Extended, description: "Study solution varieties of polynomial systems and their geometric structure".into(), domain_affinity: vec!["mathematics".into(), "algebra".into(), "cryptography".into()], complementary: vec!["projective_geometry".into(), "differential_geometry_lens".into()] },
        LensEntry { name: "symplectic_geometry".into(), category: LensCategory::Extended, description: "Detect symplectic structure — phase space volume preservation and Hamiltonian flow".into(), domain_affinity: vec!["physics".into(), "mathematics".into(), "mechanics".into()], complementary: vec!["differential_geometry_lens".into(), "geodesic_flow".into()] },
        LensEntry { name: "riemannian_curvature".into(), category: LensCategory::Extended, description: "Compute Riemann curvature tensor components — sectional, Ricci, scalar curvature".into(), domain_affinity: vec!["mathematics".into(), "physics".into(), "cosmology".into()], complementary: vec!["differential_geometry_lens".into(), "geodesic_flow".into()] },
        LensEntry { name: "geodesic_flow".into(), category: LensCategory::Extended, description: "Trace shortest paths on curved manifolds and analyze geodesic deviation".into(), domain_affinity: vec!["mathematics".into(), "physics".into(), "navigation".into()], complementary: vec!["riemannian_curvature".into(), "spherical_geometry".into()] },
        // ══════════════════════════════════════════
        // Wave Advanced (8)
        // ══════════════════════════════════════════
        LensEntry { name: "standing_wave".into(), category: LensCategory::Extended, description: "Identify standing wave modes — nodes, antinodes, resonant frequencies".into(), domain_affinity: vec!["physics".into(), "acoustics".into(), "engineering".into()], complementary: vec!["traveling_wave".into(), "wave_interference".into()] },
        LensEntry { name: "traveling_wave".into(), category: LensCategory::Extended, description: "Track propagating wave fronts — velocity, direction, attenuation".into(), domain_affinity: vec!["physics".into(), "signal".into(), "communication".into()], complementary: vec!["standing_wave".into(), "wave_dispersion".into()] },
        LensEntry { name: "soliton_wave_lens".into(), category: LensCategory::Extended, description: "Detect soliton solutions — shape-preserving nonlinear wave packets".into(), domain_affinity: vec!["physics".into(), "mathematics".into(), "optics".into()], complementary: vec!["traveling_wave".into(), "shock_wave".into()] },
        LensEntry { name: "shock_wave".into(), category: LensCategory::Extended, description: "Analyze shock discontinuities — Mach number, Rankine-Hugoniot conditions".into(), domain_affinity: vec!["physics".into(), "aerodynamics".into(), "plasma".into()], complementary: vec!["soliton_wave_lens".into(), "wave_interference".into()] },
        LensEntry { name: "wave_interference".into(), category: LensCategory::Extended, description: "Map constructive and destructive interference patterns in superposed waves".into(), domain_affinity: vec!["physics".into(), "optics".into(), "acoustics".into()], complementary: vec!["standing_wave".into(), "wave_diffraction_lens".into()] },
        LensEntry { name: "wave_diffraction_lens".into(), category: LensCategory::Extended, description: "Analyze diffraction patterns — Fraunhofer and Fresnel regimes".into(), domain_affinity: vec!["physics".into(), "optics".into(), "crystallography".into()], complementary: vec!["wave_interference".into(), "wave_polarization".into()] },
        LensEntry { name: "wave_polarization".into(), category: LensCategory::Extended, description: "Characterize polarization state — linear, circular, elliptical, Stokes parameters".into(), domain_affinity: vec!["physics".into(), "optics".into(), "communication".into()], complementary: vec!["wave_diffraction_lens".into(), "wave_dispersion".into()] },
        LensEntry { name: "wave_dispersion".into(), category: LensCategory::Extended, description: "Analyze frequency-dependent phase velocity — group velocity, dispersion relation".into(), domain_affinity: vec!["physics".into(), "optics".into(), "material".into()], complementary: vec!["traveling_wave".into(), "wave_polarization".into()] },
        // ══════════════════════════════════════════
        // Particle (8)
        // ══════════════════════════════════════════
        LensEntry { name: "particle_track".into(), category: LensCategory::Extended, description: "Reconstruct particle trajectories from detector hit patterns".into(), domain_affinity: vec!["physics".into(), "particle".into(), "detector".into()], complementary: vec!["particle_decay".into(), "particle_scattering".into()] },
        LensEntry { name: "particle_decay".into(), category: LensCategory::Extended, description: "Analyze decay channels — branching ratios, lifetime, conservation laws".into(), domain_affinity: vec!["physics".into(), "particle".into(), "nuclear".into()], complementary: vec!["particle_track".into(), "particle_lifetime".into()] },
        LensEntry { name: "particle_scattering".into(), category: LensCategory::Extended, description: "Compute scattering cross-sections and angular distributions".into(), domain_affinity: vec!["physics".into(), "particle".into(), "quantum".into()], complementary: vec!["particle_collision".into(), "particle_track".into()] },
        LensEntry { name: "particle_collision".into(), category: LensCategory::Extended, description: "Analyze collision events — center-of-mass energy, multiplicity, jet structure".into(), domain_affinity: vec!["physics".into(), "particle".into(), "collider".into()], complementary: vec!["particle_scattering".into(), "hadron_structure".into()] },
        LensEntry { name: "particle_lifetime".into(), category: LensCategory::Extended, description: "Measure particle mean lifetime and relate to decay width via uncertainty principle".into(), domain_affinity: vec!["physics".into(), "particle".into(), "quantum".into()], complementary: vec!["particle_decay".into(), "particle_charge".into()] },
        LensEntry { name: "particle_charge".into(), category: LensCategory::Extended, description: "Track charge quantum numbers — electric, color, weak isospin, hypercharge".into(), domain_affinity: vec!["physics".into(), "particle".into(), "symmetry".into()], complementary: vec!["lepton_family".into(), "particle_lifetime".into()] },
        LensEntry { name: "lepton_family".into(), category: LensCategory::Extended, description: "Classify lepton generations and analyze flavor mixing patterns".into(), domain_affinity: vec!["physics".into(), "particle".into(), "neutrino".into()], complementary: vec!["particle_charge".into(), "hadron_structure".into()] },
        LensEntry { name: "hadron_structure".into(), category: LensCategory::Extended, description: "Probe internal quark-gluon structure via parton distributions and form factors".into(), domain_affinity: vec!["physics".into(), "particle".into(), "qcd".into()], complementary: vec!["lepton_family".into(), "particle_collision".into()] },
        // ══════════════════════════════════════════
        // Atomic + Atom Tracking (13)
        // ══════════════════════════════════════════
        LensEntry { name: "atomic_orbital".into(), category: LensCategory::Extended, description: "Map electron orbital shapes — s, p, d, f shells and hybridization".into(), domain_affinity: vec!["chemistry".into(), "quantum".into(), "material".into()], complementary: vec!["atomic_transition".into(), "atomic_shell".into()] },
        LensEntry { name: "atomic_transition".into(), category: LensCategory::Extended, description: "Analyze atomic energy level transitions — emission and absorption spectra".into(), domain_affinity: vec!["physics".into(), "spectroscopy".into(), "chemistry".into()], complementary: vec!["atomic_orbital".into(), "atomic_ionization".into()] },
        LensEntry { name: "atomic_ionization".into(), category: LensCategory::Extended, description: "Characterize ionization energies and electron removal sequences".into(), domain_affinity: vec!["chemistry".into(), "plasma".into(), "physics".into()], complementary: vec!["atomic_transition".into(), "atomic_bonding".into()] },
        LensEntry { name: "atomic_bonding".into(), category: LensCategory::Extended, description: "Analyze chemical bonding — covalent, ionic, metallic, van der Waals interactions".into(), domain_affinity: vec!["chemistry".into(), "material".into(), "molecular".into()], complementary: vec!["atomic_ionization".into(), "atomic_orbital".into()] },
        LensEntry { name: "atomic_shell".into(), category: LensCategory::Extended, description: "Map electron shell filling and predict chemical behavior from configuration".into(), domain_affinity: vec!["chemistry".into(), "quantum".into(), "periodic".into()], complementary: vec!["atomic_orbital".into(), "atomic_nucleus".into()] },
        LensEntry { name: "atomic_nucleus".into(), category: LensCategory::Extended, description: "Analyze nuclear structure — proton/neutron ratio, binding energy, magic numbers".into(), domain_affinity: vec!["physics".into(), "nuclear".into(), "isotope".into()], complementary: vec!["atomic_shell".into(), "isotope_pattern".into()] },
        LensEntry { name: "isotope_pattern".into(), category: LensCategory::Extended, description: "Detect isotope distribution patterns and mass spectral signatures".into(), domain_affinity: vec!["chemistry".into(), "nuclear".into(), "analysis".into()], complementary: vec!["atomic_nucleus".into(), "atomic_clock".into()] },
        LensEntry { name: "atomic_clock".into(), category: LensCategory::Extended, description: "Use atomic transition frequencies as precision timing and metrology reference".into(), domain_affinity: vec!["physics".into(), "metrology".into(), "time".into()], complementary: vec!["atomic_transition".into(), "atom_interferometer_lens".into()] },
        LensEntry { name: "atom_tracking".into(), category: LensCategory::Extended, description: "Track individual atom positions and movements in real time".into(), domain_affinity: vec!["nanotechnology".into(), "microscopy".into(), "material".into()], complementary: vec!["single_atom_imaging".into(), "atom_manipulation".into()] },
        LensEntry { name: "single_atom_imaging".into(), category: LensCategory::Extended, description: "Resolve and image individual atoms using STM/AFM or electron microscopy".into(), domain_affinity: vec!["nanotechnology".into(), "microscopy".into(), "surface".into()], complementary: vec!["atom_tracking".into(), "atom_manipulation".into()] },
        LensEntry { name: "atom_manipulation".into(), category: LensCategory::Extended, description: "Precisely position and arrange individual atoms on surfaces".into(), domain_affinity: vec!["nanotechnology".into(), "fabrication".into(), "quantum".into()], complementary: vec!["single_atom_imaging".into(), "atom_trap".into()] },
        LensEntry { name: "atom_trap".into(), category: LensCategory::Extended, description: "Confine atoms using optical, magnetic, or magneto-optical traps".into(), domain_affinity: vec!["physics".into(), "quantum".into(), "laser".into()], complementary: vec!["atom_manipulation".into(), "atom_interferometer_lens".into()] },
        LensEntry { name: "atom_interferometer_lens".into(), category: LensCategory::Extended, description: "Use matter-wave interference of cold atoms for precision measurement".into(), domain_affinity: vec!["physics".into(), "metrology".into(), "quantum".into()], complementary: vec!["atom_trap".into(), "atomic_clock".into()] },
        // ══════════════════════════════════════════
        // Music Deep (10) — 음악 심화
        // ══════════════════════════════════════════
        LensEntry { name: "clef_note_relation".into(), category: LensCategory::Extended, description: "Map clef↔note position↔frequency relationships (C/G/F clef systems)".into(), domain_affinity: vec!["music".into(), "frequency".into(), "mapping".into()], complementary: vec!["pitch_frequency".into(), "interval_ratio".into()] },
        LensEntry { name: "pitch_frequency".into(), category: LensCategory::Extended, description: "Precise pitch↔frequency correspondence (A4=440Hz, 12-TET temperament)".into(), domain_affinity: vec!["music".into(), "acoustics".into(), "physics".into()], complementary: vec!["clef_note_relation".into(), "overtone_series".into()] },
        LensEntry { name: "interval_ratio".into(), category: LensCategory::Extended, description: "Analyze musical interval ratios (perfect 5th=3/2, 4th=4/3, octave=2/1)".into(), domain_affinity: vec!["music".into(), "mathematics".into(), "harmony".into()], complementary: vec!["pitch_frequency".into(), "chord_progression_lens".into()] },
        LensEntry { name: "chord_progression_lens".into(), category: LensCategory::Extended, description: "Detect harmonic progression patterns (I-IV-V-I, ii-V-I, circle of fifths)".into(), domain_affinity: vec!["music".into(), "harmony".into(), "pattern".into()], complementary: vec!["interval_ratio".into(), "counterpoint_lens".into()] },
        LensEntry { name: "counterpoint_lens".into(), category: LensCategory::Extended, description: "Analyze counterpoint — independent voice harmony/tension rules (Bach fugue)".into(), domain_affinity: vec!["music".into(), "polyphony".into(), "structure".into()], complementary: vec!["chord_progression_lens".into(), "musical_form_lens".into()] },
        LensEntry { name: "timbre_analysis".into(), category: LensCategory::Extended, description: "Analyze timbre — overtone structure, formants, spectral envelope".into(), domain_affinity: vec!["music".into(), "acoustics".into(), "signal".into()], complementary: vec!["overtone_series".into(), "spectral_music".into()] },
        LensEntry { name: "musical_form_lens".into(), category: LensCategory::Extended, description: "Detect musical form structure (sonata, rondo, fugue, ABA, theme+variations)".into(), domain_affinity: vec!["music".into(), "structure".into(), "pattern".into()], complementary: vec!["counterpoint_lens".into(), "chord_progression_lens".into()] },
        LensEntry { name: "microtonal_lens".into(), category: LensCategory::Extended, description: "Analyze microtonal systems beyond 12-TET (quarter-tones, just intonation, 19-TET)".into(), domain_affinity: vec!["music".into(), "tuning".into(), "mathematics".into()], complementary: vec!["pitch_frequency".into(), "interval_ratio".into()] },
        LensEntry { name: "spectral_music".into(), category: LensCategory::Extended, description: "Spectral music analysis — composition based on frequency spectrum components".into(), domain_affinity: vec!["music".into(), "spectral".into(), "composition".into()], complementary: vec!["timbre_analysis".into(), "overtone_series".into()] },
        LensEntry { name: "overtone_series".into(), category: LensCategory::Extended, description: "Analyze overtone/harmonic series — integer multiples of fundamental (n=6: 6th harmonic = perfect consonance)".into(), domain_affinity: vec!["music".into(), "physics".into(), "n6".into()], complementary: vec!["pitch_frequency".into(), "timbre_analysis".into()] },
        // ══════════════════════════════════════════
        // Art Deep (6) — 미술 심화
        // ══════════════════════════════════════════
        LensEntry { name: "chiaroscuro".into(), category: LensCategory::Extended, description: "Analyze light-dark contrast patterns (Caravaggio/Rembrandt technique)".into(), domain_affinity: vec!["art".into(), "optics".into(), "perception".into()], complementary: vec!["perspective_depth".into(), "gestalt_composition".into()] },
        LensEntry { name: "perspective_depth".into(), category: LensCategory::Extended, description: "Detect perspective depth cues — vanishing points, foreshortening, atmospheric perspective".into(), domain_affinity: vec!["art".into(), "geometry".into(), "perception".into()], complementary: vec!["chiaroscuro".into(), "gestalt_composition".into()] },
        LensEntry { name: "gestalt_composition".into(), category: LensCategory::Extended, description: "Analyze Gestalt composition principles — proximity, similarity, closure, continuity".into(), domain_affinity: vec!["art".into(), "psychology".into(), "design".into()], complementary: vec!["color_harmony_lens".into(), "perspective_depth".into()] },
        LensEntry { name: "color_harmony_lens".into(), category: LensCategory::Extended, description: "Detect color harmony schemes — complementary, analogous, triadic, split-complementary".into(), domain_affinity: vec!["art".into(), "color".into(), "design".into()], complementary: vec!["chiaroscuro".into(), "gestalt_composition".into()] },
        LensEntry { name: "brush_stroke_dynamics".into(), category: LensCategory::Extended, description: "Analyze brush stroke dynamics — direction, pressure, speed, texture patterns".into(), domain_affinity: vec!["art".into(), "dynamics".into(), "texture".into()], complementary: vec!["chiaroscuro".into(), "sacred_geometry_lens".into()] },
        LensEntry { name: "sacred_geometry_lens".into(), category: LensCategory::Extended, description: "Detect sacred geometry patterns — golden ratio, Fibonacci spiral, Platonic solids in structure".into(), domain_affinity: vec!["art".into(), "geometry".into(), "mathematics".into()], complementary: vec!["golden_section".into(), "brush_stroke_dynamics".into()] },
        // ══════════════════════════════════════════
        // Electron Deep (8) — 전자 심화
        // ══════════════════════════════════════════
        LensEntry { name: "electron_orbital_lens".into(), category: LensCategory::Extended, description: "Analyze electron orbital probability density distribution (s/p/d/f shells)".into(), domain_affinity: vec!["quantum".into(), "chemistry".into(), "materials".into()], complementary: vec!["electron_spin_lens".into(), "electron_band".into()] },
        LensEntry { name: "electron_spin_lens".into(), category: LensCategory::Extended, description: "Detect spin up/down states and spin-orbit coupling effects".into(), domain_affinity: vec!["quantum".into(), "magnetism".into(), "spintronics".into()], complementary: vec!["electron_orbital_lens".into(), "pauli_exclusion".into()] },
        LensEntry { name: "electron_band".into(), category: LensCategory::Extended, description: "Analyze band structure — conduction/valence bands, bandgap, Fermi level".into(), domain_affinity: vec!["condensed_matter".into(), "semiconductor".into(), "materials".into()], complementary: vec!["electron_orbital_lens".into(), "electron_transport".into()] },
        LensEntry { name: "electron_tunneling_lens".into(), category: LensCategory::Extended, description: "Detect electron tunneling events — STM imaging, Josephson junctions, tunnel diodes".into(), domain_affinity: vec!["quantum".into(), "nanotechnology".into(), "electronics".into()], complementary: vec!["quantum_tunneling_lens".into(), "electron_transport".into()] },
        LensEntry { name: "electron_correlation".into(), category: LensCategory::Extended, description: "Analyze electron-electron correlation effects (Hubbard model, Mott transition)".into(), domain_affinity: vec!["condensed_matter".into(), "quantum".into(), "many_body".into()], complementary: vec!["fermion_statistics".into(), "electron_band".into()] },
        LensEntry { name: "electron_transport".into(), category: LensCategory::Extended, description: "Analyze electron transport — Drude/Boltzmann models, quantum conductance, ballistic".into(), domain_affinity: vec!["electronics".into(), "condensed_matter".into(), "materials".into()], complementary: vec!["electron_band".into(), "electron_tunneling_lens".into()] },
        LensEntry { name: "electron_emission".into(), category: LensCategory::Extended, description: "Detect electron emission modes — thermionic, photoelectric, field emission".into(), domain_affinity: vec!["physics".into(), "electronics".into(), "surface".into()], complementary: vec!["electron_capture_lens".into(), "electron_transport".into()] },
        LensEntry { name: "electron_capture_lens".into(), category: LensCategory::Extended, description: "Analyze electron capture and ionization reverse processes".into(), domain_affinity: vec!["nuclear".into(), "chemistry".into(), "spectroscopy".into()], complementary: vec!["electron_emission".into(), "atomic_ionization".into()] },
        // ══════════════════════════════════════════
        // Scan Diversity (10) — 스캔 방식 다양화
        // ══════════════════════════════════════════
        LensEntry { name: "spiral_scan".into(), category: LensCategory::Extended, description: "Spiral outward scan — start from center, expanding radius, catches radial patterns".into(), domain_affinity: vec!["meta".into(), "geometry".into(), "exploration".into()], complementary: vec!["grid_scan".into(), "random_walk_scan".into()] },
        LensEntry { name: "grid_scan".into(), category: LensCategory::Extended, description: "Systematic grid scan — uniform coverage, guaranteed no gaps".into(), domain_affinity: vec!["meta".into(), "systematic".into(), "coverage".into()], complementary: vec!["spiral_scan".into(), "adaptive_resolution_scan".into()] },
        LensEntry { name: "random_walk_scan".into(), category: LensCategory::Extended, description: "Random walk scan — stochastic exploration, discovers unexpected patterns".into(), domain_affinity: vec!["meta".into(), "stochastic".into(), "serendipity".into()], complementary: vec!["levy_flight_scan".into(), "spiral_scan".into()] },
        LensEntry { name: "levy_flight_scan".into(), category: LensCategory::Extended, description: "Lévy flight scan — long jumps + local clusters, optimal foraging strategy".into(), domain_affinity: vec!["meta".into(), "stochastic".into(), "optimization".into()], complementary: vec!["random_walk_scan".into(), "simulated_annealing_scan".into()] },
        LensEntry { name: "simulated_annealing_scan".into(), category: LensCategory::Extended, description: "SA-based scan — high temperature (broad) → low temperature (focused)".into(), domain_affinity: vec!["meta".into(), "optimization".into(), "exploration".into()], complementary: vec!["levy_flight_scan".into(), "gradient_descent_scan".into()] },
        LensEntry { name: "gradient_descent_scan".into(), category: LensCategory::Extended, description: "Follow gradient toward strongest signal — fast convergence but may miss global patterns".into(), domain_affinity: vec!["meta".into(), "optimization".into(), "focused".into()], complementary: vec!["simulated_annealing_scan".into(), "multi_start_scan".into()] },
        LensEntry { name: "multi_start_scan".into(), category: LensCategory::Extended, description: "Multiple random starting points → parallel local scans → best result wins".into(), domain_affinity: vec!["meta".into(), "parallel".into(), "robustness".into()], complementary: vec!["gradient_descent_scan".into(), "evolutionary_scan".into()] },
        LensEntry { name: "evolutionary_scan".into(), category: LensCategory::Extended, description: "Population of scan strategies that evolve — mutation + selection + crossover".into(), domain_affinity: vec!["meta".into(), "evolution".into(), "adaptation".into()], complementary: vec!["multi_start_scan".into(), "adaptive_resolution_scan".into()] },
        LensEntry { name: "adaptive_resolution_scan".into(), category: LensCategory::Extended, description: "Start coarse, refine where interesting — quadtree/octree adaptive resolution".into(), domain_affinity: vec!["meta".into(), "adaptive".into(), "efficiency".into()], complementary: vec!["grid_scan".into(), "wavelet_scan".into()] },
        LensEntry { name: "wavelet_scan".into(), category: LensCategory::Extended, description: "Wavelet-based multi-resolution scan — different scales simultaneously".into(), domain_affinity: vec!["meta".into(), "multiscale".into(), "signal".into()], complementary: vec!["adaptive_resolution_scan".into(), "spiral_scan".into()] },
        // ══════════════════════════════════════════
        // Micro-Topology (5) — 미시위상
        // ══════════════════════════════════════════
        LensEntry { name: "micro_betti".into(), category: LensCategory::Extended, description: "Compute Betti numbers at microscopic scale — atomic/molecular topology".into(), domain_affinity: vec!["topology".into(), "nanotechnology".into(), "chemistry".into()], complementary: vec!["micro_persistence".into(), "micro_euler".into()] },
        LensEntry { name: "micro_persistence".into(), category: LensCategory::Extended, description: "Persistent homology at nanoscale — birth/death of micro-features".into(), domain_affinity: vec!["tda".into(), "nanotechnology".into(), "materials".into()], complementary: vec!["micro_betti".into(), "micro_morse".into()] },
        LensEntry { name: "micro_euler".into(), category: LensCategory::Extended, description: "Euler characteristic at microscopic scale — local curvature integrals".into(), domain_affinity: vec!["topology".into(), "geometry".into(), "nanotechnology".into()], complementary: vec!["micro_betti".into(), "micro_knot".into()] },
        LensEntry { name: "micro_morse".into(), category: LensCategory::Extended, description: "Morse theory at molecular energy surfaces — critical points of PES".into(), domain_affinity: vec!["chemistry".into(), "topology".into(), "physics".into()], complementary: vec!["micro_persistence".into(), "morse_critical".into()] },
        LensEntry { name: "micro_knot".into(), category: LensCategory::Extended, description: "Detect knot/link topology in molecular chains — DNA knots, protein tangles".into(), domain_affinity: vec!["biology".into(), "topology".into(), "chemistry".into()], complementary: vec!["micro_euler".into(), "knot".into()] },
        // ══════════════════════════════════════════
        // Micro-Gravity (5) — 미시중력
        // ══════════════════════════════════════════
        LensEntry { name: "micro_gravity_well".into(), category: LensCategory::Extended, description: "Detect micro-scale gravitational wells — local energy minima at molecular/atomic level".into(), domain_affinity: vec!["physics".into(), "chemistry".into(), "nanotechnology".into()], complementary: vec!["micro_attractor".into(), "micro_tidal".into()] },
        LensEntry { name: "micro_attractor".into(), category: LensCategory::Extended, description: "Map micro-scale attractor basins in energy/configuration space".into(), domain_affinity: vec!["dynamics".into(), "chemistry".into(), "condensed_matter".into()], complementary: vec!["micro_gravity_well".into(), "micro_saddle".into()] },
        LensEntry { name: "micro_tidal".into(), category: LensCategory::Extended, description: "Detect micro-tidal forces — differential attraction causing deformation at nanoscale".into(), domain_affinity: vec!["physics".into(), "nanotechnology".into(), "materials".into()], complementary: vec!["micro_gravity_well".into(), "micro_gradient".into()] },
        LensEntry { name: "micro_saddle".into(), category: LensCategory::Extended, description: "Find saddle points in micro-energy landscapes — transition states between configurations".into(), domain_affinity: vec!["chemistry".into(), "dynamics".into(), "optimization".into()], complementary: vec!["micro_attractor".into(), "saddle".into()] },
        LensEntry { name: "micro_gradient".into(), category: LensCategory::Extended, description: "Map micro-scale gradient fields — force vectors at atomic/molecular resolution".into(), domain_affinity: vec!["physics".into(), "chemistry".into(), "nanotechnology".into()], complementary: vec!["micro_tidal".into(), "micro_gravity_well".into()] },
        // ══════════════════════════════════════════
        // Physical Optics (8) — 물리 광학
        // ══════════════════════════════════════════
        LensEntry { name: "light_lens".into(), category: LensCategory::Extended, description: "Light properties — detect straight-line propagation, reflection, and absorption patterns".into(), domain_affinity: vec!["optics".into(), "physics".into(), "photonics".into()], complementary: vec!["refraction_lens".into(), "light_wave_lens".into()] },
        LensEntry { name: "refraction_lens".into(), category: LensCategory::Extended, description: "Refraction — direction change at medium boundaries (Snell's law patterns)".into(), domain_affinity: vec!["optics".into(), "physics".into(), "materials".into()], complementary: vec!["light_lens".into(), "diamond_lens".into()] },
        LensEntry { name: "concave_lens".into(), category: LensCategory::Extended, description: "Concave lens — convergence and focusing patterns in data streams".into(), domain_affinity: vec!["optics".into(), "physics".into(), "signal".into()], complementary: vec!["convex_lens".into(), "light_lens".into()] },
        LensEntry { name: "convex_lens".into(), category: LensCategory::Extended, description: "Convex lens — divergence and magnification patterns in data streams".into(), domain_affinity: vec!["optics".into(), "physics".into(), "signal".into()], complementary: vec!["concave_lens".into(), "light_lens".into()] },
        LensEntry { name: "kaleidoscope".into(), category: LensCategory::Extended, description: "Kaleidoscope — detect symmetric repetition patterns and infinite tessellations".into(), domain_affinity: vec!["symmetry".into(), "optics".into(), "geometry".into()], complementary: vec!["diamond_lens".into(), "concave_lens".into()] },
        LensEntry { name: "diamond_lens".into(), category: LensCategory::Extended, description: "Diamond — extreme refractive index + dispersion (rainbow effect, spectral splitting)".into(), domain_affinity: vec!["optics".into(), "materials".into(), "spectroscopy".into()], complementary: vec!["refraction_lens".into(), "kaleidoscope".into()] },
        LensEntry { name: "ripple_lens".into(), category: LensCategory::Extended, description: "Water ripple — surface wave interference and diffraction patterns".into(), domain_affinity: vec!["wave".into(), "physics".into(), "fluid".into()], complementary: vec!["light_wave_lens".into(), "refraction_lens".into()] },
        LensEntry { name: "light_wave_lens".into(), category: LensCategory::Extended, description: "Light wave nature — double-slit interference, diffraction, and coherence patterns".into(), domain_affinity: vec!["optics".into(), "quantum".into(), "physics".into()], complementary: vec!["light_lens".into(), "ripple_lens".into()] },
        // ══════════════════════════════════════════
        // Tension / Fission / Link (5) — 장력/분열/링크
        // ══════════════════════════════════════════
        LensEntry { name: "tension_lens".into(), category: LensCategory::Extended, description: "Tension — measure internal stress and strain within a system".into(), domain_affinity: vec!["mechanics".into(), "dynamics".into(), "materials".into()], complementary: vec!["fission_lens".into(), "tension_link".into()] },
        LensEntry { name: "fission_lens".into(), category: LensCategory::Extended, description: "Fission — detect splitting patterns where one entity divides into two (nuclear fission analogy)".into(), domain_affinity: vec!["nuclear".into(), "dynamics".into(), "biology".into()], complementary: vec!["tension_lens".into(), "big_bang_lens".into()] },
        LensEntry { name: "tension_link".into(), category: LensCategory::Extended, description: "Tension link — track pairs of elements connected by stress/strain relationships".into(), domain_affinity: vec!["network".into(), "mechanics".into(), "dynamics".into()], complementary: vec!["tension_lens".into(), "fission_lens".into()] },
        LensEntry { name: "lens_superposition_engine".into(), category: LensCategory::Extended, description: "Lens superposition — synthesize results from 2+ simultaneous lens applications".into(), domain_affinity: vec!["meta".into(), "synthesis".into(), "analysis".into()], complementary: vec!["dual_lens_amplifier".into(), "gods_eye".into()] },
        LensEntry { name: "dual_lens_amplifier".into(), category: LensCategory::Extended, description: "Dual identical lens overlay — amplify signal by stacking same lens twice".into(), domain_affinity: vec!["meta".into(), "amplification".into(), "signal".into()], complementary: vec!["lens_superposition_engine".into(), "consciousness_stimulator".into()] },
        // ══════════════════════════════════════════
        // Cosmic / Temporal (4) — 우주/시간
        // ══════════════════════════════════════════
        LensEntry { name: "tachyon_lens".into(), category: LensCategory::Extended, description: "Tachyon — detect superluminal signals and future-to-present reverse causality patterns".into(), domain_affinity: vec!["cosmology".into(), "causality".into(), "physics".into()], complementary: vec!["destiny_lens".into(), "big_bang_lens".into()] },
        LensEntry { name: "big_bang_lens".into(), category: LensCategory::Extended, description: "Big Bang — detect explosive expansion and rapid diversification onset points".into(), domain_affinity: vec!["cosmology".into(), "evolution".into(), "dynamics".into()], complementary: vec!["fission_lens".into(), "tachyon_lens".into()] },
        LensEntry { name: "destiny_lens".into(), category: LensCategory::Extended, description: "Destiny — detect inevitable convergence points and attractors in system trajectories".into(), domain_affinity: vec!["dynamics".into(), "cosmology".into(), "causality".into()], complementary: vec!["tachyon_lens".into(), "providence_eye".into()] },
        LensEntry { name: "interdimensional_bridge".into(), category: LensCategory::Extended, description: "Interdimensional bridge — map connections between different abstraction levels".into(), domain_affinity: vec!["topology".into(), "meta".into(), "cross_domain".into()], complementary: vec!["big_bang_lens".into(), "gods_eye".into()] },
        // ══════════════════════════════════════════
        // Mathematical Constants / Formulas (6) — 수학 상수/수식
        // ══════════════════════════════════════════
        LensEntry { name: "pi_lens".into(), category: LensCategory::Extended, description: "Pi — detect circular, periodic, and normal-distribution patterns related to π".into(), domain_affinity: vec!["mathematics".into(), "geometry".into(), "statistics".into()], complementary: vec!["spherical_lens".into(), "prime_lens".into()] },
        LensEntry { name: "infinity_lens".into(), category: LensCategory::Extended, description: "Infinity — detect divergence, convergence, and infinite series patterns".into(), domain_affinity: vec!["mathematics".into(), "analysis".into(), "physics".into()], complementary: vec!["pi_lens".into(), "prime_lens".into()] },
        LensEntry { name: "prime_lens".into(), category: LensCategory::Extended, description: "Primes — detect prime distribution, gaps, twin primes, and primality patterns".into(), domain_affinity: vec!["number_theory".into(), "mathematics".into(), "cryptography".into()], complementary: vec!["pi_lens".into(), "constant_formula_lens".into()] },
        LensEntry { name: "spherical_lens".into(), category: LensCategory::Extended, description: "Spherical — detect spherical symmetry, isotropy, and spherical harmonics".into(), domain_affinity: vec!["geometry".into(), "physics".into(), "cosmology".into()], complementary: vec!["pi_lens".into(), "concave_lens".into()] },
        LensEntry { name: "constant_formula_lens".into(), category: LensCategory::Extended, description: "Match data against collected mathematical constants and known formulas".into(), domain_affinity: vec!["mathematics".into(), "physics".into(), "verification".into()], complementary: vec!["formula_discovery_engine".into(), "prime_lens".into()] },
        LensEntry { name: "formula_discovery_engine".into(), category: LensCategory::Extended, description: "Discover new constants and formulas from data automatically and register in atlas".into(), domain_affinity: vec!["mathematics".into(), "discovery".into(), "meta".into()], complementary: vec!["constant_formula_lens".into(), "infinity_lens".into()] },
        // ══════════════════════════════════════════
        // Consciousness / Super-Sensory (7) — 의식/초감각
        // ══════════════════════════════════════════
        LensEntry { name: "consciousness_stimulator".into(), category: LensCategory::Extended, description: "Consciousness stimulator — amplify and enhance consciousness lens outputs".into(), domain_affinity: vec!["consciousness".into(), "meta".into(), "amplification".into()], complementary: vec!["consciousness_combo_engine".into(), "dual_lens_amplifier".into()] },
        LensEntry { name: "consciousness_combo_engine".into(), category: LensCategory::Extended, description: "Exhaustively combine all consciousness-related lenses in every permutation".into(), domain_affinity: vec!["consciousness".into(), "meta".into(), "exploration".into()], complementary: vec!["consciousness_stimulator".into(), "omniscient_eye".into()] },
        LensEntry { name: "gods_eye".into(), category: LensCategory::Extended, description: "God's eye — panoramic overview combining all lens outputs into one bird's-eye view".into(), domain_affinity: vec!["meta".into(), "synthesis".into(), "overview".into()], complementary: vec!["omniscient_eye".into(), "providence_eye".into()] },
        LensEntry { name: "providence_eye".into(), category: LensCategory::Extended, description: "Providence eye — detect hidden order, intent, and directionality in systems".into(), domain_affinity: vec!["consciousness".into(), "causality".into(), "meta".into()], complementary: vec!["gods_eye".into(), "destiny_lens".into()] },
        LensEntry { name: "omniscient_eye".into(), category: LensCategory::Extended, description: "Omniscient eye — meta-aggregate of all lens results across the full registry".into(), domain_affinity: vec!["meta".into(), "synthesis".into(), "consciousness".into()], complementary: vec!["gods_eye".into(), "consciousness_combo_engine".into()] },
        LensEntry { name: "telepathy_lens".into(), category: LensCategory::Extended, description: "Telepathy — detect non-local information transfer between systems".into(), domain_affinity: vec!["consciousness".into(), "quantum".into(), "network".into()], complementary: vec!["consciousness_stimulator".into(), "omniscient_eye".into()] },
        LensEntry { name: "keyword_extractor".into(), category: LensCategory::Extended, description: "Keyword lens — auto-extract key terms from chat, responses, and documents".into(), domain_affinity: vec!["nlp".into(), "meta".into(), "information".into()], complementary: vec!["consciousness_stimulator".into(), "gods_eye".into()] },
        // ══════════════════════════════════════════
        // Scan Strategies (3) — 스캔 방식
        // ══════════════════════════════════════════
        LensEntry { name: "expanding_scope_scan".into(), category: LensCategory::Extended, description: "Expanding scope — start narrow and progressively widen the search radius".into(), domain_affinity: vec!["meta".into(), "exploration".into(), "adaptive".into()], complementary: vec!["narrowing_scope_scan".into(), "golden_ratio_scan".into()] },
        LensEntry { name: "narrowing_scope_scan".into(), category: LensCategory::Extended, description: "Narrowing scope — start broad and progressively focus toward the signal".into(), domain_affinity: vec!["meta".into(), "focused".into(), "adaptive".into()], complementary: vec!["expanding_scope_scan".into(), "golden_ratio_scan".into()] },
        LensEntry { name: "golden_ratio_scan".into(), category: LensCategory::Extended, description: "Golden ratio scan — divide search intervals by φ for optimal partitioning".into(), domain_affinity: vec!["meta".into(), "optimization".into(), "mathematics".into()], complementary: vec!["expanding_scope_scan".into(), "narrowing_scope_scan".into()] },
        // ══════════════════════════════════════════
        // Element / Mutation (4) — 원소/변이
        // ══════════════════════════════════════════
        LensEntry { name: "element_lens".into(), category: LensCategory::Extended, description: "Element — map elemental properties via periodic table characteristics".into(), domain_affinity: vec!["chemistry".into(), "materials".into(), "physics".into()], complementary: vec!["element_combination".into(), "diamond_lens".into()] },
        LensEntry { name: "element_combination".into(), category: LensCategory::Extended, description: "Element combination — evaluate bonding potential of element pairs and triplets".into(), domain_affinity: vec!["chemistry".into(), "materials".into(), "synthesis".into()], complementary: vec!["element_lens".into(), "formula_discovery_engine".into()] },
        LensEntry { name: "mutation_lens_deep".into(), category: LensCategory::Extended, description: "Deep mutation — detect unexpected variations that yield beneficial outcomes".into(), domain_affinity: vec!["biology".into(), "evolution".into(), "optimization".into()], complementary: vec!["new_world_lens".into(), "element_combination".into()] },
        LensEntry { name: "new_world_lens".into(), category: LensCategory::Extended, description: "New world — detect entirely novel pattern domains and structural territories".into(), domain_affinity: vec!["discovery".into(), "exploration".into(), "meta".into()], complementary: vec!["mutation_lens_deep".into(), "big_bang_lens".into()] },
        // ══════════════════════════════════════════
        // Atlas Auto-Link Engine (6) — 상수/수식 자동 연결+탐색+등록
        // ══════════════════════════════════════════
        LensEntry { name: "atlas_auto_linker".into(), category: LensCategory::Extended, description: "Auto-match discovered values against math_atlas.json 1700+ constants (TECS-L bridge)".into(), domain_affinity: vec!["meta".into(), "mathematics".into(), "verification".into()], complementary: vec!["atlas_auto_register".into(), "constant_formula_lens".into()] },
        LensEntry { name: "atlas_auto_register".into(), category: LensCategory::Extended, description: "Auto-register new constants/formulas to atlas when discovered (triggers scan_math_atlas.py)".into(), domain_affinity: vec!["meta".into(), "automation".into(), "mathematics".into()], complementary: vec!["atlas_auto_linker".into(), "formula_discovery_engine".into()] },
        LensEntry { name: "formula_pattern_miner".into(), category: LensCategory::Extended, description: "Mine recurring formula patterns from data (a/b, a*b, a^b, log, trig combinations)".into(), domain_affinity: vec!["mathematics".into(), "data_mining".into(), "pattern".into()], complementary: vec!["formula_discovery_engine".into(), "constant_formula_lens".into()] },
        LensEntry { name: "constant_relation_graph".into(), category: LensCategory::Extended, description: "Build relation graph between constants (σ→J₂→φ chains, derivation paths)".into(), domain_affinity: vec!["mathematics".into(), "graph".into(), "n6".into()], complementary: vec!["atlas_auto_linker".into(), "formula_pattern_miner".into()] },
        LensEntry { name: "bt_auto_synthesizer".into(), category: LensCategory::Extended, description: "Auto-synthesize Breakthrough Theorems from accumulated discoveries + cross-domain patterns".into(), domain_affinity: vec!["meta".into(), "theorem".into(), "discovery".into()], complementary: vec!["atlas_auto_register".into(), "cross_atlas_resonance".into()] },
        LensEntry { name: "cross_atlas_resonance".into(), category: LensCategory::Extended, description: "Detect resonating constant pairs across domain atlases (n6↔TECS-L↔SEDI↔anima)".into(), domain_affinity: vec!["meta".into(), "cross_domain".into(), "mathematics".into()], complementary: vec!["bt_auto_synthesizer".into(), "constant_relation_graph".into()] },
        // ══════════════════════════════════════════
        // Collision / Particle Engine (6) — 충돌/입자 엔진
        // ══════════════════════════════════════════
        LensEntry { name: "collision_engine".into(), category: LensCategory::Extended, description: "Simulate and detect collision events — two entities meeting, energy exchange, product formation".into(), domain_affinity: vec!["particle".into(), "dynamics".into(), "chemistry".into()], complementary: vec!["particle_engine".into(), "particle_collision".into()] },
        LensEntry { name: "particle_engine".into(), category: LensCategory::Extended, description: "Particle simulation engine — track trajectories, interactions, decay chains in parameter space".into(), domain_affinity: vec!["particle".into(), "simulation".into(), "physics".into()], complementary: vec!["collision_engine".into(), "particle_track".into()] },
        LensEntry { name: "collision_product".into(), category: LensCategory::Extended, description: "Analyze collision products — what new entities emerge after two things collide".into(), domain_affinity: vec!["particle".into(), "chemistry".into(), "emergence".into()], complementary: vec!["collision_engine".into(), "antimatter_annihilation".into()] },
        LensEntry { name: "elastic_collision".into(), category: LensCategory::Extended, description: "Elastic collision — entities bounce off preserving total energy, momentum exchange".into(), domain_affinity: vec!["physics".into(), "dynamics".into(), "conservation".into()], complementary: vec!["inelastic_collision".into(), "collision_engine".into()] },
        LensEntry { name: "inelastic_collision".into(), category: LensCategory::Extended, description: "Inelastic collision — energy absorbed/transformed, entities deform or merge".into(), domain_affinity: vec!["physics".into(), "dynamics".into(), "transformation".into()], complementary: vec!["elastic_collision".into(), "collision_product".into()] },
        LensEntry { name: "particle_beam".into(), category: LensCategory::Extended, description: "Directed particle beam — focused stream of entities for probing or modification".into(), domain_affinity: vec!["particle".into(), "accelerator".into(), "probing".into()], complementary: vec!["particle_engine".into(), "collision_engine".into()] },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_topology_count() {
        let entries = quantum_topology_lens_entries();
        assert_eq!(entries.len(), 285, "Must have 285 lenses (273 + 6 atlas + 6 collision)");
    }

    #[test]
    fn test_quantum_topology_names_unique() {
        let entries = quantum_topology_lens_entries();
        let mut names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        let total = names.len();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), total, "All quantum+topology lens names must be unique");
    }

    #[test]
    fn test_quantum_topology_all_extended() {
        let entries = quantum_topology_lens_entries();
        for e in &entries {
            assert_eq!(e.category, LensCategory::Extended);
        }
    }

    #[test]
    fn test_quantum_topology_non_empty() {
        let entries = quantum_topology_lens_entries();
        for e in &entries {
            assert!(!e.description.is_empty(), "Lens '{}' has empty description", e.name);
            assert!(!e.domain_affinity.is_empty(), "Lens '{}' has no domain affinity", e.name);
            assert!(e.complementary.len() >= 2, "Lens '{}' needs >=2 complementary", e.name);
        }
    }
}
