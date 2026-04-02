use super::registry::{LensCategory, LensEntry};

/// Build metadata entries for 63 accelerated-hypothesis humanities & advanced lenses (Part D).
///
/// Covers: urban/logistics, military/strategy, visual arts/architecture, literature/narrative,
/// philosophy/ontology, sports/kinesiology, agriculture/fermentation, astronomy, cryptography,
/// law/governance, geography/geology, perceptual psychology, game design, advanced mathematics.
pub fn accel_humanities_lens_entries() -> Vec<LensEntry> {
    vec![
        // ── Urban / Logistics (5) ──
        LensEntry {
            name: "traffic_flow".into(),
            category: LensCategory::Extended,
            description: "Analyze vehicular/pedestrian flow dynamics and congestion patterns".into(),
            domain_affinity: vec!["urban".into(), "logistics".into(), "network".into()],
            complementary: vec!["bottleneck".into(), "network".into()],
        },
        LensEntry {
            name: "urban_metabolism".into(),
            category: LensCategory::Extended,
            description: "Model cities as metabolic systems — energy/material in-out flows".into(),
            domain_affinity: vec!["urban".into(), "energy".into(), "environment".into()],
            complementary: vec!["thermo".into(), "carrying_capacity".into()],
        },
        LensEntry {
            name: "supply_chain_lens".into(),
            category: LensCategory::Extended,
            description: "Map multi-tier supply networks and identify fragility nodes".into(),
            domain_affinity: vec!["logistics".into(), "manufacturing".into(), "economics".into()],
            complementary: vec!["bottleneck".into(), "network".into()],
        },
        LensEntry {
            name: "vehicle_routing".into(),
            category: LensCategory::Extended,
            description: "Optimize multi-stop routing under capacity and time constraints".into(),
            domain_affinity: vec!["logistics".into(), "optimization".into(), "urban".into()],
            complementary: vec!["combinatorial".into(), "traffic_flow".into()],
        },
        LensEntry {
            name: "critical_path".into(),
            category: LensCategory::Extended,
            description: "Identify longest dependency chains constraining project completion".into(),
            domain_affinity: vec!["project_management".into(), "manufacturing".into(), "software".into()],
            complementary: vec!["bottleneck".into(), "hierarchy".into()],
        },

        // ── Military / Strategy (4) ──
        LensEntry {
            name: "strategic_depth".into(),
            category: LensCategory::Extended,
            description: "Evaluate layered defense/fallback capacity in adversarial settings".into(),
            domain_affinity: vec!["strategy".into(), "security".into(), "game_theory".into()],
            complementary: vec!["hierarchy".into(), "option_value".into()],
        },
        LensEntry {
            name: "maneuver_warfare".into(),
            category: LensCategory::Extended,
            description: "Detect flanking, envelopment, and positional advantage patterns".into(),
            domain_affinity: vec!["strategy".into(), "game_theory".into(), "robotics".into()],
            complementary: vec!["ooda_loop".into(), "force_multiplier".into()],
        },
        LensEntry {
            name: "ooda_loop".into(),
            category: LensCategory::Extended,
            description: "Measure observe-orient-decide-act cycle speed and disruption potential".into(),
            domain_affinity: vec!["strategy".into(), "ai".into(), "decision".into()],
            complementary: vec!["causal".into(), "maneuver_warfare".into()],
        },
        LensEntry {
            name: "force_multiplier".into(),
            category: LensCategory::Extended,
            description: "Identify factors that amplify effective capability beyond linear scaling".into(),
            domain_affinity: vec!["strategy".into(), "technology".into(), "economics".into()],
            complementary: vec!["scale".into(), "comparative_advantage".into()],
        },

        // ── Visual Arts / Architecture (6) ──
        LensEntry {
            name: "color_theory".into(),
            category: LensCategory::Extended,
            description: "Analyze chromatic harmony, contrast, and perceptual color relationships".into(),
            domain_affinity: vec!["design".into(), "display".into(), "art".into()],
            complementary: vec!["wave".into(), "composition_balance".into()],
        },
        LensEntry {
            name: "composition_balance".into(),
            category: LensCategory::Extended,
            description: "Evaluate visual weight distribution and spatial equilibrium in layouts".into(),
            domain_affinity: vec!["design".into(), "architecture".into(), "art".into()],
            complementary: vec!["mirror".into(), "golden_section".into()],
        },
        LensEntry {
            name: "visual_rhythm".into(),
            category: LensCategory::Extended,
            description: "Detect repetition, alternation, and progression in visual elements".into(),
            domain_affinity: vec!["design".into(), "architecture".into(), "art".into()],
            complementary: vec!["periodicity".into(), "wave".into()],
        },
        LensEntry {
            name: "negative_space".into(),
            category: LensCategory::Extended,
            description: "Analyze the structural role of empty regions in compositions".into(),
            domain_affinity: vec!["design".into(), "architecture".into(), "art".into()],
            complementary: vec!["void".into(), "composition_balance".into()],
        },
        LensEntry {
            name: "tensegrity".into(),
            category: LensCategory::Extended,
            description: "Detect tension-integrity structures where compression islands float in tension nets".into(),
            domain_affinity: vec!["architecture".into(), "biology".into(), "materials".into()],
            complementary: vec!["stability".into(), "network".into()],
        },
        LensEntry {
            name: "golden_section".into(),
            category: LensCategory::Extended,
            description: "Identify phi-ratio proportions and self-similar scaling in structure".into(),
            domain_affinity: vec!["design".into(), "architecture".into(), "mathematics".into(), "biology".into()],
            complementary: vec!["recursion".into(), "scale".into()],
        },

        // ── Literature / Narrative (4) ──
        LensEntry {
            name: "plot_structure".into(),
            category: LensCategory::Extended,
            description: "Map exposition-rising-climax-resolution arcs in sequential data".into(),
            domain_affinity: vec!["literature".into(), "narrative".into(), "ai".into()],
            complementary: vec!["narrative_tension_lens".into(), "character_arc".into()],
        },
        LensEntry {
            name: "character_arc".into(),
            category: LensCategory::Extended,
            description: "Track entity transformation trajectories over time".into(),
            domain_affinity: vec!["literature".into(), "narrative".into(), "social".into()],
            complementary: vec!["plot_structure".into(), "evolution".into()],
        },
        LensEntry {
            name: "narrative_tension_lens".into(),
            category: LensCategory::Extended,
            description: "Quantify suspense, conflict intensity, and resolution dynamics".into(),
            domain_affinity: vec!["literature".into(), "narrative".into(), "game_design".into()],
            complementary: vec!["plot_structure".into(), "wave".into()],
        },
        LensEntry {
            name: "intertextuality".into(),
            category: LensCategory::Extended,
            description: "Detect cross-references, allusions, and shared motifs between works".into(),
            domain_affinity: vec!["literature".into(), "culture".into(), "ai".into()],
            complementary: vec!["analogy".into(), "motif".into()],
        },

        // ── Philosophy / Ontology (4) ──
        LensEntry {
            name: "ontological_commitment".into(),
            category: LensCategory::Extended,
            description: "Identify implicit existence assumptions in models and theories".into(),
            domain_affinity: vec!["philosophy".into(), "ai".into(), "mathematics".into()],
            complementary: vec!["epistemological_frame".into(), "abstraction".into()],
        },
        LensEntry {
            name: "epistemological_frame".into(),
            category: LensCategory::Extended,
            description: "Analyze knowledge justification methods and evidential standards".into(),
            domain_affinity: vec!["philosophy".into(), "science".into(), "ai".into()],
            complementary: vec!["ontological_commitment".into(), "falsification".into()],
        },
        LensEntry {
            name: "phenomenological_reduction".into(),
            category: LensCategory::Extended,
            description: "Strip assumptions to expose raw experiential structure".into(),
            domain_affinity: vec!["philosophy".into(), "consciousness".into(), "psychology".into()],
            complementary: vec!["consciousness".into(), "abstraction".into()],
        },
        LensEntry {
            name: "dialectical_synthesis".into(),
            category: LensCategory::Extended,
            description: "Resolve thesis-antithesis tensions into higher-order synthesis".into(),
            domain_affinity: vec!["philosophy".into(), "social".into(), "strategy".into()],
            complementary: vec!["contradiction".into(), "duality".into()],
        },

        // ── Sports / Kinesiology (4) ──
        LensEntry {
            name: "biomechanics".into(),
            category: LensCategory::Extended,
            description: "Analyze force, torque, and kinematic chains in biological movement".into(),
            domain_affinity: vec!["sports".into(), "robotics".into(), "biology".into()],
            complementary: vec!["motor_learning".into(), "gravity".into()],
        },
        LensEntry {
            name: "motor_learning".into(),
            category: LensCategory::Extended,
            description: "Track skill acquisition curves and motor memory consolidation".into(),
            domain_affinity: vec!["sports".into(), "robotics".into(), "neuroscience".into()],
            complementary: vec!["biomechanics".into(), "memory".into()],
        },
        LensEntry {
            name: "periodization".into(),
            category: LensCategory::Extended,
            description: "Detect phased training/loading cycles for performance optimization".into(),
            domain_affinity: vec!["sports".into(), "training".into(), "biology".into()],
            complementary: vec!["wave".into(), "periodicity".into()],
        },
        LensEntry {
            name: "flow_performance".into(),
            category: LensCategory::Extended,
            description: "Identify optimal challenge-skill balance zones for peak performance".into(),
            domain_affinity: vec!["sports".into(), "psychology".into(), "game_design".into()],
            complementary: vec!["difficulty_curve".into(), "player_flow".into()],
        },

        // ── Agriculture / Fermentation (6) ──
        LensEntry {
            name: "crop_rotation".into(),
            category: LensCategory::Extended,
            description: "Analyze cyclical planting strategies for soil nutrient sustainability".into(),
            domain_affinity: vec!["agriculture".into(), "biology".into(), "sustainability".into()],
            complementary: vec!["periodicity".into(), "soil_microbiome".into()],
        },
        LensEntry {
            name: "soil_microbiome".into(),
            category: LensCategory::Extended,
            description: "Map microbial community structure and nutrient cycling networks".into(),
            domain_affinity: vec!["agriculture".into(), "biology".into(), "environment".into()],
            complementary: vec!["symbiosis".into(), "network".into()],
        },
        LensEntry {
            name: "photosynthetic_efficiency".into(),
            category: LensCategory::Extended,
            description: "Measure light-to-biomass conversion rates and quantum yield limits".into(),
            domain_affinity: vec!["agriculture".into(), "biology".into(), "energy".into()],
            complementary: vec!["thermo".into(), "quantum".into()],
        },
        LensEntry {
            name: "fermentation_kinetics".into(),
            category: LensCategory::Extended,
            description: "Model microbial metabolic rates, substrate depletion, and product yield".into(),
            domain_affinity: vec!["food_science".into(), "biology".into(), "chemistry".into()],
            complementary: vec!["catalysis".into(), "thermo".into()],
        },
        LensEntry {
            name: "maillard_reaction".into(),
            category: LensCategory::Extended,
            description: "Analyze non-enzymatic browning kinetics and flavor compound generation".into(),
            domain_affinity: vec!["food_science".into(), "chemistry".into(), "materials".into()],
            complementary: vec!["thermo".into(), "fermentation_kinetics".into()],
        },
        LensEntry {
            name: "flavor_pairing".into(),
            category: LensCategory::Extended,
            description: "Detect shared volatile compound profiles for ingredient compatibility".into(),
            domain_affinity: vec!["food_science".into(), "chemistry".into(), "network".into()],
            complementary: vec!["network".into(), "maillard_reaction".into()],
        },

        // ── Astronomy (4) ──
        LensEntry {
            name: "stellar_evolution_lens".into(),
            category: LensCategory::Extended,
            description: "Track stellar lifecycle phases — main sequence, giant, remnant transitions".into(),
            domain_affinity: vec!["astrophysics".into(), "cosmology".into(), "nuclear".into()],
            complementary: vec!["thermo".into(), "succession".into()],
        },
        LensEntry {
            name: "orbital_mechanics".into(),
            category: LensCategory::Extended,
            description: "Analyze Keplerian orbits, perturbations, and n-body resonances".into(),
            domain_affinity: vec!["astrophysics".into(), "space".into(), "physics".into()],
            complementary: vec!["gravity".into(), "periodicity".into()],
        },
        LensEntry {
            name: "accretion_disk".into(),
            category: LensCategory::Extended,
            description: "Model angular momentum transport and viscous disk dynamics".into(),
            domain_affinity: vec!["astrophysics".into(), "plasma".into(), "physics".into()],
            complementary: vec!["gravity".into(), "thermo".into()],
        },
        LensEntry {
            name: "pulsar_timing".into(),
            category: LensCategory::Extended,
            description: "Extract precision timing residuals for gravitational wave detection".into(),
            domain_affinity: vec!["astrophysics".into(), "signal".into(), "gravity".into()],
            complementary: vec!["periodicity".into(), "wave".into()],
        },

        // ── Cryptography (4) ──
        LensEntry {
            name: "zero_knowledge".into(),
            category: LensCategory::Extended,
            description: "Analyze proof systems that verify without revealing underlying data".into(),
            domain_affinity: vec!["cryptography".into(), "blockchain".into(), "privacy".into()],
            complementary: vec!["info".into(), "lattice_crypto".into()],
        },
        LensEntry {
            name: "homomorphic_compute".into(),
            category: LensCategory::Extended,
            description: "Detect computation-on-encrypted-data patterns and noise budgets".into(),
            domain_affinity: vec!["cryptography".into(), "ai".into(), "privacy".into()],
            complementary: vec!["zero_knowledge".into(), "lattice_crypto".into()],
        },
        LensEntry {
            name: "lattice_crypto".into(),
            category: LensCategory::Extended,
            description: "Analyze lattice-based hardness assumptions and post-quantum security".into(),
            domain_affinity: vec!["cryptography".into(), "quantum".into(), "mathematics".into()],
            complementary: vec!["homomorphic_compute".into(), "zero_knowledge".into()],
        },
        LensEntry {
            name: "hash_collision".into(),
            category: LensCategory::Extended,
            description: "Estimate collision resistance and birthday-bound thresholds".into(),
            domain_affinity: vec!["cryptography".into(), "security".into(), "blockchain".into()],
            complementary: vec!["info".into(), "kolmogorov".into()],
        },

        // ── Law / Governance (3) ──
        LensEntry {
            name: "precedent_analysis".into(),
            category: LensCategory::Extended,
            description: "Map legal precedent chains and case-law dependency graphs".into(),
            domain_affinity: vec!["law".into(), "governance".into(), "social".into()],
            complementary: vec!["hierarchy".into(), "causal".into()],
        },
        LensEntry {
            name: "regulatory_compliance".into(),
            category: LensCategory::Extended,
            description: "Check constraint satisfaction against regulatory rule sets".into(),
            domain_affinity: vec!["law".into(), "governance".into(), "manufacturing".into()],
            complementary: vec!["completeness".into(), "precedent_analysis".into()],
        },
        LensEntry {
            name: "game_theoretic_law".into(),
            category: LensCategory::Extended,
            description: "Model strategic incentive structures in legal/regulatory frameworks".into(),
            domain_affinity: vec!["law".into(), "economics".into(), "game_theory".into()],
            complementary: vec!["predation".into(), "option_value".into()],
        },

        // ── Geography / Geology (3) ──
        LensEntry {
            name: "terrain_analysis".into(),
            category: LensCategory::Extended,
            description: "Extract elevation gradients, slope stability, and landform classification".into(),
            domain_affinity: vec!["geology".into(), "geography".into(), "environment".into()],
            complementary: vec!["gravity".into(), "watershed_dynamics".into()],
        },
        LensEntry {
            name: "watershed_dynamics".into(),
            category: LensCategory::Extended,
            description: "Model hydrological flow accumulation and drainage basin partitioning".into(),
            domain_affinity: vec!["geology".into(), "environment".into(), "hydrology".into()],
            complementary: vec!["terrain_analysis".into(), "diffusion".into()],
        },
        LensEntry {
            name: "erosion_pattern".into(),
            category: LensCategory::Extended,
            description: "Detect material removal patterns from fluid-solid interaction over time".into(),
            domain_affinity: vec!["geology".into(), "materials".into(), "environment".into()],
            complementary: vec!["aging".into(), "diffusion".into()],
        },

        // ── Perceptual Psychology (3) ──
        LensEntry {
            name: "weber_fechner".into(),
            category: LensCategory::Extended,
            description: "Apply psychophysical logarithmic scaling to stimulus-response relationships".into(),
            domain_affinity: vec!["psychology".into(), "neuroscience".into(), "design".into()],
            complementary: vec!["scale".into(), "signal_detection_psy".into()],
        },
        LensEntry {
            name: "signal_detection_psy".into(),
            category: LensCategory::Extended,
            description: "Separate perceptual sensitivity (d') from response bias in detection tasks".into(),
            domain_affinity: vec!["psychology".into(), "neuroscience".into(), "ai".into()],
            complementary: vec!["weber_fechner".into(), "info".into()],
        },
        LensEntry {
            name: "perceptual_grouping".into(),
            category: LensCategory::Extended,
            description: "Detect Gestalt grouping principles — proximity, similarity, continuity, closure".into(),
            domain_affinity: vec!["psychology".into(), "design".into(), "ai".into()],
            complementary: vec!["topology".into(), "composition_balance".into()],
        },

        // ── Game Design (4) ──
        LensEntry {
            name: "reward_loop".into(),
            category: LensCategory::Extended,
            description: "Analyze reinforcement schedules and dopaminergic feedback cycles".into(),
            domain_affinity: vec!["game_design".into(), "ai".into(), "psychology".into()],
            complementary: vec!["wave".into(), "difficulty_curve".into()],
        },
        LensEntry {
            name: "difficulty_curve".into(),
            category: LensCategory::Extended,
            description: "Map challenge progression and skill-gate placement over time".into(),
            domain_affinity: vec!["game_design".into(), "education".into(), "ai".into()],
            complementary: vec!["reward_loop".into(), "flow_performance".into()],
        },
        LensEntry {
            name: "emergent_gameplay".into(),
            category: LensCategory::Extended,
            description: "Detect unplanned complex behaviors arising from simple rule interactions".into(),
            domain_affinity: vec!["game_design".into(), "ai".into(), "simulation".into()],
            complementary: vec!["emergence".into(), "combinatorial".into()],
        },
        LensEntry {
            name: "player_flow".into(),
            category: LensCategory::Extended,
            description: "Measure immersion-sustaining balance between anxiety and boredom zones".into(),
            domain_affinity: vec!["game_design".into(), "psychology".into(), "design".into()],
            complementary: vec!["flow_performance".into(), "reward_loop".into()],
        },

        // ── Advanced Mathematics (9) ──
        LensEntry {
            name: "lie_algebra_lens".into(),
            category: LensCategory::Extended,
            description: "Detect continuous symmetry generators and Lie bracket structures".into(),
            domain_affinity: vec!["mathematics".into(), "physics".into(), "robotics".into()],
            complementary: vec!["mirror".into(), "topology".into()],
        },
        LensEntry {
            name: "tropical_geometry".into(),
            category: LensCategory::Extended,
            description: "Apply min-plus algebra to detect piecewise-linear combinatorial structures".into(),
            domain_affinity: vec!["mathematics".into(), "optimization".into(), "biology".into()],
            complementary: vec!["convexity".into(), "combinatorial".into()],
        },
        LensEntry {
            name: "p_adic".into(),
            category: LensCategory::Extended,
            description: "Analyze ultrametric (p-adic) valuations and hierarchical distance structures".into(),
            domain_affinity: vec!["mathematics".into(), "physics".into(), "cryptography".into()],
            complementary: vec!["hierarchy".into(), "topology".into()],
        },
        LensEntry {
            name: "operadic".into(),
            category: LensCategory::Extended,
            description: "Detect multi-input compositional structures and operad algebras".into(),
            domain_affinity: vec!["mathematics".into(), "ai".into(), "physics".into()],
            complementary: vec!["recursion".into(), "hierarchy".into()],
        },
        LensEntry {
            name: "derived_category".into(),
            category: LensCategory::Extended,
            description: "Analyze chain-complex equivalences and homological invariants".into(),
            domain_affinity: vec!["mathematics".into(), "physics".into(), "topology".into()],
            complementary: vec!["topology".into(), "duality".into()],
        },
        LensEntry {
            name: "operator_algebra".into(),
            category: LensCategory::Extended,
            description: "Study C*-algebras and von Neumann algebras for quantum/functional structures".into(),
            domain_affinity: vec!["mathematics".into(), "quantum".into(), "physics".into()],
            complementary: vec!["spectral_theory_lens".into(), "quantum".into()],
        },
        LensEntry {
            name: "spectral_theory_lens".into(),
            category: LensCategory::Extended,
            description: "Analyze eigenvalue spectra of operators for stability and resonance".into(),
            domain_affinity: vec!["mathematics".into(), "physics".into(), "signal".into()],
            complementary: vec!["operator_algebra".into(), "spectral_gap".into()],
        },
        LensEntry {
            name: "ergodic_theory_lens".into(),
            category: LensCategory::Extended,
            description: "Study time-average vs space-average equivalence in dynamical systems".into(),
            domain_affinity: vec!["mathematics".into(), "physics".into(), "statistics".into()],
            complementary: vec!["broken_ergodicity".into(), "measure_theory_lens".into()],
        },
        LensEntry {
            name: "measure_theory_lens".into(),
            category: LensCategory::Extended,
            description: "Apply sigma-algebra and measure-theoretic foundations to integration and probability".into(),
            domain_affinity: vec!["mathematics".into(), "statistics".into(), "physics".into()],
            complementary: vec!["ergodic_theory_lens".into(), "info".into()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_accel_humanities_lens_count() {
        let entries = accel_humanities_lens_entries();
        assert_eq!(entries.len(), 63, "Must have exactly 63 accel-humanities lenses");
    }

    #[test]
    fn test_accel_humanities_lens_names_unique() {
        let entries = accel_humanities_lens_entries();
        let names: HashSet<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names.len(), entries.len(), "All accel-humanities lens names must be unique");
    }

    #[test]
    fn test_accel_humanities_all_extended() {
        for entry in accel_humanities_lens_entries() {
            assert_eq!(
                entry.category,
                LensCategory::Extended,
                "Lens '{}' should be Extended category",
                entry.name
            );
        }
    }

    #[test]
    fn test_accel_humanities_no_empty_fields() {
        for entry in accel_humanities_lens_entries() {
            assert!(!entry.name.is_empty(), "Lens name must not be empty");
            assert!(!entry.description.is_empty(), "Lens '{}' description must not be empty", entry.name);
            assert!(!entry.domain_affinity.is_empty(), "Lens '{}' must have domain affinity", entry.name);
            assert!(!entry.complementary.is_empty(), "Lens '{}' must have complementary lenses", entry.name);
        }
    }
}
