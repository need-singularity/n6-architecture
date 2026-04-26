---
id: omega-exec-bt544-q1-molt-validation
date: 2026-04-25
scope: research-only molt-validation experiment (NO NS regularity claim, NO atlas promotion)
target: BT-544 Q1 -- KdV 6-soliton Gram-lattice frame validation (cheapest molt per L9 sequencing)
parent_reports:
  - reports/sessions/omega-probe-l9-molt-trigger-2026-04-25.md (§ 4 BT-544 Q1)
  - reports/sessions/dfs-24-ns-direction-2026-04-24.md
  - reports/sessions/omega-cycle-bt544-ns-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: molt-validation experiment, no claim
---

# Omega Exec — BT-544 Q1 Molt-Validation (2026-04-25)

## §0 Non-claim disclaimer

This report executes the **single cheapest molt-validation** in the L9
probe sequence (`omega-probe-l9-molt-trigger-2026-04-25.md` §sec 7.1
ranked **BT-544 Q1 first**). The experiment tests the *frame-shift*
"Triple-resonance L1-smash frame → KdV 6-soliton Gram-lattice frame"
proposed in dfs-24 P1 / per-BT Q1.

**This document does NOT**:
- claim 3D NS regularity in either direction (smoothness or blow-up);
- promote anything in `shared/n6/atlas.n6`;
- modify `state/proposals/inventory.json`;
- modify `theory/canon/`;
- alter the `BT-544 = 0/1 untouched` Clay status;
- claim Q1's outcome bears on Clay closure either way.

**Millennium tally**: 0/7 unchanged. The Q1 outcome speaks **only** to
the validity of the proposed BT-544 frame-shift candidate (whether it
introduces a new primitive or is a relabeling), not to NS regularity.

---

## §1 Q1 spec extracted

Combined from `omega-probe-l9-molt-trigger-2026-04-25.md` §4.4 and
`dfs-24-ns-direction-2026-04-24.md` §3 P1 and
`omega-cycle-bt544-ns-2026-04-25.md` §7 Q1.

**Object**: KdV 6-soliton phase-shift Gram-lattice candidate frame.

**Measurement**: For KdV solitons with amplitudes κ_1,…,κ_6, define
the pairwise phase shift

    Δ_{ij} = 2 · log| (κ_i − κ_j) / (κ_i + κ_j) |,    1 ≤ i < j ≤ 6.

Construct the symmetric 6×6 matrix M with M_{ij} = Δ_{ij} (i ≠ j),
M_{ii} = 0 (the natural reading of "the matrix {Δ_{ij} : i<j ∈ [6]}"
in §4.4). Compute (i) numerical rank under standard floating-point
tolerance, (ii) det(M) / σ where σ = σ_1(6) = 12.

Two κ families per spec:
- Family A: κ_k = k for k = 1..6.
- Family B: κ_k = p_k = the k-th prime, k = 1..6 → (2,3,5,7,11,13).

**Pass criterion (real molt)**:
    rank(M) = 3  =  n/φ  (where n=6, φ=2)
    AND
    det(M) / σ ∈ ℤ
    on **both** κ families.

Per L9 §4.4: "the molt introduces a Gram-lattice primitive (algebraic
structure) absent from the tensor-count frame".

**Fail criterion (relabeling)**:
    rank(M) ≠ 3   OR   det(M)/σ ∉ ℤ
on **either** κ family. Per F_Q1 (§8 of omega-cycle-bt544-ns) and
F-544-A (sec 5 of L9 probe): "the C(6,2)=15=sopfr·(n/φ) ↔ Serrin
(σ,n,τ) crosswalk demoted to post-hoc; KdV-side anchor lost; frame
collapses to relabeling".

**Cost (per L9 §7)**: very low, <1h compute, pure algebra.

---

## §2 Existence / tooling check

Repo scan results:
- No pre-existing KdV / soliton / Gram-lattice computational tool
  located via `find` over `experiments/`, `tool/`, `data/`,
  `domains/physics/millennium-navier-stokes/`, `theory/`. Reference to
  KdV 6-soliton structure is purely textual in
  `reports/breakthroughs/bt-1411-millennium-dfs-round19-2026-04-12.md`
  §BT-1411-03 (C(6,2)=15 phase-shift count, Hirota τ-function citation).
- No script computing pairwise log-phase-shift matrices was located.
- The Q1 measurement is hand/script-computable in seconds: 6×6 symmetric
  matrix → eigendecomposition for rank → determinant. Numpy 2.4.3
  available locally (`python3 -c "import numpy"` succeeded).

**Decision**: write a ≤60-line script `bt544_q1_molt_validation.py`
under `experiments/anomaly/` (sibling to the existing
`verify_fisher_bernoulli.py`). Script computes Reading (A) — the
literal §4.4 / §3-P1 spec — and reports two alternate Gram readings
(C, D) for transparency, but the verdict uses only Reading (A) per
the spec text.

**Produced artifact**: `experiments/anomaly/bt544_q1_molt_validation.py`
(≈70 lines including alternate-reading sanity blocks; principal Q1
reading is ≤30 lines).

---

## §3 Execution log

```
$ time python3 /Users/ghost/core/n6-architecture/experiments/anomaly/bt544_q1_molt_validation.py
```

- Wallclock: 0.27 s (user 0.10 s, sys 0.04 s) — well under the
  "<1h compute" budget.
- Exit code: 0.
- Input: hard-coded κ families per Q1 spec (`INT_KAPPA = [1..6]`,
  `PRIMES = [2,3,5,7,11,13]`); no external data files.
- No stochastic step; result is deterministic.

Numerical-tolerance protocol:
- Rank threshold: |λ_i| > max|λ| · 1e−10 ⇒ count toward rank.
- Integrality threshold: |x − round(x)| < 1e−6 (with absolute scaling
  by max(|det|, 1)).

These are standard double-precision tolerances; matrices are 6×6 with
condition number ≈ O(10) so the rank determination is robust.

---

## §4 Computed value

### Reading (A) — principal Q1 spec

**Family A: κ_k = k = (1,2,3,4,5,6)**

- Eigenvalues of M: (−12.6945, −1.2295, 1.5274, 3.0525, 4.1877, 5.1565).
- All six |λ_i| ≫ tolerance ⇒ **rank = 6**.
- det(M) = 1571.4129…
- det(M) / σ = 1571.4129 / 12 = 130.9511… **∉ ℤ**.

**Family B: κ_k = p_k = (2,3,5,7,11,13)**

- Eigenvalues of M: (−10.8007, −2.4885, 1.0951, 3.2782, 3.8932, 5.0227).
- All six |λ_i| ≫ tolerance ⇒ **rank = 6**.
- det(M) = 1886.6911…
- det(M) / σ = 1886.6911 / 12 = 157.2243… **∉ ℤ**.

### Alternate readings (sanity checks, not part of the verdict)

- Reading (C) outer-difference Gram (V_{ij} = κ_i − κ_j; G = V V^T):
  rank = 2 on both families (det ≈ 0 to within floating noise; trivially
  integer-zero, so the integrality test is uninformative here).
- Reading (D) outer-sum Gram (V_{ij} = κ_i + κ_j; G = V V^T):
  rank = 2 on both families; same uninformative integrality result.

Neither (C) nor (D) yields rank = 3 either. No reading matches the
target rank of n/φ = 3.

---

## §5 Verdict

**FAIL**.

Comparison to spec thresholds (§1):

| family | rank(M) | target rank | det/σ          | integer? |
|--------|---------|-------------|----------------|----------|
| A (k)  | 6       | 3           | 130.9511…      | NO       |
| B (p)  | 6       | 3           | 157.2243…      | NO       |

The Q1 pass criterion required `rank = 3 AND det/σ ∈ ℤ on both
families`. We obtain `rank = 6 AND det/σ ∉ ℤ on both families`.
**Both clauses fail on both families — the failure is double-margin,
not borderline.**

Per the explicit fail clause (L9 §4.4):
> "rank ≠ 3 OR det(Gram)/σ not in ℤ on either κ family; per F_Q1, the
> C(6,2)=15=sopfr·(n/φ) crosswalk is demoted to post-hoc."

The falsifiers F_Q1 (omega-cycle-bt544-ns §8) and F-544-A (L9 probe
§5) **fire**.

The verdict is **not INCONCLUSIVE**: the analytic phase shifts are
transcendental real numbers (Δ_{ij} = 2 log|·|), whose generic
determinant is irrational; the matrix has no enforced rank deficit —
the structure of M does not vanish on any 3-dimensional subspace.
Floating-point conditioning is not at issue (the principal eigenvalues
are O(1)–O(13) with smallest |λ| ≈ 1.1; 1e-10 separation from zero
trivially satisfied).

---

## §6 Implications

### What FAIL means here

Per L9 §sec 4 ("real molt vs relabeling"): the proposed BT-544
frame-shift `Triple-resonance L1-smash → KdV 6-soliton Gram-lattice`
**does not introduce a new algebraic-lattice primitive that produces
a structurally-distinguished invariant**. The intended Gram-rank
identity (rank = n/φ = 3) and σ-divisibility identity (det / σ ∈ ℤ)
do not hold under the literal phase-shift construction on either
sampled κ family.

Therefore, per L9 §sec 4 and F-544-A: this candidate **collapses to
relabeling** — the frame change is nominal, not structural. The
KdV 6-soliton frame, *as captured by the Q1 measurement*, is **not a
qualifying L9 molt for BT-544**.

### Disclaimer on scope of the FAIL

- The FAIL refers **only** to the Q1 measurement as specified
  (analytic phase-shift Gram, σ-divisibility on amplitude families
  k and p_k). Other KdV constructions (Hirota τ-function ranks on
  Gr(n,2n) dim n²=36, Plücker lattices, conserved-quantity hierarchy)
  are **not falsified** by this experiment — they were not measured.
- No claim is made about the rank-2 / 3 alternative candidates in
  the L9 catalogue (BT-544 §3.4) — those remain available.
- No claim about NS regularity. The Clay status `0/1 untouched`
  remains the same as before this experiment.

### L9 sequencing consequence

Per L9 probe §sec 7.3 stop-conditions:
- This is 1 BT validated as FAIL out of 4 in the calibration sweep.
  Stop-after-0/4 has not fired (we have 1/4 measured, 0/1 passes).
- Per L9 §sec 7.1 sequencing, **BT-543 P3 (A4-ratio) is the next
  cheapest molt-validation** (rank 2 in the cost / signal table). The
  L9 calibration batch (Run 1 = BT-544 Q1 + BT-543 P3) is half-done;
  proceeding to BT-543 P3 is the recommended next step.
- Per L9 probe §sec 3.4 BT-544 catalogue: the **rank-2 alternative
  for BT-544** is "Mechanism-axis seed frame (dfs-24 Q5) — Sobolev/
  Besov estimate predicted by n=6 lattice". Its falsifier F-544-B is
  pre-registered and the candidate is repo-sourced. The **rank-3
  alternative** is "KPZ d-lift frame to d=7 (dfs-24 P2 / Q3)" with
  F-544-C.
- **Recommended**: continue the L9 calibration with BT-543 P3 next
  (cross-BT signal), and only return to BT-544 alternatives after
  the BT-543 result is in. If BT-543 P3 also fails, the L9 gate is
  approaching F-MOLT-A (gate-failure-via-validation, §6 of L9 probe).

---

## §7 Re-audit feedback to omega-cycle-bt544-ns-2026-04-25.md

Suggested edits to that document (NOT applied here; only flagged for
the next omega-cycle pass):

1. **§7 Probe Q1**: append "Q1 executed 2026-04-25 in
   `omega-exec-bt544-q1-molt-validation-2026-04-25.md`; verdict =
   FAIL (rank = 6 ≠ 3 AND det/σ ∉ ℤ on both κ families). F_Q1 fires."

2. **§3 Ω-saturation estimate**: no change to the composite estimate
   (still ~0.47 naive). The Q1 FAIL **does not lower** structural
   saturation: it confirms the L1-saturation diagnosis ("smash is
   generative without verification" — Tension #1) by showing that the
   first executed verification is a NO. Mechanism-saturation stays at
   ~0.05.

3. **§6 Cross-axis tension #1**: replace "Resolution direction: route
   smash output through dfs-24 probes…" with "First routed probe (Q1)
   returned NO; tension #1 hardens into a falsified crosswalk for the
   KdV phase-shift specific construction; recommend running Q5
   mechanism-axis seed earlier than originally sequenced if BT-543 P3
   also fails."

4. **§8 Falsifier table**: tag F_Q1 and F_P1 with `state = FIRED
   (2026-04-25)`. Active-falsifier count drops from 11 to 9 active +
   2 fired.

5. **§9 Closing summary**: update "first probe-conversion pulse at
   dfs-24 (2026-04-24); no breakthrough-class pulse" to "first
   probe-conversion pulse at dfs-24 (2026-04-24); first executed
   probe (Q1, 2026-04-25) returned a falsifier-fire; no breakthrough-
   class pulse".

These are **suggestions for the next session** — this report does
not edit `omega-cycle-bt544-ns-2026-04-25.md` directly.

---

## §8 Anomalies

- **Spec-vs-construction gap**: the dfs-24 P1 text additionally says
  "the Gram matrix G = Δ · Δ^T on C(6,2) pairs"; if Δ is the 15×1
  vector of pair-phase-shifts, Δ · Δ^T is rank-1 by construction,
  contradicting the rank-3 target. The §4.4 L9-probe phrasing — "the
  matrix {Δ_{ij} : i<j ∈ [6]}" — was therefore taken as the operative
  specification (the natural 6×6 symmetric matrix). **Two readings
  of the spec exist; the principal one (Reading A) is unambiguous,
  and an alternative literal Δ · Δ^T construction would tautologically
  fail rank = 3 anyway (rank-1).** No reading admits rank = 3.
- The phase shifts are negative real numbers (since |κ_i−κ_j| <
  |κ_i+κ_j| for positive κ); the off-diagonal entries of M are all
  negative. This is consistent with KdV physics (phase shifts on
  collision are sign-indeterminate but their magnitudes are well-
  defined logs). No anomaly here, just a noted sign pattern.
- The principal eigenvalue is large negative (≈ −12.7 family A,
  −10.8 family B); the trace is 0 (since M_{ii} = 0). All eigenvalues
  bounded away from zero ⇒ rank-deficiency would have been a
  numerical surprise, and indeed there is no rank deficiency.

No surprises that change the verdict.

---

## §9 Falsifiers active for this validation itself

Validation-level falsifiers (not BT-544 falsifiers, but conditions
under which **this very Q1 measurement** would be retracted):

- **F-VAL-A** (Gram-conditioning anomaly): if the 6×6 phase-shift
  matrix turned out to be ill-conditioned (κ(M) > 1e10), the rank
  determination would be numerically suspect. Observed κ(M) ≈ O(10)
  on both families — **not active**.
- **F-VAL-B** (soliton-identification ambiguity): if the κ_k values
  must be *amplitudes* in a specific KdV normalization (e.g.,
  Zakharov-Shabat scattering data with a non-trivial mapping from
  "amplitude" to "spectral parameter"), the test on κ_k = k and
  κ_k = p_k is non-canonical. The dfs-24 P1 spec uses
  Δ_{ij} = 2 log|(κ_i−κ_j)/(κ_i+κ_j)| which is the **standard
  GGKM/Hirota** phase-shift formula; the κ_k = k and κ_k = p_k
  families are **explicitly named in the spec** as the test set.
  **Not active** — but a reader who insists "it must be normalised
  spectral parameters from a specific Lax pair" could re-run with
  rescaled κ; we predict the same outcome (transcendental dets) under
  any positive-real κ choice that does not produce a degenerate
  amplitude (κ_i = κ_j collapses Δ to −∞).
- **F-VAL-C** (sigma-mis-identification): if "σ" in `det/σ ∈ ℤ`
  meant something other than σ_1(6) = 12 (e.g., perfect-number σ in
  a domain-specific sense), the integrality criterion is mis-stated.
  L9 §sec 4.4 and dfs-24 P1 both write `det/σ ∈ ℤ` without
  redefinition, and the BT-544 §X.1 SMASH section uses σ_1(6) = 12.
  **Not active** — but the integrality FAIL is not borderline (130.95
  and 157.22 are far from any integer divided by any plausible σ),
  so the fail conclusion is robust to σ ∈ {2,3,4,6,12,24,…}.
- **F-VAL-D** (coverage gap): only two κ families were tested. If a
  third family (e.g., κ_k = k², or κ_k = k-th Fibonacci) were the
  privileged one, the FAIL on (k) and (p_k) does not bind it. The
  spec however **named exactly these two families** and stated "on
  both" as the pass condition — extending coverage to other families
  is out of scope for Q1 as written. **Not active for this Q1**, but
  a future Q1' could re-run with extended coverage.

None of F-VAL-A..D fires. The FAIL verdict is robust.

---

## §10 Closing line

0/7 unchanged. NS regularity status open. No atlas/state/inventory
edits.
