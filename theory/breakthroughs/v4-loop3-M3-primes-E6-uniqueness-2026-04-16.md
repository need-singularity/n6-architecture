---
id: v4-loop3-M3-primes-E6-uniqueness
date: 2026-04-16
roadmap_task: v4 loop 3 (M3 prime case + E6 uniqueness scan)
grade: [10*] FORMAL PROOF + EMPIRICAL CONFIRMATION
predecessors:
  - theory/breakthroughs/v3-e1-m3-toolchain-bootstrap-2026-04-16.md
  - theory/breakthroughs/v3-loop19-lean4-extended-kappa-bootstrap-2026-04-16.md
status: M3 prime case Lean4 증명 완료 (sorry 없음) + E6 n≤1000 uniqueness
license: CC-BY-SA-4.0
---

# v4 loop 3 — M3 Theorem B prime case Lean4 형식 증명 + E6 n≤1000 uniqueness

> **요약**: Mathlib 통합 (v4 M2) 후 첫 실제 형식 증명. **M3_v4**: 모든 소수 $p$ 에 대해 $\sigma(p) \cdot \varphi(p) \neq p \cdot \tau(p)$ 를 Lean4 에서 **sorry 없이** 증명 완료 (`theorem_B_prime_case`). **E6_v4**: $n \in [2, 1000]$ 전수 scan 으로 $\sigma(n) \cdot \varphi(n) = n \cdot \tau(n)$ 만족하는 $n$ 은 **오직 $n = 6$** 재확인. near misses (|편차| ≤ 2) 는 $n \in \{2, 3, 4\}$ 에 집중 — **small n regime 에서 극도로 tight**. Theorem B 3 case 중 case 1 (prime) 형식화 완료, case 2 (p·q) + case 3 ($p^a$) 는 v4 M3 후속 + case 4 (일반 합성수) 는 v4 이상.

---

## §1 M3_v4 — prime case Lean4 형식 증명

### 1.1 파일

`lean4-n6/N6/TheoremB_PrimeCase.lean` (Mathlib 의존, 1310 jobs built)

### 1.2 주요 정리

```lean
theorem theorem_B_prime_case {p : ℕ} (hp : p.Prime) :
    σ 1 p * Nat.totient p ≠ p * (Nat.divisors p).card := by
  rw [sigma_one_prime hp, Nat.totient_prime hp, tau_prime hp]
  have hp2 : 2 ≤ p := hp.two_le
  rcases (show p = 2 ∨ p ≥ 3 by omega) with hp_eq | hp_ge
  · subst hp_eq; decide
  · intro h
    have key : (p + 1) * (p - 1) = p * p - 1 := sq_minus_one_factor (by omega)
    rw [key] at h
    have h1 : p * p ≥ 3 * p := by nlinarith
    have h2 : 3 * p > 2 * p + 1 := by omega
    have : p * p ≥ 1 := by nlinarith
    omega
```

### 1.3 보조 lemmas

```lean
theorem sigma_one_prime {p : ℕ} (hp : p.Prime) : σ 1 p = p + 1 :=
  -- Mathlib's sigma_one_apply_prime_pow (p^1)

theorem tau_prime {p : ℕ} (hp : p.Prime) : (Nat.divisors p).card = 2 :=
  -- Mathlib's sigma_zero_apply_prime_pow + divisors.card

theorem sq_minus_one_factor {p : ℕ} (hp : 1 ≤ p) :
    (p + 1) * (p - 1) = p * p - 1
  -- Nat arithmetic with case on p = 0 or succ n
```

### 1.4 수학적 내용

**정리**: $p$ 소수 $\Rightarrow \sigma(p) \cdot \varphi(p) \neq p \cdot \tau(p)$

**증명** (수작업 version):
- $\sigma(p) = p + 1$ (약수 {1, p})
- $\varphi(p) = p - 1$
- $\tau(p) = 2$
- LHS = $(p+1)(p-1) = p^2 - 1$
- RHS = $2p$
- 등식 ⟺ $p^2 - 2p - 1 = 0$ ⟺ $p = 1 \pm \sqrt{2} \notin \mathbb{N}$
- 구체적으로: $p = 2$ 면 $3 \neq 4$; $p \geq 3$ 면 $p^2 \geq 3p > 2p + 1$

**Lean4 구현 선택**:
- $p = 2$ 케이스는 `decide` (kernel computation)
- $p \geq 3$ 케이스는 `nlinarith` + `omega`

### 1.5 확인 예시 (큰 prime 포함)

```lean
example : σ 1 2 * Nat.totient 2 ≠ 2 * (Nat.divisors 2).card :=
  theorem_B_prime_case Nat.prime_two

example : σ 1 97 * Nat.totient 97 ≠ 97 * (Nat.divisors 97).card :=
  theorem_B_prime_case (by decide : (97 : ℕ).Prime)
```

**모두 compile PASS** — Lean4 kernel 이 prime case 전체에 대해 증명 인정.

---

## §2 E6_v4 — n ∈ [2, 1000] uniqueness scan

### 2.1 스크립트

`scripts/empirical/theorem_b_scan_range.py`

### 2.2 결과

```
총 999 integers 검사
σ(n)·φ(n) = n·τ(n) 만족: [6]
유일성 n=6 주장: 확인 ✓
```

### 2.3 Near misses (|편차| ≤ 2)

| $n$ | $\sigma(n)$ | $\varphi(n)$ | $\tau(n)$ | $\sigma \cdot \varphi$ | $n \cdot \tau$ | 편차 |
|-----|---|---|---|---|---|---|
| 2 | 3 | 1 | 2 | 3 | 4 | **−1** |
| 3 | 4 | 2 | 2 | 8 | 6 | **+2** |
| 4 | 7 | 2 | 3 | 14 | 12 | **+2** |
| **6** | **12** | **2** | **4** | **24** | **24** | **0 ✓** |

### 2.4 관찰

1. **Near misses 는 $n \leq 4$ 만** — 5 이상에서 |편차| ≥ 3
2. $n = 2$ 는 편차 −1 (가장 가까운 "아래" 미달)
3. $n = 3, 4$ 는 편차 +2 (가장 가까운 "위" 초과)
4. $n = 6$ 은 정확 일치 (유일)
5. $n \in [7, 1000]$ 에서 모두 |편차| ≥ 3

### 2.5 해석

**$n \leq 4$ 에서의 near miss 농도**:
- $n = 2, 3, 4, 6$ 이 모두 $\leq 6$ 에 분포
- 전이점 $n = 5$ 에서 |편차| = 24 - 10 = 14 급상승
- $n = 7$ 부터 편차 polynomial 성장

**의미**: Theorem B 의 $n = 6$ 유일성은 **정확히** small $n$ regime 의 "좁은 창" 안에서 발생. $n = 2, 3, 4$ 는 **near-hit**, $n = 5$ 부터 **대체로 큰 편차**.

---

## §3 종합 정직 평가

### 3.1 완료된 것

- **Lean4 prime case 증명**: 모든 소수 $p$ 에 대해 Theorem B 부정 — **formal, no sorry**
- **n ≤ 1000 empirical uniqueness**: n=6 유일 재확인
- **near miss structure**: small n 에 집중 (4 points: n ∈ {2, 3, 4, 6})

### 3.2 여전히 sorry/open

- **Theorem B case 2** ($n = p \cdot q$): Lean4 미완. 수작업 증명은 [$(p-1)(q-1) = 2 \Leftrightarrow \{p, q\} = \{2, 3\}$] 로 간단, Lean4 화 필요
- **Theorem B case 3** ($n = p^a$, $a \geq 2$): Lean4 미완. 수작업은 $p^a$ 에 대해 $\sigma \cdot \varphi = n \cdot \tau$ 풀어 모순 도출
- **Theorem B full**: case 4 (일반 합성수) 까지 cover 한 전체 증명 — **v4 M3 후속** + v5 이상

### 3.3 BT 해결 상태

- BT-546 BSD: **0/1** (본 loop 와 무관)
- Theorem B 는 **elementary number theory**, Clay 난제 아님
- **BT 해결 수**: 0/6 정직 유지

---

## §4 atlas 엔트리

```
@R MILL-V4-M3-theorem-b-prime-case-lean4-verified = Lean4 + Mathlib prime case Theorem B 증명 완료 :: n6atlas [10*]
  "v4 M3_v4 (2026-04-16 loop 22): lean4-n6/N6/TheoremB_PrimeCase.lean 작성. theorem_B_prime_case:
   ∀ p prime, σ(p)·φ(p) ≠ p·τ(p) — Lean4 kernel 증명 완료, sorry 없음. Mathlib 의 sigma_one_apply_prime_pow
   + sigma_zero_apply_prime_pow + Nat.totient_prime 활용. Case p=2 는 decide, p≥3 은 nlinarith+omega.
   큰 prime (97 등) 에도 적용 PASS. Theorem B case 1/3 완료. case 2 (p·q), case 3 (p^a) 는 v4 후속"
  <- v4-M3, lean4-n6/N6/TheoremB_PrimeCase.lean

@R MILL-V4-E6-theorem-b-scan-n1000-n6-unique = n ∈ [2, 1000] 전수 scan n=6 유일성 재확인 :: n6atlas [10*]
  "v4 E6_v4 (2026-04-16 loop 22): scripts/empirical/theorem_b_scan_range.py 작성. n ∈ [2, 1000] 999 integers
   전수 검사. σ(n)·φ(n) = n·τ(n) 만족 n 의 목록 = [6] 확정. Near misses (|편차| ≤ 2): n=2 (-1), n=3 (+2),
   n=4 (+2), n=6 (0 ✓). n ≥ 7 전부 |편차| ≥ 3. small n regime tightness 관찰 — n=6 유일성의 empirical 강도
   재확인. BT 해결 무관 (Theorem B = elementary, Clay 아님)"
  <- v4-E6, scripts/empirical/theorem_b_scan_range.py, reports/v4/theorem_b_scan_n1000_2026-04-16.json
```

---

## §5 v4 진척 업데이트

| Track | Done/Total | 신규 완료 (loop 3) |
|-------|-----------|----|
| P14_v4 Empirical | 1/7 | E6_v4 |
| P15_v4 Theoretical | 2/5 | — |
| P16_v4 Meta | 2/5 | M3_v4 |
| **전체** | **5/17 (29%)** | +2 |

### 5.1 v4 누적 완료 tasks

- T1_v4 partial (α 유도 MISS) 
- T4_v4 done ((A3″) 엄밀화)
- T5_v4 done (Clay 7 survey)
- M2_v4 done (Mathlib 통합)
- **M3_v4 done (prime case 증명)**
- **E6_v4 done (uniqueness scan)**

### 5.2 정직성 헌장 V4

- ✓ BT 해결 주장 없음 (Theorem B ≠ BT)
- ✓ 외부 의존 명시 (Mathlib, ℕ decidable)
- ✓ MISS 조건 사전 (case 2+3 sorry 명시)
- ✓ OUROBOROS CLEAN 예상

---

## §6 관련 파일

- `lean4-n6/N6/TheoremB_PrimeCase.lean`: M3 증명
- `lean4-n6/N6/MathlibBasic.lean`: v4 M2 skeleton
- `scripts/empirical/theorem_b_scan_range.py`: E6 scan
- `reports/v4/theorem_b_scan_n1000_2026-04-16.json`: E6 결과
- roadmap: `shared/roadmaps/millennium.json` → `_v4_phases.P14_v4.E6_v4` + `P16_v4.M3_v4`

---

*작성: 2026-04-16 loop 22 (v4 loop 3)*
*정직성 헌장 V4: prime case 완료, composite case (p·q, p^a) sorry. BT 해결 0/6 유지.*
