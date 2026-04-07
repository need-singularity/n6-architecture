use serde::{Deserialize, Serialize};

use super::registrar::RegistrationResult;

/// Severity / importance level of a notification.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NotificationLevel {
    Info,
    Discovery,
    Breakthrough,
}

/// A notification generated from registration results.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Notification {
    pub level: NotificationLevel,
    pub message: String,
    pub details: Vec<String>,
}

/// Generate notifications from registration results.
pub fn notify(registrations: &[RegistrationResult]) -> Vec<Notification> {
    let mut notifications = Vec::new();

    for reg in registrations {
        if !reg.success {
            continue;
        }

        let level = determine_level(&reg.registered_to);
        let message = match level {
            NotificationLevel::Breakthrough => {
                format!("[BT] New breakthrough candidate: {}", reg.discovery_id)
            }
            NotificationLevel::Discovery => {
                format!("[DISCOVERY] New finding registered: {}", reg.discovery_id)
            }
            NotificationLevel::Info => {
                format!("[INFO] Registered: {}", reg.discovery_id)
            }
        };

        let details = reg
            .registered_to
            .iter()
            .map(|t| format!("-> {}", t))
            .chain(std::iter::once(reg.details.clone()))
            .collect();

        notifications.push(Notification {
            level,
            message,
            details,
        });
    }

    notifications
}

/// Determine notification level based on which backends were targeted.
fn determine_level(targets: &[String]) -> NotificationLevel {
    if targets.iter().any(|t| t == "bt_list" || t == "bt_candidates") {
        NotificationLevel::Breakthrough
    } else if targets
        .iter()
        .any(|t| t == "math_atlas" || t == "hypotheses" || t == "lens_forge")
    {
        NotificationLevel::Discovery
    } else {
        NotificationLevel::Info
    }
}

/// Format notifications for terminal display.
pub fn format_notifications(notifications: &[Notification]) -> String {
    let mut output = String::new();
    for n in notifications {
        let prefix = match n.level {
            NotificationLevel::Breakthrough => "*** ",
            NotificationLevel::Discovery => "**  ",
            NotificationLevel::Info => "*   ",
        };
        output.push_str(&format!("{}{}\n", prefix, n.message));
        for d in &n.details {
            output.push_str(&format!("      {}\n", d));
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notify_breakthrough() {
        let regs = vec![RegistrationResult {
            discovery_id: "disc-bt".to_string(),
            registered_to: vec!["bt_list".to_string(), "graph".to_string()],
            success: true,
            details: "BT candidate".to_string(),
        }];
        let notes = notify(&regs);
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].level, NotificationLevel::Breakthrough);
    }

    #[test]
    fn test_notify_discovery() {
        let regs = vec![RegistrationResult {
            discovery_id: "disc-atlas".to_string(),
            registered_to: vec!["math_atlas".to_string()],
            success: true,
            details: "constant".to_string(),
        }];
        let notes = notify(&regs);
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].level, NotificationLevel::Discovery);
    }

    #[test]
    fn test_notify_info() {
        let regs = vec![RegistrationResult {
            discovery_id: "disc-g".to_string(),
            registered_to: vec!["graph".to_string()],
            success: true,
            details: "pattern".to_string(),
        }];
        let notes = notify(&regs);
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].level, NotificationLevel::Info);
    }

    #[test]
    fn test_format_output() {
        let notes = vec![Notification {
            level: NotificationLevel::Breakthrough,
            message: "BT found".to_string(),
            details: vec!["detail1".to_string()],
        }];
        let out = format_notifications(&notes);
        assert!(out.contains("***"));
        assert!(out.contains("BT found"));
    }
}
