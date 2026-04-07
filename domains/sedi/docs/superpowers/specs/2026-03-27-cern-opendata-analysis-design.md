# CERN Open Data + TECS-L Analysis Design

## Goal

Systematically analyze 300+ particle physics measurements using TECS-L's full mathematical framework (σ,τ,φ,R,S system), generate testable predictions, and validate with CERN-grade statistics (Monte Carlo, Look-Elsewhere Effect, Bonferroni correction).

## Architecture

```
PDG Database (300+ particles)
        │
        ▼
┌─────────────────────┐
│  TECS-L Math Engine  │  ← σ,τ,φ,R,S + Koide + fermion formulas
│  (sedi/tecs.py)      │     + 206 characterizations
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│  Pattern Scanner     │  ← pairwise ratios, 3-body, trig system
│  (sedi/sources/      │     mass spectrum R-filter
│   cern_analysis.py)  │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│  Prediction Engine   │  ← generate testable mass predictions
│  (in cern_analysis)  │     from TECS-L formulas
└─────────┬───────────┘
          │
          ▼
┌─────────────────────┐
│  Statistical Engine  │  ← Monte Carlo 100k, Bonferroni,
│  (sedi/statistics.py)│     Look-Elsewhere Effect
└─────────┬───────────┘
          │
          ▼
    Results + Report
```

## Components

### 1. PDG Particle Database (`sedi/sources/pdg.py`)

Comprehensive particle database: ~80 well-measured particles (masses, widths, lifetimes). Organized by category: leptons, quarks, gauge bosons, mesons, baryons. Each entry has mass (GeV), uncertainty, PDG status.

### 2. TECS-L Math Engine (`sedi/tecs.py`)

Port the core mathematical functions from TECS-L:
- σ,τ,φ,R,S computation for any n
- n=6 target constants: {σ/τ=3, σ=12, τ=4, φ=2, n=6, σφ=24, sopfr=5}
- Koide angle δ=φτ²/σ² = 2/9
- Fermion mass formulas from TECS-L
- Trig system: sin(π/6)=φ/τ, tan²(π/6)=τ/σ
- Cross-perfect-number relations: σφ+τ=28, C(σ-τ,φ)=28
- Egyptian fraction: φ/τ+τ/σ+1/n=1 (1/2+1/3+1/6=1)

### 3. Pattern Scanner (`sedi/sources/cern_analysis.py`)

Replace existing `cern.py` analysis with comprehensive scanner:
- **Pairwise ratios**: all (N choose 2) mass ratios checked against TECS-L targets
- **Triple relations**: 3-body mass ratio sums checked for 1/2+1/3+1/6=1
- **Koide check**: apply Koide formula to each generation (e,μ,τ), (u,c,t), (d,s,b)
- **Mass spectrum R-filter**: treat mass list as signal, run R-filter FFT
- **Dimensional checks**: particle counts matching σ,τ,φ values

### 4. Prediction Engine (in `cern_analysis.py`)

Generate testable predictions using TECS-L formulas:
- Predict unmeasured/poorly-measured particle masses from TECS-L arithmetic
- Output predictions with confidence intervals from the formula system
- Flag which predictions are most testable at CERN

### 5. Statistical Validation (`sedi/statistics.py`)

- **Monte Carlo**: generate 100k random particle mass sets (same count, log-uniform distribution matching observed mass range), run identical analysis, compute empirical p-values
- **Bonferroni correction**: adjust for number of tests performed
- **Look-Elsewhere Effect (LEE)**: compute trial factor from search window
- **Effect size**: report not just significance but magnitude of deviation
- **Summary table**: each finding with raw p, corrected p, LEE-corrected p, verdict

### 6. CLI Integration

New command: `sedi history --source cern-analysis`
Output: structured report with sections for each analysis type, summary table, predictions.

## Data Flow

1. Load PDG database → list of (name, mass, uncertainty, category)
2. Run all TECS-L pattern checks → raw hits list
3. For each hit: Monte Carlo p-value + Bonferroni + LEE correction
4. Generate predictions from TECS-L formulas
5. Format comprehensive report

## Success Criteria

- Scan 80+ particles (all well-measured ones)
- Apply 10+ TECS-L mathematical tests
- Every finding has Monte Carlo p-value with multiple comparison correction
- Generate at least 3 testable predictions with error bars
- Output readable report suitable for inclusion in a paper
