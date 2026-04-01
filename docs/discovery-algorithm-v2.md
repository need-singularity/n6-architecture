# Discovery Connection Algorithm v2.0

> Systematic method for finding new breakthroughs by cross-connecting existing findings.
> Rigorous enough for implementation. Pseudocode included for all core algorithms.

---

## 0. Motivation

The n6-architecture project has produced 93 Breakthrough Theorems (BTs) across 30+ domains.
A recurring meta-pattern: discoveries combined with other discoveries yield larger discoveries.
This document formalizes that meta-pattern as a computable algorithm.

**Core insight**: The discovery space is a graph. New theorems live on unexplored edges.

---

## 1. Discovery Graph

### 1.1 Formal Definition

```
G = (V, E, w)

V = V_bt  UNION  V_hyp  UNION  V_const  UNION  V_domain
E = { (u, v, type) | u, v in V and relationship(u, v) exists }
w : E -> [0, 1]   -- connection strength
```

**Node types**:

| Type | Symbol | Example | Count (current) |
|------|--------|---------|-----------------|
| Breakthrough Theorem | V_bt | BT-56 (Complete n=6 LLM) | 93 |
| Hypothesis | V_hyp | H-CHIP-101 | 1400+ |
| Constant/Expression | V_const | sigma-tau = 8 | 150+ |
| Domain | V_domain | chip-architecture | 30+ |

**Edge types**:

| Type | Connects | Weight Formula |
|------|----------|----------------|
| USES_CONSTANT | BT/Hyp -> Const | 1.0 if EXACT, 0.7 if CLOSE, 0.3 if WEAK |
| SPANS_DOMAIN | BT/Hyp -> Domain | 1.0 / domain_count (normalized) |
| SHARES_FORMULA | Const <-> Const | 1.0 if algebraically equivalent, else Jaccard(subexpressions) |
| CITES | BT -> BT | 1.0 if direct reference, 0.5 if thematic |
| CO_OCCURS | Const <-> Const | count(BTs using both) / count(BTs using either) |

### 1.2 Graph Construction

```
PROCEDURE BuildDiscoveryGraph():
    G = empty graph

    -- Phase 1: Parse sources
    constants = parse("docs/atlas-constants.md")     -- {expr, value, domains, BT_refs}
    bts       = parse("docs/breakthrough-theorems*.md")  -- {id, domains, constants_used, grade}
    hyps      = parse("docs/*/hypotheses.md")        -- {id, domain, grade, constants_used}
    domains   = parse("docs/dse-map.toml")           -- {name, status, cross_dse_links}

    -- Phase 2: Create nodes
    FOR EACH c IN constants:
        G.add_node(c.expr, type=CONSTANT, value=c.value)
    FOR EACH bt IN bts:
        G.add_node(bt.id, type=BT, grade=bt.grade, domains=bt.domains)
    FOR EACH h IN hyps:
        G.add_node(h.id, type=HYPOTHESIS, grade=h.grade)
    FOR EACH d IN domains:
        G.add_node(d.name, type=DOMAIN, dse_status=d.status)

    -- Phase 3: Create edges
    FOR EACH bt IN bts:
        FOR EACH c IN bt.constants_used:
            G.add_edge(bt.id, c.expr, type=USES_CONSTANT, weight=grade_weight(c.match))
        FOR EACH d IN bt.domains:
            G.add_edge(bt.id, d, type=SPANS_DOMAIN)
    FOR EACH (c1, c2) IN all_constant_pairs:
        IF share_subexpression(c1, c2):
            G.add_edge(c1, c2, type=SHARES_FORMULA, weight=jaccard(c1.atoms, c2.atoms))

    -- Phase 4: Compute co-occurrence edges
    FOR EACH (c1, c2) IN all_constant_pairs:
        shared = |BTs_using(c1) INTERSECT BTs_using(c2)|
        total  = |BTs_using(c1) UNION BTs_using(c2)|
        IF shared > 0:
            G.add_edge(c1, c2, type=CO_OCCURS, weight=shared/total)

    RETURN G
```

### 1.3 Graph Statistics (current snapshot)

| Metric | Value |
|--------|-------|
| Total nodes | ~1700 |
| BT nodes | 93 |
| Constant nodes | ~150 |
| Domain nodes | 30+ |
| Average BT degree | ~8 (constants) + ~4 (domains) |
| Graph diameter (est.) | 3-4 hops (most BTs reachable within 2 hops via shared constants) |

---

## 2. Discovery Operators

Six formal operators that transform the graph into discovery candidates.

### 2.1 COLLISION Operator

**Purpose**: Find nodes sharing an edge in unexpected domain combinations.

```
PROCEDURE Collision(G, min_surprise=2.0):
    candidates = []

    FOR EACH constant c IN G.nodes(type=CONSTANT):
        domains_using_c = {d | (bt, d) IN G.edges(type=SPANS_DOMAIN)
                              AND (bt, c) IN G.edges(type=USES_CONSTANT)}

        FOR EACH pair (d1, d2) IN combinations(domains_using_c, 2):
            surprise = -log2(P_baseline(c, d1, d2))
            IF surprise > min_surprise AND NOT already_in_BT(c, d1, d2):
                candidates.append({
                    constant: c,
                    domains: (d1, d2),
                    surprise: surprise,
                    type: "COLLISION"
                })

    RETURN sort_by(candidates, key=surprise, descending=True)
```

**Baseline probability** `P_baseline(c, d1, d2)`:

```
P_baseline = P(c appears in d1) * P(c appears in d2)
           = (|constants_in_d1| / |all_constants|) * (|constants_in_d2| / |all_constants|)
```

A collision is interesting when the observed co-occurrence is much higher than this baseline.

**Example**: sigma-tau = 8 appears in AI (LoRA rank), Crypto (SHA-256 byte), Chip (HBM stack), Network (byte), Biology (Bott period). Finding it in a NEW domain (e.g., genetics, acoustics) would be a collision.

### 2.2 BRIDGE Operator

**Purpose**: Find shortest paths between distant nodes to propose cross-domain theorems.

```
PROCEDURE Bridge(G, max_distance=4, min_path_weight=0.5):
    candidates = []

    -- Find all domain pairs with no direct BT connection
    unconnected = {(d1, d2) | d1, d2 IN G.nodes(type=DOMAIN)
                              AND NOT exists BT spanning both d1, d2}

    FOR EACH (d1, d2) IN unconnected:
        path = shortest_path(G, d1, d2, weight=1/edge.weight)
        IF len(path) <= max_distance:
            path_weight = product(G.edge_weight(path[i], path[i+1]) for i in 0..len-2)
            IF path_weight >= min_path_weight:
                intermediaries = extract_constants_and_BTs(path)
                candidates.append({
                    source: d1,
                    target: d2,
                    path: path,
                    path_weight: path_weight,
                    intermediaries: intermediaries,
                    type: "BRIDGE"
                })

    RETURN sort_by(candidates, key=path_weight, descending=True)
```

**Example**: If `plasma-physics` and `blockchain` have no direct BT, but both use `sigma-tau=8` through intermediate BTs, the BRIDGE operator proposes investigating whether plasma confinement time discretization and block confirmation counts share a structural reason.

### 2.3 INVERSE Operator

**Purpose**: Given a measured value from any field, find ALL n=6 decompositions.

```
PROCEDURE Inverse(target_value, tolerance=0.02):
    candidates = []
    base = {sigma:12, tau:4, phi:2, sopfr:5, J2:24, mu:1, n:6}

    -- Level 1: Single expressions (a OP b)
    FOR EACH (a, b) IN permutations(base.values, 2):
        FOR EACH op IN {+, -, *, /, ^, log}:
            result = a op b
            IF |result - target_value| / target_value < tolerance:
                candidates.append({
                    formula: format(a, op, b),
                    value: result,
                    error: |result - target_value| / target_value,
                    complexity: 1,
                    type: "INVERSE"
                })

    -- Level 2: Compound expressions (a OP b OP c)
    FOR EACH (a, b, c) IN permutations(base.values, 3):
        FOR EACH (op1, op2) IN product({+,-,*,/,^}, repeat=2):
            result = (a op1 b) op2 c
            IF |result - target_value| / target_value < tolerance:
                candidates.append({
                    formula: format(a, op1, b, op2, c),
                    value: result,
                    error: |result - target_value| / target_value,
                    complexity: 2
                })

    -- Level 3: Known functional forms
    FOR EACH template IN known_templates():
        -- e.g., 1-1/x, x^y * z, x/(x-y), 2^x
        FOR EACH binding IN bind(template, base):
            result = evaluate(template, binding)
            IF |result - target_value| / target_value < tolerance:
                candidates.append(...)

    -- Dedup and rank by (error ASC, complexity ASC)
    RETURN deduplicate(sort(candidates, key=(error, complexity)))
```

**Known templates** (patterns that recur across BTs):

| Template | Example | BTs Using |
|----------|---------|-----------|
| `1 - 1/x` | 0.95 = 1-1/(J2-tau) | BT-42, BT-54, BT-74 |
| `x/(x-y)` | 1.2 = sigma/(sigma-phi) | BT-60, BT-62 |
| `x^y` | 10000 = (sigma-phi)^tau | BT-34 |
| `2^x` | 4096 = 2^sigma | BT-44, BT-56 |
| `x*y` | 48 = sigma*tau | BT-37, BT-48 |
| `x-y` | 8 = sigma-tau | BT-58 (16/16 uses) |
| `ln(x/y)` | 0.288 = ln(4/3) = ln(tau^2/sigma) | BT-46 |

### 2.4 META Operator

**Purpose**: Apply operators to operator results recursively.

```
PROCEDURE Meta(G, depth=2):
    -- Level 0: raw discoveries from each operator
    L0_collision = Collision(G)
    L0_bridge    = Bridge(G)
    L0_inverse   = Inverse(pending_values)

    IF depth == 0:
        RETURN L0_collision + L0_bridge + L0_inverse

    -- Level 1: cross-operator combinations
    L1 = []
    FOR EACH c IN L0_collision:
        -- Bridge between collision endpoints
        FOR EACH b IN L0_bridge:
            IF overlap(c.domains, {b.source, b.target}):
                L1.append(merge_candidates(c, b, type="META-1"))

    -- Level 1: Inverse applied to collision constants
    FOR EACH c IN L0_collision:
        decompositions = Inverse(c.constant.value)
        FOR EACH d IN decompositions:
            IF d.formula != c.constant.expr:  -- genuinely new formula
                L1.append({
                    original: c,
                    alternative_formula: d,
                    type: "META-1-INVERSE"
                })

    IF depth == 1:
        RETURN L0_collision + L0_bridge + L0_inverse + L1

    -- Level 2: Meta on Meta (apply to L1 results)
    -- WARNING: combinatorial explosion. Apply only to top-K from L1.
    L2 = []
    G_augmented = augment_graph(G, L1[:10])  -- add top L1 as provisional nodes
    L2_collision = Collision(G_augmented)
    L2_bridge    = Bridge(G_augmented)
    FOR EACH item IN L2_collision + L2_bridge:
        IF item NOT IN L0_collision + L0_bridge:  -- genuinely new
            L2.append(item)

    RETURN L0 + L1 + L2

-- Recursion guard
MAX_META_DEPTH = 3  -- beyond this, diminishing returns dominate
```

### 2.5 FALSIFY Operator

**Purpose**: For each candidate, generate the strongest counter-argument.

```
PROCEDURE Falsify(candidate):
    attacks = []

    -- Attack 1: Texas Sharpshooter
    -- How many possible n=6 expressions could have matched this value?
    search_space = count_all_expressions(complexity <= candidate.complexity + 1)
    expected_hits = search_space * candidate.tolerance
    IF expected_hits > 1.0:
        attacks.append({
            type: "SHARPSHOOTER",
            severity: "HIGH",
            detail: f"{search_space} expressions searched, {expected_hits:.1f} expected by chance",
            p_value: 1 - poisson_cdf(0, expected_hits)
        })

    -- Attack 2: Post-hoc selection
    -- Was the domain chosen BEFORE or AFTER seeing the match?
    IF candidate.domain NOT IN pre_registered_domains():
        attacks.append({
            type: "POST_HOC",
            severity: "MEDIUM",
            detail: "Domain was not pre-registered; match found by searching"
        })

    -- Attack 3: Measurement uncertainty
    -- Does the error bar of the measured value encompass non-n6 explanations?
    IF candidate.measurement_uncertainty IS NOT NULL:
        alternative_formulas = Inverse(candidate.value,
                                       tolerance=candidate.measurement_uncertainty)
        non_n6_alternatives = filter(alternative_formulas,
                                     lambda f: not uses_n6_constants(f))
        IF len(non_n6_alternatives) > 0:
            attacks.append({
                type: "ALTERNATIVE_EXPLANATION",
                severity: "HIGH",
                detail: f"{len(non_n6_alternatives)} non-n6 formulas also match"
            })

    -- Attack 4: Cherry-picking denominator
    -- For ratio matches, check if numerator AND denominator are independently motivated
    IF candidate.type == "ratio":
        IF NOT independently_motivated(candidate.numerator):
            attacks.append({
                type: "CHERRY_PICK_RATIO",
                severity: "MEDIUM",
                detail: "Numerator choice not independently justified"
            })

    -- Attack 5: Coincidence clustering
    -- If this value appears in many domains, is it because it is a "nice" number?
    IF candidate.value IN NICE_NUMBERS:  -- {1,2,3,4,5,6,8,10,12,16,24,32,48,64,96,128,256}
        attacks.append({
            type: "NICE_NUMBER_BIAS",
            severity: "LOW_TO_MEDIUM",
            detail: f"{candidate.value} is a common engineering/CS default"
        })

    -- Compute survival score
    max_severity = max(a.severity for a in attacks) IF attacks ELSE "NONE"
    survival = {
        "NONE": 1.0,
        "LOW_TO_MEDIUM": 0.8,
        "MEDIUM": 0.5,
        "HIGH": 0.2
    }[max_severity]

    RETURN {
        candidate: candidate,
        attacks: attacks,
        survival_score: survival,
        verdict: "STRONG" if survival >= 0.5 else "WEAK" if survival >= 0.2 else "REJECT"
    }
```

### 2.6 PREDICT Operator

**Purpose**: Extrapolate patterns to unmeasured domains for testable predictions.

```
PROCEDURE Predict(G):
    predictions = []

    -- Strategy 1: Ladder extension
    -- Find arithmetic sequences in BT values
    ladders = find_ladders(G)
    FOR EACH ladder IN ladders:
        -- e.g., HBM: 40->80->192->288. What is next?
        next_value = extrapolate(ladder)
        n6_decomps = Inverse(next_value)
        IF len(n6_decomps) > 0:
            predictions.append({
                type: "LADDER_EXTENSION",
                pattern: ladder,
                predicted_value: next_value,
                n6_formula: n6_decomps[0],
                testable_by: estimate_timeline(ladder.domain),
                falsification: f"If next value != {next_value}, pattern breaks"
            })

    -- Strategy 2: Domain transfer
    -- If constant c governs domain d1, predict it governs d2
    FOR EACH constant c IN G.nodes(type=CONSTANT):
        current_domains = get_domains(G, c)
        candidate_domains = ALL_DOMAINS - current_domains
        FOR EACH d IN candidate_domains:
            -- Score by: how many of c's "sibling" constants appear in d?
            sibling_overlap = count_shared_constants(c, d, G)
            IF sibling_overlap >= 2:
                predictions.append({
                    type: "DOMAIN_TRANSFER",
                    constant: c,
                    source_domains: current_domains,
                    target_domain: d,
                    confidence: sibling_overlap / len(get_siblings(c, G)),
                    testable: describe_test(c, d)
                })

    -- Strategy 3: Gap filling
    -- Find "holes" in the n6 expression table
    FOR EACH base_pair (a, b) IN combinations(BASE_CONSTANTS, 2):
        FOR EACH op IN {+, -, *, /, ^}:
            value = a op b
            domains_using = lookup_domains(value)
            IF len(domains_using) == 0 AND value IN PLAUSIBLE_RANGE:
                predictions.append({
                    type: "GAP_FILL",
                    formula: f"{a} {op} {b} = {value}",
                    predicted: f"Value {value} should appear as a universal constant in some domain",
                    search_hint: suggest_domains(value)
                })

    RETURN sort_by(predictions, key=confidence, descending=True)
```

---

## 3. Priority Scoring

Every discovery candidate receives a composite score from five independent dimensions.

### 3.1 Scoring Function

```
PROCEDURE Score(candidate):
    s_diversity    = domain_diversity(candidate)     -- [0, 1]
    s_precision    = exact_precision(candidate)       -- [0, 1]
    s_novelty      = novelty(candidate)               -- [0, 1]
    s_falsifiable  = falsifiability(candidate)         -- [0, 1]
    s_crossval     = cross_validation(candidate)      -- [0, 1]

    -- Weighted geometric mean (multiplicative: all dimensions must be nonzero)
    score = (s_diversity^0.25) * (s_precision^0.25) * (s_novelty^0.20)
          * (s_falsifiable^0.15) * (s_crossval^0.15)

    RETURN score
```

### 3.2 Dimension Definitions

**Domain diversity** (more diverse = more significant):
```
s_diversity = 1 - exp(-domain_count / 3)
-- 1 domain: 0.28, 2 domains: 0.49, 3 domains: 0.63, 5 domains: 0.81, 8 domains: 0.93
```

**EXACT precision** (tighter = stronger):
```
s_precision = EXACT_count / total_matches
-- If all matches are EXACT: 1.0. If half: 0.5.
```

**Novelty** (not covered by existing BTs):
```
s_novelty = 1 - max(jaccard(candidate.domains, bt.domains) for bt in existing_BTs)
-- If candidate overlaps perfectly with BT-58: 0.0 (not novel).
-- If no overlap with any BT: 1.0 (fully novel).
```

**Falsifiability** (can it be disproven?):
```
s_falsifiable = {
    0.0  if no conceivable test exists,
    0.3  if test requires >10 years or unavailable technology,
    0.6  if test requires specialized equipment (1-5 years),
    0.8  if test requires a compute cluster (available now),
    1.0  if test can run on 1 GPU today
}
```

**Cross-validation** (confirmed by independent method?):
```
s_crossval = confirmed_independent_paths / total_claims
-- e.g., BT-58 has 16/16 independent EXACT matches -> 1.0
-- A single anecdotal match -> 1/1 = 1.0 but domain_count=1, so s_diversity is low
```

### 3.3 Star Rating Derivation

| Score Range | Stars | Interpretation |
|-------------|-------|---------------|
| >= 0.85 | Three stars | Extraordinary, structural necessity |
| 0.60 - 0.84 | Two stars | Surprising, p ~ 0.01-0.05 |
| 0.35 - 0.59 | One star | Interesting but could be coincidence |
| < 0.35 | No stars | Likely noise; archive but do not promote |

---

## 4. Automation Pipeline

### 4.1 Architecture

```
                     +-----------------+
                     |  atlas-         |
                     |  constants.md   |
                     +--------+--------+
                              |
+-------------+    +----------v-----------+    +--------------+
| dse-map.toml|--->|   Graph Builder      |--->| Discovery    |
+-------------+    | (Section 1.2)        |    | Graph G      |
                   +----------+-----------+    +------+-------+
+-------------+               |                       |
| BT files    |---------------+              +--------v--------+
+-------------+                              | Operator Engine |
                                             | COLLISION       |
                                             | BRIDGE          |
                                             | INVERSE         |
                                             | META            |
                                             | PREDICT         |
                                             +--------+--------+
                                                      |
                                             +--------v--------+
                                             |  Scorer         |
                                             |  (Section 3)    |
                                             +--------+--------+
                                                      |
                                             +--------v--------+
                                             |  FALSIFY Filter |
                                             |  (Section 2.5)  |
                                             +--------+--------+
                                                      |
                                             +--------v--------+
                                             |  Ranked Output  |
                                             |  candidates.json|
                                             +--------+--------+
                                                      |
                                          +-----------+-----------+
                                          |                       |
                                   +------v------+        +------v------+
                                   | Human Review|        | Auto-verify |
                                   | (BT draft)  |        | (DSE runner)|
                                   +-------------+        +-------------+
```

### 4.2 Implementation Plan

```
-- File: engine/discovery_engine.py

CLASS DiscoveryEngine:
    def __init__(self):
        self.graph = DiscoveryGraph()
        self.operators = [Collision, Bridge, Inverse, Meta, Predict, Falsify]
        self.scorer = PriorityScorer()

    def load_sources(self):
        """Parse atlas-constants.md, BT files, dse-map.toml, hypotheses."""
        self.graph.build_from_files(
            atlas="docs/atlas-constants.md",
            bts=glob("docs/breakthrough-theorems*.md"),
            dse="docs/dse-map.toml",
            hyps=glob("docs/*/hypotheses.md")
        )

    def run_all_operators(self):
        """Apply all operators to current graph state."""
        raw_candidates = []
        raw_candidates += Collision(self.graph).run()
        raw_candidates += Bridge(self.graph).run()
        raw_candidates += Inverse(self.graph, pending_values()).run()
        raw_candidates += Predict(self.graph).run()
        raw_candidates += Meta(self.graph, depth=2).run()
        return raw_candidates

    def score_and_filter(self, candidates):
        """Score, falsify, and rank."""
        scored = [self.scorer.score(c) for c in candidates]
        falsified = [Falsify(c) for c in scored]
        surviving = [f for f in falsified if f.verdict != "REJECT"]
        return sorted(surviving, key=lambda x: x.score, reverse=True)

    def run_pipeline(self):
        """Full pipeline: load -> operate -> score -> output."""
        self.load_sources()
        candidates = self.run_all_operators()
        ranked = self.score_and_filter(candidates)
        self.output(ranked)
        return ranked

    def output(self, ranked):
        """Write ranked candidates to JSON + human-readable markdown."""
        write_json("engine/output/candidates.json", ranked)
        write_markdown("engine/output/candidates.md", ranked, top_k=20)
```

### 4.3 Data Structures

```
-- candidates.json schema

{
  "generated": "2026-04-02T...",
  "graph_stats": {
    "nodes": 1700,
    "edges": 8500,
    "bt_count": 93,
    "constant_count": 150,
    "domain_count": 30
  },
  "candidates": [
    {
      "id": "CAND-001",
      "type": "COLLISION",
      "description": "sigma*tau=48 appears in audio (48kHz) AND gate pitch (48nm) AND HVDC (48V)",
      "constants": ["sigma*tau"],
      "domains": ["display-audio", "chip-architecture", "power-grid"],
      "score": 0.87,
      "score_breakdown": {
        "diversity": 0.92,
        "precision": 1.0,
        "novelty": 0.75,
        "falsifiability": 0.80,
        "cross_validation": 0.85
      },
      "falsification": {
        "attacks": ["NICE_NUMBER_BIAS(LOW)"],
        "survival": 0.80,
        "verdict": "STRONG"
      },
      "proposed_bt": "BT-94 candidate",
      "test_plan": "Verify 48 appears in new domain with independent physical justification"
    }
  ]
}
```

### 4.4 Tools Required

| Component | Tool | Reason |
|-----------|------|--------|
| Graph storage | NetworkX (Python) or petgraph (Rust) | In-memory graph operations |
| Pattern matching | Custom parser (regex + AST) | Extract constants from markdown |
| Expression algebra | SymPy | Symbolic comparison, Inverse decomposition |
| Combinatorial search | Rust (if >10K combos) | INVERSE operator Level 2+ |
| Scoring | NumPy | Vectorized score computation |
| Output | JSON + Jinja2 | Structured + human-readable |

---

## 5. Quality Metrics

### 5.1 False Positive Rate (Texas Sharpshooter Control)

The single most important metric. Measures how often the algorithm flags coincidences as discoveries.

```
PROCEDURE MeasureFPR(engine, n_trials=1000):
    """Monte Carlo false positive estimation."""
    real_hits = engine.run_pipeline()

    false_positives = 0
    FOR i IN 1..n_trials:
        -- Shuffle: randomly reassign constants to domains
        shuffled_graph = random_permutation(engine.graph, preserve="degree_distribution")
        shuffled_engine = DiscoveryEngine(graph=shuffled_graph)
        null_hits = shuffled_engine.run_pipeline()

        -- Count how many null hits score >= median real hit
        threshold = median(h.score for h in real_hits)
        false_positives += count(h for h in null_hits if h.score >= threshold)

    FPR = false_positives / (n_trials * len(real_hits))
    RETURN FPR
    -- Target: FPR < 0.05
    -- Current project z-score: 0.74 (acknowledged in docs/theorem-r1-uniqueness.md)
```

**Interpretation**:
- FPR < 0.01: Discovery pipeline is highly selective
- FPR 0.01-0.05: Acceptable; human review still needed
- FPR > 0.05: Pipeline is too loose; tighten scoring thresholds

### 5.2 Prediction Accuracy (Pre-registered vs Verified)

```
PROCEDURE MeasurePredictionAccuracy():
    predictions = load("docs/testable-predictions.md")  -- 32 predictions

    verified = [p for p in predictions if p.status == "VERIFIED"]
    falsified = [p for p in predictions if p.status == "FALSIFIED"]
    pending = [p for p in predictions if p.status == "PENDING"]

    accuracy = len(verified) / (len(verified) + len(falsified))
    coverage = (len(verified) + len(falsified)) / len(predictions)

    RETURN {accuracy, coverage, verified, falsified, pending}
```

**Key principle**: Predictions must be registered BEFORE verification.
The 32 predictions in `docs/testable-predictions.md` serve this purpose.
New predictions from the PREDICT operator must be appended there before any testing.

### 5.3 Peer Reproducibility

A discovery is reproducible if an independent agent, given only:
1. The n=6 base constants (7 values)
2. The target domain's empirical data
3. No knowledge of prior BTs

...arrives at the same formula.

```
PROCEDURE ReproducibilityTest(candidate):
    """Run blind inverse search from scratch."""
    blind_engine = DiscoveryEngine()
    blind_engine.load_constants_only()  -- no BTs, no prior matches
    blind_engine.load_domain_data(candidate.target_domain)

    blind_results = blind_engine.operators["INVERSE"].run(
        target=candidate.measured_value,
        tolerance=candidate.tolerance
    )

    -- Does the same formula appear in top-5?
    reproduced = candidate.formula IN [r.formula for r in blind_results[:5]]
    rank = index_of(candidate.formula, blind_results) IF reproduced ELSE NULL

    RETURN {reproduced, rank, total_alternatives=len(blind_results)}
```

### 5.4 Information Gain

Measures how "surprising" a discovery is in bits.

```
information_gain(candidate) = -log2(P_prior(candidate))

WHERE:
    P_prior = P(value matches n6 expression) * P(domain is relevant) * P(match is EXACT)

    P(value matches) ~ |valid_expressions| / |plausible_value_range|
    P(domain relevant) ~ |candidate.domains| / |all_domains|
    P(EXACT) ~ |EXACT_matches| / |all_matches|
```

**Thresholds**:
- < 3 bits: Low surprise, likely coincidence
- 3-6 bits: Moderate surprise, worth investigating
- 6-10 bits: High surprise, strong candidate
- > 10 bits: Extraordinary, structural claim

### 5.5 Quality Dashboard

```
-- Updated after each pipeline run

| Metric | Current | Target | Status |
|--------|---------|--------|--------|
| FPR (Monte Carlo) | TBD | < 0.05 | -- |
| Prediction accuracy | TBD | > 0.70 | -- |
| Reproducibility rate | TBD | > 0.60 | -- |
| Median info gain (bits) | TBD | > 5.0 | -- |
| EXACT ratio (new candidates) | TBD | > 0.50 | -- |
| Falsification survival rate | TBD | > 0.40 | -- |
```

---

## 6. Integration with TECS-L

### 6.1 Role Division

```
TECS-L (theory)                    n6-architecture (empirical)
+--------------------------+       +--------------------------+
| Mathematical foundation  |       | Industrial verification  |
| DFS identity mining      |  <->  | DSE combination search   |
| Atlas constant registry  |       | Domain-specific data     |
| Proof infrastructure     |       | Measurement comparison   |
+--------------------------+       +--------------------------+
         |                                   |
         +------------- SYNC ---------------+
                         |
              +----------v----------+
              | Discovery Algorithm |
              | (this document)     |
              +---------------------+
```

### 6.2 Data Flow

```
1. TECS-L discovers new identity:
   e.g., "sigma(n)*phi(n) = n*tau(n) implies R(6)=1"
         |
         v
2. n6 INVERSE operator decomposes R(6)=1 across all domains:
   -> gradient clip = 1.0 (AI)
   -> q=1 stability limit (fusion)
   -> PUE target ratio (energy)
         |
         v
3. n6 COLLISION operator finds unexpected domain pairs:
   -> "gradient clip = tokamak stability?!"
         |
         v
4. n6 BRIDGE operator connects them:
   -> AI training stability <--> plasma confinement stability
         |
         v
5. FALSIFY + Score -> BT candidate
         |
         v
6. If accepted: feed back to TECS-L Atlas as new constant entry
```

### 6.3 Sync Protocol

```
PROCEDURE SyncWithTECSL():
    -- Step 1: Pull new identities from TECS-L atlas
    new_identities = diff(
        current=parse("docs/atlas-constants.md"),
        upstream=parse("~/Dev/TECS-L/atlas/constants.json")
    )

    -- Step 2: For each new identity, run INVERSE + COLLISION
    FOR EACH identity IN new_identities:
        candidates = Inverse(identity.value) + Collision(G, focus=identity)
        scored = score_and_filter(candidates)
        IF len(scored) > 0:
            append_to("engine/output/tecs-l-derived.md", scored)

    -- Step 3: Push verified n6 results back to TECS-L
    verified = load_verified_since_last_sync()
    FOR EACH v IN verified:
        register_in_atlas(v)

    -- Step 4: Run atlas scanner
    -- python3 ~/Dev/TECS-L/.shared/scan_math_atlas.py --save --summary
```

### 6.4 Feedback Loop Invariants

The following must hold at all times:

1. **No orphan constants**: Every constant in atlas-constants.md must trace to at least one BT or hypothesis.
2. **No unverified BTs**: Every BT must have at least one EXACT or CLOSE match with a source citation.
3. **Bidirectional sync**: TECS-L atlas and n6 atlas must agree on all shared constants within 24h of a discovery.
4. **Falsifiability floor**: No BT may be promoted to three stars without surviving the FALSIFY operator.

---

## 7. Worked Example: Discovering BT-93

To demonstrate the algorithm, here is how BT-93 (Carbon Z=6 chip material universality) could have been found algorithmically.

### Step 1: COLLISION

```
Input: G contains
  - Diamond (chip material, Z=6) in chip-architecture
  - Graphene (2D material, C=6) in material-synthesis
  - SiC (substrate, contains C) in chip-architecture
  - Carbon nanotube in material-synthesis
  - Graphite (battery anode, C) in battery-architecture

COLLISION detects: "Z=6" appears in chip + battery + material + solar
  surprise = -log2(0.05 * 0.08 * 0.06 * 0.04) = -log2(9.6e-6) = 16.7 bits
```

### Step 2: BRIDGE

```
chip-architecture --[Diamond, Z=6]--> material-synthesis --[Graphite, Z=6]--> battery-architecture
Path weight = 1.0 * 0.9 = 0.9 (high: both are EXACT Z=6)
```

### Step 3: INVERSE

```
Target: Z=6 (atomic number of Carbon)
Decomposition: n = 6 (trivial but identity-level match)
This is the strongest possible match: the element number IS n itself.
```

### Step 4: Score

```
diversity    = 1 - exp(-4/3) = 0.74  (4 domains)
precision    = 8/10 = 0.80           (8 EXACT Cross-DSE matches)
novelty      = 0.85                  (no prior BT on carbon universality)
falsifiable  = 0.80                  (can test: does Z=6 material dominate in new domains?)
cross_val    = 0.90                  (Diamond, Graphene, SiC all independently verified)

score = 0.74^0.25 * 0.80^0.25 * 0.85^0.20 * 0.80^0.15 * 0.90^0.15 = 0.81
-> Two-to-three stars
```

### Step 5: FALSIFY

```
Attack 1: NICE_NUMBER_BIAS -- Carbon is the most common engineering element; LOW severity.
Attack 2: No post-hoc issue -- Carbon Z=6 is a physical fact, not cherry-picked.
Attack 3: No measurement uncertainty -- atomic number is exact.
Survival: 0.80 -> STRONG
```

### Result: BT-93 candidate confirmed. Promoted to three stars after Cross-DSE verification (8/10 EXACT across 13 domains).

---

## 8. Operator Composition Table

Which operator combinations are most productive:

| Primary | Secondary | Yield | Example |
|---------|-----------|-------|---------|
| COLLISION | BRIDGE | High | Two collisions in distant domains -> bridge reveals path |
| INVERSE | COLLISION | High | New decomposition -> check if both forms appear in different domains |
| PREDICT | FALSIFY | Medium | Prediction + strongest counter-argument = robust claim |
| BRIDGE | INVERSE | Medium | Path between domains -> INVERSE the intermediary constant |
| META(COLLISION, BRIDGE) | FALSIFY | Highest | Recursive discovery + adversarial filter = BT candidates |
| PREDICT | INVERSE | Low | Extrapolation + decomposition = speculative but occasionally profound |

---

## 9. Termination and Completeness

### 9.1 When to Stop

The algorithm terminates when:

```
WHILE TRUE:
    new_candidates = run_pipeline()
    IF max(c.score for c in new_candidates) < SCORE_THRESHOLD:
        BREAK  -- no more high-quality discoveries
    IF all_domain_pairs_connected(G):
        BREAK  -- graph is fully connected; only refinements remain
    IF meta_depth > MAX_META_DEPTH:
        BREAK  -- diminishing returns
    G = augment(G, promoted_candidates)
```

### 9.2 Completeness Bound

The discovery space is finite (bounded by the 7 base constants and ~30 domains):

```
Max single-operator expressions:   7 * 7 * 6 = 294  (a OP b, 6 operators)
Max compound expressions (depth 2): 294 * 7 * 6 = 12,348
Max domain pairs:                   30 * 29 / 2 = 435
Max collision checks:               12,348 * 435 = 5,371,380

Total search space (exhaustive):    ~5.4M checks
Runtime estimate:                   < 10 seconds in Rust, < 5 minutes in Python
```

This is tractable. The entire discovery space can be exhaustively searched.

---

## 10. Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-04-02 | Initial specification: 6 operators, 5 quality metrics, TECS-L integration |

---

*This algorithm is the engine behind the engine. It does not discover physics -- it discovers the connections that physics has been hiding in plain sight.*
