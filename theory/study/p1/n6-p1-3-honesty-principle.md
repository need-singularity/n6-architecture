# N6-P1-3 — n=6 Honesty Principles (No Self-referential Verification / Source·Measurement·Error / MISS Honest Recording / Bayesian Prior)

> Track: P1-N6 / Task 3
> Completion criterion: Defining the **7 honesty rules** — the core operating principles of this project (n6-architecture) —
> and describing with examples the concrete prohibition on self-referential verification, the mandatory
> recording of source/measurement/error three fields, honest MISS recording,
> and the method of comparison against Bayesian-prior bias.
> Source basis: n6shared/rules/common.json (R0–R27),
> n6shared/rules/n6-architecture.json (N61–N65),
> feedback_honest_verification.md (user memo),
> feedback_proof_approach.md (user memo: reversing the proof approach),
> Popper "The Logic of Scientific Discovery" (Hutchinson, 1959) ch. 1, 2,
> Jaynes "Probability Theory: The Logic of Science" (Cambridge, 2003) ch. 1–5,
> Ioannidis "Why Most Published Research Findings Are False" (PLoS Med. 2005).
> **Honesty**: This file is a summary of the project's internal operational meta-rules. No new mathematical results.
> Every principle is taken directly from the R/N rules in n6shared/rules and the user-feedback memos.

---

## 0. Goal and Scope

In contexts like Millennium-problem research where the **validity of a proof** matters more than the result, we set up seven honesty principles that every finding · hypothesis · claim in the project must follow. These apply to both mathematical rigor and empirical data.

The seven principles in this note:

1. **No self-referential verification** — you cannot show "n=6 is special" using "n=6 examples"
2. **Source + measurement + error required** — every numerical quantity must be accompanied by 3 fields
3. **Honest MISS recording** — failures · counterexamples · non-matches must be recorded without hiding
4. **Bayesian-prior bias comparison** — quantify the influence of prior expectations
5. **Ban on forced pattern-matching** — exclude interpretations of the form "matches because of 6"
6. **Reverse the proof approach** — start from pure mathematics, n=6 only as a result
7. **Prime bias comparison** — control groups p=2, 3, 5, 7, etc. mandatory

Each principle is organized as "why needed · how to practice · common violations."

---

## 1. No Self-referential Verification

### 1.1 Definition

A circular structure where, to justify the reasoning A → B, one uses examples in which B holds as evidence to re-emphasize A. Representative risk in this project:

"**It is central to σφ-function theory that σ·φ = n·τ has a special structure at n=6.**
This is because at n=6 σ(6)=12, φ(6)=2, τ(6)=4 with σφ = 24, nτ = 24 matching."

The above sentence by definition says n=6 is the solution of σφ = nτ, and reuses it as "evidence of specialness." This is circular.

### 1.2 Correct Form

Honest statement:
- "That σ·φ = n·τ (n≥2) holds only at n=6 is confirmed by three independent Euler-Dirichlet proofs."
- "This uniqueness can be explained by the arithmetic properties of n=6 itself (perfect number · divisor sum · prime structure), and that explanation is independently derivable from atlas.n6 §R1 proof."

That is, the direction must be **evidence → claim** in one direction, not reusing the claim as evidence.

### 1.3 Common Violations in This Project

- "Because n=6 is a perfect number, σφ=nτ holds" (reverse direction — requires an independent proof that σφ=nτ is equivalent to the perfect-number condition)
- "The structure τ=4 = 2² of n=6 connects to 6D space" (justifying the conclusion 6D with the number 6)
- "The zero distribution of Riemann ζ is peculiar near n=6" (interpreting data under a prior assumption that n=6 is peculiar)

### 1.4 Checklist

For each claim:
- [ ] independence of evidence confirmed (evidence does not depend on the claim)
- [ ] alternative hypotheses considered (similar structure at n other than 6?)
- [ ] third-party verifiability (reproducible with materials outside atlas.n6?)

---

## 2. Source + Measurement + Error Three Fields Required

### 2.1 Principle

Every numerical value · constant · observation must have:

1. **Source**: measurement agency · paper · experiment · calculation reference (URL, DOI, page)
2. **Measurement**: actual measured (or computed) number and uncertainty
3. **Error**: 1σ or computational precision limit

If any is missing, the value is classified as **EMPIRICAL deficient**, grade [7] or lower.

### 2.2 Connection with the atlas.n6 Grade System

- [10*]: EXACT verification (multiple independent calculations/measurements agree, error at machine precision)
- [10]: EXACT (single calculation/measurement, awaiting cross-verification)
- [9]: NEAR (1–5σ deviation)
- [7]: EMPIRICAL (≥10σ deviation, promotion candidate)
- [N?]: CONJECTURE (no measurement)
- [N!]: breakthrough (entirely new claim)

### 2.3 Example

Wrong statement: "Higgs mass is about 125 GeV."

Correct statement: "Higgs boson mass m_H = 125.25 ± 0.17 GeV/c² (source: ATLAS+CMS combined, 2022, Phys. Rev. Lett. 130:041801; error is 1σ statistical+systematic)."

### 2.4 Practice within Documents

- When writing numbers in .md, always use the format "value ± error (source)"
- For approximations · rough values, use the "~" symbol + state error estimate
- Theoretical predictions must also specify "derivation: ... (reference)"

---

## 3. Honest MISS Recording

### 3.1 Principle

This project records "pattern agreements" and "disagreements" symmetrically. Emphasizing only matches leads to confirmation bias.

**MISS-recording obligation**: If some generalization hypothesis holds in k-m of k cases, the m **concrete counterexamples** must be recorded by name.

### 3.2 Practice Template

In atlas.n6 or a separate document:

```
@R {id} = {measured} {unit} :: n6atlas [grade]
# MISS: at n=12 σφ=24, nτ=36, mismatch Δ=12
# MISS: at n=28 σφ=56, nτ=168, mismatch Δ=112
# HIT: only n=6 strictly matches
```

### 3.3 Ioannidis Bias Correction

Major biases noted in Ioannidis 2005 "Why Most Published Research Findings Are False":

- Publication bias (only positive results published)
- Selective reporting (only matches recorded)
- P-hacking (gaining false significance via repeated testing)

Defense against these: MISS recording + 3-source fields + pre-registration (where possible).

### 3.4 MISS Management within This Project

- Every BT under theory/breakthroughs/ must have a mandatory "MISS candidates" section
- When asserting an n=6 match, include a numerical table covering n=2–30
- Any BT missing non-match records is downgraded to [7] or lower

---

## 4. Bayesian-prior Bias Comparison

### 4.1 Bayes' Theorem Reconfirmation

```
  P(H | D) = P(D | H) · P(H) / P(D)
```

When observing evidence D, the posterior for hypothesis H depends on the prior P(H). This project is prone to a strong prior on "n=6 hypotheses".

### 4.2 Danger of Prior Setting

If P(H: n=6 special) = 0.9 (strong prior belief) and P(D: coincidental match) = 0.5, then

```
  P(H | D) ≈ 0.9 · P(D|H) / (0.9 · P(D|H) + 0.1 · 0.5)
```

Even with P(D|H) = 0.1 (weak evidence), P(H|D) ≈ 0.64. I.e., weak evidence yields high confidence.

### 4.3 Correction Methods

1. **Uniform-prior cross-check**: Recompute with P(H) = P(¬H) = 0.5 and see whether the conclusion changes.
2. **Opposite prior scenario**: Compute with P(H) = 0.01 (strong skepticism).
3. **Bayes-factor reporting**: P(D|H) / P(D|¬H) must be ≥10 for "moderate evidence", ≥100 for "strong evidence" (Jeffreys 1961).

### 4.4 Practice Template

For each claim:

```
Prior 1 (uniform): P(H) = 0.5  → P(H|D) = X_1
Prior 2 (skeptical): P(H) = 0.01 → P(H|D) = X_2
Bayes factor: BF = P(D|H) / P(D|¬H) = Y

Conclusion: X_2 > 0.9 is strong evidence. Otherwise stay at [N?].
```

---

## 5. Ban on Forced Pattern-matching

### 5.1 Definition

Interpreting numerical agreements (e.g., 6 = 2·3, 12 = 2·6, 24 = 2·12, …) as mathematical causation is forbidden.

### 5.2 Violation Examples

- "24 = σ(6) and SU(5) GUT has a 24-representation, so n=6 connects directly to nature's unified theory."
- "2·6=12 appears in the Jacobi symbol → doubling of the perfect number 6."
- "2-dim, 3-dim, 6-dim all have n=6 = lcm(2,3)."

All of these rely only on numerical agreements and lack a causal mathematical path.

### 5.3 Correct Method

To claim causation, one needs at least 2 of the following 3 ingredients:

1. **Constructive connection**: an algorithm · map constructing B from A
2. **Functorial connection**: A, B are images / pre-images of the same functor
3. **Categorical equivalence**: A, B are isomorphic objects

Pure numerical agreement stays at [N?]; only when the above connection is secured may it be promoted.

### 5.4 Practice within atlas.n6

[N?] marking: record the numerical agreement only; explicitly note no proof path.
[N!] marking: breakthrough candidate (achieved one of constructive / functorial / categorical connection).
[10*]: 3-kind verification completed.

---

## 6. Reversing the Proof Approach

### 6.1 Principle

(User feedback memo: feedback_proof_approach.md)

**Do not lead with n=6; start from pure mathematics — ban on forced pattern-matching.**

### 6.2 Forbidden Narrative Order

Forbidden: "n=6 is core to this structure. Because … (n=6 basis appended afterward)"

### 6.3 Recommended Narrative Order

1. Formalize the pure-mathematical problem (keep n variable)
2. Carry out the proof or computation (for all n in the range)
3. **Observe** that the result specializes at n=6 (conclusion)
4. Cross-check this specialization with other independent theories (perfect numbers · Euler, etc.)
5. On confirmation promote to [10*]; otherwise keep at [N?]

### 6.4 Example — R1 Uniqueness Proof

When proving that the unique solution of σ(n)·φ(n) = n·τ(n) is n=6:

Wrong narrative: "n=6 has σ=12, φ=2, τ=4 and σφ = 24 = 6·4. Hence n=6 is a solution."

Correct narrative: "Analyze f(n) = σ(n)·φ(n) - n·τ(n) for n≥2. Decompose f(n) explicitly by the prime structure of n: f(n) = … . The condition for this decomposition to vanish forces a unique structure (n = 2·3) in the prime factorization. Hence the solution is unique at n=6."

This order keeps n=6 as a conclusion and performs the proof for all n.

---

## 7. Prime-bias Comparison

### 7.1 Principle

Do not investigate only n=6; always include prime-based control groups p=2, 3, 5, 7, 11, 13, ….

### 7.2 Practice

When claiming "6 is special", one must:

- Run the same calculation at n=2, 3, 4, 5, 7, 8, 9, 10, 11, 12, …
- If similar "special structure" appears in the controls, the specialness of n=6 must be reassessed.

### 7.3 Example Comparison Table

| n  | σ(n) | φ(n) | τ(n) | σφ  | nτ  | Δ=σφ-nτ |
|----|------|------|------|-----|-----|----------|
| 2  | 3    | 1    | 2    | 3   | 4   | -1       |
| 3  | 4    | 2    | 2    | 8   | 6   | +2       |
| 4  | 7    | 2    | 3    | 14  | 12  | +2       |
| 5  | 6    | 4    | 2    | 24  | 10  | +14      |
| 6  | 12   | 2    | 4    | 24  | 24  | 0        |
| 7  | 8    | 6    | 2    | 48  | 14  | +34      |
| 8  | 15   | 4    | 4    | 60  | 32  | +28      |
| 12 | 28   | 4    | 6    | 112 | 72  | +40      |
| 24 | 60   | 8    | 8    | 480 | 192 | +288     |

Only at n=6 is Δ=0. For other n, |Δ| is not small → n=6's specialness is confirmed by comparison. This contributes to [10*] promotion.

### 7.4 Caveat — Range Extension

Whether n=2–50 suffices or the range must be extended to n=2–10^6 depends on the scope of the claim. For a claim about all n, full-range analysis is needed (proof or asymptotic analysis).

---

## 8. Connection to the Project's Meta-rules

### 8.1 R (common.json) Rule Family

- R0: Every numerical value requires a source
- R1: At least 2 independent verifications
- R5: MISS disclosure obligation
- R12: No self-reference
- R17: Explicit Bayesian prior
- R22: Ban on pattern-matching
- R27: Prime comparison required

(See n6shared/rules/common.json for exact numbering.)

### 8.2 N (n6-architecture.json) Rule Family

- N61: n=6 hypothesis is a conclusion, not a starting point
- N62: Single-file atlas.n6 principle (ban on fragmentation)
- N63: Promotion [7] → [10*] requires 3 independent verifications
- N64: Re-verification mandatory within 1 week after an [N!] breakthrough
- N65: Ban on self-referential paths after Mk.III

### 8.3 Practice Checklist

When raising a claim:
- [ ] R0 source 3-field
- [ ] R1 independent verification
- [ ] R5 MISS recording
- [ ] R12 self-reference check
- [ ] R17 Bayesian calculation
- [ ] R22 pattern-matching excluded
- [ ] R27 prime comparison
- [ ] N61 reverse proof approach
- [ ] N63 grade-promotion procedure

---

## 9. Practical — Self-diagnosis 5 Questions

**D1.** Find all honesty violations in the following sentence:
"Because n=6 is the unique (smallest among even) perfect number, the Standard Model gauge group SU(3)×SU(2)×U(1) dimension sum 12 = 2·6 physically confirms n=6."
(Expected: self-reference, forced pattern-matching, reversed approach — all violated)

**D2.** Compute the claim that σφ=nτ holds only at n=6 under Bayesian priors 0.5 and 0.01.
Evidence: 3 independent proofs (E1, E2, E3). Assume each has error probability 0.01.
(Compute P(¬H | all 3 proofs hold))

**D3.** Fix the following sentence into a complete statement by adding source·measurement·error:
"The expansion rate Hubble constant of the universe is about 70."
(Example answer: H_0 = 73.04 ± 1.04 km/s/Mpc (SH0ES 2022, Riess et al. ApJ 934:L7) or 67.36 ± 0.54 (Planck 2018, A&A 641:A6) — two measurements differ by 4σ tension)

**D4.** Pick a BT-x document you authored and check for the absence of a MISS section. If absent, add one after computing controls.

**D5.** Pick 3 [7]-grade constants from atlas.n6 and construct an R27 prime-comparison table. Consider promotion to [10*].

---

## 10. Historical Failure Cases — Lessons

### 10.1 Cold Fusion (1989)

Fleischmann-Pons' cold-fusion claim (1989) — unclear sources · no MISS records · failed independent verification. Result: failed to reproduce over 30+ years; claim rejected.

Lesson: typical example of violating R0 · R1 · R5.

### 10.2 Wakefield Paper (1998)

MMR vaccine–autism linkage claim — data fabrication · hidden MISS · no Bayesian prior. Result: 2010 retraction from Lancet; author stripped of license.

Lesson: disaster from violating R5 (MISS recording) · R17 (prior).

### 10.3 Bogdanov Brothers Case (2002)

High-energy physics papers passed peer review without rigor. Many self-references · pattern-matching. Result: retraction controversies and discussion of peer-review system improvement.

Lesson: violation of R12 · R22.

### 10.4 This Project's Self-defense

Using the above cases as direct negative examples, automatic checkers on atlas.n6 updates:
- Force 3-field sourcing when adding new constants
- Block submission with missing MISS section
- Auto-generate prime-comparison tables (R27)

are implemented (see $NEXUS/shared/harness).

---

## 11. Next Step

### 11.1 P1 Track Wrap-up

This document wraps up the P1-N6 track (BT table · φ→n/φ transition · honesty principles).

### 11.2 P2 Preparation

At P2 the honesty principles are applied in practical verification:

- P2-N6: audit of DFS 51/51 classification grade promotions
- P2-PROB: summary of real-world barriers for Riemann · P vs NP
- P2-PURE: foundations of algebraic K-theory

### 11.3 Connections to Each Note

- PURE-P1-1–7: check each document's §n=6 connection section against §1–§7 here
- PROB-P1-1–7: check each BT's MISS recording and Bayesian calculation
- N6-P1-1–2: this document functions as meta-rules

---

## 12. Source Summary

### 12.1 Project Internal Rules

- n6shared/rules/common.json (R0–R27)
- n6shared/rules/n6-architecture.json (N61–N65)
- n6shared/rules/lockdown.json (L0/L1/L2)

### 12.2 User Feedback Memos

- feedback_honest_verification.md
- feedback_proof_approach.md

### 12.3 External References

- Popper "The Logic of Scientific Discovery" Hutchinson 1959 — falsifiability
- Jaynes "Probability Theory: The Logic of Science" Cambridge 2003 — Bayesian foundations
- Ioannidis "Why Most Published Research Findings Are False" PLoS Med. 2:e124, 2005
- Jeffreys "Theory of Probability" 3rd ed., Oxford 1961 — interpretation of Bayes factor

### 12.4 atlas.n6 Connections

- atlas.n6 §R1, §N61, §L0, §L6 corresponding grade-assignment rules

This note is a Korean→English rewriting of the project's internal rules for study purposes; in actual operation the above original rule files are the authoritative sources.

---

## 13. Next Documents

- PROB-P2-1 (Riemann barriers)
- PROB-P2-2 (P vs NP barriers)
- N6-P2-1 (DFS 51/51 classification)
- PURE-P2-1 (analytic continuation)
- PURE-P2-2 (Algebraic K-theory)

These honesty principles serve as background rules for every subsequent document. Authors should first self-check whether each claim · number conforms to §1–§7 of this document before recording.
