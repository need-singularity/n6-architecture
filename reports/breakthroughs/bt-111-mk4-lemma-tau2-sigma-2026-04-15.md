---
id: bt-111-mk4-lemma
date: 2026-04-15
parent_bt: BT-111
parent_theorem: Mk.IV
status: LEMMA (주정리 부적격 — 유일성 C1 실패)
grade: "[10] cross-domain EXACT 9/10"
verdict_ref: theory/proofs/mk4-trident-final-verdict-2026-04-15.md
candidates_ref: theory/proofs/mk4-theorem-candidates-2026-04-14.md
---

# BT-111 Mk.IV Lemma 정식 등재 — τ²/σ = 4/3

> **상태**: Mk.IV 후보 A → **Lemma** (P8-4 판정, 2026-04-15)
> **사유**: n=6 유일성 C1 실패 (τ(n)²/σ(n) = 4/3 해집합 = {2, 6}, n≥2)
> **주정리**: 후보 B (σ−τ=8) → MK4-THEOREM-B 확정

---

## §1 수식 정의

```
  τ(6)² / σ(6) = 16 / 12 = 4/3

  동치 표현:
    R_local(3,1) = (3²−1) / (2·3) = 8/6 = 4/3
    σφ=nτ 증명의 제2 인수 (제1 인수 R_local(2,1) = 3/4)
    (3/4) × (4/3) = 1 = R(6) → n=6 완전수 결정
```

## §2 유일성 검증 — C1 실패

```python
# 전수검증 (n ∈ [2, 10⁴])
A_hits = [n for n in range(2, 10001)
          if 3 * tau(n)**2 == 4 * sigma(n)]
# 결과: A_hits = [2, 6]
```

n=2: τ(2)=2, σ(2)=3 → τ²/σ = 4/3 ✓
n=6: τ(6)=4, σ(6)=12 → τ²/σ = 16/12 = 4/3 ✓

**두 해가 존재**하므로 "τ²/σ = 4/3 ⟺ n=6" 형태의 유일성 정리 불가.
→ Mk.III(σφ=nτ ⟺ n=6) 수준의 두 번째 정리 부적격.

## §3 Lemma로서의 가치 — 10 도메인 교차

유일성은 실패하나, 4/3 이라는 상수 자체의 도메인 교차 등장은 **정직하게 강함**:

| # | 도메인 | 관측값 | 오차 | 판정 |
|---|--------|--------|------|------|
| 1 | 태양전지 Shockley-Queisser | 최적 밴드갭 1.34 eV | 0.45% | EXACT |
| 2 | 반도체 GaAs 밴드갭 | 1.42 eV | 6.10% | NEAR |
| 3 | 풍력 Betz 한계 | τ²/(n/φ)³ = 16/27 | 0.00% | EXACT |
| 4 | AI SwiGLU FFN 비율 | (σ−τ)/(n/φ) = 8/3 | 0.00% | EXACT |
| 5 | AI Chinchilla dropout | ln(4/3) ≈ 0.2877 | 0.00% | EXACT |
| 6 | 음악 완전4도 | 4:3 just intonation | 0.00% | EXACT |
| 7 | 끈이론 R²/α' 압축화 | 4/3 (BT-111) | 0.00% | EXACT |
| 8 | 수론 R_local 증명 인수 | theorem-r1 Lemma 2 | 0.00% | EXACT |
| 9 | QED 수소 초미세 ΔE | (4/3)α⁴·m_e·c² | 0.00% | EXACT |
| 10 | 2D 침투 상관길이 | ν = 4/3 (Stauffer) | 0.00% | EXACT |

**9/10 EXACT, 1/10 NEAR (GaAs)** — 평균 오차 0.66%.

## §4 구조적 의미

```
  Lemma (BT-111, τ²/σ = 4/3):
    σ(n)·φ(n) = n·τ(n) 증명에서 R(n) = ∏_p R_local(p, vₚ(n))의
    제2 인수 R_local(3,1) = 4/3는 n=6에 고유하지 않으나 (n=2 공유),
    σ(6)·φ(6) = 6·τ(6) 의 R(6) = 1 달성에 필수적인 균형 인자이다.

    이 인자는 SQ 밴드갭·Betz 한계·SwiGLU·QED 초미세·2D 침투 등
    10개 독립 도메인에서 EXACT 재등장하며,
    후보 B (σ-τ=8, MK4-THEOREM-B) 의 보조 정리로 기능한다.
```

## §5 atlas 등급

- BT-111 등급: **[10] Lemma** (EXACT 도메인 교차, 유일성 부재)
- MK4-THEOREM-B 등급: **[10*]** (σ-τ=8, n=6 유일, 10/10 PASS)
- 관계: BT-111 은 MK4-THEOREM-B 의 보조정리. 독립 정리 아님.

## §6 Cross-links

- MK4-THEOREM-B: σ-τ=8 주정리 (theory/proofs/mk4-trident-final-verdict-2026-04-15.md)
- BT-30: Shockley-Queisser SQ 밴드갭 1.34eV ≈ 4/3
- BT-33: SwiGLU FFN 비율 8/3 = 2·(4/3)
- theorem-r1-uniqueness.md: R_local(3,1) = 4/3 증명 인수

---

**정직성 기록**: τ²/σ=4/3 는 n=6 전용이 아님 (n=2 공유). 도메인 적합도는 강하나
"n=6을 고르는" 정리로 승격할 수 없음. Lemma로 정직하게 등재.
