# HEXA-DISPLAY 물리한계 증명 — Display Physical Limits

> Split from docs/display-audio/physical-limit-proof.md
> Contains display-related physical limit proofs only.

---

## Source

Full combined proof: [docs/display-audio/physical-limit-proof.md](../display-audio/physical-limit-proof.md)

## Display Physical Limits Summary

| # | 정리 | 한계 값 | n=6 표현 | 분류 |
|---|------|---------|---------|------|
| 2 | 인간 CFF 포화 | ~144Hz | σ²=144 | 신경과학 |
| 3 | 인간 색 심도 JND 포화 | 12-bit | σ=12 | 심리물리학 |
| 6 | 운동 인지 최소 | 24fps | J₂=24 | 신경과학 |
| 7 | Shannon 색 채널 | 3 원색 | n/φ=3 | 정보이론 |
| 10 | HDR 동적 범위 포화 | ~12 stop | σ=12 | 광학/신경 |

### Key Results

- **CFF (Critical Flicker Fusion)**: σ²=144Hz에서 인간 시각 포화 → 144Hz가 물리적 최적 주사율
- **JND Color Depth**: σ=12-bit에서 인간 색 구별 능력 포화 → 12-bit이 최종 수렴점
- **Motion Perception**: J₂=24fps가 연속 운동 인지의 최소 요구치
- **Trichromacy**: n/φ=3 원색이 인간 시각 정보 채널의 물리적 한계

See original for full proof derivations.
