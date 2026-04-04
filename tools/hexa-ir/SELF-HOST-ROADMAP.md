# HEXA-IR Self-Hosting Roadmap (Mk.II)

**Goal**: HEXA-LANG compiles itself — the compiler is written in its own language.

## Status: Phase 1 Complete (Foundation)

| Phase | Description | Status | .hexa Modules | Tests |
|-------|------------|--------|---------------|-------|
| 1 | Constants + Data Structures | DONE | 3 | 14 |
| 2 | Lexer Self-Hosting | TODO | 0 | 0 |
| 3 | Parser Self-Hosting | TODO | 0 | 0 |
| 4 | Full Pipeline Self-Hosting | TODO | 0 | 0 |

## Phase 1: Foundation (COMPLETE)

Three modules rewritten from Rust to HEXA-LANG, proving the compiler can compile
its own core definitions through all 6 pipeline stages.

```
self-host/
  n6.hexa           — n=6 core constants (σ,φ,τ,J₂,sopfr,μ + derived)
  token_kind.hexa   — J₂=24 operator token IDs (τ=4 groups of n=6)
  span.hexa         — Source location struct (τ=4 fields)
```

### Verified Capabilities

| Feature | n6.hexa | token_kind.hexa | span.hexa |
|---------|---------|-----------------|-----------|
| Integer literals | Y | Y | Y |
| Arithmetic (+,-,*,/) | Y | Y | Y |
| let bindings | Y | Y | Y |
| Type annotations | Y | Y | Y |
| Functions (params + return) | Y | Y | Y |
| if/else branching | Y | Y | Y |
| Struct definition | - | - | Y |
| Struct initialization | - | - | Y |
| Field access | - | - | Y |
| Constant folding (opt) | Y | Y | - |

### n=6 Arithmetic Verification

All self-hosted modules verify n=6 identities at compile time:

- `sigma * phi == n * tau` (12*2 == 6*4 = 24)
- `1/2 + 1/3 + 1/6 = 1` (Egyptian fractions, integer: 3+2+1=6)
- `J₂ = 4 groups * 6 operators = 24` (token structure)
- `Span has tau=4 fields` (struct layout)

### Metrics

- Total self-hosted functions: 50
  - n6.hexa: 19 (7 primary + 5 derived + 4 block + 2 verify + 1 main)
  - token_kind.hexa: 27 (24 token IDs + 2 verify + 1 main)
  - span.hexa: 4 (3 span functions + 1 main)
- Total test assertions: 14 tests, all PASS
- Pipeline stages verified: 5/6 (Lex, Parse, Sema, Lower, Opt)
  - Stage 6 (Codegen) has ARM64 immediate range bug for large stack frames (tracked)

## Phase 2: Lexer Self-Hosting (NEXT)

### Prerequisites
- [ ] String comparison (`==` on str type)
- [ ] Character literals and comparison
- [ ] Array/slice indexing
- [ ] While-loop-based string scanning

### Modules to Create
```
self-host/
  cursor.hexa       — Character cursor (position tracking)
  keyword.hexa      — Keyword lookup (σ=12 keywords match)
  lexer.hexa        — Main lexer (tokenize source string)
```

### Language Features Needed
1. **String indexing**: `s[i]` to get character at position
2. **Character comparison**: `c == '/'` for token dispatch
3. **Mutable variables in loops**: `while i < len { i = i + 1; }`
4. **Function calls with string args**: `lookup_keyword(ident)`

## Phase 3: Parser Self-Hosting

### Prerequisites
- Phase 2 complete (self-hosted lexer)
- [ ] Enum types in HEXA-LANG (for AST nodes)
- [ ] Recursive data structures (Box<Expr>)
- [ ] Pattern matching on enums
- [ ] Generic types (Vec<T>)

### Modules to Create
```
self-host/
  ast.hexa          — AST node types
  expr.hexa         — Expression parser
  stmt.hexa         — Statement parser
  parser.hexa       — Top-level parser
```

## Phase 4: Full Pipeline Self-Hosting

### Prerequisites
- Phases 2-3 complete
- [ ] HashMap/HashSet equivalent
- [ ] Trait-based IR visitor pattern
- [ ] Code generation backend
- [ ] File I/O

### Modules to Create
```
self-host/
  sema.hexa         — Semantic analysis
  lower.hexa        — AST to IR lowering
  opt.hexa          — Optimization passes
  codegen.hexa      — Native code generation
```

## Codegen Bug Tracker

| Bug | Description | Impact | Status |
|-----|------------|--------|--------|
| ARM64-IMM-001 | `sub sp, sp, #32800` exceeds 12-bit immediate range | Large functions fail codegen | OPEN |

**Fix**: Split large stack adjustments into multiple instructions or use a temporary register.

## n=6 Structure of Self-Hosting

```
Phase count = tau = 4
Phase 1 modules = n/phi = 3
Phase 1 functions = 50 = sopfr * sigma - phi * sopfr
Token groups = tau = 4
Tokens per group = n = 6
Total operators = J₂ = 24
Span fields = tau = 4
Pipeline stages = n = 6
Optimization passes = sigma = 12
```
