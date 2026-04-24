---
id: roadmap-v3-migration-from-v2
date: 2026-04-24
roadmap_task: HONEST-PX-3 (v2 → v3 승계 매핑)
grade: [10] migration note
parent_design: theory/roadmap-v2/millennium-v3-design-2026-04-15.md
scope: what-is-carried-forward vs what-is-new
---

# v2 → v3 Migration Note

> 본 문서는 v2.3 에서 v3 로 **승계되는 것** 과 **신규 생성되는 것** 을 명시합니다.
> v3 는 v2 를 **삭제하지 않고** 위에 쌓는 구조 (additive) 입니다.

---

## 1. 승계 (CARRIED FORWARD as-is)

### 1.1 Phase 구조 (변경 없음)

v2.3 의 아래 phase 파일은 `theory/roadmap-v2/` 에 그대로 보존되며 v3 에서 **참조만** 됩니다.

| v2.3 phase | 파일 위치 | v3 상태 |
|-----------|----------|--------|
| P0 axis-final | `roadmap-v2/axis-final.md` | carried (read-only) |
| P1 foundation | `roadmap-v2/phase-01-foundation-Y-axes.md` 외 | carried |
| P2 BT-541 | `roadmap-v2/phase-02-Y1-bt541-riemann.md` 외 | carried |
| P3 BT-542 | `roadmap-v2/phase-03-Y4-bt542-pnp.md` 외 | carried |
| P4 BT-543/544 | `roadmap-v2/phase-04-Y5Y6-bt543-bt544.md` 외 | carried |
| P5 BT-545/546 | `roadmap-v2/phase-05-Y7Y8-bt545-bt546.md` 외 | carried |
| P6 BT-547 | `roadmap-v2/phase-06-bt547-poincare-retrospect.md` | carried |
| P7 cross-BT | `roadmap-v2/phase-07-cross-bt-transfer-protocol.md` | carried |
| P8 meta-audit | `roadmap-v2/phase-08-meta-audit-philosophy.md` | carried |
| P9 external | `roadmap-v2/phase-09-external-history-publication.md` | carried |
| P10 measurement | `roadmap-v2/phase-10-measurement-strategy-tools.md` | carried |
| PΩ closure | `roadmap-v2/phase-omega-Y9-closure-v3-design.md` | carried |
| PX extension | `roadmap-v2/phase-PX-*.md` (4 files) | carried |

### 1.2 JSON 산출물 (변경 없음)

- `shared/roadmaps/millennium.json` — v2.3 구조 유지, `schema_version` 만 `"3.0"` 승격 예정 (Phase-00 gate 통과 후)
- 155 tasks / 143 done / 5 partial / 7 deferred / saturation_index 1.0 / R14 CLEAN — **보존**

### 1.3 Atlas entries (변경 없음, 누적)

설계 §0.2 의 loop1~7 18 entries 는 이미 atlas 에 등록. v3 는 **추가만** 하며 삭제/수정 없음.

### 1.4 도구 (변경 없음, 확장만)

- `scripts/empirical/*.py` (5 files) — carried
- `scripts/monotone/*.py` (2 files) — carried, M5 에서 v2 확장

### 1.5 Breakthrough 문서 (변경 없음)

설계 §7 의 6 breakthrough .md 파일 — carried (M1 preprint 후보 pool).

---

## 2. Deferred → v3 task 매핑 (MANDATORY §5.1 증빙)

v2.3 deferred 7 항목 → v3 18 tasks 배치 현황:

| v2.3 deferred | v3 배치 | phase | 근거 |
|--------------|---------|-------|------|
| Sage/Pari 도구 정밀화 | **E1, E2, E3** | phase-11 | 설계 §2 E1~E3 |
| Cremona 3M 확장 | **E4, E5** | phase-11 | 설계 §2 E4~E5 |
| arXiv 정기 서베이 | **E6, E7** | phase-11 | 설계 §2 E6~E7 |
| Moonshine L5 (BT-18) | **T2** | phase-12 | 설계 §3 T2 |
| Lean4 형식화 | **M3** | phase-13 | 설계 §4 M3 |
| preprint 투고 | **M1** | phase-13 | 설계 §4 M1 |
| 수학자 peer review | **M2, M4** | phase-13 | 설계 §4 M2, M4 |

**매핑 결과**: 7/7 deferred 가 v3 track 에 배치됨. §5.1 의 "≥ 3 배치" 요건 **초과 달성**.

---

## 3. 신규 (NEW in v3)

### 3.1 Phase (3개)

- `phase-11-E-empirical.md` — E track 7 tasks (설계 §2)
- `phase-12-T-theoretical.md` — T track 6 tasks (설계 §3)
- `phase-13-M-meta.md` — M track 5 tasks (설계 §4)

**현재 상태**: PENDING (본 스캐폴드에서는 미생성, 사용자 authorize 후 착수)

### 3.2 Schema 변경

- `millennium.json.schema_version`: `"2.3"` → `"3.0"` (phase-00 gate 통과 시점)
- 신규 phase 필드 `v3_tracks: {E, T, M}` 추가 예정
- 기존 155 tasks 구조는 **불변**, v3 tasks 18 개가 추가되어 총 173 tasks 예정

### 3.3 신규 디렉토리

- `theory/roadmap-v3/` — 본 디렉토리 (스캐폴드 생성됨)

### 3.4 신규 도구 (예정)

- `scripts/monotone/ouroboros_detector_v2.py` (M5, namespace-aware)
- `.github/workflows/arxiv-quarterly.yml` (E6)
- `scripts/empirical/sage_wrapper.py` (E1)

---

## 4. 삭제/폐기 (DELETED)

**없음**. v3 는 additive migration 입니다. v2.3 산출물 중 삭제되는 항목은 0 건.

---

## 5. 호환성 경계

- v2 를 읽는 외부 도구는 v3 출범 후에도 `roadmap-v2/` 경로로 계속 작동합니다.
- `millennium.json` 은 단일 파일 유지 (분기 없음). schema_version 필드로만 v2/v3 구분.
- atlas entries 는 append-only, v2 entries 의 id 재사용 금지.

---

## 6. 관련 파일

- 설계: `../roadmap-v2/millennium-v3-design-2026-04-15.md`
- 게이트: `./phase-00-bootstrap.md`
- 상위 index: `./README.md`
- JSON: `../../shared/roadmaps/millennium.json` (cwd 기준: `shared/roadmaps/millennium.json`)

---

*작성: 2026-04-24 (v3 scaffold bootstrap)*
