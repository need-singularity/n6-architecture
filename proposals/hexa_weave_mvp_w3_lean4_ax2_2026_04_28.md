---
category: operational
umbrella: formal-verification
parent_spec: proposals/hexa-weave-formal-mechanical-verification-prep.md
predecessor: proposals/hexa_weave_mvp_w2_lean4_ax1_2026_04_28.md
milestone: W3
date: 2026-04-28
w3_partial_completed: true
---

# HEXA-WEAVE MVP — W3 lean4 `thm.AX2_strand_class_well_formed` first-attempt report

> **W3 deliverable** for `proposals/hexa-weave-formal-mechanical-verification-prep.md` Task D Spec §4 unit 2.
>
> **Date**: 2026-04-28 (cycle 5 fan-out 2/5; early start; W3 nominal window 2026-05-19 → 2026-06-01).
>
> **Build basis**: mathlib4 master @ rev `19c497800a418208f973be74c9f5c5901aac2f54`, lean4 `v4.30.0-rc1` (toolchain unchanged from W2).
>
> **W3 partial PASS** — `Strand` inductive type defined (5 constructors); 5-way disjunction totality + per-disjunct existence + `Set Strand`-level non-emptiness all PASS sorry-free. **Two load-bearing `sorry`** for the MK class-theory bridge (`AX2_strand_is_MK_class`) and HEXA-COMP closure (`AX2_strand_closed_under_HEXAComp`); both are surfaced via `opaque` predicates so the gaps are auditable. Per raw 91 C3 honest mandate.

---

## §1 W3 deliverable — what landed

**File**: `lean4-n6/N6/MechVerif/AX2.lean` (NEW, 353 lines)

**Encoding choice (mission §step 1)**: option (b) — lean4 INDUCTIVE TYPE.
- Rationale: lean4-natural; cleaner than ZFC string-encoding (option a) and more informative than axiomatic-existence (option c).
- Cost: introduces a META-LEVEL gap between the lean4 inductive type and the Spec §4 unit 2 MK proper class. This gap is the load-bearing `sorry`.

**STRAND inductive type — 5 constructors per Spec §4 unit 2**:

```lean
inductive Strand where
  | aminoAcid (seq : List AminoAcid)         : Strand
  | rna       (seq : List RNANucleotide)     : Strand
  | dna       (seq : List DNANucleotide)     : Strand
  | smallLigand (smiles : String)            : Strand
  | antibody  (heavy : List AminoAcid)
              (light : List AminoAcid)       : Strand
  deriving Repr
```

- `AminoAcid` — 22 constructors (20 standard + Sec + Pyl).
- `RNANucleotide` — 4 constructors {A, U, G, C}.
- `DNANucleotide` — 4 constructors {A, T, G, C}.
- `smallLigand` — opaque `String` payload (SMILES).
- `antibody` — pair of `List AminoAcid` (heavy + light chain).

**Theorems delivered (sorry-free unless noted)**:

| Name | Statement | Proof status |
|------|-----------|--------------|
| `Strand` (inductive) | 5-constructor type | DEFINED |
| `Strand.is{AminoAcid,RNA,DNA,SmallLigand,Antibody}` (defs) | per-constructor predicates | DEFINED |
| `Strand.cover_total` | every strand satisfies one of 5 predicates | PASS via `cases` |
| `Strand.witness{AminoAcid,RNA,DNA,SmallLigand,Antibody}` | 5 concrete witnesses | DEFINED |
| `Strand` `Inhabited` instance | `default := witnessAminoAcid` | PASS |
| `Strand.exists_each` | each disjunct realized | PASS |
| `Strand.universe_nonempty` | `(Set.univ : Set Strand).Nonempty` | PASS |
| `StrandClass` (def) | `Set.univ : Set Strand` | DEFINED |
| `StrandClass.contains_all` | every strand in StrandClass | PASS |
| `StrandClass.nonempty` | StrandClass non-empty | PASS |
| `StrandClass.exhibits_each` | each disjunct realized in StrandClass | PASS |
| `AX2_class_formation_in_MK` | weak class-formation w/ closure under constructors | PASS (object-level only) |
| `AX2_translation_fidelity_to_MK` | meta-statement sentinel | trivial PASS (`True`) |
| `IsMKProperClass` (opaque) | "α is an MK proper class" | OPAQUE PRIMITIVE |
| `ClosedUnderHEXAComp` (opaque) | "α is closed under HEXA-COMP" | OPAQUE PRIMITIVE |
| `AX2_strand_is_MK_class` | `IsMKProperClass Strand` | **`sorry`** (load-bearing, W4-W5 discharge) |
| `AX2_strand_closed_under_HEXAComp` | `ClosedUnderHEXAComp Strand` | **`sorry`** (load-bearing, W6+ discharge) |
| `AX2_strand_class_well_formed` | (a)-(d) conjunction main statement | PASS sorry-free |

**Build verification**:

```
$ lake build N6.MechVerif.AX2
⚠ [351/351] Built N6.MechVerif.AX2 (10s)
warning: N6/MechVerif/AX2.lean:277:8: declaration uses `sorry`   ← AX2_strand_is_MK_class
warning: N6/MechVerif/AX2.lean:288:8: declaration uses `sorry`   ← AX2_strand_closed_under_HEXAComp
Build completed successfully (351 jobs).
```

10.0 s warm-cache wall-clock; 351 jobs (subset of 1310 from W2 since AX2 imports only `Mathlib.Data.Set.Basic` not the full ArithmeticFunction stack).

---

## §2 STRAND inductive type definition — design rationale

**Why inductive (option b) over ZFC string encoding (a) or axiomatic (c)?**

| Option | Pro | Con |
|--------|-----|-----|
| (a) ZFC string + alphabet | most faithful to "set of finite strings" Spec gloss | introduces `String → Strand` decoder, increases proof burden, no lean4 ergonomic gain |
| (b) lean4 `inductive` ★ chosen | type-safe pattern match, mathlib-friendly, direct Repr | meta-gap to MK proper class (the load-bearing `sorry`) |
| (c) axiomatic existence | smallest surface | uninformative; cannot be exhibited or computed |

**Result**: option (b) gives sorry-free behavior for everything ABOVE the MK bridge (constructors exist, disjunction total, witnesses exhibited, set-level non-empty), and isolates the MK-bridge gap into one explicit `sorry` via the `opaque IsMKProperClass` predicate.

---

## §3 trivial existence + `lake build` result

- `Strand.universe_nonempty : (Set.univ : Set Strand).Nonempty` — PASS (uses `default` from `Inhabited Strand`).
- `Strand.exists_each` — 5-way conjunction of per-constructor existence — PASS.
- `StrandClass.nonempty` — PASS (= `universe_nonempty`).
- `AX2_strand_class_well_formed` — main composite statement combining (a) inhabitation, (b) totality, (c) per-disjunct existence, (d) class-level non-emptiness — PASS sorry-free.

`lake build N6.MechVerif.AX2` exits with status 0; only the two intentional `sorry`-warnings printed.

---

## §4 `sorry` location + count

**Sorry count: 2** (both intentional, load-bearing per raw 91 C3).

| Line | Theorem | Claim | Discharge plan |
|------|---------|-------|-----------------|
| 277 | `AX2_strand_is_MK_class` | `IsMKProperClass Strand` | W4-W5: ZFC+V_κ comprehension via Felgner 1971 conservativity |
| 288 | `AX2_strand_closed_under_HEXAComp` | `ClosedUnderHEXAComp Strand` | W6+: define HEXA-COMP operator, prove image-closure |

Both predicates are stated via `opaque IsMKProperClass : Type → Prop` and `opaque ClosedUnderHEXAComp : Type → Prop` so that lean4's logic does NOT silently accept them — any attempt to prove `IsMKProperClass Strand` without an `axiom` extension or full MK formalization will fail to elaborate. This is the strongest honest framing available in W3.

**Comparison with W2 sorry budget**: W2 had 2 sorries (tail bound + n=1 corrigendum); W3 also has 2 (MK bridge + HEXA-COMP closure). Sorry-count parity does NOT mean equivalent severity — see §5.

---

## §5 AX-2 vs AX-1 difficulty comparison (semantics encoding)

| Dimension | AX-1 (W2) | AX-2 (W3) |
|-----------|-----------|-----------|
| Mathlib coverage | `σ`, `Nat.totient`, `Nat.divisors` all present | `Strand`, `AminoAcid`, etc. ABSENT — must be defined |
| Statement complexity | single `↔` proposition | conjunction of 4 sub-claims (a)-(d) |
| Bounded computability | `decide` works for n ≤ 30 | inductive `cases` works for 5 constructors |
| Tail / unbounded portion | Robin asymptotic (HARD) | MK class-theory bridge (HARDER) |
| Sorry severity | tail = "nice to have" (capstone fallback) | MK bridge = STRUCTURAL (no fallback in lean4) |
| New definitions introduced | 0 | 4 (`AminoAcid`, `RNANucleotide`, `DNANucleotide`, `Strand`) + 2 opaque (`IsMKProperClass`, `ClosedUnderHEXAComp`) |
| Build time | 6.6 s (module) | 10.0 s (module, lighter imports) |

**Net assessment**:
- AX-2 lean4 SURROGATE proof was EASIER than AX-1 forward direction (no asymptotic combinatorics; just inductive type + 5 cases).
- AX-2 META-LEVEL bridge (lean4 inductive ↔ MK proper class) is HARDER than AX-1 tail bound (Robin asymptotic). The bridge is BLOCKED by mathlib4 MK absence (W1 audit).
- raw 91 C3 honest call: "AX-2 is NOT proved in T_MK-HW; we have a lean4 surrogate that depends on a meta-level translation that requires W4-W5 work to mechanize."

---

## §6 F-D-3 (90-day deadline) re-assessment after W3 partial

**Pre-W3 baseline** (per W2 report §6): F-D-3 probability **HIGH 64-73%**.

**W3 evidence**:
- W3 partial PASS (mission honesty mandate "W3 partial on −1pp"): −1pp on lower bound.
- BUT W3 surfaces a STRUCTURAL gap (MK absent in mathlib4) that was already known from W1 audit but is now load-bearing in code: NO additional ppm change but RISK CONCENTRATION confirmed.
- HEXA-COMP closure surfaced as second load-bearing `sorry` (was not on W3 critical path explicitly; mission §raw-91-C3 honest mandate to surface it now).
- **probability adjustment**: −1pp (W3 partial PASS); the +0pp neutral on MK-bridge already priced into the 64-73% range from W1.
- **net**: from HIGH 64-73% → HIGH **63-72%**.
- Per mission §F-D-3 "fail on +3pp": NOT applicable here (W3 is partial PASS, not fail).

**Caveat**: W4-W5 (MK port / ZFC fallback) is now the highest-risk gate. If `IsMKProperClass Strand` cannot be discharged by W6, then AX-2 (and dependent AX-3/AX-4) cannot reach GREEN-CI without an explicit `axiom` extension to lean4, which would itself require user approval per mission §safety jetsam.

---

## §7 raw 71 ≥5 falsifiers (TRANSCEND-tier) — preregistered for W3 work

Five strong falsifiers preregistered for **THIS W3 RESULT** (separate from Spec §8 F-D-1..5, W1 audit F-W1-1..5, and W2 F-W2-AX1-1..5):

- **F-W3-AX2-1** — *the inductive `Strand` 5-constructor encoding is REJECTED by Spec §4 unit 2 maintainer as an inadequate translation of the MK STRAND class*
  - condition: by 2026-06-01 (W5 end), Spec author or L5 closure-witness pipeline issues a NACK on the option-(b) inductive-type encoding choice; alternative ZFC-string (a) or axiomatic (c) encoding demanded.
  - experiment: open Spec amendment proposal documenting encoding-choice rationale; collect maintainer disposition.
  - expected outcome: does NOT fire — mission §step 1 explicitly endorsed option (b). MK-bridge is the remaining gap; encoding choice itself is acceptable.
  - severity if fires: HIGH (would force complete rewrite of `AX2.lean`).

- **F-W3-AX2-2** — *the `opaque IsMKProperClass` + `sorry` framing is REJECTED as semantically empty (the sorry can be discharged by adding a vacuous `axiom`, defeating its honest-gap purpose)*
  - condition: a reviewer or downstream consumer issues a critique that `IsMKProperClass` could be inhabited via `axiom mk_class_exists : IsMKProperClass Strand`, making the gap-disclosure cosmetic.
  - experiment: write a counter-defense doc explaining `opaque` semantics + audit trail; collect reviewer disposition.
  - expected outcome: may FIRE at LOW severity — `opaque` + sorry is the strongest honest framing in lean4 short of a full MK port; the critique is technically correct but applies to ANY axiomatic extension.
  - severity if fires: LOW (adopt explicit `axiom` declaration with disclosure paragraph; behavior unchanged).

- **F-W3-AX2-3** — *the MK-bridge `sorry` cannot be discharged by W6 even via ZFC+V_κ fallback (Felgner 1971 mechanization is too large a task)*
  - condition: by 2026-06-15 (W7 end), `lean4-n6/N6/MechVerif/MK_Bridge.lean` (W4-W5 stub) carries `sorry` count ≥ 5 and no progress on Felgner conservativity.
  - experiment: track per-week MK_Bridge.lean sorry count + lake build time.
  - expected outcome: FIRES at MEDIUM severity — Felgner 1971 is a 100+ page result; lean4 mechanization within W4-W5 (2 weeks) is aggressive.
  - severity if fires: MEDIUM (forces axiom-based path; F-D-3 +5pp).

- **F-W3-AX2-4** — *the `Strand` inductive type's `String` payload (`smallLigand`) introduces a representation gap (no SMILES validity check) that a reviewer flags as "not biology-faithful"*
  - condition: external reviewer or n6-arch maintainer flags the lack of SMILES validation as a faithfulness defect during W12 review.
  - experiment: track review feedback during W11-W12; record if flag raised.
  - expected outcome: FIRES at LOW severity — the abstraction layer (Strand carries opaque payloads) is documented in §8 of `AX2.lean`; the choice is defensible but a stricter encoding may be requested.
  - severity if fires: LOW (cosmetic refactor to `Subtype String IsValidSMILES` with sorry on validity).

- **F-W3-AX2-5** — *toolchain rc1↔rc2 churn re-occurs (continuation of F-W2-AX1-4) and breaks AX2.lean build before W4 milestone*
  - condition: by 2026-05-18, automatic mathlib master sync moves project to rc2 again, breaking AX2.lean elaboration.
  - experiment: pin `lean-toolchain` AND mathlib rev in lakefile; track weekly `lake build` health.
  - expected outcome: may FIRE at MEDIUM severity — same risk as F-W2-AX1-4; mitigated by mathlib rev pin.
  - severity if fires: MEDIUM (forces lakefile mathlib SHA pin).

Five falsifiers preregistered satisfy raw 71 TRANSCEND-tier ≥5 mandate.

---

## §8 raw 91 C3 honest disclosure

- **`sorry` count**: 2 in delivered file. Both are intentional placeholders for STRUCTURAL gaps (MK class-theory bridge, HEXA-COMP closure). No hidden `sorry`, no `admit`. 11 of 13 theorems carry zero `sorry`.
- **`AX2_strand_is_MK_class` is the load-bearing structural sorry** — without it, the Spec §4 unit 2 statement "STRAND is an MK proper class" is unproven. The honest claim is **"`Strand` lean4 inductive type DEFINED, 5 constructors EXHIBITED, lean4 `Set Strand`-level class non-empty PROVED, but MK class-theory bridge UNMECHANIZED"**, NOT "`thm.AX2_strand_class_well_formed` PROVED".
- **`AX2_strand_closed_under_HEXAComp` is a secondary structural sorry** surfaced because Spec §4 unit 2 demands "STRAND closed under HEXA-COMP" but HEXA-COMP itself is not yet mechanized in lean4. Discharge plan: define HEXA-COMP operator in W6+ together with AX-3 ENCODES relation.
- **biology semantics encoding limit**: the inductive constructors take `List AminoAcid` / `List RNANucleotide` / `String` payloads with NO biological-validity predicate. A real ribosome can produce `List [Ala, Stop, Trp, ...]` which is not a valid translation product, but the lean4 type accepts it. This is a deliberate abstraction — option (b) chosen over option (a) precisely because biological-validity is OUT OF SCOPE for AX-2 well-formedness; valid-strand semantics belongs to AX-3 (ENCODES) or higher.
- **`opaque` predicates are auditable**: `IsMKProperClass : Type → Prop` and `ClosedUnderHEXAComp : Type → Prop` are `opaque` (no canonical body), which means lean4's elaborator cannot silently inhabit them. The two `sorry`s are therefore visible at every kernel check.
- **lake build ran on production user's Mac M2** (mission §safety jetsam concern). 10.0 s warm-cache; mathlib cache hit; no SIGKILL observed. 351 jobs (lighter than W2's 1310 because AX2 imports only `Mathlib.Data.Set.Basic`).
- **No new dependencies added; no `lake update` invoked; no toolchain change required.**
- **W3 nominal window** is 2026-05-19 → 2026-06-01; this delivery is on **day −21** (early by 3 weeks; running ahead of schedule with carryover from W1+W2 early-start).

---

## §9 cross-links

- **Parent Spec**: [`hexa-weave-formal-mechanical-verification-prep.md`](./hexa-weave-formal-mechanical-verification-prep.md)
- **W1 audit**: [`hexa_weave_mvp_w1_lean4_audit_2026_04_28.md`](./hexa_weave_mvp_w1_lean4_audit_2026_04_28.md)
- **W2 report**: [`hexa_weave_mvp_w2_lean4_ax1_2026_04_28.md`](./hexa_weave_mvp_w2_lean4_ax1_2026_04_28.md)
- **W3 omega-cycle witness**: [`../design/kick/2026-04-28_lean4-w3-ax2_omega_cycle.json`](../design/kick/2026-04-28_lean4-w3-ax2_omega_cycle.json)
- **lean4 deliverable file**: [`../lean4-n6/N6/MechVerif/AX2.lean`](../lean4-n6/N6/MechVerif/AX2.lean)
- **lean4-n6 sister repo**: [`../lean4-n6/`](../lean4-n6/)
- **mathlib4 audited rev**: `19c497800a418208f973be74c9f5c5901aac2f54`
- **toolchain pin (current)**: `leanprover/lean4:v4.30.0-rc1`
- **F-CL-FORMAL-4 falsifier**: registry entry in L5 witness `raw_71_falsifiers_for_F-MX-6_construction[3]`
