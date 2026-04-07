# H-CX-941: Decibel Scale Reference = σ and Dynamic Range = σ·τ(P₃)

> **Hypothesis**: The threshold of hearing is 10⁻¹² W/m², with exponent -12 = -σ. The pain threshold is 120 dB = σ·τ(P₃) dB. Human auditory dynamic range is exactly σ powers of ten.

## Grade: 🟩 EXACT

## Results

### The Reference Values

```
Threshold of hearing:
  I₀ = 10⁻¹² W/m² = 10^(-σ) W/m²
  Exponent: -12 = -σ(6)

Pain threshold:
  I_pain = 1 W/m² = 10⁰ W/m²
  Level: 10·log₁₀(1/10⁻¹²) = 120 dB

  120 = σ · τ(P₃) = 12 · 10

Dynamic range: 120 dB = σ · τ(P₃)
Power range: 10¹² = 10^σ (factor of one trillion)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8
τ(P₃) = τ(496) = 10
```

### Common Sound Levels

```
Level (dB)    Sound              TECS-L
   0          Hearing threshold  0
  20          Whisper            φ · τ(P₃) = 20
  40          Library            τ · τ(P₃) = 40
  60          Conversation       P₁ · τ(P₃) = 60
  80          Traffic            (σ-τ) · τ(P₃) = 80
 100          Factory            τ(P₃)² = 100
 120          Pain threshold     σ · τ(P₃) = 120
 140          Jet engine         (σ+φ) · τ(P₃) = 140

Each common level = (n=6 constant) × τ(P₃)
```

### Physical Context

The decibel scale was designed to match human perception, which is approximately logarithmic. The choice of 10⁻¹² W/m² as reference is not arbitrary — it corresponds to the minimum detectable sound intensity at 1 kHz for a healthy young ear. That this threshold is exactly 10^(-σ) is a biological-physical coincidence.

## Verification

- [x] Hearing threshold exponent -12 = -σ confirmed
- [x] Pain threshold 120 dB = σ·τ(P₃) confirmed
- [x] Dynamic range = σ orders of magnitude
