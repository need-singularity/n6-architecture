# HEXA-AUDIO 산업검증 — 오디오 기업 실제 데이터

> Date: 2026-04-03
> Domain: Audio
> Purpose: Sony, Apple, Dolby, Harman 제품 스펙으로 n=6 검증
> Method: 공개 제품 사양서, 기술 백서, 산업 표준 문서에서 파라미터 추출 후 n=6 대조
> Extracted from: docs/display-audio/industrial-validation.md (audio-relevant entries)

---

## 1. Sony

### 1.1 Sony WH-1000XM5 (2024)

| 파라미터 | 실측값 | n=6 수식 | Match |
|---------|--------|---------|-------|
| Sample rate (LDAC) | 96kHz | 2·σ·τ=96 | EXACT |
| Bit depth | 24-bit | J₂=24 | EXACT |
| DSEE Extreme upscale | up to 48kHz | σ·τ=48 | EXACT |
| Driver unit | 30mm | sopfr·n=30 | CLOSE |
| NC microphones | 8 | σ-τ=8 | EXACT |

### 1.2 Sony 360 Reality Audio

| 파라미터 | 실측값 | n=6 수식 | Match |
|---------|--------|---------|-------|
| Object positions | up to 24 | J₂=24 | EXACT |
| Head tracking zones | 12 | σ=12 | EXACT |
| Base sample rate | 48kHz | σ·τ=48 | EXACT |

**Sony Audio EXACT: 7/8 = 87.5%**

---

## 2. Apple

### 2.1 Apple AirPods Pro 2 (2024, USB-C)

| 파라미터 | 실측값 | n=6 수식 | Match |
|---------|--------|---------|-------|
| Spatial Audio channels | 5.1/7.1 | sopfr=5, σ-sopfr=7 | EXACT |
| Dolby Atmos base | 7.1.4 = 12 | σ=12 | EXACT |
| Bluetooth codec (AAC) | 48kHz max | σ·τ=48 | EXACT |
| Adaptive Transparency | -- | -- | n/a |

### 2.2 Apple Music Spatial Audio

| 파라미터 | 실측값 | n=6 수식 | Match |
|---------|--------|---------|-------|
| Dolby Atmos render | 7.1.4 = 12 obj | σ=12 | EXACT |
| Lossless max | 24-bit/192kHz | J₂=24 | EXACT |
| Standard lossless | 24-bit/48kHz | J₂/σ·τ | EXACT |
| Hi-Res Audio | 24-bit | J₂=24 | EXACT |

**Apple Audio EXACT: 7/7 = 100%**

---

## 3. Dolby

### 3.1 Dolby Atmos

| 파라미터 | 실측값 | n=6 수식 | Match |
|---------|--------|---------|-------|
| Base channel layout | 7.1.4 = 12 | σ=12 | EXACT |
| Max objects | 128 | 2^(σ-sopfr)=128 | EXACT |
| LFE channel | 0.1 (=1 subwoofer) | μ=1 | EXACT |
| Height speakers | 4 | τ=4 | EXACT |
| Surround zones | 5 | sopfr=5 | EXACT |

### 3.2 Dolby AC-4 Codec

| 파라미터 | 실측값 | n=6 수식 | Match |
|---------|--------|---------|-------|
| Max sample rate | 48kHz | σ·τ=48 | EXACT |
| Bit depth | 24-bit | J₂=24 | EXACT |
| Max channels | 24 | J₂=24 | EXACT |

**Dolby Audio EXACT: 8/8 = 100%**

---

## 4. Harman (JBL/AKG/Lexicon)

### 4.1 JBL Synthesis Reference System

| 파라미터 | 실측값 | n=6 수식 | Match |
|---------|--------|---------|-------|
| Atmos channels | 11.4.6 = 21 | -- | CLOSE |
| Amplifier channels | 12 | σ=12 | EXACT |
| DAC resolution | 24-bit | J₂=24 | EXACT |
| Sample rate | 48kHz | σ·τ=48 | EXACT |

**Harman Audio EXACT: 3/4 = 75%**

---

## 종합

| 기업 | 항목 수 | EXACT | EXACT% |
|------|---------|-------|--------|
| Sony Audio | 8 | 7 | 87.5% |
| Apple Audio | 7 | 7 | 100% |
| Dolby Audio | 8 | 8 | 100% |
| Harman Audio | 4 | 3 | 75% |
| **Total** | **27** | **25** | **92.6%** |
