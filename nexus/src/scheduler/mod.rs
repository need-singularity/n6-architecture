//! Task scheduling for growth cycles and background jobs.
/// Periodic task scheduler — time-based scan automation.
/// No external crates.

#[derive(Debug, Clone)]
pub struct ScheduledTask {
    pub name: String,
    pub command: String,
    pub interval_secs: u64,
    pub last_run: Option<String>,
    pub enabled: bool,
}

pub struct Scheduler {
    tasks: Vec<ScheduledTask>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    /// Add a task. Replaces if same name exists.
    pub fn add_task(&mut self, task: ScheduledTask) {
        self.tasks.retain(|t| t.name != task.name);
        self.tasks.push(task);
    }

    /// Remove a task by name. Returns true if removed.
    pub fn remove_task(&mut self, name: &str) -> bool {
        let before = self.tasks.len();
        self.tasks.retain(|t| t.name != name);
        self.tasks.len() < before
    }

    /// List all tasks.
    pub fn list(&self) -> &[ScheduledTask] {
        &self.tasks
    }

    /// Return tasks that are due for execution.
    /// A task is due if:
    ///   - it is enabled
    ///   - it has never been run (last_run is None), OR
    ///   - the elapsed time since last_run >= interval_secs
    ///
    /// Since we don't have a real clock (no external crates), we check
    /// against a provided `now_secs` epoch value.
    pub fn due_tasks(&self, now_secs: u64) -> Vec<&ScheduledTask> {
        self.tasks.iter().filter(|t| {
            if !t.enabled {
                return false;
            }
            match &t.last_run {
                None => true,
                Some(ts) => {
                    // Parse stored epoch seconds
                    if let Ok(last) = ts.parse::<u64>() {
                        now_secs.saturating_sub(last) >= t.interval_secs
                    } else {
                        true // unparseable timestamp = treat as due
                    }
                }
            }
        }).collect()
    }

    /// Mark a task as just run at the given epoch time.
    pub fn mark_run(&mut self, name: &str, now_secs: u64) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.name == name) {
            task.last_run = Some(now_secs.to_string());
        }
    }

    /// Toggle enabled/disabled for a task.
    pub fn toggle(&mut self, name: &str) -> Option<bool> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.name == name) {
            task.enabled = !task.enabled;
            Some(task.enabled)
        } else {
            None
        }
    }

    /// Number of tasks.
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a set of default n=6 scanning tasks.
pub fn default_tasks() -> Vec<ScheduledTask> {
    vec![
        ScheduledTask {
            name: "daily-full-scan".to_string(),
            command: "scan all --full".to_string(),
            interval_secs: 86400, // 24h
            last_run: None,
            enabled: true,
        },
        ScheduledTask {
            name: "hourly-quick-scan".to_string(),
            command: "scan physics --lenses consciousness,topology,causal".to_string(),
            interval_secs: 3600, // 1h
            last_run: None,
            enabled: true,
        },
        ScheduledTask {
            name: "weekly-ouroboros".to_string(),
            command: "evolve all --max-cycles 6".to_string(),
            interval_secs: 604800, // 7d
            last_run: None,
            enabled: false, // disabled by default (heavy)
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_list() {
        let mut sched = Scheduler::new();
        sched.add_task(ScheduledTask {
            name: "test-task".to_string(),
            command: "scan physics".to_string(),
            interval_secs: 3600,
            last_run: None,
            enabled: true,
        });
        assert_eq!(sched.len(), 1);
        assert_eq!(sched.list()[0].name, "test-task");
    }

    #[test]
    fn test_remove() {
        let mut sched = Scheduler::new();
        sched.add_task(ScheduledTask {
            name: "a".to_string(),
            command: "cmd-a".to_string(),
            interval_secs: 60,
            last_run: None,
            enabled: true,
        });
        sched.add_task(ScheduledTask {
            name: "b".to_string(),
            command: "cmd-b".to_string(),
            interval_secs: 120,
            last_run: None,
            enabled: true,
        });
        assert!(sched.remove_task("a"));
        assert_eq!(sched.len(), 1);
        assert!(!sched.remove_task("nonexistent"));
    }

    #[test]
    fn test_due_tasks() {
        let mut sched = Scheduler::new();
        sched.add_task(ScheduledTask {
            name: "t1".to_string(),
            command: "cmd1".to_string(),
            interval_secs: 100,
            last_run: None, // never run -> due
            enabled: true,
        });
        sched.add_task(ScheduledTask {
            name: "t2".to_string(),
            command: "cmd2".to_string(),
            interval_secs: 100,
            last_run: Some("1000".to_string()), // ran at 1000
            enabled: true,
        });
        sched.add_task(ScheduledTask {
            name: "t3".to_string(),
            command: "cmd3".to_string(),
            interval_secs: 100,
            last_run: None,
            enabled: false, // disabled
        });

        // At time 1050: t1 is due (never run), t2 not due (50 < 100), t3 disabled
        let due = sched.due_tasks(1050);
        assert_eq!(due.len(), 1);
        assert_eq!(due[0].name, "t1");

        // At time 1150: t1 due, t2 due (150 >= 100)
        let due2 = sched.due_tasks(1150);
        assert_eq!(due2.len(), 2);
    }

    #[test]
    fn test_mark_run_and_toggle() {
        let mut sched = Scheduler::new();
        sched.add_task(ScheduledTask {
            name: "t1".to_string(),
            command: "cmd".to_string(),
            interval_secs: 60,
            last_run: None,
            enabled: true,
        });

        sched.mark_run("t1", 5000);
        assert_eq!(sched.list()[0].last_run, Some("5000".to_string()));

        // Not due at 5030 (30 < 60)
        assert!(sched.due_tasks(5030).is_empty());
        // Due at 5060
        assert_eq!(sched.due_tasks(5060).len(), 1);

        // Toggle off
        assert_eq!(sched.toggle("t1"), Some(false));
        assert!(sched.due_tasks(5060).is_empty());
    }
}
