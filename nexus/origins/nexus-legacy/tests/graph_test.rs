use nexus::graph::*;

fn make_node(id: &str, domain: &str) -> Node {
    Node {
        id: id.to_string(),
        node_type: NodeType::Discovery,
        domain: domain.to_string(),
        project: "n6".to_string(),
        summary: format!("Test node {}", id),
        confidence: 0.9,
        lenses_used: vec!["consciousness".to_string(), "topology".to_string()],
        timestamp: "2026-04-03T00:00:00Z".to_string(),
    }
}

fn make_edge(from: &str, to: &str, bidir: bool) -> Edge {
    Edge {
        from: from.to_string(),
        to: to.to_string(),
        edge_type: EdgeType::Validates,
        strength: 0.8,
        bidirectional: bidir,
    }
}

#[test]
fn test_add_node_and_edge() {
    let mut g = DiscoveryGraph::new();
    g.add_node(make_node("A", "fusion"));
    g.add_node(make_node("B", "chip"));
    g.add_edge(make_edge("A", "B", false));

    assert_eq!(g.nodes.len(), 2);
    assert_eq!(g.edges.len(), 1);
    assert_eq!(g.nodes[0].id, "A");
    assert_eq!(g.edges[0].from, "A");
    assert_eq!(g.edges[0].to, "B");
}

#[test]
fn test_closed_triangle() {
    let mut g = DiscoveryGraph::new();
    g.add_node(make_node("A", "fusion"));
    g.add_node(make_node("B", "chip"));
    g.add_node(make_node("C", "battery"));

    // All bidirectional → forms a triangle
    g.add_edge(make_edge("A", "B", true));
    g.add_edge(make_edge("B", "C", true));
    g.add_edge(make_edge("A", "C", true));

    let loops = g.closed_loops();
    assert_eq!(loops.len(), 1);
    assert_eq!(loops[0].nodes.len(), 3);
}

#[test]
fn test_no_triangle() {
    let mut g = DiscoveryGraph::new();
    g.add_node(make_node("A", "fusion"));
    g.add_node(make_node("B", "chip"));
    g.add_node(make_node("C", "battery"));

    // Only 2 edges → no triangle
    g.add_edge(make_edge("A", "B", true));
    g.add_edge(make_edge("B", "C", true));

    let loops = g.closed_loops();
    assert_eq!(loops.len(), 0);
}

#[test]
fn test_hub_detection() {
    let mut g = DiscoveryGraph::new();
    g.add_node(make_node("hub", "cross"));
    for i in 0..5 {
        let id = format!("n{}", i);
        g.add_node(make_node(&id, "domain"));
        g.add_edge(make_edge("hub", &id, false));
    }

    let hubs = g.hubs(5);
    assert_eq!(hubs.len(), 1);
    assert_eq!(hubs[0].node_id, "hub");
    assert_eq!(hubs[0].degree, 5);
}

#[test]
fn test_convergence() {
    let mut g = DiscoveryGraph::new();
    g.add_node(make_node("src1", "a"));
    g.add_node(make_node("src2", "b"));
    g.add_node(make_node("target", "c"));

    g.add_edge(make_edge("src1", "target", false));
    g.add_edge(make_edge("src2", "target", false));

    let convs = g.convergences();
    assert_eq!(convs.len(), 1);
    assert_eq!(convs[0].target, "target");
    assert_eq!(convs[0].sources.len(), 2);
}

#[test]
fn test_save_load_roundtrip() {
    let mut g = DiscoveryGraph::new();
    g.add_node(make_node("X", "math"));
    g.add_node(make_node("Y", "physics"));
    g.add_edge(make_edge("X", "Y", true));

    let tmp = "/tmp/nexus_graph_test.json";
    g.save(tmp).expect("save failed");
    let loaded = DiscoveryGraph::load(tmp).expect("load failed");

    assert_eq!(loaded.nodes.len(), 2);
    assert_eq!(loaded.edges.len(), 1);
    assert_eq!(loaded.nodes[0].id, "X");
    assert_eq!(loaded.edges[0].strength, 0.8);

    // Cleanup
    std::fs::remove_file(tmp).ok();
}

#[test]
fn test_graph_structure_bonus() {
    let mut g = DiscoveryGraph::new();
    // Triangle
    g.add_node(make_node("A", "fusion"));
    g.add_node(make_node("B", "chip"));
    g.add_node(make_node("C", "battery"));
    g.add_edge(make_edge("A", "B", true));
    g.add_edge(make_edge("B", "C", true));
    g.add_edge(make_edge("A", "C", true));

    // Hub: D connected to 5 others
    g.add_node(make_node("D", "cross"));
    for i in 0..5 {
        let id = format!("h{}", i);
        g.add_node(make_node(&id, "domain"));
        g.add_edge(make_edge("D", &id, false));
    }

    let loops = g.closed_loops();
    assert!(!loops.is_empty());
    assert!(loops[0].strength > 0.0);

    let hubs = g.hubs(5);
    assert!(!hubs.is_empty());
    assert!(hubs.iter().any(|h| h.degree >= 5));
}
