use std::collections::HashMap;

use nexus6::telescope::anima_lenses::anima_consciousness_lens_entries;
use nexus6::telescope::consensus::{weighted_consensus, ConsensusLevel};
use nexus6::telescope::core_lenses::core_lens_entries;
use nexus6::telescope::cross_lenses::cross_project_lens_entries;
use nexus6::telescope::n6_lenses::n6_industry_lens_entries;
use nexus6::telescope::tecs_lenses::tecs_math_lens_entries;
use nexus6::telescope::sedi_lenses::sedi_signal_lens_entries;
use nexus6::telescope::accel_lenses_a::accel_ml_lens_entries;
use nexus6::telescope::accel_lenses_b::accel_physics_neuro_lens_entries;
use nexus6::telescope::accel_lenses_c::accel_engineering_lens_entries;
use nexus6::telescope::accel_lenses_d::accel_humanities_lens_entries;
use nexus6::telescope::domain_combos::default_combos;
use nexus6::telescope::lens_trait::{Lens, LensResult};
use nexus6::telescope::lenses::{BarrierLens, VoidLens};
use nexus6::telescope::registry::{LensCategory, LensEntry, LensRegistry};
use nexus6::telescope::shared_data::SharedData;
use nexus6::telescope::tier::TieredScanner;
use nexus6::telescope::Telescope;

/// Helper: build two well-separated 2D clusters.
/// Cluster A around (0,0), Cluster B around (10,10).
fn two_clusters() -> (Vec<f64>, usize, usize) {
    let mut data = Vec::new();
    // Cluster A: 10 points near origin
    for i in 0..10 {
        data.push(0.0 + (i as f64) * 0.1);
        data.push(0.0 + (i as f64) * 0.05);
    }
    // Cluster B: 10 points near (10, 10)
    for i in 0..10 {
        data.push(10.0 + (i as f64) * 0.1);
        data.push(10.0 + (i as f64) * 0.05);
    }
    (data, 20, 2)
}

/// Helper: build two clusters with a clear gap and a point in the middle.
fn two_clusters_with_gap() -> (Vec<f64>, usize, usize) {
    let mut data = Vec::new();
    // Cluster A: 8 points around (0, 0)
    for i in 0..8 {
        data.push(0.0 + (i as f64) * 0.1);
        data.push(0.0 + (i as f64) * 0.1);
    }
    // Gap point at (5, 5)
    data.push(5.0);
    data.push(5.0);
    // Cluster B: 8 points around (10, 10)
    for i in 0..8 {
        data.push(10.0 + (i as f64) * 0.1);
        data.push(10.0 + (i as f64) * 0.1);
    }
    (data, 17, 2)
}

// ──────────────────────────────────────────────
// Test 1: SharedData distance computation
// ──────────────────────────────────────────────
#[test]
fn test_shared_data_distances() {
    // 3 points in 2D: (0,0), (3,0), (0,4)
    let data = vec![0.0, 0.0, 3.0, 0.0, 0.0, 4.0];
    let shared = SharedData::compute(&data, 3, 2);

    assert_eq!(shared.n, 3);
    assert_eq!(shared.d, 2);

    // dist(0,1) = 3.0
    let d01 = shared.dist(0, 1);
    assert!((d01 - 3.0).abs() < 1e-10, "dist(0,1) = {}, expected 3.0", d01);

    // dist(0,2) = 4.0
    let d02 = shared.dist(0, 2);
    assert!((d02 - 4.0).abs() < 1e-10, "dist(0,2) = {}, expected 4.0", d02);

    // dist(1,2) = 5.0 (3-4-5 triangle)
    let d12 = shared.dist(1, 2);
    assert!((d12 - 5.0).abs() < 1e-10, "dist(1,2) = {}, expected 5.0", d12);

    // Symmetry
    assert!((shared.dist(1, 0) - d01).abs() < 1e-10);
}

// ──────────────────────────────────────────────
// Test 2: VoidLens finds gap between clusters
// ──────────────────────────────────────────────
#[test]
fn test_void_lens_finds_gap() {
    let (data, n, d) = two_clusters_with_gap();
    let shared = SharedData::compute(&data, n, d);

    let lens = VoidLens;
    let result = lens.scan(&data, n, d, &shared);

    // Should find void centers
    let centers = result.get("void_centers").expect("void_centers key missing");
    let scores = result.get("void_scores").expect("void_scores key missing");

    assert!(
        !centers.is_empty(),
        "VoidLens should find at least one void center"
    );
    assert_eq!(centers.len(), scores.len());

    // The gap point (index 8, at (5,5)) should be identified
    // It's a low-density point surrounded by two high-density clusters
    let has_gap_point = centers.iter().any(|&c| (c - 8.0).abs() < 0.5);
    assert!(
        has_gap_point,
        "VoidLens should identify the gap point (index 8) as a void center, found: {:?}",
        centers
    );
}

// ──────────────────────────────────────────────
// Test 3: BarrierLens finds barrier > 1.0
// ──────────────────────────────────────────────
#[test]
fn test_barrier_lens() {
    let (data, n, d) = two_clusters();
    let shared = SharedData::compute(&data, n, d);

    let lens = BarrierLens;
    let result = lens.scan(&data, n, d, &shared);

    let heights = result
        .get("barrier_heights")
        .expect("barrier_heights key missing");

    assert!(
        !heights.is_empty(),
        "BarrierLens should find at least one barrier"
    );

    // Two well-separated clusters should have barrier >> 1.0
    let max_barrier = heights
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    assert!(
        max_barrier > 1.0,
        "Barrier between well-separated clusters should be > 1.0, got {}",
        max_barrier
    );
}

// ──────────────────────────────────────────────
// Test 4: TieredScanner early exit (no signal)
// ──────────────────────────────────────────────

/// A lens that always returns empty results (no signal).
struct SilentLens;

impl Lens for SilentLens {
    fn name(&self) -> &str { "SilentLens" }
    fn category(&self) -> &str { "T0" }
    fn scan(&self, _data: &[f64], _n: usize, _d: usize, _shared: &SharedData) -> LensResult {
        HashMap::new()
    }
}

/// A lens that returns signal — used for T1 to verify it's NOT reached.
struct LoudLens;

impl Lens for LoudLens {
    fn name(&self) -> &str { "LoudLens" }
    fn category(&self) -> &str { "T1" }
    fn scan(&self, _data: &[f64], _n: usize, _d: usize, _shared: &SharedData) -> LensResult {
        let mut r = HashMap::new();
        r.insert("signal".to_string(), vec![1.0, 2.0, 3.0]);
        r
    }
}

#[test]
fn test_tiered_scanner_early_exit() {
    let mut scanner = TieredScanner::new();
    scanner.add_tier("T0", vec![Box::new(SilentLens)]);
    scanner.add_tier("T1", vec![Box::new(LoudLens)]);

    let data = vec![1.0, 2.0, 3.0, 4.0];
    let results = scanner.scan(&data, 2, 2);

    // T0 had no signal, so T1 should NOT have run
    assert!(
        !results.contains_key("T1:LoudLens"),
        "T1 should not run when T0 has no signal"
    );
    // T0 lens should be present (with empty result)
    assert!(
        results.contains_key("T0:SilentLens"),
        "T0 lens result should be recorded"
    );
}

// ──────────────────────────────────────────────
// Test 5: Consensus — 3 lenses agree = Candidate
// ──────────────────────────────────────────────
#[test]
fn test_consensus_candidate() {
    let mut results: HashMap<String, LensResult> = HashMap::new();

    // Three lenses all detect "anomaly_region"
    for name in &["LensA", "LensB", "LensC"] {
        let mut lr = HashMap::new();
        lr.insert("anomaly_region".to_string(), vec![1.0, 2.0]);
        results.insert(name.to_string(), lr);
    }

    let hit_rates: HashMap<String, f64> = [
        ("LensA".to_string(), 0.8),
        ("LensB".to_string(), 0.7),
        ("LensC".to_string(), 0.9),
    ]
    .into_iter()
    .collect();

    let consensus = weighted_consensus(&results, &hit_rates);

    assert_eq!(consensus.len(), 1);
    assert_eq!(consensus[0].pattern_id, "anomaly_region");
    assert_eq!(consensus[0].agreeing_lenses.len(), 3);
    assert_eq!(consensus[0].level, ConsensusLevel::Candidate);
    assert!((consensus[0].weighted_score - 2.4).abs() < 1e-10); // 0.8+0.7+0.9
}

// ──────────────────────────────────────────────
// Test 6: Telescope scan_all — full pipeline
// ──────────────────────────────────────────────
#[test]
fn test_telescope_scan_all() {
    let (data, n, d) = two_clusters();
    let telescope = Telescope::new();

    let results = telescope.scan_all(&data, n, d);

    // Should have results from both lenses
    assert!(results.contains_key("VoidLens"), "Missing VoidLens results");
    assert!(
        results.contains_key("BarrierLens"),
        "Missing BarrierLens results"
    );
    assert_eq!(telescope.lens_count(), 2);
}

// ──────────────────────────────────────────────
// Test 7: Lens isolation — panic doesn't crash
// ──────────────────────────────────────────────

/// A lens that panics on purpose.
struct PanicLens;

impl Lens for PanicLens {
    fn name(&self) -> &str { "PanicLens" }
    fn category(&self) -> &str { "T0" }
    fn scan(&self, _data: &[f64], _n: usize, _d: usize, _shared: &SharedData) -> LensResult {
        panic!("intentional panic for isolation test");
    }
}

#[test]
fn test_lens_isolation() {
    let data = vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 5.0, 5.0, 6.0, 5.0, 5.0, 6.0, 6.0, 6.0];
    let n = 8;
    let d = 2;
    let shared = SharedData::compute(&data, n, d);

    // Run PanicLens — should not crash the process
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let panic_lens = PanicLens;
        panic_lens.scan(&data, n, d, &shared)
    }));
    assert!(result.is_err(), "PanicLens should panic");

    // But VoidLens should still work fine
    let void_lens = VoidLens;
    let void_result = void_lens.scan(&data, n, d, &shared);
    assert!(
        void_result.contains_key("void_centers"),
        "VoidLens should work despite PanicLens failing"
    );

    // And the TieredScanner should handle it gracefully
    let mut scanner = TieredScanner::new();
    scanner.add_tier("T0", vec![
        Box::new(PanicLens),
        Box::new(VoidLens),
    ]);

    let results = scanner.scan(&data, n, d);
    assert!(results.contains_key("T0:VoidLens"), "VoidLens should succeed in tiered scan");
    assert!(results.contains_key("T0:PanicLens"), "PanicLens should have empty result entry");
    let panic_result = &results["T0:PanicLens"];
    assert!(panic_result.is_empty(), "PanicLens result should be empty");
}

// ──────────────────────────────────────────────
// Test 8: Registry — 22 Core lenses registered
// ──────────────────────────────────────────────
#[test]
fn test_registry_core_count() {
    let reg = LensRegistry::new();
    let cores = reg.by_category(LensCategory::Core);
    assert_eq!(cores.len(), 22, "Registry must contain exactly 22 Core lenses");
    assert_eq!(reg.len(), 964, "Total registry size should be 964 after new()");
}

// ──────────────────────────────────────────────
// Test 9: Registry — get by name
// ──────────────────────────────────────────────
#[test]
fn test_registry_get() {
    let reg = LensRegistry::new();

    let consciousness = reg.get("consciousness");
    assert!(consciousness.is_some(), "Should find 'consciousness' lens");
    assert_eq!(consciousness.unwrap().category, LensCategory::Core);

    let quantum_micro = reg.get("quantum_microscope");
    assert!(quantum_micro.is_some(), "Should find 'quantum_microscope' lens");

    let nonexistent = reg.get("nonexistent_lens");
    assert!(nonexistent.is_none(), "Should return None for unknown lens");
}

// ──────────────────────────────────────────────
// Test 10: Registry — by_category filtering
// ──────────────────────────────────────────────
#[test]
fn test_registry_by_category() {
    let mut reg = LensRegistry::new();

    // 942 Extended, no Custom or DomainCombo
    assert_eq!(reg.by_category(LensCategory::Extended).len(), 942);
    assert_eq!(reg.by_category(LensCategory::Custom).len(), 0);
    assert_eq!(reg.by_category(LensCategory::DomainCombo).len(), 0);

    // Add one more Extended
    reg.register(LensEntry {
        name: "test_extended".into(),
        category: LensCategory::Extended,
        description: "test".into(),
        domain_affinity: vec![],
        complementary: vec![],
    });
    assert_eq!(reg.by_category(LensCategory::Extended).len(), 943);
    assert_eq!(reg.len(), 965);
}

// ──────────────────────────────────────────────
// Test 11: Registry — for_domain recommendation
// ──────────────────────────────────────────────
#[test]
fn test_registry_for_domain() {
    let reg = LensRegistry::new();

    let ai_lenses = reg.for_domain("ai");
    assert!(
        ai_lenses.len() >= 3,
        "At least consciousness, info, causal should match 'ai', got {}",
        ai_lenses.len()
    );

    let chip_lenses = reg.for_domain("chip");
    assert!(
        chip_lenses.len() >= 2,
        "At least topology, em should match 'chip', got {}",
        chip_lenses.len()
    );

    // Empty domain returns nothing
    let empty = reg.for_domain("zzz_nonexistent_domain");
    assert_eq!(empty.len(), 0);
}

// ──────────────────────────────────────────────
// Test 12: Domain combos — 10 combos defined
// ──────────────────────────────────────────────
#[test]
fn test_domain_combos() {
    let combos = default_combos();
    assert_eq!(combos.len(), 10, "Must have exactly 10 domain combos");

    // Verify the default combo
    let default = combos.iter().find(|c| c.name == "default").unwrap();
    assert_eq!(default.lenses, vec!["consciousness", "topology", "causal"]);

    // Verify geometric combo
    let geo = combos.iter().find(|c| c.name == "geometric").unwrap();
    assert_eq!(geo.lenses, vec!["ruler", "triangle", "compass"]);

    // Every combo references only valid Core lens names
    let core_entries = core_lens_entries();
    let core_names: Vec<&str> = core_entries.iter().map(|e| e.name.as_str()).collect();
    for combo in &combos {
        for lens in &combo.lenses {
            assert!(
                core_names.contains(&lens.as_str()),
                "Combo '{}' references unknown lens '{}'",
                combo.name,
                lens
            );
        }
    }
}

// ──────────────────────────────────────────────
// Test 13: Register custom lens
// ──────────────────────────────────────────────
#[test]
fn test_register_custom() {
    let mut reg = LensRegistry::new();
    assert_eq!(reg.len(), 964);

    reg.register(LensEntry {
        name: "my_custom_lens".into(),
        category: LensCategory::Custom,
        description: "A user-defined custom lens".into(),
        domain_affinity: vec!["robotics".into(), "ai".into()],
        complementary: vec!["consciousness".into()],
    });

    assert_eq!(reg.len(), 965);

    let custom = reg.get("my_custom_lens").unwrap();
    assert_eq!(custom.category, LensCategory::Custom);
    assert!(custom.domain_affinity.contains(&"robotics".to_string()));

    // Custom should appear in by_category
    assert_eq!(reg.by_category(LensCategory::Custom).len(), 1);

    // Custom should appear in for_domain
    let robotics = reg.for_domain("robotics");
    assert!(
        robotics.iter().any(|e| e.name == "my_custom_lens"),
        "Custom lens should appear in robotics domain results"
    );
}

// ──────────────────────────────────────────────
// Test 14: n6 industry lenses — 58 count
// ──────────────────────────────────────────────
#[test]
fn test_n6_industry_lens_count() {
    let entries = n6_industry_lens_entries();
    assert_eq!(entries.len(), 58, "Must have exactly 58 n6-industry lenses");
}

// ──────────────────────────────────────────────
// Test 15: cross-project lenses — 75 count (40 + 35 meta)
// ──────────────────────────────────────────────
#[test]
fn test_cross_project_lens_count() {
    let entries = cross_project_lens_entries();
    assert_eq!(entries.len(), 75, "Must have exactly 75 cross-project lenses (40 + 35 meta)");
}

// ──────────────────────────────────────────────
// Test 16: global name uniqueness — no duplicates across all 411
// ──────────────────────────────────────────────
#[test]
fn test_global_lens_name_uniqueness() {
    let mut all_names: Vec<String> = Vec::new();

    for e in core_lens_entries() {
        all_names.push(e.name.clone());
    }
    for e in n6_industry_lens_entries() {
        all_names.push(e.name.clone());
    }
    for e in cross_project_lens_entries() {
        all_names.push(e.name.clone());
    }
    for e in tecs_math_lens_entries() {
        all_names.push(e.name.clone());
    }
    for e in anima_consciousness_lens_entries() {
        all_names.push(e.name.clone());
    }
    for e in sedi_signal_lens_entries() {
        all_names.push(e.name.clone());
    }
    for e in accel_ml_lens_entries() {
        all_names.push(e.name.clone());
    }
    for e in accel_physics_neuro_lens_entries() {
        all_names.push(e.name.clone());
    }
    for e in accel_engineering_lens_entries() {
        all_names.push(e.name.clone());
    }
    for e in accel_humanities_lens_entries() {
        all_names.push(e.name.clone());
    }
    for e in nexus6::telescope::quantum_lenses::quantum_topology_lens_entries() {
        all_names.push(e.name.clone());
    }
    for e in nexus6::telescope::physics_deep_lenses::physics_deep_lens_entries() {
        all_names.push(e.name.clone());
    }

    let total = all_names.len();
    assert_eq!(total, 964, "Total = 22+58+75+103+88+100+58+57+55+63+236+49 = 964");

    all_names.sort();
    for i in 1..all_names.len() {
        assert_ne!(
            all_names[i - 1], all_names[i],
            "Duplicate lens name found: '{}'",
            all_names[i]
        );
    }
}

// ──────────────────────────────────────────────
// Test 17: Registry total after new() = 411
// ──────────────────────────────────────────────
#[test]
fn test_registry_total_411() {
    let reg = LensRegistry::new();
    assert_eq!(reg.len(), 964, "Registry should have 964 lenses total");

    let extended = reg.by_category(LensCategory::Extended);
    assert_eq!(extended.len(), 942, "Extended category should have 942 lenses");
}

// ──────────────────────────────────────────────
// Test 18: TECS-L math lenses — 103 count
// ──────────────────────────────────────────────
#[test]
fn test_tecs_math_lens_count() {
    let entries = tecs_math_lens_entries();
    assert_eq!(entries.len(), 103, "Must have exactly 103 TECS-L math lenses");
}

// ──────────────────────────────────────────────
// Test 19: TECS-L math lenses — names unique
// ──────────────────────────────────────────────
#[test]
fn test_tecs_math_lens_names_unique() {
    let entries = tecs_math_lens_entries();
    let mut names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
    let total = names.len();
    names.sort();
    names.dedup();
    assert_eq!(names.len(), total, "All 103 TECS-L math lens names must be unique");
}

// ──────────────────────────────────────────────
// Test 20: TECS-L lenses in registry for pure_mathematics domain
// ──────────────────────────────────────────────
#[test]
fn test_tecs_lenses_domain_affinity() {
    let reg = LensRegistry::new();
    let math_lenses = reg.for_domain("pure_mathematics");
    // Most of the 103 TECS-L lenses have pure_mathematics affinity
    assert!(
        math_lenses.len() >= 80,
        "At least 80 lenses should match 'pure_mathematics', got {}",
        math_lenses.len()
    );
}
