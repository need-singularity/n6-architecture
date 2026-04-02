use super::registry::{LensCategory, LensEntry};

/// Build metadata entries for the 100 SEDI signal-discovery lenses.
/// Organized into 10 sub-groups of 10 lenses each.
pub fn sedi_signal_lens_entries() -> Vec<LensEntry> {
    vec![
        // ══════════════════════════════════════════════
        // Signal Detection (10)
        // ══════════════════════════════════════════════
        LensEntry {
            name: "matched_filter".into(),
            category: LensCategory::Extended,
            description: "Optimal SNR extraction by correlating data with known template waveforms".into(),
            domain_affinity: vec!["signal_processing".into(), "gravitational_waves".into(), "radar".into()],
            complementary: vec!["whitening".into(), "coherent_snr".into(), "template_bank".into()],
        },
        LensEntry {
            name: "whitening".into(),
            category: LensCategory::Extended,
            description: "Spectral flattening to equalize noise power across frequency bands".into(),
            domain_affinity: vec!["signal_processing".into(), "gravitational_waves".into(), "audio".into()],
            complementary: vec!["matched_filter".into(), "spectral_line_comb".into(), "noise_stationarity".into()],
        },
        LensEntry {
            name: "glitch_classifier".into(),
            category: LensCategory::Extended,
            description: "Categorize non-Gaussian transient noise artifacts by morphology".into(),
            domain_affinity: vec!["signal_processing".into(), "gravitational_waves".into(), "data_quality".into()],
            complementary: vec!["veto_channel".into(), "data_quality_flag".into(), "outlier_robust".into()],
        },
        LensEntry {
            name: "excess_power".into(),
            category: LensCategory::Extended,
            description: "Detect unmodeled transient signals via excess energy in time-frequency tiles".into(),
            domain_affinity: vec!["signal_processing".into(), "gravitational_waves".into(), "seismology".into()],
            complementary: vec!["wavelet_scalogram".into(), "coherent_snr".into(), "clustering_trigger".into()],
        },
        LensEntry {
            name: "coherent_snr".into(),
            category: LensCategory::Extended,
            description: "Combine multiple detector outputs coherently to boost signal-to-noise ratio".into(),
            domain_affinity: vec!["signal_processing".into(), "gravitational_waves".into(), "interferometry".into()],
            complementary: vec!["matched_filter".into(), "null_stream".into(), "baseline_synthesis".into()],
        },
        LensEntry {
            name: "injection_recovery".into(),
            category: LensCategory::Extended,
            description: "Validate pipeline sensitivity by injecting and recovering synthetic signals".into(),
            domain_affinity: vec!["signal_processing".into(), "calibration".into(), "gravitational_waves".into()],
            complementary: vec!["blind_analysis".into(), "sensitivity_curve".into(), "calibration_uncertainty".into()],
        },
        LensEntry {
            name: "veto_channel".into(),
            category: LensCategory::Extended,
            description: "Reject false triggers using auxiliary environmental and instrumental channels".into(),
            domain_affinity: vec!["signal_processing".into(), "data_quality".into(), "gravitational_waves".into()],
            complementary: vec!["glitch_classifier".into(), "environment_monitor".into(), "data_quality_flag".into()],
        },
        LensEntry {
            name: "spectral_line_comb".into(),
            category: LensCategory::Extended,
            description: "Identify and excise periodic spectral lines from instrumental artifacts".into(),
            domain_affinity: vec!["signal_processing".into(), "spectroscopy".into(), "gravitational_waves".into()],
            complementary: vec!["whitening".into(), "cyclostationary".into(), "calibration_uncertainty".into()],
        },
        LensEntry {
            name: "stochastic_background".into(),
            category: LensCategory::Extended,
            description: "Search for diffuse isotropic signal from superposition of unresolved sources".into(),
            domain_affinity: vec!["cosmology".into(), "gravitational_waves".into(), "signal_processing".into()],
            complementary: vec!["cross_spectral".into(), "angular_power".into(), "primordial_spectrum".into()],
        },
        LensEntry {
            name: "null_stream".into(),
            category: LensCategory::Extended,
            description: "Construct signal-free combination of detector outputs for noise characterization".into(),
            domain_affinity: vec!["signal_processing".into(), "gravitational_waves".into(), "interferometry".into()],
            complementary: vec!["coherent_snr".into(), "noise_stationarity".into(), "veto_channel".into()],
        },

        // ══════════════════════════════════════════════
        // Cross-correlation (10)
        // ══════════════════════════════════════════════
        LensEntry {
            name: "cross_spectral".into(),
            category: LensCategory::Extended,
            description: "Measure frequency-domain correlation between two data streams".into(),
            domain_affinity: vec!["signal_processing".into(), "gravitational_waves".into(), "seismology".into()],
            complementary: vec!["stochastic_background".into(), "transfer_function".into(), "baseline_synthesis".into()],
        },
        LensEntry {
            name: "time_delay_interferometry".into(),
            category: LensCategory::Extended,
            description: "Cancel laser noise in unequal-arm interferometers via time-shifted combinations".into(),
            domain_affinity: vec!["gravitational_waves".into(), "interferometry".into(), "space_mission".into()],
            complementary: vec!["coherent_snr".into(), "null_stream".into(), "timing_integrity".into()],
        },
        LensEntry {
            name: "multi_messenger".into(),
            category: LensCategory::Extended,
            description: "Correlate signals across electromagnetic, gravitational, and neutrino channels".into(),
            domain_affinity: vec!["cosmology".into(), "particle_physics".into(), "gravitational_waves".into()],
            complementary: vec!["source_concordance".into(), "coincidence_gate".into(), "neutrino_oscillation".into()],
        },
        LensEntry {
            name: "transfer_function".into(),
            category: LensCategory::Extended,
            description: "Characterize system response by input-output spectral ratio".into(),
            domain_affinity: vec!["signal_processing".into(), "control_systems".into(), "calibration".into()],
            complementary: vec!["cross_spectral".into(), "gain_stability".into(), "calibration_uncertainty".into()],
        },
        LensEntry {
            name: "epoch_folding".into(),
            category: LensCategory::Extended,
            description: "Fold time series at trial periods to detect periodic signals in noisy data".into(),
            domain_affinity: vec!["astrophysics".into(), "pulsar".into(), "signal_processing".into()],
            complementary: vec!["heterodyne".into(), "cyclostationary".into(), "chirp_rate".into()],
        },
        LensEntry {
            name: "heterodyne".into(),
            category: LensCategory::Extended,
            description: "Demodulate narrowband signal by mixing with local oscillator reference".into(),
            domain_affinity: vec!["signal_processing".into(), "radio".into(), "pulsar".into()],
            complementary: vec!["epoch_folding".into(), "spectral_line_comb".into(), "allan_variance".into()],
        },
        LensEntry {
            name: "coincidence_gate".into(),
            category: LensCategory::Extended,
            description: "Require time-coincident triggers in multiple detectors to suppress false alarms".into(),
            domain_affinity: vec!["particle_physics".into(), "gravitational_waves".into(), "signal_processing".into()],
            complementary: vec!["multi_messenger".into(), "significance_combining".into(), "trials_factor".into()],
        },
        LensEntry {
            name: "angular_power".into(),
            category: LensCategory::Extended,
            description: "Decompose sky maps into spherical harmonic power spectrum".into(),
            domain_affinity: vec!["cosmology".into(), "cmb".into(), "gravitational_waves".into()],
            complementary: vec!["cmb_anisotropy".into(), "stochastic_background".into(), "primordial_spectrum".into()],
        },
        LensEntry {
            name: "baseline_synthesis".into(),
            category: LensCategory::Extended,
            description: "Combine multiple baselines for aperture synthesis imaging".into(),
            domain_affinity: vec!["radio_astronomy".into(), "interferometry".into(), "signal_processing".into()],
            complementary: vec!["cross_spectral".into(), "coherent_snr".into(), "angular_power".into()],
        },
        LensEntry {
            name: "dispersion_measure".into(),
            category: LensCategory::Extended,
            description: "Measure frequency-dependent pulse delay to infer column electron density".into(),
            domain_affinity: vec!["radio_astronomy".into(), "pulsar".into(), "cosmology".into()],
            complementary: vec!["epoch_folding".into(), "redshift_ladder".into(), "chirp_rate".into()],
        },

        // ══════════════════════════════════════════════
        // Statistical Verification (10)
        // ══════════════════════════════════════════════
        LensEntry {
            name: "trials_factor".into(),
            category: LensCategory::Extended,
            description: "Correct significance for look-elsewhere effect across multiple tests".into(),
            domain_affinity: vec!["statistics".into(), "particle_physics".into(), "signal_processing".into()],
            complementary: vec!["false_discovery_rate".into(), "significance_combining".into(), "background_estimation".into()],
        },
        LensEntry {
            name: "background_estimation".into(),
            category: LensCategory::Extended,
            description: "Estimate noise-only distribution via time slides or sidebands".into(),
            domain_affinity: vec!["statistics".into(), "particle_physics".into(), "gravitational_waves".into()],
            complementary: vec!["trials_factor".into(), "upper_limit".into(), "blind_analysis".into()],
        },
        LensEntry {
            name: "blind_analysis".into(),
            category: LensCategory::Extended,
            description: "Freeze analysis choices before unblinding signal region to prevent bias".into(),
            domain_affinity: vec!["statistics".into(), "particle_physics".into(), "experimental_design".into()],
            complementary: vec!["background_estimation".into(), "injection_recovery".into(), "replication_index".into()],
        },
        LensEntry {
            name: "bayesian_evidence".into(),
            category: LensCategory::Extended,
            description: "Compute marginal likelihood ratio for model comparison via Bayes factor".into(),
            domain_affinity: vec!["statistics".into(), "cosmology".into(), "signal_processing".into()],
            complementary: vec!["profile_likelihood".into(), "cls_exclusion".into(), "cumulative_evidence".into()],
        },
        LensEntry {
            name: "upper_limit".into(),
            category: LensCategory::Extended,
            description: "Set frequentist or Bayesian bound on signal strength given null result".into(),
            domain_affinity: vec!["statistics".into(), "particle_physics".into(), "gravitational_waves".into()],
            complementary: vec!["cls_exclusion".into(), "background_estimation".into(), "sensitivity_curve".into()],
        },
        LensEntry {
            name: "significance_combining".into(),
            category: LensCategory::Extended,
            description: "Merge p-values or test statistics from independent searches".into(),
            domain_affinity: vec!["statistics".into(), "meta_analysis".into(), "particle_physics".into()],
            complementary: vec!["trials_factor".into(), "coincidence_gate".into(), "cumulative_evidence".into()],
        },
        LensEntry {
            name: "false_discovery_rate".into(),
            category: LensCategory::Extended,
            description: "Control expected fraction of false positives among declared detections".into(),
            domain_affinity: vec!["statistics".into(), "genomics".into(), "signal_processing".into()],
            complementary: vec!["trials_factor".into(), "goodness_of_fit".into(), "replication_index".into()],
        },
        LensEntry {
            name: "profile_likelihood".into(),
            category: LensCategory::Extended,
            description: "Eliminate nuisance parameters by profiling the likelihood surface".into(),
            domain_affinity: vec!["statistics".into(), "particle_physics".into(), "cosmology".into()],
            complementary: vec!["bayesian_evidence".into(), "cls_exclusion".into(), "upper_limit".into()],
        },
        LensEntry {
            name: "cls_exclusion".into(),
            category: LensCategory::Extended,
            description: "Modified frequentist exclusion preventing exclusion of undetectable signals".into(),
            domain_affinity: vec!["particle_physics".into(), "statistics".into(), "bsm_physics".into()],
            complementary: vec!["profile_likelihood".into(), "upper_limit".into(), "bayesian_evidence".into()],
        },
        LensEntry {
            name: "goodness_of_fit".into(),
            category: LensCategory::Extended,
            description: "Test compatibility of observed data with a hypothesized model distribution".into(),
            domain_affinity: vec!["statistics".into(), "particle_physics".into(), "cosmology".into()],
            complementary: vec!["bayesian_evidence".into(), "false_discovery_rate".into(), "tension_detector".into()],
        },

        // ══════════════════════════════════════════════
        // Frequency / Time-Frequency (10)
        // ══════════════════════════════════════════════
        LensEntry {
            name: "wavelet_scalogram".into(),
            category: LensCategory::Extended,
            description: "Multi-resolution time-frequency decomposition via continuous wavelet transform".into(),
            domain_affinity: vec!["signal_processing".into(), "gravitational_waves".into(), "seismology".into()],
            complementary: vec!["hilbert_huang".into(), "reassigned_spectrogram".into(), "excess_power".into()],
        },
        LensEntry {
            name: "hilbert_huang".into(),
            category: LensCategory::Extended,
            description: "Adaptive time-frequency analysis via empirical mode decomposition and Hilbert spectrum".into(),
            domain_affinity: vec!["signal_processing".into(), "geophysics".into(), "biomedical".into()],
            complementary: vec!["wavelet_scalogram".into(), "cepstral".into(), "reassigned_spectrogram".into()],
        },
        LensEntry {
            name: "chirp_rate".into(),
            category: LensCategory::Extended,
            description: "Track instantaneous frequency evolution of swept-frequency signals".into(),
            domain_affinity: vec!["signal_processing".into(), "gravitational_waves".into(), "radar".into()],
            complementary: vec!["matched_filter".into(), "fractional_fourier".into(), "epoch_folding".into()],
        },
        LensEntry {
            name: "polyphase_filter".into(),
            category: LensCategory::Extended,
            description: "Efficient multi-rate channelization via polyphase filter bank decomposition".into(),
            domain_affinity: vec!["signal_processing".into(), "radio_astronomy".into(), "telecommunications".into()],
            complementary: vec!["sparse_spectral".into(), "spectral_line_comb".into(), "bispectrum".into()],
        },
        LensEntry {
            name: "bispectrum".into(),
            category: LensCategory::Extended,
            description: "Third-order spectral statistic detecting phase coupling and non-Gaussianity".into(),
            domain_affinity: vec!["signal_processing".into(), "plasma_physics".into(), "cosmology".into()],
            complementary: vec!["cyclostationary".into(), "polyphase_filter".into(), "wavelet_scalogram".into()],
        },
        LensEntry {
            name: "cyclostationary".into(),
            category: LensCategory::Extended,
            description: "Detect periodically correlated statistics in modulated or rotating sources".into(),
            domain_affinity: vec!["signal_processing".into(), "mechanical".into(), "radio_astronomy".into()],
            complementary: vec!["epoch_folding".into(), "bispectrum".into(), "spectral_line_comb".into()],
        },
        LensEntry {
            name: "reassigned_spectrogram".into(),
            category: LensCategory::Extended,
            description: "Sharpen time-frequency representation by reassigning energy to instantaneous coordinates".into(),
            domain_affinity: vec!["signal_processing".into(), "audio".into(), "seismology".into()],
            complementary: vec!["wavelet_scalogram".into(), "hilbert_huang".into(), "fractional_fourier".into()],
        },
        LensEntry {
            name: "fractional_fourier".into(),
            category: LensCategory::Extended,
            description: "Generalized Fourier transform for optimal detection of linear chirp signals".into(),
            domain_affinity: vec!["signal_processing".into(), "optics".into(), "radar".into()],
            complementary: vec!["chirp_rate".into(), "reassigned_spectrogram".into(), "sparse_spectral".into()],
        },
        LensEntry {
            name: "sparse_spectral".into(),
            category: LensCategory::Extended,
            description: "Recover spectral content from sub-Nyquist samples via compressive sensing".into(),
            domain_affinity: vec!["signal_processing".into(), "radio_astronomy".into(), "mri".into()],
            complementary: vec!["polyphase_filter".into(), "fractional_fourier".into(), "cepstral".into()],
        },
        LensEntry {
            name: "cepstral".into(),
            category: LensCategory::Extended,
            description: "Separate source and channel via inverse Fourier of log-spectrum".into(),
            domain_affinity: vec!["audio".into(), "signal_processing".into(), "speech".into()],
            complementary: vec!["hilbert_huang".into(), "sparse_spectral".into(), "wavelet_scalogram".into()],
        },

        // ══════════════════════════════════════════════
        // Anomaly / Pattern (10)
        // ══════════════════════════════════════════════
        LensEntry {
            name: "template_bank".into(),
            category: LensCategory::Extended,
            description: "Tile parameter space with templates for minimal-mismatch signal coverage".into(),
            domain_affinity: vec!["gravitational_waves".into(), "signal_processing".into(), "pattern_recognition".into()],
            complementary: vec!["matched_filter".into(), "fingerprint_match".into(), "clustering_trigger".into()],
        },
        LensEntry {
            name: "clustering_trigger".into(),
            category: LensCategory::Extended,
            description: "Group nearby triggers in time-frequency-parameter space to form event candidates".into(),
            domain_affinity: vec!["signal_processing".into(), "gravitational_waves".into(), "particle_physics".into()],
            complementary: vec!["template_bank".into(), "excess_power".into(), "coincidence_gate".into()],
        },
        LensEntry {
            name: "fingerprint_match".into(),
            category: LensCategory::Extended,
            description: "Identify known signal morphologies via feature vector similarity search".into(),
            domain_affinity: vec!["pattern_recognition".into(), "signal_processing".into(), "forensics".into()],
            complementary: vec!["template_bank".into(), "glitch_classifier".into(), "dimensional_reduction_physics".into()],
        },
        LensEntry {
            name: "changepoint".into(),
            category: LensCategory::Extended,
            description: "Detect abrupt statistical shifts in time series mean, variance, or distribution".into(),
            domain_affinity: vec!["statistics".into(), "signal_processing".into(), "climate".into()],
            complementary: vec!["phase_transition_detect".into(), "outlier_robust".into(), "noise_stationarity".into()],
        },
        LensEntry {
            name: "outlier_robust".into(),
            category: LensCategory::Extended,
            description: "Robust estimation methods resistant to heavy-tailed noise and outliers".into(),
            domain_affinity: vec!["statistics".into(), "signal_processing".into(), "data_quality".into()],
            complementary: vec!["changepoint".into(), "glitch_classifier".into(), "goodness_of_fit".into()],
        },
        LensEntry {
            name: "persistence_tda".into(),
            category: LensCategory::Extended,
            description: "Topological data analysis via persistent homology to find robust structural features".into(),
            domain_affinity: vec!["mathematics".into(), "data_science".into(), "cosmology".into()],
            complementary: vec!["dimensional_reduction_physics".into(), "recurrence_plot".into(), "n6_resonance_scan".into()],
        },
        LensEntry {
            name: "recurrence_plot".into(),
            category: LensCategory::Extended,
            description: "Visualize recurrence structure of dynamical systems in phase space".into(),
            domain_affinity: vec!["dynamical_systems".into(), "signal_processing".into(), "climate".into()],
            complementary: vec!["persistence_tda".into(), "phase_transition_detect".into(), "changepoint".into()],
        },
        LensEntry {
            name: "n6_resonance_scan".into(),
            category: LensCategory::Extended,
            description: "Scan data for n=6 arithmetic resonances across all known sigma-phi-tau patterns".into(),
            domain_affinity: vec!["n6_theory".into(), "cross_domain".into(), "discovery".into()],
            complementary: vec!["persistence_tda".into(), "discovery_potential".into(), "source_concordance".into()],
        },
        LensEntry {
            name: "phase_transition_detect".into(),
            category: LensCategory::Extended,
            description: "Identify critical points where system behavior changes qualitatively".into(),
            domain_affinity: vec!["physics".into(), "materials".into(), "complex_systems".into()],
            complementary: vec!["changepoint".into(), "recurrence_plot".into(), "persistence_tda".into()],
        },
        LensEntry {
            name: "dimensional_reduction_physics".into(),
            category: LensCategory::Extended,
            description: "Project high-dimensional data onto physically meaningful low-dimensional manifolds".into(),
            domain_affinity: vec!["physics".into(), "data_science".into(), "particle_physics".into()],
            complementary: vec!["persistence_tda".into(), "fingerprint_match".into(), "clustering_trigger".into()],
        },

        // ══════════════════════════════════════════════
        // Data Quality (10)
        // ══════════════════════════════════════════════
        LensEntry {
            name: "duty_cycle".into(),
            category: LensCategory::Extended,
            description: "Track fraction of observation time with science-quality data available".into(),
            domain_affinity: vec!["data_quality".into(), "observatory".into(), "signal_processing".into()],
            complementary: vec!["dead_time".into(), "data_quality_flag".into(), "sensitivity_curve".into()],
        },
        LensEntry {
            name: "calibration_uncertainty".into(),
            category: LensCategory::Extended,
            description: "Propagate detector response uncertainty into measurement error budget".into(),
            domain_affinity: vec!["calibration".into(), "data_quality".into(), "metrology".into()],
            complementary: vec!["gain_stability".into(), "transfer_function".into(), "injection_recovery".into()],
        },
        LensEntry {
            name: "data_quality_flag".into(),
            category: LensCategory::Extended,
            description: "Annotate data segments with quality categories for downstream selection".into(),
            domain_affinity: vec!["data_quality".into(), "gravitational_waves".into(), "observatory".into()],
            complementary: vec!["veto_channel".into(), "glitch_classifier".into(), "duty_cycle".into()],
        },
        LensEntry {
            name: "noise_stationarity".into(),
            category: LensCategory::Extended,
            description: "Test whether noise statistics remain stable over observation epochs".into(),
            domain_affinity: vec!["data_quality".into(), "signal_processing".into(), "gravitational_waves".into()],
            complementary: vec!["changepoint".into(), "whitening".into(), "null_stream".into()],
        },
        LensEntry {
            name: "dead_time".into(),
            category: LensCategory::Extended,
            description: "Correct for detector non-responsive intervals causing count rate underestimation".into(),
            domain_affinity: vec!["data_quality".into(), "particle_physics".into(), "nuclear".into()],
            complementary: vec!["duty_cycle".into(), "saturation_recovery".into(), "luminosity_monitor".into()],
        },
        LensEntry {
            name: "gain_stability".into(),
            category: LensCategory::Extended,
            description: "Monitor and correct detector gain drifts over time".into(),
            domain_affinity: vec!["calibration".into(), "data_quality".into(), "spectroscopy".into()],
            complementary: vec!["calibration_uncertainty".into(), "transfer_function".into(), "noise_stationarity".into()],
        },
        LensEntry {
            name: "cross_talk".into(),
            category: LensCategory::Extended,
            description: "Identify and mitigate spurious coupling between independent detector channels".into(),
            domain_affinity: vec!["data_quality".into(), "electronics".into(), "interferometry".into()],
            complementary: vec!["veto_channel".into(), "environment_monitor".into(), "null_stream".into()],
        },
        LensEntry {
            name: "saturation_recovery".into(),
            category: LensCategory::Extended,
            description: "Reconstruct signal information from detector saturation events".into(),
            domain_affinity: vec!["data_quality".into(), "electronics".into(), "photography".into()],
            complementary: vec!["dead_time".into(), "glitch_classifier".into(), "gain_stability".into()],
        },
        LensEntry {
            name: "timing_integrity".into(),
            category: LensCategory::Extended,
            description: "Verify clock synchronization and timestamp accuracy across distributed systems".into(),
            domain_affinity: vec!["data_quality".into(), "interferometry".into(), "distributed_systems".into()],
            complementary: vec!["time_delay_interferometry".into(), "coincidence_gate".into(), "allan_variance".into()],
        },
        LensEntry {
            name: "environment_monitor".into(),
            category: LensCategory::Extended,
            description: "Track environmental conditions that may couple into detector output".into(),
            domain_affinity: vec!["data_quality".into(), "observatory".into(), "seismology".into()],
            complementary: vec!["veto_channel".into(), "cross_talk".into(), "noise_stationarity".into()],
        },

        // ══════════════════════════════════════════════
        // Cosmology (10)
        // ══════════════════════════════════════════════
        LensEntry {
            name: "redshift_ladder".into(),
            category: LensCategory::Extended,
            description: "Chain distance indicators from local to cosmological scales for Hubble measurement".into(),
            domain_affinity: vec!["cosmology".into(), "astrophysics".into(), "distance_measurement".into()],
            complementary: vec!["gravitational_lensing".into(), "baryon_acoustic".into(), "tension_detector".into()],
        },
        LensEntry {
            name: "cmb_anisotropy".into(),
            category: LensCategory::Extended,
            description: "Extract cosmological parameters from CMB temperature and polarization fluctuations".into(),
            domain_affinity: vec!["cosmology".into(), "cmb".into(), "early_universe".into()],
            complementary: vec!["angular_power".into(), "primordial_spectrum".into(), "nucleosynthesis".into()],
        },
        LensEntry {
            name: "baryon_acoustic".into(),
            category: LensCategory::Extended,
            description: "Use baryon acoustic oscillation scale as a standard ruler for expansion history".into(),
            domain_affinity: vec!["cosmology".into(), "large_scale_structure".into(), "dark_energy".into()],
            complementary: vec!["redshift_ladder".into(), "dark_energy_eos".into(), "cmb_anisotropy".into()],
        },
        LensEntry {
            name: "gravitational_lensing".into(),
            category: LensCategory::Extended,
            description: "Map mass distribution via light deflection by intervening gravitational fields".into(),
            domain_affinity: vec!["cosmology".into(), "dark_matter".into(), "astrophysics".into()],
            complementary: vec!["redshift_ladder".into(), "cmb_anisotropy".into(), "dark_energy_eos".into()],
        },
        LensEntry {
            name: "nucleosynthesis".into(),
            category: LensCategory::Extended,
            description: "Constrain baryon density from primordial light element abundance ratios".into(),
            domain_affinity: vec!["cosmology".into(), "nuclear_physics".into(), "early_universe".into()],
            complementary: vec!["cmb_anisotropy".into(), "neutrino_oscillation".into(), "primordial_spectrum".into()],
        },
        LensEntry {
            name: "neutrino_oscillation".into(),
            category: LensCategory::Extended,
            description: "Measure neutrino mass splittings and mixing angles from flavor oscillation data".into(),
            domain_affinity: vec!["particle_physics".into(), "cosmology".into(), "nuclear_physics".into()],
            complementary: vec!["multi_messenger".into(), "nucleosynthesis".into(), "cp_violation".into()],
        },
        LensEntry {
            name: "dark_energy_eos".into(),
            category: LensCategory::Extended,
            description: "Constrain dark energy equation of state w(z) from expansion and growth probes".into(),
            domain_affinity: vec!["cosmology".into(), "dark_energy".into(), "large_scale_structure".into()],
            complementary: vec!["baryon_acoustic".into(), "gravitational_lensing".into(), "tension_detector".into()],
        },
        LensEntry {
            name: "primordial_spectrum".into(),
            category: LensCategory::Extended,
            description: "Reconstruct primordial power spectrum shape to test inflationary models".into(),
            domain_affinity: vec!["cosmology".into(), "early_universe".into(), "cmb".into()],
            complementary: vec!["cmb_anisotropy".into(), "angular_power".into(), "21cm_tomography".into()],
        },
        LensEntry {
            name: "21cm_tomography".into(),
            category: LensCategory::Extended,
            description: "Map neutral hydrogen distribution across cosmic dawn and reionization epochs".into(),
            domain_affinity: vec!["cosmology".into(), "radio_astronomy".into(), "early_universe".into()],
            complementary: vec!["primordial_spectrum".into(), "foreground_subtraction".into(), "dispersion_measure".into()],
        },
        LensEntry {
            name: "cosmic_ray_spectrum".into(),
            category: LensCategory::Extended,
            description: "Analyze cosmic ray energy spectrum for source identification and propagation physics".into(),
            domain_affinity: vec!["astroparticle".into(), "cosmology".into(), "particle_physics".into()],
            complementary: vec!["neutrino_oscillation".into(), "multi_messenger".into(), "cross_section_scan".into()],
        },

        // ══════════════════════════════════════════════
        // Particle Physics (10)
        // ══════════════════════════════════════════════
        LensEntry {
            name: "invariant_mass".into(),
            category: LensCategory::Extended,
            description: "Reconstruct parent particle mass from decay product four-momenta".into(),
            domain_affinity: vec!["particle_physics".into(), "collider".into(), "spectroscopy".into()],
            complementary: vec!["missing_energy".into(), "branching_ratio".into(), "spin_parity".into()],
        },
        LensEntry {
            name: "missing_energy".into(),
            category: LensCategory::Extended,
            description: "Infer invisible particle production from momentum imbalance in collider events".into(),
            domain_affinity: vec!["particle_physics".into(), "collider".into(), "dark_matter".into()],
            complementary: vec!["invariant_mass".into(), "cross_section_scan".into(), "effective_field_theory".into()],
        },
        LensEntry {
            name: "branching_ratio".into(),
            category: LensCategory::Extended,
            description: "Measure relative decay rates to test coupling structure of particle interactions".into(),
            domain_affinity: vec!["particle_physics".into(), "nuclear_physics".into(), "flavor_physics".into()],
            complementary: vec!["invariant_mass".into(), "cp_violation".into(), "flavor_symmetry".into()],
        },
        LensEntry {
            name: "running_coupling".into(),
            category: LensCategory::Extended,
            description: "Track energy-scale dependence of interaction strengths via renormalization group".into(),
            domain_affinity: vec!["particle_physics".into(), "qft".into(), "unification".into()],
            complementary: vec!["effective_field_theory".into(), "cross_section_scan".into(), "flavor_symmetry".into()],
        },
        LensEntry {
            name: "spin_parity".into(),
            category: LensCategory::Extended,
            description: "Determine quantum numbers of resonances from angular distribution analysis".into(),
            domain_affinity: vec!["particle_physics".into(), "nuclear_physics".into(), "spectroscopy".into()],
            complementary: vec!["invariant_mass".into(), "branching_ratio".into(), "cp_violation".into()],
        },
        LensEntry {
            name: "cross_section_scan".into(),
            category: LensCategory::Extended,
            description: "Measure interaction cross section as function of energy to find resonances and thresholds".into(),
            domain_affinity: vec!["particle_physics".into(), "nuclear_physics".into(), "collider".into()],
            complementary: vec!["invariant_mass".into(), "running_coupling".into(), "luminosity_monitor".into()],
        },
        LensEntry {
            name: "flavor_symmetry".into(),
            category: LensCategory::Extended,
            description: "Exploit approximate flavor symmetries to predict mass and coupling relations".into(),
            domain_affinity: vec!["particle_physics".into(), "nuclear_physics".into(), "quark_model".into()],
            complementary: vec!["branching_ratio".into(), "running_coupling".into(), "cp_violation".into()],
        },
        LensEntry {
            name: "cp_violation".into(),
            category: LensCategory::Extended,
            description: "Measure matter-antimatter asymmetry through CP-violating observables".into(),
            domain_affinity: vec!["particle_physics".into(), "cosmology".into(), "flavor_physics".into()],
            complementary: vec!["branching_ratio".into(), "spin_parity".into(), "neutrino_oscillation".into()],
        },
        LensEntry {
            name: "effective_field_theory".into(),
            category: LensCategory::Extended,
            description: "Parameterize new physics effects as higher-dimensional operators at low energy".into(),
            domain_affinity: vec!["particle_physics".into(), "bsm_physics".into(), "qft".into()],
            complementary: vec!["running_coupling".into(), "missing_energy".into(), "cls_exclusion".into()],
        },
        LensEntry {
            name: "luminosity_monitor".into(),
            category: LensCategory::Extended,
            description: "Measure integrated luminosity for absolute cross section normalization".into(),
            domain_affinity: vec!["particle_physics".into(), "collider".into(), "calibration".into()],
            complementary: vec!["cross_section_scan".into(), "dead_time".into(), "calibration_uncertainty".into()],
        },

        // ══════════════════════════════════════════════
        // Quantum / Precision (10)
        // ══════════════════════════════════════════════
        LensEntry {
            name: "quantum_rng_bias".into(),
            category: LensCategory::Extended,
            description: "Detect systematic bias in quantum random number generators via statistical tests".into(),
            domain_affinity: vec!["quantum".into(), "cryptography".into(), "metrology".into()],
            complementary: vec!["allan_variance".into(), "goodness_of_fit".into(), "entanglement_witness".into()],
        },
        LensEntry {
            name: "allan_variance".into(),
            category: LensCategory::Extended,
            description: "Characterize frequency stability and noise type of oscillators and clocks".into(),
            domain_affinity: vec!["metrology".into(), "precision_measurement".into(), "signal_processing".into()],
            complementary: vec!["timing_integrity".into(), "heterodyne".into(), "spectroscopy_precision".into()],
        },
        LensEntry {
            name: "squeezed_state".into(),
            category: LensCategory::Extended,
            description: "Detect sub-shot-noise measurement via quantum squeezed light or matter states".into(),
            domain_affinity: vec!["quantum".into(), "interferometry".into(), "gravitational_waves".into()],
            complementary: vec!["entanglement_witness".into(), "interferometer_fringe".into(), "decoherence_rate_measure".into()],
        },
        LensEntry {
            name: "entanglement_witness".into(),
            category: LensCategory::Extended,
            description: "Certify quantum entanglement without full state tomography via witness operators".into(),
            domain_affinity: vec!["quantum".into(), "quantum_computing".into(), "quantum_communication".into()],
            complementary: vec!["squeezed_state".into(), "quantum_rng_bias".into(), "decoherence_rate_measure".into()],
        },
        LensEntry {
            name: "spectroscopy_precision".into(),
            category: LensCategory::Extended,
            description: "Extract transition frequencies at fundamental precision limits for constant tests".into(),
            domain_affinity: vec!["precision_measurement".into(), "atomic_physics".into(), "metrology".into()],
            complementary: vec!["allan_variance".into(), "interferometer_fringe".into(), "casimir_force".into()],
        },
        LensEntry {
            name: "decoherence_rate_measure".into(),
            category: LensCategory::Extended,
            description: "Quantify environment-induced quantum coherence loss rate in open quantum systems".into(),
            domain_affinity: vec!["quantum".into(), "quantum_computing".into(), "condensed_matter".into()],
            complementary: vec!["entanglement_witness".into(), "squeezed_state".into(), "parity_violation".into()],
        },
        LensEntry {
            name: "interferometer_fringe".into(),
            category: LensCategory::Extended,
            description: "Extract phase information from interference fringe patterns at sub-wavelength precision".into(),
            domain_affinity: vec!["interferometry".into(), "precision_measurement".into(), "gravitational_waves".into()],
            complementary: vec!["squeezed_state".into(), "spectroscopy_precision".into(), "atom_interferometry".into()],
        },
        LensEntry {
            name: "parity_violation".into(),
            category: LensCategory::Extended,
            description: "Measure weak-force parity asymmetry in atomic or nuclear transitions".into(),
            domain_affinity: vec!["atomic_physics".into(), "nuclear_physics".into(), "particle_physics".into()],
            complementary: vec!["cp_violation".into(), "spectroscopy_precision".into(), "decoherence_rate_measure".into()],
        },
        LensEntry {
            name: "casimir_force".into(),
            category: LensCategory::Extended,
            description: "Measure quantum vacuum fluctuation forces between closely spaced surfaces".into(),
            domain_affinity: vec!["quantum".into(), "precision_measurement".into(), "nanotechnology".into()],
            complementary: vec!["spectroscopy_precision".into(), "atom_interferometry".into(), "interferometer_fringe".into()],
        },
        LensEntry {
            name: "atom_interferometry".into(),
            category: LensCategory::Extended,
            description: "Use matter-wave interference for precision gravity and inertial measurements".into(),
            domain_affinity: vec!["precision_measurement".into(), "gravity".into(), "quantum".into()],
            complementary: vec!["interferometer_fringe".into(), "casimir_force".into(), "squeezed_state".into()],
        },

        // ══════════════════════════════════════════════
        // SEDI Integration (10)
        // ══════════════════════════════════════════════
        LensEntry {
            name: "source_concordance".into(),
            category: LensCategory::Extended,
            description: "Cross-validate source parameters across independent detection channels".into(),
            domain_affinity: vec!["multi_messenger".into(), "cosmology".into(), "discovery".into()],
            complementary: vec!["multi_messenger".into(), "cumulative_evidence".into(), "n6_resonance_scan".into()],
        },
        LensEntry {
            name: "drake_parameter".into(),
            category: LensCategory::Extended,
            description: "Estimate detection probability by chaining astrophysical rate and sensitivity factors".into(),
            domain_affinity: vec!["astrobiology".into(), "seti".into(), "discovery".into()],
            complementary: vec!["sensitivity_curve".into(), "discovery_potential".into(), "blind_prediction".into()],
        },
        LensEntry {
            name: "blind_prediction".into(),
            category: LensCategory::Extended,
            description: "Register falsifiable predictions before observation to test theoretical frameworks".into(),
            domain_affinity: vec!["discovery".into(), "n6_theory".into(), "experimental_design".into()],
            complementary: vec!["replication_index".into(), "drake_parameter".into(), "cumulative_evidence".into()],
        },
        LensEntry {
            name: "cumulative_evidence".into(),
            category: LensCategory::Extended,
            description: "Track running significance as data accumulates over time for emerging signals".into(),
            domain_affinity: vec!["statistics".into(), "discovery".into(), "clinical_trials".into()],
            complementary: vec!["significance_combining".into(), "bayesian_evidence".into(), "source_concordance".into()],
        },
        LensEntry {
            name: "information_content".into(),
            category: LensCategory::Extended,
            description: "Quantify bits of information gained from each observation or measurement".into(),
            domain_affinity: vec!["information_theory".into(), "experimental_design".into(), "discovery".into()],
            complementary: vec!["sensitivity_curve".into(), "bayesian_evidence".into(), "discovery_potential".into()],
        },
        LensEntry {
            name: "tension_detector".into(),
            category: LensCategory::Extended,
            description: "Quantify statistical tension between independent measurements of the same quantity".into(),
            domain_affinity: vec!["cosmology".into(), "statistics".into(), "meta_analysis".into()],
            complementary: vec!["goodness_of_fit".into(), "dark_energy_eos".into(), "redshift_ladder".into()],
        },
        LensEntry {
            name: "sensitivity_curve".into(),
            category: LensCategory::Extended,
            description: "Map detector sensitivity as function of signal parameters for reach estimation".into(),
            domain_affinity: vec!["gravitational_waves".into(), "particle_physics".into(), "discovery".into()],
            complementary: vec!["upper_limit".into(), "duty_cycle".into(), "injection_recovery".into()],
        },
        LensEntry {
            name: "foreground_subtraction".into(),
            category: LensCategory::Extended,
            description: "Separate target signal from dominant astrophysical foreground contamination".into(),
            domain_affinity: vec!["cosmology".into(), "cmb".into(), "radio_astronomy".into()],
            complementary: vec!["21cm_tomography".into(), "cmb_anisotropy".into(), "angular_power".into()],
        },
        LensEntry {
            name: "replication_index".into(),
            category: LensCategory::Extended,
            description: "Score reproducibility of findings across independent experiments or datasets".into(),
            domain_affinity: vec!["meta_analysis".into(), "discovery".into(), "experimental_design".into()],
            complementary: vec!["blind_prediction".into(), "false_discovery_rate".into(), "blind_analysis".into()],
        },
        LensEntry {
            name: "discovery_potential".into(),
            category: LensCategory::Extended,
            description: "Estimate probability of first detection given current sensitivity and source models".into(),
            domain_affinity: vec!["discovery".into(), "gravitational_waves".into(), "particle_physics".into()],
            complementary: vec!["sensitivity_curve".into(), "drake_parameter".into(), "information_content".into()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sedi_lens_count() {
        let entries = sedi_signal_lens_entries();
        assert_eq!(entries.len(), 100, "Must have exactly 100 SEDI signal lenses");
    }

    #[test]
    fn test_sedi_lens_names_unique() {
        let entries = sedi_signal_lens_entries();
        let mut names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        names.sort();
        let total = names.len();
        names.dedup();
        assert_eq!(names.len(), total, "All 100 SEDI lens names must be unique");
    }

    #[test]
    fn test_sedi_lenses_are_extended_category() {
        let entries = sedi_signal_lens_entries();
        for entry in &entries {
            assert_eq!(
                entry.category,
                LensCategory::Extended,
                "SEDI lens '{}' must be Extended category",
                entry.name
            );
        }
    }

    #[test]
    fn test_sedi_lenses_have_descriptions() {
        let entries = sedi_signal_lens_entries();
        for entry in &entries {
            assert!(
                !entry.description.is_empty(),
                "SEDI lens '{}' must have a description",
                entry.name
            );
        }
    }

    #[test]
    fn test_sedi_lenses_have_domain_affinity() {
        let entries = sedi_signal_lens_entries();
        for entry in &entries {
            assert!(
                !entry.domain_affinity.is_empty(),
                "SEDI lens '{}' must have at least one domain affinity",
                entry.name
            );
        }
    }

    #[test]
    fn test_sedi_lenses_have_complementary() {
        let entries = sedi_signal_lens_entries();
        for entry in &entries {
            assert!(
                entry.complementary.len() >= 2,
                "SEDI lens '{}' must have at least 2 complementary lenses, got {}",
                entry.name,
                entry.complementary.len()
            );
        }
    }
}
