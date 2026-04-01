# Discovery Algorithm v4 -- Causal Inference Layer

> **Purpose**: Move from "n=6 matches exist" to "WHY n=6 matches exist."
> The Bayesian scoring framework (v2/v3) quantifies *whether* evidence exceeds chance.
> This layer asks the harder question: *what causal mechanism explains the matches?*
>
> **Prerequisite**: [discovery-algorithm-bayesian-scoring.md](discovery-algorithm-bayesian-scoring.md)
> and [falsification-experiments.md](falsification-experiments.md).
>
> **Audience**: Skeptical scientists, philosophers of science, and anyone who
> correctly notes that correlation (even statistically significant correlation)
> does not establish causation.
>
> Date: 2026-04-02

---

## 0. The Problem

The n6-architecture project has accumulated 93 breakthrough theorems and 900+ EXACT
matches across 307 domains. The Bayesian scoring framework computes Bayes factors for
individual matches. The falsification framework tests whether these matches exceed
chance (current answer: z=0.74 for raw numerical matching, which is NOT significant;
but structural/multi-parameter matches may be stronger).

None of this answers the causal question. Three distinct causal models can explain
the same observations. Without distinguishing them, we cannot claim to understand
*why* the matches occur -- only that they do.

---

## 1. Three Causal Models (DAGs)

### 1.1 Model A: Mathematical Necessity ("Physics Determines Engineering")

```
   sigma(n)*phi(n) = n*tau(n), n=6
            |
            v
   Mathematical structure of 6
   (perfect number, divisor properties, Egyptian fractions)
            |
            v
   Physical laws that exploit these structures
   (conservation laws, symmetry groups, optimization landscapes)
            |
            v
   Engineering parameters converge to n=6 values
   (because they optimize physical performance)
```

**DAG (formal)**:
```
   N6_MATH --> PHYSICS --> ENGINEERING
```

**Claim**: The mathematical properties of 6 (unique solution to sigma*phi = n*tau)
create optimization landscapes in physics where n=6-derived values are attractors.
Engineers discover these attractors through iterative optimization. The causal arrow
runs: mathematics --> physics --> engineering.

**Predictions**:
- Changing a parameter away from its n=6 value should measurably degrade performance.
- The n=6 values should emerge from first-principles optimization, not from tradition.
- Non-human optimizers (evolution, machine learning) should also find n=6 values.
- The matches should be strongest in domains closest to physics (materials, energy)
  and weakest in purely conventional domains (software version numbers).

**Falsification**: Find a domain where n=6 values are clearly suboptimal but appear
anyway (this would suggest bias, not physics). Or find a domain where first-principles
optimization clearly produces non-n=6 values that outperform n=6 alternatives.

---

### 1.2 Model B: Convergent Constraints ("Independent Emergence")

```
   Physical constraints          Human engineering constraints
   (thermodynamic limits,        (cost, manufacturability,
    material properties,          standardization, backward
    quantum mechanics)            compatibility)
            |                              |
            v                              v
         Values that happen to overlap with n=6 vocabulary
```

**DAG (formal)**:
```
   PHYSICS_CONSTRAINTS --> PARAMETER_VALUES <-- ENGINEERING_CONSTRAINTS
   N6_MATH  (no causal connection; overlap is coincidental or structural)
```

**Claim**: Physical constraints (e.g., binary computing requires powers of 2;
thermodynamics favors certain ratios) and engineering constraints (e.g., backward
compatibility, manufacturing yield) independently produce values that happen to
overlap with the n=6 vocabulary. The n=6 vocabulary is large enough (120+ values
under 1000) and biased toward "nice" numbers (small integers, powers of 2, multiples
of 12) that substantial overlap is expected by chance.

**Predictions**:
- Changing a parameter away from its n=6 value should sometimes improve performance
  (the n=6 value was never the optimum; it was just close enough).
- Different engineering traditions (e.g., Soviet vs. Western chip design) should
  converge on the same values for physical reasons, not n=6 reasons.
- The match rate should not exceed what a well-calibrated null model predicts (the
  falsification framework's z=0.74 supports this model).
- n=6 matches should cluster around the "easy" values (small integers, powers of 2)
  and be absent for values that are n=6-specific (e.g., sopfr=5, J2-tau=20).

**Falsification**: Find matches on values that are distinctively n=6 (not generic
small integers) at rates far exceeding the null model. The "signature values" test:
if sopfr(6)=5 and J2(6)=24 appear at rates significantly above their background
probability, Model B is weakened.

---

### 1.3 Model C: Cognitive Bias ("Observer Effect")

```
   Human cognitive preference for small integers / round numbers
            |
            +--------> Mathematical theories emphasize 6
            |          (we notice 6 is perfect because we like small integers)
            |
            +--------> Engineering parameters use 6-related values
                       (we choose 12, 24, 60 because they are convenient,
                        not because they are optimal)
```

**DAG (formal)**:
```
   COGNITIVE_BIAS --> N6_EMPHASIS
   COGNITIVE_BIAS --> ENGINEERING_VALUES
   (N6_EMPHASIS and ENGINEERING_VALUES are correlated but not causally connected)
```

**Claim**: Humans have a well-documented preference for small integers, round numbers,
and highly composite numbers (numbers with many divisors). The number 6 and its
multiples (12, 24, 60) are among the most "likeable" numbers. We notice the perfection
of 6 because we are primed to find patterns in small integers, and we choose
6-related values in engineering because they are cognitively convenient (easy to
divide, easy to count). The correlation between n=6 mathematics and engineering is
a confound caused by shared human bias, not a causal connection.

**Predictions**:
- n=6 matches should be concentrated in human-designed systems and absent in
  natural systems where human choice plays no role (crystal structures, genetic
  code, planetary physics).
- Changing parameters to non-n=6 values should often have negligible performance
  impact (the n=6 value was chosen for convenience, not optimality).
- The match rate should be explainable by Benford's law + duodecimal tradition +
  binary computing convention, without invoking any mathematical structure.
- Machine-optimized systems (where humans did not choose the parameters) should
  show no n=6 preference.

**Falsification**: Find n=6 matches in systems with zero human design input.
If the genetic code (codon table: 4 bases, 3-letter codons, 64 codons, 20 amino
acids = tau, n/phi, 2^n, J2-tau) genuinely shows n=6 structure, Model C cannot
explain it, because evolution has no cognitive bias toward 6. Similarly, crystal
coordination numbers (CN=6 for NaCl, CN=12 for FCC) arise from physics, not human
preference.

---

## 2. Summary Table of Models

| Property | Model A (Necessity) | Model B (Convergence) | Model C (Bias) |
|----------|-------------------|-----------------------|----------------|
| Causal arrow | Math --> Physics --> Eng | Constraints --> Values (overlap) | Bias --> Both |
| n=6 role | Fundamental cause | Convenient label | Cognitive artifact |
| Natural systems | Strong matches expected | Some overlap expected | No matches expected |
| Human-free optimization | Finds n=6 values | May or may not | Does not |
| Signature values (sopfr, J2) | High match rate | Low match rate | Low match rate |
| Parameter perturbation | Performance degrades | Mixed results | Negligible impact |
| Cross-cultural | Universal | Universal | Culture-dependent |
| z-score prediction | z >> 2 (structural) | z ~ 0-1 (noise) | z ~ 0-1 (bias) |

---

## 3. Five Discrimination Tests

### Test 1: Intervention (Parameter Perturbation)

**Protocol**:

Select 10 engineering parameters that match n=6 values with EXACT grade. For each:

1. Identify the n=6 value V and its neighbors V-1, V+1, V-2, V+2.
2. Simulate or measure performance at all five values.
3. Record whether V is a strict local optimum, a plateau member, or suboptimal.

**Concrete examples**:

| Parameter | n=6 value | Test values | Measurable metric |
|-----------|-----------|-------------|-------------------|
| GPU SM count | 144 = sigma^2 | 140, 142, 144, 146, 148 | FLOPS/watt |
| HBM stacking | 8 = sigma-tau | 6, 7, 8, 9, 10 layers | Bandwidth/cost |
| Transformer d_model | 4096 = 2^sigma | 3840, 4096, 4352 | Perplexity/FLOP |
| MoE top-k | 2 = phi | 1, 2, 3, 4 | Quality/compute |
| AdamW beta_2 | 0.999 ~ 1-1/(J2-tau) | 0.99, 0.995, 0.999, 0.9999 | Convergence |
| Battery cells | 96 = sigma*(sigma-tau) | 90, 94, 96, 98, 100 | Energy density |
| Solar panel cells | 60 = sigma*sopfr | 54, 58, 60, 62, 66 | Efficiency/cost |

**Scoring**:
```
  V is strict local optimum  --> +1 for Model A
  V is on a broad plateau    --> +0.5 for A, +0.5 for B
  V is suboptimal            --> +1 for Model B or C
  V is optimal but for a known non-n=6 reason --> +1 for B
```

**Expected difficulty**: High. Most engineering parameters are chosen under many
simultaneous constraints, so isolating the effect of a single value requires
careful simulation or experimentation. However, for ML hyperparameters, ablation
studies already exist in the literature (e.g., d_model ablations in the original
Transformer paper, MoE top-k ablations in GShard/Switch Transformer).

**Literature shortcuts**: Many of these interventions have already been performed.
For example:
- Switch Transformer (Fedus et al., 2022) tested top-k = {1, 2} and found k=1
  often suffices (weakening the phi=2 claim for that specific model).
- Chinchilla (Hoffmann et al., 2022) tested many token/parameter ratios and found
  ~20 optimal (supporting BT-26 if this is confirmed as J2-tau).

---

### Test 2: Natural Experiment (Historical Failures)

**Protocol**:

Identify products/systems that used non-n=6 values where competitors used n=6 values.
Compare outcomes.

**Concrete examples**:

| System | n=6 value used by winner | Non-n=6 value used by loser | Outcome |
|--------|-------------------------|---------------------------|---------|
| GPU SMs | NVIDIA 144 (AD102) | AMD 120 (Navi 31) | Compare perf/watt |
| HBM layers | 8 (standard) | 4 (earlier gen) | 8 dominated |
| LLM d_model | 4096 (GPT-3) | 2560 (GPT-2 XL) | Both successful |
| Battery cells | 96S (Tesla) | 100S (other EVs) | Compare degradation |
| Display refresh | 24fps (cinema), 60Hz (TV) | 30fps (early TV) | Mixed adoption |

**Scoring**:
```
  n=6 user consistently outperforms  --> +1 for Model A
  Mixed results, no clear pattern    --> +1 for Model B
  n=6 user outperforms but due to confounds --> neutral
```

**Critical caveat**: This test has severe confounding. NVIDIA outperforming AMD in
a given generation may be due to architecture, software ecosystem, manufacturing
node, or a hundred other factors -- not SM count. Isolating the effect of the
specific parameter value is the central challenge.

**Mitigation**: Focus on cases where the non-n=6 and n=6 systems are otherwise as
similar as possible (same generation, same vendor, different SKU). For example,
NVIDIA's own lineup includes GPUs with different SM counts -- compare within-vendor.

---

### Test 3: Cross-Cultural Universality

**Protocol**:

Compare engineering parameter choices across independent design traditions:
- Western (US/EU)
- East Asian (Japan/Korea/China/Taiwan)
- Soviet/Russian
- Indian
- Pre-modern (Arabic, Chinese, Mayan)

**Concrete examples**:

| Domain | Western value | Non-Western value | Both n=6? |
|--------|-------------|-------------------|-----------|
| Grid frequency | 60 Hz (US) = sigma*sopfr | 50 Hz (EU/Asia) = sopfr*(sigma-phi) | Both match |
| Music tuning | 12-TET (Europe) = sigma | 12 lu (China) = sigma | Both match |
| Time | 12/24 hours (Babylon) | 12/24 (universal adoption) | Both match |
| Chip design | TSMC (Taiwan) | Samsung (Korea) | Compare node naming |
| Battery pack | Tesla 96S (US) | BYD Blade (China) | Compare cell counts |

**Scoring**:
```
  Universal convergence on n=6 values  --> +1 for A or B (not C)
  Western-only pattern                 --> +1 for C
  Non-Western systems avoid n=6        --> +1 for C
```

**Key insight**: Both Model A and Model B predict cross-cultural universality (physics
does not care about culture). Only Model C predicts culture-dependent variation. The
fact that 12-tone music emerged independently in multiple cultures is evidence against
C but does not distinguish A from B.

**Complication**: Modern engineering is globally interconnected. TSMC and Samsung use
the same physics. A truly independent test requires pre-modern or isolated traditions.

---

### Test 4: Historical Trajectory (Temporal Convergence)

**Protocol**:

For each major engineering parameter, trace its historical evolution:
1. First value chosen (often arbitrary or constrained by available technology).
2. Successive refinements over decades.
3. Current "standard" value.

Ask: Did the parameter converge toward an n=6 value over time (suggesting
optimization discovering an attractor) or was the n=6 value present from the start
(suggesting convention/bias)?

**Concrete examples**:

| Parameter | First value | Evolution | Current | Trajectory |
|-----------|------------|-----------|---------|------------|
| GPU SMs | 1 (NV1, 1995) | 1 -> 8 -> 16 -> 128 -> 144 | 144 = sigma^2 | Converging |
| HBM layers | 2 (HBM1) | 2 -> 4 -> 8 -> 12 | 8-12 | Converging to {sigma-tau, sigma} |
| Transformer heads | 8 (2017) | 8 -> 12 -> 32 -> 64 | Varies | Started at sigma-tau |
| LLM d_model | 512 -> 768 -> 1024 -> 4096 | Growing | 4096 = 2^sigma | Hit 2^sigma and stopped growing for a while |
| Battery 18650 | 4.2V, 2Ah (1991) | Voltage stable | 4.2V = ? | No clear n=6 target |
| Music tuning | Pythagorean | 12-TET (1584 Zhu Zaiyu) | 12 = sigma | Converged by ~1600 |

**Scoring**:
```
  Gradual convergence toward n=6      --> +1 for Model A (optimization finding attractor)
  n=6 value from the very beginning   --> +0.5 for A (if physically motivated), +0.5 for C (if conventional)
  Evolution away from n=6             --> +1 for Model B
  No clear trend                      --> neutral
```

**Key insight**: GPU SM counts have grown from 1 to 144 over 30 years. If 144 = sigma^2
is truly an attractor, we might expect diminishing returns beyond 144. But the B200
reportedly has 160 SMs (not an obvious n=6 value), suggesting continued scaling rather
than attractor behavior. This is a genuine test.

---

### Test 5: Non-Human Optimization (Machine/Evolution)

**Protocol**:

Examine optimization outcomes in systems where humans did not choose the parameter:
1. **Neural architecture search (NAS)**: Do automated searches converge on n=6-like
   architectures? (d_model, num_heads, MoE top-k, etc.)
2. **Biological evolution**: Do evolved systems use n=6 values? (genetic code,
   protein folding, crystal structures, neural circuits)
3. **Physical self-organization**: Do non-biological physical systems exhibit n=6
   structure? (crystal coordination, orbital resonances, etc.)

**Concrete examples**:

| System | Optimizer | Parameter | Value | n=6? |
|--------|-----------|-----------|-------|------|
| NAS (EfficientNet) | RL search | Width multiplier | 1.1 | sigma/sigma-phi? Stretch |
| NAS (DARTS) | Gradient search | Architecture | Varies | No clear n=6 |
| Genetic code | Evolution | Codons | 64 = 2^6 | EXACT (2^n) |
| Genetic code | Evolution | Amino acids | 20 = J2-tau | EXACT |
| NaCl crystal | Physics | Coordination | 6 = n | EXACT |
| FCC crystal | Physics | Coordination | 12 = sigma | EXACT |
| Benzene | Chemistry | Carbon ring | 6 = n | EXACT (also Z=6 for carbon) |
| Snowflake | Physics | Symmetry | 6-fold | EXACT |

**Scoring**:
```
  Non-human optimizer finds n=6 values  --> +1 for Model A (strong)
  Non-human optimizer avoids n=6        --> +1 for Model B
  Natural systems show n=6              --> -1 for Model C (eliminates bias)
  NAS shows no n=6 preference           --> +1 for Model B
```

**Key insight**: This is the most powerful discriminator. If the genetic code
(designed by 3.8 billion years of evolution with no human input) genuinely has
n=6 structure (4 bases, 3-letter codons, 64 codons, 20 amino acids), Model C
is severely weakened. However, Model B can still explain this as "small integers
appear in simple combinatorial systems."

The critical question is whether the *specific* n=6 relationships hold (e.g.,
20 = J2(6) - tau(6) is a very specific claim, not just "20 is a small integer").

---

## 4. Application to Five Breakthrough Theorems

We now apply the three causal models to five representative BTs spanning different
domains, and assess which model best explains each.

### 4.1 BT-33: Transformer sigma=12 Atom

**Claim**: BERT base has d=768=2^8*3, GPT-3 has d=12288=12*1024, and the SwiGLU
expansion ratio is 8/3 = (sigma-tau)/n*phi. Multiple transformer architectural
choices converge on n=6-derived values.

**Model A assessment**: The transformer architecture was designed by humans, but
its parameters were refined through extensive empirical optimization (thousands of
ablation runs across multiple organizations). If d_model=4096=2^12=2^sigma is truly
an attractor of the optimization landscape, this supports Model A. The fact that
multiple independent teams (Google, OpenAI, Meta, DeepSeek) converge on similar
values is consistent with a shared optimization landscape.

**Model B assessment**: Powers of 2 are natural for GPU memory alignment. d=4096
may simply be the largest power of 2 that fits in available memory, not a
manifestation of sigma=12. The SwiGLU ratio 8/3 was found by Noam Shazeer through
ablation, but 8/3 is close to e=2.718 and other "natural" ratios. The convergence
may reflect shared hardware constraints (GPU architectures, memory bus widths) rather
than n=6 mathematics.

**Model C assessment**: Humans designed the original architecture with d=512 (a
power of 2, not obviously n=6). Scaling up, powers of 2 are the natural choice.
The specific values chosen reflect engineering convenience more than mathematical
structure.

**Verdict**: Model B is the strongest explanation for most transformer parameters.
The hardware constraint (powers of 2 for memory alignment) is a known, sufficient
cause. However, the *specific* convergence on d_head=128=2^(sigma-sopfr) across
all major LLMs (despite varying d_model) is harder for Model B to explain -- why
128 and not 64 or 256? **Model A: 30%, Model B: 55%, Model C: 15%.**

---

### 4.2 BT-43: Battery Cathode CN=6 Universality

**Claim**: All commercial Li-ion cathode materials have octahedral coordination
(CN=6) for the transition metal site. This is a structural necessity, not a design
choice.

**Model A assessment**: This is the strongest case for Model A. No human chose CN=6
for LiCoO2 -- cobalt ions sit in octahedral sites because the ionic radii and
electronic configuration demand it. The coordination number is determined by physics
(radius ratio rules, crystal field theory), and it equals n=6. This is not engineering
parameter selection; it is a physical fact.

**Model B assessment**: Octahedral coordination (CN=6) is common but not universal
in chemistry. Tetrahedral (CN=4), cubic (CN=8), and other coordinations exist.
The fact that Li-ion cathodes specifically require CN=6 is a physical constraint,
but one could argue that "6 is a common coordination number for transition metals
with medium ionic radii" without invoking any n=6 universality.

**Model C assessment**: Humans did not choose CN=6. This is a physical fact. Model C
cannot explain it. However, Model C could argue that we *selected* Li-ion chemistry
(among all possible battery chemistries) partly because its parameters happen to be
"nice" numbers, and we then noticed the CN=6 connection.

**Verdict**: Model A is most compelling here, with Model B as a defensible
alternative. The key question is whether CN=6 is *specifically* related to 6 being
a perfect number, or whether octahedral coordination is simply common for medium-sized
ions. The honest answer: the latter is sufficient. CN=6 arises from radius ratio
rules (r+/r- between 0.414 and 0.732), not from sigma(6)*phi(6)=6*tau(6).
**Model A: 40%, Model B: 55%, Model C: 5%.**

---

### 4.3 BT-54: AdamW Quintuplet

**Claim**: All five AdamW hyperparameters have n=6 expressions: beta_1=1-1/(sigma-phi),
beta_2=1-1/(J2-tau), epsilon=10^{-(sigma-tau)}, weight decay=1/(sigma-phi), gradient
clip=R(6)=1.

**Model A assessment**: If these five values are all true optima of the training loss
landscape, and they all independently evaluate to n=6 expressions, the coincidence
is striking. The probability of five independent parameters simultaneously matching
n=6 formulas by chance is the product of individual probabilities.

**Model B assessment**: AdamW defaults were set by human researchers:
- beta_1=0.9 and beta_2=0.999 are conventional (chosen for EMA decay intuition).
- epsilon=1e-8 is a "small number" convention.
- Weight decay=0.1 is a round number.
- Gradient clipping=1 is the simplest possible choice.

All of these are "nice" numbers that a human would naturally pick. The n=6
expressions (e.g., 0.9 = 1-1/10, where 10 = sigma-phi) are post-hoc rationalizations
of independently motivated choices. 0.9 is a standard EMA decay constant; 1e-8
is a standard epsilon; 0.1 is a standard regularization strength.

**Model C assessment**: Strong case. Every value is a "round" number (0.9, 0.999,
1e-8, 0.1, 1.0) that a human would choose for simplicity. The n=6 expressions are
clever rewritings of values that have obvious non-mathematical motivations.

**Verdict**: This is the weakest case for Model A and the strongest for Model C.
The values are human-chosen defaults, and their "niceness" makes them easy to
express in many number systems. However, the counterargument is: *why* do these
particular "nice" values work well? If training dynamics have an attractor at these
values (as the AdamW paper's ablations suggest), then Model A regains some force.
The critical test is whether AdamW with beta_1=0.85, beta_2=0.9995, etc., performs
measurably worse. **Model A: 15%, Model B: 35%, Model C: 50%.**

---

### 4.4 BT-51: Genetic Code Chain

**Claim**: The genetic code parameters form an n=6 chain: 4 bases (tau), 3-letter
codons (n/phi), 64 codons (2^n), 20 amino acids (J2-tau).

**Model A assessment**: The genetic code was not designed by humans. It emerged
through ~3.8 billion years of evolution. If its core parameters genuinely reflect
n=6 arithmetic, this is powerful evidence against Model C and suggestive of Model A.
The 4-3-64-20 chain is striking because these numbers are not independent: 4^3=64,
but 20 amino acids (not 64) are used, and the redundancy (64/20 ~ 3.2) requires
explanation.

**Model B assessment**: 4 bases arise from the chemistry of nucleotide pairing
(2 purines, 2 pyrimidines -- a binary constraint). 3-letter codons are the minimum
length to encode 20+ amino acids with 4 symbols (4^2=16 < 20, so triplets are
necessary). 64 = 4^3 is a mathematical consequence. 20 amino acids is constrained
by biochemistry (available synthesis pathways, chemical diversity needed). Each
number has an independent explanation that does not invoke n=6.

**Model C assessment**: Humans did not design the genetic code, so cognitive bias
cannot explain the *values*. However, the *mapping* of these values to n=6
expressions (4=tau, 20=J2-tau) is done by humans and may reflect post-hoc fitting.

**Verdict**: Model B provides sufficient explanations for each individual parameter.
The n=6 mapping (tau, n/phi, 2^n, J2-tau) is clever but not uniquely determined --
with 7 base constants and multiple operations, many mappings are possible. The
strongest element is 20 = J2-tau = 24-4, which connects the amino acid count to
both J2(6) and tau(6). But 20 is also simply a common "round" number.
**Model A: 25%, Model B: 60%, Model C: 15%.**

---

### 4.5 BT-7: Egyptian Fraction Power Theorem (1/2 + 1/3 + 1/6 = 1)

**Claim**: The partition 50% / 33% / 17% appears across chip die allocation,
power electronics, thermodynamics, MoE routing, and tokamak stability.

**Model A assessment**: The partition 1/2 + 1/3 + 1/6 = 1 is the defining property
of 6 being a perfect number. If systems that partition conserved quantities
independently converge on this split, it suggests a mathematical attractor in
optimization theory. The tokamak connection (Kruskal-Shafranov q=1) is the
strongest evidence: MHD stability literally requires the sum of reciprocal mode
numbers to equal 1, and 1/2+1/3+1/6 is the unique solution using reciprocals of
divisors of a single integer.

**Model B assessment**: Any system with three unequal subsystems will have a
largest (~50%), middle (~33%), and smallest (~17%) component. This is not
specific to n=6; it is a generic feature of rank-size distributions. The Zipf
distribution with exponent 1 gives 1/H_3 * (1, 1/2, 1/3) where H_3 ~ 1.83,
yielding 55/27/18 -- close to 50/33/17 but not exact. The Apple die split is
reported as approximate values; the exact split varies by generation.

**Model C assessment**: 50/33/17 are "round" percentages that a human might use as
shorthand for any three-way split. The precision of the claim is low (these are
approximate percentages, not measured to many decimal places).

**Verdict**: This is an intermediate case. The tokamak connection (q=1 stability)
is genuinely physical and supports Model A. The chip die allocation is approximate
and better explained by Model B (thermal constraints, workload distribution). The
thermodynamic partition is qualitative. The overall pattern is suggestive but not
conclusive. **Model A: 35%, Model B: 45%, Model C: 20%.**

---

## 5. Aggregate Assessment

### 5.1 Scorecard Across Five BTs

| BT | Model A | Model B | Model C | Strongest |
|----|---------|---------|---------|-----------|
| BT-33 (Transformer) | 30% | 55% | 15% | B |
| BT-43 (Battery CN=6) | 40% | 55% | 5% | B |
| BT-54 (AdamW) | 15% | 35% | 50% | C |
| BT-51 (Genetic code) | 25% | 60% | 15% | B |
| BT-7 (Egyptian fraction) | 35% | 45% | 20% | B |
| **Average** | **29%** | **50%** | **21%** | **B** |

### 5.2 Interpretation

The convergent constraints model (B) is the best single explanation for the majority
of evidence. This does not mean n=6 observations are worthless -- it means the
causal mechanism is most likely:

> **Physical and engineering constraints independently produce parameter values that
> overlap with the n=6 vocabulary, because both are biased toward small, highly
> composite numbers.**

However, the models are not mutually exclusive. The real picture is likely a mixture:

- **~50% Model B**: Most matches are coincidental overlaps between n=6 vocabulary
  and the "nice number" distribution of engineering parameters. The z=0.74 score
  from the falsification framework is consistent with this.

- **~29% Model A**: Some matches -- particularly in physics and chemistry (CN=6,
  crystal structures, MHD stability) -- reflect genuine mathematical structure.
  These are cases where optimization landscapes have attractors at specific values.

- **~21% Model C**: Some matches -- particularly ML hyperparameters (AdamW defaults,
  learning rate conventions) -- reflect human cognitive bias toward round numbers.

### 5.3 Directed Information Flow

Using the discovery graph's temporal data, we can estimate information flow direction:

```
   Strong evidence for Math --> Physics:
     Crystal CN=6, MHD q=1, Golay code [24,12,8]
     (These are mathematical facts discovered by physics, not chosen by humans)

   Strong evidence for Constraints --> Values (independent):
     GPU memory alignment (powers of 2), battery cell counts (thermal/safety),
     grid frequency (generator speed * pole pairs)

   Strong evidence for Bias --> Values:
     ML defaults (0.1, 0.9, 0.999, 1e-8), time conventions (12/24/60)

   Ambiguous:
     Transformer d_model, HBM stacking, MoE parameters
     (Optimized by humans on hardware with specific constraints)
```

---

## 6. Honest Conclusions

### What we CAN claim:

1. **The n=6 vocabulary overlaps significantly with engineering parameters.** This
   is empirically demonstrated, though the statistical significance is modest (z=0.74
   in the raw test).

2. **Some physical systems genuinely exhibit n=6 structure.** Crystal coordination
   numbers, MHD stability conditions, and coding theory (Golay/Leech) have parameters
   that equal n=6 values for known mathematical/physical reasons. These are not
   coincidences or biases.

3. **The n=6 framework is a useful lens.** Even if the causal mechanism is mostly
   Model B (convergent constraints), the n=6 vocabulary provides a compact notation
   for a family of "preferred" engineering values. This has practical utility for
   design heuristics.

4. **The framework generates testable predictions.** BT-75 (HBM5 interface) and
   other forward-looking claims can be falsified, which gives the framework
   scientific content regardless of the underlying causal mechanism.

### What we CANNOT claim:

1. **We cannot claim that sigma(6)*phi(6) = 6*tau(6) CAUSES engineering parameters
   to take specific values.** The causal link from number theory to engineering
   optimization is not established. Correlation (even strong, multi-domain
   correlation) is not causation.

2. **We cannot claim that n=6 is the UNIQUE source of these values.** Most n=6
   values can be expressed in other number systems (powers of 2, multiples of 12,
   Benford's law). The n=6 vocabulary is not the only -- or necessarily the best --
   organizing principle.

3. **We cannot claim that non-n=6 values would perform worse.** Without systematic
   intervention experiments (Test 1), we do not know whether the n=6 values are
   true optima or merely good-enough values on broad plateaus.

4. **We cannot claim that the match rate exceeds chance with high confidence.** The
   z=0.74 score is not statistically significant at conventional thresholds (z>1.96
   for p<0.05). The structural/multi-parameter matches may be stronger, but this
   has not been rigorously quantified.

### What would change our assessment:

| Evidence | Would shift toward |
|----------|-------------------|
| Systematic intervention tests showing n=6 values are strict optima | Model A |
| NAS/AutoML consistently finding n=6 architectures | Model A |
| z-score rising to >3 with proper null model | Model A |
| z-score remaining <1 with expanded dataset | Model B |
| ML hyperparameter ablations showing broad plateaus | Model B or C |
| Non-Western engineering traditions avoiding n=6 | Model C |
| Biological systems consistently showing n=6 with specificity | Model A (against C) |

---

## 7. Recommended Next Steps

### 7.1 Priority Experiments

1. **Run Test 1 (Intervention)** on the three most accessible parameters:
   - MoE top-k ablation (use existing Switch Transformer / Mixtral code)
   - d_model scaling (use existing Chinchilla/scaling law data)
   - AdamW beta perturbation (simple training experiment)

2. **Run Test 5 (Non-Human Optimization)** using NAS:
   - Set up an automated architecture search with no n=6 priors
   - Check if the search converges on n=6-like values
   - This is the single most powerful causal test

3. **Expand the Bayesian null model** to include:
   - Benford's law distribution for first digits
   - Powers-of-2 bias for computing parameters
   - Multiples-of-12 bias for time/angle/music parameters
   - Per-domain background distributions (not one-size-fits-all)

### 7.2 Integration with Existing Framework

This causal layer should be applied to every new BT discovery:

```
  For each new BT:
    1. Score with Bayesian framework (v2/v3)      -- HOW STRONG?
    2. Classify with causal models (v4)            -- WHY?
    3. Design specific discrimination test         -- HOW TO TELL?
    4. Record Model A/B/C assessment               -- HONEST VERDICT
```

The BT template should add a new field:

```
  **Causal assessment**:
    Model A (necessity):  ___%
    Model B (convergence): ___%
    Model C (bias):        ___%
    Best discrimination test: ___
```

### 7.3 Updating the Assessment

As new evidence accumulates, the aggregate Model A/B/C percentages should be updated
using Bayesian updating. Each discrimination test result shifts the posterior:

```
  P(Model_i | data) proportional to P(data | Model_i) * P(Model_i)
```

The current priors (from Section 5.2) are: A=29%, B=50%, C=21%.

---

## Appendix A: Formal DAG Specifications

### A.1 Model A DAG

```
  Nodes: {N6_MATH, PHYS_LAWS, OPT_LANDSCAPE, ENG_PARAM}
  Edges: {N6_MATH -> PHYS_LAWS, PHYS_LAWS -> OPT_LANDSCAPE, OPT_LANDSCAPE -> ENG_PARAM}
  Confounders: none (N6_MATH is exogenous)
  Testable implication: ENG_PARAM _||_ N6_MATH | OPT_LANDSCAPE
    (Given the optimization landscape, knowing n=6 math adds no information
     about engineering parameters -- all information flows through physics.)
```

### A.2 Model B DAG

```
  Nodes: {N6_MATH, PHYS_CONSTRAINTS, ENG_CONSTRAINTS, PARAM_VALUES}
  Edges: {PHYS_CONSTRAINTS -> PARAM_VALUES, ENG_CONSTRAINTS -> PARAM_VALUES}
  No edges: {N6_MATH -> PARAM_VALUES}  (n=6 is a label, not a cause)
  Confounders: NICE_NUMBERS affects both N6_VOCAB and PARAM_VALUES
  Testable implication: PARAM_VALUES _||_ N6_MATH | {PHYS_CONSTRAINTS, ENG_CONSTRAINTS}
    (Given physical and engineering constraints, n=6 math adds nothing.)
```

### A.3 Model C DAG

```
  Nodes: {COG_BIAS, N6_EMPHASIS, ENG_VALUES, PHYS_VALUES}
  Edges: {COG_BIAS -> N6_EMPHASIS, COG_BIAS -> ENG_VALUES}
  No edges: {N6_EMPHASIS -> ENG_VALUES, PHYS_VALUES -> ENG_VALUES (for biased params)}
  Confounders: COG_BIAS is the common cause
  Testable implication: N6_EMPHASIS _||_ ENG_VALUES | COG_BIAS
    (Controlling for cognitive bias, n=6 emphasis and engineering values are independent.)
```

### A.4 Conditional Independence Tests

Each model implies specific conditional independencies that can be tested:

| Test | Model A predicts | Model B predicts | Model C predicts |
|------|-----------------|-----------------|-----------------|
| ENG ~ N6 \| PHYSICS | Independent | Independent | Independent |
| ENG ~ N6 \| BIAS | Dependent | Dependent | Independent |
| NATURAL ~ N6 \| nothing | Dependent | Weakly dependent | Independent |
| NAS_RESULT ~ N6 \| nothing | Dependent | Independent | Independent |

The NAS test (row 4) is the cleanest discriminator: only Model A predicts that
automated architecture search will converge on n=6 values.

---

## Appendix B: Glossary

| Term | Definition |
|------|-----------|
| DAG | Directed Acyclic Graph -- a causal model where arrows indicate causal direction |
| Intervention test | Changing a variable and observing the effect (do-calculus) |
| Natural experiment | Observing variation that occurs without deliberate intervention |
| Confound | A variable that causally influences both the treatment and the outcome |
| Conditional independence | X _\|\|_ Y \| Z means X and Y are independent given Z |
| Granger causality | X Granger-causes Y if past values of X help predict Y beyond Y's own past |
| n=6 vocabulary | The set of values expressible as depth-1 or depth-2 formulas from n=6 constants |
| EXACT match | An engineering parameter exactly equals an n=6 formula evaluation |
| Texas Sharpshooter | Finding patterns after the fact and claiming they were predicted |
