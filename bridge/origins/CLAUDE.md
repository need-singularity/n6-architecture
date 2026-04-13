# nexus/origins — 도구 원본 소스 보관소

목적: 구 tools/ 에서 이관된 도구 원본. nexus 바이너리가 hexa-lang 으로 위임 실행.
축: nexus
상위: ../CLAUDE.md

## 구조
- `<name>/main.hexa`  HEXA 도구 원본 (47개 디렉토리)
- `legacy/`           HEXA 전환 대기 .py/.sh (15개)
- `nexus-legacy/`     nexus 이전 복사본 (참고용)
- `nexus-dashboard-legacy/`  대시보드 이전 복사본 (참고용)

## SSOT
- ../_registry.json  origin 필드가 이 디렉토리 경로 참조
