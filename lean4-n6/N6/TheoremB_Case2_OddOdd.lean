-- N6.TheoremB_Case2_OddOdd : Theorem B case 2b (p·q, p, q 모두 odd prime distinct)
-- v4 M3_v4 case 2b (2026-04-16 loop 24)
--
-- 목표: p, q ≥ 3 odd distinct primes 에 대해 σ(pq)·φ(pq) ≠ pq·τ(pq)
-- 전략:
--   σ(pq) = (p+1)(q+1), φ(pq) = (p-1)(q-1), τ(pq) = 4
--   σ·φ = (p²-1)(q²-1), nτ = 4pq
--   등식 ⟺ (pq-1)² = (p+q)² → pq-1 = p+q (both positive in our range)
--   → (p-1)(q-1) = 2
--   p ≥ 3, q ≥ 3 → (p-1)(q-1) ≥ 4 > 2, 모순
-- 따라서 모든 (p, q) odd prime distinct 에서 불성립

import Mathlib.NumberTheory.ArithmeticFunction.Misc
import Mathlib.NumberTheory.Divisors
import Mathlib.Data.Nat.Totient
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Tactic.NormNum.Prime
import N6.TheoremB_PrimeCase
import N6.TheoremB_Case2_P2

namespace N6Mathlib

open Nat ArithmeticFunction
open scoped ArithmeticFunction.sigma

/-- p, q odd distinct primes → coprime -/
theorem coprime_odd_distinct_primes {p q : ℕ} (hp : p.Prime) (hq : q.Prime)
    (hp3 : p ≥ 3) (hq3 : q ≥ 3) (hpq : p ≠ q) : Nat.Coprime p q :=
  (Nat.coprime_primes hp hq).mpr hpq

/-- σ(pq) = (p+1)(q+1) for coprime primes p, q -/
theorem sigma_one_pq {p q : ℕ} (hp : p.Prime) (hq : q.Prime) (hcop : Nat.Coprime p q) :
    σ 1 (p * q) = (p + 1) * (q + 1) := by
  have h_mul : σ 1 (p * q) = σ 1 p * σ 1 q := (isMultiplicative_sigma (k := 1)).right hcop
  rw [h_mul, sigma_one_prime hp, sigma_one_prime hq]

/-- φ(pq) = (p-1)(q-1) for coprime primes -/
theorem totient_pq {p q : ℕ} (hp : p.Prime) (hq : q.Prime) (hcop : Nat.Coprime p q) :
    Nat.totient (p * q) = (p - 1) * (q - 1) := by
  rw [Nat.totient_mul hcop, Nat.totient_prime hp, Nat.totient_prime hq]

/-- τ(pq) = 4 for coprime primes -/
theorem tau_pq {p q : ℕ} (hp : p.Prime) (hq : q.Prime) (hcop : Nat.Coprime p q) :
    (Nat.divisors (p * q)).card = 4 := by
  have h_mul : σ 0 (p * q) = σ 0 p * σ 0 q := (isMultiplicative_sigma (k := 0)).right hcop
  have h_left : σ 0 (p * q) = (Nat.divisors (p * q)).card := by simp [sigma_zero_apply]
  have h_p : σ 0 p = 2 := by
    have := sigma_zero_apply_prime_pow (p := p) (i := 1) hp
    simp at this; exact this
  have h_q : σ 0 q = 2 := by
    have := sigma_zero_apply_prime_pow (p := q) (i := 1) hq
    simp at this; exact this
  rw [← h_left, h_mul, h_p, h_q]

/-- Theorem B case 2b: p, q odd distinct primes → σ(pq)·φ(pq) ≠ pq·τ(pq) -/
theorem theorem_B_odd_pq {p q : ℕ} (hp : p.Prime) (hq : q.Prime)
    (hp3 : p ≥ 3) (hq3 : q ≥ 3) (hpq : p ≠ q) :
    σ 1 (p * q) * Nat.totient (p * q) ≠ (p * q) * (Nat.divisors (p * q)).card := by
  have hcop : Nat.Coprime p q := coprime_odd_distinct_primes hp hq hp3 hq3 hpq
  rw [sigma_one_pq hp hq hcop, totient_pq hp hq hcop, tau_pq hp hq hcop]
  -- Goal: (p+1)(q+1) * ((p-1)(q-1)) ≠ p*q * 4
  -- 증명 전략: (p-1)(q-1) ≥ 4 (since p,q ≥ 3) 과 (p+1)(q+1) > pq 를 결합
  -- ⟹ (p+1)(q+1)·(p-1)(q-1) ≥ 4·(p+1)(q+1) > 4pq
  intro h
  -- (p-1) ≥ 2, (q-1) ≥ 2
  have hp_sub : p - 1 ≥ 2 := by omega
  have hq_sub : q - 1 ≥ 2 := by omega
  have hpm1_qm1 : (p - 1) * (q - 1) ≥ 4 := by
    calc (p - 1) * (q - 1) ≥ 2 * 2 := by nlinarith
      _ = 4 := by norm_num
  -- (p+1)(q+1) = pq + p + q + 1 > pq
  have hpp1_qp1 : (p + 1) * (q + 1) > p * q := by nlinarith
  -- 결합: (p+1)(q+1) * ((p-1)(q-1)) ≥ (p+1)(q+1) · 4 > pq · 4
  have K_pos : (p + 1) * (q + 1) ≥ 1 := by nlinarith
  have step1 : (p + 1) * (q + 1) * ((p - 1) * (q - 1)) ≥ (p + 1) * (q + 1) * 4 :=
    Nat.mul_le_mul_left ((p + 1) * (q + 1)) hpm1_qm1
  have step2 : (p + 1) * (q + 1) * 4 > p * q * 4 := by nlinarith [hpp1_qp1]
  -- h : (p + 1) * (q + 1) * ((p - 1) * (q - 1)) = p * q * 4
  -- step1 + step2 → LHS of h > RHS of h, contradiction
  omega

/-- Case 2 combined: for any distinct primes p, q, σ(pq)·φ(pq) = pq·τ(pq) ⟺ pq = 6 -/
-- 본 정리는 case 2a (theorem_B_2q) + case 2b (theorem_B_odd_pq) 결합

example : σ 1 (2 * 3) * Nat.totient (2 * 3) = (2 * 3) * (Nat.divisors (2 * 3)).card := by decide
example : σ 1 (3 * 5) * Nat.totient (3 * 5) ≠ (3 * 5) * (Nat.divisors (3 * 5)).card := by decide
example : σ 1 (3 * 7) * Nat.totient (3 * 7) ≠ (3 * 7) * (Nat.divisors (3 * 7)).card := by decide
example : σ 1 (5 * 7) * Nat.totient (5 * 7) ≠ (5 * 7) * (Nat.divisors (5 * 7)).card := by decide

-- 일반 p, q odd prime 적용 예시 (Lean4 kernel 이 prime 판정)
example : σ 1 (3 * 5) * Nat.totient (3 * 5) ≠ (3 * 5) * (Nat.divisors (3 * 5)).card :=
  theorem_B_odd_pq (by decide : (3 : ℕ).Prime) (by decide : (5 : ℕ).Prime)
    (by norm_num) (by norm_num) (by decide)

example : σ 1 (7 * 11) * Nat.totient (7 * 11) ≠ (7 * 11) * (Nat.divisors (7 * 11)).card :=
  theorem_B_odd_pq (by decide : (7 : ℕ).Prime) (by decide : (11 : ℕ).Prime)
    (by norm_num) (by norm_num) (by decide)

-- Theorem B 진전:
--  Case 1 (prime p): ✓ theorem_B_prime_case
--  Case 2a (n = 2q, q odd prime): ✓ theorem_B_2q
--  Case 2b (n = pq, p, q odd distinct primes): ✓ theorem_B_odd_pq (본 파일)
--  Case 3 (n = p^a, a ≥ 2): 후속
--  Case 4 (n = 일반 합성수): v5 이상

end N6Mathlib
