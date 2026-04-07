/// Event bus utilities — typed subscriptions and event filtering.

use super::{Event, EventBus};

impl EventBus {
    /// Subscribe only to discovery events.
    pub fn on_discovery(&mut self, handler: impl Fn(&str, &str, f64) + 'static) {
        self.on(move |event| {
            if let Event::DiscoveryMade { id, discovery_type, confidence } = event {
                handler(id, discovery_type, *confidence);
            }
        });
    }

    /// Subscribe only to anomaly events.
    pub fn on_anomaly(&mut self, handler: impl Fn(&str, f64) + 'static) {
        self.on(move |event| {
            if let Event::Anomaly { description, severity } = event {
                handler(description, *severity);
            }
        });
    }

    /// Subscribe only to BT candidate events.
    pub fn on_bt_candidate(&mut self, handler: impl Fn(&str, &[String]) + 'static) {
        self.on(move |event| {
            if let Event::BtCandidate { title, domains } = event {
                handler(title, domains);
            }
        });
    }

    /// Get the last N events from history.
    pub fn recent(&self, n: usize) -> &[(String, Event)] {
        let len = self.history.len();
        if n >= len {
            self.history()
        } else {
            &self.history()[len - n..]
        }
    }

    /// Count events by type.
    pub fn count_by_type(&self) -> Vec<(&'static str, usize)> {
        let tags = [
            "discovery", "lens_forged", "experiment",
            "bt_candidate", "anomaly", "scan_completed",
        ];
        tags.iter()
            .map(|&tag| {
                let count = self.history.iter().filter(|(_, e)| e.type_tag() == tag).count();
                (tag, count)
            })
            .filter(|(_, count)| *count > 0)
            .collect()
    }
}
