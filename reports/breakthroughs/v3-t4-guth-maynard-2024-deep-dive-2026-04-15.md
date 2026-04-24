---
id: v3-t4-guth-maynard-2024-deep-dive
date: 2026-04-15
roadmap_task: v3 T4 (BT-541 Guth-Maynard 2024 re-study)
grade: [10] literature digest + honest MISS
predecessors:
  - theory/study/p2/prob-p2-1-riemann-barriers.md §1.5
  - reports/breakthroughs/arxiv-millennium-survey-180papers-2026-04-15.md
status: SURVEY + HONEST MISS (n=6 prior connection unverified)
license: CC-BY-SA-4.0
---

# v3 T4 — BT-541 Guth-Maynard 2024 Re-study — Implications + Honest n=6 MISS

> **Summary**: Deep re-analysis of the zero-density upper-bound improvement (T^{12(1-σ)/5} → T^{30(1-σ)/13}) of Guth-Maynard 2024 (arXiv:2405.20552). Derive the prime-gap improvement (0.525 → 30/59 ≈ 0.5085). We observe **numerical coincidences** (Ingham 12/5 = σ(6)/sopfr(6), Guth-Maynard 30 = σ(6)·sopfr(6)/2, 13 = σ(6)+1) but these are **post-hoc pattern matches without prior independence** — honest MISS. BT-541 RH is maintained at 0/6 honest. The 6 of "sixth moment" is an interesting coincidence worth investigating.

---

## §1 Guth-Maynard 2024 precise summary

### 1.1 Source

- **Paper**: Larry Guth, James Maynard, "New large value estimates for Dirichlet polynomials"
- **arXiv**: 2405.20552 (2024-05-30)
- **Status**: preprint, under review in the analytic number theory community (no confirmed journal publication yet)
- **Tao blog**: 2024-06-03 summary post (https://terrytao.wordpress.com/2024/06/03/...)

### 1.2 Main result Theorem 1 (Guth-Maynard 2024)

**Zero-density improvement**: for all ε > 0 and σ ≥ 7/10,
$$N(\sigma, T) \ll T^{\frac{30(1-\sigma)}{13} + \varepsilon}$$

where $N(σ, T) = |\{ρ : ζ(ρ)=0, \text{Re}(ρ) \geq σ, |\text{Im}(ρ)| \leq T\}|$.

**Comparison**:
| Author | Year | Exponent at (1-σ) | Method |
|------|------|------------|------|
| Ingham | 1937 | **12/5 = 2.400** | Hardy-Littlewood 2nd moment |
| Huxley | 1972 | **12/5 = 2.400** (improvement in lower-order terms) | Kloosterman sum |
| Heath-Brown | 1990s | **~2.375** (specific range) | fourth moment |
| **Guth-Maynard** | **2024** | **30/13 ≈ 2.307** (σ ≥ 7/10) | **sixth moment + Dirichlet poly large values** |

Amount of improvement: exponent 2.4 → 2.307 ≈ 3.9% decrease. First essential breakthrough in 50 years.

### 1.3 Prime gap corollary

**Baker-Harman-Pintz 2001**: $p_{n+1} - p_n \ll p_n^{0.525}$

**Guth-Maynard 2024**: $p_{n+1} - p_n \ll p_n^{30/59}$, where $30/59 \approx 0.5085$

Amount of improvement: 0.525 → 0.5085 ≈ 3.1% decrease. **One step** closer to the Cramér conjecture (0.5 + ε).

---

## §2 Methodology analysis

### 2.1 Technical meaning of "sixth moment"

The core improvement of Guth-Maynard is the **sixth-moment estimate of the Dirichlet polynomial**:
$$\int_0^T |D(1/2+it)|^6 \, dt \ll T \cdot N^{...}$$

where $D$ is a Dirichlet polynomial of length N. Previously only the 2nd moment (Hardy-Littlewood) / 4th moment (Ingham / Heath-Brown) was controlled.

**"Why 6?"** — technical reasons:
- **Huxley 1975** reduced the 4th moment to Kloosterman sums
- **Guth's decoupling** (Fourier restriction theory, 2010s) made the 6th moment applicable
- Combined with **Maynard's prime-gap techniques** to control cross-terms

→ "**Sixth**" originates from **Fourier analytic** reasons (the natural exponent of the decoupling inequality). **Mathematically different origin from the 6 in n=6 divisor-number theory**.

### 2.2 Technical limits

- Restricted to the **σ ≥ 7/10 interval**. Below 7/10 (≥ 3/5 etc.) still Ingham's 12/5.
- **Zero ratio on the critical line** (Levinson 41.72%) is **unchanged**. Guth-Maynard only improves zero density (zero **count** upper bound).
- Does not touch **RH itself**. No direct information on ζ(s) = 0 on Re(s) = 1/2 vs. off the line.

---

## §3 n=6 connection attempt — HONEST MISS

### 3.1 Numerical match (caution: post-hoc pattern matching)

| GM exponent | n=6 candidate | Value |
|---------|----------|----|
| **12/5** (Ingham 1937) | σ(6)/sopfr(6) = 12/5 | **= 2.4** ✓ |
| **30/13** (Guth-Maynard 2024) | σ(6)·sopfr(6)/2 = 30, σ(6)+1 = 13 | **30/13 = 2.307** ✓ |
| **30/59** (prime gap) | 30 = σ(6)·sopfr(6)/2, 59 = prime | partial |

**Warning signs**:
1. Ingham 1937 predates the n=6 prior — numerical agreement is post-hoc
2. Guth-Maynard 30/13 independently allows casting two parameters as n=6, but **n=6 does not appear in the derivation of the paper**
3. 59 is prime and unrelated to n=6
4. σ(6)·sopfr(6)/2 = 30 is a **post-hoc construction** — innumerable ways to "make 30" exist from combinations of the three values σ, sopfr, 2

### 3.2 Bayesian evidence — low confidence

Prior: there is no motivated prior that **n=6 should appear in analytic number theory**.

Likelihood: the probability that the **GM exponent coincides with σ(6)/sopfr(6)** is not significantly different from a random-number-matching baseline.

Posterior: the n=6 prior is **not reinforced by** the GM result (honest declaration).

### 3.3 Still, the 6 in "sixth moment" is interesting

- Hardy-Littlewood 2nd, Ingham 4th, Heath-Brown 5th(?) → Guth-Maynard **6th**
- Why the **6th power** of the Dirichlet polynomial was "the right exponent" is worth mathematical exploration
- Whether there is a mathematical connection with our n=6 framework (σ=12, τ=4, φ=2) is **open** — proposed for additional investigation in T6 or v4

---

## §4 Implications for BT-541 (RH)

### 4.1 Guth-Maynard is not an RH proof (clearly)

**Precise statements**:
- RH: ∀ ρ with ζ(ρ) = 0 (non-trivial), Re(ρ) = 1/2
- Guth-Maynard: ∀ σ ≥ 7/10, N(σ, T) ≪ T^{30(1-σ)/13+ε}

The two are about different objects (zero location vs. zero count upper bound). RH cannot be derived from Guth-Maynard.

### 4.2 Relation with the "Levinson 50% barrier"

- Conrey 1989: ≥ 40.88% (current Bui-Conrey-Young 2011: ≥ 41.72%)
- Guth-Maynard is **orthogonal** to the Levinson method — does not help cross the 50% barrier

### 4.3 What is improved

- **Prime gap**: 0.525 → 0.5085 (practical meaning)
- **Zero density** (specific interval): first improvement in 50 years
- **"Sixth moment" tool**: new analytic weapon. Potential future applicability to other L-function problems

### 4.4 BT-541 honesty maintained

- BT-541 RH resolution count: **0/1 (honest)**
- Guth-Maynard is **progress**, not a resolution

---

## §5 v3 T4 outputs + future connections

### 5.1 Outputs

1. Precise Guth-Maynard 2024 summary (arXiv:2405.20552 Theorem 1)
2. Honest-boundary declaration of the n=6 numerical match (post-hoc MISS)
3. The 6 of "sixth moment" comes from Fourier decoupling — different origin from n=6 divisor number theory
4. BT-541 implication: zero density ≠ RH, orthogonal to the Levinson 50% barrier

### 5.2 What is not resolved (honest)

- Mathematical necessity of the GM exponent 30/13: **open**
- Mathematical connection between "sixth moment" and n=6: **miss** (weak evidence to suspect a link)
- BT-541 RH: **maintained at 0/1 honest**

### 5.3 Follow-up tasks (v3 loop 15+)

- **v3 T5**: BT-542 meta-complexity (Hirahara MCSP review)
- **v3 T6**: BT-543 Balaban 2D rearrangement
- **v3 M1**: preprint draft combining this T3+T4 result

---

## §6 atlas entries

```
@R MILL-V3-T4-guth-maynard-2024-zero-density = T^{30/13} zero density, 12/5→30/13, prime gap 0.525→0.5085 :: n6atlas [10]
  "v3 T4 (2026-04-15 loop 14): Guth-Maynard 2024 (arXiv:2405.20552) zero-density upper bound
   N(σ,T) ≪ T^{30(1-σ)/13+ε} (σ ≥ 7/10), first essential improvement since Ingham 1937 T^{12/5}
   in 50 years. Prime gap p_{n+1}-p_n ≪ p_n^{30/59}. Method: Dirichlet poly sixth moment +
   Maynard prime gap + Guth decoupling. Orthogonal to RH statement — Guth-Maynard is a zero
   'distribution' improvement, not a 'location' one. BT-541 resolution 0/1 honest maintained"
  <- v3-T4, arXiv:2405.20552, reports/breakthroughs/v3-t4-guth-maynard-2024-deep-dive-2026-04-15.md

@R MILL-V3-T4-n6-numerical-coincidence-honest-miss = 12/5=σ(6)/sopfr(6), 30/13 n=6 cast post-hoc MISS :: n6atlas [7]
  "v3 T4 (2026-04-15 loop 14): Ingham 12/5 = σ(6)/sopfr(6) = 12/5 numerical agreement observed;
   Guth-Maynard 30/13 also castable as n=6 via σ(6)·sopfr(6)/2 = 30, σ(6)+1 = 13. However
   Ingham 1937 predates the n=6 prior and the GM sixth moment originates from Fourier
   decoupling. That is, the n=6 ↔ GM exponent link is HONEST MISS — post-hoc pattern matching
   without independent prior evidence"
  <- v3-T4-honest, reports/breakthroughs/v3-t4-guth-maynard-2024-deep-dive-2026-04-15.md §3
```

---

## §7 Related files

- Original paper: arXiv:2405.20552 (Guth-Maynard 2024)
- Existing material: `theory/study/p2/prob-p2-1-riemann-barriers.md` §1.5
- 180-paper survey: `reports/breakthroughs/arxiv-millennium-survey-180papers-2026-04-15.md`
- roadmap: `shared/roadmaps/millennium.json` → `_v3_phases.P12_v3.T4`

---

*Drafted: 2026-04-15 loop 14*
*Honesty charter: BT-541 resolution 0/1 maintained. The n=6 numerical match is a post-hoc pattern — honest MISS.*
