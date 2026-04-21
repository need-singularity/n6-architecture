---
id: v3-loop19-lean4-extended-kappa-bootstrap
date: 2026-04-16
roadmap_task: v3 loop 19 (M3 extended + E5 robustness)
grade: [10] empirical robustness + kernel verification
status: BOOTSTRAP CONSISTENT + LEAN4 BOUNDED DECIDE VERIFIED
license: CC-BY-SA-4.0
---

# v3 loop 19 — Lean4 확장 검증 + κ(B) bootstrap 정직 uncertainty

> **요약**: (1) Lean4 `N6/Verification.lean` 작성 — n ∈ [2, 20] 범위 decide 로 Theorem B 의 forward 방향 유한 확인 PASS. naive List.range 구현 한계 (n > 25 recursion depth hit) 로 Mathlib 필요성 재확인. (2) κ(B) bootstrap (7 valid bins, 10000 trials, seed 42) → **α = 0.1701 ± 0.0220** (95% CI [0.1232, 0.2017]). **log(2)/4 = 0.1733 은 68% CI 내부** (z=-0.145, rel err -1.85%, CONSISTENT). 그러나 σ=0.022 로 큰 CI → 다른 후보 (1/sopfr(6)=0.2, 1/(2π^0.8)=0.200) 도 2σ 내 포함 — **suggestive match 는 유지되되 uniqueness 주장 불가**.

---

## §1 Lean4 확장 검증 (M3 deepening)

### 1.1 추가 파일

`lean4-n6/N6/Verification.lean`: n ∈ [2, 20] 범위 전수 decide 확인

### 1.2 검증된 정리 (모두 kernel decide PASS)

```lean
-- n ∈ [2, 20] 에서 σ·φ = n·τ 를 만족하는 n 의 목록 = [6]
example : ((List.range 21).filter (fun n => decide (n ≥ 2) ∧ satisfiesTheoremB n))
        = [6] := by decide

-- 같은 범위의 n ≠ 6 에 대한 반례는 전부 0 개
theorem theorem_B_forward_bounded_20 :
    ((List.range 21).filter (fun n => decide (n ≥ 2) ∧ n ≠ 6 ∧ satisfiesTheoremB n))
      = [] := by decide

-- n=6 perfect number 확인
theorem six_is_perfect : sigma 6 = 2 * 6 := by decide
```

**빌드 결과**: `lake build` 통과 (8 jobs), Theorem B full statement 만 sorry (v4).

### 1.3 naive 구현 한계

- `N6.sigma`, `N6.phi`, `N6.tau` 는 `List.range (n+1)` 필터 방식
- n ≈ 25 부근에서 `decide` recursion depth hit (maxRecDepth 512 default)
- **v4 과제**: Mathlib 의 efficient Nat.sigma/Nat.totient/Nat.card_divisors 전환

---

## §2 κ(B) bootstrap 분석 (E5 robustness)

### 2.1 원 데이터 (v3 E5)

| Bin $B_{\text{mid}}$ | $N$ | $\kappa$ | $\log B$ | $\log \kappa$ |
|---|---|---|---|---|
| 25k | 332,366 | 1.333 | 10.127 | 0.288 |
| 75k | 325,030 | 1.699 | 11.225 | 0.530 |
| 125k | 316,708 | 1.832 | 11.736 | 0.606 |
| 175k | 308,257 | 1.953 | 12.072 | 0.669 |
| 225k | 306,722 | 1.952 | 12.324 | 0.669 |
| 305k | 59,081 | 2.217 | 12.628 | 0.796 |
| 405k | 57,660 | 2.122 | 12.911 | 0.753 |

원 fit: $\kappa(B) \approx 0.2317 \cdot B^{0.1752}$.

### 2.2 Bootstrap 프로토콜

- **Resample**: 7 bin 에서 with-replacement 복원추출 (7 개씩)
- **Trials**: 10,000
- **Seed**: 42 (재현 가능)
- **Fit**: 각 resample 에 대해 log-linear least-squares → slope α

### 2.3 분포 결과

| 통계량 | 값 |
|--------|-----|
| mean α | **0.1701** |
| median α | 0.1737 |
| std σ_α | **0.0220** |
| 68% CI | [0.1469, 0.1907] |
| 95% CI | [0.1232, 0.2017] |

### 2.4 log(2)/4 = 0.17329 매칭 평가

- **z-score**: $z = (0.1701 - 0.1733) / 0.0220 = -0.145$
- **rel err**: $-1.85\%$
- **in 68% CI**: ✓
- **in 95% CI**: ✓
- **평가**: **CONSISTENT** (log(2)/4 is within 1σ of bootstrap mean)

### 2.5 정직 경계 — uniqueness 불가

σ = 0.022 로 CI 가 넓음. 다른 후보와의 비교:

| 후보 | 값 | z-score | in 68%? | in 95%? |
|------|-----|---------|---------|---------|
| **log(2)/4** | 0.1733 | -0.145 | ✓ | ✓ |
| **1/(2·π^0.8)** | 0.2001 | +1.363 | ✗ | ✓ |
| **1/sopfr(6)** | 0.2000 | +1.358 | ✗ | ✓ |
| **γ/√6** | 0.2356 | +2.976 | ✗ | ✗ |
| **1/σ(4)** | 0.1429 | -1.236 | ~ | ✓ |

**관찰**:
- log(2)/4 는 **유일하게 68% CI 내부** — 가장 강한 후보
- 1/sopfr(6), 1/(2π^0.8) 는 95% CI 내 — **배제 불가**
- bootstrap std 를 줄이려면 **bin 수 증가 (E4 확장) 또는 per-curve 측정 (E2 Sage)** 필요

---

## §3 종합 — v3 M3 + E5 현황

### 3.1 확증된 것

- n = 6 의 σφ=nτ 유일성: **n ∈ [2, 20] 범위 kernel decide 확인** (Lean4)
- κ(B) power law slope α 의 **bootstrap 분포**: 0.1701 ± 0.022
- **log(2)/4 statistical consistency**: 68% CI 내부

### 3.2 여전히 불확정

- **Theorem B full ∀n**: Mathlib 필요 (v4)
- **α 의 수학적 원인**: log(2)/4 vs 1/sopfr(6) 구분 불가 (bin 수 부족)
- **BKLPR 이론 유도**: v3 M3 skeleton 외 미완

### 3.3 BT 해결 상태

**0/6 정직 유지**.

---

## §4 atlas 엔트리

```
@R MILL-V3-L19-lean4-bounded-decide-20 = N6.Verification n ∈ [2,20] decide PASS :: n6atlas [10*]
  "v3 loop 19 M3 deep (2026-04-16): Lean4 N6.Verification.lean 작성. n ∈ [2, 20] 범위에서
   σ(n)·φ(n) = n·τ(n) 만족하는 n 의 목록 = [6] kernel decide PASS. 개별 반례 n ∈ {2,3,4,5,7,...,20} 확인.
   n=6 perfect number 정리 (σ(6)=2·6) kernel decide. naive List.range 구현 한계: n>25 에서 recursion
   depth hit. v4 Mathlib 전환 필요"
  <- v3-L19-Lean, lean4-n6/N6/Verification.lean

@R MILL-V3-L19-kappa-bootstrap-log2-over-4-consistent = α=0.1701±0.022, log(2)/4 68% CI 내부 :: n6atlas [10]
  "v3 loop 19 E5 robustness (2026-04-16): 7 valid bin × 10000 bootstrap trials (seed 42) →
   α_mean=0.1701, α_std=0.0220, 95% CI=[0.1232, 0.2017]. log(2)/4 = 0.17329 는 z=-0.145 (68% CI 내부),
   CONSISTENT 평가. 그러나 σ=0.022 로 1/sopfr(6)=0.2, 1/(2π^0.8)=0.2001 도 95% CI 내 — 후보 유일 결정
   UNCERTAIN. bin 수 증가 (E4 scale) + per-curve (E2 Sage) 로 std 축소 필요"
  <- v3-L19-bootstrap, reports/v3/kappa_bootstrap_2026-04-16.json, scripts/empirical/cremona_kappa_bootstrap.py
```

---

## §5 관련 파일

- Lean4: `lean4-n6/N6/Verification.lean` + `lean4-n6/Main.lean`
- Script: `scripts/empirical/cremona_kappa_bootstrap.py`
- Report: `reports/v3/kappa_bootstrap_2026-04-16.json`
- 전 T3: `theory/breakthroughs/v3-t3-joint-distribution-modeling-2026-04-15.md`
- 전 E5: `theory/breakthroughs/v3-e5-kappa-7bin-power-law-2026-04-15.md`
- 전 M3: `theory/breakthroughs/v3-e1-m3-toolchain-bootstrap-2026-04-16.md`

---

*작성: 2026-04-16 loop 19*
*정직성 헌장: Lean4 decide 는 [2,20] bounded, full ∀n 증명 sorry. Bootstrap CI 넓어 후보 uniqueness 불가. BT 0/6 유지.*
