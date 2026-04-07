# H-CX-TC-Survey: Systematic Superconductor Tc Matching with n=6 Arithmetic

> **Hypothesis**: Critical temperatures (Tc) of well-known superconductors can be expressed as simple arithmetic combinations of n=6 invariants: sigma(6)=12, tau(6)=4, phi(6)=2, sopfr(6)=5, n=6. A systematic survey of 17 materials finds 8 exact matches and 12 within 1%, but Texas Sharpshooter analysis shows this does NOT exceed random expectation (p=0.99).

## Grade: 🟧 MIXED (8 exact matches, but Texas p > 0.05 -- not statistically significant)

## Methodology

### n=6 Invariants Used

```
  sigma(6) = 12    sum of divisors of 6
  tau(6)   = 4     number of divisors of 6
  phi(6)   = 2     Euler totient of 6
  sopfr(6) = 5     sum of prime factors with repetition (2+3)
  n        = 6     the number itself
```

### Expression Space

```
  Binary:  a op b               (5 vars x 5 vars x 5 ops = 125)
  Ternary: (a op b) op2 c       (5x5x5 x 5x5 = 3125, minus trivials)
           a op (b op2 c)       (same)
  Special: known from H-CX-657/658 hypotheses

  Total generated:       6,137 expressions
  Valid (0 < val < 1000): 3,930
  Unique values:          465
```

### Uniqueness Check

Each matching formula is tested with n=28 invariants (sigma=56, tau=6, phi=12, sopfr=11). If the formula also produces the same Tc for n=28, it is NOT n=6 specific.

## Results

### Full Survey Table

| Material | Tc (K) | Best n=6 Expression | Predicted | Error | n=6 Specific? |
|---|---|---|---|---|---|
| Al | 1.18 | (no match <1%) | -- | -- | -- |
| In | 3.41 | (sigma+sopfr)/sopfr | 3.400 | 0.293% | Yes |
| Sn | 3.72 | sopfr-(sopfr/tau) | 3.750 | 0.806% | Yes |
| Hg | 4.15 | (phi/sigma)+tau | 4.167 | 0.402% | Yes |
| Pb | 7.19 | (n/sopfr)*n | 7.200 | 0.139% | Yes |
| Nb | 9.25 | (no match <1%) | -- | -- | -- |
| NbTi | 10.0 | sigma-phi | 10.000 | EXACT | Yes |
| NbN | 16.0 | sigma+tau | 16.000 | EXACT | Yes |
| Nb3Sn | 18.3 | (no match <1%) | -- | -- | -- |
| MgB2 | 39.0 | sigma*(sigma+1)/tau | 39.000 | EXACT | Yes |
| YBCO | 93.0 | sigma*(sigma-tau)-sigma/tau | 93.000 | EXACT | Yes |
| BSCCO-2212 | 95.0 | (no match <1%) | -- | -- | -- |
| BSCCO-2223 | 110.0 | (no match <1%) | -- | -- | -- |
| Tl-2223 | 125.0 | (sopfr^tau)/sopfr | 125.000 | EXACT | Yes |
| Hg-1223 | 133.0 | sigma^2-sigma+1 | 133.000 | EXACT | Yes |
| H3S | 203.0 | sigma^2+sigma*sopfr-1 | 203.000 | EXACT | Yes |
| LaH10 | 250.0 | (tau^tau)-n | 250.000 | EXACT | Yes |

### Summary Statistics

```
  Total materials:     17
  Matches within 1%:   12 (70.6%)
  Exact (<0.1%):       8  (47.1%)
  No match:            5  (Al, Nb, Nb3Sn, BSCCO-2212, BSCCO-2223)

  n=6 specific:        12/12 = 100%
  Also works for n=28: 0/12 = 0%
```

### Exact Match Detail

```
  NbTi:    10 = sigma - phi        = 12 - 2
  NbN:     16 = sigma + tau        = 12 + 4
  MgB2:    39 = sigma*(sigma+1)/tau = 12*13/4
  YBCO:    93 = sigma*(sigma-tau) - sigma/tau = 12*8 - 3
  Tl-2223: 125 = sopfr^(tau-1)    = 5^3
  Hg-1223: 133 = sigma^2-sigma+1  = 144-12+1
  H3S:     203 = sigma^2+sigma*sopfr-1 = 144+60-1
  LaH10:   250 = tau^tau - n       = 256 - 6
```

### ASCII Histogram: Error Distribution

```
  Error %  Count
  EXACT    |######## 8
  <0.1%    |
  0.1-0.5% |### 3
  0.5-1.0% |# 1
  >1%/none |##### 5
           +------------------
            0  2  4  6  8  10
```

### Tc vs Predicted Value (Exact Matches Only)

```
  Tc (K)
  250 |                              * LaH10
      |
  200 |                         * H3S
      |
  150 |
  133 |                   * Hg-1223
  125 |                  * Tl-2223
  100 |
   93 |             * YBCO
      |
   50 |
   39 |         * MgB2
      |
   16 |   * NbN
   10 |  * NbTi
      +-----+-----+-----+-----+-----+
        10   39   93  133  203  250
                  Predicted (K)

  All points on diagonal (exact match by definition)
```

## Texas Sharpshooter Analysis

### Monte Carlo Setup

```
  Formula pool:    465 unique values (from 3930 valid expressions)
  Target count:    17 superconductor Tc values
  Threshold:       1% relative error
  Trials:          100,000

  Method: Generate 17 random targets in [1, 260] K range,
          465 random formula values in same range.
          Count how many targets have at least one <1% match.
```

### Results

```
  Expected random matches:  15.24 +/- 1.26
  Observed matches:         12
  Z-score:                  -2.6 (BELOW expectation!)
  p-value:                  0.994

  VERDICT: NOT SIGNIFICANT (p >> 0.05)
```

### Random Match Distribution

```
  Matches  Probability
     8     0.0%
     9     0.0%
    10     0.1%  |
    11     0.5%  |
    12     2.0%  |##
    13     6.6%  |######
    14    16.1%  |################
    15    28.4%  |############################
    16    30.3%  |##############################
    17    15.9%  |################
```

### Interpretation

```
  With 465 formula values and 1% threshold:
    Each target has ~465 chances to be hit
    Expected coverage: 1 - (1 - 0.02)^465 ~ 1.0 (nearly certain)

  The fact that we MISS 5/17 targets is actually WORSE than random!

  WHY: n=6 arithmetic produces values clustered at integers and
  simple fractions. The 465 values are NOT uniformly spread over [1, 260].
  Many cluster near small integers (1-20) and near sigma^2 = 144.
  Gaps exist around 9.25 (Nb), 18.3 (Nb3Sn), 95 (BSCCO-2212).

  The 8 exact matches are impressive individually but EXPECTED
  given the large formula space. The 5 misses are the real signal:
  certain Tc values RESIST n=6 arithmetic.
```

## What the Misses Tell Us

```
  Material       Tc (K)   Nearest n=6 value   Gap
  Al             1.18     1.200 (n/sopfr)      1.7%
  Nb             9.25     9.000 (various)      2.7%
  Nb3Sn          18.3     18.000 (sigma*phi-n)  1.6%
  BSCCO-2212     95.0     96.000 (sigma^2-...) 1.1%
  BSCCO-2223    110.0     108.000 (various)    1.8%

  These are all JUST outside 1% threshold.
  Al and Nb3Sn have non-integer Tc from strong-coupling corrections.
  BSCCO compounds have complex multi-layer cuprate structures.
```

## n=6 Specificity

All 12 matching formulas produce wildly different values for n=28:

```
  Formula          n=6 value   n=28 value   Ratio
  sigma-phi        10          44           4.4x
  sigma+tau        16          62           3.9x
  sigma*(s+1)/tau  39          532          13.6x
  sigma*(s-t)-s/t  93          2791         30.0x
  sopfr^(tau-1)    125         161051       1288x
  sigma^2-sigma+1  133         3081         23.2x
  tau^tau-n         250         46650        186.6x
```

This confirms the formulas are structurally tied to n=6, not generic number theory.

## Connection to Existing Hypotheses

- H-CX-646: BCS gap ratio 60/17 -- harmonic mean of sigma and sopfr
- H-CX-657: MgB2 Tc = 39 = sigma*(sigma+1)/tau (confirmed here)
- H-CX-658: YBCO Tc = 93 = sigma*(sigma-tau)-sigma/tau (confirmed here)
- H-CX-659: Superfluid He lambda point 24/11 ~ 2.18 K

## Limitations

1. Texas Sharpshooter p = 0.994 means the overall matching rate is BELOW random expectation
2. The 8 exact matches are individually interesting but collectively expected from 465 formulas
3. Expression space is biased toward integer outputs (most Tc values are near-integer)
4. No mechanism proposed for WHY n=6 arithmetic should govern Tc
5. Post-hoc formula selection: we pick the BEST match for each, which inflates apparent accuracy

## Status

- [x] Systematic survey of 17 superconductors completed
- [x] 8 exact matches, 12 within 1%
- [x] Texas Sharpshooter: NOT significant (p = 0.994)
- [x] n=6 specificity: 100% (no formula also works for n=28)
- [x] Miss analysis: 5 materials resist n=6 arithmetic
- [ ] Investigate if misses correlate with strong-coupling or multi-band nature
- [ ] Test restricted formula space (binary only) to reduce look-elsewhere effect
- [ ] Pressure-dependent Tc survey (H3S, LaH10 at various pressures)
