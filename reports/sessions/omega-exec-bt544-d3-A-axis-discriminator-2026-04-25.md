---
id: omega-exec-bt544-d3-A-axis-discriminator
date: 2026-04-25
scope: research-only molt-validation via literature/sketch (NO NS claim, NO atlas promotion)
target: BT-544 D3.A axis A -- uniform Sobolev estimate on 2.5D non-local-pressure system
parent_reports:
  - reports/sessions/omega-seed-bt544-d3-mechanism-decouple-2026-04-25.md (§ A axis)
  - reports/sessions/omega-cycle-bt544-ns-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: discriminator evaluation, no proof claim
---

# Omega Exec — BT-544 D3.A Axis Discriminator (2026-04-25)

## §0 Non-claim disclaimer

This report **evaluates** the D3.A discriminator of `omega-seed-
bt544-d3-mechanism-decouple-2026-04-25.md` (§2.1 + §5). It does
**NOT**:

- claim 3D Navier-Stokes regularity (smoothness or blow-up);
- promote any atlas entry, modify `state/proposals/inventory.json`,
  modify `theory/canon/`, or alter the `BT-544 = 0/1 untouched`
  Clay status;
- replace the L9 catalogue exhaustion verdict in
  `omega-exec-bt544-fallback-molt-validation-2026-04-25.md` (the
  rank-1/2/3 candidates remain FAILed);
- claim a new theorem. Where I cite a result, I cite the *attestable
  classical reference* known to the repo, not a new proof.

D3.A is a **discriminator**, not a Clay attempt. PASS means "axis A
in isolation is clean under standard PDE machinery"; FAIL means
"axis A is structurally obstructed even with B and C switched off".
Either way, **0/7 unchanged**.

---

## §1 Axis A spec extracted (the 2.5D system, made precise)

Per `omega-seed-bt544-d3-mechanism-decouple-2026-04-25.md` §2.1 +
§5, the 2.5D ansatz is:

  v(t, x_1, x_2) = e_3 × ∇⊥ψ(t, x_1, x_2) + w(t, x_1, x_2) e_3      (1)

where ∇⊥ψ = (-∂_2 ψ, ∂_1 ψ, 0) is the 2D Hamiltonian gradient of a
streamfunction ψ : ℝ_+ × ℝ² → ℝ, and w : ℝ_+ × ℝ² → ℝ is the third
component (NB: independent of x_3 by ansatz, hence the "2.5D"
label: 2D spatial dependence, 3-component vector field).

**Divergence-freeness (free, by construction)**:
∇·v = ∇_h · (e_3 × ∇⊥ψ) + ∂_3 w = 0 + 0 = 0,
since the horizontal part is a 2D Hamiltonian flow (always
divergence-free in the plane) and w does not depend on x_3.

**The PDE system**: substituting (1) into 3D incompressible NS

  ∂_t v + (v·∇)v = -∇p + ν∇²v,    ∇·v = 0,                            (2)

with ∇p reconstructed by the Riesz-transform projection
P = I − ∇(-Δ)⁻¹∇· acting on the nonlinearity. The horizontal
component decouples (no x_3 dependence kills (v_3 ∂_3) terms):

  - **Streamfunction equation** (horizontal vorticity ω_h := -Δψ):
    ∂_t ω_h + u_h · ∇_h ω_h = ν Δ_h ω_h,    u_h := e_3 × ∇⊥ψ;        (3)
  - **Vertical scalar equation** (passive transport of w by u_h):
    ∂_t w + u_h · ∇_h w = ν Δ_h w.                                   (4)

Equation (3) is **2D Navier-Stokes** in vorticity form. Equation (4)
is a **2D linear advection-diffusion** of w by a 2D-NS-controlled
velocity u_h. The pressure p is *the 2D NS pressure* — it depends
only on u_h, not on w, because (v·∇)v_3 = u_h·∇_h w (no ∂_3 terms),
so the third component of the momentum equation has no pressure
contribution and the third row of the Riesz projector vanishes on
the (v_3 ∂_z stuff)-terms, which are themselves zero. Thus the
non-local pressure operator acts only via 2D Riesz transforms on
the planar nonlinearity.

**Vortex-stretching (axis B) is absent**: the 3D vorticity is
ω = ∇ × v = (∂_2 w, −∂_1 w, ω_h), and the stretching term (ω·∇)v
vanishes term-by-term on functions independent of x_3. **Axis B is
switched off by construction**, as predicted in §2.1.

**Energy cascade (axis C) is absent**: at fixed Galerkin
truncation K_max (or simply at any finite-energy level for the
viscous 2D problem), the system is dissipative-coercive in the
standard L²-energy sense; no Hölder critical exponent enters.
**Axis C is switched off by construction**, as predicted in §2.1.

**The discriminator question (§5 of D3 seed, restated)**: produce

  ‖v(t)‖_{H^s(ℝ²)} ≤ C(t, ν, ‖v_0‖_{H^s})                           (5)

uniform in K_max, explicit in (s, C, t-dependence), non-relabeling.

---

## §2 Literature scan results

### §2.1 In-repo grounding

The repo grounds the 2.5D system / its sub-pieces in the following
attestable references:

- `theory/study/p3/prob-p3-2-conditional-theorems.md` Theorem 544-E
  (Ladyzhenskaya 1968 / Uchovskii-Yudovich 1968 / Chen-Strain-Tsai-
  Yau 2008): axisymmetric without swirl ⇒ global smoothness for 3D
  NS. The 2.5D ansatz (1) is a *Cartesian-coordinate sibling* of
  axisymmetric-without-swirl: in both cases, divergence-freeness
  reduces to a 2D streamfunction + an independently transported
  third component, and the vortex-stretching term vanishes.
- `theory/study/p2/prob-p2-4-navier-stokes-barriers.md` §8.4
  ("Axisymmetric, No Swirl"): "axisymmetric + no-swirl → 2D, global
  regularity." Same principle, same outcome.
- `theory/study/p3/prob-p3-1-open-subquestions.md` BT-544 (i):
  "Chen-Strain-Tsai-Yau 2008/9 demonstrated regularity for
  axisymmetric without swirl. With swirl unresolved." Confirms
  that the *clean* (B-absent) regime is a closed problem in the
  literature; the open part is exactly axis-B (with-swirl).
- `omega-seed-bt544-d3-mechanism-decouple-2026-04-25.md` §2.1 cites
  Constantin-Majda-Tabak 1994 (CMT, generalized SQG / 2D non-local
  active scalar) and Kiselev-Nazarov 2012 as the literature family
  in which the planar Riesz-transform problem lives.

### §2.2 Attestable external references (cited by author/year only,
no fabricated theorems)

- **Ladyzhenskaya 1959/1969**: 2D Navier-Stokes global existence
  and uniqueness in H^s for s ≥ 1, ν > 0. (The standard textbook
  result; see Constantin-Foias 1988 *Navier-Stokes Equations*
  monograph for a modern presentation.)
- **Beale-Kato-Majda 1984** (CMP 94, p.61): the BKM criterion gives
  ‖v‖_{H^s} bounded if ∫₀^T ‖ω‖_∞ dt < ∞. In 2D NS with viscosity,
  ‖ω‖_∞ is controlled by the maximum principle on (3), hence BKM
  is automatic in 2D — a textbook fact.
- **Calderón-Zygmund 1952**: Riesz transforms are bounded
  ℝ_h^j : H^s → H^s for all s ∈ ℝ. The non-local pressure
  reconstruction p = R_i R_j (u_h^i u_h^j) inherits H^s bounds
  from u_h.
- **Constantin-Majda-Tabak 1994** (Nonlinearity 7): SQG-family
  global existence at high regularity for the dissipative case;
  the planar-Riesz machinery for active-scalar systems used here
  is the same machinery as for our (3).
- **Chen-Strain-Tsai-Yau 2008/9** (CMP / Adv. Math.): axisymmetric
  no-swirl 3D NS global regularity. Cartesian 2.5D = same
  structural reduction.

### §2.3 Note on the D3 seed's "moderate difficulty" estimate

The D3 seed §4.1 estimated axis A's discriminator at *moderate
difficulty*. The literature scan in §2.1-§2.2 actually positions
it at **classical / settled difficulty**: the 2.5D system in (3)+
(4) is essentially the textbook 2D NS coupled to a linearly-
advected passive scalar, both of which have been globally regular
since Ladyzhenskaya 1969 (for 2D NS) and standard parabolic theory
(for the passive scalar). The D3 seed's moderate estimate was made
*before* the explicit ansatz (1) was unfolded into the (3)+(4)
decoupling. Once unfolded, it is a literature-settled regime.

---

## §3 Discriminator path chosen

**Path P (PROOF-LITERATURE)**.

Justification: the system (3)+(4) is, term-by-term:

- (3) = 2D NS in vorticity form with viscosity ν > 0 — globally
  smooth in H^s for all s ≥ 1 (Ladyzhenskaya 1959/1969);
- (4) = linear 2D advection-diffusion with bounded-Lipschitz
  advecting field u_h (since u_h ∈ H^s ⊂ C^{s-1} for s > 2 by
  2D Sobolev embedding) — globally smooth in H^s by standard
  parabolic theory (Friedman 1964; Ladyzhenskaya-Solonnikov-
  Ural'ceva 1968).

The combination v = (u_h, w) = (e_3 × ∇⊥ψ, w) inherits H^s
regularity from each component, and the pressure p is recovered
from u_h via Riesz transforms, which preserve H^s
(Calderón-Zygmund 1952). All inputs are classical; no new theorem
is required.

This places the discriminator outcome firmly in **Path P**, not Q
or R. Path S (inconclusive) is excluded because the 2.5D system
is precisely defined by (1)-(4).

---

## §4 The bound and its sources

**Claim** (literature-import, not a new theorem): for the 2.5D
system (1)-(4) with initial data v_0 ∈ H^s(ℝ²)³, s ≥ 2, ν > 0,
there exists a uniform-in-K_max (Galerkin truncation) estimate

  ‖v(t)‖_{H^s} ≤ C(t, ν, ‖v_0‖_{H^s})                              (★)

with C depending on (t, ν, ‖v_0‖_{H^s}) through the standard 2D NS
+ passive-scalar dependence, **finite for all t > 0**, and
**uniform** in K_max (the Galerkin truncation passes to the limit
in the standard way).

**Step-by-step source attribution** (sketch with classical
inputs only; ≤ 12 inequality steps):

1. **Energy estimate on ψ-component**: for (3), the standard 2D NS
   energy identity gives
   (1/2) d/dt ‖u_h‖_{L²}² + ν ‖∇u_h‖_{L²}² = 0,
   hence ‖u_h(t)‖_{L²} ≤ ‖u_{h,0}‖_{L²}. (Leray 1934.)
2. **Enstrophy estimate (2D-specific)**: for (3), since
   (u_h·∇_h)ω_h is divergence-free-transported and 2D vorticity has
   no stretching,
   (1/2) d/dt ‖ω_h‖_{L²}² + ν ‖∇ω_h‖_{L²}² = 0,
   hence ‖ω_h(t)‖_{L²} ≤ ‖ω_{h,0}‖_{L²}. (Standard 2D NS;
   Ladyzhenskaya 1969.)
3. **Maximum principle for ω_h** (2D-specific): ‖ω_h(t)‖_{L^∞} ≤
   ‖ω_{h,0}‖_{L^∞}, since (3) is a 2D scalar advection-diffusion
   for ω_h with divergence-free u_h. (Standard parabolic
   maximum principle; Friedman 1964.)
4. **BKM in 2D is automatic**: by Step 3, ∫₀^T ‖ω_h‖_∞ dt ≤
   T·‖ω_{h,0}‖_∞ < ∞ for all T < ∞, so BKM (Beale-Kato-Majda 1984)
   gives smoothness of u_h on [0, T] for all T.
5. **Higher Sobolev for u_h**: standard 2D NS H^s estimate
   (Constantin-Foias 1988): for s ≥ 1, ‖u_h(t)‖_{H^s} ≤
   C₁(t, ν, ‖u_{h,0}‖_{H^s}).
6. **Riesz transform on pressure**: p = R_i R_j (u_h^i u_h^j), so
   ‖p‖_{H^s} ≤ C₂ ‖u_h‖_{H^s}² ‖u_h‖_{H^s} ≤ C₂ ‖u_h‖_{H^s}². The
   Riesz transform is bounded H^s → H^s (Calderón-Zygmund 1952).
7. **Energy estimate for w**: for (4), since (4) is divergence-
   free advected and dissipative,
   (1/2) d/dt ‖w‖_{L²}² + ν ‖∇w‖_{L²}² = 0,
   hence ‖w(t)‖_{L²} ≤ ‖w_0‖_{L²}.
8. **Maximum principle for w**: ‖w(t)‖_{L^∞} ≤ ‖w_0‖_{L^∞}, by the
   same 2D parabolic maximum principle on (4).
9. **Higher Sobolev for w**: differentiate (4) up to order s,
   commute ∂^α through (u_h·∇_h)w producing terms ≤ C·‖u_h‖_{H^s}·
   ‖w‖_{H^s} via Sobolev product inequalities (Moser); use Step 5
   for ‖u_h‖_{H^s}; Grönwall gives
   ‖w(t)‖_{H^s} ≤ C₃(t, ν, ‖u_{h,0}‖_{H^s}, ‖w_0‖_{H^s}).
   (Standard linear advection-diffusion; Friedman 1964;
   Ladyzhenskaya-Solonnikov-Ural'ceva 1968.)
10. **Reassembly**: ‖v‖_{H^s}² = ‖u_h‖_{H^s}² + ‖w‖_{H^s}² ≤
    C₁² + C₃² =: C(t, ν, ‖v_0‖_{H^s})².
11. **Galerkin truncation passes to the limit**: bound (★) is
    independent of K_max because each constant C₁, C₂, C₃ is
    independent of K_max (each is derived from estimates that hold
    on the full equation; the Galerkin approximation satisfies the
    same estimates with the same constants — standard 2D NS
    Galerkin theory; Constantin-Foias 1988 §10).
12. **Non-relabeling check**: the bound (★) is *not* a relabeling
    of an n=6-lattice quantity post-hoc — it is derived from
    Calderón-Zygmund + 2D NS H^s + parabolic-maximum-principle
    inputs, none of which uses the labels {σ, τ, φ, sopfr, n=6}.
    (F-D3-META-C clearance for axis A.)

The bound (★) is **uniform in K_max**, **explicit in t-dependence**
(via Step 5: typical 2D NS H^s grows at most exponentially in t for
fixed ν, and polynomially for high-Re asymptotics — both finite),
**explicit in ν** (each step has ν > 0 acting as parabolic
regularizer), and **does not invoke axis B or C**. ✓

---

## §5 Obstruction analysis (Path R check)

There is **no obstruction term** in (3)+(4) that fails to close
under standard 2D estimates. The candidate obstruction term —
vortex-stretching (ω·∇)v — vanishes identically in the 2.5D
ansatz (§1, paragraph "Vortex-stretching (axis B) is absent").
The pressure non-locality is harmless because Calderón-Zygmund
preserves H^s (Step 6). The cascade-pumping term (which would
inject anomalous dissipation) is absent at finite Galerkin
resolution and at finite ν (Step 11).

**No obstruction documented**. Path R is not active.

---

## §6 Verdict

**PASS via literature** (Path P).

Specifically: the uniform-in-K_max H^s estimate (★) for the 2.5D
system (1)-(4) is provable by classical inputs (Ladyzhenskaya
1959/1969 for 2D NS; Calderón-Zygmund 1952 for Riesz transform
boundedness; Friedman 1964 / Ladyzhenskaya-Solonnikov-Ural'ceva
1968 for parabolic linear advection-diffusion; Beale-Kato-Majda
1984 for BKM-in-2D). No new theorem is required, and the
discriminator's "non-relabeling" requirement (§5 of D3 seed) is
met because the bound is derived from analytic-PDE machinery, not
from the n=6 lattice.

**Caveat**: the verdict is "PASS via literature" precisely because
the 2.5D system reduces to known-regular components (2D NS +
passive scalar). It is **not** a Clay-relevant result. The
caveat in the D3 seed §6.4 (F-D3-META-D, "top-1 dispatch already
fired") is **partially active**: the discriminator outcome was
predictable from §2.1 of the D3 seed alone (CMT 1994 + Kiselev-
Nazarov 2012 + the explicit "switch off B and C by 2.5D ansatz"
construction). The novelty is **the framing** (one of three
mechanism axes is clean) not the per-component estimate.

---

## §7 Implications for D3 axis ranking

Per `omega-seed-bt544-d3-mechanism-decouple-2026-04-25.md` §3.2
compositional strategy and §4.2 ranking:

- **Axis A is clean**: confirmed by Path P verdict in §6 above.
  This is *consistent* with the D3 seed's prediction (axis A =
  moderate difficulty, top-1 dispatch). The compositional
  strategy survives this gate.
- **Compositional consequence**: the F-544-B-survival path (§3.2
  of D3 seed) is now **half-supported**. Two clean axes are needed
  for the compositional strategy to localize the obstruction onto
  the third. Axis A is one; axis C remains to be tested.
- **Axis ranking update** (post-A-PASS):

  | axis | discriminator | post-A status | next role |
  |------|---------------|---------------|-----------|
  | A    | uniform H^s on 2.5D non-local-pressure | **PASS_LITERATURE** | clean, retired from dispatch queue |
  | B    | BKM-finite or dim_P ≤ 1 on axisymmetric-with-swirl Euler | UNTOUCHED | **predicted obstruction-carrier**, defer to last |
  | C    | Kraichnan two-sided S_6 ∼ ℓ² bound | UNTOUCHED | **next dispatch** (likely fires F-D3-C relabeling) |

- **Recommended next axis**: **C** (Onsager / Kraichnan model).
  Rationale per D3 seed §5 ("Why not C first") is now reversed:
  with A confirmed clean, C's discriminator outcome — whether
  PASS via redefinition or FAIL via F-D3-C relabeling — is the
  next compositional gate. B is held in reserve as the predicted
  obstruction-carrier, which the D3 seed already flagged at
  *maximal* difficulty (§4.1: "the same difficulty as the original
  Clay problem restricted to axisymmetric Euler").
- **Non-implication for Clay**: axis A clean does **not** imply
  3D NS regularity. The compositional argument only localizes
  *where* the obstruction lives among {A, B, C}; it does not
  resolve the obstruction. **0/7 unchanged**.

---

## §8 Anomalies

1. **D3 seed difficulty estimate vs literature reality**. The seed
   §4.1 estimated axis A at "moderate difficulty"; the literature
   scan §2.1-§2.2 places it at *settled / classical* difficulty.
   This is a downward revision, not a contradiction — the seed was
   conservative. **Honest log entry**: predictability of the A
   verdict was higher than the seed suggested.
2. **Cartesian 2.5D ↔ axisymmetric-no-swirl correspondence**. The
   2.5D ansatz (1) and the axisymmetric-no-swirl ansatz are
   *structurally the same reduction* (2D streamfunction + third
   component decoupled from stretching). The repo grounds them
   under different theorem labels (Theorem 544-E for axisymmetric;
   CMT 1994 / Kiselev-Nazarov 2012 for the Cartesian SQG-family).
   This is not an anomaly per se but a redundancy in the seed's
   framing — flagged for the D3.C dispatch, where similar
   redundancies might exist between Kraichnan and the textbook
   passive-scalar literature.
3. **F-D3-META-D partially active**. The D3 seed flagged this
   meta-falsifier ("top-1 dispatch redundant if A already known
   in literature") and answered "currently not active" because
   the K_max-uniform-H^s form was claimed not to be on the books.
   In fact, the K_max-uniform form *is* on the books (Constantin-
   Foias 1988 §10 Galerkin theory for 2D NS; passive-scalar
   regularity is standard linear theory). F-D3-META-D should be
   reclassified **partially active** in retrospect: the
   discriminator was settled by literature import, not by new
   work. The compositional information gain is real (axis A's
   cleanness is now *recorded*) but the per-axis estimate was
   not novel.

---

## §9 Falsifiers active

Inherited from D3 seed §2.1 (F-D3-A) and §7 (meta-falsifiers):

| tag | falsifier | post-discriminator status |
|-----|-----------|---------------------------|
| F-D3-A | no uniform-in-K_max H^s estimate exists for the 2.5D system at any s ≥ 1 | **DOES NOT FIRE** — bound (★) is producible from §4 sketch |
| F-D3-META-A (axes-not-independent) | 2.5D ansatz requires non-trivial vortex-stretching | DOES NOT FIRE — stretching vanishes by direct calculation (§1) |
| F-D3-META-B (compositional-not-implication) | proving A and C clean ≠ B residual | **NOT YET TESTED** — requires C dispatch |
| F-D3-META-C (decouple-itself-relabeling) | per-axis bound is a relabeled classical result | **WEAKLY ACTIVE** — bound (★) IS classical, but the *framing* (axis A as one mechanism slice) is the new content; weakly survived per §4 Step 12 non-relabeling check, modulo the seed §7 honest concession |
| F-D3-META-D (top-1 already fired) | discriminator A settled by literature import | **PARTIALLY ACTIVE** in retrospect (§8.3) |

Per-target falsifiers from `omega-cycle-bt544-ns-2026-04-25.md`
§8 (F1–F6, F_P1, F_P2, F_P3, F_Q4, F_Q5) are **not affected** by
this discriminator (axis A is upstream of those).

---

## §10 Closing

- **D3.A discriminator outcome**: PASS_LITERATURE (Path P).
- **Bound (★)**: ‖v(t)‖_{H^s} ≤ C(t, ν, ‖v_0‖_{H^s}), uniform in
  Galerkin K_max, derived from 2D NS H^s + Calderón-Zygmund +
  parabolic-maximum-principle inputs.
- **Compositional consequence**: F-544-B-survival path half-
  supported (1 of 2 required clean-axis gates passed).
- **Next dispatch**: D3.C (Onsager / Kraichnan), per the post-A
  ranking update in §7.
- **Atlas / state / inventory**: untouched.
- **Millennium tally**: **0/7 unchanged**. BT-544 = 0/1 untouched.
- **No new theorem claimed.** All cited results are pre-existing
  (Ladyzhenskaya 1959/69; Calderón-Zygmund 1952; BKM 1984; CMT
  1994; Chen-Strain-Tsai-Yau 2008; Kiselev-Nazarov 2012; Constantin-
  Foias 1988 monograph).

— end discriminator —
