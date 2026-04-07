# H-CX-634: Axion-Photon Coupling g_aγγ from n=6

> **Hypothesis**: The axion-photon coupling g_aγγ ∝ α/(2π·f_PQ). From n=6: g_aγγ ~ 1/(σφ·π·v·P₃^τ), detectable by ABRACADABRA and ADMX.

## Grade: 🟧 (structural; model-dependent E/N ratio)

## Results

### The Formula

```
g_aγγ = (α / (2π · f_PQ)) × |E/N - 1.92|

α    = 1/137.036     (fine structure constant)
2π   = φ(6) · π      (φ = 2)
f_PQ = 1.49 × 10¹³ GeV   (H-CX-632)
E/N  = model-dependent (electromagnetic/color anomaly ratio)
```

### n=6 Decomposition

```
The prefactor: α/(2π) = 1/(137 × 2π) = 1/(137 × 6.283)
             = 1/860.7 ≈ 1.162 × 10⁻³

Full coupling: g_aγγ = α·|E/N - 1.92| / (2π · f_PQ)

From n=6 constants:
  2π = φ · π
  f_PQ = v · P₃^τ

  g_aγγ ~ α / (φ · π · v · P₃^τ) × |E/N - 1.92|
```

### Numerical Estimate

```
For KSVZ axion model (E/N = 0):
  g_aγγ = (1/137) × 1.92 / (2π × 1.49×10¹³)
        = 0.01401 / (9.36 × 10¹³)
        = 1.50 × 10⁻¹⁶ GeV⁻¹

For DFSZ axion model (E/N = 8/3):
  |E/N - 1.92| = |2.667 - 1.92| = 0.747
  g_aγγ = (1/137) × 0.747 / (2π × 1.49×10¹³)
        = 5.83 × 10⁻¹⁷ GeV⁻¹
```

### Detection Experiments

```
Experiment           Sensitivity (GeV⁻¹)    Status
ADMX                ~10⁻¹⁶                  Running (μeV range)
ABRACADABRA         ~10⁻¹⁶ - 10⁻¹⁹         Running (broadband)
IAXO (helioscope)   ~10⁻¹²                  Under construction
ALPS II (LSW)       ~10⁻¹¹                  Running

For m_a ~ 0.4-0.8 μeV (H-CX-633):
  g_aγγ ~ 10⁻¹⁶ - 10⁻¹⁷ GeV⁻¹

  → ADMX and ABRACADABRA can probe this region
```

### Physical Context

The axion-photon coupling is the primary experimental handle for
axion detection. It enables:
- Axion-to-photon conversion in magnetic fields (Primakoff effect)
- Haloscope detection (resonant cavity in B field)
- Helioscope detection (solar axions → photons)

### Connection to Other Hypotheses

- H-CX-632: f_PQ = v·P₃^τ
- H-CX-633: m_a ≈ 0.4-0.8 μeV prediction
- H-CX-635: Strong CP mechanism

## Status

- [x] g_aγγ ~ 10⁻¹⁶ GeV⁻¹ for KSVZ model
- [x] Within ADMX/ABRACADABRA sensitivity
- [x] 2π = φ·π decomposition in coupling
- [ ] E/N ratio determination from n=6 (model-dependent)
- [ ] Experimental detection
