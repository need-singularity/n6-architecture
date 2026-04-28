-- N6.MechVerif.Foundation.Strand : shared Strand inductive + opaque MK predicates.
-- W6 deliverable for proposals/hexa-weave-formal-mechanical-verification-prep.md (cycle 8 fan-out 3/5).
-- Date: 2026-04-28 (W6 axiom shared-file refactor).
--
-- ## Mission (W6 cycle 8 fan-out 3/5)
-- Resolve 6-axiom (4 MKBridge + 2 AX2 mirror) duplication burden surfaced as
-- F-W5-AX2-1. AX2.lean and MKBridge.lean both need `Strand` + opaque MK
-- predicates, and MKBridge.lean must import them; the cyclic-import
-- avoidance forced AX2.lean to mirror MKBridge axioms locally.
--
-- Solution: factor `Strand` family + opaque predicates into THIS leaf
-- module. Both AX2.lean and MKBridge.lean now import this file (and
-- Foundation/Axioms.lean below); AX2.lean no longer needs to mirror.
--
-- This file imports ONLY mathlib4 leaves; it depends on no other
-- N6.MechVerif file. That keeps it import-graph leaf and breaks the
-- previous cycle.

import Mathlib.Data.Set.Basic

namespace N6Mathlib.MechVerif

/-! ## §1 Atomic monomers (alphabets)

    Identical to the previous AX2.lean §1 definitions. Moved here to make
    `Strand` available to both AX2.lean and MKBridge.lean without cyclic
    imports. -/

/-- Standard 20 proteinogenic amino acids + selenocysteine + pyrrolysine = 22. -/
inductive AminoAcid where
  | ala | arg | asn | asp | cys | gln | glu | gly | his | ile
  | leu | lys | met | phe | pro | ser | thr | trp | tyr | val
  | sec | pyl
  deriving DecidableEq, Repr

/-- 4 RNA nucleotides {A, U, G, C}. -/
inductive RNANucleotide where
  | a | u | g | c
  deriving DecidableEq, Repr

/-- 4 DNA nucleotides {A, T, G, C}. -/
inductive DNANucleotide where
  | a | t | g | c
  deriving DecidableEq, Repr

/-! ## §2 STRAND inductive type — Spec §4 unit 2 5-way disjunction. -/

/-- The STRAND inductive type. Each constructor corresponds to one disjunct
    of the Spec §4 unit 2 STRAND comprehension. -/
inductive Strand where
  /-- Amino acid sequence (peptide or protein primary structure). -/
  | aminoAcid (seq : List AminoAcid) : Strand
  /-- RNA sequence (single-strand). -/
  | rna (seq : List RNANucleotide) : Strand
  /-- DNA sequence (single-strand). -/
  | dna (seq : List DNANucleotide) : Strand
  /-- Small ligand encoded as SMILES string. -/
  | smallLigand (smiles : String) : Strand
  /-- Antibody (heavy + light chain). -/
  | antibody (heavy : List AminoAcid) (light : List AminoAcid) : Strand
  deriving Repr

/-! ## §3 Predicates for the 5 disjuncts. -/

/-- Predicate: this strand is an amino-acid sequence. -/
def Strand.isAminoAcid : Strand → Prop
  | .aminoAcid _ => True
  | _ => False

/-- Predicate: this strand is an RNA sequence. -/
def Strand.isRNA : Strand → Prop
  | .rna _ => True
  | _ => False

/-- Predicate: this strand is a DNA sequence. -/
def Strand.isDNA : Strand → Prop
  | .dna _ => True
  | _ => False

/-- Predicate: this strand is a small ligand. -/
def Strand.isSmallLigand : Strand → Prop
  | .smallLigand _ => True
  | _ => False

/-- Predicate: this strand is an antibody. -/
def Strand.isAntibody : Strand → Prop
  | .antibody _ _ => True
  | _ => False

/-- The 5-way disjunction is total (every Strand satisfies at least one). -/
theorem Strand.cover_total (s : Strand) :
    s.isAminoAcid ∨ s.isRNA ∨ s.isDNA ∨ s.isSmallLigand ∨ s.isAntibody := by
  cases s
  case aminoAcid   => left;             trivial
  case rna         => right; left;      trivial
  case dna         => right; right; left; trivial
  case smallLigand => right; right; right; left; trivial
  case antibody    => right; right; right; right; trivial

/-! ## §4 Trivial existence witnesses. -/

/-- Empty peptide — degenerate but valid amino-acid-sequence witness. -/
def Strand.witnessAminoAcid : Strand := .aminoAcid []

/-- Empty RNA — degenerate but valid RNA witness. -/
def Strand.witnessRNA : Strand := .rna []

/-- Empty DNA — degenerate but valid DNA witness. -/
def Strand.witnessDNA : Strand := .dna []

/-- Empty SMILES — placeholder small-ligand witness. -/
def Strand.witnessSmallLigand : Strand := .smallLigand ""

/-- Empty-chain antibody — degenerate but valid antibody witness. -/
def Strand.witnessAntibody : Strand := .antibody [] []

/-- `Strand` is inhabited (lean4-level surrogate for "STRAND ≠ ∅"). -/
instance : Inhabited Strand := ⟨Strand.witnessAminoAcid⟩

/-- Trivial existence: there exists a strand of each kind. -/
theorem Strand.exists_each :
    (∃ s : Strand, s.isAminoAcid)   ∧
    (∃ s : Strand, s.isRNA)         ∧
    (∃ s : Strand, s.isDNA)         ∧
    (∃ s : Strand, s.isSmallLigand) ∧
    (∃ s : Strand, s.isAntibody) := by
  refine ⟨?_, ?_, ?_, ?_, ?_⟩
  · exact ⟨Strand.witnessAminoAcid, by trivial⟩
  · exact ⟨Strand.witnessRNA, by trivial⟩
  · exact ⟨Strand.witnessDNA, by trivial⟩
  · exact ⟨Strand.witnessSmallLigand, by trivial⟩
  · exact ⟨Strand.witnessAntibody, by trivial⟩

/-- `Set Strand`-level non-emptiness. -/
theorem Strand.universe_nonempty : (Set.univ : Set Strand).Nonempty :=
  ⟨default, Set.mem_univ _⟩

/-! ## §5 Type-level "class-formation" surrogate. -/

/-- The `Set Strand`-level "class" of all strands. -/
def StrandClass : Set Strand := Set.univ

/-- `StrandClass` contains every strand. -/
theorem StrandClass.contains_all (s : Strand) : s ∈ StrandClass :=
  Set.mem_univ s

/-- `StrandClass` is non-empty as a `Set Strand`. -/
theorem StrandClass.nonempty : StrandClass.Nonempty :=
  Strand.universe_nonempty

/-- `StrandClass` exhibits each constructor kind. -/
theorem StrandClass.exhibits_each :
    (∃ s ∈ StrandClass, s.isAminoAcid)   ∧
    (∃ s ∈ StrandClass, s.isRNA)         ∧
    (∃ s ∈ StrandClass, s.isDNA)         ∧
    (∃ s ∈ StrandClass, s.isSmallLigand) ∧
    (∃ s ∈ StrandClass, s.isAntibody) := by
  refine ⟨?_, ?_, ?_, ?_, ?_⟩
  · exact ⟨Strand.witnessAminoAcid, Set.mem_univ _, by trivial⟩
  · exact ⟨Strand.witnessRNA, Set.mem_univ _, by trivial⟩
  · exact ⟨Strand.witnessDNA, Set.mem_univ _, by trivial⟩
  · exact ⟨Strand.witnessSmallLigand, Set.mem_univ _, by trivial⟩
  · exact ⟨Strand.witnessAntibody, Set.mem_univ _, by trivial⟩

/-! ## §6 Opaque MK class-theory predicates

    These remain opaque (cannot be inhabited without an MK formalization in
    mathlib4). MKBridge.lean and Foundation/Axioms.lean both use them. -/

/-- Opaque axiom-shaped predicate: "X is an MK proper class". -/
opaque IsMKProperClass (α : Type) : Prop

/-- Opaque axiom-shaped predicate: "the class is closed under HEXA-COMP". -/
opaque ClosedUnderHEXAComp (α : Type) : Prop

/-! ## §7 HEXA-COMP binary operation — cycle 11 W8+ definition surface

    Cycle 11 W8+ (this commit): introduces a placeholder `hexaComp` total
    function on `Strand` so that the C.1 well-definedness sub-axiom
    (`axiom_hexa_comp_strand_op_well_defined : True` in Foundation/Axioms.lean)
    can be CONVERTED to a derived `theorem` (no longer a `: True` axiom).

    raw 91 C3 honest disclosure:
      • The dispatch below is **placeholder semantics** — every case returns
        the first input strand unchanged. Real biological / structural
        meanings (protein–RNA complex, antigen–antibody binding pose,
        enzyme–substrate composite, etc.) are NOT captured. A faithful
        encoding requires a richer carrier (a `StrandComplex` inductive
        with binding-pose payload, planned for W9+).
      • What IS captured here, and ONLY here: that the operation has a
        total function signature `Strand → Strand → Strand`. C.1
        well-definedness reduces to that signature being inhabited as a
        Lean term — which it now is, hence C.1 becomes derivable.
      • Associativity (C.2), identity (C.3), and ZFC-class closure (C.4)
        are NOT discharged by this placeholder dispatch:
          – C.2 is biologically false in general (binding pose depends on
            ordering of association events), so it remains an axiom and is
            EXPECTED to remain a non-trivially-stated axiom even after
            W9+ enrichment.
          – C.3 has no obvious biological identity element (the empty
            peptide is not a unit for protein–RNA association, etc.), so
            it remains an axiom pending a richer model.
          – C.4 depends on the ZFC encoding (`StrandClass_ZFC`), which is
            already a chained axiom system; it remains an axiom pending
            joint W9+ work.
      • The placeholder dispatch is total but biologically uninformative.
        It MUST NOT be used as a load-bearing semantic primitive in
        downstream theorems. -/

/-- HEXA-COMP binary operation on `Strand` (cycle 11 W8+ placeholder).
    Total function — every input pair maps to a `Strand` output. The
    semantic content is a placeholder dispatch (returns the first input
    strand unchanged in every case); see §7 docstring for the raw 91 C3
    honest disclosure of what this does and does NOT capture. The single
    purpose of this definition is to make C.1 (well-definedness, i.e.
    inhabitation of the total-function signature `Strand → Strand → Strand`)
    derivable as a `theorem` rather than postulated as an axiom. -/
def hexaComp : Strand → Strand → Strand
  | s₁, _ => s₁

/-- HEXA-COMP well-definedness, derived from the existence of the
    `hexaComp` term above. The unique-existence statement is the standard
    formal expression of "the binary operation is well-defined as a
    total function": for every input pair `(s₁, s₂)`, there exists a
    unique `s₃ : Strand` equal to `hexaComp s₁ s₂` (uniqueness here is
    trivial because `hexaComp` is a function, not a relation). -/
theorem hexaComp_well_defined :
    ∀ (s₁ s₂ : Strand), ∃! (s₃ : Strand), s₃ = hexaComp s₁ s₂ := by
  intro s₁ s₂
  refine ⟨hexaComp s₁ s₂, rfl, ?_⟩
  intro y hy
  exact hy

end N6Mathlib.MechVerif
