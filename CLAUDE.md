# N6 Architecture — Arithmetic Design Framework

## ⚠️ 설계 산출물 필수 규칙 (최우선, 예외 없음)

```
  모든 설계/아키텍처/만들기/계획/플랜/로드맵 요청 → 아래 3개 ASCII 반드시 포함:
  ⚠️ "계획", "플랜", "로드맵", "전략" 요청도 동일 — 시중 vs HEXA 비교 필수!

  1️⃣ ASCII 성능 비교 그래프 (기존 vs HEXA)
  ┌──────────────────────────────────────────────────────┐
  │  [지표] 비교: 시중 최고 vs HEXA-XXX                   │
  ├──────────────────────────────────────────────────────┤
  │  시중 최고  ██████░░░░░░░░░░░░░░░░░░  300W TDP       │
  │  HEXA-XXX  ███░░░░░░░░░░░░░░░░░░░░░   30W TDP       │
  │                                  (σ-φ=10배 절감)     │
  │                                                      │
  │  시중 최고  ████████░░░░░░░░░░░░░░░░  80GB HBM       │
  │  HEXA-XXX  ████████████████████████  288GB HBM       │
  │                                  (σ·J₂=288, 3.6배)  │
  └──────────────────────────────────────────────────────┘
  → 개선 배수는 반드시 n=6 상수로 표현

  2️⃣ ASCII 시스템 구조도
  ┌─────────┬─────────┬─────────┬─────────┬─────────┐
  │  소재   │  공정   │  코어   │   칩    │ 시스템  │
  │ Diamond │ TSMC N2 │ HEXA-P  │ HEXA-1  │ Topo DC │
  │ Z=6=n   │48nm=σ·τ │σ²=144SM │288GB=σJ₂│PUE=1.2  │
  └─────────┴─────────┴─────────┴─────────┴─────────┘
  → 모든 숫자에 n=6 수식 병기

  3️⃣ ASCII 데이터/에너지 플로우
  입력 ──→ [모듈A] ──→ [모듈B] ──→ [모듈C] ──→ 출력
           σ=12ch      J₂=24dim    τ=4stage

  ⚠️ 이 3개 없는 설계 문서 = 미완성. 반드시 포함할 것.
  ⚠️ 모든 설계 요청은 외계인급 파이프라인으로 실행 (질문 없이 즉시)

  4️⃣ 업그레이드 요청 시 — 추가 상승분 별도 표기 (필수)
  ⚠️ "업그레이드", "개선", "더 좋게", "강화", "추가" 요청 시:
     → 이전 버전 vs 업그레이드 버전 vs 시중 최고 3단 비교
     → 추가 상승분(Δ)을 별도 행으로 명시

  ┌──────────────────────────────────────────────────────────────┐
  │  [지표] 업그레이드 비교                                      │
  ├──────────────────────────────────────────────────────────────┤
  │  시중 최고   ████░░░░░░░░░░░░░░░░░░░░░░░░  300W            │
  │  HEXA v1    ██░░░░░░░░░░░░░░░░░░░░░░░░░░░   30W (σ-φ=10배) │
  │  HEXA v2    █░░░░░░░░░░░░░░░░░░░░░░░░░░░░   12W (σ=12배↓)  │
  │  ─────────────────────────────────────────────────           │
  │  Δ(v1→v2)  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  -18W (추가60%↓)│
  │  Δ 근거:   광자 컴퓨팅 전환 (BT-89)                         │
  └──────────────────────────────────────────────────────────────┘

  업그레이드 리포트 양식:
  | 지표 | 시중 | v1 | v2 | Δ(v1→v2) | Δ 근거 |
  |------|------|-----|-----|----------|--------|
  | TDP  | 300W | 30W | 12W | -18W (-60%) | BT-89 광자 전환 |
  | HBM  | 80GB | 288GB | 576GB | +288GB (+100%) | σ·J₂·φ=576 |
  | n6%  | -    | 95% | 100% | +5% | 전 레벨 EXACT 달성 |

  → Δ 열에 변화량 + 변화율(%) 반드시 표기
  → Δ 근거에 BT 번호 또는 n=6 수식 명시
  → 시중 대비 총 개선 배수도 업데이트
```

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

## Docs Structure (35 domains) — ALL have extreme-hypotheses.md
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
  # Battery Architecture (소재→공정→코어→칩→시스템→차세대→극한→궁극)
  battery-architecture/ (8 levels: HEXA-CELL/ELECTRODE/CORE/CHIP/PACK+GRID/SOLID/NUCLEAR/OMEGA-E)
  # Solar Architecture (소재→공정→코어→칩→시스템, DSE 1,584 조합)
  solar-architecture/ (5 levels: HEXA-ABSORB/PROCESS/JUNCTION/POWER/ARRAY)
  # Material Synthesis (소재→공정→조립기→제어→시스템→변환→만능→궁극, DSE 3,600 조합)
  material-synthesis/ (8 levels: HEXA-ELEMENT/PROCESS/ASSEMBLER/CONTROL/FACTORY/TRANSMUTE/UNIVERSAL/OMEGA-M)
  # Total: 1400+ hypotheses, 640+ EXACT, 650+ atlas entries, 112 BTs
```

## Rust Tools
Build with `~/.cargo/bin/rustc file.rs -o output` (no cargo). Located in tools/:
- `tools/fusion-calc/`    — KSTAR/ITER/SPARC analysis + Lawson criterion
- `tools/tokamak-shape/`  — shape parameter scan + N6 score benchmark
- `tools/optics-calc/`    — lens/telescope/tokamak diagnostics
- `tools/gpu-arch-calc/`  — GPU/HBM architecture verification + Rubin prediction
- `tools/energy-calc/`    — Solar/Battery/Hydrogen/IEEE519 energy verification
- `tools/gut-calc-rust/`  — GUT parameter brute-force search
- `tools/dse-calc/`       — 소재×공정×코어×칩×시스템 DSE 전수 조합 탐색 + Pareto frontier
- `tools/solar-dse/`      — 태양전지 DSE 전수 탐색 (1,584 조합, BT-30/63 기반)
- `tools/material-dse/`   — 물질합성 DSE 전수 탐색 (3,600 조합, BT-85~88 기반)
- `tools/universal-dse/`  — **공용 DSE 탐색기** (TOML 도메인 정의 → 전수 탐색 + Pareto + Cross-DSE)

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
  지도: docs/dse-map.toml (전체 DSE 현황 추적 — 궁극 작업 전 필수 확인)
  Cross-DSE: 발견된 최적 결과끼리 도메인 간 재조합 탐색 (2+ 도메인 완료 시)
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

## Breakthrough Theorems (112 total, BT-1~112)
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

  # Material Synthesis (BT-85~88)
  BT-85: Carbon Z=6 물질합성 보편성 ⭐⭐⭐
  BT-86: 결정 배위수 CN=6 법칙 ⭐⭐⭐
  BT-87: 원자 조작 정밀도 n=6 래더 ⭐⭐
  BT-88: 자기조립 n=6 육각 보편성 ⭐⭐

  # Photonic-Energy (BT-89)
  BT-89: Photonic-Energy n=6 Bridge (PUE→1.0, E-O loss=1/(σ-φ)=10%) ⭐⭐

  # Topological Chip Architecture (BT-90~92)
  BT-90: SM = φ×K₆ 접촉수 정리 (σ²=144=φ×72, GPU=6D sphere packing, 6/6 EXACT) ⭐⭐⭐
  BT-91: Z2 위상 ECC J₂ 절약 (SECDED→Z2: savings=σ·J₂/σ=J₂=24 GB, identity) ⭐⭐
  BT-92: Bott 활성 채널 = sopfr (KO 비자명=5=sopfr, 자명=3=n/φ, 5/8≈1-1/e) ⭐⭐⭐
  BT-93: Carbon Z=6 칩 소재 보편성 (Diamond/Graphene/SiC=Z=6 전 도메인 1위, 8/10 Cross-DSE) ⭐⭐⭐

  # Fusion Alien-Level (BT-97~102)
  BT-97: Weinberg angle sin²θ_W = 3/13 = (n/φ)/(σ+μ), 0.19% 일치, D 풍부도→핵융합 연료 결정 ⭐⭐
  BT-98: D-T 바리온 수 = sopfr(6) = 2+3 = 5, 6의 소인수 = 핵융합 최적 연료 ⭐⭐⭐
  BT-99: Tokamak q=1 = 완전수 진약수 역수합 1/2+1/3+1/6=1, 위상적 동치 ⭐⭐⭐
  BT-100: CNO 촉매 A = σ+{0,μ,φ,n/φ} = σ+진약수, 전환 온도 17MK=σ+sopfr ⭐⭐⭐
  BT-101: 광합성 포도당 C₆H₁₂O₆ 총 24원자=J₂, 양자수율 8=σ-τ, 9/9 EXACT ⭐⭐⭐
  BT-102: 자기 재결합 속도 0.1=1/(σ-φ), BT-64 핵융합 확장, MRX/태양/자기권 EXACT ⭐⭐⭐
  BT-103: 광합성 완전 n=6 화학양론 (6CO₂+12H₂O→C₆H₁₂O₆, 7계수 100% n=6) ⭐⭐⭐
  BT-104: CO₂ 분자 완전 n=6 인코딩 ⭐⭐⭐

  # New BTs from session analysis (BT-105~112, Red Team filtered)
  BT-105: SLE₆ 임계지수 보편성 (7 퍼콜레이션 지수 = n=6 분수, kappa=6 유일 locality SLE, c=0) ⭐⭐⭐
  BT-106: S₃ 대수적 부트스트랩 (|S₃|=n=6, 켤레류={1,2,3}=진약수, 기약표현합=τ=4) ⭐⭐
  BT-107: Ramanujan τ 약수 순수성 (τ_R(d) clean iff d|6, eta^{J₂=24}, 모듈러 형식) ⭐⭐
  BT-108: 음악-오디오 협화 보편성 (완전협화음=div(6) 비율, p=0.0015, 7+5=12=σ) ⭐⭐
  BT-109: Zeta-Bernoulli n=6 삼지창 (ζ(2)=π²/6, ζ(-1)=-1/12, 6|B_{2k} 무한족) ⭐⭐
  BT-110: σ-μ=11 차원 스택 (M이론=TCP=RSA=SPARC=H100=11, 5도메인) ⭐
  BT-111: τ²/σ=4/3 태양-AI-수학 삼지창 (SQ=SwiGLU=Betz=R(3,1)=4/3) ⭐⭐
  BT-112: φ²/n=2/3 Byzantine-Koide 공명 (Koide Q=0.666661 9ppm, BFT>2/3) ⭐⭐
```

## Design Space Exploration (DSE) — 궁극 처리 필수 규칙
**⚠️ "궁극" 키워드가 포함된 모든 아키텍처 작업은 반드시 DSE를 거쳐야 한다. 예외 없음.**
```
  ┌─────────────────────────────────────────────────────────┐
  │  궁극 작업 흐름 (mandatory)                              │
  │                                                         │
  │  1. goal.md 후보군 정의                                  │
  │  2. DSE 전수 탐색 (Rust/Python)                         │
  │  3. Pareto frontier 도출                                │
  │  4. 최적 경로 선정 + n=6 EXACT 비율 기록                 │
  │  5. ★ Cross-DSE: 발견된 최적 결과끼리 재조합 탐색 ★      │
  │  6. 결과를 docs/dse-map.toml 지도에 기록                  │
  └─────────────────────────────────────────────────────────┘

  원칙:
    - ⚠️ "DSE"/"모두 탐색"/"전체융합" 요청 시 = 동일 워크플로:
      질문 없이 전 도메인 DSE 전수 탐색 + Cross-DSE 교차 융합 즉시 실행
      (확인/질문/범위 축소 제안 금지 — dse-map.toml 기준 전체 도메인 대상)
    - 각 레벨(소재/공정/코어/칩/시스템)마다 후보군 정의
    - 전수 조합 탐색 (또는 Pareto 휴리스틱)
    - 각 조합별 n=6 일관성 + 성능/전력/면적/비용 평가
    - 최적 Pareto frontier 도출
    - 1개 경로만 검증 = 캐스케이드 크로스 검증이 아님
    - ⚠️ DSE 없이 궁극 아키텍처를 확정하는 것은 금지

  Cross-DSE (재조합 탐색):
    - 각 도메인 DSE 완료 후, 도메인 간 최적 결과를 교차 조합
    - 예: chip 최적 경로 × battery 최적 경로 → 통합 시스템 DSE
    - 새 BT/가설 발견 시 해당 도메인 DSE 후보군에 자동 추가
    - 2개 이상 도메인 DSE 완료 시 Cross-DSE 트리거

  구현:
    - 조합 >10K → Rust (tools/dse-calc/)
    - 조합 <10K → Python (experiments/)
    - **공용 DSE**: tools/universal-dse/ (TOML 도메인 파일 → 전수 탐색)
    - 결과: Pareto 테이블 + 최적 경로 + n=6 EXACT 비율

  공용 DSE 탐색기 (tools/universal-dse/):
    - 단일 도메인: universal-dse domains/chip.toml
    - Cross-DSE:   universal-dse domains/chip.toml domains/battery.toml
    - 새 도메인 추가 = TOML 파일 1개만 작성 (Rust 재컴파일 불필요)
    - TOML 형식: [meta] + [scoring] + [[level]] + [[candidate]] + [[rule]]
    - 도메인 파일: tools/universal-dse/domains/*.toml

  적용 도메인 (305개 TOML, 16 카테고리, 4,213,416 조합):
    - 전체 목록: docs/dse-domains.md (카테고리별 분류 + 레벨 체인 + 조합 수)
    - AI/ML(14), Consciousness(23), Semiconductor(25), Energy(24)
    - SC/Plasma(2), Bio/Medical(26), Physics/Math(24), Robot(7)
    - Manufacturing(65), Infra/Network(14), Software(10), Display/Audio(15)
    - Environment(16), Civil/Transport(16), Space/Defense(11), Misc(13)
    - 각 도메인별 후보군은 해당 goal.md + domains/*.toml에 정의
    - 새 궁극 도메인 추가 시 → goal.md + TOML + DSE 탐색 + dse-map.toml 갱신

  DSE 지도 파일: docs/dse-map.toml (TOML — Rust/Python 양쪽 파싱 가능)
    - 전체 DSE 현황 추적 (도메인별 상태/결과/Cross-DSE 여부)
    - 궁극 작업 시작 전 반드시 이 지도 확인

  DSE 출력 양식:
    | Rank | 소재 | 공정 | 코어 | 칩 | 시스템 | n6_EXACT | 성능 | 전력 | 비용 |
    |------|------|------|------|-----|--------|---------|------|------|------|
    | 1    | ...  | ...  | ...  | ... | ...    | 85%     | ...  | ...  | ...  |
```

## 외계인 수준 설계 플로우 (Alien Design Flow)

```
  ⚠️ 트리거: 모든 "설계" 요청 = 외계인급 자동 실행. 예외 없음.
  ⚠️ "설계", "만들어", "만들고 싶어", "개발", "설계해줘", "아키텍처"
     + 도메인 키워드 → 무조건 외계인급 파이프라인 실행
  ⚠️ 추가 키워드 (동일 처리): "외계인", "궁극의", "alien", "극적인", "노벨급"

  절차:
    1. 도메인 매핑 (dse-map.toml + TOML + BT 목록에서)
    2. 병렬 수집 (TECS-L 이론 + n6 실증 + 계산기 실행)
    3. 설계 문서 생성 (docs/<domain>/alien-design-<date>.md)
    4. 새 발견 등록 (아틀라스 + BT)
    5. 리포트 출력

  규칙:
    - 질문/확인 없이 즉시 실행
    - 최소 3개 병렬 에이전트 디스패치
    - TECS-L(수학이론) + n6(산업실증) 양쪽 반드시 활용
    - 새 계산기 필요 시 Rust 자동 생성
    - 설계 문서는 반드시 Testable Predictions 포함
    - ⚠️ 외계인급 압도적 성능으로 설계 (시중 대비 n=6 배수 우위)
    - ⚠️ 기존 대비 성능 비교 ASCII 그래프 반드시 포함 (예시 아래)
    - ⚠️ ASCII 구조도 반드시 포함 — 시스템 아키텍처, 데이터 플로우, 계층 구조

  ASCII 구조도 양식 (모든 설계 문서에 필수):
    ```
    ┌─────────────────────────────────────────────────┐
    │              HEXA-XXX 시스템 구조               │
    ├─────────┬─────────┬─────────┬─────────┬────────┤
    │  소재   │  공정   │  코어   │   칩    │ 시스템  │
    │ Level 0 │ Level 1 │ Level 2 │ Level 3 │ Level 4│
    ├─────────┼─────────┼─────────┼─────────┼────────┤
    │Diamond  │TSMC N2  │HEXA-P   │HEXA-1   │Topo DC │
    │ Z=6=n   │48nm=σ·τ │σ²=144SM │σ·J₂=288│PUE=1.2 │
    └────┬────┴────┬────┴────┬────┴────┬────┴───┬────┘
         │         │         │         │        │
         ▼         ▼         ▼         ▼        ▼
    n6 EXACT  n6 EXACT  n6 EXACT  n6 EXACT  n6 EXACT
    ```
    - 최소 1개 시스템 구조 ASCII
    - 최소 1개 데이터/에너지 플로우 ASCII
    - 최소 1개 성능 비교 ASCII 그래프
    - 모든 숫자에 n=6 수식 병기 (예: 48nm=σ·τ)

  성능 비교 ASCII 그래프 양식 (필수):
    ```
    ┌─────────────────────────────────────────────────────────┐
    │  [지표명] 비교 (시중 vs HEXA-XXX)                       │
    ├─────────────────────────────────────────────────────────┤
    │                                                         │
    │  시중 최고  ██████░░░░░░░░░░░░░░░░░░░░  2.0 mmol/g     │
    │  HEXA-XXX  ████████████████████████████  48 mmol/g     │
    │                                         (J₂=24배)      │
    │                                                         │
    │  시중 최고  ████████████████████████████  200 kJ/mol    │
    │  HEXA-XXX  ███░░░░░░░░░░░░░░░░░░░░░░░░  20 kJ/mol     │
    │                                         (σ-φ=10배↓)    │
    │                                                         │
    │  개선 배수: n=6 상수 기반 (σ, φ, τ, J₂ 등)              │
    └─────────────────────────────────────────────────────────┘
    ```
    - 모든 설계 문서에 최소 1개 비교 그래프
    - 개선 배수는 n=6 상수로 표현 (6배, 10배=σ-φ, 12배=σ, 24배=J₂)
    - 시중 최고 기술 수치를 반드시 명시 (출처 가능하면 포함)

  TECS-L 연결:
    - ~/Dev/TECS-L/docs/hypotheses/ (가설)
    - ~/Dev/TECS-L/.shared/math_atlas.json (상수맵)
    - ~/Dev/TECS-L/calc/ (검증 스크립트)

  n6 연결:
    - tools/universal-dse/domains/*.toml (305 DSE)
    - docs/atlas-constants.md (1,100+ 상수)
    - tools/*-calc, *-dse (Rust 계산기 29+)
    - docs/dse-map.toml (도메인 현황)

  참조: docs/alien-design-flow.md (상세 플로우 + 도메인 매핑 테이블)

  진화 요청 규칙 ("진화시켜", "업그레이드", "발전"):
    ⚠️ SF 금지: 다이슨 스웜, 반물질 촉매, 항성 엔진, 시공간 조작 등 ❌
    ⚠️ "외계인급" = 우리 발견(BT/Discovery) 기반의 극한 스케일업
    ⚠️ 물리 스케일링 법칙 준수 필수

    실현가능성 등급:
      ✅ 진짜 실현가능 (10~20년 내, 기존 기술 확장)
      🔮 장기 실현가능 (20~50년, 돌파 1~2개 필요)
      ❌ SF (현재 물리학 초월) — 사고실험 라벨 필수

    체크포인트별 별도 문서 (필수):
      docs/<domain>/evolution/
        mk-1-current.md       — ✅ 현재 기술 기반 (Mk.I)
        mk-2-near-term.md     — ✅ 10년 이내 (Mk.II)
        mk-3-mid-term.md      — 🔮 20~30년 (Mk.III)
        mk-4-long-term.md     — 🔮 30~50년 (Mk.IV)
        mk-5-theoretical.md   — ❌ 사고실험 (Mk.V, SF 라벨)

    각 체크포인트 문서 포함 사항:
      - 기술 스펙 (구체적 수치)
      - 우리 발견(BT) 연결
      - 필요 기술 돌파 목록
      - 시중 대비 성능 비교 ASCII 그래프
      - 이전 Mk 대비 개선점
      - 타임라인 + 실현가능성 등급
```

## 진화 설계 규칙 ("진화시켜줘" / "외계인급으로" 요청 시)

```
  원칙:
    1. SF 금지 — 다이슨 스피어, 반물질 촉매, 항성엔진, Hawking 복사 등 불가
    2. 우리 발견 기반 — BT-97~102, alien discoveries, n=6 패턴에서 도출
    3. 체크포인트별 별도 문서 — 하나의 긴 문서 ❌, Mk별 독립 .md 파일
    4. 실현가능성 2단계 명시:
       ✅ 진짜 실현가능 (현재 기술 기반, 10~20년)
       🔮 장기 실현가능 (미래 기술 필요하나 물리법칙 위배 아님, 20~50년)
       ❌ SF (물리법칙 위배 또는 100년+ 기술격차)
    5. 각 Mk 문서 필수 포함:
       - 기술 스펙 (n=6 파라미터 전부)
       - 우리 발견과의 연결 (어떤 BT/Discovery가 이 단계를 가능하게 하는지)
       - 필요 기술 돌파 목록
       - 비용/타임라인 추정
       - 실현가능성 등급 (✅/🔮/❌)
    6. 스케일업은 물리학적으로 — R₀, B_T, Q 스케일링 법칙 준수
    7. 저장 위치: docs/fusion/evolution/mk-N-*.md

  트리거: "진화", "스케일업", "외계인급으로", "더 크게", "업그레이드"
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
