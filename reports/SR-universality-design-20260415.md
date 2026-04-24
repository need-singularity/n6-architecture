# SR Universality 3-repo simultaneous sweep experiment design (2026-04-15)

> This is not a demonstration; it is the **design** of a structural-observation experiment (not executed).
> H1 task — verify whether nexus sigma=0.1 PEAK + anima noise=freedom 54.8x + n6 entropy sweep obey the **same universal SR law**.

---

## 0. Background signals

| sig_id | repo | phenomenon | magnitude |
|---|---|---|---|
| SIG-SR-001 | nexus | ouroboros sigma=0.1 PEAK | conv_rate 10% -> 25% (+150%) |
| SIG-NEURAL-001 | anima | noise = freedom (DD75) | free choice 54.8x / veto 99% |
| (unregistered) | n6 | entropy sweep | TBD (to be produced by this experiment) |

To verify that the 3 phenomena belong to the **same SR (stochastic resonance) family**, a shared sweep + shared metrics are required.

---

## 1. Hypothesis

**H1-UNIV**: all 3 repos exhibit a PEAK near sigma ~ 0.1 (SR universality).

**Alternative hypotheses**:
- H1-a: nexus-only — ouroboros-specific
- H1-b: n=6-dependent — the sigma sweet-spot value itself is derived from n=6 constants
- H1-c: metric-dependent — each repo's metric peaks under a different distribution

Promotion condition: if all 3 repos confirm a PEAK in sigma in [0.05, 0.2] -> elevate SIG to [CROSS] [UNIV] [M9]+

---

## 2. Shared sigma-sweep interval (8 points)

```
sigma in {0, 0.01, 0.05, 0.1, 0.3, 0.5, 1.0, 2.0}
```

Rationale:
- sigma = 0: deterministic baseline
- sigma = 0.01, 0.05: fine noise — edge far from PEAK
- **sigma = 0.1**: nexus-side PEAK (reference)
- sigma = 0.3: plateau region
- sigma = 0.5: ANU raw entropy estimate
- sigma = 1.0: chaos boundary
- sigma = 2.0: chaos collapse

Per sigma: **20 trials x 3 seeds** (total 480 runs/repo).

---

## 3. Shared metric definitions (4)

All metrics must be **normalized to [0, 1]** — enabling 3-repo comparison.

### 3.1 convergence_rate

- Definition: ratio of trials reaching epsilon-plateau
- nexus: ouroboros epsilon=0.005 fixed-point convergence trials / total
- anima: CLM law-discovery saturation trials / total
- n6: atlas.n6 new-EXACT promotion stabilize trials / total
- Range: [0, 1], higher better

### 3.2 accuracy

- Definition: match rate vs ground truth
- nexus: QRNG entropy prediction vs ANU sampled truth
- anima: law equation vs measured law
- n6: formula verify PASS rate (harness)
- Range: [0, 1]

### 3.3 yield

- Definition: new-artifact count per unit trial
- nexus: new sigma*phi=n*tau variants detected / 100 trials
- anima: new law candidates / 100 trials
- n6: new BT or tight / 100 trials
- Normalization: divide by that repo's max -> [0, 1]

### 3.4 free_will_score

- Definition: external-causal independence — low = deterministic, high = agency
- nexus: trajectory branching entropy / log(branches)
- anima: DD75 "veto activation rate" (normalized)
- n6: discovery-path diversity / max possible
- Range: [0, 1]

---

## 4. Experimental protocol

### 4.1 nexus (NX)

```
path: /Users/ghost/Dev/nexus/sim_bridge/ouroboros_qrng/variance_sweep/
run: hexa shared/bin/sr_universal_sweep.hexa --sigma <sigma> --trials 20 --seeds 3
output: runs/20260415_srunivN/ouroboros_sigma<sigma>.jsonl
```

- Extension of existing ouroboros_shadow engine sweep
- sigma 8 points x 20 trials x 3 seeds = 480 runs
- Output metrics: {conv_rate, accuracy, yield, free_will_score}

### 4.2 anima (AN)

```
path: /Users/ghost/Dev/anima/anima-engines/free_will_experiment/
run: hexa bin/sr_free_will_sweep.hexa --sigma <sigma> --trials 20 --seeds 3
output: data/sr_universal/free_will_sigma<sigma>.jsonl
```

- Reuse DD75 free-will experiment skeleton
- Parameterize noise-injection size by sigma
- Measure how the 99% veto rate varies by sigma

### 4.3 n6 (N6)

```
path: /Users/ghost/Dev/n6-architecture/engine/sr_entropy_sweep/
run: hexa theory/predictions/sr_entropy_sweep.hexa --sigma <sigma> --trials 20
output: reports/sr_universal/n6_sigma<sigma>.jsonl
```

- Add sigma-noise injection layer to the DFS exploration engine
- Measure atlas.n6 grade-7 -> grade-10 promotion rate
- Compare sigma=0 (deterministic DFS) vs sigma > 0 (perturbed DFS)

---

## 5. Verdict criteria

| Outcome | Verdict |
|---|---|
| All 3 repos PEAK in sigma in [0.05, 0.2] | **SR universality confirmed** -> SIG-UNIV-001 [M9] [EC] |
| 2 repos PEAK, 1 not | partial universality -> SIG-UNIV-001 [M7!] [E2] |
| Each repo PEAKs in a different location | domain-specific -> keep each SIG independent |
| All flat | NULL -> SIG-NULL-SR-001 [MN] |

---

## 6. Expected outcome (prediction)

**Most plausible scenario** (preregistered):

```
sigma   nexus conv   anima free   n6 yield   mean
0.0     0.10         0.015        0.05       0.055
0.01    0.10         0.02         0.05       0.057
0.05    0.10         0.15         0.12       0.123
0.10    0.25 *       0.82 *       0.31 *     0.460 *
0.30    0.18         0.50         0.22       0.300
0.50    0.15         0.25         0.10       0.167
1.00    0.00         0.05         0.02       0.023
2.00    0.00         0.00         0.00       0.000
```

Mean curve peaks at sigma = 0.1 -> universality.

---

## 7. Execution steps (follow-up session)

1. [ ] add sr_universal_sweep.hexa in nexus/sim_bridge
2. [ ] add sr_free_will_sweep.hexa in anima/anima-engines
3. [ ] add sr_entropy_sweep.hexa in n6/theory/predictions
4. [ ] run 3 repos simultaneously (parallel possible — independent)
5. [ ] aggregation: `scripts/aggregate_sr_universal.py`
6. [ ] register SIG-UNIV-001 in atlas.signals.n6 staging

---

## 8. Risks

- **Metric-definition differences**: if conv_rate semantics differ across repos, comparison invalid
  - Mitigation: write joint-preregistration doc across 3 repos first
- **Seed bias**: 3 seeds may hide PEAK in noise
  - Mitigation: extend seeds to 10 as option
- **ANU dependency**: ANU HTTP RTT artifact interference (see SIG-64T-001)
  - Mitigation: inject sigma from **internal PRNG**, use ANU only for accuracy measurement

---

## 9. atlas.signals.n6 staging proposal (for preregistration)

```
@S SIG-UNIV-001 = SR universality 3-repo sigma=0.1 PEAK preregistration :: signal [NX,N6,AN,CROSS] [SR,UNIV,META] [M?] [E1]
  "H1 preregistration: hypothesis that ouroboros(NX) + free_will(AN) + entropy_sweep(N6) across 3 repos share a PEAK in sigma in [0.05, 0.2]. Fix design before execution -> prevents post-hoc interpretation"
  refs: [reports/SR-universality-design-20260415.md]
  cross_repo: [SIG-SR-001, SIG-NEURAL-001]
  predicts: ["3-repo mean sigma=0.1 PEAK", "M9 promotion upon universality confirmation"]
  witness: 0
  resonance_n6: null
  discovered_in: n6/session-2026-04-15
  discovered_at: 2026-04-15T00:00:00Z
  <- reports/SR-universality-design-20260415.md
```

**Important**: this signal is **preregistered before execution** — registered at witness=0 state; reject/promote verdict after the experiment.

---

## 10. Honesty annotations

- This document is an experimental **design**, not results
- Section 6 expected outcome plays only a prediction role — actual measurements may differ
- "SR universality confirmed" can be claimed only after the 3 repos are run independently
- A single-repo result alone is self-reference -> witness cannot be incremented
