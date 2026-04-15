-- N6.TheoremB_Case4c_EightPrimes : Theorem B ω=8 squarefree (n = pqrstuvw, 8 distinct)
-- v4 M3_v4 case 4c(xx) (2026-04-16 loop 52)
--
-- 전략: split via multiplicativity
--   σφ(pqrstuvw) = σφ(pqrstuv) · σφ(w)
--   σφ(pqrstuv) > pqrstuv·128 (SigmaPhiBounds2)
--   σφ(w) > 2w (w ≥ 19 odd)
--   곱: σφ > pqrstuvw·256 = nτ STRICT

import Mathlib.NumberTheory.ArithmeticFunction.Misc
import Mathlib.NumberTheory.Divisors
import Mathlib.Data.Nat.Totient
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Tactic.NormNum.Prime
import N6.TheoremB_PrimeCase
import N6.TheoremB_Case4_ThreePrimes
import N6.TheoremB_Case4c_FourPrimes
import N6.TheoremB_Case4c_SixPrimes
import N6.TheoremB_Case4c_SevenPrimes
import N6.TheoremB_Case4c_TwoPowQPowR
import N6.TheoremB_SigmaPhiBounds2

namespace N6Mathlib

open Nat ArithmeticFunction
open scoped ArithmeticFunction.sigma

set_option maxHeartbeats 800000

/-- (pqrstuv) coprime with w for 8 distinct primes -/
theorem coprime_seven_prod_prime {p q r s t u v w : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime)
    (ht : t.Prime) (hu : u.Prime) (hv : v.Prime) (hw : w.Prime)
    (hpw : p ≠ w) (hqw : q ≠ w) (hrw : r ≠ w) (hsw : s ≠ w)
    (htw : t ≠ w) (huw : u ≠ w) (hvw : v ≠ w) :
    Nat.Coprime (p * q * r * s * t * u * v) w := by
  have h1 : Nat.Coprime p w := coprime_of_distinct_primes hp hw hpw
  have h2 : Nat.Coprime q w := coprime_of_distinct_primes hq hw hqw
  have h3 : Nat.Coprime r w := coprime_of_distinct_primes hr hw hrw
  have h4 : Nat.Coprime s w := coprime_of_distinct_primes hs hw hsw
  have h5 : Nat.Coprime t w := coprime_of_distinct_primes ht hw htw
  have h6 : Nat.Coprime u w := coprime_of_distinct_primes hu hw huw
  have h7 : Nat.Coprime v w := coprime_of_distinct_primes hv hw hvw
  have h12 : Nat.Coprime (p * q) w := by
    rw [Nat.Coprime, Nat.coprime_comm.mp (Nat.coprime_mul_iff_right.mpr ⟨h1.symm, h2.symm⟩)]
  have h123 : Nat.Coprime (p * q * r) w := by
    rw [Nat.Coprime, Nat.coprime_comm.mp (Nat.coprime_mul_iff_right.mpr ⟨h12.symm, h3.symm⟩)]
  have h1234 : Nat.Coprime (p * q * r * s) w := by
    rw [Nat.Coprime, Nat.coprime_comm.mp (Nat.coprime_mul_iff_right.mpr ⟨h123.symm, h4.symm⟩)]
  have h12345 : Nat.Coprime (p * q * r * s * t) w := by
    rw [Nat.Coprime, Nat.coprime_comm.mp (Nat.coprime_mul_iff_right.mpr ⟨h1234.symm, h5.symm⟩)]
  have h123456 : Nat.Coprime (p * q * r * s * t * u) w := by
    rw [Nat.Coprime, Nat.coprime_comm.mp (Nat.coprime_mul_iff_right.mpr ⟨h12345.symm, h6.symm⟩)]
  rw [Nat.Coprime, Nat.coprime_comm.mp (Nat.coprime_mul_iff_right.mpr ⟨h123456.symm, h7.symm⟩)]

/-- Theorem B ω=8 squarefree: 8 distinct primes → σφ ≠ nτ -/
theorem theorem_B_eight_primes {p q r s t u v w : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime)
    (ht : t.Prime) (hu : u.Prime) (hv : v.Prime) (hw : w.Prime)
    (hp2 : p ≥ 2) (hq3 : q ≥ 3) (hr5 : r ≥ 5) (hs7 : s ≥ 7)
    (ht11 : t ≥ 11) (hu13 : u ≥ 13) (hv17 : v ≥ 17) (hw19 : w ≥ 19)
    (hpq : p ≠ q) (hpr : p ≠ r) (hqr : q ≠ r)
    (hps : p ≠ s) (hqs : q ≠ s) (hrs : r ≠ s)
    (hpt : p ≠ t) (hqt : q ≠ t) (hrt : r ≠ t) (hst : s ≠ t)
    (hpu : p ≠ u) (hqu : q ≠ u) (hru : r ≠ u) (hsu : s ≠ u) (htu : t ≠ u)
    (hpv : p ≠ v) (hqv : q ≠ v) (hrv : r ≠ v) (hsv : s ≠ v) (htv : t ≠ v) (huv : u ≠ v)
    (hpw : p ≠ w) (hqw : q ≠ w) (hrw : r ≠ w) (hsw : s ≠ w)
    (htw : t ≠ w) (huw : u ≠ w) (hvw : v ≠ w) :
    σ 1 (p * q * r * s * t * u * v * w) *
      Nat.totient (p * q * r * s * t * u * v * w) ≠
    (p * q * r * s * t * u * v * w) *
      (Nat.divisors (p * q * r * s * t * u * v * w)).card := by
  have hcop : Nat.Coprime (p * q * r * s * t * u * v) w :=
    coprime_seven_prod_prime hp hq hr hs ht hu hv hw hpw hqw hrw hsw htw huw hvw
  have hσ : σ 1 (p * q * r * s * t * u * v * w) =
            σ 1 (p * q * r * s * t * u * v) * σ 1 w :=
    (isMultiplicative_sigma (k := 1)).right hcop
  have hφ : Nat.totient (p * q * r * s * t * u * v * w) =
            Nat.totient (p * q * r * s * t * u * v) * Nat.totient w :=
    Nat.totient_mul hcop
  have hσ0 : σ 0 (p * q * r * s * t * u * v * w) =
             σ 0 (p * q * r * s * t * u * v) * σ 0 w :=
    (isMultiplicative_sigma (k := 0)).right hcop
  have hτ : (Nat.divisors (p * q * r * s * t * u * v * w)).card = 256 := by
    have h_left : σ 0 (p * q * r * s * t * u * v * w) =
                  (Nat.divisors (p * q * r * s * t * u * v * w)).card := by
      simp [sigma_zero_apply]
    have h_inner : σ 0 (p * q * r * s * t * u * v) = 128 := by
      have hcop' : Nat.Coprime (p * q * r * s * t * u) v :=
        coprime_six_prod_prime hp hq hr hs ht hu hv hpv hqv hrv hsv htv huv
      have : σ 0 (p * q * r * s * t * u * v) =
             σ 0 (p * q * r * s * t * u) * σ 0 v :=
        (isMultiplicative_sigma (k := 0)).right hcop'
      rw [this]
      have h_pqrstu_tau : σ 0 (p * q * r * s * t * u) = 64 := by
        have h_left2 : σ 0 (p * q * r * s * t * u) =
                      (Nat.divisors (p * q * r * s * t * u)).card := by
          simp [sigma_zero_apply]
        rw [h_left2, tau_pqrstu hp hq hr hs ht hu hpq hpr hqr hps hqs hrs
            hpt hqt hrt hst hpu hqu hru hsu htu]
      have h_v : σ 0 v = 2 := by
        have := sigma_zero_apply_prime_pow (p := v) (i := 1) hv
        simp at this; exact this
      rw [h_pqrstu_tau, h_v]
    have h_w : σ 0 w = 2 := by
      have := sigma_zero_apply_prime_pow (p := w) (i := 1) hw
      simp at this; exact this
    rw [← h_left, hσ0, h_inner, h_w]
  rw [hσ, hφ, hτ]
  intro h
  have reorg_lhs :
      σ 1 (p * q * r * s * t * u * v) * σ 1 w *
        (Nat.totient (p * q * r * s * t * u * v) * Nat.totient w)
      = (σ 1 (p * q * r * s * t * u * v) *
         Nat.totient (p * q * r * s * t * u * v)) *
        (σ 1 w * Nat.totient w) := by ring
  have reorg_rhs :
      p * q * r * s * t * u * v * w * 256 =
      (p * q * r * s * t * u * v) * 128 * (w * 2) := by ring
  rw [reorg_lhs, reorg_rhs] at h
  have h1 : σ 1 (p * q * r * s * t * u * v) *
            Nat.totient (p * q * r * s * t * u * v) >
            (p * q * r * s * t * u * v) * 128 :=
    sigma_phi_pqrstuv_strict hp hq hr hs ht hu hv hp2 hq3 hr5 hs7 ht11 hu13 hv17
      hpq hpr hqr hps hqs hrs hpt hqt hrt hst hpu hqu hru hsu htu
      hpv hqv hrv hsv htv huv
  have h2 : σ 1 w * Nat.totient w > w * 2 :=
    sigma_phi_odd_prime_strict hw (by omega : w ≥ 3)
  have hA_pos : (p * q * r * s * t * u * v) * 128 > 0 := by positivity
  have h_prod_gt :
      (σ 1 (p * q * r * s * t * u * v) *
       Nat.totient (p * q * r * s * t * u * v)) *
      (σ 1 w * Nat.totient w) >
      (p * q * r * s * t * u * v) * 128 * (w * 2) := by
    calc (σ 1 (p * q * r * s * t * u * v) *
          Nat.totient (p * q * r * s * t * u * v)) *
         (σ 1 w * Nat.totient w)
        > (p * q * r * s * t * u * v) * 128 * (σ 1 w * Nat.totient w) :=
          (Nat.mul_lt_mul_right (by positivity)).mpr h1
      _ > (p * q * r * s * t * u * v) * 128 * (w * 2) :=
          (Nat.mul_lt_mul_left hA_pos).mpr h2
  omega

end N6Mathlib
