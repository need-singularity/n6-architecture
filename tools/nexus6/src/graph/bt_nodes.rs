//! Breakthrough Theorem (BT-1 ~ BT-127) graph nodes and cross-domain edges.
//!
//! Each BT is a graph node with id, title, domains, and star rating.
//! Edges are created between BTs that share at least one domain.

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

/// Compact representation of a Breakthrough Theorem for graph construction.
struct BtEntry {
    id: u32,
    title: &'static str,
    domains: &'static [&'static str],
    stars: u8, // 0 = conjecture, 1..3 = star rating
}

/// All 127 BTs with their domains and star ratings.
const BT_ENTRIES: &[BtEntry] = &[
    BtEntry { id: 1, title: "phi(6)=2 Universal Pairing", domains: &["SC", "Fusion", "Magnet", "QC", "Tokamak", "Chip", "Crypto"], stars: 2 },
    BtEntry { id: 2, title: "tau(6)=4 Bohm-BCS Bridge", domains: &["SC", "Fusion", "Tokamak", "Plasma"], stars: 2 },
    BtEntry { id: 3, title: "sigma(6)=12 Energy Scale Convergence", domains: &["SC", "Fusion", "Magnet", "Particle"], stars: 2 },
    BtEntry { id: 4, title: "MHD Divisor Theorem", domains: &["Fusion", "Tokamak", "Plasma", "Magnet"], stars: 1 },
    BtEntry { id: 5, title: "q=1 Perfect Number Definition", domains: &["Fusion", "Tokamak", "NumberTheory"], stars: 3 },
    BtEntry { id: 6, title: "Golay-Leech Unification", domains: &["QC", "Crypto", "Network", "Chip", "Math"], stars: 3 },
    BtEntry { id: 7, title: "Egyptian Fraction Power Theorem", domains: &["Chip", "PowerGrid", "Thermal", "AI", "Tokamak", "Energy"], stars: 2 },
    BtEntry { id: 8, title: "Pulse Rectifier Chain", domains: &["PowerGrid", "Tokamak", "Math", "QC", "SC"], stars: 2 },
    BtEntry { id: 9, title: "Bott Periodicity Bridge", domains: &["QC", "Crypto", "Chip", "Particle", "Math"], stars: 1 },
    BtEntry { id: 10, title: "Landauer-WHH Bridge", domains: &["Thermal", "SC", "InfoTheory", "Chip"], stars: 2 },
    BtEntry { id: 11, title: "Software-Physics Isomorphism", domains: &["Software", "PowerGrid", "Fusion", "SC"], stars: 1 },
    BtEntry { id: 12, title: "Hamming-OSI-ECC Triple Bridge", domains: &["Network", "Chip", "QC", "CodingTheory"], stars: 2 },
    BtEntry { id: 13, title: "sigma-mu Internet Infrastructure Duality", domains: &["Network", "Math", "CodingTheory", "Crypto"], stars: 3 },
    BtEntry { id: 14, title: "Carbon-Silicon Tetrahedral Bridge", domains: &["Chemistry", "Chip", "Biology", "Material", "SC", "Energy"], stars: 2 },
    BtEntry { id: 15, title: "Kissing Number Quadruple", domains: &["Math", "SC", "Material", "CodingTheory"], stars: 3 },
    BtEntry { id: 16, title: "Riemann Zeta Trident", domains: &["Math", "SC", "Particle", "InfoTheory"], stars: 3 },
    BtEntry { id: 17, title: "SM Fermion-Boson sigma-Balance", domains: &["Particle", "Math", "SC"], stars: 2 },
    BtEntry { id: 18, title: "Vacuum Energy Chain R(n)=1 -> Monster", domains: &["Particle", "Math", "StringTheory", "QC", "SC", "Cosmology"], stars: 0 },
    BtEntry { id: 19, title: "GUT Hierarchy = n=6 Arithmetic", domains: &["Particle", "Math", "StringTheory"], stars: 3 },
    BtEntry { id: 20, title: "Gauge Coupling Trinity", domains: &["Particle", "Math", "Cosmology", "SC"], stars: 3 },
    BtEntry { id: 21, title: "Neutrino Mixing Trident", domains: &["Particle", "Math", "Cosmology", "Fusion"], stars: 2 },
    BtEntry { id: 22, title: "Inflation from Perfect Numbers", domains: &["Cosmology", "Particle", "Math", "Fusion", "NumberTheory"], stars: 3 },
    BtEntry { id: 23, title: "CKM Quark Mixing Hierarchy", domains: &["Particle", "Math", "Cosmology", "QC"], stars: 3 },
    BtEntry { id: 24, title: "Koide Pole Residue", domains: &["Particle", "Math", "NumberTheory"], stars: 3 },
    BtEntry { id: 25, title: "Genetic Code Arithmetic", domains: &["Biology", "Particle", "Math", "Chemistry"], stars: 2 },
    BtEntry { id: 26, title: "Chinchilla Scaling Constants", domains: &["AI", "Math", "InfoTheory"], stars: 2 },
    BtEntry { id: 27, title: "Carbon-6 Energy Chain", domains: &["Chemistry", "Battery", "Biology", "Energy"], stars: 2 },
    BtEntry { id: 28, title: "Computing Architecture Ladder", domains: &["Chip", "AI", "Semiconductor"], stars: 3 },
    BtEntry { id: 29, title: "IEEE 519 Power Quality", domains: &["PowerGrid", "Chip", "Energy"], stars: 2 },
    BtEntry { id: 30, title: "SQ Solar Bridge", domains: &["Solar", "Energy", "Chip", "Thermal"], stars: 2 },
    BtEntry { id: 31, title: "MoE Top-k Vocabulary", domains: &["AI", "Math"], stars: 2 },
    BtEntry { id: 32, title: "Nuclear Fission Scaffold", domains: &["Nuclear", "Energy", "Chemistry"], stars: 1 },
    BtEntry { id: 33, title: "Transformer sigma=12 Atom", domains: &["AI", "Chip", "Math"], stars: 1 },
    BtEntry { id: 34, title: "RoPE Base & Decimal Bridge", domains: &["AI", "Math", "Chip"], stars: 2 },
    BtEntry { id: 35, title: "Battery Voltage Table", domains: &["Battery", "Chemistry", "Energy"], stars: 1 },
    BtEntry { id: 36, title: "Grand Energy-Info-HW-Physics Chain", domains: &["Solar", "Semiconductor", "InfoTheory", "Chip", "Particle"], stars: 3 },
    BtEntry { id: 37, title: "Semiconductor Pitch Ladder", domains: &["Semiconductor", "Chip", "Material"], stars: 2 },
    BtEntry { id: 38, title: "Hydrogen Energy Quadruplet", domains: &["Hydrogen", "Energy", "Chemistry", "Thermal"], stars: 2 },
    BtEntry { id: 39, title: "KV-Head Universality", domains: &["AI", "Chip"], stars: 2 },
    BtEntry { id: 40, title: "Computing Power Ecosystem", domains: &["Chip", "PowerGrid", "Automotive"], stars: 2 },
    BtEntry { id: 41, title: "QEC at J2", domains: &["QC", "Chip", "CodingTheory"], stars: 2 },
    BtEntry { id: 42, title: "Inference Scaling", domains: &["AI", "Math"], stars: 2 },
    BtEntry { id: 43, title: "Battery Cathode CN=6 Universality", domains: &["Battery", "Material", "Chemistry"], stars: 3 },
    BtEntry { id: 44, title: "Context Window Ladder", domains: &["AI", "Chip"], stars: 2 },
    BtEntry { id: 45, title: "FP8/FP16=phi=2 Universal", domains: &["AI", "Chip", "Semiconductor"], stars: 2 },
    BtEntry { id: 46, title: "ln(4/3) RLHF Family", domains: &["AI", "Math"], stars: 2 },
    BtEntry { id: 47, title: "Interconnect Gen Counts", domains: &["Chip", "Network", "Semiconductor"], stars: 1 },
    BtEntry { id: 48, title: "Display-Audio sigma*tau=48", domains: &["DisplayAudio", "Music", "AI", "Chip"], stars: 3 },
    BtEntry { id: 49, title: "Pure Math Kissing Chain", domains: &["Math", "NumberTheory", "CodingTheory", "Topology"], stars: 3 },
    BtEntry { id: 50, title: "Crystallography n=6 Symmetry", domains: &["Material", "Chemistry", "Math"], stars: 2 },
    BtEntry { id: 51, title: "Genetic Code Chain", domains: &["Biology", "Chemistry", "Particle", "Math"], stars: 3 },
    BtEntry { id: 52, title: "Cosmological Fine Tuning", domains: &["Cosmology", "Particle", "Math"], stars: 2 },
    BtEntry { id: 53, title: "Crypto BTC/ETH n=6", domains: &["Crypto", "Blockchain", "Math"], stars: 2 },
    BtEntry { id: 54, title: "AdamW Training Quintuplet", domains: &["AI", "Math", "Chip"], stars: 3 },
    BtEntry { id: 55, title: "GPU HBM Capacity Ladder", domains: &["Chip", "Semiconductor", "AI"], stars: 2 },
    BtEntry { id: 56, title: "Complete n=6 LLM Architecture", domains: &["AI", "Chip", "Math", "Semiconductor"], stars: 3 },
    BtEntry { id: 57, title: "Battery Cell Ladder", domains: &["Battery", "Automotive", "Energy"], stars: 2 },
    BtEntry { id: 58, title: "sigma-tau=8 Universal AI Constant", domains: &["AI", "Crypto", "Chip", "Math", "Biology", "InfoTheory"], stars: 3 },
    BtEntry { id: 59, title: "8-Layer AI Stack", domains: &["AI", "Chip", "Semiconductor", "Software"], stars: 3 },
    BtEntry { id: 60, title: "DC Power Chain", domains: &["PowerGrid", "Chip", "Energy", "Thermal"], stars: 2 },
    BtEntry { id: 61, title: "Diffusion n=6 Universality", domains: &["AI", "Math"], stars: 3 },
    BtEntry { id: 62, title: "Grid Frequency Pair", domains: &["PowerGrid", "Solar", "Energy"], stars: 2 },
    BtEntry { id: 63, title: "Solar Panel Cell Ladder", domains: &["Solar", "Chip", "Hydrogen", "Energy"], stars: 2 },
    BtEntry { id: 64, title: "Universal Regularization 0.1", domains: &["AI", "Fusion", "Plasma", "Math"], stars: 3 },
    BtEntry { id: 65, title: "Mamba SSM Complete n=6", domains: &["AI", "Math"], stars: 2 },
    BtEntry { id: 66, title: "Vision AI Complete n=6", domains: &["AI", "DisplayAudio", "Chip"], stars: 3 },
    BtEntry { id: 67, title: "MoE Activation Fraction Law", domains: &["AI", "InfoTheory", "Math"], stars: 3 },
    BtEntry { id: 68, title: "HVDC Voltage Ladder", domains: &["PowerGrid", "Energy", "Chip"], stars: 2 },
    BtEntry { id: 69, title: "Chiplet Architecture Convergence", domains: &["Chip", "Semiconductor", "AI"], stars: 3 },
    BtEntry { id: 70, title: "0.1 Convergence 8th Algorithm", domains: &["AI", "Math"], stars: 2 },
    BtEntry { id: 71, title: "NeRF/3DGS Complete n=6", domains: &["AI", "DisplayAudio", "Math"], stars: 2 },
    BtEntry { id: 72, title: "Neural Audio Codec n=6", domains: &["AI", "DisplayAudio", "Music"], stars: 2 },
    BtEntry { id: 73, title: "Tokenizer Vocabulary n=6 Law", domains: &["AI", "InfoTheory", "Math"], stars: 2 },
    BtEntry { id: 74, title: "95/5 Cross-Domain Resonance", domains: &["AI", "Fusion", "PowerGrid", "Chip", "Plasma"], stars: 3 },
    BtEntry { id: 75, title: "HBM Interface Exponent Ladder", domains: &["Chip", "Semiconductor"], stars: 2 },
    BtEntry { id: 76, title: "sigma*tau=48 Triple Attractor", domains: &["Chip", "Semiconductor", "DisplayAudio", "Energy"], stars: 2 },
    BtEntry { id: 77, title: "BitNet Quantization n=6", domains: &["AI", "Chip", "Math"], stars: 3 },
    BtEntry { id: 78, title: "KV Cache Compression n=6", domains: &["AI", "Chip", "Math"], stars: 3 },
    BtEntry { id: 79, title: "Speculative Decoding n=6", domains: &["AI", "Chip"], stars: 3 },
    BtEntry { id: 80, title: "Solid-State Electrolyte CN=6", domains: &["Battery", "Material", "Chemistry"], stars: 3 },
    BtEntry { id: 81, title: "Anode Capacity Ladder sigma-phi=10x", domains: &["Battery", "Material", "Chemistry"], stars: 2 },
    BtEntry { id: 82, title: "Battery Pack n=6 Map", domains: &["Battery", "Automotive", "Energy"], stars: 2 },
    BtEntry { id: 83, title: "Li-S Polysulfide n=6 Ladder", domains: &["Battery", "Chemistry", "Material"], stars: 2 },
    BtEntry { id: 84, title: "96/192 Energy-Computing-AI Triple", domains: &["Battery", "Chip", "AI", "Automotive", "Energy"], stars: 3 },
    BtEntry { id: 85, title: "Carbon Z=6 Material Universality", domains: &["Material", "Chemistry", "Biology", "Chip"], stars: 3 },
    BtEntry { id: 86, title: "Crystal CN=6 Octahedral Law", domains: &["Material", "Chemistry", "Math"], stars: 3 },
    BtEntry { id: 87, title: "Precision Fabrication 1/(sigma-phi) Ladder", domains: &["Material", "Semiconductor", "Chemistry"], stars: 2 },
    BtEntry { id: 88, title: "Hexagonal Emergence Universality", domains: &["Material", "Biology", "Chemistry", "Math"], stars: 2 },
    BtEntry { id: 89, title: "Photonic-Energy n=6 Bridge", domains: &["Chip", "Energy", "Semiconductor"], stars: 2 },
    BtEntry { id: 90, title: "SM = phi*K6 Kissing Theorem", domains: &["Chip", "Math", "Semiconductor"], stars: 3 },
    BtEntry { id: 91, title: "Z2 Topological ECC J2 Savings", domains: &["Chip", "QC", "CodingTheory"], stars: 2 },
    BtEntry { id: 92, title: "Bott Active Channel = sopfr", domains: &["Chip", "Math", "Topology"], stars: 3 },
    BtEntry { id: 93, title: "Carbon Z=6 Chip Material", domains: &["Chip", "Material", "Chemistry", "Semiconductor"], stars: 3 },
    BtEntry { id: 94, title: "CO2 Capture Energy Ratio", domains: &["Environment", "Energy", "Chemistry"], stars: 2 },
    BtEntry { id: 95, title: "Carbon Cycle 6-Step", domains: &["Environment", "Chemistry", "Energy"], stars: 2 },
    BtEntry { id: 96, title: "MOF CN=6 Metal Universality", domains: &["Environment", "Material", "Chemistry"], stars: 2 },
    BtEntry { id: 97, title: "Weinberg Angle sin2theta_W = 3/13", domains: &["Particle", "Fusion", "Cosmology"], stars: 2 },
    BtEntry { id: 98, title: "D-T Baryon = sopfr(6) = 5", domains: &["Fusion", "Particle", "NumberTheory"], stars: 3 },
    BtEntry { id: 99, title: "Tokamak q=1 = Egyptian Fraction", domains: &["Fusion", "Tokamak", "NumberTheory"], stars: 3 },
    BtEntry { id: 100, title: "CNO Catalyst Mass Ladder", domains: &["Fusion", "Particle", "Cosmology", "NumberTheory"], stars: 3 },
    BtEntry { id: 101, title: "Photosynthesis Quantum Yield", domains: &["Biology", "Chemistry", "Energy", "Particle"], stars: 3 },
    BtEntry { id: 102, title: "Magnetic Reconnection Rate 0.1", domains: &["Fusion", "Plasma", "Cosmology", "AI"], stars: 3 },
    BtEntry { id: 103, title: "Photosynthesis Complete n=6 Stoichiometry", domains: &["Biology", "Chemistry", "Energy"], stars: 3 },
    BtEntry { id: 104, title: "CO2 Molecule Complete n=6 Encoding", domains: &["Environment", "Chemistry", "Biology"], stars: 3 },
    BtEntry { id: 105, title: "SLE6 Critical Exponent Universality", domains: &["Math", "Particle", "Topology"], stars: 3 },
    BtEntry { id: 106, title: "S3 Algebraic Bootstrap", domains: &["Math", "Particle", "Combinatorics", "Physics"], stars: 2 },
    BtEntry { id: 107, title: "Ramanujan Tau Divisor Purity", domains: &["NumberTheory", "StringTheory", "Math"], stars: 2 },
    BtEntry { id: 108, title: "Music-Audio Consonance Universality", domains: &["Music", "DisplayAudio", "NumberTheory", "Math", "AI"], stars: 2 },
    BtEntry { id: 109, title: "Zeta-Bernoulli n=6 Trident", domains: &["Math", "NumberTheory", "Particle", "StringTheory"], stars: 2 },
    BtEntry { id: 110, title: "sigma-mu=11 Dimensional Stack", domains: &["Particle", "Network", "Crypto", "Fusion", "Chip"], stars: 1 },
    BtEntry { id: 111, title: "tau2/sigma=4/3 Solar-AI-Math Trident", domains: &["Solar", "AI", "Math", "Energy"], stars: 2 },
    BtEntry { id: 112, title: "phi2/n=2/3 Byzantine-Koide Resonance", domains: &["Particle", "Crypto", "Blockchain", "Math"], stars: 2 },
    BtEntry { id: 113, title: "SW Engineering Constant Stack", domains: &["Software", "Math", "Network"], stars: 3 },
    BtEntry { id: 114, title: "Crypto Parameter Ladder", domains: &["Crypto", "Math", "Software"], stars: 3 },
    BtEntry { id: 115, title: "OS-Network Layer Count", domains: &["Software", "Network", "Chip"], stars: 2 },
    BtEntry { id: 116, title: "ACID-BASE-CAP DB Trinity", domains: &["Software", "Math", "Network"], stars: 2 },
    BtEntry { id: 117, title: "Software-Physics Isomorphism (Extended)", domains: &["Software", "Particle", "PowerGrid", "Math", "SC", "Fusion"], stars: 3 },
    BtEntry { id: 118, title: "Kyoto 6 GHGs = n + Carbon Z=6", domains: &["Environment", "Chemistry", "Energy", "Biology"], stars: 3 },
    BtEntry { id: 119, title: "Earth 6 Spheres + Troposphere sigma=12km", domains: &["Environment", "Cosmology", "Biology"], stars: 3 },
    BtEntry { id: 120, title: "Water Treatment pH=6 + CN=6 Catalyst", domains: &["Environment", "Chemistry", "Material"], stars: 3 },
    BtEntry { id: 121, title: "6 Major Plastics + C6 Backbone", domains: &["Environment", "Chemistry", "Material", "Biology"], stars: 2 },
    BtEntry { id: 122, title: "Honeycomb-Snowflake-Coral n=6 Geometry", domains: &["Environment", "Math", "Biology", "Material"], stars: 3 },
    BtEntry { id: 123, title: "SE(3) dim=n=6 Robot Universality", domains: &["Robotics", "Chip", "Math", "Material"], stars: 3 },
    BtEntry { id: 124, title: "phi=2 Bilateral Symmetry + sigma=12 Joint", domains: &["Robotics", "Biology", "Chip"], stars: 2 },
    BtEntry { id: 125, title: "tau=4 Locomotion/Flight Minimum Stability", domains: &["Robotics", "Energy", "Chip"], stars: 2 },
    BtEntry { id: 126, title: "sopfr=5 Fingers + 2^sopfr=32 Grasp Space", domains: &["Robotics", "Biology", "DisplayAudio"], stars: 2 },
    BtEntry { id: 127, title: "3D Kissing sigma=12 + Hexacopter n=6", domains: &["Robotics", "Cosmology", "Material"], stars: 2 },
];

/// Create a Node from a BtEntry.
fn bt_to_node(bt: &BtEntry) -> Node {
    let domains_str = bt.domains.join(", ");
    let star_label = match bt.stars {
        0 => "CONJECTURE",
        1 => "One star",
        2 => "Two stars",
        _ => "Three stars",
    };
    Node {
        id: format!("BT-{}", bt.id),
        node_type: NodeType::Bt,
        domain: domains_str,
        project: "n6-architecture".to_string(),
        summary: format!("{} [{}]", bt.title, star_label),
        confidence: match bt.stars {
            0 => 0.3,
            1 => 0.5,
            2 => 0.7,
            _ => 0.9,
        },
        lenses_used: vec!["consciousness".into(), "causal".into(), "topology".into()],
        timestamp: "2026-04-03".to_string(),
    }
}

/// Populate a DiscoveryGraph with all 127 BT nodes and cross-domain edges.
///
/// Edges are created between any two BTs that share at least one domain.
/// Edge strength is proportional to the number of shared domains.
pub fn populate_bt_graph(graph: &mut DiscoveryGraph) {
    // Add all BT nodes
    for bt in BT_ENTRIES {
        graph.add_node(bt_to_node(bt));
    }

    // Build domain -> list of BT ids mapping
    let mut domain_to_bts: std::collections::HashMap<&str, Vec<u32>> =
        std::collections::HashMap::new();
    for bt in BT_ENTRIES {
        for &d in bt.domains {
            domain_to_bts.entry(d).or_default().push(bt.id);
        }
    }

    // Create edges between BTs sharing domains
    let mut seen_pairs: std::collections::HashSet<(u32, u32)> =
        std::collections::HashSet::new();

    for bt_a in BT_ENTRIES {
        for bt_b in BT_ENTRIES {
            if bt_a.id >= bt_b.id {
                continue;
            }
            let pair = (bt_a.id, bt_b.id);
            if seen_pairs.contains(&pair) {
                continue;
            }

            // Count shared domains
            let shared: usize = bt_a
                .domains
                .iter()
                .filter(|d| bt_b.domains.contains(d))
                .count();

            if shared > 0 {
                seen_pairs.insert(pair);
                let max_domains = bt_a.domains.len().max(bt_b.domains.len()) as f64;
                let strength = shared as f64 / max_domains;
                graph.add_edge(Edge {
                    from: format!("BT-{}", bt_a.id),
                    to: format!("BT-{}", bt_b.id),
                    edge_type: EdgeType::Merges,
                    strength,
                    bidirectional: true,
                });
            }
        }
    }
}

/// Return the count of unique domains across all BTs.
pub fn unique_domain_count() -> usize {
    let mut domains: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for bt in BT_ENTRIES {
        for &d in bt.domains {
            domains.insert(d);
        }
    }
    domains.len()
}

/// Return the total number of BT entries.
pub fn bt_count() -> usize {
    BT_ENTRIES.len()
}

/// Return BT ids that belong to a specific domain.
pub fn bts_in_domain(domain: &str) -> Vec<String> {
    BT_ENTRIES
        .iter()
        .filter(|bt| bt.domains.contains(&domain))
        .map(|bt| format!("BT-{}", bt.id))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_populate_bt_graph_node_count() {
        let mut graph = DiscoveryGraph::new();
        populate_bt_graph(&mut graph);
        assert_eq!(graph.nodes.len(), 127, "Should have exactly 127 BT nodes");
    }

    #[test]
    fn test_populate_bt_graph_has_edges() {
        let mut graph = DiscoveryGraph::new();
        populate_bt_graph(&mut graph);
        // BT-1 and BT-2 both share SC and Fusion domains, so there must be edges
        assert!(
            graph.edges.len() > 100,
            "Should have many cross-domain edges, got {}",
            graph.edges.len()
        );
        // Verify a known edge exists: BT-1 and BT-2 share SC, Fusion, Tokamak
        let has_bt1_bt2 = graph.edges.iter().any(|e| {
            (e.from == "BT-1" && e.to == "BT-2") || (e.from == "BT-2" && e.to == "BT-1")
        });
        assert!(has_bt1_bt2, "BT-1 and BT-2 should be connected (shared SC/Fusion/Tokamak)");
    }

    #[test]
    fn test_bt_count_and_domains() {
        assert_eq!(bt_count(), 127);
        let domain_count = unique_domain_count();
        assert!(
            domain_count >= 30,
            "Should cover 30+ unique domains, got {}",
            domain_count
        );
    }

    #[test]
    fn test_bts_in_domain() {
        let robotics = bts_in_domain("Robotics");
        assert_eq!(robotics.len(), 5, "Robotics should have BT-123~127");
        assert!(robotics.contains(&"BT-123".to_string()));
        assert!(robotics.contains(&"BT-127".to_string()));
    }

    #[test]
    fn test_edge_strength_range() {
        let mut graph = DiscoveryGraph::new();
        populate_bt_graph(&mut graph);
        for edge in &graph.edges {
            assert!(
                edge.strength > 0.0 && edge.strength <= 1.0,
                "Edge strength should be in (0, 1], got {}",
                edge.strength
            );
        }
    }
}
