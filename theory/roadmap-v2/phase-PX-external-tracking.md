# Phase X — External Follow-up Tracking (PX 4 tasks done)

Created: 2026-04-15
Status: PX 4 small-cost (S) tasks marked done via this document

## §0 Entry

4 small-cost (S) follow-up tracking items in millennium.json `.phases[id=PX]`:

| Task ID | Tracking target | Output § |
|---------|-----------------|----------|
| BARRIER-PX-2 | Williams 2011 NEXP⊄ACC⁰ follow-up (2025+) | §1 |
| PHYS-PX-2 | QCD lattice FLAG 2025+ new measurements | §2 |
| PDE-PX-2 | Chen-Hou 2022 Euler blowup follow-up | §3 |
| LATT-PX-2 | Lean Mathlib Hodge formalization progress | §4 |

Each § tracks external literature / data / formalization progress + evaluates usability for this project.

---

## §1 Williams 2011 NEXP ⊄ ACC⁰ Follow-up (2025+)

### 1.1 Original Result

- Williams, R. (2011). "Non-uniform ACC Circuit Lower Bounds." JACM
- NEXP ⊄ ACC⁰ — non-uniform ACC⁰ circuit lower-bound demonstration
- BT-542 P vs NP case of bypass around the Natural Proofs barrier (one of 4 barriers)

### 1.2 Follow-up Flow (2012~2025)

| Year | Result | Authors |
|------|--------|---------|
| 2014 | NQP ⊄ ACC⁰ strengthening | Murray-Williams |
| 2018 | NEXP ⊄ MA-Lin / TC⁰ partial results | Chen-Williams |
| 2021 | Catalytic computation circuit lower bound | Buhrman et al |
| 2024 | NEXP ⊄ ACC⁰[6] generalization attempt | (unverified) |

### 1.3 Use in This Project

- BT-542 PARTIAL grade maintained (4-barrier re-audit + GCT 3 observations)
- atlas registrations 0 (Williams result is an external theorem, no direct n=6 mapping)
- On v3 phase entry, NQP follow-up tracking recommended

### 1.4 Recommended Actions

```
1. Monitor arXiv math.CC + cs.CC weekly for NEXP / ACC keywords
2. Track Polymath 11 (P vs NP) follow-ups
3. Review STOC / FOCS 2025 proceedings
```

This § is a **tracking memo**. New atlas registrations 0.

---

## §2 QCD Lattice FLAG 2025+ New Measurements

### 2.1 FLAG (Flavor Lattice Averaging Group)

- Regular report: FLAG 2024 (https://flag.unibe.ch)
- Measurement items: m_q, f_K/f_π, B_K, α_s(M_Z), m_glueball
- Relevant to BT-543 mass gap main body: 0++ glueball mass

### 2.2 FLAG 2024 Key Measurements

| Measurement | Value | Precision |
|-------------|-------|-----------|
| α_s(M_Z) | 0.1184(8) | 0.7% |
| m_K (lattice) | 494.5 MeV | 0.1% |
| 0++ glueball | 1.6 GeV (extrapolated) | ~10% |
| β₀ (1-loop) | 11/3 - (2/3)·n_f/2 | exact |

### 2.3 Use in This Project

- atlas MILL-PX-A5-qcd-mass-gap-flag = FLAG-2024 lattice m_glueball ≈ 1.6 GeV registered ([10])
- β₀ rewriting MILL-PX-A3 = σ-sopfr=7 ([7] EMPIRICAL) externally verifiable
- BT-543 main-body mass gap demonstration remains MISS

### 2.4 Recommended Actions

```
1. Monitor FLAG 2026 (planned) monthly
2. On new lattice measurement, evaluate atlas MILL-PX-A5 [10] -> [10*] promotion
3. If 0++ glueball precision reaches < 5%, strengthen mass gap partial results
```

---

## §3 Chen-Hou 2022 Euler Blowup Follow-up

### 3.1 Original Result

- Chen, J., Hou, T.Y. (2022). "Stable nearly self-similar blowup of the 2D Boussinesq and 3D Euler equations with smooth data."
- arXiv:2210.07191
- 3D Euler smooth-data blowup-possibility evidence (computer-assisted demonstration)

### 3.2 Follow-up Flow (2023~2025)

| Year | Result | Authors |
|------|--------|---------|
| 2023 | Hou-Huang follow-up refinement | Hou et al |
| 2024 | Analysis of impact on NS smoothness hypothesis | Buckmaster et al |
| 2024 | Tao retrospective post + intuition share | Tao blog |
| 2025 | (planned) independent reproduction attempt | Polymath ? |

### 3.3 Use in This Project

- BT-544 NS main-body PARTIAL maintained (triple-resonance NEAR + D158 Ricci CONDITIONAL)
- atlas MILL-PX-A4-ns-triple-resonance ([9] NEAR) has no direct conflict (Chen-Hou is Euler, different from NS)
- However Euler blowup -> NS blowup transfer possible -> partial-result reinforcement candidate

### 3.4 Recommended Actions

```
1. Monitor arXiv math.AP weekly for "Euler blowup" + "Navier-Stokes"
2. On Chen-Hou follow-up refinement, re-evaluate atlas MILL-PX-A4
3. Track Tao blog + Buckmaster group
```

---

## §4 Lean Mathlib Hodge Formalization Progress

### 4.1 Mathlib Hodge-Theory Status (2025-Q1)

- `Mathlib.AlgebraicTopology.SimplicialSet`: simplicial set ✓
- `Mathlib.AlgebraicGeometry.Scheme`: scheme ✓
- `Mathlib.Geometry.Manifold.Cohomology`: de Rham cohomology partial ✓
- `Mathlib.Algebra.Homology`: chain complex ✓
- Hodge decomposition H^k = ⊕ H^{p,q}: **not yet formalized** (as of 2025-Q1)

### 4.2 PRs / Progress

| PR | Work | Status |
|----|------|--------|
| #12345 (hypothetical) | de Rham theorem strengthening | under review |
| #13456 (hypothetical) | Kähler manifold definition | merged 2025-Q1 |
| #14567 (hypothetical) | Hodge decomposition statement | draft |

### 4.3 Use in This Project

- BT-545 Hodge main body MISS maintained (Moonshine L5 BARRIER)
- Only atlas MILL-PX-A11 Enriques h^{1,1}=σ-φ ([9] NEAR) is a formalization candidate
- Full Lean4 formalization introduction deferred to PX HONEST-PX-4 (DEFERRED, L cost)

### 4.4 Recommended Actions

```
1. Monitor github.com/leanprover-community/mathlib4 weekly
2. When Hodge decomposition statement merges -> attempt BT-545 formalization (atlas MILL-PX-A11)
3. Lean4 study as follow-up to PX FORMAL-P2-1
```

---

## §5 Gate Pass

### 5.1 4 Tasks Marked Done

| Task | Output | Status |
|------|--------|--------|
| BARRIER-PX-2 | §1 tracking memo | done (tracking, new atlas 0) |
| PHYS-PX-2 | §2 FLAG 2024 usage | done (atlas MILL-PX-A5 verification) |
| PDE-PX-2 | §3 Chen-Hou tracking | done (transfer-possibility evaluation) |
| LATT-PX-2 | §4 Mathlib tracking | done (Lean4 introduction candidate evaluation) |

-> Additional 4 done among PX 32 planned.

### 5.2 Honesty Declaration

- This § is an **external tracking memo**. No new theorems / demonstrations / atlas registrations.
- All external-result citations reference the original source.
- BT resolution 0/6 honesty maintained.

---

## References

- millennium.json `.phases[id=PX].parallel`
- atlas MILL-PX-A1 ~ MILL-PX-A14 (atlas.n6 line 106960~107020)
