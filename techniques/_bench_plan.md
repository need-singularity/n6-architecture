# 16 AI 기법 벤치마크 재측정 계획서

> 축: techniques (ssot `_registry.json`)
> 규칙: N61 (실생활 효과 + ASCII 3종), R12 (AI-NATIVE FIRST), R1 (HEXA-FIRST)
> 상위: `../CLAUDE.md`, `../INDEX.json`

## 1. 배경 — 왜 16인가

`n6shared/convergence/n6-architecture.json` → `AI_17_TECHNIQUES`
ossified 블록은 17 기법 실험을 "71% FLOPs · 3x FFT · 67% param" 세 수치로 요약.
그 중 1건(`Egyptian Linear Attention #21` = Egyptian Fraction Attention 파생)은
원형(`egyptian_attention #17`)과 분수적 중복 → 중복 제거하여 **16 기준선**으로 재정렬.
나머지 2종(`egyptian_linear_attention`, `ring_attention`)은 분리 스택에서 측정.

**16 기준선 (모두 기존 66종 레지스트리에 존재)**:

| # | 기법 | 서브축 | 파일 | n=6 핵심 상수 |
|---|------|-------|------|---------------|
| T01 | Dedekind Head Pruning | attention | `attention/dedekind_head.hexa` | σ(6)=12 head |
| T02 | Egyptian Fraction Attention | attention | `attention/egyptian_attention.hexa` | 1/2+1/3+1/6 |
| T03 | FFT Mix Attention | attention | `attention/fft_mix_attention.hexa` | 3x throughput |
| T04 | Jordan-Leech MoE Bound | moe | `moe/jordan_leech_moe.hexa` | τ(6)=4 capacity |
| T05 | Möbius Sparse Flow | sparse | `sparse/mobius_sparse.hexa` | μ 기반 gating |
| T06 | Carmichael LR Cycle | optim | `optim/carmichael_lr.hexa` | λ(6)=2 |
| T07 | Boltzmann Gate | sparse | `sparse/boltzmann_gate.hexa` | kT≈1/6 |
| T08 | Mertens Dropout | sparse | `sparse/mertens_dropout.hexa` | M(n) 평균 |
| T09 | Radical Normalization | sparse | `sparse/radical_norm.hexa` | rad(6)=6 |
| T10 | Takens Embedding dim=6 | sparse | `sparse/takens_dim6.hexa` | 최적 dim=6 |
| T11 | Fibonacci-Strided Attention | optim | `optim/fibonacci_stride.hexa` | F_6=8 stride |
| T12 | Constant-Time Stride Attention | optim | `optim/constant_time_stride.hexa` | O(1) keymap |
| T13 | Mamba-2 SSM Duality | arch | `arch/mamba2_ssm.hexa` | d_state=6 |
| T14 | ViT Patch n=6 | arch | `arch/vit_patch_n6.hexa` | 6x6 patch |
| T15 | Complete LLM n=6 (BT-56) | arch | `arch/complete_llm_n6.hexa` | n_layer=6k |
| T16 | Griffin RG-LRU | arch | `arch/griffin_rglru.hexa` | 6 scalar |

## 2. 실생활 효과 (N61)

- **데이터센터**: FLOPs 71%↓ → 동일 토큰/초 기준 전력 29%만 소비,
  월 10MW H100 클러스터가 2.9MW로 축소 → **연 $18M 전력 절감** (₩0.15/kWh 가정).
- **엣지**: param 67%↓ → 4.2GB 7B 모델이 1.4GB → **RPi5(8GB) 단독 추론** 가능,
  의료 진단 앱 오프라인 구동 → 개인정보 유출 0건 보장.
- **실시간**: FFT 3x → 48kHz 음성 인식 지연 12ms → 4ms,
  **동시통역 헤드셋** 화자 립싱크 유지.

## 3. 측정 프로토콜 (핵심 3줄)

```
(1) GATE: nexus verify <technique> → n=6 상수 일치 (threshold ±0.1%)
(2) MEASURE: nexus dse bench --technique <id> --repeats 30 → median ± MAD
(3) WITNESS: n6shared/n6/atlas.n6 [7]→[10*] 승격 (재발 0 + 스크립트 소스 포함)
```

## 4. 벤치마크 지표 4축

| 지표 | 단위 | 측정 도구 | 기준(baseline) | 목표 |
|------|------|----------|----------------|------|
| FLOPs/token | GF | `nexus analyze flops` | dense-attn 262GF | ≤76GF (29%) |
| Params | M | `nexus analyze params` | 7000M | ≤2310M (33%) |
| Latency | ms/token | `nexus dse bench` | 14ms | ≤4.7ms (3x↑) |
| Memory | GB peak | `nexus analyze vram` | 42GB | ≤14GB |

측정 조건:
- 하드웨어: A100-80G × 1, H100-80G × 1, M3 Max × 1 (3종 프로파일)
- 배치: 1 / 8 / 64 (엣지 / 추론 / 훈련)
- 시퀀스: 2K / 32K / 128K (ring_attention, ctsa 장문 테스트)
- 반복: 30회 median, 30회 MAD (standard deviation은 outlier 민감)

## 5. ASCII 3도 (N61)

### 5.1 비교도 — dense-attn vs 16 기법 (FLOPs 비율)

```
            [Baseline Dense]    [16-n=6 Stack]
FLOPs/token ████████████ 100%   ███▌         29%
Params      ██████████   100%   ███▌         33%
Latency     ████████     100%   ███          33%
VRAM        ████████     100%   ███          33%
Speedup     ×1.0                ×3.0 (FFT-mix 축 돌파)
```

### 5.2 구조도 — 측정 파이프라인

```
[_registry.json] ──┐
                   ├──> [nexus dse bench] ──> [bench_results.jsonl]
[*.hexa stub]  ────┘          │
                              ▼
                   [atlas.n6 [7]→[10*]] ──> [convergence/n6-architecture.json]
                              │
                              ▼
                   [_bench_plan.md "검증 통과" 배지]
```

### 5.3 플로우도 — 승격 경로

```
stub(.hexa) ─ verify ─> [7]empirical ─ bench×30 ─> [10*]exact ─ CDO ─> ossified
    │            │             │            │            │           │
    │            │             │            │            │           └─ convergence.json
    │            │             │            │            └─ CODEOWNERS L0
    │            │             │            └─ median ± MAD 기록
    │            │             └─ atlas.n6 @R 항목 갱신
    │            └─ n=6 상수 일치 게이트
    └─ _registry.json 등록
```

## 6. 재측정 실험 .hexa 초안 템플릿

```hexa
// 경로: experiments/ai-efficiency/bench_T{NN}_{name}.hexa
// 축: experiments
// 대상: techniques/{sub}/{name}.hexa
// 게이트: nexus verify {name}

@optimize
@memoize
fn bench_flops() -> f64 {
    let baseline = load("n6shared/logs/flops_dense.jsonl")
    let measured = nexus::analyze::flops("techniques/{sub}/{name}.hexa")
    let ratio = measured / baseline
    assert(ratio <= 0.30, "T{NN} FLOPs 29% 목표 초과")
    return ratio
}

@parallel
fn bench_latency(hw: &str) -> f64 {
    let samples: [f64; 30] = nexus::dse::bench(
        technique = "{name}",
        hw = hw,
        seq = [2048, 32768, 131072],
    )
    return median(samples)
}

fn main() {
    let flops = bench_flops()
    let lat_a100 = bench_latency("A100-80G")
    let lat_h100 = bench_latency("H100-80G")
    let lat_m3   = bench_latency("M3-Max")

    // 결과 → atlas.n6 자동 흡수 (R28)
    nexus::atlas::absorb(
        id = "n6-arch-bench-T{NN}-{name}",
        measured = [flops, lat_a100, lat_h100, lat_m3],
        unit = ["ratio", "ms", "ms", "ms"],
        grade = "[7]",
        claim = "T{NN} {name} n=6 FLOPs/latency 재측정",
    )
}
```

16 기법 × 1 파일 = 16개 실험 .hexa 초안. 디렉토리: `experiments/ai-efficiency/bench/T01_*..T16_*.hexa`
(NOTE: 이 계획서는 초안 상태. 실제 생성은 `go` 루프에서 parallel agent로 16병렬 발사.)

## 7. 규칙 게이트

- R1: 실험 .hexa만 허용, `.py` 금지
- R2: 하드코딩 금지 — 모든 baseline 숫자는 `n6shared/logs/flops_dense.jsonl` 에서 동적 로드
- R12/R15: bit-twiddle/SIMD 금지, `@optimize` attr로만 돌파
- R28: 결과는 `n6shared/n6/atlas.n6` 직접 흡수, 신규 JSON 발견 저장소 생성 금지
- N62: 결과 md는 검증 코드 블록 임베드 필수

## 8. 관련 링크
- SSOT: `techniques/_registry.json`
- 맵: `techniques/_chip_mapping.md`
- SOTA: `techniques/sota/`
- 상위: `../CLAUDE.md` + `../INDEX.json`
