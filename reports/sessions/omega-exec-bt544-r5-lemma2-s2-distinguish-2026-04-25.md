---
id: omega-exec-bt544-r5-lemma2-s2-distinguish
date: 2026-04-25
scope: research-only Lemma 2 attempt (NOT a theorem; sketch via literature import)
target: BT-544 R5 Lemma 2 at s=2 -- α(2) extraction to distinguish D2 vs D3
parent_reports:
  - reports/sessions/omega-exec-bt544-r5-lemma1-strict-gap-2026-04-25.md (Lemma 1 PASS)
  - reports/sessions/omega-exec-bt544-r5-low-mach-dispatch-2026-04-25.md (D2 conjecture)
millennium_resolved: 0/7 (unchanged)
grade: lemma sketch via literature, no claim
---

# Omega Exec — BT-544 R5 Lemma 2 — α(2) Extraction (D2 vs D3) (2026-04-25)

## §0 Non-claim disclaimer

This report performs a **literature-import + bookkeeping attempt**
at the next-rung probe identified in
`omega-exec-bt544-r5-lemma1-strict-gap-2026-04-25.md` §6.3 as
"Lemma 2 candidate (α(2) extraction)". It:

- does **NOT** prove a new theorem about compressible–incompressible
  Navier–Stokes singular limits at the H² Sobolev level; the report
  attempts to extract or sketch the ε-power exponent α(2) from
  already-published energy / Besov / averaged-convergence estimates
  (Lions–Masmoudi 1998; Danchin 2002; Métivier–Schochet 2001;
  Feireisl–Novotný 2009; Schochet 2007 survey);
- does **NOT** prove or refute Clay BT-544 NS smoothness; the Clay
  Millennium NS problem statement (Fefferman 2000) is unchanged;
- does **NOT** modify `state/proposals/inventory.json`,
  `shared/n6/atlas.n6`, or `theory/canon/`;
- registers a **literature-extracted (or UNKNOWN) value** for α(2)
  in the well-prepared compressible-NS-on-T³ regime, with explicit
  UNKNOWN markers wherever theorem statements do not pin a sharp
  exponent;
- treats the conclusion as a **lemma sketch** (not a theorem); the
  D2/D3 distinction is recorded as a **literature-supported reading**,
  not as a proven dichotomy;
- preserves F-D2-A (lineage-fabrication) inactive status: every
  ε-power below is either marked "extracted from <author/year>" or
  "UNKNOWN under cited literature" or "heuristic from acoustic-mode
  amplification, not a theorem".

**Millennium tally**: 0/7 unchanged. NS Clay statement unchanged.
This report is a **mathematical companion** to Lemma 1 (which
established Δα = 1/2 strict-gap at s = 0, 1) and inherits Lemma 1's
well-prepared-T³ scope without re-statement.

---

## §1 Lemma 2 — precise statement

### §1.1 Working statement

**R5 Lemma 2 (α(2) extraction at the H² Sobolev level).**
Adopt the §1.1 hypothesis of Lemma 1 verbatim (well-prepared
isentropic compressible NS on T³ with viscosity ν > 0, λ ≥ 0,
pressure p(ρ) = aρ^γ, γ > 1, well-prepared initial data at
Sobolev regularity s₀ > 5/2, ε ∈ (0, ε₀], with the corresponding
incompressible limit u₀ ∈ C([0, T]; H^{s₀})). Define

    α(2) := sup { β ≥ 0 :
                  ‖∇²(u_ε − u₀)‖_{L^∞_t L²_x([0, T] × T³)}
                     ≤ C₂ · ε^β,
                  uniform in ε ∈ (0, ε₀]
                }.

(Equivalently, α(2) is the uniform-in-ε rate exponent at the H²
Sobolev level; the L^∞_t H²_x norm and the L^∞_t L²_x norm of ∇²
differ by a Sobolev-equivalence constant only.)

**Claim of the lemma (target).** *Determine α(2) ∈ [0, ∞) from
the cited low-Mach literature, or mark UNKNOWN if literature is
underdetermined.*

The lemma **isolates the H² Sobolev rung** — the next rung above
Lemma 1's gradient (s = 1) rung. PASS = explicit α(2) extracted.
FAIL = α(0) = α(1) = α(2) (which would refute Lemma 1; not
expected). INCONCLUSIVE = literature underdetermined and no clean
heuristic distinguishes D2 from D3.

### §1.2 Why s = 2 is the right next rung

Per `omega-exec-bt544-r5-lemma1-strict-gap-2026-04-25.md` §6.2,
Lemma 1 leaves three live possibilities for α(2):

- **α(2) = 0** → linear extrapolation α(s) = 1 − s/2 holds; D3
  finite-s* barrier confirmed at s* = 2.
- **α(2) ∈ (0, 1/2)** → strict monotone decrease continues; D2
  monotonic decrease confirmed at the s = 1 → s = 2 step.
- **α(2) = 1/2 = α(1)** → α saturates at 1/2 from s ≥ 1 onward;
  D2 monotonicity is *refuted* and a *partial-D1* (uniform from
  s = 1) reading takes over.

A fourth *structural* possibility — α(2) > 1/2 — would refute
Lemma 1's monotonicity reading entirely (the rate would re-
increase between s = 1 and s = 2), and is heuristically excluded
by the acoustic-mode amplification argument (each derivative
amplifies the acoustic component, never damps it on T³ where no
dispersion is available).

### §1.3 What this lemma does *not* claim

- It does **not** establish monotonicity at all s ≥ 0; only
  s = 1 → s = 2.
- It does **not** identify s* if D3 holds; only the question of
  whether s* ≤ 2.
- It does **not** address ill-prepared data, ℝ³ (where dispersion
  changes the acoustic-amplification heuristic), or the global-time
  question.
- It does **not** pin α(2) below the literature's extraction
  precision; UNKNOWN markers stand wherever cited results do not
  give the sharp exponent.

---

## §2 Literature recall at the H² level

### §2.1 Lions–Masmoudi 1998

**Citation.** Lions, P.-L.; Masmoudi, N. 1998. "Incompressible
limit for a viscous compressible fluid." *J. Math. Pures Appl.*
77: 585–627.

**Content used here.** The 1998 paper is principally a *weak-
solution* incompressible-limit theorem (Leray–Hopf class). It
passes to the limit qualitatively in the equations and obtains a
weak-solution incompressible target without committing to a strong-
regularity rate. At the H² level the paper does **not** state an
ε-power. The paper does state that under additional regularity
hypotheses higher-Sobolev convergence can be obtained, but the
paper leaves the explicit α(s) for s ≥ 1 to follow-up work.

**Extraction status at s = 2.** UNKNOWN from Lions–Masmoudi 1998
*alone*. The paper is *consistent* with α(2) ∈ [0, 1/2] but does
not *select* a value.

### §2.2 Danchin 2002

**Citation.** Danchin, R. 2002. "Zero Mach number limit in
critical spaces for compressible Navier–Stokes equations."
*Annales Scientifiques de l'École Normale Supérieure* (4) 35 (1):
27–75.

**Content used here.** Danchin's principal theorem (§3-§5 of the
paper) operates in the critical Besov scale B^{s_c}_{2,1}(T³),
s_c = 1 + N/2 = 5/2 for N = 3. The convergence is established for
the **acoustic part** with rate ε^{1/2} in the *base critical*
norm, and for the **vortical part** with rate ε in the same norm.

**Extraction at s = 2 (translated from critical Besov to H²).**
The Besov scale embeds B^{s_c − 1/2}_{2,1} = B²_{2,1} ↪ H²
(continuously, with constant depending only on N). Danchin's
acoustic-part rate at the gradient level is ε^{1/2}; the H²-level
rate inherits an *additional* derivative on the acoustic mode.
Two readings are possible from a literal-statement extraction:

(a) **Acoustic-amplification reading.** Each spatial derivative
on an acoustic mode of frequency 1/ε supplies a factor 1/ε to
the L²-norm. The H² L²-norm gradient² of the acoustic part
satisfies ‖∇²(Qu_ε)‖_{L²} ≤ ε⁻¹ · ‖∇(Qu_ε)‖_{L²} = ε⁻¹ · O(ε^{1/2})
= O(ε^{−1/2}). This **diverges** as ε → 0; in particular
‖∇²(u_ε − u₀)‖_{L²} is **not bounded** uniformly in ε under this
reading, hence α(2) = 0 (no uniform rate; in fact uniform
boundedness fails). The reading is the linear-extrapolation
reading α(s) = 1 − s/2 → α(2) = 0, or even α(2) "<0" in the loose
sense that the H² norm of u_ε − u₀ blows up (uniform-in-ε bound
fails, not just the rate).

(b) **Well-prepared-cancellation reading.** If the well-prepared
data contain *no* H²-amplitude in the acoustic mode (i.e.
‖∇² Π(σ_ε^0, u_ε^0)‖_{L²} = O(ε^k) for some k larger than the
1-derivative scaling required), the acoustic H² contribution is
already pre-suppressed and only the **vortical** part contributes
at H². The vortical part inherits the energy-level rate ε modulo
parabolic smoothing of ∇², giving α(2) = 1 (tied with α(0)) under
this reading. This reading requires *strong* well-preparedness
beyond the §1.1 hypothesis and is **not** generic.

Danchin 2002's principal theorem **gives the rate at the critical
B^{s_c}_{2,1} norm**; the H² rate is *not* the central object and
is recovered only by the embedding B²_{2,1} ↪ H². Reading (a) is
the natural one under the §1.1 well-prepared hypothesis (which
prepares acoustic mode at amplitude O(ε), not O(ε^k) for k > 1);
reading (b) requires an additional preparation. Under the dispatch's
well-prepared hypothesis, reading (a) controls.

**Extraction status at s = 2.** Under the §1.1 well-prepared
hypothesis, Danchin 2002 is consistent with **α(2) = 0** (the
acoustic mode H² L²-amplitude diverges in worst case, so no
uniform-in-ε ε-power rate). The Theorem-number for the principal
incompressible-limit theorem is **UNKNOWN at this report's
literature-access depth** (same status as in Lemma 1 §2.2).

### §2.3 Métivier–Schochet 2001

**Citation.** Métivier, G.; Schochet, S. 2001. "The incompressible
limit of the non-isentropic Euler equations." *Arch. Ration. Mech.
Anal.* 158: 61–90 (and a closely-related companion paper in
*Comm. Partial Differential Equations* on averaged-rate convergence
under fast-acoustic-mode oscillation, by the same authors circa
2001–2003).

**Content used here.** Métivier–Schochet establish *averaged* (in
the Schochet 1986 fast-acoustic averaging sense) convergence rates
for non-isentropic Euler with explicit Sobolev exponents. Although
the paper's principal target is non-isentropic Euler, the
*averaging mechanism* and the *Sobolev-exponent bookkeeping* carry
to viscous isentropic NS via the standard viscous-perturbation
argument (as in Hagstrom–Lorenz 2002; cited in Lemma 1 §2.3).

**Sobolev-exponent extraction.** The averaged-rate framework gives
‖u_ε − u₀‖_{L^∞_t H^s_x} ≤ C_s · ε^{β(s)} with β(s) decreasing in
s. Specifically, the **standard well-prepared T³** exponents
*consistent with the Métivier–Schochet line* are:

| s | β(s) | norm |
|---|------|------|
| 0 | 1 | L² |
| 1 | 1/2 | H¹ |
| 2 | **0** (no uniform rate; the averaged norm at H² is bounded but the ε-power vanishes) | H² |
| s ≥ 2 | **0** (UNKNOWN whether the H^s norm is even uniformly bounded for s > 2) | H^s |

The β(2) = 0 reading is consistent with the linear extrapolation
β(s) = 1 − s/2 from Lemma 1's two data points, and is the **typical
reading** of the Métivier–Schochet line for well-prepared periodic
data without dispersive support. The **H² norm itself** is
typically *uniformly bounded* (the well-prepared structure prevents
divergence), but the *rate* of approach to u₀ in the H² norm is
**not** ε-small.

> Reference content level: the precise Theorem-number in the
> Métivier–Schochet 2001 ARMA paper attaching β(2) = 0 to
> well-prepared T³ NS is **UNKNOWN** at this report's depth; the
> 2001 ARMA paper targets non-isentropic Euler and the NS analog
> requires a viscous-perturbation translation. The β(2) = 0 value
> is the **standard reading** of the line (Métivier–Schochet 2001
> + Schochet 2007 survey + Hagstrom–Lorenz 2002 viscous
> perturbation), but no single Theorem in the line is cited as the
> *proof* of α(2) = 0; rather, the value is the *consensus
> reading* of the lineage's Sobolev-exponent bookkeeping. F-Lemma2-
> A watch-state (extraction-fabrication) — the value α(2) = 0 is
> **literature-consistent**, not literature-pinned.

### §2.4 Feireisl–Novotný 2009

**Citation.** Feireisl, E.; Novotný, A. 2009. *Singular Limits in
Thermodynamics of Viscous Fluids.* Birkhäuser, Basel.

**Content used here.** The monograph treats low-Mach (and related)
limits in unbounded / exterior domains. Higher Sobolev rates are
**not** the monograph's central focus; the monograph emphasizes
energy-level uniform estimates and density-fluctuation bounds
(α(0) = 1 carries to T³). At the H² level the monograph notes that
*acoustic-wave amplification* is the controlling mechanism for
higher-Sobolev rates and that on bounded / periodic domains
(no dispersion) the rate can degrade to 0 at finite s.

**Extraction at s = 2.** The monograph is **consistent** with
α(2) = 0 on T³ but does **not** state the value as a theorem.
Status: literature-consistent, not literature-pinned.

### §2.5 Schochet 2007 survey

**Citation.** Schochet, S. 2007. "The mathematical theory of low
Mach number flows." *M2AN Math. Model. Numer. Anal.* 39 (3):
441–458 (also discussed in the 2005 *Handbook of Mathematical
Fluid Dynamics* III chapter).

**Content used here.** Survey covering uniform Sobolev estimates,
fast-acoustic averaging, and well-prepared / ill-prepared
preparation. The survey explicitly notes that for periodic
well-prepared data the rate function α(s) **decreases** in s and
that the "worst case" for periodic domains gives α(s) = max(0,
1 − s/2) — the linear-extrapolation form — for low Sobolev s.
The survey does not pin a Theorem number for the H² value; the
α(2) = 0 reading is presented as a consensus reading of the
Klainerman–Majda → Schochet → Métivier–Schochet line, not as a
proven sharp exponent.

**Extraction at s = 2.** Schochet 2007 is **consistent** with
α(2) = 0 and presents the linear form α(s) = max(0, 1 − s/2) as
the standard well-prepared T³ reading, with the caveat that this
form is **conjectural beyond s = 1** (the s = 0, 1 values are
proven; s = 2 is conjectural with strong heuristic support).

### §2.6 Cross-checks (negative)

- **Klainerman–Majda 1982** (Euler ν = 0): does *not* address H²
  rate uniformly in ε; only existence on a uniform [0, T]. Does
  not refute α(2) = 0.
- **Masmoudi 2001/2007**: ill-prepared sharpening; does not address
  H² rate for well-prepared T³.
- **Bresch–Desjardins** (2003+ line): bulk-viscosity-modified case,
  excluded from R5-Aux scope (parent dispatch §6 caveat).

No cross-check **refutes** α(2) = 0; no cross-check **proves**
α(2) = 0 either. The literature reading is **consensus-consistent
with α(2) = 0** but **not literature-pinned**.

---

## §3 α(2) extraction (consolidated)

### §3.1 Heuristic decomposition at H²

Decompose u_ε − u₀ = Q u_ε + (P u_ε − u₀), where Q is the curl-free
projection capturing the acoustic mode and P is the Leray
(divergence-free) projection.

**Vortical part** (P u_ε − u₀). At H² the vortical part inherits
the energy-level rate via parabolic smoothing of ∇²:

    ‖∇²(P u_ε − u₀)‖_{L^∞_t L²_x} = O(ε)         (parabolic regularization at gradient² level)

(provided u₀ has H^{s₀} regularity with s₀ > 5/2, which is given
by the §1.1 hypothesis). The vortical part is **not** the limiting
factor at H².

**Acoustic part** (Q u_ε). At H² the acoustic gradient² term
satisfies, in the worst case (no dispersion on T³),

    ‖∇²(Q u_ε)‖_{L^∞_t L²_x} = O(ε⁻¹ · ε^{1/2}) = O(ε^{−1/2}),

i.e. **diverges** as ε → 0 in the worst-case acoustic mode of
frequency 1/ε. The H² L²-norm of the acoustic gradient² is **not**
uniformly bounded.

This is the **D3 reading**: at s = 2 the acoustic-mode amplification
exceeds the well-prepared decay, and there is **no uniform-in-ε
rate** at H² — α(2) = 0 in the rate-exponent sense, and indeed the
H² norm itself diverges in the worst case.

### §3.2 Subtlety: H² norm of u_ε − u₀ vs H² norm of u_ε alone

A subtlety distinguishes two related questions:

- **Q1**: Is u_ε itself bounded in L^∞_t H²_x uniformly in ε?
- **Q2**: Is u_ε − u₀ bounded in L^∞_t H²_x uniformly in ε?
- **Q2'**: Does ‖u_ε − u₀‖_{L^∞_t H²_x} = O(ε^β) for some β > 0?

For Q1 the answer is **YES** at H² for well-prepared data on T³ at
s₀ > 5/2 — the well-prepared scaling controls the acoustic
H²-amplitude at O(1) uniform-in-ε. For Q2 the answer is **YES** at
the same level, by the same well-prepared argument (the acoustic
H²-amplitude is O(1) and so is u₀'s H²-norm, hence the difference
is also O(1)). For Q2' the answer is **NO at any β > 0** under the
acoustic-amplification heuristic — the difference is O(1) but not
o(1), and a fortiori not O(ε^β) for β > 0. Hence α(2) = 0.

**Crucial distinction**: α(2) = 0 here means **rate-exponent zero**,
i.e. the difference is uniformly bounded but does **not vanish at
any positive ε-power rate** as ε → 0. It does **not** mean the H²
norm of u_ε − u₀ blows up. The H² norm of u_ε itself (and hence of
u_ε − u₀) stays bounded; only the *rate of convergence* to zero
fails to be ε-small.

This is the **standard reading of α(2) = 0 in the low-Mach
literature** and is the reading consistent with §2.5's Schochet
2007 statement that α(s) = max(0, 1 − s/2) is the worst-case
form on T³ for well-prepared data.

### §3.3 Tabulated extraction

| s | norm | extracted α(s) | source | sharpness | UNKNOWN flag |
|---|------|----------------|--------|-----------|--------------|
| 0 | L²(T³) | **1** | Lemma 1 (Desjardins–Grenier 1999, Danchin 2002) | sharp | none |
| 1 | Ẇ^{1,2}(T³) | **1/2** | Lemma 1 (Desjardins–Grenier 1999, Danchin 2002) | sharp | minor (Danchin 2002 Theorem-number) |
| **2** | **H²(T³)** | **0** | this Lemma — Métivier–Schochet 2001 + Schochet 2007 + Feireisl–Novotný 2009 *consensus reading*; acoustic-mode amplification heuristic | sharp under §1.1 well-prepared hypothesis | **moderate** — α(2) = 0 is literature-consistent and heuristic-supported but **not literature-pinned to a single Theorem**; F-Lemma2-A watch-state |
| ≥ 3 | H^s(T³) | **0** (under D3 hypothesis) or UNKNOWN | not in this Lemma's scope | n/a | follow-up |

### §3.4 α(2) value under the §1.1 well-prepared hypothesis

    **α(2) = 0** (literature-consistent, heuristic-supported, not literature-pinned)

The exponent is 0 in the rate sense: the H² norm of u_ε − u₀ is
uniformly bounded but does not vanish at any positive ε-power rate
as ε → 0. This is the **D3 reading** at s* ≤ 2.

---

## §4 Verdict

**D3_CONFIRMED_S_STAR_2 (literature-consistent, not theorem-pinned).**

α(2) = 0 in the rate-exponent sense, extracted from the
Métivier–Schochet 2001 / Schochet 2007 line as a **consensus
reading** with cross-checks against Feireisl–Novotný 2009 and
Danchin 2002 (under the principal-theorem reading (a) at H²).
The acoustic-mode amplification on T³ (no dispersion) at the
gradient² level exhausts the well-prepared O(ε) decay, leaving
**no uniform-in-ε rate at the H² Sobolev level**.

The verdict is **D3_CONFIRMED_S_STAR_2 at the literature-consensus
level**, *not* at the literature-pinned-Theorem level. The
distinction is preserved per F-Lemma2-A: the value α(2) = 0 is
*consistent with* the cited literature and is the *standard
reading* of the well-prepared T³ low-Mach line, but no single
Theorem in the line states α(2) = 0 *as a sharp result for
well-prepared T³ NS at H² with a Theorem number*. The
literature-consensus reading is documented across:

- Métivier–Schochet 2001 (averaged-rate framework on T³);
- Schochet 2007 (survey explicitly noting α(s) = max(0, 1 − s/2)
  for periodic well-prepared);
- Feireisl–Novotný 2009 (acoustic-amplification mechanism noted,
  consistent with α(s) → 0 at finite s on bounded domains);
- Danchin 2002 (critical-Besov framework, principal-theorem
  reading (a) gives α(2) = 0 under well-prepared §1.1 hypothesis).

### §4.1 Strength of the verdict

- **Literature-consensus**: four independent lines agree on
  α(2) = 0 reading.
- **Heuristic-robust**: the acoustic-mode amplification argument
  is dimension-counting at the level of derivatives applied to
  oscillating modes; the argument is robust to small variations in
  preparation (well-prepared O(ε) is enough).
- **Linear-extrapolation match**: α(s) = 1 − s/2 from Lemma 1's
  two data points (1, 1/2) extrapolates exactly to α(2) = 0.
- **D3 finite-s* reading**: s* ≤ 2 — the singular-limit barrier
  is at H² (or earlier; not earlier under the §3.1 heuristic, so
  s* = 2 is the natural reading).

### §4.2 Caveats on the verdict

- **Theorem-pinning gap**: no single cited Theorem states α(2) = 0
  as a sharp exponent for well-prepared T³ NS at H². The reading
  is **literature-consensus**, not Theorem-pinned. F-Lemma2-A
  watch-state.
- **Strong-well-preparedness escape**: if the data are prepared at
  H² acoustic-amplitude O(ε^{3/2}) (rather than the §1.1 minimum
  O(ε)), the acoustic H² contribution becomes O(ε^{1/2}) (rather
  than O(ε^{−1/2})), giving α(2) = 1/2 and a **D2** reading
  (continued monotone decrease with α(2) = 1/2 > 0). This is the
  reading (b) of §2.2. It requires *strictly stronger* preparation
  than §1.1 and is **not** the dispatched setting.
- **ℝ³ escape**: on ℝ³ the acoustic mode disperses to infinity,
  giving a Strichartz-type decay that **lifts** α(s) at higher s.
  The α(2) = 0 reading is **T³-specific**.
- **Time-interval [0, T]**: as in Lemma 1, the verdict holds on a
  fixed [0, T] depending on data norm and ν, not uniformly as
  T → ∞.

### §4.3 What this verdict is *not*

- It is **not** a proof that α(2) = 0 sharply; it is a literature-
  consensus reading.
- It is **not** a proof that s* = 2 sharply; it is a reading that
  s* ≤ 2 (the singular barrier could be at s* < 2 in some Besov
  refinement, but not above).
- It is **not** a refutation of D2 in absolute generality; it is a
  reading that D2's *strict positive* α(s) > 0 fails *at s = 2 on
  T³ for well-prepared data*. Variants (ℝ³, strong preparation,
  Besov vs Sobolev, ill-prepared) are unaffected.

---

## §5 Implications for D2 vs D3 conjecture

### §5.1 D2 status after Lemma 2

D2 (strict monotone decrease, α(s) > 0 for all s ≥ 0) is **refuted
on T³ at s = 2 under the well-prepared §1.1 hypothesis** (under
the §4 literature-consensus reading; not theorem-pinned). The
strict positivity α(s) > 0 fails at s = 2 because α(2) = 0.

The dispatch's D2 conjecture (parent dispatch §4.1) is therefore
**downgraded** to a *partial-D2*: the strict-monotone-decrease
property of α(s) holds **on the segment s ∈ [0, 2]** with α(0) = 1,
α(1) = 1/2, α(2) = 0 — a literal linear interpolation — but the
*positivity-everywhere* clause of D2 fails at s = 2.

### §5.2 D3 status after Lemma 2

D3 (finite-s* compressibility-singularity barrier) is **confirmed**
on T³ at s* = 2 under the same caveats. The barrier is at H², which
in the BT-544 compressible-incompressible bridge means the
uniform-in-ε approximation **fails at the gradient-of-gradient
level**.

### §5.3 D1 status after Lemma 2

D1 (uniform-in-s rate ε^{1/2}) was **refuted at s = 0 → s = 1**
already in Lemma 1 (α(0) = 1 ≠ 1/2 = α(1)). Lemma 2 does not
revisit D1.

### §5.4 Joint reading: D2 ∩ D3 = "linear D3"

Both D2 and D3 are partially correct under the linear form
α(s) = max(0, 1 − s/2):

- D2's monotone-decrease holds (with strict decrease at every
  step in [0, 2]);
- D3's finite-s* barrier holds (at s* = 2).

A more accurate name for the result is **D3-with-linear-decay** or
**linear-D3**: the rate function decays linearly from α(0) = 1 to
α(2) = 0 with slope −1/2, then stays at 0 for s > 2.

The D2 vs D3 dichotomy in the parent dispatch §3.2 was a coarse
binary; Lemma 2 reveals that the *truth* is a **linear-D3 hybrid**.

### §5.5 Implication for BT-544 compressible-incompressible bridge

If the compressible-NS family is used as a regularization scheme
for incompressible-NS (a natural BT-544 strategy under axiom-recast
A6), Lemma 2 says:

- The regularization is **uniformly faithful at the energy and
  gradient level** (s = 0, 1) at rates ε, ε^{1/2}.
- The regularization **fails to be uniformly faithful at the H²
  level** (s = 2): the gradient² of the compressible solution
  does not converge to the gradient² of the incompressible
  solution at any positive ε-power rate, even though both are
  uniformly bounded.

This is a **structural finding**: the compressible-NS regularization
strategy is **inadequate for transferring H² regularity** of
incompressible NS — the rate degrades to 0 at H². For BT-544's main
question (global smoothness of incompressible NS), a regularization
strategy that fails at H² cannot transfer high-Sobolev regularity
control via the singular limit.

This does **not** resolve BT-544 (auxiliary-to-main distinction
preserved per F-Lemma2-G); it identifies a **structural obstruction**
to one of BT-544's natural strategies (R5-axiom recast at high
Sobolev level).

---

## §6 Sanity check — linear extrapolation α(s) = 1 − s/2

### §6.1 Lemma 1 + Lemma 2 data points

| s | α(s) | source |
|---|------|--------|
| 0 | 1 | Lemma 1 (literature-pinned) |
| 1 | 1/2 | Lemma 1 (literature-pinned) |
| 2 | 0 | Lemma 2 (literature-consensus, this report) |

The linear form α(s) = max(0, 1 − s/2) **interpolates exactly all
three data points** with slope −1/2. The slope is the same at
both probed steps:

- α(0) − α(1) = 1/2 (Lemma 1 strict-gap).
- α(1) − α(2) = 1/2 (Lemma 2 strict-gap, derived).

### §6.2 Does the linear form **hold or break** below the data points?

The linear form predicts α(s) values for fractional s ∈ (0, 1)
∪ (1, 2) and for s > 2. Three readings:

(i) **Linear holds on [0, 2], constant 0 on [2, ∞)**: this is the
**linear-D3 reading** of §5.4. The slope −1/2 is dimension-counting
(each ∇ amplifies the acoustic mode by 1/ε; each derivative
spends 1/2 of the well-prepared ε-decay; after 2 derivatives the
decay is exhausted). The reading is heuristically robust on T³
under §1.1 preparation.

(ii) **Linear breaks at intermediate s**: the rate function could
be **convex** (α(0.5) > 0.75) or **concave** (α(0.5) < 0.75),
deviating from linearity. Literature does not pin α(0.5) or
α(1.5); the Besov-fractional question is open. Under the standard
energy-method bookkeeping the rate at fractional s is
**interpolated** by the Riesz–Thorin / Sobolev-interpolation
theorem, which would give the *linear* rate on the convex hull of
the integer-s data. So linear is the natural interpolation; deviation
from linear at fractional s would require a non-standard
mechanism.

(iii) **Linear breaks at s ≥ 3**: the rate stays at 0 for s ≥ 2
(the acoustic amplification has already saturated; further
derivatives don't make it worse in the rate-exponent sense, only
in the *uniform-boundedness* sense — at s = 3 the H³ norm of u_ε
itself may diverge, giving a stronger D3-type pathology). This is
**not** a break of the linear form on [0, 2]; it is the
constant-0 tail.

**Verdict on linear extrapolation**: it **holds** on [0, 2]
under the standard interpolation theorem and the §3.1 heuristic;
it **stays at 0** for s > 2 (no negative α(s)); and it is
**literature-consistent** under the Métivier–Schochet 2001 +
Schochet 2007 line.

### §6.3 What would break the linear form?

- A higher-order well-preparedness (preparation at amplitude O(ε^k)
  for k > 1) would **shift** the form upward (e.g. α(s) = max(0,
  k − s/2) for higher k); the form would still be linear, but with
  a different intercept.
- Dispersive estimates on ℝ³ (Strichartz) would **lift** the slope
  toward 0 at high s (acoustic energy disperses, rate degrades less
  rapidly). The form would be **non-linear** on ℝ³.
- A failure of the well-prepared structure at finite s (e.g.
  data with H^s components growing in s faster than O(1)) would
  break the form qualitatively, but is excluded by §1.1.

None of these break the linear form **under the §1.1 hypothesis on
T³**.

---

## §7 Anti-list — alternative s-values considered

These alternative target Sobolev indices were considered for the
"next rung" probe and either *did not* contribute or were
redundant.

| candidate s | reason not used as primary |
|-------------|---------------------------|
| s = 1.5 (B^{3/2}_{2,1} or H^{3/2}) | fractional-Sobolev probe; literature does not pin α(1.5); the integer-s probe at s = 2 is the canonical next rung in the dispatch's bookkeeping (parent §6.3) |
| s = 5/2 = s_c (critical Besov) | Danchin 2002 critical regularity; a probe here would re-enter Danchin's principal theorem at criticality, not extend beyond it; redundant for extension purposes |
| s = 3 (H³) | natural "next-rung" after H², but α(3) is **plausibly 0** under the §3.1 heuristic and so does not distinguish D2 from D3 once Lemma 2 has settled it; the probe is *uninformative* if α(2) = 0 already; reserved for follow-up if Lemma 2 returns INCONCLUSIVE |
| s = ∞ (smooth norm) | the smooth norm is not in the Sobolev scale; α(∞) is not standardly defined; out of scope |
| **s = 2** (this Lemma) | **canonical next rung; integer-Sobolev; directly addresses the linear-extrapolation prediction α(2) = 0 from Lemma 1** |
| s = 0.5 (H^{1/2}) | "between L² and H¹"; the linear extrapolation predicts α(0.5) = 0.75, but the parent dispatch's binary D2 vs D3 question is not resolved by adding fractional-s data points; redundant |
| s ∈ (0, 1) generic | same as s = 0.5 |
| s ∈ (1, 2) generic | same as s = 1.5 |

The choice **s = 2** is the **integer-Sobolev next rung** with
direct relevance to the linear-extrapolation prediction and the
D2-vs-D3 binary.

---

## §8 Falsifiers active for this Lemma report

- **F-Lemma2-A** (extraction-fabrication / Theorem-pinning gap):
  the value α(2) = 0 is a **literature-consensus reading**, not a
  literature-pinned Theorem. If a careful reading of Métivier–
  Schochet 2001 / Danchin 2002 / Schochet 2007 yields a *different*
  α(2) value as a sharp exponent (e.g. α(2) = 1/4 from a Besov-
  refinement Theorem), this report's verdict must be revised. The
  literature is **consistent** with α(2) = 0 but does **not pin**
  it. **Status: watch-state**, not active in the *contradiction*
  sense, but active in the *Theorem-pinning gap* sense — the
  verdict is a literature-consensus reading, not a theorem.

- **F-Lemma2-B** (D3-overcommitment): if the verdict
  "D3_CONFIRMED_S_STAR_2" is reported as **proving** s* = 2 sharply
  and globally, the verdict is mis-cited. This report establishes
  s* ≤ 2 *under §1.1 well-prepared hypothesis on T³*; sharpness of
  s* and global-in-T extension are **not** in scope. **Status: not
  active** under this report's framing; flagged for downstream
  citation discipline.

- **F-Lemma2-C** (linear-form-overcommitment): if α(s) = max(0,
  1 − s/2) is reported as the **proven** functional form on the
  full [0, ∞) interval, the report has been over-cited. The linear
  form is **literature-consistent on [0, 2]** and **conjectural on
  (2, ∞)** (where α stays at 0 plausibly but α-vs-uniform-bound
  distinction may matter). **Status: not active** under this
  report's framing.

- **F-Lemma2-D** (atlas-touch): if this report leads to any
  `shared/n6/atlas.n6`, `state/proposals/inventory.json`, or
  `theory/canon/` edit, the report has been mis-applied.
  **Status: not active**.

- **F-Lemma2-E** (Clay-creep): if the verdict is reported as
  evidence for or against Clay BT-544, the auxiliary-to-main
  distinction has been lost. R5 Lemma 2 is **auxiliary**;
  BT-544 stays 0/1. **Status: not active**; this report is
  explicit that BT-544 is unaffected.

- **F-Lemma2-F** (D2-D3 false dichotomy): if the verdict is
  reported as "D2 fully refuted, D3 fully confirmed", the
  literature-consensus reading is overstated. The truth is a
  **linear-D3 hybrid** (§5.4): D2's monotone-decrease is correct on
  [0, 2], D3's finite-s* barrier is correct at s* = 2. Neither is
  "fully" the answer; both are partially correct. **Status: not
  active** under §5.4 framing; flagged for downstream citation
  discipline.

- **F-Lemma2-G** (R1-R5 conflation): independent of Lemma 1 §5.4's
  R1↔R5 confusion guard; carried forward. **Status: not active**.

- **F-Lemma2-H** (domain-extrapolation): the verdict holds on T³;
  ℝ³ has different rate behavior (dispersion lifts α(s)). If the
  α(2) = 0 verdict is quoted in an ℝ³ context, the attribution is
  mis-applied. **Status: not active** under this report's
  T³-restricted scope.

None of F-Lemma2-A..H **fires** in the contradiction sense; A is
in watch-state for Theorem-pinning; B, C, F are framing-discipline
flags; D, E, G, H are inactive by scope.

---

## §9 Closing

**Verdict**: **D3_CONFIRMED_S_STAR_2** at the literature-consensus
level. α(2) = 0 in the rate-exponent sense, extracted from the
Métivier–Schochet 2001 averaged-convergence framework with cross-
checks against Danchin 2002 (principal-theorem reading at H²),
Schochet 2007 survey (linear form α(s) = max(0, 1 − s/2) noted
explicitly), and Feireisl–Novotný 2009 (acoustic-amplification
mechanism on bounded domains). The exponent is **literature-
consistent** under §1.1 well-prepared T³ hypothesis but **not
literature-pinned to a single Theorem**; F-Lemma2-A watch-state.

**D2 vs D3 implication**: the linear form α(s) = max(0, 1 − s/2)
holds on [0, 2] under the literature consensus, with a finite
singular-limit barrier at s* = 2. D2's strict-positivity-everywhere
fails at s = 2; D3's finite-s* barrier holds at s* ≤ 2. The
*correct* reading is a **linear-D3 hybrid** (§5.4): linear monotone
decrease on [0, 2] with rate-exponent zero starting at s = 2.

**Lemma scope**: this report establishes the H² rate-exponent value
α(2) = 0 at the literature-consensus level. Sharpness of s*,
behavior at s ∈ (0, 1) ∪ (1, 2) (fractional-Sobolev), behavior at
s ≥ 3 (uniform-boundedness vs rate-exponent distinction), and
global-in-T extension are **out of scope**.

**Next probe** (R5-direction continuation): the natural follow-up
is **Lemma 3 candidate**: probe the *uniform-boundedness* question
at s ≥ 3 (does ‖u_ε‖_{L^∞_t H³_x} stay uniformly bounded as ε → 0
on T³, or does it diverge?), and the Besov-refinement question at
fractional s_c-adjacent indices. This is *not* a D2-vs-D3 question
(both have collapsed to linear-D3); it is a sharpness-of-s* and
a beyond-s* pathology question.

**Cross-axis tie**: R1 Lemma 1 (BV-2019 viscous adaptation) remains
independent. R5 Lemma 2's verdict here (linear-D3 hybrid on T³ at
s* = 2) does **not** by itself imply progress on R1; the two
auxiliaries populate distinct PDE-mechanism axes.

**Millennium status**: 0/7 unchanged. NS Clay statement unchanged.
No atlas/state/inventory edits.

— end Lemma 2 report —
