# HEXA-WEAVE Cycle 18 — sync-readme.hexa pipeline bug diagnosis + fix

**Date**: 2026-04-28
**Cycle**: 18
**Channel**: hexa-runtime-pipeline-fix
**Predecessor witnesses**:
- design/kick/2026-04-28_readme-curation-cycle16_omega_cycle.json
- design/kick/2026-04-28_stash-pop-loss-recovery-cycle17_omega_cycle.json

## 1. Background

Since the cycle-11 README curation episode (Korean directive `README 에 노출`),
the hexa-runtime sync pipeline `n6shared/sync-readme.hexa` has been emitting
the false `[OK] README.md is up to date with readme-data.json (64 markers)`
status while only 0–2 of 64 markers were actually being processed. The
cycle-11 workaround restored `scripts/sync-readme.py` as a `/tmp` fallback;
cycle-16 narrowed automatic sync to `AUTO:BADGE` only and continued
hand-curating `AUTO:STATS / AUTO:SUMMARY_biology / AUTO:FOOTER_biology`
manually. Cycle-16 also honestly disclosed the per-domain marker bug as a
residual hexa-runtime issue.

The cycle-11 hypothesis (`exec("printenv HOME") empty` + `json_parse
multibyte+paren failure at readme-data.json L664`) turned out to be wrong.
Both subsystems are healthy in the current runtime. The actual bug is in
`scan_markers()` — a prefix-confusion defect causing marker-name pollution
on every iteration after the first.

## 2. Root-cause diagnosis (cycle-18)

Trace: re-implemented `scan_markers()` literally and dumped `out` against
the live README:

```
scan_markers returned 64 entries
  out[0]  = "BADGE"            ← clean
  out[1]  = "BADGE:END -->\n[![Discord](https://...\n...COMMON_LINKS"  ← polluted
  out[2]  = "COMMON_LINKS"     ← clean (next happened to be SHARED:PROJECTS)
  out[3]  = "COMMON_LINKS:END -->\n...AUTO:STATS"                       ← polluted
  ...
clean markers: 2 / 64
```

Mechanism:
1. `prefix = "<!-- AUTO:"` matches `<!-- AUTO:BADGE:END -->` just as well as
   `<!-- AUTO:BADGE:START -->`.
2. After matching `END` tag, `after_prefix` starts with `BADGE:END -->\n...`
3. `index_of(":START -->")` then finds the START tag of a *different*
   downstream marker (e.g. the next domain's COMMON_LINKS:START), which can
   be hundreds of bytes away.
4. The intervening text (including the spurious `BADGE:END -->` prefix and
   any markdown body in between) becomes the captured `name`.
5. `resolve_generator(spurious_name, data)` returns nil → marker preserved
   unchanged → `replace_marker` not called → `text != before` is false →
   `changed.push` skipped → final `new_text == original` is true → false
   "up to date" report.

Net effect: 64 markers scanned, only 2 (the first one in each section
before any END tag is seen) reach `resolve_generator` cleanly. Every
SUMMARY_*/FOOTER_*/STATS marker was effectively skipped by the pipeline,
explaining the cycle-11/16 manual fallback regime.

A second, independent hexa-runtime quirk was discovered during fix
verification: JSON `null` values produce `type_of(v) == "void"` (not
`"null"` or `"nil"`), so the existing `get_or` / `has_field` guards let
raw void slip through. `to_string(void) == "void"` then injected literal
`void` text into rendered markers (`DSE void`, `산업void%`, `실험void%`,
`물리한계void`).

## 3. Cycle-18 fix (option A — n6shared/sync-readme.hexa upstream)

Two atomic changes in `n6shared/sync-readme.hexa`:

### 3.1. `scan_markers` NAME validation
Added `is_valid_marker_name(name)` checking `name` is non-empty (≤ 64 chars)
and contains only `[A-Za-z0-9_-]`. The hyphen allowance preserves
domain-id convention (life-culture / tech-industry / etc.).
Invalid candidates trigger advance-by-prefix-length and continue, so the
real `:START -->` further along is found on the next iteration.

### 3.2. Void-type guard in `get_or` / `has_field`
Added `if t == "void" { return default_val }` (and `return false` for
`has_field`) in both helpers. Defensive triple-check `null` / `nil` /
`void` keeps forward-compat with future hexa-runtime classification.

Other options considered:
- **Option B** — keep manual `scripts/sync-readme.py` permanent: rejected
  because raw 9 hexa-only mandates HEXA-FIRST runtime; falling back
  permanently entrenches the regression and blocks future contributions.
- **Option C** — upstream hexa-runtime patch (string indexing, type_of
  classification): deferred. Two .hexa-level guards are sufficient for
  the immediate sync regression; runtime fix is a separate cycle.

Selected: **Option A**.

## 4. Verification

Pre-fix dry-run:
```
[OK] README.md is up to date with readme-data.json (64 markers)   ← false
```

Post-fix dry-run (after both 3.1 and 3.2):
```
[DRY-RUN] 36 marker(s) would change:
  -> AUTO:STATS
  -> AUTO:SUMMARY_biology
  -> AUTO:FOOTER_biology
  -> AUTO:SUMMARY_play
  -> AUTO:FOOTER_play
  -> AUTO:SUMMARY_aerospace
  -> AUTO:FOOTER_aerospace
  -> AUTO:SUMMARY_sf / FOOTER_sf
  -> AUTO:SUMMARY_frontier / FOOTER_frontier
  -> AUTO:SUMMARY_civilization / FOOTER_civilization
  -> AUTO:SUMMARY_life-culture / FOOTER_life-culture
  -> AUTO:SUMMARY_tech-industry / FOOTER_tech-industry
  -> AUTO:SUMMARY_computer / FOOTER_computer
  -> AUTO:SUMMARY_marketing / FOOTER_marketing
  -> AUTO:SUMMARY_millennium / FOOTER_millennium
  -> AUTO:SUMMARY_dimension / FOOTER_dimension
  -> AUTO:SUMMARY_music / FOOTER_music
  -> AUTO:SUMMARY_linguistics / FOOTER_linguistics
  -> AUTO:SUMMARY_crypto / FOOTER_crypto
  -> AUTO:SUMMARY_astronomy / FOOTER_astronomy
  -> AUTO:SUMMARY_hygiene / FOOTER_hygiene
  -> AUTO:SUMMARY_fantasy
```

The pipeline now correctly enumerates 36 hand-curated → SSOT-divergent
markers (BADGE/COMMON_LINKS/REFERENCE/ROADMAP/ALIEN_INDEX + several
already-aligned SUMMARY/FOOTER ones omitted).

## 5. Critical SSOT vs hand-curation divergence (raw 91 C3 honest)

**The fix only restores pipeline mechanics; it does NOT make the JSON
SSOT authoritative.** README.md currently contains hand-curated
content for ~30 domains (play / aerospace / sf / frontier / ...) that
either (a) have no entry in readme-data.json or (b) have minimal stub
entries (alien=0, tp=0). Applying `hexa run sync-readme.hexa` (write
mode) right now would replace rich hand-curated blocks with stubs like
`> **🛸0** | TP0`, destroying ~25KB of curation.

Post-fix WRITE was attempted in a try-and-revert (raw 142 D2)
verification: 36 markers were written, README.md changed, sealed-hash
broken (own#14 transient FAIL), then `cp /tmp/readme_before.md README.md`
restored byte-equality and own#14 sealed PASS reconfirmed
(sha256: 4a22aa270c17... clean).

So the production rule moving forward:

- **AUTO:BADGE / AUTO:STATS** (global stats only) — safe to auto-sync;
  cycle 18+ can promote these to first-class SSOT.
- **AUTO:SUMMARY_<domain> / AUTO:FOOTER_<domain>** — sync requires
  *first* enriching `readme-data.json` per-domain with the rich content
  currently embedded in README. Cycle 19+ task: "JSON SSOT enrichment
  cycle" — replicate hand-curated content into JSON, then run sync once
  to verify equality (zero changed markers).
- Until SSOT enrichment lands, sync-readme is only run with `--dry-run`
  or `--check`; WRITE mode is gated behind a future `--allow-overwrite`
  flag (added cycle 19+).

## 6. own#14 sealed-hash impact

Pre-fix sealed: `sha256:4a22aa270c17846e300a8e4c0d7aeeadb0d7fa5fb73acb31a02828ca26d57ce1`
Post-fix sealed (after revert): same. README.md byte-equal to pre-cycle.
own#14 PASS preserved. No README.md.sealed.hash update required this cycle.

## 7. Cross-repo impact (raw 47)

Sister repos (anima, nexus, hexa-lang, papers, hive, void) carry their
own README sync stacks. The bug pattern (prefix-confusion in marker
scan) is generic to any `<!-- AUTO:NAME:START -->` / `<!-- AUTO:NAME:END -->`
delimited templating that uses substring-prefix search. A cycle-19
cross-repo audit task is added to next_cycle_path.

## 8. Falsifiers (raw 71)

- **F-SR-FIX-1** — claim: cycle-19+ refactor of `scan_markers` removes
  the validity check without re-introducing the bug.
  monitor: presence of `is_valid_marker_name` in HEAD; firing condition
  removal without test coverage replacement.
- **F-SR-FIX-2** — claim: hexa-runtime upstream classifies JSON null as
  `"null"` / `"nil"` in a future release, breaking the void-only guard.
  monitor: triple-guard (null/nil/void) in `get_or` / `has_field`;
  firing condition removing void check.
- **F-SR-FIX-3** — claim: WRITE mode is run before SSOT enrichment,
  destroying hand-curated SUMMARY/FOOTER blocks.
  monitor: README.md sealed-hash unchanged across cycle 18→19; firing
  condition is unsealed README.md commit without prior JSON SSOT
  enrichment proposal.
- **F-SR-FIX-4** — claim: cross-repo sister has identical bug
  (anima / nexus / hexa-lang / papers / hive / void); audit deferred to
  cycle 19.
- **F-SR-FIX-5** — claim: future hexa-runtime patch fixes string
  classification but breaks `is_valid_marker_name` substring iteration
  (off-by-one in single-char read).
  monitor: smoke-test `hexa run sync-readme.hexa --dry-run` returns
  ≥ 30 changed markers on enriched JSON.

## 9. raw 70 K4 axes

- **CONSTANTS** PASS (n6 quartet sigma=12 phi=2 tau=4 unchanged)
- **DIMENSIONS** PASS (sync-readme.hexa LoC 702 → 758, +56 net)
- **CROSS** PASS (cross-checked against scripts/sync-readme.py legacy +
  cycle-11 /tmp fallback path)
- **SCALING** PASS (64 → 36 marker-change pre/post comparable)
- **SENSITIVITY** PASS (Korean-NAME Edge case considered: rejected by
  ASCII-only `is_valid_marker_name`, which matches readme-data.json
  domain-id convention)
- **LIMITS** PASS (HEAD sealed-hash unchanged; revert verified
  byte-equal)
- **CHI2** DEFER (zero empirical sample; static analysis only)
- **COUNTER** PASS (no marker name in current README contains characters
  outside `[A-Za-z0-9_-]`; future Korean-NAME would re-fire bug)

## 10. Next-cycle handoff

Cycle 19 tasks (rank ordered):
1. **JSON SSOT enrichment** — replicate ~30 hand-curated SUMMARY/FOOTER
   blocks into readme-data.json so `hexa run sync-readme.hexa --dry-run`
   reports 0 changed markers on equality.
2. **WRITE mode gating** — add `--allow-overwrite` flag and CI safeguard
   blocking unintended sealed-hash drift.
3. **Cross-repo audit** (raw 47) — anima / nexus / hexa-lang / papers /
   hive / void sister-repo sync templates audit.
4. **hexa-runtime upstream contribution** (option C) — fix `type_of()`
   classification for JSON null and document substring/index_of
   semantics for multi-byte strings.

## 11. raw 91 C3 honest disclosure

- The cycle-11 hypothesis (HOME PATH + json_parse multibyte) was wrong.
  Real bug was in `scan_markers` prefix confusion + `null`/`void` type
  classification.
- README.md was NOT modified in cycle 18 deliverables; sealed-hash
  preserved.
- The fix re-enables sync mechanics but does NOT restore SSOT
  authority over hand-curated SUMMARY/FOOTER blocks; that requires
  cycle-19 JSON enrichment.
- Live `[DONE] 36 marker(s) changed` proves fix correctness but is NOT
  a green-light for unattended `hexa run sync-readme.hexa` (raw 91 C3:
  WRITE mode would destroy curation; verified by try-and-revert).

## 12. Deliverables

- proposals/hexa_weave_sync_readme_bug_fix_2026_04_28.md (this file)
- design/kick/2026-04-28_sync-readme-bug-fix-cycle18_omega_cycle.json
- state/discovery_absorption/registry.jsonl (one row)
- n6shared/sync-readme.hexa (modified — Option A fix; +56 LoC; reversible
  via git revert if regression)
