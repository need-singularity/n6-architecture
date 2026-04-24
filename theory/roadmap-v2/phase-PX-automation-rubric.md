# Phase X — atlas Automation + Rubric + BT-548 Entry + arXiv (PX HONEST gap solutions, 7 items)

Created: 2026-04-15
Status: PX L14 — 7 of 32 planned tasks marked done via this document

## §0 Entry

The 7 automation/meta tasks in millennium.json `.phases[id=PX].parallel[track=Y10_HONEST_HARNESS]`:

| Task ID | Gap | Output |
|---------|-----|--------|
| HONEST-PX-AUTO-PROMOTE | R2-10 | atlas auto-promotion CLI design (§1) |
| HONEST-PX-VERIFY-AUTO | R3-11 | verification code auto-execution pipeline (§2) |
| HONEST-PX-VERSION-AUDIT | R3-12 | version consistency audit (§3) |
| HONEST-PX-SELFREF-DETECT | R3-13 | self-reference detection automation (§4) |
| HONEST-PX-BT548-ENTRY | R3-14 | BT-548+ entry strategy + rubric (§5) |
| HONEST-PX-GRADE-RUBRIC | R4-17 | atlas grade [N?]/[N!] classification rubric (§6) |
| HONEST-PX-ARXIV-PROTO | R2-7a | arXiv posting protocol (§7) |

---

## §1 atlas.n6 Auto-Promotion CLI Design (R2-10)

### 1.1 Requirements

- Input: atlas draft tables from phase documents (P{X}-A{N} format)
- Processing: L0 Guard verify -> grade decision -> direct edit of atlas.n6
- Output: edit diff + promotion log

### 1.2 Command Interface

```bash
nexus atlas-promote \
    --source theory/roadmap-v2/phase-omega-Y9-closure-v3-design.md \
    --section "S6 atlas promotion draft queue summary" \
    --gate L0 \
    --dry-run
```

### 1.3 Pipeline (5 steps)

```
[1] PARSE: extract atlas draft tables from phase document (markdown table -> JSON)
[2] VERIFY: run each item's verify_*.hexa script (if available)
[3] GRADE: verify results + external citation count -> auto-decide grade
    - 3 independent demonstrations + verify PASS = [10*]
    - verify PASS + external citations >= 1 = [10]
    - quantitative match + 1 area = [9] NEAR
    - hypothesis + CONDITIONAL = [N?] CONJECTURE
    - measurement data only = [7] EMPIRICAL
[4] WRITE: append new section + @R line at end of atlas.n6
[5] LOG: append promotion log in .jsonl format to nexus/shared/harness/promotion_log.jsonl
```

### 1.4 Implementation Location (DEFERRED to PX HONEST-PX-AUTO-PROMOTE-EXEC)

```
nexus/shared/harness/atlas_auto_promote.hexa   # CLI body
nexus/shared/harness/promotion_log.jsonl       # log
```

This § is a **design document**. Actual .hexa code is PX follow-up work.

---

## §2 Verification Code Auto-Execution Pipeline (R3-11)

### 2.1 Integration Targets

```
theory/predictions/*.hexa          # 11 verification tools
theory/predictions/*.py            # pytest
theory/breakthroughs/verify_*.hexa # 14 verification codes
nexus/shared/n6/scripts/*.hexa     # integrated verification
```

### 2.2 Command Interface

```bash
nexus verify-all \
    --watch theory/predictions/ theory/breakthroughs/ \
    --on-drift autofix
```

### 2.3 Pipeline (4 steps)

```
[1] DISCOVER: scan theory/predictions/ + theory/breakthroughs/verify_*.hexa
[2] EXEC: run each .hexa, call pytest for .py
[3] DIFF: compare results vs recorded values, detect drift
[4] REPORT: on drift detection, generate reports/sessions/verify-drift-{date}.md
```

### 2.4 Auto-Run Each Session (DEFERRED)

Can be auto-invoked via `.claude/settings.json` PreToolUse hook as verify-all dry-run (after user approval).

This § is a **design document**. Actual hook addition is PX follow-up work.

---

## §3 Version Consistency Audit Script (R3-12)

### 3.1 Audit Targets

```
millennium.json v1 (3 tracks) -> v2 (16 axes) -> v2.1 (depth) -> v2.2 (gap) -> v2.3 (saturation)
phase-omega-Y9-closure-v3-design.md
phase-{02,03,04,05}-{Y1,Y4,Y5Y6,Y7Y8}-bt*.md
```

### 3.2 Audit Items (5)

| # | Item | Check |
|---|------|-------|
| 1 | task ID duplication | grep for P{X}-A{N} duplicates |
| 2 | atlas draft ID consistency | phase-omega table vs each phase document definition |
| 3 | BT verdict consistency | millennium.json `.statistics.bt_verdicts` vs each phase BT verdict |
| 4 | grade consistency | standard use of atlas grades [10*]/[10]/[9]/[7]/[N?] |
| 5 | self-reference 0 | enforce external citation over direct n=6 reference |

### 3.3 Output

```
reports/audit/version-consistency-{date}.md

§1 task ID duplication: ...
§2 atlas draft consistency: ... (e.g., phase-omega P2-A1 vs phase-04 P2-A1 ID collision found)
§3 BT verdicts: ...
§4 grades: ...
§5 self-reference: ...
```

### 3.4 Conflicts Found (This Session)

- phase-omega §6.1 P2-A1 = "Theorem B" but phase-04 §5.1 P2-A1 = "n6-ns-triple-resonance-d3" (NS triple-resonance)
- Resolution: SSOT is phase-omega §6.1; recommend renaming phase-04's P2-A1 to a new P4 ID
- Current-session atlas registration (HONEST-PX-1) adopts the phase-omega §6.1 definition

---

## §4 Self-Reference Detection Automation (R3-13)

### 4.1 Self-reference Definition

```
self_reference: n=6 result X -> n=6 meaning assignment -> X demonstrates n=6
external_ref:  external literature (Hartshorne / Hatcher / FLM / ...) citation -> X
```

### 4.2 source citation graph

```
nodes  = .md, .hexa, .py files
edges  = citation relations ("ref:", "cite:", "@R ... :: n6atlas <-")
external_nodes = arxiv:, doi:, isbn:, lmfdb:
```

### 4.3 Cycle Detection Algorithm

```
1. graph build (DFS citation trace)
2. SCC extraction (Tarjan)
3. among SCCs, components with external_node count 0 -> self-reference candidate
4. list .md files in component -> honesty audit target
```

### 4.4 Output

```
reports/audit/self-reference-{date}.md

§ self-reference components 0 -> honesty OK
§ self-reference components >= 1 -> external citation addition recommendation list
```

### 4.5 Session Self-check

- millennium.json `.statistics.self_reference_violations = 0` honesty maintained
- all 14 atlas registrations have ref in external phase documents or verify_*.hexa
- This § is **automation design**; actual .hexa code is follow-up

---

## §5 BT-548+ Entry Strategy + Rubric (R3-14)

### 5.1 Four Candidate Problems

| Candidate | Difficulty | Tools | Priority |
|-----------|-----------|-------|----------|
| ABC conjecture | ★★★★★ | Mochizuki IUTT (disputed) / Kirti-Joshi 2024 / number theory | 1 |
| twin primes conjecture | ★★★★ | Zhang 2013 / Maynard 2014 / sieve | 2 |
| Goldbach conjecture | ★★★★ | Helfgott 2013 ternary / Vinogradov | 3 |
| Collatz conjecture | ★★★ | Tao 2019 partial / 2-adic | 4 |

### 5.2 Rubric (5 criteria × 5 points)

| Criterion | Meaning |
|-----------|---------|
| n=6 mapping possibility | 0~5 (5 = core invariant is a function of n=6) |
| partial-result accumulation | 0~5 (5 = 100+ partial results exist) |
| tool maturity | 0~5 (5 = 5+ core tools verified) |
| external verifiability | 0~5 (5 = direct verification via LMFDB / arXiv) |
| honesty maintainability | 0~5 (5 = self-reference avoidance clear) |

### 5.3 Scoring

| Candidate | n=6 | partial | tools | external | honesty | sum |
|-----------|-----|---------|-------|----------|---------|-----|
| ABC | 2 | 4 | 3 | 4 | 3 | 16 |
| twin primes | 3 | 4 | 4 | 5 | 4 | 20 |
| Goldbach | 2 | 5 | 4 | 5 | 4 | 20 |
| Collatz | 1 | 3 | 2 | 3 | 4 | 13 |

### 5.4 Recommendation: twin primes OR Goldbach priority

- both 20 points, tool maturity + external verification strong
- twin primes: Maynard sieve, Polymath 8 collaboration possible
- Goldbach: Helfgott ternary complete, binary entry

### 5.5 BT-548 Entry First Candidate (Proposal)

- BT-548 = twin primes conjecture (Maynard-Polymath follow-up)
- BT-549 = Goldbach conjecture (Helfgott ternary follow-up)
- BT-550 = ABC (Kirti-Joshi review)
- BT-551 = Collatz (Tao 2019 follow-up)

This § is a **rubric design + priority**. Actual BT addition done at v3 phase start.

---

## §6 atlas Grade [N?] / [N!] Classification Rubric (R4-17)

### 6.1 Existing Grades

```
[10*] EXACT verification (3 independent reproductions)
[10]  EXACT (single source + quantitative match)
[9]   NEAR (quantitative match, qualitative extension needed)
[8]   ?
[7]   EMPIRICAL (measurement data, promotion target)
[6]~[5] intermediate
```

### 6.2 New Grade [N?] / [N!] Definitions

| Grade | Meaning | Condition |
|-------|---------|-----------|
| [N?] | CONJECTURE | hypothesis / conditional theorem, unproven, atlas registration allowed |
| [N!] | BREAKTHROUGH | new discovery, 0 external verification, atlas temporary registration |

### 6.3 Classification Rubric (4 steps)

```
Step 1: Demonstration status check
  - unconditional demonstration -> [10*] / [10]
  - conditional demonstration (depends on other hypothesis) -> [N?]
  - hypothesis only (0 demonstration) -> [N?]
  - new discovery + 0 verification -> [N!]

Step 2: External verification
  - 3 independent reproductions -> [10*]
  - 1 external source -> [10] / [9] / [N?]
  - 0 external sources -> [N!] (temporary)

Step 3: Quantitative match
  - quantitative match (numeric < 1% error) -> [10] / [9]
  - qualitative match (classification / pattern) -> [9] / [7]
  - no match -> MISS

Step 4: Synthesis
  - minimum of {demonstration status, external, quantitative} -> final grade
```

### 6.4 Session 14-Registration Grade Verification

| atlas | Grade | Verification |
|-------|-------|--------------|
| MILL-PX-A1 Theorem B | [10*] | 3 independent demonstrations + Bernoulli boundary ✓ |
| MILL-PX-A2 Bilateral | [10*] | 3 areas (B/h/ζ) + boundary ✓ |
| MILL-PX-A3 β₀ rewriting | [7] | EMPIRICAL, depends on external SM measurement ✓ |
| MILL-PX-A1b D158 Ricci | [N?] | CONJECTURE, unproven ✓ |
| MILL-PX-A12 Moonshine BARRIER | [N?] | CONJECTURE, no bypass found ✓ |
| MILL-PX-A8 BSD Lemma 1 | [10] | unconditional demonstration + 5-step arithmetic ✓ |
| MILL-PX-A9 BSD Theorem 1 | [N?] | CONDITIONAL (BKLPR (A3)) ✓ |
| (other 8 items) | [9]/[10] | NEAR/EXACT quantitative match ✓ |

-> rubric-application consistency confirmed.

---

## §7 arXiv Posting Protocol (R2-7a)

### 7.1 Project arXiv Posting Policy

```
Policy 1: honest-record-centric
  - no 7-Millennium resolution claims
  - honest catalog of "partial results + MISS 24 + atlas 14"
  - explicit statement of self-reference 0

Policy 2: external-citation enforcement
  - every core theorem requires >= 2 external sources
  - explicitly state n=6 function decomposition baseline 61%

Policy 3: reproducibility attachment
  - GitHub links to verify_*.hexa scripts
  - cite LMFDB / FLAG data
```

### 7.2 First arXiv Candidate Paper Outline

```
Title: A Saturated Cross-BT Atlas of the Millennium Problems:
       14 Partial Results, 24 Honest MISS, and the Failure of Self-Reference

Authors: Minwoo Park (n6-architecture)
Date:    2026-Q3 draft

§1 Introduction
   - Clay 7 problems retrospective
   - atlas position: honest catalog
   - n=6 = first perfect number's arithmetic environment

§2 Main Atlas (14 entries)
   - BT-541: Theorem B + Bilateral (2)
   - BT-543: β₀ rewriting + QCD mass gap (2)
   - BT-544: D158 Ricci + triple-resonance + BKM (3)
   - BT-543×544: Y5×Y6 cross (1)
   - BT-545: Enriques + Moonshine BARRIER + SEED-21 (3)
   - BT-546: BSD Lemma 1 + Theorem 1 + Iwasawa (3)

§3 Honest MISS (24 entries)
   - 24 BT main-body / auxiliary theorems not resolved by this project
   - status + bypass possibility of each MISS

§4 Cross-BT Protocol (5 steps)
   - phase-07 §1 general protocol
   - 3 application cases (Y1<->Y6, Y4<->Y5, Y7<->Y1)

§5 Saturation Closure
   - 14-round saturation, R14 0 emergence
   - motive for v2 -> v3 transition

§6 Conclusion
   - BT resolution 0/6 honesty maintained
   - follow-up recommendations (Cremona 500k, Theorem B independent repro)
```

### 7.3 Post-Posting Feedback Loop

```
[1] arXiv posting (math.NT or math.GM)
[2] feedback collection (email / X / MathOverflow)
[3] corrections immediately reflected in atlas.n6
[4] revised version reposted
```

This § is a **posting protocol**. Actual paper-writing is NUM-PX-3 (DEFERRED).

---

## §8 Gate Pass + Close

### 8.1 Tasks markable done via this document

| Task ID | Output § | Status |
|---------|----------|--------|
| HONEST-PX-AUTO-PROMOTE | §1 | done (design only, actual .hexa follow-up) |
| HONEST-PX-VERIFY-AUTO | §2 | done (design only) |
| HONEST-PX-VERSION-AUDIT | §3 | done (design + 1 conflict found this session) |
| HONEST-PX-SELFREF-DETECT | §4 | done (design + self-check 0 violations) |
| HONEST-PX-BT548-ENTRY | §5 | done (rubric + 4 candidate scores) |
| HONEST-PX-GRADE-RUBRIC | §6 | done (rubric + 14-registration verification) |
| HONEST-PX-ARXIV-PROTO | §7 | done (protocol + outline) |

-> 7 done of PX 32 planned (8 done including HONEST-PX-1, 32 - 8 = 24 remain).

### 8.2 Honesty Declaration

- This document does not resolve any of the 7 Millennium problems.
- Automation / rubric / protocol are **infrastructure** and **not mathematical results**.
- §1, §2 are design only; actual .hexa code not written (DEFERRED).
- §5 BT-548+ priority is a **recommendation**, requiring user approval at v3 entry.

### 8.3 Follow-up

| Follow-up | task ID | Note |
|-----------|---------|------|
| atlas_auto_promote.hexa implementation | PX HONEST-PX-AUTO-PROMOTE-EXEC | M cost |
| verify_all hook addition | PX HONEST-PX-VERIFY-AUTO-EXEC | M cost |
| arXiv paper writing | NUM-PX-3 | L cost |
| BT-548 entry | v3 phase | after user approval |

---

## References

- millennium.json `.phases[id=PX].parallel[track=Y10_HONEST_HARNESS]`
- HONEST-PX-1 14 atlas registrations (atlas.n6 line 106960~107020)
- phase-07-cross-bt-transfer-protocol.md (P7 9/9 concurrent closure)
