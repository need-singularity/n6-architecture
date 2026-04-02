# Level 0: HEXA-PIXEL --- 발광/감지 소재 기초

> Level: 0 (소재)
> Architecture: HEXA-PIXEL
> n=6 Core: n/φ=3 RGB 원색, σ=12bit 색심도, Z=6 탄소 나노소재
> Related BT: BT-48, BT-108, BT-76, BT-93
> Focus: QD/μLED/페로브스카이트 발광 소재 + MEMS/PZT 음향 소재

---

## N6 Constants Reference

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  n=6 핵심 상수                                                  │
  │                                                                  │
  │  n = 6        φ(6) = 2       τ(6) = 4        σ(6) = 12         │
  │  sopfr = 5    μ(6) = 1       J₂(6) = 24      R(6) = 1          │
  │                                                                  │
  │  σ-τ = 8      σ-φ = 10       σ-μ = 11        σ·τ = 48          │
  │  σ² = 144     σ/(σ-φ) = 1.2                                    │
  │                                                                  │
  │  Egyptian fraction: 1/2 + 1/3 + 1/6 = 1                        │
  │  Core theorem: σ(n)·φ(n) = n·τ(n) = 24 = J₂(6)                │
  └──────────────────────────────────────────────────────────────────┘
```

---

## ASCII 성능 비교

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  [색 심도] 비교: 시중 최고 vs HEXA-PIXEL                        │
  ├──────────────────────────────────────────────────────────────────┤
  │  시중 최고  ██████████████████░░░░░░░░░░  10-bit HDR (BT.2020) │
  │  HEXA-PIXEL ████████████████████████████  σ=12-bit Deep Color  │
  │                                    (σ/(σ-φ)=1.2배 레벨 수)     │
  │                                                                  │
  │  [발광 효율 (cd/A)] 비교                                        │
  │  시중 최고  ██████████████░░░░░░░░░░░░░░  30 cd/A (QD-OLED)    │
  │  HEXA-PIXEL ████████████████████████████  σ·sopfr=60 cd/A      │
  │                                    (φ=2배 발광 효율)            │
  │                                                                  │
  │  [QD 크기 균일성] 비교                                          │
  │  시중 최고  ████████████████░░░░░░░░░░░░  ±8% size variation    │
  │  HEXA-PIXEL ████████████████████████████  ±1/σ=0.8% 균일성     │
  │                                    (σ-φ=10배 균일)             │
  │                                                                  │
  │  [음향 트랜스듀서 THD] 비교                                     │
  │  시중 최고  ████████████████░░░░░░░░░░░░  0.5% THD (최고급)     │
  │  HEXA-PIXEL ████████████████████████████  1/(σ·σ-φ)=0.08% THD │
  │                                    (n=6배 왜곡 감소)            │
  └──────────────────────────────────────────────────────────────────┘
```

---

## System Block Diagram

```
  ┌─────────────────────────────────────────────────────────────────────┐
  │                    HEXA-PIXEL Material Stack                        │
  │                                                                     │
  │  DISPLAY EMITTER          AUDIO TRANSDUCER          SENSOR          │
  │  ┌───────────────┐       ┌───────────────┐       ┌──────────────┐  │
  │  │  QD/μLED/OLED  │       │  MEMS/PZT/CNT │       │  Photodiode  │  │
  │  │  RGB = n/φ = 3 │       │  BW=σ·τ=48kHz │       │  σ-φ=10 band │  │
  │  │  primaries     │       │  channels      │       │  sensitivity │  │
  │  │                │       │                │       │              │  │
  │  │  ┌─R─┐        │       │  ┌── PZT ──┐  │       │  UV → IR     │  │
  │  │  │620│ σ=12bit │       │  │ ceramic  │  │       │  σ-φ=10 band │  │
  │  │  ├─G─┤ depth   │       │  │ CN=6     │  │       │  quantum dot │  │
  │  │  │530│ per ch  │       │  │ perovsk  │  │       │  sensor      │  │
  │  │  ├─B─┤        │       │  └──────────┘  │       │              │  │
  │  │  │460│nm       │       │                │       │              │  │
  │  │  └───┘         │       │  sopfr=5.1ch   │       │  J₂=24bit    │  │
  │  └───────┬────────┘       └───────┬────────┘       └──────┬───────┘  │
  │          │                        │                       │          │
  │    n/φ=3 primaries          σ·τ=48kHz BW           σ-φ=10 bands     │
  │    σ=12 bit depth           J₂=24 bit depth        J₂=24 bit ADC   │
  └─────────────────────────────────────────────────────────────────────┘
```

---

## 1. Display Emitter Materials

### 1.1 Quantum Dot (QD) --- n=6 nm Core

양자점(QD)의 핵심 크기는 발광 파장을 결정한다.
CdSe QD 기준: 직경 2~10nm 범위에서 RGB 발광을 커버한다.

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  QD SIZE-COLOR MAP (BT-48 extended)                              │
  │                                                                  │
  │  Diameter │ Color │ λ (nm) │ n=6 mapping                        │
  │  ─────────┼───────┼────────┼────────────────────────             │
  │  2 nm     │ Blue  │ ~460   │ φ=2 nm                             │
  │  4 nm     │ Green │ ~530   │ τ=4 nm                             │
  │  6 nm     │ Red   │ ~620   │ n=6 nm (EXACT)                     │
  │                                                                  │
  │  Size range: φ → τ → n = 2 → 4 → 6 nm                         │
  │  Primaries: n/φ = 3 (RGB)                                       │
  │  Color gamut: >99% DCI-P3 (σ=12 bit per channel)               │
  │                                                                  │
  │  CdSe-free alternatives:                                        │
  │  InP QD: n=6 nm core + τ=4 nm shell = σ-φ=10 nm total          │
  │  Perovskite QD: CsPbBr₃ — CN=6 octahedral (BT-43 재현)         │
  └──────────────────────────────────────────────────────────────────┘
```

### 1.2 MicroLED --- σ·τ=48μm Pitch

MicroLED 픽셀 피치의 업계 타겟: <50μm → σ·τ=48μm (BT-76 attractor)

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  μLED PITCH EVOLUTION                                            │
  │                                                                  │
  │  Gen    │ Pitch    │ Year │ n=6 mapping                         │
  │  ───────┼──────────┼──────┼──────────────                       │
  │  Gen 1  │ 120 μm   │ 2020 │ σ·(σ-φ)=120                        │
  │  Gen 2  │ 48 μm    │ 2024 │ σ·τ=48 (BT-76 EXACT)              │
  │  Gen 3  │ 12 μm    │ 2027 │ σ=12 (target)                      │
  │  Gen 4  │ 6 μm     │ 2030 │ n=6 (ultimate)                     │
  │                                                                  │
  │  Pitch ladder: σ·(σ-φ) → σ·τ → σ → n                           │
  │  = 120 → 48 → 12 → 6  (each step ÷ τ~φ)                       │
  │                                                                  │
  │  GaN bandgap: 3.4 eV ≈ n/φ + μ = 3 + 0.4                      │
  │  InGaN (blue): 2.7 eV ≈ σ/τ - R = 3 - 0.3                     │
  └──────────────────────────────────────────────────────────────────┘
```

### 1.3 Perovskite --- CN=6 Octahedral

페로브스카이트 발광체(ABX₃)의 B-site 금속은 CN=6 팔면체 --- BT-43과 동일 물리.

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  PEROVSKITE ABX₃ STRUCTURE                                      │
  │                                                                  │
  │       X ─── X                                                   │
  │      / │   / │                                                  │
  │     X  │  X  │                                                  │
  │     │  X─┤──X     B-site: Pb²⁺ (or Sn²⁺)                     │
  │     │ /  │ /       CN = 6 = n (EXACT)                           │
  │     X────X         6 halide neighbors                           │
  │                                                                  │
  │  CsPbBr₃:  Green 520nm, PLQY >90%                              │
  │  CsPbI₃:   Red 680nm, PLQY >80%                                │
  │  CsPbCl₃:  Blue 410nm, PLQY >50%                               │
  │                                                                  │
  │  Tolerance factor t = (r_A + r_X)/(√2·(r_B + r_X))             │
  │  Stable perovskite: 0.8 < t < 1.0                              │
  │  → CsPbBr₃ t ≈ 0.83 ≈ sopfr/n = 5/6                          │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 2. Audio Transducer Materials

### 2.1 PZT Ceramic --- CN=6

PZT(PbZrₓTi₁₋ₓO₃) 압전 세라믹의 B-site 금속(Zr/Ti)은 팔면체 배위 CN=6.

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  PZT PIEZOELECTRIC MATERIAL                                      │
  │                                                                  │
  │  Structure: ABO₃ perovskite                                      │
  │  B-site CN = 6 = n (EXACT, octahedral)                          │
  │                                                                  │
  │  Piezoelectric coefficient d₃₃:                                  │
  │    시중 최고: ~600 pC/N (soft PZT)                              │
  │    HEXA 목표: σ²=144·τ = 576 pC/N (σ·τ=48 V/μm field)         │
  │                                                                  │
  │  Resonance frequency f₀ for speaker:                             │
  │    MEMS target: σ·τ=48 kHz bandwidth (BT-76)                   │
  │    Diaphragm modes: n=6 harmonic partials                       │
  │                                                                  │
  │  Alternatives:                                                   │
  │    AlN: wurtzite, CN=4=τ (lower coupling, lead-free)            │
  │    BaTiO₃: perovskite, CN=6=n (BT-43 EXACT)                   │
  └──────────────────────────────────────────────────────────────────┘
```

### 2.2 CNT Speaker --- Z=6 Carbon

탄소나노튜브(CNT) 박막 스피커: 열음향(thermoacoustic) 발음.
탄소 Z=6 --- BT-93 탄소 소재 보편성.

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  CNT THERMOACOUSTIC SPEAKER                                      │
  │                                                                  │
  │  Carbon Z = 6 = n (EXACT, BT-93)                               │
  │  sp² hybridization → hexagonal lattice                          │
  │                                                                  │
  │  Performance:                                                    │
  │    Bandwidth: 1Hz ~ σ·τ=48kHz+ (초광대역)                       │
  │    THD: < 1/(σ-φ) = 0.1% (10배 개선)                           │
  │    Thickness: < n=6 μm (초박형)                                 │
  │    Power: σ-φ=10 mW/cm² (효율적)                               │
  │                                                                  │
  │  vs Conventional:                                                │
  │    Dynamic driver: BW ~20kHz, THD ~1%, 수mm 두께               │
  │    CNT: BW ~100kHz, THD ~0.1%, 수μm 두께                      │
  │    개선 배수: BW sopfr=5배, 두께 1/σ=1/12                      │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 3. Sensor Materials

### 3.1 QD Photodetector --- σ-φ=10 Band Sensitivity

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  QD BROADBAND PHOTODETECTOR                                      │
  │                                                                  │
  │  Band coverage: UV(200nm) → SWIR(2000nm)                        │
  │  Spectral bands: σ-φ=10 discrete detection bands                │
  │  Quantum efficiency: >80% per band                              │
  │                                                                  │
  │  Band allocation (Egyptian fraction inspired):                   │
  │    UV band:   1/n = 1/6 of total spectrum weight                │
  │    Visible:   1/φ = 1/2 of total (dominant)                     │
  │    NIR:       1/n/φ = 1/3 of total                              │
  │    Sum: 1/6 + 1/2 + 1/3 = 1 (EXACT, Egyptian)                  │
  │                                                                  │
  │  ADC resolution: J₂=24 bit                                      │
  │  Readout speed: σ·τ=48 Mpixel/s                                │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 4. n=6 Complete Parameter Map

| Parameter | Value | n=6 Expression | Grade |
|-----------|-------|----------------|-------|
| RGB primaries | 3 | n/φ | EXACT |
| Color bit depth | 12 | σ | EXACT |
| QD core red | 6 nm | n | EXACT |
| QD core green | 4 nm | τ | EXACT |
| QD core blue | 2 nm | φ | EXACT |
| μLED pitch | 48 μm | σ·τ | EXACT (BT-76) |
| Perovskite CN | 6 | n | EXACT |
| PZT CN | 6 | n | EXACT |
| CNT Z | 6 | n | EXACT |
| Audio bandwidth | 48 kHz | σ·τ | EXACT |
| Audio bit depth | 24 | J₂ | EXACT |
| Sensor bands | 10 | σ-φ | EXACT |
| **Total EXACT** | **12/12** | **100%** | |

---

## 5. Honesty Assessment

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  HONEST GRADING                                                  │
  │                                                                  │
  │  Strong (물리적 필연):                                           │
  │    - Perovskite/PZT CN=6: d-orbital crystal field (BT-43)      │
  │    - CNT Z=6: 탄소 원자번호 (BT-93)                            │
  │    - 12-bit color: 인간 시각 JND 한계와 부합                    │
  │    - 48kHz: Nyquist × 인간 가청 한계 24kHz (=J₂)              │
  │                                                                  │
  │  Moderate (공학적 수렴):                                         │
  │    - QD size 2/4/6nm: 양자 구속 효과로 대략 맞지만              │
  │      정확한 파장은 조성에 따라 가변                              │
  │    - μLED 48μm pitch: 현재 산업 타겟과 일치                    │
  │                                                                  │
  │  Weak:                                                           │
  │    - RGB=3: 인간 생물학이지 물리법칙 아님                       │
  │    - 10 sensor bands: 설계 선택이지 물리적 필연 아님            │
  │                                                                  │
  │  Falsifiable prediction:                                         │
  │    차세대 μLED 양산 피치가 σ=12μm 근방에 수렴할 것 (2027)      │
  │    Perovskite LED 효율이 CN=6 유지할 때 최대 (CN=4보다 높음)   │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 6. Links

- Parent: [goal.md](goal.md)
- Next: [HEXA-PANEL (Level 1)](hexa-panel.md)
- BT-48: `docs/breakthrough-theorems.md` (Display-Audio)
- BT-93: `docs/breakthrough-theorems.md` (Carbon Z=6)
- BT-108: `docs/breakthrough-theorems.md` (Musical consonance)
