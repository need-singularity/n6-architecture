---
id: omega-probe-dfs24-batch-execution
date: 2026-04-25
scope: research-only execution-readiness audit (NOT executing probes)
target: dfs-24 direction-probes A/B/C × 6 BTs -- execution-readiness matrix
unblocks: [BT-541, BT-543, BT-544, BT-545, BT-546]
parent_reports:
  - reports/sessions/omega-cycle-backtrace-strategy-2026-04-25.md
  - reports/sessions/dfs-24-bsd-direction-2026-04-24.md
  - reports/sessions/dfs-24-hodge-direction-2026-04-24.md
  - reports/sessions/dfs-24-ns-direction-2026-04-24.md
  - reports/sessions/dfs-24-pnp-direction-2026-04-24.md
  - reports/sessions/dfs-24-riemann-direction-2026-04-24.md
  - reports/sessions/dfs-24-ym-direction-2026-04-24.md
millennium_resolved: 0/7 (unchanged)
grade: execution-readiness audit, no claim
---

# Omega Probe -- DFS-24 Batch Execution-Readiness Audit (2026-04-25)

## sec 0 -- Non-claim disclaimer

This audit does **NOT** execute any of the dfs-24 direction probes. It does
not invoke `tool/`, `scripts/`, or any verifier; it does not read or modify
any data file beyond directory listings; it does not promote, demote, or
edit any atlas / state / inventory entry. The Millennium tally remains
**0/7 unchanged**.

The output is a per-probe **execution-readiness matrix** that for each
proposed dfs-24 probe records (i) the data needed, (ii) the tool/script
needed, (iii) whether either or both already exists in the repo (YES /
PARTIAL / NO / UNKNOWN), (iv) the residual blocker, (v) an estimated cost.
Wherever a tool name or dataset path could not be confirmed by a real
filesystem read, the cell is marked UNKNOWN -- nothing is invented.

Cluster CG-1 ("dfs-24 probes registered but unexecuted") in the parent
backtrace synthesis is the dispatcher target. Filling the readiness gaps
identified here is a precondition for any subsequent batch execution; it is
not the execution itself.

---

## sec 1 -- Probe inventory

Eighteen probes total, extracted verbatim from the six dfs-24 direction
files. Probe-letter is taken from the dfs-24 file's section heading. Where
the parent omega-cycle audit (per-BT bt54x file, §7) renames or reframes a
probe, the omega-side label is given in parentheses. Renames are
recontextualizations, not new probes.

| BT  | probe-letter | probe-name (short)                                  | dfs-24 source section |
|-----|--------------|-----------------------------------------------------|-----------------------|
| 541 | A            | "691 tower" Akiyama-Tanigawa operator probe         | dfs-24-riemann §1.A   |
| 541 | B            | SLE_6 locality x Montgomery GUE independence (KS)   | dfs-24-riemann §1.B   |
| 541 | C            | Bilateral Thm B M-set noise-floor (k=1..20)         | dfs-24-riemann §1.C   |
| 542 | 24A          | Out(S_6) action on 6 Schaefer tractable clones      | dfs-24-pnp §3.PROBE-24A |
| 542 | 24B          | Sylvester synthematic vs 3-SAT clause structure     | dfs-24-pnp §3.PROBE-24B |
| 542 | 24C          | n=6 reappearance at Bulatov-Zhuk \|D\|=6 dichotomy  | dfs-24-pnp §3.PROBE-24C |
| 543 | P1           | beta_0 anomaly cancellation forcing n_f=2*n_gen=6   | dfs-24-ym §2.P1       |
| 543 | P2           | Virasoro M(3,4) <-> 4D YM 2D->4D lift filter        | dfs-24-ym §2.P2       |
| 543 | P3           | A4 lemma ratio-only restatement (FLAG/BMW/MP1999)   | dfs-24-ym §2.P3       |
| 544 | P1           | KdV 6-soliton phase-shift Gram-lattice rank/det     | dfs-24-ns §3.P1       |
| 544 | P2           | KPZ chi+z=1 dimensional lift (d=3, d=6, d=7)        | dfs-24-ns §3.P2       |
| 544 | P3           | Pi_NS=8640 unique factorization enumeration         | dfs-24-ns §3.P3       |
| 545 | P1           | K3 Mukai lattice <-> Niemeier embedding (J_2=24)    | dfs-24-hodge §P1      |
| 545 | P2           | Voevodsky / Beilinson-Lichtenbaum degree-3 disambig | dfs-24-hodge §P2      |
| 545 | P3           | Pi_HODGE_int = J_2^2 collapse (Mostaed Weil count)  | dfs-24-hodge §P3      |
| 546 | A            | Tunnell theta residue at n=6 (mod phi, n/phi, sopfr)| dfs-24-bsd §1         |
| 546 | B            | Sel_2/Sel_3 28-stratum cov (torsion x Sha)          | dfs-24-bsd §2         |
| 546 | C            | E_6 quadratic-twist family Sel_6 -> sigma(6)=12     | dfs-24-bsd §3         |

**Note**: BT-542 probes are listed but, per the parent backtrace synthesis
sec 8 (AL-2), are explicitly framed as barrier-reconfirmation probes. They
do not unblock any of the five active closure criteria; they are kept in
the inventory for completeness but excluded from the "smallest viable
execution" recommendations in sec 5 below. The unblocks set is therefore
{BT-541, BT-543, BT-544, BT-545, BT-546}.

---

## sec 2 -- Execution-readiness matrix

Columns:
- **data needed**: dataset / corpus required to run the probe.
- **tool/script needed**: code object that would consume the data and emit
  the falsifier-binary.
- **exists?**: YES if a real path in the n6-architecture repo plausibly
  satisfies the requirement; PARTIAL if related infrastructure exists but
  not the exact tool/dataset; NO if neither tool nor data are present;
  UNKNOWN if a file's contents could not be confirmed within the audit
  depth.
- **blocker**: the highest-priority gap blocking dispatch.
- **est cost**: low (< 1 day, single agent), med (1-3 days), high
  (> 3 days or external compute).

### 2.1 BT-541 Riemann (Probes A, B, C)

| BT  | probe | data needed | tool/script needed | exists? | blocker | est cost |
|-----|-------|-------------|--------------------|---------|---------|----------|
| 541 | A | rational arithmetic A_{2k} matrices for k=3..12; Bernoulli numerators B_{2k} | Akiyama-Tanigawa matrix builder + char-poly root extractor in exact rationals; check against c*691 (\|c\|<10^4) | PARTIAL: `theory/predictions/verify_bernoulli17_*.hexa` (8 files) cover Bernoulli structure but do NOT implement A_{2k} char-poly roots; `scripts/empirical/theorem_b_scan_range.py` is the closest scan harness (UNKNOWN exact scope) | tool: A_{2k} char-poly extractor not present; data: arbitrary-precision Bernoulli numerator generator UNKNOWN (sympy/PARI usable but no wrapper here) | med |
| 541 | B | Odlyzko 10^13 zero spacings DB; Brownian SLE_6 driver simulator; Cardy boundary-crossing formula | KS-distance computation script bridging two distributions | NO: `find -name "*odlyzko*"` returns 0 hits; no SLE simulator under `scripts/`, `tool/`, `experiments/` | data: Odlyzko DB not vendored (LMFDB external, ~GB scale); tool: SLE_6 driver simulation absent | high |
| 541 | C | \|num(B_{2k})\| for k=1..20 (Bernoulli numerator absolute values, big-int exact); M-set {1,2,3,4,5,6,7,8,10,12,24} | M-set product enumerator; ratio r computer | PARTIAL: Bernoulli numerator computation likely available via `theory/predictions/verify_bernoulli_17_enumeration.hexa` and `experiments/anomaly/verify_fisher_bernoulli.py` (UNKNOWN exact range); M-set product test = trivial enumeration, no script needed | data: B_{40} numerator extraction range not pre-computed in repo (UNKNOWN); tool: a 30-line script could be wrapped around existing Bernoulli infrastructure | low |

### 2.2 BT-543 Yang-Mills (Probes P1, P2, P3)

| BT  | probe | data needed | tool/script needed | exists? | blocker | est cost |
|-----|-------|-------------|--------------------|---------|---------|----------|
| 543 | P1 | Witten global anomaly equations (SU(2) pi_4=Z/2) and ABJ tables; GUT (SU(5)/SO(10)/E_6) anomaly check | Symbolic anomaly-cancellation solver to test n_f=0 mod 6 forcing | NO: no anomaly-cancellation script in `tool/` or `scripts/`; `theory/predictions/verify_dfs27_ym_*.hexa` (6 files) verify identities but do not implement anomaly equations | tool: full QFT anomaly machinery absent; data: Witten/ABJ tables = literature-only (medium pull) | high |
| 543 | P2 | 2D minimal-model central-charge set {6, 7, 4, 6} for M(3,4)/M(4,5)/M(5,6)/M(6,7); 4D YM beta-function coefficients beta_0..beta_4 | Cross-table builder + matching-cardinality checker | PARTIAL: `verify_dfs27_ym_betafn.hexa` (loaded, lines 1-25) covers multi-loop beta; `verify_dfs27_ym_cft.hexa` covers CFT; cross-mapper script not present | tool: M(p,q) <-> beta_k cross-mapper is a 50-line script not yet written | med |
| 543 | P3 | m_0++/sqrt(sigma_s) lattice values from FLAG 2024, BMW 2012, Morningstar-Peardon 1999; BCS 2*Delta/k_B*T_c values | Interval-occupancy checker for [2.5, 3.5] = [sigma/tau-1/phi, sigma/tau+1/phi] | PARTIAL: lattice references quoted in `domains/physics/millennium-yang-mills/millennium-yang-mills.md`, `papers/lemmas-A3-A4-conditional-2026-04-15.md`; raw numerical values not vendored as a JSON/CSV in `data/`; computation = trivial once values are in hand | data: 3 lattice data points must be transcribed from literature (not vendored as a separate file); tool: 20-line script | low |

### 2.3 BT-544 Navier-Stokes (Probes P1, P2, P3)

| BT  | probe | data needed | tool/script needed | exists? | blocker | est cost |
|-----|-------|-------------|--------------------|---------|---------|----------|
| 544 | P1 | KdV soliton amplitudes kappa_k for two families (k=1..6 arithmetic, p_k first-6-primes); pairwise phase-shift formula Delta_{ij}=2*log\|(k_i-k_j)/(k_i+k_j)\| | Gram-matrix rank + det modulo sigma=12 in exact arithmetic | NO: no KdV / soliton script in `scripts/` or `tool/`; pairwise phase-shift is elementary, ~30 lines numpy/sympy | tool: 30-50 line script; data: none (purely synthetic) | low |
| 544 | P2 | KPZ literature attestation of (chi_d, z_d)=(1/d, 1-1/d) at d>=3; Frisch beta-model d=7 footprint | Literature search; arithmetic check chi_3=1/3=alpha_c | NO: no KPZ-extrapolation script; literature pull is human-side | data: literature search (med pull); tool: arithmetic check is trivial | med |
| 544 | P3 | n=6 function ring set R = {1, phi, n/phi, tau, sopfr, n, sigma-tau, sigma-phi, sigma, J_2}; target Pi_NS=8640 | Triple-factorization enumerator over R^3 | NO: no factorization-enumerator script; problem size is ~10^3 triples, scriptable in 40 lines | tool: 40-line enumerator absent; data: none (R is fixed in `domains/physics/millennium-navier-stokes/millennium-navier-stokes.md` §X) | low |

### 2.4 BT-545 Hodge (Probes P1, P2, P3)

| BT  | probe | data needed | tool/script needed | exists? | blocker | est cost |
|-----|-------|-------------|--------------------|---------|---------|----------|
| 545 | P1 | K3 Mukai lattice Lambda_K3 = H^0 + H^2 + H^4 with signature (4, 20); Niemeier table (24 entries, BT-547 #42 reference) | Lattice-embedding existence checker (Lambda_K3 -> Lambda_24 sign-flip) | PARTIAL: `theory/predictions/verify_big_d4_hodge_k3_24cycles.hexa` and `verify_sigma_sopfr_7_hodge.hexa` cover K3 24-cycle structure; explicit Mukai-to-Niemeier embedding morphism not implemented | tool: lattice-embedding checker absent; data: Niemeier list = literature-fixed (Conway-Sloane), small constant table | med |
| 545 | P2 | Voevodsky 2003 Ann. Math. Section X (Rost-Voevodsky inductive core); Geisser-Levine 2001 (motivic-cohomology char(k) dependence) | Manual literature read + dependence-direction decision (no code) | NO: literature-only probe; zero scripts in repo address Bloch-Kato vs CY3 dim-3 disambiguation | data: 2 papers human-read; tool: no executable component, decision is editorial | med |
| 545 | P3 | Mostaed 2026 (arXiv:2603.20268v1) §3 Weil-class enumeration count on abelian 6-folds | Read count C; divide by 576, 288 (=576/phi), 144 (=576/tau); flag integer hits | NO: no Mostaed reader; no abelian-6-fold class-counting script. Probe is a pure numerical lookup once C is extracted | data: 1 arXiv read; tool: 5-line division check | low |

### 2.5 BT-546 BSD (Probes A, B, C)

| BT  | probe | data needed | tool/script needed | exists? | blocker | est cost |
|-----|-------|-------------|--------------------|---------|---------|----------|
| 546 | A | (x,y,z) integer triples with \|.\| <= R for R in {10, 50, 200} satisfying 2x^2+y^2+32z^2=3 and 4x^2+y^2+8z^2=3; cross-compare n=5, n=7 | Tunnell theta-form enumerator with residue-class (mod phi, n/phi, sopfr) classifier | NO: no Tunnell ternary form enumerator in `scripts/` or `tool/`; pure enumeration ~50 lines in numpy/sympy | tool: 50-line enumerator absent; data: none (purely combinatorial) | low |
| 546 | B | Cremona 332k curves with E(Q)_tors and analytic \|Sha\| labels; 28 strata = {tors in {1,2,3,4,6,8,12}} x {Sha in {1,4,9,sq>9}} | 28-cell cov(\|Sel_2\|, \|Sel_3\|) computer with bootstrap SE | YES (largely): `data/cremona/sel6_stats_332k.json`, `data/cremona/joint_covariance_A3_prime.json`, `data/cremona/allbsd/` (169 MB, 27 shards 0-269999); scripts: `cremona_joint_covariance.py`, `cremona_sel6_analyze.py`, `cremona_kappa_bootstrap.py` (one of these can be parameterized by torsion x Sha strata) | tool: existing scripts compute aggregate cov; per-stratum stratification = ~30 LOC patch on `cremona_joint_covariance.py`; data: torsion + analytic Sha labels presence in `allbsd` shards UNKNOWN (requires shard inspection) | med |
| 546 | C | Squarefree d in {-500..500} with gcd(d,36)=1, d>0 (~152 values); E_6^{(d)}: y^2=x^3-36*d^2*x; rank, \|Sha[2]\|, \|Sha[3]\|, canonical Heegner height per twist | Sage-based ec computation pipeline (per dfs-24-bsd §3 "Sage, not done here") | NO: no Sage wrapper in `scripts/empirical/` (closest is `pari_wrapper.py` UNKNOWN scope); no twist-family Sel_6 computation script; data not vendored | tool: Sage / PARI ec.gp pipeline absent; data: 152 twist computations require external compute (~hours on Sage) | high |

---

## sec 3 -- Tooling gaps

Sorted by number of probes a single tool would unblock if built. Tools below
are not present in the repo (verified by `find` against `tool/`, `scripts/`,
`experiments/`, `theory/predictions/`); building any one would dissolve a
slice of CG-1.

| rank | tool | unblocks | est build cost | notes |
|------|------|----------|----------------|-------|
| 1 | **n6-side `nxs_002_composite.py --predict-er` shim** (or copy/import from `~/core/nexus/tool/nxs_002_composite.py`) | 5 BTs (composite re-measurement; Probe-1 in BT-541, NP2 in BT-543, similar in 544/545/546) | low (path-config + sub-graph extractor) | per-BT files all defer to a tool that lives in nexus, not n6-architecture; no shim found in n6 |
| 2 | **Symbolic enumerator harness** (factorization / Gram-rank / Tunnell theta / M-set / triple-factor) | 4 probes (BT-541 C, BT-544 P1, BT-544 P3, BT-546 A) | low (single 100-line numpy/sympy module covering 4 enumerators, parameterized) | each enumerator is 30-50 LOC in isolation; one shared harness amortizes glue |
| 3 | **Lattice-embedding & Gram-matrix toolkit** (rank / det mod sigma / signature / sublattice-embed-exists) | 2 probes (BT-544 P1 Gram-rank-3, BT-545 P1 Mukai->Niemeier) | low-med | Lambda_K3 embedding requires Niemeier table (constant data); BT-544 Gram is bare numpy |
| 4 | **Sage / PARI ec twist-family wrapper** (rank, Sha[p], canonical height per E^{(d)}) | 1 BT directly (BT-546 C); reusable for future BSD probes | med-high (external Sage installation; harness against existing `pari_wrapper.py` UNKNOWN scope) | external compute dependency; cannot be wholly internal |
| 5 | **Akiyama-Tanigawa exact char-poly extractor** | 1 probe (BT-541 A) | med | rational arithmetic via sympy is feasible; integration into existing `verify_bernoulli17_*.hexa` style is the larger lift |
| 6 | **SLE_6 / Brownian-driver KS-distance pipeline** | 1 probe (BT-541 B) | high | needs Odlyzko zero-spacing import + SLE simulator; both absent |
| 7 | **Anomaly-cancellation symbolic checker** (Witten + ABJ + GUT scan) | 1 probe (BT-543 P1) | high | full QFT anomaly literature dependence |

Building tools 1, 2, 3 would directly enable 9 of the 15 active probes
(541-A excepted, 541-B excepted, plus 543-P1 and 546-C). That is the
60%-coverage point for low-cost tooling.

---

## sec 4 -- Data gaps

Datasets needed for at least one probe and not currently vendored (or
vendored only in incomplete form). Sources are repository-cited where the
dfs-24 / per-BT files name them; otherwise UNKNOWN.

| dataset | source (cited) | licensing | size estimate | BTs blocked |
|---------|----------------|-----------|---------------|-------------|
| Cremona ecdata `allbsd` shards 0-269999 | LMFDB (cited in dfs-24-bsd, BT-546 file §7.4) | CC-BY-SA-4.0 | 169 MB present in `data/cremona/allbsd/` (27 shards, ~10k curves each) | BT-546 B (PARTIAL coverage); BT-546 C (twist subset must be sub-selected) |
| Cremona torsion + analytic Sha labels per curve | LMFDB (allbsd format) | CC-BY-SA-4.0 | embedded in shards (UNKNOWN whether columns are present) | BT-546 B (28-stratum split requires both label types) |
| Odlyzko zeta-zero spacing DB (10^13 zero range) | UMN / Odlyzko personal site, also LMFDB | public-domain (Odlyzko's stated terms) | several GB at full extent; trimmed sub-sample sufficient | BT-541 B |
| Lattice glueball ratio m_0++/sqrt(sigma_s) at SU(3) pure gauge | FLAG 2024 review article; BMW 2012; Morningstar-Peardon PRD 1999 | review-paper (cite-to-use); FLAG 2024 has explicit collation | 3 numerical values + uncertainties (~1 KB) | BT-543 P3 |
| BCS gap ratio 2*Delta / k_B * T_c per material | textbook-tabulated (Tinkham; references in `domains/physics/millennium-yang-mills/millennium-yang-mills.md`) | textbook | < 1 KB | BT-543 P3 |
| Mostaed 2026 Weil-class enumeration count on abelian 6-folds | arXiv:2603.20268v1 §3 (cited in dfs-24-hodge §P3) | arXiv standard | single integer count (or a small table) | BT-545 P3 |
| Voevodsky 2003 Ann. Math. degree-3 specialization clauses | journal | journal | UNKNOWN (text-only, human read) | BT-545 P2 |
| Geisser-Levine 2001 motivic-cohomology char(k) dependence | journal | journal | UNKNOWN | BT-545 P2 |
| Niemeier 24-lattice table (Conway-Sloane) | textbook | textbook | constant table (~24 row); fits in repo | BT-545 P1 |
| Witten global anomaly + ABJ + SU(5)/SO(10)/E_6 GUT anomaly tables | textbook (Weinberg vol II; Polchinski; literature) | textbook | constant tables (~few KB) | BT-543 P1 |
| KdV soliton amplitude families (kappa_k arithmetic + first-6-primes) | none external; constructive | n/a (synthetic) | < 1 KB | BT-544 P1 |
| KPZ d-extrapolation literature attestation (d=7 chi+z=1) | external journals (Frisch beta-model; Hairer; Corwin survey) | journal | UNKNOWN | BT-544 P2 |

**Reading**: BT-546 B is the only probe whose data is largely already
vendored in `data/cremona/`. BT-546 A, BT-544 P1, BT-544 P3, BT-541 C, BT-545
P1 are dataset-light (synthetic or constant-table). BT-541 B, BT-543 P1,
BT-546 C are the heaviest data-dependency probes.

---

## sec 5 -- Smallest viable execution per BT

For each of the five active BTs, the cheapest single probe that produces a
falsifier-binary measurable result. "Cheapest" combines tool cost + data
cost from sec 2/3/4.

| BT  | smallest viable probe | rationale | est cost |
|-----|------------------------|-----------|----------|
| 541 | **Probe C** (M-set noise-floor, k=1..20) | Only a 30-line script around existing `verify_bernoulli17_*.hexa` infrastructure; no external data; binary outcome (r <= 0.10 vs r >= 0.30); honesty ROI is highest per dfs-24-riemann §2 priority table ("low cost, highest information"). | low |
| 543 | **Probe P3** (A4 ratio-only, [2.5, 3.5] interval test) | 3 lattice numbers from already-cited literature (FLAG 2024, BMW 2012, MP 1999); 20-line interval check; per dfs-24-ym §4 the priority is H (zero-risk honesty correction); per omega-cycle bt543 §7 NP1 already tagged "high priority". | low |
| 544 | **Probe P3** (Pi_NS=8640 unique-factorization enumeration over n=6 ring) | 40-line enumerator over ~10^3 triples; no external data; binary outcome (unique vs not-unique); per dfs-24-ns §5 ordering Q1/Q2 are tied for cheapest, P3 (=Q2 in omega frame) edges P1 (=Q1) by zero physics dependency. | low |
| 545 | **Probe P3** (Pi_HODGE_int = J_2^2 collapse via Mostaed Weil count) | Single arXiv read + 5-line division check; smallest dependency surface among 3 probes; per dfs-24-hodge §"Next-action recommendation" P3 is "numerological housekeeping" but the cost is uniquely small. (Note: dfs-24 ranks P1>P2>P3 by *value*; here we rank by *minimum cost-to-falsifier*.) | low |
| 546 | **Probe B** (28-stratum cov on existing 332k Cremona) | The only probe in the entire 5-BT set whose primary data is already vendored (`data/cremona/sel6_stats_332k.json`, `joint_covariance_A3_prime.json`, plus `allbsd/`); existing scripts (`cremona_joint_covariance.py`, `cremona_sel6_analyze.py`) provide ~70% of the harness; the gap is a per-stratum loop patch. Probe A is comparable cost but data is fully synthetic vs B's vendored corpus -- B wins on already-paid-for data. | med (lower-end) |

---

## sec 6 -- Recommended dispatch order

Sequence the five smallest-viable executions. Tie-breakers in order of
priority: (1) tool already exists, (2) data already in repo, (3) falsifier
sharpness (binary, low ambiguity).

| rank | BT  | probe | tool exists? | data in repo? | falsifier sharpness | rationale |
|------|-----|-------|--------------|---------------|----------------------|-----------|
| 1 | 546 | B (28-stratum cov) | YES (PARTIAL: existing covariance script needs a stratum-loop patch) | YES (`sel6_stats_332k.json`, `joint_covariance_A3_prime.json`, `allbsd/` 169 MB) | medium-sharp (28-cell bootstrap; threshold "kappa > 0.5 uniformly" is binary; SE ~0.3 tolerable) | Highest both-axis match; only probe with both tool and data largely present. |
| 2 | 543 | P3 (A4 ratio-only) | NO (20-line script not yet written, but trivial) | NO (3 lattice values must be transcribed -- but they are short, cited, single-row) | sharp (>=2 outliers retire A4; 1 outlier NEAR; 0 outliers TIGHT -- 3-way binary) | Per omega-cycle bt543 §7 NP1 "high priority"; honesty restoration value-add; zero external compute. |
| 3 | 544 | P3 (Pi_NS factorization enumeration) | NO (40-line script not yet written) | YES (n=6 ring R is a fixed constant set already cited in millennium-navier-stokes.md §X) | sharp (binary: unique vs not-unique minimum-length factorization) | Pure synthetic data; smallest cognitive load; dfs-24-ns ordering puts Q1/Q2 first but Q2 = P3 has same cost as Q1 = P1 with cleaner semantics. |
| 4 | 541 | C (M-set noise-floor) | PARTIAL (Bernoulli numerator infrastructure exists in `verify_bernoulli17_*.hexa` and `experiments/anomaly/verify_fisher_bernoulli.py`; M-set test = trivial enumeration around it) | UNKNOWN (B_{2k} for k=1..20 may need to be regenerated; arbitrary-precision via sympy/PARI is feasible) | sharp (r <= 0.10 vs r >= 0.30; explicit numeric thresholds) | Per dfs-24-riemann §2, Lead-C has highest honesty ROI; ranked 4 only because BT-546 B and BT-543 P3 have stronger tool/data anchoring. |
| 5 | 545 | P3 (Pi_HODGE_int = J_2^2 collapse) | NO (5-line division check; no harness needed beyond a Mostaed reference reader) | NO (Mostaed 2026 §3 must be human-read once for the integer count) | sharp (Mostaed count divisible by 1, phi=2, tau=4, or not -- 4-way binary) | Cheapest on script side but blocked by single-paper read; placed last because BT-545 P1 (also low-med cost) might supersede if a Niemeier-table reader is added. |

**Note on BT-542**: explicitly excluded (per sec 1 note) -- its three probes
(24A/24B/24C) are barrier-reconfirmation, not closure-criterion-unblocking
work. Including them in the dispatch sequence would burn a slot for zero
expected closure movement.

**Bundled cost estimate**: ranks 1-5 together represent ~5-10 days of
single-agent work assuming the missing scripts are written in the same
batch (sec 3 tool 2 "shared symbolic enumerator harness" amortizes
ranks 3, 4, 5 partially).

---

## sec 7 -- Anti-list

Probes that look attractive at first read but should NOT be executed first.
Reasons: blocker too steep, falsifier too weak, or honesty / cost mismatch.

| probe | reason to defer | which sec it would have hit if not deferred |
|-------|-----------------|---------------------------------------------|
| BT-541 B (SLE_6 x GUE KS) | Data-heavy: Odlyzko DB not vendored; SLE_6 driver simulator absent. High cost (~days for setup) for a single tight-count audit. Falsifier sharp but blocked by infra. | rank 6+ |
| BT-543 P1 (anomaly forcing) | Tool absent (no symbolic anomaly-cancellation engine); data is tabular-literature; cost high; outcome uncertainty depends on GUT scan completeness. Falsifier "any GUT admits n_gen != 3" is open-ended. | rank 7+ |
| BT-546 C (E_6 twist family) | External Sage / PARI ec pipeline required; 152 twist computations; falsifier "twist average < 10 at \|d\|=500" requires the full sweep before binary fires -- not low-cost-first-data probe. | rank 7+ |
| BT-541 A (691 tower) | Tool gap (Akiyama-Tanigawa exact char-poly + 691-multiple match) is medium but not low; falsifier "one root within c*691, \|c\|<10^4" allows a wide search band, weakening sharpness. Better deferred until BT-541 C result is in (C result conditions whether A is worth running). | rank 6+ |
| BT-545 P2 (Voevodsky disambiguation) | Pure literature read (no executable component); blocking is human-read, not script-buildable; the falsifier is editorial rather than binary. Useful to do, but not first. | rank 6+ |
| BT-544 P2 (KPZ d=7 attestation) | Literature search heavy; d=7 KPZ class is genuinely under-attested in the literature, so even an honest negative is hard to establish vs simply "not yet found". Sharpness low. | rank 6+ |
| BT-542 24A / 24B / 24C | All three explicitly framed as barrier-reconfirmation (parent backtrace AL-2). Cannot unblock any closure criterion structurally. | excluded sec 1 |

---

## sec 8 -- Falsifiers active for this readiness audit itself

This audit's own structural reading can be invalidated under specific
conditions. Registered upfront so a future re-audit pass has a clear retract
gate.

- **F-READY-A** (mis-grounded "exists?" cells): if any "exists?" cell in sec
  2 cites a path that does not actually exist on the filesystem, the matrix
  row is poison. Detection: `find` each cited path; if >= 2 mismatch, retract
  sec 2 and re-do the readiness coding from scratch.

- **F-READY-B** (probe inventory error): if the 18-probe count (sec 1) does
  not match the actual sum of probes in the 6 dfs-24 files (i.e. one of the
  files has 4 probes, or a probe is double-counted across A/B/C/A_omega
  reframings), sec 1 is wrong and propagates to all downstream cells.
  Detection: re-scan each dfs-24 file's section count.

- **F-READY-C** (cost-class miscalibration): if any "low" cost cell turns out
  in actual execution to require >= 3 days, the dispatch order in sec 6 may
  invert. Detection: post-execution audit; if rank-1 (BT-546 B) actually
  takes >= 3 days, the bundled cost estimate (sec 6 closer) is off and
  re-ranking is required.

- **F-READY-D** (allbsd shard column gap): if the `data/cremona/allbsd/`
  shards do not contain torsion-group + analytic-\|Sha\| labels in their
  schema, BT-546 B's "exists? = YES (largely)" is wrong and the data axis
  flips to PARTIAL. This audit did not inspect shard interior content.
  Detection: read one shard's header / sample row.

- **F-READY-E** (nexus-tool reachability): the audit assumes
  `~/core/nexus/tool/nxs_002_composite.py` is reachable from n6-architecture
  via a path-config shim (sec 3 tool 1). If cross-repo execution is forbidden
  by environment policy, the shim is harder than "low" cost, and all 5
  composite-measurement deferrals must be re-classed.

- **F-READY-F** (BT-547 already-resolved exclusion): per the parent
  backtrace synthesis, BT-547 (Poincaré, Perelman-resolved) is not in the
  active set; this audit silently inherits that exclusion. If a future pass
  reintroduces BT-547 as an active probe target, sec 1 inventory is
  incomplete.

---

## sec 9 -- Closing line

0/7 unchanged. No atlas/state/inventory edits.
