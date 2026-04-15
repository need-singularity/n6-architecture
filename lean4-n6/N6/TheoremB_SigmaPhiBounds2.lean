-- N6.TheoremB_SigmaPhiBounds2 : σφ strict bounds for ω ≥ 7 squarefree
-- v4 M3_v4 helpers (2026-04-16 loop 51)
--
-- ω=7: σφ(pqrstuv) > pqrstuv·128 (loop 31 의 strict bound 추출)

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
import N6.TheoremB_Case4c_SevenPrimes
import N6.TheoremB_Case4c_TwoPowQPowR
import N6.TheoremB_SigmaPhiBounds

namespace N6Mathlib

open Nat ArithmeticFunction
open scoped ArithmeticFunction.sigma

set_option maxHeartbeats 800000

/-- σ(pqrstuv) = σ(pqrstu) · σ(v) for 7 distinct primes -/
theorem sigma_one_pqrstuv {p q r s t u v : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime)
    (ht : t.Prime) (hu : u.Prime) (hv : v.Prime)
    (hpq : p ≠ q) (hpr : p ≠ r) (hqr : q ≠ r)
    (hps : p ≠ s) (hqs : q ≠ s) (hrs : r ≠ s)
    (hpt : p ≠ t) (hqt : q ≠ t) (hrt : r ≠ t) (hst : s ≠ t)
    (hpu : p ≠ u) (hqu : q ≠ u) (hru : r ≠ u) (hsu : s ≠ u) (htu : t ≠ u)
    (hpv : p ≠ v) (hqv : q ≠ v) (hrv : r ≠ v) (hsv : s ≠ v) (htv : t ≠ v) (huv : u ≠ v) :
    σ 1 (p * q * r * s * t * u * v) =
      σ 1 (p * q * r * s * t * u) * σ 1 v := by
  have hcop : Nat.Coprime (p * q * r * s * t * u) v :=
    coprime_six_prod_prime hp hq hr hs ht hu hv hpv hqv hrv hsv htv huv
  exact (isMultiplicative_sigma (k := 1)).right hcop

/-- φ(pqrstuv) = φ(pqrstu) · φ(v) -/
theorem totient_pqrstuv_helper {p q r s t u v : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime)
    (ht : t.Prime) (hu : u.Prime) (hv : v.Prime)
    (hpv : p ≠ v) (hqv : q ≠ v) (hrv : r ≠ v) (hsv : s ≠ v) (htv : t ≠ v) (huv : u ≠ v) :
    Nat.totient (p * q * r * s * t * u * v) =
      Nat.totient (p * q * r * s * t * u) * Nat.totient v := by
  have hcop : Nat.Coprime (p * q * r * s * t * u) v :=
    coprime_six_prod_prime hp hq hr hs ht hu hv hpv hqv hrv hsv htv huv
  exact Nat.totient_mul hcop

/-- σφ(pqrstuv) > pqrstuv·128 STRICT for 7 distinct primes ≥ 2,3,5,7,11,13,17 -/
theorem sigma_phi_pqrstuv_strict {p q r s t u v : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime)
    (ht : t.Prime) (hu : u.Prime) (hv : v.Prime)
    (hp2 : p ≥ 2) (hq3 : q ≥ 3) (hr5 : r ≥ 5) (hs7 : s ≥ 7)
    (ht11 : t ≥ 11) (hu13 : u ≥ 13) (hv17 : v ≥ 17)
    (hpq : p ≠ q) (hpr : p ≠ r) (hqr : q ≠ r)
    (hps : p ≠ s) (hqs : q ≠ s) (hrs : r ≠ s)
    (hpt : p ≠ t) (hqt : q ≠ t) (hrt : r ≠ t) (hst : s ≠ t)
    (hpu : p ≠ u) (hqu : q ≠ u) (hru : r ≠ u) (hsu : s ≠ u) (htu : t ≠ u)
    (hpv : p ≠ v) (hqv : q ≠ v) (hrv : r ≠ v) (hsv : s ≠ v) (htv : t ≠ v) (huv : u ≠ v) :
    σ 1 (p * q * r * s * t * u * v) * Nat.totient (p * q * r * s * t * u * v) >
      (p * q * r * s * t * u * v) * 128 := by
  rw [sigma_one_pqrstuv hp hq hr hs ht hu hv hpq hpr hqr hps hqs hrs hpt hqt hrt hst
      hpu hqu hru hsu htu hpv hqv hrv hsv htv huv]
  rw [totient_pqrstuv_helper hp hq hr hs ht hu hv hpv hqv hrv hsv htv huv]
  have h1 : σ 1 (p * q * r * s * t * u) * Nat.totient (p * q * r * s * t * u) >
            (p * q * r * s * t * u) * 64 :=
    sigma_phi_pqrstu_strict hp hq hr hs ht hu hp2 hq3 hr5 hs7 ht11 hu13
      hpq hpr hqr hps hqs hrs hpt hqt hrt hst hpu hqu hru hsu htu
  have h2 : σ 1 v * Nat.totient v > v * 2 :=
    sigma_phi_odd_prime_strict hv (by omega : v ≥ 3)
  have reorg_lhs :
      σ 1 (p * q * r * s * t * u) * σ 1 v *
        (Nat.totient (p * q * r * s * t * u) * Nat.totient v)
      = (σ 1 (p * q * r * s * t * u) * Nat.totient (p * q * r * s * t * u)) *
        (σ 1 v * Nat.totient v) := by ring
  have reorg_rhs :
      p * q * r * s * t * u * v * 128 =
      (p * q * r * s * t * u) * 64 * (v * 2) := by ring
  rw [reorg_lhs, reorg_rhs]
  have hA_pos : (p * q * r * s * t * u) * 64 > 0 := by positivity
  calc (σ 1 (p * q * r * s * t * u) * Nat.totient (p * q * r * s * t * u)) *
       (σ 1 v * Nat.totient v)
      > (p * q * r * s * t * u) * 64 * (σ 1 v * Nat.totient v) :=
        (Nat.mul_lt_mul_right (by positivity)).mpr h1
    _ > (p * q * r * s * t * u) * 64 * (v * 2) :=
        (Nat.mul_lt_mul_left hA_pos).mpr h2

end N6Mathlib
