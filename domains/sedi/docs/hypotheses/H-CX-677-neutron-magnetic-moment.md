# H-CX-677: Neutron Magnetic Moment μ_n ≈ −sopfr·τ/(σ−φ) = −2.0

> **Hypothesis**: The neutron magnetic moment μ_n = −1.9130 μ_N ≈ −sopfr·τ/(σ−φ) = −20/10 = −2.0 (4.5% error). A harder target than the proton.

## Grade: 🟧 (4.5% — best available n=6 route)

## Results

### The Prediction

```
μ_n = −1.91304273(45) μ_N   (2018 CODATA)

−sopfr·τ/(σ−φ) = −5·4/(12−2) = −20/10 = −2.0

Error: |−2.0 − (−1.913)|/1.913 = 4.5%
```

### Alternative Routes

```
−P₁/σ·(σ/τ+1/(σ·sopfr)) = −6/12·(3+1/60)
  = −0.5·3.0167 = −1.508  (21% — poor)

−σ/(P₁+sopfr/(σ−τ)−1/σ) = −12/(6+0.625−0.083)
  = −12/6.542 = −1.834  (4.1%)

−σ·sopfr/(σ²/τ+M₃−φ) = −60/(36+7−2)
  = −60/41 = −1.463  (24% — poor)
```

### Best Route with Correction

```
−(sopfr·τ−1)/(σ−φ) = −19/10 = −1.9

Error: |−1.9 − (−1.913)|/1.913 = 0.68%  ✓✓

But (sopfr·τ−1) = 19 is less "clean" arithmetically.
```

### Proton-Neutron Ratio

```
μ_p/μ_n = 2.7928/(−1.9130) = −1.4598

−(σ/τ)/(φ+1/σ) = −3/2.083 = −1.440  (1.4%)
−σ/τ·(σ−φ)/(σ·sopfr/τ·(τ−φ)) = ... (convoluted)

Simple: −σ/(σ−τ) = −12/8 = −1.5  (2.8%)
```

### Physical Context

The neutron, being electrically neutral, has a magnetic moment entirely due to its quark substructure (udd). The non-trivial value −1.913 μ_N was one of the first hints that neutrons are composite.

## Verification

- [x] −sopfr·τ/(σ−φ) = −2.0 at 4.5%
- [x] −19/10 = −1.9 at 0.68% (with −1 correction)
- [x] Ratio μ_p/μ_n ≈ −σ/(σ−τ) = −1.5 at 2.8%
- [ ] No clean single-expression route below 1% without ad hoc corrections

## Status

Harder target than the proton moment. Best clean route at 4.5%; corrected route at 0.68%. The neutron moment may require quark-level n=6 structure.
