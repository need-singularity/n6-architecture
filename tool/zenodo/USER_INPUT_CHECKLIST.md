# HEXA-WEAVE Zenodo Deposit — User Input Checklist (cycle 14; cycle-18 status update)

> **Status (2026-04-28, cycle-18 update):** automation complete; 7
> user-input items remain before `bash tool/zenodo/deposit.sh --upload`
> can run. cycle-18 paper precision (§19 ACCEPTANCE / §20 APPENDIX /
> §21 IMPACT updated; axiom count 17 → 11 measured; 11/11 Felgner
> atomics mathlib4-backed at semantic-kernel level; F-CL-FORMAL-4
> PARTIAL-RESOLVED) and Zenodo manifest re-spin complete; 7 user-input
> items below remain PENDING (status carried forward from cycle-14
> auto-prep with cycle-18 progress notes).

When you're ready to deposit, paste the answers below into a single
session reply, then the bot will:
1. edit `tool/zenodo/metadata.json` to replace the PENDING placeholders;
2. compile the paper to PDF (pandoc or TeX);
3. re-run `gen_manifest.sh` to refresh the SHA-256 manifest;
4. run `bash tool/zenodo/deposit.sh --upload` (steps 1-4, reversible);
5. show you the staged-deposit URL on the Zenodo web UI;
6. wait for your explicit "publish" approval before un-commenting
   step 5 of `deposit.sh` and re-running with the irreversibility flag.

---

## Item 1 — ORCID iD (cycle-14 mission noted user OK; please confirm one of)

- [ ] **(a) confirm an existing ORCID iD** — paste the 16-digit iD here:
      `____-____-____-____`
- [ ] **(b) create a fresh ORCID iD now** — sign up at
      https://orcid.org/register and paste the new iD here
- [ ] **(c) deposit without ORCID** — author byline only

## Item 2 — author byline email reconciliation

Two emails coexist in the repo:

- [ ] `arsmoriendi99@proton.me` (current paper byline; `papers/...md` line 4)
- [ ] `mk55992@proton.me` (memory; alternative byline)
- [ ] `multi404error@proton.me` (current session env; alternative)

Pick one for the byline-of-record. Zenodo allows changing this in
later versions but the first deposit byline is hard to overwrite cleanly.

## Item 3 — paper title final

- [ ] **(A) formal-mechanical framing (default; current paper byline)**:
      "HEXA-WEAVE Formal-Mechanical Verification: Sorry-Free Lean 4
       Closure of AX-1 (n=6 Master Identity) and AX-2 (MK-Bridge)
       under 19 Named Axioms"
- [ ] **(B) protein-folding pivot (mission-suggested)**:
      "HEXA-WEAVE: Multi-Strand Protein Weaving with n=6 Master
       Identity Trace and Lean 4 Mechanical Verification"

## Item 4 — keyword set final

- [ ] **(A) mechanical-verification-leaning (default; 8 keywords)**:
      n6-architecture / formal-verification / Lean4 / Mathlib4 /
      mechanical-theorem-proving / Felgner-conservativity /
      Morse-Kelley-class-theory / multiplicative-number-theory
- [ ] **(B) protein-folding-pivot (mission-suggested; 5 keywords)**:
      protein-folding / multi-strand / AlphaFold-alternative /
      HEXA-WEAVE / DNA-RNA-amino-composition
- [ ] **(C) hybrid (custom; specify a list of 6-12 keywords)**

## Item 5 — license final

- [ ] **(A) Apache-2.0 (default; W1 architecture decision; covers paper + Lean source)**
- [ ] **(B) CC-BY-4.0 (Zenodo academic default; paper text only; Lean source remains Apache-2.0 in repo)**

## Item 6 — related_identifiers (Lean source linkage)

- [ ] **(a) public-GitHub URL** — paste the URL here:
      `https://github.com/_____/n6-architecture`
      (this requires a separate "make repo public" approval; not free)
- [ ] **(b) tarball-only** — keep the Lean source as a Zenodo
      supplementary upload (`lean4-n6-mechverif-cycle12.tar.gz`,
      already generated, 21 KB). No public-GitHub push required.

## Item 7 — Zenodo API token

- [ ] **sandbox token** (https://sandbox.zenodo.org → Applications →
      Personal access tokens; check `deposit:write` scope) — for
      dry-run tests; the deposition is wiped after a few weeks; no
      real DOI minted
- [ ] **production token** (https://zenodo.org → Applications →
      Personal access tokens; check `deposit:write` and `deposit:actions`
      scopes) — for the actual DOI mint

DO NOT paste the token into chat. Instead:

```bash
read -s ZENODO_TOKEN  # paste, hit enter; will be invisible
export ZENODO_TOKEN
```

Then tell the bot "token is exported in my shell; please run
`bash tool/zenodo/deposit.sh --upload`."

---

## What happens after you reply

Once items 1-7 are resolved, the bot will run the full reversible
deposit (steps 1-4). The publish step (step 5) is **commented out**
in `deposit.sh` source and only runs after a SECOND explicit "publish"
approval, with the bot manually un-commenting that block.

**alien-grade transition 4.58 → 6 fires when step 5 succeeds and the
DOI is minted.**
