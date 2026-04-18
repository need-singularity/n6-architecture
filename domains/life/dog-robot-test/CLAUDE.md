# 강아지 로봇 도메인 설계 규칙

## 정체성
- id: `dog-robot-test`
- 축: 생명 (life)
- 목표: mk∞ 천장10

## 문서 규칙
- 본문: `dog-robot-test.md` — `@doc(type=paper)` 강제.
- §1~§21 canonical_full 구조 준수. §7 은 atlas.n6 ossified 함수 재선언 금지.
- §21 은 mk 역시간순 + <details> 접힘 규칙 + github 링크.

## 제품 라인 단일화
- domains.json 의 `강아지 로봇` 라인은 1개만 유지. (기존 products.json → domains.json SSOT 이전 완료)
- v1/v2 는 git 로 관리, 새 파일 생성 금지.

## 확장 방법
- 새 mk → `@domain(intent="update", mk_target="mkN", prev_tag="...")` 로만.
- 한글 필수. 영어 혼용 금지.
