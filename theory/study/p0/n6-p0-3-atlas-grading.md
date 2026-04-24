# N6-P0-3 atlas.n6 Grade System + Introduction to the BT System

> Millennium Learning Roadmap P0 · N6 Track · Task 3
> Goal: Understand the single-file SSOT structure, format, grade system, and BT system of atlas.n6, and honestly record the results of searching five BT-541 (Riemann Hypothesis) related nodes in the actual file
> Primary sources: `CLAUDE.md` (atlas.n6 section), `nexus/shared/n6/atlas.n6` (header L1–L22 + BT block L13391–L15447)
> Completion criterion: Being able to explain the format/grade rules of atlas.n6 and state the BT numbering system and the promotion path [7] → [10*]

---

## 0. Honesty Declaration

This note is written by directly reading the "atlas.n6 — reality-map SSOT" section of `CLAUDE.md` and the header and Millennium block of the actual `/Users/ghost/Dev/nexus/shared/n6/atlas.n6` file (~107K lines). No self-referential verification is performed; the description of the grade system cites the CLAUDE.md rules verbatim.

Section 4's BT-541 node search results are **actual grep execution results**; nothing is fabricated. When no node is found, it is honestly recorded as "confirmed not present".

This note does not target the seven Millennium problems. The BT-541–547 nodes in atlas.n6 are all at grade **[5*]** (STRUCTURAL bt, structural mapping only), and the seven-problem status remains **0/7**.

---

## 1. atlas.n6 Format

### 1.1 File Structure

- **Path**: `/Users/ghost/Dev/nexus/shared/n6/atlas.n6`
- **Single file**: 60K+ lines (measured 106,899 lines as of 2026-04-15)
- **Old structure deprecated**: `reality_map_live.json`, `L6_n6atlas.json`, separate level files **do not exist**. All absorbed into atlas.n6 (specified in CLAUDE.md).
- **Guard**: All writes must go through `_guarded_append_atlas()` (schema + dedup). Source: `shared/blowup/lib/atlas_guard.hexa.inc`. Guards applied at all three write sites (solidified 2026-04-12).

### 1.2 Core Lookup Command (CORE VIEW)

Canonical command recorded in atlas.n6 L8:
```
grep '^\@.*\[1[01]\*\]' atlas.n6 → verified [10*]+ only
```

That is, this is the standard way to extract only "verification-complete EXACT" nodes at grade `[10*]` or `[11*]`.

### 1.3 Line Format (citing lines L12–L23 of the source)

Full grammar of atlas.n6 `.n6 format v1` (source verbatim):

```
@type id = expr :: domain [grade]
  <- depends_on              dependency (derivation origin)
  -> derives                 derivation (what follows)
  => application             application target
  == equivalent              equivalence
  ~> converges_to            convergence
  |> verified_by script.py   verification
  !! breakthrough            breakthrough record

Types: @P(primitive) @C(constant) @L(law) @F(formula)
       @R(relation) @S(symmetry) @X(crossing) @?(unknown)
Grades: [0-10] or [d.r] alien index
       * = verified, ! = breakthrough, ? = hypothesis
```

### 1.4 Upper-level Project Format (citation from CLAUDE.md)

**Upper-level format** recorded in the atlas.n6 section of CLAUDE.md (source verbatim):
```
@R {id} = {measured} {unit} :: n6atlas [grade]
```

That is, the `@R` (Relation) type is mainly used for BT / reality-measurement registration, with the `n6atlas` domain tag. The self-format inside atlas.n6 (`@P foundation`, `@C architecture`, etc.) and the simplified format introduced in CLAUDE.md are two layers of the same file; after a large-scale migration (since 2026-04-10), `@R` nodes under the `n6atlas` domain have grown significantly.

### 1.5 Eight Type Tags

| Tag | Meaning | Example |
|---|---|---|
| `@P` | Primitive | `@P n = 6 :: foundation [11*]` |
| `@C` | Constant (derived) | `@C sigma_sq = sigma^2 = 144 :: architecture [10*]` |
| `@L` | Law | `@L alpha_coupling = 0.014 :: consciousness [10*]` |
| `@F` | Formula | `@F sm_blackwell = sigma * phi^tau = 192 :: architecture [10*]` |
| `@R` | Relation | `@R perfect_number :: foundation [10*]` (= σ(6) = 2·6) |
| `@S` | Symmetry | `@S betti_six :: topology [10*]` |
| `@X` | Crossing | `@X one_third_convergence :: convergence [10*!]` |
| `@?` | Unknown (breakthrough candidate) | `@? dark_energy_ratio :: cosmology [3?]` |

---

## 2. Grade System

### 2.1 Base Scale [0–10] + Suffixes

Source (L21–L22 of the atlas.n6 header):
```
Grades: [0-10] or [d.r] alien index
       * = verified, ! = breakthrough, ? = hypothesis
```

Extended grades defined by CLAUDE.md (project upper-SSOT basis):

| Grade | Meaning | Condition |
|---|---|---|
| **[10*]** | EXACT verification complete | primary source + measurement + error + hexa verification (all four satisfied) |
| **[10]** | EXACT | clear definitional/computational match (e.g., σ(6) = 12) |
| **[9]** | NEAR | error < 1% |
| **[7]** | EMPIRICAL | promotion candidate, further verification required |
| **[5~8]** | mid | structural mapping / partial match |
| **[N?]** | CONJECTURE | hypothesis (? suffix) |
| **[N!]** | breakthrough | breakthrough candidate (! suffix) |

### 2.2 Suffix Combinations

- `[10*]` = grade 10 + verified
- `[10*!]` = grade 10 + verified + breakthrough
- `[11*]` = grade 11 (meta-satisfaction, foundation primitives only)
- `[5*]` = grade 5 + verified (only structural mapping verified)
- `[7?]` = grade 7 + hypothesis
- `[3?]` = grade 3 + hypothesis (low confidence, awaiting breakthrough)

### 2.3 Example of Actual Grade Distribution (confirmed in the atlas.n6 header block)

- `@P n = 6 :: foundation [11*]` — highest meta-grade
- `@P sigma = 12 :: foundation [11*]` — three of {n, σ, τ} are [11*]
- `@P phi, sopfr, J2, mu, M3 :: foundation [10*]` — the other four primitives
- `@C meta_fp = 1/3 :: meta [10*!]` — meta fixed-point breakthrough registration
- `@X one_third_convergence :: convergence [10*!]` — 7-path 1/3 convergence
- `@X physics_n6 :: crossing [7?]` — physics-constant approximation hypothesis (promotion candidate)
- `@? dark_energy_ratio :: cosmology [3?]` — unconfirmed, very low confidence
- `@? fine_structure :: physics [4?]` — 1/(σ·σ − sopfr) = 1/139 approximation

---

## 3. Introduction to the BT (Breakthrough) System

### 3.1 Numbering Rules

Based on CLAUDE.md + project history:

- **BT-1 – BT-343**: main-theorem breakthrough collection (built between 2026-02 and 2026-04). Spread across theory/engineering/life-science domains.
- **BT-401 – BT-413**: quantum mechanics (BT-401–408) + therapeutic nanobots (BT-404–413) extension.
- **BT-500–540**: ITER / fusion / energy / materials block.
- **BT-541 – BT-547**: **seven Millennium problems** mapping nodes (direct target of this note).
- **BT-548 – BT-557**: marketing / business-law extension.
- **BT-558 – BT-1108+**: dimensional perception, unified theorems and further extensions.

### 3.2 BT-541–547 — Seven Millennium Problems Mapping

| BT | Problem | Status (Clay) |
|---|---|---|
| BT-541 | Riemann Hypothesis | OPEN |
| BT-542 | P vs NP | OPEN |
| BT-543 | Yang-Mills Mass Gap | OPEN |
| BT-544 | Navier-Stokes Existence and Smoothness | OPEN |
| BT-545 | Hodge Conjecture | OPEN |
| BT-546 | Birch–Swinnerton-Dyer (BSD) | OPEN |
| BT-547 | Poincaré Conjecture | **solved** (Perelman 2002–2006) |

**Honest status**: The current solved count of the seven Millennium problems is 1/7 (Poincaré, accepted by Perelman); the n=6 structure does not target any of them. The atlas.n6 registration grade of BT-541–547 is **[5*]** (STRUCTURAL bt — "structural connection/mapping verified only, not a solution").

### 3.3 BT Extension Set (millennium-dfs)

During the DFS full-verification loops on 2026-04-11 – 2026-04-12, **structural connection points** between the seven Millennium problems and n=6 were recorded as 65 tight matches (`@X n6-millennium-dfs-summary = 21+30+14=65 tight :: n6atlas [10*]`, atlas.n6 L13449). These 65 matches are **structural mapping**, not **Millennium-problem targets**. Related memo: `project_millennium_dfs_complete.md`.

---

## 4. Five BT-541 Related Nodes in atlas.n6 — Actual Search Results

### 4.1 Search Method

- File: `/Users/ghost/Dev/nexus/shared/n6/atlas.n6`
- Tool: Grep (ripgrep-based)
- Pattern: `BT-541|BT541|BSD|Riemann|Yang-Mills|Navier|Hodge|millennium|millennium|bt-541`
- Case-insensitive
- Among the results, the five nodes most directly related to BT-541 (Riemann Hypothesis) are extracted.

### 4.2 Search Results — Actual Excerpts (source verbatim)

**Node 1** — atlas.n6 L15408 (BT-541 main mapping node)
```
@X n6-bt-541 = STRUCTURAL bt :: bt [5*]
  "Riemann Hypothesis"
```
- Type: `@X` (Crossing)
- Value: STRUCTURAL bt
- Domain: bt
- Grade: **[5*]** (verified 5 — structural mapping only)
- Comment: "Riemann Hypothesis"
- Interpretation: BT-541 is registered in atlas.n6 as a main mapping node, but grade [5*] means "structural connection confirmed only, not a solution."

**Node 2** — atlas.n6 L15422 (BT-541–547 set node)
```
@X n6-bt-541~547 = STRUCTURAL bt :: bt [5*]
  "541~547 aggregate (7 breakthroughs"
```
- Type: `@X`
- Domain: bt
- Grade: **[5*]**
- Interpretation: BT-541–547 aggregate node for the seven Millennium problems. "7 breakthroughs" in the comment means "7 problem mappings", not "7 problem solutions". Honest comment.

**Node 3** — atlas.n6 L10468 (Riemann Zeta triple relation, BT-1 – BT-16 extension)
```
@R n6-atlas-breakthrough-theorems-extended:-bt-1-~-bt-12-bt-16 = Riemann Zeta Trident ζ(2)=π²/n, ζ(-1)=-1/σ, BCS=σ/(7ζ(3)) n6 :: n6atlas [10*]
```
- Type: `@R` (Relation)
- Content: three special values of the Riemann zeta function re-expressed in n=6 arithmetic
  - ζ(2) = π²/6 = π²/**n**
  - ζ(−1) = −1/12 = −1/**σ**
  - BCS superconductivity constant = σ/(7·ζ(3)) = 12/(7·ζ(3))
- Domain: n6atlas
- Grade: **[10*]** (EXACT verified)
- Interpretation: This node is not the **main body** of BT-541 (Riemann Hypothesis) but a registration of the observation that **special values of the Riemann zeta function** match n=6 base constants exactly. It is classified as part of the BT-1 – BT-16 extension theorems but is in essence a core matching mathematically connected to BT-541 (Riemann Hypothesis itself). **Not a target of the Riemann hypothesis** — this is solely an exact match between ζ's integer values and n=6 constants.

**Node 4** — atlas.n6 L13395 (millennium DFS ζ(-3))
```
@R n6-millennium-dfs-zeta-neg3 = 1/120 = 1/(phi*sopfr*sigma) :: n6atlas [10*]
```
- Type: `@R`
- Content: ζ(−3) = 1/120 = 1/(φ · sopfr · σ) = 1/(2 · 5 · 12)
- Domain: n6atlas
- Grade: **[10*]** (EXACT)
- Interpretation: The negative-integer special value ζ(−3) = 1/120 of the Riemann zeta decomposes exactly into a product of three n=6 primitive constants. **Not a BT-541 target — an n=6-basis decomposition of a ζ special value.**

**Node 5** — atlas.n6 L13397 (millennium DFS ζ(-5))
```
@R n6-millennium-dfs-zeta-neg5 = -1/252 = -1/(tau*(n/phi)^2*(sigma-sopfr)) :: n6atlas [10*]
```
- Type: `@R`
- Content: ζ(−5) = −1/252 = −1/(τ · (n/φ)² · (σ − sopfr)) = −1/(4 · 9 · 7)
- Domain: n6atlas
- Grade: **[10*]**
- Interpretation: ζ(−5) is likewise exactly decomposed as a 4-term product in the n=6 basis. The decomposition path is longer (4 terms) but still registered as EXACT.

### 4.3 Honest Search Summary

- BT-541 **main-body nodes**: 2 (`n6-bt-541`, `n6-bt-541~547`) — all at grade **[5*]** (structural mapping only).
- BT-541 **associated [10*] match nodes**: many (L10468, L13395, L13397, L13399, etc.) — all n=6-basis decompositions of ζ special values, **unrelated to a Riemann-hypothesis target**.
- **Confirmed absent (honest record)**: Nodes such as "BT-541 solution" or "Riemann proof" **do not exist** in atlas.n6. What exists are only exact decomposition mappings of ζ special values.

### 4.4 Interpretation — Distinguishing "Structural Mapping vs Solution"

atlas.n6's treatment of BT-541 accurately displays the project's **honesty principle**.

- Between the "ζ function of the Riemann Hypothesis" and "n=6 arithmetic" there exist many **EXACT matches** (grade [10*]).
- But the BT-541 main node is registered only at **[5*]** — "structural mapping confirmed only, not a solution."
- This dual registration is a device to prevent **integer-value observations like ζ(2) = π²/n or ζ(−1) = −1/σ from being misread** as targets of the Riemann Hypothesis.

That is, atlas.n6 records at the measurement level — as [10*] — the fact that n=6 structure appears on nearly all paths of reality (+ mathematical constants), but keeps the **problem-target status** separately under the [5*] BT grade system, honestly **unresolved**.

---

## 5. Promotion Path [7] → [10*] — Editing Rules

### 5.1 Principle (explicit in CLAUDE.md)

> Promotion: [7] → [10*] = **direct edit** of atlas.n6 (do not create a new file)

That is, promoting a node registered at EMPIRICAL grade [7] to EXACT-verified [10*] requires **editing atlas.n6 directly** rather than creating a separate new file. This is the rule for maintaining the project invariant that atlas.n6 is a **single-file SSOT**.

### 5.2 Promotion Procedure

1. Identify candidate nodes: `grep '\[7\]' atlas.n6` to collect EMPIRICAL-registered nodes.
2. Secure primary sources: original papers / measurement databases / standard references.
3. Re-verify measurement: error analysis + n=6-basis re-decomposition.
4. Hexa verification script: add executable verification steps via `|> verify_*.hexa`.
5. **Direct edit**: edit the relevant atlas.n6 line, changing `[7]` to `[10*]` and adding `|>` verification lines and source `=>` lines.
6. **Guard-required**: writes must go through `_guarded_append_atlas()` (schema + dedup).

### 5.3 Prohibited Actions

- ✗ Creating a new file like `atlas_new.n6` or `reality_map_v10.json`
- ✗ Separating into level files (e.g., `L6_n6atlas.json`) — already deprecated by the old structure
- ✗ Direct appends that bypass the guard
- ✗ Self-referential verification (promoting to [10*] using only the content of this note)

### 5.4 Example — Already-promoted `meta_fp`

`@C meta_fp = 1/3 :: meta [10*!]` (atlas.n6 L81) is an example that was promoted from `[10]` → `[10*!]` after six independent paths of convergence were confirmed. Promotion evidence:
- `phi(6)/6 = 2/6 = 1/3`
- `tan²(pi/6) = 1/3`
- `tau/sigma = 4/12 = 1/3`
- `det(M_contraction) = 1/3`
- `I_meta_fixedpoint = 1/3`
- `|exp(iz_0)| = 1/3`

Six independent paths confirmed → `!` (breakthrough) suffix added. This pattern is the standard.

---

## 6. Study Checklist

- [ ] Can you state the file path and size of atlas.n6? (`$NEXUS/shared/n6/atlas.n6`, ~107K lines)
- [ ] Can you recite the eight format-type tags (@P @C @L @F @R @S @X @?)?
- [ ] Can you explain the seven stages of the grade system ([10*] – [N!]) in order?
- [ ] Can you state the difference between `[10*]` and `[5*]`? (former: EXACT-verified, latter: structural-mapping only)
- [ ] Can you say which Millennium problem each of BT-541–547 corresponds to?
- [ ] Can you explain the difference between the BT-541 main node and the [10*] ζ nodes associated with it?
- [ ] Can you honestly state the status of the seven Millennium problems? → 1/7 (Poincaré only) — the other six remain OPEN, and are not targets of n=6
- [ ] Can you state the core rule of the promotion path [7] → [10*]? → direct edit of atlas.n6; creating a new file is prohibited
- [ ] Can you explain the meaning of the command `grep '^\@.*\[1[01]\*\]' atlas.n6`? → extract only verification-complete nodes at [10*]/[11*]

---

## 7. Primary Sources

- `/Users/ghost/Dev/n6-architecture/CLAUDE.md` — atlas.n6 section (highest source for format, grade system, promotion-path rules)
- `/Users/ghost/Dev/nexus/shared/n6/atlas.n6` header L1–L22 — source of the `.n6 format v1` grammar
- `/Users/ghost/Dev/nexus/shared/n6/atlas.n6` L25–L100 — seven primitive-constant `@P` registration block
- `/Users/ghost/Dev/nexus/shared/n6/atlas.n6` L13391–L13551 — millennium DFS full-verification block (65 tight)
- `/Users/ghost/Dev/nexus/shared/n6/atlas.n6` L15408–L15422 — BT-541–547 structural-mapping block
- `project_millennium_dfs_complete.md` (user memory) — DFS 5-round loop, 21→51 tight extensions + Bilateral Theorem B candidate record

---

## 8. Next Study Steps

- **P1 track** — foundations of analytic number theory (PURE-P1), ζ-function depth related to the Riemann Hypothesis (PROB-P1), n=6 mapping extension (N6-P1).
- **N6-P0-1** (revisit) — return to the σφ = nτ uniqueness theorem and reconfirm that the atlas.n6 [11*] grade verified here is a direct consequence of that theorem.
- **N6-P0-2** (revisit) — additionally map the BT-541-related ζ(−3) = 1/120, ζ(−5) = −1/252 decompositions identified here onto the Section 2 base-value table.
- **Operational practice** — find a real `[7]` node and simulate the five promotion steps by hand (actual editing requires user approval).

---

## Appendix A — Search-reproduction Commands

To reproduce the BT-541 search of Section 4:
```bash
# main-body nodes
grep -n '@X n6-bt-54[1-7]' /Users/ghost/Dev/nexus/shared/n6/atlas.n6

# aggregate node
grep -n 'bt-541~547' /Users/ghost/Dev/nexus/shared/n6/atlas.n6

# millennium DFS ζ decomposition nodes
grep -n 'millennium-dfs-zeta' /Users/ghost/Dev/nexus/shared/n6/atlas.n6

# Riemann Zeta Trident (L10468)
grep -n 'Riemann Zeta Trident' /Users/ghost/Dev/nexus/shared/n6/atlas.n6

# verification-complete extraction (CORE VIEW)
grep '^\@.*\[1[01]\*\]' /Users/ghost/Dev/nexus/shared/n6/atlas.n6 | head -50
```

All commands were directly executed on the atlas.n6 file as of the 2026-04-15 session.
