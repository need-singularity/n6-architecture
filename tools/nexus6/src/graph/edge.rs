use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum EdgeType {
    Derives,
    Validates,
    Contradicts,
    Merges,
    Triggers,
    Refutes,
    Contains,
    Uses,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub edge_type: EdgeType,
    pub strength: f64,
    pub bidirectional: bool,
}
