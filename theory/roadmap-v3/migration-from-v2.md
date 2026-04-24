---
id: roadmap-v3-migration-from-v2
date: 2026-04-24
roadmap_task: HONEST-PX-3 (v2 -> v3 inheritance mapping)
grade: [10] migration note
parent_design: theory/roadmap-v2/millennium-v3-design-2026-04-15.md
scope: what-is-carried-forward vs what-is-new
---

# v2 -> v3 Migration Note

> This document specifies what is **carried forward** from v2.3 into v3 versus what is **newly created**.
> v3 **does not delete** v2 — it is an additive layer on top of it.

---

## 1. Carried forward (as-is)

### 1.1 Phase structure (no changes)

The v2.3 phase files below remain in `theory/roadmap-v2/` and are **only referenced** from v3.

| v2.3 phase | file location | v3 status |
|-----------|----------|--------|
| P0 axis-final | `roadmap-v2/axis-final.md` | carried (read-only) |
| P1 foundation | `roadmap-v2/phase-01-foundation-Y-axes.md` et al. | carried |
| P2 BT-541 | `roadmap-v2/phase-02-Y1-bt541-riemann.md` et al. | carried |
| P3 BT-542 | `roadmap-v2/phase-03-Y4-bt542-pnp.md` et al. | carried |
| P4 BT-543/544 | `roadmap-v2/phase-04-Y5Y6-bt543-bt544.md` et al. | carried |
| P5 BT-545/546 | `roadmap-v2/phase-05-Y7Y8-bt545-bt546.md` et al. | carried |
| P6 BT-547 | `roadmap-v2/phase-06-bt547-poincare-retrospect.md` | carried |
| P7 cross-BT | `roadmap-v2/phase-07-cross-bt-transfer-protocol.md` | carried |
| P8 meta-audit | `roadmap-v2/phase-08-meta-audit-philosophy.md` | carried |
| P9 external | `roadmap-v2/phase-09-external-history-publication.md` | carried |
| P10 measurement | `roadmap-v2/phase-10-measurement-strategy-tools.md` | carried |
| PΩ closure | `roadmap-v2/phase-omega-Y9-closure-v3-design.md` | carried |
| PX extension | `roadmap-v2/phase-PX-*.md` (4 files) | carried |

### 1.2 JSON artifacts (no changes)

- `shared/roadmaps/millennium.json` — v2.3 structure preserved; only `schema_version` to be promoted to `"3.0"` (after Phase-00 gate passes)
- 155 tasks / 143 done / 5 partial / 7 deferred / saturation_index 1.0 / R14 CLEAN — **preserved**

### 1.3 Atlas entries (no changes, cumulative)

The design §0.2 loop1~7 18 entries are already registered in atlas. v3 **only appends** and never deletes or modifies.

### 1.4 Tooling (no changes, extension only)

- `scripts/empirical/*.py` (5 files) — carried
- `scripts/monotone/*.py` (2 files) — carried, extended by v2 in M5

### 1.5 Breakthrough documents (no changes)

The 6 breakthrough .md files from design §7 — carried (M1 preprint candidate pool).

---

## 2. Deferred -> v3 task mapping (MANDATORY §5.1 evidence)

v2.3 deferred 7 items -> v3 18 tasks placement status:

| v2.3 deferred | v3 placement | phase | basis |
|--------------|---------|-------|------|
| Sage/Pari tooling refinement | **E1, E2, E3** | phase-11 | design §2 E1~E3 |
| Cremona 3M extension | **E4, E5** | phase-11 | design §2 E4~E5 |
| arXiv regular survey | **E6, E7** | phase-11 | design §2 E6~E7 |
| Moonshine L5 (BT-18) | **T2** | phase-12 | design §3 T2 |
| Lean4 formalization | **M3** | phase-13 | design §4 M3 |
| preprint submission | **M1** | phase-13 | design §4 M1 |
| mathematician peer review | **M2, M4** | phase-13 | design §4 M2, M4 |

**Mapping result**: 7/7 deferred items are placed into v3 tracks. §5.1's ">= 3 placements" requirement is **exceeded**.

---

## 3. New (NEW in v3)

### 3.1 Phases (3)

- `phase-11-E-empirical.md` — E track 7 tasks (design §2)
- `phase-12-T-theoretical.md` — T track 6 tasks (design §3)
- `phase-13-M-meta.md` — M track 5 tasks (design §4)

**Current status**: PENDING (not generated in this scaffold; work begins after user authorize)

### 3.2 Schema changes

- `millennium.json.schema_version`: `"2.3"` -> `"3.0"` (at phase-00 gate pass)
- New phase field `v3_tracks: {E, T, M}` planned
- The existing 155-tasks structure is **unchanged**; 18 v3 tasks are appended for a planned total of 173 tasks

### 3.3 New directory

- `theory/roadmap-v3/` — this directory (scaffold created)

### 3.4 New tooling (planned)

- `scripts/monotone/ouroboros_detector_v2.py` (M5, namespace-aware)
- `.github/workflows/arxiv-quarterly.yml` (E6)
- `scripts/empirical/sage_wrapper.py` (E1)

---

## 4. Deleted / retired

**None**. v3 is an additive migration. Zero v2.3 artifacts are removed.

---

## 5. Compatibility boundary

- External tools that read v2 continue to work via the `roadmap-v2/` path even after v3 launch.
- `millennium.json` remains a single file (no fork). v2/v3 are distinguished only by the schema_version field.
- atlas entries are append-only; v2 entry id reuse is forbidden.

---

## 6. Related files

- Design: `../roadmap-v2/millennium-v3-design-2026-04-15.md`
- Gate: `./phase-00-bootstrap.md`
- Upstream index: `./README.md`
- JSON: `../../shared/roadmaps/millennium.json` (cwd-relative: `shared/roadmaps/millennium.json`)

---

*Written: 2026-04-24 (v3 scaffold bootstrap)*
