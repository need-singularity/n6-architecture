use std::collections::HashMap;

use crate::graph::persistence::DiscoveryGraph;
use crate::graph::node::{Node, NodeType};
use crate::graph::edge::{Edge, EdgeType};
use crate::history::recorder::ScanRecord;
use crate::history::stats;
use crate::history::recommend;
use crate::telescope::Telescope;
use crate::verifier::feasibility;
use crate::verifier::n6_check;

use super::convergence::{ConvergenceChecker, ConvergenceStatus};
use super::mutation;

/// Result of one evolution cycle.
#[derive(Debug, Clone)]
pub struct CycleResult {
    pub cycle: usize,
    pub domain: String,
    pub lenses_used: Vec<String>,
    pub new_discoveries: usize,
    pub graph_nodes: usize,
    pub graph_edges: usize,
    pub verification_score: f64,
    pub lens_candidates_found: usize,
    pub lenses_evolved: usize,
}

/// Configuration for the evolution engine.
#[derive(Debug, Clone)]
pub struct EvolutionConfig {
    /// Domain to evolve within.
    pub domain: String,
    /// All available lens names.
    pub all_lenses: Vec<String>,
    /// Serendipity ratio for lens recommendation (0.0..1.0).
    pub serendipity_ratio: f64,
    /// Minimum verification score to accept a discovery.
    pub min_verification_score: f64,
    /// Maximum mutations to generate per cycle.
    pub max_mutations_per_cycle: usize,
}

impl Default for EvolutionConfig {
    fn default() -> Self {
        Self {
            domain: "general".to_string(),
            all_lenses: vec![
                "consciousness".into(),
                "topology".into(),
                "causal".into(),
                "void".into(),
                "barrier".into(),
                "thermo".into(),
                "wave".into(),
                "evolution".into(),
                "network".into(),
                "boundary".into(),
                "quantum".into(),
                "gravity".into(),
            ],
            serendipity_ratio: 0.2,
            min_verification_score: 0.3,
            max_mutations_per_cycle: 6, // n=6
        }
    }
}

/// The OUROBOROS v26 Evolution Engine.
///
/// Runs scan → verify → graph_update → recommend cycles,
/// generating and testing hypothesis mutations until convergence.
pub struct EvolutionEngine {
    pub config: EvolutionConfig,
    pub graph: DiscoveryGraph,
    pub history: Vec<CycleResult>,
    pub convergence_checker: ConvergenceChecker,
    telescope: Telescope,
    /// Accumulated scan records (in-memory, keyed by domain).
    scan_records: HashMap<String, Vec<ScanRecord>>,
    /// Seed hypotheses to start evolution from.
    seed_hypotheses: Vec<String>,
    /// Current cycle counter.
    cycle_count: usize,
}

impl EvolutionEngine {
    /// Create a new engine with the given config and optional seed hypotheses.
    pub fn new(config: EvolutionConfig, seed_hypotheses: Vec<String>) -> Self {
        Self {
            config,
            graph: DiscoveryGraph::new(),
            history: Vec::new(),
            convergence_checker: ConvergenceChecker::default(),
            telescope: Telescope::new(),
            scan_records: HashMap::new(),
            seed_hypotheses,
            cycle_count: 0,
        }
    }

    /// Execute one evolution cycle: scan → verify → graph_update → recommend.
    ///
    /// Returns the cycle result.
    pub fn evolve_step(&mut self) -> CycleResult {
        self.cycle_count += 1;
        let cycle = self.cycle_count;

        // a. Get domain stats from accumulated records
        let domain_records = self.scan_records
            .get(&self.config.domain)
            .cloned()
            .unwrap_or_default();
        let domain_stats = stats::compute_domain_stats(&domain_records);
        let mut all_stats = HashMap::new();
        all_stats.insert(self.config.domain.clone(), domain_stats);

        // b. Recommend lenses
        let recommendation = recommend::recommend_lenses(
            &self.config.domain,
            &all_stats,
            &self.config.all_lenses,
            self.config.serendipity_ratio,
        );
        let lenses_used = recommendation.lenses.clone();

        // c. Generate hypotheses to test (from seeds + mutations of previous discoveries)
        let hypotheses = self.generate_hypotheses();

        // d. Telescope scan (real scan with mock data derived from hypotheses)
        let scan_data = self.hypotheses_to_scan_data(&hypotheses);
        let scan_results = self.telescope.scan_all(
            &scan_data,
            scan_data.len().max(1),
            1,
        );

        // e. Verify discoveries
        let mut new_discoveries = 0usize;
        let mut total_score = 0.0f64;
        let mut verified_count = 0usize;
        let mut discovered_summaries = Vec::new();

        for hypothesis in &hypotheses {
            // Compute n6 alignment from scan data
            let n6_ratio = n6_check::n6_exact_ratio(&scan_data);

            // Compute a lens consensus score based on how many lenses returned results
            let active_lenses = scan_results.values()
                .filter(|lr| !lr.is_empty())
                .count();
            let lens_consensus = if self.telescope.lens_count() > 0 {
                active_lenses as f64 / self.telescope.lens_count() as f64
            } else {
                0.0
            };

            // Novelty: inversely proportional to cycle count (diminishing returns)
            let novelty = 1.0 / (cycle as f64).sqrt();

            // Graph bonus: based on current graph connectivity
            let graph_bonus = if self.graph.nodes.is_empty() {
                0.0
            } else {
                let hubs = self.graph.hubs(2);
                (hubs.len() as f64 * 0.1).min(1.0)
            };

            let verification = feasibility::verify(
                lens_consensus,
                0.5, // cross_validation placeholder
                0.5, // physical_check placeholder
                graph_bonus,
                novelty,
                n6_ratio,
            );

            total_score += verification.score;
            verified_count += 1;

            if verification.score >= self.config.min_verification_score {
                new_discoveries += 1;
                discovered_summaries.push(hypothesis.clone());

                // e. Add to graph
                let node_id = format!("ouroboros-c{}-d{}", cycle, new_discoveries);
                self.graph.add_node(Node {
                    id: node_id.clone(),
                    node_type: NodeType::Hypothesis,
                    domain: self.config.domain.clone(),
                    project: "nexus".to_string(),
                    summary: truncate(hypothesis, 120),
                    confidence: verification.score,
                    lenses_used: lenses_used.clone(),
                    timestamp: format!("cycle-{}", cycle),
                });

                // Connect to previous cycle's discoveries if any
                if cycle > 1 {
                    let prev_prefix = format!("ouroboros-c{}-", cycle - 1);
                    let prev_nodes: Vec<String> = self.graph.nodes.iter()
                        .filter(|n| n.id.starts_with(&prev_prefix))
                        .map(|n| n.id.clone())
                        .collect();
                    for prev_id in prev_nodes.iter().take(3) {
                        self.graph.add_edge(Edge {
                            from: prev_id.clone(),
                            to: node_id.clone(),
                            edge_type: EdgeType::Derives,
                            strength: verification.score,
                            bidirectional: false,
                        });
                    }
                }
            }
        }

        let avg_score = if verified_count > 0 {
            total_score / verified_count as f64
        } else {
            0.0
        };

        // f. Record scan in history
        let scan_record = ScanRecord {
            id: format!("ouroboros-scan-{}", cycle),
            timestamp: format!("cycle-{}", cycle),
            domain: self.config.domain.clone(),
            lenses_used: lenses_used.clone(),
            discoveries: discovered_summaries,
            consensus_level: lenses_used.len(),
        };

        self.scan_records
            .entry(self.config.domain.clone())
            .or_default()
            .push(scan_record);

        let result = CycleResult {
            cycle,
            domain: self.config.domain.clone(),
            lenses_used,
            new_discoveries,
            graph_nodes: self.graph.nodes.len(),
            graph_edges: self.graph.edges.len(),
            verification_score: avg_score,
            lens_candidates_found: 0,
            lenses_evolved: 0,
        };

        self.history.push(result.clone());

        result
    }

    /// Run the evolution loop for up to max_iterations cycles.
    ///
    /// Stops early if the convergence checker signals Saturated or if no
    /// new discoveries are being made.
    ///
    /// Returns the final convergence status and the full cycle history.
    pub fn run_loop(&mut self, max_iterations: usize) -> (ConvergenceStatus, Vec<CycleResult>) {
        for _ in 0..max_iterations {
            self.evolve_step();

            let status = self.convergence_checker.check(&self.history);
            if status == ConvergenceStatus::Saturated {
                return (status, self.history.clone());
            }
        }

        let final_status = self.convergence_checker.check(&self.history);
        (final_status, self.history.clone())
    }

    /// Generate hypotheses for the current cycle.
    ///
    /// Sources:
    /// 1. Seed hypotheses (first cycle only)
    /// 2. Mutations of previous discoveries
    /// 3. Mutations of graph node summaries
    fn generate_hypotheses(&self) -> Vec<String> {
        let mut hypotheses = Vec::new();

        // First cycle: use seed hypotheses
        if self.cycle_count == 1 {
            hypotheses.extend(self.seed_hypotheses.clone());
        }

        // Mutate from recent discoveries (last cycle's scan records)
        if let Some(records) = self.scan_records.get(&self.config.domain) {
            if let Some(last_record) = records.last() {
                for discovery in last_record.discoveries.iter().take(self.config.max_mutations_per_cycle) {
                    let mutations = mutation::mutate_hypothesis(discovery);
                    // Take limited mutations per discovery to avoid explosion
                    hypotheses.extend(mutations.into_iter().take(2));
                }
            }
        }

        // Mutate from graph node summaries (high-confidence nodes)
        let mut high_conf_nodes: Vec<&Node> = self.graph.nodes.iter()
            .filter(|n| n.confidence >= 0.5)
            .collect();
        high_conf_nodes.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));

        for node in high_conf_nodes.iter().take(3) {
            let mutations = mutation::mutate_hypothesis(&node.summary);
            hypotheses.extend(mutations.into_iter().take(1));
        }

        // Ensure at least one hypothesis per cycle
        if hypotheses.is_empty() {
            hypotheses.push(format!(
                "n=6 pattern in {} domain: sigma*phi=n*tau identity may constrain key parameters",
                self.config.domain,
            ));
        }

        // Limit total
        hypotheses.truncate(self.config.max_mutations_per_cycle * 6);

        hypotheses
    }

    /// Convert hypotheses into numeric scan data for telescope.
    ///
    /// Uses a simple deterministic hash of hypothesis text to generate
    /// data points that include n=6 constants for alignment testing.
    fn hypotheses_to_scan_data(&self, hypotheses: &[String]) -> Vec<f64> {
        let mut data = Vec::new();

        // Always include core n=6 constants as base data
        data.extend_from_slice(&[6.0, 12.0, 2.0, 4.0, 24.0, 5.0]);

        // Add hypothesis-derived values
        for h in hypotheses {
            let hash: usize = h.bytes().map(|b| b as usize).sum();
            let value = (hash % 100) as f64 + 0.5;
            data.push(value);
        }

        data
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        // Find a char boundary at or before max_len to avoid panicking on multi-byte chars
        let mut end = max_len;
        while end > 0 && !s.is_char_boundary(end) {
            end -= 1;
        }
        format!("{}...", &s[..end])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let config = EvolutionConfig::default();
        let engine = EvolutionEngine::new(config, vec!["test hypothesis".into()]);
        assert_eq!(engine.cycle_count, 0);
        assert!(engine.history.is_empty());
        assert!(engine.graph.nodes.is_empty());
    }

    #[test]
    fn test_single_step() {
        let config = EvolutionConfig::default();
        let engine_seeds = vec!["sigma=12 heads in transformer".into()];
        let mut engine = EvolutionEngine::new(config, engine_seeds);

        let result = engine.evolve_step();
        assert_eq!(result.cycle, 1);
        assert_eq!(result.domain, "general");
        assert!(!result.lenses_used.is_empty());
        assert_eq!(engine.history.len(), 1);
    }

    #[test]
    fn test_hypotheses_to_scan_data() {
        let config = EvolutionConfig::default();
        let engine = EvolutionEngine::new(config, vec![]);
        let data = engine.hypotheses_to_scan_data(&["test".into()]);
        // Should contain at least the 6 base constants + 1 hypothesis value
        assert!(data.len() >= 7);
        // First 6 should be n=6 constants
        assert_eq!(data[0], 6.0);
        assert_eq!(data[1], 12.0);
        assert_eq!(data[2], 2.0);
        assert_eq!(data[3], 4.0);
        assert_eq!(data[4], 24.0);
        assert_eq!(data[5], 5.0);
    }
}
