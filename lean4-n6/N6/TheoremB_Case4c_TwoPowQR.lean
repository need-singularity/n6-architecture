-- N6.TheoremB_Case4c_TwoPowQR : Theorem B case 4c(ii) (n = 2^a·q·r, a ≥ 2)
-- v4 M3_v4 case 4c(ii) (2026-04-16 loop 31)
--
-- 목표: a ≥ 2, q, r odd prime ≥ 3 distinct (q ≠ r) → σ(n)·φ(n) ≠ n·τ(n) for n = 2^a·q·r
--
-- 전략 (strict bound 곱):
--   σφ(2^a) > 2^a·(a+1)  (case 3 strict 형태)
--   σφ(qr)  > (qr)·4     (case 2b strict 형태)
--   곱: σφ(2^a·qr) > 2^a(a+1)·4qr = nτ  → STRICT 모순

import Mathlib.NumberTheory.ArithmeticFunction.Misc
import Mathlib.NumberTheory.Divisors
import Mathlib.Data.Nat.Totient
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Tactic.NormNum.Prime
import N6.TheoremB_PrimeCase
import N6.TheoremB_Case3_PrimePow
import N6.TheoremB_Case2_OddOdd
import N6.TheoremB_Case4_ThreePrimes

namespace N6Mathlib

open Nat ArithmeticFunction
open scoped ArithmeticFunction.sigma

/-- Strict bound: σφ(2^a) > 2^a · (a+1) for a ≥ 2.  (case 3 강화판) -/
theorem sigma_phi_2pow_strict {a : ℕ} (ha : a ≥ 2) :
    σ 1 (2 ^ a) * Nat.totient (2 ^ a) > 2 ^ a * (a + 1) := by
  rw [sigma_one_prime_pow_sum Nat.prime_two,
      Nat.totient_prime_pow Nat.prime_two (by omega : a > 0)]
  -- Goal: (Σ 2^k) · (2^(a-1) · (2-1)) > 2^a·(a+1)
  have h_geom : ∑ k ∈ Finset.range (a + 1), 2 ^ k = 2 ^ (a + 1) - 1 := by
    have := geom_sum_identity (p := 2) (a := a) (by norm_num)
    simp at this; exact this
  rw [h_geom]
  -- Goal: (2^(a+1) - 1) · (2^(a-1) · 1) > 2^a · (a+1)
  -- Simplify: 2-1 = 1, so 2^(a-1)·(2-1) = 2^(a-1)
  -- RHS: 2^a = 2·2^(a-1), so = 2·2^(a-1)·(a+1)
  have h_2a : 2 ^ a = 2 * 2 ^ (a - 1) := by
    have h1 : 2 ^ a = 2 ^ ((a - 1) + 1) := by congr 1; omega
    rw [h1, pow_succ]; ring
  rw [h_2a]
  -- Simplify 2-1 = 1
  show (2 ^ (a + 1) - 1) * (2 ^ (a - 1) * (2 - 1)) > 2 * 2 ^ (a - 1) * (a + 1)
  have h_two_sub : (2 : ℕ) - 1 = 1 := by norm_num
  rw [h_two_sub, Nat.mul_one]
  -- (2^(a+1) - 1)·2^(a-1) > 2·2^(a-1)·(a+1)
  -- Also 2^(a+1) = 4·2^(a-1): use this to reduce
  have h_pow_rel : 2 ^ (a + 1) = 4 * 2 ^ (a - 1) := by
    have h1 : 2 ^ (a + 1) = 2 ^ ((a - 1) + 2) := by congr 1; omega
    rw [h1]
    have : 2 ^ ((a - 1) + 2) = 2 ^ (a - 1) * 2 ^ 2 := by rw [pow_add]
    rw [this]; ring
  rw [h_pow_rel]
  -- (4·X - 1) · X > 2·X·(a+1) where X = 2^(a-1)
  set X := 2 ^ (a - 1) with hX_def
  have h_strict : 2 ^ (a + 1) > 2 * (a + 1) + 1 :=
    prime_pow_strict_gt (by norm_num : (2 : ℕ) ≥ 2) ha
  rw [h_pow_rel] at h_strict
  -- h_strict : 4·X > 2(a+1) + 1
  have h_pow_pos : X > 0 := Nat.pow_pos (by norm_num)
  have h_X_ge_2 : X ≥ 2 := by
    have : X = 2 ^ (a - 1) := hX_def
    calc X = 2 ^ (a - 1) := this
      _ ≥ 2 ^ 1 := Nat.pow_le_pow_right (by norm_num) (by omega)
      _ = 2 := by norm_num
  have h_4X : 4 * X ≥ 8 := by omega
  -- (4X - 1)·X > 2X(a+1)
  -- Since 4X ≥ 8, 4X - 1 ≥ 7
  have h_4X_m1 : 4 * X - 1 ≥ 4 * X - 1 := le_refl _
  -- (4X - 1)X = 4X² - X (in ℕ, with 4X ≥ 1)
  have h_expand : (4 * X - 1) * X = 4 * X * X - X := by
    rw [Nat.sub_mul, Nat.one_mul]
  rw [h_expand]
  -- 4X² - X > 2X(a+1) ⟺ 4X² > 2Xa + 2X + X = 2Xa + 3X ⟺ X(4X - 2a - 3) > 0
  -- From 4X > 2a + 3: 4X - 2a - 3 ≥ 1, so X·(≥1) ≥ X > 0 ✓
  have h_4X_gt : 4 * X > 2 * a + 3 := by omega
  -- 4X² ≥ (2a+4)X ≥ 2Xa + 4X, so 4X² - X ≥ 2Xa + 3X > 2X(a+1) = 2Xa + 2X
  have h_4X_sq : 4 * X * X ≥ (2 * a + 4) * X := Nat.mul_le_mul_right X (by omega)
  have : 4 * X * X - X ≥ 2 * a * X + 3 * X := by
    have h_expand_rhs : (2 * a + 4) * X = 2 * a * X + 4 * X := by ring
    rw [h_expand_rhs] at h_4X_sq
    omega
  have h_target : 2 * X * (a + 1) = 2 * a * X + 2 * X := by ring
  rw [h_target]; omega

/-- Strict bound: σφ(qr) > qr·4 for q, r odd distinct primes ≥ 3.  (case 2b strict) -/
theorem sigma_phi_qr_strict {q r : ℕ}
    (hq : q.Prime) (hr : r.Prime) (hq3 : q ≥ 3) (hr5 : r ≥ 5) (hqr : q ≠ r) :
    σ 1 (q * r) * Nat.totient (q * r) > (q * r) * 4 := by
  have hcop : Nat.Coprime q r := coprime_of_distinct_primes hq hr hqr
  rw [sigma_one_pq hq hr hcop, totient_pq hq hr hcop]
  -- Goal: (q+1)(r+1) * ((q-1)(r-1)) > qr·4
  -- Bound: (q+1)(q-1) ≥ 2q + 2 for q ≥ 3, (r+1)(r-1) ≥ 2r + 8 for r ≥ 5
  -- 곱: ≥ (2q+2)(2r+8) = 4qr + 16q + 4r + 16 > 4qr
  have hq_lb : (q + 1) * (q - 1) ≥ 2 * q + 2 := by
    -- Expand: (q+1)(q-1) — q ≥ 1, so q - 1 = q' with q = q' + 1
    -- (q'+2)·q' = q'² + 2q'. With q ≥ 3, q' = q-1 ≥ 2
    -- 2q + 2 = 2(q'+1) + 2 = 2q' + 4
    -- Need q'² + 2q' ≥ 2q' + 4 ⟺ q'² ≥ 4, i.e., q' ≥ 2 ✓
    have hq1 : q - 1 ≥ 2 := by omega
    have h_eq : (q + 1) * (q - 1) = (q - 1) * (q - 1) + 2 * (q - 1) := by
      have h_q : q + 1 = (q - 1) + 2 := by omega
      rw [h_q]; ring
    rw [h_eq]
    have : (q - 1) * (q - 1) ≥ 4 := by
      have : (q - 1) * (q - 1) ≥ 2 * 2 := Nat.mul_le_mul hq1 hq1
      omega
    omega
  have hr_lb : (r + 1) * (r - 1) ≥ 2 * r + 8 := by
    have hr1 : r - 1 ≥ 4 := by omega
    have h_eq : (r + 1) * (r - 1) = (r - 1) * (r - 1) + 2 * (r - 1) := by
      have h_r : r + 1 = (r - 1) + 2 := by omega
      rw [h_r]; ring
    rw [h_eq]
    have : (r - 1) * (r - 1) ≥ 16 := by
      have : (r - 1) * (r - 1) ≥ 4 * 4 := Nat.mul_le_mul hr1 hr1
      omega
    omega
  -- Reorganize LHS: (q+1)(r+1)·(q-1)(r-1) = (q+1)(q-1) · (r+1)(r-1)
  have reorg :
      (q + 1) * (r + 1) * ((q - 1) * (r - 1)) = ((q + 1) * (q - 1)) * ((r + 1) * (r - 1)) := by
    ring
  rw [reorg]
  -- Product ≥ (2q+2)(2r+8) = 4qr + 16q + 4r + 16 > 4qr
  have h_product : ((q + 1) * (q - 1)) * ((r + 1) * (r - 1)) ≥ (2 * q + 2) * (2 * r + 8) :=
    Nat.mul_le_mul hq_lb hr_lb
  nlinarith

/-- Coprime 2^a and q*r for q, r odd primes -/
theorem coprime_2pow_qr {q r a : ℕ} (hq : q.Prime) (hr : r.Prime)
    (hq3 : q ≥ 3) (hr5 : r ≥ 5) :
    Nat.Coprime (2 ^ a) (q * r) := by
  have h2q : Nat.Coprime 2 q := by
    apply (Nat.coprime_primes (by decide : (2 : ℕ).Prime) hq).mpr
    omega
  have h2r : Nat.Coprime 2 r := by
    apply (Nat.coprime_primes (by decide : (2 : ℕ).Prime) hr).mpr
    omega
  have h2qr : Nat.Coprime 2 (q * r) := Nat.Coprime.mul_right h2q h2r
  exact h2qr.pow_left a

/-- Theorem B case 4c(ii): n = 2^a·q·r, a ≥ 2, q,r odd distinct primes → σφ ≠ nτ -/
theorem theorem_B_2pow_qr {q r a : ℕ}
    (hq : q.Prime) (hr : r.Prime) (hq3 : q ≥ 3) (hr5 : r ≥ 5) (hqr : q ≠ r) (ha : a ≥ 2) :
    σ 1 (2 ^ a * (q * r)) * Nat.totient (2 ^ a * (q * r)) ≠
      (2 ^ a * (q * r)) * (Nat.divisors (2 ^ a * (q * r))).card := by
  have hcop : Nat.Coprime (2 ^ a) (q * r) := coprime_2pow_qr hq hr hq3 hr5
  have hcop_qr : Nat.Coprime q r := coprime_of_distinct_primes hq hr hqr
  -- Multiplicative decomposition
  have hσ : σ 1 (2 ^ a * (q * r)) = σ 1 (2 ^ a) * σ 1 (q * r) :=
    (isMultiplicative_sigma (k := 1)).right hcop
  have hφ : Nat.totient (2 ^ a * (q * r)) =
            Nat.totient (2 ^ a) * Nat.totient (q * r) :=
    Nat.totient_mul hcop
  have hσ0 : σ 0 (2 ^ a * (q * r)) = σ 0 (2 ^ a) * σ 0 (q * r) :=
    (isMultiplicative_sigma (k := 0)).right hcop
  have hτ : (Nat.divisors (2 ^ a * (q * r))).card = (a + 1) * 4 := by
    have h_left : σ 0 (2 ^ a * (q * r)) = (Nat.divisors (2 ^ a * (q * r))).card := by
      simp [sigma_zero_apply]
    have h_qr : σ 0 (q * r) = 4 := by
      have h_qr_left : σ 0 (q * r) = (Nat.divisors (q * r)).card := by
        simp [sigma_zero_apply]
      rw [h_qr_left, tau_pq hq hr hcop_qr]
    rw [← h_left, hσ0, sigma_zero_apply_prime_pow Nat.prime_two, h_qr]
  rw [hσ, hφ, hτ]
  intro h
  -- h: σ₁(2^a) · σ₁(qr) · (φ(2^a)·φ(qr)) = 2^a·(qr)·(a+1)·4
  -- 재정렬: (σ₁(2^a)·φ(2^a)) · (σ₁(qr)·φ(qr)) = (2^a·(a+1)) · (qr·4)
  have reorg_lhs :
      σ 1 (2 ^ a) * σ 1 (q * r) * (Nat.totient (2 ^ a) * Nat.totient (q * r))
      = (σ 1 (2 ^ a) * Nat.totient (2 ^ a)) * (σ 1 (q * r) * Nat.totient (q * r)) := by
    ring
  have reorg_rhs :
      2 ^ a * (q * r) * ((a + 1) * 4) = (2 ^ a * (a + 1)) * ((q * r) * 4) := by
    ring
  rw [reorg_lhs, reorg_rhs] at h
  -- h: (σφ(2^a)) · (σφ(qr)) = (2^a·(a+1)) · (qr·4)
  have h1 : σ 1 (2 ^ a) * Nat.totient (2 ^ a) > 2 ^ a * (a + 1) := sigma_phi_2pow_strict ha
  have h2 : σ 1 (q * r) * Nat.totient (q * r) > (q * r) * 4 :=
    sigma_phi_qr_strict hq hr hq3 hr5 hqr
  -- 양의 정수의 곱: (> A)(> B) > AB
  have hA_pos : 2 ^ a * (a + 1) > 0 := by positivity
  have hB_pos : (q * r) * 4 > 0 := by positivity
  have h_product_gt :
      (σ 1 (2 ^ a) * Nat.totient (2 ^ a)) * (σ 1 (q * r) * Nat.totient (q * r))
        > (2 ^ a * (a + 1)) * ((q * r) * 4) := by
    calc (σ 1 (2 ^ a) * Nat.totient (2 ^ a)) * (σ 1 (q * r) * Nat.totient (q * r))
        > (2 ^ a * (a + 1)) * (σ 1 (q * r) * Nat.totient (q * r)) :=
          (Nat.mul_lt_mul_right (by positivity)).mpr h1
      _ > (2 ^ a * (a + 1)) * ((q * r) * 4) :=
          (Nat.mul_lt_mul_left hA_pos).mpr h2
  omega

-- 확인 예시
-- n = 60 = 4·3·5: σφ=168·16=2688, nτ=60·12=720, σφ > nτ ✓
-- n = 120 = 8·3·5: σφ=360·32=11520, nτ=120·16=1920 ✓

end N6Mathlib
