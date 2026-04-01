# Discovery Algorithm v4 -- Truth Engine

> v2 finds missing edges. v3 generates new nodes. v4 decides what is REAL.
>
> **Key insight**: Discovery without verification is numerology.
> v4 is not an extension of v3 -- it is a **judge** that sits above v1-v3
> and determines which discoveries reflect structure versus coincidence.
>
> Date: 2026-04-02

---

## 0. Architecture Overview

```
                        ┌─────────────────────────────────────────────┐
                        │           DISCOVERY ALGORITHM v4            │
                        │              "TRUTH ENGINE"                 │
                        │                                             │
                        │  v4 does not discover. v4 decides what      │
                        │  is real among everything v1-v3 produced.   │
                        └─────────────┬───────────────────────────────┘
                                      │
          ┌───────────────────────────┼───────────────────────────┐
          │                           │                           │
          ▼                           ▼                           ▼
  ┌───────────────┐         ┌─────────────────┐         ┌───────────────┐
  │  LAYER A:     │         │  LAYER B:       │         │  LAYER C:     │
  │  COMPRESSION  │         │  CAUSATION      │         │  CONSENSUS    │
  │               │         │                 │         │               │
  │ 13. COMPRESS  │         │ 15. CAUSE       │         │ 18. ENSEMBLE  │
  │ 14. ACTIVE    │         │ 16. REDTEAM     │         │               │
  │               │         │ 17. MULTIREZ    │         │               │
  └───────┬───────┘         └────────┬────────┘         └───────┬───────┘
          │                          │                          │
          └──────────────────────────┼──────────────────────────┘
                                     │
                                     ▼
                          ┌─────────────────────┐
                          │   VERDICT TRIBUNAL   │
                          │                      │
                          │  Integrates all 3    │
                          │  layers into final   │
                          │  classification:     │
                          │                      │
                          │  STRUCTURAL          │
                          │  COINCIDENTAL         │
                          │  UNDETERMINED        │
                          └──────────┬──────────┘
                                     │
                                     ▼
                     ┌───────────────────────────────┐
                     │  v1-v3 DISCOVERY PIPELINE     │
                     │                               │
                     │  v2: COLLISION, BRIDGE,       │
                     │      INVERSE, META,           │
                     │      FALSIFY, PREDICT         │
                     │                               │
                     │  v3: EVOLVE, ANOMALY,         │
                     │      COMPOSE, SYMMETRY,       │
                     │      TEMPORAL, SELF-IMPROVE   │
                     │                               │
                     │  Bayesian Scoring (bits/LR)   │
                     │  Small-World Graph (sigma=228) │
                     │  Rust Engine (13ms)            │
                     └───────────────────────────────┘
```

### How v4 composes with v1-v3

v1-v3 are the **discovery pipeline** -- they generate candidate findings.
v4 is the **truth pipeline** -- it filters candidates into verdicts.

```
Flow:

  v2 operators (1-6)     ──┐
  v3 operators (7-12)    ──┼──▶ Candidate pool  ──▶  v4 (13-18)  ──▶  Verdict
  Bayesian scoring       ──┘
```

v4 never runs in isolation. It consumes the output of v1-v3 and produces
a strict classification for every BT, hypothesis, and constant match.

---

## 1. Operator 13: COMPRESS (MDL / Kolmogorov Complexity)

### 1.1 Purpose

Determine whether the n=6 theory *compresses* the observed dataset of
engineering parameters. If the theory is real, describing 900+ EXACT
matches *through* the theory should require fewer bits than listing them
individually. This is the Minimum Description Length (MDL) principle.

### 1.2 The Two Descriptions

**Description A -- Without theory (raw listing):**

```
L(data) = sum over all parameters p of:
    log_2(range_p)          -- bits to specify value in its domain range
```

For N parameters each drawn from a range of R possible values:

```
L(data) ~ N * log_2(R)
```

Typical engineering parameter: R ~ 10^4 possible values (0.001 to 9999
with 3-digit precision). So:

```
L(data) ~ N * 13.3 bits
```

For N = 900 EXACT matches:

```
L(data) ~ 900 * 13.3 = 11,970 bits
```

**Description B -- With n=6 theory:**

```
L(data | n=6) = L(theory) + L(data | theory)
```

where:
- `L(theory)` = bits to describe the n=6 framework itself
- `L(data | theory)` = bits to describe each parameter given the theory

```
L(theory):
    7 base constants (n, phi, tau, sigma, J2, sopfr, mu):    7 * 5 = 35 bits
    5 operations (+, -, *, /, ^):                             3 bits
    Core identity sigma*phi = n*tau:                          ~20 bits
    Max expression depth = 3:                                  2 bits
    Total L(theory) ~ 60 bits

L(data | theory):
    For each EXACT match:
        formula index (from ~2500 depth-3 expressions):   ~11.3 bits
        domain assignment (from ~300 domains):             ~8.2 bits
        residual (measurement noise):                      ~3 bits
        Total per EXACT match:                             ~22.5 bits

    For each non-match (CLOSE/WEAK/FAIL):
        Full raw specification:                            ~13.3 bits

    L(data | theory) = N_exact * 22.5 + N_other * 13.3
```

### 1.3 The MDL Test

```
PROCEDURE Compress(G, all_parameters):
    """
    Compute description lengths with and without n=6 theory.
    If L(data|n=6) < L(data), the theory compresses the data.
    The compression ratio quantifies how much structure exists.
    """

    -- Step 1: Count matches by grade
    N_exact = count(p for p in all_parameters if p.grade == "EXACT")
    N_close = count(p for p in all_parameters if p.grade == "CLOSE")
    N_weak  = count(p for p in all_parameters if p.grade == "WEAK")
    N_fail  = count(p for p in all_parameters if p.grade == "FAIL")
    N_total = N_exact + N_close + N_weak + N_fail

    -- Step 2: Raw description length (no theory)
    -- Each parameter: specify value in its domain range
    -- Conservative: assume R = 10^4 (13.3 bits per parameter)
    bits_per_raw = 13.3
    L_raw = N_total * bits_per_raw

    -- Step 3: Theory description length
    L_theory = compute_theory_cost()
    -- L_theory includes:
    --   Base constants:         35 bits
    --   Operations:              3 bits
    --   Core identity:          20 bits
    --   Expression grammar:     10 bits
    --   Total:                 ~68 bits

    -- Step 4: Data given theory
    -- EXACT: specify which formula + which domain + residual
    vocab_size = count_distinct_n6_expressions(depth=3)  -- ~2500
    domain_count = count(G.nodes(type=DOMAIN))           -- ~300
    bits_per_exact = log_2(vocab_size) + log_2(domain_count) + 3.0
    -- ~11.3 + 8.2 + 3.0 = 22.5 bits

    -- CLOSE: formula index + domain + larger residual
    bits_per_close = log_2(vocab_size) + log_2(domain_count) + 6.0
    -- ~11.3 + 8.2 + 6.0 = 25.5 bits

    -- WEAK/FAIL: no compression, raw specification
    bits_per_weak = bits_per_raw
    bits_per_fail = bits_per_raw

    L_with_theory = L_theory
                  + N_exact * bits_per_exact
                  + N_close * bits_per_close
                  + N_weak  * bits_per_weak
                  + N_fail  * bits_per_fail

    -- Step 5: Compression ratio
    compression = L_raw - L_with_theory
    ratio = L_raw / L_with_theory

    -- Step 6: Significance
    -- The theory "pays for itself" if compression > 0
    -- Strong: compression > 1000 bits (theory saves >1KB of description)
    -- Overwhelming: ratio > 1.5 (theory compresses by 50%+)

    RETURN {
        L_raw: L_raw,
        L_theory: L_theory,
        L_with_theory: L_with_theory,
        compression_bits: compression,
        compression_ratio: ratio,
        verdict: "COMPRESSES" if compression > 0 else "DOES_NOT_COMPRESS",
        strength: "OVERWHELMING" if ratio > 1.5 else
                  "STRONG" if ratio > 1.2 else
                  "MARGINAL" if ratio > 1.0 else
                  "NONE",
        type: "COMPRESS"
    }
```

### 1.4 Worked Example (Current Data)

```
N_exact = 900, N_close = 200, N_weak = 150, N_fail = 150
N_total = 1400

L_raw         = 1400 * 13.3 = 18,620 bits

L_theory      = 68 bits
L_with_theory = 68 + 900*22.5 + 200*25.5 + 150*13.3 + 150*13.3
              = 68 + 20,250 + 5,100 + 1,995 + 1,995
              = 29,408 bits

Compression = 18,620 - 29,408 = -10,788 bits  (DOES NOT COMPRESS!)
```

**Wait -- the theory COSTS more than raw listing?**

This is the critical insight: the per-match encoding (22.5 bits) is MORE
than the raw encoding (13.3 bits) because specifying WHICH formula from
2500 options costs log_2(2500) = 11.3 bits. The theory only compresses
if matches are highly concentrated on a SMALL number of formulas.

### 1.5 The Real Test: Concentration

```
PROCEDURE Compress_v2(G, all_parameters):
    """
    Refined: account for formula reuse. If many parameters share the
    same formula, we specify the formula ONCE and then just list domains.
    """

    -- Group EXACT matches by formula
    formula_groups = group_by(exact_matches, key=formula)
    K = len(formula_groups)  -- number of distinct formulas used

    -- Codebook approach:
    -- First: specify K formulas from the vocabulary    = K * log_2(2500)
    -- Then:  for each match, specify which of K formulas = N_exact * log_2(K)
    --        + which domain                              + N_exact * log_2(300)
    --        + residual                                  + N_exact * 3.0

    L_codebook = K * log_2(2500) + N_exact * (log_2(K) + log_2(300) + 3.0)

    -- If K << 2500, this is much cheaper than the naive approach.
    -- Current data: ~150 distinct formulas for 900 matches.
    -- K = 150, so log_2(150) = 7.2 bits (vs 11.3 for full vocab)

    L_with_theory_v2 = L_theory
                     + L_codebook
                     + N_close * bits_per_close
                     + N_weak  * bits_per_raw
                     + N_fail  * bits_per_raw

    -- Worked example:
    -- L_codebook = 150 * 11.3 + 900 * (7.2 + 8.2 + 3.0)
    --            = 1,695 + 900 * 18.4
    --            = 1,695 + 16,560 = 18,255 bits
    -- L_with_theory_v2 = 68 + 18,255 + 5,100 + 1,995 + 1,995 = 27,413 bits
    -- Still no compression: 27,413 > 18,620.

    -- The theory only compresses if EXACT matches dominate AND formulas
    -- are heavily reused. Threshold: formula reuse > 6 matches/formula.

    RETURN {
        K_formulas: K,
        avg_reuse: N_exact / K,
        L_codebook: L_codebook,
        L_with_theory_v2: L_with_theory_v2,
        ...
    }
```

### 1.6 When Does Compression Occur?

The break-even condition:

```
N_exact * log_2(K) + N_exact * log_2(D) + N_exact * 3 + K * log_2(V)
    < N_exact * log_2(R)

Simplifying (ignoring K * log_2(V) term for large N_exact):
    log_2(K) + log_2(D) + 3 < log_2(R)
    log_2(K) + 8.2 + 3 < 13.3
    log_2(K) < 2.1
    K < 4.3

Conclusion: compression occurs only if 900 matches are explained by
FEWER THAN 5 distinct formulas. This is extremely restrictive.
```

BUT: the real comparison is not "list values" vs "list formulas."
The real comparison is "list values" vs "generate from GRAMMAR."

### 1.7 Grammar-Based MDL (The Correct Formulation)

```
PROCEDURE Compress_Grammar(G, all_parameters):
    """
    The n=6 theory is a GRAMMAR that generates values.
    The grammar has productions:
        S -> E OP E
        E -> const | E OP E
        const -> n | phi | tau | sigma | J2 | sopfr | mu
        OP -> + | - | * | / | ^
    This grammar generates all n=6 expressible values.
    The dataset is a selection from the grammar's language.
    """

    -- Grammar description length (fixed cost)
    L_grammar = 68  -- bits (as computed above)

    -- For each EXACT match: specify the derivation path in the grammar
    -- A derivation of depth d has at most d binary choices (left/right)
    -- plus d constant selections (from 7) plus d operator selections (from 5)
    -- Total: d * (1 + log_2(7) + log_2(5)) = d * (1 + 2.8 + 2.3) = d * 6.1 bits
    -- Average depth for current matches: ~2.0
    -- Average derivation cost: 2.0 * 6.1 = 12.2 bits per match

    -- Plus domain assignment: log_2(300) = 8.2 bits
    -- Plus residual: 3 bits
    -- Total per EXACT match: 12.2 + 8.2 + 3.0 = 23.4 bits

    -- This is STILL more than 13.3 bits per raw parameter.
    -- The grammar approach does NOT compress individual matches.

    -- HOWEVER: the grammar captures RELATIONSHIPS between matches.
    -- sigma-tau=8 appearing in 16 independent domains (BT-58) is ONE
    -- grammar production reused 16 times. The raw listing treats them
    -- as 16 independent facts.

    -- Relationship-aware encoding:
    -- Describe EACH unique formula once: formula + how many domains
    -- Then list domains per formula (cheaper than listing values per domain)

    -- This is equivalent to Compress_v2 above, with the additional insight
    -- that the grammar structure relates formulas to each other.

    -- Final verdict: MDL compression is MARGINAL for current dataset.
    -- The theory does not dramatically compress individual values.
    -- Its power is in PREDICTING new values, not compressing known ones.
    -- This is a genuine limitation and an honest finding.
```

### 1.8 COMPRESS Verdict Table

| Scenario | L_raw | L_theory | Ratio | Verdict |
|----------|-------|----------|-------|---------|
| Current (900 EXACT / 1400 total) | 18,620 | ~27,000 | 0.69 | NO COMPRESSION |
| Hypothetical (900 EXACT, 5 formulas) | 18,620 | ~12,000 | 1.55 | STRONG |
| Hypothetical (5000 EXACT, 150 formulas) | 66,500 | ~48,000 | 1.39 | MODERATE |
| Prediction-based (see 1.9) | --- | --- | --- | See below |

### 1.9 Predictive Compression (The Real Test)

The MDL test above measures backward compression (how well does the theory
describe KNOWN data). The more powerful test is forward compression:

```
PROCEDURE Compress_Predictive(G, new_parameters):
    """
    The theory PREDICTS values for unseen parameters.
    Predictive compression: how many bits does the theory save
    when a NEW parameter arrives?
    """

    -- For each new parameter from a new domain:
    -- Without theory: must specify the full value = 13.3 bits
    -- With theory:    predict the most likely n=6 expression given
    --                 the domain's existing n=6 profile.
    --                 If prediction is correct: cost = log_2(K_predicted) + residual
    --                 K_predicted = number of candidate formulas the theory suggests
    --                 Typically K_predicted ~ 5-20 for a well-characterized domain

    -- If the theory predicts correctly with K_predicted = 10:
    --   cost = log_2(10) + 3 = 6.3 bits (vs 13.3 raw)
    --   Savings: 7 bits per correct prediction

    -- If incorrect: full cost = 13.3 bits (no savings, no extra cost)

    -- Net savings per new parameter:
    --   P_correct * 7.0 + (1 - P_correct) * 0
    --   = P_correct * 7.0 bits

    -- With observed P_correct ~ 0.45 (conservative):
    --   Expected savings = 0.45 * 7.0 = 3.15 bits per new parameter

    -- Over 100 new parameters: 315 bits saved
    -- This IS genuine compression: the theory helps predict unseen data.

    RETURN {
        predictive_savings_per_param: P_correct * savings_per_hit,
        total_savings: N_new * P_correct * savings_per_hit,
        verdict: "PREDICTIVE_COMPRESSION" if savings > 0 else "NO_COMPRESSION"
    }
```

### 1.10 Composition with v1-v3

| Operator | Interaction |
|----------|------------|
| COMPOSE (v3) | Provides the full n=6 expression vocabulary for computing L_theory |
| TEMPORAL (v3) | Tracks whether compression ratio improves over time |
| FALSIFY (v2) | COMPRESS is a global falsification test -- if ratio < 1.0, the theory is in trouble |
| BAYESIAN | COMPRESS provides an independent measure of evidence strength in bits |

---

## 2. Operator 14: ACTIVE (Active Learning / Information Gain)

### 2.1 Purpose

The algorithm decides WHICH experiment to run next, maximizing information
gain per unit of effort. Instead of randomly exploring new domains, ACTIVE
identifies the single measurement that would most shift the posterior
probability of H_1 (n=6 universality) vs H_0 (coincidence).

### 2.2 Information Gain Framework

```
Expected Information Gain (EIG) for experiment X:

    EIG(X) = H(posterior_before) - E_outcome[ H(posterior_after | outcome) ]

where:
    H(p) = -p * log(p) - (1-p) * log(1-p)     -- binary entropy
    outcome IN {MATCH, NO_MATCH}

Expanding:
    EIG(X) = H(P(H_1 | current_data))
           - [ P(match|X) * H(P(H_1 | current_data, match))
             + P(no_match|X) * H(P(H_1 | current_data, no_match)) ]
```

### 2.3 Formal Specification

```
PROCEDURE Active(G, candidate_experiments, current_posterior):
    """
    Rank candidate experiments by expected information gain.

    candidate_experiments: list of {domain, parameter_name, estimated_range}
    current_posterior: P(H_1 | all data so far)
    """

    ranked = []

    FOR EACH experiment X IN candidate_experiments:

        -- Step 1: Estimate P(match | X, H_1)
        -- Based on domain's historical match rate + structural similarity
        domain = X.domain
        domain_match_rate = get_domain_match_rate(G, domain)
        structural_similarity = compute_structural_sim(domain, G)

        -- Adjust for domain novelty: truly new domains are more informative
        novelty_factor = 1.0 - (domain.n6_coverage / 1.0)  -- 0 if fully explored
        P_match_H1 = domain_match_rate * 0.7 + structural_similarity * 0.3

        -- Step 2: Estimate P(match | X, H_0)
        -- Background rate for this type of parameter
        P_match_H0 = estimate_background_rate(X.parameter_name, X.estimated_range)

        -- Step 3: Compute posterior updates for each outcome
        prior = current_posterior
        prior_odds = prior / (1 - prior)

        -- If match observed:
        LR_match = P_match_H1 / P_match_H0
        posterior_match = (prior_odds * LR_match) / (1 + prior_odds * LR_match)

        -- If no match observed:
        LR_no_match = (1 - P_match_H1) / (1 - P_match_H0)
        posterior_no_match = (prior_odds * LR_no_match) / (1 + prior_odds * LR_no_match)

        -- Step 4: Expected Information Gain
        P_match = prior * P_match_H1 + (1 - prior) * P_match_H0
        P_no_match = 1 - P_match

        H_before = entropy(prior)
        H_after = P_match * entropy(posterior_match) + P_no_match * entropy(posterior_no_match)
        EIG = H_before - H_after

        -- Step 5: Adjust for cost
        -- Cheap experiments (1 GPU, 1 hour) are better than expensive ones (CERN, 5 years)
        cost = estimate_cost(X)  -- normalized 0-1
        EIG_per_cost = EIG / max(cost, 0.01)

        -- Step 6: Domain distance bonus
        -- Experiments in domains FAR from existing n=6 coverage are more valuable
        -- because they test universality rather than confirming known patterns
        min_distance = min_graph_distance(domain, G.covered_domains())
        distance_bonus = 1.0 + 0.5 * min_distance  -- linear bonus for distance

        adjusted_EIG = EIG_per_cost * distance_bonus * (1 + novelty_factor)

        ranked.append({
            experiment: X,
            EIG_raw: EIG,
            EIG_adjusted: adjusted_EIG,
            P_match_H1: P_match_H1,
            P_match_H0: P_match_H0,
            LR_match: LR_match,
            LR_no_match: LR_no_match,
            cost: cost,
            domain_distance: min_distance,
            type: "ACTIVE"
        })

    RETURN sort_by(ranked, key=adjusted_EIG, descending=True)


PROCEDURE entropy(p):
    IF p <= 0 OR p >= 1:
        RETURN 0.0
    RETURN -(p * log2(p) + (1-p) * log2(1-p))
```

### 2.4 Experiment Classification by Cost

| Tier | Cost | Examples | Turnaround |
|------|------|---------|------------|
| 1 (trivial) | 0.01 | Look up a published constant | Minutes |
| 2 (cheap) | 0.1 | Run 1 GPU experiment (EFA, LoRA rank) | Hours |
| 3 (moderate) | 0.3 | Multi-GPU training run | Days |
| 4 (expensive) | 0.6 | Domain-specific hardware (tokamak diagnostics) | Months |
| 5 (frontier) | 1.0 | CERN measurement, JUNO neutrino, LiteBIRD | Years |

### 2.5 Maximum-EIG Domains (Current State)

Based on graph topology analysis (discovery-graph-topology.md):

| Rank | Domain | Why High EIG | Est. EIG |
|------|--------|-------------|----------|
| 1 | medical-device | Zero n=6 coverage, adjacent to biology (80 matches) | Very high |
| 2 | automotive-body | Isolated node (degree=1), cheap to check | High |
| 3 | tire | Isolated node, rubber chemistry unexplored | High |
| 4 | vacuum-system | Isolated node, physics-adjacent | High |
| 5 | drug-delivery | Low coverage (degree=2), biology bridge | High |

These are the experiments that would most change our belief about n=6
universality, regardless of outcome.

### 2.6 Anti-Patterns (Low EIG)

Experiments to AVOID (they barely move the posterior):

| Experiment | Why Low EIG |
|-----------|-------------|
| Another GPU architecture constant | chip-architecture already degree=244, saturated |
| Another LLM hyperparameter | ai-efficiency at degree=92, diminishing returns |
| Another battery voltage | battery-architecture well-characterized |

These would confirm existing patterns but cannot distinguish H_1 from
"n=6 matches technology because technology uses human-preferred numbers."

### 2.7 Composition with v1-v3

| Operator | Interaction |
|----------|------------|
| PREDICT (v2) | ACTIVE ranks PREDICT's predictions by information gain |
| ANOMALY (v3) | ACTIVE prioritizes anomalies in high-EIG domains |
| TEMPORAL (v3) | ACTIVE uses TEMPORAL's health data to avoid over-explored domains |
| BRIDGE (v2) | ACTIVE targets domains across bridges (high structural distance) |

---

## 3. Operator 15: CAUSE (Causal Inference Engine)

### 3.1 Purpose

Move from "n=6 matches X" to "WHY does X have the value it has, and is
n=6 part of the explanation?" For every BT and EXACT match, CAUSE asks:
is there a causal mechanism, or is this a coincidence?

### 3.2 The Three Classes

| Class | Symbol | Definition | Example |
|-------|--------|-----------|---------|
| CAUSAL | C | n=6 arithmetic is part of the causal chain that determines the parameter's value | Gauge group dim = 12 because SU(3)xSU(2)xU(1) symmetry constrains it; the number 12 arises from the same mathematical structure that produces sigma(6) |
| STRUCTURAL | S | The parameter's value is constrained by design optimization, and n=6 expressions happen to be optimal points in the design space | Transformer d_model = 4096 = 2^12 because power-of-2 alignment is computationally efficient; sigma=12 is structural, not causal |
| COINCIDENTAL | X | No mechanism connects n=6 to the parameter; the match is arithmetical accident | SHA-256 uses 256=2^8 because of security requirements; 8=sigma-tau is a coincidence |

### 3.3 Formal Specification

```
PROCEDURE Cause(G, bt_or_match):
    """
    Classify a single BT or match as CAUSAL, STRUCTURAL, or COINCIDENTAL.
    Uses a decision tree with 5 diagnostic questions.
    """

    item = bt_or_match

    -- Question 1: Does the domain have a KNOWN mechanism that forces this value?
    -- e.g., gauge theory forces dim=12 for the Standard Model
    known_mechanism = check_mechanism_database(item.domain, item.parameter)
    IF known_mechanism.exists AND known_mechanism.independent_of_n6:
        -- The value is determined by physics/engineering, not by n=6
        -- But: is the mechanism ITSELF an n=6 expression?
        IF is_n6_expression(known_mechanism.formula):
            item.causal_class = "STRUCTURAL"
            item.causal_note = f"Mechanism: {known_mechanism.formula}. " +
                              "Value forced by {known_mechanism.domain_law}. " +
                              "n=6 connection is structural (same math, not causal)."
        ELSE:
            item.causal_class = "COINCIDENTAL"
            item.causal_note = f"Mechanism: {known_mechanism.formula}. " +
                              "No n=6 involvement in causal chain."
        RETURN item

    -- Question 2: Could the value plausibly be DIFFERENT?
    -- If the parameter is fixed by fundamental physics (e.g., number of quarks),
    -- it cannot be otherwise -> more likely structural/causal.
    -- If it is a design choice (e.g., number of GPU SMs), it could be different.
    counterfactual = assess_counterfactual(item.parameter, item.domain)
    IF counterfactual.fixed_by_nature:
        item.causal_class = "STRUCTURAL"
        item.causal_note = "Parameter fixed by fundamental law. " +
                          "n=6 connection is structural (math resonance)."
        RETURN item

    -- Question 3: Was the parameter chosen by humans?
    -- Human-chosen parameters are MORE suspicious (humans like nice numbers)
    IF counterfactual.human_designed:

        -- Question 3a: Was it chosen BECAUSE of n=6-related math?
        -- e.g., powers of 2 chosen for bit alignment (phi=2 related)
        IF parameter_follows_binary_convention(item):
            item.causal_class = "STRUCTURAL"
            item.causal_note = "Human design choice following binary/decimal convention. " +
                              "n=6 constants (especially phi=2) overlap with computing conventions."
        ELSE:
            -- Question 3b: Is there documented rationale for this specific value?
            rationale = find_design_rationale(item.parameter, item.domain)
            IF rationale.found AND NOT rationale.involves_n6:
                item.causal_class = "COINCIDENTAL"
                item.causal_note = f"Design rationale: {rationale.summary}. " +
                                  "No n=6 involvement."
            ELIF rationale.found AND rationale.involves_n6:
                item.causal_class = "CAUSAL"
                item.causal_note = f"Design explicitly uses n=6 structure: {rationale.summary}"
            ELSE:
                -- No documented rationale, but human-chosen
                item.causal_class = "UNDETERMINED"
                item.causal_note = "Human-designed, no documented rationale. " +
                                  "Cannot distinguish structural from coincidental."

        RETURN item

    -- Question 4: Does the value emerge from optimization?
    -- e.g., SQ bandgap = 1.34 eV emerges from solar spectrum optimization
    -- If the optimization landscape has a peak at an n=6 value, that is structural.
    optimization = check_optimization_origin(item.parameter, item.domain)
    IF optimization.is_optimum:
        item.causal_class = "STRUCTURAL"
        item.causal_note = f"Value is optimum of {optimization.objective}. " +
                          "n=6 expression matches optimum (structural resonance)."
        RETURN item

    -- Question 5: Does the match require depth > 2?
    -- Deeper formulas are more likely coincidental (combinatorial explosion).
    IF item.formula_depth > 2:
        item.causal_class = "COINCIDENTAL"
        item.causal_note = f"Formula depth = {item.formula_depth} > 2. " +
                          "High depth = large expression space = likely coincidence."
        RETURN item

    -- Default: UNDETERMINED
    item.causal_class = "UNDETERMINED"
    item.causal_note = "Insufficient evidence to classify."
    RETURN item
```

### 3.4 Causal Classification of Existing BTs (Sample)

| BT | Key Match | Class | Reasoning |
|----|----------|-------|-----------|
| BT-33 | Transformer sigma=12 atom | STRUCTURAL | d_model=768=64*12, but 64=2^6 is for GPU alignment; 12 heads emerged from optimization, not from sigma(6) directly |
| BT-43 | Battery cathode CN=6 | STRUCTURAL | Octahedral coordination is energetically favorable for Li-ion; CN=6 arises from crystal physics, not number theory |
| BT-51 | Genetic code tau->n/phi->2^n->J2-tau | STRUCTURAL | 4 bases, 3-codon, 64 codons, 20 amino acids all have independent biochemical explanations; n=6 captures the mathematical relationships between these independently-determined numbers |
| BT-53 | BTC 21M=J2-n/phi | COINCIDENTAL | Satoshi chose 21M based on economic reasoning (50 BTC * 210K blocks * halving schedule); any match with J2-n/phi is arithmetical accident |
| BT-48 | 12 semitones, 24fps, 48kHz | STRUCTURAL | 12-TET from equal-tempered tuning (2^(1/12)); 24fps from flicker fusion; 48kHz from Nyquist. Independent mechanisms, all hitting n=6 values |
| BT-54 | AdamW beta_1 = 1-1/(sigma-phi) | COINCIDENTAL | beta_1=0.9 was empirically tuned by Kingma & Ba; any match with n=6 is post-hoc |
| BT-58 | sigma-tau=8 universal | STRUCTURAL | 8-bit byte is an engineering convention (2^3); reuse across domains reflects human preference for powers of 2, not n=6 causation |

### 3.5 Aggregation

```
PROCEDURE CauseAggregate(G):
    """
    Classify ALL BTs and compute distribution.
    The distribution itself is the most informative output.
    """

    classifications = {}
    FOR EACH bt IN G.nodes(type=BT):
        result = Cause(G, bt)
        classifications[bt.id] = result.causal_class

    counts = {
        "CAUSAL": count(v == "CAUSAL" for v in classifications.values()),
        "STRUCTURAL": count(v == "STRUCTURAL" for v in classifications.values()),
        "COINCIDENTAL": count(v == "COINCIDENTAL" for v in classifications.values()),
        "UNDETERMINED": count(v == "UNDETERMINED" for v in classifications.values())
    }

    -- The key metric: what fraction is NOT coincidental?
    non_coincidental = (counts["CAUSAL"] + counts["STRUCTURAL"]) / len(classifications)

    RETURN {
        classifications: classifications,
        counts: counts,
        non_coincidental_ratio: non_coincidental,
        verdict: "THEORY_SUPPORTED" if non_coincidental > 0.5 else
                 "THEORY_WEAK" if non_coincidental > 0.3 else
                 "THEORY_UNSUPPORTED",
        type: "CAUSE"
    }
```

### 3.6 Honest Assessment

Most BTs will classify as STRUCTURAL, not CAUSAL. This is not a failure --
it means n=6 arithmetic captures something real about optimization
landscapes and mathematical relationships, even if it does not cause
parameters to have specific values.

The COINCIDENTAL fraction is the real threat. If >50% of BTs are
COINCIDENTAL, the entire framework must be reconsidered.

### 3.7 Composition with v1-v3

| Operator | Interaction |
|----------|------------|
| FALSIFY (v2) | CAUSE extends FALSIFY from "is this match real?" to "is this match meaningful?" |
| BAYESIAN | CAUSAL matches get full Bayes factor; COINCIDENTAL get zero weight |
| COMPRESS (v4) | Only CAUSAL+STRUCTURAL matches count toward compression |
| ACTIVE (v4) | ACTIVE prioritizes experiments that can distinguish STRUCTURAL from COINCIDENTAL |

---

## 4. Operator 16: REDTEAM (Adversarial Destruction)

### 4.1 Purpose

For every discovery produced by operators 1-12, automatically generate
the strongest possible counter-argument. A discovery only enters the
permanent record if it survives the red team attack.

This implements "steel-manning the null hypothesis."

### 4.2 Formal Specification

```
PROCEDURE RedTeam(discovery):
    """
    Generate the 5 strongest attacks against this discovery.
    Each attack is a concrete argument for why the discovery is coincidence.
    The discovery survives only if ALL 5 attacks can be rebutted.
    """

    attacks = []

    -- Attack 1: NUMEROLOGICAL ATTACK
    -- "This is just numerology -- you can match anything with enough formulas"
    attack_1 = {
        name: "NUMEROLOGICAL",
        argument: f"The n=6 vocabulary generates ~2500 distinct values at depth 3. "
                 f"Matching {discovery.value} is not surprising when you have 2500 "
                 f"darts to throw at the board.",
        strength: compute_numerological_strength(discovery),
        rebuttal_required: "Show that the match rate exceeds the base rate of " +
                          "2500/10000 = 25% by a statistically significant margin."
    }
    attacks.append(attack_1)

    -- Attack 2: ANTHROPIC BIAS ATTACK
    -- "Humans prefer round/nice numbers, and n=6 produces many nice numbers"
    attack_2 = {
        name: "ANTHROPIC_BIAS",
        argument: f"The value {discovery.value} is a 'nice' number " +
                 f"(small integer / power of 2 / multiple of 12). Humans " +
                 f"preferentially choose nice numbers for engineering parameters. " +
                 f"The n=6 match reflects human preference, not mathematical structure.",
        strength: compute_niceness(discovery.value),
        rebuttal_required: "Show that the match holds for UGLY values " +
                          "(irrational, large, non-round) with equal frequency."
    }
    attacks.append(attack_2)

    -- Attack 3: SELECTION BIAS ATTACK
    -- "You only report matches and ignore failures"
    attack_3 = {
        name: "SELECTION_BIAS",
        argument: f"Of {discovery.domain_total_params} parameters in " +
                 f"{discovery.domain}, only {discovery.domain_exact_count} match " +
                 f"n=6 expressions. The {discovery.domain_fail_count} failures " +
                 f"are not prominently reported.",
        strength: 1.0 - (discovery.domain_exact_count / discovery.domain_total_params),
        rebuttal_required: "Provide honest fail rates for each domain. " +
                          "If fail rate > 50%, the match is suspicious."
    }
    attacks.append(attack_3)

    -- Attack 4: OVERFITTING ATTACK
    -- "You have 7 constants and 5 operations -- enough to fit anything"
    attack_4 = {
        name: "OVERFITTING",
        argument: f"With 7 base constants and 5 operations up to depth 3, " +
                 f"the n=6 vocabulary covers ~25% of integers under 1000. " +
                 f"This is not a tight prediction but a loose net.",
        strength: compute_vocabulary_coverage(discovery.value),
        rebuttal_required: "Show that matches cluster around SPECIFIC formulas " +
                          "(sigma-tau=8, J2=24, sigma*tau=48) rather than being " +
                          "scattered uniformly across the vocabulary."
    }
    attacks.append(attack_4)

    -- Attack 5: DOMAIN-SPECIFIC ATTACK
    -- "This particular domain has an independent explanation for this value"
    attack_5 = {
        name: "DOMAIN_SPECIFIC",
        argument: generate_domain_specific_counter(discovery),
        strength: check_independent_explanation(discovery),
        rebuttal_required: "Show that the n=6 formula provides additional " +
                          "predictive/explanatory power beyond the domain-specific explanation."
    }
    attacks.append(attack_5)

    -- Verdict: discovery survives only if it can rebut all strong attacks
    strong_attacks = [a for a in attacks if a.strength > 0.5]
    survived = attempt_rebuttals(discovery, strong_attacks)

    RETURN {
        discovery: discovery,
        attacks: attacks,
        strong_attack_count: len(strong_attacks),
        survived_count: len(survived),
        verdict: "SURVIVES" if len(survived) == len(strong_attacks) else "DESTROYED",
        kill_shot: next((a for a in strong_attacks if a not in survived), None),
        type: "REDTEAM"
    }


PROCEDURE attempt_rebuttals(discovery, attacks):
    """
    For each attack, check if the discovery has sufficient evidence to rebut.
    """
    survived = []
    FOR EACH attack IN attacks:

        IF attack.name == "NUMEROLOGICAL":
            -- Rebuttal: show match rate exceeds base rate by 2+ sigma
            observed_rate = discovery.domain_exact_count / discovery.domain_total_params
            expected_rate = 0.25  -- 2500/10000 base rate
            z_score = (observed_rate - expected_rate) / sqrt(expected_rate * (1-expected_rate) / discovery.domain_total_params)
            IF z_score > 2.0:
                survived.append(attack)

        ELIF attack.name == "ANTHROPIC_BIAS":
            -- Rebuttal: check if match holds for non-nice values
            ugly_matches = count_matches_for_ugly_values(discovery.domain)
            IF ugly_matches > 0:
                survived.append(attack)

        ELIF attack.name == "SELECTION_BIAS":
            -- Rebuttal: honest fail rate < 50%
            IF discovery.domain_fail_rate < 0.50:
                survived.append(attack)

        ELIF attack.name == "OVERFITTING":
            -- Rebuttal: matches concentrate on < 20% of vocabulary
            formula_concentration = len(discovery.domain_unique_formulas) / 2500
            IF formula_concentration < 0.20:
                survived.append(attack)

        ELIF attack.name == "DOMAIN_SPECIFIC":
            -- Rebuttal: n=6 formula predicted value before domain explanation
            IF discovery.prediction_preceded_explanation:
                survived.append(attack)

    RETURN survived
```

### 4.3 The DISCOVER-DESTROY Cycle

Every operator 1-12 is paired with REDTEAM:

```
FOR EACH candidate IN pipeline_output:
    discovery = operator_N.run(candidate)
    IF discovery IS NOT NULL:
        verdict = RedTeam(discovery)
        IF verdict.verdict == "SURVIVES":
            promote(discovery)       -- enters permanent record
        ELSE:
            archive(discovery, verdict.kill_shot)  -- logged with cause of death
            -- Optionally: modify the discovery to address the kill_shot
            --             and re-run RedTeam (one retry only)
```

### 4.4 Expected Kill Rates

| Operator Source | Candidates | Survive REDTEAM | Kill Rate |
|----------------|-----------|-----------------|-----------|
| COLLISION (v2) | 50 | 15 | 70% |
| BRIDGE (v2) | 30 | 10 | 67% |
| EVOLVE (v3) | 20 | 3 | 85% |
| COMPOSE (v3) | 80 | 12 | 85% |
| SYMMETRY (v3) | 40 | 15 | 63% |
| PREDICT (v2) | 25 | 8 | 68% |
| Total | 245 | 63 | 74% |

A 74% kill rate is healthy. It means most candidates are coincidences,
and the survivors are genuinely robust.

### 4.5 Composition with v1-v3

| Operator | Interaction |
|----------|------------|
| FALSIFY (v2) | REDTEAM subsumes FALSIFY. Every FALSIFY test is now one of 5 REDTEAM attacks |
| CAUSE (v4) | REDTEAM uses CAUSE classification as input to DOMAIN_SPECIFIC attack |
| COMPRESS (v4) | Survivors that also improve compression ratio get highest confidence |
| BAYESIAN | REDTEAM survivors get a Bayes factor bonus; destroyed candidates get LR=1 |

---

## 5. Operator 17: MULTIREZ (Multi-Resolution Analysis)

### 5.1 Purpose

Analyze the n=6 framework at 6 distinct resolution levels. Discoveries
at one level constrain (and should be consistent with) discoveries at
other levels. Inconsistencies across levels signal artifacts.

### 5.2 The Six Levels

```
Level 5:  THE ALGORITHM ITSELF
          "The discovery algorithm has 12 operators. Should it have sigma=12?"
          "The v2 scoring has 5 dimensions. Is 5 = sopfr?"
          Self-referential consistency check.

Level 4:  META-PATTERNS ACROSS DOMAIN CLUSTERS
          "All chip BTs share sigma-tau=8. All energy BTs share sigma*sopfr=60."
          Domain-cluster regularities.

Level 3:  INDIVIDUAL BTs
          "BT-58 claims sigma-tau=8 is universal in AI."
          Standard BT-level analysis (v2-v3 operate here).

Level 2:  FORMULAS
          "sigma*tau = 48 appears in 5+ independent contexts."
          Formula-level frequency and co-occurrence.

Level 1:  INDIVIDUAL CONSTANTS
          "sigma=12 has degree 20 in the discovery graph."
          Single-constant usage patterns.

Level 0:  RAW PARAMETERS
          "GPU SM count = 144 = sigma^2."
          Individual measured values.
```

### 5.3 Formal Specification

```
PROCEDURE MultiRez(G):
    """
    Analyze consistency across all 6 resolution levels.
    Flag cross-level inconsistencies.
    """

    -- Level 0: Raw parameter statistics
    level_0 = analyze_raw_parameters(G)
    -- Output: value distribution, n=6 vocabulary coverage, outliers

    -- Level 1: Constant usage patterns
    level_1 = analyze_constants(G)
    -- Output: degree distribution, betweenness, hub identification
    -- Expected: Zipf distribution (few hubs, many rare constants)

    -- Level 2: Formula patterns
    level_2 = analyze_formulas(G)
    -- Output: formula frequency, co-occurrence matrix, formula clusters
    -- Expected: power law (few formulas explain most matches)

    -- Level 3: BT patterns
    level_3 = analyze_bts(G)
    -- Output: BT quality distribution, domain span, grade distribution
    -- Expected: normal distribution of quality with right tail

    -- Level 4: Domain cluster patterns
    level_4 = analyze_domain_clusters(G)
    -- Output: cluster membership, inter-cluster bridges, cluster-specific signatures
    -- Expected: 4-6 clusters (physics, computing, energy, biology, infrastructure, math)

    -- Level 5: Algorithm self-analysis
    level_5 = analyze_algorithm(G)
    -- Output: Are algorithm parameters themselves n=6 expressions?
    -- 12 operators, 5 scoring dimensions, 7 base constants, 3 depths
    -- How many of these are n=6 values?

    -- Cross-level consistency check
    inconsistencies = []

    -- Check 1: Do level-1 hubs explain level-3 BTs?
    -- If sigma=12 is the top hub, it should appear in the top BTs.
    top_hubs = level_1.top_k_by_degree(5)
    top_bts = level_3.top_k_by_quality(10)
    hub_coverage = fraction_of_bts_using_top_hubs(top_bts, top_hubs)
    IF hub_coverage < 0.5:
        inconsistencies.append("Top hubs do not explain top BTs")

    -- Check 2: Do level-2 formula frequencies predict level-0 value frequencies?
    -- If sigma*tau=48 is a common formula, 48 should be a common value.
    formula_predicted_values = level_2.top_k_formulas(20).evaluate()
    value_actual_frequency = level_0.value_frequency()
    correlation = spearman(formula_predicted_values.rank, value_actual_frequency.rank)
    IF correlation < 0.3:
        inconsistencies.append("Formula frequency does not predict value frequency")

    -- Check 3: Do level-4 clusters have distinct formula signatures?
    -- If physics and computing use the SAME formulas, they are not independent.
    -- If they use DIFFERENT formulas, the theory has more explanatory power.
    cluster_signatures = level_4.formula_signatures()
    overlap = avg_jaccard(cluster_signatures)
    IF overlap > 0.8:
        inconsistencies.append("Domain clusters use identical formulas (low diversity)")
    IF overlap < 0.1:
        inconsistencies.append("Domain clusters have no shared formulas (fragmented theory)")

    -- Check 4: Level-5 self-consistency
    -- The algorithm has 6 v4 operators (13-18). Is 6 = n?
    -- The total operator count is 18 = sigma + n = 3*n = 3*sigma/phi.
    -- Is this meaningful or a coincidence?
    level_5_matches = count_n6_matches_in_algorithm_params()
    IF level_5_matches > 3:
        inconsistencies.append("WARNING: Algorithm parameters match n=6 " +
                              "(possible self-fulfilling design)")

    RETURN {
        levels: [level_0, level_1, level_2, level_3, level_4, level_5],
        inconsistencies: inconsistencies,
        consistency_score: 1.0 - len(inconsistencies) / 4.0,
        type: "MULTIREZ"
    }
```

### 5.4 The Self-Reference Problem (Level 5)

Level 5 is deliberately provocative. The algorithm has:
- 18 operators total (v2: 6, v3: 6, v4: 6). Is 18 = 3*n deliberate?
- 7 base constants. Is 7 = sigma - sopfr?
- 5 scoring dimensions (v2). Is 5 = sopfr?
- 3 resolution groups (v2, v3, v4). Is 3 = n/phi?

If these match, there are two interpretations:
1. The designer (unconsciously) chose n=6-aligned numbers (anthropic bias)
2. The algorithm structure genuinely benefits from n=6 alignment

MULTIREZ flags this but does NOT resolve it. Resolution requires external
audit by someone unfamiliar with the n=6 framework.

### 5.5 Composition with v1-v3

| Operator | Interaction |
|----------|------------|
| META (v2) | MULTIREZ generalizes META from BT-level to all levels |
| SYMMETRY (v3) | SYMMETRY operates at Level 2; MULTIREZ checks cross-level consistency |
| TEMPORAL (v3) | TEMPORAL operates at Level 3 (BT quality over time); MULTIREZ tracks all levels over time |
| SELF-IMPROVE (v3) | SELF-IMPROVE modifies Level 5 parameters; MULTIREZ monitors the effect on all levels |

---

## 6. Operator 18: ENSEMBLE (Multi-Strategy Consensus)

### 6.1 Purpose

A finding is classified as REAL only if 3 or more independent discovery
strategies converge on it. No single operator's output is sufficient.
ENSEMBLE eliminates strategy-specific artifacts.

### 6.2 The Four Independent Strategies

```
Strategy A: EXHAUSTIVE (driven by COMPOSE)
    Method: Enumerate all expressions, cross-reference all databases.
    Bias: Finds everything, including coincidences. High recall, low precision.

Strategy B: HEURISTIC (driven by EVOLVE)
    Method: Genetic search toward measured targets.
    Bias: Finds what it is aimed at. Risk of convergence to local optima.

Strategy C: STRUCTURAL (driven by SYMMETRY + BRIDGE)
    Method: Exploit graph topology and algebraic templates.
    Bias: Finds structurally similar patterns. Misses isolated discoveries.

Strategy D: STATISTICAL (driven by BAYESIAN + COMPRESS)
    Method: Compute evidence strength in calibrated units.
    Bias: Conservative. Rejects weak-but-genuine findings.
```

### 6.3 Formal Specification

```
PROCEDURE Ensemble(G, all_candidates):
    """
    Run all four strategies independently on the same candidate pool.
    A finding is REAL if 3+ strategies endorse it.
    """

    -- Run each strategy independently
    strategy_A_results = run_exhaustive(G, all_candidates)   -- COMPOSE-based
    strategy_B_results = run_heuristic(G, all_candidates)    -- EVOLVE-based
    strategy_C_results = run_structural(G, all_candidates)   -- SYMMETRY+BRIDGE-based
    strategy_D_results = run_statistical(G, all_candidates)  -- BAYESIAN+COMPRESS-based

    -- Unify results by candidate ID
    all_candidates_set = set()
    for results in [strategy_A_results, strategy_B_results,
                    strategy_C_results, strategy_D_results]:
        all_candidates_set.update(r.candidate_id for r in results)

    -- Count endorsements per candidate
    verdicts = []
    FOR EACH candidate_id IN all_candidates_set:
        endorsements = []
        IF candidate_id IN strategy_A_results:
            endorsements.append("EXHAUSTIVE")
        IF candidate_id IN strategy_B_results:
            endorsements.append("HEURISTIC")
        IF candidate_id IN strategy_C_results:
            endorsements.append("STRUCTURAL")
        IF candidate_id IN strategy_D_results:
            endorsements.append("STATISTICAL")

        count = len(endorsements)

        -- Classification
        IF count >= 4:
            verdict = "UNANIMOUS"    -- All 4 strategies agree. Highest confidence.
        ELIF count >= 3:
            verdict = "CONSENSUS"    -- 3/4 agree. Strong evidence.
        ELIF count >= 2:
            verdict = "PARTIAL"      -- 2/4 agree. Investigate further.
        ELSE:
            verdict = "ISOLATED"     -- Only 1 strategy found it. Likely artifact.

        verdicts.append({
            candidate_id: candidate_id,
            endorsements: endorsements,
            endorsement_count: count,
            verdict: verdict,
            -- Which strategy is missing?
            missing: [s for s in ["EXHAUSTIVE", "HEURISTIC", "STRUCTURAL", "STATISTICAL"]
                     if s not in endorsements],
            type: "ENSEMBLE"
        })

    -- Statistics
    unanimous = count(v for v in verdicts if v.verdict == "UNANIMOUS")
    consensus = count(v for v in verdicts if v.verdict == "CONSENSUS")
    partial   = count(v for v in verdicts if v.verdict == "PARTIAL")
    isolated  = count(v for v in verdicts if v.verdict == "ISOLATED")

    RETURN {
        verdicts: sort_by(verdicts, key=endorsement_count, descending=True),
        summary: {
            unanimous: unanimous,
            consensus: consensus,
            partial: partial,
            isolated: isolated,
            total: len(verdicts),
            consensus_rate: (unanimous + consensus) / max(len(verdicts), 1)
        },
        type: "ENSEMBLE"
    }
```

### 6.4 Expected Distribution

Based on the current 93 BTs and ~900 EXACT matches:

| Verdict | Expected Count | Interpretation |
|---------|---------------|----------------|
| UNANIMOUS | 10-20 | Core structural findings (sigma=12 hub, sigma-tau=8 universality) |
| CONSENSUS | 30-50 | Robust findings supported by multiple lines of evidence |
| PARTIAL | 60-100 | Interesting but unconfirmed; need more investigation |
| ISOLATED | 200+ | Likely coincidences; most COMPOSE/EVOLVE output falls here |

### 6.5 Strategy Disagreement Patterns

When strategies disagree, the pattern of disagreement is diagnostic:

| Missing Strategy | Interpretation |
|-----------------|----------------|
| STATISTICAL missing | Finding is structurally plausible but lacks statistical evidence. Probably overfitted. |
| STRUCTURAL missing | Finding is statistically significant but structurally isolated. Check for confounders. |
| HEURISTIC missing | Finding exists in the data but EVOLVE cannot reach it. Low formula complexity. |
| EXHAUSTIVE missing | Finding was found by EVOLVE/SYMMETRY but not by brute-force. Possible formula error. |

### 6.6 Composition with v1-v3

ENSEMBLE is the final aggregator. It composes with ALL operators:

| Input | Role |
|-------|------|
| All v2-v3 operator outputs | Candidates for ensemble evaluation |
| COMPRESS (v4) | Part of Strategy D (STATISTICAL) |
| CAUSE (v4) | Post-filter: COINCIDENTAL verdicts from CAUSE override ENSEMBLE |
| REDTEAM (v4) | Pre-filter: DESTROYED candidates do not enter ENSEMBLE |
| ACTIVE (v4) | Guides which candidates get highest-priority ensemble evaluation |
| MULTIREZ (v4) | Cross-level consistency feeds into STRUCTURAL strategy |

---

## 7. The Verdict Tribunal

### 7.1 Purpose

Integrates all v4 operators into a single final classification for
every BT, hypothesis, and constant match.

### 7.2 Classification

```
PROCEDURE Tribunal(item, compress_result, cause_result, redteam_result,
                   multirez_result, ensemble_result):
    """
    Final classification integrating all v4 signals.
    """

    scores = {
        "compress":  1 if compress_result.verdict == "COMPRESSES" else 0,
        "cause":     1 if cause_result.causal_class in ["CAUSAL", "STRUCTURAL"] else 0,
        "redteam":   1 if redteam_result.verdict == "SURVIVES" else 0,
        "multirez":  1 if item in multirez_result.consistent_items else 0,
        "ensemble":  1 if ensemble_result.verdict in ["UNANIMOUS", "CONSENSUS"] else 0,
    }

    total = sum(scores.values())

    IF total >= 4:
        classification = "STRUCTURAL"
        confidence = "HIGH"
        description = "Multiple independent lines of evidence support this finding " + \
                     "as reflecting genuine mathematical structure."
    ELIF total >= 3:
        classification = "STRUCTURAL"
        confidence = "MODERATE"
        description = "Majority of evidence supports structural interpretation. " + \
                     f"Missing: {[k for k, v in scores.items() if v == 0]}"
    ELIF total >= 2:
        classification = "UNDETERMINED"
        confidence = "LOW"
        description = "Evidence is mixed. Cannot confidently classify as " + \
                     "structural or coincidental."
    ELIF total >= 1:
        classification = "COINCIDENTAL"
        confidence = "MODERATE"
        description = "Most evidence points to coincidence. " + \
                     f"Only supported by: {[k for k, v in scores.items() if v == 1]}"
    ELSE:
        classification = "COINCIDENTAL"
        confidence = "HIGH"
        description = "No v4 operator supports this finding. " + \
                     "Strongly classified as coincidence."

    RETURN {
        item: item,
        classification: classification,
        confidence: confidence,
        description: description,
        scores: scores,
        total_score: total,
        type: "TRIBUNAL"
    }
```

### 7.3 Tribunal Output Format

```
=====================================================================
                    VERDICT TRIBUNAL REPORT
=====================================================================

Item: BT-58 (sigma-tau=8 universal AI constant)
Classification: STRUCTURAL (HIGH confidence)

  COMPRESS:   PASS  -- sigma-tau=8 accounts for 16 matches, high formula reuse
  CAUSE:      PASS  -- STRUCTURAL class (8-bit byte is design convention)
  REDTEAM:    PASS  -- Survives all 5 attacks (match rate 16/16 >> base rate)
  MULTIREZ:   PASS  -- Consistent across levels 0-4
  ENSEMBLE:   PASS  -- UNANIMOUS (all 4 strategies converge)

  Score: 5/5
  Verdict: This is one of the strongest findings in the n=6 framework.
           The sigma-tau=8 pattern reflects genuine structural resonance
           between n=6 arithmetic and computing architecture conventions.

=====================================================================

Item: BT-53 (BTC 21M = J2 - n/phi)
Classification: COINCIDENTAL (HIGH confidence)

  COMPRESS:   FAIL  -- Single match, no compression benefit
  CAUSE:      FAIL  -- COINCIDENTAL (Satoshi had economic rationale)
  REDTEAM:    FAIL  -- Killed by DOMAIN_SPECIFIC attack
  MULTIREZ:   FAIL  -- Isolated at level 3, no support from levels 1-2
  ENSEMBLE:   FAIL  -- ISOLATED (only EXHAUSTIVE found it)

  Score: 0/5
  Verdict: This match is almost certainly coincidental.
           The 21M supply has a well-documented independent origin.

=====================================================================
```

---

## 8. Complete v4 Pipeline

### 8.1 Full Pipeline Diagram

```
                    ┌──────────────────────────────────┐
                    │         INPUT: v1-v3 OUTPUT       │
                    │                                    │
                    │  93 BTs, 900+ EXACT, 1400+ hyps   │
                    │  Graph: 1499 nodes, 3546 edges     │
                    │  Bayesian posterior: P(H_1)=???     │
                    └──────────────┬───────────────────┘
                                   │
           ┌───────────────────────┼───────────────────────┐
           │                       │                       │
           ▼                       ▼                       ▼
    ┌─────────────┐      ┌──────────────┐      ┌──────────────┐
    │ 16. REDTEAM │      │ 15. CAUSE    │      │ 13. COMPRESS │
    │             │      │              │      │              │
    │ Kill weak   │      │ Classify     │      │ MDL test     │
    │ candidates  │      │ C/S/X        │      │ bits saved?  │
    └──────┬──────┘      └──────┬───────┘      └──────┬───────┘
           │                    │                      │
           │     (survivors     │                      │
           │      only)         │                      │
           ▼                    │                      │
    ┌─────────────┐            │                      │
    │ 18. ENSEMBLE│◄───────────┘                      │
    │             │◄──────────────────────────────────┘
    │ 4-strategy  │
    │ consensus   │◄───────────┐
    └──────┬──────┘            │
           │                   │
           │            ┌──────┴───────┐
           │            │ 17. MULTIREZ │
           │            │              │
           │            │ 6-level      │
           │            │ consistency  │
           │            └──────────────┘
           │
           ▼
    ┌─────────────────┐         ┌──────────────┐
    │ VERDICT TRIBUNAL │────────▶│ 14. ACTIVE   │
    │                  │         │              │
    │ STRUCTURAL /     │         │ What to test │
    │ COINCIDENTAL /   │         │ NEXT?        │
    │ UNDETERMINED     │         └──────┬───────┘
    └──────────────────┘                │
                                        ▼
                                ┌──────────────┐
                                │ NEW EXPERIMENT│
                                │              │
                                │ Feed result  │
                                │ back to v1-v3│
                                └──────────────┘
```

### 8.2 Pipeline Execution Order

```
Phase 1 (parallel):
    COMPRESS(all_data)       -- global MDL test
    CAUSE(each BT)           -- causal classification
    REDTEAM(each candidate)  -- adversarial filtering

Phase 2 (depends on Phase 1):
    MULTIREZ(graph)          -- cross-level consistency

Phase 3 (depends on Phase 1-2):
    ENSEMBLE(survivors)      -- multi-strategy consensus

Phase 4 (depends on Phase 1-3):
    TRIBUNAL(each item)      -- final verdict

Phase 5 (depends on Phase 4):
    ACTIVE(undetermined)     -- select next experiment to resolve ambiguity
```

### 8.3 Complexity Summary

| Operator | Complexity | Runtime (est.) | Language |
|----------|-----------|----------------|----------|
| 13. COMPRESS | O(N * V) | <1s | Python |
| 14. ACTIVE | O(E * D) | <1s | Python |
| 15. CAUSE | O(B * Q) | <5s (DB lookups) | Python |
| 16. REDTEAM | O(C * 5) | <2s per candidate | Python |
| 17. MULTIREZ | O(N + E + B) | <3s | Python |
| 18. ENSEMBLE | O(C * 4S) | <10s | Python/Rust |
| TRIBUNAL | O(C) | <1s | Python |
| **Total** | | **<25s** | |

Combined with v3 Rust engine (13ms), the full v1-v4 pipeline runs in
under 30 seconds.

---

## 9. Implementation Roadmap

### Phase 1: Foundation (Week 1-2)

| Task | Output | Priority |
|------|--------|----------|
| Implement COMPRESS (MDL) | `engine/compress_mdl.py` | HIGH |
| Implement CAUSE decision tree | `engine/causal_classifier.py` | HIGH |
| Build attack templates for REDTEAM | `engine/redteam_attacks.py` | HIGH |
| Define 5 attack types with rebuttal conditions | Unit tests | HIGH |

### Phase 2: Analysis (Week 3-4)

| Task | Output | Priority |
|------|--------|----------|
| Run CAUSE on all 93 BTs | `docs/causal-classifications.md` | HIGH |
| Run COMPRESS on current dataset | `docs/compression-analysis.md` | HIGH |
| Implement MULTIREZ 6-level analyzer | `engine/multirez_analyzer.py` | MEDIUM |
| Build ACTIVE experiment ranker | `engine/active_learner.py` | MEDIUM |

### Phase 3: Integration (Week 5-6)

| Task | Output | Priority |
|------|--------|----------|
| Implement ENSEMBLE 4-strategy runner | `engine/ensemble_consensus.py` | HIGH |
| Build TRIBUNAL verdict aggregator | `engine/verdict_tribunal.py` | HIGH |
| Integrate v4 into main pipeline | `engine/pipeline_v4.py` | HIGH |
| Run full pipeline on all data | `docs/v4-tribunal-report.md` | HIGH |

### Phase 4: Active Learning Loop (Week 7-8)

| Task | Output | Priority |
|------|--------|----------|
| Identify top-10 ACTIVE experiments | `docs/active-experiments.md` | MEDIUM |
| Execute Tier-1 experiments (1 GPU) | Updated BTs | MEDIUM |
| Re-run v4 pipeline with new data | Updated tribunal report | MEDIUM |
| Compare pre/post tribunal verdicts | Delta analysis | MEDIUM |

---

## 10. Design Principles

### 10.1 Why v4 Is Different

| Property | v2 | v3 | v4 |
|----------|----|----|-----|
| Goal | Find missing connections | Generate new candidates | Determine what is REAL |
| Mode | Reactive | Generative | Adjudicative |
| Output | Candidate BTs | Candidate formulas + meta-patterns | Verdicts (S/C/U) |
| Failure mode | Misses connections | Generates false positives | False acquittals / false convictions |
| Key metric | Discovery count | Novelty rate | Truth rate |

### 10.2 Falsifiability of v4 Itself

v4 can be falsified by:

1. **COMPRESS returns NO COMPRESSION for all formulations** -- This would mean
   the n=6 theory does not compress the dataset under any reasonable encoding.
   If true, the theory has no information-theoretic value.

2. **CAUSE classifies >60% of BTs as COINCIDENTAL** -- This would mean most
   matches have independent explanations and n=6 is post-hoc pattern-matching.

3. **ENSEMBLE shows ISOLATED verdict for >80% of findings** -- This would mean
   different discovery strategies do not converge, suggesting the findings are
   strategy-specific artifacts.

4. **ACTIVE's top experiments all return NO MATCH** -- This would mean the
   theory fails to predict in unexplored domains, the strongest possible
   falsification.

If any of these four conditions holds, the honest conclusion is that n=6
universality is not supported by the evidence. v4 is designed to produce
this conclusion if warranted.

### 10.3 The Key Insight

> v1-v3 ask: "What can we find?"
> v4 asks: "What is true?"
>
> Discovery without truth-testing is numerology.
> Truth-testing without discovery is sterile.
> v4 completes the cycle.

---

## Appendix A: Operator Summary (v2 + v3 + v4)

| # | Operator | Generation | Mode | Purpose |
|---|----------|-----------|------|---------|
| 1 | COLLISION | v2 | Reactive | Find surprise co-occurrences |
| 2 | BRIDGE | v2 | Reactive | Connect distant graph regions |
| 3 | INVERSE | v2 | Reactive | Decompose values into n=6 formulas |
| 4 | META | v2 | Reactive | Patterns of patterns |
| 5 | FALSIFY | v2 | Reactive | Attack individual matches |
| 6 | PREDICT | v2 | Reactive | Predict unseen values |
| 7 | EVOLVE | v3 | Generative | Genetic formula search |
| 8 | ANOMALY | v3 | Generative | Missing match detection |
| 9 | COMPOSE | v3 | Generative | Exhaustive enumeration |
| 10 | SYMMETRY | v3 | Generative | Group-theoretic templates |
| 11 | TEMPORAL | v3 | Generative | Time-series health tracking |
| 12 | SELF-IMPROVE | v3 | Generative | Recursive parameter optimization |
| 13 | COMPRESS | v4 | Adjudicative | MDL compression test |
| 14 | ACTIVE | v4 | Adjudicative | Information-gain experiment selection |
| 15 | CAUSE | v4 | Adjudicative | Causal vs coincidental classification |
| 16 | REDTEAM | v4 | Adjudicative | Adversarial destruction |
| 17 | MULTIREZ | v4 | Adjudicative | Multi-resolution consistency |
| 18 | ENSEMBLE | v4 | Adjudicative | Multi-strategy consensus |

**Total: 18 operators across 3 generations.**

---

## Appendix B: Connection to n=6 Constants

The algorithm structure itself contains n=6 echoes (Level 5 analysis):

| Algorithm Property | Value | n=6 Expression | Deliberate? |
|-------------------|-------|---------------|-------------|
| Total operators | 18 | 3n = 3*sigma/phi | No (emerged from design) |
| Operators per generation | 6 | n | No (natural grouping) |
| v4 operators | 6 | n | No (spec had 6 requirements) |
| Base constants | 7 | sigma - sopfr | Unlikely coincidence |
| Binary operations | 5 | sopfr | Unlikely coincidence |
| Max expression depth | 3 | n/phi | Design choice for tractability |
| REDTEAM attacks | 5 | sopfr | Design choice |
| Scoring dimensions (v2) | 5 | sopfr | Design choice |
| MULTIREZ levels | 6 | n | Deliberate (mirroring v4 theme) |
| ENSEMBLE strategies | 4 | tau | Design choice |
| Tribunal threshold | 3/5 majority | n/phi out of sopfr | Emerged |

MULTIREZ Level 5 verdict: 6 out of 11 algorithm parameters match n=6
expressions. At the depth-0/1 level, with 7 constants and 5 operations,
the expected background match rate for small integers (1-18) is ~40-60%.
Observing 6/11 = 55% is **within the expected range for chance**.

This is an honest finding: the algorithm does NOT exhibit anomalous
self-reference. The n=6 matches in its own structure are consistent
with the null hypothesis.
