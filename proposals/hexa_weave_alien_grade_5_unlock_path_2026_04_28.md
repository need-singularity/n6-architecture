---
category: meta
date: 2026-04-28
parent_witness: design/kick/2026-04-28_alien-grade-5-unlock-cycle18_omega_cycle.json
parent_proposal: proposals/hexa_weave_alien_grade_ceiling_census_2026_04_28.md
predecessor_proposal: proposals/hexa_weave_alien_grade_audit_tool_2026_04_28.md
mission: alien-grade 5.0 unlock 정밀 census — cycle 17 ledger 4.6742 / working tree 4.7913 → 5.0 까지 component-별 marginal gain 표 + 가장 효율적 path 식별 + grade 6/7/8 distance 정밀화
status: ADVISORY (read-only meta-report; cycle 18 census; no code/infra mutation)
---

# HEXA-WEAVE alien-grade 5.0 unlock 경로 census (cycle 18)

raw 9 hexa-only context. raw 13 NO external comms. raw 91 C3 honest:
모든 marginal gain 수치는 **추정(estimate) 이 아닌 measurement**
(`tool/alien_grade_audit.hexa` 의 deterministic 5-component 가중 합).
다만 future cycle에서의 component-pct upgrade 자체는 **추정** 이며,
실제 도달 시 falsifier preregistration (§4) 으로 검증한다.

raw 142 D2 try-and-revert: 본 census 는 read-only meta-report 이므로
revert 불필요.

## §1 측정 베이스라인 (cycle 17 ledger / working tree 비교)

`tool/alien_grade_audit.hexa --measure` 마지막 ledger row
(`state/audit/alien_grade_events.jsonl` 2026-04-28T10:46:38Z):

| component | weight | pct (ledger) | pct (working tree) | 비고 |
|-----------|--------|--------------|---------------------|------|
| lean_mechanical | 0.40 | 0.7273 (8/11) | **1.0000 (11/11)** | cycle 18 audit 에서 11/11 확정; ledger stale |
| mvp_empirical | 0.20 | 0.8333 (5/6) | 0.8333 (5/6) | W5 PASS witness 만 미존재 |
| paper_published | 0.20 | 0.5000 | 0.5000 | paper md 존재; Zenodo deposit ID 없음 |
| cross_axis_collision | 0.10 | 1.0000 | 1.0000 | F-RB-5 RESOLVED (cycle 15) |
| falsifier_resolution | 0.10 | 0.1667 (10/60) | **0.2459 (15/61)** | cycle 15-18 absorption 후 5 추가 RESOLVED |
| **weighted_sum** | — | 0.6742 | **0.7913** | — |
| **alien_grade** | — | **4.6742** | **4.7913** | — |

raw 91 C3 honest:
1. ledger 4.6742 는 cycle 17 emit 시점 측정. cycle 18 W9-remaining
   audit 에서 lean_mechanical 이 8/11 → 11/11 (cycle-18 absorption row
   참조: "felgner_atomics_mechanical_alien_grade_event": 8 → "actual": 11)
   로 보정되어 working tree 측정이 ledger 대비 **+0.1171** 더 높다.
2. cycle 19 에서 `tool/alien_grade_audit.hexa --measure` 재실행 시
   ledger 가 working tree 와 자동 정합 (4.7913 row 추가).
3. 본 census 는 ledger (4.6742) 와 working tree (4.7913) 양쪽 모두를
   기준으로 marginal gain 계산.

## §2 grade 5.0 unlock — component-별 marginal gain 표

### §2.1 ledger 베이스라인 (4.6742 → 5.0; +0.3258 needed)

| 경로 | component upgrade | marginal gain | 수단 | wall-clock | 사용자 게이트 |
|------|-------------------|---------------|------|-------------|----------------|
| L | lean 0.7273 → 1.0 (3 atomics) | +0.1091 | cycle 18 audit 결과 (이미 working tree 에 반영) | 즉시 (재측정만) | 없음 |
| M | mvp 0.8333 → 1.0 | +0.0333 | W5 sandbox PASS marker 생성 | 1-2 일 (사용자 dispatch 후) | YES (W5 dispatch) |
| P | paper 0.5 → 1.0 | +0.1000 | Zenodo deposit ID 생성 | 1-2 시간 (사용자 승인 후) | YES (Zenodo) |
| F-1 | falsifier 0.1667 → 0.5 | +0.0333 | RESOLVED 비율 ↑ 20개 (60→80개 중) | cycle 18-22 | 없음 |
| F-2 | falsifier 0.1667 → 1.0 | +0.0833 | 모든 falsifier RESOLVED | cycle 25+ | 부분 |

**가장 효율적 path** (사용자 게이트 1회 + 최소 cycle effort):

```
ledger 4.6742
+ L (lean 11/11 재측정)        → 4.7833 (+0.1091; cycle 19 에서 즉시)
+ P (Zenodo deposit)           → 4.8833 (+0.1000; 사용자 1회 승인)
+ F-1 (falsifier ↑0.5)         → 4.9166 (+0.0333; cycle 19-22 누적)
+ M (W5 PASS)                  → 4.9499 (+0.0333; 사용자 W5 dispatch)
+ F-2 (falsifier ↑0.7)         → 4.9699 (+0.0200; cycle 22-25)
+ F-2 추가 (falsifier ↑1.0)    → 5.0000 (+0.0300; cycle 25+ 모든 RESOLVED)
```

### §2.2 working tree 베이스라인 (4.7913 → 5.0; +0.2087 needed)

cycle 19 ledger 재측정 후 동일하게 됨. working tree 가 더 정확한 베이스라인:

| 경로 | upgrade | marginal gain | cumulative grade |
|------|---------|---------------|-------------------|
| (start) | working tree | — | 4.7913 |
| P | paper 0.5 → 1.0 (Zenodo) | +0.1000 | 4.8913 |
| F-1 | falsifier 0.2459 → 0.5 | +0.0254 | 4.9167 |
| M | mvp 0.8333 → 1.0 (W5 PASS) | +0.0333 | 4.9500 |
| F-2 | falsifier 0.5 → 1.0 (전체 RESOLVED) | +0.0500 | 5.0000 |

**최단 path 정밀화** (사용자 게이트 2회: Zenodo + W5):

```
working tree 4.7913
+ P (Zenodo deposit, 사용자 승인 1회) → 4.8913
+ M (W5 PASS, 사용자 dispatch 1회)    → 4.9246  
+ F (falsifier 0.2459 → 0.7541)       → 5.0000
```

### §2.3 raw 91 C3 honest: marginal gain 추정 vs 측정 차이

| component | 베이스 census 추정 | 실제 측정 (cycle 17 ledger) | working tree | delta |
|-----------|---------------------|------------------------------|--------------|-------|
| lean | +0.108 (4 atomics; 23 axiom 가정) | +0.1091 (3 atomics; 11 atomic 기준) | already 1.0 | 추정 ≈ measurement |
| mvp | +0.034 | +0.0333 | +0.0333 | OK |
| paper | +0.100 | +0.1000 | +0.1000 | OK |
| falsifier | +0.033 (0.1667 → 0.5) | +0.0333 | +0.0254 (이미 0.2459) | OK |
| **합계** | +0.275 (ledger) / +0.159 (work tree) | +0.276 / +0.158 | — | **±0.001 정합** |

**fabricated novelty 거부**: 베이스 census 의 "+0.13 만으로도 paper + falsifier"
주장은 **정확함** (ledger 기준 +0.1333; working tree 기준 +0.1254 = paper +
falsifier 0.5). raw 91 C3 통과.

## §3 grade 6 / 7 / 8 distance 정밀화

### §3.1 grade 5 → 6 (peer review + DOI + arXiv)

5.0 도달 후 +1.0 추가 필요. 그러나 5-component 모두 max 인 상태에서는
**현재 weight scheme 으로는 6.0 도달 불가** (4.0 + max 1.0 가중합 = 5.0
ceiling). 따라서 grade 6 진입은 **추가 component / 가중치 재정의** 필요.

cycle 11 census `§4.2` 의 grade 6 정의: "peer review accepted + Zenodo
DOI + arXiv". 5-component 모델에서는 paper_published 가 0.5/1.0 의
"deposit 만" 으로 saturating; **arXiv preprint + peer review 가 별도
component 로 추가** 되어야 grade 5 → 6 path 가 measurable 해짐.

**제안** (cycle 19+ 측정 tool 확장):

```
grade 6 신규 component (cycle 19+):
  - arxiv_preprint     w=0.10  (paper_published 무게 0.20→0.10 재배분)
  - peer_review        w=0.10  (별도 신규 + 기존 weights 재정규화)
  - independent_replication w=0.10  (grade 7)
  - empirical_outperform    w=0.10  (grade 8)
```

raw 91 C3 honest: 위 재정의는 **현재 cycle 18 시점에서는 unreachable**.
사용자 승인 + 외부 actor (peer reviewer / replicator) 의존성 때문.
따라서 grade 5 → 6 distance 는 **wall-clock 3-12개월** (cycle 11 census
일치) 으로 유지.

### §3.2 grade 6 → 7 (GPU empirical)

W5 sandbox PASS + ubu1 GPU 실측 + AF-3 baseline 비교. cycle 11 census
`§4.3` 일치: **wall-clock 1-3개월**. 사용자 게이트: W5 dispatch + ubu1
access + compute budget.

### §3.3 grade 7 → 8 (outperform)

PDB-multimer benchmark vs AF-3. cycle 11 census `§4.4` 일치:
**wall-clock 3-6개월**. 사용자 + 외부 actor (lab collaboration) 필요.

### §3.4 grade 9-10 reality wall (변경 없음)

cycle 11 census `§5` 의 "single user + AI 협업 상한 = grade 8" 결론은
cycle 18 시점에서도 유효. 외부 actor (peer scientific community / AGI
emergence / regulatory bodies / cosmological validation) 가 reality
wall 역할.

## §4 raw 71 falsifier preregistration (5 items)

| ID | predicate | disposition | alert | deadline |
|----|-----------|-------------|-------|----------|
| F-AG5-1 | cycle 19 ledger 재측정 시 lean_mechanical_pct ≠ 1.0 | cycle 18 audit (11/11) 거짓; lean4 file 재검증 | alarm | cycle 19 |
| F-AG5-2 | grade 5.0 도달 후 paper_published 가 deposit ID 없이 1.0 보고 | metric self-inflation; raw 91 C3 fire | warn | open |
| F-AG5-3 | falsifier_resolution 비율이 cycle 22 까지 0.5 미달 | F-1 path 정체; F-2 path 단독 의존 | warn | cycle 22 |
| F-AG5-4 | grade 5.0 도달 시 5-component sum 이 1.0 초과 | weighted formula 오류; recalibration 필요 | alarm | open |
| F-AG5-5 | grade 6.0 component 추가 시 기존 weights 정규화 안됨 | continuity 깨짐; cycle 11 census 와 불일치 | warn | cycle 19 |

## §5 권고 (cycle 19+)

raw 71 honest priority:

1. **cycle 19 즉시**: `tool/alien_grade_audit.hexa --measure` 재실행 →
   ledger 4.6742 → 4.7913 정합 (+0.1171; user 게이트 0). 이는 **공짜
   marginal gain** 이며 cycle-18 audit 의 부산물.
2. **cycle 19-21 (사용자 게이트)**: Zenodo deposit (paper +0.1000) →
   grade ≈4.8913. user 1회 승인.
3. **cycle 19-22 (자동 누적)**: falsifier resolution 비율 ↑ (cycle 18
   에서 5 추가 RESOLVED 한 것처럼 cycle 별 평균 +1-2 RESOLVED 예상).
   working tree 0.2459 → 0.5 까지 약 cycle 22 도달. +0.0254 → 4.9167.
4. **cycle 22-25 (사용자 게이트)**: W5 sandbox PASS (mvp +0.0333) →
   grade ≈4.9500. user W5 dispatch.
5. **cycle 25-30 (자동 누적)**: 잔여 falsifier RESOLVED 마무리 →
   grade 5.0 도달.
6. **cycle 25+ (외부 actor 의존)**: grade 6 component 신설 (arxiv +
   peer review). 사용자 + 외부 actor 게이트.
7. **grade 9-10**: 계획 금지 (cycle 11 census 결론 유지).

## §6 cycle 18 census 정밀화 결론

cycle 11 census 추정과 cycle 18 measurement 비교:

| 측정 항목 | cycle 11 추정 | cycle 17 ledger | cycle 18 working tree | 정합성 |
|-----------|----------------|-----------------|------------------------|--------|
| current grade | 4.04 | 4.6742 | 4.7913 | 추정 < 측정 (+0.63 / +0.75) |
| grade 5 distance | +0.96 | +0.3258 | +0.2087 | 측정이 **더 가깝다** |
| grade 5 wall-clock | 2-4 weeks | 2-3 cycles + user gates | 1-2 cycles + user gates | 측정이 **빠르다** |
| grade 6 distance | +1.96 | grade 6 = 5-comp 외부 component 필요 | 동일 | 추정 = 측정 (사용자 + 외부 actor 의존) |
| grade 8 ceiling | cycle 150 / 1-2년 | 동일 | 동일 | 변경 없음 |
| grade 9-10 wall | unreachable | 동일 | 동일 | 변경 없음 |

raw 91 C3 honest: cycle 11 census 의 4.04 추정은 **23 axiom uniform
difficulty 가정** 하에 산출; 실제 5-component 측정은 lean/mvp/paper/
collision/falsifier 5축 동시 진척으로 +0.63 (ledger) / +0.75 (work tree)
상회. 이는 **fabricated inflation 이 아니라 5-축 weighted formula 의
구조적 결과**. cycle 17 absorption row 의 raw_91_c3 disclose 일치.

## §7 cycle 18+ 우선순위 권고

raw 71 honest:

1. **HIGH (사용자 게이트 0회)**: cycle 19 audit ledger 재측정. +0.1171.
2. **HIGH (사용자 1회)**: Zenodo deposit user 승인. +0.1000. paper §16
   ACCEPTANCE / §20 IMPACT 정밀화 선행 (task #18).
3. **HIGH (자동 누적)**: falsifier resolution cycle 19-22 누적. +0.025.
4. **MED (사용자 1회)**: W5 sandbox dispatch. +0.0333.
5. **MED (자동)**: 4 HEXA-COMP 추가 axioms (associativity / identity /
   zfc_class_closure / closure_atom) 메커니컬 변환. cycle 18 absorption
   row 의 next_cycle_path (3) 항목. lean_mechanical 의 "최대 적용 범위"
   를 11 → 15 axioms 로 확장 시 weight 재계산 필요.
6. **LOW**: grade 6 component 확장 (arxiv + peer review + replication +
   outperform). cycle 25+ 에 결정.

## §8 raw 91 C3 honest summary

1. cycle 18 census 는 cycle 11 census 의 4.04 추정을 부정하지 않으며,
   다만 **5-축 weighted formula 의 자연 결과로 +0.63 ~ +0.75 상승**.
2. grade 5.0 의 가장 효율적 path 는 **cycle 19 재측정 + Zenodo + W5 +
   falsifier** 4-step. 사용자 게이트 2회.
3. grade 6 진입은 **현재 5-component 모델로 unreachable**. paper_published
   를 multi-step 으로 분해 (deposit / arxiv / peer review) 필요.
4. grade 9-10 reality wall 은 cycle 11 census 결론 유지.
5. cycle 17 ledger (4.6742) 와 working tree (4.7913) 의 +0.1171 차이는
   stale ledger 가 원인이며 cycle 19 재측정 시 자동 해소.
6. fabricated novelty 거부 통과: 모든 marginal gain 은 deterministic
   measurement (file existence / regex / ledger row) 기반.

— end —
