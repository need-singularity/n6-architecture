use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::edge::Edge;
use super::node::Node;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClosedLoop {
    pub nodes: Vec<String>,
    pub strength: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hub {
    pub node_id: String,
    pub degree: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Convergence {
    pub target: String,
    pub sources: Vec<String>,
}

/// Build adjacency set (undirected for bidirectional edges, directed otherwise).
fn build_adjacency(nodes: &[Node], edges: &[Edge]) -> HashMap<String, HashSet<String>> {
    let node_ids: HashSet<String> = nodes.iter().map(|n| n.id.clone()).collect();
    let mut adj: HashMap<String, HashSet<String>> = HashMap::new();
    for id in &node_ids {
        adj.insert(id.clone(), HashSet::new());
    }
    for e in edges {
        if node_ids.contains(&e.from) && node_ids.contains(&e.to) {
            adj.entry(e.from.clone()).or_default().insert(e.to.clone());
            if e.bidirectional {
                adj.entry(e.to.clone()).or_default().insert(e.from.clone());
            }
        }
    }
    adj
}

/// Build edge strength lookup keyed by (from, to).
fn build_strength_map(edges: &[Edge]) -> HashMap<(String, String), f64> {
    let mut map = HashMap::new();
    for e in edges {
        map.insert((e.from.clone(), e.to.clone()), e.strength);
        if e.bidirectional {
            map.insert((e.to.clone(), e.from.clone()), e.strength);
        }
    }
    map
}

/// Find all triangles where A↔B↔C↔A (all three pairs connected).
pub fn find_closed_triangles(nodes: &[Node], edges: &[Edge]) -> Vec<ClosedLoop> {
    let adj = build_adjacency(nodes, edges);
    let strength_map = build_strength_map(edges);
    let ids: Vec<String> = nodes.iter().map(|n| n.id.clone()).collect();
    let mut triangles: Vec<ClosedLoop> = Vec::new();
    let mut seen: HashSet<Vec<String>> = HashSet::new();

    for i in 0..ids.len() {
        for j in (i + 1)..ids.len() {
            if !adj.get(&ids[i]).map_or(false, |s| s.contains(&ids[j])) {
                continue;
            }
            for k in (j + 1)..ids.len() {
                let a = &ids[i];
                let b = &ids[j];
                let c = &ids[k];

                let ab = adj.get(a).map_or(false, |s| s.contains(b));
                let ba = adj.get(b).map_or(false, |s| s.contains(a));
                let bc = adj.get(b).map_or(false, |s| s.contains(c));
                let cb = adj.get(c).map_or(false, |s| s.contains(b));
                let ac = adj.get(a).map_or(false, |s| s.contains(c));
                let ca = adj.get(c).map_or(false, |s| s.contains(a));

                // All three pairs must be mutually connected
                if (ab && ba) && (bc && cb) && (ac && ca) {
                    let mut key = vec![a.clone(), b.clone(), c.clone()];
                    key.sort();
                    if seen.insert(key.clone()) {
                        let s_ab = strength_map
                            .get(&(a.clone(), b.clone()))
                            .copied()
                            .unwrap_or(0.0);
                        let s_bc = strength_map
                            .get(&(b.clone(), c.clone()))
                            .copied()
                            .unwrap_or(0.0);
                        let s_ac = strength_map
                            .get(&(a.clone(), c.clone()))
                            .copied()
                            .unwrap_or(0.0);
                        let avg_strength = (s_ab + s_bc + s_ac) / 3.0;
                        triangles.push(ClosedLoop {
                            nodes: key,
                            strength: avg_strength,
                        });
                    }
                }
            }
        }
    }
    triangles
}

/// Find nodes with degree >= min_degree.
pub fn find_hubs(edges: &[Edge], min_degree: usize) -> Vec<Hub> {
    let mut degree: HashMap<String, usize> = HashMap::new();
    for e in edges {
        *degree.entry(e.from.clone()).or_insert(0) += 1;
        *degree.entry(e.to.clone()).or_insert(0) += 1;
        // Bidirectional edges still count once per endpoint
    }
    let mut hubs: Vec<Hub> = degree
        .into_iter()
        .filter(|(_, d)| *d >= min_degree)
        .map(|(node_id, d)| Hub {
            node_id,
            degree: d,
        })
        .collect();
    hubs.sort_by(|a, b| b.degree.cmp(&a.degree));
    hubs
}

/// Find DAG merge points: nodes with 2+ incoming edges.
pub fn find_convergences(edges: &[Edge]) -> Vec<Convergence> {
    let mut incoming: HashMap<String, Vec<String>> = HashMap::new();
    for e in edges {
        incoming
            .entry(e.to.clone())
            .or_default()
            .push(e.from.clone());
        if e.bidirectional {
            incoming
                .entry(e.from.clone())
                .or_default()
                .push(e.to.clone());
        }
    }
    let mut convs: Vec<Convergence> = incoming
        .into_iter()
        .filter(|(_, sources)| sources.len() >= 2)
        .map(|(target, sources)| Convergence { target, sources })
        .collect();
    convs.sort_by(|a, b| b.sources.len().cmp(&a.sources.len()));
    convs
}
