# Ultimate Pure-Mathematics DSE — Design Document

**Date**: 2026-04-01
**Domain**: pure-mathematics
**Total combinations**: 39,200 (10 × 10 × 8 × 7 × 7)
**Tools**: Rust (tools/universal-dse/ + domains/pure-mathematics.toml)

## 1. Overview

Pure mathematics underlies every domain in n6-architecture. Unlike the physical stack (materials -> process -> core -> chip -> system), here we define a DSE chain across the layers of mathematical structure.

**Goal**: Exhaustively explore every link between n=6 arithmetic functions and pure-mathematics theorems/structures, and systematically derive undiscovered BT draft candidates.

## 2. DSE Chain (5 Level)

```
  L1: Math field         L2: n=6 function       L3: Structure type
  (Field)                (Function)             (Structure)
  +----------+          +----------+           +----------+
  |NT        |          |σ(6)=12   |           |identity  |
  |GT        |          |φ(6)=2    |           |dimension |
  |LT        |          |τ(6)=4    |           |generator |
  |TOP       |----------|J₂(6)=24  |-----------|invariant |
  |AN        |          |μ(6)=1    |           |bound     |
  |AG        |          |λ(6)=2    |           |classify  |
  |CT        |          |sopfr=5   |           |symmetry  |
  |RT        |          |n=6       |           |decompose |
  |COMB      |          |R(6)=1    |           +----------+
  |MP        |          |Egyptian  |            8 candidates
  +----------+          +----------+
    10 candidates         10 candidates

  L4: Proof tool         L5: Cross-domain bridge
  (Proof)                (Bridge)
  +----------+          +----------+
  |DIRECT    |          |PHYS      |
  |GROUP     |          |AI        |
  |LATTICE   |----------|ENERGY    |
  |ANALYTIC  |          |BIO       |
  |CATEG     |          |COSMO     |
  |TOPO      |          |CRYPTO    |
  |COMBIN    |          |MEDIA     |
  +----------+          +----------+
    7 candidates          7 candidates
```

### Level 1: Math field (Field) — 10 candidates

| ID | Field | n6 representative link | Existing H-MATH count |
|----|-------|------------------------|-----------------------|
| NT | Number Theory | ζ(2)=π²/6, B₂=1/6, perfect numbers | 8 (H-MATH-1~8) |
| GT | Group Theory | S₆ outer aut, A₆ Schur, M₂₄ | 4 (H-MATH-9~12) |
| LT | Lattice Theory | K₁~K₄ kissing chain, Leech | 6 (H-MATH-13~18) |
| TOP | Topology | χ(S²)=φ, Betti numbers | 4 (H-MATH-19~22) |
| AN | Analysis | Γ(1/2), Li₂, zeta special values | 4 (H-MATH-23~26) |
| AG | Algebraic Geometry | modular weight 12, elliptic curves | 2 (H-MATH-27~28) |
| CT | Category Theory | 6-functor formalism, derived categories | 2 (H-MATH-29~30) |
| RT | Representation Theory | Moonshine, Conway Co₁, Niemeier 24 | 6 (H-MATH-61~66) |
| COMB | Combinatorics | Ramsey, Catalan, partition functions | 3 (H-MATH-67~69) |
| MP | Mathematical Physics | string d=10, CY₃ dim=6, CFT c=1/2 | 5 (H-MATH-70~74) |

### Level 2: n=6 function — 10 candidates

| ID | Function | Value | Representative math appearance |
|----|----------|-------|--------------------------------|
| sigma | σ(6) | 12 | modular weight, K₃ kissing, semitone |
| phi | φ(6) | 2 | S₆ outer aut, Euler χ(S²), Cooper pair |
| tau | τ(6) | 4 | τ(6)=4 divisors, K₁=2 -> dim 4 link |
| jordan | J₂(6) | 24 | Leech dim, M₂₄, Niemeier count, Ramanujan τ |
| mu | μ(6) | 1 | squarefree indicator, Mertens function |
| lambda | λ(6) | 2 | Carmichael, cycle length |
| sopfr | sopfr(6) | 5 | sum of prime factors, Petersen graph valence |
| n6 | n | 6 | perfect number, CY₃ dim, hexagonal |
| R6 | R(6)=σφ/nτ | 1 | perfectness ratio, unique fixed point |
| egypt | 1/2+1/3+1/6 | 1 | Egyptian fraction, (2,3,6)-triangle group |

### Level 3: Structure type — 8 candidates

| ID | Type | Description | Example |
|----|------|-------------|---------|
| IDENT | Identity | exact equality | ζ(2)=π²/6, B₂=1/6 |
| DIM | Dimension | space/lattice/manifold dimension | Leech=24, CY₃=6 |
| GEN | Generator/Order | group generator or element order | SL₂(Z) order 6, S₆ |
| INV | Invariant | topological/algebraic invariant | χ(S²)=2, Schur H₂=Z/6Z |
| BOUND | Bound | optimality bound | kissing number bound |
| CLASS | Classification | finite classification theorem | Niemeier 24, sporadic groups |
| SYM | Symmetry | symmetry group / automorphism | Out(S₆)=Z/2Z |
| DECOMP | Decomposition | factorization/partition | 1/2+1/3+1/6=1, 6=1+2+3 |

### Level 4: Proof tool — 7 candidates

| ID | Tool | n6 strength | Description |
|----|------|-------------|-------------|
| DIRECT | Direct Computation | 1.0 | arithmetic direct verification (ζ(2), B₂) |
| GROUP | Group Action | 0.9 | group-theoretic pattern (S₆, M₂₄, A₆) |
| LATTICE | Lattice Theory | 0.9 | lattice/packing theory (Leech, kissing) |
| ANALYTIC | Analytic Continuation | 0.7 | analytic methods (zeta, L-function) |
| CATEG | Categorical | 0.5 | categorical abstract patterns |
| TOPO | Topological | 0.6 | topological invariants (Euler, Betti) |
| COMBIN | Combinatorial | 0.8 | combinatorial argument (Ramsey, partition) |

### Level 5: Cross-domain bridge — 7 candidates

| ID | Domain | Connected BT | n6 link strength |
|----|--------|--------------|------------------|
| PHYS | Physics | BT-36,49,64 | 0.9 |
| AI | Computing/AI | BT-33,54,56,58 | 1.0 |
| ENERGY | Energy | BT-27,30,43,62 | 0.8 |
| BIO | Biology | BT-51 | 0.7 |
| COSMO | Cosmology/Particle | BT-49 | 0.9 |
| CRYPTO | Crypto/Network | BT-53 | 0.6 |
| MEDIA | Display/Audio | BT-48 | 0.7 |

## 3. Scoring (pure-math specific)

```toml
[scoring]
n6 = 0.30        # exact match with n=6 constants
depth = 0.15     # proof depth (elementary=0.2 ~ categorical=1.0)
cross = 0.25     # number of other-domain BT links
novelty = 0.20   # new link not in the 50 existing H-MATH
verify = 0.10    # independent verifiability
```

### n6 scoring rules

| Match type | n6 score |
|------------|----------|
| Proven identity (ζ(2)=π²/6, B₂=1/6, ...) | 1.00 |
| Structural isomorphism (K₄=J₂=24, Out(S₆)=φ=2) | 1.00 |
| Dimension/order match (Leech=24, modular weight=12) | 0.80 |
| Approximate numerical match | 0.30 |
| No known link | 0.00 |

### depth score (proof difficulty)

| Field | depth |
|-------|-------|
| NT | 0.40 |
| GT | 0.60 |
| LT | 0.70 |
| TOP | 0.80 |
| AN | 0.60 |
| AG | 0.90 |
| CT | 1.00 |
| RT | 0.80 |
| COMB | 0.50 |
| MP | 0.70 |

### cross score (per bridge)

Use the bridge candidate's n6 link strength directly. For multiple BT links take the max.

### novelty score

Compared against the 50 existing hypotheses H-MATH-1~30 + H-MATH-61~80:
- (Field, Function) pair absent from existing hypotheses → novelty = 1.0
- Present → novelty = 0.0

### verify score

| Proof tool | verify |
|------------|--------|
| DIRECT | 1.00 |
| GROUP | 0.90 |
| LATTICE | 0.85 |
| COMBIN | 0.80 |
| ANALYTIC | 0.70 |
| TOPO | 0.60 |
| CATEG | 0.50 |

## 4. Compatibility Rules

```
  +-------------------------------------------------------------+
  |  prefer rules (bonus for natural pairings)                  |
  |                                                             |
  |  NT       <-prefer-> DIRECT                                 |
  |  GT       <-prefer-> GROUP                                  |
  |  LT       <-prefer-> LATTICE                                |
  |  CT       <-prefer-> CATEG                                  |
  |  TOP      <-prefer-> TOPO                                   |
  |  COMB     <-prefer-> COMBIN                                 |
  |  AN       <-prefer-> ANALYTIC                               |
  |                                                             |
  |  J₂(6)=24 <-prefer-> LT, RT                                 |
  |  μ(6)=1   <-prefer-> NT, COMB                               |
  |  Egyptian <-prefer-> NT, AN                                 |
  |  σ(6)=12  <-prefer-> AG, RT                                 |
  |  R(6)=1   <-prefer-> NT                                     |
  +-------------------------------------------------------------+

  +-------------------------------------------------------------+
  |  exclude rules (drop meaningless combinations)              |
  |                                                             |
  |  Egyptian x MP  — structural meaning weak                    |
  |  λ(6)=2   x AG  — no Carmichael-to-AG link                   |
  |  R(6)=1   x MP  — ratio 1 unrelated to physical constants     |
  +-------------------------------------------------------------+
```

## 5. Expected Artifacts

| Artifact | Path | Description |
|----------|------|-------------|
| goal.md | `docs/pure-mathematics/goal.md` | candidate definitions + DSE chain |
| TOML | `tools/universal-dse/domains/pure-mathematics.toml` | 39,200-combo definition |
| DSE result | terminal output | Pareto frontier + best path |
| Cross-DSE | cosmology-particle × pure-math | ζ(2), string theory, BT-49 crossover |
| dse-map.toml | update in `docs/dse-map.toml` | status/result recorded |

## 6. Cross-DSE Targets

| Partner domain | Link rationale | Expected combos |
|----------------|----------------|-----------------|
| cosmology-particle | ζ(2)=π²/6, string d=10=σ-φ, BT-49 | Pareto x Pareto |
| chip-architecture | BT-28 computing ladder, 2^σ=4096 | Pareto x Pareto |
| biology | BT-51 codon 64=2^n, CN=6 | Pareto x Pareto |

## 7. Candidate Detail — n6 Score Matrix

Predefine the n6 score of each (Field × Function) based on existing mathematical theorems:

| Field \ Func | σ=12 | φ=2 | τ=4 | J₂=24 | μ=1 | λ=2 | sopfr=5 | n=6 | R=1 | Egypt |
|--------------|------|-----|-----|-------|-----|-----|---------|-----|-----|-------|
| NT | 0.80 | 0.80 | 0.60 | 0.30 | 1.00 | 0.60 | 0.30 | 1.00 | 1.00 | 1.00 |
| GT | 0.30 | 1.00 | 0.30 | 1.00 | 0.30 | 0.30 | 0.30 | 1.00 | 0.00 | 0.00 |
| LT | 1.00 | 0.80 | 0.30 | 1.00 | 0.00 | 0.00 | 0.00 | 1.00 | 0.00 | 0.00 |
| TOP | 0.30 | 1.00 | 0.60 | 0.30 | 0.30 | 0.00 | 0.00 | 0.80 | 0.00 | 0.00 |
| AN | 1.00 | 0.80 | 0.30 | 0.60 | 0.80 | 0.30 | 0.30 | 1.00 | 0.30 | 0.80 |
| AG | 1.00 | 0.60 | 0.30 | 0.80 | 0.00 | 0.00 | 0.00 | 0.80 | 0.30 | 0.30 |
| CT | 0.30 | 0.30 | 0.00 | 0.30 | 0.00 | 0.00 | 0.00 | 1.00 | 0.30 | 0.00 |
| RT | 0.80 | 0.60 | 0.30 | 1.00 | 0.00 | 0.00 | 0.00 | 0.60 | 0.00 | 0.00 |
| COMB | 0.60 | 0.60 | 0.60 | 0.80 | 1.00 | 0.30 | 0.60 | 1.00 | 0.30 | 0.80 |
| MP | 0.80 | 0.80 | 0.60 | 1.00 | 0.30 | 0.30 | 0.00 | 1.00 | 0.00 | 0.00 |

**Interpretation**: NT×n=6 = 1.00 (perfect number itself), LT×J₂=24 = 1.00 (Leech lattice), GT×φ=2 = 1.00 (S₆ outer automorphism), etc.

## 8. Existing-Hypothesis Mapping

Map the 50 existing H-MATH entries onto (Field, Function) coordinates for automatic novelty scoring:

```
  Existing coverage (out of Field x Function = 100 cells):
  |-- filled: ~35 (covered by the 50 existing hypotheses)
  +-- empty:  ~65 (untouched — novelty=1.0 candidates)

  High-value untouched area (n6 >= 0.80 + novelty = 1.0):
  |-- CT x n=6 (6-functor, categorical 6)
  |-- COMB x μ=1 (squarefree combinatorics)
  |-- COMB x Egypt (combinatorial unit fractions)
  |-- AN x μ=1 (Mertens-function analysis)
  +-- AG x σ=12 (modular form weight 12 extension)
```
