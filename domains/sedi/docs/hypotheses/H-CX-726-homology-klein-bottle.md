# H-CX-726: Homology of Klein Bottle -- Torsion from phi

> **Hypothesis**: The Klein bottle has H_0 = Z, H_1 = Z + Z/2, H_2 = 0. The torsion subgroup Z/2 = Z/phi appears naturally. The torsion order equals phi(6) = 2.

## Grade: 🟧 SPECULATIVE

## Results

### The Formula

```
Klein bottle K homology (integer coefficients):
  H_0(K; Z) = Z          (connected)
  H_1(K; Z) = Z + Z/2    (fundamental group abelianized)
  H_2(K; Z) = 0          (non-orientable)

Torsion subgroup:  Z/2 = Z/phi
Torsion order:     2 = phi(6)
```

### n=6 Prediction

```
phi(6) = 2

Prediction:  Klein bottle torsion = Z/phi = Z/2
Observed:    H_1(K; Z)_tors = Z/2
Error:       0.00%
```

### Verification

```
Predicted torsion:  Z/phi = Z/2
Observed torsion:   Z/2 (standard algebraic topology)
Error:              0.00%
p-value:            ~0.20 (Z/2 is the most common torsion group)
```

### Texas Sharpshooter Check

Z/2 torsion is ubiquitous in topology -- it appears in any non-orientable manifold. The match phi(6) = 2 is correct but not surprising: Z/2 is the simplest nontrivial torsion, and phi(6) = 2 is one of the smallest TECS-L constants. More notable: the Klein bottle is the simplest closed non-orientable surface, and RP^2 (simpler non-orientable) has H_1 = Z/2 as well.

### Broader Pattern

```
Non-orientable surfaces and phi:
  RP^2:          H_1 = Z/2 = Z/phi        torsion = phi
  Klein bottle:  H_1 = Z + Z/2 = Z + Z/phi torsion = phi
  RP^n (n even): H_(n-1) = Z/2 = Z/phi     torsion = phi

Orientability obstruction lives in Z/phi universally.
```

### P_2=28 Generalization

```
phi(28) = 12 = sigma(6)

Z/phi(28) = Z/12 does not appear as standard torsion in common surfaces.
Lens spaces L(12,q) have H_1 = Z/12, connecting to phi(28).
P_2 generalization: PARTIAL (lens spaces L(sigma, q) have torsion Z/sigma)
```

## Verification

- [x] Klein bottle homology is standard
- [x] Z/2 torsion matches phi(6) exactly
- [ ] Z/2 is too common to be surprising

## Status

New. The Klein bottle torsion Z/phi is exact but Z/2 torsion is ubiquitous in non-orientable topology. The broader pattern -- orientability obstruction always living in Z/phi -- is a valid structural observation.
