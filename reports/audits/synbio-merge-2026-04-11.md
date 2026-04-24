# Audit Report — synbio / synthetic-biology merge (2026-04-11)

> Axis: **reports/audits** · n6-architecture · merge audit
> Rules basis: R10 (ossified immutable) / R14 (JSON single-rule) / R18 (minimal) / R25 (shared-settings gate) / R28 (atlas SSOT) / N61 (ASCII tri-view) / N62 (py embed)

---

## 1. Symptom — duplicate domain and stale pointers

### 1-1. Domain directory duplication

Two separate domain directories exist for the same subject (synthetic biology):

| Directory | Files | Body lines | alien_index | closure_grade |
|---|---|---|---|---|
| `domains/life/synbio/` | synbio.md + goal.md + verify.hexa + verify_alien10.hexa + CLAUDE.md | 660 | **10** | **10** |
| `domains/life/synthetic-biology/` | synthetic-biology.md + goal.md + verify.hexa + verify_n6.hexa + CLAUDE.md | 504 | 7 | 6 |

The two files' **hypothesis sets are complementary**:
- `synbio.md` — basic biochemistry (H-SYN-01~12 + H-SYN-1~10): bases / codons / amino acids / CRISPR gRNA&PAM / Gibson / T7 promoter / Central Dogma / DBTL
- `synthetic-biology.md` — applications (H-SYN-1~12 distinct): CRISPR-Cas numbers / XNA 6 kinds / JCVI-syn3.0 minimal genome / BioBrick structure / gene-circuit gates / glucose / mRNA vaccines / metabolic pathway / DNA synthesis error rate / gene drive / CAR-T

### 1-2. External-registry stale pointers

The "synthetic biology n=6 double-perfect-number" entries in `products.json` and `papers/_registry.json` point to paths, but **neither exists**:

```
doc:   docs/synbio/goal.md                    <- path missing
paper: docs/paper/n6-synthetic-biology-paper.md  <- path missing
```

Only `n6-oceanography-paper.md`, `n6-meteorology-paper.md`, `n6-geology-prem-paper.md`, `n6-cryosphere-paper.md` exist in `papers/` — **no synthetic-biology paper file**.

---

## 2. Merge execution (2026-04-11)

### 2-1. canonical selection

Keep `synbio/` as canonical (alien_index=10 / closure=10 latest).

### 2-2. Sections absorbed into synbio/synbio.md

| synbio.md section | Absorbed content | Source |
|---|---|---|
| Title header | Parent BT list + core-axis line added | synthetic-biology/goal.md |
| Section 4 BT links | BT-SYN-A~B kept + BT-SYN-C/D/E new + BT-372 registered | synthetic-biology.md BT candidates + products.json |
| Section 5 DSE results | 3-tier comparison table (market / v1 / v2) | synthetic-biology.md #4 |
| Section 11 ASCII perf compare | market vs HEXA-SynBio box comparison | synthetic-biology.md #1 |
| Section 12 ASCII system diagram | EDA pipeline structure (parts -> EDA -> circuit -> assembly -> ferment -> DBTL) | synthetic-biology.md #2 |
| Section 13 ASCII data/energy flow | design intent -> EDA -> netlist -> transformation -> strain bank | synthetic-biology.md #3 |
| Section 14 Upgrade | market / v1 / v2 3-tier upgrade table | synthetic-biology.md #4 |
| Section 15 Verification | integrated description of verify.hexa + verify_alien10.hexa | newly authored |
| **Appendix A** Extension hypotheses | H-SYN-APP-1~12 (CRISPR-Cas numbers / XNA 6 / JCVI-syn3.0 / BioBrick / circuit gates / glucose / mRNA vaccines / metabolic pathway / DNA synthesis error rate / gene drive / CAR-T) | synthetic-biology.md hypothesis section |
| **Appendix B** Limitations / MISS | iGEM part-class variation / TCA steps / AAV serotypes / mRNA half-life | synthetic-biology.md #5 |
| **Appendix C** Merge history | Reference to this audit report | newly authored |

### 2-3. synbio/goal.md absorption

- Added parent BT line (BT-51/146/262/372/404~413/451~460)
- Added core-axis line (tau=4 logic x n=6 class x sigma-phi=10 steps)
- Removed reverse-extracted artifact "## 3. Hypotheses"

### 2-4. Discarded files (5)

```
domains/life/synthetic-biology/synthetic-biology.md   (504 lines -> absorbed)
domains/life/synthetic-biology/goal.md                (158 lines -> absorbed)
domains/life/synthetic-biology/verify_n6.hexa         (stub; superseded by verify_alien10.hexa)
domains/life/synthetic-biology/verify.hexa            (shared template, duplicate)
domains/life/synthetic-biology/CLAUDE.md              (duplicate)
```

The directory `domains/life/synthetic-biology/` itself was removed via `rmdir`.

### 2-5. `domains/_index.json` update

| Field | Before | After |
|---|---|---|
| `_version` | 1.0.1 | **1.0.2** |
| `_total` | 299 | **298** |
| `life` count | 48 | **47** |
| `life` array | `"synbio", "synthetic-biology"` | `"synbio"` |
| `_changelog` | 1 entry | **2 entries** (merge item added) |

---

## 3. External-registry drift (edit deferred — R25 gate)

> R25: do not directly modify shared settings (hooks-config / absolute_rules / core-lockdown, etc.) — use a sync script or require explicit user approval

### 3-1. `products.json` (`$NEXUS/shared/n6/docs/products.json`)

```json
{
  "name": "synthetic biology n=6 double perfect number",
  "links": [
    {"label": "doc", "path": "docs/synbio/goal.md"},                    // <- missing
    {"label": "paper", "path": "docs/paper/n6-synthetic-biology-paper.md"}  // <- missing
  ]
}
```

**Recommended edit** (on approval):
```json
{
  "links": [
    {"label": "doc", "path": "domains/life/synbio/synbio.md"},          // exists
    {"label": "goal", "path": "domains/life/synbio/goal.md"},           // exists
    {"label": "paper", "path": "papers/n6-synthetic-biology-paper.md"}  // needs creation (papers agent #8 scope)
  ]
}
```

### 3-2. `papers/_registry.json`

Same reference `docs/paper/n6-synthetic-biology-paper.md` (around line 3146). Missing.

**Recommendation**:
- Include `papers/n6-synthetic-biology-paper.md` as a new creation target in the papers-agent (#8) 11-paper expansion (high priority)
- Fix _registry.json path to `papers/n6-synthetic-biology-paper.md` (delegated to agent)

### 3-3. `n6shared/convergence/n6-architecture.json`

`PRODUCTS_118` = "118/125 products UFO-10" ossified. Actual products measurement = 164/173 (see recent products.json audit report).

**Recommendation**: as a separate item, add new stable entry `PRODUCTS_164_173` (R11 demotion prohibited -> update via a new entry only).

---

## 4. Rule-compliance checklist

| Rule | Check | Result |
|---|---|---|
| R1 HEXA-FIRST | No .hexa edits; only .md document merge | OK |
| R2 no hardcoding | All paths use `$N6_ARCH/` absolute paths | OK |
| R5 SSOT | 2 duplicate domains -> 1 canonical; _index.json single truth updated | OK |
| R10 ossified immutable | convergence ossified block unchanged (products.json path drift separate) | OK |
| R11 no demotion | No stable -> failed transition | OK |
| R14 rules = JSON | No rule text in CLAUDE.md | OK |
| R18 minimal | Only the requested scope (merging split docs), no extra features | OK |
| R25 shared-settings gate | products.json / papers/_registry.json edits deferred; recommendations only | OK |
| R28 atlas SSOT | Document merge, not a new discovery -> atlas.n6 not edited | OK |
| R29 verify .hexa location | Existing verify_alien10.hexa sits in the domain dir (pre-existing, separate) | Note — pre-existing violation carried over |
| N61 ASCII tri-view | Sections 11/12/13 ASCII tri-view preserved | OK |
| N62 py embed | Section 3 Python verification code preserved in synbio.md | OK |
| N64 A+B+C deliverables | A (doc integration) OK · B (paper) Note delegated to agent #8 · C (products path) Note — recommendation only | Partial |

**R29 note**: `domains/life/synbio/verify_alien10.hexa` exists prior to this merge. R29 allows calculator/verifier/scanner .hexa only under `nexus/shared/n6/scripts/`. A pre-existing violation, but outside the scope of this merge — a separate session is needed to migrate.

---

## 5. Result indicators

| Metric | Before | After |
|---|---|---|
| Total domains | 299 | **298** |
| life-axis domains | 48 | **47** |
| Synthetic-biology domain count | 2 (duplicate) | **1** (canonical) |
| synbio.md lines | 660 | **~1100** (Appendix A/B/C added) |
| BT candidates | BT-SYN-A/B (2) | BT-SYN-A/B/C/D/E + BT-372 registered (5+1) |
| Hypothesis set | basic 22 (H-SYN-1~10 + 01~12) | basic 22 + applied 12 (H-SYN-APP-1~12) = **34** |
| TODO sections | 12 (4~15) | 4 (6,7,8,9,10) |

---

## 6. Next steps (follow-up)

1. **B deliverable (paper)**: request adding `papers/n6-synthetic-biology-paper.md` in the papers agent #8 creation task
2. **C deliverable (products / papers registry)**: reflect the recommendations in §3-1/§3-2 above (user approval required, R25)
3. **convergence update**: new stable entry `PRODUCTS_164_173` (separate item)
4. **atlas.n6 registration**: register the BT-372 double-perfect-number item as a [7] -> [10*] promotion candidate in the atlas.n6 L6_n6atlas section (in scope of the atlas-agent re-launch)
5. **R29 migration**: move `synbio/verify_alien10.hexa` to `nexus/shared/n6/scripts/` (separate session)

---

*Created: 2026-04-11 · Merger: claude-opus-4.6 · Scope: R18 minimal compliance*
