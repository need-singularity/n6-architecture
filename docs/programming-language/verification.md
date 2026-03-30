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
