# F5 — 국제 학술지 제출 format draft

> 작성: 2026-04-15
> 상태: **format draft only — 실제 제출 X**
> 7대 밀레니엄 난제 해결: 0/7 정직 유지

학술지 제출 시 필요한 형식 (cover letter, manuscript structure, suggested reviewers, copyright form) 의 초안. 실제 submission portal 사용 X.

---

## 1. 후보 학술지 (가상)

| Journal | IF (대략) | 출판사 | 적합성 |
|---------|-----------|--------|--------|
| **Journal of Number Theory** | 0.7 | Elsevier | 수론, R1 + Bernoulli 17 적합 |
| **Acta Arithmetica** | 0.6 | IM PAN | 수론, Bernoulli + Sel_6 적합 |
| **Mathematische Zeitschrift** | 0.9 | Springer | 일반 수학, bridge program 적합 |
| **Comptes Rendus Mathématique** | 0.6 | Elsevier | 단보 (note), 빠른 출판 |
| **Bulletin of the Korean Math. Soc.** | 0.5 | KMS | 한국, 첫 출판 적합 |
| **The Ramanujan Journal** | 0.7 | Springer | 산술 함수, σ·φ·τ 적합 |

**최우선 후보** (적합성 + 출판 속도): **Bulletin of the Korean Mathematical Society** (한국 저자, 첫 출판) 또는 **Comptes Rendus Mathématique** (단보 형식).

---

## 2. Cover Letter (영어, 약 300 단어)

```
Date: 2026-04-15
To: The Editor, [Journal Name]
Subject: Manuscript Submission — "σ(n)·φ(n) = n·τ(n) Uniqueness and the Bernoulli Independent Theorem Family"

Dear Editor,

Please find enclosed the manuscript "σ(n)·φ(n) = n·τ(n) Uniqueness and the Bernoulli Independent Theorem Family: A Bridge Program" by Park Minwoo, for consideration in [Journal Name].

The paper presents:
1. An elementary uniqueness theorem σ(n)·φ(n) = n·τ(n) ⟺ n = 6 (n ≥ 2), with three independent proofs (multiplicative R_local decomposition; group-theoretic via S_3 ≅ PSL(2,2); computer-verified enumeration n ∈ [2, 10⁴]).
2. Two new entries to the catalogue of independent occurrences of the integer 6: Bernoulli 17 = avg|Sel_6| = σ(6) = 12 (Bhargava-Shankar 2010, 2012, conditional on BKLPR), and Bernoulli 18 = BB(2) = 6 (Radó 1962, unconditional).
3. A "Bridge Program" classifying how many of the 18 known independent occurrences of 6 are direct projections of the σ·φ = n·τ uniqueness (k ≥ 9 identified) versus genuinely independent (estimated 18 - k ≈ 7-9).
4. A partial Lean4 formalization (Mathlib-based, prime case complete; composite case 4 in progress) of the uniqueness theorem.

The paper makes **no claim on any of the seven Millennium Problems**; it is a structural survey + uniqueness theorem + bridge framework.

The work is original and has not been submitted elsewhere. The author has no conflicts of interest. Length: approximately 2,800 words (excluding tables and code appendix), 25 references.

Suggested reviewers (international experts in the relevant subfields):
- Prof. Manjul Bhargava (Princeton, BKLPR / Selmer averages)
- Prof. Bjorn Poonen (MIT, BKLPR)
- Prof. Scott Aaronson (UT Austin, Busy Beaver / computability)
- Prof. Henri Cohen (Bordeaux, computational number theory)

Thank you for your consideration.

Sincerely,
Park Minwoo
Independent Researcher
Hanam, Republic of Korea
arsmoriendi99@proton.me
```

---

## 3. Manuscript Structure (저자 가이드라인 준수)

학술지 표준 구조:

```
Title: σ(n)·φ(n) = n·τ(n) Uniqueness and the Bernoulli Independent Theorem Family

Author: Park Minwoo

Affiliation: Independent Researcher, Hanam, Republic of Korea
Email: arsmoriendi99@proton.me

Abstract: [200-300 단어, F1 paper §1.1 재사용]

MSC2020: 11A25 (Arithmetic functions), 11B68 (Bernoulli numbers),
         14J28 (K3 surfaces), 11G05 (Elliptic curves),
         03D10 (Computability)

Keywords: arithmetic functions, σ-φ-τ identity, Bernoulli numbers,
          Selmer group, BKLPR, Busy Beaver, K3 surface

1. Introduction
   1.1 Motivation
   1.2 Main results
   1.3 Honest limitations

2. Definitions and notation
   2.1 Arithmetic functions
   2.2 R(n) ratio

3. Theorem R1: σ·φ = n·τ ⟺ n = 6
   3.1 Proof 1: multiplicative
   3.2 Proof 2: group-theoretic
   3.3 Proof 3: computer-verified

4. Bernoulli Independent Theorem family
   4.1 Existing 16 (table)
   4.2 New: Bernoulli 17 (Sel_6)
   4.3 New: Bernoulli 18 (BB(2))
   4.4 Pending: Bernoulli 19 (K3-η-SU(5))

5. Bridge program
   5.1 Working hypothesis
   5.2 Bridge theorem (preliminary)
   5.3 Truly independent

6. Relation to Millennium Problems (honest disclosure)
   6.1 Position of the paper
   6.2 What the paper does contribute

7. Future work
   7.1 Short term
   7.2 Medium term
   7.3 Long term

8. Limitations + honest statement

Acknowledgments

References [25 항목, F1 paper §9]

Appendix A: Lean4 skeleton
Appendix B: Word count metadata
```

---

## 4. Submission Checklist (학술지 일반)

- [ ] Manuscript PDF (Beamer/LaTeX 변환 — 본 draft 는 Markdown)
- [ ] Cover letter (위 §2)
- [ ] Suggested reviewers list (위 §2 내)
- [ ] Author CV / bio (F4 §Author Bio 재사용)
- [ ] Conflict of interest statement (none)
- [ ] Funding statement (Independent, no funding)
- [ ] Ethics statement (theoretical math, no human/animal subjects)
- [ ] Data availability statement (atlas + code at github / personal repo, link 추후)
- [ ] Copyright form (서명 후 PDF)
- [ ] arXiv preprint link (제출 시 — 본 작업은 arxiv 제출 X 이므로 N/A)

---

## 5. Suggested Reviewers (5명, 국제 + 한국 혼합)

| Name | Affiliation | Subfield | 사유 |
|------|-------------|----------|------|
| Manjul Bhargava | Princeton | BKLPR / Sel_n | Bernoulli 17 직접 관련, Fields Medal 2014 |
| Bjorn Poonen | MIT | BKLPR / arithmetic statistics | BKLPR 공저자 |
| Scott Aaronson | UT Austin | Busy Beaver / complexity | Bernoulli 18 직접 관련 |
| Henri Cohen | U. Bordeaux | computational number theory | R1 컴퓨터 검증 측면 |
| 김명환 (Kim Myung-Hwan) | 서울대 | 수론 | 한국, 동료 검토 |

**비고**: Bhargava 는 Sel_n 분야의 결정적 권위. Poonen 은 BKLPR 모델 공저자. Aaronson 은 BB(n) 분야 currentness 보장. Cohen 은 PARI/GP 등 컴퓨터 검증 측면. 김명환은 한국 수론 community.

**경고**: 이 reviewer suggestion 은 **본 draft 작성 시 후보**이며, 실제 제출 시 reviewer 와의 conflict-of-interest 확인 필수 (Bhargava 의 Sel_6 결과를 직접 인용하므로 잠재 COI).

---

## 6. Cover Letter — 한글 (KMS 제출용 가상)

```
2026-04-15

대한수학회보 (Bulletin of the Korean Mathematical Society) 편집장 귀하

박민우 작성, "σ(n)·φ(n) = n·τ(n) 유일성 정리와 Bernoulli 독립 정리 가족: 환원 가설 프로그램" 의 투고를 부탁드립니다.

본 논문은 다음을 포함합니다:

1. 산술 함수 항등식 σ(n)·φ(n) = n·τ(n) 의 유일해가 n = 6 (n ≥ 2) 임을 3 개 독립 경로로 증명 (multiplicative R_local 분해, S_3 ≅ PSL(2,2) 군론, 컴퓨터 검증 n ∈ [2, 10⁴]).
2. 정수 6 의 독립 출현 가족 카탈로그에 2 건 신규 등록: Bernoulli 17 (avg|Sel_6| = σ(6) = 12, BKLPR 조건부) 및 Bernoulli 18 (BB(2) = 6, Radó 1962, unconditional).
3. 18 개 출현 중 σφ=nτ 직접 환원 (k ≥ 9 명시) vs 진정 독립 (18 - k ≈ 7-9) 분류 작업 (Bridge Program).
4. 유일성 정리의 부분 Lean4 형식화 (Mathlib 기반, prime case 완료, case 4 진행 중).

본 논문은 **7대 밀레니엄 난제 어느것도 해결을 주장하지 않습니다**. 구조적 survey + 유일성 정리 + bridge framework 제시가 목적입니다.

본 연구는 독창이며 타 학술지 동시 투고는 없습니다. 이해상충 없습니다. 분량 약 2,800 단어 (표/코드 부록 제외), 참고문헌 25 건.

심사위원 후보 (관련 분야 권위):
- Bhargava (Princeton, BKLPR)
- Poonen (MIT, BKLPR)
- Aaronson (UT Austin, Busy Beaver)
- 김명환 (서울대 수론)

검토 부탁드립니다.

감사합니다.

박민우
독립 연구자
경기도 하남시
arsmoriendi99@proton.me
```

---

## 7. 실제 제출 시 추가 작업 (현재 X)

1. LaTeX 변환 (Markdown → tex, AMS 클래스).
2. 학술지별 클래스 파일 (elsarticle.cls, springer.cls 등) 적용.
3. 그림 / 표 EPS / PDF 변환.
4. arxiv preprint 제출 (선택, COI 정리 후).
5. 실제 portal (Editorial Manager, ScholarOne 등) 업로드.
6. 수정 / rebuttal cycle (수일 ~ 수개월).

**본 작업은 위 1 ~ 6 모두 수행 X**.

---

## 8. 정직 선언

- 본 format draft 는 가상 시나리오 예시.
- 실제 학술지 제출은 R1 case 4 Lean4 잔여 sorry 해소 + 외부 동료 검토 후 권장.
- arxiv preprint 는 endorsement 절차 + 저자 신원 정리 후 별도 결정.
- 본 draft 는 papers/group-P/F5-journal-submission-format-2026-04-15.md 에만 존재.
- 7대 밀레니엄 난제 해결: 0/7 정직 유지.

---

> 산출물: papers/group-P/F5-journal-submission-format-2026-04-15.md
> 실제 제출 X. format only.
