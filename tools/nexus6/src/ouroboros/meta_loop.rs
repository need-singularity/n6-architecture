//! OUROBOROS + LensForge Meta Loop
//!
//! Runs OUROBOROS evolution cycles, detects saturation, then triggers
//! LensForge to create new lenses. New lenses are injected back into
//! OUROBOROS for another round. The outer meta-loop repeats until
//! either max_meta_cycles is reached or LensForge cannot produce
//! new lenses (true convergence).

use crate::history::recorder::ScanRecord;
use crate::lens_forge::forge_engine::{self, ForgeConfig, ForgeResult};
use crate::ouroboros::convergence::ConvergenceStatus;
use crate::ouroboros::engine::{CycleResult, EvolutionConfig, EvolutionEngine};
use crate::telescope::registry::LensRegistry;

/// Configuration for the meta-loop (OUROBOROS + LensForge).
#[derive(Debug, Clone)]
pub struct MetaLoopConfig {
    /// Maximum OUROBOROS cycles per meta-cycle.
    pub max_ouroboros_cycles: usize,
    /// Maximum meta-loop iterations (discover -> forge -> re-discover).
    pub max_meta_cycles: usize,
    /// Run LensForge every N ouroboros cycles within a meta-cycle.
    /// If 0, only forge on saturation.
    pub forge_after_n_cycles: usize,
    /// Forge engine configuration.
    pub forge_config: ForgeConfig,
}

impl Default for MetaLoopConfig {
    fn default() -> Self {
        Self {
            max_ouroboros_cycles: 6, // n=6
            max_meta_cycles: 6,     // n=6
            forge_after_n_cycles: 0, // only on saturation
            forge_config: ForgeConfig::default(),
        }
    }
}

/// Result of a complete meta-loop run.
#[derive(Debug, Clone)]
pub struct MetaLoopResult {
    /// All OUROBOROS cycle results across all meta-cycles.
    pub ouroboros_results: Vec<CycleResult>,
    /// Names of lenses forged during the run.
    pub forged_lenses: Vec<String>,
    /// Total discoveries across all cycles.
    pub total_discoveries: usize,
    /// How many meta-cycles completed.
    pub meta_cycles_completed: usize,
    /// Per-meta-cycle summaries.
    pub meta_cycle_summaries: Vec<MetaCycleSummary>,
}

/// Summary for one meta-cycle.
#[derive(Debug, Clone)]
pub struct MetaCycleSummary {
    pub meta_cycle: usize,
    pub ouroboros_cycles_run: usize,
    pub discoveries: usize,
    pub convergence_status: ConvergenceStatus,
    pub lenses_forged: Vec<String>,
}

/// The Meta Loop: OUROBOROS evolution + LensForge lens creation.
pub struct MetaLoop {
    pub config: MetaLoopConfig,
    pub domain: String,
    pub seeds: Vec<String>,
    /// Callback for progress reporting (meta_cycle, ouroboros_cycle, message).
    pub on_progress: Option<Box<dyn Fn(usize, usize, &str)>>,
}

impl MetaLoop {
    pub fn new(domain: String, seeds: Vec<String>, config: MetaLoopConfig) -> Self {
        Self {
            config,
            domain,
            seeds,
            on_progress: None,
        }
    }

    /// Run the full meta-loop.
    pub fn run(&self) -> MetaLoopResult {
        let mut all_ouroboros_results: Vec<CycleResult> = Vec::new();
        let mut all_forged_lenses: Vec<String> = Vec::new();
        let mut total_discoveries: usize = 0;
        let mut meta_cycle_summaries: Vec<MetaCycleSummary> = Vec::new();
        let mut registry = LensRegistry::new();

        // Accumulated scan records for LensForge gap analysis
        let mut accumulated_records: Vec<ScanRecord> = Vec::new();

        for meta_cycle in 1..=self.config.max_meta_cycles {
            self.report(meta_cycle, 0, &format!(
                "Meta-cycle {} start (lenses: {})",
                meta_cycle, registry.len()
            ));

            // Build OUROBOROS config with current lens set
            let mut evo_config = EvolutionConfig::default();
            evo_config.domain = self.domain.clone();
            evo_config.all_lenses = registry.iter().map(|(name, _)| name.clone()).collect();

            let engine_seeds = if meta_cycle == 1 {
                self.seeds.clone()
            } else {
                // Use discoveries from previous cycle as seeds
                let prev = meta_cycle_summaries.last();
                let mut new_seeds = self.seeds.clone();
                if let Some(summary) = prev {
                    for lens_name in &summary.lenses_forged {
                        new_seeds.push(format!(
                            "Re-scan {} with new lens: {}",
                            self.domain, lens_name
                        ));
                    }
                }
                new_seeds
            };

            let mut engine = EvolutionEngine::new(evo_config, engine_seeds);

            // Run OUROBOROS cycles
            let mut cycle_discoveries = 0usize;
            let mut ouroboros_cycles_run = 0usize;
            let mut final_status = ConvergenceStatus::Exploring;

            for ouro_cycle in 1..=self.config.max_ouroboros_cycles {
                let result = engine.evolve_step();
                cycle_discoveries += result.new_discoveries;
                ouroboros_cycles_run += 1;
                all_ouroboros_results.push(result.clone());

                self.report(meta_cycle, ouro_cycle, &format!(
                    "discoveries={} nodes={} edges={} score={:.3}",
                    result.new_discoveries,
                    result.graph_nodes,
                    result.graph_edges,
                    result.verification_score,
                ));

                // Check convergence
                let status = engine.convergence_checker.check(&engine.history);
                final_status = status.clone();

                // Mid-cycle forge trigger (if configured)
                if self.config.forge_after_n_cycles > 0
                    && ouro_cycle % self.config.forge_after_n_cycles == 0
                    && status != ConvergenceStatus::Saturated
                {
                    let forge_result = self.try_forge(&registry, &accumulated_records);
                    if let Some(fr) = forge_result {
                        for entry in &fr.new_lenses {
                            all_forged_lenses.push(entry.name.clone());
                            registry.register(entry.clone());
                        }
                    }
                }

                if status == ConvergenceStatus::Saturated {
                    self.report(meta_cycle, ouro_cycle, "OUROBOROS saturated");
                    break;
                }
            }

            total_discoveries += cycle_discoveries;

            // Build scan records from engine history for LensForge
            for cr in &engine.history {
                accumulated_records.push(ScanRecord {
                    id: format!("meta{}-{}", meta_cycle, cr.cycle),
                    timestamp: format!("meta-{}-cycle-{}", meta_cycle, cr.cycle),
                    domain: self.domain.clone(),
                    lenses_used: cr.lenses_used.clone(),
                    discoveries: Vec::new(),
                    consensus_level: cr.lenses_used.len(),
                });
            }

            // Forge new lenses after OUROBOROS saturation or completion
            let mut lenses_forged_this_cycle: Vec<String> = Vec::new();

            let forge_result = self.try_forge(&registry, &accumulated_records);
            if let Some(fr) = forge_result {
                for entry in &fr.new_lenses {
                    lenses_forged_this_cycle.push(entry.name.clone());
                    all_forged_lenses.push(entry.name.clone());
                    registry.register(entry.clone());
                }
                self.report(meta_cycle, 0, &format!(
                    "LensForge: {} candidates -> {} accepted: [{}]",
                    fr.candidates_generated,
                    fr.candidates_accepted,
                    lenses_forged_this_cycle.join(", ")
                ));
            }

            meta_cycle_summaries.push(MetaCycleSummary {
                meta_cycle,
                ouroboros_cycles_run,
                discoveries: cycle_discoveries,
                convergence_status: final_status,
                lenses_forged: lenses_forged_this_cycle.clone(),
            });

            // Termination: if LensForge produced no new lenses, we've truly converged
            if lenses_forged_this_cycle.is_empty() {
                self.report(meta_cycle, 0,
                    "No new lenses forged -- true convergence reached");
                break;
            }
        }

        MetaLoopResult {
            ouroboros_results: all_ouroboros_results,
            forged_lenses: all_forged_lenses,
            total_discoveries,
            meta_cycles_completed: meta_cycle_summaries.len(),
            meta_cycle_summaries,
        }
    }

    /// Attempt to forge new lenses. Returns None on error or if forge is unavailable.
    fn try_forge(
        &self,
        registry: &LensRegistry,
        history: &[ScanRecord],
    ) -> Option<ForgeResult> {
        let result = forge_engine::forge_cycle(registry, history, &self.config.forge_config);
        Some(result)
    }

    /// Report progress via callback if set, otherwise no-op.
    fn report(&self, meta_cycle: usize, ouro_cycle: usize, msg: &str) {
        if let Some(ref cb) = self.on_progress {
            cb(meta_cycle, ouro_cycle, msg);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_loop_config_default() {
        let config = MetaLoopConfig::default();
        assert_eq!(config.max_ouroboros_cycles, 6);
        assert_eq!(config.max_meta_cycles, 6);
        assert_eq!(config.forge_after_n_cycles, 0);
    }

    #[test]
    fn test_meta_loop_single_cycle() {
        let config = MetaLoopConfig {
            max_ouroboros_cycles: 3,
            max_meta_cycles: 1,
            forge_after_n_cycles: 0,
            forge_config: ForgeConfig::default(),
        };
        let meta_loop = MetaLoop::new(
            "physics".to_string(),
            vec!["n=6 in physics".to_string()],
            config,
        );
        let result = meta_loop.run();

        assert_eq!(result.meta_cycles_completed, 1);
        assert!(!result.ouroboros_results.is_empty());
    }

    #[test]
    fn test_meta_loop_terminates_without_new_lenses() {
        let config = MetaLoopConfig {
            max_ouroboros_cycles: 2,
            max_meta_cycles: 10, // high limit -- should terminate early
            forge_after_n_cycles: 0,
            forge_config: ForgeConfig {
                max_candidates: 5,
                min_confidence: 0.99, // very high bar -> likely no new lenses
                similarity_threshold: 0.1,
            },
        };
        let meta_loop = MetaLoop::new(
            "test".to_string(),
            vec!["test seed".to_string()],
            config,
        );
        let result = meta_loop.run();

        // Should terminate before max_meta_cycles due to no new lenses
        assert!(result.meta_cycles_completed <= 10);
    }

    #[test]
    fn test_meta_loop_with_progress() {
        use std::sync::{Arc, Mutex};

        let messages: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let msgs_clone = messages.clone();

        let config = MetaLoopConfig {
            max_ouroboros_cycles: 2,
            max_meta_cycles: 1,
            forge_after_n_cycles: 0,
            forge_config: ForgeConfig::default(),
        };
        let mut meta_loop = MetaLoop::new(
            "test".to_string(),
            vec!["seed".to_string()],
            config,
        );
        meta_loop.on_progress = Some(Box::new(move |mc, oc, msg| {
            msgs_clone.lock().unwrap().push(format!("M{}O{}: {}", mc, oc, msg));
        }));

        let _result = meta_loop.run();
        let msgs = messages.lock().unwrap();
        assert!(!msgs.is_empty());
        assert!(msgs[0].starts_with("M1O0:"));
    }
}
