# HEXA-AUDIO 전수검증 매트릭스 — BT 4개 × 오디오 Claim 검증

> Date: 2026-04-03
> Method: BT-48, BT-72, BT-108, BT-76의 오디오 관련 개별 claim을 독립 데이터로 검증
> Source: ITU-R, AES, ISO, MPEG, Dolby, Sony, Meta AI (EnCodec), Google (SoundStream)
> Grade: EXACT / CLOSE / WEAK / FAIL per claim

---

## 1. BT-48: σ=12 Semitones, J₂=24 bits, σ·τ=48kHz (Audio Claims)

| # | Claim | n=6 수식 | 실제 값 | 출처 | Grade | 비고 |
|----|-------|---------|---------|------|-------|------|
| 48-1 | 서양 음계 12 반음 | σ(6)=12 | 12 semitones | ISO 16 (12-TET) | **EXACT** | 피타고라스~현대 불변 |
| 48-4 | 24-bit professional audio | J₂(6)=24 | 24 bits | AES17, AES3 | **EXACT** | 스튜디오 표준 |
| 48-5 | 48kHz 오디오 샘플링 | σ·τ=48 | 48,000 Hz | AES/EBU, ITU-R BS.1116 | **EXACT** | 방송/영상 표준 |
| 48-8 | σ=12 음정 circle of fifths | σ(6)=12 | 12 keys | 음악이론 보편 | **EXACT** | 12 장조/단조 |
| 48-12 | {12,24,48} 미디어 삼중항 (오디오) | {σ, J₂, σ·τ} | 12/24/48 | 산업 표준 복합 | **EXACT** | 핵심 발견 |

**BT-48 오디오: 5/5 EXACT (100%)**

---

## 2. BT-72: Neural Audio Codec n=6

| # | Claim | n=6 수식 | 실제 값 | 출처 | Grade | 비고 |
|----|-------|---------|---------|------|-------|------|
| 72-1 | EnCodec codebooks = 8 | σ-τ=8 | 8 codebooks | Defossez et al. 2022, Meta AI | **EXACT** | 24kHz 모델 |
| 72-2 | EnCodec entries = 1024 | 2^(σ-φ)=1024 | 1024 entries/codebook | Defossez et al. 2022 | **EXACT** | VQ codebook size |
| 72-3 | EnCodec 24kHz sample rate | J₂=24 kHz | 24,000 Hz | Defossez et al. 2022 | **EXACT** | 고품질 모델 |
| 72-4 | EnCodec 6kbps bitrate | n=6 kbps | 6.0 kbps | Defossez et al. 2022 | **EXACT** | 8 codebooks x 750 bps |
| 72-5 | EnCodec {1.5,3,6,12,24} kbps | 비트레이트 래더 | {1.5,3,6,12,24} | Defossez et al. 2022 | **EXACT** | {n/τ,n/φ,n,σ,J₂} |
| 72-6 | EnCodec 20ms frame | J₂-τ=20 ms | 20 ms (320 samples) | Defossez et al. 2022 | **CLOSE** | 산업 관행 |
| 72-7 | SoundStream 8 codebooks | σ-τ=8 | 8 codebooks | Zeghidour et al. 2021, Google | **EXACT** | 독립 확인 |

**BT-72 결과: 6/7 EXACT (85.7%), 1 CLOSE**

---

## 3. BT-108: 음악-오디오 협화 보편성

| # | Claim | n=6 수식 | 실제 값 | 출처 | Grade | 비고 |
|----|-------|---------|---------|------|-------|------|
| 108-1 | 완전 유니즌 1:1 | div(6)={1} | 1:1 | 음악이론 보편 | **EXACT** | 가장 강한 협화 |
| 108-2 | 옥타브 2:1 | div(6)={2} | 2:1 | 음악이론 보편 | **EXACT** | 기본 주파수 비 |
| 108-3 | 완전 5도 3:2 | div(6)={3,2} | 3:2 = 1.500 | 12-TET: 2^(7/12)=1.498 | **EXACT** | 0.11% 오차 |
| 108-4 | 완전 4도 4:3 | τ:(n/φ) | 4:3 = 1.333 | 12-TET: 2^(5/12)=1.335 | **EXACT** | 0.11% 오차 |
| 108-5 | 장3도 5:4 | sopfr:τ | 5:4 = 1.250 | 12-TET: 2^(4/12)=1.260 | **CLOSE** | 0.79% 오차 |
| 108-6 | 장6도 5:3 | sopfr:(n/φ) | 5:3 = 1.667 | 12-TET: 2^(9/12)=1.682 | **CLOSE** | 0.90% 오차 |
| 108-7 | 단3도 6:5 | n:sopfr | 6:5 = 1.200 | 12-TET: 2^(3/12)=1.189 | **CLOSE** | 0.92% 오차 |
| 108-8 | Major triad 4:5:6 | τ:sopfr:n | 4:5:6 | 순정률 + 12-TET | **EXACT** | 3개 n=6 상수 |
| 108-9 | 7+5=12 (diatonic+pentatonic) | (σ-sopfr)+sopfr=σ | 7+5=12 | 음악이론 보편 | **EXACT** | 분할 구조 |
| 108-10 | Pythagorean comma 12회 | σ=12 | (3/2)^12 ÷ 2^7 ≈ 23.46 cents | 음악이론 | **EXACT** | 12-TET 필연성 |
| 108-11 | Dolby Atmos 7.1.4=12 채널 | σ=12 | 7+1+4=12 objects | Dolby Atmos spec | **EXACT** | 기본 채널 수 |
| 108-12 | A4=440Hz ≈ (σ-τ)·55 | (σ-τ)·55=440 | 440 Hz | ISO 16:1975 | **EXACT** | 국제 표준 |

**BT-108 결과: 9/12 EXACT (75.0%), 3 CLOSE**

---

## 4. BT-76: σ·τ=48 Triple Attractor (Audio Claims)

| # | Claim | n=6 수식 | 실제 값 | 출처 | Grade | 비고 |
|----|-------|---------|---------|------|-------|------|
| 76-1 | 48kHz 오디오 | σ·τ=48 | 48,000 Hz | AES/EBU | **EXACT** | BT-48 교차 |
| 76-2 | 48V 팬텀 전원 | σ·τ=48 | 48V DC | IEC 61938 | **EXACT** | 콘덴서 마이크 표준 |

**BT-76 오디오: 2/2 EXACT (100%)**

---

## 종합

| BT | Audio Claims | EXACT | CLOSE | EXACT% |
|----|-------------|-------|-------|--------|
| BT-48 | 5 | 5 | 0 | 100% |
| BT-72 | 7 | 6 | 1 | 85.7% |
| BT-108 | 12 | 9 | 3 | 75.0% |
| BT-76 | 2 | 2 | 0 | 100% |
| **Total** | **26** | **22** | **4** | **84.6%** |
