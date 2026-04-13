# nexus — HEXA-ONLY 도구 통합 워크스페이스

목적: 단일 hexa 바이너리. 계산기·DSE·분석기·HEXA 유틸·대시보드
축: nexus
상위: ../CLAUDE.md
원칙: HEXA-ONLY (AI-NATIVE) — Rust/Python 코드 전면 폐기 (2026-04-12)

## 하위
- origins/           도구 원본 .hexa 소스 (calc/dse/analyze/hexa 유틸 통합)
- scripts/           자동성장·헬스체크·벤치마크 .hexa 19종
- tests/             통합 테스트 (.hexa)
- 진짜 SSOT: $NEXUS/shared/lenses/ HEXA 네이티브 렌즈

## SSOT
- _registry.json    도구/서브커맨드 SSOT (calc 20 + dse 8 + analyze 12 + hexa 4 = 44 흡수)

## 빌드/실행

```
$HEXA nexus/main.hexa                    기본 실행
$HEXA nexus/tests/tests.hexa --verify    전체 테스트
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
- nexus/main.hexa        진입점
- nexus/origins/*/main.hexa  서브커맨드 라우팅

## 절대규칙
- 한글 필수 (.md/주석/커밋)
- HEXA-ONLY (Python/Rust 소스·빌드 매니페스트 금지)
- 도구 추가 시 _registry.json 동기화 필수
- 모든 도구는 $HEXA 로 직접 실행

## 관련 링크
- 루트: ../CLAUDE.md + INDEX.json
- 성장 시스템: GROWTH.md
