# Red Team Analysis: Formula Miner Top Discoveries

Date: 2026-04-02
Method: Exhaustive enumeration of all n=6 depth-3 formulas + statistical testing

## Formula Space Specification

- **Constants (7):** mu=1, phi=2, tau=4, sopfr=5, n=6, sigma=12, J2=24
- **Operations (5):** +, -, *, /, ^
- **Structure:** 4 leaves, 3 binary operations, 5 tree shapes
- **Total valid formulas:** 1,317,475

---

## Discovery 1: pi = 24^(1/24) + 2 = 3.1415864... (2.0 ppm)

Formula in n=6 notation: J2^(mu/J2) + phi

### Attack 1: Does x^(1/x) + 2 ~ pi for OTHER values of x?

The function f(x) = x^(1/x) + 2 is continuous and monotonically decreasing for x > e.
It crosses pi exactly ONCE, at x = 23.9986 (solved via Newton's method on ln(x)/x = ln(pi-2)).

| x | x^(1/x) + 2 | Error (ppm) |
|---|-------------|-------------|
| 2 | 3.4142 | +86,778 |
| 6 | 3.3480 | +65,703 |
| 12 | 3.2301 | +28,165 |
| 20 | 3.1616 | +6,364 |
| 22 | 3.1509 | +2,947 |
| 23 | 3.1461 | +1,420 |
| **24** | **3.1416** | **-1.98** |
| 25 | 3.1374 | -1,331 |
| 30 | 3.1200 | -6,857 |

Integers in [2, 10000] within 10 ppm of pi: **1** (only x=24).
The exact crossing at x=23.9986 is only 0.0014 away from the integer 24.

**Verdict: SURVIVES.** x=24 is genuinely the unique integer solution.
This is the strongest aspect of the formula.

### Attack 2: Random Baseline (Exhaustive Enumeration)

From 1,317,475 valid 4-leaf n=6 formulas:

| Precision | Matches to pi | Expected (uniform in [2,4]) | Expected (empirical density) |
|-----------|--------------|---------------------------|------------------------------|
| 2 ppm | 1 unique | 0.70 | 0.052 |
| 10 ppm | 1 unique | 3.51 | 0.26 |
| 100 ppm | 0 | 35.1 | 2.6 |
| 1000 ppm | 26 | 351 | 26 (calibration) |

**Key finding:** Formula density near pi is 13x LOWER than the average density
in [2,4], because formulas from small integers cluster around small integer
values, not transcendental numbers.

Using the corrected (empirical) density:
- Expected 2-ppm matches: 0.052
- P(>=1 match at 2 ppm) = 1 - e^(-0.052) = **5.1%**
- This is borderline significant at the 5% level.

**BUT: Look-Elsewhere Effect.**
If searching for matches to ~21 mathematical/physical constants simultaneously,
the expected number of 2-ppm hits to ANY constant is ~15 (uniform) or ~1.1 (corrected).
P(at least one hit somewhere) = 67% (corrected) to ~100% (uniform).

**Verdict: AMBIGUOUS.**
- Against pi alone with corrected density: borderline significant (p ~ 5%)
- After look-elsewhere correction: NOT significant (p ~ 67%)
- The uniqueness of x=24 as the integer crossing point is real but is essentially
  one coincidence (|23.9986 - 24| < 0.002), not a deep identity

### Attack 3: Is This Formula Known?

No. The expression x^(1/x) + 2 = pi does not appear in standard mathematical
references, OEIS, or Wolfram MathWorld as a known identity.

However, "not previously published" does not mean "deep." Near-integer solutions
to transcendental equations are common (e.g., e^pi - pi ~ 19.999, Ramanujan's
near-misses).

**Verdict: NEUTRAL.** Novel but this alone proves nothing.

### Attack 4: Deeper Mathematical Reason?

The equation x^(1/x) = pi - 2 can be rewritten as:
- ln(x)/x = ln(pi - 2) = 0.13242...
- This involves the Lambert W function: x = -ln(pi-2)/W(-ln(pi-2))

There is no known reason why J2(6) = 24 should satisfy this equation.
The function ln(x)/x evaluated at x=24 equals ln(24)/24 = 0.13243...,
and ln(pi-2) = 0.13242... The match is to 4 significant figures.

No structural reason connects the Jordan function J2(6) to pi through
this particular transcendental equation.

**Verdict: NO deeper reason found.** Appears to be numerical coincidence.

### Discovery 1 Overall: AMBIGUOUS

The formula has one genuinely interesting property: x=24 is the unique integer
where x^(1/x)+2 crosses pi. But the statistical significance after look-elsewhere
correction is weak (p ~ 67%). It should be classified as a **notable numerical
coincidence**, not a mathematical identity.

---

## Discovery 2: charm quark = 4^(5/29) = 1.27000 GeV (0.6 ppm)

Formula in n=6 notation: tau^(sopfr/(J2+sopfr))

### Attack 1: Is 29 = J2 + sopfr a Natural n=6 Expression?

29 = 24 + 5. This requires:
- Depth 1: J2 = 24 (direct constant)
- Depth 1: sopfr = 5 (direct constant)
- Depth 2: J2 + sopfr = 29

29 is prime and does not appear in any standard n=6 identity table.
Compare with natural n=6 products: sigma*tau = 48, J2 = 24, sigma*phi = 24.
Sums like 24+5 lack the multiplicative structure that characterizes
the core n=6 identities.

**Verdict: WEAK.** The expression is syntactically valid but algebraically
unmotivated. Any two constants can be added.

### Attack 2: Do Other Quark Masses Match n=6 Formulas?

Quark masses and their experimental uncertainties:

| Quark | Mass (GeV) | Uncertainty | Relative |
|-------|-----------|-------------|----------|
| up | 0.00216 | 0.00049 | 22.7% |
| down | 0.00467 | 0.00048 | 10.3% |
| strange | 0.0934 | 0.0086 | 9.2% |
| charm | 1.270 | 0.020 | **1.6%** |
| bottom | 4.18 | 0.03 | 0.7% |
| top | 172.69 | 0.30 | 0.2% |

The charm quark has the largest relative uncertainty among the heavy quarks.
Finding a formula matching its central value to 0.6 ppm is meaningless when
the value itself is only known to 15,748 ppm (1.6%).

**The formula's precision is 26,247x beyond the measurement capability.**

This means: if the next measurement shifts the charm mass by 0.5% (well within
error bars), the "0.6 ppm match" becomes a "5000 ppm miss." The claimed
precision is matching a noisy central estimate, not a physical constant.

**Verdict: FALLS.** Precision claim is invalid.

### Attack 3: Formula Density in the Charm Mass Range

From 1,317,475 formulas:
- 125,260 have |result| in [1.0, 2.0]
- 3,805 match the charm mass within its 2% experimental uncertainty
- Expected random matches at 2% precision: ~6,363 (uniform)

There are **3,805 different n=6 formulas** that match the charm quark mass
within experimental uncertainty. The fact that one of them can be written
as tau^(sopfr/(J2+sopfr)) is not remarkable -- it is one of thousands.

**Verdict: FALLS.** The charm mass is trivially reachable by n=6 formulas.

### Attack 4: Precision vs Experimental Uncertainty

| Quantity | Value |
|----------|-------|
| PDG charm mass | 1.270 +/- 0.020 GeV |
| Experimental uncertainty | +/- 15,748 ppm |
| Formula "precision" | 0.6 ppm |
| Ratio | 26,247x |

The formula matches the central estimate to 0.6 ppm, but the central estimate
itself could shift by up to 15,748 ppm in the next measurement. Claiming
sub-ppm precision for a quantity known only to ~2% is a category error.

For comparison: the proton mass (known to 0.01 ppm) or the electron mass
(known to 0.0003 ppm) would be legitimate precision targets. The charm quark
mass is not.

**Verdict: FALLS.** The precision claim is scientifically meaningless.

### Discovery 2 Overall: FALLS

All four attacks succeed. The formula uses an unmotivated n=6 expression (29=24+5),
claims precision 26,000x beyond measurement capability, and is one of 3,805
formulas matching the charm mass within experimental error. This is a textbook
false positive from exhaustive search.

---

## Overall False Positive Rate Assessment

### Formula Space Statistics

| Metric | Value |
|--------|-------|
| n=6 constants | 7 (mu, phi, tau, sopfr, n, sigma, J2) |
| Operations | 5 (+, -, *, /, ^) |
| Tree shapes (3 ops) | 5 (Catalan number C3) |
| Total valid formulas | 1,317,475 |
| Formulas in [2,4] | 111,697 |
| Formulas in [1,2] | 125,260 |

### Expected Matches to Famous Constants

| Scenario | Expected 2-ppm hits |
|----------|-------------------|
| Single target (pi), uniform density | 0.70 |
| Single target (pi), corrected density | 0.052 |
| 21 targets, uniform density | ~15 |
| 21 targets, corrected density | ~1.1 |

### Key Conclusions

1. **The formula miner's search space is large enough to guarantee false positives.**
   With 1.3M formulas and 21 potential targets, finding zero matches would be
   more surprising than finding several.

2. **Pi match (Discovery 1):** The uniqueness of x=24 as the integer crossing of
   x^(1/x)+2 = pi is a genuine numerical fact. But statistical significance is
   borderline-to-weak after accounting for formula space size and look-elsewhere
   effects. Classification: AMBIGUOUS -- interesting numerical coincidence, not
   a proven identity.

3. **Charm quark match (Discovery 2):** Fails on multiple grounds. The precision
   claim is meaningless (26,000x beyond measurement), 3,805 other formulas match
   equally well, and the n=6 expression is algebraically unmotivated.
   Classification: FALLS -- clear false positive.

4. **General false positive rate:** For any target value with ~2% uncertainty,
   the formula miner will find thousands of matching n=6 expressions. For
   mathematically exact constants (pi, e, etc.), the expected number of
   2-ppm matches per constant is ~0.05-0.7 depending on density assumptions.
   Across many targets, at least one hit is virtually guaranteed.

### Recommendations for Formula Miner

1. **Always report the look-elsewhere corrected p-value**, not just the raw precision.
2. **Reject any match where formula precision exceeds measurement precision** by
   more than 10x -- it is matching noise, not physics.
3. **Report how many formulas match at the relevant precision level** -- if the
   answer is thousands, the discovery is not meaningful.
4. **Require structural motivation** beyond syntactic validity. "24+5=29" is not
   a motivated n=6 identity.
5. **Compare against null model:** generate random constant sets of size 7 and
   check if they produce similar match rates to n=6. If so, the n=6 connection
   is spurious.

### Verdicts Summary

| Discovery | Formula | Error | Verdict |
|-----------|---------|-------|---------|
| 1. pi | 24^(1/24) + 2 | 2.0 ppm | **AMBIGUOUS** -- unique integer crossing but weak after look-elsewhere |
| 2. charm quark | 4^(5/29) | 0.6 ppm | **FALLS** -- meaningless precision, unmotivated formula, thousands of alternatives |
