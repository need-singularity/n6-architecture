# goal.md Axis Expansion Audit — 20 -> 295

> **Date**: 2026-04-11
> **Background**: The `n6shared/convergence/n6-architecture.json` GOAL_MD_20 (ossified) entry recorded "20 created", but filesystem investigation found that **only 1 standalone goal.md file existed (`domains/compute/software-design/hexa-ssh/goal.md`, nested path)**. The remaining 294 domains had their `goal.md` content absorbed into the `### Source: goal.md` section inside their integrated `<name>.md` files.
> **Goal**: create a `domains/<axis>/<name>/goal.md` standalone file for every domain in the 295 domain SSOT (`domains/_index.json`).

---

## 1. Final Generation Result

| Axis | List (`_index.json`) | Folder exists | Created | Extraction | Template |
|----|------|------|------|------|------|
| physics | 37 | 37 | **37** | 37 | 0 |
| life | 48 | 48 | **48** | 47 | 1 |
| energy | 16 | 16 | **16** | 16 | 0 |
| compute | 47 | 47 | **47** | 47 | 0 |
| materials | 19 | 19 | **19** | 19 | 0 |
| space | 8 | 8 | **8** | 8 | 0 |
| infra | 57 | 57 | **57** | 57 | 0 |
| cognitive | 21 | 21 | **21** | 21 | 0 |
| culture | 25 | 25 | **25** | 25 | 0 |
| sf-ufo | 17 | 17 | **17** | 17 | 0 |
| **Total** | **295** | **295** | **295** | **294** | **1** |

> `compute/software-design/hexa-ssh/goal.md` (nested path) is not in `_index.json`; it is a sub-domain and existed previously. Excluded from the 295 count.
> Under `compute/` there are 4 sub-folders not registered in `_index.json` (`chip-dse-pipeline`, `chip-isa-n6`, `chip-npu-n6`, `chip-rtl-gen`) -- excluded from this work (SSOT basis).

---

## 2. Generation Strategy

### 2.1 Extraction (Primary: 294/295)

Used regex `### Source: \`goal\.md\`\n(.*?)(?=\n### Source: |\Z)` to reverse-extract the folded goal.md section from each domain's integrated `<name>.md` -> attach axis/path header (2 lines) -> record as standalone `goal.md`.

- Merit: **lossless preservation** of rich bodies written by the original domain agents (real-life effect tables, ASCII structural diagrams, DSE Pareto, BT links, prediction verification tables).
- File size distribution: min 1.7 KB, max 168 KB, avg 17 KB.
- Parent integrated `<name>.md` is **not modified** (R14 SSOT principle, folded original preserved intentionally -- dual SSOT maintained).

### 2.2 Template (Fallback: 1/295)

For a domain without a folded goal.md section, a 4-section English template (1 Domain Goal / 2 n=6 Consistency Points / 3 Verification Metrics / 4 Linked BT/Techniques/atlas constants) is used as a draft.

- **Target**: `life/womens-intimate-cleanser` (1)
- Reason: the domain's `<name>.md` lacks the `### Source: goal.md` section
- Follow-up: the domain owner agent needs to fill in specific constants / BT / technique links

---

## 3. Template Structure (for template-method domains)

```markdown
# <name> -- n=6 Design Goal

> Axis: **<axis>** (<axis>) * n6-architecture * auto-generated template

## 1. Domain Goal
## 2. n=6 Consistency Points (n=6 / sigma=12 / tau=4 / phi=2 / sopfr=5 / J_2=24)
## 3. Verification Metrics (table)
## 4. Linked BT * Techniques * atlas constants
```

---

## 4. Verification Sample Paths

- `$N6_ARCH/domains/physics/higgs/goal.md` (5.6 KB, extraction)
- `$N6_ARCH/domains/compute/blockchain/goal.md` (7.0 KB, extraction)
- `$N6_ARCH/domains/cognitive/reality-map/goal.md` (extraction)
- `$N6_ARCH/domains/life/womens-intimate-cleanser/goal.md` (1.7 KB, template)

---

## 5. SSOT Update Recommendation

Recommend updating `n6shared/convergence/n6-architecture.json` `GOAL_MD_20` entry to:

```json
"GOAL_MD_295": {
  "status": "PASS",
  "value": "295 domains standalone goal.md created (294 extraction + 1 template)",
  "threshold": "every domain (domains/_index.json) has goal.md",
  "note": "2026-04-11 axis expansion 20 -> 295. Existing folded <name>.md sections reverse-extracted losslessly. 1 case (life/womens-intimate-cleanser) is a template draft",
  "ossified_at": "2026-04-11",
  "promoted_from": "goal_md_expansion"
}
```

---

## 6. Missing / Caveats (<200 words reason report)

**Missing: 0**. 295/295 all generated.

**Caveats: 3**:

1. **Convergence record vs actual filesystem mismatch**: `GOAL_MD_20` recorded PASS with "20 created", but the actual standalone files were 1 (nested path hexa-ssh). This is presumed to be due to the historical convention of counting folded-method (absorption into `<name>.md` body's `### Source: goal.md` section) as "created". This work physicalizes them into standalone form and restores record-actual consistency.

2. **Template draft 1 case**: `life/womens-intimate-cleanser` had no folded goal.md section in `<name>.md`, so a 4-section English template was used as draft. Needs follow-up filling by the domain owner agent (parent BT search, constant mapping, verification scenario).

3. **Dual SSOT phenomenon**: the 294 extracted standalone goal.md files are **duplicates** of the `<name>.md` original folded sections. This is a borderline case for R14 (single truth). Response: (a) recommend replacing the `<name>.md` folded sections with stubs like `This section's single truth = see goal.md` in a later session, or (b) keep goal.md as a reference copy notation. This session made **no changes** (prioritizing the lossless-original principle).
