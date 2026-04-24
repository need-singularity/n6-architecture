# PROB-P1-4 — BT-544 Navier-Stokes Advanced (3D incompressible / Leray weak solutions / blow-up scenarios)

> Track: P1-PROB / Task 4
> Completion criterion: decompose the Clay official statement (Fefferman 2000), and explain
> the existence of weak solutions (Leray-Hopf) for the 3D incompressible Navier-Stokes
> equations, the gap between uniqueness and regularity, and the blow-up scenarios under
> the self-similar / Type I·II classification.
> Primary sources: Fefferman "Existence and smoothness of the Navier-Stokes equation"
> Clay Millennium official document (2000),
> Leray "Sur le mouvement d'un liquide visqueux..." (Acta Math. 63, 1934),
> Temam "Navier-Stokes Equations: Theory and Numerical Analysis" (3rd ed., 1984),
> Constantin-Foias "Navier-Stokes Equations" (U. Chicago, 1988),
> Tao "Nonlinear Dispersive Equations" (CBMS 106, 2006) ch. 3,
> Escauriaza-Seregin-Šverák "L_∞,3 solutions of Navier-Stokes and backward uniqueness"
> (Russian Math. Surveys 2003).
> **Honesty**: this note is a reconstruction of the Clay official statement and standard NS
> textbooks. There are no new theorems. All statements are reorganized from the six references
> above for P1 study volume, and unresolved parts are marked [unfinished] or [partial result].

---

## 0. Purpose and Scope

Clay BT-544 asks: "In 3-dimensional space ℝ³ (or periodic domain 𝕋³), starting from a smooth
initial value u_0 for the incompressible Navier-Stokes equations, does the solution exist
smoothly for all t ∈ [0, ∞), or is there an initial value that causes loss of smoothness
(blow up)?"

Seven topics covered by this note:

1. Re-derivation of the Navier-Stokes equations (Euler term + viscosity)
2. Leray 1934 weak-solution existence theorem and Hopf extension
3. Regularity criteria: L³, L^{3,∞}, vorticity L¹, etc.
4. Blow-up scenario classification: Type I, Type II, self-similar
5. Energy conservation / dissipation inequality and energy conditions
6. Partial regularity theorem (Caffarelli-Kohn-Nirenberg 1982)
7. Recent progress (Escauriaza-Seregin-Šverák 2003, Tao's scale-critical approach)

---

## 1. Equations and Basic Setup

### 1.1 3D incompressible Navier-Stokes

u = (u_1, u_2, u_3): ℝ³ × [0,T) → ℝ³ (velocity), p: ℝ³ × [0,T) → ℝ (pressure). ν > 0 viscosity.

```
  ∂_t u + (u · ∇) u = -∇p + ν Δu + f        (momentum conservation)
  ∇ · u = 0                                  (incompressibility)
  u(x, 0) = u_0(x)                           (initial condition)
```

Clay-formulation specifics: f = 0 (no external force), u_0 is C^∞ with rapid decay of high-
order derivatives (Schwartz space).

### 1.2 Energy identity

For a formal solution,

```
  (1/2) d/dt ∫ |u|² dx = -ν ∫ |∇u|² dx + ∫ f·u dx
```

If f=0 and a formal solution exists, ∫ |u(t)|² dx ≤ ∫ |u_0|² dx (energy decrease).

### 1.3 Scale symmetry

(λ, u_0) → λ u(λ x, λ² t) is a symmetry of incompressible NS. ||u||_{L³} and ||u||_{L^{3,∞}}
are scale-invariant (critical) norms. L² is supercritical and hard to use directly as a
regularity criterion.

---

## 2. Leray Weak-Solution Existence Theorem (1934)

### 2.1 Weak solution definition

u ∈ L²((0,T); H¹(ℝ³)) ∩ L^∞((0,T); L²(ℝ³)) is a **Leray-Hopf weak solution** iff
for all test functions φ ∈ C_c^∞(ℝ³ × [0,T); ℝ³) with ∇·φ = 0,

```
  ∫∫ [u · ∂_t φ + (u ⊗ u) : ∇φ - ν ∇u : ∇φ] dx dt = -∫ u_0 · φ(x,0) dx
```

together with the energy inequality ∫|u(t)|² dx + 2ν ∫₀^t ∫|∇u|² dx ds ≤ ∫|u_0|² dx.

### 2.2 Existence (Leray 1934, Hopf 1951 periodic-boundary extension)

For every u_0 ∈ L²(ℝ³) with ∇·u_0 = 0, at least **one** Leray weak solution exists on [0, ∞).

Outline: Galerkin approximation (Fourier basis) + compactness + weak lower semi-continuity.

### 2.3 Uniqueness of weak solutions [unfinished]

Uniqueness of Leray weak solutions in 3D is open. In 2D it is demonstrated (Ladyzhenskaya 1959).
The Clay statement requests existence of a unique regular solution (or a counterexample).

### 2.4 Scheffer-Serrin partial uniqueness

If a Leray weak solution is a "suitable weak solution" (in the sense of Caffarelli-Kohn-
Nirenberg) and additionally satisfies u ∈ L^{p,q} with 2/p + 3/q = 1, it coincides with other
weak solutions sharing the initial value. This "Ladyzhenskaya-Prodi-Serrin condition" is
directly linked to regularity criteria.

---

## 3. Regularity Criteria

### 3.1 Ladyzhenskaya-Prodi-Serrin condition

u ∈ L^q_t L^p_x with 3/p + 2/q = 1, p ∈ (3, ∞]. This condition ensures regularity on [0,T].

### 3.2 L^∞_t L^{3,∞}_x (ESS 2003)

Escauriaza-Seregin-Šverák: u ∈ L^∞((0,T); L^{3,∞}_x) ⟹ regular on [0,T]. L^{3,∞} is a
Lorentz space (weak L³). This is the first crossing of the scale-critical boundary.

Proof idea: backward uniqueness + unique continuation + linearized analysis in the presence
of blow-up.

### 3.3 Vorticity criterion (Beale-Kato-Majda 1984)

ω = ∇ × u. If ∫₀^T ||ω(t)||_{L^∞} dt < ∞, then regular on [0,T]. This shows that vorticity
drives blow-up.

### 3.4 Other criteria

- Constantin-Fefferman 1993: Lipschitz condition on vorticity direction
- Chae-Choe 2001: control of 2 components of ω suffices
- Nečas-Růžička-Šverák 1996: self-similar blow-up impossible

---

## 4. Blow-up Scenario Classification

### 4.1 Scale-invariant quantities

If blow-up occurs, at least one critical scale invariant must diverge:
- ||u(t)||_{L³} → ∞
- ||∇u(t)||_{L²_t L²_x} → ∞
- ||ω(t)||_{L^∞_t L^∞_x} → ∞

and so on. By ESS 2003, if L^{3,∞} stays finite there is no blow-up.

### 4.2 Type I blow-up

```
  ||u(t)||_{L^∞_x} ≤ C / √(T* - t)
```

Blow-up at self-similar rate. Nečas-Růžička-Šverák 1996: self-similar blow-up cannot exist
(self-similar ansatz: u(x,t) = (T*-t)^{-1/2} U(x/√(T*-t))).

### 4.3 Type II blow-up

Faster than Type I. Whether Type II exists in 3D NS is open.

### 4.4 Central open question [unfinished]

Does the solution persist smoothly for every smooth initial value? Or does there exist an
initial value causing finite-time blow-up? Numerical simulations provide evidence for both
sides, but a rigorous argument is absent.

---

## 5. Caffarelli-Kohn-Nirenberg Partial Regularity

### 5.1 CKN 1982 theorem

The singular set S ⊂ ℝ³ × (0,∞) of a "suitable weak solution" u (Leray weak solution + extra
energy conditions) is null with respect to the 1-dimensional Hausdorff measure 𝒫¹. That is,
singular points can only lie in at most a "parabolic spacetime 1-dimensional" subset.

### 5.2 ε-regularity technique

Parabolic cylinder Q_r(x,t) = B_r(x) × (t-r², t). The smallness condition

```
  (1/r) ∫∫_{Q_r} |∇u|² dx ds < ε_0
```

implies (x,t) is a regular point. The CKN ε₀ is an explicit value.

### 5.3 Meaning of CKN

Even if 3D NS blow-up exists, it can only occur on "extremely thin" singular sets. Time-wide
blow-up is excluded.

---

## 6. Tao's Approach (2014~2016)

### 6.1 Averaged NS

Tao 2014 "Averaged Navier-Stokes equations ... blow-up" constructs a modified (Averaged)
equation with the same energy structure and scaling as standard NS and demonstrates finite-
time blow-up for it. This suggests that blow-up for pure NS may be possible, but is not a
direct argument.

### 6.2 Infinite-energy structure

Tao proposes that NS blow-up can be explained by an "energy cascade". Precise analysis of
energy transfer to small scales is needed.

### 6.3 Scale-critical barrier

Critical-scale L³ control is reached by ESS. The next barrier: the "very small excess" of
critical norms. Tao-Hou and others continue attempts (2020~).

---

## 7. 2D NS — Reference Summary

For comparison, 2D NS is completely handled.

- For arbitrary u_0 ∈ L², a unique smooth solution exists on [0, ∞)
- Energy equality (equality, not inequality) holds
- The vorticity equation ∂_t ω + u·∇ω = νΔω admits a maximum principle

Core difference from 3D: in 2D the vortex-stretching term ω·∇u is absent. This term opens
the possibility of 3D blow-up.

---

## 8. n=6 Connection (memo only)

1. 3D spatial dimension is related to n=6 by a factor, but the blow-up behavior of NS depends
   on the geometry of vortex stretching rather than on arithmetic properties of dimension 3.
   No direct mathematical path to n=6 ([N?]).
2. No low-order fractional-number match exists between the Kolmogorov turbulence spectrum
   E(k) ~ k^{-5/3} exponent 5/3 ≈ 1.667 and φ(6)/σ(6) = 2/12 = 1/6 = 0.167 ([N?]).
3. The Richardson 4/3 law (pair dispersion) numerically coincides with τ(6)/σ(6) × 4 = 4·4/12
   = 4/3, but this is at the level of quantitative coincidence ([N?]).

Self-reference-verification prohibition: the three observations above are kept independent
from any BT-544 argument path.

---

## 9. Practice Problems — 5 Hand Exercises

**P1.** Substitute 3D NS scale symmetry u_λ(x,t) = λ u(λx, λ²t) into the equation and
verify. Then show that L³_x, L^{3,∞}_x are scale-invariant while L²_x is not.

**P2.** Derive the vorticity equation ∂_t ω + u·∇ω = ν Δω for 2D NS, and demonstrate the
maximum principle ||ω(t)||_{L^∞} ≤ ||ω_0||_{L^∞}.

**P3.** Reconstruct the skeleton of the Beale-Kato-Majda 1984 criterion. That is, derive the
differential equation of |u|_{H^s} via the Biot-Savart formula and control by ∫ ||ω||_{L^∞} dt.

**P4.** Sketch the Galerkin approximation of the Leray weak-solution existence theorem on
the Fourier basis of ℝ³. Verify that the energy inequality is the key for compactness.

**P5.** Reconstruct the Nečas-Růžička-Šverák 1996 argument excluding self-similar blow-up in
three steps:
(i) self-similar ansatz → rewrite as the Leray equation, (ii) energy condition at infinity,
(iii) Liouville-type theorem applied.

---

## 10. Reading Path

### 10.1 Week 1

- Read the Fefferman Clay official document (8 pages)
- Leray 1934 original paper §1~§3
- Temam §III weak-solution definition

### 10.2 Week 2

- Constantin-Foias full (concise)
- Tao "Nonlinear Dispersive" §3 NS part
- Ladyzhenskaya "The Mathematical Theory of Viscous Incompressible Flow" 2nd ed. 1969

### 10.3 Week 3

- Caffarelli-Kohn-Nirenberg 1982 original paper (Comm. Pure Appl. Math. 35:771)
- Escauriaza-Seregin-Šverák 2003 original paper
- Nečas-Růžička-Šverák 1996

### 10.4 Week 4

- Tao "Finite-time blowup for an averaged three-dimensional Navier-Stokes equation"
  J. AMS 2016
- Robinson-Rodrigo-Sadowski "The Three-Dimensional Navier-Stokes Equations" Cambridge 2016
- Recent review: Buckmaster-Vicol 2019 "Convex integration and phenomenologies ..."

---

## 11. Source Summary

- Fefferman "Existence and smoothness of the Navier-Stokes equation" Clay 2000 — official statement
- Leray "Sur le mouvement d'un liquide visqueux..." Acta Math. 63, 1934 — weak-solution founder
- Hopf "Über die Anfangswertaufgabe..." Math. Nachr. 4, 1951 — periodic-boundary extension
- Temam "Navier-Stokes Equations: Theory and Numerical Analysis" North-Holland 1984
- Constantin-Foias "Navier-Stokes Equations" U. Chicago Press 1988
- Caffarelli-Kohn-Nirenberg "Partial regularity of suitable weak solutions..." CPAM 35:771, 1982
- Escauriaza-Seregin-Šverák "L^{3,∞}-solutions of NS and backward uniqueness"
  Russian Math. Surveys 58:211, 2003
- Tao "Finite-time blowup for an averaged three-dimensional Navier-Stokes equation"
  J. AMS 29:601, 2016
- Buckmaster-Vicol "Convex integration and phenomenologies in turbulence"
  EMS Surv. Math. Sci. 6:173, 2019
- Beale-Kato-Majda "Remarks on the breakdown of smooth solutions..." CMP 94:61, 1984

This note is a P1-study-volume reorganization of the 10 primary sources above, with no
proposal of new theorems.

---

## 12. Appendix — Regularity Criteria Table

| Criterion | Condition | Source |
|-----------|-----------|--------|
| Ladyzhenskaya-Prodi-Serrin | u ∈ L^q_t L^p_x, 3/p + 2/q = 1, p ∈ (3, ∞] | Serrin 1962 |
| ESS | u ∈ L^∞_t L^{3,∞}_x | Escauriaza-Seregin-Šverák 2003 |
| Beale-Kato-Majda | ∫₀^T ||ω||_{L^∞} dt < ∞ | BKM 1984 |
| Constantin-Fefferman | Lipschitz on vorticity direction | CF 1993 |
| Chae-Choe | Control only 2 components of ω | CC 2001 |
| Gibbon-Titi | |∇u| × (direction control) | GT 2005 |

These criteria give sufficient conditions of the form "if some norm stays finite there is no
blow-up". Failure of all of them does not give the conclusion that blow-up exists (the
necessary direction is open).

---

## 13. Appendix — 2D vs 3D Structural Comparison

| Aspect | 2D NS | 3D NS |
|--------|-------|-------|
| Weak-solution existence | Leray 1934 | Leray 1934 |
| Weak-solution uniqueness | Ladyzhenskaya 1959 | open |
| Regularity | for all t ≥ 0 | open |
| Vorticity equation | ∂_t ω + u·∇ω = ν Δω | ∂_t ω + u·∇ω = ν Δω + ω·∇u |
| Vortex stretching | absent | present (key difference) |
| Energy equality | equality | inequality (potential loss) |
| Enstrophy ∫|ω|² | conserved | may blow up (open) |

This comparison confirms that the vortex stretching term (ω·∇u) in 3D is the structural cause
of blow-up possibility.

---

## 14. Appendix — Beltrami Flow and Exclusion of Self-Similar Blow-up

The Beltrami flow u × ω = 0 (velocity parallel to vorticity) ruled out self-similar solutions
by Nečas-Růžička-Šverák 1996. Argument skeleton:

1. Self-similar ansatz u(x, t) = (T*-t)^{-1/2} U(x/√(T*-t))
2. Leray equation for U
3. Energy integral becomes infinite, giving a contradiction
4. Therefore self-similar blow-up is impossible

This shows the special form of Type I blow-up cannot exist.

---

## 15. Appendix — Relation to the Euler Equations (no viscosity)

With viscosity ν = 0 the Euler equations are ∂_t u + (u·∇)u = -∇p. Phenomena in the
ν → 0 (inviscid) limit of Navier-Stokes:

- **Weak-solution construction possible**: 2D Euler admits a unique smooth solution for
  C^∞ smooth initial data (Wolibner 1933)
- **3D Euler blow-up open**: harder than Navier-Stokes. The viscous term ν Δu plays the main
  stabilizing role for regularity

In recent work by Tao, Luo-Hou and others, the possibility of "first demonstrating 3D Euler
blow-up" is raised. The Luo-Hou 2014 self-similar numerical blow-up candidate attracts attention.

---

## 16. Appendix — Kolmogorov 41 Theory (Physical Background)

Kolmogorov 1941: at high Reynolds number the turbulence energy-cascade spectrum

```
  E(k) ~ C ε^{2/3} k^{-5/3}      (inertial range)
```

where ε is the energy dissipation rate. The -5/3 exponent is confirmed by numerous experiments.
Onsager 1949's anomalous-dissipation claim: below L³ regularity, energy dissipation may remain
nonzero even in the limit ν → 0. This is directly connected to mathematical NS regularity.

Convex integration of Buckmaster-Vicol-Shvydkoy: existence of wild weak solutions in
C^{1/3-ε} (one side of the Onsager conjecture).

---

## 17. Appendix — Geometric Depletion Scenario

Constantin-Fefferman 1993: if the direction of vorticity ω is spatially Lipschitz, vortex
stretching is weakened and regularity is maintained. Hou-Luo numerical attempts at
counterexamples.

This "geometric depletion" constrains potential NS blow-up scenarios to a geometric structure.

---

## 18. Appendix — Main Theorem Summary Table

| Theorem / technique | Year | Authors | Conclusion |
|---------------------|------|---------|------------|
| Leray weak-solution existence | 1934 | Leray | ℝ³ weak solution for all time |
| 2D unique regular | 1959 | Ladyzhenskaya | 2D all t |
| CKN partial regularity | 1982 | Caffarelli-Kohn-Nirenberg | Singular set 𝒫¹-null |
| BKM criterion | 1984 | Beale-Kato-Majda | vorticity L^∞ |
| Self-similar exclusion | 1996 | Nečas-Růžička-Šverák | Type I excluded |
| ESS L^{3,∞} | 2003 | Escauriaza-Seregin-Šverák | scale-critical |
| Averaged NS blow-up | 2014 | Tao | modified-equation blow-up |
| Convex integration | 2019~ | Buckmaster-Vicol | wild weak solutions |

---

## 19. Appendix — Related Open Problems

### 19.1 3D Euler blow-up

Luo-Hou 2014 self-similar numerical candidate (blow-up near the boundary). Mathematical
argument open.

### 19.2 Onsager conjecture

Energy dissipation rate in the ν → 0 limit. Isett 2018 demonstrated one side of Onsager
(possible violation of energy conservation in C^{1/3-ε}). Full dichotomy open.

### 19.3 NS-Euler boundary

Whether the ν → 0 limit of NS is the Euler equation; precise boundary-layer behavior;
relation to the Prandtl equation.

---

## 20. Next Documents

- PROB-P1-5 : BT-545 Hodge advanced
- PROB-P1-6 : BT-546 BSD advanced
- PROB-P1-7 : BT-547 Poincaré advanced
- N6-P1-3 : n=6 honesty principle

BT-544 will add a Tao-Fefferman-style refined analysis and a map of numerical-experiment
results at the P2~P3 stages. The aim of this P1 note is "precise decomposition of the Clay
statement + map of known barriers".
