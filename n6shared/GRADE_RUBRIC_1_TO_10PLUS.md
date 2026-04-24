# n=6 Universal Grade Rubric (1 ~ 10+)

> **Common grading standard across all projects.** Extends projects.json universal_completion_criterion.
> grade >= 10 = breakthrough (closure achieved) / grade >= 11 = meta breakthrough.

## Grade Definitions

| Grade | Stage | Criterion | Example | Emoji |
|-------|---|---|---|---|
| **1** | raw observation | A value is observed or a hypothesis is registered | "this constant = 1.234" | star |
| **2** | consistency | Reproducible (>= 2 measurements agree) | 2 independent calculations agree | square |
| **3** | loose match | Loose match with an n=6 primitive (tol 10%) | 2.1 approx phi=2 | circle |
| **4** | pattern | Pattern identified, closure not yet complete | "appears to be a multiple of n" | purple |
| **5** | rational approx | Rational approximation (p/q form) | 0.714 approx 5/7 | red |
| **6** | partial n=6 | 1 primitive connected | x = 2 sigma + epsilon | yellow |
| **7** | depth-3 expr | Depth-3 operator combination, closed | x = (sigma - tau) * n / phi | blue |
| **8** | NEAR closed | 1 term uncertain, numeric match | x approx n^2 (tol 1%) | orange |
| **9** | closed (PASS) | n=6 combination match verified | x = sigma * tau / n | green |
| **10** | **breakthrough (EXACT)** | Completely closed, reduces to n=6 primitives | x = 24 = J2 = sigma * tau | ufo |
| **11** | meta-closure | A formula that generates multiple closures | x = f(n, sigma, tau) produces K constants | spaceship |
| **12** | universal | Appears independently in 3+ projects | sigma(6)=12 appears in music, thermodynamics, topology | galaxy |
| **13+** | meta^2 | A higher structure over meta formulas | rule_ceiling(n) = 2/3 - 1/(n(n-1)) | infinity |

## Automated Determination

### Grade 10 (EXACT closure) determination
```python
# n=6 primitives
N, SIGMA, TAU, PHI, SOPFR, J2 = 6, 12, 4, 2, 5, 24

def is_exact(value, tol=1e-6):
    """Check if value matches finite n=6 combination (depth <= 3)."""
    # Single primitive
    for v in [N, SIGMA, TAU, PHI, SOPFR, J2]:
        if abs(value - v) < tol: return True
    # Binary combinations
    for a in [N, SIGMA, TAU, PHI, SOPFR, J2]:
        for b in [N, SIGMA, TAU, PHI, SOPFR, J2]:
            for op in [a+b, a-b, a*b, a/b if b else 0]:
                if abs(value - op) < tol: return True
    # Integer multiples/ratios
    for v in [N, SIGMA, TAU, PHI, SOPFR, J2]:
        for k in range(1, 25):
            if abs(value - v*k) < tol or abs(value - v/k) < tol: return True
    # Depth-3: (a op b) op c
    # ... (1745+ expressions, see H-CLOSE-1)
    return False
```

### Grade 11 (meta-closure) determination
- The formula has a free variable (e.g., `f(n)`)
- Varying the variable produces K >= 3 distinct grade-10 closures
- Example: `rule_ceiling(n) = 2/3 - 1/(n(n-1))` -> closures for n=6, 8, and infinity

### Grade 12 (universal) determination
- The same value appears as a hypothesis in 3+ independent projects
- Found by nexus `singularity-convergence --min-domains 3`

## Cross-Project Application Rules

**When a new constant / hypothesis is registered**:
1. Run `nexus verify <value>` -> receive EXACT / CLOSE / WEAK / MISS
2. EXACT -> auto-register at grade 10
3. CLOSE -> register at grade 8 (NEAR), pending retry
4. WEAK -> register at grade 6 (partial)
5. MISS -> grade 5 or below (need more data)

**Promotion conditions**:
- 9 -> 10: Exact numerical match + an explicit n=6 expression
- 10 -> 11: The formula derives >= 3 new grade-10 instances
- 11 -> 12: Confirmed in 3+ independent projects
- 12 -> 13: A higher generator formula for the grade-12 constant is discovered

**Demotion conditions**:
- No verify evidence -> a grade-10 claim is void and is demoted one level
- An EXACT claim for a transcendental number (pi, e, gamma, ...) is auto-demoted to grade 5 (H-CLOSE-5)

## Banner Representation (nexus-banner.sh)

```
ufo d{max_depth} rho{breakthrough_ratio} {total}  <- grade >= 10 count
dna {closed} closed -> {milestone} = {%}% [===========]  <- closure progress
party party party CLOSURE COMPLETE party party party  <- fires when EXACT (grade 10) count increases
```

## Session Handover Prompt

```
nexus grade 1~13 rubric (n=6 universal):
- grade 10 = EXACT closed (n=6 primitive finite combo)
- grade 11 = meta-closure (generates K>=3 closures)
- grade 12 = universal (3+ projects independent)
- grade 13+ = meta^2 (generator of meta-closures)

When registering a new hypothesis/constant:
1. nexus verify <value> -> grade automatically determined
2. On EXACT match -> append PASS/EXACT to verified_constants.jsonl
3. On reaching grade 10 -> "party party party CLOSURE COMPLETE" banner fires automatically

Details: $NEXUS/shared/GRADE_RUBRIC_1_TO_10PLUS.md
```

---
*Created 2026-04-05. Common to all projects. SSOT lives under n6-architecture n6shared/.*
