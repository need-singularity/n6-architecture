//! Distributed computing support for parallel scans.
/// Distributed computing — domain-parallel scan scheduling across worker nodes.
/// No external crates; pure std.

/// Status of a worker node.
#[derive(Debug, Clone, PartialEq)]
pub enum NodeStatus {
    Idle,
    Busy,
    Offline,
}

/// A worker node in the distributed scan cluster.
#[derive(Debug, Clone)]
pub struct WorkerNode {
    pub id: String,
    pub address: String,
    pub status: NodeStatus,
    pub assigned_domains: Vec<String>,
}

/// Distributes telescope scan domains across available workers (round-robin).
pub struct DistributedScheduler {
    workers: Vec<WorkerNode>,
}

impl DistributedScheduler {
    pub fn new() -> Self {
        Self {
            workers: Vec::new(),
        }
    }

    /// Register a worker node.
    pub fn add_worker(&mut self, node: WorkerNode) {
        self.workers.push(node);
    }

    /// Number of registered workers.
    pub fn worker_count(&self) -> usize {
        self.workers.len()
    }

    /// Return only workers that are not Offline.
    fn available_workers(&self) -> Vec<&WorkerNode> {
        self.workers
            .iter()
            .filter(|w| w.status != NodeStatus::Offline)
            .collect()
    }

    /// Distribute `domains` across available (non-Offline) workers in round-robin order.
    /// Returns `Vec<(worker_id, assigned_domains)>`.
    /// If no workers are available, returns an empty vec.
    pub fn distribute_scan(&self, domains: &[String]) -> Vec<(String, Vec<String>)> {
        let available = self.available_workers();
        if available.is_empty() {
            return Vec::new();
        }

        // Build assignment buckets — one per available worker
        let n = available.len();
        let mut buckets: Vec<Vec<String>> = vec![Vec::new(); n];

        for (i, domain) in domains.iter().enumerate() {
            buckets[i % n].push(domain.clone());
        }

        available
            .iter()
            .zip(buckets.into_iter())
            .filter(|(_, b)| !b.is_empty())
            .map(|(w, b)| (w.id.clone(), b))
            .collect()
    }

    /// Merge per-worker result lists into a single flat list.
    pub fn merge_results(&self, results: &[(String, Vec<String>)]) -> Vec<String> {
        results.iter().flat_map(|(_, v)| v.clone()).collect()
    }
}

impl Default for DistributedScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_worker(id: &str, status: NodeStatus) -> WorkerNode {
        WorkerNode {
            id: id.to_string(),
            address: format!("127.0.0.1:{}", id),
            status,
            assigned_domains: Vec::new(),
        }
    }

    #[test]
    fn test_distribute_round_robin() {
        let mut sched = DistributedScheduler::new();
        sched.add_worker(make_worker("w1", NodeStatus::Idle));
        sched.add_worker(make_worker("w2", NodeStatus::Idle));
        sched.add_worker(make_worker("w3", NodeStatus::Idle));

        let domains: Vec<String> = vec![
            "physics", "biology", "energy", "chip", "fusion", "crypto",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let plan = sched.distribute_scan(&domains);
        assert_eq!(plan.len(), 3);

        // Each worker gets exactly 2 domains (6 / 3 = 2)
        for (_, assigned) in &plan {
            assert_eq!(assigned.len(), 2);
        }
    }

    #[test]
    fn test_distribute_skips_offline() {
        let mut sched = DistributedScheduler::new();
        sched.add_worker(make_worker("w1", NodeStatus::Idle));
        sched.add_worker(make_worker("w2", NodeStatus::Offline));
        sched.add_worker(make_worker("w3", NodeStatus::Busy)); // Busy but not Offline

        let domains: Vec<String> = vec!["a", "b", "c", "d"]
            .into_iter()
            .map(String::from)
            .collect();

        let plan = sched.distribute_scan(&domains);
        // Only w1 and w3 are available
        assert_eq!(plan.len(), 2);
        let total: usize = plan.iter().map(|(_, v)| v.len()).sum();
        assert_eq!(total, 4);
    }

    #[test]
    fn test_distribute_empty_domains() {
        let mut sched = DistributedScheduler::new();
        sched.add_worker(make_worker("w1", NodeStatus::Idle));
        let plan = sched.distribute_scan(&[]);
        assert!(plan.is_empty());
    }

    #[test]
    fn test_distribute_no_workers() {
        let sched = DistributedScheduler::new();
        let domains: Vec<String> = vec!["physics".to_string()];
        let plan = sched.distribute_scan(&domains);
        assert!(plan.is_empty());
    }

    #[test]
    fn test_merge_results() {
        let sched = DistributedScheduler::new();
        let results = vec![
            ("w1".to_string(), vec!["r1".to_string(), "r2".to_string()]),
            ("w2".to_string(), vec!["r3".to_string()]),
        ];
        let merged = sched.merge_results(&results);
        assert_eq!(merged, vec!["r1", "r2", "r3"]);
    }
}
