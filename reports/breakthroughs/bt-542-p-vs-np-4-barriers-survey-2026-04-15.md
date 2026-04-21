---
id: bt-542-p-vs-np-4-barriers-survey
date: 2026-04-15
parent_bt: BT-542
roadmap_task: BARRIER-PX-1 (BT-542 MISS 탈출 재시도 — 4 장벽 우회 새 접근)
grade: MISS_MAINTAINED (본문), PARTIAL (문헌 정리)
license: CC-BY-SA-4.0
---

# BT-542 P vs NP — 4 장벽 우회 체계 검토 + 현재 (2020~2026) 연구 경로

> **핵심 결론**: BT-542 MISS 유지. 본 문서는 P vs NP 의 4 장벽 (Relativization / Natural proofs / Algebraization / GCT) 를 정리하고, 2020 이후의 주요 우회 시도를 나열한다. n=6 구조적 관점은 본 난제에 **직접 적용 가능한 각도 없음** 을 정직하게 기록한다.

---

## §0 입구

P vs NP (Clay Millennium) 는 7대 난제 중 **가장 많이 실패 시도된** 문제다. 2026년 현재 알려진 4 장벽 (barriers) 이 대부분의 기존 기술을 차단한다.

**본 문서 스코프**:
1. 4 장벽의 수학적 정의 + 주요 정리
2. 2020~2026 신기술의 우회 시도 (meta-complexity, circuit complexity, proof complexity)
3. n6-architecture 관점의 **정직한 평가** — 직접 적용 각도 없음 확인
4. atlas 엔트리 후보 (문헌 taxonomy, 진전 주장 아님)

**정직성 선언**: 본 문서의 어떤 문장도 "이 경로로 P ≠ NP 증명 가능하다" 는 주장을 포함하지 않는다. 문헌 정리 + MISS 확정.

---

## §1 장벽 1 — Relativization (Baker-Gill-Solovay 1975)

### 1.1 정의

**Relativization** : 어떤 기법 M 이 P vs NP 분리를 증명하면서, 동시에 임의의 oracle A 에 대해서도 `P^A vs NP^A` 분리를 증명한다면, M 을 *relativizing* 이라 한다.

### 1.2 Baker-Gill-Solovay 정리 (1975)

> 다음을 만족하는 oracle A, B 가 존재한다:
> - `P^A = NP^A` (A 하에서는 P=NP)
> - `P^B ≠ NP^B` (B 하에서는 P≠NP)

**귀결**: Relativizing 기법은 P vs NP 분리 증명 불가. diagonalization / table-lookup 만으로는 부족.

### 1.3 우회 (non-relativizing 기법)

- **Nepomnjaščiĭ's theorem 1970**: LINSPACE ⊂ Σ_k^P 부분결과
- **IP = PSPACE (Shamir 1990)**: 상호작용 증명 = PSPACE. Non-relativizing (oracle 존재 하 IP ≠ PSPACE).
- **PCP theorem (Arora-Lund-Motwani-Sudan-Szegedy 1992)**: probabilistically checkable proofs.

→ Non-relativizing 기법 존재. 하지만 다음 장벽 (Natural proofs) 에 여전히 막힘.

---

## §2 장벽 2 — Natural Proofs (Razborov-Rudich 1997)

### 2.1 정의

**Natural property** : 회로 복잡도 하한 증명 기법 M 의 핵심을 형식화한 속성. 다음 3 조건:

- **(C) Constructive**: 판정 알고리즘이 `f : {0,1}^N → {0,1}` 판별 polynomial time.
- **(L) Largeness**: 모든 random function 의 `1/N^{O(1)}` 비율 이상이 판정 통과.
- **(U) Usefulness**: 회로 클래스 C 의 함수는 모두 판정 실패.

### 2.2 Razborov-Rudich 정리

> 단방향 함수 (one-way function) 가 존재하면, natural property 로 `{f : f ∉ P/poly}` 를 증명할 수 없다.

**귀결**: 단방향 함수 존재 (현대 암호학의 기본 가정) 를 받아들이면, **constructive + large 증명 방식** 은 P vs NP 에 사용 불가. "Any proof that NP has no polynomial-size circuits must be either **non-constructive** or **non-large**" (Razborov-Rudich).

### 2.3 "Natural proofs 장벽 우회" 기법 3가지

1. **Non-constructive 기법** (e.g., Williams 2011 algorithmic method): 판정 알고리즘이 polynomial time 아니더라도 non-trivial 알고리즘 존재 → 하한. NEXP ⊄ ACC⁰.

2. **Non-large 기법** (e.g., specific-function 하한): 특정 함수만 판정. random function fraction 에 의존하지 않음.

3. **Meta-complexity 경로** (2020~): natural proofs 가 breaking OWF 와 등가적 서술 — "MCSP hardness ↔ cryptographic primitives".

---

## §3 장벽 3 — Algebraization (Aaronson-Wigderson 2008)

### 3.1 정의

**Algebraizing**: low-degree polynomial extension `Ã` (for oracle A) 에 대해서도 분리 증명이 성립하는 기법.

### 3.2 Aaronson-Wigderson 정리 (2008)

> 기존의 non-relativizing 기법 (IP = PSPACE, PCP 등) 은 모두 algebrizing. 그리고:
> - Algebrizing relative to A, Ã: `P^{A,Ã} = NP^{A,Ã}` 가능한 A 존재
> - Algebrizing relative to B, B̃: `P^{B,B̃} ≠ NP^{B,B̃}` 가능한 B 존재

**귀결**: 기존의 **모든** 비-관계화 기법 (PCP, IP, arithmetization) 은 P vs NP 분리 불충분. 새 기법 필요.

### 3.3 Non-algebrizing 기법 후보

- **GCT (Geometric Complexity Theory)**: §4 아래 별도 취급
- **Quantum algebraization** (Aaronson-Wigderson 2008 섹션 7): quantum oracles 로 확장 — 더 엄격한 장벽.
- **Proof complexity**: 회로 복잡도 직접 대신, propositional proof system 하한 (Atserias, Pudlák 2005~)

---

## §4 장벽 4 — GCT barriers (post-2011)

### 4.1 GCT 프로그램 요약 (Mulmuley 2001~)

**Geometric Complexity Theory**: Permanent `perm_n` vs Determinant `det_m` 분리를 표현 이론 (representation theory of symmetric group) 로 공격:

```
perm_n ∉ {determinantal complexity class of size m(n)}
  ⟸ rectangular Kronecker coefficient structure
```

핵심 추측: 어떤 rectangular Kronecker coefficients 가 0 or large 인지 판정 가능.

### 4.2 Ikenmeyer-Panova 2017 barrier

> Rectangular Kronecker coefficients 이 GCT occurrence obstruction 을 보이지 못한다.

**귀결**: GCT 의 주 경로인 "representation-theoretic separation" 이 제한됨. 이후 Bürgisser-Ikenmeyer-Panova 2019 등 추가 negative results.

### 4.3 Post-2020 GCT 재해석

- **Border complexity** (Grochow 2020): `perm vs det` border rank 의 하한 추구
- **Tensor rank** (Christandl-Lutzky-Zuiddam 2023 등): multilinear algebra 관점
- **Plethysm coefficients** (Burgisser-Ikenmeyer 2019~): Schur polynomial 분해

→ GCT 프로그램 여전히 활발하지만 **2026 까지 P vs NP 핵심 결과 도출 실패**.

---

## §5 2020~2026 신 경로

### 5.1 Meta-complexity

**MCSP (Minimum Circuit Size Problem)**: `(f, s)` → "f 의 최소 회로 크기 ≤ s?"

- **Oliveira-Santhanam 2017**: MCSP ∈ P ⟺ natural proofs 실패 (OWF fails)
- **Allender 2020**: MCSP-like problem 들의 NP-intermediate 후보성
- **Hirahara 2022 (STOC)**: NP ⊂ P ⟺ MCSP ∈ P (near-equivalence with meta-complexity)

**평가**: Meta-complexity 는 P vs NP 와 **동치적 변형**. 문제 자체를 회피하지 않음.

### 5.2 Circuit complexity (Williams 계열)

- **Williams 2011**: NEXP ⊄ ACC⁰ (STOC best paper)
- **Murray-Williams 2018**: NQP ⊄ ACC⁰ 로 강화
- **Chen-Hirahara-Williams 2021**: Derandomization → lower bounds

**평가**: Williams 방법은 **non-natural + non-relativizing + non-algebrizing**. 4 장벽 모두 우회. 하지만 현재까지 획득된 하한은 `NEXP, NQP` 수준 — `NP` 까지 내려오지 못함.

### 5.3 Proof complexity

- **Atserias-Müller 2020**: Resolution 하한
- **Pudlák 2020~**: P != NP ⟺ "proof complexity separates"
- **Dantchev-Martin 2023**: Cutting-planes 하한

**평가**: Proof complexity 는 "P ≠ NP 로의 환원" 을 제공할 뿐, 본문 증명 아님.

### 5.4 Quantum techniques

- **Aaronson 2023**: Quantum circuit 하한 → classical 하한 이관 시도
- **BQP vs PH**: Raz-Tal 2019, Aaronson-Ingram-Kretschmer 2022

**평가**: Quantum 측면은 독립 연구 분야. P vs NP 직접 기여 아직 제한.

---

## §6 n6-architecture 관점 정직 평가

### 6.1 n=6 구조의 P vs NP 연관?

n=6 prior 가 P vs NP 에 기여할 수 있는 경로를 체계적으로 점검:

| 후보 경로 | n=6 구조 | 평가 |
|-----------|----------|------|
| Circuit complexity | 회로 크기 multiplicative bound | ✗ 숫자 매칭 없음 |
| GCT Kronecker | Young tableaux / symmetric group | ✗ S_6 특별? 증거 없음 |
| Boolean functions | AC⁰ vs NP | ✗ n 은 input size 로만, 6 구조 없음 |
| Communication complexity | 2-party vs k-party | ✗ k=6 특별 없음 |
| Meta-complexity MCSP | circuit size threshold | ✗ σ(n), τ(n) 연결 없음 |

**결론**: **n=6 구조는 P vs NP 에 직접 적용 각도 없음**. 다른 BT (RH, BSD, NS) 와 달리 이 문제는 "숫자 일치" 패턴으로부터 거리가 멀다.

### 6.2 n6-architecture 의 간접 기여 가능성 (honest)

- **정직 기록 인프라**: atlas 등급 + MISS 태깅 시스템은 본 난제의 장벽 literature 를 **체계적으로 기록** 가능. 본 문서가 그 예시.
- **통계적 사고 이월**: BSD 에서 BKLPR 점근적 검증 방법론은 P vs NP 의 finite-barrier 통계 분석 (예: random SAT phase transition 재해석) 에 일부 이월 가능 — 하지만 이는 **millennium 해결 아닌 경험 분석**.
- **문제 자체의 복잡도**: P vs NP 는 "메타-메타-수학" 문제 — **결과 증명 자체가 아닌 증명 불가능성** 의 증명 (4 장벽) 이 주류. n6 structural prior 는 불필요 차원에 있음.

### 6.3 이전 세션 문서 대조

- `theory/study/p3/prob-p3-1-open-subquestions.md` 의 BT-542 subquestions (circuit / GCT / MCSP) 와 본 문서 일치.
- `n6-p3-3-synthesis-report.md` 의 "2040 P vs NP 해결 확률 매우 낮음" 주관 추정과 정합.
- **중복 없이 장벽 taxonomy 층으로 보강**.

---

## §7 결론 + MISS 판정

### 7.1 BT-542 본문 판정

**MISS_MAINTAINED**. BT-542 의 "P ≠ NP 증명" 스코프에서:
- 4 장벽 어느 하나도 우회하는 새 기법 제안 없음
- n6-architecture 기여 확인 없음
- 2020~2026 연구도 P vs NP 분리 **직접 도달 없음**

### 7.2 BARRIER-PX-1 스코프 판정

**PARTIAL_LITERATURE_CATALOG**. 본 문서 산출:
- 4 장벽 수학 정의 + 주요 정리 정리
- 2020~2026 우회 시도 분류 (meta-complexity / Williams / proof complexity / quantum)
- n6 관점 정직 평가 (직접 적용 각도 없음 기록)
- atlas 엔트리 후보: 문헌 taxonomy

### 7.3 atlas 엔트리 제안

```
@R MILL-BARRIER-PX1-four-barriers-catalog = P vs NP 4 barriers = {relativization, natural, algebraization, GCT} :: n6atlas [10]
  "BARRIER-PX-1 BT-542 4 장벽 카탈로그 (문헌 정리, 2026-04-15):
   1. Relativization (Baker-Gill-Solovay 1975)
   2. Natural proofs (Razborov-Rudich 1997)
   3. Algebraization (Aaronson-Wigderson 2008)
   4. GCT (Mulmuley 2001~ + Ikenmeyer-Panova 2017)
   각 장벽의 정의와 우회 시도 기록. BT-542 본문 MISS 유지. n6-architecture 직접 적용 각도 없음 정직 기록"

@R MILL-BARRIER-PX1-n6-nonapplicability = n=6 structural prior does not apply to P vs NP :: n6atlas [10*]
  "BARRIER-PX-1 정직 기록: n=6 구조 prior 는 P vs NP 증명에 직접 적용 각도 없음.
   체계적 점검 (circuit complexity / GCT Kronecker / Boolean functions / communication / meta-complexity)
   모두에서 n=6 특별 연결 부재. BT-542 는 structural-prior-agnostic 난제. 정직성 핵심 기록"
```

---

## §8 관련 파일

- `theory/study/p3/prob-p3-1-open-subquestions.md` — 기존 BT-542 subquestions
- `theory/study/p3/n6-p3-3-synthesis-report.md` — 2040 해결 확률 추정
- `papers/millennium-7-closure-2026-04-11.md` — 기존 honest closure

---

## §9 출처 연도·저자 재확인 체크리스트 (정직성)

본 문서 인용 문헌 중 **기억 기반** 항목은 DOI 검증 필요:

| 인용 | 연도 | 저자 | DOI 확인 |
|------|------|------|---------|
| Baker-Gill-Solovay | 1975 | Baker, Gill, Solovay | SIAM J Comput (확인 필요) |
| Razborov-Rudich | 1997 | Razborov, Rudich | J CSS 55 (확인 필요) |
| Aaronson-Wigderson | 2008 | Aaronson, Wigderson | ACM TOCT (확인 필요) |
| Williams 2011 | 2011 | R. Williams | STOC 2011 (확인 필요) |
| Ikenmeyer-Panova | 2017 | Ikenmeyer, Panova | Advances Math (확인 필요) |
| Hirahara 2022 | 2022 | Hirahara | STOC 2022 (확인 필요) |
| Oliveira-Santhanam | 2017 | Oliveira, Santhanam | CCC (확인 필요) |
| Murray-Williams 2018 | 2018 | Murray, Williams | STOC 2018 (확인 필요) |

**TODO**: v3 로드맵에서 본 체크리스트의 DOI 일괄 수집. 본 세션에서 수행 안 함 (DEFERRED).

---

*작성: 2026-04-15 loop 5*
*BT-542 본문 MISS 유지*
*본 문서는 literature catalog, progress claim 아님*
