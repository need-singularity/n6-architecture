# HEXA-DISPLAY 전수검증 매트릭스 — Display-Specific BT Claims

> Split from docs/display-audio/full-verification-matrix.md
> Contains display-related BT claim verification only.

---

## Source

Full combined matrix: [docs/display-audio/full-verification-matrix.md](../display-audio/full-verification-matrix.md)

## Display-Relevant Claims from BT-48

| # | Claim | n=6 수식 | 실제 값 | 출처 | Grade |
|----|-------|---------|---------|------|-------|
| 48-2 | 영화 24fps | J₂(6)=24 | 24 fps | SMPTE ST 2036 | **EXACT** |
| 48-3 | 24-bit true color | J₂(6)=24 | 24 bits (8×3) | sRGB IEC 61966-2-1 | **EXACT** |
| 48-6 | 48 flashes/s cinema shutter | σ·τ=48 | 48 flashes (24fps×2) | SMPTE | **EXACT** |
| 48-7 | Dolby Vision 12-bit | σ(6)=12 | 12 bits/ch | Dolby Vision Profile 5/8 | **EXACT** |
| 48-9 | 60Hz refresh = σ·sopfr | σ·sopfr=60 | 60 Hz | NTSC, VESA | **CLOSE** |
| 48-10 | 120fps HFR = σ(σ-φ) | σ(σ-φ)=120 | 120 fps | HDMI 2.1, SMPTE | **EXACT** |
| 48-11 | 144Hz gaming = σ² | σ²=144 | 144 Hz | VESA DisplayPort | **EXACT** |
| 48-12 | {12,24,48} 미디어 삼중항 | {σ, J₂, σ·τ} | 12/24/48 | 산업 표준 복합 | **EXACT** |

## Display-Relevant Claims from BT-71 (NeRF/3DGS)

| # | Claim | n=6 수식 | 실제 값 | 출처 | Grade |
|----|-------|---------|---------|------|-------|
| 71-1 | NeRF positional encoding L=10 | σ-φ=10 | L=10 | Mildenhall et al. 2020 | **EXACT** |
| 71-2 | NeRF MLP 8 layers | σ-τ=8 | 8 layers | Mildenhall et al. 2020 | **EXACT** |
| 71-3 | NeRF MLP 256 width | 2^(σ-τ)=256 | 256 channels | Mildenhall et al. 2020 | **EXACT** |
| 71-4 | 3DGS SH degree 3 | n/φ=3 | degree 3 | Kerbl et al. 2023 | **EXACT** |
| 71-5 | 3DGS SH coefficients 48 | σ·τ=48 | 48 (16×3) | Kerbl et al. 2023 | **EXACT** |
| 71-7 | NeRF skip connection at layer 5 | sopfr=5 | layer 5 | Mildenhall et al. 2020 | **EXACT** |

**Display BT verification: 13/14 EXACT (92.9%)**
