<!-- gold-standard: shared/harness/sample.md -->
---
domain: nuclear-medicine
sector: life
stage: S1
alien_index_estimate: 7
n6_verdict: fast_track (Tc-99m half-life 6 hours = empirical nuclear constant)
source_proposal: reports/p2_domain_expansion_proposal.json DC-017
registered: 2026-04-23
---

# 핵의학 (HEXA-NUCMED) — n=6 Tc-99m 6-hour half-life

## §1 WHY

The dominant radiopharmaceutical in SPECT imaging is **Tc-99m**, whose
half-life is **6.01 hours** — an empirical nuclear constant measured to high
precision by the NNDC. Essentially every diagnostic nuclear-medicine protocol
is timed around this 6-hour decay. That is not depth-2 coincidence — it is
the operating point chosen precisely because of the decay constant.

- σ(6)=12 · phi(6)=2 · tau(6)=4 — identity uniquely at n=6.
- **Tc-99m t½ = 6.01 h** (measured): balances imaging statistics (slow enough
  to inject, image, and process) against patient dose (fast enough that the
  activity is gone within ~24 h).
- **PET detector rings**: 6 detector blocks is the minimum angular sampling
  unit for tomographic reconstruction fidelity in several commercial scanners.

## §2 n=6 CONNECTION (depth > 2 — nuclear physics + clinical optimum)

The 6-hour half-life is both:

1. A nature-given nuclear constant of ⁹⁹ᵐTc decay (isomeric transition to
   ⁹⁹Tc, γ = 140.5 keV).
2. The clinical operating optimum — shorter half-lives give no time for
   uptake, longer ones give excess patient dose. 6 hours sits at the
   pharmacokinetic saddle point.

## §3 DIFFERENTIATION

Distinct from `bio-pharma` (drug development), `pharmacology` (pharmacokinetics
in general), `cancer-therapy` (treatment modalities). Nuclear medicine is the
diagnostic-imaging-with-radionuclides sub-discipline.

## §4 VERIFY (roadmap)

- [ ] Atlas node: add `@M:nuclear-medicine` under life sector
- [ ] Assert `tc99m_half_life_hours == 6` (±1% tolerance) in
      `verify_nuclear-medicine.hexa`
- [ ] Cross-link to `physics/medical-physics` and `life/cancer-therapy`.

## §5 SOURCES

- NNDC Chart of Nuclides (Tc-99m t½ = 6.0067 h ± measurement).
- Cherry, Sorenson & Phelps, *Physics in Nuclear Medicine*.

## §6 STATUS

S1 (initial registration). Fast-track candidate per DIS-P2-1 top-5 review.
