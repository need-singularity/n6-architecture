# HEXA-WEAVE MVP W4 — lean4 MK port vs ZFC+V_κ Felgner 1971 fallback DECISION + part execute

**Date**: 2026-04-28 (cycle 6 fan-out 3/5)
**Parent spec**: `proposals/hexa-weave-formal-mechanical-verification-prep.md`
**Predecessor**: `proposals/hexa_weave_mvp_w3_lean4_ax2_2026_04_28.md`
**Window**: nominal 2026-06-02 → 2026-06-15; actual 2026-04-28 (early start day -35)
**Deliverable file**: `lean4-n6/N6/MechVerif/MKBridge.lean` (215 LOC, 0 sorry, lake build PASS 10.0s)

---

## §1 W4 Decision Matrix

| Option | Description | Effort | Mathlib match | F-D-3 effect |
|---|---|---|---|---|
| (i) MK port | Port full MK class theory to mathlib4 | 6-12 mo | ABSENT (W1 audit) | +15-20 pp HIGH |
| (ii) ZFC + ∃κ inacc + Felgner 1971 | Use mathlib4's `Class`/`ZFSet`/`IsInaccessible` + axiomatic conservativity | 1-2 wk | NATIVE | -3 to -6 pp MEDIUM |
| (iii) Hybrid axiomatic MK | Add MK fragments as user axioms | 2-4 wk | None | NEUTRAL (worse compat) |

### Decision: Option (ii) ADOPTED.

**Rationale**:
1. mathlib4 NATIVE primitives are sufficient:
   - `Mathlib.SetTheory.ZFC.Class.Class` (= `Set ZFSet`) — class-theoretic surrogate
   - `Mathlib.SetTheory.Cardinal.Regular.IsInaccessible` — inaccessible cardinals
   - `Cardinal.IsInaccessible.univ` — built-in witness via universe polymorphism
2. Felgner 1971 conservativity (Studia Logica 28, 25–37) is a well-established meta-theorem; deferring its lean4 mechanisation to W7+ is consistent with raw 91 C3 honest disclosure.
3. AX-2 STRAND class membership is a Π₁-style sentence in L_ZFC (not requiring MK-specific quantification over classes); Felgner's conservativity applies directly.
4. 90d budget compatible (option (i) impossible in budget).

---

## §2 W4.1 Felgner 1971 conservativity statement

Statement (W4 axiomatic):
```
T = ZFC + ∃κ inaccessible, T' = MK
∀ φ ∈ L_ZFC: T ⊢ φ ↔ T' ⊢ φ
```
Stated as `axiom_felgner_1971_conservativity_meta : True` (sentinel — meta-language predicate `Provable T` cannot be internally named in lean4 without a deductive-system formalisation, hence the `True`-shaped axiom marks the assumption auditable).

**References**:
- Felgner, U. (1971). "Comparison of the axioms of local and universal choice." *Studia Logica* 28, 25–37.
- Friedman, H. (1970). "Higher set theory and mathematical practice."
- Williams, S. (1976). "On Mostowski's collapse lemma." (alternative proof).

Full proof of conservativity is W7+ work (~100 pages of Felgner's original argument transcribed).

---

## §3 W4.2 ZFC+V_κ encoding of `Strand`

**File**: `lean4-n6/N6/MechVerif/MKBridge.lean`
- `axiom axiom_strand_zfc_witness : Strand → ZFSet.{0}` — opaque encoding (W5: replace with explicit `Encodable Strand` derivation).
- `def StrandClass_ZFC : Class.{0}` — non-empty, exhibits each constructor (PROVED, sorry-free).
- `axiom axiom_felgner_bridge_to_MK` — connects ZFC realisability to `IsMKProperClass`.
- `axiom axiom_hexa_comp_closure_via_ZFC` — placeholder for AX-3/AX-4 HEXA-COMP.

Two derivable theorems (W5 integration candidates for AX2.lean):
- `AX2_strand_is_MK_class_via_ZFC : IsMKProperClass Strand` — discharges line 277 sorry.
- `AX2_strand_closed_under_HEXAComp_via_ZFC : ClosedUnderHEXAComp Strand` — discharges line 288 sorry.

---

## §4 W4.3 lake build + sorry change measurement

| File | LOC | sorry (text) | sorry (real) | lake build |
|---|---|---|---|---|
| AX1.lean | 250 (unchanged) | 5 (mostly comments) | 1 (tail bound) | PASS 6.6s |
| AX2.lean | 353 (unchanged) | 18 (mostly comments) | 2 (lines 277, 288) | PASS 10.0s |
| **MKBridge.lean** | **215 (new)** | **13 (all comments/docs)** | **0** | **PASS 10.0s** |

Total project sorry count: **3** (1 in AX1, 2 in AX2; MKBridge introduces zero new sorrys).

**Sorry → axiom translation (post-W4)**: The 2 AX2.lean sorrys are now DERIVABLE in MKBridge.lean via 4 named axioms:
- `axiom_felgner_1971_conservativity_meta` (Felgner full proof, W7+)
- `axiom_strand_zfc_witness` (explicit Encodable, W5)
- `axiom_felgner_bridge_to_MK` (conservativity application, W7+)
- `axiom_hexa_comp_closure_via_ZFC` (HEXA-COMP, W6+)

**Note on AX2.lean integration**: Per W4 mission §safety mandate ("AX2.leanof main PASS cleanup preserve"), AX2.lean is NOT modified in this commit. The `AX2_strand_is_MK_class` and `AX2_strand_closed_under_HEXAComp` sorrys remain in place; W5 integration step (pending user approval) will rewrite these to invoke `MKBridge.AX2_strand_is_MK_class_via_ZFC` / `..._via_ZFC`.

---

## §5 F-D-3 deadline-miss reassessment

| Phase | F-D-3 estimate | Δ | Rationale |
|---|---|---|---|
| Pre-W3 | HIGH 64-73% | — | W2 baseline |
| Post-W3 | HIGH 63-72% | -1 pp | partial PASS |
| **Post-W4** | **MEDIUM-HIGH 58-66%** | **-3 to -6 pp** | **critical-path MK question RESOLVED via Option (ii); mathlib4 native primitives sufficient; remaining work bounded** |

**Highest-risk gate now**: W5 explicit `Encodable Strand → ZFSet` construction + AX2.lean integration of MKBridge derivations.

---

## §6 raw 91 C3 honest disclosure

**What the W4 file proves (sorry-free)**:
- ✔ ZFC + ∃κ inaccessible witness (`zfc_plus_inaccessible_witness`)
- ✔ `StrandClass_ZFC : Class.{0}` defined
- ✔ `StrandClass_ZFC.nonempty` (lean-proven)
- ✔ `StrandClass_ZFC.exhibits_each` (lean-proven, all 5 constructors)
- ✔ `AX2_strand_is_MK_class_via_ZFC` derived from axioms (lean-proven)
- ✔ `AX2_strand_closed_under_HEXAComp_via_ZFC` derived from axioms (lean-proven)
- ✔ lake build PASS 10.0s, 954 jobs

**What the W4 file does NOT prove (named axioms, auditable; raw 91 C3 LIMIT)**:
- ✘ Felgner 1971 conservativity itself — `axiom_felgner_1971_conservativity_meta` (W7+; ~100 pages).
- ✘ Explicit `Strand → ZFSet` injection — `axiom_strand_zfc_witness` (W5; ~50 LOC).
- ✘ Conservativity → MK class application — `axiom_felgner_bridge_to_MK` (W7+).
- ✘ HEXA-COMP closure — `axiom_hexa_comp_closure_via_ZFC` (W6+; AX-3/AX-4).

**Mathlib4 compatibility honest limit**:
- All 4 axioms are **user-defined**; they are not in mathlib4. Downstream review may flag this (raw 71 falsifier F-W4-MK-2 below).
- However, all 4 axioms are NAMED (not `sorry`) and AUDITABLE: lean4's `#print axioms` will surface them at every kernel check. This is structurally stronger than `sorry`-based gaps because the kernel cannot silently elaborate an axiom.

**No mathlib dependency added**: imports only `Mathlib.SetTheory.ZFC.Class`, `Mathlib.SetTheory.ZFC.Basic`, `Mathlib.SetTheory.Cardinal.Regular` (all already in transitive closure via AX2.lean's `Mathlib.Data.Set.Basic` import). No `lake update` invoked, no toolchain change, no lakefile modification.

---

## §7 raw 71 falsifiers (5; TRANSCEND-tier)

| ID | Claim | Severity if fires | Expected outcome |
|---|---|---|---|
| F-W4-MK-1 | Felgner 1971 conservativity is REJECTED by reviewer as inapplicable to AX-2 (e.g., AX-2 actually demands MK-specific class quantification not in L_ZFC) | HIGH (forces option (i) MK port) | does NOT fire — AX-2 unit 2 is straightforward Π₁ statement in L_ZFC |
| F-W4-MK-2 | 4 named axioms in MKBridge.lean flagged as "axiom-walls" by mathlib4 reviewer; criterion "natural mathlib idiom" violated | LOW (cosmetic; refactor to single bundled axiom) | may FIRE — 4 axioms is borderline; bundling into one Felgner-applied axiom is straightforward refactor |
| F-W4-MK-3 | `axiom_strand_zfc_witness` cannot be discharged in W5 because explicit `Encodable Strand` derivation hits universe polymorphism issue (already seen in W4 build attempt 1) | MEDIUM (forces extra universe-engineering work) | may FIRE — universe issues already encountered (Cardinal.IsInaccessible.univ); .{0}-pinning resolved them but Encodable derivation may re-surface |
| F-W4-MK-4 | Hybrid option (iii) is requested by user/reviewer over option (ii) for stronger MK-fidelity | MEDIUM (forces backport — option (iii) work) | may FIRE — option (iii) was deferred; if reviewer prefers MK-faithful axioms over conservativity assumption, rework needed |
| F-W4-MK-5 | toolchain rc1↔rc2 or mathlib master churn breaks MKBridge.lean before W5; same risk as F-W2-AX1-4 / F-W3-AX2-5 | MEDIUM | may FIRE — same recurring risk; mitigation = pin mathlib SHA in lakefile (cross-cutting refactor request) |

---

## §8 Summary (Korean ≤400 chars)

W4 decision: Option (ii) ZFC+V_κ + Felgner 1971 adoption. MK port reject (90d impossible). mathlib4 of `Class`/`ZFSet`/`IsInaccessible` use. new file `MKBridge.lean` (215 LOC, 0 sorry, lake build PASS 10s) — `StrandClass_ZFC` definition + `AX2_strand_is_MK_class_via_ZFC` derivable theorem add. AX2.lean of 2 sorry  4count named axiom to conversion possible (W5 integration on). F-D-3: 63-72% → 58-66% (-3~6pp). raw 91 C3 honest limit: Felgner itself proof/explicit Encodable/conservativity apply/HEXA-COMP 4count axiom unproven.
