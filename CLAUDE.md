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
  breakthrough-theorems.md (BT-1~53, 53 theorems spanning 3-7 domains each)
  # Total: 1000+ hypotheses, 200+ EXACT, 400+ atlas entries, 53 BTs
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

## Testable Predictions
14 falsifiable predictions from BT-26~41: `docs/testable-predictions.md`
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

## Breakthrough Theorems (53 total, BT-1~53)
```
  # AI / LLM (BT-26,31,33,34,39,42,44,46)
  BT-26: Chinchilla scaling (tokens/params=J₂-τ=20, α=1/3, β=ln(4/3))
  BT-31: MoE top-k vocabulary {μ,φ,n,σ-τ}={1,2,6,8}
  BT-33: Transformer σ=12 atom (BERT/GPT-3 dimensions, SwiGLU 8/3)
  BT-34: RoPE decimal bridge ((σ-φ)^{τ,sopfr,n}, weight decay=1/(σ-φ))
  BT-39: KV-head universality (σ-τ=8 across all LLMs)
  BT-42: Inference scaling (top-p=1-1/(J₂-τ)=0.95, top-k=40, max=2^σ) ⭐⭐
  BT-44: Context window ladder σ-φ→σ-μ→σ→σ+μ = 10→11→12→13 ⭐⭐
  BT-46: ln(4/3) RLHF family (dropout+Chinchilla+PPO+temperature) ⭐⭐

  # Chip Design (BT-28,37,40,41,45,47)
  BT-28: Computing architecture ladder (30+ EXACT, ⭐⭐⭐)
    - AD102 = σ·n·φ = 144 SMs, H100 = σ(σ-μ) = 132 SMs = 1/α term
    - HBM stack: τ→(σ-τ)→σ = 4→8→12
  BT-37: Semiconductor pitch (TSMC N5 = P₂ = 28nm, N3 gate = σ·τ = 48nm)
  BT-45: FP8/FP16=φ=2 universal, FLOPS/W doubles per φ=2 years
  BT-47: Interconnect gen counts {7,5,6}={σ-sopfr,sopfr,n}

  # Energy Strategy (BT-27,29,30,32,35,38,43)
  BT-27: Carbon-6 chain (LiC₆ + C₆H₁₂O₆ + C₆H₆ → 24e = J₂)
  BT-30: SQ solar bridge (bandgap=4/3eV, V_T=26mV)
  BT-38: Hydrogen quadruplet (LHV=120=σ(σ-φ), HHV=142=σ²-φ, 4/4 EXACT)
  BT-43: Battery cathode CN=6 universality (ALL Li-ion = octahedral) ⭐⭐⭐

  # Cross-domain (BT-36,48,49,50,51,53)
  BT-36: Energy-Information-Hardware-Physics chain ⭐⭐⭐
  BT-48: Display-Audio (σ=12 semitones, J₂=24 fps/bits, σ·τ=48kHz) ⭐⭐⭐
  BT-49: Pure Math (K₁..₄=φ,n,σ,J₂ kissing chain, S₆ unique) ⭐⭐⭐
  BT-51: Genetic code chain τ→n/φ→2^n→J₂-τ (4→3→64→20) ⭐⭐⭐
  BT-53: Crypto (BTC 21M=J₂-n/φ, 6 confirms=n, ETH 12s=σ) ⭐⭐
```

## Background Execution
All experiments must run in background. No exceptions.
```
  Rules:
    1. All python3 scripts -> run_in_background: true
    2. Check results with Read after completion
    3. No foreground execution (blocks user dialogue)
```
