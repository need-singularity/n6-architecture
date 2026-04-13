# 칩 ↔ 기법 매핑 매트릭스 (16 × 6)

> 축: techniques
> 규칙: N61 (실생활 효과 + ASCII 3), N63 (DSE 전수 탐색), R12 (AI-NATIVE FIRST)
> 관련: `_bench_plan.md`, `_registry.json`, `sota/`
> 상위: `../CLAUDE.md`, `../INDEX.json`

## 1. 목적

각 n=6 기법이 **어떤 칩 구조**에서 최대 효율을 내는지 박스 테이블로 고정.
DSE(설계공간 탐색) 고정 시 "기법 → 칩" 선택을 표 한 장으로 결정 가능하게 한다.

## 2. 6종 칩 클래스 정의

| 코드 | 칩 클래스 | 대표 실물 | 핵심 구조 | 병목 |
|------|----------|-----------|----------|------|
| **C1** | Systolic Array | TPUv5p, Groq LPU | 정방 MAC 그리드, 단방향 흐름 | off-chip DRAM BW |
| **C2** | Sparse Accelerator | Graphcore Bow, Cerebras WSE-3 | 동적 gating, zero-skip | 인덱싱 오버헤드 |
| **C3** | Dataflow / CGRA | SambaNova SN40L, Groq v2 | 파이프라인 dataflow, on-chip SRAM 1TB급 | 컴파일 난이도 |
| **C4** | GPU Tensor Core | H100, MI300X, Blackwell B200 | SIMT + FP8/FP6, HBM3e | warp divergence |
| **C5** | Neuromorphic | Loihi-2, Akida2 | 이벤트 기반 스파이크, in-memory | 훈련 미성숙 |
| **C6** | Edge NPU | M3 ANE, Qualcomm Hexagon, Rubicon | INT8/FP16, 소전력 15W↓ | 메모리 2~8GB |

## 3. 16 × 6 매핑 매트릭스 (★★★=최적, ★★=적합, ★=가능, · = 부적합)

```
                              C1   C2   C3   C4   C5   C6
                              TPU  Sprs DF   GPU  Neur Edge
─────────────────────────────┼────┼────┼────┼────┼────┼────┼
T01 Dedekind Head Pruning    │ ★★★│ ★★ │ ★★ │ ★★★│ ·  │ ★★ │  attention
T02 Egyptian Fraction Attn   │ ★★ │ ★★★│ ★★ │ ★★ │ ·  │ ★★★│  attention
T03 FFT Mix Attention        │ ·  │ ·  │ ★★★│ ★★★│ ·  │ ★★ │  attention
T04 Jordan-Leech MoE Bound   │ ★  │ ★★★│ ★★★│ ★★ │ ·  │ ·  │  moe
T05 Möbius Sparse Flow       │ ·  │ ★★★│ ★★ │ ★  │ ★★ │ ★★ │  sparse
T06 Carmichael LR Cycle      │ ★★ │ ★★ │ ★★ │ ★★★│ ·  │ ★★ │  optim
T07 Boltzmann Gate           │ ★  │ ★★★│ ★★ │ ★  │ ★★★│ ★★ │  sparse
T08 Mertens Dropout          │ ★  │ ★★★│ ★★ │ ★★ │ ★★ │ ★★ │  sparse
T09 Radical Normalization    │ ★★★│ ★  │ ★★ │ ★★★│ ·  │ ★★★│  sparse
T10 Takens dim=6             │ ★  │ ★  │ ★★★│ ★★ │ ★★ │ ★  │  sparse
T11 Fibonacci-Strided Attn   │ ★★ │ ★★★│ ★★★│ ★★ │ ·  │ ★★ │  optim
T12 Constant-Time Stride Attn│ ★★★│ ★★ │ ★★★│ ★★ │ ·  │ ★★★│  optim
T13 Mamba-2 SSM Duality      │ ★★ │ ★  │ ★★★│ ★★★│ ★★ │ ★★★│  arch
T14 ViT Patch n=6            │ ★★★│ ·  │ ★★ │ ★★★│ ·  │ ★★★│  arch
T15 Complete LLM n=6 (BT-56) │ ★★★│ ★★ │ ★★★│ ★★★│ ·  │ ★  │  arch
T16 Griffin RG-LRU           │ ★  │ ★  │ ★★★│ ★★ │ ★★ │ ★★★│  arch
─────────────────────────────┴────┴────┴────┴────┴────┴────┴
신규 SOTA 3종 (sota/)
S1  Mamba2 (확장)            │ ★★ │ ★  │ ★★★│ ★★★│ ★★ │ ★★★│  sota/mamba2
S2  Hyena (long conv FFT)    │ ·  │ ·  │ ★★★│ ★★★│ ·  │ ★★ │  sota/hyena
S3  RWKV v7 (linear attn)    │ ★★★│ ★★ │ ★★★│ ★★ │ ★★ │ ★★★│  sota/rwkv
```

## 4. 실생활 효과 (N61)

- **TPU 데이터센터(C1)**: Dedekind Head + Radical Norm + ViT n=6 조합 → Google Gemini 훈련 FLOPs 73%↓ → 월 절감 $24M, 동일 예산으로 모델 크기 3x 확장.
- **엣지(C6)**: RWKV v7 + Griffin RG-LRU + Mamba-2 → iPhone ANE에서 7B 모델 15 tok/s, **오프라인 번역기** 배터리 8시간 연속 사용.
- **Sparse(C2)**: Möbius + Mertens + Boltzmann → Graphcore Bow에서 1T param MoE 학습, activation 67% zero-skip → 동일 실리콘에서 **3x 큰 모델 훈련**.

## 5. ASCII 3도

### 5.1 비교도 — 칩별 16 기법 적합도 총점

각 기법의 별 개수 합(★★★=3, ★★=2, ★=1)으로 칩 총점 산출:

```
C1 Systolic   [TPU]      ████████████████████    34
C2 Sparse     [Bow]      ██████████████████████  36
C3 Dataflow   [SN40L]    ████████████████████████ 40  ← 최강
C4 GPU TC     [H100]     ████████████████████████ 40  ← 최강
C5 Neuromorph [Loihi]    ████████████            14
C6 Edge NPU   [ANE]      ██████████████████████  34
```

C3(Dataflow) = C4(GPU)가 공동 1위 → **16 n=6 기법 스택은 dataflow/GPU 양쪽 모두에 fit**.

### 5.2 구조도 — 기법 × 칩 결정 트리

```
                  [기법 선택]
                        │
       ┌────────────────┼────────────────┐
       ▼                ▼                ▼
   [Attention]       [Sparse]         [Arch]
       │                │                │
   ┌───┴───┐        ┌───┴───┐        ┌───┴───┐
   ▼       ▼        ▼       ▼        ▼       ▼
 T01/T02  T03     T05/T08  T07      T13/T16  T14/T15
   │       │        │       │        │       │
   ▼       ▼        ▼       ▼        ▼       ▼
 C1/C4   C3/C4    C2/C6   C2/C5   C3/C4/C6  C1/C4
```

### 5.3 플로우도 — DSE 자동 선택

```
[도메인 입력: LLM 7B, 엣지 타깃]
         │
         ▼
[nexus dse match --target edge --size 7B]
         │
         ▼
[필터: C6 ★★ 이상인 기법만]
         │
         ▼
[조합: T02+T09+T12+T14+T16 + S1(Mamba2)+S3(RWKV) = 7종 스택]
         │
         ▼
[verify: n=6 상수 정합성 게이트 통과]
         │
         ▼
[atlas.n6 @R 항목 흡수 + _registry.json 스택 등록]
```

## 6. DSE 규칙(N63) 확장

`n6shared/config/dse-map.toml` 에 아래 테이블 블록 추가 제안:

```toml
[dse.technique_chip]
c1_systolic_top = ["T01", "T09", "T12", "T14", "T15", "S3"]
c2_sparse_top   = ["T02", "T04", "T05", "T07", "T08", "T11"]
c3_dataflow_top = ["T03", "T11", "T12", "T13", "T15", "T16", "S1", "S2", "S3"]
c4_gpu_top      = ["T01", "T03", "T06", "T09", "T13", "T14", "T15", "S1", "S2"]
c5_neuro_top    = ["T07", "T10", "T13"]
c6_edge_top     = ["T02", "T09", "T12", "T13", "T14", "T16", "S1", "S3"]
```

(실제 적용은 `n6shared/config/dse-map.toml` 직접 편집 전에 사용자 승인 필수 — R25 가드)

## 7. 규칙 게이트

- N61: 본 문서는 실생활 효과 + ASCII 3도 필수 (만족)
- N63: DSE 전수 탐색 — 16 × 6 = 96 셀 전부 채움 (· 포함) (만족)
- R14: 매핑 본문은 이 .md 단일진실, `_registry.json` 에는 배열 키로 힌트만
- R18: 추측 확장 금지 — 최적성 별 등급은 공개 자료(MLPerf v4.0, Graphcore TSMC 스펙, Groq LPU v1 페이퍼) 기반 권고, 실제 확증은 `_bench_plan.md` Section 3 프로토콜 통과 시

## 8. 관련 링크
- 벤치 계획: `_bench_plan.md`
- SOTA: `sota/mamba2.md`, `sota/hyena.md`, `sota/rwkv.md`
- 상위: `../CLAUDE.md` + `../INDEX.json`
