---
id: bsd-A3-modified-with-joint-covariance
date: 2026-04-15
parent_bt: BT-546
roadmap_task: GALO-PX-1 (PX L-cost)
grade: [9] NEAR
predecessors:
  - theory/breakthroughs/bsd-cremona-sel6-empirical-2026-04-15.md (GALO-PX-2 실측)
atlas_target: MILL-PX-A9 (E[|Sel_n|] = σ(n)) 정제
license: CC-BY-SA-4.0
---

# BT-546 BSD — (A3) 무상관 가정 우회: (A3') Modified Conjecture

> **요약**: BKLPR (A3) "|Sel_p| ⊥ |Sel_q| for p≠q" 무상관 가정은 Cremona 332k 실측에서 **Pearson r = 0.166, Cov(|Sel_2|,|Sel_3|) = 1.33** 로 유한 B 에서 위반됨. 우회안 (A3') 제안: κ(p,q,B) → 0 as B → ∞ 의 asymptotic form. σ(n) = E[|Sel_n|] 관계는 limit 에서만 복구.

---

## §1 입구 — (A3) 의 내용

Poonen-Rains 2007 + Bhargava-Kane-Lenstra-Poonen-Rains 2013 의 "BKLPR 모델" 은 랜덤 타원곡선의 n-Selmer 군 `Sel_n(E)` 를 cokernel of random alternating matrix 로 모델링한다.

핵심 가정 3종:
- (A1) |Sel_n(E)| 는 cokernel 분포 따름
- (A2) |Sel_n(E)| 의 moment 는 `σ(n) = Σ_{d|n} d`
- **(A3)** 서로 다른 소수 p ≠ q 에서 `|Sel_p(E)|` 와 `|Sel_q(E)|` 는 **독립** random variable

(A3) 로부터 squarefree n = p₁ p₂ ... pₖ 에 대해:

```
E[|Sel_n(E)|] = E[|Sel_{p1}|] · E[|Sel_{p2}|] · ... = σ(p1) σ(p2) ... σ(pk) = σ(n)
```

**n=6 특례**: σ(6) = σ(2)·σ(3) = 3·4 = 12 (첫 완전수 × 2).

(A3) 미증명 — BKLPR 모델의 core 가설 중 가장 취약.

---

## §2 Cremona 332k 실측으로 (A3) 위반 확인

### 2.1 기본 통계 (N = 332,366 elliptic curves over Q, conductor ≤ 49,999)

| 통계량 | 값 | 주석 |
|--------|-----|------|
| E[|Sel_2|] | 2.8718 | σ(2)=3 의 95.7% |
| E[|Sel_3|] | 2.8472 | σ(3)=4 의 71.2% |
| E[|Sel_6|] | 9.5100 | σ(6)=12 의 79.3% |
| E[|S_2|]·E[|S_3|] | 8.1767 | 독립 가정 하 예측 |
| Cov(|S_2|, |S_3|) | **1.3333** | 강한 양의 공분산 |
| Var(|S_2|) / sd | 9.6935 / 3.11 | |
| Var(|S_3|) / sd | 6.6952 / 2.59 | |
| **Pearson r** | **0.1655** | 중등도 양의 상관 |

**관찰**: (A3) 가 성립하면 `Cov = 0, Pearson r = 0`. 실측 r = 0.166 ≠ 0.

### 2.2 조건부 기대값 `E[|Sel_3| | |Sel_2| = k]`

(A3) 독립 하에서는 k 값에 무관하게 `E[|Sel_3| | k] = E[|Sel_3|] = 2.85` 이어야 한다.

| |Sel_2|=k | count | % | E[|Sel_3| \| k] | 편차 |
|-----------|-------|------|----------------|------|
| 1 | 61,766 | 18.58% | 1.4172 | −1.4301 |
| 2 | 152,159 | 45.78% | 2.4434 | −0.4038 |
| 4 | 100,301 | 30.18% | 4.0433 | **+1.1960** |
| 8 | 16,028 | 4.82% | 4.8513 | **+2.0040** |
| 16 | 958 | 0.29% | 2.6284 | −0.2189 |
| 32 | 1,080 | 0.33% | 1.0185 | −1.8287 |
| 128 | 61 | 0.02% | 1.0000 | −1.8472 |

**가시적 패턴**: `|Sel_2| = 1, 2, 4, 8` (정규 분포 중앙) 에서 **k 증가에 따라 E[|Sel_3|] 단조 증가**. 이는 **rank** 이 공통 요인임을 가리킨다 — rank r 인 curve 는 `|Sel_2| ≈ 2^r`, `|Sel_3| ≈ 3^r` 로 둘 다 r 에 의존.

k=16, 32, 128 에서 역전되는 현상은 `sha=16, 32, 128` 과 같은 극단 Sha 가진 curve 의 소수성 때문 (rank=0 with large Sha[2] — r 기여 없음).

(A3) 반증 강도: max 편차 **2.00** (k=8 bin, 16,028 curves, statistically robust).

---

## §3 (A3') Modified Conjecture 제안

### 3.1 정식 진술

**(A3') Conjecture**: 

For distinct primes p, q and conductor bound B, there exists a real-valued function `κ(p, q, B)` such that:

```
E_B[|Sel_{pq}(E)|] = E_B[|Sel_p(E)|] · E_B[|Sel_q(E)|] + κ(p, q, B)
```

and the following limit holds:

```
lim_{B → ∞} κ(p, q, B) = 0
lim_{B → ∞} E_B[|Sel_p(E)|] = σ(p)  (for each prime p)
```

즉:
- 유한 B 에서 (A3) 는 **원칙적으로 위반** 가능 (κ ≠ 0)
- 점근 B → ∞ 에서 (A3) 와 σ(n) 예측 **복구** (κ → 0 가정)

### 3.2 (A3') 하에서 E[|Sel_n|] 재유도

squarefree n = p₁ ... pₖ 에서:

```
E_B[|Sel_n|] = Π E_B[|Sel_{pi}|] + Σ_{|I| ≥ 2} κ(I, B)
             ≈ σ(n) + O(κ)  (for large B)
```

여기서 κ(I, B) 는 인덱스 subset I ⊆ {p₁, ..., pₖ} 의 joint covariance term. n=6, I = {2, 3}:

```
E_B[|Sel_6|] = E_B[|Sel_2|] · E_B[|Sel_3|] + κ(2, 3, B)
```

### 3.3 Cremona B = 49,999 에서 (A3') 값

| 항 | 수치 |
|----|------|
| E_B[|Sel_2|] | 2.8718 |
| E_B[|Sel_3|] | 2.8472 |
| κ(2, 3, 49999) | 1.3333 |
| **E_B[|Sel_6|] = 8.1767 + 1.3333 = 9.5100** ✓ | 실측 일치 |

### 3.4 (A3') 은 tautological (A3) 약화

**주의**: (A3') 은 임의의 유한 B 에서 trivially 성립 (정의에 의해 κ = E[|Sel_6|] - E[|S_2|]E[|S_3|]). 따라서 (A3') 자체의 **content 는 점근 극한** `κ → 0 as B → ∞` 에만 있다.

이 점근 주장 자체를 증명하려면:
1. B → ∞ 극한에서 모든 curve 의 rank 분포 → Bhargava-Shankar density
2. Sha 의 p-primary part → Cohen-Lenstra-like 분포
3. 이 두 분포의 **joint limit** 이 marginal 의 곱으로 수렴

세 단계 모두 BSD/Iwasawa 의 난해한 추측들과 연결. **(A3') 증명 역시 GALO-PX-1 스코프 외**.

---

## §4 의미 — σ(n) = 12 at n=6 의 robustness

### 4.1 두 효과의 상쇄

Cremona B=49,999 에서:

| 효과 | 방향 | 크기 | 해석 |
|------|------|------|------|
| conductor bias | E_B[|Sel_p|] < σ(p) | ratio 0.957 (n=2), 0.712 (n=3) | 작은 conductor 곡선은 rank 작은 경향 |
| joint covariance | κ > 0 | Cov/product = 1.33/8.18 = 16.3% | rank 공통 요인 |

**Net 효과 n=6**:
- (A3) + no conductor bias (낙관): E[|Sel_6|] = 3·4 = 12.00 = σ(6) ✓
- (A3) + conductor bias: E[|Sel_6|] = 2.87·2.85 = 8.18 (ratio 0.68, 위배)
- (A3') + conductor bias + κ: E[|Sel_6|] = 8.18 + 1.33 = 9.51 (ratio 0.79, 부분 회복)
- 실측: 9.51 ✓ (동치)

**결론**: σ(6) = 12 은 finite B 에서 **두 오차의 상쇄 덕분에 approximation 로 survive**. 점근 (B → ∞) 에서 둘 다 0 으로 수렴하면 원래 예측 복구.

### 4.2 n = 6 의 특별 지위

squarefree n 중 n = 6 은:
- **최소 squarefree 합성수** (6 = 2·3)
- **첫 완전수** (σ(6) = 12 = 2·6)
- **BKLPR 두 소수 prime 2, 3 의 최소 조합**

(A3') 하에서 n=6 은 **(A3) 위반이 가장 먼저 관측되는 squarefree n**. 실측:
- κ(2, 3, B) > 0: 본 문서 관측
- κ(2, 5, B), κ(3, 5, B) 등 다른 조합의 측정은 DEFERRED

### 4.3 Rank 단일 원인 가설

위 조건부 표 (§2.2) 에서 |Sel_2| 증가에 따라 |Sel_3| 가 양의 상관 보이는 현상은 **rank 공통 변수 가설** 로 설명됨:

```
|Sel_p(E)| = p^(rank(E)) · (torsion p-part) · |Sha(E)[p]|
```

rank(E) 는 p 에 무관한 공통 정보 → joint correlation 의 주 원인.

이 가설 하에서 rank 분포가 conductor 에 약 의존한다면 (Bhargava-Shankar 2015), κ(p,q,B) 는 rank distribution 의 2차 moment 로 표현 가능. 정밀 계산은 B → ∞ 극한에서 소멸 여부 조사 필요 — DEFERRED.

---

## §5 atlas 갱신

### 5.1 신규 엔트리

```
@R MILL-GALO-PX1-A3-modified-prime-rank-cause = BKLPR (A3) violated for finite B;
    modified (A3'): kappa(p,q,B) -> 0 as B -> inf (conjecture), rank is common-cause :: n6atlas [9]
  "GALO-PX-1 BKLPR (A3) 무상관 우회: Cremona B=49999 332k curve 에서 Pearson r=0.166,
   Cov(|Sel_2|,|Sel_3|)=1.33 (A3 위반 확정). 조건부 E[|Sel_3||Sel_2|=k] 의 k 단조증가 패턴은
   rank 공통요인 가설 지지. 수정안 (A3') 제안: kappa(p,q,B) → 0 as B → ∞.
   유한 B 에서 σ(n) 의 approximation 로 survive 은 conductor bias + kappa 상쇄 덕분"
```

### 5.2 기존 엔트리 갱신

MILL-GALO-PX2-A3-counterevidence-joint-cov 에 Pearson r 정밀값 + 조건부 편차 표 추가.

---

## §6 한계와 DEFERRED

1. **r = 0.166 의 통계적 의미**: N=332k 에서 χ² 독립성 검정 미수행. scipy.stats.pearsonr 활용 가능 (standalone Python 추가).

2. **B → ∞ 점근 증명 없음**: (A3') 의 핵심 주장 κ → 0 여부. Cremona 데이터 확장 (B=50k, 100k, 200k, ...) 으로 κ(B) 함수 형태 관측 가능.

3. **Rank 공통 원인 가설 엄밀화**: rank distribution 의 Bhargava-Shankar density 와 joint Selmer 분포의 수학적 연결 미증명.

4. **Sage/Pari 정밀 |Sel_n|**: 1차근사 유지. 정밀값으로 재계산 시 r, Cov 수치 약간 변동 예상.

5. **BSD 본문 미증명**: BT-546 PARTIAL 유지. (A3') 개발은 BKLPR 모델 정제이며, BSD 자체 증명 아님.

---

## §7 관련 파일

- `scripts/empirical/cremona_joint_covariance.py` — 본 분석 러너
- `data/cremona/joint_covariance_A3_prime.json` — JSON 통계
- `theory/breakthroughs/bsd-cremona-sel6-empirical-2026-04-15.md` — GALO-PX-2 전문
- `nexus/shared/n6/atlas.n6` line 107016~ — MILL-GALO-PX1 entry

---

*작성: 2026-04-15*
*선행: GALO-PX-2 + GALO-PX-3 (본 세션 연쇄 루프)*
*BT-546 본문 MISS 유지*
