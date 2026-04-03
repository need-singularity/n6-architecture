# HEXA-DISPLAY Mk.III --- 뇌-디스플레이 인터페이스 + 홀로그램

**Evolution Checkpoint**: Mk.III (Mid-Term)
**Date**: 2026-04-03
**Status**: 설계 구상 --- 핵심 기술 연구 초기
**Feasibility**: 🔮 장기 실현가능 (2036~2056, 20~30년)
**Parent**: docs/display/evolution/
**Goal Doc**: docs/display/goal.md
**BT Basis**: BT-48, BT-71, BT-56 (LLM), BT-66 (Vision AI)

> Split from docs/display-audio/evolution/mk-3-mid-term.md (display-specific content)

---

## 1. Mk.III의 의미

Mk.II가 눈의 극한 자극이라면,
Mk.III은 뇌와 직접 소통하기 시작하는 전환점이다.

> **비침습 BCI(뇌-컴퓨터 인터페이스)로 EEG/fNIRS 피드백을 받아
> 디스플레이를 실시간 최적화하고, 완전 홀로그램 120° FOV 양산을 달성한다.**

---

## 2. Display Specs

| 파라미터 | 값 | n=6 표현 | 근거 |
|---------|---|---------|------|
| BCI 채널 | 6,912 | σ²·σ·τ = 6912 | 비침습 EEG |
| 홀로그램 해상도 | 6912K sub-px | σ²·σ·τ | 볼류메트릭 |
| 디스플레이 주사율 | 288 Hz | σ²·φ = 288 | 양산 기본 |
| 홀로그램 FOV | 120° | σ·(σ-φ) | 양산 달성 |
| 촉각 포인트 | 144 | σ² = 144 | 햅틱 피드백 |
| 체감각 축 | 6 DOF | n = 6 | 모션 플랫폼 |

See original: [docs/display-audio/evolution/mk-3-mid-term.md](../../display-audio/evolution/mk-3-mid-term.md)
