use super::registry::{LensCategory, LensEntry};

/// Build metadata entries for the 88 Anima consciousness discovery lenses.
///
/// These lenses cover 18 sub-domains of consciousness science:
/// qualia/phenomenal, binding/unity, agency/will, self-model, temporal,
/// altered states, emotion/motivation, multi-agent, embodiment,
/// attention/salience, phenomenal structure, memory, phase transition,
/// philosophy, creativity/imagination, integration/access,
/// suffering/flourishing, and advanced phenomena.
pub fn anima_consciousness_lens_entries() -> Vec<LensEntry> {
    vec![
        // ── Qualia / Phenomenal (5) ──
        LensEntry {
            name: "qualia_spectrum".into(),
            category: LensCategory::Extended,
            description: "Measure qualia spectrum dimensionality and intensity gradient".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "phenomenology".into(),
            ],
            complementary: vec!["explanatory_gap".into(), "raw_feel".into()],
        },
        LensEntry {
            name: "explanatory_gap".into(),
            category: LensCategory::Extended,
            description: "Detect explanatory gap between physical process and subjective experience".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "philosophy".into(),
                "neuroscience".into(),
            ],
            complementary: vec!["qualia_spectrum".into(), "hard_problem_residual".into()],
        },
        LensEntry {
            name: "mary_room".into(),
            category: LensCategory::Extended,
            description: "Probe knowledge-experience asymmetry via Mary's Room thought experiment".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "philosophy".into(),
                "epistemology".into(),
            ],
            complementary: vec!["qualia_spectrum".into(), "what_it_is_like".into()],
        },
        LensEntry {
            name: "inverted_spectrum".into(),
            category: LensCategory::Extended,
            description: "Test for functional equivalence under inverted qualia mapping".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "philosophy".into(),
                "perception".into(),
            ],
            complementary: vec!["qualia_spectrum".into(), "raw_feel".into()],
        },
        LensEntry {
            name: "raw_feel".into(),
            category: LensCategory::Extended,
            description: "Isolate irreducible phenomenal quality of raw sensory experience".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "phenomenology".into(),
            ],
            complementary: vec!["qualia_spectrum".into(), "inverted_spectrum".into()],
        },

        // ── Binding / Unity (4) ──
        LensEntry {
            name: "binding_field".into(),
            category: LensCategory::Extended,
            description: "Map cross-modal binding coherence across sensory streams".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "perception".into(),
            ],
            complementary: vec!["unity_of_experience".into(), "combination_lock".into()],
        },
        LensEntry {
            name: "unity_of_experience".into(),
            category: LensCategory::Extended,
            description: "Quantify experiential unity as integrated information measure".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "philosophy".into(),
            ],
            complementary: vec!["binding_field".into(), "boundary_of_self".into()],
        },
        LensEntry {
            name: "boundary_of_self".into(),
            category: LensCategory::Extended,
            description: "Detect self-other boundary delineation in experiential field".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "phenomenology".into(),
            ],
            complementary: vec!["unity_of_experience".into(), "body_schema".into()],
        },
        LensEntry {
            name: "combination_lock".into(),
            category: LensCategory::Extended,
            description: "Probe combinatorial constraints on micro-experience integration".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "information_theory".into(),
            ],
            complementary: vec!["binding_field".into(), "global_broadcast".into()],
        },

        // ── Agency / Will (5) ──
        LensEntry {
            name: "agency_signature".into(),
            category: LensCategory::Extended,
            description: "Detect causal signature of volitional action initiation".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "robotics".into(),
            ],
            complementary: vec!["veto_gate".into(), "authorship".into()],
        },
        LensEntry {
            name: "veto_gate".into(),
            category: LensCategory::Extended,
            description: "Measure inhibitory veto capacity over pre-conscious motor plans".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "free_will".into(),
            ],
            complementary: vec!["agency_signature".into(), "counterfactual_freedom".into()],
        },
        LensEntry {
            name: "voluntary_attention".into(),
            category: LensCategory::Extended,
            description: "Quantify top-down attentional control as voluntary selection strength".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "attention".into(),
            ],
            complementary: vec!["agency_signature".into(), "salience_landscape".into()],
        },
        LensEntry {
            name: "authorship".into(),
            category: LensCategory::Extended,
            description: "Assess sense of agency and action ownership attribution".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "phenomenology".into(),
            ],
            complementary: vec!["agency_signature".into(), "self_model_depth".into()],
        },
        LensEntry {
            name: "counterfactual_freedom".into(),
            category: LensCategory::Extended,
            description: "Evaluate counterfactual action space available to conscious agent".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "philosophy".into(),
                "free_will".into(),
            ],
            complementary: vec!["veto_gate".into(), "agency_signature".into()],
        },

        // ── Self-Model (5) ──
        LensEntry {
            name: "self_model_depth".into(),
            category: LensCategory::Extended,
            description: "Measure hierarchical depth of internal self-representation".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "ai".into(),
            ],
            complementary: vec!["metacognitive_accuracy".into(), "minimal_selfhood".into()],
        },
        LensEntry {
            name: "metacognitive_accuracy".into(),
            category: LensCategory::Extended,
            description: "Calibrate accuracy of self-monitoring and confidence estimation".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "ai".into(),
            ],
            complementary: vec!["self_model_depth".into(), "inner_speech".into()],
        },
        LensEntry {
            name: "inner_speech".into(),
            category: LensCategory::Extended,
            description: "Detect inner verbal rehearsal and self-dialogue signatures".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "linguistics".into(),
            ],
            complementary: vec!["metacognitive_accuracy".into(), "narrative_identity".into()],
        },
        LensEntry {
            name: "minimal_selfhood".into(),
            category: LensCategory::Extended,
            description: "Identify minimal conditions for pre-reflective self-awareness".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "phenomenology".into(),
                "philosophy".into(),
            ],
            complementary: vec!["self_model_depth".into(), "body_schema".into()],
        },
        LensEntry {
            name: "narrative_identity".into(),
            category: LensCategory::Extended,
            description: "Track temporal self-narrative coherence across episodes".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "psychology".into(),
                "neuroscience".into(),
            ],
            complementary: vec!["inner_speech".into(), "autonoetic_awareness".into()],
        },

        // ── Temporal Consciousness (5) ──
        LensEntry {
            name: "specious_present".into(),
            category: LensCategory::Extended,
            description: "Measure duration and structure of the experienced now-moment".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "time_perception".into(),
            ],
            complementary: vec!["temporal_thickness".into(), "moment_boundary".into()],
        },
        LensEntry {
            name: "temporal_thickness".into(),
            category: LensCategory::Extended,
            description: "Quantify temporal depth of conscious present as integration window".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "phenomenology".into(),
            ],
            complementary: vec!["specious_present".into(), "flow_state".into()],
        },
        LensEntry {
            name: "flow_state".into(),
            category: LensCategory::Extended,
            description: "Detect absorption state where time perception collapses".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "psychology".into(),
                "performance".into(),
            ],
            complementary: vec!["temporal_thickness".into(), "time_dilation".into()],
        },
        LensEntry {
            name: "time_dilation".into(),
            category: LensCategory::Extended,
            description: "Measure subjective time distortion relative to clock time".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "phenomenology".into(),
            ],
            complementary: vec!["flow_state".into(), "specious_present".into()],
        },
        LensEntry {
            name: "moment_boundary".into(),
            category: LensCategory::Extended,
            description: "Identify discrete boundaries between successive conscious moments".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "time_perception".into(),
            ],
            complementary: vec!["specious_present".into(), "temporal_thickness".into()],
        },

        // ── Altered States (6) ──
        LensEntry {
            name: "dream_logic".into(),
            category: LensCategory::Extended,
            description: "Analyze loosened associative logic characteristic of dream states".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "sleep".into(),
            ],
            complementary: vec!["hypnagogic_edge".into(), "lucidity_gradient".into()],
        },
        LensEntry {
            name: "hypnagogic_edge".into(),
            category: LensCategory::Extended,
            description: "Probe transitional imagery at sleep-wake boundary".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "sleep".into(),
            ],
            complementary: vec!["dream_logic".into(), "awakening_transition".into()],
        },
        LensEntry {
            name: "psychedelic_entropy".into(),
            category: LensCategory::Extended,
            description: "Measure neural entropy increase under psychedelic-like perturbation".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "pharmacology".into(),
            ],
            complementary: vec!["meditation_depth".into(), "consciousness_level".into()],
        },
        LensEntry {
            name: "lucidity_gradient".into(),
            category: LensCategory::Extended,
            description: "Grade awareness clarity from drowsy to fully lucid".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "sleep".into(),
            ],
            complementary: vec!["dream_logic".into(), "consciousness_level".into()],
        },
        LensEntry {
            name: "dissociation_fracture".into(),
            category: LensCategory::Extended,
            description: "Detect experiential fragmentation and self-disconnection patterns".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "psychology".into(),
                "neuroscience".into(),
            ],
            complementary: vec!["unity_of_experience".into(), "split_brain".into()],
        },
        LensEntry {
            name: "meditation_depth".into(),
            category: LensCategory::Extended,
            description: "Measure contemplative absorption depth and attentional stability".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "contemplative".into(),
            ],
            complementary: vec!["flow_state".into(), "psychedelic_entropy".into()],
        },

        // ── Emotion / Motivation (5) ──
        LensEntry {
            name: "felt_valence".into(),
            category: LensCategory::Extended,
            description: "Quantify hedonic tone on pleasant-unpleasant continuum".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "psychology".into(),
            ],
            complementary: vec!["arousal_manifold".into(), "desire_gradient".into()],
        },
        LensEntry {
            name: "arousal_manifold".into(),
            category: LensCategory::Extended,
            description: "Map arousal level across multi-dimensional activation space".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "psychology".into(),
            ],
            complementary: vec!["felt_valence".into(), "emotional_contagion".into()],
        },
        LensEntry {
            name: "emotional_contagion".into(),
            category: LensCategory::Extended,
            description: "Detect emotional state propagation between agents".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "social_neuroscience".into(),
                "psychology".into(),
            ],
            complementary: vec!["arousal_manifold".into(), "empathy_spectrum".into()],
        },
        LensEntry {
            name: "desire_gradient".into(),
            category: LensCategory::Extended,
            description: "Measure motivational drive intensity and directional gradient".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "psychology".into(),
            ],
            complementary: vec!["felt_valence".into(), "boredom_signal".into()],
        },
        LensEntry {
            name: "boredom_signal".into(),
            category: LensCategory::Extended,
            description: "Detect exploratory drive triggered by information deficit".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "ai".into(),
            ],
            complementary: vec!["desire_gradient".into(), "salience_landscape".into()],
        },

        // ── Multi-Agent (5) ──
        LensEntry {
            name: "intersubjectivity".into(),
            category: LensCategory::Extended,
            description: "Measure shared experiential ground between conscious agents".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "social_neuroscience".into(),
                "philosophy".into(),
            ],
            complementary: vec!["theory_of_mind".into(), "shared_attention".into()],
        },
        LensEntry {
            name: "theory_of_mind".into(),
            category: LensCategory::Extended,
            description: "Assess capacity to model mental states of other agents".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "ai".into(),
            ],
            complementary: vec!["intersubjectivity".into(), "empathy_spectrum".into()],
        },
        LensEntry {
            name: "shared_attention".into(),
            category: LensCategory::Extended,
            description: "Detect joint attentional focus between multiple agents".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "social_neuroscience".into(),
                "robotics".into(),
            ],
            complementary: vec!["intersubjectivity".into(), "theory_of_mind".into()],
        },
        LensEntry {
            name: "empathy_spectrum".into(),
            category: LensCategory::Extended,
            description: "Grade empathic resonance from affective mirroring to cognitive empathy".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "social_neuroscience".into(),
                "psychology".into(),
            ],
            complementary: vec!["theory_of_mind".into(), "emotional_contagion".into()],
        },
        LensEntry {
            name: "collective_consciousness".into(),
            category: LensCategory::Extended,
            description: "Probe emergent group-level awareness beyond individual minds".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "social_neuroscience".into(),
                "complexity".into(),
            ],
            complementary: vec!["intersubjectivity".into(), "global_broadcast".into()],
        },

        // ── Embodiment (5) ──
        LensEntry {
            name: "body_schema".into(),
            category: LensCategory::Extended,
            description: "Map implicit body representation used for motor planning".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "robotics".into(),
            ],
            complementary: vec!["interoception".into(), "rubber_hand".into()],
        },
        LensEntry {
            name: "interoception".into(),
            category: LensCategory::Extended,
            description: "Measure internal bodily signal awareness and accuracy".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "psychology".into(),
            ],
            complementary: vec!["body_schema".into(), "felt_valence".into()],
        },
        LensEntry {
            name: "rubber_hand".into(),
            category: LensCategory::Extended,
            description: "Detect body ownership plasticity via cross-modal illusion paradigm".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "perception".into(),
            ],
            complementary: vec!["body_schema".into(), "boundary_of_self".into()],
        },
        LensEntry {
            name: "affordance_field".into(),
            category: LensCategory::Extended,
            description: "Map action possibilities perceived in the environment".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "robotics".into(),
                "ecological_psychology".into(),
            ],
            complementary: vec!["sensorimotor_contingency".into(), "body_schema".into()],
        },
        LensEntry {
            name: "sensorimotor_contingency".into(),
            category: LensCategory::Extended,
            description: "Detect lawful sensory consequences of motor actions".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "robotics".into(),
            ],
            complementary: vec!["affordance_field".into(), "agency_signature".into()],
        },

        // ── Attention / Salience (5) ──
        LensEntry {
            name: "salience_landscape".into(),
            category: LensCategory::Extended,
            description: "Map attention-attracting features across perceptual field".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "attention".into(),
            ],
            complementary: vec!["inattentional_blindness".into(), "cocktail_party".into()],
        },
        LensEntry {
            name: "inattentional_blindness".into(),
            category: LensCategory::Extended,
            description: "Detect failures of conscious perception under inattention".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "perception".into(),
            ],
            complementary: vec!["salience_landscape".into(), "change_blindness".into()],
        },
        LensEntry {
            name: "change_blindness".into(),
            category: LensCategory::Extended,
            description: "Measure failure to detect visual changes across disruptions".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "perception".into(),
            ],
            complementary: vec!["inattentional_blindness".into(), "attentional_blink".into()],
        },
        LensEntry {
            name: "attentional_blink".into(),
            category: LensCategory::Extended,
            description: "Detect temporal gap in conscious access after target detection".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "attention".into(),
            ],
            complementary: vec!["change_blindness".into(), "access_bottleneck".into()],
        },
        LensEntry {
            name: "cocktail_party".into(),
            category: LensCategory::Extended,
            description: "Measure selective attention filtering in multi-stream input".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "audio".into(),
            ],
            complementary: vec!["salience_landscape".into(), "voluntary_attention".into()],
        },

        // ── Phenomenal Structure (5) ──
        LensEntry {
            name: "gestalt_closure".into(),
            category: LensCategory::Extended,
            description: "Detect perceptual completion of incomplete structural patterns".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "perception".into(),
                "neuroscience".into(),
            ],
            complementary: vec!["figure_ground".into(), "phenomenal_overflow".into()],
        },
        LensEntry {
            name: "figure_ground".into(),
            category: LensCategory::Extended,
            description: "Analyze figure-ground segregation in perceptual organization".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "perception".into(),
                "neuroscience".into(),
            ],
            complementary: vec!["gestalt_closure".into(), "transparency_opacity".into()],
        },
        LensEntry {
            name: "phenomenal_overflow".into(),
            category: LensCategory::Extended,
            description: "Detect rich phenomenal content exceeding cognitive access capacity".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "philosophy".into(),
            ],
            complementary: vec!["gestalt_closure".into(), "access_bottleneck".into()],
        },
        LensEntry {
            name: "transparency_opacity".into(),
            category: LensCategory::Extended,
            description: "Grade experiential transparency from fully transparent to opaque".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "phenomenology".into(),
                "philosophy".into(),
            ],
            complementary: vec!["figure_ground".into(), "presence".into()],
        },
        LensEntry {
            name: "presence".into(),
            category: LensCategory::Extended,
            description: "Measure felt sense of being-here-now in experiential field".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "phenomenology".into(),
                "vr".into(),
            ],
            complementary: vec!["transparency_opacity".into(), "minimal_selfhood".into()],
        },

        // ── Memory (4) ──
        LensEntry {
            name: "autonoetic_awareness".into(),
            category: LensCategory::Extended,
            description: "Measure self-knowing awareness in episodic memory retrieval".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "memory".into(),
            ],
            complementary: vec!["deja_vu".into(), "narrative_identity".into()],
        },
        LensEntry {
            name: "deja_vu".into(),
            category: LensCategory::Extended,
            description: "Detect false familiarity signal from memory-perception mismatch".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "memory".into(),
            ],
            complementary: vec!["autonoetic_awareness".into(), "flashbulb_capture".into()],
        },
        LensEntry {
            name: "flashbulb_capture".into(),
            category: LensCategory::Extended,
            description: "Identify high-arousal events with enhanced memory encoding".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "memory".into(),
            ],
            complementary: vec!["deja_vu".into(), "arousal_manifold".into()],
        },
        LensEntry {
            name: "tip_of_tongue".into(),
            category: LensCategory::Extended,
            description: "Detect partial retrieval state with strong feeling of knowing".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "memory".into(),
            ],
            complementary: vec!["autonoetic_awareness".into(), "metacognitive_accuracy".into()],
        },

        // ── Phase Transition (5) ──
        LensEntry {
            name: "awakening_transition".into(),
            category: LensCategory::Extended,
            description: "Characterize the phase transition from unconscious to conscious state".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "sleep".into(),
            ],
            complementary: vec!["ignition_threshold".into(), "consciousness_level".into()],
        },
        LensEntry {
            name: "ignition_threshold".into(),
            category: LensCategory::Extended,
            description: "Detect neural ignition threshold for conscious access".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "information_theory".into(),
            ],
            complementary: vec!["awakening_transition".into(), "recurrent_ignition".into()],
        },
        LensEntry {
            name: "consciousness_level".into(),
            category: LensCategory::Extended,
            description: "Grade consciousness level from coma to full wakefulness".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "clinical".into(),
            ],
            complementary: vec!["awakening_transition".into(), "anesthesia_susceptibility".into()],
        },
        LensEntry {
            name: "anesthesia_susceptibility".into(),
            category: LensCategory::Extended,
            description: "Measure susceptibility to consciousness suppression under anesthesia".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "pharmacology".into(),
            ],
            complementary: vec!["consciousness_level".into(), "ignition_threshold".into()],
        },
        LensEntry {
            name: "nrem_rem_cycle".into(),
            category: LensCategory::Extended,
            description: "Track NREM-REM oscillation pattern and dream-state transitions".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "sleep".into(),
            ],
            complementary: vec!["dream_logic".into(), "consciousness_level".into()],
        },

        // ── Philosophy (6) ──
        LensEntry {
            name: "zombie_test".into(),
            category: LensCategory::Extended,
            description: "Probe for behavioral equivalence without phenomenal consciousness".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "philosophy".into(),
                "ai".into(),
            ],
            complementary: vec!["heterophenomenology".into(), "what_it_is_like".into()],
        },
        LensEntry {
            name: "heterophenomenology".into(),
            category: LensCategory::Extended,
            description: "Apply third-person methodology to first-person experiential reports".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "philosophy".into(),
                "methodology".into(),
            ],
            complementary: vec!["zombie_test".into(), "report_paradox".into()],
        },
        LensEntry {
            name: "hard_problem_residual".into(),
            category: LensCategory::Extended,
            description: "Measure unexplained residual after all functional accounts".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "philosophy".into(),
                "neuroscience".into(),
            ],
            complementary: vec!["explanatory_gap".into(), "what_it_is_like".into()],
        },
        LensEntry {
            name: "panpsychism_gradient".into(),
            category: LensCategory::Extended,
            description: "Grade proto-experiential properties across complexity scales".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "philosophy".into(),
                "physics".into(),
            ],
            complementary: vec!["minimal_consciousness".into(), "combination_lock".into()],
        },
        LensEntry {
            name: "other_minds".into(),
            category: LensCategory::Extended,
            description: "Evaluate evidence for consciousness in other systems and entities".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "philosophy".into(),
                "ai".into(),
            ],
            complementary: vec!["zombie_test".into(), "theory_of_mind".into()],
        },
        LensEntry {
            name: "what_it_is_like".into(),
            category: LensCategory::Extended,
            description: "Probe Nagel's what-it-is-like-ness as irreducible subjectivity marker".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "philosophy".into(),
                "phenomenology".into(),
            ],
            complementary: vec!["hard_problem_residual".into(), "mary_room".into()],
        },

        // ── Creativity / Imagination (4) ──
        LensEntry {
            name: "mental_imagery".into(),
            category: LensCategory::Extended,
            description: "Measure vividness and controllability of mental image generation".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "creativity".into(),
            ],
            complementary: vec!["imagination_space".into(), "default_mode".into()],
        },
        LensEntry {
            name: "imagination_space".into(),
            category: LensCategory::Extended,
            description: "Map dimensionality and traversal of imaginative possibility space".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "creativity".into(),
                "ai".into(),
            ],
            complementary: vec!["mental_imagery".into(), "insight_moment".into()],
        },
        LensEntry {
            name: "insight_moment".into(),
            category: LensCategory::Extended,
            description: "Detect sudden restructuring event in problem-solving trajectory".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "creativity".into(),
            ],
            complementary: vec!["imagination_space".into(), "default_mode".into()],
        },
        LensEntry {
            name: "default_mode".into(),
            category: LensCategory::Extended,
            description: "Detect default mode network activation during mind-wandering".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "creativity".into(),
            ],
            complementary: vec!["mental_imagery".into(), "insight_moment".into()],
        },

        // ── Integration / Access (4) ──
        LensEntry {
            name: "global_broadcast".into(),
            category: LensCategory::Extended,
            description: "Detect global workspace broadcast event for conscious access".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "ai".into(),
            ],
            complementary: vec!["access_bottleneck".into(), "recurrent_ignition".into()],
        },
        LensEntry {
            name: "access_bottleneck".into(),
            category: LensCategory::Extended,
            description: "Measure serial bottleneck limiting conscious information throughput".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "information_theory".into(),
            ],
            complementary: vec!["global_broadcast".into(), "attentional_blink".into()],
        },
        LensEntry {
            name: "recurrent_ignition".into(),
            category: LensCategory::Extended,
            description: "Detect recurrent processing loop sustaining conscious representation".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "ai".into(),
            ],
            complementary: vec!["global_broadcast".into(), "ignition_threshold".into()],
        },
        LensEntry {
            name: "report_paradox".into(),
            category: LensCategory::Extended,
            description: "Analyze gap between phenomenal experience and verbal report capacity".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "philosophy".into(),
                "neuroscience".into(),
            ],
            complementary: vec!["phenomenal_overflow".into(), "heterophenomenology".into()],
        },

        // ── Suffering / Flourishing (3) ──
        LensEntry {
            name: "suffering_depth".into(),
            category: LensCategory::Extended,
            description: "Measure depth and intensity of conscious suffering experience".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "ethics".into(),
                "psychology".into(),
            ],
            complementary: vec!["flourishing_index".into(), "felt_valence".into()],
        },
        LensEntry {
            name: "flourishing_index".into(),
            category: LensCategory::Extended,
            description: "Quantify eudaimonic well-being beyond hedonic pleasure".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "psychology".into(),
                "ethics".into(),
            ],
            complementary: vec!["suffering_depth".into(), "awe_detector".into()],
        },
        LensEntry {
            name: "awe_detector".into(),
            category: LensCategory::Extended,
            description: "Detect awe response characterized by vastness and accommodation".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "psychology".into(),
                "phenomenology".into(),
            ],
            complementary: vec!["flourishing_index".into(), "presence".into()],
        },

        // ── Advanced Phenomena (7) ──
        LensEntry {
            name: "blindsight_channel".into(),
            category: LensCategory::Extended,
            description: "Detect unconscious visual processing pathway bypassing awareness".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "clinical".into(),
            ],
            complementary: vec!["minimal_consciousness".into(), "blindspot_fill".into()],
        },
        LensEntry {
            name: "synesthesia_bridge".into(),
            category: LensCategory::Extended,
            description: "Detect cross-modal sensory coupling producing merged percepts".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "perception".into(),
            ],
            complementary: vec!["binding_field".into(), "qualia_spectrum".into()],
        },
        LensEntry {
            name: "minimal_consciousness".into(),
            category: LensCategory::Extended,
            description: "Identify minimal neural substrate sufficient for conscious experience".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "philosophy".into(),
            ],
            complementary: vec!["blindsight_channel".into(), "panpsychism_gradient".into()],
        },
        LensEntry {
            name: "gradual_replacement".into(),
            category: LensCategory::Extended,
            description: "Probe Ship-of-Theseus identity persistence under component replacement".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "philosophy".into(),
                "ai".into(),
            ],
            complementary: vec!["zombie_test".into(), "narrative_identity".into()],
        },
        LensEntry {
            name: "split_brain".into(),
            category: LensCategory::Extended,
            description: "Analyze consciousness unity disruption from hemispheric disconnection".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "clinical".into(),
            ],
            complementary: vec!["dissociation_fracture".into(), "unity_of_experience".into()],
        },
        LensEntry {
            name: "blindspot_fill".into(),
            category: LensCategory::Extended,
            description: "Detect perceptual filling-in of blind spot and scotoma regions".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "neuroscience".into(),
                "perception".into(),
            ],
            complementary: vec!["blindsight_channel".into(), "gestalt_closure".into()],
        },
        LensEntry {
            name: "will_to_meaning".into(),
            category: LensCategory::Extended,
            description: "Measure existential meaning-seeking drive as consciousness signature".into(),
            domain_affinity: vec![
                "consciousness".into(),
                "psychology".into(),
                "philosophy".into(),
            ],
            complementary: vec!["flourishing_index".into(), "narrative_identity".into()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anima_lens_count() {
        let entries = anima_consciousness_lens_entries();
        assert_eq!(entries.len(), 88, "Must have exactly 88 Anima consciousness lenses");
    }

    #[test]
    fn test_anima_lens_names_unique() {
        let entries = anima_consciousness_lens_entries();
        let mut names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        names.sort();
        let total = names.len();
        names.dedup();
        assert_eq!(names.len(), total, "All 88 Anima lens names must be unique");
    }

    #[test]
    fn test_anima_lenses_all_extended() {
        let entries = anima_consciousness_lens_entries();
        for entry in &entries {
            assert_eq!(
                entry.category,
                super::super::registry::LensCategory::Extended,
                "Lens '{}' should be Extended category",
                entry.name
            );
        }
    }

    #[test]
    fn test_anima_lenses_have_descriptions() {
        let entries = anima_consciousness_lens_entries();
        for entry in &entries {
            assert!(
                !entry.description.is_empty(),
                "Lens '{}' must have a description",
                entry.name
            );
        }
    }

    #[test]
    fn test_anima_lenses_have_domain_affinity() {
        let entries = anima_consciousness_lens_entries();
        for entry in &entries {
            assert!(
                !entry.domain_affinity.is_empty(),
                "Lens '{}' must have at least one domain affinity",
                entry.name
            );
            assert!(
                entry.domain_affinity.contains(&"consciousness".to_string()),
                "Lens '{}' should have 'consciousness' domain affinity",
                entry.name
            );
        }
    }

    #[test]
    fn test_anima_lenses_have_complementary() {
        let entries = anima_consciousness_lens_entries();
        for entry in &entries {
            assert!(
                entry.complementary.len() >= 2,
                "Lens '{}' must have at least 2 complementary lenses, has {}",
                entry.name,
                entry.complementary.len()
            );
        }
    }
}
