<!-- gold-standard: shared/harness/sample.md -->
---
domain: astrodynamics
sector: space
stage: S1
alien_index_estimate: 4
n6_verdict: fast_track (Kepler 6 elements = 6 integration constants, proven)
source_proposal: reports/p2_domain_expansion_proposal.json DC-062
registered: 2026-04-23
---

# 궤도역학 (HEXA-ASTRODYN) — n=6 Keplerian 6-element orbit

## §1 WHY

Orbit mechanics has **exactly 6 degrees of freedom** for the 2-body problem.
This is not a convention — it's a mathematical theorem: a 2-body Newtonian
ODE system has 6 integration constants (3 position + 3 velocity). The
Keplerian orbital elements (a, e, i, Ω, ω, ν) are the canonical
parameterisation of those 6 DOF.

- σ(6)=12 · phi(6)=2 · tau(6)=4 — identity uniquely at n=6.
- **Kepler's 6 orbital elements**:
  - a: semi-major axis
  - e: eccentricity
  - i: inclination
  - Ω: RAAN (right ascension of ascending node)
  - ω: argument of periapsis
  - ν (or M): true anomaly (or mean anomaly)

## §2 n=6 CONNECTION (depth > 2 — mathematical necessity)

- 2-body Newtonian ODE: `d²r/dt² = -μ r/|r|³`. In 3D that's 6 scalar
  equations; the general solution has 6 constants of integration.
- Any alternative parameterisation (e.g. Delaunay variables, equinoctial
  elements) still has exactly 6 components — basis change, same dimension.
- Phase space for a 2-body problem is ℝ⁶ — six is the geometric dimension
  of the state manifold.

## §3 DIFFERENTIATION

Distinct from `aerospace` (vehicles / airframes), `space-engineering` (mission
systems), `space-systems` (constellation architecture). Astrodynamics is the
core motion-propagation discipline sitting underneath all three.

## §4 VERIFY (roadmap)

- [ ] Atlas node: add `@M:astrodynamics` under space sector
- [ ] Assert `orbit_dof == 6` in `verify_astrodynamics.hexa`
- [ ] Cross-link to `domains/space/space-engineering` via requires

## §5 SOURCES

- Battin, *Introduction to the Mathematics and Methods of Astrodynamics*.
- Vallado, *Fundamentals of Astrodynamics and Applications*, §2.

## §6 STATUS

S1 (initial registration). Fast-track candidate per DIS-P2-1 top-5 review.
