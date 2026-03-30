# N6 Display & Audio — Perfect Number Arithmetic in Media Standards

## Overview

Display technology (resolution, color depth, refresh rates, color spaces) and audio
standards (sample rates, bit depth, frequency bands, codecs) analyzed through n=6
arithmetic. These are human-designed engineering standards, not physical constants.

> **Honesty principle**: Display and audio standards are chosen by committees (ITU,
> SMPTE, AES, CIE) for engineering convenience, backward compatibility, and
> perceptual optimization. Round numbers (24, 48, 60, 12) appear frequently in
> engineering for reasons unrelated to n=6. Many matches will be coincidental.

## Core Constants

```
  n = 6          (perfect number)
  σ(6) = 12     (sum of divisors)
  τ(6) = 4      (number of divisors: 1, 2, 3, 6)
  φ(6) = 2      (Euler totient)
  sopfr(6) = 5  (sum of prime factors: 2+3)
  J₂(6) = 24    (Jordan totient)
  μ(6) = 1      (Moebius)
  λ(6) = 2      (Carmichael)
  R(6) = σ·φ/(n·τ) = 1
  Egyptian: 1/2 + 1/3 + 1/6 = 1
```

---

## Category A: Color Fundamentals

---

### H-DA-1: RGB Primary Colors — 3 = n/φ(6)

> Human color vision uses 3 cone types (L, M, S), leading to 3 primary colors (R, G, B)

```
  Trichromatic color vision:
    3 cone types: Long (~564nm), Medium (~534nm), Short (~420nm)
    Young-Helmholtz theory (1802/1852): 3 primaries sufficient
    CIE 1931: X, Y, Z tristimulus → 3 coordinates

  n=6 mapping:
    3 = n/φ(6) = 6/2 ✓

  Physical basis:
    Trichromacy is biologically determined by 3 opsin genes.
    Many animals have 2 (dogs) or 4 (birds, mantis shrimp) types.
    "3" is specific to human/primate vision, not a universal constant.

  BUT:
    n/φ = 3 is a simple ratio. The number 3 appears in countless
    unrelated contexts (spatial dimensions, quarks, etc.).
    RGB is a direct consequence of human cone count, not n=6.

  Grade: CLOSE
  The match is numerically exact (3 = n/φ), and RGB is the dominant
  color model in all digital displays. But trichromacy is biological
  accident, not mathematical necessity.
```

---

### H-DA-2: 8-Bit Color Depth — 256 Levels per Channel

> Standard color depth = 8 bits per channel, and 8 = σ(6) - τ(6)

```
  8-bit color:
    2^8 = 256 levels per channel
    Established by VGA (1987), persists in sRGB, JPEG, PNG
    Matches human JND (just noticeable difference) at ~1% contrast

  n=6 mapping:
    8 = σ(6) - τ(6) = 12 - 4 ✓

  BUT:
    8 = 2³ was chosen because it is a byte — the fundamental
    addressable unit in computing since the IBM System/360 (1964).
    The byte being 8 bits comes from character encoding needs
    (ASCII 7-bit + parity), not from display science.
    10-bit, 12-bit, and 16-bit are also common in professional use.

  Grade: WEAK
  8 = σ - τ is numerically correct but the reason for 8-bit color
  is the byte, which predates and is independent of display technology.
  The formula σ - τ is also somewhat arbitrary.
```

---

### H-DA-3: 24-Bit True Color — J₂(6) = 24

> True color = 24 bits per pixel = 8 bits × 3 channels

```
  24-bit true color:
    24 = 8 bits/channel × 3 channels = 16,777,216 colors
    Standard since mid-1990s (SVGA, Windows 95)
    Exceeds human discriminable colors (~10 million)

  n=6 mapping:
    24 = J₂(6) ← EXACT match
    Also: 24 = σ(6) × φ(6) = 12 × 2
    Also: 24 = n × τ(6) = 6 × 4

  Physical basis:
    24 = 8 × 3. The 3 comes from trichromacy (H-DA-1).
    The 8 comes from byte architecture (H-DA-2).
    So 24 is a derived quantity: biology × computing convention.

  Grade: EXACT
  24 = J₂(6) is a precise match, and 24-bit is THE dominant pixel
  format in computing. The multiple n=6 decompositions (σ×φ, n×τ)
  are notable. However, 24 = 8×3 is fully explained by its components.
```

---

### H-DA-4: CMYK — 4 Inks = τ(6)

> CMYK printing uses 4 inks: Cyan, Magenta, Yellow, Key (Black)

```
  CMYK:
    Subtractive color model for printing
    C, M, Y are complements of R, G, B
    K (Key/Black) added for practical reasons:
      - CMY mix produces muddy brown, not true black
      - Black ink is cheaper than 3-color mix
      - Better text rendering

  n=6 mapping:
    4 = τ(6) ✓

  BUT:
    CMY alone (3 inks) is the theoretical subtractive system.
    K was added for PRACTICAL reasons (cost, quality), not theory.
    Some systems use 6 inks (CMYKOG), 8 inks, or 12 inks.
    Hexachrome (Pantone) uses 6 inks — that would be n itself.

  Grade: CLOSE
  CMYK's 4 = τ(6) is exact and CMYK is the universal print standard.
  But K is an engineering addition, not a color-theoretic necessity.
```

---

### H-DA-5: 12 Semitones per Octave — σ(6) = 12

> Western music divides the octave into 12 equal semitones

```
  12-tone equal temperament (12-TET):
    Frequency ratio per semitone: 2^(1/12)
    Used globally since ~18th century
    Circle of fifths: 12 keys

  n=6 mapping:
    12 = σ(6) ← EXACT

  Physical basis:
    12-TET approximates simple frequency ratios:
      Perfect fifth: 2^(7/12) = 1.4983 ≈ 3/2 = 1.5 (0.1% off)
      Perfect fourth: 2^(5/12) = 1.3348 ≈ 4/3 = 1.333 (0.1% off)
      Major third: 2^(4/12) = 1.2599 ≈ 5/4 = 1.25 (0.8% off)
    12 is chosen because it simultaneously approximates
    ratios involving 2, 3, and 5 — the first 3 primes.

  Why 12 specifically:
    12 = lcm(2,3,4) = lcm(3,4)
    12 has divisors {1,2,3,4,6,12} — the richest for its size
    This is exactly σ(6) = 1+2+3+6 = 12, and 12 is a
    highly composite number (more divisors than any smaller n).

  Grade: EXACT
  12 = σ(6) is exact. The deep reason 12 was chosen — maximal
  divisibility enabling many key signatures — is genuinely related
  to the divisor-sum structure that defines σ. Strong match.
```

---

### H-DA-6: 60Hz Refresh Rate — σ(6) × sopfr(6) = 60

> Standard display refresh = 60 Hz (NTSC legacy)

```
  60 Hz history:
    NTSC (1941): locked to US AC mains frequency 60 Hz
    European PAL: 50 Hz (locked to 50 Hz mains)
    Modern displays: 60/120/144/240 Hz (60 as base)

  n=6 mapping:
    60 = σ(6) × sopfr(6) = 12 × 5 ✓
    Also: 60 = 10n = (sopfr+n-1)×n? (forced)

  Physical basis:
    60 Hz comes from US electrical grid frequency.
    US chose 60 Hz AC because of generator pole/speed optimization
    in the 1880s-1890s. Europe chose 50 Hz for similar reasons.
    The display standard simply inherited the power frequency.

  BUT:
    60 = 2² × 3 × 5 is a highly composite number (sexagesimal base).
    Its use in time (60 seconds/minute) predates electricity by
    millennia (Babylonian base-60). The "60-ness" is ancient.

  Grade: CLOSE
  60 = σ × sopfr is numerically exact and 60Hz is universal.
  But 60Hz was inherited from the electrical grid, which was
  chosen for generator engineering, not display science.
  The deeper question is why 60 recurs in human systems — this
  relates to 60's divisibility, which connects to σ(6)=12 structure.
```

---

## Category B: Display Resolution and Standards

---

### H-DA-7: 4K Resolution — 3840/640 = 6, 2160/360 = 6

> 4K UHD: 3840 × 2160 pixels, both dimensions divisible by 6

```
  4K UHD (ITU-R BT.2020):
    3840 × 2160 = 8,294,400 pixels
    3840 = 6 × 640 = 6 × 2⁷ × 5
    2160 = 6 × 360 = 6 × 2³ × 3² × 5

  n=6 mapping:
    Both dimensions are exact multiples of 6 ✓
    Aspect ratio: 16:9 = (4n-8):(n+3)? → forced

  History:
    4K = 4 × 1080p horizontal lines? No.
    3840 = 2 × 1920 (Full HD), 2160 = 2 × 1080
    Full HD (1920×1080) → 1920 = 6 × 320, 1080 = 6 × 180

  BUT:
    Almost any resolution is divisible by 6 because video
    standards require divisibility by 2 (interlacing), 8 (DCT blocks),
    and 16 (macroblocks). 16 = 2⁴, so any resolution divisible by
    16 is automatically divisible by 2. Being divisible by both
    16 and 3 (common for 4:3/16:9 ratios) guarantees ÷6.

  Grade: WEAK
  Divisibility by 6 is real but is a side-effect of compression
  block sizes (8×8, 16×16) and aspect ratio arithmetic.
  Not specific to n=6.
```

---

### H-DA-8: 1080p — 1080/6 = 180

> Full HD vertical resolution: 1080 = 6 × 180

```
  1080p:
    1920 × 1080, introduced by ATSC (1998)
    1080 = 6 × 180 = 2³ × 3³ × 5

  Why 1080:
    CRT-era: 1080 = 1125 total lines - 45 blanking lines
    1125-line system proposed by NHK (Japan) in 1970s
    1080 active lines chosen for 16:9 with 1920 horizontal
    1920/1080 = 16/9 exactly

  n=6 mapping:
    1080 = 6 × 180 ✓
    1920 = 6 × 320 ✓

  BUT: same argument as H-DA-7. Divisibility by 6 is
  engineered into video standards for block-based compression.

  Grade: WEAK
  Same structural reason as H-DA-7. Not independently meaningful.
```

---

### H-DA-9: NTSC 29.97 fps ≈ 30 = 5n

> NTSC frame rate: nominally 30 fps, actually 29.97

```
  NTSC frame rate:
    Original (1941): exactly 30 fps = 60 fields / 2 (interlaced)
    Color NTSC (1953): 30 × 1000/1001 = 29.97002997... fps
    The 0.1% reduction avoided color subcarrier interference

  n=6 mapping:
    30 = 5n = sopfr(6) × n ← nominally exact
    29.97 ≈ 30 (0.1% off)

  Physical basis:
    30 fps = 60 Hz / 2 (interlaced scanning)
    60 Hz comes from AC mains (see H-DA-6)
    So 30 = 60/2 is a derived quantity

  Grade: CLOSE
  30 = 5n is exact for the original NTSC. The 0.1% deviation
  to 29.97 has a well-understood technical cause. But 30 is
  derived from 60/2, not independently chosen.
```

---

### H-DA-10: Cinema 24 fps — J₂(6) = 24

> Motion picture standard: 24 frames per second

```
  Cinema 24 fps:
    Established by SMPTE in 1927 (standardizing ~1920s practice)
    Chosen as minimum for acceptable motion perception
    Persists today: DCI, Blu-ray, streaming all support 24p

  History:
    Silent films: 16-18 fps (hand-cranked, variable)
    Sound films needed constant speed for audio sync
    24 fps chosen: fast enough for flicker-free projection
    (with 2-blade or 3-blade shutter: 48 or 72 flashes/sec)

  n=6 mapping:
    24 = J₂(6) ← EXACT

  BUT:
    Why 24 specifically?
    - 24 fps × 2-blade shutter = 48 flashes > flicker threshold (~45)
    - Film consumption: 24 fps = 90 feet/min (35mm)
    - Economical: 24 is the minimum that works
    - 24 is also divisible by 2,3,4,6,8,12 — useful for pulldown

  Grade: EXACT
  24 = J₂(6) is precise. Cinema 24fps is the most enduring media
  standard (nearly 100 years). The choice was driven by perceptual
  thresholds and economics, but the specific number 24 is notable.
```

---

### H-DA-11: PAL 25 fps — sopfr(6)² = 25

> PAL/SECAM: 25 fps (50Hz mains / 2)

```
  PAL frame rate:
    25 fps = 50 fields/s / 2 (interlaced)
    50 Hz from European AC mains

  n=6 mapping:
    25 = sopfr(6)² = 5² = 25 ✓

  BUT:
    25 = 50/2. The relevant question is why Europe uses 50 Hz,
    which was a generator engineering choice (different from US 60 Hz).
    sopfr(6)² is a contrived expression — squaring sopfr is not
    a natural n=6 operation.

  Grade: WEAK
  Numerically matches sopfr², but the formula is forced and
  25fps is derived from 50Hz mains, not display science.
```

---

### H-DA-12: 16:9 Aspect Ratio — (σ+τ):(σ-n/φ)?

> Modern widescreen standard: 16:9

```
  16:9 aspect ratio:
    SMPTE (1984), adopted by ATSC, DVB, all modern displays
    Geometric mean of 4:3 and CinemaScope 2.35:1
    Kerns Powers' compromise: √(4/3 × 2.35/1) ≈ √3.13 ≈ 1.77
    16/9 = 1.778 ≈ √(4/3 × 2.39) — close to geometric mean

  n=6 mapping attempts:
    16 = σ + τ = 12 + 4? (forced)
    9 = σ - n/φ = 12 - 3 = 9? (forced)
    16/9 = 1.778 ≈ σ/n - φ/σ? → no clean expression

  Grade: FAIL
  16:9 was chosen as geometric mean of existing standards.
  No natural n=6 expression yields 16/9. All mappings are
  arbitrary combinations of functions.
```

---

## Category C: Audio Standards

---

### H-DA-13: 48 kHz Audio Sample Rate — σ(6) × τ(6) × 1000

> Professional audio standard: 48,000 samples/sec

```
  48 kHz:
    AES/EBU standard for professional audio, broadcast, film
    48000 = 48 × 1000
    Nyquist: captures frequencies up to 24 kHz (above human limit ~20kHz)

  n=6 mapping:
    48 = σ(6) × τ(6) = 12 × 4 ← EXACT
    Also: 48 = 2 × J₂(6) = 2 × 24

  History:
    Why 48 kHz (not 44.1 kHz or 50 kHz)?
    - 48000 = 2⁵ × 3 × 5³ → highly factorable
    - Simple integer relationship to video rates:
      48000/24 = 2000 samples/frame (cinema)
      48000/30 = 1600 samples/frame (NTSC)
      48000/25 = 1920 samples/frame (PAL)

  Grade: EXACT
  48 = σ × τ is precise. 48 kHz is the professional audio standard,
  chosen for factorability and video synchronization. The rich
  factorization of 48 connects to σ(6) and τ(6).
```

---

### H-DA-14: 44.1 kHz CD Audio — Near Miss

> CD audio: 44,100 Hz (Red Book, 1980)

```
  44.1 kHz:
    44100 = 2² × 3² × 5² × 7²
    Origin: 44100 = 60 × 735 = 50 × 882
    This ensures integer samples per video frame for both NTSC and PAL
    (used PCM adapters that recorded on video equipment)

  n=6 mapping attempts:
    44100 / 6 = 7350 (integer, but unremarkable)
    44.1 ≈ ? → no clean n=6 expression

  Grade: FAIL
  44.1 kHz has a clear engineering origin (video compatibility)
  and no meaningful n=6 connection.
```

---

### H-DA-15: 96 kHz High-Res Audio — 2 × 48 = 2σ·τ

> High-resolution audio: 96 kHz

```
  96 kHz:
    DVD-Audio, professional recording standard
    96000 = 2 × 48000

  n=6 mapping:
    96 = 2 × 48 = φ(6) × σ(6) × τ(6) = 2 × 12 × 4

  BUT:
    96 = 2 × 48 is just doubling the base rate.
    The fundamental standard is 48 kHz (H-DA-13).
    Doubling is the most obvious extension.

  Grade: WEAK
  Derived from 48 kHz by factor of 2. Not independently meaningful.
```

---

### H-DA-16: 16-Bit Audio — σ + τ = 16

> CD audio bit depth: 16 bits (65,536 levels, ~96 dB dynamic range)

```
  16-bit audio:
    Red Book CD (1980): 16-bit PCM
    Dynamic range: 20 × log10(2^16) = 96.3 dB
    Sufficient for most listening environments

  n=6 mapping:
    16 = σ(6) + τ(6) = 12 + 4 ✓

  BUT:
    16 = 2⁴ is a power of 2 — the natural choice in binary computing.
    8-bit (too noisy), 32-bit (overkill for 1980), 16-bit was the
    practical sweet spot. Also matches 16-bit CPU word size (Intel 8086).
    Professional audio uses 24-bit (= J₂(6), more interesting match).

  Grade: WEAK
  16 = σ + τ is numerically correct but 16-bit is chosen as a
  power of 2, not because of any divisor-sum relationship.
```

---

### H-DA-17: 24-Bit Audio — J₂(6) = 24

> Professional audio bit depth: 24 bits (144 dB dynamic range)

```
  24-bit audio:
    Studio recording standard since ~2000
    Dynamic range: 20 × log10(2^24) = 144.5 dB
    Exceeds human auditory range (~120 dB pain threshold)

  n=6 mapping:
    24 = J₂(6) ← EXACT

  Context:
    24-bit appears in BOTH display (H-DA-3) and audio (this).
    True color = 24-bit, professional audio = 24-bit.
    This convergence around J₂(6) = 24 is notable.

  BUT:
    24 = 3 × 8 = 3 bytes. It's the natural "3 byte" format.
    In audio: 24 is between 16 (not enough headroom) and 32 (wasteful).
    The choice is engineering pragmatism.

  Grade: EXACT
  24 = J₂(6). The convergence of display and audio on 24-bit is
  striking. Both are THE professional standard in their domains.
```

---

### H-DA-18: MIDI — 128 Values = 2^7

> MIDI velocity, note number, CC: 0-127 (7-bit)

```
  MIDI (1983):
    Note numbers: 0-127 (128 notes)
    Velocity: 0-127
    Control change: 0-127
    7-bit data words in 8-bit serial protocol

  n=6 mapping attempts:
    128 = 2^7, 7 = σ(6) - sopfr(6)? (forced)
    MIDI channels: 16 = σ + τ? (same issue as H-DA-16)

  Grade: FAIL
  128 = 2^7 is a pure binary computing choice. MIDI uses 7-bit
  data because the MSB is reserved as status/data flag.
  No n=6 connection.
```

---

## Category D: Video Compression and Codecs

---

### H-DA-19: H.264 4×4 Transform — τ(6) = 4

> H.264/AVC uses 4×4 integer DCT as base transform

```
  H.264 transform:
    Primary: 4×4 integer DCT (replacing MPEG-2's 8×8 float DCT)
    Also supports 8×8 in High Profile
    4×4 chosen for better adaptation to local features

  n=6 mapping:
    4 = τ(6) ✓

  BUT:
    4×4 was chosen because:
    - Smaller blocks reduce ringing at edges
    - Integer transform avoids encoder-decoder mismatch
    - 4 = 2² is the minimum power-of-2 block that gives
      useful frequency decomposition
    H.265/HEVC uses 4×4, 8×8, 16×16, 32×32 — variable sizes.

  Grade: WEAK
  4×4 = τ(6)² is numerically matching but driven by signal processing
  tradeoffs, not number theory.
```

---

### H-DA-20: H.265/HEVC CTU — 64×64 Max Block

> HEVC coding tree unit: up to 64×64 pixels

```
  HEVC CTU:
    Maximum: 64×64 pixels
    Quadtree subdivision: 64→32→16→8 (4 levels)
    4 subdivision levels = τ(6)?

  n=6 mapping:
    64 = 2^6 → exponent 6 = n ✓
    4 subdivision levels = τ(6) ✓

  BUT:
    64 = 2^6 is a power of 2. The exponent happens to be 6.
    AV1 uses 128×128 (= 2^7). VP9 uses 64×64.
    Powers of 2 are universal in block-based codecs.
    The "6" in 2^6 is coincidental.

  Grade: WEAK
  2^n is a coincidence, not a structural connection.
```

---

### H-DA-21: GOP Structure — Typical I-frame Interval

> Common GOP: 12 frames between I-frames

```
  Group of Pictures (GOP):
    Typical broadcast GOP = 12 frames (0.5s at 24fps)
    Also common: 15, 30, 60 (varies by application)
    12 is popular for 24fps (every 0.5s) and divisibility

  n=6 mapping:
    12 = σ(6) ✓

  BUT:
    GOP length is configurable, not standardized.
    12 is popular because it divides evenly into 24 and 60.
    Other common values (15, 30, 250 for streaming) weaken
    the significance.

  Grade: WEAK
  12-frame GOP is common but not universal. Configurable parameter.
```

---

### H-DA-22: 3 Frame Types — I, P, B = n/φ

> Video compression uses 3 frame types: Intra, Predicted, Bidirectional

```
  Frame types:
    I-frame: independently coded (keyframe)
    P-frame: forward predicted from previous reference
    B-frame: bidirectionally predicted from past + future

  n=6 mapping:
    3 = n/φ(6) = 6/2 ✓

  Physical basis:
    Why 3 types? Temporal prediction has 3 natural modes:
    none (I), forward (P), forward+backward (B).
    This is the complete set for 1D temporal prediction.

  BUT:
    Modern codecs add more types. H.265 has multiple I subtypes
    (CRA, IDR, BLA). AV1 has S-frames (switch frames).
    The "3 basic types" is a pedagogical simplification.

  Grade: WEAK
  3 = n/φ matches, but "none/forward/bidirectional" is the natural
  trichotomy of temporal reference, not n=6-specific.
```

---

## Category E: Color Science

---

### H-DA-23: CIE XYZ — 3 Tristimulus Values

> CIE 1931: 3 tristimulus values (X, Y, Z) define a color

```
  CIE XYZ:
    X, Y, Z derived from human color matching functions
    3 values because of trichromatic vision (same as H-DA-1)

  n=6 mapping:
    3 = n/φ(6) ✓

  Grade: WEAK
  Duplicate of H-DA-1 in a different notation. Trichromacy
  is the single underlying fact; XYZ and RGB are both consequences.
```

---

### H-DA-24: sRGB Gamma ≈ 2.2

> sRGB effective gamma: ~2.2 (actual: piecewise with linear segment)

```
  sRGB transfer function:
    Overall gamma ≈ 2.2
    Actual: linear below 0.0031308, then (1.055×L^(1/2.4) - 0.055)
    The exponent 2.4 (not 2.2) is used in the power segment

  n=6 mapping:
    2.2 ≈ φ(6) + 0.2? → no clean expression
    2.4 ≈ ? → no match

  Grade: FAIL
  Gamma 2.2 approximates CRT phosphor response (actually ~2.35-2.5).
  sRGB standardized 2.2 as a perceptual compromise.
  No n=6 connection.
```

---

### H-DA-25: DCI-P3 White Point — D65 (6504K)

> DCI-P3 and sRGB white point: D65 illuminant, ~6504K

```
  D65 illuminant:
    Correlated color temperature: 6504 K
    Represents average daylight
    Used as white point for sRGB, DCI-P3, Rec.709, Rec.2020

  n=6 mapping:
    6504 ≈ 6500 → 6500/6 = 1083.3? (not clean)
    But: D65 = D + 65 → 65 = ? no match

  Alternative: D65 ≈ 6500K, and 6 × 1000 = 6000K is close (8% off)

  Grade: FAIL
  6504K is an empirical measurement of average daylight.
  It happens to be near 6500, but this is coincidence.
  D50 (5000K) is used for printing — no consistency.
```

---

### H-DA-26: HDR — 10-Bit Depth = sopfr(6) × φ(6)

> HDR10 standard: 10 bits per channel

```
  HDR10:
    10-bit color: 1024 levels per channel
    PQ (Perceptual Quantizer) transfer function
    4× the levels of 8-bit SDR

  n=6 mapping:
    10 = sopfr(6) × φ(6) = 5 × 2 ✓
    Also: 10 = n + τ(6) = 6 + 4

  BUT:
    10 = 2 + 8 = byte + 2 extra bits. 10-bit is the minimum
    increment above 8-bit that provides visually significant
    improvement (4× gradient resolution). Also 10-bit aligns
    with 5:1 DPCM coding efficiency.
    Dolby Vision uses 12-bit (= σ(6), interesting).

  Grade: WEAK
  10 = sopfr × φ works numerically but 10-bit is an engineering
  increment. The Dolby Vision 12-bit = σ(6) match is more notable
  but belongs to a different standard.
```

---

### H-DA-27: Dolby Vision 12-Bit — σ(6) = 12

> Dolby Vision: 12-bit color depth

```
  Dolby Vision:
    12-bit per channel: 4096 levels
    Dynamic metadata per scene
    Premium HDR format

  n=6 mapping:
    12 = σ(6) ← EXACT

  BUT:
    12 = 8 + 4 = 1.5 bytes. It's the next standard increment
    above 10-bit. Also 12 bits is common in professional video
    (cinema cameras: ARRI, RED shoot 12-16 bit RAW).
    12 appears as a color depth because it's a multiple of 4
    (nibble-aligned), not because of σ(6).

  Grade: CLOSE
  12 = σ(6) is exact and Dolby Vision is the premium HDR standard.
  But 12-bit is one of several common depths (8, 10, 12, 14, 16)
  and its choice is engineering-driven.
```

---

## Category F: Human Perception

---

### H-DA-28: Audible Range — ~20 Hz to 20 kHz (3 Decades)

> Human hearing spans ~3 decades of frequency (20-20000 Hz)

```
  Audible range:
    Low: ~20 Hz (varies, some hear 16 Hz)
    High: ~20,000 Hz (decreases with age)
    Range: ~10 octaves, or ~3 decades (10^1.3 to 10^4.3)

  n=6 mapping:
    3 decades ≈ n/φ(6) = 3 ✓
    10 octaves → see H-DA-5

  BUT:
    "3 decades" is approximate. The range is really 20-20000,
    and log10(20000/20) = 3.0 exactly.
    However, individual variation is enormous (15-20000 Hz typical).
    The "3" here is a convenient approximation, not a precise standard.

  Grade: WEAK
  Approximately 3 decades, but biological variability undermines
  any precise numerical match.
```

---

### H-DA-29: Visible Spectrum — ~380-780 nm (~1 Octave)

> Human visible light spans roughly one octave (factor of ~2)

```
  Visible spectrum:
    Short wavelength: ~380 nm (violet)
    Long wavelength: ~780 nm (deep red)
    Ratio: 780/380 = 2.05 ≈ 2

  n=6 mapping:
    Factor of ~2 = φ(6) ✓

  Physical basis:
    The visible range is ~1 octave because:
    - Below 380 nm: UV is absorbed by lens/cornea
    - Above 780 nm: insufficient photon energy for rhodopsin
    The ~2:1 ratio is biologically determined.

  BUT:
    2.05 is approximate. The exact boundaries are gradual, not sharp.
    "Factor of 2" appears universally (octave in music, etc.).
    φ(6) = 2 is the simplest non-trivial integer.

  Grade: WEAK
  Approximate factor of 2 in a biological system with fuzzy boundaries.
  φ(6) = 2 is too simple to be meaningful.
```

---

### H-DA-30: 6 Primary Frequency Bands in Audio

> Audio engineering divides spectrum into 6 standard bands

```
  Standard audio frequency bands:
    1. Sub-bass:    20-60 Hz
    2. Bass:        60-250 Hz
    3. Low-mid:     250-500 Hz
    4. Mid:         500-2000 Hz
    5. Upper-mid:   2000-6000 Hz
    6. Treble:      6000-20000 Hz

  n=6 mapping:
    6 bands = n ← EXACT?

  BUT:
    This 6-band division is ONE of many conventions.
    Other common divisions:
    - 3 bands (low/mid/high) — used in basic EQ
    - 5 bands (graphic EQ standard)
    - 7 bands (ISO preferred)
    - 10 bands (standard graphic EQ)
    - 31 bands (1/3 octave professional)
    There is NO universally agreed "6 bands."
    The 6-band division is a pedagogical convenience.

  Grade: FAIL
  No standardized 6-band division exists. The number of bands
  is arbitrary and varies by application. Cherry-picking "6" from
  many possible divisions is post-hoc fitting.
```

---

## Grade Summary

| ID | Hypothesis | n=6 Expression | Grade |
|----|-----------|----------------|-------|
| H-DA-1 | RGB 3 primaries | n/φ = 3 | **CLOSE** |
| H-DA-2 | 8-bit color depth | σ-τ = 8 | **WEAK** |
| H-DA-3 | 24-bit true color | J₂ = 24 | **EXACT** |
| H-DA-4 | CMYK 4 inks | τ = 4 | **CLOSE** |
| H-DA-5 | 12 semitones/octave | σ = 12 | **EXACT** |
| H-DA-6 | 60Hz refresh | σ×sopfr = 60 | **CLOSE** |
| H-DA-7 | 4K resolution ÷6 | n divides both | **WEAK** |
| H-DA-8 | 1080p ÷6 | n divides both | **WEAK** |
| H-DA-9 | NTSC 29.97 ≈ 30 | 5n = 30 | **CLOSE** |
| H-DA-10 | Cinema 24fps | J₂ = 24 | **EXACT** |
| H-DA-11 | PAL 25fps | sopfr² = 25 | **WEAK** |
| H-DA-12 | 16:9 aspect ratio | -- | **FAIL** |
| H-DA-13 | 48kHz audio | σ×τ = 48 | **EXACT** |
| H-DA-14 | 44.1kHz CD audio | -- | **FAIL** |
| H-DA-15 | 96kHz hi-res | 2σ×τ = 96 | **WEAK** |
| H-DA-16 | 16-bit audio | σ+τ = 16 | **WEAK** |
| H-DA-17 | 24-bit audio | J₂ = 24 | **EXACT** |
| H-DA-18 | MIDI 128 values | -- | **FAIL** |
| H-DA-19 | H.264 4×4 transform | τ = 4 | **WEAK** |
| H-DA-20 | HEVC 64×64 CTU | 2^n = 64 | **WEAK** |
| H-DA-21 | GOP 12 frames | σ = 12 | **WEAK** |
| H-DA-22 | 3 frame types (I,P,B) | n/φ = 3 | **WEAK** |
| H-DA-23 | CIE XYZ 3 values | n/φ = 3 | **WEAK** |
| H-DA-24 | sRGB gamma 2.2 | -- | **FAIL** |
| H-DA-25 | D65 white point 6504K | -- | **FAIL** |
| H-DA-26 | HDR10 10-bit | sopfr×φ = 10 | **WEAK** |
| H-DA-27 | Dolby Vision 12-bit | σ = 12 | **CLOSE** |
| H-DA-28 | 3 decades audible | n/φ = 3 | **WEAK** |
| H-DA-29 | Visible ~1 octave | φ = 2 | **WEAK** |
| H-DA-30 | 6 audio bands | n = 6 | **FAIL** |

### Distribution

| Grade | Count | Pct |
|-------|-------|-----|
| EXACT | 5 | 16.7% |
| CLOSE | 5 | 16.7% |
| WEAK | 14 | 46.7% |
| FAIL | 6 | 20.0% |

**Non-failing: 24/30 (80%) — but only 5 EXACT (16.7%)**

**Honest assessment**: The EXACT matches (24-bit color, 12 semitones, 24fps cinema,
48kHz audio, 24-bit audio) are genuinely striking — especially the convergence of
multiple standards on J₂(6)=24. However, most matches exploit the fact that media
standards use highly composite numbers (12, 24, 48, 60) which naturally overlap with
n=6 arithmetic. Human-designed standards SHOULD correlate with perfect-number
arithmetic because both favor numbers with rich factorizations.
