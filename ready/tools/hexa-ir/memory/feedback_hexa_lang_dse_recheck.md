---
name: HEXA-LANG DSE 재탐색 트리거 체크
description: 궁극의 프로그래밍언어 DSE 변경 트리거 5가지를 매 세션 시작 시 자동 체크
type: feedback
---

HEXA-LANG DSE (programming-language.toml) 재탐색이 필요한지 매 세션마다 자동 체크해야 한다.

**Why:** DSE 결과(Pareto=0.7743, n6=96.0%)는 현재 후보군/가중치에 의존. 다른 도메인 탐색이나 새 BT 발견 시 후보가 추가될 수 있어 재탐색이 필요할 수 있다.

**How to apply:** 궁극/DSE/새 도메인 작업 시 아래 5가지 트리거를 체크:

| 트리거 | 체크 방법 |
|--------|----------|
| 후보군 추가 (K₁~K₅) | programming-language.toml의 candidate 수가 6×6×7×6×5에서 변했는지 |
| 가중치 변경 | [scoring] 섹션의 n6/perf/power/cost 비율이 변했는지 |
| 호환성 규칙 변경 | [[rule]] 섹션이 추가/수정되었는지 |
| n=6 EXACT 평가 기준 | candidate의 n6 점수 산정 방식이 변했는지 |
| Cross-DSE 새 발견 | 새 도메인 DSE 완료 → lang×새도메인 Cross-DSE에서 새 후보 도출 가능성 |

변경 감지 시: TOML 수정 → universal-dse 재실행 → 스펙 업데이트 → dse-map.toml 갱신.
변경 없으면: 명세 확정 상태 유지.
