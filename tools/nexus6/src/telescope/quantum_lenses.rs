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
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_topology_count() {
        let entries = quantum_topology_lens_entries();
        assert_eq!(entries.len(), 38, "Must have exactly 38 quantum+topology lenses (30+8)");
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
