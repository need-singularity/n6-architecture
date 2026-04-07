# H-CX-956: Synaptic Vesicle Release

> **Hypothesis**: Quantal synaptic transmission follows m = np. Miniature EPSP amplitude is ~0.5 mV = φ/τ mV. The quantal hypothesis involves φ = 2 key parameters (n, p).

## Grade: 🟧 APPROXIMATE

## Results

### The Correspondence

```
Quantal release (del Castillo & Katz, 1954):
  m = n · p

  m = mean quantal content (average vesicles released)
  n = number of release sites
  p = release probability

  Parameters: 2 = φ (n and p determine m)

Miniature EPSP (mEPSP):
  Amplitude: ~0.5 mV (neuromuscular junction)
  TECS-L: φ/τ = 2/4 = 0.5 mV  EXACT for canonical value
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8
```

### Structural Analysis

```
Synaptic transmission steps:
  1. Action potential arrives at terminal
  2. Ca²⁺ channels open
  3. Vesicle fusion (exocytosis)
  4. Neurotransmitter crosses cleft
  5. Postsynaptic receptor binding
  6. Ion channel opens → EPSP/IPSP
  Count = 6 = P₁ major steps

SNARE complex proteins (core):
  1. Synaptobrevin (v-SNARE)
  2. Syntaxin (t-SNARE)
  3. SNAP-25 (t-SNARE)
  Count = 3 = σ/τ

Release probability range:
  Low p synapses:  0.1 - 0.3
  High p synapses: 0.5 - 0.9
  Typical CNS:     ~0.25 = 1/τ to 0.5 = 1/φ

Vesicle pools:
  1. Readily releasable pool (RRP)
  2. Recycling pool
  3. Reserve pool
  Count = 3 = σ/τ
```

### Physical Context

The quantal hypothesis, established by Katz and colleagues at the neuromuscular junction, remains a cornerstone of synaptic physiology. The ~0.5 mV mEPSP is the textbook value at the NMJ. CNS synapses show smaller quantal sizes (~0.1-0.3 mV). The binomial model m=np is the simplest statistical description of vesicle release.

### Texas Sharpshooter Check

The 0.5 mV mEPSP is specific to the NMJ; CNS values differ. The φ/τ expression is simple. The P₁ = 6 synaptic steps are a standard pedagogical sequence. SNARE triplet = σ/τ is well-established biochemistry.

## Verification

- [x] φ = 2 parameters in quantal model (n, p)
- [x] mEPSP ~0.5 mV = φ/τ (NMJ canonical value)
- [x] 6 synaptic transmission steps = P₁
- [x] 3 SNARE proteins = σ/τ
- [x] 3 vesicle pools = σ/τ
