---
id: millennium-v4-design
date: 2026-04-16
roadmap_task: v4 공식 시작 (사용자 "go v4")
grade: [10] design document
parent_roadmap: shared/roadmaps/millennium.json (v3.0)
license: CC-BY-SA-4.0
---

# Millennium v4 — 설계 (2026-04-16, "go v4" 사용자 승인)

> **요약**: v3 SATURATION_ADJACENT (14/18 done, 4 external-blocked) 상태에서 사용자 "go v4" 승인. v4 는 (1) **v3 이월 4 tasks** 를 명시적 scope 으로 포함, (2) **depth 확장** (α=log(2)/4 이론 유도, Umbral VOA explicit, Mathlib 전환), (3) **scale 확장** (Cremona 3M full, κ 50-bin). v2.3 의 FULL_SATURATION 과 v3 의 SATURATION_ADJACENT 경험을 살려 **정직성 헌장 4 원칙 + v3 + honesty charter 계승**. BT 해결 수 0/6 정직 유지 전제.

---

## §0 입구 — v3 → v4 전환 맥락

### 0.1 v3 최종 상태

| 항목 | 값 |
|------|-----|
| v3 tasks 완료 | 14/18 (78%) |
| 블록 tasks | 4 (E2, E3, E7, M2) 전부 external |
| v3 고유 발견 | α≈log(2)/4 CONSISTENT, (A3″) conj, Pari/Lean4 toolchain |
| BT 해결 | 0/6 정직 유지 |
| v2.3 → v3 loop 수 | 11 loops (loop 12-22 of 2026-04-15~16) |

### 0.2 v4 필요성

**v3 이월 (필수)**:
- E2: per-curve |Sel_3|, |Sel_6| (Sage 의존)
- E3: Iwasawa μ_p
- E7: arXiv full-text + NLP
- M2: 외부 수학자 contact

**v3 시사점 (신규 필요)**:
1. α=log(2)/4 의 **이론 유도 불가** — T track 심화 필요
2. Bootstrap σ=0.022 → **bin scale-up** 필요 (E4_v4)
3. Lean4 [2,20] decide → **Mathlib 전환** 필요 (M3_v4 deep)
4. Umbral Moonshine heuristic → **explicit 구성** 필요 (T2 심화)

### 0.3 v4 와 v3 의 구별

| 축 | v3 | v4 |
|----|----|----|
| scope | wide (18 tasks, 3 tracks) | deep (17 tasks, 3 tracks) |
| focus | 이론 survey + tool bootstrap | derivation + scale + rigor |
| BT 목표 | 정직 MISS catalog | 정직 MISS 유지 + 구체화 |
| 외부 의존 | 이월 + blocked | 명시적 handle (remote compute 등) |

---

## §1 v4 설계 원칙

### 1.1 정직성 헌장 V4 (계승 + 강화)

v3 헌장 4 원칙 전부 계승 + 추가:

1. **BT 해결 주장 금지** — 계승
2. **외부 의존 명시** — 계승 + **강화**: 각 task 의 external dep 사전 명시
3. **MISS 조건 사전** — 계승
4. **OUROBOROS 주기 감사** — 계승
5. **v4 신규**: **이월 task 의 진행 공개** — 외부 해결 상태를 atlas 에 주기 업데이트

### 1.2 v4 3-track 분할

| track | 내용 | v3 대비 | BT 연관 |
|-------|------|---------|---------|
| **E (Empirical)** | 7 tasks: 이월 3 + 신규 4 | scale + precision | BT-541, 546 |
| **T (Theoretical)** | 5 tasks: α 유도 + Umbral + 확장 | 증명 시도 추가 | BT-545, 18 |
| **M (Meta)** | 5 tasks: 이월 1 + Mathlib + preprint 개정 | Lean4 formal 강화 | 전 BT |

---

## §2 v4 Phase 14 — E Empirical track (7 tasks)

### E1_v4 (이월 v3 E2): 정밀 |Sel_n(E)| per-curve

- **목표**: v3 E2 이월. Sage 또는 remote compute 로 330 shard 중 일부 per-curve
- **cost**: L (remote) 또는 DEFERRED
- **external**: Sage ARM 빌드 또는 remote Sage 서버
- **산출**: `data/cremona/per_curve_sel/<shard>.json`

### E2_v4 (이월 v3 E3): Iwasawa μ_p 정밀

- **목표**: v3 E3 이월. Sage `E.iwasawa_invariants(p)` per curve
- **external**: Sage
- **산출**: atlas MILL-V3-E3 정밀 값

### E3_v4 (이월 v3 E7): arXiv full-text + NLP

- **목표**: 180 paper abstracts → PDF + NLP topic clustering
- **cost**: L (1GB + 하루 compute)
- **external**: arXiv PDF API
- **산출**: topic cluster graph + n=6 keyword deep

### E4_v4 신규: κ(B) 50+ bin refinement

- **목표**: v3 E5 7 bin → 50 bin (bin width 10k, 500k까지)
- **cost**: M
- **의존**: 330 shards 전부 필요 (E5_v4 의존)
- **산출**: α uncertainty 0.022 → 0.005 예상

### E5_v4 신규: Cremona 3M full (330 shards)

- **목표**: v3 E4 27 shard 1.7M → 전체 330 shard 3M+ curve
- **cost**: M (~2GB 다운로드 + hour)
- **external**: Cremona ecdata mirror
- **산출**: data/cremona/allbsd/* 전체

### E6_v4 신규: 다른 n 에서 σφ vs nτ 비교

- **목표**: n=4, 8, 12, 30 에 대해 similar bin 분석 → n=6 의 **유일성** 통계적 평가
- **cost**: M
- **산출**: atlas MILL-V4-E6 multi-n baseline

### E7_v4 신규: η(E) per-curve 분포 조사

- **목표**: v3 T3 의 (A3″) η(E) 정의 실측 — per-curve |Sel_6|/(|Sel_2|·|Sel_3|) - 1
- **cost**: L (E1_v4 결과 의존)
- **산출**: η 분포 histogram + B 의존성 직접 관측

---

## §3 v4 Phase 15 — T Theoretical track (5 tasks)

### T1_v4 신규: α = log(2)/4 BKLPR 이론 유도 시도

- **목표**: v3 T3 의 empirical 발견 → BKLPR cokernel 수학 내 유도 시도
- **cost**: L (Bhargava-Kane 2013 재독 + fitting)
- **산출**: conditional proof 또는 정직 MISS 선언
- **MISS 확률**: 70% (v3 경험)

### T2_v4 신규: Umbral Moonshine A_2^12 / A_5^4 D_4 explicit

- **목표**: Cheng-Duncan-Harvey 2014 + DFR 2017 의 VOA explicit 구성
- **cost**: L (기존 literature 이해 + 실행)
- **산출**: VOA module 명시적 character table

### T3_v4 신규: Abelian Sixfolds full-text 분석

- **목표**: arXiv:2603.20268 full-text 접근 + 정확한 결과 재확인
- **cost**: M (arXiv PDF fetch + 재독)
- **산출**: v3 T1 의 heuristic 수정 또는 확인

### T4_v4 신규: (A3″) formulation 엄밀화

- **목표**: v3 T3 의 conjecture statement 수학적 rigor 강화
- **cost**: M
- **산출**: 정식 conjecture + predictions

### T5_v4 신규: 6 난제 cross-BT survey

- **목표**: v3 T4-T6 를 나머지 BT-18, 544, 547 (Hodge non-abelian, NS Reg) 로 확장
- **cost**: L
- **산출**: 5 survey .md

---

## §4 v4 Phase 16 — M Meta track (5 tasks)

### M1_v4 (이월 v3 M2): 외부 수학자 contact

- **목표**: v3 M2 이월. 사용자 결정 후 실행
- **external**: 사용자 direction
- **status**: 여전히 DEFERRED (자동 제안 금지)

### M2_v4 신규: Lean4 Mathlib 통합

- **목표**: lean4-n6/ 에 Mathlib dep 추가 → Nat.sigma, Nat.totient 전환
- **cost**: L (Mathlib 3GB 다운로드 + compile ~1 hour)
- **산출**: N6/MathlibBasic.lean, decide 실행 속도 개선

### M3_v4 신규: Theorem B 대수적 경로 1 증명 시도

- **목표**: σφ=nτ iff n=6 의 대수 (multiplicative function) 경로를 Lean4 로
- **cost**: L (수주 분량, v4 시작 시점에서는 skeleton)
- **산출**: N6/TheoremB_Algebraic.lean — partial 또는 complete

### M4_v4 신규: preprint v0.2 개정

- **목표**: v3 M1 draft + bootstrap result + T1_v4 (α 유도 시도) 반영
- **cost**: M
- **산출**: theory/preprints/millennium-v4-preprint-draft-YYYY-MM-DD.md

### M5_v4 신규: OUROBOROS v3 semantic similarity

- **목표**: v3 M5 namespace-aware → semantic clustering 추가 (embed-based similarity)
- **cost**: M (local embed 모델 또는 API)
- **산출**: scripts/monotone/ouroboros_detector_v3.py

---

## §5 v4 전환 조건 + 종료 조건

### 5.1 v4 시작 공식 (본 session 에서)

- ✓ 사용자 "go v4" 명시 (2026-04-16)
- ✓ v3 SATURATION_ADJACENT 선언
- ✓ 4 이월 tasks 명시적 scope 포함
- ✓ millennium.json schema_version 4.0 승격

### 5.2 v4 종료 조건 (saturation)

v4 는 다음 중 하나로 종료:
- **FULL_SATURATION**: 17 task 전부 done (이월 포함, 외부 해결 전제)
- **SATURATION_ADJACENT_V4**: 내부 실행 가능 전부 소진, 외부 블록만 남음 (v3 와 유사 가능)
- **사용자 "go v5"**: 의식적 전환

### 5.3 v4 예상 loop 수

v3 가 11 loops (loop 12-22) 에서 수행됐다. v4 는 더 **depth** 지향이므로 task 당 time 길어질 수 있음. 예상 15-20 loops.

---

## §6 v4 정직성 헌장 (charter V4)

### 6.1 4+1 원칙

1. **BT 해결 주장 금지** (계승)
2. **외부 의존 명시** (계승, 강화)
3. **MISS 조건 사전 명시** (계승)
4. **OUROBOROS 주기 감사** (계승)
5. **이월 task 진행 공개** (v4 신규): atlas 엔트리 에 external 해결 상태 기록

### 6.2 v4 audit 체크리스트 (매 phase 말)

```
[ ] atlas 신규 entries N 건, 각각 external source/dep 명시
[ ] OUROBOROS R14 CLEAN (MILL-*, BT-*)
[ ] drift monotone 확인
[ ] BT 해결 수 0/6 재확인
[ ] 이월 task 상태 업데이트 (v3 E2, E3, E7, M2 = M1_v4)
[ ] v4 전용 invariant: "v3 이월 tasks 의 외부 의존 상태 공개"
```

---

## §7 v4 첫 loop 계획

### 7.1 loop 1 (본 session)

- **T1_v4 시도**: α = log(2)/4 의 BKLPR 이론 유도
- **방법**: Bhargava-Kane 2013 의 cokernel 정리 + τ(6)=4 의 natural 등장 시도
- **예상 결과**: partial 분석 + MISS 또는 conditional 단계

### 7.2 loop 2-5

- M2_v4: Mathlib 설치 + 1 lemma 전환
- E5_v4: Cremona 3M 전체 다운로드 시도
- T2_v4: Umbral A_2^12 1 module character table

---

## §8 관련 파일

- **v3 design**: `theory/roadmap-v2/millennium-v3-design-2026-04-15.md`
- **v3 saturation**: `theory/breakthroughs/v3-saturation-adjacent-2026-04-16.md`
- **v3 T3 (A3″)**: `theory/breakthroughs/v3-t3-joint-distribution-modeling-2026-04-15.md`
- **v3 loop 19 bootstrap**: `theory/breakthroughs/v3-loop19-lean4-extended-kappa-bootstrap-2026-04-16.md`
- **Pari wrapper**: `scripts/empirical/pari_wrapper.py`
- **Lean4**: `lean4-n6/`
- **roadmap**: `shared/roadmaps/millennium.json` → `_v4_meta` + `_v4_phases`

---

*작성: 2026-04-16 loop 20 (v4 design doc)*
*정직성 헌장: BT 해결 0/6 유지. v3 이월 4 tasks 외부 의존 명시. v4 도 결과물 아닌 tool + catalog + depth 목표.*
