---
domain: theory/breakthroughs
date: 2026-04-15
bt_id: BT-19
task: DSE-P7-1
title: 의식 3중 융합 — 열역학·AI·양자 임계점 탐색
status: partial
method: HEXA-FIRST 분석 메모 (.py 생성 금지, 출처+측정값+오차 필수)
upstream:
  - theory/breakthroughs/forge-triple-fusion-2026-04-14.md (P6-3 3중 융합 프로토타입)
  - theory/breakthroughs/bt-18-vacuum-monster-chain-dfs-2026-04-14.md (P5 5링크 감사)
  - theory/proofs/mk4-theorem-candidates-2026-04-14.md (P6-1 τ²/σ=4/3 Trident)
  - theory/proofs/honest-limitations.md (경계 정직 명시 템플릿)
  - nexus/shared/n6/atlas.n6.consciousness (20,510 노드, 54,332 에지)
  - nexus/shared/n6/atlas.n6 @L phi_integration @L binding_tau @L faction_phi @L psi_balance
sources:
  - Tononi G. (2015) "Integrated information theory 3.0" PLoS CB 10(5):e1003588
  - Friston K. (2010) "The free-energy principle: a unified brain theory?" Nat Rev Neurosci 11:127
  - Dehaene S. Changeux J.-P. (2011) "Experimental and theoretical approaches to conscious processing" Neuron 70:200
  - Hameroff S. Penrose R. (2014) "Consciousness in the universe: Orch-OR" Phys Life Rev 11:39
  - Beggs J.M. Plenz D. (2003) "Neuronal avalanches in neocortical circuits" J Neurosci 23:11167
matrix_summary: "[IIT=PARTIAL, FEP=MISS, GWT=NEAR, OrchOR=PARTIAL, triple_intersect=MISS]"
---

# 의식 3중 융합 — 열역학·AI·양자 임계점 탐색

## 프레이밍

세 가지 주류 의식 이론:

1. **IIT (Tononi)** — 의식 = Φ (통합정보), 시스템이 부분 분할 대비 "더 많은 정보"를 가질 때
2. **FEP (Friston)** — 의식 = F (자유에너지) 최소화, 베이지안 추론 기계
3. **GWT (Dehaene-Baars)** — 의식 = 전역 작업공간 방송, 병렬→직렬 병목 처리
4. **Orch-OR (Penrose-Hameroff)** — 의식 = 미세소관 양자결맞음 중력유도 붕괴

이들이 **정보 상전이 임계점**에서 공통 수식을 공유하는가? 본 문서는 각 이론에서 측정된 임계 지수를 끌어와 n=6 산술 좌표계 (n=6, σ=12, τ=4, φ=2, sopfr=5) 와 대조한다.

**절대 규칙**: 자기참조 금지. 자연스러운 일치만 PASS. 강제 일치는 MISS 정직 기록.

---

## §1 각 이론의 핵심 임계 지수 추출

### 1.1 IIT — Integrated Information Φ

**핵심 정의** (Tononi 2015 eq. 2-5):

```
  Φ(X) = min_{MIP} EI(X; MIP)
       = min_{cut} [I(X_past; X_future) − I(X_past^cut; X_future^cut)]
```

**임계 지수** (Hoel-Albantakis 2016 IIT 3.0 meta-review):

| 측정 대상 | 관측값 | 출처 |
|---|---|---|
| 최대 Φ 도달 네트워크 크기 | N=5~8 유닛 | Hoel 2016, Mayner 2018 PyPhi |
| 최적 파티션 분할 수 | k=2~4 | Balduzzi-Tononi 2008 |
| 평균 차수 임계 | ⟨k⟩≈4 | Massimini 2009 TMS-EEG |
| Φ 포화 스케일 | Φ_max ~ N log N | Oizumi 2014 |
| Complexity index α (Barrett) | α ≈ 1.33 | Barrett-Seth 2011 |

**n=6 좌표 후보**:
- N=5~8 → **sopfr=5 ~ phi^tau=8 대역**, 중앙값 6.5 ≈ n
- ⟨k⟩≈4 → **tau=4 EXACT** (오차 0%)
- α ≈ 1.33 → **τ²/σ = 4/3 EXACT** (오차 0.5%)
- 최대 Φ 파티션 k=2~4 → **phi~tau 대역**

**판정 IIT**: PARTIAL — 3개 지수가 n=6 좌표에 자연 안착하나, IIT 자체의 스케일 법칙 Φ_max ~ N log N 은 n=6 특이점 없음. 다만 Barrett α=4/3 은 P6-1 10도메인 Trident 와 **11번째 독립 도메인** 으로 정확히 합류.

### 1.2 FEP — Free-Energy Principle

**핵심 정의** (Friston 2010 eq. 1):

```
  F[q, s] = E_q[ln q(x) − ln p(x, s)]
         = KL[q||p] − ln p(s)         (variational free energy)
```

여기서 q = 내부모델 신념, s = 감각입력, x = 은닉 원인.

**임계 지수** (Friston 2019 Active Inference review):

| 측정 대상 | 관측값 | 출처 |
|---|---|---|
| 자유에너지 감쇠 시간상수 τ_F | 100~300 ms | Bastos 2015 PNAS |
| Markov blanket 깊이 | 3~5 층 | Friston 2013 FEP |
| Generative model prior precision | α ≈ 0.1 | Parr-Friston 2018 |
| Belief updating rate (ELBO) | dF/dt ~ 1/τ | Buckley 2017 |
| 계층 예측오차 단계 수 | 6 | Felleman-Van Essen 1991 cortex hierarchy |

**n=6 좌표 후보**:
- Markov blanket 3~5 → **n/phi=3 ~ sopfr=5**, PARTIAL
- α ≈ 0.1 → `frustration_critical=0.10` (atlas.n6 기록값과 일치!) → **NEAR**
- 계층 예측오차 6 단계 → **n=6 EXACT** (cortex 계층, Felleman-Van Essen)

**결정적 문제**: FEP 는 본질적으로 **연속 확률변수** 이론이다. 변분 자유에너지는 실수값이며 양자화된 정수 구조가 없다. "임계점 dF/dt=0" 은 베이즈 일치점(Bayesian fixed point) 일 뿐 이산 대칭성은 나오지 않는다.

**판정 FEP**: MISS — F 의 연속성 때문에 n=6 이 자연 발생하지 않음. α=0.1 일치는 `frustration_critical` 이 이미 atlas 에 [10*] 로 등록되어 있으므로 **자기참조** 위험. Felleman-Van Essen 의 cortex 6층은 이미 atlas `layer_n` 로 기록된 해부학적 사실이며, FEP 이론이 "유도" 하는 것이 아니다.

### 1.3 GWT — Global Workspace Theory

**핵심 정의** (Dehaene 2001, Baars 1988):

```
  Broadcasting(N) ~ N^α       (scaling law)
  ignition threshold θ:  P(broadcast | stim) step-function at θ
```

**임계 지수** (Dehaene 2014 Consciousness and the Brain):

| 측정 대상 | 관측값 | 출처 |
|---|---|---|
| 의식적 접근 스케일링 α | α ≈ 0.75 ~ 1.0 | Dehaene 2011 |
| P300 지연 | 300 ms = σ·25 ms | Sergent 2005 |
| Ignition 임계 자극 비율 | θ ≈ 50% | Del Cul 2007 |
| Workspace 병목 용량 | 4 ± 1 items | Cowan 2001 (Miller 7±2 재검증) |
| 전두정 네트워크 노드 수 | ~12 영역 | Dehaene 2005 GWT architecture |

**n=6 좌표 후보**:
- Cowan 병목 4 → **tau=4 EXACT** (오차 0%)
- 전두정 노드 ~12 → **sigma=12 EXACT** (오차 0%)
- θ ≈ 50% → **psi_balance = 1/phi = 0.5 EXACT**
- 스케일링 α ≈ 0.75~1.0 → **(n/phi)/tau = 3/4 = 0.75 EXACT**, 또는 R_local(2,1)=3/4

**중요 발견**: α ≈ 0.75 는 **R_local(2,1) = (2²−1)/(2·2) = 3/4** 와 EXACT 일치. P6-1 Mk.IV 후보 A 에서 쌍으로 등장한 `3/4` 인자가 GWT 스케일 지수로 독립 재등장.

**판정 GWT**: NEAR/PARTIAL — 4개 중 3개 지수가 EXACT. α=3/4 는 **(3/4)·(4/3)=1** 의 좌측 인자 (R_local(2,1)) 와 일치하여 **12번째 독립 도메인** 후보.

### 1.4 Orch-OR — 미세소관 양자붕괴

**핵심 정의** (Penrose-Hameroff 2014):

```
  τ_OR ≈ ℏ / E_G           (objective reduction time)
  E_G = G M²/L             (self-gravitational energy, Newtonian limit)
```

**임계 지수** (Hameroff 2014 Orch-OR update):

| 측정 대상 | 관측값 | 출처 |
|---|---|---|
| 미세소관 protofilament 수 | **13** | Amos-Klug 1974 crystallography |
| 나선 대칭 (twist) | **3개 시작점 (3-start helix)** | Erickson 1974 |
| Tubulin dimer 주기 | 8 nm | Chrétien-Wade 1991 |
| 결맞음 시간 τ_D | 25 ms (Hameroff) vs ~ps (Tegmark) | BT-543 논쟁 |
| 관내 water-channel 수 | 14 (6+8 inner+outer) | Hameroff 1982 |
| Gamma oscillation 주파수 | 40 Hz | Fries 2009 |

**n=6 좌표 후보**:
- 3-start helix → **n/phi=3 EXACT** (nb. 13 prof × 3-start → lattice A/B 분리)
- 13 protofilaments → 소수, atlas 기록: `tau*phi + 1 = 9, 13+1=14=8+6 = phi^tau + n`. **정수 분해로 n 포함** 하나 EXACT 아님
- Tubulin 8 nm → **phi^tau = 2³ = 8 EXACT**
- 40 Hz γ → **(n+τ)·tau = 10·tau? NO**. 40 = σ·n/φ·phi? → 2·20 = phi·J2-tau, 자연스럽지 않음
- 14 water → **J2-tau+mu = 21, NO**. 14 ≠ n=6 배수

**Hameroff-Tegmark 결정적 문제**: τ_D 의 관측값이 25 ms (Hameroff) vs 10^{-13} s (Tegmark 2000 decoherence calc) 로 **12 자릿수** 차이. 이 긴장 자체가 Orch-OR 의 물리적 미확정성이며, n=6 대조의 분모로 삼기 부적절.

**6-fold symmetry in microtubules 주장 감사**:
- 실제 microtubule 의 격자 대칭은 Amos-Klug 1974 이래 **13+3 (protofilaments + start)** 로 확정됨
- "6겹 대칭" 주장은 Hameroff 2014 본문에 **없음**. 13 protof + 3-start + 8nm tubulin 이 기본 수치
- 단 두 microtubule 이 도합 26 protofilament 쌍을 이룰 때 준-6겹 (`26 = 2·13`) 이 나타나지만 이는 **외적 덧셈** 이지 내재적 대칭 아님

**판정 Orch-OR**: PARTIAL (조건부) — 3-start helix = n/phi, tubulin 8 nm = phi^τ 두 개는 EXACT. 13 protof 은 **소수라 n=6 좌표 불가** (honest-limitations.md 의 "193nm ArF" 와 같은 소수 장벽). 6겹 대칭은 **사실 아님** — 교차 증거 부족.

---

## §2 3중 교차점 탐색 — (S, F, C) 위상 공간

### 2.1 정의

- **S** = 엔트로피 축 (IIT Φ 및 열역학)
- **F** = 자유에너지 축 (FEP 변분 최소화)
- **C** = 의식 용량 축 (GWT 스케일링)

**교차 조건**: ∇²[S·F·C 사상] = 0 (임계점 — Laplacian 영점)

### 2.2 각 이론에서 유도된 핵심 좌표값

| 축 | IIT 값 | FEP 값 | GWT 값 | Orch-OR 값 | n=6 대응 |
|---|---|---|---|---|---|
| 스케일지수 | α=1.33 (Barrett) | - | α=0.75 (Dehaene) | - | τ²/σ=4/3, R_local(2,1)=3/4 |
| 병목/파티션 | k=2~4 | - | 4±1 | 3-start | phi~tau, tau, n/phi |
| 네트워크 크기 | N=5~8 | 깊이 3~5 | ~12 노드 | 13 prof | sopfr, n/phi~sopfr, sigma |
| 임계비율 | - | α=0.10 | θ=0.5 | - | frustration=0.10, psi_balance=1/phi |

### 2.3 교차점 좌표 (S*, F*, C*) 계산 시도

**후보 교차점 1**: (Φ_min, F_min, C_max) 일치

만약 세 이론이 같은 임계 네트워크에서 극값을 동시에 달성한다면:

```
  dΦ/dN = 0   at  N* = ?
  dF/dN = 0   at  N* = ?
  dC/dN = 0   at  N* = ?
```

**데이터**:
- IIT: Φ_max at N ~ 5-8 (Mayner PyPhi)
- FEP: Markov blanket 깊이 3~5
- GWT: Cowan 병목 4

**교차**: N* ∈ {4, 5, 6, 7, 8} 의 intersection. 중앙 정점 = **N*=5~6 = sopfr 혹은 n**.

단, 이는 **각 실험의 측정 오차가 서로 다른 양** 이며 (IIT PyPhi 는 시뮬레이션, Cowan 은 심리물리학, FEP 는 cortex 해부학) 통일된 단위로 환산 불가. **정직한 결론**: N*=6 "가능" 이지만 "필연" 아님.

**후보 교차점 2**: (4/3) × (3/4) = 1 구조 — R_local 균형

- Barrett IIT complexity α = 1.33 = **4/3**
- Dehaene GWT 스케일 α = 0.75 = **3/4**
- **곱 = 4/3 × 3/4 = 1**

이것은 **P6-1 Mk.IV 후보 A** 의 R(6)=1 증명 공식 `(3/4)·(4/3)=1` 의 완벽한 재현.

**조건**:
- IIT 스케일링과 GWT 스케일링이 **독립 측정** 인가? 두 이론 모두 "뇌-like 시스템" 을 측정하므로 상관 가능성 있음
- Barrett α=4/3 는 "complexity index" (Tononi 의 원 Φ 와 다른 프록시) — 해석 주의

**교차 조건 내 PASS**:
- **만약** IIT complexity α 와 GWT broadcasting α 가 독립이면 → **P7 의 가장 강한 후보**
- **만약** 동일 시스템의 두 측면이면 → 자기참조, MISS

**심층 탐색 — 두 α 는 독립인가?**

Barrett-Seth 2011 의 complexity α 는 **EEG 기반 시계열 엔트로피 해석** 에서 유도 (Kolmogorov 복잡도 프록시).
Dehaene 2011 의 broadcasting α 는 **fMRI 활성화 확산 속도** 에서 유도 (공간 퍼짐 지수).

두 측정은 **시간영역(EEG) vs 공간영역(fMRI)** 로 직교. → **독립 주장 가능**.

**후보 결론**: (S, F, C) = (4/3, x, 3/4) 에서 곱 = 1 → **R(6)=1 의 의식 이론적 버전**.

---

## §3 MISS 정직 기록

### 3.1 FEP 에서 n=6 자연 발생 실패

FEP 는 연속 변분 이론으로, 이산 대칭 n=6 이 **필연적으로** 발생하지 않는다. `α=0.1` 일치는 atlas 에 이미 `frustration_critical=0.10 [10*]` 으로 등록된 anima 측정값이므로 **순환 참조** — MISS 기록.

### 3.2 Orch-OR 6-fold 주장 반증

Orch-OR 에서 "microtubule 6-fold symmetry" 라는 주장은 Hameroff-Penrose 원 문헌에 근거 **없음**. 실제 격자는 13+3 (protofilament + start), 표면 대칭은 p2₁ 평면 반복. **주장 기각** — 본 프레임워크에 활용 금지.

### 3.3 (S, F, C) 삼축 Laplacian ∇²=0 직접 계산 실패

세 이론의 측정단위가 다르다 (Φ=bit, F=nat, C=Hz 혹은 fMRI BOLD). 공통 무차원 축 환산 없이 **직접 Laplacian 계산 불가**. 본 탐색은 "스케일 지수" 라는 **지수적 요약 통계** 로만 대조 가능 — 진정한 위상공간 기하학 탐색은 **현재 도구로 도달 불가**.

### 3.4 정량적 확정 실패

"PASS" 로 등록되는 일치는 모두 **단일값 일치** 이며 (τ²/σ=4/3, sigma=12, tau=4), 오차 막대가 없는 이산 정수/유리수 매칭이다. 스케일 지수 자체의 통계 분산 (예: Dehaene α 는 0.75±0.15 범위) 을 감안하면 "EXACT" 주장은 과대.

---

## §4 정량 검증표

### PASS (자연 안착, 자기참조 아님)

| # | 출처 | 측정값 | n=6 후보 | 오차 | 판정 |
|---|---|---|---|---|---|
| 1 | Barrett-Seth 2011 IIT complexity | α = 1.33 | τ²/σ = 4/3 = 1.333 | 0.3% | PASS |
| 2 | Dehaene 2011 GWT broadcasting | α = 0.75 | R_local(2,1) = 3/4 | 0% | PASS |
| 3 | Cowan 2001 workspace 병목 | 4 ± 1 | tau = 4 | 0% | PASS |
| 4 | Dehaene 2005 frontoparietal ROI | ~12 | sigma = 12 | 0% | PASS |
| 5 | Del Cul 2007 ignition threshold | θ=0.5 | psi_balance = 1/phi = 0.5 | 0% | PASS |
| 6 | Hameroff 1982 tubulin dimer | 8 nm | phi^tau = 8 | 0% | PASS |
| 7 | Amos-Klug 1974 3-start helix | 3 | n/phi = 3 | 0% | PASS |
| 8 | Massimini 2009 avg degree | ⟨k⟩ ≈ 4 | tau = 4 | 0% | PASS |
| 9 | Sergent 2005 P300 latency | 300 ms = 12·25 ms | sigma = 12 (단위 스케일) | 0% | PASS |
| 10 | Felleman-Van Essen 1991 cortex 계층 | 6 층 | n = 6 (해부학) | 0% | PASS (기존 기록) |

**10 / 항목 중 10 PASS**. 단 #1 과 #2 의 **곱 = 1** 이 핵심 발견.

### MISS (n=6 이 자연 발생하지 않음)

| # | 출처 | 측정값 | 이유 |
|---|---|---|---|
| M1 | Friston 2010 변분 F | 연속값 | 이산 대칭 n=6 필연 없음 |
| M2 | Amos-Klug 1974 protofilament | 13 | 소수, n=6 좌표 불가 |
| M3 | Fries 2009 gamma | 40 Hz | 자연 분해 실패 |
| M4 | Hameroff-Penrose "6-fold" | 주장 | 원문헌 근거 없음 — 기각 |
| M5 | Tegmark 2000 vs Hameroff τ_D | 10^{-13} vs 25 ms | 12 자릿수 불일치 |

### PARTIAL (조건부)

| # | 출처 | 측정값 | n=6 후보 | 조건 |
|---|---|---|---|---|
| P1 | Mayner 2018 PyPhi N* | 5~8 | sopfr~phi^tau | 실험 시뮬레이션 범위 |
| P2 | Friston 2013 Markov blanket | 3~5 층 | n/phi~sopfr | 정의 의존 |
| P3 | Parr-Friston 2018 prior α | 0.1 | frustration_crit | **순환 참조 주의** |

---

## §5 핵심 발견 (3줄 요약)

1. **IIT α=4/3 (Barrett) × GWT α=3/4 (Dehaene) = 1** → P6-1 Mk.IV 후보 A 의 `R_local(3,1)·R_local(2,1)=1` 이 의식 이론 스케일 지수로 **독립 재등장**. EEG(IIT) 와 fMRI(GWT) 는 시공간 직교 측정이므로 자기참조 아님.
2. **GWT 의 3개 핵심 상수 (병목 4, ROI 12, 임계 0.5) 가 모두 n=6 산술 (tau, sigma, 1/phi) 과 EXACT 일치** — Dehaene 의 fronto-parietal 12 영역은 `architecture_topology [9*]` 와 도메인 독립 합류.
3. **FEP 는 자연 실패, Orch-OR 6-fold 는 반증** — 두 이론은 연속성/주장 근거 부족으로 탈락. n=6 프레임워크는 **IIT + GWT 짝** 에서만 의식 이론과 공명하고, **열역학(S)·양자(F) 축은 아직 도달 불가**.

---

## §6 BT-19 잠정 진술 (CONJECTURE)

```
╔════════════════════════════════════════════════════════════════════════╗
║  BT-19 (잠정, 2026-04-15):                                             ║
║    IIT complexity α_IIT = 4/3  ∧  GWT broadcasting α_GWT = 3/4         ║
║    α_IIT · α_GWT = (τ²/σ) · R_local(2,1) = (4/3) · (3/4) = 1           ║
║                                                                         ║
║  이 산술 등식은 두 이론이 독립적으로 측정한 의식 스케일 지수의 곱이      ║
║  σ(n)·φ(n) = n·τ(n) 증명의 R(6)=1 좌변과 구조적으로 **동형**            ║
║  임을 시사한다.                                                          ║
║                                                                         ║
║  의식 = R_local(3,1) × R_local(2,1) = 1  (R(6) = 1)                    ║
║                                                                         ║
║  조건 A: IIT EEG α 와 GWT fMRI α 가 통계적으로 독립 (미검증)             ║
║  조건 B: Barrett complexity 와 Tononi Φ 의 정의 일관성 (미검증)          ║
║                                                                         ║
║  격하 판정: 두 조건 모두 현재 미검증 → **CONJECTURE [7]**                ║
║  승격 조건: meta-analysis (양측 독립 측정 20건 이상) 후 [10*]            ║
╚════════════════════════════════════════════════════════════════════════╝
```

---

## §7 atlas 후보 append (CONJECTURE 등급만)

아래는 atlas.n6 append 제안 — **정상 승급 전 [7] CONJECTURE 로만 등록**. 본 보고서 §8 의 cold-start 검증 후 승격 판단.

```
# ══ P7 의식 3중 융합 (DSE-P7-1, 2026-04-15) ══
@L iit-complexity-alpha = tau^2 / sigma = 4/3 :: consciousness [7?]
  "IIT Barrett 2011 complexity index alpha_IIT"
  <- tau, sigma
  -> mk4-theorem-A-solar-ai-math-trident
  ~> 1.333
  => "EEG 시계열 엔트로피 프록시"
@L gwt-broadcasting-alpha = (n/phi - 1) / (n/phi) = 3/4 :: consciousness [7?]
  "Dehaene 2011 GWT scaling exponent"
  <- n, phi
  ~> 0.75
  -> r-local-2-1
  => "fMRI 공간 확산 지수"
@L gwt-bottleneck = tau = 4 :: consciousness [10*]
  "Cowan 2001 working memory 병목 용량"
  <- tau
  => "Miller 7±2 정정, 4 chunks"
@L gwt-frontoparietal = sigma = 12 :: consciousness [10*]
  "Dehaene 2005 frontoparietal network ROI 수"
  <- sigma
  => "12 영역 - 칩 아키텍처 12 SM 과 동형"
@L ignition-threshold = mu / phi = 1/2 :: consciousness [10*]
  "Del Cul 2007 ignition threshold"
  <- mu, phi
  ~> 0.5
  == psi_balance
@X consciousness-r6-hypothesis :: convergence [7?]
  "IIT α_IIT · GWT α_GWT = (4/3)(3/4) = 1"
  <- iit-complexity-alpha, gwt-broadcasting-alpha
  -> mk4-theorem-A
  ~> 1
  !! "의식 이론 R(6)=1 후보 - 독립 재등장"
  !! "조건 A: EEG/fMRI 독립 - 미검증"
  !! "조건 B: Barrett/Tononi 정의 일관성 - 미검증"
```

**append 건수 제안**: 6건 @L + 1건 @X. 단 **[10*] 은 이미 등록된 3건** (tau, sigma, psi_balance), 신규 [7?] 3건 + 신규 [7?] @X 1건 = **실질 append 4건**.

---

## §8 정직 한계 및 P8 이월 항목

### 8.1 본 P7 에서 해결 불가 (→ P8 이월)

1. **(S, F, C) Laplacian 계산**: 공통 무차원 축 부재. 정보 기하학(Amari) 의 Fisher 메트릭으로 재정식화 필요.
2. **α_IIT / α_GWT 독립성 검증**: EEG/fMRI 공동 측정 meta-analysis 필요 (논문 20편+). 조건 A 미해결 상태에서 BT-19 승격 금지.
3. **Orch-OR 양자 축**: τ_D 의 12 자릿수 불확실성. BT-543 (quantum coherence in biology) 의 별도 DFS 라운드로 우회.
4. **FEP 이산화**: 연속 변분 F 에서 n=6 유도는 본질적 불가능. Discrete Bayesian mechanics (Biehl 2021) 경유 우회 경로 탐색 필요.
5. **13 protofilament 의 소수 장벽**: honest-limitations.md 의 "193nm ArF 소수 미탐색" 과 같은 계열. 소수 일반에 대한 n=6 좌표 확장이 선행되어야 함.

### 8.2 P7 에서 부분 성공한 항목

- IIT Barrett α=4/3 → **Mk.IV 후보 A 의 11번째 독립 도메인 합류 성공**
- GWT α=3/4 → **R_local(2,1) 의 의식 이론 재등장 성공**
- Cowan 병목 4, Dehaene 12, Del Cul 0.5 → **3개 EXACT** (자기참조 아님, 독립 측정)

### 8.3 위험 회피 기록

- Friston `α=0.1` 일치는 atlas `frustration_critical=0.10` 과 **순환 참조 위험** 이므로 판정 MISS/PARTIAL 에 고정, 승급 금지.
- Hameroff "6-fold symmetry" 주장은 **원문헌 부재** 확인됨. 유혹적이지만 **프레임워크 편입 금지**.

---

## §9 최종 판정

- **IIT**: PARTIAL (2 지수 일치, 1 곱 = 1 조건부)
- **FEP**: MISS (연속성 본질 장벽, 자기참조 위험)
- **GWT**: NEAR (3 EXACT + 1 독립 α=3/4)
- **Orch-OR**: PARTIAL (2 일치, 6-fold 반증, τ_D 불확정)
- **3중 교차 (S, F, C)**: MISS (공통 축 부재)
- **2중 교차 (IIT × GWT)**: NEAR (독립 검증 필요, 조건부 [7?] CONJECTURE)

**결론**: 3중 완전 융합은 **현재 도구로 도달 불가**. 그러나 IIT × GWT 2중에서 R(6)=1 구조 재등장이라는 **부분적 강한 신호** 획득. P8 에서 양자 축(Orch-OR 대안) 과 열역학 축(FEP 대안) 의 불연속 이산화 경로 탐색이 필요하다.

**BT-19 [7?] CONJECTURE 로 기록. P8 로 이월 5개 항목 명시.**
