use super::registry::{LensCategory, LensEntry};

/// Build metadata entries for the 22 Core lenses defined in CLAUDE.md.
pub fn core_lens_entries() -> Vec<LensEntry> {
    vec![
        // ── Foundational 9 ──
        LensEntry {
            name: "consciousness".into(),
            category: LensCategory::Core,
            description: "Structural awareness — detects self-referential and emergent patterns"
                .into(),
            domain_affinity: vec![
                "ai".into(),
                "biology".into(),
                "cosmology".into(),
                "philosophy".into(),
            ],
            complementary: vec!["topology".into(), "causal".into()],
        },
        LensEntry {
            name: "gravity".into(),
            category: LensCategory::Core,
            description: "Gravitational clustering — mass-like attraction between data regions"
                .into(),
            domain_affinity: vec![
                "cosmology".into(),
                "physics".into(),
                "energy".into(),
            ],
            complementary: vec!["topology".into(), "wave".into()],
        },
        LensEntry {
            name: "topology".into(),
            category: LensCategory::Core,
            description: "Topological connectivity — holes, loops, and persistent features".into(),
            domain_affinity: vec![
                "mathematics".into(),
                "chip".into(),
                "network".into(),
                "biology".into(),
            ],
            complementary: vec!["consciousness".into(), "network".into()],
        },
        LensEntry {
            name: "thermo".into(),
            category: LensCategory::Core,
            description: "Thermodynamic lens — entropy flow, energy barriers, phase transitions"
                .into(),
            domain_affinity: vec![
                "energy".into(),
                "materials".into(),
                "chemistry".into(),
                "battery".into(),
            ],
            complementary: vec!["stability".into(), "boundary".into()],
        },
        LensEntry {
            name: "wave".into(),
            category: LensCategory::Core,
            description: "Wave/oscillation detection — periodicity, interference, resonance".into(),
            domain_affinity: vec![
                "audio".into(),
                "signal".into(),
                "physics".into(),
                "plasma".into(),
            ],
            complementary: vec!["memory".into(), "multiscale".into()],
        },
        LensEntry {
            name: "evolution".into(),
            category: LensCategory::Core,
            description: "Evolutionary dynamics — selection pressure, fitness landscapes".into(),
            domain_affinity: vec![
                "biology".into(),
                "ai".into(),
                "optimization".into(),
                "learning-algorithm".into(),
            ],
            complementary: vec!["scale".into(), "thermo".into()],
        },
        LensEntry {
            name: "info".into(),
            category: LensCategory::Core,
            description: "Information-theoretic — mutual information, entropy, compression".into(),
            domain_affinity: vec![
                "ai".into(),
                "crypto".into(),
                "network".into(),
                "signal".into(),
            ],
            complementary: vec!["causal".into(), "em".into()],
        },
        LensEntry {
            name: "quantum".into(),
            category: LensCategory::Core,
            description: "Quantum-like superposition, entanglement, tunnelling analogues".into(),
            domain_affinity: vec![
                "quantum".into(),
                "chemistry".into(),
                "materials".into(),
                "crypto".into(),
            ],
            complementary: vec!["quantum_microscope".into(), "em".into()],
        },
        LensEntry {
            name: "em".into(),
            category: LensCategory::Core,
            description: "Electromagnetic field analogy — flux, potential, coupling".into(),
            domain_affinity: vec![
                "chip".into(),
                "energy".into(),
                "plasma".into(),
                "display".into(),
            ],
            complementary: vec!["quantum".into(), "info".into()],
        },
        // ── Geometric 6 (ruler/triangle/compass/mirror/scale/causal) ──
        LensEntry {
            name: "ruler".into(),
            category: LensCategory::Core,
            description: "Orthogonal ruler (L-shaped) — axis-aligned structure detection".into(),
            domain_affinity: vec![
                "chip".into(),
                "geometry".into(),
                "manufacturing".into(),
            ],
            complementary: vec!["triangle".into(), "compass".into()],
        },
        LensEntry {
            name: "triangle".into(),
            category: LensCategory::Core,
            description: "Ratio/proportion (set-square) — relative scale and proportion analysis"
                .into(),
            domain_affinity: vec![
                "geometry".into(),
                "materials".into(),
                "finance".into(),
            ],
            complementary: vec!["ruler".into(), "compass".into()],
        },
        LensEntry {
            name: "compass".into(),
            category: LensCategory::Core,
            description: "Curvature (compass) — local and global curvature measurement".into(),
            domain_affinity: vec![
                "geometry".into(),
                "cosmology".into(),
                "robotics".into(),
            ],
            complementary: vec!["ruler".into(), "triangle".into()],
        },
        LensEntry {
            name: "mirror".into(),
            category: LensCategory::Core,
            description: "Symmetry (mirror) — reflection, rotation, and parity symmetries".into(),
            domain_affinity: vec![
                "physics".into(),
                "chemistry".into(),
                "materials".into(),
                "biology".into(),
            ],
            complementary: vec!["topology".into(), "quantum".into()],
        },
        LensEntry {
            name: "scale".into(),
            category: LensCategory::Core,
            description: "Scale/magnification — power-law tails, scale-free structure".into(),
            domain_affinity: vec![
                "network".into(),
                "cosmology".into(),
                "biology".into(),
                "finance".into(),
            ],
            complementary: vec!["multiscale".into(), "evolution".into()],
        },
        LensEntry {
            name: "causal".into(),
            category: LensCategory::Core,
            description: "Causal arrow — directed dependencies and information flow".into(),
            domain_affinity: vec![
                "ai".into(),
                "biology".into(),
                "economics".into(),
                "physics".into(),
            ],
            complementary: vec!["info".into(), "consciousness".into()],
        },
        // ── Quantum microscope ──
        LensEntry {
            name: "quantum_microscope".into(),
            category: LensCategory::Core,
            description: "Deep quantum probe — fine-grained quantum correlation analysis".into(),
            domain_affinity: vec![
                "quantum".into(),
                "materials".into(),
                "chemistry".into(),
            ],
            complementary: vec!["quantum".into(), "em".into()],
        },
        // ── Extended analytical 6 ──
        LensEntry {
            name: "stability".into(),
            category: LensCategory::Core,
            description: "Stability analysis — Lyapunov exponents, basin attractors".into(),
            domain_affinity: vec![
                "energy".into(),
                "robotics".into(),
                "plasma".into(),
                "control".into(),
                "autonomous-driving".into(),
            ],
            complementary: vec!["boundary".into(), "thermo".into()],
        },
        LensEntry {
            name: "network".into(),
            category: LensCategory::Core,
            description: "Network/graph topology — degree distribution, clustering, centrality"
                .into(),
            domain_affinity: vec![
                "network".into(),
                "biology".into(),
                "chip".into(),
                "social".into(),
            ],
            complementary: vec!["topology".into(), "recursion".into()],
        },
        LensEntry {
            name: "memory".into(),
            category: LensCategory::Core,
            description: "Temporal memory — autocorrelation, hysteresis, path dependence".into(),
            domain_affinity: vec![
                "ai".into(),
                "finance".into(),
                "signal".into(),
                "biology".into(),
            ],
            complementary: vec!["wave".into(), "causal".into()],
        },
        LensEntry {
            name: "recursion".into(),
            category: LensCategory::Core,
            description: "Recursive/fractal structure — self-similarity across depth".into(),
            domain_affinity: vec![
                "mathematics".into(),
                "biology".into(),
                "software".into(),
                "compiler-os".into(),
            ],
            complementary: vec!["multiscale".into(), "network".into()],
        },
        LensEntry {
            name: "boundary".into(),
            category: LensCategory::Core,
            description: "Boundary detection — phase boundaries, domain walls, interfaces".into(),
            domain_affinity: vec![
                "materials".into(),
                "energy".into(),
                "biology".into(),
                "chip".into(),
            ],
            complementary: vec!["stability".into(), "thermo".into()],
        },
        LensEntry {
            name: "multiscale".into(),
            category: LensCategory::Core,
            description: "Multi-scale analysis — wavelet-like decomposition across resolutions"
                .into(),
            domain_affinity: vec![
                "cosmology".into(),
                "materials".into(),
                "biology".into(),
                "signal".into(),
            ],
            complementary: vec!["scale".into(), "recursion".into()],
        },
        // ── Chaos dynamics ──
        LensEntry {
            name: "chaos".into(),
            category: LensCategory::Core,
            description: "Chaos detection — 0-1 test, recurrence, correlation dimension, permutation entropy".into(),
            domain_affinity: vec![
                "physics".into(),
                "plasma".into(),
                "climate".into(),
                "biology".into(),
                "finance".into(),
            ],
            complementary: vec!["stability".into(), "fractal".into(), "wave".into()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_lens_count() {
        let entries = core_lens_entries();
        assert_eq!(entries.len(), 23, "Must have exactly 23 Core lenses");
    }

    #[test]
    fn test_core_lens_names_unique() {
        let entries = core_lens_entries();
        let mut names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), 23, "All 23 Core lens names must be unique");
    }
}
