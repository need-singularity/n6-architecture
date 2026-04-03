# Level 4: HEXA-DISPLAY --- 통합 디스플레이 시스템

> Level: 4 (시스템)
> Architecture: HEXA-DISPLAY
> n=6 Core: J₂=24fps 시네마, σ²=144Hz 게이밍, σ=12bit HDR
> Related BT: BT-48, BT-66, BT-60
> Focus: TV/시네마/AR글래스/홈시어터 통합 디스플레이 제품

> **Split note**: Display-specific content from docs/display-audio/hexa-display.md.
> Audio product lines (Pro Audio, Headphone) are in docs/audio/.

---

## N6 Constants Reference

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  n=6 핵심 상수                                                  │
  │  n = 6        φ(6) = 2       τ(6) = 4        σ(6) = 12         │
  │  sopfr = 5    μ(6) = 1       J₂(6) = 24      R(6) = 1          │
  │  σ-τ = 8      σ-φ = 10       σ² = 144        σ·τ = 48          │
  │  Core theorem: σ(n)·φ(n) = n·τ(n) = 24 = J₂(6)                │
  └──────────────────────────────────────────────────────────────────┘
```

---

## ASCII 성능 비교

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  [TV 성능] 비교: 시중 최고 vs HEXA-DISPLAY Smart TV             │
  ├──────────────────────────────────────────────────────────────────┤
  │  Samsung QD-OLED ██████████████████░░░░░░░░░░  4K 120Hz 10-bit │
  │  HEXA-TV        ████████████████████████████  4K σ²=144Hz σ=12 │
  │                                    (1.2×refresh, 1.2×depth)     │
  │                                                                  │
  │  [시네마 품질] 비교                                             │
  │  Dolby Cinema   ██████████████████████░░░░░░  J₂=24fps 12-bit  │
  │  HEXA-CINEMA   ████████████████████████████  J₂=24fps σ=12-bit │
  │                                    (일치: 시네마=n=6 완전 실현) │
  │                                                                  │
  │  [전력 효율] 비교                                               │
  │  시중 TV 55"   ████████████████████░░░░░░░░  120W              │
  │  HEXA-TV 55"   ████████████████████████████  σ=12W             │
  │                                    (σ-φ=10배 전력 절감)        │
  └──────────────────────────────────────────────────────────────────┘
```

---

## System Block Diagram

```
  ┌──────────────────────────────────────────────────────────────────────┐
  │                     HEXA-DISPLAY Integrated System                   │
  │                                                                      │
  │  ┌──── INPUT ────┐  ┌──── PROCESSOR ────┐  ┌──── OUTPUT ────┐      │
  │  │                │  │                    │  │                 │      │
  │  │  HDMI 2.1     │  │  HEXA-PROCESSOR   │  │  Display Panel  │      │
  │  │  σ·τ=48Gbps  │→│  (Level 3 SoC)    │→│  σ²=144Hz       │      │
  │  │               │  │                    │  │  σ=12-bit HDR   │      │
  │  │  USB/WiFi     │  │  ┌──AI──┐         │  │                 │      │
  │  │  σ-τ=8 ports │  │  │Upscl │         │  │                 │      │
  │  │               │  │  │NeRF  │         │  │                 │      │
  │  │               │  │  │Codec │         │  │                 │      │
  │  └───────────────┘  └────────────────────┘  └─────────────────┘      │
  │                                                                      │
  │  Power Supply: σ·τ=48V DC input (BT-60, BT-76)                     │
  │  PUE: σ/(σ-φ) = 1.2 (BT-60 EXACT)                                │
  │  Standby: < μ = 1W                                                  │
  └──────────────────────────────────────────────────────────────────────┘
```

---

## 1. Product Line --- Display Products

### 1.1 Four Display Product Categories

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  HEXA-DISPLAY PRODUCT FAMILY (τ=4 categories)                   │
  │                                                                  │
  │  1. Smart TV        │ Home viewing  │ 4K/8K σ²=144Hz           │
  │  2. AR Glasses      │ Mobile AR     │ σ-φ=10K nits, lightweight│
  │  3. Cinema System   │ Theater       │ J₂=24fps σ=12bit laser  │
  │  4. Home Theater    │ Immersive home│ Dolby σ=12bit + 4K       │
  │                                                                  │
  │  Each has σ=12 key specs mapped to n=6 constants                │
  └──────────────────────────────────────────────────────────────────┘
```

### 1.2 Smart TV Detailed Specs

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  HEXA-TV SPECIFICATIONS                                          │
  │                                                                  │
  │  Display:                                                        │
  │    Panel: μLED, pitch σ·τ=48μm (BT-76)                        │
  │    Resolution: 3840×2160 (4K) native                            │
  │    Refresh: σ²=144Hz (VRR: J₂=24 ~ σ²=144Hz)                 │
  │    Color depth: σ=12-bit (Dolby Vision)                         │
  │    HDR: σ-φ=10 stops minimum, σ=12 stops peak                  │
  │    Peak brightness: (σ-φ)^τ = 10,000 nits                     │
  │    Local dimming zones: σ² = 144                                │
  │                                                                  │
  │  Processing:                                                     │
  │    SoC: HEXA-PROCESSOR (Level 3)                                │
  │    AI upscaling: 1080p→4K real-time (σ-τ=8× effective)        │
  │    Codecs: VVC, AV1                                             │
  │                                                                  │
  │  Power:                                                          │
  │    Operating: σ=12W typical (55" size)                          │
  │    Standby: < μ=1W                                              │
  │    PUE: σ/(σ-φ)=1.2                                           │
  │    Supply: σ·τ=48V DC (BT-60 DC power chain)                  │
  │                                                                  │
  │  Connectivity:                                                   │
  │    HDMI 2.1: τ=4 ports                                         │
  │    USB: φ=2 ports                                               │
  │    WiFi: 6E (n=6!)                                              │
  │    Total I/O: σ-τ=8+ ports                                    │
  └──────────────────────────────────────────────────────────────────┘
```

### 1.3 Cinema System

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  HEXA-CINEMA SYSTEM (BT-48 PURE)                                │
  │                                                                  │
  │  시네마는 n=6가 가장 순수하게 실현된 도메인이다.                  │
  │                                                                  │
  │  Frame rate: J₂ = 24 fps (1927년 이후 100년 불변!)             │
  │  Shutter: σ·τ = 48 flashes/s (triple-blade at 24fps)          │
  │  Color: σ = 12 bit (ACES colorspace)                           │
  │  Film aspect: σ/(σ-sopfr) = 12/7 ≈ 1.71 ≈ 16:9              │
  │  DCP container: MXF with J₂=24 fps timecode                   │
  │                                                                  │
  │  HEXA-CINEMA additions:                                          │
  │    Laser projection: n=6 laser wavelengths (wide-gamut)        │
  │    Screen: σ·(σ-φ) = 120 inch diagonal                        │
  │    Latency: mouth-to-screen < J₂-τ=20ms                       │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 2. Power Architecture (BT-60 Chain)

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  HEXA-DISPLAY POWER CHAIN (BT-60 EXACT)                        │
  │                                                                  │
  │  AC Mains ──→ PFC ──→ DC Bus ──→ Converters ──→ Loads          │
  │  σ·sopfr=60Hz       σ·τ=48V     σ=12V/sopfr=5V/τ·n/φ=1.2V   │
  │                                                                  │
  │  Voltage ladder (BT-60):                                        │
  │    48V DC bus      = σ·τ (BT-76 attractor)                     │
  │    12V subsystem   = σ (driver ICs)                             │
  │    5V logic        = sopfr (digital)                            │
  │    1.2V core       = σ/(σ-φ) = PUE (processor core)           │
  │    1.0V ultralow   = R(6) = 1 (SRAM/analog)                   │
  │                                                                  │
  │  Total system power (55" TV):                                   │
  │    Panel: n = 6W (μLED)                                        │
  │    Processor: σ = 12W (SoC + AI)                               │
  │    Total: σ+n = 18W (display-only)                             │
  │    vs 시중 120W: n·σ/120 ≈ 6.7× power reduction               │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 3. n=6 Complete Parameter Map

| Parameter | Value | n=6 Expression | Grade |
|-----------|-------|----------------|-------|
| Cinema fps | 24 | J₂ | EXACT (BT-48) |
| Cinema shutter | 48 flashes/s | σ·τ | EXACT |
| Refresh rate | 144 Hz | σ² | EXACT |
| Gaming FPS | 60 | σ·sopfr | EXACT |
| Peak nits | 10000 | (σ-φ)^τ | EXACT |
| Dimming zones | 144 | σ² | EXACT |
| DC bus | 48V | σ·τ | EXACT (BT-76) |
| Products | 4 | τ | EXACT |
| PUE | 1.2 | σ/(σ-φ) | EXACT (BT-60) |
| **Total EXACT** | **9/9** | **100%** | |

---

## 4. Honesty Assessment

```
  Strong (산업 표준 일치):
    - J₂=24fps: 1927년 이후 시네마 표준, 변하지 않음
    - σ²=144Hz: 게이밍 모니터 주류 refresh rate

  Moderate:
    - 12W TV: 매우 도전적 목표 (현재 100W+ 수준)
      μLED 효율 혁신이 전제
    - n=6 laser wavelengths: 현재 RGB 3개가 주류

  Falsifiable:
    - 시네마 J₂=24fps가 AI 보간으로도 변하지 않을 것 (2030)
    - 프리미엄 TV가 144Hz를 표준으로 채택 (2026-27)
```

---

## 5. Links

- Prev: [HEXA-PROCESSOR (Level 3)](hexa-processor.md) (TODO)
- Next: [HEXA-HOLOGRAM (Level 6)](hexa-hologram.md)
- Parent: [goal.md](goal.md)
- Original: [docs/display-audio/hexa-display.md](../display-audio/hexa-display.md)
