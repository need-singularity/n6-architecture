//! Discovery graph with BT nodes, cross-domain edges, and hubs.
pub mod node;
pub mod edge;
pub mod structure;
pub mod persistence;

pub use node::{Node, NodeType};
pub use edge::{Edge, EdgeType};
pub use structure::{ClosedLoop, Hub, Convergence};
pub use persistence::DiscoveryGraph;
