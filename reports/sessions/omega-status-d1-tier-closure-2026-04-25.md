---
id: omega-status-d1-tier-closure
date: 2026-04-25
scope: status synthesis (NOT producing new claims; declaring D1 tier closed)
target: D1 atlas-scan tier closure -- 1/4 PASS, 4 distinct failure modes, EXT-tier as highest priority next
parent_reports:
  - reports/sessions/omega-seed-bt544-d1-atlas-scan-2026-04-25.md (seed)
  - reports/sessions/omega-exec-bt544-d1-1-hvc-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt544-d1-3-ns-mhd-duality-2026-04-25.md
  - reports/sessions/omega-exec-bt544-d1-4-she-leveque-molt-validation-2026-04-25.md
  - reports/sessions/omega-exec-bt544-d1-5-group-symmetry-2026-04-25.md
millennium_resolved: 0/7 (unchanged)
grade: tier closure synthesis, no claim
---

# Omega Status -- BT-544 D1 Tier Closure (2026-04-25)

## §0 Non-claim disclaimer

This document is a **status synthesis** declaring the BT-544 D1
atlas-scan tier formally closed. It does **NOT**:

- claim 3D Navier-Stokes regularity (smoothness or blow-up) in any
  direction;
- promote, demote, or otherwise touch any entry in
  `shared/n6/atlas.n6` or `~/core/nexus/n6/atlas.millennium.n6`;
- modify `state/proposals/inventory.json`, `theory/canon/`, or any
  L9/D1/D2/D3/EXT source document;
- alter the `BT-544 = 0/1 untouched` Clay status;
- re-judge any of the four executed D1 reports (D1.1, D1.3, D1.4,
  D1.5);
- produce any new molt-validation result.

Millennium tally: **0/7 unchanged**. This synthesis records the D1
tier as fully exercised, identifies the next-priority dispatch
batch (EXT-tier), and notes the falsifiers that remain active after
D1 closure. No new claims; only re-statement of existing verdicts
plus tier-level inference about next priority.

---

## §1 D1 tier inventory

The D1 atlas-scan tier was proposed in
`omega-seed-bt544-d1-atlas-scan-2026-04-25.md` as a five-candidate
heterogeneous extension of the failed L9 BT-544 catalogue
(Q1 KdV-Gram, Q5 Sobolev/Besov, KPZ d=7). The five seed candidates
and their status as of 2026-04-25:

| ID    | Name                                  | Family                | Discriminator type            | Status                          | Source report |
|-------|---------------------------------------|-----------------------|-------------------------------|---------------------------------|---------------|
| D1.1  | HVC-import (Bekenstein cap)           | info-theory           | vacuous-magnitude (post-eval) | **FAIL** (FAIL_VACUOUS)         | `omega-exec-bt544-d1-1-hvc-molt-validation-2026-04-25.md` |
| D1.2  | Polya recurrence dimensional resonance| probability           | (not classified)              | **NOT EXECUTED** (skipped per seed §5 top-2 ranking) | n/a |
| D1.3  | NS<->MHD duality at R_m=Ha=48         | cross-PDE structural  | structural cross-PDE          | **FAIL** (FAIL_NO_DUALITY + FAIL_RELABELING secondary) | `omega-exec-bt544-d1-3-ns-mhd-duality-2026-04-25.md` |
| D1.4  | She-Leveque residual zeta_6 - 2 = -2/9| arithmetic-numerology | discrete-equality             | **PASS** (arithmetic-identity grade) | `omega-exec-bt544-d1-4-she-leveque-molt-validation-2026-04-25.md` |
| D1.5  | Axisymmetric-no-swirl s* quantization | group-symmetry        | discrete-equality + vacuous-target | **FAIL** (FAIL_NO_LITERATURE_PATH) | `omega-exec-bt544-d1-5-group-symmetry-2026-04-25.md` |

D1 tier execution rate: **4 of 5 candidates validated** (D1.2
deliberately skipped per the seed's §5 top-2 dispatch ranking,
which advanced D1.1 + D1.4 first; the subsequent decision to
extend to D1.3 + D1.5 came from post-execution strategy redirect,
not from D1.2 promotion).

---

## §2 Verdict tally

Of the four executed D1 candidates:

- **PASS**: 1 (D1.4 She-Leveque residual; arithmetic-identity grade)
- **FAIL**: 3 (D1.1 HVC, D1.3 NS<->MHD, D1.5 axisym-no-swirl)

Adding the seed's catalogue-context (the three pre-D1 catalogue
failures Q1, Q5, KPZ d=7 plus the meta-audit's earlier Lead-B,
Hirahara distrib-row PASSes from cross-BT samples) is out of scope
for this synthesis; the D1-only tally is the one that closes the
tier:

  **D1 tier verdict tally: 1 PASS / 3 FAIL out of 4 executed
  (D1.2 unexecuted-by-design).**

  **D1-tier PASS rate: 1/4 = 25%.**

This is below the seed-design's implicit expectation that
heterogeneous family coverage would surface at least one
non-arithmetic PASS; in the event the only PASS came from the
arithmetic family (D1.4), with the three non-arithmetic candidates
(D1.1 info-theory, D1.3 cross-PDE, D1.5 group-symmetry) all
failing.

---

## §3 Family heterogeneity verification

The D1 seed (§7 F-SEED-B) claimed five distinct primitive families:

> D1.1 = info-theory, D1.2 = probability, D1.3 = cross-PDE,
> D1.4 = arithmetic-numerology, D1.5 = group-symmetry-reduction

For the four executed candidates, the heterogeneity claim is
**verified**:

| Candidate | Primitive family            | Verified by                                                   |
|-----------|-----------------------------|---------------------------------------------------------------|
| D1.1      | information theory          | Phi_holo MI on (interior, boundary) of vorticity (D1.1 §2)    |
| D1.3      | cross-PDE structural        | NS<->MHD literature audit (Sermange-Temam, He-Xin, CKS)       |
| D1.4      | arithmetic-numerology       | rational-function enumeration over n=6 lattice ring (D1.4 §3) |
| D1.5      | group-theory + numerology overlay | SO(2) axial reduction + n=6-discretization overlay (D1.5 §2) |

Caveat from D1.5 §8.3 logged: D1.5's *group-theory content* is the
genuine SO(2) reduction (canonical in Ladyzhenskaya 1968 etc.),
but the n=6-discretization overlay on the swirl magnitude is
arithmetic-numerology, not group-theory (SO(2) acts on theta, not
on |u_theta|). This **partially weakens** F-SEED-B's claim that
D1.5 is "group-symmetry-reduction" cleanly distinct from D1.4's
arithmetic family. Nonetheless, the underlying group-theoretic
reduction (the SO(2) quotient itself) is real and distinct from
information theory, cross-PDE structure, and pure arithmetic
enumeration.

**F-SEED-B status post-D1.5**: not fired in the strong sense
(four distinct primitive sources are still present); partially
weakened (the overlay character of D1.5's discretization noted).

---

## §4 Failure mode taxonomy

The three D1 FAILs exhibit three structurally distinct failure
modes, each documented in its source report:

### §4.1 D1.1 -- vacuous-magnitude

The Bekenstein cap pi*R^2/(nu*T) is non-zero (52.36 on F-A
Lamb-Oseen, 13.09 on F-C Taylor-Green) and **uncrossed** by the
measured Phi_holo, but the gap is so large that the cap performs
no constraint work:

- F-A Lamb-Oseen: max Phi_holo / cap = 0.000 (boundary samples
  collapse to zero bin; "degenerate boundary")
- F-C Taylor-Green: max Phi_holo / cap = 0.004 (cap exceeded by
  factor ~250)

Failure-mode label: **F-D1.1-B fires** (cap decorative; primitive
vacuous on smooth flows). The cap is consistent (criterion (a)
holds) but empty of information (criterion (b) fails).

### §4.2 D1.3 -- structural-non-existence (no-duality)

The literature (Sermange-Temam 1983; Caflisch-Klapper-Steele 1997;
He-Xin 2005; Wu 2003-2014; Cao-Wu 2011) establishes the 3D NS / 3D
MHD relationship as **asymmetric**, not bidirectional:

- NS-regularity-knowledge => MHD-regularity (He-Xin 2005;
  trivially via B=0 reduction)
- MHD-regularity-knowledge => NS-regularity: **not in literature**
- Bidirectional duality preserving regularity transfer: **not in
  literature**

The seed's numerical proxy `Z_NS / Z_MHD -> 6 = n` at matched
(Re, R_m) = (288, 48) is a tautology of n=6 lattice arithmetic
(288/48 = J_2/tau = 6); it has no PDE-theoretic interpretation.

Failure-mode label: **FAIL_NO_DUALITY** (primary) with
**FAIL_RELABELING** (secondary; the "duality" reduces to relabeling
the trivial B=0 inclusion).

### §4.3 D1.5 -- no-literature-path (vacuous-target)

The discriminator's gate criterion PASS-1 requires a published
critical-swirl threshold s* > 0 for 3D NS. The audit (D1.5 §4)
established that the literature is structured as a **dichotomy**
(u_theta = 0 vs general u_theta), not a threshold:

- Galdi-Padula 1990, Ladyzhenskaya 1968, Chen-Strain-Tsai-Yau 2008:
  regularity at u_theta = 0; no s_c.
- Hou-Luo 2014, Chen-Hou 2022: blow-up scenarios for 3D **Euler**
  (not NS), at specific corner-of-cylinder geometries, not
  parametrized by a swirl magnitude scalar.

The discriminator cannot be evaluated because its input quantity
does not exist as a literature number.

Failure-mode label: **FAIL_NO_LITERATURE_PATH** (gate failure;
PASS-1 not satisfied).

### §4.4 D1.4 -- the unique PASS (arithmetic-identity grade)

D1.4 (the only PASS) is structurally unlike the three FAILs:

- Target value -2/9 = zeta_6 - 2 (She-Leveque model, 1994) is
  a **definite published number** (not vacuous).
- Lattice expression `-tau^2 / (n*sigma) = -16/72 = -2/9`
  reproduces the target exactly (length-4 in conservative ring;
  length-3 in extended ring including n/phi = 3).
- Selection-bias guard non-firing: nearest competitors -1/4, -1/5,
  -5/24 all sit outside the 5% window (D1.4 §3.3).

Failure modes that *would have* fired (and did not):
- F-D1.4-A (>=2 expressions compete within 5%): only one *value*
  in the window.
- F-D1.4-B (best-fit error > 5%): error = 0.000 exact.
- F-VACUOUS (target too generic): denominator 9 = n + n/phi is
  not a primary atomic factor; emerges via cancellation.

Per D1.4 §4.3-§4.4, the PASS is **arithmetic-identity grade**, not
PDE-regularity grade: it does not advance BT-544 toward Clay
closure, only registers a non-trivial arithmetic relation.

### §4.5 Taxonomy summary

Four distinct verdict types across the four executed candidates:

| Verdict label              | Candidate | Mechanism of failure (or pass) |
|----------------------------|-----------|-------------------------------|
| FAIL_VACUOUS               | D1.1      | cap non-vacuous in target value, vacuous in measured value (Phi_holo << cap) |
| FAIL_NO_DUALITY (+RELAB)   | D1.3      | structural cross-equation transfer is asymmetric in literature; "duality" reduces to relabeling B=0 inclusion |
| FAIL_NO_LITERATURE_PATH    | D1.5      | gate criterion's input quantity (s*) does not exist as a published number |
| PASS (arithmetic-identity) | D1.4      | discrete target -2/9 has unique short n=6 lattice rational expression within tolerance, no competitor |

The single PASS exceeds the 2% error threshold uniquely (error
0.000 < 0.02 * |target|); the three FAILs each fail by a
structurally distinct mode.

---

## §5 Cross-cell entries (bias 2x2 post-D1)

The discriminator-type bias 2x2 (Distrib/struct-lit row vs
Discrete-eq/interval/vacuous row), tracked across the meta-audit
samples and D1 executions:

### §5.1 D1.4 (PASS) -- the cross-cell entry that confirmed CONFOUNDED

D1.4 was the **first PASS in the discrete-equality /
vacuous-magnitude row**. Per
`omega-exec-bt544-d1-4-she-leveque-molt-validation-2026-04-25.md`
§5, this PASS:

- shifted Fisher exact two-sided p from ~0.036 (n=8 pre-D1.4) to
  ~0.095 (n=9 post-D1.4);
- demonstrated that discrete-equality discriminators **can** PASS
  when the candidate is structurally non-trivial (the target -2/9
  is grounded in the She-Leveque 1994 model with experimental
  zeta_6 in [1.7, 1.8]);
- supported the meta-audit's CONFOUNDED reading: candidate-validity
  drives outcome, not discriminator type per se.

D1.4 is the structural existence proof that the discrete-equality
row is not categorically failing; what was failing was the pairing
with weak (numerology-only) candidates.

### §5.2 D1.3 (FAIL) -- the cross-cell entry that did not happen

The seed framed D1.3 as a *cross-PDE structural* discriminator,
which under the bias 2x2 surface labels would land in the
distributional / structural-literature row (the row that had been
3 PASS / 0 FAIL pre-D1). A PASS for D1.3 would have left the
distrib row at 4 PASS / 0 FAIL, sharpening the bias evidence.

In the event, D1.3 FAILed (FAIL_NO_DUALITY). Per
`omega-exec-bt544-d1-3-ns-mhd-duality-2026-04-25.md` §5, this is
the **first non-arithmetic FAIL with a non-vacuous discriminator**
(the NS / MHD distinction is mathematically real; the asserted
*duality* is what the literature does not provide). D1.3 thus
adds the first FAIL to the structural-literature row -- but the
addition is in a row that the bias-hypothesis tracking treats
under D1.3's substantive sub-label rather than the surface label.

### §5.3 D1.5 (FAIL) -- same-cell entry; refines the diagnostic

D1.5 lands in the discrete-equality / vacuous row (per its hybrid
classification: discrete-equality target list with vacuous-target
gate failure). Its FAIL adds to the 5 prior FAILs in this row.

The post-D1.5 2x2 (per D1.5 §6):

|                                                              | PASS | FAIL |
|--------------------------------------------------------------|------|------|
| Distributional / structural-literature                       | 3    | 0    |
| Discrete-equality / numerical-interval / vacuous-magnitude   | 1    | 6    |

Fisher exact two-sided p ~ 0.067 at n=10. The CONFOUNDED verdict
stands.

### §5.4 Sub-pattern across the four D1 FAILs

A finer pattern emerges across D1.1, D1.3, D1.5 (the three FAILs):

- **D1.1**: discriminator embedded the unverified existence
  assumption that Phi_holo would be near the cap (i.e. that the
  cap is non-vacuous on the test flows). It is not.
- **D1.3**: discriminator embedded the unverified existence
  assumption that an NS<->MHD duality exists in the literature.
  It does not.
- **D1.5**: discriminator embedded the unverified existence
  assumption that a published critical swirl threshold s* exists.
  It does not.

Three of three D1 FAILs cluster on the pattern **"discriminator
embeds an unverified existence assumption"**. D1.3 additionally
exhibits the secondary mode **"reduces to relabeling"** (the
"duality" name reduces to relabeling the B=0 inclusion).

The discriminator-type bias hypothesis is now **CONFOUNDED** and
**cross-cell-confirmed**:
- CONFOUNDED at the 2x2 level (per the meta-audit; D1.4 PASS in
  the discrete-eq row + D1.3 FAIL substantively in the
  structural row prevent type-axis from isolating the failure).
- Cross-cell-confirmed by D1.4: the row that was 0/5 FAIL is now
  1/6 with the PASS coming from a structurally-grounded candidate;
  the row that was 3/3 PASS gains its first FAIL from a candidate
  embedding an unverified existence assumption.

---

## §6 D1 vs D2 vs D3 vs EXT comparison

Cross-tier status for BT-544 as of 2026-04-25:

| Tier                         | Candidates                | PASS count                   | FAIL count           | In-flight                      | Notes |
|------------------------------|---------------------------|------------------------------|----------------------|--------------------------------|-------|
| D1 (atlas-scan)              | D1.1..D1.5 (D1.2 skipped) | **1** (D1.4, arith-identity) | **3** (D1.1/D1.3/D1.5) | none (tier closed)           | this synthesis |
| D2 (axiom-recast)            | R1, R5                    | 0 strict-PASS yet            | 0 strict-FAIL yet    | R1 retry-3 BCV-2021 (#59); R5 converged (D3_CONFIRMED at s*=2) | R1 Lemma 1 OBSTRUCTION_DEEPER; R5 Lemma 1 PASS auxiliary |
| D3 (mechanism-decouple)      | A, B', C                  | **1** (A; PASS_LITERATURE)   | **2** (B', C; FAIL_INTERMITTENCY etc.) | none (program closed)        | program officially closed per `omega-decide-bt544-d3-strategy-postC-2026-04-25.md` |
| EXT (extension tier)         | EXT-A variational, EXT-B Lyapunov | 0 verdicts yet      | 0 verdicts yet       | EXT-A NS gradient-flow (#58); EXT-B CI-Lyap candidate generated | candidate generation pipeline (BT-547 retro recommended) |

Verdict shape across tiers:

- **D1**: 1/4 PASS (atlas-scan; the unique PASS is arithmetic and
  PDE-inert).
- **D2**: 0/2 strict-PASS yet (R1 OBSTRUCTION_DEEPER twice + retry-3
  in-flight; R5 Lemma 1 PASS but auxiliary, Lemma 2 D3_CONFIRMED at
  s*=2 -- R5 program could close).
- **D3**: 1/3 PASS (A only; compositional B'+C strategy failed,
  program closed).
- **EXT**: no verdicts yet (one in-flight, one candidate generated).

Cross-tier observations:
- D1 and D3 each contributed exactly one PASS, neither of which
  advanced BT-544 toward Clay closure (D1.4 = arithmetic identity;
  D3.A = literature recall).
- D2 has produced no strict-PASS but two OBSTRUCTION_DEEPER
  diagnostics on R1 and one auxiliary PASS on R5; the auxiliary
  result has converged to D3_CONFIRMED at s*=2, suggesting R5
  program could close.
- EXT-tier is the only batch with no verdicts yet; the
  generation-pipeline diagnostic (in-flight #51, fed by D1.1 +
  D1.5 existence-assumption pattern) is recommended for
  pre-validation existence audit on EXT candidates.

**EXT-tier is the highest-priority next batch**, on the basis
that: (a) it is the only tier with no verdicts yet, (b) it is the
only tier that has not been ruled CONFOUNDED or CLOSED, and (c) the
generation-pipeline diagnostic (BT-547 retro recommended)
specifically targets the EXT candidate-generation step.

---

## §7 Next priority recommendation

In priority order for the next dispatch batch:

1. **EXT-B CI-Lyap validation** (candidate generated, awaiting
   validation). Expected verdict per `omega-decide` notes:
   ~75% probability OBSTRUCTION_DOCUMENTED (Lyapunov-functional
   obstruction characterized but not lifted). The 25% residual is
   the prospect of a clean OBSTRUCTION_LIFT or PASS_LITERATURE.
   Cost: medium (a literature audit against the Lyapunov-NS
   bibliography; Foias-Manley-Rosa-Temam 2001 ch.10; Robinson
   *Infinite-Dimensional Dynamical Systems* 2001).

2. **EXT-A NS gradient-flow validation** (#58 in-flight). The NS
   gradient-flow extension is in-flight via the variational frame.
   Awaiting completion; verdict expected within the next session
   batch.

3. **R1 Lemma 1 retry-3 BCV-2021** (#59 in-flight). The third
   retry on the R1 Lemma 1 OBSTRUCTION_DEEPER, this time grounded
   in Buckmaster-Colombo-Vicol 2021 *Annals* (the convex-integration
   non-uniqueness for Leray-Hopf). Expected: another OBSTRUCTION
   diagnostic at a deeper level, or a clean failure that closes
   R1 program.

4. **R5 program closure** (already converged; auxiliary PASS plus
   D3_CONFIRMED at s*=2). R5 has reached a stable verdict
   configuration; closing it formally would free dispatcher
   bandwidth. Recommended after items 1-3.

The recommendation is to dispatch EXT-B first (lowest dispatch
cost, highest information yield since EXT-tier has no prior
verdicts), then await EXT-A and R1 retry-3 outcomes, then close
R5.

---

## §8 D1 tier formal closure declaration

**The D1 atlas-scan tier of BT-544 is declared formally closed as
of 2026-04-25.**

Closure rationale:

1. **4 of 5 candidates validated** (D1.2 deliberately unexecuted
   per seed §5 ranking; not a defect of the tier).
2. **All four executed verdicts are stable** (no in-flight D1
   candidates remaining).
3. **Verdict tally**: 1 PASS / 3 FAIL out of 4 executed.
4. **Family heterogeneity**: verified across info-theory /
   cross-PDE / arithmetic-numerology / group-theory + numerology
   overlay (4 distinct primitive sources).
5. **Failure mode taxonomy**: 3 distinct FAIL labels (FAIL_VACUOUS,
   FAIL_NO_DUALITY+RELABELING, FAIL_NO_LITERATURE_PATH) plus 1 PASS
   label (arithmetic-identity grade); 4 distinct verdict shapes.
6. **No PDE-regularity advance for BT-544**: the unique PASS
   (D1.4) is arithmetic-identity grade, PDE-inert. The 0/7
   Millennium status is unchanged.
7. **CATALOGUE_BIAS / CONFOUNDED meta-audit verdict** stands and
   is sharpened (not flipped) by the D1 outcomes.
8. **Generation-pipeline diagnostic feed** (in-flight #51): D1.1 +
   D1.5 supply two clean instances of "discriminator authored
   under unverified existence assumption", plus D1.3 supplies a
   "discriminator authored under unverified structural-transfer
   assumption". This is actionable feedback for pre-validation
   existence audits on future tier candidates.

No further D1 candidates are expected. D1.2 (Polya recurrence) is
deliberately retired as a fallback, since the tier-closure pattern
(3 of 3 non-arithmetic D1 candidates FAIL) does not predict a
non-arithmetic D1.2 PASS at competitive cost, and the next-priority
batch (EXT-tier) is more likely to produce decisive verdicts.

---

## §9 Anti-list (candidates skipped, with rationale)

The following D1 candidates were proposed in the seed but not
executed; rationale recorded for transparency.

- **D1.2 Polya recurrence dimensional resonance**. Proposed in
  seed §3.2; dispatch-skipped per seed §5 top-2 ranking (which
  promoted D1.1 + D1.4 as cheapest / most-grounded). After
  D1.1 FAIL_VACUOUS and the D1.3 + D1.5 redirect, D1.2 was not
  re-promoted because (a) the three executed non-arithmetic D1
  candidates all FAILed, providing no Bayesian update favoring
  D1.2; (b) D1.2 itself carries the highest "labeling-not-
  derivation" risk per the seed §4 ranking (d=3 = n/phi is
  already in NS-CORE-03 / NS-OBS-01, so the recurrence-transience
  result might collapse to F-Q5-style relabeling); (c) the
  next-priority EXT-tier is judged more decisive than D1.2 at
  comparable dispatch cost. Recorded as **NOT EXECUTED** rather
  than **DEFERRED** -- the tier closes without it.

The seed-design's anti-list (R-1..R-10 in seed §6) remains
unchanged: the candidates rejected during seed construction
(Reynolds-stress 6-component count, Lawson 48T cap, Carnot
efficiency, etc.) are not promoted by D1 closure.

---

## §10 Falsifiers active

Active falsifier register at D1 tier closure:

### §10.1 D1-internal falsifiers (per seed §7 + per-candidate)

- **F-D1.1-B (CONSUMED)**: cap decorative on smooth flows; fired
  at D1.1 execution.
- **F-D1.1-C (NOT EXECUTED)**: literature pre-audit for prior
  info-theoretic NS bound; deferred (no offline literature access).
  **Conditionally active** -- if a future session executes the
  pre-audit and finds Foias-Manley-Rosa-Temam 2001 ch.10 or Gibbon
  2007 subsuming HVC, the D1.1 FAIL is reinforced as
  "re-derivation".
- **F-D1.3 (FIRED in structural mode)**: NS<->MHD duality does not
  exist as posed; F-D1.3-A and F-D1.3-B were superseded by the
  structural pre-emption (no DNS run was executed because the
  literature-level finding made it unnecessary).
- **F-D1.4-A, F-D1.4-B (INACTIVE)**: D1.4 PASS, no falsifier fired.
- **F-D1.5-A (vacuously inactive)**: best-fit s* undefined because
  no s* exists.
- **F-D1.5-B (FIRED)**: critical swirl is not threshold-like; the
  reduction-to-discrete-n=6 framing is wrong.
- **F-D1.5 (FIRED in gate mode)**: PASS-1 not satisfied; gate
  failure.

### §10.2 Seed-design level falsifiers (carried)

- **F-SEED-A (atlas-grounding integrity)**: D1.1 borderline (path
  drift on `phi_holographic_measure.hexa` from nexus to anima); D1.3
  not fired; D1.4 not fired; D1.5 not fired (Hou-Luo year drift
  cosmetic only). **Status: partially-fired-once**, content
  integrity preserved.
- **F-SEED-B (heterogeneity claim)**: partially weakened by D1.5
  §8.3 (n=6 discretization on swirl magnitude is numerology
  overlay, not group-theory); the underlying SO(2) reduction
  remains genuine. **Status: partially weakened, not fired**.
- **F-SEED-C (Q2-pre-emption)**: not active (Q2 still unexecuted;
  D1.4 PASS does not pre-empt Q2).
- **F-SEED-D (D1.1 priority risk)**: pre-audit was deferred; D1.4
  remains live. **Status: conditionally active**.
- **F-SEED-E (atlas drift between seed and validation)**: not
  fired across any D1 execution.

### §10.3 Bias-hypothesis level

- **F-CATALOGUE_BIAS-N (failure-mode heterogeneity)**: not fired;
  4 distinct failure modes across D1 verdicts (FAIL_VACUOUS,
  FAIL_NO_DUALITY+RELABELING, FAIL_NO_LITERATURE_PATH, plus PASS).
- **F-CATALOGUE_BIAS-V (vacuous-primitive narrow hypothesis)**:
  fired by D1.3 (FAILed by structural-non-existence, not
  vacuous-primitive); the narrow hypothesis is **falsified**.
- **F-DSCRM-CONFLATE (parent meta-audit: type-axis confounded with
  candidate-axis)**: active and reinforced by D1 outcomes.
- **F-GENPIPE-EXISTENCE (newly active per D1.5 §9)**:
  "discriminator authored without pre-verifying existence of its
  target value". Active. Feeds in-flight #51.

### §10.4 Tier-closure level (new)

- **F-D1-TIER-CLOSURE-PREMATURE** (new, registered at this
  synthesis): if a future session re-promotes D1.2 and obtains a
  D1.2 PASS, the tier-closure declaration in §8 is premature. The
  defense: §9 records the rationale for not promoting D1.2; the
  closure is not a permanent retirement of D1.2 but a closure of
  the *current dispatch batch* on the tier. **Conditionally
  inactive**: would fire only on a future D1.2 PASS.

---

## §11 Closing

**0/7 unchanged. NS regularity status open. No atlas/state/inventory
edits.**

The BT-544 D1 atlas-scan tier is formally closed as of 2026-04-25.
D1 tier execution: 4 of 5 candidates validated (D1.2 unexecuted by
seed §5 design). Verdict tally: 1 PASS (D1.4 arithmetic-identity)
/ 3 FAIL (D1.1 vacuous, D1.3 no-duality, D1.5 no-literature-path).
Family heterogeneity verified across 4 distinct primitive sources
(info-theory, cross-PDE, arithmetic-numerology, group-theory +
numerology overlay), with the D1.5 numerology-overlay caveat
logged. Failure-mode taxonomy: 3 distinct FAIL labels plus 1 PASS
label, all 4 distinct.

Cross-cell pattern post-D1: D1.4 (PASS) is the structurally-grounded
arithmetic-identity that confirms the meta-audit's CONFOUNDED
verdict (discrete-equality discriminators can PASS with strong
candidates). D1.3 (FAIL) added the first cross-PDE structural FAIL
to the catalogue. The three D1 FAILs (D1.1, D1.3, D1.5) cluster on
the sub-pattern "discriminator embeds an unverified existence
assumption", with D1.3 additionally exhibiting the secondary
"reduces to relabeling" mode.

Cross-tier comparison places EXT-tier as the highest-priority next
batch (no prior verdicts; generation-pipeline diagnostic targets
its candidate authoring). Recommended dispatch order: (1) EXT-B
CI-Lyap validation (candidate generated), (2) EXT-A NS
gradient-flow (#58 in-flight), (3) R1 Lemma 1 retry-3 BCV-2021
(#59 in-flight), (4) close R5 program (already D3_CONFIRMED at
s*=2).

D1 tier closed. No atlas/state/inventory edits. No new claims; only
synthesis of existing verdicts plus tier-level inference about next
priority.

-- end tier closure synthesis --
