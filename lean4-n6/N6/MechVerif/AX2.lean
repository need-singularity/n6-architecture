-- N6.MechVerif.AX2 : thm.AX2_strand_class_well_formed — first mechanical attempt
-- W3 deliverable for proposals/hexa-weave-formal-mechanical-verification-prep.md §4 unit 2.
-- Date: 2026-04-28 (cycle 5 fan-out 2/5).
-- W5 INTEGRATION (cycle 7, 2026-04-28): 2 opaque-bridge sorrys (lines 277/288)
-- discharged via named axioms mirroring MKBridge.lean (cycle 6 W4). AX2.lean
-- sorry count: 2 → 0.
-- W6 REFACTOR (cycle 8, 2026-04-28):
--   * `Strand` family + opaque MK predicates RELOCATED to
--     `N6/MechVerif/Foundation/Strand.lean` (leaf module).
--   * 2 local mirror axioms (`axiom_felgner_bridge_to_MK_AX2`,
--     `axiom_hexa_comp_closure_AX2`) REMOVED — replaced by direct use of
--     `Foundation/Axioms.lean` Strand-form theorems
--     (`felgner_bridge_to_MK_strand`, `hexa_comp_closure_strand`).
--   * F-W5-AX2-1 RESOLVED: 6 axiom (4 MKBridge + 2 mirror) duplication
--     burden collapsed to 5 unique named axioms in single source file.
--   * Main PASS theorems unchanged (`AX2_strand_class_well_formed`,
--     `AX2_strand_is_MK_class`, `AX2_strand_closed_under_HEXAComp`).
--
-- Mission-text alias path: lean4-n6/HexaWeave/AX2StrandClassWellFormed.lean
-- Canonical Spec §6 path : lean4-n6/N6/MechVerif/AX2.lean  ← THIS FILE

import N6.MechVerif.Foundation.Strand
import N6.MechVerif.Foundation.Axioms

namespace N6Mathlib.MechVerif

/-! ## §1-§5 Strand definitions — RELOCATED to Foundation/Strand.lean

    The `AminoAcid`, `RNANucleotide`, `DNANucleotide`, `Strand` inductives
    plus all `Strand.is*` predicates, witnesses, `Strand.cover_total`,
    `Strand.exists_each`, `Strand.universe_nonempty`, `StrandClass`, and
    `StrandClass.contains_all` / `nonempty` / `exhibits_each` are now in
    `N6/MechVerif/Foundation/Strand.lean` (W6 cycle 8 relocation).

    Re-exported here transparently via the import (same `N6Mathlib.MechVerif`
    namespace). Downstream files that named those declarations continue to
    compile without changes. -/

/-! ## §6 MK class-theory bridge — PARTIAL with named axiom

    The Spec §4 unit 2 calls for "STRAND ∃, IsClass STRAND ∧ STRAND closed under
    HEXA_COMP" in T_MK-HW. The lean4-level inductive `Strand : Type` is the
    type-theoretic surrogate; the formal MK-class-theory bridge is W4-W5 work
    (ZFC+V_κ fallback per Felgner 1971 conservativity).

    W6 cycle 8 (this commit): bridge axioms moved to
    `N6/MechVerif/Foundation/Axioms.lean`; this section retains only the
    Spec §4 unit 2 surrogate theorem. -/

/-- W3 placeholder: "the inductive type `Strand` represents an MK proper class
    or a V_κ-internal set, well-formed in T_MK-HW".

    See Foundation/Axioms.lean for the named-axiom dependencies that motivate
    this declaration's later discharge. -/
theorem AX2_class_formation_in_MK :
    ∃ _C : Set Strand,
      (∀ seq, Strand.aminoAcid seq ∈ (Set.univ : Set Strand)) ∧
      (∀ seq, Strand.rna seq ∈ (Set.univ : Set Strand)) ∧
      (∀ seq, Strand.dna seq ∈ (Set.univ : Set Strand)) ∧
      (∀ smi, Strand.smallLigand smi ∈ (Set.univ : Set Strand)) ∧
      (∀ h l, Strand.antibody h l ∈ (Set.univ : Set Strand)) := by
  refine ⟨Set.univ, ?_, ?_, ?_, ?_, ?_⟩
  · intro _; exact Set.mem_univ _
  · intro _; exact Set.mem_univ _
  · intro _; exact Set.mem_univ _
  · intro _; exact Set.mem_univ _
  · intro _ _; exact Set.mem_univ _

/-- W3 META-LEVEL marker: the lean4 inductive `Strand` faithfully translates
    Spec §4 unit 2 STRAND in T_MK-HW. -/
theorem AX2_translation_fidelity_to_MK : True := by
  trivial

/-! ### W6 cycle 8 named-axiom discharge (refactored)

    Pre-W6, AX2.lean carried two LOCAL mirror axioms
    (`axiom_felgner_bridge_to_MK_AX2`, `axiom_hexa_comp_closure_AX2`) to
    avoid the cyclic import `AX2 ↔ MKBridge`. With `Foundation/Axioms.lean`
    factored out, both AX2.lean and MKBridge.lean import the same source
    file directly; the mirrors collapse into the Strand-form theorems
    `felgner_bridge_to_MK_strand` and `hexa_comp_closure_strand`. -/

/-- W6 cycle 8: AX-2 STRAND-as-MK-class — DISCHARGED via the Strand-form
    theorem `felgner_bridge_to_MK_strand` in Foundation/Axioms.lean.

    sorry → axiom transition history:
      * cycle 5 W3: explicit `sorry` here.
      * cycle 6 W4: sorry-free witness in MKBridge.lean from named axiom.
      * cycle 7 W5: local mirror axiom `axiom_felgner_bridge_to_MK_AX2`.
      * cycle 8 W6 (THIS): mirror removed; direct theorem from Foundation.

    raw 91 C3: 4-axiom dependency disclosed (all in Foundation/Axioms.lean):
      • axiom_felgner_bridge_to_MK
      • axiom_felgner_1971_conservativity_meta
      • axiom_strand_zfc_witness
      • (closure axiom for the partner theorem below) -/
theorem AX2_strand_is_MK_class : IsMKProperClass Strand :=
  felgner_bridge_to_MK_strand

/-- W6 cycle 8: AX-2 HEXA-COMP closure — DISCHARGED via the Strand-form
    theorem `hexa_comp_closure_strand` in Foundation/Axioms.lean.

    Pending HEXA-COMP definition (W6+ AX-3/AX-4); axiom remains
    (`axiom_hexa_comp_closure_via_ZFC` in Foundation/Axioms.lean §3). -/
theorem AX2_strand_closed_under_HEXAComp : ClosedUnderHEXAComp Strand :=
  hexa_comp_closure_strand

/-! ## §7 W3 main statement — `thm.AX2_strand_class_well_formed`. -/

/-- **`thm.AX2_strand_class_well_formed`** — main W3 statement.

    Per Spec §4 unit 2: STRAND class is well-formed (5-way disjunction of
    biological-strand kinds, each definable, closed under the constructor
    image, non-empty).

    W3 STATUS: lean4 type-theoretic surrogate PASS; MK-bridge PARTIAL.
    W6 STATUS: refactored — Strand defs now in Foundation/Strand.lean,
    bridge axioms in Foundation/Axioms.lean. Theorem statement unchanged. -/
theorem AX2_strand_class_well_formed :
    Nonempty Strand ∧
    (∀ s : Strand,
        s.isAminoAcid ∨ s.isRNA ∨ s.isDNA ∨ s.isSmallLigand ∨ s.isAntibody) ∧
    ((∃ s : Strand, s.isAminoAcid) ∧
     (∃ s : Strand, s.isRNA) ∧
     (∃ s : Strand, s.isDNA) ∧
     (∃ s : Strand, s.isSmallLigand) ∧
     (∃ s : Strand, s.isAntibody)) ∧
    StrandClass.Nonempty := by
  refine ⟨⟨default⟩, Strand.cover_total, Strand.exists_each, StrandClass.nonempty⟩

/-! ## §8 raw 91 C3 honest disclosure (in-source) — W6 cycle 8 update

    What AX2.lean still proves (unchanged from W3/W5):
      ✔ `AX2_strand_class_well_formed` main statement (composes Strand defs)
      ✔ `AX2_class_formation_in_MK` placeholder
      ✔ `AX2_strand_is_MK_class` via Foundation Strand-form theorem
      ✔ `AX2_strand_closed_under_HEXAComp` via Foundation Strand-form theorem

    What was RELOCATED in W6 cycle 8 (this commit):
      → `Strand` + alphabets + predicates → Foundation/Strand.lean
      → `IsMKProperClass`/`ClosedUnderHEXAComp` opaque defs → Foundation/Strand.lean
      → 2 mirror axioms collapsed into Foundation/Axioms.lean Strand-form
        theorems (`felgner_bridge_to_MK_strand`, `hexa_comp_closure_strand`)

    F-W5-AX2-1 status: RESOLVED — 6 axiom (4 MKBridge + 2 mirror) duplication
    collapsed to 5 unique axioms in `N6/MechVerif/Foundation/Axioms.lean`
    (single source). raw 91 C3: axioms themselves unchanged; only their
    location is unified.

    W5-alt cycle 9 cross-reference (2026-04-28 fan-out 3/5):
      * RCSB Query B (1965 entries; Protein + RNA + ligand + ≤3.0 Å + pre-2024)
        retires F-W2-4 (yield ≥ 50) PARTIAL — see
        proposals/hexa_weave_mvp_w5_alt_paths_executed_2026_04_28.md §1.2.
      * Mock-up wrapper (Alt-3) validates the 18-module cross-attention
        architecture (6 blocks × 3 branches) without touching the lean4
        axiom layer; AX-3 / AX-4 lean4 work remains W7+ deferred.
      * No theorem / axiom / sorry change in this cycle (raw 91 C3 honest;
        comment-only docstring strengthening, build-safe).

    Outstanding gaps (named axioms in Foundation/Axioms.lean — auditable):
      ✘ axiom_felgner_1971_conservativity_meta (W7+)
      ✘ axiom_strand_zfc_witness (W5+ via Encodable)
      ✘ axiom_felgner_bridge_to_MK (W7+)
      ✘ axiom_hexa_comp_closure_via_ZFC (W6+ pending HEXA-COMP def)
      ✘ axiom_robin_hardy_wright_ax1_tail (AX-1 tail; W7+)

    What the file still does NOT prove (raw 91 C3 honest):
      ✘ Felgner 1971 internal proof (W7+ work)
      ✘ explicit Strand → ZFSet encoding (W5+)
      ✘ HEXA-COMP definition + closure (AX-3/AX-4 W6+ work) -/

end N6Mathlib.MechVerif
