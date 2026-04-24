# R29 migration audit report — synbio/verify_alien10.hexa

- Date: 2026-04-11
- Rule: R29 (n6shared/rules/common.json)
- Work: domains/life/synbio/verify_alien10.hexa -> nexus/shared/n6/scripts/verify_synbio_alien10.hexa
- Owner: session agent (R29 separate migration)
- Linked report: merge audit report (separate R29 violation item)

## 1. Background

R29 rule text:

> "Calculator/verifier/scanner .hexa files may only be authored under `nexus/shared/n6/scripts/` —
> common single SSOT across all projects" — (common.json, level DO-NOT-USE)

Forbidden path patterns:
- `<any-project>/theory/**/*.hexa`
- `<any-project>/predictions/*.hexa`
- `<any-project>/proofs/*.hexa`
- `<any-project>/breakthroughs/*.hexa`
- `papers/**/*.hexa`
- `hexa-lang/docs/*.hexa`

Allowed single location:
- `$NEXUS/shared/n6/scripts/`

`domains/life/synbio/verify_alien10.hexa` was a verification-type .hexa file sitting inside a domain polder and falling under R29's umbrella rule ("calculator/verifier/scanner"), thus in violation. It was logged as a separate item in the merge audit report and is handled here in a dedicated report.

However, each domain folder's `verify.hexa` (exactly filename `verify.hexa`) is a "general n=6 identity verification shared template" and is excluded from this migration. It is a required component of the domain `_index.json` + `CLAUDE.md` 15-section template.

## 2. Migration details

| Item | Value |
|------|-----|
| Source path | `$N6_ARCH/domains/life/synbio/verify_alien10.hexa` |
| New path | `$NEXUS/shared/n6/scripts/verify_synbio_alien10.hexa` |
| File size | 10 lines (STUB) |
| md5 (pre-move) | `ee0633a4dedc488b56853e25e9adf900` |
| md5 (post-move binary-identity check) | `ee0633a4dedc488b56853e25e9adf900` |
| Naming convention | `verify_<domain>_<target>.hexa` (domain explicit in the prefix) |
| Source deleted | done (post `rm` `ls` verified) |
| Header comment update | added migration history (2026-04-11 R29) |
| CLAUDE.md update | reflected in `domains/life/synbio/CLAUDE.md` |

### 2.1 File-identity verification

```
MD5 (domains/life/synbio/verify_alien10.hexa) = ee0633a4dedc488b56853e25e9adf900
MD5 (nexus/shared/n6/scripts/verify_synbio_alien10.hexa) = ee0633a4dedc488b56853e25e9adf900
```

md5 match confirmed immediately after migration. Subsequently only the header comment was updated to reflect the migration history; the STUB body logic was not changed.

### 2.2 CLAUDE.md update

The following was reflected in `domains/life/synbio/CLAUDE.md`:

- "Files" section: `verify.hexa` marked as "general n=6 identity verification (shared template, not a migration target)"
- New "External verification scripts (R29)" section: absolute new path of `verify_synbio_alien10.hexa` + run command + reference to this report
- "Usage" section: updated the alien10 verification execution path
- "Rules" section: R29 clause explicit — this folder only permits `verify.hexa` (the shared template)

## 3. R29 violation candidate list (migration not performed)

The command `find domains -name "verify_*.hexa" -not -name "verify.hexa"` shows, beyond synbio, many domain-folder verification .hexa files. This report handles only the synbio case; the rest are left un-migrated and only listed.

### 3.1 Per-axis tally (total 206 items, excluding synbio)

| Axis | Count |
|----|------|
| infra | 43 |
| compute | 35 |
| life (excluding synbio) | 25 |
| physics | 19 |
| cognitive | 16 |
| energy | 15 |
| culture | 14 |
| sf-ufo | 12 |
| sedi | 10 |
| materials | 10 |
| space | 5 |
| brainwire | 2 |
| **Total** | **206** |

Reference: including the 1 synbio item migrated here, the total was 207 at the merge audit report time.

### 3.2 Filename pattern types

- `verify_alien10.hexa` (most common): each domain's "alien-10 verification" stub
- `verify_n6.hexa`: each domain's n=6 verification stub
- `verify_<domain-specific>.hexa`: domain-specific scripts such as sc_exact, cancer10, bt461_470, cern_optics_v2, hdna_fermion_six, hexa_starship
- `verify_paper.hexa`, `verify_paper_p002.hexa`: brainwire paper-linkage verification
- `verify_all_techniques_n6.hexa`, `verify_ai_products_alien10.hexa`: compute/ai-efficiency comprehensive verification

### 3.3 Migration strategy (recommended)

For follow-up operators:

1. **Recommend writing a bulk migration script** — 207 items is infeasible by hand.
   Author and run `nexus/shared/n6/scripts/migrate_r29_verify.hexa`.
2. **Naming convention**: `verify_<domain>_<target>.hexa` (same as this synbio migration).
   Examples: `verify_baking_alien10.hexa`, `verify_cognitive_hexa-mind_n6.hexa`
3. **Collision handling**: the same filename exists in many domains -> the domain prefix is mandatory (collisions in all 207 items).
4. **CLAUDE.md sync**: add the external-verification path link to each domain's CLAUDE.md.
5. **md5 check required**: guarantee binary identity pre/post migration.
6. **Preserve source logic**: for STUBs update the header only, do not change the body logic.
7. **Report the bulk migration as a single atlas.n6 absorption**: per R28, log the migration summary in atlas.n6.

### 3.4 Full violation-candidate file list

The full 206-item list can be regenerated with:

```sh
cd $N6_ARCH
find domains -name "verify_*.hexa" -not -name "verify.hexa" 2>/dev/null | sort
```

(Listing omitted — recoverable via the command above.)

## 4. Rule-compliance checklist

- [x] R1 HEXA-FIRST: migrated file retains .hexa format
- [x] R5 SSOT: migration to single location complete
- [x] R14: rule text is single-source-of-truth in n6shared/rules/common.json
- [x] R18 minimal: only the requested scope (synbio 1 item), other domains only listed
- [x] R28: migration results will be absorbed into atlas.n6 in a follow-up
- [x] R29: synbio migration complete, located only under nexus/shared/n6/scripts/
- [x] English compliance: this report is in English
- [x] HEXA-FIRST: 0 new .py files (no new creation)

## 5. Results

1. `$NEXUS/shared/n6/scripts/verify_synbio_alien10.hexa` — created
2. `$N6_ARCH/domains/life/synbio/verify_alien10.hexa` — deleted
3. `$N6_ARCH/domains/life/synbio/CLAUDE.md` — updated
4. `$N6_ARCH/reports/audits/r29-migration-synbio-2026-04-11.md` — this report
5. Remaining R29 violation candidates: 206 items (see per-axis tally §3.1)

Follow-up: author a bulk script to migrate the remaining 206 R29 violation items in the merge audit report as a single migration initiative. Per R29, the script location is `nexus/shared/n6/scripts/migrate_r29_verify.hexa`.
