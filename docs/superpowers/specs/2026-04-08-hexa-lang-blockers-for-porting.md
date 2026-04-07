# hexa-lang 누락 기능 — n6-architecture 포팅 차단 항목

작성일: 2026-04-08
상태: **포팅 보류** (hexa-lang 성장 대기)
관련 스펙: `2026-04-08-py-rs-sh-to-hexa-porting-design.md`
관련 계획: `docs/superpowers/plans/2026-04-08-py-rs-sh-to-hexa-porting.md`

## 결정

n6-architecture의 `.py` / `.rs` / `.sh` → `.hexa` 전면 포팅은 **현재 hexa-lang Phase 5 능력으로 수행 불가**. 아래 차단 항목이 해소될 때까지 보류한다.

## 검증된 도구체인 (Pilot-A 통과)

- 빌드 cwd: `~/Dev/hexa-lang` (필수)
- 명령: `./ready/self/hexa_bootstrap <src.hexa> -o <out>`
- 산출물: `<out>.c` + native binary
- 통과 기능: `let`, 정수 산술, 문자열 `+`, `to_string()`, **단일인수** `println()`

## 차단 항목 (Blockers)

### B-1. 다중 인수 `println` (RESOLVED 2026-04-08)

**상태:** ✅ string concat (`"a=" + to_string(x)`) 형태로 동작 확인 (Pilot-B G3 통과)
**잔여:** 다중 `{}` 치환은 B-3a로 분리

### B-2. CLI argv (RESOLVED 2026-04-08)

**상태:** ✅ `args()` 동작 확인 (Pilot-β: argv[0] 포함 정상)

### B-3. 포맷 스펙 부분 지원 (2026-04-08 재검증)

**현재 상태 (Pilot-β 측정):**
- ✅ `format("{}", x)` 단일 치환 동작
- ❌ **B-3a:** `format("{} {}", a, b)` 다중 치환 — 첫 번째만 치환, 나머지 리터럴
- ❌ **B-3b:** `{:<8}` `{:>2}` `{:.4}` 등 폭/정밀도/정렬 지정자 — 전부 리터럴 출력
- ❌ **B-3c:** `to_string(float)` 정밀도 제어 — 기본 표시만 (`0.00279841` 등)

**영향:** L2 계산기의 표 출력이 자동 포팅 불가, **수동 패딩 워크어라운드**로만 가능 (Pilot-B에서 입증)
**필요:** Rust `{:<width}` `{:>width.prec}` 등가 또는 헬퍼 (`pad_left`, `pad_right`, `to_string_prec`)

### B-4. 부동소수 수학 함수 (PARTIAL 2026-04-08)

**확인:** ✅ `pow`, `sqrt`, `floor` 동작 (Pilot-β)
**미확인:** `sin`, `cos`, `tan`, `ln`, `log10`, `exp`, `atan2`, `ceil`, `abs`
**필요:** 나머지 libm 함수 노출 확인

### B-5. 유니코드 리터럴 (RESOLVED 2026-04-08)

**상태:** ✅ `═ ✓ ✗ → ₆ ₈ τ σ` 등 BMP 문자 동작 확인 (Pilot-B G3 byte-perfect)
**미확인:** 한글, 4-byte UTF-8 (이모지 등) — 별도 검증 필요

### B-6. for/range 루프와 destructuring

**증상:** Rust `for (name, rank, val, expr) in &gut_groups` 같은 구조분해 미확인
**영향:** L2 계산기 다수
**필요:** for-in, range, tuple destructuring

### B-7. 외부 명령 호출 (shell 대체용)

**증상:** `runtime.c`에 `exec()`은 있으나 stdout 캡처 / 종료코드 / 환경변수 / pipe 미확인
**영향:** L1 .sh 전부 (git/find/jq 호출이 본질)
**필요:** `Command::new` 또는 `exec_capture(cmd) → {stdout, stderr, code}` API

### B-8. 파일 시스템 / 디렉토리 순회

**증상:** `read_file`, `write_file`은 있으나 `glob`, `read_dir`, `exists`, `mkdir -p` 미확인
**영향:** L1 .sh 다수 (`find ... -name "*.json"`)
**필요:** `fs` 모듈

### B-9. JSON 파싱

**영향:** L1 .sh가 jq로 nexus/shared/*.json을 읽음. .hexa 포팅본도 동일 동작 필요
**필요:** `json::parse(str) → Value` 또는 외부 jq 호출 패턴 안정화

### B-10. ML 수치 스택 (L3/L4 차단)

**증상:** numpy/torch/scipy/matplotlib 상응 0
**영향:** L3 `techniques/*.py` 23개, L4 `experiments/*.py` 전부
**필요:** 별도 결정 — (a) HEXA용 ML 스택 신규 작성 / (b) Python FFI / (c) L3/L4 포팅 영구 보류

### B-12. 배열 리터럴 codegen (RESOLVED 2026-04-08)

**상태:** ✅ Pilot-B 재시도에서 `["a","b","c","d"]` 정상 동작
*이전 증상 (보존):*

**증상:** `let names = ["SU(5)", "SO(10)", "E_6", "E_8"]`에서 생성된 C:
```c
HexaVal vals = hexa_array_new(), tau), sopfr), n), hexa_int(...));
```
괄호 짝 깨진 무효 C. push 체인으로 펼쳐지지 않음.
**영향:** 단일원소 외 배열 리터럴 전부 불가 → 거의 모든 비자명 프로그램 불가
**필요:** codegen_c2에서 배열 리터럴을 `hexa_array_push(hexa_array_push(... hexa_array_new() ...))` 형태로 제대로 펼치기

### B-13. while/for 본문 외부 스코프 (RESOLVED 2026-04-08)

**상태:** ✅ Pilot-B 재시도에서 `while { ... names[i] ... matches = matches + 1 ... }` 동작
*이전 증상 (보존):*

**증상:** top-level `let names = ...` 후 `while i < 4 { ... names[i] ... }`에서
```
error: use of undeclared identifier 'names'
error: use of undeclared identifier 'ranks'
```
**영향:** 루프 안에서 외부 변수 사용 불가 → 사실상 모든 비자명 프로그램 불가능
**필요:** codegen_c2의 블록 스코프 처리 — 루프 본문 C 블록이 외부 식별자를 볼 수 있게 emit

### B-14. 임베디드 변수 치환 (HIGH IMPACT)

**증상:** Rust `println!("  J₂(6) = σ·φ = n·τ = {j2}");` 같은 brace 안 식별자 직접 치환 등가 부재
**영향:** L2 calc의 거의 모든 println이 이 형태. 우회는 string concat으로 가능하나 가독성/정확도 모두 손해
**필요:** `format!("text {var}")` 또는 `println!("text {var}")` 처럼 식별자 직접 임베드

### B-15. 튜플 배열 + 구조분해 for-in (HIGH IMPACT)

**증상:** Rust
```rust
let reps = [("5̄", 5, sopfr, "sopfr(6)"), ("10", 10, sigma-phi, "σ-φ"), ...];
for (name, dim, val, expr) in &reps { ... }
```
등가 미확인. parallel array(N개)로 우회 가능하나 calc 1개당 4~6배 줄수 증가
**영향:** L2 calc PART 3/4/5/6 같은 표 출력 패턴 (gut-calc만 4회)
**필요:** 튜플 리터럴 + for-in 구조분해

### B-16. if-as-expression in argument position

**증상:** `let mark = if matched { "✓" } else { "✗" };` 또는 `println!("{}", if x { ... } else { ... })` 미확인
**영향:** L2 calc 다수
**필요:** if 식이 값 반환

### B-17. `mut` 변수 + 복합 대입 (`+=` `-=` 등)

**증상:** `let mut total = 0; total += count;` 미확인 (`matches = matches + 1`로 우회는 가능)
**영향:** 카운터·누산기 사용 코드
**필요:** `+=` `-=` `*=` 토큰 또는 명시적 reassignment 안정성

### B-18. 메서드 호출 syntax (`.len()`, `.iter()` 등)

**증상:** `checks.len()` 같은 dot-method 호출 미확인 (`len(checks)` 함수형으로 우회 가능)
**영향:** Rust에서 흔한 패턴
**필요:** dot-method 또는 함수형으로 통일 정책

### B-19. Float 나눗셈이 정수 나눗셈으로 변환 (CRITICAL)

**증상:** `let z = 12.0 / 4.0; println(z)` → `1` (정수 나눗셈 결과). 마땅히 `3.0`.
**검증 (Pilot 2nd-file probe):**
```
F1: 12.0 / 4.0 = 1
F5: pct(102.0, 100.0) = 0     // (102.0 - 100.0) / 100.0 * 100.0
```
**영향:** 모든 물리/엔지니어링 calc (energy-calc, fusion-calc, optics-calc, chip-power-calc, consciousness-calc, semiconductor-calc 등) — 정수만 다루는 gut-calc 외 거의 전부
**필요:** float literal 정확히 분기, float `/` 연산자가 HexaVal float division 사용

### B-20. Float `abs()` runtime tag mismatch (CRITICAL)

**증상:** `abs(-3.5)` → `4615063718147915776` (float bit pattern을 int로 해석한 값)
**검증:** `abs(-3.5) = 4615063718147915776` ← `-3.5`의 IEEE 754 비트를 int64로 본 값
**영향:** `check()` 류 헬퍼(`(measured - predicted).abs()`)를 사용하는 모든 calc
**필요:** runtime `hexa_abs`가 HexaTag 분기 (TAG_FLOAT면 fabs, TAG_INT면 llabs)

### B-21. `String.repeat(n)` 미지원

**증상:** `"=".repeat(20)` → C 코드에 `/* Call */` 빈 자리만 emit, 컴파일 실패
**영향:** L2 calc 다수 (`section()` 헬퍼에서 구분선 출력)
**필요:** codegen에서 `.repeat(n)` 메서드 → `hexa_str_repeat(s, n)` 호출

### B-22. Bool `to_string` 출력이 `0/1` (minor)

**증상:** `println("a && b =", false)` → `a && b = 0`. Rust는 `false`/`true` 문자열.
**영향:** G3 byte-perfect에 영향 (Rust calc 다수가 `{:?}` 또는 `{}` 로 bool 출력)
**필요:** `to_string`에서 TAG_BOOL → `"true"`/`"false"`

### B-11. 빌드 산출물 cleanup / 출력 경로

**증상:** `.c` 중간 파일이 소스 옆에 남음. `-I` 플래그가 `ready/self` 상대경로로 하드코딩 → cwd 의존
**영향:** 본격 sweep 시 빌드 디렉토리 오염, 비-hexa-lang cwd에서 빌드 불가
**필요:** `--include-dir` 옵션 또는 절대경로 / `--temp-dir` 분리

## 우선순위

| 우선 | 차단 항목 | 해소되면 가능해지는 것 |
|---|---|---|
| **P-1** | ~~B-12, B-13~~ ✅해소 | — |
| **P0** | ~~B-1, B-2~~ ✅해소, B-11 | 기본 sweep 진입 |
| **P0** | B-3a/b/c, B-4(부분), **B-14, B-15** | L2 sweep 효율적 진입 |
| P1 | B-16, B-17, B-18 | L2 직역 품질 |
| P1 | B-5, B-6 | L2 품질 유지 |
| P1 | B-7, B-8, B-9 | L1 sweep 가능 |
| P2 | B-10 | L3/L4 — 별도 결정 사안 |

## 재개 조건

- **L2 sweep 재개:** P0 5건(B-1, B-2, B-3, B-4, B-11) 해소
- **L1 sweep 재개:** P0 + P1 3건(B-7, B-8, B-9) 해소
- **L3/L4 사안:** B-10에 대한 별도 사용자 결정 필요

## 잠정 조치

1. CLAUDE.md R1(HEXA-FIRST)은 **신규 코드 .hexa 강제** 규칙으로만 운영. 기존 자산은 동결.
2. 이 worktree(`/Users/ghost/Dev/n6-architecture-porting`, 브랜치 `porting/pilot-2026-04-08`)는 재개 시까지 보존. 파일럿 산출물 `.porting-pilot/hello.{hexa,c,exe}` 포함.
3. hexa-lang 측에 본 문서 링크 또는 issue로 전달 (사용자 결정).
