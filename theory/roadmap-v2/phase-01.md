# phase-01 — Phase 1 Emergence (19-axis first breakthrough point)

**Roadmap**: nexus v2 (NEXUS-6 central hub, 19 axes)
**Stage**: Phase 1 / entry
**Created**: 2026-04-15
**Prerequisites**: `axis-final.md` (19 axes finalized), `README.md` (v2 frame)
**Seeds**: SELF-EVOLUTION + ATLAS + DISCOVERY (retained) + 16 emergent axes

---

## Previous Phase Premise
- Phase 0 concept: DSE (R1~R5) rounds run until saturation for the 19 emergent axes = axis list fully determined.
- Cumulative task count: 0 (pre-emergence).
- R3 158 domains + v1 nexus.json P0~P113 (696 tasks) available for reuse.

## Phase 1 Goal
For each of the 19 axes, identify its **first breakthrough point** = "current-state diagnosis + basic gate + baseline lock" grounded in concrete assets. Phase 1 is the axis's **existence demonstration**; it is not a solution stage but a **baseline operating lock** stage.

Phase 1 meta-principles:
- Zero imagined tasks — every task must cite at least one `evidence file path`.
- Tasks within the same axis run sequentially; across axes they run in parallel.
- Phase 1 average strength >= 0.7 (strength = reuse rate of existing assets).
- When connected to the 7 millennium targets, annotate `[BT-54X]` tag.

---

## Phase 1 Emergent Tasks (19 axes)

### A1 SELF-EVOLUTION

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| EVO-P1-1 | Diagnose growth_tick + nexus_growth_daemon state -> measure pid_file/plist | `shared/harness/growth_tick.hexa`, `nexus_growth_daemon.hexa` | `shared/harness/evo_baseline.json` | no "daemon no-evolution tick >= 3" | - | S | 0.9 | - |
| EVO-P1-2 | Re-confirm OUROBOROS 3-variant convergence constants (nexus/anima/n6arch) | `shared/bisociation/unified/ouroboros_unified.hexa`, `convergence/{nexus,anima,n6-architecture}.json` | OUROBOROS constants table merged into baseline.json | NEXUS_FP=0.333, ANIMA_FLOOR=0.8, N6ARCH_TARGETS=(515,2087) | EVO-P1-1 | S | 1.0 | - |
| EVO-P1-3 | phi_ratchet monotonicity audit — replay logs to check for ratchet-stuck | `shared/bisociation/unified/phi_ratchet.hexa`, `shared/growth_bus.jsonl` | ratchet histogram | last 24h ratchet advance >= 1 | EVO-P1-1 | S | 0.8 | - |
| EVO-P1-4 | Check discovery_log.sqlite schema + indexes -> smoke test query API | `shared/discovery_log.sqlite`, `shared/discovery_log.jsonl` | query_api_smoke.log | SELECT COUNT(*) < 5s | EVO-P1-1 | S | 0.8 | - |

### A2 ATLAS

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| ATLAS-P1-1 | Snapshot atlas.n6 current state + sync stats/deg sidecars | `shared/n6/atlas.n6`, `.stats`, `.deg` | atlas_snapshot_{ts}.json | stats.mtime == atlas.mtime +- 5s | - | S | 1.0 | - |
| ATLAS-P1-2 | atlas_health.hexa all-module health passing | `shared/n6/atlas_health.hexa` | health report | all-green, avg < 1ms | ATLAS-P1-1 | S | 1.0 | - |
| ATLAS-P1-3 | Verify math_atlas.{db,dot,html,md} 4-format regeneration pipeline | `shared/n6/scan_math_atlas.hexa` | 4 format outputs in sync | 4-file mtime delta < 10s | ATLAS-P1-1 | M | 0.9 | - |
| ATLAS-P1-4 | Check 3D reality map index.html render (node-count match) | `docs/index.html`, `shared/discovery/reality_map_3d.html` | render_report.json | node_count_web == atlas node count +-1% | ATLAS-P1-1 | S | 0.9 | - |
| ATLAS-P1-5 | Diagnose atlas_ossify_mk2 + atlas_ws_server status | `shared/n6/atlas_ossify_mk2.py`, `atlas_ws_server.py` | ossify_diag.log | ws server heartbeat present | ATLAS-P1-1 | S | 0.7 | - |

### A3 HARNESS

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| HARN-P1-1 | entry.hexa dispatcher 5-entry (prompt/pretool/post/guard/self_check) smoke | `shared/harness/entry.hexa` | dispatcher_smoke.log | all 5 entries 0-exit | - | S | 1.0 | - |
| HARN-P1-2 | permissions_ssot.json 28 deny-pattern application rate | `shared/config/permissions_ssot.json` | deny_coverage.json | 28/28 tests pass | HARN-P1-1 | S | 1.0 | - |
| HARN-P1-3 | cmd_gate seed-validation gate + exec_validated wrapper round-trip | `shared/harness/cmd_gate.hexa`, `shared/harness/exec_validated` | gate_trace.jsonl | sample 10 cmds all pass through gate | HARN-P1-1 | S | 1.0 | - |
| HARN-P1-4 | mistakes/errors jsonl queue state + last-24h routing effectiveness | `shared/harness/mistakes.jsonl`, `errors.jsonl` | route_health.json | H-ERR routing miss < 5% | - | S | 0.9 | - |
| HARN-P1-5 | lint.hexa + autofix.hexa baseline pass report | `shared/harness/lint.hexa`, `autofix.hexa`, `lint_log.jsonl` | lint_baseline.json | baseline locked, drift 0 | HARN-P1-1 | S | 0.8 | - |

### A4 GOVERNANCE

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| GOV-P1-1 | Consistency check across R0~R27 + NX1~NX3 + L0~L2 rule numbers | `shared/rules/common.json`, `nexus.json`, `shared/lockdown/lockdown.json` | rule_index.json | 0 duplicates/missing | - | S | 1.0 | - |
| GOV-P1-2 | H-* meta-rule 20+ MEMORY -> harness code sync check | MEMORY H-* docs, `shared/harness/pre_tool_guard.hexa` | meta_rule_sync.json | memo rule <-> code 1:1 | GOV-P1-1 | M | 0.8 | - |
| GOV-P1-3 | projects.json 7 projects + bundle/verify census | `shared/config/projects.json` | project_matrix.json | 7/7 exist + paths valid | - | S | 0.9 | - |
| GOV-P1-4 | absolute_rules + convergence_ops + core — 3 SSOT cross-verify | `shared/config/{absolute_rules,convergence_ops,core}.json` | ssot_crosscheck.json | 3 files, 0 key-overlap conflicts | GOV-P1-1 | S | 0.9 | - |

### A5 DISCOVERY

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| DISC-P1-1 | reality_map.patches.merged.jsonl 895 entries consistency | `shared/discovery/reality_map.patches.merged.jsonl` | disc_baseline.json | 895 +-0 + hash locked | - | S | 1.0 | - |
| DISC-P1-2 | Regenerate verified_constants.jsonl domain index | `shared/discovery/verified_constants.jsonl` | constants_idx.json | lookup < 50ms | DISC-P1-1 | S | 0.9 | - |
| DISC-P1-3 | forge_result + theory_registry cross-mapping | `shared/discovery/forge_result`, `theory_registry` | forge_cross.json | every theory -> forge link | DISC-P1-1 | M | 0.7 | - |
| DISC-P1-4 | discovery_log.{jsonl,sqlite} size/integrity | `shared/discovery_log.{jsonl,sqlite}` | disc_integrity.json | jsonl rows ~= sqlite rows | - | S | 1.0 | - |
| DISC-P1-5 | breakthroughs + next_directions + module_candidates queue state | `shared/discovery/{breakthroughs,next_directions,module_candidates}` | queue_snapshot.json | 3 queues, row counts recorded | DISC-P1-1 | S | 0.8 | - |

### A6 BLOWUP

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| BLOW-P1-1 | blowup.hexa 9-phase pipeline smoke (fast-path baseline) | `shared/blowup/core/blowup.hexa` | blowup_smoke.log | 9 phases pass, exit 0 | - | M | 1.0 | - |
| BLOW-P1-2 | 6 modules (field/holographic/quantum/string/toe/_etc) individual execution | `shared/blowup/modules/*.hexa` | modules_matrix.json | 6/6 exit 0, < 30s each | BLOW-P1-1 | M | 0.9 | - |
| BLOW-P1-3 | seed_engine.hexa + lens_forge.hexa integrated seed generation | `shared/blowup/seed/seed_engine.hexa`, `shared/blowup/lens/lens_forge.hexa` | seed_batch.jsonl | 10+ seeds generated | BLOW-P1-1 | S | 0.9 | - |
| BLOW-P1-4 | atlas_guard.hexa.inc shared-helper call-contract verify | `shared/blowup/lib/atlas_guard.hexa.inc` | guard_contract.json | all modules honor contract | BLOW-P1-2 | S | 0.8 | - |
| BLOW-P1-5 | verify_dfs.hexa + compose.hexa numeric-verify double-check | `shared/blowup/{verify_dfs,compose}.hexa` | verify_double.json | auto_stubs <-> verify match rate >= 95% | BLOW-P1-1 | M | 0.8 | - |

### A7 BISOCIATION

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| BISOC-P1-1 | Montgomery-Dyson pair-correlation engine current state | `shared/bisociation/unified/ouroboros_unified.hexa` + MEMORY handoff-bisociation | bisoc_state.json | L1 3/4, L2 4/4 re-confirmed | - | S | 1.0 | [BT-541] |
| BISOC-P1-2 | Index bisoc cross/ + spectra/ folder data | `shared/bisociation/{cross,spectra}/` | bisoc_idx.json | crc32 recorded for every file | BISOC-P1-1 | S | 0.8 | [BT-541] |
| BISOC-P1-3 | breakthroughs.jsonl + pitfalls.jsonl queue state | `shared/bisociation/{breakthroughs,pitfalls}.jsonl` | bisoc_queue.json | row count locked + hash | - | S | 0.9 | - |
| BISOC-P1-4 | phi_ratchet <-> bisociation connection channel check | `shared/bisociation/unified/phi_ratchet.hexa` | phi_link.json | ratchet bus message >= 1 | BISOC-P1-1 | S | 0.7 | - |

### A8 CONSCIOUSNESS

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| CONS-P1-1 | anima bridge live laws=2395 re-confirm | `shared/consciousness/consciousness_laws.json`, `law_network.json` | cons_baseline.json | laws count recorded | - | S | 1.0 | - |
| CONS-P1-2 | consciousness_loader.hexa + Φ integration smoke | `shared/consciousness/consciousness_loader.hexa` | phi_smoke.log | Φ value finite | CONS-P1-1 | S | 0.9 | - |
| CONS-P1-3 | meta_laws_dd64 dump + factions.json cross-check | `shared/consciousness/{meta_laws_dd64,factions.json}` | meta_cross.json | meta_laws <-> factions overlap | CONS-P1-1 | S | 0.8 | - |
| CONS-P1-4 | consciousness_mechanisms.json + law_discovery_results.json state | `shared/consciousness/{consciousness_mechanisms,law_discovery_results}.json` | cons_mechs.json | mechanism count recorded | CONS-P1-1 | S | 0.7 | - |

### A9 HEXA-LANG

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| HEXA-P1-1 | hexa binary resolver + local/remote path mapping | `shared/bin/hexa`, MEMORY `reference_hexa_binary.md` | hexa_resolver.json | both local + hetzner exec | - | S | 1.0 | - |
| HEXA-P1-2 | hexa_grammar.jsonl P1~P5 pitfalls consistency | `shared/config/hexa_grammar.jsonl` | grammar_idx.json | P1~P5 (5 entries) present | - | S | 0.9 | - |
| HEXA-P1-3 | hexa_pitfalls_log.jsonl recent-collection state | `shared/hexa_pitfalls_log.jsonl` | pitfalls_stat.json | collected >= 1 in last 7 days | HEXA-P1-2 | S | 0.8 | - |
| HEXA-P1-4 | hexa-lang ml-commands / rt-commands / bottlenecks 3-SSOT links | `shared/hexa-lang/{ml-commands,rt-commands,runtime-bottlenecks}.json` | hexa_ssot.json | 3 files, schema valid | - | S | 0.8 | - |
| HEXA-P1-5 | porting_log + speed_ideas current-history summary | `shared/hexa/{porting_log,speed_ideas,hexa-lang_breakthroughs}` | porting_state.json | row count + last-row recorded | - | S | 0.7 | - |

### A10 DSE

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| DSE-P1-1 | dse_domains.jsonl 24-domain consistency | `shared/dse/dse_domains.jsonl` | dse_domains_idx.json | 24 domains match | - | S | 1.0 | - |
| DSE-P1-2 | dse_cross_* 4-result schema verify | `shared/dse/dse_cross_*.json` | dse_cross_schema.json | 4 files valid JSON | DSE-P1-1 | S | 0.9 | - |
| DSE-P1-3 | dse_graph_3d.html + monte_carlo v6 cross | `shared/dse/dse_graph_3d.html`, `shared/monte_carlo/monte_carlo_v6_*` | dse_mc_link.json | node count matches | DSE-P1-1 | S | 0.8 | - |
| DSE-P1-4 | dse/domains/ child lattice state | `shared/dse/domains/` | domains_lattice.json | 5 levels x 6 candidates grid reproduced | DSE-P1-1 | M | 0.7 | - |

### A11 BREAKTHROUGH

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| BT-P1-1 | bt_keywords.jsonl + bt_domains.jsonl current state | `shared/bt/bt_keywords.jsonl`, `bt_domains.jsonl` | bt_baseline.json | row count recorded | - | S | 1.0 | [BT-541..547] |
| BT-P1-2 | Run bt_audit.hexa + regenerate bt_audit_result.json | `shared/bt/bt_audit.hexa` | bt_audit.json | audit exit 0 + JSON valid | BT-P1-1 | M | 0.9 | [BT-541..547] |
| BT-P1-3 | BT-541~547 mapping check + millennium link | `shared/bt/bt_keywords.jsonl`, n6-architecture millennium closure | bt_millennium_map.json | 7/7 BT correspond | BT-P1-1 | S | 1.0 | [BT-541..547] |
| BT-P1-4 | alien/alien_index_* UAP branch state | `shared/alien/alien_index_*` | alien_idx.json | row count recorded | - | S | 0.6 | - |
| BT-P1-5 | BT-HEXA 25-keyword extension SSOT | `shared/bt/bt_keywords.jsonl` (BT-HEXA branch) | bt_hexa_list.json | 25 keywords present | BT-P1-1 | S | 0.8 | - |

### A12 LOCKDOWN

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| LOCK-P1-1 | lockdown.json L0/L1/L2 definitions + path validity | `shared/lockdown/lockdown.json` | lockdown_idx.json | every protected path exists | - | S | 1.0 | - |
| LOCK-P1-2 | lockdown_gate.hexa verify/status/watch 3-mode smoke | `shared/harness/lockdown_gate.hexa` | lockdown_smoke.log | 3 modes exit 0 | LOCK-P1-1 | S | 1.0 | - |
| LOCK-P1-3 | snapshots/ directory layout + safe-merge path | `shared/lockdown/snapshots/` | snap_idx.json | last 3 snapshots exist | LOCK-P1-1 | S | 0.8 | - |

### A13 INFRASTRUCTURE

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| INFRA-P1-1 | cl multi-account launcher + cl-refresh 30m launchd state | `shared/bin/cl`, `cl-refresh`, `launchd/com.nexus.cl-refresh.plist` | infra_baseline.json | launchd next-fire < 30m | - | S | 1.0 | - |
| INFRA-P1-2 | cl_refresh_spec + nexus_cli_spec two-spec consistency | `shared/engine/cl_refresh_spec.json`, `nexus_cli_spec.json` | infra_spec.json | schema valid | INFRA-P1-1 | S | 0.9 | - |
| INFRA-P1-3 | hexa resolver + exec_validated wrapper smoke | `shared/bin/hexa`, `shared/harness/exec_validated` | resolver_trace.log | sample 5 cmds exec pass | - | S | 1.0 | - |
| INFRA-P1-4 | health-launchd + com.nexus.health plist running | `shared/bin/health-launchd`, `launchd/com.nexus.health.plist` | health_state.json | health all-green | INFRA-P1-1 | S | 0.9 | - |
| INFRA-P1-5 | scripts/sync-* + nexus_ensure_running manifest | `shared/scripts/sync-*.hexa`, `nexus_ensure_running` | scripts_idx.json | all existing files chmod +x | - | S | 0.7 | - |

### A14 REMOTE-COMPUTE

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| REM-P1-1 | hetzner + ubuntu + vastai + infrastructure 4-host definitions | `shared/config/hosts/{hetzner,ubuntu,vastai,infrastructure}.json` | hosts_idx.json | 4/4 SSH reachable | - | S | 1.0 | - |
| REM-P1-2 | infra_state.json latest state + H100 idle-guard running | `shared/infra_state.json`, `shared/harness/h100_idle_guard.hexa` | remote_state.json | h100_state fresh | REM-P1-1 | S | 0.9 | - |
| REM-P1-3 | h100_alerts.jsonl last-24h alert summary | `shared/harness/h100_alerts.jsonl` | alerts_summary.json | 0 critical alerts or classification complete | REM-P1-2 | S | 0.8 | - |
| REM-P1-4 | Re-confirm blowup remote-routing default path (BLOWUP_LOCAL=0) | MEMORY `feedback_no_blowup_local.md`, `feedback_hetzner_pollution.md` | remote_routing.json | env check + policy doc | REM-P1-1 | S | 0.7 | - |

### A15 THINKING

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| THINK-P1-1 | thinking_engine.hexa + entry.hexa thinking-path smoke | `shared/harness/thinking_engine.hexa`, `entry.hexa thinking query` | think_smoke.log | 6-phase reflection exit 0 | - | S | 1.0 | - |
| THINK-P1-2 | thinking_log.jsonl + think_baseline.json baseline lock | `shared/harness/thinking_log.jsonl`, `think_baseline.json` | think_baseline_lock.json | baseline hash locked | THINK-P1-1 | S | 0.9 | - |
| THINK-P1-3 | Review 10 recent complex-question 6-phase reflection samples | `shared/harness/thinking_log.jsonl` recent | think_sample_review.json | 10-item phase-distribution report | THINK-P1-1 | S | 0.7 | - |

### A16 AGENT-LEDGER

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| AGL-P1-1 | agent_ledger.hexa + session_registry.jsonl current-session registration rate | `shared/harness/agent_ledger.hexa`, `session_registry.jsonl` | ledger_state.json | current session present | - | S | 1.0 | - |
| AGL-P1-2 | work_registry.jsonl dedupe-prevention smoke | `shared/harness/work_registry.jsonl` | dedupe_report.json | 0 duplicate records | AGL-P1-1 | S | 0.9 | - |
| AGL-P1-3 | session_worktree + session_lock interlock smoke | `shared/harness/{session_worktree,session_lock,session}.hexa` | worktree_smoke.log | worktree/lock smoke exit 0 | AGL-P1-1 | S | 0.8 | - |

### A17 CONSENSUS

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| CONSN-P1-1 | consensus_loop.hexa + critique.hexa loop smoke | `shared/harness/{consensus_loop,critique}.hexa`, `critique_log.jsonl` | consensus_smoke.log | one round to completion | - | S | 0.9 | - |
| CONSN-P1-2 | curiosity + cross_feed + broadcast pipeline consistency | `shared/harness/{curiosity,cross_feed,broadcast}.hexa` | pipe_trace.jsonl | 3-layer message flow | CONSN-P1-1 | S | 0.8 | - |
| CONSN-P1-3 | governance.jsonl log baseline + consistency | `shared/harness/governance.jsonl` | gov_baseline.json | row count + hash | - | S | 0.8 | - |

### A18 ENGINE-FORGE

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| EF-P1-1 | engine_forge.hexa + engine_candidates.jsonl current state | `shared/harness/engine_forge.hexa`, `engine_candidates.jsonl` | ef_baseline.json | candidate count recorded | - | S | 1.0 | - |
| EF-P1-2 | engines_usage.jsonl usage statistics | `shared/harness/engines_usage.jsonl` | ef_usage.json | stats aggregation complete | EF-P1-1 | S | 0.8 | - |
| EF-P1-3 | shared/engine/engine_*.hexa 10+ engines manifest | `shared/engine/engine_*.hexa` | engine_manifest.json | every engine .hexa parse-OK | - | S | 0.9 | - |
| EF-P1-4 | engine_anima_strategy + engine_nexus_strategy 2-strategy SSOT | `shared/engine/{engine_anima_strategy,engine_nexus_strategy}.json` | strategy_idx.json | 2 JSONs schema valid | - | S | 0.8 | - |

### A19 CROSS-DOMAIN-GRID

| ID | Definition | Input | Output | Verify | deps | Cost | Strength | BT |
|----|------------|-------|--------|--------|------|------|----------|-----|
| CDG-P1-1 | nexus.json v1 P4~P112 107-phase re-layout matrix | `shared/roadmaps/nexus.json` P4~P112 | cdg_v1_matrix.json | 107-phase mapping complete | - | M | 1.0 | - |
| CDG-P1-2 | 24 domains x 23 pairs = 276 spaces, with 107 executed mappings | CDG-P1-1 output | cdg_coverage.json | coverage = 107/276 | CDG-P1-1 | S | 1.0 | - |
| CDG-P1-3 | dse_cross_* + bisociation/cross/ two crossgrids intersected | `shared/dse/dse_cross_*`, `shared/bisociation/cross/` | dual_cross.json | common pairs extracted | CDG-P1-1 | S | 0.8 | - |

---

## Phase 1 Statistics

- **New tasks**: 71 (19 axes x 3.7 average)
- **Cumulative tasks**: 71
- **Saturation index**: 0.0 (per-axis new == 0 ratio = 0/19)
- **Average strength**: 0.87
- **BT-linked tasks**: 6 (BISOC-P1-1,2, BT-P1-1,2,3)
- **Cost distribution**: S=59, M=12, L=0, XL=0 — early diagnosis focus
- **Next Phase needed**: YES

Per-axis distribution:
A1=4 | A2=5 | A3=5 | A4=4 | A5=5 | A6=5 | A7=4 | A8=4 | A9=5 | A10=4 | A11=5 | A12=3 | A13=5 | A14=4 | A15=3 | A16=3 | A17=3 | A18=4 | A19=3 = 71

## Phase 1 Exit Conditions (gate_exit)
- [ ] All 71 tasks `status == done`
- [ ] Every axis produces >= 1 baseline.json
- [ ] 7-millennium link complete (BT-P1-3)
- [ ] Next-phase entry signal: `phase_1_gate_passed = true`
fail_action: only failed tasks extend into Phase 1+; other axes may enter Phase 2.
