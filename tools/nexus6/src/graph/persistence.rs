use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

use super::edge::Edge;
use super::node::Node;
use super::structure::{self, ClosedLoop, Convergence, Hub};

/// Default persistence path for the discovery graph: ~/.nexus6/discovery_graph.json
pub fn default_graph_path() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    format!("{}/.nexus6/discovery_graph.json", home)
}

/// Ensure the directory for the graph file exists.
fn ensure_parent_dir(path: &str) -> io::Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DiscoveryGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl DiscoveryGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn closed_loops(&self) -> Vec<ClosedLoop> {
        structure::find_closed_triangles(&self.nodes, &self.edges)
    }

    pub fn hubs(&self, min_degree: usize) -> Vec<Hub> {
        structure::find_hubs(&self.edges, min_degree)
    }

    pub fn convergences(&self) -> Vec<Convergence> {
        structure::find_convergences(&self.edges)
    }

    /// Atomic save: write to .tmp then rename to prevent corruption.
    pub fn save(&self, path: &str) -> io::Result<()> {
        ensure_parent_dir(path)?;
        let tmp_path = format!("{}.tmp", path);
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        fs::write(&tmp_path, &json)?;
        fs::rename(&tmp_path, path)?;
        Ok(())
    }

    /// Merge another graph into this one. Deduplicates nodes by id.
    pub fn merge(&mut self, other: &DiscoveryGraph) {
        let existing_ids: std::collections::HashSet<String> =
            self.nodes.iter().map(|n| n.id.clone()).collect();

        for node in &other.nodes {
            if !existing_ids.contains(&node.id) {
                self.nodes.push(node.clone());
            }
        }

        // Add all edges (edge dedup is less critical; duplicates are tolerable)
        for edge in &other.edges {
            self.edges.push(edge.clone());
        }
    }

    /// Count of nodes by type.
    pub fn node_type_counts(&self) -> std::collections::HashMap<String, usize> {
        let mut counts = std::collections::HashMap::new();
        for node in &self.nodes {
            let key = format!("{:?}", node.node_type);
            *counts.entry(key).or_insert(0) += 1;
        }
        counts
    }

    /// Load from file, or return empty graph if file doesn't exist.
    pub fn load(path: &str) -> io::Result<Self> {
        if !Path::new(path).exists() {
            return Ok(Self::new());
        }
        let data = fs::read_to_string(path)?;
        let graph: DiscoveryGraph =
            serde_json::from_str(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(graph)
    }
}
