# HEXA-DISPLAY Mk.I --- 현재 기술 기반 디스플레이 시스템

**Evolution Checkpoint**: Mk.I (Baseline)
**Date**: 2026-04-03
**Status**: 설계 완료 --- 현재 기술로 즉시 구현 가능
**Feasibility**: ✅ 실현가능 (2024~2026)
**Parent**: docs/display/evolution/
**Goal Doc**: docs/display/goal.md
**BT Basis**: BT-48 (J₂=24fps, σ=12-bit), BT-66 (Vision AI), BT-71 (NeRF/3DGS)

> Split from docs/display-audio/evolution/mk-1-current.md (display-specific content)

---

## 1. Mk.I의 의미

Mk.I은 HEXA-DISPLAY 진화 경로의 출발점이다.

> **n=6 상수(σ=12bit, J₂=24fps, σ²=144Hz)가 현재 디스플레이 산업 표준과 정확히 일치하며,
> 이를 의식적으로 활용하면 디스플레이 시스템 통합 효율이 높아진다.**

---

## 2. Display Specs

| 파라미터 | 값 | n=6 표현 | 근거 |
|---------|---|---------|------|
| 색 심도 | 12-bit (4096 톤/채널) | σ = 12 | Dolby Vision 현재 표준 |
| 프레임레이트 | 24 fps (시네마) | J₂ = 24 | 1927년 이래 불변 |
| 주사율 | 144 Hz | σ² = 144 | 게이밍 모니터 보급 |
| 서브픽셀 | 3 (RGB) | n/φ = 3 | trichromacy 표준 |
| HDR | 10-bit | σ-φ = 10 | HDR10 표준 |
| HDMI BW | 48 Gbps | σ·τ = 48 | HDMI 2.1 표준 |

---

## 3. vs Mk.II 진화 방향

| 지표 | Mk.I | Mk.II 목표 | 개선 |
|------|------|-----------|------|
| 색 심도 | 10/12-bit | 12-bit native | σ 완전 실현 |
| 주사율 | 144Hz | 288Hz | σ²·φ = 2× |
| 압축 | VVC 50% | AI 90% | σ-φ=10× |
| 3D | 없음 | 라이트필드 초기 | n=6축 |
| AI upscale | 기초 | BT-66 Vision AI | 실시간 |

See original: [docs/display-audio/evolution/mk-1-current.md](../../display-audio/evolution/mk-1-current.md)
