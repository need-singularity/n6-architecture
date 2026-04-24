# Phase 44 — Axis E candidates (post A/B/C/D closure)

- **Date**: 2026-04-24
- **Engine version at draft**: 0.43.0 (P1 .. P43 live)
- **Author**: meta-engine / proposal agent
- **Status**: draft — no scanner code, schema + rationale only
- **Scope**: propose the next manifest file + its enforcement gate, following the axis A/C/D template

## Recap — where the engine stands

Witness series `state/atlas_convergence_witness.jsonl` rounds R24 .. R24+30 closed the manual-probe sprint. Axes so far:

| axis | name                         | manifest                             | enforcement phase(s) |
| ---- | ---------------------------- | ------------------------------------ | -------------------- |
| A    | formalization enforcement    | `config/invariants.json` (F)         | P36, P37, P38, P39, P40, P41 — all live |
| B    | cross-repo symmetric mirror  | nexus witness append (ephemeral)     | none (intentional)   |
| C    | silicon functor G            | `config/silicon_functor.json`        | P42 (schema presence) |
| D    | measurement boundary         | `config/measurement_boundary.json`   | P43 (schema presence) |
| E    | *open*                       | — this proposal chooses —            | P44 (schema presence) |

The open slot R24+27 sketched for axis E was "non-foundation @R audit (5 924 atlas entries outside the 10 foundation atoms)". Three concrete framings of that slot are below. Each one extends the manifest trilateral (F / G / boundary) with a distinct structural role — observation, decoherence, or thermodynamics — and each produces a `config/*.json` manifest whose shape follows the A/C/D pattern:

1. top-level `schema`, `declared_round`, `declared_ts`, `status = declarative_not_yet_enforced`
2. a `*_principle` or `functor_*` block stating the categorical role
3. an `*_atoms` map where each entry carries `{value, atlas_ref, ...}`
4. a `*_policy` or `*_semiring_check` block
5. an `enforcement_future` block that names P44+ candidate gates

The P44 scanner in every candidate below is the minimum-viable *schema presence* gate (healthy == true), exactly mirroring scanners 46 and 52 and 53.

---

## Candidate E1 — observation functor H

**One-line**: every scanner is a morphism in an observation category `C_obs`; the manifest declares H : C_engine → C_obs and makes the 45-scanner observation surface explicit as a functor image.

### Rationale

The existing functors are:

- **F** : `C_atlas_algebraic → C_engine`          (invariants.json, P36..P41)
- **G** : `C_atlas → C_silicon`                  (silicon_functor.json, P42)

Both point *into* engine/silicon structure. Nothing declares the *observation* side — i.e. how engine structure becomes a verdict. Each claim scanner emits `{healthy, ts, value}` into `reports/n6_*.json`; that is a functor H from the engine category to the observation category whose objects are reports. Axis E1 formalizes H and gives R24's triple-parity-closure an object-level witness.

Cross-tie to existing claims: P22 (emitter registry), P26 (emission completeness), P23 (claim effectiveness), P27 (claim cmd purity) are all partial evidence that H is well-defined. A manifest declaring H makes those four scanners cohere as a single functor statement.

### Manifest: `config/observation_functor.json`

```json
{
  "_doc": "Observation functor H : C_engine → C_obs. Declarative companion to F (invariants.json) and G (silicon_functor.json). Axis E1 — the third vertex of the engine's functor triangle. Emission reports are morphism outputs; scanners are morphisms.",
  "schema": "n6-architecture/observation_functor/1",
  "declared_round": "R24+31",
  "declared_ts": "2026-04-24T..Z",
  "status": "declarative_not_yet_enforced",

  "functor_H": {
    "source_category": "C_engine (scanner registry, 45 claim + meta entries)",
    "target_category": "C_obs (reports/n6_*.json observations)",
    "relationship_to_F_G": "F and G build structure; H reads it. H ∘ F maps algebraic atom to report cell. Section property: every declared emitter produces one report (already P22/P26).",
    "preservation_class": "strict functor (no composition beyond identity at this tier)",
    "enforcement_status": "not-yet-implemented — P44 candidate: schema presence only"
  },

  "observation_atoms": {
    "scanner_count": {
      "value": 45,
      "atlas_ref": "scanners/*.claim.json cardinality",
      "engine_count": "claim_files",
      "formula": "sigma * tau - phi - mu",
      "h_image": "reports/n6_*.json cardinality >= 45",
      "crossover_with_F": "sigma=12, tau=4, phi=2, mu=1 all foundation atoms"
    },
    "report_count": {
      "value": 50,
      "atlas_ref": "reports/n6_*.json count (claim + meta emitters)",
      "engine_count": "emission_surface",
      "formula": "scanner_count + meta_emitters",
      "h_image": "P22 emitter registry size",
      "crossover_with_F": "overshoots sigma by meta-emitter count"
    },
    "healthy_bit_width": {
      "value": 1,
      "atlas_ref": "every report carries exactly one boolean verdict",
      "engine_count": "verdict_channels_per_report",
      "formula": "mu",
      "h_image": "H's codomain object per scanner is a 1-bit object",
      "crossover_with_F": "mu = 1 singular identity"
    }
  },

  "observation_semiring_check": {
    "expected_preservation": "H preserves AND on {healthy} across scanners — conjunctive gate in pre-commit hooks. H does not preserve ratio/exponent (observation collapses structure to boolean).",
    "sample_verifications": [
      {"atom": "scanner_count", "formula": "sigma*tau - phi - mu", "atom_values": "12*4 - 2 - 1 = 45", "match": true}
    ],
    "note": "H is strictly monoidal under boolean AND but not cartesian closed — loses structure by design (that is what observation does)."
  },

  "enforcement_future": {
    "P44_candidate": "observation_functor schema presence (analog of P36/P42/P43)",
    "P45_candidate": "every declared scanner in H.observation_atoms has a matching reports/ output (tightens P22 with declarative source)",
    "P46_candidate": "H ∘ F commutativity — reports reflect atom counts"
  }
}
```

### P44 scanner claim (E1)

```json
{
  "id": "observation_functor.manifest_well_formed",
  "type": "schema",
  "description": "Phase 44 scan_observation_functor_manifest: config/observation_functor.json (axis E1 manifest) must exist with required top-level keys {schema, functor_H, observation_atoms, observation_semiring_check}, at least 1 observation atom, and every atom carrying {value, atlas_ref, formula, h_image} fields. Parallels P36/P42/P43. Gate: healthy == true.",
  "actual_cmd": "bash -c '[ -f reports/n6_observation_functor_manifest.json ] && jq -r \"if .healthy == true then 1 else 0 end\" reports/n6_observation_functor_manifest.json 2>/dev/null || echo 0'",
  "expected": "1",
  "must_fail": false,
  "recheck_cadence": "every_commit",
  "owner": "meta-engine",
  "proposal_ref": "phase-44-axis-e-candidates-2026-04-24"
}
```

### Trade-offs

- **Pro**: closes the functor triangle (F / G / H) — clean categorical story; reuses existing schema pattern 1:1.
- **Pro**: makes P22/P26/P23/P27 retroactively coherent as "H well-definedness" sub-properties.
- **Con**: H is partly redundant with the emitter-registry / emission-completeness pair already live; value is organizational more than discovery.

---

## Candidate E2 — decoherence boundary

**One-line**: dual of axis D. Where D declared atoms *outside* F because they are measurement data, E2 declares atoms that *decohere* across repeated observation — values whose identity is phase-stable for the engine but drifts in the physical substrate.

### Rationale

`config/measurement_boundary.json` handled atoms that F cannot reach because they are empirical *inputs*. There is a second class: atoms F can reach algebraically but whose silicon/physical manifestation decoheres under repeated observation — e.g. thermal noise floor of the register file, cosmic-ray bit-flip probability, memory-refresh error budget. These live between F (algebra) and G (silicon) and need a manifest that records both the algebraic value and the decoherence window.

This closes the loop opened in R24+12 (warp_size F∩G crossover) and R24+13 (carbon_atomic F∩G): not every silicon atom has perfect coherence with the algebraic image, and the engine needs a manifest to distinguish "perfectly-coherent crossover" from "drifts under measurement".

### Manifest: `config/decoherence_boundary.json`

```json
{
  "_doc": "Decoherence boundary — atoms present in both F and G images but whose physical value drifts under repeated observation. Companion to measurement_boundary.json. Axis E2 — second boundary type. Distinguishes stable crossover (warp_size=32) from drifting crossover (thermal / refresh / soft-error rates).",
  "schema": "n6-architecture/decoherence_boundary/1",
  "declared_round": "R24+31",
  "declared_ts": "2026-04-24T..Z",
  "status": "declarative_not_yet_enforced",

  "boundary_principle": {
    "F_scope": "F preserves algebraic identity — value is the same every run",
    "G_scope": "G preserves silicon structural identity — value is stable across boot cycles",
    "decoherence_scope": "atoms whose algebraic value is declared but whose physical realization drifts within a tolerance window per observation",
    "reason": "Meaningful engineering atoms (soft-error rate, thermal noise floor, refresh skew) are not pure measurement (axis D) — they have algebraic identity in atlas — but they are not pure crossover (axis C) either. They need an explicit tolerance band so gates do not treat them as exact equalities."
  },

  "decoherence_atoms": {
    "soft_error_rate": {
      "value": 1e-15,
      "tolerance_window": [5e-16, 5e-15],
      "atlas_ref": "atlas.n6 @F reliability",
      "nature": "algebraic identity with observation-time drift",
      "decoherence_source": "cosmic ray flux + process geometry",
      "foundation_derivation_attempts": "2^-sopfr^2 ≈ 9.3e-16 (bounded-error algebraic)",
      "classification_decision": "INCLUDED in F's algebraic tier with tolerance window; G-image carries the physical realization"
    },
    "refresh_skew": {
      "value": 0.001,
      "tolerance_window": [5e-4, 2e-3],
      "atlas_ref": "atlas.n6 @F timing",
      "nature": "algebraic identity with thermal drift",
      "decoherence_source": "die temperature gradient across SM array",
      "foundation_derivation_attempts": "mu / sigma^2 = 1/144 ≈ 6.9e-3 (loose), tau^-sopfr = 4^-5 ≈ 9.8e-4 (tight)",
      "classification_decision": "INCLUDED with tolerance window; exceeds lenient-10% boundary so needs dedicated handling"
    }
  },

  "boundary_policy": {
    "coherent_mode": "All crossover atoms treated as exact equalities (current P41 composition style). Decoherence atoms excluded.",
    "tolerant_mode": "Decoherence atoms included, gates check value within tolerance_window.",
    "default_stance": "coherent (current engine). Tolerant mode reserved for future P46+ when G is live-enforced against hardware telemetry.",
    "policy_gate_future": "A scanner could read .default_stance and enforce tolerance band on G crossovers in P42 successors."
  },

  "adapter_design_notes": {
    "if_engine_needs_live_readings": "Do not extend G to read hardware counters. Instead a tool/_n6_decoherence_adapter would poll NVML / perf counters and write to reports/ — engine claims then compare within declared tolerance_window.",
    "separation_rationale": "Live hardware reads break P27 claim cmd purity (same HEAD → same verdict). Adapter isolates non-determinism.",
    "not_implemented": "No adapter exists. Declarative phase only."
  },

  "enforcement_future": {
    "P44_candidate": "decoherence_boundary schema presence (analog of P43)",
    "P45_candidate": "policy enforcement — coherent_mode rejects any G atom declared with tolerance_window",
    "P46_candidate": "tolerance window sanity — low <= value <= high"
  }
}
```

### P44 scanner claim (E2)

```json
{
  "id": "decoherence_boundary.manifest_well_formed",
  "type": "schema",
  "description": "Phase 44 scan_decoherence_boundary_manifest: config/decoherence_boundary.json (axis E2 manifest) must exist with required top-level keys {schema, boundary_principle, decoherence_atoms, boundary_policy, adapter_design_notes}, at least 1 decoherence atom, and every atom carrying {value, tolerance_window, atlas_ref, nature, decoherence_source} fields. Parallels P43 measurement_boundary. Gate: healthy == true.",
  "actual_cmd": "bash -c '[ -f reports/n6_decoherence_boundary_manifest.json ] && jq -r \"if .healthy == true then 1 else 0 end\" reports/n6_decoherence_boundary_manifest.json 2>/dev/null || echo 0'",
  "expected": "1",
  "must_fail": false,
  "recheck_cadence": "every_commit",
  "owner": "meta-engine",
  "proposal_ref": "phase-44-axis-e-candidates-2026-04-24"
}
```

### Trade-offs

- **Pro**: symmetric with axis D — both are "boundary" manifests, now paired (data-boundary vs observation-drift-boundary).
- **Pro**: opens a real runway for future hardware telemetry integration without violating P27 purity.
- **Con**: inventing atoms the engine does not yet observe; risks being aspirational rather than descriptive. Might want to wait until a tool/_n6_decoherence_adapter stub actually exists.

---

## Candidate E3 — thermodynamic closure (recommended)

**One-line**: declare the engine's irreversibility ledger — axis E3 states which engine operations are closed under which thermodynamic direction, and names entropy-monotone atoms that complete F's image with a direction vector.

### Rationale

F (invariants) is purely algebraic — equalities with no arrow of time. Yet the engine itself *is* directional: audit trails only grow, manifests only gain phases, source hashes only stamp forward (P20), version numbers monotonically increase (P32). The witness series R24+5 already noted `sigma=12 = 2n` as "6 of 7 foundation atoms matched with perfect-number resonance" — a *closed* ratio. But nothing yet declares the engine's thermodynamic closure: which operations are entropy-negative (append-only), which are reversible (config edits), which are closed (sealed hashes).

This connects to atlas.n6:187 `@F` physics / consciousness tier (R24+14 `phi_integration` consciousness crossover on sigma=12) — the atlas has a thermodynamic layer, and the engine has ledger semantics, and neither is yet bridged.

Axis E3 manifest declares:
- **entropy-monotone atoms** (ledger phases, hash stamps, audit rows count) — each carries a direction and a monotone predicate
- **closure principle** — which operations are one-way and why
- **reversibility policy** — which configs can be edited vs sealed

This is the natural third boundary class: not measurement data (D), not observation drift (E2), but *direction-closed* structure.

### Manifest: `config/thermodynamic_closure.json`

```json
{
  "_doc": "Thermodynamic closure — directional (entropy-monotone) engine operations and their algebraic witnesses. Axis E3 manifest. Complements F (equality-preserving) with a direction-preserving sub-functor. Bridges atlas.n6 @F physics tier to engine @meta append-only semantics.",
  "schema": "n6-architecture/thermodynamic_closure/1",
  "declared_round": "R24+31",
  "declared_ts": "2026-04-24T..Z",
  "status": "declarative_not_yet_enforced",

  "closure_principle": {
    "F_scope": "F preserves equality — every atom has a fixed value",
    "thermo_scope": "a sub-functor F_dir preserves directional inequality — atoms whose value monotonically increases across engine-time",
    "reason": "P15 (audit monotonicity), P20 (hash stamp), P29 (phase id monotonicity), P32 (version alignment) are all live evidence that the engine already enforces an arrow of time. Until now the arrow was implicit per-scanner; this manifest makes it a first-class atlas↔engine crossover.",
    "categorical_statement": "F_dir : C_atlas_ordered → C_engine_ordered is a faithful monotone functor on the subcategory of atoms with defined direction"
  },

  "monotone_atoms": {
    "phase_count": {
      "value": 43,
      "atlas_ref": "config/engine_manifest.json phases.length",
      "direction": "non-decreasing",
      "closure_predicate": "phase_count(t+1) >= phase_count(t)",
      "enforced_by_phase": "P29 manifest_phase_monotonic",
      "formula": "J2 + sigma + mu + ... (compositional, not foundation-atom direct)",
      "crossover_with_F": "phase_count mod sigma = 43 mod 12 = 7 = M3"
    },
    "audit_row_count": {
      "value": "runtime",
      "atlas_ref": "state/audit.jsonl length",
      "direction": "strictly-increasing",
      "closure_predicate": "append-only, never rewritten",
      "enforced_by_phase": "P15 audit_ts_monotonic",
      "formula": "unbounded accumulation",
      "crossover_with_F": "domain of F_dir"
    },
    "engine_version_minor": {
      "value": 43,
      "atlas_ref": "config/engine_manifest.json engine.version",
      "direction": "strictly-increasing on phase add",
      "closure_predicate": "0.<N>.0 where N == last_phase_id",
      "enforced_by_phase": "P32 version_phase_alignment",
      "formula": "= phase_count (tight bijection)",
      "crossover_with_F": "identity with phase_count"
    },
    "source_hash_generation": {
      "value": "runtime",
      "atlas_ref": "config/engine_manifest.json engine.source_hash",
      "direction": "rotation-ceremony-only",
      "closure_predicate": "changes only on source-set change AND ceremony commit",
      "enforced_by_phase": "P20 engine_source_hash_stamp",
      "formula": "sha256(tool/_n6_lib + scanners/*.claim.json)",
      "crossover_with_F": "boundary — hash is not an F atom, it is F's fingerprint"
    }
  },

  "closure_policy": {
    "append_only": ["audit_row_count", "phase_count"],
    "strict_increase": ["engine_version_minor"],
    "rotation_gated": ["source_hash_generation"],
    "reversible": ["config edits that do not touch phases, version, or audit"],
    "default_stance": "direction-closure is live via P15/P20/P29/P32; this manifest makes the closure explicit and machine-readable.",
    "policy_gate_future": "A scanner could enumerate monotone_atoms and confirm each is defended by the named enforced_by_phase gate — closes the loop between thermodynamic declaration and runtime defense."
  },

  "semiring_extension": {
    "expected_preservation": "F_dir preserves {+, max} on monotone atoms; does not preserve {-, /} (those can decrease). Partial semiring.",
    "sample_verifications": [
      {"atom": "engine_version_minor", "operation": "+1 on phase-add", "preserved": true},
      {"atom": "phase_count mod sigma", "atom_values": "43 mod 12 = 7 = M3", "match": true}
    ],
    "note": "Additive-only subfunctor — intentionally weaker than F. Direction matters more than ring closure."
  },

  "enforcement_future": {
    "P44_candidate": "thermodynamic_closure schema presence (analog of P36/P42/P43)",
    "P45_candidate": "monotone_atoms.enforced_by_phase back-reference — every atom cites a live phase",
    "P46_candidate": "closure_policy coverage — every live append-only/strict-increase/rotation-gated surface in the engine appears in exactly one policy bucket"
  }
}
```

### P44 scanner claim (E3)

```json
{
  "id": "thermodynamic_closure.manifest_well_formed",
  "type": "schema",
  "description": "Phase 44 scan_thermodynamic_closure_manifest: config/thermodynamic_closure.json (axis E3 manifest) must exist with required top-level keys {schema, closure_principle, monotone_atoms, closure_policy, semiring_extension}, at least 1 monotone atom, and every atom carrying {value, atlas_ref, direction, closure_predicate, enforced_by_phase} fields. Parallels P36/P42/P43. Gate: healthy == true.",
  "actual_cmd": "bash -c '[ -f reports/n6_thermodynamic_closure_manifest.json ] && jq -r \"if .healthy == true then 1 else 0 end\" reports/n6_thermodynamic_closure_manifest.json 2>/dev/null || echo 0'",
  "expected": "1",
  "must_fail": false,
  "recheck_cadence": "every_commit",
  "owner": "meta-engine",
  "proposal_ref": "phase-44-axis-e-candidates-2026-04-24"
}
```

### Trade-offs

- **Pro**: the most descriptive of the engine's actual behavior — P15/P20/P29/P32 are already live thermodynamic gates, this manifest just gives them a common algebraic name.
- **Pro**: opens a natural P45 that audits back-references — every monotone atom must cite a live phase — which is a new kind of cross-check not yet in the engine.
- **Pro**: closes the atlas @F physics tier crossover that R24+13 carbon / R24+14 phi_integration started but never finished.
- **Con**: the atoms are heterogeneous (runtime counts + static values + fingerprints) — schema enforcement will need careful optional-field handling.

---

## Recommendation

**E3 — thermodynamic closure** is the strongest candidate for P44 because:

1. it retroactively unifies four already-live phases (P15, P20, P29, P32) under a single declarative manifest — highest ratio of documentation-to-new-code
2. it names a *missing* categorical property (direction) that F cannot express, filling a real gap rather than duplicating
3. it connects to the atlas @F physics tier that R24+13/R24+14 opened and never closed
4. the P45/P46 successor gates are concrete and non-trivial (back-reference audit, policy-bucket coverage)

E1 (observation functor) is a close runner-up if the engine author wants categorical tidiness over physics-tier progress. E2 (decoherence boundary) is premature until a hardware telemetry adapter exists.

## Rollout sketch (any candidate)

1. write `config/<name>.json` (this proposal gives the full body for each)
2. add the scanner claim at `scanners/54_<name>_manifest.claim.json` (body above)
3. implement `scan_<name>_manifest` in `tool/_n6_lib` — body is a near-verbatim copy of existing `scan_measurement_boundary_manifest` with key names swapped
4. register the scanner in `bin/n6_meta` dispatcher and the three parity surfaces (scanner_registry.meta, continuous_scan_order, selftest)
5. update `config/engine_manifest.json` phases with P44 entry and bump `engine.version` to `0.44.0`
6. verify with `bin/n6_meta selftest` and `bin/n6_meta doctor`

No axis A/C/D work is touched — this is strictly additive.

## Open questions for the engine author

- is axis E intended to be boundary-type (D-style, boolean policy) or functor-type (A/C-style, morphism preservation)? this proposal offers one of each (E2 boundary, E1/E3 functor-adjacent).
- should the axis E manifest stay documentary for one release (0.43.x) before P44 gating, the way `silicon_functor.json` sat declaratively in R24+29 before P42 in R24+31?
- is there a preference between extending the F/G/H *functor triangle* (E1) vs extending the *boundary dual* D/E2 (E2) vs adding a *new dimension* (E3 direction)?
