# N6 Plasma Physics Hypotheses -- Independent Verification

Verified: 2026-03-30
Method: Each hypothesis checked against published plasma physics data, ITER design documents, and established textbook physics.

## Grade Distribution (Summary)

| Grade | Count | Pct | Hypotheses |
|-------|-------|-----|------------|
| EXACT | 5 | 25% | H-PP-1, H-PP-10, H-PP-12, H-PP-15, H-PP-16 |
| CLOSE | 5 | 25% | H-PP-3, H-PP-6, H-PP-7, H-PP-9, H-PP-11 |
| WEAK | 5 | 25% | H-PP-2, H-PP-8, H-PP-13, H-PP-19, H-PP-20 |
| FAIL | 4 | 20% | H-PP-4, H-PP-5, H-PP-17, H-PP-18 |
| UNVERIFIABLE | 1 | 5% | H-PP-14 (general stellarator pattern) |

**Non-failing total: 10/20 (50%)**

| ID | Hypothesis | Grade |
|----|-----------|-------|
| H-PP-1 | 4 matter states = tau(6) | **EXACT** |
| H-PP-2 | 6 MHD equations = n | **WEAK** |
| H-PP-3 | 3 confinement modes = phi+1 | **CLOSE** |
| H-PP-4 | Debye length Egyptian fractions | **FAIL** |
| H-PP-5 | Plasma frequency sigma=12 | **FAIL** |
| H-PP-6 | KSTAR 300s / Q=10 = n+tau | **CLOSE** |
| H-PP-7 | Tokamak geometry (q, A, delta) | **CLOSE** |
| H-PP-8 | Plasma beta values | **WEAK** |
| H-PP-9 | Lawson criterion 14 keV = sigma+phi | **CLOSE** |
| H-PP-10 | 4 MHD instabilities = tau(6) | **EXACT** |
| H-PP-11 | ITER parameters (mixed) | **CLOSE** |
| H-PP-12 | 3 heating methods = n/phi | **EXACT** |
| H-PP-13 | Divertor heat flux Egyptian | **WEAK** |
| H-PP-14 | W7-X 5 field periods = sopfr | **CLOSE** |
| H-PP-15 | D+T mass number mapping | **EXACT** |
| H-PP-16 | PF coils = n = 6 | **EXACT** |
| H-PP-17 | Energy partition Egyptian | **FAIL** |
| H-PP-18 | Reconnection energy R(6)=1 | **FAIL** |
| H-PP-19 | 12 core diagnostics = sigma | **WEAK** |
| H-PP-20 | Fuel cycle 5 steps + Li-6 | **WEAK** |

---

Grading scale:
- **EXACT**: The claimed number/structure matches real-world data precisely, with a legitimate physical basis.
- **CLOSE**: Within ~10% of real values, or directionally correct.
- **WEAK**: Requires cherry-picking, flexible counting, or post-hoc rationalization.
- **FAIL**: Contradicted by real-world data or trivially true of any number.
- **UNVERIFIABLE**: Insufficient published data to confirm or deny.

---

## H-PP-1: Matter has tau(6)=4 States

**Grade: EXACT (but trivial)**

Plasma is indeed the 4th classical state of matter. This is textbook physics. However, this is a well-known fact that n=6 is being fitted to, not a prediction. Bose-Einstein condensates (1995), fermionic condensates, and quark-gluon plasma are additional states. The claim only holds if you restrict to "classical" states, which is a convenient boundary.

Deduction: The number 4 is not deep -- it reflects a pedagogical classification, not a fundamental constant.

---

## H-PP-2: MHD has n=6 Fundamental Equations

**Grade: WEAK**

Standard MHD textbooks (Freidberg, Goedbloed & Poedts) present ideal MHD as a system of 8 scalar equations in 8 unknowns (rho, v_x, v_y, v_z, B_x, B_y, B_z, p). The hypothesis counts "physical laws" rather than equations and arrives at 6 by grouping vector equations as single laws and including the equation of state. This counting is not standard. You could equally count 4 conservation laws + constraints, or 8 scalar equations, or 3 (mass, momentum, energy) + Maxwell subset. The number 6 is achievable only through a specific non-standard grouping.

---

## H-PP-3: Confinement Modes = phi(6)+1 = 3

**Grade: CLOSE**

L-mode and H-mode are universally recognized. I-mode (discovered at Alcator C-Mod, ~2010) is increasingly accepted but not yet a universally established "third mode" -- it is studied at a handful of devices. Furthermore, there are other recognized regimes: QH-mode (quiescent H-mode), Super H-mode, and various ELM-free regimes. Calling the count "exactly 3" requires choosing which regimes count as fundamental vs. variants.

The n=6 derivation phi(6)+mu(6)=3 is also ad hoc -- why add phi and mu specifically?

---

## H-PP-4: Debye Length Egyptian Fraction Structure

**Grade: FAIL**

The hypothesis claims Debye shielding splits as 1/2 (electrons) + 1/3 (D+) + 1/6 (T+). In reality, because m_e << m_i, the electron Debye length is far smaller than the ion Debye length. Electrons dominate shielding almost entirely (>99%). The ion contribution is negligible for shielding purposes. The Egyptian fraction decomposition has no physical basis here.

The document itself admits this: "WEAK -- electron contribution is ~99% dominant."

---

## H-PP-5: Plasma Frequency sigma(6)=12 Harmonics

**Grade: FAIL**

The hypothesis tries to connect sigma(6)=12 to toroidal mode numbers or cyclotron harmonics. Toroidal mode numbers are continuous integers with no physical cutoff at 12. ECRH uses 2nd harmonic for practical reasons (absorption efficiency at available magnetic fields), not because of any n=6 structure. The claim that "meaningful MHD modes number ~12" has no basis in stability theory -- the relevant mode numbers depend entirely on the specific equilibrium.

---

## H-PP-6: KSTAR 300s and Q=10

**Grade: Mixed -- Q=10 is CLOSE, 300s is FAIL**

- **Q=10 = n+tau**: ITER's Q=10 target is a real design goal. However, Q=10 was chosen as an engineering milestone (10x energy gain), not derived from any mathematical structure. Q=10 means "produces 10x more fusion power than input heating" -- it is a round number chosen for programmatic reasons. The match with n+tau=10 is a coincidence.
- **300s decomposition**: As the document honestly admits, any integer can be decomposed in terms of 6 and its arithmetic functions. 300 = 12*25 = sigma(6)*sopfr(6)^2 is numerology. KSTAR's 300s target was a progression from 30s and 100s, driven by engineering improvements, not mathematical structure.

---

## H-PP-7: Tokamak Geometry (q, A, delta)

**Grade: Sub-claims vary**

| Claim | Real Value | Grade | Notes |
|-------|-----------|-------|-------|
| q > phi(6)=2 (stability) | q_95 > 2 (Kruskal-Shafranov) | **EXACT** | The KS limit is genuinely q > 1 for internal kink, and q_edge > 2 for external kink. The q>2 condition is real physics. |
| q_0 = R(6) = 1 | Sawtooth onset at q=1 | **EXACT** | Sawteeth trigger when q_0 drops to 1. This is well-established. But R(6)=1 just means "one" -- mapping it to R(6) is trivial. |
| A = sigma/tau = 3 | ITER: 3.1, KSTAR: 3.6, JET: 2.4 | **CLOSE** | ITER is 3.1, close to 3. But KSTAR is 3.6, JET is 2.4, ASDEX-U is 3.1, DIII-D is 2.75. The spread is 2.4-3.6. Saying "~3" is within range but not a tight prediction. |
| kappa upper bound = phi=2 | ITER: 1.7-1.85, NSTX: 2.5+ | **FAIL** | Spherical tokamaks achieve kappa > 2.5. phi=2 is not an upper bound. |
| delta = 1/3 | ITER: 0.33 (lower), 0.49 (upper) | **CLOSE** | Lower triangularity is 0.33, but upper is 0.49. Cherry-picking the lower value. Negative triangularity designs use delta < 0. |

Overall grade for H-PP-7: **CLOSE** -- some genuine matches (q>2, q_0=1), some cherry-picked (delta), one fail (kappa).

---

## H-PP-8: Plasma Beta

**Grade: WEAK**

- beta_optimal = 1/6 = 16.7%: Most conventional tokamaks operate at beta < 5%. 16.7% is far above typical values. Only spherical tokamaks (NSTX, MAST) approach this.
- beta = 1/J_2 = 1/24 = 4.2%: Within the typical range (1-5%), but 4.2% is just one point in a broad range.
- Troyon beta_N ~ 3 vs actual ~2.8: Off by ~7%. Close but not exact, and 3 = n/phi is a common small integer.

The hypothesis tries multiple n=6 expressions until one fits approximately. This is curve-fitting.

---

## H-PP-9: Lawson Criterion -- 14 keV = sigma+phi

**Grade: CLOSE (for 14 keV), FAIL (for triple product)**

- **"Triple" = 3 variables**: Trivially true. It is called "triple product" because there are 3 quantities multiplied together. Mapping this to n/phi is meaningless.
- **D-T optimal temperature ~14 keV**: The D-T cross-section peaks around 13-15 keV (often cited as ~14 keV or ~15 keV depending on source). sigma(6)+phi(6)=14 matches this. This is a genuinely interesting numerical coincidence, but the peak depends on nuclear physics (Gamow peak), not number theory.
- **Leading coefficient 3**: The Lawson criterion threshold ~3*10^21 keV*s/m^3 -- the "3" is just the order-of-magnitude coefficient and varies by factor of 2 depending on assumptions.

---

## H-PP-10: 4 MHD Instability Classes = tau(6)

**Grade: EXACT**

The four major MHD instabilities -- kink, sausage, ballooning, and tearing -- are the standard textbook classification of macroscopic plasma instabilities. This grouping is widely used in fusion physics education and research (Freidberg, Wesson). While finer classifications exist (internal vs. external kink, neoclassical tearing modes, resistive wall modes, interchange modes), the four-class framework is the standard top-level taxonomy. tau(6)=4 matches exactly.

The extension to "6 total including kinetic instabilities" is more arbitrary and not graded here.

---

## H-PP-11: ITER Parameters

**Grade: PARTIAL -- honest assessment appreciated**

The document is commendably honest here. Checking each claim against ITER design reports:

| Claim | Verdict | Notes |
|-------|---------|-------|
| TF coils = sigma=12 | **FAIL** | ITER has 18 TF coils. This is a clear miss. 18 was chosen to minimize toroidal field ripple. |
| PF coils = n=6 | **EXACT** | ITER has 6 PF coils. Confirmed in ITER design documents. |
| CS modules = n=6 | **EXACT** | 6 CS modules. Confirmed. |
| Q = 10 = n+tau | **CLOSE** | Q=10 is the target, but it is a round engineering milestone. |
| R = 6.2m ~ n=6 | **CLOSE** | 6.2m, not 6.0m. 3% off. The major radius was optimized for performance/cost, not set to 6. |
| a = 2.0m = phi | **EXACT** | Minor radius is 2.0m. But 2 is a common engineering round number. |
| A = 3.1 ~ sigma/tau=3 | **CLOSE** | A = R/a = 6.2/2.0 = 3.1. Close to 3. |
| B_T = 5.3T ~ sopfr=5 | **CLOSE** | 5.3T, not 5.0T. 6% off. |
| 500 MW | **FAIL** | No n=6 connection. |
| 400s burn | **FAIL** | No n=6 connection. |

Match rate: 5 exact/close out of 10 key parameters = 50%. But several "exact" matches involve small integers (2, 6) that appear frequently in engineering designs. The document's own 43% assessment and honest acknowledgment of TF coil failure is appropriate.

---

## H-PP-12: 3 Heating Methods = n/phi

**Grade: EXACT (count), FAIL (Egyptian ratios)**

- **3 external heating methods**: NBI, ICRH, ECRH are indeed the three standard external heating methods. This is correct.
- **4 with Ohmic**: Adding Ohmic heating gives 4 total, matching tau(6). This is legitimate.
- **Egyptian fraction power split**: ITER design: NBI ~33 MW, ICRH ~20 MW, ECRH ~20 MW out of 73 MW total. That is 45%/27%/27%, not 50%/33%/17%. The Egyptian 1/2:1/3:1/6 split does not match. ICRH and ECRH are roughly equal, not in a 2:1 ratio.

---

## H-PP-13: Divertor Heat Flux Egyptian Distribution

**Grade: WEAK**

The in-out asymmetry (outer divertor receiving more power than inner) is well-established, with ratios typically 1.5:1 to 3:1 depending on machine and conditions. Claiming the exact split is 1/2:1/3:1/6 is not supported by published data. Experimental measurements from JET, ASDEX-U, and DIII-D show highly variable ratios depending on plasma conditions, magnetic geometry, and detachment state. The radiation fraction varies from <10% (attached) to >90% (fully detached).

---

## H-PP-14: Wendelstein 7-X has sopfr(6)=5 Field Periods

**Grade: CLOSE (EXACT for W7-X specifically, WEAK for general stellarator pattern)**

- **W7-X = 5 field periods**: Correct. This is the most advanced stellarator, and it has 5 field periods.
- **Why 5?** The choice of 5 was driven by optimization of neoclassical transport and quasi-isodynamic properties, not number theory. The design team (led by Nuhrenberg) optimized over many configurations.
- **Other stellarators**: LHD has 10 (not from n=6 arithmetic, but from heliotron design choices), HSX has 4 (optimized for quasi-helical symmetry), TJ-II has 4. Claiming all of these map to n=6 functions is post-hoc fitting -- with enough arithmetic functions of 6 (1, 2, 3, 4, 5, 6, 10, 12, 24...), you can match most small integers.

The individual W7-X match is real but likely coincidental.

---

## H-PP-15: D+T Reaction Mass Numbers

**Grade: EXACT (mass numbers), but TRIVIAL (energy split)**

- **D(2) + T(3) -> He-4(4) + n(1)**: The mass numbers are 2, 3, 4, 1. These do map to phi(6), sigma/tau, tau(6), mu(6). This is a genuine numerical fact.
- **Energy split 4:1**: As the document correctly notes, the 14.1:3.5 MeV split follows directly from momentum conservation and mass ratio. If mass numbers map to n=6 functions, the energy split automatically follows. This is not an independent prediction.
- **D(2)+T(3)=5=sopfr(6)**: The real insight is that 6=2*3, and D-T fusion combines nuclei with mass numbers equal to the prime factors of 6. This is a genuine numerical coincidence since D and T were chosen for fusion because of their nuclear cross-sections, not because of number theory.

This is the strongest hypothesis in the collection, but it is a retrodiction (explaining known facts), not a prediction.

---

## H-PP-16: PF Coils = n = 6

**Grade: EXACT (multiple machines)**

- ITER: 6 PF coils -- confirmed.
- ITER CS: 6 modules -- confirmed.
- JET: 6 PF coils -- confirmed (though JET has a more complex coil set overall).
- The physical argument (6 shape control degrees of freedom) is reasonable: vertical position, horizontal position, elongation, triangularity, squareness, X-point control.

However, not all tokamaks have exactly 6 PF coils:
- DIII-D: 18 shaping coils (though grouped differently)
- ASDEX Upgrade: different configuration
- KSTAR: 14 PF-like coils (including in-vessel coils)

The "6 PF coils" pattern holds for some major machines but is not universal. The physical argument about 6 degrees of freedom is the strongest part -- plasma shape control genuinely involves roughly this many independent parameters.

---

## H-PP-17: Egyptian Fraction Energy Partition

**Grade: FAIL**

The hypothesis claims plasma energy splits as 1/2 thermal + 1/3 magnetic + 1/6 kinetic. In a tokamak, beta ~ 3-5% means thermal energy is only 3-5% of magnetic energy. The magnetic field dominates overwhelmingly. The Egyptian fraction decomposition does not apply.

The SOL fallback (convective 1/2, conductive 1/3, radiative 1/6) is not supported by published data -- these ratios vary dramatically with plasma conditions. The document itself grades this as WEAK, which is generous.

---

## H-PP-18: Magnetic Reconnection Energy R(6)=1

**Grade: FAIL (R=1 claim), UNVERIFIABLE (Egyptian split)**

- **R(6)=1 = energy conservation**: This is trivially true of all physical processes. Energy is always conserved. Claiming R(6)=1 "predicts" energy conservation is not meaningful.
- **Egyptian fraction split (ions 1/2, electrons 1/3, accelerated particles 1/6)**: Yamada et al. (2014, Nature Communications) from the MRX experiment reported ions receive ~50% and electrons ~25-33% of reconnection energy. The accelerated particle fraction is highly variable and geometry-dependent. The approximate match for ions (~50%) is interesting but the exact 1/2:1/3:1/6 split is not established.

---

## H-PP-19: 12 Core Plasma Diagnostics = sigma(6)

**Grade: WEAK**

ITER has over 40 diagnostic systems. Grouping them into 12 "core measurements" is one possible taxonomy among many. You could equally group into 8, 10, 15, or 20 categories depending on granularity. The ITER diagnostic division itself uses a different categorization. The number 12 is achievable but not uniquely natural.

---

## H-PP-20: D-T Fuel Cycle = sopfr(6)=5 Components

**Grade: CLOSE (Li-6), WEAK (5 steps)**

- **Li-6 mass number = 6 = n**: This is a genuine physical fact. Li-6 + n -> T + He-4 is the primary tritium breeding reaction, and Li-6 has mass number 6. However, Li-7 also breeds tritium (Li-7 + n -> T + He-4 + n), and real blanket designs use natural lithium (7.5% Li-6, 92.5% Li-7) or enriched Li-6.
- **5-step fuel cycle**: The decomposition into 5 steps is one reasonable breakdown, but you could equally define 3 steps (breed, process, inject), 4 steps, or 7 steps depending on granularity.

---

## Summary Verification Table

| ID | Hypothesis | Document's Grade | Independent Grade | Notes |
|----|-----------|-----------------|-------------------|-------|
| H-PP-1 | 4 matter states = tau | EXACT | **EXACT (trivial)** | True but not predictive; pedagogical classification |
| H-PP-2 | 6 MHD equations | ARGUABLE | **WEAK** | Non-standard counting; textbooks say 8 scalar eqs |
| H-PP-3 | 3 confinement modes | EXACT | **CLOSE** | I-mode not universally established; other regimes exist |
| H-PP-4 | Debye length Egyptian | WEAK | **FAIL** | Electrons dominate >99%; no Egyptian structure |
| H-PP-5 | Plasma frequency sigma=12 | SPECULATIVE | **FAIL** | No physical basis for 12 as a cutoff |
| H-PP-6 | Q=10 = n+tau | INTERESTING | **CLOSE** | Numerical coincidence with engineering milestone |
| H-PP-7 | Tokamak geometry | STRONG | **CLOSE** | q>2 and q_0=1 are real; kappa claim fails; delta cherry-picked |
| H-PP-8 | Plasma beta | CLOSE | **WEAK** | Multiple expressions tried until one fits |
| H-PP-9 | 14 keV = sigma+phi | STRIKING | **CLOSE** | Genuine coincidence; 14 keV from nuclear physics, not n=6 |
| H-PP-10 | 4 MHD instabilities | EXACT | **EXACT** | Standard textbook 4-class taxonomy confirmed |
| H-PP-11 | ITER parameters | PARTIAL (43%) | **CLOSE (50%)** | Honest analysis; TF=18 is clear failure |
| H-PP-12 | 3 heating methods | EXACT | **EXACT (count) / FAIL (ratios)** | Count is correct; Egyptian power split is wrong |
| H-PP-13 | Divertor Egyptian | APPROXIMATE | **WEAK** | Ratios vary greatly with plasma conditions |
| H-PP-14 | W7-X = sopfr=5 | EXACT | **CLOSE** | W7-X match is exact; general stellarator pattern is post-hoc |
| H-PP-15 | D+T mass numbers | EXACT | **EXACT (but retrodiction)** | Strongest match; energy split follows automatically |
| H-PP-16 | PF coils = 6 | EXACT | **EXACT (some machines)** | ITER/JET yes; not universal across all tokamaks |
| H-PP-17 | Energy partition Egyptian | WEAK | **FAIL** | beta<<1 means thermal << magnetic; not Egyptian |
| H-PP-18 | Reconnection R=1 | TESTABLE | **FAIL / UNVERIFIABLE** | R=1 is trivial; Egyptian split not established |
| H-PP-19 | 12 diagnostics | ARGUABLE | **WEAK** | Arbitrary grouping; could be 8-20 |
| H-PP-20 | Fuel cycle + Li-6 | STRONG | **CLOSE (Li-6) / WEAK (5 steps)** | Li-6 = mass 6 is real; step count is arbitrary |

---

## Overall Verdict

| Grade | Count | Hypotheses |
|-------|-------|------------|
| EXACT | 5 | H-PP-1, H-PP-10, H-PP-12 (count only), H-PP-15 (retrodiction), H-PP-16 |
| CLOSE | 5 | H-PP-3, H-PP-6, H-PP-7, H-PP-9, H-PP-11 |
| WEAK | 5 | H-PP-2, H-PP-8, H-PP-13, H-PP-19, H-PP-20 |
| FAIL | 4 | H-PP-4, H-PP-5, H-PP-17, H-PP-18 |
| UNVERIFIABLE | 1 | H-PP-14 (general stellarator pattern; W7-X itself is CLOSE) |

**Final score: 5 EXACT + 5 CLOSE = 10/20 (50%) have genuine merit.**
**4 FAIL (20%) are contradicted by real physics or trivially true.**

---

## Structural Critique

1. **Small number bias**: The n=6 arithmetic functions produce the set {1, 2, 3, 4, 5, 6, 10, 12, 24}. These are all common small integers that appear frequently in engineering and physics. With 9+ target numbers, matching any parameter in the range 1-24 has high probability.

2. **Post-hoc fitting**: Multiple n=6 expressions are available (n, sigma, tau, phi, sopfr, n+tau, sigma/tau, sigma+phi, etc.), providing many degrees of freedom. For any physical quantity, at least one expression will likely be close.

3. **Strongest genuine results**:
   - D-T fusion mass numbers 2+3 -> 4+1 mapping to prime factors of 6 (H-PP-15)
   - Li-6 as tritium breeding material having mass number 6 (H-PP-20)
   - ITER PF coils = 6 with a physical degrees-of-freedom argument (H-PP-16)
   - Safety factor q > 2 as MHD stability condition (H-PP-7, partial)

4. **The D-T observation is the core insight**: Since 6 = 2 * 3, and D-T fusion literally fuses nuclei with mass numbers 2 and 3, there is an irreducible connection between the number 6 and D-T fusion. Everything else is downstream of this or coincidental.

5. **Credit where due**: The hypotheses document is notably honest, flagging its own failures (TF=18, Debye length, energy partition). This intellectual honesty is rare and commendable.

---

## Key Real-World Reference Data Used

- ITER: 18 TF coils, 6 PF coils, 6 CS modules, R=6.2m, a=2.0m, B_T=5.3T, Q=10 target, 500MW fusion power
- KSTAR: 16 TF coils, 300s at 100M C (2024 target)
- Wendelstein 7-X: 5 field periods, 50 non-planar + 20 planar coils
- D-T: 2+3 -> 4+1, Q=17.6 MeV (14.1 MeV neutron + 3.5 MeV alpha)
- Kruskal-Shafranov limit: q > 1 (internal kink), q_edge > 2 (external kink)
- D-T cross-section peak: ~13-15 keV
- Tokamak aspect ratios: ITER 3.1, KSTAR 3.6, JET 2.4, DIII-D 2.75
- Heating methods: NBI, ICRH, ECRH (+ Ohmic)
- Standard plasma states: solid, liquid, gas, plasma (+ BEC, etc.)
