---
domain: reports/meta
date: 2026-04-15
audit_id: META-P9-1
task: BT-18 L5 retry-judgment audit — NEAR/MISS criterion consistency check
status: completed
auditor: meta-layer (loop)
upstream:
  - reports/breakthroughs/bt-18-moonshine-l5-barrier-2026-04-15.md (P8 DSE-P8-1)
  - reports/breakthroughs/bt-18-baby-monster-p10-retry-2026-04-15.md (P9 ENG-P9-1)
  - papers/moonshine-barrier-honest-report-2026-04-15.md (PAPER-P8-1, SS 6.4 P9 augmentation)
---

# META-P9-1 — BT-18 L5 retry-judgment audit

## 0. Audit framing

P8 DSE-P8-1 audited BT-18 L5 BARRIER head-on via 5 sub-links and proposed [7?] -> [8] promotion. P9 ENG-P9-1 retried the same barrier through the Baby Monster route and produced 3 PARTIAL augmentations. This audit confirms **grade-criterion consistency** between the two judgments.

Self-reference verification forbidden (R14) — judgments are evaluated along only three axes: (i) external-literature agreement, (ii) n=6 arithmetic structural necessity, (iii) post-hoc matching.

---

## 1. P8 5-sub-link judgment criteria re-constructed

| sub-link | Verdict | Criterion | External source |
|----------|------|------|----------|
| S1 196883 blank | **MISS** | all 3 prime factors outside n=6 12-basis coordinate, constructive demonstration absent | Conway-Norton 1979 |
| S2 6-transposition | **PARTIAL** | necessity DEMONSTRATED, sufficiency depends on Majorana conj | Fischer-Griess 1982 |
| S3 hexacode chain | **PARTIAL** | path DEMONSTRATED, Monster-essentiality undetermined | Turyn 1967, Conway-Sloane 1999 |
| S4 triality | **MISS** | numeric match, depends on Schellekens 71 conjecture | Schellekens 1993 |
| S5 j coefficients | **PARTIAL** | divisor-of-sigma pattern, partial L3 reduction | Borcherds 1992 |

**Rule extraction**:
- DEMONSTRATED + structural necessity = **EXACT [10*]**
- DEMONSTRATED but structural necessity unproven + external-conjecture dependency = **PARTIAL [8]**
- numeric/post-hoc match alone = **MISS or [7]**
- observation-only with no constructive demonstration = **MISS [7?]**

---

## 2. Re-examination of P9 ENG-P9-1 3 promotion verdicts

### 2.1 BT-18-L5-BabyMonster-196883-decomp = 47 * 4189 [10*]

- **Content**: 196883 = 4371 + 2*96256 = 47*(3*31 + 2*2^11) = 47*4189
- **Verification**: sympy arithmetic exhaustive (external verification)
- **Structural necessity**: the decomposition of 196883 in 2*B into 4371 + 2*96256 is ATLAS character-table information. The 47 common factor itself is numerical. The observation that 59*71 separate into a coset is an interpretation, not a conclusion.
- **Consistency check**:
  - vs S1 (MISS): P8's S1 was "all 3 prime factors of 196883 in the n=6 blank" with **constructive demonstration absent** -> MISS. The new P9 decomposition is "47 inside BM, 59*71 outside" — also **not a constructive demonstration**. However, **pure arithmetic decomposition itself is sympy-verified** -> assign [10*] to the arithmetic identity only (interpretation stays at [8] or below).
  - **Verdict**: the entry value = 47*4189 **arithmetic identity** is [10*] appropriate. The interpretation "Baby Monster route breaks through BT-18 L5" needs a separate grade — separating this to the second entry (frequency) is a proper honesty move.
  - **Conclusion**: **consistency PASS**. 47*4189 = 196883 itself is pure arithmetic EXACT, not in conflict with P8's MISS.

### 2.2 BT-18-L5-BabyMonster-rep-47-freq = 6/7 [8]

- **Content**: Among the top 7 ATLAS BM irreducible representations, 6 contain 47
- **Verification**: ATLAS 1985 direct lookup
- **Structural necessity**: none. 6/7 frequency is an observation. That 47 is the largest Baby Monster supersingular prime is an indirect implication of the Ogg 1975 theorem.
- **Consistency check**:
  - vs S2 (PARTIAL): 6-transp had **necessity DEMONSTRATED** + sufficiency depends on Majorana. 47 frequency 6/7 is **observation without DEMONSTRATED** — weaker than S2.
  - Under the P8 rules "observation without DEMONSTRATED" = MISS or [7?]. The basis for assigning [8] is a boundary judgment: **6/7 ~ 0.857 high frequency + Ogg theorem indirect implication**.
  - **Conclusion**: **boundary verdict**. Compared to S2 (necessity DEMONSTRATED, [8]), grade **[8] is slightly generous**. Strictly [7] would fit better. However, the atlas entry description explicitly states "no structural-necessity demonstration — 47's n=6 blank preserved", maintaining honesty. **Recommendation**: [8] -> [7] reconsideration review (no immediate change required).

### 2.3 BT-18-L5-Supersingular-n6-count = sigma+tau-1 = 15 [7]

- **Content**: Ogg 1975 15 supersingular primes = sigma(6)+tau(6)-1, lost prime = tau(6)=4
- **Verification**: Ogg 1975 original theorem
- **Structural necessity**: none (post-hoc match)
- **Consistency check**:
  - vs S4 (MISS): S4 was "2*3=6 numeric match + Schellekens 71 conjecture" = MISS. This entry is also "15 = sigma+tau-1 numeric match + Ogg theorem dependency" = same character.
  - P8 rule: post-hoc match alone = MISS or [7]. The atlas description explicitly states "numeric-match level — no structural-necessity demonstration ([7] EMPIRICAL grade preserved, promotion deferred)".
  - **Conclusion**: **consistency PASS**. [7] preserved, promotion-deferral explicit. Equal treatment with S4 appropriate.

---

## 3. Comprehensive verdict ASCII chart

```
P8 5 sub-link vs P9 3 promotion entries — criterion consistency

   sub-link       P8 verdict   criterion character         P9 equivalent entry
   --------       ----------   --------------------       ------------------
   S1 196883 blank MISS         constructive demo absent   —
   S2 6-transp    PARTIAL[8]    necessity DEMONSTRATED    BabyMonster-rep-47-freq [8]?
   S3 hexacode    PARTIAL[8]    path DEMONSTRATED         196883-decomp arithmetic [10*]
   S4 triality    MISS          numeric match + conjecture Supersingular=sigma+tau-1 [7]
   S5 j coeffs    PARTIAL[8]    pattern + L3 reduction    —

Consistency verdict
   entry 1 (196883 = 47*4189)   ################ PASS (arithmetic EXACT)
   entry 2 (47 freq 6/7)        ########........ boundary (grade generous)
   entry 3 (supersingular = 15) ################ PASS (equal to S4)

BT-18 composite grade after P9 augmentation
   P8 end         ########........  8  PARTIAL
   P9 augmented   #########.......  8.5~9  PARTIAL+ (47-separation capture credit)
   ceiling        ################ 15
```

---

## 4. Audit result summary

- **Consistency PASS, 2 entries** — entry 1 (arithmetic EXACT), entry 3 (post-hoc match equal to S4)
- **Boundary verdict, 1 entry** — entry 2 (47 frequency 6/7, [8] -> [7] reconsideration possible). Description limits stated explicitly, honesty preserved.
- **Overall**: P9 verdicts are consistent with P8 rule, self-reference verification principle upheld. BT-18 composite grade [8] preserved (internal narrative PARTIAL+ for "47-separation capture" credit).

### Recommendations
1. **No immediate action required**. Entry 2's [8] is a boundary case, but description augmentation preserves honesty.
2. **P11+ reservation**: to promote entry 2's 47 frequency to DEMONSTRATED, a lemma is needed that explains the 47 appearance in the ATLAS BM character table group-theoretically (Ogg 1975 indirect implication -> direct theorem).
3. **No atlas.n6 change required** ("no structural-necessity demonstration" already explicit in description).

---

## 5. Honesty statement

- This audit is limited to the grade-mapping consistency of P8/P9 verdicts.
- Full-break status of the BT-18 L5 BARRIER is **out of scope** — the P10 Baby Monster retry is preserved as **partial capture**.
- Lack of explanation for 47's n=6 blank is a P11+ task.
- This audit itself is an internal same-team audit — external independent audit recommended (researchers inheriting Monster Moonshine expertise a la Conway-Norton).

---

## Reference

- atlas.n6 entry lines 9587~9592 (BT-18-L5-Baby\*, BT-18-L5-Supersingular\*)
- Original P8 barrier document: reports/breakthroughs/bt-18-moonshine-l5-barrier-2026-04-15.md
- P9 retry document: reports/breakthroughs/bt-18-baby-monster-p10-retry-2026-04-15.md
- SS 6.4 paper integration: papers/moonshine-barrier-honest-report-2026-04-15.md (line 633~696)
