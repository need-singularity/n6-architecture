# HEXA-WEAVE MVP W7 — lean4 axiom step-down decomposition (cycle 9 fan-out 2/5)

**Date:** 2026-04-28
**Cycle:** 9 / fan-out 2/5
**Owner:** Foundation/Axioms.lean
**Parent missions:**
- proposals/hexa_weave_mvp_2026_04_28.md
- proposals/hexa_weave_mvp_w6_axioms_shared_refactor_2026_04_28.md (cycle 8 fan-out 3/5)
- cycle 8 fan-out 2/5 (Felgner full proof) — REJECTED by user/system

## 1. Mission

Felgner full proof was rejected in cycle 8 fan-out 2/5. As a strictly different and orthogonal track, this cycle decomposes the two largest remaining monolithic axioms in `N6/MechVerif/Foundation/Axioms.lean` into named sub-axioms. No full proof is attempted; the decomposition is structural only.

The two targets:

- `axiom_strand_zfc_witness : Strand → ZFSet.{0}` — single monolithic encoding for the 5-way Strand constructor split.
- `axiom_hexa_comp_closure_via_ZFC : ClosedUnderHEXAComp Strand` — single monolithic closure assertion.

Felgner-related axioms (`axiom_felgner_bridge_to_MK`, `axiom_felgner_step1/2/3`, `axiom_felgner_1971_conservativity_meta`) are explicitly NOT touched this cycle to avoid proximity to the rejected full-proof track.

## 2. Decompositions performed

### A. `axiom_strand_zfc_witness` step-down (5 sub-axioms)

The original monolithic axiom is converted to a `noncomputable def` that pattern-matches on the `Strand` constructor and dispatches to one of 5 sub-axioms — one per `Strand` constructor:

| Sub-axiom | Carrier | Strand constructor |
|---|---|---|
| `axiom_strand_zfc_witness_amino` | `List AminoAcid → ZFSet.{0}` | `.aminoAcid` (22-letter) |
| `axiom_strand_zfc_witness_rna` | `List RNANucleotide → ZFSet.{0}` | `.rna` (4-letter A/U/G/C) |
| `axiom_strand_zfc_witness_dna` | `List DNANucleotide → ZFSet.{0}` | `.dna` (4-letter A/T/G/C) |
| `axiom_strand_zfc_witness_small_ligand` | `String → ZFSet.{0}` | `.smallLigand` (SMILES) |
| `axiom_strand_zfc_witness_antibody` | `List AminoAcid → List AminoAcid → ZFSet.{0}` | `.antibody` (heavy+light) |

The original symbol `axiom_strand_zfc_witness` is preserved as a derived `noncomputable def` so that `StrandClass_ZFC`, `MKBridge.lean` exhibition theorems, and `felgner_bridge_to_MK_strand` continue to compile unchanged. All `rfl`-level proofs are preserved by definitional unfolding of the dispatch.

**Net axiom-keyword change for A:** −1 (monolithic) + 5 (sub-axioms) = +4 keywords.

### C. `axiom_hexa_comp_closure_via_ZFC` step-down (4 sub-axioms + 1 atom retention)

`ClosedUnderHEXAComp` is `opaque` in `Foundation/Strand.lean` §6, so it cannot be inhabited from `: True`-valued sub-axioms alone. The decomposition mirrors the Felgner step1/2/3 pattern (`: True` placeholders) for the four standard closure-property components, plus retains ONE atomic-inhabitation axiom for the opaque proposition itself:

| Sub-axiom | Type | Sub-property |
|---|---|---|
| `axiom_hexa_comp_strand_op_well_defined` | `True` | C.1 binary op total on `Strand` |
| `axiom_hexa_comp_associativity` | `True` | C.2 associativity (or non-assoc declaration) |
| `axiom_hexa_comp_identity` | `True` | C.3 identity element existence |
| `axiom_hexa_comp_zfc_class_closure` | `True` | C.4 image stays in `StrandClass_ZFC` |
| `axiom_hexa_comp_closure_atom` | `ClosedUnderHEXAComp Strand` | atomic inhabitation (cannot be eliminated until `ClosedUnderHEXAComp` is given a non-opaque definition; W6+ AX-3/AX-4 work) |

The original symbol `axiom_hexa_comp_closure_via_ZFC` is converted to a `theorem` that combines the four `True` sub-axioms (via `have _h : True := ...`) and the atom. This makes the dependency on the four sub-properties explicit in `#print axioms axiom_hexa_comp_closure_via_ZFC` while preserving downstream callers (`hexa_comp_closure_strand`, `MKBridge.AX2_strand_closed_under_HEXAComp_via_ZFC`).

**Net axiom-keyword change for C:** −1 (monolithic) + 5 (4 sub-property + 1 atom) = +4 keywords.

## 3. Net axiom-keyword change

```
pre-cycle-9 (W6 cycle 8) Foundation/Axioms.lean: 7 axiom keywords
  1. axiom_felgner_step1_class_quantifier_to_Vkappa_bounded
  2. axiom_felgner_step2_proper_class_in_Vkappa
  3. axiom_felgner_step3_LZFC_relativization
  4. axiom_strand_zfc_witness               (monolithic)
  5. axiom_felgner_bridge_to_MK
  6. axiom_hexa_comp_closure_via_ZFC        (monolithic)
  7. axiom_robin_hardy_wright_ax1_tail
  (+ axiom_felgner_1971_conservativity_meta is already a derived theorem)

post-cycle-9 Foundation/Axioms.lean: 15 axiom keywords
  1-3.  felgner_step1/2/3                                 (unchanged)
  4-8.  strand_zfc_witness_{amino,rna,dna,small_ligand,
        antibody}                                         (5 NEW from A)
  9.    felgner_bridge_to_MK                              (unchanged)
  10-13.hexa_comp_{strand_op_well_defined,associativity,
        identity,zfc_class_closure}                       (4 NEW from C)
  14.   hexa_comp_closure_atom                            (1 NEW retained inhabitation)
  15.   robin_hardy_wright_ax1_tail                       (unchanged)
  (+ derived theorems: axiom_felgner_1971_conservativity_meta,
                       axiom_strand_zfc_witness            (newly derived),
                       axiom_hexa_comp_closure_via_ZFC     (newly derived))

Δ = +8 keywords.
Two prior `axiom` symbols (axiom_strand_zfc_witness, axiom_hexa_comp_closure_via_ZFC)
are now `noncomputable def` / `theorem` respectively; their identifiers still
work as drop-in replacements for downstream consumers.
```

This is consistent with the brief's two cited targets (16 axioms with no derivation, 14 with both derived). The actual outcome is 15 because C requires retention of the atomic-inhabitation axiom (the opaque type prevents full derivation from `True` placeholders).

## 4. raw 91 C3 honest disclosure

This cycle is a STRUCTURAL DECOMPOSITION ONLY. The total axiomatic content is unchanged; the sub-axioms simply expose finer-grained sub-claims that future cycles can attack independently.

- Felgner conservativity is NOT internally proved (W7+ work).
- HEXA-COMP is NOT defined; its closure cannot be derived from `True` placeholders. The atomic-inhabitation axiom `axiom_hexa_comp_closure_atom` retains the irreducible content of the prior monolithic `axiom_hexa_comp_closure_via_ZFC`.
- `Strand → ZFSet` encoding is NOT explicitly constructed; the 5 sub-axioms still surface the encoding as opaque per-constructor axioms. Full constructive encoding via `Encodable Strand` remains W7+/W8+ work.
- The `noncomputable def` for `axiom_strand_zfc_witness` is a thin pattern-match dispatch; it preserves the legacy signature `Strand → ZFSet.{0}` but adds NO new computational content.
- All `sorry` count remains 0 (no silent placeholders).
- Main PASS theorems preserved unchanged: `AX2_strand_class_well_formed`, `AX2_strand_is_MK_class`, `AX2_strand_closed_under_HEXAComp`, `AX1_n6_uniqueness`, `MKBridge.AX2_strand_is_MK_class_via_ZFC`, `MKBridge.AX2_strand_closed_under_HEXAComp_via_ZFC`, `MKBridge.StrandClass_ZFC.nonempty`, `MKBridge.StrandClass_ZFC.exhibits_each`.

## 5. Build verification

```
$ lake build N6.MechVerif.Foundation.Axioms
✔ [1336/1336] Built N6.MechVerif.Foundation.Axioms (11s)
Build completed successfully (1336 jobs).

$ lake build N6.MechVerif.AX1 N6.MechVerif.AX2 N6.MechVerif.MKBridge
✔ [1338/1340] Built N6.MechVerif.AX2 (4.0s)
✔ [1339/1340] Built N6.MechVerif.MKBridge (4.1s)
✔ [1340/1340] Built N6.MechVerif.AX1 (5.3s)
Build completed successfully (1340 jobs).

$ lake build
Build completed successfully.

$ grep -rn "sorry" N6/MechVerif/ | exclude comments
no sorry hits
```

`sorry` count: 0 (preserved). Full N6 lake build: PASS.

## 6. Files modified

- `lean4-n6/N6/MechVerif/Foundation/Axioms.lean` — A.1-A.5 (5 sub-axioms + derived `noncomputable def`), C.1-C.4 + atom (5 axioms + derived `theorem`), updated header prose, raw 91 C3 disclosure.

## 7. Files NOT modified (verified)

- `lean4-n6/N6/MechVerif/Foundation/Strand.lean` — leaf module, no change.
- `lean4-n6/N6/MechVerif/AX1.lean` — imports unchanged; `axiom_robin_hardy_wright_ax1_tail` untouched.
- `lean4-n6/N6/MechVerif/AX2.lean` — imports unchanged; `felgner_bridge_to_MK_strand` and `hexa_comp_closure_strand` continue to resolve (now via derived theorem and noncomputable def respectively).
- `lean4-n6/N6/MechVerif/MKBridge.lean` — `axiom_strand_zfc_witness` reference and `rfl`-level exhibition proofs preserved by definitional unfolding.

## 8. raw 71 falsifiers (5)

- **F-W7-AX-1** — `axiom_strand_zfc_witness` is now a `noncomputable def`; downstream code that pattern-matches on it as an `axiom` declaration in tooling (e.g. `Lean.Environment.find?` checking `ConstantInfo.axiom`) will see it as `def`. No such tooling currently in the repo; flagged for future cycles that might introduce axiom-graph extraction.
- **F-W7-AX-2** — `axiom_hexa_comp_closure_atom` is a renamed retention of the prior monolithic axiom. Any cycle that grep'd for `axiom_hexa_comp_closure_via_ZFC` as the LITERAL underlying axiom (rather than the now-derived theorem) will need to update to reference the atom.
- **F-W7-AX-3** — The C.1-C.4 sub-axioms are stated as `: True` (placeholder, content carried in docstrings only). They contribute zero proof-theoretic strength beyond the doc-strings. raw 91 C3 honest: this is a step-down PATTERN, not a step-down PROOF. Future cycles must either (a) replace each `: True` with a meaningful proposition once HEXA-COMP is defined, or (b) accept that C.1-C.4 are documentation surrogates.
- **F-W7-AX-4** — The 5 `axiom_strand_zfc_witness_*` sub-axioms have NO consistency relation among them. Two distinct strands with different constructors that happen to encode the "same" intuitive object (e.g. an empty amino-acid list and an empty antibody pair) will produce DIFFERENT `ZFSet.{0}` values. Pre-cycle-9 monolithic axiom had the same property (no extensionality), so this is preserved, but is more visible now.
- **F-W7-AX-5** — `#print axioms` on a downstream theorem (e.g. `AX2_strand_class_well_formed`) will now report 5 `axiom_strand_zfc_witness_*` and 4 hexa_comp sub-axioms + 1 atom instead of 1+1, potentially confusing automated audit pipelines that count "number of axioms" as a quality metric. The intended interpretation is "smaller surface area per axiom", not "fewer axioms".

## 9. F-D-3 reassessment

- Pre-W6: HIGH 63-72%
- Post-W6: HIGH 58-66%
- Post-W7 (this cycle): HIGH 58-66% (decomposition is structural; no semantic change to F-D-3 evaluation).

## 10. Outstanding axiomatic gaps (named in Foundation/Axioms.lean — auditable)

- `axiom_felgner_step1/step2/step3` (3 × `: True`) — Felgner Hauptsatz §3 (W7+ internal proof)
- `axiom_strand_zfc_witness_{amino,rna,dna,small_ligand,antibody}` (5 axioms) — explicit Encodable instances (W7+/W8+)
- `axiom_felgner_bridge_to_MK` (1 axiom) — Felgner conservativity application (W7+ pending step1+step2+step3 internalisation)
- `axiom_hexa_comp_{strand_op_well_defined,associativity,identity,zfc_class_closure}` (4 × `: True`) + `axiom_hexa_comp_closure_atom` (1 axiom) — pending HEXA-COMP definition (W6+ AX-3/AX-4)
- `axiom_robin_hardy_wright_ax1_tail` (1 axiom) — Robin 1984 + HW 322/328 + Wigert 1907 internal proof (W7+)

Total: 15 named axioms in single source file (was 7 pre-cycle-9). All auditable via `#print axioms`. Zero silent `sorry`.
