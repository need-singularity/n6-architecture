# phase6_b4_translate.py — RETIRED

Status: RETIRED (one-shot translation helper, output committed)
Retired: 2026-04-29
Per: raw 9 hexa-only mandate + raw 102 ADD-new path + raw 168 minimum-viable

## Original purpose

Phase 6 Batch B4 translation helper for `domains/space` YELLOW band.
Author-reviewed exact-string Korean -> English substitutions across 8 files
(aerospace, aerospace-transport, astronomy, hexa-cosmic, hexa-starship,
observational-astronomy, space-engineering, space-systems).

## Why retired

- One-shot run; no CI / .own / .github / Makefile / hexa caller references it
- Output already committed: `61541460 docs(translate): phase-6-4 space aerospace + aerospace-transport (2 files)`
- Idempotent str.replace on Korean phrases that no longer exist in the
  current English translations (re-run is a no-op)
- Hexa port would require constants-table emulation with no flow value

## Recovery (if ever needed)

```
git show 61541460:scripts/phase6_b4_translate.py > scripts/phase6_b4_translate.py
```

The PAIRS / SHARED / PER_FILE substitution tables are deterministic and
re-runnable from that commit-pinned snapshot.

## Cross-reference

- Output commit: `61541460`
- Sibling helpers (also retired): `phase6_b7_translate.retired.md`,
  `phase6_b7_repair.retired.md`
