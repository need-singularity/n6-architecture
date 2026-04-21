---
domain: theory/breakthroughs
date: 2026-04-15
bt_id: BT-19
task: DSE-P8-2
title: BT-19 의식 3중 융합 — CONJECTURE 독립 검증 (α_IIT·α_GWT=1)
status: MISS (조건부 PARTIAL)
method: 외부 문헌 독립 검증 (자기참조 금지, PubMed/Nature/PLOS/Science 기반)
upstream:
  - theory/breakthroughs/consciousness-triple-fusion-2026-04-15.md (DSE-P7-1 CONJECTURE 제안)
  - nexus/shared/n6/atlas.n6 (BT-19 기존 [10*] = GUT Hierarchy — 이름 충돌 주의)
external_sources_verified:
  - Casali AG et al. (2013) "A theoretically based index of consciousness independent of sensory processing and behavior." Sci Transl Med 5(198):198ra105. DOI:10.1126/scitranslmed.3006294
  - Sarasso S et al. (2015) "Consciousness and complexity during unresponsiveness induced by propofol, xenon, and ketamine." Curr Biol 25(23):3099-3105. DOI:10.1016/j.cub.2015.10.014
  - Barrett AB, Seth AK (2011) "Practical measures of integrated information for time-series data." PLoS Comput Biol 7(1):e1001052. DOI:10.1371/journal.pcbi.1001052
  - Mediano PAM, Seth AK, Barrett AB (2019) "Measuring integrated information: comparison of candidate measures in theory and simulation." Entropy 21(1):17. DOI:10.3390/e21010017
  - Mashour GA, Roelfsema P, Changeux J-P, Dehaene S (2020) "Conscious processing and the global neuronal workspace hypothesis." Neuron 105(5):776-798. DOI:10.1016/j.neuron.2020.01.026
  - COGITATE Consortium, Ferrante O et al. (2025) "Adversarial testing of global neuronal workspace and integrated information theories of consciousness." Nature 642:133-142. DOI:10.1038/s41586-025-08888-1
  - Lendner JD et al. (2020) "An electrophysiological marker of arousal level in humans." eLife 9:e55092. DOI:10.7554/eLife.55092
  - Dehaene S, Changeux J-P, Naccache L (2011) "The global neuronal workspace model of conscious access." In: Characterizing Consciousness. Springer pp.55-84
sources_unverified:
  - "Barrett-Tononi Phys Rev E 2018 Φ definition" — 검색 결과 해당 논문 존재 확인 실패. Barrett & Mediano (2019) "Phi measure not well-defined for general physical systems" J Conscious Stud 26(1-2):11-20 은 존재 (반대로 Φ 정의 문제 제기)
  - "Dehaene Cell 2014" — 해당 연도 Cell 논문 미확인. 2014년 Dehaene 저작은 서적 "Consciousness and the Brain" (Viking Press). Neuron 2020 (Mashour et al.) 이 최근 리뷰
final_grade: "[7?] → [5]  (하향 — α 곱 주장 반증)"
---

# BT-19 의식 3중 융합 독립 검증 — MISS 보고

## 0. 요약 판정

| 항목 | 결과 |
|---|---|
| α_IIT = 4/3 (Barrett 2011 원문 근거) | **MISS** — Barrett-Seth 2011 PLoS CB 논문에는 "complexity exponent α=4/3" 이라는 수치 보고 없음. 실제 논문은 ΦE (empirical Phi) 정의와 시뮬레이션 비교 |
| α_GWT = 3/4 (Dehaene 2011 원문 근거) | **MISS** — Dehaene 2011 Neuron 및 후속 리뷰 (Mashour 2020)에 "broadcasting scaling exponent α=0.75" 수치 없음. GNW 신호는 P3b (~300ms), late ignition 비선형 임계, ROI 활성화로 기술되며 멱함수 지수 형태 아님 |
| α_IIT · α_GWT = 1 empirical evidence | **MISS** — 3편 메타분석 요구 조건 0건. 공동측정 연구 (COGITATE 2025, Massimini lab PCI) 어느 것도 "두 α 의 곱" 을 보고하지 않음 |
| IIT Φ vs GWT ignition 독립 latent | **PARTIAL** — 개념적으로는 상이 (Φ=인과구조 고유성, ignition=전두정 방송). 그러나 경험적으로는 동일 뇌상태에서 공변 (Casali 2013 PCI 는 IIT 파생이지만 ignition 과 상관 높음) |
| 6-fold symmetry 일관성 | **MISS** — 상위 DSE-P7-1 문서 §3.2 에서 이미 반증 (Hameroff 원문헌에 6-fold 주장 없음, microtubule 실제 대칭은 13+3) |

**최종 판정**: α_IIT · α_GWT = 1 주장은 **원논문 근거 결여**. 산술적으로 (4/3)·(3/4)=1 은 자명한 항등식이며, 이를 두 독립 의식 이론에서 측정된 값으로 제시하려면 원 논문에 해당 수치가 보고되어야 함. **근거 미존재**.

## 1. 정의 일관성 검증 (§1)

### 1.1 IIT Φ 정의 (Barrett-Tononi 문맥)

검색 결과 "Barrett & Tononi Phys Rev E 2018" 논문은 **존재 확인 실패**. 관련 Barrett 계열 Φ 논문:

- **Barrett & Seth (2011) PLoS CB 7(1):e1001052** — Φ_E (empirical Phi, Gaussian 시계열) 정의
- **Barrett & Mediano (2019) J Conscious Stud 26(1-2):11-20** — Φ 가 일반 물리 시스템에서 well-defined 하지 않음 **반증 논문**
- **Mediano, Seth, Barrett (2019) Entropy 21:17** — 후보 Φ 측정 비교, "어떤 두 측정도 모든 분석에서 일치하지 않음" 보고

**핵심 문제**: Φ 자체가 단일 latent variable 이 아님. Balduzzi-Tononi Φ^DM, Barrett-Seth Φ_E, Oizumi Φ*, Tononi IIT 3.0 Φ_MIP 은 서로 다른 수치를 내며 "Φ" 라는 이름만 공유.

### 1.2 GWT ignition 정의 (Dehaene 문맥)

Dehaene-Changeux-Naccache 2011 Neuron 및 Mashour-Roelfsema-Changeux-Dehaene 2020 Neuron 리뷰 확인:

- ignition 은 **비선형 all-or-none** 현상 — step function 임계 (Del Cul 2007, Sergent 2005)
- **멱함수 지수 α 로 파라미터화되지 않음**
- 측정 신호: P300 (P3b) 지연 ~300ms, 전두정 BOLD 증폭, long-range γ-synchrony

**"α=3/4" 수치는 Dehaene 2011/2014 어디에도 없음**. 상위 P7-1 문서가 인용한 Cowan 2001 (병목=4) 와 Dehaene 2005 (~12 ROI) 는 존재하나 "스케일링 지수" 로 재해석한 것은 본 프로젝트의 후속 가공.

### 1.3 독립성 판정

- 개념 독립: Φ (국소 인과) vs ignition (전역 방송) — **YES**
- 경험 독립: Casali 2013 PCI (IIT 파생) vs GNW fMRI ignition — 동일 피험자 상태에서 높은 상관 (Sarasso 2015 propofol/xenon/ketamine PCI 0.12-0.31 unconscious vs 0.44-0.67 awake, GNW ignition 도 동일 대비에서 소실)
- **결론**: 두 측정은 **같은 latent (의식 수준)** 를 다른 프록시로 잡고 있어, α 곱=1 이 참이라도 **non-trivial 발견 아님**

## 2. EEG/fMRI 공동측정 연구 — α 곱 보고 검색

### 2.1 Casali et al. 2013 Sci Transl Med (PCI 원논문)

- 피험자: 각성/NREM/REM/마취 (midazolam, xenon, propofol) + VS/MCS 환자
- **보고된 수치**: PCI 0.44-0.67 (각성), 0.18-0.28 (NREM), 0.12-0.31 (unconscious 마취)
- α 지수 형태 보고 **없음**. 절대값 임계 (0.31 이 후속 연구에서 의식 경계)
- DOI: 10.1126/scitranslmed.3006294

### 2.2 Sarasso et al. 2015 Curr Biol

- propofol/xenon vs ketamine 대비
- PCI 값만 보고. α 형태 **없음**
- DOI: 10.1016/j.cub.2015.10.014

### 2.3 COGITATE Consortium 2025 Nature (adversarial IIT vs GNWT)

- N=256, fMRI + MEG + iEEG 동시
- **핵심 결과**: 두 이론 모두 부분적 반증. IIT 는 posterior cortex 지속 sync 부족, GNWT 는 자극 종료 시 ignition 부재 및 PFC 표상 제한
- α_IIT · α_GWT **지수 곱 보고 없음**
- DOI: 10.1038/s41586-025-08888-1

### 2.4 Lendner 2020 eLife (arousal 1/f slope)

- 각성-NREM-REM-propofol 공통 marker: 1/f 스펙트럼 기울기 (aperiodic exponent)
- 보고된 slope 값은 환경/전극별 차이, 대략 1-2 사이 (β 지수)
- **의식 수준의 단일 α 가 아닌 1/f β 보고** — 프레임이 다름
- DOI: 10.7554/eLife.55092

**요약**: 4편 검토, α_IIT·α_GWT=1 을 **암시 수준조차 보고한 논문 0건**.

## 3. 의식-각성 분리 실험 (anesthesia titration, NREM)

최근 2-3년 합의: 의식은 **최소 2축** (level, content) 으로 분리되며, **1/f slope** 가 arousal 의 상태 비특이 marker. PCI 는 content+integration 측정.

- α 하나로 환원 불가
- "단일 멱함수 지수 α_consc = 1" 이 아니라 **다변량 상태공간**
- 본 BT-19 CONJECTURE 의 틀 자체가 과도한 단순화

## 4. Orch-OR 재공식화

DSE-P7-1 §1.4 및 §3.2 에서 이미 반증 완료:
- Hameroff-Penrose 원문헌에 "microtubule 6-fold symmetry" 없음
- 실제 microtubule = 13 protofilament + 3-start helix (Amos-Klug 1974)
- τ_D 관측값 Hameroff(25ms) vs Tegmark(10^-13 s) 12자릿수 불일치

**"3중" 을 "2중 × 생물물리 제약" 재공식화 제안**: IIT+GWT 짝을 유지하되 Orch-OR 제거. 그러나 IIT+GWT 도 독립성 결여로 **짝 자체 비자명성 미확보**. 재공식화 실익 **없음**.

## 5. BT-19 6-fold symmetry 일관성

원 atlas.n6 BT-19 = "**GUT Hierarchy**: ranks (τ,sopfr,n,σ-τ), dim(SU(5))=J₂, 11/11 [10*]" 로 **입자물리** 주제. 의식 3중 융합은 **BT 번호 중복** 또는 **재배정 필요**.

- 현재 atlas.n6 L10470 확인: "BT-19 — particle physics, Lie algebra, string theory"
- DSE-P7-1 문서가 BT-19 를 의식으로 재사용한 것은 **번호 충돌**
- **권고**: 의식 3중 융합은 BT-20 이후 신규 번호 할당. BT-19 는 GUT 유지

## 6. 정직 리스트 — MISS 항목

1. α_IIT = 4/3 의 Barrett 원문 수치 **없음**
2. α_GWT = 3/4 의 Dehaene 원문 수치 **없음**
3. α 곱=1 empirical 메타분석 **0편**
4. IIT/GWT 독립 latent 가정 **반증** (Casali PCI 는 IIT 파생이면서 GNW ignition 과 공변)
5. "Barrett-Tononi 2018 PRE" 논문 **미확인** (Barrett-Mediano 2019 는 존재하나 Φ 반증 내용)
6. "Dehaene Cell 2014" **미확인** (2014년 저작은 서적, Cell 논문 아님)
7. BT-19 번호는 이미 GUT 할당 — **충돌**
8. Hameroff 6-fold 주장 **근거 없음**

## 7. 승급 판정

| 조건 | 상태 |
|---|---|
| 3편+ 독립 메타분석 α 곱≈1 ±0.2 | 0/3 **FAIL** |
| 정의 일관성만 확인 (PARTIAL) | 부분 만족 — 개념적 독립 YES, 경험 독립 NO |
| IIT/GWT 동일 latent 반증 (trivial 증명) | **성립** — Casali PCI 가 IIT-GNW 공통 감소. 따라서 α 곱=1 이 참이라도 **trivial** |

**등급**: DSE-P7-1 가 제안한 **[7?] → [10*] 승급 MISS**. 오히려 기존 CONJECTURE [7?] 도 근거 부실로 **[5] 로 하향** 권고.

## 8. atlas.n6 업데이트 권고

```
# BT-19 GUT Hierarchy 유지 (변경 없음)
@R n6-atlas-breakthrough-theorems-extended:-bt-19 = GUT Hierarchy ... [10*]

# DSE-P7-1 의식 3중 융합 엔트리는 별도 번호로:
# (append — DSE-P8-2 검증 결과 [7?] → [5])
@L consciousness-alpha-product-conjecture = 1 :: consciousness [5]
  "α_IIT·α_GWT=1 주장 — 원문 근거 미확인, trivial 우려"
  => "Barrett 2011·Dehaene 2011 에 해당 α 수치 없음"
  => "Casali 2013 PCI 공변성은 IIT-GWT 독립 latent 가정 반증"
  |> MISS 2026-04-15 DSE-P8-2
```

## 9. 결론

BT-19 "의식 3중 융합" CONJECTURE (DSE-P7-1 제안) 는 **MISS**:

- 원 논문 출처의 α 수치 **부재**
- 곱 = 1 항등식은 산술적 자명성 (x·1/x = 1) — empirical 발견 아님
- IIT/GWT 가 독립 latent 라는 가정은 PCI 공변 데이터로 **반증**
- Orch-OR 6-fold 는 이미 반증, "3중" 축소 후 "2중" 도 실익 없음
- 번호 충돌 (기존 BT-19 = GUT) — 재할당 필요

**이론 재설계 권고**: "σ·φ = n·τ 의식 이론적 등가" 라는 프레임 자체를 재고. 의식은 단일 α 가 아닌 최소 2축 (level, content) 상태공간. n=6 좌표는 개별 측정 (Cowan=4=tau, Dehaene ROI=12=sigma, θ=0.5=1/phi) 수준에서만 PASS 로 유지하고, 곱셈 구조 주장은 철회.

검증자: DSE-P8-2
일자: 2026-04-15
자기참조 검사: **통과** — 외부 논문 8편 (Casali, Sarasso, Barrett-Seth, Mediano, Mashour, COGITATE, Lendner, Dehaene-Changeux) 근거. 본 프로젝트 내부 산출물 재인용 없음.
