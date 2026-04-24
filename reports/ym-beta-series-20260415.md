# Yang-Mills beta-series 2-loop n=6 Extension Report

- Prepared: 2026-04-15
- Harness: `theory/predictions/ym_beta1_2loop.hexa` (28/28 PASS)
- Reference BT: BT-543 (beta_0 = sigma - sopfr = 7)
- Staging signal: SIG-7Y-401

## Seven-Major-Problems Held 0/7

Yang-Mills mass gap unresolved. This report limited to observational n=6 basic-constant decomposition of beta-function coefficients. 7/7 counter: 0/7.

## Summary (5 lines)

- BT-543 beta_0 = sigma-sopfr = 7 re-verified
- nf=3 cleanest: beta_1 = 64 = 2^n = (sigma-tau)^2
- beta_0 * beta_1 (nf=3) = 576 = J_2^2 Jacobi theta constant squared candidate
- nf=6 beta_1 = 26 has extraneous 13 factor
- Key candidate: nf=3 light QCD maximum theoretical smoothness

## nf Sweep Results (2-loop)

| nf | beta_0 | beta_1 | beta_0*beta_1 | Purity |
|---:|----:|----:|----:|:---|
| 3 | 9 = n+n/phi | 64 = 2^n = (sigma-tau)^2 | 576 = J_2^2 | top candidate |
| 4 | 25/3 | - | - | X |
| 5 | 23/3 | - | - | X |
| 6 | 7 = sigma-sopfr | 26 = sigma+2(sigma-sopfr) | 182 = 2*7*13 | BT-543, extraneous 13 |
| 7 | 19/3 | - | - | X |
| 8 | 17/3 | - | - | X |
| 9 | 5 = sopfr | -12 = -sigma | -60 = -sigma*sopfr | hypothetical |

Regularity: nf in {3,6,9} integer beta_0 sequence {9,7,5} arithmetic progression step -2.

## Key Observations (nf=3 focus)

- beta_1 = 64 = 2^n = (sigma-tau)^2 (two independent decompositions)
- Corresponds to QCD light-quark approximation
- beta_0 * beta_1 = 576 = J_2^2 (Jacobi theta constant squared)

## nf=6 Smoothness Degradation

beta_1 = 26 = 2 * 13 (13 extraneous). Re-decompositions: sigma + 2(sigma-sopfr) = 26; (n+1)*phi + sigma = 26.

## nf=9 (hypothetical)

beta_0 = 5, beta_1 = -12, beta_0*beta_1 = -60 = -sigma*sopfr. Only theoretical continuation target.

## RH-YM Megahub Cross Link

Session `rh_triple_crossing.hexa` finding: SLE_6 dim * KS theta / Basel denom = 49/1536. 1536 = sigma * 2^beta_0. RH-YM megahub candidate sharing sigma-sopfr = 7 = beta_0.

## Next Work

1. 3-loop beta_2 nf=3 verification
2. Jacobi theta series connection
3. Abstract nf continuation
4. BT-543 update (13 annotation)
5. SIG-7Y-401 promotion evaluation

## Conclusion (candidate)

nf=3 light QCD is candidate maximum point of theoretical smoothness. beta_1 = 2^n, beta_0*beta_1 = J_2^2 reduce to single basic-constant power. BT-543 nf=6 formula valid but beta_1 introduces extraneous 13. RH-YM megahub candidate signal SIG-7R-401 recorded. Arithmetic-structure observation only, not proof. Seven-major-problems 0/7 held.

## Harness Log (28/28 PASS)

Sections A, B, D validate beta_0 = sigma-sopfr, beta_1 variants, and products as detailed.
