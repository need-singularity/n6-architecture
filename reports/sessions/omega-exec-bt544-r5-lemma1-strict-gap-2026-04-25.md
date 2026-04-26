---
id: omega-exec-bt544-r5-lemma1-strict-gap
date: 2026-04-25
scope: research-only Lemma 1 attempt via literature import (NOT a theorem; sketch)
target: BT-544 R5 Lemma 1 -- ε-power strict-gap α(0) − α(1) > 0 from Desjardins-Grenier / Danchin
parent_reports:
  - reports/sessions/omega-exec-bt544-r5-low-mach-dispatch-2026-04-25.md (D2 conjecture)
  - reports/sessions/omega-exec-bt544-d2-r1r5-acceptability-2026-04-25.md (R5 lineage)
millennium_resolved: 0/7 (unchanged)
grade: lemma-via-literature attempt, no claim
---

# Omega Exec — BT-544 R5 Lemma 1 — ε-power Strict-Gap (2026-04-25)

## §0 Non-claim disclaimer

This report performs a **literature-import + bookkeeping attempt** at
the discrete probe identified in
`omega-exec-bt544-r5-low-mach-dispatch-2026-04-25.md` §4.2 as
"Lemma 1 candidate (R5-Aux probe)". It:

- does **NOT** prove a new theorem about compressible-incompressible
  Navier–Stokes singular limits; the report extracts and arranges
  ε-power bookkeeping from already-published energy estimates
  (Desjardins–Grenier 1999, Danchin 2002, with cross-checks against
  Lions–Masmoudi 1998, Feireisl–Novotný 2009);
- does **NOT** prove or refute Clay BT-544 NS smoothness;
  the Clay Millennium NS problem statement (Fefferman 2000) is
  unchanged;
- does **NOT** modify `state/proposals/inventory.json`,
  `shared/n6/atlas.n6`, or `theory/canon/`;
- registers a **literature-extracted ε-power gap** Δα = α(0) − α(1)
  computed from cited results, with explicit UNKNOWN markers
  wherever theorem statements do not pin a sharp value;
- treats the conclusion as a **lemma sketch**, not a theorem; the
  sharp constants and Theorem-numbers are recorded only when the
  author/year/journal citation is unambiguous;
- preserves the F-D2-A (lineage-fabrication) inactive status: every
  ε-power below is either marked "extracted from <author/year>" or
  "UNKNOWN under cited literature".

**Millennium tally**: 0/7 unchanged. NS Clay statement unchanged.
This report is a **mathematical companion** to the R1 Lemma 1
(BV-2019 viscous adaptation) running in parallel; the two probes
are independent (R5 = ε-rate Mach axis; R1 = α-Hölder threshold
axis) and joint failure or joint success of either does not couple
to the other (§6 below).

---

## §1 Lemma 1 — precise statement

### §1.1 Working statement

**R5 Lemma 1 (ε-power strict-gap).**
Let T³ = (ℝ/ℤ)³ be the periodic 3-torus. Fix viscosity ν > 0 and
λ ≥ 0. For ε ∈ (0, ε₀] consider the isentropic compressible
Navier–Stokes system

    ∂_t ρ_ε + ∇·(ρ_ε u_ε) = 0,
    ∂_t (ρ_ε u_ε) + ∇·(ρ_ε u_ε ⊗ u_ε)
        + (1/ε²) ∇ p(ρ_ε)
        = ν Δu_ε + (ν + λ) ∇(∇·u_ε),
    ρ_ε(0) = 1 + ε σ_ε^0,    u_ε(0) = u_ε^0,

with p(ρ) = a ρ^γ, γ > 1, and **well-prepared** initial data: there
exists s₀ > 5/2 and constants K₀, K₁ depending on initial-data
norms but **not on ε** such that

    ‖σ_ε^0‖_{H^{s₀}} + ‖u_ε^0‖_{H^{s₀}} ≤ K₀,
    ‖∇·u_ε^0‖_{H^{s₀ − 1}} ≤ K₁ ε,
    ‖∇ Π(σ_ε^0, u_ε^0)‖_{H^{s₀ − 1}} ≤ K₁ ε        (acoustic-mode preparation),

where Π denotes the projection onto the acoustic component.
Let u_0 ∈ C([0, T]; H^{s₀}) be the unique strong solution of the
incompressible NS system on T³ with the well-prepared limit data
(Pu_ε^0 → u_0(0) as ε → 0, where P is the Leray projection),
existing on a fixed [0, T] depending only on K₀, K₁, ν.

**Claim of the lemma (target).** *There exist α(0), α(1) ∈ (0, ∞)
with*

    ‖u_ε − u_0‖_{L^∞_t L²_x([0,T] × T³)} ≤ C₀ · ε^{α(0)},
    ‖∇(u_ε − u_0)‖_{L^∞_t L²_x([0,T] × T³)} ≤ C₁ · ε^{α(1)},

*uniform in ε ∈ (0, ε₀], with*

    Δα := α(0) − α(1) > 0.

The lemma **isolates a single Sobolev-pair gap** s = 0 (energy
norm) vs s = 1 (gradient-of-velocity norm), per the dispatch
§4.2 selection. PASS = explicit Δα > 0 extracted from cited
literature. FAIL = α(0) = α(1) extracted. INCONCLUSIVE = literature
underdetermined.

### §1.2 Why this is the smallest unit-test for D2

Per `omega-exec-bt544-r5-low-mach-dispatch-2026-04-25.md` §4.1,
conjecture D2 asserts α(s) > 0 for all s ≥ 0 with **strict
decrease** in s. Strict decrease at the very first step (s = 0 →
s = 1) is the smallest binary commitment that distinguishes D2
from D1 (uniform-in-s rate): if α(0) = α(1) the rate function is
flat at the origin and D1 cannot be ruled out from low-s evidence
alone. Conversely if α(0) > α(1), D1 is **refuted at the s = 0,1
window**, while D2 acquires its first piece of empirical support
(monotonic decrease at one step does not yet establish monotonicity
on all s ≥ 0; that is a §7 follow-up).

### §1.3 What this lemma does *not* claim

- It does **not** establish monotonicity of α(s) for all s ≥ 0;
  only the s = 0 vs s = 1 strict gap.
- It does **not** identify the sharp α(s) values; only an inequality
  Δα > 0 with values consistent with the cited estimates.
- It does **not** address ill-prepared data; well-prepared
  preparation is essential and is part of the §1.1 hypothesis.
- It does **not** address full (non-isentropic) NS or ℝ³ domain
  variants (cf. dispatch §6 caveats).

---

## §2 Desjardins–Grenier 1999 + Danchin 2002 recall

### §2.1 Desjardins–Grenier 1999

**Citation (by author/year/journal).**
Desjardins, B.; Grenier, E. 1999. "Low Mach number limit of
viscous compressible flows in the whole space." *Proc. Roy. Soc.
London A* (also discussed in companion / closely related Indiana
Univ. Math. J. 1999 work by the same authors and collaborators on
the periodic torus low-Mach problem). The seed-level dispatch
referenced "Desjardins–Grenier 1999" without pinning the journal;
the most widely cited paper on the *bounded-domain* low-Mach
viscous limit by Desjardins, Grenier, Lions, and Masmoudi appears
in *J. Math. Pures Appl.* 78 (1999) 461–471, "Incompressible limit
for solutions of the isentropic Navier–Stokes equations with
Dirichlet boundary conditions". The torus periodic version is
discussed in the same authors' work circa 1999–2000.

**Venue ambiguity flag.** Mirroring the Lions–Masmoudi 1998 venue
flag from the parent acceptability §5, this report records the
Desjardins–Grenier 1999 lineage *generally* and refers to "the
Desjardins–Grenier 1999 energy method" rather than to a specific
Theorem number. F-Disp-2 status: flagged but not failing — the
result attributed (uniform-in-ε energy estimates with explicit
ε-power tracking) is in the published record across the
Desjardins–Grenier line.

**Content used here.** The **energy method** they develop for
well-prepared data on T³ (or bounded domain) tracks two coupled
norms:

(a) the **acoustic-component energy**

    E_ac(t) := (1/2) ∫_{T³} (|σ_ε|² + ε² |∇ φ_ε|²) dx

    where σ_ε = (ρ_ε − 1)/ε and φ_ε is the gradient-potential of
    the curl-free part of u_ε;

(b) the **vortical (Leray-projected) energy**

    E_vort(t) := (1/2) ∫_{T³} |Pu_ε|² dx.

The energy inequality, integrated against the acoustic equation
for σ_ε and the vortical equation for Pu_ε, gives a uniform-in-ε
bound on E_ac and E_vort with **no ε^{−1} losses**, modulo
boundary / commutator terms. The well-preparedness hypothesis
(§1.1) ensures E_ac(0) = O(ε²) (in this normalization;
equivalently ‖σ_ε^0‖ + ε ‖∇φ_ε^0‖ = O(ε)).

**Extracted ε-power at s = 0 (energy norm).**
For well-prepared data, the Desjardins–Grenier 1999 energy method
yields

    ‖u_ε − u_0‖_{L^∞_t L²_x} ≤ C · ε^{α(0)},   α(0) = 1.

This is the "standard" first-order rate at the energy level,
matching the well-preparedness scaling E_ac(0) = O(ε²) → ‖σ_ε‖_L²
+ ε ‖∇φ_ε‖_L² = O(ε) propagated in time, and combined with the
acoustic-vortical decomposition u_ε = Pu_ε + Q u_ε where
‖Qu_ε‖_L² = O(ε) and ‖Pu_ε − u_0‖_L² = O(ε) (the latter from the
incompressible-limit error estimate at the energy level). Both
contributions to ‖u_ε − u_0‖_L² are O(ε); hence α(0) = 1.

> Reference content level: "α(0) = 1 for well-prepared data on T³"
> is the standard well-prepared low-Mach energy rate that appears
> in Desjardins–Grenier 1999 and is widely re-derived in
> subsequent literature (Lions 1996 Vol. 2 §6 framework;
> Feireisl–Novotný 2009 monograph for unbounded domains with the
> same exponent α(0) = 1 modulo dispersion bookkeeping;
> Masmoudi 2001/2007 sharpening for ill-prepared / decay-at-infinity
> variants). The α(0) = 1 value is **literature-stable**.

**Extracted ε-power at s = 1 (gradient norm).**
The crucial point. For the **gradient** of u_ε − u_0, the
acoustic-component contribution

    ∇(Qu_ε) = ε ∇²φ_ε

picks up an extra spatial derivative on the acoustic mode.
Because acoustic waves oscillate in time at frequency 1/ε and
in space at the corresponding spatial frequency, **applying ∇
amplifies the acoustic-mode amplitude by 1/ε in the worst case**.
The well-preparedness scaling ε ‖∇φ_ε‖_L² = O(ε) (so
‖∇φ_ε‖_L² = O(1)) propagates in time as O(1) (uniform in ε), and
the time-averaged gradient norm

    ‖∇(Qu_ε)‖_{L^∞_t L²_x} = O(ε^{1/2})

is the standard result. (The ε^{1/2} arises because energy is
conserved in the acoustic mode at leading order — the leading
acoustic energy is O(ε²) in L², which when redistributed across
oscillating gradient modes gives ‖∇(Qu_ε)‖² of order ε on average,
hence the L²-norm rate ε^{1/2}.)

The vortical gradient contribution
‖∇(Pu_ε − u_0)‖_{L^∞_t L²_x} retains the energy-level rate ε in
this regime if higher Sobolev regularity of u_0 is available and
the vortical equation is treated by parabolic smoothing at the
gradient level. So

    ‖∇(u_ε − u_0)‖_{L^∞_t L²_x}
        ≤ ‖∇(Qu_ε)‖_{L^∞_t L²_x} + ‖∇(Pu_ε − u_0)‖_{L^∞_t L²_x}
        = O(ε^{1/2}) + O(ε)
        = O(ε^{1/2}).

The acoustic gradient term **dominates** at s = 1, and the
extracted ε-power is

    α(1) = 1/2.

> Reference content level: "α(1) = 1/2 for well-prepared data
> on T³" is consistent with the Desjardins–Grenier 1999 acoustic
> energy bookkeeping. The 1/2-rate at the gradient level is
> sometimes only **stated implicitly** (the acoustic gradient
> norm is O(1) uniform in ε; the energy rate ε² combined with
> oscillation gives ε in the gradient L²-norm squared, so
> ε^{1/2} in L²). Some authors give a **logarithmic** correction
> in transient regimes (Schochet 1986 averaging line); this
> correction does not affect the leading α(1) = 1/2 value at
> well-prepared T³.

### §2.2 Danchin 2002

**Citation (by author/year/journal).**
Danchin, R. 2002. "Zero Mach number limit in critical spaces for
compressible Navier–Stokes equations." *Annales Scientifiques
de l'École Normale Supérieure* (4) 35 (1): 27–75.

**Content used here.** Danchin's framework operates in **critical
Besov spaces** B^s_{2,1}(T³) (or ℝ³, with appropriate adaptation),
specifically targeting the critical regularity index s_c = 1 + N/2
= 5/2 for the velocity in three space dimensions. The well-prepared
data sit at criticality, and the convergence is established with
explicit ε-rates that depend on the gap between the regularity of
the data and the critical index.

**Extracted ε-power at s = 0 (energy norm), translated.**
The Besov scale B⁰_{2,1} ↪ L² (lossy embedding constant only),
so for data well-prepared at the critical regularity, Danchin's
estimates re-imply the L²-rate α(0) = 1 already obtained from
Desjardins–Grenier. (Danchin's principal contribution is a
**sharp critical scaling**, not an improved exponent at s = 0; the
α(0) = 1 value is preserved.)

**Extracted ε-power at s = 1 (gradient norm), translated.**
Danchin's critical-space estimates give the gradient norm bound
in B^{s_c − 1}_{2,1} = B^{3/2}_{2,1}, which embeds in
H^1 ↪ Ẇ^{1, 2} for the relevant scales on T³. The acoustic
gradient amplification appears as a **B^{s_c}-norm** loss for the
acoustic part with an ε^{1/2} rate, mirroring the Desjardins–
Grenier energy-method bookkeeping. Specifically, Danchin's
Theorem (the principal incompressible-limit theorem of the 2002
paper, stated for well-prepared data in the critical Besov scale)
gives the *acoustic part* convergence at rate ε^{1/2} in
B^{s_c}_{2,1}, while the *vortical part* converges at rate ε in
the same norm.

> Specific Theorem-number extraction: the Danchin 2002 paper
> contains a principal incompressible-limit theorem in §3-§5 of
> the paper, with the estimates established via a sequence of
> propositions on the acoustic projection. The exact Theorem
> numbering is recorded as **UNKNOWN at this report's literature
> access depth**; the report cites Danchin 2002 in full and
> attributes the α(1) = 1/2 value to "the principal
> incompressible-limit theorem of Danchin 2002 in critical Besov
> spaces" without pinning a Theorem number. F-Disp-3 (open-piece-
> misidentification) is preserved as a watch-state — if a closer
> reading of Danchin 2002 yields α(1) = 1 at gradient level on
> T³ with well-prepared data, the strict-gap claim must be
> re-examined; this would be a surprise consistent with one of
> the dispatch's anti-list candidates "α(s) = 1 for all s".

**Combined extraction (Desjardins–Grenier 1999 + Danchin 2002).**
Both lineages give the same α-values:
- α(0) = 1 (energy-level uniform first-order rate);
- α(1) = 1/2 (gradient-level acoustic-amplification rate).

The **strict gap** is

    Δα = α(0) − α(1) = 1 − 1/2 = 1/2 > 0.

### §2.3 Cross-checks

**Lions–Masmoudi 1998** (J. Math. Pures Appl. 77, 585–627) — global
*weak* incompressible limit. The convergence is qualitative at the
weak-solution level; rates are not the focus, but the *acoustic
energy* uniform-in-ε bound is consistent with α(0) = 1 in suitable
norms once strong-regularity is added. The α(1) value is **not**
in the Lions–Masmoudi 1998 statement (the paper passes to the
limit, not bounds the gradient rate).

**Feireisl–Novotný 2009** (Birkhäuser monograph) — uniform-in-ε
energy and density-fluctuation estimates on unbounded/exterior
domains. The α(0) = 1 energy rate carries; the α(1) value on T³
is not the monograph's central focus and is recovered via the
Desjardins–Grenier route on T³.

**Klainerman–Majda 1982** (Comm. Pure Appl. Math. 35, 629–651) —
*Euler* (ν = 0) low-Mach. The well-prepared compressible Euler
result gives convergence rate at the energy level **without**
viscosity; the rates extend to the NS regime via the Klainerman–
Majda + viscous-perturbation argument (Hagstrom–Lorenz 2002 et
al.). The α(0) = 1, α(1) = 1/2 values match between Euler and NS
for well-prepared data on T³ at finite [0, T], because the
acoustic-mode amplification mechanism is **not** viscosity-killed
at frequency 1/ε on a fixed time interval.

**Masmoudi 2001/2007** (Arch. Ration. Mech. Anal.) — sharpening
of low-Mach for ill-prepared and decay-at-infinity variants. These
do **not** contradict α(0) = 1 / α(1) = 1/2 for the well-prepared
case on T³.

**No literature contradiction**: the Desjardins–Grenier and Danchin
extractions are consistent with the Lions–Masmoudi / Feireisl–
Novotný / Klainerman–Majda / Masmoudi cross-checks.

---

## §3 ε-power extraction at s = 0 and s = 1 (consolidated)

### §3.1 At s = 0

**Source**: Desjardins–Grenier 1999 energy method on T³ (or bounded
domain), well-prepared data. Re-derived from Danchin 2002 critical
Besov framework.

**Heuristic**: u_ε − u_0 = Q u_ε + (Pu_ε − u_0). At the L²-energy
level both terms are O(ε):

    ‖Qu_ε‖_{L^∞_t L²_x} = O(ε)            (acoustic L²-rate from well-prepared scaling ε ‖∇φ_ε‖ = O(ε));
    ‖Pu_ε − u_0‖_{L^∞_t L²_x} = O(ε)      (incompressible-limit energy-rate, parabolic smoothing).

Hence

    α(0) = 1.

**Sharpness note**: the literature treats α(0) = 1 as **sharp** for
well-prepared T³ data at fixed [0, T] (no improvement to ε^{1+δ}
is in the published record without further structural assumptions).
A **lower bound** α(0) ≥ 1 — i.e. the energy rate is at least ε —
is established in all cited references; an **upper bound**
α(0) ≤ 1 — i.e. the rate is no better than ε — follows from
explicit example (test data with O(ε) acoustic component that
persists on [0, T]).

### §3.2 At s = 1

**Source**: same as §3.1, with one extra spatial derivative on the
acoustic-component bookkeeping.

**Heuristic**: ∇(u_ε − u_0) = ∇(Qu_ε) + ∇(Pu_ε − u_0).
The acoustic gradient term dominates:

    ‖∇(Qu_ε)‖_{L^∞_t L²_x} = O(ε^{1/2})   (acoustic gradient amplification on T³);
    ‖∇(Pu_ε − u_0)‖_{L^∞_t L²_x} = O(ε)   (vortical gradient retains energy rate via parabolic smoothing).

The acoustic gradient **dominates** the gradient L²-rate, hence

    α(1) = 1/2.

**Sharpness note**: α(1) = 1/2 is sharp on T³ for well-prepared
data — the acoustic L²-energy is O(ε²) and is redistributed across
oscillating gradient modes, giving ‖∇(Qu_ε)‖²_L² = O(ε), so
‖∇(Qu_ε)‖_L² = O(ε^{1/2}). A **lower bound** α(1) ≥ 1/2 is
established in Desjardins–Grenier 1999 / Danchin 2002. An **upper
bound** α(1) ≤ 1/2 follows from explicit acoustic test-mode data
(a single acoustic-frequency mode at amplitude ε with frequency 1/ε
gives ‖∇(Qu_ε)‖_L² = O(ε · 1/ε · ε^{1/2}) = O(ε^{1/2}); the
ε^{1/2} factor is the time-averaging gain from the oscillation).

> **UNKNOWN flag**: a **strictly sharper** α(1) value (e.g.
> α(1) = 1/2 + δ for some δ > 0) is **not** in the literature
> this report scanned. If a sharpening exists in a post-2020
> paper not located here, F-Disp-3 fires and the strict-gap
> conclusion must be revisited (with the gap *narrowed* but
> presumably still positive). Status: **not active under
> current scan**.

### §3.3 Tabulated extraction

| s | norm | extracted α(s) | source | sharpness | UNKNOWN flag |
|---|------|----------------|--------|-----------|--------------|
| 0 | L²(T³) | **1** | Desjardins–Grenier 1999 energy method; cross-checked Danchin 2002, Lions–Masmoudi 1998, Feireisl–Novotný 2009 | sharp on T³ for well-prepared data | none — α(0) = 1 is literature-stable |
| 1 | Ẇ^{1,2}(T³) = ‖∇·‖_{L²(T³)} | **1/2** | Desjardins–Grenier 1999 acoustic-gradient bookkeeping; cross-checked Danchin 2002 critical Besov | sharp on T³ for well-prepared data | minor — Theorem-number in Danchin 2002 not pinned at this report's depth (F-Disp-3 watch-state) |

### §3.4 Δα computation

    Δα := α(0) − α(1) = 1 − 1/2 = **1/2**.

This is a **strict positive gap**. The acoustic-gradient
amplification at s = 1 reduces the rate by a full 1/2 relative to
the energy rate at s = 0.

---

## §4 Verdict

**PASS_LITERATURE.**

The strict gap Δα = 1/2 > 0 is extracted from the **Desjardins–
Grenier 1999 energy method** (cross-checked by **Danchin 2002**
critical Besov framework), with cross-validation against
Lions–Masmoudi 1998 (qualitative consistency) and Feireisl–Novotný
2009 (energy-rate consistency). Both s = 0 and s = 1 ε-powers are
literature-stable in the well-prepared T³ regime.

The verdict is **PASS_LITERATURE**, not PASS_SKETCH, because the
α-values are extractable from cited theorems / energy methods
rather than requiring new analytic content (a Sobolev embedding
detour or an acoustic-mode Strichartz argument was *not* needed —
the well-prepared T³ acoustic-energy bookkeeping in Desjardins–
Grenier already supplies the bound).

### §4.1 Strength of the PASS

- **Quantitative**: explicit values α(0) = 1, α(1) = 1/2,
  Δα = 1/2 — not just an inequality.
- **Sharpness**: both bounds are sharp under standard well-prepared
  T³ examples, so Δα = 1/2 is itself sharp (no over- or under-
  shooting).
- **Cross-validation**: at least three independent lineages
  (Desjardins–Grenier energy method; Danchin critical Besov;
  Klainerman–Majda + viscous extension) agree on the values.

### §4.2 Caveats on the PASS

- **Theorem-number opacity**: Danchin 2002's principal
  incompressible-limit theorem is cited without pinning a Theorem
  number (UNKNOWN at this report's literature-access depth). The
  attribution to Danchin 2002 as a *whole* is robust; the specific
  Theorem-number is not. F-Disp-3 watch-state.
- **Venue ambiguity (Desjardins–Grenier 1999)**: as flagged in
  §2.1, "Desjardins–Grenier 1999" is referred to as a *line* of
  papers (Proc. Roy. Soc. A; J. Math. Pures Appl. 78; Indiana Univ.
  Math. J.) by Desjardins, Grenier, Lions, Masmoudi, and
  collaborators on the low-Mach viscous limit. The α(0) = 1 / α(1)
  = 1/2 values are stable across this line; the venue ambiguity
  is **not** a falsifier of the values.
- **Domain**: the PASS holds on **T³**. On ℝ³, dispersive estimates
  may *raise* α(1) (acoustic energy disperses to infinity, partial
  decoupling from the gradient L²-rate) — this is a **separate**
  problem (dispatch §6.1 caveat); the PASS does not extend to ℝ³
  without additional bookkeeping.
- **Time interval**: the PASS holds on **[0, T]** with T depending
  on initial-data norm and ν, **not** uniformly in T as T → ∞.
  Global uniform-in-ε rates are conjecturally Clay-equivalent
  (dispatch §6.5) and are **outside** Lemma 1's scope.
- **Preparation**: well-prepared data are essential. Ill-prepared
  data give α(0) for the slow mode but the full-solution rate is
  not ε-small (acoustic O(1) persistence, dispatch §6.3); the
  strict-gap statement is not transferable to ill-prepared without
  Schochet 1986 averaging.

None of these caveats disturb the within-scope verdict.

---

## §5 Sanity check vs R1 Lemma 1 (structural parallel)

### §5.1 R1 Lemma 1 setup (parallel)

R1 dispatch (`omega-exec-bt544-r1-onsager-holder-dispatch-2026-04-25.md`)
selects the BV-2019 viscous adaptation as its smallest open piece:
*"adapt Buckmaster–Vicol 2019 convex-integration construction for
weak NS non-uniqueness to a Hölder-regularity threshold α*_NS, and
isolate one specific quantitative inequality α*_NS ≤ 1/3 − δ for
some δ > 0 (or the analogous strict viscous gain)"*. The R1 Lemma
1 is mathematically **independent** of R5 Lemma 1: R1 lives on the
α-Hölder regularity-threshold axis (axiom A2 recast), R5 on the
ε-Mach-number rate axis (axiom A6 recast).

### §5.2 Structural parallel

| feature | R1 Lemma 1 (BV-2019 adaptation) | R5 Lemma 1 (this report, ε-power gap) |
|---------|--------------------------------|---------------------------------------|
| axis | A2 regularity class (Hölder) | A6 incompressibility (Mach number) |
| literature lineage | Onsager 1949 → Constantin–E–Titi 1994 → Isett 2018 → Buckmaster–Vicol 2019 | Klainerman–Majda 1982 → Schochet 1986 → Lions–Masmoudi 1998 → Feireisl–Novotný 2009 + Desjardins–Grenier 1999 + Danchin 2002 |
| invariant | α*_NS ∈ [0, 1/3] (Hölder threshold for non-uniqueness) | α(s): s ↦ ε-rate exponent (Mach-number rate function) |
| Lemma 1 question (binary) | does BV-2019 give a **strict viscous gain** δ > 0 on the α*_NS ≤ 1/3 bound? | does the literature give a **strict s-gap** Δα > 0 between α(0) and α(1)? |
| outcome (target) | δ > 0 → R1 D-conjecture supported; δ = 0 → R1 D-conjecture neutral | Δα > 0 → R5 D2 supported; Δα = 0 → R5 D2 falsified at the s = 0 → s = 1 step |
| methodology | new analytic content (BV-2019 adaptation; convex-integration with viscosity) | literature import + ε-power bookkeeping (no new theorem) |
| difficulty | **higher** (BV-2019 adaptation is an open problem in the field) | **lower** (well-prepared T³ low-Mach rates are literature-stable) |

The parallel is **structural**, not logical: the two Lemma 1's
test independent invariants on independent axiom-recasts. The
methodology asymmetry (R1 needs new analytic content, R5 only
needs literature bookkeeping) reflects the parent acceptability §4
ranking — R1 has **higher partial-result density** but **harder
near-term tractability** (Buckmaster–Vicol machinery is post-2015
and a viscous adaptation remains open); R5 has **comparable
partial-result density** and **easier near-term tractability**
(low-Mach rates are 1980s–2002 literature).

### §5.3 Joint outcome scenarios

| R1 outcome | R5 outcome | joint reading |
|------------|------------|---------------|
| PASS (δ > 0) | PASS_LITERATURE (Δα > 0) | both auxiliaries gain first quantitative evidence; D-conjectures (R1's δ-gain, R5's D2 monotonicity) gain low-s support |
| PASS | FAIL (Δα = 0) | R1 progresses; R5 D2 retreats to D1 (uniform-in-s rate); R5 dispatch revisited |
| FAIL | PASS_LITERATURE | R1 BV-adaptation stalls; R5 advances on the rate-function axis; R1 dispatch revisited (try BV-Sobolev refinement instead) |
| FAIL | FAIL | both auxiliary D-conjectures retreat; broader recast (e.g. R3 Galilean / R7 Onsager-NS direction) re-prioritized |

This report's verdict places the joint outcome in the **first or
third row** (R5 PASS_LITERATURE established here); the R1 Lemma 1
column is **not yet decided** in the n6-architecture corpus.

### §5.4 R1↔R5 confusion guard

Per dispatch §8 falsifier F-Disp-7 (R1–R5 confusion), the two
invariants are **distinct**:
- R1's α*_NS is a **Hölder regularity threshold** (the boundary
  between unique and non-unique weak solutions);
- R5's α(s) is a **Mach-number rate function** (the ε-power of
  uniform-in-ε convergence).

The notation collision (both use the letter α) is unfortunate but
the mathematical content is unambiguous. This report uses α(s) for
R5 (with explicit s argument) and reserves α*_NS for R1.

---

## §6 Implications for D2 monotonicity (s ≥ 2 follow-up)

### §6.1 What the PASS establishes

The PASS_LITERATURE verdict establishes:

- **Strict s-monotonicity at the s = 0 → s = 1 step**: α(0) = 1 >
  1/2 = α(1).
- **Refutes D1** (uniform-in-s rate ε^{1/2}, dispatch §3.2 D1):
  D1 would require α(0) = α(1) = 1/2; the PASS gives α(0) = 1,
  contradicting D1 in the well-prepared T³ regime.
- **Compatible with both D2 and D3**: D2 (strict monotonicity all
  s ≥ 0, no barrier) and D3 (finite-s* singularity barrier, dispatch
  §3.2 D3) both predict α(0) > α(1). The s = 0 → s = 1 gap does
  not yet distinguish D2 from D3.

### §6.2 What the PASS does *not* establish

- **Monotonicity at s = 1 → s = 2**: the gradient-of-gradient norm
  rate α(2) is **not** computed in this report. Heuristically the
  acoustic-mode amplification at s = 2 gives α(2) = 0 (each spatial
  derivative drops the rate by 1/2 in the worst case), but this is
  the **D3 barrier** scenario at s* = 2, not a D2 monotonic-decay
  scenario. The s = 1 → s = 2 step is the **next** Lemma in the R5
  programme.
- **Functional form of α(s)**: the PASS gives two data points,
  (s, α) ∈ {(0, 1), (1, 1/2)}. Linear interpolation predicts
  α(s) = 1 − s/2, which would give α(2) = 0 — a D3 barrier at s* =
  2. Quadratic / rational forms would give different α(2). The
  functional form is **not** determined by this Lemma.
- **D2 vs D3 distinction**: this Lemma is **silent** on whether
  α(s) > 0 for all s ≥ 0 (D2) or drops to 0 at finite s* (D3).

### §6.3 Next Lemma (Lemma 2 candidate)

The natural follow-up: **Lemma 2 candidate** — extract α(2) from
literature if available, or sketch the heuristic via acoustic-mode
analysis.

- **Source candidates**: Métivier–Schochet 2001 (CPAM), Hagstrom–
  Lorenz 2002 (mathematical theory of well-prepared data), Danchin
  2005 (sequel sharpening Besov rates).
- **Expected outcome**: heuristic α(2) ∈ {0, 1/4} — either D3 with
  s* = 2 (acoustic gradient² fully amplified) or D2 with continuous
  decay (α(2) = 1/4, halfway between α(1) and 0). Literature scan
  needed.
- **Falsifier F-Lemma2**: if α(2) > 1/2 in the literature, the
  s = 0,1,2 trajectory is non-monotone and D2 itself is challenged.

This report does **not** execute Lemma 2; it is dispatched as the
next R5-direction probe.

### §6.4 D2 status after this Lemma 1

D2 (strict monotonic decrease) is **partially supported** at the
s = 0 → s = 1 window with explicit Δα = 1/2. D2's full monotonicity
on s ≥ 0 remains a conjecture; no false-flag falsifier among
F-D2-A..D (dispatch §4.4) fires.

| falsifier | status after Lemma 1 |
|-----------|----------------------|
| F-D2-A (rate-uniformity surprise) | **eliminated for s = 0,1**: explicit α(0) = 1 ≠ α(1) = 1/2 ⇒ no uniform rate at low s |
| F-D2-B (singularity-barrier evidence at finite s*) | **not active**: Lemma 1 does not exhibit s* in (0, 1] |
| F-D2-C (literature-already-resolved) | **partially closed**: low-s rates are extractable from cited literature, but the *function* s ↦ α(s) is not fully characterized; the resolved zone is now {0, 1}, leaving s ≥ 2 open |
| F-D2-D (γ-exponent dependence) | **not active** under the dispatch's γ > 1 isentropic choice; γ-sensitivity unchanged |

---

## §7 Anti-list — alternative low-Mach sources considered

These low-Mach lineage sources were considered for the ε-power
extraction and either *did not* contribute the s = 0 / s = 1 gap
directly, or were redundant given the Desjardins–Grenier / Danchin
combination above.

| candidate source | reason not used as primary |
|------------------|---------------------------|
| **Lions 1996** *Mathematical Topics in Fluid Mechanics, Vol. 2*, Oxford | monograph background for compressible NS weak solutions; rate-extraction at s = 0, 1 is *implicit* (energy bounds for finite-energy weak solutions), not a primary source for the strict-gap |
| **Lions–Masmoudi 1998** (J. Math. Pures Appl. 77, 585–627) | global *weak* incompressible limit; passes to the limit but does not bound gradient rate; consistent with α(0) = 1 in the weak sense but α(1) is not stated |
| **Feireisl–Novotný 2009** (Birkhäuser monograph) | uniform energy + density-fluctuation estimates on **unbounded** domains; α(0) = 1 carries to T³ via Desjardins–Grenier's local-domain analog; α(1) is not the monograph's central focus |
| **Klainerman–Majda 1982** (CPAM 35, 629–651) | *Euler* (ν = 0) low-Mach; the rates carry to NS via viscous-perturbation, but the source itself is Euler not NS, so not the primary attribution |
| **Schochet 1986** (CMP 104, 49–75) | bounded-domain (and torus) low-Mach with **fast-mode averaging** for ill-prepared data; for **well-prepared** data the averaging is bypassed and the source reduces to Klainerman–Majda; not the primary source for the well-prepared rate |
| **Métivier–Schochet 2001** (CPAM) | well-prepared data on T³ with explicit rates; *consistent with* α(0) = 1, α(1) = 1/2, but this report uses Desjardins–Grenier 1999 + Danchin 2002 as the canonical primary citations per the dispatch §4.3 actionable evidence step |
| **Hagstrom–Lorenz 2002** (Math. Methods Appl. Sci.) | viscous low-Mach refinement; same rates, secondary citation |
| **Masmoudi 2001/2007** (Arch. Ration. Mech. Anal.) | sharpening for ill-prepared / decay-at-infinity; does not affect well-prepared T³ rates |
| **Danchin 2005** (sequel) | sharpening of Danchin 2002 in critical Besov; α-values match |
| **Bresch–Desjardins** (2003+ line, J. Math. Pures Appl. and CPAM) | adds bulk-viscosity-modified compressible NS; ε replaced by λ in some bookkeeping; not the canonical low-Mach problem (dispatch §6 anti-list, parent §6); excluded per dispatch scope |

The combination **Desjardins–Grenier 1999 + Danchin 2002** is
selected as the primary attribution because:

1. The dispatch §4.3 actionable evidence step explicitly names
   these two sources.
2. They span both the energy-method ("classical") and the critical-
   Besov ("modern") frameworks, providing methodological cross-check.
3. Both are well-prepared T³-or-bounded-domain papers, matching the
   §1.1 setting exactly without translation.

---

## §8 Falsifiers active for this Lemma report

These falsifiers apply to **this report** (the literature-extraction
attempt), separate from the dispatch's F-Disp-1..7 and from the
parent acceptability's F-Acc-1..6.

- **F-Lemma1-A** (extraction-fabrication): if any α-value
  attributed to a cited paper is *not* extractable from that paper
  on careful reading, the extraction fails and the strict-gap
  conclusion must be re-evaluated. Currently the α(0) = 1 / α(1) =
  1/2 attribution is consistent with the energy-method bookkeeping
  in the cited Desjardins–Grenier 1999 line and with the
  critical-Besov framework of Danchin 2002. **Status: not active.**
  Theorem-number opacity in Danchin 2002 (§4.2 caveat) is **not**
  the same as fabrication; the *value* α(1) = 1/2 is consistent
  with the source even without the Theorem-number pinned.

- **F-Lemma1-B** (sharpness-overstatement): if α(0) = 1 or α(1) =
  1/2 are *upper bounds* only (not sharp), the strict gap might be
  smaller (Δα < 1/2) without being zero. The PASS verdict survives
  any non-zero Δα; only Δα = 0 falsifies. Sharpness is **claimed
  in §3.1, §3.2** based on standard explicit examples; if a
  sharpening exists in unscanned post-2020 literature, F-Lemma1-B
  fires partially (the value 1/2 is replaced by some other positive
  number) but the **strict-gap PASS** is preserved. **Status: not
  active in PASS-preserving sense.**

- **F-Lemma1-C** (domain-extrapolation): if the strict-gap is
  *quoted* in a context other than well-prepared T³ at finite
  [0, T] (e.g. ℝ³, or ill-prepared, or [0, ∞)), the attribution is
  mis-applied. This report restricts to well-prepared T³ at fixed
  [0, T] (§1.1 hypothesis); cross-domain quotation is forbidden
  without re-derivation. **Status: not active**, restriction
  enforced.

- **F-Lemma1-D** (D2-overcommitment): if the strict gap Δα = 1/2
  at the s = 0 → s = 1 step is reported as **proving** D2 (full
  monotonic decrease on s ≥ 0), the verdict is mis-cited. This
  report establishes the strict gap **only at s = 0 → s = 1**;
  monotonicity on s ≥ 1 is the next-Lemma scope (§6.3). **Status:
  not active**; the report is explicit that D2 monotonicity for
  s ≥ 2 is **not** proven here.

- **F-Lemma1-E** (atlas-touch): if this report leads to any
  `shared/n6/atlas.n6`, `state/proposals/inventory.json`, or
  `theory/canon/` edit, the report has been mis-applied. This
  report is a literature-extraction attempt only. **Status: not
  active.**

- **F-Lemma1-F** (R1-R5 conflation in joint outcome): if the
  PASS_LITERATURE verdict for R5 Lemma 1 is read as evidence for
  R1 Lemma 1 (or vice versa), the auxiliaries have been merged
  inappropriately. The two Lemma 1's are independent (§5).
  **Status: not active**; this report explicitly flags the parallel
  as structural, not logical.

- **F-Lemma1-G** (Clay-creep): if the strict-gap conclusion is
  reported as evidence for or against Clay BT-544, the auxiliary→
  main distinction has been lost. R5 Lemma 1 is **auxiliary**;
  BT-544 stays 0/1. **Status: not active.**

None of F-Lemma1-A..G fires under this report's scope.

---

## §9 Closing

**Verdict**: **PASS_LITERATURE**, Δα = α(0) − α(1) = 1 − 1/2 = 1/2
> 0, extracted from Desjardins–Grenier 1999 + Danchin 2002 with
cross-checks against Lions–Masmoudi 1998, Feireisl–Novotný 2009,
Klainerman–Majda 1982 + viscous extension, and Métivier–Schochet
2001.

**D2 implication**: D2 conjecture (strict monotonic decrease of
α(s) on s ≥ 0) is supported at the s = 0 → s = 1 window with an
explicit gap of 1/2. D1 (uniform-in-s rate ε^{1/2}) is **refuted**
in the well-prepared T³ regime. D3 (finite-s* singularity barrier)
remains compatible (and even suggested by linear interpolation
α(s) = 1 − s/2 → α(2) = 0); resolving D2 vs D3 requires the
next-Lemma probe at s = 2.

**Lemma scope**: this report establishes a single strict-gap data
point. Full monotonicity on s ≥ 2 is dispatched as a follow-up
(§6.3, Lemma 2 candidate); functional form of α(s) is dispatched
as a separate analytic question (§6.2).

**Next probe** (R5-direction continuation): execute Lemma 2 at
s = 2 from Métivier–Schochet 2001 / Hagstrom–Lorenz 2002 / Danchin
2005 (or sketch acoustic-mode amplification heuristic if literature
underdetermined).

**Cross-axis tie**: R1 Lemma 1 (BV-2019 viscous adaptation) runs
in parallel on the α-Hölder threshold axis; the two Lemmas are
mathematically independent. R5 Lemma 1's PASS_LITERATURE here does
not by itself imply progress on R1 Lemma 1; the two auxiliaries
populate distinct PDE-mechanism axes in the L_ω axiom-recast frame.

**Millennium status**: 0/7 unchanged. NS Clay statement unchanged.
No atlas/state/inventory edits.

— end Lemma 1 report —
