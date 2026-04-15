-- N6.TheoremB_Case4c_TwoPowQRST : Theorem B case 4c(v) (n = 2^a·q·r·s·t, a ≥ 2)
-- v4 M3_v4 case 4c(v) (2026-04-16 loop 35)
--
-- 목표: a ≥ 2, q,r,s,t ≥ 3,5,7,11 odd distinct primes → σφ(n) ≠ nτ(n)
--
-- 전략 (loop 13 2^a·qrs 확장):
--   σφ(2^a) > 2^a·(a+1) [loop 12]
--   σφ(qrst) > qrst·16 [새: case 4c(i) pqrs 의 strict 강화 + 4 odd primes]

import Mathlib.NumberTheory.ArithmeticFunction.Misc
import Mathlib.NumberTheory.Divisors
import Mathlib.Data.Nat.Totient
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Tactic.NormNum.Prime
import N6.TheoremB_PrimeCase
import N6.TheoremB_Case3_PrimePow
import N6.TheoremB_Case4_ThreePrimes
import N6.TheoremB_Case4c_FourPrimes
import N6.TheoremB_Case4c_TwoPowQR

namespace N6Mathlib

open Nat ArithmeticFunction
open scoped ArithmeticFunction.sigma

/-- Strict bound: σφ(qrst) > qrst·16 for 4 odd distinct primes ≥ 3,5,7,11 -/
theorem sigma_phi_qrst_strict {q r s t : ℕ}
    (hq : q.Prime) (hr : r.Prime) (hs : s.Prime) (ht : t.Prime)
    (hq3 : q ≥ 3) (hr5 : r ≥ 5) (hs7 : s ≥ 7) (ht11 : t ≥ 11)
    (hqr : q ≠ r) (hqs : q ≠ s) (hrs : r ≠ s)
    (hqt : q ≠ t) (hrt : r ≠ t) (hst : s ≠ t) :
    σ 1 (q * r * s * t) * Nat.totient (q * r * s * t) > (q * r * s * t) * 16 := by
  rw [sigma_one_pqrs hq hr hs ht hqr hqs hrs hqt hrt hst]
  rw [totient_pqrs hq hr hs ht hqr hqs hrs hqt hrt hst]
  rw [sigma_one_prime hq, sigma_one_prime hr, sigma_one_prime hs, sigma_one_prime ht]
  rw [Nat.totient_prime hq, Nat.totient_prime hr, Nat.totient_prime hs, Nat.totient_prime ht]
  -- (q+1)(r+1)(s+1)(t+1)·(q-1)(r-1)(s-1)(t-1) > qrst·16
  -- Bounds:
  -- (q+1)(q-1) ≥ 2q+2 (q≥3)
  -- (r+1)(r-1) ≥ 2r+8 (r≥5)
  -- (s+1)(s-1) ≥ 2s+12 (s≥7)
  -- (t+1)(t-1) ≥ 2t+20 (t≥11, from (t-1)² ≥ 100)
  have hq_lb : (q + 1) * (q - 1) ≥ 2 * q + 2 := by
    have hq1 : q - 1 ≥ 2 := by omega
    have h_eq : (q + 1) * (q - 1) = (q - 1) * (q - 1) + 2 * (q - 1) := by
      have h_q : q + 1 = (q - 1) + 2 := by omega
      rw [h_q]; ring
    rw [h_eq]
    have : (q - 1) * (q - 1) ≥ 4 := Nat.mul_le_mul hq1 hq1 |>.trans_eq (by ring)
    omega
  have hr_lb : (r + 1) * (r - 1) ≥ 2 * r + 8 := by
    have hr1 : r - 1 ≥ 4 := by omega
    have h_eq : (r + 1) * (r - 1) = (r - 1) * (r - 1) + 2 * (r - 1) := by
      have h_r : r + 1 = (r - 1) + 2 := by omega
      rw [h_r]; ring
    rw [h_eq]
    have : (r - 1) * (r - 1) ≥ 16 := Nat.mul_le_mul hr1 hr1 |>.trans_eq (by ring)
    omega
  have hs_lb : (s + 1) * (s - 1) ≥ 2 * s + 12 := by
    have hs1 : s - 1 ≥ 6 := by omega
    have h_eq : (s + 1) * (s - 1) = (s - 1) * (s - 1) + 2 * (s - 1) := by
      have h_s : s + 1 = (s - 1) + 2 := by omega
      rw [h_s]; ring
    rw [h_eq]
    have : (s - 1) * (s - 1) ≥ 36 := Nat.mul_le_mul hs1 hs1 |>.trans_eq (by ring)
    omega
  have ht_lb : (t + 1) * (t - 1) ≥ 2 * t + 20 := by
    have ht1 : t - 1 ≥ 10 := by omega
    have h_eq : (t + 1) * (t - 1) = (t - 1) * (t - 1) + 2 * (t - 1) := by
      have h_t : t + 1 = (t - 1) + 2 := by omega
      rw [h_t]; ring
    rw [h_eq]
    have : (t - 1) * (t - 1) ≥ 100 := Nat.mul_le_mul ht1 ht1 |>.trans_eq (by ring)
    omega
  have reorg :
      (q + 1) * (r + 1) * (s + 1) * (t + 1) * ((q - 1) * (r - 1) * (s - 1) * (t - 1))
      = ((q + 1) * (q - 1)) * ((r + 1) * (r - 1)) *
        ((s + 1) * (s - 1)) * ((t + 1) * (t - 1)) := by
    ring
  rw [reorg]
  have h_prod :
      ((q + 1) * (q - 1)) * ((r + 1) * (r - 1)) *
      ((s + 1) * (s - 1)) * ((t + 1) * (t - 1)) ≥
      (2 * q + 2) * (2 * r + 8) * (2 * s + 12) * (2 * t + 20) :=
    Nat.mul_le_mul (Nat.mul_le_mul (Nat.mul_le_mul hq_lb hr_lb) hs_lb) ht_lb
  -- (2q+2)(2r+8)(2s+12)(2t+20) > (2q)(2r)(2s)(2t) = 16qrst (곱한 strict positives)
  -- Step-by-step: each factor strictly > corresponding 2·prime.
  have h_qq : 2 * q + 2 > 2 * q := by omega
  have h_rr : 2 * r + 8 > 2 * r := by omega
  have h_ss : 2 * s + 12 > 2 * s := by omega
  have h_tt : 2 * t + 20 > 2 * t := by omega
  have h_2q : 2 * q ≥ 1 := by omega
  have h_2r : 2 * r ≥ 1 := by omega
  have h_2s : 2 * s ≥ 1 := by omega
  have h_2t : 2 * t ≥ 1 := by omega
  have h_strict_prod :
      (2 * q + 2) * (2 * r + 8) * (2 * s + 12) * (2 * t + 20) >
      (2 * q) * (2 * r) * (2 * s) * (2 * t) := by
    calc (2 * q + 2) * (2 * r + 8) * (2 * s + 12) * (2 * t + 20)
        > (2 * q) * (2 * r + 8) * (2 * s + 12) * (2 * t + 20) := by
          have h_pos : (2 * r + 8) * (2 * s + 12) * (2 * t + 20) > 0 := by positivity
          nlinarith
      _ ≥ (2 * q) * (2 * r) * (2 * s + 12) * (2 * t + 20) := by
          have h_mid : (2 * q) * (2 * r + 8) ≥ (2 * q) * (2 * r) :=
            Nat.mul_le_mul_left _ (by omega)
          have h_pos : (2 * s + 12) * (2 * t + 20) > 0 := by positivity
          nlinarith
      _ ≥ (2 * q) * (2 * r) * (2 * s) * (2 * t + 20) := by
          have h_mid : (2 * q) * (2 * r) * (2 * s + 12) ≥ (2 * q) * (2 * r) * (2 * s) :=
            Nat.mul_le_mul_left _ (by omega)
          have h_pos : 2 * t + 20 > 0 := by omega
          nlinarith
      _ ≥ (2 * q) * (2 * r) * (2 * s) * (2 * t) :=
          Nat.mul_le_mul_left _ (by omega)
  have h_equal : (2 * q) * (2 * r) * (2 * s) * (2 * t) = q * r * s * t * 16 := by ring
  omega

/-- Coprime 2^a and qrst for q,r,s,t odd primes -/
theorem coprime_2pow_qrst {q r s t a : ℕ}
    (hq : q.Prime) (hr : r.Prime) (hs : s.Prime) (ht : t.Prime)
    (hq3 : q ≥ 3) (hr5 : r ≥ 5) (hs7 : s ≥ 7) (ht11 : t ≥ 11) :
    Nat.Coprime (2 ^ a) (q * r * s * t) := by
  have h2q : Nat.Coprime 2 q := by
    apply (Nat.coprime_primes (by decide : (2 : ℕ).Prime) hq).mpr; omega
  have h2r : Nat.Coprime 2 r := by
    apply (Nat.coprime_primes (by decide : (2 : ℕ).Prime) hr).mpr; omega
  have h2s : Nat.Coprime 2 s := by
    apply (Nat.coprime_primes (by decide : (2 : ℕ).Prime) hs).mpr; omega
  have h2t : Nat.Coprime 2 t := by
    apply (Nat.coprime_primes (by decide : (2 : ℕ).Prime) ht).mpr; omega
  have h2qr : Nat.Coprime 2 (q * r) := Nat.Coprime.mul_right h2q h2r
  have h2qrs : Nat.Coprime 2 (q * r * s) := Nat.Coprime.mul_right h2qr h2s
  have h2qrst : Nat.Coprime 2 (q * r * s * t) := Nat.Coprime.mul_right h2qrs h2t
  exact h2qrst.pow_left a

/-- Theorem B case 4c(v): n = 2^a·q·r·s·t, a ≥ 2, q,r,s,t odd distinct → σφ ≠ nτ -/
theorem theorem_B_2pow_qrst {q r s t a : ℕ}
    (hq : q.Prime) (hr : r.Prime) (hs : s.Prime) (ht : t.Prime)
    (hq3 : q ≥ 3) (hr5 : r ≥ 5) (hs7 : s ≥ 7) (ht11 : t ≥ 11)
    (hqr : q ≠ r) (hqs : q ≠ s) (hrs : r ≠ s)
    (hqt : q ≠ t) (hrt : r ≠ t) (hst : s ≠ t)
    (ha : a ≥ 2) :
    σ 1 (2 ^ a * (q * r * s * t)) * Nat.totient (2 ^ a * (q * r * s * t)) ≠
      (2 ^ a * (q * r * s * t)) * (Nat.divisors (2 ^ a * (q * r * s * t))).card := by
  have hcop : Nat.Coprime (2 ^ a) (q * r * s * t) :=
    coprime_2pow_qrst hq hr hs ht hq3 hr5 hs7 ht11
  have hσ : σ 1 (2 ^ a * (q * r * s * t)) = σ 1 (2 ^ a) * σ 1 (q * r * s * t) :=
    (isMultiplicative_sigma (k := 1)).right hcop
  have hφ : Nat.totient (2 ^ a * (q * r * s * t)) =
            Nat.totient (2 ^ a) * Nat.totient (q * r * s * t) :=
    Nat.totient_mul hcop
  have hσ0 : σ 0 (2 ^ a * (q * r * s * t)) = σ 0 (2 ^ a) * σ 0 (q * r * s * t) :=
    (isMultiplicative_sigma (k := 0)).right hcop
  have hτ : (Nat.divisors (2 ^ a * (q * r * s * t))).card = (a + 1) * 16 := by
    have h_left : σ 0 (2 ^ a * (q * r * s * t)) = (Nat.divisors (2 ^ a * (q * r * s * t))).card := by
      simp [sigma_zero_apply]
    have h_qrst : σ 0 (q * r * s * t) = 16 := by
      have h_left2 : σ 0 (q * r * s * t) = (Nat.divisors (q * r * s * t)).card := by
        simp [sigma_zero_apply]
      rw [h_left2, tau_pqrs hq hr hs ht hqr hqs hrs hqt hrt hst]
    rw [← h_left, hσ0, sigma_zero_apply_prime_pow Nat.prime_two, h_qrst]
  rw [hσ, hφ, hτ]
  intro h
  have reorg_lhs :
      σ 1 (2 ^ a) * σ 1 (q * r * s * t) *
        (Nat.totient (2 ^ a) * Nat.totient (q * r * s * t))
      = (σ 1 (2 ^ a) * Nat.totient (2 ^ a)) *
        (σ 1 (q * r * s * t) * Nat.totient (q * r * s * t)) := by
    ring
  have reorg_rhs :
      2 ^ a * (q * r * s * t) * ((a + 1) * 16)
      = (2 ^ a * (a + 1)) * ((q * r * s * t) * 16) := by
    ring
  rw [reorg_lhs, reorg_rhs] at h
  have h1 : σ 1 (2 ^ a) * Nat.totient (2 ^ a) > 2 ^ a * (a + 1) := sigma_phi_2pow_strict ha
  have h2 : σ 1 (q * r * s * t) * Nat.totient (q * r * s * t) > (q * r * s * t) * 16 :=
    sigma_phi_qrst_strict hq hr hs ht hq3 hr5 hs7 ht11 hqr hqs hrs hqt hrt hst
  have hA_pos : 2 ^ a * (a + 1) > 0 := by positivity
  have h_product_gt :
      (σ 1 (2 ^ a) * Nat.totient (2 ^ a)) *
      (σ 1 (q * r * s * t) * Nat.totient (q * r * s * t))
        > (2 ^ a * (a + 1)) * ((q * r * s * t) * 16) := by
    calc (σ 1 (2 ^ a) * Nat.totient (2 ^ a)) *
         (σ 1 (q * r * s * t) * Nat.totient (q * r * s * t))
        > (2 ^ a * (a + 1)) *
          (σ 1 (q * r * s * t) * Nat.totient (q * r * s * t)) :=
          (Nat.mul_lt_mul_right (by positivity)).mpr h1
      _ > (2 ^ a * (a + 1)) * ((q * r * s * t) * 16) :=
          (Nat.mul_lt_mul_left hA_pos).mpr h2
  omega

-- 확인 예시
-- n = 4620 = 4·3·5·7·11: σ=6912·σ(4)=6912·7=48384... 실제 σ(4620) = σ(4)·σ(3)·σ(5)·σ(7)·σ(11) = 7·4·6·8·12 = 16128
-- σφ = 16128·960 = 15,482,880, nτ = 4620·48 = 221760 ✓ (매우 큰 비율)

end N6Mathlib
