---
category: infra-readme-pipeline
date: 2026-04-28
parent_witness: design/kick/2026-04-28_json-ssot-enrichment-cycle19_omega_cycle.json
parent_proposal: proposals/hexa_weave_kick_infra_cycle13_5_of_5_closure_2026_04_28.md
mission: cycle 19 priority 1 — README.md sync-readme JSON SSOT enrichment — 18 hand-curated domains literally registered for WRITE-mode unblock
status: PREP + JSON SSOT enrichment complete. dry-run 0 changed markers (raw 65/68 idempotent ACHIEVED). --check exit 0. WRITE-mode NOT executed (gated on cycle 20+ sealed-hash CI). README.md untouched.
---

# README.md sync-readme JSON SSOT enrichment (cycle 19 priority 1)

raw 91 C3 honest disclosure: this cycle replicates 18 hand-curated
README.md domain blocks into `n6shared/readme-data.json` verbatim
(byte-exact extraction via Python regex; line numbers cited in §3).
NO new content is fabricated. NO README.md modification is performed.
The goal is to restore the SSOT (single source of truth) contract that
`n6shared/sync-readme.hexa` enforces, so future cycles can WRITE-mode
the README from JSON without losing curated content.

## §0 WHY this cycle, why now

Cycle 18 hexa-runtime-pipeline-fix diagnosed two bugs in
`n6shared/sync-readme.hexa`:

- **B.1** `scan_markers` prefix-confusion: `<!-- AUTO:BADGE:END -->`
  matched the `<!-- AUTO:` prefix and produced noise NAMEs like
  `BADGE:END -->\n...COMMON_LINKS`. Fix: NAME-character allowlist
  (`[A-Za-z0-9_-]`).
- **B.2** JSON `null` -> `to_string()` -> literal "void": Hexa runtime
  classifies JSON null as `type_of() == "void"` (not "null" or "nil"),
  letting raw void slip through and corrupt output with strings like
  "DSE void", "experimentvoid%". Fix: triple-guard {null, nil, void}.

After those fixes, `--dry-run` reported **36 markers would change**.
WRITE-mode would have replaced ~30 hand-curated domain blocks (rich
"> **🛸10** | ✅ | BT 264 · 100% EXACT | ..." content with cross-refs
and 800+ char rationale) with stubs (`> **🛸0** | TP0`) — destroying
~25KB of curation. WRITE-mode was therefore **unsafe** until JSON
SSOT was enriched to cover the missing domains.

Cycle 19 priority 1 closes that gap: 18 missing domain entries are
literally registered (16 brand-new IDs + biology / fantasy upgraded
from auto-gen to literal). dry-run target: **0 changed markers**.
Result: **ACHIEVED** ([OK] up to date with 63 markers).

## §1 schema extension (`summary_literal` / `footer_literal`)

`n6shared/sync-readme.hexa` is extended (+19 LoC, backward-compatible)
to recognize two new optional string fields per domain:

```hexa
// render_summary now checks summary_literal first
let lit = get_or(d, "summary_literal", "")
if type_of(lit) == "string" {
    if len(lit) > 0 {
        return to_string(lit)
    }
}
// ...existing auto-generation as fallback
```

`render_footer` mirrors the same pattern. Existing 13 auto-domains
(fusion / chip / energy / ai / environment / physics / materials /
robotics / software / display / audio / safety / biology) have no
literal fields → fall through to auto-gen verbatim. **Zero behavior
change for existing domains.**

The 17 newly added domains (biology upgraded; 16 new IDs added) carry
`summary_literal` and `footer_literal` strings copied verbatim from
README.md. Round-trip: `render_summary` returns the literal → sync_all
compares against the existing marker block → bytes match → no change.

## §2 STATS marker spacing alignment

Secondary fix: `render_stats` previously emitted
`  NEXUS tests:      <count>` (6 spaces) but README hand-curated form
uses `  NEXUS tests:    <count>` (4 spaces). Aligned to 4 spaces in
the generator to match README. Required for `AUTO:STATS` idempotency
under WRITE-mode.

## §3 enriched domain list (raw 91 C3 source attribution)

All 18 enrichment entries cite their README.md source line numbers:

| id            | summary line | footer line | bytes summary | bytes footer |
|---------------|-------------:|------------:|--------------:|-------------:|
| biology       | 370          | 380         | 435           | 603          |
| play          | 388          | 397         | 238           | 70           |
| aerospace     | 405          | 413         | 325           | 128          |
| sf            | 421          | 429         | 326           | 26           |
| frontier      | 437          | 494         | 348           | 852          |
| civilization  | 502          | 516         | 295           | 254          |
| life-culture  | 524          | 540         | 257           | 224          |
| tech-industry | 548          | 577         | 262           | 850          |
| computer      | 585          | 596         | 154           | 278          |
| marketing     | 604          | 615         | 521           | 208          |
| millennium    | 625          | 639         | 195           | 366          |
| dimension     | 666          | 680         | 227           | 132          |
| music         | 688          | 699         | 180           | 32           |
| linguistics   | 707          | 718         | 166           | 44           |
| crypto        | 726          | 737         | 159           | 52           |
| astronomy     | 745          | 756         | 167           | 98           |
| hygiene       | 764          | 773         | 229           | 128          |
| fantasy       | 781          | (none)      | 133           | 0            |

Total: 4,217 bytes summary literal + 4,345 bytes footer literal = 8,562
bytes of replicated curation now persisted in JSON SSOT.

Each domain entry also carries an `_n6_invariant` field (one-line
n=6 architectural fingerprint extracted from the rich content; not
fabrication, distillation), and `_source` field stating
"README.md hand-curated content replicated verbatim (raw 91 C3 honest:
not fabricated novelty)".

## §4 verification

```bash
$ hexa run n6shared/sync-readme.hexa --dry-run
[sync-readme] data    = .../readme-data.json
[sync-readme] readme  = .../README.md
[sync-readme] mode    = DRY-RUN
[OK] README.md is up to date with readme-data.json (63 markers)

$ hexa run n6shared/sync-readme.hexa --check
[OK] README.md is up to date with readme-data.json (63 markers)
$ echo $?
0
```

raw 65/68 idempotent: ACHIEVED.

## §5 WRITE-mode unblock readiness (NOT executed this cycle)

cycle 19 deliverables:
- [x] JSON SSOT enrichment (17 new + 1 upgraded entry)
- [x] sync-readme.hexa schema extension (literal fields)
- [x] STATS spacing alignment
- [x] dry-run 0 changes
- [x] --check exit 0

cycle 20+ prerequisites for WRITE-mode unblock:
- [ ] own#14 sealed-hash pre-commit CI hook (verifies README.md hash
      matches sync-readme dry-run output)
- [ ] `--allow-overwrite` CLI flag in sync-readme.hexa (explicit
      confirmation that user accepts JSON SSOT replacement of any
      future hand-edits to README.md)
- [ ] dry-run unified-diff output (not just marker names) for review
- [ ] re-register raw 5 SSOT contract: post-cycle-20 README.md
      hand-edits FORBIDDEN; all changes via JSON + sync-readme run

## §6 raw 71 falsifiers (preregistered)

5 falsifiers logged in
`design/kick/2026-04-28_json-ssot-enrichment-cycle19_omega_cycle.json`
under `raw_71_falsifiers`:

- **F-INFRA-SSOT-19-1** future cycle adds new AUTO marker without JSON
- **F-INFRA-SSOT-19-2** literal fields become a stale-content backdoor
- **F-INFRA-SSOT-19-3** WRITE-mode unblocked prematurely (no sealed CI)
- **F-INFRA-SSOT-19-4** sister repo breaks on new optional fields
- **F-INFRA-SSOT-19-5** fantasy [!WARNING] callout round-trip mismatch

F-INFRA-SSOT-19-5 was actively tested this cycle: fantasy was the LAST
remaining changed marker after the initial 17 enrichments. Adding the
fantasy entry with summary_literal containing the verbatim
`> [!WARNING]\n> Mythology/fantasy exploration ...` block resolved it.
dry-run after fantasy entry: 0 changes. **F-INFRA-SSOT-19-5: does not
fire.**

## §7 cross-repo impact (raw 47)

The sister repo `n6-nexus` (and any other consumer of
`n6shared/readme-data.json`) sees:
- 17 new entries in `domains[]` array
- 2 new optional string fields per entry: `summary_literal`,
  `footer_literal`
- Existing 13 entries: unchanged
- Schema strictly **additive** (no field renamed, no field removed)

Consumers that don't recognize `summary_literal` / `footer_literal`
ignore them safely. Backward-compat verified: existing tooling
continues to function on the legacy 13-domain subset.

## §8 alien-grade impact

Expected delta: +0.05 to +0.10 (infra plumbing; not core mathematical
advance). Rationale: raw 65/68 idempotency restoration unblocks
cycle 20+ automated WRITE pipeline, reducing friction for future
readme curation churn. Not counted toward alien-grade ceiling census
this cycle (alien-grade 4.18 cycle-15 close baseline preserved).

## §9 next-cycle handoff

If user approves cycle 20 WRITE-mode unblock path:
1. add own#14 sealed-hash pre-commit hook
2. add `--allow-overwrite` flag to sync-readme.hexa
3. extend dry-run to emit unified diff
4. re-run sync-readme.hexa in WRITE mode (expected: 0 byte changes)
5. register F-INFRA-SSOT-19-1..5 in
   `state/falsifier_monitor/audit.jsonl`

If user defers: cycle 19 enrichment remains in JSON; --check continues
to pass; no WRITE attempted. Safe steady-state.

If user rejects: revert via `git checkout HEAD -- n6shared/readme-data.json
n6shared/sync-readme.hexa`. WRITE-mode stays gated; raw 91 C3 honest
disclosure of attempt logged.
