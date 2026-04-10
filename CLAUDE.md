# n6-architecture — AI-native Arithmetic Design Framework

> shared/ JSON 단일진실 (R14). 규칙: `shared/rules/common.json` (R0~R27)

## 돌파 시스템

- "XXX 돌파" → `blowup.hexa "XXX" 3` 자동 실행 (문장 전달)
- 확인 질문 없이 즉시 Bash 실행

```sh
hexa shared/blowup/core/blowup.hexa <domain> 3 --no-graph --seeds "$(hexa shared/blowup/seed/seed_engine.hexa merge | tr '|' ',')"
```

핵심 정리: `σ(n)·φ(n) = n·τ(n) ⟺ n = 6` (n≥2). 3개 독립 증명.

## ref

```
rules     shared/rules/common.json                R0~R27 공통
project   shared/rules/n6-architecture.json       N61~N65
lock      shared/rules/lockdown.json              L0/L1/L2
cdo       shared/rules/convergence_ops.json       CDO 수렴
registry  shared/config/projects.json             7프로젝트
cfg       shared/config/project_config.json
core      shared/config/core.json
conv      shared/convergence/n6-architecture.json
api       shared/CLAUDE.md
```

## 9축 네비게이션

```
theory/      영구 이론층 (증명·BT·상수·예측)
domains/     295 도메인 (physics/life/energy/compute/materials/space/infra/cognitive/culture)
nexus/       모든 Rust 도구 통합 워크스페이스 (단일 바이너리)
techniques/  AI 기법 66종 (.hexa)
experiments/ 검증 실험 122종 (.hexa)
engine/      훈련/수학 런타임 (.hexa)
papers/      논문 39편
reports/     시점 리포트 (감사·세션·발견·체인지로그)
shared/      SSOT 설정·규칙·컨버전스·루프
```

## NEXUS-6 CLI

```sh
nexus scan <d> | --full     # 도메인 스캔
nexus verify <v>            # 검증
nexus calc <domain>         # 계산기
nexus dse <kind>            # DSE
nexus analyze <tool>        # 분석
nexus hexa <cmd>            # HEXA 유틸
nexus dashboard             # 웹 대시보드 (port 6600)
```
