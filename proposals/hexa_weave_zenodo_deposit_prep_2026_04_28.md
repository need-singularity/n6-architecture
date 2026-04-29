<!--
DEPRECATED: 2026-04-28
deprecated_reason: papers CLI delegation (papers/.own own 9)
migration: paper publishing papers/bin/papers publish <id> --target=all use
historical: cycle 8 deposit prep task — archived as record
-->

# HEXA-WEAVE Formal-Mechanical W2/W3/W5 Paper — Zenodo Deposit (Option-A) Prep Checklist

> **Status**: PREP only. No deposit performed this cycle. Deposit gated on explicit user approval.
> **Author (proposer)**: cycle-8 fan-out 5/5 sub-agent (n6-architecture private framework).
> **Date**: 2026-04-28.
> **Parent paper**: `papers/hexa-weave-formal-mechanical-w2-2026-04-28.md` (cycle-7 sorry-free, cycle-8 refresh).
> **Witness JSON**: `design/kick/2026-04-28_zenodo-deposit-prep_omega_cycle.json`.
> **Disclosure tier targeted**: Option-A (Zenodo DOI).
> **Governance gates**: raw 71 paper-publication-tier (explicit user approval); raw 76 paper-DOI mandate (Zenodo automatic DOI satisfies); raw 91 C3 honest disclosure (mechanical vs empirical); own#11 (no Clay Millennium claim); own#15 (21-section completeness preserved by parent paper); PII-safe (no sensitive personal data committed without user approval).

## §1 WHY (why prep, not deposit, this cycle)

Cycle-7 closed the Lean 4 sorry-free milestone (AX-1 + AX-2 + MKBridge), making external exposure substantially safer than it was when the paper carried two open `sorry`s. Per the cycle-8 mission, this proposal compiles a Zenodo Option-A deposit checklist so the user can approve a deposit in a single later step. Anything that would burn an irreversible external surface (DOI minting, ORCID linkage, public author-name disclosure) is **NOT** done here.

## §2 8-item Zenodo deposit checklist

| # | Item | Status | Notes |
|---|---|---|---|
| 1 | Zenodo account (login via ORCID or institutional SSO) | **USER ACTION REQUIRED** | User must confirm an existing ORCID iD or create one; or use ResearchGate handoff. PII: ORCID is public-by-design, but linking it to this paper is the user's call. |
| 2 | DOI (Zenodo automatic) | AUTOMATIC on deposit | Satisfies raw 76 paper-DOI mandate the moment deposit is submitted. No pre-allocation required. |
| 3 | Paper title | **PROPOSED** | "HEXA-WEAVE Formal-Mechanical Verification: Sorry-Free Lean 4 Closure of AX-1 (n=6 Master Identity) and AX-2 (MK-Bridge) under Seven Named Axioms" — alternative shorter form: "HEXA-WEAVE: Multi-Strand Protein Weaving with n=6 Master Identity Trace" (mission-suggested; protein-folding framing). User to choose. |
| 4 | Authors | **USER ACTION REQUIRED** | Primary author: M. Park (independent; arsmoriendi99@proton.me — paper byline) or `ghost / mk55992@proton.me` (memory email). User must reconcile which email is the byline-of-record. Co-author: optional acknowledgement of "Claude (Anthropic)" as AI assistant — Zenodo allows non-human contributors but the user may prefer omission. |
| 5 | Keywords | **PROPOSED** | `protein-folding`, `multi-strand`, `n6-architecture`, `formal-verification`, `Lean4`, `Mathlib4`, `mechanical-theorem-proving`, `sigma-phi-tau`, `multiplicative-number-theory`, `Robin-1984`, `Hardy-Wright`, `Felgner-1971`, `MK-class-theory`, `AlphaFold`. User may trim to ~6-10 per Zenodo recommendation. |
| 6 | License | **PROPOSED**: Apache-2.0 | Consistent with W1 architecture decision (`proposals/hexa_weave_mvp_w1_architecture_decision_2026_04_28.md`). Apache-2.0 covers the paper text + Lean 4 source under a single permissive license. Alternative: CC-BY-4.0 for the paper text alone (Zenodo default for academic work). User to choose. |
| 7 | BibTeX cite-as | **DRAFTED** (see §3 below) | Final form depends on author byline + DOI (item 2 → automatic on deposit). |
| 8 | Supplementary materials | **DRAFTED** | Link to GitHub `lean4-n6/N6/MechVerif/{AX1,AX2,MKBridge}.lean` plus the embedded Python verify block. GitHub link requires the repo to be public (currently private per raw 71). Two paths: (a) deposit only the paper PDF + verify block, with private-repo-link placeholder pending user public-release approval; (b) tarball the three Lean files + a README.md and upload as Zenodo supplementary, keeping the GitHub repo private. Recommend (b) for this cycle. |

## §3 BibTeX cite-as (draft)

```bibtex
@misc{park2026hexaweave,
  title         = {HEXA-WEAVE Formal-Mechanical Verification: Sorry-Free Lean 4 Closure
                   of AX-1 (n=6 Master Identity) and AX-2 (MK-Bridge) under Seven
                   Named Axioms},
  author        = {Park, M.},
  year          = {2026},
  month         = {04},
  publisher     = {Zenodo},
  doi           = {10.5281/zenodo.XXXXXXX},
  url           = {https://doi.org/10.5281/zenodo.XXXXXXX},
  note          = {DOI placeholder; assigned automatically on deposit. Companion
                   Lean 4 source: \texttt{lean4-n6/N6/MechVerif/\{AX1,AX2,MKBridge\}.lean}.}
}
```

User to fill: author byline (M. Park vs ghost), DOI (auto on deposit).

## §4 Pre-flight safety gates

- **PII**: byline email + ORCID iD must be the user's deliberate choice. Default to `arsmoriendi99@proton.me` (paper byline) unless user redirects to `mk55992@proton.me`.
- **own#11 (no Clay Millennium)**: re-checked in parent paper §20.2; Robin 1984 cited without invoking the RH-equivalent sharp form.
- **raw 91 C3 (mechanical vs empirical honesty)**: parent paper §20.2 explicitly enumerates the seven named axioms and the zero-empirical-claim posture. No protein-folding empirical claims migrate into the deposit.
- **raw 76 (paper-DOI mandate)**: Zenodo automatic DOI satisfies this on deposit.
- **raw 71 (paper-publication-tier governance)**: Option-A requires explicit user approval; this proposal does NOT initiate deposit.
- **own#1 (English primary)**: parent paper is English; this proposal is bilingual where helpful but English-primary.
- **GitHub supplementary link**: only if the user opts to make the repo public. Default is private-repo + Zenodo-tarball supplementary (option (b) in item 8).

## §5 Five raw 71 falsifiers (Zenodo-prep tier)

- **F-W6-ZEN-1**: parent paper `lake build` regresses to non-sorry-free between this prep and any future deposit moment. Mitigation: re-run the §7.1(b) snippet immediately before deposit; abort if any `sorry` reappears.
- **F-W6-ZEN-2**: a counter-example `n ∈ [51, 1000]` is found by sympy. Falsifies the named-axiom basis. Mitigation: covered by parent paper §7.1 sweep; re-run before deposit.
- **F-W6-ZEN-3**: a reviewer demonstrates `axiom_robin_hardy_wright_ax1_tail` overstates Robin 1984. Triggers parent §19 MISS (f). Mitigation: the axiom statement uses Hardy-Wright unconditional asymptotic, not Robin's RH-conditional sharp bound; verify wording with one independent number-theory reader before deposit.
- **F-W6-ZEN-4**: Zenodo rejects deposit due to license mismatch (Apache-2.0 paper text vs Zenodo-preferred CC-BY-4.0). Mitigation: pre-flight check Zenodo policy; switch to CC-BY-4.0 for paper text and keep Apache-2.0 for the Lean 4 source if necessary.
- **F-W6-ZEN-5**: PII surface — paper byline email gets indexed by search engines and the user later regrets exposure. Mitigation: confirm with user *before* deposit which email to use (or use an institutional/throwaway address); ORCID iD remains optional.

## §6 Reproducibility recipe (will be embedded in the deposit description)

```bash
# Reproducibility recipe (Mac M2; Lean 4 v4.30.0-rc1; Mathlib4 rev 19c4978)
git clone <repo-url> n6-architecture
cd n6-architecture/lean4-n6
lake exe cache get
lake build N6.MechVerif.AX1
lake build N6.MechVerif.AX2
lake build N6.MechVerif.MKBridge
echo '#print axioms AX1_n6_uniqueness_corrected' | lake env lean --stdin
# Expected: 7 named axioms surfaced (Robin/Hardy-Wright + Felgner + 3x HEXA-COMP)
# Expected: 0 sorry tokens in proof terms
```

## §7 mk-history

- 2026-04-28T17:00:00Z — initial draft as cycle-8 fan-out 5/5 deliverable (Zenodo deposit Option-A prep). 8-item checklist + BibTeX draft + 5 falsifiers + reproducibility recipe. **No deposit performed; user approval required.**
