# Un-migrated design-MD 48-item investigation report

- Created: 2026-04-11
- Source reference: `reports/audits/products-link-remap-2026-04-11.md` (MISS 174 items)
- Investigation scope: 48 items of pattern `docs/<dom>/<nested>.md` among the 174 MISS (excluding 116 paper, 10 calc)
- Base path: `$N6_ARCH/`
- Method: read-only (investigation only, no migration)
- Target domains: 24 / 6 of 9 axes (compute · culture · energy · infra · life · materials)

## Summary (at a glance)

- Of 48 total, **46 FOUND_INTEGRATED** (95.8%) — already absorbed into `domains/<axis>/<dom>/<dom>.md` consolidations
- **2 FOUND_ALT (DIR)** — directory entries, mapped to `domains/infra/environmental-protection`
- **0 MISSING** — no real ghost files requiring authoring (all integrated)
- **0 FOUND_AT_ORIG** — no files actually present at the original `docs/<dom>/...` paths

Key finding: all 48 items have been absorbed during consolidation. Since products.json links point at the old `docs/` structure, merely updating the links converges MISS to 0.

## Investigation method

1. Extracted 175 `| MISS |` rows from `products-link-remap-2026-04-11.md`
2. Excluded `docs/paper/` (116) + `calc/*.py` (10) + 1 table header -> 48 items
3. 3-tier search per item:
   - **Original-path existence**: whether `docs/<rel>` exists
   - **Directory-entry handling**: if the path ends with `/`, search `domains/*/<dom>`
   - **Consolidation-absorption check**: grep `### Source: \`<nested>\`` or `### Source: \`<basename>\`` header in `domains/<axis>/<dom>/<dom>.md`
4. On failure: domain-internal -> global basename search (careful with shared filenames such as hypotheses.md)

## 1. Un-migrated MD 48-item list

| No | Original link (products.json basis) | Actual location | Class |
|---:|---|---|---|
| 01 | `docs/ai-efficiency/techniques-complete.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | FOUND_INTEGRATED |
| 02 | `docs/ai-efficiency/full-verification-matrix.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | FOUND_INTEGRATED |
| 03 | `docs/ai-efficiency/next-model-blowup-2026-04.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | FOUND_INTEGRATED |
| 04 | `docs/ai-efficiency/bt-391-code-generation.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | FOUND_INTEGRATED |
| 05 | `docs/ai-efficiency/bt-397-n6-novel-architectures.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | FOUND_INTEGRATED |
| 06 | `docs/audio/full-verification-matrix.md` | `domains/culture/audio/audio.md` | FOUND_INTEGRATED |
| 07 | `docs/audio/hexa-ear-ultimate.md` | `domains/culture/audio/audio.md` | FOUND_INTEGRATED |
| 08 | `docs/audio/hexa-bone-ultimate.md` | `domains/culture/audio/audio.md` | FOUND_INTEGRATED |
| 09 | `docs/audio/hexa-ear-cell.md` | `domains/culture/audio/audio.md` | FOUND_INTEGRATED |
| 10 | `docs/audio/hexa-speaker-ultimate.md` | `domains/culture/audio/audio.md` | FOUND_INTEGRATED |
| 11 | `docs/chip-architecture/ultimate-consciousness-soc.md` | `domains/compute/chip-architecture/chip-architecture.md` | FOUND_INTEGRATED |
| 12 | `docs/chip-architecture/hexa-topological-performance-chip.md` | `domains/compute/chip-architecture/chip-architecture.md` | FOUND_INTEGRATED |
| 13 | `docs/chip-architecture/hexa-asic-skywater.md` | `domains/compute/chip-architecture/chip-architecture.md` | FOUND_INTEGRATED |
| 14 | `docs/chip-architecture/full-verification-matrix.md` | `domains/compute/chip-architecture/chip-architecture.md` | FOUND_INTEGRATED |
| 15 | `docs/horology/hypotheses.md` | `domains/culture/horology/horology.md` | FOUND_INTEGRATED |
| 16 | `docs/display/full-verification-matrix.md` | `domains/compute/display/display.md` | FOUND_INTEGRATED |
| 17 | `docs/battery-architecture/hexa-auto-battery.md` | `domains/energy/battery-architecture/battery-architecture.md` | FOUND_INTEGRATED |
| 18 | `docs/environmental-protection/` | `domains/infra/environmental-protection` (DIR) | FOUND_ALT |
| 19 | `docs/environmental-protection/microplastics-solution.md` | `domains/infra/environmental-protection/environmental-protection.md` | FOUND_INTEGRATED |
| 20 | `docs/environmental-protection/evolution/` | `domains/infra/environmental-protection` (DIR) | FOUND_ALT |
| 21 | `docs/environmental-protection/testable-predictions-2030.md` | `domains/infra/environmental-protection/environmental-protection.md` | FOUND_INTEGRATED |
| 22 | `docs/mycology/hypotheses.md` | `domains/life/mycology/mycology.md` | FOUND_INTEGRATED |
| 23 | `docs/mining/hypotheses.md` | `domains/infra/mining/mining.md` | FOUND_INTEGRATED |
| 24 | `docs/veterinary/hypotheses.md` | `domains/life/veterinary/veterinary.md` | FOUND_INTEGRATED |
| 25 | `docs/horticulture/hypotheses.md` | `domains/life/horticulture/horticulture.md` | FOUND_INTEGRATED |
| 26 | `docs/fusion/evolution/mk-1-first-light.md` | `domains/energy/fusion/fusion.md` | FOUND_INTEGRATED |
| 27 | `docs/fusion/alien-level-discoveries.md` | `domains/energy/fusion/fusion.md` | FOUND_INTEGRATED |
| 28 | `docs/fusion/physical-limit-proof.md` | `domains/energy/fusion/fusion.md` | FOUND_INTEGRATED |
| 29 | `docs/hiv-treatment/evolution/mk-1-basic.md` | `domains/life/hiv-treatment/hiv-treatment.md` | FOUND_INTEGRATED |
| 30 | `docs/hiv-treatment/evolution/mk-2-short.md` | `domains/life/hiv-treatment/hiv-treatment.md` | FOUND_INTEGRATED |
| 31 | `docs/hiv-treatment/evolution/mk-3-mid.md` | `domains/life/hiv-treatment/hiv-treatment.md` | FOUND_INTEGRATED |
| 32 | `docs/hiv-treatment/evolution/mk-4-long.md` | `domains/life/hiv-treatment/hiv-treatment.md` | FOUND_INTEGRATED |
| 33 | `docs/hiv-treatment/evolution/mk-5-ultimate.md` | `domains/life/hiv-treatment/hiv-treatment.md` | FOUND_INTEGRATED |
| 34 | `docs/mens-intimate-cleanser/breakthrough.md` | `domains/life/mens-intimate-cleanser/mens-intimate-cleanser.md` | FOUND_INTEGRATED |
| 35 | `docs/womens-intimate-cleanser/breakthrough.md` | `domains/life/womens-intimate-cleanser/womens-intimate-cleanser.md` | FOUND_INTEGRATED |
| 36 | `docs/dolphin/hypotheses.md` | `domains/life/dolphin/dolphin.md` | FOUND_INTEGRATED |
| 37 | `docs/coffee-science/hypotheses.md` | `domains/life/coffee-science/coffee-science.md` | FOUND_INTEGRATED |
| 38 | `docs/perfumery/hypotheses.md` | `domains/life/perfumery/perfumery.md` | FOUND_INTEGRATED |
| 39 | `docs/ceramics/hypotheses.md` | `domains/materials/ceramics/ceramics.md` | FOUND_INTEGRATED |
| 40 | `docs/material-synthesis/breakthrough-theorems.md` | `domains/materials/material-synthesis/material-synthesis.md` | FOUND_INTEGRATED |
| 41 | `docs/material-synthesis/hypotheses.md` | `domains/materials/material-synthesis/material-synthesis.md` | FOUND_INTEGRATED |
| 42 | `docs/material-synthesis/industrial-validation.md` | `domains/materials/material-synthesis/material-synthesis.md` | FOUND_INTEGRATED |
| 43 | `docs/material-synthesis/experimental-verification.md` | `domains/materials/material-synthesis/material-synthesis.md` | FOUND_INTEGRATED |
| 44 | `docs/material-synthesis/physical-limit-proof.md` | `domains/materials/material-synthesis/material-synthesis.md` | FOUND_INTEGRATED |
| 45 | `docs/robotics/full-verification-matrix.md` | `domains/infra/robotics/robotics.md` | FOUND_INTEGRATED |
| 46 | `docs/safety/hypotheses.md` | `domains/infra/safety/safety.md` | FOUND_INTEGRATED |
| 47 | `docs/software-design/full-verification-matrix.md` | `domains/compute/software-design/software-design.md` | FOUND_INTEGRATED |
| 48 | `docs/virology/evolution/mk-1-current.md` | `domains/life/virology/virology.md` | FOUND_INTEGRATED |

> Note 1: The `FOUND_INTEGRATED` classification is assigned only when the `### Source: \`<original nested path>\`` or `### Source: \`<basename>\`` header is grep-confirmed inside the consolidation. Accidental basename matches (e.g., sf-ufo/hypotheses files) are excluded.
>
> Note 2: The 2 `FOUND_ALT (DIR)` items are directory entries ending in `docs/<dom>/` or `docs/<dom>/evolution/`; they refer to the directory itself rather than the consolidation. Updating the link to `domains/infra/environmental-protection` resolves them.

## 2. Per-category statistics

| Category | Count | Share | Meaning |
|---|---:|---:|---|
| FOUND_INTEGRATED | 46 | 95.8% | absorbed into the consolidation; only links need updating |
| FOUND_ALT (DIR) | 2 | 4.2% | directory entry; update to `domains/...` |
| MISSING | 0 | 0.0% | ghost files needing authoring (none) |
| FOUND_AT_ORIG | 0 | 0.0% | original `docs/` actually exists (none) |
| **Total** | **48** | **100.0%** | |

Conclusion: actual authoring required is **0**. All 48 are "products.json link-drift" problems.

## 3. Recommended migration-mapping table (all 48, to apply after user approval)

This task is investigation only; no actual edits/migrations were performed. Below is the recommended mapping for updating products.json links.

| products.json link (old) | Recommended target (new) | Method |
|---|---|---|
| `docs/ai-efficiency/techniques-complete.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | consolidation link update |
| `docs/ai-efficiency/full-verification-matrix.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | consolidation link update |
| `docs/ai-efficiency/next-model-blowup-2026-04.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | consolidation link update |
| `docs/ai-efficiency/bt-391-code-generation.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | consolidation link update |
| `docs/ai-efficiency/bt-397-n6-novel-architectures.md` | `domains/compute/ai-efficiency/ai-efficiency.md` | consolidation link update |
| `docs/audio/full-verification-matrix.md` | `domains/culture/audio/audio.md` | consolidation link update |
| `docs/audio/hexa-ear-ultimate.md` | `domains/culture/audio/audio.md` | consolidation link update |
| `docs/audio/hexa-bone-ultimate.md` | `domains/culture/audio/audio.md` | consolidation link update |
| `docs/audio/hexa-ear-cell.md` | `domains/culture/audio/audio.md` | consolidation link update |
| `docs/audio/hexa-speaker-ultimate.md` | `domains/culture/audio/audio.md` | consolidation link update |
| `docs/chip-architecture/ultimate-consciousness-soc.md` | `domains/compute/chip-architecture/chip-architecture.md` | consolidation link update |
| `docs/chip-architecture/hexa-topological-performance-chip.md` | `domains/compute/chip-architecture/chip-architecture.md` | consolidation link update |
| `docs/chip-architecture/hexa-asic-skywater.md` | `domains/compute/chip-architecture/chip-architecture.md` | consolidation link update |
| `docs/chip-architecture/full-verification-matrix.md` | `domains/compute/chip-architecture/chip-architecture.md` | consolidation link update |
| `docs/horology/hypotheses.md` | `domains/culture/horology/horology.md` | consolidation link update |
| `docs/display/full-verification-matrix.md` | `domains/compute/display/display.md` | consolidation link update |
| `docs/battery-architecture/hexa-auto-battery.md` | `domains/energy/battery-architecture/battery-architecture.md` | consolidation link update |
| `docs/environmental-protection/` | `domains/infra/environmental-protection/` | directory link update |
| `docs/environmental-protection/microplastics-solution.md` | `domains/infra/environmental-protection/environmental-protection.md` | consolidation link update |
| `docs/environmental-protection/evolution/` | `domains/infra/environmental-protection/` | directory link update (no evolution subdir) |
| `docs/environmental-protection/testable-predictions-2030.md` | `domains/infra/environmental-protection/environmental-protection.md` | consolidation link update |
| `docs/mycology/hypotheses.md` | `domains/life/mycology/mycology.md` | consolidation link update |
| `docs/mining/hypotheses.md` | `domains/infra/mining/mining.md` | consolidation link update |
| `docs/veterinary/hypotheses.md` | `domains/life/veterinary/veterinary.md` | consolidation link update |
| `docs/horticulture/hypotheses.md` | `domains/life/horticulture/horticulture.md` | consolidation link update |
| `docs/fusion/evolution/mk-1-first-light.md` | `domains/energy/fusion/fusion.md` | consolidation link update |
| `docs/fusion/alien-level-discoveries.md` | `domains/energy/fusion/fusion.md` | consolidation link update |
| `docs/fusion/physical-limit-proof.md` | `domains/energy/fusion/fusion.md` | consolidation link update |
| `docs/hiv-treatment/evolution/mk-1-basic.md` | `domains/life/hiv-treatment/hiv-treatment.md` | consolidation link update |
| `docs/hiv-treatment/evolution/mk-2-short.md` | `domains/life/hiv-treatment/hiv-treatment.md` | consolidation link update |
| `docs/hiv-treatment/evolution/mk-3-mid.md` | `domains/life/hiv-treatment/hiv-treatment.md` | consolidation link update |
| `docs/hiv-treatment/evolution/mk-4-long.md` | `domains/life/hiv-treatment/hiv-treatment.md` | consolidation link update |
| `docs/hiv-treatment/evolution/mk-5-ultimate.md` | `domains/life/hiv-treatment/hiv-treatment.md` | consolidation link update |
| `docs/mens-intimate-cleanser/breakthrough.md` | `domains/life/mens-intimate-cleanser/mens-intimate-cleanser.md` | consolidation link update |
| `docs/womens-intimate-cleanser/breakthrough.md` | `domains/life/womens-intimate-cleanser/womens-intimate-cleanser.md` | consolidation link update |
| `docs/dolphin/hypotheses.md` | `domains/life/dolphin/dolphin.md` | consolidation link update |
| `docs/coffee-science/hypotheses.md` | `domains/life/coffee-science/coffee-science.md` | consolidation link update |
| `docs/perfumery/hypotheses.md` | `domains/life/perfumery/perfumery.md` | consolidation link update |
| `docs/ceramics/hypotheses.md` | `domains/materials/ceramics/ceramics.md` | consolidation link update |
| `docs/material-synthesis/breakthrough-theorems.md` | `domains/materials/material-synthesis/material-synthesis.md` | consolidation link update |
| `docs/material-synthesis/hypotheses.md` | `domains/materials/material-synthesis/material-synthesis.md` | consolidation link update |
| `docs/material-synthesis/industrial-validation.md` | `domains/materials/material-synthesis/material-synthesis.md` | consolidation link update |
| `docs/material-synthesis/experimental-verification.md` | `domains/materials/material-synthesis/material-synthesis.md` | consolidation link update |
| `docs/material-synthesis/physical-limit-proof.md` | `domains/materials/material-synthesis/material-synthesis.md` | consolidation link update |
| `docs/robotics/full-verification-matrix.md` | `domains/infra/robotics/robotics.md` | consolidation link update |
| `docs/safety/hypotheses.md` | `domains/infra/safety/safety.md` | consolidation link update |
| `docs/software-design/full-verification-matrix.md` | `domains/compute/software-design/software-design.md` | consolidation link update |
| `docs/virology/evolution/mk-1-current.md` | `domains/life/virology/virology.md` | consolidation link update |

## 4. Ghost files list (authoring required)

Actual authoring required: **0**.

All 48 are already absorbed as `### Source: \`<path>\`` sections inside `domains/<axis>/<dom>/<dom>.md` consolidations. Verification:

```sh
# Sample 1: ai-efficiency consolidation-absorption check
grep "### Source:" domains/compute/ai-efficiency/ai-efficiency.md | wc -l  # 47
grep -F "Source: \`techniques-complete.md\`" domains/compute/ai-efficiency/ai-efficiency.md
# -> ### Source: `techniques-complete.md`

# Sample 2: hiv-treatment nested path absorption check
grep -F "Source: \`evolution/mk-1-basic.md\`" domains/life/hiv-treatment/hiv-treatment.md
# -> ### Source: `evolution/mk-1-basic.md`
```

Therefore, this investigation concludes no additional authoring is necessary; updating the products.json link table alone resolves the 48 MISS.

## 5. Category-distribution ASCII bar chart

```
Category              Count                  [     Bar (scale: 1 cell = 2 items)     ]
-----------------------------------------------------------------------------------
FOUND_INTEGRATED       46 (95.8%)            [#######################]
FOUND_ALT (DIR)         2 ( 4.2%)            [#]
MISSING                 0 ( 0.0%)            []
FOUND_AT_ORIG           0 ( 0.0%)            []
-----------------------------------------------------------------------------------
Total                  48                    [########################]
```

### 5.1 Domain-cluster distribution (Top 10)

```
Domain                    Count  [ Bar (1 cell = 1 item) ]
-----------------------------------------------------
material-synthesis           5   [#####]
hiv-treatment                5   [#####]
audio                        5   [#####]
ai-efficiency                5   [#####]
environmental-protection     4   [####]
chip-architecture            4   [####]
fusion                       3   [###]
virology                     1   [#]
veterinary                   1   [#]
software-design              1   [#]
-----------------------------------------------------
(14 more domains 1 each)    14   [##############]
```

Largest cluster: material-synthesis / hiv-treatment / audio / ai-efficiency four-way tie for first (5 each).

### 5.2 9-axis distribution

```
Axis        Count  [ Bar (1 cell = 1 item) ]
-------------------------------------------
life          14   [##############]
compute       11   [###########]
infra          7   [#######]
culture        6   [######]
materials      6   [######]
energy         4   [####]
physics        0   []
space          0   []
cognitive      0   []
-------------------------------------------
Total         48
```

life axis leads with 14 (hiv-treatment 5 + evolution-nested + virology + 4 life-domain hypotheses + 2 intimate-cleanser); compute is 2nd with 11 (ai-efficiency 5 + chip-architecture 4 + display 1 + software-design 1).

## Appendix A. Investigation script (reproducible)

```sh
# 1. Extract MISS 48 items
grep "^| MISS |" reports/audits/products-link-remap-2026-04-11.md \
  | grep -v "docs/paper/" | grep -v "calc/" | grep -v "174" \
  > /tmp/miss_48_clean.txt
sed -E 's/.*`docs\/([^`]+)`.*/\1/' /tmp/miss_48_clean.txt > /tmp/miss_48_paths.txt
# -> 48 lines

# 2. For each path, grep the domain consolidation
while IFS= read -r rel; do
  dom=$(echo "$rel" | cut -d/ -f1)
  base=$(basename "$rel")
  nested=$(echo "$rel" | sed "s|^$dom/||")
  dom_dir=$(find domains -type d -name "$dom" 2>/dev/null | head -1)
  integrated="$dom_dir/$dom.md"
  if [ -f "$integrated" ]; then
    hit=$(grep -F "Source: \`$nested\`" "$integrated" 2>/dev/null ||
          grep -F "Source: \`$base\`" "$integrated" 2>/dev/null)
    echo "$rel | $integrated | $hit"
  fi
done < /tmp/miss_48_paths.txt
```

## Appendix B. Next actions (recommended)

1. **products.json link update**: based on the §3 mapping table, replace 48 links in `$NEXUS/shared/n6/docs/products.json`
2. **Re-audit**: after the update, re-run the `products-link-audit` script -> 174 - 48 = 126 expected residual (paper 116 + calc 10)
3. **Paper/calc handling**: the remaining 126 are out of scope; handle in a separate session
4. **Archive folder check**: optional cross-check against `n6shared/logs/absorbed/` absorption-history logs to add consolidation year/date
