# own#1 HARD English-Only Mass Translation Roadmap — 1,050 legacy → 0 by 2026 Q4

**Date**: 2026-04-24
**Author**: own-lint policy scoping (automatic proposal, no engine/config changes)
**Status**: scoping only — roadmap draft prior to execution phase
**Scope**: adds only this single file under `proposals/`. allowlist/tool/bridge/n6shared/.own remain untouched.
**Compliance**:
- own#1 (doc-english-hard) — this proposal lives under `proposals/` and is outside own#1 scope at the time of writing. Korean is permitted (candidate for future translation once scope expands).
- own#11 (bt-solution-claim-ban) — translation progress is described only as "draft/target". An incomplete phase must not be asserted as "solved/done".
- own#17 (README English-only) — README is outside the scope of this document.

---

## 1. Executive Summary

With own#1's promotion to `on_fail: block` (`2ef79fe7` → `4b472b0c`), new .md files containing any Korean bytes are now a **HARD block**. The existing 1,050 legacy files were grandfathered via `tool/own1_legacy_allowlist.json`, but under the user's emphatic **"hard!!! english only for everything"** principle, the allowlist itself is a shrinkage target; the medium-term goal is **polyfill retirement → absolute HARD enforcement**.

The **draft target** of this roadmap is to shrink the legacy count from 1,050 → 0 by 2026 Q4 and to delete the allowlist file itself, so that own#1 HARD is enforced repository-wide without exception. At present, Phase 0 (bridge + n6shared) is in flight; every other phase is in target state.

## 2. Current Inventory

| Directory | Files | Max CJK/file | Difficulty | Priority | Notes |
|---|---:|---:|:---:|:---:|---|
| `domains/` | 417 | 1457 | HIGH | P2 | 4 files in culture/ are the largest debt (~1450 chars/file). chip-* sub-domains tracked separately |
| `reports/` | 284 | (not tallied) | MED | P3 | Many auto-generated formats → prioritize template translation |
| `papers/` | 165 | (not tallied) | HIGH | P4 | High formula/citation density, AST verification required |
| `theory/` | 149 | 896 | HIGH | P4 | Conceptual depth — glossary enforcement required |
| `experiments/` | 25 | 829 | MED | P1 | Pilot target (small + manageable difficulty) |
| `n6shared/` | 6 | 417 | LOW | P0 | Running in parallel (agent B) |
| `bridge/` | 4 | (not tallied) | LOW | P0 | Running in parallel (agent B) |
| **Total** | **1,050** | — | — | — | Per the latest report |

Caveat (own#11): P0 entries above are "in-flight / target completion" at the time of writing; they are not described as "solved" until their commits land.

## 3. Phase Plan (5 phases + Phase 0)

### Phase 0 — Immediate (2026-04-24 target)
- Scope: `bridge/` 4 + `n6shared/` 6 = **10 files**
- Owner: agent B (currently running in parallel)
- allowlist goal: 1,050 → **1,040**
- Status: in-flight (this proposal does not intervene)

### Phase 1 — experiments/ pilot (2026-05 target)
- Scope: 25 files, MED difficulty
- Purpose: validation sample for the translation pipeline
- Deliverables: actual batch output from `tool/batch_translate.py`, error regression report
- allowlist goal: 1,040 → **1,015**

### Phase 2 — domains/ priority track (2026-05 ~ 06 target)
- Scope: the top 200 priority files out of 417
  - 4 files in `domains/culture/` (largest debt, 1457 chars)
  - `chip-*` sub-domains (many consistent templates)
- Deliverables: 200/417 drafts completed
- allowlist goal: 1,015 → **815**

### Phase 3 — reports/ bulk (2026-07 target)
- Scope: 284 files
- Strategy: automated translation + automated review pipeline (high template repetition rate)
- allowlist goal: 815 → **531**

### Phase 4 — papers/ + theory/ (2026-08 ~ 09 target)
- Scope: 165 + 149 = **314 files**
- High-difficulty axes: formula preservation, theoretical-context semantic fidelity, citation hash invariance
- Anthropic API review loop 2-pass required (1st: translation, 2nd: technical-term re-verification)
- allowlist goal: 531 → **217**

### Phase 5 — domains/ remainder + cleanup (2026-10 ~ 12 target)
- Scope: remaining 217 files under `domains/`
- Final step: delete the allowlist file itself → own#1 HARD exceptions = 0
- allowlist goal: 217 → **0**

## 4. Automation Pipeline Design

**Translation pipeline (draft design common to all phases)**

1. Batch selection: pull N files from the priority queue (batch size N = 10~25, adjusted per phase).
2. Translation run: Claude API (sonnet-4-6) — technical-term preservation prompt + glossary injection.
3. Three-stage verification:
   - CJK=0 regex check (reusing the own#1 runner).
   - markdown AST diff (structural equivalence of headers/links/code blocks/formula blocks).
   - Terminology consistency (glossary matching for `atlas`, `ouroboros`, `σ`, `φ`, `J₂`, `@R/@P/@C/@L`, BT-xxx, etc.).
4. allowlist shrink: remove the translated-file entries from `tool/own1_legacy_allowlist.json`.
5. Commit: `docs(translate): batch-N English conversion (N files)` format.
6. PR-based review (one human final sign-off).

**Tool candidates (draft target, not yet actual files)**

| File | Role | Status |
|---|---|---|
| `tool/batch_translate.py` | API call + verification orchestrator | target |
| `tool/translation_glossary.json` | English-retention term dictionary (atlas proper nouns / math symbols / BT codes) | target |
| `tool/markdown_ast_diff.py` | Structural equivalence check before/after translation | target |

**Prompt design guidelines (draft)**

- system: "You translate N6-architecture legacy Korean Markdown to English. Preserve all math ($...$), code blocks, frontmatter, links, and glossary terms verbatim. Do not add/remove sections."
- user: full source + glossary JSON + "return only translated markdown, nothing else".
- 2-pass (Phase 4 only): 1st=translation, 2nd=technical-term re-verification (return diff-only patch).

**Batch execution protocol**

- Up to 3 retries per file; on failure, hold in the manual queue.
- Daily batch ceiling: 50 files (to manage token cost cap).
- If failure rate > 10%, pipeline halt + retrospective round (own#19 agenda).

## 5. Expected Cost / Resources (estimates, not finalized)

- Token volume estimate: avg input 3~5K + output 3~5K per file → 1,050 × 10K ≈ **10.5M tokens**.
- Cost on Claude sonnet-4-6 basis: published rate × 10.5M tokens (exact rate to be reconfirmed at execution time).
- Human review effort: 10 min per file → **~175 hours**, distributed across phases (25~50 hours per phase).
- Execution window: 2026-04 ~ 2026-12, a 9-month window.

## 6. Risks & Mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Technical-term mistranslation | Semantic regression | glossary enforcement + prompt engineering + sampling review |
| Formula/code block corruption | Build / proof-chain breakage | `markdown_ast_diff.py` blocking gate |
| Raised entry barrier for developers | Korean-native contributor attrition | Provide a "Korean draft → auto-translate PR" workflow; discussion channels allow multiple languages |
| Translation-quality regression | Long-term documentation noise | Weekly sampling review + own#1 HARD re-verification |
| Concurrent-agent conflicts | Fast-forward failure | PR-level separation, serialized allowlist edits (agenda for own#19 roadmap review) |

## 7. Success Metrics

- **allowlist count**: 1,050 → 0 (2026 Q4 target).
- **CI own1-doc-english-hard**: 100% PASS rate sustained.
- **AST diff regressions = 0**: structural equivalence of markdown before/after translation.
- **Sampling review pass rate**: weekly N=10 sample, 90%+ technical-term accuracy.
- **Developer satisfaction survey**: quarterly (once per quarter).

## 8. Next Actions

- [ ] `tool/batch_translate.py` prototype (2026-04-27 target)
- [ ] `tool/translation_glossary.json` draft (2026-04-27 target)
- [ ] Phase 1 `experiments/` 25-file pilot (2026-05-04 target)
- [ ] Confirm Phase 0 completion and reflect allowlist shrinkage (inherit agent B results)
- [ ] Wire weekly roadmap review workflow (own#19)
- [ ] Promotion of an official execution proposal (scoping → execution) is deferred to a separate round

## 9. References

- own#1 (HARD): `/Users/ghost/core/n6-architecture/.own`
- allowlist SSOT: `/Users/ghost/core/n6-architecture/tool/own1_legacy_allowlist.json`
- Policy commit (promote to HARD): `2ef79fe7` — `feat(own): promote own#1 to HARD block for new docs`
- Policy commit (elevate on_fail to block): `4b472b0c` — `docs(own): elevate own#1 on_fail to block; add verify runner`
- Peer proposal: `proposals/kr-ai-grant-2026-strategic-matching-2026-04-24.md`
- Format references: `proposals/r24-31-round-open-2026-04-24.md`, `proposals/kr-ai-grant-2026-strategic-matching-2026-04-24.md`

---

*This document is a roadmap draft; the phase goals described are target dates, not completed execution. Actual shrinkage is confirmed only by the commits and PRs of each phase (own#11 honesty).*
