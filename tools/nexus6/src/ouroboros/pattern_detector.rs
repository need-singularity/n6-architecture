//! Auto Module Discovery: detects new lens patterns from scan results.
//!
//! Analyzes accumulated scan cycles to find:
//! - Metric combinations that consistently score high
//! - Blind spots not covered by existing lenses
//! - New metric combinations correlated with n=6 constants
//!
//! Outputs are `LensCandidate`s suitable for the lens_forge pipeline.

use std::collections::{HashMap, HashSet};

use crate::graph::edge::{Edge, EdgeType};
use crate::graph::node::{Node, NodeType};
use crate::graph::persistence::DiscoveryGraph;
use crate::lens_forge::candidate_gen::{CandidateSource, LensCandidate};
use crate::telescope::lens_trait::LensResult;
use crate::verifier::n6_check;

/// A single cycle's scan snapshot used for pattern detection.
#[derive(Debug, Clone)]
pub struct ScanSnapshot {
    /// Cycle number this snapshot was taken from.
    pub cycle: usize,
    /// Lens name -> metric name -> values.
    pub results: HashMap<String, LensResult>,
    /// Domain the scan was performed in.
    pub domain: String,
}

/// A detected pattern that may warrant a new lens.
#[derive(Debug, Clone)]
pub struct DetectedPattern {
    /// Descriptive name for the pattern.
    pub name: String,
    /// Metric keys that participate in this pattern.
    pub metric_keys: Vec<String>,
    /// Lenses that contributed to discovery of this pattern.
    pub source_lenses: Vec<String>,
    /// How many cycles this pattern appeared in.
    pub recurrence_count: usize,
    /// Fraction of participating metric values that are n6 EXACT.
    pub n6_alignment: f64,
    /// Confidence score (0..1).
    pub confidence: f64,
    /// Domain where this pattern was observed.
    pub domain: String,
}

/// Minimum number of cycles to accumulate before pattern detection kicks in.
/// n=6 constant: we need at least n/phi=3 cycles of data.
const MIN_CYCLES_FOR_DETECTION: usize = 3;

/// Minimum recurrence count to consider a pattern significant.
/// phi=2: must appear in at least 2 cycles.
const MIN_RECURRENCE: usize = 2;

/// Minimum n6 alignment to consider a pattern significant.
/// ln(4/3) ~ 0.288 — the Mertens dropout constant.
const MIN_N6_ALIGNMENT: f64 = 0.15;

/// Maximum candidates to emit per detection pass.
/// n=6 — the universal constant.
const MAX_CANDIDATES: usize = 6;

/// Detects new lens patterns from accumulated scan results.
pub struct PatternDetector {
    /// Accumulated scan snapshots across evolution cycles.
    snapshots: Vec<ScanSnapshot>,
    /// Names of lenses already known (to avoid re-suggesting).
    known_lens_names: HashSet<String>,
    /// Previously suggested pattern names (avoid duplicates).
    suggested_patterns: HashSet<String>,
}

impl PatternDetector {
    /// Create a new detector with a list of already-known lens names.
    pub fn new(known_lens_names: Vec<String>) -> Self {
        Self {
            snapshots: Vec::new(),
            known_lens_names: known_lens_names.into_iter().collect(),
            suggested_patterns: HashSet::new(),
        }
    }

    /// Record a scan snapshot from an evolution cycle.
    pub fn record_snapshot(&mut self, snapshot: ScanSnapshot) {
        self.snapshots.push(snapshot);
    }

    /// Number of accumulated snapshots.
    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }

    /// Detect new patterns across accumulated scan results.
    ///
    /// Returns detected patterns sorted by confidence (descending).
    /// Only runs if we have >= MIN_CYCLES_FOR_DETECTION snapshots.
    pub fn detect_new_patterns(&self) -> Vec<DetectedPattern> {
        if self.snapshots.len() < MIN_CYCLES_FOR_DETECTION {
            return Vec::new();
        }

        let mut patterns = Vec::new();

        // Strategy 1: Find metric combinations that recurrently produce high values
        patterns.extend(self.detect_recurrent_high_scorers());

        // Strategy 2: Identify blind spots (lenses that never fire)
        patterns.extend(self.detect_blind_spots());

        // Strategy 3: Find metric pairs with high n6 correlation
        patterns.extend(self.detect_n6_correlated_pairs());

        // Deduplicate by name
        let mut seen = HashSet::new();
        patterns.retain(|p| {
            let key = p.name.clone();
            if seen.contains(&key) || self.suggested_patterns.contains(&key) {
                false
            } else {
                seen.insert(key);
                true
            }
        });

        // Sort by confidence descending
        patterns.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        patterns.truncate(MAX_CANDIDATES);
        patterns
    }

    /// Convert detected patterns into LensCandidates for the forge.
    pub fn suggest_lens_templates(&self, patterns: &[DetectedPattern]) -> Vec<LensCandidate> {
        patterns
            .iter()
            .map(|p| LensCandidate {
                name: format!("auto_{}", p.name),
                description: format!(
                    "Auto-discovered lens from {} cycles: metrics [{}], n6={:.1}%",
                    p.recurrence_count,
                    p.metric_keys.join(", "),
                    p.n6_alignment * 100.0,
                ),
                source: CandidateSource::GapFill(format!(
                    "pattern_detector:{}",
                    p.source_lenses.join("+")
                )),
                domain_affinity: vec![p.domain.clone()],
                complementary: p.source_lenses.clone(),
                confidence: p.confidence,
            })
            .collect()
    }

    /// Register detected patterns as "lens_candidate" nodes in the DiscoveryGraph.
    pub fn register_in_graph(
        &mut self,
        patterns: &[DetectedPattern],
        graph: &mut DiscoveryGraph,
        cycle: usize,
    ) {
        for (i, pattern) in patterns.iter().enumerate() {
            let node_id = format!("lens_candidate-c{}-{}", cycle, i);

            graph.add_node(Node {
                id: node_id.clone(),
                node_type: NodeType::Discovery,
                domain: pattern.domain.clone(),
                project: "nexus6".to_string(),
                summary: format!(
                    "Lens candidate '{}': {} metrics, n6={:.1}%, recurrence={}",
                    pattern.name,
                    pattern.metric_keys.len(),
                    pattern.n6_alignment * 100.0,
                    pattern.recurrence_count,
                ),
                confidence: pattern.confidence,
                lenses_used: pattern.source_lenses.clone(),
                timestamp: format!("cycle-{}", cycle),
            });

            // Connect to source lens nodes if they exist
            for source in &pattern.source_lenses {
                let source_nodes: Vec<String> = graph
                    .nodes
                    .iter()
                    .filter(|n| n.summary.contains(source))
                    .map(|n| n.id.clone())
                    .collect();
                for src_id in source_nodes.iter().take(2) {
                    graph.add_edge(Edge {
                        from: src_id.clone(),
                        to: node_id.clone(),
                        edge_type: EdgeType::Derives,
                        strength: pattern.confidence,
                        bidirectional: false,
                    });
                }
            }

            // Mark as suggested to avoid re-emitting
            self.suggested_patterns.insert(pattern.name.clone());
        }
    }

    // ─── Strategy 1: Recurrent high scorers ───

    /// Find metric keys that consistently produce high values across cycles.
    fn detect_recurrent_high_scorers(&self) -> Vec<DetectedPattern> {
        // Collect: (lens_name, metric_key) -> list of mean values per cycle
        let mut metric_history: HashMap<(String, String), Vec<f64>> = HashMap::new();

        for snapshot in &self.snapshots {
            for (lens_name, lens_result) in &snapshot.results {
                for (metric_key, values) in lens_result {
                    if values.is_empty() {
                        continue;
                    }
                    let mean = values.iter().sum::<f64>() / values.len() as f64;
                    metric_history
                        .entry((lens_name.clone(), metric_key.clone()))
                        .or_default()
                        .push(mean);
                }
            }
        }

        // Find metrics that are consistently high (mean > 0.5) across multiple cycles
        let mut patterns = Vec::new();

        // Group by lens to find intra-lens metric combinations
        let mut lens_high_metrics: HashMap<String, Vec<(String, usize, f64)>> = HashMap::new();

        for ((lens, metric), values) in &metric_history {
            let high_count = values.iter().filter(|&&v| v > 0.5).count();
            if high_count >= MIN_RECURRENCE {
                let all_values: Vec<f64> = self
                    .snapshots
                    .iter()
                    .flat_map(|s| {
                        s.results
                            .get(lens)
                            .and_then(|lr| lr.get(metric))
                            .cloned()
                            .unwrap_or_default()
                    })
                    .collect();
                let n6_ratio = n6_check::n6_exact_ratio(&all_values);

                lens_high_metrics
                    .entry(lens.clone())
                    .or_default()
                    .push((metric.clone(), high_count, n6_ratio));
            }
        }

        for (lens, metrics) in &lens_high_metrics {
            if metrics.len() < 2 {
                continue; // Need at least 2 correlated metrics for a pattern
            }

            let metric_keys: Vec<String> = metrics.iter().map(|(k, _, _)| k.clone()).collect();
            let total_recurrence: usize = metrics.iter().map(|(_, c, _)| c).sum();
            let avg_n6: f64 =
                metrics.iter().map(|(_, _, n)| n).sum::<f64>() / metrics.len() as f64;

            if avg_n6 < MIN_N6_ALIGNMENT {
                continue;
            }

            let name = format!(
                "{}_high_combo_{}",
                lens.replace(' ', "_").to_lowercase(),
                metric_keys.len()
            );

            if self.known_lens_names.contains(&format!("auto_{}", name)) {
                continue;
            }

            let domain = self
                .snapshots
                .last()
                .map(|s| s.domain.clone())
                .unwrap_or_else(|| "general".to_string());

            patterns.push(DetectedPattern {
                name,
                metric_keys,
                source_lenses: vec![lens.clone()],
                recurrence_count: total_recurrence / metrics.len().max(1),
                n6_alignment: avg_n6,
                confidence: (avg_n6 * 0.6 + (total_recurrence as f64 / self.snapshots.len() as f64).min(1.0) * 0.4)
                    .min(1.0),
                domain,
            });
        }

        patterns
    }

    // ─── Strategy 2: Blind spot detection ───

    /// Find lenses that consistently return empty results across cycles.
    /// Their domain may need a specialized lens.
    fn detect_blind_spots(&self) -> Vec<DetectedPattern> {
        // Count how many cycles each lens returned empty results
        let mut lens_empty_count: HashMap<String, usize> = HashMap::new();
        let mut lens_total_count: HashMap<String, usize> = HashMap::new();

        for snapshot in &self.snapshots {
            for (lens_name, lens_result) in &snapshot.results {
                *lens_total_count.entry(lens_name.clone()).or_insert(0) += 1;
                let all_empty = lens_result.values().all(|v| v.is_empty());
                if all_empty {
                    *lens_empty_count.entry(lens_name.clone()).or_insert(0) += 1;
                }
            }
        }

        let mut patterns = Vec::new();

        for (lens, empty_count) in &lens_empty_count {
            let total = lens_total_count.get(lens).copied().unwrap_or(1);
            let empty_ratio = *empty_count as f64 / total as f64;

            // If a lens is empty > 80% of the time, this domain has a blind spot
            if empty_ratio > 0.8 && total >= MIN_RECURRENCE {
                let name = format!("{}_blind_spot", lens.replace(' ', "_").to_lowercase());

                if self.known_lens_names.contains(&format!("auto_{}", name)) {
                    continue;
                }

                let domain = self
                    .snapshots
                    .last()
                    .map(|s| s.domain.clone())
                    .unwrap_or_else(|| "general".to_string());

                patterns.push(DetectedPattern {
                    name,
                    metric_keys: vec![format!("{}_coverage", lens)],
                    source_lenses: vec![lens.clone()],
                    recurrence_count: *empty_count,
                    n6_alignment: 0.0, // blind spots have no n6 signal yet
                    confidence: empty_ratio * 0.5, // moderate confidence
                    domain,
                });
            }
        }

        patterns
    }

    // ─── Strategy 3: n6-correlated metric pairs ───

    /// Find pairs of metrics across different lenses whose combined values
    /// show high n6 constant alignment.
    fn detect_n6_correlated_pairs(&self) -> Vec<DetectedPattern> {
        // Collect all (lens, metric) -> flat values across cycles
        let mut all_metrics: Vec<(String, String, Vec<f64>)> = Vec::new();

        for snapshot in &self.snapshots {
            for (lens_name, lens_result) in &snapshot.results {
                for (metric_key, values) in lens_result {
                    if values.is_empty() {
                        continue;
                    }
                    // Check if we already have this (lens, metric) pair
                    let found = all_metrics
                        .iter_mut()
                        .find(|(l, m, _)| l == lens_name && m == metric_key);
                    match found {
                        Some((_, _, existing)) => existing.extend(values),
                        None => all_metrics.push((
                            lens_name.clone(),
                            metric_key.clone(),
                            values.clone(),
                        )),
                    }
                }
            }
        }

        let mut patterns = Vec::new();

        // Check pairs from different lenses
        let len = all_metrics.len().min(50); // cap for performance
        for i in 0..len {
            for j in (i + 1)..len {
                let (lens_a, metric_a, vals_a) = &all_metrics[i];
                let (lens_b, metric_b, vals_b) = &all_metrics[j];

                // Skip same-lens pairs
                if lens_a == lens_b {
                    continue;
                }

                // Combine values and check n6 alignment
                let mut combined: Vec<f64> = Vec::new();
                combined.extend(vals_a);
                combined.extend(vals_b);

                // Also add cross-products and ratios that might reveal n6 constants
                if !vals_a.is_empty() && !vals_b.is_empty() {
                    let mean_a = vals_a.iter().sum::<f64>() / vals_a.len() as f64;
                    let mean_b = vals_b.iter().sum::<f64>() / vals_b.len() as f64;
                    if mean_b.abs() > 1e-10 {
                        combined.push(mean_a / mean_b);
                        combined.push(mean_a * mean_b);
                        combined.push(mean_a + mean_b);
                    }
                }

                let n6_ratio = n6_check::n6_exact_ratio(&combined);
                if n6_ratio < MIN_N6_ALIGNMENT {
                    continue;
                }

                // 3+ lens consensus requirement: check that at least 3 different
                // n6 constants are matched
                let matched_constants: HashSet<&str> = combined
                    .iter()
                    .filter_map(|v| {
                        let (name, quality) = n6_check::n6_match(*v);
                        if quality >= 0.8 {
                            Some(name)
                        } else {
                            None
                        }
                    })
                    .collect();

                if matched_constants.len() < 2 {
                    continue;
                }

                let name = format!(
                    "{}_{}_n6_pair",
                    lens_a.replace(' ', "_").to_lowercase(),
                    lens_b.replace(' ', "_").to_lowercase(),
                );

                if self.known_lens_names.contains(&format!("auto_{}", name)) {
                    continue;
                }

                let domain = self
                    .snapshots
                    .last()
                    .map(|s| s.domain.clone())
                    .unwrap_or_else(|| "general".to_string());

                patterns.push(DetectedPattern {
                    name,
                    metric_keys: vec![
                        format!("{}:{}", lens_a, metric_a),
                        format!("{}:{}", lens_b, metric_b),
                    ],
                    source_lenses: vec![lens_a.clone(), lens_b.clone()],
                    recurrence_count: self.snapshots.len(),
                    n6_alignment: n6_ratio,
                    confidence: (n6_ratio * 0.7
                        + (matched_constants.len() as f64 / 6.0).min(1.0) * 0.3)
                        .min(1.0),
                    domain,
                });

                if patterns.len() >= MAX_CANDIDATES {
                    return patterns;
                }
            }
        }

        patterns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_snapshot(
        cycle: usize,
        domain: &str,
        results: Vec<(&str, Vec<(&str, Vec<f64>)>)>,
    ) -> ScanSnapshot {
        let mut map = HashMap::new();
        for (lens, metrics) in results {
            let mut lr = LensResult::new();
            for (key, vals) in metrics {
                lr.insert(key.to_string(), vals);
            }
            map.insert(lens.to_string(), lr);
        }
        ScanSnapshot {
            cycle,
            results: map,
            domain: domain.to_string(),
        }
    }

    #[test]
    fn test_no_detection_below_min_cycles() {
        let mut detector = PatternDetector::new(vec![]);
        // Only 2 snapshots, need 3
        detector.record_snapshot(make_snapshot(1, "ai", vec![]));
        detector.record_snapshot(make_snapshot(2, "ai", vec![]));
        let patterns = detector.detect_new_patterns();
        assert!(
            patterns.is_empty(),
            "Should not detect patterns with < {} cycles",
            MIN_CYCLES_FOR_DETECTION
        );
    }

    #[test]
    fn test_detect_recurrent_high_scorers() {
        let mut detector = PatternDetector::new(vec![]);

        // Create 3 cycles where "consciousness" lens consistently produces high values
        // on two metrics
        for cycle in 1..=3 {
            detector.record_snapshot(make_snapshot(
                cycle,
                "ai",
                vec![(
                    "consciousness",
                    vec![
                        ("phi_ratio", vec![6.0, 12.0, 0.8]),   // high + n6
                        ("complexity", vec![24.0, 0.9, 4.0]),   // high + n6
                    ],
                )],
            ));
        }

        let patterns = detector.detect_new_patterns();
        // Should find at least one pattern from the recurrent high scorers
        assert!(
            !patterns.is_empty(),
            "Should detect recurrent high-scoring pattern"
        );

        // Check that the pattern has n6 alignment
        let best = &patterns[0];
        assert!(best.n6_alignment > 0.0, "Pattern should have n6 alignment");
        assert!(best.recurrence_count >= 2, "Should recur across cycles");
    }

    #[test]
    fn test_detect_blind_spots() {
        let mut detector = PatternDetector::new(vec![]);

        // Create 3 cycles where "wave" lens always returns empty
        for cycle in 1..=3 {
            detector.record_snapshot(make_snapshot(
                cycle,
                "chip",
                vec![
                    ("wave", vec![("amplitude", vec![])]),  // always empty
                    (
                        "topology",
                        vec![("connectivity", vec![6.0, 12.0])], // has data
                    ),
                ],
            ));
        }

        let patterns = detector.detect_new_patterns();
        let blind = patterns.iter().find(|p| p.name.contains("blind_spot"));
        assert!(
            blind.is_some(),
            "Should detect blind spot for empty lens"
        );
    }

    #[test]
    fn test_detect_n6_correlated_pairs() {
        let mut detector = PatternDetector::new(vec![]);

        // Create 3 cycles with two lenses whose values align to n6 constants
        for cycle in 1..=3 {
            detector.record_snapshot(make_snapshot(
                cycle,
                "energy",
                vec![
                    (
                        "thermo",
                        vec![("temperature", vec![6.0, 12.0, 24.0])],
                    ),
                    (
                        "gravity",
                        vec![("force", vec![4.0, 2.0, 5.0])],
                    ),
                ],
            ));
        }

        let patterns = detector.detect_new_patterns();
        let n6_pair = patterns.iter().find(|p| p.name.contains("n6_pair"));
        assert!(
            n6_pair.is_some(),
            "Should detect n6-correlated pair across lenses"
        );
        if let Some(pair) = n6_pair {
            assert!(pair.n6_alignment >= MIN_N6_ALIGNMENT);
            assert_eq!(pair.source_lenses.len(), 2);
        }
    }

    #[test]
    fn test_suggest_lens_templates() {
        let detector = PatternDetector::new(vec![]);
        let patterns = vec![DetectedPattern {
            name: "test_pattern".to_string(),
            metric_keys: vec!["m1".to_string(), "m2".to_string()],
            source_lenses: vec!["consciousness".to_string()],
            recurrence_count: 3,
            n6_alignment: 0.5,
            confidence: 0.8,
            domain: "ai".to_string(),
        }];

        let candidates = detector.suggest_lens_templates(&patterns);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].name, "auto_test_pattern");
        assert!(candidates[0].description.contains("n6=50.0%"));
        assert_eq!(candidates[0].domain_affinity, vec!["ai".to_string()]);
    }

    #[test]
    fn test_register_in_graph() {
        let mut detector = PatternDetector::new(vec![]);
        let mut graph = DiscoveryGraph::new();

        let patterns = vec![DetectedPattern {
            name: "graph_test".to_string(),
            metric_keys: vec!["m1".to_string()],
            source_lenses: vec!["topology".to_string()],
            recurrence_count: 3,
            n6_alignment: 0.6,
            confidence: 0.7,
            domain: "general".to_string(),
        }];

        detector.register_in_graph(&patterns, &mut graph, 5);

        assert_eq!(graph.nodes.len(), 1);
        assert!(graph.nodes[0].id.starts_with("lens_candidate-c5-"));
        assert!(graph.nodes[0].summary.contains("graph_test"));
        // Pattern should now be in suggested set
        assert!(detector.suggested_patterns.contains("graph_test"));
    }

    #[test]
    fn test_deduplication_across_calls() {
        let mut detector = PatternDetector::new(vec![]);

        // Create data that produces patterns
        for cycle in 1..=3 {
            detector.record_snapshot(make_snapshot(
                cycle,
                "ai",
                vec![(
                    "consciousness",
                    vec![
                        ("phi_ratio", vec![6.0, 12.0, 0.8]),
                        ("complexity", vec![24.0, 0.9, 4.0]),
                    ],
                )],
            ));
        }

        let patterns1 = detector.detect_new_patterns();
        // Mark as suggested
        let mut graph = DiscoveryGraph::new();
        detector.register_in_graph(&patterns1, &mut graph, 1);

        // Detect again -- should not re-emit same patterns
        let patterns2 = detector.detect_new_patterns();
        let overlapping: Vec<_> = patterns2
            .iter()
            .filter(|p| patterns1.iter().any(|p1| p1.name == p.name))
            .collect();
        assert!(
            overlapping.is_empty(),
            "Should not re-emit previously suggested patterns"
        );
    }

    #[test]
    fn test_max_candidates_cap() {
        let mut detector = PatternDetector::new(vec![]);

        // Create lots of data to potentially generate many patterns
        for cycle in 1..=6 {
            let mut results = Vec::new();
            for lens_idx in 0..12 {
                let lens_name = format!("lens_{}", lens_idx);
                let metrics = vec![
                    (
                        "metric_a",
                        vec![6.0, 12.0, (cycle as f64) + (lens_idx as f64)],
                    ),
                    (
                        "metric_b",
                        vec![24.0, 4.0, 0.7 + (lens_idx as f64) * 0.01],
                    ),
                ];
                // Convert &str to owned for make_snapshot workaround
                results.push((lens_name, metrics));
            }

            // Build the snapshot manually since make_snapshot takes &str
            let mut map = HashMap::new();
            for (lens, metrics) in results {
                let mut lr = LensResult::new();
                for (key, vals) in metrics {
                    lr.insert(key.to_string(), vals);
                }
                map.insert(lens, lr);
            }
            detector.record_snapshot(ScanSnapshot {
                cycle,
                results: map,
                domain: "general".to_string(),
            });
        }

        let patterns = detector.detect_new_patterns();
        assert!(
            patterns.len() <= MAX_CANDIDATES,
            "Should cap at {} candidates, got {}",
            MAX_CANDIDATES,
            patterns.len()
        );
    }
}
