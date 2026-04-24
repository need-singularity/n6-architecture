# R29 Bulk Migration Audit Report — 2026-04-11

- Date: 2026-04-11
- Rule: R29 (n6shared/rules/common.json)
- Work: domains/**/verify_*.hexa (excluding verify.hexa) -> nexus/shared/n6/scripts/
- Script: `nexus/scripts/migrate_r29_verify.hexa`
- Prior report: `reports/audits/r29-migration-synbio-2026-04-11.md` (synbio 1-item migration)

## 1. Background

R29 rule body:

> "Calculator/verification/scanner .hexa must be written only under `nexus/shared/n6/scripts/` --
> a single common SSOT for all projects" (common.json, level block-block)

After the prior synbio migration, a scan found 207 R29 violations still remaining.
This work uses the bulk migration script `nexus/scripts/migrate_r29_verify.hexa` to migrate them all.

`verify.hexa` (common template) is **not** a migration target. It is a required piece of
each domain's `_index.json` + `CLAUDE.md` 15-section template.

## 2. Migration Result Summary

| Item | Value |
|------|-----|
| Target total | 207 |
| Success | 207 |
| Failure | 0 |
| Naming conflict | 0 |
| Remaining (domains/) | 0 |
| md5 match rate | 207/207 (100%) |
| Backup path | `/tmp/r29_backup_20260411` |

## 3. Per-Axis Counts

| Axis | Migrated |
|----|----------|
| brainwire | 2 |
| cognitive | 16 |
| compute | 35 |
| culture | 14 |
| energy | 15 |
| infra | 43 |
| life | 26 |
| materials | 10 |
| physics | 19 |
| sedi | 10 |
| sf-ufo | 12 |
| space | 5 |
| **Total** | **207** |

## 4. Naming Rules

- General form: `verify_<domain>_<stem>.hexa`
- sedi exception: `verify_sedi_<sublayer>_<stem>.hexa` (sedi has 3-level path)
- Examples:
  - `domains/life/baking/verify_alien10.hexa` -> `verify_baking_alien10.hexa`
  - `domains/physics/higgs/verify_alien10.hexa` -> `verify_higgs_alien10.hexa`
  - `domains/brainwire/verify_paper.hexa` -> `verify_brainwire_paper.hexa` (axis direct)
  - `domains/sedi/scripts/cern-optics/verify_cern_optics.hexa` -> `verify_sedi_cern-optics_cern_optics.hexa`
  - `domains/sedi/sources/verify_hdna_fermion_six.hexa` -> `verify_sedi_sources_hdna_fermion_six.hexa`
- Domain defined as basename of the directory immediately above the file. For sedi, axis prefix added to preserve meaning.
- Naming conflicts 0 (all 207 unique)

## 5. Verification

### 5.1 md5 Binary Identity

All migrated files md5-matched before and after the move. If mismatches occurred, it would be recorded as a partial failure and the destination file removed to maintain idempotency.

### 5.2 Remaining R29 Violations

```sh
find domains -name 'verify_*.hexa' -not -name 'verify.hexa' 2>/dev/null | wc -l
# Result: 0
```

**Remaining R29 violations: 0 -- full convergence**

### 5.3 Destination File Count

```sh
ls $NEXUS/shared/n6/scripts/verify_*.hexa | wc -l
# Result: 211
```

## 6. Failure List

No failures. All succeeded.

## 7. Rule Compliance Checklist

- [x] R1 HEXA-FIRST: all migrated files keep .hexa, script also .hexa
- [x] R5 SSOT: single location (`nexus/shared/n6/scripts/`) convergence
- [x] R14: rule body in `n6shared/rules/common.json` single truth (cited only)
- [x] R18 Minimal: requested scope only -- no logic modification, just file move
- [x] R19 SILENT EXIT forbidden: all errors stdout + report logged
- [x] R21 No blocking: find / md5 / cp called individually, no large blocking
- [x] R22: bash single interpreter + absolute-path shq quoting
- [x] R28: this migration result is a follow-up atlas.n6 absorption target
- [x] R29: all violations converged to single location, `verify.hexa` common template preserved
- [x] English required: this report in English

## 8. Rollback Guide

Backup files are kept in `/tmp/r29_backup_20260411` in `<axis>__<domain>__<filename>` format. To restore:

```sh
# 1) Remove new file from destination
rm $NEXUS/shared/n6/scripts/verify_<domain>_*.hexa
# 2) Copy from backup to original path
# (domain path restored as domains/<axis>/<domain>/)
```

## 9. Per-Domain CLAUDE.md Sync

Follow-up script `nexus/scripts/sync_r29_claude_md.hexa` extracted (axis, domain) groups from backup filenames and appended an "External Verification Scripts (R29)" section at the bottom of each domain CLAUDE.md.

| Item | Value |
|------|-----|
| Domain group count | 189 |
| CLAUDE.md updated (auto) | 186 |
| Already processed (skipped) | 0 |
| CLAUDE.md missing (auto skipped) | 3 |

The 3 missed by the auto script had different path structures and were corrected manually:

- `domains/brainwire/CLAUDE.md` -- brainwire is a single axis-direct domain (manual add done)
- `domains/sedi/CLAUDE.md` -- sedi axis has 2 sub-paths (scripts/cern-optics/, sources/), so axis-level CLAUDE.md got all 10 appended (manual add done)
- `domains/sedi/scripts/cern-optics/CLAUDE.md` / `domains/sedi/sources/CLAUDE.md` did not originally exist (integrated into upper `sedi/CLAUDE.md`)

Total domain CLAUDE.md sync: **189/189 (100%)**

## 10. Follow-ups

1. atlas.n6 auto absorption (R28): record migration result as summary claim
2. Weekly_audit.hexa next-week run confirms R29 violations 0
3. Backup directory cleanup (7 days later): `rm -rf /tmp/r29_backup_20260411`

## 11. Migration Scripts

- `nexus/scripts/migrate_r29_verify.hexa` -- 207-item bulk migration (md5 verification included)
- `nexus/scripts/sync_r29_claude_md.hexa` -- per-domain CLAUDE.md auto sync

R29 exception: nexus/scripts is the migration agent so allowed (common rule targets only filenames with `verify_` prefix; migration scripts use `migrate_`, `sync_` prefix).

---

*This report was auto-generated by `nexus/scripts/migrate_r29_verify.hexa` and manually corrected.*
