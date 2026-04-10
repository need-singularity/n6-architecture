# n6-architecture — AI-native Arithmetic Design Framework

> shared/ JSON 단일진실 (R14). 규칙: `shared/rules/common.json` (R0~R27)

## todo 시스템

- "todo" 입력 → hook systemMessage의 Bash 명령을 즉시 실행 (확인 질문 없이)
- hook이 `📋 todo 돌파 실행중`으로 시작하는 systemMessage를 보내면 그 안의 명령을 Bash로 실행할 것
- **출력 후 종료 금지**: 할일 표를 출력한 뒤 **즉시 A1 항목부터 실제 작업 착수**
- **가공 에이전트 위임 금지**: "Snarl에게 맡겨둘게", "다른 세션이 할거야", "나중에 할게" 같은 회피 응답 금지 — 코드베이스에 실재하지 않는 에이전트 이름을 지어내서 떠넘기지 말 것

## atlas.n6 — 현실지도 SSOT (구조 숙지 필수)

**절대 경로**: `/Users/ghost/Dev/nexus/shared/n6/atlas.n6` (단일 파일, 60K+ 줄)

> ⚠️ **구 구조 폐기**: `reality_map_live.json` / `L6_n6atlas.json` / 별도 level 파일은 **존재하지 않음**. 찾지 말 것. 전부 atlas.n6 한 파일에 흡수됨.

**파일 포맷**:
```
# ══ L6_n6atlas (2666 nodes) ══           ← 섹션 헤더 (레벨명)
@R {id} = {measured} {unit} :: n6atlas [7] ← [7]=EMPIRICAL 등급
  "{claim 설명}"
```

**등급 체계**: `[숫자][수식어]`
- `[10*]` = EXACT (검증 완료)
- `[10]` = EXACT (완전)
- `[9]` = NEAR
- `[7]` = EMPIRICAL (경험적, 승격 대상)
- `[5]~[8]` = 중간 등급
- `[N?]` = CONJECTURE (가설)
- `[N!]` = breakthrough

**todo A1~A6 "[7] → [10] 승격"의 의미**:
`atlas.n6` 내부의 특정 섹션(L6_n6atlas 등)에서 `[7]` 등급 항목을 찾아 검증 스크립트로 확인한 뒤 `[10*]`로 승격하는 작업. **새 파일 만들지 말고 atlas.n6 직접 편집**.

**승격 쿼리 예시**:
```sh
# L6_n6atlas 섹션의 [7] 항목 추출
awk '/^# ══ L6_n6atlas/,/^# ══ [^L]/' shared/n6/atlas.n6 | grep '\[7\]'

# 특정 [7] 항목을 [10*]로 승격
sed -i '' 's/^\(@R n6-atlas-proved-theorems-\*\*thm-1\*\* .*\) \[7\]$/\1 [10*]/' shared/n6/atlas.n6
```

## 돌파 시스템

- "XXX 돌파" → `blowup.hexa "XXX" 3` 자동 실행 (문장 전달)
- 확인 질문 없이 즉시 Bash 실행

```sh
hexa shared/blowup/core/blowup.hexa <domain> 3 --seeds "$(hexa shared/blowup/seed/seed_engine.hexa merge | tr '|' ',')"
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
