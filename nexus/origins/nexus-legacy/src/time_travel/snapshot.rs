//! Time Travel — snapshot and rollback of discovery engine state.

use std::fs;
use std::io;
use std::path::Path;

use crate::graph::persistence::DiscoveryGraph;

/// A point-in-time snapshot of the discovery engine state.
#[derive(Debug, Clone)]
pub struct Snapshot {
    /// Unique snapshot identifier.
    pub id: String,
    /// When the snapshot was taken (ISO-like string).
    pub timestamp: String,
    /// Serialized discovery graph state.
    pub graph_state: String,
    /// Serialized knowledge base / scan records.
    pub kb_state: String,
    /// Number of entries in the registry at snapshot time.
    pub registry_size: usize,
    /// Human-readable description.
    pub description: String,
}

/// The TimeTravel engine — manages snapshots for undo/branch operations.
pub struct TimeTravel {
    snapshots: Vec<Snapshot>,
    path: String,
}

impl TimeTravel {
    /// Create a new TimeTravel instance with a storage directory.
    pub fn new(path: &str) -> Self {
        Self {
            snapshots: Vec::new(),
            path: path.to_string(),
        }
    }

    /// Save a snapshot of the current state.
    ///
    /// Returns the snapshot ID.
    pub fn save_snapshot(
        &mut self,
        graph: &DiscoveryGraph,
        kb_state: &str,
        registry_size: usize,
        description: &str,
    ) -> String {
        let id = format!("snap-{:04}", self.snapshots.len() + 1);
        let timestamp = format!("snapshot-{}", self.snapshots.len() + 1);

        let graph_state = serde_json::to_string(graph).unwrap_or_default();

        let snapshot = Snapshot {
            id: id.clone(),
            timestamp,
            graph_state,
            kb_state: kb_state.to_string(),
            registry_size,
            description: description.to_string(),
        };

        // Save to disk if path exists
        let _ = self.persist_snapshot(&snapshot);

        self.snapshots.push(snapshot);
        id
    }

    /// List all snapshots (most recent first).
    pub fn list_snapshots(&self) -> Vec<&Snapshot> {
        self.snapshots.iter().rev().collect()
    }

    /// Get a snapshot by ID.
    pub fn get_snapshot(&self, snapshot_id: &str) -> Option<&Snapshot> {
        self.snapshots.iter().find(|s| s.id == snapshot_id)
    }

    /// Rollback: restore state from a snapshot.
    ///
    /// Returns the snapshot's graph state for the caller to apply.
    pub fn rollback(&self, snapshot_id: &str) -> Option<DiscoveryGraph> {
        let snapshot = self.get_snapshot(snapshot_id)?;
        let graph: DiscoveryGraph = serde_json::from_str(&snapshot.graph_state).ok()?;
        Some(graph)
    }

    /// Branch: create a new snapshot based on an existing one with a branch label.
    ///
    /// Returns the new branch snapshot ID.
    pub fn branch(&mut self, snapshot_id: &str, branch_name: &str) -> Option<String> {
        let source = self.get_snapshot(snapshot_id)?.clone();

        let new_id = format!("{}-branch-{}", source.id, branch_name);
        let branched = Snapshot {
            id: new_id.clone(),
            timestamp: format!("branched-from-{}", source.id),
            graph_state: source.graph_state,
            kb_state: source.kb_state,
            registry_size: source.registry_size,
            description: format!("Branch '{}' from {}: {}", branch_name, source.id, source.description),
        };

        let _ = self.persist_snapshot(&branched);
        self.snapshots.push(branched);
        Some(new_id)
    }

    /// Number of snapshots stored.
    pub fn count(&self) -> usize {
        self.snapshots.len()
    }

    /// Persist a snapshot to disk.
    fn persist_snapshot(&self, snapshot: &Snapshot) -> io::Result<()> {
        let dir = Path::new(&self.path);
        fs::create_dir_all(dir)?;
        let file_path = dir.join(format!("{}.json", snapshot.id));
        let json = serde_json::to_string_pretty(snapshot)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        fs::write(file_path, json)?;
        Ok(())
    }
}

// Implement Serialize/Deserialize for Snapshot (for disk persistence)
impl serde::Serialize for Snapshot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Snapshot", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.serialize_field("graph_state", &self.graph_state)?;
        state.serialize_field("kb_state", &self.kb_state)?;
        state.serialize_field("registry_size", &self.registry_size)?;
        state.serialize_field("description", &self.description)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_snapshot() {
        let mut tt = TimeTravel::new("/tmp/nexus-test-snapshots");
        let graph = DiscoveryGraph::new();
        let id = tt.save_snapshot(&graph, "kb-data", 100, "test snapshot");
        assert_eq!(id, "snap-0001");
        assert_eq!(tt.count(), 1);
    }

    #[test]
    fn test_list_snapshots() {
        let mut tt = TimeTravel::new("/tmp/nexus-test-snapshots-list");
        let graph = DiscoveryGraph::new();
        tt.save_snapshot(&graph, "kb1", 10, "first");
        tt.save_snapshot(&graph, "kb2", 20, "second");

        let list = tt.list_snapshots();
        assert_eq!(list.len(), 2);
        // Most recent first
        assert_eq!(list[0].id, "snap-0002");
        assert_eq!(list[1].id, "snap-0001");
    }

    #[test]
    fn test_rollback() {
        let mut tt = TimeTravel::new("/tmp/nexus-test-snapshots-rollback");
        let mut graph = DiscoveryGraph::new();
        graph.add_node(crate::graph::node::Node {
            id: "test-node".to_string(),
            node_type: crate::graph::node::NodeType::Hypothesis,
            domain: "test".to_string(),
            project: "nexus".to_string(),
            summary: "test hypothesis".to_string(),
            confidence: 0.9,
            lenses_used: vec![],
            timestamp: "now".to_string(),
        });

        let id = tt.save_snapshot(&graph, "kb", 5, "with nodes");
        let restored = tt.rollback(&id).unwrap();
        assert_eq!(restored.nodes.len(), 1);
        assert_eq!(restored.nodes[0].id, "test-node");
    }

    #[test]
    fn test_branch() {
        let mut tt = TimeTravel::new("/tmp/nexus-test-snapshots-branch");
        let graph = DiscoveryGraph::new();
        let id = tt.save_snapshot(&graph, "kb", 5, "original");

        let branch_id = tt.branch(&id, "experiment-a").unwrap();
        assert!(branch_id.contains("branch-experiment-a"));
        assert_eq!(tt.count(), 2);
    }

    #[test]
    fn test_rollback_nonexistent() {
        let tt = TimeTravel::new("/tmp/nexus-test-snapshots-noexist");
        assert!(tt.rollback("nonexistent").is_none());
    }

    #[test]
    fn test_branch_nonexistent() {
        let mut tt = TimeTravel::new("/tmp/nexus-test-snapshots-noexist-branch");
        assert!(tt.branch("nonexistent", "test").is_none());
    }
}
