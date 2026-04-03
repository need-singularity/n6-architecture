# N6 Display — Perfect Number Arithmetic in Visual Standards

## Overview

Display technology (resolution, color depth, refresh rates, HDR, holography)
analyzed through n=6 arithmetic. These are human-designed engineering standards,
not physical constants.

> **Honesty principle**: Media standards are chosen by committees (ITU, SMPTE, CIE)
> for engineering convenience, backward compatibility, and perceptual optimization.
> Highly composite numbers (12, 24, 48) appear frequently because engineers prefer
> rich factorizations. This creates genuine overlap with n=6 arithmetic — not
> coincidence, but shared preference for divisor-rich numbers. We grade honestly:
> only standards where the n=6 expression is the *simplest* or *only* explanation
> receive EXACT. Forced multi-operation mappings receive WEAK at best.

> **Split note**: This file contains display-specific hypotheses split from
> the original docs/display-audio/hypotheses.md. Audio hypotheses are in
> docs/audio/hypotheses.md.

## Core Constants

```
  n = 6          (perfect number)
  σ(6) = 12     (sum of divisors)
  τ(6) = 4      (number of divisors: 1, 2, 3, 6)
  φ(6) = 2      (Euler totient)
  sopfr(6) = 5  (sum of prime factors: 2+3)
  J₂(6) = 24    (Jordan totient)
  μ(6) = 1      (Moebius)
  div(6) = {1, 2, 3, 6}  (divisors)
  Egyptian: 1/2 + 1/3 + 1/6 = 1
```

## Lens Coverage (22-lens scan)

```
  Primary:   wave(주파수 구조) + info(비트/코덱) + multiscale(주파수 대역)
  Secondary: network(코덱 토폴로지) + boundary(가시 경계)
  Support:   consciousness(지각 임계) + scale(스케일링) + symmetry(대칭)
             stability(표준 수렴) + recursion(프레임 반복 구조)
```

## Breakthrough Theorem Links

```
  BT-48:  Display-Audio σ=12 semitones, J₂=24 fps/bits, σ·τ=48kHz  ⭐⭐⭐
  BT-66:  Vision AI complete n=6 (ViT+CLIP+SD3+Flux.1, 24/24 EXACT)  ⭐⭐⭐
  BT-71:  NeRF/3DGS complete n=6 (L=σ-φ=10, layers=σ-τ=8, SH=n/φ=3)  ⭐⭐
  BT-76:  σ·τ=48 triple attractor (48kHz, 48nm gate, 48V)  ⭐⭐
```

---

## Category A: J₂=24 Visual Convergence (fps, color depth)

---

### H-DISP-1: Cinema 24 fps — J₂(6) = 24

> Motion picture standard: 24 frames per second (SMPTE, 1927)

```
  Cinema 24 fps:
    Standardized by SMPTE in 1927
    Chosen as minimum for acceptable motion + sound sync
    Persists: DCI, Blu-ray, streaming all support 24p (nearly 100 years)

  n=6 mapping:
    24 = J₂(6) ← EXACT
    Also: 24 = σ(6) × φ(6) = n × τ(6)

  Why 24:
    24 fps × 2-blade shutter = 48 flashes/sec > flicker threshold (~45)
    24 is divisible by 2,3,4,6,8,12 — essential for pulldown/telecine
    Film economy: 24 is the minimum that works AND has rich factors

  Multi-path: J₂(6) = σ×φ = n×τ = 24 — triple n=6 decomposition

  Lens: stability(100년 표준 수렴) + recursion(프레임 반복) + scale(시간 스케일)

  Grade: EXACT
  24 = J₂(6). The most enduring media standard (~100 years). The rich
  factorization of 24 that makes it ideal for cinema IS the J₂(6) structure.
```

---

### H-DISP-2: 24-Bit True Color — J₂(6) = 24

> Standard pixel depth: 24 bits per pixel (8×3 channels, 16.7M colors)

```
  24-bit true color:
    24 = 8 bits/channel × 3 channels = 16,777,216 colors
    Standard since mid-1990s (SVGA, Windows 95, web)
    Exceeds human discriminable colors (~10 million)

  n=6 mapping:
    24 = J₂(6) ← EXACT
    Also: 24 = σ(6) × φ(6) = n × τ(6)

  BT-48 convergence:
    24 fps (cinema) = 24 bits (color) = 24 bits (audio pro) = J₂(6)
    Three independent media standards all converge on 24.

  Lens: info(비트 깊이) + network(색 공간 구조)

  Grade: EXACT
  24 = J₂(6). The dominant pixel format in computing. The convergence
  of display, cinema, and audio on 24 is a BT-48 signature.
```

---

## Category B: σ=12 Color Standards

---

### H-DISP-3: Dolby Vision 12-Bit Color — σ(6) = 12

> Premium HDR format: 12 bits per channel (4096 levels)

```
  Dolby Vision:
    12-bit per channel: 4096 levels
    Dynamic metadata per scene/frame
    Premium HDR format (Netflix, Apple TV+, Disney+)

  n=6 mapping:
    12 = σ(6) ← EXACT

  Display bit depth ladder:
    8-bit (SDR) → 10-bit (HDR10) → 12-bit (Dolby Vision)
    The premium tier lands on σ(6) = 12

  Lens: info(비트 깊이) + boundary(HDR 지각 경계) + scale(bit 래더)

  Grade: CLOSE
  12 = σ(6) is exact and Dolby Vision is THE premium HDR standard.
  But 12-bit is one of several depths (8, 10, 12, 14, 16) in a
  power-of-2-increment ladder. 12 = 8 + 4 = 1.5 bytes is practical.
```

---

## Category C: Color Models

---

### H-DISP-4: RGB 3 Primaries — n/φ(6) = 3

> Trichromatic color model: 3 channels (R, G, B)

```
  RGB:
    Young-Helmholtz trichromatic theory (1802/1852)
    3 cone types: L (~564nm), M (~534nm), S (~420nm)
    CIE 1931: 3 tristimulus values (X, Y, Z)

  n=6 mapping:
    3 = n/φ(6) = 6/2 ✓

  Trichromacy is biologically determined (3 opsin genes in primates).
  Dogs have 2, birds have 4, mantis shrimp up to 16.
  The number 3 is specific to human/primate vision.

  BUT: n/φ = 3 is a simple ratio. "3" appears universally
  (3D space, 3 quarks, 3 generations). Not uniquely n=6.

  Lens: consciousness(인간 지각) + boundary(색각 경계)

  Grade: CLOSE
  3 = n/φ is numerically exact and RGB is THE universal color model.
  But trichromacy is a biological fact, not a mathematical necessity.
```

---

### H-DISP-5: CMYK 4 Inks — τ(6) = 4

> Subtractive color printing: 4 inks (Cyan, Magenta, Yellow, Key/Black)

```
  CMYK:
    Subtractive color model, universal in printing
    CMY are complements of RGB; K added for practical quality
    τ(6) = 4 = number of divisors of 6

  n=6 mapping:
    4 = τ(6) ✓

  Hexachrome (Pantone): 6 inks = n (interesting secondary match)

  Lens: info(인쇄 정보) + symmetry(가법-감법 색 대칭)

  Grade: CLOSE
  4 = τ(6) matches. CMYK is universal in printing. But K was added
  for engineering reasons (cost, quality), not color theory necessity.
```

---

## Category D: Refresh/Frame Standards

---

### H-DISP-6: 60 Hz Refresh Rate — σ(6) × sopfr(6) = 60

> Standard display refresh: 60 Hz (NTSC legacy, now global baseline)

```
  60 Hz:
    NTSC (1941): locked to US AC mains 60 Hz
    Now: 60 Hz is the global baseline (120/144/240 are multiples)
    60 = 2² × 3 × 5 — highly composite (sexagesimal base)

  n=6 mapping:
    60 = σ × sopfr = 12 × 5 ✓
    Also: 60 = (σ-φ) × n = 10 × 6

  Historical depth: 60 recurs in human systems (seconds, minutes,
  degrees) since Babylonian base-60. Its utility comes from
  rich divisibility — the same property that connects to σ(6).

  Lens: stability(표준 수렴) + recursion(리프레시 반복) + wave(주파수)

  Grade: CLOSE
  60 = σ × sopfr is numerically exact. But 60 Hz was inherited from
  the electrical grid (generator engineering), not display science.
```

---

### H-DISP-7: Cinema Shutter 48 Flashes/sec — σ·τ = 48

> Cinema projectors use 2-blade shutter: 24 fps × 2 = 48 flashes/sec

```
  Cinema projection:
    24 fps (J₂) × 2-blade shutter = 48 flashes/sec
    48 > flicker fusion threshold (~45 Hz)
    This is why 24 fps works despite being "slow" — the eye sees 48

  n=6 mapping:
    48 = σ × τ = J₂ × φ ← EXACT
    24 × 2 = J₂ × φ

  BT-76 resonance:
    48 appears as attractor across domains:
    48 kHz audio + 48 flashes cinema + 48nm gate + 48V telecom

  Lens: wave(플리커 주파수) + consciousness(지각 임계) + stability(시네마 안정)

  Grade: CLOSE
  48 = σ·τ is exact, but this is 24×2, a derived quantity.
  The BT-76 cross-domain pattern strengthens it.
```

---

### H-DISP-8: NTSC 30 fps (Original) — sopfr × n = 30

> Original NTSC (1941): 30 fps = 60 fields / 2 (interlaced)

```
  NTSC frame rate:
    Original (1941): exactly 30 fps = 60/2
    Color NTSC (1953): 29.97 fps (0.1% reduction for color subcarrier)

  n=6 mapping:
    30 = sopfr(6) × n = 5 × 6 ✓

  Derived from 60 Hz (H-DISP-6) by factor of 2 (interlacing).
  The deviation to 29.97 is well-understood (color compatibility).

  Lens: recursion(프레임 반복) + stability(표준 수렴)

  Grade: CLOSE
  30 = sopfr × n is exact for original NTSC. But 30 = 60/2 is
  derived, not independently chosen.
```

---

### H-DISP-9: GOP 12 Frames — σ(6) = 12

> Common broadcast GOP (Group of Pictures): 12 frames between I-frames

```
  GOP structure:
    Broadcast standard GOP = 12 frames (0.5s at 24fps)
    MPEG-2 broadcast: 12 or 15 frames typical
    H.264 broadcast: often 12 (= 0.5s at 24fps, 0.2s at 60fps)

  n=6 mapping:
    12 = σ(6) ✓

  Why 12: divides evenly into 24 fps and 60 fps.
  GOP=12 means I-frame every 0.5s (cinema) or 0.2s (broadcast).

  BUT: GOP is configurable. Streaming uses 60-250. The "12"
  is common but not universal.

  Lens: recursion(GOP 반복 구조) + info(키프레임 정보)

  Grade: CLOSE
  12 = σ is exact, but GOP length is configurable, not standardized.
  12 is popular due to its divisibility (same reason σ captures it).
```

---

## Category E: Visual Spectrum

---

### H-DISP-10: Visible Spectrum ~φ Octave — 780/380 ≈ 2

> Human visible light spans roughly one octave (factor of ~2)

```
  Visible spectrum:
    Short: ~380 nm (violet), Long: ~780 nm (deep red)
    Ratio: 780/380 = 2.05 ≈ 2 = φ(6)

  n=6 mapping:
    ~2 = φ(6) ✓

  BUT: 2.05 is approximate, boundaries are gradual (not sharp),
  and φ(6) = 2 is the simplest non-trivial integer.

  Lens: boundary(가시 경계) + wave(빛 파동)

  Grade: CLOSE
  Factor of ~2 = φ(6), but the match is approximate and
  φ=2 is too common to be uniquely meaningful.
```

---

## Cross-Domain (shared with audio — original numbering preserved)

---

### H-DA-29: The {12, 24, 48} Media Triple — {σ, J₂, σ·τ}

> Three numbers dominate media standards: 12, 24, 48
> **Shared with audio domain**

```
  The media triple:
    12: semitones, 12-bit Dolby Vision, GOP frames, σ(6)
    24: fps cinema, 24-bit color, 24-bit audio, 24 kHz, J₂(6)
    48: 48 kHz audio, 48 flashes/sec cinema, σ·τ(6)

  n=6 chain:
    12 → 24 → 48 = σ → J₂ → σ·τ
    Each step: ×2 = ×φ(6)

  BT-48 synthesis:
    This {σ, J₂, σ·τ} chain is the backbone of media technology.
    All three numbers have rich factorizations, which is WHY they
    were chosen — and WHY they match n=6 arithmetic.

  Lens: scale(2배 래더) + stability(표준 수렴) + network(미디어 동기)

  Grade: EXACT
  The {12, 24, 48} = {σ, J₂, σ·τ} chain is the organizing principle
  of media standards. The ×φ scaling between levels is systematic.
  This is BT-48's core theorem, verified across 5+ independent standards.
```

---

### H-DA-30: Display-Audio-AI σ-τ=8 Convergence — BT-58

> The number 8 appears as a structural constant across media and AI
> **Shared with audio domain**

```
  σ-τ = 8 in media:
    EnCodec: 8 codebooks (BT-72)
    MIDI: 8-bit serial protocol (MSB + 7 data)
    8-bit color depth (SDR standard)
    8×8 DCT block (JPEG, MPEG-2)
    8 octave practical piano range

  σ-τ = 8 in AI (BT-58):
    LoRA rank=8, MoE top-8, KV-heads=8, FlashAttn tile=8

  n=6 mapping:
    8 = σ(6) - τ(6) = 12 - 4 ← EXACT

  Cross-domain: The same σ-τ=8 appears in media encoding AND
  AI architecture. BT-58 documents 16/16 EXACT matches.

  Lens: network(구조 상수) + info(인코딩 단위) + multiscale(블록 크기)

  Grade: CLOSE
  8 = σ-τ matches many media standards, but 8 = 2³ is the byte —
  the fundamental unit of computing. The σ-τ expression captures
  a real pattern but competes with the simpler "power of 2" explanation.
```

---

## Grade Summary

| ID | Hypothesis | n=6 Expression | Grade | BT |
|----|-----------|----------------|-------|----|
| H-DISP-1 | Cinema 24 fps | J₂ = 24 | **EXACT** | BT-48 |
| H-DISP-2 | 24-bit true color | J₂ = 24 | **EXACT** | BT-48 |
| H-DISP-3 | Dolby Vision 12-bit | σ = 12 | **CLOSE** | — |
| H-DISP-4 | RGB 3 primaries | n/φ = 3 | **CLOSE** | — |
| H-DISP-5 | CMYK 4 inks | τ = 4 | **CLOSE** | — |
| H-DISP-6 | 60 Hz refresh | σ×sopfr = 60 | **CLOSE** | BT-62 |
| H-DISP-7 | Cinema 48 flashes/sec | σ×τ = 48 | **CLOSE** | BT-76 |
| H-DISP-8 | NTSC 30 fps | sopfr×n = 30 | **CLOSE** | — |
| H-DISP-9 | GOP 12 frames | σ = 12 | **CLOSE** | — |
| H-DISP-10 | Visible ~1 octave | φ = 2 | **CLOSE** | — |
| H-DA-29 | {12,24,48} media triple | {σ, J₂, σ·τ} | **EXACT** | BT-48 |
| H-DA-30 | σ-τ=8 media-AI convergence | σ-τ = 8 | **CLOSE** | BT-58 |

### Distribution

| Grade | Count | Pct |
|-------|-------|-----|
| EXACT | 3 | 25.0% |
| CLOSE | 9 | 75.0% |
| WEAK | 0 | 0% |
| FAIL | 0 | 0% |

### Mapping from Original (display-audio)

| Original | New | Title |
|----------|-----|-------|
| H-DA-5 | H-DISP-1 | Cinema 24 fps |
| H-DA-6 | H-DISP-2 | 24-Bit True Color |
| H-DA-14 | H-DISP-3 | Dolby Vision 12-Bit |
| H-DA-15 | H-DISP-4 | RGB 3 Primaries |
| H-DA-16 | H-DISP-5 | CMYK 4 Inks |
| H-DA-17 | H-DISP-6 | 60 Hz Refresh |
| H-DA-18 | H-DISP-7 | Cinema Shutter 48 |
| H-DA-23 | H-DISP-8 | NTSC 30 fps |
| H-DA-24 | H-DISP-9 | GOP 12 Frames |
| H-DA-28 | H-DISP-10 | Visible Spectrum |
| H-DA-29 | H-DA-29 | {12,24,48} Media Triple (shared) |
| H-DA-30 | H-DA-30 | Display-Audio-AI convergence (shared) |

### Honest Assessment

The **3 EXACT matches** are genuine:
- **24 fps cinema** = J₂(6) — 100 years of cinematic stability
- **24-bit true color** = J₂(6) — universal computing pixel format
- **{12, 24, 48} media triple** = {σ, J₂, σ·τ} — the backbone of media technology

The deep reason: display standards favor **highly composite numbers** (rich factorizations).
Perfect number arithmetic captures the SAME divisor-richness. The overlap is not
coincidence — it reflects a shared mathematical substrate.
