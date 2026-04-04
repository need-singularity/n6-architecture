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
pub mod bt185_discovery_nodes;
pub mod math_sw_extended_nodes;
pub mod cross_cluster_nodes;
pub mod bt118_deep_nodes;
pub mod advanced_cross_nodes;
pub mod bt118_bridge_nodes;
pub mod cognitive_temporal_social_nodes;
pub mod transport_medical_nodes;
pub mod grand_cross_nodes;

pub use node::{Node, NodeType};
pub use edge::{Edge, EdgeType};
pub use structure::{ClosedLoop, Hub, Convergence};
pub use persistence::DiscoveryGraph;
