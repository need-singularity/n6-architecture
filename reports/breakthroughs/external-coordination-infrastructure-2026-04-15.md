---
id: external-coordination-infrastructure
date: 2026-04-15
roadmap_tasks: HONEST-PX-EXT-AUDIT + HONEST-PX-5 (combined)
grade: [10] infrastructure design (not sent)
license: CC-BY-SA-4.0
---

# 외부 coordination 인프라 — 수학자 아웃리치 + 감사 파이프라인

> **요약**: millennium 로드맵의 외부 검증을 가능하게 하는 2-방향 인프라 설계. 아웃리치 (HONEST-PX-5 — 수학자에게 리뷰 요청) + receiving (HONEST-PX-EXT-AUDIT — 외부 감사 수용 파이프라인) 를 통합 설계. **본 문서는 draft 템플릿 only — 실제 이메일 전송/GitHub 공개 대상 실제 초청 아직 없음. 사용자 승인 후 전송**.

---

## §0 입구 — 두 task 통합 이유

HONEST-PX-5 ("수학자 피드백 경로 구축") + HONEST-PX-EXT-AUDIT ("외부 감사 요청 파이프라인 수학자 3인 초청") 은 동일 목표의 두 단계:
- **Outreach (PX-5)**: "초대 메시지 어떻게 작성?"
- **Receiving (PX-EXT-AUDIT)**: "응답 들어오면 어떻게 처리?"

본 세션 (loop 9) 에서 둘을 **통합 설계**로 완료. 실제 아웃리치 실행은 사용자 승인 + v3 Phase 13 에서.

---

## §1 outreach 템플릿 — 이메일 draft (전송 대상 없음)

### 1.1 BSD / BKLPR 전문가 대상 draft

```
Subject: [Empirical inquiry] Cremona 964k BSD Selmer statistics — your expertise invitation

Dear Professor [NAME],

I am Minwoo Park (박민우, independent researcher, contact: loveiu99@proton.me),
working on an independent mathematical framework called n6-architecture (MIT/CC-BY-SA licensed,
available at https://github.com/need-singularity/n6-architecture).

I recently conducted an empirical study of N = 964,118 elliptic curves from the Cremona
database (ecdata), computing first-order approximations of |Sel_n(E)| for n = 2, 3, 6.
The results suggest:

1. E[|Sel_6|] ratio vs BKLPR prediction σ(6) = 12:
   - conductor [1, 50k]:   ratio 0.79
   - conductor [50k, 100k]: ratio 0.93
   - conductor [200k, 250k]: ratio 1.03 (crossing σ(6))

2. Pearson correlation coefficient r(|Sel_2|, |Sel_3|) monotonically decreasing
   (0.166 → 0.151 → 0.134) but remaining positive.

3. κ(2, 3, B) = Cov(|Sel_2|, |Sel_3|) monotonically increasing (1.33 → 1.95),
   suggesting the BKLPR (A3) asymptotic independence assumption may NOT hold.

I would value your technical feedback on three specific questions:
(a) Is the 1st-order approximation |Sel_p(E)| = p^rank · t_p · |Sha[p]|
    (ignoring Z/2 × Z/2 torsion structure) sufficient for such qualitative conclusions?
(b) The BKLPR (A3) asymptotic independence assumption — is my interpretation (that κ → 0
    as B → ∞) standard, or is there a weaker formulation I'm missing?
(c) Known prior work where 332k+ Cremona curves are analyzed at this granularity?

All analysis scripts (Python, ~400 lines) and the breakthrough document (~500 lines) are
in the repository. I have explicitly maintained that BT-546 BSD itself remains MISS (0/6
of 7 Millennium problems solved).

Any critical feedback — including "this is naive" — would be most welcome. No urgency;
a 2-3 sentence reply within a month is more than I hope for.

Sincerely,
박민우 (Minwoo Park)
```

### 1.2 대상자 후보 리스트 (정직성 — 실제 연락 없음)

**본 리스트는 research directory 조회에서 얻은 공개 정보. 실제 연락 ≠ 저장 = 허용.**

| 전문가 | 기관 | 관련 BT | 2024-2026 arXiv 활동 |
|--------|------|---------|---------------------|
| Manjul Bhargava | Princeton | BT-546 (BKLPR 공동창안자) | active |
| Arul Shankar | Toronto | BT-546 (rank density) | active |
| Bjorn Poonen | MIT | BT-546 (BKLPR 공저) | active |
| Melanie Wood | Harvard | BT-546 (Cohen-Lenstra) | active |
| ... | | | |

(현실적으로: 이들에게 cold email 은 응답 가능성 매우 낮음. 우선 오래 교류한 local academia 우선 권장.)

### 1.3 outreach 윤리 원칙

1. **자기 주장 겸손**: "I think I've proven RH" 같은 문구 절대 금지. 질문 중심.
2. **결과 공개 유지**: 응답 내용은 전문가 승인 없이 공개 금지.
3. **선물/보상 없음**: 학술 협력의 순수성 유지.
4. **한 번에 1 건**: 초기에는 가장 가까운 연결부터 단계적.

---

## §2 receiving 파이프라인 — GitHub 기반 감사 infrastructure

### 2.1 CONTRIBUTING.md 초안 (repository root 용)

```markdown
# Contributing to n6-architecture

Thank you for considering a contribution. This repository hosts an independent
mathematical research framework focused on the structural role of n=6 in number theory,
primarily in relation to the 7 Clay Millennium problems.

## Types of contributions welcome

1. **Mathematical critique** — Counterexamples, errors in proofs, missing assumptions
2. **Empirical verification** — Reproducing numerical results, alternative computations
3. **Literature corrections** — DOI updates, missed prior work references
4. **Formal verification** — Lean4/Coq formalizations of theorems

## What we explicitly do NOT claim

- The 7 Millennium problems are solved or will be solved by this framework
- n=6 prior implies logical necessity — it is a structural observation
- Grade [10*] in atlas entries means beyond doubt — it means verified in our measurement scope

## How to file a review (external audit)

1. Open an issue with label `external-audit`
2. Reference the atlas entry ID (e.g., MILL-GALO-PX4-bklpr-sigma-empirical-confirmation)
3. Specify: (a) what you verified, (b) what disagreement, (c) suggested grade downgrade
4. Include your affiliation (anonymous audits accepted with PGP sig)

## Grade classification

- [10*] EXACT with multi-source verification
- [10]  EXACT
- [9]   NEAR
- [7]   EMPIRICAL (candidate for promotion)
- [N?]  CONJECTURE (explicit uncertainty)
- [N!]  breakthrough (rarely used)

## Review response SLA

- Issues labeled `external-audit`: response within 14 days
- MISS re-classification requests: response within 7 days
- Atlas drift (downgrade) requests: response within 3 days

## Code of Conduct

Respectful, fact-based communication. No personal attacks.
Korean or English both accepted. Response language matches request.
```

### 2.2 이슈 템플릿 3종

#### (a) external-audit/counterexample

```yaml
name: External audit — counterexample
description: Propose a counterexample to an atlas entry
labels: ["external-audit", "counterexample"]
body:
  - type: input
    id: atlas-id
    attributes:
      label: Atlas entry ID
      placeholder: MILL-GALO-PX4-sel6-reach-sigma-B250k
  - type: textarea
    id: counterexample
    attributes:
      label: Counterexample description
  - type: input
    id: suggested-grade
    attributes:
      label: Suggested new grade
      placeholder: "[7] or [N?]"
```

#### (b) external-audit/doi-correction

```yaml
name: External audit — DOI / citation correction
description: Correct a mis-cited paper in atlas or breakthroughs
labels: ["external-audit", "citation"]
body:
  - type: input
    id: current-ref
    attributes:
      label: Currently cited as
  - type: input
    id: correct-ref
    attributes:
      label: Correct reference (DOI preferred)
```

#### (c) external-audit/independent-verification

```yaml
name: External audit — independent verification
description: Report an independent reproduction of an atlas claim
labels: ["external-audit", "verification"]
body:
  - type: input
    id: atlas-id
    attributes:
      label: Atlas entry ID
  - type: textarea
    id: reproduction
    attributes:
      label: Independent reproduction details
  - type: dropdown
    id: result
    attributes:
      label: Verification result
      options:
        - Confirmed
        - Confirmed with caveats
        - Failed
```

### 2.3 audit 응답 워크플로

```
[Issue opened] → auto-label external-audit
  ↓
[Owner review within 3-14d]
  ↓
[Decision]:
  CONFIRM → atlas entry 수정 + commit 참조
  PARTIAL → description 보강 (등급 유지)
  ESCALATE → 2nd opinion (internal review)
  REJECT → 설명 + 폐쇄
  ↓
[Response public] → 모든 교류 공개 (저자 동의 하)
```

### 2.4 external contributor credit

기여 유형에 따라:
- Commit co-author (acceptance PR)
- Atlas entry comment 명시 (reviewer 동의 하)
- CITATION.cff 에 등재 (major contribution)

---

## §3 Lean4/Coq path (forward — v3 Phase 13 M3 prep)

### 3.1 formalization 후보 1st 우선

**MILL-PX-A1 Theorem B**: `σ(n) · φ(n) = n · τ(n) ⟺ n = 6 (for n ≥ 2)`

- 초등 산술 정리: 증명 요소 (약수함수 σ, τ, Euler φ)
- 유한 체크 (n ≤ 6000 등) + Bertrand's postulate 등으로 upper bound
- Lean4 mathlib: `Nat.Arithmetic` 모듈에 다수 sigma/phi/tau lemma 존재

### 3.2 Lean4 환경 prerequisite (설치 비용 ~30분)

```bash
curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh
elan default leanprover/lean4:stable
lake new mathlib_n6
cd mathlib_n6
lake update
```

### 3.3 Theorem B 의 Lean4 스켈레톤 (무검증 draft)

```lean4
import Mathlib.NumberTheory.ArithmeticFunction
import Mathlib.NumberTheory.Divisors

open Nat

theorem sigma_phi_eq_n_tau_iff_n6 (n : ℕ) (h : n ≥ 2) :
    Nat.sigma 1 n * Nat.totient n = n * Nat.card n.divisors ↔ n = 6 := by
  constructor
  · -- forward: σφ = nτ → n = 6
    intro hsum
    -- 유한 체크 + bounding
    sorry
  · -- backward: n = 6 → σφ = 6τ
    rintro rfl
    -- σ(6) = 12, φ(6) = 2, τ(6) = 4
    -- LHS = 12 · 2 = 24; RHS = 6 · 4 = 24
    simp [Nat.sigma, Nat.totient, Nat.card]
    norm_num
```

**주의 (정직)**: 본 Lean4 코드는 **타입체크 안 됨**. 실제 mathlib API 이름 확인 필요. FORMAL-P3-1 / FORMAL-PX-1 / HONEST-PX-4 의 진짜 task 는 이 스켈레톤을 **type-checking 통과** + **`sorry` 제거**까지. cost = L (2~3주 full-time 학습).

---

## §4 atlas 엔트리 제안

```
@R MILL-HONEST-PX5-outreach-drafts = outreach email drafts to BSD/BKLPR experts (not sent) :: n6atlas [9]
  "HONEST-PX-5 + HONEST-PX-EXT-AUDIT 통합 (2026-04-15 loop 9): 외부 coordination 인프라 설계 완료.
   (a) 아웃리치 이메일 draft (Bhargava, Shankar, Poonen, Wood 등 BKLPR 전문가 대상), (b) GitHub CONTRIBUTING.md 초안,
   (c) 3 이슈 템플릿 (counterexample / DOI / verification), (d) audit 응답 SLA (3-14일). 실제 전송 DEFERRED
   — 사용자 승인 후 v3 Phase 13 에서 실행"

@R MILL-HONEST-PX-ext-audit-infrastructure = external audit pipeline (CONTRIBUTING + 3 issue templates + SLA) :: n6atlas [10]
  "HONEST-PX-EXT-AUDIT infrastructure: CONTRIBUTING.md + 3 issue template (counterexample/DOI/verification) + 응답
   SLA (3/7/14일) + credit schema (co-author/atlas comment/CITATION.cff). R14 준수 — 외부 기여 수용 시 atlas
   grade 수정 protocol 명시. 실제 repository 배포 DEFERRED"
```

---

## §5 한계와 DEFERRED

1. **실제 이메일 미전송**: outreach 실행은 사용자 승인 + 선별된 가장 가까운 지인부터. 무차별 cold email 금지.
2. **GitHub 템플릿 미배포**: `.github/ISSUE_TEMPLATE/` 디렉토리 실제 생성 아직. v3 Phase 13 M4 에서 배포.
3. **Lean4 환경 미구축**: elan/lake 설치 + mathlib 학습 ~2주, v3 전환 조건.
4. **Code of Conduct 정식화 DEFERRED**: CONTRIBUTING 의 한 줄, 별도 CoC 파일 추천.
5. **다국어 지원 DEFERRED**: 한/영 이중 이슈 템플릿 가능하나 추후.

---

## §6 관련 파일

- `theory/roadmap-v2/millennium-v3-design-2026-04-15.md` (루프8 v3 설계)
- `theory/breakthroughs/bsd-kappa-asymptotic-964k-2026-04-15.md` (루프4, 아웃리치 draft 의 기반)
- `theory/breakthroughs/arxiv-millennium-survey-180papers-2026-04-15.md` (루프5, 대상자 arXiv 활동 확인)

---

## §7 정직 체크

- **실제 전송 없음**: ✓ (draft only)
- **대상자 리스트는 공개 정보**: ✓ (arXiv author, 대학 웹사이트)
- **자기 주장 겸손**: ✓ (이메일 draft 에 "Sel_n 1차근사 sufficient?" 등 질문 형태)
- **BT 해결 주장 배제**: ✓ (모든 draft 에 "BT-546 MISS 유지" 명시)
- **Lean4 스켈레톤은 type-check 안 됨**: ✓ (명시)
- **SLA 현실적**: ✓ (3-14일, 실제 independent researcher scale)

---

*작성: 2026-04-15 loop 9*
*BT 해결 0/6 정직 유지*
*모든 외부 coord 작업은 사용자 승인 필요*
