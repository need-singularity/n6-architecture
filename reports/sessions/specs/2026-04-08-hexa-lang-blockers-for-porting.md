# hexa-lang missing features — n6-architecture porting blockers

Authored: 2026-04-08
Status: **porting on hold** (awaiting hexa-lang growth)
Related spec: `2026-04-08-py-rs-sh-to-hexa-porting-design.md`
Related plan: `docs/superpowers/plans/2026-04-08-py-rs-sh-to-hexa-porting.md`

## Decision

Full porting of n6-architecture `.py` / `.rs` / `.sh` -> `.hexa` **cannot be carried out with current hexa-lang Phase 5 capability**. On hold until the blockers below are resolved.

## Verified toolchain (Pilot-A passed)

- Build cwd: `$HEXA_LANG` (required)
- Command: `./ready/self/hexa_bootstrap <src.hexa> -o <out>`
- Artifact: `<out>.c` + native binary
- Passing features: `let`, integer arithmetic, string `+`, `to_string()`, **single-arg** `println()`

## Blockers

### B-1. Multi-argument `println` (RESOLVED 2026-04-08)

**Status:** works via string concat (`"a=" + to_string(x)`) form (Pilot-B G3 passed)
**Remaining:** multi `{}` substitution split out to B-3a

### B-2. CLI argv (RESOLVED 2026-04-08)

**Status:** `args()` works (Pilot-beta: argv[0] inclusive OK)

### B-3. Partial format spec support (re-checked 2026-04-08)

**Current status (Pilot-beta measured):**
- `format("{}", x)` single substitution works
- **B-3a:** `format("{} {}", a, b)` multiple substitution — only the first is substituted, rest literal
- **B-3b:** `{:<8}` `{:>2}` `{:.4}` width/precision/alignment specifiers — all printed literally
- **B-3c:** `to_string(float)` precision control — default display only (`0.00279841` etc.)

**Impact:** L2 calculator table output cannot be auto-ported; **manual padding workaround** is the only path (demonstrated in Pilot-B)
**Need:** Rust `{:<width}` `{:>width.prec}` equivalent, or helpers (`pad_left`, `pad_right`, `to_string_prec`)

### B-4. Floating-point math functions (PARTIAL 2026-04-08)

**Confirmed:** `pow`, `sqrt`, `floor` work (Pilot-beta)
**Unconfirmed:** `sin`, `cos`, `tan`, `ln`, `log10`, `exp`, `atan2`, `ceil`, `abs`
**Need:** remaining libm function exposure confirmation

### B-5. Unicode literals (RESOLVED 2026-04-08)

**Status:** BMP characters like `== check x -> _6 _8 tau sigma` work (Pilot-B G3 byte-perfect)
**Unconfirmed:** Korean, 4-byte UTF-8 (emoji etc.) — separate verification needed

### B-6. for/range loops and destructuring

**Symptom:** Rust-style destructuring like `for (name, rank, val, expr) in &gut_groups` unconfirmed
**Impact:** many L2 calculators
**Need:** for-in, range, tuple destructuring

### B-7. External command invocation (shell replacement)

**Symptom:** `runtime.c` has `exec()` but stdout capture / exit code / env vars / pipe unconfirmed
**Impact:** all L1 `.sh` (calls like git/find/jq are essential)
**Need:** `Command::new` or `exec_capture(cmd) -> {stdout, stderr, code}` API

### B-8. File system / directory traversal

**Symptom:** `read_file`, `write_file` exist, but `glob`, `read_dir`, `exists`, `mkdir -p` unconfirmed
**Impact:** many L1 `.sh` (`find ... -name "*.json"`)
**Need:** `fs` module

### B-9. JSON parsing

**Impact:** L1 `.sh` uses jq to read nexus/shared/*.json. The `.hexa` port needs the same behavior
**Need:** `json::parse(str) -> Value` or stable pattern for external jq calls

### B-10. ML numeric stack (blocks L3/L4)

**Symptom:** numpy/torch/scipy/matplotlib equivalents — none
**Impact:** 23 L3 `techniques/*.py`, all L4 `experiments/*.py`
**Need:** separate decision — (a) write a new HEXA ML stack / (b) Python FFI / (c) permanently defer L3/L4 porting

### B-12. Array literal codegen (RESOLVED 2026-04-08)

**Status:** Pilot-B retry confirmed `["a","b","c","d"]` works
*Previous symptom (preserved):*

**Symptom:** `let names = ["SU(5)", "SO(10)", "E_6", "E_8"]` produced C:
```c
HexaVal vals = hexa_array_new(), tau), sopfr), n), hexa_int(...));
```
Invalid C with mismatched parens. Not expanded as a push chain.
**Impact:** arrays with more than one element impossible -> almost all non-trivial programs impossible
**Need:** in codegen_c2, expand array literal into `hexa_array_push(hexa_array_push(... hexa_array_new() ...))` properly

### B-13. Outer-scope visibility inside while/for body (RESOLVED 2026-04-08)

**Status:** Pilot-B retry confirmed `while { ... names[i] ... matches = matches + 1 ... }` works
*Previous symptom (preserved):*

**Symptom:** after top-level `let names = ...`, inside `while i < 4 { ... names[i] ... }`:
```
error: use of undeclared identifier 'names'
error: use of undeclared identifier 'ranks'
```
**Impact:** outer variables not usable inside loops -> practically all non-trivial programs impossible
**Need:** codegen_c2 block-scope handling — the loop body's C block must see outer identifiers

### B-14. Embedded variable substitution (HIGH IMPACT)

**Symptom:** no equivalent to Rust `println!("  J2(6) = sigma*phi = n*tau = {j2}");` where an identifier is substituted inside braces
**Impact:** nearly every println in L2 calc is this shape. String concat is possible but hurts readability/accuracy
**Need:** `format!("text {var}")` or `println!("text {var}")` identifier-embed form

### B-15. Tuple arrays + destructuring for-in (HIGH IMPACT)

**Symptom:** Rust
```rust
let reps = [("5_bar", 5, sopfr, "sopfr(6)"), ("10", 10, sigma-phi, "sigma-phi"), ...];
for (name, dim, val, expr) in &reps { ... }
```
no equivalent. Parallel arrays (N arrays) workaround possible but increases calc line count 4-6x
**Impact:** L2 calc PART 3/4/5/6 table-output patterns (gut-calc alone has 4)
**Need:** tuple literals + for-in destructuring

### B-16. if-as-expression in argument position

**Symptom:** `let mark = if matched { "OK" } else { "NG" };` or `println!("{}", if x { ... } else { ... })` unconfirmed
**Impact:** many L2 calcs
**Need:** if expression returns a value

### B-17. `mut` variables + compound assignment (`+=` `-=` etc.)

**Symptom:** `let mut total = 0; total += count;` unconfirmed (workaround `matches = matches + 1` works)
**Impact:** code using counters / accumulators
**Need:** `+=` `-=` `*=` tokens or stable explicit reassignment

### B-18. Method-call syntax (`.len()`, `.iter()` etc.)

**Symptom:** dot-method calls like `checks.len()` unconfirmed (workaround `len(checks)` functional form works)
**Impact:** common Rust patterns
**Need:** dot-method or a unified functional-form policy

### B-19. Float division compiled as integer division (CRITICAL)

**Symptom:** `let z = 12.0 / 4.0; println(z)` -> `1` (integer division result). Should be `3.0`.
**Verification (Pilot 2nd-file probe):**
```
F1: 12.0 / 4.0 = 1
F5: pct(102.0, 100.0) = 0     // (102.0 - 100.0) / 100.0 * 100.0
```
**Impact:** every physics / engineering calc (energy-calc, fusion-calc, optics-calc, chip-power-calc, consciousness-calc, semiconductor-calc etc.) — basically all except the integer-only gut-calc
**Need:** split float literal correctly so the float `/` operator uses HexaVal float division

### B-20. Float `abs()` runtime tag mismatch (CRITICAL)

**Symptom:** `abs(-3.5)` -> `4615063718147915776` (float bit pattern interpreted as int)
**Verification:** `abs(-3.5) = 4615063718147915776` — IEEE 754 bit pattern of `-3.5` viewed as int64
**Impact:** every calc using `check()` helpers like `(measured - predicted).abs()`
**Need:** runtime `hexa_abs` branches on HexaTag (TAG_FLOAT -> fabs, TAG_INT -> llabs)

### B-21. `String.repeat(n)` unsupported

**Symptom:** `"=".repeat(20)` -> C code emits only `/* Call */` placeholder, build fails
**Impact:** many L2 calcs (`section()` helper for divider output)
**Need:** in codegen, `.repeat(n)` method -> `hexa_str_repeat(s, n)` call

### B-22. Bool `to_string` outputs `0/1` (minor)

**Symptom:** `println("a && b =", false)` -> `a && b = 0`. Rust prints `false`/`true`.
**Impact:** affects G3 byte-perfect (many Rust calcs output bool via `{:?}` or `{}`)
**Need:** `to_string` on TAG_BOOL -> `"true"`/`"false"`

### B-28. fn body cannot see top-level `let` constants

**Symptom:** module-level `let SIGMA = 12.0` is undeclared inside `fn solar() { ... SIGMA ... }`
**Impact:** every calc written in the Rust `const` pattern — cannot split into fns
**Workaround:** flatten (no fn) or redeclare constants inside fn
**Need:** module-scope identifiers visible in fn body

### B-29. `format_float` scientific notation unsupported

**Symptom:** no equivalent to Rust `{:.4e}`. `format_float(2.87e-21, 4)` -> `0.0000` (only 4 decimal places)
**Impact:** physics-constant output (Boltzmann, Planck, Avogadro etc.)
**Workaround:** manually decompose mantissa (normalize by `landauer * 1e21` and display)
**Need:** `format_float_sci(f, prec)` or `{:.4e}` equivalent

### B-23. Scientific-notation literal (`1.38e-23`) (RESOLVED 2026-04-08)

**Symptom:** lexer split `1.38e` + `-23` -> C code had an undeclared `e` identifier
**Impact:** Boltzmann/Planck physics constants, energy-calc/optics-calc/photonic-energy-calc
**Need:** lexer recognizes float exponent (`[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?`)

### B-24. `ln(x)` natural logarithm

**Symptom:** undefined function, implicit declaration in C
**Impact:** Landauer `kT*ln(2)`, Shannon entropy etc.
**Need:** runtime `hexa_ln` (libm `log`)

### B-25. `log10(x)` name collision

**Symptom:** undefined in hexa runtime -> C `log10` called with HexaVal struct and fails to compile
**Impact:** dB calc, exponent estimation
**Need:** define runtime `hexa_log10` (wraps libm `log10`)

### B-26. `array.contains(item)` always `false`

**Symptom:** `["--solar","--all"].contains("--all")` -> `false`
**Impact:** CLI-flag checks like `args.contains("--solar")` — energy-calc and nearly every other calc
**Need:** `array.contains` method performs real comparison and returns bool

### B-27. Float precision formatting (`{:>10.4}`, `{:>6.2}%`, `{:.4e}`)

**Symptom:** pad_right/pad_left are string padding only, no float digit control. `to_string(0.337)` is just `"0.337"` (cannot force 2/4 decimal places)
**Impact:** almost all L2 calc table output — no G3 byte-perfect automation
**Need:** `format_float(f, prec)` or `{:.4}` equivalent format helper

### B-11. Build artifact cleanup / output path

**Symptom:** `.c` intermediate file remains next to source. `-I` flag is hardcoded to `ready/self` relative path -> depends on cwd
**Impact:** during a full sweep, build directory is polluted; build fails from non-hexa-lang cwd
**Need:** `--include-dir` option or absolute path / `--temp-dir` separation

## Priority

| Priority | Blocker | Unlocks |
|---|---|---|
| **P-1** | ~~B-12, B-13~~ resolved | — |
| **P0** | ~~B-1, B-2~~ resolved, B-11 | basic sweep entry |
| **P0** | B-3a/b/c, B-4(partial), **B-14, B-15** | efficient L2 sweep entry |
| P1 | B-16, B-17, B-18 | L2 direct-translation quality |
| P1 | B-5, B-6 | L2 quality maintenance |
| P1 | B-7, B-8, B-9 | L1 sweep feasible |
| P2 | B-10 | L3/L4 — separate decision |

## Resume conditions

- **L2 sweep resumes:** 5 P0s resolved (B-1, B-2, B-3, B-4, B-11)
- **L1 sweep resumes:** P0 + P1 3 items resolved (B-7, B-8, B-9)
- **L3/L4 matter:** requires a separate user decision on B-10

## Interim measures

1. CLAUDE.md R1 (HEXA-FIRST) operates only as a **new-code `.hexa` enforcement** rule. Existing assets frozen.
2. This worktree (`$N6_ARCH-porting`, branch `porting/pilot-2026-04-08`) preserved until resumption. Pilot artifacts `.porting-pilot/hello.{hexa,c,exe}` included.
3. Forward this document to the hexa-lang side as a link or issue (user decision).
