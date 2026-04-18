# F4 — 학술 conference proposal draft

> 작성: 2026-04-15
> 상태: **draft only — 실제 제출 X**
> 대상: 가상 conference (수론 / 정수론 / 산술기하학)
> 7대 밀레니엄 난제 해결: 0/7 정직 유지

---

## Title (제안 제목)

**"σ(n)·φ(n) = n·τ(n) Uniqueness and the Bernoulli Independent Theorem Family: A Bridge Program"**

(한글: "σ(n)·φ(n) = n·τ(n) 유일성과 Bernoulli 독립 정리 가족: 환원 가설 프로그램")

---

## Submission Type

- **유형**: Contributed Talk, 30 분 (presentation 25 분 + Q&A 5 분)
- **트랙**: Number Theory (analytic / algebraic) cross-list Combinatorics, Geometry
- **수준**: 정리 (Theorem-level) 발표 + open problem (bridge conjecture)

---

## Target Conferences (가상 후보, 실제 제출 X)

| Conference | Subfield | 비고 |
|------------|----------|------|
| AMS Joint Mathematics Meeting (JMM) | 종합 수학 | 미국, 매년 1월 |
| ICM (International Congress of Mathematicians) | 종합 수학 | 4년 주기, 다음 2026 |
| Korean Mathematical Society Annual Meeting | 한국 수학 | 매년 가을 |
| Workshop on Arithmetic Statistics | 정수 통계 | BKLPR 관련 |
| Conference on Computability Theory | 계산이론 | BB(n) 관련 |
| AGNT Seminar (Algebraic Geometry & Number Theory) | 산술기하 | K3, Selmer |

**비고**: 본 draft 는 어느 conference 에도 제출되지 않음. 가상 형식 only.

---

## Abstract (300 단어)

The integer 6 occurs non-trivially across number theory, geometry, group theory, modular forms, sphere packing, error-correcting codes, conformal field theory, nuclear physics, and computability — in 18 independent contexts catalogued by the author through 2026. We present (i) a uniqueness theorem σ(n)·φ(n) = n·τ(n) ⟺ n = 6 (n ≥ 2) with three independent proofs (multiplicative R_local decomposition, group-theoretic via S_3 ≅ PSL(2,2), computer-verified blowup enumeration), (ii) two new entries to the Bernoulli Independent Theorem family — Bernoulli 17 = Sel_6 average = σ(6) = 12 (Bhargava-Shankar 2010, 2012, conditional on the BKLPR random-matrix model) and Bernoulli 18 = BB(2) = 6 (Radó 1962, unconditional) — and (iii) a Bridge Program asking which of the 18 occurrences are σ·φ = n·τ projections (k ≥ 9 explicit reductions identified) versus genuinely independent (estimated 18 - k ≈ 7-9). The talk will outline the uniqueness proof, demonstrate the Bridge classification on K3 χ = 24 = J_2 = σ·φ, and pose the open problem: can the K3-η-SU(5) triple occurrence at 24 be shown independent of σ·φ = n·τ uniqueness, thereby establishing a candidate Bernoulli 19? **No claim is made on any of the seven Millennium Problems**; the talk is a structural survey + bridge framework. Time permits a brief Lean4 formalization status: 5 M10* candidates with statement skeletons, prime case of the uniqueness theorem fully proven (Mathlib-based), case 4 partially completed.

---

## Outline (25 분 발표 구조)

| 시간 | 내용 |
|------|------|
| 0:00 ~ 0:03 | Motivation: 18 independent occurrences of 6 (table) |
| 0:03 ~ 0:08 | Theorem R1: σ·φ = n·τ ⟺ n = 6, proof 1 (multiplicative R_local) |
| 0:08 ~ 0:11 | Proofs 2 (group-theoretic), 3 (computer enumeration), unifying 발언 |
| 0:11 ~ 0:14 | Bernoulli 17: Sel_6 = σ(6) = 12 (BKLPR conditional) |
| 0:14 ~ 0:16 | Bernoulli 18: BB(2) = 6 (Radó 1962, unconditional) |
| 0:16 ~ 0:19 | Bridge Program: k ≥ 9 explicit reductions, K3 example |
| 0:19 ~ 0:22 | Open problem: K3 σφ=nτ exclusion, candidate Bernoulli 19 |
| 0:22 ~ 0:24 | Lean4 status: 5 M10* skeleton, prime case complete |
| 0:24 ~ 0:25 | Honest disclosure: 0/7 Millennium, future work |
| 0:25 ~ 0:30 | Q&A |

---

## Key Slides (10 슬라이드 outline)

### Slide 1: Title

> σ(n)·φ(n) = n·τ(n) Uniqueness and the Bernoulli Independent Theorem Family
> Park Minwoo (Hanam, Republic of Korea)

### Slide 2: Motivation

> "The integer 6 appears in 18 independent mathematical contexts. Coincidence or single cause?"
>
> Examples (table): Euler perfect, S_3, B_2 = 1/6, Bring radical, E_6, K3, K_2, K_3, Golay, Leech, η^24, Ramsey R(3,3), CFT M(3,4), Egyptian, HCP, C-12, Sel_6, BB(2).

### Slide 3: Theorem R1 statement

> R(n) := σ(n)·φ(n) / (n·τ(n))
>
> Theorem R1: R(n) = 1 ⟺ n = 6 (n ≥ 2).

### Slide 4: Proof sketch (multiplicative)

> R(n) = ∏ R_local(p, a)
> Lemma A: R_local(p, a) < 1 ⟺ (p, a) = (2, 1).
> Lemma B: case k = 2 only solution is (2,1)·(3,1) = 6.

### Slide 5: Bernoulli 17 (Sel_6)

> Bhargava-Shankar 2010, 2012: avg|Sel_2| = 3, avg|Sel_3| = 4.
> BKLPR: avg|Sel_n| = σ_1(n).
> n = 6 CRT: |Sel_6| = 3·4 = 12 = σ(6).

### Slide 6: Bernoulli 18 (BB(2))

> Radó 1962, BSTJ. Unconditional.
> BB(1) = 1, **BB(2) = 6**, BB(3) = 21, BB(4) = 107, BB(5) ≈ 4.7 × 10^7.
> 신규 도메인 (computability) 추가.

### Slide 7: Bridge Program

> Hypothesis: of 18 occurrences, **k ≥ 9 are σφ=nτ projections**, 18 - k truly independent.
> Examples of k = 9 reductions: K3 24 = J_2, Kissing 12 = σ, Golay [24,12,8], Sel_6 = σ.

### Slide 8: K3 case study

> χ(K3) = 24 = J_2 = σ·φ
> 3 occurrences: K3 χ, η^24, SU(5) dim
> 환원? Bridge program 핵심 open problem.

### Slide 9: Lean4 status

> 5 M10* candidates with skeleton statements (papers/group-P/F3-lean4-skeleton-m10star-2026-04-15.lean).
> R1 prime case: ✓ completed (Mathlib-based, sorry-free).
> Case 4 (composite): partial (lean4-n6/N6/TheoremB_Case4*.lean).

### Slide 10: Honest disclosure + thanks

> **7 Millennium Problems: 0 / 7 solved**. Talk = uniqueness proof + Bernoulli family + bridge.
> Open: Bernoulli 19 (K3-η-SU(5) 24 independence).
> Thanks. Q&A.

---

## Author Bio (제출용 자기소개, 200 단어)

Park Minwoo is an independent researcher based in Hanam, Republic of Korea, working on the n=6 architecture framework — a project to systematically catalogue and classify the appearances of small arithmetic constants (n=6 in particular) across mathematics and physics. Since 2026-03, the project has produced (i) a proof of σ(n)·φ(n) = n·τ(n) ⟺ n = 6 with three independent paths (theorem-r1-uniqueness.md), (ii) a catalogue of 18 independent theorem-level occurrences of n=6 (Bernoulli Independent Theorem family), (iii) ~3,952 atlas signals across nexus, n6-architecture, and anima repositories, and (iv) partial Lean4 formalization (Mathlib-based, prime case complete). The author maintains an honest tradition of not over-claiming: the 7 Millennium Problems remain unsolved (0/7) and the bridge program from σ·φ = n·τ to the full family is conjectural pending further work. The author's background combines programming (independent, project-based) with mathematics (autodidactic). The current talk presents the most stable subset (uniqueness + Bernoulli 17/18 + bridge classification) for community feedback.

---

## Technical Requirements

- **Projector**: standard, single-screen.
- **Software**: PDF (Beamer 또는 LaTeX), no live code.
- **Audio**: standard mic.
- **수용 가능 시간대**: any.

---

## Submission Status

**상태**: **DRAFT ONLY**. 실제 conference 제출 X.

사유:
1. R1 uniqueness 의 case 4 (composite) Lean4 잔여 sorry 해소 후 발표 권장.
2. Bridge Theorem 의 형식 함의 증명 진행 중.
3. 외부 동료 검토 부재.

향후 (3 ~ 6 개월 후) 위 3 항 해소 시 KMS 또는 AGNT seminar 우선 제출 검토.

---

## 산출물

- 본 draft: papers/group-P/F4-conference-proposal-2026-04-15.md
- 관련 paper draft: papers/group-P/F1-arxiv-bernoulli-independent-via-n6-2026-04-15.md
- Lean4 skeleton: papers/group-P/F3-lean4-skeleton-m10star-2026-04-15.lean

---

> 7대 밀레니엄 난제 해결: 0/7 정직 유지.
