-- N6.TheoremB_Case4c_SixPrimes : Theorem B case 4c(xiii) (n = pqrstu, 6 distinct primes)
-- v4 M3_v4 case 4c(xiii) (2026-04-16 loop 42)
--
-- 목표: 6 distinct primes p,q,r,s,t,u with p≥2, q≥3, r≥5, s≥7, t≥11, u≥13
--       → σφ(pqrstu) ≠ pqrstu·τ
--
-- 전략 (case 4c(iv) pqrst의 6-prime 확장):
--   ∏(pᵢ-1) ≥ 1·2·4·6·10·12 = 5760
--   ∏(pᵢ+1) > ∏pᵢ
--   σφ ≥ 5760·∏(pᵢ+1) > 5760·∏pᵢ > 64·∏pᵢ = nτ  [5760 > 64]

import Mathlib.NumberTheory.ArithmeticFunction.Misc
import Mathlib.NumberTheory.Divisors
import Mathlib.Data.Nat.Totient
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Tactic.NormNum.Prime
import N6.TheoremB_PrimeCase
import N6.TheoremB_Case4_ThreePrimes
import N6.TheoremB_Case4c_FourPrimes
import N6.TheoremB_Case4c_FivePrimes

namespace N6Mathlib

open Nat ArithmeticFunction
open scoped ArithmeticFunction.sigma

set_option maxHeartbeats 800000

/-- (pqrst) coprime with u for 6 distinct primes -/
theorem coprime_quint_prod_prime {p q r s t u : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime)
    (ht : t.Prime) (hu : u.Prime)
    (hpu : p ≠ u) (hqu : q ≠ u) (hru : r ≠ u) (hsu : s ≠ u) (htu : t ≠ u) :
    Nat.Coprime (p * q * r * s * t) u := by
  have h1 : Nat.Coprime p u := coprime_of_distinct_primes hp hu hpu
  have h2 : Nat.Coprime q u := coprime_of_distinct_primes hq hu hqu
  have h3 : Nat.Coprime r u := coprime_of_distinct_primes hr hu hru
  have h4 : Nat.Coprime s u := coprime_of_distinct_primes hs hu hsu
  have h5 : Nat.Coprime t u := coprime_of_distinct_primes ht hu htu
  have h12 : Nat.Coprime (p * q) u := by
    rw [Nat.Coprime, Nat.coprime_comm.mp (Nat.coprime_mul_iff_right.mpr ⟨h1.symm, h2.symm⟩)]
  have h123 : Nat.Coprime (p * q * r) u := by
    rw [Nat.Coprime, Nat.coprime_comm.mp (Nat.coprime_mul_iff_right.mpr ⟨h12.symm, h3.symm⟩)]
  have h1234 : Nat.Coprime (p * q * r * s) u := by
    rw [Nat.Coprime, Nat.coprime_comm.mp (Nat.coprime_mul_iff_right.mpr ⟨h123.symm, h4.symm⟩)]
  rw [Nat.Coprime, Nat.coprime_comm.mp (Nat.coprime_mul_iff_right.mpr ⟨h1234.symm, h5.symm⟩)]

/-- σ(pqrstu) = ∏ σ(pᵢ) -/
theorem sigma_one_pqrstu {p q r s t u : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime)
    (ht : t.Prime) (hu : u.Prime)
    (hpq : p ≠ q) (hpr : p ≠ r) (hqr : q ≠ r)
    (hps : p ≠ s) (hqs : q ≠ s) (hrs : r ≠ s)
    (hpt : p ≠ t) (hqt : q ≠ t) (hrt : r ≠ t) (hst : s ≠ t)
    (hpu : p ≠ u) (hqu : q ≠ u) (hru : r ≠ u) (hsu : s ≠ u) (htu : t ≠ u) :
    σ 1 (p * q * r * s * t * u) = σ 1 p * σ 1 q * σ 1 r * σ 1 s * σ 1 t * σ 1 u := by
  have hcop : Nat.Coprime (p * q * r * s * t) u :=
    coprime_quint_prod_prime hp hq hr hs ht hu hpu hqu hru hsu htu
  have h1 : σ 1 (p * q * r * s * t * u) = σ 1 (p * q * r * s * t) * σ 1 u :=
    (isMultiplicative_sigma (k := 1)).right hcop
  rw [h1, sigma_one_pqrst hp hq hr hs ht hpq hpr hqr hps hqs hrs hpt hqt hrt hst]

/-- φ(pqrstu) = ∏ φ(pᵢ) -/
theorem totient_pqrstu {p q r s t u : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime)
    (ht : t.Prime) (hu : u.Prime)
    (hpq : p ≠ q) (hpr : p ≠ r) (hqr : q ≠ r)
    (hps : p ≠ s) (hqs : q ≠ s) (hrs : r ≠ s)
    (hpt : p ≠ t) (hqt : q ≠ t) (hrt : r ≠ t) (hst : s ≠ t)
    (hpu : p ≠ u) (hqu : q ≠ u) (hru : r ≠ u) (hsu : s ≠ u) (htu : t ≠ u) :
    Nat.totient (p * q * r * s * t * u) =
      Nat.totient p * Nat.totient q * Nat.totient r * Nat.totient s *
      Nat.totient t * Nat.totient u := by
  have hcop : Nat.Coprime (p * q * r * s * t) u :=
    coprime_quint_prod_prime hp hq hr hs ht hu hpu hqu hru hsu htu
  rw [Nat.totient_mul hcop,
      totient_pqrst hp hq hr hs ht hpq hpr hqr hps hqs hrs hpt hqt hrt hst]

/-- τ(pqrstu) = 64 -/
theorem tau_pqrstu {p q r s t u : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime)
    (ht : t.Prime) (hu : u.Prime)
    (hpq : p ≠ q) (hpr : p ≠ r) (hqr : q ≠ r)
    (hps : p ≠ s) (hqs : q ≠ s) (hrs : r ≠ s)
    (hpt : p ≠ t) (hqt : q ≠ t) (hrt : r ≠ t) (hst : s ≠ t)
    (hpu : p ≠ u) (hqu : q ≠ u) (hru : r ≠ u) (hsu : s ≠ u) (htu : t ≠ u) :
    (Nat.divisors (p * q * r * s * t * u)).card = 64 := by
  have hcop : Nat.Coprime (p * q * r * s * t) u :=
    coprime_quint_prod_prime hp hq hr hs ht hu hpu hqu hru hsu htu
  have hσ0 : σ 0 (p * q * r * s * t * u) = σ 0 (p * q * r * s * t) * σ 0 u :=
    (isMultiplicative_sigma (k := 0)).right hcop
  have h_left : σ 0 (p * q * r * s * t * u) = (Nat.divisors (p * q * r * s * t * u)).card := by
    simp [sigma_zero_apply]
  have h_pqrst_tau : σ 0 (p * q * r * s * t) = 32 := by
    have h_left' : σ 0 (p * q * r * s * t) = (Nat.divisors (p * q * r * s * t)).card := by
      simp [sigma_zero_apply]
    rw [h_left', tau_pqrst hp hq hr hs ht hpq hpr hqr hps hqs hrs hpt hqt hrt hst]
  have h_u : σ 0 u = 2 := by
    have := sigma_zero_apply_prime_pow (p := u) (i := 1) hu
    simp at this; exact this
  rw [← h_left, hσ0, h_pqrst_tau, h_u]

/-- Theorem B case 4c(xiii): 6 distinct primes → σφ ≠ nτ -/
theorem theorem_B_six_primes {p q r s t u : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime)
    (ht : t.Prime) (hu : u.Prime)
    (hp2 : p ≥ 2) (hq3 : q ≥ 3) (hr5 : r ≥ 5) (hs7 : s ≥ 7)
    (ht11 : t ≥ 11) (hu13 : u ≥ 13)
    (hpq : p ≠ q) (hpr : p ≠ r) (hqr : q ≠ r)
    (hps : p ≠ s) (hqs : q ≠ s) (hrs : r ≠ s)
    (hpt : p ≠ t) (hqt : q ≠ t) (hrt : r ≠ t) (hst : s ≠ t)
    (hpu : p ≠ u) (hqu : q ≠ u) (hru : r ≠ u) (hsu : s ≠ u) (htu : t ≠ u) :
    σ 1 (p * q * r * s * t * u) * Nat.totient (p * q * r * s * t * u) ≠
      (p * q * r * s * t * u) * (Nat.divisors (p * q * r * s * t * u)).card := by
  rw [sigma_one_pqrstu hp hq hr hs ht hu hpq hpr hqr hps hqs hrs hpt hqt hrt hst hpu hqu hru hsu htu]
  rw [totient_pqrstu hp hq hr hs ht hu hpq hpr hqr hps hqs hrs hpt hqt hrt hst hpu hqu hru hsu htu]
  rw [tau_pqrstu hp hq hr hs ht hu hpq hpr hqr hps hqs hrs hpt hqt hrt hst hpu hqu hru hsu htu]
  rw [sigma_one_prime hp, sigma_one_prime hq, sigma_one_prime hr,
      sigma_one_prime hs, sigma_one_prime ht, sigma_one_prime hu]
  rw [Nat.totient_prime hp, Nat.totient_prime hq, Nat.totient_prime hr,
      Nat.totient_prime hs, Nat.totient_prime ht, Nat.totient_prime hu]
  intro h
  have hp1 : p - 1 ≥ 1 := by omega
  have hq1 : q - 1 ≥ 2 := by omega
  have hr1 : r - 1 ≥ 4 := by omega
  have hs1 : s - 1 ≥ 6 := by omega
  have ht1 : t - 1 ≥ 10 := by omega
  have hu1 : u - 1 ≥ 12 := by omega
  have h_sub6 : (p - 1) * (q - 1) * (r - 1) * (s - 1) * (t - 1) * (u - 1) ≥ 5760 := by
    have h_pq : (p - 1) * (q - 1) ≥ 2 := by
      calc (p - 1) * (q - 1) ≥ 1 * 2 := Nat.mul_le_mul hp1 hq1
        _ = 2 := by ring
    have h_pqr : (p - 1) * (q - 1) * (r - 1) ≥ 8 :=
      (Nat.mul_le_mul h_pq hr1).trans_eq (by ring)
    have h_pqrs : (p - 1) * (q - 1) * (r - 1) * (s - 1) ≥ 48 :=
      (Nat.mul_le_mul h_pqr hs1).trans_eq (by ring)
    have h_pqrst : (p - 1) * (q - 1) * (r - 1) * (s - 1) * (t - 1) ≥ 480 :=
      (Nat.mul_le_mul h_pqrs ht1).trans_eq (by ring)
    calc (p - 1) * (q - 1) * (r - 1) * (s - 1) * (t - 1) * (u - 1) ≥ 480 * 12 :=
        Nat.mul_le_mul h_pqrst hu1
      _ = 5760 := by ring
  -- (p+1)(q+1)(r+1)(s+1)(t+1)(u+1) > pqrstu
  have h_add : (p + 1) * (q + 1) * (r + 1) * (s + 1) * (t + 1) * (u + 1) > p * q * r * s * t * u := by
    have h_pq_pos : p * q > 0 := by positivity
    have h_rs_pos : r * s > 0 := by positivity
    have h_tu_pos : t * u > 0 := by positivity
    have h_prod1_pos : p * q * (r * s) > 0 := by positivity
    have h_pq_gt : (p + 1) * (q + 1) > p * q := by nlinarith
    have h_rs_gt : (r + 1) * (s + 1) > r * s := by nlinarith
    have h_tu_gt : (t + 1) * (u + 1) > t * u := by nlinarith
    calc (p + 1) * (q + 1) * (r + 1) * (s + 1) * (t + 1) * (u + 1)
        = ((p + 1) * (q + 1)) * ((r + 1) * (s + 1)) * ((t + 1) * (u + 1)) := by ring
      _ > (p * q) * ((r + 1) * (s + 1)) * ((t + 1) * (u + 1)) :=
          (Nat.mul_lt_mul_right (by positivity)).mpr
          ((Nat.mul_lt_mul_right (by positivity)).mpr h_pq_gt)
      _ > (p * q) * (r * s) * ((t + 1) * (u + 1)) :=
          (Nat.mul_lt_mul_right (by positivity)).mpr
          ((Nat.mul_lt_mul_left h_pq_pos).mpr h_rs_gt)
      _ > (p * q) * (r * s) * (t * u) :=
          (Nat.mul_lt_mul_left h_prod1_pos).mpr h_tu_gt
      _ = p * q * r * s * t * u := by ring
  -- σφ ≥ 5760·(p+1)…(u+1) > 5760·pqrstu > 64·pqrstu = nτ
  have lhs_bound :
      (p + 1) * (q + 1) * (r + 1) * (s + 1) * (t + 1) * (u + 1) *
        ((p - 1) * (q - 1) * (r - 1) * (s - 1) * (t - 1) * (u - 1)) ≥
      (p + 1) * (q + 1) * (r + 1) * (s + 1) * (t + 1) * (u + 1) * 5760 :=
    Nat.mul_le_mul_left _ h_sub6
  have rhs_bound :
      (p + 1) * (q + 1) * (r + 1) * (s + 1) * (t + 1) * (u + 1) * 5760 >
      p * q * r * s * t * u * 5760 :=
    (Nat.mul_lt_mul_right (by norm_num)).mpr h_add
  have h_5760_64 : p * q * r * s * t * u * 5760 > p * q * r * s * t * u * 64 := by
    have h_pos : p * q * r * s * t * u > 0 := by positivity
    nlinarith
  omega

-- 확인 예시
-- n = 30030 = 2·3·5·7·11·13: σ=82944, φ=5760, τ=64
-- σφ = 477,757,440, nτ = 1,921,920, 비율 249× ✓

end N6Mathlib
