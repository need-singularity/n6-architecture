# HEXA-IR / HEXA-LANG Knowledge Base

> Consolidated knowledge from all project memories + source code analysis.
> Last updated: 2026-04-04

---

## 1. Compiler Architecture

### Pipeline (n=6 stages)

```
Source -> [Lexer] -> [Parser] -> [Sema] -> [Lower] -> [Opt] -> [Codegen] -> Native Binary
          Stage 1    Stage 2     Stage 3    Stage 4    Stage 5   Stage 6
```

Module dependency (topological order):
```
util -> ir -> diag -> proof -> alloc -> lexer -> parser -> sema -> lower -> opt -> codegen
```

### Module Map (67 .rs files, ~13,288 LOC)

| Module | Files | Role |
|--------|-------|------|
| `util/` | n6.rs, intern.rs, mod.rs | n=6 constants (SSOT), string interning, RNG |
| `ir/` | opcode.rs, instr.rs, types.rs, builder.rs, print.rs, mod.rs | J2=24 opcodes, SSA instructions, sigma-tau=8 primitives |
| `diag/` | message.rs, render.rs, mod.rs | Diagnostic messages + terminal rendering |
| `proof/` | ownership.rs, invariant.rs, mod.rs | Proof obligation tracking (ownership, invariants) |
| `alloc/` | arena.rs, egyptian.rs, mod.rs | Arena allocator + Egyptian fraction heap (1/2+1/3+1/6=1) |
| `lexer/` | cursor.rs, error.rs, keyword.rs, span.rs, token.rs, mod.rs | Tokenization: 20 keywords (sigma=12 flow + sigma-tau=8 type) |
| `parser/` | ast.rs, decl.rs, error.rs, expr.rs, stmt.rs, mod.rs | Recursive descent: Decl(8) + Stmt(7) + Expr(17) + Pattern(7) |
| `sema/` | resolve.rs, typecheck.rs, ownership.rs, trait_impl.rs, error.rs, mod.rs | 3-layer analysis: types + ownership + traits |
| `lower/` | expr_lower.rs, stmt_lower.rs, closure.rs, pattern.rs, proof_emit.rs, mod.rs | AST -> HEXA-IR lowering with proof emission |
| `opt/` | front/(4), mid/(4), back/(4), proof_info.rs, mod.rs | sigma=12 passes in n/phi=3 waves |
| `codegen/` | regalloc.rs, x86_64.rs, arm64.rs, elf.rs, macho.rs, mod.rs | ARM64 asm (primary) + x86-64 + ELF/Mach-O |

---

## 2. n=6 Constant Mapping

All compiler constants derive from sigma(n)*phi(n) = n*tau(n), uniquely n=6.

### Primary Constants (from `util/n6.rs`)

| Constant | Value | Formula | Compiler Usage |
|----------|-------|---------|----------------|
| N | 6 | perfect number | Pipeline stages, paradigms |
| PHI | 2 | phi(6) | Compile modes, linear/nonlinear |
| TAU | 4 | tau(6) | Opcode categories, type layers, pass groups |
| SIGMA | 12 | sigma(6) | Optimization passes, keywords |
| SOPFR | 5 | 2+3 | Error categories, fingers |
| J2 | 24 | J_2(6) | Total opcodes, Leech lattice dim |
| MU | 1 | mu(6) | Squarefree, byte size |

### Derived Constants

| Constant | Value | Formula | Usage |
|----------|-------|---------|-------|
| SIGMA_TAU | 8 | sigma-tau | Primitive types, inline threshold |
| SIGMA_PHI | 10 | sigma-phi | Improvement multiplier |
| N_PHI | 3 | n/phi | Pass wave groups (front/mid/back) |
| PHI_TAU | 16 | 2^tau | x86-64 register count |
| SIGMA_SQ | 144 | sigma^2 | Benchmark scale |
| PHI_N | 64 | 2^n | Store address space |
| PHI_SOPFR | 32 | 2^sopfr | Load address space |
| BLOCK_LARGE | 4096 | 2^sigma | Egyptian region A block |
| BLOCK_MEDIUM | 1024 | 2^(sigma-phi) | Egyptian region B block |
| BLOCK_SMALL | 256 | 2^(sigma-tau) | Egyptian region C block |
| BLOCK_MIN | 64 | 2^n | Minimum allocation |

---

## 3. Instruction Set (J2=24)

Organized into tau=4 categories of n=6 each:

| Category | Opcodes | Purpose |
|----------|---------|---------|
| Arithmetic | Add, Sub, Mul, Div, Mod, Neg | Computation |
| Memory | Load, Store, Alloc, Free, Copy, Move | Data movement |
| Control | Jump, Branch, Call, Return, Phi, Switch | Flow control |
| Proof | ProofAssert, ProofInvariant, ProofWitness, OwnershipTransfer, BorrowCheck, LifetimeEnd | Zero-cost verification (HEXA unique) |

The Proof category is what differentiates HEXA-IR from LLVM/WASM/JVM:
- Proof instructions are emitted during lowering
- Exploited by optimization passes P2, P6, P11 for extra eliminations
- Erased at codegen (zero runtime cost)

---

## 4. Type System

### Primitives (sigma-tau = 8)

| Type | Size | n=6 size constant |
|------|------|-------------------|
| i64 | 8 bytes | SIGMA_TAU |
| f64 | 8 bytes | SIGMA_TAU |
| bool | 1 byte | MU |
| char | 4 bytes | TAU (UTF-32) |
| str | 16 bytes | PHI_TAU (ptr+len) |
| byte | 1 byte | MU |
| void | 0 bytes | 0 |
| any | 8 bytes | SIGMA_TAU |

### Compound Types (tau = 4)

Struct, Enum, Array, Fn

---

## 5. Optimization Pipeline (sigma=12 passes)

Three waves of tau=4 passes each (n/phi=3 waves):

### Front (P1-P4): Early Cleanup
- P1: Type Inference (Hindley-Milner style, resolve Any types)
- P2: Ownership Proof (deduplicate BorrowCheck instructions)
- P3: Dead Store Elimination (proof-guided)
- P4: Constant Folding + redundant load elimination

### Mid (P5-P8): Core Optimization
- P5: Function Inlining (threshold: sigma-tau=8 instructions)
- P6: Loop Invariant Code Motion (proof-guided LICM)
- P7: Common Subexpression Elimination
- P8: Strength Reduction

### Back (P9-P12): Final Cleanup + Verification
- P9: Code Sinking (move computations to use sites)
- P10: Copy Coalescing (algebraic simplification)
- P11: Final DCE (proof-guided dead code elimination)
- P12: IR Verification (read-only invariant check)

**HEXA advantage**: Passes P2, P6, P11 exploit proof instructions to eliminate code that LLVM must conservatively keep.

---

## 6. Semantic Analysis (3 Layers)

| Layer | Module | Purpose |
|-------|--------|---------|
| 1 | typecheck.rs + resolve.rs | Name resolution + Hindley-Milner type checking |
| 2 | ownership.rs | Single-owner move semantics, borrow checking, use-after-move detection |
| 3 | trait_impl.rs | Trait definition/impl verification, dispatch tables |

Ownership states: Owned, ImmutBorrowed(count), MutBorrowed(borrower), Moved(span)

---

## 7. Code Generation

### Targets
- **ARM64 macOS** (primary, via assembly text emission)
- x86-64 Linux (ELF binary)
- x86-64 macOS (Mach-O binary)

### Built-in Functions (syscall-based, no libc)
- `print(s: str)` -- SYS_write to stdout
- `file_open(path: str)` -- SYS_open
- `file_read(fd, buf, n)` -- SYS_read
- `file_write(fd, data, n)` -- SYS_write
- `file_close(fd)` -- SYS_close
- `heap_alloc(size)` -- SYS_mmap
- `heap_free(ptr, size)` -- SYS_munmap

### Egyptian Allocator
Memory split into 3 regions by Egyptian fractions of 6:
- Region A: 1/2 of heap (block = 4096 = 2^sigma)
- Region B: 1/3 of heap (block = 1024 = 2^(sigma-phi))
- Region C: 1/6 of heap (block = 256 = 2^(sigma-tau))
- External fragmentation = 0 by design

---

## 8. Supported Features (Mk.I + Current)

### Language Features
- Integer/float/bool/string literals
- Arithmetic: +, -, *, /, %
- Comparison: ==, !=, <, >, <=, >=
- Logical: &&, ||, !
- let/mut variable bindings
- if/else conditionals
- while loops, for loops
- Functions with parameters and return types
- Struct definitions + field access + initialization
- Enum definitions with variants
- Trait definitions + impl blocks
- Match expressions with patterns (wildcard, literal, binding, variant, struct, tuple, guard)
- Closures: `|x, y| expr`
- Generics: `fn foo<T>(x: T)`
- Module system: mod/use/pub
- Type aliases
- Array literals + indexing
- Try expressions: `expr?`
- String literals with escape sequences
- String pool deduplication

### Parser Constructs
- 20 keywords: 12 flow (fn, let, mut, if, else, while, return, struct, enum, true, false, type) + 8 extended (match, mod, use, pub, trait, impl, for, in)
- 8 type keywords: i64, f64, bool, char, str, byte, void, any
- 18 binary operators (6 arithmetic + 6 comparison + 3 logic + 2 bitwise + 1 range)
- 2 unary operators (neg, not)

---

## 9. Test Status

- **83 tests passing, 0 failures** (as of 2026-04-04)
- Test categories:
  - Basic compilation pipeline tests
  - String literal tests (basic, escape, pool dedup, multiple, type checking, IR alloc)
  - Struct tests (init, field access, offset, sema, codegen asm, three fields)
  - Trait/impl tests (parse, typecheck, method call lowering, multiple methods)
  - Ownership tests (move, borrow, mut exclusivity, copy types, fn args, return types)
  - Module tests (use import resolution)
  - Earlier Mk.I tests (arithmetic, if/else, while, functions)

---

## 10. Design Philosophy

### LLVM 0% Dependency
HEXA-IR is completely independent of LLVM. No LLVM libraries, no LLVM IR emission.
This enables:
- Full control over optimization (proof-guided passes impossible in LLVM)
- Minimal binary size (no 100MB+ LLVM dependency)
- n=6 aligned instruction set (24 vs ~1000 opcodes)
- Proof preservation through the entire pipeline

### Proof-Preserving Pipeline
Proof instructions (OwnershipTransfer, BorrowCheck, LifetimeEnd, ProofAssert, ProofInvariant, ProofWitness) flow through the entire pipeline and are only erased at codegen. This allows optimization passes to use proof information for more aggressive elimination.

### n=6 Alignment
Every architectural decision maps to n=6 arithmetic:
- Pipeline stages = n = 6
- Opcodes = J2 = 24 = tau * n
- Opcode categories = tau = 4
- Optimization passes = sigma = 12
- Pass waves = n/phi = 3
- Primitive types = sigma - tau = 8
- Keywords = sigma = 12 (flow) + sigma-tau = 8 (types)

---

## 11. Known Limitations / Bugs

1. **No floating-point codegen**: f64 type exists in IR but ARM64 codegen does not emit FP instructions yet
2. **No heap GC**: Egyptian allocator exists but no automatic garbage collection
3. **No error recovery in parser**: First parse error aborts (no error recovery/continuation)
4. **Single-file compilation only**: No multi-file/crate compilation yet
5. **No standard library**: Only syscall builtins (print, file_*, heap_*)
6. **x86-64 codegen limited**: ARM64 is primary; x86-64 uses .byte directive approach

---

## 12. Evolution Roadmap

### Mk.I (Current) -- Basic Compiler
- 67 .rs files, ~13,288 LOC
- 83 tests passing
- Struct, trait/impl, match, closures, generics, modules
- ARM64 native codegen
- Proof-preserving pipeline

### Mk.II (Next) -- Self-Hosting
- Write HEXA-LANG compiler in HEXA-LANG itself
- Multi-file compilation (mod system fully functional)
- Standard library bootstrap
- Error recovery in parser

### Mk.III -- Full Language
- Async/await
- Algebraic effects
- Dependent types (proof integration)
- Package manager

### Mk.IV -- AI-Native
- "Make this app" natural language compilation
- N6AgentChain: 6-stage agent pipeline
- BT-56 LLM integration for code generation

### Mk.V -- Physical Limit
- Wafer-scale compilation targets
- Quantum computing backend
- Photonic computing backend

---

## 13. NEXUS-6 Co-evolution Rules

HEXA-IR evolves in sync with NEXUS-6:

1. **Lens addition -> Pass addition**: New NEXUS-6 lens -> new optimization pass in opt/
2. **Constant discovery -> n6.rs update**: New n=6 constant -> util/n6.rs auto-update -> all modules reflect
3. **Convergent refinement**: convergent_refinement.py results -> automatic anomaly correction
4. **OUROBOROS cycle**: HEXA-IR included in NEXUS-6 evolution cycles
5. **Growth daemon**: Scans compiler code for improvement opportunities

---

## 14. DSE Results

### Optimal Path (from 7,560 combinations)
- **Foundation**: MetaLang (F2) -- 6 paradigms as DSL
- **Process**: LLVM_Native (P1) -- system-level native (but HEXA-IR replaces LLVM)
- **Core**: Full_N6 (C7) -- all constants n=6 aligned
- **Engine**: N6AgentChain (E2) -- 6-stage agent pipeline
- **System**: FullStack (S4) -- DB+API+UI auto generation

Pareto score: 0.7743, n6 EXACT: 96.0%

---

## 15. Key Commits

| Hash | Description |
|------|-------------|
| 0c00875 | HEXA-LANG initial design + LLVM IR compiler concept |
| f11acf8 | Mk.I complete compiler stack -- 66 .rs, 22 lenses consensus |
| 12488b1 | First Hello World -- return 42 native execution (LLVM 0%) |
| 38237c0 | Mk.I completion -- 6/6 single-function tests + J2=24 codegen |
| 7860b32 | Real optimization passes (12 passes) + scaling bug fix |
| 90a1528 | Mk.III subdomain certification + DSE domain expansion |
| 4f15aaa | All-domain breakthrough -- 12/12 physical limits reached |

---

## 16. File Statistics

- Source files: 67 .rs
- Total LOC: ~13,288
- Total commits: 11
- Tests: 83 pass / 0 fail / 0 ignored
- Dependencies: 0 external crates (pure Rust, no LLVM)
- Binary targets: ARM64 macOS (primary), x86-64 Linux, x86-64 macOS
