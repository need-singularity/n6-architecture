<!-- gold-standard: shared/harness/sample.md -->
---
domain: computational-fluid-dynamics
sector: physics
stage: S1
alien_index_estimate: 4
n6_verdict: fast_track (3D symmetric stress tensor, 6 independent components — derivable draft)
source_proposal: reports/p2_domain_expansion_proposal.json DC-032
registered: 2026-04-23
---

# Computational Fluid Dynamics (HEXA-CFD) — n=6 symmetric 3D stress tensor

## §1 WHY

A 3D symmetric tensor has **exactly n(n+1)/2 = 6 independent components**.
For Navier–Stokes / Reynolds-stress modelling, n=3 dimensions gives six
independent entries: (tau_xx, tau_yy, tau_zz, tau_xy, tau_xz, tau_yz). This is pure
linear algebra — not coincidence.

- sigma(6)=12 · phi(6)=2 · tau(6)=4 — identity uniquely at n=6.
- The same 6-component structure appears in `domains/materials/darcy` permeability
  tensor (DC-008) and in `continuum-mechanics` stress/strain. CFD inherits it
  naturally because momentum transport is governed by a symmetric-tensor flux.

## §2 n=6 CONNECTION (depth > 2 — geometric necessity)

For any symmetric rank-2 tensor on R^n the independent-component count is:
`k(n) = n(n+1)/2`. At **n = 3 spatial dimensions**:

| n | k(n) |
|---|------|
| 1 | 1    |
| 2 | 3    |
| 3 | **6** |
| 4 | 10   |

Our physical universe is 3-dimensional; therefore stress and Reynolds-stress
tensors in CFD have 6 independent components by theorem. Turbulence closure
models (Reynolds-stress transport) use exactly these 6 equations.

## §3 DIFFERENTIATION

Distinct from `fluid` (classical continuum, analytic focus) and
`plasma-physics` (charged fluids / MHD). CFD is the discrete/numerical
discipline bound to the 6-component stress-tensor closure.

## §4 VERIFY (roadmap)

- [ ] Atlas node: add `@M:cfd` under physics sector
- [ ] Assert `stress_tensor_independent_components == 6` in
      `verify_computational-fluid-dynamics.hexa`
- [ ] Cross-link to `materials/darcy` (DC-008) and `continuum-mechanics`.

## §5 SOURCES

- Pope, *Turbulent Flows*, Ch. 11 (Reynolds-stress transport model).
- Any continuum-mechanics text, e.g. Gurtin §6 (symmetric stress tensor).

## §6 STATUS

S1 (initial registration). Fast-track candidate per DIS-P2-1 top-5 review.
