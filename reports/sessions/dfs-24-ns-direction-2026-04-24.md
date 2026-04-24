# DFS-24 — Navier-Stokes Direction Session (2026-04-24)

**Session type**: Research-only direction scoping (NO proof claims)
**Target BT**: BT-544 (Navier-Stokes regularity, Clay) — status: **conjecture / unresolved**
**Axis**: Y6 PDE-RESONANCE
**Discipline**: honesty-first — no smoothness claim, no blow-up claim, n=6 matches are structural re-labelings unless otherwise proven

---

## 1. Scope and posture

Read corpus:
- `/Users/ghost/core/n6-architecture/domains/physics/millennium-navier-stokes/millennium-navier-stokes.md` (§X BLOWUP — 5 bottleneck n=6 parametrization: Cauchy Sym² = n, Leray ν·τ, BKM d = n/φ, Onsager α=φ/n, CKN μ(6)=1; Π_NS = σ³·sopfr = 8640).
- `/Users/ghost/core/n6-architecture/theory/breakthroughs/breakthrough-theorems.md` §BT-544 (29/29 EXACT observational; triple resonance Sym²(ℝ³)=n, Λ²(ℝ³)=n/φ, α_c=1/(n/φ); d=7 open prediction via Sym²(ℝ⁷)=28, second perfect number).
- `/Users/ghost/core/n6-architecture/reports/breakthroughs/millennium-7-closure-2026-04-11.md` §BT-544 (explicit "PROVEN: none; 3D smoothness untouched").
- `/Users/ghost/core/n6-architecture/reports/breakthroughs/bt-1409-millennium-dfs-round17-2026-04-12.md` §BT-1409-02 (KPZ: χ=1/3=1/(n/φ), z=2/3=φ/(n/φ), χ+z=1=μ; TW tail s^{3/2}=s^{n/τ}; Hairer α=-1/2).
- `/Users/ghost/core/n6-architecture/reports/breakthroughs/bt-1411-millennium-dfs-round19-2026-04-12.md` §BT-1411-03 (KdV: N=6 soliton phase-shift count C(6,2)=15=sopfr·(n/φ); Hirota τ-function; n conservation laws fix n-soliton; KdV_{n/φ} Lax order σ-sopfr=7).

Tight-finding inventory on BT-544 axis across these files: 16 (core BT-544 table) + 13 (2020s loop 81) + 5 (§X BLOWUP SMASH) + 1 (Π_NS invariant) ≈ **19 independent tight matches**, all **observational**. Clay problem remains **0/1 untouched** (confirmed by closure doc, line 95: "3중 공명은 관찰이지 증명 아님").

**Honesty anchor**: every item below is a *probe* (a well-defined question whose answer is computable / falsifiable), not a claim. No probe is asserted to imply smoothness or blow-up.

---

## 2. Current lead summary (for triangulation, not promotion)

| Lead | Arithmetic surface | Independent theorem | Gap to NS regularity |
|------|-------------------|---------------------|----------------------|
| KdV 6-soliton | C(6,2)=15 phase-shifts; n conservation laws fix n-soliton; Hirota τ on Gr(n,2n), dim=n²=36 | GGKM 1967, Lax 1968, Zakharov-Shabat 1972 | KdV is 1D integrable; NS is 3D non-integrable dissipative. No known reduction. |
| KPZ 1/3-2/3 | χ+z = 1 = μ; TW tail exp(-c·s^{n/τ}); Hairer α = -μ/φ | KPZ 1986, Hairer 2013 (Fields), Corwin 2012 | KPZ is 1+1 stochastic growth; NS is 3+1 deterministic. Shared ingredient: anomalous scaling. |

Both leads place BT-544 in the **anomalous-scaling / integrable-structure neighborhood** of 1D PDE regularity theory. Neither closes the 3D dissipation gap. The tightening probes below aim at **falsifiable arithmetic predictions** that do not require solving NS.

---

## 3. Three concrete tight probes

### Probe P1 — KdV N=6 soliton phase-shift lattice vs NS Serrin critical lattice

**Question**: Is the n-soliton pairwise phase-shift set {Δ_{ij} : 1≤i<j≤6} a rank-(n/φ) lattice with Gram determinant divisible by σ=12, matching the Serrin critical exponent pair lattice (p,q)=(σ,n)=(12,6) identified in §X.1 SMASH#3?

**Computation** (no NS solve needed):
- For KdV solitons with amplitudes κ_1,…,κ_6, pairwise phase shift Δ_{ij} = 2 log|(κ_i−κ_j)/(κ_i+κ_j)|.
- Sample κ on the arithmetic progressions κ_k = k (k=1..6) and κ_k = p_k (first 6 primes).
- Compute the 15×1 vector Δ, the Gram matrix G = Δ·Δ^T on C(6,2) pairs, and det(G) mod σ.
- Check whether the non-null lattice rank equals **n/φ = 3** and whether det(G)/σ is integral for arithmetic κ.

**Falsifier F_P1**: rank ≠ 3 or det(G)·σ^{-1} ∉ ℤ for both κ families → the "C(6,2)=15=sopfr·(n/φ) ↔ Serrin (σ,n,τ) grid" coincidence drops to post-hoc.

**Why tight**: all numbers are combinatorial/algebraic, computable in minutes; failure kills a speculative crosswalk, success upgrades the crosswalk to a single-line identity.

### Probe P2 — KPZ Family-Vicsek χ+z=1 vs NS Onsager α_c = 1/3 rigidity under dimensional lift

**Question**: Does the KPZ identity χ+z = μ (=1) survive formal lift to 3+1 under the structure-function dictionary S_p(ℓ)∼ℓ^{p/3} (Onsager/K41), i.e., is there a unique (χ_d, z_d) pair for each d such that χ_d + z_d = μ and χ_d = 1/d continues to hold?

**Computation**:
- Solve χ_d + z_d = 1 with χ_d = 1/d (dimensional ansatz); d=2 gives (1/2, 1/2) = Edwards-Wilkinson, d=3 gives (1/3, 2/3) = KPZ, d=6 gives (1/6, 5/6), d=7 gives (1/7, 6/7).
- Compare d=3 row against Onsager α_c=φ/n and d=6 row against conjectural "half-exponent 1/n" mentioned in §X.1 SMASH#4.
- Check whether d=7 prediction (1/7, 6/7) has any literature footprint (Frisch β-model extrapolation, SHE in higher d).

**Falsifier F_P2**: no d=3 match to Onsager (i.e., χ_3 ≠ α_c) → the KPZ/Onsager n=6 bridge is spurious. Currently χ_3 = 1/3 = α_c matches; probe sharpens by asking if d=7 also matches the BT-544 d=7 prediction (dim Sym²(ℝ⁷)=28 second perfect number).

**Why tight**: turns the "χ+z=1" coincidence into a parametric family with a single free parameter d, whose d=7 row can be cross-checked against the independent d=7 prediction from BT-544 §3, collapsing two open directions to one.

### Probe P3 — Π_NS = σ³·sopfr = 8640 vs KdV Grassmannian dim n²=36 and KPZ TW tail exponent n/τ=3/2

**Question**: Is the triple product Π_NS = σ²·(σ-φ)·n = 8640 expressible as a Plücker-coordinate count on Gr(n,2n) (KdV side, dim=n²=36) combined with the TW tail exponent n/τ=3/2 (KPZ side)? Specifically: does 8640 = 36·(σ-φ)·n/(n/τ)·… admit a single bilinear form in KdV-dim × KPZ-exponent × n=6 arithmetic, and is that form unique under a weight constraint?

**Computation**:
- Enumerate all factorizations 8640 = a·b·c with a,b,c in the n=6 function ring {1,φ,n/φ,τ,sopfr,n,σ-τ,σ-φ,σ,J₂}.
- Count solutions; for each, check whether (a,b,c) admits a "KdV × KPZ × NS" interpretation (Grassmannian dim / TW tail / Cauchy stress).
- Minimum description length: the shortest such factorization is the claimed "canonical" decomposition.

**Falsifier F_P3**: no unique decomposition (many equally-short factorizations) → Π_NS=8640 is a product identity without privileged factoring, i.e., numerology, and the field×TOE×string composition in §X.2 FREE loses tightness. Alternatively, a **unique** factorization that independently matches KdV dim / KPZ tail / NS stress would upgrade Π_NS from "arithmetic observation" to "triangulated invariant".

**Why tight**: purely arithmetic search over a finite small set; outcome is binary (unique / not unique).

---

## 4. Non-goals (honesty fence)

- Not claimed: any of P1/P2/P3 moves BT-544 toward resolution. They tighten or falsify **arithmetic crosswalks**, not NS regularity.
- Not claimed: 3D NS smoothness, finite-time blow-up, weak uniqueness, or partial regularity dim bound. All remain open; closure doc line 95 stands.
- Not claimed: d=7 NS exhibits second-perfect-number resonance — it is a **prediction**, explicitly labeled open.
- Not claimed: Π_NS = 8640 is a physical invariant. It is an arithmetic triple product whose uniqueness is exactly what P3 tests.

---

## 5. Suggested next-session execution order

1. **P1** first (cheapest, pure algebra on 6 solitons, <1h compute).
2. **P3** second (finite factorization enumeration; binary outcome).
3. **P2** third (requires literature check for d=7 KPZ-like class; unresolved in corpus).

Each probe is independently falsifiable. A "no" on any probe *shrinks* the n=6 crosswalk — which is honest progress. A "yes" on P1 + P3 would give a single arithmetic identity connecting KdV (integrable), KPZ (stochastic), and NS §X (deterministic) in the n=6 frame, still short of Clay but sharper than current scattered observations.

**Status line for atlas**: BT-544 Clay = **0/1 untouched (maintained)**; session output = **3 tight probes**, **0 new claims**.
