# H-CX-633: Axion Mass Prediction m_a ≈ 0.84 μeV from f_PQ

> **Hypothesis**: Using f_PQ = v·P₃^τ = 1.49×10¹³ GeV (H-CX-632), the axion mass m_a = f_π·m_π/f_PQ ≈ 0.84 μeV, within the ADMX haloscope detection range.

## Grade: 🟧★ (testable prediction; awaiting experimental confirmation)

## Results

### The Calculation

```
m_a ≈ f_π · m_π / f_PQ     (leading order)

f_π  = 93 MeV       (pion decay constant)
m_π  = 135 MeV      (neutral pion mass)
f_PQ = 1.49 × 10¹³ GeV = 1.49 × 10²² eV    (H-CX-632)

m_a = (93 × 10⁶ × 135 × 10⁶) / (1.49 × 10²² )
    = 1.2555 × 10¹⁶ / 1.49 × 10²²
    = 8.43 × 10⁻⁷ eV
    = 0.84 μeV
```

### More Precise Formula

```
m_a = √(z)/(1+z) × f_π × m_π / f_PQ

where z = m_u/m_d ≈ 0.48 (up/down quark mass ratio)

√(0.48)/(1.48) = 0.693/1.48 = 0.468

m_a = 0.468 × 93 × 135 / (1.49 × 10¹³) MeV
    = 0.468 × 12555 / (1.49 × 10¹³) MeV
    = 5876 / (1.49 × 10¹³) MeV
    = 3.94 × 10⁻¹⁰ MeV
    = 0.394 μeV

Note: √z ≈ √(0.48) = 0.693 ≈ ln(2) = ln(φ(6))!
```

### Detection Prospects

```
Axion mass range from f_PQ = 1.49×10¹³ GeV:
  m_a ≈ 0.4 - 0.8 μeV (depending on QCD corrections)

Experimental sensitivity:
  ADMX:        0.2 - 40 μeV (current and planned)   ← IN RANGE
  ABRACADABRA: broadband search, sub-μeV             ← IN RANGE
  MADMAX:      40 - 400 μeV                          ← above
  CASPEr:      sub-neV to μeV                        ← IN RANGE

This prediction is TESTABLE within current experimental programs.
```

### Axion Dark Matter Density

```
If m_a ≈ 0.4-0.8 μeV, axion contribution to dark matter:
  Ω_a ∝ (f_PQ/10¹²)^(7/6) ≈ (14.9)^(7/6) ≈ 25

This suggests axions at this mass could comprise a significant
fraction of dark matter, possibly requiring tuned initial misalignment
angle θ_i < 1 to avoid overproduction.
```

### Connection to Other Hypotheses

- H-CX-632: f_PQ = v·P₃^τ = 1.49×10¹³ GeV
- H-CX-634: Axion-photon coupling from this mass
- H-CX-635: Strong CP bound θ < 10⁻¹⁰
- H-CX-535: Dark matter ratio

## Status

- [x] m_a ≈ 0.4-0.8 μeV computed from f_PQ
- [x] Within ADMX detection range
- [x] √(m_u/m_d) ≈ ln(φ(6)) coincidence noted
- [ ] ADMX detection at this frequency (~100 MHz)
