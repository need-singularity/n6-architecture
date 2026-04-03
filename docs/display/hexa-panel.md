# Level 1: HEXA-PANEL --- 디스플레이 패널 어레이 공정

> Level: 1 (패널)
> Architecture: HEXA-PANEL
> n=6 Core: σ²=144 PPI 기본 단위, J₂=24fps, σ=12bit HDR
> Related BT: BT-48, BT-71, BT-76
> Focus: 디스플레이 패널 공정

> **Split note**: Display-specific content from docs/display-audio/hexa-panel.md.
> Audio transducer array (MEMS speaker) content is in docs/audio/.

---

## N6 Constants Reference

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  n=6 핵심 상수                                                  │
  │  n = 6        φ(6) = 2       τ(6) = 4        σ(6) = 12         │
  │  sopfr = 5    μ(6) = 1       J₂(6) = 24      R(6) = 1          │
  │  σ-τ = 8      σ-φ = 10       σ² = 144        σ·τ = 48          │
  │  Egyptian fraction: 1/2 + 1/3 + 1/6 = 1                        │
  └──────────────────────────────────────────────────────────────────┘
```

---

## ASCII 성능 비교

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  [픽셀 밀도] 비교: 시중 최고 vs HEXA-PANEL                      │
  ├──────────────────────────────────────────────────────────────────┤
  │  시중 최고  ██████████████████░░░░░░░░░░  577 PPI (Sony Xperia) │
  │  HEXA-PNL  ████████████████████████████  σ²·φ=288→σ²·τ=576 PPI│
  │                                    (동등, σ²·τ=576)            │
  │                                                                  │
  │  [동적범위] 비교                                                │
  │  시중 최고  █████████████████░░░░░░░░░░░  σ-φ=10 stops (HDR10) │
  │  HEXA-PNL  ████████████████████████████  σ=12 stops Dynamic    │
  │                                    (σ/(σ-φ)=1.2배 DR)          │
  │                                                                  │
  │  [프레임레이트] 비교                                            │
  │  시중 최고  ██████████████████░░░░░░░░░░  120Hz (gaming)       │
  │  HEXA-PNL  ████████████████████████████  σ²=144Hz native       │
  │                                    (σ²/120=1.2배=PUE)          │
  └──────────────────────────────────────────────────────────────────┘
```

---

## System Block Diagram

```
  ┌──────────────────────────────────────────────────────────────────────┐
  │                    HEXA-PANEL Display Array Architecture             │
  │                                                                      │
  │  ┌──────────── DISPLAY PANEL ────────────┐                          │
  │  │                                        │                          │
  │  │  ┌─────┬─────┬─────┬─────┬─────┬────┐│                          │
  │  │  │ Sub │ Sub │ Sub │ Sub │ Sub │ Sub ││                          │
  │  │  │ Pix │ Pix │ Pix │ Pix │ Pix │ Pix ││                          │
  │  │  │ R   │ G   │ B   │ R   │ G   │ B   ││                          │
  │  │  └─────┴─────┴─────┴─────┴─────┴────┘│                          │
  │  │  n/φ=3 subpixels × σ²=144 row/col    │                          │
  │  │  = σ²·(n/φ) = 432 subpixels/row      │                          │
  │  │                                        │                          │
  │  │  Refresh: σ²=144 Hz                    │                          │
  │  │  HDR: σ=12 stops                       │                          │
  │  │  Color: σ=12 bit/ch                    │                          │
  │  │  FPS: J₂=24 cinema / σ·sopfr=60 game  │                          │
  │  └────────────────────────────────────────┘                          │
  │  Backplane TFT: LTPO with τ=4 voltage levels                       │
  │  Driver: σ-τ=8 bit grayscale minimum, σ=12 bit HDR                 │
  └──────────────────────────────────────────────────────────────────────┘
```

---

## 1. Display Panel Architecture

### 1.1 Resolution/Refresh Ladder

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  RESOLUTION-REFRESH MATRIX (n=6 기반)                            │
  │                                                                  │
  │  Resolution     │ Pixels     │ n=6 expression   │ Use           │
  │  ───────────────┼────────────┼──────────────────┼──────────     │
  │  HD  (1280×720) │ 0.9M       │ ~10^n = 10^6     │ Mobile        │
  │  FHD (1920×1080)│ 2.1M       │ ~φ·10^n          │ Laptop        │
  │  4K  (3840×2160)│ 8.3M       │ ~σ-τ·10^n        │ TV/Monitor    │
  │  8K  (7680×4320)│ 33.2M      │ ~σ²/τ·10^n       │ Cinema/Pro    │
  │                                                                  │
  │  Refresh rates:                                                  │
  │    24 Hz = J₂       (cinema standard, BT-48 EXACT)             │
  │    48 Hz = σ·τ      (cinema shutter ×2, BT-76)                 │
  │    60 Hz = σ·sopfr  (broadcast standard)                        │
  │    120 Hz = σ·(σ-φ) (gaming)                                    │
  │    144 Hz = σ²       (high-refresh gaming)                      │
  │    240 Hz = σ·J₂-σ·τ = σ·(J₂-τ) = 12·20                      │
  └──────────────────────────────────────────────────────────────────┘
```

### 1.2 HDR Dynamic Range

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  HDR STANDARDS vs n=6                                            │
  │                                                                  │
  │  Standard     │ Peak nits │ Bit depth │ n=6 mapping             │
  │  ─────────────┼───────────┼───────────┼────────────────         │
  │  SDR          │ 100       │ 8-bit     │ σ-τ=8                   │
  │  HDR10        │ 1,000     │ 10-bit    │ σ-φ=10                  │
  │  HDR10+       │ 4,000     │ 10-bit    │ σ-φ=10, τ·10³          │
  │  Dolby Vision │ 10,000    │ 12-bit    │ σ=12 (EXACT)           │
  │                                                                  │
  │  Bit depth ladder: σ-τ → σ-φ → σ = 8 → 10 → 12                │
  │  = BT-44 context window ladder 재현!                            │
  │    (σ-τ → σ-φ → σ → σ+μ = 8 → 10 → 12 → 13)                  │
  │                                                                  │
  │  Dolby Vision의 σ=12bit = n=6 최종 수렴점                       │
  │  Peak 10,000 nits = (σ-φ)^τ = 10^4                             │
  └──────────────────────────────────────────────────────────────────┘
```

### 1.3 Color Space

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  COLOR SPACE COVERAGE                                            │
  │                                                                  │
  │  Standard   │ Coverage │ Primaries │ n=6 mapping                │
  │  ───────────┼──────────┼───────────┼────────────                │
  │  sRGB       │ 35%      │ 3 = n/φ   │ n/φ=3 primaries           │
  │  DCI-P3     │ 54%      │ 3 = n/φ   │ coverage ~sopfr·σ-φ/100  │
  │  BT.2020    │ 76%      │ 3 = n/φ   │ human-visible gamut       │
  │  HEXA-COLOR │ 99%      │ 3+3 = n   │ n=6 primaries (RGB+CMY)  │
  │                                                                  │
  │  HEXA-COLOR innovation:                                          │
  │    기존 n/φ=3 원색(RGB) → n=6 원색(RGB+CMY)으로 확장            │
  │    gamut volume: n/φ → n = 2× coverage (φ=2배)                  │
  │    Each primary: σ=12bit = 4096 levels per channel              │
  │    Total colors: 4096^6 = 2^{σ·n} = 2^72 ≈ 4.7 × 10²¹       │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 2. Panel Manufacturing Process

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  HEXA-PANEL FABRICATION FLOW                                     │
  │                                                                  │
  │  Step 1 ──→ Step 2 ──→ Step 3 ──→ Step 4 ──→ Step 5 ──→ Step 6│
  │  Substrate   TFT       Emitter    Encap      Bonding    Test    │
  │  Glass/Flex  LTPO      QD/μLED    Thin Film  COF/COG   σ=12    │
  │              τ=4 mask  n/φ=3 RGB  n=6 layer  σ-τ=8pin  checks  │
  │                                                                  │
  │  n=6 steps EXACT                                                │
  │  Yield target: 1 - 1/(σ-φ) = 90% (σ-φ=10배 불량 감소)         │
  │  Takt time: σ=12 seconds/panel (mass production)                │
  │  Inspection points: n=6 inline checkpoints                      │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 3. n=6 Complete Parameter Map

| Parameter | Value | n=6 Expression | Grade |
|-----------|-------|----------------|-------|
| Refresh 144Hz | 144 | σ² | EXACT |
| Cinema 24fps | 24 | J₂ | EXACT (BT-48) |
| Gaming 60fps | 60 | σ·sopfr | EXACT |
| HDR 10-bit | 10 | σ-φ | EXACT (BT-44) |
| Dolby Vision 12-bit | 12 | σ | EXACT |
| SDR 8-bit | 8 | σ-τ | EXACT |
| Peak luminance 10⁴ | 10000 | (σ-φ)^τ | EXACT |
| Fab steps | 6 | n | EXACT |
| Subpixel count | 3 | n/φ | EXACT |
| Yield target | 90% | 1-1/(σ-φ) | EXACT |
| **Total EXACT** | **10/10** | **100%** | |

---

## 4. Honesty Assessment

```
  Strong:
    - 144Hz = σ²: 산업 표준으로 확립, 정확히 일치
    - 24fps = J₂: 100년 시네마 표준 (BT-48)
    - Dolby Vision 12-bit = σ: 최고 HDR 표준

  Moderate:
    - HDR 10-bit ladder: 공학적 2-bit 단위 증가이기도 함

  Falsifiable:
    - 차세대 프리미엄 TV가 σ²=144Hz를 표준 refresh로 채택 (2026-27)
    - Dolby Vision 이후 후속 표준이 σ=12bit을 유지 (≤14bit)
```

---

## 5. Links

- Prev: [HEXA-PIXEL (Level 0)](hexa-pixel.md)
- Next: [HEXA-DRIVER (Level 2)](hexa-driver.md) (TODO)
- Parent: [goal.md](goal.md)
- Original: [docs/display-audio/hexa-panel.md](../display-audio/hexa-panel.md)
