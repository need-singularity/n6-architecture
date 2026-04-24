---
id: roadmap-v3-index
date: 2026-04-24
roadmap_task: HONEST-PX-3 (v3 execution scaffold)
grade: [10] skeleton index
parent_design: theory/roadmap-v2/millennium-v3-design-2026-04-15.md
parent_json: shared/roadmaps/millennium.json (v2.3 -> v3.0 pending)
license: CC-BY-SA-4.0
status: SCAFFOLD (phase-00 bootstrap only; phase-01+ require authorization)
---

# Millennium 7 Problems Roadmap v3 — Execution Scaffold

> **This directory is an execution skeleton.** The v3 design is fixed in
> `theory/roadmap-v2/millennium-v3-design-2026-04-15.md`, and this README is
> the entry point that implements that design as a directory structure.
>
> **Phase contents require research, and are not generated at this scaffold stage.**
> Each phase file is only populated after the user's explicit "authorize phase-NN" instruction.

---

## 0. Scaffold status (as of 2026-04-24)

| Item | Status |
|------|------|
| Design document | DONE (`roadmap-v2/millennium-v3-design-2026-04-15.md`) |
| v3 directory created | DONE (this directory) |
| `phase-00-bootstrap.md` | DONE (this scaffold commit) |
| `migration-from-v2.md` | DONE (this scaffold commit) |
| `phase-01-E-empirical.md` ~ `phase-13-closure.md` | **PENDING (research required)** |
| `millennium.json` schema_version promotion (2.3 -> 3.0) | PENDING (after the 4 MANDATORY items in §5.3) |

---

## 1. v3 structure (plan)

Per design doc §1.3, v2.3's P0~P10/PΩ/PX are **carried forward as-is**, and only the three new phases are
managed in this directory.

```
theory/roadmap-v3/
├── README.md                       # this file (index)
├── phase-00-bootstrap.md           # v2.3 -> v3 transition gate (MANDATORY §5.1)
├── migration-from-v2.md            # v2 -> v3 inheritance mapping
├── phase-11-E-empirical.md         # [PENDING] E track 7 tasks (E1~E7)
├── phase-12-T-theoretical.md       # [PENDING] T track 6 tasks (T1~T6)
├── phase-13-M-meta.md              # [PENDING] M track 5 tasks (M1~M5)
└── phase-omega-v3-closure.md       # [PENDING] v3 closure meta
```

> Note: phase-11~13 numbering continues the sequence from v2.3's PX.
> Only v3 new tracks are created in this directory; P0~P10/PΩ/PX remain in `roadmap-v2/`.

---

## 2. 3-track summary (quoted from design §1.2, for execution)

| track | phase file | tasks | cost sum | BT linkage |
|-------|-----------|-------|---------|---------|
| **E** Empirical | phase-11-E-empirical.md | E1~E7 | L+L+M | BT-541, 546 |
| **T** Theoretical | phase-12-T-theoretical.md | T1~T6 | L+L+L | BT-542, 545 |
| **M** Meta | phase-13-M-meta.md | M1~M5 | M+L+M | all BTs |

---

## 3. v3 entry conditions (MANDATORY, quoted from design §5.1)

- [ ] >= 3 of the v2.3 deferred 7 items "placed into v3 Phase" (placements finalized in design §2-4)
- [ ] v2.3 loop1-7 atlas 18 entries reconfirmed (R14 CLEAN 2026-04-15 confirmation required)
- [ ] Monotone drift check baseline registered (MONOTONE-PX-1 CLI)
- [ ] BT resolved count 0/6 reconfirmed

When all 4 items are satisfied and the user issues "go v3", `shared/roadmaps/millennium.json.schema_version` is
promoted to `"3.0"`.

---

## 4. Honesty charter (quoted from design §6.1, applied across this directory)

1. **No BT-resolution claims** — at no point during v3 are statements such as "we demonstrated RH" permitted
2. **Name external dependencies** — explicit on Sage / Lean4 / arXiv API / external-mathematician reliance
3. **State MISS criteria in advance** — each task pre-specifies its MISS threshold
4. **OUROBOROS periodic audit** — confirm R14 CLEAN at the end of every phase

---

## 5. Related files

- Design: `theory/roadmap-v2/millennium-v3-design-2026-04-15.md`
- Inheritance mapping: `./migration-from-v2.md`
- Transition gate: `./phase-00-bootstrap.md`
- Upstream JSON: `shared/roadmaps/millennium.json` (v2.3)

---

*Written: 2026-04-24 (v3 scaffold bootstrap)*
*Next step: on user `authorize phase-01`, begin generating contents of `phase-11-E-empirical.md`*
