# n6-architecture 주간 감사 리포트 — 2026-04-11

> **감사 범위**: 2026-04-04 ~ 2026-04-11 (지난 7일)
> **생성 시각**: 2026-04-11 20:55:15
> **생성기**: `nexus/scripts/weekly_audit.hexa` (자동)

---

## 요약 스냅샷

```
  ┌────────────────────────────────────────────────────────────────────┐
  │                n6-architecture 주간 감사 요약                       │
  ├──────────────┬──────────────┬──────────────┬──────────────────────┤
  │  커밋         │  성장버스     │  발견그래프   │  atlas 승격          │
  ├──────────────┼──────────────┼──────────────┼──────────────────────┤
  │ 493 건      │ 19317 신규 │ +2132 노드 │ +4647 [10*]          │
  │ 56557 파일 │ 19322 누적 │ +1702 엣지 │ +967 [7]             │
  └──────────────┴──────────────┴──────────────┴──────────────────────┘
```

## 1. Git 활동 (지난 7일)

| 지표 | 값 |
|------|-----|
| 커밋 수 | **493** |
| 저자 수 | **1** |
| 변경 파일 | **56557** |
| 삽입 라인 | 1756223 |
| 삭제 라인 | 1558755 |

**저자**: dancinlife

### 1.1 9축 파일 변경 분포

| 축 | 파일 수 |
|----|---------|
| `theory/` | 58 |
| `domains/` | 4146 |
| `nexus/` | 1777 |
| `techniques/` | 189 |
| `experiments/` | 299 |
| `engine/` | 19 |
| `papers/` | 2 |
| `reports/` | 117 |
| `shared/` | 2954 |

### 1.2 최근 커밋 (최대 12건)

| SHA | 저자 | 제목 |
|-----|------|------|
| `0c23ad27` | dancinlife | refactor(telescope): 56개 렌즈 Rust→HEXA 전환 완료 — mod.rs 등록 해제 |
| `b1ceb88f` | dancinlife | feat(telescope): 프론티어 렌즈 확장 + discovery_graph v10 + experiments 결과 갱신 |
| `cc0c683d` | dancinlife | L0-수정: CODEOWNERS — lockdown.json L0 전체 동기화 (자동 생성) |
| `dca8f87f` | dancinlife | feat: GO 세션 — RTL .hexa 전환, 칩 DSE 5종, discovery_graph v9, chaos_lens 확장 |
| `2a869e64` | dancinlife | feat(L0): CODEOWNERS 등록 |
| `b278def1` | dancinlife | feat(L0): l0-block.hexa PreToolUse 훅 등록 |
| `c6b02dd9` | dancinlife | sync: CLAUDE.md from nexus/shared/project-claude/n6-architecture.md |
| `ba74747e` | dancinlife | sync: CLAUDE.md from nexus/shared/project-claude/n6-architecture.md |
| `39ec3bd4` | dancinlife | sync: CLAUDE.md from nexus/shared/project-claude/n6-architecture.md |
| `a4d26494` | dancinlife | chore(claude): allow Edit/Write on settings.local.json |
| `fd0ea93c` | dancinlife | chore(hooks): hexa 바이너리 경로 target/release/hexa → hexa 통일 |
| `03978cc9` | dancinlife | chore(hooks): project-aware-improver + build-fail-diag 연동 |

## 2. growth_bus.jsonl 신규 엔트리

| 지표 | 값 |
|------|-----|
| 누적 라인 | **19322** |
| 지난 7일 신규 | **19317** |

### 2.1 이벤트 타입 TOP

| 타입 | 건수 |
|------|------|
| `anomaly_observed` | 11838 |
| `gpu_idle_forge` | 193 |
| `offload` | 3202 |
| `tick` | 4081 |

### 2.2 도메인 TOP

| 도메인 | 건수 |
|--------|------|
| `airgenome` | 15563 |
| `anima` | 357 |
| `hexa-lang` | 1401 |
| `n6-architecture` | 531 |
| `nexus` | 1363 |
| `void` | 102 |

## 3. discovery_graph.json 변화

| 지표 | 이전 스냅샷 | 현재 | 델타 |
|------|-------------|------|------|
| 노드 | 0 | **2132** | +2132 |
| 엣지 | 0 | **1702** | +1702 |

## 4. atlas.n6 등급 집계 ([7] → [10*] 승격 추적)

```
  파일: $NEXUS/shared/n6/atlas.n6

  [10*] EXACT 검증완료    │   4647 │ +4647
  [10]/[9] NEAR           │     83 │
  [7]  EMPIRICAL 승격대상 │    967 │ +967
  [N?] CONJECTURE         │    973 │
  [N!] BREAKTHROUGH       │     19 │
```

✅ 이번 주 `[7] → [10*]` 승격 **4647 건** 추정

## 5. cargo test 결과

| 상태 | 값 |
|------|-----|
| 요약 | 생략 (--no-cargo) |
| PASS | 0 |
| FAIL | 0 |
| IGNORED | 0 |

## 6. convergence 상태 (`n6shared/convergence/n6-architecture.json`)

| 구분 | 항목 수 |
|------|---------|
| 🔒 ossified (골화 완료) | **18** |
| 🟡 stable (안정, 골화 대기) | 3 |
| ❌ failed (실패) | 0 |

## 7. 권장 후속 액션

- ✅ 이번 주 특별 이슈 없음 — 정상 성장 중

---

*본 리포트는 `nexus/scripts/weekly_audit.hexa` 로 자동 생성되었습니다.*
*규칙: R1 HEXA-FIRST / R14 shared SSOT / 한글 필수*
