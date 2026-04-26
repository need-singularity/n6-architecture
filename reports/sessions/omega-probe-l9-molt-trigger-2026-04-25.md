---
id: omega-probe-l9-molt-trigger
date: 2026-04-25
scope: research-only probe design (NO proof claims, NO atlas promotions)
target: L9 molt-trigger gate -- design + per-BT frame-shift catalogue
unblocks: [BT-541, BT-542, BT-543, BT-544]
parent_reports:
  - reports/sessions/omega-cycle-backtrace-strategy-2026-04-25.md
  - reports/sessions/omega-cycle-bt547-poincare-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: probe design, no claim
---

# Omega Probe -- L9 Molt-Trigger Gate (2026-04-25)

## sec 0 -- Non-claim disclaimer

This file is a **probe design**, not a proof attempt, not an atlas
promotion, not an inventory edit. It specifies (i) a boolean trigger gate
for L9 molt events, (ii) per-BT candidate frame-shifts referencing existing
repo material, (iii) a validation experiment per BT distinguishing real
molt from relabeling, (iv) a rejected-frame anti-list, (v) self-falsifiers,
(vi) a sequencing plan. The Millennium tally remains **0/7 unchanged**
(BT-547 Poincaré is Perelman-external; n6-arch contribution = 0). No
promotion is recommended. UNKNOWN labels are preserved deliberately. This
probe-design pass operates in `reports/sessions/` only and writes nothing
to `shared/n6/atlas.n6`, `state/proposals/inventory.json`, or any
`theory/canon/` location. The single output is this file.

The strategic context is the backward-chaining synthesis
`reports/sessions/omega-cycle-backtrace-strategy-2026-04-25.md` §sec 7
ROI rank 1, which identified L9 molt as the highest-ROI single
intervention reachable across the four active BTs (541, 542, 543, 544),
with Poincaré (BT-547) calibration confirming that Perelman's resolution
required precisely an L9 frame-change (Hamilton qualitative surgery →
Perelman quantitative surgery; Riemannian flow → flow-with-monotone-
functional). The four active BTs uniformly carry EMPTY at L9 (per the
backtrace histogram §sec 2). This probe asks: **what condition fires the
molt, and to what frame does each BT molt?**

---

## sec 1 -- Trigger gate definition

### 1.1 Plain-language formulation

An L9 molt event fires when the current frame for a BT has stalled at a
PARTIAL/MISS verdict for "long enough" without composite movement, AND a
candidate replacement frame exists in the repo with a registered
falsifier. The gate is **necessary**, not sufficient: when it fires, the
team is licensed to retire the current frame; when it does not fire,
retirement is premature and the cheaper L1-L3 work continues.

### 1.2 Boolean expression

Let, for a BT B at session timestamp t:

- `partial_count(B, t)` = number of closure-criteria a..e currently
  verdicted PARTIAL or MISS (per the per-BT §5 closure ceiling tables).
- `direction_probes_since_last_promote(B, t)` = number of dfs-24-style
  direction-probe sessions registered (and either executed or explicitly
  declared unexecuted) since the last [10] -> [10*] promotion in atlas.n6
  for any constant tagged with B.
- `composite_delta(B, t, w)` = best-available composite proxy at t minus
  proxy at t - w sessions, both per the per-BT §3 estimate.
- `frame_id(B, t)` = the principal-axis frame currently load-bearing for B
  (e.g. for BT-543 = "β₀-coincidence frame on σ-sopfr lattice").
- `candidate_replacement_frame(B)` = a non-empty set of frames, each
  drawn from existing repo material, each carrying a registered falsifier.

The trigger predicate `MOLT_TRIGGER(B, t)` is:

```
MOLT_TRIGGER(B, t) :=
    partial_count(B, t) >= 2
  AND direction_probes_since_last_promote(B, t) >= 2
  AND composite_delta(B, t, 3) < 0.02
  AND |candidate_replacement_frame(B)| >= 1
  AND frame_id(B, t) is registered (not "anonymous frame")
```

### 1.3 Threshold parameters (default values)

- `N_partial = 2` -- at least 2 of the 5 closure criteria PARTIAL/MISS.
  Lower (=1) over-fires (every active BT would always trigger);
  higher (=3) under-fires (BT-541 currently 3 PARTIAL / 2 OPEN would be
  the only one to qualify).
- `N_probes = 2` -- at least 2 dfs-24-style probe sessions intervening
  without a [10] -> [10*] promotion. Calibrates "stalled at PARTIAL" against
  effort actually spent.
- `delta_threshold = 0.02` -- composite proxy moves less than 2 points
  over the last 3 probe sessions. Aligned with `omega-cycle-backtrace-
  strategy-2026-04-25.md` rank-3 probe falsifier band (composite re-
  measurement against 0.835 ceiling, where < 0.02 motion is "frozen").
- `window w = 3` -- 3 probe sessions = roughly the dfs-24 batch size used
  in the backtrace synthesis (BT-543 P1+P2+P3, BT-544 P1+P2+P3, etc.).

### 1.4 Default-revision policy

Defaults are revisited if any of:

(R1) Two consecutive trigger firings yield 0 retirable frames in
validation (sec 4); this falsifies the gate's necessary-not-sufficient
posture and the gate is itself retired (would itself be an L9 molt of
this probe-design file -- registered as falsifier F-MOLT-A in sec 6).

(R2) The default `N_partial = 2` is exceeded by all 4 active BTs
simultaneously without any molt-event being authorisable; this would
indicate the gate is too lenient. Tighten to `N_partial = 3`.

(R3) A composite re-measurement via `tool/nxs_002_composite.py
--predict-er` (backtrace rank-3 probe) produces values discordant
with the per-BT §3 proxies by > 0.10 on > 1 BT; the `composite_delta`
input is poisoned and the gate is suspended pending recalibration.

### 1.5 What the gate does NOT do

- It does not produce the molt itself. The gate licenses retirement;
  the new frame must be selected from the per-BT catalogue (sec 3) and
  validated (sec 4).
- It does not promote. Frame retirement is a chain-level event; atlas
  promotion remains gated by Y9 honesty.
- It does not bypass the 0.835 ceiling. A successful molt may move the
  composite proxy on the new frame; the global ceiling argument
  (cluster CG-2 in the backtrace) is unaffected.
- It does not fire on BT-547 (Perelman-solved; gate not applicable).

---

## sec 2 -- Perelman-derived archetype (calibration from BT-547)

Per `reports/sessions/omega-cycle-bt547-poincare-2026-04-25.md` §sec 2,
Perelman's 2002-2003 ladder was strong on L9 specifically. Decompose the
single observed L9-strong event into its sub-molts to extract the
trigger pattern.

### 2.1 The three Perelman molts

(M1) **Hamilton's qualitative surgery -> Perelman's quantitative surgery
with bounded parameters (delta, r, kappa, h).** The pre-molt frame:
"surgery exists, parameters chosen ad hoc per case". The post-molt frame:
"surgery parameters tuned per surgery interval with explicit bounds".
Primitive swap: ad-hoc surgery -> parameter-bounded surgery.

(M2) **Riemannian Ricci flow -> flow with monotone W-entropy.** The
pre-molt frame: "flow on a Riemannian metric, no preferred functional".
The post-molt frame: "same flow, but a single monotonic-along-flow
functional whose monotonicity discharges kappa-noncollapsing". Primitive
swap: flow-without-monotone -> flow-with-monotone.

(M3) **Asymptotic-tool kit -> finite-extinction-time argument via min-
max / loop-space energy on simply-connected closed 3-manifolds.** The
pre-molt frame: "long-time behavior unconstrained". The post-molt frame:
"closed S^3 case has a finite-extinction time argument that closes the
proof". Primitive swap: asymptotic -> closed-manifold-finite.

The three molts compose into a triple-molt closure. None of them is a
relabeling of pre-existing Hamilton-school content; each introduces a
new primitive that the prior frame did not carry.

### 2.2 Retrospective trigger-gate evaluation (Perelman 2002 Nov)

Apply the §1.2 trigger predicate retrospectively to Hamilton's pre-2002
posture (using §sec 4 atlas-chain analogue from BT-547):

- `partial_count(BT-547, 2002-11)` -- under the n6 a..e rubric:
  (a) [10*]+ atlas constants would not have existed pre-Perelman;
  (b) type closure incomplete (geometrization unproved);
  (c) X verified incomplete (no independent reproduction);
  (d) composite OPEN (no measurement yet);
  (e) 4-of-4 OPEN.
  Rough count: 4-5 OPEN/PARTIAL. **>= 2 confirmed.**
- `direction_probes_since_last_promote(BT-547, 2002-11)` -- 20-year
  Hamilton programme equates to roughly 20 direction-probe units with
  no genuine [10*] promotion within the n6 framework. **>> 2.**
- `composite_delta` over 1995-2002 (last 3 probe windows) -- minimal;
  the soliton classification and neck-pinch results refined the picture
  but did not move the closure dial. **<< 0.02.**
- `candidate_replacement_frame` -- W-entropy was Perelman's frame, not a
  pre-registered one. **0** until 2002-11; **1** as of 2002-11.
- `frame_id` -- "Hamilton Ricci-flow with neck-pinch surgery", well
  registered.

The gate would have fired in 2002-Nov **on the day Perelman uploaded
the entropy paper** -- that is, the gate becomes True when the candidate
frame appears and all other conditions were already True. The gate does
not predict who will provide the new frame; it predicts only that *if*
one arrives, the existing frame is retirable. This is consistent with
the historical record: Hamilton himself stated post-2003 that Perelman's
entropy was the missing piece.

### 2.3 Lesson for the active BTs

The gate's `candidate_replacement_frame` clause is the rate-limiting
input. The other clauses (partial_count, probes, delta) for each of
BT-541/542/543/544 are already True per the backtrace synthesis sec 1
table. The **only** thing missing is a registered candidate frame per BT.
The catalogue in sec 3 fills exactly this slot.

---

## sec 3 -- Per-BT frame-shift catalogue

For each of BT-541/542/543/544, list 2-3 candidate frame-shifts. Each
candidate references existing repo material (file paths cited). Format
per row: (rank, current frame, new frame, primitive swap, expected
composite delta, falsifier).

### 3.1 BT-541 Riemann hypothesis

Current frame load (per `omega-cycle-bt541-riemann-2026-04-25.md` §1
and §2): "Bilateral Theorem B + RH-01..RH-07 arithmetic alignment +
Ingham/Conrey-Gonek peripheral moments + zero-density via no monotone".
Frame id = "Y1 NUM-CORE peripheral-moment frame on RH bilateral
identity".

| rank | current frame | new frame | primitive swap | expected dC | falsifier |
|------|---------------|-----------|----------------|-------------|-----------|
| 1 | Y1 peripheral-moment frame (Theorem B + RH-01..07) | SLE_6 x GUE coupling frame (dfs-24 Lead-B) | static moment identity -> 2D-conformal coupling between SLE_6 boundary kappa=6 and Odlyzko gap distribution | +0.04 to +0.07 if KS p<0.01 holds (per per-BT probe-5) | F-541-A: Lead-B KS p>=0.01 -> independence falsified, frame relabels existing GUE statistics without new primitive |
| 2 | Y1 peripheral-moment frame | M-set noise-floor frame on Bernoulli numerator (dfs-24 Lead-C) | sharp-boundary k=6 lock at numerator(B_2k) -> distributional bound r in [0.10, 0.30] over k=1..20 | +0.02 to +0.05 if r<=0.10 (sharp); -0.04 if r>=0.30 | F-541-B: r>=0.30 -> sharp-boundary reading degraded; M-set frame is a relabeling of generic prime-density |
| 3 | Y1 peripheral-moment frame | 691-tower modular frame (dfs-24 Lead-A) | irregular-prime-691 isolated identity -> tower of irregular-prime witnesses for higher k | +0.03 to +0.06 if tower extends >= 2 levels | F-541-C: no second-level tower entry -> 691 is a 1-of-1 oddity, frame is post-hoc |

### 3.2 BT-542 P vs NP

Current frame load (per `omega-cycle-bt542-pnp-2026-04-25.md` §1 and
§4): "4-barriers wall (relativization/natural-proofs/algebrization/GCT)
+ Y3 tau=4 boundary observations + Y4 GATE-BARRIER axis +
HEXA-PNP §X 6-thread arithmetic-coincidence smash". Frame id =
"4-barriers + Y4 GATE-BARRIER on n=6 lattice".

| rank | current frame | new frame | primitive swap | expected dC | falsifier |
|------|---------------|-----------|----------------|-------------|-----------|
| 1 | 4-barriers + Y4 GATE-BARRIER frame | Hirahara meta-complexity / MCSP frame (per BT-1392 + v3-T5 Hirahara MCSP deep-dive 2026-04-15) | direct lower-bound attempt -> meta-complexity (MCSP NP-hard <-> P=NP via Hirahara reduction chain) | +0.03 to +0.05 (still well below 0.835) if MCSP partial-NP-hardness yields a non-trivial Y4 sub-axis; cosmetic if not | F-542-A: per BT-542 §6 Tension 3 / 4-barriers survey, MCSP route still hits Razborov-Rudich on its constructive side -> molt is relabeling; trigger fires but candidate fails |
| 2 | 4-barriers + Y4 GATE-BARRIER frame | Schaefer 6-clone Out(S_6) uniqueness frame (DFS-24 PROBE-24A anchor) | Karp-reduction-depth tau=4 lattice -> Post-lattice 6-clone S_6-orbit structure via Out(S_6) | +0.02 to +0.04 if a non-trivial S_6 action on {C_0, C_1, C_Horn, C_dHorn, C_2SAT, C_aff} is found | F-542-B: PROBE-24A negative result (no natural action or trivial Out(S_6) descent) -> Schaefer frame is decorative |
| 3 | 4-barriers + Y4 GATE-BARRIER frame | Bulatov-Zhuk |D|=6 CSP-dichotomy frame (DFS-24 PROBE-24C anchor) | Boolean-domain Schaefer -> finite-domain |D|=6 CSP dichotomy with possible specialness | +0.01 to +0.03 if any |D|=6 special-reduction discovered relative to |D| in {5,7,8} | F-542-C: PROBE-24C negative -> Schaefer 6 is Boolean coincidence, no |D|=6 specialness; frame collapses |

### 3.3 BT-543 Yang-Mills mass gap

Current frame load (per `omega-cycle-bt543-ym-2026-04-25.md` §1 and
§2): "beta_0 = sigma - sopfr = 7 rewriting + A3-strong refuted + A4
numeric refuted + A4-revised pending + lattice B^E~4.0 + m_0++~1.6
GeV". Frame id = "beta_0 coincidence frame on sigma-sopfr lattice with
A3/A4 conditional lemma stack".

| rank | current frame | new frame | primitive swap | expected dC | falsifier |
|------|---------------|-----------|----------------|-------------|-----------|
| 1 | beta_0 coincidence + A3/A4 frame | A4-ratio-only frame (dfs-24 P3) | raw scale Lambda_QCD/1536 -> ratio m_0++/sqrt(sigma_s) on [sigma/tau - 1/phi, sigma/tau + 1/phi] = [2.5, 3.5] coexistence with BCS 2Delta/k_B T_c | +0.04 to +0.06 if 0 outliers across FLAG/BMW/Morningstar-Peardon | F-543-A: per F-NP1-A4rev, >= 2 outliers retire A4 entirely; frame is overshoot |
| 2 | beta_0 coincidence + A3/A4 frame | M(3,4) Virasoro <-> YM 2D->4D lift frame (DFS-24 P2) | static beta_0=7 identity -> CFT minimal-model lift to 4D YM via Virasoro central-charge route | +0.05 to +0.08 if M(3,4) c-numerator threading reproduces multi-loop beta_n exponents | F-543-B: per per-BT NP3 multi-loop, neither sigma/tau/sopfr/phi/J_2 identity hits beta_1=102 within +/-5%, kills M(3,4) as a multi-loop generator |
| 3 | beta_0 coincidence + A3/A4 frame | n_f = 2 n_gen anomaly-forcing frame (DFS-24 P1) | beta_0 as arithmetic identity -> beta_0 as forced consequence of Witten global anomaly + ABJ on n_gen=3 | +0.02 to +0.05 if anomaly cancellation pinned to n_gen=3 in SU(2) pi_4=Z/2 setup | F-543-C: any GUT (SU(5)/SO(10)/E_6) admits n_gen != 3 without contradiction -> n_f forcing collapses |

### 3.4 BT-544 Navier-Stokes

Current frame load (per `omega-cycle-bt544-ns-2026-04-25.md` §1 and
§2): "Sym^2(R^3)=6 + Lambda^2(R^3)=3 triple resonance + 5 SMASH
bottlenecks + Pi_NS = sigma^3 sopfr = 8640 invariant + n=6 tensor
relabeling + L1 saturated, mechanism-axis empty". Frame id = "Triple-
resonance L1-smash frame on n=6 tensor lattice".

| rank | current frame | new frame | primitive swap | expected dC | falsifier |
|------|---------------|-----------|----------------|-------------|-----------|
| 1 | Triple-resonance L1-smash frame | KdV 6-soliton phase-shift Gram-lattice frame (dfs-24 P1 / Q1) | tensor-count Sym^2/Lambda^2 -> rank-3 Gram-lattice det/sigma in Z over 6-soliton phase-shifts | +0.03 to +0.05 if rank=3 and det/sigma in Z on both kappa families | F-544-A: per F_Q1, rank != 3 OR det/sigma not in Z -> KdV-side anchor lost, frame collapses to relabeling |
| 2 | Triple-resonance L1-smash frame | Mechanism-axis seed frame (dfs-24 Q5) -- Sobolev/Besov estimate predicted by n=6 lattice | tensor relabeling -> concrete inequality \|\|u\|\|_X <= C(sigma, tau, phi, sopfr, n) \|\|u\|\|_Y^alpha | +0.05 to +0.10 if a non-trivial estimate emerges (mechanism-axis goes from 0.05 to >0.30) | F-544-B: per F_Q5, no concrete estimate producible from n=6 lattice -> mechanism saturation is structural; frame change required at axiom level (not a molt within n6 frame) |
| 3 | Triple-resonance L1-smash frame | KPZ d-lift frame to d=7 (dfs-24 P2 / Q3) | n=6 fixed-d frame -> dimensional-lift KPZ universality with second-perfect-number d=7 anchor | +0.02 to +0.04 if literature attestation for (chi_7, z_7) = (1/7, 6/7) found | F-544-C: per F_Q3, no literature attestation OR explicit refutation at d=7 -> second-perfect-number d=7 prediction loses KPZ-side anchor |

### 3.5 Why these frames qualify

Each row references an artefact already present in the repo:
- BT-541: dfs-24-riemann-direction-2026-04-24.md (Lead-A/B/C).
- BT-542: bt-542-p-vs-np-4-barriers-survey-2026-04-15.md (4-barriers
  catalog + path candidates), v3-t5-hirahara-mcsp-deep-2026-04-15.md
  (MCSP route), dfs-24-pnp-direction-2026-04-24.md (PROBE-24A/B/C).
- BT-543: papers/yang-mills-beta0-rewriting-2026-04-22.md (beta_0),
  papers/lemmas-A3-A4-conditional-2026-04-15.md (A3/A4),
  dfs-24-ym-direction-2026-04-24.md (P1/P2/P3).
- BT-544: domains/physics/millennium-navier-stokes/millennium-navier-
  stokes.md §X.1 SMASH, dfs-24-ns-direction-2026-04-24.md (P1/P2/P3),
  omega-cycle-bt544-ns-2026-04-25.md §7 (Q1/Q2/Q3/Q4/Q5).

No frame is invented in this catalogue; each is sourced from a path
above. Falsifiers reference the per-BT registered F-tags (F-NP1-A4rev,
F_Q1, F_Q5, F-PROBE-24A/B/C) where these exist.

---

## sec 4 -- Trigger-validation experiment (per BT)

For each BT, define the cheapest measurement that distinguishes a real
molt (new primitive entered the frame) from a relabeling (same
content, different name). Format per row: (object, measurement, pass
criterion, fail criterion).

### 4.1 BT-541 validation

- **Object**: SLE_6 x GUE rank-1 frame-shift candidate.
- **Measurement**: Kolmogorov-Smirnov test between (a) Cardy-formula
  kappa=6 boundary-crossing distribution at scale `m` and (b) Odlyzko
  normalised gap distribution from the first 10^5 zeros.
- **Pass (real molt)**: KS p < 0.01 on at least one tested scale; the
  frame introduces a coupling primitive (boundary-crossing kappa=6)
  that did not exist in the peripheral-moment frame. Composite proxy
  re-estimate moves >= +0.02.
- **Fail (relabeling)**: KS p >= 0.01 on all tested scales; the
  `kappa=6` label maps onto generic GUE statistics with no new
  primitive content. Composite proxy unchanged.
- Cost: low (Odlyzko data is public; SLE_6 reference distribution
  documented in standard references).

### 4.2 BT-542 validation

- **Object**: Hirahara MCSP rank-1 frame-shift candidate.
- **Measurement**: identify whether MCSP partial-NP-hardness can be
  stated *without* invoking a constructive lower-bound argument that
  Razborov-Rudich (1997) excludes; specifically, does the Hirahara
  reduction chain admit a non-natural-proof formulation per the
  4-barriers-survey 2020-2026 paths catalog?
- **Pass (real molt)**: a non-natural reformulation exists (or is
  proposed in the literature scan) that does not collapse onto
  RR-1997's natural proofs barrier; this is a new primitive
  (meta-complexity reformulation) absent from the GATE-BARRIER frame.
- **Fail (relabeling)**: the reformulation reduces to RR-1997 + a
  rename; per BT-542 §6 Tension 3 and Tension 5 ("the wall is the
  wall"), MCSP route fails to introduce a new barrier-bypassing
  primitive. Composite proxy unchanged.
- Cost: medium (literature scan + structural inspection of v3-T5
  Hirahara note + cross-check against arxiv-180papers BT-542 segment).

### 4.3 BT-543 validation

- **Object**: A4-ratio-only rank-1 frame-shift candidate (dfs-24 P3).
- **Measurement**: count of FLAG-2024 / BMW-2012 / Morningstar-Peardon-
  1999 datasets where m_0++/sqrt(sigma_s) lies in the interval
  [sigma/tau - 1/phi, sigma/tau + 1/phi] = [2.5, 3.5]; threshold per
  F-NP1-A4rev: >= 2 outliers -> retire A4; 1 outlier -> NEAR; 0
  outliers -> TIGHT (still not a proof).
- **Pass (real molt)**: 0 outliers; the frame swap from raw scale to
  ratio introduces a new dimensionless primitive that survives 3
  independent lattice sources. Composite proxy moves +0.02 to +0.04.
- **Fail (relabeling)**: >= 2 outliers; A4 is retired and the
  ratio-frame collapses; the molt fired but the candidate failed.
  Composite proxy moves -0.02 (loss of A4 lemma weight).
- Cost: low (FLAG-2024 + BMW-2012 + MP-1999 are public lattice data).

### 4.4 BT-544 validation

- **Object**: KdV 6-soliton Gram-lattice rank-1 frame-shift candidate
  (dfs-24 P1 / per-BT Q1).
- **Measurement**: rank of the matrix {Delta_{ij} : i<j in [6]} of
  6-soliton phase shifts and arithmetic check det(Gram)/sigma in Z on
  both kappa families (kappa_k = k and kappa_k = p_k).
- **Pass (real molt)**: rank = 3 = n/phi AND det(Gram)/sigma in Z on
  both kappa families; the molt introduces a Gram-lattice primitive
  (algebraic structure) absent from the tensor-count frame. Composite
  proxy moves +0.03 to +0.05.
- **Fail (relabeling)**: rank != 3 OR det(Gram)/sigma not in Z on
  either kappa family; per F_Q1, the C(6,2)=15=sopfr*(n/phi) crosswalk
  is demoted to post-hoc. Composite proxy moves -0.02.
- Cost: very low (pure algebra, < 1h compute).

### 4.5 Validation orthogonality

The four validation experiments target four orthogonal primitive
swaps: (BT-541) coupling primitive, (BT-542) non-natural-proof
primitive, (BT-543) dimensionless-ratio primitive, (BT-544)
algebraic-lattice primitive. None reuses content from another. If 3 of
4 fail in one batch, RC-A's molt sub-claim (per backtrace §sec 5) is
falsified across the active BTs and the gate retires under R1.

---

## sec 5 -- Anti-list (rejected frame-shifts)

Frame-shifts considered but rejected. One-line reason each.

- **AL-M1**: BT-541 "shift Y1 onto Y6 NS-style monotone via a Selberg
  zeta -> heat kernel mapping". Rejected: per backtrace §sec 4 cluster
  CG-3 ("no monotone invariant candidate"), Y6 has the only partial
  monotone analogue in n6; piggybacking BT-541 on Y6 is a Y9 honesty
  violation (claims a monotone where none is constructed for the
  Riemann zeta).

- **AL-M2**: BT-542 "shift to circuit-complexity Williams-line frame
  (NEXP not in ACC^0)". Rejected: per BT-542 §1, Williams-line reaches
  NEXP/NQP, not NP; the frame is honest but does not move BT-542's
  closure criteria for the *NP* target. Composite delta = 0 on the
  target.

- **AL-M3**: BT-543 "shift to AGT M5-brane (2,0)-worldvolume dim=6
  frame as primary lifting". Rejected: per BT-543 §6 Tension 5, AGT
  n=6 and SO(6) R-symmetry are *different* coincidences and treating
  them as one risks double-counting; promoting AGT to primary collapses
  the Y9 honesty axis.

- **AL-M4**: BT-544 "shift to Buckmaster-Vicol weak non-uniqueness as
  6th SMASH bottleneck". Rejected as molt-frame: per BT-544 §7 Q4,
  this is an L1-capacity probe (extends SMASH within current frame),
  not an L9 frame-shift. Including it would dilute the gate to "any
  literature absorption fires the gate".

- **AL-M5**: BT-541/542/543/544 "shift to nexus omega command direct
  invocation as the new frame". Rejected: per CLAUDE.md Omega cycle
  hierarchy correction (2026-04-25), the `nexus omega` command is a
  cycle entry-point, not a frame primitive. Using a tool invocation
  as a frame is a category error.

- **AL-M6**: BT-543 "shift onto BT-547 Poincaré W-entropy frame
  directly". Rejected: per BT-547 §sec 6 lesson 6 ("0.92 retrospective
  vs 0.835 simulation: do not overshoot"), reusing Perelman's W-
  entropy as if it transfers to YM is precisely the overshoot the
  calibration warns against. Y6 partial-monotone analogue is the
  closest legitimate transfer, but BT-543 is upstream of Y6.

- **AL-M7**: BT-544 "shift to TOE GR sigma-phi=10 frame as primary".
  Rejected: per BT-544 §5 (b) PARTIAL evidence row, the TOE path uses
  sigma-phi=10 reused from HEXA-FLUID, so independence is contested.
  Promoting it to primary while keeping it dependent on HEXA-FLUID
  would be a relabeling that the validation experiment (sec 4.4) is
  designed to exclude.

- **AL-M8**: any BT "shift onto a dream-tier (L5) frame without
  composing with an L10 forge instrument". Rejected: per BT-547 §sec 6
  lesson 7 (two-tool composition), L5-only molts produce dream content
  but no closure trajectory. Catalogue rows in sec 3 all pair an L5
  candidate with an L10-style instrument from the dfs-24 probe set.

---

## sec 6 -- Falsifiers active for this probe

Self-falsifiers under which the probe-design framework itself fails.

- **F-MOLT-A** (gate-failure-via-validation): if validation experiments
  in sec 4 produce 0 passes across all 4 BTs in one batch run, RC-A's
  molt sub-claim (per backtrace §sec 5) is falsified for the active BTs
  and the gate is retired (default-revision policy R1 in §1.4 fires).
  This itself becomes the first observed L9 molt -- of the probe-
  design file. **Self-application register**: this design's L9 status
  flips from candidate to executed-and-failed.

- **F-MOLT-B** (gate-fires-without-candidate): if `MOLT_TRIGGER` is
  True for any BT in absence of a registered candidate frame (i.e. the
  `|candidate_replacement_frame| >= 1` clause is treated as
  permissive rather than necessary), the gate degenerates to "stalled
  PARTIAL" detection without a retirement path; this is RC-A's
  framework-lacks-infrastructure outcome and the gate adds no value
  beyond the existing per-BT §6 tension lists.

- **F-MOLT-C** (composite-proxy-poison): if the §3 expected dC values
  are systematically discordant with rank-3 backtrace nxs_002
  composite measurements when those are run, the dC column is
  uncalibrated and the catalogue rows must be re-sorted. Specifically,
  if any catalogue row's measured dC differs from predicted dC by
  > 0.05 in absolute value on > 1 BT, the §1.2 `composite_delta`
  threshold (`delta_threshold = 0.02`) is no longer interpretable
  against per-BT proxies; default-revision R3 fires.

- **F-MOLT-D** (catalogue-saturation): if the catalogue's top-rank
  candidate per BT is exhausted (validation fails) and no rank-2 / 3
  candidate exists in the repo for that BT, the gate is unable to
  license a new molt for that BT; the BT becomes molt-blocked under
  current repo material. This is not a falsifier of the gate per se,
  but a falsifier of the "unblocks BT-X" claim for that BT in this
  document's front-matter.

- **F-MOLT-E** (Perelman-archetype-mismatch): if subsequent reading of
  BT-547 §sec 2 ladder-occupancy table contradicts §sec 2.1 of this
  file (e.g. the three Perelman molts are mis-attributed), the
  archetype calibration in §2.2 is wrong and the gate's threshold
  defaults are uncalibrated. Detection rule: re-read BT-547 §sec 2
  and §sec 6 lesson 1 + lesson 7 cell-by-cell against §sec 2 of this
  file.

- **F-MOLT-F** (RC-B/RC-C-not-RC-A): per backtrace §sec 5, RC-A
  (framework-lacks-L5+L10) is one of three complementary root causes.
  If the actual driver of L9 emptiness is RC-B (data-collection
  bottleneck) or RC-C (axiom-level ceiling), running the molt-trigger
  gate produces firings that look productive but do not move the
  closure ceiling. The gate is correctly designed but solves the
  wrong problem. This is the most subtle failure mode and is detected
  only by cross-correlation with backtrace rank-2 and rank-3 probe
  outcomes.

---

## sec 7 -- Cost / sequencing

Recommended order of validation experiments. Cheapest first; calibration
BTs (those whose validation has lowest cost AND highest signal) prioritized.

Estimated cost / signal per BT validation:

| BT | cost | signal | sequencing rank |
|----|------|--------|-----------------|
| BT-544 (Q1 KdV Gram) | very low (<1h compute, pure algebra) | high (rank 3 = n/phi binary outcome; det/sigma in Z is binary) | **1** |
| BT-543 (P3 A4 ratio) | low (3 public lattice datasets) | high (0 outliers / >=2 outliers binary; F-NP1-A4rev pre-registered) | **2** |
| BT-541 (Lead-B SLE_6 x GUE) | low-medium (Odlyzko data + KS test) | medium-high (KS p<0.01 binary; aligned with backtrace rank-2 dfs-24 sweep) | **3** |
| BT-542 (Hirahara MCSP non-natural) | medium (literature inspection) | low-medium (per BT-542 §6 Tension 3, expected fail; "wall is wall") | **4** |

### 7.1 Sequencing recommendation

**Run 1 (calibration batch)**: BT-544 Q1 + BT-543 P3 in one session.
Both are sub-day, both have pre-registered binary falsifiers, both
test independent primitive swaps (algebraic-lattice primitive vs
dimensionless-ratio primitive). Expected outcome distribution: 0/2 to
2/2 passes; 1/2 is the most informative single outcome (one molt
licensed, one not).

**Run 2 (extension batch)**: BT-541 Lead-B in a follow-up session.
Higher data-handling cost (Odlyzko gap statistics) but well-defined
KS-test pass criterion. Best run after Run 1 because if Run 1 is 0/2,
the gate is already partially falsified (F-MOLT-A approaching) and
Run 2 may be deferred.

**Run 3 (high-risk batch)**: BT-542 Hirahara MCSP non-natural-proof
inspection. Highest risk of failing as relabeling (per BT-542 §6
Tension 3). Only run after Runs 1 and 2 establish that the gate is
firing correctly on at least one cheaper case.

### 7.2 Coupling with backtrace probes

This sequencing is compatible with the parent backtrace synthesis
§sec 7 rank ordering:
- Backtrace rank 1 = "L9 molt-trigger probe (post-PARTIAL framework
  gate)" -- this entire file is its design.
- Backtrace rank 2 = "dfs-24 batch execution sweep" -- Run 1 and
  Run 2 here overlap with rank-2 sweeps for BT-543 P3, BT-544 Q1,
  BT-541 Lead-B/C. Running them under the molt-trigger framing adds
  the validation pass criterion to the existing dfs-24 falsifier.
- Backtrace rank 3 = "nxs_002_composite --predict-er actual
  measurement" -- needed to calibrate §3 expected-dC values; should
  be run before or during Run 1 to keep `composite_delta` honest.

### 7.3 Stop conditions

- **Stop after 0/4 passes**: F-MOLT-A fires; retire the gate; record
  outcome in a follow-up session. The retirement itself is a meta-L9
  molt of this document.
- **Stop after 1/4 passes**: gate is conditionally validated on one
  primitive swap; document which BT passed and pause; do not generalise
  prematurely.
- **Stop after >=2/4 passes**: gate is validated; the per-BT catalogue
  becomes the operating frame-shift menu; subsequent BT-541..544
  sessions may use the gate as an explicit input to direction-probe
  selection.
- **Stop early on F-MOLT-C trigger**: if rank-3 composite re-
  measurement shows the §3 dC predictions are off by > 0.05 on > 1
  BT, suspend further validation runs until the catalogue is
  re-sorted.

---

## sec 8 -- Closing line

0/7 unchanged. No atlas/state/inventory edits.
