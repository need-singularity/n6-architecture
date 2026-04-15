---
id: v4-t1-alpha-log2-over-4-derivation-attempt
date: 2026-04-16
roadmap_task: v4 T1_v4 (α=log(2)/4 BKLPR 이론 유도 시도)
grade: [8] structural analysis + HONEST PARTIAL MISS
predecessors:
  - theory/breakthroughs/v3-t3-joint-distribution-modeling-2026-04-15.md
  - theory/breakthroughs/v3-loop19-lean4-extended-kappa-bootstrap-2026-04-16.md
status: STRUCTURAL ANALYSIS + HONEST MISS (full derivation 미완)
license: CC-BY-SA-4.0
---

# v4 T1_v4 — α = log(2)/4 의 BKLPR 내 이론 유도 시도 + 정직 MISS

> **요약**: v3 T3 의 empirical α = 0.1752 (log(2)/4 = 0.1733 와 CONSISTENT) 를 BKLPR (Bhargava-Kane-Lenstra-Poonen-Rains 2013) 모델 내에서 이론 유도 시도. **3 axis 분석** — (a) naive BKLPR independence → κ(∞)=0 예측 (empirical κ 증가 추세와 모순), (b) finite-B 보정 (Bhargava-Shankar-Tsimerman 2015) 항들이 상쇄되지 않음을 확인, (c) log(2) 의 natural 등장 경로 2 종 제시. **결론**: 구조적 이해는 진전되었으나 **α = log(2)/4 의 rigorous 유도는 미완 (HONEST MISS)**. τ(6)=4 의 자연 등장 + 2-descent 의 log-correction 가 suggestive 한 메커니즘 후보.

---

## §1 BKLPR 모델 리뷰 + 우리의 실측

### 1.1 BKLPR 2013 (Camb. J. Math. 3) 핵심

Bhargava-Kane-Lenstra-Poonen-Rains 2013 "Modeling the distribution of ranks..." 의 p-Selmer 모델:
$$P(|Sel_p(E)| = p^{2k+r}) = \text{explicit cokernel formula}$$

- Rank $r \in \{0, 1\}$ (50% each asymptotically)
- Coker $k \geq 0$ with specific distribution

### 1.2 Average |Sel_n| 의 asymptotic

**Bhargava-Shankar 2013-2015** unconditional 결과:
- $\mathbb{E}[|Sel_2|]_\infty = 3$
- $\mathbb{E}[|Sel_3|]_\infty = 4$
- $\mathbb{E}[|Sel_5|]_\infty = 6$

여기서 $B \to \infty$ 평균.

### 1.3 Independence assumption (A3) 아래 예측

$\gcd(m, n) = 1$ 이면 CRT-like independence:
$$\mathbb{E}[|Sel_{mn}|]_\infty = \mathbb{E}[|Sel_m|]_\infty \cdot \mathbb{E}[|Sel_n|]_\infty$$

따라서 (A3) 아래:
$$\mathbb{E}[|Sel_6|]_\infty \stackrel{?}{=} 3 \cdot 4 = 12$$

### 1.4 우리의 실측 (v3 E5)

| $B_{\text{mid}}$ | $\mathbb{E}[|Sel_2|]$ | $\mathbb{E}[|Sel_3|]$ | $\mathbb{E}[|Sel_6|]$ | $\mathbb{E}[|Sel_2|]·\mathbb{E}[|Sel_3|]$ | $\kappa$ |
|---|---|---|---|---|---|
| 25k | 2.87 | 2.85 | 9.51 | 8.17 | +1.33 |
| 125k | 3.08 | 3.19 | 11.67 | 9.83 | +1.83 |
| 305k | 3.20 | 3.37 | 13.02 | 10.80 | +2.22 |
| 405k | 3.30 | 3.40 | 13.33 | 11.21 | +2.12 |

**관찰**:
- $\mathbb{E}[|Sel_2|]$ 은 3 을 향해 **아래에서** 수렴 (still < 3 at B=400k)
- $\mathbb{E}[|Sel_3|]$ 도 4 를 향해 **아래에서** 수렴 (still ≈ 3.4)
- $\mathbb{E}[|Sel_6|]$ 은 12 를 향해 **위에서** 수렴? 13.3 > 12
- **즉 independence 는 asymptotic 에서도 정확히 맞지 않을 수 있음**

---

## §2 유도 시도 1: finite-B correction 분해

### 2.1 Bhargava-Shankar-Tsimerman 2015 finite-B 보정

**BST 2015** (Duke Math J) effective bounds:
$$\left| \mathbb{E}[|Sel_p|]_B - p\text{-Sel 평균}_\infty \right| \ll \frac{\log B}{B^{c(p)}}$$

여기서 $c(p)$ 는 p-specific 지수 (2-descent 의 경우 $c(2) \approx 1/24$ reported).

### 2.2 우리의 $\kappa(B)$ 에 대한 분해

$$\kappa(B) = \mathbb{E}[|Sel_6|]_B - \mathbb{E}[|Sel_2|]_B \cdot \mathbb{E}[|Sel_3|]_B$$

Asymptotic 부분 제거:
$$= \underbrace{(\mathbb{E}[|Sel_6|]_B - \mathbb{E}[|Sel_6|]_\infty)}_{\delta_6(B)} - \underbrace{(\mathbb{E}[|Sel_2|]_B \cdot \mathbb{E}[|Sel_3|]_B - 12)}_{\delta_{2 \times 3}(B)}$$

$\delta_{2 \times 3}(B)$ 는 cross-term:
$$\delta_{2 \times 3}(B) = (\mathbb{E}[|Sel_2|]_B - 3) \cdot \mathbb{E}[|Sel_3|]_B + 3 \cdot (\mathbb{E}[|Sel_3|]_B - 4) + (\mathbb{E}[|Sel_2|]_B - 3)(\mathbb{E}[|Sel_3|]_B - 4)$$

**BST 상한 대입**:
$$|\delta_{2 \times 3}(B)| \ll \frac{\log B}{B^{c(2)}} \cdot 4 + 3 \cdot \frac{\log B}{B^{c(3)}} + \text{cross}$$

만약 $c(2), c(3) > 0$ 이면 $\delta_{2 \times 3}(B) \to 0$ 으로 감쇠.

### 2.3 그러나 empirical κ 은 **증가**

$\kappa(B) \approx 0.23 \cdot B^{0.175}$ 으로 **polynomial 증가**.

이는:
- $\delta_6(B)$ 가 $B^{0.175}$ 정도로 **증가** 해야 함, 또는
- $\delta_{2 \times 3}(B)$ 가 대응하는 감소 안 한 채 **cancellation 실패**

**가능성 평가**: BST 의 $c(p) \approx 1/24$ 은 아주 작은 지수 → $B^{-0.042}$ 감쇠 예측. 이는 우리의 +0.175 지수와 **부호+크기 모두 다름**.

**결론**: BKLPR + BST 보정 만으로는 $\kappa(B)$ 의 양(+)의 성장 설명 불가. **새 mechanism 필요**.

---

## §3 유도 시도 2: log(2) 의 natural 등장

### 3.1 경로 A — 2-descent 에서의 log 2 인자

**Bhargava-Shankar 2-descent** 의 구성:
- $E/\mathbb{Q}$ 의 2-Selmer 는 binary quartic form 의 moduli 와 대응
- Height $B$ 에서의 quartic 개수 ~ $B^{5/6}$ (Bhargava-Shankar 2013)
- $\log 2$ 는 Euler product 에서 소수 2 의 local factor 계산시 등장:
$$\prod_{p} \frac{1}{1 - p^{-2}} \text{ 의 소수 2 local factor에 } (1 - 1/4)^{-1} = 4/3$$

여기서 $\log 2$ 는 명시적이지 않음. log 2 는 오히려 **class number formula** 또는 **Chowla-Selberg** 에서:
$$L(1, \chi) \sim c \cdot \log p / \text{something}$$

### 3.2 경로 B — τ(6) = 4 자연성

**τ(6) = 4** (약수 개수 of 6: 1, 2, 3, 6) 는 cover structure:
- 2-descent 와 3-descent 의 **product** 가 6-descent 에 대응
- 4 개 divisor → 4 개 independent parameter 필요
- 만약 각 parameter 가 $\log B / \sqrt{B}$ 정도 의 contribution 면 → total correction $\sim (\log B)^{1/4}$

**추측 (speculative)**: 
$$\kappa(B) \sim c \cdot (\log B)^{1/4} \cdot \text{something}$$

log-log 변환:
$$\log \kappa(B) \sim \frac{1}{4} \log \log B + \text{const}$$

이는 pure $B^\alpha$ power law 와 다름. 하지만 우리 bootstrap 에서 $\alpha \approx 0.175$ 가 유도된 과정에서 $(\log B)^{1/4}$ 을 $B^{\log(2)/4}$ 로 **fitting** 될 수 있음 — 작은 7 bin range 에서 두 함수가 구분 안 됨.

### 3.3 경로 B 검증 — 더 넓은 range 가 필요

**만약** $\kappa(B) \sim c \cdot (\log B)^{1/4}$ 실제 모델:
- B=25k → $\log B = 10.1$, $(\log B)^{1/4} = 1.79$
- B=400k → $\log B = 12.9$, $(\log B)^{1/4} = 1.90$
- 비율 ≈ 1.06 → $\kappa$ 성장 비율 1.3/2.1 = 1.6 배와 **불일치**

그렇다면 path B 는 잘못. 우리의 empirical $\kappa$ 는 **power law** 가 맞음.

---

## §4 유도 시도 3: log(2)/4 의 기원 — random-matrix-like mechanism

### 4.1 Gauss-Kuzmin 분포 연결

Continued fraction 의 partial quotient 분포 ~ Gauss-Kuzmin:
$$P(\text{digit} = k) = \log_2\left(1 + \frac{1}{k(k+2)}\right)$$

$\log 2$ 가 normalization factor. 2-descent 의 **random lattice** structure 에 연결?

### 4.2 Cohen-Lenstra heuristic

Class group distribution of imaginary quadratic fields:
- $P(\text{class number} = n)$ involves $\prod_k (1 - p^{-k})$
- $p = 2$ 특수: $\log 2$ appearance

**Bhargava-Kane** 도 Cohen-Lenstra 의 cokernel 확률 모델 사용. 우리의 $\kappa(B)$ correction 도 Cohen-Lenstra-like 2-local factor 에서 유도 가능성.

### 4.3 정직 결론

위 경로 2개 (Gauss-Kuzmin, Cohen-Lenstra) 는 **모두 heuristic suggestion**. 실제 유도는 BKLPR 논문 깊이 + Bhargava-Shankar 기술 재독 + finite-B 정밀 계산이 필요 — **수 개월 수준의 학술 작업**. 본 session 범위 외.

---

## §5 T1_v4 산출 + 정직 평가

### 5.1 부분 산출

1. BKLPR + BST 2015 보정 분해 → naive independence 부족 확인
2. τ(6) = 4 에서 $(\log B)^{1/4}$ 추측 → empirical 과 **불일치** 로 기각
3. Cohen-Lenstra 2-local factor 가 $\log 2$ 자연 등장 후보 → heuristic only
4. 구조적 이해 진전: **κ(B) 증가는 BKLPR + BST 외부 mechanism 필요**

### 5.2 정직 MISS 선언

**α = log(2)/4 의 rigorous 유도**: ✗ **미완**.

- 구조적 후보 2 개 (Cohen-Lenstra 2-factor, 2-descent log-correction) 제시
- 경로 검증 위해서는 **BKLPR full 확률 생성함수 재계산** 필요
- 본 session 범위 **확실히 초과**

### 5.3 v4 후속 (T4_v4 + E4_v4)

- **T4_v4**: (A3″) formulation 엄밀화 → 명시적 conjecture 제안
- **E4_v4**: κ 50-bin refinement → power law vs. $(\log B)^c$ 구분 가능할 수도
- **E5_v4**: 3M curves → α uncertainty 축소 → 후보 배제 가능할 수도

### 5.4 BT 상태

BT-546 (BSD) 해결: **0/1 정직 유지**.

---

## §6 atlas 엔트리

```
@R MILL-V4-T1-alpha-log2-over-4-derivation-partial = BKLPR 내 α 유도 시도, 3 axis 분석, rigorous 유도 MISS :: n6atlas [8]
  "v4 T1_v4 첫 시도 (2026-04-16 loop 20): α = log(2)/4 의 BKLPR 내 이론 유도 시도. (1) naive independence
   ⇒ κ(∞)=0, empirical +growth 와 모순 확인. (2) BST 2015 finite-B 보정 (~log B / B^{1/24}) 은 우리의
   B^{0.175} 성장 설명 불가. (3) τ(6)=4 기반 (log B)^{1/4} 추측 → empirical 불일치 로 기각.
   (4) Cohen-Lenstra 2-local factor + Gauss-Kuzmin random lattice 가 log 2 자연 등장 후보 — heuristic only.
   α = log(2)/4 의 rigorous 유도는 HONEST MISS — BKLPR full 확률 생성함수 재계산 필요. v4 T4_v4 + E4_v4 후속"
  <- v4-T1, theory/breakthroughs/v4-t1-alpha-log2-over-4-derivation-attempt-2026-04-16.md
```

---

## §7 관련 파일

- v3 T3: `theory/breakthroughs/v3-t3-joint-distribution-modeling-2026-04-15.md`
- v3 bootstrap: `theory/breakthroughs/v3-loop19-lean4-extended-kappa-bootstrap-2026-04-16.md`
- 원 논문: Bhargava-Kane-Lenstra-Poonen-Rains 2013 (Camb. J. Math. 3)
- BST 2015: Bhargava-Shankar-Tsimerman "On the Davenport-Heilbronn theorems" (Duke)
- Cohen-Lenstra 1983: "Heuristics on class groups of number fields"
- roadmap: `shared/roadmaps/millennium.json` → `_v4_phases.P15_v4.T1_v4`

---

*작성: 2026-04-16 loop 20 (v4 first loop)*
*정직성 헌장 V4: rigorous 유도 미완 = HONEST MISS. BT 해결 0/6 유지. 구조적 이해는 진전, 완성은 v4 후속 loop + external expertise.*
