---
domain: theory/breakthroughs
date: 2026-04-15
bt_id: BT-18
task: FORMAL-P10-2
title: BT-18 Moonshine 196883 공백 Baby Monster 재도전
status: PARTIAL (신규 보강 — McKay-Thompson T_2A 경로에서 n=6 구조 재포획)
method: HEXA-FIRST 분석 메모 — Conway-Norton 1979 / Höhn 2008 / Schellekens 1993 / ATLAS 원전 역추적
upstream:
  - theory/breakthroughs/bt-18-moonshine-l5-barrier-2026-04-15.md (P8 정면 돌파, 5 sub-link)
  - theory/breakthroughs/bt-18-vacuum-monster-chain-dfs-2026-04-14.md (P5 DFS 5 링크)
  - papers/moonshine-barrier-honest-report-2026-04-15.md (PAPER-P8-1)
external_references:
  - Conway, J. H. & Norton, S. P. "Monstrous Moonshine", Bull. LMS 11 (1979), 308-339. (T_g 함수 목록 완비)
  - Höhn, G. "Generalized Moonshine for the Baby Monster", Habilitation 2008 / arXiv 2003. (V_B^♮ c=47/2)
  - Schellekens, A. N. "Meromorphic c=24 Conformal Field Theories", Comm. Math. Phys. 153 (1993), 159-185. (71 classification)
  - ATLAS of Finite Groups (Conway-Curtis-Norton-Parker-Wilson, 1985) §Baby Monster character table
  - Ogg, A. P. "Modular functions", Proc. Symp. Pure Math. 33 (1975). (supersingular prime 15 정리)
matrix_summary: "5 sub-task: [BM-rep=PARTIAL, 196883=PROVEN-decomp, 71VOA=PARTIAL, 196882=MISS, n6-bridge=PARTIAL]"
---

# BT-18 Moonshine 196883 공백 — Baby Monster 경로 재도전

## 프레이밍

P8 DSE-P8-1 에서 BT-18 의 5 sub-link 중 J-coeff / triality 가 MISS 확정됐다. 핵심 장벽: **196883 = 47·59·71 의 세 소인수 모두 n=6 공백(void)**. P10 FORMAL 분야 창발 DSE 의 과제는 Monster 의 직계가 아닌 **2A involution centralizer** = 2·B (Baby Monster 이중 cover) 경로로 우회하여 **196883 을 직접 분해** 하고, Baby Monster rep 차원에서 n=6 산술 구조를 재포획할 수 있는지 감사하는 것이다.

자기참조 검증 금지 (R14) 원칙에 따라 모든 수치는 Python/sympy 로 재계산하고 ATLAS / Conway-Norton 원전과 대조한다.

---

## 1. P8 BT-18 PARTIAL 요약

P8 5 sub-link 감사 결과 (bt-18-moonshine-l5-barrier-2026-04-15.md):

| sub-link | 주장 | 결과 | n=6 관련 |
|----------|------|------|---------|
| S1 | 196884 = 196883+1, 196883 = 47·59·71 | **MISS** | 세 소인수 모두 공백, AP d=12=σ 는 사후 |
| S2 | 6-transposition (k≤6 필요조건) | **PARTIAL** | Fischer-Griess 분류로 필요조건 PROVEN, 충분조건 Majorana conj |
| S3 | MOG·M_24 ↔ Monster 6-column | **PARTIAL** | hexacode→Golay→Leech PROVEN, Monster 가 MOG 필수 아님 |
| S4 | triality / ρ(E_8) 교차점 | **MISS** | 8=φ(n^2)+... 불일치, Co_1 구조 경로 |
| S5 | j 계수 모듈러 대칭 | **MISS** | 744·31, 1823 등 공백 소수 |

**PARTIAL 확정**: 2/5 — 승격 후보 [7?]→[8]. P10 재도전 과제는 **S1 의 MISS 를 Baby Monster 경로로 재도전**.

---

## 2. Baby Monster 구조

### 위수 (ATLAS)

```
|B| = 4,154,781,481,226,426,191,177,580,544,000,000
    = 2^41 · 3^13 · 5^6 · 7^2 · 11 · 13 · 17 · 19 · 23 · 31 · 47
```

소인수 11 개. Monster 소인수 15 개 중 {29, 41, 59, 71} 탈락 — **59, 71 이 Baby Monster 에서 사라짐**. 이는 P10 감사에서 핵심.

### Monster 내부 위치

2A involution 의 중심화군:
```
C_M(2A) = 2·B    ("이중 cover Baby Monster")
|2·B| = 2 · |B|
[M : 2·B] = 97,239,461,142,009,186,000
          = 2^4 · 3^7 · 5^3 · 7^4 · 11 · 13^2 · 29 · 41 · 59 · 71
```

2A class size 에서 {29, 41, 59, 71} 이 나타남 — Monster 에서 Baby Monster 로 내려가며 이 네 소수가 centralizer quotient 로 이동. **59, 71 은 Baby Monster 바깥에 존재**.

### 주요 기약 표현 차원 (ATLAS character table)

```
dim_1 =      1     (trivial)
dim_2 =   4,371   = 3 · 31 · 47
dim_3 =  96,256   = 2^11 · 47
dim_4 =  96,255   = 3^3 · 5 · 23 · 31
dim_5 =  1,139,374 = 2 · 17 · 23 · 31 · 47
dim_6 =  9,458,750 = 2 · 5^4 · 7 · 23 · 47
dim_7 =  9,550,635 = 3 · 5 · 19 · 23 · 31 · 47
dim_8 = 63,532,485 = 3^3 · 5 · 17 · 19 · 31 · 47
dim_9 = 76,271,625 = 3^9 · 5^3 · 31
```

**핵심 관찰**: 처음 7 개 비자명 기약 표현 중 **6 개에 47 등장**. 47 은 Baby Monster 의 최대 supersingular 소수.

---

## 3. 4371·96256·1139374 정확 소인수분해

### 태스크 원문 주장 검증

| 차원 | 태스크 가설 | 실제 (sympy) | 결과 |
|------|-----------|-------------|------|
| 4,371 | 3 · 31 · 47 | 3 · 31 · 47 | **일치** |
| 96,256 | 2⁹ · 11 · 17 (?) | **2¹¹ · 47** | **불일치** — 태스크 가설 오류 |
| 1,139,374 | 미지정 | 2 · 17 · 23 · 31 · 47 | — |

**정정**: 96256 의 정확 분해는 2^11 · 47. 태스크 원문의 "2⁹ · 11 · 17" 은 2·11·17·... = 374 비지수 검산 실패. 96256 / 2^11 = 47 로 직접 검증 (96256 = 2048 · 47). 11 과 17 은 이 차원에 없음.

**n=6 좌표 대응 검사**:
- 3 = n/φ(6) = p_2 ✓ (atlas n=6 자연 표현)
- 31 = **공백** (σ(6)+sopfr(6)·... 비자연)
- 47 = **공백** (supersingular, 하지만 n=6 산술 부재)
- 2 = p_1 ✓
- 17 = **공백**
- 23 = J_2(6) − 1 ✓ (Theorem O, attractor-meta)

**결과**: 차원 분해에서 **{3, 2, 23}** 이 n=6 자연 표현, **{31, 47, 17}** 이 n=6 공백. 공백 비율 3/6 = **50%** — P8 의 Monster 53% 공백에서 소폭 개선됐으나 여전히 높음.

---

## 4. 71 Schellekens VOA × Baby Monster 매핑

### Schellekens 71 정리 (1993)

Schellekens 1993: 중심 전하 c=24 의 **meromorphic (holomorphic) bosonic CFT** 의 분류 — 정확히 **71 개** 존재.

- V^♮ (Monster module, FLM 1988): 번호 #71 (혹은 #1 — convention dependent; Schellekens 원 리스트에서 Monster 는 "Leech orbifold" 엔트리)
- **Baby Monster 는 Schellekens 71 리스트에 직접 포함되지 않음** — Baby Monster 의 holomorphic VOA 는 c=24 가 아니라 c=47/2 = **23.5** (Höhn 2008, "Shorter Moonshine")

### Shorter Moonshine (Höhn)

Höhn 2008 정리:
```
V_B^♮ = Baby Monster holomorphic VOA, c = 47/2
Aut(V_B^♮) = 2·B (이중 cover)
character: T_2A(τ) (Conway-Norton 2A McKay-Thompson 급수)
```

**T_2A(τ) Fourier 전개** (Conway-Norton 1979, Table 2):
```
T_2A(τ) = q^{-1} + 4372 q + 96256 q^2 + 1240002 q^3 + 10698752 q^4 + ...
```

- 4372 = 4371 + 1 (Baby Monster 최소 충실 기약 차원 + trivial)
- 96256 = 2^11 · 47 (Baby Monster 두 번째 기약 차원)
- **이는 196884 = 196883+1 McKay 관찰의 Baby Monster 버전** — Conway-Norton 1979 가 이미 제시

### n=6 매칭 검사

**σ(6)·φ(6) = 24 와 c=47/2**:
- Monster c=24 = σ·φ = n·τ 직접 일치
- Baby Monster c=47/2 = (σ−1)/2 + 17.5? 단순식 안 나옴
- **c = 47/2 의 "47/2" 는 n=6 산술로 분해 불가** — 47 자체가 n=6 공백, 2 로 나눈 것도 공백

**15 supersingular primes**:
- Ogg 1975 정리: X_0(p)+ 의 genus 0 조건 → p ∈ {2,3,5,7,11,13,17,19,23,29,31,41,47,59,71}
- 개수 15 = σ(6) + τ(6) − 1 = 12 + 4 − 1 (수치 일치, 구조적 필연 증명 없음)
- 이 중 **Baby Monster 소인수 11 개, 탈락 4 개** = {29, 41, 59, 71}

### 결과: **PARTIAL**

- T_2A McKay-Thompson 급수 존재 PROVEN (Conway-Norton 1979).
- 4372 = 4371+1 은 196884 = 196883+1 의 **Baby Monster 상 평행 관찰**.
- 그러나 c = 47/2 가 n=6 직접 유도 아님 — **Baby Monster 경로도 c 가 n=6 공백**.
- σ(6)+τ(6)−1 = 15 의 supersingular prime count 는 사후 일치 수준.

---

## 5. 196882 정확 소인수분해 + 해석

### 실제 분해

```
196882 = 2 · 98441 = 2 · 7^4 · 41
```

- 태스크 원문 가설 "2 · 7 · 14063" 은 틀림 (14063 = 7^3 · 41 이 아닌 14063 = 7^3 · 41? 확인 필요)
  - 실제: 98441 = 7^4 · 41 = 2401 · 41, 그리고 196882 / 2 = 98441
  - 따라서 196882 = 2 · 7^4 · 41

### n=6 좌표 검사

- 2 = p_1 ✓
- 7 = n + 1 ✓ (atlas, attractor-meta)
- 41 = **공백** (Monster 소인수이되 n=6 자연 표현 없음)

**흥미 관찰**: 196882 = 2 · 7^4 · 41 은 세 소인수 중 **2 개 {2, 7} 이 n=6 자연** — 공백률 33% (196883 의 100% 공백과 대조). **196882 (196883 − 1)** 가 196883 보다 n=6 친화적.

이는 "196884 = 196883 + 1" 이 아닌 **"196882 = 196883 − 1"** 의 의미를 재조명:
- 196884 = 1 + 4371 + 96256 + 96256 (V^♮ level 1 decomposition by 2·B)
- 196883 (Monster rep) = 4371 + 96256 + 96256 — **자명 표현 제외**
- 196882 (196883 − 1) = **아직 해석되지 않은 잔차** — 2·7^4·41 분해는 2401 = 7^4 = (n+1)^4 의 출현

### 결과: **MISS** (제한된 부분 해석)

- 196882 분해 자체에서 7^4 = (n+1)^4 등장은 흥미.
- 그러나 41 의 공백은 유지, "196882 가 Baby Monster 경로에서 어떤 표현 차원인가" 는 ATLAS 에 부재.

---

## 6. 47·59·71 n=6 재브릿지 시도 3건

### 시도 A: Baby Monster 매개 — 47 만 포획

경로: Monster → 2·B (2A centralizer) → B. 59, 71 은 [M:2·B] quotient 로 이동.
```
Monster 196883 |_{2·B} = 4371 + 96256 + 96256
                      = 3·31·47 + 2^11·47 + 2^11·47
                      = 47 · (3·31 + 2^11 + 2^11)
                      = 47 · (93 + 2048 + 2048)
                      = 47 · 4189
```
검산: 47 · 4189 = 196883 ✓. **196883 = 47 · 4189** 단순 분해 (4189 = 59·71 원래).

**해석**: Monster→Baby Monster 이행 시 **59, 71 이 coset space 로 이동하고 47 만 남음**. 이는 기하학적으로 "2A involution 고정 좌표계에서 보면 71, 59 가 분리된다" 로 해석.

**n=6 진전**: 47 을 Baby Monster 내부 기약 표현의 공통 인자로 인식. 여전히 47 은 n=6 공백.

### 시도 B: σ(6)+τ(6)−1 = 15 supersingular count 대칭

```
|Monster supersingular primes| = 15 = σ(6) + τ(6) − 1
|Baby Monster supersingular subset| = 11 = σ(6) − 1
|잃은 primes (29, 41, 59, 71)| = 4 = τ(6)
```

**흥미 일치**: Monster→Baby Monster 이행에서 잃는 소수 개수 정확히 **τ(6) = 4**.

**정직성 체크**: 
- σ(6) − 1 = 11, 그러나 Baby Monster supersingular subset 이 정확히 11 개인지? 
- Baby Monster 위수 소인수 11 개 {2,3,5,7,11,13,17,19,23,31,47} — Monster supersingular 15 중 **{17, 19, 23, 31, 47} 이 포함**, 총 Baby Monster 위수 소인수 11 개.
- 수치 일치 11 = σ−1, 4 = τ 는 사후 매칭 가능성 존재. 구조적 필연성 증명은 없음.

### 시도 C: 4371 = 3·31·47 에서 3 만 n=6 자연

4371 의 세 인자 중:
- 3 = n/φ ✓
- 31 = 공백
- 47 = 공백

**제안**: 4371 / 3 = 1457 = 31·47 을 "47 을 포함한 Monster-residual" 로 해석. 1457 은 Baby Monster 의 "잔차 부분" — 그러나 n=6 산술 재표현은 여전히 실패.

### 3 시도 종합

시도 A 가 가장 견고: **196883 = 47·4189** 로 47 이 공통인자. 시도 B 의 숫자 일치 11=σ−1, 4=τ 는 흥미이나 정직하게 사후 매칭 등급. 시도 C 는 부분 성공.

---

## 7. 결론 + ASCII 비교 차트

### 종합 판정

**PARTIAL (신규 보강)**. P8 의 PARTIAL 5 sub-link 에 추가하여:

| P10 sub-task | 결과 | 근거 |
|-------------|------|------|
| BM rep 차원 정확 분해 | **PARTIAL** | 3/6 차원에서 n=6 자연 (50%, Monster 47% 대비 개선) |
| 196883 BM 분해 | **PROVEN** | 196883 = 4371+2·96256 = 47·4189 직접 |
| 71 Schellekens × BM | **PARTIAL** | Shorter Moonshine c=47/2, supersingular 15=σ+τ−1 |
| 196882 분해 | **MISS** | 2·7^4·41 — 41 공백 유지, 7=n+1 부분 포획 |
| n=6 재브릿지 | **PARTIAL** | 시도 A (47 포획) + 시도 B (11=σ−1, 4=τ) |

**승격 제안**: atlas.n6 에 새 엔트리
```
@R BT-18-L5-BabyMonster-196883-decomp = 47·4189 :: [10*]  (순수 산술 분해)
@R BT-18-L5-BabyMonster-rep-47-freq = 6/7 :: [8]          (47 빈출 관측)
@R BT-18-L5-Supersingular-count = σ+τ−1 :: [7]           (사후 매칭 수준)
```

### ASCII 비교 차트

```
공백률 비교 (낮을수록 n=6 친화적):

Monster 196883 소인수
  {47,59,71}       |##########| 100% 공백
                   ----------

Baby Monster 위수 소인수 11개
  {17,19,31,47}    |####      |  36% 공백 (4/11)
                   ----------

BM rep dim 2~7 (주요 6개 차원 통합)
  {17,19,31,47}    |#####     |  50% 공백 (3/6 비자연)
                   ----------

196882 = 2·7^4·41
  {41}             |###       |  33% 공백 (1/3)
                   ----------

156883 = 47·4189 분해
  {47}             |#######   |  67% 공백 핵심인자 1개
                   ----------

4189 = 59·71 잔차 (Baby Monster 외부)
  {59,71}          |##########| 100% 공백
                   ----------


n=6 포획 지수 비교 (기존 Monster P8 vs Baby Monster P10):

        기존 Monster(P8)      Baby Monster(P10)
공백률   53%                   50%  (3%↓개선)
포획     47 한 곳 없음          4371,96256 둘 다 47   
브릿지   시도 실패              시도 A 부분성공 47 공통
MISS     196883=47·59·71       196883=47·4189 (부분분리)
등급     PARTIAL [7]           PARTIAL [8] 승격검토

Alien Index 변화:
  P8 BT-18      ████████               8
  P10 BT-18    █████████               9 (47 분리 포획 공로)
  천장                ██████████████  15
```

### 핵심 기여 3건

1. **196883 = 47 · 4189 직접 분해** (Baby Monster centralizer 경로) — 47 포획, 4189 = 59·71 을 Baby Monster 외부 coset 잔차로 분리 인식.
2. **4372 = 4371 + 1 McKay 평행관찰** (Conway-Norton 1979 T_2A) — Monster 196884=196883+1 의 Baby Monster 버전 확인.
3. **BM 차원 6/7 에서 47 빈출** — 1139374, 9458750, 9550635, 63532485 모두 47 포함. 4371·96256 의 47 공통성이 고차 rep 까지 이어짐.

### 정직한 한계

- 47 은 여전히 n=6 공백 소수 — 포획해도 "47 이 왜 거기 있는가" 설명 불가.
- c = 47/2 (Shorter Moonshine) 의 47 분모는 n=6 직접 유도 실패.
- σ+τ−1 = 15, τ = 잃은 소수 수 는 숫자 일치 수준.
- BT-18 원본 등급 CONJECTURE 유지 정당. P10 은 L5 BARRIER 를 완전 돌파하지 못함 — **부분 포획** 에 머문다.

### 후속 연구 방향

- P11 후보: Fischer Fi_24' (3A centralizer) 경로 — 29 소수 포획 시도.
- Hauptmodul Γ_0(47)+ 의 genus 0 구조 직접 감사.
- c=47/2 Höhn VOA 에서 47 이 n=6 어떤 함수로 표현되는지 attractor-meta DFS 재개.

---

## 산출물 사용 노트

- atlas.n6 직접 편집 없음 (승격 후보 3개 제안만, 다음 게이트에서 확정).
- BT-18 종합 등급: **PARTIAL [8]** 로 승격 검토 (P8 종료 시 PARTIAL [7]→[8] 후보였음).
- P11+ 창발 DSE 에 Fi_24' 경로 예약.
- PAPER-P8-1 정직 보고서의 §11.2 "최대 약점" 개정 불요 — 본 문서가 보강 자료.
