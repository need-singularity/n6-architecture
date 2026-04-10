use std::collections::HashMap;

use nexus::history::recorder::{append_record, load_records, ScanRecord};
use nexus::history::stats::{compute_domain_stats, compute_lens_affinity};
use nexus::history::recommend::recommend_lenses;

fn make_record(id: &str, domain: &str, lenses: &[&str], discoveries: &[&str], consensus: usize) -> ScanRecord {
    ScanRecord {
        id: id.to_string(),
        timestamp: "2026-04-03T00:00:00Z".to_string(),
        domain: domain.to_string(),
        lenses_used: lenses.iter().map(|s| s.to_string()).collect(),
        discoveries: discoveries.iter().map(|s| s.to_string()).collect(),
        consensus_level: consensus,
    }
}

fn tmpdir_with_prefix(prefix: &str) -> String {
    let dir = std::env::temp_dir().join(format!("nexus_test_{}_{}", prefix, std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir.to_string_lossy().to_string()
}

#[test]
fn test_append_and_load_records() {
    let dir = tmpdir_with_prefix("append_load");

    let r1 = make_record("r1", "physics", &["consciousness", "topology"], &["disc-A"], 3);
    let r2 = make_record("r2", "physics", &["wave", "quantum"], &[], 1);
    let r3 = make_record("r3", "physics", &["gravity"], &["disc-B", "disc-C"], 5);

    append_record(&dir, "physics", &r1).unwrap();
    append_record(&dir, "physics", &r2).unwrap();
    append_record(&dir, "physics", &r3).unwrap();

    let loaded = load_records(&dir, "physics");
    assert_eq!(loaded.len(), 3);
    assert_eq!(loaded[0].id, "r1");
    assert_eq!(loaded[1].id, "r2");
    assert_eq!(loaded[2].id, "r3");
    assert_eq!(loaded[0].discoveries, vec!["disc-A"]);
    assert_eq!(loaded[2].consensus_level, 5);

    // Loading a non-existent domain returns empty
    let empty = load_records(&dir, "nonexistent");
    assert!(empty.is_empty());

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_compute_domain_stats() {
    // 5 records with known patterns:
    // - "consciousness" used 4 times, contributed 3 times (in discovery scans)
    // - "topology" used 3 times, contributed 2 times
    // - "wave" used 2 times, contributed 1 time
    // - "quantum" used 1 time, contributed 0 times (scan had no discoveries)
    // - "gravity" used 1 time, contributed 1 time
    let records = vec![
        make_record("1", "d", &["consciousness", "topology"], &["A"], 3),
        make_record("2", "d", &["consciousness", "wave"], &["B"], 2),
        make_record("3", "d", &["consciousness", "topology", "gravity"], &["C"], 5),
        make_record("4", "d", &["consciousness", "quantum"], &[], 0),
        make_record("5", "d", &["topology", "wave"], &["D"], 4),
    ];

    let stats = compute_domain_stats(&records);
    assert_eq!(stats.total_scans, 5);
    assert_eq!(stats.total_discoveries, 4); // A + B + C + D

    let c = stats.lens_stats.get("consciousness").unwrap();
    assert_eq!(c.used, 4);
    assert_eq!(c.contributed, 3);
    assert!((c.hit_rate - 0.75).abs() < 0.001);

    let t = stats.lens_stats.get("topology").unwrap();
    assert_eq!(t.used, 3);
    assert_eq!(t.contributed, 3); // records 1,3,5 all have discoveries
    assert!((t.hit_rate - 1.0).abs() < 0.001);

    let w = stats.lens_stats.get("wave").unwrap();
    assert_eq!(w.used, 2);
    assert_eq!(w.contributed, 2); // records 2,5 both have discoveries
    assert!((w.hit_rate - 1.0).abs() < 0.001);

    let q = stats.lens_stats.get("quantum").unwrap();
    assert_eq!(q.used, 1);
    assert_eq!(q.contributed, 0);
    assert!((q.hit_rate - 0.0).abs() < 0.001);

    let g = stats.lens_stats.get("gravity").unwrap();
    assert_eq!(g.used, 1);
    assert_eq!(g.contributed, 1);
    assert!((g.hit_rate - 1.0).abs() < 0.001);
}

#[test]
fn test_lens_affinity() {
    // 3 discovery records, 1 non-discovery
    let records = vec![
        make_record("1", "d", &["consciousness", "topology", "wave"], &["A"], 3),
        make_record("2", "d", &["consciousness", "topology"], &["B"], 2),
        make_record("3", "d", &["wave", "gravity"], &["C"], 1),
        make_record("4", "d", &["quantum"], &[], 0), // no discoveries, ignored
    ];

    let affinity = compute_lens_affinity(&records);
    // 3 discovery records total
    // (consciousness, topology): appears in records 1 and 2 => 2/3
    let ct = affinity.get(&("consciousness".to_string(), "topology".to_string())).unwrap();
    assert!((ct - 2.0 / 3.0).abs() < 0.001);

    // (consciousness, wave): appears in record 1 => 1/3
    let cw = affinity.get(&("consciousness".to_string(), "wave".to_string())).unwrap();
    assert!((cw - 1.0 / 3.0).abs() < 0.001);

    // (gravity, wave): appears in record 3 => 1/3
    let gw = affinity.get(&("gravity".to_string(), "wave".to_string())).unwrap();
    assert!((gw - 1.0 / 3.0).abs() < 0.001);

    // (topology, wave): appears in record 1 => 1/3
    let tw = affinity.get(&("topology".to_string(), "wave".to_string())).unwrap();
    assert!((tw - 1.0 / 3.0).abs() < 0.001);
}

#[test]
fn test_recommend_with_history() {
    // Domain "physics" has stats with known hit rates
    let records = vec![
        make_record("1", "physics", &["consciousness", "topology"], &["A"], 3),
        make_record("2", "physics", &["consciousness", "wave"], &["B"], 2),
        make_record("3", "physics", &["consciousness", "topology", "gravity"], &["C"], 5),
        make_record("4", "physics", &["quantum"], &[], 0),
    ];

    let stats = compute_domain_stats(&records);
    let mut all_stats = HashMap::new();
    all_stats.insert("physics".to_string(), stats);

    let all_lenses: Vec<String> = vec![
        "consciousness", "topology", "wave", "gravity", "quantum",
        "thermo", "evolution", "info", "em", "boundary",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect();

    let rec = recommend_lenses("physics", &all_stats, &all_lenses, 0.15);

    // Should include high hit_rate lenses first
    // consciousness: 3/3=1.0, topology: 2/2=1.0, wave: 1/1=1.0, gravity: 1/1=1.0
    // quantum: 0/1=0.0 (below 0.05 threshold, excluded from base)
    assert!(rec.lenses.contains(&"consciousness".to_string()));
    assert!(rec.lenses.contains(&"topology".to_string()));
    assert!(rec.lenses.contains(&"wave".to_string()));
    assert!(rec.lenses.contains(&"gravity".to_string()));
    assert!(rec.lenses.len() >= 4);
    assert!(rec.lenses.len() <= 40);
    assert!(rec.reason.contains("physics"));
}

#[test]
fn test_recommend_cold_start() {
    // No stats at all
    let all_stats: HashMap<String, nexus::history::DomainStats> = HashMap::new();
    let all_lenses: Vec<String> = vec![
        "consciousness", "topology", "void", "thermo", "evolution",
        "network", "boundary", "triangle", "wave", "quantum",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect();

    let rec = recommend_lenses("unknown_domain", &all_stats, &all_lenses, 0.15);

    // Should return default 8 cold start lenses + serendipity
    assert!(rec.lenses.contains(&"consciousness".to_string()));
    assert!(rec.lenses.contains(&"topology".to_string()));
    assert!(rec.lenses.contains(&"void".to_string()));
    assert!(rec.lenses.contains(&"thermo".to_string()));
    assert!(rec.lenses.contains(&"evolution".to_string()));
    assert!(rec.lenses.contains(&"network".to_string()));
    assert!(rec.lenses.contains(&"boundary".to_string()));
    assert!(rec.lenses.contains(&"triangle".to_string()));
    assert!(rec.lenses.len() >= 8); // 8 default
    assert!(rec.reason.contains("cold start"));
}

#[test]
fn test_recommend_serendipity() {
    // Domain with stats that only has 2 effective lenses
    let records = vec![
        make_record("1", "bio", &["consciousness"], &["A"], 3),
        make_record("2", "bio", &["consciousness"], &["B"], 2),
        make_record("3", "bio", &["topology"], &[], 0),
    ];
    let stats = compute_domain_stats(&records);
    let mut all_stats = HashMap::new();
    all_stats.insert("bio".to_string(), stats);

    let all_lenses: Vec<String> = vec![
        "consciousness", "topology", "wave", "gravity", "quantum",
        "thermo", "evolution", "info", "em", "boundary",
        "network", "memory", "recursion", "scale",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect();

    let rec = recommend_lenses("bio", &all_stats, &all_lenses, 0.15);

    // consciousness: hit_rate=1.0 (>0.05, included)
    // topology: hit_rate=0.0 (<0.05, excluded from base)
    // So base = [consciousness], then serendipity adds extra from unused
    assert!(rec.lenses.contains(&"consciousness".to_string()));
    // Total should be more than just the base due to serendipity
    assert!(rec.lenses.len() >= 4); // minimum enforced
    // There should be lenses beyond "consciousness" (serendipity or min-pad)
    assert!(rec.lenses.len() > 1);
}
