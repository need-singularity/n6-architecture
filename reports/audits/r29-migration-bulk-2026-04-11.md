# R29 대량 이관 감사 리포트 — 2026-04-11

- 일자: 2026-04-11
- 규칙: R29 (n6shared/rules/common.json)
- 작업: domains/**/verify_*.hexa (verify.hexa 제외) → nexus/shared/n6/scripts/
- 스크립트: `nexus/scripts/migrate_r29_verify.hexa`
- 선행 리포트: `reports/audits/r29-migration-synbio-2026-04-11.md` (synbio 1건 이관)

## 1. 배경

R29 규칙 본문:

> "계산기/검증/스캐너 .hexa 는 `nexus/shared/n6/scripts/` 에만 작성 —
> 모든 프로젝트 공통 단일 SSOT" (common.json, level ⛔⛔)

선행 synbio 이관 직후 scan 결과 총 207 건 R29 위반 잔존.
본 작업에서 일괄 이관 스크립트 `nexus/scripts/migrate_r29_verify.hexa` 로 전량 이관.

`verify.hexa` (공통 템플릿) 는 이관 대상 **아님**. 각 도메인 `_index.json` +
`CLAUDE.md` 15섹션 템플릿의 필수 구성이다.

## 2. 이관 결과 요약

| 항목 | 값 |
|------|-----|
| 대상 총 건수 | 207 |
| 성공 | 207 |
| 실패 | 0 |
| 네이밍 충돌 | 0 |
| 잔여 (domains/) | 0 |
| md5 일치율 | 207/207 (100%) |
| 백업 경로 | `/tmp/r29_backup_20260411` |

## 3. 축별 집계

| 축 | 이관 건수 |
|----|----------|
| brainwire | 2 |
| cognitive | 16 |
| compute | 35 |
| culture | 14 |
| energy | 15 |
| infra | 43 |
| life | 26 |
| materials | 10 |
| physics | 19 |
| sedi | 10 |
| sf-ufo | 12 |
| space | 5 |
| **합계** | **207** |

## 4. 네이밍 규칙

- 일반 형식: `verify_<도메인>_<stem>.hexa`
- sedi 예외: `verify_sedi_<하위계층>_<stem>.hexa` (sedi 는 3 단계 경로)
- 예시:
  - `domains/life/baking/verify_alien10.hexa` → `verify_baking_alien10.hexa`
  - `domains/physics/higgs/verify_alien10.hexa` → `verify_higgs_alien10.hexa`
  - `domains/brainwire/verify_paper.hexa` → `verify_brainwire_paper.hexa` (축 직속)
  - `domains/sedi/scripts/cern-optics/verify_cern_optics.hexa` → `verify_sedi_cern-optics_cern_optics.hexa`
  - `domains/sedi/sources/verify_hdna_fermion_six.hexa` → `verify_sedi_sources_hdna_fermion_six.hexa`
- 도메인은 파일의 바로 위 디렉토리 basename 으로 정의. sedi 축은 axis prefix 를 추가해 의미 보존.
- 네이밍 충돌 0 건 (207 건 전량 유니크)

## 5. 검증

### 5.1 md5 바이너리 동일성

모든 이관 파일은 이동 전후 md5 일치 검증 완료. 불일치 발생 시 부분 실패로
기록되며 목적지 파일을 제거하여 멱등성 유지.

### 5.2 잔여 R29 위반

```sh
find domains -name 'verify_*.hexa' -not -name 'verify.hexa' 2>/dev/null | wc -l
# 결과: 0
```

✅ **잔여 R29 위반 0 건 — 완전 수렴**

### 5.3 목적지 파일 수

```sh
ls /Users/ghost/Dev/nexus/shared/n6/scripts/verify_*.hexa | wc -l
# 결과: 211
```

## 6. 실패 목록

실패 없음. 전량 성공.

## 7. 규칙 준수 체크리스트

- [x] R1 HEXA-FIRST: 이관 파일 전량 .hexa 유지, 스크립트도 .hexa
- [x] R5 SSOT: 단일 위치 (`nexus/shared/n6/scripts/`) 수렴
- [x] R14: 규칙 본문은 `n6shared/rules/common.json` 단일 진실 (인용만)
- [x] R18 미니멀: 요청 범위만 — 로직 수정 없이 파일 이동만
- [x] R19 SILENT EXIT 금지: 모든 에러 stdout 출력 + 리포트 기록
- [x] R21 블로킹 금지: find/md5/cp 개별 호출로 대형 블로킹 없음
- [x] R22: bash 단일 인터프리터 + 절대경로 shq quoting
- [x] R28: 본 이관 결과는 후속 atlas.n6 흡수 대상
- [x] R29: 전 위반 단일 위치로 수렴, `verify.hexa` 공통 템플릿 보존
- [x] 한글 필수: 본 리포트 한글

## 8. 롤백 안내

백업 파일은 `/tmp/r29_backup_20260411` 에 `<axis>__<domain>__<filename>` 형식으로
보관되어 있다. 복원이 필요하면:

```sh
# 1) 목적지에서 신규 파일 제거
rm /Users/ghost/Dev/nexus/shared/n6/scripts/verify_<도메인>_*.hexa
# 2) 백업에서 원본 경로로 복사
# (도메인 경로는 domains/<axis>/<domain>/ 로 복원)
```

## 9. 도메인 CLAUDE.md 동기화

후속 스크립트 `nexus/scripts/sync_r29_claude_md.hexa` 가 백업 파일명에서
(axis, domain) 그룹을 추출하여 각 도메인 CLAUDE.md 하단에
"외부 검증 스크립트 (R29)" 섹션을 append 했다.

| 항목 | 값 |
|------|-----|
| 도메인 그룹 수 | 189 |
| CLAUDE.md 업데이트 (자동) | 186 |
| 이미 처리됨 (스킵) | 0 |
| CLAUDE.md 없음 (자동 스킵) | 3 |

자동 스크립트로 누락된 3 건은 경로 구조가 달라 수동 보정했다:

- `domains/brainwire/CLAUDE.md` — brainwire 는 축 직속 단일 도메인 (수동 추가 완료)
- `domains/sedi/CLAUDE.md` — sedi 축은 scripts/cern-optics/, sources/ 2 하위 경로를 가지므로 축 수준 CLAUDE.md 에 10 건 모두 append (수동 추가 완료)
- `domains/sedi/scripts/cern-optics/CLAUDE.md` / `domains/sedi/sources/CLAUDE.md` 는 원래 존재하지 않음 (상위 `sedi/CLAUDE.md` 에 통합)

총 도메인 CLAUDE.md 동기화: **189/189 (100%)**

## 10. 후속 작업

1. atlas.n6 자동 흡수 (R28): 이관 결과를 요약 claim 으로 기록
2. weekly_audit.hexa 차주 실행 시 R29 위반 0 재확인
3. 백업 디렉토리 정리 (7일 후): `rm -rf /tmp/r29_backup_20260411`

## 11. 이관 스크립트

- `nexus/scripts/migrate_r29_verify.hexa` — 207 건 대량 이관 (md5 검증 포함)
- `nexus/scripts/sync_r29_claude_md.hexa` — 도메인 CLAUDE.md 자동 동기화

R29 예외: nexus/scripts 는 이관 주체이므로 허용 (공통 규칙은 파일명에
`verify_` prefix 가 있는 것만 대상; 이관 스크립트는 `migrate_`, `sync_` prefix).

---

*본 리포트는 `nexus/scripts/migrate_r29_verify.hexa` 로 자동 생성 후 수동 보정되었습니다.*
