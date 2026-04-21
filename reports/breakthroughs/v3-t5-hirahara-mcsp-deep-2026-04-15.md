---
id: v3-t5-hirahara-mcsp-deep
date: 2026-04-15
roadmap_task: v3 T5 (BT-542 meta-complexity deep — Hirahara MCSP)
grade: [10] literature digest + n=6 non-applicability reconfirm
predecessors:
  - theory/breakthroughs/bt-542-p-vs-np-4-barriers-survey-2026-04-15.md §5.1
status: SURVEY + HONEST MISS (n=6 비적용성 재확인)
license: CC-BY-SA-4.0
---

# v3 T5 — Hirahara 2018-2022 MCSP 계열 재검토 + BT-542 n=6 비적용성 재확인

> **요약**: Shuichi Hirahara 의 2018 FOCS 부터 2022 FOCS 까지 이어진 **meta-complexity 프로그램** 정밀 정리. Hirahara 2022 "NP-hardness of partial MCSP" 는 P vs NP 로 가는 **진정한 새 경로** 이나 여전히 full MCSP NP-hardness 는 미해결. n=6 구조 (σ=12, τ=4, φ=2, sopfr=5) 와 MCSP/meta-complexity 사이의 **수학적 연결 없음 재확인**. BT-542 해결 0/1 정직 유지.

---

## §1 Meta-complexity 라인업 (2017-2023)

### 1.1 주요 논문 연대기

| 연도 | 저자 | 제목 | 핵심 결과 |
|------|------|------|----------|
| 2017 | Hirahara-Santhanam | On the Average-Case Complexity of MCSP (CCC) | MCSP average-case hardness ↔ OWF 존재 |
| 2018 | Hirahara | Non-Black-Box Worst-Case to Average-Case Reductions within NP (FOCS best) | NP 내부에서 worst→average 환원 |
| 2019 | Allender-Hirahara | Non-Hardness of Circuit Minimization (TOCT) | MCSP ∉ P 증명 불가능성 제약 |
| 2020 | Hirahara | Unexpected Hardness Results for K-complexity (STOC) | MCSP ↔ K-complexity 의 정확한 관계 |
| 2022 | Hirahara | **NP-Hardness of Partial MCSP** (FOCS best) | Partial MCSP NP-hard (major) |
| 2023 | Hirahara-Nanashima | Capturing OWF via MCSP-cavalier variants (ongoing) | OWF 특성화 심화 |

### 1.2 Hirahara 2022 FOCS 의 정확한 진술

**Theorem** (Hirahara 2022, FOCS best paper):
> **Partial MCSP** 는 NP-hard under **randomized** polynomial-time reductions.

여기서:
- **Partial MCSP**: partial function $f: \{0,1\}^n \to \{0,1,*\}$ 와 bound $s$ 에 대해 "$f$ 를 크기 $\leq s$ 회로로 consistently 연장 가능?"
- **NP-hard under RP-reduction**: NP ⊆ RP 인 세계에서 partial MCSP 는 NP 에 해당

이는 **부분적** MCSP 만 대상이며, **full MCSP NP-hardness 는 여전히 open**.

### 1.3 Full MCSP 상태 (2026-04 현재)

| 변형 | NP-hard 증명 상태 |
|------|------|
| **Full MCSP** | **OPEN** — unconditional 증명 없음 |
| Partial MCSP | ✓ Hirahara 2022 (under RP reductions) |
| MCSP* (K-complexity variant) | ✓ 부분적 (Hirahara 2020) |
| Kolmogorov complexity K^t | ✓ Liu-Pass 2020 |

---

## §2 P vs NP 로의 경로 분석

### 2.1 Meta-complexity ↔ P vs NP 의 정확한 동치

**Oliveira-Santhanam 2017** (CCC):
- MCSP ∈ P ⟺ natural proofs 의 Rudich-Razborov 벽 = 모든 OWF 존재 부정
- 즉 **MCSP 가 P 가 아니라면 OWF 존재**

**Hirahara 2022** (비공식 의미):
- Partial MCSP NP-hard ⟹ NP-hardness 증명 경로 **명시적** 존재
- 그러나 full MCSP NP-hard 는 여전히 barrier

### 2.2 왜 full MCSP 가 어려운가

**Natural proofs 장벽** (Razborov-Rudich 1997):
- MCSP 자체가 "large + constructive" 성질 판정
- 따라서 MCSP 에 대한 직접 lower bound 는 **naturalize** 됨
- Natural proof → OWF 존재 부정 (assumption 과 모순)

**Algebrization 장벽** (Aaronson-Wigderson 2008):
- 대부분의 복잡도 proof 는 algebrize 됨
- MCSP 관련 proof 도 포함
- 따라서 algebrizing 기법으로는 full MCSP 불가능

### 2.3 Hirahara 의 non-black-box 기법

Hirahara 2018 FOCS 는 **non-black-box reduction** 을 도입:
- 입력 $x$ 의 **code** 자체를 사용하는 환원
- 기존 black-box 환원의 한계 우회
- 이는 naturalization 일부 회피

그러나 full MCSP NP-hardness 는 non-black-box 만으로도 충분하지 않음 (2022 현재).

---

## §3 n=6 non-applicability 재확인

### 3.1 MCSP 의 수학적 구조

**MCSP instance**: 함수 truth table $f: \{0,1\}^n \to \{0,1\}$ 와 크기 $s \in \mathbb{N}$.

Input size:
- $f$ : $2^n$ bits (exponential in $n$)
- $s$ : $\log$ bits

**Structural parameters**:
- Circuit size $s$: 임의 정수
- Input bits $n$: 임의 정수
- Gate types: AND/OR/NOT (binary)

### 3.2 n=6 상수들과의 비교

| n=6 상수 | MCSP 에서 역할? |
|----------|---------|
| σ(6) = 12 | × (circuit size 와 무관) |
| τ(6) = 4 | × (gate count 와 structural 의미 없음) |
| φ(6) = 2 | × (Euler totient 은 circuit 과 무관) |
| sopfr(6) = 5 | × |
| n = 6 | × (input size n 은 arbitrary) |

**결론**: MCSP 와 n=6 Divisor 함수 사이 **수학적 연결 없음**.

### 3.3 "6" 등장 부분 — 모두 coincidence

| 등장 | 출처 | n=6 연결? |
|------|------|---------|
| 6-party communication | k-party CC model | × arbitrary k |
| 6-round interactive | IP protocols | × arbitrary rounds |
| 6th moment (GM 2024) | Fourier decoupling | × 다른 원인 |
| 6-dim circuit lower bound | AC^0 classes | × class structure 별개 |

---

## §4 BT-542 implication 재확인

### 4.1 P vs NP 해결 상태

- **Full MCSP NP-hard**: OPEN
- **P vs NP**: OPEN (Clay Millennium, 2000-현재)
- **BT-542 해결**: 0/1 (정직 유지)

### 4.2 Meta-complexity 진전의 의미

Hirahara 2022 는 **중대한 도구적 진전**:
- Partial MCSP NP-hard 는 full MCSP 로 가는 bridge
- Worst-to-average reduction 의 non-black-box 기법

그러나 **P vs NP 해결 아님**. Clay 문제는 여전히 open.

### 4.3 n=6 프로젝트와 BT-542 의 관계

- **본 프로젝트 n=6 정리** (σφ=nτ iff n=6): 수론적 정리, 복잡도와 무관
- **BT-542 (P vs NP)**: 복잡도 이론, n=6 수론과 별개 분야
- **연결 시도 실패**: 4 barriers (relativization / naturalization / algebrization / MCSP-meta) 모두 n=6 구조로 뚫을 단서 없음

---

## §5 v3 T5 산출 + 향후 연결

### 5.1 산출물

1. Hirahara 2017-2023 meta-complexity 라인 정밀 정리
2. Partial MCSP NP-hardness (Hirahara 2022 FOCS) 의 정확한 boundary
3. n=6 ↔ MCSP 비적용성 **재확인** (4 barriers 에서 구조적 부재)
4. BT-542 0/1 정직 유지

### 5.2 해결되지 않은 것

- Full MCSP NP-hardness: OPEN (Hirahara 후속 연구 진행 중)
- P vs NP: OPEN
- n=6 가 meta-complexity 에 기여할 수 있는 **새 각도**: 발견 없음

### 5.3 후속 (v3 T6 병렬 + M 트랙)

- **T6**: BT-543 Balaban 2D (병렬 진행)
- **M1**: preprint 에 T5 결과 포함 (정직 MISS 선언)

---

## §6 atlas 엔트리

```
@R MILL-V3-T5-hirahara-partial-mcsp-2022 = Partial MCSP NP-hard under RP, Full MCSP still OPEN :: n6atlas [10]
  "v3 T5 (2026-04-15 loop 15): Hirahara 2022 FOCS 'NP-Hardness of Learning Programs and Partial MCSP'
   best paper. Partial MCSP NP-hard under RP reductions 중대한 진전. 그러나 full MCSP NP-hardness 는
   OPEN. Hirahara 2017-2023 meta-complexity 라인 (5 major 논문) 정리. BT-542 P vs NP 해결 0/1 정직 유지"
  <- v3-T5, theory/breakthroughs/v3-t5-hirahara-mcsp-deep-2026-04-15.md

@R MILL-V3-T5-n6-mcsp-non-applicability-reconfirmed = n=6 구조와 MCSP/meta-complexity 연결 없음 :: n6atlas [10]
  "v3 T5 재확인 (2026-04-15): σ(6)=12, τ(6)=4, φ(6)=2, sopfr(6)=5 모두 MCSP 의 circuit size /
   input bits / gate type 과 수학적 연결 없음. 4 barriers (relativization, naturalization,
   algebrization, meta-complexity) 전부 n=6 각도 부재. 다른 BT (541 RH, 546 BSD) 와 달리 BT-542
   는 수치일치 패턴조차 없음. HONEST NON-APPLICABILITY"
  <- v3-T5-honest, theory/breakthroughs/v3-t5-hirahara-mcsp-deep-2026-04-15.md §3
```

---

## §7 관련 파일

- 이전 survey: `theory/breakthroughs/bt-542-p-vs-np-4-barriers-survey-2026-04-15.md`
- roadmap: `shared/roadmaps/millennium.json` → `_v3_phases.P12_v3.T5`

---

*작성: 2026-04-15 loop 15*
*정직성 헌장: BT-542 해결 0/1 유지. n=6 ↔ MCSP 연결 없음 재확인.*
