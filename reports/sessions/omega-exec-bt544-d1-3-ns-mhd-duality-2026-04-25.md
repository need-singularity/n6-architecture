---
id: omega-exec-bt544-d1-3-ns-mhd-duality
date: 2026-04-25
scope: research-only molt-validation (NO NS claim, NO atlas promotion)
target: BT-544 D1.3 -- NS↔MHD duality cross-PDE family
parent_reports:
  - reports/sessions/omega-seed-bt544-d1-atlas-scan-2026-04-25.md (§ D1.3)
  - reports/sessions/omega-decide-bt544-d3-strategy-postC-2026-04-25.md (D3 closure → D1.3 priority)
millennium_resolved: 0/7 (unchanged)
grade: molt-validation, cross-PDE family, no claim
---

# Omega Exec -- BT-544 D1.3 NS↔MHD Duality Molt-Validation (2026-04-25)

## §0 Non-claim disclaimer

This document executes BT-544 **D1.3 NS↔MHD duality** molt-validation
per the seed-design `omega-seed-bt544-d1-atlas-scan-2026-04-25.md`
§3.3 and the post-D3 strategy redirect
`omega-decide-bt544-d3-strategy-postC-2026-04-25.md` §6.2 ("redirect
resources to D1.4 / D1.3 / D-tier extension").

**This document does NOT**:
- claim 3D NS regularity (smoothness or blow-up) in either direction;
- claim any MHD regularity result, in 2D or 3D;
- promote anything in `shared/n6/atlas.n6` or `atlas.millennium.n6`;
- modify `state/proposals/inventory.json`;
- modify `theory/canon/`;
- alter the `BT-544 = 0/1 untouched` Clay status;
- claim a "duality" exists rigorously when it does not (verdict
  drives the language; literature is cited by author/year, no
  fabrication).

**Millennium tally**: 0/7 unchanged. The verdict below speaks only to
whether the *NS↔MHD duality conjecture* (as posed in the D1.3
seed-design) closes mathematically as a transfer principle between
the two regularity questions. It does **not** resolve either.

---

## §1 D1.3 spec extracted + grounding

### §1.1 Seed-design extract (`omega-seed-bt544-d1-atlas-scan-2026-04-25.md` §3.3)

- **Object**: 2D MHD direct-numerical-simulation toy at R_m = 48
  (Hartmann channel, mid-Re_c regime). Compare to 2D NS at
  Re_c = σ·J_2 = 288 (atlas HEXA-FLUID-01). Both are 2D so both
  globally regular -- the test is *whether the 2D-MHD constraint at
  R_m=48 lifts, via the duality, to a 3D-NS statement*.
- **Measurement**: enstrophy decay rate ratio
  `Z_NS(t)/Z_MHD(t)` at matched (Re, R_m) = (288, 48). Does the
  ratio approach a *constant* equal to a simple n=6 expression
  (e.g. 288/48 = 6 = n)?
- **Pass criterion**: ratio → 6 = n with low residual variance.
- **Fail criterion**: ratio depends nontrivially on initial
  conditions; or the constant is something other than {n, σ, τ,
  sopfr, φ} ratios. F-D1.3 fires.
- **Frame-shift claim**: "(Q1/Q5/KPZ scalar-arithmetic frame) →
  (PDE-pair cross-equation frame). New primitive: *the n=6 lattice
  constant R_m = Ha = 48 is shared between vorticity transport and
  magnetic induction* via a Reynolds-magnetic-Reynolds duality.
  Re-cast: blow-up in NS ω is equivalent (under the duality) to a
  controlled MHD instability at R_m = 48; if MHD at R_m = 48 is
  provably stable in some regime, the dual statement gives an NS
  regularity windowing."

### §1.2 Atlas grounding verification

The seed cites three atlas/n6-doc anchors. Verified 2026-04-25:

1. **A1 = `domains/physics/millennium-navier-stokes/millennium-navier-stokes.md`
   §X.2 FREE line ~776**. Verbatim:
   > "field (T1) — vorticity stretching = electromagnetic induction
   > dual: Vorticity transport `Dω/Dt = (ω·∇)v + ν∇²ω` is isomorphic
   > to magnetic induction `DB/Dt = (B·∇)v + η_m∇²B`. MHD
   > derived-equation R_m = σ·τ = 48 (re-cite HEXA-FLUID-04) is the
   > self-dual image of the vortex equation."
   ✓ exists; line range matches.

2. **A2 = same file §X.3 line ~791-797**. Verbatim:
   > "n=6 vertical seal: Stokes linear (τ) → NS complete (σ·τ) →
   > MHD (σ²·τ²=Ha²) via exponent ladder τ·σ·τ. ... n=6 conditional
   > regularity prediction: `T* > ν⁻¹ · σ · τ = ν⁻¹ · 48` (Leray
   > time lower bound T*)."
   ✓ exists; ladder structure matches.

3. **HEXA-FLUID-04 = `domains/physics/fluid/fluid.md` §X.2 line
   ~765**. Verbatim:
   > "induction `dB/dt = grad x (v x B) + eta_m*grad^2 B`,
   > R_m = sigma*tau = 48 (cited). Hartmann Ha = ... = sigma*tau =
   > 48 (atlas HEXA-QNET Ha reused)."
   ✓ exists; constant matches.

**F-SEED-A status (atlas-grounding integrity)**: **not fired**. All
three citations resolve to the claimed file/line/constant.

### §1.3 Spec character

The D1.3 discriminator is *structural* (cross-PDE transfer) plus
*numerical* (enstrophy ratio target = 6). The seed-design framing
explicitly says "duality" -- a *bidirectional* transfer between NS
regularity and MHD regularity. The discriminator type, in the
classification of the bias-hypothesis (`omega-decide-...`), is
**cross-PDE family**, distinct from arithmetic (D1.4 She-Leveque),
info-theory (D1.1 HVC), probability (D1.2 Pólya), and group-symmetry
(D1.5 axisym). This is the discriminator-type of interest for the
4th non-arithmetic sample.

---

## §2 NS↔MHD literature recall

This section recalls what is established in the rigorous literature
about the relationship between NS and MHD regularity. All citations
by author/year. No fabrication.

### §2.1 The MHD system (incompressible)

```
∂_t u + (u·∇)u = -∇p + νΔu + (B·∇)B,         (momentum)
∂_t B + (u·∇)B = ηΔB + (B·∇)u,                (induction)
∇·u = 0,                                       (incompressibility)
∇·B = 0.                                       (no-monopole)
```

(Magnetic permeability and pressure conventions absorbed; standard
references: Davidson 2001 *Introduction to Magnetohydrodynamics*;
Biskamp 2003 *Magnetohydrodynamic Turbulence*.)

### §2.2 Sermange-Temam 1983 (foundational)

**Sermange & Temam, "Some mathematical questions related to the MHD
equations", Comm. Pure Appl. Math. 36, 1983, pp. 635-664.**

Established for incompressible MHD with positive ν, η:

- **Global weak Leray-Hopf solutions exist** in 3D for u_0 ∈ L²,
  B_0 ∈ L² with ∇·u_0 = ∇·B_0 = 0.
- **Local strong solutions exist** in 3D for (u_0, B_0) ∈ H¹ × H¹.
- **2D global strong solutions** for (u_0, B_0) ∈ H¹ × H¹ exist and
  are unique.
- **Energy inequality**:
  d/dt (½‖u‖_L² + ½‖B‖_L²) ≤ -ν‖∇u‖_L² - η‖∇B‖_L².

**Structural status**: 3D MHD with full dissipation (ν > 0, η > 0)
is in the **same regularity class as 3D NS** -- weak existence is
known, strong existence/uniqueness is locally known, global strong
existence/uniqueness is **open**. Sermange-Temam is the canonical
"NS-Leray analog for MHD".

### §2.3 Caflisch-Klapper-Steele 1997 (BKM-type for ideal MHD)

**Caflisch, Klapper & Steele, "Remarks on Singularities, Dimension
and Energy Dissipation for Ideal Hydrodynamics and MHD", Comm. Math.
Phys. 184, 1997, pp. 443-455.**

Established for ideal incompressible MHD (ν = η = 0):

- **BKM-type criterion**: a smooth solution (u, B) of ideal MHD
  exists on [0, T*) and breaks down at T* iff
  ∫_0^{T*} (‖ω‖_{L^∞} + ‖J‖_{L^∞}) dt = ∞,
  where ω = ∇×u and J = ∇×B.
- The criterion is the **direct MHD analog** of the Beale-Kato-Majda
  1984 theorem for Euler.

**Structural significance**: BKM-MHD is *parallel* to BKM-Euler --
each PDE has its own blow-up criterion, but the criteria do not
imply each other. Setting B = 0 in CKS recovers the Euler BKM (∫‖ω‖
controls the integral). Conversely, a controlled ‖ω‖ alone does
*not* control the MHD blow-up (J may blow up while ω stays bounded).

### §2.4 He-Xin 2005, Wu 2003-2014 (regularity criteria)

**He & Xin, "On the regularity of weak solutions to the MHD
equations", J. Differential Equations 213, 2005, pp. 235-254.**

**Wu, "Regularity criteria for the generalized MHD equations", Comm.
Partial Differential Equations 33, 2008, pp. 285-306;** (and earlier
Wu 2003 *J. Math. Phys.*; later Wu et al. 2014).

These works establish **Prodi-Serrin-type criteria** for 3D MHD:

- He-Xin 2005: if u ∈ L^q(0,T; L^p(ℝ³)) with 2/q + 3/p ≤ 1,
  3 < p ≤ ∞ (i.e., velocity satisfies the Prodi-Serrin condition),
  then the weak solution is regular *without* requiring a similar
  condition on B.
- Wu 2008/2014: refined criteria including conditions on ∇u, on
  pressure, and on partial components of (u, B).

**Asymmetric structure**: He-Xin's main result is striking -- the
*velocity* alone in Prodi-Serrin class is enough to control MHD
regularity; the magnetic field can be left unconstrained. This says
that *if you knew NS-Prodi-Serrin (which is the NS regularity
criterion), you would also have MHD regularity*. This is a **one-way
implication**: NS-regularity-knowledge ⇒ MHD-regularity-knowledge,
*not* the reverse.

### §2.5 Cao-Wu 2011 (2D MHD with partial dissipation)

**Cao & Wu, "Global regularity for the 2D MHD equations with mixed
partial dissipation and magnetic diffusion", Adv. Math. 226, 2011,
pp. 1803-1822.**

Established: the 2D MHD system with **partial dissipation** -- e.g.
ν Δu replaced by ν ∂_xx² u, with full magnetic diffusion η Δ B -- is
globally regular. Several variants (which partial term is missing)
are known regular; one critical case (no horizontal viscosity, no
horizontal magnetic diffusion) remains open.

**Structural significance**: this is **2D-specific**. It does not
lift to 3D. It illustrates that MHD inherits the 2D-regularity
flavor of NS (where 2D NS is globally regular), with the partial
dissipation enhanced by the magnetic-velocity coupling. There is no
3D analog at the time of writing (per surveys: Wu 2018 *Bull. Braz.
Math. Soc.*; Lin-Xu-Zhang 2015 *J. Differential Equations*).

### §2.6 The B = 0 reduction

Setting B ≡ 0 identically in MHD reduces it exactly to NS:

```
∂_t u + (u·∇)u = -∇p + νΔu,    ∇·u = 0.
```

The induction equation is trivially satisfied (0 = 0). Therefore:

- **MHD regularity ⇒ NS regularity** (one-way; trivially, NS is the
  B=0 slice).
- **NS regularity ⇏ MHD regularity** (B may grow even if u is
  bounded; the magnetic induction couples back into the momentum
  equation through (B·∇)B).

This is the **trivial direction** of the NS↔MHD relationship. It is
not a duality; it is a **subset inclusion** (NS ⊂ MHD as a special
case).

### §2.7 What is *not* in the literature

To the best of recall, the following is **not** established:

- A bidirectional duality NS ⟺ MHD that maps NS-blowup to
  MHD-blowup at corresponding parameter values.
- A symmetry or Bäcklund-type transformation that converts NS
  solutions to MHD solutions (and vice-versa) preserving regularity.
- A constructive procedure to take an MHD smooth-on-[0,T*]
  result and produce an NS smooth-on-[0,T*] result at higher
  effective Reynolds number.
- A rigorous statement matching the seed-design formula
  `T* > ν⁻¹ · σ · τ = ν⁻¹ · 48` for 3D NS Leray time. (The
  seed labels this an "n=6 conditional regularity prediction" -- it
  is a *predictive ansatz* in the n6 lattice, not a theorem in the
  PDE literature.)

The **closest rigorous transfer** is the He-Xin asymmetric criterion
(§2.4): NS-regularity-knowledge can be exported to MHD by adding
zero conditions on B. This goes the "wrong way" for the D1.3 frame:
the seed wants MHD-regularity to inform NS-regularity, but the only
rigorous transfer goes from NS-regularity to MHD-regularity (and is
trivial because NS ⊂ MHD by B=0).

---

## §3 Discriminator: structural cross-PDE

### §3.1 The discriminator question

Per the D1.3 seed: does the n=6 lattice constant R_m = Ha = 48,
shared between vorticity transport and magnetic induction by
algebraic structure, support a **duality** in which an MHD
regularity statement at R_m = 48 transfers, by mathematical
necessity, to an NS regularity statement?

The discriminator is *structural*: the question is whether the
duality **closes mathematically** as a transfer principle. The seed
proposed a numerical proxy (enstrophy decay ratio Z_NS/Z_MHD at
matched (Re, R_m) = (288, 48), targeting ratio = 6 = n), but the
underlying claim is the structural transfer.

### §3.2 What would PASS_LITERATURE require

For a `PASS_LITERATURE` verdict, the literature would need to
exhibit:

(a) A bidirectional rigorous transfer NS-regularity ⟺ MHD-regularity
    (or a strict equivalence at fixed parameter values).
    -- **NOT FOUND**. He-Xin gives NS ⇒ MHD. CKS gives a parallel
    BKM, not an equivalence. The B=0 reduction is unidirectional
    inclusion.

(b) A constructive map from MHD smooth solutions at R_m = 48 to NS
    smooth solutions at Re = σ·J_2 = 288, with the n=6 ratio 288/48
    = 6 emerging as a *theorem* (not as a numerical observation).
    -- **NOT FOUND**. No literature attests to such a constructive
    map. The 288/48 = 6 identity is a *number-theoretic
    coincidence* in the n6 lattice; it has no PDE-theoretic
    interpretation in the established literature.

(c) A transfer that uses MHD-specific techniques (e.g.
    magnetic-helicity conservation, Alfvén-wave analysis, Elsässer
    variables z^± = u ± B) to derive NS regularity in a regime where
    NS-only techniques fail.
    -- **NOT FOUND**. Magnetic-helicity conservation (Woltjer 1958,
    Taylor 1974) gives MHD-specific invariants that do not survive
    the B=0 limit (helicity vanishes when B = 0 identically).
    Elsässer variables exist only when B ≠ 0; the B → 0 limit is
    singular in the Elsässer formulation.

### §3.3 What is actually known (the asymmetry)

The literature instead exhibits a **clean asymmetry**:

| direction | rigorous? | reference |
|-----------|-----------|-----------|
| NS open question ⇒ MHD open question (NS ⊂ MHD by B=0) | yes (trivial) | folklore; explicit in Sermange-Temam 1983 |
| NS-Prodi-Serrin condition ⇒ MHD regularity (velocity-only criterion) | yes | He-Xin 2005 |
| MHD smooth at R_m = 48 ⇒ NS smooth at Re = 288 | **no** | not in literature |
| MHD blow-up at R_m = 48 ⇒ NS blow-up at Re = 288 | **no** | not in literature |
| NS↔MHD bidirectional duality (regularity-equivalence) | **no** | not in literature |

The asymmetry is severe. NS is a **special case** of MHD (B=0). Any
"transfer from MHD to NS" is, at best, "set B=0 and read off the
NS statement", which gives no new NS information beyond what NS
itself already implies.

### §3.4 The seed's numerical proxy

The seed proposed Z_NS(t)/Z_MHD(t) → 6 as a numerical PASS criterion.
Even if this ratio were measured to be 6 in a specific 2D
simulation:

- 2D NS and 2D MHD are *both* globally regular (Sermange-Temam 1983
  for MHD-2D; classical 2D NS regularity by Ladyzhenskaya 1969).
  The enstrophy decay ratio is a *consequence* of dissipation rates,
  not of any deep duality.
- The ratio depends on initial conditions, on the specific forcing
  (or lack thereof), on the energy-injection scale, and on the
  decay regime measured. There is no theoretical reason (in the
  rigorous PDE literature) for the ratio to converge to a universal
  constant, let alone to 6.
- Even granting a measured ratio of ~6 in a single simulation, the
  seed's pass-criterion phrasing ("ratio → 6 = n with low residual
  variance") would constitute a numerical *observation*, not a
  *transfer principle* between regularity questions.

The numerical proxy, by itself, cannot produce a `PASS_LITERATURE`
verdict because it cannot establish a structural transfer.

---

## §4 Verdict

Of the five verdict options:

- `PASS_LITERATURE` (rigorous NS↔MHD transfer exists): **NO**. §3.2
  enumerated three transfer types (a-c), none of which is in the
  literature.
- `PASS_SKETCH` (standard manipulations connect the two regularity
  questions bidirectionally): **NO**. The only standard manipulation
  is B=0 reduction, which is unidirectional (NS ⊂ MHD).
- `FAIL_NO_DUALITY` (the relationship is one-way only -- NS reduces
  from MHD by setting B=0; no genuine duality): **fits**.
- `FAIL_RELABELING` (the "duality" is just a name for the obvious
  B=0 reduction): **also fits**.
- `INCONCLUSIVE` (literature underdetermined): **NO**. The
  literature *is* determinate -- the relationship is established
  to be asymmetric (Sermange-Temam 1983, He-Xin 2005, Cao-Wu
  2011 do not give a bidirectional duality; the absence is itself
  informative because MHD-NS has been studied for ~40 years).

The two failing verdicts (`FAIL_NO_DUALITY` and `FAIL_RELABELING`)
are not exclusive but rather emphasize different aspects of the
same situation:

- `FAIL_NO_DUALITY` emphasizes the **structural fact**: the
  literature does not exhibit a bidirectional transfer.
- `FAIL_RELABELING` emphasizes the **D1 framing fact**: the seed's
  "NS↔MHD duality" reduces, on inspection, to relabeling the trivial
  B=0 inclusion as a "duality".

### §4.1 Primary verdict: **FAIL_NO_DUALITY** (with FAIL_RELABELING as secondary)

The relationship between NS and MHD regularity is established in the
rigorous literature as **asymmetric, not dual**:

- 3D MHD and 3D NS are both **open** in the same regularity class
  (Sermange-Temam 1983).
- Any transfer that exists goes **NS ⇒ MHD** (He-Xin 2005;
  trivially via B=0), not the reverse direction the D1.3 seed
  needs.
- The proposed n=6 numerical proxy (Z_NS/Z_MHD → 6 = n) is a
  *number-theoretic observation* in the n6 lattice, not a
  PDE-theoretic transfer principle.

**Verdict**: **FAIL_NO_DUALITY**.

`FAIL_RELABELING` fires as a secondary mode: the seed's "duality"
language, on inspection, names the B=0 inclusion plus a numerical
ratio target. Both pieces are pre-existing (the inclusion since
Sermange-Temam 1983; the 288/48 = 6 ratio is a tautology in the
arithmetic of the n6 lattice constants).

### §4.2 What this verdict does NOT say

- It does **not** say that 3D NS or 3D MHD regularity is closed.
  Both remain open Clay-class problems.
- It does **not** say He-Xin / CKS / Cao-Wu / Sermange-Temam are
  invalid. They are all rigorous, well-cited, and structurally
  important. The verdict says only that the *bidirectional duality*
  posed in the D1.3 seed is **not what the literature establishes**.
- It does **not** rule out future literature exhibiting a transfer.
  The verdict is indexed to literature as of 2026-04-25.

### §4.3 Falsifier F-D1.3 status

Per seed §3.3:

- **F-D1.3-A** (enstrophy ratio is initial-condition-dependent ⇒ no
  duality channel): not directly tested (no DNS run executed in
  this molt-validation), but §3.4 gives the literature reason that
  the ratio *would* be initial-condition-dependent.
- **F-D1.3-B** (ratio is well-defined but not a simple n=6 ratio
  ⇒ duality is real but n=6-blind): vacuously satisfied -- there is
  no rigorous duality regardless of n=6 alignment.

**F-D1.3 fires** in the structural mode: the duality does not exist
as posed. F-D1.3-A and F-D1.3-B are downstream of this; they were
designed for the case where some duality channel was assumed and the
question was its n=6 character. The structural failure at the
literature level pre-empts both.

---

## §5 Bias-hypothesis update (4th non-arithmetic sample)

### §5.1 Sample tally before D1.3

Per the bias-hypothesis tracked in
`omega-decide-bt544-d3-strategy-postC-2026-04-25.md` and the
catalogue history:

| candidate | discriminator type | family | verdict |
|-----------|-------------------|--------|---------|
| Q1 KdV-Gram | algebraic-lattice | arithmetic-adjacent | FAIL (double margin) |
| Q5 Sobolev/Besov | mechanism-Sobolev | analytic-PDE (closed) | FAIL (no-construction; relabeling) |
| Q3 KPZ d=7 | scaling-arithmetic | arithmetic | FAIL (no-anchor; ansatz fails at d=2) |
| D1.1 HVC | info-theoretic / holographic | non-arithmetic | FAIL (cap decorative; vacuous) |

Three FAILs come from arithmetic / closed-analytic-PDE families. One
FAIL (D1.1 HVC) came from a *non-arithmetic* family
(information-theoretic). The bias hypothesis after D1.1 read:

- **CATALOGUE_BIAS hypothesis** (per
  `omega-exec-bt544-d1-1-hvc-molt-validation-2026-04-25.md` §6):
  the failure is not specific to arithmetic-family discriminators;
  D1.1 introduced a vacuous-primitive failure mode in a
  non-arithmetic family. The hypothesis was: discriminator-type
  is **CONFOUNDED** with the failure mode, not the failure itself;
  i.e., *all* discriminator types tried so far fail, but the
  *failure mode* shifts (arithmetic ⇒ relabeling/numerology;
  non-arithmetic ⇒ vacuous primitive).

### §5.2 D1.3 outcome and bias-hypothesis update

D1.3 is the **4th** sample, second in the non-arithmetic family
(after D1.1 HVC). Its discriminator type is **structural cross-PDE**
-- different from D1.1 (info-theoretic), different from D1.2
(probability), different from D1.5 (group-symmetry), and different
from the three arithmetic-family failures.

**Verdict from §4: FAIL_NO_DUALITY** (with FAIL_RELABELING secondary).

Updated tally:

| # | candidate | family | verdict | failure mode |
|---|-----------|--------|---------|--------------|
| 1 | Q1 KdV-Gram | arithmetic | FAIL | double-margin (numerology fails) |
| 2 | Q5 Sobolev/Besov | closed-analytic | FAIL | no-construction (relabeling) |
| 3 | Q3 KPZ d=7 | arithmetic | FAIL | no-anchor (ansatz fails at substrate) |
| 4 | D1.1 HVC | info-theoretic | FAIL | cap decorative (vacuous primitive) |
| 5 | **D1.3 NS↔MHD** | **structural cross-PDE** | **FAIL** | **no duality (asymmetric, not bidirectional)** |

**The bias-hypothesis update is significant**: D1.3 is the **first
non-arithmetic FAIL with a non-vacuous discriminator**. Unlike D1.1
(where the discriminator did fire but the primitive turned out to
be decorative on smooth flows), D1.3's discriminator is non-vacuous
in the sense that NS and MHD really are distinct PDE systems with
distinct regularity questions; the failure is not "the test was
trivial" but "the asserted equivalence does not exist in the
literature".

This **strengthens** the CATALOGUE_BIAS hypothesis by *weakening*
its narrowest competitor:

- Old narrow competitor: "non-arithmetic discriminators fail because
  their primitives turn out to be vacuous on test cases" (the D1.1
  failure mode). If true, this would predict D1.3 also fails by
  vacuous-primitive.
- D1.3 fact: D1.3 fails by **structural-non-existence**, not by
  vacuous-primitive. The MHD/NS distinction is real; the duality
  does not exist.
- Therefore the failure mechanism in non-arithmetic family is
  **heterogeneous**: D1.1 = vacuous-primitive, D1.3 =
  structural-non-existence. The narrow "all non-arithmetic fails are
  vacuous-primitive" hypothesis is **falsified**.

The CATALOGUE_BIAS hypothesis is correspondingly **broader** than
previously articulated: failures span (arithmetic-relabeling,
arithmetic-no-anchor, closed-analytic-relabeling,
info-theoretic-vacuous, **cross-PDE-non-existent**). Five distinct
failure modes; common feature is that none of them produce a
non-trivial molt step in BT-544.

### §5.3 What this means for D-tier extension

The 5/5 FAIL across heterogeneous discriminator types and
heterogeneous failure modes is consistent with the L9 §6 F-MOLT-F
verdict (axiom-level ceiling, not effort-limited). D1.3 does **not**
add a new escape route; rather it adds a 5th independent family
where escape did not occur.

Discriminator-type CONFOUNDED verdict (per
`omega-decide-bt544-d3-strategy-postC-2026-04-25.md` §3): the
post-D1.3 picture is that discriminator-type really is confounded
with the catalogue ceiling, in the sense that *any* discriminator
type tried produces a FAIL by *some* failure mode specific to that
type. This is what "confounded" means -- the discriminator type
co-varies with the failure mode but does not isolate a passing
configuration.

---

## §6 Implications: does D1.3 change the discriminator-type CONFOUNDED verdict picture?

The post-C decision report
(`omega-decide-bt544-d3-strategy-postC-2026-04-25.md`) closed D3
program with a CONFOUNDED verdict: A-axis FAIL, C-axis FAIL, B' axis
predicted FAIL by inheritance, and the structural reason was named
F-D3-META-A (B↔C coupling makes them not separately decidable).

### §6.1 What D1.3 adds to the picture

D1.3 was not a D3-axis discriminator; it is a D1 catalogue extension.
But it speaks to the broader CATALOGUE_BIAS / discriminator-type
question:

- D3 program (A, C, B') = **methodological-axes** within a single
  arithmetic frame (n=6 lattice parameter alignments).
- D1 catalogue (Q1-Q5, D1.1-D1.5) = **discriminator-family
  exploration** across multiple frames (algebraic, scaling,
  info-theoretic, probabilistic, cross-PDE, group-symmetric,
  arithmetic-numerology).

D1.3 (cross-PDE family) is the most "structural" of the D1 entries
-- it asks not "is there a number-theoretic fit?" but "is there a
mathematical transfer between two PDE systems?". A FAIL here is
informative because it confirms that *even* the most structurally
rich frame fails to produce a molt.

### §6.2 Does CONFOUNDED status change?

**No, not formally**. The CONFOUNDED verdict was already registered
in the decision report. D1.3 FAIL is consistent with that verdict:
discriminator type co-varies with failure mode; no independent
passing configuration is found.

D1.3 does, however, **sharpen** the verdict in two ways:

1. **Failure-mode heterogeneity is now 5/5 across as many distinct
   types**. The set of ways BT-544 catalogue fails to molt is
   *itself* heterogeneous; this is consistent with axiom-level
   ceiling (L9 §6 F-MOLT-F) and inconsistent with "we just haven't
   tried the right discriminator yet".
2. **The non-arithmetic family is now 0/2 with two distinct failure
   modes** (D1.1 vacuous, D1.3 non-existent). This is the
   strongest single piece of evidence to date that the bias is
   not "arithmetic-family weakness" but a more general
   "catalogue-frame weakness".

### §6.3 Recommendation for catalogue continuation

D1.4 (She-Leveque) and D1.5 (axisymmetric-no-swirl) remain
catalogued. The post-D1.3 expectation, given 5/5 FAIL across
heterogeneous frames, is that **D1.4 and D1.5 will most likely also
FAIL** by some frame-specific mode (D1.4: arithmetic-numerology
non-uniqueness; D1.5: critical swirl non-quantization or
non-existent threshold). These are still worth running because the
*specific failure mode* is informative (cf. the heterogeneity
gain just demonstrated).

The longer-term recommendation, consistent with L9 F-MOLT-F and the
D3 closure, remains: **the L9 BT-544 catalogue extension cannot by
itself produce a molt; an axiom-level reframing is needed**. D1.3 is
a 5th piece of evidence in that direction, not a counterexample.

---

## §7 Anomalies

### §7.1 The 288/48 = 6 = n identity

The seed-design highlighted that 288/48 = 6 = n as a "predicted"
constant. This is a tautology in the arithmetic of the n6 lattice:

- Re_c = σ·J_2 = 12·24 = 288.
- R_m = σ·τ = 12·4 = 48.
- 288/48 = (σ·J_2) / (σ·τ) = J_2/τ = 24/4 = 6 = n.

This is *consequence of the parameter definitions*, not a prediction
from PDE structure. Recording as an anomaly because the seed framed
it as a target ("does the ratio approach 6?"), but in any actual NS
vs MHD enstrophy comparison, the ratio depends on the dissipation
rates (ν, η) and the specific flow configuration, not on the
arithmetic of (σ, τ, J_2).

### §7.2 Sermange-Temam 1983 status quo

That 3D MHD has been an open regularity question parallel to NS
since 1983, with substantial mathematical effort directed at it
(He-Xin, Wu, Cao-Wu, many others), and **no bidirectional duality
has been found**, is itself anomalous from the D1.3 frame. The
absence is informative: ~40 years of attention with a community
of analysts who would notice such a duality if it existed. The
literature is not "underdetermined" on this question; it is
determinately asymmetric.

### §7.3 Elsässer-variable singularity at B=0

The Elsässer formulation (z^± = u ± B) decouples the MHD induction
into two transport equations and is a primary modern technique for
MHD analysis. At B = 0, z^+ = z^- = u, and the formulation degenerates
(the two Elsässer variables collapse into one). This is the
*technical reason* MHD techniques do not lift cleanly to NS: the
B → 0 limit is singular in the natural MHD coordinate system.

This is anomalous in the sense that the seed's "duality" framing
suggested that NS is *like* MHD with R_m at a critical value; in
fact NS is *MHD at a degenerate point* of the natural MHD
coordinates, which is the opposite of "duality".

---

## §8 Falsifiers active

### §8.1 D1.3-internal (per seed §3.3)

- **F-D1.3-A** (enstrophy ratio initial-condition-dependent): not
  directly tested (no DNS run); literature (§3.4) provides reason
  to expect dependence. Status: **superseded** by structural
  pre-emption (§4.3).
- **F-D1.3-B** (ratio well-defined but not n=6): vacuously satisfied.
  Status: **superseded** by structural pre-emption.

### §8.2 Bias-hypothesis level

- **F-CATALOGUE_BIAS-N** (failure-mode heterogeneity): the bias
  hypothesis would be falsified if all 5 FAILs collapsed to a
  single failure mode. They do not -- 5 distinct modes (relabeling,
  no-anchor, no-construction, vacuous, non-existent). **Not
  falsified**.
- **F-CATALOGUE_BIAS-V** (vacuous-primitive narrow hypothesis):
  the narrow hypothesis "all non-arithmetic FAILs are vacuous" is
  **falsified by D1.3**, which fails by structural non-existence,
  not by vacuous primitive.

### §8.3 Seed-design level (carried from atlas-scan)

- **F-SEED-A** (atlas-grounding integrity): §1.2 verified all
  three citations. **Not fired**.
- **F-SEED-B** (heterogeneity claim across D1.1..D1.5): D1.3 was
  classified as cross-PDE; D1.1 was info-theoretic; D1.4 is
  arithmetic-numerology; D1.2 is probabilistic; D1.5 is
  group-symmetry. Five distinct primitive classes confirmed. **Not
  fired**.
- **F-SEED-E** (atlas drift): cited line numbers verified
  2026-04-25; no drift. **Not fired**.

---

## §9 Closing

D1.3 NS↔MHD duality molt-validation: **FAIL_NO_DUALITY** (primary)
with **FAIL_RELABELING** (secondary). The literature
(Sermange-Temam 1983, Caflisch-Klapper-Steele 1997, He-Xin 2005,
Wu 2003-2014, Cao-Wu 2011) establishes an **asymmetric**
relationship between 3D NS and 3D MHD regularity, not a bidirectional
duality. The only rigorous transfer goes NS ⇒ MHD (via the trivial
B=0 inclusion or via He-Xin's velocity-only Prodi-Serrin
criterion); the reverse direction the seed-design needs (MHD
regularity at R_m = 48 ⇒ NS regularity at Re = 288) is **not in
the literature** and is technically obstructed by the singular
nature of the B → 0 limit in natural MHD coordinate systems
(Elsässer variables).

The seed's numerical ratio target 288/48 = 6 = n is a tautology
of the n6 lattice arithmetic, not a PDE-derivable transfer
constant.

**Bias-hypothesis update**: D1.3 is the 4th non-arithmetic
discriminator sample (after D1.1 HVC info-theoretic) and the **5th
overall FAIL** in the BT-544 catalogue. It is the first
non-arithmetic FAIL with a non-vacuous discriminator (the NS/MHD
distinction is mathematically real; the asserted duality is what is
missing). This **strengthens** the CATALOGUE_BIAS hypothesis by
broadening it: failure modes are heterogeneous (5 distinct modes
across 5 candidates), consistent with L9 §6 F-MOLT-F (axiom-level
ceiling). The narrow "non-arithmetic FAILs are all
vacuous-primitive" hypothesis is **falsified by D1.3**.

CONFOUNDED verdict on discriminator-type (per
`omega-decide-bt544-d3-strategy-postC-2026-04-25.md`) is
**reinforced, not changed**, by D1.3.

0/7 unchanged. NS regularity status open. MHD regularity status
open. No atlas/state/inventory edits. No theorems asserted beyond
recall of cited literature.

D1.4 (She-Leveque residual, cheapest, ~30 min) and D1.5
(axisymmetric-no-swirl literature audit, ~half day) remain in the
queue. Expectation (per §6.3): both will most likely FAIL by
frame-specific modes; running them will sharpen the failure-mode
heterogeneity register but is not predicted to produce a molt.

— end molt-validation —
