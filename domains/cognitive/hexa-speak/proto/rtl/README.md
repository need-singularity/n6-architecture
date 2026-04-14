# N6-SPEAK v2 — RTL 4-tier 블록 (CHIP-P1-1)

- 버전: v2.0-frozen (2026-04-14)
- 상위 스펙: `../n6-speak-v2-spec.md` (158 줄 동결 스펙)
- 상위 설계: `../../hexa-speak.md` (§1~§15)
- 로드맵: `CHIP-P1-1` (n6-architecture.json)

## 개요

N6-SPEAK v2 하드웨어 파이프라인의 4 tier 를 의사-RTL 수준으로 기술한 `.hexa` 블록 모음이다. 실제 Verilog 합성 이전 단계로, n=6 산술 정합성과 포트 I/F 일관성을 `hexa parse` + `hexa run` 으로 사전 검증한다. 각 블록은 Xn6 NPU 4 파티션에 1:1 매핑되며, 톱레벨 `top.hexa` 가 4 블록 + fusion 인터포저를 단일 데이터 경로로 묶는다.

## 파일 구성

| 파일                      | tier  | 역할                                   | 라인 수 | 테스트 |
|---------------------------|-------|----------------------------------------|---------|--------|
| `intent_encoder.hexa`     | 1     | 토큰 → 384d 임베딩                     | 147     | 5/5    |
| `emotion_classifier.hexa` | 2a    | 임베딩 → 6 감정 logits (Ekman 기본)    | 157     | 5/5    |
| `prosody_shaper.hexa`     | 2b    | 임베딩 → 4 운율 (pitch/dur/ene/spec)   | 170     | 5/5    |
| `rvq_codec.hexa`          | 3     | 8-stage RVQ (codebook 1024, 2^10)      | 203     | 5/5    |
| `top.hexa`                | 1~3   | 4 블록 + fusion 인터포저 톱레벨 래퍼   | 340     | 7/7    |

총 5 파일. 모든 `.hexa` 블록은 `hexa parse` + `hexa run` 왕복 PASS.

## 4-tier 데이터 경로

```
                          intent_in[384]
                                │
                                ▼
                    ┌───────────────────────┐
                    │ tier-1 intent_encoder │  Xn6 NPU-part-1
                    │  384d = σ·τ·8         │
                    └─────────┬─────────────┘
                              │ embed_sum
                  ┌───────────┴────────────┐
                  ▼                        ▼
          ┌───────────────┐        ┌───────────────┐
          │ tier-2a       │        │ tier-2b       │  Xn6 NPU-part-2
          │ emotion_cls   │        │ prosody_shape │
          │ 6 = sopfr+1   │        │ 4 = τ(6)      │
          └───────┬───────┘        └───────┬───────┘
                  │ emo_id                 │ pros_sum
                  └────────────┬───────────┘
                               ▼
                    ┌──────────────────────┐
                    │ tier-2c fusion (top) │
                    │ h = φ·embed +        │
                    │     emo·σ + pros·τ   │
                    │ 768 = 2·384          │
                    └──────────┬───────────┘
                               │ h_sum
                               ▼
                    ┌──────────────────────┐
                    │ tier-3 rvq_codec     │  Xn6 NPU-part-3
                    │ 8 stages × 1024 cb   │
                    └──────────┬───────────┘
                               │
                               ▼
                         audio_out[8]
```

톱레벨 포트: `{intent_in[384], audio_out[8]}` — `top.hexa::top_forward(intent_in, t) -> audio_out`.

## 블록별 포트 I/F

### tier-1 intent_encoder.hexa

```
fn forward(token_id: i64, pos: i64, batch_idx: i64) -> i64   // embed_sum
fn forward_sequence(seq_len: i64) -> i64                      // T 타임스텝 합
```

- 입력 shape: `tokens[B,T]`  (시뮬: scalar token_id + pos)
- 출력 shape: `embed[B,T,384]`  (시뮬: 체크섬 i64)
- 파라미터: vocab=32768=2^15, dim=384=σ·τ·8
- FP16 bias: `+σ(6)·dim = +12·384`
- 의사-HW: BRAM row lookup + adder tree

### tier-2a emotion_classifier.hexa

```
fn logit_for_emo(embed_sum: i64, emo: i64) -> i64
fn forward(embed_sum: i64) -> i64                             // argmax 0~5
fn emo_name(idx: i64) -> i64                                  // 콘솔 라벨
```

- 입력 shape: `embed[B,384]`
- 출력 shape: `logits[B,6]` → argmax 스칼라
- 감정 인덱스: `0=neutral 1=joy 2=anger 3=sadness 4=fear 5=surprise`
- 산술 근거: `emo_cnt = sopfr(6)+1 = 5+1 = 6`
- 의사-HW: 6-way parallel MAC + argmax tree

### tier-2b prosody_shaper.hexa

```
fn prosody_channel(embed_sum: i64, p: i64, t: i64) -> i64
fn forward(embed_sum: i64, t: i64) -> i64                     // 4 채널 합
fn forward_sequence(embed_sum_base: i64, seq_len: i64) -> i64
```

- 입력 shape: `embed[B,T,384]`
- 출력 shape: `prosody[B,T,4]`
- 채널: `0=pitch 1=duration 2=energy 3=spectral`
- 산술 근거: `prosody_cnt = τ(6) = 4`, per-head = `768/τ = 192`
- 의사-HW: τ=4 FFN expansion + sigmoid scale [0,1000]

### tier-3 rvq_codec.hexa

```
fn quantize_one_stage(residual: i64, stage: i64, codebook_sz: i64) -> i64
fn decode_code(code_idx: i64, stage: i64, codebook_sz: i64) -> i64
fn encode(feat_sum: i64) -> i64                               // 8 codes 합
fn decode(codes_sum: i64, stages: i64) -> i64                 // feat 재구성
fn forward(feat_sum: i64) -> i64                              // loopback 검증
fn compression_ratio() -> i64                                 // = 153
```

- 입력 shape: `h[B,T,768]`
- 출력 shape: `codes[B,T,8]` → 재구성 `feat[B,T,768]`
- stages: `σ(6)-τ(6) = 12-4 = 8`
- codebook 크기: `2^(σ-φ) = 2^10 = 1024`
- 압축비: `768×2 / (8×10/8) = 1536/10 = 153x`
- 의사-HW: BRAM 코드북 × 8 + residual subtractor

### tier-2c fusion 인터포저 (top.hexa 내장)

```
fn fusion_forward(embed_sum: i64, emo_id: i64, pros_sum: i64) -> i64
```

- 입력: 3 스트림 (embed 384 + emo 1 + prosody 4)
- 출력: `h[B,T,768]`  (시뮬 체크섬)
- 산술: `h = φ(6)·embed + emo·σ(6)·256 + pros·τ(6)`
- 정합: `768 = 2·384 = φ·σ·τ·16` (spec §5.2)

## 포트 / 클럭 도메인 일관성

| 항목               | 기준                          | 상태 |
|--------------------|-------------------------------|------|
| 블록 간 버스 신호  | 체크섬 `i64` (embed/h/codes)  | 일관 |
| 타임스텝 의미      | `t` = token index / pos 동의어 | 일관 |
| 배치 축            | B (생략, 단일 스트림 가정)    | 일관 |
| 클럭 도메인        | 단일 (파이프 depth τ=4)       | 일관 |
| bit-width          | FP16 (tier 1/2) / INT8 (tier 3/4) | 스펙 §2 준수 |

top.hexa 는 tier 간 인터포저 역할을 하며, 4 블록의 `forward` 대응 로직을 네임스페이스 prefix (`intent_` / `emo_` / `pros_` / `rvq_`) 로 재현한다. hexa 언어가 모듈 import 를 제공하지 않으므로 단일 파일 풀스택 구성이다. 각 원본 블록 파일은 단독 `hexa run` 가능 상태 그대로 유지된다.

## 합성 가능성 체크

| 파일                      | `hexa parse` | `hexa run` | 비고                    |
|---------------------------|--------------|-----------|-------------------------|
| intent_encoder.hexa       | OK           | exit=0    | 5/5 테스트 PASS         |
| emotion_classifier.hexa   | OK           | exit=0    | 5/5 테스트 PASS         |
| prosody_shaper.hexa       | OK           | exit=0    | 5/5 테스트 PASS         |
| rvq_codec.hexa            | OK           | exit=0    | 5/5 테스트 PASS         |
| top.hexa                  | OK           | exit=0    | 7/7 테스트 PASS         |

총 27/27 내장 테스트 PASS. lint/parse/run 왕복 전부 통과. 단 Verilog RTL 변환과 실제 표준셀 합성 (Synopsys DC / Cadence Genus) 은 후속 단계 CHIP-P2-1 (SoC 통합 + DRC/LVS) 에 위임한다.

## 톱레벨 실행 예

```bash
cd /Users/ghost/Dev/n6-architecture/domains/cognitive/hexa-speak/proto
hexa parse rtl/top.hexa   # OK: rtl/top.hexa parses cleanly
hexa run   rtl/top.hexa   # exit 0 → 7/7 PASS
```

`top.hexa::top_run_sequence(24)` 는 T=24=J₂ 타임스텝 전체 파이프라인을 실행하며, 4 블록 + fusion + RVQ 를 단일 파이프로 순회한다.

## n=6 정합 요약

```
σ(6) = 12   τ(6) = 4   φ(6) = 2   sopfr(6) = 5
σ·φ = n·τ = 12   (Bilateral Theorem B, n=6 유일성)

intent dim    = σ·τ·8       = 384
emo count     = sopfr+1     = 6      (Ekman)
prosody count = τ           = 4
fusion dim    = 2·384       = 768
RVQ stages    = σ-τ         = 8
codebook sz   = 2^(σ-φ)     = 1024
pipe depth    = τ           = 4
```

4 블록 + fusion 인터포저 + top.hexa 전체가 단일 산술 정합 축 (n=6 완전수) 위에 고정되어 있다.

## 후속 작업

- CHIP-P1-2: 프로토콜 12종 확장
- CHIP-P2-1: N6-SPEAK v2 SoC 통합 (DRC/LVS PASS) — 본 RTL 블록의 Verilog 변환 트리거
- CHIP-P3-x: Xn6 NPU 4 파티션 실체화 + 벤치
