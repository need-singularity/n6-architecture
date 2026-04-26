---
id: omega-meta-audit-l9-generation-pipeline
date: 2026-04-25
scope: research-only meta-audit (NOT modifying catalogue; diagnosing generation mechanism)
target: L9 catalogue arithmetic-family bias -- generation-pipeline mechanism diagnostic
parent_reports:
  - reports/sessions/omega-probe-l9-molt-trigger-2026-04-25.md
  - reports/sessions/omega-meta-audit-l9-catalogue-design-2026-04-25.md (CATALOGUE_BIAS)
  - reports/sessions/omega-exec-bt547-poincare-L9-retro-2026-04-25.md (generation 0/3)
  - reports/sessions/dfs-24-bsd-direction-2026-04-24.md
  - reports/sessions/dfs-24-hodge-direction-2026-04-24.md
  - reports/sessions/dfs-24-ns-direction-2026-04-24.md
  - reports/sessions/dfs-24-pnp-direction-2026-04-24.md
  - reports/sessions/dfs-24-riemann-direction-2026-04-24.md
  - reports/sessions/dfs-24-ym-direction-2026-04-24.md
millennium_resolved: 0/7 (unchanged)
grade: meta-audit, no claim
---

# Omega Meta-Audit -- L9 Generation Pipeline Mechanism (2026-04-25)

## §0 Non-claim disclaimer

This file is a **meta-audit**, not a proof attempt, not an atlas
promotion, not an inventory edit, not an L9 catalogue modification.
It diagnoses the mechanism by which the L9 molt-trigger catalogue
(`omega-probe-l9-molt-trigger-2026-04-25.md`) generated 4 rank-1
candidates that the BT-547 Poincaré retrospective
(`omega-exec-bt547-poincare-L9-retro-2026-04-25.md`) found 0/3 fit
on the **generation axis** (cannot generate Perelman-class molts:
variational re-interpretation, analytic-Lyapunov construction,
procedure-class with parameter bounds). The parent CATALOGUE_BIAS
verdict identified WHICH family was over-represented; this audit
asks WHY the pipeline drew exclusively from that family.

**Hard constraints honoured**:
- No edits to `omega-probe-l9-molt-trigger-2026-04-25.md` or any of
  the dfs-24 direction probes.
- No edits to `shared/n6/atlas.n6`, `state/proposals/inventory.json`,
  `theory/canon/`, or any per-BT report.
- No fabricated references; every quote is from a real file.
- Verdict drawn from the five-element fixed set
  {H1_DOMINANT, H2_DOMINANT, H3_DOMINANT, H4_DOMINANT, MULTIPLE} only.
- Millennium tally remains **0/7 unchanged**.

---

## §1 Generation pipeline reconstruction

For each of the 4 rank-1 candidates in the L9 catalogue
(omega-probe-l9-molt-trigger §3.1-§3.4), reconstruct the generation
flow: (i) source document, (ii) generation step, (iii) family
classification per CATALOGUE_BIAS.

### §1.1 BT-541 Lead-B (SLE_6 × GUE coupling)

- **Source document**: `dfs-24-riemann-direction-2026-04-24.md`
  §"Lead-B SLE_6 locality × Montgomery pair-correlation independence
  probe" (L55-L82).
- **Generation step**: literature import. dfs-24-riemann L57 cites
  "DFS taxonomy §6.3" in which "SLE_6 locality and RH GUE statistics
  (Dyson β=2=φ) are recorded as **independent Bernoulli candidates**".
  The probe specifies the KS-test discriminator (L66-L72): "measure
  KS distance between the spacing distribution S(x) derived from 2D
  critical percolation boundary crossing probability (Cardy's formula,
  κ=6 specific) and the normalized gap distribution of the Odlyzko
  10^13 zero DB". The L9 catalogue (omega-probe-l9 §3.1 row 1)
  inherits this candidate verbatim with the falsifier "F-541-A:
  Lead-B KS p>=0.01 -> independence falsified".
- **Generation transform**: dfs-24 P-stage (literature-import probe)
  → L9 rank-1 candidate. The transform adds the framing "introduces
  a coupling primitive (boundary-crossing kappa=6) that did not
  exist in the peripheral-moment frame" (omega-probe-l9 §4.1) but
  does not change the underlying mathematical object.
- **Family**: **arithmetic / coupling-statistics identity**
  (per CATALOGUE_BIAS §2.4: "a coupling-statistics identity (a
  KS-test independence rejection) ... tests a numerical/distributional
  identity predicted by an n=6-compatible structure (κ=6 SLE +
  GUE Dyson β=2=φ)"). Anchored on n=6 via κ=6 + β=2=φ.

### §1.2 BT-542 Hirahara (MCSP meta-complexity)

- **Source document**: not from `dfs-24-pnp-direction-2026-04-24.md`
  (which only specifies PROBE-24A/B/C = Schaefer / Bulatov-Zhuk /
  Sylvester lines). Per CATALOGUE_BIAS §2.3, the candidate was
  drawn from `BT-1392 + v3-T5 Hirahara MCSP deep-dive 2026-04-15`
  via the BT-542 §6 Tension 3 cross-reference.
- **Generation step**: structural analogy / cross-BT import. The L9
  catalogue (omega-probe-l9 §3.2 row 1) cites `BT-1392 + v3-T5
  Hirahara MCSP deep-dive 2026-04-15` directly. The dfs-24-pnp
  probes (PROBE-24A/B/C) are explicitly **arithmetic-family**:
  PROBE-24A targets "Out(S_6) × Schaefer 6 S_6-orbit match"
  (dfs-24-pnp L44-55), PROBE-24C targets "n=6 reappearance at
  Schaefer 6 ↔ Bulatov-Zhuk 2017 dichotomy boundary" (dfs-24-pnp
  L67-78). The L9 catalogue **bypassed dfs-24-pnp** for rank 1 and
  selected an externally-sourced structural candidate.
- **Generation transform**: external-corpus import (v3-T5 deep-dive)
  + author-chosen discriminator ("non-natural reformulation exists").
- **Family**: **structural / meta-complexity**
  (per CATALOGUE_BIAS §2.3 "structurally different from Q1/P3 -- it
  is a barrier-bypass meta-complexity reformulation, not an n=6
  arithmetic identity. **Out of the bias family** for Q1/P3").
  This is the **only structurally diverse** candidate in the rank-1
  catalogue.

### §1.3 BT-543 P3 (A4-ratio-only frame)

- **Source document**: `dfs-24-ym-direction-2026-04-24.md` §"P3.
  A4 unit correction" (L56-L67).
- **Generation step**: literature import + arithmetic transform.
  dfs-24-ym P3 specifies verbatim (L60-L62): "do m_0++/√σ_s and the
  BCS ratio 2Δ/k_BT_c coexist in the interval [σ/τ − 1/φ, σ/τ +
  1/φ] = [2.5, 3.5]?" with measurement on FLAG 2024 + BMW 2012 +
  Morningstar-Peardon 1999. The interval bounds [2.5, 3.5] are
  derived by **arithmetic combination of n=6 invariants** (σ=12,
  τ=4, φ=2, σ/τ=3, 1/φ=0.5). The L9 catalogue (omega-probe-l9 §3.3
  row 1) inherits the candidate with falsifier "F-NP1-A4rev: ≥2
  outliers retire A4".
- **Generation transform**: dfs-24-ym P3 → L9 rank-1 candidate.
  The L9 framing adds "introduces a new dimensionless primitive
  that survives 3 independent lattice sources" (omega-probe-l9
  §4.3) but the test is identical.
- **Family**: **arithmetic / lattice empirical-interval identity**
  (per CATALOGUE_BIAS §2.2: "a lattice empirical-interval identity
  (a ratio falls inside a number-theoretic-derived interval). Same
  family as Q1: 'n=6 arithmetic produces a closed-form numerical
  invariant'"). Interval bounds derived from σ/τ ± 1/φ.

### §1.4 BT-544 Q1 (KdV 6-soliton Gram lattice)

- **Source document**: `dfs-24-ns-direction-2026-04-24.md` §"Probe
  P1 — KdV N=6 soliton phase-shift lattice" (L38-L50).
- **Generation step**: literature import + arithmetic transform.
  dfs-24-ns P1 specifies verbatim (L40-L48): "Is the n-soliton
  pairwise phase-shift set {Δ_{ij} : 1≤i<j≤6} a rank-(n/φ) lattice
  with Gram determinant divisible by σ=12 ...". The pre-registered
  falsifier F_P1 (L48): "rank ≠ 3 or det(G)·σ^{-1} ∉ ℤ for both κ
  families → the 'C(6,2)=15=sopfr·(n/φ) ↔ Serrin (σ,n,τ) grid'
  coincidence drops to post-hoc". The L9 catalogue (omega-probe-l9
  §3.4 row 1) inherits with falsifier "F-544-A".
- **Generation transform**: dfs-24-ns P1 → L9 rank-1 candidate
  with framing "introduces a Gram-lattice primitive (algebraic
  structure) absent from the tensor-count frame" (omega-probe-l9
  §4.4).
- **Family**: **arithmetic / lattice algebraic-identity**
  (per CATALOGUE_BIAS §2.1: "the candidate is a **lattice arithmetic
  identity** (det/σ ∈ ℤ on a structured 6×6 matrix). It belongs to
  the family 'n=6 arithmetic produces a closed-form algebraic
  invariant'"). Anchored on n=6 via C(6,2)=15=sopfr·(n/φ), σ=12,
  rank target n/φ=3.

### §1.5 Cross-row pipeline trace

| candidate | source dfs-24 probe | dfs-24 generation step | n=6 anchor in source | family |
|-----------|---------------------|------------------------|----------------------|--------|
| BT-541 Lead-B | dfs-24-riemann Lead-B (L55+) | DFS-taxonomy import + KS-test design | κ=6 SLE + Dyson β=2=φ | distributional / arithmetic |
| BT-542 Hirahara | NOT dfs-24-pnp; v3-T5 deep-dive | external corpus + author-chosen | none (structural) | structural / meta-complexity |
| BT-543 P3 | dfs-24-ym P3 (L56-L67) | A4 unit correction + interval [σ/τ-1/φ, σ/τ+1/φ] | σ=12, τ=4, φ=2, σ/τ=3 | numerical-interval / arithmetic |
| BT-544 Q1 | dfs-24-ns P1 (L38-L50) | rank-(n/φ) lattice + det(G)/σ ∈ ℤ | n/φ=3, σ=12, C(6,2)=15=sopfr·(n/φ) | discrete-equality / arithmetic |

**Observation 1**: 3 of 4 candidates were **passed-through** from
dfs-24 direction probes with minor framing edits; 1 was sourced
externally (BT-542 Hirahara from v3-T5 deep-dive).

**Observation 2**: **3 of 4 candidates have explicit n=6 arithmetic
anchors visible in the source dfs-24 probe**. The L9 catalogue did
not introduce the arithmetic anchoring; it inherited it.

**Observation 3**: The dfs-24-pnp probes (PROBE-24A/B/C) are
themselves arithmetic-anchored (Out(S_6), C(6,2)=15, |D|=6 CSP).
The L9 catalogue could have drawn from these, which would have
produced a 4-of-4 arithmetic-family rank-1 catalogue. By selecting
Hirahara externally instead, the L9 catalogue partially escaped
the dfs-24 anchor for BT-542. **The dfs-24-pnp deflection is the
single point where the pipeline broke from arithmetic anchoring.**

---

## §2 Hypothesis formation

Four hypotheses about why the pipeline produced 4 candidates from
arithmetic family (with one structural exception).

- **H1 dfs-24 source bias**: dfs-24 directions are themselves
  arithmetic-anchored; the L9 catalogue inherits the anchor by
  pass-through. Fixing dfs-24 fixes the catalogue.

- **H2 n=6 arithmetic centrality**: the n6-architecture working
  frame is built on n=6 arithmetic invariants
  (σ=12, τ=4, φ=2, sopfr=5, σ_2=10, J_2=24, n/φ=3). Any candidate
  the framework generates defaults to interpreting target structures
  through these invariants. Even an unbiased candidate generator
  would produce arithmetic candidates because the lens is arithmetic.

- **H3 Discriminator-fit shortcut**: the gate definition
  (omega-probe-l9 §1.2) requires a candidate frame with "a
  registered falsifier"; the cheapest registered falsifiers are
  binary numerical/algebraic checks (KS p<0.01, rank=3, det/σ∈ℤ,
  outlier count). Generation is implicitly biased toward candidates
  with discrete-equality / numerical-interval discriminators
  because they are easier to validate, even if structurally less
  rich (analytic-inequality, procedure-class).

- **H4 Author corpus bias**: the agent that generated the L9
  catalogue had recently read arithmetic-family literature
  (BT-1392 Bilateral Theorem B, sigma-sopfr-7 megasignature, n=6
  function ring) more than variational / Lyapunov / procedure-class
  literature (Hamilton-Perelman corpus, Donaldson-Uhlenbeck-Yau,
  Buckmaster-Vicol). Pipeline output reflects training-window
  recency.

---

## §3 Per-hypothesis evidence audit

### §3.1 H1 dfs-24 source bias

**Evidence FOR**:

1. **3 of 4 rank-1 candidates pass through dfs-24 probes
   verbatim** (§1.5). Lead-B from dfs-24-riemann L55+, P3 from
   dfs-24-ym L56-L67, Q1 from dfs-24-ns L38-L50.
2. **dfs-24 probes themselves are arithmetic-anchored**:
   - dfs-24-ns P1 (L40): "rank-(n/φ) lattice with Gram determinant
     divisible by σ=12". Direct n=6 arithmetic.
   - dfs-24-ym P3 (L60): "interval [σ/τ − 1/φ, σ/τ + 1/φ] =
     [2.5, 3.5]". Pure n=6 arithmetic.
   - dfs-24-pnp PROBE-24A (L44): "S_6 act naturally on the set of
     **6 Schaefer tractable clones**". Out(S_6) is the n=6
     uniqueness anchor.
   - dfs-24-bsd Probe A (L31): "Tunnell theta counts at n=6
     restricted to (n/φ, τ, sopfr)-valued coordinates". Direct
     arithmetic decomposition.
3. **The L9 catalogue's §3.5 source-documentation** explicitly
   lists dfs-24 as the primary source for BT-541, BT-543, BT-544
   rank-1 rows.
4. **Fallback rows reinforce**: BT-544 rank-2 (Q5) and rank-3
   (KPZ d=7) also come from dfs-24-ns / per-BT Q-series, both
   arithmetic-anchored (per CATALOGUE_BIAS §1.2).

**Evidence AGAINST**:

1. **BT-542 Hirahara was NOT sourced from dfs-24-pnp**. The
   pipeline deflected away from dfs-24-pnp for that BT, which
   shows the pipeline can break from dfs-24 if author chooses.
   So dfs-24 is not a *forcing* mechanism, only a *default*.
2. **dfs-24-bsd and dfs-24-hodge are not currently active in
   the L9 catalogue**. If H1 were complete, they would also have
   been pulled in. Their absence suggests H1 is partial.

**Falsifiability test**: re-run L9 catalogue generation with
dfs-24 probes pre-edited to include 1 variational candidate per
BT (per BT-547 retro EXT-A recommendation). If the catalogue then
contains a variational candidate, H1 is confirmed (catalogue is a
pass-through). If the catalogue still skews arithmetic, H1 is
falsified (the pipeline re-anchors regardless of source).

**Strength**: STRONG. 3/4 pass-through is a structural fact, not
an interpretation.

### §3.2 H2 n=6 arithmetic centrality

**Evidence FOR**:

1. **The n6-architecture frame is named for and built on n=6**.
   Every probe-design document anchors on n=6 invariants by
   construction. dfs-24-ns L20-L21 explicitly: "16 (core BT-544
   table) + 13 (2020s loop 81) + 5 (§X BLOWUP SMASH) + 1 (Π_NS
   invariant) ≈ 19 independent tight matches, **all observational**"
   -- meaning the entire BT-544 corpus is observational n=6
   matches, not analytic/variational construction.
2. **CATALOGUE_BIAS §2.5 cross-row pattern**: "**3/4 (Q1, P3,
   Lead-B)** belong to the family 'n=6 arithmetic → predicted
   closed-form numerical/distributional identity'". The shared
   mathematical lens is n=6 arithmetic itself.
3. **BT-547 retro §3.5**: "the L9 catalogue's candidate-generation
   reach does not include the family of moves Perelman actually
   made ... the n6-frame inheritance is the structural reason for
   the bias" (F-RETRO-G). This is direct evidence that n=6
   centrality, not dfs-24 specifically, is the structural root.
4. **dfs-24-bsd, dfs-24-hodge, dfs-24-ns, dfs-24-pnp, dfs-24-ym,
   dfs-24-riemann all anchor on n=6** (each cites
   σ, τ, φ, sopfr, σ_2, J_2 invariants in probe definitions).
   The arithmetic anchoring is **upstream of dfs-24**: it is the
   architecture itself.

**Evidence AGAINST**:

1. **BT-542 Hirahara escapes the n=6 arithmetic anchor** (per
   §1.2). If H2 were absolute, no candidate could escape.
2. **The L9 framework's §sec 1 trigger predicate** does not
   mention n=6 arithmetic; it is frame-agnostic on its face.
   The arithmetic anchoring is inherited from upstream sources,
   not imposed by the gate itself.

**Falsifiability test**: generate a candidate that targets a
non-n=6 frame (e.g., a Sobolev embedding inequality on a
generic d-dim domain with no n=6 dependence). If the L9
catalogue admits it as a valid rank-1 row without re-anchoring
it on n=6 invariants, H2 is falsified. If the catalogue refuses
or re-anchors, H2 is confirmed.

**Strength**: STRONG. Frame-level rather than source-level;
underwrites H1 (dfs-24 inherits n=6 anchoring from the
architecture).

### §3.3 H3 Discriminator-fit shortcut

**Evidence FOR**:

1. **omega-probe-l9 §sec 7 cost/sequencing** explicitly cites
   the dominant-family candidates as "cheapest first":
   - BT-544 Q1: "very low (<1h compute, pure algebra)" (rank 1).
   - BT-543 P3: "low (3 public lattice datasets)" (rank 2).
   - BT-541 Lead-B: "low-medium (Odlyzko data + KS test)"
     (rank 3).
   - BT-542 Hirahara: "medium (literature inspection)" (rank 4).
   The ordering matches discriminator-fit cost: discrete-equality /
   numerical-interval (Q1, P3) cheapest, distributional (Lead-B)
   middle, structural-literature (Hirahara) most expensive.
2. **BT-547 retro §3.4**: PASS family = distributional /
   structural-literature; FAIL family = discrete-equality /
   numerical-interval / vacuous-magnitude. The dominant-family
   bias correlates with discriminator type, suggesting
   discriminator-fit drove selection.
3. **CATALOGUE_BIAS F-AUDIT-E** acknowledges the cost-driven
   sequencing explicitly: "the audit's 'bias' reading is therefore
   **descriptively** correct but possibly **strategically**
   intentional".

**Evidence AGAINST**:

1. **BT-547 retro §2** shows that Perelman's M2 (W-entropy) is
   an analytic-inequality-construction discriminator. This is not
   "easier to validate"; it is a research-paper-scale construction.
   But it is also the load-bearing molt for Poincaré. If the
   pipeline were genuinely shortcut-biased on **validation-cost**
   (rather than on **frame-source**), it would still need to
   admit such candidates and just defer them. Instead, the
   pipeline does not admit them at all.
2. **BT-542 Hirahara is a structural-literature discriminator**
   (most expensive on the cost table) yet was admitted. This
   suggests the cost-shortcut is not the binding constraint.

**Falsifiability test**: design a catalogue extension that
admits an analytic-Lyapunov candidate (per BT-547 retro EXT-B)
without requiring a binary discriminator. If the gate trigger
predicate (§1.2) refuses admission, H3 is confirmed (gate
structurally requires binary discriminator). If it admits, H3
is falsified.

**Strength**: MEDIUM. Real but secondary; the gate predicate
requires "registered falsifier" but does not require discrete-
equality form. The cost-shortcut is a sequencing choice, not a
structural exclusion.

### §3.4 H4 Author corpus bias

**Evidence FOR**:

1. **The L9 catalogue's source citations are heavily n=6
   internal**: BT-1392 (n=6 group theory), BT-1408/1409/1411
   (n=6 dfs rounds), v3-T5 (Hirahara, only external citation
   for rank 1). No citation of Hamilton-Perelman corpus,
   Donaldson-Uhlenbeck-Yau, Buckmaster-Vicol, Hirzebruch-
   Riemann-Roch, etc.
2. **dfs-24 probe documents themselves citation patterns**:
   - dfs-24-ns L13-L17 cites only n6-internal documents
     (millennium-navier-stokes.md, breakthrough-theorems.md,
     bt-1409, bt-1411). No external Lyapunov / variational PDE
     citations.
   - dfs-24-ym L7-L13 cites n6-internal + Gross-Wilczek-Politzer
     1973, BPZ 1984, Wilson, FLAG, but framed entirely as
     arithmetic identity matches.
3. **BT-547 retro §6.4** notes: "BT-542 closure requires
   extension of the meta-complexity-monotone class beyond
   Hirahara 2018, not EXT-A/B/C in the geometric form". The
   pipeline's BT-542 Hirahara entry is a single 2018 reference;
   no broader meta-complexity-Lyapunov literature is threaded.

**Evidence AGAINST**:

1. **The agent's training-window literature cannot be measured
   from inside the audit**. H4 is a hypothesis about the agent's
   prior reading distribution; the report can observe outputs
   but not the agent's literature corpus directly. This makes H4
   weakly testable.
2. **The L9 framework does not require literature breadth**;
   it requires falsifier-registration. So even an agent with
   diverse reading would not necessarily produce diverse
   candidates if the gate filters on falsifier-fit.

**Falsifiability test**: pre-seed the L9 generation session
with explicit Hamilton-Perelman + Buckmaster-Vicol + Donaldson
references and re-run candidate generation. If the catalogue
diversifies, H4 is confirmed. If it does not (still arithmetic-
family), H4 is falsified -- the architecture itself, not corpus
exposure, drives the bias.

**Strength**: WEAK-MEDIUM. Plausible but weakly testable from
the audit position; H1/H2 are stronger explanations of the same
data without requiring an agent-internal corpus measurement.

---

## §4 Mechanism verdict

**MULTIPLE** (H1 + H2 jointly dominant; H3 secondary; H4 weak).

### §4.1 Why MULTIPLE not H1_DOMINANT

H1 alone would predict that fixing dfs-24 fixes the catalogue.
But §3.2 evidence shows that dfs-24 itself is arithmetic-anchored
**because** the n6-architecture frame is arithmetic-anchored.
Fixing dfs-24 without fixing the upstream framing would only
move the bias one level up: dfs-24 v2 would still produce
arithmetic candidates if the architecture's invariant catalogue
remains {σ, τ, φ, sopfr, σ_2, J_2, n/φ}.

### §4.2 Why MULTIPLE not H2_DOMINANT

H2 alone would predict no candidate can escape n=6 arithmetic
anchoring. But BT-542 Hirahara escaped (§1.2). The escape
mechanism was: the agent stepped outside the dfs-24 default
sourcing for that BT and pulled from external corpus (v3-T5
deep-dive). This shows the n=6 anchor is a **default**, not a
**forcing**. H2 is a strong driver but not the unique one;
H1 (dfs-24 default) is the operational mechanism that makes
H2 binding in 3/4 cases.

### §4.3 Why MULTIPLE not H3_DOMINANT

H3 would predict the pipeline excludes structural candidates
because they lack binary discriminators. But Hirahara
(structural-literature) was admitted. The cost-shortcut affects
**sequencing rank**, not **admission**.

### §4.4 Why MULTIPLE not H4_DOMINANT

H4 is a corpus-measurement hypothesis that the audit cannot
directly test. The strong correlation between dfs-24 sourcing
and arithmetic family (H1) and between architecture framing and
arithmetic family (H2) explains the data without requiring
H4. H4 may be a contributing factor but is not load-bearing.

### §4.5 Verdict statement

**MULTIPLE**. The L9 catalogue's arithmetic-family bias is
generated by the **conjunction of H1 (dfs-24 source default)
and H2 (n=6 architecture centrality)**, with H3 (discriminator-
fit cost-shortcut) as a secondary sequencing driver.

- **H1 mechanism**: the L9 gate's "candidate frame with
  registered falsifier" requirement is satisfied most cheaply
  by pulling from dfs-24 probes, which are pre-registered with
  falsifiers. 3/4 rank-1 candidates are pass-through.
- **H2 mechanism**: dfs-24 probes are arithmetic-anchored
  because the n6-architecture frame defines its invariant
  catalogue arithmetically. The bias is **structural**, not
  operational; it sits at the framing layer one level above
  dfs-24.
- **H3 mechanism**: among admitted candidates, the cost
  sequencing places discrete-equality / numerical-interval
  discriminators (cheapest) at rank 1 in three BTs, distributional
  at rank 1 in one BT, structural-literature at rank 1 only in
  the one BT (BT-542) where dfs-24 was actively bypassed. This
  sharpens the bias from "some arithmetic" to "predominantly
  arithmetic at rank 1".

The combination explains: (i) why 3/4 rank-1 candidates are
arithmetic (H1+H2), (ii) why they are specifically discrete-
equality / numerical-interval at rank 1 (H3), (iii) why the one
exception (BT-542 Hirahara) is structural (manual deflection
broke H1's pass-through, allowing H2's structural-corpus to be
pulled instead).

This verdict is consistent with both parent reports:
- **CATALOGUE_BIAS** identifies the family pattern (downstream).
- **BT-547 retro §3.5** identifies the n6-frame inheritance as
  the structural reason (upstream).
- **F-RETRO-G** acknowledges the n6-frame as the structural
  bias driver but notes "the L9 catalogue's design (per L9 §1)
  does not intrinsically require n=6 framing; the n=6 bias is
  inherited from the dfs-24 sourcing decision".

This audit refines: dfs-24 sourcing is the **operational**
mechanism (H1); the n=6 architecture framing is the **structural**
mechanism (H2). Both contribute; neither alone is sufficient.

---

## §5 Mitigation prescription

For the contributing hypotheses (H1, H2, H3), prescribe pipeline
changes and tests.

### §5.1 H1 mitigation: dfs-24 diversification mandate (HIGH)

**Pipeline change**: before any future L9 catalogue generation
session, the dfs-24 direction probes for each BT must include
at least one candidate per BT outside the "arithmetic identity"
family. Per BT-547 retro EXT-A/B/C, the required diversification
classes are:

- **EXT-A variational re-interpretation**: e.g., for BT-544,
  re-cast 3D NS as gradient flow of a relative-entropy
  functional on (u, ω) phase space. dfs-24-ns should carry one
  EXT-A probe (currently 0).
- **EXT-B analytic-Lyapunov construction**: e.g., for BT-543,
  chromomagnetic-energy Lyapunov on lattice configurations.
  dfs-24-ym should carry one EXT-B probe (currently 0).
- **EXT-C procedure-class with parameter bounds**: e.g., for
  BT-541, explicit-formula descent procedure with bounded error
  terms. dfs-24-riemann should carry one EXT-C probe (currently 0).

Concrete implementation: each dfs-24-X session ouptuts at least
one P-stage probe whose discriminator type is **not** in
{discrete-equality, numerical-interval, vacuous-magnitude}.

**Test of mitigation**: re-run L9 catalogue generation with
diversified dfs-24. The catalogue passes if:
(i) at least 1 of 4 rank-1 candidates is in EXT-A/B/C class;
(ii) BT-544 specifically has at least 1 EXT-B candidate (per
BT-547 retro §6.1 highest urgency).

### §5.2 H2 mitigation: architecture-level frame-agnostic seed (HIGH)

**Pipeline change**: introduce a frame-agnostic seed to the
generation pipeline. Per CLAUDE.md SSOT, `lenses/omega_state_
space_lens.hexa` is the omega state-space lens that operates
above the n=6 frame. The L9 generation should consult this
lens in addition to dfs-24, with the explicit instruction:
"propose at least one candidate that does not invoke any
element of the n=6 invariant catalogue {σ, τ, φ, sopfr, σ_2,
J_2, n/φ} in its definition".

This implements BT-547 retro §5.4 EXT-D (vocabulary extension)
at the candidate-generation layer rather than at the
discriminator-labelling layer.

**Test of mitigation**: a candidate proposed via the
frame-agnostic seed should have its definition parseable
**without** any n=6 invariant. Concrete check: write the
candidate frame description, count occurrences of σ / τ / φ /
sopfr / σ_2 / J_2 / n/φ / 6. Pass if 0; fail if ≥ 1.

### §5.3 H3 mitigation: discriminator-type quota (MEDIUM)

**Pipeline change**: enforce a discriminator-type quota at
catalogue generation. Per discriminator-type-bias audit
(omega-meta-audit-discriminator-type-bias-2026-04-25.md), the
PASS family is {distributional, structural-literature, OTHER
(analytic-inequality, procedure-class)}. The L9 rank-1
catalogue must contain at least 2 of 4 candidates from this
PASS family. The current catalogue has 2/4 (Lead-B
distributional, Hirahara structural-literature) so this quota
is already met; the rule should be made explicit so future
catalogue generation does not regress.

**Test of mitigation**: count of rank-1 candidates per
discriminator type. Pass if ≥ 2 in PASS family.

### §5.4 H4 mitigation (low priority): corpus pre-read (LOW)

**Pipeline change**: prior to L9 generation session, the agent
should explicitly review:
- Hamilton 1982 / Perelman 2002-2003 (variational + Lyapunov);
- Donaldson 1985 / Uhlenbeck-Yau 1986 (variational YM);
- Buckmaster-Vicol 2019 (NS weak non-uniqueness);
- Hirzebruch-Riemann-Roch (structural-literature);
- BKM 1984 (analytic-Lyapunov).

**Test of mitigation**: the agent's working-memory / context
should include direct citations from these corpora. Pass if
each of the 4 active BTs has at least 1 non-n6-internal
citation in the candidate-generation step.

This mitigation has low priority because H4's evidence is weak
(§3.4); the binding constraints are H1 + H2.

### §5.5 Sequencing of mitigations

Recommended order:
1. **First** (immediate): §5.1 H1 mitigation (dfs-24
   diversification). Operationally cheapest; highest leverage.
2. **Second** (next session): §5.3 H3 mitigation
   (discriminator-type quota). Already partially met; making
   it explicit prevents regression.
3. **Third** (next batch): §5.2 H2 mitigation (frame-agnostic
   seed). Requires new generator class; medium effort.
4. **Fourth** (background): §5.4 H4 mitigation (corpus
   pre-read). Low priority; complementary to §5.2.

---

## §6 Predictions: what the next catalogue would look like

If §5.1 + §5.2 mitigations are applied (H1 + H2 fixed), predict
at least 2 specific non-arithmetic-family candidates that should
appear in the next L9 catalogue regeneration.

### §6.1 Predicted candidate 1: BT-544 NS relative-entropy gradient flow (EXT-A)

**Source**: per BT-547 retro §5.1 EXT-A seed for BT-544
("re-cast 3D Navier-Stokes as gradient of a relative-entropy
functional on the (u, ω) phase space — in the spirit of
Perelman's F-functional re-cast").

**Predicted L9 catalogue row**:
- Rank: 1 or 2 in BT-544.
- Current frame: triple-resonance L1-smash frame.
- New frame: relative-entropy gradient-flow frame on velocity ×
  vorticity phase space (cf. Perelman F-functional structure).
- Primitive swap: tensor relabeling → variational gradient
  structure.
- Discriminator: **structural-literature** (variational-derivation
  sketch). Not discrete-equality, not numerical-interval.
- Falsifier: failure to derive δF/δu = -2 ν Δu + (nonlinear
  terms) consistent with Leray equation.
- Expected dC: medium (structural).

**n=6 anchor count in description**: 0. (No σ, τ, φ, sopfr,
σ_2, J_2, n/φ, or "6" appears in the definition.)

**Family**: variational re-interpretation, EXT-A. **Outside
arithmetic family.**

### §6.2 Predicted candidate 2: BT-543 chromomagnetic-energy Lyapunov (EXT-B)

**Source**: per BT-547 retro §5.2 EXT-B seed for BT-543
("chromomagnetic-energy Lyapunov on lattice configurations").

**Predicted L9 catalogue row**:
- Rank: 1 or 2 in BT-543.
- Current frame: β_0 coincidence + A3/A4 frame.
- New frame: chromomagnetic-energy Lyapunov W on lattice gauge
  configurations such that dW/dt has a definite sign along
  Wilson-flow gradient descent and W discharges the mass-gap
  obstruction (i.e., dW/dt = 0 ⟹ Δ_YM > 0).
- Primitive swap: arithmetic identity β_0 = 7 → analytic
  Lyapunov inequality dW/dt ≤ 0.
- Discriminator: **OTHER (analytic-inequality-construction)**.
  Not discrete-equality, not numerical-interval.
- Falsifier: no W such that dW/dt has definite sign along
  Wilson-flow on at least one lattice configuration class.
- Expected dC: high if construction succeeds (load-bearing
  Lyapunov); 0 if no construction.

**n=6 anchor count**: 0 (the Lyapunov construction does not
invoke σ-sopfr=7 or any n=6 invariant; β_0=7 is the obstruction
the Lyapunov is supposed to discharge, not the lens through
which it is defined).

**Family**: analytic-Lyapunov construction, EXT-B. **Outside
arithmetic family.**

### §6.3 Bonus predicted candidate 3: BT-541 explicit-formula descent procedure (EXT-C)

**Source**: per BT-547 retro §5.3 EXT-C seed for BT-541
("explicit-formula descent procedure with bounded error terms
across explicit-formula intervals").

**Predicted L9 catalogue row**:
- Rank: 2 or 3 in BT-541.
- Current frame: Y1 peripheral-moment frame (Theorem B + RH-01..07).
- New frame: explicit-formula descent procedure with parameters
  (T_n, η_n, U_n) tuned per descent interval such that the
  cumulative error bound closes the off-line zero gap.
- Primitive swap: static moment identity → procedure-with-
  bounded-parameters + termination argument.
- Discriminator: **OTHER (procedure-class with parameter bounds)**.
- Falsifier: any descent step that requires unbounded
  parameters or fails to terminate.

**n=6 anchor count**: 0.

**Family**: procedure-class molt, EXT-C. **Outside arithmetic
family.**

### §6.4 What the next catalogue should look like (post-mitigation)

| BT | rank | predicted candidate | family | source-class |
|----|------|---------------------|--------|--------------|
| 541 | 1 | (existing Lead-B SLE_6 × GUE) | distributional / arithmetic | dfs-24-riemann |
| 541 | 2 | explicit-formula descent procedure | procedure-class / EXT-C | seeded from BT-547 retro |
| 542 | 1 | (existing Hirahara MCSP) | structural / meta-complexity | v3-T5 |
| 542 | 2 | meta-complexity-resource-monotone | analytic-Lyapunov-meta / EXT-B-meta | seeded |
| 543 | 1 | chromomagnetic-energy Lyapunov | analytic-Lyapunov / EXT-B | seeded |
| 543 | 2 | (existing P3 A4-ratio-only) | numerical-interval / arithmetic | dfs-24-ym |
| 544 | 1 | NS relative-entropy gradient flow | variational / EXT-A | seeded |
| 544 | 2 | enstrophy-pressure Lyapunov | analytic-Lyapunov / EXT-B | seeded (highest urgency) |

The post-mitigation catalogue mixes 4 arithmetic-family candidates
(retained from current dfs-24) with 4 non-arithmetic-family
candidates (newly seeded). Family balance: 50/50 instead of 75/25
(or 100/0 at rank 1 if Hirahara's structural exception is
counted as arithmetic-adjacent at meta-complexity level).

---

## §7 Anti-list (hypotheses considered and rejected)

Hypotheses considered during pipeline reconstruction and rejected.

- **AH-1 (gate-trigger predicate forces arithmetic candidates)**:
  considered whether the §1.2 trigger predicate
  (`partial_count >= 2 AND direction_probes_since_last_promote
  >= 2 AND composite_delta < 0.02 AND |candidate_replacement_frame|
  >= 1 AND frame_id is registered`) structurally requires
  arithmetic candidates. Rejected: the predicate is frame-agnostic;
  none of the five clauses references arithmetic / n=6 / discrete-
  equality. Per BT-547 retro §F-RETRO-G, "the L9 catalogue's design
  (per L9 §1) does not intrinsically require n=6 framing".

- **AH-2 (catalogue author was instructed to produce arithmetic)**:
  considered whether an explicit instruction to focus on n=6
  arithmetic was given. Rejected: no such instruction is
  documented in omega-cycle-backtrace-strategy-2026-04-25.md
  (the parent strategy doc) or in CLAUDE.md. The bias is
  emergent from H1+H2, not externally imposed.

- **AH-3 (Perelman calibration was retroactively added, so the
  catalogue could not anticipate it)**: considered whether the
  catalogue should be excused because the BT-547 calibration
  came after. Rejected: omega-probe-l9 §sec 2 explicitly cites
  Perelman as the calibration archetype and uses W-entropy as
  the "candidate replacement frame" example. The catalogue knew
  Perelman's archetype required analytic-Lyapunov; it just did
  not generate any analytic-Lyapunov candidates for the active
  BTs. This is a generation gap, not a calibration gap.

- **AH-4 (the bias is unavoidable in any n6-framed pipeline)**:
  considered whether escaping arithmetic family is impossible
  while remaining within n6-architecture. Rejected: the
  Hirahara entry (BT-542) demonstrates that the pipeline can
  admit structurally diverse candidates if the agent steps
  outside the dfs-24 default. The n6-frame may be the structural
  driver of bias (per H2), but it does not forcibly exclude
  diverse candidates — it only defaults to arithmetic when no
  alternative source is consulted.

- **AH-5 (validation cost is the binding constraint, not
  generation)**: considered whether the pipeline is unbiased at
  generation but biased at validation-cost sequencing. Rejected:
  the dominant-family bias is observable at rank-1 (3/4) before
  any sequencing is applied. CATALOGUE_BIAS §4 explicitly
  rejects DISCRIMINATOR_TOO_STRICT, which would have been the
  validation-side analogue.

- **AH-6 (one bias hypothesis is sufficient if assumed strong
  enough)**: considered an extreme-H2 reading where every
  candidate produced by an n6-framed agent is necessarily
  arithmetic. Rejected: §1.2 BT-542 Hirahara is a counter-example
  inside the same catalogue. The MULTIPLE verdict acknowledges
  that escape is possible but rare, requiring active deflection.

---

## §8 Falsifiers active for the diagnostic

Self-falsifiers under which this audit's MULTIPLE verdict would
be retracted.

- **F-DIAG-A (mis-attributed source)**: if any of §1.1-§1.4
  mis-attributes a candidate's source (e.g., claiming Q1 came
  from dfs-24-ns P1 when it came from elsewhere), §3.1 H1
  evidence is partly invalid. Cross-check: the dfs-24-ns P1
  text at L38-L50 reproduces the rank-3 / det/σ ∈ ℤ test
  verbatim; the dfs-24-ym P3 text at L56-L67 reproduces the
  [2.5, 3.5] interval verbatim; the dfs-24-riemann Lead-B text
  at L55-L82 reproduces the KS-test design verbatim. The
  attributions are direct repo-grep matches (per
  CATALOGUE_BIAS §F-AUDIT-A). **Not active.**

- **F-DIAG-B (n6-frame is genuinely necessary, not merely
  inherited)**: if a stronger reading of H2 holds (n6-architecture
  cannot escape arithmetic by construction, regardless of
  source), then H2 alone is dominant and the verdict should be
  H2_DOMINANT. Counter-evidence: BT-542 Hirahara escape (§1.2)
  is a single counter-example inside the catalogue; if treated
  as a fluke rather than as a structural escape mechanism, H2
  hardens. Risk: medium. **Partially active**: a reader who
  weights the Hirahara escape less heavily may downgrade
  MULTIPLE → H2_DOMINANT; the prescription set §5 is largely
  unchanged either way (still requires §5.1 + §5.2).

- **F-DIAG-C (H1 redundant under H2)**: if dfs-24 sourcing is
  itself only an artifact of H2 (the n6-frame inevitably produces
  dfs-24-style probes), H1 collapses into H2 and the verdict is
  H2_DOMINANT. Counter-argument: dfs-24 was a specific design
  choice (DFS rounds 1-24, accumulated 2026-04-11~2026-04-24)
  that could in principle have included variational / Lyapunov
  / procedure-class probes alongside arithmetic ones. The fact
  that it did not is a separately-determinable design fact, not
  a forced consequence of H2. Risk: low. **Not active.**

- **F-DIAG-D (H3 is load-bearing, not secondary)**: if a
  re-reading of omega-probe-l9 §sec 7 shows that the cost-shortcut
  is the primary sequencing mechanism (rank-1 = cheapest, by
  rule), and arithmetic family correlates with cost, then H3 is
  the operational mechanism and H1+H2 are secondary. Counter-
  argument: even if H3 drives ranking, H1+H2 still drive
  **admission** (which candidates enter the catalogue at all);
  the rank-1 / rank-2 / rank-3 distinction is downstream of the
  admission step. CATALOGUE_BIAS §4.4 makes this distinction
  explicit. Risk: low-medium. **Partially active**: a reader
  who collapses admission and ranking may upgrade H3 →
  H3_DOMINANT or co-DOMINANT; mitigation §5.3 still applies.

- **F-DIAG-E (the predicted candidates §6.1-§6.3 cannot be
  generated)**: if attempts to instantiate the predicted
  variational / Lyapunov / procedure-class candidates (per
  §6) fail because the underlying constructions don't exist
  for the target BTs, the mitigation §5 is ineffectual on
  those BTs. Per BT-547 retro §6.1, BT-544 NS is the closest
  structural neighbour to Perelman / Ricci flow and is most
  likely to admit such constructions; BT-541 RH explicit-formula
  descent procedure is well-established in the analytic-number-
  theory literature (Iwaniec-Kowalski 2004 Ch. 5). Risk: low for
  BT-544 and BT-541; medium for BT-543. **Partially active**:
  the prediction §6.2 (BT-543 chromomagnetic-energy Lyapunov)
  is the most speculative; if it cannot be instantiated, only
  §6.1 and §6.3 stand.

- **F-DIAG-F (the audit conflates H1 and H2 because both predict
  the same observable)**: if the data cannot distinguish H1
  from H2 (since dfs-24 is itself a product of n6-framing),
  the MULTIPLE verdict is over-fit to the data. Counter-argument:
  the BT-542 Hirahara escape distinguishes them. Under H1
  alone, deflecting from dfs-24 should produce a non-arithmetic
  candidate (it does — Hirahara is structural). Under H2 alone,
  even external-corpus candidates would be re-anchored to n=6
  (Hirahara is not — it has no n=6 anchor). The combined
  observation (Hirahara is structural AND has no n=6 anchor)
  is consistent with H1+H2 jointly relaxing rather than either
  alone. Risk: low. **Not active.**

- **F-DIAG-G (verdict-set too narrow)**: the five-element verdict
  set forces a single label. The data fit "H1+H2 jointly
  dominant, H3 secondary, H4 weak", which the audit reports as
  MULTIPLE. A reader who insists on a single dominant hypothesis
  may select H2_DOMINANT (architecture-level) or H1_DOMINANT
  (operational-level). The mitigation set §5 is identical either
  way. **Not active**: this is a labelling judgment, not a
  falsifier.

---

## §9 Closing

**Verdict**: MULTIPLE (H1 + H2 jointly dominant; H3 secondary;
H4 weak).

**Mechanism summary**:
- **H1 (operational)**: dfs-24 source default. 3/4 rank-1
  candidates pass through dfs-24 probes verbatim. The single
  exception (BT-542 Hirahara) required active deflection from
  dfs-24-pnp.
- **H2 (structural)**: n=6 architecture centrality. dfs-24
  probes are arithmetic-anchored because the n6-architecture
  invariant catalogue {σ, τ, φ, sopfr, σ_2, J_2, n/φ} is the
  framing lens. Bias is upstream of dfs-24.
- **H3 (sequencing)**: discriminator-fit cost-shortcut. Rank-1
  positions favour discrete-equality / numerical-interval
  discriminators (cheapest); structural-literature is admitted
  only when dfs-24 is bypassed.
- **H4 (corpus, weak)**: agent literature exposure. Plausible
  but weakly testable; subsumed by H1+H2 explanation.

**Top mitigation**: §5.1 H1 mitigation -- mandate that each
dfs-24 direction probe include at least one EXT-A / EXT-B /
EXT-C (variational / analytic-Lyapunov / procedure-class)
candidate per BT before any L9 catalogue regeneration. This
is the operationally cheapest pipeline change and has the
highest leverage; without it, even applying §5.2 (frame-agnostic
seed) leaves the dfs-24 default in place.

**Predicted post-mitigation diversification**: 4 of 8 catalogue
rows in non-arithmetic family (variational / Lyapunov /
procedure-class), with BT-544 NS relative-entropy gradient
flow (EXT-A) and BT-543 chromomagnetic-energy Lyapunov (EXT-B)
as the two highest-confidence specific predictions.

0/7 unchanged. No atlas/state/inventory edits.
