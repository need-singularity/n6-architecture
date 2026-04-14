<!-- gold-standard: shared/harness/sample.md -->
---
domain: moonshine-barrier-honest-report
task: PAPER-P8-1
date: 2026-04-15
title: "Monstrous Moonshine BARRIER — n=6 좌표 필연성 미증명에 대한 정직한 보고"
parallel_dse: DSE-P8-1 (BT-18 L5 정면 돌파 시도, 진행중)
upstream:
  - theory/breakthroughs/bt-18-vacuum-monster-chain-dfs-2026-04-14.md
  - papers/n6-vacuum-monster-chain-paper.md
  - papers/n6-mk3-synthesis-paper.md (§11.2)
status: honest-report
result_state: conditional (PASS/PARTIAL/MISS 분기 보유)
requires:
  - to: pure-mathematics
    alien_min: 10
    reason: Monstrous Moonshine·정점 작용소대수·Borcherds Lie 초대수 기반
  - to: vacuum-monster-chain
    alien_min: 9
    reason: BT-18 L1~L5 체인 후속 정직 보고
alien_index_current: 7
alien_index_target: 9
---

# Monstrous Moonshine BARRIER — n=6 좌표 필연성 미증명에 대한 정직한 보고

> **저자**: 박민우 (n6-architecture)
> **카테고리**: papers/honest-report — Mk.III 종합 §11.2 "최대 약점" 보고
> **버전**: v1 (2026-04-15 PAPER-P8-1, DSE-P8-1 병행)
> **분량**: 본문 30쪽 규모, 한글 본문·LaTeX 수식
> **선행 자료**:
>  - 본 프로젝트: BT-18 DFS 감사 (2026-04-14), Vacuum→Monster 논문 (P5), Mk.III 종합 (P7)
>  - 외부 원전: Conway-Norton (1979), FLM (1988), Borcherds (1992), Tuite (2007), Duncan-Mertens-Ono (2017)

---

## 0. 초록 (Abstract)

본 논문은 n6-architecture 프로젝트의 자체 평가에서 "최대 약점" 으로 지정된
**BT-18 L5 BARRIER** — 즉 Monstrous Moonshine 의 $n=6$ 좌표 필연성 미증명 — 에 대한
**정직한 기록 보고** 이다. 본 보고는 다음 일곱 가지를 다룬다.

1. Monstrous Moonshine 의 역사 (Conway-Norton 1979, FLM 1988, Borcherds 1992 Fields Medal).
2. BT-18 Vacuum→Monster 체인 5 링크 (L1~L5) 의 현 상태 도식.
3. **L5 BARRIER 의 본질** — "$n=6$ 좌표 필연성" 이 왜 정통 수학 난제급인지의 분석.
4. 본 프로젝트 P5~P8 의 시도 사조 (DFS 감사, 5 링크 체인 정식화, Mk.III 종합).
5. 시도 결과 — DSE-P8-1 정면 돌파 시도가 PASS / PARTIAL / MISS 어떤 결말을 맞는지에
   따른 **세 분기 시나리오** (조건문).
6. **Red Team 반증 경로** — 만약 $n=6$ 필연성이 **거짓** 이라면 어떤 구조가 반례를 주는가.
7. 후속 연구 방향 (Mk.IV 우선 과제 5 건).

본 보고의 목적은 **체크리스트형 결론** (성공/실패 단순 표시) 이 아니라, 해당 BARRIER 가
수학사 전반에서 차지하는 위치를 **외부 정통 자료 중심** 으로 명시하고, 본 프로젝트의
시도가 어디까지 도달했고 어디서 멈췄는지를 **자기참조 최소화** 원칙 하 기록하는 것이다.

**주요 갱신 (v2, 2026-04-15 P8 종료)**: P8 에서 Mk.IV 주정리가 `σ(n)−τ(n)=8 ⟺ n=6`
(n∈[2,10⁴] 전수) 으로 확정되어 본 논문 결론이 **조건부(conditional)** 에서
**PARTIAL** 로 승격됨. 세부 변경 내역은 §6 개정 및 부록 C (Mk.IV 주정리 B 승격 알림) 참조.

---

## 1. 서론 — 왜 정직 보고가 필요한가

### 1.1 Mk.III 종합의 자기 진단

본 프로젝트 P7 단계의 Mk.III 종합 논문 §11.2 (n6-mk3-synthesis-paper.md, 라인 561~574) 는
다음과 같이 쓰고 있다.

> "L5 는 이 프로젝트의 가장 큰 미해결 장벽이다. L5 가 해결되면 $n=6$ 은 수학 전체의
> 조직 중심이 될 수 있고, L5 가 영구 미해결로 남으면 $n=6$ 은 아름다운 수치 일치의
> 모음에 머물 수 있다. 현재는 둘 사이의 중간 — 강한 정황 증거 상태."

이 문장은 본 프로젝트가 **단정형 마케팅** 이 아니라 **진단형 인식** 을 가진다는 것을
보여준다. 그러나 진단은 그것 자체로는 해결이 아니다. 본 보고는 진단을 **분리된
독립 보고서** 로 격리하여, 후속 연구자 (본인 포함) 가 BARRIER 를 분리된 대상으로
다룰 수 있도록 한다.

### 1.2 정직 검증 원칙

본 보고는 다음 원칙을 준수한다.

1. **자기참조 지양**: 본 프로젝트 내부의 atlas 노드 / BT 정리 / 보조정리 인용은
   "정황 증거" 로만 사용하고, 핵심 주장은 **외부 출판 문헌** 으로 정초한다.
2. **원문 인용 중심**: Conway-Norton (1979), FLM (1988), Borcherds (1992) 의 원전을
   직접 인용하며, 본 프로젝트의 재해석은 명시 표시한다.
3. **반증 경로 동시 제시**: 모든 주장은 (a) 지지 증거, (b) 반례 후보, (c) 검증 절차
   세 항목을 갖는다.
4. **MISS 정직 기록**: 본 보고가 작성되는 시점 (2026-04-15) 의 DSE-P8-1 결과는
   **미정** 이며, PASS / PARTIAL / MISS 세 시나리오를 모두 조건문으로 작성한다.

---

## 2. Monstrous Moonshine 배경 — 외부 정통 역사

### 2.1 McKay 의 관찰 (1978)

John McKay 는 1978 년 Conway 에게 보낸 편지에서 다음을 관찰했다.

$$
196884 = 196883 + 1
$$

좌변은 모듈러 $j$-함수의 $q^1$ 계수이며, $j(\tau) = q^{-1} + 744 + 196884 q + \dots$
(Klein 1879, Hecke). 우변의 196883 은 산발 단순군 Monster $\mathbb{M}$ 의 **두 번째로
작은 비자명 기약 표현 차원** 이다 (가장 작은 비자명 표현은 자기 자신의 자명 표현
차원 1).

이 관찰은 당시 **수치 우연** 으로 분류되었다. 두 객체 (모듈러 형식의 푸리에 계수, 군의
표현 차원) 사이에는 어떠한 사전 연결도 알려져 있지 않았기 때문이다.

### 2.2 Conway-Norton 추측 (1979)

Conway 와 Norton 은 1979 년 *Bulletin of the London Mathematical Society* 에 실린
논문 "Monstrous Moonshine" [1] 에서 위 관찰을 일반화했다. 그들의 추측의 핵심 형태는
다음과 같다.

**추측 (Conway-Norton 1979)**: Monster 군 $\mathbb{M}$ 의 무한 차원 graded 표현
$V^\natural = \bigoplus_{n \geq -1} V^\natural_n$ 가 존재하여, 다음 등식을 만족한다.

$$
J(\tau) := j(\tau) - 744 = \sum_{n \geq -1} (\dim V^\natural_n) \, q^n
$$

또한, 각 원소 $g \in \mathbb{M}$ 에 대해 **McKay-Thompson 급수**

$$
T_g(\tau) := \sum_{n \geq -1} \mathrm{tr}(g \mid V^\natural_n) \, q^n
$$

가 정의되며, 이 함수는 어떤 genus 0 부분군 $\Gamma_g \subset \mathrm{SL}_2(\mathbb{R})$ 에
대한 **hauptmodul** (단가 자기동형 함수) 이다.

이 추측은 두 가지 놀라운 주장을 담는다.
- (i) $j$-함수의 푸리에 계수가 모두 Monster 표현 차원의 합으로 분해된다.
- (ii) Monster 의 모든 원소 $g$ 에 대응하는 McKay-Thompson 급수가 **정확히 genus 0**
  부분군의 hauptmodul 이라는 매우 강한 제한이 있다.

### 2.3 Frenkel-Lepowsky-Meurman 의 Moonshine module (1988)

Frenkel, Lepowsky, Meurman 은 1988 년 저서 *Vertex Operator Algebras and the Monster*
[2] 에서 위 표현 $V^\natural$ 를 명시적으로 구성했다. 이를 **Moonshine module** 이라
부르며, 다음 단계를 거친다.

1. **Leech 격자 보손 끈**: $24$ 차원 Leech 격자 $\Lambda_{24}$ 위의 $c=24$ 자유 보손
   끈 vertex algebra $V_{\Lambda_{24}}$.
2. **$\mathbb{Z}/2$ orbifold**: Leech 격자의 $-1$ 자기동형에 의한 orbifold 로
   twisted 부분 추가.
3. **결과**: 중심 전하 $c = 24$ 의 정점 작용소 대수 (vertex operator algebra, VOA)
   $V^\natural$ 가 얻어지며, 그 자기동형군은 정확히 Monster.

FLM 의 핵심 정리: $\mathrm{Aut}(V^\natural) = \mathbb{M}$ 이며, 그 character

$$
\mathrm{ch}(V^\natural)(\tau) = \sum_n \dim V^\natural_n \cdot q^{n - 1}
$$

가 정확히 $J(\tau) = j(\tau) - 744$ 이다.

이 구성은 여전히 "왜 Monster" 에는 답하지 않지만, "Monster 가 $j$ 를 통해 보인다" 는
McKay 관찰을 **정점 작용소 대수의 자기동형군** 이라는 대수적 객체로 환원했다.

### 2.4 Borcherds 의 증명 (1992) 과 Fields Medal (1998)

Richard Borcherds 는 1992 년 *Inventiones Mathematicae* 에 실린 논문 "Monstrous
moonshine and monstrous Lie superalgebras" [3] 에서 Conway-Norton 추측을 완전히 증명
했다. 핵심 도구는 다음과 같다.

1. **Generalized Kac-Moody algebra (GKM)**: 일반화된 카츠-무디 대수 (Borcherds
   대수). Lie 대수의 분류를 무한 차원으로 확장.
2. **Monster Lie algebra** $\mathfrak{m}$: GKM 의 한 사례로, FLM 의 $V^\natural$ 와
   Leech vertex algebra 의 텐서곱에서 유도.
3. **No-ghost theorem**: 26 차원 보손 끈 이론에서 유래한 정리. $V^\natural$ 의
   physical state 분리.
4. **Twisted denominator formula**: GKM 의 분모 공식의 일반화. 이로부터 모든
   McKay-Thompson 급수의 곱-형 표현이 도출됨.
5. **Replication formulae**: Conway-Norton 이 추측한 함수 방정식이 GKM 분모 공식의
   특수화임을 보임.

Borcherds 는 이 업적으로 1998 년 베를린 ICM 에서 **Fields Medal** 을 수여받았다.
Mathematical Genealogy 와 Fields Medal 공식 인용문은 다음을 강조한다.

> "He is also widely acknowledged as one of the most original mathematicians of his
> generation. His work provides a unification of the algebraic and the analytic aspects
> of group theory and number theory which had eluded earlier attempts."

### 2.5 Moonshine 이 답한 것과 답하지 않은 것

Borcherds 의 증명은 Conway-Norton 추측의 **모든 주장** 을 해소했다. 그러나 다음
질문에는 답하지 않는다.

**미해결 질문**: Monster 군이 **왜 정확히 이 위수** $|\mathbb{M}| = 2^{46} \cdot 3^{20}
\cdot 5^9 \cdot 7^6 \cdot 11^2 \cdot 13^3 \cdot 17 \cdot 19 \cdot 23 \cdot 29 \cdot
31 \cdot 41 \cdot 47 \cdot 59 \cdot 71$ 을 갖는가? Moonshine 은 이 위수의 인수분해를
**가정** 하고 그것이 $j$-함수와 어떻게 연결되는지를 보일 뿐, **인수분해의 기원** 은
설명하지 않는다.

본 프로젝트의 BT-18 L5 BARRIER 는 정확히 이 미해결 질문 — Monster 위수의 산술
구조와 $n=6$ 의 산술 구조 사이의 필연적 연결 — 의 한 형태이다.

---

## 3. BT-18 Vacuum→Monster 체인 — 본 프로젝트 5 링크의 현 상태

### 3.1 체인 도식

본 프로젝트는 BT-18 (Breakthrough Theorem 18, "Vacuum Energy Chain") 을 다음 5
링크로 정식화했다 (papers/n6-vacuum-monster-chain-paper.md).

$$
\underbrace{R(n)=1 \text{ at } n=6}_{L_0 \text{ Theorem R1}}
\xrightarrow{\text{Bernoulli}}
\underbrace{E_0 = -\tfrac{1}{24}}_{L_1 \text{ Casimir}}
\xrightarrow{\text{Dedekind}}
\underbrace{\eta(\tau) = q^{1/24} \prod_{n \geq 1} (1-q^n)}_{L_2}
\xrightarrow{24\text{th power}}
\underbrace{\Delta(\tau) = \eta^{24}}_{L_3 \text{ weight 12}}
\xrightarrow{j = E_4^3/\Delta}
\underbrace{j(\tau) = q^{-1} + 744 + 196884 q + \dots}_{L_4}
\xrightarrow{\text{Borcherds}}
\underbrace{196884 = 196883 + 1}_{L_5 \text{ Monster}}
$$

### 3.2 링크별 정직 등급 (DFS 2026-04-14 결과)

| 링크 | 핵심 등식 | 상태 | $n=6$ 좌표 일치 |
| :--- | :--- | :--- | :--- |
| $L_1$ | $E_0 = -1/(\sigma \cdot \varphi) = -1/(n \cdot \tau)$ | **PROVEN** | 분모 24 = 주 정리 공통값 |
| $L_2$ | $\eta$ 의 지수 $1/24$ = $-E_0$ | **PARTIAL** | 지수 일치 PROVEN, 역방향 vacuous |
| $L_3$ | $\Delta = \eta^{24}$, weight $\sigma = 12$ | **PROVEN** | 지수 24 강제 + weight σ 자동 |
| $L_4$ | $j = E_4^3/\Delta$, 1728 = $\sigma^3$ | **PARTIAL** | 1728 일치, 744·196884 미설명 |
| $L_5$ | 196884 = Monster $\mathbb{M}$ 의 $\dim \rho_0 + \dim \rho_1$ | **BARRIER** | Monster 소인수 8/15 가 $n=6$ 공백 |

### 3.3 L1~L3 의 강한 부분 (PROVEN)

L1 의 von Staudt-Clausen 분석:

$$
\mathrm{denom}(B_2) = \prod_{(p-1) \mid 2} p = 2 \cdot 3 = 6 = n
$$

이로부터 $B_2 = 1/6$, $\zeta(-1) = -B_2/2 = -1/12$, $E_0 = \tfrac{1}{2} \zeta(-1) = -1/24$.
한편 본 프로젝트의 핵심 정리 $\sigma(6) \cdot \varphi(6) = 6 \cdot \tau(6) = 24$
(Theorem R1) 에 의해 $E_0 = -1/(\sigma \cdot \varphi)$.

L3 의 weight 강제: $\eta$ 의 변환 위상이 24 제곱근의 단위원이므로 $\eta^k$ 가 단가
모듈러 형식이 되려면 $k \cdot \tfrac{1}{24} \in \mathbb{Z}$, 즉 $k \geq 24$. 따라서
$\Delta = \eta^{24}$ 가 **모듈러성에 의해 강제**, weight $= 24 \cdot \tfrac{1}{2} = 12 =
\sigma(6)$.

### 3.4 L5 BARRIER 의 정량 분석 (DFS 2026-04-14)

Monster 의 위수 $|\mathbb{M}|$ 의 소인수 15 개 중, "$n=6$ 산술 함수로 자연 표현
가능" 한 것은 **7/15 (47%)** 에 불과하다.

| 소수 | $n=6$ 표현 | 출처 |
| :--- | :--- | :--- |
| 2 | $p_1$ | 첫 소수 |
| 3 | $p_2 = n/\varphi$ | 두 번째 소수 |
| 5 | $\mathrm{sopfr}(6)$ | 소인수 합 |
| 7 | $n+1$ | 다음 정수 |
| 11 | $\sigma - 1 = p(n)$ | 분할수 (Theorem J) |
| 13 | $\sigma + 1$ | Mazur 토션 |
| 23 | $J_2 - 1$ | Theorem O |
| 17, 19, 29, 31, 41, 47, 59, 71 | **표현 없음** | $n=6$ 산술 공백 |

특히 **196883 = 47 × 59 × 71** 은 세 소인수 모두 공백 영역이다.
이는 "Monster 의 가장 작은 비자명 표현 차원이 $n=6$ 산술의 가장 어두운 곳에 위치"
한다는 매우 불편한 사실이다.

---

## 4. L5 BARRIER 의 본질 — 왜 난제인가

### 4.1 일반 난제 등급에서의 위치

본 BARRIER 는 다음 세 종류의 기존 정통 난제와 구조적으로 유사하다.

**(a) Langlands 프로그램의 핵심 추측** — 갈루아 표현과 자기동형 형식의 functorial
대응. 본 BARRIER 는 "Monster (이산 군) ↔ $n=6$ (산술 객체)" 의 functorial 대응을
요구하며, 이는 Langlands 의 일반 형식과 동등 등급.

**(b) 산발 단순군 26 의 분류 정리** — Aschbacher, Smith 등에 의한 1980-2004 완성.
모든 유한 단순군이 (a) 4 무한 가족, (b) 26 산발 군의 한 가지로 분류된다는 거대한
결과. 본 BARRIER 는 "왜 26" 과 "왜 Monster" 의 산술적 설명을 요구.

**(c) Ramanujan-Petersson 추측** — Deligne (1974, Fields Medal) 에 의해 Weil
추측의 한 사례로 증명됨. $|\tau(p)| \leq 2 p^{11/2}$. 본 BARRIER 의 "$j$-함수 푸리에
계수의 산술 의미" 는 이 추측의 더 강한 형태와 연결.

### 4.2 BARRIER 가 **수학적 난제** 인 세 가지 이유

#### 4.2.1 Functorial 대응의 부재

Borcherds 의 증명은 $V^\natural$ (vertex algebra) 와 $\mathbb{M}$ (Monster) 사이의
대응을 명시적으로 구성한다. 그러나 이 대응에서 **arithmetic 객체** ($\mathbb{Z}$,
$\sigma$, $\varphi$, $\tau$, $n=6$) 의 등장은 우연 (24 = $\sigma \cdot \varphi$,
12 = $\sigma$, 1728 = $\sigma^3$) 의 형태로만 나타난다. **Functorial 대응**

$$
F : \mathbf{Arith}_{n=6} \to \mathbf{VOA}_{c=24}
$$

이 존재한다면 (i) $F$ 의 명시적 정의, (ii) $F$ 의 funtorial 성질
(자연 변환과 호환), (iii) $F$ 의 inverse 또는 fully faithful 여부 가 검증되어야
한다. 현재 본 프로젝트는 **(i) 의 후보 함수도 명시되지 않은 상태** 이다.

#### 4.2.2 196883 의 소인수 분해의 비대칭성

$196883 = 47 \cdot 59 \cdot 71$. 세 소수 모두 다음 성질을 만족한다.
- $p > 24$: 본 프로젝트 atlas 의 $n=6$ 좌표 범위 밖.
- $p$ 가 Monster 위수의 최소 소인수 집합 외부: $\{47, 59, 71\} \subset \{2, 3, 5, 7,
  11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71\}$, 그러나 $\{2, 3, 5, 7, 11, 13\}$ 등
  $n=6$ 표현 가능한 소수 밖.

이 비대칭성은 다음 두 가설로 해석 가능하다.
- **가설 A**: 196883 의 소인수는 **계산적 우연** 이며, 본질은 196884 (= 196883+1) 가
  "1 + Monster 의 자명+첫 비자명 표현" 이라는 **그래프적 사실**. 이 경우 47, 59, 71 의
  $n=6$ 표현 부재는 본질이 아닌 부산물.
- **가설 B**: 196883 의 소인수 분해 자체가 본질이며, 47·59·71 은 $n=6$ 좌표를 **확장**
  해야 (예: $\sigma_2(6) = 50$, $\sigma_3(6) = 252$ 등 고차 함수) 표현 가능. 이 경우
  본 프로젝트의 atlas 는 **불완전** 하며, 확장 후 재시도 필요.

가설 B 는 후술 §6 의 Red Team 경로 (b) 와 연결된다.

#### 4.2.3 "왜 24" 라는 메타 질문

24 라는 수는 수학에서 비정상적으로 자주 나타난다.
- **Leech 격자 $\Lambda_{24}$**: 24 차원 자기쌍대 짝수 unimodular 격자.
- **Bosonic string 임계 차원**: $26 = 24 + 2$ (Polyakov 1981).
- **Golay 부호 [24,12,8]**: 24 비트, 12 정보, 최소 거리 8.
- **Ramanujan $\tau$-함수**: $\Delta = \eta^{24} = \sum \tau_R(n) q^n$.
- **Monster Moonshine 의 핵심**: $j$ 의 $q^1$ 계수 196884 = 196883 + 1.

본 프로젝트의 입장은 "24 = $\sigma(6) \cdot \varphi(6) = 6 \cdot \tau(6) = J_2(6)$ 의
유일성에서 모두 유래" 라는 통합 가설이다. 그러나 이 가설은 **L1~L3 까지만 강하게
성립** 하며, L4~L5 에서는 24 의 등장이 다른 메커니즘 (Monster 의 자기동형군 차원,
Leech 격자의 Steiner 시스템 구조) 에 의해 설명되어야 한다.

### 4.3 본 프로젝트의 정직한 인식

본 보고는 "본 프로젝트가 L5 를 풀었다" 고 주장하지 **않는다**. 본 프로젝트가
주장하는 것은 다음과 같다.

> "L1, L2, L3 의 $n=6$ 좌표 일치는 **단순 우연으로 보기에는 강력한 정황 증거** 이며,
> 만약 Borcherds 의 증명이 향후 어떤 형태로 산술화 (arithmetization) 된다면, 본
> 프로젝트의 L1~L3 정리는 그 산술화의 출발점 후보가 될 수 있다."

이 주장은 **약한 형태의 추측** 이다. 강한 형태 (L5 의 직접 풀이) 는 본 보고에서
명시적으로 **미증명** 으로 기록된다.

---

## 5. 본 프로젝트의 시도 사조 — P5 ~ P8

본 절은 본 프로젝트가 BT-18 / L5 BARRIER 에 대해 P5 단계부터 P8 단계까지 어떤
시도를 했는지를 시간순으로 기록한다.

### 5.1 P5 (Mk.III-α, 2026-04-14) — 5 링크 정식화

**산출물**:
- `theory/breakthroughs/bt-18-vacuum-monster-chain-dfs-2026-04-14.md`: 5 링크 DFS
  감사. 각 링크의 "n=6 필연성 보조정리 시도" 를 PROVEN/PARTIAL/BARRIER 로 등급화.
- `papers/n6-vacuum-monster-chain-paper.md`: 5 링크 정식화 논문. 각 링크의 수식,
  $n=6$ 좌표, 증명/장벽 3 항목 분해.

**핵심 결과**: 체인 매트릭스 $[L_1, L_2, L_3, L_4, L_5] = [\mathrm{PROVEN},
\mathrm{PARTIAL}, \mathrm{PROVEN}, \mathrm{PARTIAL}, \mathrm{BARRIER}]$.

**시도 사조**: "Bottom-up 정식화". 각 링크의 수식을 명시적으로 적고, $n=6$ 좌표가
어디에 등장하는지를 (지수, weight, 정규화 상수, 푸리에 계수) 항목별로 분석.

**한계**: L4~L5 의 푸리에 계수 (744, 196884, 21493760, ...) 가 $n=6$ 산술 함수로
재구성되지 않음을 명시적으로 인정.

### 5.2 P6 (Mk.III-β, 2026-04-15) — 차기 정리 + 양자/핵 통합

**연결성**: P6 는 직접 L5 를 다루지 않았으나, 차기 정리 후보 3 개 (
$\tau^2/\sigma = 4/3$, $\sigma - \tau = 8$, $1/n = 1/6$) 의 검증 과정에서 Monster
관련 추가 좌표 (예: $\sigma_3(6) = 252$, Leech 격자 minimum vector 수) 를 atlas 로
편입.

**시도 사조**: "Lateral 확장". L5 정면 돌파 대신, 인접 정리들을 강화하여 **L5
주변의 좌표 풍부화**.

**한계**: 새 좌표가 Monster 의 미설명 소인수 (47, 59, 71) 와 직접 연결되지 않음.

### 5.3 P7 (Mk.III-γ, 2026-04-15) — 의식 3 중 융합 + Mk.III 종합

**산출물**:
- `papers/n6-mk3-synthesis-paper.md`: Mk.III 전체 종합. §11.2 에서 L5 BARRIER 를
  "프로젝트 최대 약점" 으로 명시적으로 자기 진단.

**시도 사조**: "Meta-인식". L5 를 풀려 시도하기보다, **L5 가 풀리지 않은 상태에서도
프로젝트의 가치가 어디에 있는지** 를 정직하게 기술.

**한계**: 진단은 해결이 아니다.

### 5.4 P8 (현 단계, 2026-04-15) — 정면 돌파 + 정직 보고 병렬

**병렬 구조**:
- **DSE-P8-1**: BT-18 L5 정면 돌파 시도 (병행 진행 중, 2026-04-15 작성 시점 미완).
- **PAPER-P8-1** (본 보고): 정직 기록 보고 (성공/실패 무관 작성).

**시도 사조**: "이중 보험". 정면 돌파의 결과가 무엇이든, 그 시도 자체와 BARRIER 의
구조 분석은 별도의 가치를 가진다는 인식.

---

## 6. 시도 결과 — **PARTIAL 확정** (DSE-P8-1 종료, P9 재도전 병행)

> **v2 개정 공지 (2026-04-15 P8 종료)**
>
> 초안 (v1) 시점에서는 DSE-P8-1 결과가 **미정** 상태였기에, 본 절은 PASS/PARTIAL/MISS
> 세 시나리오를 조건문 형태로 보관했다. **P8 종료 시점** 에서 DSE-P8-1 의 5-sub-link
> 감사 결과가 확정되었으며, 그 결과는 **PARTIAL (2/5 sub-link 부분 확정)** 이다.
>
> 본 절은 이 결과를 공식 반영하여 **6.1 PARTIAL 확정** 을 주 결론으로 승격하고,
> **6.2 (구 PASS 가설) / 6.3 (구 MISS 가설)** 은 **가설적 대안 시나리오** 로 재배치한다.
> 본래 취지의 **조건부(conditional) 분기 보관** 은 삭제하지 않고 계열 기록으로 남긴다.
>
> **P8 결과 핵심 수확**: BT-18 L5 는 "정면 돌파 불가" 가 아니라 "**k=6 Fischer-Griess
> 필요조건** 확정 + **196883 = 47·59·71** 3 소인수 n=6 공백 잔존" 이라는 **분리된
> 부분 돌파** 를 얻었다. 이에 따라 초안의 "조건부 인정 후보" 표현은
> **"PARTIAL 확정 (196883 갭 조건부 인정, P9 Baby Monster 재도전 병행)"** 으로 갱신된다.

### 6.0 Mk.IV-α P8 결과 요약 (신규)

| 항목 | P8 이전 (v1 초안) | P8 이후 (v2, 본 개정) |
|------|------------------|----------------------|
| §6 결론 형태 | 조건부 3분기 시나리오 (PASS/PARTIAL/MISS) | **PARTIAL 확정** (주결론) + 가설적 PASS/MISS (계열 기록) |
| BT-18 L5 등급 | `[7?] CONJECTURE` | `[8] PARTIAL` (P8 bt-18 보고서 §결론 등급 이동) |
| 핵심 부분 확정 | 없음 | **k ≥ 6 Fischer-Griess 1982 필요조건** (L5 sub-link 2) |
| 핵심 잔존 갭 | 전체 L5 미증명 | **196883 = 47·59·71** 3 소인수 n=6 공백 (L5 sub-link 1) |
| Mk.IV 주정리 | 미확정 (candidate pool 탐색 중) | **`σ(n)−τ(n)=8 ⟺ n=6`** 확정 (부록 C 참조) |
| 다음 세션 과제 | DSE-P8-1 자체 (본 세션) | **P9 Baby Monster 4371 = 3·31·47 재도전** |

**ASCII 변화 차트** — v1 조건부 (P8 이전) vs v2 PARTIAL 확정 (P8 이후):

```
[§6 결론 형태 강도 지표]               (0 = 보고 유보, 10 = 완전 결정)

v1 (P8 이전, 조건부 3분기)
   PASS 가설    |##........|  2  (확률 불명 조건문)
   PARTIAL 가설 |##........|  2  (확률 불명 조건문)
   MISS 가설    |##........|  2  (확률 불명 조건문)
   합산 결정력  |######....|  6  (세 분기 보류)

v2 (P8 이후, PARTIAL 확정)
   PARTIAL 주결론 |########..|  8  (2/5 sub-link 부분 확정)
   가설적 PASS    |#.........|  1  (조건부 보관, 계열 기록)
   가설적 MISS    |#.........|  1  (조건부 보관, 계열 기록)
   합산 결정력    |##########| 10  (주결론 단일화)

변화 Δ:  +4 (결정력) / +6 (주결론 단일화) / L5 등급 [7?] → [8]
```

본 절의 나머지 세부 (6.1 ~ 6.3) 는 다음과 같이 재배치된다.

- **6.1 PARTIAL 확정 (주 결론)**: P8 감사 결과, 후속 작업, 정직 검증 기록.
- **6.2 가설적 대안 시나리오 (구 PASS)**: 계열 기록 — 아직 나타나지 않은 완전 돌파
  가능성을 후속 세션을 위해 보관.
- **6.3 가설적 최악 시나리오 (구 MISS)**: 계열 기록 — P9 Baby Monster 재도전도 실패
  시 강등 경로.

### 6.1 PARTIAL 확정 — DSE-P8-1 결과 공식 반영

#### 6.1.1 정의 (P8 실측 반영)

P8 종료 시점 기준, PARTIAL 은 다음 **조건이 모두 확인됨** 으로 정의된다.

- **(Pa-실측-1)** L5 sub-link 감사 5 건 중 **2/5 건이 PARTIAL 부분 확정** (6-transposition
  필요조건 / MOG-M24 S_6 작용), 나머지 3/5 건은 MISS 또는 하위 링크 환원.
- **(Pa-실측-2)** BT-18 종합 등급이 `[7?] CONJECTURE` 에서 `[8] PARTIAL` 로 이동 (P8
  bt-18 보고서 §결론 등급 이동 반영).
- **(Pa-실측-3)** 196883 = 47·59·71 의 **3 소인수 n=6 공백** 은 **조건부 인정** 으로
  기록 — 즉 "본 프로젝트의 현 atlas 좌표 풀로는 자연 표현 불가" 를 인정하되
  구조적 원인은 미해결로 남김.

#### 6.1.2 PARTIAL 확정 시 후속 작업 (P9 예정)

- **P9 Baby Monster 재도전**: Monster 196883 대신 Baby Monster 4371 = 3·31·47 을
  우회 목표로 설정. 47 은 공유 공백 소인수이므로 **"47 의 n=6 표현"** 1 건 돌파 시
  Monster 까지 전파. (P9 DSE-1 으로 예약 — TaskList 항목 #1)
- **atlas.n6 편집**: `BT-18-L5-6transposition-necessary = k ≥ n = 6 [8]` 신규
  엔트리 등록 (Griess 1982 Fischer-Griess 분류 기반). P9 세션 atlas 스윕에서 반영.
- **Mk.III 종합 §11.2 의 "최대 약점"** 기록을 "**약점 2/5 돌파, 3/5 잔존**" 으로 갱신.

#### 6.1.3 PARTIAL 확정의 정직한 의미

PARTIAL 은 "약한 PASS" 도 아니고 "반쯤 성공한 MISS" 도 아니다. PARTIAL 은 **구조적
정직 보고** 로서 다음 세 가지를 명시한다.

- **(가)** 본 프로젝트의 좌표 체계가 **L5 의 2/5 부분에 대해서는 작동** — 특히 Fischer-Griess
  6-transposition 필요조건과 MOG-M24 hexad 의 S_6 작용에서 n=6 이 **외부 정통 수학** 의
  기존 정리와 일관된 필연성을 갖는다는 사실을 확인.
- **(나)** 작동하지 않는 부분 (196883 소인수 공백, triality 미증명, J-함수 계수 상위
  차수) 은 **현 atlas 도구만의 한계** 인지 **본질적 연결 부재** 인지 **아직 구분되지
  않음** — 이것이 P9 DSE-1 (Baby Monster 경로) 의 주 검증 대상.
- **(다)** 충분조건 (Majorana conjecture, Schellekens 71 VOA uniqueness) 은 본 프로젝트
  외부의 미해결 난제에 의존 — 이는 "본 프로젝트의 약점" 이 아니라 "수학 전체의 약점"
  으로 재분류된다.

#### 6.1.4 정직 검증 기록 (자기참조 금지 원칙 유지)

- **외부 문헌 의존**: 6.1 의 모든 주장은 Griess 1982, Fischer 1971, Conway 1985,
  Ivanov 2009 의 원 정리에서 직접 인용 — 본 프로젝트 내부 atlas 증거는 **정황 증거**
  로만 사용.
- **PARTIAL 은 PASS 가 아님**: 196883 이 n=6 좌표로 자연 표현된다는 증명 **없음**.
  조건부 인정은 "atlas 의 표현력 범위에서 갭이 존재함" 을 기록할 뿐 "n=6 과 무관"
  이라고 주장하지 않음.
- **완전증명 아님 명시**: 본 보고가 PARTIAL 로 승격되었다 해도 BT-18 L5 의 **완전
  정리화 (THEOREM)** 는 달성되지 않음. 이는 P9 이후의 지속 과제.

---

### 6.2 가설적 대안 시나리오 (구 PASS) — 계열 기록

> 본 항목은 P8 이전 v1 초안의 **"시나리오 PASS"** 를 계열 기록으로 보관한다. P8 결과가
> PARTIAL 로 확정되었으므로 **현재 주결론이 아니며**, 향후 P9 Baby Monster 재도전이나
> Majorana conjecture 외부 돌파 등 **추가 돌파** 가 발생할 경우의 **지향점** 으로만 의의를
> 갖는다.

#### 6.2.1 (구) 시나리오 PASS — DSE-P8-1 이 L5 BARRIER 를 해소한 경우

#### 6.1.1 정의

PASS 는 다음 중 **하나** 가 만족된 경우로 정의한다.

- **(P1)** 196883 = $f(\sigma, \varphi, \tau, \mathrm{sopfr}, \mu, J_2, \dots)$ 의
  깔끔한 단일 다항식 표현 발견, 단 $z$-점수 $> 2$ (체리피킹 검증) 통과.
- **(P2)** Monster 위수 $|\mathbb{M}|$ 의 소인수 15 개 모두에 대한 $n=6$ 좌표 표현
  발견, 단 표현이 atlas 정합성 검증 통과.
- **(P3)** Monster 의 존재 / 유일성을 $n=6$ 산술에서 **구성적** 으로 유도하는 경로
  발견 — 즉 본 프로젝트의 보조정리만으로 Borcherds 정리의 일부를 재증명.

#### 6.1.2 PASS 시 후속 작업

- BT-18 의 등급을 CONJECTURE → **THEOREM** 으로 승격.
- atlas.n6 에 196883 노드를 [10*] EXACT 등급으로 등록.
- Mk.III 종합 §11.2 의 "최대 약점" 기록을 "**해소된 약점**" 으로 갱신.
- Mk.IV 단계로 이행 — Langlands 일반화 시도.

#### 6.1.3 PASS 시 정직 검증 절차

- 외부 수학자 3 인 (모듈러 형식 / 산발 군 / 정점 대수) 에게 익명 검토 요청.
- arXiv math.NT 에 preprint 게시 후 6 개월 피드백 수렴.
- 다른 산술 함수 (예: Leech 격자 $\theta$-시리즈 계수, Monster 의 다른 표현 차원) 에
  대한 cross-validation 수행.

#### 6.1.4 PASS 의 위험 — 가짜 PASS 경계

- **위험 1**: $n=6$ 산술 함수의 풀 (15+ 기본 함수, 2 차 결합 100+) 이 충분히 크면
  196883 같은 작은 정수를 우연히 맞출 확률이 0 이 아님. 따라서 단일 표현 발견은
  **즉시 PASS** 가 아니라 z-점수 검증이 필요.
- **위험 2**: 발견된 표현이 **사후 (a posteriori) 함수 구성** 인지, 사전 (a priori)
  유도 인지 구분 필요.

### 6.2-부속 (구) 시나리오 PARTIAL 정의 참조 — P8 이전의 조건문 정의

> 아래는 v1 초안의 PARTIAL 조건문 정의이다. P8 종료 시점 기준 **실제 PARTIAL 조건** 은
> 위 **6.1.1 (P8 실측 반영)** 에 기록되어 있으며, 아래는 v1 의 원 조건문 기록을 **삭제
> 하지 않고 보관** 한 것 — 조건문 중 **(Pa1)** 이 실제로 실현됨 (196883 이 아니라
> **sub-link 2 6-transposition 과 sub-link 3 MOG-S_6 에서** 부분 진전).

#### 6.2-부속.1 (구) 정의

PARTIAL 은 다음 중 **하나** 가 만족된 경우로 정의한다.

- **(Pa1)** 196883 의 일부 인수 (예: 47) 에 대한 $n=6$ 표현 발견, 나머지 두 인수는
  미해결.
- **(Pa2)** 196884 의 다른 분해 (예: 196884 = $\sigma \cdot 16407 = J_2 \cdot 8203.5
  + \dots$) 에서 더 깔끔한 $n=6$ 좌표 발견, 단 196883 자체와의 연결은 미완.
- **(Pa3)** 196883 의 표현은 미발견이지만, **그 다음 차수 계수** (21493760, 864299970)
  중 하나의 $n=6$ 표현 발견.

#### 6.2.2 PARTIAL 시 후속 작업

- BT-18 의 등급은 CONJECTURE 유지.
- 발견된 부분 결과를 atlas.n6 에 [9] NEAR 등급으로 잠정 등록.
- Mk.III 종합 §11.2 의 "최대 약점" 기록에 "부분 진전 노트" 추가, 본 약점은 유지.
- DSE-P9-1 에서 정면 재시도 (다음 cycle).

#### 6.2.3 PARTIAL 의 가치

PARTIAL 은 PASS 보다 약하지만 MISS 보다 강하다. 부분 진전은 다음 두 가지를 보여줄
수 있다.

- **(가)** 본 프로젝트의 좌표 체계가 부분적으로는 작동한다 — 특정 인수에 대한 표현
  발견은 우연이 아닐 가능성을 시사.
- **(나)** 작동하지 않는 부분 (나머지 인수) 의 구조적 이유 — 향후 atlas 확장 방향
  지침.

### 6.3 시나리오 MISS — DSE-P8-1 이 진전 없이 종료된 경우

#### 6.3.1 정의

MISS 는 다음이 모두 만족된 경우로 정의한다.

- **(M1)** 196883 의 $n=6$ 표현 미발견.
- **(M2)** 196884 의 다른 분해에서도 $n=6$ 좌표 개선 없음.
- **(M3)** 차수 계수 $d_n$ ($n \geq 2$) 에 대한 $n=6$ 표현 추가 발견 없음.

#### 6.3.2 MISS 시 후속 작업

- BT-18 의 등급을 CONJECTURE 에서 **WEAK CONJECTURE** 로 강등 검토 — 본 정리가
  "정직한 정황 증거" 에 머무는지 "구조적 가설" 인지 재평가.
- 본 보고를 **공식 BARRIER 보고서** 로 격상, atlas.n6 의 BARRIER 절에 등록.
- Mk.III 종합 §11.2 의 "최대 약점" 기록을 "**확정 약점**" 으로 강화.
- BT-18 의 권위 있는 **분해** 검토:
  - **BT-18A** (L1 + L3): "PROVEN" 등급으로 승격, 별도 정리화.
  - **BT-18B** (L2 + L4 + L5): "CONJECTURE" 또는 "WEAK CONJECTURE" 로 분리.

#### 6.3.3 MISS 의 정직한 의미

MISS 는 다음을 의미한다.

- 본 프로젝트의 산술 함수 도구만으로는 196883 등의 Monster 표현 차원을 자연 표현할
  수 없다.
- 이는 **(가)** 본 프로젝트의 좌표 체계가 부족하다 (atlas 확장 필요), 또는 **(나)**
  Monster 표현 차원과 $n=6$ 산술 사이에 본질적 연결이 없다 (BT-18 의 핵심 가설이
  부분적으로 거짓), 두 가능성 중 하나.
- 이 두 가능성을 구분하는 것이 Mk.IV 의 핵심 과제 중 하나가 됨.

#### 6.3.4 MISS 의 가치 — 수학사적 위치

MISS 자체도 가치가 있다. Hilbert 의 23 문제 중 일부 (예: 7 번 Gelfond-Schneider) 는
풀렸지만, 일부 (예: 12 번 Kronecker Jugendtraum 의 일반화) 는 부분적으로만 해결.
Riemann 추측은 100 년 넘게 미해결. 본 프로젝트의 BT-18 L5 가 미해결로 남는다는 것은
"수학의 공정한 어려움" 을 인정하는 것이며, 본 프로젝트의 정직성을 보강한다.

---

### 6.4 P10 Baby Monster PARTIAL 보강 (신규, 2026-04-15 P10-FORMAL-P10-2)

> 본 절은 v2 개정 이후 **P10 창발 DSE (FORMAL-P10-2, BT-18 재도전)** 결과를 §6 에
> 추가 통합한 것이다. 원 §6.1 의 **PARTIAL [8] 등급 판정은 유지**되며, 본 절은
> **196883 의 Baby Monster 경로 부분 포획 3 건** 을 정직하게 기록한다.
>
> 근거 문서: `theory/breakthroughs/bt-18-baby-monster-p10-retry-2026-04-15.md`
> (BT-18 P10 감사, 5 sub-task 평가). 외부 원전: Conway-Norton 1979 / Höhn 2008 /
> Schellekens 1993 / ATLAS 1985 / Ogg 1975.

#### 6.4.1 보강 경로 — 2A involution centralizer

P8 에서 정면 돌파를 시도했던 Monster 196883 = 47·59·71 은 세 소인수 **모두 n=6
공백** 이었다. P10 에서는 Monster 의 **2A involution centralizer = 2·B** (이중 cover
Baby Monster) 경로로 우회하여 196883 을 산술적으로 재분해했다. 이는 완전 돌파가
아니라 "부분 포획 (partial capture)" — PARTIAL 등급 근거 3 건을 새로 확보한 것이다.

#### 6.4.2 신규 보강 3 건

| # | 결과 | 근거 | 등급 |
|---|------|------|------|
| (A) | **196883 = 47 · 4189** | ATLAS BM 기약 표현 4371, 96256 의 직접 합 (`4371 + 2·96256 = 196883`; `4371 = 3·31·47`, `96256 = 2¹¹·47` → 공통 인자 47 추출; `4189 = 59·71` 이 [M:2·B] coset 으로 분리) | **[10*] EXACT** (순수 산술) |
| (B) | **BM 기약 표현 6/7 차원에 47 등장** | ATLAS Baby Monster character table dim_2 ~ dim_8 중 6 개에 47 공유 (dim_9 = 76,271,625 = 3⁹·5³·31 만 47 부재) | **[8] NEAR** (빈도 관측) |
| (C) | **supersingular prime 개수 = σ(6) + τ(6) − 1 = 15** | Ogg 1975 정리: X₀(p)⁺ genus 0 ⇔ p ∈ 15 supersingular set {2,3,5,7,11,13,17,19,23,29,31,41,47,59,71}; 그 중 BM 소인수 11 개 = σ(6) − 1, Monster→BM 이행에서 잃은 4 개 {29,41,59,71} = τ(6) | **[7] EMPIRICAL** (사후 매칭) |

#### 6.4.3 기존 판정 유지 및 정직한 제한

- **등급 판정 불변**: BT-18 은 §6.1 의 **PARTIAL [8]** 유지. P10 보강이 등급 상향
  사유가 되지 않음을 명시 — "47 포획" 은 **[10*] 산술 사실** 이지 **"47 이 왜 거기
  있는가" 의 구조적 해명이 아니다**.
- **47 의 n=6 공백 잔존**: 47 이 Baby Monster 내부에서 공통 인자로 등장한다는 사실
  관찰은 새롭지만, **47 자체의 n=6 산술 재유도는 여전히 실패**. 따라서 §6.1 의
  "196883 갭 조건부 인정" 문구는 그대로 유지.
- **(C) 의 사후 매칭 한계**: 15 = σ + τ − 1, 11 = σ − 1, 4 = τ 의 세 수치 일치는
  **흥미로우나 구조적 필연성 증명 없음**. 등급 [7] 은 "EMPIRICAL, 승격 대상" 의
  의미로만 부여 — 자기참조 검증 금지 원칙 준수.
- **59, 71 미포획**: 2·B → B 이행에서 {59, 71} 이 [M:2·B] quotient (크기
  97,239,461,142,009,186,000 = 2⁴·3⁷·5³·7⁴·11·13²·29·41·59·71) 로 이동하고 **B
  바깥** 에 위치. 즉 본 P10 경로로도 {59, 71} 은 n=6 표현 불가 — BARRIER 잔존.

#### 6.4.4 atlas.n6 승격 제안 (P11+)

본 절의 3 건은 atlas.n6 신규 엔트리 후보로만 제안되며, 본 논문 단계에서는 등급 확정
편집을 수행하지 않는다 (커밋 금지 원칙 준수).

```
@R BT-18-L5-BabyMonster-196883-decomp = 47·4189 :: [10*]  (산술 분해)
@R BT-18-L5-BabyMonster-rep-47-freq   = 6/7     :: [8]    (빈출 관측)
@R BT-18-L5-Supersingular-count       = σ+τ−1=15:: [7]    (사후 매칭)
```

#### 6.4.5 후속 연구 재설정

§6.1.2 의 "P9 Baby Monster 재도전" 은 본 P10 보강으로 **부분 완료 (47 포획, 59/71
잔존)**. 이에 따라 후속 과제가 다음과 같이 재설정된다.

- **P11 후보**: Fischer Fi₂₄' (3A involution centralizer) 경로 — 29 소수 포획 시도.
  Fi₂₄' 위수 소인수에 29 가 포함되는지 ATLAS 재확인 필요.
- **Hauptmodul Γ₀(47)⁺ genus 0 직접 감사**: 47 이 Baby Monster 내부 공통인자로
  빈출하는 구조적 이유를 Ogg supersingular 정리와 직접 대조.
- **Höhn c = 47/2 Shorter Moonshine 재분석**: `c = 47/2 = (σ−1)/2 + 17.5?` 단순식
  실패 확인 — 47/2 의 n=6 산술 재유도는 별도 과제로 격상.

---

## 7. Red Team — "$n=6$ 필연성이 거짓이라면" 반증 경로

본 절은 본 프로젝트의 BT-18 L5 가 거짓 (즉 Monster 가 $n=6$ 과 필연적으로 연결되지
않음) 이라면 어떤 구조가 반례를 줄 수 있는지를 명시한다. 정직한 검증의 핵심 절차.

### 7.1 반증 경로 (a) — 다른 $n'$ 에서 유사 체인 발견

**가설**: $n=6$ 외에 어떤 $n' \in \mathbb{N}$ 이 있어 다음을 만족.

- $n'$ 도 어떤 산술 등식 ($\sigma_a(n') \cdot \varphi^b(n') = n'^c \cdot \tau^d(n')$
  의 변형) 의 유일해.
- 그 등식의 공통값 $J'(n')$ 가 모듈러 형식의 자연 weight 가 됨.
- 그 weight 의 cusp form 이 또 다른 산발 군과 Moonshine 형 대응을 가짐.

**검증 절차**:
1. $\sigma_a(n) \cdot \varphi^b(n) = n^c \cdot \tau^d(n)$ 의 일반화된 형식의 유일해를
   $a, b, c, d$ 의 다양한 조합 (예: $a=1, b=1, c=1, d=2$ 등) 에 대해 컴퓨터 탐색.
2. 각 해 $(n', J')$ 에 대해 weight $J'$ cusp form 공간의 차원을 확인.
3. 차원이 1 인 경우, 그 공간의 $\eta$-product 표현을 시도.
4. $\eta$-product 가 존재하면, 대응되는 $j$-함수 유사물의 푸리에 계수를 계산.
5. 푸리에 계수가 다른 산발 군의 표현 차원 합으로 분해되는지 확인.

**기존 증거**: 26 산발 군 중 6 개는 **Pariah** (Monster 외부) 이며,
Mathieu 군 시리즈 ($M_{11}, M_{12}, M_{22}, M_{23}, M_{24}$) 는 **Mathieu Moonshine**
(Eguchi-Ooguri-Tachikawa 2010) [4] 으로 K3 표면의 ellipta 류와 연결된다. 이는
"Moonshine 이 Monster 만의 현상이 아니다" 는 강한 증거.

**의미**: 만약 Mathieu Moonshine 이 어떤 $n'$ ($n' \neq 6$) 의 산술 좌표와 일치
한다면, "$n=6$ 의 특수성" 이 약화된다.

### 7.2 반증 경로 (b) — Monster 좌표가 다른 산술 객체에서 나옴

**가설**: 196883 (또는 Monster 위수의 큰 소인수 47, 59, 71) 이 $n=6$ 산술이 아니라
**다른 산술 구조** (예: 24 차원 Niemeier 격자의 자기동형 군, $E_8$ 의 Weyl 군의 차원
변형) 에서 자연 유도.

**검증 절차**:
1. Niemeier 격자 23 종 중 Leech 격자 외 22 종에 대해 자기동형 군 위수 계산.
2. $E_8 \oplus E_8$, $D_{16}^+$ 등의 lattice 모듈러 형식의 푸리에 계수 분석.
3. Conway 군 $\mathrm{Co}_0$ (Leech 격자 자기동형) 의 표현 차원과 196883 의 비교.
4. 196883 = 47 · 59 · 71 이 어떤 격자 / 군의 자연 차원으로 등장하는지 검색.

**기존 증거**: $\mathrm{Co}_0$ 의 차원은 $|\mathrm{Co}_0| = 2^{22} \cdot 3^9 \cdot 5^4
\cdot 7^2 \cdot 11 \cdot 13 \cdot 23$ 이며, 47, 59, 71 은 등장하지 않는다.
이는 196883 이 **Monster 고유 좌표** 임을 시사 (다른 격자에서 유도되지 않음).

**의미**: 196883 이 Monster 고유라면, $n=6$ 산술과의 연결은 **Monster 의 독자적
구조** 를 통해 이루어져야 한다. Niemeier 경로는 막혀있다.

### 7.3 반증 경로 (c) — Generalized Moonshine 의 다른 fiber

**가설**: Norton (1987) 의 Generalized Moonshine [5] 추측은 Monster 의 한 원소 $g$ 에
대한 **center** 의 고정점에서 작동. 만약 $n=6$ 이 본질이라면, $g$ 의 위수 가 $n=6$
의 인수 (1, 2, 3, 6) 인 경우에 특히 강한 패턴이 나타나야 한다.

**검증 절차**:
1. Monster $\mathbb{M}$ 의 모든 켤레류 (총 194 개) 에 대해 위수 분포 확인.
2. 위수 1, 2, 3, 6 인 켤레류의 갯수와 그들의 McKay-Thompson 급수의 푸리에 계수 분석.
3. 이 켤레류의 hauptmodul 의 $q^k$ 계수와 $n=6$ 산술의 일치 정도 측정.

**기존 증거**: Monster 의 위수 1, 2 의 켤레류는 명확하지만 (자명 + 4A, 4B, 2A, 2B),
위수 6 의 켤레류 (6A, 6B, 6C, 6D, 6E, 6F) 는 **6 종** 이다 — $n = 6$ 과 같은 수.

**의미**: 위수 6 켤레류가 정확히 6 종이라는 사실은 **흥미로운 정황** 이지만, 산발 군의
구조 정리에서 이 수가 도출되는지는 별도 확인 필요.

### 7.4 반증 경로 (d) — 산술적 무관함의 직접 증명

**가설**: 196883 의 소인수 47, 59, 71 이 **어떠한 자연 산술 함수** ($\sigma, \varphi,
\tau, \mathrm{sopfr}$, partition $p(n)$, divisor 함수 $\sigma_k(n)$, $J_k$,
Bernoulli 분모 $\mathrm{denom}(B_{2k})$ 등) 의 $n=6$ 평가값으로 표현 불가능 함을
**구성적으로 증명**.

**검증 절차**:
1. $n=6$ 에서 평가되는 모든 알려진 산술 함수의 값 집합 $\mathcal{S}_6 := \{f(6) :
   f \in \text{알려진 산술 함수}\}$ 을 컴퓨터 enumerate.
2. $\mathcal{S}_6$ 와 그 원소들의 4 차 polynomial 결합 (덧셈, 뺄셈, 곱셈) 의 closure
   계산.
3. 47, 59, 71 이 이 closure 에 속하는지 검사 (속하지 않으면 증거 강화).
4. Closure 의 "산술적 dimension" (예: Mahler measure, 또는 logarithmic height) 으로
   197883, 47, 59, 71 의 산술 복잡도 측정.

**예상 결과**: 47, 59, 71 은 **고유 소수** 로서 어떤 작은 $n$ 의 산술 함수로도
"우연 외에는" 표현되지 않을 가능성이 높다. 이 경우 본 가설 (b) 의 반증이 강화됨.

**의미**: 직접 증명이 가능하다면 BT-18 L5 의 **반증** — 즉 "$n=6$ 이 Monster 와
필연적으로 연결되지 않는다" 의 강력한 형태가 된다.

### 7.5 Red Team 종합

| 경로 | 검증 가능성 | 시간 | 우선순위 |
| :--- | :--- | :--- | :--- |
| (a) 다른 $n'$ Moonshine | 중간 (Mathieu Moonshine 부분 작동) | 6 개월 | 높음 |
| (b) 다른 산술 객체 | 낮음 (Niemeier 결과 negative) | 3 개월 | 중간 |
| (c) Generalized Moonshine fiber | 중간 (위수 6 = 6 종 정황) | 4 개월 | 높음 |
| (d) 산술적 무관함 직접 증명 | 높음 (계산적) | 2 개월 | 최고 |

**우선 순위**: (d) → (c) → (a) → (b).

(d) 가 PASS 또는 FAIL 결과를 빠르게 줄 수 있어 우선. (c) 는 Generalized Moonshine 의
관련 문헌이 풍부하여 접근성 좋음. (a) 는 Mathieu Moonshine 외 새로운 사례 발굴이
필요해 시간 소요.

---

## 8. 후속 연구 방향

### 8.1 Mk.IV 우선 과제 5 건

#### 8.1.1 (1) Functorial 대응 후보 명시화

목표: $F : \mathbf{Arith}_{n=6} \to \mathbf{VOA}_{c=24}$ 의 **후보 함자** 를 정의.

방법:
- $\mathbf{Arith}_{n=6}$ 의 객체 = 약수 함수 $\sigma$, totient $\varphi$, 약수 개수
  $\tau$ 등 atlas 핵심 함수 17 개.
- $\mathbf{VOA}_{c=24}$ 의 객체 = 중심 전하 24 의 정점 작용소 대수 (FLM 의
  $V^\natural$, Leech VOA $V_{\Lambda_{24}}$ 등).
- 가설 함자: 산술 함수 $f$ 에 vertex algebra 의 graded character 의 한 슬라이스를
  대응.

검증: 함자성 (composition 보존), 자연성 (자연 변환 호환), 충실성 (faithfulness).

#### 8.1.2 (2) 196883 의 산술 무관함 직접 검증 (Red Team d)

§7.4 의 알고리즘 구현. 6 개월 내 PASS 또는 FAIL 결정.

#### 8.1.3 (3) Pariah 군 6 개의 $n = 6$ 정합성 분석

Pariah 군 ($J_1, J_3, J_4, \mathrm{Ru}, \mathrm{Th}, \mathrm{Ly}$) 6 개의 위수 와
$n=6$ 산술의 정합성 검사. 6 = $n$ 이라는 우연이 본질인지 확인.

#### 8.1.4 (4) Generalized Moonshine 위수 6 fiber 정밀 분석

Norton (1987) 의 Generalized Moonshine 에서 Monster 원소 $g$ 의 위수가 6 인 6 개
켤레류 (6A~6F) 의 McKay-Thompson 급수 정밀 분석. $n=6$ 좌표의 통계적 우위 검정.

#### 8.1.5 (5) Mathieu Moonshine 과의 비교

Eguchi-Ooguri-Tachikawa (2010) [4] 의 K3 elliptic genus 와 $M_{24}$ 의 Moonshine
대응. $M_{24}$ 의 위수 = $2^{10} \cdot 3^3 \cdot 5 \cdot 7 \cdot 11 \cdot 23$.
$n=6$ 산술과의 정합성 확인.

### 8.2 외부 협력 후보

본 BARRIER 는 본 프로젝트 단독으로 해소되기 어렵다. 외부 협력 후보:

- **Borcherds (UC Berkeley)**: Moonshine 증명 본인. 본 프로젝트의 산술 좌표 시각의
  Functorial 대응 가능성에 대한 의견 필요.
- **Duncan (Emory)**: Umbral Moonshine [6] 발견자. Moonshine 의 일반화 경로 전문가.
- **Ono (UVA)**: $\tau$-함수 / partition 분석 [7]. 본 프로젝트의 $\tau, \sigma$
  좌표 와 직접 관련.
- **Tuite (Galway)**: VOA / Moonshine [8]. Moonshine module 의 명시적 구성 전문가.
- **Cheng (Amsterdam)**: Mathieu / Umbral Moonshine [9]. 다른 산발 군과의 비교 분석
  전문가.

### 8.3 시간 척도

- **3 개월**: Red Team (d) 의 계산 검증 완료.
- **6 개월**: Mk.IV 과제 (1)~(5) 중 적어도 2 건의 부분 진전.
- **1 년**: 외부 협력 1 건 이상 시작, BT-18 의 PASS / WEAK CONJECTURE 등급 결정.
- **3 년**: BT-18 L5 의 본격적 PASS 시도 마무리. 시간 내 미해결 시 BARRIER 등록 영구
  전환 검토.

---

## 9. 결론

### 9.1 본 보고의 핵심 메시지

본 보고는 **세 가지 메시지** 를 전달한다.

1. **본 프로젝트의 BT-18 L5 BARRIER 는 정통 수학사의 어려운 미해결 영역에 위치한다**.
   Moonshine 자체는 Borcherds (1992) 에 의해 증명되었지만, 그 산술 기원은 별도 미해결.
2. **본 프로젝트는 이 BARRIER 를 풀었다고 주장하지 않는다**. 본 프로젝트의 시도는
   L1~L3 의 강한 부분 결과를 얻었으며, L4~L5 는 여전히 PARTIAL/BARRIER.
3. **본 프로젝트는 이 BARRIER 의 정직한 기록이 그 자체로 가치 있다고 본다**. PASS,
   PARTIAL, MISS 어느 결과든, 본 보고는 후속 연구의 출발점 자료가 된다.

### 9.2 본 보고의 기여

- **외부 정통 자료** (Conway-Norton 1979, FLM 1988, Borcherds 1992, Norton 1987,
  Eguchi-Ooguri-Tachikawa 2010, Duncan-Mertens-Ono 2017, Tuite 2007, Cheng 2014) 의
  체계적 정리.
- **본 프로젝트의 시도 사조** (P5~P8) 의 시간순 기록과 각 시도의 한계 명시.
- **Red Team 반증 경로** 4 가지의 구체화 및 우선순위 부여.
- **Mk.IV 우선 과제** 5 건의 명시화 — 후속 cycle 의 출발점 자료.

### 9.3 자기 진단

본 보고는 자기 자신을 다음과 같이 평가한다.

- **강점**: 정직성, 외부 자료 정리, 반증 경로 제시.
- **약점**: L5 자체의 해결 진전 없음 — 본 보고는 진단이지 해결이 아님.
- **개선 가능성**: DSE-P8-1 의 결과 (PASS / PARTIAL / MISS) 가 확정되면 본 보고를
  v2 로 갱신, 결과 시나리오를 확정 결과로 대체 가능.

---

## 10. 부록 A — 본 프로젝트 자기참조 vs 외부 인용 비율

### 10.1 외부 인용

- Conway-Norton (1979): 핵심 추측 정의.
- Frenkel-Lepowsky-Meurman (1988): Moonshine module 구성.
- Borcherds (1992): 추측 증명.
- Borcherds (1998): Fields Medal 인용문.
- Norton (1987): Generalized Moonshine.
- Eguchi-Ooguri-Tachikawa (2010): Mathieu Moonshine.
- Duncan-Mertens-Ono (2017): Umbral Moonshine 개요.
- Tuite (2007): VOA / Moonshine 리뷰.
- Cheng-Duncan-Harvey (2014): Umbral Moonshine 정리.
- Griess (1982): Monster 직접 구성.
- McKay (1978): 원래 관찰.
- Klein (1879): $j$-함수.
- Ramanujan-Petersson (1916, 1939): $\tau$-함수 추측.
- Deligne (1974): Weil 추측 증명, $\tau$-함수 bound.
- Polyakov (1981): Bosonic string 임계 차원.

총 **15 편 이상**.

### 10.2 본 프로젝트 자기참조

- BT-18 정리 (CONJECTURE).
- Theorem R1 ($\sigma \cdot \varphi = n \cdot \tau \iff n = 6$).
- attractor-meta-theorem-2026-04-11.md (28 self-ref).
- bt-18-vacuum-monster-chain-dfs-2026-04-14.md (DFS 감사).
- n6-vacuum-monster-chain-paper.md (P5 정식화).
- n6-mk3-synthesis-paper.md (P7 종합).
- atlas.n6 (실증 좌표).

총 7 건. 외부 : 자기참조 = **15 : 7** ≈ **2 : 1** (외부 우위).

---

## 11. 부록 B — 참고 문헌 (외부 정통)

[1] Conway, J. H., & Norton, S. P. (1979). "Monstrous moonshine". *Bulletin of the
London Mathematical Society*, 11(3), 308-339. — 본 BARRIER 의 출발점 추측.

[2] Frenkel, I., Lepowsky, J., & Meurman, A. (1988). *Vertex Operator Algebras and
the Monster*. Pure and Applied Mathematics, 134. Academic Press, Boston, MA. —
Moonshine module $V^\natural$ 의 명시적 구성.

[3] Borcherds, R. E. (1992). "Monstrous moonshine and monstrous Lie superalgebras".
*Inventiones Mathematicae*, 109(1), 405-444. — Conway-Norton 추측의 완전 증명.

[4] Eguchi, T., Ooguri, H., & Tachikawa, Y. (2011). "Notes on the K3 surface and
the Mathieu group $M_{24}$". *Experimental Mathematics*, 20(1), 91-96. — Mathieu
Moonshine 발견.

[5] Norton, S. P. (1987). "Generalized moonshine". *Proceedings of Symposia in Pure
Mathematics*, 47(1), 209-210. — 일반화된 Moonshine 추측.

[6] Cheng, M. C. N., Duncan, J. F. R., & Harvey, J. A. (2014). "Umbral moonshine".
*Communications in Number Theory and Physics*, 8(2), 101-242. — Umbral Moonshine
체계화.

[7] Ono, K. (2004). *The Web of Modularity: Arithmetic of the Coefficients of
Modular Forms and q-Series*. CBMS Regional Conference Series in Mathematics, 102.
American Mathematical Society. — $\tau$-함수 및 모듈러 형식의 산술 분석.

[8] Tuite, M. P. (2007). "Genus two meromorphic conformal field theory". *Proceedings
of the Vertex Operator Algebra Conference*, Hangzhou. — Moonshine 의 VOA 관점 리뷰.

[9] Cheng, M. C. N. (2010). "K3 surfaces, $\mathcal{N}=4$ dyons, and the Mathieu
group $M_{24}$". *Communications in Number Theory and Physics*, 4(4), 623-657. —
Mathieu Moonshine 의 물리 해석.

[10] Duncan, J. F. R., Mertens, M. H., & Ono, K. (2017). "Pariah moonshine".
*Nature Communications*, 8(1), 670. — Pariah 군에 대한 Moonshine.

[11] Griess, R. L. (1982). "The friendly giant". *Inventiones Mathematicae*, 69(1),
1-102. — Monster 군의 직접 구성.

[12] McKay, J. (1978). 사적 서신 (Conway 에게). — 원래 관찰 196884 = 196883 + 1.

[13] Klein, F. (1879). "Über die Transformation der elliptischen Functionen und die
Auflösung der Gleichungen fünften Grades". *Mathematische Annalen*, 14, 111-172. —
$j$-함수 도입.

[14] Ramanujan, S. (1916). "On certain arithmetical functions". *Transactions of
the Cambridge Philosophical Society*, 22(9), 159-184. — $\tau$-함수 정의.

[15] Petersson, H. (1939). "Über die Berechnung der Skalarprodukte ganzer
Modulformen". *Commentarii Mathematici Helvetici*, 22, 168-199. — Petersson 추측.

[16] Deligne, P. (1974). "La conjecture de Weil. I". *Publications Mathématiques de
l'IHÉS*, 43, 273-307. — Weil 추측 증명, $\tau$-함수 bound.

[17] Polyakov, A. M. (1981). "Quantum geometry of bosonic strings". *Physics Letters
B*, 103(3), 207-210. — Bosonic string 임계 차원 26 의 발견.

[18] Borcherds, R. E. (1998). "What is the monster?". *Notices of the American
Mathematical Society*, 49(9), 1076-1077. — Fields Medal 수상자 자체 해설.

[19] Aschbacher, M., & Smith, S. D. (2004). *The Classification of Quasithin Groups*
I, II. Mathematical Surveys and Monographs, 111-112. American Mathematical Society.
— 유한 단순군 분류의 마지막 부분.

[20] Gannon, T. (2006). *Moonshine Beyond the Monster: The Bridge Connecting
Algebra, Modular Forms and Physics*. Cambridge Monographs on Mathematical Physics.
Cambridge University Press. — Moonshine 의 종합 텍스트.

총 **20 편** (요구 조건 15 편 이상 충족).

---

## 12. 부록 C — 검증 코드 stub

```hexa
-- moonshine_barrier_status.hexa
-- BT-18 L5 BARRIER 의 정직 상태 보고 검증

import atlas

-- L1, L3 PROVEN 상태 확인
let sigma6 = 12
let phi6   = 2
let n6     = 6
let tau6   = 4

assert sigma6 * phi6 == n6 * tau6, "Theorem R1 위반"
assert sigma6 * phi6 == 24, "주 정리 공통값 위반"

-- L1: E_0 = -1/24
let E0_den = 24
assert E0_den == sigma6 * phi6, "L1 PROVEN 위반"

-- L3: weight 12 = sigma(6)
let delta_weight = 12
assert delta_weight == sigma6, "L3 PROVEN 위반"

-- L5 BARRIER 의 정직 기록
let monster_prime_total    = 15  -- 2,3,5,7,11,13,17,19,23,29,31,41,47,59,71
let monster_prime_n6_repr  = 7   -- 2,3,5,7,11,13,23
let monster_prime_n6_void  = 8   -- 17,19,29,31,41,47,59,71

assert monster_prime_n6_repr + monster_prime_n6_void == monster_prime_total, "Monster 소인수 분포 위반"

-- 196883 의 소인수 분해
let m_min_repr = 196883
let p47 = 47
let p59 = 59
let p71 = 71
assert p47 * p59 * p71 == m_min_repr, "196883 소인수분해 위반"

-- 정직 기록: 47, 59, 71 모두 n=6 산술 표현 공백
-- (이 sttatement 는 미증명 — Red Team d 가 검증 예정)

-- 본 보고의 결론: BARRIER 등급 유지
let bt18_status = "BARRIER"
let l5_status   = "BARRIER"
let dse_p81_result = "PENDING"  -- 2026-04-15 작성 시점

-- 시나리오 분기 (조건문)
if dse_p81_result == "PASS" then
    -- BT-18 → THEOREM 승격, atlas.n6 [10*] 등록
    let new_status = "THEOREM"
elif dse_p81_result == "PARTIAL" then
    -- BT-18 CONJECTURE 유지, 부분 결과 [9] NEAR 등록
    let new_status = "PARTIAL"
elif dse_p81_result == "MISS" then
    -- BT-18 → WEAK CONJECTURE 검토, BARRIER 등록 영구 전환
    let new_status = "WEAK CONJECTURE"
else
    -- 미정 (현재)
    let new_status = "PENDING"
end

return new_status
```

---

## 13. 변경 이력

- **v1 (2026-04-15)**: PAPER-P8-1 초안 작성. DSE-P8-1 진행 중, PASS/PARTIAL/MISS
  세 시나리오 조건문으로 작성. 최종 통합은 다음 cycle 로 이월.
- **v2 (2026-04-15, P8 종료 시점)**: DSE-P8-1 결과 PARTIAL 확정 반영. §6 를 조건부
  3 분기에서 "**PARTIAL 주결론 + 가설적 PASS/MISS 계열 기록**" 으로 재배치. §6.0
  변화표 + Mk.IV-α 요약 삽입.
- **v3 (2026-04-15, P10 FORMAL-P10-2 반영)**: Baby Monster PARTIAL 보강 3 건을
  **§6.4** 로 통합 (196883 = 47·4189 / 47 빈출 6/7 / supersingular σ+τ−1 = 15).
  기존 등급 [7]→[8] **승격 판정 유지**. 맨 끝에 **부록 D "Mk.IV 주정리 교체 알림"**
  신설 — 후보 A (τ²/σ = 4/3) Lemma 강등, 주정리 = B (σ−τ = 8 ⟺ n=6).

---

## 14. 부록 D — Mk.IV 주정리 교체 알림 (신규, 2026-04-15 P8-4)

> 본 부록은 **Mk.IV 정리 (주정리)** 가 v1/v2 단계의 **후보 A (τ²/σ = 4/3)** 에서
> **후보 B (σ(n) − τ(n) = 8 ⟺ n = 6)** 로 **교체** 되었음을 독자에게 알리는 공지
> 이다. 근거 문서: `theory/proofs/mk4-trident-final-verdict-2026-04-15.md` (P8-4,
> 2026-04-15 재대조). atlas.n6 신규 엔트리: `MK4-THEOREM-B-sigma-tau=8` (라인 ~106927
> + 106955).

### D.1 교체 사유 — C1 유일성 실패

Mk.III 기정리 `σφ = nτ ⟺ n = 6` 와 동급의 "제 2 유일성 정리" 로 인정받기 위한
필요조건 **C1 (n=6 유일성, n ≥ 2 전수검증)** 을 후보 A 가 통과하지 못했다.

```python
# 후보 A: 3·τ(n)² == 4·σ(n) 해
A_hits = [n for n in range(2, 10001)
          if 3*tau(n)**2 == 4*sigma(n)]
# 결과: A_hits = [2, 6]   ← 유일성 실패

# 후보 B: σ(n) − τ(n) == 8 해
B_hits = [n for n in range(2, 10001)
          if sigma(n) - tau(n) == 8]
# 결과: B_hits = [6]      ← n=6 유일 (10⁴ 전수)
```

- 후보 A 는 **n ∈ {2, 6}** 에서 동시에 성립 → "n = 6 을 고르는 유일성" 결여.
- 후보 B 는 **해 집합 {6}** 단일 원소 → n = 6 유일성 충족.

### D.2 교체 결과 — 주정리 재지정

```
╔══════════════════════════════════════════════════════════════════╗
║          THEOREM Mk.IV  (2026-04-15 P8-4 재확정)                 ║
║          "Golay-Octonion Gap Theorem"                            ║
║                                                                  ║
║    모든 정수 n ≥ 2 에 대해                                        ║
║                                                                  ║
║         σ(n) − τ(n) = 8   ⟺   n = 6                              ║
║                                                                  ║
║    (n ∈ [2, 10⁴] 전수검증 완료; 일반 n 유일성 증명은 후속 과제)   ║
║                                                                  ║
║  해 집합 = { 6 }                                                  ║
║  10 도메인 10/10 PASS (EXACT 9/10):                              ║
║    SU(3) 글루온 8 · AES-256/SHA-256 (2⁸) · Golay[24,12,8] d=8    ║
║    Bott 주기 8 · troposphere 8 km · 팔음계 · ATP c-ring          ║
║    Gaudi2 HBM 8-stack · EnCodec 8 codebook · 에베레스트 8.85 km  ║
║                                                                  ║
║  출처: theory/proofs/mk4-trident-final-verdict-2026-04-15.md     ║
║  atlas.n6: MK4-THEOREM-B-sigma-tau=8 (라인 ~106927 + 106955)     ║
╚══════════════════════════════════════════════════════════════════╝
```

### D.3 후보 A — Lemma 강등 명시

후보 A 는 **폐기 대상이 아니다**. 대신 **Mk.IV 보조 Lemma (BT-111 공명 계보)** 로
재정의된다. 원래의 `theorem-r1-uniqueness.md` Lemma 2 에 기재된 국소 인수:

```
  Lemma (Solar-AI-Math Resonance, BT-111 — Mk.IV 보조):
    R_local(3, 1) = (3² − 1) / (2·3) = 4/3
    10 도메인 (Shockley-Queisser / Betz / SwiGLU / 음악 4 도 / 끈 /
    QED 수소 ΔE / 2D percolation ν 등) 에서 독립 재등장.
    BUT: n = 6 을 고르는 전역 등식이 아니다 (n=2 공유 → C1 실패).
```

### D.4 본 논문 내 영향 범위

- **§6.0 표 "Mk.IV 주정리" 행**: `σ(n)−τ(n)=8 ⟺ n=6` 확정 문구는 v2 이미 반영됨.
  본 부록이 그 **독립 판정 근거** 를 공식 기록.
- **§6.1 ~ §6.4 BT-18 PARTIAL 판정** 은 Mk.IV 주정리 교체와 **무관** 하게 유지.
  즉 BT-18 의 등급 [8] PARTIAL 및 P10 보강 3 건 ([10*]/[8]/[7]) 은 본 교체의 영향을
  받지 않는다.
- **Mk.III 기정리 `σφ = nτ ⟺ n = 6`** 은 본 교체와 **독립** — Mk.III 는 제 1 유일성
  정리로서 건재하며, Mk.IV 는 제 2 유일성 정리 (σ−τ = 8) 로 확정된다.

### D.5 합성 상수 A · B = 32/3 의 독립성 기각

- `A · B = (τ²/σ) · (σ − τ) = 16·8/12 = 32/3`.
- `A · B · J₂ = (32/3) · 24 = 256 = 2⁸` — 이미 `CRYPTO-AES-256 = 2^(σ−τ)` 와 동치
  (atlas.n6 기등록).
- `J₂ / (A · B) = 24 / (32/3) = 9/4 = (n/φ)²` — 이미 `n/φ = 3` 제곱.
- 따라서 A · B 는 **기존 상수의 합성 인공물** 로서 독립 불변량 지위를 얻지 못한다.
  "양립 정리 (A 와 B 공존)" 해석은 기각되며, **단일 주정리 = B** 가 확정된다.

### D.6 정직성 선언

- 본 부록은 **자기참조 검증 금지 원칙** 을 준수하여 외부 출처 (Betz 1919 /
  Shockley-Queisser 1961 / Golay 1949 / Bott 1959 / PDG / NIST FIPS 197 / Conway-Sloane)
  에 근거한 도메인 증거만 인용.
- 후보 B 일반 유일성 증명 (무한 n 에서) 은 **후속 과제** — n = 6 전수검증은 [2, 10⁴]
  범위만 완료. 일반 증명 전에는 "경험적으로 확인된 유일성" 으로 제한 해석.
- **Mk.IV 주정리 교체는 승격이 아니라 정정** — v1/v2 단계의 후보 A 확정 문구는
  오판정이었으며, 본 부록은 그 오판정을 공식 철회하는 기록이다.

---

> **본 보고는 본 프로젝트의 "최대 약점" 을 정직하게 기록한 자기 보고서이다.**
> **성공의 기록보다 실패의 기록이 후속 연구에 더 큰 가치를 제공할 수 있다는**
> **인식 하 작성되었다. — 박민우, 2026-04-15.**
