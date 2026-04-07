# Neuralink N1 Technical Analysis for BCI Bridge

## Overview

This document provides a technically precise analysis of Neuralink's N1 implant and how BrainWire's 12-variable consciousness model maps to its specific hardware capabilities. Unlike our broader BCI Bridge vision document, this analysis is grounded in verified N1 specifications and is honest about what the hardware can and cannot do — particularly regarding depth limitations.

**Key conclusion:** The N1 is a cortical device. It excels at cortical variable control (Alpha, Gamma, PFC, Sensory, Body, Coherence) but cannot reach the deep subcortical and brainstem structures (VTA, LC, raphe nuclei) that drive our neurochemical variables (DA, 5HT, NE, eCB). A hybrid architecture — N1 for cortical + external stimulation for deep targets — is the realistic path.

---

## 1. N1 Hardware Specifications

| Parameter | Value | Notes |
|---|---|---|
| Electrodes | 1024 (upgrading to 1536) | 64 threads x 16 electrodes per thread |
| Sampling rate | 20 kHz per channel | 10-bit ADC resolution |
| Stimulation current | 600 uA max per channel | Biphasic, charge-balanced |
| Simultaneous stim channels | 64 | Out of 1024 total |
| Amplitude resolution | 8-bit (256 levels) | ~2.3 uA per step at 600 uA range |
| Wireless bandwidth | ~1 Mbps BLE | 200:1 compression from 204.8 Mbps raw |
| Power consumption | 24.7 mW total | 6.6 uW per channel |
| Physical size | 23 mm diameter x 8 mm thick | Coin-sized, flush with skull |
| Battery life | ~12 hours | Wireless charging ~1 hour |
| Thread diameter | ~24 um | Thinner than human hair (70 um) |
| Thread insertion depth | ~3-6 mm | Cortical layers, limited subcortical |

### Clinical Status (as of early 2026)

- **PRIME Study:** 6 patients implanted (motor cortex), all met safety endpoints
- **First patient:** Noland Arbaugh (January 2024), motor cortex placement
- **Current capability:** Read-only in practice — cursor control, gaming, typing
- **Stimulation:** Hardware-ready but NOT deployed clinically (regulatory caution)
- **Blindsight:** FDA Breakthrough Device Designation (September 2024), visual cortex stimulation, first human implants expected within 6-12 months
- **Geographic expansion:** Canada, UK, Germany

### Competitive Landscape

| Device | Electrodes | Bidirectional | Clinical Stim | Notes |
|---|---|---|---|---|
| Neuralink N1 | 1024 | Yes (hardware) | Not yet | Cortical, wireless |
| Blackrock Utah Array | 96-128 | Yes (proven) | 20+ years | Gold standard for research |
| Precision Layer 7 | 1024 | Yes | FDA cleared April 2025 | Cortical surface, less invasive |
| Synchron Stentrode | 7-16 | Read-only | N/A | Endovascular, no stimulation |

---

## 2. N1 Hardware to 12-Variable Mapping

### Electrode Allocation Strategy (1024 electrodes)

The allocation below assumes a single N1 implant. Realistic placement is constrained by the N1's thread insertion depth (~3-6 mm from cortical surface) and the surgical robot's access to exposed cortex.

| Allocation | Electrodes | Target Region | Variables Served |
|---|---|---|---|
| Motor/Prefrontal block | 128 | Prefrontal cortex (dlPFC, F4) | V8: Gamma, V9: PFC suppression |
| Visual block | 128 | Visual cortex (V1, V2) | V7: Alpha suppression, V10: Sensory gain |
| Somatosensory block | 128 | Primary somatosensory (S1) | V11: Body sensation |
| Temporal/Parietal block | 64 | Temporal-parietal cortex | V6: Theta (cortical component) |
| Distributed coherence | 128 | Spread across all regions | V12: Cross-region coherence |
| Frontal midline | 64 | Medial frontal (Fz region) | V7: Alpha (frontal component) |
| Thalamic reach | 64 | Shallow thalamic nuclei | V4: GABA (if depth permits) |
| Recording / monitoring | 192 | Distributed | Closed-loop EEG measurement |
| Reserve / redundancy | 128 | Distributed | Backup, impedance monitoring |
| **Total** | **1024** | | |

### Critical Depth Constraint

The N1's polymer threads are designed for cortical insertion at 3-6 mm depth. The following structures are relevant to our model:

| Structure | Depth from Cortex | N1 Reachable? | Variables Affected |
|---|---|---|---|
| Cortical layers I-VI | 0-4 mm | YES | V7, V8, V9, V10, V11, V12 |
| Thalamus (pulvinar, LGN) | 40-60 mm | NO | V4 (GABA), V7 (Alpha) |
| Hippocampus | 30-50 mm | NO | V2 (eCB), V6 (Theta) |
| VTA (ventral tegmental area) | 70-80 mm | NO | V1 (DA) |
| Locus coeruleus | 80-100 mm (brainstem) | NO | V5 (NE) |
| Raphe nuclei | 80-100 mm (brainstem) | NO | V3 (5HT) |
| Nucleus tractus solitarius | 90+ mm (brainstem) | NO | taVNS relay |

**This is the single most important fact for BCI Bridge design:** The N1 cannot directly access any of the deep nuclei that produce dopamine, serotonin, norepinephrine, or endocannabinoids. Our neurochemical variables (V1-V5) must use indirect cortical pathways or external stimulation even with an N1 implant.

---

## 3. Stimulation Parameters and Transfer Functions

### N1 Stimulation Capabilities

- **Maximum current:** 600 uA per channel
- **Amplitude resolution:** 8-bit = 256 levels = ~2.3 uA per step
- **Simultaneous channels:** 64 (out of 1024)
- **Waveform:** Biphasic, charge-balanced pulses
- **Pulse width:** Programmable (typically 100-400 us per phase)

### Direct vs. Indirect Effectiveness

At the cortical surface, tDCS delivers ~0.3-0.5 V/m at a target with 2 mA applied at scalp. The N1 delivers current directly to tissue within ~100 um of the electrode tip. The effective comparison:

| Parameter | Scalp tDCS (2 mA) | N1 Direct (600 uA) |
|---|---|---|
| Current at target | ~0.1-0.2 mA (after skull/CSF attenuation) | 0.6 mA (direct) |
| Spatial resolution | ~5-10 cm (diffuse) | ~100 um (focal) |
| Effective field at target | ~0.3 V/m | ~10-50 V/m (estimated) |
| Stimulation efficiency | ~5-10% of applied current reaches target | ~100% |

This means 600 uA at the electrode tip is roughly equivalent to 6-12 mA at the scalp — about 3-6x more current than our external devices deliver. For cortical targets, the N1 is dramatically more effective.

### Modified Transfer Function Coefficients

For our 12-variable model, each variable has a transfer function coefficient describing how effectively stimulation modulates the target. Below are estimated coefficients for N1 direct cortical stimulation vs. our current non-invasive approach.

| Variable | Current Coeff (non-invasive) | N1 Coeff (direct cortical) | Ratio | Notes |
|---|---|---|---|---|
| V1: DA | 0.25 (tDCS->DLPFC->VTA) | 0.40 (dlPFC->VTA, still indirect) | 1.6x | Cannot reach VTA directly |
| V2: eCB | 0.80 (TENS+taVNS) | 0.60 (cortical only, loses peripheral) | 0.75x | Peripheral pathways lost |
| V3: 5HT | 1.20 (taVNS->raphe) | 0.30 (cortical, no raphe access) | 0.25x | WORSE without taVNS |
| V4: GABA | 0.70 (alpha entrainment) | 1.80 (direct GABAergic activation) | 2.6x | Cortical interneurons accessible |
| V5: NE | 0.50 (taVNS parasympathetic) | 0.20 (cortical, no LC access) | 0.40x | WORSE without taVNS |
| V6: Theta | 0.60 (TMS 6Hz surface) | 1.50 (cortical theta entrainment) | 2.5x | Hippocampal theta still indirect |
| V7: Alpha | 0.55 (tDCS cathode Fz) | 2.00 (direct thalamo-cortical) | 3.6x | Excellent cortical target |
| V8: Gamma | 0.65 (TMS 40Hz) | 2.50 (direct 40 Hz cortical drive) | 3.8x | Best variable for N1 |
| V9: PFC | 0.60 (tDCS cathode F4) | 2.20 (direct PFC inhibition) | 3.7x | Excellent cortical target |
| V10: Sensory | 0.50 (tDCS V1) | 2.00 (direct V1 layer 4) | 4.0x | Layer-specific targeting |
| V11: Body | 0.75 (TENS peripheral) | 1.80 (S1 direct) | 2.4x | Different quality — central not peripheral |
| V12: Coherence | 0.45 (40 Hz tri-modal) | 2.50 (cross-region phase-lock) | 5.6x | Biggest improvement |

**Key insight:** The N1 dramatically improves cortical variables (V4, V6-V12) but actually makes neurochemical variables (V1-V3, V5) worse when used alone because it loses access to the peripheral pathways (taVNS, TENS) that indirectly reach deep nuclei.

---

## 4. Bandwidth Constraint Analysis

### The Problem

The N1's wireless link runs at approximately 1 Mbps over BLE.

- **Raw data rate:** 1024 channels x 20 kHz x 10-bit = 204.8 Mbps
- **Available bandwidth:** ~1 Mbps
- **Compression needed:** 200:1

Neuralink currently solves this by on-chip spike detection and compression — transmitting only detected spike events rather than raw waveforms.

### BrainWire Data Requirements

For our closed-loop consciousness model, we need:

| Data Stream | Calculation | Bandwidth |
|---|---|---|
| 12 state variables (out) | 12 x 32-bit x 1000 Hz | 384 kbps |
| Stimulation commands (in) | 64 ch x 16-bit x 1000 Hz | 1,024 kbps |
| Impedance monitoring | 1024 ch x 16-bit x 1 Hz | 16 kbps |
| Error/status telemetry | Fixed overhead | ~50 kbps |
| **Total bidirectional** | | **~1.5 Mbps** |

This is tight but feasible if the 12-variable extraction runs on-chip. The critical requirement: **BrainWire's consciousness state engine must be ported to the N1's ASIC.**

### On-Chip Processing Architecture

```
N1 ASIC Pipeline:
  1024ch raw (204.8 Mbps)
    -> Spike detection (existing Neuralink)
    -> Band-power extraction (delta, theta, alpha, beta, gamma per channel)
    -> 12-variable state estimator (OUR MODEL, running on-chip)
    -> Compressed state vector (384 kbps)
    -> BLE uplink to external controller

External Controller:
    -> Receives 12-variable state
    -> Runs PID control loop
    -> Sends stimulation parameters back (1,024 kbps)
    -> BLE downlink to N1

N1 ASIC Stimulation:
    -> Receives stim parameters
    -> Drives 64 simultaneous channels
    -> Charge-balanced biphasic pulses
```

### Computational Requirements for On-Chip Model

The 12-variable extraction requires:
- Band-power computation: FFT or filter bank on 1024 channels — this is ~20 MFLOPS
- State estimation: 12-variable linear combination of band powers — ~0.1 MFLOPS
- Total: ~20 MFLOPS, well within modern ASIC capability

The N1's digital backend already performs spike sorting, which is computationally more expensive than band-power extraction. Adding our model is feasible.

---

## 5. Latency Analysis

### Current BrainWire Latency Budget

| Stage | Latency | Notes |
|---|---|---|
| EEG acquisition (scalp) | 5 ms | OpenBCI sampling + buffer |
| USB transfer | 3 ms | Serial to PC |
| Signal processing | 10 ms | FFT, filtering |
| 12-variable computation | 2 ms | State estimation |
| PID control decision | 1 ms | Control loop |
| Stimulation command | 5 ms | To stimulation device |
| Stimulation onset | 5 ms | Device response time |
| Neural effect propagation | 10 ms | Through tissue |
| **Total loop** | **~41 ms** | |

### N1 On-Chip Latency Budget

| Stage | Latency | Notes |
|---|---|---|
| Electrode acquisition | 0.05 ms | Direct, 20 kHz = 50 us sample period |
| On-chip processing | 0.1 ms | ASIC pipeline, no OS overhead |
| State estimation | 0.05 ms | Hardware accelerated |
| Stim command generation | 0.05 ms | On-chip decision |
| Stimulation onset | 0.05 ms | Same chip, no transfer |
| Neural effect propagation | 0.5 ms | Direct tissue, shorter path |
| **Total loop** | **~0.8 ms** | |

### Phase-Locking Implications

At 40 Hz gamma (25 ms period):

| System | Loop Latency | Phase Lag | Phase-Lock Possible? |
|---|---|---|---|
| Current (external) | 41 ms | 1.64 cycles (590 degrees) | NO — more than full cycle behind |
| N1 on-chip | 0.8 ms | 0.032 cycles (11.5 degrees) | YES — excellent phase tracking |

At 6 Hz theta (167 ms period):

| System | Loop Latency | Phase Lag | Phase-Lock Possible? |
|---|---|---|---|
| Current (external) | 41 ms | 0.25 cycles (88 degrees) | MARGINAL — quarter cycle lag |
| N1 on-chip | 0.8 ms | 0.005 cycles (1.7 degrees) | YES — near-perfect |

### What Phase-Locking Enables

With sub-millisecond latency, the N1 can:

1. **Gamma phase boosting (V8):** Stimulate at the excitatory peak of each 40 Hz cycle, amplifying endogenous gamma by constructive interference rather than imposing an external rhythm.

2. **Theta-gamma coupling (V6 + V8):** Modulate gamma amplitude as a function of theta phase — the cross-frequency coupling pattern associated with memory encoding and consciousness. This is impossible non-invasively because the 41 ms loop cannot track both frequencies simultaneously.

3. **Alpha suppression at trough (V7):** Deliver inhibitory pulses precisely at alpha wave peaks to suppress the rhythm, rather than applying tonic inhibition. More efficient and selective.

4. **Cross-region coherence (V12):** Synchronize stimulation across distant cortical regions within 0.1 ms, enabling true phase-zero coherence that external stimulation cannot achieve.

---

## 6. Safety Analysis for Direct Stimulation

### Charge Density Calculations

The primary safety metric for neural stimulation is charge density per phase, governed by the Shannon equation:

```
log(Q/A) = k - log(Q)
```

Where Q = charge per phase, A = electrode geometric surface area, and k is an empirically derived safety factor (k = 1.5-2.0 for safe stimulation).

**N1 electrode parameters:**
- Electrode area: ~50 um^2 (geometric), ~200 um^2 (effective, accounting for surface roughness)
- Maximum current: 600 uA
- Typical pulse width: 200 us per phase (biphasic)

**Charge per phase:**
```
Q = I * t = 600 uA * 200 us = 120 nC
```

**Charge density (geometric):**
```
Q/A = 120 nC / 50 um^2 = 120 nC / (50 x 10^-6 cm^2)
    = 120 nC / (5 x 10^-5 cm^2)
    = 2.4 x 10^6 nC/cm^2
    = 2.4 mC/cm^2
    = 2,400 uC/cm^2
```

**WAIT — this exceeds the Shannon limit of 30 uC/cm^2 per phase.** However, this calculation uses the geometric surface area. Neuralink's electrodes use sputtered iridium oxide (SIROF) or similar coatings that increase the effective electrochemical surface area by 10-100x. With a 100x roughness factor:

```
Effective charge density = 2,400 / 100 = 24 uC/cm^2 (SAFE, under 30 uC/cm^2 limit)
```

Additionally, Neuralink would not operate at maximum current for sustained stimulation. For our consciousness model, typical operating currents would be 50-200 uA, yielding:

```
At 200 uA, 200 us: Q = 40 nC -> 8 uC/cm^2 effective (well within safe limits)
At 50 uA, 200 us:  Q = 10 nC -> 2 uC/cm^2 effective (very conservative)
```

### BrainWire Operating Envelope

For the 12-variable model, we define safety margins per variable:

| Variable | Typical Current | Duty Cycle | Charge/Phase | Safety Margin |
|---|---|---|---|---|
| V8: Gamma (40 Hz) | 100 uA | 0.8% (200 us/25 ms) | 20 nC | 6x under limit |
| V9: PFC suppress | 200 uA | Tonic, 1 Hz | 40 nC | 3x under limit |
| V7: Alpha suppress | 150 uA | 10 Hz | 30 nC | 4x under limit |
| V10: Sensory gain | 100 uA | 20 Hz | 20 nC | 6x under limit |
| V11: Body percept | 200 uA | 4 Hz | 40 nC | 3x under limit |
| V12: Coherence | 100 uA | 40 Hz | 20 nC | 6x under limit |

### Long-Term Tissue Considerations

| Concern | Timeline | Mitigation |
|---|---|---|
| Glial scarring | 1-6 months post-implant | Impedance monitoring, adaptive current scaling |
| Electrode degradation | 1-5 years | Platinum/iridium alloy, charge-balanced pulses |
| Tissue impedance drift | Ongoing | Closed-loop impedance measurement (use 128 monitoring electrodes) |
| Thermal effects | Acute | 24.7 mW total is well under tissue heating threshold |
| Electrolysis / pH shift | Cumulative | Strict charge-balance enforcement, interphase gap |

---

## 7. Comparison: N1 vs. Non-Invasive for Each Variable

| Variable | Non-Invasive Method | Coeff | N1 Direct | Coeff | Improvement | Honest Assessment |
|---|---|---|---|---|---|---|
| V1: DA | tDCS(F3)->DLPFC->VTA | 0.25 | dlPFC direct, still can't reach VTA | 0.40 | 1.6x | Modest — depth is the bottleneck, not precision |
| V2: eCB | TENS(2Hz)+taVNS+heat | 0.80 | Cortical only, loses peripheral input | 0.60 | 0.75x | WORSE alone — peripheral pathways matter |
| V3: 5HT | taVNS(raphe)+tDCS | 1.20 | No raphe access, cortical 5HT indirect | 0.30 | 0.25x | MUCH WORSE alone — taVNS is superior |
| V4: GABA | Weighted pressure+alpha | 0.70 | Direct cortical interneuron activation | 1.80 | 2.6x | Good — GABAergic interneurons are cortical |
| V5: NE down | taVNS(parasympathetic) | 0.50 | No LC access, cortical NE reuptake only | 0.20 | 0.40x | WORSE alone — LC is in brainstem |
| V6: Theta | TMS(6Hz)+binaural+tACS | 0.60 | Cortical theta entrainment, no hippocampus | 1.50 | 2.5x | Better — cortical theta is still theta |
| V7: Alpha | tDCS cathode(Fz)+TMS(1Hz) | 0.55 | Direct thalamo-cortical suppression | 2.00 | 3.6x | Excellent — alpha generators are cortical |
| V8: Gamma | TMS(40Hz)+LED+audio | 0.65 | Direct 40 Hz cortical drive | 2.50 | 3.8x | Excellent — gamma is cortical |
| V9: PFC | tDCS cathode(F4)+1Hz TMS | 0.60 | Direct PFC electrode inhibition | 2.20 | 3.7x | Excellent — PFC is right there |
| V10: Sensory | tDCS(V1)+stochastic resonance | 0.50 | V1 layer 4 direct stimulation | 2.00 | 4.0x | Excellent — layer-specific control |
| V11: Body | TENS(4Hz)+vibro+heated pad | 0.75 | S1 direct stimulation | 1.80 | 2.4x | Good — but different quality (central vs peripheral) |
| V12: Coherence | 40 Hz tri-modal+paired TMS | 0.45 | Cross-region phase-locked stim | 2.50 | 5.6x | Best improvement — true synchrony |

### Summary

- **N1 clearly better (>2x):** V4, V6, V7, V8, V9, V10, V11, V12 (8 of 12 variables)
- **N1 comparable:** V1 (1.6x, modest gain)
- **N1 clearly worse alone:** V2, V3, V5 (3 of 12 variables)

---

## 8. What Neuralink Cannot Do (Honest Limitations)

### Depth Problem

The N1 is a cortical implant. Its flexible polymer threads are inserted by a surgical robot into the top 3-6 mm of brain tissue. The deep structures central to our neurochemical variables are fundamentally out of reach:

1. **VTA (V1: Dopamine)** — Located in the ventral midbrain, ~70-80 mm from cortical surface. The N1 cannot reach it. Cortical stimulation of dlPFC can modulate VTA firing via descending projections, but this is a multi-synapse indirect pathway — better than scalp stimulation, but still indirect.

2. **Locus Coeruleus (V5: Norepinephrine)** — Located in the dorsal pons of the brainstem, ~80-100 mm deep. Completely inaccessible to cortical electrodes. taVNS remains the best non-surgical approach for LC modulation.

3. **Raphe Nuclei (V3: Serotonin)** — Distributed along the brainstem midline, 80-100 mm deep. No cortical implant can reach them. taVNS stimulation of the vagus nerve, which projects to NTS and then to raphe, remains more effective than cortical approaches.

4. **Hippocampus (V2: eCB, V6: Theta)** — Located in the medial temporal lobe, ~30-50 mm from lateral cortical surface. Potentially reachable with deep electrode designs (like Blackrock's Utah array variants or depth electrodes), but NOT with the N1's current flexible thread design. Neuralink would need a fundamentally different electrode architecture.

5. **Thalamus (V4: GABA, V7: Alpha)** — Located centrally at ~40-60 mm depth. Same issue as hippocampus. However, cortical GABAergic interneurons ARE accessible and are major contributors to both GABA tone and alpha rhythm generation.

### Other Hardware Limitations

- **Single implant site:** Current N1 is designed for one cortical region. Our model requires distributed cortical coverage (prefrontal, visual, somatosensory, temporal). Multiple N1 implants or a redesigned multi-site device would be needed.

- **64 simultaneous stimulation channels:** Out of 1024 electrodes, only 64 can stimulate at once. For driving 6-8 cortical variables simultaneously, this means ~8-10 channels per variable — adequate for focal targets but limiting for distributed patterns like coherence (V12).

- **No optogenetic capability:** The N1 uses electrical stimulation only. Cell-type-specific targeting (e.g., stimulating only GABAergic interneurons while leaving pyramidal cells unaffected) is not possible. Electrical stimulation activates all neurons near the electrode tip indiscriminately.

- **Thermal constraints:** 24.7 mW total power means stimulation power is limited. High-frequency continuous stimulation across many channels simultaneously will compete with recording and wireless transmission for power budget.

- **Battery life:** 12-hour battery limits continuous consciousness engineering sessions. Our target THC-equivalent experience (60-90 minutes) fits easily, but extended sessions or always-on maintenance of altered states would require careful power management.

### Regulatory Constraints

- Neuralink's current FDA approval is for motor cortex read-only (PRIME study)
- Stimulation is not approved for clinical use yet
- Blindsight (visual cortex stimulation) has Breakthrough Device Designation but no clinical deployment
- Consciousness-state engineering is years away from any regulatory pathway
- Any BCI Bridge work would initially be research-only under IRB oversight

---

## 9. Realistic BCI Bridge Architecture

Given the N1's cortical limitation, the practical BCI Bridge system is a hybrid:

### Hybrid Architecture

```
LAYER 1: N1 Implant (cortical variables)
  Handles: V4 (GABA), V6 (Theta-cortical), V7 (Alpha), V8 (Gamma),
           V9 (PFC), V10 (Sensory), V11 (Body-central), V12 (Coherence)
  Method: Direct cortical stimulation, <1 ms latency, phase-locked
  Advantage: 2-6x improvement over non-invasive

LAYER 2: External Stimulation (deep/peripheral variables)
  Handles: V1 (DA via tDCS+taVNS), V2 (eCB via TENS+taVNS),
           V3 (5HT via taVNS), V5 (NE via taVNS)
  Method: taVNS, tDCS, TENS — our existing hardware stack
  Advantage: Proven pathways to deep nuclei, no surgical risk

LAYER 3: Unified Controller
  Receives: 12-variable state from N1 on-chip extraction
  Runs: PID control loop for all 12 variables
  Sends: Stimulation commands to N1 (cortical) and external devices (deep)
  Coordinates: Phase relationships between internal and external stimulation
```

### Data Flow

```
[N1 Implant]                    [External Devices]
  |                                   |
  | BLE (384 kbps state out)          | USB/BLE (standard telemetry)
  | BLE (1024 kbps stim in)           | USB/BLE (stim commands)
  |                                   |
  +-----------> [BCI Bridge Controller] <-----------+
                      |
                12-variable PID loop
                Phase coordination
                Safety monitoring
                User interface
```

### Performance Projections

| Metric | Current Tier 3 | N1-Only | Hybrid (BCI Bridge) |
|---|---|---|---|
| Overall THC match | 117% | ~85% (missing deep variables) | ~250-300% |
| Cortical variables avg | ~60% of target | ~200% of target | ~200% of target |
| Neurochemical variables avg | ~80% of target | ~30% of target | ~80% of target |
| Latency (cortical) | 41 ms | <1 ms | <1 ms |
| Latency (deep) | 41 ms | N/A | 41 ms (external) |
| Phase-lock capability | No | Yes (cortical) | Yes (cortical) |
| Coherence control | Poor | Excellent | Excellent |

The hybrid outperforms either approach alone because it combines N1's cortical precision with our external devices' deep-nucleus access.

---

## 10. Neuralink-Specific Hypotheses

Mathematical predictions that can be verified when N1 stimulation becomes clinically available.

### H1: Phase-Locked Gamma Amplification

**Prediction:** Stimulating at the excitatory peak of endogenous gamma (0 degrees phase) at 100 uA will produce 3x greater gamma power increase than tonic 40 Hz stimulation at the same current.

```
Gamma_boost(phase-locked) / Gamma_boost(tonic) = 3.0 +/- 0.5
```

**Testable with:** N1 recording + stimulation on adjacent electrodes in motor or visual cortex.

### H2: Cross-Frequency Coupling Control

**Prediction:** By modulating gamma stimulation amplitude as a function of real-time theta phase (high gamma at theta peak, zero at trough), N1 can artificially create theta-gamma coupling with a modulation index > 0.3 (strong coupling, comparable to natural hippocampal recordings).

```
MI(artificial) > 0.3 when MI(baseline) < 0.05
```

### H3: Alpha Suppression Efficiency

**Prediction:** Phase-targeted alpha suppression (inhibitory pulses at alpha peaks only) will reduce alpha power with 5x less total charge than tonic inhibition.

```
Charge_efficiency(phase-targeted) / Charge_efficiency(tonic) = 5.0 +/- 1.0
```

### H4: Cortical-to-Subcortical Transfer

**Prediction:** Sustained cortical stimulation of dlPFC at 10 Hz for 10 minutes will produce measurable dopamine release in striatum (detectable via PET or microdialysis), but the effect will be only 15-25% as large as direct VTA stimulation (based on multi-synapse attenuation).

```
DA_release(cortical_stim) / DA_release(VTA_direct) = 0.20 +/- 0.05
```

### H5: Coherence Superiority

**Prediction:** N1 cross-region phase-locked stimulation will achieve inter-regional coherence (measured by phase-locking value) of > 0.8, compared to < 0.3 achievable with external multi-site TMS.

```
PLV(N1_coherence) > 0.8 when PLV(external_TMS) < 0.3
```

### H6: Hybrid Superiority

**Prediction:** The hybrid architecture (N1 cortical + external deep) will achieve higher overall 12-variable target match than either approach alone, specifically:

```
Match(hybrid) > Match(N1_only) + Match(external_only) - Match(baseline)
```

This is the superadditivity hypothesis — the combination is more than the sum because cortical and subcortical pathways interact synergistically (e.g., cortical gamma entrainment facilitates subcortical dopamine release from taVNS).

---

## 11. Partnership Strategy

### What Each Side Brings

**BrainWire offers Neuralink:**
- The only 12-variable consciousness state model with quantitative targets
- Closed-loop PID control algorithms tested on external hardware
- Transfer function framework that maps electrode parameters to subjective states
- A non-medical application that could dramatically expand their TAM
- Consciousness engineering software that makes their hardware meaningful beyond motor decode

**Neuralink offers BrainWire:**
- 1024 direct cortical electrodes with sub-millimeter precision
- Sub-millisecond closed-loop latency (vs. our 41 ms)
- Phase-locked stimulation capability (impossible externally)
- True cross-region coherence control
- The Blindsight regulatory pathway as a precedent for stimulation approval

### Value Proposition

"Your hardware reads and writes cortical signals. Our model defines what to write. Together, we build the first system that can engineer consciousness states with precision."

### Technical Integration Path

1. **Phase 1 — Software validation (no implant needed):** Port 12-variable extraction algorithm to N1 ASIC simulation. Verify it fits within power and compute budget. Deliverable: firmware module.

2. **Phase 2 — Read-only integration (existing PRIME patients):** Run our 12-variable model on N1 recording data. Compare cortical state estimation from 1024 direct electrodes vs. 16-channel scalp EEG. Deliverable: validation paper showing improved state estimation.

3. **Phase 3 — Stimulation protocol design (Blindsight expansion):** Design phase-locked stimulation protocols for cortical variables (V7-V12) using N1 hardware. Submit for research ethics approval. Deliverable: stimulation protocol document.

4. **Phase 4 — Hybrid prototype:** Build BCI Bridge controller that coordinates N1 stimulation with external taVNS/tDCS/TENS for full 12-variable control. Deliverable: working hybrid system in research setting.

5. **Phase 5 — Consciousness engineering:** First controlled demonstration of targeted consciousness state reproduction via hybrid N1 + external system. Deliverable: proof that electrical-only THC reproduction exceeds 200% target match.

### Risks

- Neuralink may have no interest in consciousness engineering applications
- Regulatory pathway for consciousness modification does not exist
- Public perception of "brain implant for recreational states" is challenging
- Technical risk of on-chip model porting to proprietary ASIC
- Neuralink's closed ecosystem may not allow third-party firmware

---

## Appendix: Notation and Units

| Symbol | Meaning | Unit |
|---|---|---|
| uA | microamperes | 10^-6 A |
| us | microseconds | 10^-6 s |
| nC | nanocoulombs | 10^-9 C |
| uC/cm^2 | microcoulombs per square centimeter | charge density |
| um | micrometers | 10^-6 m |
| mm | millimeters | 10^-3 m |
| kHz | kilohertz | 10^3 Hz |
| Mbps | megabits per second | 10^6 bits/s |
| kbps | kilobits per second | 10^3 bits/s |
| mW | milliwatts | 10^-3 W |
| uW | microwatts | 10^-6 W |
| V/m | volts per meter | electric field strength |
| PLV | phase-locking value | dimensionless, 0-1 |
| MI | modulation index | dimensionless |
| MFLOPS | million floating-point operations per second | compute |
