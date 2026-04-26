---
id: omega-exec-bt544-axisB-targeted-attack
date: 2026-04-25
scope: research-only candidate generation (NOT validating; axis-B-specific frame-shift)
target: BT-544 axis-B vortex-stretching -- targeted candidate after 3-pillar localization
parent_reports:
  - reports/sessions/omega-seed-bt544-d3-mechanism-decouple-2026-04-25.md (§4.2 axis B prediction)
  - reports/sessions/omega-exec-bt544-extb-cilyap-validation-2026-04-25.md (cross-term unsigned)
  - reports/sessions/omega-exec-bt544-d3-Bprime-axis-discriminator-2026-04-25.md (B' FAIL precedent)
millennium_resolved: 0/7 (unchanged)
grade: candidate generation, axis-B-targeted, no claim
---

# Omega Exec — BT-544 Axis-B Targeted Attack (2026-04-25)

## §0 Non-claim disclaimer

This report **generates** a single BT-544 frame-shift candidate
that is **specifically targeted at axis B** (vortex-stretching) of
the D3 decouple, after the 3-pillar obstruction-localization
analysis (D3.A PASS_LITERATURE; D3.C FAIL_INTERMITTENCY; D3.B'
merged-axis FAIL; EXT-A Helmholtz unsigned; EXT-B Path-Q
cross-term unsigned) localized the residual difficulty onto
axis B.

This file does **NOT**:

- claim 3D Navier-Stokes regularity, blow-up, or any Clay-form
  resolution;
- prove or sketch-prove any new theorem on vortex-stretching,
  geometric depletion, multifractal regularity, or Euler / NS
  blow-up — every cited result is a pre-existing published
  reference, cited by author / year / journal;
- promote any atlas entry, modify `state/proposals/inventory.json`,
  edit `theory/canon/`, alter the `BT-544 = 0/1 untouched` Clay
  status, or add to the L9 catalogue's active-candidate ledger;
- supersede or replace D1 / D2 / D3.A / D3.B' / D3.C / EXT-A /
  EXT-B / EXT-C catalogue rows. The axis-B targeted candidate
  is a **new generation**, not a rerun;
- claim a near-PASS verdict — see §6 expected verdict, which
  honestly anticipates F-AXISB-B firing.

The candidate generated here is a **research-direction-design**
artifact, not a near-closure attempt. The axis-B literature
(50+ years for axisymmetric-with-swirl Euler) is dense with
partial results that do not close, and any single candidate in
this lineage is realistically expected to either reduce to a
known result or hit a known structural obstruction. The value of
the generation is in **registering** the candidate's precise
form, its discriminator, and its falsifiers — so a future
validation session can fire the correct falsifier rather than
repeat the literature scan.

**0/7 unchanged. NS regularity status open. No atlas / state /
inventory edits.**

---

## §1 Axis-B obstruction recap (3-pillar localization)

### §1.1 Where the localization came from

The D3 mechanism-decouple seed
(`omega-seed-bt544-d3-mechanism-decouple-2026-04-25.md` §1.2) split
the BT-544 mechanism axis into three independent components:

- **Axis A** — Sym² / pressure non-locality (dim Sym²(ℝ³) = 6);
- **Axis B** — Λ² / vortex-stretching (dim Λ²(ℝ³) = 3);
- **Axis C** — Onsager / energy cascade (α_c = 1/3).

The seed's §4.2 ranking placed axis B as the **obstruction-carrier
candidate** with maximal expected discriminator difficulty:

> The full research community has worked on axis-B-in-isolation
> for 50+ years without resolving it; the discriminator is the
> same difficulty as the original Clay problem restricted to
> axisymmetric Euler.
> — D3 seed §4.1, axis B row

### §1.2 The three pillars that confirmed the localization

Three independent discriminators / validations each pointed back
at axis B as the residual carrier:

**Pillar 1 — D3.A axis A** (PASS_LITERATURE, recorded in
`omega-exec-bt544-d3-A-axis-discriminator-2026-04-25.md`):
the 2.5D incompressible advection-with-non-local-pressure system
admits a uniform-in-K_max H^s estimate via standard Calderón-
Zygmund + Galerkin-truncation machinery (Constantin-Majda-Tabak
1994, Kato-Fujita 1962 chain). Axis A is **clean**; the
obstruction does **not** live on the pressure non-locality alone.

**Pillar 2 — D3.C axis C** (FAIL_INTERMITTENCY,
`omega-exec-bt544-d3-C-axis-discriminator-2026-04-25.md`):
the Kraichnan-passive-scalar two-sided S_6 ∼ ℓ² bound is
**inconsistent** with the empirically established intermittency
corrections (She-Leveque 1994, Frisch 1995 *Turbulence: The
Legacy of A. N. Kolmogorov* Cambridge §8). The K41 / Onsager
exponent ζ_p = p/3 is *deviated from* in 3D NS, and the
deviation couples back to the field-geometry of the velocity —
i.e. axis C **couples to axis B** via intermittency.

**Pillar 3 — D3.B' merged axis** (FAIL_NO_LITERATURE_PATH,
`omega-exec-bt544-d3-Bprime-axis-discriminator-2026-04-25.md`):
the merged B' = "vortex-stretching kinematics on a multifractal
She-Leveque Hölder class" was tested against four cited
candidates (Constantin-Fefferman 1993, Bernard-Gawedzki-Kupiainen
1998, Hou-Luo 2014, Elgindi 2021) and none closed: every one
either (a) gave a partial bound whose hypothesis fails
multifractally, (b) furnished a blow-up precedent contradicting
uniform regularity, or (c) treated a different geometric setting.

### §1.3 The EXT-A / EXT-B confirmations of axis B as residual

Two further validations independently surfaced axis B's
vortex-stretching residual as the **uncontrolled** term:

**EXT-A Helmholtz unsigned**
(`omega-exec-bt544-exta-uomega-gradflow-validation-2026-04-25.md`):
the proposed gradient-flow recast of NS produced a Helmholtz-
decomposition residual whose sign on the vortex-stretching
production term is not constrained by the variational structure.

**EXT-B Path-Q cross-term unsigned**
(`omega-exec-bt544-extb-cilyap-validation-2026-04-25.md` §4.2,
§7.2): differentiating the candidate Constantin-Iyer / Otto /
Bakry-Émery functional W_NS along the coupled NS + stochastic-
Lagrangian flow produced two uncontrolled terms — the
Hess(log ρ):∇u cross term **and** the vortex-stretching production
term ∫(ω·∇)u·ω · ρ dx. The validation §8.3 explicitly noted:

> The vortex-stretching term ∫(ω·∇)u·ω ρ in (4.2) is **the NS
> regularity obstruction itself**. Even if the §8.1 / §8.2
> steps were resolved, the candidate functional inherits this
> obstruction unchanged.
> — EXT-B validation §8.3

### §1.4 The localized obstruction

Combining the three D3 axes' discriminator outcomes with EXT-A
and EXT-B's structural failures, the residual obstruction is:

**Vortex-stretching production (ω·∇)u acting on a velocity field
of low regularity, where the vorticity direction ξ = ω/|ω|
does not satisfy a Lipschitz / Hölder coherence hypothesis.**

This is the textbook axis-B obstruction (Beale-Kato-Majda 1984,
Constantin-Fefferman 1993, Hou-Luo 2014, Elgindi 2021). The
3-pillar localization confirms that the BT-544 difficulty
**does not factor through** A or C alone, and that the merged
B' = B + intermittency does not have a literature path to closure.

The natural next move is to **directly engage** the axis-B
literature with a candidate that addresses the vortex-stretching
production under a relaxed direction-of-vorticity hypothesis.
That is the candidate this report generates.

---

## §2 Existing axis-B literature inventory

This section catalogues what the axis-B literature actually
proves, distinguishing each citation's **published statement**
from its **adjacent open question**. The inventory anchors the
§3 candidate specification: the candidate must not duplicate any
established result, and must engage a clearly-identified gap.

### §2.1 Beale-Kato-Majda 1984

**Citation**: J. T. Beale, T. Kato, A. Majda, "Remarks on the
breakdown of smooth solutions for the 3-D Euler equations", Comm.
Math. Phys. 94 (1984), 61-66.

**Actual content**: a smooth solution u of 3D Euler (or NS) on
[0, T*) with T* < ∞ extends past T* if and only if

  ∫_0^{T*} ‖ω(t)‖_{L^∞} dt < ∞.                                  (BKM)

**Domain of validity**: smooth (C^∞ or sufficiently high Sobolev)
solutions; the criterion is a **continuation theorem**, converting
one regularity question (Sobolev breakdown) to another
(L^∞-vorticity time-integral).

**What BKM does NOT prove**: it does not prove ‖ω‖_{L^∞} stays
finite, nor that it diverges; it merely identifies the breakdown
norm. The L^∞-time-integral side is the open question.

**Role in axis B**: BKM is the **standard breakdown norm** for
axis B. Any axis-B candidate must either close (BKM) (i.e. prove
the time-integral is finite) or exhibit a blow-up scenario in
which the time-integral diverges with a controlled rate.

### §2.2 Constantin-Fefferman 1993

**Citation**: P. Constantin, C. Fefferman, "Direction of vorticity
and the problem of global regularity for the Navier-Stokes
equations", Indiana Univ. Math. J. 42 (1993), 775-789.

**Actual content**: define the vorticity direction
ξ(x, t) = ω(x, t) / |ω(x, t)| on the support of ω. If ξ is
**Lipschitz-continuous in space** with a modulus controlled by
|ω|^{-1} on the high-vorticity region — specifically, if there
exists a constant C such that

  |ξ(x, t) - ξ(y, t)| ≤ C · |x - y| · ρ(x, y, t)                 (CF1)

with ρ a function decaying at the rate that compensates the
worst-case vortex-stretching geometry — then the vortex-stretching
term (ω·∇)u is **geometrically depleted**: its contribution to
enstrophy growth dZ/dt is reduced relative to the naive bound,
and global regularity can be derived under the (CF1) hypothesis.

**Domain of validity**: smooth NS (Leray-Hopf weak + classical
strong continuation) solutions on which ξ is well-defined; the
hypothesis is on ξ, not on u or ω directly.

**What CF 1993 does NOT prove**:
- that ξ is actually Lipschitz on supp(ω) for generic NS data —
  this is a **conjectured geometric property** of NS solutions,
  unverified at this regularity tier;
- a depletion mechanism under weaker regularity hypotheses on ξ
  (BMO, VMO, Hölder C^β with β < 1, multifractal class);
- a blow-up scenario; CF 1993 is purely positive-direction.

**Companion result — Constantin 1994**: P. Constantin, "Geometric
statistics in turbulence", SIAM Rev. 36 (1994), 73-98. Reviews
the geometric depletion mechanism with experimental evidence
supporting alignment of ω with the intermediate eigenvector of
the strain-rate tensor S; the alignment is *statistical* (not
pointwise Lipschitz), so the depletion mechanism is *partially*
realised in numerical / experimental data but not rigorously
extended below Lipschitz.

**Companion result — Constantin-Fefferman-Procaccia 1996**: P.
Constantin, C. Fefferman, A. Procaccia, "Geometric constraints on
potentially singular solutions for the 3-D Euler equations",
Comm. PDE 21 (1996), 559-571. Extends CF 1993 to Euler under
related geometric hypotheses; same Lipschitz / Hölder coherence
requirement on ξ.

**Role in axis B**: the **load-bearing positive direction**
result. Any axis-B candidate aiming at uniform regularity has to
either (i) verify (CF1) directly on NS solutions, or (ii) extend
the depletion mechanism to a hypothesis weaker than Lipschitz
that **can** be verified on NS solutions.

### §2.3 Caffarelli-Kohn-Nirenberg 1982 (axis-B-adjacent)

**Citation**: L. Caffarelli, R. Kohn, L. Nirenberg, "Partial
regularity of suitable weak solutions of the Navier-Stokes
equations", Comm. Pure Appl. Math. 35 (1982), 771-831.

**Actual content**: the 1-dimensional parabolic Hausdorff measure
of the singular set S of a suitable weak solution is zero
(P^1(S) = 0); a singular point requires an ε-regularity violation
that is locally unavailable for L²-energy-class data.

**Role in axis B**: bounds the **size** of the potential singular
set, not the existence. CKN gives an upper bound on the singular
set's parabolic Hausdorff dimension (≤ 1) but does not prove
S = ∅. The axis-B question (does ω blow up in L^∞?) is logically
upstream of CKN.

### §2.4 Hou-Luo 2014

**Citation**: T. Y. Hou, G. Luo, "Toward the finite-time blowup
of the 3D axisymmetric Euler equations: a numerical investigation",
Multiscale Model. Simul. 12 (2014), 1722-1776; G. Luo, T. Y. Hou,
"Potentially singular solutions of the 3D axisymmetric Euler
equations", Proc. Natl. Acad. Sci. 111 (2014), 12968-12973.

**Actual content**: high-resolution numerical computation of 3D
axisymmetric incompressible Euler **with swirl**, on a solid
boundary, exhibits a self-similar finite-time blow-up at a ring
of points on the boundary. The maximum vorticity ‖ω(t)‖_{L^∞}
appears to diverge at finite time T* with a specific self-similar
profile.

**Status**: **numerical, not rigorous**. The computation is
state-of-the-art (Hou's group has refined the resolution and
verified the self-similar scaling consistent across multiple
follow-up papers; e.g. Liu-Hou 2015, Wang-Lai-Gomez-Serrano-Hou
2023), but the blow-up has not been promoted to a rigorous
theorem in the unmodified setting.

**Role in axis B**: the **strongest blow-up evidence** in
axisymmetric-with-swirl Euler. Any axis-B candidate that argues
for global regularity *in this geometry* contradicts the Hou-Luo
numerical evidence; conversely, any candidate that argues for
blow-up has the Hou-Luo scenario as the leading exemplar but
must extract rigorous control of the singular set's geometric
properties (Hölder regularity at the singularity, parabolic
Hausdorff dimension).

### §2.5 Chen-Hou 2022 (axisymmetric Boussinesq)

**Citation**: J. Chen, T. Y. Hou, "Stable nearly self-similar
blowup of the 2D Boussinesq and 3D Euler equations with smooth
data", arXiv:2210.07191 (2022); follow-up: J. Chen, T. Y. Hou, D.
Huang, "On the finite time blowup of the De Gregorio model for
the 3D Euler equations", Comm. Pure Appl. Math. 74 (2021).

**Actual content**: rigorous proof of finite-time blow-up for the
2D Boussinesq equations (which share the axisymmetric-Euler
self-similar structure) and for related 1D models (De Gregorio,
generalized Constantin-Lax-Majda). The proofs use computer-
assisted analysis of a fixed-point problem for the self-similar
profile.

**Role in axis B**: rigorously confirms blow-up in axis-B-
adjacent settings, and provides a template for promoting Hou-Luo
2014's numerical evidence to a rigorous theorem in 3D Euler with
swirl. **As of 2026-04-25, the 3D Euler axisymmetric-with-swirl
case remains numerically established (Hou-Luo 2014) and
adjacent-case-rigorous (Chen-Hou 2022) but not rigorous in the
target geometry.** This is the literature's frontier.

### §2.6 Elgindi 2021

**Citation**: T. M. Elgindi, "Finite-time singularity formation
for C^{1,α} solutions to the incompressible Euler equations on
ℝ³", Annals of Mathematics 194 (2021), 647-727.

**Actual content**: rigorous existence of a class of C^{1,α}
initial data (for some small α > 0) for 3D incompressible Euler
such that the solution develops a finite-time singularity. The
construction uses a self-similar profile and is for an
**axisymmetric-without-swirl-but-modified** geometry.

**Domain of validity**: axisymmetric, no swirl, C^{1,α}
regularity, Euler (not NS), some structural modification of the
data class. The blow-up is proved at α small enough — the
threshold α_max above which Elgindi's construction breaks is not
explicitly characterised in the 2021 paper.

**What Elgindi 2021 does NOT prove**:
- blow-up for NS (the dissipation ν Δu may regularize the
  Elgindi profile; this is the open NS question);
- blow-up at α = 0 (i.e. C^1 / Lipschitz vorticity) — the
  construction degenerates;
- blow-up at α near 1 (i.e. close to C² regularity) — Elgindi's
  α is small;
- a multifractal-class blow-up; the construction is in a single
  Hölder class C^{1,α} for one α.

**Role in axis B**: the **strongest rigorous blow-up** in 3D
Euler. Demonstrates that uniform regularity *is* refutable in a
Hölder class adjacent to the typical turbulent regime. Any
axis-B candidate aiming at uniform-NS-regularity in a class
related to C^{1,α} must explain why Elgindi's construction
**does not transfer** (e.g. the dissipation in NS damps the
Elgindi profile, or the candidate's hypothesis excludes the
Elgindi data class).

### §2.7 Synthesis — what's known, what's not

**Known (positive direction)**:
- BKM 1984: breakdown criterion via L^∞-vorticity time-integral.
- CF 1993: geometric depletion under Lipschitz-coherent ξ.
- CKN 1982: P^1(S) = 0 for suitable weak solutions.
- Axisymmetric-without-swirl: global smoothness (Ladyzhenskaya
  1968, Ukhovskii-Yudovich 1968, Chen-Strain-Tsai-Yau 2009).

**Known (negative direction)**:
- Hou-Luo 2014: numerical blow-up in axisymmetric-with-swirl
  Euler.
- Chen-Hou 2022: rigorous blow-up in 2D Boussinesq + adjacent
  models.
- Elgindi 2021: rigorous blow-up in C^{1,α} 3D Euler (modified
  geometry, small α).

**Open (the BT-544 axis-B residual)**:
- Existence / non-existence of a finite-time blow-up for 3D NS
  smooth Cauchy problem (the Clay question).
- Verification of the CF 1993 Lipschitz hypothesis on NS
  solutions.
- Extension of the CF depletion mechanism to non-Lipschitz
  vortex direction (BMO, VMO, multifractal).
- Promotion of Hou-Luo 2014 numerical blow-up to rigorous in 3D
  axisymmetric-with-swirl Euler.
- Whether NS dissipation regularizes the Elgindi profile.

The candidate generated in §3 engages the **third open
question**: extension of CF 1993's geometric depletion mechanism
to a vortex-direction regularity weaker than Lipschitz.

---

## §3 Candidate frame-shift specification

### §3.1 Selection rationale

Among the four candidate forms suggested by the prompt:

(a) **CF-Lipschitz extension** (relax the Lipschitz hypothesis on
    ξ to BMO / VMO / multifractal);
(b) **Hou-Luo numerical-bound** (convert numerical evidence to a
    quantitative bound);
(c) **Elgindi-NS-adjacency** (argue NS at α near 1+ inherits some
    Euler blow-up structure);
(d) **Vortex direction-frequency decomposition** (decompose ω
    into low/high frequency components, track regularity per
    component).

The cleanest grounded choice is **(a) CF-Lipschitz extension**, for
three reasons:

1. **Direct engagement with axis B**: CF 1993 is the load-bearing
   positive-direction axis-B result; relaxing its hypothesis is
   a positive-direction axis-B move (uniform regularity), not a
   negative-direction axis-B move (blow-up). The 3-pillar
   localization places the residual difficulty on the
   *uncontrolled* vortex-stretching production, which is exactly
   what CF depletion would control.

2. **Well-defined target hypothesis**: BMO and VMO have rigorous
   harmonic-analytic definitions (John-Nirenberg 1961 Comm. Pure
   Appl. Math. 14; Sarason 1975 Trans. Amer. Math. Soc. 207);
   they are **strictly weaker than Lipschitz** but **stronger
   than L^∞**, and they are the natural function spaces for
   directional fields with logarithmic singularities. Replacing
   "ξ Lipschitz" by "ξ ∈ BMO" or "ξ ∈ VMO" is a precise relaxation,
   not a vague one.

3. **Avoids known hard fronts**: option (b) requires converting
   numerical data to rigorous bounds (a known-hard task); option
   (c) requires adjacency arguments between Euler and NS that
   depend on the open dissipation-regularizes question; option
   (d) requires a multi-scale decomposition that the EXT-B
   validation already showed produces uncontrolled cross terms.
   Option (a) has a single, explicit hypothesis-relaxation
   target.

The other three options are noted in the §8 anti-list with
specific rejection reasons.

### §3.2 Candidate specification — CF-BMO depletion

**Candidate name**: **CF-BMO** (Constantin-Fefferman direction-
of-vorticity depletion under BMO regularity).

**Setting**: 3D incompressible Navier-Stokes on the periodic
torus T³ (or equivalently on ℝ³ with sufficient decay), viscosity
ν > 0, smooth divergence-free initial data u_0:

  ∂_t u + (u·∇)u = -∇p + ν Δu, ∇·u = 0, u(0) = u_0.              (3.1)

**Hypothesis (CF-BMO)**: there exists a constant C_BMO depending
only on the data u_0 such that for all t in the maximal interval
of smoothness [0, T*),

  ‖ξ(·, t)‖_{BMO(supp(ω))} ≤ C_BMO · log(2 + ‖ω(·, t)‖_{L^∞}),    (3.2)

where ξ = ω / |ω| on the support of ω, and BMO is the John-
Nirenberg bounded-mean-oscillation seminorm restricted to
supp(ω) with the appropriate ball-localisation.

**Conjectured conclusion (the candidate's monotonicity-style
statement)**: under (3.2), the vortex-stretching term (ω·∇)u
admits a logarithmically-corrected geometric depletion:

  d/dt (1/2)‖ω‖_{L²}² = - ν‖∇ω‖_{L²}² + ∫_{ℝ³} (ω · S · ω) dx
                      ≤ - ν‖∇ω‖_{L²}² + κ · ‖ω‖_{L²}² · log(2 + ‖ω‖_{L^∞}),  (3.3)

with κ a constant depending on ν, ‖u_0‖_{H^s}, and C_BMO. The
logarithmic-loss factor on the right of (3.3) is what
distinguishes the BMO-relaxation from the original CF Lipschitz
case (which gives a depletion without log-loss).

**Conjectured regularity consequence**: if (3.2) and (3.3)
hold, then a Gronwall argument with double-exponential growth
controls ‖ω‖_{L^∞} via BKM, yielding global smoothness for
3D NS on T³.

### §3.3 Term-by-term anatomy

| piece | symbol | structural role | NS / CF analog |
|-------|--------|-----------------|----------------|
| ω / |ω| | ξ | vorticity direction | CF 1993 (Lipschitz hypothesis) |
| ‖·‖_{BMO} | BMO norm | relaxed regularity space | John-Nirenberg 1961 |
| (ω · S · ω) | stretching production | enstrophy source | Constantin 1994 alignment |
| log(2 + ‖ω‖_{L^∞}) | log-loss | BMO-inheritance scale | Beale-Kato-Majda 1984 |
| ν ‖∇ω‖_{L²}² | dissipation | enstrophy sink | Leray 1934 |

The candidate's distinctive content is the **combination** of
(3.2) and (3.3) — a relaxation of CF's Lipschitz hypothesis to
BMO with an explicit logarithmic loss accounting for the BMO
function-space scaling (BMO ≈ L^∞ + log).

### §3.4 What this candidate is NOT

Honest scope-limitation:

- (CF-BMO) does **not** propose a new theorem on 3D NS regularity.
  The hypothesis (3.2) is itself **conjectural** — it is the
  axis-B-targeted relaxation of CF 1993's Lipschitz hypothesis,
  and it has not been verified on actual NS solutions.
- (CF-BMO) does **not** establish (3.3) as a theorem. The
  logarithmically-corrected depletion is the candidate's
  **conjectural sketch consequence** of (3.2); deriving (3.3)
  rigorously would require a BMO-version of the CF 1993
  computation, which is open.
- (CF-BMO) does **not** address the Hou-Luo / Elgindi blow-up
  scenarios directly. If the actual NS solution develops a
  blow-up profile in which ξ leaves BMO, (CF-BMO) does not
  apply (this is exactly F-AXISB-B in §5).

---

## §4 Discriminator (structural-literature, axis-B-specific)

### §4.1 The discriminator question

**Discriminator (CF-BMO)**: does the published harmonic-analysis
+ NS-regularity literature produce, in some chain of cited papers,
either (i) a proof of (3.2) on smooth NS data, or (ii) a proof of
(3.3) under the assumed (3.2)?

This is a **structural-literature** discriminator: it asks
whether the cited literature already contains the ingredients for
the candidate's two conjectural pieces, or whether at least one
piece is genuinely new.

### §4.2 Three discriminator paths (structurally)

**Path P (literature import for (3.2))**: does any paper prove
that ξ ∈ BMO with a logarithmic modulus (3.2) on NS solutions?

The expected literature scan:
- CF 1993 has Lipschitz, not BMO.
- Constantin 1994 has alignment statistics, not BMO bound.
- CFP 1996 has Hölder β > 0, not BMO.
- The post-2000 BMO-NS literature (e.g. Koch-Tataru 2001
  Adv. Math. 157, BMO^{-1} initial data) places **u_0** in BMO^{-1}
  for short-time existence, not ξ in BMO uniformly in time.
- Vasseur 2007/2010 (De Giorgi method) gives L² → L^∞ improvements
  but not vortex-direction BMO.

If (3.2) is not in the literature, Path P fails (which is
expected).

**Path Q (literature import for (3.3) given (3.2))**: does any
paper prove a logarithmically-corrected geometric depletion
under BMO vortex direction?

The expected literature scan:
- Coifman-Lions-Meyer-Semmes 1993 Annals 137 establishes
  div-curl lemmas in BMO/H¹ duality, which are the harmonic-
  analysis precursor to (3.3) but for *scalar* products, not
  the vortex-stretching tensor.
- Chae 2003 J. Diff. Equ. 195 ("On the vorticity direction near
  the maximum vorticity") refines CF 1993 but stays in
  Lipschitz / Hölder.
- Beirão da Veiga 2007 Z. Anal. Anwend. 26, Beirão da Veiga-Berselli
  2009 J. Diff. Equ. 246: refinements of CF with localized
  regularity hypotheses, still Hölder.

If (3.3) given (3.2) is not in the literature, Path Q fails
(also expected).

**Path R (sketch derivation of (3.3) from (3.2) + standard
tools)**: can (3.3) be derived from (3.2) using the CF 1993
proof structure with BMO replacing Lipschitz?

The expected sketch:
- CF 1993's depletion uses |ξ(x) - ξ(y)| ≤ C |x - y| · ρ(x, y)
  inside a singular-integral computation for ∫(ω·∇)u·ω dx.
- Replacing the Lipschitz bound with a BMO bound introduces a
  logarithmic factor via the John-Nirenberg distribution
  inequality
    |{x ∈ B : |f(x) - f_B| > λ}| ≤ C |B| · exp(-c λ / ‖f‖_BMO),
- The logarithmic factor propagates into the depletion estimate,
  giving the log-loss in (3.3).

Whether this sketch closes without uncontrolled cross terms is
an open question — which is precisely what the discriminator is
asking. If the sketch closes, the candidate is structurally
sound (subject to verifying (3.2)). If the sketch fails at a
specific algebraic step, the failure mode identifies the precise
obstruction.

### §4.3 Discriminator pass / fail criteria

**PASS_LITERATURE**: a published paper proves both (3.2) and
(3.3) (extremely unlikely; would close axis B).

**PASS_SKETCH**: Path R sketch closes — (3.3) is derivable from
(3.2) via standard CF 1993 + BMO singular-integral machinery
without uncontrolled cross terms (less unlikely; would shift the
candidate to "verify (3.2)" only).

**OBSTRUCTION_DOCUMENTED**: Path R sketch identifies a specific
algebraic step that fails (the BMO version of the CF singular-
integral computation produces an unsigned residual), or Path R
closes but (3.2) itself is incompatible with the Hou-Luo /
Elgindi blow-up scenarios.

**FAIL_RELABELING**: the BMO relaxation reduces post-hoc to the
original Lipschitz statement (i.e. (3.2) implies, modulo
constants, the Lipschitz hypothesis of CF 1993, and the
candidate adds no new content).

**FAIL_BLOWUP_PRECEDENT**: the Hou-Luo / Elgindi blow-up profiles
have ξ leaving BMO at the singular set, so (3.2) is provably
false on those profiles, contradicting the candidate's premise.

**INCONCLUSIVE**: the literature scan is underdetermined.

The discriminator is **structural-literature** (axis-B-specific):
it does not ask for a new theorem, only for a literature trace
or a sketch derivation grounded in published machinery.

---

## §5 Falsifiers (registered upfront)

Per the brief's hard-constraint structure, three falsifiers are
registered for the CF-BMO candidate **before** any validation
session runs. Each falsifier is paired with a sub-criterion that
unambiguously identifies its activation.

### §5.1 F-AXISB-A (relabeling)

**Statement**: the BMO hypothesis (3.2) is, modulo bookkeeping
constants, **equivalent to** the Lipschitz hypothesis (CF1) of
Constantin-Fefferman 1993 — i.e. on the support of ω with the
relevant localisation, BMO with a logarithmic modulus collapses
to Lipschitz, so the candidate produces no new content beyond
CF 1993.

**Activation criterion**: a published or sketch result establishes
that for vector fields on supp(ω) with a logarithmic-modulus
constraint of the form (3.2), the BMO seminorm and the Lipschitz
seminorm differ by at most a constant (i.e. the embedding
Lip(supp(ω)) → BMO(supp(ω)) is essentially invertible under (3.2)).

**Status**: NOT KNOWN to fire. BMO is **strictly weaker** than
Lipschitz in general (a function with logarithmic singularity,
e.g. log|x|, is in BMO but not in Lipschitz; Stein 1993 *Harmonic
Analysis* §IV.1.3 example). The activation requires a special
property of vector fields on *supp(ω)* with a logarithmic-
modulus constraint that does not hold for generic BMO functions.
Possible-but-not-likely activation.

### §5.2 F-AXISB-B (blow-up-incompatibility)

**Statement**: the Hou-Luo 2014 numerical blow-up scenario or
the Elgindi 2021 rigorous blow-up scenario contradicts the
candidate's premise (3.2) — i.e. on those blow-up profiles, the
vortex direction ξ **leaves BMO** at the singular set with a
modulus inconsistent with (3.2).

**Activation criterion**: examination of the Hou-Luo self-similar
profile (Luo-Hou 2014 PNAS §4) or the Elgindi 2021 self-similar
profile (Elgindi 2021 §6) shows that ξ acquires a logarithmic
singularity faster than (3.2) allows, or fails BMO altogether.

**Status**: PARTIALLY ACTIVE — likely. The Hou-Luo numerical
profile has a singular ring where ω has a self-similar blow-up;
the direction ξ rotates rapidly across the ring, and the
logarithmic-modulus constraint (3.2) is plausibly violated. The
Elgindi C^{1,α} profile has ω acquiring a 1/|x|^β-type
singularity with β > 0, and the direction ξ may leave BMO at
α small enough.

This is the **expected primary activation mode**.

### §5.3 F-AXISB-C (circular)

**Statement**: the candidate's premise (3.2) — that ξ ∈ BMO with
a logarithmic modulus on smooth NS solutions — is **itself
implied by NS regularity**, so the implication "(3.2) → (3.3) →
global smoothness" is circular: the hypothesis (3.2) is at
least as strong as the conclusion.

**Activation criterion**: a published result establishes that
(3.2) holds *only* on solutions that are already known to be
smooth — e.g. (3.2) requires u ∈ H^s for s large, which is
exactly the smoothness conclusion the candidate aims to deliver.

**Status**: PARTIALLY ACTIVE — non-trivial. CF 1993's original
Lipschitz hypothesis is not known to be implied by NS regularity
in any direct way (it is a *geometric* hypothesis on alignment,
not a *functional* hypothesis on Sobolev norms), and the BMO
relaxation should inherit this structural property. However, if
the BMO bound (3.2) is shown to follow from u ∈ H^s for some s
(via an interpolation chain), F-AXISB-C activates.

This is a **secondary activation mode**: even if F-AXISB-B does
not fire (i.e. the candidate's premise is compatible with known
blow-up scenarios), F-AXISB-C may fire as a structural circularity.

### §5.4 Falsifier-active summary

| tag | name | expected status |
|-----|------|------------------|
| F-AXISB-A | BMO-Lipschitz relabeling | NOT LIKELY (BMO strictly weaker than Lipschitz generically) |
| F-AXISB-B | Hou-Luo / Elgindi blow-up incompatibility | LIKELY (primary activation; vortex direction acquires log-singularity at blow-up) |
| F-AXISB-C | Premise circular w.r.t. NS regularity | POSSIBLE (secondary; BMO bound may follow from Sobolev regularity) |

The predicted activation hierarchy is **F-AXISB-B > F-AXISB-C >
F-AXISB-A**, with F-AXISB-B as the highest-probability primary
firing and F-AXISB-A as the lowest-probability backstop.

---

## §6 Cost estimate + expected verdict

### §6.1 Discriminator type and cost

**Discriminator type**: structural-literature (axis-B-specific).

The discriminator runs three paths (P, Q, R per §4.2) against
the axis-B literature. Each path asks a distinct question:
- P imports (3.2) directly;
- Q imports (3.3) given (3.2);
- R sketches (3.3) from (3.2) via CF 1993 + BMO singular-integral
  machinery.

**Validation cost**: HIGH.

Required reading depth (estimated from the cited references' page
counts and typical specialist time):

| reference | pages | reading depth |
|-----------|-------|----------------|
| CF 1993 (Indiana) | 14 pages | full proof reading |
| CFP 1996 (Comm. PDE) | 12 pages | proof structure |
| Hou-Luo 2014 (PNAS + MMS) | 6 + 55 pages | profile structure, regularity claims |
| Elgindi 2021 (Annals) | 80 pages | full statement, geometric class |
| Chen-Hou 2022 (arXiv 2210.07191) | 99 pages | sketch reading for adjacent rigorous case |
| Coifman-Lions-Meyer-Semmes 1993 (Annals) | 22 pages | div-curl in BMO/H¹ |
| John-Nirenberg 1961 (CPAM) | 11 pages | distribution inequality |
| Stein 1993 *Harmonic Analysis* §IV | 50+ pages | BMO machinery |
| Beirão da Veiga 2007 / 2009 | 25 + 30 pages | CF refinements |
| Chae 2003 (JDE) | 20 pages | vortex-direction near max |

Total estimated reading: ~400+ pages with proof-level depth
required for ~150 of these. This is **substantially higher** than
the EXT-B validation's reading load (~250 pages, primarily
Constantin-Iyer 2008 + Otto 2001 + Bakry-Émery 1985).

**Cost classification**: the validation cost is at the upper
range of structural-literature discriminators; comparable to a
specialist NS-regularity literature review.

### §6.2 Expected verdict

**Expected verdict**: **F-AXISB-B fires** (Hou-Luo / Elgindi
blow-up scenarios contradict (3.2)), with verdict
**OBSTRUCTION_DOCUMENTED** (or FAIL_BLOWUP_PRECEDENT).

**Probability calibration** (subjective; based on §1-§5 analysis):

| outcome | probability | reasoning |
|---------|-------------|-----------|
| PASS_LITERATURE (both (3.2) and (3.3) in literature) | < 1% | would close axis B; not in any cited reference |
| PASS_SKETCH (Path R closes) | ~10% | requires BMO version of CF 1993 to close without unsigned residual; plausible but unverified |
| OBSTRUCTION_DOCUMENTED via F-AXISB-B | ~60% | Hou-Luo / Elgindi profiles have rapidly-rotating ξ; logarithmic-modulus constraint plausibly violated |
| OBSTRUCTION_DOCUMENTED via F-AXISB-C | ~15% | (3.2) may follow from Sobolev regularity; secondary circular failure |
| FAIL_RELABELING via F-AXISB-A | ~5% | unlikely; BMO is generically strictly weaker than Lipschitz |
| INCONCLUSIVE (literature underdetermined) | ~10% | unlikely; the cited references span the literature's frontier |

**Most-likely-active falsifier**: F-AXISB-B (primary), with
F-AXISB-C as secondary. Verdict-class: OBSTRUCTION_DOCUMENTED.

This is an **honest** generation: the candidate is **not**
expected to be a near-PASS. Axis B is the 50+-year-open hard
front, and any single candidate in this lineage is realistically
expected to either reduce to a known result or hit a known
structural obstruction. The candidate's value lies in
**registering the precise BMO-relaxation form** so a future
validation can fire the correct falsifier rather than repeat the
literature scan.

### §6.3 What "OBSTRUCTION_DOCUMENTED" would teach

If the validation produces OBSTRUCTION_DOCUMENTED via F-AXISB-B,
the documented obstruction would be:

> The vortex direction ξ = ω/|ω| does not satisfy a logarithmic-
> modulus BMO bound (3.2) on the Hou-Luo / Elgindi self-similar
> blow-up profiles. The CF 1993 geometric depletion, even
> relaxed to BMO, **does not extend** to the Hölder regularity
> class in which finite-time singularities are known.

This is a **structural negative result** for the CF-extension
program: it would demonstrate that *no relaxation of CF 1993's
Lipschitz hypothesis to a BMO-class regularity* can recover
uniform regularity, because the failure mode (vortex direction
losing regularity) is built into the known blow-up profiles.

The result is informative for the L9 catalogue:
- it removes the "extend-CF-to-BMO" research direction from the
  axis-B candidate space;
- it directs attention to either (i) **even-weaker** vortex-
  direction hypotheses that are compatible with the known blow-
  up profiles but still permit some depletion mechanism, or
  (ii) **negative-direction** axis-B candidates (rigorously
  promote Hou-Luo to a theorem).

### §6.4 What PASS_SKETCH would teach (less likely, ~10%)

If Path R closes (sketch derives (3.3) from (3.2)), the
candidate would shift to a "verify (3.2) on NS solutions" task,
which is itself a major axis-B problem (verification of a
CF-style geometric hypothesis on actual NS solutions has been
open since 1993). PASS_SKETCH would not close BT-544 but would
**reduce** the candidate to a single open verification question.

---

## §7 Cross-axis ties

### §7.1 The 3-pillar localization recap

After the D3 program closure
(`omega-exec-bt544-d3-Bprime-axis-discriminator-2026-04-25.md`
§5) and the EXT-A / EXT-B validations:

- **D3.A** (axis A) — **PASS_LITERATURE** (clean axis);
- **D3.C** (axis C) — **FAIL_INTERMITTENCY + FAIL_RELABELING**
  (couples back to B);
- **D3.B'** (merged B + intermittency) — **FAIL_NO_LITERATURE_PATH**
  (no closure);
- **EXT-A** Helmholtz residual — **OBSTRUCTION_DOCUMENTED** at
  Helmholtz unsigned (vortex-stretching cross-term);
- **EXT-B** CI-Lyap — **OBSTRUCTION_DOCUMENTED** at Path-Q
  cross-term (vortex-stretching ∫(ω·∇)u·ω · ρ unsigned);
- **EXT-C** procedure-class — recorded with documented
  obstruction at procedure-translation step.

The five outcomes converge on a single residual:
**vortex-stretching production under low vortex-direction
regularity**.

### §7.2 Where CF-BMO sits in the L9 catalogue

The CF-BMO candidate is the **next non-redundant candidate**
after EXT-A / EXT-B / EXT-C exhaust the variational / Lyapunov /
procedure-class families. The justification chain:

1. EXT-A (variational recast) — failed at Helmholtz residual on
   vortex-stretching cross-term.
2. EXT-B (analytic-Lyapunov) — failed at Path-Q cross-term on
   vortex-stretching production.
3. EXT-C (procedure-class) — failed at procedure-translation
   step (no surgery analog for NS).
4. CF-BMO (axis-B targeted) — engages the vortex-stretching
   production **directly** via the CF 1993 geometric-depletion
   lineage, not via a higher-order variational structure.

The strategic shift from EXT-A/B/C to CF-BMO is: **stop looking
for a new framework that bypasses axis B; start engaging axis B
directly with the literature's strongest positive-direction
result.**

### §7.3 What CF-BMO does NOT compete with

CF-BMO is **upstream-orthogonal** to:
- the L9 catalogue rank-1/2/3 entries (KdV / Q5 / KPZ-d=7), which
  are FAILED for separate reasons;
- D1.4 She-Leveque dispatch, D1.3 NS↔MHD duality, D1.5 group-
  symmetry — these are mechanism-axis-A or cascade-axis-C
  candidates, not direct axis-B engagements;
- D2 R1+R5 axiom rewrite (auxiliary status only).

CF-BMO **inherits** the residual identified by:
- D3.A + D3.C + D3.B' (3-pillar localization);
- EXT-A Helmholtz unsigned (vortex-stretching cross-term);
- EXT-B Path-Q cross-term (vortex-stretching production).

### §7.4 Strategic implication

The 3-pillar localization + EXT-A/B/C exhaustion + CF-BMO
candidate together implement the strategic shift:

> **From**: "find a new framework (variational / Lyapunov /
> procedure) that closes BT-544 by bypassing axis B."
>
> **To**: "directly engage the known hard problem (CF 1993
> Lipschitz hypothesis) by relaxing it to a literature-natural
> weaker class (BMO), and document precisely where the
> relaxation fails."

This is **research-direction-design**, not near-PASS work. The
3-pillar + EXT exhaustion has confirmed that the bypass strategies
do not close; the remaining option is direct engagement, which
this candidate registers.

---

## §8 Anti-list — candidates considered and rejected

The following candidate forms were considered alongside CF-BMO
and rejected. The rejection rationale is logged for honesty.

### §8.1 Hou-Luo numerical-bound conversion

**Form**: convert Hou-Luo 2014's numerical self-similar blow-up
profile to a quantitative bound — e.g. extract the self-similar
exponent from the numerical data and prove a matching rigorous
bound on ‖ω‖_{L^∞}(t) ≤ C / (T* - t)^β.

**Rejected because**:
- the Hou-Luo profile is **numerical**, not rigorous; promoting
  a numerical exponent to a rigorous bound is itself a major
  axis-B problem (Chen-Hou 2022 does this for 2D Boussinesq
  using computer-assisted proof, but the 3D Euler axisymmetric-
  with-swirl case remains open as of 2026-04-25);
- the candidate would not be a positive-direction axis-B move
  (uniform regularity); it would be a negative-direction move
  (rigorous blow-up), which does not engage the BT-544 question
  in the direction of a Clay-style closure;
- the validation cost would be even higher than CF-BMO (full
  reading of the Hou-Luo numerical machinery + Chen-Hou
  computer-assisted proof template) without a clean
  pass-criterion.

### §8.2 Elgindi-NS adjacency

**Form**: argue that NS solutions at the C^{1,α}-near-1+ regularity
class inherit some Euler blow-up structure from Elgindi 2021,
i.e. that the NS dissipation ν Δu does **not** regularize the
Elgindi profile.

**Rejected because**:
- the question whether NS dissipation regularizes the Elgindi
  profile is the *open* axis-B question — generating a candidate
  that *assumes* one direction of the open question is circular
  in the F-AXISB-C sense;
- Elgindi 2021's α is small (α ≪ 1); the typical turbulent
  regularity is closer to C^{1/3-} or below, so the C^{1,α}
  class is *smoother* than turbulent regularity, and the
  adjacency argument runs in the wrong direction;
- a rigorous version of the adjacency argument would require
  developing new analytic machinery for NS at exactly the
  Elgindi regularity tier — which is a major research program
  in its own right, not a single-session candidate.

### §8.3 Vortex direction-frequency decomposition

**Form**: decompose ω into low-frequency ω_lo = P_K ω
(Littlewood-Paley projection on |k| ≤ K) and high-frequency
ω_hi = ω - ω_lo, track regularity per component, control
vortex-stretching cross-terms ω_lo · ∇u_hi etc.

**Rejected because**:
- the EXT-B validation §4.5 already demonstrated that the
  cross-term technique on the augmented (u, ρ, τ) state space
  produces uncontrolled residuals at the Hess(log ρ):∇u step;
- the Littlewood-Paley decomposition of ω generates
  vortex-stretching cross-terms ∫(ω_a · ∇)u_b · ω_c dx for
  a, b, c ∈ {lo, hi}, and these cross-terms are *known to be
  uncontrolled* without a CF-style depletion (this is the
  "paraproduct" approach to NS, Chemin 1998 *Perfect
  Incompressible Fluids* Oxford, Bahouri-Chemin-Danchin 2011
  *Fourier Analysis and Nonlinear Partial Differential
  Equations* Springer §3-§5);
- the candidate would inherit the same cross-term obstruction
  as EXT-B, just at a different decomposition.

### §8.4 BMO^{-1} initial-data extension (Koch-Tataru 2001)

**Form**: extend Koch-Tataru 2001 (Adv. Math. 157) BMO^{-1} small-
data global existence to a critical-norm bound on vortex direction.

**Rejected because**:
- Koch-Tataru's BMO^{-1} bound is on **velocity** at the initial
  time, not on **vortex direction** uniformly in time; the two
  are structurally different (BMO^{-1} ≈ ∂BMO via heat-extension
  characterisation);
- Koch-Tataru's smallness condition is the standard barrier; for
  large data, the global-existence question is open (the BT-544
  question);
- the candidate would reduce to "extend Koch-Tataru to large
  data", which is the open Koch-Tataru question — not a CF-axis-B
  question.

### §8.5 Constantin-Lax-Majda 1985 model adjacency

**Form**: use the Constantin-Lax-Majda 1985 model
∂_t ω = ω · Hω (H = Hilbert transform) as a 1D model of
vortex-stretching, and apply the model's known finite-time
blow-up to argue NS axis-B carries similar blow-up.

**Rejected because**:
- the CLM 1985 model is **1D** (scalar vorticity); the 3D
  vortex-stretching geometry is fundamentally different (the 1D
  "stretching" via Hilbert transform is non-local and
  unidirectional, whereas the 3D (ω·∇)u term has tensor structure
  via the strain rate S);
- the De Gregorio refinement (De Gregorio 1990 J. Stat. Phys.
  59) and the rigorous Chen-Hou-Huang 2021 CPAM proof close the
  CLM-De Gregorio model, but the 3D NS analog is not implied;
- using the CLM model as evidence for NS axis-B blow-up is the
  D1.1 candidate path (already FAILed at HVC validation,
  `omega-exec-bt544-d1-1-hvc-molt-validation-2026-04-25.md`).

### §8.6 Axisymmetric-with-swirl Euler standalone

**Form**: directly attempt the axisymmetric-with-swirl Euler
regularity problem (the D3.B standalone candidate that the
parent D3 strategy deferred per §7.4 anti-list).

**Rejected because**:
- this is the **maximally-hard** axis-B problem, the 50+-year-open
  question that the D3 seed §4.1 explicitly identified as
  "expected difficulty: maximal";
- a single-session candidate generation cannot close it; the
  candidate would be effectively a research-program-design
  document, not a discriminator-bearing candidate;
- the parent D3 strategy correctly deferred this candidate; the
  CF-BMO candidate engages the same axis but with a
  literature-import-friendly relaxation rather than a direct
  attack on the open problem.

### §8.7 Synthesis — why CF-BMO over the alternatives

Among the seven considered forms (CF-BMO + six rejected), CF-BMO
is the **only** option that:
- engages axis B in the positive direction (uniform regularity),
- relaxes a published hypothesis to a literature-natural weaker
  class,
- has an unambiguous discriminator (Path P / Q / R with explicit
  pass / fail criteria),
- registers three sharp falsifiers upfront (F-AXISB-A / B / C),
- does not assume one direction of the open NS regularity
  question.

The other six options either reduce to known-hard problems
(§8.1, §8.6), assume open questions (§8.2), inherit cross-term
obstructions (§8.3), reduce to other open questions (§8.4), or
operate in inappropriate models (§8.5).

---

## §9 Falsifiers active

### §9.1 Falsifiers for the CF-BMO candidate's verdict

The three §5 falsifiers (F-AXISB-A / B / C) are registered
upfront. Their post-generation status (before any validation
session) is:

| tag | falsifier | pre-validation status |
|-----|-----------|------------------------|
| F-AXISB-A | BMO-Lipschitz relabeling | NOT YET TESTED; expected NOT LIKELY |
| F-AXISB-B | Hou-Luo / Elgindi blow-up incompatibility | NOT YET TESTED; expected LIKELY (primary) |
| F-AXISB-C | premise circular w.r.t. NS regularity | NOT YET TESTED; expected POSSIBLE (secondary) |

A future validation session will fire one of these (or report
INCONCLUSIVE) per the §4 discriminator paths.

### §9.2 Inherited falsifiers from D3 + EXT-A/B/C

**F-D3-META-A** (axes-not-independent) — confirmed firing in D3.C
§5.4 and D3.B' §5.1. Relevance to CF-BMO: the candidate engages
**axis B alone** (positive direction), so F-D3-META-A's "axes
couple" observation is **honored**, not contradicted, by the
candidate. CF-BMO does not claim axis-B-cleanness in isolation;
it asks whether the CF depletion mechanism extends to BMO.

**F-EXTA-residual** (Helmholtz unsigned on vortex-stretching) —
confirmed firing in EXT-A validation. Relevance to CF-BMO: the
candidate is **specifically designed** to address this residual
by replacing the EXT-A variational structure with the CF 1993
geometric structure. F-EXTA-residual does not propagate to
CF-BMO.

**F-EXTB-D** (CI-2008 representation too weak) — confirmed firing
in EXT-B validation. Relevance to CF-BMO: the candidate does
**not** use the Constantin-Iyer 2008 representation; it uses the
direct Eulerian formulation of NS with the CF 1993 vorticity-
direction approach. F-EXTB-D does not propagate.

### §9.3 Meta-falsifiers for the candidate-generation step

**F-AXISB-META-A (candidate-not-axis-B-targeted)**: if the
CF-BMO hypothesis (3.2) does not actually engage the
vortex-stretching production (e.g. if (3.2) is a hypothesis on
something other than ξ = ω/|ω|, or if (3.3) is a depletion of
the wrong term), the candidate is mislabeled and should be
removed from the axis-B-targeted slot.

**Status**: NOT ACTIVE. (3.2) is on ξ = ω/|ω| explicitly, and
(3.3) is a depletion of the strain-vorticity production
(ω·S·ω) which is the standard form of vortex-stretching in
enstrophy balance.

**F-AXISB-META-B (literature already exhausted)**: if the BMO
relaxation of CF 1993 has *already* been studied and refuted in
a published paper that the literature scan missed, the candidate
generation is redundant.

**Status**: NOT ACTIVE per the §2 inventory (Constantin 1994,
CFP 1996, Chae 2003, Beirão da Veiga 2007/2009 stay in
Lipschitz / Hölder; no published BMO version surfaces). Risk of
missed paper exists; cross-check at validation extension.

**F-AXISB-META-C (atlas / state / inventory leakage)**: if any
change is made to atlas, state, or inventory files as a result
of this generation, the brief's hard constraint is violated.

**Status**: NOT ACTIVE. This generation edits no atlas / state /
inventory file. The git status at session start (specs and
inventory.json modified by *unrelated* prior sessions per the
header) is unaffected.

### §9.4 Falsifier-active summary

| tag | name | status |
|-----|------|--------|
| F-AXISB-A | BMO-Lipschitz relabeling | NOT YET TESTED |
| F-AXISB-B | blow-up incompatibility | NOT YET TESTED (expected LIKELY primary) |
| F-AXISB-C | premise circular | NOT YET TESTED (expected POSSIBLE secondary) |
| F-AXISB-META-A | candidate not axis-B-targeted | NOT ACTIVE |
| F-AXISB-META-B | literature already exhausted | NOT ACTIVE |
| F-AXISB-META-C | atlas/state/inventory leakage | NOT ACTIVE |
| F-D3-META-A (inherited) | axes-not-independent | confirmed firing; HONORED by CF-BMO |
| F-EXTA-residual (inherited) | Helmholtz unsigned | does not propagate to CF-BMO |
| F-EXTB-D (inherited) | CI-2008 too weak | does not propagate to CF-BMO |

---

## §10 Closing

**Candidate generated**: **CF-BMO** — a BMO relaxation of
Constantin-Fefferman 1993's Lipschitz vortex-direction
hypothesis, with conjectured logarithmically-corrected geometric
depletion of the vortex-stretching production.

**Setting**: 3D incompressible NS on T³, smooth Cauchy data,
positive viscosity ν > 0.

**Hypothesis (3.2)**: ξ = ω/|ω| ∈ BMO(supp(ω)) with logarithmic
modulus tied to ‖ω‖_{L^∞}.

**Conjectured conclusion (3.3)**: enstrophy growth bounded by a
log-corrected linear ODE, yielding double-exponential ‖ω‖_{L^∞}
control via BKM 1984.

**Discriminator type**: structural-literature (axis-B-specific),
running three paths P (literature for (3.2)), Q (literature for
(3.3) given (3.2)), R (sketch derivation).

**Falsifiers (registered)**:
- F-AXISB-A: BMO-Lipschitz relabeling (NOT LIKELY);
- F-AXISB-B: Hou-Luo / Elgindi blow-up incompatibility (LIKELY,
  primary);
- F-AXISB-C: premise circular w.r.t. NS regularity (POSSIBLE,
  secondary).

**Validation cost**: HIGH (~400+ pages of axis-B and
harmonic-analysis literature with proof-level depth on ~150
pages).

**Expected verdict**: **OBSTRUCTION_DOCUMENTED** via F-AXISB-B
firing (~60% probability) — the Hou-Luo / Elgindi self-similar
blow-up profiles plausibly violate the logarithmic-modulus BMO
constraint (3.2), so the candidate's premise fails on the known
blow-up scenarios.

**Cross-axis ties**: CF-BMO is the next non-redundant candidate
after the 3-pillar D3 localization + EXT-A / EXT-B / EXT-C
exhaustion. It implements the strategic shift from "find new
framework that bypasses axis B" to "directly engage the known
hard axis-B problem with a literature-natural relaxation".

**Anti-list**: six alternative axis-B candidate forms
(Hou-Luo numerical-bound, Elgindi-NS adjacency, frequency
decomposition, Koch-Tataru BMO^{-1} extension, CLM model
adjacency, axisymmetric-with-swirl standalone) considered and
rejected with specific reasons.

**Honest scope**: this is a HARD problem. CF-BMO is a
research-direction-design candidate, not a near-PASS. The axis-B
literature has 50+ years of partial results; any single
candidate is realistically expected to either reduce to a known
result or hit a known structural obstruction. The candidate's
value lies in registering the precise BMO-relaxation form so a
future validation can fire the correct falsifier rather than
repeat the literature scan.

**0/7 unchanged. NS regularity status open. No atlas / state /
inventory edits.** All cited references are pre-existing
(Beale-Kato-Majda 1984 Comm. Math. Phys. 94; Constantin-Fefferman
1993 Indiana Univ. Math. J. 42; Constantin 1994 SIAM Rev. 36;
Constantin-Fefferman-Procaccia 1996 Comm. PDE 21; Caffarelli-
Kohn-Nirenberg 1982 Comm. Pure Appl. Math. 35; Hou-Luo 2014
Multiscale Model. Simul. 12 / PNAS 111; Chen-Hou 2022
arXiv:2210.07191 + Chen-Hou-Huang 2021 CPAM 74; Elgindi 2021
Annals 194; John-Nirenberg 1961 Comm. Pure Appl. Math. 14;
Sarason 1975 Trans. Amer. Math. Soc. 207; Coifman-Lions-Meyer-
Semmes 1993 Annals 137; Stein 1993 Princeton (*Harmonic
Analysis*); Chae 2003 J. Diff. Equ. 195; Beirão da Veiga 2007
Z. Anal. Anwend. 26 / Beirão da Veiga-Berselli 2009 J. Diff.
Equ. 246; Koch-Tataru 2001 Adv. Math. 157; Constantin-Lax-Majda
1985 Comm. Pure Appl. Math. 38 + De Gregorio 1990 J. Stat.
Phys. 59; Vasseur 2007 SIAM J. Math. Anal. 39 / 2010 Arch. Rat.
Mech. Anal. 197; Chemin 1998 Oxford / Bahouri-Chemin-Danchin
2011 Springer; Frisch 1995 Cambridge; She-Leveque 1994 Phys.
Rev. Lett. 72; Ladyzhenskaya 1968 / Ukhovskii-Yudovich 1968 /
Chen-Strain-Tsai-Yau 2009; Leray 1934 Acta Math. 63).

— end candidate generation —
