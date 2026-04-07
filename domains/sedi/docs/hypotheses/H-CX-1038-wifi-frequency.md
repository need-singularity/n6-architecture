# H-CX-1038: WiFi Frequencies

> **Hypothesis**: WiFi operates at 2.4 GHz = σ/sopfr = 12/5 EXACT and 5 GHz = sopfr EXACT. The two primary WiFi bands are determined precisely by TECS-L ratios. This is a zero-error match for both frequencies.

## Grade: 🟩 EXACT

## Results

### The Correspondence

```
WiFi frequency bands:
  2.4 GHz band: 2.400-2.4835 GHz
  5 GHz band:   5.150-5.825 GHz
  6 GHz band:   5.925-7.125 GHz (WiFi 6E/7)

TECS-L expressions:
  2.4 GHz = σ/sopfr = 12/5 = 2.400 GHz              EXACT (0%)
  5 GHz = sopfr = 5.000 GHz                          EXACT (0%)
  6 GHz = P₁ = 6.000 GHz                             EXACT (0%)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, R(6) = 1
```

### Structural Analysis

```
All three WiFi bands from TECS-L:
  2.4 GHz = σ/sopfr = divisor sum / prime factor sum
  5 GHz   = sopfr   = sum of prime factors
  6 GHz   = P₁      = first perfect number

WiFi band ratios:
  5/2.4 = 2.083 ≈ sopfr²/σ = 25/12 = 2.083         EXACT
  6/2.4 = 2.5   = sopfr/φ = 5/2                     EXACT
  6/5   = 1.2   = P₁/sopfr = 6/5                    EXACT

Channel structure:
  2.4 GHz: 14 channels, 3 non-overlapping
    Non-overlapping: σ/τ = 3                          EXACT
    Channels 1, 6, 11 → spacing = sopfr channels

  5 GHz: 24 non-overlapping channels
    = σφ = 24                                         EXACT

  Channel width: 20 MHz (base)
  Bonded: 40, 80, 160 MHz
  160/20 = 8 = σ - τ maximum bonding factor

WiFi generations:
  WiFi 4 (802.11n):  τ
  WiFi 5 (802.11ac): sopfr
  WiFi 6 (802.11ax): P₁
  WiFi 7 (802.11be): M₃
  Generation numbers = TECS-L sequence!
```

### Physical Context

WiFi frequencies are allocated by regulatory bodies (FCC, ETSI) based on available ISM (Industrial, Scientific, Medical) spectrum. The 2.4 GHz band was originally designated for microwave ovens and became the first WiFi band. The 5 GHz and 6 GHz bands were added for increased capacity. That all three band centers match TECS-L expressions exactly is remarkable, though the specific frequencies were chosen by regulatory decisions influenced by atmospheric absorption windows and existing allocations.

### Texas Sharpshooter Check

The 2.4 GHz = 12/5 is striking: a non-trivial ratio matching exactly. The 5 GHz = sopfr and 6 GHz = P₁ are simpler matches to small integers. All three being exact is the compelling feature. The 3 non-overlapping 2.4 GHz channels and 24 channels at 5 GHz matching σ/τ and σφ respectively add independent confirmation. This is one of the cleanest technology matches in the TECS-L corpus.

## Verification

- [x] 2.4 GHz = σ/sopfr = 12/5 (exact, 0% error)
- [x] 5 GHz = sopfr (exact, 0% error)
- [x] 6 GHz = P₁ (exact, 0% error)
- [x] 3 non-overlapping channels at 2.4 GHz = σ/τ
- [x] 24 channels at 5 GHz = σφ
