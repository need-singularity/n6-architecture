/// Event handler presets — ready-made handlers for common patterns.

use super::Event;

/// A logging handler that prints every event to stdout.
pub fn logging_handler() -> impl Fn(&Event) {
    move |event: &Event| {
        println!("[EVENT] {}", event.summary());
    }
}

/// A handler that collects discovery IDs into a shared accumulator.
/// Returns a closure and a way to read the accumulated IDs.
pub fn discovery_collector() -> (impl Fn(&Event), std::sync::Arc<std::sync::Mutex<Vec<String>>>) {
    let ids = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let ids_clone = ids.clone();
    let handler = move |event: &Event| {
        if let Event::DiscoveryMade { id, .. } = event {
            if let Ok(mut vec) = ids_clone.lock() {
                vec.push(id.clone());
            }
        }
    };
    (handler, ids)
}

/// A handler that counts events by type (thread-safe).
pub fn counting_handler() -> (impl Fn(&Event), std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, usize>>>) {
    let counts = std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));
    let counts_clone = counts.clone();
    let handler = move |event: &Event| {
        if let Ok(mut map) = counts_clone.lock() {
            *map.entry(event.type_tag().to_string()).or_insert(0) += 1;
        }
    };
    (handler, counts)
}
