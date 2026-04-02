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
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_topology_count() {
        let entries = quantum_topology_lens_entries();
        assert_eq!(entries.len(), 145, "Must have 145 lenses (125 + 5 fast + 5 range + 6 evolved + 4 weight)");
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
