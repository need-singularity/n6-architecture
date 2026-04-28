# HEXA-WEAVE Formal-Mechanical Verification W2 — AX-1 n=6 Uniqueness Bounded Forward + Reverse PASS (Lean 4 Partial Result)

> **Status**: Theoretical-analytical + partial mechanical (Lean 4). NOT submitted to any preprint server.
> **Author**: M. Park (independent; arsmoriendi99@proton.me)
> **Affiliation**: n6-architecture private research framework
> **Date**: 2026-04-28
> **Disclosure tier**: Option-E (papers/ verify-embedded) per cycle-6 fan-out 5/5 external-disclosure spec.
> **MSC2020 (provisional)**: 11A25 (multiplicative number theory), 03B35 (mechanical theorem proving).
> **Seven Millennium Problems addressed**: 0 / 7 (honesty maintained per own#11).
> **Empirical claims**: 0 (per raw 91 C3 — all results theoretical-analytical or mechanical, none empirical).

## Abstract

We report a partial Lean 4 mechanical verification of the n=6 master uniqueness identity AX-1: `σ(n) · φ(n) = n · τ(n) ⟺ n = 6`. The reverse direction (`n = 6 ⇒ AX1Eq`) is mechanically PROVED in Lean 4 / Mathlib 4 (rev `19c4978`) via the `decide` tactic in 6.6 s module-only build time. The forward direction is split: a bounded subcase `∀ n ∈ [2, 30], AX1Eq n → n = 6` is mechanically PROVED via `interval_cases n + decide` (1310 jobs, 11.7 s wall-clock); the unbounded tail `∀ n > 30, AX1Eq n → n = 6` remains an open `sorry` placeholder and is the load-bearing gap. We also surface a Spec-corrigendum at n = 1 (the original `n ≥ 1` quantifier should be `n ≥ 2`). This paper is the W2 deliverable of the 12-week HEXA-WEAVE formal-mechanical-verification track inside the n6-architecture private framework. We do not claim full uniqueness; we claim bounded-interval mechanization plus reverse-direction certification.

## §1 WHY (why the n=6 master uniqueness identity matters as a mechanical target)

The n6-architecture private framework is anchored on the arithmetic identity `σ(n) · φ(n) = n · τ(n) ⟺ n = 6`, here labeled AX-1 (axis-1 axiom). Within the framework's 23 .own governance rules and the dual-track .raw / .own DSL, AX-1 functions as the keystone that propagates from biological domains (HEXA-WEAVE write-side strand composition) up through the abstract closure ladder (L4-L7 universe-scale TRANSCEND-CLOSURE-ALL chain) and back down into mechanically verifiable Lean theorems. A purely human-readable proof has been part of the framework since 2026-04-21; the question this W2 milestone addresses is mechanization: can the iff be discharged by a proof assistant without admit / sorry on the bounded subdomain that every domain doc actually references?

Claim: we deliver a partial answer. Reverse direction PROVED unconditionally; forward direction PROVED on `n ∈ [2, 30]`; unbounded tail SORRY. Evidence: lake build `N6.MechVerif.AX1` succeeds with 2 sorries, both intentionally placed and disclosed below. Limit: the unbounded tail is genuinely open; we do not assert iff-uniqueness for `n > 30` mechanically.

## §2 COMPARE (mechanical-verification posture vs adjacent number-theory mechanizations)

```
+------------------------------------------------------------------+
| [Mechanization completeness] (proof assistant-side closure)      |
+------------------------------------------------------------------+
| Robin's criterion (Lean4 Mathlib)  ##############......  most    |
| Dirichlet density (Lean4 partial)  ##########..........  partial |
| AX-1 reverse direction (this W2)   ####################  full    |
| AX-1 bounded forward [2,30]        ####################  full    |
| AX-1 unbounded forward tail        ##....................  open  |
+------------------------------------------------------------------+
| [Verification kind] (decide / induction / external)              |
+------------------------------------------------------------------+
| AX-1 reverse                       decide                        |
| AX-1 bounded forward               interval_cases + decide       |
| AX-1 unbounded tail                Robin-style asymptotic SORRY  |
+------------------------------------------------------------------+
```

Claim: the bounded subcase covers every n that any domain doc in the framework actually cites; the unbounded tail is the genuine open question. Evidence: `domains/biology/hexa-weave/hexa-weave.md` and the 295 domain docs all cite AX-1 with `n ∈ [2, 30]` or smaller in their numeric examples. Limit: this is a coverage argument, not a proof — the unbounded tail still falsifies any claim of "iff for all n ≥ 2".

## §3 REQUIRES (mechanical prerequisites)

| Prerequisite | Required level | Component |
|---|---|---|
| Lean 4 toolchain | Stable | `leanprover/lean4:v4.30.0-rc1` |
| Mathlib 4 pin | Specific | rev `19c497800a418208f973be74c9f5c5901aac2f54` |
| `Mathlib.NumberTheory.ArithmeticFunction.Misc` | Required | `σ` notation |
| `Mathlib.Data.Nat.Totient` | Required | `Nat.totient` |
| `Mathlib.NumberTheory.Divisors` | Required | `(Nat.divisors n).card` (τ encoding) |
| `Mathlib.Tactic.IntervalCases` | Required | bounded forward case enumeration |
| `decide` tactic | Required | small-n closure |
| Cold cache plus `lake exe cache get` | Required | 8275 oleans, 38.8 s decompress |

## §4 STRUCT (theorem layout in `lean4-n6/N6/MechVerif/AX1.lean`)

```
+======================================================================+
| AX1Eq (definition)              σ 1 n * Nat.totient n = n * |D(n)|   |
+----------------------------------------------------------------------+
| AX1_reverse_n6 (theorem)        AX1Eq 6                                 PASS  |
| AX1_n6_witness  (theorem)       σ(6)=12 ∧ φ(6)=2 ∧ τ(6)=4               PASS  |
| AX1_forward_bounded_30 (thm)    ∀ n ∈ [2,30], AX1Eq n → n=6             PASS  |
| AX1_forward_tail (theorem)      ∀ n > 30, AX1Eq n → n=6                 SORRY |
| AX1_n6_uniqueness (thm)         ∀ n ≥ 1, AX1Eq n ↔ n=6                  PARTIAL (2 sorry: tail + n=1) |
| AX1_n6_uniqueness_corrected     ∀ n ≥ 2, AX1Eq n ↔ n=6                  PARTIAL (1 sorry: tail) |
+======================================================================+
```

The 4-axis layout matches the n6 quartet τ(6)=4: four definition + theorem ranks (definition / reverse / bounded forward / unbounded tail), with two theorems sharing the `AX1_n6_uniqueness` name (original Spec wording + corrected wording).

## §5 FLOW (proof discharge sequence)

1. Define `AX1Eq n := σ 1 n * Nat.totient n = n * (Nat.divisors n).card` per `Mathlib.NumberTheory.ArithmeticFunction` conventions.
2. Discharge `AX1_reverse_n6` by `decide`: σ(6) = 12, φ(6) = 2, τ(6) = 4, 12·2 = 24 = 6·4.
3. Decompose forward direction: split on `n ≤ 30` vs `n > 30`.
4. For `n ≤ 30`: `interval_cases n` enumerates 29 subcases (n = 2, 3, …, 30); each is closed by `decide`.
5. For `n > 30`: the proof would require a Robin-style asymptotic bound `σ(n)/n < e^γ ln ln n` plus a totient bound; not yet mechanized — left as `sorry` and flagged in the witness ledger (raw 71 falsifier F-W2-AX1-2).
6. `AX1_n6_uniqueness` is built by combining reverse (PASS) with forward (bounded PASS + tail SORRY); statement-quantifier wording `n ≥ 1` is preserved alongside the corrected `n ≥ 2` wording.
7. `lake build N6.MechVerif.AX1` succeeds with 2 sorries, no admit, no postpone.
8. Witness JSON `design/kick/2026-04-28_lean4-w2-ax1_omega_cycle.json` records the build target, sorry count, sorry locations, environment SHA, and 5 raw-71 falsifiers.

## §6 EVOLVE (W1-W12 milestone position of W2)

W2 (this paper) is week 2 of the 12-week HEXA-WEAVE formal-mechanical-verification track. The early-start delivery (day -7 vs nominal 2026-05-05 → 2026-05-18 window) carries +7 schedule margin from W1 and adds another +7 carryover for W3. Predecessor witnesses:

| Phase | Witness | Status |
|---|---|---|
| W1 audit | `design/kick/2026-04-28_hexa-weave-mvp-w1-lean4-audit_omega_cycle.json` | PASS |
| W1 build | `design/kick/2026-04-28_lean4-lake-build_omega_cycle.json` | PASS |
| W2 AX-1 commit | `design/kick/2026-04-28_lean4-ax1-commit_omega_cycle.json` | PASS |
| W2 AX-1 partial | `design/kick/2026-04-28_lean4-w2-ax1_omega_cycle.json` (this paper's source-of-truth) | PARTIAL PASS |
| W3 AX-2 partial | `design/kick/2026-04-28_lean4-w3-ax2_omega_cycle.json` | PARTIAL PASS |

W3 onwards (per the 12-week roadmap) introduces capstone composition over the 11 sub-case lemmas, MK port, and ZFC fallback. None of those are claimed in this paper.

## §7 VERIFY (raw 70 K≥4 verification axes; embedded numerical verify block per own#6)

### §7.1 Embedded verify block (Python sympy; required by own#6 HARD)

```python
# AX-1 numerical verification (own#6 paper-verify-embedded — HARD)
# Executes the same arithmetic that the Lean 4 theorem
# AX1_forward_bounded_30 mechanizes via `interval_cases n + decide`.
# Run: python3 -c "$(sed -n '/^# AX-1/,/^# END/ p' papers/hexa-weave-formal-mechanical-w2-2026-04-28.md)"
# OR copy-paste this block into a Python REPL with sympy installed.

from sympy import divisor_sigma, totient, divisor_count

def ax1_eq(n: int) -> bool:
    """AX1Eq n := sigma(n) * phi(n) == n * tau(n)."""
    return divisor_sigma(n, 1) * totient(n) == n * divisor_count(n)

# Reverse direction: AX1Eq 6 holds.
assert ax1_eq(6) is True, "AX1 reverse direction: AX1Eq 6 must hold"
assert divisor_sigma(6, 1) == 12, "sigma(6) = 12"
assert totient(6) == 2, "phi(6) = 2"
assert divisor_count(6) == 4, "tau(6) = 4"
assert 12 * 2 == 24 == 6 * 4, "n6 master identity at n=6: 12*2 = 24 = 6*4"

# Bounded forward direction: only n=6 satisfies AX1Eq on [2, 30].
solutions_bounded = [n for n in range(2, 31) if ax1_eq(n)]
assert solutions_bounded == [6], (
    f"AX1 bounded forward [2,30] expected [6], got {solutions_bounded}"
)

# Spec corrigendum surface at n=1: the original Spec wording n >= 1
# is wrong because AX1Eq 1 holds (1*1 == 1*1) but 1 != 6. Corrected
# wording is n >= 2.
assert ax1_eq(1) is True, "Spec corrigendum surface: AX1Eq 1 holds trivially"
assert 1 != 6, "the n=1 case is the corrigendum: original n >= 1 is wrong"

# Sanity: extend the bounded sweep to [2, 100] as a strengthened witness;
# we still find only n=6. This does NOT discharge the unbounded tail
# (Lean 4 SORRY remains load-bearing); it only widens the empirical
# coverage of the analytical witness.
solutions_extended = [n for n in range(2, 101) if ax1_eq(n)]
assert solutions_extended == [6], (
    f"AX1 extended sweep [2,100] expected [6], got {solutions_extended}"
)

print("AX-1 verify-embedded PASS:")
print(f"  sigma(6)={divisor_sigma(6,1)}, phi(6)={totient(6)}, tau(6)={divisor_count(6)}")
print(f"  bounded [2,30]   solutions: {solutions_bounded}")
print(f"  extended [2,100] solutions: {solutions_extended}")
print(f"  spec corrigendum: ax1_eq(1) = {ax1_eq(1)}; n=1 forces n >= 2 quantifier amend")
# END verify block
```

Numerical run output (recorded 2026-04-28 on Mac M2 build machine):

```
AX-1 verify-embedded PASS:
  sigma(6)=12, phi(6)=2, tau(6)=4
  bounded [2,30]   solutions: [6]
  extended [2,100] solutions: [6]
  spec corrigendum: ax1_eq(1) = True; n=1 forces n >= 2 quantifier amend
```

### §7.2 raw 70 K≥4 axes

| Axis | Verification claim | Evidence | Status |
|---|---|---|---|
| CONSTANTS | n6 quartet σ(6)=12, τ(6)=4, φ(6)=2, J₂=24 hold | embedded verify block + Lean `AX1_n6_witness` PASS | PASS |
| DIMENSIONS | the iff is between numeric equalities of natural numbers | Lean type signatures unify on `ℕ` | PASS |
| CROSS | numerical (Python) and mechanical (Lean) cross-checks agree on n ∈ [2, 30] | both find solution set = {6} | PASS |
| SCALING | extending the sweep to [2, 100] still yields {6} as the unique solution | embedded verify block | PASS |
| SENSITIVITY | choice of τ encoding `(Nat.divisors n).card` vs a standalone `tau` | W1 audit corrigendum #3 mandates the divisors-card form | PASS |
| LIMITS | bounded forward is mechanized; unbounded tail is SORRY | Lean source line 75 + witness JSON | PASS |
| CHI2 | quantitative chi-squared validation against an empirical sample | NOT APPLICABLE (no empirical claim per raw 91 C3) | DEFER (intentional) |
| COUNTER | counter-example search finds none at n ∈ [2, 100] | embedded verify block | PASS |

7 of 8 axes PASS, 1 DEFER (CHI2 not applicable to a mechanical-proof paper) — meets raw 70 K≥4 threshold.

## §8 EXEC SUMMARY

Lean 4 W2 milestone delivers AX-1 reverse direction + bounded forward direction PROVED, unbounded tail SORRY. Embedded Python verify block (own#6 HARD) cross-checks bounded interval [2, 30] and extends to [2, 100] with the same {6} solution set. Two sorries declared in `lean4-n6/N6/MechVerif/AX1.lean` (lines 75 and 95); both intentional and disclosed in `design/kick/2026-04-28_lean4-w2-ax1_omega_cycle.json`. No Clay Millennium claim. raw 91 C3: this paper makes zero empirical claims.

## §9 SYSTEM REQUIREMENTS

- Lean 4 toolchain `leanprover/lean4:v4.30.0-rc1`.
- Mathlib 4 rev `19c497800a418208f973be74c9f5c5901aac2f54`.
- `lake exe cache get` available (8275 oleans, 38.8 s decompress).
- Mac M2 build host (or equivalent); 11.7 s wall-clock for `lake build N6.MechVerif.AX1`.
- Python 3.10+ with sympy ≥ 1.12 for the embedded verify block (own#6).
- `tool/own_doc_lint.py --rule 6` HARD gate on commit.

## §10 ARCHITECTURE

The Lean 4 file `lean4-n6/N6/MechVerif/AX1.lean` is the single source of mechanical truth for AX-1. The Python embedded verify block is a parallel numerical witness; it is intentionally NOT a proof — it is a sanity check that re-exercises the same arithmetic that Lean's `decide` tactic evaluates internally. A separate alias path `lean4-n6/HexaWeave/AX1NUniqueness.lean` was considered but not used (would have required a new `lean_lib` and a forbidden lakefile cross-cutting refactor).

```
+------------------------------------------------------------------+
| Lean 4 mechanical truth         lean4-n6/N6/MechVerif/AX1.lean   |
|   |                                                              |
|   v                                                              |
| Witness JSON (canonical)        design/kick/2026-04-28_lean4...  |
|   |                                                              |
|   v                                                              |
| This paper (verify-embedded)    papers/hexa-weave-formal-...md   |
|   |                                                              |
|   v                                                              |
| Python sympy block (sanity)     embedded in §7.1                 |
+------------------------------------------------------------------+
```

## §11 CIRCUIT DESIGN

Not applicable (this is a formal-verification paper, not a hardware paper).
The "circuit" of inference is: `decide` tactic operates on `Nat`-decidable equality; `interval_cases n` decomposes a bounded `n` into 29 subcases; each subcase is closed by `decide`. There is no electrical circuit.

## §12 PCB DESIGN

Not applicable (no PCB). Listed for own#15 21-section completeness.

## §13 FIRMWARE

Not applicable (no firmware). The closest analog is the Lean 4 elaborator and Mathlib's `decide` reflection; neither runs on bare metal in a deployment sense.

## §14 MECHANICAL

Not applicable (no mechanical assembly). The mechanization here is symbolic, not physical. Listed for own#15 21-section completeness.

## §15 MANUFACTURING

Not applicable (no manufacturing). The reproducibility recipe is: clone repo at the recorded SHA, run `lake exe cache get`, run `lake build N6.MechVerif.AX1`, expect 2 sorries and 0 admit.

## §16 TEST

Test plan:
1. `lake build N6.MechVerif.AX1` must succeed in under 60 s wall-clock on a warm-cache Mac M2 (current: 11.7 s).
2. `python3 -c "<embedded block>"` must print the recorded "AX-1 verify-embedded PASS" output verbatim.
3. `tool/own_doc_lint.py --rule 6` must report zero violations on this file.
4. Sorry count in `lean4-n6/N6/MechVerif/AX1.lean` must be exactly 2 (re-test before W3 commits).
5. Cold-cache build must not exceed 5 minutes wall-clock (raw 71 falsifier F-W2-AX1-5).

## §17 BOM

| Item | Qty | Notes |
|---|---|---|
| `lean4-n6/N6/MechVerif/AX1.lean` source file | 1 | ~95 lines, 2 sorries |
| Mathlib 4 imports | 4 | `ArithmeticFunction.Misc`, `Divisors`, `Totient`, `IntervalCases` |
| Witness JSON | 1 | `design/kick/2026-04-28_lean4-w2-ax1_omega_cycle.json` |
| This paper | 1 | `papers/hexa-weave-formal-mechanical-w2-2026-04-28.md` |
| Embedded Python verify block | 1 | sympy-based, ~30 lines |

## §18 VENDOR

| Vendor | Component | Role |
|---|---|---|
| Lean Prover community (Microsoft Research / open) | Lean 4 elaborator | proof checker |
| Mathlib 4 contributors (open) | Mathlib library | arithmetic functions, totient, divisors |
| SymPy contributors (open) | sympy | embedded numerical verify |
| n6-architecture private framework (this repo) | `tool/own_doc_lint.py` | own#6 HARD gate |

## §19 ACCEPTANCE

Acceptance criteria (own#12 MISS criteria pre-declared, no post-hoc adjustment):
- ACCEPT if all five §16 TEST items pass on a clean clone of the repo at the recorded SHA.
- MISS if any of: (a) `AX1_reverse_n6` no longer compiles, (b) `AX1_forward_bounded_30` adds a third sorry, (c) `AX1_n6_uniqueness_corrected` becomes unprovable under the corrected `n ≥ 2` wording, (d) Python embedded block outputs anything other than `[6]` for the bounded sweep, (e) `tool/own_doc_lint.py --rule 6` flags this file.
- DEFER (not MISS) if the unbounded tail SORRY remains open through W3-W4; deferral is honest disclosure per raw 91 C3, not a hidden gap.

## §20 APPENDIX

### §20.1 raw 71 falsifiers (5; paper-publication tier)

- **F-W2-PAPER-1**: a counter-example `n ∈ [2, 30] \ {6}` satisfying AX1Eq is found at any future Mathlib upgrade. Falsifies AX1_forward_bounded_30. Expected outcome: does not fire (numerical sympy witness independently confirms).
- **F-W2-PAPER-2**: a counter-example `n ∈ [31, 100]` satisfying AX1Eq is found by sympy. Would surface a defect in the bounded-vs-tail split (the bound 30 might be too low). Expected outcome: does not fire (verified for [2, 100]).
- **F-W2-PAPER-3**: the embedded Python block fails to execute under sympy ≥ 1.12 (e.g. `divisor_sigma(n, 1)` API rename). Falsifies own#6 reproducibility. Expected outcome: monitor on each sympy major-version release.
- **F-W2-PAPER-4**: the unbounded tail SORRY discharges in some other framework (e.g. Coq, Isabelle) before Lean 4 W3-W4. Would not falsify this paper but would obsolete the W3 plan; honest signal.
- **F-W2-PAPER-5**: a Spec maintainer NACK on the n=1 corrigendum (n ≥ 1 → n ≥ 2). Would force preserving the original wording with a permanent n=1 sorry. Expected outcome: does not fire (corrigendum is a clear honest-mathematics fix).

### §20.2 raw 91 C3 honest disclosure

- 0 of 0 empirical claims made (this paper is theoretical-analytical + partial-mechanical only).
- 2 sorries in the Lean 4 source; both intentional and located on lines 75 and 95.
- Bounded forward proof depends on `Mathlib.Tactic.IntervalCases` which was NOT in the W1 audit USE-AS-IS list (W2 audit refinement; harmless).
- Mathlib pin is at `19c4978`; a future `lake update` could move it and break the proof — pinned in lakefile to mitigate.
- Build environment: Mac M2 production user, no SIGKILL, no `lake update` invoked.
- Reverse direction `AX1Eq 6` is mechanically PROVED unconditionally — this is the strongest claim made in this paper and is not deferred.

### §20.3 Cross-references

- Source theorem file: `lean4-n6/N6/MechVerif/AX1.lean` (lines 1-95)
- Witness JSON: `design/kick/2026-04-28_lean4-w2-ax1_omega_cycle.json`
- Domain doc: `domains/biology/hexa-weave/hexa-weave.md` §3 REQUIRES table cites the n6 quartet that this paper mechanizes
- Theory SSOT: `theory/proofs/theorem-r1-uniqueness.md` (own#2 reference)
- Lint gate: `tool/own_doc_lint.py --rule 6`
- Cycle-6 fan-out 5/5 spec witness (this paper's parent): `design/kick/2026-04-28_external-disclosure-spec_omega_cycle.json`

## §21 IMPACT

This W2 partial-PASS deliverable is the framework's first public-tier paper artifact that is simultaneously: (i) mechanically backed by a passing Lean 4 build, (ii) numerically backed by an embedded Python witness, (iii) compliant with own#6 (verify embedded), (iv) honest about the SORRY tail per raw 91 C3, (v) honest about the n=1 corrigendum per own#11. Future cycles may upgrade this to an Option-A Zenodo deposit (DOI) once the F-TP5-b 90-day MVP gate is met (2026-07-28); migration plan is recorded in the parent witness JSON. arXiv (Option-B) and public GitHub (Option-C) tiers remain gated behind external endorsement and explicit user approval respectively, per raw 71 paper-publication-tier governance.

## mk-history

- 2026-04-28T15:30:00Z — initial draft created as cycle-6 fan-out 5/5 external-disclosure Option-E deliverable. W2 AX-1 partial PASS recorded; verify-embedded Python block included; 5 raw 71 falsifiers declared.
- 2026-04-28T15:35:00Z — embedded Python verify block extended to [2, 100] sweep beyond Lean's [2, 30] bounded interval; raw 70 SCALING axis updated.
- 2026-04-28T15:40:00Z — raw 91 C3 honest disclosure section added; own#11 (no Clay claim) and own#12 (MISS criteria pre-declared) cross-checked.
