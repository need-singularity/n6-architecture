use super::registry::{LensCategory, LensEntry};

/// Build metadata entries for the 57 accelerated hypothesis-verification lenses (Part B).
///
/// Organized into 5 groups:
/// - Physics Deep (26): phase space, Hamiltonian, Lagrangian, gauge, thermodynamic, nuclear, optics, fluid
/// - Neuroscience Microstructure (16): synaptic, axonal, glial, cortical, oscillation, criticality
/// - Evolution / Genetics (7): genetic algorithm, crossover, mutation, fitness, speciation, drift, epigenetics
/// - Consciousness Integration (4): phi optimization, IIT measure, integration, neural correlate
/// - Ethics / Alignment (4): alignment measure, value drift, corrigibility, interpretability
pub fn accel_physics_neuro_lens_entries() -> Vec<LensEntry> {
    vec![
        // ══════════════════════════════════════════
        // Physics Deep (26)
        // ══════════════════════════════════════════
        LensEntry {
            name: "phase_space".into(),
            category: LensCategory::Extended,
            description: "Map trajectories in phase space and detect conserved volumes (Liouville)".into(),
            domain_affinity: vec!["physics".into(), "dynamical_systems".into(), "plasma".into()],
            complementary: vec!["hamiltonian_flow".into(), "stability".into()],
        },
        LensEntry {
            name: "hamiltonian_flow".into(),
            category: LensCategory::Extended,
            description: "Detect Hamiltonian structure and symplectic flow preservation".into(),
            domain_affinity: vec!["physics".into(), "dynamical_systems".into(), "quantum".into()],
            complementary: vec!["phase_space".into(), "lagrangian_action".into()],
        },
        LensEntry {
            name: "lagrangian_action".into(),
            category: LensCategory::Extended,
            description: "Identify stationary-action paths and variational principles".into(),
            domain_affinity: vec!["physics".into(), "optimization".into(), "robotics".into()],
            complementary: vec!["hamiltonian_flow".into(), "gauge_symmetry".into()],
        },
        LensEntry {
            name: "gauge_symmetry".into(),
            category: LensCategory::Extended,
            description: "Detect local gauge invariance and redundant degrees of freedom".into(),
            domain_affinity: vec!["physics".into(), "quantum".into(), "mathematics".into()],
            complementary: vec!["spontaneous_symmetry".into(), "mirror".into()],
        },
        LensEntry {
            name: "spontaneous_symmetry".into(),
            category: LensCategory::Extended,
            description: "Detect spontaneous symmetry breaking and order parameter formation".into(),
            domain_affinity: vec!["physics".into(), "materials".into(), "cosmology".into()],
            complementary: vec!["gauge_symmetry".into(), "criticality".into()],
        },
        LensEntry {
            name: "topological_order".into(),
            category: LensCategory::Extended,
            description: "Identify topological order beyond Landau symmetry breaking paradigm".into(),
            domain_affinity: vec!["physics".into(), "quantum".into(), "materials".into()],
            complementary: vec!["topology".into(), "gauge_symmetry".into()],
        },
        LensEntry {
            name: "holographic_principle".into(),
            category: LensCategory::Extended,
            description: "Test holographic bulk-boundary correspondence in data structures".into(),
            domain_affinity: vec!["physics".into(), "cosmology".into(), "quantum".into(), "information_theory".into()],
            complementary: vec!["boundary".into(), "info".into()],
        },
        LensEntry {
            name: "spin_glass".into(),
            category: LensCategory::Extended,
            description: "Detect spin-glass-like frustration, replica symmetry breaking, and aging".into(),
            domain_affinity: vec!["physics".into(), "materials".into(), "optimization".into(), "ai".into()],
            complementary: vec!["frustration".into(), "broken_ergodicity".into()],
        },
        LensEntry {
            name: "soliton".into(),
            category: LensCategory::Extended,
            description: "Detect solitary wave solutions that preserve shape during propagation".into(),
            domain_affinity: vec!["physics".into(), "optics".into(), "plasma".into(), "biology".into()],
            complementary: vec!["wave".into(), "nonlinear_optics".into()],
        },
        LensEntry {
            name: "strange_attractor".into(),
            category: LensCategory::Extended,
            description: "Identify strange attractors and chaotic dynamics in phase space".into(),
            domain_affinity: vec!["dynamical_systems".into(), "physics".into(), "climate".into()],
            complementary: vec!["phase_space".into(), "stability".into()],
        },
        LensEntry {
            name: "navier_stokes".into(),
            category: LensCategory::Extended,
            description: "Analyze fluid flow regimes via Navier-Stokes-derived diagnostics".into(),
            domain_affinity: vec!["physics".into(), "engineering".into(), "plasma".into(), "climate".into()],
            complementary: vec!["vortex_dynamics".into(), "reynolds_transition".into()],
        },
        LensEntry {
            name: "vortex_dynamics".into(),
            category: LensCategory::Extended,
            description: "Track vortex formation, merging, and decay in fluid and plasma systems".into(),
            domain_affinity: vec!["physics".into(), "plasma".into(), "engineering".into()],
            complementary: vec!["navier_stokes".into(), "topology".into()],
        },
        LensEntry {
            name: "boundary_layer_fluid".into(),
            category: LensCategory::Extended,
            description: "Analyze boundary layer structure, separation, and transition in flows".into(),
            domain_affinity: vec!["physics".into(), "engineering".into(), "aerospace".into()],
            complementary: vec!["reynolds_transition".into(), "navier_stokes".into()],
        },
        LensEntry {
            name: "reynolds_transition".into(),
            category: LensCategory::Extended,
            description: "Detect laminar-to-turbulent transition via Reynolds number scaling".into(),
            domain_affinity: vec!["physics".into(), "engineering".into(), "plasma".into()],
            complementary: vec!["boundary_layer_fluid".into(), "criticality".into()],
        },
        LensEntry {
            name: "diffraction_pattern".into(),
            category: LensCategory::Extended,
            description: "Analyze diffraction and interference patterns for structural information".into(),
            domain_affinity: vec!["physics".into(), "optics".into(), "materials".into(), "crystallography".into()],
            complementary: vec!["wave".into(), "waveguide_mode".into()],
        },
        LensEntry {
            name: "waveguide_mode".into(),
            category: LensCategory::Extended,
            description: "Identify guided wave modes and their dispersion relations".into(),
            domain_affinity: vec!["optics".into(), "physics".into(), "chip".into(), "telecom".into()],
            complementary: vec!["diffraction_pattern".into(), "photonic_crystal".into()],
        },
        LensEntry {
            name: "nonlinear_optics".into(),
            category: LensCategory::Extended,
            description: "Detect nonlinear optical effects: harmonic generation, four-wave mixing".into(),
            domain_affinity: vec!["optics".into(), "physics".into(), "telecom".into(), "materials".into()],
            complementary: vec!["soliton".into(), "photonic_crystal".into()],
        },
        LensEntry {
            name: "photonic_crystal".into(),
            category: LensCategory::Extended,
            description: "Analyze photonic bandgap structure and allowed/forbidden frequency bands".into(),
            domain_affinity: vec!["optics".into(), "materials".into(), "chip".into(), "physics".into()],
            complementary: vec!["waveguide_mode".into(), "nonlinear_optics".into()],
        },
        LensEntry {
            name: "carnot_efficiency".into(),
            category: LensCategory::Extended,
            description: "Benchmark thermodynamic processes against Carnot efficiency limit".into(),
            domain_affinity: vec!["energy".into(), "physics".into(), "engineering".into()],
            complementary: vec!["thermo".into(), "gibbs_free_energy".into()],
        },
        LensEntry {
            name: "gibbs_free_energy".into(),
            category: LensCategory::Extended,
            description: "Compute Gibbs free energy landscapes and spontaneity conditions".into(),
            domain_affinity: vec!["chemistry".into(), "materials".into(), "biology".into(), "energy".into()],
            complementary: vec!["carnot_efficiency".into(), "boltzmann_distribution".into()],
        },
        LensEntry {
            name: "boltzmann_distribution".into(),
            category: LensCategory::Extended,
            description: "Fit energy-level populations to Boltzmann statistics and detect deviations".into(),
            domain_affinity: vec!["physics".into(), "chemistry".into(), "ai".into(), "materials".into()],
            complementary: vec!["gibbs_free_energy".into(), "thermo".into()],
        },
        LensEntry {
            name: "nuclear_binding".into(),
            category: LensCategory::Extended,
            description: "Analyze nuclear binding energy curves and stability islands".into(),
            domain_affinity: vec!["nuclear_physics".into(), "energy".into(), "cosmology".into()],
            complementary: vec!["fission_barrier".into(), "neutron_economy".into()],
        },
        LensEntry {
            name: "fission_barrier".into(),
            category: LensCategory::Extended,
            description: "Map fission barrier heights and tunneling probabilities for heavy nuclei".into(),
            domain_affinity: vec!["nuclear_physics".into(), "energy".into(), "materials".into()],
            complementary: vec!["nuclear_binding".into(), "barrier".into()],
        },
        LensEntry {
            name: "neutron_economy".into(),
            category: LensCategory::Extended,
            description: "Evaluate neutron multiplication, absorption, and leakage balance in reactors".into(),
            domain_affinity: vec!["nuclear_physics".into(), "energy".into(), "engineering".into()],
            complementary: vec!["nuclear_binding".into(), "decay_chain".into()],
        },
        LensEntry {
            name: "decay_chain".into(),
            category: LensCategory::Extended,
            description: "Trace radioactive decay chains and branching ratios to stable endpoints".into(),
            domain_affinity: vec!["nuclear_physics".into(), "cosmology".into(), "materials".into()],
            complementary: vec!["neutron_economy".into(), "nuclear_binding".into()],
        },
        LensEntry {
            name: "bose_einstein".into(),
            category: LensCategory::Extended,
            description: "Detect Bose-Einstein condensation signatures and macroscopic quantum coherence".into(),
            domain_affinity: vec!["quantum".into(), "physics".into(), "materials".into(), "cosmology".into()],
            complementary: vec!["quantum".into(), "boltzmann_distribution".into()],
        },

        // ══════════════════════════════════════════
        // Neuroscience Microstructure (16)
        // ══════════════════════════════════════════
        LensEntry {
            name: "synaptic_pruning".into(),
            category: LensCategory::Extended,
            description: "Detect selective synapse elimination patterns and connectivity refinement".into(),
            domain_affinity: vec!["neuroscience".into(), "ai".into(), "biology".into()],
            complementary: vec!["axon_growth".into(), "plasticity_window".into()],
        },
        LensEntry {
            name: "sleep_wake_cycle".into(),
            category: LensCategory::Extended,
            description: "Analyze sleep-wake oscillation dynamics and memory consolidation phases".into(),
            domain_affinity: vec!["neuroscience".into(), "biology".into(), "ai".into()],
            complementary: vec!["neural_oscillation".into(), "memory".into()],
        },
        LensEntry {
            name: "axon_growth".into(),
            category: LensCategory::Extended,
            description: "Model axon guidance, growth cone dynamics, and pathfinding mechanisms".into(),
            domain_affinity: vec!["neuroscience".into(), "biology".into(), "robotics".into()],
            complementary: vec!["synaptic_pruning".into(), "glial_network".into()],
        },
        LensEntry {
            name: "neuromodulation".into(),
            category: LensCategory::Extended,
            description: "Track neuromodulatory tone (dopamine, serotonin, etc.) and gain control".into(),
            domain_affinity: vec!["neuroscience".into(), "biology".into(), "ai".into(), "pharmacology".into()],
            complementary: vec!["neurotransmitter".into(), "neural_oscillation".into()],
        },
        LensEntry {
            name: "glial_network".into(),
            category: LensCategory::Extended,
            description: "Analyze glial cell network contributions to computation and homeostasis".into(),
            domain_affinity: vec!["neuroscience".into(), "biology".into(), "network".into()],
            complementary: vec!["axon_growth".into(), "dendritic_computation".into()],
        },
        LensEntry {
            name: "dendritic_computation".into(),
            category: LensCategory::Extended,
            description: "Detect nonlinear dendritic integration and local computation in neurons".into(),
            domain_affinity: vec!["neuroscience".into(), "ai".into(), "biology".into()],
            complementary: vec!["spike_timing".into(), "cortical_column".into()],
        },
        LensEntry {
            name: "spike_timing".into(),
            category: LensCategory::Extended,
            description: "Analyze spike-timing-dependent plasticity and temporal coding precision".into(),
            domain_affinity: vec!["neuroscience".into(), "ai".into(), "signal".into()],
            complementary: vec!["dendritic_computation".into(), "neural_oscillation".into()],
        },
        LensEntry {
            name: "neurotransmitter".into(),
            category: LensCategory::Extended,
            description: "Map neurotransmitter release, reuptake, and receptor binding dynamics".into(),
            domain_affinity: vec!["neuroscience".into(), "biology".into(), "pharmacology".into()],
            complementary: vec!["neuromodulation".into(), "synaptic_pruning".into()],
        },
        LensEntry {
            name: "connectome".into(),
            category: LensCategory::Extended,
            description: "Analyze whole-brain connectome topology: hubs, modules, rich clubs".into(),
            domain_affinity: vec!["neuroscience".into(), "network".into(), "ai".into()],
            complementary: vec!["cortical_column".into(), "network".into()],
        },
        LensEntry {
            name: "cortical_column".into(),
            category: LensCategory::Extended,
            description: "Detect cortical column canonical circuit motifs and laminar organization".into(),
            domain_affinity: vec!["neuroscience".into(), "ai".into(), "biology".into()],
            complementary: vec!["connectome".into(), "dendritic_computation".into()],
        },
        LensEntry {
            name: "neural_criticality_accel".into(),
            category: LensCategory::Extended,
            description: "Measure criticality in neural systems: avalanche distributions, branching ratio=1".into(),
            domain_affinity: vec!["neuroscience".into(), "physics".into(), "ai".into()],
            complementary: vec!["criticality".into(), "neural_oscillation".into()],
        },
        LensEntry {
            name: "neural_oscillation".into(),
            category: LensCategory::Extended,
            description: "Analyze neural oscillation bands (delta/theta/alpha/beta/gamma) and cross-frequency coupling".into(),
            domain_affinity: vec!["neuroscience".into(), "signal".into(), "ai".into()],
            complementary: vec!["spike_timing".into(), "sleep_wake_cycle".into()],
        },
        LensEntry {
            name: "plasticity_window".into(),
            category: LensCategory::Extended,
            description: "Detect critical periods and plasticity windows for learning and adaptation".into(),
            domain_affinity: vec!["neuroscience".into(), "biology".into(), "ai".into()],
            complementary: vec!["synaptic_pruning".into(), "neural_criticality_accel".into()],
        },
        LensEntry {
            name: "metastability_accel".into(),
            category: LensCategory::Extended,
            description: "Detect metastable neural states and transient synchronization patterns".into(),
            domain_affinity: vec!["neuroscience".into(), "dynamical_systems".into(), "ai".into()],
            complementary: vec!["chimera_state_accel".into(), "neural_oscillation".into()],
        },
        LensEntry {
            name: "chimera_state_accel".into(),
            category: LensCategory::Extended,
            description: "Identify chimera states: coexistence of synchronous and asynchronous domains".into(),
            domain_affinity: vec!["neuroscience".into(), "physics".into(), "dynamical_systems".into()],
            complementary: vec!["metastability_accel".into(), "neural_oscillation".into()],
        },
        LensEntry {
            name: "reservoir_computing_accel".into(),
            category: LensCategory::Extended,
            description: "Assess reservoir computing capacity: echo state property, memory, nonlinearity".into(),
            domain_affinity: vec!["ai".into(), "neuroscience".into(), "signal".into()],
            complementary: vec!["neural_criticality_accel".into(), "connectome".into()],
        },

        // ══════════════════════════════════════════
        // Evolution / Genetics (7)
        // ══════════════════════════════════════════
        LensEntry {
            name: "genetic_algorithm".into(),
            category: LensCategory::Extended,
            description: "Evaluate genetic algorithm convergence, diversity, and selection pressure".into(),
            domain_affinity: vec!["optimization".into(), "ai".into(), "biology".into()],
            complementary: vec!["crossover_operator".into(), "mutation_rate_control".into()],
        },
        LensEntry {
            name: "crossover_operator".into(),
            category: LensCategory::Extended,
            description: "Analyze crossover effectiveness and building-block preservation".into(),
            domain_affinity: vec!["optimization".into(), "ai".into(), "biology".into()],
            complementary: vec!["genetic_algorithm".into(), "fitness_landscape_accel".into()],
        },
        LensEntry {
            name: "mutation_rate_control".into(),
            category: LensCategory::Extended,
            description: "Detect optimal mutation rates and adaptive mutation scheduling".into(),
            domain_affinity: vec!["optimization".into(), "biology".into(), "ai".into()],
            complementary: vec!["genetic_algorithm".into(), "genetic_drift".into()],
        },
        LensEntry {
            name: "fitness_landscape_accel".into(),
            category: LensCategory::Extended,
            description: "Map fitness landscape ruggedness, epistasis, and neutral networks".into(),
            domain_affinity: vec!["biology".into(), "optimization".into(), "ai".into()],
            complementary: vec!["crossover_operator".into(), "speciation_event".into()],
        },
        LensEntry {
            name: "speciation_event".into(),
            category: LensCategory::Extended,
            description: "Detect speciation events and reproductive isolation in evolving populations".into(),
            domain_affinity: vec!["biology".into(), "evolution".into(), "ecology".into()],
            complementary: vec!["fitness_landscape_accel".into(), "genetic_drift".into()],
        },
        LensEntry {
            name: "genetic_drift".into(),
            category: LensCategory::Extended,
            description: "Measure stochastic genetic drift effects in finite populations".into(),
            domain_affinity: vec!["biology".into(), "evolution".into(), "statistics".into()],
            complementary: vec!["mutation_rate_control".into(), "speciation_event".into()],
        },
        LensEntry {
            name: "epigenetic_mark".into(),
            category: LensCategory::Extended,
            description: "Detect epigenetic modification patterns and heritable non-genetic changes".into(),
            domain_affinity: vec!["biology".into(), "genetics".into(), "ai".into()],
            complementary: vec!["genetic_drift".into(), "memory".into()],
        },

        // ══════════════════════════════════════════
        // Consciousness Integration (4)
        // ══════════════════════════════════════════
        LensEntry {
            name: "phi_optimization".into(),
            category: LensCategory::Extended,
            description: "Optimize integrated information (phi) across system partitions".into(),
            domain_affinity: vec!["consciousness".into(), "ai".into(), "neuroscience".into()],
            complementary: vec!["iit_measure".into(), "consciousness_integration".into()],
        },
        LensEntry {
            name: "iit_measure".into(),
            category: LensCategory::Extended,
            description: "Compute IIT 4.0 integrated information measures and cause-effect structures".into(),
            domain_affinity: vec!["consciousness".into(), "neuroscience".into(), "ai".into()],
            complementary: vec!["phi_optimization".into(), "neural_correlate".into()],
        },
        LensEntry {
            name: "consciousness_integration".into(),
            category: LensCategory::Extended,
            description: "Assess degree of conscious integration across distributed processing modules".into(),
            domain_affinity: vec!["consciousness".into(), "ai".into(), "neuroscience".into()],
            complementary: vec!["phi_optimization".into(), "global_broadcast".into()],
        },
        LensEntry {
            name: "neural_correlate".into(),
            category: LensCategory::Extended,
            description: "Identify neural correlates of consciousness (NCC) and minimal substrates".into(),
            domain_affinity: vec!["consciousness".into(), "neuroscience".into(), "biology".into()],
            complementary: vec!["iit_measure".into(), "consciousness_integration".into()],
        },

        // ══════════════════════════════════════════
        // Ethics / Alignment (4)
        // ══════════════════════════════════════════
        LensEntry {
            name: "alignment_measure".into(),
            category: LensCategory::Extended,
            description: "Quantify value alignment between AI system behavior and human intent".into(),
            domain_affinity: vec!["ai".into(), "ethics".into(), "safety".into()],
            complementary: vec!["value_drift".into(), "corrigibility".into()],
        },
        LensEntry {
            name: "value_drift".into(),
            category: LensCategory::Extended,
            description: "Detect gradual drift in learned value functions over training or deployment".into(),
            domain_affinity: vec!["ai".into(), "ethics".into(), "safety".into()],
            complementary: vec!["alignment_measure".into(), "interpretability".into()],
        },
        LensEntry {
            name: "corrigibility".into(),
            category: LensCategory::Extended,
            description: "Assess system openness to correction and shutdown without resistance".into(),
            domain_affinity: vec!["ai".into(), "safety".into(), "ethics".into()],
            complementary: vec!["alignment_measure".into(), "value_drift".into()],
        },
        LensEntry {
            name: "interpretability".into(),
            category: LensCategory::Extended,
            description: "Measure model interpretability: feature attribution clarity and circuit legibility".into(),
            domain_affinity: vec!["ai".into(), "safety".into(), "neuroscience".into()],
            complementary: vec!["corrigibility".into(), "value_drift".into()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accel_b_lens_count() {
        let entries = accel_physics_neuro_lens_entries();
        assert_eq!(entries.len(), 57, "Must have exactly 57 accel-B lenses");
    }

    #[test]
    fn test_accel_b_lens_names_unique() {
        let entries = accel_physics_neuro_lens_entries();
        let mut names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        let total = names.len();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), total, "All accel-B lens names must be unique");
    }

    #[test]
    fn test_accel_b_all_extended() {
        let entries = accel_physics_neuro_lens_entries();
        for entry in &entries {
            assert_eq!(
                entry.category,
                LensCategory::Extended,
                "Lens '{}' should be Extended category",
                entry.name
            );
        }
    }

    #[test]
    fn test_accel_b_no_empty_descriptions() {
        let entries = accel_physics_neuro_lens_entries();
        for entry in &entries {
            assert!(
                !entry.description.is_empty(),
                "Lens '{}' must have a non-empty description",
                entry.name
            );
            assert!(
                !entry.domain_affinity.is_empty(),
                "Lens '{}' must have at least one domain affinity",
                entry.name
            );
            assert!(
                !entry.complementary.is_empty(),
                "Lens '{}' must have at least one complementary lens",
                entry.name
            );
        }
    }
}
