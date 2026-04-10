# nexus — 모든 Rust 도구 통합 워크스페이스

목적: 62 크레이트 흡수 단일 바이너리. 계산기·DSE·분석기·HEXA 위임·대시보드
축: nexus
상위: ../CLAUDE.md

## 하위
- src/cmd/calc/      계산기 20종 (optics/energy/gpu/quantum/fusion 등)
- src/cmd/dse/       DSE 8종 (universal/material/solar/fusion/battery/sc/robot)
- src/cmd/analyze/   분석기 12종 (deep-miner/formula-miner/discovery-engine 등)
- src/cmd/hexa/      HEXA 유틸 4종 (rtl/sim/ssh/ready-absorber)
- src/telescope/     렌즈 시스템 (215+ 렌즈)
- src/ouroboros/     자기재귀 엔진 (메타루프/수렴/돌연변이)
- src/growth/        자율성장 시스템
- src/consciousness_bridge/  의식 브릿지
- crates/dashboard/  Axum 웹 대시보드 (port 6600)
- origins/           도구 원본 소스 (구 tools/ 에서 이관, .hexa 47개 + legacy .py 15개)
- scripts/           자동성장·헬스체크·벤치마크 쉘 19종
- tests/             통합 테스트

## SSOT
- _registry.json    도구/서브커맨드 SSOT (calc 20 + dse 8 + analyze 12 + hexa 4 = 44 흡수)

## 빌드/실행

```
cargo build --release                    기본 빌드
cargo build --release --features python  파이썬 바인딩 포함
cargo test                               전체 테스트
```

## CLI 서브커맨드

```
nexus scan <d> | --full     도메인 스캔
nexus verify <v>            검증
nexus calc <domain>         계산기 (optics/energy/gpu/crypto/quantum/carbon/chip-*/...)
nexus dse <kind>            DSE (universal/material/solar/fusion/battery/sc/robot)
nexus analyze <tool>        분석 (deep-miner/formula-miner/discovery-engine/atlas-verifier/...)
nexus hexa <cmd>            HEXA 유틸 (rtl/sim/ssh)
nexus dashboard             웹 대시보드 (port 6600)
```

## API

```
nexus.scan_all() / .analyze() / .n6_check() / .evolve()
```

합의 기준: 3+ 후보 / 7+ 고신뢰 / 12+ 확정

## 디스패처
- src/main.rs       진입점
- src/cmd/mod.rs    서브커맨드 라우팅

## 절대규칙
- 한글 필수 (.md/주석/커밋)
- HEXA-FIRST (.py 금지)
- 도구 추가 시 _registry.json 동기화 필수
- 원본 .hexa 있는 도구는 hexa-lang 으로 위임 실행

## 관련 링크
- 루트: ../CLAUDE.md + INDEX.json
- 대시보드: crates/dashboard/
- 성장 시스템: GROWTH.md
