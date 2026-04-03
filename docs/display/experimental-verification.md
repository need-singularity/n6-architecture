# HEXA-DISPLAY 실험검증 — Display Product Specs vs n=6 Predictions

> Split from docs/display-audio/experimental-verification.md
> Contains display-related experimental verification only.

---

## Source

Full combined verification: [docs/display-audio/experimental-verification.md](../display-audio/experimental-verification.md)

## Display Verification Results

### Samsung S95D 77" QD-OLED (2024)

| 파라미터 | n=6 예측 | 실측 | Match |
|---------|---------|------|-------|
| Peak refresh | σ²=144Hz | 144Hz | MATCH |
| Color depth | σ-φ=10 bit | 10-bit | MATCH |
| Subpixel | n/φ=3 (RGB) | RGB 3색 | MATCH |
| 4K 120fps | σ(σ-φ)=120 | 120Hz@4K | MATCH |
| HDMI 2.1 BW | σ·τ=48 Gbps | 48 Gbps | MATCH |

**S95D: 5/5 MATCH (100%)**

### LG G4 77" WOLED (2024)

| 파라미터 | n=6 예측 | 실측 | Match |
|---------|---------|------|-------|
| Peak refresh | σ(σ-φ)=120Hz | 120Hz | MATCH |
| Dolby Vision | σ=12 bit (signal) | 12-bit DV | MATCH |
| Subpixel | τ=4 (WRGB) | WRGB 4색 | MATCH |
| HDMI 2.1 BW | σ·τ=48 Gbps | 48 Gbps | MATCH |
| HDR10 | σ-φ=10 bit | 10-bit | MATCH |

**G4: 5/5 MATCH (100%)**

See original for complete product lineup verification (Apple, Sony, etc.).
