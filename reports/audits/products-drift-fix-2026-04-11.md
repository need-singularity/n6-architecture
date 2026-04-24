# products.json Drift Fix Audit Report

- Date: 2026-04-11
- Target: `$NEXUS/shared/n6/docs/products.json` (154 KB, 4,862 lines)
- Operator: Claude Opus 4.6 (GO session)
- Basis: Structural drift between `_meta` and actual sections identified in prior audit
- Backup: `$N6_ARCH/reports/audits/products-backup-2026-04-11.json`

## 1. Drift Found (Before Fix)

| Field | Declared | Actual | Status |
|------|--------|--------|------|
| `_meta.total_products` | 156 | 173 | Missing -17 |
| `_meta.alien_index_order` (len) | 27 | 34 | Missing -7 |
| `_meta.last_updated` | 2026-04-10 | 2026-04-11 (today) | Stale |
| `sections[id=quantum-computer].products[0].ceiling` | false | true (ufo=10) | Grade mismatch |

### 1.1 alien_index_order Missing Sections (7)

Sections present in actual sections but missing from `alien_index_order` array:

1. `tattoo-removal`
2. `keyboard`
3. `mouse`
4. `manufacturing-quality`
5. `network`
6. `quantum-computer`
7. `horology`

### 1.2 ceiling Flag Mismatch

`sections[id=quantum-computer].products[0]` (HEXA-QUANTUM quantum computer architecture):

- `ufo`: 10 (ceiling grade)
- `ceiling`: false  -- when ufo=10, `ceiling=true` is required
- Product description: "20/24 EXACT -- NeutralAtom n=6 atoms, SurfaceCode sigma=12 data qubit, Clifford tau*n=24 gate, Grover sopfr, kissing number BT-49"

## 2. Fixes Applied (4 items)

### 2.1 `_meta.total_products`
```diff
- "total_products": 156,
+ "total_products": 173,
```

### 2.2 `_meta.last_updated`
```diff
- "last_updated": "2026-04-10",
+ "last_updated": "2026-04-11",
```

### 2.3 `_meta.alien_index_order` (insert 7 right after hygiene)

Inserted right after `hygiene` to match physical JSON declaration order (actual sections array order):

```diff
  "hygiene",
+ "tattoo-removal",
+ "keyboard",
+ "mouse",
+ "manufacturing-quality",
+ "network",
+ "quantum-computer",
+ "horology",
  "tech-industry",
```

### 2.4 `sections[quantum-computer].products[0].ceiling`
```diff
  "name": "HEXA-QUANTUM quantum computer architecture",
  "ufo": 10,
- "ceiling": false,
+ "ceiling": true,
```

Note: Section-level `quantum-computer.ceiling` not changed (out of scope -- only products[0].ceiling).

## 3. Post-Fix Verification

```
JSON_VALID
total_products:        173
last_updated:          2026-04-11
alien_index_order_len: 34
actual_sections_count: 34
missing_from_index:    []
extra_in_index:        []
HEXA-QUANTUM.ceiling:  True (ufo=10 match)
actual_total_products: 173 (_meta match)
```

- `python3 -m json.tool` passed -> syntax valid
- `_meta.total_products == sum(len(s.products))` -> **173 == 173** OK
- `len(_meta.alien_index_order) == len(sections)` -> **34 == 34** OK
- `set(alien_index_order) == set(section ids)` -> exact match (missing/extra both 0) OK
- `products[].ufo==10 implies ceiling==true` invariant restored OK

## 4. Existing Product Data Preserved

Per work directive, **only `_meta` block and `HEXA-QUANTUM.products[0].ceiling` flag (1 item)** modified. All per-section `bt_exact_pct`, `alien_index`, `products[*].description`, `domains`, `tools`, `links`, and other actual data unchanged.

## 5. Rule Compliance

- R8 (data remote store): under `n6shared/n6/docs/` -- direct edit allowed scope
- R14 (shared/ JSON single truth): synchronized to match actuals
- R25 (shared-config gate): GO mode + prior audit report basis -> approved scope
- R0-R27 (common): JSON validity / backup / invariant restoration all complied

## 6. Follow-up Recommendations

1. New `tools/audit/products_drift.hexa` (if absent) -> automate daily drift detection
2. Include `_meta.total_products <-> sum(sections.*.products)` invariant in schema validation
3. Enforce `ufo==10 <-> ceiling==true` invariant via pre-commit hook
4. Add helper to auto-regenerate `alien_index_order` from sections array (remove manual sync dependency)

---

**Fields Modified Summary**: `_meta.total_products`, `_meta.last_updated`, `_meta.alien_index_order`, `sections[quantum-computer].products[0].ceiling` (4 fields total). JSON valid, all invariants restored. Total products 156 -> **173**.
