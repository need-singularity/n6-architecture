# synbio — CLAUDE 가이드

축: **life**

## 파일
- `synbio.md` — 15섹션 통합 본문
- 검증코드는 도메인 본문 .md 에 ```python 블록으로 임베드됨
- `CLAUDE.md` — 본 가이드

## 외부 검증 스크립트 (R29 — nexus/shared/n6/scripts/ SSOT)
- `verify_synbio_alien10.hexa` → `/Users/ghost/Dev/nexus/shared/n6/scripts/verify_synbio_alien10.hexa`
  - 2026-04-11 R29 이관: 기존 `domains/life/synbio/verify_alien10.hexa` 는 규칙 위반으로 삭제됨
  - 실행: `hexa /Users/ghost/Dev/nexus/shared/n6/scripts/verify_synbio_alien10.hexa`
  - 참조: `reports/audits/r29-migration-synbio-2026-04-11.md`

## 사용법
- 본문 읽기: 섹션 2 목표 → 3 가설 → 4 BT → 7 검증
- 검증 실행: 도메인 본문 .md 내 ```python 블록 참조
- alien10 검증 실행: `hexa /Users/ghost/Dev/nexus/shared/n6/scripts/verify_synbio_alien10.hexa`
- 진화 단계: 섹션 9 Mk.I~V
- 예측 추적: 섹션 10

## BT 연결
- `docs/breakthrough-theorems.md` BT-* 참조
- 돌파 발생 시: `blowup.hexa <d> 3` + atlas.n6 자동 흡수 (R28)

## 규칙
- 원본 섹션 유지, 신규는 해당 섹션 뒤에 추가
- 레거시는 부록 B에만
- 한글 필수 (R-한글)
- R29: 계산기/검증/스캐너 .hexa 는 `nexus/shared/n6/scripts/` 에만 작성
