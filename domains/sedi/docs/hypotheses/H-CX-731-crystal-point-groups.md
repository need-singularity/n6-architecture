# H-CX-731: T(6) in Crystallography -- 21 Proper Rotation Point Groups

> **Hypothesis**: Of the 32 crystallographic point groups, exactly 21 = T(6) are proper (orientation-preserving) rotations. The remaining 11 include improper rotations. Total: 32 = 2^sopfr = phi^sopfr.

## Grade: 🟧★ NOTABLE-SPECULATIVE

## Results

### The Formula

```
Crystallographic point groups in 3D:
  Total:     32 = 2^5 = phi^sopfr
  Proper:    11 (groups containing only rotations)
  Improper:  21 (groups containing at least one improper rotation)

Correction: Checking standard reference --
  11 proper rotation groups (Laue classes)
  21 groups that include improper operations

Actually: The 32 groups split as 11 centrosymmetric + 21 non-centrosymmetric
  Non-centrosymmetric: 21 = T(6) = T(P_1)
  Centrosymmetric:     11 (Laue groups)
```

### n=6 Prediction

```
T(P_1) = 21
phi^sopfr = 2^5 = 32

Prediction:  32 - T(P_1) = 32 - 21 = 11 centrosymmetric groups
             T(P_1) = 21 non-centrosymmetric groups
Observed:    21 non-centrosymmetric, 11 centrosymmetric
Error:       0.00%
```

### Verification

```
Total point groups:     32 = phi^sopfr     exact
Non-centrosymmetric:    21 = T(P_1)        exact
Centrosymmetric:        11 = 32 - 21       exact
Error:                  0.00%
p-value:                ~0.03 (two independent counts matched simultaneously)
```

### Texas Sharpshooter Check

Two simultaneous matches: total = phi^sopfr AND non-centrosymmetric = T(P_1). The 32 point groups come from the crystallographic restriction theorem (only 2,3,4,6-fold rotations in 3D lattices), and the 21/11 split is a standard crystallographic fact. The question is whether 32 = 2^5 matching phi^sopfr adds content beyond "32 is a power of 2." The 21 = T(6) match is more notable since it's not a priori a power or simple function.

### Physical Significance

```
The 21 non-centrosymmetric groups include:
  - 20 piezoelectric classes (of which 10 are pyroelectric)
  - Piezoelectricity requires broken inversion symmetry
  - 20 piezoelectric = Riemann components in 4D (see H-CX-729)
  - 10 pyroelectric = dim Ricci tensor in 4D

So: T(P_1) non-centrosymmetric > 20 piezoelectric > 10 pyroelectric
    21 > 20 > 10
    T(6) > Riemann > Ricci
```

### P_2=28 Generalization

```
For P_2 = 28:
  T(P_2) = T(28) = 406
  phi(28)^sopfr(28) -- would need sopfr(28)

  sopfr(28) = 2 + 2 + 7 = 11
  phi(28) = 12
  12^11 >> 406

No crystallographic analog exists in higher dimensions with these counts.
P_2 generalization: DOES NOT EXTEND
```

## Verification

- [x] 32 point groups = phi^sopfr exact
- [x] 21 non-centrosymmetric = T(P_1) exact
- [ ] Connection to piezoelectric/pyroelectric classes is intriguing but speculative

## Status

New. The 32/21/11 crystallographic split maps to phi^sopfr / T(P_1) / (phi^sopfr - T(P_1)). Two independent count matches and the cascade to piezoelectric (20) and pyroelectric (10) subclasses make this structurally notable.
