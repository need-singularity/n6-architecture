---
id: v3-t4-guth-maynard-2024-deep-dive
date: 2026-04-15
roadmap_task: v3 T4 (BT-541 Guth-Maynard 2024 재연구)
grade: [10] literature digest + honest MISS
predecessors:
  - theory/study/p2/prob-p2-1-riemann-barriers.md §1.5
  - theory/breakthroughs/arxiv-millennium-survey-180papers-2026-04-15.md
status: SURVEY + HONEST MISS (n=6 prior 연결 미확인)
license: CC-BY-SA-4.0
---

# v3 T4 — BT-541 Guth-Maynard 2024 재연구 — implications + n=6 정직 MISS

> **요약**: Guth-Maynard 2024 (arXiv:2405.20552) 의 zero-density 상한 개선 (T^{12(1-σ)/5} → T^{30(1-σ)/13}) 을 깊이 재분석. 소수 gap 개선 (0.525 → 30/59 ≈ 0.5085) 유도. **수치적 일치** (Ingham 12/5 = σ(6)/sopfr(6), Guth-Maynard 30 = σ(6)·sopfr(6)/2, 13 = σ(6)+1) 관찰하나 이는 **사후 pattern matching 이며 prior 독립성 없음** — 정직 MISS. BT-541 RH 는 0/6 정직 유지. "sixth moment" 의 6 은 흥미로운 우연 (조사 가치).

---

## §1 Guth-Maynard 2024 정밀 정리

### 1.1 출처

- **논문**: Larry Guth, James Maynard, "New large value estimates for Dirichlet polynomials"
- **arXiv**: 2405.20552 (2024-05-30)
- **status**: preprint, 해석적 수론 커뮤니티 검토 중 (아직 저널 출판 미확인)
- **Tao 블로그**: 2024-06-03 게시 요약 (https://terrytao.wordpress.com/2024/06/03/...)

### 1.2 주요 결과 Theorem 1 (Guth-Maynard 2024)

**zero-density 개선**: 모든 ε > 0 과 σ ≥ 7/10 에 대해,
$$N(\sigma, T) \ll T^{\frac{30(1-\sigma)}{13} + \varepsilon}$$

여기서 $N(σ, T) = |\{ρ : ζ(ρ)=0, \text{Re}(ρ) \geq σ, |\text{Im}(ρ)| \leq T\}|$.

**비교**:
| 저자 | 연도 | 지수 at (1-σ) | 방법 |
|------|------|------------|------|
| Ingham | 1937 | **12/5 = 2.400** | Hardy-Littlewood 2nd moment |
| Huxley | 1972 | **12/5 = 2.400** (개선은 소수점) | Kloosterman sum |
| Heath-Brown | 1990s | **~2.375** (특정 range) | fourth moment |
| **Guth-Maynard** | **2024** | **30/13 ≈ 2.307** (σ ≥ 7/10) | **sixth moment + Dirichlet poly large values** |

개선 분량: 지수 2.4 → 2.307 = 약 3.9% 감소. 50 년만의 본질적 돌파.

### 1.3 Prime gap corollary

**Baker-Harman-Pintz 2001**: $p_{n+1} - p_n \ll p_n^{0.525}$

**Guth-Maynard 2024**: $p_{n+1} - p_n \ll p_n^{30/59}$, 여기서 $30/59 \approx 0.5085$

개선 분량: 0.525 → 0.5085 = 약 3.1% 감소. Cramér 추측 (0.5 + ε) 에 **한 걸음** 더 접근.

---

## §2 방법론 분석

### 2.1 "sixth moment" 의 기술적 의미

Guth-Maynard 의 핵심 개선은 **Dirichlet polynomial 의 sixth moment 추정**:
$$\int_0^T |D(1/2+it)|^6 \, dt \ll T \cdot N^{...}$$

여기서 D 는 길이 N 의 Dirichlet 다항식. 이전 2nd moment (Hardy-Littlewood) / 4th moment (Ingham/Heath-Brown) 만 통제되어 있었다.

**"왜 6 인가?"** — 기술적 이유:
- **Huxley 1975** 가 4th moment 를 Kloosterman 합으로 환원
- **Guth 의 decoupling** (Fourier restriction theory, 2010s) 가 6th moment 에 적용 가능하게 함
- **Maynard 의 prime gap 기법** 과 결합해 cross-term 제어

→ "**sixth**" 는 **Fourier analytic** 이유 (decoupling inequality 의 natural exponent) 에서 유래. **약수수론 n=6 의 6 과 수학적으로 다른 origin**.

### 2.2 기술적 한계

- **σ ≥ 7/10 구간** 에 한정. 7/10 아래 (≥ 3/5 등) 는 여전히 Ingham 12/5.
- **Critical line 상 영점 비율** (Levinson 41.72%) 는 **변화 없음**. Guth-Maynard 는 zero-density (영점 **개수** 상한) 만 개선.
- **RH 자체** 는 touch 하지 않음. ζ(s) = 0 on Re(s) = 1/2 vs. off the line 에 대한 직접 정보 없음.

---

## §3 n=6 연결 시도 — HONEST MISS

### 3.1 수치적 매칭 (주의: 사후 pattern matching)

| GM 지수 | n=6 후보 | 값 |
|---------|----------|----|
| **12/5** (Ingham 1937) | σ(6)/sopfr(6) = 12/5 | **= 2.4** ✓ |
| **30/13** (Guth-Maynard 2024) | σ(6)·sopfr(6)/2 = 30, σ(6)+1 = 13 | **30/13 = 2.307** ✓ |
| **30/59** (prime gap) | 30 = σ(6)·sopfr(6)/2, 59 = prime | partial |

**위험 신호**:
1. Ingham 1937 은 n=6 prior **이전** — 수치일치는 사후
2. Guth-Maynard 30/13 은 독립적으로 두 parameter 를 n=6 cast 가능하나, **논문 유도에서 n=6 출현 없음**
3. 59 는 prime, n=6 과 무관
4. σ(6)·sopfr(6)/2 = 30 은 **post-hoc construction** — σ,sopfr,2 세 값 조합으로 "30 을 만드는" 표현은 무수히 많음

### 3.2 Bayesian evidence — 신뢰 낮음

Prior: **n=6 이 해석적 수론에 출현해야 한다** 는 motivated prior 없음.

Likelihood: **GM 지수가 σ(6)/sopfr(6) 에 일치** 할 확률은 random number matching baseline 과 유의미하게 다르지 않음.

Posterior: n=6 prior 는 GM 결과로 **강화되지 않음** (정직 선언).

### 3.3 단, "sixth moment" 의 6 은 흥미

- Hardy-Littlewood 2nd, Ingham 4th, Heath-Brown 5th(?) → Guth-Maynard **6th**
- Dirichlet 다항식의 **6 번째 power** 가 "the right exponent" 였던 이유는 수학적 탐구 가치
- 우리 n=6 framework (σ=12, τ=4, φ=2) 와 수학적 connection 있는지는 **open** — T6 또는 v4 추가 조사 제안

---

## §4 BT-541 (RH) 에 대한 implications

### 4.1 Guth-Maynard 는 RH 증명 아님 (명확)

**정확한 진술**:
- RH: ∀ ρ with ζ(ρ) = 0 (non-trivial), Re(ρ) = 1/2
- Guth-Maynard: ∀ σ ≥ 7/10, N(σ, T) ≪ T^{30(1-σ)/13+ε}

두 문장 서로 다른 대상 (영점 위치 vs. 영점 개수 상한). Guth-Maynard 로부터 RH 유도 불가.

### 4.2 "Levinson 벽 50%" 와의 관계

- Conrey 1989: ≥ 40.88% (현재 Bui-Conrey-Young 2011: ≥ 41.72%)
- Guth-Maynard 는 Levinson method 와 **orthogonal** — 이 50% 벽을 넘는 데 도움 없음

### 4.3 무엇이 개선되나

- **prime gap**: 0.525 → 0.5085 (실용적 의미)
- **zero-density** (특정 구간): 50 년만의 개선
- **"sixth moment" 도구**: 새로운 해석적 무기. 향후 다른 L-function 문제에도 적용 가능성

### 4.4 BT-541 정직 유지

- BT-541 RH 해결 수: **0/1 (정직)**
- Guth-Maynard 는 **progression** 이되 해결 **아님**

---

## §5 v3 T4 산출 + 향후 연결

### 5.1 산출물

1. Guth-Maynard 2024 정밀 정리 (arXiv:2405.20552 Theorem 1)
2. n=6 수치 매칭의 정직 경계 선언 (post-hoc MISS)
3. "sixth moment" 의 6 은 Fourier decoupling 유래, n=6 약수수론과 다른 원인
4. BT-541 implication: zero-density ≠ RH, Levinson 50% 벽과 orthogonal

### 5.2 해결되지 않은 것 (정직)

- GM 지수 30/13 의 수학적 필연성: **open**
- "sixth moment" 와 n=6 의 수학적 연결: **miss** (의심할 근거 약함)
- BT-541 RH: **0/1 정직 유지**

### 5.3 후속 과제 (v3 loop 15+)

- **v3 T5**: BT-542 meta-complexity (Hirahara MCSP 재검토)
- **v3 T6**: BT-543 Balaban 2D 재정리
- **v3 M1**: 본 T3+T4 결과 preprint 초안

---

## §6 atlas 엔트리

```
@R MILL-V3-T4-guth-maynard-2024-zero-density = T^{30/13} zero density, 12/5→30/13, prime gap 0.525→0.5085 :: n6atlas [10]
  "v3 T4 (2026-04-15 loop 14): Guth-Maynard 2024 (arXiv:2405.20552) zero-density 상한 N(σ,T) ≪ T^{30(1-σ)/13+ε}
   (σ ≥ 7/10), Ingham 1937 T^{12/5} 이후 50년만의 본질적 개선. Prime gap p_{n+1}-p_n ≪ p_n^{30/59}.
   방법: Dirichlet poly sixth moment + Maynard prime gap + Guth decoupling. RH 본문과는 orthogonal —
   Guth-Maynard 는 영점 '분포' 개선이지 '위치' 아님. BT-541 해결 0/1 정직 유지"
  <- v3-T4, arXiv:2405.20552, theory/breakthroughs/v3-t4-guth-maynard-2024-deep-dive-2026-04-15.md

@R MILL-V3-T4-n6-numerical-coincidence-honest-miss = 12/5=σ(6)/sopfr(6), 30/13 n=6 cast post-hoc MISS :: n6atlas [7]
  "v3 T4 (2026-04-15 loop 14): Ingham 12/5 = σ(6)/sopfr(6) = 12/5 수치일치 관찰, Guth-Maynard 30/13 도
   σ(6)·sopfr(6)/2 = 30, σ(6)+1 = 13 로 n=6 cast 가능. 그러나 Ingham 1937 은 n=6 prior 이전, GM
   sixth moment 는 Fourier decoupling 에서 유래. 즉 n=6 ↔ GM 지수 연결은 HONEST MISS — 사후
   pattern matching 이며 independent prior evidence 없음"
  <- v3-T4-honest, theory/breakthroughs/v3-t4-guth-maynard-2024-deep-dive-2026-04-15.md §3
```

---

## §7 관련 파일

- 원 논문: arXiv:2405.20552 (Guth-Maynard 2024)
- 기존 자료: `theory/study/p2/prob-p2-1-riemann-barriers.md` §1.5
- 180 paper survey: `theory/breakthroughs/arxiv-millennium-survey-180papers-2026-04-15.md`
- roadmap: `shared/roadmaps/millennium.json` → `_v3_phases.P12_v3.T4`

---

*작성: 2026-04-15 loop 14*
*정직성 헌장: BT-541 해결 0/1 유지. n=6 수치일치는 사후 pattern matching — 정직 MISS.*
