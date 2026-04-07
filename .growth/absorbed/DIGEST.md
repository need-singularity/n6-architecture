# Absorbed Findings Digest

> Generated: 2026-04-04
> Source: ~/Dev/ready/n6-architecture/ (backup/corrupted version)
> Target: ~/Dev/n6-architecture/ (main)
> Scanner: ready-absorber NEXUS-6

---

## 1. Overview

| Metric | Value |
|--------|-------|
| Total absorbed files | 2,918 |
| Unique paths (deduplicated) | 507 |
| Redundancy ratio | 82.6% (same file across 16 agent worktrees) |
| Value grade: critical | 1,868 |
| Value grade: high | 1,050 |
| Worktree agent sessions | 16 |
| Files missing from main | 86 |
| New doc domains (not in main) | 4 (aerospace, cognitive-architecture, social-architecture, transportation) |
| New DSE .toml domains | 0 (all 54 already in main's 324) |
| New BTs beyond main (BT-133) | Title references to BT-135~225 (content truncated at 500 chars) |

---

## 2. Category Breakdown

| Category | Count | % | Notes |
|----------|------:|--:|-------|
| Docs (domain-specific) | 706 | 24.2% | Largest category, spans 30+ domains |
| DSE domains (.toml) | 540 | 18.5% | 54 unique domains, all already in main |
| Tools/Calculators | 425 | 14.6% | hexa-sim, hexa-ir, nexus, Python tools |
| Experiments | 265 | 9.1% | 11 missing from main |
| Papers | 120 | 4.1% | Paper drafts and notes |
| Evolution (Mk docs) | 116 | 4.0% | Mk-1 through Mk-5 evolution stages |
| Hypotheses | 110 | 3.8% | Domain-specific hypothesis files |
| Goal docs | 91 | 3.1% | Domain goal definitions |
| Breakthrough Theorems | 90 | 3.1% | BT files, some with BT-128~225 refs |
| Techniques | 90 | 3.1% | AI/ML technique implementations |
| Alien-level designs | 83 | 2.8% | Alien-10 certifications and discoveries |
| Verification | 71 | 2.4% | Hypothesis verification docs |
| README files | 52 | 1.8% | Domain READMEs |
| TECS-L files | 49 | 1.7% | Cross-repo TECS-L references |
| NEXUS-6 | 45 | 1.5% | Engine source and tests |
| Extreme hypotheses | 40 | 1.4% | Extreme-hypotheses.md files |
| HEXA-IR compiler | 11 | 0.4% | IR compiler source |
| Cross-repo (anima/sedi) | 9 | 0.3% | Anima and Sedi project files |
| Diff patches | 2 | 0.1% | Direct diff records |
| Other | 3 | 0.1% | Miscellaneous |
| **TOTAL** | **2,918** | **100%** | |

---

## 3. Top 20 Most Valuable Absorbed Items

Ranked by n6_score, deduplicated across agent worktrees.

| Rank | Score | Grade | Path | Status |
|------|------:|-------|------|--------|
| 1 | 50.0 | critical | docs/biology/cross-dse-analysis.md | MISSING from main |
| 2 | 50.0 | critical | experiments/experiment_mertens_dropout.py | MISSING from main |
| 3 | 50.0 | critical | docs/transportation/goal.md | MISSING (new domain) |
| 4 | 50.0 | critical | docs/aerospace/goal.md | MISSING (new domain) |
| 5 | 50.0 | critical | experiments/experiment_jordan_leech_moe.py | MISSING from main |
| 6 | 50.0 | critical | experiments/experiment_fibonacci_stride.py | MISSING from main |
| 7 | 50.0 | critical | tools/perfect_number_physics.py | MISSING from main |
| 8 | 50.0 | critical | docs/cognitive-architecture/cross-dse-analysis.md | MISSING (new domain) |
| 9 | 50.0 | critical | docs/compiler-os/physical-limit-proof.md | MISSING from main |
| 10 | 50.0 | critical | experiments/experiment_takens_dim6.py | MISSING from main |
| 11 | 50.0 | critical | docs/power-grid/README.md | Exists in main (update) |
| 12 | 50.0 | critical | tools/llm_architecture_constants_verify.py | Exists in main (update) |
| 13 | 50.0 | critical | tools/hexa-sim/analysis/compare.py | Exists in main (update) |
| 14 | 50.0 | critical | docs/display/physical-limit-proof.md | Exists in main (update) |
| 15 | 50.0 | critical | experiments/experiment_r_equals_1_search.py | Exists in main (update) |
| 16 | 50.0 | critical | docs/superconductor/evolution/mk-5-limit.md | Exists in main (update) |
| 17 | 50.0 | critical | docs/plasma-physics/kstar-steady-state-research.md | Exists in main (update) |
| 18 | 50.0 | critical | docs/energy-architecture/goal.md | Exists in main (update) |
| 19 | 50.0 | critical | docs/material-synthesis/physical-necessity-map.md | Exists in main (update) |
| 20 | 50.0 | critical | tools/nexus/tests/telescope_test.rs | Exists in main (update) |

---

## 4. Files Missing from Main (86 total)

### 4.1 New Domains (not in main docs/)

| Domain | Files | Key Content |
|--------|------:|-------------|
| aerospace | 30 | Goal, verification, evolution Mk docs, BTs |
| cognitive-architecture | 13 | Cross-DSE, physical-limit-proof, evolution, hypotheses |
| social-architecture | 10 | Hypotheses, evolution docs |
| transportation | 3 | Goal (HEXA-FUNCAR), verification, BT connections |

### 4.2 Missing Experiments (11)

| File | Score | Description |
|------|------:|-------------|
| experiments/experiment_mertens_dropout.py | 50.0 | Mertens dropout p=0.288 validation |
| experiments/experiment_jordan_leech_moe.py | 50.0 | J_2(6)=24 expert capacity MoE |
| experiments/experiment_fibonacci_stride.py | 50.0 | Fibonacci stride pattern |
| experiments/experiment_takens_dim6.py | 50.0 | Takens embedding d=6 diagnostic |
| experiments/experiment_radical_norm.py | 45.0 | Radical norm experiment |
| experiments/experiment_mobius_sparse.py | 45.0 | Mobius sparsity mu(6)=1 |
| experiments/experiment_partition_routing.py | 43.8 | Partition-based routing |
| experiments/experiment_rfilter_phase.py | 41.7 | R-filter phase detection |
| experiments/experiment_egyptian_linear_attention.py | 40.9 | Egyptian fraction linear attention |
| experiments/experiment_entropy_early_stop.py | 40.9 | Entropy early stopping |
| experiments/experiment_hcn_dimensions.py | 40.0 | HCN tensor alignment |

### 4.3 Missing Techniques (2)

| File | Score |
|------|------:|
| techniques/constant_time_stride.py | 45.5 |
| techniques/predictive_early_stop.py | 42.9 |

### 4.4 Missing Tools (15)

| File | Score | Type |
|------|------:|------|
| tools/perfect_number_physics.py | 50.0 | Python calculator |
| tools/hexa-ir/src/opt/mod.rs | 45.5 | HEXA-IR optimizer module |
| tools/nexus-ref/README.md | 37.5 | NEXUS-6 reference docs |
| tools/nexus/tests/cognitive_temporal_social_test.rs | 37.5 | NEXUS-6 test |
| tools/divisor_field_theory.py | 37.5 | Divisor field theory calculator |
| tools/hexa-ir/memory/feedback_hexa_lang_dse_recheck.md | 36.4 | HEXA-IR feedback |
| tools/nexus/tests/recent_discovery_test.rs | 35.7 | NEXUS-6 test |
| tools/hexa-ir/Cargo.toml | 33.3 | HEXA-IR build config |
| tools/nexus/src/graph/robotics_env_deep_nodes.rs | 30.0 | Graph nodes |
| tools/nexus/tests/extended_discovery_test.rs | 28.6 | NEXUS-6 test |
| tools/n6_uniqueness_tester.py | 25.0 | Uniqueness validation |
| tools/nexus/src/graph/recent_discoveries.rs | 9.1 | Discovery graph |
| tools/nexus/src/graph/energy_voltage_chain_nodes.rs | 9.1 | Energy chain nodes |
| tools/nexus/src/graph/transport_medical_nodes.rs | 0.0 | Transport/medical nodes |
| tools/nexus/src/graph/cognitive_temporal_social_nodes.rs | 0.0 | Cognitive nodes |

### 4.5 Missing Docs (alien-10, physical-limit, cross-DSE, evolution)

| File | Score | Category |
|------|------:|----------|
| docs/biology/cross-dse-analysis.md | 50.0 | Cross-DSE |
| docs/cognitive-architecture/cross-dse-analysis.md | 50.0 | Cross-DSE |
| docs/compiler-os/physical-limit-proof.md | 50.0 | Physical limit |
| docs/thermal-management/cross-dse-analysis.md | 38.9 | Cross-DSE |
| docs/blockchain/alien-10-certification.md | 36.4 | Alien-10 cert |
| docs/biology/alien-10-certification.md | 36.4 | Alien-10 cert |
| docs/plasma-physics/alien-10-certification.md | 36.4 | Alien-10 cert |
| docs/blockchain/industrial-validation.md | 36.4 | Industrial val |
| docs/cosmology-particle/alien-10-certification.md | 35.0 | Alien-10 cert |
| docs/biology/physical-limit-proof.md | 33.3 | Physical limit |
| docs/programming-language/alien-10-certification.md | 33.3 | Alien-10 cert |
| docs/energy-architecture/alien-10-certification.md | 31.8 | Alien-10 cert |
| docs/energy-architecture/alien-level-discoveries.md | 31.8 | Alien discoveries |
| docs/software-design/alien-10-certification.md | 31.2 | Alien-10 cert |
| docs/thermal-management/physical-limit-proof.md | 31.2 | Physical limit |
| docs/cognitive-architecture/physical-limit-proof.md | 30.0 | Physical limit |
| docs/programming-language/hexa-ir-spec.md | 44.4 | Spec doc |
| docs/programming-language/implementation-spec-2026-04-04.md | 27.3 | Impl spec |
| docs/plasma-physics/full-verification-matrix.md | 25.0 | Verification |
| docs/paper/blowup-invariant-core-2026-04-04.md | 14.3 | Paper draft |
| + 18 evolution docs (mk-2 through mk-5 across multiple domains) | 9-22 | Evolution |

---

## 5. Breakthrough Theorem Analysis

| Item | Current Main | Absorbed |
|------|-------------|----------|
| Main BT file range | BT-1 ~ BT-133 | Title claims BT-1 ~ BT-178 / BT-225 |
| Superconductor domain BTs | BT-135 ~ BT-139 (exists) | BT-135 ~ BT-139 (same) |
| Content beyond BT-133 | BT-135~139 in superconductor only | Headers only (500 char preview limit) |

Note: The absorbed files contain titles referencing up to BT-225, but the content_preview
is truncated at 500 characters. The actual BT definitions beyond BT-139 cannot be verified
from the absorbed JSON alone. The original source files at ~/Dev/ready/n6-architecture/
still exist and should be consulted for full content.

---

## 6. Recommended Integration Actions

### Priority 1: New Domains (HIGH VALUE)

These 4 domains exist in the absorbed backup but are completely absent from main:

1. **docs/aerospace/** (30 files) -- Full domain with goal, verification, evolution, BTs
2. **docs/transportation/** (3 files) -- HEXA-FUNCAR goal + verification + BT connections
3. **docs/cognitive-architecture/** (13 files) -- Cross-DSE, physical-limit, evolution, hypotheses
4. **docs/social-architecture/** (10 files) -- Hypotheses and evolution docs

Action: Copy from ~/Dev/ready/n6-architecture/ (originals still exist).

### Priority 2: Missing Experiments (11 files)

Experiment scripts for core techniques (mertens_dropout, jordan_leech_moe, takens_dim6,
mobius_sparse, etc.) are missing. These are implementations of techniques listed in
CLAUDE.md but without corresponding experiment files in main.

Action: Copy from ready source, verify they run.

### Priority 3: Missing Tools (15 files)

Key tools including perfect_number_physics.py, divisor_field_theory.py, n6_uniqueness_tester.py,
and several NEXUS-6 graph nodes and tests.

Action: Copy tools, rebuild NEXUS-6 (`cargo build --release`).

### Priority 4: Alien-10 Certifications and Physical Limit Proofs

11 alien-10-certification.md and 5 physical-limit-proof.md files across domains.
These represent completeness documentation for domains that have reached alien-level.

Action: Copy from ready source.

### Priority 5: Evolution Docs (Mk-2 through Mk-5)

18 evolution checkpoint documents across cryptography, compiler-os, network-protocol,
thermal-management, medical-device, power-grid, plasma-physics, biology, blockchain,
quantum-computing, programming-language, cognitive-architecture, and learning-algorithm.

Action: Copy from ready source.

### Priority 6: BT Content Verification

The absorbed BT files reference theorems up to BT-225, but content previews are truncated.
Main currently goes to BT-133 (with BT-135~139 in superconductor domain).

Action: Read full BT files from ~/Dev/ready/n6-architecture/docs/breakthrough-theorems.md
to determine if BT-134+ content actually exists or if the titles are aspirational headers.

### Low Priority: Existing File Updates

421 unique paths already exist in main but may have different content in the absorbed version.
These are mostly the same content replicated across 16 worktree agents.

Action: Diff selectively for high-score items (score >= 50, grade = critical).

---

## 7. Duplication Analysis

The 2,918 files reduce to 507 unique paths after deduplication. This is because
the backup contained 16 parallel agent worktrees that each had copies of the same files.

| Agent Worktree | Description |
|----------------|-------------|
| agent-a08b332c | Primary worktree (most files) |
| agent-a0ceda72 | Secondary |
| agent-a1c17143 | Secondary |
| agent-a2265680 | Secondary |
| agent-a254b3fa | Secondary |
| agent-a26748c0 | Secondary |
| agent-a5bc807d | Secondary |
| agent-a5d9f8bd | Secondary |
| agent-a83822a7 | Secondary |
| agent-a9650fcf | Secondary |
| agent-abb59ac1 | Anima worktree |
| agent-abeebc10 | Secondary |
| agent-acc2cafb | Secondary |
| agent-ad01689f | Secondary |
| agent-af641afc | Secondary |
| agent-afa95e1d | Secondary |

Most duplicated file: docs/carbon-capture/testable-predictions.md (x11 copies)

---

## 8. Summary

- **86 genuinely missing files** should be recovered from ~/Dev/ready/n6-architecture/
- **4 entire domains** (aerospace, transportation, cognitive-architecture, social-architecture) are absent from main
- **0 new DSE .toml domains** -- all 54 absorbed DSE domains already exist in main's 324
- **BT content beyond 133** needs manual verification from original source
- **82.6% of absorbed files are duplicates** across worktree agents
- Most valuable integration targets: new domains, missing experiments, missing tools
