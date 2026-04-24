# Registry path-drift fix audit report (2026-04-11)

> Axis: **reports/audits** · n6-architecture · registry path drift fix
> Rules basis: R2 (no hardcoding) / R5 (SSOT) / R14 (shared JSON single-source-of-truth) / R18 (minimal) / R25 (shared-settings gate)
> Prior audit: `reports/audits/synbio-merge-2026-04-11.md` §3-1 / §3-2
> Scope: GO mode + approved path-drift correction (R18 minimal)

---

## 1. Background — stale pointers flagged in prior audit

In `synbio-merge-2026-04-11.md` §3-1/§3-2, the following stale pointers were deferred under the R25 shared-settings gate:

| Registry | Field | Stale path | Reality |
|---|---|---|---|
| `papers/_registry.json` | sections[tech-industry].products[synthetic-biology].links[0].path | `docs/synbio/goal.md` | **absent** |
| `papers/_registry.json` | sections[tech-industry].products[synthetic-biology].links[1].path | `docs/paper/n6-synthetic-biology-paper.md` | **absent** |
| `papers/_registry.json` | `_meta.papers_chunk_c_2026-04-08.papers[9]` | `docs/paper/n6-synthetic-biology-paper.md` | **absent** |
| `nexus/shared/n6/docs/products.json` | sections[tech-industry].products[synthetic-biology].links[0].path | `docs/synbio/goal.md` | **absent** |
| `nexus/shared/n6/docs/products.json` | sections[tech-industry].products[synthetic-biology].links[1].path | `docs/paper/n6-synthetic-biology-paper.md` | **absent** |
| `nexus/shared/n6/docs/products.json` | `_meta.papers_chunk_c_2026-04-08.papers[9]` | `docs/paper/n6-synthetic-biology-paper.md` | **absent** |
| `nexus/shared/n6/n6_products.json` | sections[tech-industry].products[synthetic-biology].links[0].path | `docs/synbio/goal.md` | **absent** |

### Reality files (absolute-path confirmation)

```
$N6_ARCH/domains/life/synbio/synbio.md   (38640 B)  # merge canonical
$N6_ARCH/domains/life/synbio/goal.md     (13552 B)  # SSOT
$N6_ARCH/papers/n6-synthetic-biology-paper.md  (25702 B)  # paper body
```

---

## 2. Backups (pre-edit)

```
reports/audits/_registry-backup-2026-04-11.json              (original 157741 B)
reports/audits/products-backup-path-fix-2026-04-11.json      (original 154751 B)
reports/audits/n6_products-backup-path-fix-2026-04-11.json   (original  99430 B)
```

Note: the existing `reports/audits/products-backup-2026-04-11.json` is a backup for the structural drift fix in `products-drift-fix-2026-04-11.md` and is unrelated to this task.

---

## 3. Applied edits

### 3-1. `papers/_registry.json`

**(a) synthetic-biology product entry `links` array** (around line 3207)

Before:
```json
"links": [
  { "label": "doc", "path": "docs/synbio/goal.md" },
  { "label": "paper", "path": "docs/paper/n6-synthetic-biology-paper.md" }
]
```

After:
```json
"links": [
  { "label": "domain", "path": "domains/life/synbio/synbio.md" },
  { "label": "goal",   "path": "domains/life/synbio/goal.md" },
  { "label": "paper",  "path": "papers/n6-synthetic-biology-paper.md" }
]
```

- Path `docs/synbio/goal.md` -> `domains/life/synbio/goal.md` (label `doc` -> `goal`)
- Path `docs/paper/n6-synthetic-biology-paper.md` -> `papers/n6-synthetic-biology-paper.md`
- New link added: `domains/life/synbio/synbio.md` (label `domain`, merged canonical body 915 lines)

**(b) `_meta.papers_chunk_c_2026-04-08.papers[9]`** (line 20)

Before:
```
"docs/paper/n6-synthetic-biology-paper.md"
```

After:
```
"papers/n6-synthetic-biology-paper.md"
```

### 3-2. `$NEXUS/shared/n6/docs/products.json`

**(a) synthetic-biology product entry `links` array** (around line 3141)

Same shape as 3-1 (a) in `_registry.json`. 3 new links.

**(b) `_meta.papers_chunk_c_2026-04-08.papers[9]`** (line 20)

Same shape as 3-1 (b) in `_registry.json`.

### 3-3. `$NEXUS/shared/n6/n6_products.json`

synthetic-biology product entry `links` array (around line 2664). Previously held only 1 `doc` link. Expanded to 3 links (domain/goal/paper):

```json
"links": [
  { "label": "domain", "path": "domains/life/synbio/synbio.md" },
  { "label": "goal",   "path": "domains/life/synbio/goal.md" },
  { "label": "paper",  "path": "papers/n6-synthetic-biology-paper.md" }
]
```

---

## 4. Modified-field summary (total 7 items)

| # | File | JSON path | Edit type |
|---|---|---|---|
| 1 | `papers/_registry.json` | `_meta.papers_chunk_c_2026-04-08.papers[9]` | path substitution |
| 2 | `papers/_registry.json` | `sections[tech-industry].products[synthetic-biology].links[0]` | label+path substitution (`doc` -> `goal`) |
| 3 | `papers/_registry.json` | `sections[tech-industry].products[synthetic-biology].links[1]` | path substitution |
| 4 | `papers/_registry.json` | `sections[tech-industry].products[synthetic-biology].links[2]` | new insert (domain body) |
| 5 | `nexus/shared/n6/docs/products.json` | same shape as (1)~(4) above (`_meta` + 3 `links`) | same |
| 6 | `nexus/shared/n6/docs/products.json` | `_meta.papers_chunk_c_2026-04-08.papers[9]` | path substitution |
| 7 | `nexus/shared/n6/n6_products.json` | `sections[tech-industry].products[synthetic-biology].links[]` | expand 1 -> 3 |

Effective individual JSON field edits: **_registry.json 4 + products.json 4 + n6_products.json 3 = 11 fields**.

---

## 5. JSON validity check

```
python3 -m json.tool papers/_registry.json             -> VALID
python3 -m json.tool nexus/shared/n6/docs/products.json  -> VALID
python3 -m json.tool nexus/shared/n6/n6_products.json  -> VALID
```

All 3 files valid per JSON grammar (RFC 8259). Entry structure (keys/types/order) preserved.

---

## 6. Residual drift (out of scope)

### 6-1. Path drift — `docs/synbio/` / `docs/paper/n6-synthetic-biology-paper.md`

| File | Line | Type | Handling |
|---|---|---|---|
| `$N6_ARCH/README.md` | 560 | Manual table after `AUTO:SUMMARY_tech-industry:END` | regenerate via sync-readme.hexa (R5) |
| `$N6_ARCH/n6shared/config/dse-map.toml` | 18775 | `note` comment field | update text or remove (separate item) |
| `$NEXUS/shared/n6/docs/dse-map.toml` | 18775 | same comment field | same as above |
| `$N6_ARCH/reports/audits/paper-legacy-audit/verify-coverage.md` | 114 | historical audit record | **immutable** (reports point-in-time immutability) |
| `$N6_ARCH/reports/audits/synbio-merge-2026-04-11.md` | 28-29, 98-99, 117 | audit report body | **immutable** |
| `$N6_ARCH/reports/audits/_registry-backup-2026-04-11.json` | 20, 3215, 3219 | backup of this task | **immutable** |
| `$N6_ARCH/reports/audits/products-backup*-2026-04-11.json` | 20, 3142/3146/3149/3153 | prior + this task backup | **immutable** |

**Main residual items**: `README.md:560` (sync regeneration needed). `dse-map.toml:18775` (comment only).

### 6-2. Domain-ID drift — `synthetic-biology` (deleted directory)

The `synbio-merge-2026-04-11.md` merge removed `domains/life/synthetic-biology/`, but the `sections[tech-industry].domains[]` array still contains the domain id `"synthetic-biology"` in the following 3 JSONs:

| File | Line | Current value | Actual directory |
|---|---|---|---|
| `papers/_registry.json` | 3157 | `"synthetic-biology"` | `domains/life/synbio/` |
| `nexus/shared/n6/docs/products.json` | 3091 | `"synthetic-biology"` | `domains/life/synbio/` |
| `nexus/shared/n6/n6_products.json` | 2641 | `"synthetic-biology"` | `domains/life/synbio/` |

**Recommendation**: handle `"synthetic-biology"` -> `"synbio"` replacement as a separate item. This task is limited to "path drift" scope (R18 minimal) — domain ID replacement is a separate task requiring full tech-industry section consistency verification.

### 6-3. Synthetic-biology paper file (resolved)

In `synbio-merge-2026-04-11.md` §3-2, there was a recommendation to "include `papers/n6-synthetic-biology-paper.md` as a new creation target in the papers agent (#8) 11-paper expansion". At the time of this audit, that file **already exists** (25702 B, 2026-04-11 20:47). The path references have been updated to this real file.

---

## 7. Rule-compliance checklist

| Rule | Check | Result |
|---|---|---|
| R2 no hardcoding | all paths are repo-relative (standard JSON convention) | OK |
| R5 SSOT | remove stale pointers, reference only real files | OK |
| R10 ossified immutable | convergence ossified block not edited | OK |
| R11 no demotion | no stable -> failed transition | OK |
| R14 rules = JSON | no rule text recorded in CLAUDE.md | OK |
| R18 minimal | only 7 path substitutions + 3 link insertions; no scope creep such as domain ID replacement | OK |
| R25 shared-settings gate | based on GO-mode + `synbio-merge-2026-04-11.md` §3-1/§3-2 approval recommendation | OK |
| R28 atlas SSOT | atlas.n6 not edited (only path fixes) | OK |
| N61 ASCII tri-art | not applicable (JSON edits) | N/A |

---

## 8. Follow-up work (separate items)

1. **Domain-ID replacement** (§6-2): bulk-update `"synthetic-biology"` -> `"synbio"` across 3 files
2. **README regeneration** (§6-1): run `nexus analyze sync-readme` or `sync-readme.hexa` to update line 560
3. **dse-map.toml comment update** (§6-1): update line 18775 `note` (n6-architecture + nexus 2 files)
4. **Synthetic-biology paper N62 completion**: confirm ```python verification code block embed of `papers/n6-synthetic-biology-paper.md` (as a separate audit)
5. **Backup management**: keep this task's 3 backup files (for roll-back); expiry policy is separate

---

## 9. Result indicators

| Metric | Before | After |
|---|---|---|
| Stale `docs/synbio/` paths (across 3 JSONs) | 3 | **0** |
| Stale `docs/paper/n6-synthetic-biology-paper.md` paths (across 3 JSONs) | 4 | **0** |
| synthetic-biology product entry `links` count (_registry.json) | 2 | **3** |
| synthetic-biology product entry `links` count (products.json) | 2 | **3** |
| synthetic-biology product entry `links` count (n6_products.json) | 1 | **3** |
| JSON validity | valid | **valid maintained** |
| Residual path drift (`docs/synbio/` active files) | - | **3** (README + dse-map.toml x2) |
| Residual domain-ID drift (`synthetic-biology`) | - | **3** (across 3 JSON domains[] arrays) |

---

*Created: 2026-04-11 · Editor: claude-opus-4.6 (1M context) · Scope: R18 minimal + R25 gate approval*
