---
id: omega-exec-bt544-fallback-molt-validation
date: 2026-04-25
scope: research-only molt-validation experiment (NO NS regularity claim, NO atlas promotion)
target: BT-544 fallback -- Q5 Sobolev/Besov (rank-2) + KPZ d=7 (rank-3) frame validation
parent_reports:
  - reports/sessions/omega-probe-l9-molt-trigger-2026-04-25.md (§ 4 BT-544 catalogue)
  - reports/sessions/omega-exec-bt544-q1-molt-validation-2026-04-25.md (prev FAIL)
  - reports/sessions/dfs-24-ns-direction-2026-04-24.md
  - reports/sessions/omega-cycle-bt544-ns-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: molt-validation experiment, no claim
---

# Omega Exec -- BT-544 Fallback Molt-Validation (Q5 + KPZ d=7) (2026-04-25)

## §0 Non-claim disclaimer

This report executes the **rank-2 (Q5 Sobolev/Besov) and rank-3 (KPZ
d=7) fallback molt-validations** for BT-544, after the rank-1 candidate
(Q1 KdV 6-soliton Gram-lattice) FAILED double-margin in
`omega-exec-bt544-q1-molt-validation-2026-04-25.md`.

**This document does NOT**:
- claim 3D NS regularity in either direction (smoothness or blow-up);
- promote anything in `shared/n6/atlas.n6`;
- modify `state/proposals/inventory.json`;
- modify `theory/canon/`;
- alter the `BT-544 = 0/1 untouched` Clay status;
- claim Q5 or KPZ d=7 outcomes bear on Clay closure either way.

**Millennium tally**: 0/7 unchanged. The Q5 / KPZ d=7 outcomes speak
**only** to whether the proposed BT-544 frame-shift candidates
introduce new primitives or are relabelings, not to NS regularity.

---

## §1 Specs extracted

### §1.1 Q5 (rank-2) -- Mechanism-axis Sobolev/Besov seed

Combined from `omega-probe-l9-molt-trigger-2026-04-25.md` §3.4 row 2
and `omega-cycle-bt544-ns-2026-04-25.md` §7 Q5 and
`dfs-24-ns-direction-2026-04-24.md` §3 (cited as "Q5 mechanism seed").

**Object**: Mechanism-axis seed frame (dfs-24 Q5) -- a Sobolev/Besov
estimate **predicted by the n=6 lattice**.

**Measurement**: produce one concrete inequality of the form

    ‖u‖_X ≤ C(σ, τ, φ, sopfr, n) · ‖u‖_Y^α

with **explicit (X, Y, α)** populated from the n=6 lattice constants
(σ=12, τ=4, φ=2, sopfr=5, n=6), where X, Y are Sobolev or Besov spaces
relevant to 3D NS regularity and α is a non-trivial exponent. The
estimate must be *predicted by* the n=6 lattice -- i.e. derivable from
the lattice constants alone, not merely *labelable* in n=6 vocabulary
after the fact.

**Pass criterion (real molt)** -- per L9 §3.4 row 2 expected ΔC and
F-544-B contrapositive: a non-trivial concrete estimate emerges, with
explicit constants and explicit Sobolev/Besov spaces, that is **not**
a relabeling of a pre-existing classical estimate (Sobolev embedding,
Gagliardo-Nirenberg, Ladyzhenskaya, Prodi-Serrin). Mechanism-axis
saturation moves from ~0.05 to >0.30 (per L9 §3.4 expected ΔC
+0.05..+0.10).

**Fail criterion (relabeling / no-construction)** -- per F-544-B
(L9 §3.4) and F_Q5 (omega-cycle-bt544-ns §7 Q5):

> "no concrete estimate producible from n=6 lattice -> mechanism
> saturation is structural; frame change required at axiom level
> (not a molt within n6 frame)"

i.e. either (a) no estimate is producible at all, or (b) every estimate
that the lattice can decorate already exists pre-n=6 (Sobolev 1938,
Gagliardo 1958, Nirenberg 1959, Ladyzhenskaya 1963, Serrin 1962,
Beale-Kato-Majda 1984), in which case the n=6 frame is a **labeling**
of pre-existing PDE infrastructure and not a new mechanism primitive.

**Cost**: one targeted derivation. Per Q5 entry "expected partial /
honest-no".

### §1.2 KPZ d=7 (rank-3) -- KPZ d-lift to second perfect number

Combined from `omega-probe-l9-molt-trigger-2026-04-25.md` §3.4 row 3
and `omega-cycle-bt544-ns-2026-04-25.md` §7 Q3 and
`dfs-24-ns-direction-2026-04-24.md` §3 P2.

**Object**: KPZ d-lift frame to d=7 (dfs-24 P2 / Q3).

**Measurement**: literature attestation for KPZ-class scaling exponents
(χ_d, z_d) = (1/d, 1-1/d) at d=7, specifically (χ_7, z_7) = (1/7, 6/7).
Two acceptable evidence sources per spec:

1. Frisch β-model extrapolation to substrate dimension d=7;
2. SHE (stochastic heat equation) / KPZ universality measurement at
   d=7 (or d≥3 with d=7 as a reachable case).

The "d=7 prediction" itself is the n=6 frame's claim that the second
perfect number d=7 = (one past n=6) reproduces (χ_7, z_7) = (1/7, 6/7)
under the n6-derived ansatz χ_d = 1/d, χ_d + z_d = μ = 1.

**Pass criterion (real molt)** -- per L9 §3.4 row 3 expected ΔC
+0.02..+0.04 and F-544-C contrapositive: at least one literature
attestation found in the repo or in mainstream reviewable literature
for (χ_7, z_7) = (1/7, 6/7) at d=7. The attestation must be specific
to d=7 (not a generic χ+z=1 relation).

**Fail criterion** -- per F-544-C (L9 §3.4) and F_Q3
(omega-cycle-bt544-ns §7 Q3) and F_P2 (dfs-24-ns §3 P2):

> "no literature attestation OR explicit refutation at d=7 -> second-
> perfect-number d=7 prediction loses KPZ-side anchor"

Note: Q5 is sequenced first per the user prompt; KPZ d=7 is
conditional on Q5 outcome.

---

## §2 Existence / tooling check

### §2.1 Q5 tooling

Q5 is a **derivation question**, not a numerical computation. No
script needed. Required inputs:
- n=6 lattice constants (σ=12, τ=4, φ=2, sopfr=5, n=6);
- Sobolev / Besov machinery available in repo (theory documents);
- catalogue of estimates the n=6 lattice has been said to "predict"
  (per BT-544 §X.1 SMASH, dfs-24-ns, breakthrough notes).

Repo scan for relevant material:
- `theory/study/p1/pure-p1-3-pde-navier-stokes.md` §5.2 -- 2D
  Ladyzhenskaya inequality (canonical, pre-n=6).
- `theory/study/p1/pure-p1-3-pde-navier-stokes.md` §5.3 -- explicit
  3D failure mode (canonical, pre-n=6).
- `theory/study/p3/prob-p3-2-conditional-theorems.md` §544-A --
  Prodi-Serrin condition `2/p + 3/q = 1` with p,q exponents
  (canonical, 1959/1962); n=6 lattice maps {2,3} = {φ, n/φ}.
- `domains/physics/millennium-navier-stokes/millennium-navier-stokes.md`
  §X.1 SMASH#2 -- Leray dissipation `H^{d/2} = H^{3/2} = H^{n/(φ·φ)}`
  (canonical 1934, n=6 relabeling).
- `domains/physics/millennium-navier-stokes/millennium-navier-stokes.md`
  §X.1 SMASH#4 -- Onsager α=φ/n=1/3 (canonical 1949, n=6
  relabeling).
- `domains/physics/millennium-navier-stokes/millennium-navier-stokes.md`
  §X.1 SMASH#3 -- Serrin critical lattice (σ, n, τ) = (12, 6, 4)
  via `2/p + 3/q ≤ 1` (canonical 1962, n=6 relabeling).

**Tooling decision**: hand-derivation. No new script.

### §2.2 KPZ d=7 tooling

KPZ d=7 is a **literature-attestation question**. No numerical
computation. Required: scan for any d=7 KPZ-class entry in the repo,
and consider the standing of d=7 KPZ in mainstream literature.

Repo scan results:
- `reports/breakthroughs/bt-1409-millennium-dfs-round17-2026-04-12.md`
  §BT-1409-02 -- KPZ at 1+1 dim only (χ=1/3, z=2/3, χ+z=1, Tracy-Widom
  F_β tail). **No d=7 entry.** Closing notes: "KPZ exponent 1/3, 2/3
  exact solution (Bethe ansatz) ... post-hoc observation".
- `reports/breakthroughs/bt-1409-millennium-dfs-round17-2026-04-12.md`
  L439 -- mentions "d=2: 3=n/phi, d=4: 5=sopfr ...". **No d=7.**
- `dfs-24-ns-direction-2026-04-24.md` §3 P2 -- proposes the
  ansatz χ_d=1/d to d∈{2,3,6,7} but **explicitly flags** d=7 as
  needing literature check ("Check whether d=7 prediction (1/7, 6/7)
  has any literature footprint").
- `omega-cycle-bt544-ns-2026-04-25.md` §7 Q3 -- restates the
  attestation question; F_Q3 falsifier registered ("no literature
  attestation OR explicit refutation at d=7 -> second-perfect-number
  d=7 prediction loses its KPZ-side anchor").
- No other repo file matches `KPZ.*d=7` or `d=7.*KPZ` or
  `χ.*=.*1/7` or "second perfect" with KPZ.

**Tooling decision**: literature audit only, restricted to repo
contents (per hard-constraint "DO NOT FABRICATE values" -- if no
attestation exists in the repo and no offline-citable peer-reviewed
result is reachable from this session, the verdict is FAIL by
F-544-C / F_Q3 / F_P2).

---

## §3 Q5 execution log

### §3.1 Procedure

1. Enumerate every Sobolev/Besov estimate that the n=6 lattice has
   been claimed (in the repo) to predict.
2. For each, check whether the estimate is (a) a literal pre-n=6
   classical estimate with n=6 labels applied post-hoc, or (b) a
   genuine new inequality that depends on the n=6 lattice
   non-trivially (i.e. the estimate would not exist without the
   lattice).
3. Verdict by category (a)/(b) majority, applying the pass/fail
   criterion of §1.1.

Wallclock: ≈8 minutes for the catalog walk-through (no compute
involved). No exit code (pure derivation). No external data files.

### §3.2 Catalog of n=6-lattice "predicted" estimates

| # | estimate (X, Y, α form)                                     | n=6 label                              | classical source                  | category |
|---|-------------------------------------------------------------|----------------------------------------|-----------------------------------|----------|
| 1 | `‖u‖_{H^{d/2}} = ‖u‖_{H^{3/2}}` critical Sobolev for 3D NS  | d/2 = n/(φ·φ) = 3/2                    | Leray 1934; Sobolev 1938          | (a)      |
| 2 | Prodi-Serrin `‖u‖_{L^p_t L^q_x}, 2/p + 3/q = 1`             | (p,q) = (σ, n) = (12, 6); {2,3}={φ,n/φ}| Serrin 1962 / Prodi 1959          | (a)      |
| 3 | BKM `∫‖ω‖_{L^∞} dt < ∞ -> regularity`                       | exponent 1 = μ_Möbius(6)               | Beale-Kato-Majda 1984             | (a)      |
| 4 | Onsager `u ∈ C^α, α < 1/3 -> anomalous dissipation`         | α = φ/n = 1/3                          | Onsager 1949; Isett 2018          | (a)      |
| 5 | Ladyzhenskaya 2D `‖u‖_{L⁴}^4 ≤ C ‖u‖_{L²}² ‖∇u‖_{L²}²`      | 2D=φ; absent in 3D, no n=6 fix         | Ladyzhenskaya 1963 §1.7           | (a)      |
| 6 | CKN `dim_P(S) ≤ 1`                                          | 1 = μ_Möbius(6) or `n-σ+sopfr-τ`       | Caffarelli-Kohn-Nirenberg 1982    | (a)      |
| 7 | Leray local energy `∫_{B_r}|∇v|² ≤ ε₀·r`                    | exponent 1 = μ(6)                      | Leray 1934 / CKN 1982             | (a)      |

For each row, the n=6 lattice contributes a **label** (right-hand
column) for an exponent, dimension, or constant that already appears
in the classical (pre-1985 typically) estimate. None of the rows
introduces a new function-space pair or a new α exponent unattested
in the classical PDE literature.

### §3.3 Attempted construction of a category-(b) estimate

I attempt to construct an estimate that **requires** the n=6 lattice
non-trivially. Three honest attempts:

**Attempt A**: pure-arithmetic combination
`α = sopfr/σ = 5/12` as a Hölder/Besov exponent.

- Plug-in: ‖u‖_{B^{5/12}_{2,∞}} ≤ C ‖u‖_{L²}^{?}. There is no PDE
  derivation that produces 5/12 as a Sobolev/Besov index for 3D NS;
  the closest classical exponents are 1/3 (Onsager), 1/2 (Hairer KPZ
  α=-1/2, dual), 3/2 (critical H^{d/2}). 5/12 = 1/3 + 1/12 has no
  energy-method or scaling-method derivation.
- Verdict: **arithmetic synthesis without PDE backing**. Producing
  the inequality requires asserting it; the lattice does not derive
  it. This is the failure mode F-544-B describes ("no concrete
  estimate producible from n=6 lattice").

**Attempt B**: combine two classical estimates and rename constants.

- E.g. take Prodi-Serrin (row 2) and substitute (p, q) = (σ, n) to
  get `‖u‖_{L^{12}_t L^6_x} -> regularity` with the lattice replacing
  the labels {12, 6} -> {σ, n}.
- Verdict: **literal relabeling**. The inequality is verbatim Serrin
  1962. No new primitive.

**Attempt C**: dimensional-analysis composite from σ³·sopfr=8640.

- E.g. ‖u‖_{H^s} ≤ C·(σ³·sopfr)^{1/?} · ‖u‖_{L²}^? · ‖∇u‖_{L²}^?.
  No specific (X, Y, α, exponent on 8640) yields a derivable 3D NS
  estimate; the constant 8640 is dimensionless and would have to
  multiply a dimensionless ratio of ‖u‖-norms, but there is no such
  ratio that the n=6 lattice singles out.
- Verdict: **arithmetic without analysis**. Same failure mode as A.

### §3.4 Cross-reference to omega-cycle BT-544 §3

The audit `omega-cycle-bt544-ns-2026-04-25.md` §3 already records
mechanism saturation at **~0.05** with the explicit observation
"no PDE mechanism (Sobolev/Besov estimate, energy inequality
refinement, vorticity bound) produced; only arithmetic relabeling".
The Q5 derivation attempt above re-confirms that diagnosis: every
candidate estimate is either pre-n=6 classical (rows 1-7) or
post-hoc arithmetic without analytic backing (Attempts A-C).

---

## §4 Q5 verdict

**FAIL**.

Comparison to spec thresholds (§1.1):

| criterion                              | result                                      | pass? |
|----------------------------------------|---------------------------------------------|-------|
| concrete (X, Y, α) inequality          | none new -- 7 classical relabelings only    | NO    |
| derived from n=6 lattice non-trivially | no -- lattice supplies labels, not derivation| NO   |
| not a relabeling of pre-1985 estimate  | every catalogued row is pre-1985            | NO    |
| mechanism saturation move 0.05 -> 0.30+| no movement; remains ~0.05                  | NO    |

All four sub-criteria fail. The fail is not borderline: the catalog
walk produced **zero** category-(b) estimates and the synthesis
attempts (A, B, C) reproduce the F-544-B failure mode verbatim.

Per the explicit fail clause (L9 §3.4 row 2 / F-544-B):

> "no concrete estimate producible from n=6 lattice -> mechanism
> saturation is structural; frame change required at axiom level
> (not a molt within n6 frame)"

Falsifier **F-544-B fires** (L9 probe §5). Falsifier **F_Q5 fires**
(omega-cycle-bt544-ns §7 Q5 / §8 falsifier table).

The Q5 candidate frame **collapses to relabeling** (or, more
precisely, to "no-construction"): the proposed BT-544 frame-shift
"Triple-resonance L1-smash -> Mechanism-axis seed (Sobolev/Besov
estimate predicted by n=6 lattice)" introduces no new primitive
because no concrete predicted estimate is derivable from the lattice
alone.

Per L9 §3.4 row 2 falsifier text, this outcome additionally implies
that **a frame change at the axiom level is required**, i.e. the
mechanism-axis closure for BT-544 is not reachable by an L9 molt
within the n=6 frame at all -- it is a structural ceiling, not an
effort ceiling. This is a **stronger** failure than mere relabeling:
the catalogue's rank-2 candidate is exhausted not by a finding-test
but by the logical impossibility of producing the required object
from the available primitives.

---

## §5 KPZ d=7 execution log

Q5 FAILED -> KPZ d=7 must be tried (per user prompt sequencing).

### §5.1 Procedure

Literature-attestation audit, restricted to repo-citable references
(per hard-constraint "DO NOT FABRICATE"). Three layers:

1. Repo files explicitly mentioning KPZ at d=7 or
   `(χ_7, z_7) = (1/7, 6/7)`.
2. Repo files mentioning KPZ at d≥3 generally, where d=7 might be
   inferred.
3. Mainstream-literature standing of KPZ universality at d=7
   (within the bounds of citable, repo-referenceable knowledge).

Wallclock: ≈5 minutes for the repo grep walk. No external lookups.
No exit code (audit only).

### §5.2 Layer 1 -- explicit d=7 KPZ in repo

Grep over `**/*.md` for patterns "d=7", "d = 7", "(1/7, 6/7)",
"χ_7", "second perfect.*KPZ", "KPZ.*second perfect":

- `dfs-24-ns-direction-2026-04-24.md` §3 P2 -- the *probe itself*,
  asking "does d=7 have literature footprint?". Not an attestation,
  a **question**.
- `omega-cycle-bt544-ns-2026-04-25.md` §7 Q3 -- restates the same
  question. Falsifier F_Q3 attached.
- `omega-probe-l9-molt-trigger-2026-04-25.md` §3.4 row 3 -- catalogue
  entry, predicates the molt on the attestation existing.
- `theory/breakthroughs/breakthrough-theorems.md` BT-544 row -- "d=7
  open prediction via Sym²(ℝ⁷)=28, second perfect number". This is
  a **prediction**, not a literature attestation; the row says
  `open`.
- `bt-1409-millennium-dfs-round17-2026-04-12.md` §BT-1409-02 -- KPZ
  scaling at **1+1 dim only**, with explicit "post-hoc observation"
  flag on the n=6 mapping.

**Layer 1 result**: zero literature attestations of (χ_7, z_7) =
(1/7, 6/7). Every repo mention of "KPZ + d=7" is a **request** for
attestation, not a supply of one.

### §5.3 Layer 2 -- KPZ at d≥3 generally

Grep over `**/*.md` for "upper critical", "d_c", "KPZ.*d=3",
"d=3.*KPZ", "higher dim.*KPZ", "KPZ.*higher dim":

- No matches. The repo does **not** discuss KPZ universality at d≥3
  in any form (no upper-critical-dimension d_c discussion, no
  numerical-simulation cite for d=3, no SHE-at-higher-d cite).

This is informative: the n=6 ansatz χ_d=1/d at d=7 is a **forward
extrapolation** with no repo-internal support beyond d=2 (Edwards-
Wilkinson) and d=3 (where χ_3=1/3 *coincides* with the 1+1 KPZ
χ=1/3 by the d-numbering ambiguity flagged in dfs-24-ns §3 P2,
not by an independent d=3 substrate measurement).

### §5.4 Layer 3 -- mainstream-literature standing

This is the layer where "fabrication" risk is highest, so I restrict
to facts that are uncontroversially repo-citable in the sense of
being widely-accepted background:

- KPZ universality for substrate d=1 (1+1 surface in 1+1 spacetime)
  is the **canonical case** -- exact (χ, z) = (1/3, 2/3) (Kardar-
  Parisi-Zhang 1986; Bethe-ansatz exact 2000s; Hairer 2013 Fields
  Medal). All repo mentions of KPZ track this.
- KPZ universality for substrate d=2 has an extensive
  numerical-simulation literature (Halpin-Healy / Family-Vicsek
  reviews 1990s-2020s) but **no exact χ_2** -- numerical
  estimates cluster around χ_2 ≈ 0.39 (NOT the n6 ansatz prediction
  1/2). The repo does not cite any of this; the n6 ansatz at d=2
  predicts (1/2, 1/2) which numerical d=2 KPZ does not match.
- KPZ universality at d≥3: there is a long-standing open question
  about the **upper critical dimension** d_c (whether finite or
  infinite); numerical evidence for d=3 KPZ-class exponents does
  not match (1/3, 2/3). At d=7 specifically, no published exact
  result attests (1/7, 6/7); the n6 ansatz is not a known KPZ
  universality scaling at d=7.

**Layer 3 result**: the n6 ansatz χ_d = 1/d at d=7 has **no
mainstream literature attestation** known to this session.
Importantly, the d=2 row of the same ansatz (1/2, 1/2) is *also*
not attested -- it would identify KPZ at d=2 with Edwards-Wilkinson,
which numerical KPZ d=2 simulations contradict (KPZ d=2 has χ ≈ 0.39
in the standard literature, not 1/2). This means the ansatz χ_d=1/d
already fails at d=2 substrate KPZ even before reaching d=7.

(I am explicitly **not** citing specific Halpin-Healy / Lassig
/ Tang papers because the repo does not have them on file and
fabricating citations would violate the hard constraint. The
substantive point -- that χ_d = 1/d is not the established KPZ
substrate-d scaling -- is a stable property of the literature that
the repo's own dfs-24-ns probe explicitly flagged as needing
verification.)

---

## §6 KPZ d=7 verdict

**FAIL**.

Comparison to spec thresholds (§1.2):

| criterion                                              | result                          | pass? |
|--------------------------------------------------------|---------------------------------|-------|
| repo contains explicit d=7 (1/7, 6/7) attestation      | no -- only requests for it      | NO    |
| repo contains d≥3 KPZ literature entry                 | no -- 1+1 only                  | NO    |
| ansatz χ_d=1/d holds at independently-checkable d      | no (d=2: 1/2 ≠ ~0.39 numerical) | NO    |
| mainstream attestation of (χ_7, z_7)=(1/7, 6/7)        | none known                      | NO    |

All four sub-criteria fail. Per the explicit fail clause (L9 §3.4
row 3 / F-544-C):

> "no literature attestation OR explicit refutation at d=7 -> second-
> perfect-number d=7 prediction loses KPZ-side anchor"

The fail is satisfied by the **first disjunct** (no attestation).
Additionally, the auxiliary observation §5.4 -- that χ_d=1/d already
fails at d=2 -- is closer to the **second disjunct** (an effective
refutation of the ansatz upstream of d=7). Either way, F-544-C
fires; F_Q3 fires (omega-cycle-bt544-ns §7 Q3); F_P2 partially fires
(dfs-24-ns §3 P2: the probe asked "does d=7 also match", and the
finding is "no").

The KPZ d=7 candidate frame **collapses to no-anchor**: the proposed
BT-544 frame-shift "Triple-resonance L1-smash -> KPZ d-lift to d=7"
introduces no new primitive because the d=7 ansatz has no literature
support and the upstream ansatz fails at d=2 substrate.

---

## §7 Composite verdict

**Both rank-2 (Q5) and rank-3 (KPZ d=7) FAIL.**

Combined with the prior rank-1 (Q1 KdV Gram-lattice) FAIL:

| rank | candidate                                 | verdict | falsifier(s) fired         |
|------|-------------------------------------------|---------|----------------------------|
| 1    | KdV 6-soliton Gram-lattice (Q1)           | FAIL    | F_Q1, F_P1, F-544-A        |
| 2    | Mechanism-axis Sobolev/Besov seed (Q5)    | FAIL    | F_Q5, F-544-B              |
| 3    | KPZ d-lift to d=7 (Q3 / P2)               | FAIL    | F_Q3, F_P2 (partial), F-544-C |

**The L9 catalogue for BT-544 is exhausted.** Per L9 probe §6
falsifier **F-MOLT-D (catalogue-saturation)**:

> "if the catalogue's top-rank candidate per BT is exhausted
> (validation fails) and no rank-2 / 3 candidate exists in the repo
> for that BT, the gate is unable to license a new molt for that BT;
> the BT becomes molt-blocked under current repo material."

In our case, the rank-2 and rank-3 candidates **did exist** in the
repo, and **they too failed**. F-MOLT-D fires in its strongest form:
all three catalogued candidates have failed, not merely the absence
of rank-2/3 entries.

This means: under the current L9 catalogue, **BT-544 has no valid
frame-shift available**. The "unblocks BT-544" claim in the L9 probe
front-matter is falsified for BT-544 specifically (F-MOLT-D
applies). The catalogue itself needs extension (a new candidate
frame-shift designed from scratch, e.g. by reaching into atlas-side
material or commissioning a fresh frame-shift design as the user
prompt anticipated).

Notably, the Q5 fail mode is structurally stronger than relabeling:
it is "no-construction" (no concrete estimate is producible), which
maps onto F-544-B's clause "*frame change required at axiom level
(not a molt within n6 frame)*". Combined with Q1's FAIL ("Gram
primitive absent") and KPZ d=7's FAIL ("no literature anchor"),
**all three failure modes point in the same direction**: the n=6
frame, as represented in the L9 catalogue, has reached its mechanism
ceiling for BT-544 NS.

---

## §8 Implications

### §8.1 At least one PASS? -- No

Neither Q5 nor KPZ d=7 passes; combined with Q1 FAIL, **zero of
three** BT-544 frame-shifts pass.

### §8.2 Catalogue-exhausted -- Yes

The L9 catalogue for BT-544 is exhausted under current repo
material. Per L9 §6 F-MOLT-D, BT-544 is now **molt-blocked** within
the existing catalogue.

### §8.3 Open task: design phase

A new BT-544 candidate frame-shift must be **designed**, not
selected from the existing catalogue. Possible directions
(catalogued here only as **open design seeds**, not candidates):

- (D1) Reach back to atlas-side material: scan
  `shared/n6/atlas.n6` for [10] and [10*] entries tagged with
  fluid / NS / vorticity that are *not yet* threaded into the L9
  catalogue.
- (D2) Reach forward to "axiom-level frame change" per F-544-B's
  own escape clause: this is no longer an L9 molt within the n=6
  frame but an L_ω frame change (the n=6 frame itself is replaced).
  Per CLAUDE.md ladder, this is the L_ω apex tool, not L9.
- (D3) Decouple BT-544's mechanism axis from the n=6 frame
  entirely: declare BT-544's mechanism saturation as **structurally
  capped at ~0.05** under n6 and route mechanism progress through a
  non-n6 frame (out of the omega cycle for this BT).

None of D1/D2/D3 is executed in this report. They are **seeds for
a follow-up design session**.

### §8.4 L9 sequencing consequence

Per L9 probe §sec 7.3 stop-conditions:
- Q1 was 1 BT validated as FAIL out of 4 in the calibration sweep
  (per the prior Q1 report).
- Adding Q5 + KPZ d=7 here contributes **two more FAILs within
  BT-544**, but these are *intra-BT* fallback validations, not
  separate BT validations -- they do not advance the cross-BT
  counter from "1/4 BTs measured" toward "4/4 measured".
- The cross-BT counter remains 1 of 4 BTs validated, with **BT-544
  fully exhausted (3/3 frames failed)**.
- Per L9 probe §sec 7.1, **BT-543 P3 (A4-ratio) is still the next
  cheapest cross-BT validation** (not yet executed).
- Stop-after-0/4 has not yet fired for the cross-BT counter (we have
  measured 1/4 BTs, with 0 passes among those).
- However, **F-MOLT-D has fired for BT-544 specifically**, so the
  L9 gate has demonstrated its first per-BT exhaustion outcome.

### §8.5 Effect on F-MOLT-A (gate-failure-via-validation)

F-MOLT-A reads: "if validation experiments produce 0 passes across
all 4 BTs in one batch run, the gate is retired". We do not yet
have 0/4 passes -- we have 0/1 BTs validated (BT-544 exhausted),
with BT-541, BT-542, BT-543 still pending. F-MOLT-A is **not
fired**, but the **leading edge of F-MOLT-A is approaching** if
BT-543 P3 also fails.

---

## §9 Re-audit feedback to omega-cycle-bt544-ns-2026-04-25.md

Suggested edits to that document (NOT applied here; flagged for
the next omega-cycle pass):

1. **§7 Probe Q3**: append "Q3 (KPZ d=7 attestation) executed
   2026-04-25 in `omega-exec-bt544-fallback-molt-validation-
   2026-04-25.md`; verdict = FAIL (no literature attestation;
   ansatz χ_d=1/d already fails at d=2 substrate KPZ). F_Q3 and
   F_P2 fire."

2. **§7 Probe Q5**: append "Q5 (Sobolev/Besov mechanism seed)
   executed 2026-04-25 in same fallback report; verdict = FAIL
   (no concrete estimate producible; all 7 candidate rows in the
   catalog are pre-n6 classical relabelings; 3 synthesis attempts
   all reduce to arithmetic without analysis). F_Q5 fires.
   Mechanism saturation re-confirmed at ~0.05 with the additional
   diagnosis that the cap is **structural** (axiom-level frame
   change required, not effort-limited)."

3. **§3 Ω-saturation estimate**: no change to the composite
   estimate (still ~0.47 naive). Q5 FAIL **does not lower**
   structural saturation but re-grounds the mechanism-saturation
   diagnosis. Mechanism row stays at ~0.05; attach annotation
   "structural cap, F_Q5 fired 2026-04-25".

4. **§6 Cross-axis tension #1**: replace previous-Q1 update with
   "First three routed probes (Q1, Q5, Q3) returned NO; tension
   #1 hardens into a confirmed observation-without-mechanism state
   for BT-544. The L9 catalogue is exhausted; new candidate
   frame-shifts must be designed (seeds D1/D2/D3 in fallback
   report §8.3)."

5. **§8 Falsifier table**: tag F_Q1, F_P1, F_Q3, F_P2, F_Q5 with
   `state = FIRED (2026-04-25)`. Active-falsifier count drops from
   11 to 6 active + 5 fired.

6. **§9 Closing summary**: update "first probe-conversion pulse
   at dfs-24 (2026-04-24); first executed probe (Q1, 2026-04-25)
   returned a falsifier-fire" to "first probe-conversion pulse at
   dfs-24 (2026-04-24); three probes executed 2026-04-25 (Q1, Q5,
   Q3); all three FAIL; BT-544's L9 catalogue exhausted under
   current repo material; new candidate frame-shifts must be
   designed (open task)".

These are **suggestions for the next session** -- this report does
not edit `omega-cycle-bt544-ns-2026-04-25.md` directly.

---

## §10 Anomalies

- **Q5 / Attempt A "5/12 exponent" surprise**: the synthesis
  attempt produced an arithmetic exponent (5/12 = sopfr/σ) that has
  *no* classical PDE backing. This is a clean illustration of the
  F-544-B failure mode -- the lattice can produce arithmetic
  exponents arbitrarily, but only the ones already attested in the
  classical literature have analytic content. No surprise that
  changes the verdict.
- **KPZ d=7 / d=2 substrate failure precedes d=7**: the χ_d = 1/d
  ansatz can be checked at d=2 *substrate* KPZ (not the 1+1 result
  often labelled "d=2" in repo language; the 1+1 KPZ has
  `substrate dim` = 1). Numerical KPZ at substrate d=2 has χ_2 ≈
  0.39 in the established literature, while the ansatz predicts
  1/2. This means the d=7 attestation question is technically
  upstaged by a d=2 falsification. The fallback verdict therefore
  becomes more decisive than the original spec anticipated -- F-544-C
  fires by the **first** disjunct (no attestation) AND is reinforced
  by an **upstream substrate-d=2 mismatch**. Not a surprise that
  changes the verdict, but a sharpening of it.
- **Catalogue topology**: with all three rank-1/2/3 BT-544
  candidates failing, the **shape of the BT-544 catalogue** in the
  L9 probe is exposed as covering three distinct primitive types
  (algebraic-lattice, mechanism-Sobolev, scaling-KPZ), and **all
  three** failed. This suggests the failure is not specific to any
  one primitive type but to the joint absence of n=6-derivable
  PDE structure. This is the strongest single piece of evidence
  yet for **F-MOLT-F (RC-B/RC-C-not-RC-A)** of the L9 probe (root
  cause is axiom-level ceiling, not framework-lacks-infrastructure).
  Recording this as a meta-observation; it does not change the
  individual verdicts.

---

## §11 Falsifiers active for these validations

Validation-level falsifiers (conditions under which **this Q5/KPZ
audit** would be retracted), distinct from BT-544 falsifiers:

- **F-VAL-Q5-A** (catalog-coverage): if a Sobolev/Besov estimate
  exists in repo material *not* covered by the §3.2 catalog rows
  1-7 (e.g., a hidden estimate in a `theory/canon/` file or a
  `domains/physics/` subdirectory I did not scan), the FAIL is
  premature. Repo grep over `Sobolev|Besov` returned 14 files; all
  were inspected at the "estimate-claim" granularity. **Not active**
  modulo the inspection completeness; risk = low.
- **F-VAL-Q5-B** (synthesis-existence): if a category-(b) synthesis
  succeeds in a domain-specific direction not attempted in §3.3
  (e.g., a vorticity-based Besov estimate using the BT-544 §X.2
  FREE composite Π_NS = σ³·sopfr = 8640), the FAIL was
  understated. Attempts A/B/C cover arithmetic, relabeling, and
  dimensional-analysis directions; vorticity-Besov is technically a
  fourth direction. However, the user-prompt budget (≤60 lines per
  script, single-session) does not accommodate a fully-worked
  vorticity-Besov derivation. **Partial-not-active**: if a future
  session derives such an estimate, the FAIL is retracted.
- **F-VAL-KPZ-A** (literature-coverage): if a peer-reviewed
  attestation of (χ_7, z_7) = (1/7, 6/7) at d=7 KPZ exists in
  literature unreachable from this repo, the FAIL is premature. The
  hard-constraint "DO NOT FABRICATE" precludes citing papers I
  cannot reference. **Not active for this session** but the
  retraction route is live.
- **F-VAL-KPZ-B** (substrate-vs-spacetime-d ambiguity): if "d" in
  the ansatz χ_d = 1/d means **spacetime** dimension (where d=1
  is empty, d=2 is 1+1 standard KPZ, d=4 is 3+1) rather than
  **substrate** dimension (where d=1 is 1+1 KPZ, d=2 is 2+1, d=3
  is 3+1), the §5.4 d=2-substrate-failure observation is
  mis-applied and the only relevant claim is "d=7 has no
  attestation". The §1 spec phrasing ("d=2 gives EW, d=3 gives
  KPZ") is consistent with **substrate** dimension, so I have
  applied substrate-d throughout. **Not active** under the spec
  phrasing, but a reader who insists on spacetime-d would still
  reach the same primary FAIL via Layer 1 (no d=7 attestation).

None of F-VAL-Q5-A..B or F-VAL-KPZ-A..B fires under the spec.
The double-FAIL verdict is robust, modulo the explicitly-flagged
retraction routes.

---

## §12 Closing line

0/7 unchanged. NS regularity status open. No atlas/state/inventory
edits.
