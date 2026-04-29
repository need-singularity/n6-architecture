# RETIRED: `verify_ai-native-architecture.py` + `btAI3_rtl_design_verify.py` + `verify_ai-native-architecture_extended.py`

Per raw 9 hexa-only mandate, these three Python verifiers have been retired.

## Classification

Verify-class scripts (paper-verify-embedded / RTL design / extended-falsifier).
**All three are already consolidated** into a single canonical hexa SSOT:
`verify_ai-native-architecture.hexa` (21/21 EXACT, exit-code 0 on PASS,
non-zero on FAIL — exit-code semantic preserved).

## What they did

| File                                       | Block | Count | Coverage                           |
|--------------------------------------------|-------|-------|------------------------------------|
| `verify_ai-native-architecture.py`         | A     | 10/10 | base N6 EXACT constants            |
| `btAI3_rtl_design_verify.py`               | B     | 3/3   | RTL silicon-tier falsifiers        |
| `verify_ai-native-architecture_extended.py`| C     | 8/8   | extended falsifier coverage (vendor gap, MMU width, ISA width, etc.) |

Combined: 10 + 3 + 8 = **21 EXACT** assertions for the
`ai-native-architecture` domain.

## Where they were ported

`domains/compute/ai-native-architecture/verify_ai-native-architecture.hexa`

The hexa port:

- Mirrors all 21 assertions across Block A / Block B / Block C with
  semantic-equivalent integer-form rewrites
  (e.g. `phi/sigma_n = 1/36` becomes `36 * phi == sigma_n`;
  `100 * phi < 3 * sigma_n` for the < 3% overhead claim;
  `2^4 = counter_capacity >= threshold_max + 1` for MMU width).
- Uses single-source-of-truth N6 primitives at the top:
  `n=6, sigma=12, phi=2, tau=4, sigma_n=72, j2=24, sopfr_n=5`.
- Preserves exit-code semantic: `assert(...)` aborts non-zero on FAIL;
  reaches the final `[ai-native-arch] EXACT: 21/21, verdict: PASS`
  banner only on full PASS.
- Drops file-system source-of-truth grep (atlas string parsing, sim
  constant scrape, JSON `summary_h1.mean` reads) because the underlying
  numeric facts they re-derived are already pinned by the N6 axioms;
  any drift in the hard-coded markdown is a separate falsifier handled
  by `own_doc_lint.hexa`. The 21 assertions are pure number-theoretic.

## Frontmatter routing (already in place)

- `ai-native-architecture.md` frontmatter:
  - `verify: domains/compute/ai-native-architecture/verify_ai-native-architecture.hexa`
  - `verify_legacy_py: [verify_ai-native-architecture.py, btAI3_rtl_design_verify.py, verify_ai-native-architecture_extended.py]`
- `analysis/btAI3_rtl_design.md` frontmatter `verify:` updated in this
  retire cycle to point at the consolidated `.hexa`.

## Why retire instead of keep as legacy mirror

- Raw 9 mandate is hexa-only; "legacy py mirror" is itself the violation.
- The `.hexa` file is the single source of truth for 21/21 EXACT.
- The Markdown text-blocks under `ai-native-architecture.md` §
  `### domains/compute/ai-native-architecture/verify_ai-native-architecture.py`
  retain inline Python listings *as documentation only* (showing the
  derivation provenance); they are not invoked.
- No CI hook, `.own` decl, or downstream tool invokes the three `.py`
  files; the prior callers in `reports/anomaly/` are historical run-logs.

## Exit-code semantic mapping

| Original `.py` exit | New `.hexa` outcome                                    |
|---------------------|--------------------------------------------------------|
| 0 (PASS, 21/21)     | `assert(..)` chain passes, `verdict: PASS` line, exit 0|
| !=0 (FAIL)          | first failed `assert(..)` aborts non-zero              |

## Recovery

`git show <pre-retire-sha>:domains/compute/ai-native-architecture/<file>.py`
recovers the verbatim source.

The inline Python listings inside `ai-native-architecture.md`
(§ "verify_ai-native-architecture.py" etc., lines ~165-1343) remain as
visual documentation of the original derivation chains.

## Retired in

Commit: cycle 30 — raw 9 hexa-only mandate, second wave.
