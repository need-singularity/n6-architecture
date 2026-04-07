# 🔌 BrainWire Hardware Catalog

> 전체 하드웨어 목록 — 제품별 스펙, 가격, 구매처, 타겟 변수

## 1. 전기 자극 장비 (Electrical Stimulation)

### 1.1 tDCS (경두개 직류자극)

| 제품 | 가격 | 스펙 | 구매처 | 타겟 변수 |
|------|------|------|--------|-----------|
| **DIY tDCS** | ~$10 | 9V배터리+10kΩ+스폰지전극 | 전자부품몰 | V1,V3,V4,V7,V9,V10 |
| **AliExpress tDCS** | ~$30 | 0-2mA, CC mode, LED | AliExpress | V1,V3,V4,V7,V9,V10 |
| **TheBrainDriver v2** | ~$200 | 0-2mA, 안전회로, 타이머 | Amazon | V1,V3,V4,V7,V9,V10 |
| **foc.us v3** | ~$300 | 앱제어, 다중몽타주, BLE | foc.us | V1,V3,V4,V7,V9,V10 |
| **Soterix 1×1** | ~$2,000 | 연구등급, 정밀CC, FDA | Soterix | V1,V3,V4,V7,V9,V10 |

**몽타주 (전극 배치):**
```
  Anode F3 (left DLPFC)    → V1(DA↑), V3(5HT↑), V10(Sensory↑)
  Cathode Fz (midline)     → V7(Alpha↓)
  Cathode F4 (right DLPFC) → V9(PFC↓)
  Anode Oz (visual cortex)  → V10(Sensory↑)
```

### 1.2 tACS (경두개 교류자극)

| 제품 | 가격 | 스펙 | 구매처 | 타겟 변수 |
|------|------|------|--------|-----------|
| **DIY tACS (Arduino DAC)** | ~$15 | sine wave via DAC+amp | 자작 | V6,V8 |
| **AliExpress tACS** | ~$80 | 0.1-100Hz, 0-2mA | AliExpress | V6,V8 |
| **Neuroelectrics Starstim** | ~$5,000 | 8ch, EEG+tACS, SW | Neuroelectrics | V6,V8,V12 |

**주파수 설정:**
```
  6Hz (theta)  → V6(Theta↑↑)           전극: Fz-Pz
  10Hz (alpha) → V4(GABA↑)             전극: Oz-Fz
  40Hz (gamma) → V8(Gamma↑), V12(Coh↑) 전극: Pz-Fz (Helfrich 2014, Polanía 2012)
```

### 1.3 TMS (경두개 자기자극)

| 제품 | 가격 | 스펙 | 구매처 | 타겟 변수 |
|------|------|------|--------|-----------|
| **중고 Magstim** | ~$3,000 | figure-8 coil, rTMS | eBay/의료기기 중고 | V7,V8,V9 |
| **MagVenture MagPro** | ~$5,000 | figure-8, theta burst | MagVenture | V6,V7,V8,V9 |
| **Nexstim NBS** | ~$15,000 | navigated TMS, MRI | Nexstim | 전체 |

**프로토콜:**
```
  1Hz rTMS (inhibitory)     → V7(Alpha↓), V9(PFC↓)
  6Hz theta burst (iTBS)    → V6(Theta↑↑)
  10Hz rTMS (excitatory)    → V1(DA↑)
  40Hz gamma burst          → V8(Gamma↑)
```

### 1.4 taVNS (경피 미주신경자극)

| 제품 | 가격 | 스펙 | 구매처 | 타겟 변수 |
|------|------|------|--------|-----------|
| **TENS ear-clip 자작** | ~$0 | TENS 채널 + 귀집게 전극 | DIY | V1,V2,V3,V5 |
| **Parasym tVNS** | ~$100 | 25Hz, 0.1-0.5mA, ear-clip | Amazon | V1,V2,V3,V5 |
| **NEMOS (cerbomed)** | ~$500 | 의료등급, 처방용 | cerbomed | V1,V2,V3,V5 |
| **gammaCore** | ~$600/월 | non-invasive VNS, 휴대형 | electroCore | V1,V2,V3,V5 |

**작용 경로:**
```
  Auricular branch → NTS → VTA → DA↑ (V1)
  NTS → raphe → 5HT↑ (V3)
  NTS → LC inhibition → NE↓ (V5)
  Vagal tone↑ → parasympathetic → eCB tone↑ (V2)
```

### 1.5 TENS (경피신경전기자극)

| 제품 | 가격 | 스펙 | 구매처 | 타겟 변수 |
|------|------|------|--------|-----------|
| **시판 TENS (4ch)** | ~$25 | 2-150Hz, 0-80mA, 패드 | 약국/쿠팡 | V2,V10,V11 |
| **OMRON HV-F128** | ~$50 | 4ch, 12프로그램 | 약국 | V2,V10,V11 |
| **iReliev ET-7070** | ~$80 | TENS+EMS, BLE, 앱 | Amazon | V2,V10,V11 |

**주파수별 효과:**
```
  2-4Hz (low)     → endorphin + eCB release → V2(eCB↑), V10(Sens↑), V11(Body↑)
  50-100Hz (high)  → gate control + stochastic resonance → V10(Sens↑), V11(Body↑)
  burst mode       → 혼합 효과
```

## 2. 감각 자극 장비 (Sensory Stimulation)

### 2.1 시각 (40Hz Entrainment)

| 제품 | 가격 | 스펙 | 구매처 | 타겟 변수 |
|------|------|------|--------|-----------|
| **Arduino + LED strip** | ~$10 | PWM 40Hz, 밝기 조절 | AliExpress | V8,V10,V12 |
| **Kasina DeepVision** | ~$350 | LED 고글, 다중 프로그램 | Mindplace | V8,V10,V12 |
| **Lucia N°03** | ~$15,000 | 할로겐+LED, 연구등급 | Lucia Light | V8,V10,V12 |

**DIY 40Hz LED (Arduino 코드):**
```cpp
  // 40Hz = 25ms period, 50% duty cycle
  void loop() {
    digitalWrite(LED_PIN, HIGH);
    delayMicroseconds(12500);
    digitalWrite(LED_PIN, LOW);
    delayMicroseconds(12500);
  }
```

### 2.2 청각 (Binaural Beats + Click Train)

| 제품 | 가격 | 스펙 | 구매처 | 타겟 변수 |
|------|------|------|--------|-----------|
| **Audacity (SW)** | $0 | binaural beat 생성 | audacityteam.org | V6,V8 |
| **Brain.fm** | ~$7/월 | AI 뇌파 유도 음악 | brain.fm | V6,V8 |
| **이어폰 (아무거나)** | ~$10 | 스테레오 필수 | — | V6,V8 |
| **골전도 이어폰** | ~$30 | 뼈전도, 외부소리 가능 | AliExpress | V6,V8 |

**Audacity binaural 생성:**
```
  6Hz theta:  L=200Hz, R=206Hz → 6Hz beat
  40Hz gamma: L=400Hz, R=440Hz → 40Hz beat
  Generate > Tone > 각 채널 개별 설정
```

### 2.3 촉각 (Vibrotactile)

| 제품 | 가격 | 스펙 | 구매처 | 타겟 변수 |
|------|------|------|--------|-----------|
| **Arduino + 진동모터** | ~$5 | ERM/LRA, PWM 제어 | AliExpress | V2,V8,V11,V12 |
| **진동 마사지 매트** | ~$30 | 다중 모터, 온열 포함 | 쿠팡 | V2,V8,V11 |
| **SubPac M2X** | ~$400 | 저주파 촉각, 음악 동기 | SubPac | V8,V11,V12 |

**40Hz 진동 (Arduino):**
```cpp
  // 40Hz vibration via PWM
  analogWrite(MOTOR_PIN, 128);  // 50% duty
  // Timer interrupt at 40Hz for precise timing
```

### 2.4 온열 (Thermal Stimulation)

| 제품 | 가격 | 스펙 | 구매처 | 타겟 변수 |
|------|------|------|--------|-----------|
| **전기 온열패드** | ~$20 | 30-50°C, 타이머 | 다이소/쿠팡 | V2,V10,V11 |
| **온열 가중담요** | ~$60 | 7kg + 가열 | 쿠팡 | V2,V4,V10,V11 |
| **적외선 램프** | ~$25 | 원적외선, 국소 가열 | 약국 | V2,V10,V11 |

**TRPV1 활성화 온도:**
```
  38°C: TRPV1 활성 시작 → anandamide 방출 개시
  40°C: 최적 (안전 범위 내 최대 eCB)
  42°C: 상한 (화상 위험, 20분 미만)
```

## 3. 측정 장비 (Measurement)

### 3.1 EEG (뇌파 측정)

| 제품 | 가격 | 채널 | 구매처 | 용도 |
|------|------|------|--------|------|
| **DIY ADS1299** | ~$50 | 8ch | AliExpress | 기본 EEG |
| **Muse 2** | ~$250 | 4ch | Amazon | 간이 측정 |
| **OpenBCI Ganglion** | ~$250 | 4ch | OpenBCI | 오픈소스, 커스텀 |
| **OpenBCI Cyton** | ~$500 | 8ch | OpenBCI | 연구용 |
| **OpenBCI Cyton+Daisy** | ~$1,000 | 16ch | OpenBCI | 풀 연구 (보유) |
| **Emotiv EPOC X** | ~$850 | 14ch | Emotiv | 무선, SW포함 |

**변수별 측정 채널:**
```
  V1  DA:     F3/F4 (FAA = frontal alpha asymmetry)
  V6  Theta:  Fz/Cz (theta power)
  V7  Alpha:  F3/F4/Oz (alpha power)
  V8  Gamma:  Pz (gamma power)
  V9  PFC:    F3/F4 (theta/beta ratio)
  V12 Coh:    all pairs (PLV at gamma)
```

### 3.2 생체 신호

| 제품 | 가격 | 측정 | 구매처 | 용도 |
|------|------|------|--------|------|
| **펄스옥시미터** | ~$15 | SpO2, HR | 약국 | 안전 모니터링 |
| **Arduino MAX30102** | ~$10 | SpO2, HR, HRV | AliExpress | V3(5HT proxy), V5(NE proxy) |
| **GSR 센서** | ~$5 | 피부 전도도 | AliExpress | V5(NE→arousal) |
| **웹캠** | $0 | 동공 크기 | 보유 | V5(NE→pupil) |

## 4. Tier별 구매 목록

### Tier 1: 저가 (~$95)

```
  ┌──┬──────────────────────────────┬────────┬──────────────────────────┐
  │# │ 품목                         │ 가격   │ 구매처                   │
  ├──┼──────────────────────────────┼────────┼──────────────────────────┤
  │1 │ TENS 기기 (4ch)              │ $25    │ 약국/쿠팡                │
  │2 │ tDCS 보드 (AliExpress)       │ $30    │ AliExpress               │
  │3 │ 전기 온열패드                 │ $20    │ 다이소                   │
  │4 │ Arduino Nano + LED + 진동모터 │ $15    │ AliExpress               │
  │5 │ 모래주머니 × 4 (가중 대체)    │ $5     │ 다이소                   │
  │6 │ 이어폰 (보유)                │ $0     │ —                        │
  │7 │ Audacity + Spotify           │ $0     │ 무료                     │
  ├──┼──────────────────────────────┼────────┼──────────────────────────┤
  │  │ TOTAL                        │ $95    │ 달성률: 81% (4/12≥100%)  │
  └──┴──────────────────────────────┴────────┴──────────────────────────┘
```

### Tier 2: 중가 (~$525)

```
  ┌──┬──────────────────────────────┬────────┬──────────────────────────┐
  │# │ Tier 1 전체                  │ $95    │                          │
  ├──┼──────────────────────────────┼────────┼──────────────────────────┤
  │8 │ taVNS 전용 디바이스           │ $100   │ Amazon                   │
  │9 │ tACS 보드                    │ $80    │ AliExpress               │
  │10│ OpenBCI Ganglion (4ch EEG)   │ $250   │ OpenBCI                  │
  ├──┼──────────────────────────────┼────────┼──────────────────────────┤
  │  │ TOTAL                        │ $525   │ 달성률: 92% (7/12≥100%)  │
  └──┴──────────────────────────────┴────────┴──────────────────────────┘
```

### Tier 3: 고가 (~$7,575)

```
  ┌──┬──────────────────────────────┬────────┬──────────────────────────┐
  │# │ Tier 2 전체                  │ $525   │                          │
  ├──┼──────────────────────────────┼────────┼──────────────────────────┤
  │11│ TMS coil (중고 figure-8)     │ $3,000 │ eBay/의료기기중고        │
  │12│ 연구용 tDCS (Soterix)        │ $2,000 │ Soterix Medical          │
  │13│ OpenBCI Cyton+Daisy (16ch)   │ $1,000 │ OpenBCI (보유)           │
  │14│ tACS multi-channel           │ $500   │ Soterix/DIY              │
  │15│ 안전장비 (비상정지 등)        │ $500   │ 의료기기 서플라이        │
  │16│ 고출력 TENS + 진동모터 업그레이드│ $50 │ AliExpress               │
  │17│ Stochastic resonance 노이즈  │ $0     │ Arduino 코드             │
  ├──┼──────────────────────────────┼────────┼──────────────────────────┤
  │  │ TOTAL                        │ $7,575 │ 달성률: 108% (12/12≥100%)│
  └──┴──────────────────────────────┴────────┴──────────────────────────┘
```

## 5. 하드웨어 → 변수 매트릭스

```
  ┌──────────────┬────┬────┬─────┬─────┬────┬────┬────┬────┬────┬─────┬─────┬─────┐
  │ Hardware     │ V1 │ V2 │ V3  │ V4  │ V5 │ V6 │ V7 │ V8 │ V9 │ V10 │ V11 │ V12 │
  │              │ DA │eCB │ 5HT │GABA │ NE↓│ θ↑ │ α↓ │ γ↑ │PFC↓│Sens↑│Body↑│Coh↑ │
  ├──────────────┼────┼────┼─────┼─────┼────┼────┼────┼────┼────┼─────┼─────┼─────┤
  │ tDCS         │ ●  │    │ ●   │ ●   │    │    │ ●  │    │ ●  │ ●   │     │     │
  │ tACS         │    │    │     │ ○   │    │ ●  │    │ ●  │    │     │     │ ●   │
  │ TMS          │ ○  │    │     │     │    │ ●  │ ●  │ ●  │ ●  │ ○   │     │ ●   │
  │ taVNS        │ ●  │ ●  │ ●   │     │ ●  │    │    │    │    │     │     │     │
  │ TENS         │    │ ●  │     │     │    │    │    │    │    │ ●   │ ●   │     │
  │ LED 40Hz     │    │    │     │     │    │    │    │ ●  │    │ ●   │     │ ●   │
  │ Audio 40Hz   │    │    │     │     │    │    │    │ ●  │    │     │     │ ●   │
  │ Binaural 6Hz │    │    │     │     │    │ ●  │    │    │    │     │     │     │
  │ Vibro motor  │    │ ●  │     │     │    │    │    │ ●  │    │     │ ●   │ ●   │
  │ Heated pad   │    │ ●  │     │     │    │    │    │    │    │ ●   │ ●   │     │
  │ Weight press │    │    │     │ ●   │    │    │    │    │    │     │     │     │
  │ Noise (SR)   │    │    │     │     │    │    │    │    │    │ ●   │     │     │
  │ Music        │ ●  │    │     │     │    │    │    │    │    │     │     │     │
  ├──────────────┼────┼────┼─────┼─────┼────┼────┼────┼────┼────┼─────┼─────┼─────┤
  │ Tier 1 ($95) │ ●  │ ●  │ ●   │ ○   │ ○  │ ○  │ ○  │ ○  │ ○  │ ○   │ ●   │ ○   │
  │   81% avg    │101 │101 │106  │ 89  │ 75 │ 51 │ 60 │ 76 │ 60 │  78 │114  │ 62  │
  │ Tier 2 ($525)│ ●  │ ●  │ ●   │ ●   │ ●  │ ○  │ ○  │ ●  │ ○  │ ○   │ ●   │ ○   │
  │   92% avg    │104 │103 │114  │101  │100 │ 76 │ 60 │101 │ 60 │  93 │114  │ 81  │
  │ Tier 3 ($8K) │ ●  │ ●  │ ●   │ ●   │ ●  │ ●  │ ●  │ ●  │ ●  │ ●   │ ●   │ ●   │
  │  108% avg    │109 │110 │119  │106  │100 │102 │100 │115 │100 │ 103 │122  │104  │
  └──────────────┴────┴────┴─────┴─────┴────┴────┴────┴────┴────┴─────┴─────┴─────┘

  ● = 100%+ 달성    ○ = 부분 달성 (<100%)
  숫자 = 달성률(%) — calc.py tiers 기준
```

## 6. 안전 가이드라인

```
  tDCS:   max 2mA, max 20min, 발적 체크
  tACS:   max 2mA, max 30min, phosphene 모니터링
  TMS:    rTMS safety table 준수, 청력보호
  taVNS:  max 0.5mA, 심박 모니터링, 부정맥 금기
  TENS:   max 80mA, 전극 피부 상태 확인
  온열:   max 42°C, max 30min, 화상 주의
  40Hz:   광과민 간질 금기, 눈 감은 상태 권장
```

---

*12 variables. 12 hardware solutions. One system.*
