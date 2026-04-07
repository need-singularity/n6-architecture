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

/// Build metadata entries for 8 new-domain lenses (todo#16).
///
/// Domains: morphology, tonal harmony, ecological niche, macroeconomics,
/// immunogenetics, phonetics, food web, behavioral economics.
pub fn new_domain_lens_entries() -> Vec<LensEntry> {
    vec![
        LensEntry {
            name: "morphology".into(),
            category: LensCategory::Extended,
            description: "Detect morpheme distribution: 6 POS classes, tau=4 inflection, sigma=12 phoneme patterns (BT-112,BT-215)".into(),
            domain_affinity: vec!["linguistics".into(), "nlp".into(), "ai".into(), "social".into()],
            complementary: vec!["linguistic".into(), "entropy".into(), "clustering".into()],
        },
        LensEntry {
            name: "tonal_harmony".into(),
            category: LensCategory::Extended,
            description: "Measure hexatonic scale (6-note), triad tau=4 inversions, circle-of-fifths cycle 24 (BT-108,BT-135)".into(),
            domain_affinity: vec!["music".into(), "audio".into(), "signal".into(), "physics".into()],
            complementary: vec!["music_harmony".into(), "acoustic".into(), "resonance".into()],
        },
        LensEntry {
            name: "ecological_niche".into(),
            category: LensCategory::Extended,
            description: "Compute Simpson diversity and 6-tier trophic hierarchy alignment (BT-178,BT-201)".into(),
            domain_affinity: vec!["ecology".into(), "biology".into(), "environment".into(), "climate".into()],
            complementary: vec!["evolution".into(), "network".into(), "stability".into()],
        },
        LensEntry {
            name: "macroeconomics".into(),
            category: LensCategory::Extended,
            description: "Detect 6-phase business cycle, 4-quarter periodicity, sigma*phi=288 attractor (BT-143,BT-189)".into(),
            domain_affinity: vec!["economics".into(), "finance".into(), "social".into(), "energy".into()],
            complementary: vec!["game_theory".into(), "periodicity".into(), "phase_transition".into()],
        },
        LensEntry {
            name: "immunogenetics".into(),
            category: LensCategory::Extended,
            description: "Measure 6 Ig class diversity, HLA 6-locus polymorphism, MHC-II 6 subunit proximity (BT-155,BT-194,BT-220)".into(),
            domain_affinity: vec!["immunology".into(), "biology".into(), "medicine".into(), "genetics".into()],
            complementary: vec!["immune_response".into(), "genome_coding".into(), "evolution".into()],
        },
        LensEntry {
            name: "phonetics".into(),
            category: LensCategory::Extended,
            description: "Detect 6 articulation positions, F1/F2 formant ratios, sigma=12 vowel formant pairs (BT-112,BT-137)".into(),
            domain_affinity: vec!["linguistics".into(), "audio".into(), "nlp".into(), "signal".into()],
            complementary: vec!["morphology".into(), "acoustic".into(), "spectral".into()],
        },
        LensEntry {
            name: "food_web".into(),
            category: LensCategory::Extended,
            description: "Compute 6-tier biomass pyramid, sigma/tau=3 predator-prey ratio, energy transfer efficiency (BT-178,BT-201,BT-233)".into(),
            domain_affinity: vec!["ecology".into(), "biology".into(), "environment".into(), "ocean".into()],
            complementary: vec!["ecological_niche".into(), "network".into(), "graph".into()],
        },
        LensEntry {
            name: "behavioral_economics".into(),
            category: LensCategory::Extended,
            description: "Detect 6 cognitive bias clusters, phi=2 loss aversion, n=6 choice overload threshold (BT-143,BT-189,BT-212)".into(),
            domain_affinity: vec!["economics".into(), "psychology".into(), "social".into(), "ai".into()],
            complementary: vec!["macroeconomics".into(), "game_theory".into(), "decision".into()],
        },
    ]
}

#[cfg(test)]
mod new_domain_tests {
    use super::*;

    #[test]
    fn test_new_domain_lens_count() {
        let entries = new_domain_lens_entries();
        assert_eq!(entries.len(), 8, "Must have exactly 8 new-domain lenses");
    }

    #[test]
    fn test_new_domain_lens_names_unique() {
        let entries = new_domain_lens_entries();
        let mut names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        let total = names.len();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), total, "All new-domain lens names must be unique");
    }

    #[test]
    fn test_new_domain_all_extended() {
        for entry in new_domain_lens_entries() {
            assert_eq!(
                entry.category,
                LensCategory::Extended,
                "Lens '{}' should be Extended category",
                entry.name
            );
        }
    }

    #[test]
    fn test_new_domain_no_empty_fields() {
        for entry in new_domain_lens_entries() {
            assert!(!entry.name.is_empty());
            assert!(!entry.description.is_empty(), "Lens '{}' needs description", entry.name);
            assert!(!entry.domain_affinity.is_empty(), "Lens '{}' needs domain affinity", entry.name);
            assert!(!entry.complementary.is_empty(), "Lens '{}' needs complementary", entry.name);
        }
    }
}
