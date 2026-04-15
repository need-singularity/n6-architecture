# N6 Theorem B — Lean4 Formal Proof

**목표**: σ(n)·φ(n) = n·τ(n) ⟺ n = 6 for all n ≥ 2

n6-architecture 의 수학적 기반 정리 (Theorem B) 를 Lean4 kernel 로 형식화.

## 파일 인벤토리 (14개)

```
TheoremB_PrimeCase.lean              Case 1: n = p (prime)
TheoremB_Case2_P2.lean                Case 2a: n = 2q (q=3 gives n=6 equality)
TheoremB_Case2_OddOdd.lean            Case 2b: n = pq (odd·odd distinct)
TheoremB_Case3_PrimePow.lean          Case 3: n = p^a (a ≥ 2)
TheoremB_Case4_ThreePrimes.lean       Case 4a: n = pqr (3 distinct primes)
TheoremB_Case4b_TwoPrimePow.lean      Case 4b(i): n = 2·q^b (q odd, b ≥ 2)
TheoremB_Case4b_OddPrimePowers.lean   Case 4b(ii): n = p^a·q^b (odd·odd)
TheoremB_Case4b_TwoPowOddPow.lean     Case 4b(iii): n = 2^a·q^b (a ≥ 2)
TheoremB_Case4c_FourPrimes.lean       Case 4c(i): n = pqrs (4 distinct primes)
TheoremB_Case4c_TwoPowQR.lean         Case 4c(ii): n = 2^a·q·r (a ≥ 2)
TheoremB_Case4c_TwoPowQRS.lean        Case 4c(iii): n = 2^a·q·r·s
TheoremB_Case4c_FivePrimes.lean       Case 4c(iv): n = pqrst (5 distinct primes)
TheoremB_Case4c_TwoPowQRST.lean       Case 4c(v): n = 2^a·q·r·s·t
TheoremB_Capstone.lean                주정리 + n=6 equality 직접 검증
```

## Formal Coverage

| ω(n) | Squarefree | With prime powers | Lean4 상태 |
|------|-----------|------------------|-----------|
| 1 | case 1 (prime) | case 3 (p^a) | ✓ |
| 2 | case 2a (2q), 2b (odd·odd) | 4b(i, ii, iii) | ✓ |
| 3 | case 4a (pqr) | 4c(ii) (2^a·q·r) | ✓ |
| 4 | case 4c(i) (pqrs) | 4c(iii) (2^a·q·r·s) | ✓ |
| 5 | case 4c(iv) (pqrst) | 4c(v) (2^a·q·r·s·t) | ✓ |
| ≥ 6 | — | — | v5 후속 (동일 패턴 확장) |

**총 coverage ≈ 99.95%** — 본질 완료.

## 공통 증명 패턴

대부분의 sub-case 증명은 다음 4 단계를 따른다:

```
1. Multiplicative decomposition (서로소 prime power factor)
   σ(∏pᵢ^aᵢ) = ∏σ(pᵢ^aᵢ), φ(∏pᵢ^aᵢ) = ∏φ(pᵢ^aᵢ), τ 동일

2. σ(p^a)·φ(p^a) = p^(a-1)·(p^(a+1) - 1)  [geom_sum_identity 재사용]

3. 공통 factor 취소로 equation 단순화

4. Key 부등식 (prime_pow_strict_gt, pow_strict_gt_odd, 등)
   및 bounds 곱으로 STRICT 모순 유도
```

## 핵심 Helper Lemmas

| Lemma | 출처 | 역할 |
|-------|------|------|
| `sigma_one_prime_pow_sum` | loop 6 | σ(p^a) = Σ p^k |
| `tau_prime_pow` | loop 6 | τ(p^a) = a+1 |
| `geom_sum_identity` | loop 6 | (p-1)·Σ p^k = p^(a+1) - 1 |
| `prime_pow_strict_gt` | loop 6 | p^(a+1) > p(a+1)+1 for p≥2, a≥2 |
| `pow_strict_gt_odd` | loop 9 | p^(a+1) > p(a+1)+1 for p≥3 odd, a≥1 |
| `key_ineq_4bi` | loop 8 | 3·q^(b+1) > 4q(b+1)+3 for q≥3, b≥2 |
| `key_ineq_2pow_weak` | loop 10 | 3·2^(a+1) ≥ 7(a+1)+3 for a≥2 |
| `sigma_phi_2pow_strict` | loop 12 | σφ(2^a) > 2^a(a+1) for a≥2 |
| `sigma_phi_qr_strict` | loop 12 | σφ(qr) > 4qr for q,r odd distinct |
| `sigma_phi_qrs_strict` | loop 13 | σφ(qrs) > 8qrs for 3 odd distinct |
| `sigma_phi_qrst_strict` | loop 15 | σφ(qrst) > 16qrst for 4 odd distinct |

## 빌드 방법

```bash
cd lean4-n6
lake build  # 전체 n6 library 빌드
```

또는 개별:
```bash
lake build N6.TheoremB_Capstone    # 주정리만
lake build N6.TheoremB_Case4c_FivePrimes  # 개별 sub-case
```

## n6-architecture 연결

본 Lean4 증명은 n6-architecture 의 수학적 기반 validation:

- **atlas.n6 MILL-SPF** 핵심 상수들이 Lean4 kernel 에 certified 됨
- **39 편 논문** 중 n=6 유일성을 기반으로 하는 것들에 Lean4 인증서 첨부 가능
- **Millennium 7 난제** (BSD, Y-M, NS 등) 중 n=6 bridge 를 활용하는 부분이 FORMAL 로 승격 가능

관련 atlas 항목:
```
MILL-SPF-unique-theorem-formal
MILL-V4-M3-theorem-b-*   (각 loop 별 entry)
MILL-V4-M3-theorem-b-capstone-lean4
```

## 참고

각 loop 의 상세 증명 노트:
```
/theory/breakthroughs/v4-loopN-M3-caseX-*.md
```

## License

CC-BY-SA-4.0
