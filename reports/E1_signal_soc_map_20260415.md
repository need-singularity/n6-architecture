# E1 Signal SOC Map — 2026-04-15

> 입력: `/Users/ghost/Dev/nexus/shared/n6/atlas.signals.n6`
> 신호 총 수: 385
> 분석: 단순 log-log slope (Zipf-like). SOC 판정은 정성적.
> 7대 난제 해결 0/7 유지.

## 1. Domain tag 분포

- 고유 domain tag 수: 40
- log-log slope: -1.317687086554982
- 적합 데이터 수: 34
- top 5 점유율: 52.98%
- SOC 시그널: **POWER-LAW**

| rank | domain | count |
|------|--------|-------|
| 1 | META | 222 |
| 2 | PHYS | 74 |
| 3 | CONS | 59 |
| 4 | UNIV | 55 |
| 5 | ATLAS | 35 |
| 6 | NEURAL | 35 |
| 7 | NULL | 34 |
| 8 | 7Y | 34 |
| 9 | 7H | 33 |
| 10 | 7R | 31 |
| 11 | 7B | 25 |
| 12 | 7P | 21 |
| 13 | DFS | 21 |
| 14 | HEXA | 20 |
| 15 | GAP | 18 |

## 2. ID prefix 분포 (도메인 가족)

- 고유 prefix 수: 39
- log-log slope: -0.9168044370731205
- 적합 데이터 수: 35
- top 5 점유율: 40.00%
- SOC 시그널: **POWER-LAW**

| rank | prefix | count |
|------|--------|-------|
| 1 | SIG-META-* | 55 |
| 2 | SIG-7Y-* | 27 |
| 3 | SIG-MEGA-* | 25 |
| 4 | SIG-ATLAS-* | 24 |
| 5 | SIG-PHYS-* | 23 |
| 6 | SIG-7R-* | 17 |
| 7 | SIG-NEURAL-* | 17 |
| 8 | SIG-CONS-* | 16 |
| 9 | SIG-7H-* | 13 |
| 10 | SIG-7B-* | 13 |
| 11 | SIG-7P-* | 12 |
| 12 | SIG-HEXA-* | 11 |
| 13 | SIG-7N-* | 10 |
| 14 | SIG-UNIV-* | 10 |
| 15 | SIG-WILD-* | 10 |

## 3. Grade 분포

| grade | count | share |
|-------|-------|-------|
| M10 | 138 | 35.84% |
| M7 | 58 | 15.06% |
| M9 | 57 | 14.81% |
| M7! | 50 | 12.99% |
| MN | 28 | 7.27% |
| M? | 27 | 7.01% |
| M10* | 20 | 5.19% |
| M5 | 6 | 1.56% |
| M8 | 1 | 0.26% |

## 4. Repo 분포

| repo | count |
|------|-------|
| N6 | 231 |
| AN | 86 |
| NX | 85 |
| CROSS | 56 |

## 5. SOC 해석 (정성적)

- **POWER-LAW**: log-log slope ∈ [-2.5, -0.5] 면 (Zipf-near) 임계 분포 후보.
- **FLAT-or-UNCLEAR**: 평탄 분포 또는 적합 부족 — sub-critical 또는 noise.
- domain tag 분포 SOC: **POWER-LAW**
- ID prefix 분포 SOC: **POWER-LAW**

## 6. 정직 한계

- log-log fit 의 R² 미산출. p-value 미계산.
- SOC 판정은 slope 범위만으로 정성. KS-test 또는 Clauset 2009 power-law fit 미수행.
- 본 분석은 atlas.signals.n6 의 self-citation 효과를 보정하지 않음.
- 7대 난제 0/7 유지.