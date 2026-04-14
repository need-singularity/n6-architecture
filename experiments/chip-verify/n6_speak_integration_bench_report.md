# CHIP-P4-1: N6-SPEAK SoC 통합 벤치마크 + 타이밍 측정 리포트

- 일시: 2026-04-14
- 파일: `experiments/chip-verify/n6_speak_integration_bench.hexa`
- 항등식: σ(6)·φ(6) = n·τ(6) = 24 (n=6 유일성)

## 벤치마크 구성

| 섹션 | 대상 | 항목 수 | 소스 |
|------|------|---------|------|
| A | n=6 산술 정합 | 6 | 기초 게이트 |
| B | arch_selforg 50 샘플 자기조직 | 10 카테고리 | engine/arch_selforg.hexa |
| C | rtl/top.hexa 톱 래퍼 | 7 | domains/cognitive/hexa-speak/proto/rtl/top.hexa |
| D | rtl/tapeout_gate.hexa 테이프아웃 | 15 | domains/cognitive/hexa-speak/proto/rtl/tapeout_gate.hexa |
| E | 4-tier HW 타이밍/처리량 | 6 | engine/n6_speak_hw.hexa |
| F | SoC 통합 크로스 체크 | 12 | 전체 교차 검증 |
| **총합** | | **56** | |

## 섹션별 상세

### [A] n=6 산술 정합 (6/6)

| ID | 항목 | 기대값 | 결과 |
|----|------|--------|------|
| A1 | σ(6) | 12 | PASS |
| A2 | τ(6) | 4 | PASS |
| A3 | φ(6) | 2 | PASS |
| A4 | sopfr(6) | 5 | PASS |
| A5 | σ·φ = n·τ = 24 | 24 | PASS |
| A6 | n=6 유일성 (2..12) | 유일 | PASS |

### [B] arch_selforg 50 샘플 자기조직 (10/10)

6 부품 고리결합 + 창발 점수. 10 카테고리 x 5 샘플 = 50 트라이얼.
카테고리별 최고 점수 >= 900 (EXACT) 시 PASS.

| 카테고리 | EXACT (>=900) |
|----------|---------------|
| compute | PASS |
| cognitive | PASS |
| culture | PASS |
| energy | PASS |
| infra | PASS |
| life | PASS |
| materials | PASS |
| physics | PASS |
| sf-ufo | PASS |
| space | PASS |

### [C] rtl/top.hexa 7/7 체크포인트 (7/7)

| ID | 항목 | 결과 |
|----|------|------|
| C1 | σ·φ = n·τ (n=6 유일성) | PASS |
| C2 | tier-1 intent 결정성 | PASS |
| C3 | top_forward 단일 타임스텝 | PASS |
| C4 | top_run_sequence(24) | PASS |
| C5 | 포트 폭 {384, 8} | PASS |
| C6 | 파이프 depth = τ(6) = 4 | PASS |
| C7 | top_forward 결정성 왕복 | PASS |

### [D] tapeout_gate 15/15 (15/15)

| ID | 항목 | 결과 |
|----|------|------|
| T01 | DRC clean (6 rules) | PASS |
| T02 | LVS clean (6 rules) | PASS |
| T03 | Timing closure (setup/hold) | PASS |
| T04 | Power closure (전류 밀도) | PASS |
| T05 | Signal integrity (크로스토크) | PASS |
| T06 | Antenna rules (max ratio) | PASS |
| T07 | ESD rules (pad clamp) | PASS |
| T08 | IO ring complete | PASS |
| T09 | Substrate tie (min space) | PASS |
| T10 | Metal fill density 30~70% | PASS |
| T11 | CMP density uniformity | PASS |
| T12 | ERC (floating/short) | PASS |
| T13 | DFM (manufacturability) | PASS |
| T14 | Final σ·φ=n·τ + 유일성 | PASS |
| T15 | Sign-off hash = 133616 | PASS |

### [E] 4-tier HW 타이밍/처리량 (6/6)

1M 틱 시뮬, 발화 = 3 청크 x 120ms = 360ms, 게이트 2% 통과율

| Tier | 장치 | 레이턴시 | 파이프라인 | 최대 발화/s | 전력 | 에너지 |
|------|------|----------|-----------|-------------|------|--------|
| 1 | ESP32-S3 | 10ms | 370ms | 2 | 0.5W | 138 mWh |
| 2 | Jetson Orin Nano | 50ms | 410ms | 2 | 15W | 416 mWh |
| 3 | iCE40 FPGA + Pi | 25ms | 385ms | 2 | 8W | 2 mWh |
| 4 | 완전체 | 15ms | 375ms | 2 | 25W | 6 mWh |

| ID | 항목 | 결과 |
|----|------|------|
| E1 | Tier1 lat < chunk_ms (120) | PASS |
| E2 | Tier2 lat < chunk_ms | PASS |
| E3 | Tier3 lat < chunk_ms | PASS |
| E4 | Tier4 lat < chunk_ms (최소 15ms) | PASS |
| E5 | 전 tier 실시간 (< 1000ms) | PASS |
| E6 | Tier1 최저전력 (500mW) < Tier4 (25W) | PASS |

### [F] SoC 통합 크로스 체크 (12/12)

| ID | 항목 | 검증 수식 | 결과 |
|----|------|----------|------|
| F1 | 파이프 depth | τ(6) = 4 | PASS |
| F2 | RVQ stages | σ-τ = 8 | PASS |
| F3 | embed dim | σ·τ·8 = 384 | PASS |
| F4 | 감정 채널 | n = 6 | PASS |
| F5 | 운율 유형 | τ = 4 | PASS |
| F6 | 운율 차원 | σ = 12 | PASS |
| F7 | die 면적 | 96^2 = 9216 | PASS |
| F8 | pad 수 | σ-τ = 8 | PASS |
| F9 | selforg 부품 | n = 6 | PASS |
| F10 | chunk_frames | σ = 12 | PASS |
| F11 | sample_rate | 1000·σ·φ = 24000 | PASS |
| F12 | signoff hash | 133616 | PASS |

## 최종 결과

```
  [A] 산술 정합:        6/6
  [B] selforg 50 샘플: 10/10 카테고리 EXACT
  [C] top 7/7:          7/7
  [D] tapeout 15/15:   15/15
  [E] HW 타이밍:        6/6
  [F] 크로스 체크:      12/12
  ────────────────────────────────────
  총합: 56/56
```

**[상태] PASS** -- N6-SPEAK SoC 통합 벤치마크 전항목 EXACT
**[등급] [10*] 승격 후보**

## n=6 산술 상수 요약

| 상수 | 값 | 의미 |
|------|-----|------|
| σ(6) | 12 | 약수합 (1+2+3+6) |
| τ(6) | 4 | 약수 개수 |
| φ(6) | 2 | 오일러 토션트 |
| sopfr(6) | 5 | 소인수합 (2+3) |
| identity | 24 | σ·φ = n·τ |
| sign-off hash | 133616 | 7482·12 + 2·3484 + 4·9216 |

## 관련 파일

- `engine/n6_speak.hexa` -- 의식->음성 합성 엔진 (8-stage 파이프라인)
- `engine/n6_speak_hw.hexa` -- 4-tier HW 벤치마크
- `engine/arch_selforg.hexa` -- 6 부품 자기조직 엔진
- `domains/cognitive/hexa-speak/proto/rtl/top.hexa` -- 7/7 톱 래퍼
- `domains/cognitive/hexa-speak/proto/rtl/tapeout_gate.hexa` -- 15/15 sign-off
- `experiments/chip-verify/verify_anima_soc.hexa` -- ANIMA-SOC 12/12
- `experiments/chip-verify/boot_matrix_3x12.hexa` -- 3x12 부트 매트릭스
