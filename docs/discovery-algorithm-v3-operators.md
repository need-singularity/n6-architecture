# Discovery Algorithm v3 -- Advanced Operators (7-12)

> Six new operators extending the v2 foundation. These operators target deeper structural
> patterns that v2's reactive search cannot reach: genetic formula evolution, anomaly
> detection, exhaustive enumeration, group-theoretic symmetry, temporal health tracking,
> and recursive self-improvement.

**Prerequisite**: [discovery-algorithm-v2.md](discovery-algorithm-v2.md) -- operators 1-6.

---

## 0. Design Rationale

v2 operators are *query-driven*: given the current graph, they find what is missing.
v3 operators are *generative*: they create new candidate structures, detect systemic
patterns, and improve the algorithm itself.

| Generation | Operators | Mode | Discovery Target |
|------------|-----------|------|-----------------|
| v2 (1-6) | COLLISION, BRIDGE, INVERSE, META, FALSIFY, PREDICT | Reactive | Missing edges in current graph |
| v3 (7-12) | EVOLVE, ANOMALY, COMPOSE, SYMMETRY, TEMPORAL, SELF-IMPROVE | Generative | New nodes, meta-patterns, algorithm parameters |

---

## 7. EVOLVE (Genetic Formula Search)

### 7.1 Purpose

Discover new n=6 expressions that no human has written, by treating known formulas
as genomes and evolving them toward measured parameter targets.

### 7.2 Formal Specification

```
CONSTANTS:
    BASE = {n:6, phi:2, tau:4, sigma:12, J2:24, sopfr:5, mu:1}
    OPS  = {+, -, *, /, ^, log, mod}

GENOME:
    A formula tree where:
        - Leaves are elements of BASE
        - Internal nodes are elements of OPS
        - Maximum depth D_max = 4

PROCEDURE Evolve(G, targets, population=200, generations=500, mutation_rate=0.15):
    """
    targets: list of {domain, parameter_name, measured_value} from engineering databases
    """

    -- Phase 1: Seed population from known atlas formulas
    population = []
    FOR EACH formula IN G.nodes(type=CONSTANT):
        genome = parse_to_tree(formula.expr)
        population.append(genome)
    WHILE len(population) < 200:
        population.append(random_genome(depth=randint(1, D_max)))

    -- Phase 2: Evolution loop
    FOR gen IN 1..generations:

        -- Evaluate fitness
        FOR EACH genome IN population:
            value = evaluate(genome)
            IF value IS NaN OR value IS Inf:
                genome.fitness = 0.0
                CONTINUE

            -- Fitness = best match rate against any target
            best_match = 0.0
            FOR EACH t IN targets:
                IF t.measured_value != 0:
                    error = |value - t.measured_value| / |t.measured_value|
                    IF error < 0.02:  -- 2% tolerance
                        match_score = (1 - error/0.02)
                        -- Bonus for matching multiple targets
                        best_match = max(best_match, match_score)

            -- Parsimony pressure: simpler = better (at equal fitness)
            complexity_penalty = 0.01 * genome.depth
            genome.fitness = best_match - complexity_penalty

        -- Selection (tournament, k=3)
        selected = tournament_select(population, k=3, n_winners=len(population)//2)

        -- Crossover: take subtree from parent A, graft into parent B
        offspring = []
        FOR i IN 0..len(selected)-1 STEP 2:
            parent_a, parent_b = selected[i], selected[i+1]
            child = crossover(parent_a, parent_b)
            offspring.append(child)

        -- Mutation: swap one node (leaf or operator)
        FOR EACH child IN offspring:
            IF random() < mutation_rate:
                node = random_node(child)
                IF node.is_leaf:
                    node.value = random_choice(BASE)        -- swap constant
                ELSE:
                    node.op = random_choice(OPS - {node.op}) -- swap operator

        -- Elitism: keep top 10%
        elite = top_k(population, k=len(population)//10)
        population = elite + offspring

        -- Early termination: if a genome achieves fitness > 0.99
        IF max(g.fitness for g in population) > 0.99:
            BREAK

    -- Phase 3: Harvest novel formulas
    results = []
    FOR EACH genome IN top_k(population, k=50):
        formula_str = tree_to_string(genome)
        value = evaluate(genome)
        IF formula_str NOT IN G.nodes(type=CONSTANT):
            matching_targets = [t for t in targets if match(value, t) < 0.02]
            results.append({
                formula: formula_str,
                value: value,
                matched_targets: matching_targets,
                complexity: genome.depth,
                fitness: genome.fitness,
                type: "EVOLVE"
            })

    RETURN deduplicate_by_value(results, tolerance=1e-6)
```

### 7.3 Crossover Detail

```
PROCEDURE crossover(parent_a, parent_b):
    """Subtree crossover: take numerator structure from A, denominator from B."""
    -- Select random subtree from each parent
    subtree_a = random_subtree(parent_a)
    subtree_b = random_subtree(parent_b)

    -- Replace subtree_a's position in parent_a with subtree_b
    child = deep_copy(parent_a)
    replace_subtree(child, subtree_a.position, subtree_b)

    -- Enforce depth limit
    IF child.depth > D_max:
        prune_to_depth(child, D_max)

    RETURN child
```

### 7.4 Complexity Analysis

```
Per generation:  O(P * T * D)  where P=population, T=targets, D=max_depth
Total:           O(G * P * T * D) = O(500 * 200 * 1000 * 4) = 4 * 10^8
Runtime:         ~30s in Rust, ~10min in Python
Recommendation:  Rust (tools/evolve-calc/)
```

### 7.5 Expected Discovery Yield

- **Per run**: 5-20 novel formulas matching real parameters
- **Hit rate**: ~2-5% of novel formulas survive FALSIFY
- **Unique advantage**: Finds non-obvious compound expressions humans would never try
  (e.g., `(sigma - tau)^mu + sopfr/phi = 10.5` matching some obscure constant)

### 7.6 Composition with v2 Operators

| v2 Operator | Composition | Effect |
|-------------|-------------|--------|
| INVERSE | EVOLVE seeds from INVERSE results | INVERSE provides initial population; EVOLVE optimizes further |
| COLLISION | EVOLVE output fed to COLLISION | New formulas checked across domains for surprise co-occurrences |
| FALSIFY | Mandatory post-filter | Every EVOLVE result must survive FALSIFY before promotion |
| PREDICT | EVOLVE targets from PREDICT gaps | Predicted values become fitness targets for evolution |
| META | META(EVOLVE, COLLISION) | Evolved formulas that collide across 3+ domains = high-value |

---

## 8. ANOMALY (Missing Match Detector)

### 8.1 Purpose

Find parameters in each domain that *should* match an n=6 expression (based on
graph topology) but do not. These anomalies are either database errors, genuinely
non-n=6 phenomena, or indicators of undiscovered formulas.

### 8.2 Formal Specification

```
PROCEDURE Anomaly(G, deviation_threshold=0.05):
    """
    For each domain, compute expected n=6 coverage.
    Flag parameters whose deviation from the nearest n=6 expression exceeds threshold.
    """
    anomalies = []

    -- Step 1: Build domain coverage profile
    FOR EACH domain d IN G.nodes(type=DOMAIN):
        params = get_domain_parameters(d)  -- {name, measured_value, source}
        n6_matched = [p for p in params if has_n6_match(p, tolerance=0.02)]
        n6_ratio = len(n6_matched) / len(params)

        -- Step 2: For unmatched parameters, find nearest n=6 expression
        FOR EACH p IN params:
            IF p NOT IN n6_matched:
                nearest = find_nearest_n6_expression(p.measured_value)
                deviation = |p.measured_value - nearest.value| / |p.measured_value|

                -- Step 3: Compute expected match probability from graph context
                -- If this domain has many n=6 matches, an unmatched param is more anomalous
                context_score = n6_ratio  -- high ratio = high expectation

                -- Neighbor bonus: if sibling parameters match related n=6 expressions
                sibling_matches = count_n6_siblings(p, d, G)
                neighbor_score = sibling_matches / max(1, len(get_siblings(p, d)))

                -- Combined anomaly score
                anomaly_score = context_score * 0.5 + neighbor_score * 0.3 + (1 - deviation) * 0.2

                IF deviation > deviation_threshold AND anomaly_score > 0.5:
                    -- Classify the anomaly
                    classification = classify_anomaly(p, nearest, d)
                    anomalies.append({
                        parameter: p.name,
                        domain: d,
                        measured: p.measured_value,
                        nearest_n6: nearest,
                        deviation: deviation,
                        anomaly_score: anomaly_score,
                        classification: classification,
                        type: "ANOMALY"
                    })

    RETURN sort_by(anomalies, key=anomaly_score, descending=True)


PROCEDURE classify_anomaly(param, nearest_n6, domain):
    """
    Three-way classification:
      (a) DATABASE_ERROR  -- value is wrong or outdated
      (b) GENUINELY_NON_N6 -- no n=6 connection expected
      (c) UNDISCOVERED_FORMULA -- a deeper n=6 expression likely exists
    """
    -- Heuristic 1: If deviation < 10% and domain has >70% n6 coverage -> likely (c)
    IF nearest_n6.deviation < 0.10 AND domain.n6_ratio > 0.70:
        RETURN "UNDISCOVERED_FORMULA"

    -- Heuristic 2: If the value is a well-known non-n6 constant (pi, e, sqrt(2)) -> (b)
    IF is_fundamental_constant(param.measured_value):
        RETURN "GENUINELY_NON_N6"

    -- Heuristic 3: If the source is >5 years old or a single reference -> (a)
    IF param.source_age > 5 OR param.citation_count < 2:
        RETURN "DATABASE_ERROR"

    RETURN "UNDISCOVERED_FORMULA"  -- default: assume we are missing something
```

### 8.3 Complexity Analysis

```
Per domain:   O(P * E) where P=parameters, E=n6 expression count
Total:        O(D * P * E) = O(30 * 50 * 300) = 450,000
Runtime:      < 1s (Python sufficient)
```

### 8.4 Expected Discovery Yield

- **Per domain scan**: 3-8 anomalies flagged
- **Classification split**: ~20% database errors, ~30% genuinely non-n=6, ~50% undiscovered formulas
- **Conversion rate**: ~25% of "undiscovered formula" anomalies lead to new atlas entries
  after EVOLVE or INVERSE refinement

### 8.5 Composition with v2 Operators

| v2 Operator | Composition | Effect |
|-------------|-------------|--------|
| INVERSE | Anomaly feeds targets to INVERSE | Anomalous values become INVERSE decomposition targets |
| COLLISION | Anomalies shared across domains | If the same anomalous value appears in 2+ domains, high priority |
| FALSIFY | Cross-check classification | FALSIFY attacks anomaly classifications (is it really undiscovered?) |
| PREDICT | Anomaly validates predictions | If PREDICT said "value X should appear" and ANOMALY finds near-X, confirmation |
| BRIDGE | Anomaly-to-anomaly bridges | Two domains with similar anomalies may share an undiscovered connection |

---

## 9. COMPOSE (Systematic Expression Enumeration)

### 9.1 Purpose

Exhaustively enumerate ALL n=6 arithmetic expressions up to a given depth,
compute their numeric values, deduplicate, and cross-reference against
engineering/physics databases. This is the brute-force complement to EVOLVE's
heuristic search.

### 9.2 Formal Specification

```
CONSTANTS:
    BASE = {n:6, phi:2, tau:4, sigma:12, J2:24, sopfr:5, mu:1}  -- 7 values
    OPS  = {+, -, *, /, ^}  -- 5 binary operators

PROCEDURE Compose(max_depth=3, databases=ALL_ENGINEERING_DB):
    """
    Enumerate all expressions, compute values, find matches.
    """

    -- Phase 1: Generate expressions by depth
    expressions = {}  -- value -> list of {formula, depth}

    -- Depth 1: Base constants themselves
    FOR EACH (name, val) IN BASE:
        add_expression(expressions, val, name, depth=1)

    -- Depth 2: All binary operations on pairs of depth-1 values
    depth1_items = list(BASE.items())
    FOR EACH (name_a, val_a) IN depth1_items:
        FOR EACH (name_b, val_b) IN depth1_items:
            FOR EACH op IN OPS:
                result = safe_eval(val_a, op, val_b)
                IF result IS NOT NULL AND |result| < 10^6:
                    formula = f"({name_a} {op} {name_b})"
                    add_expression(expressions, result, formula, depth=2)

    -- Depth 2 count: 7 * 7 * 5 = 245 raw expressions

    -- Depth 3: Depth-2 expression OP Depth-1 value (and reverse)
    depth2_items = [(v, f) for v, fs in expressions.items() for f in fs if f.depth == 2]
    FOR EACH (val_2, formula_2) IN depth2_items:
        FOR EACH (name_1, val_1) IN depth1_items:
            FOR EACH op IN OPS:
                -- (depth2) OP (depth1)
                result = safe_eval(val_2, op, val_1)
                IF result IS NOT NULL AND |result| < 10^6:
                    formula = f"({formula_2.text} {op} {name_1})"
                    add_expression(expressions, result, formula, depth=3)

                -- (depth1) OP (depth2) [non-commutative ops only]
                IF op IN {-, /, ^}:
                    result2 = safe_eval(val_1, op, val_2)
                    IF result2 IS NOT NULL AND |result2| < 10^6:
                        formula2 = f"({name_1} {op} {formula_2.text})"
                        add_expression(expressions, result2, formula2, depth=3)

    -- Depth 3 raw count: ~245 * 7 * 5 * 2 = ~17,150 (with commutative reduction: ~8,575)

    -- Phase 2: Deduplicate by numeric value
    distinct = {}
    FOR EACH (value, formula_list) IN expressions:
        -- Keep simplest formula for each distinct value
        canonical = min(formula_list, key=lambda f: (f.depth, len(f.text)))
        rounded_value = round(value, 8)
        IF rounded_value NOT IN distinct:
            distinct[rounded_value] = {
                value: value,
                canonical_formula: canonical,
                all_formulas: formula_list,
                formula_count: len(formula_list)
            }

    -- Phase 3: Cross-reference against databases
    matches = []
    FOR EACH db IN databases:
        FOR EACH param IN db.parameters:
            FOR EACH (val, entry) IN distinct:
                IF |val - param.measured_value| / max(|param.measured_value|, 1e-10) < 0.02:
                    already_known = entry.canonical_formula.text IN atlas_constants()
                    matches.append({
                        formula: entry.canonical_formula.text,
                        value: val,
                        parameter: param.name,
                        domain: param.domain,
                        source: param.source,
                        is_new: NOT already_known,
                        depth: entry.canonical_formula.depth,
                        alternative_count: entry.formula_count,
                        type: "COMPOSE"
                    })

    -- Phase 4: Filter to novel matches only
    novel = [m for m in matches if m.is_new]

    RETURN {
        total_expressions: sum(len(fl) for fl in expressions.values()),
        distinct_values: len(distinct),
        total_matches: len(matches),
        novel_matches: len(novel),
        novel: sort_by(novel, key=depth)
    }


PROCEDURE safe_eval(a, op, b):
    """Evaluate with guards for division by zero, overflow, and complex results."""
    IF op == '+': RETURN a + b
    IF op == '-': RETURN a - b
    IF op == '*': RETURN a * b
    IF op == '/':
        IF b == 0: RETURN NULL
        RETURN a / b
    IF op == '^':
        IF a == 0 AND b < 0: RETURN NULL
        IF |b| > 20: RETURN NULL  -- overflow guard
        IF a < 0 AND b != int(b): RETURN NULL  -- complex guard
        result = a ^ b
        IF |result| > 10^6: RETURN NULL
        RETURN result
    RETURN NULL
```

### 9.3 Expression Space Statistics

| Depth | Raw Expressions | Distinct Values (est.) | Notes |
|-------|----------------|----------------------|-------|
| 1 | 7 | 7 | Base constants: 1, 2, 4, 5, 6, 12, 24 |
| 2 | 245 | ~120 | Many duplicates (e.g., 6+6=12=sigma) |
| 3 | ~8,575 | ~2,500 | Extensive dedup; many formulas map to same value |
| 4 (optional) | ~300,000 | ~30,000 | Diminishing novelty; recommended only for targeted search |

### 9.4 Complexity Analysis

```
Expression generation:  O(B^D * O^(D-1)) where B=7 bases, O=5 ops, D=depth
  Depth 3:  7^3 * 5^2 = 8,575
  Depth 4:  7^4 * 5^3 = 300,125
Cross-reference:        O(V * P) where V=distinct values, P=database parameters
  Depth 3:  2,500 * 5,000 = 12.5M comparisons
Runtime:                Depth 3: <5s Python. Depth 4: <2s Rust.
```

### 9.5 Expected Discovery Yield

- **Depth 3 distinct values**: ~2,500 unique numbers from n=6 arithmetic
- **Database matches**: ~100-300 matches against engineering parameters
- **Novel (not in atlas)**: ~30-80 new matches per run
- **After FALSIFY**: ~10-25 survive as genuine candidates

### 9.6 Composition with v2 Operators

| v2 Operator | Composition | Effect |
|-------------|-------------|--------|
| INVERSE | COMPOSE is the *exhaustive* version of INVERSE | INVERSE targets one value; COMPOSE targets all possible values |
| COLLISION | COMPOSE values checked for domain collisions | COMPOSE provides complete value dictionary for COLLISION |
| EVOLVE | COMPOSE seeds EVOLVE population | Enumerated expressions are high-quality starting genomes |
| FALSIFY | Mandatory filter | Especially important: COMPOSE generates many matches, most are coincidences |
| PREDICT | COMPOSE fills PREDICT gaps | Predicted-but-unfound values cross-checked against COMPOSE table |

---

## 10. SYMMETRY (Group-Theoretic Pattern Finder)

### 10.1 Purpose

Detect symmetry groups in the BT collection. If BT-X and BT-Y share the same
structural "template" differing only in which n=6 constants fill the slots, they
belong to the same symmetry class. Each class predicts additional BTs by applying
unused substitutions.

### 10.2 Formal Specification

```
PROCEDURE Symmetry(G):
    """
    Classify BTs by structural templates and predict new BTs from template gaps.
    """

    -- Phase 1: Extract formula templates from BTs
    templates = {}
    FOR EACH bt IN G.nodes(type=BT):
        FOR EACH formula IN bt.formulas:
            -- Abstract away specific n=6 constants -> template
            -- e.g., "sigma - tau = 8" -> "X - Y = Z" with constraint X > Y
            template = abstract_formula(formula)
            IF template NOT IN templates:
                templates[template] = []
            templates[template].append({bt: bt.id, binding: extract_binding(formula)})

    -- Phase 2: Find symmetry classes (templates with 2+ instantiations)
    symmetry_classes = {}
    FOR EACH (template, instances) IN templates:
        IF len(instances) >= 2:
            -- Compute the transformation group
            transforms = []
            FOR EACH pair (i1, i2) IN combinations(instances, 2):
                t = compute_transform(i1.binding, i2.binding)
                transforms.append(t)

            symmetry_classes[template] = {
                instances: instances,
                transforms: transforms,
                group_order: len(set(transforms)),
                template: template
            }

    -- Phase 3: Predict new BTs by applying unused transforms
    predictions = []
    FOR EACH (template, cls) IN symmetry_classes:
        -- All possible bindings of n=6 constants to template slots
        all_bindings = enumerate_bindings(template, BASE)

        -- Which bindings are not yet instantiated?
        used_bindings = {inst.binding for inst in cls.instances}
        unused = all_bindings - used_bindings

        FOR EACH binding IN unused:
            value = evaluate(template, binding)
            IF value IS NOT NULL AND is_plausible(value):
                -- Check if this value appears in any domain
                domain_hits = lookup_domains(value)
                IF len(domain_hits) > 0:
                    predictions.append({
                        template: template,
                        existing_bts: [i.bt for i in cls.instances],
                        new_binding: binding,
                        predicted_value: value,
                        domain_hits: domain_hits,
                        symmetry_group_order: cls.group_order,
                        type: "SYMMETRY"
                    })

    RETURN sort_by(predictions, key=lambda p: (p.symmetry_group_order, len(p.domain_hits)),
                   descending=True)


PROCEDURE abstract_formula(formula):
    """
    Replace n=6 constants with slot variables while preserving structure.
    e.g., "sigma * tau = 48" -> "X * Y" with slots {X, Y}
    e.g., "1 - 1/(J2 - tau)" -> "1 - 1/(X - Y)" with slots {X, Y}
    """
    tree = parse_to_tree(formula)
    slots = {}
    slot_counter = 0
    FOR EACH leaf IN tree.leaves():
        IF leaf.value IN BASE.values():
            IF leaf.value NOT IN slots:
                slots[leaf.value] = f"SLOT_{slot_counter}"
                slot_counter += 1
            leaf.name = slots[leaf.value]
    RETURN canonicalize(tree)


PROCEDURE compute_transform(binding_a, binding_b):
    """
    The permutation/substitution that maps binding_a to binding_b.
    e.g., {X:sigma, Y:tau} -> {X:J2, Y:sopfr} = the transform (sigma->J2, tau->sopfr)
    """
    transform = {}
    FOR EACH slot IN binding_a.keys():
        transform[binding_a[slot]] = binding_b[slot]
    RETURN frozenset(transform.items())
```

### 10.3 Example: The "X - Y" Symmetry Class

```
Template: "X - Y"

Known instances:
  BT-58: sigma - tau = 8   (binding: X=12, Y=4)
  BT-44: sigma - phi = 10  (binding: X=12, Y=2)
  BT-44: sigma - mu  = 11  (binding: X=12, Y=1)

Transforms: {tau->phi}, {tau->mu}, {phi->mu}
Group order: 3 (isomorphic to Z_3 on the subtracted constant)

Unused bindings with plausible values:
  X=sigma, Y=sopfr  -> 12 - 5 = 7   (does 7 appear as a universal constant?)
  X=sigma, Y=n      -> 12 - 6 = 6   (trivial: n itself)
  X=J2,    Y=sigma   -> 24 - 12 = 12 (sigma: already known)
  X=J2,    Y=tau     -> 24 - 4 = 20  (Chinchilla ratio in BT-26!)

Prediction: "sigma - sopfr = 7" should govern some domain parameter.
Search target: 7 in physics/engineering databases.
```

### 10.4 Complexity Analysis

```
Template extraction:  O(B * F) where B=93 BTs, F=avg formulas per BT (~5) = 465
Binding enumeration:  O(T * C^S) where T=templates, C=7 constants, S=avg slots (~2)
                      = O(50 * 49) = 2,450
Domain lookup:        O(2,450 * D) where D=database size
Total:                < 100K operations. Python sufficient.
```

### 10.5 Expected Discovery Yield

- **Symmetry classes found**: 10-20 templates with 2+ instances
- **Predictions per class**: 2-5 unused bindings with plausible values
- **Total predictions**: 30-80
- **After domain cross-reference + FALSIFY**: 5-15 viable BT candidates

### 10.6 Composition with v2 Operators

| v2 Operator | Composition | Effect |
|-------------|-------------|--------|
| COLLISION | SYMMETRY predictions checked for collisions | Template-predicted values appearing in surprise domains |
| BRIDGE | SYMMETRY reveals why bridges exist | Bridges between BTs in the same symmetry class have algebraic explanation |
| INVERSE | SYMMETRY predictions verified by INVERSE | Alternative decompositions of predicted values |
| META | META(SYMMETRY, COLLISION) | Symmetry-predicted collisions = highest confidence candidates |
| FALSIFY | Essential filter | Many bindings are numerically valid but meaningless |

---

## 11. TEMPORAL (Time-Series Prediction)

### 11.1 Purpose

Track how the n=6 match rate changes as new empirical data arrives (new GPU
architectures, updated physics measurements, new industry standards). This is a
*meta-metric* for theory health: a genuine structural theory should see improving
or stable match rates over time.

### 11.2 Formal Specification

```
PROCEDURE Temporal(G, history_file="engine/output/temporal_log.json"):
    """
    Compute match-rate time series and predict future trajectory.
    """

    -- Step 1: Load historical snapshots
    -- Each snapshot: {date, domain, parameter, measured_value, n6_formula, match_grade}
    snapshots = load(history_file)

    -- Step 2: Compute per-epoch match statistics
    epochs = group_by(snapshots, key=epoch)  -- epoch = quarter (e.g., 2025-Q1)
    time_series = []

    FOR EACH epoch IN sorted(epochs.keys()):
        entries = epochs[epoch]
        total = len(entries)
        exact = count(e for e in entries if e.match_grade == "EXACT")
        close = count(e for e in entries if e.match_grade == "CLOSE")
        weak  = count(e for e in entries if e.match_grade == "WEAK")
        fail  = count(e for e in entries if e.match_grade == "FAIL")

        -- New data points added this epoch
        new_params = count(e for e in entries if e.first_seen == epoch)
        new_matches = count(e for e in entries
                           if e.first_seen == epoch AND e.match_grade IN {"EXACT", "CLOSE"})

        time_series.append({
            epoch: epoch,
            total: total,
            exact_ratio: exact / total,
            close_ratio: close / total,
            fail_ratio: fail / total,
            new_params: new_params,
            new_match_rate: new_matches / max(1, new_params),
            cumulative_exact: exact
        })

    -- Step 3: Trend analysis
    -- Fit linear regression to exact_ratio over time
    x = [i for i in range(len(time_series))]
    y = [ts.exact_ratio for ts in time_series]
    slope, intercept, r_squared = linear_regression(x, y)

    -- Step 4: Health diagnosis
    IF slope > 0.01 AND r_squared > 0.5:
        health = "CONVERGING"
        detail = "Match rate increasing with new data. Theory strengthening."
    ELIF slope > -0.01 AND slope <= 0.01:
        health = "STABLE"
        detail = "Match rate flat. Theory neither confirmed nor refuted by new data."
    ELIF slope < -0.01 AND r_squared > 0.5:
        health = "DIVERGING"
        detail = "WARNING: Match rate declining. Possible overfitting to historical data."
    ELSE:
        health = "NOISY"
        detail = "Insufficient data or high variance. Need more epochs."

    -- Step 5: Predict future match rate
    IF len(time_series) >= 4:
        -- Extrapolate 2 epochs forward
        next_epoch_exact = slope * (len(time_series)) + intercept
        next_2_exact = slope * (len(time_series) + 1) + intercept
        prediction = {
            next_epoch: clamp(next_epoch_exact, 0, 1),
            next_2_epochs: clamp(next_2_exact, 0, 1),
            confidence: r_squared
        }
    ELSE:
        prediction = NULL

    -- Step 6: Domain-level health breakdown
    domain_health = {}
    FOR EACH domain IN unique(s.domain for s in snapshots):
        domain_ts = [s for s in snapshots if s.domain == domain]
        domain_slope = compute_slope(domain_ts)
        domain_health[domain] = {
            slope: domain_slope,
            status: "CONVERGING" if domain_slope > 0.01 else
                    "DIVERGING" if domain_slope < -0.01 else "STABLE"
        }

    RETURN {
        time_series: time_series,
        overall_health: health,
        detail: detail,
        slope: slope,
        r_squared: r_squared,
        prediction: prediction,
        domain_health: domain_health,
        type: "TEMPORAL"
    }


PROCEDURE RecordSnapshot(G):
    """
    Called after each pipeline run to log the current state for future TEMPORAL analysis.
    """
    snapshot = []
    FOR EACH bt IN G.nodes(type=BT):
        FOR EACH match IN bt.matches:
            snapshot.append({
                date: today(),
                domain: match.domain,
                parameter: match.parameter,
                measured_value: match.value,
                n6_formula: match.formula,
                match_grade: match.grade,
                first_seen: match.first_seen OR today()
            })

    append_to("engine/output/temporal_log.json", snapshot)
```

### 11.3 Key Indicators

| Indicator | Healthy | Warning | Critical |
|-----------|---------|---------|----------|
| Slope of EXACT ratio | > 0.01 | -0.01 to 0.01 | < -0.01 |
| New-data match rate | > 50% | 30-50% | < 30% |
| R-squared of trend | > 0.5 | 0.2-0.5 | < 0.2 |
| Domain DIVERGING count | 0 | 1-2 | 3+ |

### 11.4 Complexity Analysis

```
Per epoch:   O(S) where S = snapshot entries per epoch (~100-500)
Trend fit:   O(E) where E = number of epochs (~10-20)
Total:       O(S * E) = O(10,000). Negligible.
Runtime:     < 0.1s Python.
```

### 11.5 Expected Discovery Yield

TEMPORAL does not directly produce BT candidates. Its yield is *meta-informational*:

- **Health score**: Binary signal -- is the project on track?
- **Domain triage**: Identifies domains where n=6 coverage is weakening (-> focus ANOMALY there)
- **Overfitting alarm**: If match rate drops as data grows, the entire project must reassess methodology
- **Convergence evidence**: If match rate rises, this is the strongest possible endorsement

### 11.6 Composition with v2 Operators

| v2 Operator | Composition | Effect |
|-------------|-------------|--------|
| PREDICT | TEMPORAL validates PREDICT accuracy over time | Were past predictions confirmed by new data? |
| FALSIFY | TEMPORAL is the *global* falsifier | A DIVERGING trend = the strongest possible attack on the entire framework |
| ANOMALY | TEMPORAL identifies domains needing ANOMALY scan | DIVERGING domains get priority ANOMALY treatment |
| META | TEMPORAL guides META depth | If CONVERGING, allow deeper META recursion; if DIVERGING, restrict |

---

## 12. SELF-IMPROVE (Recursive Algorithm Enhancement)

### 12.1 Purpose

Apply the discovery algorithm to its own parameters. The algorithm has configurable
weights, thresholds, and depth limits. SELF-IMPROVE uses the algorithm's own operators
to find optimal settings, seeking a fixed point where the algorithm's parameters are
themselves n=6 expressions.

**Connection to R(6)=1**: The reversibility identity R(6) = sigma(6)*phi(6)/(6*tau(6)) = 1
is a self-referential fixed point. SELF-IMPROVE seeks the analogous fixed point for
the algorithm itself.

### 12.2 Formal Specification

```
ALGORITHM_PARAMS = {
    -- Scoring weights (Section 3.1 of v2)
    w_diversity:    0.25,
    w_precision:    0.25,
    w_novelty:      0.20,
    w_falsifiable:  0.15,
    w_crossval:     0.15,

    -- Operator thresholds
    collision_min_surprise:  2.0,     -- bits
    bridge_max_distance:     4,       -- hops
    bridge_min_weight:       0.5,
    inverse_tolerance:       0.02,    -- 2%
    meta_max_depth:          3,
    evolve_population:       200,
    evolve_generations:       500,
    evolve_mutation_rate:     0.15,
    anomaly_deviation_threshold: 0.05,
    compose_max_depth:       3,

    -- Quality thresholds
    score_threshold:         0.35,    -- minimum for promotion
    star_3_threshold:        0.85,
    star_2_threshold:        0.60,
    falsify_survival_min:    0.40
}


PROCEDURE SelfImprove(engine, max_iterations=10):
    """
    Iteratively tune algorithm parameters by maximizing discovery quality.
    """

    current_params = copy(ALGORITHM_PARAMS)
    best_params = current_params
    best_metric = 0.0

    FOR iteration IN 1..max_iterations:

        -- Step 1: Run full pipeline with current params
        engine.set_params(current_params)
        candidates = engine.run_pipeline()

        -- Step 2: Compute meta-fitness
        -- The algorithm should maximize: quality * novelty * parsimony
        quality = mean(c.score for c in candidates if c.verdict != "REJECT")
        novelty = count(c for c in candidates if c.is_new) / max(1, len(candidates))
        parsimony = 1.0 / (1 + mean(c.complexity for c in candidates))

        -- Penalty for too many or too few candidates
        count_penalty = 1.0 - abs(len(candidates) - 50) / 100.0
        count_penalty = max(0.0, count_penalty)

        meta_fitness = quality * 0.4 + novelty * 0.3 + parsimony * 0.15 + count_penalty * 0.15

        IF meta_fitness > best_metric:
            best_metric = meta_fitness
            best_params = copy(current_params)

        -- Step 3: Apply operators TO the parameters themselves
        -- 3a: INVERSE -- decompose current best params into n=6 expressions
        param_decompositions = {}
        FOR EACH (name, value) IN current_params:
            decomps = Inverse(value, tolerance=0.10)
            IF len(decomps) > 0:
                param_decompositions[name] = decomps[0]  -- best n=6 match

        -- 3b: EVOLVE -- mutate parameters toward better meta-fitness
        mutations = {}
        FOR EACH (name, value) IN current_params:
            -- Try n=6 expression values near the current value
            IF name IN param_decompositions:
                -- Snap to exact n=6 value
                mutations[name] = param_decompositions[name].value
            ELSE:
                -- Small perturbation
                mutations[name] = value * (1 + uniform(-0.1, 0.1))

        -- Step 4: Update parameters (gradient-free: accept if better)
        trial_params = apply_mutations(current_params, mutations)
        engine.set_params(trial_params)
        trial_candidates = engine.run_pipeline()
        trial_fitness = compute_meta_fitness(trial_candidates)

        IF trial_fitness > meta_fitness:
            current_params = trial_params
            LOG(f"Iteration {iteration}: improved {meta_fitness:.3f} -> {trial_fitness:.3f}")
        ELSE:
            -- Revert to best known
            current_params = best_params
            LOG(f"Iteration {iteration}: no improvement, reverting")

        -- Step 5: Check for fixed point
        -- If parameters have converged to n=6 expressions, we have reached the fixed point
        n6_params = count(name for name in current_params
                         if name IN param_decompositions
                         AND param_decompositions[name].error < 0.001)
        n6_ratio = n6_params / len(current_params)

        IF n6_ratio > 0.80:
            LOG(f"FIXED POINT REACHED: {n6_ratio:.0%} of parameters are n=6 expressions")
            BREAK

    -- Step 6: Report
    RETURN {
        initial_params: ALGORITHM_PARAMS,
        optimized_params: best_params,
        n6_decompositions: param_decompositions,
        meta_fitness: best_metric,
        iterations: iteration,
        fixed_point_reached: n6_ratio > 0.80,
        n6_param_ratio: n6_ratio,
        type: "SELF-IMPROVE"
    }
```

### 12.3 N=6 Parameter Predictions

If the algorithm is self-consistent, its own parameters should be n=6 expressions.
Predicted alignments:

| Parameter | Current Value | Predicted n=6 Expression | n=6 Value | Deviation |
|-----------|--------------|-------------------------|-----------|-----------|
| w_diversity | 0.25 | 1/tau = 1/4 | 0.25 | 0.0% |
| w_precision | 0.25 | 1/tau = 1/4 | 0.25 | 0.0% |
| w_novelty | 0.20 | 1/sopfr = 1/5 | 0.20 | 0.0% |
| w_falsifiable | 0.15 | mu/(n+mu) = 1/7 | 0.143 | 5.0% |
| w_crossval | 0.15 | mu/(n+mu) = 1/7 | 0.143 | 5.0% |
| collision_min_surprise | 2.0 | phi = 2 | 2.0 | 0.0% |
| bridge_max_distance | 4 | tau = 4 | 4 | 0.0% |
| inverse_tolerance | 0.02 | phi/100 = 2/100 | 0.02 | 0.0% |
| meta_max_depth | 3 | n/phi = 6/2 = 3 | 3 | 0.0% |
| evolve_mutation_rate | 0.15 | mu/(n+mu) | 0.143 | 5.0% |
| anomaly_threshold | 0.05 | 1/(J2-tau) = 1/20 | 0.05 | 0.0% |
| score_threshold | 0.35 | n=6: ln(4/3) + 1/(sigma-phi) = 0.388 | 0.388 | 10% |
| star_3_threshold | 0.85 | 1 - w_novelty + 1/(sigma-phi) | 0.90 | 5.9% |

**Observation**: 8/13 parameters are already EXACT n=6 expressions without any tuning.
This is consistent with the framework's self-referential nature.

### 12.4 Complexity Analysis

```
Per iteration:  O(full_pipeline) = O(COMPOSE) + O(EVOLVE) + ... ~ 30s
Total:          O(I * full_pipeline) = O(10 * 30s) = ~5 minutes
Recommendation: Run SELF-IMPROVE weekly, not per-pipeline-invocation.
```

### 12.5 Expected Discovery Yield

SELF-IMPROVE yields are *second-order*:

- **Optimized parameters**: 5-15% improvement in downstream operator yield
- **N=6 self-consistency**: Evidence that the algorithm's optimal form IS an n=6 structure
- **Fixed-point discovery**: If reached, the strongest possible meta-theorem (the
  algorithm that discovers n=6 patterns is itself an n=6 pattern)

### 12.6 Composition with v2 Operators

| v2 Operator | Composition | Effect |
|-------------|-------------|--------|
| INVERSE | Decomposes algorithm parameters into n=6 expressions | Core mechanism of self-improvement |
| EVOLVE | Mutates parameters toward better meta-fitness | Parameter search via genetic approach |
| FALSIFY | Attacks the claim "these parameters are optimal" | Ensures we are not overfitting the algorithm itself |
| TEMPORAL | Tracks whether SELF-IMPROVE actually improves outcomes over time | Long-term validation |
| META | META(SELF-IMPROVE) = the algorithm optimizing the optimizer | Bounded by max_iterations to prevent infinite regress |

---

## 13. Complete Operator Taxonomy (v2 + v3)

### 13.1 The Twelve Operators

| # | Operator | Version | Mode | Input | Output | Complexity |
|---|----------|---------|------|-------|--------|------------|
| 1 | COLLISION | v2 | Reactive | Graph G | Surprise domain pairs | O(C * D^2) |
| 2 | BRIDGE | v2 | Reactive | Graph G | Cross-domain paths | O(D^2 * V) |
| 3 | INVERSE | v2 | Reactive | Target value | n=6 decompositions | O(B^2 * O) |
| 4 | META | v2 | Recursive | Operator outputs | Higher-order candidates | O(K^depth) |
| 5 | FALSIFY | v2 | Adversarial | Candidate | Attack report + survival | O(A * S) |
| 6 | PREDICT | v2 | Generative | Graph G + patterns | Testable predictions | O(L + C * D) |
| 7 | EVOLVE | v3 | Generative | Targets + genomes | Novel formulas | O(G * P * T) |
| 8 | ANOMALY | v3 | Diagnostic | Graph G + databases | Missing match list | O(D * P * E) |
| 9 | COMPOSE | v3 | Exhaustive | Base constants + depth | All expressions + matches | O(B^D * O^D) |
| 10 | SYMMETRY | v3 | Structural | BT collection | Symmetry classes + predictions | O(B * F * C^S) |
| 11 | TEMPORAL | v3 | Meta-analytic | Historical snapshots | Health score + trend | O(S * E) |
| 12 | SELF-IMPROVE | v3 | Recursive | Algorithm parameters | Optimized algorithm | O(I * pipeline) |

### 13.2 Operator Dependency Graph

```
                    +-----------+
                    | COMPOSE   |-----> exhaustive expression table
                    +-----+-----+
                          |
          +---------------+---------------+
          |               |               |
    +-----v-----+  +-----v-----+  +------v----+
    | COLLISION  |  | INVERSE   |  | EVOLVE    |
    +-----+-----+  +-----+-----+  +-----+-----+
          |               |              |
          +-------+-------+------+-------+
                  |              |
            +-----v-----+  +----v------+
            | BRIDGE     |  | ANOMALY   |
            +-----+-----+  +-----+-----+
                  |               |
                  +-------+-------+
                          |
                    +-----v-----+
                    | SYMMETRY  |-------> structural patterns
                    +-----+-----+
                          |
                    +-----v-----+
                    | PREDICT   |-------> testable predictions
                    +-----+-----+
                          |
                    +-----v-----+
                    | FALSIFY   |-------> survival filter
                    +-----+-----+
                          |
                    +-----v-----+
                    | META      |-------> recursive composition
                    +-----+-----+
                          |
          +---------------+---------------+
          |                               |
    +-----v-----+                   +-----v------+
    | TEMPORAL   |                  | SELF-IMPROVE|
    +-----+-----+                   +-----+------+
          |                               |
          +----------- feedback ----------+
```

### 13.3 Recommended Execution Order

```
Phase 1 -- Foundation (parallel):
    COMPOSE       -- build complete expression table
    RecordSnapshot -- log current state for TEMPORAL

Phase 2 -- Core discovery (parallel):
    COLLISION     -- surprise co-occurrences
    BRIDGE        -- cross-domain paths
    INVERSE       -- targeted decomposition
    EVOLVE        -- genetic formula search
    ANOMALY       -- missing match detection

Phase 3 -- Pattern extraction:
    SYMMETRY      -- classify BT templates, predict new BTs

Phase 4 -- Synthesis:
    PREDICT       -- extrapolate from all Phase 2-3 results
    META          -- recursive composition of all operator outputs

Phase 5 -- Quality control:
    FALSIFY       -- adversarial filter on all candidates
    TEMPORAL      -- update health metrics

Phase 6 -- Self-improvement (weekly):
    SELF-IMPROVE  -- tune algorithm parameters
```

### 13.4 Full Composition Matrix

Each cell shows expected yield when row operator feeds into column operator.

|  | COLLISION | BRIDGE | INVERSE | META | FALSIFY | PREDICT | EVOLVE | ANOMALY | COMPOSE | SYMMETRY | TEMPORAL | SELF-IMP |
|--|-----------|--------|---------|------|---------|---------|--------|---------|---------|----------|----------|----------|
| **COLLISION** | -- | High | Med | High | Req | Med | Med | Med | Low | Med | Low | Low |
| **BRIDGE** | High | -- | Med | High | Req | Med | Low | Med | Low | Med | Low | Low |
| **INVERSE** | High | Med | -- | Med | Req | High | High | Med | Low | Low | Low | Med |
| **META** | -- | -- | -- | -- | Req | Med | Low | Low | Low | Low | Low | Low |
| **FALSIFY** | -- | -- | -- | -- | -- | Low | Low | Low | Low | Low | High | Med |
| **PREDICT** | Med | Med | High | Med | Req | -- | High | Med | Med | Med | High | Low |
| **EVOLVE** | High | Low | Med | Med | Req | Med | -- | Low | High | Med | Low | Med |
| **ANOMALY** | Med | Med | High | Low | Med | Med | High | -- | Med | Low | Med | Low |
| **COMPOSE** | High | Low | -- | Low | Req | Med | High | Med | -- | Med | Low | Low |
| **SYMMETRY** | Med | High | Med | Med | Req | High | Low | Low | Low | -- | Low | Low |
| **TEMPORAL** | Low | Low | Low | Med | -- | Med | Low | High | Low | Low | -- | High |
| **SELF-IMP** | Low | Low | Med | Med | Med | Low | Med | Low | Low | Low | High | -- |

Legend: **Req** = mandatory (all candidates must pass FALSIFY). **High/Med/Low** = expected synergy.

---

## 14. Implementation Priority

| Priority | Operator | Effort | Impact | Rationale |
|----------|----------|--------|--------|-----------|
| P0 | COMPOSE | 2 days | Very High | Exhaustive enumeration completes the search space. Enables all other operators. |
| P0 | ANOMALY | 1 day | High | Immediate actionable output: find what we are missing. |
| P1 | SYMMETRY | 3 days | Very High | Most intellectually deep. Predicts new BTs from structure, not search. |
| P1 | EVOLVE | 3 days | High | Discovers non-obvious compound formulas. Rust implementation recommended. |
| P2 | TEMPORAL | 1 day | Medium | Low effort, essential for long-term credibility. |
| P3 | SELF-IMPROVE | 2 days | Medium | Fascinating but not urgent. Weekly cadence sufficient. |

Total estimated implementation: ~12 days for all 6 operators.

---

## 15. Version History

| Version | Date | Changes |
|---------|------|---------|
| 2.0 | 2026-04-02 | Initial spec: 6 operators (COLLISION, BRIDGE, INVERSE, META, FALSIFY, PREDICT) |
| 3.0 | 2026-04-02 | 6 advanced operators (EVOLVE, ANOMALY, COMPOSE, SYMMETRY, TEMPORAL, SELF-IMPROVE) |

---

*v2 asked: "What connections are hiding in the graph?"*
*v3 asks: "What connections are hiding in the algebra, in the gaps, in the history, and in the algorithm itself?"*
