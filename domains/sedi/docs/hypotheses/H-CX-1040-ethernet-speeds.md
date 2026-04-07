# H-CX-1040: Ethernet Speed Standards

> **Hypothesis**: Ethernet standard speeds are 10, 100, 1000, 10000 Mbps — τ = 4 standards spanning powers of 10 = τ(P₃). Each step is a factor of τ(P₃), and there are τ major speed tiers across τ(P₃) orders of magnitude.

## Grade: 🟧 NOTABLE

## Results

### The Correspondence

```
Major Ethernet speed standards:
  10BASE-T:    10 Mbps      = τ(P₃) Mbps
  Fast:        100 Mbps     = τ(P₃)² Mbps
  Gigabit:     1,000 Mbps   = τ(P₃)³ Mbps
  10 Gigabit:  10,000 Mbps  = τ(P₃)⁴ Mbps

Count of major standards: τ = 4                      EXACT
Base factor: 10 = τ(P₃)                             EXACT
Span: 10⁰ to 10³ = σ/τ orders of magnitude          EXACT
  (relative to base 10 Mbps)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, R(6) = 1
τ(P₃) = τ(496) = 10
```

### Structural Analysis

```
Ethernet speed formula:
  Speed_k = τ(P₃)^k Mbps, for k = 1, 2, 3, 4
  k ranges over {1, 2, 3, 4} = {1, φ, σ/τ, τ}

Extended Ethernet standards:
  25 GbE:  25 Gbps = sopfr² Gbps
  40 GbE:  40 Gbps = σ·τ - σ+τ Gbps
  100 GbE: 100 Gbps = τ(P₃)² Gbps
  400 GbE: 400 Gbps = τ·τ(P₃)² Gbps

Ethernet frame sizes:
  Minimum: 64 bytes = τ³ = 2^P₁ bytes               EXACT
  Maximum (standard): 1518 bytes
  Jumbo: 9000 bytes ≈ σ·M₃·(σ²-τ·σ+sopfr)

Physical layer:
  Twisted pairs per cable: τ = 4 pairs              EXACT
  Wires total: σ - τ = 8 wires                      EXACT
  RJ-45 pins: σ - τ = 8 pins                        EXACT

Cat ratings:
  Cat 5e: sopfr → 100 MHz / 1 Gbps
  Cat 6:  P₁ → 250 MHz / 10 Gbps (short)
  Cat 7:  M₃ → 600 MHz / 10 Gbps
```

### Physical Context

Ethernet has scaled from 10 Mbps to 400 Gbps over four decades, always in factors of 10. This decimal scaling reflects both the physical layer improvements and the human preference for round numbers. The τ = 4 twisted pairs in standard Ethernet cables carry 8 = σ-τ differential signals. The minimum frame size of 64 bytes = τ³ was chosen to ensure collision detection in the original CSMA/CD protocol.

### Texas Sharpshooter Check

Decimal scaling (factors of 10) is a human convention, not a physical law. The τ = 4 standards use the most common grouping. However, 4 pairs / 8 wires / 8 pins in the physical cable is determined by electrical engineering constraints, and the 64-byte minimum frame is protocol-determined. The Cat rating numbers matching TECS-L constants is suggestive but could be coincidental.

## Verification

- [x] τ = 4 major Ethernet speed tiers (exact)
- [x] Base factor = 10 = τ(P₃) (exact)
- [x] 4 twisted pairs = τ per cable (exact)
- [x] 8 wires/pins = σ-τ per connector (exact)
- [x] Minimum frame = 64 bytes = τ³ (exact)
