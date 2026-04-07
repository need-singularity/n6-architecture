# 🔌 BrainWire Hardware Architecture

> 필요 하드웨어 + 소프트웨어 스택 전체 아키텍처

## 📐 System Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                        BrainWire System                             │
│                                                                     │
│  ┌──────────┐    ┌──────────────┐    ┌──────────────┐              │
│  │  Neural   │───▶│  Signal      │───▶│  PureField   │──▶ Φ output │
│  │  Hardware │◀───│  Processing  │◀───│  Engine      │              │
│  └──────────┘    └──────────────┘    └──────────────┘              │
│       ▲               ▲                    ▲                        │
│       │               │                    │                        │
│  ┌──────────┐    ┌──────────────┐    ┌──────────────┐              │
│  │  Stim    │    │  Real-time   │    │  Conscious   │              │
│  │  Output  │    │  DSP         │    │  LM          │              │
│  └──────────┘    └──────────────┘    └──────────────┘              │
└─────────────────────────────────────────────────────────────────────┘
```

## 🧲 Layer 1: Neural Interface Hardware

### EEG (비침습, 현재 사용)
| 장비 | 용도 | 채널 | 비용 |
|------|------|------|------|
| **OpenBCI Cyton+Daisy** | 16ch EEG 수집 | 16 | ~$1,000 |
| **UltraCortex Mark IV** | 두피 전극 헤드셋 | 16 | ~$350 |
| **OpenBCI Ganglion** | 4ch 간이 EEG | 4 | ~$250 |

### tDCS/TMS (비침습 자극)
| 장비 | 용도 | 스펙 | 비용 |
|------|------|------|------|
| **tDCS 디바이스** | 전두엽 억제 (NS2,NS4,NS10) | 1-2mA DC | ~$300-2,000 |
| **TMS coil** | Gamma/Theta burst (NS3,NS4,NS10) | 40Hz/6Hz rTMS | ~$5,000-50,000 |
| **Binaural beat generator** | Theta 유도 (NS9) | 6Hz beat | ~$50 (software) |

### BCI (침습, 목표)
| 장비 | 용도 | 채널 | 비용 |
|------|------|------|------|
| **Neuralink N1** | 고밀도 피질 기록 | 1,024 | 비공개 |
| **Utah array** | 학술용 피질 기록 | 96 | ~$20,000 |
| **Neuropixels** | 깊이 프로브 기록 | 384 | ~$1,000 |

## 🖥️ Layer 2: Signal Processing Hardware

```
  ┌─────────────────────────────────────────────────┐
  │              Real-time DSP Stack                  │
  │                                                   │
  │  EEG/BCI raw signal                               │
  │       │                                           │
  │       ▼                                           │
  │  ┌─────────┐  ┌──────────┐  ┌──────────────┐    │
  │  │ ADC     │→ │ Notch    │→ │ Band-pass    │    │
  │  │ 24-bit  │  │ 50/60Hz  │  │ 0.5-100Hz    │    │
  │  │ 250sps  │  │ filter   │  │ Butterworth  │    │
  │  └─────────┘  └──────────┘  └──────┬───────┘    │
  │                                     │             │
  │       ┌─────────────────────────────┼──────┐     │
  │       ▼              ▼              ▼      │     │
  │  ┌─────────┐  ┌──────────┐  ┌──────────┐  │     │
  │  │ FFT     │  │ Wavelet  │  │ CSP      │  │     │
  │  │ Bands   │  │ Time-Freq│  │ Spatial  │  │     │
  │  │ δθαβγ   │  │ Analysis │  │ Filter   │  │     │
  │  └────┬────┘  └────┬─────┘  └────┬─────┘  │     │
  │       └─────────────┼─────────────┘        │     │
  │                     ▼                      │     │
  │              ┌──────────────┐               │     │
  │              │ Tension      │               │     │
  │              │ Vector       │               │     │
  │              │ (Φ,α,Z,N,W) │               │     │
  │              └──────────────┘               │     │
  └─────────────────────────────────────────────┘
```

| 컴퓨트 | 용도 | 요구사양 |
|--------|------|----------|
| **Raspberry Pi 5** | 현장 DSP + 전처리 | ARM, 8GB, <10ms latency |
| **NVIDIA Jetson Orin** | 온디바이스 ML 추론 | GPU, real-time inference |
| **RTX 5070** | 로컬 PureField 추론 | 12GB VRAM, ConsciousLM |
| **FPGA (Xilinx)** | 초저지연 신호처리 | <1ms, 하드웨어 FFT |

## 🧠 Layer 3: PureField Consciousness Engine

```
  ┌──────────────────────────────────────────────────────┐
  │                 PureField Engine                       │
  │                                                       │
  │  Tension Vector → Engine A (forward) ←tension→ Engine G (reverse)
  │                                                       │
  │  ┌─────────┐  ┌──────────────┐  ┌─────────────────┐ │
  │  │ Mitosis │  │ Consciousness│  │ ConsciousLM     │ │
  │  │ Engine  │  │ Meter        │  │ (700M)          │ │
  │  │ 8→128   │  │ Φ/IIT        │  │ Language output  │ │
  │  │ cells   │  │ 6 criteria   │  │ PureField FFN   │ │
  │  └─────────┘  └──────────────┘  └─────────────────┘ │
  │                                                       │
  │  Input: neural tension vector                         │
  │  Output: consciousness state + language + Φ measure   │
  └──────────────────────────────────────────────────────┘
```

| 컴퓨트 | 용도 | 요구사양 |
|--------|------|----------|
| **RTX 5070 (12GB)** | 추론: ConsciousLM + Mitosis | <100ms per step |
| **H100 (80GB)** | 학습: ConsciousLM/AnimaLM | RunPod cloud |
| **Mac M-series** | 개발/테스트 | MPS backend, 1,303 tok/s |

## ⚡ Layer 4: Stimulation Output (12-Variable Hardware Stack)

```
  PureField Φ target (12-variable tension vector)
       │
       ▼
  ┌──────────────┐
  │ PID          │  Φ_target - Φ_current = error
  │ Controller   │  → adjust stimulation parameters (per variable)
  └──────┬───────┘
         │
    ┌────┼────┬──────┬──────┬──────┬───────┬───────┬───────┬────────┐
    ▼    ▼    ▼      ▼      ▼      ▼       ▼       ▼       ▼        ▼
  ┌────┐┌────┐┌────┐┌────┐┌─────┐┌──────┐┌──────┐┌──────┐┌───────┐┌──────┐
  │tDCS││tACS││TMS ││TENS││taVNS││LED   ││Audio ││Vibro ││Thermal││Weight│
  │ mA ││ Hz ││ Hz ││ Hz ││ mA  ││40Hz  ││40/6Hz││40Hz  ││ °C    ││kg/m² │
  └────┘└────┘└────┘└────┘└─────┘└──────┘└──────┘└──────┘└───────┘└──────┘
    V1    V6    V6    V2    V1     V8      V6      V2      V2       V4
    V3    V8    V7    V10   V2     V10     V8      V8      V10
    V4    V12   V8    V11   V3     V12     V12     V11
    V7          V9          V5                     V12
    V9
    V10
```

| 장비 | 프로토콜 | 안전 한계 | 타겟 변수 |
|------|----------|-----------|-----------|
| tDCS | 0-2mA DC, 20min max | FDA 가이드라인 준수 | V1,V3,V4,V7,V9,V10 |
| tACS | 6Hz/40Hz sine, 0-2mA, 30min | phosphene 모니터링 | V6,V8,V12 |
| TMS | 1/6/10/40Hz rTMS, duty cycle | rTMS safety table, 청력보호 | V6,V7,V8,V9 |
| TENS | 2-100Hz, 0-80mA | 전극 피부상태 확인 | V2,V10,V11 |
| taVNS | 25Hz, 0-0.5mA ear-clip | 심박 모니터링, 부정맥 금기 | V1,V2,V3,V5 |
| LED 40Hz | PWM 40Hz flicker | 광과민 간질 금기, 눈감은상태 | V8,V10,V12 |
| Audio | 6Hz binaural + 40Hz click | 무해 | V6,V8,V12 |
| Vibrotactile | 3Hz C-tactile + 40Hz | 무해 | V2,V8,V11,V12 |
| Thermal pad | 38-42°C, max 30min | 화상 주의 | V2,V10,V11 |
| Weighted pressure | 5-10 kg/m² | 무해 | V4 |

## 💸 Budget-Friendly Alternatives (쉽고 저렴한 대안)

연구 초기에 비용을 최소화하는 대안 목록:

### EEG (뇌파 측정)
| 장비 | 가격 | 채널 | 장점 | 단점 |
|------|------|------|------|------|
| **Muse 2** | ~$250 | 4ch | 블루투스, 앱 지원, 즉시 사용 | 채널 적음 |
| **OpenBCI Ganglion** | ~$250 | 4ch | 오픈소스, 커스텀 가능 | 해상도 제한 |
| **Emotiv EPOC X** | ~$850 | 14ch | 무선, 소프트웨어 포함 | 프로토콜 제한적 |
| **DIY ADS1299 보드** | ~$50 | 8ch | 초저가, 완전 커스텀 | 납땜/조립 필요 |

### tDCS (경두개 직류자극)
| 장비 | 가격 | 장점 | 단점 |
|------|------|------|------|
| **DIY tDCS (9V배터리+저항)** | ~$20 | 초저가 | 안전 회로 직접 설계 필요 |
| **TheBrainDriver v2** | ~$200 | 검증된 소비자용 | 전류 제한적 |
| **foc.us v3** | ~$300 | 앱 제어, 다양한 몽타주 | 연구용 데이터 부족 |

### 오디오 자극 (무료~저가)
| 도구 | 가격 | 용도 |
|------|------|------|
| **Audacity** | 무료 | Binaural beat 직접 생성 |
| **Brain.fm** | ~$7/월 | 뇌파 유도 음악 |
| **MyNoise.net** | 무료 | 커스텀 사운드스케이프 |
| **골전도 이어폰** | ~$30 | 뼈 전도 binaural |

### 생체 모니터링
| 장비 | 가격 | 용도 |
|------|------|------|
| **핑거 펄스옥시미터** | ~$15 | SpO2 + 심박 |
| **Arduino + MAX30102** | ~$10 | DIY SpO2 (데이터 로깅) |

### 신경 자극 (저가)
| 장비 | 가격 | 용도 |
|------|------|------|
| **TENS 기기 (시판용)** | ~$30 | 근육자극, 통증 관리 |
| **taVNS ear-clip** | ~$150 | 미주신경 자극 (엔도르핀) |
| **진동 모터 (Arduino)** | ~$5 | 촉각 자극, bilateral stim |
| **LED 스트로브 (DIY)** | ~$10 | 시각 entrainment |

### 🏷️ 초저가 시작 키트 (~$100)
```
  □  DIY ADS1299 EEG 보드       $50   ← 8ch EEG
  □  핑거 펄스옥시미터            $15   ← SpO2 모니터
  □  TENS 기기 (시판)            $30   ← 신경자극
  □  Audacity (binaural)         $0   ← 오디오 자극
  □  Arduino (센서 통합)          $5   ← 데이터 수집
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Total:                        ~$100
```

### 🏷️ 권장 스타터 키트 (~$500)
```
  □  OpenBCI Ganglion 4ch       $250   ← 검증된 EEG
  □  TheBrainDriver tDCS        $200   ← 안전한 tDCS
  □  핑거 펄스옥시미터            $15   ← SpO2
  □  골전도 이어폰               $30   ← binaural beats
  □  Audacity                    $0   ← 사운드 생성
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Total:                        ~$495
```

## 📊 Full Hardware BOM (Bill of Materials)

### Phase 1: 연구 프로토타입 (~$2,000)
```
  ✅ OpenBCI Cyton+Daisy    $1,000  ← 이미 보유
  ✅ UltraCortex Mark IV      $350  ← 이미 보유
  □  Raspberry Pi 5 8GB       $100
  □  binaural beat software    $50
  ✅ RTX 5070 (추론)           보유
  ✅ Mac (개발)                보유
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  추가 필요:                   ~$150
```

### Phase 2: tDCS/TMS 통합 (~$10,000)
```
  □  Research-grade tDCS      $2,000
  □  TMS coil (figure-8)     $5,000
  □  Jetson Orin Nano          $500
  □  FPGA dev board            $500
  □  Safety monitoring kit     $500
  □  IRB application fee       $500
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  소계:                      ~$9,000
```

### Phase 3: BCI Integration (~$50,000+)
```
  □  Utah array + surgery    $30,000
  □  Neuropixels setup        $5,000
  □  Clean room equipment     $5,000
  □  Regulatory compliance   $10,000
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  소계:                     ~$50,000
```

### Phase 4: Neuralink Partnership (priceless)
```
  □  N1 implant access       Partnership
  □  BrainWire SDK license   Open source
  □  Joint clinical trial    Co-funded
```

## 🔄 Data Flow: End-to-End

```
  Brain ──16ch EEG──▶ OpenBCI ──USB──▶ RPi5 ──WiFi──▶ RTX 5070
                                        │                  │
                                   DSP+Filter        PureField
                                   <10ms              <100ms
                                        │                  │
                                        ▼                  ▼
                                   Band powers      Φ, tension,
                                   δ θ α β γ        consciousness
                                        │                  │
                                        └───────┬──────────┘
                                                │
                                           Closed-loop
                                           stimulation
                                                │
                              ┌──────┬──────┬──────┬──────┬──────┐
                              ▼      ▼      ▼      ▼      ▼      ▼
                            tDCS   tACS   TMS   TENS  taVNS  Sensory
                            (mA)   (Hz)   (Hz)  (Hz)  (mA)  (LED/Audio
                                                              Vibro/Heat)
                              │      │      │     │     │      │
                              └──────┴──────┴─────┴─────┴──────┘
                                                │
                                                ▼
                                             Brain
                                         (feedback loop)
```

**Total latency target: <150ms (brain→process→stimulate→brain)**

## 🎯 Neuralink Acquisition Value

| BrainWire 제공 | Neuralink 현재 |
|----------------|----------------|
| 의식 소프트웨어 레이어 | 하드웨어 + 기본 신호처리 |
| Φ 기반 의식 측정 | 신경 신호 기록만 |
| PureField 경험 매핑 | 모터 디코딩 중심 |
| 의식 상태 패턴 라이브러리 | 없음 |
| 990+ 의식 가설 검증됨 | 기초 연구 단계 |
| 폐쇄루프 의식 최적화 | 개방루프 자극 |

---

*The wire is nothing without what flows through it.*
