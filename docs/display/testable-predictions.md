# HEXA-DISPLAY Testable Predictions — Display-Specific

> Split from docs/display-audio/testable-predictions.md
> Contains display-related predictions only.

---

## Source

Full combined predictions (28 total): [docs/display-audio/testable-predictions.md](../display-audio/testable-predictions.md)

## Display-Specific Predictions

### Tier 1: 즉시 검증 가능

| # | 예측 | n=6 수식 | 검증 방법 | 반증 조건 |
|---|------|---------|---------|----------|
| TP-DISP-1 | VVC 이후 코덱도 12-bit max 유지 | σ=12 | ITU-T 표준 | 14/16-bit 표준화 |
| TP-DISP-2 | 3DGS 후속도 SH degree 3 유지 | n/φ=3 | 논문 확인 | SH degree!=3이 SOTA |
| TP-DISP-3 | NeRF 후속도 L=10 PE 유지 | σ-φ=10 | 논문 확인 | L!=10이 SOTA |

### Tier 2: 단기 검증 (1-3년)

| # | 예측 | n=6 수식 | 검증 방법 | 반증 조건 |
|---|------|---------|---------|----------|
| TP-DISP-4 | 차세대 gaming monitor 표준 = 144Hz 또는 288Hz | σ²=144, σ²·φ=288 | VESA/제품 출시 | 360Hz가 표준화 |
| TP-DISP-5 | Apple Vision Pro 2 refresh = 120Hz | σ(σ-φ)=120 | Apple 발표 | 90Hz 유지 |
| TP-DISP-6 | HDMI 2.2 bandwidth = 96Gbps | 2·σ·τ=96 | HDMI Forum | !=96Gbps |
| TP-DISP-7 | 차세대 OLED bit depth = 12-bit native | σ=12 | 디스플레이 스펙 | 14-bit native |

### Tier 3: 중기 검증 (3-10년)

| # | 예측 | n=6 수식 | 검증 방법 | 반증 조건 |
|---|------|---------|---------|----------|
| TP-DISP-8 | microLED 서브픽셀 = 3색 (RGB) 유지 | n/φ=3 | 디스플레이 기술 | RGBW가 microLED 표준 |
| TP-DISP-9 | 차세대 holographic display FOV > 120° | σ(σ-φ)=120 최소 | 프로토타입 스펙 | 상용화 FOV < 60° |
| TP-DISP-10 | 뇌-컴퓨터 인터페이스 시각 채널 = 12 또는 24 | σ 또는 J₂ | BCI 논문 | 완전 다른 수 |
| TP-DISP-11 | Light field display angular resolution = 48 views 이상 | σ·τ=48 | LF 프로토타입 | < 24 views |

### Goal.md Predictions (from TP-DA-1~8)

| # | 예측 | n=6 수식 | Tier |
|---|------|---------|------|
| TP-DA-1 | σ=12-bit MicroLED 색 심도 우위 | σ=12 | Tier 1 |
| TP-DA-2 | σ²=144Hz 주사율 인지 한계 | σ²=144 | Tier 1 |
| TP-DA-5 | σ·(σ-φ)=120° 홀로그램 FOV 임계점 | σ·(σ-φ)=120 | Tier 3 |
| TP-DA-6 | n/φ=3 SH 차수 충분성 (3DGS) | n/φ=3 | Tier 1 |
