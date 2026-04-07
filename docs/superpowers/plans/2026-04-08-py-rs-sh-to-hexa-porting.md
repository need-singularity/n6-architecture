# py/rs/sh → hexa 포팅 구현 계획

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** n6-architecture 리포 내 모든 `.py`/`.rs`/`.sh`(`tools/nexus/` + L0 제외)를 self-hosting HEXA로 포팅, 파일별 D 게이트(컴파일→실행→값동치) 통과 후 원본 삭제.

**Architecture:** 4 계층(L1 shell → L2 rust 계산기 → L3 python techniques → L4 experiments) 위상정렬 후 리프부터. 파일 1개당 포팅 추가 커밋 + 원본 삭제 커밋 분리. 진행 로그는 `~/Dev/nexus/shared/porting_log.jsonl`.

**Tech Stack:** HEXA (self-hosting), `hexa build`, NEXUS-6 스캔, git.

**스펙:** `docs/superpowers/specs/2026-04-08-py-rs-sh-to-hexa-porting-design.md`

---

## 사전 준비 (Phase 0)

### Task 0.1: L0 제외 리스트 확정

**Files:**
- Read: `~/Dev/nexus/shared/core-lockdown.json`
- Create: `~/Dev/nexus/shared/porting_exclude.json`

- [ ] **Step 1: L0 22개 파일 목록 추출**

`shared/core-lockdown.json`에서 L0 등급 파일 경로 전부 추출. 그 중 n6-architecture 리포 내 `.py/.rs/.sh` 경로만 필터링.

- [ ] **Step 2: 제외 리스트 파일 작성**

`~/Dev/nexus/shared/porting_exclude.json`:
```json
{
  "scope": "n6-architecture",
  "created_at": "2026-04-08",
  "exclusions": {
    "tools_nexus": ["tools/nexus/**"],
    "l0_lockdown": ["<L0 파일 경로 리스트>"]
  }
}
```

- [ ] **Step 3: 검증**

리스트가 비어있지 않고 JSON 파싱 가능 확인.

### Task 0.2: 작업 로그 파일 초기화

- [ ] **Step 1: porting_log.jsonl 존재 확인 / 생성**

경로: `~/Dev/nexus/shared/porting_log.jsonl`. 없으면 빈 파일 생성. 있으면 마지막 레코드 읽어 재개 지점 파악.

### Task 0.3: HEXA 빌드 명령 검증

- [ ] **Step 1: `hexa build` 명령 동작 확인**

가장 단순한 hello.hexa 1개 작성 후 `hexa build hello.hexa` 실행. 성공 종료코드 0 확인. 실패 시 → 사용자에게 보고하고 전체 작업 중단 (포팅 자체 불가).

- [ ] **Step 2: NEXUS-6 스캔 명령 확인**

`nexus scan n6-architecture` 동작 확인. 결과 digest 추출 방식 확정.

---

## L1: scripts/*.sh 포팅

### Task L1.1: 인벤토리 + 위상정렬

**Files:**
- Read: `scripts/*.sh`
- Create: `~/Dev/nexus/shared/porting_inventory_l1.json`

- [ ] **Step 1: 파일 목록 수집**

`scripts/` 하위 모든 `.sh` 파일 경로 수집. L0 제외 리스트 대조 후 제외 항목 표기.

- [ ] **Step 2: 의존성 분석**

각 `.sh`가 호출하는 다른 `.sh` 추출 (`source`, `bash <file>`, 직접 실행). 인접 리스트 작성.

- [ ] **Step 3: 위상정렬**

리프(다른 .sh를 호출하지 않는 것)부터 정렬. 사이클 있으면 보고.

- [ ] **Step 4: 인벤토리 파일 작성**

```json
{
  "layer": "L1",
  "total": <n>,
  "excluded": [...],
  "order": [{"file":"...","depends_on":[],"epsilon":"1e-9","external_cmds":["git","jq"]}, ...]
}
```

- [ ] **Step 5: 사용자 보고 + 승인 대기 (계층 시작 게이트)**

처리 순서, 외부 명령 사용 통계, 예상 작업량을 출력. 승인 후에만 다음 태스크 진행.

### Task L1.2: 첫 파일 — 외부 명령 호출 패턴 수립

L1의 가장 단순한 리프 1개를 골라 .hexa 변환. 이 파일 하나로 "shell 외부 명령(git, find, jq)을 .hexa에서 호출하는 방법"의 표준 패턴 확립.

- [ ] **Step 1: NEXUS-6 스캔 (before)**

`nexus scan n6-architecture` 실행, digest 저장.

- [ ] **Step 2: 원본 .sh 읽기 + 의도 파악**

원본 파일 전체 읽기. 사용 외부 명령, 인자, stdout 패턴 식별.

- [ ] **Step 3: .hexa 포팅본 작성**

같은 디렉토리에 동일 basename `.hexa`로 작성. HEXA의 외부 프로세스 호출 API 사용 (없으면 그 시점에 멈춤·보고).

- [ ] **Step 4: G1 컴파일**

```bash
hexa build scripts/<name>.hexa
```
종료코드 0 확인. 실패 시 → 즉시 멈춤·보고.

- [ ] **Step 5: G2 실행**

원본과 동일 인자로 포팅본 실행. 종료코드 0, 런타임 에러 0 확인.

- [ ] **Step 6: G3 값 동치**

```bash
diff <(bash scripts/<name>.sh <args>) <(hexa run scripts/<name>.hexa <args>)
```
ε=1e-9 (shell은 보통 텍스트 출력이라 직접 diff). 차이 있으면 → 즉시 멈춤·보고.

- [ ] **Step 7: NEXUS-6 스캔 (after)**

digest 저장. before/after 비교.

- [ ] **Step 8: 포팅본 추가 커밋**

```bash
git add scripts/<name>.hexa
git commit -m "port(L1): scripts/<name>.sh → .hexa (G1~G3 통과)"
```

- [ ] **Step 9: G4 원본 삭제 + 별도 커밋**

```bash
git rm scripts/<name>.sh
git commit -m "remove(L1): scripts/<name>.sh (포팅 완료, .hexa로 대체)"
```

- [ ] **Step 10: 로그 append**

`~/Dev/nexus/shared/porting_log.jsonl`에 레코드 추가 (스펙 §6 형식).

### Task L1.3: L1 나머지 일괄 처리

- [ ] **Step 1: 위상정렬 순서대로 자동 루프**

각 파일에 대해 Task L1.2의 Step 1~10을 동일하게 반복. **검증(G1~G3) 없는 자동 통과 절대 금지.** 게이트 실패 시 즉시 멈춤·보고.

- [ ] **Step 2: 계층 완료 보고 + 다음 계층 승인 (계층 종료 게이트)**

처리 파일 수, 총 커밋 수, 로그 위치 출력. 사용자 승인 후 L2 진행.

---

## L2: tools/*/src/*.rs 계산기 포팅

### Task L2.1: 인벤토리 + 위상정렬

**Files:**
- Read: `tools/*/src/*.rs` (단, `tools/nexus/` 제외)
- Create: `~/Dev/nexus/shared/porting_inventory_l2.json`

- [ ] **Step 1: 파일 목록 수집 (`tools/nexus/` 제외)**

`tools/` 하위 `*/src/*.rs` 중 `tools/nexus/`를 제외한 모든 파일.

- [ ] **Step 2: crate 단위 의존성 분석**

각 crate의 `Cargo.toml` 읽고 내부 의존(다른 calc crate 참조 여부) 파악. 대부분 std-only 독립일 것.

- [ ] **Step 3: 도메인별 ε 결정**

물리/핵융합/광학 → `1e-12`, 일반 → `1e-9`. 인벤토리에 파일별 메타로 기록.

- [ ] **Step 4: 위상정렬 + 인벤토리 작성 + 사용자 승인**

L1.1과 동일 형식. 승인 후 진행.

### Task L2.2: 첫 파일 — Rust 수치 계산 패턴 수립

가장 단순한 std-only 계산기 1개(예: `gut-calc-rust/src/main.rs` 류)를 골라 변환.

- [ ] **Step 1~10: L1.2와 동일한 D 게이트 사이클**

차이점:
- G1: `hexa build`
- G2 실행 인자: 원본 Rust 바이너리 빌드 후 동일 인자 비교
- G3: 부동소수 비교 스크립트 사용 (단순 diff 부족), ε는 도메인별

- [ ] **Step 11: 부동소수 비교 헬퍼 .hexa 1개 작성**

이후 모든 L2/L3 파일 G3에 재사용. 작성 위치: `tools/_porting/float_diff.hexa`. 입력 두 stdout, ε 인자.

### Task L2.3: L2 나머지 일괄 처리

- [ ] **Step 1: 위상정렬 순서대로 자동 루프**

L1.3과 동일.

- [ ] **Step 2: 계층 완료 보고 + L3 진행 승인**

---

## L3: techniques/*.py 포팅

### Task L3.1: 인벤토리 + 의존성 분석 + numpy/torch 사용 식별

**Files:**
- Read: `techniques/*.py`
- Create: `~/Dev/nexus/shared/porting_inventory_l3.json`

- [ ] **Step 1: 파일 목록 + import 분석**

각 .py의 `import` 문 추출. numpy/torch/scipy/matplotlib 사용 여부 표시.

- [ ] **Step 2: 위상정렬**

서로 import 관계 분석. 리프 우선.

- [ ] **Step 3: 위험 R-1 점검 — HEXA ML 스택 가용성 확인**

HEXA 표준 라이브러리에 numpy 상응 모듈 존재 여부 확인. 없으면 → **L3 진입 전에 사용자에게 보고**, 결정 대기 (스킵 / 래퍼 작성 / 범위 재정의).

- [ ] **Step 4: 인벤토리 작성 + 사용자 승인**

### Task L3.2: 첫 파일 — Python 알고리즘 패턴 수립

순수 Python(numpy 미사용) 리프부터. 예: `techniques/phi6simple.py`가 순수 산술이라면 우선.

- [ ] **Step 1~10: D 게이트 사이클** (L1.2와 동일 구조)

차이점:
- ε: ML 확률값은 `1e-6`, 수치 계산은 `1e-9`
- G3: stdout이 dict/json이면 키별 ε 비교, 그 외 부동소수 라인 단위 비교

### Task L3.3: L3 나머지 일괄 처리

- [ ] **Step 1: numpy 사용 파일 진입 시점**

처음 numpy 의존 파일에 도달하면 멈춤·보고. 사용자가 진행 방식 결정 (HEXA numpy 상응 / 외부 호출 래퍼 / 보류).

- [ ] **Step 2: 자동 루프 + 계층 완료 보고 + L4 승인**

---

## L4: experiments/*.py 포팅

### Task L4.1: 인벤토리 + matplotlib 처리 방침

**Files:**
- Read: `experiments/*.py`
- Create: `~/Dev/nexus/shared/porting_inventory_l4.json`

- [ ] **Step 1: 파일 목록 + L3 의존 매핑**

각 experiment가 어떤 technique 모듈에 의존하는지 그래프화. L3가 미완료면 해당 experiment 보류.

- [ ] **Step 2: 위험 R-2 점검 — matplotlib 출력 비교**

PNG 출력이 G3 비교 대상이면: (a) PNG 바이트 동일성 (시드 고정 필요) / (b) 수치 데이터만 비교, 시각 출력은 면제. 사용자 결정.

- [ ] **Step 3: 인벤토리 작성 + 사용자 승인**

### Task L4.2: L4 일괄 처리

- [ ] **Step 1~N: 자동 루프**

L1~L3과 동일 D 게이트. matplotlib 처리는 R-2 결정에 따름.

- [ ] **Step 2: 계층 완료 보고**

---

## Phase 종료: 잔여 검증

### Task Z.1: 리포 잔여 0 확인

- [ ] **Step 1: 잔여 파일 스캔**

```
Glob: **/*.py, **/*.rs, **/*.sh
```
결과에서 `tools/nexus/**`, L0 제외 리스트, `target/`, `.git/` 제외 후 잔여가 0인지 확인.

- [ ] **Step 2: 잔여 있으면 보고**

각 잔여 파일에 대해 사유(스킵·미완료·예외) 표시. 사용자 결정 대기.

### Task Z.2: 회귀 sanity 체크

- [ ] **Step 1: 각 계층에서 무작위 N개 .hexa 재실행**

원본은 git history에서 임시 복원하여 다시 G3 비교. 회귀 없음 확인.

- [ ] **Step 2: NEXUS-6 전체 스캔**

`nexus scan n6-architecture --full` 실행. 이상 없음 확인.

### Task Z.3: 최종 보고

- [ ] **Step 1: 계층별 통계, 총 커밋 수, 로그 경로, 잔여 파일, 미해결 위험 정리해 사용자 출력**

---

## 자기검토 체크리스트

- 스펙 §1~§10 모든 항목이 태스크에 매핑되는가? ✅ (1→0/L1~4, 2→0.1/L1~4 인벤토리, 3→사이클 구조, 4→D 게이트 스텝, 5→계층 사이클, 6→로그 스텝, 7→"즉시 멈춤·보고" 명시, 8→R-1/R-2/R-3/R-4 각 태스크에 점검 스텝, 9→산출물 스텝, 10→Z 단계)
- placeholder 없음 (TBD/TODO/"적절히" 없음)
- 타입/명령 일관성: `hexa build`, `nexus scan`, `porting_log.jsonl` 경로 모두 동일
- 위험 R-1(numpy)·R-2(matplotlib)는 진입 전 명시적 게이트로 처리
- 모든 게이트 실패 시 처리 = "즉시 멈춤·보고" 일관

