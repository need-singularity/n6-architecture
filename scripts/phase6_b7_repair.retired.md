# phase6_b7_repair.py — RETIRED

Status: RETIRED (one-shot repair helper, output committed)
Retired: 2026-04-29
Per: raw 9 hexa-only mandate + raw 102 ADD-new path + raw 168 minimum-viable

## Original purpose

Repair pass for Phase 6 Batch B7 translation artifacts. Cleans up known
overreach patterns produced by `phase6_b7_translate.py` (e.g.
"everyunified" -> "everyday", "min기" -> "minimum term", stray "통day",
"시뮬레이션" -> "simulation", etc.).

## Why retired

- One-shot run; no CI / .own / .github / Makefile / hexa caller references it
- Output already committed: `7c566cb5 docs(translate): phase-6-7 physics Millennium + pure-math (18 files, 7 final + 11 partial)`
- Idempotent str.replace on broken intermediate phrases that no longer
  exist in the post-repair English text (re-run is a no-op)
- Hexa port would require constants-table emulation with no flow value

## Recovery (if ever needed)

```
git show 7c566cb5:scripts/phase6_b7_repair.py > scripts/phase6_b7_repair.py
```

The REPAIRS substitution table is deterministic and re-runnable from
that commit-pinned snapshot.

## Cross-reference

- Output commit: `7c566cb5`
- Companion (also retired): `phase6_b7_translate.retired.md`
- Sibling (also retired): `phase6_b4_translate.retired.md`
