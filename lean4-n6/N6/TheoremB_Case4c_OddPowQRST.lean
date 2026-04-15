-- N6.TheoremB_Case4c_OddPowQRST : Theorem B case 4c(xvi) (n = p^a·q·r·s·t, p odd, ω=5)
-- v4 M3_v4 case 4c(xvi) (2026-04-16 loop 45)
--
-- 목표: p odd ≥ 3, a ≥ 2, q,r,s,t 4 distinct odd primes ≠ p → σφ(p^a·q·r·s·t) ≠ nτ
--
-- 전략:
--   σφ(p^a) > p^a(a+1) strict (sigma_phi_prime_pow_strict)
--   σφ(qrst) > qrst·16 strict (sigma_phi_qrst_strict, loop 15)
--   곱: σφ(n) > p^a(a+1)·qrst·16 = nτ(n) STRICT

import Mathlib.NumberTheory.ArithmeticFunction.Misc
import Mathlib.NumberTheory.Divisors
import Mathlib.Data.Nat.Totient
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Tactic.NormNum.Prime
import N6.TheoremB_PrimeCase
import N6.TheoremB_Case3_PrimePow
import N6.TheoremB_Case4c_FourPrimes
import N6.TheoremB_Case4c_TwoPowQRST
import N6.TheoremB_Case4c_OddPowQR

namespace N6Mathlib

open Nat ArithmeticFunction
open scoped ArithmeticFunction.sigma

set_option maxHeartbeats 400000

/-- Coprime p^a and (q·r·s·t) for distinct primes -/
theorem coprime_ppow_qrst {p q r s t a : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime) (ht : t.Prime)
    (hpq : p ≠ q) (hpr : p ≠ r) (hps : p ≠ s) (hpt : p ≠ t) :
    Nat.Coprime (p ^ a) (q * r * s * t) := by
  have hpq_cop : Nat.Coprime p q := (Nat.coprime_primes hp hq).mpr hpq
  have hpr_cop : Nat.Coprime p r := (Nat.coprime_primes hp hr).mpr hpr
  have hps_cop : Nat.Coprime p s := (Nat.coprime_primes hp hs).mpr hps
  have hpt_cop : Nat.Coprime p t := (Nat.coprime_primes hp ht).mpr hpt
  have h_pq : Nat.Coprime p (q * r) := Nat.Coprime.mul_right hpq_cop hpr_cop
  have h_pqrs : Nat.Coprime p (q * r * s) := Nat.Coprime.mul_right h_pq hps_cop
  have h_pqrst : Nat.Coprime p (q * r * s * t) := Nat.Coprime.mul_right h_pqrs hpt_cop
  exact h_pqrst.pow_left a

/-- Theorem B case 4c(xvi): p^a·q·r·s·t, p odd, a ≥ 2 → σφ ≠ nτ -/
theorem theorem_B_odd_ppow_qrst {p q r s t a : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime) (ht : t.Prime)
    (hp3 : p ≥ 3) (hq3 : q ≥ 3) (hr5 : r ≥ 5) (hs7 : s ≥ 7) (ht11 : t ≥ 11)
    (hpq : p ≠ q) (hpr : p ≠ r) (hps : p ≠ s) (hpt : p ≠ t)
    (hqr : q ≠ r) (hqs : q ≠ s) (hrs : r ≠ s)
    (hqt : q ≠ t) (hrt : r ≠ t) (hst : s ≠ t)
    (ha : a ≥ 2) :
    σ 1 (p ^ a * (q * r * s * t)) * Nat.totient (p ^ a * (q * r * s * t)) ≠
      (p ^ a * (q * r * s * t)) *
        (Nat.divisors (p ^ a * (q * r * s * t))).card := by
  have hcop : Nat.Coprime (p ^ a) (q * r * s * t) :=
    coprime_ppow_qrst hp hq hr hs ht hpq hpr hps hpt
  have hσ : σ 1 (p ^ a * (q * r * s * t)) = σ 1 (p ^ a) * σ 1 (q * r * s * t) :=
    (isMultiplicative_sigma (k := 1)).right hcop
  have hφ : Nat.totient (p ^ a * (q * r * s * t)) =
            Nat.totient (p ^ a) * Nat.totient (q * r * s * t) :=
    Nat.totient_mul hcop
  have hσ0 : σ 0 (p ^ a * (q * r * s * t)) = σ 0 (p ^ a) * σ 0 (q * r * s * t) :=
    (isMultiplicative_sigma (k := 0)).right hcop
  have hτ : (Nat.divisors (p ^ a * (q * r * s * t))).card = (a + 1) * 16 := by
    have h_left : σ 0 (p ^ a * (q * r * s * t)) =
                  (Nat.divisors (p ^ a * (q * r * s * t))).card := by
      simp [sigma_zero_apply]
    have h_qrst : σ 0 (q * r * s * t) = 16 := by
      have h_left2 : σ 0 (q * r * s * t) = (Nat.divisors (q * r * s * t)).card := by
        simp [sigma_zero_apply]
      rw [h_left2, tau_pqrs hq hr hs ht hqr hqs hrs hqt hrt hst]
    rw [← h_left, hσ0, sigma_zero_apply_prime_pow hp, h_qrst]
  rw [hσ, hφ, hτ]
  intro h
  have reorg_lhs :
      σ 1 (p ^ a) * σ 1 (q * r * s * t) *
        (Nat.totient (p ^ a) * Nat.totient (q * r * s * t))
      = (σ 1 (p ^ a) * Nat.totient (p ^ a)) *
        (σ 1 (q * r * s * t) * Nat.totient (q * r * s * t)) := by ring
  have reorg_rhs :
      p ^ a * (q * r * s * t) * ((a + 1) * 16) =
      (p ^ a * (a + 1)) * ((q * r * s * t) * 16) := by ring
  rw [reorg_lhs, reorg_rhs] at h
  have h1 : σ 1 (p ^ a) * Nat.totient (p ^ a) > p ^ a * (a + 1) :=
    sigma_phi_prime_pow_strict hp (by omega : p ≥ 2) ha
  have h2 : σ 1 (q * r * s * t) * Nat.totient (q * r * s * t) > (q * r * s * t) * 16 :=
    sigma_phi_qrst_strict hq hr hs ht hq3 hr5 hs7 ht11 hqr hqs hrs hqt hrt hst
  have hA_pos : p ^ a * (a + 1) > 0 := by positivity
  have h_prod_gt :
      (σ 1 (p ^ a) * Nat.totient (p ^ a)) * (σ 1 (q * r * s * t) * Nat.totient (q * r * s * t)) >
      (p ^ a * (a + 1)) * ((q * r * s * t) * 16) := by
    calc (σ 1 (p ^ a) * Nat.totient (p ^ a)) *
         (σ 1 (q * r * s * t) * Nat.totient (q * r * s * t))
        > (p ^ a * (a + 1)) * (σ 1 (q * r * s * t) * Nat.totient (q * r * s * t)) :=
          (Nat.mul_lt_mul_right (by positivity)).mpr h1
      _ > (p ^ a * (a + 1)) * ((q * r * s * t) * 16) :=
          (Nat.mul_lt_mul_left hA_pos).mpr h2
  omega

end N6Mathlib
