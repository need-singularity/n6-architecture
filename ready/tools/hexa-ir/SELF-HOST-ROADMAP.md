# HEXA-IR Self-Hosting Roadmap (Mk.II)

**Goal**: HEXA-LANG compiles itself — the compiler is written in its own language.

## Status: Phase 2 In Progress (Lexer)

| Phase | Description | Status | .hexa Modules | Tests |
|-------|------------|--------|---------------|-------|
| 1 | Constants + Data Structures | DONE | 3 | 14 |
| 2 | Lexer Self-Hosting | IN PROGRESS | 1 | 4 |
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

## Phase 2: Lexer Self-Hosting (IN PROGRESS)

### Prerequisites
- [ ] String comparison (`==` on str type)
- [ ] Character literals and comparison
- [ ] Array/slice indexing
- [ ] While-loop-based string scanning

### Modules Created
```
self-host/
  lexer.hexa        — Unified lexer module (cursor + keyword + tokenizer)
                       All-in-one: 12 core functions (= σ)
```

**Design decision**: Instead of 3 separate files (cursor.hexa, keyword.hexa, lexer.hexa),
the lexer is a single unified module. The cursor logic is inlined (position state passed
as parameters), keyword lookup is a function within the module, and all 12 core lexer
functions live together. This avoids cross-module imports which aren't yet supported.

### Functions (σ=12 core + helpers)

| # | Function | Description |
|---|----------|-------------|
| 1 | `lex(source)` | Main entry: tokenize entire source |
| 2 | `lex_token(source, pos, line, col)` | Lex one token |
| 3 | `lex_number(source, pos, line, col)` | Int/float literals (dec, hex, bin, oct) |
| 4 | `lex_string(source, pos, line, col)` | String literals with escapes |
| 5 | `lex_char(source, pos, line, col)` | Character literals |
| 6 | `lex_ident_or_keyword(source, pos, line, col)` | Identifiers + keyword dispatch |
| 7 | `skip_whitespace(source, pos, line, col)` | Skip spaces/tabs/newlines |
| 8 | `skip_comment(source, pos, line, col)` | // and /* */ comments (nested) |
| 9 | `is_digit(c)` | ASCII digit check |
| 10 | `is_alpha(c)` | ASCII letter or underscore check |
| 11 | `is_alnum(c)` | Alphanumeric or underscore check |
| 12 | `keyword_lookup(s)` | Match 28 keywords (20 core + 8 type) |

### Token Coverage

| Category | Count | IDs |
|----------|-------|-----|
| Operators | J₂=24 | 0..23 (from token_kind.hexa) |
| Delimiters | σ=12 | 24..35 |
| Literals | τ=4 | 36..39 |
| Keywords | 20 | 40..59 |
| Type keywords | σ-τ=8 | 60..67 |
| Ident/EOF/Error | 3 | 68..70 |
| DotDotEq (..=) | 1 | 71 |

### Handles
- All J₂=24 operators including multi-char: `==`, `!=`, `<=`, `>=`, `&&`, `||`, `..`, `..=`, `->`, `=>`, `::`
- Integer literals: decimal, 0x hex, 0b binary, 0o octal, with `_` separators
- Float literals: decimal point + optional `e`/`E` exponent with sign
- String literals: `\n`, `\t`, `\\`, `\"`, `\0` escape sequences
- Character literals: `'a'`, `'\n'`, `'\0'`, etc.
- Comments: `//` line comments and `/* */` nested block comments
- Proper span tracking (line, col) through all constructs

### Verification (τ=4 tests)
1. Lexer structure: σ=12 functions, J₂=24 ops, σ=12 delims
2. Operator groups: τ=4 groups of n=6
3. Keyword lookup: correct kind mapping
4. Character classification: digit/alpha/alnum predicates

### Language Features Used
1. **String builtins**: `str_len()`, `char_at()`, `str_slice()`, `str_append_char()`
2. **Character comparison**: integer-based (char codes)
3. **Mutable variables in loops**: `while p < len { p = p + 1; }`
4. **Structs for multiple returns**: `PosState`, `CommentResult`, `LexResult`
5. **Vec<Token>**: `vec_new()`, `vec_push()`

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
