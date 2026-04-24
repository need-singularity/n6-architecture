# DFS-24 방향 탐침 — BT-542 P vs NP (Out(S_6) 유일성 & Schaefer 6치 이분법)

**세션 유형**: 연구 방향 설정 (RESEARCH-ONLY, NO proof claims)
**대상 BT**: 542 (P vs NP) · Clay OPEN since 1971
**선행 상태**: DFS 23 완료 · BT-542 약 40 tight · MISS→OBSERVATION 탈출은 DFS 3~5(2026-04-11~12) 에 이미 종료
**작성일**: 2026-04-24
**브랜치**: main

---

## 0. 정직성 선언 (`reports/millennium-dfs-status.md` §명시적 부인 계승)

본 문서는 **방향 제안**이다. P vs NP 에 대한 증명·부분 증명·해결 근접성을 주장하지 않는다. BT-542 현 상태는 여전히 **OPEN · 0/7 미해결** 이며, 약 40 tight 는 n=6 산술 시그니처의 **구조적 관찰**이지 증명이 아니다. 세 장벽(relativization / natural proofs / algebrization)은 미돌파. 본 제안의 모든 탐침은 "n=6 유일성 구조를 **더 선명하게 고립**" 시키는 목적이지 **우회 증명** 이 아니다.

---

## 1. 현 위치 (40 tight 핵심 축)

BT-542 의 견고한 앵커는 **두 개** 이다 (나머지는 보조 관찰):

- **A. Out(S_n) 유일성** (Hölder 1895): `Out(S_n) ≠ 1 ⟺ n = 6`. 군론 내부 정리, Bernoulli 무관, T4.
  - 원천: C(6,2)=15 공액류 + PGL(2,5)≅S_5 의 **이중 우연** (`theory/study/p0/pure-p0-2-group-theory.md` §5.8).
  - 합성군 외부에서 n=6 을 "분리" 하는 유일 구조 사실.

- **B. Schaefer 6치 이분법** (Schaefer STOC 1978): Boolean CSP 가 P 에 속하는 제약 관계 family **정확히 6 개** (0-valid / 1-valid / Horn / dual-Horn / bijunctive(2-SAT) / affine). 나머지는 NP-완전. T4, Post lattice/universal algebra 기반.
  - 복잡도 **이분법** 의 원형 — CSP 의 모든 유한 template 가 P 또는 NP-완전 (Bulatov/Zhuk 2017 이분법 정리) 로 가는 출발점.

보조 tight (참고): AC⁰[2]≠AC⁰[3] (Razborov-Smolensky φ,n/φ), Barrington width-5, Karp 21=3·7, Ramsey R(3,3)=6, AKS 최선 Õ(log^6 n), Savitch 지수 φ. **MISS 2 건** (Immerman-Szelepcsényi, Toda) 은 정직히 기록 유지.

---

## 2. 방향 명제 (본 세션의 가설, 증명 아님)

**D-1 (가설, 반증 가능)**: Out(S_6) 비자명성의 두 "이중 우연" — (i) `|C(6,2)|=15` 가 transposition-클래스 크기와 일치, (ii) `PGL(2,5) ≅ S_5` 의 두 비동치 transitive embedding — 이 P vs NP 복잡도 이론 대상과 **구성적으로 매칭** 되는 객체가 있는가?

**D-2 (가설, 반증 가능)**: Schaefer 6 tractable family 의 "6" 은 Post lattice 의 **clone-격자 구조**에서 자연스럽게 나온다 (Böhler-Reith-Vollmer 2003). 이 6 에 Out(S_6) 의 두 `S_6`-궤도 구조가 대응 가능한가? 아니면 순전한 우연인가?

두 가설 모두 **P ≠ NP 와 독립** 이다. 성립해도 본 난제 증명과는 무관하며, 실패해도 구조 관찰 40 tight 는 유지된다.

---

## 3. 세 개의 구체 탐침 (DFS-24 후보)

### PROBE-24A — Out(S_6) × Schaefer 6 `S_6`-궤도 매칭 (이분법 측)

**질문**: Boolean 영역 `{0,1}` 위의 **6개 Schaefer tractable clone** 집합 `𝓢 = {C_0, C_1, C_Horn, C_dHorn, C_2SAT, C_aff}` 에 `S_6` 가 자연스럽게 작용하는가? 작용한다면 Out(S_6) 의 비자명 `α` 가 `𝓢` 위에 **5+1 이 아닌** 비자명 치환을 유도하는가?

- **관측 대상**:
  - `C_0 ↔ C_1` (negation 쌍), `C_Horn ↔ C_dHorn` (dual 쌍), `C_2SAT` 와 `C_aff` (고정점 후보) — 2·2·1·1 구조.
  - Out(S_6) 는 전치 클래스↔3-사이클 클래스를 교환. `𝓢` 위 `α` 가 `{C_0,C_1}↔{C_Horn,C_dHorn}` 교환을 유도하는지 검증.
- **단단한 사실 (베이스라인)**: Schaefer 정리는 `Aut({0,1})=ℤ/2` (negation) 만 사용. `S_6` 작용은 **문헌에 없음** — 존재 자체가 미탐색.
- **반증조건 (falsifier)**: `𝓢` 위 자연 `S_6` 작용이 존재하지 않거나, 존재해도 `α` 가 trivial 로 descend 하면 D-1 기각.
- **기대 산출**: tight +0~+2 (존재 시), negative tight +1 (부존재 증명 시 — 마찬가지로 가치).
- **tier**: T4 구조 탐색. 즉시 실행 가능 (universal algebra stdlib + Post lattice 표).

### PROBE-24B — Sylvester synthematic total 과 3-SAT 인증자의 비아벨 구조

**질문**: S_6 의 15 duads · 15 synthemes · 6 synthematic totals (Sylvester 구성, `pure-p0-2-group-theory.md` §5.8) 는 `C(6,2)=15` 더블 카운트의 원천이다. 3-SAT `(x∨y∨z)` 의 절당 리터럴 배치에서 **15 패턴 = `C(6,2)`** 이 등장하는가?

- **기초 사실**: 3-SAT 절 변수 개수 3 = n/φ. 6 변수 3-SAT 에서 절의 변수 선택 `C(6,3) = 20`. 리터럴 부호 쌍 `C(6,2) = 15` — 두 변수 간 부호쌍 관계.
- **시도**: 6변수 Boolean formula 공간에 synthematic total 의 `S_6`-궤도 구조를 올릴 수 있는지. 외부 자기동형 `α` 가 "어려운 SAT 인스턴스" 와 "쉬운 인스턴스" 를 교환하는가?
- **반증조건**: 궤도가 NP-완전성 클래스를 존중하지 않으면 기각 (즉 `α(tractable)` 이 tractable 유지되지 않음 = 우연).
- **현실 제약**: 이 방향은 **Natural Proofs 장벽을 건드린다** — `α` 가 largeness + constructivity 를 만족하면 Razborov-Rudich 1997 에 걸릴 수 있음. 오히려 장벽을 **확인** 하는 음성 결과를 목표로 설계.
- **기대 산출**: 궤도 구조 존재/부재 확정. tight +1 (어느 쪽이든).
- **tier**: T3-T4 혼합. 실험 코드 + Sylvester 수기 검증.

### PROBE-24C — Schaefer 6 ↔ Bulatov-Zhuk 2017 이분법 경계에서 n=6 재현 검증

**질문**: Schaefer 의 6 은 Boolean (|D|=2) 한정이다. 일반 유한 domain CSP 이분법(Bulatov-Zhuk 2017 독립 증명) 의 tractability 경계 `omega`-minimal polymorphism 조건을 `|D|=6` 에 제한하면 **tractable clone 수** 가 구조적으로 등장하는가?

- **기초 사실**:
  - Schaefer (|D|=2): 6 tractable families. `= n`.
  - Bulatov (|D|=3) 2002: 이분법 증명, tractable clone 수는 분류되어 있으나 "단일 수" 가 아님.
  - 일반 |D|: 이분법은 참이지만 `|D|` 가 자체로 파라미터.
- **시도**: `|D|=6` 에서 Siggers/`omega`-Taylor 조건을 만족하는 "Schaefer-style" canonical family 수를 세기. 만약 Bulatov 이후 결과에서 `|D|=6` 에 특이한 축소가 있다면 Out(S_6) 가 관여하는지 별도 검증.
- **반증조건**: `|D|=6` 가 `|D|=5, 7, 8` 보다 특별하지 않으면 D-2 기각. Schaefer 6 은 Boolean 도메인 크기 `2` 의 power-set 구조의 우연.
- **기대 산출**: 정상 결과 = "우연" 확정 (tight -1, 기존 Schaefer 6 tight 보존). 특이 결과 = D-2 후속 탐침.
- **tier**: T3. 문헌 조사 + Post-type clone 카운팅.

---

## 4. 세 탐침 공통 Falsifier 요약

| Probe | 가장 직접적 반증 | 반증 시 행동 |
|---|---|---|
| 24A | `𝓢` 위 자연 `S_6` 작용 없음 or `α` 가 trivial | D-1 기각, 기존 tight 유지 |
| 24B | synthematic 궤도가 tractable/hard 클래스 가로지름 | D-1 확장판 기각 |
| 24C | `|D|=6` 에서 특이 축소 없음 | D-2 기각, Schaefer 6 은 Boolean 우연 |

**세 탐침 모두 실패해도** BT-542 의 40 tight 중 2 앵커(Out(S_6), Schaefer 6)는 불변이다. 이 탐침은 **유일성 구조의 정밀화** 가 목표이지 **증명** 이 아니다.

---

## 5. DFS-24 실행 권고

- **우선순위**: 24A > 24C > 24B (24B 는 Natural Proofs 장벽 근접 — 장벽 확인이 목표).
- **병렬 에이전트**: 3기 (Universal algebra / CSP 이분법 / Sylvester 조합론).
- **금지 사항**:
  - "P ≠ NP 증명에 가까워졌다" 류 표현 금지.
  - tight 카운트 인플레 금지 (음성 결과도 정직 기록).
  - Razborov-Rudich natural proofs 조건을 우회했다는 주장 금지.
- **예상 산출**: tight +0~+3, MISS 기록 가능 (환영 — `reports/millennium-dfs-status.md` §6.2 baseline 원칙).

---

## 6. 연결 파일

- 앵커 증거: `theory/breakthroughs/breakthrough-theorems.md` §BT-542 (#1~#21).
- Out(S_6) 내부 증명: `theory/study/p0/pure-p0-2-group-theory.md` §5.7~§5.9 (Hölder 1895).
- Schaefer 맥락: `theory/study/p2/n6-p2-4-honesty-audit.md` §8.2 · `theory/study/p3/prob-p3-3-hexa-verification.md` §2.2.
- MISS→OBSERVATION 복기: `theory/study/p2/n6-p2-4-honesty-audit.md` §6 (DFS 3~5 기록, 2026-04-11~12 — **본 세션 이전**).
- 정직 규약: `reports/millennium-dfs-status.md` §명시적 부인 + §6 정직성 감사.

---

## 7. 종결

DFS-24 는 **방향 제안만** 한다. 탐침 실행은 별도 세션. 본 문서 작성으로 7대 난제 해결 수치 `0/7` 는 변하지 않으며, BT-542 는 `OPEN (1971~)` 이다. 본 세션은 P vs NP 에 대한 새로운 증명이나 부분 증명을 주장하지 않는다.
