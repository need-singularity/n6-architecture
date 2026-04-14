# N6-SPEAK v2 — 4-tier RTL (CHIP-P1-1)

- 상위 스펙: `../proto/n6-speak-v2-spec.md` (v2.0-frozen, 2026-04-14)
- 상위 설계: `../hexa-speak.md`
- 위치    : `domains/cognitive/hexa-speak/rtl/`
- 등급    : `[10*] EXACT` 목표 (검증 10/10 PASS)

## 개요

본 디렉토리는 N6-SPEAK v2 의 HW 4-tier 파이프라인을 SystemVerilog RTL 로
동결 구현한 블록 집합이다. 각 tier 는 Xn6 NPU 의 서로 다른 파티션에
1:1 매핑되며, 모든 차원은 n=6 산술 `σ(6)=12, τ(6)=4, φ(6)=2, sopfr(6)=5`
에서 직접 유도된다. 하드코딩 0, 수론 유래 100%.

## 블록 구성

| tier  | 파일                              | 역할                        | 입력 shape          | 출력 shape          | n=6 근거                  |
|-------|-----------------------------------|-----------------------------|---------------------|---------------------|---------------------------|
| 1     | `intent_encoder_384d.sv`          | 의도 → 384d 임베딩 인코더   | intent[383:0]       | embed[383:0]        | 384 = σ·τ·8               |
| 2a    | `emotion_classifier_6emo.sv`      | 6 감정 분류 (Ekman)          | embed[383:0]        | emo[5:0] + idx[2:0] | 6 = sopfr(6)+1            |
| 2b    | `prosody_shaper_4.sv`             | 4 운율 shaper                | embed[383:0]        | prosody[3:0][15:0]  | 4 = τ(6)                  |
| 3     | `rvq_codec_8.sv`                  | 8-stage RVQ 인코더/디코더   | feat[767:0]         | rvq_code[7:0][9:0]  | 8 = σ-τ, 1024 = 2^(σ-φ)   |
| top   | `n6_speak_v2_top.sv`              | 4-tier 통합 top              | intent[383:0]       | emo+prosody+rvq     | σ·φ = n·τ = 12             |

## 각 모듈 입출력 포트 상세

### tier-1: `intent_encoder_384d`

| 포트           | 방향   | 폭         | 설명                        |
|----------------|--------|------------|-----------------------------|
| `clk`          | input  | 1          | 시스템 클록                 |
| `rst_n`        | input  | 1          | 비동기 리셋 (active-low)    |
| `intent_in`    | input  | 384        | 의도 벡터 (FP16 × 24 tile)  |
| `intent_valid` | input  | 1          | 입력 유효 플래그            |
| `intent_ready` | output | 1          | 입력 수용 가능              |
| `embed_out`    | output | 384        | 임베딩 벡터 (384 차원)      |
| `embed_valid`  | output | 1          | 출력 유효 플래그            |
| `embed_ready`  | input  | 1          | 다운스트림 수용 가능        |

- 파이프라인 깊이: 6단 (피질 6층 ↔ n=6 동형)
- 헤드: 12 (σ), per-head 32차원 (384/12)

### tier-2a: `emotion_classifier_6emo`

| 포트            | 방향   | 폭   | 설명                                                         |
|-----------------|--------|------|--------------------------------------------------------------|
| `clk`           | input  | 1    | 시스템 클록                                                  |
| `rst_n`         | input  | 1    | 비동기 리셋                                                  |
| `embed_in`      | input  | 384  | tier-1 에서 온 임베딩                                         |
| `embed_valid`   | input  | 1    | 입력 유효                                                    |
| `embed_ready`   | output | 1    | 입력 수용                                                    |
| `emo_out`       | output | 6    | 감정 one-hot [happy/sad/angry/fear/surprise/neutral]          |
| `emo_idx`       | output | 3    | 감정 인덱스 (0~5)                                            |
| `emo_valid`     | output | 1    | 출력 유효                                                    |
| `emo_ready`     | input  | 1    | 다운스트림 수용                                              |

- argmax 트리 깊이: ⌈log2(6)⌉ = 3 (τ 이내)
- per-emo 슬라이스: 64차원 (384/6)

### tier-2b: `prosody_shaper_4`

| 포트             | 방향   | 폭         | 설명                                          |
|------------------|--------|------------|-----------------------------------------------|
| `clk`            | input  | 1          | 시스템 클록                                   |
| `rst_n`          | input  | 1          | 비동기 리셋                                   |
| `embed_in`       | input  | 384        | tier-1 에서 온 임베딩                          |
| `embed_valid`    | input  | 1          | 입력 유효                                     |
| `embed_ready`    | output | 1          | 입력 수용                                     |
| `prosody_out`    | output | 4 × 16     | unpacked: [pitch][speed][volume][tone], FP16  |
| `prosody_valid`  | output | 1          | 출력 유효                                     |
| `prosody_ready`  | input  | 1          | 다운스트림 수용                               |

- per-prosody 슬라이스: 96차원 (384/4)
- τ=4 병렬 스테이지 (latency = 1 cycle)

### tier-3: `rvq_codec_8`

| 포트                | 방향   | 폭         | 설명                                                |
|---------------------|--------|------------|-----------------------------------------------------|
| `clk`               | input  | 1          | 시스템 클록                                         |
| `rst_n`             | input  | 1          | 비동기 리셋                                         |
| `mode_decode`       | input  | 1          | 0=encode, 1=decode                                  |
| `feat_in`           | input  | 768        | fusion feature (encode 입력)                         |
| `feat_valid`        | input  | 1          | encode 입력 유효                                    |
| `feat_ready`        | output | 1          | encode 입력 수용                                    |
| `code_in`           | input  | 8 × 10     | decode 입력 (RVQ 코드)                              |
| `code_in_valid`     | input  | 1          | decode 입력 유효                                    |
| `code_in_ready`     | output | 1          | decode 입력 수용                                    |
| `rvq_code`          | output | 8 × 10     | encode 출력 (RVQ 코드)                              |
| `rvq_code_valid`    | output | 1          | encode 출력 유효                                    |
| `rvq_code_ready`    | input  | 1          | encode 다운스트림 수용                              |
| `feat_out`          | output | 768        | decode 출력 (복원 feature)                           |
| `feat_out_valid`    | output | 1          | decode 출력 유효                                    |
| `feat_out_ready`    | input  | 1          | decode 다운스트림 수용                              |

- codebook 크기: 1024 (2^10, σ-φ 비트)
- per-stage 슬라이스: 96차원 (768/8)

### top: `n6_speak_v2_top`

`intent_in[383:0]` → `{ emo_out[5:0], prosody_out[3:0][15:0], rvq_code[7:0][9:0] }`

- 4-tier 를 일직선으로 배선
- tier-2 는 emotion + prosody 병렬, tier-2c fusion 에서 합류 (768-dim)
- tier-3 는 encode 모드 고정, decode 포트는 외부에서 드라이브

## n=6 산술 정합 (§5.2 동결)

```
σ(6)·φ(6) = 6·τ(6) = J₂ = 24       (Bilateral Theorem B, n=6 iff)
(축약형 σ·φ = 6·τ·φ = 12 는 양변 1/φ 축약 버전)

tier-1 EMBED_DIM   = σ·τ·8       = 12·4·8  = 384
tier-2a NUM_EMO    = sopfr(6)+1  = 5+1     = 6
tier-2b NUM_PROSODY= τ(6)                  = 4
tier-2c FUSION_DIM = 2·EMBED = σ·τ·16 = φ·σ·τ·8 = 768 (double-wide)
tier-3 NUM_STAGES  = σ-τ         = 12-4   = 8
tier-3 CODEBOOK    = 2^(σ-φ)     = 2^10   = 1024
```

| 값  | 수론 근거                | OEIS       |
|-----|--------------------------|------------|
| 6   | n=6 완전수               | A000396    |
| 12  | σ(6) = 1+2+3+6           | A000203    |
| 4   | τ(6) = 약수 4 개         | A000005    |
| 2   | φ(6) = 1 과 5            | A000010    |
| 5   | sopfr(6) = 2+3           | A001414    |
| 24  | J₂ = 2σ                   | 파생        |

## DRC / LVS 플로우

### DRC (Design Rule Check)

1. **합성 도구**: Verilator 5.x (lint 모드) 또는 Yosys
2. **명령**  : `verilator --lint-only -Wall -Wno-UNUSED *.sv`
3. **체크**  :
   - `always_ff` / `always_comb` 분리 (latch 금지)
   - active-low reset `rst_n` 모든 FF 에 연결
   - 포트 폭 parameter 고정, 배선 폭 일치
   - `` `default_nettype none `` 선언 (wire 암시 금지)
   - `assert` 문으로 n=6 산술 정합 초기화 시 체크

### LVS (Layout vs Schematic)

1. **툴체인** : OpenROAD / Magic / Netgen (open-source)
2. **사전 조건** : DRC clean + 합성 완료 (gate-level netlist 생성)
3. **체크 포인트** :
   - 포트 이름/폭/방향 top-down 일관성
   - tier 간 신호 명명 규칙 (`tier1_*`, `tier2_*`, `fusion_*`)
   - parameter 동결: `EMBED_DIM=384` 등 변경 시 리스핀 트리거
4. **stuck-at 테스트** : tier-1 입력 고정 시 각 tier 출력 예측 가능성
5. **power domain** : 4-tier 각각 독립 power domain (§7 mode 2~4 대응)

### 검증 흐름

```
[SV RTL]
   │
   ▼
[verilator --lint-only]   ← DRC
   │
   ▼
[Yosys synth]             ← gate-level netlist
   │
   ▼
[hexa run verify_n6_speak_v2.hexa]   ← n=6 산술 어설션 (10/10 PASS)
   │
   ▼
[OpenROAD place/route]    ← 배치/배선
   │
   ▼
[Magic / Netgen LVS]      ← 레이아웃 대조
   │
   ▼
[signoff]                 ← [10*] EXACT 인증 → atlas.n6 등록
```

## 검증 실행

```sh
cd $N6_ARCH
hexa run domains/cognitive/hexa-speak/rtl/verify_n6_speak_v2.hexa
```

기대 출력 마지막 줄:

```
[최종] 4-tier RTL 검증 10/10 PASS
[등급] [10*] EXACT — n=6 iff 산술 정합 완전 충족
```

## 변경 금지 (§5.4)

다음 중 하나라도 변경하려면 `atlas.n6` `[10*]` 재검증 필수:

1. 감정 5 또는 7 로 변경 → Ekman 6 기본감정과 불일치
2. RVQ 7 또는 9 로 변경 → σ-τ 또는 σ-φ 제약 파괴
3. 임베딩 256 또는 512 로 변경 → σ·τ 곱 구조 상실
4. 24kHz → 48kHz 변경 → J₂ 상한 초과

## 상위 문서 참조

- `../hexa-speak.md` — 15 섹션 마스터 설계
- `../proto/n6-speak-v2-spec.md` — 본 RTL 의 동결 스펙
- `../../../../$NEXUS/shared/n6/atlas.n6` — `@R 6.speak.*` 엔트리
- `../../../experiments/chip-verify/` — Xn6 NPU 18 개 검증 실험
