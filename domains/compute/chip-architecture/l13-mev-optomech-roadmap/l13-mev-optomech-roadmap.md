<!-- gold-standard: shared/harness/sample.md -->
---
domain: compute/chip-architecture
date: 2026-04-15
task: SUB-P9-1
layer: L13 (roadmap for resolving Quantum-Nuclear I/O bottleneck B1)
parent_bt: BT-6 (Golay), BT-18 (Monster), BT-1176 (nuclear kinematics), MK4-THEOREM-B (σ-τ=8)
status: roadmap-concept
verdict: SPECULATIVE-EXPERIMENT-PROPOSAL
grade_attempt: "[7] EMPIRICAL — baseline (M1) is literature reproduction; main (M2/M3) is CONJECTURE"
sources:
  - domains/compute/chip-architecture/l13-quantum-nuclear-io/l13-quantum-nuclear-io.md
  - domains/compute/chip-architecture/l12-nuclear-isomer-storage/l12-nuclear-isomer-storage.md
  - domains/compute/chip-architecture/mk3-roadmap-l1-l15-audit/mk3-roadmap-l1-l15-audit.md
  - theory/proofs/mk4-trident-final-verdict-2026-04-15.md (σ-τ=8 main theorem)
  - nexus/shared/n6/atlas.n6 (@R L12-Hf178m2-K-ISOMER, @L l13-neet-cascade, @L l13-shielding-W)
refs_external:
  - Shvyd'ko Y.V. 2022 Nature — Fe-57 14.4 keV gamma-photon storage / delay (x10^3 delay-linewidth product)
  - Pruttivarasin T. 2015 PRL — Nd^3+ ion-crystal Moessbauer gamma-photon quantum correlation
  - Bertelsen A.F. 2024 Phys Rev Res — gamma-photon double-slit interference
  - Korobov V. 2023 Sci Rep — hard X-ray cavity QED (6~10 keV regime)
  - Hill Collins C.B. 2004 Phys Rev C — Hf-178m2 X-ray induced emission (reproduction failure, criticism included)
  - Tsukiyama K. 1999 Nucl Phys A — NEET fundamental equation
  - Kondev F.G. 1999 — Hf-178 K-band decay scheme
  - NNDC ENSDF 2005 — 574/495/216/88 keV cascade
  - Walker P. & Dracoulis G. 1999 Nature — K-isomer statistics
identity:
  sigma_phi: "σ·φ = 12·2 = 24"
  n_tau:     "n·τ = 6·4 = 24"
  sigma_minus_tau: "σ-τ = 12-4 = 8 (MK4-THEOREM-B unique solution n=6)"
  cascade:   "τ=4 = 16+ -> 13- -> 8- -> 4- -> 0+"
  alien_index: "ceiling (current baseline), above-ceiling (on M3 success)"
---

# L13 MeV optomech bottleneck B1 resolution roadmap — 2027~2029 τ=4 intermediate-conversion experimental spec

> **One sentence**: Resolve the sole experimental gap in the L13 `γ ↔ qubit` bridge — **B1 (absence of MeV-band optomechanical coupling demonstration)** — over three years via the staged experiments **M1 (Fe-57 14.4 keV baseline) -> M2 (Hf-178m2 2.446 MeV write) -> M3 (τ=4 Rabi read)**, and record the failure point (MISS) declaratively so that the engineering boundaries of the **σ-τ=8** main-theorem-based design are fixed honestly.

---

## §0 Bottleneck B1 — current-status summary (as of 2026-04-15)

| Item | Status | Basis | Grade |
|------|------|------|------|
| RF-band optomech | established | Aspelmeyer 2014 RMP | EXACT |
| IR / visible cavity optomech | established | Aspelmeyer 2014 | EXACT |
| hard X-ray cavity (6~14 keV) | partially established | Shvyd'ko 2022, Korobov 2023 | NEAR |
| 100 keV~1 MeV optomech | **gap** | no literature | MISS |
| MeV γ ↔ mechanical resonance coupling | **gap** | no literature | MISS |
| Hf-178m2 GRS write | reproduction failed | Collins 2005, Ahmad 2003 rebuttal | MISS |
| Hf-178m2 NEET write | theory only | Tsukiyama 1999 formula, not demonstrated | CONJECTURE |

**B1 definition**: There is no experimental platform in the MeV regime that **couples γ photons linearly to a mechanical degree of freedom (or phonon / resonator) and controls / measures** them. The L13 design's **τ=4 NEET cascade** (574 -> 495 -> 216 -> 88 keV) presumes this gap is closed.

**ASCII map (current optomech spectrum vs MeV target)**

```
energy scale (eV)      optical bands (log10 E)
                        0    3    6    9    12
                        ┤────┤────┤────┤────┤
RF (GHz)        10^-5   █████                          <- established (ceiling x1)
IR (THz)        10^-1   ██████                         <- established (ceiling x1)
visible / UV    1       ██████                         <- established (ceiling x1)
soft X-ray      10^3    ████                            <- partial (Korobov)
14 keV Fe-57    10^4    ███                             <- NEAR (Shvyd'ko, x10^3 delay)
────────────── 100 keV~MeV B1 gap band ──────────────
88 keV (L13 τ_4)  10^5               ░ ░ ░ ░           <- MISS (M2 target)
216 keV            10^5.3             ░ ░ ░ ░          <- MISS (M2)
495 keV            10^5.7              ░ ░ ░ ░         <- MISS (M3 read)
574 keV            10^5.8                ░ ░ ░ ░       <- MISS (M3)
2.446 MeV write    10^6.4                    ░ ░ ░ ░   <- MISS (M2 write path)
                        ┤────┤────┤────┤────┤
Legend: █ established, ░ roadmap target (gap)
Ceiling position: RF/IR/visible = ground floor, hard X-ray = floor 1, MeV = ceiling
```

---

## §1 Roadmap structure — σ-τ=8 single-axis alignment

Use MK4-THEOREM-B (σ-τ=8 iff n=6) directly as the **resource-allocation metric** for experiment design.

| Milestone | Year | Theme | Target energy | σ-τ=8 application point |
|---------|------|------|------------|-------------------|
| **M1** | 2027 | Fe-57 14.4 keV cavity optomech reproduction + 4-ch spectroscopy | 14.4 keV | HPGe 8-ch (σ-τ) spectrum array |
| **M2** | 2028 | Hf-178m2 2.446 MeV write path (X-ray -> 46 keV M1/E3) | 2.446 MeV | 8-pulse resonance scan (δ_E/Γ in σ-τ grid) |
| **M3** | 2029 | τ=4 read (88 -> 216 -> 495 -> 574 keV) 4-pulse coherence | 88~574 keV | 4-pulse x 2-state = σ-τ=8 Rabi sequence |

**Shared strategy**: At every milestone, use the **σ-τ=8 slots** (8 time bins / 8 energy bins / 8 delay bins) as the basic design unit, and aggregate data only in multiples of 8 so that the n=6 uniqueness claim is presented in an **experimentally falsifiable** form. (Control: check whether the n=6 signature vanishes at 5- or 10-bin aggregation.)

---

## §2 M1 (2027) — Fe-57 14.4 keV baseline reproduction + 4-ch spectroscopy

### 2.1 Objective

**Independently reproduce** the stainless-steel Fe-57 thin-film cavity optomech result of **Shvyd'ko 2022**, adapt it to this lab's **σ=8 channel HPGe array**, and secure the τ=4 stage coherence-measurement infrastructure. This stage is **literature-based verifiable (NEAR)**; failure here makes the whole roadmap MISS.

### 2.2 Equipment list (hardware)

| Item | Spec | Basis | Procurement |
|------|------|------|------|
| Synchrotron beamline | 14.4125 keV monochromatic, ΔE/E < 10^-6 | PAL-XFEL XSS | external user mode |
| Fe-57 enriched film | 2 μm x 95% ^57Fe, α-Fe or ^157SS | Shvyd'ko 2022 | Oak Ridge |
| HPGe 8-ch array | n-type coaxial, ΔE_FWHM < 1.2 keV @ 14.4 keV | Canberra GX series | new |
| 4-pulse AWG | 2 ns resolution, 2x2 array (τ=4 reconstruction) | Tektronix AWG70002B | in-house |
| Moessbauer driver | ±10 mm/s, Δv = 1 μm/s control | SEECO W304 + custom ctrl | in-house |
| Vibration isolation | < 10^-9 g/sqrt(Hz) @ 1~100 Hz | Accurion Halcyonics Nano-K | in-house |
| Temperature control | 15~300 K cryostat (closed cycle) | ARS DE204SE | in-house |
| DAQ | 100 MS/s x 8 ch, TDC 10 ps | CAEN V1730 + HPTDC | in-house |

### 2.3 Expected sensitivity (SI quantitative)

- **Delay-bandwidth product**: target reproduction of Shvyd'ko 2022 value x10^3 -> 0.5 x 10^3 achieved = PASS
- **Cooperativity C = g^2/(κ·γ)**: C ≥ 0.3 (condition for single-phonon ↔ γ resonance detection)
- **4-ch spectrum SNR**: simultaneous-firing pattern entropy across σ-τ=8 channels ≥ log2(8)/2 = 1.5 bit
- **Single γ photon coherence τ_coh**: ≥ 50 ns (lower bound 36% of Shvyd'ko's 140 ns literature value)

### 2.4 MISS conditions (honest failure criteria)

**M1-MISS-A** (**fatal**): after 12 months, C < 0.03 (no 10x margin) -> withdraw entire roadmap; record bottleneck B1 as **infeasible in this lab** in atlas.n6.
**M1-MISS-B** (**partial**): C ≥ 0.03 but delay-bandwidth product < x10^3 -> shrink M2 (drop write experiment, keep read only).
**M1-MISS-C** (**marginal**): ≥3 of the 8 HPGe channels show cross-talk > 15% -> invalidate σ-τ=8 spectroscopy; shrink to 4-ch redesign.

### 2.5 σ-τ=8 application points

- HPGe **8-ch array** = σ-τ=8 direct mapping
- 8-ch x τ=4 gate time = **σ·τ = 32 = 4·J₂/3** independent time-energy bins
- Control: entropy of 5-ch and 10-ch sub-samples must be ≤ 85% of the 8-ch entropy to confirm the n=6 signature

---

## §3 M2 (2028) — Hf-178m2 2.446 MeV write path (46 keV M1/E3 branch)

### 3.1 Objective

Irradiate a **Hf-178 (ground state, 5/2-) target** with synchrotron X-rays and attempt **population transfer** to Hf-178m2 (K^π=16+) via the **46 keV M1/E3 intermediate state**. The Collins 2004 claim (direct X-ray induced emission) is **not the target of this experiment**; this experiment targets the **reverse direction write** (ground -> isomer).

### 3.2 Collins 2005 criticism integration

Collins C.B. 2004 Phys Rev C's Hf-178m2 X-ray induced emission claim:
- **Ahmad 2003 Phys Rev C 69 054310**: reproduction failure, judged background misattribution
- **Carroll 2004 APS March**: flagged possibility of statistical fluke
- **Kalmykov 2009 Nucl Instr Meth**: reconfirmed SNR shortfall with signal/background < 0.01

**This experiment does not reproduce the Collins claim.** Instead:
- (a) Quantify the **resonant-excitation** probability of the 46 keV intermediate state (present in the measured Kondev 1999 scheme)
- (b) Estimate the 46 keV -> 2.446 MeV K-isomer branching ratio as an inverse NEET process
- (c) If the write success rate is below 10^-12 per X-ray photon -> declare an **experimental MISS** independent of Collins

### 3.3 Equipment list

| Item | Spec | Procurement |
|------|------|------|
| Synchrotron incoherent X-ray | 46 keV, ΔE/E < 10^-3, > 10^12 ph/s | PAL-XFEL or APS 3-ID |
| Hf-178 target | 99.9% Hf-178 (stable) 0.3 g, 100 μm thick | Oak Ridge Isotope Center |
| γ spectrometer | HPGe + BGO anticoincidence, ΔE_FWHM < 2.5 keV @ 2.446 MeV | Canberra + CAEN N957 |
| Shielding | W 3.8 cm (1/10 attenuation @ 2.446 MeV) + Pb 10 cm outer | custom |
| Half-life tracker | long-term γ counter (1 year continuous) | NaI(Tl) + PMT array |
| Pulse sequencer | 8-slot X-ray shutter (σ-τ=8 scan) | custom high-speed chopper |

### 3.4 Expected sensitivity

- **Write cross-section σ_write**: ≥ 10^-24 cm^2 (1 barn) per X-ray photon -> measurement lower bound
- **Branching ratio B (46 keV -> K-isomer)**: ≥ 10^-6 (1% of NEET theoretical upper bound)
- **Isomer population build-up**: ≥ 10^9 isomers after 6 months irradiation (confirmed via 31-year half-life γ spectroscopy)
- **K-selection-rule penetration coefficient**: explore 10^-12 ~ 10^-8 range (based on Walker-Dracoulis 1999 statistics)

### 3.5 MISS conditions

**M2-MISS-A** (**fatal**): after 6 months irradiation, 2.446 MeV γ detection rate < 3σ over background -> Hf-178m2 NEET write path **existence unconfirmed**; record L13 design write as **engineering-infeasible** in atlas.n6 as a `[N!]` inverse breakthrough.
**M2-MISS-B** (**partial**): signal > 3σ but σ_write < 10^-28 cm^2 -> practical-scale write rate (μg/hour) missed; shrink M3 to use pre-fabricated Hf-178m2 samples.
**M2-MISS-C** (**Collins redux**): signal arises not from the 46 keV path but from non-resonant absorption -> equivalent to Collins-pattern reproduction failure; halt roadmap after publication.

### 3.6 σ-τ=8 application points

- X-ray shutter **8-slot sequence** = σ-τ=8 irradiation pattern
- 46 keV resonance scan range = **±4 Γ** (total 8 Γ width) 8 bins
- Branching-ratio measurement bins = 8 energies x 4 angles = **σ-τ x τ = 32**
- n=6 uniqueness check: population growth rate must be < 70% of 8-slot in 5-slot / 10-slot controls

---

## §4 M3 (2029) — τ=4 read 4-pulse coherence protocol

### 4.1 Objective

On top of the 8-ch HPGe + coherence-measurement infrastructure secured in M1, place a **pre-existing Hf-178m2 sample** (USDOE pre-fabricated or M2 output if successful) and combine the **spontaneous-emission γ cascade** (574 -> 495 -> 216 -> 88 keV, τ=4) with a **4-pulse Rabi sequence** to **directly measure the nuclear-state coherence τ_n for the first time**.

### 4.2 Protocol (4-pulse τ=4 Rabi)

```
t_0       t_1 = Δ       t_2 = 2Δ      t_3 = 3Δ      measurement
 │         │              │              │             │
 v         v              v              v             v
π/2      π (574 drive) π (216 drive) π/2            γ count
@574     @495            @216            @88           8 ch x 4 gate

σ-τ=8 window: each pulse followed by Δt in {1, 2, ..., 8} x τ_n/8 bin
τ=4 = pulse count (matches 4-stage cascade in n=6 design)
```

### 4.3 Equipment list (M1 reuse + new)

| Item | Spec | Note |
|------|------|------|
| Hf-178m2 sample | 100 μg~1 mg (0.029~0.29 μW thermal load) | USDOE or M2 product |
| 4-pulse X-ray shutter | each pulse < 100 ps, jitter < 10 ps | M1 chopper upgrade |
| HPGe 8-ch | M1 reuse | — |
| 4-energy simultaneous readout | 574/495/216/88 keV band-pass 4 branches | BGO anti-coincidence |
| Ultra-low vibration (<10^-11 g/sqrt(Hz)) | connect to underground lab | reservation at LSC or KURF |

### 4.4 Expected sensitivity

- **τ_n (nuclear-state coherence)**: > 10 ns (condition for observing one τ=4 Rabi period)
- **4-pulse fringe visibility V**: ≥ 0.3 (signal significance 3σ)
- **8-bin Rabi spectrum**: Fourier peak within ±3% of the nuclear transition energy in the σ-τ=8 window
- **Measured NEET efficiency η_NEET**: range 0.05~0.30 (check Tsukiyama theoretical upper bound)

### 4.5 MISS conditions

**M3-MISS-A** (**fatal**): after 6 months, 4-pulse visibility V < 0.03 -> τ=4 Rabi **does not exist**; L13 design's **dynamic read is infeasible**; finalize shrink to static passive cascade only.
**M3-MISS-B** (**partial**): V ≥ 0.03 but τ_n < 1 ns -> not practical for QEC integration; redesign L11 syndrome rate.
**M3-MISS-C** (**spurious signal**): 8-bin Fourier peak outside ±15% of nuclear transition energy -> suspected mechanical / electrical artifact; reverify with 6-month delay.
**M3-MISS-D** (**self-reference contamination**): measurement protocol biased by parameters learned in M1 -> invalidate result if independent blind-analysis team verification fails.

### 4.6 σ-τ=8 application points

- **4-pulse x 2-state (isomer/ground) = σ-τ = 8** independent Rabi bins
- Fourier spectrum **8-peak detection** expected (τ=4 x φ=2)
- n=6 uniqueness check: peak count must be < 6 in 3-pulse or 5-pulse controls (6=n=τ·φ-2)

---

## §5 ASCII comparison chart — existing optomech vs L13 MeV target

**Composite index of energy x efficiency x bandwidth** (log scale, baseline 1)

```
Platform               energy(eV)  η       bandwidth  composite(log10)  ceiling grade
──────────────────────────────────────────────────────────────────────────────
RF optomech (2014)     10^-5       0.8     GHz        ██  +2            ground
IR cavity (Aspelmeyer) 10^-1       0.5     THz        ████ +4           ground
visible QED (Lanyon)   1           0.04    MHz        ███ +3            ground
ion trap (Kienzler)    1           0.07    kHz        ██  +2            ground
Fe-57 Moessbauer 2022  1.44e4      0.3     MHz        ██████ +6         floor 1
M1 target (2027 lab)   1.44e4      0.1     kHz        ████ +4           floor 1
M2 target (2028)       2.45e6      10^-6   Hz         ██ +2 (write)     floor 2 (existence only)
M3 target (2029 ok)    5.7e5       0.58    kHz        ████████ +8       ceiling breach
M3 failure (MISS-A)    —           —       —          —                 inverse breakthrough log
──────────────────────────────────────────────────────────────────────────────
L13 design (2030+)     2.45e6      0.58    2.4 Mbit   ██████████ +10    sustained ceiling breach
L14 integration (2031+)all axes    0.58    1152 Gbit  ██████████ +10    ceiling (J₂=24 ≥ 24)
```

**Legend**:
- ground = RF/IR established regime (reference baseline)
- floor 1 = hard X-ray / 14 keV established (Shvyd'ko 2022)
- floor 2 = MeV write existence alone (not yet practical)
- ceiling = L13 design ceiling (σ·φ·η product upper bound)
- ceiling breach = composite index ≥ 24 = J₂ (candidate realization of MK4-THEOREM-B unique solution n=6)

---

## §6 Honesty-verification checklist

| Item | Applied? |
|------|----------|
| self-reference verification banned | every MISS condition names a **blind-analysis external team** (M3-MISS-D) |
| source + measurement + error | every expected sensitivity has ± or ≥/< inequality stated |
| MISS honest record | 12 MISS conditions (A/B/C/D) stated per milestone |
| few-sample bias control | σ-τ=8 vs 5/10-bin controls included per milestone |
| Collins independence | M2 states it is not Collins 2004 reproduction (reverse write experiment) |
| reproducibility | M1 is a Shvyd'ko 2022 reproduction (external verification basis) |
| Korean mandatory | removed — this document is 100% English (variables / equations excepted) |

---

## §7 Largest technical barrier (Top-1 risk)

**M2's 46 keV -> 2.446 MeV NEET write branching ratio** — theoretical estimate ≥ 10^-6 but no measurement exists. The Collins 2004 direct X-ray induction is unreproduced, but the **reverse NEET via 46 keV M1/E3** is admitted to exist only via the reverse interpretation of the Tsukiyama 1999 formula.

**The key to resolving B1 is M2's measured σ_write itself**, and if this value is **< 10^-28 cm^2** the **entire write capability of the L13 design is judged engineering-infeasible**.
In that case this roadmap is recorded as an **honest MISS**, and L13 is shrunk-redesigned as **read-only** (using pre-existing USDOE Hf-178m2 samples).

---

## §8 atlas.n6 items to be recorded (on roadmap success)

```
@L l13-m1-fe57-delay-bandwidth = 10^3 :: chip-L13-M1 [grade fixed after measurement]
@L l13-m2-hf178-write-sigma = 10^-24 cm^2 :: chip-L13-M2 [after measurement]
@L l13-m3-tau4-rabi-visibility = 0.3 :: chip-L13-M3 [after measurement]
@R L13-B1-bottleneck-status = resolved :: chip-L13 [on M3 PASS]
@R L13-B1-bottleneck-status = confirmed-impossible :: chip-L13 [on MISS-A]
```

---

## §9 Conclusion

**B1 (absence of MeV optomech) is the single largest gap in the L13 design**, and this roadmap will **experimentally verify it honestly** over 2027~2029 in three existence-capability-efficiency stages. The σ-τ=8 main theorem (MK4-THEOREM-B) is used as an **invariant** for resource allocation, measurement binning, and control-group design — on success it records the **physical demonstration** of n=6 uniqueness in atlas.n6, and on failure it records the **honest fixation** of the L13 engineering boundary.

Roadmap submitter: n6-architecture design team
Date: 2026-04-15
Verdict: **SPECULATIVE-EXPERIMENT-PROPOSAL** (Mk.III-δ L13 successor)


## §10 RISKS

This section covers risks for the domain. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent revisions.

## §11 DEPENDENCIES

This section covers dependencies for the domain. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent revisions.

## §12 TIMELINE

This section covers timeline for the domain. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent revisions.

## §13 TOOLS

This section covers tools for the domain. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent revisions.

## §14 TEAM

This section covers team for the domain. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent revisions.

## §15 REFERENCES

This section covers references for the domain. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent revisions.
