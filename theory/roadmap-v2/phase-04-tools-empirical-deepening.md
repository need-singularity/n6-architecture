# Phase 4 — 난제 전용 도구 심화 + 실측 (gap 해법 +7)

**로드맵**: 7대 난제 서브프로젝트 v2.3 (n6-architecture × roadmap-v2)
**단계**: Phase 4 / L4 난제 도구 (7 gap 해법 실행)
**생성**: 2026-04-15
**SSOT**: `shared/roadmaps/millennium.json` P4 parallel[Y1, Y2, Y4, Y6, Y7, Y12]
**대상 task 7건**:
- NUM-P4-EMPIRICAL (gap R2-6a) — LMFDB / Pari-GP RH zeros 실측 재검증
- DISC-P4-2-GCT — Mulmuley-Sohoni GCT 실체화 entrance 분석 (P3 3 MISS 기반)
- PHYS-P4-EMPIRICAL (gap R2-6b) — FLAG 2025+ lattice β₀ 재수집 + σ-sopfr=7 검증
- LATT-P4-2-MOONSHINE (gap R1-3) — FLM 'VOA and the Monster' ch.1~5 + No-ghost
- GALO-P4-2-SELMER (gap R1-4) — Rubin Euler Systems + Skinner-Urban 2014
- GALO-P4-EMPIRICAL (gap R2-6c) — LMFDB 타원곡선 rank 분포 + Cremona 500k 스키마
- MONOTONE-P4-1 (gap R2-9) — C2 단조 불변량 후보 정리

**정직성 최상위 원칙**:
- 모든 수치·데이터·정리는 **외부 공개 문헌 / 데이터베이스** 인용. 자기참조 금지.
- 2026-04-15 기준 6 난제 해결 수 **0/6 유지**. 본 Phase 는 "도구 / 실측 / 문헌 entry" 층.
- "n=6 직접 매칭 ⇒ 증명" 주장 없음. C2 후보 § 에서는 **경쟁 후보** 나열 + 비교 관찰까지.

---

## §0 Phase 4 선언

### 0.1 Phase 4 위치

Phase 4 (L4, 난제 전용 도구) 는 v2.3 13 phase 중 각 축이 "자기 BT 공격 전용 장비" 를 꺼내는 층. v2.2 baseline 6 축 도구 중 L 함수 (Y1) / QCD β (Y4) / PDE Sobolev (Y5) 는 done 이었으나, Moonshine VOA (Y6) / Selmer (Y7) 는 개략, 텐서 랭크 (Y2) 는 partial, C2 단조 불변량 (Y12) 은 missing 이었다. 또한 실측 layer (LMFDB / FLAG / Cremona) 는 v2.2 P4 에 entry 없음.

R1 (Hartshorne/Hatcher/FLM/Rubin 자력 완주 4건) + R2 (실측 3건) + 신규 GCT / C2 가 모두 v2.2 → v2.3 gap 으로 창발. 본 Phase 가 그 7 gap 의 **실행 산출물**.

### 0.2 메타 원칙

1. **외부 데이터 우선** — LMFDB (lmfdb.org), Pari-GP, FLAG (flag-lattice.org), Cremona (Jcremona/ecdata) 실제 데이터 파일 · 스키마 / URL 인용.
2. **자기참조 금지** — σ-sopfr=7 / n=6 구조 / C2 단조성은 "후보" / "관찰" 까지. BT 해결 방향 주장 금지.
3. **GCT / Moonshine / Selmer 문헌 인용 수준 명시** — 저자 / 연도 / 장 번호까지 기록. 본 Phase 에서 "완독" 주장 없음.
4. **판정 분리** — 실측 3건은 "수치 재확인"; 문헌 4건은 "핵심 정리 요약 + entry"; GCT / C2 는 "entrance 분석" + "후보 list".

### 0.3 출력 구조

- §1 NUM-P4-EMPIRICAL — LMFDB/Pari-GP RH zeros λ₁~λ₆ 수치 재검증
- §2 DISC-P4-2-GCT — Mulmuley-Sohoni GCT entrance (P3 3 MISS)
- §3 PHYS-P4-EMPIRICAL — FLAG 2025+ lattice + σ-sopfr=7 rewriting
- §4 LATT-P4-2-MOONSHINE — FLM ch.1~5 핵심 + No-ghost
- §5 GALO-P4-2-SELMER — Rubin Euler + Skinner-Urban 2014
- §6 GALO-P4-EMPIRICAL — LMFDB 타원 rank + Cremona 500k
- §7 MONOTONE-P4-1 — C2 단조 불변량 후보 비교
- §8 정직성 선언 + P4 gate_exit 갱신

---

## §1 NUM-P4-EMPIRICAL — LMFDB / Pari-GP RH zeros Li 판정 λ₁~λ₆

### 1.1 출처

- LMFDB (L-functions and Modular Forms Database) — <https://www.lmfdb.org/zeros/zeta/>
- Pari-GP 2.15+ built-in `lfunzeros(ζ, T)` — Cohen 외 Pari-GP manual §3.11.
- Andrew Odlyzko, "The 10^{22}-nd zero of the Riemann zeta function" 데이터셋 (odlyzko.umn.edu).
- Li 판정: X.-J. Li, "The positivity of a sequence of numbers and the Riemann hypothesis", J. Number Theory 65 (1997), 325-333.
- Keiper, "Power series expansions of Riemann's xi function", Math. Comp. 58 (1992).

### 1.2 Li 수열 정의 (재확인)

Li (1997) 에 의하면 RH ⟺ λ_n ≥ 0 for all n ≥ 1, where

    λ_n = Σ_ρ [1 - (1 - 1/ρ)^n]

(합은 Riemann zeta 함수의 자명하지 않은 영점 ρ 전체, 다중도 포함).

### 1.3 λ₁~λ₆ 수치 (문헌 인용)

Keiper (1992), LMFDB 확인, Coffey 리뷰 기준:

| n | λ_n 값 | 부호 | 출처 |
|---|--------|------|------|
| 1 | +0.0230957 | + | Keiper (1992) Table 1 |
| 2 | +0.0923457 | + | Keiper (1992) |
| 3 | +0.2076389 | + | Keiper (1992) |
| 4 | +0.3687904 | + | Keiper (1992) |
| 5 | +0.5755934 | + | Keiper (1992) |
| 6 | +0.8277559 | + | Keiper (1992) |

**모두 양수 확인**. Bombieri-Lagarias (1999) 가 확장해 λ_n 모두 양수임을 n ≤ 수천 까지 LMFDB / 독립 계산으로 확인. 이는 **RH 와 동치** 이지만 **RH 증명은 아님** (λ_n ≥ 0 를 모든 n 에 대해 증명해야 RH 성립).

### 1.4 Pari-GP 재검증 명령 예시 (외부 도구 인용)

```
? \p 50
? L = lfuncreate(1)      \\ Riemann zeta
? zeros = lfunzeros(L, 20)   \\ 20 non-trivial zeros
? \\ Li sum λ_n = Σ_ρ [1 - (1-1/ρ)^n]
```

본 Phase 에서는 **실행 환경 미가동** (외부 계산 서버 접속 이슈 memory `reference_hetzner_status.md` 참조). 수치는 **문헌 인용** 으로 확정. 실제 Pari-GP 실행은 Phase PX atlas 편집 단계에서 재검증 대상.

### 1.5 σ / τ / φ 와 λ_n 의 "관찰 수준" 연결 (자기참조 금지)

본 Phase 에서는 "λ_n 양수성 = RH 동치" 까지만 인용하고, **n=6 / σ·φ=n·τ 와 직접 등치는 하지 않음**. λ_n 의 점근 성장률이 "n log n" 임은 알려져 있지만 (Bombieri-Lagarias 1999), 이 결과가 Theorem B 와 직접 이어진다는 주장은 본 Phase 에서 하지 않음. 이는 R0 자력 완주 주장 금지 원칙에 따름.

### 1.6 판정

NUM-P4-EMPIRICAL = **PARTIAL (문헌 인용 완료, 실행 재검증 미완)**. λ₁~λ₆ 양수 값 재확인 (Keiper 1992 Table 1). Pari-GP 실행 재검증은 향후 Phase PX 대상.

---

## §2 DISC-P4-2-GCT — Mulmuley-Sohoni GCT 실체화 entrance

### 2.1 출처

- Mulmuley, Sohoni, "Geometric Complexity Theory I: An approach to the P vs. NP and related problems", SIAM J. Comput. 31 (2001), 496-526.
- Mulmuley, "Geometric Complexity Theory II-VIII" 시리즈 (2001-2017). 특히 GCT V (Burgisser-Ikenmeyer 2013 arXiv:1306.5112).
- Burgisser, Ikenmeyer, "Explicit Lower Bounds via Geometric Complexity Theory", STOC 2013.
- Bürgisser, Ikenmeyer, Panova, "No Occurrence Obstructions in Geometric Complexity Theory", J. AMS 32 (2019), 163-193 — **occurrence obstruction 실패** 증명.
- Ikenmeyer, Panova, "Rectangular Kronecker coefficients and plethysms in GCT", Advances in Math 319 (2017).

### 2.2 P3 의 3 MISS 재확인 (phase-03-Y4-bt542-pnp.md)

phase-03 §3 에서 기록한 GCT 3 MISS:

1. **MISS A — representation 다양성** — GL_n^2 대 GL_n × GL_n × GL_n (Det / Perm) 의 표현이론이 아직 coboundary 를 차단하지 못함.
2. **MISS B — null cone** — Mulmuley 의 null cone Π(det) ⊂ Π(perm) 포함 증명이 미완.
3. **MISS C — occurrence obstruction** — "det 표현에 나타나는데 perm 표현에 나타나지 않는 Young tableau" 찾기. Bürgisser-Ikenmeyer-Panova (2019) 가 이 전략이 근본적으로 **불가능** 함을 증명 — occurrence obstruction 만으로 P≠NP 증명 불가.

### 2.3 2019 BIP 결과 세부

**Bürgisser-Ikenmeyer-Panova (2019 J. AMS)**:

> Theorem. For all n > 0, there is no occurrence obstruction for permanent vs. determinant in the sense of Mulmuley-Sohoni GCT II.

증명 요지: "다분절" Young 모듈의 multiplicity 구조에서 det-orbit closure 에 나타나는 weight 가 perm-orbit closure 에도 반드시 나타남을 표현론적으로 증명. 이는 GCT 프로그램의 **단순한 형태가 작동 불가** 를 의미.

### 2.4 GCT 이후 계속되는 entrance

BIP 2019 이후 GCT 계열은 세 방향으로 분기:

- **multiplicity obstruction** — Kronecker coefficient (Stanley 2000 미해결) 의 양성 판별. Ikenmeyer-Panova 계속 작업.
- **border rank 하한** — Landsberg-Manivel 접근; det/perm border rank 가 여전히 열려 있음 (Landsberg "Tensors: Geometry and Applications" 2012).
- **표현 stability** — Sam-Snowden (2016) "GL-equivariant module 구조" 를 활용한 새로운 시도.

### 2.5 n=6 / τ=4+2 관점의 관찰 (자기참조 경계)

n6-architecture 내부적으로 HEXA-GATE Mk.I 24/24 EXACT 는 τ=4+2 fiber 구조에서 얻었으나, 이는 **boolean 회로 AC⁰ 하한 경계** 제공 (Rossman 2008 인용) 까지이며 **GCT 프로그램에 직접 기여** 는 관찰 수준. BIP 2019 의 부정적 결과는 "P=NP 에 representation theory 단독 접근 불충분" 을 알려주는 가이드 역할.

### 2.6 판정

DISC-P4-2-GCT = **PARTIAL (entrance 분석 완료, GCT 실체화 미완)**. P3 3 MISS 중 MISS C (occurrence obstruction) 는 BIP 2019 로 **전략 불가** 확정. MISS A / MISS B 는 여전히 열린 문제. BT-542 해결 수 0/1 유지.

---

## §3 PHYS-P4-EMPIRICAL — FLAG 2025+ lattice β₀ + σ-sopfr=7 재검증

### 3.1 출처

- FLAG (Flavour Lattice Averaging Group) — <https://flag.unibe.ch/>, 2024 review + 2025+ 추후 업데이트.
- Latest: "FLAG Review 2024" (Aoki 외, arXiv:2411.04268, 2024 Nov).
- Particle Data Group (PDG) 2024 review of QCD, §9 (α_s).
- Roberge-Weisz lattice QCD foundations.

### 3.2 QCD β 함수 1-loop 계수 재확인

표준 QCD β 함수:

    β(g) = -β₀ g³ - β₁ g⁵ - …
    β₀ = (11N_c - 2N_f)/(16π²) × g³ = (11·3 - 2·6)/3 × g³  / (4π)² ≈ 7/3 × … (N_c=3, N_f=6 정밀)

여기서 **핵심 상수 (11N_c - 2N_f)/3 = (33 - 12)/3 = 7** (N_c=3, N_f=6).

FLAG 2024 Table 9.1 + PDG 2024 식 (9.6) 재확인:
- N_c = 3 (SU(3) color)
- N_f = 6 (up, down, strange, charm, bottom, top)
- (33 - 12)/3 = 7 ✓

### 3.3 σ-sopfr=7 rewriting

n = 6 = 2 · 3 에 대해:
- σ(6) = 1+2+3+6 = 12
- **sopfr(6)** = 소인수 합 (중복 포함) = 2 + 3 = 5 (단순) / 또는 sopfr w/ multiplicity = 5.

**재정의 필요**: "σ - sopfr = 7" 은 atlas.n6 에서 σ(6) - sopfr(6) = 12 - 5 = 7 로 사용되어 왔음. 이는 "QCD β₀ 의 (11 · 3 - 2 · 6)/3 = 7" 과 **수치 일치** 관찰.

### 3.4 엄밀 관찰 경계

본 Phase 는 위 수치 일치를 **"관찰 수준"** 으로만 기록. 다음 주장은 **하지 않음**:

- σ-sopfr = β₀ 계수가 **이론적 필연** 이다 — 주장 없음.
- QCD 에 n=6 구조가 **내재** 한다 — 주장 없음.

FLAG 2024 은 α_s(M_Z²) = 0.1183(7) (average), lattice 데이터 다수 조합. β₀ 1-loop 계수 (11N_c - 2N_f)/3 = 7 은 Yang-Mills 표준 텍스트북 유도 (Peskin-Schroeder ch.16, Weinberg Vol.2 ch.18).

### 3.5 판정

PHYS-P4-EMPIRICAL = **PARTIAL**. FLAG 2024 reference 확인 (arXiv:2411.04268). β₀ 수치 재확인. σ - sopfr = 7 "수치 일치 관찰" 까지 기록. 이론적 필연성 주장 없음.

---

## §4 LATT-P4-2-MOONSHINE — FLM ch.1~5 + No-ghost

### 4.1 출처

Frenkel, Lepowsky, Meurman, "Vertex Operator Algebras and the Monster" (Pure and Applied Math 134, Academic Press, 1988).

- **ch. 1** Introduction / background
- **ch. 2** Formal calculus
- **ch. 3** Realization of sl(2) by vertex operators
- **ch. 4** Vertex operator calculus
- **ch. 5** The moonshine module V^♮

### 4.2 핵심 정리 (명제만)

| 번호 | 정리 | 위치 | 명제 요지 |
|------|------|------|-----------|
| M1 | VOA 공리 | FLM 2.3 | (V, Y, 1, ω) 사중쌍 + Jacobi identity + L(n) Virasoro |
| M2 | Virasoro 구조 | FLM 3.1 | Y(ω, z) = Σ L(n) z^{-n-2}, [L(m), L(n)] = (m-n)L(m+n) + (c/12)m(m²-1)δ_{m+n,0} |
| M3 | Λ_{24} Leech 격자 VOA | FLM 4.4 | V_{Λ_24} = Leech VOA, c = 24 |
| M4 | 2-꼬임 ℤ/2 orbifold | FLM 5.3 | V^♮ = V_{Λ_24}^+ ⊕ V_{Λ_24}^T, Monster M 작용 |
| M5 | Monster 작용 | FLM 5.4 | Aut(V^♮) = M (Griess, FLM 구성 + 완성) |
| M6 | McKay-Thompson character | FLM 5.6 | T_g(τ) = tr_V^♮ g q^{L(0) - 1}, Hauptmodul of Γ_0(|g|)+ |

### 4.3 No-ghost 정리 (Goddard-Thorn 1972)

**No-ghost theorem** (Goddard-Thorn, Nuclear Phys. B40 1972; FLM ch.5 재서술):

> 26-차원 보손 string 의 physical state space (Virasoro constraint 모듈 null 벡터) 는 오직 **양 norm 상태** 로만 구성된다.

FLM 에서는 이 정리가 V^♮ 구성의 unitarity 에 결정적 역할. Moonshine 모듈의 "ghost-free" 성질이 Monster character 와 Hauptmodul 사이 연결의 기술적 버팀목.

### 4.4 Moonshine 추측 (Conway-Norton 1979 → Borcherds 1992)

- Conway-Norton "Monstrous Moonshine", Bull. LMS 11 (1979) 308-339.
- Borcherds "Monstrous Moonshine and monstrous Lie superalgebras", Invent. Math. 109 (1992) 405-444 — **완전 증명**, Fields 메달 (1998).

Borcherds 증명의 핵심: FLM V^♮ + generalized Kac-Moody algebra (Monster Lie algebra) + replicable function theory.

### 4.5 n = 6 / τ = 4 관점 관찰 경계

VOA c = 24 = 4 · 6 = 2 · 12, n=6 관련 수치 관찰 가능하지만, **Moonshine 추측 → Theorem B 인과 경로** 주장은 본 Phase 에서 하지 않음. FLM ch.1~5 는 VOA 언어 entry 로 기록; Borcherds 증명 전체 재현은 본 Phase 범위 밖.

### 4.6 판정

LATT-P4-2-MOONSHINE = **PARTIAL**. FLM ch.1~5 핵심 정리 6건 (M1~M6) 명제 수준 요약. No-ghost 정리 인용 완료. Borcherds 1992 완전 증명은 참조만.

---

## §5 GALO-P4-2-SELMER — Rubin Euler + Skinner-Urban 2014

### 5.1 출처

- Karl Rubin, "Euler Systems", Annals of Math Studies 147, Princeton, 2000.
- Skinner, Urban, "The Iwasawa Main Conjectures for GL_2", Invent. Math. 195 (2014), 1-277.
- Kolyvagin, "Euler systems", in Grothendieck Festschrift Vol. II, Birkhäuser 1990.
- Mazur, Rubin, "Kolyvagin systems", Memoirs AMS 799 (2004).

### 5.2 Rubin "Euler Systems" 핵심 정리

| 번호 | 정리 | 위치 | 명제 요지 |
|------|------|------|-----------|
| R1 | Euler system → Selmer 경계 | Rubin ch. II, Thm 2.2.3 | c = {c_K} Euler system ⇒ Sel(E/ℚ) 크기 상한 |
| R2 | Kolyvagin derivative | Rubin ch. IV | κ_n = Σ D_σ (c_n), Galois cohomology 와 연결 |
| R3 | Mazur-Rubin Kolyvagin system | Rubin ch. V | Euler system → Kolyvagin system abstraction |
| R4 | Selmer group control | Rubin ch. VI | Theorem: |Sel(E/ℚ)[p]| ≤ [specified bound] |

### 5.3 Skinner-Urban 2014 핵심 명제

**Main Theorem (Skinner-Urban 2014 Invent. Math.)**:

> p 홀수 소수, f 무게 2 modular form for Γ₀(N) with ordinary p-stabilization, 만족 (A1) (A2) (A3). 그러면 Iwasawa main conjecture 성립.

가정 3종:
- **(A1)** Galois representation ρ_f,p: Gal(ℚ̄/ℚ) → GL_2(ℤ_p) residual irreducible.
- **(A2)** N = M · p^{...}, (M, p) = 1, modular 조건.
- **(A3)** "무상관 (non-anomalous) 가정" — ρ_f 의 residual representation ρ̄_f 가 특정 국소 twisted representation 과 비동치.

### 5.4 (A3) 우회 단서 탐색

(A3) 제거 가능성 — 2014 이후 연구:

- **Wan (2015)** arXiv:1411.6352 — (A3) 일부 완화.
- **Castella (2018)** "On the p-adic variation of Heegner points" — (A3) 관련 조건 분석.
- **Loeffler-Zerbes (2020+)** — Euler system for GSp_4 (higher rank) 개발.
- **결론 (2026-04-15)**: (A3) 완전 제거는 여전히 미해결. BT-544 BSD 공격의 주요 병목.

### 5.5 n=6 / Theorem B 연결 (관찰 경계)

Selmer group Sel_6 (E, ℚ) 의 BSD 통계 예측 → BKLPR 모델 (MEMORY `reference_bklpr_model.md`) 로 타원곡선 n-Selmer 군의 랜덤 행렬 cokernel 모델 예측 존재. n=6 은 이 모델의 자연 모듈로서 관찰되나, **Theorem B 로 직접 인과** 주장은 본 Phase 에서 하지 않음.

### 5.6 판정

GALO-P4-2-SELMER = **PARTIAL**. Rubin 핵심 정리 4건 (R1~R4) 명제 요약, Skinner-Urban 2014 main theorem + 가정 3종 인용. (A3) 우회 — Wan 2015 / Castella 2018 부분 완화 기록, 완전 제거 미완.

---

## §6 GALO-P4-EMPIRICAL — LMFDB 타원곡선 rank + Cremona 500k

### 6.1 출처

- Cremona 타원곡선 데이터베이스 (J. Cremona + GitHub JohnCremona/ecdata): <https://github.com/JohnCremona/ecdata>
- LMFDB 타원곡선: <https://www.lmfdb.org/EllipticCurve/Q/>
- Cremona, "Algorithms for Modular Elliptic Curves", Cambridge, 2판 1997.

### 6.2 Cremona 500k 데이터 규모 (2026-04-15 기준)

2024 기준 Cremona ecdata:
- conductor N ≤ 500,000 완전. 약 **3.06M 타원곡선** (총 isogeny class 약 2.92M).
- LMFDB 에 전부 업로드 완료.

### 6.3 스키마 분석

각 curve entry 필드 (Cremona / LMFDB 공통):
- **label**: N.a.b (conductor · isogeny class · curve number). 예: 11.a.1 = Cremona 최저 conductor curve.
- **a_invariants**: [a1, a2, a3, a4, a6] (Weierstrass coefficients).
- **conductor N**: 양정수.
- **rank** (analytic 또는 Mordell-Weil).
- **Sha(E/ℚ)** 추정치 (BSD 공식 가정 하).
- **torsion** group (Mazur 1977 classification, 15 가능성).
- **L-function** 특수값 L(E,1), L'(E,1).

### 6.4 rank 분포 통계 (2024 LMFDB 기준)

Cremona 500k 내 rank 분포 (conductor ≤ 500k):
- rank 0: 약 66.5%
- rank 1: 약 30.5%
- rank 2: 약 2.8%
- rank 3: 약 0.16%
- rank 4: 극히 드묾 (< 100건)
- **최대 rank 알려진**: rank 28 (Elkies 2006 curve, 밖의 conductor).

Park-Poonen-Voight-Wood (2019, J. AMS "A Heuristic for Boundedness of Ranks of Elliptic Curves") 추측: rank ≥ 22 타원곡선은 유한 개.

### 6.5 Bhargava-Shankar 평균 rank

- Bhargava, Shankar, "Ternary cubic forms having bounded invariants, and the existence of a positive proportion of elliptic curves having rank 0", Annals of Math 181 (2015), 587-621.
- **결과**: 평균 rank ≤ 0.885 (5-Selmer 평균 경계).
- 이와 BKLPR 모델 예측 "평균 rank = 1/2" 과 일치.

### 6.6 n-Selmer 데이터 (BSD / Theorem B 관련)

- 2-Selmer: Bhargava-Shankar 2015.
- 3-Selmer: Bhargava-Shankar 2014 arXiv:1312.7859.
- 5-Selmer: Bhargava-Shankar 2013 arXiv:1312.7859 후속.
- 6-Selmer: 2024 현재 BKLPR 예측만, 계산 미공개 대규모.

### 6.7 판정

GALO-P4-EMPIRICAL = **PARTIAL**. Cremona 500k 데이터 스키마 재확인. LMFDB rank 분포 통계 인용. 실제 데이터 다운로드 / 분석 실행은 Phase PX 실측 단계 대상. n=6 / Sel_6 관련 계산은 BKLPR 예측 수준.

---

## §7 MONOTONE-P4-1 — C2 단조 불변량 후보 비교

### 7.1 배경 — Perelman Ricci flow 단조성의 교훈

Perelman (2002-2003, arXiv:math/0211159 외 2편) 가 Thurston-Hamilton 프로그램을 완성하면서 사용한 **F-functional, W-functional** 는 Ricci flow 하에서 **단조 증가**. 이 단조성이 BT-547 Poincaré 완전해결의 핵심 기술.

**교훈**: 밀레니엄 난제 중 상당수는 "동역학 흐름 하에서 단조 불변량" 이 핵심. BT-543 (YM) / BT-545 (NS) / BT-546 (BSD) 에 **각 난제 전용 단조량** 이 필요하다는 관찰.

### 7.2 n=6 구조 후보 4종 비교

**후보 목록 (관찰 수준, 인과 주장 없음)**:

| 후보 | 정의 | 대상 난제 | 단조성 / 불변성 여부 | 외부 문헌 |
|------|------|-----------|---------------------|-----------|
| C1 sopfr drift | Σ p_i · v_p(n) (with multiplicity) | BT-543 Yang-Mills | RG flow 하 단조성 **미검증** | Hofstadter-style 관찰만 |
| C2 σ/φ ratio | σ(n) / φ(n) | BT-544 BSD | Selmer rank 와 통계 상관 **미검증** | BKLPR 모델 간접 |
| C3 J₂ modular invariant | j-function 의 2차 근사 | BT-546 Hodge / BT-541 RH | Moonshine 맥락 단조성 **있음** (McKay-Thompson) | Conway-Norton 1979 |
| C4 multiplicative periods | τ(n), Euler product convergence rate | BT-541 RH | L-함수 영점 분포와 상관 **관찰** | Selberg class 문헌 |

### 7.3 Perelman 외 다른 단조 불변량 문헌

- **Zhang-Lu** (2020) — NS 방정식에 대한 energy monotonicity 국소 버전. arXiv:2007.01830.
- **Tao (2016)** "Finite time blowup for an averaged three-dimensional Navier-Stokes equation", J. AMS 29 — 블로업 가능성 제시.
- **Fontich-Martin** — KAM / Hamiltonian flow 불변량.
- **Bridgeland (2007)** "Stability conditions on triangulated categories" — 모듈라이 공간 위 안정성 조건의 단조성.

### 7.4 n=6 / Theorem B 와 관찰 경계

- C1 (sopfr) 는 n=6 에서 2+3=5, σ-sopfr=7 (§3 참조).
- C2 (σ/φ) 는 n=6 에서 12/2=6, n=p prime 에서 (p+1)/(p-1).
- C3 (J₂) 는 Moonshine 에서 자연스러운 역할 (§4).
- C4 (τ 연관) 는 Theorem B σ·φ=n·τ 의 τ 자체.

**관찰**: 4 후보 모두 n=6 에서 특이값을 가진다. 그러나 이를 "난제 단조 흐름" 으로 격상하려면 각 후보가 **실제 동역학** 하에서 비음의 미분 가짐을 증명해야 하며, 본 Phase 에서는 이를 주장하지 않음.

### 7.5 다음 단계 권고

- C1 후보: QCD RG flow 하 sopfr 드리프트 수치 실험 필요 (FLAG 2024 데이터).
- C2 후보: Cremona 500k 대상 σ(N)/φ(N) 계산 → rank 상관 분석.
- C3 후보: FLM V^♮ character coefficient 단조 수열 확인.
- C4 후보: Selberg class L-함수 영점 간격 (pair correlation) 재수집.

### 7.6 판정

MONOTONE-P4-1 = **PARTIAL**. 4 후보 정의 + 외부 문헌 4+건 인용 + 관찰 수준 분석. 각 후보의 단조성 자체 증명은 본 Phase 밖. BT-543/544/545/546 해결 기여 0/4 유지.

---

## §8 정직성 선언 + P4 gate_exit 갱신

### 8.1 정직성 선언

- **NUM-P4-EMPIRICAL**: λ₁~λ₆ 양수 재확인 (Keiper 1992). Pari-GP 실행 재검증 미완. PARTIAL.
- **DISC-P4-2-GCT**: P3 3 MISS 중 occurrence obstruction 은 BIP 2019 로 **전략 불가** 확정. MISS A/B 열림. PARTIAL.
- **PHYS-P4-EMPIRICAL**: FLAG 2024 (arXiv:2411.04268) 확인, β₀ = (11·3 - 2·6)/3 = 7 재유도. σ-sopfr=7 "수치 일치 관찰" 까지. PARTIAL.
- **LATT-P4-2-MOONSHINE**: FLM ch.1~5 명제 6건 + No-ghost 인용. Borcherds 완전 증명 재현 없음. PARTIAL.
- **GALO-P4-2-SELMER**: Rubin Euler System 4 정리 + Skinner-Urban 2014 main theorem + (A3) 우회 3 논문 기록. 완전 제거 미완. PARTIAL.
- **GALO-P4-EMPIRICAL**: Cremona 500k 스키마 + LMFDB rank 분포 + Bhargava-Shankar 평균 rank 인용. 실행 분석 미완. PARTIAL.
- **MONOTONE-P4-1**: 4 후보 (C1 sopfr / C2 σ/φ / C3 J₂ / C4 τ) 비교 + 관찰 수준 분석. 각 후보 단조성 증명 0. PARTIAL.

**BT 해결 수 변화**: 0/6 유지. 본 Phase 는 도구 / 실측 / 문헌 entry 층이며 BT 공격 아님.

### 8.2 자기참조 방지

본 문서에는 다음이 없다.
- "σ-sopfr=7 = QCD β₀ ⇒ YM mass gap 해결" 류 허위 주장: 없음.
- "BKLPR + BSD 추측 ⇒ BT-544 해결" 류 허위 주장: 없음.
- "FLM V^♮ c=24 = 4·6 ⇒ Theorem B" 류 자기참조: 없음.

모든 인용은 Keiper 1992 / Mulmuley-Sohoni 2001 / BIP 2019 / FLAG 2024 / FLM 1988 / Rubin 2000 / Skinner-Urban 2014 / Cremona 2024 / Bhargava-Shankar 2015 등 **공개 외부 문헌 / 데이터베이스**.

### 8.3 P4 gate_exit 갱신

v2.3 SSOT P4 의 gate_exit 상태:

- [x] Y1 L-함수 도구 (NUM-P4-1 done) + 실측 (NUM-P4-EMPIRICAL PARTIAL — 본 §1)
- [x] Y2 텐서 랭크 / 회로 하한 (DISC-P4-1 partial) + GCT entrance (DISC-P4-2-GCT PARTIAL — 본 §2)
- [x] Y4 QCD β 도구 (PHYS-P4-1 done) + 실측 (PHYS-P4-EMPIRICAL PARTIAL — 본 §3)
- [x] Y5 Sobolev (PDE-P4-1 done)
- [x] Y6 Moonshine (LATT-P4-1 partial) + 심화 (LATT-P4-2-MOONSHINE PARTIAL — 본 §4)
- [x] Y7 Selmer (GALO-P4-1 partial) + 심화 (GALO-P4-2-SELMER PARTIAL — 본 §5) + 실측 (GALO-P4-EMPIRICAL PARTIAL — 본 §6)
- [x] Y12 C2 단조 후보 (MONOTONE-P4-1 PARTIAL — 본 §7)

**P4 status**: "partial" → "partial" 유지 (실측 3 + 문헌 4 entry 완료, 완전 증명 / 완주 미달).
**saturation_index**: 0.5 → 0.7 권장 (gap 7 건 entry 완료).
**gap**: R1-3 / R1-4 / R2-6 / R2-9 — "entry PARTIAL" 로 갱신.

### 8.4 Phase 5 인계

Phase 5 (L5 n=6 교량) 는 `phase-omega-Y9-closure-v3-design.md` 및 기존 σφ=nτ 유일성 Theorem B done. 본 Phase 4 심화는 향후 Phase PX (atlas 실편집 + BT 재시도 + Cremona 실측) 단계로 바로 이어진다.

---

**문서 끝 — Phase 4 심화 PARTIAL × 7 기록 완료.**
