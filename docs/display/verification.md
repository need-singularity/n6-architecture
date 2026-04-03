# N6 Display Hypotheses -- Independent Verification

> Split from docs/display-audio/verification.md
> Contains display-specific verification results.

---

## Source

Full verification (combined display+audio): [docs/display-audio/verification.md](../display-audio/verification.md)

## Display-Specific Verification Summary

| ID | Hypothesis | n=6 수식 | Grade | 검증 코멘트 |
|----|-----------|---------|-------|-----------|
| H-DISP-1 (H-DA-5) | Cinema 24fps = J₂ | J₂(6)=24 | **EXACT** | SMPTE ST 2036, 1927~ 100년 불변 |
| H-DISP-2 (H-DA-6) | 24-bit true color = J₂ | J₂(6)=24 | **EXACT** | sRGB IEC 61966-2-1 웹/모니터 표준 |
| H-DISP-3 (H-DA-14) | Dolby Vision 12-bit = σ | σ(6)=12 | **CLOSE** | HDR 최고 표준, bit 래더 일부 |
| H-DISP-4 (H-DA-15) | RGB 3 primaries = n/φ | n/φ=3 | **CLOSE** | Young-Helmholtz, 생물학적 |
| H-DISP-5 (H-DA-16) | CMYK 4 inks = τ | τ(6)=4 | **CLOSE** | 인쇄 보편 표준 |
| H-DISP-6 (H-DA-17) | 60 Hz refresh = σ·sopfr | σ·sopfr=60 | **CLOSE** | 전원 주파수 유래 |
| H-DISP-7 (H-DA-18) | Cinema 48 flashes = σ·τ | σ·τ=48 | **CLOSE** | 2-blade shutter 유도량 |
| H-DISP-8 (H-DA-23) | NTSC 30 fps = sopfr·n | sopfr·n=30 | **CLOSE** | 60Hz/2 유도 |
| H-DISP-9 (H-DA-24) | GOP 12 frames = σ | σ=12 | **CLOSE** | 설정값, 비표준 |
| H-DISP-10 (H-DA-28) | Visible ~1 octave = φ | φ=2 | **CLOSE** | 근사적 |

**Display EXACT: 2/10 = 20%** (cross-domain H-DA-29 adds 1 more EXACT)

---

## Method

Each hypothesis checked against published standards (ITU-R, SMPTE, CIE, ISO),
engineering history, perceptual science literature, and 2024-2026 product data.

See full verification matrix: [full-verification-matrix.md](full-verification-matrix.md)
