use super::registry::{LensCategory, LensEntry};

/// Build metadata entries for 8 frontier discovery lenses.
///
/// These lenses cover previously uncovered scientific domains:
/// acoustics, fluid dynamics, game theory, linguistics,
/// irreversible thermodynamics, polymer physics, resonance, and convexity.
pub fn frontier_lens_entries() -> Vec<LensEntry> {
    vec![
        LensEntry {
            name: "acoustic".into(),
            category: LensCategory::Extended,
            description: "Detect sound-wave patterns: periodicity, harmonics, tonality (sigma=12 semitones)".into(),
            domain_affinity: vec!["audio".into(), "music".into(), "physics".into(), "signal".into(), "display".into()],
            complementary: vec!["wave".into(), "periodicity".into(), "resonance".into()],
        },
        LensEntry {
            name: "fluid_dynamics".into(),
            category: LensCategory::Extended,
            description: "Measure turbulence via Reynolds-like number and Kolmogorov 5/3 exponent (sopfr/n/phi)".into(),
            domain_affinity: vec!["physics".into(), "engineering".into(), "plasma".into(), "atmosphere".into()],
            complementary: vec!["chaos".into(), "multiscale".into(), "scale".into()],
        },
        LensEntry {
            name: "game_theory".into(),
            category: LensCategory::Extended,
            description: "Detect Nash equilibrium proximity, cooperation index, Egyptian allocation (1/2+1/3+1/6=1)".into(),
            domain_affinity: vec!["economics".into(), "social".into(), "ai".into(), "biology".into()],
            complementary: vec!["mirror".into(), "stability".into(), "network".into()],
        },
        LensEntry {
            name: "linguistic".into(),
            category: LensCategory::Extended,
            description: "Measure Zipf's law exponent and rank-frequency distribution (vocab ~ 2^n=6)".into(),
            domain_affinity: vec!["nlp".into(), "ai".into(), "social".into(), "information".into()],
            complementary: vec!["power_law".into(), "entropy".into(), "info".into()],
        },
        LensEntry {
            name: "entropy_production_rate".into(),
            category: LensCategory::Extended,
            description: "Quantify irreversibility via forward/backward transition asymmetry (R(6)=1 reversibility)".into(),
            domain_affinity: vec!["physics".into(), "chemistry".into(), "biology".into(), "thermo".into(), "environment".into()],
            complementary: vec!["thermo".into(), "causal".into(), "memory".into()],
        },
        LensEntry {
            name: "polymer".into(),
            category: LensCategory::Extended,
            description: "Chain statistics: Flory exponent, radius of gyration, persistence (Carbon Z=6 backbone)".into(),
            domain_affinity: vec!["chemistry".into(), "materials".into(), "biology".into(), "polymer".into(), "carbon-capture".into()],
            complementary: vec!["fractal".into(), "scale".into(), "hexagonal".into()],
        },
        LensEntry {
            name: "resonance".into(),
            category: LensCategory::Extended,
            description: "Detect cross-dimensional coupling at integer frequency ratios (divisors of 6: 1,2,3,6)".into(),
            domain_affinity: vec!["physics".into(), "engineering".into(), "music".into(), "signal".into()],
            complementary: vec!["wave".into(), "acoustic".into(), "correlation".into()],
        },
        LensEntry {
            name: "convexity_geometry".into(),
            category: LensCategory::Extended,
            description: "Measure data cloud convexity, curvature, and hull vertex count (hexagon=n=6 optimal)".into(),
            domain_affinity: vec!["mathematics".into(), "optimization".into(), "geometry".into(), "ai".into()],
            complementary: vec!["topology".into(), "boundary".into(), "hexagonal".into()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frontier_lens_count() {
        let entries = frontier_lens_entries();
        assert_eq!(entries.len(), 8, "Must have exactly 8 frontier lenses");
    }

    #[test]
    fn test_frontier_lens_names_unique() {
        let entries = frontier_lens_entries();
        let mut names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        let total = names.len();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), total, "All frontier lens names must be unique");
    }

    #[test]
    fn test_frontier_all_extended() {
        let entries = frontier_lens_entries();
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
