use super::registry::{LensCategory, LensEntry};

/// Build metadata entries for the 58 n6-industry discovery lenses.
///
/// These lenses emerged from industrial pattern discovery across 32+ domains
/// in the n6-architecture project. Organized by functional group (II–XVII).
pub fn n6_industry_lens_entries() -> Vec<LensEntry> {
    vec![
        // ── II. Exploration (3) ──
        LensEntry {
            name: "void".into(),
            category: LensCategory::Extended,
            description: "Detect empty regions and structural gaps in data landscapes".into(),
            domain_affinity: vec!["cosmology".into(), "materials".into(), "network".into()],
            complementary: vec!["boundary".into(), "topology".into()],
        },
        LensEntry {
            name: "isomorphism".into(),
            category: LensCategory::Extended,
            description: "Find structural equivalences between seemingly different systems".into(),
            domain_affinity: vec!["mathematics".into(), "software".into(), "physics".into()],
            complementary: vec!["mirror".into(), "analogy".into()],
        },
        LensEntry {
            name: "extrapolation".into(),
            category: LensCategory::Extended,
            description: "Project observed trends beyond measured boundaries".into(),
            domain_affinity: vec!["ai".into(), "finance".into(), "cosmology".into()],
            complementary: vec!["scale".into(), "causal".into()],
        },
        // ── III. Synthesis (3) ──
        LensEntry {
            name: "inverse".into(),
            category: LensCategory::Extended,
            description: "Reverse-engineer causes from observed effects".into(),
            domain_affinity: vec!["physics".into(), "ai".into(), "materials".into()],
            complementary: vec!["causal".into(), "info".into()],
        },
        LensEntry {
            name: "combinatorial".into(),
            category: LensCategory::Extended,
            description: "Enumerate and score discrete design-space combinations".into(),
            domain_affinity: vec!["chip".into(), "materials".into(), "software".into(), "chemistry".into()],
            complementary: vec!["completeness".into(), "diminishing_returns".into()],
        },
        LensEntry {
            name: "frustration".into(),
            category: LensCategory::Extended,
            description: "Detect competing constraints that prevent global optimum".into(),
            domain_affinity: vec!["materials".into(), "physics".into(), "optimization".into()],
            complementary: vec!["stability".into(), "saddle".into()],
        },
        // ── IV. Verification (3) ──
        LensEntry {
            name: "emergence".into(),
            category: LensCategory::Extended,
            description: "Identify macro-patterns arising from micro-level interactions".into(),
            domain_affinity: vec!["biology".into(), "ai".into(), "physics".into(), "social".into()],
            complementary: vec!["consciousness".into(), "multiscale".into()],
        },
        LensEntry {
            name: "periodicity".into(),
            category: LensCategory::Extended,
            description: "Detect repeating cycles and periodic structures".into(),
            domain_affinity: vec!["signal".into(), "chemistry".into(), "cosmology".into(), "audio".into()],
            complementary: vec!["wave".into(), "memory".into()],
        },
        LensEntry {
            name: "completeness".into(),
            category: LensCategory::Extended,
            description: "Check whether a set of components covers the full space".into(),
            domain_affinity: vec!["mathematics".into(), "software".into(), "chip".into()],
            complementary: vec!["combinatorial".into(), "blind_spot".into()],
        },
        // ── V. Quality (3) ──
        LensEntry {
            name: "surprise".into(),
            category: LensCategory::Extended,
            description: "Quantify deviation from expectation — anomaly significance".into(),
            domain_affinity: vec!["ai".into(), "finance".into(), "biology".into()],
            complementary: vec!["info".into(), "falsification".into()],
        },
        LensEntry {
            name: "falsification".into(),
            category: LensCategory::Extended,
            description: "Attempt to disprove hypotheses via counter-evidence search".into(),
            domain_affinity: vec!["science".into(), "ai".into(), "mathematics".into()],
            complementary: vec!["surprise".into(), "contradiction".into()],
        },
        LensEntry {
            name: "duality".into(),
            category: LensCategory::Extended,
            description: "Find dual representations that simplify the problem".into(),
            domain_affinity: vec!["mathematics".into(), "physics".into(), "optimization".into()],
            complementary: vec!["mirror".into(), "isomorphism".into()],
        },
        // ── VI. Materials-specific (3) ──
        LensEntry {
            name: "defect".into(),
            category: LensCategory::Extended,
            description: "Locate point defects, vacancies, and structural imperfections".into(),
            domain_affinity: vec!["materials".into(), "chip".into(), "battery".into()],
            complementary: vec!["boundary".into(), "interface".into()],
        },
        LensEntry {
            name: "interface".into(),
            category: LensCategory::Extended,
            description: "Analyze boundaries between distinct material phases".into(),
            domain_affinity: vec!["materials".into(), "battery".into(), "chip".into(), "biology".into()],
            complementary: vec!["defect".into(), "boundary".into()],
        },
        LensEntry {
            name: "catalysis".into(),
            category: LensCategory::Extended,
            description: "Identify rate-enhancing pathways and activation energy lowering".into(),
            domain_affinity: vec!["chemistry".into(), "materials".into(), "energy".into(), "biology".into()],
            complementary: vec!["thermo".into(), "barrier".into()],
        },
        // ── VII. Dynamics (5) ──
        LensEntry {
            name: "tipping".into(),
            category: LensCategory::Extended,
            description: "Detect proximity to tipping points and regime shifts".into(),
            domain_affinity: vec!["climate".into(), "biology".into(), "finance".into(), "energy".into()],
            complementary: vec!["criticality".into(), "stability".into()],
        },
        LensEntry {
            name: "coevolution".into(),
            category: LensCategory::Extended,
            description: "Track coupled evolutionary dynamics between interacting systems".into(),
            domain_affinity: vec!["biology".into(), "ai".into(), "economics".into()],
            complementary: vec!["evolution".into(), "symbiosis".into()],
        },
        LensEntry {
            name: "percolation".into(),
            category: LensCategory::Extended,
            description: "Measure connectivity thresholds and percolation transitions".into(),
            domain_affinity: vec!["network".into(), "materials".into(), "physics".into()],
            complementary: vec!["network".into(), "criticality".into()],
        },
        LensEntry {
            name: "hysteresis".into(),
            category: LensCategory::Extended,
            description: "Detect path-dependent history effects and irreversibility".into(),
            domain_affinity: vec!["materials".into(), "physics".into(), "economics".into()],
            complementary: vec!["memory".into(), "barrier".into()],
        },
        LensEntry {
            name: "diffusion".into(),
            category: LensCategory::Extended,
            description: "Model spread dynamics — Fickian and anomalous transport".into(),
            domain_affinity: vec!["materials".into(), "biology".into(), "physics".into(), "network".into()],
            complementary: vec!["anomalous_diffusion".into(), "thermo".into()],
        },
        // ── VIII. Meta-structure (4) ──
        LensEntry {
            name: "hierarchy".into(),
            category: LensCategory::Extended,
            description: "Detect nested hierarchical organization and tree structures".into(),
            domain_affinity: vec!["biology".into(), "software".into(), "network".into(), "social".into()],
            complementary: vec!["recursion".into(), "multiscale".into()],
        },
        LensEntry {
            name: "conservation".into(),
            category: LensCategory::Extended,
            description: "Find conserved quantities and invariants under transformation".into(),
            domain_affinity: vec!["physics".into(), "chemistry".into(), "mathematics".into()],
            complementary: vec!["mirror".into(), "thermo".into()],
        },
        LensEntry {
            name: "arbitrage".into(),
            category: LensCategory::Extended,
            description: "Detect exploitable imbalances and free-energy gradients".into(),
            domain_affinity: vec!["finance".into(), "energy".into(), "optimization".into()],
            complementary: vec!["bottleneck".into(), "comparative_advantage".into()],
        },
        LensEntry {
            name: "serendipity".into(),
            category: LensCategory::Extended,
            description: "Flag unexpected co-occurrences that may indicate hidden links".into(),
            domain_affinity: vec!["science".into(), "materials".into(), "ai".into()],
            complementary: vec!["surprise".into(), "analogy".into()],
        },
        // ── IX. Transition (5) ──
        LensEntry {
            name: "renormalization".into(),
            category: LensCategory::Extended,
            description: "Apply scale-elimination to extract effective parameters".into(),
            domain_affinity: vec!["physics".into(), "ai".into(), "materials".into()],
            complementary: vec!["multiscale".into(), "universality_class".into()],
        },
        LensEntry {
            name: "saddle".into(),
            category: LensCategory::Extended,
            description: "Locate saddle points and transition states in energy landscapes".into(),
            domain_affinity: vec!["chemistry".into(), "materials".into(), "optimization".into()],
            complementary: vec!["barrier".into(), "frustration".into()],
        },
        LensEntry {
            name: "criticality".into(),
            category: LensCategory::Extended,
            description: "Detect critical phenomena and diverging correlation lengths".into(),
            domain_affinity: vec!["physics".into(), "biology".into(), "network".into()],
            complementary: vec!["tipping".into(), "universality_class".into()],
        },
        LensEntry {
            name: "succession".into(),
            category: LensCategory::Extended,
            description: "Track sequential replacement dynamics in evolving systems".into(),
            domain_affinity: vec!["biology".into(), "materials".into(), "economics".into()],
            complementary: vec!["evolution".into(), "coevolution".into()],
        },
        LensEntry {
            name: "resonance_cascade".into(),
            category: LensCategory::Extended,
            description: "Detect cascading resonance amplification across coupled modes".into(),
            domain_affinity: vec!["physics".into(), "plasma".into(), "audio".into(), "energy".into()],
            complementary: vec!["wave".into(), "criticality".into()],
        },
        // ── X. Information-deep (4) ──
        LensEntry {
            name: "fisher_info".into(),
            category: LensCategory::Extended,
            description: "Measure Fisher information — sensitivity of distributions to parameters".into(),
            domain_affinity: vec!["statistics".into(), "ai".into(), "physics".into()],
            complementary: vec!["info".into(), "kolmogorov".into()],
        },
        LensEntry {
            name: "spectral_gap".into(),
            category: LensCategory::Extended,
            description: "Analyze eigenvalue gaps indicating mixing times and phase structure".into(),
            domain_affinity: vec!["physics".into(), "network".into(), "quantum".into()],
            complementary: vec!["stability".into(), "criticality".into()],
        },
        LensEntry {
            name: "kolmogorov".into(),
            category: LensCategory::Extended,
            description: "Estimate algorithmic complexity and compressibility of patterns".into(),
            domain_affinity: vec!["ai".into(), "mathematics".into(), "signal".into()],
            complementary: vec!["info".into(), "fisher_info".into()],
        },
        LensEntry {
            name: "contradiction".into(),
            category: LensCategory::Extended,
            description: "Identify logical or empirical contradictions within data".into(),
            domain_affinity: vec!["mathematics".into(), "science".into(), "ai".into()],
            complementary: vec!["falsification".into(), "duality".into()],
        },
        // ── XI. Topology-deep (4) ──
        LensEntry {
            name: "knot".into(),
            category: LensCategory::Extended,
            description: "Detect topological knots and linking numbers in entangled structures".into(),
            domain_affinity: vec!["physics".into(), "materials".into(), "biology".into(), "topology".into()],
            complementary: vec!["topology".into(), "chirality".into()],
        },
        LensEntry {
            name: "convexity".into(),
            category: LensCategory::Extended,
            description: "Measure convexity/concavity of landscapes and objective functions".into(),
            domain_affinity: vec!["optimization".into(), "geometry".into(), "finance".into()],
            complementary: vec!["saddle".into(), "compass".into()],
        },
        LensEntry {
            name: "motif".into(),
            category: LensCategory::Extended,
            description: "Find recurring subgraph motifs and structural building blocks".into(),
            domain_affinity: vec!["network".into(), "biology".into(), "software".into()],
            complementary: vec!["network".into(), "recursion".into()],
        },
        LensEntry {
            name: "skeleton".into(),
            category: LensCategory::Extended,
            description: "Extract topological skeletons and medial axes of shapes".into(),
            domain_affinity: vec!["geometry".into(), "materials".into(), "biology".into()],
            complementary: vec!["topology".into(), "hierarchy".into()],
        },
        // ── XII. Ecology (4) ──
        LensEntry {
            name: "carrying_capacity".into(),
            category: LensCategory::Extended,
            description: "Estimate system capacity limits and saturation thresholds".into(),
            domain_affinity: vec!["biology".into(), "energy".into(), "economics".into(), "network".into()],
            complementary: vec!["diminishing_returns".into(), "tipping".into()],
        },
        LensEntry {
            name: "niche".into(),
            category: LensCategory::Extended,
            description: "Identify distinct ecological niches and specialization regions".into(),
            domain_affinity: vec!["biology".into(), "economics".into(), "ai".into()],
            complementary: vec!["comparative_advantage".into(), "void".into()],
        },
        LensEntry {
            name: "symbiosis".into(),
            category: LensCategory::Extended,
            description: "Detect mutualistic coupling between distinct subsystems".into(),
            domain_affinity: vec!["biology".into(), "economics".into(), "materials".into()],
            complementary: vec!["coevolution".into(), "network".into()],
        },
        LensEntry {
            name: "predation".into(),
            category: LensCategory::Extended,
            description: "Model predator-prey oscillation and competitive displacement".into(),
            domain_affinity: vec!["biology".into(), "economics".into(), "security".into()],
            complementary: vec!["coevolution".into(), "wave".into()],
        },
        // ── XIII. Physics-deep (4) ──
        LensEntry {
            name: "morphogenesis".into(),
            category: LensCategory::Extended,
            description: "Detect Turing-like pattern formation and symmetry breaking in growth".into(),
            domain_affinity: vec!["biology".into(), "materials".into(), "chemistry".into()],
            complementary: vec!["emergence".into(), "mirror".into()],
        },
        LensEntry {
            name: "polarity".into(),
            category: LensCategory::Extended,
            description: "Identify directional asymmetry and polar ordering".into(),
            domain_affinity: vec!["biology".into(), "materials".into(), "physics".into()],
            complementary: vec!["mirror".into(), "chirality".into()],
        },
        LensEntry {
            name: "broken_ergodicity".into(),
            category: LensCategory::Extended,
            description: "Detect ergodicity breaking — trapped states and aging dynamics".into(),
            domain_affinity: vec!["physics".into(), "materials".into(), "finance".into()],
            complementary: vec!["aging".into(), "memory".into()],
        },
        LensEntry {
            name: "anomalous_diffusion".into(),
            category: LensCategory::Extended,
            description: "Measure sub/super-diffusive transport deviating from Brownian motion".into(),
            domain_affinity: vec!["physics".into(), "biology".into(), "materials".into()],
            complementary: vec!["diffusion".into(), "broken_ergodicity".into()],
        },
        // ── XIV. Meta-cognition (4) ──
        LensEntry {
            name: "blind_spot".into(),
            category: LensCategory::Extended,
            description: "Detect under-explored regions and systematic observational gaps".into(),
            domain_affinity: vec!["ai".into(), "science".into(), "software".into()],
            complementary: vec!["completeness".into(), "void".into()],
        },
        LensEntry {
            name: "abstraction".into(),
            category: LensCategory::Extended,
            description: "Identify useful abstraction layers and information compression levels".into(),
            domain_affinity: vec!["ai".into(), "software".into(), "mathematics".into()],
            complementary: vec!["hierarchy".into(), "kolmogorov".into()],
        },
        LensEntry {
            name: "narrative".into(),
            category: LensCategory::Extended,
            description: "Detect causal story arcs and temporal narrative structures".into(),
            domain_affinity: vec!["ai".into(), "social".into(), "biology".into()],
            complementary: vec!["causal".into(), "memory".into()],
        },
        LensEntry {
            name: "analogy".into(),
            category: LensCategory::Extended,
            description: "Find structural analogies between disparate domains".into(),
            domain_affinity: vec!["ai".into(), "science".into(), "mathematics".into(), "education".into()],
            complementary: vec!["isomorphism".into(), "duality".into()],
        },
        // ── XV. Decision (4) ──
        LensEntry {
            name: "bottleneck".into(),
            category: LensCategory::Extended,
            description: "Locate throughput-limiting bottlenecks in flow systems".into(),
            domain_affinity: vec!["network".into(), "chip".into(), "energy".into(), "manufacturing".into()],
            complementary: vec!["arbitrage".into(), "info".into()],
        },
        LensEntry {
            name: "diminishing_returns".into(),
            category: LensCategory::Extended,
            description: "Detect marginal returns decay and optimization plateaus".into(),
            domain_affinity: vec!["economics".into(), "ai".into(), "energy".into()],
            complementary: vec!["carrying_capacity".into(), "scale".into()],
        },
        LensEntry {
            name: "option_value".into(),
            category: LensCategory::Extended,
            description: "Quantify value of preserving future optionality".into(),
            domain_affinity: vec!["finance".into(), "strategy".into(), "ai".into()],
            complementary: vec!["bottleneck".into(), "comparative_advantage".into()],
        },
        LensEntry {
            name: "comparative_advantage".into(),
            category: LensCategory::Extended,
            description: "Identify relative strengths for optimal resource allocation".into(),
            domain_affinity: vec!["economics".into(), "ai".into(), "manufacturing".into()],
            complementary: vec!["niche".into(), "arbitrage".into()],
        },
        // ── XVI. Extreme (3) ──
        LensEntry {
            name: "universality_class".into(),
            category: LensCategory::Extended,
            description: "Classify critical exponents into universality classes".into(),
            domain_affinity: vec!["physics".into(), "mathematics".into(), "network".into()],
            complementary: vec!["criticality".into(), "renormalization".into()],
        },
        LensEntry {
            name: "aging".into(),
            category: LensCategory::Extended,
            description: "Detect aging phenomena — slow relaxation and non-stationarity".into(),
            domain_affinity: vec!["materials".into(), "physics".into(), "biology".into()],
            complementary: vec!["broken_ergodicity".into(), "memory".into()],
        },
        LensEntry {
            name: "potential".into(),
            category: LensCategory::Extended,
            description: "Map effective potential landscapes and energy surfaces".into(),
            domain_affinity: vec!["physics".into(), "chemistry".into(), "optimization".into()],
            complementary: vec!["saddle".into(), "thermo".into()],
        },
        // ── XVII. Additional (2) ──
        LensEntry {
            name: "chirality".into(),
            category: LensCategory::Extended,
            description: "Detect handedness and chiral asymmetry in structures".into(),
            domain_affinity: vec!["chemistry".into(), "biology".into(), "materials".into(), "physics".into()],
            complementary: vec!["mirror".into(), "polarity".into()],
        },
        LensEntry {
            name: "barrier".into(),
            category: LensCategory::Extended,
            description: "Measure energy barriers separating metastable states".into(),
            domain_affinity: vec!["chemistry".into(), "materials".into(), "physics".into(), "energy".into()],
            complementary: vec!["saddle".into(), "thermo".into()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n6_industry_lens_count() {
        let entries = n6_industry_lens_entries();
        assert_eq!(entries.len(), 58, "Must have exactly 58 n6-industry lenses");
    }

    #[test]
    fn test_n6_industry_lens_names_unique() {
        let entries = n6_industry_lens_entries();
        let mut names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        let total = names.len();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), total, "All n6-industry lens names must be unique");
    }

    #[test]
    fn test_n6_industry_all_extended() {
        let entries = n6_industry_lens_entries();
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
