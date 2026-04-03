//! Discovery graph with BT nodes, cross-domain edges, and hubs.
pub mod node;
pub mod edge;
pub mod structure;
pub mod persistence;
pub mod bt_nodes;
pub mod expanded_nodes;
pub mod discovery_nodes;
pub mod extended_discovery_nodes;
pub mod recent_discoveries;
pub mod recent_extended_nodes;

pub use node::{Node, NodeType};
pub use edge::{Edge, EdgeType};
pub use structure::{ClosedLoop, Hub, Convergence};
pub use persistence::DiscoveryGraph;
