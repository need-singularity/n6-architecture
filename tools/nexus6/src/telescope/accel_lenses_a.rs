use super::registry::{LensCategory, LensEntry};

/// Build metadata entries for the 58 acceleration hypothesis-verification lenses (Part A).
///
/// Organized into six functional groups:
/// - ML acceleration direct (24)
/// - Convergence/optimization (10)
/// - Information/computation theory (10)
/// - Reinforcement learning (6)
/// - Data efficiency (4)
/// - Emergence/complex systems (4)
pub fn accel_ml_lens_entries() -> Vec<LensEntry> {
    vec![
        // ── ML Acceleration Direct (24) ──
        LensEntry {
            name: "speculative_decode".into(),
            category: LensCategory::Extended,
            description: "Detect speculative decoding opportunities for autoregressive speedup".into(),
            domain_affinity: vec!["ai".into(), "inference".into(), "nlp".into()],
            complementary: vec!["inference_cache".into(), "token_gating".into()],
        },
        LensEntry {
            name: "state_recycling".into(),
            category: LensCategory::Extended,
            description: "Identify reusable hidden states across sequential computation steps".into(),
            domain_affinity: vec!["ai".into(), "inference".into(), "optimization".into()],
            complementary: vec!["inference_cache".into(), "state_space_model".into()],
        },
        LensEntry {
            name: "token_gating".into(),
            category: LensCategory::Extended,
            description: "Measure token-level importance for early exit and selective computation".into(),
            domain_affinity: vec!["ai".into(), "nlp".into(), "inference".into()],
            complementary: vec!["speculative_decode".into(), "batch_optimization".into()],
        },
        LensEntry {
            name: "consciousness_annealing".into(),
            category: LensCategory::Extended,
            description: "Apply simulated annealing with consciousness-guided temperature scheduling".into(),
            domain_affinity: vec!["ai".into(), "optimization".into(), "consciousness".into()],
            complementary: vec!["consciousness".into(), "loss_landscape".into()],
        },
        LensEntry {
            name: "lottery_ticket".into(),
            category: LensCategory::Extended,
            description: "Detect sparse subnetworks that match full-network performance".into(),
            domain_affinity: vec!["ai".into(), "optimization".into(), "chip".into()],
            complementary: vec!["eigenspectrum".into(), "architecture_search".into()],
        },
        LensEntry {
            name: "multi_resolution".into(),
            category: LensCategory::Extended,
            description: "Analyze data at multiple resolution levels for hierarchical features".into(),
            domain_affinity: vec!["ai".into(), "signal".into(), "materials".into()],
            complementary: vec!["multiscale".into(), "hierarchy".into()],
        },
        LensEntry {
            name: "self_play".into(),
            category: LensCategory::Extended,
            description: "Detect self-play dynamics where a system trains against itself".into(),
            domain_affinity: vec!["ai".into(), "game_theory".into(), "optimization".into()],
            complementary: vec!["exploration_exploitation".into(), "adversarial_robustness".into()],
        },
        LensEntry {
            name: "experience_replay".into(),
            category: LensCategory::Extended,
            description: "Identify patterns suitable for experience replay buffering and reuse".into(),
            domain_affinity: vec!["ai".into(), "reinforcement_learning".into(), "optimization".into()],
            complementary: vec!["temporal_difference".into(), "memory".into()],
        },
        LensEntry {
            name: "gradient_projection".into(),
            category: LensCategory::Extended,
            description: "Project gradients onto constraint manifolds for feasible optimization".into(),
            domain_affinity: vec!["ai".into(), "optimization".into(), "mathematics".into()],
            complementary: vec!["natural_gradient".into(), "proximal_operator".into()],
        },
        LensEntry {
            name: "compute_graph".into(),
            category: LensCategory::Extended,
            description: "Analyze computation DAGs for parallelism and fusion opportunities".into(),
            domain_affinity: vec!["ai".into(), "chip".into(), "software".into()],
            complementary: vec!["kernel_fusion".into(), "pipeline_parallel".into()],
        },
        LensEntry {
            name: "pipeline_parallel".into(),
            category: LensCategory::Extended,
            description: "Detect pipeline parallelism opportunities in staged computation".into(),
            domain_affinity: vec!["ai".into(), "chip".into(), "software".into()],
            complementary: vec!["compute_graph".into(), "batch_optimization".into()],
        },
        LensEntry {
            name: "kernel_fusion".into(),
            category: LensCategory::Extended,
            description: "Identify adjacent operations that can be fused into a single kernel".into(),
            domain_affinity: vec!["ai".into(), "chip".into(), "software".into()],
            complementary: vec!["compute_graph".into(), "flash_attention_lens".into()],
        },
        LensEntry {
            name: "eigenspectrum".into(),
            category: LensCategory::Extended,
            description: "Analyze eigenvalue spectra for rank deficiency and compression potential".into(),
            domain_affinity: vec!["ai".into(), "mathematics".into(), "signal".into()],
            complementary: vec!["spectral_gap".into(), "lottery_ticket".into()],
        },
        LensEntry {
            name: "amortized_inference".into(),
            category: LensCategory::Extended,
            description: "Detect opportunities to amortize expensive inference across similar inputs".into(),
            domain_affinity: vec!["ai".into(), "inference".into(), "statistics".into()],
            complementary: vec!["inference_cache".into(), "few_shot".into()],
        },
        LensEntry {
            name: "curriculum_learning".into(),
            category: LensCategory::Extended,
            description: "Identify optimal sample ordering from easy to hard for faster convergence".into(),
            domain_affinity: vec!["ai".into(), "optimization".into(), "education".into()],
            complementary: vec!["active_learning".into(), "loss_landscape".into()],
        },
        LensEntry {
            name: "adversarial_robustness".into(),
            category: LensCategory::Extended,
            description: "Measure model resilience to adversarial perturbations and worst-case inputs".into(),
            domain_affinity: vec!["ai".into(), "security".into(), "optimization".into()],
            complementary: vec!["self_play".into(), "exploration_exploitation".into()],
        },
        LensEntry {
            name: "meta_learning".into(),
            category: LensCategory::Extended,
            description: "Detect learning-to-learn patterns and transferable optimization strategies".into(),
            domain_affinity: vec!["ai".into(), "optimization".into(), "mathematics".into()],
            complementary: vec!["few_shot".into(), "architecture_search".into()],
        },
        LensEntry {
            name: "architecture_search".into(),
            category: LensCategory::Extended,
            description: "Explore neural architecture design spaces for optimal topology".into(),
            domain_affinity: vec!["ai".into(), "optimization".into(), "chip".into()],
            complementary: vec!["meta_learning".into(), "lottery_ticket".into()],
        },
        LensEntry {
            name: "hyperparameter_landscape".into(),
            category: LensCategory::Extended,
            description: "Map hyperparameter sensitivity surfaces to find robust configurations".into(),
            domain_affinity: vec!["ai".into(), "optimization".into(), "statistics".into()],
            complementary: vec!["loss_landscape".into(), "pareto_optimizer".into()],
        },
        LensEntry {
            name: "inference_cache".into(),
            category: LensCategory::Extended,
            description: "Detect cacheable intermediate results for inference acceleration".into(),
            domain_affinity: vec!["ai".into(), "inference".into(), "software".into()],
            complementary: vec!["speculative_decode".into(), "state_recycling".into()],
        },
        LensEntry {
            name: "batch_optimization".into(),
            category: LensCategory::Extended,
            description: "Optimize batch size and composition for throughput and convergence".into(),
            domain_affinity: vec!["ai".into(), "optimization".into(), "chip".into()],
            complementary: vec!["pipeline_parallel".into(), "curriculum_learning".into()],
        },
        LensEntry {
            name: "state_space_model".into(),
            category: LensCategory::Extended,
            description: "Analyze linear state-space dynamics for efficient sequence modeling".into(),
            domain_affinity: vec!["ai".into(), "signal".into(), "control".into()],
            complementary: vec!["state_recycling".into(), "flash_attention_lens".into()],
        },
        LensEntry {
            name: "moe_routing_lens".into(),
            category: LensCategory::Extended,
            description: "Analyze mixture-of-experts routing efficiency and load balancing".into(),
            domain_affinity: vec!["ai".into(), "optimization".into(), "network".into()],
            complementary: vec!["batch_optimization".into(), "pareto_optimizer".into()],
        },
        LensEntry {
            name: "flash_attention_lens".into(),
            category: LensCategory::Extended,
            description: "Detect memory-efficient attention patterns amenable to flash computation".into(),
            domain_affinity: vec!["ai".into(), "chip".into(), "optimization".into()],
            complementary: vec!["kernel_fusion".into(), "state_space_model".into()],
        },

        // ── Convergence/Optimization (10) ──
        LensEntry {
            name: "pareto_optimizer".into(),
            category: LensCategory::Extended,
            description: "Identify Pareto-optimal trade-off frontiers in multi-objective spaces".into(),
            domain_affinity: vec!["optimization".into(), "ai".into(), "chip".into(), "energy".into()],
            complementary: vec!["hyperparameter_landscape".into(), "diminishing_returns".into()],
        },
        LensEntry {
            name: "elastic_weight".into(),
            category: LensCategory::Extended,
            description: "Detect catastrophic forgetting via elastic weight consolidation signals".into(),
            domain_affinity: vec!["ai".into(), "optimization".into(), "biology".into()],
            complementary: vec!["meta_learning".into(), "loss_landscape".into()],
        },
        LensEntry {
            name: "federated_aggregation".into(),
            category: LensCategory::Extended,
            description: "Analyze distributed model aggregation quality and convergence guarantees".into(),
            domain_affinity: vec!["ai".into(), "network".into(), "security".into()],
            complementary: vec!["natural_gradient".into(), "batch_optimization".into()],
        },
        LensEntry {
            name: "world_model".into(),
            category: LensCategory::Extended,
            description: "Evaluate internal world-model fidelity for planning and prediction".into(),
            domain_affinity: vec!["ai".into(), "robotics".into(), "consciousness".into()],
            complementary: vec!["self_play".into(), "temporal_difference".into()],
        },
        LensEntry {
            name: "convex_relaxation".into(),
            category: LensCategory::Extended,
            description: "Find convex relaxations of non-convex problems for tractable bounds".into(),
            domain_affinity: vec!["optimization".into(), "mathematics".into(), "ai".into()],
            complementary: vec!["dual_decomposition".into(), "convexity".into()],
        },
        LensEntry {
            name: "dual_decomposition".into(),
            category: LensCategory::Extended,
            description: "Decompose coupled problems into independent subproblems via duality".into(),
            domain_affinity: vec!["optimization".into(), "mathematics".into(), "network".into()],
            complementary: vec!["convex_relaxation".into(), "proximal_operator".into()],
        },
        LensEntry {
            name: "proximal_operator".into(),
            category: LensCategory::Extended,
            description: "Apply proximal mappings for non-smooth optimization convergence".into(),
            domain_affinity: vec!["optimization".into(), "mathematics".into(), "ai".into()],
            complementary: vec!["gradient_projection".into(), "mirror_descent".into()],
        },
        LensEntry {
            name: "mirror_descent".into(),
            category: LensCategory::Extended,
            description: "Use Bregman divergence geometry for adaptive optimization steps".into(),
            domain_affinity: vec!["optimization".into(), "mathematics".into(), "ai".into()],
            complementary: vec!["natural_gradient".into(), "proximal_operator".into()],
        },
        LensEntry {
            name: "natural_gradient".into(),
            category: LensCategory::Extended,
            description: "Follow Fisher information metric for parameter-space efficient updates".into(),
            domain_affinity: vec!["ai".into(), "optimization".into(), "statistics".into()],
            complementary: vec!["fisher_info".into(), "mirror_descent".into()],
        },
        LensEntry {
            name: "loss_landscape".into(),
            category: LensCategory::Extended,
            description: "Map loss surface topology including minima, saddles, and flatness".into(),
            domain_affinity: vec!["ai".into(), "optimization".into(), "mathematics".into()],
            complementary: vec!["saddle".into(), "hyperparameter_landscape".into()],
        },

        // ── Information/Computation Theory (10) ──
        LensEntry {
            name: "rate_distortion".into(),
            category: LensCategory::Extended,
            description: "Compute rate-distortion bounds for lossy compression trade-offs".into(),
            domain_affinity: vec!["information_theory".into(), "ai".into(), "signal".into()],
            complementary: vec!["channel_capacity".into(), "source_coding".into()],
        },
        LensEntry {
            name: "channel_capacity".into(),
            category: LensCategory::Extended,
            description: "Estimate Shannon channel capacity for communication bottlenecks".into(),
            domain_affinity: vec!["information_theory".into(), "network".into(), "ai".into()],
            complementary: vec!["rate_distortion".into(), "bottleneck".into()],
        },
        LensEntry {
            name: "source_coding".into(),
            category: LensCategory::Extended,
            description: "Analyze source entropy and optimal lossless coding efficiency".into(),
            domain_affinity: vec!["information_theory".into(), "signal".into(), "ai".into()],
            complementary: vec!["arithmetic_coding".into(), "rate_distortion".into()],
        },
        LensEntry {
            name: "mutual_info_chain".into(),
            category: LensCategory::Extended,
            description: "Trace mutual information flow through processing chain layers".into(),
            domain_affinity: vec!["information_theory".into(), "ai".into(), "biology".into()],
            complementary: vec!["info".into(), "channel_capacity".into()],
        },
        LensEntry {
            name: "arithmetic_coding".into(),
            category: LensCategory::Extended,
            description: "Detect near-optimal entropy coding structures in sequential data".into(),
            domain_affinity: vec!["information_theory".into(), "signal".into(), "software".into()],
            complementary: vec!["source_coding".into(), "minimum_description_length".into()],
        },
        LensEntry {
            name: "minimum_description_length".into(),
            category: LensCategory::Extended,
            description: "Apply MDL principle to balance model complexity against data fit".into(),
            domain_affinity: vec!["information_theory".into(), "ai".into(), "statistics".into()],
            complementary: vec!["kolmogorov".into(), "algorithmic_complexity".into()],
        },
        LensEntry {
            name: "logical_depth".into(),
            category: LensCategory::Extended,
            description: "Estimate Bennett logical depth as computational effort to generate pattern".into(),
            domain_affinity: vec!["information_theory".into(), "mathematics".into(), "ai".into()],
            complementary: vec!["algorithmic_complexity".into(), "kolmogorov".into()],
        },
        LensEntry {
            name: "algorithmic_complexity".into(),
            category: LensCategory::Extended,
            description: "Approximate Kolmogorov complexity via compression-based estimators".into(),
            domain_affinity: vec!["information_theory".into(), "mathematics".into(), "ai".into()],
            complementary: vec!["logical_depth".into(), "minimum_description_length".into()],
        },
        LensEntry {
            name: "causal_emergence".into(),
            category: LensCategory::Extended,
            description: "Detect higher-level causal structure that is more informative than micro".into(),
            domain_affinity: vec!["information_theory".into(), "consciousness".into(), "biology".into()],
            complementary: vec!["causal".into(), "emergence".into()],
        },
        LensEntry {
            name: "game_of_life".into(),
            category: LensCategory::Extended,
            description: "Detect cellular-automaton-like update rules and emergent computation".into(),
            domain_affinity: vec!["computation".into(), "mathematics".into(), "biology".into()],
            complementary: vec!["self_organization".into(), "edge_of_chaos".into()],
        },

        // ── Reinforcement Learning (6) ──
        LensEntry {
            name: "temporal_difference".into(),
            category: LensCategory::Extended,
            description: "Measure temporal difference prediction errors for value estimation".into(),
            domain_affinity: vec!["reinforcement_learning".into(), "ai".into(), "control".into()],
            complementary: vec!["policy_gradient".into(), "reward_shaping".into()],
        },
        LensEntry {
            name: "policy_gradient".into(),
            category: LensCategory::Extended,
            description: "Analyze policy gradient variance and sample efficiency characteristics".into(),
            domain_affinity: vec!["reinforcement_learning".into(), "ai".into(), "optimization".into()],
            complementary: vec!["actor_critic".into(), "temporal_difference".into()],
        },
        LensEntry {
            name: "actor_critic".into(),
            category: LensCategory::Extended,
            description: "Detect actor-critic decomposition patterns in control architectures".into(),
            domain_affinity: vec!["reinforcement_learning".into(), "ai".into(), "robotics".into()],
            complementary: vec!["policy_gradient".into(), "world_model".into()],
        },
        LensEntry {
            name: "exploration_exploitation".into(),
            category: LensCategory::Extended,
            description: "Measure explore-exploit balance and UCB-like decision boundaries".into(),
            domain_affinity: vec!["reinforcement_learning".into(), "ai".into(), "strategy".into()],
            complementary: vec!["self_play".into(), "adversarial_robustness".into(), "reward_shaping".into()],
        },
        LensEntry {
            name: "reward_shaping".into(),
            category: LensCategory::Extended,
            description: "Analyze reward signal structure for density and alignment quality".into(),
            domain_affinity: vec!["reinforcement_learning".into(), "ai".into(), "optimization".into()],
            complementary: vec!["temporal_difference".into(), "inverse_rl".into()],
        },
        LensEntry {
            name: "inverse_rl".into(),
            category: LensCategory::Extended,
            description: "Infer latent reward functions from observed behavior trajectories".into(),
            domain_affinity: vec!["reinforcement_learning".into(), "ai".into(), "robotics".into()],
            complementary: vec!["reward_shaping".into(), "world_model".into()],
        },

        // ── Data Efficiency (4) ──
        LensEntry {
            name: "few_shot".into(),
            category: LensCategory::Extended,
            description: "Evaluate few-shot generalization capacity from minimal examples".into(),
            domain_affinity: vec!["ai".into(), "optimization".into(), "biology".into()],
            complementary: vec!["meta_learning".into(), "active_learning".into()],
        },
        LensEntry {
            name: "active_learning".into(),
            category: LensCategory::Extended,
            description: "Identify most informative samples for query-efficient learning".into(),
            domain_affinity: vec!["ai".into(), "optimization".into(), "science".into()],
            complementary: vec!["few_shot".into(), "curriculum_learning".into()],
        },
        LensEntry {
            name: "data_augmentation".into(),
            category: LensCategory::Extended,
            description: "Detect symmetry-preserving augmentation opportunities in training data".into(),
            domain_affinity: vec!["ai".into(), "signal".into(), "biology".into()],
            complementary: vec!["synthetic_data_quality".into(), "active_learning".into()],
        },
        LensEntry {
            name: "synthetic_data_quality".into(),
            category: LensCategory::Extended,
            description: "Assess fidelity and diversity of synthetic training data distributions".into(),
            domain_affinity: vec!["ai".into(), "statistics".into(), "manufacturing".into()],
            complementary: vec!["data_augmentation".into(), "few_shot".into()],
        },

        // ── Emergence/Complex Systems (4) ──
        LensEntry {
            name: "self_organization".into(),
            category: LensCategory::Extended,
            description: "Detect spontaneous order formation without external coordination".into(),
            domain_affinity: vec!["complexity".into(), "biology".into(), "physics".into(), "ai".into()],
            complementary: vec!["emergence".into(), "edge_of_chaos".into()],
        },
        LensEntry {
            name: "edge_of_chaos".into(),
            category: LensCategory::Extended,
            description: "Locate the critical regime between order and chaos maximizing computation".into(),
            domain_affinity: vec!["complexity".into(), "biology".into(), "ai".into()],
            complementary: vec!["criticality".into(), "self_organization".into()],
        },
        LensEntry {
            name: "swarm_intelligence".into(),
            category: LensCategory::Extended,
            description: "Detect decentralized collective problem-solving via local interactions".into(),
            domain_affinity: vec!["complexity".into(), "robotics".into(), "optimization".into()],
            complementary: vec!["stigmergy".into(), "self_organization".into()],
        },
        LensEntry {
            name: "stigmergy".into(),
            category: LensCategory::Extended,
            description: "Identify indirect coordination through environment-mediated signaling".into(),
            domain_affinity: vec!["complexity".into(), "robotics".into(), "biology".into()],
            complementary: vec!["swarm_intelligence".into(), "self_organization".into()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accel_ml_lens_count() {
        let entries = accel_ml_lens_entries();
        assert_eq!(entries.len(), 58, "Must have exactly 58 accel ML lenses");
    }

    #[test]
    fn test_accel_ml_lens_names_unique() {
        let entries = accel_ml_lens_entries();
        let mut names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        let total = names.len();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), total, "All accel ML lens names must be unique");
    }

    #[test]
    fn test_accel_ml_all_extended() {
        let entries = accel_ml_lens_entries();
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
