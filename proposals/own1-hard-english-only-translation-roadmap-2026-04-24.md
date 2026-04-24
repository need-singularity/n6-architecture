# own#1 HARD English-Only 대량 번역 로드맵 — 1,050 legacy → 0 by 2026 Q4

**Date**: 2026-04-24
**Author**: own-lint policy scoping (자동 proposal, 엔진/설정 변경 없음)
**Status**: scoping only — 실행 단계 이전의 로드맵 초안
**Scope**: `proposals/` 에 본 파일 1건 추가만. allowlist/tool/bridge/n6shared/.own 불변.
**Compliance**:
- own#1 (doc-english-hard) — 본 proposal 은 `proposals/` 경로이며 작성 시점 own#1 scope 외. 한국어 허용 (scope 확장 시 차후 번역 대상 후보).
- own#11 (bt-solution-claim-ban) — 번역 진척을 "draft/target" 으로만 기술. 미완 phase 를 "solved/완료" 로 단정 금지.
- own#17 (README English-only) — README 는 본 문서의 scope 외.

---

## 1. Executive Summary

own#1 의 `on_fail: block` 승격 (`2ef79fe7` → `4b472b0c`) 으로 신규 .md 는 한국어 0 bytes 가 **HARD block** 이 되었다. 기존 legacy 1,050 파일은 `tool/own1_legacy_allowlist.json` 으로 grandfather 처리했으나, 사용자의 **"hard!!! 모두 english only"** 강조 원칙 하에 allowlist 자체가 축소 대상이며 중기 목표는 **polyfill 폐지 → HARD 절대화** 이다.

본 로드맵의 **draft target** 은 2026 Q4 까지 legacy 카운트를 1,050 → 0 으로 축소하고, allowlist 파일 자체를 삭제하여 own#1 HARD 가 저장소 전역에서 예외 없이 강제되는 상태이다. 현 시점에는 Phase 0 (bridge + n6shared) 실행 중이며, 그 이외 phase 는 모두 target 상태.

## 2. 현황 인벤토리

| 디렉터리 | 파일 수 | 최대 CJK/파일 | 난이도 | 우선순위 | 비고 |
|---|---:|---:|:---:|:---:|---|
| `domains/` | 417 | 1457 | HIGH | P2 | culture/ 4파일이 최대 부채 (~1450자/파일). chip-* 서브도메인 별도 트랙 |
| `reports/` | 284 | (미집계) | MED | P3 | 자동 생성 형식 다수 → 템플릿 번역 우선 |
| `papers/` | 165 | (미집계) | HIGH | P4 | 수식/인용 밀도 높음, AST 검증 필수 |
| `theory/` | 149 | 896 | HIGH | P4 | 개념 깊이 — glossary 강제 필요 |
| `experiments/` | 25 | 829 | MED | P1 | 파일럿 대상 (소규모 + 난이도 관리 가능) |
| `n6shared/` | 6 | 417 | LOW | P0 | 동시 진행 (에이전트 B) |
| `bridge/` | 4 | (미집계) | LOW | P0 | 동시 진행 (에이전트 B) |
| **합계** | **1,050** | — | — | — | 직전 보고 기준 |

주의 (own#11): 위 테이블의 P0 항목도 본 문서 작성 시점에는 "in-flight / target 완료" 이며, 커밋 완료 전까지 "solved" 로 기술하지 않는다.

## 3. Phase 계획 (5 phase + Phase 0)

### Phase 0 — 즉시 (2026-04-24 target)
- 대상: `bridge/` 4 + `n6shared/` 6 = **10 파일**
- 책임: 동시 실행 중 에이전트 B
- allowlist 목표: 1,050 → **1,040**
- 상태: in-flight (본 proposal 은 개입하지 않음)

### Phase 1 — experiments/ 파일럿 (2026-05 target)
- 대상: 25 파일, MED 난이도
- 목적: 번역 파이프라인 검증 샘플
- 산출: `tool/batch_translate.py` 의 실제 배치 결과, 오류 회귀 리포트
- allowlist 목표: 1,040 → **1,015**

### Phase 2 — domains/ 우선 트랙 (2026-05 ~ 06 target)
- 대상: 417 파일 중 우선순위 상위 200 파일
  - `domains/culture/` 4개 (최대 부채 1457자)
  - `chip-*` 서브도메인 (일관 템플릿 다수)
- 산출: 200/417 draft 완료
- allowlist 목표: 1,015 → **815**

### Phase 3 — reports/ 일괄 (2026-07 target)
- 대상: 284 파일
- 전략: 자동 번역 + 자동 검토 파이프라인 (템플릿 반복 비율 높음)
- allowlist 목표: 815 → **531**

### Phase 4 — papers/ + theory/ (2026-08 ~ 09 target)
- 대상: 165 + 149 = **314 파일**
- 고난이도 축: 수식 보존, 이론 문맥 의미 유지, 인용 해시 불변
- Anthropic API 검수 루프 2-pass 필수 (1st: 번역, 2nd: 기술 용어 재검증)
- allowlist 목표: 531 → **217**

### Phase 5 — domains/ 잔여 + 청소 (2026-10 ~ 12 target)
- 대상: `domains/` 잔여 217 파일
- 최종 단계: allowlist 파일 자체 삭제 → own#1 HARD 예외 0
- allowlist 목표: 217 → **0**

## 4. 자동화 파이프라인 설계

**번역 파이프라인 (phase 공통 draft 설계)**

1. batch 선정: 우선순위 큐에서 N 파일 추출 (배치 크기 N = 10~25, phase 별 조정).
2. 번역 실행: Claude API (sonnet-4-6) — 기술 용어 보존 프롬프트 + glossary injection.
3. 검증 3단계:
   - CJK 0건 regex 체크 (own#1 runner 재활용).
   - markdown AST diff (헤더/링크/코드블록/수식 blocks 구조 동일성).
   - 용어 일관성 (`atlas`, `ouroboros`, `σ`, `φ`, `J₂`, `@R/@P/@C/@L`, BT-xxx 등 glossary 매칭).
4. allowlist shrink: 번역 완료 파일 entries 를 `tool/own1_legacy_allowlist.json` 에서 제거.
5. 커밋: `docs(translate): batch-N English conversion (N files)` 형식.
6. PR 기반 리뷰 (사람 1명 최종 sign-off).

**도구 후보 (draft target, 아직 실존 파일 아님)**

| 파일 | 역할 | 상태 |
|---|---|---|
| `tool/batch_translate.py` | API 호출 + 검증 orchestrator | target |
| `tool/translation_glossary.json` | 영어 유지 용어 사전 (atlas 고유명/수학 기호/BT 코드) | target |
| `tool/markdown_ast_diff.py` | 번역 전후 구조 동일성 검증 | target |

**프롬프트 설계 지침 (draft)**

- system: "You translate N6-architecture legacy Korean Markdown to English. Preserve all math ($...$), code blocks, frontmatter, links, and glossary terms verbatim. Do not add/remove sections."
- user: 원문 전체 + glossary JSON + "return only translated markdown, nothing else".
- 2-pass (Phase 4 한정): 1st=번역, 2nd=기술용어 재검증 (diff-only patch 반환).

**배치 실행 프로토콜**

- 파일당 최대 3회 재시도, 실패 시 수동 큐로 보류.
- 하루 최대 배치: 50 파일 (토큰 비용 상한 관리).
- 실패율 > 10% 시 pipeline halt + 회고 라운드 (own#19 안건).

## 5. 예상 비용 / 리소스 (추정치, 확정 아님)

- 토큰량 추정: 파일당 평균 input 3~5K + output 3~5K → 1,050 × 10K ≈ **10.5M tokens**.
- Claude sonnet-4-6 기준 비용: 공개 단가 × 10.5M 토큰 (정확 단가는 실행 시점 재확인).
- 인간 리뷰 공수: 파일당 10분 → **약 175시간**, phase 전반에 분산 (phase 당 25~50시간).
- 실행 구간: 2026-04 ~ 2026-12, 9개월 window.

## 6. 리스크 & 대응

| 리스크 | 영향 | 대응 |
|---|---|---|
| 기술 용어 의역 | 내용 의미 회귀 | glossary 강제 + prompt engineering + 샘플링 리뷰 |
| 수식/코드 블록 파괴 | 빌드/증명 체인 파손 | `markdown_ast_diff.py` 차단 게이트 |
| 개발자 진입장벽 상승 | 한국어 네이티브 기여자 이탈 | "한국어 초안 → 자동 번역 PR" 워크플로 제공, 논의 채널은 다국어 허용 |
| 번역 품질 회귀 | 장기적 문서 노이즈 | 주간 샘플링 리뷰 + own#1 HARD 재검증 |
| 동시 에이전트 충돌 | fast-forward 실패 | PR 단위 분리, allowlist 편집 직렬화 (own#19 roadmap-review 의제화) |

## 7. 성공 지표

- **allowlist count**: 1,050 → 0 (2026 Q4 target).
- **CI own1-doc-english-hard**: 100% PASS rate 지속.
- **AST diff 회귀 0건**: 번역 전후 마크다운 구조 동일성.
- **샘플링 리뷰 통과율**: 주간 샘플 N=10 에서 90%+ 기술 용어 정확성.
- **개발자 만족도 설문**: 분기별 실행 (분기 1회).

## 8. Next Actions

- [ ] `tool/batch_translate.py` 프로토타입 (2026-04-27 target)
- [ ] `tool/translation_glossary.json` 초안 (2026-04-27 target)
- [ ] Phase 1 `experiments/` 25파일 파일럿 (2026-05-04 target)
- [ ] Phase 0 완료 확인 및 allowlist 감축 반영 (에이전트 B 결과 승계)
- [ ] 주간 roadmap review 워크플로 연동 (own#19)
- [ ] 공식 실행 proposal 승격 (scoping → execution) 결정은 별도 라운드

## 9. References

- own#1 (HARD): `/Users/ghost/core/n6-architecture/.own`
- allowlist SSOT: `/Users/ghost/core/n6-architecture/tool/own1_legacy_allowlist.json`
- Policy commit (promote to HARD): `2ef79fe7` — `feat(own): promote own#1 to HARD block for new docs`
- Policy commit (elevate on_fail to block): `4b472b0c` — `docs(own): elevate own#1 on_fail to block; add verify runner`
- 동료 proposal: `proposals/kr-ai-grant-2026-strategic-matching-2026-04-24.md`
- 양식 참조: `proposals/r24-31-round-open-2026-04-24.md`, `proposals/kr-ai-grant-2026-strategic-matching-2026-04-24.md`

---

*본 문서는 로드맵 draft 이며, 기술된 phase 목표는 실행 완료가 아닌 target 시점이다. 실제 감축은 각 phase 의 커밋·PR 에 의해서만 확정된다 (own#11 honesty).*
