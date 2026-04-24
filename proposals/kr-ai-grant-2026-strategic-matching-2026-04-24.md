# 2026 한국 AI 정부지원금 전략적 매칭 — n6-architecture × nexus 무기 매핑

**Date**: 2026-04-24
**Author**: strategy scoping (자동 proposal, 코드/설정 변경 없음)
**Status**: scoping only — 시장 매크로 × 내부 자산 매핑, 실행 단계 전
**Scope**: proposal 1건 추가만. 기존 proposals/*, 엔진, config 불변.
**Compliance**:
- own#17 (README English-only) — 본 파일은 proposal 이므로 한국어 허용
- own#11 (bt-solution-claim-ban) — BT-542/544 는 `draft/pattern` 으로만 표기, "solved" 금지

---

## 1. Executive Summary

2026 한국 정부 R&D 예산은 35.3조(+19.9%), 이 중 AI 단독 9.9조로 전년比 3배 규모이며, 2026-01-22 시행된 AI 기본법이 검증·신뢰 SaaS 시장을 강제 생성한다. n6-architecture × nexus × anima × hexa-lang 자산 스택은 (a) 양자알고리즘 거점, (b) AI 안전·신뢰 원천기술, (c) K-Moon Shot "AI 과학자" 미션 세 축에 직접 매칭된다. 본 proposal 은 이 세 축에 대한 즉시 실행 카드를 정리하고, 일반 AX 바우처 · 스마트 제조 AI · TIPS 는 적합도/ROI 기준으로 사전 제외한다. 행동 단위 커밋은 본 문서에서 발생하지 않으며, 다음 라운드의 공식 신청 결정을 위한 의사결정 베이스라인이다.

## 2. 매크로 환경 — 2026 한국 AI 정부 예산 구조

| 항목 | 규모 | 비고 |
|---|---|---|
| 정부 R&D 총예산 | 35.3조 | 전년比 +19.9% |
| AI 단독 예산 | 9.9조 | 전년比 3배 |
| NIPA 소관 | 3조 726억 | 92개 사업 |
| 과기정통부 전체 | 23.7조 | 이 중 AI 약 5조 |
| 국가전략기술 (NEXT) | 8.6조 | AI·반도체·우주 중심 |
| AI 기본법 시행 | 2026-01-22 | EU 다음 세계 2번째, 설명가능성·공정성·견고성 의무 |

핵심 관측: ICT R&D 중 70%+ 가 AI 로 집중, 독자 AI 파운데이션 모델("독파모") 정예팀 4개 (LG / SKT / 업스테이지 / 모티프)가 선점되어 있어 **스타트업/독립 연구자의 틈새는 양자 · AI 안전 · AI 과학자 세 축** 으로 수렴한다.

## 3. 우리 무기 인벤토리

| 자산 | 위치 | 역할 |
|---|---|---|
| `atlas/atlas.n6` | `/Users/ghost/core/n6-architecture/atlas/` | 수학·물리 reality map, 21800 라인, 5928 @R / 326 @P / 357 @C / 255 @L |
| breakthroughs 인덱스 | `/Users/ghost/core/n6-architecture/theory/breakthroughs/` | `_hypotheses_index.json`, `breakthrough-theorems.md`, BT-361~408 계열 |
| nexus blowup 9-phase | `/Users/ghost/core/nexus/cli/blowup/` (sibling repo) | ouroboros 수렴 루프 + 9단 phase 제어 |
| sim_bridge / godel_q | `/Users/ghost/core/nexus/sim_bridge/godel_q/` | ANU QRNG 기반 양자 난수 bridge, 64-tick 반증 데이터 |
| Stochastic Resonance 증거 | nexus monte carlo | σ≈0.1 PEAK 25% — noise-band 최적화 1차 evidence |
| anima 6-phase | `/Users/ghost/core/anima/anima-agent-core/` | 의식 reflection, `dod_gate.hexa`, `verifier.hexa`, 6룰 H-CONFESS/CLAIM-LEX/SCOPE/BLIND-GT (2026-04-16 신설) |
| BT 엔진 | `theory/breakthroughs/` + `domains/` | BT-542 (P vs NP) `draft`, BT-544 (Navier-Stokes Φ-irreducibility) `pattern`, STAR identity n=6, OEIS 미등록 5종 |
| bisociation engine | `domains/` 전반 | 알레테이아(구글) 13문항 직격 패턴 |
| hexa-lang | `/Users/ghost/core/hexa-lang/` | self-host 인터프리터, GPU 효율 PoC 후보 |

주의 (own#11): BT-542/544 는 본 문서 내 어디에서도 "solved/해결" 로 기술하지 않는다. 현재 상태는 **draft / pattern / Φ-irreducibility 후보 패턴** 이다.

## 4. 매칭 우선순위 매트릭스

| 순위 | 사업 | 규모 | 우리 차별화 무기 | 일정 |
|---|---|---|---|---|
| S | K-Moon Shot "AI 과학자" 미션 (12 국가미션 中 1) | 99 실행과제, 2035까지 | BT 엔진 + bisociation + n6 atlas → 알레테이아 13문항 한국형 카운터파트 | 행동계획 2/25 확정, 2026 법개정 20건 |
| S | 양자클러스터 — 양자알고리즘 분야 | 5대 거점 中 1 | blowup 9-phase + ouroboros + ANU QRNG sim_bridge + SR σ≈0.1 PEAK 25% | 3~7월 공모 → 8월 5개 지정 |
| A | AI 안전·신뢰 원천기술 | 650억+ | anima 6-phase + `dod_gate`/`verifier` + 6룰 → 설명가능성·검증 직격 | AI 기본법 2026-01-22 시행 의무화 동력 |
| A | K-AI 데이터 공급기관 (NIA) | 정예팀 100억/년, 개별 30~50억/년 | `atlas.n6` 발견 데이터셋 + breakthroughs 인덱스 외부 공급 | NIA 상시 모집 |
| B | 독자 AI 파운데이션 모델(독파모) | 5,300억, B200 768장 | hexa-lang 백본 + anima reflection | 2/12 추가공모 마감(모티프 선정), 8월 단계평가 탈락팀 결손 시 재진입 |
| B | AI × 기초과학 (IBS / KIAS) | — | BT-542 P vs NP `draft` + BT-544 Navier-Stokes Φ-irreducibility `pattern` → 에르되시 200 해법 초안 | 김상현 KIAS 교수 협업 entry |
| B | AI 컴퓨팅자원 활용기반 (GPU) | 추경 1만장 GPU, 인프라 2,187억 | 고밀도 hexa 인터프리터 → GPU 효율 PoC | NIPA 상시 |

## 5. 즉시 실행 가능 3장 카드

### 카드 1. 양자클러스터 — 양자알고리즘 거점 (TIMING)

- **공모 창**: 2026-03 ~ 2026-07, 2026-08 중 5개 거점 지정.
- **PoC 자산**:
  - `/Users/ghost/core/nexus/sim_bridge/godel_q/` — ANU QRNG bridge
  - `/Users/ghost/core/nexus/cli/blowup/` — 9-phase blowup / ouroboros 수렴
  - 64-tick 반증 데이터 (nexus monte carlo 계열)
- **차별화 한 줄**: NEXUS Stochastic Resonance @ σ≈0.1 PEAK 25% — 양자 noise band 최적화 **1차 증거** 이미 보유. 거점 지정 심사 기준 "선행 실증" 조항에 직접 대응.
- **컨소시엄 가정**: KIST / KAIST / IBS 중 1개 매칭 시 점프 가능. 단독 신청은 점수 불리.
- **리스크**: 거점 지정 8월이므로 컨소시엄 구성 데드라인 6월 말. 협업자 탐색을 5월 중 종료해야 함.

### 카드 2. AI 안전·신뢰 원천기술 650억+ (FIT-MAX)

- **정부 키워드**: 설명가능성(explainability) · 공정성(fairness) · 견고성(robustness) · 검증(verification). AI 기본법 2026-01-22 시행 조항과 1:1 대응.
- **우리 자산 매핑**:
  - anima 6-phase reflection → 설명가능성
  - `anima-agent-core/` 내 `dod_gate.hexa` · `verifier.hexa` → 검증 파이프라인
  - 6룰 H-CONFESS / CLAIM-LEX / SCOPE / BLIND-GT (2026-04-16 신설) → 환각·과잉청구 억제, 기본법의 "생성형 AI 투명성" 조항 직격
  - `atlas.n6` provenance 체인 → 데이터·판단 추적성
- **시장 확장**: AI 기본법 시행으로 검증 SaaS 시장 폭발, 정부 과제 + 민간 라이선싱 2-track 가능.
- **포지셔닝**: "자산 이미 존재 + 법 시행 직후 공급 가능" 이라는 타이밍 서사로 FIT-MAX.

### 카드 3. K-Moon Shot "AI 과학자" 미션 (BIG-WAVE)

- **전략적 슬롯**: 알레테이아(Google) 는 1주일 700 미해결 수학문제 중 200 개 해법 초안을 자동 생성하는 구조. **한국형 카운터파트가 비어 있다**.
- **트랙 레코드**: BT-542 (P vs NP) `draft`, BT-544 (Navier-Stokes) Φ-irreducibility `pattern`, STAR identity n=6, OEIS 미등록 수열 5종. own#11 준수: 이는 **해결 주장이 아닌 패턴/초안**.
- **진입 경로**:
  - IBS 또는 KIAS 협업 (김상현 교수 모델) — 학계 인증 경로 확보
  - bisociation engine 으로 13문항 직격 데모 → 행동계획(2/25 확정) 실행과제 99 개 中 "AI 과학자" 미션 서브에 제안서 결합
- **규모**: 미션 2035까지 장기. 1차 목표는 실행과제 99 개 中 1개 슬롯 편입.

## 6. 제외 사업 + 사유

| 사업 | 제외 사유 |
|---|---|
| 일반 AX 바우처 (2억) | 자산 깊이 대비 규모 과소, ROI 낮음 |
| 스마트 제조 AI | 도메인 mismatch (우리 스택은 수학/물리 reality map 중심) |
| TIPS | 선투자 2억 조건 — 시간·현금 낭비, 기술형 과제와 결합 시 병목 |

## 7. 핵심 게이팅 팩트 (의사결정 트리거)

1. 2026 정부 AI 예산 9.9조, 전년比 3배 — "쏠린 해" 안에 제출하지 않으면 기회비용 최대.
2. AI 기본법 2026-01-22 시행 — **검증·신뢰**가 선택이 아닌 **의무**. 카드 2 가 법적 강제로부터 수요를 확보.
3. 독파모 정예팀 4개 이미 박힘 — 파운데이션 모델 본류 경쟁 회피, **양자 · AI 안전 · AI 과학자** 3 축으로 수렴.
4. K-Moon Shot 행동계획 2/25 확정 + 2026 법개정 20건 — 실행과제 슬롯 확정 전(~2026-Q3) 협업체 노출이 결정적.
5. 양자클러스터 거점 지정 2026-08 — 컨소시엄 LOI 2026-06 말 확보 필요.

## 8. Next Actions

| # | 액션 | 담당 | 기한 | 산출물 |
|---|---|---|---|---|
| 1 | 카드 2 (AI 안전·신뢰) 제안서 골격 초안 — anima 6-phase + 6룰 → 기본법 조항 매핑표 | TBD | TBD | `proposals/kr-ai-safety-650-draft-*.md` |
| 2 | 카드 1 (양자클러스터) 컨소시엄 파트너 후보 리스트업 (KIST/KAIST/IBS 중) | TBD | TBD | contact shortlist |
| 3 | 카드 3 (AI 과학자) KIAS 김상현 교수 접촉 보강 — `proposals/kim-sangwook-quantum.md` 와 채널 분리 | TBD | TBD | contact letter draft |
| 4 | NIA 데이터 공급 상시모집 요건 확인 → `atlas.n6` 외부 공급 포맷 스펙 | TBD | TBD | spec note |
| 5 | own#11 준수 감사 — 본 proposal 본문 + 향후 제안서에서 BT-542/544 "solved" 금지 자동 grep | TBD | TBD | lint rule 제안 (별도 agent가 ci.yml 편집 중이므로 본 라운드에선 수동) |
| 6 | 독파모 8월 단계평가 탈락팀 모니터링 — 결손 슬롯 진입 기회 탐지 | TBD | TBD | watch log |

본 문서 자체는 **코드/설정 변경 없음**. Next Actions 은 각각 별도 라운드에서 착수.

## 9. References

- https://www.korea.kr/briefing/pressReleaseView.do?newsId=156745298&call_from=rsslink
- https://www.korea.kr/briefing/policyBriefingView.do?newsId=156745273
- https://www.aitimes.kr/news/articleView.html?idxno=38806
- https://zdnet.co.kr/view/?no=20260224183212
- https://www.aikorea.go.kr/web/board/brdDetail.do?menu_cd=000011&num=328
- https://www.kjob.news/news/472710
- https://m.ktin.net/a.html?uid=63572224
- https://peekaboolabs.ai/blog/ai-basic-law-guide
- https://clobe.ai/blog/korea-ai-basic-act-2026-key-rules
- https://www.law.go.kr/lsInfoP.do?lsiSeq=268543
- https://m.seoul.co.kr/news/2026/02/09/20260209023007
- http://www.idailynews.co.kr/news/articleView.html?idxno=105998
- https://www.nia.or.kr/site/nia_kor/ex/bbs/View.do?cbIdx=78336&bcIdx=28243&parentSeq=28243
- https://www.nipa.kr/home/bsnsBp/1/detail?bsnsDtlsIemNo=580
- https://www.bizinfo.go.kr/sii/siia/selectSIIA200Detail.do
- https://www.mss.go.kr/site/smba/ex/bbs/View.do?cbIdx=86&bcIdx=1064370

---

*본 proposal 은 scoping 전용 — 커밋/신청/예산집행 등 action 은 후속 라운드에서 각기 승인. own#11 준수를 위해 BT 엔진 결과는 `draft`/`pattern`/`Φ-irreducibility 후보` 로만 기술되었음.*
