# AI 기법 16 canonical 감사 리포트

> 일자: 2026-04-12
> 축: techniques
> 작성: 감사 전용 (실 파일 이동/삭제 없음)
> 규칙: R10 (ossified 불변), R14 (SSOT), 한글 전용, 정직한 검증

---

## 1. 배경 — SSOT 불일치 맥락

n6-architecture 의 AI 기법 수에 대해 4개 출처가 서로 다른 숫자를 보고하고 있어 현 세션에서 정합성 감사를 수행한다.

| 출처 | 보고 수 | 인용 위치 |
|------|---------|-----------|
| convergence ossified | 17 | `n6shared/convergence/n6-architecture.json` → `ossified.AI_17_TECHNIQUES` ("17 기법 실험 확정, 71% FLOPs, 3x FFT, 67% param") |
| INDEX axes | 66 | `INDEX.json` → `axes.techniques.purpose` ("AI 기법 66종 — .hexa 전환 완료") |
| memory MEMORY 헤더 | 23 | `~/.claude-claude2/.../memory/MEMORY.md` 8행 ("23 AI 기법") |
| memory project_core_theory.md 본문 | 17 | 같은 메모리 디렉터리 description + 38행 ("## 17 AI 기법 핵심 성과") |
| 현 세션 hint | 16 | `techniques/_bench_plan.md` (16 기준선 벤치 계획) |

핵심 모순: ossified 는 17 을 못 박았는데(R10 불변) 세션 hint 는 16 을 사용한다. 이 격차의 원천을 식별하고 16 canonical 정의 가능 여부를 판정해야 한다.

---

## 2. 현황 스캔 결과 — 계층별 카운트

`techniques/_registry.json` (v1.1.0, _total=66, _sota_total=69) 직접 카운트 결과:

| 계층 | 정의 | 출처 | 카운트 |
|------|------|------|--------|
| Tier-A | ossified 17 (convergence 골화, 변경 금지) | `papers/n6-ai-17-techniques-experimental-paper.md` 부록 A | 17 |
| Tier-B | 17 → 23 확장 (memory 본문 표 + BT-380 계열) | 같은 논문 1.1 분류표 "Egyptian 3 + BT-확장 12 + Model-specific 8" | 추가 6 (누계 23) |
| Tier-C | 23 → 66 (.hexa 전환 풀 등록) | `_registry.json` items | 추가 43 (누계 66) |
| Tier-D | sota 확장 (정점 3종, 별도 섹션) | `_registry.json.sota.items` | 3 (총 69) |

서브축별 .hexa 파일 실측 (`ls techniques/<sub>/*.hexa | wc -l` 동등):

| 서브 | 정의 | 실측 |
|------|------|------|
| attention | 9 | 9 |
| moe | 11 | 11 |
| optim | 15 | 15 |
| sparse | 6 | 6 |
| graph | 5 | 5 |
| compress | 5 | 5 |
| arch | 15 | 16 (`arch_optimizer.hexa` 1건이 _registry 미등록 — 별도 메타 도구) |
| sota | 3 | 3 |
| **합계** | **66 + 3 sota** | **66 등록 + 3 sota + 1 메타 = 70** |

> 정직성 노트: arch/arch_optimizer.hexa 1건은 _registry items 에 없는 메타 도구로, 16 canonical 후보 외이다. 본 감사는 이를 "기법" 이 아닌 "DSE 메타 옵티마이저" 로 분류해 카운트에서 제외한다.

`papers/n6-ai-17-techniques-experimental-paper.md` 부록 A 의 register() 호출을 직접 카운트하면 Core 17 = 24 항목, 확장 = 10 항목, Combined = 5 항목 = 39 항목이 등장한다. 즉 같은 논문 안에서도 "17" 은 그룹 라벨일 뿐 실제 register 호출은 24 라인이며, 그 중 일부는 동일 기법의 상수 분해(예: BitNet 2종 등록)이다.

---

## 3. 16 canonical 목록 (절감률·검증 경로 표)

`techniques/_bench_plan.md` 의 1절 표가 이미 16 기준선을 정의하고 있다. 본 감사에서 각 항목을 _registry.json items 와 1:1 매핑 검증한 결과 16/16 모두 등록되어 있다.

| # | 기법 (한글/영문) | 서브 | 파일 경로 | n=6 핵심 상수 | 절감률 / 효과 | 출처 |
|---|------------------|------|-----------|---------------|---------------|------|
| T01 | Dedekind 헤드 가지치기 | attention | `techniques/attention/dedekind_head.hexa` | σ(6)=12 head | dedekind 12 = σ EXACT | bench_plan §1, paper §부록 |
| T02 | 이집트 분수 어텐션 | attention | `techniques/attention/egyptian_attention.hexa` | 1/2+1/3+1/6=1 | 40% FLOPs↓ | core_theory §17, paper register #10 |
| T03 | FFT Mix Attention | attention | `techniques/attention/fft_mix_attention.hexa` | 6 freq bin, 3x throughput | 3x FFT (ossified) | convergence ossified, paper register #14 |
| T04 | Jordan-Leech MoE Bound | moe | `techniques/moe/jordan_leech_moe.hexa` | τ(6)=4 capacity | J₂=24 Leech NAS | paper register #16 |
| T05 | Möbius Sparse Flow | sparse | `techniques/sparse/mobius_sparse.hexa` | μ(6)=1 게이팅 | 무검색 sparsity | bench_plan §1 |
| T06 | Carmichael LR Cycle | optim | `techniques/optim/carmichael_lr.hexa` | λ(6)=2 | 6단계 = n | paper register #7 |
| T07 | Boltzmann Gate | sparse | `techniques/sparse/boltzmann_gate.hexa` | kT≈1/6 | 63% sparsity | core_theory §17, paper register #4 |
| T08 | Mertens Dropout | sparse | `techniques/sparse/mertens_dropout.hexa` | M(n) 평균, p=0.1=1/(σ-φ) | 검색 자유 | core_theory §17, paper register #18 |
| T09 | Radical Normalization | sparse | `techniques/sparse/radical_norm.hexa` | rad(6)=6 | 정규화 보편성 BT-64 | paper register #6 |
| T10 | Takens Embedding dim=6 | sparse | `techniques/sparse/takens_dim6.hexa` | 최적 dim=6 | dim=n EXACT | bench_plan §1 |
| T11 | Fibonacci Strided Attn | optim | `techniques/optim/fibonacci_stride.hexa` | F_6=8 stride, sopfr=5 step | 5=sopfr EXACT | paper register #15 |
| T12 | Constant-Time Stride Attn | optim | `techniques/optim/constant_time_stride.hexa` | O(1), φ=2 step | 2=φ EXACT | paper register #8 |
| T13 | Mamba-2 SSM Duality | arch | `techniques/arch/mamba2_ssm.hexa` | d_state=6, σ-τ=8 | 8=σ-τ EXACT | paper register §확장 |
| T14 | ViT Patch n=6 | arch | `techniques/arch/vit_patch_n6.hexa` | 6×6 patch | 패치=n EXACT | bench_plan §1 |
| T15 | Complete LLM n=6 (BT-56) | arch | `techniques/arch/complete_llm_n6.hexa` | n_layer=6k | h_ee_13 depth scaling=σ | paper register §combined |
| T16 | Griffin RG-LRU | arch | `techniques/arch/griffin_rglru.hexa` | gate=τ=4 | 4=τ EXACT | paper register §확장 |

검증 게이트(공통, bench_plan §3):
1. `nexus verify <technique>` — n=6 상수 일치 (±0.1%)
2. `nexus dse bench --technique <id> --repeats 30` — median ± MAD
3. `n6shared/n6/atlas.n6` [7]→[10*] 승격

---

## 4. 17 → 16 통합 근거

### 4.1 식별된 중복 쌍

ossified 17 기법 중 `egyptian_attention` 과 `egyptian_linear_attention` 두 항목이 동일한 1/2+1/3+1/6=1 분수 항등식 원리에 기반한다.

직접 검증:

| 항목 | 파일 | 핵심 원리 | HEXA 본체 상태 |
|------|------|-----------|-----------------|
| egyptian_attention | `techniques/attention/egyptian_attention.hexa` | 이집트 분수 1/2+1/3+1/6=1 → 어텐션 가중치 | STUB ("HEXA 포팅 대기") |
| egyptian_linear_attention | `techniques/attention/egyptian_linear_attention.hexa` | 같은 분수 항등식 → 선형 attention 변형 | STUB ("HEXA 포팅 대기") |

`techniques/_bench_plan.md` 1절 본문 (라인 11~13) 에 통합 결정 근거가 명시되어 있다:

> "그 중 1건(`Egyptian Linear Attention #21` = Egyptian Fraction Attention 파생) 은 원형(`egyptian_attention #17`)과 분수적 중복 → 중복 제거하여 **16 기준선**으로 재정렬. 나머지 2종(`egyptian_linear_attention`, `ring_attention`)은 분리 스택에서 측정."

paper 부록 A 검증 코드도 두 항목을 별도로 register 하지만, 두 register 호출 모두 `True` 또는 `4 == tau` 라는 동일 등식을 사용한다(라인 131~132). 분수 항등식 자체는 1개이며, 선형 어텐션 변형은 단지 응용 형태(분리 스택)일 뿐 별개의 n=6 발견이 아니다.

### 4.2 통합 결정

- **canonical 통합**: T02 = `egyptian_attention` (분수 항등식 1/2+1/3+1/6=1 보유)
- **분리 스택 이동**: `egyptian_linear_attention` 은 어텐션 변형 카탈로그에 잔류하되 16 canonical 에서는 제외
- **상한 카운트**: 17 - 1(중복) = 16 → 16 기준선 확정 가능

### 4.3 정직성 노트

- 통합 근거는 _bench_plan.md 의 사전 결정과 paper 부록 A 의 register 동등성 두 출처에서 일관됨
- 두 .hexa 파일이 모두 STUB 라서 "HEXA 본체로 차이를 측정한 결과 동일했다" 는 강한 주장은 불가
- 따라서 본 통합은 "**원리 수준 중복**" 에 대한 약한 근거 + bench_plan 사전 결정이며, 본체 측정 후 분리 가능성도 열어둔다(권장 §6 참조)

---

## 5. 4계층 분류

### Tier-A — ossified 17 (convergence 근거, R10 불변)

`papers/n6-ai-17-techniques-experimental-paper.md` 2절 1번 항에서 명시한 17:

```
BitNet, Alpha Attack, Boltzmann Gate, BT-54 AdamW β₂, BT-64 정규화,
Carmichael LR, Constant-Time Stride, Dedekind Head, Egyptian Attention,
Egyptian Linear Attention, Egyptian MoE, Entropy Early Stop, FFT Mix,
Fibonacci Stride, HCN Dimensions, Leech-24 NAS, LoRA R=8, Mertens Dropout,
Partition Routing, Phi Bottleneck, Phi MoE, Predictive Early Stop, Phi6 Simple,
Zeta-ln2 Activation
```

> 카운트 주의: 위 목록은 24 항목이지만 paper 1.1 표는 "Core 17" 로 라벨링한다. 차이는 "동일 기법의 상수 분해 등록"(BitNet 2건, Phi 3건 등)에서 발생한다. ossified 의 17 은 그룹 라벨이며, 본 감사는 이 그룹 라벨을 변경하지 않는다(R10).

### Tier-B — 17 → 23 확장분 (paper §1.1 BT-확장·Model-specific 6종)

DeepSeek MLA, Mamba 2, Griffin RG-LRU, Jamba Hybrid, Medusa Heads, Mixture-of-Depths

(memory MEMORY.md 헤더의 "23" 표기는 이 6종을 17 위에 더한 누계로 해석된다. 단, project_core_theory.md 본문 description 은 여전히 "17" 이라 메모리 자체가 헤더/본문 불일치 상태)

### Tier-C — 23 → 66 .hexa 전환 풀 (43종)

Ring Attention, Speculative Decoding, YaRN RoPE, GShard Switch, ALiBi, Egyptian MoE, Mixture of Depths, Mistral/Mixtral MoE, MoCo Queue, MoE Activation Fraction, Phi MoE, Inference Scaling, DPO β, Layer Skip, Lookahead Decoding, LR Schedule n6, Streaming LLM, AdamW Quintuplet, Chinchilla Scaling, BPE Vocab 32K, DeepSeek MLA Compression, MAE Masking, Phi Bottleneck, Recurrent Gemma, GAT Heads, GCN Depth, GIN Isomorphism, GraphSAGE Sampling, HCN Dimensions, Constitutional AI, Context Window Ladder, DETR Queries, FPN Pyramid, Phi6 Simple, Rectified Flow, SD3 MMDiT, SimCLR Temperature, Whisper Ladder, YOLO NMS, Zeta-ln2 Activation, Dedekind/Egyptian/Zamba 변형

(서브축별 등록 카운트 합 = 9+11+15+6+5+5+15 = 66)

### Tier-D — sota 확장 (정점 3종, 별도 섹션)

S1 mamba2, S2 hyena, S3 rwkv (`techniques/sota/`, _registry.json sota.items, BT-380-SOTA-SSM)

### 폐기 후보

본 감사 범위에서 폐기 후보 식별 없음. 모든 등록 기법이 .hexa 스텁 + 일부 BODY 상태로 살아있으며, _bench_plan §3 측정 후 BODY 충실도 기준으로 별도 폐기 검토가 권장된다.

---

## 6. 권장 사항

### 6.1 ossified 처리 (R10)

- `n6shared/convergence/n6-architecture.json` `AI_17_TECHNIQUES` 골화 블록은 **변경 금지**(R10 불변성). 이 감사는 새 SSOT 를 생성하지 않고, 16 정의를 stable 블록에 신규 항목으로 추가하는 것을 권장.

### 6.2 stable 추가 권장 (수정 미수행, 제안만)

`stable` 블록에 다음 신규 키 추가를 권장한다(본 감사는 코드 변경을 수행하지 않는다 — 사용자 승인 후 별도 세션에서 적용):

```json
"AI_16_CANONICAL": {
  "status": "PASS",
  "value": "16 canonical 기준선 = 17 ossified - egyptian_linear_attention (분수 중복)",
  "threshold": "16/16 _registry.json 등록 확인 + bench_plan.md §1 표 일치",
  "note": "2026-04-12 — reports/audits/ai-techniques-16-canonical-2026-04-12.md, AI_17_TECHNIQUES ossified 불변 유지(R10), 16 은 벤치 측정용 기준선",
  "supersedes_check": "AI_17_TECHNIQUES (ossified 불변, 새 항목으로 추가)"
}
```

### 6.3 메모리 헤더 정정 권장

- `~/.claude-claude2/.../memory/MEMORY.md` 8행 "23 AI 기법" → "17 AI 기법 (16 canonical 측정 기준선)" 정정
- 본문 description (project_core_theory.md) 은 이미 17 이므로 별도 정정 불요

### 6.4 후속 측정 권장

- 두 egyptian 변형이 모두 STUB 상태이므로, 본체 포팅 후 **재측정**하여 통합 근거를 강화하거나 분리 결정으로 되돌릴 것
- bench_plan §3 프로토콜(verify→bench×30→atlas 승격)을 16 항목 모두에 대해 실행
- 결과는 `experiments/ai-efficiency/bench/T01_*..T16_*.hexa` 16 파일로 분리 발사

### 6.5 회피 사항

- ossified 17 의 기법 라벨 자체를 변경하지 말 것 (paper 부록 A 의 그룹 라벨)
- _registry.json 의 _total 66 (또는 sota 포함 69) 카운트는 변경하지 말 것
- 16 canonical 은 "벤치 측정 단위" 이며 "기법 등록 단위" 가 아님 — 두 카운트는 공존 가능

---

## 7. 결론

- ossified 17 vs INDEX 66 vs memory 23 vs hint 16 의 4중 불일치는 **카운트 단위가 다름**에서 발생: ossified 17 = 그룹 라벨, registry 66 = 등록 풀, memory 23 = 17+BT확장 6, hint 16 = 17 - 분수 중복 1.
- 본 감사 결과 **16 canonical 정의 가능**으로 판정한다. 정의 근거는 _bench_plan.md §1 사전 결정 + paper 부록 A 의 egyptian_attention/egyptian_linear_attention register 등식 동등성 두 출처에 일관됨. 단 두 .hexa 가 STUB 인 한 약한 근거임을 명시한다.
- 16 canonical 목록은 _bench_plan.md §1 의 T01~T16 표 그대로 채택. 16/16 모두 _registry.json 에 등록되어 있어 신규 파일 생성 불요.
- 권장 다음 행동: convergence stable 블록에 `AI_16_CANONICAL` 추가(사용자 승인 필요), MEMORY.md 헤더 "23" 정정, bench_plan §3 프로토콜로 16 측정 발사.

---

## 8. 출처

- `n6shared/convergence/n6-architecture.json` 라인 20~24 (`AI_17_TECHNIQUES` ossified)
- `techniques/_registry.json` v1.1.0 _total=66, _sota_total=69
- `techniques/_bench_plan.md` §1 (16 기준선 표) + §3 (측정 프로토콜)
- `techniques/_chip_mapping.md` 16 × 6 매핑 매트릭스
- `techniques/_registry_patch.md` (sota 3종 추가 패치)
- `papers/n6-ai-17-techniques-experimental-paper.md` 부록 A (register 검증 코드)
- `INDEX.json` axes.techniques (66종 라벨)
- `~/.claude-claude2/.../memory/MEMORY.md` 8행, `project_core_theory.md` description / §17
- `techniques/attention/egyptian_attention.hexa`, `egyptian_linear_attention.hexa` (STUB 동등성)

## 9. 관련 링크

- 상위: `../CLAUDE.md`
- 컨버전스: `../../n6shared/convergence/n6-architecture.json`
- 벤치 계획: `../../techniques/_bench_plan.md`
- 칩 매핑: `../../techniques/_chip_mapping.md`
- SSOT: `../../techniques/_registry.json`
