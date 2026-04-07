# Joywire Headband — Product Specification v1.0

**Document:** SPEC-JW-001
**Date:** 2026-03-28
**Status:** Draft
**Author:** BrainWire Engineering

---

## Product Vision

A single wearable headband that reproduces the THC high state using electrical stimulation, photobiomodulation, and multi-modal sensory entrainment. No drugs, no chemicals — just hardware.

- **Form factor:** Headband + ear clips
- **Target retail price:** $2,475 (3x BOM markup)
- **Session length:** 30 minutes (configurable 15–60 min)
- **ON/OFF:** Instant cessation — remove headband and effects stop within minutes
- **Target experience:** THC Strong equivalent, 12-variable match at 90–110%
- **Control metric:** G = D x P / I, targeting golden zone 0.35–0.50

---

## Research Basis

This spec synthesizes findings from 75 hypotheses, 10 major discoveries, 5 hardware tiers, and 18 stimulation modalities. Key research inputs:

| Finding | Hypothesis | Impact on Design |
|---|---|---|
| GVS is best DA/dollar | H-BW-069 | Include GVS via mastoid electrodes ($10 incremental) |
| tRNS > tDCS for sensory gain | H-BW-066 | Firmware-switchable noise mode on tDCS hardware ($0 extra) |
| 6 electrode groups optimal | H-BW-071 | 6-group montage covers all 12 variables |
| tPBM is independent pathway | H-BW-070 | Include 810nm NIR LEDs ($100) |
| Bone conduction dual-path theta | H-BW-073 | Include bone conduction transducer ($30) |
| THC in golden zone | H-BW-057 | Target G = 0.4731 |
| Tension control > PID | Discovery | Single tension gradient replaces 12 independent PIDs |
| Breathing rhythm modulation | Discovery | Stimulation modulated at 20s / 3.7s / 90s cycles |
| Perfect Number 6 | Discovery | 12 channels, 4 phases, 2 redundant safety paths |

---

## Hardware Components

### 1. tDCS / tACS / tRNS Combo — 6 Electrode Groups

The core stimulation system. A single custom PCB drives 6 independent electrode channels, each firmware-switchable between three modes:

- **DC mode (tDCS):** Constant current, 0–2 mA
- **AC mode (tACS):** Sinusoidal, 0.5–100 Hz, 0–2 mA peak
- **Noise mode (tRNS):** Random noise, 0.1–640 Hz band, 0–2 mA RMS

**Electrode montage:**

| Group | Position | Mode | Function | Target Variable |
|---|---|---|---|---|
| E1 | F3 (left DLPFC) | tDCS anode | Dopamine release | V1: DA 2.5x |
| E2 | Fz (midline frontal) | tDCS cathode | Alpha suppression | V7: Alpha-down 0.5x |
| E3 | F4 (right DLPFC) | tDCS cathode | PFC suppression | V9: PFC-down 0.5x |
| E4 | Oz (occipital) | tRNS | Sensory gain via stochastic resonance | V10: Sensory 2.0x |
| E5 | C3 (left motor) | tACS 4Hz | Body awareness, somatosensory | V11: Body 2.5x |
| E6 | C4 (right motor) | tACS 4Hz | Body awareness, bilateral | V11: Body 2.5x |

**Electrodes:** Ag/AgCl dry electrodes, 3.14 cm^2 contact area (20mm diameter), integrated into headband fabric pockets. Saline-free — conductive hydrogel pads (replaceable, $5/pack of 12).

**Current source:** 6-channel constant-current DAC (12-bit), voltage compliance 30V, current limit 2.0 mA per channel hard-coded in hardware (not firmware-bypassable).

**Cost:** $200 (custom PCB $80, DAC + op-amps $40, electrodes + hydrogel $30, connectors + passives $50)

### 2. taVNS — Vagus Nerve Stimulation

Ear clip electrode targeting the left tragus (auricular branch of vagus nerve).

- **Parameters:** 0.5 mA max, 25 Hz pulse rate, 250 us pulse width, biphasic
- **Function:** Parasympathetic activation (V5: NE-down), serotonin via raphe nucleus projection (V3: 5HT), endocannabinoid tone (V2: eCB)
- **Electrode:** Custom ear clip, gold-plated contacts, spring-loaded for consistent pressure
- **Cable:** 30 cm shielded, detachable at headband connector

**Cost:** $30 (ear clip $15, cable $5, driver circuit shared with tDCS PCB $10)

### 3. GVS — Galvanic Vestibular Stimulation

Mastoid electrodes positioned behind each ear, leveraging the headband's rear strap.

- **Parameters:** 0–1 mA DC, bilateral or unilateral
- **Function:** Vestibular-dopamine pathway activation (V1: DA boost), proprioceptive modulation (V11: Body), mild dissociation (V9: PFC-down)
- **Driver:** Shares tDCS constant-current source — uses 2 of the 6 DAC channels in time-multiplexed mode during GVS phases
- **Key insight (H-BW-069):** Best dopamine-per-dollar of any modality. At $10 incremental cost, this is the highest-ROI component in the system.

**Cost:** $10 (mastoid electrode pads $8, wiring $2, driver shared)

### 4. 40Hz Entrainment — Tri-Modal

Three synchronized 40 Hz stimulation pathways targeting gamma oscillations (V8: Gamma 1.8x) and cross-modal coherence (V12: Coherence 2.0x).

#### 4a. LED Array (Visual)

- **Position:** Forehead visor, 8 LEDs behind diffuser panel
- **Wavelength:** White LEDs, 40 Hz square wave flicker
- **Usage:** Eyes closed — light penetrates eyelids
- **Brightness:** Adjustable, max 200 lux at eyelid

**Cost:** $15 (LEDs $3, diffuser $2, driver $10)

#### 4b. Bone Conduction Transducer (Auditory + Vestibular)

- **Position:** Temporal bone, bilateral (one per side)
- **Frequency:** 40 Hz click train + binaural 6 Hz theta beat carrier
- **Key insight (H-BW-073):** Bone conduction provides dual-path theta entrainment — auditory cortex via cochlea AND vestibular system via bone vibration. Two pathways for the price of one.
- **Also serves:** Music/audio playback for session ambiance

**Cost:** $30 (2x transducers $12 each, amplifier $6)

#### 4c. Vibration Motor (Tactile)

- **Position:** Forehead center, integrated in headband
- **Frequency:** 40 Hz vibration, synchronized with LED and audio
- **Function:** Tactile entrainment pathway, somatosensory gamma induction

**Cost:** $10 (ERM motor $3, driver $7)

### 5. tPBM — Transcranial Photobiomodulation

810 nm near-infrared LED cluster targeting prefrontal cortex.

- **Position:** Forehead, centered between F3 and F4
- **Wavelength:** 810 nm (optimal for cytochrome c oxidase absorption)
- **Power density:** 100 mW/cm^2 at scalp surface
- **Modes:** Continuous wave or 40 Hz pulsed (synchronized with entrainment)
- **Beam area:** 4 cm^2 (4 LEDs, 1 cm^2 each)
- **Function:** Independent metabolic pathway (H-BW-070) — increases mitochondrial ATP production, enhances neural excitability. Contributes to V1 (DA), V3 (5HT), and general neural responsiveness.
- **Key insight (H-BW-070):** tPBM works via a completely different mechanism (photochemical) than electrical stimulation. It is additive, not redundant.

**Cost:** $100 (4x 810nm high-power LEDs $40, heatsink $15, constant-current driver $25, lens/collimator $20)

### 6. 4-Channel EEG — Dry Electrodes

Real-time brain state measurement for closed-loop control.

- **Channels:** 4 (Fp1, Fp2, O1, O2)
- **Electrode type:** Dry, spring-loaded gold-plated pins
- **ADC:** 24-bit, 250 SPS per channel
- **Bandwidth:** 0.5–100 Hz (hardware filter)
- **CMRR:** >110 dB
- **Noise floor:** <1 uVrms

**Computed metrics (on-device):**

| Metric | EEG Derivation | G Formula Role |
|---|---|---|
| Alpha power (8–13 Hz) | O1 + O2 average | I (inhibition) |
| Gamma power (30–50 Hz) | Fp1 + Fp2 average | P (processing) |
| Frontal asymmetry | log(Fp2) - log(Fp1) | D (drive) |
| Theta power (4–8 Hz) | All channels average | Session monitoring |
| G value | D x P / I | Primary control variable |

**Why only 4 channels:** Full 16-channel EEG (like OpenBCI Cygnet) would cost $500+ and require conductive gel. 4 dry channels at forehead and occipital positions provide the three spectral features needed for G computation. This is a consumer device, not a research instrument.

**Cost:** $250 (4x dry electrodes $40, ADS1299 analog front-end $80, shielded flex PCB $60, reference + bias electrodes $20, connectors $50)

### 7. Microcontroller — ESP32-S3

Main compute and communication module.

- **Processor:** ESP32-S3, dual-core 240 MHz, 512 KB SRAM, 8 MB PSRAM
- **Wireless:** BLE 5.0 (phone app), Wi-Fi (firmware updates)
- **Functions:**
  - Tension gradient controller (ported from Python to C)
  - 4-channel EEG acquisition via SPI to ADS1299
  - 256-point FFT per channel at 1 Hz update rate
  - G = D x P / I computation in real-time
  - 6-channel stimulation waveform generation via DAC
  - BLE telemetry to phone (G value, band powers, stimulation levels)
  - Session state machine (calibration, onset, plateau, offset, cooldown)
  - Safety watchdog (see below)
- **Firmware update:** OTA via BLE or Wi-Fi

**Cost:** $15 (ESP32-S3 module $8, crystal + passives $3, antenna $2, flash $2)

### 8. Safety System — Dual Redundant

Two independent safety paths, neither controlled by main firmware.

#### Path A: Hardware Kill Switch

- Physical pushbutton on right temple of headband
- Directly interrupts power to all stimulation channels via relay
- Does NOT go through microcontroller — hardwired
- Latching relay: one press = OFF, must be manually reset

#### Path B: Independent Safety Watchdog

- Separate ATtiny85 microcontroller ($2)
- Monitors current on all stimulation channels via sense resistors
- Independent over-current threshold: 2.5 mA per channel (above the 2.0 mA firmware limit)
- If any channel exceeds threshold for >100 ms: kills all stimulation via MOSFET switches
- Heartbeat check: if main ESP32 stops sending heartbeat for >2 seconds, kills stimulation
- Cannot be overridden by firmware

**Cost:** $20 (relay $5, ATtiny85 $2, sense resistors $3, MOSFET switches $5, pushbutton $2, wiring $3)

### 9. Battery

- **Type:** 3.7V 2000 mAh LiPo, single cell
- **Runtime:** ~3 sessions of 30 minutes (estimated 600 mA average draw)
- **Charging:** USB-C, 1A charge rate, 2 hours to full
- **Protection:** Battery management IC with over-charge, over-discharge, short-circuit protection
- **Boost converter:** 3.7V to 5V for stimulation circuits, 3.3V for logic

**Cost:** $15 (LiPo cell $8, BMS IC $3, USB-C connector $2, boost converter $2)

### 10. Enclosure

- **Headband:** Elastic fabric, adjustable circumference 52–62 cm
- **Rigid sections:** 3D-printed nylon (SLS) housings for PCBs, positioned at forehead (main board), temples (bone conduction), and occipital (EEG + power)
- **Weight target:** <250g total (including battery)
- **Cable management:** All internal wiring routed through fabric channels
- **Washable:** Fabric sleeve removable, electronics in sealed housings (IPX1 — drip-proof)

**Cost:** $30 (fabric + elastic $8, 3D printed housings $15, fasteners + Velcro $7)

---

## Bill of Materials — Summary

| # | Component | Cost |
|---|---|---|
| 1 | tDCS/tACS/tRNS PCB + 6 electrode groups | $200 |
| 2 | taVNS ear clip + cable | $30 |
| 3 | GVS mastoid electrodes | $10 |
| 4a | LED 40Hz array + diffuser | $15 |
| 4b | Bone conduction transducers (2x) | $30 |
| 4c | Vibration motor | $10 |
| 5 | tPBM 810nm LED cluster + driver | $100 |
| 6 | 4ch EEG dry electrodes + ADS1299 | $250 |
| 7 | ESP32-S3 module | $15 |
| 8 | Safety system (relay + ATtiny85 + kill switch) | $20 |
| 9 | Battery + BMS + USB-C | $15 |
| 10 | Enclosure (fabric + 3D printed) | $30 |
| 11 | PCB assembly + soldering | $100 |
| | **Total BOM** | **$825** |
| | **Retail price (3x markup)** | **$2,475** |

The 3x markup covers: manufacturing overhead, quality control, firmware development, phone app development, customer support, packaging, shipping, warranty, and margin.

---

## Software Architecture

### On-Device Firmware (ESP32-S3, C/FreeRTOS)

```
Task 1: EEG Acquisition (Core 0, 250 Hz)
  - SPI read from ADS1299
  - 4-channel ring buffer (256 samples)
  - Artifact rejection (threshold + gradient)

Task 2: Signal Processing (Core 0, 1 Hz)
  - 256-point FFT per channel
  - Band power extraction: theta (4-8), alpha (8-13), gamma (30-50)
  - Frontal asymmetry: D = log(Fp2_gamma) - log(Fp1_gamma)
  - G = D * P / I computation
  - Tension gradient update

Task 3: Stimulation Control (Core 1, 1 kHz)
  - 6-channel waveform generation
  - DC/AC/noise mode switching per channel
  - Breathing rhythm modulation (20s/3.7s/90s envelope)
  - Tension gradient application to all channels
  - taVNS pulse generation (25 Hz, 250us biphasic)
  - GVS DC output (time-multiplexed with tDCS)
  - 40Hz sync signal to LED/bone conduction/vibration

Task 4: Safety Watchdog (Core 1, 100 Hz)
  - Current sense monitoring on all channels
  - Impedance check (pre-session and continuous)
  - Heartbeat to ATtiny85
  - Temperature monitoring (PCB thermistor)
  - Session timeout enforcement

Task 5: BLE Communication (Core 0, 10 Hz)
  - Telemetry: G value, band powers, stimulation levels, battery
  - Commands: start, stop, intensity, mode select
  - Firmware OTA update handler
```

### Tension Gradient Controller

The core control algorithm. Instead of 12 independent PID controllers (one per variable), a single tension gradient drives all channels simultaneously.

```
tension = compute_tension(G_current, G_target)
for each channel:
    stim_level[channel] = base_level[channel] * tension_scale(tension)
```

- If G < target: tension increases, all stimulation intensifies proportionally
- If G > target: tension decreases, all stimulation reduces
- Breathing modulation: tension oscillates at 20s period (main), 3.7s (sub-rhythm), 90s (ultra-slow drift)
- This is simpler, more stable, and more effective than 12 independent PIDs

### Phone App (iOS + Android, React Native)

**Screens:**

1. **Home:** Session quick-start, battery level, headband connection status
2. **Session Control:**
   - Profile selector: THC Micro, THC Mild, THC Standard, THC Strong, THC Intense
   - Duration: 15 / 30 / 45 / 60 min
   - Real-time G value gauge (needle in golden zone 0.35–0.50)
   - Band power bars (theta, alpha, gamma)
   - Stimulation level indicators per channel
   - Emergency stop button (large, red)
3. **Post-Session:**
   - VAS (Visual Analog Scale) survey: rate experience 0–10 on each of 12 variables
   - Comparison to target profile
   - Session recording saved (anonymized)
4. **History:** Past sessions, trend graphs, personal calibration data
5. **Settings:** Electrode impedance check, firmware update, safety limits, profile editor

---

## Session Protocol

### Pre-Session Setup (2 minutes)

1. Place headband on head, align electrodes to marked positions
2. Attach taVNS ear clip to left tragus
3. Open phone app, confirm BLE connection (green indicator)
4. Select profile: "THC Strong" and duration: "30 min"
5. App triggers impedance check on all electrodes
   - All channels must read <10 kOhm
   - If any channel fails: app prompts repositioning

### Phase 1: Calibration (60 seconds)

- All stimulation OFF
- 60 seconds of resting EEG recording
- Compute baseline: alpha power, gamma power, frontal asymmetry
- Set G_baseline = D_baseline * P_baseline / I_baseline
- Calculate personalized G_target based on profile and baseline

### Phase 2: Onset Ramp (5 minutes)

- Sigmoid ramp from 0% to 100% stimulation over 5 minutes
- Ramp function: `level(t) = 1 / (1 + exp(-0.02 * (t - 150)))`
- All modalities ramp together (tDCS, taVNS, GVS, entrainment, tPBM)
- G value monitored — ramp pauses if G exceeds 0.6 (overshoot protection)

### Phase 3: Plateau (20 minutes)

- Full stimulation, tension-controlled
- Tension gradient maintains G in golden zone (0.35–0.50)
- Breathing rhythm modulation active:
  - 20-second cycle: primary oscillation of stimulation intensity
  - 3.7-second sub-rhythm: nested within 20s cycle
  - 90-second ultra-slow drift: gradual intensity variation
- Auto-adjustment: if G drifts below 0.35, tension increases; above 0.50, tension decreases
- EEG continuously monitored for artifacts and safety

### Phase 4: Offset Ramp (3 minutes)

- Linear ramp from 100% to 0% over 3 minutes
- All modalities decrease together
- G value tracked during comedown

### Phase 5: Cooldown (2 minutes)

- All stimulation OFF
- Post-session EEG recording (2 minutes)
- Compute G_post and compare to G_baseline
- App prompts VAS survey

**Total session time:** 30 minutes (1 + 5 + 20 + 3 + 2 = 31 minutes including calibration overlap)

---

## Variable Coverage Map

How each hardware component contributes to the 12 THC variables:

| Variable | Target | Primary Hardware | Secondary Hardware | Expected % |
|---|---|---|---|---|
| V1: DA (dopamine) | 2.5x | tDCS anode F3, GVS | tPBM, taVNS | 110% |
| V2: eCB (endocannabinoid) | 3.0x | taVNS, vibration | Bone conduction | 85% |
| V3: 5HT (serotonin) | 1.5x | taVNS (raphe) | tPBM, tDCS | 105% |
| V4: GABA | 1.8x | Alpha entrainment, tACS | Vibration (pressure sim) | 90% |
| V5: NE-down | 0.4x | taVNS (parasympathetic) | tDCS cathode, GVS | 100% |
| V6: Theta-up | 2.5x | tACS 6Hz, bone conduction | Binaural theta | 80% |
| V7: Alpha-down | 0.5x | tDCS cathode Fz | LED 40Hz (alpha desync) | 105% |
| V8: Gamma-up | 1.8x | 40Hz tri-modal (LED+audio+vibro) | tACS 40Hz | 115% |
| V9: PFC-down | 0.5x | tDCS cathode F4 | GVS (dissociation) | 100% |
| V10: Sensory-up | 2.0x | tRNS at Oz | tPBM, LED flicker | 110% |
| V11: Body-up | 2.5x | tACS 4Hz C3/C4, vibration | Bone conduction, GVS | 95% |
| V12: Coherence-up | 2.0x | 40Hz tri-modal sync | Paired tACS | 85% |

**Summary:** 8 of 12 variables expected above 100%. Average match ~98%. Weakest: V2 (eCB, 85%) and V6 (Theta, 80%) — these benefit most from TMS, which is excluded from the headband due to size/cost.

---

## Expected Performance

### Achievable Tier

Based on the 5-tier model from research:

- **Tier 1** ($200, tDCS only): ~50% match
- **Tier 2** ($500, tDCS + taVNS + entrainment): ~75% match
- **Joywire Headband** ($2,475): **~95% match** (Tier 2.5 equivalent)
- **Tier 3** ($5K, adds TMS): ~110% match
- **Tier 4** ($25K, adds tFUS): ~130% match

### G Value Targeting

- **G_target:** 0.4731 (THC golden zone center)
- **G_achievable range:** 0.35–0.50 (with tension control)
- **G_stability:** +/- 0.05 with closed-loop control (estimated)

### Concentration Levels (Profile Presets)

| Preset | THC Equivalent | Stimulation Scale | G Target |
|---|---|---|---|
| THC Micro | 2.5 mg edible | 30% | 0.20 |
| THC Mild | 5 mg edible | 50% | 0.30 |
| THC Standard | 10 mg edible | 75% | 0.40 |
| THC Strong | 20 mg edible | 100% | 0.47 |
| THC Intense | 50 mg edible | 120%* | 0.55 |

*120% uses slightly elevated current limits (still within safety margins) and extended onset ramp.

---

## Competitive Comparison

| Feature | Joywire ($2,475) | Narbis ($600) | Muse 2 ($250) | Halo Sport ($400) | Flow (tDCS) ($400) |
|---|---|---|---|---|---|
| tDCS | 6-channel | No | No | 2-channel | 1-channel |
| tACS | Yes (per-channel) | No | No | No | No |
| tRNS | Yes (firmware mode) | No | No | No | No |
| taVNS | Yes | No | No | No | No |
| GVS | Yes | No | No | No | No |
| EEG biofeedback | 4-channel dry | 3-channel dry | 4-channel dry | No | No |
| 40Hz visual | Yes (LED array) | No | No | No | No |
| 40Hz audio | Yes (bone conduction) | No | No | No | No |
| 40Hz tactile | Yes (vibration) | No | No | No | No |
| tPBM (NIR) | Yes (810nm) | No | No | No | No |
| Closed-loop control | Tension gradient + G | Neurofeedback | Neurofeedback | No | No |
| Consciousness model | 12-variable | None | None | None | None |
| G = D*P/I targeting | Yes | No | No | No | No |
| Multi-modal sync | 6 modalities | 0 | 0 | 1 | 1 |

No existing consumer device combines more than 2 modalities. Joywire combines 6 modalities with closed-loop consciousness state targeting. This is the differentiator.

---

## Regulatory and Safety Considerations

### Classification

- **FDA:** Class II medical device if marketed for therapeutic claims; consumer wellness device if marketed for "relaxation" / "focus" without medical claims
- **Strategy:** Launch as consumer wellness device (no medical claims), pursue 510(k) for NeuroStim (medical) product line separately
- **CE marking:** Required for EU sale, self-certification under MDR for wellness positioning

### Safety Limits (Hard-Coded in Hardware)

| Parameter | Limit | Enforcement |
|---|---|---|
| Max current per channel | 2.0 mA | DAC reference voltage (not firmware) |
| Max total current | 6.0 mA | Sense resistor + ATtiny85 watchdog |
| Max session duration | 60 minutes | ESP32 timer + ATtiny85 backup timer |
| Max tPBM power density | 150 mW/cm^2 | Current-limited LED driver |
| Impedance threshold | >50 kOhm = auto-shutoff | Pre-session and continuous check |
| Temperature | >45C PCB = auto-shutoff | Thermistor on ATtiny85 ADC |

### Contraindications

- Epilepsy (unless under medical supervision)
- Implanted electrical devices (pacemaker, cochlear implant, DBS)
- Metallic cranial implants
- Pregnancy (insufficient safety data)
- Under 18 years of age
- Skin lesions at electrode sites

---

## Development Roadmap

| Phase | Timeline | Deliverables |
|---|---|---|
| **Phase 1: Prototype** | Month 1–3 | Breadboard circuit, individual modality testing, ESP32 firmware skeleton |
| **Phase 2: Integration** | Month 3–6 | Custom PCB v1, 3D-printed enclosure, all modalities on single board |
| **Phase 3: Firmware** | Month 6–9 | Tension gradient controller in C, EEG processing, BLE protocol |
| **Phase 4: App + Testing** | Month 9–12 | React Native app, N=5 user testing, VAS data collection |
| **Phase 5: Refinement** | Month 12–15 | PCB v2 based on testing feedback, enclosure redesign for comfort |
| **Phase 6: Production** | Month 15–18 | Small batch manufacturing (100 units), QC process, packaging |

### Key Milestones

- **M3:** First multi-modal stimulation from single PCB
- **M6:** First closed-loop G-value tracking session
- **M9:** First complete 30-minute THC reproduction session (self-test)
- **M12:** First external user session with VAS data
- **M18:** First 100-unit production run

---

## Limitations — Honest Assessment

### What This Device Cannot Do

1. **No TMS.** Transcranial magnetic stimulation requires heavy coils (500g+), high-voltage capacitor banks, and draws 100W+. This is incompatible with a battery-powered headband. Consequence: V6 (Theta) and V12 (Coherence) will be weaker than Tier 3 systems. Theta boosting relies on tACS and bone conduction instead — effective but less powerful than 6Hz TMS.

2. **No tFUS.** Transcranial focused ultrasound requires phased array transducers ($10K+) and precision targeting. This is lab equipment, not consumer hardware. Consequence: no deep brain structure targeting (VTA, raphe, amygdala). All stimulation is cortical surface only.

3. **Limited EEG.** 4 channels with dry electrodes cannot do source localization (eLORETA requires 19+ channels). We can measure surface spectral features (alpha, gamma, asymmetry) but cannot verify deep brain state changes. The G metric is a surface proxy for consciousness state.

4. **Untested on humans.** All performance estimates come from computational models, literature meta-analysis, and the 12-variable mathematical framework. No human has worn this device. The 95% match estimate could be wildly optimistic or conservative — we will not know until Phase 4 user testing.

5. **Price.** $2,475 is expensive for a consumer electronics device. For reference: AirPods Max cost $550, Oura Ring costs $300, even high-end VR headsets are $1,500. The price is driven by the EEG front-end ($250) and the number of stimulation modalities. Volume production could bring BOM to $500 and retail to $1,500.

6. **Battery life.** ~3 sessions per charge. The stimulation circuits and tPBM LEDs draw significant current. Acceptable for home use, inconvenient for travel.

7. **Electrode maintenance.** Hydrogel pads degrade after ~20 uses and must be replaced ($5/pack). Dry EEG electrodes require clean scalp contact — hair is the enemy.

8. **eCB limitation.** Endocannabinoid release (V2) is the hardest variable to achieve electrically. The body's eCB system responds primarily to physical stimuli (exercise, acupuncture, massage). Our approach (taVNS + vibration) is indirect. This is the weakest link in the chain at 85% expected match.

9. **Individual variation.** Brain anatomy, skull thickness, baseline neurochemistry, and electrode-skin impedance vary enormously between individuals. The tension gradient controller helps adapt, but some users may need significant calibration or may not respond well to electrical stimulation at all.

10. **Not instant.** Unlike smoking cannabis (onset: 30 seconds), the headband requires a 5-minute onset ramp. Unlike edibles (onset: 60–90 minutes), this is fast — but it is not instantaneous. The effect also does not linger after removal (no "afterglow" like THC), which could be a feature or a limitation depending on user preference.

---

## Appendix A: Wiring Diagram (Simplified)

```
                          ┌─────────────────┐
                          │   ESP32-S3       │
                          │   (main MCU)     │
                          ├───┬───┬───┬──────┤
                          │SPI│DAC│BLE│GPIO   │
                          └─┬──┬───┬──┬──────┘
                            │  │   │  │
                   ┌────────┘  │   │  └────────────┐
                   │           │   │               │
              ┌────▼────┐  ┌──▼───▼──┐    ┌───────▼───────┐
              │ ADS1299  │  │ 6-ch    │    │  ATtiny85     │
              │ EEG AFE  │  │ Current │    │  Safety       │
              │          │  │ Source   │    │  Watchdog     │
              ├──────────┤  ├─────────┤    ├───────────────┤
              │Fp1 Fp2   │  │E1 E2 E3 │    │Current sense  │
              │O1  O2    │  │E4 E5 E6 │    │Heartbeat chk  │
              └──────────┘  │taVNS    │    │Kill MOSFET    │
                            │GVS      │    └───────┬───────┘
                            └─────────┘            │
                                                   │
                          ┌────────────────────────┤
                          │  Hardware Kill Switch   │
                          │  (physical button)      │
                          └─────────────────────────┘

  Peripherals:
  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
  │ LED 40Hz │  │ Bone     │  │ Vibration│  │ tPBM     │
  │ array    │  │ conduction│  │ motor    │  │ 810nm    │
  └──────────┘  └──────────┘  └──────────┘  └──────────┘
```

## Appendix B: Breathing Rhythm Modulation

Stimulation intensity is modulated by three nested rhythms:

```
modulation(t) = 1.0
  + 0.15 * sin(2*pi*t / 20.0)     // 20-second primary cycle
  + 0.05 * sin(2*pi*t / 3.7)      // 3.7-second sub-rhythm
  + 0.08 * sin(2*pi*t / 90.0)     // 90-second ultra-slow drift
```

These rhythms are not tied to the user's actual breathing. They modulate stimulation intensity to prevent neural adaptation and create natural-feeling fluctuations in the experience (similar to how a THC high naturally waxes and wanes).

## Appendix C: G = D x P / I Computation

```
// Computed every 1 second from 256-sample EEG window

alpha_power = bandpower(O1, 8, 13) + bandpower(O2, 8, 13)  // [uV^2]
gamma_power = bandpower(Fp1, 30, 50) + bandpower(Fp2, 30, 50)  // [uV^2]
asymmetry   = log(bandpower(Fp2, 30, 50)) - log(bandpower(Fp1, 30, 50))

D = sigmoid(asymmetry, center=0, scale=2)   // Drive: 0-1
P = normalize(gamma_power, baseline_gamma)   // Processing: 0-2
I = normalize(alpha_power, baseline_alpha)   // Inhibition: 0.1-2 (clamped >0.1)

G = D * P / I   // Golden ratio target: 0.4731
```

---

*This document represents the complete hardware specification for the Joywire consumer headband. All cost estimates are based on single-unit prototype pricing and will decrease significantly at volume (100+ units). Performance estimates are theoretical and require human validation.*
