-- N6.TheoremB_Case4c_FivePrimePowers : Theorem B case 4c(xviii) (n = p^a·q^b·r^c·s^d·t^e, all ≥ 2)
-- v4 M3_v4 case 4c(xviii) (2026-04-16 loop 48)
--
-- 목표: p,q,r,s,t 5 distinct primes ≥ 2, a,b,c,d,e ≥ 2
--       → σφ ≠ nτ
--
-- 전략: 5 generic strict bounds 곱 (sigma_phi_prime_pow_strict 5회)

import Mathlib.NumberTheory.ArithmeticFunction.Misc
import Mathlib.NumberTheory.Divisors
import Mathlib.Data.Nat.Totient
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Tactic.NormNum.Prime
import N6.TheoremB_PrimeCase
import N6.TheoremB_Case3_PrimePow
import N6.TheoremB_Case4c_FourPrimePowers
import N6.TheoremB_Case4c_OddPowQR

namespace N6Mathlib

open Nat ArithmeticFunction
open scoped ArithmeticFunction.sigma

set_option maxHeartbeats 800000

/-- Coprime (p^a·q^b·r^c·s^d) and t^e for 5 distinct primes -/
theorem coprime_four_prime_pows_t {p q r s t a b c d e : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime) (ht : t.Prime)
    (hpt : p ≠ t) (hqt : q ≠ t) (hrt : r ≠ t) (hst : s ≠ t) :
    Nat.Coprime (p ^ a * q ^ b * r ^ c * s ^ d) (t ^ e) := by
  have hpt_cop : Nat.Coprime p t := (Nat.coprime_primes hp ht).mpr hpt
  have hqt_cop : Nat.Coprime q t := (Nat.coprime_primes hq ht).mpr hqt
  have hrt_cop : Nat.Coprime r t := (Nat.coprime_primes hr ht).mpr hrt
  have hst_cop : Nat.Coprime s t := (Nat.coprime_primes hs ht).mpr hst
  have h_pa_te : Nat.Coprime (p ^ a) (t ^ e) := (hpt_cop.pow_left a).pow_right e
  have h_qb_te : Nat.Coprime (q ^ b) (t ^ e) := (hqt_cop.pow_left b).pow_right e
  have h_rc_te : Nat.Coprime (r ^ c) (t ^ e) := (hrt_cop.pow_left c).pow_right e
  have h_sd_te : Nat.Coprime (s ^ d) (t ^ e) := (hst_cop.pow_left d).pow_right e
  have h_pq_te : Nat.Coprime (p ^ a * q ^ b) (t ^ e) := by
    rw [Nat.Coprime, Nat.coprime_comm.mp
      (Nat.coprime_mul_iff_right.mpr ⟨h_pa_te.symm, h_qb_te.symm⟩)]
  have h_pqr_te : Nat.Coprime (p ^ a * q ^ b * r ^ c) (t ^ e) := by
    rw [Nat.Coprime, Nat.coprime_comm.mp
      (Nat.coprime_mul_iff_right.mpr ⟨h_pq_te.symm, h_rc_te.symm⟩)]
  rw [Nat.Coprime, Nat.coprime_comm.mp
    (Nat.coprime_mul_iff_right.mpr ⟨h_pqr_te.symm, h_sd_te.symm⟩)]

/-- Theorem B case 4c(xviii): 5 distinct primes, all powers ≥ 2 → σφ ≠ nτ -/
theorem theorem_B_five_prime_powers {p q r s t a b c d e : ℕ}
    (hp : p.Prime) (hq : q.Prime) (hr : r.Prime) (hs : s.Prime) (ht : t.Prime)
    (hp2 : p ≥ 2) (hq2 : q ≥ 2) (hr2 : r ≥ 2) (hs2 : s ≥ 2) (ht2 : t ≥ 2)
    (hpq : p ≠ q) (hpr : p ≠ r) (hps : p ≠ s) (hpt : p ≠ t)
    (hqr : q ≠ r) (hqs : q ≠ s) (hqt : q ≠ t)
    (hrs : r ≠ s) (hrt : r ≠ t) (hst : s ≠ t)
    (ha : a ≥ 2) (hb : b ≥ 2) (hc : c ≥ 2) (hd : d ≥ 2) (he : e ≥ 2) :
    σ 1 (p ^ a * q ^ b * r ^ c * s ^ d * t ^ e) *
      Nat.totient (p ^ a * q ^ b * r ^ c * s ^ d * t ^ e) ≠
    (p ^ a * q ^ b * r ^ c * s ^ d * t ^ e) *
      (Nat.divisors (p ^ a * q ^ b * r ^ c * s ^ d * t ^ e)).card := by
  have hcop_main : Nat.Coprime (p ^ a * q ^ b * r ^ c * s ^ d) (t ^ e) :=
    coprime_four_prime_pows_t hp hq hr hs ht hpt hqt hrt hst
  have hcop_4 : Nat.Coprime (p ^ a * q ^ b * r ^ c) (s ^ d) :=
    coprime_three_prime_pows_s hp hq hr hs hps hqs hrs
  have hcop_3 : Nat.Coprime (p ^ a * q ^ b) (r ^ c) :=
    coprime_ppow_qpow_rpow hp hq hr hpq hpr hqr
  have hcop_pq : Nat.Coprime (p ^ a) (q ^ b) :=
    coprime_ppow_qpow_gen hp hq hpq
  have hσ : σ 1 (p ^ a * q ^ b * r ^ c * s ^ d * t ^ e) =
            σ 1 (p ^ a * q ^ b * r ^ c * s ^ d) * σ 1 (t ^ e) :=
    (isMultiplicative_sigma (k := 1)).right hcop_main
  have hσ_4 : σ 1 (p ^ a * q ^ b * r ^ c * s ^ d) =
              σ 1 (p ^ a * q ^ b * r ^ c) * σ 1 (s ^ d) :=
    (isMultiplicative_sigma (k := 1)).right hcop_4
  have hσ_3 : σ 1 (p ^ a * q ^ b * r ^ c) =
              σ 1 (p ^ a * q ^ b) * σ 1 (r ^ c) :=
    (isMultiplicative_sigma (k := 1)).right hcop_3
  have hσ_pq : σ 1 (p ^ a * q ^ b) = σ 1 (p ^ a) * σ 1 (q ^ b) :=
    (isMultiplicative_sigma (k := 1)).right hcop_pq
  have hφ : Nat.totient (p ^ a * q ^ b * r ^ c * s ^ d * t ^ e) =
            Nat.totient (p ^ a * q ^ b * r ^ c * s ^ d) * Nat.totient (t ^ e) :=
    Nat.totient_mul hcop_main
  have hφ_4 : Nat.totient (p ^ a * q ^ b * r ^ c * s ^ d) =
              Nat.totient (p ^ a * q ^ b * r ^ c) * Nat.totient (s ^ d) :=
    Nat.totient_mul hcop_4
  have hφ_3 : Nat.totient (p ^ a * q ^ b * r ^ c) =
              Nat.totient (p ^ a * q ^ b) * Nat.totient (r ^ c) :=
    Nat.totient_mul hcop_3
  have hφ_pq : Nat.totient (p ^ a * q ^ b) =
               Nat.totient (p ^ a) * Nat.totient (q ^ b) :=
    Nat.totient_mul hcop_pq
  have hτ : (Nat.divisors (p ^ a * q ^ b * r ^ c * s ^ d * t ^ e)).card =
            (a + 1) * (b + 1) * (c + 1) * (d + 1) * (e + 1) := by
    have hσ0 : σ 0 (p ^ a * q ^ b * r ^ c * s ^ d * t ^ e) =
               σ 0 (p ^ a * q ^ b * r ^ c * s ^ d) * σ 0 (t ^ e) :=
      (isMultiplicative_sigma (k := 0)).right hcop_main
    have hσ0_4 : σ 0 (p ^ a * q ^ b * r ^ c * s ^ d) =
                 σ 0 (p ^ a * q ^ b * r ^ c) * σ 0 (s ^ d) :=
      (isMultiplicative_sigma (k := 0)).right hcop_4
    have hσ0_3 : σ 0 (p ^ a * q ^ b * r ^ c) =
                 σ 0 (p ^ a * q ^ b) * σ 0 (r ^ c) :=
      (isMultiplicative_sigma (k := 0)).right hcop_3
    have hσ0_pq : σ 0 (p ^ a * q ^ b) = σ 0 (p ^ a) * σ 0 (q ^ b) :=
      (isMultiplicative_sigma (k := 0)).right hcop_pq
    have h_left : σ 0 (p ^ a * q ^ b * r ^ c * s ^ d * t ^ e) =
                  (Nat.divisors (p ^ a * q ^ b * r ^ c * s ^ d * t ^ e)).card := by
      simp [sigma_zero_apply]
    rw [← h_left, hσ0, hσ0_4, hσ0_3, hσ0_pq,
        sigma_zero_apply_prime_pow hp,
        sigma_zero_apply_prime_pow hq,
        sigma_zero_apply_prime_pow hr,
        sigma_zero_apply_prime_pow hs,
        sigma_zero_apply_prime_pow ht]
  rw [hσ, hσ_4, hσ_3, hσ_pq, hφ, hφ_4, hφ_3, hφ_pq, hτ]
  intro h
  have reorg_lhs :
      σ 1 (p ^ a) * σ 1 (q ^ b) * σ 1 (r ^ c) * σ 1 (s ^ d) * σ 1 (t ^ e) *
        (Nat.totient (p ^ a) * Nat.totient (q ^ b) * Nat.totient (r ^ c) *
         Nat.totient (s ^ d) * Nat.totient (t ^ e))
      = (σ 1 (p ^ a) * Nat.totient (p ^ a)) *
        (σ 1 (q ^ b) * Nat.totient (q ^ b)) *
        (σ 1 (r ^ c) * Nat.totient (r ^ c)) *
        (σ 1 (s ^ d) * Nat.totient (s ^ d)) *
        (σ 1 (t ^ e) * Nat.totient (t ^ e)) := by ring
  have reorg_rhs :
      p ^ a * q ^ b * r ^ c * s ^ d * t ^ e *
        ((a + 1) * (b + 1) * (c + 1) * (d + 1) * (e + 1))
      = (p ^ a * (a + 1)) * (q ^ b * (b + 1)) *
        (r ^ c * (c + 1)) * (s ^ d * (d + 1)) * (t ^ e * (e + 1)) := by ring
  rw [reorg_lhs, reorg_rhs] at h
  have h1 : σ 1 (p ^ a) * Nat.totient (p ^ a) > p ^ a * (a + 1) :=
    sigma_phi_prime_pow_strict hp hp2 ha
  have h2 : σ 1 (q ^ b) * Nat.totient (q ^ b) > q ^ b * (b + 1) :=
    sigma_phi_prime_pow_strict hq hq2 hb
  have h3 : σ 1 (r ^ c) * Nat.totient (r ^ c) > r ^ c * (c + 1) :=
    sigma_phi_prime_pow_strict hr hr2 hc
  have h4 : σ 1 (s ^ d) * Nat.totient (s ^ d) > s ^ d * (d + 1) :=
    sigma_phi_prime_pow_strict hs hs2 hd
  have h5 : σ 1 (t ^ e) * Nat.totient (t ^ e) > t ^ e * (e + 1) :=
    sigma_phi_prime_pow_strict ht ht2 he
  have h_prod_gt :
      (σ 1 (p ^ a) * Nat.totient (p ^ a)) *
      (σ 1 (q ^ b) * Nat.totient (q ^ b)) *
      (σ 1 (r ^ c) * Nat.totient (r ^ c)) *
      (σ 1 (s ^ d) * Nat.totient (s ^ d)) *
      (σ 1 (t ^ e) * Nat.totient (t ^ e)) >
      (p ^ a * (a + 1)) * (q ^ b * (b + 1)) *
      (r ^ c * (c + 1)) * (s ^ d * (d + 1)) * (t ^ e * (e + 1)) := by
    calc (σ 1 (p ^ a) * Nat.totient (p ^ a)) *
         (σ 1 (q ^ b) * Nat.totient (q ^ b)) *
         (σ 1 (r ^ c) * Nat.totient (r ^ c)) *
         (σ 1 (s ^ d) * Nat.totient (s ^ d)) *
         (σ 1 (t ^ e) * Nat.totient (t ^ e))
        > (p ^ a * (a + 1)) *
          (σ 1 (q ^ b) * Nat.totient (q ^ b)) *
          (σ 1 (r ^ c) * Nat.totient (r ^ c)) *
          (σ 1 (s ^ d) * Nat.totient (s ^ d)) *
          (σ 1 (t ^ e) * Nat.totient (t ^ e)) :=
          (Nat.mul_lt_mul_right (by positivity)).mpr
          ((Nat.mul_lt_mul_right (by positivity)).mpr
          ((Nat.mul_lt_mul_right (by positivity)).mpr
          ((Nat.mul_lt_mul_right (by positivity)).mpr h1)))
      _ > (p ^ a * (a + 1)) * (q ^ b * (b + 1)) *
          (σ 1 (r ^ c) * Nat.totient (r ^ c)) *
          (σ 1 (s ^ d) * Nat.totient (s ^ d)) *
          (σ 1 (t ^ e) * Nat.totient (t ^ e)) :=
          (Nat.mul_lt_mul_right (by positivity)).mpr
          ((Nat.mul_lt_mul_right (by positivity)).mpr
          ((Nat.mul_lt_mul_right (by positivity)).mpr
          ((Nat.mul_lt_mul_left (by positivity)).mpr h2)))
      _ > (p ^ a * (a + 1)) * (q ^ b * (b + 1)) * (r ^ c * (c + 1)) *
          (σ 1 (s ^ d) * Nat.totient (s ^ d)) *
          (σ 1 (t ^ e) * Nat.totient (t ^ e)) :=
          (Nat.mul_lt_mul_right (by positivity)).mpr
          ((Nat.mul_lt_mul_right (by positivity)).mpr
          ((Nat.mul_lt_mul_left (by positivity)).mpr h3))
      _ > (p ^ a * (a + 1)) * (q ^ b * (b + 1)) * (r ^ c * (c + 1)) *
          (s ^ d * (d + 1)) *
          (σ 1 (t ^ e) * Nat.totient (t ^ e)) :=
          (Nat.mul_lt_mul_right (by positivity)).mpr
          ((Nat.mul_lt_mul_left (by positivity)).mpr h4)
      _ > (p ^ a * (a + 1)) * (q ^ b * (b + 1)) * (r ^ c * (c + 1)) *
          (s ^ d * (d + 1)) * (t ^ e * (e + 1)) :=
          (Nat.mul_lt_mul_left (by positivity)).mpr h5
  omega

end N6Mathlib
