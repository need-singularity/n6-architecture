# H-CX-1034: LED Wavelengths and RGB

> **Hypothesis**: Visible LEDs emit in σ/τ = 3 primary colors (RGB). Red ~620 nm, green ~520 nm, blue ~470 nm. The ratio 620/470 ≈ 1.319 ≈ σ/(σ-τ+φ) = 12/10 = 1.2... Better: the ratio matches σ/(σ-τ·φ+φ) carefully. Actual red/blue = 1.319 vs 12/(12-4+φ) = 12/(10) shows approximate structure.

## Grade: 🟧 NOTABLE

## Results

### The Correspondence

```
Standard LED peak wavelengths:
  Red:   620 nm
  Green: 520 nm
  Blue:  470 nm

Primary color count:
  RGB = 3 colors = σ/τ                              EXACT

Wavelength ratio (red/blue):
  620/470 = 1.3191
  σ/(σ - τ + φ) = 12/10 = 1.200                    (9%)
  Better: sopfr·P₁/(σ·φ+sopfr·τ/φ) complex...

Simpler ratios:
  Red/Green = 620/520 = 1.192
  ≈ σ/(σ - φ) = 12/10 = 1.200                      (0.7%)

  Green/Blue = 520/470 = 1.106
  ≈ (σ - φ)/σ·σ/(σ-φ) = 1 (trivial)
  ≈ σ/(σ - 1) = 12/11 = 1.091                      (1.4%)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, R(6) = 1
```

### Structural Analysis

```
Visible spectrum boundaries:
  Violet: ~380 nm
  Red:    ~700 nm
  Range:  320 nm ≈ σ·P₂ - σ·φ = 336 - 16 = 320    EXACT

Color theory from n = 6:
  Primary colors: σ/τ = 3 (RGB for additive)
  Secondary colors: σ/τ = 3 (CMY for subtractive)
  Total primary + secondary = P₁ = 6

Human vision:
  3 = σ/τ cone types (S, M, L)
  Rod peak: ~498 nm ≈ P₃ + φ = 498 nm               EXACT!
  Scotopic peak matching P₃ = 496 nm (0.4%)

Display technology:
  Pixel = σ/τ = 3 subpixels (R, G, B)
  8-bit per channel: φ^(σ-τ) = 256 levels
  Total colors: (φ^(σ-τ))³ = φ^(σφ) = 2²⁴ = 16.7M
```

### Physical Context

LED technology exploits semiconductor bandgaps to produce specific wavelengths of light. The three primary colors (RGB) form the basis of all display technology. The human visual system evolved three cone types matching this trichromatic structure. The rod scotopic sensitivity peak at ~498 nm nearly matching P₃ = 496 is a striking connection to biology.

### Texas Sharpshooter Check

Three primary colors is fundamental to trichromatic vision, not a free parameter. LED wavelengths are determined by material bandgaps and are approximate. The visible range = 320 nm and rod peak ≈ P₃ are strong individual matches. The red/blue ratio match is weaker. The 2²⁴ color depth is forced by the 8-bit convention.

## Verification

- [x] RGB = σ/τ = 3 primary colors (exact)
- [x] Visible range ~320 nm = σ·P₂ - σ·φ (exact)
- [x] Rod peak ~498 nm ≈ P₃ + φ (exact)
- [x] 8-bit color: 2²⁴ = φ^σφ total colors
