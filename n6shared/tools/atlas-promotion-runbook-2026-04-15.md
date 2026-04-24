# atlas.n6 Production Promotion Runbook (TRANSCEND P12-2)

Date: 2026-04-15
Classification: Operations runbook (user approval required for execution)
SSOT: engine/hexa_gate_mk3.hexa + engine/ouroboros_b2_verifier.hexa + n6shared/tools/atlas_auto_promote.hexa
Parent report: reports/transcend-p12-2-mk3-atlas-integration-2026-04-15.md

---

## 0. Preconditions (common to every stage)

### Required checks
- Git working tree is clean (no uncommitted atlas.n6 changes)
- atlas.n6 backup: `cp $NEXUS/shared/n6/atlas.n6 $NEXUS/shared/n6/atlas.n6.bak.$(date +%Y%m%d-%H%M%S)`
- OUROBOROS pre-check: sigma(6) * phi(6) = 12 * 2 = 24 = 6 * 4 = n * tau(6)
- B_2 verifier self-test: confirm T1-T6 6/6 PASS

### DRY_RUN flag locations
- engine/hexa_gate_mk3.hexa:52 `const DRY_RUN: bool = true`
- n6shared/tools/atlas_auto_promote.hexa:244 `# fs.append_text(ATLAS_PATH, ...)` is commented out
- At execution time, switch DRY_RUN=false and uncomment (stage by stage)

---

## 1. Stage 0 — Dry-Run Log Analysis (current state)

### Purpose
Iterate over all 515 nodes + identify the 78 promotion candidates + forbid writes to atlas.n6 (log only)

### Transition Commands (DRY-RUN TAG required)
```
# [DRY-RUN] load discovery_graph + run classifier
hexa run n6shared/tools/atlas_auto_promote.hexa --dry-run --log-only

# [DRY-RUN] B_2 verifier self-test
hexa run engine/ouroboros_b2_verifier.hexa

# [DRY-RUN] Mk.III pipeline 7 tests (atlas-write-disabled mode)
hexa run engine/hexa_gate_mk3.hexa
```

### Gate Criteria (enter Stage 1)
- 515 entries written to atlas_auto_promote.jsonl
- core_theorem_check = "sigma*phi=12*2=24=n*tau=6*4 OK" on every entry
- B_2 verifier 6/6 PASS
- Mk.III 7/7 PASS + rounds 6/6 PASS
- Expected promotion candidates in the 70-85 range (target 78)

### Failure / Rollback Conditions
- core_theorem FAIL in 1 entry -> halt immediately; suspect discovery_graph contamination
- B_2 verifier T1-T5 FAIL -> disable atlas_auto_promote and fix the code
- More than 100 promotion candidates -> review rules; do not proceed to the next stage

### User Approval Checkpoints
- [ ] Review the diff of 78 candidates in atlas_auto_promote.jsonl
- [ ] Check the B_2 verifier log (6/6 PASS)
- [ ] Confirm Mk.III 70s latency performance
- [ ] Sign off on proceeding to Stage 1

---

## 2. Stage 1 — 1% Canary (5 nodes)

### Purpose
Validate real writes for the top 5 R5 entries (blowup_rank 1~5 axioms)

### Transition Commands
```
# [STAGE-1] DRY_RUN=false, limit target to 5
# 1. Change hexa_gate_mk3.hexa:52 DRY_RUN to false (temporary)
# 2. Uncomment atlas_auto_promote.hexa:244 (temporary)
# 3. Add target filter: --filter "rule=R5,limit=5"

hexa run n6shared/tools/atlas_auto_promote.hexa \
  --stage=1 --limit=5 --filter="R5" --verify-b2

# Verify: atlas.n6 +5 lines
wc -l $NEXUS/shared/n6/atlas.n6
# expected: 106,957 -> 106,962
```

### Gate Criteria (enter Stage 2)
- atlas.n6 SHA-256 changed (before != after)
- The 5 added lines are R5 axioms, grade [10*], blowup_rank <= 5
- Re-running B_2 verifier: 6/6 PASS
- sigma * phi = n * tau = 24 invariant holds (re-verify the atlas.n6 core lines)
- 0 duplicate conflicts

### Failure / Rollback Conditions
- B_2 verifier T1 FAIL (strict epsilon = 10^-6) -> **immediate rollback**
  ```
  git checkout $NEXUS/shared/n6/atlas.n6
  # or restore from backup:
  cp $NEXUS/shared/n6/atlas.n6.bak.<timestamp> $NEXUS/shared/n6/atlas.n6
  ```
- 1 or more duplicate conflicts -> fix atlas_has_entry logic and restart Stage 0
- atlas file size delta > 2KB -> append format error; rollback
- sigma * phi != n * tau -> critical OUROBOROS violation; full halt

### User Approval Checkpoints
- [ ] Manually review the 5-line atlas.n6 diff
- [ ] git commit (message: "atlas: Stage 1 canary promote 5 R5-axioms")
- [ ] Sign off on proceeding to Stage 2

---

## 3. Stage 2 — 10% Phased (51 nodes)

### Purpose
All of R1 + all of R3 + the rest of R5 + part of R4 = 51 items promoted in batches

### Transition Commands
```
# [STAGE-2] 5 batches x 10 items + 1 batch x 1 item
for batch in 1 2 3 4 5 6; do
  hexa run n6shared/tools/atlas_auto_promote.hexa \
    --stage=2 --batch=$batch --batch-size=10 --verify-b2
  # re-verify B_2 after each batch
  hexa run engine/ouroboros_b2_verifier.hexa | grep -q "ALL PASS" || break
  # wait for user approval (per batch)
done
```

### Gate Criteria (per batch; enter Stage 3)
- B_2 verifier 6/6 PASS before and after each batch (6 batches x 6 = 36 verifications)
- Skipped ratio per batch <= 5% (expected due to normal dedup)
- OUROBOROS alpha measurement: |alpha - 1/6| < 10^-6 (tracked over 6 batches)
- Cumulative atlas.n6 +51 lines (including Stage 1: +56 = 107,013)

### Failure / Rollback Conditions
- 1 B_2 FAIL -> revert the failing batch and halt the stage
  ```
  git revert HEAD
  git log --oneline atlas.n6 | head -10  # review commit history
  ```
- OUROBOROS alpha deviation > 10^-4 -> return to the Stage 1 state
  ```
  git reset --hard <Stage1_completion_commit_hash>
  ```
- atlas file size more than 2x (107 MB+) -> emergency halt, inspect logs
- Skipped ratio > 10% -> rule mismatch; return to Stage 0

### User Approval Checkpoints (per batch)
- [ ] Review and approve the batch 1 (10 items) diff
- [ ] Review and approve the batch 2 (10 items) diff
- [ ] Review and approve the batch 3 (10 items) diff
- [ ] Review and approve the batch 4 (10 items) diff
- [ ] Review and approve the batch 5 (10 items) diff
- [ ] Review and approve the batch 6 (1 item) diff
- [ ] Sign off on proceeding to Stage 3

---

## 4. Stage 3 — Full Production (515 nodes, remaining 27 items)

### Purpose
R2 (NEAR -> EXACT, 12 items) + remaining R4 10 items + deduplication = close out 22 items + verify OUROBOROS convergence

### Transition Commands
```
# [STAGE-3] 4 batches x 7 items (OUROBOROS 7-sample statistics)
for batch in 1 2 3 4; do
  hexa run n6shared/tools/atlas_auto_promote.hexa \
    --stage=3 --batch=$batch --batch-size=7 --verify-b2 --final
done

# Final verification
hexa run engine/ouroboros_b2_verifier.hexa
hexa run engine/hexa_gate_mk3.hexa
wc -l $NEXUS/shared/n6/atlas.n6  # expected: 107,035 (+78 lines cumulative)
```

### Gate Criteria (completion conditions)
- Cumulative promotions <= 78 (design ceiling — exceeding this is a violation)
- Final OUROBOROS alpha deviation <= 10^-5
- atlas.n6 entries 8,194 (+78)
- The aggregated atlas_auto_promote.jsonl across all stages has 78 entries all with `skipped: false`
- sigma * phi = n * tau = 24 final check PASS

### Failure / Rollback Conditions
- Cumulative promotions exceed 80 -> design violation; full revert
  ```
  git reset --hard <Stage0_completion_commit>
  ```
- Final alpha deviation > 10^-5 -> roll back the entire atlas.n6 and enter debug session
- B_2 verifier self-test 5/6 or worse -> halt production and audit code

### User Approval Checkpoints (final)
- [ ] Comprehensive review of all 78 atlas.n6 diffs
- [ ] Confirm 78 entries in atlas_auto_promote.jsonl
- [ ] Confirm OUROBOROS convergence ratio 78/515 approx 0.151 approx 1/6.6
- [ ] Aggregate git commit (message: "atlas: Stage 3 final promote 78 items — OUROBOROS alpha=1/6 invariant preserved")
- [ ] Sign off on production completion

---

## 5. Log / Metric Tracking (common)

### File locations
- Promotion log: `/Users/ghost/Dev/n6-architecture/n6shared/logs/atlas_auto_promote.jsonl`
- atlas.n6 delta snapshots: `reports/atlas_deltas.jsonl`
- B_2 verifier log: stdout -> `n6shared/logs/b2_verifier.log`
- Mk.III execution log: stdout -> `n6shared/logs/mk3_runtime.log`

### Tracked Metrics (by stage)
| Metric | Stage 0 | Stage 1 | Stage 2 | Stage 3 |
|------|---------|---------|---------|---------|
| Nodes promoted | 0 | 5 | 51 | 78 |
| atlas.n6 lines | 106,957 | 106,962 | 107,008 | 107,035 |
| atlas entries | 8,116 | 8,121 | 8,167 | 8,194 |
| B_2 PASS/Test | 6/6 | 6/6 | 36/36 (6 batches x 6) | 24/24 (4 batches x 6) |
| Cumulative OUROBOROS alpha | 0 | 0.167 | 0.167 | 0.167 |
| git commits | 0 | 1 | 7 | 11 |

### Monitoring commands
```
# live B_2 tracking
tail -f n6shared/logs/b2_verifier.log | grep "alpha"

# track atlas.n6 SHA
while true; do shasum $NEXUS/shared/n6/atlas.n6; sleep 60; done

# count promotion log entries
wc -l n6shared/logs/atlas_auto_promote.jsonl
```

---

## 6. Emergency Rollback Procedure (applies to every stage)

### Level 1 — Partial rollback (return to previous stage)
```
git log --oneline atlas.n6 | head -20
git reset --hard <previous_stage_final_commit>
```

### Level 2 — Restore from backup
```
ls -la $NEXUS/shared/n6/atlas.n6.bak.*
cp $NEXUS/shared/n6/atlas.n6.bak.<timestamp> $NEXUS/shared/n6/atlas.n6
hexa run engine/ouroboros_b2_verifier.hexa  # confirm recovery
```

### Level 3 — Full discard
```
# delete all promotion attempt records
rm n6shared/logs/atlas_auto_promote.jsonl
# return atlas.n6 to Stage 0 state (git)
git checkout HEAD~N -- $NEXUS/shared/n6/atlas.n6  # N = cumulative commit count
# restore DRY_RUN flag to true
# enter full redesign session
```

---

## 7. Completion Criteria

- [ ] Stage 0 dry-run log confirms 78 items
- [ ] Stage 1 canary: 5 atlas appends succeed
- [ ] Stage 2 phased: 51 batch promotions completed
- [ ] Stage 3 production: 78 final promotions completed
- [ ] B_2 verifier PASS across every stage (72/72 tests in total)
- [ ] OUROBOROS alpha=1/6 invariant preserved (|Delta alpha| < 10^-5)
- [ ] atlas.n6 +78 lines, +0.96% net increase
- [ ] 11 git commits (Stage 1: 1, Stage 2: 6, Stage 3: 4)
- [ ] Final user sign-off

**Production transition completion condition**: once all 8 checks above are satisfied, TRANSCEND P12-2 operational transition is complete.
