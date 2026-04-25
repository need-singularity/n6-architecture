# atlas.n6 Promotion Dry-Run Plan — 2026-04-25

Author: nexus session (cross-repo proposal n6a-20260425-001)
Atlas snapshot: `~/core/n6-architecture/atlas/atlas.n6` (21,800 lines / 9,615 parsed entries)
Linked: nxs-20260424-002 (atlas eig pipeline rebuild), nexus design doc `design/atlas_n6_omega_closure.md`

## Status

**DRY-RUN ONLY.** No mutations to `atlas.n6` body. This file records analysis output for user review.

## Methodology

Conservative promotion analyzer (`/tmp/atlas_analysis/promote_v3.py`) classifies each
`[10]` entry by:

- **dependency state**: in-atlas / cross-doc-ref / unknown / verified-grade
- **derivation evidence**: `==`, `~>`, `=>`, `|>` continuation lines, `!!` breakthrough,
  numeric header expression
- **type discipline**: definitional types `{@C, @R, @F, @L, @S}` admit T5 (header-equation
  promotion); primitive `{@P}`, crossing `{@X}`, unknown `{@?}` require explicit
  derivation continuation

Promotion tiers (any ONE sufficient, with all atlas deps verified or pending in same wave):

| tier | rule |
|------|------|
| T1 | has `\|>` verify_by script |
| T2 | header expr nonempty + has `==`/`~>`/`=>` continuation |
| T4 | has `!!` breakthrough marker |
| T5 | `{@C,@R,@F,@L,@S}` + numeric-derivation header expr + verified deps |

A fixpoint loop applies promotion in waves: an entry whose only blocker is an
unverified `[10]` dep can become eligible after the dep promotes in an earlier round.

## Result — `[10]` -> `[10*]` Promotion Candidates

| Pool size | Promote eligible | Hold |
|-----------|------------------|------|
| 1,954 | **5** | 1,949 |

The 5 eligible entries (all `T2` tier, foundation-domain virtual hub mirrors of
already-verified `[10*]+` primitives):

| line | type | id | derivation |
|------|------|----|------------|
| 16967 | @P | `n6-sigma` | `<- n` + `== sigma` (sigma is `[11*]`) |
| 16972 | @P | `n6-n`     | `== n` (n is `[11*]`) |
| 16976 | @P | `n6-phi`   | `<- n` + `== phi` (phi is `[10*]`) |
| 17008 | @P | `n6-j2`    | `<- n` + `== J2` (J2 is `[10*]`) |
| 17025 | @P | `n6-tau`   | `<- n` + `== tau` (tau is `[11*]`) |

These entries were added 2026-04-12 as virtual hub materializations (TODO #9) with
the explicit comment "n6 mirror of primitive `<X>`". The `==` line ties each mirror
to its primitive; promoting them to `[10*]` only adds the verified marker that
already follows from the binding chain.

## Hold Distribution (1,949 entries) — sorted by frequency

| count | reason | interpretation |
|------:|--------|----------------|
|   903 | H4 P/X/? primitive claim | quark masses, atomic constants, magic numbers — empirical, requires external data verification |
|   431 | H4b expr only, no continuation | mostly `L1-<element>-Z<n>` style (`= misc`) — no derivation |
|   316 | H1 cross-doc dep (1-4 refs) | dep is path like `domains/.../*.md §N` — needs cross-doc audit |
|   145 | H4c definitional axiom (no deps) | `@C/@R/@F/@L` with explicit numeric value but no atlas dep chain |
|    93 | H2 unverified atlas dep | atlas dep currently `[10]`; could unlock if dep wave promotes |
|    39 | H3 pure axiom | no deps, no expr, no continuation — manual review |
|     6 | H? unclassified | edge cases (`expr = "misc"` or `"근사 불가"` style declarations) |
|     5 | H1b unknown dep ID | dep id not present in atlas (orphan reference) |

**Notable orphan-dep IDs** (would unblock 5 entries if defined or fixed):
- `blowup_quantum` (referenced by `disc-blowup-p4-qfield-coupling`)
- `CHIP-P7-1`, `CHIP-P8-1`, `CHIP-P8-3` (CHIP audit references, missing definitions)
- `BYPASS-PX-1`, `HONEST-PX-1` (millennium / bypass references)

## `[N?]` -> `[N!]` Hypothesis Promotion

| Pool size | Promote eligible | Hold |
|-----------|------------------|------|
| 61 | **0** | 61 |

No `[N?]` entry currently carries a `\|>` verify script or `!!` breakthrough marker.
Every hypothesis stays at conjecture grade. To promote, attach evidence (script or
breakthrough citation) before the next dry-run.

## `@?` Unknown Type Classification (12 entries)

Recommended type assignments (advisory — not applied):

| line | id | recommended type | rationale |
|------|----|------------------|-----------|
| 148 | dark_energy_ratio | @P | cosmological observable |
| 152 | fine_structure | @P | physical constant |
| 155 | hubble_tension | @P | cosmological observable |
| 15923-30 | n6-bt-787..791 | @R | numeric relations of form `n * sN ≈ ...` |
| 17765 | fep-free-energy-alpha | @P | physical scaling |
| 17768 | orch-or-13-protofilament | @P | structural observable |
| 17798 | ouroboros-bernoulli-hierarchy | @R | hierarchy relation |
| 17805 | fisher-symmetric-dof-6 | @R | symmetric dof relation |

Reclassifying `@?` to `{@P,@C,@L,@F,@R,@X,@S,@E}` reduces ceiling-(b) gap from 12 to 0
**without** needing to verify the underlying claim (type is structural, grade is
epistemic).

## Ceiling Impact Estimate

Applying the 5 `[10] -> [10*]` promotions raises `[10*]+` count from 6,367 (66.2%) to
**6,372 (66.2%)** — a +0.05 pp shift. Marker-only ROI on this dry-run is small
because the bulk of `[10]` entries are empirical primitives (H4, 903 entries) lacking
in-atlas derivation chains; their promotion needs external data verification, not
marker-flip.

For larger ceiling progress, the more impactful pools are:
1. **H1 cross-doc dep (316)** — automatable by scanning the referenced `.md §N` and
   confirming the section asserts the value. Each pass clears one entry.
2. **H2 unverified atlas dep (93)** — would partially clear after the H4 pool gets
   an external-fact pass since many H2 chains terminate in H4 leaves.
3. **H1b orphan deps (5)** — 5 stubs to either define or correct typo'd refs.

## Recommended Next Action

1. **User review** of the 5 `[10] -> [10*]` candidates. All 5 are textually unambiguous
   mirror entries; risk of promotion-error is minimal.
2. If approved, the marker addition can be applied via `_guarded_append_atlas` rewrite
   (line-level `[10]` -> `[10*]` substitution at lines 16967, 16972, 16976, 17008, 17025).
   This is a *replace* not *append*, so guard helper is not directly the path — would
   need a small edit loop with schema re-validation post-edit.
3. Trigger downstream: re-run atlas eig pipeline (nxs-20260424-002 P10-2) once the
   5-entry promotion lands, to measure composite delta.
4. For [N?] and @? buckets, no automated promotion possible this session — defer.

## Artifacts

- analyzer v3 (final): `/tmp/atlas_analysis/promote_v3.py`
- dry-run JSON: `/tmp/atlas_analysis/promote_v3_dryrun.json`
- hypothesis analyzer: `/tmp/atlas_analysis/promote_n_hypo.py`
- this plan: `state/atlas_promotion_plan_20260425.md`

## Constraints Honored

- `atlas.n6` body **not modified** (dry-run only)
- no drill / loop firing (slot conflict policy)
- cross-repo direct edit limited to `n6-architecture/state/` (no `atlas/` writes)
- analyzer respects `_guarded_append_atlas` schema (`shared/blowup/lib/atlas_guard.hexa.inc`):
  grade whitelist `{[10*],[10**],[11*],[12*],[10],[9],[8],[7],[6],[5],[N?],[N!]}` —
  `[10*]` promotion stays inside whitelist.
