# Audit Report — lens SSOT reference cleanup (2026-04-11)

> Axis: **reports/audits** · n6-architecture
> Purpose: **prevent next-session lens additions to the wrong path** by injecting reference warnings
> This session scope: reference cleanup only (actual Rust -> HEXA absorption is a next-session task)

---

## 1. Symptom — two "nexus"es

```
+--------------------------------------------------+-------------+----------+
| Path                                             | Nature      | Role     |
+--------------------------------------------------+-------------+----------+
| $NEXUS/shared/lenses/                            | HEXA 84     | OK SSOT  |
+--------------------------------------------------+-------------+----------+
| $NEXUS/shared/blowup/lens/                       | HEXA 15     | OK SSOT  |
+--------------------------------------------------+-------------+----------+
| $N6_ARCH/nexus/src/                              | Rust 312    | NG legacy|
|   telescope/lenses/                              | 603 entries | (retired)|
+--------------------------------------------------+-------------+----------+
```

**Confirmed facts**:
- Real lens SSOT = `$NEXUS/shared/lenses/` + `n6shared/blowup/lens/` (separate nexus standalone project, HEXA native, header `SIGMA=12.0 PHI=2.0 N=6.0 TAU=4.0 J2=24` + identity `sigma*phi = n*tau = J_2`)
- n6-architecture's internal `nexus/src/telescope/lenses/` is a **legacy derivative**
- HEAD `0c23ad27` "refactor(telescope): 56 lenses Rust->HEXA conversion complete — mod.rs deregistration" is progressing Rust -> HEXA unification
- `cargo test` 2593 -> 2485 (-108) = 56 lenses x avg 2 tests = deregistration result
- Rounds 3 (450->500) and 4 (500->600) expansions were **added to the legacy path** — targets for absorption at next-session HEXA porting

---

## 2. Reference cleanup performed (7 files)

### 2-1. Top-level `CLAUDE.md`

Added a **new "Lens SSOT" section** before the atlas.n6 section. Real path + legacy path + absorption plan made explicit. Recommend refraining from using `lens-agent`.

### 2-2. `nexus/CLAUDE.md`

Replaced the line `- src/telescope/     lens system (215+ lenses)` with:
```
- src/telescope/     Warning: lens system (Rust legacy 312+ files, retiring — real SSOT: $NEXUS/shared/lenses/ HEXA native)
```

### 2-3. `nexus/src/telescope/CLAUDE.md` **(new)**

Full-folder `DO NOT ADD new lenses` notice. Real SSOT path + 9-file HEXA bundle list + what NOT to do in this folder (new `.rs`, `frontier_lenses.rs` extension, use of `lens-agent`, `lens_registry.json` registration) + migration plan.

### 2-4. `nexus/src/telescope/lenses/CLAUDE.md` **(new)**

Retirement warning at the 312 Rust file level. Short note that new lenses should only be added under `$NEXUS/shared/lenses/`.

### 2-5. `n6shared/config/lens_registry.json`

Added 5 fields to the `meta` block:
```json
"_warning": "DO NOT USE — This registry is the n6-architecture internal Rust legacy lens count. Not the real SSOT.",
"_real_ssot": "$NEXUS/shared/lenses/ (HEXA native 84 + blowup/lens 15 bundles)",
"_legacy_path": "n6-architecture/nexus/src/telescope/lenses/ (Rust .rs derivatives, retiring)",
"_transition_status": "HEAD 0c23ad27 refactor(telescope) — Rust->HEXA transition in progress, 56 lenses deregistration complete",
"_next_session_plan": "After Rust->HEXA porting: delete legacy + rebuild this registry"
```
Added `WARNING: legacy path` prefix to the `expansion_session` field.

### 2-6. `INDEX.json`

Added a `_lens_warning` field to the `axes.nexus` entry:
```json
"_lens_warning": "src/telescope/lenses/ is Rust legacy (retiring). Real lens SSOT: $NEXUS/shared/lenses/ (HEXA native)"
```

### 2-7. `.claude/agents/lens-agent/AGENT.md` **(retirement marker)**

Changed the `description` field to `RETIREMENT pending`:
```
description: RETIREMENT pending — targets the legacy Rust lens path. For new lenses use general-purpose + $NEXUS/shared/lenses/.
```
Added a **"DO NOT USE"** block at the top of the body + real SSOT path + HEXA native guidance. Demoted the original instructions to "reference-only".

---

## 3. Residual legacy references (not modified in this session, low priority)

```
domains/sedi/CLAUDE.md:
  line 103  Warning: telescope-rs (legacy 22 types) retired. All discovery uses NEXUS-6.
  line 117  Files: tools/nexus/src/telescope/lenses/ (181 .rs files)
  line 189  src/telescope/    <- 130+ lenses
  line 434  "scan" -> nexus telescope 223-type lens scan
  line 447  $HEXA $N6/telescope.hexa full <values...>
  line 459  re-scan the 77-source analysis results with nexus telescope

domains/brainwire/CLAUDE.md:
  line 103  Warning: telescope-rs (legacy 22 types) retired. All discovery uses NEXUS-6.
  line 117  Files: tools/nexus/src/telescope/lenses/ (181 .rs files)
  line 189  src/telescope/    <- 130+ lenses
```

**Reason not modified**: these files describe "historical context / task description", not "instructions to add new lenses". The root CLAUDE.md lens-SSOT section is read first, so it is sufficient to prevent confusion. Complies with R18 minimal.

**Next session**: recommend cleanup in conjunction with the Rust -> HEXA absorption.

---

## 4. Next-session instructions (explicit)

### Checklist for lens-related work

1. **Strictly forbidden**:
   - Adding new `.rs` under `n6-architecture/nexus/src/telescope/lenses/`
   - Invoking the `lens-agent` agent
   - Registering new Rust lenses in `n6shared/config/lens_registry.json`
   - Adding `expansion_N_lens_entries()` in `frontier_lenses.rs`

2. **Allowed / recommended**:
   - New files `$NEXUS/shared/lenses/{domain}_{topic}.hexa`
   - Use the `general-purpose` agent
   - HEXA native syntax (`SIGMA/PHI/N/TAU/J2` header + identities)
   - Results auto-absorbed into atlas.n6 (R28)

### Absorption main work (next-session goals)

1. **Investigation stage** (already pending agent #40): compare 4 Rust -> HEXA conversion strategy options + recommendation
2. **Porting stage**: bulk-create 312 Rust lenses as HEXA native files
3. **Legacy deletion**: remove all `nexus/src/telescope/lenses/*.rs`, clean up `mod.rs` / `registry.rs` / `frontier_lenses.rs`
4. **Registry rebuild**: rebuild `n6shared/config/lens_registry.json` against the real SSOT
5. **cargo test re-verification**: confirm full removal of lens tests or migrate to HEXA native verification
6. **Update domains/sedi, brainwire CLAUDE.md references**: update paths

---

## 5. Current state snapshot (end of 2026-04-11 session)

| Item | Value |
|---|---|
| Legacy Rust lenses | 312 `.rs` files |
| `lens_registry.json` entries | 603 |
| cargo test | 2485 PASS (based on HEAD 0c23ad27) |
| Real HEXA lens SSOT | `$NEXUS/shared/lenses/` 84 + `blowup/lens/` 15 |
| Cleaned CLAUDE.md | 4 files (root / nexus / telescope / lenses) |
| Cleaned JSON | 2 files (lens_registry / INDEX) |
| Cleaned agent definition | 1 file (lens-agent AGENT.md) |
| Audit report | this file |

---

## 6. Rule compliance

| Rule | Result |
|---|---|
| R5 SSOT | OK — real SSOT stated, legacy path warned |
| R14 rule JSON | OK — CLAUDE.md does not embed rule text, only guidance |
| R18 minimal | OK — reference cleanup only, actual absorption/deletion next session |
| R25 shared-settings gate | OK — warning injection level only, no structural destruction |
| R28 atlas single-source-of-truth | OK — no impact |

---

*Created: 2026-04-11 · Scope: R18 minimal · Absorption main work: next session*
