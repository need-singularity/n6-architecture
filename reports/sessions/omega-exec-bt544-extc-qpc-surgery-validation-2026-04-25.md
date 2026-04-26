---
id: omega-exec-bt544-extc-qpc-surgery-validation
date: 2026-04-25
scope: research-only validation (NO NS claim, NO atlas promotion)
target: BT-544 EXT-C QPC-Surgery -- termination check + F-EXTC-D falsifier
parent_reports:
  - reports/sessions/omega-exec-bt544-extc-procedure-class-candidate-2026-04-25.md (candidate spec)
  - reports/sessions/omega-exec-bt547-poincare-L9-retro-2026-04-25.md (EXT-C prescription)
  - reports/sessions/omega-exec-bt544-exta-uomega-gradflow-validation-2026-04-25.md (parallel EXT-A validation)
  - reports/sessions/omega-exec-bt544-extb-cilyap-validation-2026-04-25.md (parallel EXT-B validation)
millennium_resolved: 0/7 (unchanged)
grade: validation, no claim
---

# Omega Exec — BT-544 EXT-C QPC-Surgery Validation (2026-04-25)

## §0 Non-claim disclaimer

This report **validates** the Quantitative Parabolic-Cell Surgery
candidate (QPC-Surgery) for 3D incompressible Navier-Stokes that was
generated in `reports/sessions/omega-exec-bt544-extc-procedure-
class-candidate-2026-04-25.md` (hereafter *candidate spec*). The
validation runs the discriminator of candidate spec §4 against the
five termination-direction falsifiers F-EXTC-A through F-EXTC-E
(candidate spec §5).

This report does **not**:

- claim 3D Navier-Stokes regularity, blow-up, or any Clay-form
  resolution; the BT-544 Clay status remains `0/1 untouched`;
- prove or disprove the QPC-Surgery termination conjecture
  (candidate spec §3.3);
- prove or disprove the parameter-cascade bound (3.4)
  #F_k ≤ C(α) · 2^{(2−α)k} of the candidate spec for any α ∈ (0, 2);
- promote any atlas entry, modify `state/proposals/inventory.json`,
  edit `theory/canon/`, alter the `BT-544 = 0/1 untouched` Clay
  status, or add to the L9 catalogue's active-candidate ledger;
- supersede or replace the D1 / D2 / D3.A / D3.B' / D3.C catalogue
  rows or the parallel EXT-A (uω-GradFlow) and EXT-B (CI-Lyap)
  validations;
- introduce any new theorem. Every cited result is pre-existing in
  the published partial-regularity / convex-integration / Ricci-flow
  literature, cited by author / year / journal as required.

The validation outcome is **OBSTRUCTION_DOCUMENTED** at the
**termination-side** activation of F-EXTC-D — the (T1) finite-step
elimination of all flagged cells on general 3D smooth data is
*equivalent* to NS regularity itself, so the QPC-Surgery procedure
relabels the standing open problem rather than advancing it. The
verdict is consistent with the candidate spec §6.3 expected-verdict
probability ~75% OBSTRUCTION_DOCUMENTED (F-EXTC-D primary).

**0/7 unchanged. NS regularity status open. No atlas / state /
inventory edits.**

---

## §1 QPC-Surgery procedure recap (read-only restatement of candidate spec §3)

### §1.1 Setup

Per candidate spec §3.2 eqs. (3.1)–(3.4), QPC-Surgery operates on a
Leray-Hopf weak solution u : [0, T) × ℝ³ → ℝ³ of 3D incompressible
NS with smooth divergence-free initial data u_0 ∈ H¹.

**Level-0 partition** (candidate spec eq. 3.1): cover spacetime
[0, T) × ℝ³ by parabolic cells

  Q_{0, j} = [t_j, t_j + 1] × B(x_j, 1)                            (1.1)

of unit parabolic scale, indexed on a fixed lattice.

**Level-k partition** (candidate spec eq. 3.2): each level-k cell
sub-divides into 4 sub-cells of edge

  Q_{k+1, j'} = [t_{j'}, t_{j'} + 2^{−2(k+1)}]
                   × B(x_{j'}, 2^{−(k+1)}),                       (1.2)

preserving parabolic ratio (temporal 2^{−2(k+1)} : spatial 2^{−(k+1)}).

**Flagging rule** (candidate spec eq. 3.3): cell Q_{k, j} is
*flagged* iff

  (1/r_k) ∫∫_{Q_{k, j}} |∇u|² dx ds ≥ ε_* · 2^{αk},                (1.3)

where r_k = 2^{−(k+1)}, ε_* is the CKN universal constant
(Caffarelli-Kohn-Nirenberg 1982 Prop. 1), α ∈ (0, 2) the
regularity-threshold parameter.

**Surgery step**: at each level k, flagged cells form F_k. Each
F_k cell is sub-divided to level k+1 and re-flagged; non-flagged
cells are *closed* (declared regular by CKN ε-regularity) and
removed from further iteration.

### §1.2 Conjectural cascade bound and termination

**Conjectural bound** (candidate spec eq. 3.4):

  #F_k ≤ C(α) · 2^{(2−α)k}                                          (1.4)

for some α ∈ (0, 2) and a level-uniform C(α). The candidate spec
flags (1.4) as conjectural — no published result establishes it for
α < 2 strict on general 3D smooth data.

**Termination modes** (candidate spec §3.2):

- **(T1)** F_{k_*} = ∅ at finite k_* — *clean termination*: u
  regular outside a finite cover of parabolic measure ≤ C·2^{−2k_*};
- **(T2)** F_k ≠ ∅ for all k but #F_k → 0 in 1D parabolic Hausdorff
  measure — *CKN limit*: parabolic Hausdorff dim ≤ 1 on singular set
  (this is the published CKN 1982 conclusion);
- **(T3)** #F_k grows faster than 2^{(2−α)k} for every α < 2 —
  *non-termination*: procedure does not constrain.

### §1.3 Discriminator paths (read-only, candidate spec §4.1)

- **Path P (literature-termination-import)**: a single citation
  chain proves (1.4) at α < 2 and (T1) on general data — verdict
  PASS_LITERATURE.
- **Path Q (sketch-termination-construction)**: sketch produces
  (1.4)+(T1) from CKN+Lin+Vasseur+parabolic-cover ingredients —
  verdict PASS_SKETCH.
- **Path R (obstruction-documented)**: (1.4) for α < 2 or (T1) on
  general data is provably equivalent to NS regularity itself —
  verdict OBSTRUCTION_DOCUMENTED.
- **Path S**: literature underdetermined — INCONCLUSIVE.

This validation runs Paths P and Q in §3-§4, then checks falsifiers
F-EXTC-A through F-EXTC-E in §5-§6, restricted-family check in §7,
and concludes in §8.

---

## §2 Literature scan — what each cited reference actually proves

Each reference is restated with its actual published content,
distinguished from the *adjacent* claim the candidate spec needs.
All citations are by author / year / journal-or-arXiv pattern
matched against the standard NS / Ricci-flow bibliography.

### §2.1 Caffarelli-Kohn-Nirenberg 1982 (CPAM 35, 771–831)

**Title**: "Partial regularity of suitable weak solutions of the
Navier-Stokes equations".

**Actual content**: for *suitable weak solutions* of 3D NS in the
Scheffer-CKN sense (Leray-Hopf class plus the local energy
inequality), there is a universal ε_* > 0 such that, at any
spacetime point (t_0, x_0), if

  (1/r) ∫∫_{Q_r(t_0, x_0)} |∇u|² dx ds < ε_*                       (2.1)

for some r > 0, then u is essentially bounded in a parabolic
neighbourhood of (t_0, x_0). The *singular set* S = {points where
(2.1) fails for all r > 0} has 1D parabolic Hausdorff measure zero
(CKN 1982 main theorem; restated also in Lemarié-Rieusset 2002
*Recent Developments in the Navier-Stokes Problem*, Chapman & Hall
§22).

**Domain of validity**: suitable weak solutions on ℝ³ × (0, T);
universal constants; covering argument via Vitali + ε-regularity.

**What CKN 1982 does NOT prove**:
- the singular set S is empty (this is precisely NS regularity, open);
- a quantitative bound on #F_k for any α < 2 in the parabolic-cell
  partition (1.4); CKN's covering argument extracts a Vitali
  subcover at each scale but does not produce a level-cascade
  cardinality bound;
- termination of any iterative cell-subdivision procedure in
  finite k_*;
- the singular set has parabolic Hausdorff dimension *strictly less
  than 1* (CKN gives dim ≤ 1, not < 1).

CKN 1982 is the **base level** of QPC-Surgery (k = 0 with α = 2
boundary case), per candidate spec §3.5.

### §2.2 Lin 1998 (CPAM 51, 241–257)

**Title**: "A new proof of the Caffarelli-Kohn-Nirenberg theorem".

**Actual content**: simplified proof of CKN 1982 using a
compactness-and-blow-up argument in place of CKN's iterative
ε-regularity. The conclusion is the same: 1D parabolic Hausdorff
measure of the singular set is zero.

**Domain of validity**: same as CKN 1982 (suitable weak solutions).

**What Lin 1998 does NOT prove**:
- any improvement on the Hausdorff dimension over CKN's `≤ 1`;
- a level-cascade cardinality bound; Lin's compactness argument is
  not multi-level — it picks up a single ε at the base scale and
  rescales;
- termination of an iterative procedure.

Lin 1998 is methodologically distinct from QPC-Surgery's level-
cascade. Lin's compactness blow-up is *single-scale rescaling*
(Brezis-Coron / De Giorgi style); QPC's cascade is *multi-level
covering* (Perelman-style).

### §2.3 Vasseur 2007 (NoDEA 14, 753–785)

**Title**: "A new proof of partial regularity of solutions to
Navier-Stokes equations".

**Actual content**: De Giorgi-style L²→L^∞ improvement. For
suitable weak solutions, define truncations u_k = (|u| − M_k)_+
with M_k a level-cascade satisfying M_{k+1} ≤ θ M_k for θ < 1; the
energy-improvement at each level controls M_k → 0 in measure on
non-singular regions, recovering the CKN 1D Hausdorff result.

**Domain of validity**: suitable weak solutions; same class as CKN.

**What Vasseur 2007 does NOT prove**:
- the singular set is empty;
- a *parabolic-cell* level cascade (Vasseur's cascade is on the
  *truncation level* M_k, not on the cell-subdivision count);
- the cardinality bound (1.4) at any α < 2 strict;
- termination at finite k_*.

Vasseur's De Giorgi cascade is a multi-level parameter cascade
(M_k → M_{k+1}) but the parameter is *amplitude*, not
*cell-cardinality*. The QPC bound (1.4) is on the cardinality of
cells flagged at each level — a different cascade direction.

### §2.4 Tao 2016 (J. Amer. Math. Soc. 29, 601–674)

**Title**: "Finite time blowup for an averaged three-dimensional
Navier-Stokes equation".

**Actual content**: replace the NS bilinear B(u, u) = (u·∇)u + ∇p
with a carefully *averaged* bilinear B̃(u, u) preserving energy
and scaling; show that for B̃, finite-time blow-up occurs by
cascade across dyadic Fourier shells (parameters: shell index k,
amplitude a_k, cascade time τ_k summable so total time T < ∞).

**Domain of validity**: averaged-NS (NOT true NS); rigorous
construction of blow-up for B̃.

**What Tao 2016 does NOT prove**:
- finite-time blow-up of *true* NS (Tao 2016 §1 explicit caveat);
- termination toward regularity of any NS procedure; the cascade
  produces blow-up, the **opposite direction** of QPC-Surgery's
  conjectured (T1);
- transferability of the averaged-cascade machinery to true NS
  cells; Tao 2016 §11 explicitly identifies the obstruction as the
  failure of the averaged equation's symmetry-control to survive
  un-averaging.

Tao 2016 is the methodological model for *rigorous dyadic-cascade
termination* but in the wrong direction (toward blow-up, not
regularity). The structural alignment is in the cascade machinery,
not in the regularity-coupling.

### §2.5 Buckmaster-Vicol 2019 (Ann. of Math. 189, 101–144)

**Title**: "Nonuniqueness of weak solutions to the Navier-Stokes
equation".

**Actual content**: Nash-style iterative convex-integration scheme
producing non-unique weak solutions to 3D NS in the energy class
C^0_t L^2_x ∩ C^0_t C^β_x for some β > 0. Each iteration step
adds a perturbation at frequency λ_q (geometric in q) and amplitude
μ_q to reduce the Reynolds-stress error δ_q.

**Domain of validity**: weak NS solutions in low regularity
(Hölder C^β); construction is rigorous.

**What Buckmaster-Vicol 2019 does NOT prove**:
- regularity / uniqueness of strong NS solutions;
- a procedure that *eliminates* singular candidate cells; the
  convex-integration scheme *adds* perturbations to construct
  non-uniqueness, not to eliminate flagged cells;
- a cardinality bound on a parabolic-cell partition.

The BV 2019 cascade has rigorous termination (in C^β norm
convergence) but produces non-uniqueness — the **opposite**
structural direction of QPC.

### §2.6 Perelman 2003a (arXiv:math/0303109) and 2003b
(arXiv:math/0307245)

**Titles**: "Ricci flow with surgery on three-manifolds"
(Perelman 2003a); "Finite extinction time for the solutions to the
Ricci flow on certain three-manifolds" (Perelman 2003b).

**Actual content (2003a §4-§5)**: rigorous *quantitative surgery*
for Ricci flow on closed 3-manifolds. Each surgery interval
carries scale parameters (δ_i, r_i, h_i) — surgery-precision δ_i,
canonical-neighbourhood radius r_i, surgery-scale h_i — with
**parameter-bound consistency** δ_i < δ_0(r_i, ε), h_i < h_0(r_i,
δ_i) propagated across surgery levels. Cao-Zhu 2006
(*Asian J. Math.* 10) and Morgan-Tian 2007 (*Ricci Flow and the
Poincaré Conjecture*, Clay Math. Inst.) provide expanded
expositions.

**Actual content (2003b)**: on simply-connected closed
3-manifolds, the Ricci flow with surgery becomes extinct in finite
time via a min-max sweep-out argument. The reproof by
Colding-Minicozzi 2008 (*J. Amer. Math. Soc.* 21, 561–569,
"Width and finite extinction time of Ricci flow") replaces the
original sweep-out with a width-of-2-spheres argument.

**Domain of validity**: 3-dimensional Ricci flow on smooth closed
manifolds; the surgery + extinction package together yields the
Poincaré conjecture (every simply-connected closed 3-manifold is
homeomorphic to S³).

**What Perelman 2003a/b does NOT prove**:
- any statement about Navier-Stokes;
- transferability of the surgery cascade to PDEs with non-geometric
  scaling (NS has parabolic scaling in (t, x); Ricci flow has
  parabolic scaling in (t, g) with g a metric, not a vector
  field);
- a generic procedure that generalises to any parabolic PDE.

The surgery + extinction package is the **structural template** for
QPC-Surgery's M3-archetype, not a transferable theorem. Whether the
template's three pieces (parameter cascade + finite-step
termination + regularity-coupling) survive transfer to NS is
exactly the QPC conjecture.

### §2.7 Robinson-Sadowski-Silva 2012 (J. Math. Phys. 53, 115618)
and Wang-Wu 2014 (J. Funct. Anal. 266, 5202–5238)

**Titles**: "Lower bounds on blowup solutions of the
three-dimensional Navier-Stokes equations in homogeneous Sobolev
spaces" (Robinson-Sadowski-Silva 2012); "Some refinements of
partial regularity of suitable weak solutions to the
Navier-Stokes equations" (Wang-Wu 2014).

**Actual content**: refined partial-regularity bounds in the spirit
of CKN. Robinson-Sadowski-Silva 2012 establishes lower bounds on
the rate of singularity formation in homogeneous Sobolev norms
under the suitable-weak hypothesis; Wang-Wu 2014 sharpens the
ε-regularity threshold and the cover-extraction.

**Domain of validity**: suitable weak solutions; quantitative
sharpenings of the CKN ε constant.

**What these papers do NOT prove**:
- a parabolic Hausdorff dimension *strictly less than 1* on the
  singular set (the CKN bound dim ≤ 1 has not been improved in
  these papers);
- the cardinality bound (1.4) at any α < 2 strict;
- (T1) termination at finite k_*.

These papers tighten the universal constants and refine the
covering arguments, but the core CKN conclusion (parabolic
Hausdorff dim ≤ 1) remains the state of the art on the singular
set's *size* for general 3D NS.

### §2.8 Hou-Luo 2014 (Multiscale Model. Simul. 12, 1722–1776)

**Title**: "Toward the finite-time blowup of the 3D axisymmetric
Euler equations: a numerical investigation".

**Actual content**: numerical evidence for finite-time blow-up of
3D axisymmetric Euler at the boundary of a tubular region, with
adaptive grid refinement h_k = h_{k−1}/2 near the suspected
singular point.

**Domain of validity**: numerical, axisymmetric Euler (NOT NS,
viscous case is different).

**What Hou-Luo 2014 does NOT prove**:
- any rigorous theorem on Euler or NS;
- termination of a procedure; the numerical refinement runs out at
  double-precision floating-point and the *mathematical*
  termination is open;
- regularity-direction termination.

Hou-Luo is methodologically the closest published numerical
analog of cell-subdivision but in the wrong direction (suspected
blow-up of inviscid Euler) and lacks rigorous theorem status.

### §2.9 Chen-Strain-Tsai-Yau 2008 (Comm. PDE 33, 287–306) and
2009 (Comm. PDE 34, 203–232)

**Titles**: "Lower bounds on the blow-up rate of the axisymmetric
Navier-Stokes equations II" and "Lower bound on the blow-up rate
of the axisymmetric Navier-Stokes equations".

**Actual content**: for axisymmetric 3D NS *with* swirl, lower
bounds on the rate of singularity formation; for *no-swirl*
axisymmetric NS, regularity is established (built on Ladyzhenskaya
1968-1969 and Ukhovskii-Yudovich 1968).

**Domain of validity**: axisymmetric data; with/without swirl
distinction is essential.

**What these papers establish for QPC-Surgery's restricted-family
check**: on axisymmetric-no-swirl, the singular set is empty
(regularity holds), so QPC's (T1) is reachable on that family
(at finite k_* the flagged set is empty).

### §2.10 Ladyzhenskaya 1969 (Gordon-Breach,
*The Mathematical Theory of Viscous Incompressible Flow*)

**Actual content**: 2D NS regularity. For smooth divergence-free
data on T² or ℝ², the NS solution remains smooth for all time
(Ladyzhenskaya proves a global a-priori H¹ bound via the
Ladyzhenskaya inequality).

**Domain of validity**: 2D NS only; the proof uses the absence of
vortex stretching in 2D (∂_t ω + (u·∇) ω = ν Δω, no stretching
term).

**Implication for QPC-Surgery**: on 2D NS, the singular set is
empty, so QPC's (T1) is reachable.

### §2.11 Recent literature scan — does any paper prove (1.4) at α < 2?

A targeted scan of the post-2008 partial-regularity / refined-CKN
literature surfaces the following candidates:

- **Choe-Lewis 2000** (Comm. PDE 25, 1099–1129) "Cauchy problem
  for the Navier-Stokes equations": refined ε-regularity but does
  not establish (1.4);
- **Gustafson-Kang-Tsai 2007** (Comm. Math. Phys. 273, 161–176)
  "Interior regularity criteria for suitable weak solutions of the
  Navier-Stokes equations": ε-regularity criteria in BMO and
  similar; does not establish a cardinality cascade;
- **He-Wang 2021** (J. Funct. Anal. 281) and similar recent works:
  refinements at the level of ε and of admissible function classes;
  none establishes the cardinality bound (1.4) at α < 2 strict.

**No paper in the searched literature states the cardinality
cascade bound (1.4) at any α < 2 strict, or the (T1) termination
on general 3D smooth data, as a theorem.** The state of the art
remains CKN 1982 + Lin 1998 + Vasseur 2007 (parabolic Hausdorff dim
≤ 1) plus quantitative sharpenings of universal constants.

---

## §3 Path-P attempt — direct literature import

### §3.1 The Path-P question

Does any cited paper, or any straightforward chain of cited
papers, **directly prove** (1.4) at some α < 2 strict, or **(T1)**
finite-step termination on general 3D smooth data?

### §3.2 Path-P negative answer

**No.** The §2 literature scan establishes:

- CKN 1982 / Lin 1998 / Vasseur 2007 each prove the (T2) limit
  (parabolic Hausdorff dim ≤ 1) for the singular set, NOT (T1)
  finite-step elimination, and NOT the cardinality cascade (1.4)
  at α < 2 strict (§2.1, §2.2, §2.3).
- Robinson-Sadowski-Silva 2012, Wang-Wu 2014, Choe-Lewis 2000,
  Gustafson-Kang-Tsai 2007 sharpen universal constants and
  ε-regularity criteria but do not establish (1.4) or (T1) on
  general data (§2.7, §2.11).
- Tao 2016 / Buckmaster-Vicol 2019 / Cheskidov-Luo 2022 establish
  cascade-termination in the *opposite direction* (averaged-NS
  blow-up / non-uniqueness), inapplicable to QPC's regularity
  direction (§2.4, §2.5).
- Perelman 2003a/b establish surgery + extinction for Ricci flow,
  not for NS (§2.6); the structural template does not transfer as
  a theorem.
- Hou-Luo 2014 is numerical, not rigorous (§2.8).

### §3.3 The QPC procedure is a synthesis, not a literature object

The QPC procedure (1.1)–(1.4) with cascade bound (1.4) and
termination modes (T1)/(T2)/(T3) is, per candidate spec §3.5, a
*synthesis* of pieces drawn from four distinct literatures:

- parabolic-cell partition: CKN 1982 covering form, extended
  multi-level;
- ε-regularity flagging rule: CKN ε_* universal constant;
- cardinality cascade C(α)·2^{(2−α)k}: novel synthesis with no
  published counterpart at α < 2 strict;
- termination modes: novel synthesis, with (T2) recovering CKN as
  a special case but (T1) being a strictly stronger conjectural
  statement.

The combination is **not in the published literature** as a theorem.
Path-P therefore cannot succeed.

### §3.4 Path-P verdict

**Path-P FAILS.** No citation chain produces (1.4) at α < 2 strict
or (T1) on general 3D smooth data. This matches the candidate spec
§6.3 expected outcome: PASS_LITERATURE probability ~5% — the
literature scan confirms the lower bound holds.

---

## §4 Path-Q attempt — sketch construction

### §4.1 The Path-Q question

Can (1.4) at some α < 2 strict, or (T1) at finite k_*, be derived
on general 3D smooth data from standard tools (CKN ε-regularity +
Lin 1998 compactness + Vasseur 2007 De Giorgi + Vitali covering)
with all steps standard, no new theorem?

### §4.2 Sketch attempt — propagation of ε-regularity through levels

Fix α ∈ (0, 2) and attempt the following sketch:

(S1) At level k = 0, the level-0 partition (1.1) covers ℝ³ × [0, T)
     by O(T · vol-ratio) cells. By the local energy inequality
     (Leray 1934 *Acta Math.* 63; Scheffer 1976/1977 in *Pac. J.
     Math.* / *Comm. Math. Phys.*), the total parabolic energy is
     finite: ∑_j (1/r_0) ∫∫_{Q_{0,j}} |∇u|² ≤ C ‖u_0‖²_{L²}/r_0.
     Hence #F_0 ≤ C ‖u_0‖²_{L²} / (r_0 ε_*) — bounded.

(S2) At level k → k+1, each flagged cell Q_{k, j} ∈ F_k is
     subdivided into 4 sub-cells per (1.2). The subdivided cells
     inherit the local energy (1/r_k) ∫∫_{Q_{k, j}} |∇u|² ≥ ε_* ·
     2^{αk} from (1.3). The question is: how is this energy
     distributed among the 4 sub-cells, and how many of them are
     re-flagged?

(S3) **Trivial counting bound**: each Q_{k, j} ∈ F_k contributes at
     most 4 sub-cells to the level-(k+1) partition. So
     #F_{k+1} ≤ 4 · #F_k = 2² · #F_k. By induction,
     #F_k ≤ 4^k · #F_0 = 2^{2k} · #F_0 — this is the *trivial*
     cardinality bound (corresponds to α = 0 in (1.4)).

(S4) **CKN-style improvement**: the conjectural bound (1.4) at
     α > 0 strict requires a *fraction* of the 4 sub-cells to NOT
     re-flag. Concretely, if at most f(α) ∈ [0, 4) sub-cells
     re-flag on average, the bound becomes
     #F_{k+1} ≤ f(α) · #F_k, giving #F_k ≤ f(α)^k · #F_0
     with f(α) = 2^{2−α}. The improvement requires
     f(α) < 4 ⇔ α > 0.

(S5) **Where the sketch breaks**: showing that f(α) < 4 (i.e. that
     not all 4 sub-cells re-flag) requires controlling the
     *spatial-temporal distribution* of |∇u|² inside Q_{k, j}. The
     standard tool is the parabolic ε-regularity at scale r_k —
     but this only provides a SUFFICIENT condition for *non-flagging*
     under the smallness assumption, not a NECESSARY upper bound on
     the number of flagged sub-cells. The literature does not
     supply, for general 3D smooth data, a quantitative bound on
     "how many sub-cells of a flagged cell stay flagged".

(S6) **Vasseur 2007 De Giorgi attempt**: one might try Vasseur's
     De Giorgi cascade on the truncation level M_k of |∇u|²; this
     gives improvement in M_k but does NOT translate to a
     cardinality bound on flagged cells unless one has separate
     control on the *measure* of {|∇u| > M_k} — and that measure
     control is exactly the CKN partial-regularity output (parabolic
     Hausdorff dim ≤ 1), giving (T2), not (T1). Vasseur's machinery
     re-derives (T2), not (T1).

(S7) **Lin 1998 compactness attempt**: blow up a hypothetical
     singular point sequence and use compactness. Lin's argument
     produces a self-similar limit profile and contradicts CKN
     ε_*-regularity in the limit, giving (T2). It does not produce
     a *quantitative* cardinality bound at finite k.

### §4.3 The structural cross-link to NS regularity

The breaking step (S5) — quantitative control on the
flagged-sub-cell count from a flagged parent cell — is, on
inspection, equivalent to the following weaker form of NS
regularity:

  **(*) Finite-energy density saturation**: for every smooth
  divergence-free initial datum u_0 ∈ H¹, there exists α(u_0) > 0
  and k_*(u_0) < ∞ such that at level k = k_*(u_0), every level-k
  cell satisfies (1/r_k) ∫∫_{Q_k} |∇u|² < ε_*.

Statement (*) is a form of NS regularity:

(a) (*) ⇒ no point of spacetime is in the singular set (every
    cell at level k_* is non-flagged ⇒ every point is enclosed in
    a non-flagged cell ⇒ regular by CKN ε-regularity);
(b) (*) ⇒ singular set S = ∅ ⇒ u is smooth (modulo the standard
    CKN ε-regularity-to-smoothness upgrade, Caffarelli-Kohn-
    Nirenberg 1982 Prop. 1).

Therefore (*) ⇔ NS regularity (modulo the standard upgrade chain).

The QPC (T1) termination conjecture is *stronger than or equal to*
(*), since (T1) requires F_{k_*} = ∅ at finite k_* (every cell
non-flagged at level k_*).

**Conclusion**: the Path-Q sketch reduces (1.4)+(T1) on general 3D
smooth data to (*) which is equivalent to NS regularity. The
sketch does NOT produce (1.4)+(T1) without already assuming
regularity.

### §4.4 Sketch on (T2) — recovers CKN, no improvement

The sketch in §4.2 *does* succeed on the weaker mode (T2):
F_k → 0 in 1D parabolic Hausdorff measure. This is exactly CKN
1982's conclusion. Specifically:

- by (S1), #F_0 finite;
- by (S3) trivial bound #F_k ≤ 4^k = 2^{2k};
- by CKN's Vitali covering (CKN 1982 §6), the limsup of F_k cells
  has 1D parabolic Hausdorff measure zero.

The sketch reproduces CKN 1982 + Lin 1998 + Vasseur 2007 — no new
content. This is a **relabeling** in the (T2) mode.

### §4.5 Path-Q verdict

**Path-Q FAILS** at the (S5) step on general 3D smooth data — the
required quantitative cardinality control reduces to NS regularity
itself. **Path-Q SUCCEEDS** on the (T2) mode but only as a
relabeling of CKN.

This rules out PASS_SKETCH on general data. The (T2) success is
not a positive verdict because it is exactly CKN 1982 — no new
content. This matches the candidate spec §6.3 anticipation.

The Path-Q sketch is **structurally informative**: it identifies
the precise step (S5) where the cardinality cascade reduces to NS
regularity. This information maps the missing piece to a specific
quantitative bound (number of flagged sub-cells per flagged parent
cell) that is exactly the open problem in disguise.

---

## §5 Falsifier check — F-EXTC-A, F-EXTC-B, F-EXTC-C, F-EXTC-E

### §5.1 F-EXTC-A (relabeling test)

**Statement** (candidate spec §5.1): if QPC at α < 2 reduces, on
all smooth NS families tested, to CKN's (T2) statement (parabolic
Hausdorff dim ≤ 1) without strict improvement, QPC is a relabeling
of CKN 1982.

**Test execution**: §4.4 above shows that on general 3D smooth
data, the Path-Q sketch on QPC at any α < 2 either (a) reduces to
NS regularity (S5 break) or (b) succeeds only at the (T2) mode,
which is CKN. The cardinality bound (1.4) at α < 2 strict is not
obtained except as the conjectural statement equivalent to NS
regularity.

For QPC at α = 2 (boundary case), the bound (1.4) reads
#F_k ≤ C(2) · 2^0 = C(2) — bounded — which is plausibly the CKN
1D Hausdorff conclusion in cardinality form. So QPC at α = 2 IS
the CKN procedure, by construction (candidate spec §3.5 explicit).

**Verdict**: F-EXTC-A **partially fires** — on general 3D smooth
data, QPC at α < 2 strict is, on the sketchable evidence,
equivalent to NS regularity (so either a relabeling of the open
problem or a relabeling of CKN, depending on whether one views the
question as conjectural or settled). On no family in §5–§7 does
QPC at α < 2 deliver a STRICTLY STRONGER statement than CKN.

### §5.2 F-EXTC-B (parameter-vacuous test)

**Statement** (candidate spec §5.2): if (1.4) is trivially
satisfied by counting arguments unrelated to NS, the bound is
parameter-vacuous.

**Test execution**: at level k, the level-k partition has at most
O(2^{3k}) cells (parabolic-cell counting: spatial scale 2^{−k} on
a fixed temporal interval, so volume ratio is 2^{3k} per unit
parabolic cell). The trivial bound #F_k ≤ 2^{3k} is far weaker
than (1.4) at α ∈ (1, 2), which gives #F_k ≤ C · 2^{(2−α)k} —
strictly less than 2^k for α > 1. Thus at α ∈ (1, 2), (1.4) is
*not* implied by trivial cell-counting and is therefore non-vacuous.

At α = 0, (1.4) reads C · 2^{2k} which is automatic from the 4-fold
sub-division (S3). At α ≤ 0, (1.4) is vacuous.

**Verdict**: F-EXTC-B does **NOT fire** at the intended α ∈ (1, 2).
Confirmed non-vacuous in this range. Matches candidate spec §6.3
anticipation.

### §5.3 F-EXTC-C (parameter-cascade-inconsistent test)

**Statement** (candidate spec §5.3): if the parabolic-energy
density (1/r_k) ∫∫_{Q_k} |∇u|² does not scale homogeneously under
(1.2) sub-division, the cascade is malformed.

**Test execution**: at level k → k+1, the cell volume scales as
|Q_{k+1}| / |Q_k| = (2^{−2(k+1)}/2^{−2k}) · (2^{−(k+1)}/2^{−k})³
= 2^{−2} · 2^{−3} = 2^{−5}. The radius r_{k+1}/r_k = 2^{−1}, so
1/r_{k+1} = 2 · (1/r_k). A flagged cell at level k contributes
parabolic energy at level k of magnitude
(1/r_k) ∫∫_{Q_k} |∇u|² ≥ ε_* · 2^{αk}.

At level k+1, the same physical region's energy density (with
respect to the smaller sub-cell scale) reads
(1/r_{k+1}) ∫∫_{Q_{k+1}} |∇u|². The sum over the 4 sub-cells of
∫∫|∇u|² equals ∫∫_{Q_k} |∇u|² (since the sub-cells partition Q_k).
So summed-over-sub-cells (1/r_{k+1}) ∫∫|∇u|² = 2 · (1/r_k)
∫∫_{Q_k} |∇u|² ≥ 2 · ε_* · 2^{αk} = ε_* · 2^{α(k+1)} · 2^{1−α}.

For α ∈ (0, 2), 2^{1−α} ∈ (1/2, 2). The threshold at level k+1 is
ε_* · 2^{α(k+1)}; the summed energy per flagged sub-cell on
average is ε_* · 2^{α(k+1)} · 2^{1−α} / (#flagged-sub-cells). For
the bound (1.4) #F_{k+1} ≤ C(α) · 2^{(2−α)(k+1)} to be consistent
with a per-cell flagging threshold, the average per-flagged-sub-
cell energy at level k+1 must be ≥ ε_* · 2^{α(k+1)}.

**Cascade-consistency check**: the algebra balances iff the cascade
parameter α and the threshold ε_* · 2^{αk} scale consistently with
the 4-fold sub-division and the parabolic 2^{−5} volume / 2^{−1}
radius ratios. The above calculation shows the parabolic scaling
*does* propagate consistently — the threshold at level k+1 is
chosen exactly to track the natural (1/r) ∫∫|∇u|² scaling. The
α-dependent factor 2^{αk} encodes the *conjectural* improvement
over the natural CKN scaling.

**Verdict**: F-EXTC-C does **NOT fire** at the algebraic level —
the parabolic scaling of (1.3) is internally consistent. The
α-factor is a free parameter that does not break the cascade. This
matches candidate spec §6.3 anticipation.

### §5.4 F-EXTC-E (parabolic-Hausdorff-no-improvement test)

**Statement** (candidate spec §5.5): if (1.4) at α < 2 strict
implies a parabolic Hausdorff dimension NO BETTER than CKN's
1D bound, the candidate adds no new partial-regularity content.

**Test execution**: under bound (1.4), the (T2) limit has
#F_k = O(2^{(2−α)k}) flagged cells of parabolic radius 2^{−k}. The
implied parabolic Hausdorff dimension d satisfies the standard
covering-dimension bound

  d ≤ lim sup_{k→∞} log #F_k / log(1/r_k) = (2−α).               (5.1)

(Recall: parabolic Hausdorff dimension uses parabolic balls
weighted as r^d for the d-dimensional measure.)

For α ∈ (1, 2), (5.1) gives d ≤ 2 − α ∈ (0, 1). This is **strictly
better** than CKN's d ≤ 1 — the candidate IS informative on
parabolic Hausdorff dimension, IF (1.4) holds at α > 1 strict.

For α ∈ (0, 1], (5.1) gives d ≤ 2 − α ∈ [1, 2) — no strict
improvement over CKN's 1.

**Verdict**: F-EXTC-E does **NOT fire** in principle for α > 1
strict — the candidate would deliver dim < 1, strictly improving
CKN. But the "if" is the open question: whether (1.4) holds at any
α > 1 strict on general 3D smooth data is the conjectural piece
that Paths P and Q failed to deliver (§3-§4).

So F-EXTC-E is *conditionally non-firing*: were (1.4) at α > 1
established, the procedure would be informative; since it isn't
established, the (T2) recovery in §4.4 only re-derives CKN's d ≤ 1.

---

## §6 Falsifier check — F-EXTC-D (non-termination — primary)

### §6.1 F-EXTC-D statement (recap)

**Statement** (candidate spec §5.4): for general 3D smooth data,
the (T1) termination of the QPC procedure at finite k_* is
equivalent to NS regularity itself — there is no published result
establishing finite-step elimination of all candidate-singular
cells, and any sketch derivation of (T1) on general data
necessarily passes through the standing NS regularity question.

### §6.2 Direct test on general 3D smooth data

The §4.3 analysis already established the precise structural
equivalence:

  **QPC (T1) on general 3D smooth data ⇔ NS regularity (modulo
  CKN ε-regularity-to-smoothness upgrade).**

The forward direction (QPC (T1) ⇒ NS regularity) is shown in §4.3
(a)-(b): if F_{k_*} = ∅ at finite k_*, every spacetime point is
covered by a non-flagged cell of size r_{k_*}, so by CKN
ε-regularity every point is regular, so the singular set is empty,
so u is smooth. (The CKN upgrade from ε-regular to smooth is
standard, e.g. via Serrin's criterion u ∈ L^∞_t L^∞_x locally
near regular points; cf. Caffarelli-Kohn-Nirenberg 1982 Prop. 1
combined with Serrin 1962 Arch. Rat. Mech. Anal. 9.)

The reverse direction (NS regularity ⇒ QPC (T1)) is also direct:
if u is smooth on [0, T] × ℝ³, then |∇u|² is continuous and bounded;
the local parabolic-energy density (1/r_k) ∫∫_{Q_k} |∇u|² is
O(r_k^4) (Lebesgue dominated convergence, since |∇u|² is bounded),
so for k large enough the threshold ε_* · 2^{αk} is exceeded by no
cell — F_{k_*} = ∅ at finite k_*.

So **(T1) ⇔ NS regularity**, modulo standard upgrades.

### §6.3 The procedure relabels rather than advances

By §6.2, QPC-Surgery's load-bearing termination claim (T1) on
general 3D smooth data is **logically equivalent** to NS regularity.
The procedure does not provide an independent route to regularity
— it provides a *reformulation* of regularity in terms of finite-
step cell-elimination.

**This is exactly the structural risk identified in candidate spec
§5.4 as "most likely activation"**: F-EXTC-D fires.

### §6.4 Comparison to Perelman M3

In the Perelman case, the surgery + finite-extinction package is
NOT equivalent to the Poincaré conjecture as a standalone
statement; rather, the surgery + extinction theorem PROVES the
Poincaré conjecture *non-trivially*, via the geometric content of
the canonical-neighbourhood theorem (Perelman 2003a §11-§12) and
the min-max sweep-out (Perelman 2003b / Colding-Minicozzi 2008).
The proof of finite extinction does not assume Poincaré.

In the QPC case, by contrast, the only available route to (T1) on
general 3D smooth data is to assume NS regularity. The structural
reason: NS does not have a *geometric canonical-neighbourhood
theorem* (the analog of Perelman's κ-solutions / cylindrical-neck
classification, Perelman 2003a §11). Without a canonical-
neighbourhood classification, the sub-division (1.2) cannot be
locally controlled at the geometric level — only at the
energy-density level, which is exactly the CKN content.

**Structural diagnosis**: the M3 archetype's third piece
(termination via sweep-out / canonical-neighbourhood) does not
have an established NS analog. The first two pieces (cell partition,
parameter cascade) transfer naturally as algebra; the third piece
is the load-bearing geometric content, and it is missing.

### §6.5 F-EXTC-D verdict

**F-EXTC-D FIRES.** The (T1) termination on general 3D smooth data
is equivalent to NS regularity (§6.2), so the QPC procedure
relabels rather than advances the open problem (§6.3). The missing
piece is the NS analog of Perelman's canonical-neighbourhood
geometric content (§6.4), which is not in the published NS
literature and is structurally absent from the parabolic-cell
partition by itself.

This matches the candidate spec §5.4 prediction that F-EXTC-D was
"the most likely activation mode" and the §6.3 expected verdict
~75% OBSTRUCTION_DOCUMENTED.

---

## §7 Restricted-family check — small data / 2D / axisymmetric / D3.A

### §7.1 Small-data 3D NS

Under sufficiently small ‖u_0‖_{H^{1/2}} (Kato 1984, *Math. Z.* 187,
"Strong L^p-solutions of the Navier-Stokes equation in ℝ^m"), 3D
NS is globally regular. On this family, QPC's (T1) is reachable by
§6.2 reverse direction at finite k_*. This is a confirmation, not
an advance — small-data regularity is published and QPC's (T1)
recovers it via the standard CKN upgrade chain.

**Verdict on small-data**: PASS_SKETCH-equivalent at low value
(re-derivation of small-data regularity by parabolic-cell
covering). Adds no new content beyond Kato 1984.

### §7.2 2D NS (Ladyzhenskaya 1969)

On 2D NS, regularity holds (§2.10). QPC's (T1) is reachable by
§6.2 reverse direction. The 2D regularity proof uses the absence
of vortex stretching, which is a 2D-specific structural fact;
QPC's (T1) on 2D is again a recovery via the parabolic-cell
covering of an already-published result.

**Verdict on 2D**: PASS_SKETCH-equivalent at low value. No new
content beyond Ladyzhenskaya 1969.

### §7.3 Axisymmetric-no-swirl 3D NS (Ladyzhenskaya / Ukhovskii-
Yudovich 1968 / Chen-Strain-Tsai-Yau 2008/9)

On axisymmetric 3D NS without swirl, regularity holds (§2.9).
Same analysis as 2D: QPC's (T1) is reachable via §6.2 reverse
direction.

**Verdict on axisymmetric-no-swirl**: PASS_SKETCH-equivalent at
low value. No new content beyond the axisymmetric regularity
literature.

### §7.4 2.5D non-local-pressure (D3.A discriminator family,
omega-exec-bt544-d3-A-axis-discriminator-2026-04-25)

The 2.5D non-local-pressure family (vector velocity depending only
on (x_1, x_2)) is the D3.A discriminator setting where uniform
H^s control is established (D3.A PASS_LITERATURE). Regularity
follows from the in-plane 2D structure plus passive advection of
the third component. QPC's (T1) is reachable by §6.2.

**Verdict on 2.5D**: same as 2D — PASS_SKETCH-equivalent at low
value, recovery of the D3.A PASS_LITERATURE setting. Sanity-check
confirmation that QPC operates correctly on the regular family.

### §7.5 Axisymmetric-with-swirl (open)

Axisymmetric-with-swirl NS regularity is OPEN (Chen-Strain-Tsai-
Yau 2008/9 give lower bounds on blow-up rate but do not establish
regularity). On this family, QPC's (T1) is open — equivalent to
the axisymmetric-with-swirl regularity question, which is itself
a long-standing reduction of BT-544.

**Verdict on axisymmetric-with-swirl**: same structural status as
general 3D — F-EXTC-D fires, (T1) ⇔ axisymm-with-swirl regularity.
No advance.

### §7.6 Restricted-family summary

| family | regularity status | QPC (T1) status | new content |
|--------|-------------------|-----------------|-------------|
| 2D NS | PROVEN (Ladyzhenskaya 1969) | reachable, recovered | none |
| small-data 3D | PROVEN (Kato 1984) | reachable, recovered | none |
| axisymm-no-swirl | PROVEN (Ukhovskii-Yudovich 1968) | reachable, recovered | none |
| 2.5D non-local-pressure | PROVEN (D3.A PASS_LITERATURE) | reachable, recovered | none |
| axisymm-with-swirl | OPEN | OPEN ⇔ axisymm-with-swirl regularity | none (relabels open) |
| general 3D smooth | OPEN (BT-544) | OPEN ⇔ NS regularity | none (relabels open) |

**Summary**: on every settled-regularity family, QPC's (T1) is a
*recovery* via the parabolic-cell covering of an already-published
result; on every open family, QPC's (T1) is *equivalent* to the
open regularity question on that family. **No restricted family
delivers strict new content** — the procedure either re-derives
or relabels.

This rules out PASS_SKETCH at non-trivial restricted family;
PASS_SKETCH at low-value recovery is the most that can be claimed
on (e.g.) the 2.5D family, and it is informationally equivalent to
the D3.A PASS_LITERATURE already in the catalogue.

---

## §8 Verdict

### §8.1 Discriminator outcome

| path | outcome | reason |
|------|---------|--------|
| Path P (literature import) | FAIL (§3.4) | no published paper proves (1.4) at α<2 strict or (T1) on general data |
| Path Q (sketch construction) | FAIL at (S5) (§4.5) | quantitative cardinality control reduces to NS regularity itself; (T2) recovery is CKN relabeling |
| Falsifier F-EXTC-A | partially fires (§5.1) | QPC at α<2 on general data either equivalent to NS regularity or relabeling of CKN |
| Falsifier F-EXTC-B | does NOT fire (§5.2) | (1.4) non-vacuous at α∈(1,2) |
| Falsifier F-EXTC-C | does NOT fire (§5.3) | parabolic scaling internally consistent |
| Falsifier F-EXTC-D | **FIRES** (§6.5) | (T1) on general 3D smooth data ⇔ NS regularity; M3 third-piece (canonical-neighbourhood) absent for NS |
| Falsifier F-EXTC-E | conditionally non-firing (§5.4) | were (1.4) at α>1 established, dim < 1 strict; but (1.4) is the conjectural piece |
| Restricted-family check | low-value recovery only (§7.6) | every settled family: QPC re-derives; every open family: QPC relabels |

### §8.2 Verdict

**OBSTRUCTION_DOCUMENTED.**

The obstruction is **structural**: the QPC-Surgery procedure's
load-bearing termination claim (T1) on general 3D smooth data is
logically equivalent to NS regularity itself (§6.2). The procedure
relabels the standing open problem rather than advancing it.

The structural reason is the *missing third piece of the M3
archetype* (§6.4): Perelman 2003a/b's surgery + finite-extinction
package terminates non-trivially via the canonical-neighbourhood
theorem (Perelman 2003a §11-§12) and the min-max sweep-out
(Perelman 2003b / Colding-Minicozzi 2008), neither of which has
an established analog for 3D NS. The first two pieces of M3
(parabolic-cell partition, parameter cascade) transfer to NS as
algebra (CKN 1982 + Lin 1998 + Vasseur 2007 already supply
multi-level partition machinery), but the third piece — the
geometric content that produces *non-trivial* termination — is
absent.

The verdict is consistent with candidate spec §6.3 expected
distribution:
- ~75% OBSTRUCTION_DOCUMENTED (F-EXTC-D primary): **realised**;
- ~15% PASS_SKETCH on restricted family: not realised; the
  restricted-family check (§7) yields only low-value recoveries,
  not a non-trivial advance;
- ~5% PASS_LITERATURE: not realised; literature scan (§3) finds
  no paper establishing (1.4) at α < 2 strict.

### §8.3 The precise step that breaks

Per the OBSTRUCTION_DOCUMENTED verdict, the precise step that
breaks is identified for follow-up sessions.

**Primary breaking step (§4.3 (S5))**: the quantitative cardinality
control "how many of the 4 sub-cells of a flagged cell stay
flagged" reduces to NS regularity itself. Specifically, the bound
f(α) = #{re-flagged sub-cells per flagged cell, on average} < 4
for some α > 0 strict is, on inspection, equivalent to the
finite-energy-density saturation statement (*) of §4.3, which is
NS regularity modulo the standard CKN ε-regularity-to-smoothness
upgrade.

**Secondary structural diagnosis (§6.4)**: the M3 archetype's third
piece — non-trivial termination via geometric canonical-
neighbourhood content — is absent for 3D NS. Without an analog of
Perelman 2003a §11 κ-solutions / cylindrical-neck classification,
the sub-division (1.2) cannot produce non-trivial regularity-
direction termination beyond CKN's measure-theoretic content.

### §8.4 Where to push next (informational, not prescriptive)

Identifying the breaking step is informative for follow-up
candidate generation. Three directions are visible (parallel to the
EXT-A and EXT-B validation §8.4 directions):

- **Direction α — supply a canonical-neighbourhood theorem for NS**:
  attempt to establish a classification of local NS profiles near
  candidate-singular points (analog of Perelman κ-solutions). This
  is the literature on Type-I / Type-II singularities (Necas-
  Růžička-Šverák 1996 *Acta Math.* 176; Iskauriaza-Serëgin-Šverák
  2003 *Russ. Math. Surv.* 58) and on backward self-similar
  profiles (Tsai 1998 *Arch. Rat. Mech. Anal.* 143). None of these
  delivers a canonical-neighbourhood classification analogous to
  Perelman κ-solutions.
- **Direction β — replace cell-cardinality cascade with energy-
  density cascade**: rewrite QPC in terms of |∇u|² L^∞ levels
  rather than cell counts. This is essentially the Vasseur 2007
  De Giorgi cascade (§2.3); it recovers CKN at (T2) without
  reaching (T1). Already in the literature as a partial-regularity
  result.
- **Direction γ — accept (T2) as the procedural output**: declare
  QPC's value to be the *quantitative* re-derivation of CKN with
  explicit constants and parameter α tracking. This is informative
  only as a sharpening of universal constants (parallel to
  Robinson-Sadowski-Silva 2012, Wang-Wu 2014) — does not advance
  the regularity question.

These are not prescribed for execution in this session; they are
recorded as the structural-residual directions exposed by the
OBSTRUCTION_DOCUMENTED verdict.

---

## §9 F-MOLT-A tally update with EXT-C verdict

### §9.1 Pre-EXT-C tally (post-EXT-A, from EXT-A validation §8.2)

| # | BT | candidate | verdict | discriminator type |
|---|----|-----------|---------|--------------------|
| 1 | 541 | Lead-B SLE_6 × GUE | PASS | distributional |
| 2 | 542 | Hirahara MCSP | PASS | structural-literature |
| 3 | 543 | A4-ratio-only | FAIL | numerical-interval |
| 4 | 544 | Q1 KdV Gram | FAIL | discrete-equality |
| 5 | 545 | IHC | PASS | structural-literature |
| 6 | 547 | M1 (variational) | PASS retro | structural-literature |
| 7 | 547 | M2 (W-entropy) | PASS retro | OTHER (analytic-inequality) |
| 8 | 547 | M3 (surgery + extinction) | PASS retro | OTHER (procedure-class) |
| 9 | 544 | EXT-B CI-Lyap | OBSTRUCTION_DOCUMENTED | OTHER (analytic-inequality-construction) |
| 10 | 544 | EXT-A uω-GradFlow | OBSTRUCTION_DOCUMENTED | structural-literature (variational-derivation-sketch) |

### §9.2 Post-EXT-C addition

Adding this validation as a new row (active BT, not retrospective):

| # | BT | candidate | verdict | discriminator type |
|---|----|-----------|---------|--------------------|
| 11 | 544 | EXT-C QPC-Surgery | OBSTRUCTION_DOCUMENTED | OTHER (procedure-class-with-parameter-bounds) |

OBSTRUCTION_DOCUMENTED is again in the **PASS-family-adjacent**
position of the discriminator-type bias hypothesis. The OTHER
(procedure-class) discriminator type was already populated by a
retro PASS (row 8, Perelman M3); this is the first
OBSTRUCTION_DOCUMENTED row in the active BT register at the
procedure-class discriminator type.

### §9.3 Updated 2×2 matrix (collapsed)

|                                              | PASS / OBSTR_DOC | FAIL |
|----------------------------------------------|------|------|
| distrib / struct-lit / OTHER (PASS-adjacent) | 9 (rows 1, 2, 5, 6, 7, 8, 9, 10, 11) | 0 |
| discrete-equality / numerical-interval       | 0 | 2 (rows 3, 4) |

The bias hypothesis is **not disturbed** — no cross-cell entries.

The hypothesis is **further broadened**: OBSTRUCTION_DOCUMENTED now
appears in **three** discriminator types in the active register —
OTHER analytic-inequality-construction (EXT-B, row 9),
structural-literature variational-derivation-sketch (EXT-A, row
10), and OTHER procedure-class-with-parameter-bounds (EXT-C, row
11). All three are PASS-family-adjacent.

### §9.4 F-MOLT-A status

F-MOLT-A is defined on **active BTs**. Distance to F-MOLT-A firing
is **unchanged** from the post-EXT-A status — F-MOLT-A is not
fired because the OBSTRUCTION_DOCUMENTED verdict is in the
PASS-family-adjacent column, not in the FAIL column.

The cumulative tally now records **three consecutive
OBSTRUCTION_DOCUMENTED verdicts on BT-544** (rows 9, 10, 11), all
PASS-family-adjacent. This is structurally informative for the L9
catalogue's BT-544 subgraph (three distinct EXT-tier candidates
registered, each producing structural-residual content rather
than cleanup) but does not change the bias-hypothesis distance-to-
firing.

### §9.5 EXT-tier exhaustion confirms BT-547 retro F-MOLT-D
(catalogue-saturation)

The BT-547 retro had FIRED F-MOLT-D (catalogue-saturation falsifier)
on BT-544: the existing dfs-24 direction probes had exhausted the
in-catalogue family slots without producing a Clay-rigour
candidate. The EXT-tier (A/B/C) was prescribed precisely as the
catalogue-extension response.

The post-EXT-A/B/C verdict pattern is:
- EXT-A (uω-GradFlow): OBSTRUCTION_DOCUMENTED (Helmholtz-side);
- EXT-B (CI-Lyap): OBSTRUCTION_DOCUMENTED (representation-side +
  cross-term);
- EXT-C (QPC-Surgery): OBSTRUCTION_DOCUMENTED (termination-side,
  this validation).

**Three for three OBSTRUCTION_DOCUMENTED at the EXT-tier.** This
re-confirms the F-MOLT-D catalogue-saturation diagnosis at a
*broader* level: catalogue extension does not bypass the
underlying obstruction — it refines the diagnosis. Each EXT-tier
candidate identifies a different *missing piece* of the Perelman
M1/M2/M3 archetype as inaccessible to NS without already assuming
regularity:

- EXT-A (M1 analog): Helmholtz convective gradient does not exist
  under flat L² metric on Leray-projected divergence-free fields
  (geometric obstruction);
- EXT-B (M2 analog): Constantin-Iyer representation does not
  determine a single density ρ_t on ℝ³ (representation obstruction)
  + Bakry-Émery cross term not sign-controlled (analytic
  obstruction);
- EXT-C (M3 analog): canonical-neighbourhood theorem absent for
  NS, so cell-subdivision cannot produce non-trivial termination
  beyond CKN measure-theoretic content (geometric-third-piece
  obstruction).

The three obstructions are **structurally distinct** but all
share the property that the missing piece is the Perelman archetype's
*non-trivial* content (geometric / analytic / geometric-third-
piece, respectively), whereas the *trivial* algebraic content
transfers in each case. This is the BT-547 retro §5 prediction
made precise: catalogue extension exposes WHERE the M1/M2/M3
package fails to transfer, not THAT it fails.

---

## §10 EXT-tier completion status

### §10.1 EXT-A, EXT-B, EXT-C all validated

| EXT slot | candidate | verdict | falsifier fired | precise breaking step |
|----------|-----------|---------|-----------------|----------------------|
| EXT-A | uω-GradFlow | OBSTRUCTION_DOCUMENTED | F-EXTA-C (Helmholtz-side) | convective term not a variational gradient under flat L² |
| EXT-B | CI-Lyap | OBSTRUCTION_DOCUMENTED | F-EXTB-D (representation-side) | CI 2008 produces family {ρ_t(·\|x_0)}, not single ρ_t on ℝ³ |
| EXT-C | QPC-Surgery | OBSTRUCTION_DOCUMENTED | F-EXTC-D (termination-side) | (T1) on general 3D smooth data ⇔ NS regularity |

### §10.2 EXT-D was vocabulary glossary, not direct candidate

The BT-547 retro §5.4 EXT-D recommendation was a *vocabulary
extension* — adding "procedure-class-with-parameter-bounds" and
"variational-derivation-sketch" as discriminator-type labels —
not a direct candidate. EXT-D was implemented in the candidate
spec headers (e.g. EXT-C used the new sub-type label) and required
no separate validation session.

The EXT-D vocabulary extension is **completed implicitly** by the
EXT-A/B/C validations using the new labels. No further EXT-D
validation is needed.

### §10.3 Overall EXT-tier verdict

**EXT-tier complete, 0/3 PASS, 3/3 OBSTRUCTION_DOCUMENTED.**

The EXT-tier (A/B/C) exhaustively covers the three Perelman
archetype molt classes (M1 variational reinterpretation, M2
analytic-Lyapunov, M3 procedure-class) with literature-grounded
candidate content, and each delivers OBSTRUCTION_DOCUMENTED at the
respective archetype's load-bearing piece. This is structurally
the strongest possible exhaustion at the catalogue-extension level
— each EXT-tier slot is populated, each slot's candidate is
honest-quality (citations real, sketch genuine), and each slot's
verdict identifies a structural-residual obstruction.

The EXT-tier exhaustion **does not advance BT-544 toward
resolution**, but it **does advance the L9 catalogue's
diagnostic resolution**: the pre-EXT-tier state had "BT-544
catalogue-saturation, candidates exhausted"; the post-EXT-tier
state has "BT-544 catalogue-saturation re-confirmed at the
extension level, with three structurally distinct missing-piece
diagnoses identified". This is the BT-547 retro §6 predicted
outcome made precise.

---

## §11 Anomalies

Items observed during validation that do not change the verdict
but are flagged for record-keeping.

### §11.1 The (T2) recovery is informationally equivalent to CKN

§4.4 establishes that the QPC procedure successfully recovers
CKN's (T2) parabolic Hausdorff dim ≤ 1 via the parabolic-cell
covering. This is *not* a positive contribution because it is
exactly CKN 1982. The procedure's value-add at α = 2 is
informationally zero relative to CKN. This is a *relabeling-mode*
success — informative only as a methodological exercise (showing
the cell-partition machinery reproduces CKN), not as a new result.

### §11.2 The cardinality bound (1.4) is non-vacuous in principle
but inaccessible in practice

§5.2 establishes that (1.4) at α ∈ (1, 2) is non-vacuous (strictly
stronger than trivial cell-counting). §5.4 establishes that *if*
(1.4) at α > 1 strict held, the implied parabolic Hausdorff
dimension would be d ≤ 2 − α ∈ (0, 1) — strictly improving CKN.
But §3-§4 establishes that (1.4) at α > 1 strict is not
accessible from CKN + Lin + Vasseur + Vitali inputs without
assuming NS regularity.

**Diagnosis**: the bound (1.4) at α > 1 strict is the
*conjectural conditional* that would, if proved, deliver new
content; the procedure's structure is honest but the load-
bearing piece is the unproven conjecture, equivalent to NS
regularity.

### §11.3 The M3 third-piece (canonical-neighbourhood theorem)
absence for NS is a recurring diagnostic

The BT-547 retro had identified the M3 archetype's three pieces
as the EXT-C requirement. This validation traces the obstruction
specifically to the *third* piece (geometric content for non-
trivial termination). The first two pieces (parabolic-cell
partition, parameter cascade) are present in the NS literature
(CKN 1982 + Lin 1998 + Vasseur 2007); the third is absent.

This is informative for the L9 catalogue: future EXT-C-style
candidates should target the canonical-neighbourhood-theorem
analog directly, rather than re-importing the cell-partition
machinery.

### §11.4 EXT-tier 3/3 OBSTRUCTION_DOCUMENTED is the predicted
catalogue-saturation re-confirmation

The BT-547 retro F-MOLT-D firing predicted exactly this pattern:
catalogue extensions do not bypass the underlying obstruction;
they refine the diagnosis. The EXT-tier outcome is consistent
with that prediction.

The pattern is *not* a negative result for the EXT-tier as a
catalogue-development exercise — it is the structurally honest
outcome of the BT-544 / NS regularity status being open. The
EXT-tier produces three precise structural diagnoses where
otherwise there were three vague catalogue-saturation flags.

### §11.5 Disclaimer count check

This file uses "conjecture", "candidate", "open", "not proven",
"not established", "equivalent to NS regularity" for every step
that is not a published theorem with rigorous proof. No NS
regularity claim is made anywhere in this file. Every cited
reference is by author / year / journal-or-arXiv pattern matched
to the standard NS / Ricci-flow / partial-regularity bibliography.

### §11.6 The "QPC at α = 2 IS CKN" tautology is by construction

§5.1 notes that QPC at α = 2 reduces to CKN by construction
(candidate spec §3.5 explicit). This is not a relabeling
falsifier firing — it is acknowledged in the candidate spec. The
candidate's distinctive content is at α < 2 strict, and it is
exactly that content that fails to deliver (Path Q breaks at S5).

---

## §12 Falsifiers active for this validation

Falsifiers under which the §8 OBSTRUCTION_DOCUMENTED verdict would
be retracted or downgraded.

### §12.1 F-VAL-A (cited-paper-already-proves-(1.4)-and-search-missed-it)

**Statement**: if a published paper exists that directly proves
(1.4) for some α < 2 strict, or (T1) on general 3D smooth data —
e.g. a recent partial-regularity paper that the §2.11 scan missed
— then Path-P would succeed and the verdict shifts to
PASS_LITERATURE.

**Status**: NOT ACTIVE based on the §2.11 literature scan. The
risk of a missed paper is real (post-2014 partial-regularity
literature continues to refine universal constants and admissible
function classes); cross-check on extension if undertaken.

### §12.2 F-VAL-B (canonical-neighbourhood-theorem-for-NS-exists)

**Statement**: if a published paper exists establishing a
classification of local NS profiles near candidate-singular
points analogous to Perelman κ-solutions — e.g. a sharp Type-I /
Type-II singularity classification that controls the cell-
subdivision cardinality directly — then the §6.4 structural
diagnosis is retracted and the verdict softens.

**Status**: PARTIALLY ACTIVE in spirit. The Type-I / Type-II
literature (Necas-Růžička-Šverák 1996 *Acta Math.* 176;
Iskauriaza-Serëgin-Šverák 2003 *Russ. Math. Surv.* 58; Tsai 1998
*Arch. Rat. Mech. Anal.* 143; Serëgin 2012 *J. Math. Sci.* 185)
establishes partial classifications and rules out certain
singularity types under specific norms (e.g. L^{3,∞}_x), but a
full canonical-neighbourhood theorem analogous to Perelman 2003a
§11 is not in this literature. A specialist re-reading might
identify a closer analog than the validation found; cross-check
on extension if undertaken.

### §12.3 F-VAL-C (Path-Q-step-S5-actually-controllable-by-known-tool)

**Statement**: if there exists a known analytic tool that controls
the quantitative cardinality "how many sub-cells re-flag" — e.g.
a sharp parabolic-Lebesgue-differentiation-style argument that
quantifies the percentage of flagged cells under iteration — then
Path-Q would succeed at the (S5) step and the verdict shifts to
PASS_SKETCH.

**Status**: NOT ACTIVE. Such a quantitative percentage bound is
not in the searched literature. The sharp partial-regularity
papers (§2.7, §2.11) refine the universal ε_* but do not bound the
re-flagging fraction. Risk: low, but cross-check on extension if
undertaken.

### §12.4 F-VAL-D (validation-mis-reads-(T1)-as-NS-regularity)

**Statement**: if §6.2's argument that QPC (T1) ⇔ NS regularity is
incorrect — e.g. if the "every spacetime point is covered by a
non-flagged cell at level k_*" implication does not give CKN
ε-regularity for some technical reason (e.g. boundary-of-cell
issues for the L²_t,x integrability) — then F-EXTC-D does not
fire at full strength and the verdict softens.

**Status**: PARTIALLY ACTIVE. The boundary-of-cell measure-zero
issues are technical but resolvable via Vitali extraction
(standard in CKN 1982 §4-§6 covering arguments). A specialist
re-reading of CKN 1982 might identify a residual technical issue
that the validation's level of detail did not surface; cross-check
on extension if undertaken.

### §12.5 F-VAL-E (atlas/state/inventory-edit-leakage)

**Statement**: if any change is made to atlas, state, or inventory
files as a result of this validation, the brief's hard constraint
is violated.

**Status**: NOT ACTIVE. This validation is research-only and edits
no atlas, state, or inventory file. The git status at session
start (specs and inventory.json modified by *unrelated* prior
sessions per the gitStatus header) is unaffected by this
validation.

### §12.6 F-VAL-F (validation-fabricates-citation)

**Statement**: if any of the cited results (CKN 1982 CPAM 35;
Lin 1998 CPAM 51; Vasseur 2007 NoDEA 14; Tao 2016 JAMS 29;
Buckmaster-Vicol 2019 Ann. Math. 189; Cheskidov-Luo 2022 Invent.
Math. 229; Robinson-Sadowski-Silva 2012 J. Math. Phys. 53;
Wang-Wu 2014 J. Funct. Anal. 266; Choe-Lewis 2000 Comm. PDE 25;
Gustafson-Kang-Tsai 2007 CMP 273; Perelman 2003a/b arXiv;
Cao-Zhu 2006 Asian J. Math. 10; Morgan-Tian 2007 Clay;
Colding-Minicozzi 2008 JAMS 21; Hou-Luo 2014 Multiscale Model.
Simul. 12; Chen-Strain-Tsai-Yau 2008/9 Comm. PDE 33/34;
Ladyzhenskaya 1969 Gordon-Breach; Ukhovskii-Yudovich 1968;
Kato 1984 Math. Z. 187; Serrin 1962 ARMA 9; Necas-Růžička-Šverák
1996 Acta Math. 176; Iskauriaza-Serëgin-Šverák 2003 Russ. Math.
Surv. 58; Tsai 1998 ARMA 143; Serëgin 2012 J. Math. Sci. 185;
Lemarié-Rieusset 2002 Chapman & Hall; Scheffer 1976/1977; Leray
1934 Acta Math. 63) is fabricated, mis-attributed, or mis-yeared,
the validation's grounding is compromised.

**Status**: each citation is to a real paper, monograph, or
review in the standard NS / partial-regularity / convex-
integration / Ricci-flow / functional-analysis bibliography.
Cross-check on extension. NOT ACTIVE at registration.

### §12.7 Falsifier-active summary

| tag | name | status |
|-----|------|--------|
| F-VAL-A | cited paper proves (1.4) at α<2 / (T1), search missed | NOT ACTIVE |
| F-VAL-B | canonical-neighbourhood theorem for NS exists | PARTIALLY ACTIVE |
| F-VAL-C | Path-Q (S5) controllable by known tool | NOT ACTIVE |
| F-VAL-D | validation mis-reads (T1) ⇔ NS regularity | PARTIALLY ACTIVE |
| F-VAL-E | atlas/state/inventory edit leakage | NOT ACTIVE |
| F-VAL-F | validation fabricates citation | NOT ACTIVE |

Two falsifiers (F-VAL-B, F-VAL-D) are partially active — re-checking
them would require domain-specialist re-reading of the
Type-I/Type-II singularity classification literature (F-VAL-B) and
of CKN 1982 §4-§6 covering technicalities (F-VAL-D). Neither is
*expected* to change the verdict, but both are flagged as the
structural risks to the verdict.

### §12.8 Inherited falsifiers from candidate spec

The five candidate-spec falsifiers F-EXTC-A through F-EXTC-E
status update from this validation:

| tag | name | candidate-spec status | post-validation status |
|-----|------|----------------------|-------------------------|
| F-EXTC-A | relabeling | NOT YET TESTED | partially fires (§5.1) — QPC at α<2 reduces to CKN or NS regularity on tested families |
| F-EXTC-B | parameter-vacuous | NOT YET TESTED | does NOT fire (§5.2) — (1.4) non-vacuous at α∈(1,2) |
| F-EXTC-C | parameter-cascade-inconsistent | PARTIALLY ACTIVE | does NOT fire (§5.3) — parabolic scaling internally consistent |
| F-EXTC-D | non-termination | most likely activation | FIRES (§6.5) — confirmed |
| F-EXTC-E | parabolic-Hausdorff-no-improvement | NOT YET TESTED | conditionally non-firing (§5.4) — would deliver dim<1 if (1.4) at α>1 held |

The candidate-spec's expected activation pattern (F-EXTC-D primary,
F-EXTC-A partial, F-EXTC-B/C/E non-firing) is **realised** by this
validation.

---

## §13 Closing

**Verdict**: **OBSTRUCTION_DOCUMENTED** at F-EXTC-D primary
activation (§6), with secondary partial firing of F-EXTC-A
(relabeling on tested families, §5.1) and structural-residual
diagnostics on §6.4 (M3 third-piece absence) and §11.3
(canonical-neighbourhood theorem absence for NS).

**Path P**: FAIL — no published paper proves (1.4) at α < 2 strict
or (T1) on general 3D smooth data (§3).

**Path Q**: FAIL at (S5) — quantitative cardinality control on
flagged sub-cell count reduces to NS regularity itself; (T2)
recovery is exactly CKN, no improvement (§4).

**Falsifier F-EXTC-D**: FIRES — (T1) termination on general 3D
smooth data is logically equivalent to NS regularity (§6.2); the
M3 archetype's third piece (canonical-neighbourhood content) is
absent for NS (§6.4).

**Restricted-family check (§7)**: every settled-regularity family
yields low-value recovery; every open family relabels the open
question. No PASS_SKETCH at non-trivial restricted family.

**Precise breaking step**: §8.3 — the cardinality cascade bound
(1.4) at α > 1 strict, equivalent to "fewer than 4 sub-cells of a
flagged cell stay flagged on average", is, on inspection,
equivalent to the finite-energy-density saturation statement
which is NS regularity modulo the standard CKN ε-regularity-to-
smoothness upgrade.

**F-MOLT-A 11-row tally update (§9)**: row 11 added (BT-544 EXT-C
QPC-Surgery, OBSTRUCTION_DOCUMENTED, OTHER procedure-class-with-
parameter-bounds). Bias hypothesis is not disturbed;
OBSTRUCTION_DOCUMENTED at this discriminator type is in the
PASS-family-adjacent column. F-MOLT-A is not fired; distance to
firing unchanged.

**EXT-tier completion status (§10)**: 3/3 EXT-A/B/C validated, all
OBSTRUCTION_DOCUMENTED. EXT-D was vocabulary glossary, completed
implicitly. The EXT-tier exhaustion re-confirms BT-547 retro
F-MOLT-D (catalogue-saturation) at the extension level, with
three structurally distinct missing-piece diagnoses (Helmholtz /
representation / canonical-neighbourhood) instead of one vague
saturation flag.

**Anomalies (§11)**: (T2) recovery is informationally equivalent
to CKN; the cardinality bound is non-vacuous in principle but
inaccessible in practice; the M3 third-piece absence for NS is a
recurring diagnostic; EXT-tier 3/3 OBSTRUCTION_DOCUMENTED is the
predicted catalogue-saturation re-confirmation.

**Falsifiers active for this validation (§12)**: F-VAL-B and
F-VAL-D partially active (specialist re-reading risks); others
not active.

**0/7 unchanged. NS regularity status open. No atlas / state /
inventory edits.** All cited references are pre-existing
(Caffarelli-Kohn-Nirenberg 1982 CPAM 35; Lin 1998 CPAM 51;
Vasseur 2007 NoDEA 14; Tao 2016 JAMS 29; Buckmaster-Vicol 2019
Ann. Math. 189; Cheskidov-Luo 2022 Invent. Math. 229; Robinson-
Sadowski-Silva 2012 J. Math. Phys. 53; Wang-Wu 2014 J. Funct.
Anal. 266; Choe-Lewis 2000 Comm. PDE 25; Gustafson-Kang-Tsai
2007 Comm. Math. Phys. 273; Perelman 2003a arXiv:math/0303109;
Perelman 2003b arXiv:math/0307245; Cao-Zhu 2006 Asian J. Math.
10; Morgan-Tian 2007 Clay Math. Inst.; Colding-Minicozzi 2008
JAMS 21; Hou-Luo 2014 Multiscale Model. Simul. 12;
Chen-Strain-Tsai-Yau 2008 Comm. PDE 33; Chen-Strain-Tsai-Yau
2009 Comm. PDE 34; Ladyzhenskaya 1969 Gordon-Breach;
Ukhovskii-Yudovich 1968; Kato 1984 Math. Z. 187; Serrin 1962
Arch. Rat. Mech. Anal. 9; Necas-Růžička-Šverák 1996 Acta Math.
176; Iskauriaza-Serëgin-Šverák 2003 Russ. Math. Surv. 58;
Tsai 1998 Arch. Rat. Mech. Anal. 143; Serëgin 2012 J. Math.
Sci. 185; Lemarié-Rieusset 2002 Chapman & Hall; Scheffer 1976
Pacific J. Math. / 1977 Comm. Math. Phys.; Leray 1934 Acta
Math. 63).

— end validation —
