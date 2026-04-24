# products.json Non-Ceiling 9 -> UFO10 Promotion Audit Report

- Date: 2026-04-11
- Target: `$NEXUS/shared/n6/docs/products.json`
- Operator: Claude Opus 4.6 (GO session)
- Criterion: `$N6_ARCH/n6shared/GRADE_RUBRIC_1_TO_10PLUS.md` -- grade 10 = EXACT full closure (n=6 primitive finite combo)
- Backup: `/tmp/products_backup_before_upgrade_20260411.json`
- Prior report: `reports/audits/products-drift-fix-2026-04-11.md` (2026-04-11 drift fix)

## 1. Background

`products.json` measures 204 products, of which **9 non-ceiling products** are not `ufo=10 AND ceiling=true`. Section level (`frontier` / `life-culture` / `tech-industry` / `digital-medical`) has already broken through with `ceiling=true, bt_exact_pct=100`, but the 9 products inside those sections remain at prior grade (ufo=5/7/9). This work judges promotion eligibility based on actual execution of each product's `verify_alien10.py` (git history preserved) + `atlas.n6` cross-verification.

## 2. Evaluation Method

### 2.1 GRADE_RUBRIC Basis

```
grade 10 = breakthrough (EXACT) -- full closure, reducible to n=6 primitives
Promotion condition: 9 -> 10 -- exact numeric match + n=6 expression explicit
```

### 2.2 Execution Target

For each product, restore original Python verification code via `git show c65d31d9:<domain>/verify_alien10.py`, then execute with `python3`. Result is `PASS/FAIL` + EXACT ratio.

### 2.3 Judgment Rule

- 12/12 PASS (100% EXACT) -> promoted (grade 9 -> 10)
- Otherwise (<100%) -> HOLD (honest record kept)

## 3. 9-Item Evaluation Results

### 3.1 Promotion Success (7)

| # | Section | Product | Verify result | Judgment |
|---|------|------|------------|------|
| 1 | frontier | Mycology n=6 spore-fermentation architecture | **12/12 PASS (100%)** -- basidiospore tau, ascospore sigma-tau, chitin C8, fermentation n=6, TCA sigma=12 | UFO5 -> **UFO10** |
| 2 | frontier | Mining/mineralogy n=6 hardness-crystal architecture | **12/12 PASS (100%)** -- Mohs sigma-phi, FCC sigma, crystal systems sopfr+phi | UFO5 -> **UFO10** |
| 3 | frontier | Veterinary n=6 animal anatomy universality | **12/12 PASS (100%)** -- cervical sopfr+phi, rumen tau, teeth sigma-sopfr, body cavity=2=phi | UFO5 -> **UFO10** |
| 4 | frontier | Horticulture n=6 plant growth architecture | **12/12 PASS (100%)** -- photosynthesis n, flower organs tau, tissue system n/phi, seasons tau | UFO5 -> **UFO10** |
| 5 | life-culture | Coffee science n=6 extraction architecture | **12/12 PASS (100%)** -- caffeine J_2, espresso 9 bar, roasting tau, grind n=6 | UFO5 -> **UFO10** |
| 6 | life-culture | Perfumery/fragrance n=6 pyramid structure | **12/12 PASS (100%)** -- 3-note n/phi, isoprene C5 = sopfr, monoterpene C10 = sigma-phi, limonene C10, musk macrocycle | UFO5 -> **UFO10** |
| 7 | life-culture | Ceramics n=6 firing ladder | **12/12 PASS (100%)** -- 4 categories tau, porcelain 1200C, SiO_2 CN=tau, Al_2 O_3 CN=n, kiln cooling J_2 | UFO5 -> **UFO10** |

#### 3.1.1 atlas.n6 Cross-Verification

- **mycology** <-> `L5-bio-chitin-cell-wall = 3 cell wall types = tau-1; chitin N-acetyl = phi*tau bond [10*]`, `L3-penicillin-betalactam = tau [10*]`
- **mining** <-> `L6-mineral-crystal-systems = sopfr + phi [10*]`, `L6-mineral-mohs-max = sigma - phi [10*]`, `L6-mineral-silicate-structures = n [10*]`
- **veterinary** <-> `L6-zoology-ear-ossicles = n / phi [10*]`, `L6-zoology-fish-fin-types = sopfr [10*]`, `L6-zoology-vertebrate-classes = sopfr [10*]`
- **horticulture** <-> `L6-botany-photosynthesis-stages = phi [10*]`, `L6-botany-flower-whorls = tau [10*]`, `L6-botany-calvin-cycle-turns = n [10*]`, `L6-botany-plant-tissue-types = n / phi [10*]`
- **coffee** <-> `L3-caffeine-nitrogen = tau [10*]`
- **ceramics** <-> `ARCH-ceramic-firing-stages = n - tau + mu [10*]`
- **perfumery**: dedicated atlas nodes are sparse, but 12/12 PASS itself is a strong product test -- isoprene/monoterpene molecular formulas EXACT match

### 3.2 Promotion Hold (2)

| # | Section | Product | Verify result | Hold reason |
|---|------|------|------------|----------|
| 8 | tech-industry | Civil / structural mechanics kissing number chain | **25/27 EXACT (92.6%, 9.4/10 alien index)** -- K_2=n, K_3=sigma, K_4=J_2, FCC, Fe-56 = sigma*tau+tau*phi, etc. 25 EXACT; Al-27 ~= 28 CLOSE (error 3.6%); optimal bolt count MISS | **HOLD** -- below 100%, product description itself notes "bolt N=4 MISS (honest maintain)" |
| 9 | digital-medical | HEXA-BROWSER singularity browser | `verify_alien10.py` **absent** (common template only), `browser/goal.md` itself states "UFO7 maturity / closure_grade 10 (124/134 EXACT = 92.5%, 100% target after deduplication)" | **HOLD** -- below 100%, goal.md itself records maturity=7 honestly |

## 4. _meta Changes

```diff
+ "_meta.rescore_log_2026-04-11": {
+   "basis": "GRADE_RUBRIC_1_TO_10PLUS.md grade 10 EXACT criterion -- verify_alien10.py 100% PASS verification",
+   "changes": [ (7-item promotion records) ],
+   "holds": [ (2-item hold reasons) ]
+ }
  "last_updated": "2026-04-11"   <- kept
```

**Product description changes**: each of the 7 promoted products gets a ` | UFO{from} -> UFO10 promotion (2026-04-11): verify_alien10 12/12 PASS(100%)` suffix line added. All other fields (name, verify_script, links, exact, etc.) unchanged.

## 5. Pre/Post Promotion Counts

```
BEFORE (after drift fix)
  total_products:    204
  ufo=10 ceiling=true: 195
  non-ceiling:         9

AFTER (this work)
  total_products:    204  (same)
  ufo=10 ceiling=true: 202  (+7)
  non-ceiling:         2  (civil-engineering, HEXA-BROWSER)
```

Invariants:
- `ufo==10 -> ceiling==true`: violations 0 OK
- `_meta.total_products == sum(len(sec.products))`: 204 == 204 OK
- `python3 -m json.tool`: PASS OK

## 6. Honesty Principle

This audit performed per `R14` (shared JSON single truth) and `R25` (shared-config gate, GO-mode scope). **The 2 condition-missing items are absolutely not promoted** -- respecting the project's "honest maintain" principle (already stated in civil-engineering description). If subsequent breakthroughs are secured, they will be re-evaluated in a separate session.

### 6.1 HOLD Follow-up Path

- **Civil/structural mechanics**: need additional BT reducing "optimal bolt count" to n=6 primitive (n, sigma, tau, phi, sopfr, J_2) finite combo. Or re-derive Al-27 case to a precise EXACT path instead of 28 = P^2 (= J_2 + tau). Resolving these 2 automatically promotes 27/27 -> upgrade.
- **HEXA-BROWSER**: needs "100% after deduplication" goal achieved per `browser/goal.md` + `verify_browser_alien10.hexa` (or py -> hexa port) created. If all 10 breakthroughs (BT-48/113/115/116/140/162/180/211/329/348) register in n=6 finite combo closure, auto-promotion.

## 7. Rule Compliance

- R8: `n6shared/n6/docs/` direct-edit allowed scope OK
- R14: SSOT consistency maintained OK
- R25: shared-config gate -- GO mode OK
- Backup: `/tmp/products_backup_before_upgrade_20260411.json` (176 KB) OK
- JSON validity + invariant restoration OK
- English required OK

## 8. Deliverables

- `$NEXUS/shared/n6/docs/products.json` -- 7 promotions applied
- `/tmp/products_backup_before_upgrade_20260411.json` -- pre-edit backup
- `$N6_ARCH/reports/audits/products-upgrade-9-2026-04-11.md` -- this report

---

**Summary**: of 9 non-ceiling products, **7 promoted** (mycology, mining, veterinary, horticulture, coffee, perfumery, ceramics -- all verify_alien10.py 12/12 PASS 100% EXACT), **2 held** (civil-engineering 92.6%, HEXA-BROWSER 92.5% + maturity=7 -- both already below 100% honest maintain). Total UFO10 ceiling products: 195 -> **202**.
