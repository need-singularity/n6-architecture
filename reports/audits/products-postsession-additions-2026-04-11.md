# products.json Residual Drift Integrated Cleanup — 2026-04-11 Post-Session

| Item | Value |
|------|----:|
| Target file | `$NEXUS/shared/n6/docs/products.json` |
| Backup | `reports/audits/products-backup-2026-04-11-postsession.json` |
| Migration script | `/tmp/n6_migrations/products_postsession_2026_04_11.py` |
| Execution date | 2026-04-11 |
| Rule basis | R5 / R10 / R18 / R19 / R22 |
| Prior audit | `reports/audits/readme-products-drift-2026-04-11.md` (Agent 2) |
| Prior audit | `reports/audits/products-link-remap-2026-04-11.md` (Agent 4) |

---

## 1. Work Scope and Result

| # | Task | Prior state | Result |
|---|------|-----------|-----------|
| 1 | HEXA-BCI -> add `digital-medical` section | README L728 standalone | 1 product inserted, `brain-computer-interface` added to `domains[]` |
| 2 | `energy` section HEXA-AUTO missing fill | **already present** (products.json 5th product) | **NOOP** -- drift direction README<->products has products side excess. products.json as SSOT preserved |
| 3 | `audio` section HEXA-BONE/EAR-CELL/SPEAKER missing fill | **already present** (5, 6, 7th products) | **NOOP** -- same reason, products.json as SSOT preserved |
| 4 | 6 orphan sections new registration | README standalone (L793-910) | `millennium/dimension/music/linguistics/crypto/astronomy` newly registered (30 products total) |
| 5 | `_meta` sync | total=173, order 34 items | total=204, order 40 items (append 6 new section ids) |
| 6 | Re-parse verification | — | `json.load` PASS, measured total match |

Tasks 2 and 3 per R5 SSOT principle: **products.json is already correct**. Drift against README is a README generator problem, not within this work's scope (products.json SSOT). (See `readme-products-drift-2026-04-11.md` body lines 29-33: drift direction products.json -> README is **products excess**.)

---

## 2. Count Changes (measured)

```
BEFORE  sections = 34   total_products = 173
AFTER   sections = 40   total_products = 204
delta   sections = +6   total_products = +31
```

New additions distribution:

| Section | Products | BT range | Ceiling |
|------|-----:|---------|:---:|
| `digital-medical` (+1) | 3 -> 4 | BT-48/113 etc. | yes HEXA-BCI 1 |
| `millennium` (new) | — -> 7 | BT-541-547 | yes |
| `dimension` (new) | — -> 7 | BT-588-597 | yes |
| `music` (new) | — -> 4 | BT-598-607 | yes |
| `linguistics` (new) | — -> 4 | BT-608-617 | yes |
| `crypto` (new) | — -> 4 | BT-618-627 | yes |
| `astronomy` (new) | — -> 4 | BT-628-637 | yes |
| **Total additions** | | | **+31** |

HEXA-BCI 1 + 6 orphan sections 30 = 31 new products.

---

## 3. Section Size Distribution (ASCII)

```
                          products ->
 frontier               |###############################################################################################| 39
 tech-industry          |######################|                  22
 ai                     |#########|                                9
 life-culture           |#########|                                9
 audio                  |#######|                                  7
 civilization           |#######|                                  7
 millennium (new)       |#######|                                  7
 dimension (new)        |#######|                                  7
 environment            |######|                                   6
 materials              |######|                                   6
 cognitive-social       |######|                                   6
 fusion                 |#####|                                    5
 chip                   |#####|                                    5
 energy                 |#####|                                    5
 physics                |#####|                                    5
 software               |#####|                                    5
 digital-medical (+1)   |####|                                     4
 virology               |####|                                     4
 natural-science        |####|                                     4
 marketing              |####|                                     4
 music (new)            |####|                                     4
 linguistics (new)      |####|                                     4
 crypto (new)           |####|                                     4
 astronomy (new)        |####|                                     4
 robotics               |##|                                       2
 display                |##|                                       2
 safety                 |##|                                       2
 play                   |##|                                       2
 mobility               |##|                                       2
 hygiene                |##|                                       2
 aerospace              |#|                                        1
 sf                     |#|                                        1
 hiv-treatment          |#|                                        1
 tattoo-removal         |#|                                        1
 keyboard               |#|                                        1
 mouse                  |#|                                        1
 manufacturing-quality  |#|                                        1
 network                |#|                                        1
 quantum-computer       |#|                                        1
 horology               |#|                                        1
```

---

## 4. Section Count Change (ASCII)

```
section count | before 34 ================ after 40
 0    10   20   30   34   40
 |----|----|----|---|----|
 ####################       <- 34 (before)
 #######################    <- 40 (after, +6)
                    ^  ^
                    |  +-- millennium/dimension/music/linguistics/crypto/astronomy
                    +-- baseline
```

---

## 5. `_meta` Change Diff

```diff
   "_meta": {
-    "total_products": 173,
+    "total_products": 204,
     "total_domains": 145,
     "total_papers": 128,
     ...
     "last_updated": "2026-04-11",
     ...
     "alien_index_order": [
       "fusion", "chip", "energy", "ai", "environment", "physics",
       "materials", "robotics", "software", "display", "audio",
       "safety", "play", "aerospace", "sf", "frontier",
       "civilization", "life-culture", "hygiene", "tattoo-removal",
       "keyboard", "mouse", "manufacturing-quality", "network",
       "quantum-computer", "horology", "tech-industry", "virology",
       "hiv-treatment", "natural-science", "cognitive-social",
       "mobility", "digital-medical", "marketing"
+      , "millennium", "dimension", "music", "linguistics", "crypto", "astronomy"
     ],
     ...
   }
```

`last_updated` kept at 2026-04-11.

---

## 6. New Sections Common Schema

Each new section follows the `digital-medical` / `hygiene` / `horology` schema:

```json
{
  "id": "<section-id>",
  "title": "<title>",
  "icon": "<emoji>",
  "heading": "<english/bilingual>",
  "alien_index": 10,
  "ceiling": true,
  "bt_exact_pct": <number>,
  "bt_count": <number>,
  "industry_pct": null,
  "industry_detail": null,
  "experiment_pct": <number>,
  "experiment_detail": "<BT all ... EXACT>",
  "physics_limit_count": 0,
  "tp_count": 0,
  "discovery_count": <number>,
  "cross_dse_domains": 0,
  "mk_count": 5,
  "summary_extra": "<README SUMMARY excerpt>",
  "domains": ["..."],
  "tools": [],
  "products": [...],
  "bt_detail": "BT-xxx~yyy <section name> -- N/M EXACT P%"
}
```

Each product has `{name, ufo, ceiling, ver, description, links, verify_script?}` structure. `verify_script` only filled when `domains/**/verify.hexa` actually exists.

## 7. Domain Path Check (actual)

HEXA-BCI actual path: `domains/cognitive/brain-computer-interface/goal.md`
(README L728 lists it as `docs/brain-computer-interface/goal.md`, which is the README generator's abbreviation; actual file location is under `domains/cognitive/`. products.json uses actual path.)

Verified existing files:

```
domains/cognitive/brain-computer-interface/goal.md          PRESENT
domains/cognitive/brain-computer-interface/verify.hexa      PRESENT
domains/physics/millennium-riemann/goal.md                  PRESENT
domains/physics/millennium-p-vs-np/goal.md                  PRESENT
domains/physics/millennium-yang-mills/goal.md               PRESENT
domains/physics/millennium-navier-stokes/goal.md            PRESENT
domains/physics/millennium-hodge/goal.md                    PRESENT
domains/physics/millennium-bsd/goal.md                      PRESENT
domains/physics/millennium-poincare/goal.md                 PRESENT
domains/compute/hexa-holo/goal.md                           PRESENT
domains/compute/display-8stack/goal.md                      PRESENT
domains/compute/consciousness-chip/goal.md                  PRESENT
domains/culture/music/goal.md                               PRESENT
domains/culture/linguistics/goal.md                         PRESENT
domains/compute/software-crypto/goal.md                     PRESENT
domains/physics/particle-cosmology/goal.md                  PRESENT
domains/space/space-systems/goal.md                         PRESENT
```

No guessed paths. `docs/breakthrough-theorems.md` link kept with the same notation as README (existence confirmed).

---

## 8. Demonstration / Invariant Handling

- **R10 ossified invariant**: `_meta.closure_grade_12_log_2026-04-05`, `rescore_log_*`, and `PRODUCTS_173_REMAP_582` related blocks **not touched**.
- This work is a **follow-on increment** to `PRODUCTS_173_REMAP_582`; a separate `PRODUCTS_178_PLUS_6_SECTIONS` marker is replaced by this report. (Subsequent convergence / ossified promotion to be done in a separate session.)

---

## 9. Remaining Tasks (after this work)

| Item | Status | Note |
|------|------|------|
| README <-> products.json sync | **unresolved** | `readme-products-drift-2026-04-11.md` body points out additional `energy +1`, `audio +3`, but per R5 SSOT, **README direct-edit forbidden**. `sync_products_readme.hexa` automation is the correct answer. |
| `sync_products_readme.hexa` STUB release | **unresolved** | Needed in a later session after this work |
| `PRODUCTS_204_POSTSESSION_578` ossified promotion | **pending** | After re-verification + threshold numerical confirmation, to be done in a separate session if it passes convergence ops |
| README L581 HEXA-QC, L519 horology duplicate row removal | **unresolved** | Agent 2 audit pointed this out -- README-side work (outside this work's scope) |
| `fantasy` orphan section (README line 931) | **policy undecided** | Audit flagged as "myth/fantasy -- not an engineering design target" warning. SSOT acceptance decision needed |

---

## 10. JSON Integrity Verification

```
$ python3 -c "import json; json.load(open('.../products.json'))"
(exit 0, parse success)

sections=40, total_products=204, measured_match=True
alien_index_order len=40, sections len=40, diff=[]
```

R19 compliance: all exception blocks in the migration script do stderr output then `sys.exit(1)`.

---

## 11. Rule Compliance Summary

| Rule | Content | Complied |
|-----|------|:---:|
| R5 | SSOT -- README direct-edit forbidden, only products.json edited | yes |
| R10 | ossified invariant -- `rescore_log_*`, existing section contents not touched | yes |
| R14 | Rule body is n6shared/config/absolute_rules.json | yes (report is data) |
| R18 | Minimal -- only 6 specified tasks, no further expansion | yes |
| R19 | SILENT EXIT forbidden -- all exceptions stderr logged | yes (`die()` function) |
| R22 | Script interpreter absolute path -- `/usr/bin/python3` | yes |
| English required | Report in English | yes |
