# R29 이관 감사 리포트 — synbio/verify_alien10.hexa

- 일자: 2026-04-11
- 규칙: R29 (n6shared/rules/common.json)
- 작업: domains/life/synbio/verify_alien10.hexa → nexus/shared/n6/scripts/verify_synbio_alien10.hexa
- 담당: 세션 에이전트 (R29 별건 이관)
- 연계 리포트: 병합 감사 리포트 (R29 위반 별건)

## 1. 배경

R29 규칙 본문:

> "계산기/검증/스캐너 .hexa는 `nexus/shared/n6/scripts/` 에만 작성 —
> 모든 프로젝트 공통 단일 SSOT" — (common.json, level ⛔⛔)

금지 경로 패턴:
- `<any-project>/theory/**/*.hexa`
- `<any-project>/predictions/*.hexa`
- `<any-project>/proofs/*.hexa`
- `<any-project>/breakthroughs/*.hexa`
- `papers/**/*.hexa`
- `hexa-lang/docs/*.hexa`

허용 단일 위치:
- `/Users/ghost/Dev/nexus/shared/n6/scripts/`

`domains/life/synbio/verify_alien10.hexa` 는 도메인 polder 내부에 존재하는
검증용 .hexa 파일로 R29 의 포괄 규칙("계산기/검증/스캐너") 에 해당하여 위반
상태였다. 병합 감사 리포트에서 별건으로 기록되어 본 리포트로 분리 처리한다.

단, 각 도메인 폴더의 `verify.hexa` (파일명이 정확히 `verify.hexa`) 는
"범용 n=6 정체성 검증 공통 템플릿" 으로 본 이관 작업 범위에서 제외한다.
이는 도메인 `_index.json` + `CLAUDE.md` 15섹션 템플릿의 필수 구성이다.

## 2. 이관 내역

| 항목 | 값 |
|------|-----|
| 원본 경로 | `/Users/ghost/Dev/n6-architecture/domains/life/synbio/verify_alien10.hexa` |
| 신규 경로 | `/Users/ghost/Dev/nexus/shared/n6/scripts/verify_synbio_alien10.hexa` |
| 파일 크기 | 10 줄 (STUB) |
| md5 (이관 전) | `ee0633a4dedc488b56853e25e9adf900` |
| md5 (이관 후 바이너리 동일성 확인) | `ee0633a4dedc488b56853e25e9adf900` |
| 네이밍 규칙 | `verify_<도메인>_<대상>.hexa` (prefix 에 도메인 명시) |
| 원본 삭제 | 완료 (rm 후 `ls` 검증) |
| 헤더 주석 업데이트 | 이관 이력 (2026-04-11 R29) 추가 |
| CLAUDE.md 업데이트 | `domains/life/synbio/CLAUDE.md` 반영 |

### 2.1 파일 동일성 검증

```
MD5 (domains/life/synbio/verify_alien10.hexa) = ee0633a4dedc488b56853e25e9adf900
MD5 (nexus/shared/n6/scripts/verify_synbio_alien10.hexa) = ee0633a4dedc488b56853e25e9adf900
```

이관 직후 md5 일치 확인. 이후 헤더 주석만 이관 이력 반영으로 업데이트했으며,
STUB 본문 로직은 변경하지 않았다.

### 2.2 CLAUDE.md 업데이트

`domains/life/synbio/CLAUDE.md` 는 다음 사항이 반영되었다:

- "파일" 섹션: `verify.hexa` 는 "범용 n=6 정체성 검증 (공통 템플릿,
  이동 대상 아님)" 으로 명시
- 신설 "외부 검증 스크립트 (R29)" 섹션: `verify_synbio_alien10.hexa` 의
  신규 절대경로 + 실행 명령 + 본 리포트 참조 포함
- "사용법" 섹션: alien10 검증 실행 경로 업데이트
- "규칙" 섹션: R29 조항 명시 — 이 폴더는 `verify.hexa` (공통 템플릿) 만 허용

## 3. R29 위반 후보 목록 (이관 미실행)

`find domains -name "verify_*.hexa" -not -name "verify.hexa"` 결과,
synbio 외에도 도메인 폴더 내부 검증 .hexa 가 다수 존재한다. 본 리포트는
synbio 건만 처리하며 나머지는 이관 미실행 상태로 목록만 기록한다.

### 3.1 축별 집계 (총 206 건, synbio 제외)

| 축 | 개수 |
|----|------|
| infra | 43 |
| compute | 35 |
| life (synbio 제외) | 25 |
| physics | 19 |
| cognitive | 16 |
| energy | 15 |
| culture | 14 |
| sf-ufo | 12 |
| sedi | 10 |
| materials | 10 |
| space | 5 |
| brainwire | 2 |
| **합계** | **206** |

참고: 위 206 건에 이번 이관된 synbio 1 건을 포함하면 병합 감사 리포트 시점
총 207 건이었다.

### 3.2 파일명 패턴 유형

- `verify_alien10.hexa` (가장 많음): 각 도메인의 "alien-10 검증" 스텁
- `verify_n6.hexa`: 각 도메인의 n=6 검증 스텁
- `verify_<domain-specific>.hexa`: sc_exact, cancer10, bt461_470,
  cern_optics_v2, hdna_fermion_six, hexa_starship 등 도메인 고유 스크립트
- `verify_paper.hexa`, `verify_paper_p002.hexa`: brainwire 논문 연결 검증
- `verify_all_techniques_n6.hexa`, `verify_ai_products_alien10.hexa`:
  compute/ai-efficiency 의 종합 검증

### 3.3 이관 전략 (권장)

후속 작업자 참고:

1. **일괄 이관 스크립트 작성 권장** — 207 건은 수작업 불가능 규모.
   `nexus/shared/n6/scripts/migrate_r29_verify.hexa` 작성 후 일괄 실행.
2. **네이밍 규칙**: `verify_<도메인>_<대상>.hexa` (본 synbio 이관과 동일).
   예: `verify_baking_alien10.hexa`, `verify_cognitive_hexa-mind_n6.hexa`
3. **충돌 처리**: 동일 파일명이 여러 도메인에 존재 → prefix 에 도메인
   필수 (충돌 207 건 전수 발생).
4. **CLAUDE.md 동기화**: 각 도메인 CLAUDE.md 에 외부 검증 경로 링크 추가.
5. **md5 검증 필수**: 이관 전후 바이너리 동일성 보장.
6. **원본 로직 보존**: STUB 파일은 헤더만 업데이트, 본문 로직 무변경.
7. **대량 이관 시 단일 atlas.n6 흡수 보고**: R28 에 따라 이관 요약은
   atlas.n6 에 기록.

### 3.4 위반 후보 전체 파일 목록

전체 206 건 목록은 다음 명령으로 재생성 가능:

```sh
cd /Users/ghost/Dev/n6-architecture
find domains -name "verify_*.hexa" -not -name "verify.hexa" 2>/dev/null | sort
```

(목록 생략 — 위 명령으로 복원 가능)

## 4. 규칙 준수 체크리스트

- [x] R1 HEXA-FIRST: 이관 파일은 .hexa 형식 유지
- [x] R5 SSOT: 단일 위치 이관 완료
- [x] R14: 규칙 본문은 n6shared/rules/common.json 이 단일 진실
- [x] R18 미니멀: 요청 범위 (synbio 1 건 이관) 외 다른 도메인은 목록만
- [x] R28: 본 이관 결과는 후속 atlas.n6 흡수 대상
- [x] R29: synbio 건 이관 완료, nexus/shared/n6/scripts/ 에만 위치
- [x] 한글 준수: 본 리포트 한글
- [x] HEXA-FIRST: .py 생성 0건 (신규 생성 없음)

## 5. 결과

1. `/Users/ghost/Dev/nexus/shared/n6/scripts/verify_synbio_alien10.hexa` — 생성
2. `/Users/ghost/Dev/n6-architecture/domains/life/synbio/verify_alien10.hexa` — 삭제
3. `/Users/ghost/Dev/n6-architecture/domains/life/synbio/CLAUDE.md` — 업데이트
4. `/Users/ghost/Dev/n6-architecture/reports/audits/r29-migration-synbio-2026-04-11.md` — 본 리포트
5. R29 위반 후보 잔여: 206 건 (축별 집계 섹션 3.1 참조)

후속 작업: 병합 감사 리포트에 R29 위반 잔여 206 건 건을 단일 이관
이니셔티브로 이관하기 위한 일괄 스크립트 작성 필요. 스크립트 위치는
R29 에 따라 `nexus/shared/n6/scripts/migrate_r29_verify.hexa` 로 한다.
