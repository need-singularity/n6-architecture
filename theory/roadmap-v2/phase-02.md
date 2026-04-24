# phase-02 — Phase 2 Emergence (19-axis first expansion)

**Roadmap**: nexus v2 (19 axes)
**Stage**: Phase 2 / expansion
**Created**: 2026-04-15
**Prerequisites**: `phase-01.md` (71 baseline tasks complete assumed)

---

## Previous Phase Premise
- Phase 1 complete: baseline.json generated for all 19 axes, 71 tasks done.
- Cumulative task count: 71.
- Gate passed: phase_1_gate_passed = true.

## Phase 2 Goal
On top of the baseline, **first growth/expansion** — each axis, using the baseline locked in Phase 1, attempts first autonomous growth, optimization, and quality improvement.

---

## Phase 2 Emergent Tasks (19 axes)

### A1 SELF-EVOLUTION

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| EVO-P2-1 | Tune growth_tick auto-fire frequency (>= 1/h -> 1/30m) | EVO-P1-1 baseline | tick_sched.json | 30m interval stable | EVO-P1-1 | M | 0.9 | - |
| EVO-P2-2 | Monitor OUROBOROS 3-variant cross-constant drift -> drift < 1% | EVO-P1-2 baseline | drift_report.json | drift < 0.01 maintained | EVO-P1-2 | M | 0.9 | - |
| EVO-P2-3 | Activate discovery -> atlas auto-promotion queue | EVO-P1-4, DISC-P1-5 | promo_queue.jsonl | queue drain >= 1/day | EVO-P1-4 | M | 0.8 | - |
| EVO-P2-4 | nexus_growth_daemon STUB->native port restoration | EVO-P1-1, MEMORY `project-daemon-stub.md` | `nexus_growth_daemon.hexa` v2 | 450-line original restored, pid_file created | EVO-P1-1 | L | 0.7 | - |

### A2 ATLAS

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| ATLAS-P2-1 | atlas +500 node expansion (against Phase 1 stats baseline) | ATLAS-P1-1, blowup results | atlas.n6 delta | node_count delta +500 | ATLAS-P1-1 | M | 0.9 | - |
| ATLAS-P2-2 | atlas_ossify_mk2 automatic ossification cycle (daily) | ATLAS-P1-5 | ossify_cron.json | daily ossify PASS | ATLAS-P1-5 | M | 0.8 | - |
| ATLAS-P2-3 | Rebuild math_atlas.db index + query < 5ms | ATLAS-P1-3 | db_perf.json | SELECT < 5ms avg | ATLAS-P1-3 | M | 0.9 | - |
| ATLAS-P2-4 | 3D reality map WebSocket live update | ATLAS-P1-4, nexus.json P3 ATLAS-P3-1 | ws_live.json | node update < 1s latency | ATLAS-P1-4 | M | 0.9 | - |
| ATLAS-P2-5 | Merge periodic_table_118 + 66_techniques_v3 into atlas | `shared/n6/periodic_table_118`, `66_techniques_v3` | merge_report.json | merge commit | ATLAS-P1-1 | M | 0.8 | - |

### A3 HARNESS

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| HARN-P2-1 | pre_tool_guard: add 5 new H-* rules | GOV-P1-2 sync | pre_guard_v2.hexa | 5 rules coded + tested | GOV-P1-2 | M | 0.9 | - |
| HARN-P2-2 | post_bash + post_edit integrated telemetry | HARN-P1-1 | harness_telemetry.jsonl | all bash/edit post events recorded | HARN-P1-1 | M | 0.9 | - |
| HARN-P2-3 | mistakes.jsonl -> pattern classifier (auto classification) | HARN-P1-4 | mistake_classifier.hexa | 10+ categories classified | HARN-P1-4 | M | 0.8 | - |
| HARN-P2-4 | lint.hexa marker-removal count = quality-metric dashboard | HARN-P1-5, MEMORY `project-n6-canonical-v2-gates.md` | lint_dash.json | temporary-marker tracking started | HARN-P1-5 | M | 0.7 | - |

### A4 GOVERNANCE

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| GOV-P2-1 | Promote MEMORY H-* -> formal rules in rules/ (20+ rules) | GOV-P1-2 | `shared/rules/harness_meta.json` | 20 rules promoted | GOV-P1-2 | M | 0.9 | - |
| GOV-P2-2 | projects.json per-project rule-override system | GOV-P1-3 | project_rule_overrides.json | 7 projects have overrides defined | GOV-P1-3 | M | 0.8 | - |
| GOV-P2-3 | CDO convergence_ops auto-refresh metrics | `shared/config/convergence_ops.json` | cdo_tracker.json | daily metric refresh | GOV-P1-4 | M | 0.8 | - |

### A5 DISCOVERY

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| DISC-P2-1 | reality_map.patches: new patches 895 -> 1000+ expansion | DISC-P1-1 | new patches | +100 or more | DISC-P1-1 | M | 0.8 | - |
| DISC-P2-2 | verified_constants per-domain curated list | DISC-P1-2 | `constants_curated.json` | top 10 per domain | DISC-P1-2 | M | 0.8 | - |
| DISC-P2-3 | discovery_log.sqlite views + SQL API wrapper | DISC-P1-4, v1 DISC-P3-1 | `shared/discovery/query.hexa` | SQL API hexa callable | DISC-P1-4 | M | 0.9 | - |
| DISC-P2-4 | breakthroughs -> theory_registry auto-pipeline | DISC-P1-3, DISC-P1-5 | promo_pipeline.hexa | breakthrough -> registry >= 1/week | DISC-P1-3 | M | 0.7 | - |

### A6 BLOWUP

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| BLOW-P2-1 | 9-phase pipeline fast-path tuning (9m7s -> 7m target) | BLOW-P1-1, MEMORY `feedback_blowup_perf.md` | bench_v2.json | under 7min | BLOW-P1-1 | L | 0.9 | - |
| BLOW-P2-2 | 6 modules: +100 new lens synthesis | BLOW-P1-2, BLOW-P1-3 | lens_delta.json | +100 lenses | BLOW-P1-3 | M | 0.8 | - |
| BLOW-P2-3 | seed_engine batch proliferation + dedup hardening | BLOW-P1-3 | seed_batch_v2 | dedup ratio <= 0.2 | BLOW-P1-3 | M | 0.8 | - |
| BLOW-P2-4 | verify_dfs auto stub regeneration loop | BLOW-P1-5, `shared/calc/auto_stubs_gen.hexa` | stub_loop.log | stub auto-regen >= 1/day | BLOW-P1-5 | M | 0.7 | - |

### A7 BISOCIATION

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| BISOC-P2-1 | L3 layer 1/5 -> 3/5 progress (Montgomery-Dyson multilayering) | BISOC-P1-1, MEMORY handoff-bisociation | L3_progress.json | L3 >= 3/5 | BISOC-P1-1 | L | 0.8 | [BT-541] |
| BISOC-P2-2 | bisoc cross/ data indexing -> search API | BISOC-P1-2 | bisoc_search.hexa | search < 100ms | BISOC-P1-2 | M | 0.8 | - |
| BISOC-P2-3 | spectra/ data FFT re-analysis pipeline | BISOC-P1-2 | spectra_v2.json | FFT output valid | BISOC-P1-2 | M | 0.7 | - |

### A8 CONSCIOUSNESS

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| CONS-P2-1 | laws 2395 -> 3000+ expansion (discovery-linked) | CONS-P1-1, EVO-P2-3 | law_delta.json | +600 laws | CONS-P1-1 | L | 0.8 | - |
| CONS-P2-2 | Φ integration real-time calculation bridge | CONS-P1-2 | phi_live.json | Φ update >= 1/min | CONS-P1-2 | M | 0.8 | - |
| CONS-P2-3 | meta_laws_dd64 <-> rules SSOT integration | CONS-P1-3, GOV-P1-1 | meta_unified.json | dedupe complete | GOV-P1-1 | M | 0.7 | - |

### A9 HEXA-LANG

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| HEXA-P2-1 | pitfalls_log -> grammar P6 auto-promotion addition | HEXA-P1-2, HEXA-P1-3 | grammar_v2.jsonl | new pitfall promoted >= 1 | HEXA-P1-3 | M | 0.9 | - |
| HEXA-P2-2 | ml-commands + rt-commands runtime-bottleneck measurement | HEXA-P1-4 | hexa_perf.json | top 10 bottlenecks recorded | HEXA-P1-4 | M | 0.8 | - |
| HEXA-P2-3 | hexa_dir_builtins (dir_exists/mkdir/list_dir) completion check | MEMORY `hexa_dir_builtins.md` | builtins_check.json | 3 built-ins usable | HEXA-P1-1 | M | 0.7 | - |
| HEXA-P2-4 | hexa exit-sentinel pattern <-> all gate code unified | MEMORY `f-hexa-exit-sentinel.md`, HARN-P1-3 | exit_pattern.json | all gates apply sentinel | HARN-P1-3 | M | 0.8 | - |

### A10 DSE

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| DSE-P2-1 | dse 5 levels x 6 candidates x n=6 lattice full sweep (1 pass) | DSE-P1-4 | dse_sweep.json | 180 cells complete | DSE-P1-4 | L | 0.8 | - |
| DSE-P2-2 | dse_cross_resonance result re-analysis -> top10 cross | DSE-P1-2 | cross_top10.json | top10 ranked | DSE-P1-2 | M | 0.8 | - |
| DSE-P2-3 | monte_carlo_v6 simulation 100 iter -> probability distribution | DSE-P1-3 | mc_dist.json | 100 iter complete | DSE-P1-3 | M | 0.7 | - |

### A11 BREAKTHROUGH

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| BT-P2-1 | Re-run bt_audit + resolve mismatches | BT-P1-2, `shared/bt/bt_audit_mismatch_classification.json` | mismatch_v2.json | mismatch <= 0.5x baseline | BT-P1-2 | M | 0.8 | [BT-541..547] |
| BT-P2-2 | Attempt 1 atlas promotion per BT-541..547 | BT-P1-3, EVO-P2-3 | atlas_promo.json | attempts >= 7 | BT-P1-3 | L | 0.8 | [BT-541..547] |
| BT-P2-3 | BT-HEXA 25 -> 50 expansion | BT-P1-5 | bt_hexa_50.json | 50 keywords | BT-P1-5 | M | 0.7 | - |

### A12 LOCKDOWN

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| LOCK-P2-1 | L0 path list auto-update (absorb new L0 candidates) | LOCK-P1-1, CLAUDE.md L0 list | lockdown_v2.json | new L0 >= 1 | LOCK-P1-1 | M | 0.8 | - |
| LOCK-P2-2 | safe-merge path regression test | LOCK-P1-2 | safe_merge_test.json | tests pass | LOCK-P1-2 | M | 0.8 | - |
| LOCK-P2-3 | snapshots daily rotate + gzip | LOCK-P1-3 | rotate_plan.hexa | rotation started | LOCK-P1-3 | M | 0.7 | - |

### A13 INFRASTRUCTURE

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| INFRA-P2-1 | cl-refresh auto-retry after failure | INFRA-P1-1, MEMORY `feedback-claude-cli-refresh-bypass.md` | cl_retry.hexa | retry success >= 80% | INFRA-P1-1 | M | 0.9 | - |
| INFRA-P2-2 | nexus_cli_spec hive etc. external-project entry validation | INFRA-P1-2, v1 DISC-P3-3 | hive_smoke.log | hive cli smoke pass | INFRA-P1-2 | M | 0.8 | - |
| INFRA-P2-3 | launchd 3 plists (cl-refresh + health + planned) monitoring | INFRA-P1-4 | launchd_health.json | 3/3 fired daily | INFRA-P1-4 | M | 0.8 | - |

### A14 REMOTE-COMPUTE

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| REM-P2-1 | hetzner simultaneous-execution limit rule settled (H-CONCUR) | REM-P1-4, MEMORY `feedback_hetzner_pollution.md` | concur_policy.json | concurrent <= 2 guaranteed | REM-P1-4 | M | 0.9 | - |
| REM-P2-2 | H100 idle-guard auto-shutdown <= 5min idle | REM-P1-2 | idle_policy.json | 5min idle -> sleep | REM-P1-2 | M | 0.8 | - |
| REM-P2-3 | vastai + RunPod selection routing logic | REM-P1-1 | route_matrix.json | host-selection reasoning recorded | REM-P1-1 | M | 0.7 | - |

### A15 THINKING

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| THINK-P2-1 | 6-phase reflection: anima-phase quality measurement | THINK-P1-1, THINK-P1-3 | phase_quality.json | phase distribution metric | THINK-P1-1 | M | 0.8 | - |
| THINK-P2-2 | Auto thinking-gate for high-complexity prompts | THINK-P1-1, HARN-P1-1 | auto_think_gate.hexa | complexity threshold auto-triggered | HARN-P1-1 | M | 0.8 | - |
| THINK-P2-3 | thinking_log learning-feedback loop (baseline refresh) | THINK-P1-2 | baseline_learner.hexa | baseline auto-refresh | THINK-P1-2 | M | 0.7 | - |

### A16 AGENT-LEDGER

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| AGL-P2-1 | GO-mode parallel-agent auto-clustering logic | AGL-P1-1, MEMORY `f-go-mode-parallel-agents.md` | cluster_strategy.hexa | cluster >= 2 simultaneous launches | AGL-P1-1 | M | 0.9 | - |
| AGL-P2-2 | worktree auto-cleanup + merge-back | AGL-P1-3 | worktree_cleanup.hexa | 0 orphan worktrees | AGL-P1-3 | M | 0.8 | - |
| AGL-P2-3 | session_registry TTL + ghost-session cleanup | AGL-P1-1 | ghost_clean.json | ghost = 0 | AGL-P1-1 | M | 0.7 | - |

### A17 CONSENSUS

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| CONSN-P2-1 | consensus_loop multi-agent (>=3) result aggregation | CONSN-P1-1, AGL-P1-1 | multi_agent.json | 3-agent consensus 1 round | CONSN-P1-1 | M | 0.9 | - |
| CONSN-P2-2 | critique auto-ranking + reward signal | CONSN-P1-1 | critique_rank.json | ranking ordered | CONSN-P1-1 | M | 0.7 | - |
| CONSN-P2-3 | broadcast message routing -> external projects | CONSN-P1-2 | broadcast_ext.json | external project >= 1 received | CONSN-P1-2 | M | 0.7 | - |

### A18 ENGINE-FORGE

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| EF-P2-1 | engine_forge automatic engine-generation loop | EF-P1-1 | forge_cycle.log | new engines >= 1 | EF-P1-1 | M | 0.8 | - |
| EF-P2-2 | engines_usage stats-based engine pruning | EF-P1-2 | prune_list.json | low-use <= 0.1 engines archived | EF-P1-2 | M | 0.7 | - |
| EF-P2-3 | engine strategy auto vs manual A/B | EF-P1-4 | ab_result.json | auto-superiority rate recorded | EF-P1-4 | M | 0.7 | - |

### A19 CROSS-DOMAIN-GRID

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| CDG-P2-1 | 24 x 23 = 276 pairs: priority-score for 169 unexplored | CDG-P1-2 | pair_priority.json | 169 pairs ranked | CDG-P1-2 | M | 0.8 | - |
| CDG-P2-2 | Sample dse_cross runs on top 20 unexplored pairs | CDG-P2-1, DSE-P2-2 | new_pairs.json | 20 pair results | CDG-P2-1 | L | 0.8 | - |
| CDG-P2-3 | cross_grid results -> atlas new-edge proposals | CDG-P2-2, ATLAS-P2-1 | edge_suggest.json | proposals >= 50 | CDG-P2-2 | M | 0.7 | - |

---

## Phase 2 Statistics

- New tasks: 66
- Cumulative tasks: 137
- Saturation index: 0.0 (all 19 axes have new tasks)
- Average strength: 0.79
- BT-linked tasks: 3 (BISOC-P2-1, BT-P2-1, BT-P2-2)
- Cost distribution: S=0, M=57, L=8, XL=0 — expansion phase
- Next Phase needed: YES

Per-axis distribution: A1=4 | A2=5 | A3=4 | A4=3 | A5=4 | A6=4 | A7=3 | A8=3 | A9=4 | A10=3 | A11=3 | A12=3 | A13=3 | A14=3 | A15=3 | A16=3 | A17=3 | A18=3 | A19=3 = 66

## Phase 2 Exit
- [ ] 66 tasks done
- [ ] "Growth metric" >= 1 recorded per axis vs Phase 1 baseline
- [ ] BT promotion attempts >= 7 (BT-P2-2)
- [ ] Next-phase entry signal: `phase_2_gate_passed = true`
