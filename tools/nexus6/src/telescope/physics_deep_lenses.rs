use super::registry::{LensCategory, LensEntry};

/// Build metadata entries for the 49 physics-deep / biophysics / cosmology lenses.
///
/// Organized into 9 groups: electromagnetic deep (6), thermal/entropy deep (5),
/// fundamental forces (6), optics deep (5), acoustics (5), fluid deep (5),
/// electrical circuits (5), biophysics (6), and cosmology (6).
pub fn physics_deep_lens_entries() -> Vec<LensEntry> {
    vec![
        // ── Electromagnetic Deep (6) ──
        LensEntry {
            name: "electromagnetic_wave".into(),
            category: LensCategory::Extended,
            description: "Analyze EM wave propagation, polarization, and interference patterns".into(),
            domain_affinity: vec!["physics".into(), "optics".into(), "communications".into()],
            complementary: vec!["wave".into(), "em".into(), "laser_coherence".into()],
        },
        LensEntry {
            name: "maxwell_equation".into(),
            category: LensCategory::Extended,
            description: "Detect curl/divergence field structure consistent with Maxwell equations".into(),
            domain_affinity: vec!["physics".into(), "electromagnetism".into(), "engineering".into()],
            complementary: vec!["em".into(), "faraday_induction".into(), "ampere_law".into()],
        },
        LensEntry {
            name: "faraday_induction".into(),
            category: LensCategory::Extended,
            description: "Identify time-varying magnetic flux inducing electric response".into(),
            domain_affinity: vec!["energy".into(), "electromagnetism".into(), "motors".into()],
            complementary: vec!["maxwell_equation".into(), "em".into()],
        },
        LensEntry {
            name: "ampere_law".into(),
            category: LensCategory::Extended,
            description: "Detect current-loop magnetic field generation and circulation patterns".into(),
            domain_affinity: vec!["electromagnetism".into(), "circuits".into(), "physics".into()],
            complementary: vec!["maxwell_equation".into(), "faraday_induction".into()],
        },
        LensEntry {
            name: "magnetic_monopole".into(),
            category: LensCategory::Extended,
            description: "Search for isolated magnetic charge signatures or topological defects".into(),
            domain_affinity: vec!["particle_physics".into(), "topology".into(), "cosmology".into()],
            complementary: vec!["topology".into(), "quantum".into(), "dark_matter_halo".into()],
        },
        LensEntry {
            name: "superconductor_lens".into(),
            category: LensCategory::Extended,
            description: "Detect zero-resistance phase transitions and Meissner-effect signatures".into(),
            domain_affinity: vec!["superconductor".into(), "materials".into(), "quantum".into()],
            complementary: vec!["quantum".into(), "thermo".into(), "ion_channel".into()],
        },

        // ── Thermal / Entropy Deep (5) ──
        LensEntry {
            name: "heat_conduction".into(),
            category: LensCategory::Extended,
            description: "Analyze Fourier-law heat diffusion and thermal gradient patterns".into(),
            domain_affinity: vec!["thermal".into(), "materials".into(), "chip".into()],
            complementary: vec!["thermo".into(), "convection_pattern".into()],
        },
        LensEntry {
            name: "convection_pattern".into(),
            category: LensCategory::Extended,
            description: "Identify Rayleigh-Benard and natural/forced convection cell structures".into(),
            domain_affinity: vec!["thermal".into(), "atmosphere".into(), "fluid".into()],
            complementary: vec!["heat_conduction".into(), "turbulent_flow_lens".into()],
        },
        LensEntry {
            name: "radiation_spectrum".into(),
            category: LensCategory::Extended,
            description: "Detect blackbody and emission/absorption spectral signatures".into(),
            domain_affinity: vec!["astrophysics".into(), "thermal".into(), "optics".into()],
            complementary: vec!["wave".into(), "thermo".into(), "neutron_star_lens".into()],
        },
        LensEntry {
            name: "entropy_production".into(),
            category: LensCategory::Extended,
            description: "Measure irreversible entropy generation and dissipative structure formation".into(),
            domain_affinity: vec!["thermodynamics".into(), "chemistry".into(), "biology".into()],
            complementary: vec!["thermo".into(), "info".into(), "maxwell_demon".into()],
        },
        LensEntry {
            name: "maxwell_demon".into(),
            category: LensCategory::Extended,
            description: "Detect information-entropy coupling and apparent entropy-reduction mechanisms".into(),
            domain_affinity: vec!["information_theory".into(), "thermodynamics".into(), "quantum".into()],
            complementary: vec!["entropy_production".into(), "info".into(), "quantum".into()],
        },

        // ── Fundamental Forces (6) ──
        LensEntry {
            name: "gravitational_force_lens".into(),
            category: LensCategory::Extended,
            description: "Analyze gravitational attraction, tidal effects, and geodesic curvature".into(),
            domain_affinity: vec!["gravity".into(), "cosmology".into(), "astrophysics".into()],
            complementary: vec!["gravity".into(), "galaxy_rotation".into()],
        },
        LensEntry {
            name: "electromagnetic_force_lens".into(),
            category: LensCategory::Extended,
            description: "Detect Coulomb and Lorentz force interactions in charged systems".into(),
            domain_affinity: vec!["electromagnetism".into(), "chemistry".into(), "plasma".into()],
            complementary: vec!["em".into(), "maxwell_equation".into()],
        },
        LensEntry {
            name: "strong_nuclear".into(),
            category: LensCategory::Extended,
            description: "Identify QCD color-confinement and binding-energy signatures in nuclear data".into(),
            domain_affinity: vec!["particle_physics".into(), "nuclear".into(), "fusion".into()],
            complementary: vec!["quantum".into(), "weak_nuclear".into()],
        },
        LensEntry {
            name: "weak_nuclear".into(),
            category: LensCategory::Extended,
            description: "Detect beta-decay, flavor-changing, and parity-violation signatures".into(),
            domain_affinity: vec!["particle_physics".into(), "nuclear".into(), "cosmology".into()],
            complementary: vec!["quantum".into(), "strong_nuclear".into()],
        },
        LensEntry {
            name: "centrifugal_effect".into(),
            category: LensCategory::Extended,
            description: "Identify rotating-frame pseudo-force patterns and radial stratification".into(),
            domain_affinity: vec!["mechanics".into(), "astrophysics".into(), "engineering".into()],
            complementary: vec!["coriolis_effect".into(), "gravity".into()],
        },
        LensEntry {
            name: "coriolis_effect".into(),
            category: LensCategory::Extended,
            description: "Detect rotational deflection patterns in large-scale fluid and particle flows".into(),
            domain_affinity: vec!["atmosphere".into(), "ocean".into(), "astrophysics".into()],
            complementary: vec!["centrifugal_effect".into(), "turbulent_flow_lens".into()],
        },

        // ── Optics Deep (5) ──
        LensEntry {
            name: "laser_coherence".into(),
            category: LensCategory::Extended,
            description: "Measure temporal and spatial coherence, stimulated emission signatures".into(),
            domain_affinity: vec!["optics".into(), "photonics".into(), "communications".into()],
            complementary: vec!["electromagnetic_wave".into(), "hologram".into()],
        },
        LensEntry {
            name: "hologram".into(),
            category: LensCategory::Extended,
            description: "Detect phase-encoded volumetric information and holographic reconstruction".into(),
            domain_affinity: vec!["optics".into(), "display".into(), "information_theory".into()],
            complementary: vec!["laser_coherence".into(), "wave".into()],
        },
        LensEntry {
            name: "optical_fiber_lens".into(),
            category: LensCategory::Extended,
            description: "Analyze total internal reflection waveguide modes and dispersion".into(),
            domain_affinity: vec!["photonics".into(), "communications".into(), "network".into()],
            complementary: vec!["electromagnetic_wave".into(), "metamaterial".into()],
        },
        LensEntry {
            name: "metamaterial".into(),
            category: LensCategory::Extended,
            description: "Detect engineered sub-wavelength structures producing exotic EM responses".into(),
            domain_affinity: vec!["materials".into(), "optics".into(), "acoustics".into()],
            complementary: vec!["plasmon_resonance".into(), "acoustic_metamaterial".into()],
        },
        LensEntry {
            name: "plasmon_resonance".into(),
            category: LensCategory::Extended,
            description: "Identify surface plasmon polariton coupling and near-field enhancement".into(),
            domain_affinity: vec!["nanophotonics".into(), "materials".into(), "biosensing".into()],
            complementary: vec!["metamaterial".into(), "electromagnetic_wave".into()],
        },

        // ── Acoustics (5) ──
        LensEntry {
            name: "acoustic_resonance".into(),
            category: LensCategory::Extended,
            description: "Detect standing-wave modes, natural frequencies, and harmonic series".into(),
            domain_affinity: vec!["acoustics".into(), "music".into(), "engineering".into()],
            complementary: vec!["wave".into(), "resonant_circuit".into()],
        },
        LensEntry {
            name: "ultrasound_lens".into(),
            category: LensCategory::Extended,
            description: "Analyze high-frequency acoustic propagation, imaging, and attenuation".into(),
            domain_affinity: vec!["medical".into(), "ndt".into(), "acoustics".into()],
            complementary: vec!["acoustic_resonance".into(), "wave".into()],
        },
        LensEntry {
            name: "infrasound".into(),
            category: LensCategory::Extended,
            description: "Detect sub-20Hz acoustic signatures from geophysical and atmospheric sources".into(),
            domain_affinity: vec!["geophysics".into(), "atmosphere".into(), "seismology".into()],
            complementary: vec!["acoustic_resonance".into(), "wave".into()],
        },
        LensEntry {
            name: "phononic_crystal_lens".into(),
            category: LensCategory::Extended,
            description: "Identify periodic elastic structures with phonon bandgaps and wave steering".into(),
            domain_affinity: vec!["materials".into(), "acoustics".into(), "thermal".into()],
            complementary: vec!["metamaterial".into(), "acoustic_metamaterial".into()],
        },
        LensEntry {
            name: "acoustic_metamaterial".into(),
            category: LensCategory::Extended,
            description: "Detect engineered acoustic structures with negative bulk modulus or cloaking".into(),
            domain_affinity: vec!["acoustics".into(), "materials".into(), "engineering".into()],
            complementary: vec!["metamaterial".into(), "phononic_crystal_lens".into()],
        },

        // ── Fluid Deep (5) ──
        LensEntry {
            name: "laminar_flow".into(),
            category: LensCategory::Extended,
            description: "Identify smooth, layered fluid motion below critical Reynolds number".into(),
            domain_affinity: vec!["fluid".into(), "microfluidics".into(), "biology".into()],
            complementary: vec!["turbulent_flow_lens".into(), "viscosity_gradient".into()],
        },
        LensEntry {
            name: "turbulent_flow_lens".into(),
            category: LensCategory::Extended,
            description: "Detect chaotic vortex cascade, Kolmogorov scaling, and energy dissipation".into(),
            domain_affinity: vec!["fluid".into(), "atmosphere".into(), "plasma".into()],
            complementary: vec!["laminar_flow".into(), "bernoulli_principle".into()],
        },
        LensEntry {
            name: "bernoulli_principle".into(),
            category: LensCategory::Extended,
            description: "Detect velocity-pressure trade-off in steady incompressible flow".into(),
            domain_affinity: vec!["fluid".into(), "aerodynamics".into(), "engineering".into()],
            complementary: vec!["laminar_flow".into(), "turbulent_flow_lens".into()],
        },
        LensEntry {
            name: "viscosity_gradient".into(),
            category: LensCategory::Extended,
            description: "Analyze shear stress and viscosity stratification in fluid layers".into(),
            domain_affinity: vec!["fluid".into(), "materials".into(), "geology".into()],
            complementary: vec!["laminar_flow".into(), "surface_tension_lens".into()],
        },
        LensEntry {
            name: "surface_tension_lens".into(),
            category: LensCategory::Extended,
            description: "Detect capillary, Marangoni, and wetting phenomena at fluid interfaces".into(),
            domain_affinity: vec!["fluid".into(), "microfluidics".into(), "biology".into()],
            complementary: vec!["viscosity_gradient".into(), "membrane_transport".into()],
        },

        // ── Electrical Circuits (5) ──
        LensEntry {
            name: "resonant_circuit".into(),
            category: LensCategory::Extended,
            description: "Detect LC/RLC resonance peaks, Q-factor, and bandwidth selectivity".into(),
            domain_affinity: vec!["electronics".into(), "rf".into(), "communications".into()],
            complementary: vec!["acoustic_resonance".into(), "filter_response".into()],
        },
        LensEntry {
            name: "filter_response".into(),
            category: LensCategory::Extended,
            description: "Analyze frequency-domain transfer functions: lowpass, highpass, bandpass".into(),
            domain_affinity: vec!["electronics".into(), "signal_processing".into(), "audio".into()],
            complementary: vec!["resonant_circuit".into(), "wave".into()],
        },
        LensEntry {
            name: "amplification_lens".into(),
            category: LensCategory::Extended,
            description: "Detect gain stages, transistor operating regions, and signal amplification".into(),
            domain_affinity: vec!["electronics".into(), "rf".into(), "chip".into()],
            complementary: vec!["feedback_network".into(), "oscillator_lens".into()],
        },
        LensEntry {
            name: "oscillator_lens".into(),
            category: LensCategory::Extended,
            description: "Identify self-sustaining oscillation via positive feedback and limit cycles".into(),
            domain_affinity: vec!["electronics".into(), "physics".into(), "biology".into()],
            complementary: vec!["amplification_lens".into(), "feedback_network".into()],
        },
        LensEntry {
            name: "feedback_network".into(),
            category: LensCategory::Extended,
            description: "Analyze positive/negative feedback loops and stability criteria (Nyquist, Bode)".into(),
            domain_affinity: vec!["control".into(), "electronics".into(), "biology".into()],
            complementary: vec!["oscillator_lens".into(), "amplification_lens".into()],
        },

        // ── Biophysics (6) ──
        LensEntry {
            name: "dna_helix".into(),
            category: LensCategory::Extended,
            description: "Detect double-helix base-pair encoding, replication fork, and codon patterns".into(),
            domain_affinity: vec!["biology".into(), "genetics".into(), "medicine".into()],
            complementary: vec!["protein_folding".into(), "info".into()],
        },
        LensEntry {
            name: "protein_folding".into(),
            category: LensCategory::Extended,
            description: "Analyze amino-acid sequence to 3D structure energy landscape and folding pathways".into(),
            domain_affinity: vec!["biology".into(), "chemistry".into(), "medicine".into()],
            complementary: vec!["dna_helix".into(), "enzyme_kinetics".into()],
        },
        LensEntry {
            name: "enzyme_kinetics".into(),
            category: LensCategory::Extended,
            description: "Detect Michaelis-Menten saturation, catalytic turnover, and inhibition".into(),
            domain_affinity: vec!["biochemistry".into(), "medicine".into(), "biology".into()],
            complementary: vec!["protein_folding".into(), "membrane_transport".into()],
        },
        LensEntry {
            name: "membrane_transport".into(),
            category: LensCategory::Extended,
            description: "Analyze selective permeability, osmosis, and active/passive transport".into(),
            domain_affinity: vec!["biology".into(), "medicine".into(), "materials".into()],
            complementary: vec!["ion_channel".into(), "surface_tension_lens".into()],
        },
        LensEntry {
            name: "ion_channel".into(),
            category: LensCategory::Extended,
            description: "Detect gated ion conductance, Nernst potential, and action potential generation".into(),
            domain_affinity: vec!["neuroscience".into(), "biology".into(), "medicine".into()],
            complementary: vec!["membrane_transport".into(), "neural_synapse".into()],
        },
        LensEntry {
            name: "neural_synapse".into(),
            category: LensCategory::Extended,
            description: "Analyze synaptic transmission, neurotransmitter release, and plasticity".into(),
            domain_affinity: vec!["neuroscience".into(), "ai".into(), "medicine".into()],
            complementary: vec!["ion_channel".into(), "network".into()],
        },

        // ── Cosmology (6) ──
        LensEntry {
            name: "galaxy_rotation".into(),
            category: LensCategory::Extended,
            description: "Detect flat rotation curves implying dark matter halo contributions".into(),
            domain_affinity: vec!["cosmology".into(), "astrophysics".into(), "gravity".into()],
            complementary: vec!["dark_matter_halo".into(), "gravitational_force_lens".into()],
        },
        LensEntry {
            name: "cosmic_web".into(),
            category: LensCategory::Extended,
            description: "Identify large-scale filament-void-node structure of matter distribution".into(),
            domain_affinity: vec!["cosmology".into(), "network".into(), "topology".into()],
            complementary: vec!["galaxy_rotation".into(), "dark_matter_halo".into()],
        },
        LensEntry {
            name: "dark_matter_halo".into(),
            category: LensCategory::Extended,
            description: "Detect gravitational lensing and velocity dispersion from unseen mass".into(),
            domain_affinity: vec!["cosmology".into(), "particle_physics".into(), "gravity".into()],
            complementary: vec!["galaxy_rotation".into(), "cosmic_web".into()],
        },
        LensEntry {
            name: "gravitational_wave_lens".into(),
            category: LensCategory::Extended,
            description: "Identify spacetime ripple signatures from compact binary mergers".into(),
            domain_affinity: vec!["gravity".into(), "astrophysics".into(), "cosmology".into()],
            complementary: vec!["neutron_star_lens".into(), "gravitational_force_lens".into()],
        },
        LensEntry {
            name: "neutron_star_lens".into(),
            category: LensCategory::Extended,
            description: "Detect ultra-dense matter EOS signatures, pulsar timing, and magnetar flares".into(),
            domain_affinity: vec!["astrophysics".into(), "nuclear".into(), "gravity".into()],
            complementary: vec!["white_dwarf".into(), "gravitational_wave_lens".into()],
        },
        LensEntry {
            name: "white_dwarf".into(),
            category: LensCategory::Extended,
            description: "Analyze electron-degenerate remnant cooling curves and Chandrasekhar limit".into(),
            domain_affinity: vec!["astrophysics".into(), "thermodynamics".into(), "quantum".into()],
            complementary: vec!["neutron_star_lens".into(), "radiation_spectrum".into()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn count_is_49() {
        assert_eq!(physics_deep_lens_entries().len(), 49);
    }

    #[test]
    fn names_are_unique() {
        let entries = physics_deep_lens_entries();
        let names: HashSet<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names.len(), entries.len(), "duplicate lens names detected");
    }

    #[test]
    fn all_category_extended() {
        for entry in physics_deep_lens_entries() {
            assert_eq!(
                entry.category,
                LensCategory::Extended,
                "lens {} should be Extended",
                entry.name
            );
        }
    }

    #[test]
    fn descriptions_non_empty() {
        for entry in physics_deep_lens_entries() {
            assert!(
                !entry.description.is_empty(),
                "lens {} has empty description",
                entry.name
            );
            assert!(
                !entry.domain_affinity.is_empty(),
                "lens {} has no domain affinity",
                entry.name
            );
            assert!(
                !entry.complementary.is_empty(),
                "lens {} has no complementary lenses",
                entry.name
            );
        }
    }
}
