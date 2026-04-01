# N6 Programming Language Hypotheses -- Independent Verification

**Date:** 2026-03-30
**Method:** Each hypothesis checked against real-world facts, standard references, and known counts.
**Grading:** EXACT (number matches precisely), CLOSE (off by 1-2 or defensible variant), WEAK (requires cherry-picking or subjective categorization), FAIL (factually wrong), UNVERIFIABLE (no objective count exists).

---

## Overall Honesty Note on Cherry-Picking

This document is blunt about a structural problem: the N6 framework has ~7 arithmetic constants (n=6, sigma=12, tau=4, phi=2, sopfr=5, J2=24, mu=1) plus derived values (sigma-tau=8, J2-mu=23, tau-1=3). With that many numbers to choose from (2, 3, 4, 5, 6, 8, 12, 23, 24), and with the freedom to count things in flexible ways (e.g., "operations not terms," "categories not individual items," "excluding CONNECT"), hitting a match is far more likely than it appears. Many of these hypotheses work backwards from a known count to find a matching arithmetic expression rather than predicting the count from first principles.

---

## Summary Scorecard

| ID | Hypothesis | Grade | Notes |
|----|-----------|-------|-------|
| H-PL-1 | Primitive types = 8 | **FAIL** | C has far more than 8; list includes non-primitives |
| H-PL-2 | Type categories = 4 | **WEAK** | Defensible but arbitrary taxonomy |
| H-PL-3 | OOP pillars = 4 | **CLOSE** | Common but not universal (some say 3) |
| H-PL-4 | Paradigms = 6 | **WEAK** | No authoritative count; ranges from 4 to 34 |
| H-PL-5 | Indentation = 4 | **EXACT** | PEP 8 = 4 spaces |
| H-PL-6 | Operator categories = 12 | **WEAK** | Custom grouping to hit 12; standard says 15 precedence levels |
| H-PL-7 | Error handling = 2 | **EXACT (TRIVIAL)** | Binary by definition; not informative |
| H-PL-8 | SOLID = 5 | **EXACT** | Unambiguous match |
| H-PL-9 | GoF patterns = 23 | **EXACT** | Strongest match; sub-counts also align |
| H-PL-10 | HTTP methods = 8 | **CLOSE** | 9 in spec; 8 only by excluding CONNECT |
| H-PL-11 | REST constraints = 6 | **EXACT** | Fielding's dissertation = 6 |
| H-PL-12 | Lambda calculus ops = 2 | **FAIL** | 3 syntactic forms, not 2 |
| H-PL-13 | Memory model = Egyptian | **UNVERIFIABLE** | No empirical support for claimed ratios |
| H-PL-14 | GC generations = 3 | **CLOSE** | Some GCs use 3, others use 2; debatable |
| H-PL-15 | Concurrency primitives = 2 | **WEAK** | Mutex is a special case of semaphore |
| H-PL-16 | Language generations = 5 | **EXACT** | Standard CS taxonomy |
| H-PL-17 | Compilation stages = 4 | **CLOSE** | Coarse grouping of what is classically 6 phases |
| H-PL-18 | Boolean values = 2 | **EXACT (TRIVIAL)** | True by definition |
| H-PL-19 | Architecture layers = 6 | **WEAK** | No standard mandates 6 layers |
| H-PL-20 | SemVer components = 3 | **EXACT** | semver.org standard |
| H-PL-21 | Scope levels = 4 | **CLOSE** | Python = 4; not universal |
| H-PL-22 | Access modifiers = 4 | **CLOSE** | Java = 4; C++ = 3 |
| H-PL-23 | FP core functions = 3 | **EXACT** | map/filter/reduce is canonical |
| H-PL-24 | Test pyramid = 3 | **EXACT** | Cohn's model |

### Score Distribution

| Grade | Count | Percentage |
|-------|-------|-----------|
| EXACT | 8 | 33% |
| EXACT (TRIVIAL) | 2 | 8% |
| CLOSE | 6 | 25% |
| WEAK | 5 | 21% |
| FAIL | 2 | 8% |
| UNVERIFIABLE | 1 | 4% |

---

## Detailed Verdicts

---

### H-PL-1: C Primitive Types = 8 -- FAIL

**Claim:** sigma(6) - tau(6) = 8 primitive types in C.
**Reality:** C11 defines far more than 8 primitive types if you count unsigned variants: `char`, `signed char`, `unsigned char`, `short`, `unsigned short`, `int`, `unsigned int`, `long`, `unsigned long`, `long long`, `unsigned long long`, `float`, `double`, `long double`, `_Bool`, `void` -- that is 16+ types. The hypothesis's list (`char, short, int, long, float, double, void, pointer`) is problematic: `pointer` is not a primitive type in C; it is a type constructor. The list also omits `long long`, `_Bool`, `long double`, and all unsigned variants.

The Rust claim ("8 core primitives: i32/i64/f32/f64/bool/char/usize/()") is equally contrived -- Rust has bool, char, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, isize, usize, (), str, and the never type `!` -- at least 18 primitive types.

**Verdict:** The count 8 requires aggressive cherry-picking and non-standard grouping. **FAIL.**

---

### H-PL-2: Type System Categories = 4 -- WEAK

**Claim:** tau(6) = 4 type categories (primitive/composite/reference/function).
**Reality:** No universally accepted taxonomy of "type categories" exists. Pierce's "Types and Programming Languages" does not use a 4-category system. Where do sum types go? Generic types? The proposed {primitive, composite, reference, function} is one reasonable decomposition among many. You could equally argue for 3 (value, reference, function) or 5+ (adding generic/parametric).

**Verdict:** No standard to verify against. **WEAK.**

---

### H-PL-3: OOP Pillars = 4 -- CLOSE

**Claim:** tau(6) = 4 OOP pillars.
**Reality:** "Four pillars of OOP" (encapsulation, inheritance, polymorphism, abstraction) is the most commonly taught formulation, especially in Java-centric education. However, many authoritative sources list only 3 (encapsulation, inheritance, polymorphism), treating "abstraction" as too vague or as subsumed by the others. Alan Kay, who coined "object-oriented," emphasized messaging, not these four. The count of 4 is a pedagogical convention, not a formal definition.

**Verdict:** Most popular teaching version = 4, but 3 is also widely cited. **CLOSE.**

---

### H-PL-4: Major Programming Paradigms = 6 -- WEAK

**Claim:** n = 6 major programming paradigms.
**Reality:** Peter Van Roy (2009) identifies 34 paradigms. Wikipedia lists far more than 6. A minimal list gives 4 (imperative, OOP, functional, declarative). The hypothesis's list conflates a paradigm ("functional") with a language characteristic ("scripting/dynamic") and treats "concurrent" as a paradigm when most sources consider it a cross-cutting concern. No authoritative source fixes the count at exactly 6.

**Verdict:** 6 is one possible count among many. **WEAK.**

---

### H-PL-5: Python Standard Indentation = tau = 4 Spaces -- EXACT

**Claim:** tau(6) = 4 = PEP 8 standard indentation.
**Reality:** PEP 8 specifies 4 spaces. Google, Microsoft, and most major style guides agree. Unambiguous fact.

**Verdict:** **EXACT.** (Though 4 is also just a common, human-friendly number. No language designer consulted number theory to set this.)

---

### H-PL-6: C Operator Categories = 12 -- WEAK

**Claim:** sigma(6) = 12 operator categories in C.
**Reality:** C11 Section 6.5 defines 15 levels of operator precedence. The hypothesis admits this ("C의 operator precedence level은 15") then manually groups them into 12 "semantically independent categories." This grouping is custom-built: combining `[]` and `()` into one category, separating `sizeof` into its own category, etc. Different textbooks categorize differently.

**Verdict:** Standard count is 15 precedence levels. 12 requires reverse-engineered grouping. **WEAK.**

---

### H-PL-7: Error Handling = phi = 2 Outcomes -- EXACT (TRIVIAL)

**Claim:** phi(6) = 2 error handling outcomes.
**Reality:** Success/failure is indeed binary. Rust's `Result<T, E>`, Go's `(value, error)`, try/catch all reflect 2 paths. The match is real.

**However:** phi(6)=2 trivially matches ANY binary concept. Boolean logic, coin flips, yes/no questions, on/off switches -- all are "phi(6)=2." Claiming this derives from Euler's totient function adds zero explanatory power.

**Verdict:** **EXACT (TRIVIAL).** True but vacuous.

---

### H-PL-8: SOLID Principles = sopfr = 5 -- EXACT

**Claim:** sopfr(6) = 2+3 = 5 = SOLID principles.
**Reality:** SOLID has exactly 5 principles (Robert C. Martin). Unambiguous, universally agreed count.

**Verdict:** **EXACT.** The "2+3 split" narrative (class-level vs. relationship-level) is post-hoc.

---

### H-PL-9: GoF Design Patterns = J2 - mu = 23 -- EXACT

**Claim:** J_2(6) - mu(6) = 24 - 1 = 23 GoF patterns.
**Reality:** The Gang of Four book (1994) defines exactly 23 design patterns:
- Creational: 5 (Abstract Factory, Builder, Factory Method, Prototype, Singleton)
- Structural: 7 (Adapter, Bridge, Composite, Decorator, Facade, Flyweight, Proxy)
- Behavioral: 11 (Chain of Responsibility, Command, Interpreter, Iterator, Mediator, Memento, Observer, State, Strategy, Template Method, Visitor)

The sub-counts (5+7+11) also match the hypothesis's derived values (sopfr=5, sigma-sopfr=7, J_2-sigma-1=11).

**Verdict:** **EXACT.** This is the strongest match in the entire document. 23 is an unusual number, the count is unambiguous, and the sub-category breakdown also aligns. Still a coincidence -- the GoF authors did not use number theory, and other pattern catalogs have different counts.

---

### H-PL-10: HTTP Methods = sigma - tau = 8 -- CLOSE

**Claim:** sigma(6) - tau(6) = 8 HTTP methods.
**Reality:** RFC 7231 + RFC 5789 define **9 standard methods**: GET, HEAD, POST, PUT, DELETE, CONNECT, OPTIONS, TRACE, PATCH. The hypothesis gets to 8 by excluding CONNECT as "proxy tunneling, not REST-relevant." But the claim is about HTTP methods, not REST methods, and CONNECT is a fully standard HTTP method.

**Verdict:** 9 in the spec, not 8. **CLOSE.**

---

### H-PL-11: REST Constraints = n = 6 -- EXACT

**Claim:** n = 6 = REST architectural constraints.
**Reality:** Roy Fielding's 2000 dissertation, Chapter 5, defines exactly 6 constraints: Client-Server, Stateless, Cacheable, Layered System, Code on Demand (optional), Uniform Interface. This is unambiguous from the primary source.

**Verdict:** **EXACT.**

---

### H-PL-12: Lambda Calculus Operations = phi = 2 -- FAIL

**Claim:** phi(6) = 2 lambda calculus operations.
**Reality:** Lambda calculus has **3 syntactic forms** (terms): variable, abstraction, application. The standard grammar is: `M ::= x | (lambda x. M) | (M M)` -- three production rules. The hypothesis acknowledges "3가지 항" (3 terms) but excludes variable as "just a name, not an operation." This is a non-standard redefinition. In every formal treatment (Church 1936, Barendregt 1984, Pierce 2002), all three are term constructors on equal footing.

The hypothesis also claims SKI combinator calculus has "2 base combinators (S and K)," which is true, but that is a different formal system, not lambda calculus.

**Verdict:** The standard count is 3 terms. **FAIL.**

---

### H-PL-13: Memory Model = Egyptian Fraction -- UNVERIFIABLE

**Claim:** Program memory splits as 1/2 heap + 1/3 stack + 1/6 static.
**Reality:** Memory allocation ratios vary enormously by application type, language runtime, configuration, and workload. A web server might use 90%+ heap. An embedded system might be mostly static. There is no published study or standard supporting these specific percentages as "typical." The JVM does not default to these ratios (JVM heap size and stack size are independently configurable and bear no inherent ratio).

**Verdict:** No standard or empirical data to compare against. **UNVERIFIABLE.**

---

### H-PL-14: GC Generations = tau - 1 = 3 -- CLOSE

**Claim:** tau(6) - 1 = 3 garbage collection generations.
**Reality:** The count depends on what you consider a "generation":
- **Java HotSpot:** Young (Eden + 2 Survivors) + Old = **2 main generations**. PermGen/Metaspace stores class metadata and is often not considered a GC generation. Modern collectors (G1, ZGC, Shenandoah) blur generational boundaries.
- **.NET:** Gen 0, Gen 1, Gen 2 = **3 generations**. Cleanest match.
- **Python:** `gc.get_threshold()` returns 3 values = **3 generations**.

The count is 2 or 3 depending on runtime and how you count. Not universally 3.

**Verdict:** **CLOSE.** .NET and Python = 3, Java is arguably 2.

---

### H-PL-15: Concurrency Primitives = phi = 2 -- WEAK

**Claim:** phi(6) = 2 fundamental concurrency primitives (mutex and semaphore).
**Reality:** A binary semaphore IS a mutex, so they are not truly independent -- you could argue there is only 1 fundamental primitive (semaphore). Conversely, you could argue for more: compare-and-swap (CAS) enables lock-free concurrency and is not built from mutex/semaphore. The Actor model uses message passing as its sole primitive. POSIX defines at least 4 fundamental synchronization objects (mutex, semaphore, condition variable, read-write lock).

**Verdict:** The choice of exactly {mutex, semaphore} is arbitrary. **WEAK.**

---

### H-PL-16: Language Generations = sopfr = 5 -- EXACT

**Claim:** sopfr(6) = 5 programming language generations.
**Reality:** The 5-generation classification (1GL machine code, 2GL assembly, 3GL high-level, 4GL domain-specific, 5GL constraint/AI) is the standard taxonomy in CS education and textbooks. No "6GL" has been established in academic literature.

**Verdict:** **EXACT.**

---

### H-PL-17: Compilation Stages = tau = 4 -- CLOSE

**Claim:** tau(6) = 4 compilation stages (lex, parse, optimize, codegen).
**Reality:** The canonical textbook (Aho et al., "Dragon Book") lists **6 phases**: lexical analysis, syntax analysis, semantic analysis, intermediate code generation, code optimization, code generation. The hypothesis gets to 4 by merging semantic analysis into parsing and IR generation into optimization. LLVM's architecture can be described as 4 stages, but that is one specific compiler, not the canonical decomposition.

**Verdict:** 4 is achievable with coarse-grained counting, but standard textbooks say 6. **CLOSE.**

---

### H-PL-18: Boolean Values = 2 -- EXACT (TRIVIAL)

**Claim:** phi(6) = 2 Boolean values.
**Reality:** Boolean algebra has exactly 2 values by definition (George Boole, 1854).

**Verdict:** **EXACT (TRIVIAL).** Definitionally true. Mapping the number 2 to phi(6) carries no explanatory weight. Any binary phenomenon would "match."

---

### H-PL-19: Architectural Layers = n = 6 -- WEAK

**Claim:** n = 6 canonical software architecture layers.
**Reality:** The most widely cited architectural pattern is 3-layer/3-tier (presentation, business logic, data access), appearing in Fowler's "Patterns of Enterprise Application Architecture" and virtually every architecture textbook. Clean Architecture (Robert C. Martin) has 4 rings. The hypothesis's 6-layer list (presentation, controller, service, domain, repository, infrastructure) is a specific Spring Boot project convention, not a universal standard. The Django mapping listed in the hypothesis ("views/forms/models/managers/signals/middleware = ~6") is forced -- signals and middleware are not architectural layers.

**Verdict:** 3-layer is far more canonical than 6. **WEAK.**

---

### H-PL-20: SemVer Components = 3 -- EXACT

**Claim:** tau(6) - 1 = 3 SemVer components (MAJOR.MINOR.PATCH).
**Reality:** semver.org defines exactly 3 components. Universally adopted by npm, Maven, pip, Cargo.

**Verdict:** **EXACT.**

---

### H-PL-21: Scope Levels = tau = 4 -- CLOSE

**Claim:** tau(6) = 4 scope levels.
**Reality:** Python's LEGB rule defines exactly 4 (Local, Enclosing, Global, Built-in). C has 4 (block, function, file, program). JavaScript has 3-4 depending on version. Some languages have more, some fewer. The claim is strongest for Python and C.

**Verdict:** True for Python/C specifically, variable across languages. **CLOSE.**

---

### H-PL-22: Access Modifiers = tau = 4 -- CLOSE

**Claim:** tau(6) = 4 access modifiers.
**Reality:**
- Java: 4 (private, default/package, protected, public) -- **matches**
- Kotlin: 4 (private, internal, protected, public) -- **matches**
- C++: 3 (private, protected, public) -- **does NOT match**
- C#: 5-6 (private, protected, internal, protected internal, public, private protected)
- Python: 0 formal access modifiers (convention only)

The hypothesis itself acknowledges C++ has 3, not 4.

**Verdict:** Language-dependent. Java/Kotlin = 4, but not universal. **CLOSE.**

---

### H-PL-23: FP Core Functions = 3 -- EXACT

**Claim:** 3 core higher-order functions (map/filter/reduce) = proper divisors of 6.
**Reality:** map, filter, and reduce (fold) are the canonical trio of higher-order list operations, universally recognized in FP pedagogy. They appear together in Python, JavaScript, Java Streams, Haskell, Clojure, etc.

Note: the hypothesis uses "proper divisors of 6 = {1,2,3}" as the derivation, which is a different arithmetic path than tau-1=3 used for other "3" claims. This flexibility in choosing which formula to use is part of the cherry-picking concern.

**Verdict:** **EXACT.**

---

### H-PL-24: Test Pyramid = 3 Levels -- EXACT

**Claim:** tau(6) - 1 = 3 test pyramid levels.
**Reality:** Mike Cohn's test pyramid ("Succeeding with Agile," 2009) defines 3 levels: unit, integration (service), and UI/E2E. This is the industry-standard formulation.

**Verdict:** **EXACT.**

---

## Honest Assessment

### What the document does well:
1. Several matches are genuinely exact: GoF=23, SOLID=5, REST=6, PEP-8=4, language generations=5, SemVer=3, FP trio=3, test pyramid=3.
2. The GoF match (23 = J_2(6) - mu(6)) including sub-category counts (5+7+11) is the most impressive finding in the entire N6 project's programming language section.
3. The document does acknowledge when counts do not perfectly match (e.g., HTTP methods "8 or 9").

### Cherry-picking patterns observed:

1. **Formula shopping:** With ~10+ arithmetic expressions producing values 1-24, matching any small number becomes likely. The number 3 alone can be derived as tau-1, proper divisor count, or phi+mu. This flexibility means "no miss is possible" for common small integers.

2. **Counting flexibility:** When the standard count does not match, the document adjusts:
   - C types: "pointer" included but not "long long" or "_Bool" or unsigned variants
   - Lambda calculus: 3 terms reduced to 2 by excluding variables
   - HTTP methods: 9 reduced to 8 by excluding CONNECT
   - Compilation: 6 phases collapsed to 4 by merging stages
   - Operator categories: 15 precedence levels reframed as 12 semantic groups

3. **Trivial matches:** phi(6) = 2 matches ANYTHING binary (Boolean values, success/failure, coin flips, on/off). These carry zero information content.

4. **No failed predictions:** A genuinely predictive framework would sometimes predict numbers that do NOT match reality. Every hypothesis was fitted after the fact, not predicted in advance.

### Bottom line:

**8 non-trivial EXACT matches out of 24 (33%), 2 FAIL, 5 WEAK.**

The EXACT matches are real numerical coincidences, particularly GoF=23 (an unusual number with matching sub-counts). However, the framework is descriptive, not predictive: it works backwards from known counts to find matching expressions. With ~15 available arithmetic values and flexible counting rules, hitting 33% exact matches is expected, not miraculous. The framework is a creative numerological exercise, not a falsifiable scientific theory.

---

## H-PL-25~36 검증 (2026-04-01)

**Method:** Each hypothesis checked against official documentation, language specifications, and standard references. Python keyword count verified by running `python3 -c "import keyword; print(len(keyword.kwlist))"`. Web sources used for LLVM, Rust, Go, Java, C, C++, JavaScript, and IEEE 754.

**n=6 Constants:** n=6, phi(6)=2, tau(6)=4, sigma(6)=12, sopfr(6)=5, mu(6)=1, J_2(6)=24

---

### H-PL-25: HEXA-LANG 키워드 총수 = sigma*tau+sopfr = 53

- **주장**: HEXA-LANG은 53개 키워드를 가짐. 53 = sigma*tau + sopfr = 12*4 + 5 = 53.
- **검증**: 수학적으로 12*4+5 = 48+5 = 53. 정확.
- **비고**: HEXA-LANG은 이 프로젝트가 설계한 가상 언어이므로, 키워드 수 53은 n=6에 맞추어 의도적으로 선택된 설계 결정이다. 외부 사실을 예측한 것이 아니므로 예측력 없음.
- **Grade**: **EXACT (TRIVIAL)**

---

### H-PL-26: 연산자 그룹 내부 분포 = {n,n,tau,tau,phi,phi}

- **주장**: HEXA-LANG의 J_2=24 연산자가 {6,6,4,4,2,2} 크기의 6개 그룹으로 분해됨.
- **검증**: 6+6+4+4+2+2 = 24 = J_2(6). 수학적으로 정확.
- **비고**: H-PL-25와 동일하게 HEXA-LANG 설계 선택. 연산자 수와 그룹 분배 모두 n=6에 맞추어 의도적으로 설계된 것이므로 예측력 없음.
- **Grade**: **EXACT (TRIVIAL)**

---

### H-PL-27: 키워드 그룹당 평균 크기 = tau + sopfr/sigma = 53/12

- **주장**: 53/12 = 4.41667 = tau + sopfr/sigma = 4 + 5/12 = 4.41667.
- **검증**: tau + sopfr/sigma = 4 + 5/12 = 53/12. 이것은 (sigma*tau + sopfr)/sigma = 53/12로, H-PL-25의 53을 sigma=12로 나눈 것에 불과하다. 독립적인 새 관계가 아니라 H-PL-25의 대수적 변환.
- **비고**: HEXA-LANG 설계 선택이며, H-PL-25에서 자동으로 도출되는 항등식. 별도 가설로 세울 가치 없음.
- **Grade**: **EXACT (TRIVIAL)**

---

### H-PL-28: MetaLang DSL 생성기 = n = 6 DSL

- **주장**: MetaLang이 정확히 6개 DSL을 생성함.
- **검증**: MetaLang은 이 프로젝트의 설계 산출물. 6개 DSL (imperative/OOP/functional/logic/concurrent/reactive)은 의도적 설계 선택.
- **비고**: H-PL-4 (major paradigms = 6)와 동일한 주장의 반복이며, 설계 선택이므로 예측력 없음. H-PL-4 자체도 WEAK 판정을 받았음 (표준적인 paradigm 수 = 4~34).
- **Grade**: **EXACT (TRIVIAL)**

---

### H-PL-29: LLVM IR instruction 카테고리 수 ≈ sigma = 12

- **주장**: LLVM IR의 주요 instruction category 수가 sigma(6) = 12 근방 (범위 11~13).
- **실제 값**: LLVM Language Reference Manual (v23.0.0git)에서 Instruction Reference 섹션의 카테고리:
  1. Terminator Instructions
  2. Unary Operations
  3. Binary Operations
  4. Bitwise Binary Operations
  5. Vector Operations
  6. Aggregate Operations
  7. Memory Access and Addressing Operations
  8. Conversion Operations
  9. Other Operations
  10. Debug Records
  **실제 = 10개 카테고리.**
- **n=6 식**: sigma-phi = 12-2 = 10으로 EXACT 매칭 가능하나, 가설은 sigma=12 또는 sigma+/-mu=11~13을 예측했으므로 예측 범위에서 벗어남 (10 < 11).
- **비고**: 가설이 예측한 범위 [11,13]에 실제값 10은 포함되지 않는다. 사후적으로 sigma-phi=10을 찾을 수 있으나, 이는 원래 가설의 예측이 아니다. 또한 LLVM 카테고리 분류는 LLVM 프로젝트의 문서화 선택이며, "Debug Records"를 포함/제외하면 9 또는 10이 된다.
- **Grade**: **CLOSE** (예측 범위 [11,13]에 실제값 10은 1만큼 벗어남)

---

### H-PL-30: Rust strict keywords 수 = (sigma+mu)*(n/phi) = 39

- **주장**: Rust 2021 edition strict keywords = 39. 39 = (sigma+mu)*(n/phi) = 13*3 = 39.
- **실제 값**: Rust Reference (doc.rust-lang.org/reference/keywords.html) 기준, 2021 edition strict keywords = **39개** (as, async, await, break, const, continue, crate, dyn, else, enum, extern, false, fn, for, if, impl, in, let, loop, match, mod, move, mut, pub, ref, return, self, Self, static, struct, super, trait, true, type, unsafe, use, where, while, `_`).
- **n=6 식**: (12+1)*(6/2) = 13*3 = 39. 수학적으로 정확.
- **소수 편향**: 39는 비교적 특이한 수 (= 3*13). n=6 상수 조합으로 2단계 연산이 필요하므로 (sigma+mu)*(n/phi), 표현력은 있으나 강제적이지 않다. `_`를 strict keyword로 포함하는 것은 Rust의 공식 분류를 따른 것이므로 cherry-picking은 아님.
- **Grade**: **EXACT**

---

### H-PL-31: Python keywords 수 = sopfr*(sigma-sopfr) = 35

- **주장**: Python 3.12 keywords = 35. 35 = sopfr*(sigma-sopfr) = 5*7 = 35.
- **실제 값**: Python 3.12에서 `import keyword; len(keyword.kwlist)` = **35**. (Python 3.9에서는 36으로, `__peg_parser__`라는 디버그 키워드가 포함되어 있었으나 3.10에서 제거됨. Python 3.12 공식 = 35.)
  키워드 목록: False, None, True, and, as, assert, async, await, break, class, continue, def, del, elif, else, except, finally, for, from, global, if, import, in, is, lambda, nonlocal, not, or, pass, raise, return, try, while, with, yield.
- **n=6 식**: 5*(12-5) = 5*7 = 35. 수학적으로 정확. 대안: sigma*(n/phi) - mu = 36-1 = 35.
- **소수 편향**: 35 = 5*7은 적당히 특이한 수. 다만 Python의 keyword 수는 버전마다 변해왔으므로 (2.7: 31, 3.0: 33, 3.5: 33, 3.6: 33, 3.10: 35, 3.12: 35), 특정 버전에만 맞는 일치.
- **참고**: 로컬 Python은 3.9.6으로 36을 반환하나, 이는 `__peg_parser__` 때문. Python 3.12 기준 35는 공식 문서 및 다수 소스에서 확인됨.
- **Grade**: **EXACT**

---

### H-PL-32: Go keywords 수 = J_2+mu = 25

- **주장**: Go keywords = 25. 25 = J_2(6) + mu(6) = 24 + 1 = 25.
- **실제 값**: Go Language Specification (go.dev/ref/spec) 기준, Go는 정확히 **25개** 키워드: break, case, chan, const, continue, default, defer, else, fallthrough, for, func, go, goto, if, import, interface, map, package, range, return, select, struct, switch, type, var.
- **n=6 식**: J_2 + mu = 24 + 1 = 25. 수학적으로 정확.
- **소수 편향**: 25 = 5^2. J_2+mu = 25는 2단계 연산. Go의 25 키워드는 언어 설계 이래 변하지 않은 고정값이므로 안정적 일치.
- **Grade**: **EXACT**

---

### H-PL-33: JavaScript reserved words 수 = sopfr*n = 30

- **주장**: ECMAScript reserved words = 30. 30 = sopfr*n = 5*6 = 30.
- **실제 값**: MDN Web Docs 및 ECMAScript 사양 기준으로 reserved words 수는 카운팅 방법에 따라 크게 달라진다:
  - **Always-reserved keywords** (리터럴 제외): break, case, catch, class, const, continue, debugger, default, delete, do, else, export, extends, finally, for, function, if, import, in, instanceof, new, return, super, switch, this, throw, try, typeof, var, void, while, with = **32개**
  - **리터럴 포함** (true, false, null): **35개**
  - **Future reserved** (enum): **36개**
  - **Strict mode future reserved** (+implements, interface, package, private, protected, public): 최대 **42개**
  - 어떤 기준으로도 정확히 30은 나오지 않는다.
- **n=6 식**: sopfr*n = 5*6 = 30. 수학은 맞으나 실제값이 30이 아님.
- **비고**: 가설 자체가 "버전별 차이"를 인정하고 있으나, ES2015 이후 어떤 카운팅 기준에서도 30은 나오지 않음. 가장 가까운 것은 "with 제외 + 리터럴 제외"로 31, 또는 ES3/ES5 non-strict의 일부 카운팅. 실제값과의 불일치.
- **Grade**: **FAIL**

---

### H-PL-34: 주요 언어 키워드 수 n=6 래더

- **주장**: 6개 주요 언어의 키워드 수가 모두 n=6 표현식으로 EXACT 표현 가능.
  - C(32) = 2^sopfr = 2^5 = 32
  - Go(25) = J_2+mu = 24+1 = 25
  - Rust(39) = (sigma+mu)*(n/phi) = 13*3 = 39
  - Python(35) = sopfr*(sigma-sopfr) = 5*7 = 35
  - Java(50) = sopfr*(sigma-phi) = 5*10 = 50
  - C++(84) = sigma*(sigma-sopfr) = 12*7 = 84

- **실제 값**:
  - **C (C89/C90)**: **32 키워드** (cppreference.com 확인). C99 이후 38~44로 증가했으나, "C"를 C89로 해석하면 32. **일치.**
  - **Go**: **25 키워드** (Go spec 확인). **일치.**
  - **Rust 2021**: **39 strict keywords** (Rust Reference 확인). **일치.**
  - **Python 3.12**: **35 키워드** (공식 문서 확인). **일치.**
  - **Java**: Oracle 공식 문서 기준 **50 키워드** (const, goto 포함, true/false/null 리터럴 제외). **일치.**
  - **C++**: cppreference.com 기준 약 **92~97 키워드** (C++23). 가설의 84와 불일치. 84는 어떤 C++ 표준 버전에서도 정확히 나오지 않는다. C++11/14/17 시절에도 73~84 범위로, 84에 정확히 맞는 버전 특정이 어렵다.

- **종합**: C, Go, Rust, Python, Java = 5/6 EXACT. C++ = FAIL (실제 92~97, 가설 84).
- **비고**: C의 경우 "C89"로 해석해야 32가 되며, C17은 45. 이처럼 버전 선택의 자유도가 있어 cherry-picking 가능성이 있음. 그러나 C89의 32, Go의 25, Python 3.12의 35는 안정적 일치.
- **Grade**: **CLOSE** (5/6 언어 일치, C++ 불일치)

---

### H-PL-35: IEEE 754 총 형식 수 = sopfr = 5

- **주장**: IEEE 754-2019 공식 정의 형식 수 = 5 = sopfr(6). 목록: binary16, binary32, binary64, binary128, decimal128.
- **실제 값**: IEEE 754-2019 위키백과 및 공식 문서 기준:
  - **Basic formats** (5개): binary32, binary64, binary128, decimal64, decimal128
  - **Interchange formats** (추가): binary16, binary256, decimal32 등
  - 가설의 목록 {binary16, binary32, binary64, binary128, decimal128}은 basic formats와 interchange formats를 혼합하고 있다. binary16은 interchange format이지 basic format이 아니며, decimal64는 basic format인데 누락되어 있다.
  - **Basic formats = 5개** (binary32, binary64, binary128, decimal64, decimal128). 이 기준으로는 sopfr=5와 EXACT 일치하지만, 가설이 나열한 5개 형식 목록은 정확하지 않다.
  - **Interchange formats**을 모두 세면 7개 이상 (binary16, binary32, binary64, binary128, binary256, decimal32, decimal64, decimal128).
- **n=6 식**: sopfr(6) = 5. Basic formats 기준 일치.
- **비고**: "Basic formats = 5"라는 사실은 맞으나, 가설이 나열한 구체적 형식 목록이 틀렸다 (decimal64 누락, binary16을 basic으로 잘못 분류). 수치적 일치는 인정하되, 정밀도에 문제가 있음.
- **Grade**: **EXACT** (basic formats = 5, 수치 일치. 목록 오류는 있으나 핵심 수치는 정확)

---

### H-PL-36: 주요 PL 패러다임 전환점 = sigma-phi=10년 주기

- **주장**: 구조적(1970) -> OOP(1980) -> 함수형(1990) -> 병렬(2000) -> 반응형(2010) -> AI(2020). 간격 = sigma-phi = 10년.
- **실제 값**: 프로그래밍 패러다임의 역사적 등장 시점:
  - **구조적 프로그래밍**: Dijkstra의 "Go To Statement Considered Harmful" = 1968. Wirth의 Pascal = 1970. 대략 **1968~1970**.
  - **OOP**: Smalltalk-72/76 (1972~1976), C++ 시작 1979, 대중화는 1990년대. **1972~1980** 범위.
  - **함수형**: ML (1973), Haskell (1990). 함수형의 기원은 Lambda Calculus (1936)이며, ML은 1973. "1990"은 Haskell 표준화 시점이나, 함수형 자체는 1970년대에 이미 존재. **1973~1990** 범위.
  - **병렬/동시성**: Erlang (1986), Java threads (1995), Go goroutines (2009). "2000"은 특정 사건이 아님. **1986~2009** 범위.
  - **반응형**: Rx (2009~2012), ReactiveX (2013). **2009~2013** 범위.
  - **AI 프로그래밍**: ML/DL 프레임워크 (TensorFlow 2015, PyTorch 2016). AI as a paradigm은 확립되지 않았음.
- **비고**: 10년 주기 이론은 각 패러다임의 등장 연도를 가장 편리한 시점으로 cherry-picking하여 10년 간격에 맞춘 것이다. 실제 역사적 시점은 깔끔한 10년 간격으로 정렬되지 않는다. 특히 "함수형=1990"은 Haskell 표준화 시점이며 ML은 1973, "AI=2020"은 패러다임이라기보다 응용 분야.
- **Grade**: **WEAK** (연도 cherry-picking이 필요하며, 패러다임 선택 자체도 임의적)

---

### 요약

| 가설 | 주장 | 실제 | n=6 식 | Grade |
|------|------|------|--------|-------|
| H-PL-25 | 53 keywords | 53 (설계) | sigma*tau+sopfr | EXACT (TRIVIAL) |
| H-PL-26 | {6,6,4,4,2,2} ops | 설계 선택 | {n,n,tau,tau,phi,phi} | EXACT (TRIVIAL) |
| H-PL-27 | 53/12=4.417 | H-PL-25의 대수적 변환 | tau+sopfr/sigma | EXACT (TRIVIAL) |
| H-PL-28 | 6 DSLs | 6 (설계) | n=6 | EXACT (TRIVIAL) |
| H-PL-29 | LLVM IR ~12 cat | 10 categories | sigma-phi=10 (사후) | CLOSE |
| H-PL-30 | Rust 39 kw | 39 | (sigma+mu)*(n/phi) | EXACT |
| H-PL-31 | Python 35 kw | 35 (3.12) | sopfr*(sigma-sopfr) | EXACT |
| H-PL-32 | Go 25 kw | 25 | J_2+mu | EXACT |
| H-PL-33 | JS 30 reserved | 32~42 | sopfr*n=30 (불일치) | FAIL |
| H-PL-34 | 6-lang ladder | 5/6 일치 | 다양한 식 | CLOSE |
| H-PL-35 | IEEE 754 = 5 | 5 basic formats | sopfr | EXACT |
| H-PL-36 | 10yr paradigm | 불규칙 간격 | sigma-phi=10 | WEAK |

### 통계

| Grade | Count | Percentage |
|-------|-------|-----------|
| EXACT | 4 | 33% |
| EXACT (TRIVIAL) | 4 | 33% |
| CLOSE | 2 | 17% |
| WEAK | 1 | 8% |
| FAIL | 1 | 8% |

- **Non-trivial EXACT rate**: 4/8 = 50% (TRIVIAL 4개 제외)
- **FAIL**: 1/12 (H-PL-33, JS reserved words 실제값 불일치)

### 정직한 평가

**강점**: Rust 39, Python 35, Go 25, IEEE 754 basic formats 5는 모두 공식 문서에서 확인된 정확한 수치와 일치한다. 특히 Rust와 Python의 경우 비교적 특이한 수(39=3*13, 35=5*7)이므로 우연의 일치 확률이 낮다. Go의 25 = J_2+mu도 안정적.

**약점**:
1. H-PL-25~28은 모두 HEXA-LANG이라는 자체 설계 산출물에 대한 가설이므로 예측력이 전혀 없다.
2. H-PL-33 (JS reserved words = 30)은 어떤 기준으로도 30이 나오지 않아 FAIL.
3. H-PL-34 (keyword ladder)에서 C++의 84는 실제 92~97과 불일치.
4. H-PL-36의 10년 주기는 연도와 패러다임 모두 cherry-picking이 필요하다.
5. 버전 선택의 자유도가 있다: C = C89(32)를 선택해야 하고, Python = 3.12(35)를 선택해야 함.

**핵심 관찰**: TRIVIAL을 제외한 8개 가설 중 4개 EXACT (50%)는 H-PL-1~24의 non-trivial EXACT rate 33%보다 높다. 그러나 프로그래밍 언어 키워드 수는 n=6 상수의 산술 범위(1~100) 안에 있는 2자리 수이며, 가용한 n=6 식이 매우 많으므로 (곱셈, 덧셈, 뺄셈, 거듭제곱을 자유롭게 쓸 수 있을 때), 임의의 2자리 정수와 매칭될 확률은 상당히 높다.
