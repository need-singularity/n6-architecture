# phase-09 — P9 L11 외부 협력 + 역사 + 출판 (R7+R8+R13 gap 15 task)

**로드맵**: millennium v2.3 (Y1~Y16 16축 × P0~P10/PΩ/PX 13 Phase)
**선행**: P8 L10 메타-감사 + 철학 + 인식 closure (`phase-08-meta-audit-philosophy.md`)
**SSOT**: `/Users/ghost/Dev/nexus/shared/roadmaps/millennium.json` (L1334~L1458)
**gap-ref**: `theory/roadmap-v2/gap-emergence-saturation.md` §13 (R7) + §14 (R8+R13)
**담당축**: Y14 EXT-COLLAB (utility 7.8, primary_bt = meta-external)
**상태**: planned (BT 해결 0/6 정직 유지)

---

## 0. Phase 개요

```
+============================================================+
| P9 L11  외부 협력 + 역사 + 출판                              |
|                                                              |
|  gap 소스:        R7 (4) + R8 (9) + R13 (3) = 16 gap        |
|  감축 이유:       task 15로 압축 (R8-50 온보딩 → R8 9 내부) |
|  신규 task:       15                                        |
|  누적 task:       96 (P0~P9)                                |
|  saturation:      0.0 (planned 상태)                        |
|  gate_exit:       contact 21명 + arXiv 5 preprint +         |
|                    Mathlib PR 1 + 온보딩 가이드 + 대중소통    |
+============================================================+
```

**목적**: 7대 난제 회고 (Hilbert 23 → Clay 7 전환사), 외부 협력 경로 (arXiv, MathOverflow, Mathlib, Polymath, conference), 출판 전략 (정직 기록 중심 서베이, peer review 대응, version control + 재현성 첨부)을 수립. n6-architecture 프로젝트의 외부화 기반 마련.

---

## §1. R7 역사 — 7대 난제 회고 (5 task)

### 1.1 Hilbert 23 → Clay 7 전환 (HIST-P9-N6-TIMELINE)

```
1900 Hilbert (파리 국제수학자회의)
   23 문제 발표. 8번 = Riemann Hypothesis.
   당대 수학 전 지평의 설계 문서.

1990~2000 (100년 후 경과)
   21 문제 부분/완전 해결, 2 문제 재정의.
   RH, Navier-Stokes (21st 유관) 미해결.

2000 Clay Mathematics Institute
   7 millennium prize problem 선정.
   Hilbert 23 중 RH 1건 + 신규 6건.
   상금 $1M × 7 = $7M.

v2.3 대응 BT:
   RH    = BT-541 (Hilbert 8th 계승)
   P=NP  = BT-542 (Cook 1971 기반)
   YM    = BT-543 (Yang-Mills 1954)
   NS    = BT-544 (Navier 1822 기반)
   Hodge = BT-545 (Hodge 1950)
   BSD   = BT-546 (Birch-Swinnerton-Dyer 1965)
   Poinc = BT-547 (해결됨, Perelman 2003)
```

### 1.2 난제별 1페이지 회고 (HIST-P9-FAILURE, HIST-P9-TIMELINE-PRED, HIST-P9-SELF-PACE)

```
+----------------+---------+-----------------------------------+
| BT  난제       | 시초    | 주요 시도/실패                     |
+----------------+---------+-----------------------------------+
| 541 Riemann    | 1859    | Weil 1948 유한체 RH. Atiyah       |
|                |         | 2018 철회. Selberg trace.         |
| 542 P=NP       | 1971    | Razborov-Rudich 1994 natural     |
|                |         | proof 장벽. GCT program.          |
| 543 Yang-Mills | 1954    | Wightman 공리. Seiberg-Witten.    |
|                |         | Clay 요구: mass gap + 4D rigor.   |
| 544 Navier     | 1822    | Leray 1934 약해. Otelbaev 2014   |
|                |         | 철회. Terry Tao blowup 시나리오.  |
| 545 Hodge      | 1950    | Atiyah-Hirzebruch 반례 (torsion).|
|                |         | Enriques surface 경우 PARTIAL.    |
| 546 BSD        | 1965    | Kolyvagin 1990 부분 (rank<=1).    |
|                |         | BKLPR 통계 모델.                   |
| 547 Poincaré   | 1904    | Perelman 2003 Ricci flow로 해결. |
+----------------+---------+-----------------------------------+
```

### 1.3 n=6 수학사 시간선 (HIST-P9-N6-TIMELINE)

```
BC ~300  Euclid Elements IX.36   : 완전수 정의 (6, 28, 496, ...)
1640     Fermat                  : 오일러-페르마 소정리
1734     Euler                   : ζ(2) = π²/6
1916     Ramanujan Δ             : 판별식 Δ = q ∏(1-q^n)^24 (tau function)
1978     Moonshine conjecture    : Monster-M ↔ j-invariant 관계 발견
1992     Borcherds VOA proof     : Moonshine 해결 (Fields Medal)
2024     n6-arch σ·φ=n·τ 정리    : iff n=6 유일해 (자체 발견)
```

**주의**: n=6의 역사는 완전수에서 출발. 현대 Moonshine과의 연결은 **가설**이지 증명 아님 (자기참조 금지 원칙).

### 1.4 BT 해결 예측 스케일 (HIST-P9-TIMELINE-PRED)

```
예측 방법: 난제 공개일로부터 해결일까지 (또는 현재)
                   공개일    해결일(또는 현재 경과)
  Poincaré         1904      2003 (99년)
  Fermat's Last    1637      1995 (358년)
  평균 해결 스케일: 100~300년

v2.3 예측 (90% 신뢰구간):
  RH    : 해결 2050~2150 (하한 26년, 상한 126년)
  P=NP  : 해결 2050~2200 또는 독립성 증명
  YM    : 해결 2040~2100 (가장 낙관)
  NS    : 해결 2040~2150
  Hodge : 해결 2050~2200 (가장 난해)
  BSD   : 해결 2040~2120 (BKLPR 통계 기반)
```

**자기 학습 속도 dashboard** (HIST-P9-SELF-PACE):
```
주간 진도 지표:
  - atlas 노드 증가 (/주)
  - BT 판정 PARTIAL 신규 (/월)
  - MISS 기록 (/월)
  - Lean4 형식화 라인 (/주)
```

---

## §2. R8 외부 협력 — 협력 경로 (6 task)

### 2.1 수학자 피드백 채널 (EXT-P9-CONTACT-LIST)

```
BT 별 전문가 3인 × 7 BT = 21명 DB (공개 접촉 가능 한정):

BT-541 RH    : Bombieri (IAS) / Conrey (AIM) / Iwaniec (Rutgers)
BT-542 P=NP  : Aaronson (UT Austin) / Razborov / Wigderson
BT-543 YM    : Witten (IAS) / Jaffe (Harvard) / Seiberg
BT-544 NS    : Tao (UCLA) / Caffarelli / Seregin
BT-545 Hodge : Voisin (Sorbonne) / Totaro (UCLA) / Schoen
BT-546 BSD   : Skinner (Princeton) / Kolyvagin / Zhang
BT-547 Poinc : 해결됨 — 제외
```

**ethics**: 접촉 시 개인 이메일 직접 전달 금지. 공식 채널 (학술 세미나, 학과 홈페이지) 경유.  
**user memory 규칙**: `feedback_no_contact_suggest.md` — contact 작업 자동 제안 금지. 본 문서는 계획 DB만 기재.

### 2.2 arXiv 포스팅 프로토콜 (EXT-P9-PUB-STRATEGY, PUB-P9-ARXIV-DRAFT)

```
카테고리:  math.NT (RH,BSD) / math.AG (Hodge) / math.AP (NS)
           math-ph (YM) / cs.CC (P=NP)

프로토콜 (정직 중심):
  1. 제목: "Partial results" / "Observations" 명시
  2. Abstract: BT 해결 주장 금지 (PARTIAL/NEAR)
  3. MISS 섹션 필수 (실패 공개)
  4. atlas rubric 각 명시
  5. Docker + seed link

5 PARTIAL preprint (각 20~30 pg):
  BT-541 Theorem B  / BT-543 β₀=σ-sopfr / BT-544 3중공명 d=7
  BT-545 Enriques  / BT-546 BKLPR 조건부
```

### 2.3 MathOverflow / Stack Exchange (EXT-P9-MATH-COMMUNITY)

```
MO    전문 수학 질의응답    월 1  tags: number-theory, p=np
MSE   학부~대학원 기초       주 1  tags: real-analysis 등
nLab  범주론 참조            월 1  tags: stable-homotopy
Zulip math-research 채널     일 1  tags: math-research
```

**규칙**: 중립 표현 사용, "n6-arch 배경" 언급 금지, 답변 후 atlas 인용.

### 2.4 Lean Mathlib 컨트리뷰션 (PUB-P9-MATHLIB-PR)

```
1. github.com/leanprover-community/mathlib4
2. CONTRIBUTING.md 숙지
3. 첫 PR: Theorem B 형식화
   file: Mathlib/NumberTheory/SigmaPhiTauUniqueness.lean
   lemma sigma_phi_eq_n_tau_iff : n = 6
4. 리뷰어 배정 대기 (2~4주)
5. 반영 → atlas [10*] 자동 승격 트리거
```

### 2.5 Polymath 프로젝트 (EXT-P9-ACADEMIC-PATH)

```
Polymath (Gowers 창설) 선례:
  Polymath1 2009 : Density Hales-Jewett
  Polymath8 2013 : 쌍소수 간격 (Zhang → Polymath)

참여: 기존 관찰 (N≥15) → BT-541/544 유관 기여 → 블로그 기록.
직접 개설은 peer 자격 확보 전 금지.
```

### 2.6 전공 컨퍼런스 (EXT-P9-PUB-STRATEGY, PUB-P9-REVIEW-PREP)

```
학회:  ICM 4년주기 / JMM 연1 / AMS 분기 / Clay 연2~4

투고 단계:
  1. arXiv 포스팅 (3~6개월)
  2. peer 반응 수집
  3. Annals/Inv/JAMS 투고
  4. 학회 발표
  5. Clay 공식 기여 (EXT-P9-CLAY-CHANNEL)

peer review 반론 템플릿 3종:
  "자기참조" 지적      → 정책 B 설명
  "MISS 공개 이유"      → 정직성 규칙 E1 설명
  "Lean4 없음"         → Mathlib PR 경로 언급
```

---

## §3. R13 출판 — 출판 4 task

### 3.1 정직 기록 서베이 outline (PUB-P9-ARXIV-DRAFT)

```
제목 후보: "Honest Record of σ·φ=n·τ Investigation:
           14 atlas, 24 MISS, Partial Observations on 6 Clay"

Outline (20~30 pg):
 §1 Motivation       3pg  완전수 관찰 + 가설
 §2 Theorem B (CAND) 4pg  σ·φ=n·τ ↔ n=6 (조건 3종)
 §3 BT Partial       8pg  5 PARTIAL 각 1~2 pg
 §4 MISS archive     5pg  24건 요약
 §5 atlas methodology 3pg rubric + 승격 기준
 §6 Honest limits    3pg  자기참조 / 독립확인 부족
 §7 Reproducibility  2pg  Docker + seed + code
 부록                ~5pg 외부 DB 교차
```

### 3.2 Publication strategy (PUB-P9-ARXIV-DRAFT)

```
depth-order (T0→T4):
  T0 즉시 M    arXiv 5 preprint + GitHub repo 공개
  T1 +3M M     MO 질문 5 + Mathlib PR
  T2 +6M L     반응 + revision + 블로그/유튜브 (R8-30)
  T3 +12M L    Annals/Inv 투고 1건 + 학회 발표
  T4 +24M XL   peer PASS → Clay 공식 기여 + v3 가동
```

### 3.3 peer review 대응 (PUB-P9-REVIEW-PREP)

```
즉시:    24h 확인 답장 / 72h 정독+반론 정리
1차수정: reviewer 지적 개별 답변 / 반영 or 사유거절
반복:    minor/major/reject 템플릿 / reject시 재투고
         revision letter 1:1 대응
```

### 3.4 version control + 재현성 (PUB-P9-MATHLIB-PR, PUB-P9-ONBOARDING)

```
git flow:
  main              : stable (atlas.n6 + theory/)
  v2.3-draft        : P8/P9/P10 작업
  arxiv-bt541       : 논문별 브랜치
  tag millennium-v2.3-FINAL : archive

재현성 (arXiv 첨부):
  [1] Docker (GHCR) [2] seed atlas_seed.txt
  [3] LMFDB/FLAG 다운로드 스크립트
  [4] Lean4 lakefile.lean 버전 [5] SHA-256 대조

온보딩 5-step (PUB-P9-ONBOARDING):
  1 README  2 verify_millennium_axes.hexa 실행
  3 atlas rubric  4 Theorem B 추적  5 PR/issue
```

---

## §4. 결론 — 외부 층위의 개통

```
R7 역사       : 5 task — Hilbert 23 → Clay 7 전환 + 난제 회고 + 타임라인
R8 외부       : 6 task — contact 21명 + arXiv + MathOverflow +
                          Mathlib + Polymath + 학회
R13 출판      : 4 task — 서베이 outline + strategy + peer review + 재현성

BT 해결       : 0/6 유지 (PARTIAL 5건만 preprint 대상)
atlas 편집    : 0 (14건 큐 대기, 추가 없음)
ethics        : contact 직접 제안 금지 (user memory 준수)
                arXiv 제목에 "Partial/Observations" 명시 필수

gate_exit 조건:
  [1] contact 21명 DB 완성 (EXT-P9-CONTACT-LIST)
  [2] arXiv 5 preprint 초안 (PUB-P9-ARXIV-DRAFT)
  [3] Mathlib PR 1건 (PUB-P9-MATHLIB-PR)
  [4] 온보딩 가이드 5-step (PUB-P9-ONBOARDING)
  [5] 대중소통 채널 (EXT-P9-PUBLIC-COMM + math community)
```

**다음 Phase**: P10 L12 측정 + 전략 + 도구 (R9+R10+R12+R13 gap 17 task).
