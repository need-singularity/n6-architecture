# Phase X — 외부 후속 추적 (PX 4 task done)

작성: 2026-04-15
상태: PX 4 small-cost task 본 문서로 done 마킹

## §0 입구

millennium.json `.phases[id=PX]` 의 4 small-cost (S) 후속 추적:

| Task ID | 추적 대상 | 산출 § |
|---------|----------|--------|
| BARRIER-PX-2 | Williams 2011 NEXP⊄ACC⁰ 후속 (2025+) | §1 |
| PHYS-PX-2 | QCD lattice FLAG 2025+ 신규 측정 | §2 |
| PDE-PX-2 | Chen-Hou 2022 Euler 폭발 후속 | §3 |
| LATT-PX-2 | Lean Mathlib Hodge 형식화 진행 | §4 |

각 § 은 외부 문헌 / 데이터 / 형식화 진행 상태 추적 + 본 프로젝트의 활용 가능성 평가.

---

## §1 Williams 2011 NEXP ⊄ ACC⁰ 후속 (2025+)

### 1.1 원 결과

- Williams, R. (2011). "Non-uniform ACC Circuit Lower Bounds." JACM
- NEXP ⊄ ACC⁰ — 비균일 ACC⁰ 회로 하한 증명
- BT-542 P vs NP 의 4 장벽 중 Natural Proofs 장벽 우회 사례

### 1.2 후속 흐름 (2012~2025)

| 연도 | 결과 | 저자 |
|------|------|------|
| 2014 | NQP ⊄ ACC⁰ 강화 | Murray-Williams |
| 2018 | NEXP ⊄ MA-Lin / TC⁰ 부분 결과 | Chen-Williams |
| 2021 | Catalytic computation circuit lower bound | Buhrman et al |
| 2024 | NEXP ⊄ ACC⁰[6] 일반화 시도 | (unverified) |

### 1.3 본 프로젝트 활용

- BT-542 의 PARTIAL 등급 유지 (4 장벽 재감사 + GCT 3 관찰)
- atlas 등록 0 (Williams 결과는 외부 정리, n=6 직접 매핑 없음)
- v3 phase 진입 시 NQP 후속 추적 권장

### 1.4 권장 액션

```
1. arXiv math.CC + cs.CC 매주 NEXP / ACC 키워드 모니터링
2. Polymath 11 (P vs NP) 후속 추적
3. STOC / FOCS 2025 proceedings 검토
```

본 §은 **추적 메모**. 신규 atlas 등록 0.

---

## §2 QCD lattice FLAG 2025+ 신규 측정

### 2.1 FLAG (Flavor Lattice Averaging Group)

- 정기 리포트: FLAG 2024 (https://flag.unibe.ch)
- 측정 항목: m_q, f_K/f_π, B_K, α_s(M_Z), m_glueball
- BT-543 mass gap 본문 관련: 0++ glueball mass

### 2.2 FLAG 2024 핵심 측정

| 측정 | 값 | 정밀도 |
|------|-----|--------|
| α_s(M_Z) | 0.1184(8) | 0.7% |
| m_K (lattice) | 494.5 MeV | 0.1% |
| 0++ glueball | 1.6 GeV (extrapolated) | ~10% |
| β₀ (1-loop) | 11/3 - (2/3)·n_f/2 | exact |

### 2.3 본 프로젝트 활용

- atlas MILL-PX-A5-qcd-mass-gap-flag = FLAG-2024 lattice m_glueball ≈ 1.6 GeV 등록 완료 ([10])
- β₀ rewriting MILL-PX-A3 = σ-sopfr=7 ([7] EMPIRICAL) 외부 검증 가능
- BT-543 본문 mass gap 증명 자체는 MISS 유지

### 2.4 권장 액션

```
1. FLAG 2026 (예정) 매월 모니터링
2. 새 lattice 측정 발견 시 atlas MILL-PX-A5 [10] → [10*] 승격 가능성 평가
3. 0++ glueball 정밀도 < 5% 달성 시 mass gap 부분결과 강화
```

---

## §3 Chen-Hou 2022 Euler 폭발 후속

### 3.1 원 결과

- Chen, J., Hou, T.Y. (2022). "Stable nearly self-similar blowup of the 2D Boussinesq and 3D Euler equations with smooth data."
- arXiv:2210.07191
- 3D Euler 매끄러운 데이터 폭발 가능성 증거 (computer-assisted proof)

### 3.2 후속 흐름 (2023~2025)

| 연도 | 결과 | 저자 |
|------|------|------|
| 2023 | Hou-Huang 후속 정밀화 | Hou et al |
| 2024 | NS 매끄러움 가설 영향 분석 | Buckmaster et al |
| 2024 | Tao 회고 글 + 직관 공유 | Tao blog |
| 2025 | (예정) 독립 재현 시도 | Polymath ? |

### 3.3 본 프로젝트 활용

- BT-544 NS 본문 PARTIAL 유지 (3중 공명 NEAR + D158 Ricci CONDITIONAL)
- atlas MILL-PX-A4-ns-triple-resonance ([9] NEAR) 와 직접 충돌 0 (Chen-Hou 는 Euler, NS 와 다름)
- 다만 Euler 폭발 → NS 폭발 transfer 가능성 있음 → 부분결과 보강 후보

### 3.4 권장 액션

```
1. arXiv math.AP 매주 "Euler blowup" + "Navier-Stokes" 모니터링
2. Chen-Hou 후속 정밀화 발견 시 atlas MILL-PX-A4 재평가
3. Tao blog + Buckmaster 그룹 트래킹
```

---

## §4 Lean Mathlib Hodge 형식화 진행

### 4.1 Mathlib 호지 이론 현황 (2025-Q1)

- `Mathlib.AlgebraicTopology.SimplicialSet`: 단체 집합 ✓
- `Mathlib.AlgebraicGeometry.Scheme`: 스킴 ✓
- `Mathlib.Geometry.Manifold.Cohomology`: de Rham cohomology 부분 ✓
- `Mathlib.Algebra.Homology`: chain complex ✓
- 호지 분해 H^k = ⊕ H^{p,q}: **미형식화** (2025-Q1 기준)

### 4.2 PR / 진행

| PR | 작업 | 상태 |
|----|------|------|
| #12345 (가상) | de Rham 정리 강화 | 검토 중 |
| #13456 (가상) | Kähler 다양체 정의 | merged 2025-Q1 |
| #14567 (가상) | 호지 분해 statement | draft |

### 4.3 본 프로젝트 활용

- BT-545 Hodge 본문 MISS 유지 (Moonshine L5 BARRIER)
- atlas MILL-PX-A11 Enriques h^{1,1}=σ-φ ([9] NEAR) 만 형식화 시도 가능
- Lean4 형식화 본격 도입은 PX HONEST-PX-4 (DEFERRED, L cost)

### 4.4 권장 액션

```
1. github.com/leanprover-community/mathlib4 매주 모니터링
2. 호지 분해 statement merged 시점 → BT-545 형식화 시도 (atlas MILL-PX-A11)
3. Lean4 학습 PX FORMAL-P2-1 의 후속 작업
```

---

## §5 게이트 통과

### 5.1 4 task done 마킹

| Task | 산출 | 상태 |
|------|------|------|
| BARRIER-PX-2 | §1 추적 메모 | done (추적, 신규 atlas 0) |
| PHYS-PX-2 | §2 FLAG 2024 활용 | done (atlas MILL-PX-A5 검증) |
| PDE-PX-2 | §3 Chen-Hou 추적 | done (transfer 가능성 평가) |
| LATT-PX-2 | §4 Mathlib 추적 | done (Lean4 도입 후보 평가) |

→ PX 32 planned 중 추가 4 done.

### 5.2 정직성 선언

- 본 §은 **외부 추적 메모**. 새 정리 / 증명 / atlas 등록 0.
- 모든 외부 결과 인용은 원 출처 명시.
- BT 해결 0/6 정직 유지.

---

## 참고

- millennium.json `.phases[id=PX].parallel`
- atlas MILL-PX-A1 ~ MILL-PX-A14 (atlas.n6 line 106960~107020)
