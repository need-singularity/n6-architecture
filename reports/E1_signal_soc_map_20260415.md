# E1 Signal SOC Map -- 2026-04-15

> Input: `/Users/ghost/Dev/nexus/shared/n6/atlas.signals.n6`
> Total signals: 385 | Analysis: log-log slope (Zipf-like) | SOC judgement qualitative
> Seven-major-problems resolution held at 0/7

## 1. Domain Tag Distribution

- Unique domain tags: 40 | log-log slope: -1.3177 | fit N: 34 | top 5 share: 52.98%
- SOC signal: POWER-LAW

Top 15: META 222, PHYS 74, CONS 59, UNIV 55, ATLAS 35, NEURAL 35, NULL 34, 7Y 34, 7H 33, 7R 31, 7B 25, 7P 21, DFS 21, HEXA 20, GAP 18.

## 2. ID Prefix Distribution (domain family)

- Unique prefixes: 39 | log-log slope: -0.9168 | fit N: 35 | top 5 share: 40%
- SOC signal: POWER-LAW

Top: SIG-META-* 55, SIG-7Y-* 27, SIG-MEGA-* 25, SIG-ATLAS-* 24, SIG-PHYS-* 23.

## 3. Grade Distribution

M10 138 (35.84%), M7 58 (15.06%), M9 57 (14.81%), M7! 50 (12.99%), MN 28 (7.27%), M? 27 (7.01%), M10* 20 (5.19%), M5 6 (1.56%), M8 1 (0.26%).

## 4. Repo Distribution

N6 231, AN 86, NX 85, CROSS 56.

## 5. SOC Interpretation

Slope in [-2.5, -0.5] indicates (Zipf-near) critical distribution candidate. Both domain tag and ID prefix are POWER-LAW.

## 6. Honest Limits

R-squared of log-log fit not computed, p-value not calculated. SOC judgement qualitative via slope range only. KS-test not performed. Self-citation in atlas.signals.n6 not corrected. Seven major problems 0/7.
