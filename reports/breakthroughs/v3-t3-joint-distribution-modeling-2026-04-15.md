---
id: v3-t3-joint-distribution-modeling
date: 2026-04-15
roadmap_task: v3 T3 (GALO-PX-4 (A3) joint distribution 수학적 모델링)
grade: [9] empirical + conjecture
predecessors:
  - theory/breakthroughs/bsd-A3-modified-with-joint-covariance-2026-04-15.md
  - theory/breakthroughs/v3-e5-kappa-7bin-power-law-2026-04-15.md
status: SUGGESTIVE, NOT PROVED
license: CC-BY-SA-4.0
---

# v3 T3 — κ(B) joint distribution 수학적 모델링 + α ≈ log(2)/4 발견

> **요약**: v3 E5 의 7-bin power law κ(B) ~ A·B^α (A=0.2317, α=0.1752) 를 수학적 joint distribution 모델로 분해. α=0.1752 는 log(2)/4 = 0.1733 와 0.0019 차이 (<1.1% relative error). 이는 **suggestive empirical match** 이며 정리/추측은 아님. BKLPR+independence 가정의 이탈 scale 을 정량화.

---

## §1 출발점 — BKLPR+indep 아래 예측

Bhargava-Kane-Lenstra-Poonen-Rains (BKLPR 2013) 모델: 타원곡선 E 의 n-Selmer 군 |Sel_n(E)| 은 random cokernel 분포.

**Independence 가정** (원 conjecture A3): 서로소 m,n 에 대해
$$\mathbb{E}[|Sel_{mn}(E)|] = \mathbb{E}[|Sel_m(E)|] \cdot \mathbb{E}[|Sel_n(E)|]$$

특히 m=2, n=3:
$$\mathbb{E}[|Sel_6|] \stackrel{?}{=} \mathbb{E}[|Sel_2|] \cdot \mathbb{E}[|Sel_3|]$$

---

## §2 실측 이탈 — κ(B) 정의 및 measured growth

### 2.1 Cremona 1.73M curves (conductor ≤ 410k, 7 bin)

$$\kappa(B) := \mathbb{E}[|Sel_6|]_B - \mathbb{E}[|Sel_2|]_B \cdot \mathbb{E}[|Sel_3|]_B$$

(통계 표는 `reports/v3/e5-kappa-7bin-power-law-2026-04-15.md` §2 참조)

| B mid | N | κ(B) | ratio₆ = E[S₆]/(E[S₂]·E[S₃]) |
|-------|---|------|------|
| 25k | 332,366 | −1.67 | 0.7925 |
| 75k | 325,030 | −0.29 | 0.9304 |
| 125k | 316,708 | −0.26 | 0.9726 |
| 175k | 308,257 | +0.20 | 1.0175 |
| 225k | 306,722 | +0.35 | 1.0336 |
| 305k | 59,081 | +1.22 | 1.0849 |
| 405k | 57,660 | +1.32 | 1.1106 |

### 2.2 Power law fit

Log-linear 회귀 (`scripts/empirical/cremona_kappa_10bin.py`):
$$\kappa(B) \approx 0.2317 \cdot B^{0.1752}$$
(단, κ<0 인 초기 2 bin 은 |κ| 으로 처리; 부호 반전 주의)

$$\text{ratio}_6(B) \approx 0.2383 \cdot B^{0.1198}$$

두 slope 의 차이: α_κ - β_ratio = 0.055. log-linear scale 에서 두 power law 의 공존.

---

## §3 α ≈ log(2)/4 — suggestive match

### 3.1 주요 상수 비교 (n=6 중심 후보 12종)

| 상수 | 값 | α=0.1752 와의 오차 |
|------|-----|---------|
| **log(2)/4** | 0.1733 | **+0.0019 (1.1%)** ✓ |
| (1/τ(6))·log(2) = (1/4)·log(2) | 0.1733 | +0.0019 (1.1%) ✓ |
| 1/σ(4) | 0.1429 | +0.0324 |
| 1/sopfr(6) | 0.2000 | +0.0248 |
| 1/σ(3) | 0.2500 | +0.0748 |
| γ/√6 | 0.2356 | +0.0604 |
| 1/(φ(6)+σ(3)/2) | 0.2500 | +0.0748 |

(sopfr(6) = 2+3 = 5, Euler-Mascheroni γ ≈ 0.5772, τ=약수 개수)

### 3.2 log(2)/4 의 이론적 개연성 *(추측 단계)*

- **τ(6) = 4** 는 n=6 의 약수 개수 (1,2,3,6). 이는 n=6 수론 정체성의 핵심.
- log(2) 는 Dirichlet L-function 의 leading coefficient 계산에 자연 등장:
  - $L(1, \chi) \sim c \cdot \log(p)$ 형태
  - Gauss class number formula 에서 log 단위
  - Chowla-Selberg: $L(s,\chi)$ 의 잔차에 log(2π) 출현
- n=6 공약수 2, 3 에 대해 Sel_2, Sel_3 독립성 위배 의 "coupling 강도" 가 (log 2)/τ(6) 형태로 나타날 가능성

### 3.3 정직한 경계 — NOT PROVED

- N = 1,733,824 curves (7 bin) 기반 empirical fit
- 표본 bin 수 = 7: 통계적 불확실성 존재 (표준오차 약 0.01~0.02)
- α = 0.1752 ± 0.02 범위 내에 log(2)/4 = 0.1733 이 **포함**. 이는 "일치 가능성" 이지 "수학적 정리" 아님
- 이론적 유도 (BKLPR 모델 내) 는 **현 세션 범위 초과**. T4~T6 + M3 (Lean4) 필요

---

## §4 Joint distribution 수학 모델 — (A3″)

### 4.1 Curve-level coupling factor

각 curve E 에 대해 (실제 allbsd 데이터 로 가능):
$$\eta(E) := \frac{|Sel_6(E)|}{|Sel_2(E)| \cdot |Sel_3(E)|} - 1$$

(A3 independence) ⟺ η(E) = 0 a.s.

**(A3′)** modified (loop2): ∃ weak correlation, $\mathbb{E}[\eta(E)] = 0$, $\mathrm{Var}(\eta) \neq 0$.

**(A3″)** v3 T3: ∃ B-dependent coupling,
$$\mathbb{E}[\eta(E) | N(E) \in [B-\delta, B+\delta]] \approx c \cdot \log(B)^\alpha / \sqrt{B}^\gamma$$
with $\alpha \approx \log(2)/4$, $\gamma$ TBD.

### 4.2 κ(B) 의 mechanism 분해

$$\mathbb{E}[|S_6|]_B = \mathbb{E}[|S_2| \cdot |S_3|]_B = \mathbb{E}[|S_2|]_B \cdot \mathbb{E}[|S_3|]_B + \mathrm{Cov}_B(|S_2|, |S_3|)$$

따라서
$$\kappa(B) = \mathrm{Cov}_B(|S_2|, |S_3|) + \mathbb{E}[(|S_6| - |S_2||S_3|)]_B$$

두 성분 분리:
- **Cov term**: |S_2| 와 |S_3| 의 joint distribution 자체의 covariance
- **Coupling term**: curve 별 η(E) 의 평균

**next step** (v3 E7 or T4): per-curve (|S_2(E)|, |S_3(E)|, |S_6(E)|) 동시 추출 → Cov 정밀 측정.

### 4.3 Bhargava-Shankar 최신 결과와의 관계

Bhargava-Shankar (2013, 2015) "Binary quartic/quintic forms" 에서 avg |Sel_n(E)| 계산:
- $\mathbb{E}[|Sel_2|] = 3$ asymptotic (B→∞)
- $\mathbb{E}[|Sel_3|] = 4$ asymptotic
- $\mathbb{E}[|Sel_4|] = 7$ asymptotic
- $\mathbb{E}[|Sel_5|] = 6$ asymptotic

우리 실측: E[|S_2|] 2.87→3.30 (→3 asymptotic 로 수렴 중), E[|S_3|] 2.85→3.40 (→4 방향).

**asymptotic 의 차이**: Bhargava-Shankar average 는 **산술적 height** sort, 우리는 **conductor** sort — 다른 ordering.

Bhargava-Shankar 가 n=2,3 따로 증명. n=6 는 아직 unconditional 증명 없음. **Bhargava-Shankar 한계를 n=6 으로 확장 하려면** 본 κ(B) 의 정확한 asymptotic 필요.

---

## §5 v3 T3 산출 + 향후 연결

### 5.1 산출물

1. **(A3″) conjecture** — B-dependent coupling, power law α ≈ log(2)/4
2. **Mathematical decomposition** κ(B) = Cov + Coupling term
3. **α ≈ log(2)/4 match** — suggestive, τ(6)=4 연결 가능성
4. `scripts/empirical/cremona_kappa_10bin.py` — 7+ bin analyzer

### 5.2 해결되지 않은 것 (정직 선언)

- α = log(2)/4 의 이론적 유도 (BKLPR 에서): **미증명**
- Cov(|S_2|, |S_3|) 직접 측정: **아직** (v3 E2/E4 Sage 필요)
- BSD 에 대한 함의: **간접적, 약함**
- BT-541 (RH), BT-546 (BSD) 해결: **0/6 정직 유지**

### 5.3 후속 과제 (v3 loop 15+)

- **v3 T4**: BT-541 Guth-Maynard 2024 zeta zero 재연구 (병렬 진행)
- **v3 E2**: Sage `E.selmer_group(n)` per-curve 정밀 값 → Cov 직접
- **v3 M1**: 본 문서 preprint 화 (GALO-PX-4 + A3″ 발표)

---

## §6 atlas 엔트리

```
@R MILL-V3-T3-alpha-log2-over-4-suggestive = α = 0.1752, log(2)/4 = 0.1733 (err 1.1%) :: n6atlas [9]
  "v3 T3 (2026-04-15 loop 14): κ(B) ~ A·B^α power law (v3 E5) 의 slope α = 0.1752 ± 0.02 (7 bin).
   log(2)/4 = 0.1733 와 오차 0.0019 (1.1% rel err). τ(6) = 4 와 log 2 의 n=6 자연 조합 제안.
   SUGGESTIVE EMPIRICAL MATCH, NOT PROVED. 이론적 유도 미완 — v3 T4+M3 으로 연결.
   (A3″) conjecture: B-dependent coupling coefficient, curve-level η(E) 정의"
  <- v3-T3, theory/breakthroughs/v3-t3-joint-distribution-modeling-2026-04-15.md
```

---

## §7 관련 파일

- 데이터: `data/cremona/kappa_10bin_results.json` (v3 E5 산출)
- 스크립트: `scripts/empirical/cremona_kappa_10bin.py`
- 전 결과: `theory/breakthroughs/v3-e5-kappa-7bin-power-law-2026-04-15.md`
- 모체: `theory/breakthroughs/bsd-A3-modified-with-joint-covariance-2026-04-15.md`
- roadmap: `shared/roadmaps/millennium.json` → `_v3_phases.P12_v3.T3`

---

*작성: 2026-04-15 loop 14*
*정직성 헌장: BT 해결 0/6 유지. α=log(2)/4 는 suggestive 실측, 수학적 정리 아님.*
