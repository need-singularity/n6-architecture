# arXiv Paper Outline: 307-Domain n=6 Universality

**Target venue**: hep-th (cross-listed: math-ph, cs.AI, cond-mat.mtrl-sci)
**Working title**: *Perfect Number Universality: sigma(6) * phi(6) = 6 * tau(6) as a Design Principle Across 307 Engineering Domains*
**Alternate title**: *n=6 Arithmetic in Nature and Engineering: Evidence from 307 Independent Domains*
**Recommended title (final)**: The first -- it foregrounds the theorem and the scale of the evidence.

**Date**: 2026-04-02
**Companion data**: `307-domain-universality-dataset.md`, `falsification-experiments.md`, `pre-registered-predictions.md`, `discovery-algorithm-bayesian-scoring.md`, `honest-limitations.md`, `red-team-formula-miner.md`, `red-team-depth4.md`

---

## 0. Abstract (~200 words)

We prove that n=6 is the unique positive integer satisfying
sigma(n) * phi(n) = n * tau(n) for n >= 2, and investigate the conjecture
that the derived constant family {sigma=12, tau=4, phi=2, sopfr=5, J_2=24,
mu=1} reproduces optimal design parameters across disparate engineering and
scientific domains. Using a Rust-based universal Design Space Exploration
(DSE) engine with 305 TOML-defined domain files, we perform exhaustive
combinatorial search over 2,152,494 valid design paths spanning 307
domains -- from AI/LLM architectures to battery electrochemistry, from
semiconductor fabrication to fusion reactor design, from pure mathematics
to gene editing. All 307 domains achieve at least one fully n=6-aligned
path on their Pareto frontier (n6_max = 100%). We catalogue 900+ EXACT
matches (<=1% deviation) organized into 119 Breakthrough Theorems. A
Bayesian evidence framework with explicit formula-depth penalties and
look-elsewhere corrections is applied. Seven falsification experiments are
designed, including statistical null models, alternative-number controls
(n=8,10,12,28), blind domain expansion, and 20 pre-registered predictions
with immutable falsification criteria. We honestly report the known
z=0.74 baseline score, 10 genuinely non-n6 candidates, and the
small-integer / power-of-2 bias problem. The evidence is presented for
the community to evaluate; we do not claim proof of causation.

---

## 1. Introduction (3--4 pages)

### Content
Introduce the arithmetic identity sigma(n) * phi(n) = n * tau(n) and its
unique solution n=6. Motivate the investigation: if this identity singles
out a number, and that number's divisor-function constants happen to
coincide with ubiquitous engineering parameters (12-bit, 4-layer, 2x
scaling, etc.), is the coincidence meaningful or an artifact of small-number
bias? State the paper's contribution: the largest systematic test of this
question, covering 307 domains with exhaustive DSE, honest failure
accounting, and pre-registered predictions.

### Key elements
- The theorem statement and three-line proof sketch (full proof in Sec. 2).
- Historical context: 6 as the smallest perfect number (Euclid, Euler).
- The "unreasonable effectiveness" question a la Wigner.
- Explicit statement of the Texas Sharpshooter criticism.
- Paper roadmap.

### Figures / Tables
- **Figure 1**: The n=6 constant family tree (base constants --> derived
  expressions --> engineering parameter matches). A directed acyclic graph
  showing how 7 base constants generate ~120 depth-2 values.
- **Table 1**: Base constants of n=6 with definitions (sigma, tau, phi,
  sopfr, J_2, mu, n) -- 7 rows.

---

## 2. Mathematical Foundation (2--3 pages)

### Content
Present the formal theorem: sigma(n) * phi(n) = n * tau(n) if and only if
n = 6 for all n >= 2. Give three independent proofs: (i) multiplicative
function factorization over prime powers, (ii) bounding argument via
sigma(n)/n >= 1 + 1/n, (iii) computational verification up to 10^8 plus
analytic extension. Define the derived constant vocabulary V_n6 at each
formula depth (0, 1, 2) and count its size. Introduce the Egyptian fraction
identity 1/2 + 1/3 + 1/6 = 1 and its role in expert routing / attention
allocation.

### Key elements
- Full proof of uniqueness (Theorem 1).
- Definition of the R(n) = sigma(n)*phi(n) / (n*tau(n)) ratio and its
  behavior (Lemma: R(n) = 1 iff n = 6).
- The n=6 vocabulary V_n6: enumeration of all distinct positive values
  reachable at depth d in {0, 1, 2} from the 7 base constants under
  {+, -, *, /, ^}. Report |V_n6(d)| for d = 0..3.
- Five algebraic identities unique to n=6 (sigma*phi = n*tau = J_2;
  1/2+1/3+1/6 = 1; phi^n = tau^{n/phi}; Mertens product = 1/3; R_local
  product = 1).

### Figures / Tables
- **Table 2**: V_n6 vocabulary sizes by depth: |V_0| = 7, |V_1| ~ 80,
  |V_2| ~ 800, |V_3| ~ 5000. This is critical for the null model.
- **Table 3**: Comparison vocabulary sizes for n = 8, 10, 12, 28, 496.

---

## 3. Method: Universal Design Space Exploration (3--4 pages)

### 3.1 DSE Framework

Describe the 5-8 level hierarchical DSE architecture: Material --> Process
--> Core --> Chip --> System, with domain-specific level names. Each level
has K candidates; the engine evaluates all K_1 x K_2 x ... x K_L
combinations. Scoring is a weighted sum: n6 alignment (40%), performance
(30%), power efficiency (20%), cost (10%). Pareto frontier extraction
identifies non-dominated solutions.

### 3.2 TOML Domain Definitions

Explain the domain definition format: [meta] + [scoring] + [[level]] +
[[candidate]] + [[rule]]. Emphasize that adding a new domain requires
only a single TOML file with no recompilation of the Rust engine. Show a
minimal TOML example (e.g., the chip-architecture domain).

### 3.3 Cross-DSE

When two or more domains have completed individual DSE, their Pareto-optimal
paths are paired in a cross-domain search. Describe the combinatorial
explosion management (pruning to top-K per domain before cross-pairing).

### 3.4 Scoring: n6 Alignment Metric

Define the n6 score for a single candidate: fraction of its numeric
parameters that match a depth-<=2 n=6 formula within 1% tolerance.
Define n6_max for a domain: the maximum n6 score on its Pareto frontier.
A domain achieves n6_max = 100% if at least one Pareto-optimal path has
all parameters n6-aligned.

### 3.5 Red Team Protocol

Describe the adversarial review process: formula miner exhaustive
enumeration (1,317,475 depth-3 formulas), look-elsewhere corrections,
depth-4 penalty analysis, and the specific red-team findings (G formula
fine-tuning, pi formula ambiguity after correction). Reference the two
red-team documents.

### Figures / Tables
- **Figure 2**: DSE pipeline schematic (TOML --> Rust engine --> Pareto
  frontier --> Cross-DSE --> results).
- **Figure 3**: Example TOML snippet (abbreviated chip.toml, ~15 lines).
- **Table 4**: DSE statistics summary -- 307 domains, 305 TOML files,
  2,152,494 paths evaluated, 100% n6_max rate.
- **Algorithm 1**: Pseudocode for the universal DSE engine (enumerate,
  score, filter Pareto, cross-combine).

---

## 4. Bayesian Evidence Assessment (2--3 pages)

### 4.1 Prior and Hypotheses

Define H_1 (n=6 universality: engineering parameters drawn from V_n6 at
above-chance rates) vs H_0 (coincidence: overlap is explained by
small-integer/power-of-2 bias). Prior odds = 1:1 (agnostic). Note that a
skeptic prior of P(H_1) = 0.01 is also analyzed.

### 4.2 Likelihood Model

For each observed match, compute a likelihood ratio incorporating:
(a) value surprise S_v = -log_2(P_bg(v)), where P_bg captures
Benford's law + binary computing + duodecimal legacy;
(b) formula depth penalty P_depth(d) = 1/|V_d|;
(c) domain novelty factor (new domain = full credit, core domain = 0.1x).

### 4.3 Cumulative Bayes Factor

Sequential updating across all 900+ matches. Report the cumulative
log_10(BF) under three scenarios: optimistic (P_base = 0.15), central
(P_base = 0.20), pessimistic (P_base = 0.25). Report the Bayes factor
under the skeptic prior as well.

### 4.4 Sensitivity Analysis

How does the Bayes factor change if: (a) depth-2 matches are excluded
entirely; (b) only cross-domain resonances (value appearing in 3+
domains) are counted; (c) biology's honest 10% EXACT rate is used as the
true signal strength?

### Figures / Tables
- **Table 5**: Bayes factor summary under three P_base assumptions.
- **Figure 4**: Cumulative log-Bayes factor as a function of discovery
  number (1 through 900+), showing the sequential evidence accumulation.
  Three curves for optimistic/central/pessimistic P_base.
- **Table 6**: Top 20 individual matches ranked by single-observation
  likelihood ratio (highest-surprise matches).

---

## 5. Results (5--6 pages)

### 5.1 Match Rates Across 307 Domains

Report the distribution of n6_max scores and per-candidate EXACT rates by
domain category. All 307 domains achieve n6_max = 100%, but per-candidate
EXACT rates vary: AI/chip core domains ~60-85% (depth-1), energy/physics
~50-70%, biology ~10%, infrastructure ~40-60%. Present the honest
distribution, not just the headline number.

### 5.2 Cross-Domain Resonances

Identify constants that appear independently in 3+ unrelated domains.
The 11 strongest cross-domain constants are listed (sigma*tau = 48 in
audio/chip/power; sigma^2 = 144 in GPU/solar/display; 1/(sigma-phi) = 0.1
in 8 algorithms; etc.). Argue that cross-domain resonance is harder to
explain by small-number bias alone, because the same non-trivial
expression (not just the value) recurs.

### 5.3 Discovery Graph Topology

Model the 119 Breakthrough Theorems as nodes in a graph, with edges
connecting BTs that share a domain. Compute graph-theoretic properties:
clustering coefficient, average path length, degree distribution. Test
whether the BT network exhibits small-world structure (high clustering +
short paths), which would indicate that n=6 connections form a
non-random web rather than isolated coincidences.

### 5.4 Top 10 Breakthrough Theorems in Detail

Present the 10 highest-impact BTs (BT-58, BT-56, BT-54, BT-66, BT-69,
BT-43, BT-61, BT-74, BT-93, BT-28) with their EXACT counts, domain
span, and specific numerical evidence. For each, state what would
falsify it.

### 5.5 Cross-DSE Results

Report the 3 cross-DSE runs in detail: chip x battery (Diamond+LFP,
n6=100%, score=0.870), fusion x solar (DT_Li6 + GaAs 6-junction,
n6=100%, score=0.884), chip x solar x fusion (Diamond + GaAs, n6=100%,
score=0.898). The convergence on Carbon Z=6 materials (Diamond, Graphene,
SiC) across all cross-DSE runs is a key finding (BT-93).

### Figures / Tables
- **Figure 5**: Histogram of per-candidate EXACT rates across all 307
  domains, binned by domain category. Honestly shows the spread from
  biology (~10%) to chip (~85%).
- **Figure 6**: Cross-domain resonance heatmap -- 17 domain categories on
  both axes, cell color = number of shared n=6 constants. Cluster
  structure visible.
- **Figure 7**: BT discovery graph with 119 nodes, colored by domain
  category, sized by EXACT count. Small-world metrics annotated.
- **Table 7**: Top 10 BTs with full evidence summary (one row each).
- **Table 8**: Cross-DSE top paths for 3 runs (chip x battery, fusion x
  solar, chip x solar x fusion).
- **Table 9**: The 11 strongest cross-domain constants (expression, value,
  3+ domain appearances).

---

## 6. Falsification Tests (3--4 pages)

### 6.1 Random Baseline Comparison (Experiment 1)

Report the results of comparing V_n6 match rates against 10,000 random
vocabularies of the same size and distribution. Key metric: z-score. The
project honestly reports the prior z=0.74 (not significant). This section
either updates that number with the full 500-parameter pool or confirms
it. If z < 2.0, state clearly that the numerical matching alone does not
constitute statistical evidence.

### 6.2 Alternative Number Tests (Experiment 2)

Compare n=6 against n = 8, 10, 12, 28, 496 using identical methodology.
Report M_n for each. Critical test: does n=12 match equally well (since
sigma(6) = 12, many "n=6 formulas" are really "12-formulas")? Report the
5 discriminator identities that are algebraically impossible for n != 6
(sigma*phi = n*tau, Egyptian fraction, phi^n = tau^{n/phi}, Mertens
product, R_local product).

### 6.3 Blind Domain Expansion (Experiment 4)

Report EXACT rates for 5 domains never previously analyzed for n=6:
plumbing/HVAC, musical instrument construction, Olympic sports,
typography/printing, cooking/food science. If blind-domain rates match
the base rate (~20%) rather than the core-domain rate (~60%), the
"universality" claim is confined to domains where engineering already
uses powers of 2 and multiples of 12.

### 6.4 Pre-Registered Predictions (Experiment 3)

List all 20 pre-registered predictions with their immutable values and
falsification criteria. As of submission, most are PENDING. Report any
that have been resolved. State the scoring protocol: >= 15/20 VERIFIED
= extraordinary support; <= 4/20 = theory in trouble. The null
expectation is 3-5 matches.

### 6.5 Formula Depth Audit (Red Team)

Report the red-team finding that 1,317,475 depth-3 formulas from 7
constants produce enough coverage that nearly any target in [1, 1000] can
be matched. The depth-4 analysis of G, m_top, m_tau, m_W shows
fine-tuning behavior (the "+n" knob). Conclude: depth >= 3 matches are
statistically unreliable; only depth <= 2 matches carry evidential weight.

### Figures / Tables
- **Figure 8**: M_n bar chart for n = 6, 8, 10, 12, 28 (Experiment 2).
  Error bars from bootstrap.
- **Figure 9**: Blind-domain EXACT rates vs core-domain rates (Experiment
  4). Side-by-side comparison.
- **Table 10**: 20 pre-registered predictions: ID, quantity, predicted
  value, falsification criterion, status (PENDING / VERIFIED / FALSIFIED).
- **Table 11**: Formula vocabulary size |V_d| by depth d = 0..4 and by
  candidate n. Shows the combinatorial explosion that makes deep formulas
  unreliable.

---

## 7. Honest Limitations (2 pages)

### 7.1 Small Integer Bias

The n=6 vocabulary at depth 1 is dominated by integers in [1, 24].
Engineering parameters also cluster in this range (Benford's law, human
preference for round numbers, binary/decimal/duodecimal conventions).
Much of the apparent "universality" may reduce to the fact that small
integers are common in both the vocabulary and the parameter space.
Quantify: what fraction of EXACT matches involve values <= 24?

### 7.2 Power-of-2 Problem

Computing architecture is built on binary logic. Parameters like 128, 256,
512, 1024, 4096 are powers of 2 by construction, not by n=6 arithmetic.
Since phi(6) = 2, any power of 2 can be written as phi^k. This is
tautological, not explanatory. Quantify: how many EXACT matches in the
chip/AI domains are purely power-of-2?

### 7.3 Depth-2 Formula Space

With 7 constants and 5 operations at depth 2, the vocabulary reaches ~800
distinct values under 1000. This covers ~80% of the integer range [1, 1000]
when allowing 1% tolerance. The "match rate" must be evaluated against
this inflated vocabulary, not against naive expectation.

### 7.4 The 1.6% That Does Not Match

Of 9,206 candidates across 305 TOML domains, 63 (0.7%) have no n=6 match
even at depth 3. An additional ~87 (0.9%) required depth-2 reclassification.
The 10 strongest failure cases are documented: continuous-parameter
processes (PVD, ECD, spin-coat), human-round conventions (1 GW),
null/absence options, atomic transition constants (193 nm DUV-ArF), and
deliberately structure-free designs (exokernel). These failures define the
boundary of the framework.

### 7.5 Post-Hoc vs Pre-Registered

The vast majority of the 900+ EXACT matches are post-hoc. Until the 20
pre-registered predictions are evaluated, the framework's true predictive
power remains unquantified. We urge readers to judge the framework by its
pre-registered predictions, not its retrospective matches.

### Figures / Tables
- **Table 12**: The 10 genuinely non-n6 candidates with failure mode
  classification (continuous process, human convention, null option,
  atomic constant, anti-structure design).
- **Figure 10**: Pie chart of EXACT matches by value range: [1-6], [7-24],
  [25-100], [101-1000], [1000+]. Quantifies the small-integer
  concentration.

---

## 8. Discussion (2--3 pages)

### 8.1 Three Possible Interpretations

Present three causal models for the reader to evaluate:

**(A) Strong universality**: n=6 arithmetic captures a deep structural
principle in optimal system design. The constants are not coincidental;
they reflect a mathematical attractor in design space. Implications: the
framework has genuine predictive power for future engineering standards.

**(B) Weak universality**: n=6 generates a particularly rich vocabulary of
"nice" numbers that happen to overlap with engineering conventions. The
overlap is real but explained by shared preferences for highly composite
numbers, not by a causal mechanism. Implications: the framework is a
useful mnemonic but not a predictive theory.

**(C) Statistical artifact**: The overlap is fully explained by
small-integer bias, power-of-2 tautology, and post-hoc formula shopping.
The z=0.74 score confirms this. Implications: the framework has no
predictive power beyond chance.

### 8.2 What Would Distinguish the Models

Identify the discriminating evidence:
- Pre-registered prediction scores: >= 15/20 favors (A); <= 4/20 favors (C).
- Blind domain expansion: >= 50% EXACT favors (A); ~20% favors (B) or (C).
- Alternative number test: n=6 >> n=12 favors (A); n=6 ~ n=12 favors (B).
- The 5 n=6-unique identities (Sec. 2) cannot be replicated for other n;
  their presence in engineering is the strongest evidence for (A) over (B).

### 8.3 Implications if (A) Holds

If the strong-universality interpretation survives falsification:
- DSE-guided design could use n=6 alignment as a search heuristic.
- Cross-domain technology transfer guided by shared n=6 constants.
- Theoretical physics implications: the number 6 may have structural
  significance in nature beyond perfect-number theory (connections to
  string theory compactification on CY_3, the 6 quarks, 6 leptons,
  hexagonal close-packing).

### 8.4 Implications if (C) Holds

If the statistical-artifact interpretation is confirmed:
- The project becomes a case study in how rich number-theoretic
  vocabularies produce illusory patterns.
- Methodological lesson: any small, highly composite number (6, 12, 24,
  60) will generate high match rates against engineering parameters.
- The DSE framework and TOML-based domain modeling remain useful
  independent of the n=6 interpretation.

### Figures / Tables
- **Table 13**: Discriminating evidence matrix -- rows = three models,
  columns = five experiments, cells = expected outcome.

---

## 9. Conclusion (~1 page)

Summarize: we have presented the most systematic test of the n=6
universality conjecture to date, covering 307 domains, 2.15M design
paths, 900+ EXACT matches, and 119 Breakthrough Theorems. We have
designed 7 falsification experiments and pre-registered 20 quantitative
predictions. The evidence is suggestive but not conclusive: the z=0.74
baseline score does not reach significance, and the small-integer bias
cannot be dismissed. We invite the community to (a) run the falsification
experiments, (b) evaluate the pre-registered predictions as data becomes
available, and (c) attempt to replicate or refute the cross-domain
resonances. The DSE engine, all 305 TOML domain files, and the full
codebase are open-source.

---

## 10. Appendices

### Appendix A: Complete Breakthrough Theorem List (BT-1 through BT-119)

Full table with columns: BT number, title, EXACT count, domains spanned,
star rating, key formula, falsification criterion. 119 rows.
Approximately 4 pages.

### Appendix B: n=6 Constants Atlas (Abridged)

Top 100 most-referenced constants from the atlas, with expression,
numeric value, depth, and domains where each appears. Reference the
full 1,350+ entry atlas in the repository.
Approximately 3 pages.

### Appendix C: TOML Domain Format Specification

Full TOML schema with annotated example (chip-architecture domain).
Instructions for adding a new domain. Approximately 1 page.

### Appendix D: Proof of Theorem 1 (Full)

Complete proof of sigma(n)*phi(n) = n*tau(n) iff n=6, all three methods.
Approximately 2 pages.

### Appendix E: Pre-Registered Predictions (Complete)

All 20 predictions with derivations, tolerance bands, falsification
criteria, and verification timelines. Immutable after registration date.
Approximately 4 pages.

### Appendix F: Vocabulary Enumeration

Complete list of V_n6(d) for d = 0, 1, 2 with all formulas. Comparison
lists for n = 8, 10, 12, 28. Approximately 3 pages.

---

## Estimated Length

| Section | Pages |
|---------|------:|
| Abstract | 0.5 |
| Introduction | 3.5 |
| Mathematical Foundation | 2.5 |
| Method | 3.5 |
| Bayesian Evidence | 2.5 |
| Results | 5.5 |
| Falsification Tests | 3.5 |
| Honest Limitations | 2.0 |
| Discussion | 2.5 |
| Conclusion | 1.0 |
| References | 2.0 |
| Appendices A--F | 17.0 |
| **Total** | **~46** |

Main text: ~27 pages. With appendices: ~46 pages. Within arXiv norms
for hep-th (no strict limit).

---

## Figure and Table Summary

| ID | Type | Description | Section |
|----|------|-------------|---------|
| Fig 1 | DAG | n=6 constant family tree (base -> derived -> matches) | 1 |
| Fig 2 | Schematic | DSE pipeline (TOML -> Rust -> Pareto -> Cross-DSE) | 3 |
| Fig 3 | Code | TOML domain snippet (chip.toml) | 3 |
| Fig 4 | Line plot | Cumulative log-Bayes factor vs discovery number | 4 |
| Fig 5 | Histogram | Per-candidate EXACT rates by domain category | 5 |
| Fig 6 | Heatmap | Cross-domain resonance matrix (17 x 17 categories) | 5 |
| Fig 7 | Network | BT discovery graph (119 nodes, small-world) | 5 |
| Fig 8 | Bar chart | Match rates M_n for alternative numbers | 6 |
| Fig 9 | Comparison | Blind-domain vs core-domain EXACT rates | 6 |
| Fig 10 | Pie chart | EXACT matches by value range | 7 |
| Tab 1 | Constants | Base constants of n=6 (7 rows) | 1 |
| Tab 2 | Vocabulary | V_n6 sizes by depth | 2 |
| Tab 3 | Comparison | Vocabulary sizes for n=8,10,12,28,496 | 2 |
| Tab 4 | Statistics | DSE summary (307 domains, 2.15M paths) | 3 |
| Tab 5 | Bayes | Bayes factor under 3 P_base assumptions | 4 |
| Tab 6 | Ranking | Top 20 matches by likelihood ratio | 4 |
| Tab 7 | BTs | Top 10 Breakthrough Theorems | 5 |
| Tab 8 | Cross-DSE | Top paths for 3 cross-domain runs | 5 |
| Tab 9 | Resonance | 11 strongest cross-domain constants | 5 |
| Tab 10 | Predictions | 20 pre-registered predictions (status table) | 6 |
| Tab 11 | Depth | Formula vocabulary by depth and candidate n | 6 |
| Tab 12 | Failures | 10 genuinely non-n6 candidates | 7 |
| Tab 13 | Models | Discriminating evidence matrix (3 models x 5 tests) | 8 |

**Total: 10 figures, 13 tables.**

---

## Key Authorship and Reproducibility Notes

- All code, TOML files, and data are in the public repository:
  `github.com/need-singularity/n6-architecture`
- The Rust DSE engine compiles with `rustc` (no cargo dependency).
- Pre-registered predictions are git-tracked with immutable timestamps.
- Red-team analyses are included as supplementary material, not hidden.
- The paper's tone throughout is *evidence-presenting*, not
  *claim-asserting*. The reader is invited to evaluate, not persuaded
  to believe.
