# CHIP-P1-4 — 6 도구 브릿지 통합 요약

P1 로드맵 CHIP-P1-4 항목. 6개 계산기/DSE 도구를 `bridge/_registry.json` 에 통합하여 `nexus` CLI 에서 직접 라우팅 가능하게 만든다.

- 완료: 2026-04-14
- 원칙: HEXA-ONLY (Rust 재구현 보류, 모든 도구는 hexa 바이너리로 직접 위임)
- 통합 SSOT: `bridge/_registry.json`
- 라우팅 방식: `nexus <category> <name>` → `$HEXA bridge/origins/<origin>/main.hexa`
- 상위 규칙: `bridge/CLAUDE.md` (HEXA-ONLY 워크스페이스)

## 통합 결과 표

| 도구 | 슬롯 | 카테고리 | origin (hexa 소스) | 호출 | 상태 |
|------|------|----------|--------------------|------|------|
| crypto-calc | calc.crypto | 계산기 | bridge/origins/crypto-calc/main.hexa | `nexus calc crypto` | integrated |
| interconnect-calc | calc.interconnect | 계산기 | bridge/origins/interconnect-calc/main.hexa | `nexus calc interconnect` | integrated |
| lang-dse | dse.lang | DSE | bridge/origins/universal-dse/domains/programming-language.toml | `nexus dse lang` | integrated (신규) |
| gpu-arch-calc | calc.gpu-arch | 계산기 | bridge/origins/gpu-arch-calc/main.hexa | `nexus calc gpu-arch` | integrated |
| chip-n6-calc | calc.chip-n6 | 계산기 | bridge/origins/chip-n6-calc/main.hexa | `nexus calc chip-n6` | integrated |
| semiconductor-calc | calc.semiconductor | 계산기 | bridge/origins/semiconductor-calc/main.hexa | `nexus calc semiconductor` | integrated |

## 도구별 요약

### 1. crypto-calc — 암호/해시 계산기
- 역할: 암호학적 해시, 서명, KDF, 대칭/비대칭 라운드 비용 등 암호 원시연산 파라미터 n=6 정렬 계산.
- 슬롯: `calc.crypto`
- 호출: `nexus calc crypto`
- 상태: 기존 origin 존재, 레지스트리에 invocation/status 메타 추가.

### 2. interconnect-calc — 인터커넥트 계산기
- 역할: 칩 내부/패키지/다이-투-다이 인터커넥트 대역폭·레이턴시·면적·전력 trade-off 계산.
- 슬롯: `calc.interconnect`
- 호출: `nexus calc interconnect`
- 상태: 기존 origin 존재, 메타 보강.

### 3. lang-dse — HEXA-LANG DSE
- 역할: 궁극 프로그래밍언어 DSE. BT-50 IEEE-754 ladder, BT-52 6-phase 컴파일러, BT-56 complete LLM 통합 탐색. 5 레벨(Foundation/Process/Core/Engine/System) n6/perf/power/cost 4축 스코어링.
- 슬롯: `dse.lang` (신규)
- 호출: `nexus dse lang`
- 소스: `bridge/origins/universal-dse/domains/programming-language.toml` (별도 origin 크레이트 없이 universal-dse 엔진이 해당 도메인 TOML 을 로딩)
- 상태: 신규 슬롯 추가로 통합.

### 4. gpu-arch-calc — GPU 아키텍처 계산기
- 역할: SM/CU 구성, 캐시 계층, 메모리 대역폭, 피크 TFLOPS, 루프라인 상의 n=6 정렬 비율 계산. (chip_design_n6_analysis.hexa 함께 배치)
- 슬롯: `calc.gpu-arch`
- 호출: `nexus calc gpu-arch`
- 상태: 기존 origin 존재, 메타 보강.

### 5. chip-n6-calc — n=6 칩 계산기
- 역할: n=6 원리 기반 칩 파라미터 검증 (타일 개수, 레지스터 파일, 파이프 단, 전력-성능-면적 6축 균형).
- 슬롯: `calc.chip-n6`
- 호출: `nexus calc chip-n6`
- 상태: 기존 origin 존재, 메타 보강.

### 6. semiconductor-calc — 반도체 계산기
- 역할: 공정 노드, ITRS/IRDS 스케일링, 트랜지스터 밀도·지연·누설 계산.
- 슬롯: `calc.semiconductor`
- 호출: `nexus calc semiconductor`
- 상태: 기존 origin 존재, 메타 보강.

## 라우팅 방식

`bridge/_registry.json` 은 HEXA-ONLY 디스패처 SSOT 이다. 별도 바이너리 수정 없이 config 만 갱신한다.

- 도구 등록 포맷: `{ desc, src, origin, invocation, status }`
- 실행 경로: `nexus <category> <name>` → `$HEXA bridge/origins/<origin>` 위임
- `_runtime` 필드로 hexa 호출 약속 명시.
- 5 도구는 이전부터 calc 슬롯에 등록되어 있었고, 본 작업에서 invocation/status 메타 및 `p1_chip_p1_4: true` 플래그를 추가.
- lang-dse 는 `dse.lang` 슬롯을 신규 추가 (universal-dse 도메인 TOML 재사용).

## 검증 항목

- 6/6 도구 SSOT 등록 확인 (`bridge/_registry.json` `p1_chip_p1_4.tools`)
- 6/6 origin 경로 실제 파일 존재 (5 main.hexa + 1 programming-language.toml)
- 이모지 없음. 한글 문서. 바이너리/컴파일 산출물 미수정.
- 로드맵 연동: `$NEXUS/shared/roadmaps/n6-architecture.json` CHIP-P1-4 status done + result_2026_04_14.
