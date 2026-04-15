-- N6.TheoremB_Case4c_OddPowQPowR : Theorem B case 4c(xi) (n = p^a·q^b·r, all odd)
-- v4 M3_v4 case 4c(xi) (2026-04-16 loop 40)
--
-- 목표: p, q, r odd prime ≥ 3 distinct, a ≥ 1, b ≥ 1 → σφ(p^a·q^b·r) ≠ nτ
--
-- (a=b=1 overlap with case 4a, allowed redundancy.)
--
-- 전략 (27·σφ ≥ 64·nτ → σφ > nτ):
--   3·σφ(p^a) ≥ 4·p^a·(a+1)   (loop 18 odd weak, equality only at (3,1))
--   3·σφ(q^b) ≥ 4·q^b·(b+1)
--   3·σφ(r)   ≥ 4·r·2 = 8r
--   곱: 27·σφ(n) ≥ 128·p^a·q^b·r·(a+1)(b+1) = 64·nτ(n)  (nτ = 2·p^a·q^b·r·(a+1)(b+1))
--   σφ = nτ 이면 27·nτ ≥ 64·nτ → 27 ≥ 64 모순.

import Mathlib.NumberTheory.ArithmeticFunction.Misc
import Mathlib.NumberTheory.Divisors
import Mathlib.Data.Nat.Totient
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Tactic.NormNum.Prime
import N6.TheoremB_PrimeCase
import N6.TheoremB_Case2_OddOdd
import N6.TheoremB_Case4c_TwoQPowR

namespace N6Mathlib

open Nat ArithmeticFunction
open scoped ArithmeticFunction.sigma

/-- Coprime p^a · q^b for p, q distinct primes -/
theorem coprime_ppow_qpow_distinct {p q a b : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hpq : p ≠ q) :
    Nat.Coprime (p ^ a) (q ^ b) := by
  have hpq_cop : Nat.Coprime p q := (Nat.coprime_primes hp hq).mpr hpq
  exact (hpq_cop.pow_left a).pow_right b

/-- Coprime (p^a · q^b) and r for distinct primes -/
theorem coprime_ppow_qpow_r_distinct {p q r a b : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime)
    (hpr : p ≠ r) (hqr : q ≠ r) :
    Nat.Coprime (p ^ a * q ^ b) r := by
  have hpr_cop : Nat.Coprime p r := (Nat.coprime_primes hp hr).mpr hpr
  have hqr_cop : Nat.Coprime q r := (Nat.coprime_primes hq hr).mpr hqr
  have hpa : Nat.Coprime (p ^ a) r := hpr_cop.pow_left a
  have hqb : Nat.Coprime (q ^ b) r := hqr_cop.pow_left b
  rw [Nat.Coprime, Nat.coprime_comm.mp
    (Nat.coprime_mul_iff_right.mpr ⟨hpa.symm, hqb.symm⟩)]

/-- Theorem B case 4c(xi): n = p^a·q^b·r, 3 odd distinct primes → σφ ≠ nτ.
    (a=b=1 overlaps case 4a, allowed) -/
theorem theorem_B_odd_ppow_qpow_r {p q r a b : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime)
    (hp3 : p ≥ 3) (hq3 : q ≥ 3) (hr3 : r ≥ 3)
    (hpq : p ≠ q) (hpr : p ≠ r) (hqr : q ≠ r)
    (ha : a ≥ 1) (hb : b ≥ 1) :
    σ 1 (p ^ a * q ^ b * r) * Nat.totient (p ^ a * q ^ b * r) ≠
      (p ^ a * q ^ b * r) * (Nat.divisors (p ^ a * q ^ b * r)).card := by
  have hcop_main : Nat.Coprime (p ^ a * q ^ b) r :=
    coprime_ppow_qpow_r_distinct hp hq hr hpr hqr
  have hcop_pq : Nat.Coprime (p ^ a) (q ^ b) :=
    coprime_ppow_qpow_distinct hp hq hpq
  have hσ : σ 1 (p ^ a * q ^ b * r) = σ 1 (p ^ a * q ^ b) * σ 1 r :=
    (isMultiplicative_sigma (k := 1)).right hcop_main
  have hσ_inner : σ 1 (p ^ a * q ^ b) = σ 1 (p ^ a) * σ 1 (q ^ b) :=
    (isMultiplicative_sigma (k := 1)).right hcop_pq
  have hφ : Nat.totient (p ^ a * q ^ b * r) =
            Nat.totient (p ^ a * q ^ b) * Nat.totient r :=
    Nat.totient_mul hcop_main
  have hφ_inner : Nat.totient (p ^ a * q ^ b) =
                  Nat.totient (p ^ a) * Nat.totient (q ^ b) :=
    Nat.totient_mul hcop_pq
  have hτ : (Nat.divisors (p ^ a * q ^ b * r)).card = (a + 1) * (b + 1) * 2 := by
    have hσ0 : σ 0 (p ^ a * q ^ b * r) = σ 0 (p ^ a * q ^ b) * σ 0 r :=
      (isMultiplicative_sigma (k := 0)).right hcop_main
    have hσ0_inner : σ 0 (p ^ a * q ^ b) = σ 0 (p ^ a) * σ 0 (q ^ b) :=
      (isMultiplicative_sigma (k := 0)).right hcop_pq
    have h_left : σ 0 (p ^ a * q ^ b * r) = (Nat.divisors (p ^ a * q ^ b * r)).card := by
      simp [sigma_zero_apply]
    have h_r : σ 0 r = 2 := by
      have := sigma_zero_apply_prime_pow (p := r) (i := 1) hr
      simp at this; exact this
    rw [← h_left, hσ0, hσ0_inner,
        sigma_zero_apply_prime_pow hp,
        sigma_zero_apply_prime_pow hq, h_r]
  rw [hσ, hσ_inner, hφ, hφ_inner, hτ]
  intro h
  -- Reorganize h
  have h_eq :
      (σ 1 (p ^ a) * Nat.totient (p ^ a)) *
      (σ 1 (q ^ b) * Nat.totient (q ^ b)) *
      (σ 1 r * Nat.totient r) =
      p ^ a * (a + 1) * (q ^ b * (b + 1)) * (r * 2) := by
    have h_reorg_lhs :
      (σ 1 (p ^ a) * Nat.totient (p ^ a)) *
      (σ 1 (q ^ b) * Nat.totient (q ^ b)) *
      (σ 1 r * Nat.totient r) =
      σ 1 (p ^ a) * σ 1 (q ^ b) * σ 1 r *
        (Nat.totient (p ^ a) * Nat.totient (q ^ b) * Nat.totient r) := by ring
    have h_reorg_rhs :
      p ^ a * q ^ b * r * ((a + 1) * (b + 1) * 2) =
      p ^ a * (a + 1) * (q ^ b * (b + 1)) * (r * 2) := by ring
    rw [h_reorg_lhs, h]; ring
  -- Weak bounds
  have hb1 : b ≥ 1 := hb
  have h_p : 3 * (σ 1 (p ^ a) * Nat.totient (p ^ a)) ≥ 4 * (p ^ a * (a + 1)) :=
    sigma_phi_odd_pow_ge_four_thirds hp hp3 ha
  have h_q : 3 * (σ 1 (q ^ b) * Nat.totient (q ^ b)) ≥ 4 * (q ^ b * (b + 1)) :=
    sigma_phi_odd_pow_ge_four_thirds hq hq3 hb
  have h_r : 3 * (σ 1 r * Nat.totient r) ≥ 4 * (r * 2) :=
    sigma_phi_odd_prime_ge_four_thirds hr hr3
  -- Product: 27·σφ(p^a)·σφ(q^b)·σφ(r) ≥ 128·p^a·q^b·r·(a+1)(b+1)
  have h_prod :
      27 * ((σ 1 (p ^ a) * Nat.totient (p ^ a)) *
            (σ 1 (q ^ b) * Nat.totient (q ^ b)) *
            (σ 1 r * Nat.totient r)) ≥
      128 * (p ^ a * q ^ b * r * ((a + 1) * (b + 1))) := by
    have h_arith : 27 * ((σ 1 (p ^ a) * Nat.totient (p ^ a)) *
                         (σ 1 (q ^ b) * Nat.totient (q ^ b)) *
                         (σ 1 r * Nat.totient r)) =
                   (3 * (σ 1 (p ^ a) * Nat.totient (p ^ a))) *
                   (3 * (σ 1 (q ^ b) * Nat.totient (q ^ b))) *
                   (3 * (σ 1 r * Nat.totient r)) := by ring
    rw [h_arith]
    have h_bound_arith :
        (4 * (p ^ a * (a + 1))) * (4 * (q ^ b * (b + 1))) * (4 * (r * 2)) =
        128 * (p ^ a * q ^ b * r * ((a + 1) * (b + 1))) := by ring
    rw [← h_bound_arith]
    exact Nat.mul_le_mul (Nat.mul_le_mul h_p h_q) h_r
  -- h_eq 에 27 곱: 27·σφ(n) = 27·p^a·q^b·r·2·(a+1)(b+1) = 54·p^a·q^b·r·(a+1)(b+1)
  -- Wait: h_eq = p^a·(a+1)·(q^b·(b+1))·(r·2) = 2·p^a·q^b·r·(a+1)(b+1)
  -- 27·2 = 54, so 27·σφ = 54·p^a·q^b·r·(a+1)(b+1)
  -- h_prod: 27·σφ ≥ 128·p^a·q^b·r·(a+1)(b+1).
  -- So 54·... ≥ 128·... Thus 54 ≥ 128 if p^a·q^b·r·(a+1)(b+1) > 0.
  have h_times_27 :
      27 * ((σ 1 (p ^ a) * Nat.totient (p ^ a)) *
            (σ 1 (q ^ b) * Nat.totient (q ^ b)) *
            (σ 1 r * Nat.totient r)) =
      54 * (p ^ a * q ^ b * r * ((a + 1) * (b + 1))) := by
    have : 27 * ((σ 1 (p ^ a) * Nat.totient (p ^ a)) *
                 (σ 1 (q ^ b) * Nat.totient (q ^ b)) *
                 (σ 1 r * Nat.totient r)) =
           27 * (p ^ a * (a + 1) * (q ^ b * (b + 1)) * (r * 2)) := by
      rw [h_eq]
    rw [this]; ring
  have h_pos : p ^ a * q ^ b * r * ((a + 1) * (b + 1)) > 0 := by positivity
  omega

-- 확인 예시
-- n = 3²·5·7 = 315: (case 4c(ix) 도 커버)
-- n = 3·5²·7 = 525: 대체
-- n = 3²·5²·7 = 1575: σ=13·31·8=3224, φ=6·20·6=720. σφ=2321280. τ=3·3·2=18. nτ=28350. 비율 81.9 ✓

end N6Mathlib
