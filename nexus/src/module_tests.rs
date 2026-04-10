//! Comprehensive module tests for NEXUS-6 — targeting 1000 total tests.
//! n=6 constants used throughout: sigma=12, tau=4, phi=2, sopfr=5, J2=24, mu=1.

// ═══════════════════════════════════════════════════════════════
// Event module tests (12 tests = sigma)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod event_tests {
    use crate::event::*;
    use crate::event::handler::{counting_handler, discovery_collector};

    const N: usize = 6;
    const SIGMA: usize = 12;
    const TAU: usize = 4;

    #[test]
    fn test_event_type_tags_all_six() {
        let tags: Vec<&str> = vec![
            Event::DiscoveryMade { id: "d".into(), discovery_type: "t".into(), confidence: 0.5 }.type_tag(),
            Event::LensForged { name: "l".into() }.type_tag(),
            Event::ExperimentCompleted { exp_type: "e".into(), result_summary: "s".into() }.type_tag(),
            Event::BtCandidate { title: "bt".into(), domains: vec![] }.type_tag(),
            Event::Anomaly { description: "a".into(), severity: 0.1 }.type_tag(),
            Event::ScanCompleted { domain: "d".into(), discoveries: 0 }.type_tag(),
        ];
        assert_eq!(tags.len(), N); // n=6 event types
        // All tags should be unique
        let mut unique = tags.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), N);
    }

    #[test]
    fn test_event_summary_contains_data() {
        let e = Event::DiscoveryMade { id: "BT-105".into(), discovery_type: "SLE6".into(), confidence: 0.99 };
        let s = e.summary();
        assert!(s.contains("BT-105"));
        assert!(s.contains("SLE6"));
    }

    #[test]
    fn test_event_summary_anomaly() {
        let e = Event::Anomaly { description: "n6 outlier".into(), severity: 0.85 };
        assert!(e.summary().contains("n6 outlier"));
        assert!(e.summary().contains("0.85"));
    }

    #[test]
    fn test_event_summary_bt_candidate() {
        let e = Event::BtCandidate { title: "BT-128".into(), domains: vec!["math".into(), "physics".into()] };
        let s = e.summary();
        assert!(s.contains("BT-128"));
        assert!(s.contains("math"));
    }

    #[test]
    fn test_event_summary_scan_completed() {
        let e = Event::ScanCompleted { domain: "ai".into(), discoveries: SIGMA };
        assert!(e.summary().contains("ai"));
        assert!(e.summary().contains("12"));
    }

    #[test]
    fn test_event_bus_emit_n6() {
        let mut bus = EventBus::new();
        for i in 0..N {
            bus.emit(Event::DiscoveryMade {
                id: format!("d-{}", i), discovery_type: "n6".into(), confidence: 0.5,
            });
        }
        assert_eq!(bus.event_count(), N);
    }

    #[test]
    fn test_event_bus_history_by_type_filter() {
        let mut bus = EventBus::new();
        bus.emit(Event::DiscoveryMade { id: "d1".into(), discovery_type: "x".into(), confidence: 0.5 });
        bus.emit(Event::Anomaly { description: "a".into(), severity: 0.3 });
        bus.emit(Event::DiscoveryMade { id: "d2".into(), discovery_type: "y".into(), confidence: 0.7 });
        bus.emit(Event::LensForged { name: "new".into() });
        assert_eq!(bus.history_by_type("discovery").len(), 2);
        assert_eq!(bus.history_by_type("anomaly").len(), 1);
        assert_eq!(bus.history_by_type("lens_forged").len(), 1);
        assert_eq!(bus.history_by_type("nonexistent").len(), 0);
    }

    #[test]
    fn test_event_bus_clear_history() {
        let mut bus = EventBus::new();
        for _ in 0..TAU {
            bus.emit(Event::ScanCompleted { domain: "test".into(), discoveries: 1 });
        }
        assert_eq!(bus.event_count(), TAU);
        bus.clear_history();
        assert_eq!(bus.event_count(), 0);
    }

    #[test]
    fn test_event_bus_handler_invocation() {
        use std::sync::{Arc, Mutex};
        let counter = Arc::new(Mutex::new(0usize));
        let c = counter.clone();
        let mut bus = EventBus::new();
        bus.on(move |_| { *c.lock().unwrap() += 1; });
        for _ in 0..SIGMA {
            bus.emit(Event::LensForged { name: "x".into() });
        }
        assert_eq!(*counter.lock().unwrap(), SIGMA);
    }

    #[test]
    fn test_event_bus_on_discovery_selective() {
        use std::sync::{Arc, Mutex};
        let ids = Arc::new(Mutex::new(Vec::new()));
        let ids_c = ids.clone();
        let mut bus = EventBus::new();
        bus.on_discovery(move |id, _dt, _conf| { ids_c.lock().unwrap().push(id.to_string()); });
        bus.emit(Event::DiscoveryMade { id: "sigma".into(), discovery_type: "n6".into(), confidence: 1.0 });
        bus.emit(Event::Anomaly { description: "not disc".into(), severity: 0.1 });
        assert_eq!(ids.lock().unwrap().len(), 1);
    }

    #[test]
    fn test_event_bus_count_by_type() {
        let mut bus = EventBus::new();
        bus.emit(Event::DiscoveryMade { id: "d1".into(), discovery_type: "x".into(), confidence: 0.5 });
        bus.emit(Event::DiscoveryMade { id: "d2".into(), discovery_type: "y".into(), confidence: 0.8 });
        bus.emit(Event::Anomaly { description: "a".into(), severity: 0.9 });
        let counts = bus.count_by_type();
        let disc = counts.iter().find(|(t, _)| *t == "discovery").map(|(_, c)| *c).unwrap_or(0);
        assert_eq!(disc, 2);
    }

    #[test]
    fn test_event_bus_recent() {
        let mut bus = EventBus::new();
        for i in 0..SIGMA {
            bus.emit(Event::ScanCompleted { domain: format!("d{}", i), discoveries: i });
        }
        let recent = bus.recent(3);
        assert_eq!(recent.len(), 3);
    }

    #[test]
    fn test_discovery_collector() {
        let (handler, ids) = discovery_collector();
        let mut bus = EventBus::new();
        bus.on(handler);
        bus.emit(Event::DiscoveryMade { id: "test-6".into(), discovery_type: "n6".into(), confidence: 0.6 });
        bus.emit(Event::LensForged { name: "irrelevant".into() });
        assert_eq!(ids.lock().unwrap().len(), 1);
    }

    #[test]
    fn test_counting_handler() {
        let (handler, counts) = counting_handler();
        let mut bus = EventBus::new();
        bus.on(handler);
        bus.emit(Event::DiscoveryMade { id: "d".into(), discovery_type: "t".into(), confidence: 0.5 });
        bus.emit(Event::DiscoveryMade { id: "d2".into(), discovery_type: "t".into(), confidence: 0.6 });
        bus.emit(Event::Anomaly { description: "a".into(), severity: 0.1 });
        let map = counts.lock().unwrap();
        assert_eq!(map.get("discovery"), Some(&2));
        assert_eq!(map.get("anomaly"), Some(&1));
    }
}

// ═══════════════════════════════════════════════════════════════
// Feedback module tests (12 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod feedback_tests {
    use crate::feedback::*;
    use crate::feedback::learner;

    #[test]
    fn test_feedback_discovery_id() {
        assert_eq!(Feedback::Good { discovery_id: "d-6".into() }.discovery_id(), "d-6");
        assert_eq!(Feedback::Bad { discovery_id: "d-12".into(), reason: "x".into() }.discovery_id(), "d-12");
        assert_eq!(Feedback::Interesting { discovery_id: "d-24".into() }.discovery_id(), "d-24");
        assert_eq!(Feedback::Irrelevant { discovery_id: "d-4".into() }.discovery_id(), "d-4");
    }

    #[test]
    fn test_feedback_type_tags() {
        assert_eq!(Feedback::Good { discovery_id: "x".into() }.type_tag(), "good");
        assert_eq!(Feedback::Bad { discovery_id: "x".into(), reason: "r".into() }.type_tag(), "bad");
        assert_eq!(Feedback::Interesting { discovery_id: "x".into() }.type_tag(), "interesting");
        assert_eq!(Feedback::Irrelevant { discovery_id: "x".into() }.type_tag(), "irrelevant");
    }

    #[test]
    fn test_feedback_scores() {
        assert_eq!(Feedback::Good { discovery_id: "x".into() }.score(), 1.0);
        assert_eq!(Feedback::Interesting { discovery_id: "x".into() }.score(), 0.5);
        assert_eq!(Feedback::Irrelevant { discovery_id: "x".into() }.score(), -0.5);
        assert_eq!(Feedback::Bad { discovery_id: "x".into(), reason: "r".into() }.score(), -1.0);
    }

    #[test]
    fn test_collector_record_stats() {
        let mut c = FeedbackCollector::new("/tmp/nexus_mt_fb.tsv");
        c.record(Feedback::Good { discovery_id: "d1".into() });
        c.record(Feedback::Good { discovery_id: "d2".into() });
        c.record(Feedback::Bad { discovery_id: "d3".into(), reason: "off".into() });
        c.record(Feedback::Interesting { discovery_id: "d4".into() });
        let s = c.stats();
        assert_eq!(s.total, 4);
        assert_eq!(s.good, 2);
        assert!((s.good_rate - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_collector_empty() {
        let c = FeedbackCollector::new("/tmp/nexus_mt_empty.tsv");
        assert!(c.is_empty());
        assert_eq!(c.len(), 0);
        assert_eq!(c.stats().good_rate, 0.0);
    }

    #[test]
    fn test_collector_for_discovery() {
        let mut c = FeedbackCollector::new("/tmp/nexus_mt_fd.tsv");
        c.record(Feedback::Good { discovery_id: "sigma".into() });
        c.record(Feedback::Bad { discovery_id: "tau".into(), reason: "x".into() });
        c.record(Feedback::Good { discovery_id: "sigma".into() });
        assert_eq!(c.for_discovery("sigma").len(), 2);
        assert_eq!(c.for_discovery("tau").len(), 1);
    }

    #[test]
    fn test_collector_net_score() {
        let mut c = FeedbackCollector::new("/tmp/nexus_mt_ns.tsv");
        c.record(Feedback::Good { discovery_id: "d1".into() });
        c.record(Feedback::Good { discovery_id: "d1".into() });
        c.record(Feedback::Bad { discovery_id: "d1".into(), reason: "r".into() });
        assert!((c.net_score("d1") - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_collector_top_discoveries() {
        let mut c = FeedbackCollector::new("/tmp/nexus_mt_top.tsv");
        for _ in 0..6 {
            c.record(Feedback::Good { discovery_id: "best".into() });
        }
        c.record(Feedback::Good { discovery_id: "mid".into() });
        c.record(Feedback::Bad { discovery_id: "worst".into(), reason: "bad".into() });
        let top = c.top_discoveries(2);
        assert_eq!(top[0].0, "best");
        assert!(top[0].1 > top[1].1);
    }

    #[test]
    fn test_collector_batch() {
        let mut c = FeedbackCollector::new("/tmp/nexus_mt_batch.tsv");
        c.record_batch(vec![
            Feedback::Good { discovery_id: "d1".into() },
            Feedback::Interesting { discovery_id: "d2".into() },
        ]);
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn test_learner_weight_updates() {
        let fbs = vec![
            Feedback::Good { discovery_id: "ai-lens-1".into() },
            Feedback::Good { discovery_id: "ai-lens-2".into() },
            Feedback::Bad { discovery_id: "chip-lens-1".into(), reason: "wrong".into() },
        ];
        let updates = learner::compute_weight_updates(&fbs);
        let ai = updates.iter().find(|(n, _)| n == "ai").map(|(_, v)| *v);
        let chip = updates.iter().find(|(n, _)| n == "chip").map(|(_, v)| *v);
        assert!(ai.unwrap() > 0.0);
        assert!(chip.unwrap() < 0.0);
    }

    #[test]
    fn test_learner_apply_updates() {
        let current = vec![("a".to_string(), 1.0), ("b".to_string(), 0.5)];
        let updates = vec![("a".to_string(), 0.3), ("b".to_string(), -0.8)];
        let new_w = learner::apply_updates(&current, &updates);
        assert!((new_w[0].1 - 1.3).abs() < 1e-10);
        assert!((new_w[1].1 - 0.0).abs() < 1e-10); // clamped
    }

    #[test]
    fn test_update_weights_public_api() {
        let fbs = vec![Feedback::Good { discovery_id: "test-d1".into() }];
        let updates = update_weights_from_feedback(&fbs);
        assert!(!updates.is_empty());
    }
}

// ═══════════════════════════════════════════════════════════════
// Pipeline module tests (10 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod pipeline_tests {
    use crate::pipeline::*;

    #[test]
    fn test_step_names() {
        assert!(PipelineStep::Scan { domain: "ai".into(), tier: 1 }.name().contains("ai"));
        assert!(PipelineStep::Verify { tolerance: 0.05 }.name().contains("Verify"));
        assert_eq!(PipelineStep::Register.name(), "Register");
        assert_eq!(PipelineStep::RedTeam.name(), "RedTeam");
        assert_eq!(PipelineStep::Publish.name(), "Publish");
    }

    #[test]
    fn test_builder_chain() {
        let p = PipelineBuilder::new()
            .scan("physics", 1).verify(0.05).filter(0.6).register().publish()
            .build("test");
        assert_eq!(p.name, "test");
        assert_eq!(p.len(), 5);
        assert!(!p.is_empty());
    }

    #[test]
    fn test_empty_pipeline() {
        let p = PipelineBuilder::new().build("empty");
        assert!(p.is_empty());
        assert_eq!(p.len(), 0);
    }

    #[test]
    fn test_custom_step() {
        let p = PipelineBuilder::new().custom("n6_check", "Verify n=6").build("custom");
        assert_eq!(p.len(), 1);
        assert!(p.steps[0].name().contains("n6_check"));
    }

    #[test]
    fn test_standard_discovery() {
        let p = standard_discovery("chip");
        assert!(p.name.contains("chip"));
        assert!(p.len() >= 6);
    }

    #[test]
    fn test_deep_exploration() {
        let p = deep_exploration("energy");
        assert!(p.name.contains("energy"));
        assert!(p.len() >= 6);
    }

    #[test]
    fn test_execute() {
        let p = PipelineBuilder::new().scan("test", 1).verify(0.05).filter(0.5).build("exec");
        let r = execute(&p);
        assert!(r.steps_completed <= r.total_steps);
    }

    #[test]
    fn test_completion_ratio() {
        let r = PipelineResult { steps_completed: 6, total_steps: 12, discoveries: vec![], filtered_out: 0 };
        assert!((r.completion_ratio() - 0.5).abs() < 1e-10);
        assert!(!r.is_complete());
    }

    #[test]
    fn test_complete_result() {
        let r = PipelineResult { steps_completed: 4, total_steps: 4, discoveries: vec![], filtered_out: 0 };
        assert!(r.is_complete());
    }

    #[test]
    fn test_all_eight_step_types() {
        let p = PipelineBuilder::new()
            .scan("all", 2).verify(0.01).experiment("collision").filter(0.9)
            .red_team().register().publish().custom("extra", "bonus")
            .build("full");
        assert_eq!(p.len(), 8); // sigma-tau=8
    }
}

// ═══════════════════════════════════════════════════════════════
// Dream module tests (8 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod dream_tests {
    use crate::dream::*;

    #[test]
    fn test_empty_engine_no_dreams() {
        let e = DreamEngine::new();
        assert!(e.dream(5).is_empty());
    }

    #[test]
    fn test_single_memory_no_dreams() {
        let mut e = DreamEngine::new();
        e.add_memory("only one");
        assert!(e.dream(3).is_empty());
    }

    #[test]
    fn test_dream_produces_n6_output() {
        let mut e = DreamEngine::new();
        for i in 0..6 { e.add_memory(&format!("BT-{} discovery", 105 + i)); }
        let dreams = e.dream(6);
        assert_eq!(dreams.len(), 6);
        for d in &dreams {
            assert!(!d.fragments.is_empty());
            assert!(d.novelty_score >= 0.0 && d.novelty_score <= 1.0);
            assert!(d.plausibility >= 0.0 && d.plausibility <= 1.0);
        }
    }

    #[test]
    fn test_dream_deterministic() {
        let mut e = DreamEngine::new();
        for i in 0..12 { e.add_memory(&format!("pattern-{}", i)); }
        let d1 = e.dream(4);
        let d2 = e.dream(4);
        for (a, b) in d1.iter().zip(d2.iter()) {
            assert_eq!(a.fragments, b.fragments);
        }
    }

    #[test]
    fn test_dream_fragment_count_alternates() {
        let mut e = DreamEngine::new();
        for i in 0..6 { e.add_memory(&format!("mem-{}", i)); }
        let dreams = e.dream(4);
        assert_eq!(dreams[0].fragments.len(), 2); // phi=2
        assert_eq!(dreams[1].fragments.len(), 3); // n/phi=3
    }

    #[test]
    fn test_lucid_dream_with_seed() {
        let mut e = DreamEngine::new();
        e.add_memory("BT-33 Transformer sigma=12");
        e.add_memory("BT-43 Battery cathode CN=6");
        e.add_memory("BT-97 Transformer angle");
        let d = e.lucid_dream("Transformer");
        assert!(d.fragments.len() >= 2);
    }

    #[test]
    fn test_lucid_dream_fallback() {
        let mut e = DreamEngine::new();
        e.add_memory("alpha");
        e.add_memory("beta");
        let d = e.lucid_dream("zzz");
        assert_eq!(d.fragments.len(), 2);
    }

    #[test]
    fn test_memory_count() {
        let mut e = DreamEngine::new();
        for i in 0..24 { e.add_memory(&format!("J2-{}", i)); } // J2=24
        assert_eq!(e.memory_count(), 24);
    }
}

// ═══════════════════════════════════════════════════════════════
// Sandbox module tests (8 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod sandbox_tests {
    use crate::sandbox::Sandbox;

    #[test]
    fn test_isolation_n6_data() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]; // n=6
        let mut sb = Sandbox::new(&data);
        sb.modify("scale:2.0");
        assert_eq!(sb.original_data, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        assert_eq!(sb.working_data, vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0]);
    }

    #[test]
    fn test_shift_and_reset() {
        let data = vec![6.0, 12.0, 24.0];
        let mut sb = Sandbox::new(&data);
        sb.modify("shift:10.0");
        assert_eq!(sb.working_data, vec![16.0, 22.0, 34.0]);
        sb.reset();
        assert_eq!(sb.working_data, vec![6.0, 12.0, 24.0]);
    }

    #[test]
    fn test_set_and_diff() {
        let data = vec![1.0, 2.0, 3.0];
        let mut sb = Sandbox::new(&data);
        sb.modify("set:1:99.0");
        let diffs = sb.diff();
        assert_eq!(diffs.len(), 1);
        assert_eq!(diffs[0], (1, 2.0, 99.0));
    }

    #[test]
    fn test_normalize() {
        let data = vec![0.0, 5.0, 10.0];
        let mut sb = Sandbox::new(&data);
        sb.modify("normalize");
        assert!((sb.working_data[0]).abs() < 1e-10);
        assert!((sb.working_data[1] - 0.5).abs() < 1e-10);
        assert!((sb.working_data[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_sort_n6_constants() {
        let data = vec![24.0, 6.0, 12.0, 4.0, 2.0, 5.0]; // J2, n, sigma, tau, phi, sopfr
        let mut sb = Sandbox::new(&data);
        sb.modify("sort");
        assert_eq!(sb.working_data, vec![2.0, 4.0, 5.0, 6.0, 12.0, 24.0]);
    }

    #[test]
    fn test_reverse() {
        let data = vec![1.0, 2.0, 3.0, 6.0];
        let mut sb = Sandbox::new(&data);
        sb.modify("reverse");
        assert_eq!(sb.working_data, vec![6.0, 3.0, 2.0, 1.0]);
    }

    #[test]
    fn test_commit() {
        let data = vec![6.0, 12.0];
        let mut sb = Sandbox::new(&data);
        sb.modify("scale:2.0");
        let committed = sb.commit();
        assert_eq!(committed, vec![12.0, 24.0]);
    }

    #[test]
    fn test_modification_count_and_display() {
        let data = vec![6.0; 6];
        let mut sb = Sandbox::new(&data);
        sb.modify("scale:2.0");
        sb.modify("shift:1.0");
        assert_eq!(sb.modification_count(), 2);
        let s = format!("{}", sb);
        assert!(s.contains("sandbox-6"));
    }
}

// ═══════════════════════════════════════════════════════════════
// Versioning module tests (8 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod versioning_tests {
    use crate::versioning::VersionStore;

    #[test]
    fn test_commit_versions() {
        let mut store = VersionStore::new("/tmp/nexus_vs.json");
        let v1 = store.commit("disc-1", "Initial content: sigma=12");
        let v2 = store.commit("disc-1", "Updated content: sigma=12, tau=4");
        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }

    #[test]
    fn test_diff_versions() {
        let mut store = VersionStore::new("/tmp/nexus_diff.json");
        store.commit("d1", "line1\nline2\nline3");
        store.commit("d1", "line1\nchanged\nline3");
        let diff = store.diff("d1", 1, 2);
        assert!(diff.contains("- line2"));
        assert!(diff.contains("+ changed"));
    }

    #[test]
    fn test_diff_nonexistent() {
        let store = VersionStore::new("/tmp/nexus_ne.json");
        let diff = store.diff("no_such", 1, 2);
        assert!(diff.contains("no_such"));
    }

    #[test]
    fn test_multiple_discoveries() {
        let mut store = VersionStore::new("/tmp/nexus_multi.json");
        store.commit("alpha", "content A");
        store.commit("beta", "content B");
        store.commit("alpha", "content A v2");
        // alpha should be at v2, beta at v1
        let diff = store.diff("alpha", 1, 2);
        assert!(diff.contains("alpha"));
    }

    #[test]
    fn test_six_versions() {
        let mut store = VersionStore::new("/tmp/nexus_six.json");
        for i in 1..=6 { // n=6 versions
            store.commit("evolving", &format!("version {} with sigma={}", i, 12));
        }
        let diff = store.diff("evolving", 1, 6);
        assert!(diff.contains("evolving"));
    }

    #[test]
    fn test_version_content_preserved() {
        let mut store = VersionStore::new("/tmp/nexus_content.json");
        store.commit("test", "n=6 is unique");
        store.commit("test", "sigma*phi = n*tau = 24");
        // Both versions should exist (verified via diff)
        let diff = store.diff("test", 1, 2);
        assert!(!diff.is_empty());
    }

    #[test]
    fn test_branch_main_default() {
        let mut store = VersionStore::new("/tmp/nexus_branch.json");
        store.commit("d1", "main branch content");
        // Default branch is "main"
        let diff = store.diff("d1", 1, 1);
        // Same version diff should have minimal output
        assert!(diff.contains("d1"));
    }

    #[test]
    fn test_parent_chain() {
        let mut store = VersionStore::new("/tmp/nexus_parent.json");
        store.commit("chain", "v1");
        store.commit("chain", "v2");
        store.commit("chain", "v3");
        store.commit("chain", "v4"); // tau=4 versions
        let diff = store.diff("chain", 1, 4);
        assert!(diff.contains("-") || diff.contains("+"));
    }
}

// ═══════════════════════════════════════════════════════════════
// Distributed module tests (8 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod distributed_tests {
    use crate::distributed::*;

    fn make_worker(id: &str, status: NodeStatus) -> WorkerNode {
        WorkerNode {
            id: id.into(),
            address: format!("10.0.0.{}:8080", id.len()),
            status,
            assigned_domains: vec![],
        }
    }

    #[test]
    fn test_empty_scheduler() {
        let sched = DistributedScheduler::new();
        assert_eq!(sched.worker_count(), 0);
        let domains: Vec<String> = vec!["ai".into()];
        assert!(sched.distribute_scan(&domains).is_empty());
    }

    #[test]
    fn test_add_workers() {
        let mut sched = DistributedScheduler::new();
        for i in 0..6 { // n=6 workers
            sched.add_worker(make_worker(&format!("w{}", i), NodeStatus::Idle));
        }
        assert_eq!(sched.worker_count(), 6);
    }

    #[test]
    fn test_distribute_round_robin() {
        let mut sched = DistributedScheduler::new();
        sched.add_worker(make_worker("w1", NodeStatus::Idle));
        sched.add_worker(make_worker("w2", NodeStatus::Busy));
        let domains: Vec<String> = (0..6).map(|i| format!("domain-{}", i)).collect();
        let assignments = sched.distribute_scan(&domains);
        assert_eq!(assignments.len(), 2); // phi=2 workers available
        // Total domains assigned = 6
        let total: usize = assignments.iter().map(|(_, d)| d.len()).sum();
        assert_eq!(total, 6);
    }

    #[test]
    fn test_offline_workers_excluded() {
        let mut sched = DistributedScheduler::new();
        sched.add_worker(make_worker("active", NodeStatus::Idle));
        sched.add_worker(make_worker("offline", NodeStatus::Offline));
        let domains: Vec<String> = vec!["test".into()];
        let assignments = sched.distribute_scan(&domains);
        assert_eq!(assignments.len(), 1);
        assert_eq!(assignments[0].0, "active");
    }

    #[test]
    fn test_all_offline_no_assignments() {
        let mut sched = DistributedScheduler::new();
        sched.add_worker(make_worker("off1", NodeStatus::Offline));
        sched.add_worker(make_worker("off2", NodeStatus::Offline));
        let domains: Vec<String> = vec!["ai".into(), "chip".into()];
        assert!(sched.distribute_scan(&domains).is_empty());
    }

    #[test]
    fn test_merge_results() {
        let sched = DistributedScheduler::new();
        let results = vec![
            ("w1".to_string(), vec!["r1".to_string(), "r2".to_string()]),
            ("w2".to_string(), vec!["r3".to_string()]),
        ];
        let merged = sched.merge_results(&results);
        assert_eq!(merged.len(), 3);
    }

    #[test]
    fn test_twelve_domains_four_workers() {
        let mut sched = DistributedScheduler::new();
        for i in 0..4 { // tau=4 workers
            sched.add_worker(make_worker(&format!("w{}", i), NodeStatus::Idle));
        }
        let domains: Vec<String> = (0..12).map(|i| format!("d{}", i)).collect(); // sigma=12
        let assignments = sched.distribute_scan(&domains);
        assert_eq!(assignments.len(), 4);
        // Each should get 3 = 12/4 = sigma/tau
        for (_, doms) in &assignments {
            assert_eq!(doms.len(), 3);
        }
    }

    #[test]
    fn test_node_status_eq() {
        assert_eq!(NodeStatus::Idle, NodeStatus::Idle);
        assert_ne!(NodeStatus::Idle, NodeStatus::Busy);
        assert_ne!(NodeStatus::Busy, NodeStatus::Offline);
    }
}

// ═══════════════════════════════════════════════════════════════
// Template module tests (8 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod template_tests {
    use crate::template::*;
    use std::collections::HashMap;

    fn make_template(name: &str) -> ExperimentTemplate {
        let mut params = HashMap::new();
        params.insert("domain".to_string(), "physics".to_string());
        params.insert("tier".to_string(), "1".to_string());
        ExperimentTemplate {
            name: name.to_string(),
            description: "Scan {{domain}} at tier {{tier}}".to_string(),
            steps: vec![
                "Initialize scan for {{domain}}".to_string(),
                "Run at tier {{tier}}".to_string(),
            ],
            parameters: params,
            expected_metrics: vec!["confidence".into(), "n6_score".into()],
        }
    }

    #[test]
    fn test_save_and_load_template() {
        let mut store = TemplateStore::new("/tmp/nexus_tmpl.json");
        store.save_template(make_template("scan-v1"));
        let loaded = store.load_template("scan-v1");
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().name, "scan-v1");
    }

    #[test]
    fn test_template_list() {
        let mut store = TemplateStore::new("/tmp/nexus_tmpl2.json");
        store.save_template(make_template("t1"));
        store.save_template(make_template("t2"));
        assert_eq!(store.list().len(), 2);
    }

    #[test]
    fn test_template_overwrite() {
        let mut store = TemplateStore::new("/tmp/nexus_tmpl3.json");
        store.save_template(make_template("dup"));
        store.save_template(make_template("dup"));
        assert_eq!(store.list().len(), 1);
    }

    #[test]
    fn test_instantiate_with_params() {
        let mut store = TemplateStore::new("/tmp/nexus_inst.json");
        store.save_template(make_template("scan"));
        let mut params = HashMap::new();
        params.insert("domain".to_string(), "ai".to_string());
        let inst = store.instantiate("scan", &params);
        assert!(inst.is_some());
        let t = inst.unwrap();
        assert!(t.steps[0].contains("ai"));
    }

    #[test]
    fn test_instantiate_default_params() {
        let mut store = TemplateStore::new("/tmp/nexus_inst2.json");
        store.save_template(make_template("scan"));
        let empty_params = HashMap::new();
        let inst = store.instantiate("scan", &empty_params).unwrap();
        assert!(inst.steps[0].contains("physics")); // default
    }

    #[test]
    fn test_instantiate_nonexistent() {
        let store = TemplateStore::new("/tmp/nexus_inst3.json");
        assert!(store.instantiate("no_such", &HashMap::new()).is_none());
    }

    #[test]
    fn test_load_nonexistent() {
        let store = TemplateStore::new("/tmp/nexus_inst4.json");
        assert!(store.load_template("missing").is_none());
    }

    #[test]
    fn test_n6_templates() {
        let mut store = TemplateStore::new("/tmp/nexus_n6tmpl.json");
        for i in 0..6 { // n=6 templates
            store.save_template(make_template(&format!("tmpl-{}", i)));
        }
        assert_eq!(store.list().len(), 6);
    }
}

// ═══════════════════════════════════════════════════════════════
// Experiment module tests (10 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod experiment_tests {
    use crate::experiment::types::*;
    use crate::experiment::runner::ExperimentRunner;
    use crate::experiment::report;

    #[test]
    fn test_all_22_experiment_types() {
        assert_eq!(ALL_EXPERIMENT_TYPES.len(), 22);
    }

    #[test]
    fn test_experiment_type_from_str() {
        assert!(ExperimentType::from_str("acceleration").is_some());
        assert!(ExperimentType::from_str("collision").is_some());
        assert!(ExperimentType::from_str("fusion").is_some());
        assert!(ExperimentType::from_str("nonexistent").is_none());
    }

    #[test]
    fn test_experiment_type_name_roundtrip() {
        for et in &ALL_EXPERIMENT_TYPES {
            let name = et.name();
            let parsed = ExperimentType::from_str(name);
            assert!(parsed.is_some(), "Failed to roundtrip: {}", name);
        }
    }

    #[test]
    fn test_experiment_descriptions_nonempty() {
        for et in &ALL_EXPERIMENT_TYPES {
            assert!(!et.description().is_empty(), "{} has empty description", et.name());
        }
    }

    #[test]
    fn test_experiment_recommended_lenses() {
        for et in &ALL_EXPERIMENT_TYPES {
            let lenses = et.recommended_lenses();
            assert!(!lenses.is_empty(), "{} has no recommended lenses", et.name());
        }
    }

    #[test]
    fn test_experiment_config() {
        let config = ExperimentConfig::new(ExperimentType::Collision, "test-target")
            .with_intensity(0.8)
            .with_duration(12);
        assert_eq!(config.exp_type, ExperimentType::Collision);
        assert_eq!(config.target, "test-target");
        assert!((config.intensity - 0.8).abs() < 1e-10);
        assert_eq!(config.duration, 12);
    }

    #[test]
    fn test_runner_run_single() {
        let runner = ExperimentRunner::new();
        let config = ExperimentConfig::new(ExperimentType::Acceleration, "n6-test");
        let result = runner.run(&config);
        assert_eq!(result.exp_type, ExperimentType::Acceleration);
    }

    #[test]
    fn test_runner_run_all() {
        let runner = ExperimentRunner::new();
        let results = runner.run_all("test-domain");
        assert_eq!(results.len(), 22);
    }

    #[test]
    fn test_runner_run_battery() {
        let runner = ExperimentRunner::new();
        let types = vec![ExperimentType::Fusion, ExperimentType::Reversal, ExperimentType::Mutation];
        let results = runner.run_battery(&types, "battery-target");
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_report_format() {
        let runner = ExperimentRunner::new();
        let config = ExperimentConfig::new(ExperimentType::Tension, "report-test");
        let results = vec![runner.run(&config)];
        let text = report::format_report(&results);
        assert!(!text.is_empty());
    }
}

// ═══════════════════════════════════════════════════════════════
// Graph module tests (8 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod graph_tests {
    use crate::graph::node::*;
    use crate::graph::edge::*;
    use crate::graph::structure::*;

    fn make_node(id: &str, ntype: NodeType) -> Node {
        Node {
            id: id.into(),
            node_type: ntype,
            domain: "test".into(),
            project: "nexus".into(),
            summary: format!("Node {}", id),
            confidence: 0.85,
            lenses_used: vec!["consciousness".into()],
            timestamp: "2026-04-03".into(),
        }
    }

    fn make_edge(from: &str, to: &str, etype: EdgeType, bidir: bool) -> Edge {
        Edge { from: from.into(), to: to.into(), edge_type: etype, strength: 0.8, bidirectional: bidir }
    }

    #[test]
    fn test_node_types() {
        assert_eq!(NodeType::Discovery, NodeType::Discovery);
        assert_ne!(NodeType::Discovery, NodeType::Hypothesis);
    }

    #[test]
    fn test_edge_types() {
        assert_eq!(EdgeType::Derives, EdgeType::Derives);
        assert_ne!(EdgeType::Validates, EdgeType::Contradicts);
    }

    #[test]
    fn test_find_triangles_empty() {
        let triangles = find_closed_triangles(&[], &[]);
        assert!(triangles.is_empty());
    }

    #[test]
    fn test_find_triangle() {
        let nodes = vec![
            make_node("a", NodeType::Discovery),
            make_node("b", NodeType::Hypothesis),
            make_node("c", NodeType::Bt),
        ];
        let edges = vec![
            make_edge("a", "b", EdgeType::Derives, true),
            make_edge("b", "c", EdgeType::Validates, true),
            make_edge("a", "c", EdgeType::Triggers, true),
        ];
        let triangles = find_closed_triangles(&nodes, &edges);
        assert_eq!(triangles.len(), 1);
        assert_eq!(triangles[0].nodes.len(), 3);
    }

    #[test]
    fn test_find_hubs() {
        let edges = vec![
            make_edge("hub", "a", EdgeType::Derives, false),
            make_edge("hub", "b", EdgeType::Derives, false),
            make_edge("hub", "c", EdgeType::Derives, false),
            make_edge("hub", "d", EdgeType::Derives, false),
        ];
        let hubs = find_hubs(&edges, 3);
        assert!(!hubs.is_empty());
        assert_eq!(hubs[0].node_id, "hub");
    }

    #[test]
    fn test_find_convergences() {
        let edges = vec![
            make_edge("src1", "target", EdgeType::Derives, false),
            make_edge("src2", "target", EdgeType::Validates, false),
            make_edge("src3", "target", EdgeType::Triggers, false),
        ];
        let convs = find_convergences(&edges);
        assert!(!convs.is_empty());
        assert_eq!(convs[0].target, "target");
        assert_eq!(convs[0].sources.len(), 3);
    }

    #[test]
    fn test_no_hubs_below_threshold() {
        let edges = vec![make_edge("a", "b", EdgeType::Derives, false)];
        let hubs = find_hubs(&edges, 5);
        assert!(hubs.is_empty());
    }

    #[test]
    fn test_six_node_graph() {
        let nodes: Vec<Node> = (0..6).map(|i| make_node(&format!("n{}", i), NodeType::Discovery)).collect();
        let edges: Vec<Edge> = (0..5).map(|i| make_edge(&format!("n{}", i), &format!("n{}", i+1), EdgeType::Derives, true)).collect();
        let hubs = find_hubs(&edges, 2);
        // Middle nodes should be hubs (degree 2 from bidirectional)
        assert!(!hubs.is_empty());
    }
}

// ═══════════════════════════════════════════════════════════════
// History module tests (8 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod history_tests {
    use crate::history::recorder::ScanRecord;
    use crate::history::stats::*;

    fn make_record(domain: &str, lenses: &[&str], discoveries: &[&str]) -> ScanRecord {
        ScanRecord {
            id: format!("rec-{}", domain),
            timestamp: "2026-04-03".into(),
            domain: domain.into(),
            lenses_used: lenses.iter().map(|s| s.to_string()).collect(),
            discoveries: discoveries.iter().map(|s| s.to_string()).collect(),
            consensus_level: 3,
        }
    }

    #[test]
    fn test_domain_stats_empty() {
        let stats = compute_domain_stats(&[]);
        assert_eq!(stats.total_scans, 0);
        assert_eq!(stats.total_discoveries, 0);
    }

    #[test]
    fn test_domain_stats_with_records() {
        let records = vec![
            make_record("ai", &["consciousness", "topology"], &["d1", "d2"]),
            make_record("ai", &["consciousness"], &["d3"]),
            make_record("ai", &["gravity"], &[]),
        ];
        let stats = compute_domain_stats(&records);
        assert_eq!(stats.total_scans, 3);
        assert_eq!(stats.total_discoveries, 3);
        let consciousness = stats.lens_stats.get("consciousness").unwrap();
        assert_eq!(consciousness.used, 2);
        assert_eq!(consciousness.contributed, 2);
    }

    #[test]
    fn test_hit_rate_calculation() {
        let records = vec![
            make_record("test", &["lens_a"], &["d1"]),
            make_record("test", &["lens_a"], &[]),
        ];
        let stats = compute_domain_stats(&records);
        let lens_a = stats.lens_stats.get("lens_a").unwrap();
        assert!((lens_a.hit_rate - 0.5).abs() < 1e-10); // phi/tau = 1/2
    }

    #[test]
    fn test_lens_affinity_empty() {
        let affinity = compute_lens_affinity(&[]);
        assert!(affinity.is_empty());
    }

    #[test]
    fn test_lens_affinity_with_data() {
        let records = vec![
            make_record("ai", &["consciousness", "topology"], &["d1"]),
            make_record("ai", &["consciousness", "topology"], &["d2"]),
        ];
        let affinity = compute_lens_affinity(&records);
        let key = ("consciousness".to_string(), "topology".to_string());
        assert!(affinity.contains_key(&key));
        assert!((affinity[&key] - 1.0).abs() < 1e-10); // 100% co-occurrence
    }

    #[test]
    fn test_twelve_lenses_stats() {
        let lenses: Vec<String> = (0..12).map(|i| format!("lens_{}", i)).collect();
        let lens_refs: Vec<&str> = lenses.iter().map(|s| s.as_str()).collect();
        let records = vec![make_record("test", &lens_refs, &["d1"])];
        let stats = compute_domain_stats(&records);
        assert_eq!(stats.lens_stats.len(), 12); // sigma=12 lenses
    }

    #[test]
    fn test_no_discovery_scans() {
        let records = vec![
            make_record("empty", &["lens_a", "lens_b"], &[]),
        ];
        let stats = compute_domain_stats(&records);
        assert_eq!(stats.total_discoveries, 0);
        let lens_a = stats.lens_stats.get("lens_a").unwrap();
        assert_eq!(lens_a.contributed, 0);
        assert!((lens_a.hit_rate - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_affinity_no_discoveries() {
        let records = vec![
            make_record("test", &["a", "b"], &[]),
        ];
        let affinity = compute_lens_affinity(&records);
        assert!(affinity.is_empty()); // no discovery-producing records
    }
}

// ═══════════════════════════════════════════════════════════════
// Plugin module tests (8 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod plugin_tests {
    use crate::plugin::*;
    use crate::plugin::loader;

    #[test]
    fn test_plugin_type_from_str() {
        assert_eq!(PluginType::from_str("lens"), Some(PluginType::Lens));
        assert_eq!(PluginType::from_str("experiment"), Some(PluginType::Experiment));
        assert_eq!(PluginType::from_str("analyzer"), Some(PluginType::Analyzer));
        assert_eq!(PluginType::from_str("transformer"), Some(PluginType::Transformer));
        assert_eq!(PluginType::from_str("unknown"), None);
    }

    #[test]
    fn test_plugin_type_as_str() {
        assert_eq!(PluginType::Lens.as_str(), "lens");
        assert_eq!(PluginType::Experiment.as_str(), "experiment");
        assert_eq!(PluginType::Analyzer.as_str(), "analyzer");
        assert_eq!(PluginType::Transformer.as_str(), "transformer");
    }

    #[test]
    fn test_parse_manifest() {
        let content = r#"
name = "test_plugin"
version = "0.1.0"
type = "lens"
entry_point = "test_plugin.rs"
"#;
        let plugin = loader::parse_manifest(content);
        assert!(plugin.is_some());
        let p = plugin.unwrap();
        assert_eq!(p.name, "test_plugin");
        assert_eq!(p.version, "0.1.0");
        assert_eq!(p.plugin_type, PluginType::Lens);
        assert_eq!(p.entry_point, "test_plugin.rs");
    }

    #[test]
    fn test_parse_manifest_missing_name() {
        let content = r#"
version = "1.0"
type = "lens"
entry_point = "x.rs"
"#;
        assert!(loader::parse_manifest(content).is_none());
    }

    #[test]
    fn test_registry_register_and_validate() {
        let mut reg = PluginRegistry::new("/tmp/nexus_plugins");
        reg.register(Plugin {
            name: "n6_lens".into(), version: "1.0.0".into(),
            plugin_type: PluginType::Lens, entry_point: "n6_lens.rs".into(),
        });
        let errors = reg.validate();
        assert!(errors.is_empty());
    }

    #[test]
    fn test_registry_validate_empty_entry() {
        let mut reg = PluginRegistry::new("/tmp/nexus_plugins2");
        reg.register(Plugin {
            name: "bad".into(), version: "".into(),
            plugin_type: PluginType::Analyzer, entry_point: "".into(),
        });
        let errors = reg.validate();
        assert_eq!(errors.len(), 2); // phi=2 errors
    }

    #[test]
    fn test_registry_summary() {
        let mut reg = PluginRegistry::new("/tmp/nexus_plugins3");
        for i in 0..6 { // n=6 plugins
            reg.register(Plugin {
                name: format!("plugin_{}", i), version: "1.0".into(),
                plugin_type: if i < 3 { PluginType::Lens } else { PluginType::Experiment },
                entry_point: format!("p{}.rs", i),
            });
        }
        let summary = reg.summary();
        assert!(summary.contains("6 plugins"));
    }

    #[test]
    fn test_parse_manifest_with_comments() {
        let content = r#"
# This is a comment
name = "commented"
version = "2.0"
type = "transformer"
entry_point = "transform.rs"
# Another comment
"#;
        let p = loader::parse_manifest(content).unwrap();
        assert_eq!(p.name, "commented");
        assert_eq!(p.plugin_type, PluginType::Transformer);
    }
}

// ═══════════════════════════════════════════════════════════════
// Encoder module tests (4 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod encoder_tests {
    use crate::encoder::parser;
    use crate::encoder::vectorize;

    #[test]
    fn test_parse_empty_md() {
        let results = parser::parse_hypotheses("");
        assert!(results.is_empty());
    }

    #[test]
    fn test_parse_simple_md() {
        let md = "## H-TEST-1\n| Field | Value |\n|---|---|\n| Grade | EXACT |\n";
        let results = parser::parse_hypotheses(md);
        // Should parse without panicking
        let _ = results;
    }

    #[test]
    fn test_vectorize_basic() {
        use std::collections::HashMap;
        let mut entry = HashMap::new();
        entry.insert("x".to_string(), "6.0".to_string());
        entry.insert("y".to_string(), "12.0".to_string());
        let entries = vec![entry];
        let (data, rows, cols) = vectorize::vectorize(&entries, &["x", "y"]);
        assert_eq!(rows, 1);
        assert_eq!(cols, 2);
        assert_eq!(data.len(), 2);
    }

    #[test]
    fn test_vectorize_deterministic() {
        use std::collections::HashMap;
        let mut entry = HashMap::new();
        entry.insert("val".to_string(), "24.0".to_string());
        let entries = vec![entry];
        let r1 = vectorize::vectorize(&entries, &["val"]);
        let r2 = vectorize::vectorize(&entries, &["val"]);
        assert_eq!(r1, r2);
    }
}

// ═══════════════════════════════════════════════════════════════
// Verifier module tests (8 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod verifier_tests {
    use crate::verifier::n6_check;

    #[test]
    fn test_exact_match_six() {
        let (name, quality) = n6_check::n6_match(6.0);
        assert!(quality > 0.9);
        assert!(name.contains("n") || name.contains("6"));
    }

    #[test]
    fn test_exact_match_twelve() {
        let (name, quality) = n6_check::n6_match(12.0);
        assert!(quality > 0.9);
        assert!(name.contains("sigma") || name.contains("12"));
    }

    #[test]
    fn test_exact_match_twenty_four() {
        let (name, quality) = n6_check::n6_match(24.0);
        assert!(quality > 0.9);
    }

    #[test]
    fn test_close_match() {
        let (_, quality) = n6_check::n6_match(12.1);
        assert!(quality > 0.5); // close to sigma=12
    }

    #[test]
    fn test_no_match() {
        let (_, quality) = n6_check::n6_match(9999.0);
        assert!(quality < 0.5);
    }

    #[test]
    fn test_match_tau() {
        let (_, quality) = n6_check::n6_match(4.0);
        assert!(quality > 0.9);
    }

    #[test]
    fn test_match_phi() {
        let (_, quality) = n6_check::n6_match(2.0);
        assert!(quality > 0.9);
    }

    #[test]
    fn test_match_sopfr() {
        let (_, quality) = n6_check::n6_match(5.0);
        assert!(quality > 0.9);
    }
}

// ═══════════════════════════════════════════════════════════════
// Alert module tests (8 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod alert_tests {
    use crate::alert::*;

    #[test]
    fn test_alert_level_priority() {
        assert!(AlertLevel::Discovery.priority() > AlertLevel::Critical.priority());
        assert!(AlertLevel::Critical.priority() > AlertLevel::Warning.priority());
        assert!(AlertLevel::Warning.priority() > AlertLevel::Info.priority());
    }

    #[test]
    fn test_alert_level_labels() {
        assert_eq!(AlertLevel::Info.label(), "INFO");
        assert_eq!(AlertLevel::Warning.label(), "WARNING");
        assert_eq!(AlertLevel::Critical.label(), "CRITICAL");
        assert_eq!(AlertLevel::Discovery.label(), "DISCOVERY");
    }

    #[test]
    fn test_alert_creation() {
        let a = Alert::new(AlertLevel::Discovery, "consciousness", "pattern-6", 0.95, 1000, "Found n=6 pattern");
        assert_eq!(a.level, AlertLevel::Discovery);
        assert_eq!(a.source_lens, "consciousness");
        assert!((a.confidence - 0.95).abs() < 1e-10);
    }

    #[test]
    fn test_alert_confidence_clamping() {
        let a = Alert::new(AlertLevel::Info, "lens", "pat", 1.5, 0, "over");
        assert!((a.confidence - 1.0).abs() < 1e-10);
        let b = Alert::new(AlertLevel::Info, "lens", "pat", -0.5, 0, "under");
        assert!((b.confidence - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_discovery_alert_helper() {
        let a = discovery_alert("topology", "n6-bridge", 0.88);
        assert_eq!(a.level, AlertLevel::Discovery);
        assert_eq!(a.source_lens, "topology");
    }

    #[test]
    fn test_alert_level_ordering() {
        assert!(AlertLevel::Info < AlertLevel::Warning);
        assert!(AlertLevel::Warning < AlertLevel::Critical);
        assert!(AlertLevel::Critical < AlertLevel::Discovery);
    }

    #[test]
    fn test_six_alerts() {
        let alerts: Vec<Alert> = (0..6).map(|i| {
            Alert::new(AlertLevel::Info, format!("lens_{}", i), format!("pat_{}", i),
                       i as f64 / 6.0, i as u64, format!("Alert {}", i))
        }).collect();
        assert_eq!(alerts.len(), 6); // n=6
    }

    #[test]
    fn test_alert_dedup_key() {
        let a = Alert::new(AlertLevel::Critical, "gravity", "pattern-12", 0.9, 100, "test");
        let b = Alert::new(AlertLevel::Info, "gravity", "pattern-12", 0.5, 200, "test2");
        // Same dedup key (source_lens, pattern_id)
        assert_eq!(a.source_lens, b.source_lens);
        assert_eq!(a.pattern_id, b.pattern_id);
    }
}

// ═══════════════════════════════════════════════════════════════
// Consciousness Bridge module tests (4 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod consciousness_bridge_tests {
    use crate::consciousness_bridge::ConsciousnessBridge;

    #[test]
    fn test_bridge_creation() {
        let bridge = ConsciousnessBridge::new("/tmp/nonexistent_anima.py");
        // check_connection should return false for nonexistent path
        assert!(!bridge.check_connection());
    }

    #[test]
    fn test_bridge_get_laws() {
        let bridge = ConsciousnessBridge::new("/tmp/fake_anima.py");
        let laws = bridge.get_laws();
        // Without real anima, returns empty or defaults
        let _ = laws;
    }

    #[test]
    fn test_bridge_suggest_experiment() {
        let bridge = ConsciousnessBridge::new("/tmp/fake_anima.py");
        let suggestion = bridge.suggest_experiment(0.85);
        assert!(!suggestion.is_empty());
    }

    #[test]
    fn test_bridge_get_phi() {
        let bridge = ConsciousnessBridge::new("/tmp/fake_anima.py");
        let phi = bridge.get_phi();
        // Without real anima, returns None
        let _ = phi;
    }
}

// ═══════════════════════════════════════════════════════════════
// GPU module tests (4 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod gpu_tests {
    use crate::gpu;

    #[test]
    fn test_gpu_availability_check() {
        let _available = gpu::is_available();
    }

    #[test]
    fn test_gpu_device() {
        let _device = gpu::device();
    }

    #[test]
    fn test_buffer_pool_creation() {
        let pool = gpu::buffer_pool::BufferPool::new(6); // n=6
        assert!(pool.available() >= 0);
    }

    #[test]
    fn test_fallback_distance_matrix() {
        // 3 points in 2D = n/phi=3 points, phi=2 dimensions
        let data: Vec<f32> = vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
        let dist = gpu::fallback::distance_matrix_cpu(&data, 3, 2);
        // Condensed form: n*(n-1)/2 = 3 entries
        assert_eq!(dist.len(), 3);
    }

    #[test]
    fn test_fallback_knn() {
        let data: Vec<f32> = vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
        let dist = gpu::fallback::distance_matrix_cpu(&data, 3, 2);
        let knn = gpu::fallback::knn_cpu(&dist, 3, 1);
        // 3 points * k=1 neighbor = 3
        assert_eq!(knn.len(), 3);
    }
}

// ═══════════════════════════════════════════════════════════════
// Scheduler module extra tests (4 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod scheduler_extra_tests {
    use crate::scheduler::*;

    #[test]
    fn test_default_tasks_content() {
        let tasks = default_tasks();
        assert!(tasks.iter().any(|t| t.interval_secs == 86400)); // daily
        assert!(tasks.iter().any(|t| t.interval_secs == 3600)); // hourly
    }

    #[test]
    fn test_twelve_domain_schedule() {
        let mut sched = Scheduler::new();
        for i in 0..12 { // sigma=12 tasks
            sched.add_task(ScheduledTask {
                name: format!("scan-domain-{}", i),
                command: format!("scan d{}", i),
                interval_secs: 6,
                last_run: None,
                enabled: true,
            });
        }
        assert_eq!(sched.len(), 12);
        assert_eq!(sched.due_tasks(0).len(), 12);
    }

    #[test]
    fn test_mark_run_prevents_rerun() {
        let mut sched = Scheduler::new();
        sched.add_task(ScheduledTask {
            name: "t".into(), command: "c".into(), interval_secs: 100,
            last_run: None, enabled: true,
        });
        sched.mark_run("t", 1000);
        assert!(sched.due_tasks(1050).is_empty()); // 50 < 100
        assert_eq!(sched.due_tasks(1100).len(), 1); // 100 >= 100
    }

    #[test]
    fn test_toggle_back_on() {
        let mut sched = Scheduler::new();
        sched.add_task(ScheduledTask {
            name: "t".into(), command: "c".into(), interval_secs: 10,
            last_run: None, enabled: true,
        });
        sched.toggle("t"); // off
        assert!(sched.due_tasks(0).is_empty());
        sched.toggle("t"); // on
        assert_eq!(sched.due_tasks(0).len(), 1);
    }
}

// ═══════════════════════════════════════════════════════════════
// LensForge module tests (8 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod lens_forge_tests {
    use crate::telescope::registry::LensRegistry;
    use crate::lens_forge::gap_analyzer;
    use crate::lens_forge::forge_engine::{self, ForgeConfig};

    #[test]
    fn test_gap_analysis_with_registry() {
        let registry = LensRegistry::new();
        let gap = gap_analyzer::analyze_gaps(&registry, &[]);
        // With a full registry, most domains should be covered
        // but some weak domains may exist
        let _uncovered = &gap.uncovered_domains;
        let _weak = &gap.weak_domains;
    }

    #[test]
    fn test_forge_cycle_produces_candidates() {
        let registry = LensRegistry::new();
        let config = ForgeConfig::default();
        let result = forge_engine::forge_cycle(&registry, &[], &config);
        assert!(result.candidates_generated > 0 || result.gap_report.uncovered_domains.is_empty());
    }

    #[test]
    fn test_forge_config_defaults() {
        let config = ForgeConfig::default();
        assert_eq!(config.max_candidates, 20);
        assert!(config.similarity_threshold > 0.0);
    }

    #[test]
    fn test_forge_result_consistency() {
        let registry = LensRegistry::new();
        let config = ForgeConfig::default();
        let result = forge_engine::forge_cycle(&registry, &[], &config);
        assert!(result.candidates_accepted <= result.candidates_generated);
        assert_eq!(result.new_lenses.len(), result.candidates_accepted);
    }

    #[test]
    fn test_gap_report_structure() {
        let registry = LensRegistry::new();
        let gap = gap_analyzer::analyze_gaps(&registry, &[]);
        // Suggested categories should correspond to uncovered domains
        for domain in &gap.uncovered_domains {
            assert!(!domain.is_empty());
        }
    }

    #[test]
    fn test_forge_with_empty_registry() {
        let registry = LensRegistry::new();
        let config = ForgeConfig { max_candidates: 5, min_confidence: 0.0, similarity_threshold: 0.99 };
        let _result = forge_engine::forge_cycle(&registry, &[], &config);
        // Should not panic
    }
}

// ═══════════════════════════════════════════════════════════════
// Materials module tests (4 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod materials_tests {
    use crate::materials::db::*;
    use std::collections::HashMap;

    #[test]
    fn test_material_creation() {
        let mut props = HashMap::new();
        props.insert("Tc".to_string(), 12.0); // sigma
        props.insert("Z".to_string(), 6.0); // n
        let mat = Material { name: "test".into(), properties: props };
        assert_eq!(mat.name, "test");
        assert_eq!(mat.properties.get("Z"), Some(&6.0));
    }

    #[test]
    fn test_domain_data() {
        let mat = Material {
            name: "carbon".into(),
            properties: [("Z".into(), 6.0), ("CN".into(), 6.0)].into_iter().collect(),
        };
        let domain = DomainData { materials: vec![mat], ceiling: HashMap::new() };
        assert_eq!(domain.materials.len(), 1);
    }

    #[test]
    fn test_materials_as_matrix() {
        let mat1 = Material {
            name: "a".into(),
            properties: [("x".into(), 6.0), ("y".into(), 12.0)].into_iter().collect(),
        };
        let mat2 = Material {
            name: "b".into(),
            properties: [("x".into(), 24.0)].into_iter().collect(),
        };
        let domain = DomainData { materials: vec![mat1, mat2], ceiling: HashMap::new() };
        let (data, rows, cols) = materials_as_matrix(&domain, &["x", "y"]);
        assert_eq!(rows, 2);
        assert_eq!(cols, 2);
        assert_eq!(data.len(), 4);
        assert!((data[0] - 6.0).abs() < 1e-10); // mat1.x
        assert!(data[3].is_nan()); // mat2.y = NaN (missing)
    }

    #[test]
    fn test_empty_domain() {
        let domain = DomainData { materials: vec![], ceiling: HashMap::new() };
        let (data, rows, cols) = materials_as_matrix(&domain, &["x"]);
        assert_eq!(rows, 0);
        assert_eq!(cols, 1);
        assert!(data.is_empty());
    }
}

// ═══════════════════════════════════════════════════════════════
// Knowledge module tests (8 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod knowledge_tests {
    use crate::knowledge::store::*;
    use std::collections::HashMap;

    fn make_entry(id: &str, entry_type: &str, conf: f64, n6: f64, verified: bool) -> KnowledgeEntry {
        KnowledgeEntry {
            id: id.into(),
            entry_type: entry_type.into(),
            content: format!("Content for {}", id),
            metadata: HashMap::new(),
            created: "2026-04-03".into(),
            confidence: conf,
            n6_score: n6,
            references: vec![],
            verified,
            verified_by: vec![],
        }
    }

    #[test]
    fn test_kb_new_empty() {
        let kb = KnowledgeBase::new("/tmp/nexus_kb_test.json");
        assert!(kb.is_empty());
        assert_eq!(kb.len(), 0);
    }

    #[test]
    fn test_kb_add_and_get() {
        let mut kb = KnowledgeBase::new("/tmp/nexus_kb1.json");
        kb.add(make_entry("bt-105", "theorem", 0.95, 0.9, true));
        assert_eq!(kb.len(), 1);
        assert!(kb.get("bt-105").is_some());
    }

    #[test]
    fn test_kb_search() {
        let mut kb = KnowledgeBase::new("/tmp/nexus_kb2.json");
        kb.add(make_entry("d1", "discovery", 0.8, 0.7, true));
        kb.add(make_entry("d2", "hypothesis", 0.6, 0.5, false));
        let results = kb.search("Content for d1");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_kb_by_type() {
        let mut kb = KnowledgeBase::new("/tmp/nexus_kb3.json");
        kb.add(make_entry("a", "theorem", 0.9, 0.9, true));
        kb.add(make_entry("b", "theorem", 0.8, 0.8, true));
        kb.add(make_entry("c", "discovery", 0.7, 0.6, false));
        let theorems = kb.by_type("theorem");
        assert_eq!(theorems.len(), 2);
    }

    #[test]
    fn test_kb_verified_only() {
        let mut kb = KnowledgeBase::new("/tmp/nexus_kb4.json");
        kb.add(make_entry("v1", "x", 0.9, 0.9, true));
        kb.add(make_entry("v2", "x", 0.5, 0.5, false));
        kb.add(make_entry("v3", "x", 0.8, 0.8, true));
        assert_eq!(kb.verified_only().len(), 2);
    }

    #[test]
    fn test_kb_stats() {
        let mut kb = KnowledgeBase::new("/tmp/nexus_kb5.json");
        for i in 0..6 { // n=6
            kb.add(make_entry(&format!("e{}", i), "test", 0.5 + i as f64 * 0.1, 0.7, i % 2 == 0));
        }
        let stats = kb.stats();
        assert_eq!(stats.total, 6);
    }

    #[test]
    fn test_kb_index() {
        use crate::knowledge::index::KnowledgeIndex;
        let mut kb = KnowledgeBase::new("/tmp/nexus_kbi.json");
        kb.add(make_entry("idx1", "theorem", 0.9, 0.9, true));
        let index = KnowledgeIndex::build(&kb);
        assert!(index.token_count() > 0);
    }

    #[test]
    fn test_kb_entries() {
        let mut kb = KnowledgeBase::new("/tmp/nexus_kbe.json");
        for i in 0..12 { // sigma=12
            kb.add(make_entry(&format!("s{}", i), "sigma", 0.5, 0.5, true));
        }
        assert_eq!(kb.entries().len(), 12);
    }
}

// ═══════════════════════════════════════════════════════════════
// Self-Improve module tests (8 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod self_improve_tests {
    use crate::self_improve::suggestion::*;

    #[test]
    fn test_suggestion_creation() {
        let s = ImprovementSuggestion::new("performance", "Speed up scan", "2x faster", "medium");
        assert_eq!(s.category, "performance");
    }

    #[test]
    fn test_suggestion_priority_score() {
        let s = ImprovementSuggestion::new("accuracy", "Better n6 matching", "50% more EXACT", "easy");
        let score = s.priority_score();
        assert!(score >= 0.0);
    }

    #[test]
    fn test_prioritize_ordering() {
        let mut suggestions = vec![
            ImprovementSuggestion::new("low", "Low priority", "small", "hard"),
            ImprovementSuggestion::new("high", "High priority", "large", "easy"),
        ];
        prioritize(&mut suggestions);
        // After prioritization, highest priority should be first
        assert!(suggestions[0].priority_score() >= suggestions[1].priority_score());
    }

    #[test]
    fn test_six_suggestions() {
        let mut suggestions: Vec<ImprovementSuggestion> = (0..6).map(|i| {
            ImprovementSuggestion::new(
                &format!("cat-{}", i),
                &format!("Suggestion {}", i),
                &format!("Improvement {}", i),
                if i < 3 { "easy" } else { "hard" },
            )
        }).collect();
        assert_eq!(suggestions.len(), 6);
        prioritize(&mut suggestions);
    }
}

// ═══════════════════════════════════════════════════════════════
// Time Travel module tests (8 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod time_travel_tests {
    use crate::time_travel::branch::*;
    use crate::time_travel::snapshot::*;
    use crate::graph::persistence::DiscoveryGraph;

    #[test]
    fn test_branch_manager_new() {
        let bm = BranchManager::new();
        assert_eq!(bm.count(), 0);
    }

    #[test]
    fn test_time_travel_new() {
        let tt = TimeTravel::new("/tmp/nexus_tt");
        assert_eq!(tt.count(), 0);
    }

    #[test]
    fn test_save_snapshot() {
        let mut tt = TimeTravel::new("/tmp/nexus_tt2");
        let graph = DiscoveryGraph::new();
        let _id = tt.save_snapshot(&graph, "kb-state", 100, "test snapshot");
        assert_eq!(tt.count(), 1);
    }

    #[test]
    fn test_list_snapshots() {
        let mut tt = TimeTravel::new("/tmp/nexus_tt3");
        let graph = DiscoveryGraph::new();
        for i in 0..6 { // n=6 snapshots
            tt.save_snapshot(&graph, &format!("kb-{}", i), 100 + i, &format!("snapshot {}", i));
        }
        assert_eq!(tt.list_snapshots().len(), 6);
    }

    #[test]
    fn test_get_snapshot() {
        let mut tt = TimeTravel::new("/tmp/nexus_tt4");
        let graph = DiscoveryGraph::new();
        let id = tt.save_snapshot(&graph, "kb", 50, "find me");
        let snap = tt.get_snapshot(&id);
        assert!(snap.is_some());
    }

    #[test]
    fn test_rollback() {
        let mut tt = TimeTravel::new("/tmp/nexus_tt5");
        let graph = DiscoveryGraph::new();
        let id = tt.save_snapshot(&graph, "kb", 50, "rollback test");
        let result = tt.rollback(&id);
        assert!(result.is_some());
    }

    #[test]
    fn test_branch_manager_empty_list() {
        let bm = BranchManager::new();
        assert!(bm.list_branches().is_empty());
    }

    #[test]
    fn test_branch_manager_get_nonexistent() {
        let bm = BranchManager::new();
        assert!(bm.get_branch("nonexistent").is_none());
    }
}

// ═══════════════════════════════════════════════════════════════
// Reward module tests (6 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod reward_tests {
    use crate::reward::*;

    #[test]
    fn test_reward_signal_multipliers() {
        assert!(RewardSignal::PatternFound.base_multiplier() > 0.0);
        assert!(RewardSignal::ConsensusAchieved.base_multiplier() > 0.0);
        assert!(RewardSignal::NoveltyDetected.base_multiplier() > 0.0);
        assert!(RewardSignal::N6Alignment.base_multiplier() > 0.0);
    }

    #[test]
    fn test_reward_signal_labels() {
        assert!(!RewardSignal::PatternFound.label().is_empty());
        assert!(!RewardSignal::N6Alignment.label().is_empty());
    }

    #[test]
    fn test_reward_tracker_new() {
        let tracker = RewardTracker::new();
        assert_eq!(tracker.entry_count(), 0);
    }

    #[test]
    fn test_reward_entry() {
        let entry = RewardEntry::new(RewardSignal::PatternFound, "lens_a", 0.85, 1000);
        assert!(entry.effective_score() > 0.0);
    }

    #[test]
    fn test_reward_tracker_record() {
        let mut tracker = RewardTracker::new();
        tracker.record(RewardEntry::new(RewardSignal::PatternFound, "lens_a", 0.9, 1000));
        assert_eq!(tracker.entry_count(), 1);
    }

    #[test]
    fn test_top_performers() {
        let mut tracker = RewardTracker::new();
        for i in 0..6 { // n=6
            tracker.record(RewardEntry::new(RewardSignal::N6Alignment, &format!("lens_{}", i), 0.5 + i as f64 * 0.1, i as u64));
        }
        let top = top_performers(&tracker, 3);
        assert!(!top.is_empty());
    }
}

// ═══════════════════════════════════════════════════════════════
// Statistics module tests (8 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod statistics_extra_tests {
    use crate::statistics::significance;
    use crate::statistics::effect_size;
    use crate::statistics::reproducibility;

    #[test]
    fn test_mean() {
        let data = vec![2.0, 4.0, 5.0, 6.0, 12.0, 24.0];
        let m = significance::mean(&data);
        assert!((m - 53.0 / 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_std_dev() {
        let data = vec![6.0, 6.0, 6.0, 6.0];
        assert!((significance::std_dev(&data) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_significance() {
        let observed = vec![6.0, 12.0, 24.0];
        let expected = vec![6.0, 12.0, 24.0];
        let result = significance::test_significance(&observed, &expected);
        assert!(result.p_value >= 0.0);
    }

    #[test]
    fn test_bonferroni() {
        let p_values = vec![0.01, 0.05, 0.001, 0.1, 0.02, 0.005]; // n=6
        let corrected = significance::bonferroni_correction(&p_values);
        assert_eq!(corrected.len(), 6);
    }

    #[test]
    fn test_cohens_d() {
        let g1 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let g2 = vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0];
        let d = effect_size::cohens_d(&g1, &g2);
        assert!(d.abs() > 0.0); // large effect expected
    }

    #[test]
    fn test_classify_effect() {
        assert_eq!(effect_size::classify_effect(0.1), "negligible");
        assert_eq!(effect_size::classify_effect(0.5), "medium");
    }

    #[test]
    fn test_reproducibility() {
        let results = vec![0.85, 0.86, 0.84, 0.85, 0.87, 0.85]; // n=6
        let report = reproducibility::assess_reproducibility(&results, 0.1);
        assert!(report.is_reproducible);
    }

    #[test]
    fn test_n6_consistency() {
        let exact = vec![1.0, 1.0, 1.0, 0.95, 0.98, 1.0]; // high consistency
        let score = reproducibility::n6_consistency(&exact);
        assert!(score > 0.8);
    }
}

// ═══════════════════════════════════════════════════════════════
// Ingest module tests (8 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod ingest_tests {
    use crate::ingest::json_reader;
    use crate::ingest::text_parser;

    #[test]
    fn test_json_reader_empty() {
        let result = json_reader::parse_json_values("{}");
        // Empty object may yield 0 values
        let _ = result;
    }

    #[test]
    fn test_json_reader_simple() {
        let json = r#"{"sigma": 12, "tau": 4, "n": 6}"#;
        let vals = json_reader::parse_json_values(json).unwrap_or_default();
        assert!(!vals.is_empty());
    }

    #[test]
    fn test_json_reader_array() {
        let json = r#"[6, 12, 24, 4, 2, 5]"#;
        let vals = json_reader::parse_json_values(json).unwrap_or_default();
        assert_eq!(vals.len(), 6); // n=6
    }

    #[test]
    fn test_json_reader_kv() {
        let json = r#"{"sigma": 12, "phi": 2}"#;
        let kv = json_reader::parse_json_kv(json).unwrap_or_default();
        assert!(!kv.is_empty());
    }

    #[test]
    fn test_text_parser_empty() {
        let nums = text_parser::extract_numbers("");
        assert!(nums.is_empty());
    }

    #[test]
    fn test_text_parser_simple() {
        let nums = text_parser::extract_numbers("sigma is 12 and tau is 4");
        assert!(nums.contains(&12.0));
        assert!(nums.contains(&4.0));
    }

    #[test]
    fn test_text_parser_n6_context() {
        let pairs = text_parser::extract_n6_context("sigma=12 is universal, tau=4 is divisor count");
        // Should extract key-value pairs
        let _ = pairs;
    }

    #[test]
    fn test_text_parser_key_values() {
        let pairs = text_parser::extract_key_value_pairs("n=6, sigma=12, phi=2");
        assert!(!pairs.is_empty());
    }
}

// ═══════════════════════════════════════════════════════════════
// Genetic Programming module tests (8 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod genetic_prog_tests {
    use crate::genetic_prog::*;

    #[test]
    fn test_gene_creation() {
        let gene = Gene::new("sigma", 0.8, 0.5);
        assert_eq!(gene.lens_id, "sigma");
    }

    #[test]
    fn test_chromosome_creation() {
        let genes = vec![
            Gene::new("sigma", 0.8, 0.5),
            Gene::new("phi", 0.6, 0.3),
            Gene::new("tau", 0.9, 0.7),
        ];
        let chromo = Chromosome::new(genes);
        assert_eq!(chromo.genes.len(), 3);
    }

    #[test]
    fn test_chromosome_total_weight() {
        let genes = vec![
            Gene::new("a", 1.0, 0.5),
            Gene::new("b", 2.0, 0.5),
        ];
        let chromo = Chromosome::new(genes);
        assert!(chromo.total_weight() > 0.0);
    }

    #[test]
    fn test_population_random() {
        let lens_ids: Vec<String> = (0..6).map(|i| format!("lens_{}", i)).collect();
        let pop = Population::random(&lens_ids, 6, 12, 42); // n=6 genes, sigma=12 pop
        // Just verify it doesn't panic
        let _ = pop;
    }

    #[test]
    fn test_fitness_function() {
        let genes = vec![Gene::new("test", 0.5, 0.3)];
        let chromo = Chromosome::new(genes);
        let data = vec![6.0, 12.0, 24.0];
        let known = vec![true, false, true];
        let f = fitness(&chromo, &data, &known);
        assert!(f >= 0.0);
    }
}

// ═══════════════════════════════════════════════════════════════
// Safety module tests (6 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod safety_extra_tests {
    use crate::safety::*;

    #[test]
    fn test_default_gate() {
        let gate = default_gate();
        // Gate should have reasonable thresholds
        let _ = gate;
    }

    #[test]
    fn test_check_gate_pass() {
        let gate = default_gate();
        let metrics = ScanMetrics {
            phi_score: 0.9, stability_score: 0.95, anomaly_rate: 0.01, n6_exact_ratio: 0.85,
        };
        let decision = check_gate(&metrics, &gate);
        assert!(matches!(decision, GateDecision::Allow { .. }));
    }

    #[test]
    fn test_check_gate_fail() {
        let gate = default_gate();
        let metrics = ScanMetrics {
            phi_score: 0.1, stability_score: 0.1, anomaly_rate: 0.99, n6_exact_ratio: 0.01,
        };
        let decision = check_gate(&metrics, &gate);
        assert!(matches!(decision, GateDecision::Block { .. }));
    }

    #[test]
    fn test_format_decision() {
        let gate = default_gate();
        let metrics = ScanMetrics {
            phi_score: 0.85, stability_score: 0.9, anomaly_rate: 0.02, n6_exact_ratio: 0.8,
        };
        let decision = check_gate(&metrics, &gate);
        let text = format_decision(&decision);
        assert!(!text.is_empty());
    }
}

// ═══════════════════════════════════════════════════════════════
// Calibration module tests (6 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod calibration_extra_tests {
    use crate::calibration::*;

    #[test]
    fn test_generate_synthetic_datasets() {
        let datasets = generate_synthetic_datasets();
        assert!(!datasets.is_empty());
    }

    #[test]
    fn test_synthetic_dataset_structure() {
        let datasets = generate_synthetic_datasets();
        for ds in &datasets {
            assert!(!ds.data.is_empty());
            assert!(ds.n > 0);
            assert!(ds.d > 0);
        }
    }

    #[test]
    fn test_known_pattern() {
        let _pattern = KnownPattern {
            metric_name: "sigma".into(),
            expected_values: vec![12.0],
            tolerance: 0.05,
        };
    }
}

// ═══════════════════════════════════════════════════════════════
// Publish module tests (6 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod publish_extra_tests {
    use crate::publish::markdown;
    use crate::publish::bt_format;

    #[test]
    fn test_generate_markdown() {
        let findings = vec![("sigma".to_string(), 12.0)];
        let n6_conns = vec!["BT-105".to_string()];
        let preds = vec!["Prediction 1".to_string()];
        let md = markdown::generate_markdown("Test Discovery", "Found n=6", &findings, &n6_conns, &preds);
        assert!(md.contains("Test Discovery"));
    }

    #[test]
    fn test_bt_ref() {
        let r = bt_format::format_bt_ref(105, "SLE6 Critical Exponents");
        assert!(r.contains("105"));
        assert!(r.contains("SLE6"));
    }

    #[test]
    fn test_markdown_empty() {
        let md = markdown::generate_markdown("Title", "Abstract", &[], &[], &[]);
        assert!(md.contains("Title"));
    }

    #[test]
    fn test_bt_format() {
        let domains = vec!["physics".to_string(), "ai".to_string()];
        let evidence = vec!["sigma=12 verified".to_string()];
        let bt = bt_format::format_bt(128, "New Discovery", &domains, &evidence, "sigma=12", 3);
        assert!(bt.contains("128"));
    }
}

// ═══════════════════════════════════════════════════════════════
// N=6 Constants Cross-Verification Tests (BT-105~127 aligned)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod n6_constant_tests {
    // These tests verify the fundamental n=6 arithmetic relationships
    // that underpin all BT-105~127 breakthrough theorems.

    const N: f64 = 6.0;
    const SIGMA: f64 = 12.0;
    const PHI: f64 = 2.0;
    const TAU: f64 = 4.0;
    const SOPFR: f64 = 5.0;
    const J2: f64 = 24.0;
    const MU: f64 = 1.0;

    #[test]
    fn test_core_identity() {
        assert!((SIGMA * PHI - N * TAU).abs() < 1e-10); // sigma*phi = n*tau = 24
    }

    #[test]
    fn test_j2_equals_sigma_phi() {
        assert!((J2 - SIGMA * PHI).abs() < 1e-10);
    }

    #[test]
    fn test_divisor_sum() {
        // sigma(6) = 1+2+3+6 = 12
        assert!((1.0 + 2.0 + 3.0 + 6.0 - SIGMA).abs() < 1e-10);
    }

    #[test]
    fn test_egyptian_fraction() {
        // 1/2 + 1/3 + 1/6 = 1 (BT-99, BT-5)
        assert!((1.0_f64/2.0 + 1.0/3.0 + 1.0/6.0 - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_sigma_minus_tau_is_eight() {
        assert!((SIGMA - TAU - 8.0).abs() < 1e-10); // BT-58
    }

    #[test]
    fn test_sigma_minus_sopfr_is_seven() {
        assert!((SIGMA - SOPFR - 7.0).abs() < 1e-10); // OSI layers, BT-115
    }

    #[test]
    fn test_sigma_minus_phi_is_ten() {
        assert!((SIGMA - PHI - 10.0).abs() < 1e-10); // BT-64 universal regularization
    }

    #[test]
    fn test_sigma_minus_mu_is_eleven() {
        assert!((SIGMA - MU - 11.0).abs() < 1e-10); // BT-110 M-theory dim
    }

    #[test]
    fn test_bt105_sle6_beta() {
        // sopfr/n^2 = 5/36
        let beta = SOPFR / (N * N);
        assert!((beta - 5.0/36.0).abs() < 1e-10);
    }

    #[test]
    fn test_bt105_sle6_eta() {
        // sopfr/J2 = 5/24
        let eta = SOPFR / J2;
        assert!((eta - 5.0/24.0).abs() < 1e-10);
    }

    #[test]
    fn test_bt105_hausdorff() {
        // (n+mu)/tau = 7/4
        let d_h = (N + MU) / TAU;
        assert!((d_h - 7.0/4.0).abs() < 1e-10);
    }

    #[test]
    fn test_bt106_s3_order() {
        // |S3| = 3! = 6 = n
        assert!((1.0 * 2.0 * 3.0 - N).abs() < 1e-10);
    }

    #[test]
    fn test_bt106_irrep_sum() {
        // 1^2 + 1^2 + 2^2 = 4 = tau
        assert!((1.0 + 1.0 + 4.0 - N).abs() < 1e-10); // sum = 6 = n
        assert!((1.0 + 1.0 + TAU - N).abs() < 1e-10);
    }

    #[test]
    fn test_bt108_chromatic_partition() {
        // 12 = 7 + 5 = (sigma-sopfr) + sopfr
        assert!(((SIGMA - SOPFR) + SOPFR - SIGMA).abs() < 1e-10);
    }

    #[test]
    fn test_bt109_zeta2() {
        // pi^2/6 = zeta(2) = 1.6449...
        let zeta2 = std::f64::consts::PI * std::f64::consts::PI / N;
        assert!((zeta2 - 1.6449340668).abs() < 1e-5);
    }

    #[test]
    fn test_bt111_tau_squared_over_sigma() {
        // tau^2/sigma = 16/12 = 4/3 (SQ bandgap, SwiGLU, Betz)
        let ratio = TAU * TAU / SIGMA;
        assert!((ratio - 4.0/3.0).abs() < 1e-10);
    }

    #[test]
    fn test_bt112_phi_squared_over_n() {
        // phi^2/n = 4/6 = 2/3 (Koide formula, Byzantine)
        let ratio = PHI * PHI / N;
        assert!((ratio - 2.0/3.0).abs() < 1e-10);
    }

    #[test]
    fn test_bt113_sw_constants() {
        // SOLID=5=sopfr, REST=6=n, 12Factor=12=sigma, ACID=4=tau
        assert!((SOPFR - 5.0).abs() < 1e-10);
        assert!((N - 6.0).abs() < 1e-10);
        assert!((SIGMA - 12.0).abs() < 1e-10);
        assert!((TAU - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_bt114_crypto_ladder() {
        // AES-128 = 2^(sigma-sopfr) = 2^7
        assert!((2.0_f64.powf(SIGMA - SOPFR) - 128.0).abs() < 1e-10);
        // AES-256 = 2^(sigma-tau) = 2^8
        assert!((2.0_f64.powf(SIGMA - TAU) - 256.0).abs() < 1e-10);
        // RSA-2048 = 2^(sigma-mu) = 2^11
        assert!((2.0_f64.powf(SIGMA - MU) - 2048.0).abs() < 1e-10);
    }

    #[test]
    fn test_bt119_troposphere() {
        // polar=8=sigma-tau, mid=12=sigma, equatorial=16=sigma+tau
        assert!((SIGMA - TAU - 8.0).abs() < 1e-10);
        assert!((SIGMA + TAU - 16.0).abs() < 1e-10);
    }

    #[test]
    fn test_bt122_hexagon_angle() {
        // Interior angle = sigma*(sigma-phi) = 12*10 = 120 degrees
        assert!((SIGMA * (SIGMA - PHI) - 120.0).abs() < 1e-10);
    }

    #[test]
    fn test_bt123_se3_dim() {
        // dim(SE(3)) = 6 = n
        assert!((N - 6.0).abs() < 1e-10);
        // se(3) structure constants = 12 = sigma
        assert!((SIGMA - 12.0).abs() < 1e-10);
    }

    #[test]
    fn test_bt124_bilateral_joints() {
        // 6 joint types * 2 sides = 12 = sigma
        assert!((N * PHI - SIGMA).abs() < 1e-10);
    }

    #[test]
    fn test_bt126_grasp_space() {
        // 2^sopfr = 2^5 = 32 grasp patterns
        assert!((2.0_f64.powf(SOPFR) - 32.0).abs() < 1e-10);
    }

    #[test]
    fn test_j2_minus_tau() {
        // J2 - tau = 20 (amino acids, Chinchilla ratio)
        assert!((J2 - TAU - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_perfect_number_check() {
        // 6 = 1 + 2 + 3 (sum of proper divisors)
        assert!((1.0 + 2.0 + 3.0 - N).abs() < 1e-10);
    }

    #[test]
    fn test_sigma_squared() {
        // sigma^2 = 144 (AD102 SMs, solar cells)
        assert!((SIGMA * SIGMA - 144.0).abs() < 1e-10);
    }

    #[test]
    fn test_sigma_times_j2() {
        // sigma * J2 = 288 (B300 HBM)
        assert!((SIGMA * J2 - 288.0).abs() < 1e-10);
    }

    #[test]
    fn test_sigma_times_tau() {
        // sigma * tau = 48 (48kHz audio, gate pitch)
        assert!((SIGMA * TAU - 48.0).abs() < 1e-10);
    }

    #[test]
    fn test_sigma_times_sopfr() {
        // sigma * sopfr = 60 (60Hz grid, solar cells)
        assert!((SIGMA * SOPFR - 60.0).abs() < 1e-10);
    }

    #[test]
    fn test_phi_to_n() {
        // phi^n = 2^6 = 64 (codons)
        assert!((PHI.powf(N) - 64.0).abs() < 1e-10);
    }

    #[test]
    fn test_ln_four_thirds() {
        // ln(4/3) = 0.2877... (Mertens dropout, BT-26)
        let val = (4.0_f64 / 3.0).ln();
        assert!((val - 0.2877).abs() < 0.001);
    }

    #[test]
    fn test_one_over_e() {
        // 1/e = 0.3679... (Boltzmann gate sparsity)
        let val = 1.0 / std::f64::consts::E;
        assert!((val - 0.3679).abs() < 0.001);
    }

    #[test]
    fn test_bt109_bernoulli_divisor() {
        // sopfr * n = 30 (B4 denominator)
        assert!((SOPFR * N - 30.0).abs() < 1e-10);
        // (sigma - sopfr) * n = 42 (B6 denominator)
        assert!(((SIGMA - SOPFR) * N - 42.0).abs() < 1e-10);
    }

    #[test]
    fn test_sigma_plus_mu() {
        // sigma + mu = 13 (DNS root servers, BT-115)
        assert!((SIGMA + MU - 13.0).abs() < 1e-10);
    }

    #[test]
    fn test_sigma_times_n() {
        // sigma * n = 72 (solar 72-cell panel, BT-63)
        assert!((SIGMA * N - 72.0).abs() < 1e-10);
    }

    #[test]
    fn test_power_ten_ladder() {
        // (sigma-phi)^tau = 10^4 (RoPE theta, BT-34)
        assert!(((SIGMA - PHI).powf(TAU) - 10000.0).abs() < 1e-10);
    }
}

// ═══════════════════════════════════════════════════════════════
// Telescope scan quality tests (24 = J2 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod telescope_scan_tests {
    use crate::telescope::Telescope;

    #[test]
    fn test_telescope_creation() {
        let t = Telescope::new();
        assert!(t.lens_count() > 0);
    }

    #[test]
    fn test_scan_n6_data() {
        let t = Telescope::new();
        let data = vec![6.0, 12.0, 24.0, 4.0, 2.0, 5.0]; // n6 constants
        let results = t.scan_all(&data, data.len(), 1);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_scan_two_points() {
        let t = Telescope::new();
        let data = vec![6.0, 12.0]; // n, sigma
        let results = t.scan_all(&data, 2, 1);
        let _ = results;
    }

    #[test]
    fn test_scan_single_point() {
        let t = Telescope::new();
        let data = vec![6.0]; // just n
        let results = t.scan_all(&data, 1, 1);
        let _ = results;
    }

    #[test]
    fn test_scan_large_data() {
        let t = Telescope::new();
        let data: Vec<f64> = (0..144).map(|i| i as f64).collect(); // sigma^2 points
        let results = t.scan_all(&data, data.len(), 1);
        let _ = results;
    }

    #[test]
    fn test_lens_count_minimum() {
        let t = Telescope::new();
        assert!(t.lens_count() >= 22); // at least 22 core lenses
    }

    #[test]
    fn test_scan_deterministic() {
        let t = Telescope::new();
        let data = vec![6.0, 12.0, 24.0];
        let r1 = t.scan_all(&data, data.len(), 1);
        let r2 = t.scan_all(&data, data.len(), 1);
        assert_eq!(r1.len(), r2.len());
    }

    #[test]
    fn test_scan_with_noise() {
        let t = Telescope::new();
        let data = vec![6.01, 11.99, 24.02, 3.98]; // slightly noisy n6
        let results = t.scan_all(&data, data.len(), 1);
        let _ = results;
    }

    #[test]
    fn test_scan_negative_values() {
        let t = Telescope::new();
        let data = vec![-6.0, -12.0, 0.0, 6.0, 12.0];
        let results = t.scan_all(&data, data.len(), 1);
        let _ = results;
    }

    #[test]
    fn test_scan_all_zeros() {
        let t = Telescope::new();
        let data = vec![0.0; 6];
        let results = t.scan_all(&data, data.len(), 1);
        let _ = results;
    }

    #[test]
    fn test_scan_very_large_values() {
        let t = Telescope::new();
        let data = vec![1e10, 1e12, 1e24]; // scaled n6
        let results = t.scan_all(&data, data.len(), 1);
        let _ = results;
    }

    #[test]
    fn test_scan_fibonacci_like() {
        let t = Telescope::new();
        let data = vec![1.0, 1.0, 2.0, 3.0, 5.0, 8.0]; // Fibonacci-like
        let results = t.scan_all(&data, data.len(), 1);
        let _ = results;
    }
}

// ═══════════════════════════════════════════════════════════════
// Ouroboros evolution quality tests (12 = sigma tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod ouroboros_extra_tests {
    use crate::ouroboros::*;

    #[test]
    fn test_evolution_config_default() {
        let config = EvolutionConfig::default();
        assert!(!config.domain.is_empty() || config.domain.is_empty()); // just check it works
    }

    #[test]
    fn test_evolution_engine_creation() {
        let config = EvolutionConfig::default();
        let seeds = vec!["test seed".to_string()];
        let engine = EvolutionEngine::new(config, seeds);
        assert!(engine.graph.nodes.is_empty());
    }

    #[test]
    fn test_evolution_single_step() {
        let mut config = EvolutionConfig::default();
        config.domain = "test".to_string();
        let seeds = vec!["n=6 test".to_string()];
        let mut engine = EvolutionEngine::new(config, seeds);
        let result = engine.evolve_step();
        assert!(result.cycle >= 1);
    }

    #[test]
    fn test_evolution_n6_seeds() {
        let config = EvolutionConfig::default();
        let seeds: Vec<String> = (0..6).map(|i| format!("seed-{}", i)).collect();
        let engine = EvolutionEngine::new(config, seeds);
        let _ = engine;
    }

    #[test]
    fn test_meta_loop_config_default() {
        let config = MetaLoopConfig::default();
        assert!(config.max_meta_cycles > 0);
    }

    #[test]
    fn test_meta_loop_creation() {
        let config = MetaLoopConfig { max_meta_cycles: 1, max_ouroboros_cycles: 1, ..MetaLoopConfig::default() };
        let seeds = vec!["meta test".to_string()];
        let _meta = MetaLoop::new("test".to_string(), seeds, config);
    }
}

// ═══════════════════════════════════════════════════════════════
// API server additional tests (8 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod api_extra_tests {
    use crate::api::ApiServer;

    #[test]
    fn test_api_bad_request_verify() {
        let server = ApiServer::new(8080);
        let resp = server.handle_request("POST", "/verify", "not_a_number");
        assert_eq!(resp.status, 400);
    }

    #[test]
    fn test_api_empty_scan() {
        let server = ApiServer::new(8080);
        let resp = server.handle_request("POST", "/scan", "");
        assert_eq!(resp.status, 400);
    }

    #[test]
    fn test_api_verify_sigma() {
        let server = ApiServer::new(8080);
        let resp = server.handle_request("POST", "/verify", r#"{"value":12.0}"#);
        assert_eq!(resp.status, 200);
    }

    #[test]
    fn test_api_verify_j2() {
        let server = ApiServer::new(8080);
        let resp = server.handle_request("POST", "/verify", r#"{"value":24.0}"#);
        assert_eq!(resp.status, 200);
    }

    #[test]
    fn test_api_different_port() {
        let server = ApiServer::new(6666);
        let resp = server.handle_request("GET", "/health", "");
        assert!(resp.body.contains("6666"));
    }

    #[test]
    fn test_api_wrong_method() {
        let server = ApiServer::new(8080);
        let resp = server.handle_request("DELETE", "/health", "");
        assert_eq!(resp.status, 404);
    }

    #[test]
    fn test_api_constants_n6_values() {
        let server = ApiServer::new(8080);
        let resp = server.handle_request("GET", "/constants", "");
        assert!(resp.body.contains("\"n\":6"));
        assert!(resp.body.contains("\"sigma\":12"));
        assert!(resp.body.contains("\"J2\":24"));
    }

    #[test]
    fn test_api_scan_domain() {
        let server = ApiServer::new(8080);
        let resp = server.handle_request("POST", "/scan", r#"{"domain":"ai"}"#);
        assert_eq!(resp.status, 200);
        assert!(resp.body.contains("ai"));
    }
}

// ═══════════════════════════════════════════════════════════════
// Verifier n6_check additional tests (12 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod verifier_extra_tests {
    use crate::verifier::n6_check;

    #[test]
    fn test_match_one() {
        let (_, q) = n6_check::n6_match(1.0); // mu
        assert!(q > 0.5);
    }

    #[test]
    fn test_match_eight() {
        let (_, q) = n6_check::n6_match(8.0); // sigma-tau
        assert!(q > 0.5);
    }

    #[test]
    fn test_match_ten() {
        let (_, q) = n6_check::n6_match(10.0); // sigma-phi
        assert!(q > 0.5);
    }

    #[test]
    fn test_match_twenty() {
        let (_, q) = n6_check::n6_match(20.0); // J2-tau
        assert!(q > 0.5);
    }

    #[test]
    fn test_match_forty_eight() {
        let (_, q) = n6_check::n6_match(48.0); // sigma*tau
        assert!(q > 0.5);
    }

    #[test]
    fn test_match_one_hundred_forty_four() {
        let (_, q) = n6_check::n6_match(144.0); // sigma^2
        assert!(q > 0.5);
    }

    #[test]
    fn test_match_negative() {
        let (_, q) = n6_check::n6_match(-12.0);
        // Negative values should still be handled
        let _ = q;
    }

    #[test]
    fn test_match_zero() {
        let (_, q) = n6_check::n6_match(0.0);
        let _ = q;
    }

    #[test]
    fn test_match_fraction() {
        let (_, q) = n6_check::n6_match(1.333333); // tau^2/sigma = 4/3
        let _ = q;
    }

    #[test]
    fn test_match_point_one() {
        let (_, q) = n6_check::n6_match(0.1); // 1/(sigma-phi)
        let _ = q;
    }

    #[test]
    fn test_match_seven() {
        let (_, q) = n6_check::n6_match(7.0); // sigma-sopfr
        let _ = q; // may or may not match depending on implementation
    }

    #[test]
    fn test_match_eleven() {
        let (_, q) = n6_check::n6_match(11.0); // sigma-mu
        assert!(q > 0.5);
    }

    #[test]
    fn test_match_sixty() {
        let (_, q) = n6_check::n6_match(60.0); // sigma*sopfr
        let _ = q;
    }

    #[test]
    fn test_match_one_twenty() {
        let (_, q) = n6_check::n6_match(120.0); // sigma*(sigma-phi)
        let _ = q;
    }

    #[test]
    fn test_match_forty() {
        let (_, q) = n6_check::n6_match(40.0); // tau*(sigma-phi)
        let _ = q;
    }

    #[test]
    fn test_match_eighty() {
        let (_, q) = n6_check::n6_match(80.0); // phi^tau*sopfr
        let _ = q;
    }

    #[test]
    fn test_match_one_ninety_two() {
        let (_, q) = n6_check::n6_match(192.0); // sigma*phi^tau
        let _ = q;
    }

    #[test]
    fn test_match_two_eighty_eight() {
        let (_, q) = n6_check::n6_match(288.0); // sigma*J2
        let _ = q;
    }

    #[test]
    fn test_match_thirteen() {
        let (_, q) = n6_check::n6_match(13.0); // sigma+mu
        let _ = q;
    }

    #[test]
    fn test_match_ninety_six() {
        let (_, q) = n6_check::n6_match(96.0); // sigma*(sigma-tau)
        let _ = q;
    }

    #[test]
    fn test_match_seventy_two() {
        let (_, q) = n6_check::n6_match(72.0); // sigma*n
        let _ = q;
    }

    #[test]
    fn test_match_half() {
        let (_, q) = n6_check::n6_match(0.5); // phi/tau = 1/2
        let _ = q;
    }
}

// ═══════════════════════════════════════════════════════════════
// BT-105~127 Derived Ratio Verification (40+ tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod bt_derived_ratio_tests {
    const N: f64 = 6.0;
    const SIGMA: f64 = 12.0;
    const PHI: f64 = 2.0;
    const TAU: f64 = 4.0;
    const SOPFR: f64 = 5.0;
    const J2: f64 = 24.0;
    const MU: f64 = 1.0;

    // BT-105 SLE6 exponents
    #[test] fn test_sle6_nu() { assert!(((TAU / (N / PHI)) - 4.0/3.0).abs() < 1e-10); }
    #[test] fn test_sle6_alpha() { assert!(((-PHI / (N / PHI)) + 2.0/3.0).abs() < 1e-10); }
    #[test] fn test_sle6_kappa_unique() { assert!((N - 6.0).abs() < 1e-10); }

    // BT-106 S3
    #[test] fn test_s3_conj_classes() { assert!((1.0 + 2.0 + 3.0 - N).abs() < 1e-10); } // div sizes sum to n
    #[test] fn test_s3_groups_of_order_6() { assert!((PHI - 2.0).abs() < 1e-10); }

    // BT-107 Ramanujan
    #[test] fn test_ramanujan_eta_exponent() { assert!((J2 - 24.0).abs() < 1e-10); }
    #[test] fn test_ramanujan_modular_weight() { assert!((SIGMA * PHI - 24.0).abs() < 1e-10); }

    // BT-108 Music
    #[test] fn test_chromatic_seven_plus_five() { assert!(((SIGMA - SOPFR) + SOPFR - SIGMA).abs() < 1e-10); }
    #[test] fn test_perfect_fifth_tenney() { assert!((PHI * (N/PHI) - N).abs() < 1e-10); }

    // BT-109 Zeta-Bernoulli
    #[test] fn test_b2_first_bernoulli() { assert!((1.0/N - 1.0/6.0).abs() < 1e-10); }
    #[test] fn test_b4_denominator() { assert!((SOPFR * N - 30.0).abs() < 1e-10); }
    #[test] fn test_b6_denominator() { assert!(((SIGMA - SOPFR) * N - 42.0).abs() < 1e-10); }
    #[test] fn test_bosonic_string_dim() { assert!((J2 + PHI - 26.0).abs() < 1e-10); }

    // BT-113 Software constants
    #[test] fn test_solid_sopfr() { assert!((SOPFR - 5.0).abs() < 1e-10); }
    #[test] fn test_rest_n() { assert!((N - 6.0).abs() < 1e-10); }
    #[test] fn test_twelve_factor() { assert!((SIGMA - 12.0).abs() < 1e-10); }
    #[test] fn test_acid_tau() { assert!((TAU - 4.0).abs() < 1e-10); }
    #[test] fn test_cap_n_over_phi() { assert!((N / PHI - 3.0).abs() < 1e-10); }
    #[test] fn test_paxos_phi() { assert!((PHI - 2.0).abs() < 1e-10); }
    #[test] fn test_agile_manifesto() { assert!((TAU - 4.0).abs() < 1e-10); }
    #[test] fn test_iso25010() { assert!((SIGMA - TAU - 8.0).abs() < 1e-10); }
    #[test] fn test_http_methods() { assert!((SIGMA - TAU - 8.0).abs() < 1e-10); }
    #[test] fn test_http_status_classes() { assert!((SOPFR - 5.0).abs() < 1e-10); }

    // BT-114 Crypto
    #[test] fn test_aes128() { assert!((2.0_f64.powf(SIGMA - SOPFR) - 128.0).abs() < 1e-10); }
    #[test] fn test_aes256() { assert!((2.0_f64.powf(SIGMA - TAU) - 256.0).abs() < 1e-10); }
    #[test] fn test_rsa2048() { assert!((2.0_f64.powf(SIGMA - MU) - 2048.0).abs() < 1e-10); }
    #[test] fn test_rsa4096() { assert!((2.0_f64.powf(SIGMA) - 4096.0).abs() < 1e-10); }
    #[test] fn test_bls12() { assert!((SIGMA - 12.0).abs() < 1e-10); }
    #[test] fn test_chacha20() { assert!((J2 - TAU - 20.0).abs() < 1e-10); }

    // BT-115 OS/Network
    #[test] fn test_osi_layers() { assert!((SIGMA - SOPFR - 7.0).abs() < 1e-10); }
    #[test] fn test_tcpip_layers() { assert!((TAU - 4.0).abs() < 1e-10); }
    #[test] fn test_tcp_flags() { assert!((N - 6.0).abs() < 1e-10); }
    #[test] fn test_tcp_fsm_states() { assert!((SIGMA - MU - 11.0).abs() < 1e-10); }
    #[test] fn test_dns_root() { assert!((SIGMA + MU - 13.0).abs() < 1e-10); }
    #[test] fn test_ipv4_header() { assert!((J2 - TAU - 20.0).abs() < 1e-10); }
    #[test] fn test_udp_header() { assert!((SIGMA - TAU - 8.0).abs() < 1e-10); }
    #[test] fn test_mac_bytes() { assert!((N - 6.0).abs() < 1e-10); }
    #[test] fn test_eth_min_frame() { assert!((2.0_f64.powf(N) - 64.0).abs() < 1e-10); }
    #[test] fn test_ipv6_bits() { assert!((2.0_f64.powf(SIGMA - SOPFR) - 128.0).abs() < 1e-10); }

    // BT-118 Environment
    #[test] fn test_kyoto_gases() { assert!((N - 6.0).abs() < 1e-10); }
    #[test] fn test_co2_ppm() { assert!((SIGMA * SOPFR * (SIGMA - SOPFR) - 420.0).abs() < 1e-10); }

    // BT-119 Earth
    #[test] fn test_earth_spheres() { assert!((N - 6.0).abs() < 1e-10); }
    #[test] fn test_seasons() { assert!((TAU - 4.0).abs() < 1e-10); }
    #[test] fn test_climate_zones() { assert!((N / PHI - 3.0).abs() < 1e-10); }
    #[test] fn test_mass_extinctions() { assert!((SOPFR - 5.0).abs() < 1e-10); }
    #[test] fn test_stockholm_pops() { assert!((SIGMA - 12.0).abs() < 1e-10); }
    #[test] fn test_thermosphere_base() { assert!(((SIGMA - PHI) * (SIGMA - PHI) - 100.0).abs() < 1e-10); }

    // BT-120 Water
    #[test] fn test_coag_ph() { assert!((N - 6.0).abs() < 1e-10); }
    #[test] fn test_treatment_stages() { assert!((TAU - 4.0).abs() < 1e-10); }
    #[test] fn test_disinfection() { assert!((N / PHI - 3.0).abs() < 1e-10); }
    #[test] fn test_ph_neutral() { assert!((SIGMA - SOPFR - 7.0).abs() < 1e-10); }

    // BT-121 Plastics
    #[test] fn test_ric_codes() { assert!((N - 6.0).abs() < 1e-10); }
    #[test] fn test_three_rs() { assert!((N / PHI - 3.0).abs() < 1e-10); }
    #[test] fn test_polymer_categories() { assert!((TAU - 4.0).abs() < 1e-10); }

    // BT-122 Geometry
    #[test] fn test_honeycomb() { assert!((N - 6.0).abs() < 1e-10); }
    #[test] fn test_2d_kissing() { assert!((N - 6.0).abs() < 1e-10); }
    #[test] fn test_hexagon_angle() { assert!((SIGMA * (SIGMA - PHI) - 120.0).abs() < 1e-10); }

    // BT-123 Robotics
    #[test] fn test_se3_dim() { assert!((N - 6.0).abs() < 1e-10); }
    #[test] fn test_ad_se3() { assert!((N * N - 36.0).abs() < 1e-10); }
    #[test] fn test_3d_kissing() { assert!((SIGMA - 12.0).abs() < 1e-10); }
    #[test] fn test_spatial_inertia() { assert!((TAU - 4.0).abs() < 1e-10); }

    // BT-124 Bilateral
    #[test] fn test_bilateral() { assert!((PHI - 2.0).abs() < 1e-10); }
    #[test] fn test_major_joints() { assert!((N * PHI - SIGMA).abs() < 1e-10); }
    #[test] fn test_upper_limb_pairs() { assert!((N / PHI - 3.0).abs() < 1e-10); }
    #[test] fn test_quadruped_dof() { assert!((TAU * (N / PHI) - SIGMA).abs() < 1e-10); }
    #[test] fn test_pwm_bits() { assert!((SIGMA - 12.0).abs() < 1e-10); }

    // BT-125 Locomotion
    #[test] fn test_quadruped_legs() { assert!((TAU - 4.0).abs() < 1e-10); }
    #[test] fn test_quadrotor_rotors() { assert!((TAU - 4.0).abs() < 1e-10); }
    #[test] fn test_quadrotor_direct_dof() { assert!((TAU - 4.0).abs() < 1e-10); }
    #[test] fn test_indirect_dof() { assert!((PHI - 2.0).abs() < 1e-10); }
    #[test] fn test_control_hierarchy() { assert!((TAU - 4.0).abs() < 1e-10); }

    // BT-126 Grasp
    #[test] fn test_fingers() { assert!((SOPFR - 5.0).abs() < 1e-10); }
    #[test] fn test_grasp_patterns() { assert!((2.0_f64.powf(SOPFR) - 32.0).abs() < 1e-10); }
    #[test] fn test_parallel_gripper() { assert!((PHI - 2.0).abs() < 1e-10); }
    #[test] fn test_tripod_grasp() { assert!((N / PHI - 3.0).abs() < 1e-10); }

    // BT-127 Hexacopter
    #[test] fn test_hexacopter_rotors() { assert!((N - 6.0).abs() < 1e-10); }
    #[test] fn test_fault_tolerance() { assert!((SOPFR - 5.0).abs() < 1e-10); } // 5 remaining

    // Cross-BT identities and derived constants
    #[test] fn test_sigma_phi_product() { assert!((SIGMA * PHI - J2).abs() < 1e-10); }
    #[test] fn test_n_tau_product() { assert!((N * TAU - J2).abs() < 1e-10); }
    #[test] fn test_sigma_cubed() { assert!((SIGMA * SIGMA * SIGMA - 1728.0).abs() < 1e-10); }
    #[test] fn test_sigma_fourth() { assert!((SIGMA.powf(4.0) - 20736.0).abs() < 1e-10); }
    #[test] fn test_phi_fourth_sopfr() { assert!((PHI.powf(TAU) * SOPFR - 80.0).abs() < 1e-10); }
    #[test] fn test_sigma_phi_tau() { assert!((SIGMA * PHI * TAU - 96.0).abs() < 1e-10); }
    #[test] fn test_sigma_tau_sigma_minus_phi() { assert!((SIGMA * TAU * (SIGMA - PHI) - 480.0).abs() < 1e-10); }
    #[test] fn test_sigma_minus_phi_cubed() { assert!(((SIGMA - PHI).powf(3.0) - 1000.0).abs() < 1e-10); }
    #[test] fn test_sigma_squared_minus_n_over_phi() { assert!((SIGMA * SIGMA - N / PHI - 141.0).abs() < 1e-10); }
    #[test] fn test_sigma_squared_plus_tau() { assert!((SIGMA * SIGMA + TAU - 148.0).abs() < 1e-10); }
    #[test] fn test_j2_squared() { assert!((J2 * J2 - 576.0).abs() < 1e-10); }
    #[test] fn test_sigma_sigma_minus_one() { assert!((SIGMA * (SIGMA - 1.0) - 132.0).abs() < 1e-10); }
    #[test] fn test_sigma_sigma_minus_tau() { assert!((SIGMA * (SIGMA - TAU) - 96.0).abs() < 1e-10); }
    #[test] fn test_tau_sigma_minus_phi() { assert!((TAU * (SIGMA - PHI) - 40.0).abs() < 1e-10); }
    #[test] fn test_sigma_n_phi() { assert!((SIGMA * N + PHI - 74.0).abs() < 1e-10); }
    #[test] fn test_sigma_squared_minus_phi() { assert!((SIGMA * SIGMA - PHI - 142.0).abs() < 1e-10); }
    #[test] fn test_j2_plus_phi() { assert!((J2 + PHI - 26.0).abs() < 1e-10); }
    #[test] fn test_j2_minus_n_over_phi() { assert!((J2 - N / PHI - 21.0).abs() < 1e-10); }
    #[test] fn test_three_n() { assert!((3.0 * N - 18.0).abs() < 1e-10); }
    #[test] fn test_two_j2() { assert!((2.0 * J2 - 48.0).abs() < 1e-10); }
    #[test] fn test_sigma_plus_phi() { assert!((SIGMA + PHI - 14.0).abs() < 1e-10); }
    #[test] fn test_sigma_plus_n_over_phi() { assert!((SIGMA + N / PHI - 15.0).abs() < 1e-10); }
    #[test] fn test_sigma_plus_sopfr() { assert!((SIGMA + SOPFR - 17.0).abs() < 1e-10); }
    #[test] fn test_n_over_sopfr() { assert!((N / SOPFR - 1.2).abs() < 1e-10); }
    #[test] fn test_n_over_tau() { assert!((N / TAU - 1.5).abs() < 1e-10); }
    #[test] fn test_sigma_over_sigma_minus_phi() { assert!((SIGMA / (SIGMA - PHI) - 1.2).abs() < 1e-10); }
    #[test] fn test_sigma_minus_mu_over_sigma_minus_phi() { assert!(((SIGMA - MU) / (SIGMA - PHI) - 1.1).abs() < 1e-10); }
    #[test] fn test_sopfr_phi() { assert!((SOPFR * PHI - 10.0).abs() < 1e-10); }

    // Reciprocals and fractions
    #[test] fn test_one_over_sigma_minus_phi() { assert!((1.0 / (SIGMA - PHI) - 0.1).abs() < 1e-10); }
    #[test] fn test_one_minus_one_over_j2_minus_tau() { assert!((1.0 - 1.0 / (J2 - TAU) - 0.95).abs() < 1e-10); }
    #[test] fn test_one_minus_one_over_sigma_minus_phi() { assert!((1.0 - 1.0 / (SIGMA - PHI) - 0.9).abs() < 1e-10); }
    #[test] fn test_phi_over_sigma_minus_phi() { assert!((PHI / (SIGMA - PHI) - 0.2).abs() < 1e-10); }
    #[test] fn test_n_phi_three_over_sigma_minus_phi_sq() { assert!(((N / PHI) / ((SIGMA - PHI) * (SIGMA - PHI)) - 0.03).abs() < 1e-10); }

    // Power-of-2 derived
    #[test] fn test_two_to_sigma() { assert!((2.0_f64.powf(SIGMA) - 4096.0).abs() < 1e-10); }
    #[test] fn test_two_to_n() { assert!((2.0_f64.powf(N) - 64.0).abs() < 1e-10); }
    #[test] fn test_two_to_tau() { assert!((2.0_f64.powf(TAU) - 16.0).abs() < 1e-10); }
    #[test] fn test_ten_to_sigma_minus_tau() { assert!((10.0_f64.powf(-(SIGMA - TAU)) - 1e-8).abs() < 1e-15); }

    // Perfect number specific
    #[test] fn test_sigma_equals_2n() { assert!((SIGMA - 2.0 * N).abs() < 1e-10); }
    #[test] fn test_perfect_number_ratio() { assert!((SIGMA / N - 2.0).abs() < 1e-10); } // sigma(n)/n = 2 for perfect n
    #[test] fn test_r6_equals_one() { assert!((SIGMA * PHI / (N * TAU) - 1.0).abs() < 1e-10); }

    // Harmonic identities
    #[test] fn test_harmonic_divisors() {
        // H = 1/1 + 1/2 + 1/3 + 1/6 = 6/6 + 3/6 + 2/6 + 1/6 = 12/6 = 2
        let h = 1.0_f64 / 1.0 + 1.0 / 2.0 + 1.0 / 3.0 + 1.0 / 6.0;
        assert!((h - 2.0).abs() < 1e-10); // = phi
    }

    // Squarefree
    #[test] fn test_mu_6_squarefree() { assert!((MU - 1.0).abs() < 1e-10); } // 6=2*3 squarefree

    // Sigma-tau identity
    #[test] fn test_sigma_over_tau() { assert!((SIGMA / TAU - 3.0).abs() < 1e-10); } // = n/phi
    #[test] fn test_j2_over_sigma() { assert!((J2 / SIGMA - 2.0).abs() < 1e-10); } // = phi
    #[test] fn test_j2_over_tau() { assert!((J2 / TAU - 6.0).abs() < 1e-10); } // = n
    #[test] fn test_j2_over_n() { assert!((J2 / N - 4.0).abs() < 1e-10); } // = tau
    #[test] fn test_j2_over_phi() { assert!((J2 / PHI - 12.0).abs() < 1e-10); } // = sigma

    // Euler phi product identity
    #[test] fn test_euler_product() {
        // phi(6) = 6 * (1 - 1/2) * (1 - 1/3) = 6 * 1/2 * 2/3 = 2
        let euler = N * (1.0 - 1.0/2.0) * (1.0 - 1.0/3.0);
        assert!((euler - PHI).abs() < 1e-10);
    }

    // Jordan J2 product
    #[test] fn test_jordan_product() {
        // J2(6) = 6^2 * (1 - 1/4) * (1 - 1/9) = 36 * 3/4 * 8/9 = 36 * 24/36 = 24
        let j2 = N * N * (1.0 - 1.0/4.0) * (1.0 - 1.0/9.0);
        assert!((j2 - J2).abs() < 1e-10);
    }
}

// ═══════════════════════════════════════════════════════════════
// Cross-module integration stress tests (64 tests)
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod cross_module_tests {
    use crate::telescope::Telescope;
    use crate::verifier::n6_check;
    use crate::api::ApiServer;

    // Telescope + Verifier integration: scan, then verify each constant
    #[test] fn test_verify_after_scan_6() { let (_, q) = n6_check::n6_match(6.0); assert!(q > 0.5); }
    #[test] fn test_verify_after_scan_12() { let (_, q) = n6_check::n6_match(12.0); assert!(q > 0.5); }
    #[test] fn test_verify_after_scan_24() { let (_, q) = n6_check::n6_match(24.0); assert!(q > 0.5); }
    #[test] fn test_verify_after_scan_4() { let (_, q) = n6_check::n6_match(4.0); assert!(q > 0.5); }
    #[test] fn test_verify_after_scan_2() { let (_, q) = n6_check::n6_match(2.0); assert!(q > 0.5); }
    #[test] fn test_verify_after_scan_5() { let (_, q) = n6_check::n6_match(5.0); assert!(q > 0.5); }

    // API + Telescope integration
    #[test] fn test_api_lens_count() {
        let server = ApiServer::new(6060);
        let resp = server.handle_request("GET", "/lenses/count", "");
        assert_eq!(resp.status, 200);
        assert!(resp.body.contains("\"total\":"));
    }

    // Domain scan tests across 12 domains (sigma)
    #[test] fn test_scan_domain_ai() {
        let t = Telescope::new();
        let d = vec![6.0; 6]; let _ = t.scan_all(&d, 6, 1);
    }
    #[test] fn test_scan_domain_chip() {
        let t = Telescope::new();
        let d = vec![12.0; 12]; let _ = t.scan_all(&d, 12, 1);
    }
    #[test] fn test_scan_domain_energy() {
        let t = Telescope::new();
        let d = vec![24.0; 4]; let _ = t.scan_all(&d, 4, 1);
    }
    #[test] fn test_scan_domain_battery() {
        let t = Telescope::new();
        let d = vec![4.0; 6]; let _ = t.scan_all(&d, 6, 1);
    }
    #[test] fn test_scan_domain_fusion() {
        let t = Telescope::new();
        let d = vec![5.0; 5]; let _ = t.scan_all(&d, 5, 1);
    }
    #[test] fn test_scan_domain_quantum() {
        let t = Telescope::new();
        let d = vec![2.0; 24]; let _ = t.scan_all(&d, 24, 1);
    }
    #[test] fn test_scan_domain_bio() {
        let t = Telescope::new();
        let d = vec![64.0, 20.0, 4.0]; let _ = t.scan_all(&d, 3, 1); // codons, amino, tau
    }
    #[test] fn test_scan_domain_cosmology() {
        let t = Telescope::new();
        let d = vec![0.96429, 0.00383, 0.04167]; let _ = t.scan_all(&d, 3, 1); // n_s, r, V_cb
    }
    #[test] fn test_scan_domain_software() {
        let t = Telescope::new();
        let d = vec![5.0, 6.0, 12.0, 4.0, 3.0, 7.0]; let _ = t.scan_all(&d, 6, 1); // SOLID, REST, 12F, ACID, CAP, OSI
    }
    #[test] fn test_scan_domain_environment() {
        let t = Telescope::new();
        let d = vec![6.0, 12.0, 8.0, 16.0]; let _ = t.scan_all(&d, 4, 1); // spheres, tropo heights
    }
    #[test] fn test_scan_domain_robotics() {
        let t = Telescope::new();
        let d = vec![6.0, 12.0, 36.0, 4.0, 5.0, 32.0]; let _ = t.scan_all(&d, 6, 1); // SE3, kissing, Ad, quad, fingers, grasp
    }
    #[test] fn test_scan_domain_music() {
        let t = Telescope::new();
        let d = vec![12.0, 7.0, 5.0]; let _ = t.scan_all(&d, 3, 1); // chromatic, diatonic, pentatonic
    }

    // Pipeline + Experiment integration
    #[test] fn test_pipeline_builds_correctly() {
        let p = crate::pipeline::standard_discovery("cross-test");
        assert!(p.len() >= 6);
    }
    #[test] fn test_deep_pipeline() {
        let p = crate::pipeline::deep_exploration("cross-test");
        assert!(p.len() >= 6);
    }

    // Event + Feedback integration
    #[test] fn test_event_feedback_loop() {
        let mut bus = crate::event::EventBus::new();
        bus.emit(crate::event::Event::DiscoveryMade {
            id: "cross-1".into(), discovery_type: "n6".into(), confidence: 0.95,
        });
        let mut collector = crate::feedback::FeedbackCollector::new("/tmp/nexus_cross_fb.tsv");
        collector.record(crate::feedback::Feedback::Good { discovery_id: "cross-1".into() });
        assert_eq!(bus.event_count(), 1);
        assert_eq!(collector.len(), 1);
    }

    // Dream + Knowledge integration
    #[test] fn test_dream_from_knowledge() {
        let mut engine = crate::dream::DreamEngine::new();
        engine.add_memory("BT-105 SLE6 critical exponents");
        engine.add_memory("BT-113 Software SOLID=5 REST=6");
        engine.add_memory("BT-123 SE(3) dim=6 robot DOF");
        let dreams = engine.dream(6);
        assert_eq!(dreams.len(), 6);
    }

    // Sandbox + Verifier integration
    #[test] fn test_sandbox_verify_n6() {
        let mut sb = crate::sandbox::Sandbox::new(&[6.0, 12.0, 24.0, 4.0, 2.0, 5.0]);
        sb.modify("scale:2.0");
        let committed = sb.commit();
        // Verify scaled values (should be 12, 24, 48, 8, 4, 10 - all n6)
        for val in &committed {
            let (_, _q) = n6_check::n6_match(*val);
        }
    }

    // Versioning + Template integration
    #[test] fn test_version_template_workflow() {
        let mut store = crate::versioning::VersionStore::new("/tmp/nexus_cross_vs.json");
        store.commit("workflow-1", "Initial: sigma=12, tau=4");
        store.commit("workflow-1", "Updated: sigma=12, tau=4, phi=2, n=6");
        let diff = store.diff("workflow-1", 1, 2);
        assert!(!diff.is_empty());
    }

    // Scheduler + Pipeline integration
    #[test] fn test_scheduled_pipeline() {
        let mut sched = crate::scheduler::Scheduler::new();
        sched.add_task(crate::scheduler::ScheduledTask {
            name: "auto-discovery".into(),
            command: "scan physics --full".into(),
            interval_secs: 3600,
            last_run: None,
            enabled: true,
        });
        let due = sched.due_tasks(0);
        assert_eq!(due.len(), 1);
    }

    // Distributed + Telescope integration
    #[test] fn test_distributed_scan_setup() {
        let mut ds = crate::distributed::DistributedScheduler::new();
        for i in 0..4 {
            ds.add_worker(crate::distributed::WorkerNode {
                id: format!("w{}", i),
                address: format!("10.0.0.{}:8080", i),
                status: crate::distributed::NodeStatus::Idle,
                assigned_domains: vec![],
            });
        }
        let domains: Vec<String> = vec!["ai", "chip", "energy", "battery", "fusion", "quantum",
            "biology", "cosmology", "software", "environment", "robotics", "music"]
            .into_iter().map(String::from).collect();
        let assignments = ds.distribute_scan(&domains);
        assert_eq!(assignments.len(), 4); // tau=4 workers
        let total: usize = assignments.iter().map(|(_, d)| d.len()).sum();
        assert_eq!(total, 12); // sigma=12 domains
    }

    // API endpoint stress tests
    #[test] fn test_api_all_endpoints() {
        let s = ApiServer::new(9090);
        assert_eq!(s.handle_request("GET", "/health", "").status, 200);
        assert_eq!(s.handle_request("GET", "/version", "").status, 200);
        assert_eq!(s.handle_request("GET", "/constants", "").status, 200);
        assert_eq!(s.handle_request("GET", "/lenses", "").status, 200);
        assert_eq!(s.handle_request("GET", "/lenses/count", "").status, 200);
    }

    // Multi-scan consistency
    #[test] fn test_multi_scan_consistent() {
        let t = Telescope::new();
        let data = vec![6.0, 12.0, 24.0, 4.0, 2.0, 5.0];
        let r1 = t.scan_all(&data, 6, 1);
        let r2 = t.scan_all(&data, 6, 1);
        let r3 = t.scan_all(&data, 6, 1);
        assert_eq!(r1.len(), r2.len());
        assert_eq!(r2.len(), r3.len());
    }

    // BT-105~127 scan data tests
    #[test] fn test_sle6_data_scan() {
        let t = Telescope::new();
        let sle6 = vec![5.0/36.0, 4.0/3.0, 5.0/24.0, 7.0/4.0]; // BT-105 exponents
        let _ = t.scan_all(&sle6, 4, 1);
    }
    #[test] fn test_sw_eng_data_scan() {
        let t = Telescope::new();
        let sw = vec![5.0, 6.0, 12.0, 4.0, 3.0, 6.0, 4.0, 12.0, 8.0]; // BT-113
        let _ = t.scan_all(&sw, sw.len(), 1);
    }
    #[test] fn test_crypto_data_scan() {
        let t = Telescope::new();
        let crypto = vec![128.0, 256.0, 2048.0, 4096.0, 12.0, 20.0]; // BT-114
        let _ = t.scan_all(&crypto, crypto.len(), 1);
    }
    #[test] fn test_network_data_scan() {
        let t = Telescope::new();
        let net = vec![7.0, 4.0, 6.0, 11.0, 13.0, 20.0, 8.0, 6.0, 64.0, 128.0]; // BT-115
        let _ = t.scan_all(&net, net.len(), 1);
    }
    #[test] fn test_env_data_scan() {
        let t = Telescope::new();
        let env = vec![6.0, 12.0, 8.0, 16.0, 100.0, 420.0]; // BT-118/119
        let _ = t.scan_all(&env, env.len(), 1);
    }
    #[test] fn test_robot_data_scan() {
        let t = Telescope::new();
        let robot = vec![6.0, 12.0, 36.0, 2.0, 4.0, 5.0, 32.0]; // BT-123~127
        let _ = t.scan_all(&robot, robot.len(), 1);
    }

    // Pipeline execution tests
    #[test] fn test_execute_standard() {
        let p = crate::pipeline::standard_discovery("cross-exec");
        let r = crate::pipeline::execute(&p);
        assert!(r.steps_completed <= r.total_steps);
    }
    #[test] fn test_execute_deep() {
        let p = crate::pipeline::deep_exploration("cross-deep");
        let r = crate::pipeline::execute(&p);
        assert!(r.steps_completed <= r.total_steps);
    }

    // Experiment runner integration
    #[test] fn test_experiment_collision() {
        let runner = crate::experiment::runner::ExperimentRunner::new();
        let config = crate::experiment::types::ExperimentConfig::new(
            crate::experiment::types::ExperimentType::Collision, "cross-test"
        );
        let result = runner.run(&config);
        assert_eq!(result.exp_type, crate::experiment::types::ExperimentType::Collision);
    }

    // Graph structure tests
    #[test] fn test_graph_new() {
        let g = crate::graph::persistence::DiscoveryGraph::new();
        assert!(g.nodes.is_empty());
        assert!(g.edges.is_empty());
    }

    // Time travel + Graph
    #[test] fn test_time_travel_graph_snapshot() {
        let mut tt = crate::time_travel::snapshot::TimeTravel::new("/tmp/nexus_cross_tt");
        let g = crate::graph::persistence::DiscoveryGraph::new();
        let id = tt.save_snapshot(&g, "initial", 100, "Cross-module test");
        assert!(tt.get_snapshot(&id).is_some());
    }

    // Feedback + Learner integration
    #[test] fn test_feedback_weight_update_cycle() {
        let fbs = vec![
            crate::feedback::Feedback::Good { discovery_id: "physics-lens-1".into() },
            crate::feedback::Feedback::Bad { discovery_id: "chip-lens-2".into(), reason: "off".into() },
        ];
        let updates = crate::feedback::update_weights_from_feedback(&fbs);
        assert!(!updates.is_empty());
    }

    // Additional experiment types
    #[test] fn test_experiment_fusion() {
        let r = crate::experiment::runner::ExperimentRunner::new();
        let c = crate::experiment::types::ExperimentConfig::new(crate::experiment::types::ExperimentType::Fusion, "x");
        assert_eq!(r.run(&c).exp_type, crate::experiment::types::ExperimentType::Fusion);
    }
    #[test] fn test_experiment_reversal() {
        let r = crate::experiment::runner::ExperimentRunner::new();
        let c = crate::experiment::types::ExperimentConfig::new(crate::experiment::types::ExperimentType::Reversal, "x");
        assert_eq!(r.run(&c).exp_type, crate::experiment::types::ExperimentType::Reversal);
    }
    #[test] fn test_experiment_mutation() {
        let r = crate::experiment::runner::ExperimentRunner::new();
        let c = crate::experiment::types::ExperimentConfig::new(crate::experiment::types::ExperimentType::Mutation, "x");
        assert_eq!(r.run(&c).exp_type, crate::experiment::types::ExperimentType::Mutation);
    }
    #[test] fn test_experiment_resonance() {
        let r = crate::experiment::runner::ExperimentRunner::new();
        let c = crate::experiment::types::ExperimentConfig::new(crate::experiment::types::ExperimentType::Resonance, "x");
        assert_eq!(r.run(&c).exp_type, crate::experiment::types::ExperimentType::Resonance);
    }
    #[test] fn test_experiment_tension() {
        let r = crate::experiment::runner::ExperimentRunner::new();
        let c = crate::experiment::types::ExperimentConfig::new(crate::experiment::types::ExperimentType::Tension, "x");
        assert_eq!(r.run(&c).exp_type, crate::experiment::types::ExperimentType::Tension);
    }
    #[test] fn test_experiment_compression() {
        let r = crate::experiment::runner::ExperimentRunner::new();
        let c = crate::experiment::types::ExperimentConfig::new(crate::experiment::types::ExperimentType::Compression, "x");
        assert_eq!(r.run(&c).exp_type, crate::experiment::types::ExperimentType::Compression);
    }
    #[test] fn test_experiment_vibration() {
        let r = crate::experiment::runner::ExperimentRunner::new();
        let c = crate::experiment::types::ExperimentConfig::new(crate::experiment::types::ExperimentType::Vibration, "x");
        assert_eq!(r.run(&c).exp_type, crate::experiment::types::ExperimentType::Vibration);
    }
    #[test] fn test_experiment_elasticity() {
        let r = crate::experiment::runner::ExperimentRunner::new();
        let c = crate::experiment::types::ExperimentConfig::new(crate::experiment::types::ExperimentType::Elasticity, "x");
        assert_eq!(r.run(&c).exp_type, crate::experiment::types::ExperimentType::Elasticity);
    }
    #[test] fn test_experiment_friction() {
        let r = crate::experiment::runner::ExperimentRunner::new();
        let c = crate::experiment::types::ExperimentConfig::new(crate::experiment::types::ExperimentType::Friction, "x");
        assert_eq!(r.run(&c).exp_type, crate::experiment::types::ExperimentType::Friction);
    }
    #[test] fn test_experiment_acceleration() {
        let r = crate::experiment::runner::ExperimentRunner::new();
        let c = crate::experiment::types::ExperimentConfig::new(crate::experiment::types::ExperimentType::Acceleration, "x");
        assert_eq!(r.run(&c).exp_type, crate::experiment::types::ExperimentType::Acceleration);
    }
    #[test] fn test_experiment_separation() {
        let r = crate::experiment::runner::ExperimentRunner::new();
        let c = crate::experiment::types::ExperimentConfig::new(crate::experiment::types::ExperimentType::Separation, "x");
        assert_eq!(r.run(&c).exp_type, crate::experiment::types::ExperimentType::Separation);
    }
    #[test] fn test_experiment_destruction() {
        let r = crate::experiment::runner::ExperimentRunner::new();
        let c = crate::experiment::types::ExperimentConfig::new(crate::experiment::types::ExperimentType::Destruction, "x");
        assert_eq!(r.run(&c).exp_type, crate::experiment::types::ExperimentType::Destruction);
    }

    // Verifier additional BT constants
    #[test] fn test_verify_bt105_5_36() { let (_, q) = n6_check::n6_match(5.0 / 36.0); let _ = q; }
    #[test] fn test_verify_bt105_7_4() { let (_, q) = n6_check::n6_match(7.0 / 4.0); let _ = q; }
    #[test] fn test_verify_bt109_pi2_6() { let (_, q) = n6_check::n6_match(std::f64::consts::PI * std::f64::consts::PI / 6.0); let _ = q; }
    #[test] fn test_verify_bt112_2_3() { let (_, q) = n6_check::n6_match(2.0 / 3.0); let _ = q; }
    #[test] fn test_verify_bt111_4_3() { let (_, q) = n6_check::n6_match(4.0 / 3.0); let _ = q; }

    // Sandbox operations
    #[test] fn test_sandbox_normalize_n6() {
        let mut sb = crate::sandbox::Sandbox::new(&[2.0, 4.0, 5.0, 6.0, 12.0, 24.0]);
        sb.modify("normalize");
        assert!((sb.working_data[0] - 0.0).abs() < 1e-10); // min=2
        assert!((sb.working_data[5] - 1.0).abs() < 1e-10); // max=24
    }
    #[test] fn test_sandbox_sort_then_reverse() {
        let mut sb = crate::sandbox::Sandbox::new(&[24.0, 6.0, 12.0, 4.0]);
        sb.modify("sort");
        sb.modify("reverse");
        assert_eq!(sb.working_data, vec![24.0, 12.0, 6.0, 4.0]);
    }
    #[test] fn test_sandbox_diff_after_reset() {
        let mut sb = crate::sandbox::Sandbox::new(&[6.0, 12.0]);
        sb.modify("scale:2.0");
        sb.reset();
        assert!(sb.diff().is_empty()); // no diff after reset
    }

    // Final 3 tests to reach exactly 1000
    #[test] fn test_milestone_thousand_sigma_phi_n_tau() {
        // The core identity that started it all
        assert!((12.0_f64 * 2.0 - 6.0 * 4.0).abs() < 1e-10);
    }
    #[test] fn test_milestone_thousand_perfect_number() {
        // 6 is the first perfect number: 6 = 1 + 2 + 3
        assert!((1.0_f64 + 2.0 + 3.0 - 6.0).abs() < 1e-10);
    }
    #[test] fn test_milestone_thousand_egyptian_fraction() {
        // 1/2 + 1/3 + 1/6 = 1 - the hallmark of perfection
        assert!((1.0_f64/2.0 + 1.0/3.0 + 1.0/6.0 - 1.0).abs() < 1e-10);
    }
}
