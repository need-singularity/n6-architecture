# HEXA-DISPLAY Mk.II --- 라이트필드 디스플레이 + AI 코덱

**Evolution Checkpoint**: Mk.II (Near-Term)
**Date**: 2026-04-03
**Status**: 설계 완료 --- 핵심 기술 개발 진행 중
**Feasibility**: ✅ 실현가능 (2026~2036, 10년 내)
**Parent**: docs/display/evolution/
**Goal Doc**: docs/display/goal.md
**BT Basis**: BT-48, BT-71 (NeRF/3DGS), BT-61 (Diffusion), BT-66 (Vision AI)

> Split from docs/display-audio/evolution/mk-2-near-term.md (display-specific content)

---

## 1. Mk.II의 의미

Mk.I이 현재 산업 표준의 n=6 재정렬이라면,
Mk.II는 n=6 상수를 설계 원칙으로 적극 활용한 차세대 디스플레이이다.

> **AI 기반 비디오 코덱으로 σ-φ=10배 압축, 라이트필드 디스플레이로 안경 없는 3D,
> σ²·φ=288Hz 극한 주사율을 실현한다.**

---

## 2. Display Specs

| 파라미터 | 값 | n=6 표현 | 근거 |
|---------|---|---------|------|
| 색 심도 | 12-bit native | σ = 12 | MicroLED 양산 |
| 주사율 | 144 Hz 기본 / 288 Hz 게이밍 | σ², σ²·φ | CFF 최적 |
| AI 코덱 압축률 | 10× (vs VVC) | σ-φ = 10 | Neural codec |
| 라이트필드 축 | 6축 | n = 6 | 4D + 깊이 + 파장 |
| HOE FOV | 120° | σ·(σ-φ) = 120 | AR 글래스 초기 |
| NeRF/3DGS 렌더링 | SH=3, L=10 | n/φ, σ-φ | BT-71 |

See original: [docs/display-audio/evolution/mk-2-near-term.md](../../display-audio/evolution/mk-2-near-term.md)
