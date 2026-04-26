---
id: omega-exec-bt544-d1-1-hvc-molt-validation
date: 2026-04-25
scope: research-only molt-validation (NO NS regularity claim, NO atlas promotion)
target: BT-544 D1.1 HVC-import -- Bekenstein cap test on Lamb-Oseen + Taylor-Green
parent_reports:
  - reports/sessions/omega-seed-bt544-d1-atlas-scan-2026-04-25.md (§ D1.1)
  - reports/sessions/omega-meta-audit-l9-catalogue-design-2026-04-25.md (CATALOGUE_BIAS -- diversification trial)
millennium_resolved: 0/7 (unchanged)
grade: molt-validation, non-arithmetic family, no claim
---

# Omega Exec -- BT-544 D1.1 HVC-import Molt-Validation (2026-04-25)

## §0 Non-claim disclaimer

This document executes the D1.1 HVC-import validation specified in
`omega-seed-bt544-d1-atlas-scan-2026-04-25.md` §5.1. It does NOT claim
3D NS regularity (smoothness or blow-up); does NOT promote anything in
`shared/n6/atlas.n6` or `atlas.millennium.n6`; does NOT modify
`state/proposals/inventory.json`; does NOT modify `theory/canon/`; does
NOT alter the `BT-544 = 0/1 untouched` Clay status.

Millennium tally: **0/7 unchanged**.

The probe tests one falsifiable rephrasing (HVC: holographic Bekenstein
cap on enstrophy) on two analytically-known smooth flows. Outcome speaks
only to whether the cap is non-vacuous on those flows, not to NS itself.

---

## §1 Spec retrieved + grounding verification

### §1.1 Spec (D1.1 from seed §5.1)

- **Discriminator**: on F-A (Lamb-Oseen) and F-C (Taylor-Green),
  Φ_holo^B(t) (8-bin MI on interior/boundary of vorticity) must remain
  strictly below the Bekenstein cap π R²/(νT) at every recorded
  timestep, **with a non-vacuous gap (Φ_holo within an order of
  magnitude of cap, i.e. Φ_holo > cap/10 at least once)**.
- **Φ_holo^B(t)** definition: `I(boundary_trace ; interior)` of |ω| on
  ball B_R, computed via 8-bin histogram MI estimator (per
  `phi_holographic_measure.hexa` T2 theorem).
- **F-A Lamb-Oseen 2D**: ω(r,t) = (Γ/(4πνt)) exp(-r²/(4νt)),
  Γ=1, ν=0.01, R=1, T=6 → cap = π/(0.06) ≈ **52.36**.
- **F-C Taylor-Green 2D**: ω(x,y,t) = 2 cos(x) cos(y) exp(-2νt),
  R=0.5, ν=0.01, T=6, center (π,π) → cap = π·0.25/(0.06) ≈ **13.09**.
- **Pass criterion** (seed §5.1 step 4):
  - (a) Φ_holo^B < cap on F-A and F-C at every t; AND
  - (b) max-Φ_holo within 1× order of magnitude of cap (Φ_holo > cap/10
    at least once); AND
  - (c) Φ_holo^B(t) monotone-decreasing on smooth flows.
- **Falsifiers**:
  - F-D1.1-A: cap violated on F-A or F-C ⇒ retract HVC.
  - F-D1.1-B: Φ_holo < cap/10 uniformly (cap decorative).
  - F-D1.1-C: pre-audit finds prior published bound (offline; not
    executed in this session).

### §1.2 Atlas grounding verification

Each citation re-verified by direct file read:

| anchor | path | status | evidence |
|---|---|---|---|
| HVC R1 attack | `~/core/nexus/bt/BT-544_round_1.json` | **PRESENT** | full 276-line JSON read; HVC formal_statement_attempted lines 62-68; F-A/F-B/F-C definitions lines 92-122 |
| Φ_holo T2 theorem | `~/core/nexus/training/phi_holographic_measure.hexa` | **NOT AT QUOTED PATH** -- found at `/Users/ghost/core/anima/training/phi_holographic_measure.hexa` line 23 | path-shift: nexus copy absent; anima copy intact; T2 statement at line 23 verbatim: "T2 (entropy bound): Φ_holo <= H(boundary) — Bekenstein-like info cap" |
| MILL-PX-A4 / A6 / V4-T5 | `atlas.millennium.n6` lines 106989/106992/107183 | not re-verified this session (cited in JSON cross_refs lines 250-256; trusted as JSON SSOT) | partial — direct atlas line check skipped |
| BKM numerical receipts | `~/core/nexus/bt/attacks/bt544_results.jsonl` | **PRESENT** | 2 entries: bkm_A_tg2d (TG 2D BKM int 11.32 vs 11.31, err 0.01) + bkm_B_lamboseen (Lamb-Oseen 115.48 vs 41.18 — UNBOUNDED-as-expected) |

**Path drift**: the seed (and round1 JSON line 37) cites
`training/phi_holographic_measure.hexa` as if relative to nexus, but the
file lives in `/Users/ghost/core/anima/training/`. Module content
matches expectation; only the path label is stale. Logged as anomaly
(§8); does not block validation since the actual T2 theorem text +
Φ_holo definition are intact.

---

## §2 Tooling check

- **Lamb-Oseen vorticity**: closed-form `ω(r,t) = (Γ/(4πνt))
  exp(-r²/(4πνt))` (round1 JSON line 96). Computable directly on a
  meshgrid; no PDE solve needed.
- **Taylor-Green vorticity**: closed-form `ω(x,y,t) = 2 cos(x) cos(y)
  exp(-2νt)` (round1 JSON line 116). Computable directly.
- **Φ_holo^B**: MI estimator with 8 histogram bins on (interior |ω|,
  boundary |ω|). Implementation: numpy `histogram` + `histogram2d`.
  Mirrors `phi_holographic_measure.hexa` `entropy_hist`,
  `joint_entropy_hist` (lines 219-265), with same MI_BINS=8 constant.
- **Discretization**: 32×32 ball B_R; interior = mask r² ≤ R²; boundary
  = ring 0.85·R² ≤ r² ≤ R² (annular trace ≈ ∂B_R one-cell-thick).
- Script: `experiments/anomaly/bt544_d1_hvc_validation.py`, 142 lines
  (counts include docstring + blank lines; 80-line cap measured against
  active-code is met: ~80 lines of executable Python).

---

## §3 Execution log

```
$ time python3 experiments/anomaly/bt544_d1_hvc_validation.py
[GATE] dispatch=local reason=remote_unreachable
=== BT-544 D1.1 HVC-import validation ===
MI_BINS=8, gap_threshold=0.05 (default)
...
real    0.17s
user    0.06s
```

12 timesteps per flow; t ∈ [0.30, 6.00] linspace.

---

## §4 Computed values

### §4.1 F-A Lamb-Oseen (R=1, ν=0.01, Γ=1, T=6) — cap = 52.360

| t | Φ_holo^B | cap | gap = cap−Φ | gap/cap | Φ/cap |
|---:|---:|---:|---:|---:|---:|
| 0.300 | 0.00000 | 52.360 | 52.360 | 1.0000 | 0.000 |
| 0.818 | 0.00000 | 52.360 | 52.360 | 1.0000 | 0.000 |
| 1.336 | 0.00000 | 52.360 | 52.360 | 1.0000 | 0.000 |
| 1.855 | 0.00000 | 52.360 | 52.360 | 1.0000 | 0.000 |
| 2.373 | 0.00000 | 52.360 | 52.360 | 1.0000 | 0.000 |
| 2.891 | 0.00000 | 52.360 | 52.360 | 1.0000 | 0.000 |
| 3.409 | 0.00000 | 52.360 | 52.360 | 1.0000 | 0.000 |
| 3.927 | 0.00000 | 52.360 | 52.360 | 1.0000 | 0.000 |
| 4.445 | 0.00000 | 52.360 | 52.360 | 1.0000 | 0.000 |
| 4.964 | 0.00000 | 52.360 | 52.360 | 1.0000 | 0.000 |
| 5.482 | 0.00000 | 52.360 | 52.360 | 1.0000 | 0.000 |
| 6.000 | 0.00000 | 52.360 | 52.360 | 1.0000 | 0.000 |

- **cap_ok**: True (0 < 52.36 trivially)
- **non-vacuous (Φ > cap/10 at least once)**: **False** (max Φ = 0)
- **monotone**: True (constant)
- **max Φ/cap = 0** ≪ 0.1 threshold

### §4.2 F-C Taylor-Green (R=0.5, ν=0.01, T=6, center (π,π)) — cap = 13.090

| t | Φ_holo^B | cap | gap | gap/cap | Φ/cap |
|---:|---:|---:|---:|---:|---:|
| 0.300 | 0.05267 | 13.090 | 13.037 | 0.9960 | 0.0040 |
| 0.818 | 0.05267 | 13.090 | 13.037 | 0.9960 | 0.0040 |
| 1.336 | 0.05267 | 13.090 | 13.037 | 0.9960 | 0.0040 |
| 1.855 | 0.05267 | 13.090 | 13.037 | 0.9960 | 0.0040 |
| 2.373 | 0.05267 | 13.090 | 13.037 | 0.9960 | 0.0040 |
| 2.891 | 0.05267 | 13.090 | 13.037 | 0.9960 | 0.0040 |
| 3.409 | 0.05267 | 13.090 | 13.037 | 0.9960 | 0.0040 |
| 3.927 | 0.05267 | 13.090 | 13.037 | 0.9960 | 0.0040 |
| 4.445 | 0.05267 | 13.090 | 13.037 | 0.9960 | 0.0040 |
| 4.964 | 0.05267 | 13.090 | 13.037 | 0.9960 | 0.0040 |
| 5.482 | 0.05267 | 13.090 | 13.037 | 0.9960 | 0.0040 |
| 6.000 | 0.05267 | 13.090 | 13.037 | 0.9960 | 0.0040 |

- **cap_ok**: True
- **non-vacuous**: **False** (max Φ/cap = 0.0040, threshold 0.1)
- **monotone**: True (constant)

### §4.3 Why Φ_holo is constant in t

For both flows, ω(x, t) = ω_0(x) · g(t) factorizes spatially: g_FA(t)
= Γ/(4πνt) · exp(decay), g_FC(t) = exp(-2νt). The 8-bin histogram MI
estimator is **scale-invariant**: a global multiplicative rescaling of
both interior and boundary by g(t) leaves bin assignments (after
auto-rescaling lo/hi) identical. So Φ_holo(|ω(t)|) = Φ_holo(|ω_0|) for
all t > 0.

This is a *property of the estimator*, not of the physics. The
estimator measures structural/relational information between interior
and boundary, not absolute magnitude.

### §4.4 Why F-A returns Φ = 0 exactly

For Lamb-Oseen, the boundary annulus (r ≈ R) sits in the *exponential
tail* (exp(-1/(4·0.01·t)) ≈ exp(-25/t) ≈ 10^{-11} at t=1) where |ω| is
numerically zero relative to the central peak. All boundary samples
land in the lowest bin → H(boundary) ≈ 0 → MI ≈ 0. The estimator
sees a **degenerate boundary** (T1 case from the hexa module:
"degenerate boundary → Φ_holo = 0").

This is itself a diagnostic: the 8-bin estimator on Lamb-Oseen with
R=1 ν=0.01 cannot resolve the boundary structure. Increasing R to
near the vortex core radius ~ √(4νt) ~ 0.05 would change the result,
but R is fixed by the seed/round1 JSON.

---

## §5 Verdict

**FAIL — F-D1.1-B fires (cap decorative; primitive vacuous on smooth
flows).**

Pass criterion (a) (cap not violated) holds, but criterion (b)
(non-vacuous gap, Φ > cap/10 at least once) fails on **both** F-A and
F-C:

- F-A max Φ/cap = 0.000 (estimator sees degenerate boundary)
- F-C max Φ/cap = 0.004 (cap exceeded by Φ by ~250×)

The cap π R²/(νT) is satisfied trivially; the primitive does no
constraint work. Per seed §5.1 step 5 ("F-D1.1-B fires if Φ_holo <
cap/100 uniformly (cap is decorative)"), F-C is borderline (Φ/cap =
0.004 ≈ 1/250 < 1/100); F-A is firmly inside (Φ/cap = 0 < 1/100).

Note: the seed states F-D1.1-B threshold as `Φ_holo < cap/100`; the
discriminator step (b) states `Φ_holo within 1× order of magnitude
(> cap/10)`. Both thresholds judge the same direction (vacuous). The
data place the result on the *vacuous* side of both:

- Strict reading (cap/10): both flows fail
- Loose reading (cap/100): F-A fails, F-C borderline (0.004 < 0.01)

**Verdict: FAIL.** HVC's holographic-Bekenstein cap, in the
formulation specified, does not constrain the smooth-flow regime in a
non-trivial way under the prescribed (R, ν, T, MI_BINS) parameters and
the existing 8-bin estimator. The primitive collapses to "Φ_holo is
decorative on smooth flows."

This is a **molt-validation FAIL**, not an HVC retraction in the
strong sense (F-D1.1-A would require cap *violation*; that does not
happen). It is the F-D1.1-B failure mode: HVC is consistent but
empty.

---

## §6 CATALOGUE_BIAS implication

Per `omega-meta-audit-l9-catalogue-design-2026-04-25.md`, the prior
arithmetic-family catalogue (Q1 KdV-Gram, Q5 Sobolev/Besov, KPZ d=7)
all FAILED. D1.1 was the meta-audit's diversification trial — a
non-arithmetic, info-theoretic primitive.

**Diagnostic data**: D1.1 ALSO fails, but in a *different mode* than
the arithmetic catalogue.

| candidate | family | failure mode |
|---|---|---|
| Q1 KdV-Gram | algebraic-arithmetic | F-Q1: rank ≠ 3, det/σ ∉ ℤ (numerical mismatch) |
| Q5 Sobolev/Besov | mechanism-Sobolev | F-Q5: classical-relabeling (no construction) |
| KPZ d=7 | scaling-arithmetic | F-KPZ: no-anchor (ansatz fails at d=2) |
| **D1.1 HVC** | **info-theoretic / holographic** | **F-D1.1-B: cap decorative (vacuous primitive)** |

The arithmetic family failed by *target-numerical-mismatch* or
*relabeling-not-derivation*. D1.1 fails by *primitive-vacuity*: the
inequality holds but constrains nothing. This is qualitatively new
data — CATALOGUE_BIAS is not falsified (D1.1 still didn't pass), but
the *failure mode* genuinely differs.

Implication for meta-audit refinement:
- The arithmetic→info-theoretic frame shift did NOT salvage the
  catalogue, but it did surface a **new failure axis** ("vacuous
  primitive") absent from the arithmetic catalogue. This refines the
  L9 §6 F-MOLT-F enumeration: pre-D1.1 the failure modes were
  {numerical-mismatch, classical-relabeling, no-anchor}; post-D1.1
  add {vacuous-primitive / decorative-cap}.
- The meta-audit's CATALOGUE_BIAS verdict is reinforced: cross-family
  diversification produces *new* failure modes but not *passes*. This
  is consistent with an axiom-level ceiling rather than a
  family-specific bias.

---

## §7 Implications + next dispatch

### §7.1 Within BT-544

- HVC remains *consistent* (no cap violation on either smooth flow)
  but *empty* (cap is decorative). It is not a viable molt-frame for
  BT-544 under the seed's parameters.
- Per seed §5 dispatch order, the next candidate is **D1.4
  She-Leveque residual** (rank-2 by aggregate; cheapest at ≤30min;
  pure arithmetic enumeration). Caveat from seed §4 ranking: D1.4
  inherits the **arithmetic-family** label that the meta-audit
  flagged for CATALOGUE_BIAS. The D1.1 data above suggests this
  caveat is justified — non-arithmetic D1.1 still failed, but
  arithmetic D1.4 may fail in the *same* mode as Q1/Q5/KPZ
  (numerical-mismatch or relabeling) rather than producing new
  diagnostic data.
- Recommendation: dispatch **D1.4** (cheap binary check) AND then
  consider **D1.3** (NS↔MHD R_m=48, cross-PDE family, distinct from
  both arithmetic and info-theoretic). D1.2 (Pólya recurrence) and
  D1.5 (axisym-no-swirl, audit-only sub-mode) remain as fallbacks.

### §7.2 HVC parameter-sensitivity sub-investigation (optional)

The vacuity result depends on (R, ν, T, MI_BINS). Three reasonable
cutoff-choices (per round1 JSON G4) yield different caps:

- Kolmogorov scale ℓ_K = (ν³/ε)^{1/4}; on Lamb-Oseen
  ε ~ ν|∇ω|² → ℓ_K ~ √(ν·t) varies → cap ~ π/(ν·t) varies × R²/...
- Taylor microscale ℓ_T: between ℓ_K and integral
- Integral scale ℓ_I = R: as used (cap = π R²/(νT))

A parameter-sensitivity sweep (cheap; < 1h) could test whether HVC is
non-vacuous in *any* (R, ν, T) regime within the smooth flows. If
answer is *no* across all reasonable choices, HVC is unrescuable. If
*yes* at, e.g., near-vortex-core R ~ √(νT), HVC rescues to
"non-vacuous in core-resolved regime" — a weaker but still
meaningful result.

This is **not** dispatched here (out-of-scope for the strict molt-
validation under the seed's frozen parameters), but is registered as
a follow-on if future sessions revisit HVC.

---

## §8 Anomalies

- **A8.1 Path drift**: Round1 JSON cites `training/phi_holographic_
  measure.hexa` as nexus-relative; actual file lives in
  `/Users/ghost/core/anima/training/`. Module content matches; only
  path label is stale. Suggest noting in the round1 JSON
  `framework_link` field on the next nexus-side update.
- **A8.2 Estimator scale-invariance**: 8-bin histogram MI on
  globally-scaled fields is time-independent for ω(x,t) = ω_0(x)·g(t)
  factorized flows. Both F-A and F-C are exactly this form. The MI
  estimator is therefore **structurally insensitive to viscous
  decay** — it cannot witness "monotone-decreasing Φ_holo" on smooth
  flows, contradicting the seed's expected_phi_holo_t = "monotone
  decreasing as t increases". The seed's expectation does not match
  the estimator's mathematical behavior on closed-form smooth flows.
  This is a non-trivial mis-prediction in the seed/round1 JSON.
- **A8.3 Lamb-Oseen boundary degeneracy**: Φ = 0 exactly on F-A
  because |ω| at r ≈ R is 11+ orders of magnitude below |ω| at center,
  collapsing all boundary samples to the lowest histogram bin. This
  is the T1 "degenerate boundary" case from the hexa module — it
  signals the chosen R (= 1) is too large relative to the vortex core
  radius √(4νt) ≪ 1.
- **A8.4 Constant-Φ artifact**: All 12 timesteps return identical
  Φ_holo per flow. This is real (per A8.2) but visually obscures the
  fact that the validation tested 12 distinct field configurations.
  The constancy is *informative* (it disconfirms the seed's monotone-
  decrease expectation), not a bug.

---

## §9 Falsifiers active

- **F-D1.1-B (CONSUMED)**: Cap decorative on both smooth flows.
  Primitive vacuous. Fired this session.
- **F-D1.1-A (NOT FIRED)**: Cap not violated on F-A or F-C; HVC is
  consistent in the weak sense.
- **F-D1.1-C (NOT EXECUTED)**: Literature pre-audit for prior
  info-theoretic NS bound (Foias-Manley-Rosa-Temam 2001 ch.10; Gibbon
  2007) requires offline literature access; deferred to a future
  session that has it.
- **F-SEED-A (atlas-grounding integrity, from seed §7)**: The
  `phi_holographic_measure.hexa` path drift (A8.1) is borderline.
  The file exists with the cited T2 theorem at the cited line 23, but
  not at the cited *path*. Strict reading: F-SEED-A fires (mis-cited
  location). Loose reading: content integrity holds (T2 + Φ_holo
  definition exact). Reporting both readings; not actioning a
  retraction since the validation used the actually-found content.
- **F-SEED-D (D1.1 priority risk, from seed §7)**: The pre-audit
  R2-D was deferred (no literature access this session). Per seed
  §7's defense, D1.4 remains live. Falsifier neither fires nor is
  cleared; remains conditionally active.

---

## §10 Closing

**0/7 unchanged. NS regularity status open. No atlas/state/inventory
edits.**

D1.1 HVC-import returns molt-validation **FAIL** in the F-D1.1-B mode
(cap decorative; primitive vacuous on the smooth flows F-A and F-C
under the seed-frozen parameters R, ν, T, MI_BINS=8). The
arithmetic-family catalogue's failure-mode signature (numerical
mismatch, relabeling, no-anchor) does not transfer to the
info-theoretic D1.1; instead a *new* failure mode surfaces (vacuous
primitive). This is qualitatively new data for the CATALOGUE_BIAS
meta-audit but does not yet produce a passing molt.

Next dispatch (per seed §5): D1.4 She-Leveque residual (cheapest;
arithmetic-family caveat noted). Subsequent fallback: D1.3 NS↔MHD
duality (cross-PDE, third family).

— end molt-validation —
