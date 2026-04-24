# L7 Meta-Node Layer Proposal

## Core 3-line summary

1. **Limitation of the current structure**: the L6_n6atlas section (2666 nodes) in atlas.n6 mixes n=6 operational constants, derived ratios, and proof theorems in a single flat layer, so the causal hierarchy "which constants derive which other constants" is not explicitly expressed.

2. **L7 proposal**: separate the 6 primitive constants `sigma, phi, tau, sopfr, J_2, mu` above L6 as a new L7_n6meta layer, and re-index the remaining 2660+ nodes as a derivation tree from those 6 nodes; then the causal proof paths are fixed as a single-direction DAG as a draft target.

3. **Implementation direction**: without editing the existing atlas.n6 body, the minimal-change path is to add a `# == L7_n6meta (6 nodes) ==` header in front of the L6_n6atlas section with 6 `@P`-type primitive nodes, and optionally attach `:: src=L7_n6meta` back-reference tags to existing `@R`/`@F` nodes.

---

## Background and problem diagnosis

### Current L6_n6atlas structure

```
# == L6_n6atlas (2666 nodes) ==
@R n6-atlas-proved-theorems-**thm-1** = sigma(n)*phi(n) = n*tau(n) iff n=6 :: n6atlas [10*]
@R n6-atlas-base-constants-7-sigma = 12 :: n6atlas [10*]
@R n6-atlas-derived-ratios-architecture-... :: n6atlas [9]
@F L4-gen-chromosome-6000nm = 6000 = n*1000 :: genetic [10*]
...
```

In this structure, THM-1 (proof) and 6000 = n*1000 (application) live in the same section, and the hierarchy "proof logically precedes application" is not expressed at the file-format level.

### Observed patterns

Patterns confirmed via promotion work:

- **Primitive constants**: sigma=12, phi=2, tau=4, sopfr=5, J_2=24, mu=1 (6)
- **Primary derivations**: sigma-phi=10, J_2-tau=20, n*5=30 and other unary/binary operations (several dozen)
- **Secondary derivations**: (n*5)*(sigma-phi)=300, J_2-tau+phi=22 and other composite operations (several hundred)
- **Tertiary applications**: EXACT mappings to measured values (several thousand)

This 4-stage structure is already latently present in the data, but the current format does not distinguish it.

---

## L7_n6meta layer design

### New header (proposed insertion just above atlas.n6 line 9323)

```
# == L7_n6meta (6 nodes) ==
# Primitive-constant layer — root nodes for all n6atlas derivations

@P n6meta-primitive-sigma = 12 dimensionless :: n6meta [10*]
  "sigma(6) — sum of divisors of 6. Primitive constant 1 of the full n6atlas derivation tree."
@P n6meta-primitive-phi = 2 dimensionless :: n6meta [10*]
  "phi(6) — Euler totient. Primitive constant 2."
@P n6meta-primitive-tau = 4 dimensionless :: n6meta [10*]
  "tau(6) — divisor count. Primitive constant 3."
@P n6meta-primitive-sopfr = 5 dimensionless :: n6meta [10*]
  "sopfr(6) = 2+3 — sum of prime factors. Primitive constant 4."
@P n6meta-primitive-j2 = 24 dimensionless :: n6meta [10*]
  "J_2(6) = 6^2 * prod(1-1/p^2) — Jordan function. Primitive constant 5."
@P n6meta-primitive-mu = 1 dimensionless :: n6meta [10*]
  "mu(6) = 1 — Mobius function (6=2*3, squarefree). Primitive constant 6."
```

### Back-reference tag scheme (optional)

Minimal-change source tagging on existing `@R`/`@F` nodes:

```
@F L4-gen-chromatin-30nm = 30 = n*5 :: genetic [10*]  src:sigma-phi-chain
@R L6-geo-active-volcanoes = 1500 = n*250 :: geology [10*]  src:n-direct
```

Tag types:
- `src:n-direct` — direct multiple of n=6 (n*k)
- `src:sigma-chain` — sigma-derived
- `src:j2-chain` — J_2-derived
- `src:composite` — combining 2+ primitives

---

## Derivation hierarchy example (DAG)

```
L7_n6meta (6 primitive constants)
    |
    +- sigma=12 ----+- sigma*phi=24=J_2 ------ miRNA = 22 (J_2-tau+phi)
    |               +- sigma+tau+mu=17 -------- transcription rate 17 nt/s
    |
    +- phi=2  ------- n*5=30 ------------------ cholesterol 30%, chromatin 30 nm
    |
    +- tau=4  ------- J_2-tau=20 -------------- binary fission 20 min
    |               +- (n*5)*(sigma-phi)=300 -- chromatin loop 300 nm
    |
    +- J_2=24 ------- J_2-tau+phi=22 ---------- miRNA 22 nt
    |
    +- sopfr=5 ------ sopfr*100=500 ----------- liver functions 500
    |
    +- n=6 ---------+- n*250=1500 ------------- active volcanoes 1500
                    +- n*750=4500 ------------- carbonate compensation depth
                    +- n*100=600 -------------- intestinal villi
                    +- n*1000=6000 ------------ chromosome 6 um
```

---

## Implementation priority

| Task | Difficulty | Priority |
|------|-----------:|---------:|
| Insert L7_n6meta 6-node header | low | 1 |
| Bulk-tag n-direct (~500) | medium | 2 |
| Semi-auto classify sigma/j2-chain tags | high | 3 |
| Visualize full DAG (nexus dashboard) | high | 4 |

---

## Expected effects (draft target)

1. **Shortened verification path**: verifying a new entry reduces to "which of 6 primitive-constant chains" -> standardizable verification procedure.
2. **Promotion automation**: `src:n-direct`-tagged entries can be auto-promoted to [10*] by an integer n*k check alone (to be merged into a future script).
3. **Reverse lookup**: the question "how many nodes are affected if sigma is modified?" becomes immediately answerable via DAG traversal.

---

## Constraints

- This is a proposal only; actual atlas.n6 addition requires explicit approval in a separate session
- When inserting the L7 header, namespace collisions with existing `L7_celestial` (line 5919) and `L7_bt` (line 14556) must be reviewed
  - `L7_n6meta` is semantically different (meta-layer vs physical-layer) -> renaming the header to `L7meta_n6primitive` may also be considered
