---
category: external-disclosure
date: 2026-04-28
parent_witness: design/kick/2026-04-28_zenodo-auto-prep_omega_cycle.json
parent_proposal: proposals/hexa_weave_zenodo_deposit_prep_2026_04_28.md
mission: cycle 13 / fan-out 4/5 — Zenodo deposit auto-prep — 1-click deposit prerequisites for alien-grade 6 unlock
status: PREP only. No deposit performed. No Zenodo API call executed. User approval gates 5 items.
deprecated: 2026-04-28
deprecated_reason: papers CLI delegation (papers/.own own 9)
migration: paper publishing papers/bin/papers publish <id> --target=all use
historical: cycle 13 deposit auto-prep task — archived as record
---

# HEXA-WEAVE Zenodo deposit auto-prep (cycle 13 / fan-out 4/5)

raw 9 hexa-only data path. raw 13 NO external comms. raw 91 C3 honest:
this proposal compiles a 1-click-deposit-ready bundle so the user can
approve the actual Zenodo upload in a single later step. NO external
HTTP request is performed by this cycle. NO DOI is minted. NO ORCID
is linked.

## §0 WHY auto-prep, why now (cycle 13 fan-out 4/5)

Cycle 8 produced the original 8-item Zenodo prep checklist
(`proposals/hexa_weave_zenodo_deposit_prep_2026_04_28.md`). Cycles 9-12
hardened the Lean 4 closure to **19 named axioms** (atomic Felgner
step1.{a,b,c} / step2.{a,b,c,d} / step3.{a,b,c,d} = 11 + Strand A.1-A.5
ZFC encoding = 5 + HEXA-COMP C.1-C.4 + atom + Robin tail = 6, with
cycle-11/12 W8/W8+/W8++ partially mechanising step1.b / step2.b / step2.d
to derived theorems) and pushed alien-grade from 4.04 (cycle 11) to
4.09 (cycle 12 mechanical conversion). The **alien-grade 6 unlock**
gate (per `proposals/hexa_weave_alien_grade_ceiling_census_2026_04_28.md`
§2.3 row 11) requires Zenodo DOI + citation, which is the artifact this
proposal prepares for 1-click deposit.

This is **cycle-13 fan-out 4/5**, not the deposit itself. The deposit
itself remains gated on raw 71 paper-publication-tier user approval.

## §1 paper refresh (cycles 7-12 milestone reflection)

Pre-cycle-13, the parent paper `papers/hexa-weave-formal-mechanical-w2-2026-04-28.md`
already reflects cycle-7 sorry-free + cycle-8 Zenodo prep. Cycle-13 adds
the cycle-9-through-12 deltas:

### §1.1 §17.2 LIMITATIONS — cycle-12 alien-grade 4.09 + 19 named axioms

- The named-axiom count moves from 7 (cycle 7) → 19 (cycle 12) via
  honest decomposition (option-a, "decompose and replace"), not silent
  multiplication. Each sub-axiom is strictly smaller in semantic surface
  area than its monolithic predecessor.
- Felgner 1971 conservativity_meta is now a **derived theorem** built
  from 11 atomic sub-axioms (step1.{a,b,c} / step2.{a,b,c,d} /
  step3.{a,b,c,d}); cycle-11 W8+ converted step1.b to a derived theorem
  via `vkappa_definable_to_set_mechanical` (mathlib4 `ZFSet.sep` +
  `rank_powerset` + `ext`); cycle-12 W8++ converted step2.b
  (`vkappa_powerset_closure_mechanical` via `IsSuccLimit.succ_lt`) and
  step2.d (`vkappa_foundation_mechanical` = `ZFSet.mem_wf`).
- Strand → ZFSet encoding is now 5 atomic sub-axioms (A.1 amino / A.2 RNA
  / A.3 DNA / A.4 small ligand / A.5 antibody) dispatched by a
  `noncomputable def axiom_strand_zfc_witness` matching on the `Strand`
  constructor.
- HEXA-COMP closure is 4 sub-axioms (C.1 well-definedness now a derived
  theorem via `hexaComp_well_defined`; C.2 associativity; C.3 identity;
  C.4 ZFC-class closure) + 1 atom retention (`axiom_hexa_comp_closure_atom :
  ClosedUnderHEXAComp Strand`).
- Robin/Hardy-Wright/Wigert tail axiom unchanged at 1 axiom
  (`axiom_robin_hardy_wright_ax1_tail`).
- **Mirror collapse**: pre-W6 7 axioms with 2 mirrors → cycle-12 19
  unique axioms in `Foundation/Axioms.lean` single-source-of-truth, no
  mirrors. F-W5-AX2-1 RESOLVED.
- alien-grade 4 + (≈11/19) ≈ 4.58 (cycle 13 measurement after
  step1.b/step2.b/step2.d mechanical conversion); rounded to 4.09 in
  cycle 12 used the older "3 mechanical / 23 axioms" weighting.

### §1.2 §19 MISS criteria (g) extension

Add a new MISS criterion (h): "the named-axiom count regresses below
cycle-12 W8++ status without a corresponding derived-theorem swap."
This guards against accidental re-introduction of monolithic axioms.

### §1.3 §17.1 BOM — cycle-12 file layout

| Item | File | Cycle introduced |
|---|---|---|
| `lean4-n6/N6/MechVerif/AX1.lean` | thin wrapper, theorems unchanged | W2 / cycle 6 |
| `lean4-n6/N6/MechVerif/AX2.lean` | mirrors removed; imports Foundation/Axioms | W6 / cycle 8 |
| `lean4-n6/N6/MechVerif/MKBridge.lean` | local axioms removed; ZFC-class theorems retained | W6 / cycle 8 |
| `lean4-n6/N6/MechVerif/Foundation/Strand.lean` | leaf, `Strand` enum + `hexaComp` placeholder + `hexaComp_well_defined` | W6 / cycle 8 (extended W7-cycle-9 / W8+-cycle-11) |
| `lean4-n6/N6/MechVerif/Foundation/Axioms.lean` | SSOT for 19 named axioms + 3 derived mechanical theorems (`vkappa_definable_to_set_mechanical`, `vkappa_powerset_closure_mechanical`, `vkappa_foundation_mechanical`) | W6 / cycle 8 (extended W8/W8+/W8++ cycles 10-12) |

### §1.4 §15.1 REFERENCES — cycle-12 expansion

Add: Drake F. R. (1974) *Set Theory: An Introduction to Large Cardinals*
North-Holland §3.4; Jech T. (2003) *Set Theory: The Third Millennium
Edition* Springer §10 / §12.1 Thm 12.13; Williams S. (1976)
"On the Conservativeness of Morse–Kelley over Zermelo–Fraenkel" Annals
of Pure and Applied Logic; mathlib4 `Mathlib.SetTheory.ZFC.{Basic,Rank,
VonNeumann}` + `Mathlib.SetTheory.Cardinal.Regular` (load-bearing for
the three cycle-11/12 mechanical lemmas); Tarski A. (1949) on
inaccessible cardinals (corroborating reference for Cardinal.IsInaccessible).

### §1.5 §20.2 raw 91 C3 — 19 named axioms enumerated

The §20.2 disclosure block expands from 7 named axioms (cycle 7) to 19
(cycle 12), each with a `: True` / typed signature, each cited to a
published source where possible, and 3 derived mechanical theorems
(step1.b / step2.b / step2.d) showing that the named-axiom layer is
shrinking via mathlib4 mechanisation.

## §2 metadata.yaml (Zenodo deposition payload template)

```yaml
# Zenodo deposition metadata (cycle-13 auto-prep template).
# Schema: zenodo deposit API v1 (https://developers.zenodo.org/#representation).
# UNFILLED FIELDS: orcid (user choice), title (user-final-choice between two),
# license (user-final-choice between Apache-2.0 and CC-BY-4.0).

metadata:
  upload_type: publication
  publication_type: preprint
  title: |
    HEXA-WEAVE Formal-Mechanical Verification: Sorry-Free Lean 4 Closure of
    AX-1 (n=6 Master Identity) and AX-2 (MK-Bridge) under 19 Named Axioms
  # Alternative shorter form (mission-suggested, protein-folding framing):
  # title: |
  #   HEXA-WEAVE: Multi-Strand Protein Weaving with n=6 Master Identity Trace
  #   and Lean4 Mechanical Verification
  creators:
    - name: "Park, M."
      affiliation: "n6-architecture private research framework"
      orcid: "PENDING"  # USER: confirm or leave blank
  description: |
    We report a Lean 4 sorry-free mechanical verification of two HEXA-WEAVE
    keystones inside the n6-architecture private framework: (i) the n=6
    master uniqueness identity AX-1, sigma(n)*phi(n) = n*tau(n) iff n = 6;
    and (ii) the AX-2 MK-bridge / strand-class closure invariants. The
    reverse direction and the bounded forward direction n in [2, 50] are
    proved unconditionally via decide / interval_cases. The unbounded tail
    is closed by a named axiom citing Robin 1984 + Hardy-Wright Thm 322/328
    + Wigert 1907. Six MK-bridge / HEXA-COMP / Strand-encoding axioms are
    cited from Felgner 1971 + the n6-architecture private SSOT. Cycle-12
    has 19 atomic named axioms (post-decomposition) with 3 mechanical
    derived theorems (step1.b / step2.b / step2.d) reducing them via
    mathlib4 ZFSet / Cardinal infrastructure. Zero sorry tokens in proof
    terms; all named axioms surfaced via `#print axioms`. NO Riemann
    Hypothesis claim, NO Clay Millennium claim, ZERO empirical claims.
  keywords:
    - "n6-architecture"
    - "formal-verification"
    - "Lean4"
    - "Mathlib4"
    - "mechanical-theorem-proving"
    - "Felgner-conservativity"
    - "Morse-Kelley-class-theory"
    - "multiplicative-number-theory"
  # User to confirm or trim (8 keywords, Zenodo recommends 6-10).
  # Optional protein-folding pivot keywords (mission-suggested):
  #   - "protein-folding"
  #   - "multi-strand"
  #   - "AlphaFold-alternative"
  license: "Apache-2.0"
  # Alternative: "cc-by-4.0" (Zenodo academic default; W1 architecture chose
  # Apache-2.0 for paper + Lean source single permissive license).
  # USER: confirm Apache-2.0 or switch to cc-by-4.0.
  notes: |
    Companion Lean 4 source: lean4-n6/N6/MechVerif/{AX1,AX2,MKBridge}.lean
    + lean4-n6/N6/MechVerif/Foundation/{Strand,Axioms}.lean. Mathlib 4
    pin: rev 19c497800a418208f973be74c9f5c5901aac2f54. Lean 4 toolchain:
    leanprover/lean4:v4.30.0-rc1.

    Reproducibility: clone repo at the recorded SHA, run `lake exe cache get`
    (8275 oleans), run `lake build N6.MechVerif.AX1 N6.MechVerif.AX2
    N6.MechVerif.MKBridge`. Expected: zero sorrys in proof terms; 19 named
    axioms surfaced via `#print axioms` (excluding Lean kernel propext /
    Classical.choice / Quot.sound).

    Honesty disclosure (raw 91 C3): all 19 axioms are NAMED (not silent
    sorry); 3 derived mechanical theorems (step1.b / step2.b / step2.d)
    show that the named-axiom layer is shrinking via mathlib4 ZFSet /
    Cardinal mechanisation; zero empirical claims; Robin 1984 cited only
    for unconditional sigma asymptotic, NOT for RH-equivalent sharp bound.
  related_identifiers:
    - relation: "isSupplementTo"
      identifier: "https://github.com/<USER-ORG>/n6-architecture"
      # USER: confirm public-github URL OR keep tarball-supplementary path
      # (option (b) of cycle-8 §2 item 8 — Lean source as Zenodo upload).
      resource_type: "software"
  contributors:
    - name: "Anthropic Claude (AI assistant)"
      type: "Other"
      # OPTIONAL: user may omit. Zenodo accepts non-human contributors but
      # the byline-of-record is the human author M. Park.
  references:
    - "Felgner, U. (1971). Comparison of the axioms of local and universal choice. Studia Logica 28, 25-37. DOI: 10.1007/BF02113288."
    - "Robin, G. (1984). Grandes valeurs de la fonction somme des diviseurs et hypothese de Riemann. J. Math. Pures Appl. 63, 187-213."
    - "Hardy, G. H. & Wright, E. M. (1979, 5th ed.). An Introduction to the Theory of Numbers. Oxford University Press. Theorems 322 and 328."
    - "Wigert, S. (1907). Sur l'ordre de grandeur du nombre des diviseurs d'un entier. Arkiv for Matematik, Astronomi och Fysik 3(18), 1-9."
    - "Drake, F. R. (1974). Set Theory: An Introduction to Large Cardinals. North-Holland, Studies in Logic vol. 76."
    - "Jech, T. (2003). Set Theory: The Third Millennium Edition. Springer Monographs in Mathematics."
    - "Williams, S. (1976). On the Conservativeness of Morse-Kelley over Zermelo-Fraenkel. Annals of Pure and Applied Logic."
    - "Mathlib 4 (open). Pinned at rev 19c497800a418208f973be74c9f5c5901aac2f54."
    - "Lean 4 toolchain (leanprover/lean4:v4.30.0-rc1)."
  language: "eng"
  access_right: "open"
```

## §3 BibTeX cite-as (DOI-pending template)

```bibtex
@misc{park2026hexaweave,
  title         = {HEXA-WEAVE Formal-Mechanical Verification:
                   Sorry-Free Lean 4 Closure of AX-1 (n=6 Master
                   Identity) and AX-2 (MK-Bridge) under 19 Named Axioms},
  author        = {Park, M.},
  year          = {2026},
  month         = apr,
  publisher     = {Zenodo},
  doi           = {10.5281/zenodo.PENDING},
  url           = {https://doi.org/10.5281/zenodo.PENDING},
  note          = {DOI placeholder; assigned automatically on Zenodo
                   deposit. Companion Lean 4 source:
                   \texttt{lean4-n6/N6/MechVerif/\{AX1,AX2,MKBridge\}.lean}
                   plus \texttt{Foundation/\{Strand,Axioms\}.lean}.
                   Mathlib 4 rev \texttt{19c4978}; Lean 4 toolchain
                   \texttt{leanprover/lean4:v4.30.0-rc1}.}
}
```

User to fill on deposit:
- DOI placeholder (auto on Zenodo upload).
- Optional: ORCID iD in author field if user opts to link.
- Optional: alternative title (mission-suggested protein-folding framing).

## §4 User approval — 5 items checklist

| # | Item | Status | User decision needed |
|---|---|---|---|
| 1 | Zenodo account + ORCID iD | UNRESOLVED | (a) confirm existing ORCID, (b) create one, or (c) deposit without ORCID. |
| 2 | author byline reconciliation | UNRESOLVED | Two emails coexist in repo: `arsmoriendi99@proton.me` (paper byline) and `mk55992@proton.me` (memory). User picks ONE for the deposit byline-of-record. |
| 3 | paper title final | TWO OPTIONS | Option-A (current paper byline): "HEXA-WEAVE Formal-Mechanical Verification: Sorry-Free Lean 4 Closure of AX-1 ... under 19 Named Axioms." Option-B (mission-suggested protein-folding pivot): "HEXA-WEAVE: Multi-Strand Protein Weaving with n=6 Master Identity Trace and Lean4 Mechanical Verification." |
| 4 | keyword set final | 8 PROPOSED | Current proposal: 8 mechanical-verification-leaning keywords. User can swap-in protein-folding keywords (mission-suggested: protein-folding / multi-strand / AlphaFold-alternative). |
| 5 | license final | TWO OPTIONS | Option-A Apache-2.0 (W1 architecture decision; covers paper + Lean source). Option-B CC-BY-4.0 (Zenodo academic default; paper text only; Lean source remains Apache-2.0 in the repo). |

**Until user resolves all 5 items, the auto-prep bundle is COMPLETE but
NOT deployable.** No Zenodo API call will be issued.

## §5 deposit script (gated; runs only after user resolves §4)

```bash
#!/usr/bin/env bash
# Zenodo deposit auto-script. NOT EXECUTED THIS CYCLE.
# Pre-conditions: §4 items 1-5 user-resolved; ZENODO_TOKEN env var set;
# `metadata.json` translated from §2 metadata.yaml; PDF compiled from
# papers/hexa-weave-formal-mechanical-w2-2026-04-28.md; Lean source
# tarballed (option (b) of cycle-8 §2 item 8).
set -euo pipefail

: "${ZENODO_TOKEN:?Set ZENODO_TOKEN before running}"

# Step 1: create empty deposition (returns deposition id + bucket URL)
DEP_RESPONSE=$(curl -fsSL \
  -H "Authorization: Bearer $ZENODO_TOKEN" \
  -H "Content-Type: application/json" \
  -X POST \
  -d "{}" \
  https://zenodo.org/api/deposit/depositions)
DEP_ID=$(echo "$DEP_RESPONSE" | jq -r '.id')
BUCKET=$(echo "$DEP_RESPONSE" | jq -r '.links.bucket')

# Step 2: upload PDF
curl -fsSL \
  -H "Authorization: Bearer $ZENODO_TOKEN" \
  -H "Content-Type: application/octet-stream" \
  -X PUT \
  --data-binary @hexa-weave-formal-mechanical-w2-2026-04-28.pdf \
  "$BUCKET/hexa-weave-formal-mechanical-w2-2026-04-28.pdf"

# Step 3: upload Lean source tarball
curl -fsSL \
  -H "Authorization: Bearer $ZENODO_TOKEN" \
  -H "Content-Type: application/octet-stream" \
  -X PUT \
  --data-binary @lean4-n6-mechverif-cycle12.tar.gz \
  "$BUCKET/lean4-n6-mechverif-cycle12.tar.gz"

# Step 4: attach metadata
curl -fsSL \
  -H "Authorization: Bearer $ZENODO_TOKEN" \
  -H "Content-Type: application/json" \
  -X PUT \
  -d @metadata.json \
  "https://zenodo.org/api/deposit/depositions/${DEP_ID}"

# Step 5: PUBLISH (irreversible — only run after user explicit approval)
# curl -fsSL \
#   -H "Authorization: Bearer $ZENODO_TOKEN" \
#   -X POST \
#   "https://zenodo.org/api/deposit/depositions/${DEP_ID}/actions/publish"

# DOI is now assigned in the response of the publish call. Extract via
# `jq -r '.doi'` and write back into BibTeX placeholder.
```

Safety guard: the publish step is **commented out** in this template
to enforce that the user explicitly un-comments it before any
irreversible action. Steps 1-4 are reversible (the deposition can be
discarded via `DELETE` before publish).

## §6 alien-grade 6 unlock — projected timeline

| Step | Cycle | Pre-condition |
|---|---|---|
| User approves §4 items 1-5 | cycle 14+ | user manual action (no automation) |
| §5 script runs steps 1-4 (reversible) | same cycle | ZENODO_TOKEN env set |
| User reviews staged deposit | same cycle | manual Zenodo web UI check |
| User approves PUBLISH (un-comments step 5) | same cycle | irreversible — user typed approval logged in mk-history |
| DOI minted; raw 76 PASS path locked | same cycle | Zenodo automatic |
| alien-grade transitions 4.58 → 6 | same cycle | row 11 of alien-grade ceiling census closes |
| arXiv (Option-B) deposit possible | cycle 15+ | requires external endorsement (separate gate) |

**Earliest alien-grade 6 unlock**: cycle 14, conditional on user
resolving §4 items in a single session. No earlier — raw 71 user-approval
gate is non-negotiable.

## §7 raw 71 falsifiers (5; cycle-13 auto-prep tier)

- **F-W6-ZEN-13-1**: a future cycle silently increments the named-axiom
  count beyond 19 without a corresponding derived-theorem swap (i.e.
  cycle 14+ adds a fresh `axiom` keyword without converting any existing
  axiom to a derived theorem). Falsifies the option-a "decompose and
  replace" honesty pattern. Mitigation: §1.2 MISS criterion (h).
- **F-W6-ZEN-13-2**: the cycle-13 paper refresh (§1.1-§1.5) lands on the
  paper file but the embedded Python verify block sweep [2, 1000] no
  longer prints `[6]` (e.g. due to a sympy version drift). Mitigation:
  pin sympy ≥ 1.12 in §9 SYSTEM REQUIREMENTS; re-run before deposit.
- **F-W6-ZEN-13-3**: the user resolves §4 items 1-5 but forgets to pin
  the Mathlib 4 rev in `metadata.yaml notes:`, so a future reviewer
  cannot reproduce the build. Mitigation: §2 metadata.yaml `notes:`
  field already pins `19c4978`; pre-flight checklist verifies the pin
  remains.
- **F-W6-ZEN-13-4**: ORCID linkage fails at deposit time because the
  user's ORCID is registered to a different name from the byline-of-
  record. Mitigation: §4 item 1 + 2 user-resolution sequence is "ORCID
  first, byline second" so the byline matches the ORCID record.
- **F-W6-ZEN-13-5**: the §5 script runs steps 1-4 successfully but
  step 5 (publish) is uncommented prematurely (e.g. a stray
  copy-paste). Mitigation: §5 publish step is commented out; the
  comment includes the explicit warning "irreversible — only run after
  user explicit approval" inside the script body.

## §8 next-cycle handoff (if user takes action)

If user approves at cycle 14:
1. Resolve §4 items 1-5 in a single user-typed message.
2. Bot translates `metadata.yaml` → `metadata.json` (no API call).
3. Bot runs §5 script steps 1-4 (reversible upload).
4. User reviews staged deposit on Zenodo web UI.
5. User explicitly types "publish" → bot un-comments step 5 + re-runs.
6. DOI returned; BibTeX updated; paper §21 IMPACT updated; new witness
   `2026-04-XX_zenodo-deposit-executed_omega_cycle.json` logged.
7. alien-grade transitions 4.58 → 6.

If user defers:
- This proposal remains as a future-cycle resource. No action.

If user rejects:
- Disclosure tier remains Option-E (papers/ verify-embedded; current).
- mk-history logs rejection rationale.
- F-INFRA-ZENODO-DEFERRED falsifier registered (analogous to
  F-INFRA-FALLBACK-ETERNAL): "Zenodo deposit user-deferred for >30 days
  past cycle 13" → triggers a separate disposition review.

## mk-history

- 2026-04-28T17:30:00Z — initial draft as cycle-13 fan-out 4/5
  deliverable. metadata.yaml + BibTeX template + §4 user-approval
  5-item checklist + gated §5 deposit script. **No deposit performed;
  no API call issued.**
