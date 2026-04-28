# HEXA-WEAVE Zenodo Deposit — Landing Page (cycle 14 auto-prep)

> **Status (2026-04-28):** AUTO-PREP only. NO Zenodo upload performed.
> NO DOI minted. NO ORCID linked. NO public-GitHub push. The five
> user-decision items in
> `proposals/hexa_weave_zenodo_auto_prep_2026_04_28.md` §4 must be
> resolved before any deposit is executed.

## What is this artifact?

This is the cycle-14 1-click deposit-prep bundle for the
HEXA-WEAVE Formal-Mechanical Verification paper (W2 milestone), authored
inside the n6-architecture private research framework.

The companion paper proves, in Lean 4, the n=6 master uniqueness
identity `sigma(n) * phi(n) = n * tau(n)  iff  n = 6`, sorry-free under
**19 named axioms** (cycle-12 atomic decomposition; cycles 11/12 W8+/W8++
mechanically convert 4 of those axioms into derived theorems via
mathlib4 ZFSet / Cardinal infrastructure).

## Deposit contents

| File | Role |
|---|---|
| `hexa-weave-formal-mechanical-w2-2026-04-28.pdf` | The paper (compiled from `papers/hexa-weave-formal-mechanical-w2-2026-04-28.md`). |
| `lean4-n6-mechverif-cycle12.tar.gz` | Lean 4 source tarball: `N6/MechVerif/{AX1,AX2,MKBridge}.lean` + `Foundation/{Strand,Axioms}.lean`. |
| `metadata.json` | Zenodo deposit metadata (this directory). |
| `manifest.sha256` | SHA-256 manifest of all uploaded artifacts. |

## How to reproduce the mechanical proof

```bash
# 1. clone repo at the recorded SHA (see manifest.sha256)
git clone <repo-url> n6-architecture
cd n6-architecture/lean4-n6

# 2. fetch the matching Lean / Mathlib pin
#    Lean 4: leanprover/lean4:v4.30.0-rc1 (lean-toolchain file)
#    Mathlib 4 rev 19c497800a418208f973be74c9f5c5901aac2f54 (lake-manifest.json)
lake exe cache get          # 8275 oleans, ~38.8 s decompress

# 3. build the three theorem targets
lake build N6.MechVerif.AX1
lake build N6.MechVerif.AX2
lake build N6.MechVerif.MKBridge

# 4. surface the named-axiom dependencies
echo '#print axioms AX1_n6_uniqueness_corrected' | lake env lean --stdin
echo '#print axioms AX2_strand_class_closed_under_hexa_comp' | lake env lean --stdin
```

Expected: build succeeds in < 60s on Mac M2 warm cache; **zero `sorry`
tokens** in proof terms; 19 named axioms surfaced (excluding Lean
kernel `propext` / `Classical.choice` / `Quot.sound`).

## How to reproduce the numerical sanity check

```bash
python3 -m venv .zenodo-venv
source .zenodo-venv/bin/activate
pip install -r tool/zenodo/requirements.txt
python3 tool/zenodo/verify_paper_block.py
```

Expected output:

```
AX-1 verify-embedded PASS:
  sigma(6)=12, phi(6)=2, tau(6)=4
  bounded [2,50]    solutions: [6]
  extended [2,1000] solutions: [6]
  spec corrigendum: ax1_eq(1) = True; n=1 forces n >= 2 quantifier amend
```

## Honest scope (raw 91 C3)

- **NO Riemann Hypothesis claim.** Robin 1984 is *cited* for its
  unconditional `sigma` asymptotic only (Hardy-Wright form), NOT for
  the RH-equivalent sharp bound.
- **NO Clay Millennium claim.** The paper addresses 0 of 7.
- **ZERO empirical claims.** This is a mechanical-verification paper;
  there is no lab data, no AlphaFold inference, no protein-folding
  empirical milestone. The HEXA-WEAVE multi-strand protein weaving
  framing is the *target domain* the formal layer supports;
  empirical milestones are gated separately on F-TP5-b 90-day MVP
  (2026-07-28).
- **19 named axioms.** All axioms are explicitly named (not silent
  `sorry`), surfaced via `#print axioms`, cited from published
  literature (Felgner 1971 + Drake 1974 + Jech 2003 + Williams 1976 +
  Robin 1984 + Hardy-Wright + Wigert 1907) and the n6-architecture
  private SSOT (HEXA-COMP). 3 of those 19 axioms (Felgner step1.b /
  step2.b / step2.d) are additionally backed by derived mechanical
  theorems (`vkappa_definable_to_set_mechanical` /
  `vkappa_powerset_closure_mechanical` / `vkappa_foundation_mechanical`)
  showing that the named-axiom layer is shrinking via mathlib4.

## Cite as

See the BibTeX entry in `proposals/hexa_weave_zenodo_auto_prep_2026_04_28.md`
§3. The DOI placeholder will be replaced with the assigned Zenodo DOI
post-publish.

## Author / contact

- **M. Park** — independent (n6-architecture private framework).
- Email: PENDING — to be reconciled with ORCID before deposit
  (proposal §4 item 2; two emails coexist in the repo,
  `arsmoriendi99@proton.me` and `mk55992@proton.me`; user picks one).
- ORCID: PENDING — proposal §4 item 1.
