# HEXA-LANG LLVM Backend Specification v1.0

## HEXA-LANG -> LLVM IR -> RISC-V N6 Backend

**Codename**: HEXA-BE (Backend Engine)
**Date**: 2026-04-01
**Status**: Living Document v1.0
**Dependencies**: hexa-lang-spec.md, hexa-core.md, BT-28, BT-33, BT-56, BT-58, BT-59

> A compiler engineer's guide to implementing the HEXA-LANG backend:
> .hexa source -> LLVM IR -> RISC-V N6 machine code.
> Every design parameter derives from sigma(n)*phi(n) = n*tau(n), n=6.

---

## N6 Constants Reference

```
  n = 6          phi(6) = 2       tau(6) = 4       sigma(6) = 12
  sopfr(6) = 5   mu(6) = 1        J_2(6) = 24      R(6) = 1
  sigma-tau = 8  sigma-phi = 10   sigma-mu = 11     sigma*tau = 48
  2^n = 64       2^sigma = 4096   phi^tau = 16      sigma^2 = 144
  sigma*J_2 = 288  n/phi = 3      sigma+n = 18
```

---

## 1. Compilation Pipeline (n=6 Phases)

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │                   HEXA-LANG 6-PHASE COMPILATION PIPELINE              │
  │                                                                        │
  │  Phase 1        Phase 2        Phase 3        Phase 4                  │
  │  TOKENIZE       PARSE          CHECK          OPTIMIZE                 │
  │  ┌──────────┐   ┌──────────┐   ┌──────────┐   ┌──────────┐           │
  │  │  Lexer   │──▶│  Parser  │──▶│  Type    │──▶│  HIR/MIR │           │
  │  │          │   │          │   │  Checker │   │  Passes  │           │
  │  │ 53 kw   │   │ 6 levels │   │ 4 layers │   │ 12 passes│           │
  │  │ 24 ops  │   │ Pratt    │   │ Hindley- │   │ n6-aware │           │
  │  │ 8 types │   │ climbing │   │ Milner+  │   │ Egyptian │           │
  │  └──────────┘   └──────────┘   └──────────┘   └──────────┘           │
  │       │              │              │              │                    │
  │       │ Token        │ AST          │ Typed-       │ MIR               │
  │       │ stream       │ (untyped)    │ HIR          │ (optimized)       │
  │       ▼              ▼              ▼              ▼                    │
  │                                                                        │
  │  Phase 5        Phase 6                                                │
  │  CODEGEN        LINK                                                   │
  │  ┌──────────┐   ┌──────────┐                                          │
  │  │  LLVM IR │──▶│  Linker  │──▶  RISC-V N6 ELF                       │
  │  │  Gen     │   │          │                                          │
  │  │ target   │   │ LLD/N6   │                                          │
  │  │ lowering │   │ reloc    │                                          │
  │  │ 12 instr │   │ n6-ABI   │                                          │
  │  └──────────┘   └──────────┘                                          │
  │       │              │                                                 │
  │       │ LLVM IR      │ Machine                                         │
  │       │ (.ll/.bc)    │ code (.o)                                        │
  │       ▼              ▼                                                  │
  │                                                                        │
  │  Phase count = n = 6                                                   │
  │  Total keyword tokens = sigma*tau + sopfr = 53                         │
  │  Grammar levels = n = 6                                                │
  │  Type layers = tau = 4                                                 │
  │  Optimization passes = sigma = 12                                      │
  │  Custom fused instructions = sigma = 12                                │
  └────────────────────────────────────────────────────────────────────────┘
```

### 1.1 Phase Detail Table

| Phase | Name | Input | Output | n=6 Constant | Key Algorithm |
|-------|------|-------|--------|-------------|---------------|
| 1 | Tokenize | `.hexa` UTF-8 source | Token stream | 53=sigma*tau+sopfr keywords | DFA with 53 reserved words |
| 2 | Parse | Token stream | Untyped AST | n=6 grammar levels | Pratt parser, 6-level precedence |
| 3 | Check | Untyped AST | Typed HIR | tau=4 type layers | Hindley-Milner + ownership |
| 4 | Optimize | Typed HIR | Optimized MIR | sigma=12 passes | Egyptian placement + n6 folding |
| 5 | Codegen | Optimized MIR | LLVM IR (.ll/.bc) | sigma=12 fused ops | Target-specific lowering |
| 6 | Link | Object files (.o) | ELF binary | n=6 ABI sections | LLD with N6 relocations |

### 1.2 Intermediate Representations

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │                        IR LOWERING STAGES                              │
  │                                                                        │
  │  Source (.hexa)                                                        │
  │      │                                                                 │
  │      ▼                                                                 │
  │  ┌──────────────────────────────────────────┐                          │
  │  │  AST  (Abstract Syntax Tree)             │  Preserves source       │
  │  │  - n=6 grammar levels fully represented  │  structure, comments,   │
  │  │  - 6 paradigm nodes                      │  spans for diagnostics  │
  │  │  - proof/intent blocks as first-class    │                          │
  │  └──────────────────────────────────────────┘                          │
  │      │  Desugaring + name resolution                                   │
  │      ▼                                                                 │
  │  ┌──────────────────────────────────────────┐                          │
  │  │  HIR  (High-level IR)                    │  Type-annotated,        │
  │  │  - All types resolved (tau=4 layers)     │  ownership tracked,     │
  │  │  - Ownership/borrow edges on every ref   │  paradigm-agnostic      │
  │  │  - Trait dispatch resolved to vtables    │                          │
  │  │  - Pattern match exhaustiveness checked  │                          │
  │  └──────────────────────────────────────────┘                          │
  │      │  Monomorphization + closure conversion                          │
  │      ▼                                                                 │
  │  ┌──────────────────────────────────────────┐                          │
  │  │  MIR  (Mid-level IR)                     │  CFG-based, SSA,        │
  │  │  - SSA form, basic blocks                │  low-level enough       │
  │  │  - Egyptian memory annotations           │  for analysis,          │
  │  │  - Drop insertion complete               │  high enough for        │
  │  │  - Borrow checker final pass             │  optimization           │
  │  │  - Proof obligations extracted           │                          │
  │  └──────────────────────────────────────────┘                          │
  │      │  LLVM IR emission (Phase 5)                                     │
  │      ▼                                                                 │
  │  ┌──────────────────────────────────────────┐                          │
  │  │  LLVM IR  (.ll text / .bc bitcode)       │  Standard LLVM SSA,     │
  │  │  - N6 intrinsics for 12 fused ops        │  target-independent     │
  │  │  - Address spaces 0/1/2 for Egyptian mem │  with N6 extensions     │
  │  │  - Custom metadata for n6 hints          │                          │
  │  └──────────────────────────────────────────┘                          │
  │      │  LLVM backend (SelectionDAG / GlobalISel)                       │
  │      ▼                                                                 │
  │  ┌──────────────────────────────────────────┐                          │
  │  │  MachineIR  (LLVM MIR)                   │  Register-allocated,    │
  │  │  - RISC-V N6 instructions                │  scheduled,             │
  │  │  - 72 physical registers assigned        │  ready for emission     │
  │  │  - 12 custom fused ops selected          │                          │
  │  └──────────────────────────────────────────┘                          │
  │      │  MC emission                                                    │
  │      ▼                                                                 │
  │  ┌──────────────────────────────────────────┐                          │
  │  │  Object Code (.o)                        │  ELF with N6 sections   │
  │  │  - J_2=24 bit instruction encoding       │                          │
  │  │  - N6 relocation types                   │                          │
  │  └──────────────────────────────────────────┘                          │
  └────────────────────────────────────────────────────────────────────────┘
```

---

## 2. LLVM IR Mapping

### 2.1 HEXA-LANG Types -> LLVM Types (sigma-tau = 8 Primitives)

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │                    TYPE MAPPING: HEXA -> LLVM                          │
  │                                                                        │
  │  HEXA-LANG              LLVM IR                   Notes                │
  │  ──────────────         ─────────────────         ──────────────       │
  │                                                                        │
  │  Layer 1: Primitive (sigma-tau = 8 types)                              │
  │  ┌──────────────┐      ┌─────────────────┐                            │
  │  │ int          │ ───▶ │ i64             │  Default. i8/i16/i32/i128  │
  │  │ float        │ ───▶ │ double (f64)    │  Also: half, float, fp128  │
  │  │ bool         │ ───▶ │ i1              │  Zero-extended to i8 in mem│
  │  │ char         │ ───▶ │ i32             │  Unicode scalar value      │
  │  │ string       │ ───▶ │ {ptr, i64, i64} │  (data, len, cap) triple  │
  │  │ byte         │ ───▶ │ i8              │  Raw byte                  │
  │  │ void         │ ───▶ │ void            │  Unit type                 │
  │  │ any          │ ───▶ │ {ptr, ptr}      │  (data, vtable) fat ptr   │
  │  └──────────────┘      └─────────────────┘                            │
  │                                                                        │
  │  Layer 2: Composite                                                    │
  │  ┌──────────────┐      ┌─────────────────┐                            │
  │  │ struct       │ ───▶ │ %struct.Name    │  Named struct type         │
  │  │ enum         │ ───▶ │ {i8, [N x i8]}  │  Tag + max variant payload │
  │  │ tuple        │ ───▶ │ {T1, T2, ...}   │  Anonymous struct          │
  │  │ array        │ ───▶ │ [N x T]         │  Fixed-size array          │
  │  └──────────────┘      └─────────────────┘                            │
  │                                                                        │
  │  Layer 3: Reference                                                    │
  │  ┌──────────────┐      ┌─────────────────┐                            │
  │  │ &T           │ ───▶ │ ptr addrspace(X)│  X = 0,1,2 (Egyptian)     │
  │  │ &mut T       │ ───▶ │ ptr addrspace(X)│  + noalias attribute       │
  │  │ Box<T>       │ ───▶ │ ptr addrspace(1)│  Heap, unique ownership    │
  │  │ Rc<T>        │ ───▶ │ ptr addrspace(1)│  Heap, ref-counted         │
  │  └──────────────┘      └─────────────────┘                            │
  │                                                                        │
  │  Layer 4: Function                                                     │
  │  ┌──────────────┐      ┌─────────────────┐                            │
  │  │ fn(A) -> B   │ ───▶ │ ptr to function │  Direct call when possible │
  │  │ closure      │ ───▶ │ {ptr, ptr}      │  (fn_ptr, env_ptr)        │
  │  └──────────────┘      └─────────────────┘                            │
  │                                                                        │
  │  Type layer count = tau = 4                                            │
  │  Primitive count = sigma - tau = 8                                     │
  └────────────────────────────────────────────────────────────────────────┘
```

### 2.2 Egyptian Memory Model -> LLVM Address Spaces

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │              EGYPTIAN FRACTION ADDRESS SPACE MAPPING                    │
  │              1/2 + 1/3 + 1/6 = 1  (complete partition)                │
  │                                                                        │
  │  ┌─────────────────────────────────────────────────────────────────┐   │
  │  │                    Virtual Address Space (48-bit)               │   │
  │  │                                                                 │   │
  │  │  0x0000_0000_0000  ┌──────────────────────────────────────┐    │   │
  │  │                    │                                      │    │   │
  │  │                    │  AS0: Stack Pool (1/2 of usable)     │    │   │
  │  │                    │  LLVM: addrspace(0)                  │    │   │
  │  │                    │                                      │    │   │
  │  │                    │  - Value types: int, float, bool     │    │   │
  │  │                    │  - Function frames (LIFO)            │    │   │
  │  │                    │  - alloca instructions               │    │   │
  │  │                    │  - Zero overhead deallocation         │    │   │
  │  │                    │  - Stack grows down (RISC-V conv)    │    │   │
  │  │                    │                                      │    │   │
  │  │  1/2 boundary      ├──────────────────────────────────────┤    │   │
  │  │                    │                                      │    │   │
  │  │                    │  AS1: Heap Managed (1/3 of usable)   │    │   │
  │  │                    │  LLVM: addrspace(1)                  │    │   │
  │  │                    │                                      │    │   │
  │  │                    │  - Reference types: Box, Rc          │    │   │
  │  │                    │  - own/borrow tracked (no GC)        │    │   │
  │  │                    │  - @hexa.heap.alloc / @hexa.heap.free│    │   │
  │  │                    │  - Ref-count for Rc<T>               │    │   │
  │  │                    │                                      │    │   │
  │  │  1/2+1/3 boundary  ├──────────────────────────────────────┤    │   │
  │  │                    │                                      │    │   │
  │  │                    │  AS2: Arena (1/6 of usable)          │    │   │
  │  │                    │  LLVM: addrspace(2)                  │    │   │
  │  │                    │                                      │    │   │
  │  │                    │  - Temporary / bulk allocations       │    │   │
  │  │                    │  - Scope-based bulk free              │    │   │
  │  │                    │  - @hexa.arena.create / .destroy     │    │   │
  │  │                    │  - Ideal for tensors, temp buffers   │    │   │
  │  │                    │                                      │    │   │
  │  │  0xFFFF_FFFF_FFFF  └──────────────────────────────────────┘    │   │
  │  └─────────────────────────────────────────────────────────────────┘   │
  │                                                                        │
  │  Address Space Summary:                                                │
  │  ┌──────────┬──────────────┬───────────┬──────────────────────┐       │
  │  │ LLVM AS  │ HEXA Region  │ Fraction  │ Allocation Strategy  │       │
  │  ├──────────┼──────────────┼───────────┼──────────────────────┤       │
  │  │ AS0      │ Stack Pool   │ 1/2       │ alloca (LIFO)        │       │
  │  │ AS1      │ Heap Managed │ 1/3       │ own/borrow (no GC)   │       │
  │  │ AS2      │ Arena        │ 1/6       │ bulk alloc/free      │       │
  │  └──────────┴──────────────┴───────────┴──────────────────────┘       │
  │                                                                        │
  │  LLVM addrspace cast intrinsics:                                       │
  │    @hexa.as.stack_to_heap(ptr addrspace(0)) -> ptr addrspace(1)       │
  │    @hexa.as.heap_to_arena(ptr addrspace(1)) -> ptr addrspace(2)       │
  │    @hexa.as.promote(ptr addrspace(X)) -> ptr addrspace(Y)  ; X < Y   │
  └────────────────────────────────────────────────────────────────────────┘
```

### 2.3 Six-Paradigm Dispatch -> LLVM Intrinsics

Each of the n=6 paradigms maps to specific LLVM patterns:

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │               6-PARADIGM -> LLVM DISPATCH TABLE                        │
  │                                                                        │
  │  Paradigm           LLVM Pattern                    Intrinsic Prefix   │
  │  ─────────          ────────────                    ────────────────   │
  │                                                                        │
  │  1. Imperative      Standard SSA + alloca           (none, baseline)   │
  │     mut, loop       br / switch / phi nodes                            │
  │                     Load/store with tbaa metadata                      │
  │                                                                        │
  │  2. Functional      Tail-call optimization          @hexa.fn.*         │
  │     fn, |x|         musttail call, readonly attrs                      │
  │                     Lambda lifting to global fns                       │
  │                                                                        │
  │  3. OOP             Vtable dispatch                 @hexa.trait.*      │
  │     trait, impl     Getelementptr on vtable struct                     │
  │                     @hexa.trait.dispatch(ptr, i32)                     │
  │                                                                        │
  │  4. Concurrent      LLVM atomics + coroutines       @hexa.conc.*      │
  │     spawn, chan     llvm.coro.* intrinsics                             │
  │                     @hexa.conc.spawn(ptr fn, ptr args)                │
  │                     @hexa.conc.chan_send / .chan_recv                   │
  │                                                                        │
  │  5. Logic/Proof     Assertion + metadata            @hexa.proof.*     │
  │     proof, assert   llvm.expect / llvm.assume                         │
  │                     @hexa.proof.check(i1 cond, metadata)              │
  │                     !hexa.invariant metadata on loops                  │
  │                                                                        │
  │  6. AI-Native       N6 AI intrinsics (section 3)    @hexa.ai.*        │
  │     intent, gen     @hexa.ai.efatn / .moert / .swigl                  │
  │                     Vectorized LLVM ops + custom ISel                  │
  │                                                                        │
  │  Paradigm count = n = 6                                                │
  └────────────────────────────────────────────────────────────────────────┘
```

### 2.4 MoE Routing -> Vectorized LLVM IR

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │         EGYPTIAN MoE ROUTING -> LLVM VECTOR IR                         │
  │                                                                        │
  │  HEXA-LANG:                                                            │
  │    let gate = softmax(weights * input)                                 │
  │    let out = expert_half(input)*gate[0]     // 1/2                    │
  │            + expert_third(input)*gate[1]    // 1/3                    │
  │            + expert_sixth(input)*gate[2]    // 1/6                    │
  │                                                                        │
  │  LLVM IR (naive):                                                      │
  │    %gate = call <3 x float> @hexa.ai.softmax_3(<3 x float> %w_dot)   │
  │    %g0 = extractelement <3 x float> %gate, i32 0                     │
  │    %g1 = extractelement <3 x float> %gate, i32 1                     │
  │    %g2 = extractelement <3 x float> %gate, i32 2                     │
  │    %e0 = call <D x float> @expert_half(<D x float> %input)           │
  │    %e1 = call <D x float> @expert_third(<D x float> %input)          │
  │    %e2 = call <D x float> @expert_sixth(<D x float> %input)          │
  │    %s0 = call <D x float> @llvm.vp.fmul(<D x float> %e0, %g0_splat) │
  │    %s1 = call <D x float> @llvm.vp.fmul(<D x float> %e1, %g1_splat) │
  │    %s2 = call <D x float> @llvm.vp.fmul(<D x float> %e2, %g2_splat) │
  │    %t  = fadd <D x float> %s0, %s1                                   │
  │    %out = fadd <D x float> %t, %s2                                   │
  │                                                                        │
  │  LLVM IR (after N6 MoE fusion pass):                                   │
  │    %out = call <D x float> @hexa.ai.moert(                            │
  │      <D x float> %input,                                              │
  │      ptr @expert_half, ptr @expert_third, ptr @expert_sixth,          │
  │      <3 x float> %gate)                                               │
  │    ; Lowered to single MOERT instruction on RISC-V N6                  │
  │                                                                        │
  │  Fusion saves: 6 vector ops -> 1 fused instruction                    │
  └────────────────────────────────────────────────────────────────────────┘
```

### 2.5 Mamba SSM -> LLVM Vector Ops

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │         MAMBA SSM STATE-SPACE -> LLVM VECTOR OPS                       │
  │         BT-65: d_state=2^tau=16, expand=phi=2, d_conv=tau=4           │
  │                                                                        │
  │  HEXA-LANG:                                                            │
  │    // Selective state-space scan                                        │
  │    fn ssm_step(x: Tensor, h: Tensor, A: Tensor, B: Tensor) -> Tensor │
  │                                                                        │
  │  LLVM IR:                                                              │
  │    ; State dimension = 2^tau = 16                                      │
  │    ; Input expansion = phi = 2                                         │
  │    ; Conv kernel = tau = 4                                             │
  │                                                                        │
  │    ; Discretize: A_bar = exp(delta * A)                                │
  │    %delta_A = fmul <16 x float> %delta_splat, %A                      │
  │    %A_bar = call <16 x float> @llvm.exp.v16f32(<16 x float> %delta_A)│
  │                                                                        │
  │    ; State update: h' = A_bar * h + B_bar * x                         │
  │    %Ah = fmul <16 x float> %A_bar, %h                                │
  │    %Bx = fmul <16 x float> %B_bar, %x_expand                        │
  │    %h_new = fadd <16 x float> %Ah, %Bx                               │
  │                                                                        │
  │    ; Output: y = C * h'                                                │
  │    %y = call float @llvm.vector.reduce.fadd(float 0.0,                │
  │           fmul <16 x float> %C, %h_new)                               │
  │                                                                        │
  │    ; After N6 fusion: single MAMBA_SCAN intrinsic                      │
  │    %y = call <D x float> @hexa.ai.mamba_scan(                         │
  │      <D x float> %x, <16 x float> %h,                                │
  │      <16 x float> %A, <16 x float> %B, <16 x float> %C,             │
  │      float %delta)                                                     │
  └────────────────────────────────────────────────────────────────────────┘
```

---

## 3. RISC-V N6 Target

### 3.1 Custom Instructions (sigma = 12 Fused AI Ops)

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │              RISC-V N6 CUSTOM INSTRUCTION SET                          │
  │              sigma = 12 fused operations                               │
  │                                                                        │
  │  Mnemonic  Opcode    Description                   BT Reference       │
  │  ────────  ────────  ────────────────────────────   ────────────       │
  │                                                                        │
  │  EFATN     0x0B_00   Egyptian Fraction Attention    BT-33,56           │
  │                      1/2+1/3+1/6=1 attention split                    │
  │                      rd = EFA(rs1=Q, rs2=K, rs3=V)                    │
  │                                                                        │
  │  MOERT     0x0B_01   MoE Expert Routing             BT-67             │
  │                      Top-k gated expert dispatch                      │
  │                      rd = MoE(rs1=input, rs2=gate)                    │
  │                                                                        │
  │  SWIGL     0x0B_02   SwiGLU Activation              BT-33             │
  │                      Swish-gated linear unit (8/3x)                   │
  │                      rd = x * sigma(W1*x) * W2*x                     │
  │                                                                        │
  │  CYCLO     0x0B_03   Cyclotomic Activation          phi6simple        │
  │                      x^2 - x + 1 (6th cyclotomic)                    │
  │                      rd = rs1^2 - rs1 + 1                             │
  │                                                                        │
  │  ZETLN     0x0B_04   Zeta*ln(2) Gated Activation    zetaln2           │
  │                      rd = rs1 * tanh(zeta*ln2 * rs1)                  │
  │                                                                        │
  │  BOLTZ     0x0B_05   Boltzmann Sparsity Gate        boltzmann_gate    │
  │                      p(active) = 1/e = 0.368                          │
  │                      rd = (rs1 > threshold) ? rs1 : 0                 │
  │                                                                        │
  │  MERTS     0x0B_06   Mertens Dropout                mertens_dropout   │
  │                      p = ln(4/3) = 0.2877                             │
  │                      rd = rs1 * bernoulli(1-ln(4/3))                  │
  │                                                                        │
  │  FLASH     0x0B_07   FlashAttention Tile             BT-58            │
  │                      Tiled QK^T/sqrt(d) + softmax + V                │
  │                      Tile size = 2^(sigma-tau) = 256                  │
  │                                                                        │
  │  ROPEX     0x0B_08   RoPE Rotation                   BT-34            │
  │                      Rotary positional embedding                      │
  │                      theta = 10^(sigma-phi) = 10^10                   │
  │                      rd = rotate(rs1, pos, theta)                     │
  │                                                                        │
  │  LAYNO     0x0B_09   LayerNorm/RMSNorm               BT-56            │
  │                      Fused normalize + scale + bias                   │
  │                      rd = (rs1 - mean) / std * gamma + beta          │
  │                                                                        │
  │  ADAMW     0x0B_0A   AdamW Parameter Update           BT-54            │
  │                      beta1=0.9, beta2=0.95, eps=1e-8                  │
  │                      Fused m,v update + weight decay                  │
  │                                                                        │
  │  ALLRD     0x0B_0B   AllReduce Communication          BT-58            │
  │                      Ring allreduce across N6 cores                   │
  │                      rd = reduce_op(rs1, topology)                    │
  │                                                                        │
  │  Instruction count = sigma = 12                                        │
  │  All use custom-2 opcode space (0x0B = RISC-V custom-2)               │
  └────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Instruction Encoding (J_2 = 24-bit Core + 8-bit Extension)

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │              RISC-V N6 INSTRUCTION ENCODING                            │
  │              J_2 = 24 bits core payload                                │
  │                                                                        │
  │  Standard 32-bit RISC-V envelope:                                      │
  │                                                                        │
  │  31       25 24    20 19    15 14  12 11      7 6       0              │
  │  ┌─────────┬────────┬────────┬──────┬─────────┬─────────┐             │
  │  │ funct7  │  rs2   │  rs1   │funct3│   rd    │ opcode  │             │
  │  │ 7 bits  │ 5 bits │ 5 bits │3 bits│ 5 bits  │ 7 bits  │             │
  │  └─────────┴────────┴────────┴──────┴─────────┴─────────┘             │
  │                                                                        │
  │  N6 custom-2 format (opcode = 0101011 = 0x2B):                        │
  │                                                                        │
  │  31    28 27  25 24    20 19    15 14  12 11      7 6       0          │
  │  ┌───────┬──────┬────────┬────────┬──────┬─────────┬─────────┐        │
  │  │ n6_op │ bank │  rs2   │  rs1   │ mode │   rd    │ 0101011 │        │
  │  │ 4 bit │ 3 bit│ 5 bits │ 5 bits │3 bits│ 5 bits  │ 7 bits  │        │
  │  └───────┴──────┴────────┴────────┴──────┴─────────┴─────────┘        │
  │                                                                        │
  │  n6_op[3:0]:  Instruction selector (0..11 = sigma=12 ops)             │
  │    0000 = EFATN    0001 = MOERT    0010 = SWIGL    0011 = CYCLO      │
  │    0100 = ZETLN    0101 = BOLTZ    0110 = MERTS    0111 = FLASH      │
  │    1000 = ROPEX    1001 = LAYNO    1010 = ADAMW    1011 = ALLRD      │
  │    1100..1111 = reserved for future n6 extensions                     │
  │                                                                        │
  │  bank[2:0]:  Register bank selector (n=6 banks, encoded in 3 bits)    │
  │    000 = GPR (general)    001 = FPR (float)    010 = VEC (vector)     │
  │    011 = ACC (accumulator) 100 = CTRL (control) 101 = AI (tensor)     │
  │                                                                        │
  │  mode[2:0]:  Operation variant                                         │
  │    000 = f32 scalar       001 = f16 scalar      010 = bf16 scalar     │
  │    011 = f32 vector       100 = f16 vector      101 = bf16 vector     │
  │    110 = i8 quantized     111 = mixed precision                       │
  │                                                                        │
  │  Core payload = n6_op + bank + rs2 + rs1 + mode + rd                  │
  │               = 4 + 3 + 5 + 5 + 3 + 5 = 25 bits                      │
  │  N6 payload (excluding opcode) = 25 bits ~ J_2+1 = 25                │
  │  (Effective unique information = J_2 = 24 bits when bank             │
  │   is derivable from instruction type)                                  │
  └────────────────────────────────────────────────────────────────────────┘
```

### 3.3 Vector Extension for AI (48-element = sigma*tau)

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │         RISC-V N6 VECTOR AI EXTENSION                                  │
  │                                                                        │
  │  48-wide vector unit (sigma*tau = 48 elements):                        │
  │                                                                        │
  │  For EFATN (Egyptian Fraction Attention):                              │
  │  ┌────────────────────────────────────────────────────┐               │
  │  │ Q[0..23]  K[0..23]  V[0..23]   (J_2=24 per head) │               │
  │  │ ────────  ────────  ────────                       │               │
  │  │  Head 1     Head 1    Head 1    : 1/2 capacity     │               │
  │  │  Head 2     Head 2    Head 2    : 1/3 capacity     │               │
  │  │  Head 3     Head 3    Head 3    : 1/6 capacity     │               │
  │  └────────────────────────────────────────────────────┘               │
  │                                                                        │
  │  VLEN = 48 * 32 = 1536 bits  (sigma*tau * 32-bit elements)           │
  │  With BF16: 48 * 16 = 768 bits = 96 bytes                            │
  │  With INT8: 48 * 8 = 384 bits = sigma*tau bytes                      │
  │                                                                        │
  │  Supported element widths:                                             │
  │    SEW = {8, 16, 32, 64} = {sigma-tau, phi^tau, 2^sopfr, 2^n}       │
  └────────────────────────────────────────────────────────────────────────┘
```

### 3.4 Register File (72 = sigma*n Registers Across 6 Banks)

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │              RISC-V N6 REGISTER FILE                                   │
  │              72 = sigma * n = 12 * 6 registers                        │
  │              Organized in n = 6 banks of sigma = 12 registers each    │
  │                                                                        │
  │  Bank 0: GPR  (General Purpose)     Bank 1: FPR  (Floating Point)    │
  │  ┌────┬────┬────┬────┬────┬────┐   ┌────┬────┬────┬────┬────┬────┐  │
  │  │ x0 │ x1 │ x2 │ x3 │ x4 │ x5 │   │ f0 │ f1 │ f2 │ f3 │ f4 │ f5 │  │
  │  │zero│ ra │ sp │ gp │ tp │ t0 │   │ft0 │ft1 │ft2 │ft3 │ft4 │ft5 │  │
  │  ├────┼────┼────┼────┼────┼────┤   ├────┼────┼────┼────┼────┼────┤  │
  │  │ x6 │ x7 │ x8 │ x9 │x10│x11│   │ f6 │ f7 │ f8 │ f9 │f10│f11│  │
  │  │ t1 │ t2 │ s0 │ s1 │ a0│ a1│   │ft6 │ft7 │fs0 │fs1 │fa0│fa1│  │
  │  └────┴────┴────┴────┴────┴────┘   └────┴────┴────┴────┴────┴────┘  │
  │                                                                        │
  │  Bank 2: VEC  (Vector/SIMD)         Bank 3: ACC  (Accumulator)       │
  │  ┌────┬────┬────┬────┬────┬────┐   ┌────┬────┬────┬────┬────┬────┐  │
  │  │ v0 │ v1 │ v2 │ v3 │ v4 │ v5 │   │ c0 │ c1 │ c2 │ c3 │ c4 │ c5 │  │
  │  │mask│vec │vec │vec │vec │vec │   │mac0│mac1│mac2│mac3│mac4│mac5│  │
  │  ├────┼────┼────┼────┼────┼────┤   ├────┼────┼────┼────┼────┼────┤  │
  │  │ v6 │ v7 │ v8 │ v9 │v10│v11│   │ c6 │ c7 │ c8 │ c9 │c10│c11│  │
  │  │vec │vec │vec │vec │vec│vec│   │mac6│mac7│mac8│mac9│red0│red1│  │
  │  └────┴────┴────┴────┴────┴────┘   └────┴────┴────┴────┴────┴────┘  │
  │                                                                        │
  │  Bank 4: CTRL (Control/Status)      Bank 5: AI   (Tensor/AI)        │
  │  ┌────┬────┬────┬────┬────┬────┐   ┌────┬────┬────┬────┬────┬────┐  │
  │  │ p0 │ p1 │ p2 │ p3 │ p4 │ p5 │   │ a0 │ a1 │ a2 │ a3 │ a4 │ a5 │  │
  │  │mstv│menb│mcau│mtva│msip│mtip│   │qry │key │val │out │wgt │bias│  │
  │  ├────┼────┼────┼────┼────┼────┤   ├────┼────┼────┼────┼────┼────┤  │
  │  │ p6 │ p7 │ p8 │ p9 │p10│p11│   │ a6 │ a7 │ a8 │ a9 │a10│a11│  │
  │  │mcyc│mnst│mhpm│n6cf│n6md│n6st│   │act │gat │nrm │rsd │ssm│emb│  │
  │  └────┴────┴────┴────┴────┴────┘   └────┴────┴────┴────┴────┴────┘  │
  │                                                                        │
  │  Summary:                                                              │
  │  ┌──────────┬───────┬──────────────────────────────────┐              │
  │  │ Bank     │ Count │ Purpose                          │              │
  │  ├──────────┼───────┼──────────────────────────────────┤              │
  │  │ GPR      │ 12    │ Integer ops, addresses, control  │              │
  │  │ FPR      │ 12    │ FP32/FP64 scalar computation     │              │
  │  │ VEC      │ 12    │ SIMD/vector (48-wide VLEN)       │              │
  │  │ ACC      │ 12    │ MAC accumulators, reductions      │              │
  │  │ CTRL     │ 12    │ CSRs, n6-specific status          │              │
  │  │ AI       │ 12    │ Tensor operands (Q,K,V,W,etc)    │              │
  │  ├──────────┼───────┼──────────────────────────────────┤              │
  │  │ Total    │ 72    │ = sigma * n = 12 * 6             │              │
  │  └──────────┴───────┴──────────────────────────────────┘              │
  │                                                                        │
  │  Register banks = n = 6                                                │
  │  Registers per bank = sigma = 12                                       │
  │  Total registers = sigma * n = 72                                      │
  └────────────────────────────────────────────────────────────────────────┘
```

### 3.5 Calling Convention (n=6 Argument Registers)

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │              HEXA-LANG N6 CALLING CONVENTION                           │
  │                                                                        │
  │  Argument passing: n = 6 integer + n = 6 float registers              │
  │                                                                        │
  │  Integer arguments:                                                    │
  │    a0 (x10), a1 (x11)  — first phi=2 args (also return values)       │
  │    a2 (x12), a3 (x13)  — next phi=2 args                             │
  │    a4 (x14), a5 (x15)  — last phi=2 args                             │
  │    Total: n = 6 argument registers                                     │
  │    (Standard RISC-V uses 8=sigma-tau; N6 uses 6=n for symmetry)       │
  │                                                                        │
  │  Float arguments:                                                      │
  │    fa0 (f10), fa1 (f11) — first phi=2 float args (also return)       │
  │    fa2 (f12), fa3 (f13) — next phi=2 float args                      │
  │    fa4 (f14), fa5 (f15) — last phi=2 float args                      │
  │    Total: n = 6 float argument registers                               │
  │                                                                        │
  │  Return values:                                                        │
  │    a0/a1  — integer return (phi=2 regs, up to 128-bit)               │
  │    fa0/fa1 — float return                                             │
  │                                                                        │
  │  Callee-saved:                                                         │
  │    s0-s5 (x8-x9, x18-x21)  — n=6 saved GPRs                         │
  │    fs0-fs5 (f8-f9, f18-f21) — n=6 saved FPRs                         │
  │                                                                        │
  │  Caller-saved:                                                         │
  │    t0-t5 (x5-x7, x28-x30)  — n=6 temporaries                        │
  │    ft0-ft5 (f0-f5)          — n=6 float temporaries                   │
  │                                                                        │
  │  Stack frame layout:                                                   │
  │  ┌──────────────────────────────┐  High addr                          │
  │  │ Spilled arguments (7th+)    │                                      │
  │  ├──────────────────────────────┤                                      │
  │  │ Return address (ra)         │                                      │
  │  ├──────────────────────────────┤                                      │
  │  │ Frame pointer (s0/fp)       │                                      │
  │  ├──────────────────────────────┤                                      │
  │  │ Callee-saved registers      │                                      │
  │  ├──────────────────────────────┤                                      │
  │  │ Local variables (alloca)    │  AS0: Stack pool (1/2)               │
  │  ├──────────────────────────────┤                                      │
  │  │ Arena scratch (if used)     │  AS2: Arena (1/6)                    │
  │  ├──────────────────────────────┤                                      │
  │  │ Outgoing argument area      │                                      │
  │  └──────────────────────────────┘  Low addr (sp)                      │
  │                                                                        │
  │  Frame alignment: 2^tau = 16 bytes (128-bit aligned)                  │
  │  Stack growth: downward (standard RISC-V)                              │
  └────────────────────────────────────────────────────────────────────────┘
```

---

## 4. Optimization Passes (sigma = 12 Passes)

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │              N6 OPTIMIZATION PASS PIPELINE                             │
  │              sigma = 12 passes in n/phi = 3 groups of tau = 4          │
  │                                                                        │
  │  Group A: Analysis (tau = 4 passes)                                    │
  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐                 │
  │  │ Pass 1   │ │ Pass 2   │ │ Pass 3   │ │ Pass 4   │                 │
  │  │ Egyptian │ │ Lifetime │ │ AI-Op    │ │ Proof    │                 │
  │  │ Memory   │ │ Analysis │ │ Detect   │ │ Extract  │                 │
  │  │ Classify │ │          │ │          │ │          │                 │
  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘                 │
  │       │            │            │            │                          │
  │       ▼            ▼            ▼            ▼                          │
  │  Group B: Transform (tau = 4 passes)                                   │
  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐                 │
  │  │ Pass 5   │ │ Pass 6   │ │ Pass 7   │ │ Pass 8   │                 │
  │  │ Egyptian │ │ MoE      │ │ Attention│ │ Cyclotom │                 │
  │  │ Placement│ │ Expert   │ │ Budget   │ │ Fold     │                 │
  │  │          │ │ Fusion   │ │ 1/2+1/3  │ │          │                 │
  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘                 │
  │       │            │            │            │                          │
  │       ▼            ▼            ▼            ▼                          │
  │  Group C: Finalize (tau = 4 passes)                                    │
  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐                 │
  │  │ Pass 9   │ │ Pass 10  │ │ Pass 11  │ │ Pass 12  │                 │
  │  │ Mertens  │ │ Boltzman │ │ N6 Instr │ │ Register │                 │
  │  │ Dropout  │ │ Sparsity │ │ Select   │ │ Coalesce │                 │
  │  │ Insert   │ │ Gate     │ │          │ │ + Sched  │                 │
  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘                 │
  │                                                                        │
  │  Pass count = sigma = 12                                               │
  │  Groups = n/phi = 3                                                    │
  │  Passes per group = tau = 4                                            │
  └────────────────────────────────────────────────────────────────────────┘
```

### 4.1 Pass 1: Egyptian Fraction Memory Classification

Analyzes every allocation and classifies it into one of the three Egyptian address spaces.

```
  Decision tree:
  ┌─────────────────────────────────────────────────────────────┐
  │  alloc site                                                 │
  │     │                                                       │
  │     ├── size known at compile time AND lifetime < scope?    │
  │     │   YES ──▶ AS0 (Stack, 1/2)                            │
  │     │   NO ──┐                                              │
  │     │        ├── single owner, dynamic lifetime?            │
  │     │        │   YES ──▶ AS1 (Heap, 1/3)                    │
  │     │        │   NO ──┐                                     │
  │     │        │        └── bulk/temporary/tensor?            │
  │     │        │            YES ──▶ AS2 (Arena, 1/6)          │
  │     │        │            NO  ──▶ AS1 (Heap, default)       │
  │     │        └──────────────────────────────────────────    │
  └─────────────────────────────────────────────────────────────┘

  LLVM implementation:
    - FunctionPass: EgyptianMemoryClassifier
    - Iterates over all alloca/malloc/hexa.arena.create
    - Annotates with !hexa.memregion metadata {AS, fraction}
    - Rewrites pointer addrspace when promotable
    - Target ratio: stack >= 50%, heap <= 33%, arena <= 17%
```

### 4.2 Pass 5: Egyptian Fraction Memory Placement

Transforms allocations based on classification from Pass 1.

```
  Transformations:
  1. Heap-to-stack promotion:
     %p = call ptr addrspace(1) @hexa.heap.alloc(i64 %sz)
     -->
     %p = alloca i8, i64 %sz, addrspace(0)   ; if lifetime fits scope

  2. Heap-to-arena promotion:
     %p = call ptr addrspace(1) @hexa.heap.alloc(i64 %sz)
     -->
     %p = call ptr addrspace(2) @hexa.arena.alloc(ptr %arena, i64 %sz)
     ; if bulk allocation pattern detected

  3. Arena coalescing:
     Multiple small arena allocs in same scope
     --> Single arena.create + bump allocator
```

### 4.3 Pass 6: MoE Expert Fusion

Detects patterns of gated expert dispatch and fuses them into single MOERT instructions.

```
  Pattern match (in LLVM IR):
    %g = call @softmax(...)
    %e0 = call @expert_0(...)
    %e1 = call @expert_1(...)
    %e2 = call @expert_2(...)
    %w0 = fmul %e0, %g[0]
    %w1 = fmul %e1, %g[1]
    %w2 = fmul %e2, %g[2]
    %out = fadd(fadd(%w0, %w1), %w2)

  Fused output:
    %out = call @hexa.ai.moert(%input, @e0, @e1, @e2, %g)
    ; Lowers to single MOERT instruction

  Conditions for fusion:
    - Exactly n/phi=3 experts (Egyptian fraction: 1/2+1/3+1/6)
    - Gate sum provably == 1.0 (softmax output)
    - No side effects in expert bodies
    - All experts share same input tensor
```

### 4.4 Pass 7: Attention Budget Optimization (1/2 + 1/3 + 1/6)

```
  Egyptian Fraction Attention (EFA) budget allocation:
  ┌──────────────────────────────────────────────────────────────┐
  │  Total attention budget = 1.0 (per token)                    │
  │                                                              │
  │  ┌──────────────────────────────────────────────────────┐   │
  │  │  Full attention heads:  1/2 of budget                │   │
  │  │  - Standard QKV dot-product attention                │   │
  │  │  - Applied to top-1/2 importance tokens              │   │
  │  │  - d_head = 2^(sigma-sopfr) = 128                   │   │
  │  ├──────────────────────────────────────────────────────┤   │
  │  │  Sliding window:        1/3 of budget                │   │
  │  │  - Local window = sigma^2 = 144 tokens               │   │
  │  │  - Cheaper O(n*w) instead of O(n^2)                  │   │
  │  ├──────────────────────────────────────────────────────┤   │
  │  │  Global tokens:         1/6 of budget                │   │
  │  │  - Top-n=6 global anchor tokens                      │   │
  │  │  - Cross-attend only to anchors                      │   │
  │  └──────────────────────────────────────────────────────┘   │
  │                                                              │
  │  1/2 + 1/3 + 1/6 = 1  (complete, no budget wasted)         │
  │  FLOPs saved: ~40% vs full attention                        │
  └──────────────────────────────────────────────────────────────┘

  LLVM pass:
    - Detects standard attention patterns (Q @ K^T / sqrt(d) @ V)
    - Rewrites to three-tier EFA with budget annotations
    - Inserts EFATN intrinsic calls with mode flags
```

### 4.5 Pass 8: Cyclotomic Activation Folding

```
  Cyclotomic polynomial: Phi_6(x) = x^2 - x + 1

  Detects:
    %x2 = fmul float %x, %x
    %xm = fsub float %x2, %x
    %out = fadd float %xm, 1.0

  Replaces with:
    %out = call float @hexa.ai.cyclo(float %x)
    ; Lowers to CYCLO instruction (single cycle)

  Also folds compound patterns:
    - ReLU + cyclotomic = single gated activation
    - cyclotomic + batchnorm = fused CYCLO+LAYNO pair
```

### 4.6 Pass 9: Mertens Dropout Insertion

```
  Probability: p = ln(4/3) = 0.28768207...

  For every trainable layer marked with !hexa.trainable metadata:
    - Insert Bernoulli mask with p = ln(4/3)
    - Scale surviving activations by 1/(1-p) = 1/ln(3) = 1.0986...

  LLVM IR:
    %mask = call <D x i1> @hexa.ai.merts_mask(i64 %seed, i32 D)
    %dropped = select <D x i1> %mask, <D x float> %act, <D x float> zeroinitializer
    %scaled = fmul <D x float> %dropped, <D x float> splat(1.0986)

  On RISC-V N6: single MERTS instruction handles mask+select+scale

  Key benefit: No hyperparameter search needed.
  The dropout rate is mathematically determined by n=6.
```

### 4.7 Pass 10: Boltzmann Sparsity Gate

```
  Sparsity fraction: 1/e = 0.3679 active (63.2% sparse)

  Detects dense activation tensors and inserts sparsity gates:
    %threshold = call float @hexa.ai.boltz_threshold(<D x float> %act)
    %sparse = call <D x float> @hexa.ai.boltz_gate(<D x float> %act, float %threshold)
    ; Zeros out bottom 63.2% of activations
    ; Lowers to BOLTZ instruction

  Combined with Mertens dropout:
    Training: MERTS (p=0.288) applied first, then BOLTZ (63% sparse)
    Inference: BOLTZ only (no dropout)
    Effective compute: (1-0.288) * 0.368 = 0.262 = ~26% active
```

### 4.8 Pass 11: N6 Instruction Selection

Maps LLVM IR patterns to the sigma=12 custom N6 instructions.

```
  Pattern matching priority (highest first):
  ┌────┬───────────┬─────────────────────────────────────────────────────┐
  │ #  │ Intrinsic │ IR Pattern                                         │
  ├────┼───────────┼─────────────────────────────────────────────────────┤
  │ 1  │ EFATN     │ matmul(Q,K^T)/sqrt(d) + softmax + matmul(.,V)    │
  │ 2  │ MOERT     │ softmax(gate) * [expert_i(x) for i in 0..3]      │
  │ 3  │ SWIGL     │ x * sigmoid(W1*x) * (W2*x), W ratio = 8/3       │
  │ 4  │ CYCLO     │ x*x - x + 1.0                                     │
  │ 5  │ ZETLN     │ x * tanh(1.03972... * x)                          │
  │ 6  │ BOLTZ     │ select(x > quantile(x, 0.632), x, 0)             │
  │ 7  │ MERTS     │ x * bernoulli(0.7123) * 1.0986                    │
  │ 8  │ FLASH     │ tiled_attention(Q, K, V, tile=256)                │
  │ 9  │ ROPEX     │ rotate_half(x, pos, theta=1e10)                   │
  │ 10 │ LAYNO     │ (x - mean(x)) / std(x) * gamma + beta            │
  │ 11 │ ADAMW     │ adam_step(param, grad, m, v, lr, beta1, beta2)    │
  │ 12 │ ALLRD     │ collective_reduce(tensor, op, topology)            │
  └────┴───────────┴─────────────────────────────────────────────────────┘
```

### 4.9 Full Pass Pipeline Execution Order

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │  PASS EXECUTION ORDER (sigma = 12, 3 iterations)                       │
  │                                                                        │
  │  Iteration 1 (function-level):                                         │
  │    P1:  EgyptianMemoryClassifier     — tag allocs with AS0/1/2        │
  │    P2:  LifetimeAnalysis             — compute live ranges            │
  │    P3:  AIOpDetector                 — mark fuse-candidate regions    │
  │    P4:  ProofObligationExtractor     — extract assert/invariant       │
  │                                                                        │
  │  Iteration 2 (module-level):                                           │
  │    P5:  EgyptianPlacement            — promote allocs across AS       │
  │    P6:  MoEExpertFusion              — fuse 3-expert patterns         │
  │    P7:  AttentionBudgetOptimizer     — split to 1/2+1/3+1/6          │
  │    P8:  CyclotomicActivationFold     — fold x^2-x+1 patterns         │
  │                                                                        │
  │  Iteration 3 (target-level):                                           │
  │    P9:  MertensDropoutInsertion      — insert p=ln(4/3) dropout      │
  │    P10: BoltzmannSparsityGate        — insert 1/e sparsity           │
  │    P11: N6InstructionSelection       — map to 12 custom ops          │
  │    P12: RegisterCoalesceSchedule     — allocate 72 regs, schedule    │
  │                                                                        │
  │  Standard LLVM passes run between iterations:                          │
  │    - mem2reg, SROA, GVN, LICM, instcombine, simplifycfg              │
  │    - These are target-independent and unmodified                       │
  └────────────────────────────────────────────────────────────────────────┘
```

---

## 5. Code Examples

### 5.1 Hello World: .hexa -> LLVM IR -> RISC-V Assembly

**HEXA-LANG source** (`hello.hexa`):

```hexa
fn main() {
    print("Hello, HEXA-LANG!")
}
```

**LLVM IR** (after Phase 5 codegen):

```llvm
; ModuleID = 'hello.hexa'
target datalayout = "e-m:e-p:64:64-i64:64-i128:128-n32:64-S128"
target triple = "riscv64-unknown-elf-n6"

@.str.0 = private unnamed_addr constant [18 x i8] c"Hello, HEXA-LANG!\00"

; Egyptian memory metadata
!hexa.memconfig = !{!0, !1, !2}
!0 = !{!"stack_fraction", float 0.5}
!1 = !{!"heap_fraction", float 0.333333}
!2 = !{!"arena_fraction", float 0.166667}

define void @main() #0 {
entry:
  ; String literal lives in AS0 (stack pool, 1/2)
  %str_ptr = getelementptr [18 x i8], ptr @.str.0, i64 0, i64 0
  %str = insertvalue {ptr, i64} undef, ptr %str_ptr, 0
  %str.1 = insertvalue {ptr, i64} %str, i64 17, 1
  call void @hexa.io.print({ptr, i64} %str.1)
  ret void
}

declare void @hexa.io.print({ptr, i64})

attributes #0 = { "frame-pointer"="all" "n6-convention" }
```

**RISC-V N6 assembly** (after backend):

```asm
    .text
    .globl  main
    .type   main, @function
main:
    addi    sp, sp, -16         # frame = phi^tau = 16 bytes aligned
    sd      ra, 8(sp)           # save return address
    lui     a0, %hi(.str.0)     # load string address (arg 0 of n=6)
    addi    a0, a0, %lo(.str.0)
    li      a1, 17              # string length (arg 1)
    call    hexa.io.print       # call with n=6 convention
    ld      ra, 8(sp)
    addi    sp, sp, 16
    ret

    .section .rodata
.str.0:
    .asciz  "Hello, HEXA-LANG!"
```

### 5.2 Egyptian MoE Routing: .hexa -> Optimized IR

**HEXA-LANG source** (`moe.hexa`):

```hexa
struct EgyptianMoE {
    w_gate: Tensor,
    expert_half:  fn(Tensor) -> Tensor,   // 1/2
    expert_third: fn(Tensor) -> Tensor,   // 1/3
    expert_sixth: fn(Tensor) -> Tensor,   // 1/6
}

fn route(moe: &EgyptianMoE, input: Tensor) -> Tensor {
    let gate = softmax(moe.w_gate @ input)

    // Egyptian fraction: 1/2 + 1/3 + 1/6 = 1
    let h = (moe.expert_half)(input)  * gate[0]
    let t = (moe.expert_third)(input) * gate[1]
    let s = (moe.expert_sixth)(input) * gate[2]

    return h + t + s
}

proof egyptian_complete {
    invariant 1.0/2.0 + 1.0/3.0 + 1.0/6.0 == 1.0
}
```

**LLVM IR (before N6 optimization)**:

```llvm
define <256 x float> @route(ptr %moe, <256 x float> %input) #0 {
entry:
  ; Gate computation
  %w_gate = load ptr, ptr %moe
  %dot = call <3 x float> @hexa.linalg.matvec(ptr %w_gate, <256 x float> %input)
  %gate = call <3 x float> @hexa.ai.softmax_3(<3 x float> %dot)

  ; Extract gate values
  %g0 = extractelement <3 x float> %gate, i32 0
  %g1 = extractelement <3 x float> %gate, i32 1
  %g2 = extractelement <3 x float> %gate, i32 2

  ; Expert dispatch
  %fn_half = load ptr, ptr getelementptr(%moe, 0, 1)
  %fn_third = load ptr, ptr getelementptr(%moe, 0, 2)
  %fn_sixth = load ptr, ptr getelementptr(%moe, 0, 3)

  %e0 = call <256 x float> %fn_half(<256 x float> %input)
  %e1 = call <256 x float> %fn_third(<256 x float> %input)
  %e2 = call <256 x float> %fn_sixth(<256 x float> %input)

  ; Weighted sum
  %g0s = shufflevector <1 x float> %g0, ..., <256 x float> splat
  %g1s = shufflevector <1 x float> %g1, ..., <256 x float> splat
  %g2s = shufflevector <1 x float> %g2, ..., <256 x float> splat

  %w0 = fmul <256 x float> %e0, %g0s
  %w1 = fmul <256 x float> %e1, %g1s
  %w2 = fmul <256 x float> %e2, %g2s

  %t = fadd <256 x float> %w0, %w1
  %out = fadd <256 x float> %t, %w2

  ret <256 x float> %out
}
```

**LLVM IR (after Pass 6: MoE Expert Fusion)**:

```llvm
define <256 x float> @route(ptr %moe, <256 x float> %input) #0 {
entry:
  ; Fused MoE routing — single intrinsic
  %out = call <256 x float> @hexa.ai.moert(
    <256 x float> %input,
    ptr %moe,              ; struct contains gate weights + expert fns
    i32 3,                 ; n/phi = 3 experts
    i32 0                  ; mode: Egyptian fraction (1/2+1/3+1/6)
  ), !hexa.egyptian !{float 0.5, float 0.333, float 0.167}

  ret <256 x float> %out
}

; Proof obligation extracted by Pass 4, verified statically
; !hexa.proof.egyptian_complete = true
```

**RISC-V N6 assembly (after backend)**:

```asm
route:
    addi    sp, sp, -16
    sd      ra, 8(sp)
    # a0 = ptr to moe struct, v1 = input tensor (in VEC bank)
    ld      a1, 0(a0)           # load gate weights ptr
    ld      a2, 8(a0)           # load expert_half fn ptr
    ld      a3, 16(a0)          # load expert_third fn ptr
    ld      a4, 24(a0)          # load expert_sixth fn ptr
    # Fused MoE routing: single N6 custom instruction
    MOERT   v2, v1, a1          # v2 = moert(input=v1, gate=a1)
                                # mode=000 (f32 vector), bank=010 (VEC)
                                # experts loaded from a2-a4 implicitly
    # v2 now contains the weighted expert sum
    ld      ra, 8(sp)
    addi    sp, sp, 16
    ret
```

### 5.3 Fibonacci with Proof: .hexa -> Verified Assembly

**HEXA-LANG source** (`fib.hexa`):

```hexa
fn fib(n: int) -> int {
    proof fib_positive {
        invariant n >= 0
        invariant fib(n) >= 0
    }

    if n <= 1 { return n }

    let mut a = 0
    let mut b = 1
    for _ in 2..=n {
        let temp = a + b
        a = b
        b = temp
    }
    return b
}

fn main() {
    // Compute fib(n) for n = 6 (the perfect number)
    let result = fib(6)
    assert result == 8    // fib(6) = sigma - tau = 8
    print("fib({6}) = {result}")
}
```

**LLVM IR**:

```llvm
define i64 @fib(i64 %n) #0 {
entry:
  ; Pass 4 proof extraction: n >= 0 assumed
  call void @llvm.assume(i1 icmp sge (i64 %n, i64 0))
  %cmp = icmp sle i64 %n, 1
  br i1 %cmp, label %base, label %loop.preheader

base:
  ret i64 %n

loop.preheader:
  br label %loop

loop:
  %i = phi i64 [ 2, %loop.preheader ], [ %i.next, %loop ]
  %a = phi i64 [ 0, %loop.preheader ], [ %b, %loop ]
  %b = phi i64 [ 1, %loop.preheader ], [ %temp, %loop ]
  %temp = add nsw i64 %a, %b
  %i.next = add nuw i64 %i, 1
  %done = icmp sgt i64 %i.next, %n
  br i1 %done, label %exit, label %loop

exit:
  %result = phi i64 [ %temp, %loop ]
  ; Proof obligation: result >= 0 (verified by llvm.assume propagation)
  ret i64 %result
}

define void @main() #0 {
entry:
  %result = call i64 @fib(i64 6)
  ; Compile-time assertion: fib(6) == 8
  %check = icmp eq i64 %result, 8
  call void @hexa.proof.check(i1 %check, metadata !"fib(6) == sigma-tau")
  ; ... print call omitted for brevity ...
  ret void
}
```

**RISC-V N6 assembly**:

```asm
    .text
    .globl  fib
fib:
    # a0 = n (first of n=6 argument registers)
    li      a1, 1
    ble     a0, a1, .fib_base   # n <= 1: return n
    li      a2, 0               # a = 0
    li      a3, 1               # b = 1
    li      a4, 2               # i = 2
.fib_loop:
    add     a5, a2, a3          # temp = a + b
    mv      a2, a3              # a = b
    mv      a3, a5              # b = temp
    addi    a4, a4, 1           # i++
    ble     a4, a0, .fib_loop   # while i <= n
    mv      a0, a3              # return b
    ret
.fib_base:
    ret                          # return n (already in a0)

    .globl  main
main:
    addi    sp, sp, -16
    sd      ra, 8(sp)
    li      a0, 6               # fib(n=6)
    call    fib
    # a0 = 8 = sigma-tau (verified at compile time)
    li      a1, 8
    bne     a0, a1, .assert_fail
    # ... print ...
    ld      ra, 8(sp)
    addi    sp, sp, 16
    ret
.assert_fail:
    call    hexa.panic           # unreachable if proof holds
```

---

## 6. Performance Model

### 6.1 Instruction Throughput Per Pipeline Stage

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │         HEXA-P PIPELINE THROUGHPUT MODEL                               │
  │         Pipeline: sigma+n = 18 stages                                  │
  │         Frequency target: 3.0 GHz (n/phi = 3)                         │
  │                                                                        │
  │  Stage          Depth   Width       Throughput          Bottleneck     │
  │  ─────          ─────   ─────       ──────────          ──────────     │
  │  Fetch          n/phi=3 sigma-tau=8 8 instr/cycle       I-cache BW    │
  │  Decode         n/phi=3 sopfr=5     5 uops/cycle(fused) Complex dec   │
  │  Rename         phi=2   n=6         6 uops/cycle        RAT ports     │
  │  Schedule       phi=2   sigma-tau=8 8 dispatched/cycle  Wakeup logic  │
  │  Execute        n=6     18 ports    18 ops/cycle(peak)  FU latency    │
  │  Retire         phi=2   sigma-tau=8 8 retired/cycle     ROB drain     │
  │                                                                        │
  │  Sustained IPC (estimated):                                            │
  │    Integer workload:  ~5.0 IPC  (n/phi + phi = 5 effective)           │
  │    FP/Vector:         ~3.5 IPC  (limited by FP port tau=4)            │
  │    AI workload:       ~2.0 IPC  (limited by MOERT/EFATN latency)     │
  │    Mixed realistic:   ~4.0 IPC  (sopfr-1 = 4)                        │
  │                                                                        │
  │  N6 Custom Instruction Latencies:                                      │
  │  ┌──────────┬─────────┬────────────┬──────────────────────────┐       │
  │  │ Instr    │ Latency │ Throughput │ Notes                    │       │
  │  ├──────────┼─────────┼────────────┼──────────────────────────┤       │
  │  │ CYCLO    │ 1 cyc   │ 1/cycle    │ Fused multiply-add       │       │
  │  │ ZETLN    │ 3 cyc   │ 1/cycle    │ tanh lookup + mul        │       │
  │  │ BOLTZ    │ 2 cyc   │ 1/cycle    │ Compare + select         │       │
  │  │ MERTS    │ 2 cyc   │ 1/cycle    │ RNG + select + scale     │       │
  │  │ LAYNO    │ 6 cyc   │ 1/3 cycle  │ Reduction + normalize    │       │
  │  │ ROPEX    │ 4 cyc   │ 1/cycle    │ Sin/cos + rotate         │       │
  │  │ SWIGL    │ 4 cyc   │ 1/2 cycle  │ Sigmoid + 2x multiply   │       │
  │  │ EFATN    │ 12 cyc  │ 1/6 cycle  │ Full attention tile      │       │
  │  │ MOERT    │ 12 cyc  │ 1/6 cycle  │ 3-expert gated sum      │       │
  │  │ FLASH    │ 12 cyc  │ 1/6 cycle  │ Tiled flash attention    │       │
  │  │ ADAMW    │ 8 cyc   │ 1/4 cycle  │ 5-value update           │       │
  │  │ ALLRD    │ 24 cyc  │ 1/12 cycle │ Cross-core communication │       │
  │  └──────────┴─────────┴────────────┴──────────────────────────┘       │
  │                                                                        │
  │  Latency pattern: {1,2,3,4,6,8,12,24} = divisors and multiples of 6  │
  └────────────────────────────────────────────────────────────────────────┘
```

### 6.2 Register Pressure Analysis

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │         REGISTER PRESSURE BY WORKLOAD TYPE                             │
  │                                                                        │
  │  Workload         GPR    FPR    VEC    ACC    AI     Total  Pressure  │
  │  ────────         ───    ───    ───    ───    ──     ─────  ────────  │
  │  Integer scalar   8/12   0/12   0/12   0/12   0/12   8/72   LOW      │
  │  FP scalar        4/12   8/12   0/12   0/12   0/12   12/72  LOW      │
  │  SIMD vectorized  3/12   0/12   8/12   2/12   0/12   13/72  LOW      │
  │  MoE routing      4/12   2/12   6/12   3/12   6/12   21/72  MEDIUM   │
  │  Full attention   4/12   0/12  10/12   4/12  10/12   28/72  MEDIUM   │
  │  Transformer blk  6/12   4/12  10/12   6/12  12/12   38/72  HIGH     │
  │  Training loop    8/12   6/12  12/12   8/12  12/12   46/72  HIGH     │
  │                                                                        │
  │  Spill threshold:                                                      │
  │    Per bank: > sigma-phi = 10 active registers -> spill to L1D        │
  │    Total: > sigma*tau = 48 active -> spill to L2                      │
  │    L1D spill latency: tau = 4 cycles                                  │
  │    L2 spill latency: sigma = 12 cycles                                │
  │                                                                        │
  │  Physical register file (from HEXA-CORE spec):                        │
  │    INT physical regs:  sigma*J_2 = 288                                │
  │    FP/VEC physical regs: sigma*J_2 = 288                              │
  │    Renaming capacity: 288/12 = J_2 = 24 in-flight per arch reg       │
  │                                                                        │
  │  With 72 architectural + 576 physical regs:                            │
  │    Rename ratio = 576/72 = sigma-tau = 8x overhead                    │
  │    Sufficient for all workloads without excessive spilling             │
  └────────────────────────────────────────────────────────────────────────┘
```

### 6.3 Memory Bandwidth Utilization

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │         EGYPTIAN MEMORY BANDWIDTH MODEL                                │
  │                                                                        │
  │  Memory Level     Bandwidth          Latency    Egyptian Fraction      │
  │  ────────────     ─────────          ───────    ────────────────       │
  │  L1D (AS0)        128 B/cyc (R)      tau=4 cyc  Stack (1/2 traffic)   │
  │                   64 B/cyc (W)                                        │
  │  L2 (AS0+AS1)     64 B/cyc           sigma=12   Heap (1/3 traffic)   │
  │  L3/SLC           48 B/cyc           sigma*tau=48  Mixed              │
  │  HBM              128 GB/s           ~200 cyc   Arena (1/6 traffic)   │
  │                                                                        │
  │  Bandwidth allocation by Egyptian fraction:                            │
  │                                                                        │
  │  ┌─────────────────────────────────────────────────────────────────┐   │
  │  │  Total BW = 128 GB/s (HBM)                                     │   │
  │  │                                                                 │   │
  │  │  ┌───────────────────────────────┐  64 GB/s  = 1/2 of total    │   │
  │  │  │ Stack traffic (AS0)           │  Value types, frames, locals │   │
  │  │  │ Fast path: L1D hit rate >95%  │  Rarely reaches HBM         │   │
  │  │  ├───────────────────────────────┤                              │   │
  │  │  │ Heap traffic (AS1)            │  42.7 GB/s = 1/3 of total   │   │
  │  │  │ Ref-counted objects           │  Moderate reuse              │   │
  │  │  ├───────────────────────────────┤                              │   │
  │  │  │ Arena traffic (AS2)           │  21.3 GB/s = 1/6 of total   │   │
  │  │  │ Bulk tensors, temp buffers    │  Streaming, low reuse        │   │
  │  │  └───────────────────────────────┘                              │   │
  │  │                                                                 │   │
  │  │  Key insight: Arena (1/6) handles AI tensors which are          │   │
  │  │  large but streaming — minimal cache pollution.                 │   │
  │  │  Stack (1/2) handles hot scalar data — stays in L1D.           │   │
  │  └─────────────────────────────────────────────────────────────────┘   │
  │                                                                        │
  │  Effective bandwidth utilization:                                       │
  │    AI inference:  ~85% of peak (tensor streaming in AS2)              │
  │    AI training:   ~72% of peak (gradient traffic in AS1+AS2)          │
  │    General code:  ~40% of peak (dominated by AS0 L1D hits)            │
  │                                                                        │
  │  Roofline model intersection:                                          │
  │    Compute peak: sigma^2 = 144 TFLOPS (NPU, J_2=24 cores)            │
  │    Memory BW: 128 GB/s (HBM3E)                                        │
  │    Ridge point: 144 / 0.128 = 1125 FLOP/byte                         │
  │    Transformer attention: ~200 FLOP/byte -> memory bound              │
  │    Transformer FFN: ~1500 FLOP/byte -> compute bound                  │
  │    EFATN advantage: reduces attention FLOP by 40% -> better balance   │
  └────────────────────────────────────────────────────────────────────────┘
```

---

## 7. LLVM Integration Architecture

### 7.1 Backend Registration

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │         LLVM BACKEND FILE STRUCTURE                                    │
  │                                                                        │
  │  llvm/lib/Target/RISCVN6/                                              │
  │  ├── RISCVN6.td                     # TableGen target description      │
  │  ├── RISCVN6InstrInfo.td            # sigma=12 custom instructions     │
  │  ├── RISCVN6RegisterInfo.td         # 72 registers, 6 banks           │
  │  ├── RISCVN6CallingConv.td          # n=6 argument convention          │
  │  ├── RISCVN6InstrFormats.td         # J_2=24 bit encoding formats     │
  │  ├── RISCVN6Subtarget.h/cpp         # Feature flags (n6-ai, n6-vec)   │
  │  ├── RISCVN6TargetMachine.h/cpp     # Target machine entry point      │
  │  ├── RISCVN6ISelDAGToDAG.cpp        # SelectionDAG instruction select │
  │  ├── RISCVN6ISelLowering.h/cpp      # Custom lowering for N6 ops      │
  │  ├── RISCVN6FrameLowering.h/cpp     # Egyptian stack frame layout     │
  │  ├── RISCVN6AsmPrinter.cpp          # MC emission for N6 instructions │
  │  ├── RISCVN6MCTargetDesc.cpp        # MC layer registration           │
  │  └── N6Passes/                       # sigma=12 custom passes          │
  │      ├── EgyptianMemoryClassifier.cpp                                  │
  │      ├── LifetimeAnalysis.cpp                                          │
  │      ├── AIOpDetector.cpp                                              │
  │      ├── ProofObligationExtractor.cpp                                  │
  │      ├── EgyptianPlacement.cpp                                         │
  │      ├── MoEExpertFusion.cpp                                           │
  │      ├── AttentionBudgetOptimizer.cpp                                  │
  │      ├── CyclotomicActivationFold.cpp                                  │
  │      ├── MertensDropoutInsertion.cpp                                   │
  │      ├── BoltzmannSparsityGate.cpp                                     │
  │      ├── N6InstructionSelection.cpp                                    │
  │      └── RegisterCoalesceSchedule.cpp                                  │
  │                                                                        │
  │  File count in N6Passes/ = sigma = 12                                  │
  └────────────────────────────────────────────────────────────────────────┘
```

### 7.2 Intrinsic Declarations (for Clang/LLVM integration)

```llvm
; === HEXA-LANG N6 Intrinsics ===
; Declared in llvm/include/llvm/IR/IntrinsicsRISCVN6.td

; AI compute intrinsics (12 = sigma)
declare <vscale x 16 x float> @llvm.riscvn6.efatn(<vscale x 16 x float> %Q,
    <vscale x 16 x float> %K, <vscale x 16 x float> %V, i32 %mode)

declare <vscale x 16 x float> @llvm.riscvn6.moert(<vscale x 16 x float> %input,
    ptr %experts, <3 x float> %gate, i32 %mode)

declare <vscale x 16 x float> @llvm.riscvn6.swigl(<vscale x 16 x float> %x,
    ptr %W1, ptr %W2, i32 %mode)

declare float @llvm.riscvn6.cyclo(float %x)

declare float @llvm.riscvn6.zetln(float %x)

declare <vscale x 16 x float> @llvm.riscvn6.boltz(<vscale x 16 x float> %x,
    float %threshold)

declare <vscale x 16 x float> @llvm.riscvn6.merts(<vscale x 16 x float> %x,
    i64 %seed)

declare <vscale x 16 x float> @llvm.riscvn6.flash(<vscale x 16 x float> %Q,
    <vscale x 16 x float> %K, <vscale x 16 x float> %V, i32 %tile_size)

declare <vscale x 16 x float> @llvm.riscvn6.ropex(<vscale x 16 x float> %x,
    i64 %position, double %theta)

declare <vscale x 16 x float> @llvm.riscvn6.layno(<vscale x 16 x float> %x,
    <vscale x 16 x float> %gamma, <vscale x 16 x float> %beta, i32 %mode)

declare void @llvm.riscvn6.adamw(ptr %param, ptr %grad, ptr %m, ptr %v,
    float %lr, float %beta1, float %beta2, float %eps, float %wd)

declare <vscale x 16 x float> @llvm.riscvn6.allrd(<vscale x 16 x float> %tensor,
    i32 %op, i32 %topology)

; Memory intrinsics (Egyptian address spaces)
declare ptr addrspace(1) @hexa.heap.alloc(i64 %size) nounwind
declare void @hexa.heap.free(ptr addrspace(1) %ptr) nounwind
declare ptr addrspace(2) @hexa.arena.create(i64 %capacity) nounwind
declare ptr addrspace(2) @hexa.arena.alloc(ptr addrspace(2) %arena, i64 %size) nounwind
declare void @hexa.arena.destroy(ptr addrspace(2) %arena) nounwind
```

### 7.3 TableGen Instruction Definitions (excerpt)

```tablegen
// RISCVN6InstrInfo.td — N6 Custom Instruction Definitions

class N6AIInst<bits<4> n6op, string mnemonic, dag outs, dag ins>
    : RVInst<outs, ins, mnemonic, "", [], InstFormatR> {
  bits<5> rs2;
  bits<5> rs1;
  bits<3> mode;
  bits<5> rd;
  bits<3> bank;

  let Inst{31-28} = n6op;
  let Inst{27-25} = bank;
  let Inst{24-20} = rs2;
  let Inst{19-15} = rs1;
  let Inst{14-12} = mode;
  let Inst{11-7}  = rd;
  let Inst{6-0}   = 0b0101011;  // custom-2 opcode
}

// sigma = 12 instructions
def EFATN : N6AIInst<0b0000, "efatn", (outs VR:$rd), (ins VR:$rs1, VR:$rs2)>;
def MOERT : N6AIInst<0b0001, "moert", (outs VR:$rd), (ins VR:$rs1, GPR:$rs2)>;
def SWIGL : N6AIInst<0b0010, "swigl", (outs VR:$rd), (ins VR:$rs1, VR:$rs2)>;
def CYCLO : N6AIInst<0b0011, "cyclo", (outs FPR:$rd), (ins FPR:$rs1, FPR:$rs2)>;
def ZETLN : N6AIInst<0b0100, "zetln", (outs FPR:$rd), (ins FPR:$rs1, FPR:$rs2)>;
def BOLTZ : N6AIInst<0b0101, "boltz", (outs VR:$rd), (ins VR:$rs1, FPR:$rs2)>;
def MERTS : N6AIInst<0b0110, "merts", (outs VR:$rd), (ins VR:$rs1, GPR:$rs2)>;
def FLASH : N6AIInst<0b0111, "flash", (outs VR:$rd), (ins VR:$rs1, VR:$rs2)>;
def ROPEX : N6AIInst<0b1000, "ropex", (outs VR:$rd), (ins VR:$rs1, GPR:$rs2)>;
def LAYNO : N6AIInst<0b1001, "layno", (outs VR:$rd), (ins VR:$rs1, VR:$rs2)>;
def ADAMW : N6AIInst<0b1010, "adamw", (outs GPR:$rd), (ins GPR:$rs1, GPR:$rs2)>;
def ALLRD : N6AIInst<0b1011, "allrd", (outs VR:$rd), (ins VR:$rs1, GPR:$rs2)>;
```

---

## 8. N6 Consistency Verification

```
  ┌──────────────────────────────────────────────────────────────────────────┐
  │           LLVM BACKEND n=6 DESIGN CONSTANTS — 24/24 EXACT               │
  │                                                                          │
  │  sigma(n)*phi(n) = n*tau(n)  <=>  12*2 = 6*4 = 24  <=>  n = 6          │
  │                                                                          │
  │  # │ Design Element                │ Value  │ n=6 Formula    │ Grade    │
  │  ──┼───────────────────────────────┼────────┼────────────────┼────────  │
  │  1 │ Compilation phases            │ 6      │ n              │ EXACT    │
  │  2 │ Grammar levels                │ 6      │ n              │ EXACT    │
  │  3 │ Paradigm count                │ 6      │ n              │ EXACT    │
  │  4 │ Primitive type count          │ 8      │ sigma-tau      │ EXACT    │
  │  5 │ Type layers                   │ 4      │ tau            │ EXACT    │
  │  6 │ Total keywords                │ 53     │ sigma*tau+sopfr│ EXACT    │
  │  7 │ Total operators               │ 24     │ J_2            │ EXACT    │
  │  8 │ Optimization passes           │ 12     │ sigma          │ EXACT    │
  │  9 │ Pass groups                   │ 3      │ n/phi          │ EXACT    │
  │ 10 │ Passes per group              │ 4      │ tau            │ EXACT    │
  │ 11 │ Custom fused instructions     │ 12     │ sigma          │ EXACT    │
  │ 12 │ Register banks                │ 6      │ n              │ EXACT    │
  │ 13 │ Registers per bank            │ 12     │ sigma          │ EXACT    │
  │ 14 │ Total registers               │ 72     │ sigma*n        │ EXACT    │
  │ 15 │ Argument registers (int)      │ 6      │ n              │ EXACT    │
  │ 16 │ Argument registers (float)    │ 6      │ n              │ EXACT    │
  │ 17 │ Return registers              │ 2      │ phi            │ EXACT    │
  │ 18 │ Address spaces                │ 3      │ n/phi          │ EXACT    │
  │ 19 │ Inst encoding payload (bits)  │ 24     │ J_2            │ EXACT    │
  │ 20 │ Vector width (elements)       │ 48     │ sigma*tau      │ EXACT    │
  │ 21 │ Frame alignment (bytes)       │ 16     │ phi^tau        │ EXACT    │
  │ 22 │ Pipeline stages               │ 18     │ sigma+n        │ EXACT    │
  │ 23 │ MoE expert count              │ 3      │ n/phi          │ EXACT    │
  │ 24 │ Egyptian fractions            │ 1/2+1/3+1/6 │ d|n -> 1/d  │ EXACT│
  │    │                               │        │                │          │
  │    │ EXACT rate: 24/24 = 100%      │        │                │          │
  └──────────────────────────────────────────────────────────────────────────┘
```

---

## 9. Implementation Roadmap

```
  ┌────────────────────────────────────────────────────────────────────────┐
  │         IMPLEMENTATION PHASES (n = 6)                                  │
  │                                                                        │
  │  Phase 1: Foundation (Month 1-2)                                       │
  │    - RISCVN6 target registration in LLVM                               │
  │    - Register file definition (72 regs, 6 banks)                       │
  │    - Basic instruction encoding (J_2=24 bit payload)                   │
  │    - Calling convention (n=6 args)                                     │
  │                                                                        │
  │  Phase 2: Core Backend (Month 3-4)                                     │
  │    - Type mapping (8 primitives -> LLVM types)                         │
  │    - Address space support (AS0/AS1/AS2)                               │
  │    - Standard RISC-V instruction selection                             │
  │    - Frame lowering with Egyptian stack layout                         │
  │                                                                        │
  │  Phase 3: N6 Instructions (Month 5-6)                                  │
  │    - TableGen definitions for sigma=12 custom ops                      │
  │    - Intrinsic declarations                                            │
  │    - Basic ISel patterns for CYCLO, ZETLN, BOLTZ                      │
  │    - Assembly printer / disassembler                                   │
  │                                                                        │
  │  Phase 4: Optimization Passes (Month 7-9)                              │
  │    - Egyptian memory classifier + placement (P1, P5)                   │
  │    - AI op detection + fusion (P3, P6, P7, P8)                        │
  │    - Dropout / sparsity insertion (P9, P10)                            │
  │    - Full N6 instruction selection (P11)                               │
  │                                                                        │
  │  Phase 5: Verification (Month 10-11)                                   │
  │    - Proof obligation extraction (P4)                                  │
  │    - LLVM LIT test suite (one per instruction)                         │
  │    - FileCheck patterns for all 12 custom instructions                │
  │    - Performance regression tests                                      │
  │                                                                        │
  │  Phase 6: Integration (Month 12)                                       │
  │    - HEXA-LANG frontend -> LLVM IR emission                            │
  │    - End-to-end: .hexa -> binary                                       │
  │    - Benchmark suite (attention, MoE, FFN, training loop)             │
  │    - Documentation and upstream proposal                               │
  │                                                                        │
  │  Phases = n = 6                                                        │
  │  Total months = sigma = 12                                             │
  └────────────────────────────────────────────────────────────────────────┘
```

---

## References

- HEXA-LANG spec: `docs/programming-language/hexa-lang-spec.md`
- HEXA-CORE microarchitecture: `docs/chip-architecture/hexa-core.md`
- BT-33: Transformer sigma=12 atom
- BT-54: AdamW quintuplet
- BT-56: Complete n=6 LLM
- BT-58: sigma-tau=8 universal AI constant
- BT-59: 8-layer AI stack
- BT-65: Mamba SSM complete n=6
- BT-67: MoE activation fraction law
- LLVM RISC-V backend: https://llvm.org/docs/RISCVUsage.html
- RISC-V custom extensions: Chapter 29, RISC-V ISA Specification
