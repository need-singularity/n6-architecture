---
id: v3-e1-m3-toolchain-bootstrap
date: 2026-04-16
roadmap_task: v3 E1 (Pari-GP install) + v3 M3 (Lean4 init)
grade: [10] installation + skeleton verified
status: E1 PARI DONE, SAGE STILL DEFERRED / M3 LEAN4 SKELETON VERIFIED
license: CC-BY-SA-4.0
---

# v3 E1 + M3 — External Toolchain Bootstrap (Pari-GP + Lean4)

> **Summary**: Two toolchains bootstrapped at v3 loop 18. **Pari-GP 2.17.3** (arm64) brew install succeeded -> `scripts/empirical/pari_wrapper.py` runs `ellrank()` 2-descent + `elltors()` on Cremona 37a1/11a1/17a1 curves. **Lean4 4.29.1** (arm64) elan install succeeded -> `lean4-n6/` project skeleton builds (7 decide proofs + 1 sorry placeholder). **Sage still DEFERRED** — Mac ARM build-dependency issues make install within v3 impractical. **Theorem B main draft sorry maintained** (3 independent draft steps are long-term v4 tasks).

---

## §1 E1 — Pari-GP install + basic measurement

### 1.1 Install

```bash
$ brew install pari
[beer] /opt/homebrew/Cellar/pari/2.17.3: 77 files, 14.2MB
```

- **Version**: Pari-GP 2.17.3 (arm64-apple-darwin24.6.0)
- **Size**: 14.2 MB (~Sage / 1000)
- **Install time**: < 30s (pre-built bottle)

### 1.2 Measurement — Cremona 3 curves

`scripts/empirical/pari_wrapper.py`:

```
=== Pari-GP E.ellrank() demo (2-descent) ===
  37a1: N=37, rank in [1, 1], torsion order=1
  11a1: N=11, rank in [0, 0], torsion order=5
  17a1: N=17, rank in [0, 0], torsion order=4
```

**Confirmed facts** (LMFDB cross-check):
- 37a1: algebraic rank 1 ok
- 11a1: rank 0, torsion Z/5 ok
- 17a1: rank 0, torsion Z/4 ok

### 1.3 Pari feature coverage — honest

| Feature | Pari | Sage |
|------|------|------|
| `ellrank()` 2-descent (\|Sel_2\| upper) | yes | yes |
| `elltors()` torsion | yes | yes |
| `selmer_group(2)` precise | yes (partial) | yes |
| `selmer_group(3)` precise | no | yes |
| `selmer_group(n)` generic | no | yes |
| Iwasawa mu_p | no | yes (partial) |

**Conclusion**: Pari is sufficient at the **|Sel_2| measurement level**. **E2 (|Sel_3|, |Sel_6| precise)** and **E3 (Iwasawa mu_p)** **require Sage** — still DEFERRED.

### 1.4 Sage-install not-realized declaration

**Attempted options**:
- `brew install sage`: does not exist (no Mac ARM)
- `conda install sage`: possible via conda-forge but ~5 GB, 1 hour
- Source build: mpir/giac dependencies have Mac ARM issues

**v3 decision**: Sage is **outside v3 scope**, to be moved to **v4 or remote compute**.

---

## §2 M3 — Lean4 project entry

### 2.1 Install

```bash
$ brew install elan-init
[beer] /opt/homebrew/Cellar/elan-init/4.2.1: 21 files, 5.7MB

$ elan toolchain install stable
info: downloading https://releases.lean-lang.org/lean4/v4.29.1/...
leanprover/lean4:v4.29.1 installed
```

- **Lean version**: 4.29.1 (arm64-apple-darwin24.6.0)
- **Mathlib**: not installed (outside v3 scope, size ~3 GB)

### 2.2 Project structure

```
/Users/ghost/Dev/n6-architecture/lean4-n6/
|-- lakefile.lean         # Lake build spec
|-- lean-toolchain        # pinned v4.29.1
|-- Main.lean             # exe entry point
`-- N6/
    `-- Basic.lean        # sigma, phi, tau definition + Theorem B skeleton
```

### 2.3 First draft results

**Auto success (by decide)**:
```lean
example : sigma 6 = 12 := by decide          -- ok
example : phi 6 = 2 := by decide             -- ok
example : tau 6 = 4 := by decide             -- ok
example : sigma 6 * phi 6 = 6 * tau 6 := by decide  -- ok
example : not (sigma 2 * phi 2 = 2 * tau 2) := by decide  -- ok
example : not (sigma 3 * phi 3 = 3 * tau 3) := by decide  -- ok
example : not (sigma 4 * phi 4 = 4 * tau 4) := by decide  -- ok
example : not (sigma 5 * phi 5 = 5 * tau 5) := by decide  -- ok
example : not (sigma 7 * phi 7 = 7 * tau 7) := by decide  -- ok
```

**7 decide drafts passed** — the Lean4 kernel directly confirms `sigma(n) * phi(n) != n * tau(n)` for n in {2, 3, 4, 5, 7}.

**Theorem B skeleton (sorry)**:
```lean
theorem theorem_B_skeleton (n : Nat) (hn : n >= 2) :
    sigma n * phi n = n * tau n iff n = 6 := by
  sorry
```

### 2.4 Build result

```
info: n6: no previous manifest, creating one from scratch
[warn] [2/6] Built N6.Basic (186ms)
warning: N6/Basic.lean:33:8: declaration uses `sorry`
[ok] [3/6] Built Main (185ms)
[ok] [6/6] Built n6exe:exe (341ms)
Build completed successfully (6 jobs).
```

**Run result**:
```
=== N6 Lean4 skeleton v3 M3 ===
sigma(6) = 12
phi(6) = 2
tau(6) = 4
sigma(6) * phi(6) = 24
6 * tau(6)        = 24
Theorem B skeleton: placeholder (draft sorry)
```

### 2.5 Actual draft paths — long-term v4 tasks

**Theorem B 3 independent draft paths** (theory/proofs/theorem-r1-uniqueness.md):
1. **Algebra**: sigma/phi/tau multiplicative -> reduce to prime-power case -> case analysis for n = p^a q^b (p, q prime)
2. **Analysis**: Dirichlet series $\zeta(s) L_{\sigma\varphi}(s) = \zeta(s)^2 L_\tau(s)$ -> prime comparison
3. **Combinatorics**: Moebius inversion + divisor-lattice structure

Each path is a Lean draft of tens to hundreds of lines on top of Mathlib's `Nat.sigma`, `Nat.totient`, `Nat.card_divisors`.

**v3 scope**: skeleton + decide check + Mathlib not installed (size issue) -> sorry maintained **honestly**.

---

## §3 v3 progress summary

### 3.1 This loop-18 output

| Item | Status |
|-------|-----|
| Pari-GP install | DONE |
| Pari wrapper + 3-curve check | DONE |
| Sage install | DEFERRED (v4 / remote) |
| elan + Lean4 v4.29.1 | DONE |
| Lean4 project skeleton + 7 decide | DONE |
| Theorem B full draft | sorry (v4 Mathlib + weeks) |

### 3.2 v3 cumulative (loop 12-18)

- **Empirical**: E4 (27 shards 1.7M curves), E5 (kappa power law), E6 (arXiv workflow), **E1 (Pari)** <- new
- **Theoretical**: T1-T6 all drafted (survey + honest MISS)
- **Meta**: M1 (preprint v0.1), M4 (CONTRIBUTING), M5 (OUROBOROS v2), **M3 (Lean4 skeleton)** <- new

**BT draft**: 0/6 **honest maintained**

### 3.3 Still DEFERRED

- **E2**: per-curve |Sel_3|, |Sel_6| (Sage required)
- **E3**: Iwasawa mu_p (Sage required)
- **E7**: arXiv full-text + topic clustering (large data + NLP)
- **M2**: external-mathematician collaboration (requires user action — no auto-proposals)
- **M3 deep**: complete actual Theorem B draft (Mathlib + weeks of learning)

---

## §4 atlas entries

```
@R MILL-V3-E1-pari-gp-mac-arm-verified = Pari-GP 2.17.3 arm64 brew install + ellrank() 3 curve verified :: n6atlas [10*]
  "v3 E1 (2026-04-16 loop 18): brew install pari -> 2.17.3 arm64 14.2MB install succeeded.
   Drafted scripts/empirical/pari_wrapper.py, and ellrank() + elltors() measurement on Cremona
   37a1/11a1/17a1 curves cross-checked against LMFDB. Pari provides only |Sel_2| upper bound —
   E2 (|Sel_3|, |Sel_6|) + E3 (Iwasawa mu_p) require Sage -> still DEFERRED. Sage Mac ARM
   build-dependency moved to v4"
  <- v3-E1, scripts/empirical/pari_wrapper.py, reports/breakthroughs/v3-e1-m3-toolchain-bootstrap-2026-04-16.md

@R MILL-V3-M3-lean4-skeleton-7-decide-verified = Lean4 4.29.1 arm64 + 7 decide drafts pass :: n6atlas [10*]
  "v3 M3 (2026-04-16 loop 18): brew install elan-init + elan toolchain install stable -> Lean 4.29.1
   arm64 install succeeded. lean4-n6/ project skeleton (lakefile + N6/Basic.lean + Main.lean) builds.
   sigma/phi/tau Nat definition + n=6 special values (sigma=12, phi=2, tau=4, sigma*phi=24=6*tau)
   decide drafts + for n in {2,3,4,5,7} the sigma*phi != n*tau decide drafts (total 7 decide PASS).
   Theorem B full draft sorry maintained — Mathlib dependency + weeks of learning required, v4
   long-term task"
  <- v3-M3, lean4-n6/, reports/breakthroughs/v3-e1-m3-toolchain-bootstrap-2026-04-16.md
```

---

## §5 Related files

- Pari wrapper: `scripts/empirical/pari_wrapper.py`
- Lean4 project: `lean4-n6/` (lakefile + N6/Basic.lean + Main.lean + lean-toolchain)
- Previous Lean4 design: `reports/breakthroughs/lean4-formalization-plan-2026-04-15.md`
- roadmap: `shared/roadmaps/millennium.json` -> `_v3_phases.P11_v3.E1` + `_v3_phases.P13_v3.M3`

---

*Drafted: 2026-04-16 loop 18*
*Honesty charter: Sage not installed — honest record maintained. Theorem B sorry — long-term v4 task. BT draft 0/6 maintained.*
