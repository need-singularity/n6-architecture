-- N6.TheoremB_Case4c_TwoPowersQRS : Theorem B case 4c(xv) (n = p^a·q^b·r·s, a,b ≥ 2)
-- v4 M3_v4 case 4c(xv) (2026-04-16 loop 44)
--
-- 목표: p, q, r, s distinct primes (p, q any ≥ 2; r, s odd ≥ 3 distinct), a, b ≥ 2
--       → σφ(p^a·q^b·r·s) ≠ nτ(n)
--
-- 전략 (3 strict bounds 곱):
--   σφ(p^a) > p^a·(a+1)     (sigma_phi_prime_pow_strict, loop 19)
--   σφ(q^b) > q^b·(b+1)
--   σφ(r·s) > rs·4           (sigma_phi_qr_strict, loop 12 — r,s odd distinct)
--   곱 strict: σφ(n) > p^a·q^b·rs·(a+1)(b+1)·4 = nτ(n)

import Mathlib.NumberTheory.ArithmeticFunction.Misc
import Mathlib.NumberTheory.Divisors
import Mathlib.Data.Nat.Totient
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Tactic.NormNum.Prime
import N6.TheoremB_PrimeCase
import N6.TheoremB_Case2_OddOdd
import N6.TheoremB_Case3_PrimePow
import N6.TheoremB_Case4c_TwoPowQR
import N6.TheoremB_Case4c_OddPowQR
import N6.TheoremB_Case4c_ThreePrimePowers

namespace N6Mathlib

open Nat ArithmeticFunction
open scoped ArithmeticFunction.sigma

set_option maxHeartbeats 400000

/-- Coprime (p^a·q^b) and (r·s) for distinct primes -/
theorem coprime_ppow_qpow_rs {p q r s a b : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime)
    (hpr : p ≠ r) (hps : p ≠ s) (hqr : q ≠ r) (hqs : q ≠ s) :
    Nat.Coprime (p ^ a * q ^ b) (r * s) := by
  have hpr_cop : Nat.Coprime p r := (Nat.coprime_primes hp hr).mpr hpr
  have hps_cop : Nat.Coprime p s := (Nat.coprime_primes hp hs).mpr hps
  have hqr_cop : Nat.Coprime q r := (Nat.coprime_primes hq hr).mpr hqr
  have hqs_cop : Nat.Coprime q s := (Nat.coprime_primes hq hs).mpr hqs
  have h_p_rs : Nat.Coprime p (r * s) := Nat.Coprime.mul_right hpr_cop hps_cop
  have h_q_rs : Nat.Coprime q (r * s) := Nat.Coprime.mul_right hqr_cop hqs_cop
  have h_pa_rs : Nat.Coprime (p ^ a) (r * s) := h_p_rs.pow_left a
  have h_qb_rs : Nat.Coprime (q ^ b) (r * s) := h_q_rs.pow_left b
  rw [Nat.Coprime, Nat.coprime_comm.mp
    (Nat.coprime_mul_iff_right.mpr ⟨h_pa_rs.symm, h_qb_rs.symm⟩)]

/-- Theorem B case 4c(xv): n = p^a·q^b·r·s, 4 distinct primes, a,b ≥ 2 → σφ ≠ nτ -/
theorem theorem_B_ppow_qpow_rs {p q r s a b : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime)
    (hp2 : p ≥ 2) (hq2 : q ≥ 2) (hr3 : r ≥ 3) (hs5 : s ≥ 5)
    (hpq : p ≠ q) (hpr : p ≠ r) (hps : p ≠ s)
    (hqr : q ≠ r) (hqs : q ≠ s) (hrs : r ≠ s)
    (ha : a ≥ 2) (hb : b ≥ 2) :
    σ 1 (p ^ a * q ^ b * (r * s)) * Nat.totient (p ^ a * q ^ b * (r * s)) ≠
      (p ^ a * q ^ b * (r * s)) *
        (Nat.divisors (p ^ a * q ^ b * (r * s))).card := by
  have hcop_main : Nat.Coprime (p ^ a * q ^ b) (r * s) :=
    coprime_ppow_qpow_rs hp hq hr hs hpr hps hqr hqs
  have hcop_pq : Nat.Coprime (p ^ a) (q ^ b) :=
    coprime_ppow_qpow_gen hp hq hpq
  have hσ : σ 1 (p ^ a * q ^ b * (r * s)) = σ 1 (p ^ a * q ^ b) * σ 1 (r * s) :=
    (isMultiplicative_sigma (k := 1)).right hcop_main
  have hσ_inner : σ 1 (p ^ a * q ^ b) = σ 1 (p ^ a) * σ 1 (q ^ b) :=
    (isMultiplicative_sigma (k := 1)).right hcop_pq
  have hφ : Nat.totient (p ^ a * q ^ b * (r * s)) =
            Nat.totient (p ^ a * q ^ b) * Nat.totient (r * s) :=
    Nat.totient_mul hcop_main
  have hφ_inner : Nat.totient (p ^ a * q ^ b) =
                  Nat.totient (p ^ a) * Nat.totient (q ^ b) :=
    Nat.totient_mul hcop_pq
  have hτ : (Nat.divisors (p ^ a * q ^ b * (r * s))).card = (a + 1) * (b + 1) * 4 := by
    have hσ0 : σ 0 (p ^ a * q ^ b * (r * s)) = σ 0 (p ^ a * q ^ b) * σ 0 (r * s) :=
      (isMultiplicative_sigma (k := 0)).right hcop_main
    have hσ0_inner : σ 0 (p ^ a * q ^ b) = σ 0 (p ^ a) * σ 0 (q ^ b) :=
      (isMultiplicative_sigma (k := 0)).right hcop_pq
    have h_left : σ 0 (p ^ a * q ^ b * (r * s)) =
                  (Nat.divisors (p ^ a * q ^ b * (r * s))).card := by
      simp [sigma_zero_apply]
    have h_rs : σ 0 (r * s) = 4 := by
      have h_left2 : σ 0 (r * s) = (Nat.divisors (r * s)).card := by
        simp [sigma_zero_apply]
      have hcop_rs : Nat.Coprime r s := coprime_of_distinct_primes hr hs hrs
      rw [h_left2, tau_pq hr hs hcop_rs]
    rw [← h_left, hσ0, hσ0_inner,
        sigma_zero_apply_prime_pow hp,
        sigma_zero_apply_prime_pow hq, h_rs]
  rw [hσ, hσ_inner, hφ, hφ_inner, hτ]
  intro h
  have reorg_lhs :
      σ 1 (p ^ a) * σ 1 (q ^ b) * σ 1 (r * s) *
        (Nat.totient (p ^ a) * Nat.totient (q ^ b) * Nat.totient (r * s))
      = (σ 1 (p ^ a) * Nat.totient (p ^ a)) *
        (σ 1 (q ^ b) * Nat.totient (q ^ b)) *
        (σ 1 (r * s) * Nat.totient (r * s)) := by ring
  have reorg_rhs :
      p ^ a * q ^ b * (r * s) * ((a + 1) * (b + 1) * 4)
      = (p ^ a * (a + 1)) * (q ^ b * (b + 1)) * ((r * s) * 4) := by ring
  rw [reorg_lhs, reorg_rhs] at h
  have h1 : σ 1 (p ^ a) * Nat.totient (p ^ a) > p ^ a * (a + 1) :=
    sigma_phi_prime_pow_strict hp hp2 ha
  have h2 : σ 1 (q ^ b) * Nat.totient (q ^ b) > q ^ b * (b + 1) :=
    sigma_phi_prime_pow_strict hq hq2 hb
  have h3 : σ 1 (r * s) * Nat.totient (r * s) > (r * s) * 4 :=
    sigma_phi_qr_strict hr hs hr3 hs5 hrs
  have hA_pos : p ^ a * (a + 1) > 0 := by positivity
  have hB_pos : q ^ b * (b + 1) > 0 := by positivity
  have h_prod_gt :
      (σ 1 (p ^ a) * Nat.totient (p ^ a)) *
      (σ 1 (q ^ b) * Nat.totient (q ^ b)) *
      (σ 1 (r * s) * Nat.totient (r * s)) >
      (p ^ a * (a + 1)) * (q ^ b * (b + 1)) * ((r * s) * 4) := by
    calc (σ 1 (p ^ a) * Nat.totient (p ^ a)) *
         (σ 1 (q ^ b) * Nat.totient (q ^ b)) *
         (σ 1 (r * s) * Nat.totient (r * s))
        > (p ^ a * (a + 1)) *
          (σ 1 (q ^ b) * Nat.totient (q ^ b)) *
          (σ 1 (r * s) * Nat.totient (r * s)) :=
          (Nat.mul_lt_mul_right (by positivity)).mpr
          ((Nat.mul_lt_mul_right (by positivity)).mpr h1)
      _ > (p ^ a * (a + 1)) * (q ^ b * (b + 1)) * (σ 1 (r * s) * Nat.totient (r * s)) :=
          (Nat.mul_lt_mul_right (by positivity)).mpr
          ((Nat.mul_lt_mul_left hA_pos).mpr h2)
      _ > (p ^ a * (a + 1)) * (q ^ b * (b + 1)) * ((r * s) * 4) :=
          (Nat.mul_lt_mul_left (by positivity)).mpr h3
  omega

end N6Mathlib
