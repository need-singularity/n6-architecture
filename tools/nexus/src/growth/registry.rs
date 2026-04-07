//! Universal Growth Registry — tracks ALL dimensions of NEXUS-6 evolution.
//!
//! Every growth dimension is registered here. The registry tracks metrics,
//! targets, and progress for each dimension, computes health, identifies
//! weaknesses, and renders an ASCII dashboard.

use std::collections::HashMap;

// ── n=6 constants ────────────────────────────────────────────────────
#[allow(unused)]
const N: usize = 6;                   // the perfect number
const SIGMA: usize = 12;             // sigma(6) = sum of divisors
const PHI: usize = 2;                // phi(6) = Euler totient
const _TAU: usize = 4;                // tau(6) = number of divisors
const _J2: usize = 24;                // J_2(6) = Jordan totient
const _SOPFR: usize = 5;              // sopfr(6) = 2+3
const SIGMA_MINUS_PHI: usize = 10;   // sigma - phi = 10
const _SIGMA_MINUS_TAU: usize = 8;    // sigma - tau = 8

/// All 15 growth dimensions of NEXUS-6.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum GrowthDimension {
    Performance,     // speed, throughput, latency
    Architecture,    // module structure, integrations, gaps
    Lenses,          // implemented lens count (24 -> 200+)
    Modules,         // module maturity levels
    Tests,           // test count and coverage
    Hypotheses,      // BT count, verification rate
    DSE,             // DSE domain coverage (322 TOML)
    Experiments,     // experiment count and pass rate
    Calculators,     // Rust/Python calculator count
    CrossResonance,  // cross-domain pattern discoveries
    KnowledgeGraph,  // graph nodes, edges, hubs
    RedTeam,         // adversarial challenge coverage
    Atlas,           // math atlas constant count
    Documentation,   // docs quality and completeness
    Integration,     // cross-module integration test count
}

impl GrowthDimension {
    /// Return all 15 dimensions in canonical order.
    pub fn all() -> Vec<GrowthDimension> {
        vec![
            GrowthDimension::Performance,
            GrowthDimension::Architecture,
            GrowthDimension::Lenses,
            GrowthDimension::Modules,
            GrowthDimension::Tests,
            GrowthDimension::Hypotheses,
            GrowthDimension::DSE,
            GrowthDimension::Experiments,
            GrowthDimension::Calculators,
            GrowthDimension::CrossResonance,
            GrowthDimension::KnowledgeGraph,
            GrowthDimension::RedTeam,
            GrowthDimension::Atlas,
            GrowthDimension::Documentation,
            GrowthDimension::Integration,
        ]
    }

    /// Short display name for dashboard columns.
    pub fn short_name(&self) -> &str {
        match self {
            GrowthDimension::Performance    => "Performance",
            GrowthDimension::Architecture   => "Architecture",
            GrowthDimension::Lenses         => "Lenses",
            GrowthDimension::Modules        => "Modules",
            GrowthDimension::Tests          => "Tests",
            GrowthDimension::Hypotheses     => "Hypotheses",
            GrowthDimension::DSE            => "DSE",
            GrowthDimension::Experiments    => "Experiments",
            GrowthDimension::Calculators    => "Calculators",
            GrowthDimension::CrossResonance => "CrossReson",
            GrowthDimension::KnowledgeGraph => "KnowledgeGr",
            GrowthDimension::RedTeam        => "RedTeam",
            GrowthDimension::Atlas          => "Atlas",
            GrowthDimension::Documentation  => "Docs",
            GrowthDimension::Integration    => "Integration",
        }
    }

    /// Impact weight for priority scoring (sums to ~1.0).
    /// Higher weight = more critical to overall system health.
    pub fn impact_weight(&self) -> f64 {
        match self {
            GrowthDimension::Tests          => 0.12,  // sigma/100
            GrowthDimension::Lenses         => 0.10,  // sigma-phi / 100
            GrowthDimension::Architecture   => 0.10,
            GrowthDimension::Performance    => 0.08,  // sigma-tau / 100... close
            GrowthDimension::Hypotheses     => 0.08,
            GrowthDimension::Integration    => 0.08,
            GrowthDimension::DSE            => 0.06,  // n/100
            GrowthDimension::KnowledgeGraph => 0.06,
            GrowthDimension::RedTeam        => 0.06,
            GrowthDimension::CrossResonance => 0.05,  // sopfr/100
            GrowthDimension::Atlas          => 0.05,
            GrowthDimension::Experiments    => 0.05,
            GrowthDimension::Calculators    => 0.04,  // tau/100
            GrowthDimension::Modules        => 0.04,
            GrowthDimension::Documentation  => 0.03,
        }
    }
}

/// Health status of a single dimension.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DimensionHealth {
    Thriving,   // growing faster than target
    OnTrack,    // growing at target rate
    Stagnant,   // no growth for 3+ cycles
    Declining,  // value decreased
    Critical,   // far below target (< 25% progress)
}

impl DimensionHealth {
    pub fn label(&self) -> &str {
        match self {
            DimensionHealth::Thriving  => "Thriving",
            DimensionHealth::OnTrack   => "OnTrack",
            DimensionHealth::Stagnant  => "Stagnant",
            DimensionHealth::Declining => "Declining",
            DimensionHealth::Critical  => "Critical",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            DimensionHealth::Thriving  => "[++]",
            DimensionHealth::OnTrack   => "[ +]",
            DimensionHealth::Stagnant  => "[ =]",
            DimensionHealth::Declining => "[ -]",
            DimensionHealth::Critical  => "[!!]",
        }
    }
}

/// State of a single growth dimension.
#[derive(Debug, Clone)]
pub struct DimensionState {
    pub dimension: GrowthDimension,
    pub current_value: f64,
    pub target_value: f64,
    pub unit: String,          // "count", "percent", "ops/sec", etc.
    pub growth_rate: f64,      // per cycle (computed from history)
    pub last_updated: String,  // ISO-8601 timestamp
    pub health: DimensionHealth,
    /// History of values for velocity computation (last sigma-phi=10 values).
    history: Vec<f64>,
}

impl DimensionState {
    pub fn new(dimension: GrowthDimension, target: f64, unit: &str) -> Self {
        DimensionState {
            dimension,
            current_value: 0.0,
            target_value: target,
            unit: unit.to_string(),
            growth_rate: 0.0,
            last_updated: String::new(),
            health: DimensionHealth::Stagnant,
            history: Vec::new(),
        }
    }

    /// Progress as a fraction in [0.0, 1.0].
    pub fn progress(&self) -> f64 {
        if self.target_value <= 0.0 {
            return if self.current_value <= 0.0 { 1.0 } else { 1.0 };
        }
        (self.current_value / self.target_value).min(1.0).max(0.0)
    }

    /// Gap: how far from target (1.0 = no progress, 0.0 = target met).
    pub fn gap(&self) -> f64 {
        1.0 - self.progress()
    }

    /// Update the value and recompute health.
    pub fn update(&mut self, value: f64, timestamp: &str) {
        let previous = self.current_value;
        self.current_value = value;
        self.last_updated = timestamp.to_string();
        self.history.push(value);

        // Keep only last sigma-phi=10 history entries
        if self.history.len() > SIGMA_MINUS_PHI {
            let excess = self.history.len() - SIGMA_MINUS_PHI;
            self.history.drain(..excess);
        }

        // Compute growth rate from recent history
        if self.history.len() >= PHI {
            let n = self.history.len();
            self.growth_rate = (self.history[n - 1] - self.history[0]) / (n - 1) as f64;
        }

        // Determine health
        let progress = self.progress();
        if progress < 0.25 {
            self.health = DimensionHealth::Critical;
        } else if value < previous {
            self.health = DimensionHealth::Declining;
        } else if self.growth_rate > 0.01 && progress > 0.5 {
            self.health = DimensionHealth::Thriving;
        } else if self.growth_rate.abs() < 0.001 && self.history.len() >= 3 {
            self.health = DimensionHealth::Stagnant;
        } else {
            self.health = DimensionHealth::OnTrack;
        }
    }
}

/// Snapshot of all dimensions at a point in time.
#[derive(Debug, Clone)]
pub struct GrowthSnapshot {
    pub cycle: usize,
    pub timestamp: String,
    pub values: HashMap<GrowthDimension, f64>,
    pub actions_taken: Vec<(GrowthDimension, String)>,
}

/// The central growth registry tracking all 15 dimensions.
pub struct GrowthRegistry {
    pub dimensions: HashMap<GrowthDimension, DimensionState>,
    pub cycle_count: usize,
    pub history: Vec<GrowthSnapshot>,
}

impl GrowthRegistry {
    /// Initialize with all 15 dimensions and default targets.
    pub fn new() -> Self {
        let mut dimensions = HashMap::new();

        // Performance: target throughput 10000 ops/sec
        dimensions.insert(
            GrowthDimension::Performance,
            DimensionState::new(GrowthDimension::Performance, 10000.0, "ops/sec"),
        );

        // Architecture: target 0 gaps (inverted: track completeness out of 100%)
        dimensions.insert(
            GrowthDimension::Architecture,
            DimensionState::new(GrowthDimension::Architecture, 100.0, "percent"),
        );

        // Lenses: target 200 implemented (from ~24)
        dimensions.insert(
            GrowthDimension::Lenses,
            DimensionState::new(GrowthDimension::Lenses, 200.0, "count"),
        );

        // Modules: target mean maturity 4.0/5.0 (expressed as percentage: 80%)
        dimensions.insert(
            GrowthDimension::Modules,
            DimensionState::new(GrowthDimension::Modules, 4.0, "score"),
        );

        // Tests: target 1000 (from ~483)
        dimensions.insert(
            GrowthDimension::Tests,
            DimensionState::new(GrowthDimension::Tests, 1000.0, "count"),
        );

        // Hypotheses: target 150 BTs
        dimensions.insert(
            GrowthDimension::Hypotheses,
            DimensionState::new(GrowthDimension::Hypotheses, 150.0, "count"),
        );

        // DSE: target 322 domains explored
        dimensions.insert(
            GrowthDimension::DSE,
            DimensionState::new(GrowthDimension::DSE, 322.0, "count"),
        );

        // Experiments: target 50
        dimensions.insert(
            GrowthDimension::Experiments,
            DimensionState::new(GrowthDimension::Experiments, 50.0, "count"),
        );

        // Calculators: target 50 tools
        dimensions.insert(
            GrowthDimension::Calculators,
            DimensionState::new(GrowthDimension::Calculators, 50.0, "count"),
        );

        // CrossResonance: target 100 resonance hits
        dimensions.insert(
            GrowthDimension::CrossResonance,
            DimensionState::new(GrowthDimension::CrossResonance, 100.0, "count"),
        );

        // KnowledgeGraph: target 500 nodes
        dimensions.insert(
            GrowthDimension::KnowledgeGraph,
            DimensionState::new(GrowthDimension::KnowledgeGraph, 500.0, "count"),
        );

        // RedTeam: target 100 challenges
        dimensions.insert(
            GrowthDimension::RedTeam,
            DimensionState::new(GrowthDimension::RedTeam, 100.0, "count"),
        );

        // Atlas: target 2000 constants
        dimensions.insert(
            GrowthDimension::Atlas,
            DimensionState::new(GrowthDimension::Atlas, 2000.0, "count"),
        );

        // Documentation: target 90% coverage
        dimensions.insert(
            GrowthDimension::Documentation,
            DimensionState::new(GrowthDimension::Documentation, 90.0, "percent"),
        );

        // Integration: target 50 cross-module tests
        dimensions.insert(
            GrowthDimension::Integration,
            DimensionState::new(GrowthDimension::Integration, 50.0, "count"),
        );

        GrowthRegistry {
            dimensions,
            cycle_count: 0,
            history: Vec::new(),
        }
    }

    /// Update a single dimension's current value.
    pub fn update_dimension(&mut self, dim: GrowthDimension, value: f64) {
        let timestamp = format!("cycle-{}", self.cycle_count);
        if let Some(state) = self.dimensions.get_mut(&dim) {
            state.update(value, &timestamp);
        }
    }

    /// Capture a snapshot of all current values.
    pub fn snapshot(&mut self) -> GrowthSnapshot {
        self.cycle_count += 1;
        let timestamp = format!("cycle-{}", self.cycle_count);

        let values: HashMap<GrowthDimension, f64> = self
            .dimensions
            .iter()
            .map(|(dim, state)| (dim.clone(), state.current_value))
            .collect();

        let snap = GrowthSnapshot {
            cycle: self.cycle_count,
            timestamp,
            values,
            actions_taken: Vec::new(),
        };

        self.history.push(snap.clone());

        // Keep history bounded to sigma^2 = 144 entries
        if self.history.len() > SIGMA * SIGMA {
            let excess = self.history.len() - SIGMA * SIGMA;
            self.history.drain(..excess);
        }

        snap
    }

    /// Find the n dimensions most behind their target (largest gap).
    pub fn weakest_dimensions(&self, n: usize) -> Vec<(&GrowthDimension, &DimensionState)> {
        let mut dims: Vec<(&GrowthDimension, &DimensionState)> =
            self.dimensions.iter().collect();
        dims.sort_by(|a, b| {
            b.1.gap()
                .partial_cmp(&a.1.gap())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        dims.truncate(n);
        dims
    }

    /// Which dimension needs attention most (furthest from target, weighted by impact).
    pub fn next_growth_priority(&self) -> GrowthDimension {
        let mut best_dim = GrowthDimension::Tests; // default fallback
        let mut best_score = -1.0_f64;

        for (dim, state) in &self.dimensions {
            // Weighted priority = gap * impact_weight
            let score = state.gap() * dim.impact_weight();
            if score > best_score {
                best_score = score;
                best_dim = dim.clone();
            }
        }

        best_dim
    }

    /// Rate of growth per cycle for each dimension.
    pub fn growth_velocity(&self) -> HashMap<GrowthDimension, f64> {
        self.dimensions
            .iter()
            .map(|(dim, state)| (dim.clone(), state.growth_rate))
            .collect()
    }

    /// Render an ASCII dashboard showing ALL dimensions.
    pub fn format_dashboard(&self) -> String {
        let bar_width = SIGMA; // sigma=12 character progress bar
        let mut s = String::new();

        s.push_str("+-----------------------------------------------------------------+\n");
        s.push_str(&format!(
            "| NEXUS-6 Growth Dashboard -- Cycle {:<6}                         |\n",
            self.cycle_count
        ));
        s.push_str("+--------------+---------+---------+--------------+----------------+\n");
        s.push_str("| Dimension    |Current  |Target   |Progress      | Health         |\n");
        s.push_str("+--------------+---------+---------+--------------+----------------+\n");

        // Sort dimensions by gap (worst first) for quick triage
        let mut ordered: Vec<(&GrowthDimension, &DimensionState)> =
            self.dimensions.iter().collect();
        ordered.sort_by(|a, b| {
            b.1.gap()
                .partial_cmp(&a.1.gap())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for (dim, state) in &ordered {
            let progress = state.progress();
            let filled = (progress * bar_width as f64).round() as usize;
            let empty = bar_width.saturating_sub(filled);
            let bar = format!(
                "{}{}",
                "#".repeat(filled),
                ".".repeat(empty),
            );
            let pct = progress * 100.0;

            // Format current value: use integer display if > 10, else one decimal
            let cur_str = if state.current_value >= 10.0 {
                format!("{:.0}", state.current_value)
            } else {
                format!("{:.1}", state.current_value)
            };

            let tgt_str = if state.target_value >= 10.0 {
                format!("{:.0}", state.target_value)
            } else {
                format!("{:.1}", state.target_value)
            };

            // Velocity arrow
            let arrow = if state.growth_rate > 0.5 {
                "^^"
            } else if state.growth_rate > 0.01 {
                "^ "
            } else if state.growth_rate < -0.01 {
                "v "
            } else {
                "- "
            };

            s.push_str(&format!(
                "| {:<12} | {:>7} | {:>7} | {} {:>3.0}% {} | {} {:<8} |\n",
                dim.short_name(),
                cur_str,
                tgt_str,
                bar,
                pct,
                arrow,
                state.health.icon(),
                state.health.label(),
            ));
        }

        s.push_str("+--------------+---------+---------+--------------+----------------+\n");

        // Summary line: overall progress
        let total_progress: f64 = self.dimensions.values().map(|s| s.progress()).sum::<f64>()
            / self.dimensions.len() as f64;
        let critical_count = self
            .dimensions
            .values()
            .filter(|s| s.health == DimensionHealth::Critical)
            .count();
        let thriving_count = self
            .dimensions
            .values()
            .filter(|s| s.health == DimensionHealth::Thriving)
            .count();

        s.push_str(&format!(
            "| Overall: {:.1}%  |  Critical: {}  |  Thriving: {}  | Dims: {}     |\n",
            total_progress * 100.0,
            critical_count,
            thriving_count,
            self.dimensions.len(),
        ));
        s.push_str("+-----------------------------------------------------------------+\n");

        // Next priority
        let priority = self.next_growth_priority();
        s.push_str(&format!(
            "| >> Next priority: {:<44} |\n",
            priority.short_name()
        ));
        s.push_str("+-----------------------------------------------------------------+\n");

        s
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_registry_has_all_dimensions() {
        let reg = GrowthRegistry::new();
        // All 15 dimensions present
        assert_eq!(reg.dimensions.len(), 15);  // 15 = sigma + n/phi = 12+3
        for dim in GrowthDimension::all() {
            assert!(
                reg.dimensions.contains_key(&dim),
                "missing dimension: {:?}",
                dim
            );
        }
    }

    #[test]
    fn test_update_dimension_and_health() {
        let mut reg = GrowthRegistry::new();

        // Lenses: target=200, update to 24 => progress=12% => Critical
        reg.update_dimension(GrowthDimension::Lenses, 24.0);
        let state = &reg.dimensions[&GrowthDimension::Lenses];
        assert!((state.current_value - 24.0).abs() < f64::EPSILON);
        assert_eq!(state.health, DimensionHealth::Critical); // < 25%

        // Update to 180 => progress=90%, growth_rate high => Thriving
        reg.update_dimension(GrowthDimension::Lenses, 180.0);
        let state = &reg.dimensions[&GrowthDimension::Lenses];
        assert_eq!(state.health, DimensionHealth::Thriving);
    }

    #[test]
    fn test_snapshot_increments_cycle() {
        let mut reg = GrowthRegistry::new();
        assert_eq!(reg.cycle_count, 0);

        let snap1 = reg.snapshot();
        assert_eq!(snap1.cycle, 1);
        assert_eq!(reg.cycle_count, 1);

        let snap2 = reg.snapshot();
        assert_eq!(snap2.cycle, 2);
        assert_eq!(reg.history.len(), 2);
    }

    #[test]
    fn test_weakest_dimensions() {
        let mut reg = GrowthRegistry::new();

        // Set some dimensions close to target, others far
        reg.update_dimension(GrowthDimension::Tests, 950.0);       // 95% progress
        reg.update_dimension(GrowthDimension::Hypotheses, 145.0);  // 97%
        reg.update_dimension(GrowthDimension::Lenses, 10.0);       // 5%  -- weakest
        reg.update_dimension(GrowthDimension::RedTeam, 5.0);       // 5%  -- also weak

        let weakest = reg.weakest_dimensions(3);
        assert_eq!(weakest.len(), 3);
        // The first two should be among the weakest (gap near 1.0)
        assert!(weakest[0].1.gap() >= weakest[1].1.gap());
        assert!(weakest[1].1.gap() >= weakest[2].1.gap());
    }

    #[test]
    fn test_next_growth_priority() {
        let mut reg = GrowthRegistry::new();

        // Make Tests the weakest with high impact weight
        reg.update_dimension(GrowthDimension::Tests, 0.0);          // gap=1.0, weight=0.12
        reg.update_dimension(GrowthDimension::Documentation, 0.0);  // gap=1.0, weight=0.03

        let priority = reg.next_growth_priority();
        // Tests has higher impact weight, so it should win
        assert_eq!(priority, GrowthDimension::Tests);
    }

    #[test]
    fn test_growth_velocity() {
        let mut reg = GrowthRegistry::new();

        // Feed multiple updates to build velocity
        reg.update_dimension(GrowthDimension::Lenses, 24.0);
        reg.update_dimension(GrowthDimension::Lenses, 30.0);
        reg.update_dimension(GrowthDimension::Lenses, 36.0);

        let velocity = reg.growth_velocity();
        let lens_vel = velocity[&GrowthDimension::Lenses];
        // (36 - 24) / (3-1) = 6.0 per cycle
        assert!((lens_vel - N as f64).abs() < f64::EPSILON, "expected n=6, got {}", lens_vel);
    }

    #[test]
    fn test_format_dashboard() {
        let mut reg = GrowthRegistry::new();
        reg.update_dimension(GrowthDimension::Lenses, 24.0);
        reg.update_dimension(GrowthDimension::Tests, 483.0);
        reg.update_dimension(GrowthDimension::Hypotheses, 127.0);
        reg.snapshot();

        let dashboard = reg.format_dashboard();
        assert!(dashboard.contains("NEXUS-6 Growth Dashboard"));
        assert!(dashboard.contains("Dimension"));
        assert!(dashboard.contains("Current"));
        assert!(dashboard.contains("Target"));
        assert!(dashboard.contains("Next priority"));
    }

    #[test]
    fn test_dimension_state_progress() {
        let mut state = DimensionState::new(GrowthDimension::Tests, 1000.0, "count");
        assert!((state.progress() - 0.0).abs() < f64::EPSILON);

        state.update(500.0, "t1");
        assert!((state.progress() - 0.5).abs() < f64::EPSILON);

        state.update(1000.0, "t2");
        assert!((state.progress() - 1.0).abs() < f64::EPSILON);

        // Over target still clamps to 1.0
        state.update(2000.0, "t3");
        assert!((state.progress() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_declining_health() {
        let mut state = DimensionState::new(GrowthDimension::Tests, 1000.0, "count");
        state.update(500.0, "t1");
        state.update(400.0, "t2");  // declined
        assert_eq!(state.health, DimensionHealth::Declining);
    }
}
