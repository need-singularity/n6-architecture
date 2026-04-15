-- N6.TheoremB_Case4c_FourPrimes : Theorem B case 4c(i) (n = pqrs, 4 distinct primes)
-- v4 M3_v4 case 4c(i) (2026-04-16 loop 30)
--
-- 목표: p, q, r, s distinct primes 에서 σ(pqrs)·φ(pqrs) ≠ pqrs·τ(pqrs)
--
-- 전략 (case 4a 의 pqr 패턴 확장):
--   σ(pqrs) = (p+1)(q+1)(r+1)(s+1), φ = (p-1)(q-1)(r-1)(s-1), τ = 16
--   p ≥ 2, q ≥ 3, r ≥ 5, s ≥ 7  (distinct primes 최소값)
--   (p-1)(q-1)(r-1)(s-1) ≥ 1·2·4·6 = 48
--   (p+1)(q+1)(r+1)(s+1) > pqrs  (확장 적용)
--   σφ ≥ 48·(p+1)(q+1)(r+1)(s+1) > 48·pqrs > 16·pqrs = nτ

import Mathlib.NumberTheory.ArithmeticFunction.Misc
import Mathlib.NumberTheory.Divisors
import Mathlib.Data.Nat.Totient
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Tactic.NormNum.Prime
import N6.TheoremB_PrimeCase
import N6.TheoremB_Case4_ThreePrimes

namespace N6Mathlib

open Nat ArithmeticFunction
open scoped ArithmeticFunction.sigma

/-- (p*q*r) coprime with s for pairwise distinct primes -/
theorem coprime_triple_prod_prime {p q r s : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime)
    (hps : p ≠ s) (hqs : q ≠ s) (hrs : r ≠ s) :
    Nat.Coprime (p * q * r) s := by
  have h1 : Nat.Coprime p s := coprime_of_distinct_primes hp hs hps
  have h2 : Nat.Coprime q s := coprime_of_distinct_primes hq hs hqs
  have h3 : Nat.Coprime r s := coprime_of_distinct_primes hr hs hrs
  have : Nat.Coprime (p * q) s := by
    rw [Nat.Coprime, Nat.coprime_comm.mp (Nat.coprime_mul_iff_right.mpr ⟨h1.symm, h2.symm⟩)]
  rw [Nat.Coprime, Nat.coprime_comm.mp (Nat.coprime_mul_iff_right.mpr
    ⟨this.symm, h3.symm⟩)]

/-- σ(pqrs) = σ(p)·σ(q)·σ(r)·σ(s) for pairwise distinct primes -/
theorem sigma_one_pqrs {p q r s : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime)
    (hpq : p ≠ q) (hpr : p ≠ r) (hqr : q ≠ r)
    (hps : p ≠ s) (hqs : q ≠ s) (hrs : r ≠ s) :
    σ 1 (p * q * r * s) = σ 1 p * σ 1 q * σ 1 r * σ 1 s := by
  have hpqrs_cop : Nat.Coprime (p * q * r) s :=
    coprime_triple_prod_prime hp hq hr hs hps hqs hrs
  have h1 : σ 1 (p * q * r * s) = σ 1 (p * q * r) * σ 1 s :=
    (isMultiplicative_sigma (k := 1)).right hpqrs_cop
  rw [h1, sigma_one_pqr hp hq hr hpq hpr hqr]

/-- φ(pqrs) = φ(p)·φ(q)·φ(r)·φ(s) -/
theorem totient_pqrs {p q r s : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime)
    (hpq : p ≠ q) (hpr : p ≠ r) (hqr : q ≠ r)
    (hps : p ≠ s) (hqs : q ≠ s) (hrs : r ≠ s) :
    Nat.totient (p * q * r * s) =
      Nat.totient p * Nat.totient q * Nat.totient r * Nat.totient s := by
  have hpqrs_cop : Nat.Coprime (p * q * r) s :=
    coprime_triple_prod_prime hp hq hr hs hps hqs hrs
  rw [Nat.totient_mul hpqrs_cop, totient_pqr hp hq hr hpq hpr hqr]

/-- τ(pqrs) = 16 -/
theorem tau_pqrs {p q r s : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime)
    (hpq : p ≠ q) (hpr : p ≠ r) (hqr : q ≠ r)
    (hps : p ≠ s) (hqs : q ≠ s) (hrs : r ≠ s) :
    (Nat.divisors (p * q * r * s)).card = 16 := by
  have hpqrs_cop : Nat.Coprime (p * q * r) s :=
    coprime_triple_prod_prime hp hq hr hs hps hqs hrs
  have hσ0 : σ 0 (p * q * r * s) = σ 0 (p * q * r) * σ 0 s :=
    (isMultiplicative_sigma (k := 0)).right hpqrs_cop
  have h_left : σ 0 (p * q * r * s) = (Nat.divisors (p * q * r * s)).card := by
    simp [sigma_zero_apply]
  have h_pqr_tau : σ 0 (p * q * r) = 8 := by
    have h_pqr_left : σ 0 (p * q * r) = (Nat.divisors (p * q * r)).card := by
      simp [sigma_zero_apply]
    rw [h_pqr_left, tau_pqr hp hq hr hpq hpr hqr]
  have h_s : σ 0 s = 2 := by
    have := sigma_zero_apply_prime_pow (p := s) (i := 1) hs
    simp at this; exact this
  rw [← h_left, hσ0, h_pqr_tau, h_s]

/-- Theorem B case 4c(i): 4 distinct primes → σφ ≠ nτ -/
theorem theorem_B_four_primes {p q r s : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime)
    (hp2 : p ≥ 2) (hq3 : q ≥ 3) (hr5 : r ≥ 5) (hs7 : s ≥ 7)
    (hpq : p ≠ q) (hpr : p ≠ r) (hqr : q ≠ r)
    (hps : p ≠ s) (hqs : q ≠ s) (hrs : r ≠ s) :
    σ 1 (p * q * r * s) * Nat.totient (p * q * r * s) ≠
      (p * q * r * s) * (Nat.divisors (p * q * r * s)).card := by
  rw [sigma_one_pqrs hp hq hr hs hpq hpr hqr hps hqs hrs]
  rw [totient_pqrs hp hq hr hs hpq hpr hqr hps hqs hrs]
  rw [tau_pqrs hp hq hr hs hpq hpr hqr hps hqs hrs]
  rw [sigma_one_prime hp, sigma_one_prime hq, sigma_one_prime hr, sigma_one_prime hs]
  rw [Nat.totient_prime hp, Nat.totient_prime hq, Nat.totient_prime hr, Nat.totient_prime hs]
  -- Goal: (p+1)(q+1)(r+1)(s+1) * ((p-1)(q-1)(r-1)(s-1)) ≠ p*q*r*s * 16
  intro h
  -- (p-1)(q-1)(r-1)(s-1) ≥ 48
  have hp1 : p - 1 ≥ 1 := by omega
  have hq1 : q - 1 ≥ 2 := by omega
  have hr1 : r - 1 ≥ 4 := by omega
  have hs1 : s - 1 ≥ 6 := by omega
  have h_sub4 : (p - 1) * (q - 1) * (r - 1) * (s - 1) ≥ 48 := by
    have h_pq : (p - 1) * (q - 1) ≥ 2 := by
      calc (p - 1) * (q - 1) ≥ 1 * 2 := Nat.mul_le_mul hp1 hq1
        _ = 2 := by ring
    have h_pqr : (p - 1) * (q - 1) * (r - 1) ≥ 8 := by
      calc (p - 1) * (q - 1) * (r - 1) ≥ 2 * 4 :=
          Nat.mul_le_mul h_pq hr1
        _ = 8 := by ring
    calc (p - 1) * (q - 1) * (r - 1) * (s - 1) ≥ 8 * 6 := Nat.mul_le_mul h_pqr hs1
      _ = 48 := by ring
  -- (p+1)(q+1)(r+1)(s+1) > pqrs
  have hp3' : p + 1 ≥ 3 := by omega
  have hq4 : q + 1 ≥ 4 := by omega
  have hr6 : r + 1 ≥ 6 := by omega
  have hs8 : s + 1 ≥ 8 := by omega
  have h_add : (p + 1) * (q + 1) * (r + 1) * (s + 1) > p * q * r * s := by
    -- (p+1)(q+1) > pq and (r+1)(s+1) > rs, multiply
    have h_pq_pos : p * q > 0 := by positivity
    have h_rs_pos : r * s > 0 := by positivity
    have h_pq_gt : (p + 1) * (q + 1) > p * q := by nlinarith
    have h_rs_gt : (r + 1) * (s + 1) > r * s := by nlinarith
    calc (p + 1) * (q + 1) * (r + 1) * (s + 1)
        = ((p + 1) * (q + 1)) * ((r + 1) * (s + 1)) := by ring
      _ > (p * q) * ((r + 1) * (s + 1)) := by
          exact (Nat.mul_lt_mul_right (by nlinarith : (r + 1) * (s + 1) > 0)).mpr h_pq_gt
      _ > (p * q) * (r * s) := by
          exact (Nat.mul_lt_mul_left h_pq_pos).mpr h_rs_gt
      _ = p * q * r * s := by ring
  -- σφ ≥ 48·(p+1)(q+1)(r+1)(s+1) > 48·pqrs > 16·pqrs = nτ
  have lhs_bound :
      (p + 1) * (q + 1) * (r + 1) * (s + 1) * ((p - 1) * (q - 1) * (r - 1) * (s - 1)) ≥
        (p + 1) * (q + 1) * (r + 1) * (s + 1) * 48 :=
    Nat.mul_le_mul_left ((p + 1) * (q + 1) * (r + 1) * (s + 1)) h_sub4
  have rhs_bound :
      (p + 1) * (q + 1) * (r + 1) * (s + 1) * 48 > p * q * r * s * 48 := by
    have h48 : (48 : ℕ) > 0 := by norm_num
    exact (Nat.mul_lt_mul_right h48).mpr h_add
  -- p*q*r*s*48 > p*q*r*s*16
  have h_48_16 : p * q * r * s * 48 > p * q * r * s * 16 := by
    have h_pos : p * q * r * s > 0 := by positivity
    nlinarith
  omega

/-- 확인 예시 -/
-- n = 210 = 2·3·5·7: σ=576, φ=48, τ=16, σφ=27648, nτ=3360
example : Nat.divisors (2 * 3 * 5 * 7) = Nat.divisors 210 := by decide

end N6Mathlib
