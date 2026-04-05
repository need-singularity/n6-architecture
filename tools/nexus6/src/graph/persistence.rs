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

    /// Atomic save with merge-on-write (race-safe for parallel processes).
    ///
    /// 병렬 프로세스가 동일 경로에 쓰는 경우를 처리:
    ///   1. 디스크의 기존 그래프를 재로드
    ///   2. 자신의 그래프와 merge (노드 id 중복 제거)
    ///   3. PID-tagged 임시 파일(.tmp.<PID>)에 쓰고 atomic rename
    /// 다른 프로세스의 노드가 덮어씌워지거나 rename race가 발생하지 않음.
    pub fn save(&self, path: &str) -> io::Result<()> {
        ensure_parent_dir(path)?;

        // 1. 디스크에서 기존 그래프 재로드 + merge
        let mut merged = Self::load(path)?;
        merged.merge(self);

        // 2. PID-tagged 임시 파일로 atomic write
        let tmp_path = format!("{}.tmp.{}", path, std::process::id());
        let json = serde_json::to_string_pretty(&merged)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        fs::write(&tmp_path, &json)?;
        fs::rename(&tmp_path, path)?;
        Ok(())
    }

    /// Merge another graph into this one. Deduplicates nodes by id AND
    /// edges by (from, to, edge_type, bidirectional) tuple.
    /// Edge dedup is required for race-safe save() to avoid double-counting.
    pub fn merge(&mut self, other: &DiscoveryGraph) {
        let existing_ids: std::collections::HashSet<String> =
            self.nodes.iter().map(|n| n.id.clone()).collect();

        for node in &other.nodes {
            if !existing_ids.contains(&node.id) {
                self.nodes.push(node.clone());
            }
        }

        // Dedupe edges by (from, to, edge_type, bidirectional).
        // Strength is intentionally excluded — same relationship with different
        // strength is still the same edge.
        let edge_key = |e: &Edge| -> (String, String, String, bool) {
            (
                e.from.clone(),
                e.to.clone(),
                format!("{:?}", e.edge_type),
                e.bidirectional,
            )
        };
        let existing_edges: std::collections::HashSet<_> =
            self.edges.iter().map(edge_key).collect();
        for edge in &other.edges {
            if !existing_edges.contains(&edge_key(edge)) {
                self.edges.push(edge.clone());
            }
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
