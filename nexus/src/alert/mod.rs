//! Alert notification system for growth events and anomalies.
/// Alert & Notification System — real-time alerting when telescope lenses
/// detect significant patterns during discovery scans.

/// Severity level for alerts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
    Discovery,
}

impl AlertLevel {
    /// Numeric priority (higher = more important).
    pub fn priority(&self) -> u32 {
        match self {
            Self::Info => 0,
            Self::Warning => 1,
            Self::Critical => 2,
            Self::Discovery => 3,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Info => "INFO",
            Self::Warning => "WARNING",
            Self::Critical => "CRITICAL",
            Self::Discovery => "DISCOVERY",
        }
    }
}

/// A single alert emitted by a lens scan.
#[derive(Debug, Clone)]
pub struct Alert {
    pub level: AlertLevel,
    /// Name of the lens that produced this alert.
    pub source_lens: String,
    /// Identifier for the detected pattern.
    pub pattern_id: String,
    /// Confidence in the detection, 0.0–1.0.
    pub confidence: f64,
    /// Unix timestamp (seconds).
    pub timestamp: u64,
    /// Human-readable description.
    pub message: String,
}

impl Alert {
    /// Create a new alert.
    pub fn new(
        level: AlertLevel,
        source_lens: impl Into<String>,
        pattern_id: impl Into<String>,
        confidence: f64,
        timestamp: u64,
        message: impl Into<String>,
    ) -> Self {
        Self {
            level,
            source_lens: source_lens.into(),
            pattern_id: pattern_id.into(),
            confidence: confidence.clamp(0.0, 1.0),
            timestamp,
            message: message.into(),
        }
    }

    /// Deduplication key: (source_lens, pattern_id).
    #[allow(dead_code)]
    fn dedup_key(&self) -> (&str, &str) {
        (&self.source_lens, &self.pattern_id)
    }
}

/// Create a special Discovery-level alert for new findings.
pub fn discovery_alert(
    lens_name: impl Into<String>,
    pattern: impl Into<String>,
    confidence: f64,
) -> Alert {
    let lens = lens_name.into();
    let pat = pattern.into();
    let msg = format!(
        "Discovery by lens '{}': pattern '{}' (confidence {:.3})",
        lens, pat, confidence
    );
    Alert::new(AlertLevel::Discovery, lens, pat, confidence, 0, msg)
}

/// Filter alerts, keeping only those at or above `min_level`.
pub fn filter_by_level(alerts: &[Alert], min_level: AlertLevel) -> Vec<Alert> {
    alerts
        .iter()
        .filter(|a| a.level >= min_level)
        .cloned()
        .collect()
}

/// Engine that collects alerts, deduplicates, and prioritizes them.
/// Deduplication keeps the alert with the highest confidence for each
/// (source_lens, pattern_id) pair.
#[derive(Debug, Default)]
pub struct AlertEngine {
    alerts: Vec<Alert>,
}

impl AlertEngine {
    pub fn new() -> Self {
        Self { alerts: Vec::new() }
    }

    /// Ingest a new alert. If a duplicate key already exists, keep whichever
    /// has higher confidence.
    pub fn ingest(&mut self, alert: Alert) {
        let key = (alert.source_lens.clone(), alert.pattern_id.clone());
        if let Some(existing) = self.alerts.iter_mut().find(|a| {
            a.source_lens == key.0 && a.pattern_id == key.1
        }) {
            if alert.confidence > existing.confidence {
                *existing = alert;
            }
        } else {
            self.alerts.push(alert);
        }
    }

    /// Return all alerts sorted by priority (highest first), then by
    /// confidence descending within the same level.
    pub fn prioritized(&self) -> Vec<Alert> {
        let mut sorted = self.alerts.clone();
        sorted.sort_by(|a, b| {
            b.level
                .priority()
                .cmp(&a.level.priority())
                .then(b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal))
        });
        sorted
    }

    /// Number of stored (deduplicated) alerts.
    pub fn count(&self) -> usize {
        self.alerts.len()
    }

    /// Filter stored alerts by minimum level.
    pub fn filter(&self, min_level: AlertLevel) -> Vec<Alert> {
        filter_by_level(&self.alerts, min_level)
    }

    /// Clear all alerts.
    pub fn clear(&mut self) {
        self.alerts.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_level_ordering() {
        assert!(AlertLevel::Info < AlertLevel::Warning);
        assert!(AlertLevel::Warning < AlertLevel::Critical);
        assert!(AlertLevel::Critical < AlertLevel::Discovery);
    }

    #[test]
    fn test_filter_by_level() {
        let alerts = vec![
            Alert::new(AlertLevel::Info, "lens_a", "p1", 0.5, 100, "info"),
            Alert::new(AlertLevel::Warning, "lens_b", "p2", 0.7, 101, "warn"),
            Alert::new(AlertLevel::Critical, "lens_c", "p3", 0.9, 102, "crit"),
            Alert::new(AlertLevel::Discovery, "lens_d", "p4", 0.95, 103, "disc"),
        ];
        let critical_up = filter_by_level(&alerts, AlertLevel::Critical);
        assert_eq!(critical_up.len(), 2);
        assert!(critical_up.iter().all(|a| a.level >= AlertLevel::Critical));
    }

    #[test]
    fn test_discovery_alert_constructor() {
        let a = discovery_alert("consciousness", "spiral_pattern", 0.88);
        assert_eq!(a.level, AlertLevel::Discovery);
        assert_eq!(a.source_lens, "consciousness");
        assert_eq!(a.pattern_id, "spiral_pattern");
        assert!((a.confidence - 0.88).abs() < 1e-9);
        assert!(a.message.contains("consciousness"));
    }

    #[test]
    fn test_engine_deduplication() {
        let mut engine = AlertEngine::new();
        engine.ingest(Alert::new(AlertLevel::Warning, "lens_a", "p1", 0.5, 1, "first"));
        engine.ingest(Alert::new(AlertLevel::Warning, "lens_a", "p1", 0.8, 2, "second"));
        // Same key, higher confidence wins
        assert_eq!(engine.count(), 1);
        let alerts = engine.prioritized();
        assert!((alerts[0].confidence - 0.8).abs() < 1e-9);
        assert_eq!(alerts[0].message, "second");
    }

    #[test]
    fn test_engine_prioritization() {
        let mut engine = AlertEngine::new();
        engine.ingest(Alert::new(AlertLevel::Info, "a", "p1", 0.9, 1, "info"));
        engine.ingest(Alert::new(AlertLevel::Discovery, "b", "p2", 0.6, 2, "disc"));
        engine.ingest(Alert::new(AlertLevel::Critical, "c", "p3", 0.95, 3, "crit"));
        let sorted = engine.prioritized();
        // Discovery first, then Critical, then Info
        assert_eq!(sorted[0].level, AlertLevel::Discovery);
        assert_eq!(sorted[1].level, AlertLevel::Critical);
        assert_eq!(sorted[2].level, AlertLevel::Info);
    }

    #[test]
    fn test_confidence_clamping() {
        let a = Alert::new(AlertLevel::Info, "x", "y", 1.5, 0, "over");
        assert!((a.confidence - 1.0).abs() < 1e-9);
        let b = Alert::new(AlertLevel::Info, "x", "y", -0.3, 0, "under");
        assert!((b.confidence - 0.0).abs() < 1e-9);
    }
}
