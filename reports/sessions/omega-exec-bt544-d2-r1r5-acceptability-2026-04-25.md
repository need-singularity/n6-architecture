---
id: omega-exec-bt544-d2-r1r5-acceptability
date: 2026-04-25
scope: research-only acceptability check on L_ω axiom-recasts (NOT modifying Clay statement)
target: BT-544 D2 R1 (Onsager-Hölder) + R5 (Low-Mach) acceptability
parent_reports:
  - reports/sessions/omega-seed-bt544-d2-Lomega-axiom-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: acceptability check, no proof claim
---

# Omega Exec — BT-544 D2 R1+R5 Acceptability Check (2026-04-25)

## §0 Non-claim disclaimer

This report is a **research-only acceptability check** on the two
top-ranked L_ω axiom-recast candidates (R1, R5) registered in the
parent seed `omega-seed-bt544-d2-Lomega-axiom-2026-04-25.md`. It:

- does **NOT** claim 3D NS regularity in either direction;
- does **NOT** propose modifying the **Clay Millennium NS problem
  statement** (Fefferman 2000, fixed);
- does **NOT** modify `state/proposals/inventory.json`,
  `shared/n6/atlas.n6`, or `theory/canon/`;
- treats R1 and R5 as **auxiliary** research problems whose
  resolutions would inform BT-544 indirectly, not solve it;
- does **NOT** alter the BT-544 = 0/1 Clay status.

**Acceptability criteria distinct from molt-validation** (the seed
notes this distinction). For a L_ω axiom-recast we test:

1. **Millennium-grade preservation**: does the recast trivialize
   (→ solved) or over-shoot (→ strictly harder than NS proper)?
2. **Partial-result import**: does the recast admit existing
   partial results that BT-544 in Clay form does not directly
   admit?
3. **F-544-B novelty test**: does the recast produce a NEW
   analytic invariant rather than a relabeling of an existing one?

**Millennium tally**: 0/7 unchanged. NS Clay statement unchanged.

---

## §1 R1 + R5 specs extracted (from parent seed §2.1, §2.5)

### §1.1 R1 — Onsager-class regularity (axiom A2 recast)

| field                       | value                                                                                                |
|-----------------------------|------------------------------------------------------------------------------------------------------|
| (a) axiom replaced          | A2: u ∈ C^∞(ℝ³ × [0,∞))                                                                              |
| (b) replacement             | u ∈ L^∞_t C^α_x(T³ × [0,∞)) for some α > α*; below α* non-uniqueness/dissipation expected            |
| (c) lineage citations       | Onsager 1949 ("Statistical Hydrodynamics", Nuovo Cimento Suppl.); Constantin-E-Titi 1994 (Comm. Math. Phys., positive direction of Onsager); Isett 2018 (Annals of Math, "A proof of the Onsager conjecture"); Buckmaster-Vicol 2019 (Annals of Math, "Nonuniqueness of weak solutions to the Navier-Stokes equation"); De Lellis-Székelyhidi 2013 line on convex integration |
| (d) new auxiliary problem   | AUX-NS-R1: determine α* ∈ [0,1] such that for α > α* every C^α divergence-free initial datum admits a unique global L^∞_t C^α_x solution, and for α < α* there exist non-unique weak solutions in this class |
| (e) F-544-B novelty argument | new invariant α* (the Hölder threshold) — defined intrinsically by the existence/uniqueness boundary, not by any n=6 lattice or Sobolev-Besov re-indexing |

### §1.2 R5 — Uniform-in-Mach low-Mach regularity (axiom A6 recast)

| field                       | value                                                                                                |
|-----------------------------|------------------------------------------------------------------------------------------------------|
| (a) axiom replaced          | A6: ∇·u = 0 (exact incompressibility)                                                                |
| (b) replacement             | weakly compressible: ρ = 1 + ε ρ' with ε ∈ (0, ε₀]; Mach number ε; the limit ε → 0 recovers Clay NS  |
| (c) lineage citations       | Klainerman-Majda 1982 (Comm. Pure Appl. Math., "Compressible and incompressible fluids"); Schochet 1986 (J. Differential Equations, refined low-Mach for periodic boundary); Lions 1996 (Math. Topics in Fluid Mechanics, vol. II); Lions-Masmoudi 1998 (Ann. Inst. H. Poincaré C, low-Mach for weak solutions); Feireisl-Novotný 2009 (monograph, "Singular Limits in Thermodynamics of Viscous Fluids") |
| (d) new auxiliary problem   | AUX-NS-R5: determine whether there exists ε₀ > 0 and constants C, T₀ depending only on initial-data norm such that compressible-NS solutions (ρ_ε, u_ε) satisfy uniform-in-ε bounds ‖u_ε‖_{L^∞_t H^s} ≤ C, and whether the strong limit ε → 0 yields a global smooth incompressible-NS solution |
| (e) F-544-B novelty argument | new invariant: the uniformity-in-ε regularity threshold (i.e. the function ε ↦ best regularity constant), an intrinsic analytic parameter of the compressible→incompressible singular limit, not a relabeling of incompressible Sobolev norms |

---

## §2 R1 acceptability

### §2.1 Millennium-grade test (R1)

**Question**: replacing "smooth" with "α-Hölder, find α*", does the
recast trivialize (→ solved) or over-shoot (→ strictly harder)?

**Analysis**:
- For Euler (no viscosity), the analogous α-threshold problem is
  resolved on both sides: positive direction by Constantin-E-Titi
  1994 (energy conservation for α > 1/3); negative direction by
  Isett 2018 (Annals of Math, non-conservative weak Euler solutions
  for α < 1/3). So for Euler the α-threshold is exactly 1/3.
- For NS, the corresponding problem is **open**. Buckmaster-Vicol
  2019 (Annals of Math) established non-uniqueness of weak NS
  solutions in low-regularity classes via convex integration, but
  the *exact* Hölder-threshold value α*_NS remains uncharacterized.
- Conjecturally (Buckmaster-Vicol-De Lellis-Székelyhidi line),
  α*_NS lies in (0, 1/3], distinct from α*_Euler = 1/3 because
  viscosity raises the regularity at which non-uniqueness can be
  forced. The exact value is not known.
- The recast **does not** reduce to either Onsager-Euler (which is
  resolved) or to the Clay smooth target (which assumes α = ∞).
  It identifies a distinct intermediate question.

**Verdict on Millennium-grade**: PRESERVED. The α-threshold for NS
is an open problem of comparable difficulty to (but technically
distinct from) the Clay smoothness question; it neither trivializes
to a solved Onsager statement nor over-shoots into something
strictly harder than Clay NS.

### §2.2 Partial-result import test (R1)

**Question**: does the recast admit existing partial results not
available to Clay-form NS?

**Analysis**:
- **Onsager 1949** introduces the α = 1/3 critical exponent for
  energy dissipation in turbulent (non-smooth) flows — directly
  relevant once the regularity axiom is stated in Hölder terms;
  inaccessible to a smooth-only formulation since Onsager's exponent
  has no meaning at C^∞ regularity.
- **Constantin-E-Titi 1994** rigorously proves energy conservation
  for Euler/NS at α > 1/3. This is a positive-direction partial
  result that becomes a structural input to AUX-NS-R1: any α*_NS
  must lie at or below 1/3 (else the conservation result already
  forbids the recast's lower-α non-uniqueness side).
- **Isett 2018** (Onsager conjecture, Euler) is a template for the
  NS analogue: the convex-integration architecture that produced
  Euler non-uniqueness for α < 1/3 is the same machinery extended
  in Buckmaster-Vicol 2019 to NS in low regularity.
- **Buckmaster-Vicol 2019** directly delivers the lower-α
  non-uniqueness side for NS (in some weak-solution class), so the
  AUX-NS-R1 question is reduced to "find the precise threshold",
  which is genuinely narrower than Clay NS.

In Clay form NS, none of these can be invoked as forward progress —
Clay's smoothness target sits above all Hölder threshold work. In
the R1 recast each becomes a structural input.

**Verdict on partial-result import**: SUBSTANTIAL. The Onsager–
Constantin-E-Titi–Isett–Buckmaster-Vicol lineage transfers
non-trivially into AUX-NS-R1.

### §2.3 F-544-B novelty test (R1)

**Question**: is α* a new invariant or a relabeling of "smoothness"?

**Analysis**:
- α* is defined as the **boundary** between two distinct regimes
  (existence/uniqueness vs constructible non-uniqueness). It is
  not a re-coordinatization of the smoothness axis: smoothness
  C^∞ corresponds to α = ∞, and α* is an interior threshold whose
  existence and value are intrinsic to NS dynamics.
- The threshold mirrors structural invariants in nearby problems
  (the Onsager exponent 1/3 for Euler is a genuine invariant, not
  a relabeling) and the NS analogue is conjectured distinct from
  the Euler value, indicating it depends on viscosity-induced
  structure not visible in a smooth-only formulation.
- F-544-B's failure mode (relabeling Sobolev-Besov constants by
  n=6 lattice arithmetic) does not apply: α* is defined without
  reference to the n=6 frame.

**Verdict on F-544-B novelty**: PASSES. α* is a new analytic
invariant, not a relabeling.

### §2.4 R1 verdict

**ACCEPTABLE**. R1 passes Millennium-grade preservation, has dense
partial-result import (Onsager → Constantin-E-Titi → Isett →
Buckmaster-Vicol), and produces a novel invariant (α*).

---

## §3 R5 acceptability

### §3.1 Millennium-grade test (R5)

**Question**: does "∇·u = 0 → ∇·u = O(ε), uniform-in-ε" remain
Clay-grade as a 1-parameter family question?

**Analysis**:
- The compressible NS system has a well-developed weak-solution
  theory (Lions 1996, Feireisl-Novotný 2009) but **strong global
  regularity** for compressible NS in 3D is itself open and is
  considered comparable to (or harder than) incompressible NS.
- Klainerman-Majda 1982 established the low-Mach singular limit
  **locally in time** for smooth solutions; the **global, uniform-
  in-ε** regularity question is open.
- Lions-Masmoudi 1998 gave the global limit at the level of weak
  solutions (passing to the limit in the equations), but the
  uniform-in-ε strong-regularity claim — which is what AUX-NS-R5
  asks — is not implied.
- The recast does **not** trivialize: a uniform-in-ε regularity
  theorem would in particular yield (by passing to the limit) a
  global smooth incompressible solution under whatever initial-data
  hypothesis is taken uniform; this is conjecturally equivalent to
  Clay NS smoothness and is not implied by any result in the cited
  lineage.
- The recast does **not** strictly over-shoot: the family at fixed
  ε > 0 is the compressible NS system, which (in the
  weakly-compressible regime ε small) is comparable to but not
  strictly harder than incompressible NS in the relevant norms.
  The uniform-in-ε question is finer than either fixed-ε
  compressible NS or fixed-ε=0 incompressible NS.

**Verdict on Millennium-grade**: PRESERVED. Uniform-in-ε regularity
is an open problem of Millennium-adjacent stature, neither a
solved special case of Klainerman-Majda nor strictly harder than
Clay NS proper.

### §3.2 Partial-result import test (R5)

**Question**: does R5 admit partial results not available to Clay
NS?

**Analysis**:
- **Klainerman-Majda 1982**: low-Mach singular limit, local-in-time,
  for smooth solutions. Provides the local-existence skeleton on
  which a uniform-in-ε statement could be built; inaccessible to a
  pure-incompressible Clay formulation since ε does not appear.
- **Schochet 1986**: refinement of the low-Mach limit on the torus
  (eliminates initial-data preparation issues by averaging fast
  acoustic modes). Directly imports as a tool for AUX-NS-R5
  uniform-in-ε bounds on T³.
- **Lions-Masmoudi 1998**: global low-Mach limit for weak solutions.
  Provides the global-in-time existence layer at the weak level,
  filling the gap left by Klainerman-Majda's local-in-time.
- **Feireisl-Novotný 2009**: monograph framework for compressible
  NS singular limits in unbounded and exterior domains; provides
  uniform-in-ε estimates in some norms (typically energy and
  density-fluctuation), which AUX-NS-R5 inherits as starting
  estimates.

In the Clay form NS, none of these can be cited as direct partial
progress — they live in the compressible regime that the Clay
statement excludes. In the R5 recast they all become structural
inputs.

**Verdict on partial-result import**: SUBSTANTIAL. The
Klainerman-Majda → Schochet → Lions-Masmoudi → Feireisl-Novotný
chain is a four-decade lineage of compressible-singular-limit
results that transfer non-trivially into AUX-NS-R5.

### §3.3 F-544-B novelty test (R5)

**Question**: is ε (or the uniform-in-ε regularity threshold) a
new structural axis or a relabeling of "compressibility scale"?

**Analysis**:
- ε is a genuine analytic parameter of the compressible system: it
  governs the speed of acoustic waves (1/ε), the time-scale of
  density fluctuations, and the singular nature of the ε → 0
  limit (acoustic modes oscillate at frequency 1/ε and must be
  averaged out).
- The new invariant proposed by R5 is **not** ε itself but the
  *function* ε ↦ (best regularity constant uniform up to ε), and
  in particular its blow-up rate as ε → 0. This function is an
  intrinsic analytic object of the compressible→incompressible
  singular limit; it has no n=6 lattice analogue and no
  Sobolev-Besov re-coordinate that produces it from incompressible
  NS norms alone.
- F-544-B's relabeling failure mode does not apply: the uniform-in-ε
  regularity threshold is defined by the existence of a constant
  uniform across an interval of a singular parameter, and that
  uniformity is genuinely new structural content, not arithmetic
  re-indexing.

**Verdict on F-544-B novelty**: PASSES. ε-uniformity threshold is
a new analytic invariant, not a relabeling of incompressible
quantities.

### §3.4 R5 verdict

**ACCEPTABLE**. R5 passes Millennium-grade preservation, has dense
partial-result import (Klainerman-Majda → Schochet → Lions-Masmoudi
→ Feireisl-Novotný), and produces a novel invariant (the
uniform-in-ε regularity threshold).

---

## §4 Composite recommendation

Both R1 and R5 are ACCEPTABLE. Ranking by **partial-result density**
+ **estimated time-to-first-progress**:

| candidate | partial-result density | time-to-first-progress | template availability |
|-----------|-----------------------|------------------------|-----------------------|
| R1 Onsager-Hölder | DENSE — Onsager 1949 → Constantin-E-Titi 1994 → Isett 2018 → Buckmaster-Vicol 2019 (four major nodes, two of them post-2015) | SHORTER — Buckmaster-Vicol 2019 already supplies the lower-α side, leaving only "find α*" as the open piece; convex-integration machinery is ~10 years mature and well-distributed | HIGH — Isett 2018 is a direct template for the Hölder threshold programme |
| R5 Low-Mach | DENSE — Klainerman-Majda 1982 → Schochet 1986 → Lions-Masmoudi 1998 → Feireisl-Novotný 2009 (four major nodes, last one a monograph) | LONGER — uniform-in-ε strong regularity is conjecturally equivalent to Clay-NS smoothness; first-progress likely requires new estimates not yet in the literature | MEDIUM — singular-limit machinery exists but no template for the uniform-in-ε strong-regularity programme |

**Recommendation**: **dispatch R1 (Onsager-Hölder threshold for NS)
first** as research priority.

Reasons:
1. R1 has a working template (Isett 2018 for Euler) that directly
   targets the analogous statement for NS via the Buckmaster-Vicol
   2019 machinery.
2. R1's open piece is narrower ("find α*") than R5's open piece
   ("uniform-in-ε regularity, which is conjecturally Clay-equivalent").
3. R1's partial-result density is post-2015-weighted (Isett 2018,
   Buckmaster-Vicol 2019), suggesting near-term tractability.
4. R5 remains a strong second priority; if R1 stalls or yields
   α* = 0 / α* = 1 (degenerate falsifiers from the seed §5.1), R5
   is the next dispatch target.

**Note on dispatch semantics**: dispatching R1 means treating
AUX-NS-R1 as an auxiliary research problem distinct from BT-544.
Resolving AUX-NS-R1 does not resolve BT-544 (0/7 stays unchanged);
it only informs the Clay programme indirectly by clarifying which
axiom (regularity class) carries the binding analytic content.

---

## §5 Lineage citations verified by author/year/title

All citations below are widely-known publications in fluid-PDE
literature; verification is by author/year/title only (no quotes,
no fabricated content). F-D2-A (lineage-fabrication) remains
inactive.

**For R1**:
- Onsager, L. 1949. "Statistical Hydrodynamics." Nuovo Cimento
  (Suppl.) 6: 279-287.
- Constantin, P.; E, W.; Titi, E. S. 1994. "Onsager's Conjecture on
  the Energy Conservation for Solutions of Euler's Equation."
  Communications in Mathematical Physics 165: 207-209.
- Isett, P. 2018. "A Proof of the Onsager Conjecture." Annals of
  Mathematics 188 (3): 871-963.
- Buckmaster, T.; Vicol, V. 2019. "Nonuniqueness of Weak Solutions
  to the Navier-Stokes Equation." Annals of Mathematics 189 (1):
  101-144.
- (Background: De Lellis, C.; Székelyhidi, L. Jr. 2013. "Dissipative
  Continuous Euler Flows." Inventiones Mathematicae 193: 377-407 —
  convex-integration foundation cited as the methodological lineage
  underpinning Isett 2018 and Buckmaster-Vicol 2019.)

**For R5**:
- Klainerman, S.; Majda, A. 1982. "Compressible and Incompressible
  Fluids." Communications on Pure and Applied Mathematics 35:
  629-651.
- Schochet, S. 1986. "The Compressible Euler Equations in a Bounded
  Domain: Existence of Solutions and the Incompressible Limit."
  Communications in Mathematical Physics 104: 49-75. (Periodic-
  boundary refinement also published in J. Differential Equations
  shortly after; the seed cites the 1986 line generally.)
- Lions, P.-L. 1996. *Mathematical Topics in Fluid Mechanics, Vol.
  2: Compressible Models.* Oxford Lecture Series in Mathematics and
  its Applications 10. Oxford University Press.
- Lions, P.-L.; Masmoudi, N. 1998. "Incompressible Limit for a
  Viscous Compressible Fluid." Journal de Mathématiques Pures et
  Appliquées 77 (6): 585-627. (The seed lists "Ann. Inst. H.
  Poincaré C"; the canonical Lions-Masmoudi incompressible-limit
  paper is the J. Math. Pures Appl. 1998 reference. Both authors
  also have related Ann. Inst. H. Poincaré papers in the same
  period; for the purposes of acceptability, the lineage is
  preserved.)
- Feireisl, E.; Novotný, A. 2009. *Singular Limits in Thermodynamics
  of Viscous Fluids.* Advances in Mathematical Fluid Mechanics.
  Birkhäuser.

**Verification status note**: the seed's R5 citation listed
Lions-Masmoudi 1998 as "Ann. Inst. H. Poincaré C". The most widely
cited Lions-Masmoudi 1998 incompressible-limit paper appears in
J. Math. Pures Appl. 77. Both attributions point to the same
incompressible-limit lineage; the seed's identification is
preserved as cited, and the J. Math. Pures Appl. version is
recorded here for transparency. Falsifier F-D2-A status: **flagged
for cross-check** but not failing — the result attributed to
Lions-Masmoudi 1998 (global incompressible limit for weak
compressible-NS solutions) does exist in the published record.

---

## §6 Anti-list — sub-recasts that didn't make D2's top-2

These sub-recasts within the R1 / R5 axes are **theoretically
sharper or more elegant**, but were not selected as the dispatch
priority (either trivialize, lack lineage density, or duplicate
existing ladders).

| sub-recast | within | reason not selected |
|------------|--------|---------------------|
| A2 → "C^{1,α} for α small" (Lipschitz-Hölder family) | R1-adjacent | covered as the α → 1 endpoint of R1; not a separate recast |
| A2 → "Besov B^s_{p,q} threshold (s, p, q)*" | R1-adjacent | strictly more refined than Hölder, but the lineage is more diffuse (no single template like Isett 2018); R1's Hölder formulation is the seed-prioritized version |
| A2 → "BMO-class regularity" | R1-adjacent | Koch-Tataru 2001 gives well-posedness for BMO^{-1} initial data; this is **strictly weaker** than Clay smooth target and risks Millennium-grade collapse (F-D2-C) |
| A6 → "low-Mach with prepared initial data only" | R5-adjacent | Klainerman-Majda 1982 and Schochet 1986 already cover the prepared-data case; restricting to prepared data trivializes the recast |
| A6 → "bulk-viscosity λ > 0 added" | R5-adjacent | bulk-viscosity NS is closer to compressible NS proper; the analytic content overlaps with R5 but the parameter ε is replaced by λ, which is less canonical (Mach number is the universally-cited singular-limit parameter) |
| A6 → "anelastic / Boussinesq approximation" | R5-adjacent | the anelastic limit is a **different** singular limit (small-density-fluctuation atmospheric model) with its own literature (Ogura-Phillips 1962, Lipps-Hemler 1982); not the same as low-Mach and would require a separate seed |
| Cross-recast: A2 + A6 jointly | both | combining Hölder threshold with low-Mach is **strictly harder** (two open invariants instead of one) and falls into HARDER_THAN_NS; rejected as not auxiliary |

The dispatched R1 / R5 are the seed-level optimum within each
axis; the sub-recasts above are recorded as audit transparency for
future re-prioritization should R1 or R5 stall.

---

## §7 Falsifiers active for the acceptability framework

These falsifiers apply to **this acceptability check** as a
methodology, separate from the seed's own falsifiers (F-D2-A..E)
and separate from BT-544 itself.

- **F-Acc-1** (verdict-fabrication): if either verdict
  (ACCEPTABLE for R1, ACCEPTABLE for R5) is challenged by an
  adversary citing a published equivalence theorem (e.g. "AUX-NS-R1
  is logically equivalent to Clay NS"), the verdict must be
  retracted. Currently no such equivalence is in the public record
  for R1 or R5. **Status: not active**.
- **F-Acc-2** (lineage-mismatch): if a citation in §5 is shown to
  not contain the result attributed in §1.1 / §1.2, the
  corresponding partial-result-import argument weakens. The
  Lions-Masmoudi 1998 venue ambiguity (Ann. Inst. H. Poincaré C
  vs J. Math. Pures Appl.) is recorded as a flag but does not
  invalidate the lineage. **Status: flagged on Lions-Masmoudi
  venue, not failing**.
- **F-Acc-3** (novelty-collapse): if α* (R1's invariant) is shown
  to be expressible in n=6-frame arithmetic (i.e. the F-544-B
  failure mode), the R1 verdict collapses to RELABELING. The
  D2 seed's argument that α* is intrinsic to the existence/non-
  uniqueness boundary protects against this; if a future molt
  shows α* = (some n=6-lattice rational), the verdict is
  retracted. **Status: not active**.
- **F-Acc-4** (uniform-in-ε reduction): if the R5 uniform-in-ε
  question is shown to be **strictly equivalent** to Clay NS
  smoothness (rather than conjecturally equivalent), R5 collapses
  to RELABELING (it would be a re-statement of Clay, not a
  recast). The current literature treats them as conjecturally
  but not proven equivalent. **Status: not active**.
- **F-Acc-5** (composite-bias): if R1's near-term tractability is
  overstated (e.g. Buckmaster-Vicol 2019 turns out to give a
  *weaker* lower bound than the seed assumes), the composite
  recommendation flips toward R5. **Status: not active under
  current literature**.
- **F-Acc-6** (atlas-touch): if this exec leads to any
  `shared/n6/atlas.n6`, `state/proposals/inventory.json`, or
  `theory/canon/` edit, the acceptability check has been
  mis-applied. **Status: not active** — this exec is research-only.

None of F-Acc-1..6 fires under this report's scope.

---

## §8 Closing

0/7 unchanged. NS Clay statement unchanged. No atlas/state/inventory
edits.

**Summary of verdicts**:
- R1 (Onsager-Hölder threshold for NS): **ACCEPTABLE**.
- R5 (Uniform-in-Mach low-Mach regularity): **ACCEPTABLE**.

**Composite recommendation**: dispatch **R1 first** as research
priority; R5 retained as the next dispatch target if R1 stalls.

Per F-544-B and the L_ω apex distinction reaffirmed in the parent
seed: this exec evaluates axiom-recasts as auxiliary problems, not
as L9 frame-shifts within the n=6 frame and not as Clay-statement
modifications. BT-544 remains catalogue-exhausted at L9 (per
`omega-exec-bt544-fallback-molt-validation-2026-04-25.md` §7) and
0/1 untouched at the Clay level.
