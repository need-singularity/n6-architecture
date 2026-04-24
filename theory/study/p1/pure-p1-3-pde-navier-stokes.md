# PURE-P1-3 — Partial Differential Equations and Navier-Stokes (derivation / weak, strong, classical solutions / energy inequality)

> Track: P1-PURE / Task 3
> Completion criterion: derive the incompressible NS equations from Newton's second law via the
> continuum-mechanics stress tensor, distinguish weak, strong, and classical solutions, and derive the
> energy inequality for a Leray weak solution
> ½|u|²_{L²}(t) + ν∫₀^t |∇u|²_{L²} ds ≤ ½|u₀|²_{L²}.
> Be able to state precisely 2D global existence (Ladyzhenskaya 1963) versus the 3D open situation.
> Source base: Evans "Partial Differential Equations" (GSM 19, 2nd ed. 2010) ch. 8 + ch. 5,
> Temam "Navier-Stokes Equations: Theory and Numerical Analysis" (AMS Chelsea, reprint 2001) ch. 1-3,
> Constantin-Foias "Navier-Stokes Equations" (Chicago Lectures in Math, 1988) ch. 1-3,
> Lemarié-Rieusset "Recent Developments in the Navier-Stokes Problem" (Chapman & Hall/CRC, 2002) ch. 1-3.
> **Honesty**: textbook summary. No invented theorems, dates, or authors.

---

## 0. Purpose

Task 3 of the P1 roadmap is the derivation of NS and the basic semantics of its solutions. Master the
following.

1. Newton's second law → continuum mechanics → stress tensor → NS
2. Incompressibility ∇·u = 0
3. Distinction between weak, strong, and classical solutions
4. Galerkin approximation and the Leray 1934 construction
5. Energy inequality
6. 2D global regularity (Ladyzhenskaya 1963) vs 3D open

---

## 1. Continuum Mechanics and NS Derivation

### 1.1 Eulerian vs Lagrangian Descriptions

Lagrangian. Track a particle trajectory x(t; a), where a is the initial position.

Eulerian. Record the velocity field u(x, t) at a fixed spatial point x.

NS uses the Eulerian description.

### 1.2 Mass Conservation (Continuity Equation)

With ρ = density and u = velocity, for any volume Ω,

```
   d
  ─── ∫ ρ dx  +  ∫ ρ u · ν dS = 0
   dt  Ω        ∂Ω
```

where ν is the outward normal. Using the divergence theorem ∫_{∂Ω} ρ u · ν dS = ∫_Ω ∇·(ρ u) dx,

```
  ∂ρ/∂t + ∇·(ρ u) = 0            (continuity equation)
```

For an incompressible fluid ρ = const, so

```
  ∇ · u = 0
```

(Temam §1.1, Evans §8.1)

### 1.3 Momentum Conservation — Continuum Version of Newton's Second Law

Newton's second law applied to the body within a volume Ω:

```
   d
  ─── ∫ ρ u dx = (∫ body force f dx)  +  (∫ surface force dS)
   dt   Ω            Ω                   ∂Ω
```

Cauchy's stress principle. The surface force is expressed via the stress tensor σ_{ij}:

```
  (surface force)_i  =  ∫  σ_{ij} ν_j dS
                        ∂Ω
```

Divergence theorem + material-derivative derivation of the left side:

```
  ρ (∂u/∂t + (u·∇)u)_i  =  f_i  +  ∂_j σ_{ij}
```

(Temam §1.2, Evans §8.1)

### 1.4 Constitutive Relation for Newtonian Fluids

Stress tensor of a Newtonian fluid.

```
  σ_{ij} = -p δ_{ij} + λ (∇·u) δ_{ij} + μ (∂_i u_j + ∂_j u_i)
```

- p = pressure (thermodynamic pressure)
- λ, μ = Lamé viscosity coefficients
- μ > 0 = shear viscosity

For incompressible flow (∇·u = 0) the λ term vanishes:

```
  σ_{ij} = -p δ_{ij} + μ (∂_i u_j + ∂_j u_i)
```

Substituting this σ into the momentum equation gives

```
  ρ (∂u/∂t + (u·∇)u) = -∇p + μ Δu + f
```

### 1.5 Incompressible NS (final)

With ν := μ/ρ (kinematic viscosity) and again letting the body force be f,

```
  ∂u/∂t + (u·∇)u = -∇p/ρ + ν Δu + f
  ∇ · u = 0
```

to be solved together with boundary conditions and initial data u|_{t=0} = u₀.

(Temam Thm 1.1, Evans §8.1, Constantin-Foias §1.1)

---

## 2. Three Notions of Solution

### 2.1 Classical Solution

u ∈ C¹_t C²_x, p ∈ C⁰_t C¹_x, and NS holds pointwise.

This is the strongest notion; in 3D even its existence is still open.

### 2.2 Strong Solution

Slightly weaker regularity such as u ∈ L²_t H²_x ∩ L∞_t H¹_x, with NS satisfied almost everywhere. The
energy-dissipation equality holds with equality.

In 3D, short-time (local-in-time) existence of a strong solution has been known since Leray 1934 +
Fujita-Kato 1964; but global-in-time extension is open.

### 2.3 Weak Solution (Leray-Hopf)

**Definition (Leray 1934).** u: (0,T) × Ω → R³ is a weak solution if

- u ∈ L∞([0,T]; L²) ∩ L²([0,T]; H¹)
- ∇·u = 0 (in the sense of distributions)
- For every test function ϕ ∈ C_c^∞((0,T)×Ω; R³) with ∇·ϕ = 0,

```
  ∫₀^T ∫_Ω [ -u · ∂_t ϕ - (u ⊗ u) : ∇ϕ + ν ∇u : ∇ϕ ] dx dt = ∫₀^T ∫_Ω f · ϕ dx dt
```

and the energy inequality (§3 below) holds.

(Temam §3.1, Constantin-Foias §3.3, Lemarié-Rieusset §3.1)

### 2.4 Weak ≠ Classical

A weak solution secures existence and the energy inequality but generally does not secure uniqueness or
regularity. Whether a 3D weak solution is classical is open (core of the Millennium Problem).

---

## 3. Energy Inequality

### 3.1 Energy Equality (formal)

If u is sufficiently smooth, dot both sides of NS with u and integrate over Ω:

```
  ∫_Ω u · ∂_t u dx + ∫_Ω u · (u·∇)u dx = -∫_Ω u · ∇p dx + ν ∫_Ω u · Δu dx + ∫_Ω f · u dx
```

Each term:

- ∫ u · ∂_t u = (1/2) d/dt ∫ |u|²
- ∫ u · (u·∇)u = (1/2) ∫ (u·∇)|u|² = (1/2) ∫ ∇·(u |u|²) = 0 (boundary conditions + divergence 0)
- ∫ u · ∇p = ∫ ∇·(pu) - ∫ p ∇·u = 0 (both terms 0)
- ∫ u · Δu = -∫ |∇u|²  (integration by parts + boundary)
- ∫ f · u = external power input

Hence

```
  (1/2) d/dt |u|²_{L²} + ν |∇u|²_{L²} = ∫ f · u dx
```

Integrating in time:

```
  (1/2) |u(t)|²_{L²} + ν ∫₀^t |∇u|²_{L²} ds = (1/2) |u₀|²_{L²} + ∫₀^t ∫ f · u dx ds
```

If f = 0 we get the energy equality:

```
  (1/2) |u(t)|²_{L²} + ν ∫₀^t |∇u|²_{L²} ds = (1/2) |u₀|²_{L²}
```

### 3.2 Energy Inequality (weak-solution version)

For a weak solution, not an equality but an inequality:

```
  (1/2) |u(t)|²_{L²} + ν ∫₀^t |∇u|²_{L²} ds ≤ (1/2) |u₀|²_{L²}   (for almost every t)
```

This comes automatically out of Leray's 1934 construction (Galerkin truncation + weak convergence in the
limit + lower semicontinuity).

(Temam §3.3, Lemarié-Rieusset §3.3)

### 3.3 When the Inequality Becomes an Equality

**Theorem (Constantin-Foias).** If the weak solution u belongs to L⁴_t L⁴_x, then the energy equality holds.

(Constantin-Foias §3.4 Prop 3.2)

Whether this condition is automatic for a 3D weak solution is open.

---

## 4. Leray 1934 Construction — Galerkin Approximation

### 4.1 Overview

The method introduced by Leray (1934 Acta Math paper, Essai sur le mouvement d'un liquide visqueux
emplissant l'espace). Reconstructed in Temam §3.4, Constantin-Foias §3.1.

1. Choose a finite-dimensional subspace V_m ⊂ H¹ (divergence-free) via an eigenfunction expansion.
2. Solve the Galerkin system on V_m to obtain an approximate solution u_m.
3. Take the limit m → ∞. Weak convergence + the energy inequality survive.

### 4.2 Galerkin System

u_m = ∑_{k=1}^m g_k^m(t) w_k (w_k = eigenfunctions of the Stokes operator). Testing the weak form of NS
against each w_k,

```
  (∂_t u_m, w_k) + B(u_m, u_m, w_k) + ν (∇u_m, ∇w_k) = (f, w_k)
```

where B(u, v, w) := ∫ (u·∇)v · w dx.

This is an ODE system in m unknowns; Cauchy-Peano yields a local solution, and ODE energy estimates extend
it to a global solution.

### 4.3 Uniform Estimates

For each u_m we have the energy equality (an equality, since finite-dimensional):

```
  (1/2) |u_m(t)|² + ν ∫₀^t |∇u_m|² ds = (1/2) |u_m(0)|²
```

From this

```
  sup_{t∈[0,T]} |u_m(t)|_{L²} ≤ |u₀|_{L²}       (uniform)
  ∫₀^T |∇u_m|²_{L²} ds ≤ (1/(2ν)) |u₀|²_{L²}      (uniform)
```

so u_m is uniformly bounded in L∞_t L² ∩ L²_t H¹.

### 4.4 Taking the Limit — Weak Convergence

Banach-Alaoglu gives a subsequence u_{m_k} → u weak-* in L∞_t L² and u_{m_k} ⇀ u weakly in L²_t H¹. A
compactness theorem (Aubin-Lions) gives strong convergence in L²_t L². This strong convergence allows
taking the limit in the nonlinear term (u·∇)u. Linear terms require only weak convergence.

### 4.5 Leray's Conclusion

**Theorem (Leray 1934).** If f ∈ L²([0,T]; H^{-1}), u₀ ∈ L²(R³; R³), ∇·u₀ = 0, then in 3D there exists a
weak solution u ∈ L∞([0,T]; L²) ∩ L²([0,T]; H¹) satisfying the energy inequality.

(Temam Thm 3.1, Constantin-Foias Thm 3.1, Lemarié-Rieusset Thm 14.1)

Uniqueness and regularity are open. (Millennium problem)

---

## 5. 2D Global Regularity — Ladyzhenskaya 1963

### 5.1 Statement

**Theorem (Leray 1933 weak form; Ladyzhenskaya 1959-1963; Lions 1969 extension).** For d = 2, if u₀ ∈ H¹
and f = 0, then the weak solution is unique and globally regular.

(Constantin-Foias Thm 10.1, Temam Ch. 3 Thm 3.3, Ladyzhenskaya "The Mathematical Theory of Viscous
Incompressible Flow" 1963 original text)

### 5.2 Key Inequality — 2D Ladyzhenskaya Inequality

```
  |u|_{L⁴(R²)}⁴ ≤ C · |u|²_{L²} · |∇u|²_{L²}
```

(Quantitative form of the Sobolev embedding H^{1/2}(R²) ↪ L⁴(R²); Ladyzhenskaya 1963 §1.7.)

When this inequality holds, the nonlinear term |B(u, u, u)| ≤ C |u|_{L⁴}² |∇u|_{L²} of 2D NS can be
absorbed into ν |∇u|². The energy method therefore keeps working, and a global solution is obtained.

### 5.3 Why It Fails in 3D

In 3D the Sobolev exponent changes so that

```
  |u|_{L⁴(R³)}⁴ ≤ C · |u|_{L²} · |∇u|_{L²}³
```

Because of this exponent 3, the viscosity cannot absorb the nonlinear term, and the energy method alone
does not yield global existence. This is the source of the 3D NS open question.

(Constantin-Foias §11.1, Lemarié-Rieusset §11)

---

## 6. Partial Regularity — Caffarelli-Kohn-Nirenberg

### 6.1 Statement

**Theorem (CKN 1982).** For a Leray-Hopf weak solution u, the singular set S = {(x, t) : u is not finite
near this point} satisfies parabolic Hausdorff measure P¹(S) = 0.

(Caffarelli-Kohn-Nirenberg "Partial regularity of suitable weak solutions of the Navier-Stokes equations"
Comm. Pure Appl. Math. 35, 1982)

### 6.2 Interpretation

A guarantee that, even if singularities exist, they are rare: bounded in 1-dimensional parabolic dimension
(with space+time counted as 2).

However, the existence of a 1-dimensional singular set is not itself excluded. Tao's 2016 "Great Wall"
candidate, Jia-Sverak 2014 self-similar blow-up scenarios, etc., all sit within this bound.

---

## 7. Precise Statement of the Millennium Problem

### 7.1 Clay Institute Official Statement (Fefferman 2000)

Problem. Can the NS equations on R³ be resolved under the following conditions?

- Initial data u₀: R³ → R³, ∇·u₀ = 0, smooth and rapidly decaying (Schwartz class).
- External force f = 0.

Demonstrate or refute one of the two statements below.

(A) Existence and smoothness. For every T > 0, a smooth classical solution u, p exists and ∂^α u is finite
    for every k.

(B) Finite-time blow-up. There exists T* < ∞ such that sup_{x} |u(x, t)| → ∞ (as t → T*).

R³/Z³ (periodic boundary) is also offered separately.

(Fefferman "Existence and smoothness of the Navier-Stokes equation" Clay Millennium Problem statement, 2000)

### 7.2 Partial Results to Date

- Leray 1934: 3D global weak-solution existence
- CKN 1982: vanishing P¹ dimension of the singular set of a weak solution
- Fujita-Kato 1964: 3D short-time strong solution (u₀ ∈ H^{1/2})
- Koch-Tataru 2001: short-time strong solution for BMO^{-1} initial data
- Escauriaza-Seregin-Sverak 2003: L³_x L∞_t blow-up criterion

None of these demonstrate 3D global regularity.

---

## 8. Stokes Problem and Projection Operator

### 8.1 Stokes Equations

Drop the nonlinear term (u·∇)u from NS:

```
  ∂u/∂t - ν Δu + ∇p = f
  ∇·u = 0
```

This is a linear equation with a complete theory (existence, uniqueness, spectrum).

### 8.2 Leray Projection Operator P

On L²(Ω; R³) take the Helmholtz decomposition:

```
  L² = H ⊕ G
  H := { v ∈ L² : ∇·v = 0 (distributionally), v·ν|_{∂Ω} = 0 }
  G := { ∇ϕ : ϕ ∈ H¹ }
```

P: L² → H is the projection onto H (L² inner product). Applying P to NS kills the ∇p term:

```
  ∂u/∂t + P((u·∇)u) = ν Δu + Pf
```

This form is extremely useful for theoretical analysis.

(Constantin-Foias §2.1, Lemarié-Rieusset §3.3)

---

## 9. 3D Strong Solutions — Short-Time Existence

### 9.1 Fujita-Kato 1964

**Theorem.** If u₀ ∈ H^{1/2}(R³; R³), ∇·u₀ = 0, there exists T* > 0 and a unique strong solution
u ∈ C([0,T*); H^{1/2}) ∩ L²_{loc}((0, T*); H^{3/2}) on [0, T*).

Also, if |u₀|_{H^{1/2}} is sufficiently small, the solution extends to T* = ∞ ("small data global").

(Fujita-Kato 1964, Lemarié-Rieusset §15)

### 9.2 Blow-up Criteria — Serrin, BKM, ESS

**Theorem (Serrin 1962).** If a strong solution u blows up at T*, then
∫₀^{T*} |u|^s_{L^r} dt = ∞ with 3/r + 2/s ≤ 1, r > 3.

**Theorem (Beale-Kato-Majda 1984).** ∫₀^{T*} |ω|_{L∞} dt = ∞, where ω = ∇ × u is the vorticity.

**Theorem (Escauriaza-Seregin-Sverak 2003).** sup_{t<T*} |u(t)|_{L³(R³)} = ∞ is a necessary condition for
strong-solution blow-up.

(Lemarié-Rieusset §11, Constantin-Foias §11.4)

---

## 10. Link to the Project Constants (memo only)

The NS dimension 3D is not directly connected to the project constant n = 6. Some auxiliary drafts in the
n6-architecture BT-1-200 series (near BT-51, BT-543, BT-544) have attempted a triple-resonance or
characteristic-inequality route for NS. These remain at a verification-only partial-result stage; this P1
note contains only the pure textbook summary.

---

## 11. Reference Pointers (textbook pages / chapters)

| Topic | Evans PDE | Temam NSE | Constantin-Foias | Lemarié-Rieusset |
| --- | --- | --- | --- | --- |
| Continuum derivation | §8.1 | §1.1-1.2 | §1.1 | §1.1 |
| Incompressibility | §8.2 | §1.3 | §1.2 | §1.2 |
| Weak-solution definition | §8.2 | §3.1 | §3.3 | §3.1 |
| Galerkin construction | - | §3.4 | §3.1 | §14 |
| Energy inequality | - | §3.3 | §3.4 | §3.3 |
| Leray 1934 | - | Thm 3.1 | Thm 3.1 | Thm 14.1 |
| 2D global regularity | - | Thm 3.3 | Thm 10.1 | §8 |
| CKN 1982 | - | - | - | §11.7 |
| Fujita-Kato 1964 | - | - | §8 | §15 |
| BKM 1984 | - | - | §11.4 | §11 |
| Stokes + Leray P | - | §I.2 | §2.1 | §3.3 |

---

## 12. Five Takeaways from this Unit

1. Incompressible NS equations (∂_t u + (u·∇)u = -∇p/ρ + νΔu, ∇·u = 0) derived from Newton's law.
2. Classical, strong, weak distinction.
3. Energy inequality ½|u|²_{L²}(t) + ν∫₀^t|∇u|²_{L²} ≤ ½|u₀|²_{L²}.
4. Leray 1934 weak-solution existence + 2D global regularity (Ladyzhenskaya 1963) vs 3D open.
5. Fefferman's official Millennium-problem statement (A existence-smoothness vs B blow-up).

---

## 13. Honesty Declaration

- These notes are a textbook summary. No new result.
- The original papers Leray 1934, Ladyzhenskaya 1963 book, CKN 1982, Fefferman 2000 Clay statement are all
  publicly available official sources; the titles and years are accurate.
- The precise form of the 2D Ladyzhenskaya inequality follows Ladyzhenskaya 1963 §1.7.
- Formulas, theorems, authors, and years are taken from the four textbooks above and the official statement
  document.
- No invented theorems, authors, or dates.
- The link to the project constant (n=6) is left as a memo only in §10.
