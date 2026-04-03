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
