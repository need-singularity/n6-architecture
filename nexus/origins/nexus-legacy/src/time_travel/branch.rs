//! Branch management — track diverging exploration paths.

use super::snapshot::TimeTravel;

/// A named branch representing an alternative exploration path.
#[derive(Debug, Clone)]
pub struct Branch {
    /// Branch name.
    pub name: String,
    /// Snapshot ID this branch was created from.
    pub parent_snapshot: String,
    /// Snapshot ID for the branch head.
    pub head_snapshot: String,
    /// Description of why this branch was created.
    pub reason: String,
}

/// Branch manager — track multiple exploration branches.
pub struct BranchManager {
    branches: Vec<Branch>,
}

impl BranchManager {
    pub fn new() -> Self {
        Self {
            branches: Vec::new(),
        }
    }

    /// Create a new branch from an existing snapshot.
    pub fn create_branch(
        &mut self,
        time_travel: &mut TimeTravel,
        snapshot_id: &str,
        branch_name: &str,
        reason: &str,
    ) -> Option<Branch> {
        let head_id = time_travel.branch(snapshot_id, branch_name)?;

        let branch = Branch {
            name: branch_name.to_string(),
            parent_snapshot: snapshot_id.to_string(),
            head_snapshot: head_id,
            reason: reason.to_string(),
        };

        self.branches.push(branch.clone());
        Some(branch)
    }

    /// List all branches.
    pub fn list_branches(&self) -> &[Branch] {
        &self.branches
    }

    /// Find a branch by name.
    pub fn get_branch(&self, name: &str) -> Option<&Branch> {
        self.branches.iter().find(|b| b.name == name)
    }

    /// Number of branches.
    pub fn count(&self) -> usize {
        self.branches.len()
    }
}

impl Default for BranchManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::persistence::DiscoveryGraph;

    #[test]
    fn test_branch_manager_creation() {
        let mgr = BranchManager::new();
        assert_eq!(mgr.count(), 0);
    }

    #[test]
    fn test_create_branch() {
        let mut mgr = BranchManager::new();
        let mut tt = TimeTravel::new("/tmp/nexus-branch-test");
        let graph = DiscoveryGraph::new();
        let snap_id = tt.save_snapshot(&graph, "kb", 5, "base");

        let branch = mgr.create_branch(&mut tt, &snap_id, "experiment-x", "testing hypothesis").unwrap();
        assert_eq!(branch.name, "experiment-x");
        assert_eq!(branch.parent_snapshot, snap_id);
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_list_branches() {
        let mut mgr = BranchManager::new();
        let mut tt = TimeTravel::new("/tmp/nexus-branch-list-test");
        let graph = DiscoveryGraph::new();
        let snap_id = tt.save_snapshot(&graph, "kb", 5, "base");

        mgr.create_branch(&mut tt, &snap_id, "branch-a", "reason a");
        mgr.create_branch(&mut tt, &snap_id, "branch-b", "reason b");

        assert_eq!(mgr.list_branches().len(), 2);
    }

    #[test]
    fn test_get_branch() {
        let mut mgr = BranchManager::new();
        let mut tt = TimeTravel::new("/tmp/nexus-branch-get-test");
        let graph = DiscoveryGraph::new();
        let snap_id = tt.save_snapshot(&graph, "kb", 5, "base");

        mgr.create_branch(&mut tt, &snap_id, "my-branch", "test");

        let found = mgr.get_branch("my-branch");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "my-branch");

        assert!(mgr.get_branch("nonexistent").is_none());
    }

    #[test]
    fn test_branch_from_nonexistent_snapshot() {
        let mut mgr = BranchManager::new();
        let mut tt = TimeTravel::new("/tmp/nexus-branch-noexist-test");
        let result = mgr.create_branch(&mut tt, "no-such-snap", "test", "test");
        assert!(result.is_none());
    }
}
