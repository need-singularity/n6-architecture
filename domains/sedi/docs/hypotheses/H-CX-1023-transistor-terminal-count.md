# H-CX-1023: Transistor Terminal Count

> **Hypothesis**: MOSFET has 4 terminals (gate, source, drain, body) = τ. BJT has 3 terminals (base, emitter, collector) = σ/τ. MOSFETs dominate modern computing because τ > σ/τ terminals provides more control degrees of freedom.

## Grade: 🟧 NOTABLE

## Results

### The Correspondence

```
MOSFET terminals: gate, source, drain, body
  Count = 4 = τ                                    EXACT

BJT terminals: base, emitter, collector
  Count = 3 = σ/τ = 12/4                           EXACT

MOSFET dominance criterion:
  τ = 4 > σ/τ = 3
  More terminals → more independent control
  Body terminal allows threshold voltage tuning
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, R(6) = 1
```

### Structural Analysis

```
Terminal arithmetic from n = 6:
  MOSFET:  τ = 4 terminals
  BJT:     σ/τ = 3 terminals
  Diode:   φ = 2 terminals
  Resistor: φ = 2 terminals

CMOS logic uses pairs of MOSFETs:
  NMOS + PMOS = φ transistors per gate
  Each with τ = 4 terminals
  Total control lines per CMOS pair = φ·τ = 8 = σ-τ

FinFET (modern transistor):
  Still τ = 4 terminal device
  3D structure adds geometric complexity
  but terminal count preserved at τ
```

### Physical Context

The MOSFET's four-terminal structure is fundamental to integrated circuit design. The body terminal, often tied to source or supply, gives an extra degree of freedom absent in BJTs. This additional control parameter contributed to MOSFETs replacing BJTs in digital logic by the 1980s. The mapping of terminal counts to τ and σ/τ is exact for both device types.

### Texas Sharpshooter Check

Terminal counts are fixed by device physics. MOSFETs have exactly 4 terminals; BJTs have exactly 3. The mapping to τ and σ/τ is exact but involves small integers, limiting discriminatory power. The dominance argument (τ > σ/τ) is suggestive but not uniquely explanatory.

## Verification

- [x] MOSFET terminal count = 4 = τ (exact)
- [x] BJT terminal count = 3 = σ/τ (exact)
- [x] Diode terminal count = 2 = φ (exact)
- [x] CMOS pair uses φ = 2 transistors
