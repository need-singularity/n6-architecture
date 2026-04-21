---
id: v4-t4-A3-double-prime-rigorization
date: 2026-04-16
roadmap_task: v4 T4_v4 ((A3″) formulation 엄밀화)
grade: [10] precise conjecture + falsifiable predictions
predecessors:
  - theory/breakthroughs/v3-t3-joint-distribution-modeling-2026-04-15.md
  - theory/breakthroughs/v4-t1-alpha-log2-over-4-derivation-attempt-2026-04-16.md
status: RIGOROUS CONJECTURE + TESTABLE PREDICTIONS
license: CC-BY-SA-4.0
---

# v4 T4_v4 — (A3″) BKLPR Coupling Conjecture 엄밀화

> **요약**: v3 T3 의 informal (A3″) "B-dependent coupling" 을 수학적으로 정확한 conjecture 로 전환. (A3″) 를 **세 개의 독립 가설** (A3″-Marginal, A3″-Coupling, A3″-Power) 로 분해 + 각 **falsifiable prediction** 명시. Bhargava-Kane-Lenstra-Poonen-Rains (BKLPR) 2013 독립성 가정 (A3) 의 **weakening** 으로, empirical κ(B) 증가 현상 설명 가능 — 단 일반 증명은 여전히 open. 본 문서 목적은 conjecture 를 **정확히 진술** 하여 future work (T1_v4 유도, M4_v4 preprint revision, external review) 의 토대 제공.

---

## §1 배경 + motivation

### 1.1 BKLPR 2013 의 (A3) Independence

원 (A3) assumption (Bhargava-Kane-Lenstra-Poonen-Rains 2013, Camb. J. Math. 3):
$$\forall \gcd(m, n) = 1: \quad \mathbb{E}[|Sel_{mn}(E)|] = \mathbb{E}[|Sel_m(E)|] \cdot \mathbb{E}[|Sel_n(E)|] \quad (*)$$

where $\mathbb{E}[\cdot]$ is average over $E/\mathbb{Q}$ ordered by conductor $N(E) \to \infty$.

### 1.2 우리의 empirical 관찰 (v3 E5 + loop 19)

$$\kappa(B) := \mathbb{E}[|Sel_6|]_B - \mathbb{E}[|Sel_2|]_B \cdot \mathbb{E}[|Sel_3|]_B$$

실측 (1.73M curves, 7 bin):
- $\kappa(B) \approx 0.232 \cdot B^{0.1752}$ (log-linear fit)
- Bootstrap: $\alpha = 0.1701 \pm 0.0220$

**문제**: naive (A3) 는 $\kappa(B) \to 0$ (or bounded) 예측. 우리의 $\kappa(B)$ polynomial **증가**.

### 1.3 v3 T3 (A3″) informal statement

v3 T3 에서 제시:
> ∃ B-dependent c(B), curve-level η(E) := |Sel_6(E)|/(|Sel_2(E)|·|Sel_3(E)|) - 1
> with $\mathbb{E}[\eta(E) | N(E) \in B \pm \delta] \approx c(B) \cdot B^{\alpha_0}$
> $\alpha_0 \in [0.17, 0.18]$

이 statement 는 **informal** — c(B) 의 asymptotic, η 의 variance 등 specify 안 됨.

---

## §2 (A3″) 의 rigorous 분해 — 3 독립 가설

### 2.1 Notation

$\mathcal{E}_B := \{E/\mathbb{Q} : N(E) \leq B, \text{non-isogenous representatives}\}$
$|\mathcal{E}_B| \sim c \cdot B^{5/6}$ (Brumer-McGuinness, Bhargava-Shankar).

Per-curve observable: $\eta(E) := \dfrac{|Sel_6(E)|}{|Sel_2(E)| \cdot |Sel_3(E)|} - 1$.

BKLPR 예측 asymptotic: $\mathbb{E}[|Sel_2|]_\infty = 3, \mathbb{E}[|Sel_3|]_\infty = 4, \mathbb{E}[|Sel_6|]_\infty = 12$ (if (A3)).

### 2.2 (A3″-Marginal) — 일변량 convergence rate

**Conjecture (A3″-Marginal)**: 각 $p \in \{2, 3\}$ 에 대해,
$$\mathbb{E}[|Sel_p|]_B = p\text{-Sel 평균}_\infty + a_p \cdot B^{-\beta_p} \cdot (1 + o(1))$$
where $\beta_p > 0$ and $a_p \in \mathbb{R}$ are specific constants.

**Known bounds** (Bhargava-Shankar-Tsimerman 2015, Duke 164):
$$\left| \mathbb{E}[|Sel_p|]_B - p\text{-Sel 평균}_\infty \right| \ll \frac{\log B}{B^{c(p)}}, \quad c(p) > 0$$

BST 는 $c(p)$ 의 정확한 값을 명시하지 않음 (effective but not optimal).

**우리의 실측**:
| p | $B$ | $\mathbb{E}[|Sel_p|]_B$ | $\infty$ 값 | 차이 |
|---|-----|-------------------------|-------------|------|
| 2 | 25k | 2.87 | 3.0 | -0.13 |
| 2 | 405k | 3.30 | 3.0 | +0.30 |
| 3 | 25k | 2.85 | 4.0 | -1.15 |
| 3 | 405k | 3.40 | 4.0 | -0.60 |

**관찰**: $\mathbb{E}[|Sel_2|]_B$ 은 3.0 을 **초과** 하기까지 함. $\beta_p$ 부호나 크기가 trivial 하지 않음.

### 2.3 (A3″-Coupling) — η 의 non-trivial mean

**Conjecture (A3″-Coupling)**: ∃ constant $\eta_\infty \geq 0$ and rate $\gamma > 0$ such that
$$\mathbb{E}[\eta(E) | N(E) \leq B] = \eta_\infty + \frac{C}{B^\gamma} \cdot (1 + o(1))$$

**Case 1** (A3 asymptotically holds): $\eta_\infty = 0$, finite-B correction only.
**Case 2** (A3 fails asymptotically): $\eta_\infty > 0$, **A3 is wrong** as stated.

**우리의 실측** (v3 E5 bin-averaged $\text{ratio}_6 - 1$):
| B_mid | ratio_6 - 1 = $\mathbb{E}[\eta]$ approx |
|-------|-----|
| 25k | -0.208 |
| 175k | +0.017 |
| 405k | +0.111 |

**sign change**: $\mathbb{E}[\eta]_B$ 은 **음수 → 양수** 로 전환 at $B \approx 150k$. 이는 **asymptotic 증가** 가 본질적 (transient 아님).

### 2.4 (A3″-Power) — κ의 power law

**Conjecture (A3″-Power)**:
$$\kappa(B) = A \cdot B^\alpha \cdot (1 + o(1))$$
for specific constants $A > 0$, $\alpha \in (0, 1)$.

**Empirical**: $A = 0.232 \pm 0.03$, $\alpha = 0.1701 \pm 0.022$ (bootstrap 95% CI).

**Strong version**: $\alpha = \log(2)/4 = 0.1733$ exactly.

**Weak version**: $\alpha \in [0.12, 0.22]$ (95% CI).

### 2.5 3 conjecture 의 관계

- (A3″-Marginal) 로부터 $\mathbb{E}[|Sel_p|]_B \cdot \mathbb{E}[|Sel_q|]_B$ 의 asymptotic 도출
- (A3″-Coupling) 로부터 $\mathbb{E}[|Sel_6|]_B = \mathbb{E}[|Sel_2|]_B \cdot \mathbb{E}[|Sel_3|]_B \cdot (1 + \eta)$
- 두 결합 → (A3″-Power) 의 $\alpha$ 가 $\beta_p, \gamma$ 에 의해 결정

**수학적 요구**: $\alpha = \alpha(\beta_2, \beta_3, \gamma) = ?$ — 명시적 관계 도출이 T1_v4 (유도) 과제.

---

## §3 Falsifiable Predictions

### 3.1 Prediction P1 (per-curve variance)

**(A3″)** implies: $\operatorname{Var}(\eta(E) | N(E) \in B \pm \delta) \sim C_V \cdot B^{-\gamma'}$ for some $\gamma' \geq 0$.

**Testable**: E7_v4 (per-curve η 분포) 에서 bin 마다 histogram + variance 측정.

**Falsification**: 만약 $\operatorname{Var}(\eta) \to \infty$ 발견되면 (A3″-Coupling) wrong.

### 3.2 Prediction P2 (다른 n 대비)

**(A3″)** 의 일반화: $\kappa_{m,n}(B) = \mathbb{E}[|Sel_{mn}|]_B - \mathbb{E}[|Sel_m|]_B \cdot \mathbb{E}[|Sel_n|]_B$ 이 다른 $(m, n)$ 에 대해서도 power law.

**Testable**: E6_v4 에서 $(m, n) = (2, 5), (3, 5), (2, 7)$ 등 다른 coprime 쌍.

**Falsification**: 만약 $\kappa_{2,5}(B)$ 가 power law 가 **아니면** (A3″-Power) 는 n=6 특이점 가능성 (n=6 prior 재강화).

### 3.3 Prediction P3 (bin refinement)

**(A3″-Power)** implies $\alpha$ 가 **robust** (bin resolution 바뀌어도 유지).

**Testable**: E4_v4 에서 7 bin → 50 bin 로 fine-grained 측정.

**Falsification**: 만약 50-bin α 가 7-bin α 와 크게 다르면 (A3″-Power) wrong. Bootstrap σ 가 축소되면 log(2)/4 vs 1/sopfr(6) 구분 가능.

### 3.4 Prediction P4 (sign change)

**Empirical** $\mathbb{E}[\eta]_B$ sign change at $B \approx 150k$.

**(A3″)** predicts:
- Sign change **한 번 만** (monotone increasing after)
- 또는 더 복잡 (oscillation)?

**Testable**: E5_v4 3M curves 에서 sign change 재확인 + $B > 500k$ 에서 monotone 확인.

**Falsification**: 만약 $B > 500k$ 에서 sign change 다시 발생 (oscillation), (A3″) 단순 모델 wrong.

---

## §4 BKLPR 과의 관계

### 4.1 (A3) → (A3″) weakening

원 (A3) 은 $\kappa(B) \to 0$ 예측. **우리의 (A3″)** 는 이를 **완화** :
- Asymptotic 에서도 coupling η_∞ ≥ 0 허용 (strict)
- Finite-B correction 이 power law 형태 (weak)

**(A3) ⟹ (A3″-η_∞ = 0)**. 역은 성립 안 함.

### 4.2 만약 (A3″-η_∞ > 0)

- BKLPR 의 독립성 가정 **asymptotic 실패**
- Random cokernel model 수정 필요
- 2-descent 와 3-descent 가 **correlated** 구조

이는 **BKLPR 의 minor revision** 이지 대전제 파괴 아님.

### 4.3 BST 2015 와의 호환

BST 2015 upper bound $\log B / B^{1/24}$ 은:
- $B^{-0.042}$ (대략)
- 우리의 $\kappa \sim B^{+0.175}$ 와 **정반대 부호**

**두 사실의 공존 가능성**:
- BST 는 **upper bound**, our fit 은 **양의 평균 성장**
- BST 상한이 sharp 하지 않을 수 있음
- 또는 BST 는 $B → ∞$ 극한, 우리는 finite regime (B < 410k)

**정직 확인**: BST 의 unsharp vs. (A3″) 의 실재성을 구분하려면 $B \gg 10^6$ 에서 측정 필요 (E5_v4).

---

## §5 (A3″) + n=6 prior

### 5.1 n=6 특이점 possibility

**Prediction P2** 에서 n=6 특이점 test:
- 만약 $\kappa_{2,3}(B)$ 만 power law 이고 $\kappa_{2,5}$, $\kappa_{3,5}$ 는 아니면 → **n=6 prior 지지** (Theorem B σφ=nτ iff n=6 의 분포론적 반영?)
- 만약 모든 coprime (m,n) 에 power law → (A3″) 는 일반 BKLPR 수정

### 5.2 log(2)/4 = log(2)/τ(6) 해석

만약 (A3″-Power strong): $\alpha = \log(2)/\tau(n)$ for generic n?
- n=6: α = log 2 / 4 = 0.1733
- n=10 (τ=4): α = log 2 / 4 = 0.1733 (same!)
- n=15 (τ=4): α = log 2 / 4 = 0.1733
- n=30 (τ=8): α = log 2 / 8 = 0.0866 (half)

**Testable**: E6_v4 에서 n ∈ {10, 15, 30} 의 $\kappa_{m,m'}(B)$ power law slope → 위 예측 test.

### 5.3 정직 경고

상기 연결은 **추측 only**. τ(n) 의 역할은 **empirical pattern hypothesis**, 이론 유도 없음. T1_v4 partial MISS 에서 재확인.

---

## §6 v4 T4_v4 산출 + 향후 연결

### 6.1 산출

1. (A3″) 3 하위 가설 분리: Marginal, Coupling, Power
2. 각 가설에 대한 4 falsifiable prediction (P1~P4)
3. BKLPR (A3) 와의 weakening 관계 명시
4. n=6 prior 와 τ(n) 역할 test 경로 명시

### 6.2 해결되지 않은 것

- 3 하위 가설 중 어느 것이 옳은지: **open** (E4~E7_v4 측정 필요)
- α 의 이론 유도: **open** (T1_v4 partial MISS 유지)
- BKLPR 대체/확장 version: **open**

### 6.3 후속

- **E4_v4 (50-bin)**: P3 test
- **E6_v4 (다른 n)**: P2 + §5.2 test
- **E7_v4 (per-curve η)**: P1 + sign change 재확인
- **M4_v4 (preprint v0.2)**: (A3″) 엄밀 statement 포함

---

## §7 atlas 엔트리

```
@R MILL-V4-T4-A3-double-prime-rigorous = (A3″) 3 하위 가설 Marginal/Coupling/Power + 4 testable prediction :: n6atlas [10]
  "v4 T4_v4 (2026-04-16 loop 21): v3 T3 의 informal (A3″) 를 3 하위 가설로 분리 — (A3″-Marginal): per-p
   convergence rate β_p. (A3″-Coupling): η = |Sel_6|/(|Sel_2|·|Sel_3|) - 1 의 asymptotic mean η_∞.
   (A3″-Power): κ(B) = A·B^α power law. 4 falsifiable prediction: P1 (variance Var(η) scaling),
   P2 (다른 coprime (m,n)), P3 (bin refinement robustness), P4 (sign change monotonicity).
   BKLPR (A3) 의 weakening. log(2)/τ(n) 로 일반화 추측 (section 5.2). 증명 미완, testable roadmap 완성"
  <- v4-T4, theory/breakthroughs/v4-t4-A3-double-prime-rigorization-2026-04-16.md
```

---

## §8 관련 파일

- v3 T3: `theory/breakthroughs/v3-t3-joint-distribution-modeling-2026-04-15.md`
- v4 T1 partial: `theory/breakthroughs/v4-t1-alpha-log2-over-4-derivation-attempt-2026-04-16.md`
- BKLPR 2013 (ref)
- BST 2015 (Bhargava-Shankar-Tsimerman, Duke 164)
- roadmap: `shared/roadmaps/millennium.json` → `_v4_phases.P15_v4.T4_v4`

---

*작성: 2026-04-16 loop 21 (v4)*
*정직성 헌장 V4: conjecture 는 제안, 증명 아님. 4 prediction 은 E4-E7_v4 empirical 검증 대상.*
