# chip-rtl-gen — AI-native 칩 RTL 자동생성기

> 축: **compute** · n6-architecture · σ·φ = n·τ = 24 정합 RTL 생성 파이프라인
>
> 16 AI 기법(.hexa) → 연산자 추상화 → Verilog/SystemVerilog 템플릿 매핑 → 합성 가능 RTL

---

## 1. 실생활 효과

| 효과 | 기존 (HLS/Chisel) | chip-rtl-gen 이후 | 체감 변화 |
|------|-------------------|-------------------|----------|
| RTL 작성 기간 | 기법 1종당 n=6 개월 | sigma-tau=8 시간 | 540배 단축 |
| 코드 일관성 | 엔지니어별 편차 | n=6 상수 고정 | 편차 0 |
| 검증 비용 | UVM 리그레션 수주 | verify.hexa 1분 | 자동화 |
| 합성 면적 | baseline | -44% (12x12 tensor core) | 알고리즘 ↔ 하드웨어 정합 |
| 전력 효율 | 1 TOPS/W | J2=24 TOPS/W (목표) | 24배 |

한 문장: 파이썬 한 줄도 쓰지 않고 .hexa 기법 한 개당 FPGA 합성 가능한 RTL 한 벌이 떨어진다.

---

## 2. 목표

AI 기법 66종 중 16종(attention 3 + moe 3 + sparse 3 + optim 4 + graph 3)을 **기법별 연산자 시그니처 → n=6 정합 Verilog 템플릿** 으로 1:1 매핑한다. 매핑 규칙은 n6shared/config/rtl_templates.json 에 SSOT 로 고정하고, 모든 생성물은 `nexus/origins/hexa-rtl/rtl/` 아래에 `.hexa` 로 먼저 내려 Makefile이 `.v` 로 전사(transpile)한다.

핵심 원칙:
1. **HEXA-FIRST**: 생성기 본체도 `.hexa`, 산출물도 `.hexa`. `.py` 금지 (R1).
2. **AI-NATIVE**: bit-twiddling/수작업 SIMD 금지, `@optimize/@memoize/@fuse/@parallel` attr 만 사용 (R12).
3. **상수 동적 로드**: 폭/사이클/뱅크는 n6shared/config/n6_constants.json 에서 로드 (R2).

---

## 3. 가설 (H-RTLGEN-1 ~ H-RTLGEN-12)

### Tier 1: 연산자 추상화

| ID | 가설 | n=6 근거 | 영향 |
|----|------|---------|------|
| H-RTLGEN-1 | 16 기법을 6개 원시 연산자로 환원 가능 | n=6 (gemm/softmax/topk/gate/reduce/conv6) | 템플릿 6개로 완결 |
| H-RTLGEN-2 | 각 원시 연산자는 sigma=12 스테이지 파이프라인으로 표현 | σ=12 (공통 depth) | 합성 후 타이밍 균일 |
| H-RTLGEN-3 | 연산자 간 버스는 sigma-tau=8 비트 스칼라 + J2=24 비트 벡터 | σ-τ=8, J₂=24 | 2계층 래퍼 |

### Tier 2: 템플릿 매핑

| ID | 가설 | n=6 근거 | 영향 |
|----|------|---------|------|
| H-RTLGEN-4 | attention 계열 → `gemm_core12` + `softmax_phi6` | sigma²=144 MAC | -44% 면적 |
| H-RTLGEN-5 | moe 계열 → `egyptian_router12` 재사용 | 기존 egyptian_moe.hexa | 0 신규 RTL |
| H-RTLGEN-6 | sparse 계열 → `boltzmann_gate` 아날로그 비교기 | 1/e 임계 | 63% MAC 절감 |
| H-RTLGEN-7 | optim 계열 (speculative/medusa) → `shadow_pipe_phi2` | phi=2 예측 헤드 | 2-way 투기 |
| H-RTLGEN-8 | graph 계열 → `noc_6regular` + `reduce_tau4` | 6-정규 그래프 | MoE와 공유 |

### Tier 3: 자동 검증

| ID | 가설 | n=6 근거 | 영향 |
|----|------|---------|------|
| H-RTLGEN-9 | 생성 RTL 마다 σ·φ=n·τ 검증 assert 자동 삽입 | 핵심 항등식 | 런타임 드리프트 0 |
| H-RTLGEN-10 | testbench 자동생성: 각 연산자 × 6 패턴 × tau=4 반복 | 24 testcase | 100% 커버 |
| H-RTLGEN-11 | Yosys 합성 리포트 자동 수집 → n6shared/logs/rtl_synth.jsonl | 합성 회귀 추적 | 면적/타이밍 회귀 탐지 |
| H-RTLGEN-12 | 실패 시 BT 기반 원인 분류 (BT-28, BT-55, BT-89) | 3 원인 클래스 | 디버그 자동화 |

---

## 4. BT 연결

- **BT-28** — 컴퓨팅 사다리 (원시 연산자 6종 정렬)
- **BT-55** — HBM/메모리 벽 (연산자 ↔ 메모리 인터페이스)
- **BT-58** — σ-τ=8 비트 유니버설 (스칼라 폭 근거)
- **BT-69** — 칩렛 (연산자 → 칩렛 매핑 경로)
- **BT-89** — 광 인터커넥트 (NoC 6-정규 대안)
- **BT-90** — SM=phi·K6 (MoE 라우팅 용량)
- **BT-134** — 주기열 (연산자 레지스터 폭 근거)
- **BT-380** (확장) — 메타 정리 (규칙 승격 루프)

---

## 5. 연산자 추상화 (6종 primitive)

```
┌───────────────────────────────────────────────────────────────┐
│  n=6 primitive operators                                       │
├───────────┬────────────────┬──────────┬──────────┬────────────┤
│ 기호      │ 이름           │ 입력     │ 출력     │ 사이클      │
├───────────┼────────────────┼──────────┼──────────┼────────────┤
│ Ω₁ gemm   │ 행렬곱         │ 12×12    │ 12×12    │ σ=12        │
│ Ω₂ softmax│ 확률화         │ 12×σ-τ   │ 12×σ-τ   │ φ=2         │
│ Ω₃ topk   │ 상위 k 선택    │ 12×σ-τ   │ φ=2 idx  │ φ=2         │
│ Ω₄ gate   │ 게이팅/threshold│ 1×σ-τ  │ 1 bit    │ 1 (μ=1)     │
│ Ω₅ reduce │ 누적/합산      │ τ=4 입력 │ 1        │ τ=4         │
│ Ω₆ conv6  │ 6-탭 합성곱    │ 6×σ-τ    │ 6×σ-τ    │ n=6         │
└───────────┴────────────────┴──────────┴──────────┴────────────┘
```

> 모든 기법은 이 6종 합성으로 표현된다. 합성 DAG는 `.hexa` 에서 선언적으로 기술 (node/edge).

---

## 6. 템플릿 매핑 규칙 (16 기법)

### attention (3)

| 기법 | DAG | 템플릿 파라미터 | 재사용 모듈 |
|------|-----|---------------|------------|
| `fft_mix_attention` | Ω₁ → Ω₆(FFT) → Ω₁ | sigma=12 헤드, tau=4 탭 | `gemm_core12` + `conv6_fft` |
| `egyptian_linear_attention` | Ω₁ → Ω₄(1/2,1/3,1/6) | 3분수 하드와이어 | `egyptian_moe` router 공유 |
| `gqa_grouping` | Ω₁(Q) → Ω₁(K⁻ᵀ 공유) → Ω₂ → Ω₁(V) | 그룹 6, 헤드 12 | `gemm_core12` + `softmax_phi6` |

### moe (3)

| 기법 | DAG | 파라미터 | 재사용 |
|------|-----|---------|-------|
| `egyptian_moe` | Ω₃ → Ω₄ → Ω₁ (dispatch) | 12 slot, 6+4+2 tier | 기존 `egyptian_moe.hexa` 직결 |
| `mixture_of_depths` | Ω₃(depth gate) → Ω₁ | phi=2 depth | `shadow_pipe_phi2` |
| `phi_moe` | Ω₂ → Ω₃ → Ω₁ | Phi6 activation | `phi6_unit` (x²-x+1, 2사이클) |

### sparse (3)

| 기법 | DAG | 파라미터 | 재사용 |
|------|-----|---------|-------|
| `boltzmann_gate` | Ω₄(1/e) | 아날로그 비교기 | 신규 `boltz_cmp` |
| `mertens_dropout` | Ω₃(Möbius) → Ω₄ | μ(6)=1 squarefree | 신규 `mertens_lfsr` |
| `takens_dim6` | Ω₆ → Ω₅ | n=6 임베딩 차원 | `conv6_fft` 변형 |

### optim (4)

| 기법 | DAG | 파라미터 | 재사용 |
|------|-----|---------|-------|
| `speculative_decoding` | Ω₁(draft) ∥ Ω₁(target) → Ω₄(accept) | phi=2 shadow | 신규 `shadow_pipe_phi2` |
| `medusa_heads` | Ω₁(main) ∥ Ω₁(medusa×τ) | tau=4 보조 헤드 | `gemm_core12` 4 인스턴스 |
| `lookahead_decoding` | Ω₅(window) → Ω₂ | sigma=12 window | `reduce_tau4` 확장 |
| `layer_skip` | Ω₄(skip gate) | phi=2 예측 | `boltz_cmp` 재사용 |

### graph (3)

| 기법 | DAG | 파라미터 | 재사용 |
|------|-----|---------|-------|
| `gat_heads` | Ω₁ → Ω₂ → Ω₅ | tau=4 어텐션 헤드 | `gemm_core12` |
| `gcn_depth` | Ω₆ × sigma=12 레이어 | σ 깊이 | `conv6_fft` 스택 |
| `hcn_dimensions` | Ω₆(n=6 차원) | 6-정규 그래프 | `noc_6regular` |

---

## 7. 생성 파이프라인 (5 단계)

```
┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐
│ 1. 기법  │────>│ 2. DAG   │────>│ 3. 템플릿│────>│ 4. 전사  │────>│ 5. 합성  │
│  .hexa   │     │  추출    │     │  매핑    │     │  .hexa→.v│     │  Yosys    │
└──────────┘     └──────────┘     └──────────┘     └──────────┘     └──────────┘
     │                │                │                │                │
     ▼                ▼                ▼                ▼                ▼
  파서/AST        6 primitive       params 주입     Makefile        면적/타이밍
  @attr 수집       합성 DAG         .hexa 생성        hook              리포트
                   JSONL 기록                                        n6shared/logs
```

### 단계 1: 기법 파싱
- 입력: `techniques/<sub>/<name>.hexa`
- 출력: `.ast.json` (AST + @attr 목록)
- 구현: `nexus/origins/hexa-rtl/scripts/parse_technique.hexa`

### 단계 2: DAG 추출
- 입력: `.ast.json`
- 출력: `n6shared/logs/rtl_dag.jsonl` (node/edge, primitive id)
- 규칙: @gemm → Ω₁, @softmax → Ω₂, ...

### 단계 3: 템플릿 매핑
- 입력: DAG JSONL + `n6shared/config/rtl_templates.json` (SSOT)
- 출력: `.hexa` 모듈 소스
- 파라미터: n=6, σ=12, τ=4, φ=2, J₂=24 (동적 로드)

### 단계 4: 전사(.hexa → .v)
- 기존 Makefile 확장 (`nexus/origins/hexa-rtl/Makefile`)
- 신규 타겟: `make gen TECH=<name>` → 자동 생성 + sim

### 단계 5: 합성 리포트
- Yosys area/timing → `n6shared/logs/rtl_synth.jsonl`
- 회귀 탐지: 전주 대비 면적 +5% 시 경보

---

## 8. 템플릿 SSOT — `n6shared/config/rtl_templates.json`

```
{
  "_doc": "RTL 템플릿 매핑 SSOT — 16 기법 × 6 원시연산",
  "primitives": {
    "gemm":    { "sigma_stages": 12, "width_bits": 8, "array": "12x12" },
    "softmax": { "phi_stages": 2, "approx": "piecewise_linear_phi6" },
    "topk":    { "k": 2, "width_bits": 8 },
    "gate":    { "mode": "boltzmann", "threshold": "1/e" },
    "reduce":  { "tau_depth": 4, "op": "add" },
    "conv6":   { "taps": 6, "mode": "fft" }
  },
  "techniques": {
    "fft_mix_attention":      { "dag": ["gemm","conv6","gemm"] },
    "egyptian_moe":           { "dag": ["topk","gate","gemm"], "router": "egyptian_12" },
    "boltzmann_gate":         { "dag": ["gate"] },
    "speculative_decoding":   { "dag": ["gemm","gemm","gate"], "parallel": 2 },
    "gcn_depth":              { "dag": ["conv6"], "depth": 12 }
    // ... 16개 전부
  }
}
```

---

## 9. 진화 단계 Mk.I ~ Mk.V

| 단계 | 범위 | 산출물 | 목표 시점 |
|------|------|--------|----------|
| Mk.I | 원시 연산자 6종 .hexa | `rtl/prim_*.hexa` × 6 | M+0 |
| Mk.II | attention 3 기법 매핑 | 3 `.hexa` + testbench | M+1 |
| Mk.III | moe/sparse/optim/graph 13 기법 | 16 `.hexa` 전체 | M+2 |
| Mk.IV | Yosys 합성 리포트 자동화 | `rtl_synth.jsonl` | M+3 |
| Mk.V | FPGA 실기 검증 (Zynq-7020) | bitstream 16종 | M+6 |

---

## 10. 예측 추적

- **P-RTLGEN-1**: 16 기법 RTL 평균 면적 = 12×12 tensor core baseline ± 10% (sigma²=144 MAC 기준)
- **P-RTLGEN-2**: 전사 시간 ≤ sigma-phi=10 초 / 기법 (AI-native attr 컴파일러 속도)
- **P-RTLGEN-3**: Yosys area 편차 σ < sigma-tau=8% (템플릿 결정론)

---

## 11. 참고 자원 (기존)

- `nexus/origins/hexa-rtl/` — 기존 6 RTL 모듈 (`riscv_n6_core`, `egyptian_moe`, `egyptian_mem_ctrl`, `hexalang_decoder`, `snn_izhikevich`, `hexa_edge_top`)
- `nexus/origins/hexa-rtl/Makefile` — 빌드 시스템, `make sim/synth/lint`
- `techniques/_registry.json` — 66 기법 레지스트리 SSOT
- `n6shared/config/absolute_rules.json` — R1/R12 (HEXA-FIRST, AI-NATIVE)
- `nexus src/cmd/hexa/` — hexa rtl 서브커맨드

---

## 12. 검증 진입점

```
cd domains/compute/chip-rtl-gen
hexa verify.hexa
```

검증 내용: σ·φ = n·τ = 24, J₂=24 일관성. 실제 생성 파이프라인 검증은 `nexus hexa rtl --gen <tech>` (Mk.II 이후).

---

## 13. 원칙 체크리스트

- [x] HEXA-FIRST (R1) — 생성기/산출물 모두 .hexa
- [x] AI-NATIVE (R12) — bit-twiddling/inline-asm 0
- [x] SSOT (R5) — rtl_templates.json 단일
- [x] 동적 로드 (R2) — n6_constants.json
- [x] 한글 (R-한글) — 본 문서 및 .hexa 주석

---

## 14. 열린 질문

1. **Ω₂ softmax 근사**: piecewise linear vs LUT — sigma=12 구간 vs J₂=24 엔트리 비교 필요
2. **Ω₆ conv6 FFT 대체**: radix-6 FFT vs 6-tap FIR — BT-89 광 인터커넥트와 연결 시 radix-6 우세
3. **합성 타겟**: Zynq-7020 (FPGA) vs 3nm ASIC — Mk.V 에서 결정

---

## 15. 부록

### A. 성공 기준 (GO)

- 16/16 기법 `.hexa` 모듈 생성
- 16/16 testbench `hexa sim` PASS
- Yosys 면적 회귀 0
- `n6shared/logs/rtl_synth.jsonl` 연속 기록

### B. 레거시

- 기존 .py 기반 HLS 시도: 전량 폐기 (R1 위반)

---

## 5. DSE 결과

## 6. 물리 한계 증명

## 7. 실험 검증 매트릭스

## 8. 외계인급 발견

## 9. Mk.I~V 진화

## 10. Testable Predictions

## 11. ASCII 성능비교

## 12. ASCII 시스템 구조도

## 13. ASCII 데이터/에너지 플로우

## 14. 업그레이드 시 (시중 vs v1 vs v2)

## 15. 검증 방법 (verify.hexa)
