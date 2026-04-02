# HEXA-DA Testable Predictions — 28 검증가능 예측

> Date: 2026-04-02
> Domain: Display-Audio
> Purpose: BT-48, BT-71, BT-72, BT-108, BT-76 기반 검증가능 예측
> Method: 기존 산업 표준 + 차세대 기술 트렌드에서 n=6 예측값 도출
> Falsifiability: 각 예측에 대해 반증 조건 명시

---

## Tier 1: 즉시 검증 가능 (Today, 공개 데이터)

| # | 예측 | n=6 수식 | 검증 방법 | 반증 조건 |
|---|------|---------|---------|----------|
| TP-DA-1 | 차세대 신경 코덱(DAC, Vocos 등)도 8 codebooks 유지 | σ-τ=8 | 논문 확인 | codebook≠8이 SOTA 달성 |
| TP-DA-2 | 차세대 신경 코덱 VQ entries = 1024 유지 | 2^(σ-φ)=1024 | 논문 확인 | entries≠1024가 SOTA |
| TP-DA-3 | NeRF 후속 (Zip-NeRF, Nerfacto)도 L=10 PE 유지 | σ-φ=10 | 논문 확인 | L≠10이 SOTA |
| TP-DA-4 | 3DGS 후속도 SH degree 3 유지 | n/φ=3 | 논문 확인 | SH degree≠3이 SOTA |
| TP-DA-5 | VVC 이후 코덱도 12-bit max 유지 | σ=12 | ITU-T 표준 | 14/16-bit 표준화 |
| TP-DA-6 | Dolby Atmos 12채널 기본 유지 (7.1.4) | σ=12 | Dolby 스펙 | 기본 채널≠12 |
| TP-DA-7 | AV2 (차세대 AOMedia) max block = 128 유지 | 2^(σ-sopfr)=128 | AOM 스펙 | max block≠128 |

---

## Tier 2: 단기 검증 (1-3년, 차세대 제품)

| # | 예측 | n=6 수식 | 검증 방법 | 반증 조건 |
|---|------|---------|---------|----------|
| TP-DA-8 | 차세대 gaming monitor 표준 = 144Hz 또는 288Hz | σ²=144, σ²·φ=288 | VESA/제품 출시 | 360Hz가 표준화 |
| TP-DA-9 | Apple Vision Pro 2 refresh = 120Hz | σ(σ-φ)=120 | Apple 발표 | 90Hz 유지 |
| TP-DA-10 | HDMI 2.2 bandwidth = 96Gbps | 2·σ·τ=96 | HDMI Forum | ≠96Gbps |
| TP-DA-11 | 차세대 OLED bit depth = 12-bit native | σ=12 | 디스플레이 스펙 | 14-bit native |
| TP-DA-12 | 차세대 Bluetooth audio codec = 48kHz/24-bit | σ·τ / J₂ | BT SIG 스펙 | ≠48kHz 또는 ≠24-bit |
| TP-DA-13 | Meta Codec 2.0 bitrate ladder에 {6,12,24}kbps 포함 | {n,σ,J₂} | Meta AI 논문 | {6,12,24} 제거 |
| TP-DA-14 | 차세대 spatial audio objects = 24 base | J₂=24 | MPEG-H/Dolby | ≠24 objects |

---

## Tier 3: 중기 검증 (3-10년, 기술 진화)

| # | 예측 | n=6 수식 | 검증 방법 | 반증 조건 |
|---|------|---------|---------|----------|
| TP-DA-15 | microLED 서브픽셀 = 3색 (RGB) 유지 | n/φ=3 | 디스플레이 기술 | RGBW가 microLED 표준 |
| TP-DA-16 | 차세대 holographic display FOV > 120° | σ(σ-φ)=120 최소 | 프로토타입 스펙 | 상용화 FOV < 60° |
| TP-DA-17 | 뇌-컴퓨터 인터페이스 시각 채널 = 12 또는 24 | σ 또는 J₂ | BCI 논문 | 완전 다른 수 |
| TP-DA-18 | 차세대 이머시브 오디오 = 24채널 + object | J₂=24 | MPEG-I 표준 | ≠24채널 |
| TP-DA-19 | 8K 방송 표준 프레임 = 60fps (σ·sopfr) | σ·sopfr=60 | NHK/ATSC 표준 | 8K@120fps 표준화 |
| TP-DA-20 | Light field display angular resolution = 48 views 이상 | σ·τ=48 | LF 프로토타입 | < 24 views |
| TP-DA-21 | AI upscaling 최적 입력 = 1080p (σ²·90/12) → 4K | -- | AI 논문 | 720p 입력이 최적 |

---

## Tier 4: 장기/이론적 예측 (10년+)

| # | 예측 | n=6 수식 | 검증 방법 | 반증 조건 |
|---|------|---------|---------|----------|
| TP-DA-22 | 인간 CFF 한계 실험 = 144Hz 부근 포화 확인 | σ²=144 | 신경과학 논문 | CFF 포화 > 200Hz |
| TP-DA-23 | 디지털 후각 기본 향 = 6종 이하 | n=6 | 화학 센서 연구 | > 10종 필요 |
| TP-DA-24 | 완전 몰입 VR에 필요한 감각 = 6가지 | n=6 | VR 연구 | < 4 또는 > 8 |
| TP-DA-25 | 청각 Bark scale 24 bands 재확인 | J₂=24 | 청각 연구 | ≠24 bands |
| TP-DA-26 | 음악 AI 생성 모델도 12 semitone 기반 | σ=12 | AI 음악 논문 | microtonal 표준화 |
| TP-DA-27 | 6DoF audio rendering = n=6 자유도 | n=6 | MPEG-I Audio | > 6 DoF |
| TP-DA-28 | 차세대 HDR 표준 동적 범위 = 12 stop | σ=12 | ITU-R/SMPTE | > 14 stop 표준 |

---

## 예측 요약

### 분포

| Tier | 예측 수 | 검증 시점 | 핵심 n=6 상수 |
|------|---------|----------|-------------|
| 1 (즉시) | 7 | 2026 | σ-τ, σ-φ, n/φ, σ, 2^(σ-sopfr) |
| 2 (단기) | 7 | 2027-2028 | σ², σ·τ, J₂, σ(σ-φ) |
| 3 (중기) | 7 | 2029-2035 | J₂, σ·sopfr, σ(σ-φ), σ·τ |
| 4 (장기) | 7 | 2036+ | σ², n, J₂, σ |
| **합계** | **28** | | |

### n=6 상수별 예측 빈도

| 상수 | 값 | 예측 수 | 예측 ID |
|------|-----|---------|---------|
| σ=12 | 12 | 6 | DA-5,6,11,26,27,28 |
| J₂=24 | 24 | 5 | DA-13,14,18,25 |
| σ·τ=48 | 48 | 3 | DA-10,12,20 |
| σ-τ=8 | 8 | 2 | DA-1,2 |
| σ-φ=10 | 10 | 1 | DA-3 |
| n/φ=3 | 3 | 2 | DA-4,15 |
| σ²=144 | 144 | 2 | DA-8,22 |
| σ(σ-φ)=120 | 120 | 2 | DA-9,16 |
| n=6 | 6 | 3 | DA-23,24,27 |

### Falsifiability Assessment

- **28/28 예측이 반증 가능** (구체적 수치 + 반증 조건 명시)
- **Tier 1의 7개는 현재 즉시 검증 가능** (논문/표준 확인)
- **Tier 2의 7개는 2027-2028 제품 출시로 검증** (HDMI 2.2, Vision Pro 2 등)
- 예측 실패 시 해당 BT claim 하향 조정 (EXACT → CLOSE / CLOSE → WEAK)
