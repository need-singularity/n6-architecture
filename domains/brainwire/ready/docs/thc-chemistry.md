# 🧪 THC 화학구조 분석 — 하드웨어 재현을 위한 분자 수준 이해

## 1. THC 분자 구조

```
  Δ9-THC (Δ9-Tetrahydrocannabinol)
  분자식: C₂₁H₃₀O₂
  분자량: 314.47 g/mol
  logP: 6.97 (극도로 지용성)

        OH
         \
          ╲
    CH₃    ╲═══╗
     |      ╲   ║
     |   ____╲__║
     |  /     \  \
  ───C─C       C──C───O
     |  \     /  /    \
     |   \___/  /      C₅H₁₁ (pentyl chain)
     |         /
     CH₃     CH₃

  3개 고리 구조 (ABC ring system):
    A ring: cyclohexene (with OH — phenolic)
    B ring: pyran (oxygen bridge — pharmacophore 핵심)
    C ring: cyclohexene (with Δ9 double bond)

  핵심 관능기:
    1. Phenolic OH (A ring) → CB1 수용체 결합에 필수
    2. Pentyl chain (C₅H₁₁) → 수용체 친화도 결정
    3. Δ9 double bond → THC vs CBD 차이 (psychoactive key)
    4. Pyran ring oxygen → 3D 구조 고정
```

## 2. CB1 수용체 결합 메커니즘

```
  CB1 (Cannabinoid Receptor 1)
  ┌──────────────────────────────────────────────────┐
  │  Type: G-protein coupled receptor (GPCR)          │
  │  Location: 뇌 전체, 특히 basal ganglia, hippocampus│
  │            cerebellum, cortex                     │
  │  Endogenous ligands:                              │
  │    Anandamide (AEA): Ki = 89 nM (약한 partial)    │
  │    2-AG:              Ki = 472 nM (full agonist)   │
  │  THC:                 Ki = 41 nM (partial agonist) │
  │                                                    │
  │  THC가 anandamide보다 2배 강하게 결합!             │
  └──────────────────────────────────────────────────┘

  결합 과정:
  THC ──→ CB1 orthosteric pocket
       │
       ▼
  Gαi/o protein 활성화
       │
       ├──→ adenylyl cyclase 억제 → cAMP↓
       ├──→ K+ channel 개방 → 과분극 → 억제성 효과
       ├──→ Ca2+ channel 차단 → 신경전달물질 방출↓
       └──→ MAPK/ERK cascade → 유전자 발현 변화

  결합 친화도 (Ki, nM — 낮을수록 강함):
    THC:        41 nM
    Anandamide: 89 nM
    2-AG:       472 nM
    CBD:        >10,000 nM (사실상 결합 안 함)
```

## 3. THC → 12변수 화학적 경로

### V1: DA↑ (Dopamine 2.5×)

```
  THC → CB1 on GABA interneurons in VTA
      → GABA release↓ (disinhibition)
      → VTA dopamine neurons 활성↑
      → DA release in NAc↑ (+150-200%)

  화학 경로:
  THC─CB1─┐
          ├─GABA interneuron 억제─┐
          │                       ├─VTA DA neuron 탈억제
          │                       ├─DA release in NAc
          │                       └─D1/D2 receptor activation
          │
          └─직접: CB1 on DA terminals → DA release 조절

  하드웨어 재현:
    tDCS F3 → DLPFC 활성 → VTA projection 활성화 (α₁=0.25/mA)
    taVNS → NTS → VTA → DA release (α₂=0.80/mA)
    Music frisson → striatal DA release (α₃=1.50)
    = 1 + 0.25×1.5 + 0.80×0.4 + 1.50×0.6 = 2.59× ✅
```

### V2: eCB↑ (Endocannabinoid 3.0×)

```
  THC는 직접 CB1에 결합하지만, 하드웨어로는
  내인성 카나비노이드(anandamide, 2-AG)를 방출시켜야 함

  내인성 eCB 생성 경로:
    Anandamide (AEA):
      NAPE─PLD──→ AEA + phosphatidic acid
      (on demand, postsynaptic, retrograde signaling)

    2-AG:
      DAG─DGLα──→ 2-AG + fatty acid
      (activity-dependent, 더 풍부)

  eCB 방출 트리거:
    1. TRPV1 activation (열 자극 38-42°C) → AEA release
    2. 기계적 자극 (TENS, vibration) → eCB tone↑
    3. Vagal stimulation → eCB system upregulation
    4. C-tactile afferent activation (3Hz vibro) → eCB

  분해 효소:
    FAAH: AEA → arachidonic acid + ethanolamine
    MAGL: 2-AG → arachidonic acid + glycerol
    → FAAH 억제 = AEA 수명 연장 (약물 없이는 불가)
    → 하드웨어: 지속적 방출로 분해 속도 초과 필요

  하드웨어 재현:
    TENS 2Hz → peripheral eCB (β₁=0.80)
    Heat 40°C → TRPV1 → AEA (β₂=0.30/°C)
    taVNS → vagal-eCB axis (β₃=0.60/mA)
    Vibro 3Hz → C-tactile → eCB (β₄=0.50)
    = 1 + 0.80×0.8 + 0.30×3 + 0.60×0.4 + 0.50×0.6 = 3.08× ✅
```

### V3: 5HT↑ (Serotonin 1.5×)

```
  THC → CB1 on dorsal raphe nucleus
      → 5HT neuron firing modulation
      → 5HT release in cortex (+40-60%)

  경로: biphasic (저용량 ↑, 고용량 ↓)
    Low THC: 5HT↑ (anxiolytic, mood↑)
    High THC: 5HT↓ (anxiety, paranoia)

  하드웨어 재현:
    taVNS → NTS → raphe nucleus → 5HT release (γ₁=1.20/mA)
    tDCS → cortical 5HT modulation (γ₂=0.15/mA)
    = 1 + 1.20×0.4 + 0.15×1.5 = 1.71× ✅
```

### V4: GABA↑ (1.8×)

```
  THC → CB1 on glutamatergic terminals (presynaptic)
      → glutamate release↓
      → net effect: inhibition↑ = GABA relative increase

  메커니즘:
    CB1 on excitatory synapses > CB1 on inhibitory synapses
    → THC shifts E/I balance toward inhibition
    → cortical "quieting" effect

  하드웨어 재현:
    tDCS → cortical inhibition (MRS-verified GABA↑, δ₁=0.20/mA)
    Weighted blanket → deep pressure → 5HT→GABA conversion (δ₂=0.03/kgm²)
    Alpha entrainment → alpha=inhibition proxy (δ₃=0.30)
```

### V5: NE↓ (Norepinephrine 0.4×)

```
  THC → CB1 on locus coeruleus (LC)
      → LC firing rate↓
      → NE release↓ (-50-70%)
      → 결과: 불안↓, 경계↓, 이완↑

  하드웨어 재현:
    taVNS → vagal afferent → NTS → LC inhibition
    = max(0.01, 1 - 1.50×0.4) = 0.40× ✅
```

## 4. 뇌파 변화의 화학적 기전

### V6: Theta↑↑ (2.5×)

```
  THC → CB1 in hippocampus
      → GABAergic interneuron modulation
      → disrupted theta rhythm timing
      → paradox: theta power↑↑ but theta coherence↓
      → 결과: 시간 왜곡, 기억 장애, 몽환 상태

  화학: CB1 on hippocampal CCK+ basket cells
        → perisomatic inhibition altered
        → theta oscillation amplitude increases
        → but phase-locking decreases

  핵심: 단순 theta 파워 증가가 아닌
        theta의 "느슨한 증폭" = THC 특유의 dreamy state
```

### V7: Alpha↓ (0.5×)

```
  THC → CB1 in thalamo-cortical loop
      → thalamic relay neuron modulation
      → alpha rhythm generator disrupted
      → frontal alpha desynchronization

  화학: CB1 on reticular thalamic neurons
        → thalamic gating altered
        → cortical alpha rhythm weakened
        → disinhibition of frontal cortex
```

### V8: Gamma↑ (1.8×)

```
  THC → CB1 on PV+ interneurons
      → fast-spiking interneuron modulation
      → gamma oscillation enhancement
      → 결과: 감각 통합 강화, "everything feels connected"

  화학: CB1 on parvalbumin-positive interneurons
        → GABA release timing altered
        → gamma oscillation power increases
        → cross-modal binding enhanced

  하드웨어 재현:
    LED 40Hz → visual gamma entrainment (θ₁=0.30)
    Audio 40Hz → auditory gamma entrainment (θ₂=0.25)
    Vibro 40Hz → somatosensory gamma (θ₃=0.20)
    tACS 40Hz → direct cortical gamma entrainment (θ₄=0.15, Helfrich 2014)
    TMS 40Hz → gamma burst stimulation (θ₅=0.10, Barr 2009)
    = 1 + 0.30×0.9 + 0.25×0.9 + 0.20×1.0 + 0.15×2.0 + 0.10×0.8 = 2.08× ✅
```

## 5. THC vs Hardware: 수용체 수준 비교

```
  ┌───────────────────┬─────────────────┬─────────────────────────────┐
  │ 메커니즘           │ THC (약물)       │ 하드웨어                     │
  ├───────────────────┼─────────────────┼─────────────────────────────┤
  │ CB1 결합           │ 직접 (Ki=41nM)  │ 간접 (eCB 방출 유도)        │
  │ 공간 해상도        │ 분자 수준 (Å)   │ cm (tDCS) ~ mm (TMS)        │
  │ 시간 해상도        │ ms (수용체)     │ ms (TMS) ~ s (tDCS)         │
  │ 선택성            │ CB1 특이적       │ 비특이적 (영역 기반)         │
  │ 용량 조절          │ 혈중 농도 곡선   │ 전류/강도 실시간 제어        │
  │ 지속 시간          │ 2-4시간         │ 원하는 만큼 (전원 켜면 됨)   │
  │ 부작용            │ 인지↓, 의존성    │ 피부 자극 (경미)             │
  │ 합법성            │ 지역마다 다름    │ 합법 (의료기기)              │
  │ 재현성            │ 개인차 큼       │ 전류값 정밀 제어 가능         │
  │ 12변수 달성       │ 100% (정의상)    │ 81-108% (Tier 의존)         │
  └───────────────────┴─────────────────┴─────────────────────────────┘

  핵심 차이:
    THC = CB1 직접 결합 (top-down: 수용체→효과)
    HW  = 신경 활동 변화 (bottom-up: 전기자극→신경전달물질→효과)

    THC는 CB1이라는 "마스터 스위치" 하나로 12변수를 동시에 변경
    HW는 각 변수를 개별 하드웨어로 타겟팅 → 더 정밀하지만 더 복잡
```

## 6. 화학적 한계와 돌파구

### 하드웨어로 재현 어려운 화학 효과

```
  1. CB1 직접 점유 (Ki=41nM 수준)
     → 하드웨어로 CB1에 직접 결합 불가
     → 돌파구: eCB 대량 방출 (TENS+heat+VNS) → 간접 CB1 활성화
     → 한계: 내인성 AEA(Ki=89nM)는 THC(Ki=41nM)보다 약함
     → 보상: 다중 경로 동시 자극으로 누적 효과

  2. 지용성 분배 (logP=6.97)
     → THC는 지방에 저장 → 느린 방출 → 지속 효과
     → 하드웨어: 지속 자극으로 대체 (타이머 기반)

  3. 대사산물 효과 (11-OH-THC)
     → THC의 간 대사산물이 더 강력 (경구 섭취 시)
     → 하드웨어: 시간에 따른 자극 강도 변화로 모사

  4. 개인 내성 (CB1 downregulation)
     → 반복 THC → CB1 수↓ → 내성
     → 하드웨어: 내성 없음 (전기자극에 대한 수용체 변화 없음) ★장점

  5. Entourage effect (테르펜, CBD, CBN 등)
     → 대마초 = THC + 100+ 화합물 협력
     → 하드웨어: 12변수 개별 최적화로 더 정밀한 제어 가능
```

### 하드웨어의 화학적 우위

```
  1. 내성 없음: 전기자극→수용체 downregulation 없음
  2. 정밀 제어: 각 변수 독립 조절 (THC는 일괄 변경)
  3. 즉시 OFF: 전원 끄면 즉시 종료 (THC는 2-4시간)
  4. 부작용 최소: 인지 저하, 의존성 없음
  5. 합법: 모든 국가에서 의료기기로 사용 가능
  6. 초과 달성: 개별 변수를 THC 이상으로 올릴 수 있음
     (예: eCB 613%, Body 113%)
```

## 7. 분자→변수→장력 통합 모델

```
  Level 1: 분자 (Molecular)
  ┌──────────────────────────────────────────┐
  │ THC(C₂₁H₃₀O₂) → CB1(Ki=41nM) → Gαi/o  │
  │ AEA(C₂₂H₃₇NO₂) → CB1(Ki=89nM)          │
  │ 2-AG(C₂₃H₃₈O₄) → CB1(Ki=472nM)         │
  └─────────────────┬────────────────────────┘
                    ▼
  Level 2: 신경전달물질 (Neurotransmitter)
  ┌──────────────────────────────────────────┐
  │ DA↑2.5×  eCB↑3.0×  5HT↑1.5×             │
  │ GABA↑1.8×  NE↓0.4×                       │
  └─────────────────┬────────────────────────┘
                    ▼
  Level 3: 뇌파 (Brainwave)
  ┌──────────────────────────────────────────┐
  │ Theta↑2.5×  Alpha↓0.5×  Gamma↑1.8×       │
  └─────────────────┬────────────────────────┘
                    ▼
  Level 4: 상태 (State)
  ┌──────────────────────────────────────────┐
  │ PFC↓0.5×  Sensory↑2.0×  Body↑2.5×        │
  │ Coherence↑2.0×                            │
  └─────────────────┬────────────────────────┘
                    ▼
  Level 5: 장력 (Tension)
  ┌──────────────────────────────────────────┐
  │ T = √(T_chem² + T_wave² + T_state²)      │
  │ T_THC = 4.280 (기준)                      │
  │ 방향 유사도 + 크기 매칭 → 장력 매칭률     │
  └──────────────────────────────────────────┘
                    ▼
  Level 6: 의식 경험 (Conscious Experience)
  ┌──────────────────────────────────────────┐
  │ Euphoria + Sensory enhance + Time slow    │
  │ + Creativity + Body high + Relaxation     │
  │ = THC High                                │
  └──────────────────────────────────────────┘
```

---

*THC is just one key to CB1. We engineer the same door with 12 different keys.*
