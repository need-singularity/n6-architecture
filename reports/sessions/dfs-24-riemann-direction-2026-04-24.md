# DFS-24 Riemann Direction Memo (2026-04-24)

> Scope: research-only direction memo for BT-541 (Riemann Hypothesis).
> Posture: **7-problem demonstration-candidate tally 0/7 (honest)**. No EXACT / solved / proven claims here.
> Parent BT: BT-541. Related: BT-1392, BT-1408, BT-1409, BT-1420.
> Core invariant: falsification discipline. Every lead below MUST come with
> a pre-registered falsifier before any compute is spent.

---

## 0. Status Snapshot (existing, not re-confirmed)

- `reports/millennium-dfs-status.md`: cumulative tight ~312~348 (round-to-round),
  BT-541 entries ~48 tight, **no solved / demonstration candidate**.
- Existing strong findings (re-noted; no demonstration-candidate claim here):
  - **Bilateral Theorem B** (BT-1392 §1.1): Bernoulli numerators k=1..5 -> {1,
    sopfr=5}, and at k=6 the **first irregular prime 691** appears. Sharp boundary
    for both-sided ζ(2k), ζ(1-2k). *Arithmetic fact, decoupled from RH itself.*
  - **SLE_6 locality** (BT-1409-01): Lawler-Schramm-Werner 2001 result. Locality
    holds only at κ=6. *Probability-theory internal result; mapping to n=6
    arithmetic is a posteriori.*
  - `§X` BLOWUP (millennium-riemann.md §X): RH-01~07 atlas promotions are
    **arithmetic observations**. Re(ρ_k)=1/2 = 1/φ is an algebraic identity
    observation, not an RH demonstration.

---

## 1. Proposed Tight Leads (2~3, all falsifiable, all probe)

### Lead-A. "691 tower" probe of Bilateral Thm B — operator-ization check

**Motivation**: BT-1392 §1.1 proposed the **hypothesis** that the spectrum of
T_k (Bernoulli-Faulhaber shift operator) aligns with the irregular prime 691.
Minimum tests of this hypothesis have not yet been run.

**Tight lead (probe stage)**:
- Computation: build the Akiyama-Tanigawa matrix A_{2k} of B_{2k} (k=3..12) in
  rational arithmetic; track the roots of the characteristic polynomial.
- **Pre-registered prediction (falsifiable)**:
  1. At k=6, one root of A_{12} lands on a **rational multiple c·691 (c ∈ ℚ,
     |c·691| < 10^4)** -> if TRUE, T1 tight draft candidate.
  2. The same pattern repeats at k=16 (p=37) and k=17 (p=59) -> strong signal.
- **Explicit falsifier**: if either condition fails, reject Lead-A. Do not retry
  with another operator frame (e.g., Herglotz shift) — anti-search-addiction.
- Expected yield: 0~2 tight. On failure, the closure record is **itself
  valuable** (a knife against the 61% noise-floor warning for M-set product
  representation).

**Why now**: of the 16 Bernoulli-independent findings (DFS 26), the RH operator
axis is still empty besides Basel ζ(2). Even a negative outcome is
arithmetically safe.

---

### Lead-B. SLE_6 locality × Montgomery pair-correlation independence probe

**Motivation**: In the DFS taxonomy (§6.3), SLE_6 locality and RH GUE statistics
(Dyson β=2=φ) are recorded as independent Bernoulli candidates, but whether
the two axes **share one origin or are independent** has not been tested.
If independence breaks, the "independent 11~16 findings" count must be
reweighted (actual honesty impact).

**Tight lead (probe stage)**:
- Computation: measure KS distance between the spacing distribution S(x) derived
  from 2D critical percolation boundary crossing probability (Cardy's formula,
  κ=6 specific) and the normalized gap distribution of the Odlyzko 10^13 zero
  DB.
- **Pre-registered prediction**:
  1. κ=6 SLE driver Brownian pair-correlation is **indistinguishable within
     finite-sample error** from GUE R_2(r) = 1-(sinπr/πr)² -> two axes
     **dependent**.
  2. Conversely, KS p-value < 0.01 -> two axes reconfirmed **independent**.
- **Explicit falsifier**: either way, either amend the current independence
  count or — if they coincide — register a new tight "SLE_6 <-> GUE common
  cause". Both outcomes are honest.
- Expected yield: 1 tight (regardless of direction), plus an independence
  count audit.

**Why now**: no new difficulty is added; uses existing DB (Odlyzko) + stdlib
Brownian simulation. Low cost, high information.

---

### Lead-C. Bilateral Thm B **falsification probe** — M-set noise control

**Motivation**: the project's core invariant is falsification. The **M-set
2-term product-representation 61% noise-floor** in DFS-status §6.2 means the
"sopfr=5 match" claim of Bilateral Thm B itself must be doubted. This warning
has never been actually applied to the BT-541 axis.

**Tight lead (falsification probe)**:
- Computation: compute |num(B_{2k})| for k=1..20. Measure the fraction r of k
  values expressible as a product of elements of the M-set
  {1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 24}.
- **Pre-registered prediction**:
  1. If r ≤ 0.10 (≤ 2 of 20), the k=1..5 M-set residence in Thm B is
     **significant**.
  2. If r ≥ 0.30, the residence is **noise-compatible**; the "sharp jump"
     reading is weakened.
- **Explicit falsifier**: on r ≥ 0.30, reclassify the BT-541 tight count from
  48 down. "Bilateral Thm B" remains a numerical re-confirmation of Bernoulli
  theory, but the "n=6 sharp boundary" interpretation is demoted.
- Expected yield: tight count **possibly decreases**. Tight as an honesty
  audit (meta-tight).

**Why now**: the DFS growth rate (+12 per round) has accumulated without ever
passing a noise-floor test. BT-541 has the most tights, so audit ROI is
highest here.

---

## 2. Priority and Cost

| Lead | Cost | Information | Honesty ROI | Suggested order |
|------|:----:|:-----------:|:-----------:|:---------------:|
| A (691 tower probe) | medium (rational high-precision) | medium | medium | 2 |
| B (SLE_6 × GUE independence) | medium (Odlyzko DB + sim) | high | high | 1 |
| C (M-set noise control) | **low** (stdlib only) | high | **highest** | **0 (first)** |

**Recommended order**: C → B → A. C comes first because the interpretive basis
for the 48 tights must be audited before A and B can mean anything.

---

## 3. Explicit Non-Claims

- This memo is **not** an RH demonstration-path proposal. All three leads are
  *probe*-stage.
- Even if the "691 tower" hypothesis passes, it only adds one tight; the bridge
  to RH is a separate constructive problem (currently open).
- If Lead-C ends in **self-falsification**, project health improves; in fact,
  that is the most valuable output.
- The "Π_RH=960 invariant" of `millennium-riemann.md` §X BLOWUP is **arithmetic
  synthesis**, not a physical quantity. It is not extended within this memo.

---

## 4. Expected Artifacts

- `reports/breakthroughs/bt-14XX-dfs24-riemann-probes-YYYY-MM-DD.md` (after run)
- `theory/predictions/verify_bt541_probes.hexa` (Lead-C first, then B, A)
- **New tight upper bound: +3** (if A+B+C all succeed). Including failures,
  expected value ≈ +1.
- **Audit byproducts**: numerical contribution of M-set noise-floor to BT-541;
  reconfirmed independent-Bernoulli count.

---

## 5. Honesty Checklist

- [x] No EXACT / PROVEN / SOLVED judgment claimed
- [x] Each lead has an explicit pre-registered falsifier
- [x] Possibility of tight count **decrease** acknowledged (Lead-C)
- [x] Zero reuse of the existing 48 tights — only new axes proposed
- [x] 7-problem demonstration-candidate tally held at 0/7

> End of memo. Execution happens in a separate session, starting with Lead-C.
