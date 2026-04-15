---
id: millennium-v3-design
date: 2026-04-15
roadmap_task: HONEST-PX-3 (v3 로드맵 실체화)
grade: [10] design document
parent_roadmap: shared/roadmaps/millennium.json (v2.3)
license: CC-BY-SA-4.0
---

# Millennium 7 난제 로드맵 v3 — 설계 (v2.3 7 loop 경험 기반)

> **요약**: v2.3 FULL_SATURATION (saturation_round 14) 선언 이후 session-level 7 loop 에서 실측 BSD (Cremona 964k) + arXiv 180 papers + BT-542 4 barriers + OUROBOROS R14 CLI 가 추가되었다. v3 는 이 진전을 공식 phase 로 흡수 + 아직 지평 밖 인 DEFERRED 스코프 (Sage / Lean4 / external coord) 를 실행가능 단계로 분해한다.

---

## §0 입구 — v2.3 → v3 전환 동기

### 0.1 v2.3 최종 상태 (2026-04-15 13:10 UTC)

| 항목 | v2.3 수치 |
|------|-----------|
| total_tasks | 155 |
| done | 143 |
| partial | 5 |
| deferred | 7 |
| planned | 0 |
| phases_done | 4 (P0, P1, P3, P7) |
| phases_partial | 0 |
| saturation_index | 1.0 |
| saturation_round | R14 (0 신규 gap) |
| BT 해결 | 0/6 (정직 유지) |

### 0.2 v2.3 loop 1-7 에서 발생한 **새로운 진전** (v2 범위 외)

| loop | task | 산출 | atlas entries |
|------|------|------|--------------|
| 1 | GALO-PX-2 | Cremona 332k Sel_6 empirical | 4 |
| 1 | GALO-PX-3 | Iwasawa mod 6 surrogate | 2 |
| 1 | HONEST-PX-AUTO-EMPIRICAL | LMFDB/Cremona pipeline | 0 |
| 2 | GALO-PX-1 | (A3) Modified Conjecture (A3') | 2 |
| 2 | MONOTONE-PX-1 | atlas drift monitor CLI | 0 |
| 3 | BARRIER-PX-1 | BT-542 4 barriers catalog | 2 |
| 4 | GALO-PX-4 | κ(B) 점근 + σ(6)=12 경험 도달 | 3 |
| 5 | NUM-PX-3 | arXiv 180 papers survey | 3 |
| 6 | HONEST-PX-2 | OUROBOROS R14 감사 CLI | 2 |
| 7 | — | (본 문서 HONEST-PX-3) | — |

**누적**: 18 atlas entries, 11 breakthrough .md, 4 CLI 도구, 964k Cremona 실측 + 180 arXiv 수집.

### 0.3 v3 필요성

v2.3 schema 에 **missing** 인 스코프:
- (P) 실제 도구 정밀화 (Sage, Pari-GP, Lean4) — cost L, infrastructure
- (Q) 외부 수학자 협력 (peer review, preprint 투고) — cost M, multi-session
- (R) scale 확장 (Cremona 3M full DB, arXiv full-text, Cremona conductor 10^7) — cost L, time
- (S) meta-verification 심화 (namespace-aware OUROBOROS, Drift monotone 확장) — cost M

이들을 v2.3 PX 에 deferred 로만 두지 않고, v3 에서 **실행 가능 단계로 분해**.

---

## §1 v3 설계 원칙

### 1.1 정직성 첫째

- **BT 해결 주장 금지**: v3 끝까지 0/6 가능. progression 은 catalog / empirical / tool 개선.
- **외부 의존 명시**: Sage, Pari, Lean4, mathematician peer — 모두 v3 진입 조건의 **prerequisite** 로 명시.
- **점근 주장 ban**: "v3 끝에 RH 풀린다" 같은 문구 엄금. 대신 "v3 Phase N 완료 시 x 측정 가능" 식 operational 목표만.

### 1.2 3-track 분할

| track | 내용 | cost 합 | BT 연관 |
|-------|------|---------|---------|
| **E (Empirical)** | 실측 확장, 정밀 도구, scale-up | L+L+M | BT-541, 546 |
| **T (Theoretical)** | 증명 단계, 4 barriers 우회, Moonshine L5 | L+L+L | BT-542, 545 |
| **M (Meta)** | peer review, preprint, formal verification | M+L+M | 전 BT |

### 1.3 Phase 구조 (v3 P0~PX)

v3 는 v2.3 의 16 axis × 13 phase 구조를 **유지** 하고, **신규 P-tier 만 추가**:

| v3 phase | 내용 | v2.3 에서 승계 | 신규 |
|---------|------|---------------|------|
| P0~P10 | v2.3 그대로 | done/partial 유지 | 변경 없음 |
| PΩ | v2.3 그대로 | — | — |
| PX | v2.3 + loop1-7 entries | 15 done | 18 done (loop1-7) |
| **P11 (new)** | E track: Sage 정밀 + scale-up | — | 7 tasks |
| **P12 (new)** | T track: theoretical deepening | — | 6 tasks |
| **P13 (new)** | M track: peer coord + preprint | — | 5 tasks |

---

## §2 v3 Phase 11 — E Empirical track (7 tasks)

### E1: Sage/Pari-GP 설치 + 연동

- **목표**: Python runner 가 Sage/Pari 호출 가능하도록 로컬 환경 구축
- **cost**: L (설치 ~2시간 + API learning ~1시간)
- **산출**: `scripts/empirical/sage_wrapper.py` + 사용법 문서
- **MISS 조건**: Sage 의존성 Mac ARM 빌드 실패
- **DEFERRED 해결**: v2.3 loop 내 실행 불가 (session 격리)

### E2: Cremona 정밀 |Sel_n(E)| 재계산

- **목표**: v2.3 GALO-PX-2 의 1차근사 대신 정밀 |Sel_2|, |Sel_3| 계산
- **cost**: L (Sage `E.selmer_group(2)` 등)
- **산출**: 332k curve 정밀 값 → mean |Sel_6| 재측정
- **MISS 조건**: Sage 계산 시간 초과 (curve 당 10초+ 예상)

### E3: Iwasawa μ_p 정밀 계산

- **목표**: GALO-PX-3 surrogate → Sage `E.iwasawa_invariants(p)` 정밀
- **cost**: L (Iwasawa 계산은 curve 당 수초)
- **산출**: atlas MILL-GALO-PX3 정밀 버전 엔트리

### E4: Cremona 3M 전체 데이터 수집

- **목표**: ecdata 전체 ~330 shard (3M curve) 일괄 다운로드
- **cost**: M (~2GB, 1 hour)
- **산출**: data/cremona/allbsd/* 전체

### E5: κ(B) 10+ bin 점근 검증

- **목표**: GALO-PX-4 3-bin → 10+ bin 확장 (conductor 1M, 2M, 5M, 10M, ...)
- **cost**: M (E4 완료 후 분석 ~30 min)
- **산출**: κ(B) 의 asymptotic 함수 형태 추정

### E6: arXiv 정기 서베이 파이프라인

- **목표**: NUM-PX-3 원샷 → 월간 자동 수집 cron
- **cost**: M (GitHub Actions 설정 + 스크립트)
- **산출**: `.github/workflows/arxiv-quarterly.yml`

### E7: arXiv full-text download + topic clustering

- **목표**: 180 paper abstract → PDF fetch + NLP topic modeling
- **cost**: L (~1GB PDF, 하루 compute)
- **산출**: topic graph + n=6 keyword deep search

---

## §3 v3 Phase 12 — T Theoretical track (6 tasks)

### T1: BT-545 "Abelian Sixfolds" deep dive

- **목표**: arxiv:2603.20268 full-text 분석 → atlas MILL-PX-A11 (Enriques) 와 수학적 연결
- **cost**: L (~1 week reading + writing)
- **산출**: `theory/breakthroughs/bt-545-abelian-sixfolds-connection-v3.md`

### T2: LATT-PX-1 Moonshine L5 재시도

- **목표**: v2.3 deferred. V♮ 구성 5-step 중 L5 (Monster action) 의 새 접근 탐색
- **cost**: L (Conway-Norton, Borcherds literature 재독)
- **산출**: BT-18 PARTIAL 승격 또는 정직 MISS 유지

### T3: GALO-PX-4 (A3') 의 joint distribution 모델링

- **목표**: κ(B) 추세의 수학적 형태 추정 (power law? log?)
- **cost**: L (BKLPR 논문 재독 + fitting)
- **산출**: 새 conjecture 제안

### T4: BT-541 Guth-Maynard 2024 재연구

- **목표**: zeta zero large value 개선의 implications
- **cost**: L (preprint 재독)
- **산출**: atlas MILL-GUTH-MAYNARD-2024 entry

### T5: BT-542 meta-complexity deep

- **목표**: Hirahara 2022 MCSP 결과의 n=6 비적용성 재확인
- **cost**: L
- **산출**: BARRIER-PX-1 보강 문서

### T6: BT-543 Balaban 2D 재정리

- **목표**: Balaban 1980s continuum 구성 현대적 재정리
- **cost**: L (엄청난 분량)
- **산출**: survey, MISS 가능

---

## §4 v3 Phase 13 — M Meta track (5 tasks)

### M1: preprint 투고 (1건 이상)

- **목표**: v2.3 loop1-7 의 breakthrough .md 중 가장 강한 것 → arXiv 프리프린트
- **cost**: M (formatting + submit + 3-day review)
- **후보**:
  - `bsd-kappa-asymptotic-964k-2026-04-15.md` (루프4)
  - `arxiv-millennium-survey-180papers-2026-04-15.md` (루프6, meta-survey)
- **MISS 조건**: arXiv moderation 거부

### M2: 수학자 3인 초청 리뷰

- **목표**: HONEST-PX-5 = peer review 파이프라인. 이메일 템플릿 + 대상자 리스트
- **cost**: M (선정 + 전송)
- **대상**: BSD/BKLPR 전문가 (Bhargava, Bhargava-Shankar, ...)
- **MISS 조건**: 답변 없음 (정상적으로 예상됨)

### M3: Lean4/Coq 형식화 탐색

- **목표**: HONEST-PX-4 + FORMAL-P3-1 + FORMAL-PX-1 3 tasks 통합
- **cost**: L (Lean4 학습 2~3주)
- **산출**: MILL-PX-A1 Theorem B (σφ=nτ iff n=6) 의 Lean4 증명 시도

### M4: HONEST-PX-EXT-AUDIT

- **목표**: 외부 감사 파이프라인 — GitHub PR 형식으로 외부인 기여 수용
- **cost**: M (CONTRIBUTING.md + 이슈 템플릿 작성)
- **산출**: repository 인프라

### M5: OUROBOROS 2.0 — namespace-aware severity

- **목표**: 루프7 기반 향상
- **cost**: M (기존 CLI 확장)
- **산출**: scripts/monotone/ouroboros_detector_v2.py

---

## §5 v3 전환 조건 (v2.3 → v3)

### 5.1 **MANDATORY** (v3 공식 시작 전 필수)

- [ ] v2.3 deferred 7 항목 중 최소 3 이상 "v3 Phase 배치" 완료 → 본 문서 §2-4
- [ ] v2.3 loop1-7 atlas 18 entries 재확인 (R14 CLEAN 2026-04-15 확정)
- [ ] Monotone drift 체크 (MONOTONE-PX-1 CLI 로 baseline 등록)
- [ ] BT 해결 수 0/6 재확인 (v3 시작 전 정직 선언)

### 5.2 **RECOMMENDED** (v3 시작 전 권장)

- [ ] Sage 환경 구축 시도 (E1)
- [ ] arXiv 월간 파이프라인 prototype (E6)
- [ ] Lean4 학습 시작 (M3 prep)
- [ ] preprint 1건 초안 (M1 prep)

### 5.3 v3 시작 공식 선언

조건 §5.1 4 건 전부 완료 + 사용자 "go v3" 명시 → millennium.json schema_version: "3.0" 으로 승격.

---

## §6 v3 정직성 헌장 (charter)

### 6.1 4 대 원칙

1. **BT 해결 주장 금지**: v3 진행 어느 시점에도 "RH 풀었다" 등의 claim 금지.
2. **외부 의존 노출**: v3 작업은 Sage / Lean4 / arXiv API / 외부 수학자 에 의존. 이 의존을 숨기지 않고 명시.
3. **MISS 조건 사전 명시**: 각 task 에 MISS 기준 정확 규정 (v2.3 의 원칙 계속).
4. **OUROBOROS 주기 감사**: v3 매 phase 종료 시 OUROBOROS CLI 실행, R14 CLEAN 확인.

### 6.2 v3 audit 체크리스트 (매 phase 말)

```
[ ] atlas 신규 entries N 건, 각각 외부 source 명시
[ ] OUROBOROS R14 CLEAN (MILL-*, BT-*)
[ ] drift monotone check (하향 등급 0)
[ ] BT 해결 수 0/6 재확인
[ ] peer review 요청 상태 갱신
```

---

## §7 관련 파일

- `shared/roadmaps/millennium.json` (v2.3, parent)
- `theory/breakthroughs/bsd-cremona-sel6-empirical-2026-04-15.md` (loop1)
- `theory/breakthroughs/bsd-A3-modified-with-joint-covariance-2026-04-15.md` (loop2)
- `theory/breakthroughs/bsd-kappa-asymptotic-964k-2026-04-15.md` (loop4)
- `theory/breakthroughs/bt-542-p-vs-np-4-barriers-survey-2026-04-15.md` (loop3)
- `theory/breakthroughs/arxiv-millennium-survey-180papers-2026-04-15.md` (loop5)
- `theory/breakthroughs/ouroboros-atlas-audit-2026-04-15.md` (loop6)
- `scripts/empirical/*.py` (5 files)
- `scripts/monotone/*.py` (2 files)

---

## §8 atlas 엔트리 제안

```
@R MILL-HONEST-PX3-v3-design-published = v3 로드맵 설계 문서 (7 loop 경험 기반, 3-track 18 tasks) :: n6atlas [10]
  "HONEST-PX-3 v3 로드맵 실체화 (2026-04-15 loop 8): v2.3 FULL_SATURATION 후 session loop1-7 진전 흡수.
   3-track (E Empirical 7 / T Theoretical 6 / M Meta 5) = 18 신규 task. v2.3 → v3 전환 조건 4 MANDATORY
   + 4 RECOMMENDED. 정직성 헌장 4 원칙 명시 (BT 해결 금지, 외부 의존 노출, MISS 사전, OUROBOROS 주기).
   BT 해결 0/6 정직 유지 — v3 도 결과물 아닌 tool + catalog 개선 목표"
```

---

*작성: 2026-04-15 loop 8*
*위치: theory/roadmap-v2/millennium-v3-design-2026-04-15.md*
*다음 단계: 사용자 승인 시 v3 공식 시작 + millennium.json schema_version 3.0 으로 승격*
