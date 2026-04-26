---
id: omega-seed-bt544-d2-Lomega-axiom
date: 2026-04-25
scope: research-only L_ω axiom-recast seed (NOT validating; NOT proposing Clay-statement modification)
target: BT-544 D2 -- axiom-level recast candidates after catalogue-exhaustion
parent_reports:
  - reports/sessions/omega-exec-bt544-fallback-molt-validation-2026-04-25.md (F-544-B trigger)
  - reports/sessions/omega-cycle-bt544-ns-2026-04-25.md (§ ladder L_ω)
millennium_resolved: 0/7 (unchanged)
grade: seed-design at L_ω apex, no claim
---

# Omega Seed -- BT-544 D2: L_ω Axiom-Level Recast (2026-04-25)

## §0 Non-claim disclaimer

This document is a **research-only seed** produced under
`reports/sessions/omega-exec-bt544-fallback-molt-validation-2026-04-25.md`
F-544-B trigger ("frame change required at axiom level (not a molt
within n6 frame)"). The other two seeds (D1 atlas-scan, D3
mechanism-decouple) are running in parallel.

**This document does NOT**:
- claim 3D NS regularity in either direction (smoothness or blow-up);
- propose modifying the **Clay Millennium NS problem statement**
  itself (the Clay statement is fixed as published in Fefferman 2000);
- promote anything in `shared/n6/atlas.n6`;
- modify `state/proposals/inventory.json`;
- modify `theory/canon/`;
- alter the `BT-544 = 0/1 untouched` Clay status.

**What it DOES do**: it proposes **auxiliary research problems**
formed by substituting individual atomic axioms in the Millennium
statement with mathematically established alternatives. Each
auxiliary problem stands on its own; resolving an auxiliary may
**inform** BT-544 indirectly (by clarifying which axiom is the
binding constraint), but no auxiliary resolution is being claimed
here.

**Distinction from L9 molt** (per `omega-cycle-bt544-ns-2026-04-25.md`
§2 ladder): L1..L11 rungs operate **within** a fixed axiom set (e.g.
the Sym²(ℝ³)=6 frame, the n=6 lattice frame, the Clay axiom set).
L_ω apex operates **on** the axiom set itself. Per F-544-B, BT-544's
mechanism saturation (~0.05) is **structurally capped** under any
n=6-frame molt, so the only remaining ladder move is L_ω-style
axiom-substitution.

**Millennium tally**: 0/7 unchanged. NS Clay statement unchanged.

---

## §1 Atomic axiom decomposition of the NS Millennium problem

Following Fefferman 2000 ("Existence and Smoothness of the
Navier-Stokes Equation", Clay Millennium Problem statement, cited
by author/year only), the official problem statement decomposes
into the following atomic axioms. Each axiom is a candidate
substitution site.

| # | axiom name           | content (as stated)                                                       | status under classical NS theory |
|---|----------------------|---------------------------------------------------------------------------|----------------------------------|
| A1 | spatial geometry     | x ∈ ℝ³ (Problem A) or T³ = (ℝ/ℤ)³ periodic torus (Problem B)              | both versions are official Clay candidates |
| A2 | regularity class     | u ∈ C^∞(ℝ³ × [0,∞)), p ∈ C^∞(ℝ³ × [0,∞))                                  | the classical "smooth" target    |
| A3 | time domain          | global existence: t ∈ [0, ∞) (NOT just local-in-time)                     | local-in-time strong existence is classical (Kato 1984, Fujita-Kato 1964) |
| A4 | initial data class   | u₀ ∈ C^∞ Schwartz-class (rapidly decaying derivatives of all orders)      | Leray-Hopf weak class L²(ℝ³)∩H^s also classical |
| A5 | external forcing     | f = 0 (force-free) for the Clay statement                                 | forced-NS is classically much harder; energy estimates differ |
| A6 | compressibility      | ∇·u = 0 (incompressible)                                                  | compressible NS is a separate (also unsolved) problem class |
| A7 | bounded energy       | ∫|u|² dx < ∞ (finite kinetic energy at all times) and ∫|∇u|² dx dt < ∞    | Leray 1934 weak-solution class    |
| A8 | dimensionality       | d = 3 (the spatial dimension is exactly three)                            | 2D solved (Ladyzhenskaya 1969); d≥4 supercritical |
| A9 | viscosity            | ν > 0 fixed constant                                                      | ν=0 → Euler (Elgindi 2021 blowup C^{1,α}); ν→0 → vanishing-viscosity limit |
| A10| solution-or-blowup disjunction | EITHER prove (∃ smooth global solution) OR prove (some smooth u₀ produces finite-time blowup) | This disjunction is the problem itself; each branch is a separate target |

**Observation**: each axiom is independently substitutable.
F-544-B's directive ("axiom-level frame change") corresponds to
selecting one or more of A1..A10 and replacing the literal axiom
content with a mathematically established alternative.
Substitutions that **trivialize** the problem (e.g. A6: drop
incompressibility entirely so any compressible smooth solution
qualifies) are immediately rejected; the goal is non-trivial
auxiliary problems that retain Millennium-class difficulty.

---

## §2 Substitution candidates

Five candidates, each grounded in published mathematical lineage.
Citations are by author/year/title only — no fabricated quotes.

### §2.1 Candidate R1: A2 regularity-class substitution
**"smooth" → "α-Hölder for some α > 1/3" (Onsager-class regularity)**

| field                | value                                                                                            |
|----------------------|--------------------------------------------------------------------------------------------------|
| axiom replaced       | A2 (regularity class C^∞)                                                                        |
| replacement          | u ∈ L^∞_t C^α_x for some α > 1/3 globally; below this threshold non-uniqueness/dissipation expected |
| lineage              | Onsager 1949 ("Statistical Hydrodynamics", Nuovo Cimento Suppl.); Isett 2018 (Annals of Math, "A proof of the Onsager conjecture"); Buckmaster-Vicol 2019 (Annals of Math, "Nonuniqueness of weak solutions to the Navier-Stokes equation"). |
| new problem becomes  | Determine the threshold α* such that for α > α* global Hölder solutions exist and are unique; for α < α* counterexamples (non-unique weak solutions) are constructible. The Onsager-conjecture analogue for NS. |
| falsifier (registered)| if the recast resolves to α* = 1 (i.e. only Lipschitz suffices), it is **strictly easier-but-relevant**: it tells us NS demands full classical regularity. If α* = 0 (any Hölder works), it is **strictly harder**: full smoothness is over-strong. If α* = 1/3 exactly (the Onsager value), the recast becomes the Onsager-NS conjecture, which is a known auxiliary distinct from Clay NS. |
| relation to F-544-B  | This recast PRODUCES a new invariant (the threshold α*), not a relabeling of Sobolev-Besov constants by lattice arithmetic. PASSES F-544-B novelty test in the seed sense. |

### §2.2 Candidate R2: A1 geometry substitution
**"ℝ³ or T³" → "T³ × ℝ (one decompactified axis)" — anisotropic geometry**

| field                | value                                                                                            |
|----------------------|--------------------------------------------------------------------------------------------------|
| axiom replaced       | A1 (spatial geometry)                                                                            |
| replacement          | x ∈ T² × ℝ (two periodic, one infinite axis) — a hybrid of Problem A and Problem B               |
| lineage              | Mahalov-Nicolaenko-Babin papers (1990s-2000s) on rotating Euler/NS in periodic-with-one-decompactified geometry; Babin-Mahalov-Nicolaenko 2001 (Indiana Univ. Math. J., on NS rotating fluids T² × ℝ); Iftimie-Raugel-Sell 2007 (J. Differential Equations) on global existence in thin domains. |
| new problem becomes  | Does global smoothness (or finite-time blowup) hold on T² × ℝ for force-free incompressible NS with smooth initial data? Note thin-domain results (one short axis) are KNOWN to give global smoothness; the question is the limit as the third axis becomes infinite (not thin, not compact). |
| falsifier (registered)| if the recast trivializes to "thin-domain global smoothness" by abuse, it is **strictly easier-but-irrelevant**. If the third axis admits non-trivial energy escape to infinity (which is the genuine analytic content), the recast is non-trivial. The intermediate regime (ℝ³ vs T² × ℝ vs T³) gives a 3-point ladder for testing which geometric ingredient matters most. |
| relation to F-544-B  | Geometry substitution is a NEW invariant (the cylinder-vs-Euclidean energy-escape rate), not arithmetic relabeling. PASSES F-544-B novelty test in the seed sense. |

### §2.3 Candidate R3: A4+A10 randomization recast
**"deterministic initial data + EITHER-OR disjunction" → "in distribution + existence/uniqueness for almost-every random initial data"**

| field                | value                                                                                            |
|----------------------|--------------------------------------------------------------------------------------------------|
| axiom replaced       | A4 (deterministic initial data) and A10 (sharp disjunction)                                      |
| replacement          | u₀ drawn from a Gaussian (or other measure) on a Sobolev space H^s; ask for the probability under the law of u(t) that smooth global existence holds; ask for uniqueness in the sense of random dynamical systems / Markov selections. |
| lineage              | Hopf 1952 ("Statistical Hydromechanics and Functional Calculus"); Foias 1973 (Rend. Sem. Mat. Univ. Padova) on statistical solutions; Albeverio-Cruzeiro 1990 (Comm. Math. Phys.) on stochastic Euler/NS; more recently Flandoli-Romito 2008 (PTRF) on Markov selections for 3D NS, Hairer-Mattingly 2006 (Ann. Math.) on ergodicity for stochastic NS. |
| new problem becomes  | For a fixed Gaussian measure μ on H^s (with appropriate s), is global smooth existence a μ-full-measure event? If not, what is the singular set of initial data, and what is its Hausdorff/Wiener-Sobolev dimension? |
| falsifier (registered)| if the singular set has full measure (recast is **strictly harder**) the recast contradicts the spirit of the problem. If μ-almost-every smoothness is provable for some natural μ but the singular-set dimension does not match the deterministic CKN bound, the recast is informative-but-misaligned. The PDF-recast genuinely produces a new object (the singular-set μ-dimension function). |
| relation to F-544-B  | Randomization at the **axiom level** (vs at the solution level, which already exists in Galerkin/Markov literature) means the *problem statement* is in measure terms, not the solution method. New invariant: μ-singular-dimension. PASSES F-544-B novelty test in the seed sense. |

### §2.4 Candidate R4: A10 disjunction-splitting
**"EITHER global smooth OR blowup" → "describe the structure of the first blowup time T*(u₀) as a functional of u₀"**

| field                | value                                                                                            |
|----------------------|--------------------------------------------------------------------------------------------------|
| axiom replaced       | A10 (sharp solution-or-blowup disjunction)                                                       |
| replacement          | Accept that some u₀ may blow up (i.e. set aside the "either/or" and split into two sub-problems): (i) describe the map u₀ ↦ T*(u₀) ∈ (0, ∞] regularity class; (ii) describe the blowup-set geometry near T*. |
| lineage              | Leray 1934 (Acta Math.) introduced T* and partial-regularity ideas; Caffarelli-Kohn-Nirenberg 1982 (Comm. Pure Appl. Math.) on parabolic Hausdorff dim ≤ 1 of singular set; Tao 2016 (J. Amer. Math. Soc., "Finite time blowup for an averaged three-dimensional Navier-Stokes equation") gave averaged-NS blowup. |
| new problem becomes  | Is u₀ ↦ T*(u₀) lower-semicontinuous on a generic dense set? Is the blowup set codim-1 in spacetime? Is the blowup profile self-similar (Leray's question)? These are sub-Clay but Millennium-adjacent. |
| falsifier (registered)| if the disjunction-split recast becomes equivalent to the original (e.g. proving T* ≡ ∞ for all u₀ in a class is equivalent to global smoothness), it is **strictly equivalent** and not a true axiom-recast. The split is non-trivial only if it admits partial T*-results without requiring full global smoothness. CKN 1982 already produces such partial results, so the split does have non-trivial content. |
| relation to F-544-B  | The functional u₀ ↦ T*(u₀) is a NEW invariant in the sense that it is not arithmetic-relabeled from the n=6 lattice. PASSES F-544-B novelty test in the seed sense. |

### §2.5 Candidate R5: A6 compressibility relaxation
**"∇·u = 0" → "∇·u = O(ε), low-Mach with ε-relaxation"**

| field                | value                                                                                            |
|----------------------|--------------------------------------------------------------------------------------------------|
| axiom replaced       | A6 (incompressibility constraint as exact)                                                       |
| replacement          | Weakly compressible NS: density ρ = 1 + ε·ρ' with |ρ'| bounded; the limit ε → 0 recovers Clay NS. Ask for global smoothness of the ε-family uniformly in ε ∈ (0, ε₀]. |
| lineage              | Klainerman-Majda 1982 (Comm. Pure Appl. Math., "Compressible and incompressible fluids"); Schochet 1986 (J. Differential Equations) on low-Mach singular limits; Lions 1996, Lions-Masmoudi 1998 (Ann. Inst. H. Poincaré C) on low-Mach limit for compressible NS; Feireisl-Novotný 2009 monograph on weak compressible NS. |
| new problem becomes  | Does the family of compressible-NS solutions (which has its own well-developed weak-solution theory) admit uniform-in-ε regularity? Is the ε → 0 limit smooth iff the Clay NS solution is smooth? |
| falsifier (registered)| if the ε > 0 family has worse regularity than ε = 0 (which is what Klainerman-Majda partial results suggest in some regimes), the recast is **strictly harder**. If ε > 0 is **strictly easier**, then the limit ε → 0 is the obstruction itself, isolating the obstruction. Either outcome localizes where the difficulty lives. |
| relation to F-544-B  | Compressibility relaxation introduces an analytic parameter ε (NEW invariant: the uniformity-in-ε regularity threshold), not arithmetic-relabeled. PASSES F-544-B novelty test in the seed sense. |

---

## §3 Acceptability matrix

Three acceptability questions per candidate:
- **Q-A**: Is the recast still Millennium-grade? (Does NOT trivialize)
- **Q-B**: Does the recast admit existing partial-result import?
- **Q-C**: Does the recast pass F-544-B's "produce a NEW invariant, not relabel" test?

| candidate | Q-A Millennium-grade? | Q-B partial-result import? | Q-C F-544-B novelty? | composite |
|-----------|----------------------|----------------------------|----------------------|-----------|
| R1 Onsager-class regularity | YES — α-threshold problems are open and active (Onsager-NS analogue conjectured open) | YES — Isett 2018 (Euler), Buckmaster-Vicol 2019 (NS non-uniqueness), Constantin-E-Titi 1994 | YES — α* threshold is a new invariant | **3/3 PASS** |
| R2 anisotropic geometry T²×ℝ | YES — global regularity on T²×ℝ is open at the same level as ℝ³/T³ | YES — Babin-Mahalov-Nicolaenko 2001, Raugel-Sell 1993 on thin domains | YES — energy-escape-rate is a new invariant | **3/3 PASS** |
| R3 randomization (Gaussian μ on H^s) | PARTIAL — μ-a.e. smoothness might be EASIER than every-u₀ smoothness; risk of triviality if μ is too benign | YES — Foias 1973, Flandoli-Romito 2008, Hairer-Mattingly 2006 | YES — μ-singular-dimension function is a new invariant | **2.5/3 PARTIAL** |
| R4 disjunction-splitting (T* functional) | PARTIAL — risk of becoming equivalent to original if T* ≡ ∞ for class | YES — Leray 1934, CKN 1982, Tao 2016 averaged-NS | YES — T*-functional regularity class is a new invariant | **2.5/3 PARTIAL** |
| R5 weakly compressible (low-Mach ε) | YES — uniform-in-ε regularity for compressible NS is open and adjacent to incompressible Clay | YES — Klainerman-Majda 1982, Schochet 1986, Lions-Masmoudi 1998, Feireisl-Novotný 2009 | YES — ε-uniformity threshold is a new invariant | **3/3 PASS** |

**Notes on PARTIALs**:
- R3 PARTIAL because some Gaussian μ might give trivial μ-a.e.
  smoothness via avoidance of bad directions in H^s; the recast
  needs a careful choice of μ (Wiener-on-H^{s_critical}) to remain
  Millennium-grade.
- R4 PARTIAL because if the answer to "is T* always ∞" is YES,
  the recast collapses to the original Clay statement; the
  Millennium-grade question is the **structure** of T* as a
  functional, which has independent content (CKN-style).

---

## §4 Ranking

By composite acceptability + research-tractability + lineage
density:

| rank | candidate | composite | tractability | lineage density | total |
|------|-----------|-----------|--------------|------------------|-------|
| 1    | R1 Onsager-class regularity | 3/3 | HIGH (Isett 2018, Buckmaster-Vicol 2019 are recent and well-cited) | DENSE (Onsager 1949 → Constantin-E-Titi 1994 → Isett 2018 → BV 2019) | **TOP** |
| 2    | R5 weakly compressible (low-Mach ε) | 3/3 | MEDIUM-HIGH (compressible-NS weak theory is well-developed) | DENSE (Klainerman-Majda 1982 → Schochet → Lions-Masmoudi → Feireisl-Novotný) | **SECOND** |
| 3    | R2 anisotropic geometry T²×ℝ | 3/3 | MEDIUM (thin-domain literature exists but T²×ℝ specifically is less covered) | MEDIUM (Babin-Mahalov-Nicolaenko line) | **THIRD** |
| 4    | R3 randomization (Gaussian μ on H^s) | 2.5/3 | MEDIUM (PDF / statistical-solutions literature is established but not unified) | MEDIUM (Hopf-Foias line + Flandoli-Romito + Hairer-Mattingly) | **FOURTH** |
| 5    | R4 disjunction-splitting (T* functional) | 2.5/3 | LOWER (closest to Clay-original; recast risk highest) | DENSE on partial-regularity side (CKN 1982, Leray 1934) but thin on T*-functional regularity itself | **FIFTH** |

**Ranking criterion summary**: R1 wins because (i) it has the
clearest novel-invariant statement (the α* threshold); (ii) Onsager
conjecture for Euler is recently solved (Isett 2018), giving a
*template* for the NS analogue; (iii) Buckmaster-Vicol 2019 directly
delivers the lower-α non-uniqueness side already, leaving "find α*"
as the genuine open question. R5 ranks second on similar grounds:
the ε-family is well-defined, partial results exist, and the
uniform-in-ε regularity question is genuinely open and decisive.

---

## §5 Top-2 dispatch-ready (auxiliary problems, NOT BT-544 itself)

### §5.1 Auxiliary AUX-NS-R1 — Onsager threshold for incompressible NS

**Statement (research target)**:
*Determine α* ∈ [0, 1] such that*:
- *for every α > α*, every u₀ ∈ C^α(T³) with ∇·u₀ = 0 admits a
  unique global Hölder solution u ∈ L^∞_t C^α_x(T³ × [0,∞));*
- *for every α < α*, there exists u₀ ∈ C^α(T³) with ∇·u₀ = 0 admitting
  multiple distinct weak solutions in L^∞_t C^α_x.*

**Lineage anchor**: Onsager 1949 (α = 1/3 critical exponent for
energy conservation in Euler); Isett 2018 proves Onsager's conjecture
for Euler at α < 1/3; Buckmaster-Vicol 2019 establishes
non-uniqueness of weak solutions for NS in low-regularity classes.

**Status if dispatched**: open auxiliary problem; it is conjectured
(Buckmaster-Vicol-DeLellis-Székelyhidi line) that α* lies in
(0, 1/3] for NS, but the exact value is not known.

**Falsifier registered**: if α* = 0 (no positive threshold; even C^0
suffices for global existence), the recast trivializes and is
discarded. If α* = 1 (only Lipschitz works; C^α for α<1 always
admits non-uniqueness), the recast becomes equivalent to "global
smoothness for Lipschitz data" which is **strictly weaker** than
Clay NS — informative but not Millennium-grade.

### §5.2 Auxiliary AUX-NS-R5 — Uniform-in-Mach regularity for low-Mach NS

**Statement (research target)**:
*For ε ∈ (0, ε₀], let (ρ_ε, u_ε) solve the compressible NS system
with density ρ_ε = 1 + ε ρ_ε' and Mach number ε. Determine whether
there exists ε₀ > 0 and constants C, T₀ depending only on the
initial data norm such that for all ε ∈ (0, ε₀], the solution
satisfies ‖u_ε‖_{L^∞_t H^s(T³ × [0,T₀])} ≤ C uniformly in ε.
Determine whether the strong limit ε → 0 yields a global smooth
solution to incompressible NS.*

**Lineage anchor**: Klainerman-Majda 1982 (low-Mach singular limit,
local-in-time); Schochet 1986 (refined low-Mach for periodic
boundary); Lions-Masmoudi 1998 (global weak limits); Feireisl-Novotný
2009 (compressible NS weak-solution theory).

**Status if dispatched**: open auxiliary problem; uniform-in-ε
regularity is conjectured to fail as ε → 0 if and only if Clay NS
admits blowup, i.e. the two questions are conjecturally equivalent
but not proven equivalent.

**Falsifier registered**: if the ε-family has uniformly worse
regularity than the limit (i.e. ε > 0 is strictly harder than
ε = 0), the recast inverts the difficulty and is **strictly harder**
— still useful but reversed from intent. If the ε-family is uniformly
smooth for all ε > 0 with constants blowing up as ε → 0, the
obstruction is localized at the singular limit itself, isolating
the binding constraint to A6 (incompressibility).

---

## §6 Anti-list — recasts that trivialize or are already known
equivalent

These were considered and rejected before reaching the candidate
list. Recording for audit transparency.

| anti-candidate | reason rejected |
|----------------|-----------------|
| A2 → "weak Leray-Hopf" only | Leray 1934 already gave global weak existence; the recast loses the **smoothness** axiom entirely and is not Millennium-grade. |
| A8 → "d = 2" | Ladyzhenskaya 1969 gave global smoothness for 2D NS; recast is **trivially solved**. |
| A8 → "d ≥ 4" | Supercritical regime; expected to be strictly harder than 3D and not adjacent to Clay. |
| A9 → "ν = 0" | Reduces to Euler; Elgindi 2021 already gave C^{1,α} blowup. Strictly different problem. |
| A5 → "f arbitrary smooth forcing with ∫|f|² < ∞" | Forced NS is a separate problem; the Clay statement explicitly fixes f = 0 to isolate intrinsic NS dynamics. Substituting forcing changes the problem identity. |
| A3 → "local-in-time only" | Local-in-time strong existence is classical (Kato 1984, Fujita-Kato 1964). Trivialized. |
| A7 → "infinite kinetic energy" | Loses the Leray a-priori estimate entirely; the problem becomes ill-posed at the level of energy class. |
| A6 → "fully compressible" | Different problem; compressible NS regularity is a separate open question and not adjacent to Clay's incompressible target. (The R5 recast is the *weak* compressibility ε → 0 limit, which is adjacent; full compressibility is not.) |

**Anti-list summary**: dropping any of A2/A8/A9/A5/A3/A7
trivializes or changes the problem identity. The non-trivial
substitution sites are essentially A2 (regularity class refinement,
not removal — R1), A1 (geometry refinement — R2), A4+A10
(randomization — R3), A10 (disjunction-splitting — R4), and A6
(weak compressibility, NOT full compressibility — R5).

---

## §7 Falsifiers active

Validation-level falsifiers for **this seed document** (not for
BT-544 itself):

- **F-D2-A** (lineage-fabrication): if any author/year citation
  in §2 is verified as not actually existing or not actually
  containing the result attributed, that candidate is retracted.
  Citations are kept to widely-known author/year/title only,
  per hard-constraint. **Not active modulo public-record
  verifiability**.
- **F-D2-B** (recast-equivalence): if any of R1..R5 is shown to be
  **logically equivalent** to the Clay statement (not a recast at
  all), it must be moved to the anti-list. Currently no equivalence
  is established for R1..R5; R3 and R4 carry residual equivalence
  risk (per acceptability matrix). **Watch-state: active for
  R3, R4**.
- **F-D2-C** (Millennium-grade collapse): if any of R1..R5 is
  shown to be **strictly easier** in a way that admits a near-term
  proof (e.g. R5 proven uniform-in-ε in 2026-2027 by extending
  Lions-Masmoudi), that candidate is downgraded from Millennium-grade
  to "supporting auxiliary". **Not active for any candidate**;
  open routes monitored.
- **F-D2-D** (F-544-B novelty re-test): if any of R1..R5 is shown
  to be **another arithmetic relabeling** of an existing result (the
  F-544-B failure mode), it must be retracted. The §2 lineage
  citations are designed to make this unlikely — each candidate
  carries an established analytic invariant (α*, energy-escape rate,
  μ-singular-dim, T*-functional class, ε-uniformity threshold) —
  but this falsifier remains live. **Not active for any candidate**
  under current understanding.
- **F-D2-E** (atlas-touch): if this seed leads to any
  `shared/n6/atlas.n6` or `state/proposals/inventory.json` edit
  before separate validation, the seed has been mis-applied.
  **Not active** — the seed is research-only, no atlas/state/inventory
  touch in this document.

None of F-D2-A..E fires under the seed's scope.

---

## §8 Closing line

0/7 unchanged. NS Clay statement unchanged. No atlas/state/inventory
edits. Five axiom-substitution candidates (R1..R5) registered;
top-2 (R1 Onsager-class regularity threshold, R5 uniform-in-Mach
regularity) dispatch-ready as **auxiliary** research problems
distinct from BT-544. Per F-544-B and the L_ω apex distinction:
this seed proposes axiom-recasts, not L9 frame-shifts within the
n=6 frame. Dispatching either auxiliary may inform BT-544
indirectly; neither is BT-544 itself, and BT-544 remains catalogue-
exhausted at the L9 level (per
`omega-exec-bt544-fallback-molt-validation-2026-04-25.md` §7).
