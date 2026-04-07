//! Event system for inter-module communication and triggers.
/// Event system — publish-subscribe event bus for NEXUS-6 discovery engine.
pub mod bus;
pub mod handler;

/// Events emitted during discovery engine operation.
#[derive(Debug, Clone)]
pub enum Event {
    DiscoveryMade {
        id: String,
        discovery_type: String,
        confidence: f64,
    },
    LensForged {
        name: String,
    },
    ExperimentCompleted {
        exp_type: String,
        result_summary: String,
    },
    BtCandidate {
        title: String,
        domains: Vec<String>,
    },
    Anomaly {
        description: String,
        severity: f64,
    },
    ScanCompleted {
        domain: String,
        discoveries: usize,
    },
}

impl Event {
    /// Return a string tag for the event type (for filtering).
    pub fn type_tag(&self) -> &'static str {
        match self {
            Event::DiscoveryMade { .. } => "discovery",
            Event::LensForged { .. } => "lens_forged",
            Event::ExperimentCompleted { .. } => "experiment",
            Event::BtCandidate { .. } => "bt_candidate",
            Event::Anomaly { .. } => "anomaly",
            Event::ScanCompleted { .. } => "scan_completed",
        }
    }

    /// Human-readable one-line summary.
    pub fn summary(&self) -> String {
        match self {
            Event::DiscoveryMade { id, discovery_type, confidence } => {
                format!("Discovery [{}] type={} confidence={:.2}", id, discovery_type, confidence)
            }
            Event::LensForged { name } => {
                format!("Lens forged: {}", name)
            }
            Event::ExperimentCompleted { exp_type, result_summary } => {
                format!("Experiment {} completed: {}", exp_type, result_summary)
            }
            Event::BtCandidate { title, domains } => {
                format!("BT candidate: {} (domains: {})", title, domains.join(", "))
            }
            Event::Anomaly { description, severity } => {
                format!("Anomaly (severity={:.2}): {}", severity, description)
            }
            Event::ScanCompleted { domain, discoveries } => {
                format!("Scan of {} completed: {} discoveries", domain, discoveries)
            }
        }
    }
}

/// Event bus that dispatches events to registered handlers and keeps history.
pub struct EventBus {
    handlers: Vec<Box<dyn Fn(&Event)>>,
    history: Vec<(String, Event)>, // (timestamp, event)
}

impl EventBus {
    /// Create a new empty event bus.
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            history: Vec::new(),
        }
    }

    /// Emit an event: run all handlers and record in history.
    pub fn emit(&mut self, event: Event) {
        // Generate timestamp
        let ts = Self::now_timestamp();

        // Notify all handlers
        for handler in &self.handlers {
            handler(&event);
        }

        // Record
        self.history.push((ts, event));
    }

    /// Register an event handler.
    pub fn on(&mut self, handler: impl Fn(&Event) + 'static) {
        self.handlers.push(Box::new(handler));
    }

    /// Full event history.
    pub fn history(&self) -> &[(String, Event)] {
        &self.history
    }

    /// Filter history by event type tag.
    pub fn history_by_type(&self, event_type: &str) -> Vec<&Event> {
        self.history
            .iter()
            .filter(|(_, e)| e.type_tag() == event_type)
            .map(|(_, e)| e)
            .collect()
    }

    /// Number of events in history.
    pub fn event_count(&self) -> usize {
        self.history.len()
    }

    /// Clear all history (handlers are preserved).
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Simple monotonic timestamp (no external crate).
    fn now_timestamp() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let dur = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        format!("{}.{:03}", dur.as_secs(), dur.subsec_millis())
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_event_type_tags_n6_variants() {
        let events = vec![
            Event::DiscoveryMade { id: "D-1".into(), discovery_type: "n6".into(), confidence: 0.95 },
            Event::LensForged { name: "sigma_lens".into() },
            Event::ExperimentCompleted { exp_type: "fusion".into(), result_summary: "ok".into() },
            Event::BtCandidate { title: "BT-6".into(), domains: vec!["ai".into(), "chip".into()] },
            Event::Anomaly { description: "phi spike".into(), severity: 0.12 },
            Event::ScanCompleted { domain: "energy".into(), discoveries: 24 },
        ];
        let tags: Vec<&str> = events.iter().map(|e| e.type_tag()).collect();
        assert_eq!(tags.len(), 6);
        assert_eq!(tags, vec!["discovery", "lens_forged", "experiment", "bt_candidate", "anomaly", "scan_completed"]);
    }

    #[test]
    fn test_event_summary_format() {
        let event = Event::ScanCompleted { domain: "fusion".into(), discoveries: 12 };
        let summary = event.summary();
        assert!(summary.contains("fusion"));
        assert!(summary.contains("12"));
    }

    #[test]
    fn test_event_bus_emit_and_history() {
        let mut bus = EventBus::new();
        assert_eq!(bus.event_count(), 0);
        for i in 0..6 {
            bus.emit(Event::DiscoveryMade {
                id: format!("D-{}", i),
                discovery_type: "constant".into(),
                confidence: 0.5 + i as f64 * 0.1,
            });
        }
        assert_eq!(bus.event_count(), 6);
        assert_eq!(bus.history().len(), 6);
    }

    #[test]
    fn test_event_bus_filter_and_handler() {
        let mut bus = EventBus::new();
        let count = Arc::new(Mutex::new(0usize));
        let count_clone = count.clone();
        bus.on(move |_event| {
            *count_clone.lock().unwrap() += 1;
        });
        bus.emit(Event::Anomaly { description: "test".into(), severity: 6.0 });
        bus.emit(Event::LensForged { name: "phi_lens".into() });
        bus.emit(Event::Anomaly { description: "test2".into(), severity: 12.0 });
        assert_eq!(*count.lock().unwrap(), 3);
        let anomalies = bus.history_by_type("anomaly");
        assert_eq!(anomalies.len(), 2);
        let forged = bus.history_by_type("lens_forged");
        assert_eq!(forged.len(), 1);
    }

    #[test]
    fn test_event_bus_recent_and_clear() {
        let mut bus = EventBus::new();
        for i in 0..12 {
            bus.emit(Event::ScanCompleted { domain: format!("d{}", i), discoveries: i });
        }
        assert_eq!(bus.event_count(), 12);
        let recent = bus.recent(6);
        assert_eq!(recent.len(), 6);
        let all = bus.recent(100);
        assert_eq!(all.len(), 12);
        bus.clear_history();
        assert_eq!(bus.event_count(), 0);
    }

    #[test]
    fn test_event_bus_count_by_type() {
        let mut bus = EventBus::new();
        bus.emit(Event::DiscoveryMade { id: "a".into(), discovery_type: "x".into(), confidence: 1.0 });
        bus.emit(Event::DiscoveryMade { id: "b".into(), discovery_type: "x".into(), confidence: 1.0 });
        bus.emit(Event::Anomaly { description: "z".into(), severity: 0.5 });
        let counts = bus.count_by_type();
        assert!(counts.contains(&("discovery", 2)));
        assert!(counts.contains(&("anomaly", 1)));
        assert!(!counts.iter().any(|(tag, _)| *tag == "lens_forged"));
    }

    #[test]
    fn test_discovery_collector_handler() {
        let mut bus = EventBus::new();
        let (hdl, ids) = handler::discovery_collector();
        bus.on(hdl);
        bus.emit(Event::DiscoveryMade { id: "N6-1".into(), discovery_type: "exact".into(), confidence: 1.0 });
        bus.emit(Event::LensForged { name: "test".into() });
        bus.emit(Event::DiscoveryMade { id: "N6-2".into(), discovery_type: "close".into(), confidence: 0.8 });
        let collected = ids.lock().unwrap();
        assert_eq!(collected.len(), 2);
        assert_eq!(collected[0], "N6-1");
        assert_eq!(collected[1], "N6-2");
    }
}
