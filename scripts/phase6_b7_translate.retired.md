# phase6_b7_translate.py — RETIRED

Status: RETIRED (one-shot translation helper, output committed)
Retired: 2026-04-29
Per: raw 9 hexa-only mandate + raw 102 ADD-new path + raw 168 minimum-viable

## Original purpose

Phase 6 Batch B7 translation helper. Author-reviewed exact-string
Korean -> English substitutions across 18 physics domain files
(Millennium problems + pure-math + cosmology + holography etc.).

Companion to `phase6_b7_repair.py` (follow-up dict pass).

## Why retired

- One-shot run; no CI / .own / .github / Makefile / hexa caller references it
- Output already committed: `7c566cb5 docs(translate): phase-6-7 physics Millennium + pure-math (18 files, 7 final + 11 partial)`
- Idempotent str.replace on Korean phrases that no longer exist in the
  current English translations (re-run is a no-op for the 7 FINAL files;
  11 PARTIAL files have remaining CJK only inside per-file BLOWUP blocks
  not handled by this helper)
- Hexa port would require constants-table emulation with no flow value

## Recovery (if ever needed)

```
git show 7c566cb5:scripts/phase6_b7_translate.py > scripts/phase6_b7_translate.py
```

The PAIRS substitution table is deterministic and re-runnable from that
commit-pinned snapshot.

## Cross-reference

- Output commit: `7c566cb5`
- Companion (also retired): `phase6_b7_repair.retired.md`
- Sibling (also retired): `phase6_b4_translate.retired.md`
