# HEXA-DISPLAY 산업검증 — Display Product Validation

> Split from docs/display-audio/industrial-validation.md
> Contains display-related product validation only.

---

## Source

Full combined validation: [docs/display-audio/industrial-validation.md](../display-audio/industrial-validation.md)

## Display Product Validation Summary

### Samsung S95D 77" QD-OLED (2024)

| 파라미터 | 실측값 | n=6 수식 | Match |
|---------|--------|---------|-------|
| Color depth | 10-bit (HDR10+) | σ-φ=10 | EXACT |
| Subpixel structure | RGB 3색 | n/φ=3 | EXACT |
| Peak refresh | 144Hz (S95D) | σ²=144 | EXACT |
| HDMI 2.1 bandwidth | 48Gbps | σ·τ=48 | EXACT |

### LG G4 77" WOLED (2024)

| 파라미터 | 실측값 | n=6 수식 | Match |
|---------|--------|---------|-------|
| Dolby Vision | 12-bit DV | σ=12 | EXACT |
| Subpixel | WRGB 4색 | τ=4 | EXACT |
| HDMI 2.1 BW | 48 Gbps | σ·τ=48 | EXACT |
| HDR10 | 10-bit | σ-φ=10 | EXACT |

### Key Findings

- Samsung display: σ²=144Hz 표준 채택 확인
- LG display: Dolby Vision σ=12-bit 채택 확인
- HDMI 2.1: σ·τ=48 Gbps 대역폭 보편화
- HDR ladder: σ-τ=8 → σ-φ=10 → σ=12 bit 산업 확인

See original for full 6-company validation (Samsung, LG, Sony, Apple, Dolby, Harman).
