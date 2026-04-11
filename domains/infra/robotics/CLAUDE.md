# robotics — CLAUDE 가이드

축: **infra**

## 파일
- `robotics.md` — 15섹션 통합 본문
- 검증코드는 도메인 본문 .md 에 ```python 블록으로 임베드됨
- `CLAUDE.md` — 본 가이드

## 사용법
- 본문 읽기: 섹션 2 목표 → 3 가설 → 4 BT → 7 검증
- 검증 실행: 도메인 본문 .md 내 ```python 블록 참조
- 진화 단계: 섹션 9 Mk.I~V
- 예측 추적: 섹션 10

## BT 연결
- `docs/breakthrough-theorems.md` BT-* 참조
- 돌파 발생 시: `blowup.hexa <d> 3` + `growth_bus.jsonl`

## 규칙
- 원본 섹션 유지, 신규는 해당 섹션 뒤에 추가
- 레거시는 부록 B에만
- 한글 필수 (R-한글)

## 외부 검증 스크립트 (R29 — nexus/shared/n6/scripts/ SSOT)

2026-04-11 R29 이관: 이 폴더의 `verify_*.hexa` (verify.hexa 제외) 는
R29 규칙에 따라 `/Users/ghost/Dev/nexus/shared/n6/scripts/` 로 이관되었습니다.

- 이관 참조: `reports/audits/r29-migration-bulk-2026-04-11.md`
- verify.hexa 는 N62 규칙에 따라 삭제됨. 검증코드는 도메인 본문 .md 에 임베드.

### 이관된 파일

- `verify_robotics_alien10.hexa` → `/Users/ghost/Dev/nexus/shared/n6/scripts/verify_robotics_alien10.hexa`
  - 실행: `hexa /Users/ghost/Dev/nexus/shared/n6/scripts/verify_robotics_alien10.hexa`

