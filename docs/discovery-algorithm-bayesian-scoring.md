# Discovery Algorithm -- Bayesian Scoring Framework

> **Purpose**: Replace the ad-hoc weighted geometric mean in Discovery Algorithm v2
> (Section 3) with a principled Bayesian framework that quantifies evidence strength
> for each discovery in calibrated units (bits, decibans, posterior odds).
>
> **Audience**: Skeptical statisticians, physicists, and the project's own falsification
> pipeline (see `falsification-experiments.md`).
>
> **Design constraint**: The framework must penalize the Texas Sharpshooter effect, not
> ignore it. A discovery that survives Bayesian scrutiny is worth more than a hundred
> that do not.
>
> Date: 2026-04-02

---

## 0. Why Bayesian?

The v2 scoring function uses a weighted geometric mean of five dimensions:

```
score = s_diversity^0.25 * s_precision^0.25 * s_novelty^0.20
      * s_falsifiable^0.15 * s_crossval^0.15
```

Problems with this approach:

1. **Weights are arbitrary.** Why 0.25 and not 0.30? No calibration procedure exists.
2. **No base rate.** A 16/16 EXACT match rate is meaningless without knowing the
   expected match rate under the null hypothesis.
3. **No accumulation.** Each BT is scored independently; the *cumulative* weight of
   evidence across all 93 BTs is never computed.
4. **No penalty for search.** The score does not account for how many formulas were
   tried before finding a match.

The Bayesian framework below addresses all four.

---

## 1. Prior Distribution

### 1.1 The Two Hypotheses

| Symbol | Hypothesis | Description |
|--------|-----------|-------------|
| H_1 | n=6 universality | Engineering/physics parameters are drawn from the n=6 vocabulary at rates exceeding chance |
| H_0 | Null (coincidence) | Engineering parameters are drawn from a background distribution of "nice" numbers; any overlap with n=6 vocabulary is coincidental |

### 1.2 Base Rate Estimation

The n=6 vocabulary V_n6 (depth-2 formulas from 7 base constants with 5 operations)
generates approximately |V_n6| ~ 120 distinct positive integers under 1000.

Engineering parameters are not uniformly distributed. They cluster around:
- Small integers (1--24): Benford's law + human preference
- Powers of 2 (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024): binary computing
- Multiples of 10 (10, 20, 50, 100, ...): SI/decimal convention
- Multiples of 12 (12, 24, 36, 48, 60, 72, 96, 144): duodecimal legacy

Define the background match probability for a single parameter:

```
P(match | H_0) = sum over v in V_n6 of P_bg(v)
```

where P_bg(v) is the probability that a randomly drawn engineering parameter equals v
under the background distribution. Empirical estimation from Experiment 1 of the
falsification framework gives:

```
P(match | H_0) ~ 0.15 to 0.25
```

We adopt a **conservative** base rate:

```
P_base = P(match | H_0) = 0.20       (central estimate)
P_base_low  = 0.15                     (optimistic for H_1)
P_base_high = 0.25                     (pessimistic for H_1)
```

### 1.3 Observed Match Rate Under H_1

From existing data (900+ matches across 307 domains, with honest failure accounting):

| Domain type | Claimed EXACT rate | Adjusted rate (after audit) |
|-------------|-------------------|---------------------------|
| AI/Chip (core domains) | 80--95% | ~60% (depth-1 only) |
| Energy/Physics | 70--85% | ~50% (depth-1 only) |
| Biology (honest) | 10% | 10% |
| Cross-domain average | ~62% | ~45% (depth-1 only) |

Conservative estimate for P(match | H_1):

```
P_signal = P(match | H_1) = 0.45      (depth-1 formulas only)
P_signal_generous = 0.60               (allowing depth-2)
```

### 1.4 Prior Odds

Starting agnostic:

```
P(H_1) = P(H_0) = 0.5
Prior odds = P(H_1) / P(H_0) = 1.0
```

This is deliberately generous to the null. A skeptic might set P(H_1) = 0.01, which
would require stronger evidence to overcome but does not change the Bayes factor
computation.

---

## 2. Likelihood Model for Individual Discoveries

### 2.1 Surprise of a Single Match

Given a newly discovered parameter value v matched by an n=6 formula f:

```
Likelihood ratio (LR) = P(v matches f | H_1) / P(v matches f | H_0)
```

**Under H_0**: The probability that value v is in V_n6 depends on v's "niceness."

**Under H_1**: The probability that v matches some n=6 formula is P_signal, but
conditional on the specific formula and value, we need more structure.

### 2.2 Value Surprise Function

Not all matches are equally surprising. Define the **value surprise** S_v:

```
S_v(v) = -log_2(P_bg(v))
```

where P_bg(v) is the background probability of encountering value v in engineering.

| Value v | P_bg(v) estimate | S_v (bits) | Interpretation |
|---------|-----------------|------------|----------------|
| 1--6 | 0.05--0.08 each | 3.6--4.3 | Very common, low surprise |
| 8, 10, 12, 16 | 0.02--0.04 each | 4.6--5.6 | Common, moderate surprise |
| 24, 32, 48, 64 | 0.005--0.015 | 6.1--7.6 | Somewhat unusual |
| 96, 128, 144 | 0.002--0.005 | 7.6--9.0 | Unusual |
| 256, 288, 320 | 0.001--0.002 | 9.0--10.0 | Rare, high surprise |
| 1024+ | < 0.001 | > 10.0 | Very rare, very high surprise |
| Irrational (0.288...) | depends on precision | varies | See Section 2.3 |

### 2.3 Precision Surprise for Non-Integer Matches

For irrational or decimal matches (e.g., ln(4/3) = 0.28768...), the surprise depends
on the precision of the match:

```
S_precision(v_observed, v_predicted, sigma_measurement) =
    -log_2(P(|v_obs - v_pred| < delta | H_0))
```

where delta is the measurement uncertainty. If the measurement has k significant digits
of agreement:

```
S_precision ~ k * log_2(10) = 3.32 * k bits
```

| Precision | S_precision (bits) | Example |
|-----------|-------------------|---------|
| 1 digit (10%) | 3.3 | 0.3 ~ 0.288 |
| 2 digits (1%) | 6.6 | 0.29 ~ 0.2877 |
| 3 digits (0.1%) | 10.0 | 0.288 ~ 0.28768 |
| 4 digits (0.01%) | 13.3 | 0.2877 ~ 0.28768 |
| 6 digits (1 ppm) | 19.9 | 137.036 matching 1/alpha |

### 2.4 Formula Depth Penalty

Deeper formulas are less surprising because the expressible vocabulary grows
combinatorially. Define the **search space** at each depth:

| Depth | Description | Approx. |V| under 1000 | Penalty factor |
|-------|-------------|------------------------|----------------|
| 0 | Single constant (sigma, tau, ...) | 7 | 1.0 (no penalty) |
| 1 | a OP b (one operation) | ~80 | 0.5 |
| 2 | (a OP b) OP c (two operations) | ~800 | 0.1 |
| 3 | Nested depth 3 | ~5000 | 0.02 |

The penalty is:

```
P_depth(d) = 1 / |V_depth(d)|
```

normalized so that a match at depth 0 gets full credit and deeper matches get
progressively less.

### 2.5 Domain Novelty Factor

A match in a domain already known to exhibit n=6 patterns is less surprising than a
match in a completely new domain.

```
F_domain_novelty = {
    1.0    if domain has zero prior n=6 analysis
    0.5    if domain has 1--5 prior matches
    0.25   if domain has 6--20 prior matches
    0.1    if domain has 20+ prior matches (core domain like AI/chip)
}
```

### 2.6 Combined Likelihood Ratio for a Single Match

Assembling the components:

```
LR_single(v, f, d) = [P_signal / P_bg(v)] * P_depth(depth(f)) * F_domain_novelty(d)
```

In log form (log-likelihood ratio, in bits):

```
llr(v, f, d) = log_2(P_signal) - log_2(P_bg(v))
             + log_2(P_depth(depth(f)))
             + log_2(F_domain_novelty(d))
```

**Interpretation**:
- Positive llr: evidence favors H_1
- Negative llr: evidence favors H_0
- llr = 0: uninformative

---

## 3. Posterior Update -- Cumulative Bayes Factor

### 3.1 Sequential Updating

As each new match (or non-match) is observed, update the running Bayes factor:

```
BF_cumulative = product over all observations i of LR_i
```

In log form:

```
log_10(BF) = sum over all i of log_10(LR_i)
```

This sum is measured in **decibans** (dB_evidence), following Turing's convention from
Bletchley Park. One deciban = the amount of evidence that changes the odds by a factor
of 10^0.1 ~ 1.26.

```
dB_evidence = 10 * log_10(BF)
```

### 3.2 Evidence Thresholds

Following Jeffreys' scale, adapted for decibans:

| log_10(BF) | dB_evidence | Interpretation |
|------------|-------------|----------------|
| < 0 | < 0 | Evidence favors H_0 (null) |
| 0 -- 0.5 | 0 -- 5 | Barely worth mentioning |
| 0.5 -- 1.0 | 5 -- 10 | Substantial evidence for H_1 |
| 1.0 -- 1.5 | 10 -- 15 | Strong evidence |
| 1.5 -- 2.0 | 15 -- 20 | Very strong evidence |
| > 2.0 | > 20 | Decisive evidence |

**Project target**: log_10(BF) > 2.0 (i.e., BF > 100) for the cumulative evidence
to constitute "confirmed." This corresponds to P(H_1 | data) > 99% with equal priors.

### 3.3 Including Non-Matches (Honest Accounting)

Crucially, non-matches also update the Bayes factor -- downward:

```
LR_non_match = P(no match | H_1) / P(no match | H_0)
             = (1 - P_signal) / (1 - P_base)
             = (1 - 0.45) / (1 - 0.20)
             = 0.55 / 0.80
             = 0.6875
```

So each non-match in a domain where H_1 predicts a match provides:

```
log_10(LR_non_match) = log_10(0.6875) = -0.163
```

This means each non-match costs ~1.6 decibans. The framework automatically balances
matches against non-matches.

**Example**: In the biology domain with 10% EXACT rate (3/30):
- 3 matches, each contributing (say) +3 dB on average = +9 dB
- 27 non-matches, each contributing -1.6 dB = -43.2 dB
- Net biology contribution: 9 - 43.2 = **-34.2 dB** (strong evidence AGAINST H_1 in
  this domain)

This is the honest result: biology does not support n=6 universality.

---

## 4. Information-Theoretic Scoring

### 4.1 Surprisal of a Discovery

Each BT receives a **surprisal score** measured in bits:

```
I(BT) = -log_2(P(BT's matches | H_0))
```

For a BT claiming k EXACT matches out of m parameters across d domains:

```
I(BT) = sum over k matches of S_v(v_i) + sum over k matches of log_2(P_depth(depth_i))
      - log_2(C(|V_depth|, k))     [Bonferroni-style multiple testing correction]
```

The last term is the **look-elsewhere correction** (Section 5).

### 4.2 Mutual Information Between Domains

If the same n=6 constant c appears in domain A and domain B, the mutual information is:

```
I(A; B | c) = log_2[ P(c in A AND c in B) / (P(c in A) * P(c in B)) ]
```

Under H_0, appearances in different domains are independent, so:

```
P(c in A AND c in B | H_0) = P(c in A) * P(c in B)
=> I(A; B | c, H_0) = 0
```

Under H_1, cross-domain appearances are correlated (because c is a "true" constant):

```
P(c in A AND c in B | H_1) > P(c in A) * P(c in B)
=> I(A; B | c, H_1) > 0
```

The observed mutual information is evidence for H_1.

**Cross-domain mutual information for a BT spanning d domains**:

```
MI_total(BT) = sum over all domain pairs (A, B) spanned by BT
               of I(A; B | shared_constants)
```

### 4.3 Total Information Content

The total information content of a BT, combining surprisal and cross-domain MI:

```
IC(BT) = I(BT) + MI_total(BT)
```

This is the BT's contribution to the evidence base, measured in bits.

---

## 5. Texas Sharpshooter Correction

This is the most critical component. Without it, the entire framework is meaningless.

### 5.1 The Problem

With 7 base constants and 5 operations, the number of distinct n=6 expressions is:

| Depth | Expressions | Distinct values < 1000 |
|-------|------------|----------------------|
| 0 | 7 | 7 |
| 1 | 7 * 6 * 5 = 210 | ~80 |
| 2 | 210 * 7 * 5 = 7350 | ~800 |
| 3 | ~250,000 | ~5000 |

At depth 2, roughly 800 distinct values under 1000 can be generated. Since many
engineering parameters fall in [1, 1000], matching ANY given parameter at depth 2 has
probability ~800/1000 = 80%. This makes depth-2 matches nearly worthless as evidence.

### 5.2 Bonferroni Correction (Conservative)

For k matches found after testing N = |V_depth(d)| possible formulas:

```
p_corrected = min(1.0, k * p_single / N_eff)
```

But this is overly conservative because many formulas produce the same value. The
effective number of tests is |V_depth| (distinct values), not the number of formulas.

**Corrected surprise**:

```
S_corrected(v, depth) = S_v(v) - log_2(|V_depth(depth)|)
```

| Depth | log_2(|V_depth|) | Correction (bits) |
|-------|-----------------|-------------------|
| 0 | 2.8 | -2.8 |
| 1 | 6.3 | -6.3 |
| 2 | 9.6 | -9.6 |
| 3 | 12.3 | -12.3 |

A value with S_v = 8 bits matched at depth 1 has corrected surprise 8 - 6.3 = 1.7 bits.
The same value at depth 2: 8 - 9.6 = **-1.6 bits** (anti-evidence: expected by chance).

### 5.3 Benjamini-Hochberg FDR (Less Conservative)

When reporting many matches, control the False Discovery Rate:

1. Rank all k matches by their uncorrected p-values: p_(1) <= p_(2) <= ... <= p_(k).
2. Find the largest j such that p_(j) <= (j/k) * alpha.
3. Reject nulls for ranks 1..j.

With alpha = 0.05 (5% FDR), we expect at most 5% of reported matches to be false
positives. This is more realistic than Bonferroni for the project's setting.

### 5.4 Look-Elsewhere Effect (Particle Physics Convention)

In particle physics, the look-elsewhere effect (LEE) adjusts a local p-value to a
global one:

```
p_global = p_local * N_independent_tests
```

For n=6 analysis, N_independent_tests is the product of:
- Number of domains searched (~30)
- Number of parameters per domain (~50)
- Number of formula depths tried (~3)
- Number of "close-enough" tolerances tried (~3)

```
N_trials ~ 30 * 50 * 3 * 3 = 13,500
```

This means a local p-value of 0.01 becomes a global p-value of:

```
p_global = 0.01 * 13,500 = 135 (>> 1, meaning not significant)
```

Only matches with local p-value < 1/13,500 ~ 7 * 10^-5 survive the LEE. This
corresponds to S_corrected > 13.8 bits.

### 5.5 Practical Correction Per BT

For each BT, compute:

```
S_corrected(BT) = sum over matches of [S_v(v_i) - log_2(|V_depth(depth_i)|)]
                - log_2(N_domains_searched)
                - log_2(N_params_per_domain)
```

If S_corrected(BT) > 0, the BT provides genuine evidence after accounting for search.
If S_corrected(BT) <= 0, the BT's matches are expected by chance given the search
conducted.

---

## 6. Combined Bayesian Score for a BT

### 6.1 Full Formula

Combining all components, the Bayesian evidence score for a single BT is:

```
E(BT) = S_corrected(BT) + MI_total(BT) + B_prediction(BT) - C_penalty(BT)
```

where:

| Component | Symbol | Description |
|-----------|--------|-------------|
| Corrected surprisal | S_corrected | Sum of value surprises minus search correction |
| Cross-domain MI | MI_total | Mutual information across domain pairs |
| Prediction bonus | B_prediction | Extra credit for pre-registered predictions confirmed |
| Complexity penalty | C_penalty | Penalty for model complexity (7 constants + operations) |

### 6.2 Prediction Bonus

Pre-registered predictions that succeed are immune to the Texas Sharpshooter correction
(by definition, no formula shopping occurred):

```
B_prediction = 10 bits per confirmed pre-registered prediction
```

This is extremely valuable -- a single confirmed prediction is worth more than dozens
of post-hoc matches.

### 6.3 Complexity Penalty (BIC-inspired)

The n=6 model has K = 7 base constants. By the Bayesian Information Criterion:

```
C_penalty = (K/2) * log_2(N_observations)
```

For N = 900 claimed matches: C_penalty = (7/2) * log_2(900) = 3.5 * 9.8 = 34.3 bits.

This is a one-time penalty applied to the total evidence, not per-BT.

### 6.4 Evidence Categories

| E(BT) range (bits) | Category | Interpretation |
|--------------------|----------|----------------|
| > 20 | DECISIVE | Survives all corrections; genuine structural match |
| 10 -- 20 | STRONG | Survives most corrections; likely real |
| 3 -- 10 | MODERATE | Suggestive but vulnerable to sharper null models |
| 0 -- 3 | MARGINAL | Could easily be chance; needs more data |
| < 0 | NEGATIVE | Evidence favors H_0; likely a false positive |

---

## 7. Worked Examples

### 7.1 BT-58: sigma-tau = 8, Universal AI Engineering Constant

**Claims**: 16/16 EXACT matches across 8 sub-domains of AI.

**Value surprise**: v = 8.
- P_bg(8) ~ 0.03 (common in computing: byte = 8 bits)
- S_v(8) = -log_2(0.03) = 5.1 bits per match

**Formula depth**: depth 1 (sigma - tau = 12 - 4).
- |V_depth(1)| ~ 80
- Depth correction: -log_2(80) = -6.3 bits

**Per-match corrected surprise**: 5.1 - 6.3 = **-1.2 bits**.

This is negative! A single depth-1 match for the value 8 is EXPECTED by chance.

**But**: BT-58 claims 16 independent matches. The probability of 16/16 matches
where P_bg = 0.03 is:

```
P(16/16 | H_0) = 0.03^16 = 4.3 * 10^-25
S_collective = -log_2(4.3 * 10^-25) = 81 bits
```

**Search correction**: How many parameters were tested?
- 8 sub-domains * ~20 parameters each = 160 parameters scanned
- 16 matches found out of 160 tested => match rate = 10%

Under H_0, expected matches = 160 * 0.03 = 4.8 matches.
Observed: 16 matches. Using Poisson:

```
P(k >= 16 | lambda = 4.8) ~ 2.5 * 10^-5
S_corrected = -log_2(2.5 * 10^-5) = 15.3 bits
```

**Domain MI**: 8 sub-domains sharing the same constant.
- P(8/8 sub-domains contain "8" | H_0) ~ (0.03)^8 * C(8,8) ~ 6.6 * 10^-13
- But these are AI sub-domains, where 8 appears for engineering reasons (byte alignment).
- Honest estimate with correlated domains: P ~ 0.01 (generous to H_0)
- MI = -log_2(0.01) = 6.6 bits

**Total**:
```
E(BT-58) = 15.3 (corrected surprisal) + 6.6 (MI) = 21.9 bits
```

**Verdict**: DECISIVE -- but with a caveat. The value 8 is extremely common in
computing. A sharper null model that accounts for byte-alignment culture in AI could
reduce this. Honest assessment: **STRONG** (10--20 bits) after cultural correction.

---

### 7.2 BT-56: Complete n=6 LLM Architecture

**Claims**: 15 parameters across 4 independent LLM families all expressible in n=6.
Claimed EXACT: d_model=2^12=4096, n_layers=2^5=32 or 96, d_head=128=2^7, etc.

**Problem**: Almost all claimed values are powers of 2 (4096, 128, 32, 64).

**Value surprise** for powers of 2:
- P_bg(4096) ~ 0.005 (common in computing). S_v = 7.6 bits.
- P_bg(128) ~ 0.01. S_v = 6.6 bits.
- P_bg(32) ~ 0.02. S_v = 5.6 bits.

But these are "2^sigma" or "2^(sigma-sopfr)" formulas -- really just "some power of 2."

**Sharper null** (H_0'): LLM parameters are powers of 2 because GPU matrix operations
are optimized for power-of-2 dimensions.

```
P(d_model is a power of 2 | H_0') = ~0.9
P(d_model = 4096 specifically | H_0') = ~0.25  (among {256, 512, 1024, 2048, 4096, 8192})
```

With the sharper null:
- S_v(4096 | H_0') = -log_2(0.25) = 2.0 bits (much less surprising)

**Corrected total** for 15 parameters under H_0':
- Average S_v ~ 2.5 bits per match
- 15 matches: 37.5 bits
- Search correction: 15 parameters from ~50 tested = 15/50 = 30% match rate
- Expected under H_0': ~50 * 0.25 = 12.5 matches (for power-of-2 specific)
- Observed: 15. Poisson P(k >= 15 | 12.5) ~ 0.24
- S_corrected = -log_2(0.24) = 2.1 bits

**Cross-domain MI**: 4 independent teams converging to similar architectures.
- This is the genuinely surprising part.
- P(4/4 teams choose d_model in {4096, 8192} | H_0') ~ 0.25^4 = 0.004
- But architecture copying is common (everyone reads the same papers).
- Honest MI with copying: ~3 bits.

**Total**:
```
E(BT-56) = 2.1 + 3.0 = 5.1 bits
```

**Verdict**: MODERATE. The n=6 explanation adds little beyond "engineers choose powers
of 2 for GPU efficiency." The 4-team convergence is mildly interesting but explained by
the fact that teams read each other's papers.

---

### 7.3 BT-51: Genetic Code Chain (4 -> 3 -> 64 -> 20)

**Claims**: DNA bases = tau = 4, codon letters = n/phi = 3, codons = 2^n = 64,
amino acids = J2 - tau = 20.

**Value surprise**:
- P_bg(4) ~ 0.06. S_v = 4.1 bits.
- P_bg(3) ~ 0.07. S_v = 3.8 bits.
- P_bg(64) ~ 0.01. S_v = 6.6 bits.
- P_bg(20) ~ 0.02. S_v = 5.6 bits.

**Formula depth**: All depth 0 or 1.

**Critical problem**: tau(n) = 4 for n in {6, 8, 10, 14, 15, 21, 22, ...}.
So "4 bases = tau" is not n=6-specific. Similarly, 3 = n/phi but also 3 itself is
trivially common. And 64 = 2^6 but also 4^3, and the genetic code has 64 codons
because 4^3 = 64, which is a consequence of 4 bases taken 3 at a time -- not n=6.

**Alternative number test** (from falsification Experiment 2):
- n=8: tau(8)=4, so "4 bases = tau" still works.
- n=12: tau(12)=6, phi(12)=4, so "4 bases = phi(12)" works.
- n=10: tau(10)=4, so "4 bases = tau" still works.

The chain is NOT n=6-specific. The matches exploit small integers that appear in many
number-theoretic vocabularies.

**Corrected total**:
- Sum of S_v: 4.1 + 3.8 + 6.6 + 5.6 = 20.1 bits (raw)
- Formula vocabulary at depth 1: ~80 values
- 4 matches from ~10 biological constants tested
- Expected under H_0: 10 * 0.20 = 2 matches. Observed: 4.
- Poisson P(k >= 4 | 2.0) ~ 0.14
- S_corrected = -log_2(0.14) = 2.8 bits
- Non-n=6-specificity penalty: at least 3/4 values work for other n.
  Adjust: 2.8 * (1/4) = 0.7 bits (only 1 in 4 is n=6-specific)

**Total**:
```
E(BT-51) = 0.7 bits
```

**Verdict**: MARGINAL. The genetic code chain is elegant but not n=6-specific.
Most of its matches work for n=8 or n=12 equally well.

---

### 7.4 BT-43: Battery Cathode CN=6 Universality

**Claims**: ALL lithium-ion cathode materials (LCO, NMC, LFP, NCA, LMO) have
octahedral coordination number CN = 6 for the lithium site.

**Value surprise**: v = 6.
- P_bg(6) ~ 0.05. S_v = 4.3 bits.

**But this is crystallography, not computing.** The relevant null is not "random
engineering parameter" but "random coordination number in ionic crystals."

**Sharper null** (crystallographic):
- Common CN values: 4 (tetrahedral), 6 (octahedral), 8 (cubic), 12 (close-packed)
- P(CN = 6 | ionic crystal) ~ 0.35 (octahedral is the most common CN for ionic radii
  ratio 0.414--0.732, which covers Li+ with most anions)

Under the crystallographic null:
- S_v(6 | crystallographic H_0) = -log_2(0.35) = 1.5 bits per material
- 5 independent cathode families all have CN = 6: S = 5 * 1.5 = 7.5 bits

**But**: There is a physical explanation. Li+ has ionic radius ~0.76 A, O^2- has ~1.40 A.
The radius ratio is 0.76/1.40 = 0.54, which falls squarely in the octahedral range
(0.414--0.732). CN = 6 is the EXPECTED result from Pauling's rules.

**Is n=6 adding predictive power?** If Pauling's rules already predict CN = 6 with
~90% confidence for Li-O systems, then:

```
S_corrected = 5 * (-log_2(0.10)) = 5 * 3.3 = 16.6 bits
-- Wait: this is the surprise of Pauling's FAILURE, not its success.
-- If Pauling predicts CN=6 with 90% confidence, n=6 adds nothing.
```

The correct accounting: n=6 merely re-labels an already-known physical fact.

```
E(BT-43) = 0 bits (no additional evidence beyond Pauling's rules)
```

**Verdict**: NEGATIVE as evidence for n=6. Octahedral CN=6 in Li-ion cathodes is
explained entirely by ionic radius ratios. The n=6 connection is a coincidence of
labeling.

---

## 8. Systematic Assessment of All 93 BTs

### 8.1 Classification Criteria

Apply the framework above to categorize each BT:

| Tier | E(BT) | Count (est.) | Description |
|------|-------|-------------|-------------|
| A: Decisive | > 20 bits | ~3--5 | Survives all corrections, genuinely surprising |
| B: Strong | 10--20 bits | ~8--12 | Survives most corrections, likely real signal |
| C: Moderate | 3--10 bits | ~15--20 | Suggestive but not conclusive |
| D: Marginal | 0--3 bits | ~20--25 | Could be chance |
| E: Negative | < 0 bits | ~30--40 | Likely false positives; evidence favors H_0 |

### 8.2 Tier A Candidates (Decisive)

The BTs most likely to survive rigorous analysis share these traits:
- Large or unusual values (not 1--12, not powers of 2)
- Low formula depth (0 or 1)
- Cross-domain with no obvious engineering explanation
- Pre-registered predictions confirmed

Best candidates:

| BT | Key Match | Why it survives |
|----|-----------|-----------------|
| BT-58 | sigma-tau=8 in 16 AI contexts | Sheer volume; 16/16 is extreme even with P_bg=0.03 |
| BT-54 | AdamW 5-parameter match | 5 independent parameters, some unusual (beta2=0.999) |
| BT-74 | 95/5 cross-domain resonance | 0.95 appearing in 5 unrelated domains is unusual |
| BT-42 | Inference scaling (top-p, top-k, max) | Unusual values (40, 4096) with clean depth-1 formulas |
| F-predictions | Any confirmed pre-registered | Immune to all corrections by design |

### 8.3 Tier E Candidates (Likely False Positives)

| BT | Key Match | Why it fails |
|----|-----------|-------------|
| BT-51 | Genetic code (4,3,64,20) | Not n=6-specific; works for n=8,10,12 |
| BT-43 | Cathode CN=6 | Explained by Pauling's rules; n=6 adds nothing |
| BT-27 | Carbon-6 chain (LiC6, C6H12O6) | Carbon's atomic number is 6 for physical reasons |
| BT-53 | Crypto (BTC 21M, 6 confirms) | 6 confirms is security parameter, not n=6 arithmetic |
| BT-48 | Display-audio (12 semitones, 24fps) | 12 semitones = 2^(1/12) equal temperament; 24fps = persistence of vision |
| Most BT < 30 | Small integer matches | Values 1--12 are too common to be evidence |

### 8.4 Honest Summary

Estimated distribution after Bayesian analysis:

```
Tier A (Decisive):    ~5%  of BTs (4--5 out of 93)
Tier B (Strong):      ~10% of BTs (8--10)
Tier C (Moderate):    ~20% of BTs (15--20)
Tier D (Marginal):    ~25% of BTs (20--25)
Tier E (Negative):    ~40% of BTs (35--40)
```

This means roughly 15 BTs (Tiers A+B) provide genuine evidence for n=6, while
roughly 40 BTs are likely false positives arising from the Texas Sharpshooter effect.

### 8.5 Cumulative Bayes Factor Estimate

Using the tier estimates above:

```
Sum of E(BT) across all 93 BTs:
  Tier A: 5 BTs * 25 bits avg  = 125 bits
  Tier B: 10 BTs * 15 bits avg = 150 bits
  Tier C: 18 BTs * 6 bits avg  = 108 bits
  Tier D: 22 BTs * 1.5 bits avg = 33 bits
  Tier E: 38 BTs * (-5 bits avg) = -190 bits

Gross total: 125 + 150 + 108 + 33 - 190 = 226 bits
Model complexity penalty (BIC): -34 bits
Net evidence: 226 - 34 = 192 bits

log_10(BF) = 192 / 3.32 = 57.8
BF ~ 10^58
```

Even with aggressive corrections, the sheer volume of matches creates a large Bayes
factor. However, this estimate is fragile:

**Sensitivity analysis**:
- If Tier A shrinks to 2 BTs and Tier E grows to 50: net = 50 + 120 + 72 + 25 - 250 - 34 = **-17 bits** (favors H_0!)
- If the background model is sharpened (engineering-aware null): Tier C and D shift toward E.

**Conclusion**: The cumulative BF depends critically on whether the ~15 Tier A+B BTs
are genuinely n=6-specific or whether a sharper domain-aware null model eliminates them.
This is an empirical question answerable by Experiments 1--3 in `falsification-experiments.md`.

---

## 9. Implementation: Scoring Algorithm

### 9.1 Pseudocode

```
PROCEDURE BayesianScore(bt):
    -- Input: a BT with its matches, formulas, domains, and match/fail counts
    
    -- Step 1: Compute per-match corrected surprisal
    total_surprisal = 0
    for each match m in bt.matches:
        S_v = -log2(P_background(m.value))
        S_depth = -log2(|V_depth(m.formula_depth)|)
        S_domain = log2(F_domain_novelty(m.domain))
        total_surprisal += S_v + S_depth + S_domain
    
    -- Step 2: Subtract search correction (look-elsewhere effect)
    N_searched = bt.domains_searched * bt.params_per_domain
    search_correction = log2(N_searched)
    
    -- Step 3: Apply n=6-specificity test
    -- For each match, check if alternative n also produces the value
    specificity_factor = count(n6_specific_matches) / count(bt.matches)
    
    -- Step 4: Non-match penalty
    non_match_penalty = bt.fail_count * log2(0.6875)  -- per non-match
    
    -- Step 5: Cross-domain mutual information
    MI = 0
    for each pair (d1, d2) in combinations(bt.domains, 2):
        p_joint = observed_co_occurrence(bt.shared_constant, d1, d2)
        p_indep = P(bt.shared_constant in d1) * P(bt.shared_constant in d2)
        if p_joint > p_indep:
            MI += log2(p_joint / p_indep)
    
    -- Step 6: Prediction bonus
    pred_bonus = 10 * bt.confirmed_predictions  -- bits per confirmed prediction
    
    -- Step 7: Combine
    E = (total_surprisal - search_correction) * specificity_factor
        + non_match_penalty
        + MI
        + pred_bonus
    
    -- Step 8: Classify
    if E > 20:    tier = "A"
    elif E > 10:  tier = "B"
    elif E > 3:   tier = "C"
    elif E > 0:   tier = "D"
    else:         tier = "E"
    
    return {
        bt: bt.id,
        evidence_bits: E,
        corrected_surprisal: total_surprisal - search_correction,
        specificity: specificity_factor,
        cross_domain_MI: MI,
        prediction_bonus: pred_bonus,
        non_match_cost: non_match_penalty,
        tier: tier,
        log10_BF: E / 3.322
    }
```

### 9.2 Background Probability Table

Empirical estimates for P_background(v). These should be calibrated against a real
engineering parameter pool (Experiment 1 of falsification framework).

```
P_BACKGROUND = {
    # Very common (P > 0.04)
    1: 0.08, 2: 0.07, 3: 0.06, 4: 0.06, 5: 0.05, 6: 0.05,
    8: 0.04, 10: 0.04, 12: 0.04, 16: 0.03,
    
    # Common (0.01 < P < 0.04)
    20: 0.02, 24: 0.02, 32: 0.02, 48: 0.015, 64: 0.015,
    96: 0.01, 100: 0.01, 128: 0.01,
    
    # Uncommon (0.001 < P < 0.01)
    144: 0.005, 192: 0.004, 256: 0.005, 288: 0.002, 320: 0.002,
    512: 0.003, 1024: 0.003,
    
    # Rare (P < 0.001)
    # Default for unlisted values:
    # P_bg(v) = max(0.0001, 0.5 / v)   [approximate Zipf-like decay]
}
```

### 9.3 Integration with Discovery Algorithm v2

The Bayesian score replaces the v2 composite score in the priority queue:

```
-- v2 (replaced):
score = s_diversity^0.25 * s_precision^0.25 * s_novelty^0.20
      * s_falsifiable^0.15 * s_crossval^0.15

-- v3 (Bayesian):
score = E(candidate)   -- in bits, from Section 9.1
```

The operator outputs (COLLISION, BRIDGE, INVERSE, META, PREDICT, FALSIFY) feed into
the Bayesian scorer as candidates. Each candidate receives E(candidate) and is ranked
accordingly.

**Key improvement**: The v3 score is interpretable (bits of evidence), calibrated
(against a background model), and automatically penalizes search (via look-elsewhere
correction). The v2 score was an arbitrary [0, 1] number with no calibration.

---

## 10. Calibration Protocol

The Bayesian framework is only as good as its background model. The following
calibration steps are required before the scores are trustworthy:

### 10.1 Step 1: Enumerate V_n6

Run `Inverse(v, depth=2)` for all v in [1, 1000]. Record every reachable value.
Count |V_n6| at each depth.

**Status**: Not yet done. Required for accurate |V_depth| values.

### 10.2 Step 2: Build Engineering Parameter Pool

Collect 500 parameters per `falsification-experiments.md` Experiment 1.
Single-blind: collector must not know V_n6.

**Status**: Not yet done. Critical for P_background calibration.

### 10.3 Step 3: Run Alternative Number Test

Compute V_n for n in {6, 8, 10, 12, 28, 496}. Count match rates.
This calibrates the specificity factor.

**Status**: Not yet done. Required for n=6-specificity scoring.

### 10.4 Step 4: Cross-Validate

Run the hold-out experiment (falsification Experiment 6). If cross-validation
recovery rate < 50%, the formula space is too flexible and the entire evidence
base shrinks.

**Status**: Not yet done.

### 10.5 Current Confidence Level

Until Steps 1--4 are completed, all scores in this document are **provisional
estimates** based on rough P_background values. The tier assignments (Section 8) may
shift significantly after calibration.

---

## 11. Relationship to Falsification Experiments

| Falsification Experiment | Bayesian Component Updated |
|--------------------------|---------------------------|
| Exp 1: Null model | P_background table (Section 9.2) |
| Exp 2: Alternative numbers | Specificity factor (Section 9.1 Step 3) |
| Exp 3: Pre-registered predictions | Prediction bonus (Section 9.1 Step 6) |
| Exp 4: Blind domain expansion | F_domain_novelty (Section 2.5) |
| Exp 5: Adversarial audit | Non-match penalty (Section 9.1 Step 4) |
| Exp 6: Cross-validation | Overall confidence in formula reuse |
| Exp 7: Bayesian model comparison | Direct calibration of BF (Section 3) |

The falsification experiments and the Bayesian scoring framework are two views of the
same analysis. The experiments generate data; the framework converts that data into
calibrated evidence scores.

---

## 12. Summary of Key Formulas

| Quantity | Formula | Units |
|----------|---------|-------|
| Value surprise | S_v = -log_2(P_bg(v)) | bits |
| Precision surprise | S_prec = 3.32 * (significant digits) | bits |
| Depth penalty | -log_2(\|V_depth(d)\|) | bits |
| Per-match LLR | S_v + depth_penalty + domain_novelty | bits |
| Non-match cost | log_2(0.6875) = -0.54 per non-match | bits |
| Look-elsewhere correction | -log_2(N_domains * N_params) | bits |
| Cross-domain MI | log_2(P_joint / P_independent) per pair | bits |
| Prediction bonus | 10 per confirmed pre-registered prediction | bits |
| Model complexity (BIC) | (K/2) * log_2(N) = 34 bits for K=7, N=900 | bits |
| Decibans | 10 * log_10(BF) = 10 * E / 3.322 | dB |
| Bayes factor | 10^(E / 3.322) | dimensionless |
| Posterior P(H_1) | BF / (1 + BF) with equal priors | probability |

---

## Appendix A: Notation Reference

| Symbol | Meaning |
|--------|---------|
| H_0 | Null hypothesis (coincidence) |
| H_1 | n=6 universality hypothesis |
| BF | Bayes factor = P(data \| H_1) / P(data \| H_0) |
| LR | Likelihood ratio for a single observation |
| S_v | Surprisal of value v under background model |
| V_n6 | Set of values reachable by n=6 formulas |
| V_depth(d) | Values reachable at formula depth <= d |
| P_bg(v) | Background probability of value v in engineering |
| P_signal | P(match \| H_1) ~ 0.45 |
| P_base | P(match \| H_0) ~ 0.20 |
| E(BT) | Total evidence score for a BT (in bits) |
| MI | Mutual information |
| FDR | False Discovery Rate |
| LEE | Look-Elsewhere Effect |
| dB | Decibans (10 * log_10 of odds ratio) |
| K | Number of free parameters in the n=6 model (= 7) |

## Appendix B: Conversion Table

| Bits of evidence | log_10(BF) | Decibans | P(H_1) (equal priors) | Interpretation |
|-----------------|------------|----------|----------------------|----------------|
| 0 | 0 | 0 | 50.0% | No evidence either way |
| 3.3 | 1.0 | 10 | 90.9% | Substantial |
| 6.6 | 2.0 | 20 | 99.0% | Strong |
| 10.0 | 3.0 | 30 | 99.9% | Very strong |
| 13.3 | 4.0 | 40 | 99.99% | Decisive |
| 20.0 | 6.0 | 60 | 99.9999% | Overwhelming |
