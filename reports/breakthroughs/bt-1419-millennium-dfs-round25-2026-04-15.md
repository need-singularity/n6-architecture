---
id: bt-1419-millennium-dfs-round25
date: 2026-04-15
parent_bt: BT-541~BT-547 (7 Clay Millennium)
roadmap_task: PX (DFS 25차)
grade: "[10] DFS round"
dfs_round: 25
dfs_area: "V·W·X·Y·Z (군분류/원분체/조합설계/특수함수/수론함수)"
new_tight: 12
cumulative_tight: 336
solved: "0/7 (정직)"
harness: theory/predictions/verify_millennium_dfs25.hexa
harness_result: "40 PASS / 0 FAIL / 0 MISS"
---

# DFS 25차 — 군 분류·원분체·Graeco-Latin·특수함수 (2026-04-15)

> **누적 tight**: 324 → **336** (+12 신규)
> **7대 난제 해결**: **0/7** (정직)
> **하네스**: 40 PASS / 0 FAIL / 0 MISS
> **Bernoulli 독립**: 12 → **14** (S_3 최소 비가환, Tarry 36 장교 추가)

---

## 신규 12건 tight

| # | ID | 발견 | 영역 | 등급 |
|---|-----|------|------|------|
| 25-01 | \|S_3\| = n = 6 | 최소 비가환 군, D_3 ≅ S_3 유일 합치 | 군론 | **T4 EXACT + Bernoulli** |
| 25-02 | \|Groups(6)\| = 2 = φ | Z_6, S_3 정확히 2개 | 군 분류 | **T1+T4 STRONG** |
| 25-03 | Φ_6(x) = x² - x + 1 | 최소 이차 원분 다항식, 판별식 −3 | 원분체 | T1-STRONG |
| 25-04 | Q(ζ_6) = Q(ζ_3) = Q(√−3) | degree φ = 2, disc = −3 | 대수적 수론 | T1-STRONG |
| 25-05 | \|U(Z[ζ_6])\| = n = 6 | Eisenstein 단위군 {±1,±ω,±ω²} | 대수 | T1+T4 |
| 25-06 | Graeco-Latin order 6 미존재 | Euler 1782 추측, Tarry 1900 증명 | 조합설계 | **T4 EXACT + Bernoulli** |
| 25-07 | Euler 36 장교 = n² 배치 | 6 계급 × 6 연대 동시 직교 불가 | 조합설계 | T4 EXACT |
| 25-08 | S_3 Cayley table = n×n 라틴사각 | 군 곱 표 구조 | 군·조합 | T2 |
| 25-09 | Gauss 곱셈 공식 n Γ 인수 | 지수 (n−1)/2 = sopfr/2 | 특수함수 | T1 |
| 25-10 | E_6 Eisenstein 무게 = n | modular 형식 무게, 계수 504 = 2³σ | 모듈러 형식 | T1-STRONG |
| 25-11 | μ(6) = 1, ω(6) = φ | Möbius 함수 + 소인수 개수 | 수론 | T1 |
| 25-12 | λ(6) = φ = 2 | Carmichael = Euler → 순환 | 수론 | T1-STRONG |

---

## Bernoulli 독립 목록 (DFS 1~25)

DFS 24 12건 + **S_3 (25-01), Tarry Graeco-Latin (25-06)** → **14건**

| # | 정리 | 독립 근거 |
|---|------|----------|
| 1 | Out(S_6) ≠ 1 | Hölder 1895 |
| 2 | K_6 Steiner 삼중계 | Cayley 1850 |
| 3 | 완전수 6 | Euclid-Euler |
| 4 | SO(6) ≅ SU(4)/Z_2 | Cartan |
| 5 | Heawood 토러스 χ=7 | Ringel-Youngs |
| 6 | Schaefer k=6 | Schaefer 1978 |
| 7 | Θ_6 = 1 | Kervaire-Milnor 1963 |
| 8 | M_12 5-transitive | CFSG |
| 9 | Pell D=6 (5,2) | Euler-Lagrange |
| 10 | PG(2,6) 미존재 | Bruck-Ryser 1949 |
| 11 | PSL(2,2) = 6 | Jordan 1870 |
| 12 | R(3,3) = 6 | Greenwood-Gleason 1955 |
| **13** | **\|S_3\| = 6 최소 비가환** | **Galois 1832** |
| **14** | **Graeco-Latin n=6 미존재** | **Tarry 1900** |

---

## 창발 분석 재검증

σφ=nτ 단일원인이 DFS 25 에도 투영:

- **\|Groups(6)\| = 2 = φ(6)**: 군 분류 투영
- **Graeco-Latin 미존재 = {φ, n} = {2, 6}**: 조합설계 투영
- **Z[ζ_6] 단위 × 원분체 degree = n·φ = 12 = σ**: 수론 투영

σ = n·φ 관계가 Eisenstein-원분체 경로에서 직접 나타남. 이는 **σ(6)·φ(6) = n·τ(6) = 24** 의 또 다른 대수적 재현.

---

## 명시적 부인

tight 336건은 n=6 산술 시그니처의 수학 내 **구조 관찰**이다.

- 밀레니엄 7대 난제 증명 **0/7**
- hexa 하네스 40 PASS 는 **산술 일치** 검증
- 창발 단일원인 (σφ=nτ) 은 **패턴 설명** 이지 **증명 경로** 아님
