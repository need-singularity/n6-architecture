---
id: v3-e1-m3-toolchain-bootstrap
date: 2026-04-16
roadmap_task: v3 E1 (Pari-GP install) + v3 M3 (Lean4 init)
grade: [10] installation + skeleton verified
status: E1 PARI DONE, SAGE STILL DEFERRED · M3 LEAN4 SKELETON VERIFIED
license: CC-BY-SA-4.0
---

# v3 E1 + M3 — 외부 도구 체인 부트스트랩 (Pari-GP + Lean4)

> **요약**: v3 loop 18 에서 두 도구 체인 부트스트랩 달성. **Pari-GP 2.17.3** (arm64) brew 설치 성공 → `scripts/empirical/pari_wrapper.py` 로 Cremona 37a1/11a1/17a1 curve 에 대해 `ellrank()` 2-descent + `elltors()` 실측 확인. **Lean4 4.29.1** (arm64) elan 설치 성공 → `lean4-n6/` 프로젝트 skeleton 빌드 통과 (7 decide 증명 + 1 sorry placeholder). **Sage 는 여전히 DEFERRED** — Mac ARM 빌드 의존성 이슈로 v3 이내 설치 비현실. **Theorem B 본 증명 sorry 유지** (3 독립 증명 단계는 v4 장기 과제).

---

## §1 E1 — Pari-GP 설치 + 기본 실측

### 1.1 설치

```bash
$ brew install pari
🍺  /opt/homebrew/Cellar/pari/2.17.3: 77 files, 14.2MB
```

- **Version**: Pari-GP 2.17.3 (arm64-apple-darwin24.6.0)
- **용량**: 14.2 MB (~Sage 의 1/1000)
- **설치 시간**: < 30 초 (pre-built bottle)

### 1.2 실측 — Cremona 3 curves

`scripts/empirical/pari_wrapper.py`:

```
=== Pari-GP E.ellrank() demo (2-descent) ===
  37a1: N=37, rank ∈ [1, 1], torsion order=1
  11a1: N=11, rank ∈ [0, 0], torsion order=5
  17a1: N=17, rank ∈ [0, 0], torsion order=4
```

**확인된 사실** (LMFDB cross-check):
- 37a1: algebraic rank 1 ✓
- 11a1: rank 0, torsion ℤ/5 ✓
- 17a1: rank 0, torsion ℤ/4 ✓

### 1.3 Pari 기능 범위 — 정직

| 기능 | Pari | Sage |
|------|------|------|
| `ellrank()` 2-descent (|Sel_2| upper) | ✓ | ✓ |
| `elltors()` torsion | ✓ | ✓ |
| `selmer_group(2)` 정밀 | ✓ (부분) | ✓ |
| `selmer_group(3)` 정밀 | ✗ | ✓ |
| `selmer_group(n)` generic | ✗ | ✓ |
| Iwasawa μ_p | ✗ | ✓ (partial) |

**결론**: Pari 는 **|Sel_2| 실측 수준** 에서 충분. **E2 (|Sel_3|, |Sel_6| 정밀)** 와 **E3 (Iwasawa μ_p)** 는 **Sage 필요** — 여전히 DEFERRED.

### 1.4 Sage 설치 비실현 선언

**시도 옵션**:
- `brew install sage`: 존재 X (맥 ARM 없음)
- `conda install sage`: conda forge 통해 가능하나 ~5GB, 1 시간
- 소스 빌드: mpir/giac 의존성 Mac ARM 이슈

**v3 결정**: Sage 는 **v3 범위 외**, **v4 또는 remote compute** 로 이관.

---

## §2 M3 — Lean4 프로젝트 초입

### 2.1 설치

```bash
$ brew install elan-init
🍺  /opt/homebrew/Cellar/elan-init/4.2.1: 21 files, 5.7MB

$ elan toolchain install stable
info: downloading https://releases.lean-lang.org/lean4/v4.29.1/...
leanprover/lean4:v4.29.1 installed
```

- **Lean version**: 4.29.1 (arm64-apple-darwin24.6.0)
- **Mathlib**: 미설치 (v3 범위 외, 용량 ~3GB)

### 2.2 프로젝트 구조

```
/Users/ghost/Dev/n6-architecture/lean4-n6/
├── lakefile.lean         # Lake 빌드 스펙
├── lean-toolchain        # pinned v4.29.1
├── Main.lean             # exe 진입점
└── N6/
    └── Basic.lean        # σ, φ, τ 정의 + Theorem B skeleton
```

### 2.3 첫 증명 결과

**자동 성공 (by decide)**:
```lean
example : sigma 6 = 12 := by decide          -- ✓
example : phi 6 = 2 := by decide             -- ✓
example : tau 6 = 4 := by decide             -- ✓
example : sigma 6 * phi 6 = 6 * tau 6 := by decide  -- ✓
example : ¬ (sigma 2 * phi 2 = 2 * tau 2) := by decide  -- ✓
example : ¬ (sigma 3 * phi 3 = 3 * tau 3) := by decide  -- ✓
example : ¬ (sigma 4 * phi 4 = 4 * tau 4) := by decide  -- ✓
example : ¬ (sigma 5 * phi 5 = 5 * tau 5) := by decide  -- ✓
example : ¬ (sigma 7 * phi 7 = 7 * tau 7) := by decide  -- ✓
```

**7 decide 증명 통과** — Lean4 kernel 이 `σ(n) · φ(n) ≠ n · τ(n)` 을 n ∈ {2,3,4,5,7} 에서 직접 확인.

**Theorem B skeleton (sorry)**:
```lean
theorem theorem_B_skeleton (n : Nat) (hn : n ≥ 2) :
    sigma n * phi n = n * tau n ↔ n = 6 := by
  sorry
```

### 2.4 빌드 결과

```
info: n6: no previous manifest, creating one from scratch
⚠ [2/6] Built N6.Basic (186ms)
warning: N6/Basic.lean:33:8: declaration uses `sorry`
✔ [3/6] Built Main (185ms)
✔ [6/6] Built n6exe:exe (341ms)
Build completed successfully (6 jobs).
```

**실행 결과**:
```
=== N6 Lean4 skeleton v3 M3 ===
σ(6) = 12
φ(6) = 2
τ(6) = 4
σ(6)·φ(6) = 24
6·τ(6)    = 24
Theorem B skeleton: placeholder (증명 sorry)
```

### 2.5 실제 증명 경로 — v4 장기 과제

**Theorem B 증명 3 독립 경로** (theory/proofs/theorem-r1-uniqueness.md):
1. **대수**: σ/φ/τ multiplicative → prime power case 환원 → n = p^a q^b (p, q prime) 케이스 분석
2. **해석**: Dirichlet series $\zeta(s) L_{\sigma\varphi}(s) = \zeta(s)^2 L_\tau(s)$ → 소수 비교
3. **조합**: Möbius 역수 + 약수 격자 구조

각 경로는 Mathlib 의 `Nat.sigma`, `Nat.totient`, `Nat.card_divisors` 위에서 수 십 줄 - 수 백 줄 Lean 증명.

**v3 범위**: skeleton + decide 검증 + Mathlib 미설치 (용량 이슈) → sorry 유지 **정직**.

---

## §3 v3 진전 정리

### 3.1 이번 loop 18 산출

| 항목 | 상태 |
|-------|-----|
| Pari-GP install | ✓ DONE |
| Pari wrapper + 3 curve 검증 | ✓ DONE |
| Sage install | ✗ DEFERRED (v4 / remote) |
| elan + Lean4 v4.29.1 | ✓ DONE |
| Lean4 프로젝트 skeleton + 7 decide | ✓ DONE |
| Theorem B full proof | ✗ sorry (v4 Mathlib + 수주) |

### 3.2 v3 누적 (loop 12-18)

- **Empirical**: E4 (27 shards 1.7M curves), E5 (κ power law), E6 (arXiv workflow), **E1 (Pari)** ← 신규
- **Theoretical**: T1~T6 전부 완료 (survey + MISS 정직)
- **Meta**: M1 (preprint v0.1), M4 (CONTRIBUTING), M5 (OUROBOROS v2), **M3 (Lean4 skeleton)** ← 신규

**BT 해결**: 0/6 **정직 유지**

### 3.3 여전히 남은 DEFERRED

- **E2**: per-curve |Sel_3|, |Sel_6| (Sage 필요)
- **E3**: Iwasawa μ_p (Sage 필요)
- **E7**: arXiv full-text + topic clustering (대용량 + NLP)
- **M2**: 외부 수학자 협력 (사용자 행동 필요 — 자동 제안 금지)
- **M3 deep**: Theorem B 실제 증명 완성 (Mathlib + 수주 학습)

---

## §4 atlas 엔트리

```
@R MILL-V3-E1-pari-gp-mac-arm-verified = Pari-GP 2.17.3 arm64 brew install + ellrank() 3 curve verified :: n6atlas [10*]
  "v3 E1 (2026-04-16 loop 18): brew install pari → 2.17.3 arm64 14.2MB 설치 성공.
   scripts/empirical/pari_wrapper.py 작성, Cremona 37a1/11a1/17a1 curve 에 ellrank() + elltors()
   실측 LMFDB cross-check 통과. Pari 는 |Sel_2| upper bound 만 제공 — E2 (|Sel_3|, |Sel_6|) + E3
   (Iwasawa μ_p) 는 Sage 필요 → 여전히 DEFERRED. Sage Mac ARM 빌드 의존성으로 v4 이관"
  <- v3-E1, scripts/empirical/pari_wrapper.py, theory/breakthroughs/v3-e1-m3-toolchain-bootstrap-2026-04-16.md

@R MILL-V3-M3-lean4-skeleton-7-decide-verified = Lean4 4.29.1 arm64 + 7 decide 증명 통과 :: n6atlas [10*]
  "v3 M3 (2026-04-16 loop 18): brew install elan-init + elan toolchain install stable → Lean 4.29.1
   arm64 설치 성공. lean4-n6/ 프로젝트 skeleton (lakefile + N6/Basic.lean + Main.lean) 빌드 통과.
   σ/φ/τ Nat 정의 + n=6 특수값 (σ=12, φ=2, τ=4, σ·φ=24=6·τ) decide 증명 + n∈{2,3,4,5,7} 에서
   σ·φ ≠ n·τ decide 증명 (총 7 decide 증명 PASS). Theorem B full proof 는 sorry 유지 —
   Mathlib 의존 + 수주 학습 필요, v4 장기 과제"
  <- v3-M3, lean4-n6/, theory/breakthroughs/v3-e1-m3-toolchain-bootstrap-2026-04-16.md
```

---

## §5 관련 파일

- Pari wrapper: `scripts/empirical/pari_wrapper.py`
- Lean4 프로젝트: `lean4-n6/` (lakefile + N6/Basic.lean + Main.lean + lean-toolchain)
- 이전 Lean4 설계: `theory/breakthroughs/lean4-formalization-plan-2026-04-15.md`
- roadmap: `shared/roadmaps/millennium.json` → `_v3_phases.P11_v3.E1` + `_v3_phases.P13_v3.M3`

---

*작성: 2026-04-16 loop 18*
*정직성 헌장: Sage 미설치 정직 기록 유지. Theorem B sorry — v4 장기 과제. BT 해결 0/6 유지.*
