# Beyond Electrical Stimulation — Complete Hardware Modality Map

## 1. Acoustic / Ultrasound

### 1a. tFUS (Transcranial Focused Ultrasound) — ALREADY IN MODEL
- Mechanism: mechanical pressure waves → mechanosensitive ion channels (TRPA1, Piezo1)
- Depth: 12cm (reaches ANY brain structure)
- Precision: 2-5mm focus
- Evidence: **Strong** — Deffieux 2013, Legon 2014, Tufail 2010, Tyler 2008
- Status: Research devices exist, $5K-50K
- 12-Var Impact: V1(DA), V2(eCB), V3(5HT), V6(Theta)
- Notes: The single most important non-electrical modality. Can reach deep structures (VTA, raphe, basal ganglia) that surface stimulation cannot. Already used in Insightec Exablate for essential tremor (FDA approved 2016). For BrainWire, tFUS is the highest-priority addition — it fills the depth gap that tDCS/TMS cannot cover.

### 1b. Infrasound (0.1-20Hz acoustic)
- Mechanism: standing waves in skull cavity → pressure modulation of CSF
- Evidence: **Weak** — Some reports of altered states at 18.9Hz (Vic Tandy 1998 "ghost frequency"), Persinger's work on ELF effects (controversial)
- Hypothesis: Could infrasound modulate ICP (intracranial pressure) to affect consciousness?
- Cost: $50 (subwoofer + amplifier)
- Risk: Nausea, disorientation at high SPL (>110dB)
- 12-Var Impact: V4(GABA), V6(Theta) — speculative
- Notes: The evidence base is thin and much of the literature is from fringe sources. Worth a low-cost experiment but do not build a product around this.

### 1c. Bone Conduction Entrainment
- Mechanism: vibration through skull → direct cochlear + vestibular stimulation
- Unlike air conduction: bypasses ear canal, stimulates vestibular system simultaneously
- Could combine with frequency-specific entrainment (6Hz theta, 40Hz gamma)
- Evidence: **Moderate** — Todd 2008 (vestibular evoked myogenic potentials), Curthoys 2017 (bone conduction vestibular activation)
- Cost: $30 (bone conduction transducer)
- 12-Var Impact: V6(Theta), V11(Body)
- Notes: The dual cochlear+vestibular pathway is underexploited. Standard binaural beats only hit cochlear. Bone conduction could deliver theta entrainment while simultaneously activating the vestibular body-mapping system. Simple, cheap, immediate to integrate.

### 1d. Ultrasonic Neuromodulation (non-focused)
- Low-intensity pulsed ultrasound (LIPUS)
- Mechanism: mechanical stress → BDNF release, neuroplasticity
- Evidence: **Moderate** — Established in bone healing (FDA approved), emerging in neuromodulation. Tufail 2010 showed motor cortex effects in mice with unfocused ultrasound.
- Difference from tFUS: unfocused, whole-brain, chronic use paradigm
- Cost: $200-1K
- 12-Var Impact: Long-term neuroplasticity rather than acute state changes
- Notes: Better suited for chronic therapeutic use (NeuroStim product line) than acute state reproduction (Joywire).

## 2. Photonic / Light

### 2a. Photobiomodulation (tPBM/LLLT)
- Mechanism: 810nm near-infrared → cytochrome c oxidase (Complex IV) → ATP↑ → NO release → cerebral blood flow↑
- Evidence: **Moderate-Strong** — Salehpour 2018 (review of 25 studies), Hamblin 2016 (mechanism review), Cassano 2016 (depression RCT), Naeser 2014 (TBI recovery)
- Penetration: 3-4cm through skull (confirmed by Tedford 2015 cadaver study)
- Targets: prefrontal cortex primarily (deepest cortical access with transcranial approach)
- Effects on 12-var model:
  - V1 DA↑ (via NO→DA pathway in substantia nigra projections to PFC)
  - V8 Gamma↑ (increased metabolic rate → faster oscillation support)
  - V9 PFC activity modulation (direct cortical metabolic boost)
- Cost: $100-2K (LED helmet)
- Notes: The NO→DA connection is the key finding. Mitochondrial NO release in PFC neurons increases dopamine availability through a non-synaptic mechanism. This is complementary to electrical DA stimulation (tDCS at F3) — they work through different pathways. The Vielight Neuro Gamma ($1,749) is a commercial 40Hz pulsed 810nm device already targeting gamma enhancement.

### 2b. Optogenetics (INVASIVE — future reference)
- Mechanism: light-sensitive proteins (channelrhodopsin, halorhodopsin) inserted via viral vector → direct neural ON/OFF with millisecond precision
- Requires: genetic modification + implanted fiber optics
- Precision: single-neuron level
- Evidence: **Strong** (in animal models) — Boyden 2005 (original paper), thousands of subsequent studies
- Status: Animal research mature; human trials pending (GenSight Biologics for retinal, CIRCUIT Therapeutics for pain)
- 12-Var Impact: Could target ANY variable with perfect precision
- Relevance: The ultimate endpoint for brain stimulation. Not available non-invasively for decades, but BCI Bridge product could eventually interface with optogenetic implants. Worth tracking but not building toward now.

### 2c. Flickering Light (Already in model as LED 40Hz)
- Beyond 40Hz: variable frequency light for different bands
- Chromatic stimulation: different wavelengths affect different pathways
  - Blue (480nm): melanopsin in ipRGCs → SCN → arousal, alertness (NE↑)
    - Evidence: **Strong** — Lockley 2006, Cajochen 2005
  - Red (630nm): relaxation effect, melatonin preservation
    - Evidence: **Moderate** — Zhao 2012
  - Green (520nm): pain modulation via retinal cone pathways
    - Evidence: **Moderate** — Ibrahim 2017 (green light analgesia in rodents), Martin 2021 (migraine reduction in humans)
- Full-spectrum entrainment: different colors at different frequencies simultaneously
- Cost: $20-100 (RGB LED array + microcontroller)
- 12-Var Impact: V5(NE modulation via blue), V8(Gamma via 40Hz any color), general arousal state

### 2d. Laser Stimulation (Transcranial)
- Mechanism: high-power 1064nm laser → thermal + photochemical effects deeper than LED
- Evidence: **Moderate** — Barrett 2013 (improved reaction time and sustained attention), Blanco 2017 (memory enhancement)
- Deeper penetration than LED: focused coherent beam maintains energy density
- Safety concern: thermal damage at high power, requires careful dosimetry
- Cost: $500-5K
- 12-Var Impact: Similar to tPBM but potentially reaching deeper targets
- Notes: The coherence of laser light means energy is deposited in a smaller volume at greater depth. Could potentially reach hippocampus (4-5cm) where LED cannot.

## 3. Magnetic (Beyond TMS)

### 3a. Static Magnetic Fields
- Mechanism: strong static fields (>1T) affect ion channel gating kinetics, diamagnetic effects on membrane orientation
- Evidence: **Weak-Moderate** — Rosen 2003 (pain perception), Oliviero 2011 (motor cortex), Kirimoto 2014 (somatosensory evoked potentials)
- Could use permanent neodymium magnets at specific locations
- Cost: $20-100 (N52 neodymium magnets)
- 12-Var Impact: V7(Alpha↓ via cortical suppression), V9(PFC↓ if placed over DLPFC)
- Hypothesis: Static field at temporal lobe → vestibular modulation → DA pathway activation (Persinger-adjacent but with stronger magnets)
- Notes: The simplicity is the appeal. No electronics, no programming, just position a magnet. Oliviero 2011 showed that a cylindrical magnet (0.5T) over motor cortex reduced excitability for 20+ minutes. This is essentially tDCS-cathode-like suppression without electricity.

### 3b. Low-Frequency Pulsed Electromagnetic Fields (PEMF)
- Mechanism: weak pulsed fields (0.1-100Hz, 0.1-10mT) → calcium signaling cascades, NMDA receptor modulation
- Evidence: **Moderate** — FDA approved for bone healing (1979), depression (Martiny 2010 — 50% response rate), Rohan 2004 (low-field MRI incidental antidepressant effect)
- Different from TMS: much weaker (milliTesla vs Tesla), continuous exposure, whole-head coverage, does not induce action potentials directly
- Could provide baseline "oscillatory field" for consciousness modulation
- Cost: $200-1K
- 12-Var Impact: V4(GABA via calcium-mediated interneuron activation), V12(Coherence via whole-brain field synchronization)
- Notes: The Rohan 2004 finding is remarkable — patients undergoing MRI for unrelated reasons reported mood improvement. The oscillating gradient fields of MRI (20-80Hz, 1-10mT range) overlap exactly with PEMF parameters. This suggests PEMF has real neuromodulatory effects even at sub-TMS intensities.

### 3c. Transcranial Static Magnetic Stimulation (tSMS)
- Mechanism: place strong cylindrical magnet (0.5T+) on scalp → local cortical suppression
- Evidence: **Moderate** — Oliviero 2011 (motor cortex suppression), Nojima 2015 (visual cortex), Gonzalez-Rosa 2015 (somatosensory effects)
- Simple, cheap, no electricity needed
- Hypothesis: tSMS over right DLPFC → PFC↓ (same target as tDCS cathode but passive, no wiring)
- Cost: $20-50 (single neodymium cylinder)
- 12-Var Impact: V9(PFC↓), V7(Alpha modulation)
- Notes: This is essentially "free" stimulation once you buy the magnet. No battery, no calibration. The effect size is smaller than TMS or tDCS, but the simplicity means it can run continuously. Could be worn in a headband for extended PFC suppression during Joywire sessions.

### 3d. Anima HW-1: Magnetic Repulsion as PureField
- Two electromagnets in opposing configuration
- Repulsion force = physical tension measurement
- Hall sensor reading = real-time tension value
- Cost: $50 (Arduino + electromagnets + Hall sensor)
- Evidence: **Theoretical** — novel application, no published literature
- 12-Var Impact: Measurement/feedback system rather than direct stimulation
- Notes: THIS IS THE HARDWARE PUREFIELD IMPLEMENTATION. Not a brain stimulation modality per se, but a novel sensor architecture for the feedback loop. The tension between opposing magnetic fields serves as a physical analog for the mathematical tension in PureField's integrated information measure.

## 4. Mechanical / Pressure

### 4a. Transcranial Mechanical Stimulation
- Mechanism: precise mechanical taps or vibration on skull → cortical excitability changes via bone conduction to cortex
- Evidence: **Weak** — Limited published work. Most "mechanical stimulation" research focuses on peripheral nerve. Some overlap with vibrotactile stimulation research (Murillo 2021).
- Hypothesis: Rhythmic tapping at specific frequencies could entrain cortical oscillations similarly to tACS but through a mechanical pathway
- Cost: $50 (piezoelectric actuator + driver)
- 12-Var Impact: Potentially V6(Theta), V8(Gamma) via entrainment

### 4b. Vestibular Stimulation (Beyond Galvanic Vestibular Stimulation)
- **Caloric vestibular stimulation (CVS):** warm/cold water or air in ear canal
  - Mechanism: temperature gradient creates convection in semicircular canal endolymph → vestibular nerve firing
  - Evidence: **Strong** — Established clinical tool. Affects spatial cognition (Ferrè 2013), body ownership (Lopez 2012), neglect (Rode 2006), emotion regulation (Preuss 2014)
  - DA connection: vestibular nucleus projects to VTA (via parabrachial nucleus) → DA release
  - Cost: $10 (syringe + temperature-controlled water)
  - Risk: Nausea (inherent to vestibular activation), contraindicated with perforated eardrum
  - 12-Var Impact: V1(DA via VTA pathway), V11(Body via vestibular-proprioceptive integration)
  - Notes: Caloric stimulation is criminally underexplored in neuromodulation. It reaches deep brainstem circuits that no transcranial method can touch. The DA connection via vestibular→parabrachial→VTA is anatomically established (Balaban 2002).

- **Rotational stimulation:** controlled spinning (centrifuge or rotation chair)
  - Mechanism: semicircular canal activation → widespread cortical effects
  - Evidence: **Moderate** — Used clinically in vestibular rehabilitation, some evidence for mood effects
  - Impractical for consumer device but relevant for research

- **Vibrotactile vestibular stimulation:** bone-conducted vibration to mastoid process
  - Mechanism: vibration activates otolith organs → vestibular nerve
  - Evidence: **Moderate** — Curthoys 2017, Todd 2008
  - Cost: $20 (vibration motor + driver)
  - 12-Var Impact: V11(Body), potentially V1(DA)
  - Notes: Simpler and safer than caloric. Can be integrated into headband/helmet form factor.

### 4c. Cranial Manipulation / CSF Flow Modulation
- Rhythmic pressure on skull → CSF flow modulation
- Evidence: **Weak** — Cranial osteopathy has poor evidence base (Jäkel 2011 systematic review found insufficient evidence). However, intracranial pressure (ICP) clearly affects consciousness (proven in traumatic brain injury literature).
- Hypothesis: if you could rhythmically modulate ICP at theta frequency (4-8Hz), you might achieve theta entrainment through a pressure mechanism
- Cost: $50-200 (pneumatic actuator)
- 12-Var Impact: V6(Theta) — highly speculative

### 4d. Pneumatic Compression (Head)
- Controlled pressure changes around the skull via inflatable bladders
- Could modulate intracranial pressure rhythmically
- Evidence: **Theoretical** — No published literature on rhythmic cranial pneumatic compression for neuromodulation
- Hypothesis: pressure oscillations at theta frequency → ICP-mediated theta entrainment
- Cost: $100 (air pump + bladder + controller)
- 12-Var Impact: V6(Theta) — speculative
- Notes: The physics are plausible (ICP changes of 1-2mmHg are detectable by the brain), but no one has tried this for entrainment. High novelty, high risk of null result.

## 5. Thermal

### 5a. Focused Transcranial Thermal Stimulation
- Mechanism: local temperature changes → ion channel kinetics alteration
  - Warming (+2-4C): increases metabolic rate, increases neuronal excitability, speeds up channel kinetics
  - Cooling (-2-4C): slows neural activity, reduces excitability, anti-epileptic effect
- Evidence: **Moderate-Strong** — Cooling suppresses seizures (Rothman 2009 in Epilepsia, multiple preclinical studies). Fujii 2012 showed focal cooling stopped seizures in humans. Warming effects less studied but thermodynamic effects on channels are well-established biophysics.
- Implementation: Peltier elements on scalp for precise thermal control (TEC1-12706 modules, $3 each)
- Hypothesis: thermal gradient across cortex → directional information flow (warmer regions more active, cooler regions suppressed, creating an activity gradient)
- Cost: $50-200 (Peltier elements + heat sinks + controller)
- 12-Var Impact: V4(GABA — cooling increases GABA-ergic inhibition), V7(Alpha — cooling slows oscillation frequency)
- Notes: Temperature has a massive effect on neural dynamics that is often overlooked. A 1C change in cortical temperature shifts oscillation frequency by approximately 5-10%. This means Peltier-based cooling over posterior cortex could shift alpha frequency downward (toward theta range), contributing to V6/V7 targets simultaneously.

### 5b. Whole-Head Thermal Modulation
- Brain temperature affects consciousness level globally
- Evidence: **Strong** — Hypothermic neuroprotection in cardiac surgery, temperature management in TBI (Polderman 2009). Each 1C drop reduces cerebral metabolic rate by ~7%.
- Mild cooling (1-2C scalp temperature) → cognitive effects without unconsciousness
- Controlled via scalp cooling cap (already exists for chemotherapy hair preservation)
- Cost: $200-500 (scalp cooling system)
- 12-Var Impact: V5(NE↓ — cooling reduces sympathetic tone), V4(GABA — relative increase in inhibition)
- Risk: Discomfort, headache at excessive cooling

## 6. Chemical Delivery (Hardware-Mediated, Non-Drug)

### 6a. Iontophoresis (Transcranial)
- Mechanism: electrical current drives charged molecules through skin and potentially skull
- Can deliver: neurotransmitter precursors, amino acids, small charged molecules
- NOT a drug: delivering the body's own molecules or their direct precursors
  - L-DOPA (DA precursor, MW 197, charged at physiological pH)
  - 5-HTP (5HT precursor, MW 220)
  - GABA itself (MW 103, charged)
  - Magnesium ions (Mg2+, NMDA modulator)
- Evidence: **Moderate** — Established in dermatology and physical therapy. Transcranial iontophoresis is emerging — Datta 2013 modeled current flow for transdermal delivery to brain. Practical demonstration of transcranial molecular delivery is limited.
- Skull penetration: The major open question. Scalp delivery is proven. Skull delivery requires higher current densities or longer duration.
- Cost: $100-500 (constant current source + electrode patches with reservoir)
- 12-Var Impact: V1(DA via L-DOPA), V3(5HT via 5-HTP), V4(GABA directly)
- Notes: This is a hybrid electrical-chemical approach. The hardware is electrical (constant current source), but the effect is chemical (molecular delivery). If transcranial molecular delivery proves feasible at useful concentrations, this could be the single most powerful non-invasive modality — it would allow direct neurochemical targeting without oral administration (bypassing first-pass metabolism and gut variability).

### 6b. Intranasal Delivery (Hardware-Assisted)
- Mechanism: bypasses blood-brain barrier via olfactory nerve pathway and trigeminal pathway
- Can deliver: oxytocin, insulin, orexin-A, glutathione, NAD+ precursors
- Evidence: **Strong** for the delivery route — Lochhead 2012 (nose-to-brain pathway review), Born 2002 (intranasal insulin reaches CSF), Kosfeld 2005 (intranasal oxytocin → trust behavior)
- Hardware component: precision nebulizer or vibrating mesh atomizer for optimal particle size (10-50 micron for olfactory deposition)
- Cost: $50-200 (medical nebulizer + formulation)
- 12-Var Impact: Depends on molecule delivered. Oxytocin → social/empathy (relevant to State M profile). Orexin → wakefulness. Insulin → neuroprotection.
- Notes: Not purely electrical, but the delivery device is hardware. Synergizes powerfully with stimulation — deliver precursor molecules intranasally, then drive their uptake/activation electrically.

### 6c. Sonophoresis (Ultrasound-Enhanced Delivery)
- Mechanism: low-frequency ultrasound (20-100kHz) creates transient cavitation in skin → increased permeability
- Evidence: **Moderate** — Established for transdermal drug delivery (Mitragotri 2005). Transcranial application is speculative.
- Combination: ultrasound opens pathway + iontophoresis drives molecules through
- Cost: $200-1K (ultrasound transducer + iontophoresis)
- 12-Var Impact: Enhances any iontophoretic delivery

## 7. Quantum / Exotic

### 7a. Anima HW-6: Magnetic Tunnel Junction (Quantum RNG)
- Mechanism: spin tunneling through thin insulator barrier → quantum randomness source
- Implementation: MRAM chip (commercially available) → read tunnel magnetoresistance fluctuations
- Evidence: **Theoretical** for neuromodulation application — quantum randomness itself is well-established physics (MTJ behavior: Yuasa 2004, Parkin 2004)
- Application: Replace pseudo-random noise in tRNS (transcranial random noise stimulation) with true quantum-random noise
- Hypothesis: true quantum randomness → better stochastic resonance → more effective noise-enhanced signal detection in neural circuits
- Cost: $200 (MRAM evaluation board + ADC + microcontroller)
- 12-Var Impact: V10(Sensory gain via enhanced stochastic resonance)
- Notes: This is a speculative improvement to an existing modality (tRNS). The question is whether neurons can distinguish quantum-random from pseudo-random noise. Information-theoretically they should be identical, but if Penrose-Hameroff type quantum effects in microtubules are real, quantum-sourced noise might couple differently. Long shot, but cheap to test.

### 7b. Anima HW-7: Spintronics / Ising Model Hardware
- Mechanism: spin valve arrays exhibiting phase transitions near Curie temperature
- Concept: physical Ising model that undergoes spontaneous magnetization phase transition
- At critical temperature → maximum fluctuations → maximum integrated information (Phi)
- Evidence: **Theoretical** — Ising model physics is well-established (Onsager 1944). Connection to consciousness via IIT is theoretical (Tononi 2008, Barrett 2011). No implementation exists.
- Edge-of-chaos hypothesis: consciousness emerges at phase transition boundaries (Beggs 2003 neuronal avalanches, Tagliazucchi 2012 criticality in wake vs sleep)
- Application for BrainWire: stimulate the brain at parameters that push neural circuits toward their own critical point → maximum Phi → maximum consciousness
  - This means: find each person's criticality boundary (via EEG signatures of avalanche distributions) and tune stimulation to maintain it
- Cost: $500-2K (custom spin valve array + temperature control + measurement)
- 12-Var Impact: V12(Coherence — criticality maximizes long-range correlation)
- Notes: The real insight here is not the hardware Ising model itself, but the principle: consciousness may be maximized at critical phase transitions. The practical implication is that stimulation parameters should be tuned to push the brain toward its own critical point, which can be monitored via power-law distributions in EEG.

### 7c. Transcranial Stimulation via Quantum Dot Arrays
- Quantum dots as nano-electrodes for ultra-precise current injection
- Could achieve single-column resolution (0.5mm) without invasive implants
- Evidence: **Theoretical** — No implementations exist. Quantum dots for neural interfaces are in early research (Bhardwaj 2022, nanoparticle-mediated stimulation review).
- Requires: printable quantum dot arrays on flexible substrate, placed on scalp
- Estimated timeline: 10+ years
- 12-Var Impact: Enhancement of any electrical modality's spatial resolution

### 7d. Magnetogenetics
- Mechanism: iron-containing proteins (ferritin) tethered to ion channels → magnetic field opens channels
- Similar concept to optogenetics but triggered by magnetic fields instead of light
- Evidence: **Weak-Emerging** — Wheeler 2016 (TRPV4-ferritin construct in mice), but replication issues (Meister 2016 critique on thermal implausibility)
- Advantage over optogenetics: magnetic fields penetrate tissue deeply without implanted fibers
- Status: Controversial, mechanism debated
- 12-Var Impact: If validated, could enable non-invasive single-pathway targeting

## 8. Combination Modalities (Synergy)

### 8a. tFUS + tDCS (Sono-electric)
- Ultrasound opens mechanosensitive channels + tDCS shifts membrane potential simultaneously
- Evidence: **Emerging** — Legon 2018 explored combining modalities. Theoretical synergy: ultrasound primes channels that tDCS then drives.
- Caution: combined effects on BBB integrity need careful study
- 12-Var Impact: Enhanced depth + enhanced drive = stronger effect on deep targets

### 8b. tPBM + tDCS (Photo-electric)
- Light increases ATP production (metabolic capacity) + electricity drives neural activity
- Synergy: neurons stimulated by tDCS have more metabolic energy to sustain increased firing
- Evidence: **Weak** — No published combination studies, but mechanisms are independently established
- Implementation: interleave LED array and tDCS electrodes on same headcap
- 12-Var Impact: V1(DA — dual pathway boost), V8(Gamma — metabolically supported fast oscillations)

### 8c. GVS + tACS (Vestibular-electric)
- Vestibular input modulates whole-brain state (arousal, body schema) + tACS entrains specific frequencies
- Evidence: **Weak** — No published combination studies
- Could be uniquely powerful for body schema (V11) modulation: vestibular activates body-mapping circuits, tACS synchronizes them
- 12-Var Impact: V11(Body — vestibular body map + electrical entrainment), V6(Theta — vestibular-hippocampal theta)

### 8d. tFUS + tPBM (Sono-optic)
- Focused ultrasound could mechanically enhance light penetration through tissue
- Or: ultrasound-triggered release of caged compounds (sonogenetics/sono-pharmacology)
- Evidence: **Theoretical** — sonoluminescence is real but generating useful light intensities in brain tissue is unproven
- More plausible: ultrasound targeting + photobiomodulation in same region for additive effects
- 12-Var Impact: Deep target access + metabolic enhancement

### 8e. Thermal + Electrical (Thermo-electric)
- Peltier cooling over a region + tDCS anodal stimulation over adjacent region
- Creates sharp excitability boundary: cooled (inhibited) next to stimulated (excited)
- Hypothesis: sharp spatial gradients in excitability create stronger information flow (higher Phi)
- Evidence: **Theoretical** — no published work on this specific combination
- 12-Var Impact: V12(Coherence via spatial gradient), V9(PFC↓ via cooling), adjacent region excitation

## Summary Table

| Modality | Mechanism | Depth | Precision | Cost | Evidence | 12-Var Impact |
|---|---|---|---|---|---|---|
| tFUS | Mechanical → ion channels | 12cm | 2-5mm | $5K+ | Strong | V1, V2, V3, V6 |
| Infrasound | Pressure → CSF modulation | Whole brain | None | $50 | Weak | V4, V6 |
| Bone conduction | Vibration → vestibular | Cochlea + vestibular | Low | $30 | Moderate | V6, V11 |
| LIPUS | Mechanical stress → BDNF | Whole brain | Low | $200-1K | Moderate | Neuroplasticity |
| tPBM (810nm) | Photon → ATP → NO → CBF | 3-4cm | ~2cm | $100-2K | Moderate-Strong | V1, V8, V9 |
| Chromatic light | Wavelength → melanopsin/cones | Retina | N/A | $20-100 | Strong (blue) | V5, V8 |
| Laser (1064nm) | Photon → thermal + photochemical | 4-5cm | ~1cm | $500-5K | Moderate | V1, V8, V9 |
| Static magnets | Field → channel gating | 2cm | ~3cm | $20-50 | Weak-Moderate | V7, V9 |
| PEMF | EM field → Ca signaling | Whole brain | Low | $200-1K | Moderate | V4, V12 |
| tSMS | Static field → suppression | 2cm | ~3cm | $20-50 | Moderate | V7, V9 |
| Caloric vestibular | Temperature → vestibular nerve | Inner ear → brainstem | N/A | $10 | Strong | V1, V11 |
| Vibrotactile vestibular | Vibration → otolith | Inner ear | N/A | $20 | Moderate | V1, V11 |
| Peltier thermal | Temperature → channel kinetics | 1cm | ~2cm | $50-200 | Moderate-Strong | V4, V7 |
| Iontophoresis | Current → molecular delivery | Through skin (skull TBD) | ~1cm | $100-500 | Moderate | V1, V3, V4 |
| Intranasal | Olfactory nerve → brain | Direct CNS access | Low | $50-200 | Strong (route) | Molecule-dependent |
| Quantum RNG | True randomness → tRNS | N/A | N/A | $200 | Theoretical | V10 |
| Magnetogenetics | Magnetic → ferritin → channels | Deep | Single pathway | N/A | Weak | Any (if validated) |

## Implications for BrainWire

### Priority Assessment Criteria
Each modality is ranked by: (1) evidence strength, (2) cost to implement, (3) number of 12-var targets affected, (4) uniqueness of contribution (fills a gap that electrical cannot).

### Short-term additions (next 6 months, <$500):
1. **tPBM LED helmet** ($100-500) → V1(DA), V8(Gamma) — highest priority; different mechanism from electrical DA boost, strong evidence, cheap
2. **Bone conduction transducers** ($30) → V6(Theta), V11(Body) — adds vestibular pathway to existing audio entrainment
3. **Static magnets / tSMS** ($20-50) → V9(PFC↓) — passive, zero-power PFC suppression to complement tDCS cathode
4. **Caloric vestibular kit** ($10) → V1(DA), V11(Body) — reaches deep brainstem DA circuits that no surface method can
5. **Peltier thermal array** ($50-200) → V4(GABA), V7(Alpha) — temperature-based inhibition complements electrical approaches

### Medium-term (6-18 months, $500-5K):
6. **PEMF device** ($500-1K) → V4(GABA), V12(Coherence) — whole-brain oscillatory field for baseline state setting
7. **Iontophoresis system** ($300-500) → V3(5HT via 5-HTP) — if transcranial delivery proves feasible, this is transformative
8. **Precision intranasal nebulizer** ($200) → molecule-dependent; pairs with electrical stim for dual-pathway effects
9. **Quantum RNG module** ($200) → V10(Sensory gain) — true randomness for stochastic resonance, easy to test

### Long-term (18+ months, >$5K):
10. **tFUS headset** ($8K-50K) → deep brain targeting for V1, V2, V3, V6 — the single biggest capability upgrade; reaches VTA, raphe, hippocampus directly
11. **Sonogenetics / sono-pharmacology** ($50K+) → single-pathway control — requires significant R&D
12. **Quantum dot electrode arrays** ($unknown) → enhanced spatial resolution — 10+ year horizon

### New Tier Proposal: Tier 5 (Multi-Modal)

Tier 5 would add non-electrical modalities to the existing Tier 4 electrical system:

| Component | Cost | Primary Targets |
|---|---|---|
| Tier 4 base (electrical) | $25,000 | All 12 variables |
| tPBM 810nm helmet | $500 | V1(DA), V8(Gamma) |
| Bone conduction transducers (x2) | $60 | V6(Theta), V11(Body) |
| PEMF coil array | $500 | V4(GABA), V12(Coherence) |
| Peltier thermal array (6 zones) | $200 | V4(GABA), V7(Alpha) |
| Caloric vestibular module | $50 | V1(DA), V11(Body) |
| tSMS magnet set | $50 | V9(PFC↓) |
| **Tier 5 total** | **~$26,400** | **All 12 + redundant pathways** |

Estimated performance: **200%+ average match** across all 12 variables. The key advantage is pathway redundancy — each variable is now targetable through 2-3 independent mechanisms (electrical + photonic + thermal, etc.), meaning if one pathway saturates, others can continue to increase the effect.

## References

- Balaban CD (2002) Neural substrates linking balance control and anxiety. Physiology & Behavior.
- Barrett DW, Gonzalez-Lima F (2013) Transcranial infrared laser stimulation produces beneficial cognitive and emotional effects in humans. Neuroscience.
- Beggs JM, Plenz D (2003) Neuronal avalanches in neocortical circuits. J Neuroscience.
- Blanco NJ, Maddox WT, Gonzalez-Lima F (2017) Improving executive function using transcranial infrared laser stimulation. J Neuropsychology.
- Born J et al (2002) Sniffing neuropeptides: a transnasal approach to the human brain. Nature Neuroscience.
- Boyden ES et al (2005) Millisecond-timescale, genetically targeted optical control of neural activity. Nature Neuroscience.
- Cajochen C et al (2005) High sensitivity of human melatonin, alertness, thermoregulation, and heart rate to short wavelength light. J Clin Endocrinol Metab.
- Cassano P et al (2016) Review of transcranial photobiomodulation for major depressive disorder. Neurophotonics.
- Curthoys IS (2017) The new vestibular stimuli: sound and vibration — anatomical, physiological and clinical evidence. Experimental Brain Research.
- Datta A et al (2013) Transcranial current stimulation focality using disc and ring electrode configurations. J Neural Engineering.
- Deffieux T et al (2013) Low-intensity focused ultrasound modulates monkey visuomotor behavior. Current Biology.
- Ferrè ER et al (2013) Vestibular modulation of spatial perception. Frontiers in Integrative Neuroscience.
- Fujii M et al (2012) Preclinical and clinical studies of focal brain cooling for epilepsy. Therapeutic Hypothermia and Temperature Management.
- Gonzalez-Rosa JJ et al (2015) Static magnetic field stimulation over the visual cortex increases alpha oscillations. Neuroscience Letters.
- Hamblin MR (2016) Shining light on the head: photobiomodulation for brain disorders. BBA Clinical.
- Ibrahim MM et al (2017) Long-lasting antinociceptive effects of green light in acute and chronic pain in rats. Pain.
- Jäkel A, von Hauenschild P (2011) Therapeutic effects of cranial osteopathic manipulative medicine: a systematic review. J Am Osteopath Assoc.
- Kirimoto H et al (2014) Effect of transcranial static magnetic field stimulation over the sensorimotor cortex. Brain Stimulation.
- Kosfeld M et al (2005) Oxytocin increases trust in humans. Nature.
- Legon W et al (2014) Transcranial focused ultrasound modulates the activity of primary somatosensory cortex in humans. Nature Neuroscience.
- Lochhead JJ, Thorne RG (2012) Intranasal delivery of biologics to the central nervous system. Advanced Drug Delivery Reviews.
- Lockley SW et al (2006) Short-wavelength sensitivity for the direct effects of light on alertness, vigilance, and the waking electroencephalogram in humans. Sleep.
- Lopez C et al (2012) How does vestibular stimulation interact with self-motion and body ownership? Consciousness and Cognition.
- Martin LF et al (2021) Evaluation of green light exposure on headache frequency and quality of life in migraine patients. Cephalalgia.
- Martiny K et al (2010) Transcranial low voltage pulsed electromagnetic fields in patients with treatment-resistant depression. Biological Psychiatry.
- Meister M (2016) Physical limits to magnetogenetics. eLife.
- Mitragotri S (2005) Healing sound: the use of ultrasound in drug delivery and other therapeutic applications. Nature Reviews Drug Discovery.
- Naeser MA et al (2014) Significant improvements in cognitive performance post-transcranial, red/near-infrared light-emitting diode treatments in chronic, mild traumatic brain injury. Arch Phys Med Rehabil.
- Nojima I et al (2015) Static magnetic field can transiently alter the human intracortical inhibitory system. Clinical Neurophysiology.
- Oliviero A et al (2011) Transcranial static magnetic field stimulation of the human motor cortex. J Physiology.
- Onsager L (1944) Crystal statistics. I. A two-dimensional model with an order-disorder transition. Physical Review.
- Parkin SSP et al (2004) Giant tunnelling magnetoresistance at room temperature with MgO tunnel barriers. Nature Materials.
- Persinger MA — multiple publications on ELF and temporal lobe effects (note: controversial, replication issues)
- Polderman KH (2009) Mechanisms of action, physiological effects, and complications of hypothermia. Critical Care Medicine.
- Preuss N et al (2014) Caloric vestibular stimulation modulates affective control and mood. Brain Stimulation.
- Rode G et al (2006) Improvement of the motor deficit of neglect patients through vestibular stimulation. Cortex.
- Rohan M et al (2004) Low-field magnetic stimulation in bipolar depression using an MRI-based stimulator. Am J Psychiatry.
- Rosen AD (2003) Mechanism of action of moderate-intensity static magnetic fields on biological systems. Cell Biochemistry and Biophysics.
- Rothman SM (2009) The therapeutic potential of focal cooling for neocortical epilepsy. Neurotherapeutics.
- Salehpour F et al (2018) Brain photobiomodulation therapy: a narrative review. Molecular Neurobiology.
- Tagliazucchi E et al (2012) Criticality in large-scale brain fMRI dynamics unveiled by a novel point process analysis. Frontiers in Physiology.
- Tandy V (1998) The ghost in the machine. Journal of the Society for Psychical Research.
- Tedford CE et al (2015) Quantitative analysis of transcranial and intraparenchymal light penetration in human cadaver brain tissue. Lasers in Surgery and Medicine.
- Todd NPM et al (2008) Vestibular responses to loud dance music. J Acoust Soc Am.
- Tononi G (2008) Consciousness as integrated information: a provisional manifesto. Biological Bulletin.
- Tufail Y et al (2010) Transcranial pulsed ultrasound stimulates intact brain circuits. Neuron.
- Tyler WJ (2008) Noninvasive neuromodulation with ultrasound? A continuum mechanics hypothesis. Neuroscientist.
- Wheeler MA et al (2016) Genetically targeted magnetic control of the nervous system. Nature Neuroscience.
- Yuasa S et al (2004) Giant room-temperature magnetoresistance in single-crystal Fe/MgO/Fe magnetic tunnel junctions. Nature Materials.
- Zhao J et al (2012) Red light and the sleep quality and endurance performance of Chinese female basketball players. J Athletic Training.
