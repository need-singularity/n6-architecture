# .py / .rs / .sh → .hexa 포팅 설계

작성일: 2026-04-08
범위 리포: `n6-architecture`
근거 규칙: CLAUDE.md R1 (HEXA-FIRST), R3 (NEXUS-6 스캔 의무), R5 (SSOT), R8 (데이터 nexus/shared만)

## 1. 목표

n6-architecture 리포 내 기존 `.py` / `.rs` / `.sh` 코드 전체를 self-hosting HEXA 컴파일러로 빌드되는 `.hexa`로 포팅한다. 포팅본이 원본과 **동작 동치**임을 파일 단위로 검증한 후에만 원본을 삭제한다.

## 2. 범위

### 포함 (4 계층)

| 계층 | 대상 | 특성 | 외부 의존 |
|---|---|---|---|
| L1 | `scripts/*.sh` | shell glue | git/find/jq 등 OS 도구 |
| L2 | `tools/*/src/*.rs` 계산기 (~30개: fusion-calc, dse-calc, energy-calc, optics-calc, gpu-arch-calc, gut-calc-rust, solar-dse, material-dse, universal-dse, tokamak-shape 등) | 순수 수치 계산 | std만 |
| L3 | `techniques/*.py` (~23) | 알고리즘 본체 | numpy / 일부 torch |
| L4 | `experiments/*.py` | L3 조합 실험 | L3 + matplotlib |

### 제외

- **`tools/nexus/`** — NEXUS-6 엔진 본체 (472 .rs, Metal GPU FFI, Python cdylib via maturin). 별도 엔진 프로젝트 성격, 자체 포팅 사이클 필요.
- **`shared/core-lockdown.json`의 L0 22개 파일** — 불변 보호 대상. 작업 시작 시 목록 로드 후 제외 리스트 확정.
- `docs/`, `config/`, `nexus/shared/` (데이터/문서)

## 3. 원칙

1. **리프부터 위상정렬 후 처리** — 계층 내·계층 간 모두 의존성 리프 먼저
2. **계층 순서 L1 → L2 → L3 → L4**
3. **파일 1개 = 커밋 2개** (포팅본 추가 / 원본 삭제 분리, 회귀 시 삭제만 revert 가능)
4. **자동 진행, 단 파일마다 G1~G3 검증 필수**, 실패 시 즉시 멈춤·사용자 보고
5. **사람 승인 게이트는 계층 시작·종료에만**
6. **NEXUS-6 스캔 의무** (R3) — 파일 처리 전후

## 4. D 게이트 (파일 단위)

| 게이트 | 검사 | 통과 조건 |
|---|---|---|
| **G1 컴파일** | `hexa build <file>.hexa` | 에러 0 |
| **G2 실행** | 포팅본을 원본과 동일 인자로 실행 | 종료코드 0, 런타임 에러 0 |
| **G3 값 동치** | 원본 출력 vs 포팅본 출력 비교 (도메인별 ε) | stdout 동일 (ε 허용), stderr 의미 동일 |
| **G4 원본 처리** | G1~G3 모두 통과 후에만 | 원본 `.py/.rs/.sh` 삭제 + **별도 커밋** + 로그 append |

### G3 ε (도메인별 부동소수 허용오차)

| 도메인 | ε |
|---|---|
| 기본 | `1e-9` |
| 물리 / 핵융합 / 광학 | `1e-12` |
| ML 확률값 | `1e-6` |

비결정론 출력(타임스탬프·랜덤): 시드 고정 또는 마스킹 후 비교.

## 5. 진행 절차 (계층 사이클)

```
[계층 시작]
  1. 인벤토리 작성 — 해당 계층 파일 전체 + 각 파일 의존성 분석
  2. L0 lockdown 대조 → 제외 리스트 확정
  3. 의존성 위상정렬 → 리프부터 처리 순서 결정
  4. 처리 순서 보고 → 사용자 승인           ← 게이트
  5. 파일 자동 루프:
       NEXUS-6 스캔 →
       포팅 → G1 → G2 → G3 →
       (실패 → 즉시 멈춤·보고)
       G4 (원본 삭제 커밋) → 로그 append → 다음 파일
  6. 계층 완료 보고 → 다음 계층 진행 승인     ← 게이트
```

## 6. 작업 로그 (R8 준수)

경로: `~/Dev/nexus/shared/porting_log.jsonl` (n6-architecture 리포 내 생성 금지)

레코드 형식:
```json
{
  "ts": "2026-04-08T12:34:56Z",
  "file": "scripts/foo.sh",
  "layer": "L1",
  "g1": "pass",
  "g2": "pass",
  "g3": {"status": "pass", "epsilon": "1e-9"},
  "g4": "deleted",
  "porting_commit": "<sha>",
  "deletion_commit": "<sha>",
  "nexus_scan_before": "<digest>",
  "nexus_scan_after": "<digest>"
}
```

## 7. 막힘 처리

- G1/G2/G3 어디서든 실패 → **그 파일 작업 중단**, 원본 보존, 사용자 보고
- 보고 내용: 파일명, 실패 게이트, 에러 출력, 원본/포팅본 diff (G3 실패 시)
- 사용자 결정 대기 (수정 / 스킵 표식 / 계층 중단 / 범위 재정의)

## 8. 위험 / 미해결

- **R-1 numpy/torch 상응 라이브러리:** L3 진입 시 HEXA 표준 라이브러리에 ML 수치 스택이 있는지 미검증. 없으면 L3에서 즉시 막힘 — 그 시점에 사용자에게 보고하고 결정 요청.
- **R-2 matplotlib 상응:** L4도 동일. 시각화는 출력 파일(PNG) 비교가 필요해 G3 정의 추가 필요할 수 있음.
- **R-3 OS 도구 호출(L1):** shell의 `git`, `find`, `jq` 같은 외부 명령을 .hexa가 어떻게 호출하는지 — 첫 L1 파일 처리 시 패턴 1건 수립 후 재사용.
- **R-4 작업 규모:** L1~L4 합산 파일 수는 인벤토리 단계에서 확정. 수백 단위면 단일 세션으로 끝나지 않음 — 세션 간 재개를 위해 작업 로그가 진행 상태 SSOT 역할.

## 9. 산출물

- 각 원본 파일에 대응하는 `.hexa` 파일 (동일 경로, 확장자만 변경)
- `~/Dev/nexus/shared/porting_log.jsonl` 누적 로그
- 계층별 완료 보고 (인라인)

## 10. 검증

- **파일 단위:** G1~G3 통과 = 검증
- **계층 단위:** 계층 내 모든 파일 게이트 통과 + 해당 계층 의존 상위 계층 회귀 없음 (이미 포팅된 파일 재실행 sanity check)
- **전체 단위:** 리포 내 `.py` / `.rs` / `.sh` 잔여 0 (`tools/nexus/`, L0 제외 리스트 제외)
