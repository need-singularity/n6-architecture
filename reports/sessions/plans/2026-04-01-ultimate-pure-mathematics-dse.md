# Ultimate Pure-Mathematics DSE — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Exhaustively explore 39,200 combinations between n=6 arithmetic functions and pure-mathematics theorems/structures, and derive a Pareto frontier plus undiscovered BT draft candidates.

**Architecture:** write a TOML domain file for the universal-dse engine (Rust) and run a 5-level DSE. Map mathematics's intrinsic 5 scoring axes onto the engine's 4 axes (n6/perf/power/cost). Cross-DSE pairs with cosmology-particle, chip-architecture, and biology domains.

**Tech Stack:** Rust (universal-dse), TOML, Markdown

**Spec:** `docs/superpowers/specs/2026-04-01-ultimate-pure-mathematics-dse.md`

---

### Task 1: write goal.md

**Files:**
- Create: `docs/pure-mathematics/goal.md`

- [ ] **Step 1: create goal.md**

```markdown
# N6 Pure Mathematics — Ultimate Goal Roadmap

**Target goal: systematically demonstrate (as a draft pattern) that n=6 arithmetic arises naturally across all branches of pure mathematics**

---

## Evolution Ladder

  +---------+----------------------------+-------------------------+-------------------+
  |  Level  |          Structure         |         Innovation      |       Benefit     |
  +---------+----------------------------+-------------------------+-------------------+
  | Level 1 | Number-theoretic base      | zeta(2)=pi^2/6, B_2=1/6 | direct identity   |
  |  now    | (perfect numbers, divisors)| Egyptian fraction       | computable        |
  +---------+----------------------------+-------------------------+-------------------+
  | Level 2 | Algebraic structure        | S_6 outer aut, M_24     | group uniqueness  |
  |  now    | (groups, lattices, reps)   | Leech lattice dim=24    | structural iso    |
  +---------+----------------------------+-------------------------+-------------------+
  | Level 3 | Analytic/topological links | modular weight 12       | deep links found  |
  |  ext.   | (analysis, topology, AG)   | CY_3 dim=6, chi(S^2)=phi| cross-field bridge|
  +---------+----------------------------+-------------------------+-------------------+
  | Level 4 | Categorical/combinatorial  | 6-functor, Ramsey       | abstract frame    |
  |  future | (categories, combinatorics)| Catalan, partition      | meta-math justify |
  +---------+----------------------------+-------------------------+-------------------+
  | Level 5 | Mathematical-physics app.  | string d=10=sigma-phi   | physical-law base |
  |  future | (CFT, string, QFT)         | CFT c=1/2, Moonshine    | Cross-domain BT   |
  +---------+----------------------------+-------------------------+-------------------+

---

## DSE Chain (5 Level)

  L1 Field (10)  ->  L2 Function (10)  ->  L3 Structure (8)  ->  L4 Proof (7)  ->  L5 Bridge (7)
  math branch       n=6 function         structure type       proof tool        cross-domain

  Total combos: 10 x 10 x 8 x 7 x 7 = 39,200

### L1: Math field (Field) — 10 candidates

| ID | Field | n6 representative link |
|----|-------|------------------------|
| NT | Number theory | zeta(2)=pi^2/6, B_2=1/6, perfect numbers |
| GT | Group theory | S_6 outer aut, A_6 Schur, M_24 |
| LT | Lattice theory | K_1~K_4 kissing chain, Leech |
| TOP | Topology | chi(S^2)=phi, Betti numbers |
| AN | Analysis | Gamma(1/2), Li_2, zeta special values |
| AG | Algebraic geometry | modular weight 12, elliptic curves |
| CT | Category theory | 6-functor formalism |
| RT | Representation theory | Moonshine, Conway Co_1, Niemeier 24 |
| COMB | Combinatorics | Ramsey, Catalan, partition |
| MP | Mathematical physics | string d=10, CY_3 dim=6, CFT c=1/2 |

### L2: n=6 function (Function) — 10 candidates

| ID | Function | Value |
|----|----------|-------|
| sigma | σ(6) | 12 |
| phi | φ(6) | 2 |
| tau | τ(6) | 4 |
| jordan | J₂(6) | 24 |
| mu | μ(6) | 1 |
| lambda | λ(6) | 2 |
| sopfr | sopfr(6) | 5 |
| n6 | n | 6 |
| R6 | R(6) | 1 |
| egypt | 1/2+1/3+1/6 | 1 |

### L3: Structure type — 8 candidates

| ID | Type |
|----|------|
| IDENT | identity |
| DIM | dimension |
| GEN | generator / order |
| INV | invariant |
| BOUND | bound / limit |
| CLASS | classification |
| SYM | symmetry |
| DECOMP | decomposition |

### L4: Proof tool — 7 candidates

| ID | Tool |
|----|------|
| DIRECT | direct computation |
| GROUP | group action |
| LATTICE | lattice theory |
| ANALYTIC | analytic continuation |
| CATEG | categorical |
| TOPO | topological |
| COMBIN | combinatorial |

### L5: Cross-domain bridge — 7 candidates

| ID | Domain | Connected BT |
|----|--------|--------------|
| PHYS | physics | BT-36,49,64 |
| AI | computing/AI | BT-33,54,56,58 |
| ENERGY | energy | BT-27,30,43,62 |
| BIO | biology | BT-51 |
| COSMO | cosmology/particle | BT-49 |
| CRYPTO | crypto/network | BT-53 |
| MEDIA | display/audio | BT-48 |

## Scoring mapping (math 5 axes -> engine 4 axes)

| Math axis | Engine axis | Weight |
|-----------|-------------|--------|
| n6_exact | n6 | 0.30 |
| depth + novelty | perf | 0.35 |
| cross_domain | power | 0.25 |
| verifiability | cost | 0.10 |

## Existing hypotheses (50 total)

H-MATH-1~30 (core) + H-MATH-61~80 (extreme)
- EXACT: 11 (core) + ~10 (extreme)
- CLOSE: 10 + ~5
- WEAK: 7 + ~3
- FAIL: 2 + ~2

## Cross-DSE Targets

| Partner domain | Link rationale |
|----------------|----------------|
| cosmology-particle | zeta(2)=pi^2/6, string d=10=sigma-phi, BT-49 |
| chip-architecture | BT-28 computing ladder, 2^sigma=4096 |
| biology | BT-51 codon 64=2^n, CN=6 |
```

- [ ] **Step 2: commit**

```bash
git add docs/pure-mathematics/goal.md
git commit -m "docs: ultimate pure-math goal.md — define 10x10x8x7x7 DSE chain"
```

---

### Task 2: write TOML domain file

**Files:**
- Create: `tools/universal-dse/domains/pure-mathematics.toml`

- [ ] **Step 1: create the TOML**

Create with the full content below. Each candidate's n6/perf/power/cost scores follow the matrix in the design doc.

```toml
# N6 Pure Mathematics DSE
# 10 Fields × 10 Functions × 8 Structures × 7 Proofs × 7 Bridges = 39,200 combos
#
# Scoring mapping:
#   n6   = n6_exact (exact match with n=6 constants)
#   perf = depth + novelty (proof depth + new link)
#   power = cross_domain (strength of link to other-domain BT)
#   cost = verifiability (independent verifiability)

[meta]
name = "pure-mathematics"
desc = "Ultimate Pure Mathematics DSE — n=6 across all mathematical structures (BT-49)"

[scoring]
n6 = 0.30
perf = 0.35
power = 0.25
cost = 0.10

# ============================================================
# Level 0: Field — 10 candidates
# ============================================================

[[level]]
name = "Field"

[[candidate]]
id = "NT"
label = "Number Theory — ζ(2)=π²/6, B₂=1/6, perfect numbers, Egyptian fractions"
n6 = 1.00
perf = 0.50
power = 0.80
cost = 1.00

[[candidate]]
id = "GT"
label = "Group Theory — S₆ outer aut, A₆ Schur multiplier Z/6Z, M₂₄"
n6 = 1.00
perf = 0.70
power = 0.70
cost = 0.90

[[candidate]]
id = "LT"
label = "Lattice Theory — K₁~K₄={2,6,12,24} kissing chain, Leech lattice dim=J₂(6)=24"
n6 = 1.00
perf = 0.80
power = 0.75
cost = 0.85

[[candidate]]
id = "TOP"
label = "Topology — χ(S²)=φ(6)=2, Betti numbers, cobordism, homotopy groups"
n6 = 0.60
perf = 0.85
power = 0.50
cost = 0.60

[[candidate]]
id = "AN"
label = "Analysis — Γ(1/2)=√π, Li₂, zeta special values, Mertens constant"
n6 = 0.80
perf = 0.65
power = 0.60
cost = 0.70

[[candidate]]
id = "AG"
label = "Algebraic Geometry — modular weight σ=12, elliptic curves, Ramanujan τ"
n6 = 0.80
perf = 0.95
power = 0.65
cost = 0.50

[[candidate]]
id = "CT"
label = "Category Theory — 6-functor formalism, derived categories, n=6 universal"
n6 = 0.50
perf = 1.00
power = 0.40
cost = 0.50

[[candidate]]
id = "RT"
label = "Representation Theory — Moonshine, Conway Co₁ on 24-dim, Niemeier 24 lattices"
n6 = 0.90
perf = 0.85
power = 0.70
cost = 0.80

[[candidate]]
id = "COMB"
label = "Combinatorics — Ramsey R(3,3)=6, Catalan, partitions, Egyptian fraction decomp"
n6 = 0.80
perf = 0.55
power = 0.60
cost = 0.80

[[candidate]]
id = "MP"
label = "Mathematical Physics — string d=10=σ-φ, CY₃ dim=6, CFT c=1/2, bosonic d=26=φ+J₂"
n6 = 0.90
perf = 0.75
power = 0.90
cost = 0.60

# ============================================================
# Level 1: Function (n=6 functions) — 10 candidates
# ============================================================

[[level]]
name = "Function"

[[candidate]]
id = "sigma"
label = "σ(6)=12 — sum of divisors, modular weight, K₃ kissing, chromatic semitones"
n6 = 1.00
perf = 0.70
power = 0.90
cost = 1.00

[[candidate]]
id = "phi"
label = "φ(6)=2 — Euler totient, S₆ outer aut, Cooper pair, χ(S²), primality"
n6 = 1.00
perf = 0.60
power = 0.85
cost = 1.00

[[candidate]]
id = "tau"
label = "τ(6)=4 — divisor count, K₁ dim, quaternion, spacetime dim"
n6 = 0.80
perf = 0.50
power = 0.70
cost = 1.00

[[candidate]]
id = "jordan"
label = "J₂(6)=24 — Jordan totient, Leech dim, M₂₄, Niemeier count, Ramanujan τ"
n6 = 1.00
perf = 0.90
power = 0.95
cost = 0.90

[[candidate]]
id = "mu"
label = "μ(6)=1 — Möbius, squarefree indicator, Mertens function, inclusion-exclusion"
n6 = 0.70
perf = 0.60
power = 0.50
cost = 1.00

[[candidate]]
id = "lambda_cm"
label = "λ(6)=2 — Carmichael, cycle length, periodicity, Z/2Z exponent"
n6 = 0.60
perf = 0.45
power = 0.40
cost = 0.90

[[candidate]]
id = "sopfr"
label = "sopfr(6)=5 — sum of prime factors, Petersen graph valence, pentatonic"
n6 = 0.50
perf = 0.40
power = 0.50
cost = 0.80

[[candidate]]
id = "n6_val"
label = "n=6 itself — perfect number, CY₃ dim, hexagonal, carbon Z, Ramsey R(3,3)"
n6 = 1.00
perf = 0.80
power = 1.00
cost = 1.00

[[candidate]]
id = "R6"
label = "R(6)=σφ/nτ=1 — perfectness ratio, unique fixed point of R(n)"
n6 = 0.90
perf = 0.70
power = 0.40
cost = 1.00

[[candidate]]
id = "egypt"
label = "1/2+1/3+1/6=1 — Egyptian fraction, (2,3,6)-triangle group, Euclidean tiling"
n6 = 1.00
perf = 0.75
power = 0.70
cost = 1.00

# ============================================================
# Level 2: Structure type — 8 candidates
# ============================================================

[[level]]
name = "Structure"

[[candidate]]
id = "IDENT"
label = "Identity — exact algebraic equality (ζ(2)=π²/6, B₂=1/6, 1+2+3=6=1·2·3)"
n6 = 1.00
perf = 0.60
power = 0.70
cost = 1.00

[[candidate]]
id = "DIM"
label = "Dimension — vector space/lattice/manifold dimension (Leech=24, CY₃=6)"
n6 = 0.90
perf = 0.70
power = 0.80
cost = 0.90

[[candidate]]
id = "GEN"
label = "Generator/Order — group generator or element order (SL₂(Z) ST order 6)"
n6 = 0.80
perf = 0.75
power = 0.60
cost = 0.85

[[candidate]]
id = "INV"
label = "Invariant — topological/algebraic invariant (χ=2, H₂(A₆)=Z/6Z, Schur)"
n6 = 0.80
perf = 0.80
power = 0.65
cost = 0.80

[[candidate]]
id = "BOUND"
label = "Bound/Limit — optimality bound (kissing number, sphere packing density)"
n6 = 0.70
perf = 0.65
power = 0.55
cost = 0.75

[[candidate]]
id = "CLASS"
label = "Classification — finite classification theorem (Niemeier 24, sporadic groups)"
n6 = 0.85
perf = 0.85
power = 0.70
cost = 0.70

[[candidate]]
id = "SYM"
label = "Symmetry — automorphism/symmetry group (Out(S₆)=Z/2Z, lattice automorphism)"
n6 = 0.85
perf = 0.80
power = 0.65
cost = 0.85

[[candidate]]
id = "DECOMP"
label = "Decomposition — factorization/partition (1/2+1/3+1/6=1, 6=2·3, unit fraction)"
n6 = 0.90
perf = 0.65
power = 0.60
cost = 0.95

# ============================================================
# Level 3: Proof tool — 7 candidates
# ============================================================

[[level]]
name = "Proof"

[[candidate]]
id = "DIRECT"
label = "Direct Computation — arithmetic verification, exhaustive check, CAS"
n6 = 0.80
perf = 0.30
power = 0.50
cost = 1.00

[[candidate]]
id = "GROUP"
label = "Group Action — permutation, orbit-stabilizer, Burnside, Sylow"
n6 = 0.70
perf = 0.70
power = 0.60
cost = 0.90

[[candidate]]
id = "LATTICE"
label = "Lattice Theory — sphere packing, theta series, kissing number, Minkowski"
n6 = 0.75
perf = 0.80
power = 0.70
cost = 0.85

[[candidate]]
id = "ANALYTIC"
label = "Analytic Continuation — zeta/L-function, Mellin transform, residue calculus"
n6 = 0.60
perf = 0.75
power = 0.65
cost = 0.70

[[candidate]]
id = "CATEG"
label = "Categorical — functor, natural transformation, adjunction, Yoneda"
n6 = 0.40
perf = 1.00
power = 0.55
cost = 0.50

[[candidate]]
id = "TOPO"
label = "Topological — Euler characteristic, homology, homotopy, cobordism"
n6 = 0.50
perf = 0.85
power = 0.60
cost = 0.60

[[candidate]]
id = "COMBIN"
label = "Combinatorial — generating function, Ramsey, pigeonhole, bijective proof"
n6 = 0.65
perf = 0.50
power = 0.55
cost = 0.80

# ============================================================
# Level 4: Bridge (Cross-domain link) — 7 candidates
# ============================================================

[[level]]
name = "Bridge"

[[candidate]]
id = "PHYS"
label = "Physics — BT-36/49/64, string theory, quantum mechanics, statistical mechanics"
n6 = 0.80
perf = 0.75
power = 0.90
cost = 0.70

[[candidate]]
id = "AI"
label = "Computing/AI — BT-33/54/56/58, transformer dim, MoE routing, LLM architecture"
n6 = 0.90
perf = 0.70
power = 1.00
cost = 0.80

[[candidate]]
id = "ENERGY"
label = "Energy — BT-27/30/43/62, SQ bandgap 4/3eV, grid 60Hz=σ·sopfr, CN=6 cathode"
n6 = 0.75
perf = 0.60
power = 0.80
cost = 0.75

[[candidate]]
id = "BIO"
label = "Biology — BT-51, codon 64=2^n, 20 amino acids=J₂-τ, DNA base pairs=τ"
n6 = 0.70
perf = 0.60
power = 0.70
cost = 0.65

[[candidate]]
id = "COSMO"
label = "Cosmology/Particle — BT-49, ζ(2)=π²/6, string landscape, fine structure"
n6 = 0.80
perf = 0.80
power = 0.90
cost = 0.60

[[candidate]]
id = "CRYPTO"
label = "Crypto/Network — BT-53, BTC 21M=J₂-n/φ, 6 confirms=n, ETH 12s=σ"
n6 = 0.60
perf = 0.50
power = 0.60
cost = 0.70

[[candidate]]
id = "MEDIA"
label = "Display/Audio — BT-48, σ=12 semitones, J₂=24 fps/bits, σ·τ=48kHz"
n6 = 0.70
perf = 0.55
power = 0.70
cost = 0.75

# ============================================================
# Compatibility Rules
# ============================================================

# --- prefer rules: natural pairings ---

# NT prefers DIRECT proof
[[rule]]
type = "prefer"
if_level = 0
if_id = "NT"
then_level = 3
then_ids = "DIRECT,COMBIN"

# GT prefers GROUP proof
[[rule]]
type = "prefer"
if_level = 0
if_id = "GT"
then_level = 3
then_ids = "GROUP"

# LT prefers LATTICE proof
[[rule]]
type = "prefer"
if_level = 0
if_id = "LT"
then_level = 3
then_ids = "LATTICE"

# TOP prefers TOPO proof
[[rule]]
type = "prefer"
if_level = 0
if_id = "TOP"
then_level = 3
then_ids = "TOPO"

# AN prefers ANALYTIC proof
[[rule]]
type = "prefer"
if_level = 0
if_id = "AN"
then_level = 3
then_ids = "ANALYTIC"

# CT prefers CATEG proof
[[rule]]
type = "prefer"
if_level = 0
if_id = "CT"
then_level = 3
then_ids = "CATEG"

# COMB prefers COMBIN proof
[[rule]]
type = "prefer"
if_level = 0
if_id = "COMB"
then_level = 3
then_ids = "COMBIN,DIRECT"

# J₂(6)=24 strongest in lattice and representation theory
[[rule]]
type = "prefer"
if_level = 1
if_id = "jordan"
then_level = 0
then_ids = "LT,RT"

# μ(6)=1 strongest in number theory and combinatorics
[[rule]]
type = "prefer"
if_level = 1
if_id = "mu"
then_level = 0
then_ids = "NT,COMB"

# Egyptian fraction strongest in NT and AN
[[rule]]
type = "prefer"
if_level = 1
if_id = "egypt"
then_level = 0
then_ids = "NT,AN"

# σ(6)=12 strongest in AG (modular weight) and RT
[[rule]]
type = "prefer"
if_level = 1
if_id = "sigma"
then_level = 0
then_ids = "AG,RT"

# R(6)=1 is NT-specific
[[rule]]
type = "prefer"
if_level = 1
if_id = "R6"
then_level = 0
then_ids = "NT"

# --- exclude rules: meaningless combinations ---

# Egyptian fraction has no structural connection to Mathematical Physics
[[rule]]
type = "exclude"
if_level = 1
if_id = "egypt"
then_level = 0
then_ids = "MP"

# λ(6)=2 has no known AG connection
[[rule]]
type = "exclude"
if_level = 1
if_id = "lambda_cm"
then_level = 0
then_ids = "AG"

# R(6)=1 ratio is not meaningful in Mathematical Physics
[[rule]]
type = "exclude"
if_level = 1
if_id = "R6"
then_level = 0
then_ids = "MP"
```

- [ ] **Step 2: commit**

```bash
git add tools/universal-dse/domains/pure-mathematics.toml
git commit -m "feat: ultimate pure-math TOML — 39,200 combos (10x10x8x7x7)"
```

---

### Task 3: run DSE

**Files:**
- Read: `tools/universal-dse/universal-dse` (existing binary)
- Read: `tools/universal-dse/domains/pure-mathematics.toml` (created in Task 2)

- [ ] **Step 1: run universal-dse (single domain)**

```bash
cd $N6_ARCH
tools/universal-dse/universal-dse tools/universal-dse/domains/pure-mathematics.toml --top 30
```

Expected: number of valid combos among 39,200, Pareto frontier, best-path output.

- [ ] **Step 2: inspect output**

Items to check:
- Total combinations (near 39,200)
- Compatible combinations (some excluded via exclude rules)
- Top 30 Pareto paths
- Best n6 path, best perf path
- n6 stats (min/max/avg/p50/p75/p90)

- [ ] **Step 3: record the result in docs/pure-mathematics/dse-results.md**

Capture top Pareto paths + stats in markdown. Include ASCII charts.

- [ ] **Step 4: commit**

```bash
git add docs/pure-mathematics/dse-results.md
git commit -m "feat: ultimate pure-math DSE results — Pareto frontier + best paths"
```

---

### Task 4: run Cross-DSE (3 domains)

**Files:**
- Read: `tools/universal-dse/domains/pure-mathematics.toml`
- Read: existing domain TOMLs for cross partners

- [ ] **Step 1: Cross-DSE — cosmology-particle**

If the cosmology-particle TOML does not exist, skip this step and start with chip-architecture.

```bash
# Cross-DSE with chip-architecture
tools/universal-dse/universal-dse \
  tools/universal-dse/domains/pure-mathematics.toml \
  tools/universal-dse/domains/chip.toml
```

Expected: each domain's individual result + top 10 cross combos.

- [ ] **Step 2: additional Cross-DSE (available domains)**

```bash
# Cross-DSE with solar
tools/universal-dse/universal-dse \
  tools/universal-dse/domains/pure-mathematics.toml \
  tools/universal-dse/domains/solar.toml

# Cross-DSE with battery
tools/universal-dse/universal-dse \
  tools/universal-dse/domains/pure-mathematics.toml \
  tools/universal-dse/domains/battery.toml
```

- [ ] **Step 3: append Cross-DSE results to dse-results.md**

Add tables per cross exploration. State cross-domain BT linkage points.

- [ ] **Step 4: commit**

```bash
git add docs/pure-mathematics/dse-results.md
git commit -m "feat: pure-math Cross-DSE — chip/solar/battery cross exploration complete"
```

---

### Task 5: update dse-map.toml

**Files:**
- Modify: `docs/dse-map.toml:184-187` (pure-mathematics section)

- [ ] **Step 1: update the pure-mathematics section**

Before:
```toml
[pure-mathematics]
goal = false
dse = "none"
cross_dse = ["cosmology-particle"]
```

After (with DSE results):
```toml
[pure-mathematics]
goal = true
dse = "done"
combos = 39200
tool = "tools/universal-dse/ domains/pure-mathematics.toml"
levels = ["Field", "Function", "Structure", "Proof", "Bridge"]
cross_dse = ["cosmology-particle", "chip-architecture", "biology"]
best_pareto = "<top path from Task 3>"
best_n6 = "<best n6% path from Task 3>"
n6_max = 0.0
n6_avg = 0.0
note = "Ultimate pure-math 5-level DSE. BT-49 based. 10 Field x 10 Function x 8 Structure x 7 Proof x 7 Bridge"
```

- [ ] **Step 2: add Cross-DSE sections**

Append at the bottom of the file:
```toml
[cross-dse.math-x-chip]
domains = ["pure-mathematics", "chip-architecture"]
status = "done"
tool = "tools/universal-dse/"
best = "<Task 4 result>"
note = "pure-math x chip — BT-28/49 linkage"

[cross-dse.math-x-solar]
domains = ["pure-mathematics", "solar-architecture"]
status = "done"
tool = "tools/universal-dse/"
best = "<Task 4 result>"
note = "pure-math x solar — SQ bandgap 4/3 = tau/n/phi"

[cross-dse.math-x-battery]
domains = ["pure-mathematics", "battery-architecture"]
status = "done"
tool = "tools/universal-dse/"
best = "<Task 4 result>"
note = "pure-math x battery — CN=6 cathode, BT-43"
```

- [ ] **Step 3: commit**

```bash
git add docs/dse-map.toml
git commit -m "docs: dse-map.toml — record ultimate pure-math DSE complete + 3 Cross-DSEs"
```

---

### Task 6: update the DSE column in README.md

**Files:**
- Modify: `README.md` (pure-math row in the ultimate-architecture roadmap table)

- [ ] **Step 1: change the DSE cell in the pure-math row from "—" to "39,200" or a check mark**

- [ ] **Step 2: commit**

```bash
git add README.md
git commit -m "docs: README pure-math DSE completion marker"
```
