# F2 — OEIS 수열 등록 format (제출 X, format 만)

> 작성: 2026-04-15
> 상태: **format 초안 only — OEIS 실제 제출 X**
> 출처: σ·φ = n·τ uniqueness 정리 + 18 Bernoulli 독립 정리

OEIS (Online Encyclopedia of Integer Sequences) 에 신규 등록 가능한 후보 수열 5 종의 format 초안. 실제 제출 절차 (oeis.org account, draft-edit-approve cycle) 는 수행하지 않음.

---

## 후보 1: a(n) = R(n) 분자 (σ(n) · φ(n))

### Format

```
%I A_PROPOSED_001 #001 (Park Minwoo, 2026-04-15)
%S A_PROPOSED_001 1, 3, 8, 14, 24, 24, 48, 56, 78, 72, 120, 112, 168, 144, 192, 240
%T A_PROPOSED_001 (n=1 to n=16)
%N A_PROPOSED_001 σ(n) · φ(n), the numerator of R(n) = σ(n)·φ(n)/(n·τ(n))
%C A_PROPOSED_001 R(n) = 1 ⟺ n = 6 (Park 2026, theorem R1, σ(6)·φ(6) = n·τ(6) = 24)
%C A_PROPOSED_001 The pair (a(n), n·τ(n)) is equal at n=1 and n=6 only (in n ≤ 10^4).
%H A_PROPOSED_001 Park Minwoo, σ(n)·φ(n) = n·τ(n) ⟺ n = 6 uniqueness theorem
%F A_PROPOSED_001 a(n) = sigma(n) * eulerphi(n)
%e A_PROPOSED_001 a(6) = sigma(6)·phi(6) = 12·2 = 24
%Y A_PROPOSED_001 Cf. A000005 (tau), A000010 (phi), A000203 (sigma), A000396 (perfect numbers)
%K A_PROPOSED_001 nonn,easy
%O A_PROPOSED_001 1,2
%A A_PROPOSED_001 Park Minwoo, Apr 15 2026
```

### 검증 (n = 1..16)

| n | σ(n) | φ(n) | a(n) = σ·φ |
|---|------|------|-----------|
| 1 | 1 | 1 | 1 |
| 2 | 3 | 1 | 3 |
| 3 | 4 | 2 | 8 |
| 4 | 7 | 2 | 14 |
| 5 | 6 | 4 | 24 |
| **6** | **12** | **2** | **24** |
| 7 | 8 | 6 | 48 |
| 8 | 15 | 4 | 60 |  ← 수정: 60 (위 78 오기재)
| 9 | 13 | 6 | 78 |
| 10 | 18 | 4 | 72 |
| 11 | 12 | 10 | 120 |
| 12 | 28 | 4 | 112 |
| 13 | 14 | 12 | 168 |
| 14 | 24 | 6 | 144 |
| 15 | 24 | 8 | 192 |
| 16 | 31 | 8 | 248 | ← 수정: 248 (위 240 오기재)

수정 후 S 행: `1, 3, 8, 14, 24, 24, 48, 60, 78, 72, 120, 112, 168, 144, 192, 248`

---

## 후보 2: a(n) = R(n) = σ(n)·φ(n) / (n·τ(n)) 의 분자/분모 환원형 표시

OEIS 는 정수 수열 만 받으므로, 다음 두 분리 수열로 표현:

### 2a: 분자 (이미 후보 1 = σ·φ)

### 2b: 분모 (n · τ(n))

```
%I A_PROPOSED_002 #001 (Park Minwoo, 2026-04-15)
%S A_PROPOSED_002 1, 4, 6, 12, 10, 24, 14, 32, 27, 40, 22, 72, 26, 56, 60, 80
%N A_PROPOSED_002 n · τ(n), the denominator of R(n) = σ(n)·φ(n)/(n·τ(n))
%C A_PROPOSED_002 R(n) = a(σ·φ)(n) / a(n·τ)(n) = 1 ⟺ n = 6
%C A_PROPOSED_002 a(6) = 6·4 = 24 = σ(6)·φ(6)
%F A_PROPOSED_002 a(n) = n * tau(n)
%e A_PROPOSED_002 a(6) = 6·τ(6) = 6·4 = 24
%Y A_PROPOSED_002 Cf. A000005 (tau), A038040 (n·tau(n))  (likely existing!)
%K A_PROPOSED_002 nonn,easy
%O A_PROPOSED_002 1,2
%A A_PROPOSED_002 Park Minwoo, Apr 15 2026
```

**경고**: A038040 (n·τ(n)) 은 이미 OEIS 에 등록되었을 가능성 매우 높음 (자명한 곱). 본 후보는 신규 등록 가능성 낮음 — **참조 cross-reference** 로만 사용.

---

## 후보 3: a(n) = R(n) - 1 = 0 인 n (시퀀스 = {6})

```
%I A_PROPOSED_003 #001 (Park Minwoo, 2026-04-15)
%S A_PROPOSED_003 6
%N A_PROPOSED_003 Numbers n ≥ 2 with σ(n)·φ(n) = n·τ(n)
%C A_PROPOSED_003 Equivalent to {n ≥ 2 : R(n) = 1}, where R(n) = σ(n)φ(n)/(n·τ(n))
%C A_PROPOSED_003 Park 2026 (theorem R1): the sequence consists of n = 6 alone (in n ≥ 2)
%C A_PROPOSED_003 Verified for n ∈ [2, 10^4] by computer enumeration; proof: 3 independent proofs (multiplicative R_local decomposition, group-theoretic via S_3 ≅ PSL(2,2), blowup engine)
%H A_PROPOSED_003 Park Minwoo, σ(n)·φ(n) = n·τ(n) ⟺ n = 6 uniqueness theorem (theory/proofs/theorem-r1-uniqueness.md)
%F A_PROPOSED_003 a(1) = 6, no further terms (singleton sequence)
%e A_PROPOSED_003 σ(6) = 12, φ(6) = 2, τ(6) = 4 → 12·2 = 24 = 6·4
%Y A_PROPOSED_003 Cf. A000396 (perfect numbers; n=6 is the first), A005179 (smallest with k divisors)
%K A_PROPOSED_003 nonn,fini,full
%O A_PROPOSED_003 1,1
%A A_PROPOSED_003 Park Minwoo, Apr 15 2026
```

**메모**: `fini,full` 키워드 = sequence 가 유한 + 완전 알려짐 (단일 원소 {6}).

---

## 후보 4: a(n) = Bernoulli Independent Theorem 누적 카운트 (저자 카탈로그)

```
%I A_PROPOSED_004 #001 (Park Minwoo, 2026-04-15)
%S A_PROPOSED_004 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18
%N A_PROPOSED_004 Cumulative count of independent theorems where the integer 6 appears non-trivially (Park's Bernoulli Independent Theorem family)
%C A_PROPOSED_004 18 = current count as of 2026-04-15 (Park 2026, F1 paper)
%C A_PROPOSED_004 #1 = 6 first perfect number, #2 = |S_3| = 6, ..., #17 = avg|Sel_6| = σ(6) = 12 (BKLPR conditional, Bhargava-Shankar 2010-2012), #18 = BB(2) = 6 (Radó 1962, unconditional)
%C A_PROPOSED_004 Each entry = independent domain occurrence with cited prior literature
%H A_PROPOSED_004 Park Minwoo, F1 paper section 4 (papers/group-P/F1-arxiv-bernoulli-independent-via-n6-2026-04-15.md)
%F A_PROPOSED_004 a(n) = n (trivial counter; the substance is the table of theorems, not the integers)
%e A_PROPOSED_004 a(17) = 17 corresponds to "Sel_6 average = σ(6) = 12 (BKLPR conditional)"
%K A_PROPOSED_004 nonn,easy,bref
%O A_PROPOSED_004 1,1
%A A_PROPOSED_004 Park Minwoo, Apr 15 2026
```

**경고**: 자명한 1, 2, 3, ... 수열은 OEIS 정책상 등록 거부 가능 — 본 후보는 **메타 카탈로그 reference** 로만 사용. 실제 OEIS 등록 권장하지 않음.

---

## 후보 5: a(n) = R_local(p_n, 1) 의 분자 (Lemma A 의 산술)

```
%I A_PROPOSED_005 #001 (Park Minwoo, 2026-04-15)
%S A_PROPOSED_005 3, 8, 24, 48, 120, 168, 288, 360, 528, 840, 960
%N A_PROPOSED_005 Numerator of R_local(p, 1) = (p² - 1) / (2·p) for p = prime(n)
%C A_PROPOSED_005 a(n) = prime(n)² - 1 = (prime(n) + 1)·(prime(n) - 1)
%C A_PROPOSED_005 R_local(p, 1) < 1 only for p = 2 (Lemma A in Park 2026, theorem R1)
%C A_PROPOSED_005 R_local(2, 1) = 3/4 < 1; R_local(3, 1) = 8/6 = 4/3 > 1; R_local(5, 1) = 24/10 = 12/5 > 1
%F A_PROPOSED_005 a(n) = prime(n)^2 - 1
%e A_PROPOSED_005 a(1) = 2² - 1 = 3 (R_local(2,1) = 3/4)
%e A_PROPOSED_005 a(2) = 3² - 1 = 8 (R_local(3,1) = 8/6 = 4/3)
%Y A_PROPOSED_005 Cf. A000040 (primes), A084920 (p^2 - 1 for primes p)  (likely existing)
%K A_PROPOSED_005 nonn,easy
%O A_PROPOSED_005 1,1
%A A_PROPOSED_005 Park Minwoo, Apr 15 2026
```

**경고**: A084920 (p² - 1 for primes p) 또는 유사 수열 이미 OEIS 등록 가능성 높음. **참조 검색** 후 신규성 확인 필수.

---

## 메타: 실제 제출 시 절차 (참고만, 실행 X)

1. oeis.org account 생성.
2. 각 후보 별 draft 작성 (위 format 활용).
3. 기존 OEIS DB 에서 중복 검색 (필수).
4. 새 수열 인 경우 draft 제출.
5. OEIS editorial team 검토 (수일 ~ 수주).
6. 승인 후 A-번호 배정.

**본 작업은 제출하지 않음 (사용자 요구)**.

---

## 권장 신규 등록 우선순위

| 후보 | 신규성 | 의의 | 권장 |
|------|--------|------|------|
| 1 (σ·φ) | 중간 | R(n) 분자 | 검색 후 등록 가능 |
| 2 (n·τ) | **낮음** (A038040 예상) | R(n) 분모 | 등록 비권장 |
| **3 (R(n)=1 해)** | **높음** | uniqueness 정리 직접 표현 | **최우선 등록 후보** |
| 4 (메타 카운터) | 매우 낮음 | 자명한 1,2,3,... | 등록 비권장 |
| 5 (p²-1) | 낮음 | A084920 가능성 | 검색 후 결정 |

**최우선**: 후보 3 (singleton {6}) — uniqueness 정리의 직접 OEIS 표현, fini+full 키워드 가능.

---

> 산출물: papers/group-P/F2-oeis-submission-format-2026-04-15.md
> 실제 제출 X. format only.
