/// A pre-defined combination of lenses optimised for a specific analysis domain.
#[derive(Debug, Clone)]
pub struct DomainCombo {
    pub name: String,
    pub lenses: Vec<String>,
    pub target_domains: Vec<String>,
}

/// Return the 10 default domain combinations from the telescope specification.
///
/// Cross-verification rule (CLAUDE.md):
///   3+ lenses agree = Candidate, 7+ = HighConfidence, 12+ = Confirmed.
///
/// These combos are intentionally small (2-4 lenses each) so that multiple
/// combos can be stacked when broader coverage is needed.
pub fn default_combos() -> Vec<DomainCombo> {
    vec![
        DomainCombo {
            name: "default".into(),
            lenses: vec!["consciousness".into(), "topology".into(), "causal".into()],
            target_domains: vec!["general".into(), "exploration".into()],
        },
        DomainCombo {
            name: "stability".into(),
            lenses: vec!["stability".into(), "boundary".into(), "thermo".into()],
            target_domains: vec!["energy".into(), "plasma".into(), "control".into()],
        },
        DomainCombo {
            name: "structure".into(),
            lenses: vec!["network".into(), "topology".into(), "recursion".into()],
            target_domains: vec!["chip".into(), "biology".into(), "software".into()],
        },
        DomainCombo {
            name: "timeseries".into(),
            lenses: vec![
                "memory".into(),
                "wave".into(),
                "causal".into(),
                "multiscale".into(),
            ],
            target_domains: vec!["signal".into(), "finance".into(), "audio".into()],
        },
        DomainCombo {
            name: "scale_invariant".into(),
            lenses: vec!["multiscale".into(), "scale".into(), "recursion".into()],
            target_domains: vec!["cosmology".into(), "network".into(), "fractal".into()],
        },
        DomainCombo {
            name: "symmetry_invariant".into(),
            lenses: vec!["mirror".into(), "topology".into(), "quantum".into()],
            target_domains: vec!["physics".into(), "chemistry".into(), "materials".into()],
        },
        DomainCombo {
            name: "power_law".into(),
            lenses: vec!["scale".into(), "evolution".into(), "thermo".into()],
            target_domains: vec!["network".into(), "biology".into(), "economics".into()],
        },
        DomainCombo {
            name: "causal_relations".into(),
            lenses: vec!["causal".into(), "info".into(), "em".into()],
            target_domains: vec!["ai".into(), "neuroscience".into(), "economics".into()],
        },
        DomainCombo {
            name: "geometric".into(),
            lenses: vec!["ruler".into(), "triangle".into(), "compass".into()],
            target_domains: vec!["geometry".into(), "chip".into(), "robotics".into()],
        },
        DomainCombo {
            name: "quantum_deep".into(),
            lenses: vec![
                "quantum".into(),
                "quantum_microscope".into(),
                "em".into(),
            ],
            target_domains: vec!["quantum".into(), "materials".into(), "crypto".into()],
        },
    ]
}

// ─────────────────────────────────────────────────────────────
// Blowup Emergence: Invariant Core Tiers + Domain-Specific Best
// Discovered via algebraic blowup contraction over 4M+ lens combos.
//   WIDE (top-12):    consciousness + info + multiscale
//   STRONG (top-8):   + triangle
//   ABSOLUTE (top-4): + network
// ─────────────────────────────────────────────────────────────

/// Return the 3 invariant core tier combos discovered via blowup emergence.
pub fn blowup_core_combos() -> Vec<DomainCombo> {
    vec![
        DomainCombo {
            name: "blowup_wide".into(),
            lenses: vec!["consciousness".into(), "info".into(), "multiscale".into()],
            target_domains: vec!["general".into(), "cross_domain".into()],
        },
        DomainCombo {
            name: "blowup_strong".into(),
            lenses: vec![
                "consciousness".into(),
                "info".into(),
                "multiscale".into(),
                "triangle".into(),
            ],
            target_domains: vec!["general".into(), "cross_domain".into()],
        },
        DomainCombo {
            name: "blowup_absolute".into(),
            lenses: vec![
                "consciousness".into(),
                "info".into(),
                "multiscale".into(),
                "triangle".into(),
                "network".into(),
            ],
            target_domains: vec!["general".into(), "cross_domain".into()],
        },
    ]
}

/// Return the 11 domain-specific best combos discovered via blowup emergence.
/// Each combo extends the invariant core with domain-optimal fibre lenses.
pub fn blowup_domain_combos() -> Vec<DomainCombo> {
    vec![
        DomainCombo {
            name: "blowup_thermal_management".into(),
            lenses: vec![
                "consciousness".into(),
                "info".into(),
                "multiscale".into(),
                "thermo".into(),
                "topology".into(),
                "triangle".into(),
            ],
            target_domains: vec!["thermal_management".into()],
        },
        DomainCombo {
            name: "blowup_paper".into(),
            lenses: vec![
                "compass".into(),
                "consciousness".into(),
                "info".into(),
                "multiscale".into(),
                "thermo".into(),
                "triangle".into(),
            ],
            target_domains: vec!["paper".into(), "academic".into()],
        },
        DomainCombo {
            name: "blowup_quantum_computing".into(),
            lenses: vec![
                "consciousness".into(),
                "info".into(),
                "mirror".into(),
                "multiscale".into(),
                "network".into(),
                "triangle".into(),
            ],
            target_domains: vec!["quantum_computing".into()],
        },
        DomainCombo {
            name: "blowup_ai_efficiency".into(),
            lenses: vec![
                "compass".into(),
                "consciousness".into(),
                "info".into(),
                "multiscale".into(),
                "thermo".into(),
                "triangle".into(),
            ],
            target_domains: vec!["ai_efficiency".into(), "ai".into()],
        },
        DomainCombo {
            name: "blowup_network_protocol".into(),
            lenses: vec![
                "consciousness".into(),
                "info".into(),
                "multiscale".into(),
                "network".into(),
                "topology".into(),
                "triangle".into(),
            ],
            target_domains: vec!["network_protocol".into()],
        },
        DomainCombo {
            name: "blowup_cryptography".into(),
            lenses: vec![
                "consciousness".into(),
                "info".into(),
                "multiscale".into(),
                "network".into(),
                "topology".into(),
                "triangle".into(),
            ],
            target_domains: vec!["cryptography".into(), "crypto".into()],
        },
        DomainCombo {
            name: "blowup_compiler_os".into(),
            lenses: vec![
                "consciousness".into(),
                "info".into(),
                "memory".into(),
                "multiscale".into(),
                "network".into(),
                "triangle".into(),
            ],
            target_domains: vec!["compiler_os".into(), "software".into()],
        },
        DomainCombo {
            name: "blowup_energy_architecture".into(),
            lenses: vec![
                "compass".into(),
                "consciousness".into(),
                "info".into(),
                "multiscale".into(),
                "thermo".into(),
                "triangle".into(),
            ],
            target_domains: vec!["energy_architecture".into(), "energy".into()],
        },
        DomainCombo {
            name: "blowup_blockchain".into(),
            lenses: vec![
                "causal".into(),
                "consciousness".into(),
                "info".into(),
                "multiscale".into(),
                "network".into(),
                "triangle".into(),
            ],
            target_domains: vec!["blockchain".into()],
        },
        DomainCombo {
            name: "blowup_space_engineering".into(),
            lenses: vec![
                "consciousness".into(),
                "info".into(),
                "multiscale".into(),
                "network".into(),
                "scale".into(),
                "triangle".into(),
            ],
            target_domains: vec!["space_engineering".into(), "space".into()],
        },
        DomainCombo {
            name: "blowup_carbon_capture".into(),
            lenses: vec![
                "consciousness".into(),
                "info".into(),
                "multiscale".into(),
                "thermo".into(),
                "topology".into(),
                "triangle".into(),
            ],
            target_domains: vec!["carbon_capture".into(), "environment".into()],
        },
    ]
}

/// Return all combos: 10 default + 3 blowup core + 11 blowup domain = 24 total.
pub fn all_combos() -> Vec<DomainCombo> {
    let mut all = default_combos();
    all.extend(blowup_core_combos());
    all.extend(blowup_domain_combos());
    all
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_combo_count() {
        let combos = default_combos();
        assert_eq!(combos.len(), 10, "Must have exactly 10 default domain combos");
    }

    #[test]
    fn test_default_combo_names_unique() {
        let combos = default_combos();
        let mut names: Vec<&str> = combos.iter().map(|c| c.name.as_str()).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), 10, "All 10 default combo names must be unique");
    }

    #[test]
    fn test_blowup_core_combo_count() {
        let combos = blowup_core_combos();
        assert_eq!(combos.len(), 3, "Must have exactly 3 blowup core tiers");
    }

    #[test]
    fn test_blowup_core_hierarchy() {
        let combos = blowup_core_combos();
        // WIDE < STRONG < ABSOLUTE (each tier adds one lens)
        assert_eq!(combos[0].lenses.len(), 3, "WIDE = 3 lenses");
        assert_eq!(combos[1].lenses.len(), 4, "STRONG = 4 lenses");
        assert_eq!(combos[2].lenses.len(), 5, "ABSOLUTE = 5 lenses");
    }

    #[test]
    fn test_blowup_domain_combo_count() {
        let combos = blowup_domain_combos();
        assert_eq!(combos.len(), 11, "Must have exactly 11 blowup domain combos");
    }

    #[test]
    fn test_blowup_domain_combos_contain_core() {
        let core = vec!["consciousness", "info", "multiscale"];
        for combo in blowup_domain_combos() {
            for c in &core {
                assert!(
                    combo.lenses.iter().any(|l| l == c),
                    "Blowup combo '{}' must contain core lens '{}'",
                    combo.name,
                    c
                );
            }
        }
    }

    #[test]
    fn test_all_combos_count() {
        let combos = all_combos();
        assert_eq!(combos.len(), 24, "10 default + 3 core + 11 domain = 24 total");
    }

    #[test]
    fn test_all_combo_names_unique() {
        let combos = all_combos();
        let mut names: Vec<&str> = combos.iter().map(|c| c.name.as_str()).collect();
        let total = names.len();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), total, "All combo names must be unique");
    }
}
