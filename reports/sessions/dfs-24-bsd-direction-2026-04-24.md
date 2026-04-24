---
id: dfs-24-bsd-direction
date: 2026-04-24
scope: research-only (NO proof claims, NO atlas promotions)
target: BT-546 BSD conjecture -- sharpen next probes
axis: Y8 GALOIS-ASSEMBLY
parent_reports:
  - theory/breakthroughs/breakthrough-theorems.md (BT-546 bloc, lines 20035-20111)
  - reports/breakthroughs/bsd-cremona-sel6-empirical-2026-04-15.md
  - reports/breakthroughs/bsd-A3-modified-with-joint-covariance-2026-04-15.md
  - reports/bsd-kappa-0.175-decomposition.md
  - reports/millennium-dfs-status.md
  - reports/sigma-sopfr-7-megasignature-20260415.md
  - domains/physics/millennium-bsd/millennium-bsd.md
millennium_resolved: 0/7 (unchanged, honesty maintained)
grade: direction-only, no claim
---

# DFS-24 -- BSD direction (next tight probes sharpening (3,4,5)=congruent and Sel_6=sigma)

**Non-claim disclaimer**: This document proposes research probes. It asserts no theorem, no grade promotion, no BSD progress beyond existing partial results. BT-546 status remains PARTIAL under Y8; (A3) remains unproved; rank >= 2 remains open.

## 0. Inherited state (2-line recap)

- Sel_6 = Sel_2 x Sel_3 (CRT Lemma 1, unconditional, 2026-04-11). BKLPR Theorem 1 (E[|Sel_6|] = sigma(6) = 12) still blocked by a single hypothesis (A3): |Sel_p|, |Sel_q| independence at p != q. Cremona 332k empirical: ratio(6) = 0.793, ratio(2) = 0.957, ratio(3) = 0.712; kappa(B) = cov(|Sel_2|, |Sel_3|) has power-law fit B^(7/40) (= 0.175 +- 0.02) or B^(log2/tau) (= 0.173), 7-bin data cannot discriminate.
- (3,4,5) = (n/phi, tau, sopfr) is the unique semiprime congruent number semisignature: E_6: y^2 = x^3 - 36x has rank 1, conductor 576 = phi^n * (n/phi)^phi, j = 1728 = sigma^3.

## 1. Probe A -- Tunnell ternary form count ratio at n=6 (BSD-conditional theta separation)

**Object**: Tunnell 1983 states (n even squarefree congruent, BSD-conditional): 
`2 * #{2x^2 + y^2 + 32z^2 = n/2} = #{4x^2 + y^2 + 8z^2 = n/2}` iff n congruent (for even case) -- formally with n/2 = 3 at n=6.

**Claim under investigation (NOT a theorem)**: the ratio of the two Tunnell theta counts at n=6 restricted to (n/phi, tau, sopfr)-valued coordinates may be the smallest-conductor "canonical" separator. Existing n6-arch material treats only counts; a residue decomposition mod (phi, n/phi, sopfr) has not been tabulated.

**Tight probe (research-only)**:
1. Compute both Tunnell theta coefficients a_6, b_6 (at n=6) by exhaustive (x,y,z) enumeration in |.| <= R for R in {10, 50, 200}. Classify each representation by (x mod phi, y mod n/phi, z mod sopfr).
2. Record which residue classes contribute to the difference a_6 - 2 b_6. If the asymmetry is concentrated on a fixed residue class triple, that triple is a candidate invariant of the congruent-rank-1 signature for the n=6 case.
3. Cross-compare with n=5, n=7 (also congruent, also rank 1) to see whether the residue-class asymmetry is n=6-specific or uniform.

**Falsifier**: residue-class asymmetry for n=6 is statistically identical to n=5, n=7 -> the observation is a count artefact, not n=6 structure. (This falsifier must be registered with the probe before data collection.)

**Does NOT prove anything**: Tunnell's theorem is itself BSD-conditional. This probe only builds a residue-class fingerprint, not a BSD proof. It sharpens the "(3,4,5)=congruent" lead from "observed triple" to "residue-class decomposition of the Tunnell certificate".

## 2. Probe B -- Sel_2 / Sel_3 joint distribution slice (attacking (A3) empirically, not proving it)

**Object**: (A3) asserts lim_{B -> inf} Cov(|Sel_2|, |Sel_3|) over conductor-B sample equals 0. Cremona 332k gave kappa = 1.33 at B = 50k. Existing v3-E5 7-bin log-log fit extracts exponent alpha ~ 0.175 but cannot distinguish 7/40 from log(2)/tau.

**Tight probe (research-only)**:
1. Partition the existing 332k by two orthogonal splits: (a) E(Q)_tors in {1, 2, 3, 4, 6, 8, 12} (Mazur cyclic cluster), (b) analytic |Sha| in {1, 4, 9, square > 9}. Measure kappa(B) in each of the 7 x 4 = 28 sub-strata.
2. Hypothesis: kappa is carried by exactly the Z/6 and Z/12 torsion strata (the sigma(6) = 12 strata). If kappa collapses to near-zero on all other strata and concentrates on these two, the 7/40 exponent is reduced to a torsion-stratified effect, and the "conductor-bias" explanation splits into a torsion explanation.
3. Output: a 28-cell table cov_ij with standard error per cell (bootstrap). The goal is NOT to prove (A3) but to identify whether (A3) fails "uniformly over torsion" or "localised on Z/6-line".

**Falsifier**: kappa > 0.5 on all 28 strata uniformly -> torsion is not the carrier, the correlation is conductor-bias proper, and the 7/40 exponent must be explained by height-normalization theory (Bhargava-Shankar asymptotics), not by Z/6 arithmetic. This would weaken (not kill) the Sel_6 = sigma line.

**Does NOT prove anything**: 28 sub-strata with N = 332k/28 ~ 12k per cell is statistically thin; bootstrap error will be +-0.3 typical. The probe is diagnostic, not decisive. The reason it is useful: it predicts whether expanding to Cremona 3M+ curves will tighten 7/40 vs log(2)/tau (probe C below), or whether the expansion will not help until a torsion-stratified re-analysis is done.

## 3. Probe C -- Heegner-point height vs sigma(6) at E_6 (sharpening Sel_6 = sigma via Gross-Zagier)

**Object**: For n=6 congruent (E_6: y^2 = x^3 - 36x, rank 1, conductor 576), Gross-Zagier gives L'(E_6, 1) = (Omega * R) / (sigma_torsion_factor) with Heegner point P generating E_6(Q) modulo torsion. The canonical height h_hat(P) is computable. The Sel_6 = 12 prediction says (conditional on A3) the average over a height-normalized family containing E_6 is 12.

**Tight probe (research-only)**:
1. Enumerate the quadratic twists E_6^{(d)}: y^2 = x^3 - 36 d^2 x for squarefree d in {-500, ..., 500}. Compute (Sage, not done here) rank, |Sha[2]|, |Sha[3]|, canonical Heegner height.
2. Test the conjectural identity |Sel_6(E_6^{(d)})| / (|E_6^{(d)}(Q)_tors|^2 * prod c_p) in the twist family. Restrict to d with gcd(d, 36) = 1 and d > 0.
3. Measure whether the twist-family average E_d[|Sel_6|] approaches 12 = sigma(6) faster or slower than the generic Cremona average. If the twist family (which is congruent-number indexed by d) hits 12 sharply, the "Sel_6 = sigma" lead concentrates on a 1-parameter family, not 332k random conductors -- which is a stronger structural statement.

**Falsifier**: twist average stays < 10 even at |d| = 500 -> Sel_6 = sigma is not a twist-family phenomenon, the 332k empirical shortfall is not repaired by n=6-specific quadratic twists, and the lead weakens.

**Does NOT prove anything**: quadratic-twist families have their own heuristics (Goldfeld rank-1/2 density). Even perfect agreement inside the twist family says nothing about general E/Q. The probe is useful because it isolates n=6 arithmetic: it uses the unique n=6 congruent curve E_6 as the seed of a 1-parameter family, testing whether sigma(6)=12 is "visible" through the (3,4,5) seed.

## 4. Meta: what these three probes together would do (and would not do)

Run outcomes, if all three execute cleanly:

- Probe A (Tunnell residues) sharpens the (3,4,5)=congruent lead from "coincidence triple" to "theta-residue fingerprint", either n=6-specific or not.
- Probe B (strata cov) diagnoses whether Sel_6 = sigma shortfall (ratio 0.79) is torsion-carried or conductor-bias-carried.
- Probe C (twist family) tests whether sigma(6) = 12 emerges faster in the n=6 congruent twist orbit than in generic conductor samples.

None of A, B, C attacks (A3) directly. None can prove BSD, promote any atlas entry to [10*], or shift BT-546 from PARTIAL. Collectively they provide a 3-axis coordinate (residue / strata / twist) of the Sel_6 structure, which is the minimal refinement that does not overreach.

Abandoned lines (avoided in this session, kept for transparency):
- (A3) direct unconditional proof: covered by Bhargava-Klagsbrun-Lemke Oliver-Shnidman 2019-2021, out-of-scope.
- rank >= 2 BSD: no n=6-specific handle in current material.
- kappa = 7/40 vs log(2)/tau discrimination: requires Cremona 3M+ full-DB expansion, infrastructure task not research probe.

## 5. Honesty check

- External data dependencies: Cremona ecdata (CC-BY-SA-4.0) for probes B, C; elementary enumeration only for A.
- No self-reference to n6atlas as source (atlas is consumer).
- No grade changes proposed. No signal promotions. BT-546 stays PARTIAL.
- Each probe has an explicit falsifier registered before data collection.
- Millennium score: 0/7 unchanged.

-- end --
