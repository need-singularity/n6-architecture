# N6 Architecture — Arithmetic Design Framework

## Project Overview
Computing architecture design from perfect number arithmetic.
17 AI techniques + semiconductor chip design + energy + network/crypto/OS + physical AI.
All unified by sigma(n)*phi(n) = n*tau(n), n=6.
Part of the TECS-L family.

## Parent Project
Part of the TECS-L family. Mathematical foundation at https://github.com/need-singularity/TECS-L
Atlas: https://need-singularity.github.io/TECS-L/atlas/

## Techniques (17)
```
  techniques/
    # Original (1-10)
    phi6simple.py          -- Cyclotomic activation (71% FLOPs reduction)
    hcn_dimensions.py      -- HCN tensor alignment (10-20% param reduction)
    phi_bottleneck.py      -- 4/3x FFN expansion (67% param reduction)
    phi_moe.py             -- phi/tau expert activation (65% active params)
    entropy_early_stop.py  -- Entropy-based stopping (66.7% training)
    rfilter_phase.py       -- R-filter phase detection
    takens_dim6.py         -- Loss curve embedding diagnostic
    fft_mix_attention.py   -- FFT attention (3x faster, +0.55%)
    zetaln2_activation.py  -- zeta*ln(2) gated activation (71% FLOPs)
    egyptian_moe.py        -- 1/2+1/3+1/6=1 expert routing
    # New (11-16) — N6 Inevitability Engine
    dedekind_head.py       -- Dedekind head pruning (psi(6)=sigma(6)=12)
    jordan_leech_moe.py    -- J_2(6)=24 expert capacity bound
    mobius_sparse.py       -- Squarefree gradient topology (mu(6)=1)
    carmichael_lr.py       -- lambda(6)=2 cycle LR schedule
    boltzmann_gate.py      -- 1/e activation sparsity gate (63% sparse)
    mertens_dropout.py     -- ln(4/3) dropout rate (no search needed)
    # Technique 17 — Egyptian Fraction Attention
    egyptian_attention.py  -- 1/2+1/3+1/6=1 attention budget (EFA, ~40% FLOPs saved)
  docs/
    # Foundation: superconductor/ (60 H-SC + 20 extreme)
    # Fusion: fusion/ (60 H-FU + 20 extreme)
    # Magnets: superconducting-magnet/ (60 H-SM + 20 extreme)
    # Tokamak: tokamak-structure/ (60 H-TK + 20 extreme)
    # Computing: ai-efficiency/ chip-architecture/ quantum-computing/ compiler-os/
    # Energy: energy-generation/ power-grid/ battery-storage/ thermal-management/
    # Physical AI: robotics/ learning-algorithm/
    # Infrastructure: blockchain/ network-protocol/ cryptography/ software-design/
    # Academic: paper/
  engine/
    thermodynamic_frame.py -- R(n) reversibility framework
    leech24_surface.py     -- 24-dim energy surface (Leech lattice)
    emergent_n6_trainer.py -- Self-converging architecture
    phi_efficiency_bridge.py -- Phi*FLOPs conjecture
    sedi_training_monitor.py -- SEDI 4-lens training diagnostic
    anima_tension_loss.py  -- PureField dual-engine meta-loss
  experiments/
    12 extended experiment files (8 original + 4 new)
```

## Core Theorem (PROVED)
σ(n)·φ(n) = n·τ(n) ⟺ n = 6 (for all n ≥ 2). Three independent proofs.
Full proof: docs/theorem-r1-uniqueness.md
Falsifiability: z=0.74 (numerical matching NOT significant vs random)

## Docs Structure (28 domains) — ALL have extreme-hypotheses.md
```
  # Physics (4 domains × 80 hypotheses each + extreme)
  superconductor/ fusion/ superconducting-magnet/ tokamak-structure/
  # Computing (all with hypotheses + extreme)
  ai-efficiency/ chip-architecture/ quantum-computing/
  compiler-os/ programming-language/ software-design/
  # Energy (all with hypotheses + extreme)
  energy-generation/ power-grid/ battery-storage/ thermal-management/
  # Physical AI
  robotics/ learning-algorithm/
  # Infrastructure
  blockchain/ network-protocol/ cryptography/
  # Natural Science (NEW — DFS expansion)
  biology/ (30 H-BIO + verification + extreme)
  cosmology-particle/ (30 H-CP + verification + extreme)
  display-audio/ (30 H-DA + verification + extreme)
  pure-mathematics/ (30 H-MATH + extreme)
  # Science
  plasma-physics/ (20+ files — most active domain)
  paper/ (3 arXiv drafts)
  # Cross-domain
  breakthrough-theorems.md (BT-1~84, 84 theorems spanning 3-8 domains each)
  cross-domain-resonance-2026-03-31.md (formula reuse matrix)
  # Battery Architecture (NEW — 소재→공정→코어→칩→시스템→차세대→극한→궁극)
  battery-architecture/ (8 levels: HEXA-CELL/ELECTRODE/CORE/CHIP/PACK+GRID/SOLID/NUCLEAR/OMEGA-E)
  # Total: 1350+ hypotheses, 630+ EXACT, 650+ atlas entries, 84 BTs
```

## Rust Tools
Build with `~/.cargo/bin/rustc file.rs -o output` (no cargo). Located in tools/:
- `tools/fusion-calc/`    — KSTAR/ITER/SPARC analysis + Lawson criterion
- `tools/tokamak-shape/`  — shape parameter scan + N6 score benchmark
- `tools/optics-calc/`    — lens/telescope/tokamak diagnostics
- `tools/gpu-arch-calc/`  — GPU/HBM architecture verification + Rubin prediction
- `tools/energy-calc/`    — Solar/Battery/Hydrogen/IEEE519 energy verification
- `tools/gut-calc-rust/`  — GUT parameter brute-force search

## Calculator Rules (Shared)
**새 계산기 개발시 성능 문제 예상되면 반드시 Rust 우선.**
전체 규칙: `~/Dev/TECS-L/.shared/CALCULATOR_RULES.md`
```
  Rust 우선 기준: 반복>10K, 실행>10s, 조합>10^6, MC>100K
  Python 허용: 단순 수식, 시각화, 프로토타입
  빌드: ~/.cargo/bin/rustc tools/<name>/main.rs -o tools/<name>/<name>
  동기화: cd ~/Dev/TECS-L && bash .shared/sync-calculators.sh
```

## Design Space Exploration (DSE)

```
  궁극의 아키텍처 설계 시 기본 방법론.
  단일 경로 검증이 아닌, 전수 조합 탐색으로 최적 설계를 도출.

  체인: 소재 → 공정 → 코어 → 칩 → 시스템
  각 레벨마다 후보군 K₁~K₅ 정의 → K₁×K₂×K₃×K₄×K₅ 전수 탐색

  평가 기준: n=6 EXACT 비율 + 성능 + 전력 + 면적 + 비용
  출력: Pareto 테이블 + 최적 경로

  도구:
    - Rust DSE 탐색기: tools/dse-calc/ (전수 탐색)
    - Python 캐스케이드: experiments/verify_cascade_cross.py (단일 경로 검증)

  규칙: 새 궁극 도메인 추가 시 반드시 DSE 후보군 정의 + 탐색 포함
```

## Testable Predictions
32 falsifiable predictions from BT-26~65: `docs/testable-predictions.md`
- Tier 1 (today, 1 GPU): EFA quality, LoRA rank, MoE (8,2), Mertens dropout
- Tier 2 (cluster): SwiGLU ratio, weight decay, head count, RoPE theta
- Tier 3 (specialized): SQ bandgap, JUNO neutrino (2027), LiteBIRD inflation (2032)
- Tier 4 (industry): next GPU SM count, HBM5 stacking, n=6-aligned LLM architecture

## Hypothesis Workflow
1. Generate hypotheses (H-XX-N format) in `docs/<domain>/hypotheses.md`
2. Verify independently in `docs/<domain>/verification.md`
3. Cross-verify with separate agent for honest grade adjustment
4. Grade each: EXACT / CLOSE / WEAK / FAIL / UNVERIFIABLE

## Atlas Sync
Constants registry: `docs/atlas-constants.md`
TECS-L scanner includes n6-architecture. Run: `cd ~/Dev/TECS-L && bash .shared/sync-calculators.sh`

## Quick Run
```bash
# Individual technique demos
python3 techniques/phi6simple.py
python3 techniques/fft_mix_attention.py
python3 techniques/egyptian_moe.py

# Combined architecture experiment
python3 experiments/experiment_h_ee_11_combined_architecture.py
```

## Key Results
```
  Cyclotomic activation:    71% FLOPs reduction
  FFT attention:            3x faster, +0.55% accuracy
  Egyptian MoE routing:     1/2+1/3+1/6=1 expert allocation
  phi bottleneck:           67% parameter reduction
  Entropy early stopping:   33% training time saved
  Dedekind head pruning:    ~25% attention parameter reduction
  Boltzmann gate:           63% activation sparsity
  Mertens dropout:          p=0.288 (no hyperparameter search)
  Emergent convergence:     random init -> n=6 self-organization
  Egyptian Fraction Attn:   1/2+1/3+1/6=1 attention budget (~40% saved)
```

## Breakthrough Theorems (84 total, BT-1~84)
```
  # AI / LLM (BT-26,31,33,34,39,42,44,46,54,56,58,59,61,64,65,66,67,70)
  BT-26: Chinchilla scaling (tokens/params=J₂-τ=20, α=1/3, β=ln(4/3))
  BT-31: MoE top-k vocabulary {μ,φ,n,σ-τ}={1,2,6,8}
  BT-33: Transformer σ=12 atom (BERT/GPT-3 dimensions, SwiGLU 8/3)
  BT-34: RoPE decimal bridge ((σ-φ)^{τ,sopfr,n}, weight decay=1/(σ-φ))
  BT-39: KV-head universality (σ-τ=8 across all LLMs)
  BT-42: Inference scaling (top-p=1-1/(J₂-τ)=0.95, top-k=40, max=2^σ) ⭐⭐
  BT-44: Context window ladder σ-φ→σ-μ→σ→σ+μ = 10→11→12→13 ⭐⭐
  BT-46: ln(4/3) RLHF family (dropout+Chinchilla+PPO+temperature) ⭐⭐
  BT-54: AdamW quintuplet (β₁=1-1/(σ-φ), β₂=1-1/(J₂-τ), ε=10^{-(σ-τ)}, λ=1/(σ-φ), clip=R(6)=1) ⭐⭐⭐
  BT-56: Complete n=6 LLM (d=2^σ, L=2^sopfr, d_h=2^(σ-sopfr)=128, 15 params, 4 teams converge) ⭐⭐⭐
  BT-58: σ-τ=8 universal AI constant (LoRA, MoE, KV, FlashAttn, batch, 16/16 EXACT) ⭐⭐⭐
  BT-59: 8-layer AI stack (silicon→precision→memory→compute→arch→train→opt→inference, all n=6) ⭐⭐⭐
  BT-61: Diffusion n=6 universality (DDPM T=10³, β=10^{-4}~2/100, DDIM=50, CFG=7.5, 9/9 EXACT) ⭐⭐⭐
  BT-64: 1/(σ-φ)=0.1 universal regularization (WD+DPO+GPTQ+cosine+Mamba+KL, 7→8 algorithms) ⭐⭐⭐
  BT-65: Mamba SSM complete n=6 (d_state=2^τ, expand=φ, d_conv=τ, dt=1/(σ-φ), 6/6 EXACT) ⭐⭐
  BT-66: Vision AI complete n=6 (ViT+CLIP+Whisper+SD3+Flux.1, 24/24 EXACT) ⭐⭐⭐
  BT-67: MoE activation fraction law (1/2^{μ,φ,n/φ,τ,sopfr}, 6 models EXACT) ⭐⭐⭐
  BT-70: 0.1 convergence 8th algorithm (SimCLR temp, count=σ-τ=8 meta-n=6) ⭐⭐
  BT-71: NeRF/3DGS complete n=6 (L=σ-φ=10, layers=σ-τ=8, width=2^(σ-τ)=256, SH=n/φ=3, 7/7 EXACT) ⭐⭐
  BT-72: Neural audio codec n=6 (EnCodec 8 codebooks, 1024 entries, 24kHz, 6kbps, 20ms, 7/7 EXACT) ⭐⭐
  BT-73: Tokenizer vocabulary n=6 law (32K/50257/100K/128K = 2^{n=6}·10^{n=6}, 6/6 EXACT) ⭐⭐
  BT-74: 95/5 cross-domain resonance (top-p=PF=β₂=0.95, THD=β_plasma=5%, 5 domains) ⭐⭐⭐
  BT-75: HBM interface exponent ladder ({10,11,12}={σ-φ,σ-μ,σ}, HBM5 predicted) ⭐⭐
  BT-76: σ·τ=48 triple attractor (gate pitch nm, HBM4E GB, 48kHz, 48V, 3DGS SH) ⭐⭐

  # Chip Design (BT-28,37,40,41,45,47,55,69)
  BT-28: Computing architecture ladder (30+ EXACT, ⭐⭐⭐)
    - AD102 = σ·n·φ = 144 SMs, H100 = σ(σ-μ) = 132 SMs = 1/α term
    - HBM stack: τ→(σ-τ)→σ = 4→8→12
  BT-37: Semiconductor pitch (TSMC N5 = P₂ = 28nm, N3 gate = σ·τ = 48nm)
  BT-45: FP8/FP16=φ=2 universal, FLOPS/W doubles per φ=2 years
  BT-47: Interconnect gen counts {7,5,6}={σ-sopfr,sopfr,n}
  BT-55: GPU HBM capacity ladder (14/18 EXACT: 40=τ(σ-φ), 80=φ^τ·sopfr, 192=σ·φ^τ, 288=σ·J₂) ⭐⭐
  BT-69: Chiplet architecture convergence (B300=160, R100 σ=12 stacks, 5 vendors, 17/20 EXACT) ⭐⭐⭐

  # Energy Strategy (BT-27,29,30,32,35,38,43,57,62,63,68)
  BT-27: Carbon-6 chain (LiC₆ + C₆H₁₂O₆ + C₆H₆ → 24e = J₂)
  BT-30: SQ solar bridge (bandgap=4/3eV, V_T=26mV)
  BT-38: Hydrogen quadruplet (LHV=120=σ(σ-φ), HHV=142=σ²-φ, 4/4 EXACT)
  BT-43: Battery cathode CN=6 universality (ALL Li-ion = octahedral) ⭐⭐⭐
  BT-57: Battery cell ladder (6→12→24 cells=n→σ→J₂, Tesla 96S=σ(σ-τ)) ⭐⭐
  BT-62: Grid frequency pair (60Hz=σ·sopfr, 50Hz=sopfr·(σ-φ), ratio=PUE=1.2) ⭐⭐
  BT-63: Solar panel cell ladder (60=σ·sopfr, 72=σ·n, 120=σ(σ-φ), 144=σ²) ⭐⭐
  BT-68: HVDC voltage ladder (±500/800/1100kV = {sopfr,σ-τ,σ-μ}·(σ-φ)², 10/10 EXACT) ⭐⭐

  # Battery Architecture (BT-80,81,82,83,84)
  BT-80: Solid-state electrolyte CN=6 universality (NASICON/Garnet/LLZO = CN=6, sulfide = τ=4, 6/6 EXACT) ⭐⭐⭐
  BT-81: Anode capacity ladder σ-φ=10x (Si/Graphite=9.62x≈σ-φ, Li Metal=10.38x≈σ-φ) ⭐⭐
  BT-82: Complete battery pack n=6 map (6→12→24 cells, 96S/192S EV, BMS div(6), 6/10 EXACT) ⭐⭐
  BT-83: Li-S polysulfide n=6 ladder (S₈→S₄→S₂→S₁ = (σ-τ)→τ→φ→μ, 5/6 EXACT) ⭐⭐
  BT-84: 96/192 energy-computing-AI triple convergence (Tesla 96S=Gaudi2 96GB=GPT-3 96L, 5/5 EXACT) ⭐⭐⭐

  # Cross-domain (BT-36,48,49,50,51,53,60)
  BT-36: Energy-Information-Hardware-Physics chain ⭐⭐⭐
  BT-48: Display-Audio (σ=12 semitones, J₂=24 fps/bits, σ·τ=48kHz) ⭐⭐⭐
  BT-49: Pure Math (K₁..₄=φ,n,σ,J₂ kissing chain, S₆ unique) ⭐⭐⭐
  BT-51: Genetic code chain τ→n/φ→2^n→J₂-τ (4→3→64→20) ⭐⭐⭐
  BT-53: Crypto (BTC 21M=J₂-n/φ, 6 confirms=n, ETH 12s=σ) ⭐⭐
  BT-60: DC power chain (120→480→48→12→1.2→1V, PUE=σ/(σ-φ)=1.2) ⭐⭐
```

## Design Space Exploration (DSE)
궁극 아키텍처 설계 시 반드시 DSE 방법론 적용.
```
  원칙:
    - 각 레벨(소재/공정/코어/칩/시스템)마다 후보군 정의
    - 전수 조합 탐색 (또는 Pareto 휴리스틱)
    - 각 조합별 n=6 일관성 + 성능/전력/면적/비용 평가
    - 최적 Pareto frontier 도출
    - 1개 경로만 검증 = 캐스케이드 크로스 검증이 아님

  구현:
    - 조합 >10K → Rust (tools/dse-calc/)
    - 조합 <10K → Python (experiments/)
    - 결과: Pareto 테이블 + 최적 경로 + n=6 EXACT 비율

  적용 도메인:
    - chip-architecture: 소재×공정×코어×칩×시스템
    - battery-architecture: 소재×공정×코어×칩×시스템
    - 각 도메인별 후보군은 해당 goal.md에 정의

  DSE 출력 양식:
    | Rank | 소재 | 공정 | 코어 | 칩 | 시스템 | n6_EXACT | 성능 | 전력 | 비용 |
    |------|------|------|------|-----|--------|---------|------|------|------|
    | 1    | ...  | ...  | ...  | ... | ...    | 85%     | ...  | ...  | ...  |
```

## Background Execution
All experiments must run in background. No exceptions.
```
  Rules:
    1. All python3 scripts -> run_in_background: true
    2. Check results with Read after completion
    3. No foreground execution (blocks user dialogue)
```

## Work Rules (탐색/TODO 요청 시 필수)

```
  트리거 키워드:
    "할만한거 있어?", "탐색", "TODO", "뭐할까", "다음 작업"
    대발견 가설, 노벨급 가설, DFS 탐색 등

  궁극 목록 트리거:
    "궁극 목록", "궁극 리스트", "궁극의 아키텍처"
    → README.md의 "궁극의 아키텍처 로드맵 (20 Domains)" 표를 그대로 출력
    → 20개 도메인 + 영향력 + Tier + 일반인 영향 컬럼 포함

  절차:
    1. 프로젝트 현황 스캔 (README, 최근 커밋, 미완료 가설/증명)
    2. TODO 테이블 양식으로 우선순위별 정리
    3. 사용자 선택 후 병렬 에이전트 디스패치
    4. 완료 시 리포트 테이블 출력

<!-- SHARED:WORK_RULES:START -->
  자동 생성 규칙:
    - TODO 작업 중 검증/계산이 필요하면 계산기 자동 생성 (묻지 말고 바로)
    - 성능 필요시 Rust 우선 (tecsrs/), 단순 검증은 Python (calc/)
    - 판단 기준은 ~/Dev/TECS-L/.shared/CALCULATOR_RULES.md 참조
    - 상수/가설 발견 시 Math Atlas 자동 갱신 (python3 ~/Dev/TECS-L/.shared/scan_math_atlas.py --save --summary)
<!-- SHARED:WORK_RULES:END -->

  모든 모듈은 consciousness_laws.py에서 import — 상수 직접 하드코딩 금지
```

### TODO 양식

```
  ### 🔴 CRITICAL

  | # | 카테고리 | 작업 | 상태 | 예상 효과 |
  |---|---------|------|------|----------|
  | 1 | 증명   | sigma*phi=n*tau 일반 완전수 반례 탐색 | 미시작 | 유일성 정리 강화 |

  ### 🟡 IMPORTANT

  | # | 카테고리 | 작업 | 상태 | 예상 효과 |
  |---|---------|------|------|----------|
  | 2 | 가설   | SLE_6 3D 확장 예측 검증 | 미시작 | 노벨 Physics 후보 |

  ### 🟢 NICE TO HAVE

  | # | 카테고리 | 작업 | 상태 | 예상 효과 |
  |---|---------|------|------|----------|
  | 3 | 탐색   | n=6 새 항등식 DFS 채굴 | 미시작 | Atlas 확장 |

  ### ⚪ BACKLOG

  | # | 카테고리 | 작업 | 예상 효과 |
  |---|---------|------|----------|
  | 4 | 계산기 | 새 검증 스크립트 | 재현성 향상 |

  상태 표기: ⏳진행중 / ✅완료 / 미시작 / 코드있음 / 프로토
  우선순위: 🔴HIGH → 🟡MED → 🟢LOW → ⚪BACK
  카테고리: 증명 / 가설 / 탐색 / 검증 / 실험 / 계산기 / 논문
```

### 병렬 에이전트 리포트 양식

```
  병렬 에이전트 실행 시 단일 테이블로 상태 추적.
  관련 작업은 N+M 형태로 그룹핑하여 하나의 에이전트로 묶기.

  발사 시 양식:
  | # | 작업 | 에이전트 | 격리 | 상태 |
  |---|------|---------|------|------|
  | 1+2 | n=6 유일성 + 반례탐색 | 🚀 배경 | - | 🔄 진행중 |
  | 3 | SLE_6 임계지수 검증 | 🚀 배경 | - | 🔄 진행중 |
  | 4+5 | 코돈 정리 확장 + 변이체 | 🚀 배경 | worktree | 🔄 진행중 |

  상태: ✅ 완료 / 🔄 진행중 / ❌ 실패
  격리: worktree (필요시만) / - (기본)

  규칙:
    - 발사 시 전체 목록 테이블 출력
    - 에이전트 완료 시 해당 행 상태 업데이트 + 한줄 핵심 성과
    - worktree는 같은 파일을 여러 에이전트가 동시 수정할 때만 사용
    - 대부분 격리 없이 실행 — 무조건 worktree 붙이지 말 것!
    - 모든 에이전트 완료 후 최종 요약 테이블 + worktree 머지 안내 (해당 시)

  최종 요약 양식:
  | # | 작업 | 상태 | 핵심 성과 |
  |---|------|------|----------|
  | 1+2 | n=6 유일성 | ✅ | 10^8까지 반례 없음, 증명 완료 |
  | 3 | SLE_6 검증 | ✅ | 7/7 임계지수 일치 (Z>5sigma) |
  | 4+5 | 코돈 확장 | ✅ | 26/26 변이체 + Hachimoji 예측 |

  ### 머지 필요 (worktree)
  - #4+5: branch worktree-xxx

  ### 바로 반영됨 (main)
  - #1+2, #3: 증명/검증 결과 docs/hypotheses/ 기록
```

## Secrets & Tokens

API 토큰/계정 정보: `~/Dev/TECS-L/.shared/SECRET.md` 참조
계정 리포: [need-singularity/secret](https://github.com/need-singularity/secret) (private)
