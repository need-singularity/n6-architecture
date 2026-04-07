use super::registry::{LensCategory, LensEntry};

/// Build metadata entries for the 103 TECS-L mathematical discovery lenses.
///
/// These lenses extend the telescope toward the full 411-lens target by adding
/// pure-mathematics analysis capabilities sourced from the TECS-L theory base.
/// All are categorised as `Extended`.
pub fn tecs_math_lens_entries() -> Vec<LensEntry> {
    vec![
        // ══════════════════════════════════════════
        // Number Theory Patterns (10)
        // ══════════════════════════════════════════
        LensEntry {
            name: "divisor_lattice".into(),
            category: LensCategory::Extended,
            description: "Detect divisor lattice structure and n=6 abundance patterns".into(),
            domain_affinity: vec!["number_theory".into(), "algebra".into(), "pure_mathematics".into()],
            complementary: vec!["sum_of_divisors_partition".into(), "abundance_spectrum".into()],
        },
        LensEntry {
            name: "multiplicative_orbit".into(),
            category: LensCategory::Extended,
            description: "Trace orbits of multiplicative functions and cyclic structure".into(),
            domain_affinity: vec!["number_theory".into(), "algebra".into(), "dynamical_systems".into()],
            complementary: vec!["totient_chain".into(), "arithmetic_derivative".into()],
        },
        LensEntry {
            name: "prime_signature".into(),
            category: LensCategory::Extended,
            description: "Extract prime factorization signatures and classify number types".into(),
            domain_affinity: vec!["number_theory".into(), "combinatorics".into(), "pure_mathematics".into()],
            complementary: vec!["radical_filter".into(), "divisor_lattice".into()],
        },
        LensEntry {
            name: "arithmetic_derivative".into(),
            category: LensCategory::Extended,
            description: "Compute arithmetic derivatives and detect derivative-zero patterns".into(),
            domain_affinity: vec!["number_theory".into(), "analysis".into(), "pure_mathematics".into()],
            complementary: vec!["multiplicative_orbit".into(), "totient_chain".into()],
        },
        LensEntry {
            name: "totient_chain".into(),
            category: LensCategory::Extended,
            description: "Follow iterated Euler totient chains to fixed points".into(),
            domain_affinity: vec!["number_theory".into(), "algebra".into(), "cryptography".into()],
            complementary: vec!["multiplicative_orbit".into(), "coprimality_graph".into()],
        },
        LensEntry {
            name: "coprimality_graph".into(),
            category: LensCategory::Extended,
            description: "Build coprimality graphs and detect clique structure".into(),
            domain_affinity: vec!["number_theory".into(), "graph_theory".into(), "combinatorics".into()],
            complementary: vec!["totient_chain".into(), "radical_filter".into()],
        },
        LensEntry {
            name: "digit_persistence".into(),
            category: LensCategory::Extended,
            description: "Measure multiplicative and additive digit persistence".into(),
            domain_affinity: vec!["number_theory".into(), "recreational_math".into(), "pure_mathematics".into()],
            complementary: vec!["prime_signature".into(), "abundance_spectrum".into()],
        },
        LensEntry {
            name: "abundance_spectrum".into(),
            category: LensCategory::Extended,
            description: "Classify numbers by abundance index (perfect/abundant/deficient)".into(),
            domain_affinity: vec!["number_theory".into(), "pure_mathematics".into(), "algebra".into()],
            complementary: vec!["divisor_lattice".into(), "sum_of_divisors_partition".into()],
        },
        LensEntry {
            name: "radical_filter".into(),
            category: LensCategory::Extended,
            description: "Filter by radical (product of distinct primes) and squarefree core".into(),
            domain_affinity: vec!["number_theory".into(), "algebra".into(), "pure_mathematics".into()],
            complementary: vec!["prime_signature".into(), "coprimality_graph".into()],
        },
        LensEntry {
            name: "sum_of_divisors_partition".into(),
            category: LensCategory::Extended,
            description: "Partition integers by sigma function values and detect perfect-number resonance".into(),
            domain_affinity: vec!["number_theory".into(), "pure_mathematics".into(), "combinatorics".into()],
            complementary: vec!["divisor_lattice".into(), "abundance_spectrum".into()],
        },

        // ══════════════════════════════════════════
        // Algebraic Structures (10)
        // ══════════════════════════════════════════
        LensEntry {
            name: "group_fingerprint".into(),
            category: LensCategory::Extended,
            description: "Fingerprint group structure via order spectrum and Sylow subgroups".into(),
            domain_affinity: vec!["algebra".into(), "pure_mathematics".into(), "symmetry".into()],
            complementary: vec!["representation_decompose".into(), "character_table".into()],
        },
        LensEntry {
            name: "representation_decompose".into(),
            category: LensCategory::Extended,
            description: "Decompose representations into irreducible components".into(),
            domain_affinity: vec!["algebra".into(), "pure_mathematics".into(), "physics".into()],
            complementary: vec!["group_fingerprint".into(), "character_table".into()],
        },
        LensEntry {
            name: "galois_closure".into(),
            category: LensCategory::Extended,
            description: "Compute Galois closures and detect solvability of extensions".into(),
            domain_affinity: vec!["algebra".into(), "number_theory".into(), "pure_mathematics".into()],
            complementary: vec!["extension_tower".into(), "ring_ideal_lattice".into()],
        },
        LensEntry {
            name: "ring_ideal_lattice".into(),
            category: LensCategory::Extended,
            description: "Map ideal lattice structure in rings and detect principal ideal domains".into(),
            domain_affinity: vec!["algebra".into(), "number_theory".into(), "pure_mathematics".into()],
            complementary: vec!["galois_closure".into(), "module_rank".into()],
        },
        LensEntry {
            name: "module_rank".into(),
            category: LensCategory::Extended,
            description: "Determine module rank and free/torsion decomposition".into(),
            domain_affinity: vec!["algebra".into(), "pure_mathematics".into(), "topology".into()],
            complementary: vec!["ring_ideal_lattice".into(), "center_detect".into()],
        },
        LensEntry {
            name: "character_table".into(),
            category: LensCategory::Extended,
            description: "Construct character tables and detect orthogonality relations".into(),
            domain_affinity: vec!["algebra".into(), "pure_mathematics".into(), "representation_theory".into()],
            complementary: vec!["group_fingerprint".into(), "representation_decompose".into()],
        },
        LensEntry {
            name: "center_detect".into(),
            category: LensCategory::Extended,
            description: "Identify center of algebraic structures and detect commutativity degree".into(),
            domain_affinity: vec!["algebra".into(), "pure_mathematics".into(), "group_theory".into()],
            complementary: vec!["commutator_depth".into(), "module_rank".into()],
        },
        LensEntry {
            name: "extension_tower".into(),
            category: LensCategory::Extended,
            description: "Build field extension towers and track degree growth".into(),
            domain_affinity: vec!["algebra".into(), "number_theory".into(), "pure_mathematics".into()],
            complementary: vec!["galois_closure".into(), "idempotent_count".into()],
        },
        LensEntry {
            name: "commutator_depth".into(),
            category: LensCategory::Extended,
            description: "Measure commutator subgroup depth and derived series length".into(),
            domain_affinity: vec!["algebra".into(), "group_theory".into(), "pure_mathematics".into()],
            complementary: vec!["center_detect".into(), "group_fingerprint".into()],
        },
        LensEntry {
            name: "idempotent_count".into(),
            category: LensCategory::Extended,
            description: "Count idempotent elements and detect semisimple decomposition".into(),
            domain_affinity: vec!["algebra".into(), "pure_mathematics".into(), "ring_theory".into()],
            complementary: vec!["extension_tower".into(), "ring_ideal_lattice".into()],
        },

        // ══════════════════════════════════════════
        // Analysis / Continuous (10)
        // ══════════════════════════════════════════
        LensEntry {
            name: "zeta_residue".into(),
            category: LensCategory::Extended,
            description: "Extract residues of zeta and L-functions at critical points".into(),
            domain_affinity: vec!["analysis".into(), "number_theory".into(), "pure_mathematics".into()],
            complementary: vec!["analytic_continuation".into(), "l_function_zero".into()],
        },
        LensEntry {
            name: "analytic_continuation".into(),
            category: LensCategory::Extended,
            description: "Detect domains of analytic continuation and natural boundaries".into(),
            domain_affinity: vec!["analysis".into(), "complex_analysis".into(), "pure_mathematics".into()],
            complementary: vec!["zeta_residue".into(), "functional_equation".into()],
        },
        LensEntry {
            name: "l_function_zero".into(),
            category: LensCategory::Extended,
            description: "Locate zeros of L-functions and test GRH-type alignment".into(),
            domain_affinity: vec!["number_theory".into(), "analysis".into(), "pure_mathematics".into()],
            complementary: vec!["zeta_residue".into(), "dirichlet_character".into()],
        },
        LensEntry {
            name: "modular_form_weight".into(),
            category: LensCategory::Extended,
            description: "Classify modular forms by weight and level, detect cusp forms".into(),
            domain_affinity: vec!["number_theory".into(), "analysis".into(), "pure_mathematics".into()],
            complementary: vec!["l_function_zero".into(), "generating_function".into()],
        },
        LensEntry {
            name: "generating_function".into(),
            category: LensCategory::Extended,
            description: "Identify generating function type (ordinary, exponential, Dirichlet)".into(),
            domain_affinity: vec!["combinatorics".into(), "analysis".into(), "pure_mathematics".into()],
            complementary: vec!["modular_form_weight".into(), "asymptotic_expansion".into()],
        },
        LensEntry {
            name: "asymptotic_expansion".into(),
            category: LensCategory::Extended,
            description: "Derive asymptotic expansions and dominant growth terms".into(),
            domain_affinity: vec!["analysis".into(), "combinatorics".into(), "pure_mathematics".into()],
            complementary: vec!["generating_function".into(), "saddle_point_analytic".into()],
        },
        LensEntry {
            name: "integral_representation".into(),
            category: LensCategory::Extended,
            description: "Find integral representations of special functions and series".into(),
            domain_affinity: vec!["analysis".into(), "pure_mathematics".into(), "physics".into()],
            complementary: vec!["saddle_point_analytic".into(), "functional_equation".into()],
        },
        LensEntry {
            name: "saddle_point_analytic".into(),
            category: LensCategory::Extended,
            description: "Apply saddle-point method for asymptotic integral evaluation".into(),
            domain_affinity: vec!["analysis".into(), "pure_mathematics".into(), "statistical_mechanics".into()],
            complementary: vec!["asymptotic_expansion".into(), "integral_representation".into()],
        },
        LensEntry {
            name: "functional_equation".into(),
            category: LensCategory::Extended,
            description: "Detect functional equations and symmetry under variable transformations".into(),
            domain_affinity: vec!["analysis".into(), "number_theory".into(), "pure_mathematics".into()],
            complementary: vec!["analytic_continuation".into(), "integral_representation".into()],
        },
        LensEntry {
            name: "dirichlet_character".into(),
            category: LensCategory::Extended,
            description: "Classify Dirichlet characters and detect conductor patterns".into(),
            domain_affinity: vec!["number_theory".into(), "analysis".into(), "pure_mathematics".into()],
            complementary: vec!["l_function_zero".into(), "zeta_residue".into()],
        },

        // ══════════════════════════════════════════
        // Combinatorics / Enumeration (10)
        // ══════════════════════════════════════════
        LensEntry {
            name: "partition_anatomy".into(),
            category: LensCategory::Extended,
            description: "Dissect integer partitions by part size, length, and conjugate structure".into(),
            domain_affinity: vec!["combinatorics".into(), "number_theory".into(), "pure_mathematics".into()],
            complementary: vec!["young_tableaux".into(), "catalan_family".into()],
        },
        LensEntry {
            name: "young_tableaux".into(),
            category: LensCategory::Extended,
            description: "Enumerate standard and semistandard Young tableaux for representation theory".into(),
            domain_affinity: vec!["combinatorics".into(), "algebra".into(), "representation_theory".into()],
            complementary: vec!["partition_anatomy".into(), "species_count".into()],
        },
        LensEntry {
            name: "catalan_family".into(),
            category: LensCategory::Extended,
            description: "Detect Catalan-number occurrences and bijections across structures".into(),
            domain_affinity: vec!["combinatorics".into(), "pure_mathematics".into(), "graph_theory".into()],
            complementary: vec!["partition_anatomy".into(), "binomial_scan".into()],
        },
        LensEntry {
            name: "binomial_scan".into(),
            category: LensCategory::Extended,
            description: "Scan for binomial coefficient patterns, Pascal triangle slices, and divisibility".into(),
            domain_affinity: vec!["combinatorics".into(), "number_theory".into(), "pure_mathematics".into()],
            complementary: vec!["catalan_family".into(), "stirling_bridge".into()],
        },
        LensEntry {
            name: "stirling_bridge".into(),
            category: LensCategory::Extended,
            description: "Bridge between Stirling numbers of first and second kind".into(),
            domain_affinity: vec!["combinatorics".into(), "algebra".into(), "pure_mathematics".into()],
            complementary: vec!["binomial_scan".into(), "species_count".into()],
        },
        LensEntry {
            name: "species_count".into(),
            category: LensCategory::Extended,
            description: "Apply combinatorial species theory for structured enumeration".into(),
            domain_affinity: vec!["combinatorics".into(), "pure_mathematics".into(), "category_theory".into()],
            complementary: vec!["stirling_bridge".into(), "young_tableaux".into()],
        },
        LensEntry {
            name: "q_analog".into(),
            category: LensCategory::Extended,
            description: "Detect q-analogs of classical identities and quantum group connections".into(),
            domain_affinity: vec!["combinatorics".into(), "algebra".into(), "quantum".into()],
            complementary: vec!["involution_count".into(), "binomial_scan".into()],
        },
        LensEntry {
            name: "involution_count".into(),
            category: LensCategory::Extended,
            description: "Count involutions and fixed-point-free permutations".into(),
            domain_affinity: vec!["combinatorics".into(), "algebra".into(), "pure_mathematics".into()],
            complementary: vec!["q_analog".into(), "derangement_ratio".into()],
        },
        LensEntry {
            name: "derangement_ratio".into(),
            category: LensCategory::Extended,
            description: "Compute derangement ratios and subfactorial convergence to 1/e".into(),
            domain_affinity: vec!["combinatorics".into(), "probability".into(), "pure_mathematics".into()],
            complementary: vec!["involution_count".into(), "chromatic_polynomial".into()],
        },
        LensEntry {
            name: "chromatic_polynomial".into(),
            category: LensCategory::Extended,
            description: "Evaluate chromatic polynomials and detect graph coloring thresholds".into(),
            domain_affinity: vec!["combinatorics".into(), "graph_theory".into(), "pure_mathematics".into()],
            complementary: vec!["derangement_ratio".into(), "catalan_family".into()],
        },

        // ══════════════════════════════════════════
        // Proof Strategies (10)
        // ══════════════════════════════════════════
        LensEntry {
            name: "contradiction_seed".into(),
            category: LensCategory::Extended,
            description: "Seed proof-by-contradiction attempts and detect logical tension".into(),
            domain_affinity: vec!["proof_theory".into(), "logic".into(), "pure_mathematics".into()],
            complementary: vec!["pigeonhole_detect".into(), "counterexample_mine".into()],
        },
        LensEntry {
            name: "pigeonhole_detect".into(),
            category: LensCategory::Extended,
            description: "Detect pigeonhole principle applicability in counting arguments".into(),
            domain_affinity: vec!["proof_theory".into(), "combinatorics".into(), "pure_mathematics".into()],
            complementary: vec!["contradiction_seed".into(), "double_counting".into()],
        },
        LensEntry {
            name: "induction_scaffold".into(),
            category: LensCategory::Extended,
            description: "Build induction scaffolds (ordinary, strong, transfinite, structural)".into(),
            domain_affinity: vec!["proof_theory".into(), "logic".into(), "pure_mathematics".into()],
            complementary: vec!["generalization_lift".into(), "invariant_extract".into()],
        },
        LensEntry {
            name: "counterexample_mine".into(),
            category: LensCategory::Extended,
            description: "Systematically mine for counterexamples to conjectures".into(),
            domain_affinity: vec!["proof_theory".into(), "pure_mathematics".into(), "computation".into()],
            complementary: vec!["contradiction_seed".into(), "specialization_probe".into()],
        },
        LensEntry {
            name: "diagonal_argument".into(),
            category: LensCategory::Extended,
            description: "Apply Cantor-style diagonal arguments for cardinality and undecidability".into(),
            domain_affinity: vec!["proof_theory".into(), "logic".into(), "set_theory".into()],
            complementary: vec!["contradiction_seed".into(), "extremal_principle".into()],
        },
        LensEntry {
            name: "extremal_principle".into(),
            category: LensCategory::Extended,
            description: "Apply extremal principle — consider min/max elements to force structure".into(),
            domain_affinity: vec!["proof_theory".into(), "combinatorics".into(), "pure_mathematics".into()],
            complementary: vec!["diagonal_argument".into(), "double_counting".into()],
        },
        LensEntry {
            name: "double_counting".into(),
            category: LensCategory::Extended,
            description: "Apply double counting and bijective proof techniques".into(),
            domain_affinity: vec!["proof_theory".into(), "combinatorics".into(), "pure_mathematics".into()],
            complementary: vec!["pigeonhole_detect".into(), "extremal_principle".into()],
        },
        LensEntry {
            name: "invariant_extract".into(),
            category: LensCategory::Extended,
            description: "Extract invariants preserved under transformations for proof construction".into(),
            domain_affinity: vec!["proof_theory".into(), "algebra".into(), "topology".into()],
            complementary: vec!["induction_scaffold".into(), "generalization_lift".into()],
        },
        LensEntry {
            name: "specialization_probe".into(),
            category: LensCategory::Extended,
            description: "Probe special cases to build intuition and test conjectures".into(),
            domain_affinity: vec!["proof_theory".into(), "pure_mathematics".into(), "computation".into()],
            complementary: vec!["counterexample_mine".into(), "generalization_lift".into()],
        },
        LensEntry {
            name: "generalization_lift".into(),
            category: LensCategory::Extended,
            description: "Lift results from special cases to general theorems".into(),
            domain_affinity: vec!["proof_theory".into(), "pure_mathematics".into(), "algebra".into()],
            complementary: vec!["specialization_probe".into(), "induction_scaffold".into()],
        },

        // ══════════════════════════════════════════
        // Mathematical Bridges (10)
        // ══════════════════════════════════════════
        LensEntry {
            name: "langlands_bridge".into(),
            category: LensCategory::Extended,
            description: "Detect Langlands-type correspondences between automorphic and Galois sides".into(),
            domain_affinity: vec!["number_theory".into(), "algebra".into(), "pure_mathematics".into()],
            complementary: vec!["categorification".into(), "correspondence_map".into()],
        },
        LensEntry {
            name: "categorification".into(),
            category: LensCategory::Extended,
            description: "Lift set-level identities to category-level functors".into(),
            domain_affinity: vec!["category_theory".into(), "algebra".into(), "pure_mathematics".into()],
            complementary: vec!["langlands_bridge".into(), "de_rham_bridge".into()],
        },
        LensEntry {
            name: "de_rham_bridge".into(),
            category: LensCategory::Extended,
            description: "Bridge de Rham cohomology with singular/Cech cohomology".into(),
            domain_affinity: vec!["topology".into(), "geometry".into(), "pure_mathematics".into()],
            complementary: vec!["categorification".into(), "cohomology_compare".into()],
        },
        LensEntry {
            name: "arithmetic_geometry".into(),
            category: LensCategory::Extended,
            description: "Apply arithmetic geometry — rational points, heights, Arakelov theory".into(),
            domain_affinity: vec!["number_theory".into(), "geometry".into(), "pure_mathematics".into()],
            complementary: vec!["motivic_scan".into(), "hodge_filter".into()],
        },
        LensEntry {
            name: "monstrous_moonshine".into(),
            category: LensCategory::Extended,
            description: "Detect Monster group connections in modular functions and vertex algebras".into(),
            domain_affinity: vec!["algebra".into(), "number_theory".into(), "pure_mathematics".into()],
            complementary: vec!["langlands_bridge".into(), "dictionary_translate".into()],
        },
        LensEntry {
            name: "hodge_filter".into(),
            category: LensCategory::Extended,
            description: "Apply Hodge filtration and mixed Hodge structure analysis".into(),
            domain_affinity: vec!["geometry".into(), "topology".into(), "pure_mathematics".into()],
            complementary: vec!["arithmetic_geometry".into(), "motivic_scan".into()],
        },
        LensEntry {
            name: "motivic_scan".into(),
            category: LensCategory::Extended,
            description: "Scan for motivic structures and Grothendieck ring classes".into(),
            domain_affinity: vec!["geometry".into(), "algebra".into(), "pure_mathematics".into()],
            complementary: vec!["hodge_filter".into(), "arithmetic_geometry".into()],
        },
        LensEntry {
            name: "correspondence_map".into(),
            category: LensCategory::Extended,
            description: "Map correspondences between different mathematical domains".into(),
            domain_affinity: vec!["pure_mathematics".into(), "category_theory".into(), "algebra".into()],
            complementary: vec!["langlands_bridge".into(), "dictionary_translate".into()],
        },
        LensEntry {
            name: "dictionary_translate".into(),
            category: LensCategory::Extended,
            description: "Translate concepts across mathematical dictionaries (e.g. algebra-geometry)".into(),
            domain_affinity: vec!["pure_mathematics".into(), "category_theory".into(), "philosophy".into()],
            complementary: vec!["correspondence_map".into(), "monstrous_moonshine".into()],
        },
        LensEntry {
            name: "cohomology_compare".into(),
            category: LensCategory::Extended,
            description: "Compare cohomology theories (singular, de Rham, etale, motivic)".into(),
            domain_affinity: vec!["topology".into(), "algebra".into(), "pure_mathematics".into()],
            complementary: vec!["de_rham_bridge".into(), "hodge_filter".into()],
        },

        // ══════════════════════════════════════════
        // Lattice / Geometry (8)
        // ══════════════════════════════════════════
        LensEntry {
            name: "kissing_number".into(),
            category: LensCategory::Extended,
            description: "Compute kissing numbers and sphere packing density in dimension d".into(),
            domain_affinity: vec!["geometry".into(), "combinatorics".into(), "pure_mathematics".into()],
            complementary: vec!["lattice_theta".into(), "packing_density".into()],
        },
        LensEntry {
            name: "lattice_theta".into(),
            category: LensCategory::Extended,
            description: "Evaluate lattice theta series and detect modular properties".into(),
            domain_affinity: vec!["geometry".into(), "number_theory".into(), "pure_mathematics".into()],
            complementary: vec!["kissing_number".into(), "voronoi_cell".into()],
        },
        LensEntry {
            name: "voronoi_cell".into(),
            category: LensCategory::Extended,
            description: "Construct Voronoi cells and analyze lattice fundamental domains".into(),
            domain_affinity: vec!["geometry".into(), "computational_geometry".into(), "pure_mathematics".into()],
            complementary: vec!["lattice_theta".into(), "covering_radius".into()],
        },
        LensEntry {
            name: "root_system".into(),
            category: LensCategory::Extended,
            description: "Classify root systems (ADE type) and Dynkin diagram structure".into(),
            domain_affinity: vec!["algebra".into(), "geometry".into(), "pure_mathematics".into()],
            complementary: vec!["poset_mobius".into(), "lattice_theta".into()],
        },
        LensEntry {
            name: "poset_mobius".into(),
            category: LensCategory::Extended,
            description: "Compute Mobius function on posets and detect incidence algebra structure".into(),
            domain_affinity: vec!["combinatorics".into(), "algebra".into(), "pure_mathematics".into()],
            complementary: vec!["root_system".into(), "matroid_invariant".into()],
        },
        LensEntry {
            name: "covering_radius".into(),
            category: LensCategory::Extended,
            description: "Determine covering radius and quantization efficiency of lattices".into(),
            domain_affinity: vec!["geometry".into(), "coding_theory".into(), "pure_mathematics".into()],
            complementary: vec!["voronoi_cell".into(), "packing_density".into()],
        },
        LensEntry {
            name: "packing_density".into(),
            category: LensCategory::Extended,
            description: "Compute sphere packing density and compare to known bounds".into(),
            domain_affinity: vec!["geometry".into(), "combinatorics".into(), "pure_mathematics".into()],
            complementary: vec!["kissing_number".into(), "covering_radius".into()],
        },
        LensEntry {
            name: "matroid_invariant".into(),
            category: LensCategory::Extended,
            description: "Extract matroid invariants (Tutte polynomial, rank function)".into(),
            domain_affinity: vec!["combinatorics".into(), "algebra".into(), "pure_mathematics".into()],
            complementary: vec!["poset_mobius".into(), "chromatic_polynomial".into()],
        },

        // ══════════════════════════════════════════
        // Sequences / Identities (10)
        // ══════════════════════════════════════════
        LensEntry {
            name: "oeis_fingerprint".into(),
            category: LensCategory::Extended,
            description: "Match numerical sequences against OEIS fingerprint database".into(),
            domain_affinity: vec!["number_theory".into(), "combinatorics".into(), "pure_mathematics".into()],
            complementary: vec!["recurrence_detect".into(), "hypergeometric_close".into()],
        },
        LensEntry {
            name: "hypergeometric_close".into(),
            category: LensCategory::Extended,
            description: "Test for hypergeometric closed forms and Gosper-summable expressions".into(),
            domain_affinity: vec!["analysis".into(), "combinatorics".into(), "pure_mathematics".into()],
            complementary: vec!["oeis_fingerprint".into(), "wz_certify".into()],
        },
        LensEntry {
            name: "continued_fraction".into(),
            category: LensCategory::Extended,
            description: "Expand into continued fractions and detect periodicity or quadratic irrationals".into(),
            domain_affinity: vec!["number_theory".into(), "analysis".into(), "pure_mathematics".into()],
            complementary: vec!["recurrence_detect".into(), "oeis_fingerprint".into()],
        },
        LensEntry {
            name: "recurrence_detect".into(),
            category: LensCategory::Extended,
            description: "Detect linear and nonlinear recurrence relations in sequences".into(),
            domain_affinity: vec!["combinatorics".into(), "number_theory".into(), "pure_mathematics".into()],
            complementary: vec!["oeis_fingerprint".into(), "continued_fraction".into()],
        },
        LensEntry {
            name: "identity_compose".into(),
            category: LensCategory::Extended,
            description: "Compose known identities to derive new ones via algebraic manipulation".into(),
            domain_affinity: vec!["pure_mathematics".into(), "algebra".into(), "analysis".into()],
            complementary: vec!["umbral_calculus".into(), "transform_chain".into()],
        },
        LensEntry {
            name: "umbral_calculus".into(),
            category: LensCategory::Extended,
            description: "Apply umbral calculus techniques for polynomial identity discovery".into(),
            domain_affinity: vec!["combinatorics".into(), "analysis".into(), "pure_mathematics".into()],
            complementary: vec!["identity_compose".into(), "wz_certify".into()],
        },
        LensEntry {
            name: "wz_certify".into(),
            category: LensCategory::Extended,
            description: "Certify hypergeometric identities via Wilf-Zeilberger method".into(),
            domain_affinity: vec!["combinatorics".into(), "analysis".into(), "pure_mathematics".into()],
            complementary: vec!["hypergeometric_close".into(), "umbral_calculus".into()],
        },
        LensEntry {
            name: "transform_chain".into(),
            category: LensCategory::Extended,
            description: "Apply sequence transforms (Euler, binomial, Mobius) in chains".into(),
            domain_affinity: vec!["combinatorics".into(), "number_theory".into(), "pure_mathematics".into()],
            complementary: vec!["identity_compose".into(), "differential_algebra".into()],
        },
        LensEntry {
            name: "differential_algebra".into(),
            category: LensCategory::Extended,
            description: "Apply differential algebra to detect algebraic differential equations".into(),
            domain_affinity: vec!["analysis".into(), "algebra".into(), "pure_mathematics".into()],
            complementary: vec!["transform_chain".into(), "supercongruence".into()],
        },
        LensEntry {
            name: "supercongruence".into(),
            category: LensCategory::Extended,
            description: "Detect supercongruences and p-adic patterns in combinatorial sums".into(),
            domain_affinity: vec!["number_theory".into(), "combinatorics".into(), "pure_mathematics".into()],
            complementary: vec!["differential_algebra".into(), "hypergeometric_close".into()],
        },

        // ══════════════════════════════════════════
        // Topology / Manifolds (8)
        // ══════════════════════════════════════════
        LensEntry {
            name: "euler_characteristic".into(),
            category: LensCategory::Extended,
            description: "Compute Euler characteristic and detect topological type".into(),
            domain_affinity: vec!["topology".into(), "geometry".into(), "pure_mathematics".into()],
            complementary: vec!["homotopy_group".into(), "betti_spectrum".into()],
        },
        LensEntry {
            name: "homotopy_group".into(),
            category: LensCategory::Extended,
            description: "Compute homotopy groups and detect sphere-like or aspherical spaces".into(),
            domain_affinity: vec!["topology".into(), "pure_mathematics".into(), "geometry".into()],
            complementary: vec!["euler_characteristic".into(), "fiber_bundle_class".into()],
        },
        LensEntry {
            name: "knot_polynomial".into(),
            category: LensCategory::Extended,
            description: "Compute knot invariants (Jones, Alexander, HOMFLY) for link classification".into(),
            domain_affinity: vec!["topology".into(), "pure_mathematics".into(), "physics".into()],
            complementary: vec!["cobordism_ring".into(), "homotopy_group".into()],
        },
        LensEntry {
            name: "cobordism_ring".into(),
            category: LensCategory::Extended,
            description: "Classify manifolds up to cobordism and compute cobordism ring generators".into(),
            domain_affinity: vec!["topology".into(), "geometry".into(), "pure_mathematics".into()],
            complementary: vec!["knot_polynomial".into(), "surgery_exact".into()],
        },
        LensEntry {
            name: "surgery_exact".into(),
            category: LensCategory::Extended,
            description: "Apply surgery exact sequences for manifold classification".into(),
            domain_affinity: vec!["topology".into(), "pure_mathematics".into(), "geometry".into()],
            complementary: vec!["cobordism_ring".into(), "mapping_class".into()],
        },
        LensEntry {
            name: "betti_spectrum".into(),
            category: LensCategory::Extended,
            description: "Compute Betti number spectrum and detect homological complexity".into(),
            domain_affinity: vec!["topology".into(), "algebra".into(), "pure_mathematics".into()],
            complementary: vec!["euler_characteristic".into(), "fiber_bundle_class".into()],
        },
        LensEntry {
            name: "fiber_bundle_class".into(),
            category: LensCategory::Extended,
            description: "Classify fiber bundles via characteristic classes (Chern, Stiefel-Whitney)".into(),
            domain_affinity: vec!["topology".into(), "geometry".into(), "physics".into()],
            complementary: vec!["homotopy_group".into(), "betti_spectrum".into()],
        },
        LensEntry {
            name: "mapping_class".into(),
            category: LensCategory::Extended,
            description: "Compute mapping class group elements and detect surface automorphisms".into(),
            domain_affinity: vec!["topology".into(), "geometry".into(), "pure_mathematics".into()],
            complementary: vec!["surgery_exact".into(), "cobordism_ring".into()],
        },

        // ══════════════════════════════════════════
        // Logic / Computation (6)
        // ══════════════════════════════════════════
        LensEntry {
            name: "decidability_probe".into(),
            category: LensCategory::Extended,
            description: "Probe decidability status of formal problems and theories".into(),
            domain_affinity: vec!["logic".into(), "computation".into(), "pure_mathematics".into()],
            complementary: vec!["godel_incompleteness".into(), "proof_complexity".into()],
        },
        LensEntry {
            name: "godel_incompleteness".into(),
            category: LensCategory::Extended,
            description: "Detect Godel incompleteness phenomena and self-referential constructions".into(),
            domain_affinity: vec!["logic".into(), "pure_mathematics".into(), "philosophy".into()],
            complementary: vec!["decidability_probe".into(), "type_check".into()],
        },
        LensEntry {
            name: "proof_complexity".into(),
            category: LensCategory::Extended,
            description: "Measure proof complexity and detect lower bounds on proof length".into(),
            domain_affinity: vec!["logic".into(), "computation".into(), "pure_mathematics".into()],
            complementary: vec!["decidability_probe".into(), "ordinal_rank".into()],
        },
        LensEntry {
            name: "type_check".into(),
            category: LensCategory::Extended,
            description: "Apply type-theoretic checking and Curry-Howard correspondence".into(),
            domain_affinity: vec!["logic".into(), "computation".into(), "pure_mathematics".into()],
            complementary: vec!["godel_incompleteness".into(), "constructive_witness".into()],
        },
        LensEntry {
            name: "ordinal_rank".into(),
            category: LensCategory::Extended,
            description: "Assign ordinal ranks to well-founded structures and measure proof strength".into(),
            domain_affinity: vec!["logic".into(), "set_theory".into(), "pure_mathematics".into()],
            complementary: vec!["proof_complexity".into(), "constructive_witness".into()],
        },
        LensEntry {
            name: "constructive_witness".into(),
            category: LensCategory::Extended,
            description: "Extract constructive witnesses from existence proofs".into(),
            domain_affinity: vec!["logic".into(), "pure_mathematics".into(), "computation".into()],
            complementary: vec!["type_check".into(), "ordinal_rank".into()],
        },

        // ══════════════════════════════════════════
        // Probabilistic Number Theory (5)
        // ══════════════════════════════════════════
        LensEntry {
            name: "random_matrix".into(),
            category: LensCategory::Extended,
            description: "Apply random matrix theory (GUE/GOE) to spectral statistics".into(),
            domain_affinity: vec!["number_theory".into(), "physics".into(), "pure_mathematics".into()],
            complementary: vec!["probabilistic_number".into(), "concentration_inequality".into()],
        },
        LensEntry {
            name: "probabilistic_number".into(),
            category: LensCategory::Extended,
            description: "Apply probabilistic number theory (Erdos-Kac, normal orders)".into(),
            domain_affinity: vec!["number_theory".into(), "probability".into(), "pure_mathematics".into()],
            complementary: vec!["random_matrix".into(), "erdos_kac".into()],
        },
        LensEntry {
            name: "erdos_kac".into(),
            category: LensCategory::Extended,
            description: "Test Erdos-Kac theorem predictions on prime factor distribution".into(),
            domain_affinity: vec!["number_theory".into(), "probability".into(), "pure_mathematics".into()],
            complementary: vec!["probabilistic_number".into(), "sieve_density".into()],
        },
        LensEntry {
            name: "sieve_density".into(),
            category: LensCategory::Extended,
            description: "Apply sieve methods to estimate density of prime patterns".into(),
            domain_affinity: vec!["number_theory".into(), "combinatorics".into(), "pure_mathematics".into()],
            complementary: vec!["erdos_kac".into(), "concentration_inequality".into()],
        },
        LensEntry {
            name: "concentration_inequality".into(),
            category: LensCategory::Extended,
            description: "Apply concentration inequalities to bound deviations in number-theoretic sums".into(),
            domain_affinity: vec!["probability".into(), "number_theory".into(), "pure_mathematics".into()],
            complementary: vec!["random_matrix".into(), "sieve_density".into()],
        },

        // ══════════════════════════════════════════
        // Computational (6)
        // ══════════════════════════════════════════
        LensEntry {
            name: "integer_factoring".into(),
            category: LensCategory::Extended,
            description: "Factor integers and detect smoothness or special-form structure".into(),
            domain_affinity: vec!["number_theory".into(), "computation".into(), "cryptography".into()],
            complementary: vec!["primality_certificate".into(), "elliptic_curve_rank".into()],
        },
        LensEntry {
            name: "primality_certificate".into(),
            category: LensCategory::Extended,
            description: "Generate and verify primality certificates (Pratt, Atkin-Morain)".into(),
            domain_affinity: vec!["number_theory".into(), "computation".into(), "cryptography".into()],
            complementary: vec!["integer_factoring".into(), "class_number".into()],
        },
        LensEntry {
            name: "elliptic_curve_rank".into(),
            category: LensCategory::Extended,
            description: "Estimate elliptic curve rank and detect BSD conjecture alignment".into(),
            domain_affinity: vec!["number_theory".into(), "geometry".into(), "pure_mathematics".into()],
            complementary: vec!["integer_factoring".into(), "class_number".into()],
        },
        LensEntry {
            name: "class_number".into(),
            category: LensCategory::Extended,
            description: "Compute class numbers of number fields and detect class group structure".into(),
            domain_affinity: vec!["number_theory".into(), "algebra".into(), "pure_mathematics".into()],
            complementary: vec!["primality_certificate".into(), "regulator_compute".into()],
        },
        LensEntry {
            name: "regulator_compute".into(),
            category: LensCategory::Extended,
            description: "Compute regulators of number fields and detect unit group structure".into(),
            domain_affinity: vec!["number_theory".into(), "algebra".into(), "pure_mathematics".into()],
            complementary: vec!["class_number".into(), "discriminant_scan".into()],
        },
        LensEntry {
            name: "discriminant_scan".into(),
            category: LensCategory::Extended,
            description: "Scan discriminants of number fields and polynomials for n=6 patterns".into(),
            domain_affinity: vec!["number_theory".into(), "algebra".into(), "pure_mathematics".into()],
            complementary: vec!["regulator_compute".into(), "integer_factoring".into()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tecs_math_lens_count() {
        let entries = tecs_math_lens_entries();
        assert_eq!(entries.len(), 103, "Must have exactly 103 TECS-L math lenses");
    }

    #[test]
    fn test_tecs_math_lens_names_unique() {
        let entries = tecs_math_lens_entries();
        let mut names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        let total = names.len();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), total, "All 103 TECS-L math lens names must be unique");
    }

    #[test]
    fn test_tecs_math_lens_all_extended() {
        let entries = tecs_math_lens_entries();
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
    fn test_tecs_math_lens_no_empty_fields() {
        let entries = tecs_math_lens_entries();
        for entry in &entries {
            assert!(!entry.name.is_empty(), "Lens name must not be empty");
            assert!(!entry.description.is_empty(), "Lens '{}' description must not be empty", entry.name);
            assert!(!entry.domain_affinity.is_empty(), "Lens '{}' must have at least one domain affinity", entry.name);
            assert!(!entry.complementary.is_empty(), "Lens '{}' must have at least one complementary lens", entry.name);
        }
    }
}
