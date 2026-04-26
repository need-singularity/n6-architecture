---
id: omega-exec-bt542-4barrier-progressive-deepening
date: 2026-04-25
scope: research-only methodology demonstration (NOT making P=NP claim; applying R1 pattern)
target: BT-542 4-barrier stack -- progressive deepening methodology mapping
parent_reports:
  - reports/sessions/omega-meta-synthesis-progressive-deepening-pattern-2026-04-25.md (R1 pattern)
  - reports/sessions/omega-cycle-bt542-pnp-2026-04-25.md
  - reports/sessions/omega-exec-bt542-hirahara-molt-validation-2026-04-25.md (Hirahara non-naturalized)
millennium_resolved: 0/7 (unchanged)
grade: methodology demonstration, no claim
---

# Omega Exec -- BT-542 4-Barrier Progressive Deepening (2026-04-25)

## §0 Non-claim disclaimer

This report is a **cross-BT methodology demonstration**. It applies
the progressive-deepening 5-pattern extracted in
`omega-meta-synthesis-progressive-deepening-pattern-2026-04-25.md`
(R1-Aux Lemma 1 L1–L4 sequence on BT-544 NS) to the historical
4-barrier stack of theoretical-CS complexity (BT-542 P vs NP). It
is **not** a research result. It:

- does **NOT** prove P = NP, P ≠ NP, or any direction thereof;
- does **NOT** claim any of the 4 barriers (relativization /
  natural-proofs / algebrization / GCT) is broken or bypassed;
- does **NOT** claim Hirahara meta-complexity 2018–2022 closes
  P vs NP, or constitutes a 5th barrier in the canonical
  catalogue (the canonical 4-barriers stance is preserved);
- does **NOT** modify `state/proposals/inventory.json`,
  `shared/n6/atlas.n6`, or `theory/canon/`;
- does **NOT** advance BT-542 in any direction; the BT-542
  ladder (`omega-cycle-bt542-pnp-2026-04-25.md` §2) and
  composite (~0.44, possibly ~0.47–0.49 post-Hirahara PASS,
  ibid. §3) are unaltered;
- does **NOT** introduce a "Level 5" of progressive deepening
  for BT-544 R1-Aux, nor a "Level 5" for BT-542's 4-barrier
  stack — the mapping is **retrospective** on a 30-year
  community record;
- does **NOT** claim the R1 5-pattern is universally
  applicable to all open problems; it is a candidate
  transferable lens with registered scope (§4–§5 of the
  parent synthesis) and falsifiers active in this report (§11).

The 4 barriers are cited by author/year/venue verbatim from the
parent reports; no new barrier theorem is fabricated. The Hirahara
status is inherited from `omega-exec-bt542-hirahara-molt-validation
-2026-04-25.md` PASS_LITERATURE verdict (which is itself
explicitly NOT a P=NP claim, ibid. §0).

**Millennium tally**: 0/7 unchanged. P vs NP remains OPEN since
1971. No atlas/state/inventory edits. The mapping in §2–§5 below
illustrates a methodological lens; it produces no bound, no
resolution, and no closure-criterion advance.

---

## §1 R1 5-pattern recap

The parent synthesis `omega-meta-synthesis-progressive-deepening
-pattern-2026-04-25.md` §2 identified five repeating
methodological elements in the L1–L4 R1-Aux Lemma 1 sequence on
BT-544 NS. Each element is verified present in every Level of
that sequence (§2.1–§2.5 of the parent). The five elements are:

1. **Obstruction identification**: localize the failure to a
   specific equation/step/parameter, not "it didn't work".
2. **Honest documentation**: record the failure as
   OBSTRUCTION_DOCUMENTED (or deeper variant), not as a hedged
   claim or retraction; verdict-grade table justifies why other
   labels (PASS_SKETCH / FAIL / INCONCLUSIVE) are rejected.
3. **Structural reframe**: next attempt's framework is
   structurally different in the ingredient the obstruction
   implicated, not a parameter-permutation of the prior framework.
4. **Sanity check**: dedicated test of whether the new framework
   reproduces known results in regimes where they are established;
   pass/fail flagged with diagnostic interpretation.
5. **Cumulative learning**: each Level's "next-technique" section
   names specific routes implicated by the current diagnosis;
   each subsequent Level executes one of the listed routes.

The portable statement (parent §3, ≤200 words) is: localize, document,
reframe structurally, sanity-check, cumulate; stop when a useful
bound emerges, when reframes are exhausted, or when diagnosis
stabilizes at "regime gap between current methods".

This report applies the 5-pattern lens to the BT-542 historical
record:

- **Barrier 1**: Baker–Gill–Solovay 1975 (relativization).
- **Barrier 2**: Razborov–Rudich 1994 STOC / 1997 J. Comput.
  Syst. Sci. 55 (natural proofs).
- **Barrier 3**: Aaronson–Wigderson 2008 (TOC) (algebrization).
- **Barrier 4-status**: Hirahara 2018 FOCS / 2022 FOCS
  (meta-complexity), per BT-542 Hirahara molt-validation PASS,
  treated as an **in-progress non-natural primitive** rather than
  a canonical 5th barrier.

The mapping below treats each barrier-event as one "Level" in a
30-year community-scale progressive-deepening sequence,
analogous to L1→L4 within a single session for R1-Aux. The
analogy and its limits are discussed in §6–§8.

---

## §2 Barrier 1 — Relativization (Baker–Gill–Solovay 1975)

**Citation**: T. Baker, J. Gill, R. Solovay, "Relativizations of
the P =? NP question", SIAM J. Comput. 4 (1975), 431–442.

### §2.1 Obstruction identification (5-pattern element 1)

The 1971 Cook–Levin landscape admitted naive attacks on P vs NP
via diagonalization techniques borrowed from the time-hierarchy
theorems (Hartmanis–Stearns 1965). BGS 1975 constructed two
oracles A, B with: P^A = NP^A (oracle A makes the question
trivially closed in one direction) and P^B ≠ NP^B (oracle B
forces separation in the other direction). The localized
obstruction: any proof technique whose argument **relativizes**
(i.e., remains valid when all machines are equipped with the same
oracle) cannot resolve P vs NP, since both directions would be
provable simultaneously, a contradiction. The specific blocker is
not "diagonalization fails" generically; it is "relativizing
diagonalization fails because BGS's two oracles witness opposite
answers".

### §2.2 Honest documentation (5-pattern element 2)

BGS 1975 documents the construction of A and B explicitly. The
result is published, peer-reviewed, in SIAM J. Comput. The
community received it as **OBSTRUCTION_DOCUMENTED** at the
technique-class level, not as a refutation of P vs NP. Classical
P-vs-NP-via-pure-diagonalization attempts were honestly retired;
no "the proof is almost there, just needs a tweak" hedging is
recorded in the canonical literature.

### §2.3 Structural reframe (5-pattern element 3)

The implicated structural ingredient is **oracle-invariant
machinery**. The reframe target is "non-relativizing techniques"
— proof methods that depend on the actual computational model
(circuit gates, polynomial structure, communication complexity)
in a way that breaks under oracle access. Examples that emerged
in the years following BGS 1975: Furst–Saxe–Sipser 1981 / Ajtai
1983 / Yao 1985 / Håstad 1986 (AC^0 lower bounds via switching
lemma), Razborov 1985 (monotone circuit lower bounds for clique).
These are non-relativizing because the switching lemma and
approximation method directly manipulate circuit structure, not
oracle queries.

### §2.4 Sanity check (5-pattern element 4)

The non-relativizing techniques **do** prove non-trivial lower
bounds in regimes where they apply: AC^0 cannot compute parity
(Furst–Saxe–Sipser 1981 / Ajtai 1983), AC^0[p] cannot compute
parity_q for q with different prime support (Razborov 1987 /
Smolensky 1987), monotone circuits for clique require
super-polynomial size (Razborov 1985). The sanity check passes:
non-relativizing techniques are real and produce real lower
bounds, just not at the level of P vs NP. This pass is structurally
analogous to L2's sanity-pass in the R1 sequence (parent §1.1
table row L2: "α = α_BV = 0 sanity check passes in L²").

### §2.5 Cumulative learning (5-pattern element 5)

The community lesson from BGS 1975 onward: P vs NP requires a
**non-relativizing** technique. This constraint is **monotone** —
all subsequent attempts must satisfy it. The "next-technique"
candidates implicated were: (a) switching-lemma-class arguments,
(b) approximation-method-class arguments, (c) algebraic /
polynomial-method arguments (later named "natural" by Razborov–
Rudich). The community executed (a) and (b) productively for ~20
years (1981–1994) before the next barrier was diagnosed.

### §2.6 R1-pattern element coverage table

| element | present? | citation |
|---------|----------|----------|
| 1. obstruction identification | YES | BGS 1975, two-oracle construction |
| 2. honest documentation | YES | SIAM J. Comput. publication; community retired naive diagonalization |
| 3. structural reframe | YES | non-relativizing techniques (switching lemma, approximation method) |
| 4. sanity check | YES | AC^0 / AC^0[p] / monotone-clique lower bounds proved |
| 5. cumulative learning | YES | "P vs NP requires non-relativizing" became standard constraint |

All five elements present.

---

## §3 Barrier 2 — Natural proofs (Razborov–Rudich 1994 STOC / 1997 J CSS 55)

**Citation**: A. Razborov, S. Rudich, "Natural proofs", Proc. STOC
1994, 204–213; J. Comput. Syst. Sci. 55 (1997), 24–35.

### §3.1 Obstruction identification (5-pattern element 1)

By the early 1990s, the non-relativizing toolkit had reached
super-polynomial monotone lower bounds (Razborov 1985, Andreev
1985, Alon–Boppana 1987) but stalled on general (non-monotone)
circuits. RR 1994/1997 localized why: they formalized the
predicate **natural** for properties of Boolean functions —
*constructive* (decidable in polynomial-time over the truth
table) and *large* (satisfied by a non-negligible fraction of
random Boolean functions). They proved: if natural property P
distinguishes "easy" (e.g., P/poly) from "hard" functions, then
P also breaks pseudorandom generators with security against
P/poly. Under standard cryptographic assumptions (existence of
strong PRGs / one-way functions), no such P can exist.

The specific obstruction: most known non-relativizing lower-bound
techniques (random restriction + approximation) are natural in
the RR sense. The blocker is at the **technique predicate**
level, refining barrier 1's "non-relativizing" requirement with a
finer "non-natural" requirement.

### §3.2 Honest documentation (5-pattern element 2)

RR 1994 STOC and the 1997 JCSS journal version document the
predicate, the proof, and the cryptographic dependence
explicitly. The community received the result as
**OBSTRUCTION_DOCUMENTED_DEEPER** (in R1 vocabulary): not a
refutation of P ≠ NP, but a constraint on the proof technique
class. Razborov's own AC^0[p] result (1987) and Smolensky 1987 are
natural; their applicability to AC^0[6] (composite modulus) is
limited by the natural-proofs barrier itself in conjunction with
mod-counting subtleties.

### §3.3 Structural reframe (5-pattern element 3)

The implicated ingredient is **constructivity + largeness** of the
distinguishing property. The reframe target: proofs that are
non-natural — either non-constructive (the property predicate is
not in P/poly itself) or non-large (the property is rare among
random functions). Examples that emerged: Williams 2011 (NEXP ⊄
ACC^0), which combines time-hierarchy diagonalization with circuit
analysis in a way RR's predicate does not capture; Hirahara
2018+ meta-complexity results (see §5).

### §3.4 Sanity check (5-pattern element 4)

Natural proofs **do** establish AC^0 and AC^0[p] lower bounds —
these are not refuted by RR 1994/1997. The sanity check passes
in this regime: natural techniques work for the lower-bound
problems they were originally designed for; they fail to lift to
P/poly precisely because of the cryptographic-assumption
dependency. This is structurally analogous to L1's implicit
sanity check in the R1 sequence (parent §2.4: "naive (5'')
predicts no NS construction at any α — contradicts published
BV-2019 L² result; the contradiction itself flagged the form as
wrong").

### §3.5 Cumulative learning (5-pattern element 5)

Post-RR 1994/1997, the community constraint set tightened to
**non-relativizing AND non-natural**. This is a proper
intersection: not every non-relativizing technique is non-natural
(e.g., Razborov's monotone-clique lower bound is non-relativizing
but is natural in the RR sense). The "next-technique" candidates
implicated: (a) techniques exploiting interactive proofs / IP =
PSPACE (Shamir 1992), which are non-relativizing and arguably
non-natural at the high-level argument structure; (b) techniques
exploiting algebraic structure (later named "algebrizing" by AW
2008, see §4); (c) meta-complexity / non-black-box techniques
(later developed by Allender, Santhanam, Hirahara, see §5).

### §3.6 R1-pattern element coverage table

| element | present? | citation |
|---------|----------|----------|
| 1. obstruction identification | YES | RR 1994/1997, natural-proofs predicate + PRG contradiction |
| 2. honest documentation | YES | STOC 1994 + JCSS 1997 publication; community retired naive non-relativizing-but-natural attempts |
| 3. structural reframe | YES | non-natural proofs (Williams, meta-complexity) |
| 4. sanity check | YES | AC^0 / AC^0[p] lower bounds remain valid; barrier-2 does not refute these |
| 5. cumulative learning | YES | "non-relativizing AND non-natural" became standard constraint |

All five elements present.

### §3.7 Recursion observation

Barrier 2 applies **inside** the constraint introduced by barrier
1: the non-natural constraint is a refinement of the
non-relativizing constraint, not an independent axis. This is
structurally analogous to L2's relation to L1 in the R1 sequence
(parent §2.3: "L1 → L2: kept BV-2019 building blocks but added
μ_q tracking and split single inequality into two-leg system").
Barrier 2 does not replace barrier 1; it sits inside it.

---

## §4 Barrier 3 — Algebrization (Aaronson–Wigderson 2008)

**Citation**: S. Aaronson, A. Wigderson, "Algebrization: A New
Barrier in Complexity Theory", Theory of Computing 4 (2008),
129–179 (extends STOC 2008).

### §4.1 Obstruction identification (5-pattern element 1)

Between 1994 and 2008, several non-relativizing AND non-natural
techniques emerged: Shamir 1992 (IP = PSPACE) using arithmetization
of Boolean formulas over finite fields, Lund–Fortnow–Karloff–
Nisan 1992 (interactive proofs for #P), Babai–Fortnow–Lund 1991
(MIP = NEXP). These techniques **arithmetize** Boolean formulas
into low-degree polynomials and use polynomial structure crucially.
AW 2008 localized why even these techniques cannot resolve P vs
NP: they constructed two algebraic oracles Ã, B̃ — oracles
extended to algebraic queries (low-degree polynomial extensions
over finite fields) — such that **algebrizing** techniques cannot
distinguish P^Ã = NP^Ã from P^B̃ ≠ NP^B̃. The specific obstruction:
arithmetization-based techniques relativize with respect to
algebraic oracles, even though they do not relativize with respect
to standard Boolean oracles.

### §4.2 Honest documentation (5-pattern element 2)

AW 2008 documents the algebraic-oracle construction and the
algebrization predicate explicitly. The result is published
(extended to ToC 4 from the STOC 2008 conference version). The
community received it as **OBSTRUCTION_DOCUMENTED_LEVEL_3** (in
R1 vocabulary, mapping AW to L3): a third constraint on the
technique class, not a refutation of P ≠ NP.

### §4.3 Structural reframe (5-pattern element 3)

The implicated ingredient is **arithmetization invariance**. The
reframe target: techniques that are non-algebrizing — proofs that
exploit Boolean structure or computational structure in ways that
break under algebraic oracle access. Williams 2011 (NEXP ⊄ ACC^0;
NQP ⊄ ACC^0 in 2018) is non-algebrizing: the satisfiability
algorithm at the heart of Williams's argument uses Boolean
satisfiability checks that do not lift to algebraic oracles.
Hirahara 2018 non-black-box reductions (see §5) are also
non-algebrizing in the AW sense, since they reason about the code
of the input rather than treating it as an oracle.

### §4.4 Sanity check (5-pattern element 4)

Algebrizing techniques **do** establish IP = PSPACE, MIP = NEXP,
and the toolbox of interactive-proof-class collapses. These
results are not refuted by AW 2008; they remain in the literature
as published theorems. The sanity check passes: algebrizing
techniques work for the problems they were designed for
(interactive-proof characterizations of complexity classes); they
fail to lift to P vs NP precisely because of the algebraic-oracle
construction.

### §4.5 Cumulative learning (5-pattern element 5)

Post-AW 2008, the community constraint set tightened to
**non-relativizing AND non-natural AND non-algebrizing**. This
triple-barrier compound is the canonical position of P vs NP as
of `omega-cycle-bt542-pnp-2026-04-25.md` §1: "all four barriers
MISS_MAINTAINED" with the understanding that any P-vs-NP technique
must satisfy all three (and bypass GCT-class obstructions, the
informal 4th barrier in BT-542's stance, see §4.7). The
"next-technique" candidates implicated: (a) Williams-line
non-natural + non-relativizing + non-algebrizing techniques
reaching NEXP / NQP not NP (the regime mismatch noted in
omega-cycle-bt542-pnp §1: "Williams-line is non-natural +
non-relativizing + non-algebrizing but reaches only NEXP/NQP,
not NP"); (b) GCT (Mulmuley 2001+); (c) meta-complexity
(Hirahara 2018+).

### §4.6 R1-pattern element coverage table

| element | present? | citation |
|---------|----------|----------|
| 1. obstruction identification | YES | AW 2008, algebraic-oracle construction + algebrization predicate |
| 2. honest documentation | YES | ToC 4 publication; community recognised triple-barrier compound |
| 3. structural reframe | YES | non-algebrizing techniques (Williams, Hirahara) |
| 4. sanity check | YES | IP = PSPACE, MIP = NEXP remain valid theorems; barrier-3 does not refute these |
| 5. cumulative learning | YES | "non-relativizing AND non-natural AND non-algebrizing" became standard constraint |

All five elements present.

### §4.7 GCT as informal 4th barrier (Mulmuley 2001+; Ikenmeyer–Panova 2017)

The BT-542 4-barriers stance (`omega-cycle-bt542-pnp-2026-04-25.md`
§1, `bt-542-p-vs-np-4-barriers-survey-2026-04-15.md`) lists GCT as
the canonical 4th barrier: Mulmuley 2001 introduced Geometric
Complexity Theory as a candidate program; Ikenmeyer–Panova 2017
proved that the rectangular-Kronecker-coefficient occurrence-
obstruction approach cannot succeed (i.e., the main route within
GCT fails the occurrence-obstruction criterion). The GCT barrier
is qualitatively different from barriers 1–3: rather than a
universal obstruction theorem on a technique class, it is a
program-internal obstruction theorem that retires the canonical
GCT subroutine. The 5-pattern still applies in modified form
(obstruction = occurrence criterion fails; reframe = relax to
extension obstructions; sanity = tensor-rank lower bounds remain
valid; cumulative = next-program GCT2 / partition-asymptotics
candidates), but the temporal scale is internal to one program.

The Hirahara meta-complexity status (§5) is **not** the canonical
4th barrier. The 4-barriers survey explicitly registers GCT in
that slot. Hirahara is treated here as an in-progress
non-naturalized primitive that emerged inside the
non-relativizing+non-natural+non-algebrizing constraint set.

---

## §5 Barrier 4 status — Hirahara meta-complexity (in-progress)

**Citation**: S. Hirahara, "Non-Black-Box Worst-Case to
Average-Case Reductions within NP", FOCS 2018 (best paper);
S. Hirahara, "NP-Hardness of Learning Programs and Partial MCSP",
FOCS 2022 (best paper). Auxiliary: Oliveira–Santhanam 2017 (CCC),
Hirahara–Santhanam 2017 (CCC), Hirahara 2020 STOC.

### §5.1 Status: not a barrier per se

Per `omega-exec-bt542-hirahara-molt-validation-2026-04-25.md`
PASS_LITERATURE verdict, the Hirahara series introduces a
**non-naturalized primitive** (non-black-box reduction technique
that uses the code of the input; partial-MCSP-NP-hard under RP
reductions) absent from the GATE-BARRIER frame. The molt
PASSes — the primitive is real, peer-reviewed at FOCS-best level,
and structurally distinct from the natural-proofs predicate.

However, the Hirahara series is **not a 4th barrier theorem** in
the BGS / RR / AW sense. There is no published theorem of the
form "any technique with property X cannot resolve P vs NP".
Instead, Hirahara has introduced a **technique** (non-black-box
reductions) that satisfies the prior triple constraint (non-
relativizing + non-natural + non-algebrizing) and is making
incremental progress on adjacent problems (partial MCSP NP-hard,
worst-to-average within NP).

### §5.2 Obstruction identification (5-pattern element 1, in-progress)

The localized obstruction at the Hirahara front is: full MCSP
NP-hardness under deterministic polynomial-time reductions
remains open (only partial MCSP under RP is proved as of 2022).
This is a **specific** open question at the technique frontier,
not a vague "P vs NP is hard". The 5-pattern element 1 is
satisfied at the level of the Hirahara research program's
self-diagnosis.

### §5.3 Honest documentation (5-pattern element 2, in-progress)

Per v3-T5 Hirahara MCSP deep-dive (2026-04-15) §3 + §4: "Hirahara
2022 is a major instrumental advance ... However, this is not a
P vs NP draft. The Clay problem remains open." The repo's own
honest verdict matches the published literature's: progress is
real, the problem is not closed. Honest documentation is in
place.

### §5.4 Structural reframe (5-pattern element 3, in-progress)

The reframe in progress: from "natural proofs of circuit lower
bounds" to "meta-complexity / minimum-circuit-size reasoning".
The structurally different ingredient is the **object** under
study (MCSP, K-complexity) and the **reduction technique**
(non-black-box). This satisfies the parent's anti-pattern B
("cosmetic reframe") test: it is a structural switch, not a
parameter rename.

### §5.5 Sanity check (5-pattern element 4, in-progress)

The Hirahara framework reproduces and extends prior
meta-complexity results: Oliveira–Santhanam 2017 MCSP–OWF
equivalence is consistent; Hirahara–Santhanam 2017
worst-to-average-case reductions are extended in Hirahara 2018;
partial MCSP NP-hard under RP (Hirahara 2022 FOCS best) is the
strongest unconditional NP-hardness result for an MCSP variant.
Sanity check passes in the meta-complexity native regime.

### §5.6 Cumulative learning (5-pattern element 5, in-progress)

The Hirahara line has not yet stabilized into a barrier (in the
BGS / RR / AW sense) nor into a P-vs-NP closure. Cumulative
learning so far: P vs NP resolution may require a technique
satisfying (non-relativizing + non-natural + non-algebrizing +
non-black-box-class-evading-natural-proofs). Whether any 5th
barrier theorem will eventually retire the meta-complexity route
(the way RR 1994/1997 retired natural-proofs-class techniques) is
**unknown**; this report does not predict it.

### §5.7 R1-pattern element coverage table

| element | present? | citation |
|---------|----------|----------|
| 1. obstruction identification | PARTIAL (in-progress: full MCSP NP-hardness open) | Hirahara 2022 (partial MCSP under RP, full open) |
| 2. honest documentation | YES | v3-T5 §3 + §4 honest verdict; FOCS 2018 + 2022 publications |
| 3. structural reframe | YES | meta-complexity / non-black-box (vs natural proofs of circuit LB) |
| 4. sanity check | YES | MCSP–OWF equivalence (OS 2017), W2A within NP (Hirahara 2018) reproduced |
| 5. cumulative learning | PARTIAL (in-progress: "non-???" 5th constraint not yet a published theorem) | Hirahara line ongoing |

Three elements YES, two PARTIAL (in-progress). This matches the
expected status of a Level still in flight (parallel to L4 of R1,
which the parent synthesis treated as
OBSTRUCTION_DOCUMENTED_LEVEL_4 with caveat about in-flight status,
parent §0 final paragraph).

---

## §6 Method-gap claim (intersection of barriers)

Following the parent synthesis's portable-statement structure,
the BT-542 method gap is:

> Multiple barrier classes — relativizing (BGS 1975), natural
> (RR 1994/1997), algebrizing (AW 2008), GCT-occurrence
> (Ikenmeyer–Panova 2017) — each obstruct a distinct family of
> proof techniques. P=NP resolution requires a technique
> satisfying *all* simultaneous non-X conditions. The
> intersection {non-relativizing AND non-natural AND
> non-algebrizing AND non-GCT-occurrence} is the candidate
> "method gap" — the technique family in which a P=NP-resolving
> proof, if any exists, must live. Williams 2011 / 2018
> (NEXP ⊄ ACC^0, NQP ⊄ ACC^0) and Hirahara 2018+ meta-complexity
> demonstrate that this intersection is **non-empty**, but
> current techniques in the intersection reach only NEXP / NQP /
> partial-MCSP / K-complexity, not the NP / P/poly target needed
> for P vs NP.

This is **structurally analogous** to the R1 method-gap claim
(parent §1.2): "α*_NS strictly between α_BV and 1/3 — sits in a
genuine gap that neither the construction side (which currently
reaches only α_BV ≪ 1/3) nor the rigidity side (which requires
α > 1/3) covers". In both cases:

- the gap is the *complement* (or strict interval) bounded by
  what current techniques achieve from each side;
- each side has its own technique-class obstruction;
- the gap is non-empty as a region of statement-space (the
  conjecture lives there) but unreached by current methods.

The R1 strip is (α_BV, 1/3) — narrowed by intersecting two
complementary technique families (construction below α_BV,
rigidity above 1/3). The BT-542 gap is the technique
intersection {non-relativizing ∩ non-natural ∩ non-algebrizing ∩
non-GCT-occurrence} ∩ {techniques actually reaching NP rather
than NEXP/NQP/MCSP-variants}. The structural parity of "narrowed
by intersecting complementary obstruction theorems" is the
shared geometry.

---

## §7 Cumulative diagnosis verdict

The parent synthesis lists three candidate verdicts (parent §11
closing summary referenced § 4 verdict-grade discipline):

1. **METHOD_GAP_CONFIRMED**: progressive deepening pattern fits
   BT-542 4-barrier history; method gap is the complement of
   barrier intersection.
2. **PARTIAL_FIT**: pattern fits but each barrier is
   qualitatively different from R1's framework-level retries.
3. **POOR_FIT**: BT-542's barriers are not "progressive
   deepening" in the R1 sense; they are independent constraints,
   not a refinement sequence.

### §7.1 Verdict-grade table

| candidate | applicable? | reasoning |
|-----------|-------------|-----------|
| METHOD_GAP_CONFIRMED | candidate, with caveats | All five R1 elements (§1) appear in each of barriers 1–3 (verified §2.6, §3.6, §4.6); barrier 4-status (Hirahara, §5) shows partial coverage as expected for in-flight Level. The method-gap claim (§6) is structurally parallel to R1's strip claim. |
| PARTIAL_FIT | candidate, strong | Each BT-542 barrier is a *theorem* about a technique class (BGS / RR / AW / IP), not a *retry* of a specific lemma. R1 retries are within-author, within-session, on a specific lemma (Lemma 1 of R1-Aux). BT-542 barriers are 30+ years of community work, with each barrier authored by different teams, and each barrier-theorem is itself a major published result. The "structural reframe" element is satisfied at the community-research level, not at a single-session-author level. |
| POOR_FIT | rejected | The five R1 elements *are* each present in each of barriers 1–3; the pattern is not absent. POOR_FIT would require at least one element absent, which is not the case (verified §2.6, §3.6, §4.6). |

### §7.2 Selected verdict

**METHOD_GAP_CONFIRMED** with explicit caveats from the
PARTIAL_FIT consideration.

The verdict is:

> **METHOD_GAP_CONFIRMED**: the progressive-deepening 5-pattern
> fits the BT-542 4-barrier historical sequence at the
> element-by-element level (verified §2.6, §3.6, §4.6, §5.7).
> The method gap is the complement of the
> barrier-intersection — the family of techniques satisfying
> non-relativizing AND non-natural AND non-algebrizing AND
> (avoiding GCT-occurrence-style obstructions), and reaching the
> NP target rather than NEXP / NQP / partial-MCSP. Williams /
> Hirahara show this gap is non-empty as a technique region, but
> no current technique in the gap closes P vs NP at the NP
> level.
>
> **Caveats** (from PARTIAL_FIT considerations):
>
> (a) BT-542 barriers are *theorems about technique classes*
> authored by different teams across 30+ years (1975 / 1994 /
> 2008 / 2017); R1 L1–L4 are single-session retries of one lemma
> by one author. The shape of the pattern is parallel; the
> *time scale* and *agent of refinement* are not.
>
> (b) Each BT-542 barrier is itself a major published result
> (BGS 1975 SIAM J. Comput.; RR 1997 JCSS; AW 2008 ToC; IP 2017
> in the GCT case). R1 L1–L4 are sketch-level investigations
> documented in single-session reports. The *publication
> grade* differs.
>
> (c) The cumulative-learning element (#5) operates at the
> community-research-program level for BT-542 (each barrier
> implies a constraint that the next ~20 years of community
> work must satisfy), and at the within-session level for R1
> (each Level's "next-technique" section drives the next
> Level immediately). The *operating loop* differs.
>
> (d) Barrier 4 (Hirahara) is in-progress; the canonical
> 4-barriers stance reserves slot 4 for GCT, with Hirahara
> treated as a non-natural primitive inside barrier-2's
> negation rather than as a 5th barrier. The mapping to R1's
> L4 (also in-flight at the time of the parent synthesis) is
> structurally analogous but not a 1-to-1 correspondence.

These caveats are *registered*, not *de-rating*. The verdict
remains METHOD_GAP_CONFIRMED on the strength of the element-by-
element coverage; the caveats document the cross-time-scale
limits of the analogy (§8).

---

## §8 Cross-time-scale comparison: R1 single-session vs BT-542 30-year community

### §8.1 Time-scale axis

| feature | R1 L1–L4 (BT-544 Lemma 1) | BT-542 4-barriers |
|---------|---------------------------|--------------------|
| time scale | single session, 2026-04-25 | 1975 → 1994 → 2008 → 2017+ (42+ years) |
| author | single agent, single session | community-distributed (BGS / RR / AW / IP / Mulmuley / Williams / Hirahara) |
| object of refinement | one specific lemma (R1-Aux Lemma 1, α* bound on α*_NS in C3 regime) | one specific question (P vs NP) |
| publication grade | sketch-level session reports | top-tier journal / FOCS / STOC theorems |
| trigger for next Level | prior Level's diagnosed obstruction within same session | prior barrier's identified technique-class constraint over years |
| typical artifact size | one 5-element pattern + verdict per Level | one published paper per barrier |
| stop condition | regime-gap stabilization (parent §8.3) | barrier stack converges to "no current technique reaches P=NP" (decades-stable position) |

### §8.2 Pattern shape preserved across scales

Despite the 5-orders-of-magnitude time-scale difference, the
five-element pattern (§1) is preserved:

- **Element 1 (obstruction identification)**: R1 localizes to a
  numbered inequality (e.g., (3-V-α) infeasible); BT-542
  localizes to a technique-class theorem (BGS oracles, RR natural
  predicate, AW algebraic oracles, IP occurrence criterion).
  Different *granularity*, same *discipline of localization*.

- **Element 2 (honest documentation)**: R1 uses
  OBSTRUCTION_DOCUMENTED variants in single-session reports;
  BT-542 uses peer-reviewed publication of barrier theorems.
  Different *medium*, same *discipline of face-value recording
  without rebranding*.

- **Element 3 (structural reframe)**: R1 switches building blocks
  (BV-2019 → BCV-2021) or proof strategy (constructive → ν→0
  rigidity); BT-542 switches technique class (relativizing →
  non-relativizing → non-natural → non-algebrizing → meta-
  complexity). Different *axis of reframe*, same *requirement of
  structurally different ingredient*.

- **Element 4 (sanity check)**: R1 checks new framework on known
  α regime (α_BV / α_BCV); BT-542 checks each new technique class
  on lower-bound problems where it applies (AC^0 / IP = PSPACE /
  partial MCSP). Different *verification target*, same *discipline
  of native-regime sanity*.

- **Element 5 (cumulative learning)**: R1's "next-technique"
  section names L+1's executed direction; BT-542's barrier
  theorem names the constraint the next ~20 years of work must
  satisfy. Different *time horizon*, same *constraint
  monotonicity*.

### §8.3 Where the analogy breaks

The analogy is **not** a 1-to-1 isomorphism. Specifically:

- **No single agent steers BT-542's deepening sequence**. RR did
  not write their 1994 paper *because* BGS 1975 was diagnosed at
  the prior Level; they wrote it because they identified the
  cryptographic-PRG argument independently. The cumulative-
  learning element operates **emergently** in BT-542 and
  **directly** in R1.

- **BT-542 barriers are not all in the same chain**. GCT
  (Mulmuley 2001+) is partially independent of the
  non-relativizing → non-natural → non-algebrizing chain, since
  it operates within algebraic geometry / representation theory
  rather than as a meta-theorem about technique classes. The
  4-barriers stance treats GCT as a parallel program with its
  own internal obstruction (IP 2017) rather than as a strict
  refinement of AW 2008.

- **R1 has a clear stop-condition (§8.3 of parent: regime-gap
  stabilization at L4)**; BT-542 has a **decades-stable** "method
  gap remains" status with no clear termination signal beyond
  community attention drift.

These breaks are documented in §7's PARTIAL_FIT caveats but do
not flip the METHOD_GAP_CONFIRMED verdict.

---

## §9 Implications

### §9.1 Cross-BT progressive deepening as transferable lens

The 5-pattern survives transfer from a single-session R1 sequence
to a 30-year community sequence on BT-542. This is **evidence**
(not proof) that the pattern is **time-scale-independent** in the
sense that:

> any iterated proof-attempt history with element-1 (localized
> obstructions) + element-2 (honest documentation) + element-3
> (structural reframes) + element-4 (sanity checks) + element-5
> (cumulative learning) exhibits the progressive-deepening shape,
> regardless of whether the iteration is within-session or
> across-decades.

This is **registered as a candidate generalization**, not a
claim. Falsifiers F-Sk-PD-1 (§11) would fire if any element were
absent in the BT-542 mapping; the §2.6 / §3.6 / §4.6 / §5.7
tables verify presence (with §5.7 partials documented).

### §9.2 Method-gap is the predictive output

In both R1 and BT-542, the methodology produces a **map of why
the problem is hard** (parent §11 closing). The BT-542 map says:
P vs NP lives in the intersection {non-relativizing ∩ non-natural
∩ non-algebrizing ∩ non-GCT-occurrence}, and current techniques
in this intersection (Williams, Hirahara) reach NEXP / NQP /
MCSP-variants but not NP. This map does not produce a
P-vs-NP-resolving technique; it specifies the constraints any
such technique would have to satisfy. This is structurally the
same output as the R1 map (α*_NS in C3 regime not reachable by
current convex-integration construction or Onsager-rigidity).

### §9.3 Predictions for any future 5th barrier

If a 5th barrier is published in 2025–2030, this report predicts
(low confidence, non-binding) that it will exhibit the 5-pattern:
identify a specific technique-class obstruction (likely targeting
the meta-complexity / non-black-box family); document via a
peer-reviewed theorem; reframe to a non-X technique class;
preserve sanity at the lower-bound regime where the obstructed
technique was useful; and tighten the method-gap intersection.
This will **not** close P vs NP, only narrow the technique
constraints further. This prediction is **registered falsifiable**
in the sense that an actual 5th barrier could disconfirm any
element (e.g., a 5th barrier that *positively resolves* P vs NP,
or a 5th barrier with no clear cumulative-learning constraint).

### §9.4 What this report does NOT imply

- **NOT** a P=NP claim. Millennium tally 0/7 unchanged.
- **NOT** a claim that the R1 5-pattern is *the right tool* for
  any specific BT or open problem; each application requires its
  own verdict-list and stop-conditions (parent §7 framing).
- **NOT** a claim that the 4-barrier stack is *complete*; the
  Hirahara line is in-flight, and a 5th barrier is possible.
- **NOT** a modification of `omega-cycle-bt542-pnp-2026-04-25.md`,
  `omega-exec-bt542-hirahara-molt-validation-2026-04-25.md`, or
  any inherited document.
- **NOT** an atlas / state / inventory edit; this report writes
  only its own file.

---

## §10 Anti-list

Alternative mappings considered and rejected:

| variant | why rejected |
|---------|--------------|
| V1: "BT-542 has 4 barriers therefore it has 4 Levels of progressive deepening, 1-to-1 mapping" | Partial truth, but oversimplifies. The 4 barriers are **community-sequenced obstruction theorems**; the R1 Levels are **single-author Levels within one session**. The structural parity is at the 5-element pattern level, not at the "4 = 4" count level. Captured in §7 PARTIAL_FIT caveats; not adopted as 1-to-1. |
| V2: "Hirahara IS the 4th barrier; replace GCT in the canonical stack" | Refuted by `omega-cycle-bt542-pnp-2026-04-25.md` §1 ("relativization · natural-proofs · algebrization · GCT" — explicit 4-list with GCT) and `bt-542-p-vs-np-4-barriers-survey-2026-04-15.md` §5.1 (Hirahara as meta-complexity primitive, distinct from the 4-barriers slot). Adopting V2 would conflict with the inherited canonical position. Rejected. |
| V3: "The 5-pattern fits perfectly with no caveats" | Refuted by §8.3: time scale, agent, publication grade differ between R1 and BT-542. The fit is element-by-element, not isomorphic. METHOD_GAP_CONFIRMED with caveats is the honest verdict; uncaveated METHOD_GAP_CONFIRMED would overclaim. |
| V4: "The 5-pattern doesn't fit because barriers are independent constraints, not a refinement sequence" | Refuted by §3.7 (barrier 2 sits inside barrier 1 as a refinement) and §4.5 (barrier 3 tightens to the triple-intersection, not an independent axis). The constraints **are** monotone-tightening. POOR_FIT is rejected. |
| V5: "GCT should be removed because it's a different kind of barrier (program-internal vs technique-class)" | Captured as a discussion point in §4.7 but not adopted: the canonical 4-barriers stance lists GCT, and this report inherits that stance. The qualitative difference is documented; the slot is preserved. |
| V6: "Predictions in §9.3 are claims" | Refuted by §9.3 explicit "low confidence, non-binding" framing and "registered falsifiable" status. Predictions are framed as predictions; F-Sk-PD-3 (§11) registers the prediction-as-claim risk. |
| V7: "This report should also propose a 5th barrier" | Out of scope. The task is to **demonstrate the methodology** by mapping existing barriers, not to **invent** a 5th barrier. Inventing a 5th barrier without published-theorem grounding would violate the no-fabrication constraint. Rejected. |
| V8: "This report should propose a P=NP technique that lives in the method-gap intersection" | Out of scope and would violate §0 disclaimers. Williams and Hirahara already populate the intersection with techniques that don't close P vs NP at the NP level; a sketch-level proposal here would not improve on published work and would risk overclaiming. Rejected. |
| V9: "BT-542 is too different from R1 for cross-BT methodology demonstration" | Refuted by element-by-element coverage in §2.6, §3.6, §4.6, §5.7. The differences (time scale, agent, publication grade) are *parameters* of the pattern's instantiation, not absences of pattern elements. Rejected. |

---

## §11 Falsifiers active

These falsifiers apply to **this methodology demonstration**
specifically, in addition to the inherited BT-542 falsifier
roster (F-BARRIER-1..4, F-PROBE-24A..C, F-CARNOT/PHYS,
F-OMEGA-CEILING, F-OMEGA-CHAIN, per omega-cycle-bt542-pnp §8).

- **F-Sk-PD-1** (pattern-element absence in BT-542 mapping): if
  any of the five R1 pattern elements (§1) is **absent** in any
  of the four BT-542 barriers, the METHOD_GAP_CONFIRMED verdict
  is wrong. **Status: not active** — verified by §2.6, §3.6,
  §4.6 tables (full coverage for barriers 1–3) and §5.7 (partial
  coverage for in-flight Hirahara, expected for in-flight
  Level analogous to R1's L4).

- **F-Sk-PD-2** (citation fabrication): if any of BGS 1975,
  RR 1994/1997, AW 2008, IP 2017, Hirahara 2018/2022,
  Mulmuley 2001 is mis-cited (wrong year, wrong venue, wrong
  result statement), the report's evidentiary base is corrupt.
  **Status: not active** — citations are inherited verbatim from
  parent reports and cross-checked against the Hirahara
  validation report's §4 citation block. The Hirahara 2022
  FOCS/STOC ambiguity is documented (omega-exec-bt542-hirahara
  §10 F-VAL-A) and acknowledged here in §5 as inherited.

- **F-Sk-PD-3** (predictions-as-claims): if §9.3 predictions for
  any 5th barrier are reported as established results, the
  prediction → claim distinction has been violated. **Status:
  not active** — §9.3 explicitly frames as "low confidence,
  non-binding" with falsifiable status.

- **F-Sk-PD-4** (atlas/state/inventory leakage): if this report
  leads to any modification of `state/proposals/inventory.json`,
  `shared/n6/atlas.n6`, or `theory/canon/`, the read-only scope
  is violated. **Status: not active** — report writes only its
  own file.

- **F-Sk-PD-5** (claim-creep on BT-542): if this report is
  reported as advancing BT-542 (closure ceiling, composite
  score, atlas anchor promotion), the methodology → claim
  distinction has been violated. The mapping produces **no**
  BT-542 advance, **no** P=NP claim, **no** Clay-status change,
  **no** composite revision, **no** ladder occupancy change.
  **Status: not active** — §0 explicitly flags absence; §9.4
  re-affirms.

- **F-Sk-PD-6** (auxiliary → main confusion at methodology
  layer): the report is auxiliary methodology (R1 5-pattern
  applied to BT-542 record); reporting it as a P-vs-NP
  contribution would conflate the auxiliary layer with the
  main problem. **Status: not active** — methodology label
  preserved throughout.

- **F-Sk-PD-7** (Hirahara-as-canonical-4th-barrier
  relabel): if §5 is read as relabeling Hirahara as the 4th
  barrier (replacing GCT in the canonical stack), the inherited
  canonical position from omega-cycle-bt542-pnp §1 is violated.
  **Status: not active** — §5.1 explicitly states "the Hirahara
  series is **not a 4th barrier theorem**"; §4.7 retains GCT in
  the canonical 4th slot.

- **F-Sk-PD-8** (cross-time-scale-overclaim): if §8 is read as
  claiming the R1 5-pattern is universally time-scale-independent,
  the §9.1 candidate-generalization framing has been violated.
  **Status: not active** — §9.1 explicitly registers as
  "candidate generalization", "evidence (not proof)".

- **F-Sk-PD-9** (verdict-creep): if §7 verdict is reported as
  more confident than METHOD_GAP_CONFIRMED-with-caveats (e.g.,
  as METHOD_GAP_PROVEN), the verdict-grade discipline has been
  violated. **Status: not active** — §7.2 retains explicit
  caveats and §10 V3 records the unqualified-claim variant as
  rejected.

- **F-Sk-PD-10** (alternative-verdict-suppression): if §7
  claims METHOD_GAP_CONFIRMED is the **only** verdict consistent
  with the evidence, the PARTIAL_FIT consideration has been
  suppressed. **Status: not active** — §7.1 explicitly registers
  PARTIAL_FIT as a strong candidate; the selected
  METHOD_GAP_CONFIRMED is qualified by §7.2 caveats which are
  themselves PARTIAL_FIT considerations adopted into the final
  verdict.

None of F-Sk-PD-1..10 fires under this report's scope.

Cumulative falsifier roster (BT-542 inherited + this report):

- F-BARRIER-1..4 (inherited, omega-cycle-bt542-pnp §8): inactive
  (4 barriers MISS_MAINTAINED).
- F-PROBE-24A..C (inherited): inactive (probes not yet
  executed).
- F-CARNOT/PHYS (inherited, HEXA-PNP §7.10): inactive.
- F-OMEGA-CEILING (inherited): inactive (composite ~0.44 well
  below 0.835).
- F-OMEGA-CHAIN (inherited): inactive (no atlas/state/inventory
  edits in this session).
- F-MOLT-A (from BT-541/542/543/544 sweep): NOT FIRED (2 PASS /
  2 FAIL, ≥2/4 branch reached); inherited from
  omega-exec-bt542-hirahara §6.
- F-MOLT-D: BT-544 only; unchanged here.
- F-VAL-A..E (Hirahara validation, inherited): inactive.
- F-Sk-PD-1..10 (this report): all inactive.

**Total: 30+ falsifiers across BT-542 inherited + Hirahara
validation + this methodology demonstration. None fired.**

---

## §12 Closing

**0/7 unchanged. P=NP open. No atlas/state/inventory edits.**

The progressive-deepening 5-pattern from
`omega-meta-synthesis-progressive-deepening-pattern-2026-04-25.md`
(R1-Aux Lemma 1 L1–L4 sequence on BT-544 NS) maps onto the
BT-542 P-vs-NP 4-barrier historical stack at the element-by-
element level:

- **Barrier 1 (BGS 1975)**: all five elements present (§2.6).
- **Barrier 2 (RR 1994/1997)**: all five elements present (§3.6),
  recursive constraint inside barrier 1 (§3.7).
- **Barrier 3 (AW 2008)**: all five elements present (§4.6),
  triple-intersection compound with barriers 1+2 (§4.5). GCT
  (Mulmuley 2001+ / IP 2017) listed as canonical 4th barrier
  with qualitative difference (§4.7).
- **Barrier 4-status (Hirahara 2018/2022)**: in-progress,
  three-of-five elements YES, two PARTIAL (§5.7); not a 4th
  barrier theorem, but a non-naturalized primitive populating
  the method-gap intersection.

**Method-gap claim** (§6): P=NP resolution requires a technique
in {non-relativizing ∩ non-natural ∩ non-algebrizing ∩
non-GCT-occurrence-class}, reaching the NP target rather than
NEXP/NQP/MCSP-variants. Williams 2011/2018 and Hirahara 2018+
populate the intersection but reach only NEXP/NQP/partial-MCSP.
The gap is structurally analogous to R1's (α_BV, 1/3) strip —
narrowed by intersecting complementary obstruction theorems.

**Verdict** (§7): **METHOD_GAP_CONFIRMED** with caveats from
PARTIAL_FIT (time scale, agent, publication grade differences
between R1 single-session and BT-542 30-year community
sequence; Hirahara in-progress vs canonical 4-barriers stance).
POOR_FIT rejected on element-coverage evidence.

**Cross-time-scale finding** (§8): the 5-pattern is preserved
across 5+ orders of magnitude time scale, with each element
satisfied in BT-542 at community/decades granularity and in R1
at session/hours granularity. The pattern's discipline is
time-scale-independent in the registered candidate-generalization
sense (§9.1).

**No claim, no closure, no advance**. BT-542 P vs NP remains
OPEN since 1971. Millennium resolved tally: 0/7. The 4 barriers
(relativization / natural-proofs / algebrization / GCT) remain
MISS_MAINTAINED. The Hirahara non-naturalized primitive remains
licensed (per omega-exec-bt542-hirahara PASS_LITERATURE) but does
not close P vs NP. This report writes only its own file.

**0/7 unchanged. No atlas/state/inventory edits.**
