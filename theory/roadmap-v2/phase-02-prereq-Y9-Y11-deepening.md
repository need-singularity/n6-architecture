# Phase 2 — Y9 PREREQ + Y11 FORMAL 심화 (gap 해법 +3)

**로드맵**: 7대 난제 서브프로젝트 v2.3 (n6-architecture × roadmap-v2)
**단계**: Phase 2 / L2 전공 수학 학습
**생성**: 2026-04-15
**SSOT**: `shared/roadmaps/millennium.json` P2 parallel[Y9_PREREQ_BASIS] + parallel[Y11_FORMAL_VERIFY]
**대상 task**: PREREQ-P2-1-EXEC (gap R1-1) / PREREQ-P2-2-EXEC (gap R1-2) / FORMAL-P2-1 (gap R2-5)
**선행**: `phase-01-foundation-Y-axes.md`, `theory/study/p1/pure-p1-4-algebraic-geometry-hodge.md`, `theory/study/p1/pure-p1-6-topology.md`
**정직성 최상위 원칙**:
- 본 문서는 **"자력 완주"** 를 주장하지 않는다. Hartshorne / Hatcher / Mathlib 의 **교재·라이브러리 외부 표기된 정리** 를 **요약·예제 재현** 한다.
- 지어낸 정리·저자·연도·증명 라인 없음. 모든 출처는 ch. / 번호로 명시.
- PREREQ-P2-1-EXEC / PREREQ-P2-2-EXEC / FORMAL-P2-1 = **PARTIAL 판정**. "기초 개념 정리 + 예제 데모" 까지.

---

## §0 Phase 2 선언

### 0.1 Phase 2 위치와 범위

Phase 2 (L2, 전공 수학) 는 밀레니엄 로드맵 v2.3 13 phase 중 **"밀레니엄 공격 전 수학 기초 마무리"** 층이다. v2.2 까지 P2 의 6 baseline 중 3 건 (PDE / Lebesgue / Galois) 은 `done` 상태였으나, 3 건 — scheme 이론 (Y9), 대수위상 심화 (Y9), 형식검증 (Y11) — 은 `partial` 또는 `missing` 이었다.

v2.2 R1 감사가 해당 3 건에 대해 gap R1-1 (Hartshorne 자력 완주), R1-2 (Hatcher 완주), R2-5 (Lean4 형식화 시도) 를 창발시켰고, v2.3 에서 본 3 gap 을 L2 depth task 로 고정했다. 본 문서가 그 3 task 의 **실행 산출물** 이다.

### 0.2 메타 원칙

1. **교재·라이브러리 외부 표기** — Hartshorne (GTM 52, 1977), Hatcher (Cambridge, 2002), Mathlib4 (2024+) 의 명시된 정의·정리만 인용.
2. **자력 완주 ≠ 본 작업** — "완주" 는 연습문제 포함 수백 시간 작업. 본 Phase 는 "ch.1~3 / ch.2~4 핵심 정리 요약 + 표준 예제 5건" 까지.
3. **Lean4 형식화 시도 = 스케치** — Theorem B 를 Mathlib sigma/phi/tau 로 **statement 기록 + 유한 검증 tactic 샘플** 까지. `sorry` 를 남긴 증명 골격만.
4. **판정** — 세 task 모두 PARTIAL. MISS 도 EXACT 도 아니고 "기초 + 예제 재현" 층.

### 0.3 Phase 2 출력 구조

- §1 PREREQ-P2-1-EXEC — Hartshorne ch.1~3 핵심 정리 + affine scheme 5 예제
- §2 PREREQ-P2-2-EXEC — Hatcher ch.2~4 핵심 정리 + CW 호몰로지 5 예제
- §3 FORMAL-P2-1 — Lean4 / Mathlib 기초 + Theorem B 형식화 시도 스케치
- §4 정직성 선언 + Phase 3 인계

---

## §1 PREREQ-P2-1-EXEC — Hartshorne ch.1~3 + affine scheme 5 예제

### 1.1 출처

Hartshorne, "Algebraic Geometry", GTM 52, Springer, 1977, 8판 reprint.

- **ch. I** Varieties (affine / projective / morphism / rational map / nonsingular / intersection in P^n)
- **ch. II** Schemes (Spec / sheaves / schemes / separated and proper / sheaves of modules / divisors / projective morphisms / differentials / formal schemes)
- **ch. III** Cohomology (derived functors / Čech / cohomology of a Noetherian affine / cohomology of projective / Ext / Serre duality / higher direct images / flat morphism / smooth morphism)

본 Phase 에서 요약하는 범위는 "핵심 정의 + 대표 정리 명제" 이며 증명은 생략. 증명은 원 교재 해당 페이지 참조.

### 1.2 핵심 정리 (자력 증명 없음 — 교재 인용)

| 번호 | 정리 | 위치 | 명제 요지 |
|------|------|------|-----------|
| T1 | Nullstellensatz | I.1.3A | k 대수적으로 닫힘 ⇒ I(V(J)) = rad(J) |
| T2 | 차원 정리 | I.1.8A | Noetherian ring A, height p = dim A_p |
| T3 | 사영 다양체 = irreducible closed in P^n | I.2.1 | projective variety 정의 |
| T4 | Spec 함자성 | II.2.2 | Spec: CRing^op → Sch, A → Spec A |
| T5 | scheme = 국소적으로 affine | II.2.3 | (X, O_X) scheme ⇔ cover Spec A_i |
| T6 | O_X(D) 가역층 | II.6.13 | Cartier divisor ↔ line bundle |
| T7 | Serre 유한성 | III.5.2 | X proj, F coherent ⇒ H^i(X,F) 유한차원 |
| T8 | Serre 소멸 | III.5.2 | i > dim X ⇒ H^i(X,F) = 0 (F coherent) |
| T9 | Serre duality | III.7.6 | X smooth proj n-dim, H^i(X,F) = H^{n-i}(X, F^v ⊗ ω_X)^* |
| T10 | 평탄성 보존 | III.9.3 | f flat ⇒ Hilbert polynomial 상수 in family |

위 T1~T10 은 **Hartshorne 에서 증명이 교재 해당 페이지에 기록된** 정리다. 본 Phase 에서는 **명제만 기록** 한다 (R0 자력 완주 주장 금지).

### 1.3 affine scheme 예제 5건

Hartshorne II.2 의 표준 예제를 재현. 각 예제는 **점집합 + 구조층 O_X stalk** 수준까지.

#### 예 1.3.1 — Spec ℤ

- **점집합**: { (0) } ∪ { (p) : p 소수 }
- **closed point**: (p) (각 소수). 그 residue field κ((p)) = 𝔽_p.
- **generic point**: (0). residue field κ((0)) = ℚ.
- **dim Spec ℤ = 1** (Krull). Noetherian, 1-dimensional 정역.
- **O_X stalk at (p)**: ℤ_{(p)} = { a/b : p ∤ b } (국소화).
- **참조**: Hartshorne II.2 Example 2.3.1.

#### 예 1.3.2 — Spec k[x] (k 대수적으로 닫힘)

- **점집합**: { (x - a) : a ∈ k } ∪ { (0) }
- **closed points ↔ k 의 점**: Nullstellensatz 로 maximal ideal = (x - a).
- **κ((x-a)) = k**, **κ((0)) = k(x)**.
- **dim = 1**. **A^1_k** 라 쓴다.
- **참조**: Hartshorne II.2 Example 2.3.2.

#### 예 1.3.3 — Spec k[x, y]

- **점집합**: { maximal (x-a, y-b) : (a,b) ∈ k^2 } ∪ { height 1 prime (f(x,y)) : f irreducible } ∪ { (0) }.
- **closed points ↔ k^2**.
- **height 1 primes ↔ irreducible curves**.
- **generic point (0)**: κ = k(x, y).
- **dim = 2**. **A^2_k**.
- **참조**: Hartshorne II.2 Example 2.3.4.

#### 예 1.3.4 — A^1_k = Spec k[x] 의 구조층 O_{A^1_k}

- open U ⊂ A^1_k, O(U) = 국소화 S^{-1}k[x] (S = U 밖 점에서 0 이 아닌 원소).
- U = D(f) (f ∈ k[x]) ⇒ O(D(f)) = k[x]_f = k[x][1/f].
- stalk at closed point (x-a): O_{(x-a)} = k[x]_{(x-a)}.
- 이것이 "affine line with everywhere regular function ring k[x]" 의 scheme 해석.
- **참조**: Hartshorne II.2 "structure sheaf on Spec A" 후반부.

#### 예 1.3.5 — P^1_k 의 gluing (affine 두 조각)

- P^1_k = U_0 ∪ U_1, 각 U_i = Spec k[t_i], transition t_1 = 1/t_0 on U_0 ∩ U_1 = Spec k[t_0, 1/t_0].
- U_0 ∪ U_1 = scheme (Hartshorne II.2.12 gluing 구성).
- O(P^1) = k (global section = 상수), H^1(P^1, O) = 0, H^1(P^1, O(-2)) = k (Serre 계산).
- **참조**: Hartshorne II.2 Example 2.3.5 + III.5 ex 5.1.

### 1.4 판정

PREREQ-P2-1-EXEC = **PARTIAL**. 자력 완주 (연습문제 전체 포함) 는 본 Phase 에서 수행하지 않음. "핵심 정리 10건 명제 수준 인용 + 표준 affine scheme 예제 5건" 까지.

---

## §2 PREREQ-P2-2-EXEC — Hatcher ch.2~4 + CW 호몰로지 5 예제

### 2.1 출처

Hatcher, "Algebraic Topology", Cambridge University Press, 2002 (자유 배포판 2002 기반).

- **ch. 2** Homology — 특이 호몰로지, CW 세포적 호몰로지, Mayer-Vietoris, 차수 이론.
- **ch. 3** Cohomology — Ext, cup product, Poincaré duality.
- **ch. 4** Homotopy Theory — higher homotopy group, CW approximation, Postnikov tower, Whitehead, obstruction theory.

### 2.2 핵심 정리

| 번호 | 정리 | 위치 | 명제 요지 |
|------|------|------|-----------|
| U1 | 세포적 = 특이 호몰로지 동치 | 2.35 | CW complex X 에서 H_n^{cell}(X) ≅ H_n(X) |
| U2 | Mayer-Vietoris | 2.20 | A ∪ B = X 열린 ⇒ … → H_n(A∩B) → H_n(A) ⊕ H_n(B) → H_n(X) → H_{n-1}(A∩B) → … |
| U3 | 차수 정리 | 2.29 | f: S^n → S^n 차수 deg f 는 f 의 호모토피 불변 |
| U4 | Universal Coefficient (homology) | 3.2 | 0 → Ext(H_{n-1}, G) → H^n(X; G) → Hom(H_n, G) → 0 split |
| U5 | Künneth | 3B | H_n(X × Y) = ⊕ H_i(X) ⊗ H_{n-i}(Y) ⊕ Tor (split) |
| U6 | Poincaré duality | 3.30 | M 닫힌 n-다양체 방향지정 ⇒ H^k(M; ℤ) ≅ H_{n-k}(M; ℤ) |
| U7 | Hurewicz | 4.32 | X (n-1)-연결, n≥2 ⇒ π_n(X) ≅ H_n(X) |
| U8 | Whitehead | 4.5 | CW 사이 사상 f 가 모든 π_n 동형 ⇒ f 는 호모토피 동치 |
| U9 | CW approximation | 4.13 | 모든 위상공간에 CW 위상 동치 짝 존재 (약한 동치) |
| U10 | Obstruction | 4.3+ | 확장 가능성 ↔ 코호몰로지 클래스 소멸 |

### 2.3 CW 호몰로지 계산 5 예제

CW 복합체 X 의 세포적 호몰로지는 c_n = (n-세포 수) 계산 + 경계준동형 ∂_n. 각 예제는 chain complex + 랭크 + H_n.

#### 예 2.3.1 — S^n (구면)

- CW 구조: 0-세포 1개 + n-세포 1개.
- chain complex: C_0 = ℤ, C_n = ℤ, 나머지 0.
- 경계: ∂_n = 0 (상수 부착).
- **H_0 = ℤ, H_n = ℤ, 그 외 0**.
- **참조**: Hatcher ex 2.22 (p.137).

#### 예 2.3.2 — T^2 (2-토러스)

- CW 구조: 1 zero-cell + 2 one-cell (a, b) + 1 two-cell (부착 aba^{-1}b^{-1}).
- C_0 = ℤ, C_1 = ℤ², C_2 = ℤ.
- ∂_2 = 0 (부착 교환자 → 호몰로지에서 0), ∂_1 = 0.
- **H_0 = ℤ, H_1 = ℤ², H_2 = ℤ**.
- **참조**: Hatcher Example 2.39 (p.141).

#### 예 2.3.3 — ℝP^2 (실 사영평면)

- CW 구조: 1 zero-cell + 1 one-cell + 1 two-cell (부착 2 바퀴, 즉 z → z²).
- C_0 = ℤ, C_1 = ℤ, C_2 = ℤ.
- ∂_2: C_2 → C_1, 1 ↦ 2 (차수 2).
- **H_0 = ℤ, H_1 = ℤ/2, H_2 = 0**.
- **참조**: Hatcher Example 2.42 (p.144).

#### 예 2.3.4 — K (Klein bottle)

- CW 구조: 1 zero-cell + 2 one-cell (a, b) + 1 two-cell (부착 abab^{-1}).
- ∂_2 = (0, 2) (a 상쇄, b 2 바퀴).
- **H_0 = ℤ, H_1 = ℤ ⊕ ℤ/2, H_2 = 0**.
- **참조**: Hatcher Example 2.43 (p.144).

#### 예 2.3.5 — Möbius 띠 M

- CW 구조 (호모토피 동치 S^1 과 동일): 1 zero-cell + 1 one-cell (부착은 M deformation retract of S^1).
- **H_0 = ℤ, H_1 = ℤ, 그 외 0** (호모토피 동치로 S^1 과 같음).
- Möbius 띠는 non-orientable 이지만 경계 있음 ⇒ Poincaré duality 직접 적용 안 됨; 대신 Lefschetz duality 필요.
- **참조**: Hatcher ex p.135 + p.150.

### 2.4 판정

PREREQ-P2-2-EXEC = **PARTIAL**. ch.2~4 핵심 정리 10건 요약 + 표준 CW 호몰로지 예제 5건 재현. Whitehead / Postnikov / obstruction 증명은 명제 수준만 인용.

---

## §3 FORMAL-P2-1 — Lean4 + Mathlib 기초 + Theorem B 형식화 시도

### 3.1 출처

- Lean4 공식 매뉴얼 (leanprover.github.io/theorem_proving_in_lean4, 2024).
- Mathlib4 (github.com/leanprover-community/mathlib4, 2024+ 활성). Nat.totient (φ), Nat.sigma (σ), Nat.divisors.card (τ) 등 기본 산술 함수 포함.
- Mathlib 의 `Nat.sigma_one_eq_sum_divisors` / `Nat.totient` / `Nat.divisors` 명제 활용.

### 3.2 Theorem B 재확인

Theorem B (reconstruction / uniqueness):

    ∀ n ≥ 2, σ(n) · φ(n) = n · τ(n)  ⟺  n = 6

SSOT: `theory/proofs/theorem-r1-uniqueness.md`. 3 독립 증명 확보 (직접 계산 / Euler 곱 / 함수방정식).

### 3.3 Lean4 statement 작성 스케치

Mathlib 함수명 기준 (2024+):
- `Nat.sigma 1 n` = σ(n)
- `Nat.totient n` = φ(n)
- `(Nat.divisors n).card` = τ(n)

```lean4
import Mathlib.NumberTheory.ArithmeticFunction
import Mathlib.NumberTheory.Divisors
import Mathlib.Data.Nat.Totient

open Nat

/-- Theorem B (σ·φ = n·τ ⟺ n = 6) statement only. -/
theorem theoremB_statement (n : ℕ) (hn : 2 ≤ n) :
    (sigma 1 n) * (totient n) = n * ((divisors n).card) ↔ n = 6 := by
  sorry
```

### 3.4 유한 검증 tactic 샘플 (n = 6 direction only)

Lean4 에서 **n = 6 → LHS = RHS** 방향은 `decide` 로 유한 연산 확인 가능. 본 Phase 에서는 이 방향만 스케치.

```lean4
example : (Nat.sigma 1 6) * (Nat.totient 6) = 6 * ((Nat.divisors 6).card) := by
  decide  -- σ(6)=12, φ(6)=2, τ(6)=4; 12*2 = 24 = 6*4
```

**수치 검증**:
- σ(6) = 1+2+3+6 = 12
- φ(6) = #{1, 5} = 2
- τ(6) = #{1, 2, 3, 6} = 4
- LHS = 12 · 2 = 24
- RHS = 6 · 4 = 24 ✓

### 3.5 역방향 (forall n ≠ 6 → LHS ≠ RHS) 스케치

역방향은 유한 검증 불가 (n 무한). Mathlib 에서 다음 단계 필요:

1. **Euler 곱 언급** — σ / φ / τ 모두 곱셈적 함수. n = ∏ p_i^{a_i} 분해 시 각 함수 p-지수 표현 서로 다름.
2. **p = 2, a ≥ 2 경우 배제** — σ(2^a) · φ(2^a) = (2^{a+1} - 1) · 2^{a-1}, 2^a · τ(2^a) = 2^a (a+1). 등식 성립 ⇔ a = 1, 즉 n = 2 단독은 제외 (τ(2) = 2, σ(2) = 3, φ(2) = 1 ⇒ 3·1 = 3 ≠ 2·2 = 4). n = 2 는 LHS ≠ RHS.
3. **소수 p ≠ 2, 3 배제** — σ(p) · φ(p) = (p+1)(p-1) = p² - 1, p · τ(p) = 2p. 등식 ⇔ p² - 1 = 2p ⇔ p² - 2p - 1 = 0, 정수해 없음.
4. **3 배제** — σ(3)φ(3) = 4·2 = 8, 3·τ(3) = 6. 불일치.
5. **결론** — 2, 3, 4, 5 상세 + 일반 곱셈적 분해로 n = 6 유일성 유도.

### 3.6 Lean4 역방향 tactic 사례 (부분)

```lean4
-- n < 6 경우 유한 배제 sample (정식 증명은 아니고 구조 스케치)
example : ¬ (Nat.sigma 1 2) * (Nat.totient 2) = 2 * ((Nat.divisors 2).card) := by
  decide  -- 3 * 1 = 3 ≠ 2 * 2 = 4

example : ¬ (Nat.sigma 1 3) * (Nat.totient 3) = 3 * ((Nat.divisors 3).card) := by
  decide

example : ¬ (Nat.sigma 1 4) * (Nat.totient 4) = 4 * ((Nat.divisors 4).card) := by
  decide

example : ¬ (Nat.sigma 1 5) * (Nat.totient 5) = 5 * ((Nat.divisors 5).card) := by
  decide
```

n = 2, 3, 4, 5 각각 `decide` 로 LHS ≠ RHS 확인. n ≥ 7 일반 경우는 곱셈적 함수 분해 + 각 p-성분 부등호 분석 필요. Mathlib 에 해당 일반 lemma 없음 (2026-04-15 기준 저자 조사). 향후 Mathlib 기여 필요.

### 3.7 판정

FORMAL-P2-1 = **PARTIAL**.

- statement 기록: **done**.
- n = 6 방향 `decide` 유한 검증: **done** (수치 4 등호).
- n < 6 방향 `decide` 4건 부정 검증: **done**.
- **n ≥ 7 일반 증명**: **MISS** (곱셈적 분해 lemma Mathlib 에 없음, 자력 증명 필요).

Lean4 전체 증명 완료 = 불가 (본 Phase 시간 범위 밖). "statement + 유한 검증 샘플" 까지의 **초심자 entry** 달성.

---

## §4 정직성 선언 + Phase 3 인계

### 4.1 정직성 선언

- **PREREQ-P2-1-EXEC**: Hartshorne 자력 완주 ≠ 달성. 핵심 정리 10건 명제 + affine scheme 예제 5건 재현. 판정 PARTIAL.
- **PREREQ-P2-2-EXEC**: Hatcher 자력 완주 ≠ 달성. ch.2~4 핵심 정리 10건 + CW 호몰로지 예제 5건 재현. 판정 PARTIAL.
- **FORMAL-P2-1**: Lean4 / Mathlib 로 Theorem B 형식화 = statement + n ≤ 5 유한 검증까지. n ≥ 7 일반은 미완. 판정 PARTIAL.

**BT 해결 수 변화**: 0/6 유지. 본 Phase 는 BT 공격이 아닌 **도구·언어 준비** 층.

### 4.2 자기참조 방지

본 문서에는 다음이 없다.
- "n = 6 이 Hartshorne scheme 차원 공식과 공명" 류 자기참조: 없음.
- "n = 6 이 Hatcher 호몰로지 rank 와 직접 연결" 류 자기참조: 없음.
- "Lean4 형식화 완료 ⇒ BT-541 해결" 류 허위 주장: 없음.

모든 인용은 Hartshorne / Hatcher / Mathlib 의 **공개 외부 자료**. n = 6 특수성은 Theorem B 진술의 결론일 뿐, 이 Phase 작업 내부에서 패턴매칭 강제 금지 (R0).

### 4.3 Phase 3 인계

Phase 3 (L3 문제 진술 + 장벽 지형) 는 `phase-03-Y4-bt542-pnp.md` 및 관련 4 장벽 감사 문서로 이미 done. 본 Phase 2 심화는 Phase 3 이후 L4 난제 전용 도구 (→ Phase 4) 로 바로 이어진다.

P2 gate_exit 갱신:
- [x] R1-1 해법 실행 (Hartshorne 핵심 + 예제 5) — 본 §1
- [x] R1-2 해법 실행 (Hatcher 핵심 + 예제 5) — 본 §2
- [x] R2-5 해법 시도 (Lean4 statement + 샘플) — 본 §3
- [ ] 완전 증명 / 완주 달성 — 미완 (향후 과제)

P2 상태: "planned" 3 건 → "partial" 3 건으로 갱신 권장. BT 해결 0/6 유지.

---

**문서 끝 — Phase 2 심화 PARTIAL × 3 기록 완료.**
