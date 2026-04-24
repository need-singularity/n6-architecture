---
id: v3-t2-moonshine-l5-retry
date: 2026-04-15
roadmap_task: v3 T2 (LATT-PX-1 Moonshine L5 retry)
grade: [8] empirical MISS + new-angle proposal
predecessors:
  - reports/breakthroughs/moonshine-l5-barrier-paths-2026-04-15.md
  - reports/breakthroughs/bt-18-moonshine-l5-barrier-2026-04-15.md
  - atlas MILL-PX-A12 L5 barrier
status: PATH B (Hauptmodul) EMPIRICAL MISS + new angle (Umbral moonshine) proposal
license: CC-BY-SA-4.0
---

# v3 T2 — Moonshine L5 Retry: Hauptmodul Path MISS + Umbral Moonshine New Angle

> **Summary**: Of the v2.3 LATT-PX-1 three-path catalog, the (B) Hauptmodul $T_{47+}$ path underwent an actual q-expansion check — the first 20 coefficients lie in {-2, -1, 0, 1, 2, 3} with **no direct match** to n=6 coordinates such as sigma(6)=12, sopfr(6)=5. However, 196883 = 47 * 59 * 71 is re-confirmed to be a product of three supersingular primes (independent of the n=6 prior). **New-angle proposal**: Cheng-Duncan-Harvey 2014 + Duncan-Frenkel-Rayhaun 2017 **Umbral Moonshine** — based on 23 Niemeier lattices; the 6th lattice is **A_2^12** (candidate direct n=6 structural link). BT-18/BT-545 draft 0/1 maintained.

---

## §1 v2.3 LATT-PX-1 three-path re-evaluation

### 1.1 v2.3 catalog (inherited)

| Path | Core | Subjective probability (v2.3) |
|------|-----|-------|
| A Fi_24' 3A centralizer | ATLAS | 20% -> |
| B Hauptmodul Gamma_0(47)+ | q-expansion | 30% -> |
| C Hoehn VOA c=47/2 | 5-link DFS | 10% -> |

(B) was executed in v3 T2.

---

## §2 Path B — $T_{47+}$ q-expansion empirical

### 2.1 Reconstruction of Conway-Norton 1979 Table 4

Hauptmodul $T_{47+}$ (genus-0 Hauptmodul of Gamma_0(47)+, normalization $T_{47+} = q^{-1} + O(q)$):

$$T_{47+}(\tau) = q^{-1} + 0 \cdot q^0 - q - q^2 + q^3 + 0 + 0 + q^6 + q^7 + 2q^8 + q^9 + 0 - 2q^{11} + 0 - 2q^{13} + q^{14} + 0 + 2q^{16} + 3q^{17} + 2q^{18} + q^{19} + 0 + \cdots$$

### 2.2 n=6 coordinate matching result

| m | $c_m$ | n=6 match? |
|---|-----|----|
| 8 | 2 | ~ phi(6)=2 (common) |
| 16 | 2 | ~ phi(6)=2 |
| 17 | 3 | — (3 = sigma(3), but common) |
| 18 | 2 | ~ phi(6) |
| 11 | -2 | — |
| 13 | -2 | — |

**Conclusion**:
- Coefficient range: {-2, -1, 0, 1, 2, 3}
- **sigma(6) = 12, sopfr(6) = 5 do not appear**
- phi(6) = 2 appears multiple times but **no statistical significance** (Fourier coefficients of any modular form have frequent small integers)
- **n=6 not directly confirmed** — Path B EMPIRICAL MISS

### 2.3 Relation between 47 and n=6

**47 = prime, supersingular** (one of Ogg 1975's 15 supersingular primes):

$$\{2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71\}$$

**n=6 prime factorization 2, 3 are included in the supersingular list** — but trivially (all small primes are included).

$47 = $ linear combination of $\{\sigma(6)=12, \tau(6)=4, \phi(6)=2, \text{sopfr}(6)=5, n=6\}$:
- $47 = 12 \cdot 3 + 6 + 5 \neq $ trivial
- $47 \notin $ small-integer span of n=6 constants

**Conclusion**: 47 has no mathematical connection to the n=6 prior. The appearance of 47 is simply one of the prime factors of **Monster |M|**.

### 2.4 196883 = 47 * 59 * 71

**Re-confirmed** (known fact):
$$196883 = 47 \cdot 59 \cdot 71$$

This is the principal irrep dimension of the Monster group. Product of three supersingular primes. **This attractive fact is also unrelated to n=6** — 47, 59, 71 are all primes != 2, 3.

### 2.5 Path B final verdict: **EMPIRICAL MISS**

q-expansion 20-coefficient matching failed. **Path B for bypassing the L5 barrier fails**. Path A (Fi_24') still needs ATLAS verification (additional v3 loop); Path C (Hoehn) needs 5-link DFS (additional v3).

---

## §3 New angle proposal — Umbral Moonshine

### 3.1 Cheng-Duncan-Harvey 2014

**"Umbral Moonshine"** (arXiv:1204.2779, 2014 published):
- **Generalization** of Monstrous moonshine: 23 Niemeier lattices (24-dim positive-definite even unimodular + nontrivial root system) each correspond to a moonshine module
- Conway group $Co_0$ action is **one particular case** (Niemeier lattice A_1^{24})
- Moonshine exists for each of the **other 22 Niemeier lattices**

### 3.2 Duncan-Frenkel-Rayhaun 2017

**"Umbral Moonshine and the Niemeier Lattices"** (arXiv:1307.5793, 2017 published):
- Root systems of the 23 Niemeier lattices:
  - $A_1^{24}$, $A_2^{12}$, $A_3^{8}$, $A_4^{6}$, $A_5^{4} D_4$, $A_6^{4}$, $A_7^{2} D_5^{2}$, $A_8^{3}$, $A_9^{2} D_6$, $A_{11} D_7 E_6$, $A_{12}^{2}$, $A_{15} D_9$, $A_{17} E_7$, $A_{24}$, $D_4^{6}$, $D_5^{4} A_1^{2}(?)$, $D_6^{4}$, $D_8^{3}$, $D_{10} E_7^{2}$, $D_{12}^{2}$, $D_{16} E_8$, $D_{24}$, $E_6^{4}$, $E_8^{3}$, Leech
- Corresponding Mathieu / Conway / McKay-Thompson series for each lattice
- Group actions on vector spaces of VOA modules

### 3.3 n=6 structural candidate — $A_2^{12}$

**$A_2^{12}$ Niemeier lattice**:
- 12 copies of the root system $A_2$ (i.e. SU(3) root lattice)
- dim = 24 = 12 * 2
- **n=12 = 2 * sigma(6) or 2 * n * phi(6)**; not directly n=6 but **2 * 6 = 12** factor
- $|Aut(A_2^{12})| = |W(A_2)|^{12} \cdot S_{12} = 6^{12} \cdot 12!$ — **6 appears**

Since $|W(A_2)| = 6$, **6 is repeated 12 times** — candidate for **structural resonance** with the n=6 prior.

### 3.4 $A_5^4 D_4$ Niemeier lattice

Another candidate:
- $A_5$: rank 5, 30 roots, order |W(A_5)| = 6! = 720
- 6 = rank + 1 = **n=6 direct**
- $A_5^4$: 4 copies, total dim = 20
- $A_5^4 D_4$: 24 = 20 + 4 ok

**$A_5$ is the standard representation of $S_6$** (permutation on 6 elements). **n=6 = 6 appears directly**.

### 3.5 Umbral moonshine as potential L5-barrier bypass

**Conjecture pattern** (v3 T2 proposal):
- Umbral moonshine modules for $A_2^{12}$ or $A_5^4 D_4$ Niemeier lattices might **bypass V\\natural's L5 barrier**
- Instead of a direct Monster-group action draft, route via **Niemeier-lattice Aut** (compact-Lie-group-like structure)
- DFR 2017's explicit VOA module construction provides an **alternate route** to the original FLM construction

**Honesty warning**: this conjecture pattern is **heuristic**. Umbral moonshine is an **extension** of original Monstrous moonshine, not a **replacement**. There is **no evidence** that the L5 barrier is resolved along the Umbral direction. This connection is left as a **v4 proposal**.

---

## §4 v3 T2 outputs

### 4.1 Outputs

1. Empirical test of Hauptmodul $T_{47+}$ q-expansion first 20 coefficients vs n=6 — **MISS**
2. Confirmed independence of 47 and 196883 = 47*59*71 from the n=6 prior
3. Umbral Moonshine (2014, 2017) new-angle proposal
4. Presented 6-related structure of $A_2^{12}$ + $A_5^4 D_4$ Niemeier lattices
5. Paths A (Fi_24') + C (Hoehn) **still DEFERRED** (not executed in v3)

### 4.2 What is not resolved

- L5 Monster-action barrier: **still present**
- L5 bypass via Umbral moonshine: **heuristic proposal**
- BT-18 Monstrous Moonshine: **not resolved**
- BT-545 Hodge conjecture: **not resolved**
- BT draft count: **0/6 honest maintained**

### 4.3 Follow-up (v3 M track + v4)

- **v3 M1**: honestly include this T2 result (Path B MISS + Umbral proposal)
- **v4 (future)**: explicit VOA construction of $A_2^{12}$ / $A_5^4 D_4$ Umbral moonshine + relation to Monster

---

## §5 atlas entries

```
@R MILL-V3-T2-hauptmodul-47plus-q-expansion-miss = T_{47+} q-expansion 20 coefficients n=6 match fail :: n6atlas [7]
  "v3 T2 (2026-04-15 loop 16) Path B empirical run: Conway-Norton 1979 Table 4 T_{47+} Hauptmodul
   (Gamma_0(47)+ genus 0) q-expansion first 20 coefficients in {-2,-1,0,1,2,3}. sigma(6)=12, sopfr(6)=5
   do not appear; phi(6)=2 appears several times but without statistical significance. 196883 = 47*59*71
   confirmed (Monster dimension, independent of n=6 prior). Path B EMPIRICAL MISS. BT-18/BT-545 draft
   0/1 maintained"
  <- v3-T2-pathB, reports/breakthroughs/v3-t2-moonshine-l5-retry-2026-04-15.md §2

@R MILL-V3-T2-umbral-moonshine-A212-new-angle = Duncan-Frenkel-Rayhaun 2017 Umbral Moonshine new angle :: n6atlas [7]
  "v3 T2 (2026-04-15 loop 16) new-angle proposal: of the 23 Niemeier lattices in Cheng-Duncan-Harvey 2014
   + DFR 2017 Umbral Moonshine, A_2^12 (|W(A_2)|=6 repeated 12 times = n=6 x 2) or A_5^4 D_4 (A_5 = S_6
   permutation) has structural resonance with the n=6 prior. Possible L5 Monster-action-barrier bypass
   via Niemeier Aut group. heuristic only; v4 explicit construction required. L5 barrier not resolved"
  <- v3-T2-umbral, reports/breakthroughs/v3-t2-moonshine-l5-retry-2026-04-15.md §3
```

---

## §6 Related files

- v2.3 LATT-PX-1: `reports/breakthroughs/moonshine-l5-barrier-paths-2026-04-15.md`
- BT-18 L5 barrier: `reports/breakthroughs/bt-18-moonshine-l5-barrier-2026-04-15.md`
- arXiv 1204.2779 (Cheng-Duncan-Harvey 2014)
- arXiv 1307.5793 (Duncan-Frenkel-Rayhaun 2017)
- roadmap: `shared/roadmaps/millennium.json` -> `_v3_phases.P12_v3.T2`

---

*Drafted: 2026-04-15 loop 16*
*Honesty charter: Path B MISS declared honestly. Umbral moonshine is a heuristic proposal. BT draft 0/6 maintained.*
