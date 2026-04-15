-- N6.TheoremB_Case4c_SevenPrimes : Theorem B ω=7 (n = pqrstuv, 7 distinct primes)
-- v4 M3_v4 case 4c(xix) (2026-04-16 loop 50)
--
-- 전략 (split via multiplicativity):
--   σφ(pqrstuv) = σφ(pqrstu) · σφ(v)
--   σφ(pqrstu) > pqrstu·64 (sigma_phi_pqrstu_strict, loop 30)
--   σφ(v) > 2v (sigma_phi_odd_prime_strict, v ≥ 17 odd)
--   곱: σφ(pqrstuv) > pqrstu·64 · 2v = pqrstuv·128 = nτ STRICT

import Mathlib.NumberTheory.ArithmeticFunction.Misc
import Mathlib.NumberTheory.Divisors
import Mathlib.Data.Nat.Totient
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Tactic.NormNum.Prime
import N6.TheoremB_PrimeCase
import N6.TheoremB_Case4_ThreePrimes
import N6.TheoremB_Case4c_FourPrimes
import N6.TheoremB_Case4c_FivePrimes
import N6.TheoremB_Case4c_SixPrimes
import N6.TheoremB_Case4c_TwoPowQR
import N6.TheoremB_Case4c_TwoPowQPowR
import N6.TheoremB_SigmaPhiBounds

namespace N6Mathlib

open Nat ArithmeticFunction
open scoped ArithmeticFunction.sigma

set_option maxHeartbeats 800000

/-- Coprime (pqrstu) and v for 7 distinct primes -/
theorem coprime_six_prod_prime {p q r s t u v : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime)
    (ht : t.Prime) (hu : u.Prime) (hv : v.Prime)
    (hpv : p ≠ v) (hqv : q ≠ v) (hrv : r ≠ v) (hsv : s ≠ v) (htv : t ≠ v) (huv : u ≠ v) :
    Nat.Coprime (p * q * r * s * t * u) v := by
  have h1 : Nat.Coprime p v := coprime_of_distinct_primes hp hv hpv
  have h2 : Nat.Coprime q v := coprime_of_distinct_primes hq hv hqv
  have h3 : Nat.Coprime r v := coprime_of_distinct_primes hr hv hrv
  have h4 : Nat.Coprime s v := coprime_of_distinct_primes hs hv hsv
  have h5 : Nat.Coprime t v := coprime_of_distinct_primes ht hv htv
  have h6 : Nat.Coprime u v := coprime_of_distinct_primes hu hv huv
  have h12 : Nat.Coprime (p * q) v := by
    rw [Nat.Coprime, Nat.coprime_comm.mp (Nat.coprime_mul_iff_right.mpr ⟨h1.symm, h2.symm⟩)]
  have h123 : Nat.Coprime (p * q * r) v := by
    rw [Nat.Coprime, Nat.coprime_comm.mp (Nat.coprime_mul_iff_right.mpr ⟨h12.symm, h3.symm⟩)]
  have h1234 : Nat.Coprime (p * q * r * s) v := by
    rw [Nat.Coprime, Nat.coprime_comm.mp (Nat.coprime_mul_iff_right.mpr ⟨h123.symm, h4.symm⟩)]
  have h12345 : Nat.Coprime (p * q * r * s * t) v := by
    rw [Nat.Coprime, Nat.coprime_comm.mp (Nat.coprime_mul_iff_right.mpr ⟨h1234.symm, h5.symm⟩)]
  rw [Nat.Coprime, Nat.coprime_comm.mp (Nat.coprime_mul_iff_right.mpr ⟨h12345.symm, h6.symm⟩)]

/-- Theorem B ω=7 squarefree: 7 distinct primes → σφ ≠ nτ.
    split 접근: σφ(pqrstuv) = σφ(pqrstu) · σφ(v), 각각 strict bound 곱. -/
theorem theorem_B_seven_primes {p q r s t u v : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime)
    (ht : t.Prime) (hu : u.Prime) (hv : v.Prime)
    (hp2 : p ≥ 2) (hq3 : q ≥ 3) (hr5 : r ≥ 5) (hs7 : s ≥ 7)
    (ht11 : t ≥ 11) (hu13 : u ≥ 13) (hv17 : v ≥ 17)
    (hpq : p ≠ q) (hpr : p ≠ r) (hqr : q ≠ r)
    (hps : p ≠ s) (hqs : q ≠ s) (hrs : r ≠ s)
    (hpt : p ≠ t) (hqt : q ≠ t) (hrt : r ≠ t) (hst : s ≠ t)
    (hpu : p ≠ u) (hqu : q ≠ u) (hru : r ≠ u) (hsu : s ≠ u) (htu : t ≠ u)
    (hpv : p ≠ v) (hqv : q ≠ v) (hrv : r ≠ v) (hsv : s ≠ v) (htv : t ≠ v) (huv : u ≠ v) :
    σ 1 (p * q * r * s * t * u * v) * Nat.totient (p * q * r * s * t * u * v) ≠
      (p * q * r * s * t * u * v) * (Nat.divisors (p * q * r * s * t * u * v)).card := by
  have hcop : Nat.Coprime (p * q * r * s * t * u) v :=
    coprime_six_prod_prime hp hq hr hs ht hu hv hpv hqv hrv hsv htv huv
  have hσ : σ 1 (p * q * r * s * t * u * v) = σ 1 (p * q * r * s * t * u) * σ 1 v :=
    (isMultiplicative_sigma (k := 1)).right hcop
  have hφ : Nat.totient (p * q * r * s * t * u * v) =
            Nat.totient (p * q * r * s * t * u) * Nat.totient v :=
    Nat.totient_mul hcop
  have hσ0 : σ 0 (p * q * r * s * t * u * v) =
             σ 0 (p * q * r * s * t * u) * σ 0 v :=
    (isMultiplicative_sigma (k := 0)).right hcop
  have hτ : (Nat.divisors (p * q * r * s * t * u * v)).card = 128 := by
    have h_left : σ 0 (p * q * r * s * t * u * v) =
                  (Nat.divisors (p * q * r * s * t * u * v)).card := by
      simp [sigma_zero_apply]
    have h_pqrstu_tau : σ 0 (p * q * r * s * t * u) = 64 := by
      have h_left' : σ 0 (p * q * r * s * t * u) =
                    (Nat.divisors (p * q * r * s * t * u)).card := by
        simp [sigma_zero_apply]
      rw [h_left', tau_pqrstu hp hq hr hs ht hu hpq hpr hqr hps hqs hrs
          hpt hqt hrt hst hpu hqu hru hsu htu]
    have h_v : σ 0 v = 2 := by
      have := sigma_zero_apply_prime_pow (p := v) (i := 1) hv
      simp at this; exact this
    rw [← h_left, hσ0, h_pqrstu_tau, h_v]
  rw [hσ, hφ, hτ]
  intro h
  -- Reorganize: σ(pqrstu)·σ(v)·(φ(pqrstu)·φ(v)) = σφ(pqrstu)·σφ(v)
  have reorg_lhs :
      σ 1 (p * q * r * s * t * u) * σ 1 v *
        (Nat.totient (p * q * r * s * t * u) * Nat.totient v)
      = (σ 1 (p * q * r * s * t * u) * Nat.totient (p * q * r * s * t * u)) *
        (σ 1 v * Nat.totient v) := by ring
  have reorg_rhs :
      p * q * r * s * t * u * v * 128 =
      (p * q * r * s * t * u) * 64 * (v * 2) := by ring
  rw [reorg_lhs, reorg_rhs] at h
  -- Apply strict bounds
  have h1 : σ 1 (p * q * r * s * t * u) * Nat.totient (p * q * r * s * t * u) >
            (p * q * r * s * t * u) * 64 :=
    sigma_phi_pqrstu_strict hp hq hr hs ht hu hp2 hq3 hr5 hs7 ht11 hu13
      hpq hpr hqr hps hqs hrs hpt hqt hrt hst hpu hqu hru hsu htu
  have h2 : σ 1 v * Nat.totient v > v * 2 :=
    sigma_phi_odd_prime_strict hv (by omega : v ≥ 3)
  have hA_pos : (p * q * r * s * t * u) * 64 > 0 := by positivity
  have h_prod_gt :
      (σ 1 (p * q * r * s * t * u) * Nat.totient (p * q * r * s * t * u)) *
      (σ 1 v * Nat.totient v) >
      (p * q * r * s * t * u) * 64 * (v * 2) := by
    calc (σ 1 (p * q * r * s * t * u) * Nat.totient (p * q * r * s * t * u)) *
         (σ 1 v * Nat.totient v)
        > (p * q * r * s * t * u) * 64 * (σ 1 v * Nat.totient v) :=
          (Nat.mul_lt_mul_right (by positivity)).mpr h1
      _ > (p * q * r * s * t * u) * 64 * (v * 2) :=
          (Nat.mul_lt_mul_left hA_pos).mpr h2
  omega

end N6Mathlib
