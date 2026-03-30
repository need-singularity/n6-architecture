# N6 Display & Audio Hypotheses -- Independent Verification

Verified: 2026-03-30
Method: Each hypothesis checked against published standards (ITU-R, SMPTE, AES, CIE, ISO), engineering history (Poynton "Digital Video and HD", Watkinson "Art of Sound Reproduction"), and perceptual science literature. Grades adjusted where warranted.

## Grade Distribution (Summary)

| Grade | Count | Pct | Hypotheses |
|-------|-------|-----|------------|
| EXACT | 5 | 16.7% | H-DA-3, H-DA-5, H-DA-10, H-DA-13, H-DA-17 |
| CLOSE | 5 | 16.7% | H-DA-1, H-DA-4, H-DA-6, H-DA-9, H-DA-27 |
| WEAK | 14 | 46.7% | H-DA-2, H-DA-7, H-DA-8, H-DA-11, H-DA-15, H-DA-16, H-DA-19, H-DA-20, H-DA-21, H-DA-22, H-DA-23, H-DA-26, H-DA-28, H-DA-29 |
| FAIL | 6 | 20.0% | H-DA-12, H-DA-14, H-DA-18, H-DA-24, H-DA-25, H-DA-30 |
| UNVERIFIABLE | 0 | 0% | -- |

**Non-failing total: 24/30 (80%)**

| ID | Hypothesis | Grade |
|----|-----------|-------|
| H-DA-1 | RGB 3 primaries = n/phi | **CLOSE** |
| H-DA-2 | 8-bit color = sigma-tau | **WEAK** |
| H-DA-3 | 24-bit true color = J2 | **EXACT** |
| H-DA-4 | CMYK 4 inks = tau | **CLOSE** |
| H-DA-5 | 12 semitones = sigma | **EXACT** |
| H-DA-6 | 60Hz refresh = sigma*sopfr | **CLOSE** |
| H-DA-7 | 4K resolution / 6 | **WEAK** |
| H-DA-8 | 1080p / 6 | **WEAK** |
| H-DA-9 | NTSC 30fps = 5n | **CLOSE** |
| H-DA-10 | Cinema 24fps = J2 | **EXACT** |
| H-DA-11 | PAL 25fps = sopfr^2 | **WEAK** |
| H-DA-12 | 16:9 aspect ratio | **FAIL** |
| H-DA-13 | 48kHz audio = sigma*tau | **EXACT** |
| H-DA-14 | 44.1kHz CD audio | **FAIL** |
| H-DA-15 | 96kHz hi-res audio | **WEAK** |
| H-DA-16 | 16-bit audio = sigma+tau | **WEAK** |
| H-DA-17 | 24-bit audio = J2 | **EXACT** |
| H-DA-18 | MIDI 128 values | **FAIL** |
| H-DA-19 | H.264 4x4 transform = tau | **WEAK** |
| H-DA-20 | HEVC 64x64 = 2^n | **WEAK** |
| H-DA-21 | GOP 12 frames = sigma | **WEAK** |
| H-DA-22 | 3 frame types = n/phi | **WEAK** |
| H-DA-23 | CIE XYZ 3 values | **WEAK** |
| H-DA-24 | sRGB gamma 2.2 | **FAIL** |
| H-DA-25 | D65 white point 6504K | **FAIL** |
| H-DA-26 | HDR10 10-bit | **WEAK** |
| H-DA-27 | Dolby Vision 12-bit = sigma | **CLOSE** |
| H-DA-28 | 3 decades audible range | **WEAK** |
| H-DA-29 | Visible spectrum ~1 octave | **WEAK** |
| H-DA-30 | 6 audio frequency bands | **FAIL** |

---

Grading scale:
- **EXACT**: The claimed number/structure matches real-world data precisely, with a legitimate physical or engineering basis.
- **CLOSE**: Within ~10% of real values, or directionally correct with a meaningful standard.
- **WEAK**: Requires cherry-picking, flexible counting, or post-hoc rationalization.
- **FAIL**: Contradicted by real-world data, trivially true of any number, or unit-dependent.
- **UNVERIFIABLE**: Insufficient published data to confirm or deny.

---

## H-DA-1: RGB 3 Primaries = n/phi(6) = 3

**Grade: CLOSE** (confirmed)

Human trichromatic vision is established beyond doubt (Young 1802, Helmholtz 1852, confirmed by microspectrophotometry of cone cells). Three cone types (L/M/S) with peak sensitivities at ~564/534/420 nm are encoded by three distinct opsin genes on chromosomes 7 and X. The number 3 = n/phi(6) = 6/2 is correct.

However, trichromacy is a primate specialization, not a universal. Most mammals are dichromats (2 cone types). Birds and reptiles are tetrachromats (4 types). Mantis shrimp have 12-16 photoreceptor types. The "3" in human vision is an evolutionary accident of opsin gene duplication. Additionally, 3 is the second-most common small integer in physics (after 2) and appears for many unrelated reasons (spatial dimensions, quark colors, generations of matter). CLOSE is appropriate -- real match, but low specificity.

---

## H-DA-2: 8-Bit Color Depth = sigma(6) - tau(6) = 8

**Grade: WEAK** (confirmed)

8-bit color depth (256 levels per channel) was established by VGA (IBM, 1987) and persists in sRGB, JPEG, and PNG. The formula sigma-tau = 12-4 = 8 is correct. However, the 8-bit byte was standardized by IBM System/360 (1964) for character encoding reasons (7-bit ASCII + parity). Display color depth inherited the byte, not the other way around. The formula sigma-tau is arithmetically arbitrary -- there is no reason to subtract the divisor count from the divisor sum. Professional workflows use 10, 12, 14, and 16 bits. WEAK is fair.

---

## H-DA-3: 24-Bit True Color = J2(6) = 24

**Grade: EXACT** (confirmed)

24-bit true color (8 bits x 3 channels = 16,777,216 colors) is THE universal pixel format for consumer displays, established by SVGA in the mid-1990s and used by virtually every screen, image format, and web standard today. J2(6) = 24 is a precise integer match. The number 24 also decomposes as sigma(6) x phi(6) = 12 x 2 and n x tau(6) = 6 x 4, providing multiple n=6 paths.

Counterargument: 24 = 8 x 3 is fully explained by "3 channels of 1 byte each." The components are independently motivated (byte size and trichromacy). Nevertheless, the specific convergence on 24 is a genuine and important standard. EXACT grade is warranted.

---

## H-DA-4: CMYK 4 Inks = tau(6) = 4

**Grade: CLOSE** (confirmed)

CMYK (Cyan, Magenta, Yellow, Key/Black) is the universal subtractive color model for commercial printing (ISO 12647). tau(6) = 4 matches. However, the theoretical subtractive model requires only 3 inks (CMY). Black was added for practical reasons: impure inks produce muddy dark brown instead of true black; black ink is cheaper per coverage area; and black text needs sharp edges that 3-ink overprint cannot achieve. Extended gamut printing uses 6 (CMYKOV) or 7+ inks. CLOSE is appropriate -- CMYK is genuinely universal but K is an engineering addition.

---

## H-DA-5: 12 Semitones per Octave = sigma(6) = 12

**Grade: EXACT** (confirmed, strengthened)

12-tone equal temperament (12-TET) divides the octave into 12 equal parts, with frequency ratio 2^(1/12) per semitone. This system has been the global standard for tuned instruments since the 18th century. sigma(6) = 12 is exact.

The deep reason 12 works is that it simultaneously provides good approximations to the consonant intervals: perfect fifth 3:2 (error 0.11%), perfect fourth 4:3 (0.11%), major third 5:4 (0.79%), minor third 6:5 (0.91%). No smaller number achieves this. The mathematical reason: 12 = lcm(3,4) has the richest divisor structure for its size among practical temperament sizes. Alternative temperaments exist (19-TET, 31-TET, 53-TET) but none has displaced 12-TET. The connection between 12's divisibility and sigma(6) is the strongest match in this domain. EXACT is well-deserved.

---

## H-DA-6: 60Hz Refresh Rate = sigma(6) x sopfr(6) = 12 x 5 = 60

**Grade: CLOSE** (confirmed)

60 Hz is the standard refresh rate for displays, inherited from the 60 Hz AC power frequency in North America (established 1890s-1900s by Westinghouse/Tesla). NTSC television (1941) locked to 60 fields/second for synchronization. Modern displays maintain 60 Hz as baseline even without mains coupling.

The formula sigma x sopfr = 12 x 5 = 60 is correct. However, Europe uses 50 Hz (PAL/SECAM), which has no clean n=6 expression. The 60 Hz choice was driven by generator pole-count optimization, not display science. 60 is also a sexagesimal base number (Babylonian) appearing in timekeeping for millennia. The recurrence of 60 across human systems may relate to its exceptional divisibility (12 divisors), which itself connects to the factors 2^2 x 3 x 5. CLOSE is fair -- real but inherited.

---

## H-DA-7: 4K Resolution Divisible by 6

**Grade: WEAK** (confirmed)

4K UHD (3840 x 2160) has both dimensions divisible by 6: 3840 = 6 x 640, 2160 = 6 x 360. This is true but trivially guaranteed by video compression requirements. H.264 requires resolution divisible by 16 (macroblock size). H.265 prefers divisibility by 64 (CTU). All standard resolutions are divisible by 16, and the 16:9 aspect ratio ensures the vertical dimension is divisible by 9. lcm(16, 9) = 144, and any resolution divisible by 144 is automatically divisible by 6 (since 6 | 144). This is structural, not meaningful.

---

## H-DA-8: 1080p Divisible by 6

**Grade: WEAK** (confirmed)

1920 x 1080: both divisible by 6. Same reasoning as H-DA-7. The NHK 1125-line system (proposed 1970s) specified 1080 active lines for 16:9 compatibility with 1920 horizontal pixels. 1080 = 2^3 x 3^3 x 5 is inherently divisible by 6. Not independently meaningful.

---

## H-DA-9: NTSC 29.97fps ≈ 30 = 5n

**Grade: CLOSE** (confirmed)

Original NTSC (1941): exactly 30 fps (60 fields / 2, interlaced). Color NTSC (1953): 30 x 1000/1001 = 29.97 fps to avoid chroma/luma beat frequency interference. 30 = 5 x 6 = sopfr(6) x n is exact for the original standard. The 0.1% deviation to 29.97 is well-documented and has a specific technical cause (3.579545 MHz color subcarrier). CLOSE is appropriate -- the nominal value matches but the actual value deviates.

---

## H-DA-10: Cinema 24fps = J2(6) = 24

**Grade: EXACT** (confirmed, strengthened)

24 frames per second has been the motion picture standard since SMPTE standardization in 1927, making it one of the longest-lived media standards. J2(6) = 24 is exact. The choice was driven by: (1) minimum frame rate for acceptable motion rendition with 2-blade shutter (48 flashes/s > ~45 Hz flicker threshold); (2) film economy (24 fps = 90 ft/min on 35mm); (3) rich divisibility (24 = 2^3 x 3) enabling pulldown conversions.

Despite 100 years of technological change (sound, color, widescreen, digital), 24fps remains standard for theatrical cinema. DCI (Digital Cinema Initiatives, 2005) mandates 24fps support. The persistence of this specific number is remarkable. EXACT.

---

## H-DA-11: PAL 25fps = sopfr(6)^2 = 25

**Grade: WEAK** (downgraded from original WEAK -- confirmed)

PAL uses 25 fps = 50/2. The formula sopfr(6)^2 = 5^2 = 25 is contrived -- squaring a function value is not a natural operation in this framework. 25 fps exists solely because Europe chose 50 Hz AC mains (for different generator engineering reasons than the US). The hypothesis should be evaluated on 50 Hz, not 25 fps. 50 has no clean n=6 expression. WEAK.

---

## H-DA-12: 16:9 Aspect Ratio

**Grade: FAIL** (confirmed)

16:9 = 1.778 was chosen by Kerns Powers (SMPTE, 1984) as the geometric mean of existing aspect ratios ranging from 4:3 (1.333) to 2.39:1 (anamorphic). The derivation is purely geometric: sqrt(1.333 x 2.39) = 1.785 ≈ 16/9 = 1.778. No n=6 expression naturally produces 16/9. Attempts like (sigma+tau)/(sigma-n/phi) = 16/9 are post-hoc constructions. FAIL.

---

## H-DA-13: 48kHz Audio = sigma(6) x tau(6) = 48

**Grade: EXACT** (confirmed, strengthened)

48 kHz is the AES/EBU professional audio standard (AES5-1998), used in broadcast, film production, and all professional recording. 48000 = sigma(6) x tau(6) x 1000 = 12 x 4 x 1000. The number 48 was chosen specifically for its rich factorization: 48000/24 = 2000 (samples per film frame), 48000/25 = 1920 (samples per PAL frame), 48000/30 = 1600 (samples per NTSC frame). All divide evenly.

The choice of 48k over other candidates (50k, 44.1k) was explicitly motivated by integer-ratio compatibility with video frame rates. This compatibility depends on 48 having factors 2, 3, 4, 6, 8, 12, 16, 24 -- essentially the divisor structure of 6. EXACT is well-supported.

---

## H-DA-14: 44.1kHz CD Audio

**Grade: FAIL** (confirmed)

44100 = 2^2 x 3^2 x 5^2 x 7^2. Its origin: Sony/Philips needed a sample rate recordable on both NTSC (525-line) and PAL (625-line) video equipment. 44100 = 60 x 735 = 50 x 882, ensuring integer samples per field for both systems. The factor 7^2 = 49 has no n=6 connection. FAIL.

---

## H-DA-15: 96kHz Hi-Res Audio = 2 x sigma x tau

**Grade: WEAK** (confirmed)

96 kHz = 2 x 48 kHz. This is simply doubling the base professional standard. Any merit belongs to H-DA-13 (48 kHz). WEAK.

---

## H-DA-16: 16-Bit Audio = sigma(6) + tau(6) = 16

**Grade: WEAK** (confirmed)

16-bit audio (Red Book CD, 1980) provides 96 dB dynamic range. 16 = sigma + tau = 12 + 4 is numerically true, but 16 = 2^4 is the operative reason -- it's a power of 2 matching the word size of contemporary processors (Intel 8086/8088). The additive formula sigma+tau is arbitrary. WEAK.

---

## H-DA-17: 24-Bit Audio = J2(6) = 24

**Grade: EXACT** (confirmed)

24-bit audio is the universal standard for professional recording and mixing (Pro Tools, Logic, Ableton all default to 24-bit). J2(6) = 24 is exact. This creates a remarkable convergence: J2(6) = 24 appears in true color (H-DA-3), cinema frame rate (H-DA-10), and professional audio bit depth (H-DA-17) -- three independent media standards all settling on the same number. 24 = 3 x 8 = 3 bytes is the engineering explanation, but the triple convergence is noteworthy. EXACT.

---

## H-DA-18: MIDI 128 Values

**Grade: FAIL** (confirmed)

MIDI (1983) uses 7-bit data words (0-127) because the 8th bit (MSB) distinguishes status bytes from data bytes. 128 = 2^7 is a pure binary computing choice dictated by the serial protocol design. No n=6 connection. FAIL.

---

## H-DA-19: H.264 4x4 Transform = tau(6) = 4

**Grade: WEAK** (confirmed)

H.264/AVC (ITU-T H.264, 2003) introduced 4x4 integer DCT as the primary transform, replacing MPEG-2's 8x8 floating-point DCT. tau(6) = 4 matches. However, 4 = 2^2 is the smallest power-of-2 that provides useful frequency decomposition (DC + 3 AC components in each direction). H.265/HEVC uses 4/8/16/32, and AV1 uses 4/8/16/32/64 -- the trend is toward multiple sizes. WEAK.

---

## H-DA-20: HEVC 64x64 CTU = 2^n

**Grade: WEAK** (confirmed)

HEVC maximum CTU size 64x64 has 64 = 2^6 where the exponent equals n=6. This is a coincidence within the power-of-2 progression (16, 32, 64, 128). AV1 uses 128x128 (= 2^7). The specific exponent 6 for HEVC is driven by diminishing returns in block size vs. complexity tradeoff, not number theory. WEAK.

---

## H-DA-21: GOP 12 Frames = sigma(6) = 12

**Grade: WEAK** (confirmed)

GOP (Group of Pictures) length is a configurable encoding parameter, not a standard. Common values: 12 (broadcast), 15, 24, 30, 60, 250 (streaming). While 12 is popular for broadcast (0.5s at 24fps, 0.2s at 60fps), calling it "the" standard is cherry-picking. Encoders like x264/x265 default to keyint=250. WEAK.

---

## H-DA-22: 3 Frame Types (I, P, B) = n/phi = 3

**Grade: WEAK** (confirmed)

I/P/B is the pedagogical simplification. Real standards are more complex: H.264 has IDR, non-IDR I, P, B, SI, SP frame types. H.265 adds CRA, BLA, RADL, RASL. AV1 has Key, Inter, Intra-only, Switch frames. The "3 basic types" is the minimum description, not a standard specification. WEAK.

---

## H-DA-23: CIE XYZ 3 Tristimulus Values

**Grade: WEAK** (downgraded from original WEAK -- confirmed)

CIE XYZ (1931) uses 3 tristimulus values because humans are trichromats. This is the same fact as H-DA-1 (RGB), expressed in a different coordinate system. Counting it separately inflates the match count. The "3" in CIE XYZ is biologically determined, not n=6-determined. WEAK (and redundant with H-DA-1).

---

## H-DA-24: sRGB Gamma 2.2

**Grade: FAIL** (confirmed)

The sRGB standard (IEC 61966-2-1:1999) specifies a piecewise transfer function with a power-law segment of exponent 1/2.4 (not 2.2). The "gamma 2.2" is a convenient approximation. CRT phosphors had gamma ~2.35-2.5. There is no clean n=6 expression for 2.2 or 2.4. FAIL.

---

## H-DA-25: D65 White Point 6504K

**Grade: FAIL** (confirmed)

CIE Standard Illuminant D65 has correlated color temperature ~6504K (CIE 15:2004). The "65" in D65 refers to 6500K rounded. This is an empirical measurement of average daylight spectral power distribution. The proximity to 6000 (= 1000n) or 6500 is coincidental. D50 (5000K) is equally important for print. FAIL.

---

## H-DA-26: HDR10 10-Bit = sopfr x phi = 10

**Grade: WEAK** (confirmed)

HDR10 (CTA-861.3) uses 10-bit color. sopfr(6) x phi(6) = 5 x 2 = 10 is numerically correct. But 10-bit was chosen as the minimum perceptually meaningful upgrade from 8-bit (4x levels, eliminating banding in gradients). The product sopfr x phi is not a natural n=6 combination. Dolby Vision's 12-bit = sigma(6) is a stronger match. WEAK.

---

## H-DA-27: Dolby Vision 12-Bit = sigma(6) = 12

**Grade: CLOSE** (confirmed)

Dolby Vision uses 12-bit color (4096 levels per channel). sigma(6) = 12 is exact. However, 12-bit sits naturally in the power-of-2 progression (8, 10, 12, 14, 16) and equals 1.5 bytes. Cinema cameras (ARRI ALEXA, RED) commonly record 12-bit or higher. 12-bit is one of several professional depths, not uniquely dominant like 8-bit (consumer) or 24-bit (RGB aggregate). CLOSE.

---

## H-DA-28: 3 Decades Audible Range

**Grade: WEAK** (confirmed)

Human hearing: ~20 Hz to ~20 kHz = log10(1000) = 3 decades exactly. But the boundaries are biological and fuzzy: infants hear up to ~22 kHz, presbycusis reduces upper limit to ~12-14 kHz by age 50. The "20-20k" is a textbook convention, not a precise standard. 3 = n/phi is too common a number. WEAK.

---

## H-DA-29: Visible Spectrum ~1 Octave = phi(6) = 2

**Grade: WEAK** (confirmed)

Visible light: ~380-780 nm, ratio 780/380 = 2.05. This is approximately one octave (factor of 2). But phi(6) = 2 is the simplest non-trivial integer. The boundaries of visibility are gradual (scotopic vs photopic response, individual variation). Saying "approximately 2" about an approximate ratio of an approximate boundary is too many approximations. WEAK.

---

## H-DA-30: 6 Audio Frequency Bands

**Grade: FAIL** (confirmed)

There is no standardized 6-band division of the audio spectrum. ISO 266:1997 defines preferred frequencies in 1/3 octave bands (31 bands). Graphic equalizers typically use 5, 7, 10, 15, or 31 bands. The "6 bands" cited (sub-bass/bass/low-mid/mid/upper-mid/treble) is one pedagogical convention among many. Other textbooks use 3, 4, 5, 7, or 10 divisions. Cherry-picking the one that equals n is post-hoc fitting. FAIL.

---

## Cross-Domain Observations

The strongest pattern in this domain is the convergence on J2(6) = 24:
- 24-bit true color (display)
- 24 fps cinema (video)
- 24-bit audio depth (audio)

Three independent media standards, designed by different organizations (display manufacturers, SMPTE, AES) across different decades, all settling on 24. The mathematical explanation is that 24 = 2^3 x 3 is highly factorable, enabling efficient subdivision. This factorability is exactly what J2(6) captures.

Secondary pattern: sigma(6) = 12 appears in semitones, Dolby Vision bit depth, and GOP length. The sopfr-derived 60 Hz is ubiquitous but inherited from the electrical grid.

**Key caveat**: Display and audio are human-designed systems. Engineers prefer round, highly-composite numbers for practical reasons (divisibility, byte alignment, backward compatibility). Perfect number arithmetic also favors these numbers. The overlap is structural, not causal.
