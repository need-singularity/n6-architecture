---
name: HEXA-LANG NEXUS-6 공진화 요구사항
description: HEXA-LANG은 NEXUS-6 성장과 동기화되어 함께 진화해야 함 — 렌즈 추가 → 패스 추가, 발견 → 최적화 규칙
type: project
---

HEXA-LANG/HEXA-IR은 NEXUS-6와 공진화(co-evolution)해야 한다.

**Why:** NEXUS-6가 새 렌즈/패턴/상수를 발견하면, 이것이 컴파일러의 최적화 패스, 타입 시스템, 증명 규칙에 자동 반영되어야 함. 언어가 정적이면 NEXUS-6 성장의 가치가 반감됨.

**How to apply:**
- NEXUS-6 렌즈 추가 → HEXA-IR opt/ 패스 자동 생성 트리거
- NEXUS-6 상수 발견 → util/n6.rs 자동 갱신 → 컴파일러 전 모듈 반영
- 수렴 루프(convergent_refinement.py) 결과 → 자동 anomaly 수정
- OUROBOROS 진화 사이클에 HEXA-IR을 포함
- 성장 데몬이 컴파일러 코드도 스캔/개선
