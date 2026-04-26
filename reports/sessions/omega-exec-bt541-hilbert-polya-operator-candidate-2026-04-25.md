---
id: omega-exec-bt541-hilbert-polya-operator-candidate
date: 2026-04-25
scope: research-only candidate generation (NOT validating; NOT proving RH)
target: BT-541 Hilbert-Pólya operator candidate -- operator-theoretic frame-shift
parent_reports:
  - reports/sessions/omega-meta-prediction-bt541-method-gap-2026-04-25.md (BT-541 method-gap)
  - reports/sessions/omega-cycle-bt541-riemann-2026-04-25.md
  - reports/sessions/omega-exec-bt544-extb-analytic-lyapunov-candidate-2026-04-25.md (template)
millennium_resolved: 0/7 (unchanged)
grade: candidate generation, no claim
---

# Omega Exec — BT-541 Hilbert-Pólya Operator Candidate (2026-04-25)

## §0 Non-claim disclaimer

This report **generates** a single BT-541 frame-shift candidate of the
operator-theoretic class identified by the BT-541 method-gap analysis
(`omega-meta-prediction-bt541-method-gap-2026-04-25.md` §3, §6). It
does **not**:

- claim the Riemann Hypothesis (RH), nor any partial reduction of RH,
  nor any density / line-coverage improvement;
- prove or even sketch-prove that the candidate operator H is
  self-adjoint on a separable Hilbert space, has discrete real
  spectrum, or has spectrum equal to {Im ρ_n : ζ(ρ_n) = 0, 0 < Re ρ_n < 1};
- promote any atlas entry from `[10]` to `[10*]`, modify
  `state/proposals/inventory.json`, edit `theory/canon/`, alter the
  BT-541 PARTIAL verdict from `phase-02-Y1-bt541-riemann.md`, or
  generalise the BT-541 Lead-B PASS into RH progress;
- declare Hilbert-Pólya correct or incorrect; the conjecture is
  unfilled, and this report is one more unfilling, not a closure.

Per the BT-541 method-gap diagnosis, the gap is **single-family (B)
incompleteness**: Family B has the structural blueprint
(RH ⇔ ∃ self-adjoint H with spec(H) = {Im ρ : ζ(ρ) = 0}); 110+ years
of attempts have not produced an instance. This report instantiates
the same template as
`omega-exec-bt544-extb-analytic-lyapunov-candidate-2026-04-25.md`:
a candidate (not a proof), with falsifiers registered upfront, with a
discriminator design, and with an explicit expected verdict skewed to
OBSTRUCTION_DOCUMENTED rather than PASS.

The cross-BT EXT trio (BT-544/543/542 EXT-B analytic-Lyapunov class)
addresses *between-families* gaps via inequality construction. BT-541's
analog is **operator-theoretic** rather than inequality-based, but the
*pattern* is identical: a structurally-prescribed framework with a
missing instantiation.

**Methodological honesty**: at axis-B difficulty (110+ years
unsuccessful), the realistic prior on PASS_LITERATURE is < 1%. The
candidate is generated to populate the EXT slot for BT-541, to surface
the falsifier structure of the gap, and to cross-tie with BT-544/543/542
EXT-B. It is **not** a near-miss, **not** a near-PASS, and **not** a
proposed proof route.

**0/7 unchanged. No atlas/state/inventory edits.**

---

## §1 Hilbert-Pólya framework recap

### §1.1 The conjecture as program

The Hilbert-Pólya conjecture (attributed to David Hilbert and George
Pólya, no published source from either; documented via Pólya's verbal
statement to Andrew Odlyzko, recounted in Odlyzko's 1981/1985
correspondence with Pólya) is the proposition:

> There exists a self-adjoint operator H on a separable Hilbert space
> ℋ such that the non-trivial zeros ρ_n = 1/2 + i γ_n of ζ(s)
> correspond bijectively to eigenvalues of H, with γ_n ∈ Spec(H) (or,
> in the standard variant, γ_n ∈ Spec(H − 1/2 i) under an analogous
> shift; the precise spectral identification varies by formulation).

If H exists with this property, the spectral theorem forces all γ_n
to be real, hence ρ_n ∈ {Re s = 1/2}, hence RH. The conjecture is
therefore equivalent to RH in the direction "operator exists ⇒ RH";
the converse direction (RH ⇒ operator exists) is *not* part of the
conjecture and is independently open.

### §1.2 Structural anatomy

Per the §3 of the method-gap report, Family B (operator-theoretic)
reaches a **complete structural blueprint** with four anatomical pieces
that any candidate H must satisfy:

(i) **Self-adjointness** on a separable Hilbert space ℋ. Equivalently
    (Stone's theorem; Reed-Simon 1972 *Methods of Modern Mathematical
    Physics, Vol. I* §VIII.3) D(H) ⊂ ℋ is dense, H = H* on D(H), and
    H is closed.

(ii) **Spectrum** is discrete (point spectrum) with no accumulation in
     ℝ; equivalently the resolvent (H − λ)^{-1} is compact for some λ
     ∉ Spec(H) (Reed-Simon 1972 §VI; the spectrum may also be
     continuous in some formulations, but the standard Hilbert-Pólya
     reading expects discrete with the same density as ζ-zeros).

(iii) **Spectral identification**: Spec(H) = {γ_n : n ∈ ℤ_{>0}} where
      γ_n is the n-th non-trivial ζ-zero ordinate, counted with
      multiplicity. (Multiplicity questions are themselves open: ζ-
      zeros are conjectured simple, but this is not proved.)

(iv) **Structural reason** for (iii): Spec(H) matches γ_n not by
     coincidence or curve-fit but for a *constructive* reason
     (e.g. trace formula, quantisation of a classical system, modular-
     form interpretation, cohomological identification).

A candidate H satisfying (i), (ii), (iii) gives RH. A candidate
satisfying (i), (ii), and *spectral density matching* but not (iii)
exactly would be a Hilbert-Pólya **partial fit** (interesting but
non-conclusive, in the GUE-coupling spirit).

### §1.3 Spectral density and Riemann-von Mangoldt

The conjectured spectral density at large height T is, per Riemann
1859 / von Mangoldt 1895:

  N(T) = #{γ_n ∈ (0, T]} = (T / 2π) log(T / 2π e) + O(log T).      (1.1)

Any candidate H must reproduce (1.1) at leading order. This is a
**necessary** but not sufficient consistency check; many operators
have this density without matching γ_n pointwise.

---

## §2 Existing candidate inventory — what the literature has tried

The active operator-theoretic literature on RH contains five
substantial candidate families. Each is recorded with its operator
proposal, its monotonicity / spectrum claim, the structural item
(i)–(iv) it fails, and the standard literature reference. Citations
are by author / year / journal and are pre-existing; nothing in this
section is fabricated.

### §2.1 Berry-Keating xp Hamiltonian (1999)

- **Source**: Michael Berry and Jonathan Keating, "H = xp and the
  Riemann zeros", in *Supersymmetry and Trace Formulae* (Lerner et al.
  eds.), Plenum 1999; and Berry-Keating "The Riemann zeros and
  eigenvalue asymptotics", *SIAM Review* 41 (1999), 236-266.
- **Operator**: classical Hamiltonian H_cl = xp on phase space (x, p) ∈
  ℝ × ℝ. Quantisation: H = (1/2)(xp + px) = (1/2)(xp̂ + p̂x) where
  p̂ = −i ℏ d/dx, formally on L²(ℝ_{>0}) or a regularised half-line.
- **Spectral claim** (heuristic): the semi-classical level density
  matches Riemann-von Mangoldt (1.1) in a calculation that uses
  classical periodic orbits in the (x, p) phase space and a
  semi-classical Gutzwiller trace formula.
- **Failure mode**: H = xp on L²(ℝ_{>0}) is **not self-adjoint** and
  has no canonical self-adjoint extension; H has continuous spectrum
  (not discrete) on natural domains. Berry-Keating themselves note
  the proposal is heuristic / semi-classical, not rigorous (see SIAM
  Rev. 1999 §6 "open problems"). Item (i) self-adjointness fails;
  item (ii) discrete spectrum fails; item (iv) structural reason is at
  the level of asymptotic counting, not pointwise spectrum.
- **Status as Hilbert-Pólya candidate**: the most-cited candidate, but
  acknowledged by its authors as a *suggestive heuristic* and not a
  rigorous instantiation. Has motivated 25+ years of regularisation
  attempts, none accepted.

### §2.2 Connes adelic / noncommutative-geometry approach (1996, 1999)

- **Source**: Alain Connes, "Trace formula in noncommutative geometry
  and the zeros of the Riemann zeta function", *Selecta Mathematica*
  (New Series) 5 (1999), 29-106; and the earlier "Formule de trace en
  géométrie non-commutative et hypothèse de Riemann", *C. R. Acad.
  Sci. Paris* 323 (1996), 1231-1236.
- **Operator**: H acts on a function space attached to the adele class
  space 𝔸_ℚ / ℚ* (the quotient of the rational adele ring by its
  units). The candidate H is an absorption operator analogous to a
  Laplacian on this noncommutative space, formulated via a trace
  formula whose explicit-formula side reproduces the von Mangoldt
  expression.
- **Spectral claim**: in Connes' setup, RH is equivalent to a positivity
  property of a trace functional on the adele class space, rather than
  to an explicit spectral identification. The operator-theoretic
  formulation is via a generalised Selberg trace formula on the
  noncommutative space.
- **Failure mode**: the trace-formula side is established only
  conjecturally; the absorption operator's spectrum is not constructed
  in a closed form that matches γ_n. Item (iii) spectral identification
  is conditional; item (iv) the structural reason is via the trace
  formula whose validity in the form needed is itself open. Connes'
  programme is widely regarded as profound but unfinished.
- **Status**: the most-developed structural framework; absorbs much of
  the BT-541 method-gap analysis (Family B framework existence) without
  closing it.

### §2.3 de Branges Hilbert spaces of entire functions (1986+)

- **Source**: Louis de Branges, multiple papers and preprints
  beginning with "The Riemann Hypothesis for Hilbert Spaces of Entire
  Functions", *Bull. Amer. Math. Soc.* 15 (1986), 1-17; followed by a
  series of revised attempts hosted at Purdue (1990s-2010s) under
  various titles.
- **Operator**: H is a multiplication-by-z operator on a de Branges
  space ℋ(E) of entire functions of exponential type, where E is a
  Hermite-Biehler entire function tied to ζ. The de Branges spaces
  framework (de Branges 1968 monograph *Hilbert Spaces of Entire
  Functions*, Prentice-Hall) provides general theorems on when
  multiplication operators on ℋ(E) are self-adjoint and their spectra
  are real.
- **Spectral claim**: if E is chosen via a specific construction tied
  to ζ, RH would follow from the standard theorems in *Hilbert Spaces
  of Entire Functions* applied to ℋ(E).
- **Failure mode**: the proposed proofs by de Branges have been
  examined by the analytic number theory community and have been found
  to contain gaps; specific objections include those of Conrey and Li
  (Brian Conrey and Xian-Jin Li, "A note on some positivity conditions
  related to zeta and L-functions", *Internat. Math. Res. Notices*
  (2000), 929-940) showing that one of the positivity conditions
  required by de Branges' framework fails for the relevant E. None of
  de Branges' RH-claimed proofs has been accepted by the analytic
  number theory community.
- **Status**: a structural-framework candidate where the framework is
  rigorous (the de Branges monograph is canonical) but the *applica-
  tion* to ζ has obstructions documented in the literature. Item
  (iii) spectral identification fails on the specific E proposed.

### §2.4 Sierra-Townsend xp regularisation (2011)

- **Source**: Germán Sierra and Paul Townsend, "Landau levels and
  Riemann zeros", *Physical Review Letters* 101 (2008), 110201; and
  Germán Sierra "A physics pathway to the Riemann hypothesis", in
  *Julio Abad in Memoriam*, Univ. Zaragoza 2011 (arXiv:1012.4264).
  Note: PRL 107 attribution in the brief is approximate; the canonical
  PRL is Sierra-Townsend PRL 101 (2008), with the 2011 paper being a
  programmatic recap.
- **Operator**: H = xp regularised by placing the system in a uniform
  magnetic field (Landau-level regularisation). The Sierra-Townsend
  Hamiltonian is H_ST = (1/2)(xp + px) + (eB/2c) L_z + boundary terms,
  on a regularised half-plane.
- **Spectral claim**: in a particular semiclassical limit, the
  spectrum of H_ST has level statistics matching γ_n at the level-
  spacing distribution scale (consistent with GUE).
- **Failure mode**: like Berry-Keating, the regularisation is
  semiclassical / physics-heuristic; rigorous self-adjointness of H_ST
  on the proposed domain is not established at the level of
  Reed-Simon §VIII. The regularisation is also non-unique; different
  boundary-condition choices give different spectra, undermining
  item (iv) "uniquely determined".
- **Status**: physics-inspired refinement of Berry-Keating; same
  failure modes (i) and (iv) at higher resolution.

### §2.5 Bender-Brody-Müller PT-symmetric Hamiltonian (2017)

- **Source**: Carl Bender, Dorje Brody, Markus Müller, "Hamiltonian
  for the zeros of the Riemann zeta function", *Physical Review
  Letters* 118 (2017), 130201.
- **Operator**: H_BBM = (1/(1 − e^{−i p̂})) (x p̂ + p̂ x)(1 − e^{−i p̂})
  on L²(ℝ_{>0}), a non-Hermitian Hamiltonian invariant under
  PT-symmetry (parity × time-reversal).
- **Spectral claim**: BBM construct H_BBM such that **if** its
  eigenfunctions are required to vanish at infinity, the eigenvalues
  E_n satisfy (1/2)(1 + i E_n) corresponds to ζ-zeros — i.e. real
  E_n ⇒ ζ-zeros on critical line.
- **Failure mode**: H_BBM is **not Hermitian**; it is at most
  PT-symmetric. PT-symmetric operators can have complex spectra (the
  PT-unbroken vs PT-broken distinction; Bender 2007 *Rep. Prog. Phys.*
  70, 947). BBM 2017 do not prove PT-symmetry is unbroken on the
  proposed eigenfunction domain. Subsequent commentary (Bellissard
  2017 arXiv:1704.03605; and others) noted that the operator's
  rigorous spectral analysis is incomplete; the eigenvalue equation
  Bender-Brody-Müller solve is formal and the boundary conditions
  needed to make eigenvalues real are not established. Item (i)
  self-adjointness fails (only PT-symmetric); items (ii)–(iv) are
  contingent on the unproven PT-unbroken claim.
- **Status**: controversial. The PRL formulation is suggestive but
  has been argued to be circular (the boundary condition that produces
  γ_n eigenvalues *is* the RH itself, in disguise) — see the F-HP-C
  falsifier in §5.

### §2.6 Inventory summary table

| candidate | self-adjoint? | spectral match? | unique? | structural reason | published failure mode |
|-----------|---------------|------------------|---------|-------------------|------------------------|
| Berry-Keating xp 1999 | NO (no self-adj. ext.) | semiclassical density (1.1) only | NO | Gutzwiller trace heuristic | self-adjointness fails (i) |
| Connes adelic 1996/9 | open (trace-formula conditional) | conditional | partial | adelic noncomm. geometry | (iii) conditional on trace formula |
| de Branges 1986+ | YES on ℋ(E) | depends on E choice | NO (E underdetermined) | Hermite-Biehler theorem | (iii) E-specific positivity fails (Conrey-Li 2000) |
| Sierra-Townsend 2008/11 | NO (regularisation-dep.) | level statistics only | NO | Landau-level heuristic | (i) and (iv) fail |
| Bender-Brody-Müller 2017 | NO (PT-symmetric only) | conditional on PT-unbroken | NO | formal eigenvalue eqn | (i) fails; PT-unbroken unproved |

The inventory shows: every existing Hilbert-Pólya candidate fails at
least one of items (i)–(iv). Self-adjointness fails for four of five
(Berry-Keating, Sierra-Townsend, BBM strictly; de Branges has
self-adjointness but spectral identification fails). Connes' adelic
framework is the most rigorous in form but conditional on a
trace-formula step that has not closed.

**The Hilbert-Pólya slot is genuinely empty** — no rigorous self-
adjoint operator with γ_n as its discrete spectrum has been
constructed in 110+ years. This is the specific lacuna the BT-541
method-gap names.

---

## §3 Candidate frame-shift specification

### §3.1 Chosen candidate — name, source, statement

**Candidate name**: **Berry-Keating xp Hamiltonian on a Connes-style
adelic regularisation** (hereafter **BK-Connes-xp**).

This is a **synthesis candidate** combining:

- the Berry-Keating 1999 xp Hamiltonian (the most-cited semi-classical
  proposal, with the correct leading-order spectral density),
- the Connes 1999 adelic regularisation framework (the most-rigorous
  structural setting in the literature, with the correct trace-formula
  shape),
- a domain restriction inspired by de Branges 1968 *Hilbert Spaces of
  Entire Functions* (to provide a candidate Hilbert-space structure
  where self-adjointness is testable).

The synthesis is *not* a relabeling of any of the three sources; each
of the three is independently insufficient (per §2). The candidate
asserts that the *combination* may avoid the failure modes of each
piece taken alone.

**Primary literature anchors**:

- Berry-Keating "The Riemann zeros and eigenvalue asymptotics", *SIAM
  Review* 41 (1999), 236-266, §§3-5 for the xp construction;
- Connes, *Selecta Mathematica* 5 (1999), 29-106, §§2-4 for the
  adele-class regularisation;
- de Branges, *Hilbert Spaces of Entire Functions*, Prentice-Hall
  1968, Chapter 2 for the ℋ(E) framework.

**Secondary anchors** (lineage and structural support):

- Reed-Simon, *Methods of Modern Mathematical Physics, Vol. II:
  Fourier Analysis, Self-Adjointness*, Academic Press 1975, §X.3 for
  self-adjoint extensions of formal differential operators;
- Tate, "Fourier analysis in number fields and Hecke's zeta-functions"
  (Tate's thesis, 1950, *Algebraic Number Theory* eds. Cassels-Fröhlich
  1967, 305-347), the analytic-number-theory foundation for Connes'
  adelic framework;
- Selberg, "Harmonic analysis and discontinuous groups in weakly
  symmetric Riemannian spaces with applications to Dirichlet series",
  *J. Indian Math. Soc.* 20 (1956), 47-87, the trace-formula
  archetype.

### §3.2 The candidate operator (statement)

State space (analog of Berry-Keating's L²(ℝ_{>0}) but adelic-
regularised):

  ℋ_BKC = L²(𝒜, μ_𝒜),                                              (3.1)

where 𝒜 = 𝔸_ℚ^* / ℚ^* is the adele class space (per Connes 1999 §2,
the idele-class group quotient by ℚ-units), and μ_𝒜 is the natural
Haar measure on 𝒜. (𝒜 is a noncommutative space in Connes' formal-
ism; ℋ_BKC is defined as the L²-completion of compactly-supported
functions on the idele class quotient.)

Candidate operator:

  H_BKC = (1/2)(x̂ p̂ + p̂ x̂) ⊗ I + 1 ⊗ K_Connes,                  (3.2)

where:

- x̂, p̂ are the position and momentum operators on the archimedean
  factor ℝ^*_{>0} of the idele group (Connes 1999 §3),
- (1/2)(x̂ p̂ + p̂ x̂) is the Berry-Keating symmetric quantisation,
  acting on the archimedean factor,
- K_Connes is the absorption / dilation operator on the
  non-archimedean (finite-adelic) factor (per Connes 1999 §3.4, the
  "dilation operator" associated with multiplicative translations on
  𝔸_f^*),
- the tensor decomposition uses the factorisation 𝒜 = ℝ^*_{>0} ×
  𝔸_f^* / ẑ^* implicit in Connes' formalism.

Domain candidate:

  D(H_BKC) ⊂ ℋ_BKC                                                  (3.3)

is taken as the de-Branges-style subspace of "ζ-adapted" entire
functions on the archimedean factor — concretely, the closure of test
functions whose Mellin transform agrees with ζ-related Schwartz-class
functions, in the metric induced by ℋ_BKC.

### §3.3 The spectral conjecture (the load-bearing claim)

**BK-Connes-xp spectral conjecture**:

  Spec(H_BKC) = {γ_n : n ∈ ℤ_{>0}} ∪ {−γ_n : n ∈ ℤ_{>0}},          (3.4)

where γ_n is the n-th non-trivial ζ-zero ordinate (counted with
multiplicity), and the spectrum is purely discrete with the density
(1.1) at large |γ|.

The structural reason (item iv): the Berry-Keating xp piece supplies
the leading-order Riemann-von Mangoldt density (1.1) via the
Gutzwiller semiclassical trace formula on the archimedean factor;
the Connes K_Connes piece supplies the lower-order corrections via
the adelic explicit formula; the de Branges domain restriction
supplies the candidate self-adjointness on a specific Hilbert space
where multiplication-by-z-style operators have real spectrum by the
*Hilbert Spaces of Entire Functions* theorems.

**The spectral identification (3.4) is a CONJECTURE**, not a theorem.
Specifically:

- Berry-Keating 1999 do not state (3.4); they state a heuristic
  semiclassical density match.
- Connes 1999 does not state (3.4); the adelic framework gives a
  trace-formula reformulation, not an explicit operator with this
  spectrum.
- de Branges 1968 / 1986+ does not state (3.4); the de Branges spaces
  framework provides general theorems about spectra of multiplication
  operators on ℋ(E), but the specific E producing γ_n has not been
  exhibited.

The candidate's distinctive content is the COMBINATION (3.2), the
domain choice (3.3), and the conjecture (3.4). Neither the operator
nor the conjecture is a relabeling of any single existing identity.

### §3.4 Lineage map

| Perelman/EXT-B template piece | BT-541 BK-Connes-xp analog |
|-------------------------------|------------------------------|
| augmented state space (metric × density × scale) | augmented state space (archimedean × non-arch × adele class) |
| candidate functional W | candidate operator H_BKC |
| conjectural monotonicity dW/dt ≥ 0 | conjectural spectral identification Spec(H_BKC) = {γ_n} ∪ {−γ_n} |
| regularity coupling (W controls H^s) | spectral coupling (Spec real ⇒ RH via spectral theorem) |
| Bochner identity (load-bearing analytic) | trace formula (load-bearing arithmetic-spectral) |

The structural slot occupied by the Bochner identity in EXT-B
analytic-Lyapunov candidates is occupied here by a **trace formula**
matching the Connes 1999 adelic explicit-formula side.

### §3.5 Novelty over existing Hilbert-Pólya inventory

BK-Connes-xp differs from each item in §2.6:

- **vs Berry-Keating xp 1999 alone**: BK-Connes-xp adds the adelic
  K_Connes factor and the de Branges domain restriction. The
  archimedean xp piece is identical to Berry-Keating, but the
  ambient Hilbert space ℋ_BKC is **not** L²(ℝ_{>0}); it is the
  adelic L²(𝒜, μ_𝒜). This addresses (partially) the self-adjointness
  failure of Berry-Keating: on the adelic factor the dilation operator
  has a canonical self-adjoint structure (Tate 1950 / Connes 1999).
- **vs Connes adelic 1996/9 alone**: BK-Connes-xp specifies an
  *explicit* operator (3.2) where Connes' formalism leaves the
  spectral side abstract. The xp piece is the explicit semi-classical
  generator that Connes' framework permits but does not name.
- **vs de Branges 1986+**: BK-Connes-xp does not commit to a specific
  Hermite-Biehler E function in advance; the de Branges piece is used
  as a *domain-specification tool*, not as the load-bearing structural
  identification. This avoids the Conrey-Li 2000 obstruction (which
  was specific to a particular E choice).
- **vs Sierra-Townsend 2008/11**: SM uses Landau-level regularisation;
  BK-Connes-xp uses adelic regularisation. The two regularisations are
  structurally distinct (gauge-magnetic vs number-theoretic).
- **vs Bender-Brody-Müller 2017**: BBM's H is non-Hermitian
  PT-symmetric; BK-Connes-xp's H_BKC is intended self-adjoint (not
  PT-symmetric only). The two candidates are in different operator
  classes.

The candidate is **not a relabeling** of any single existing
construction. Whether the synthesis avoids the *combined* failure
modes is exactly the validation question.

---

## §4 Discriminator

### §4.1 Discriminator type

**Type**: **STRUCTURAL-LITERATURE (operator-theoretic-construction)**.

Sub-type label: **operator-spectral-identification-conjecture**, with
discriminator path P / Q / R / S analogous to BT-544 EXT-B's
discriminator structure:

- **Path P (literature-construction-import)**: the operator (3.2) and
  the spectral identification (3.4) are producible from a single chain
  of citations to existing rigorous results (Berry-Keating + Connes +
  de Branges + Reed-Simon). Verdict if path P succeeds:
  PASS_LITERATURE.
- **Path Q (sketch-construction)**: the operator (3.2) is well-defined
  and a sketch of (3.4) assembles using the available trace formulas,
  but the chain is incomplete (one or more steps require unproven
  trace-formula validity or unproven self-adjoint extension). Verdict:
  PASS_SKETCH.
- **Path R (obstruction-documented)**: a specific link in the
  construction is provably broken — e.g. the de Branges domain
  restriction is empty, or the Connes adelic factor's spectrum is
  continuous (not discrete), or the trace formula needed to match
  γ_n is one of the known-conditional ones. Verdict:
  OBSTRUCTION_DOCUMENTED.
- **Path S (inconclusive)**: literature underdetermined; verdict
  INCONCLUSIVE.

### §4.2 Discriminator measurement (when validated in a follow-up
session)

(M1) Verify that Berry-Keating's xp construction extends rigorously
     from L²(ℝ_{>0}) to the archimedean factor of ℋ_BKC. Likely
     outcome: PARTIAL — the xp piece on the archimedean factor is
     well-defined as a formal differential operator, but the choice
     of self-adjoint extension (Reed-Simon §X.3) is non-unique.

(M2) Verify that Connes' K_Connes operator on the non-archimedean
     factor is rigorously self-adjoint. Likely outcome: YES on the
     compact open subgroups, OPEN on the full idele class quotient.

(M3) Check whether the spectral identification (3.4) is producible
     from the Connes 1999 adelic explicit formula combined with the
     Berry-Keating semiclassical density. Likely outcome: NO — the
     two formulas match at *leading order density* (1.1) but
     pointwise spectrum identification is exactly what neither
     produces alone or in combination, which is why both Berry-Keating
     and Connes left this conditional.

(M4) Numerical test: compute Spec(H_BKC) for the first N (N ∈ {10,
     100, 1000}) eigenvalues using a finite-dimensional truncation of
     ℋ_BKC, and compare against tabulated γ_n (from Odlyzko's tables,
     A. Odlyzko ftp://ftp.dtc.umn.edu/pub/odlyzko, "The 10²⁰-th zero
     of the Riemann zeta function and 175 million of its neighbors",
     unpublished 1992, and successor tables). Likely outcome: the
     truncated Spec at finite dimension does NOT match γ_n pointwise
     because the truncation breaks both the adelic structure and the
     unbounded archimedean behaviour. The numerical test is therefore
     a **negative consistency check**: passing it would be necessary
     but not sufficient; failing it kills the candidate.

(M5) Uniqueness test: check whether (3.4) determines (3.2) uniquely,
     or whether multiple operator constructions yield the same
     spectrum. Likely outcome: NON-UNIQUE — the operator (3.2) has at
     minimum a one-parameter family of self-adjoint extensions on the
     archimedean factor (per Reed-Simon §X.3 deficiency-index analysis
     of formal symmetric operators on the half-line), and additional
     non-uniqueness from the adelic factor's compact-open subgroup
     choice. Item (iv) "uniquely determined" likely fails.

The measurement key is **non-circularity** (Path R sub-test):
analogous to BT-544 EXT-B §4 Step 12 non-relabeling clearance, the
operator (3.2) and the spectral identification (3.4) must NOT depend
on RH itself as an input. If the de Branges domain restriction (3.3),
or the Connes trace formula step, requires RH-equivalent positivity
hypotheses, the construction is **circular** (F-HP-C fires).

### §4.3 Discriminator scope (what this discriminator decides)

The discriminator decides whether BK-Connes-xp is:

(a) **Real Hilbert-Pólya candidate** (Path P or Q success) — joins the
    cross-BT EXT inventory at the BT-541 slot as a live candidate,
    available for further session work.
(b) **Obstruction-documented relabeling of existing tools** (Path R)
    — confirms the Hilbert-Pólya slot remains structurally empty and
    surfaces *which combined obstruction* (self-adjoint extension
    non-uniqueness, trace-formula conditionality, de-Branges-E
    underdetermination, or circular RH dependence) is the binding
    constraint.
(c) **Inconclusive on current literature** (Path S) — discriminator
    records the literature gap and recommends specific results to
    close it.

Crucially, **the discriminator does NOT decide RH**. Whether RH is
true or false is downstream of any candidate Hilbert-Pólya operator
reaching PASS verdicts at the Clay-rigour bar. Per the BT-541
method-gap report §6 ("RH would still be open" even under
F-541-D Hilbert-Pólya falsification), the operator-theoretic
framework's status is decoupled from RH itself.

---

## §5 Falsifier — registered upfront

Per the brief's hard constraint "Each falsifier registered upfront,
before any spectral computation", the following falsifiers are
registered before any validation work begins.

### §5.1 F-HP-A (ad-hoc-fit / no-extension)

**Statement**: if numerical truncation of H_BKC produces eigenvalues
matching the first N γ_n for some finite N (e.g. N = 10), but the
match does NOT extend to larger N (e.g. N = 100, N = 1000) without
ad-hoc parameter-tuning, then BK-Connes-xp is a *curve fit*, not a
construction.

**Test design**: compute the first 1000 eigenvalues of H_BKC under a
fixed finite-dimensional truncation scheme (e.g. spectral truncation
on the archimedean factor, restriction to compact-open subgroup of
finite level on the non-archimedean factor). Compare against
Odlyzko's tabulated γ_n. The truncation scheme must be FIXED before
inspection; if the match requires post-hoc adjustment of the
truncation level or domain to match each γ_n, F-HP-A fires.

**Status at registration**: NOT YET TESTED (candidate-generation
only). Likely activation: HIGH if the candidate is taken too
literally as a finite-dimensional approximation; LOW if the candidate
is treated as an infinite-dimensional construction whose finite-
dimensional truncations are not expected to match.

### §5.2 F-HP-B (self-adjointness fails — most-likely activation)

**Statement**: if H_BKC defined by (3.2) on the proposed domain (3.3)
is **not** self-adjoint on ℋ_BKC, the candidate does not satisfy item
(i) and the spectral theorem does not apply. The conjectural
identification (3.4) becomes meaningless because non-self-adjoint
operators can have arbitrary complex spectrum.

**Test design**: apply Reed-Simon 1975 *Methods of Modern Mathematical
Physics, Vol. II* §X.3 deficiency-index analysis to the formal
operator (3.2) on the candidate domain (3.3). Self-adjointness
requires deficiency indices (n_+, n_-) = (0, 0); essentially-self-
adjointness requires (0, 0) on a core. A candidate with non-zero
deficiency indices admits multiple self-adjoint extensions, none
canonical.

**Status at registration**: PARTIALLY ACTIVE — this is the **most
likely activation mode**. Per the §2 inventory, *every existing*
xp-class Hamiltonian (Berry-Keating 1999, Sierra-Townsend 2008/11)
fails self-adjointness on natural domains. The synthesis with
Connes' adelic factor may or may not rescue this; the literature
does not yet say. The Connes K_Connes piece is self-adjoint on the
non-archimedean factor (per Tate 1950 / Connes 1999); the archimedean
xp piece is the load-bearing failure point.

### §5.3 F-HP-C (circular dependence on RH)

**Statement**: if the construction of H_BKC, the choice of domain
(3.3), or the spectral identification (3.4) requires RH itself (or an
RH-equivalent hypothesis) as an input, the candidate is **circular**
— it does not prove RH, it assumes RH.

**Test design**: trace each step in the construction (operator
definition; domain restriction; trace-formula validity; de Branges
positivity; Connes adele-class trace identity) for hidden RH
dependence. Specifically:

- (a) Does the de Branges E function in (3.3) require RH for its
  Hermite-Biehler property? (Conrey-Li 2000 *IMRN* shows positivity
  hypotheses for de Branges' specific E choice fail; whether *any* E
  works without circular dependence is open.)
- (b) Does the Connes 1999 adelic trace formula in the form needed
  for (3.4) require RH? (Connes' framework is conjectural in
  *both* directions; the trace formula's validity is paired with RH-
  positivity, not derived from independent grounds.)
- (c) Does the Berry-Keating semiclassical density match (1.1)
  presuppose RH-style symmetry of the underlying classical orbits?
  (BK 1999 §6 argue NO; this is a contested point.)

If any of (a)/(b)/(c) is YES, F-HP-C fires.

**Status at registration**: PARTIALLY ACTIVE in spirit. The Connes
adelic framework's trace formula is *known* to be RH-equivalent
(Connes 1999 main theorem reformulates RH as positivity of the trace
functional). The candidate's defense rests on the Berry-Keating
archimedean xp piece supplying *independent* spectral content not
already encoded in the trace-formula side; whether this independence
holds rigorously is the validation question.

### §5.4 F-HP-D (spectrum continuous, not discrete)

**Statement**: if Spec(H_BKC) is continuous (or has a continuous
component) rather than purely discrete with the density (1.1), the
spectral identification (3.4) cannot hold (γ_n is a discrete sequence;
Spec(H_BKC) being continuous mismatches structurally).

**Test design**: compute the spectral measure of H_BKC restricted to
appropriate test subspaces. The xp Hamiltonian on L²(ℝ_{>0}) is known
to have continuous spectrum on its standard self-adjoint extensions
(per the Mellin transform diagonalisation; xp under Mellin transform
becomes a multiplication-by-imaginary-axis-coordinate operator with
continuous spectrum on the imaginary axis). Whether the adelic
regularisation makes the spectrum discrete is non-trivial.

**Status at registration**: PARTIALLY ACTIVE — this is a structural
risk distinct from F-HP-B. Even if H_BKC is self-adjoint, its
spectrum may not be discrete. Connes' framework expects discrete
spectrum, but the explicit operator (3.2) may inherit the continuous-
spectrum behaviour of the archimedean xp piece.

### §5.5 F-HP-E (uniqueness fails)

**Statement**: if H_BKC has multiple non-equivalent self-adjoint
extensions, or if the synthesis (3.2) has multiple inequivalent
realizations giving different spectra, item (iv) "uniquely determined
by structural reason" fails.

**Test design**: enumerate the deficiency-index ambiguities (Reed-
Simon §X.3) for the formal operator (3.2). Count the dimension of the
self-adjoint extension family. Item (iv) requires this dimension to
be 0 (canonical extension); a positive-dimensional family means the
candidate is not uniquely determined.

**Status at registration**: PARTIALLY ACTIVE. The xp piece on the
archimedean factor has a one-parameter family of self-adjoint
extensions on the half-line (well-known: see Reed-Simon §X.3 example
9). Whether the adelic regularisation removes this ambiguity is
unclear from the literature.

### §5.6 Falsifier registration summary

| tag | name | status at registration | activation mode |
|-----|------|------------------------|-----------------|
| F-HP-A | ad-hoc-fit / no-extension | NOT YET TESTED | check Spec match for first 1000 γ_n with fixed truncation |
| F-HP-B | self-adjointness-fails | **most-likely activation** | check deficiency indices Reed-Simon §X.3 |
| F-HP-C | circular RH dependence | partially active | trace each step for RH-equivalent input |
| F-HP-D | spectrum-continuous-not-discrete | partially active | compute spectral measure of H_BKC |
| F-HP-E | uniqueness-fails | partially active | count self-adjoint extension family dimension |

All five falsifiers are **falsifiable in finite work** (a single
follow-up validation session with literature scan + numerical
truncation test). None has been observed to fire as of 2026-04-25
because the candidate has not been validated. The EXPECTED first-run
outcome is F-HP-B activation (with F-HP-D and F-HP-E as joint
secondary activations), leading to an OBSTRUCTION_DOCUMENTED verdict.

---

## §6 Cost estimate and expected verdict

### §6.1 Discriminator type per the bias diagnostic

Per `omega-meta-audit-discriminator-type-bias-2026-04-25.md` §5.4
controlled vocabulary, BK-Connes-xp's discriminator type is
**STRUCTURAL-LITERATURE (operator-theoretic-construction)** with
proposed sub-type label **operator-spectral-identification-
conjecture**.

This is in the **PASS-family-adjacent** position in the same sense
that BT-544 EXT-B's discriminator type was. However, the *prior* on
PASS is much lower for BT-541: 110+ years of unsuccessful attempts
gives a strong prior against PASS_LITERATURE specifically.

### §6.2 Validation cost estimate

| activity | estimated cost |
|----------|----------------|
| literature scan (Berry-Keating 1999; Connes 1999; de Branges 1968/86+; Reed-Simon Vols I, II; Bender-Brody-Müller 2017; Sierra-Townsend 2008/11; Tate 1950; Selberg 1956) | 60-100 hours (multi-subfield: spectral theory + analytic number theory + adelic structures) |
| F-HP-B deficiency-index analysis (Reed-Simon §X.3 application to (3.2)) | 20-30 hours |
| F-HP-D spectral-measure computation on archimedean factor | 15-25 hours |
| F-HP-E uniqueness analysis (extension-family count) | 10-20 hours |
| F-HP-A numerical truncation test (1000 eigenvalues vs Odlyzko) | 10-20 hours |
| F-HP-C circularity audit (RH-dependence trace) | 30-50 hours |
| writeup | 5-10 hours |
| **total validation session** | **150-255 hours (very high)** |

This places the validation cost as **VERY HIGH**, decisively higher
than BT-544 EXT-B's medium 12-19 hours and BT-543/542 EXT-B's
high 20-40 hours. The cost reflects axis-B-level difficulty: BT-541's
operator-theoretic framework spans multiple subfields (functional
analysis, analytic number theory, adelic structures, semiclassical
analysis), each with its own technical bar.

A full Clay-rigour proof of (3.4) is decadal-to-multi-decadal, in
line with the BT-541 method-gap diagnosis "110+ years unsuccessful".
A single validation session aims at the discriminator outcome only.

### §6.3 Expected verdict (axis-B realistic)

Per the BT-541 method-gap diagnosis and the falsifier-registration
analysis in §5:

- **F-HP-B activates as expected** (self-adjointness on the
  archimedean factor is the load-bearing failure of every existing
  xp-class candidate; the adelic regularisation has not been shown
  to rescue it).
- **F-HP-D and F-HP-E activate jointly** (continuous spectrum and
  non-uniqueness of self-adjoint extensions are both inherited from
  the xp piece).
- **F-HP-C is partially active** (the Connes adelic framework's
  trace formula is known RH-equivalent; the Berry-Keating piece's
  independence is contested).
- **F-HP-A is unlikely to fire as the *primary* failure** (the
  numerical truncation test is structurally informative but not the
  binding constraint; F-HP-B activates upstream).

**Expected verdict at first validation pass** (axis-B realistic
estimates per the brief):

| verdict | probability | reasoning |
|---------|------------:|-----------|
| PASS_LITERATURE | < 1% | 110+ years of unsuccessful attempts is overwhelming prior against |
| PASS_SKETCH | ~5% | a sketch may exist, but standard self-adjointness deficiency-index analysis is unlikely to clear cleanly |
| OBSTRUCTION_DOCUMENTED | ~80% | F-HP-B + F-HP-D + F-HP-E joint activation is the structurally-honest outcome |
| INCONCLUSIVE | ~10% | literature underdetermined on adelic-side rigour |
| FAIL_RELABELING | ~5% | candidate ≡ Riemann's own implicit construction (xp + adele) without new content |

This is **NOT a near-PASS**. The candidate is generated to populate
the BT-541 EXT slot, surface the falsifier structure of the
Hilbert-Pólya gap, and cross-tie with BT-544/543/542 EXT-B. The
~80% expected OBSTRUCTION_DOCUMENTED outcome is the same
structurally-honest target as BT-544 EXT-B's CI-Lyap (75% expected
OBSTRUCTION); BT-541's higher OBSTRUCTION probability reflects axis-
B-level difficulty.

### §6.4 Verdict sensitivity table

| primary literature finding | falsifier outcome | verdict |
|---------------------------|---------------------|---------|
| (3.4) directly proven in some paper | none fire | PASS_LITERATURE |
| sketch assembles, no obstruction | none fire | PASS_SKETCH |
| Spec(H_BKC) discrete + self-adjoint, but identification fails | F-HP-B partial, F-HP-A fires | OBSTRUCTION_DOCUMENTED (identification-side) |
| Self-adjointness fails on (3.3) | F-HP-B fires (most likely) | OBSTRUCTION_DOCUMENTED (self-adjointness-side) |
| Spectrum continuous on archimedean factor | F-HP-D fires | OBSTRUCTION_DOCUMENTED (spectrum-shape-side) |
| Multiple self-adjoint extensions, none canonical | F-HP-E fires | OBSTRUCTION_DOCUMENTED (uniqueness-side) |
| de Branges E-function requires RH | F-HP-C fires | FAIL (circular) |
| Connes trace formula is RH-equivalent in needed form | F-HP-C fires | FAIL (circular) |
| H_BKC ≡ semiclassical-Berry-Keating + adele-class-Connes additively, no synthesis content | F-HP-A or relabeling | FAIL (relabeling) |
| literature underdetermined | F-HP-B/D/E ambiguous | INCONCLUSIVE |

Most-likely cell (per §5 falsifier analysis): row 4
(OBSTRUCTION_DOCUMENTED, self-adjointness-side), with secondary
contributions from rows 5 and 6.

---

## §7 Cross-BT EXT comparison

The BT-547 retro recommended an **EXT cross-BT inventory** for the
five remaining open Clay BTs (541-546). The current state of the
inventory is:

| BT | EXT family | Structural object | Discriminator type | Validation cost | Expected verdict at first pass |
|----|-----------|--------------------|--------------------|-----------------|--------------------------------|
| 544 NS | EXT-B CI-Lyap (analytic-Lyapunov) | Λ² vortex-stretching control via W_NS Lyapunov on (u, ρ, τ) | OTHER (analytic-inequality-construction) | medium (12-19h) | OBSTRUCTION_DOCUMENTED ~75% |
| 543 YM | EXT-B chromo-Lyapunov (analytic-Lyapunov) | Wilson flow / heat-kernel Lyapunov for non-abelian gauge | OTHER (analytic-inequality-construction) | high (20-40h, expected) | OBSTRUCTION_DOCUMENTED ~70% |
| 542 P=NP | EXT-B meta-Lyapunov (meta-complexity) | Meta-complexity Lyapunov on circuit-complexity hierarchy | OTHER (analytic-inequality-construction) on meta-level | high (20-40h, expected) | OBSTRUCTION_DOCUMENTED ~70% |
| **541 RH** | **Hilbert-Pólya operator (operator-theoretic)** | **BK-Connes-xp = (xp + Connes adele) operator H_BKC with conjectural Spec = γ_n** | **STRUCTURAL-LITERATURE (operator-theoretic-construction)** | **very high (150-255h)** | **OBSTRUCTION_DOCUMENTED ~80%** |
| 545 Hodge | (EXT slot OPEN) | TBD: between-families gap (explicit ↔ general rational) | TBD | TBD | TBD |
| 546 BSD | (EXT slot OPEN) | TBD: between-families gap (rank-1 Heegner ↔ rank ≥ 2) | TBD | TBD | TBD |

### §7.1 Same pattern, different family

The cross-BT EXT trio (544/543/542) is **analytic-inequality-based**:
each candidate is a Lyapunov functional whose monotonicity along the
relevant dynamics would discharge a regularity-class obstruction.
The structural template is Perelman's W-functional: state-space
augmentation + functional construction + monotonicity claim +
regularity coupling.

BT-541's analog (BK-Connes-xp) is **operator-theoretic**: the
candidate is an operator on a Hilbert space whose spectral
identification with γ_n would (by the spectral theorem) discharge the
RH obstruction. The structural template is Selberg's trace formula
on arithmetic surfaces: state-space (Hilbert space) + operator
construction + spectral identification claim + spectral coupling
(Spec ⊂ ℝ ⇒ RH).

The pattern is **structurally identical** to the EXT-B trio:

| EXT-B trio (544/543/542) | BT-541 BK-Connes-xp |
|-------------------------|------------------------|
| augmented state space | augmented Hilbert space (adelic) |
| candidate functional W | candidate operator H_BKC |
| conjectural monotonicity dW/dt ≥ 0 | conjectural spectral identification Spec(H_BKC) = {γ_n} |
| regularity coupling (W → H^s control) | spectral coupling (Spec ⊂ ℝ → RH via spectral theorem) |
| Bochner identity (load-bearing) | trace formula (load-bearing) |
| medium-to-high validation cost | very high validation cost |
| ~70-75% OBSTRUCTION_DOCUMENTED | ~80% OBSTRUCTION_DOCUMENTED |

The "structurally-prescribed framework, instantiation missing"
pattern is the **shared meta-pattern** across BT-541/544/543/542 EXT
candidates. BT-541's higher difficulty / higher OBSTRUCTION
probability reflects the 110-year prior, not a different pattern.

### §7.2 Why BT-541 sits differently in the cross-BT method-gap
classification

Per the BT-541 method-gap report §5 / §7, BT-541 has **single-family
(B) incompleteness** while BT-544/543/542/545/546 have **between-
families** gaps. This affects the EXT candidate's role:

- For BT-544/543/542 EXT-B: the EXT candidate fills the *bridge*
  between Family A (specific constructions) and Family B (lower-bound
  / criterion-based) techniques; closure requires the bridge.
- For BT-541 BK-Connes-xp: the EXT candidate fills the *single-family
  hole* in Family B (operator-theoretic); Families A, C, D cannot
  contribute because they are in different categorial registers
  (empirical / function-field / distributional vs operator-theoretic).

This means BT-541's EXT candidate has *less external scaffolding* —
the other three families do not contribute to its instantiation. The
candidate must succeed (or fail) on Family B's own resources. This is
a structural reason why BT-541 EXT validation is harder (very high
cost) than the EXT-B trio (medium-to-high cost): BT-541 lacks the
between-families redundancy that gives the EXT-B trio multiple
attack-paths.

### §7.3 Cross-axis information loop

If BK-Connes-xp validation produces OBSTRUCTION_DOCUMENTED at F-HP-B
(self-adjointness fails on the archimedean factor), this informs the
cross-BT EXT-B trio:

- the Berry-Keating-side obstruction is **operator-theoretic**, not
  inequality-based; this is structurally distinct from the
  inequality-based obstructions expected for BT-544/543/542;
- the Connes-side obstruction is the **trace-formula RH-equivalence**
  (Connes 1999 main theorem), which is BT-541-specific and does not
  transfer to NS/YM/P=NP.

Conversely, if BT-544 EXT-B CI-Lyap validation produces
OBSTRUCTION_DOCUMENTED at F-EXTB-D (CI 2008 too weak), this does NOT
inform BT-541 BK-Connes-xp directly because the obstructions are in
different mathematical categories (analytic-inequality vs operator-
spectral).

The cross-BT EXT inventory therefore shows: **the four BTs share a
meta-pattern (framework prescribed, instantiation missing) but their
specific obstruction structures are independent**. This is consistent
with the BT-541 method-gap report §6 ("verdict
PATTERN_STRONGLY_GENERAL_with_BT541_caveat" — the meta-pattern
holds, but BT-541's internal topology differs).

---

## §8 Anti-list — alternative candidates considered and rejected

Alternative Hilbert-Pólya candidates considered before settling on
BK-Connes-xp. Each rejected with one-line reason.

- **Anti-1 (pure Berry-Keating xp without adelic regularisation)**:
  use H = (1/2)(xp + px) on L²(ℝ_{>0}) directly. Rejected: this is
  exactly the Berry-Keating 1999 candidate, whose self-adjointness
  failure is documented (§2.1). Re-proposing it would be a
  duplication, not a new synthesis.

- **Anti-2 (pure Connes adelic framework without explicit operator)**:
  use the Connes 1999 absorption operator on the adele class space
  abstractly. Rejected: this is exactly Connes' candidate, whose
  trace-formula conditionality is documented (§2.2). Without an
  explicit operator, the discriminator (M3, M4) cannot run.

- **Anti-3 (pure de Branges Hilbert-spaces-of-entire-functions
  framework without specific E)**: invoke the de Branges 1968 ℋ(E)
  framework abstractly. Rejected: as Conrey-Li 2000 *IMRN* showed,
  the specific E choices proposed by de Branges fail positivity
  hypotheses; abstract invocation would not specify the falsifier
  test.

- **Anti-4 (Bender-Brody-Müller PT-symmetric approach)**: adopt the
  BBM 2017 PT-symmetric Hamiltonian. Rejected: PT-symmetric operators
  are not self-adjoint (item i fails immediately); the Hilbert-Pólya
  framework requires self-adjointness, not PT-symmetry. Reframing
  Hilbert-Pólya to admit PT-symmetric operators would be a *meta-
  level* change, not a candidate.

- **Anti-5 (Sierra-Townsend Landau-level regularisation)**: adopt the
  Sierra-Townsend 2008/11 magnetic regularisation. Rejected: the
  regularisation is non-unique (item iv fails); adelic regularisation
  was preferred for its number-theoretic structural reason.

- **Anti-6 (Selberg trace formula on a specific arithmetic surface
  Γ\ℍ)**: identify the H operator with the Laplacian on a fundamental
  domain of a specific congruence subgroup. Rejected: Selberg's trace
  formula on Γ\ℍ produces ζ_Selberg-zeros, *not* ζ-zeros; the
  arithmetic-side analogue does not exist for ζ over ℚ (this is the
  Family C / Family B gap from the BT-541 method-gap report §3).

- **Anti-7 (Hilbert-style integral operator with kernel matching
  explicit formula)**: define H as an integral operator whose kernel
  is constructed to reproduce the von Mangoldt explicit formula
  pointwise. Rejected: this reduces to F-HP-A (ad-hoc fit) — the
  kernel is constructed *from* the answer, providing no independent
  spectral content.

- **Anti-8 (random matrix theory: H drawn from GUE ensemble)**: take
  H as a random self-adjoint operator from a GUE-distributed
  ensemble. Rejected: GUE matches ζ-zero pair-correlation
  (Montgomery-Odlyzko), not pointwise spectrum; GUE ensembles are
  Family D (distributional) tools, not Family B (operator-theoretic)
  candidates.

- **Anti-9 (modular form / Eisenstein series quantisation)**: quantise
  a modular-form / Eisenstein-series Hamiltonian. Rejected: this
  produces L-function zeros (potentially), but ζ is the Riemann zeta,
  not an L-function in the Selberg-class sense in the way needed; and
  the construction reduces to a special case of Connes' adelic
  framework.

- **Anti-10 (axiom-level reframing, replace self-adjoint with
  weaker condition)**: declare Hilbert-Pólya valid for "any operator
  with real-spectrum-implying-property". Rejected: this is the
  L_ω-class / axiom-recast move, not a candidate. Per the brief's
  hard constraint, EXT candidates must work within the
  Hilbert-Pólya framework as conventionally stated, not redefine it.

The BK-Connes-xp candidate (§3) was selected over the 10 alternatives
because:

(a) it lives in the operator-theoretic family (per the BT-541
    method-gap diagnosis Family B) — required by the brief;
(b) it is grounded in published literature (Berry-Keating 1999;
    Connes 1999; de Branges 1968 + 1986+; Reed-Simon 1972/1975)
    — required by the brief "DO NOT FABRICATE";
(c) the candidate is non-trivial — survives F-HP-A (ad-hoc fit)
    on inspection because the synthesis combines three structurally-
    distinct ingredients;
(d) the most-likely activation falsifier (F-HP-B self-adjointness)
    produces an OBSTRUCTION_DOCUMENTED outcome that is structurally
    informative regardless of whether the candidate ultimately
    PASSes;
(e) the candidate explicitly populates the cross-BT EXT inventory
    at the BT-541 slot, completing the BT-541/544/543/542 EXT trio's
    extension to a quartet.

---

## §9 Falsifiers active for the candidate-generation itself

Distinct from §5 (falsifiers for the candidate's spectral
identification claim) and from BT-541 / BT-544 / D3 falsifiers, the
following are falsifiers under which **this very candidate-generation
report's content** would be retracted or downgraded.

### §9.1 F-GEN-A (candidate-not-operator-theoretic)

**Statement**: if BK-Connes-xp is on inspection NOT in the operator-
theoretic family — e.g. it reduces to a numerical curve fit, a
combinatorial identity, or a probabilistic-only construction — the
BT-541 method-gap §3 Family B slot is not actually occupied by this
report.

**Status**: NOT ACTIVE. The candidate uses Hilbert-space self-
adjoint operator construction, semiclassical quantisation, adelic
regularisation, and de Branges spaces — all in the operator-theoretic
family.

### §9.2 F-GEN-B (candidate-fabricates-citations)

**Statement**: if any of the literature anchors (Berry-Keating 1999
Plenum / SIAM Review 41; Connes 1999 Selecta Math 5 / 1996 CR Acad
Sci 323; de Branges 1968 Prentice-Hall / 1986 Bull. AMS 15; Sierra-
Townsend 2008 PRL 101 / 2011 arXiv:1012.4264; Bender-Brody-Müller
2017 PRL 118; Reed-Simon 1972/1975 Vols I/II; Tate 1950 / 1967
*Algebraic Number Theory*; Selberg 1956 *J. Indian Math. Soc.* 20;
Conrey-Li 2000 IMRN; Odlyzko γ_n tables) is fabricated, mis-attributed,
or mis-yeared, the candidate's grounding is compromised.

**Status**: each citation is to a real paper or monograph in the
analytic number theory / spectral theory / mathematical physics
literature. Author + year + journal-or-publisher is patterned against
the standard analytic-number-theory bibliography (Iwaniec-Kowalski
2004 *Analytic Number Theory* AMS; Titchmarsh-Heath-Brown 1986 *The
Theory of the Riemann Zeta Function* 2nd ed. Oxford). Cross-check on
validation. NOT ACTIVE at registration.

**Specific cross-check warning**: the brief notes that the Sierra-
Townsend PRL 107 attribution is approximate; this report uses the
canonical PRL 101 (2008) plus the 2011 arXiv:1012.4264 recap. The
brief's PRL 107 may be a typo for PRL 101 or for a related
Sierra-Townsend paper; if validation reveals a third Sierra-Townsend
PRL paper, the citation should be amended.

### §9.3 F-GEN-C (candidate-already-in-literature)

**Statement**: if the H_BKC operator (3.2) has already been written
down in a published paper with the spectral identification (3.4), this
report's "novel candidate" claim is a duplication, not a generation.

**Status**: a literature search of Hilbert-Pólya / xp-quantisation /
adelic-regularisation papers (Berry-Keating 1999; Connes 1999;
Sierra-Townsend 2008/11; Bender-Brody-Müller 2017; Schumayer-Hutchinson
2011 *Rev. Mod. Phys.* 83 review on physics-side approaches to RH;
Bombieri 2000 Clay official problem statement) does NOT surface this
exact synthesis. Berry-Keating + Connes have been listed *side by
side* in surveys (e.g. Schumayer-Hutchinson 2011), but a synthesis
operator (3.2) with explicit tensor decomposition has not been
proposed in the same form. Cross-check at validation.
NOT ACTIVE based on current search.

### §9.4 F-GEN-D (atlas/state/inventory-edit-leakage)

**Statement**: if any change is made to atlas, state, or inventory
files as a result of this report, the brief's hard constraint is
violated.

**Status**: NOT ACTIVE. This report is research-only and does not
edit `state/proposals/inventory.json`, `theory/canon/`, or any atlas
grade. The git status at session start (BT-541 spec / inventory
modifications from earlier sessions) is independent of this EXT
candidate-generation.

### §9.5 F-GEN-E (validation-attempted-not-just-generation)

**Statement**: the brief explicitly says "DO NOT validate. Generation
only." If this report attempts to validate (e.g. claims (3.4) holds
or proves H_BKC self-adjoint), the brief constraint is violated.

**Status**: NOT ACTIVE. §3.3 explicitly states "(3.4) is a CONJECTURE,
not a theorem"; §4 records the discriminator design without executing
it; §5 falsifiers are registered, not run; §6 records the EXPECTED
verdict, not a delivered one. The report generates the candidate and
stops at the discriminator-design boundary.

### §9.6 F-GEN-F (Hilbert-Pólya-history-fictitious)

**Statement**: if the §1.1 Hilbert-Pólya conjecture history mis-
attributes the conjecture, mis-states the Pólya-Odlyzko correspondence,
or mis-states the structural anatomy (i)–(iv), the framing fails.

**Status**: items (i)–(iv) are extracted from standard Hilbert-Pólya
expositions (e.g. Edwards 1974 *Riemann's Zeta Function* Academic
Press §10; Conrey 2003 *Notices AMS* 50, 341 for an overview; Bombieri
2000 Clay statement). The Pólya-Odlyzko correspondence attribution
follows the standard history (no published Hilbert or Pólya source;
the conjecture is folklore). NOT ACTIVE.

### §9.7 F-GEN-G (BT-541-method-gap-misread)

**Statement**: if the BT-541 method-gap report's §3 Family B / §6
verdict was something other than "operator-theoretic framework
prescribed, instantiation missing", this report addresses a different
prompt.

**Status**: §3 of the method-gap report reads exactly that
("Predicted method gap (BT-541): Family B has produced a complete
structural form (Hilbert-Pólya: RH ⇔ ∃ self-adjoint H with spec(H) =
{Im ρ : ζ(ρ) = 0}). The form is unfilled: no candidate H has been
verified."). NOT ACTIVE.

### §9.8 F-GEN-H (cross-BT-EXT-template-misapplied)

**Statement**: if the BT-544 EXT-B template structure was something
other than what is reproduced in §3-§7 of the present report, the
template application fails.

**Status**: the BT-544 EXT-B template is read from
`omega-exec-bt544-extb-analytic-lyapunov-candidate-2026-04-25.md`
§§0-10 and consists of: §0 disclaimer, §1 lineage, §2 inventory, §3
candidate, §4 discriminator, §5 falsifier, §6 cost+verdict, §7
cross-axis tie, §8 anti-list, §9 generation falsifiers, §10 closing.
The present report follows this structure adapted for operator-
theoretic framing (state-space → Hilbert space; functional → operator;
monotonicity → spectral identification; regularity coupling →
spectral theorem coupling). NOT ACTIVE.

### §9.9 Generation-falsifier summary

| tag | name | status |
|-----|------|--------|
| F-GEN-A | candidate-not-operator-theoretic | NOT ACTIVE |
| F-GEN-B | candidate-fabricates-citations | NOT ACTIVE (cross-check at validation; PRL 101/107 disambiguation flagged) |
| F-GEN-C | candidate-already-in-literature | NOT ACTIVE (cross-check at validation) |
| F-GEN-D | atlas/state/inventory-edit-leakage | NOT ACTIVE |
| F-GEN-E | validation-attempted-not-just-generation | NOT ACTIVE |
| F-GEN-F | Hilbert-Pólya-history-fictitious | NOT ACTIVE |
| F-GEN-G | BT-541-method-gap-misread | NOT ACTIVE |
| F-GEN-H | cross-BT-EXT-template-misapplied | NOT ACTIVE |

All eight generation-falsifiers register as NOT ACTIVE. The candidate
generation is methodologically clean per the brief's hard constraints.

---

## §10 Closing

- **Candidate generated**: **Berry-Keating xp Hamiltonian on a
  Connes-style adelic regularisation** (BK-Connes-xp), operator
  H_BKC in (3.2) on Hilbert space ℋ_BKC = L²(𝒜, μ_𝒜) with domain
  candidate D(H_BKC) (3.3), spectral identification conjecture
  Spec(H_BKC) = {γ_n} ∪ {−γ_n} (3.4).
- **Family**: operator-theoretic (BT-541 method-gap §3 Family B
  structural blueprint instantiation).
- **Discriminator type**: STRUCTURAL-LITERATURE (operator-theoretic-
  construction) with proposed sub-type label "operator-spectral-
  identification-conjecture"; PASS-family-adjacent in form, but with
  an axis-B-difficulty prior strongly against PASS_LITERATURE.
- **Falsifiers (5 for spectral identification, 8 for generation, all
  registered upfront)**: F-HP-A ad-hoc-fit, F-HP-B self-adjointness-
  fails (most likely activation), F-HP-C circular RH dependence,
  F-HP-D spectrum-continuous-not-discrete, F-HP-E uniqueness-fails;
  plus F-GEN-A through F-GEN-H for the candidate-generation
  methodology.
- **Cost estimate**: very high (150-255 hours validation session,
  spanning multiple subfields: spectral theory + analytic number
  theory + adelic structures + de Branges spaces).
- **Expected verdict at validation**: **OBSTRUCTION_DOCUMENTED**
  (~80% probability) via F-HP-B activation (with F-HP-D and F-HP-E
  joint secondary activations); PASS_SKETCH (~5%); PASS_LITERATURE
  (< 1%); INCONCLUSIVE (~10%); FAIL_RELABELING (~5%). Realistic at
  axis-B-level difficulty: 110+ years of unsuccessful attempts is
  overwhelming prior against PASS.
- **Cross-BT EXT comparison**: completes the BT-544/543/542 EXT-B
  analytic-Lyapunov trio with BT-541's operator-theoretic analog;
  same meta-pattern (framework prescribed, instantiation missing),
  different family (operator-theoretic vs analytic-inequality),
  higher difficulty (axis-B 110-year prior), independent obstruction
  structure.
- **No new theorem claimed.** All cited results are pre-existing
  (Berry-Keating 1999 SIAM Rev. 41 / Plenum 1999; Connes 1999 Selecta
  Math 5; de Branges 1968 Prentice-Hall / 1986 Bull. AMS 15; Sierra-
  Townsend 2008 PRL 101 / 2011 arXiv:1012.4264; Bender-Brody-Müller
  2017 PRL 118; Reed-Simon 1972/1975 Vols I/II; Tate 1950 / 1967
  *Algebraic Number Theory*; Selberg 1956 *J. Indian Math. Soc.* 20;
  Conrey-Li 2000 IMRN; Schumayer-Hutchinson 2011 Rev. Mod. Phys. 83;
  Bombieri 2000 Clay official problem statement; Edwards 1974
  *Riemann's Zeta Function* Academic Press; Iwaniec-Kowalski 2004
  *Analytic Number Theory* AMS; Titchmarsh-Heath-Brown 1986 *The
  Theory of the Riemann Zeta Function* 2nd ed. Oxford; Riemann 1859;
  von Mangoldt 1895; Hilbert / Pólya folklore via Odlyzko 1981/1985
  correspondence; Odlyzko γ_n tables).
- **RH status**: **OPEN**. BK-Connes-xp is a candidate Hilbert-Pólya
  operator, not an RH proof; its self-adjointness and spectral
  identification are conjectural; its full validation is decadal-to-
  multi-decadal scale per the BT-541 method-gap diagnosis (110+ years
  unsuccessful prior).
- **0/7 unchanged. RH status open. No atlas/state/inventory edits.**

— end candidate-generation —
