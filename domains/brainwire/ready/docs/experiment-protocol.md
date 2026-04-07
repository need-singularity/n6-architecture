# Experimental Validation Protocol

**BrainWire — Joywire THC Reproduction**
**Document status:** Protocol design (pre-experiment)
**Last updated:** 2026-03-28

---

## 1. The Core Question

**Can simultaneous electrical stimulation across 12 neurochemical and brainwave targets reproduce the subjective experience of a THC high?**

### What We Know

- Transcranial direct current stimulation (tDCS) modulates cortical excitability. Published, replicated.
- Transcutaneous vagus nerve stimulation (taVNS) influences norepinephrine and serotonin pathways. Published, moderate evidence.
- Transcranial alternating current stimulation (tACS) can entrain neural oscillations at target frequencies. Published, emerging.
- TENS at low frequencies can trigger endorphin release. Published, well-established for pain.
- 40 Hz gamma entrainment via light and sound is feasible. Published (Tsai/Bhatt labs).
- EEG can measure theta, alpha, and gamma power with reasonable reliability. Well-established.

### What We Assume (Unvalidated)

- That modulating neural correlates of a conscious state is sufficient to reproduce that state. **This is a massive assumption.** Neural correlates may be necessary but not sufficient. The relationship between measurable brain activity and subjective phenomenology is the hard problem of consciousness, and we do not have a solution.
- That our 12-variable model captures the relevant dimensions of the THC experience. This model was derived from neuropharmacology literature, not from empirical dimension reduction of actual THC experiences.
- That effects of simultaneous multi-site stimulation are additive or at least predictable. In reality, interactions between stimulation modalities are poorly characterized.
- That surface-level stimulation (non-invasive) can reach the relevant deep brain structures with sufficient specificity. tDCS current spread is diffuse. We are not targeting individual nuclei.
- That the subjective THC experience is consistent enough across individuals to have a single target profile. Inter-individual variability in drug response is enormous.

### The Gap

There is a fundamental gap between:
1. "We changed theta power by X%" (measurable)
2. "This feels like being high on THC" (subjective)

No amount of EEG data bridges this gap without asking the person what they experience. That is why subjective reports are our **primary outcome measure**, not EEG.

### Why This Experiment Matters

If even partial reproduction works (Level 2-3), it demonstrates that:
- Conscious experience can be steered by non-invasive hardware
- A parametric model of subjective states has practical utility
- Non-pharmacological alternatives to recreational substances are feasible

If it fails completely, that is equally valuable. It tells us that correlates are not causes, and we need a fundamentally different approach.

---

## 2. Experimental Design

### Design Type

- **Within-subject:** Each participant serves as their own control, eliminating inter-individual variability in baseline brain states.
- **Single-blind minimum:** Participant does not know which condition they receive. Double-blind is preferable but difficult when the experimenter must configure hardware.
- **Sham-controlled:** Sham condition uses identical electrode placement with brief current ramp (30s on, then off) to mimic sensation without sustained stimulation.
- **Randomized order:** Condition sequence randomized per participant to control for order effects and expectation buildup.

### Conditions

| Condition | Description | Purpose |
|---|---|---|
| A — Sham | Electrodes placed, brief ramp, no sustained current | Baseline / placebo control |
| B — Single variable | Dopamine pathway only: tDCS at F3, 2mA | Does one variable do anything? |
| C — Partial (6 vars) | DA + eCB + 5HT + Theta + Alpha + Gamma | Does adding variables improve experience? |
| D — Full (12 vars) | Complete protocol, all variables targeted | Full model test |

### Washout

- Minimum 48 hours between sessions to prevent carryover effects.
- No caffeine 4 hours before session. No alcohol or cannabis 24 hours before.
- Sessions at consistent time of day per participant.

### Sample Progression

| Stage | N | Purpose | Go/No-Go Criterion |
|---|---|---|---|
| N=1 self-pilot | 1 (researcher) | Safety, feasibility, protocol debugging | No serious adverse events |
| Small pilot | 5 | Effect detection, sham discrimination | Sham detection >50% in real conditions |
| Powered study | 20 | Effect size estimation, condition comparison | Pre-registered analysis plan |

Progression to each stage requires meeting the go criterion of the previous stage. Do not skip stages.

---

## 3. Hardware Setup by Phase

### Phase 0: N=1 Self-Pilot (Week 1-2)

**Hardware:** Tier 1 only (~$85)
- tDCS unit (e.g., TheBrainDriver v2 or DIY with LM334 current source): $40-60
- TENS unit (consumer grade, adjustable frequency): $25
- Arduino Nano + LED strip + piezo buzzer + vibration motor: $20

**Protocol:**
- Week 1: Single variables only, one per day
  - Day 1: tDCS F3 anode, 1mA (50% target), 10 min
  - Day 2: TENS forearm, 2 Hz, low intensity, 10 min
  - Day 3: 40 Hz LED flicker only, 10 min
  - Day 4: 40 Hz audio click train only, 10 min
  - Day 5: Vibration motor on wrist, 4 Hz, 10 min
  - Day 6-7: Rest, journal review
- Week 2: Combinations of 2-3 variables, 15 min sessions
  - Gradually layer effects, noting interactions

**Data collection:** Unstructured daily journal. The question is simply: "Does anything feel different at all?" No formal metrics yet. This is exploratory.

**Safety:** Start all intensities at 50% of literature-reported effective levels. Ramp on over 60 seconds. Emergency off-switch within arm's reach at all times.

### Phase 1: Controlled Pilot (Week 3-6)

**Hardware:** Tier 1 + optional EEG (~$335 total)
- Same Tier 1 hardware as Phase 0
- OpenBCI Ganglion 4-channel EEG ($250) if budget allows
- Electrodes: Fz, Cz, Pz, O1 (midline + occipital)

**Protocol:**
- 4 sessions per participant (conditions A-D), randomized order
- Each session: 5 min baseline, 20 min stimulation, 10 min post-monitoring
- Sham condition: 30-second ramp to target current, then ramp down. This produces the initial scalp tingling that makes sham credible.
- Minimum 48-hour washout between sessions

**Data collection:** Formal VAS scales (see Section 4), EEG recording, heart rate, free-text reports.

### Phase 2: Escalation (Month 2-3)

**Additional hardware:** (~$180 added)
- taVNS unit (targeting left auricular branch of vagus nerve): ~$100
- tACS capability (modify tDCS or dedicated unit for AC waveforms): ~$80

**Protocol:**
- Repeat conditions C and D with taVNS and tACS added
- Compare VAS scores: Does more hardware produce better subjective match?
- This is the critical test of the "more variables = better" assumption

**New variables accessible:**
- V3 (5HT) via taVNS at tragus
- V5 (NE reduction) via taVNS parasympathetic activation
- V6 (Theta) via tACS at 6 Hz

### Phase 3: Full Stack (Month 4-6)

**Additional hardware:** (~$5K-8K added)
- TMS unit (if accessible through university collaboration or used equipment)
- OpenBCI Cyton 16-channel EEG ($1K) for proper spatial resolution
- Pulse oximeter, GSR sensor for autonomic monitoring

**Protocol:**
- Full 12-variable protocol as specified in the model
- Direct comparison condition: participants who have used THC within past month rate both experiences on identical VAS scales
- Formal pre-registration on OSF or AsPredicted before data collection

**This phase only proceeds if Phase 2 shows at least Level 2 effects.** If Phase 2 shows nothing, spending $5K+ on TMS is not justified.

---

## 4. Measurements

### Primary Outcome: Subjective Reports (VAS 0-100)

These are rated immediately after each session on visual analog scales.

| Dimension | Anchors (0 / 100) | THC-relevant? |
|---|---|---|
| Euphoria | "No mood change" / "Intense euphoria" | Core |
| Body sensation | "Normal body feeling" / "Intense warmth, tingling, heaviness" | Core |
| Sensory enhancement | "Normal perception" / "Colors vivid, sounds rich, touch amplified" | Core |
| Time distortion | "Normal time sense" / "Time dramatically slowed or distorted" | Core |
| Altered thinking | "Normal thought patterns" / "Highly creative, divergent, associative" | Core |
| Relaxation | "Normal tension" / "Profoundly relaxed" | Core |
| Anxiety | "No anxiety" / "Intense anxiety" | Safety (should be LOW) |
| THC similarity | "Nothing like THC" / "Indistinguishable from THC" | Primary (experienced users only) |

Additional per-session items:
- **Sham detection:** "Do you believe you received real stimulation? (Y/N, confidence 0-100)"
- **Free-text:** "Describe your experience in your own words."
- **Adverse effects checklist:** Headache, dizziness, nausea, skin irritation, visual disturbance, mood change (each rated 0-10)

### Secondary Outcome: Objective Measures

These support but do not replace subjective reports.

**EEG (when available):**
- Theta power (4-8 Hz) at Fz — target increase
- Alpha power (8-12 Hz) at Pz — target decrease
- Gamma power (30-50 Hz) at Cz — target increase
- G = D x P / I ratio (golden zone metric) — exploratory
- Pre vs. during vs. post comparison

**Autonomic:**
- Heart rate (resting, during, post) — expect mild decrease
- Skin conductance — expect decrease (relaxation)

**Cognitive:**
- Psychomotor vigilance task (reaction time) — expect mild slowing
- Alternate uses task (divergent thinking) — expect increase in fluency

---

## 5. Safety Protocol

### Stimulation Parameters: Hard Limits

| Modality | Maximum Current | Maximum Duration | Maximum Frequency |
|---|---|---|---|
| tDCS | 2.0 mA | 30 min | N/A (DC) |
| tACS | 2.0 mA peak-to-peak | 30 min | 40 Hz |
| taVNS | 1.0 mA | 30 min | 25 Hz |
| TENS | Per device spec | 30 min | 100 Hz |
| TMS | Per device spec | 20 min | 40 Hz |

These limits are within published safety guidelines (Bikson et al. 2016, Antal et al. 2017). Do not exceed them under any circumstances.

### Ramp Protocol

1. All electrical stimulation starts at 0 and ramps to target over minimum 60 seconds.
2. First session for any new modality starts at 50% of target intensity.
3. Intensity increases by max 25% per session.
4. Full target intensity not reached until session 3 at earliest.

### Emergency Procedures

- **Hardware kill switch:** Physical toggle within arm's reach that cuts all current to zero. Not a software button. A physical switch.
- **Session abort criteria (immediate stop):**
  - Participant requests stop (for any reason, no questions asked)
  - Visible skin redness >2cm at electrode site
  - Reported sharp pain (not tingling) at electrode site
  - Dizziness requiring lying down
  - Nausea
  - Visual disturbance not caused by LED flicker
  - Heart rate >120 bpm or <45 bpm
- **Post-abort:** Monitor for 30 minutes. Log event. No further stimulation that day.

### Exclusion Criteria

Do not proceed with any participant (including self) who has:
- History of epilepsy or seizures
- Cardiac pacemaker or other implanted electrical device
- Metallic cranial implants
- Pregnancy (or possibility of pregnancy)
- Current psychiatric medication (SSRIs, benzodiazepines, antipsychotics)
- History of adverse reaction to electrical stimulation
- Open wounds or skin conditions at electrode sites
- Consumed alcohol within 12 hours or cannabis within 24 hours

### Adverse Event Logging

Every session ends with an adverse event check. Any event rated >3/10 gets documented with:
- Description, onset time, duration, severity
- Relationship to stimulation (definite/probable/possible/unlikely)
- Action taken
- Resolution

Any serious adverse event (lasting >30 min post-session, requiring medical attention, or involving loss of consciousness) halts the entire protocol until reviewed.

---

## 6. Success Criteria

### Honest Thresholds

| Level | Criterion | What It Means |
|---|---|---|
| Level 0 | Sham detection: real > sham at p<0.05 | Participants can reliably tell real from sham |
| Level 1 | VAS "something different" > 30/100 | Noticeable altered state produced |
| Level 2 | VAS "relaxation" > 50/100 | Meaningful relaxation effect |
| Level 3 | VAS "similar to mild THC" > 40/100 (experienced users) | Partial THC resemblance |
| Level 4 | VAS "similar to THC" > 60/100 (experienced users) | Strong THC resemblance |
| Level 5 | Blinded experienced users cannot distinguish from THC | Perceptual equivalence |

### Realistic Expectations

| Hardware Tier | Expected Level | Rationale |
|---|---|---|
| Tier 1 ($85) | Level 1, maybe 2 | tDCS + TENS can produce relaxation, unlikely to mimic THC |
| Tier 1.5 (+taVNS) | Level 1-2 | taVNS adds vagal tone, modest serotonin/NE effects |
| Tier 2 (+tACS) | Level 2, maybe 3 | Better oscillatory control, still surface-level |
| Tier 3 (+TMS) | Level 2-3, maybe 4 | Deep modulation, but still not pharmacological specificity |

**Level 5 is aspirational. It may never be achieved with non-invasive surface stimulation.** The pharmacological specificity of THC (CB1 receptor agonism in specific neural circuits) may not be replicable through external electrical fields. This is an open scientific question and we should be prepared for the answer to be "no."

### Statistical Notes

- With N=5, we can detect large effects (Cohen's d > 1.0) at p<0.05. We cannot detect subtle effects.
- With N=20, we can detect medium effects (d > 0.5). This is our minimum for meaningful claims.
- All comparisons are within-subject (paired), which increases power.
- Multiple comparison correction (Bonferroni or FDR) required for the 8 VAS dimensions.
- Pre-register the analysis plan before collecting Phase 1 data.

---

## 7. Data Collection Templates

### Per-Session Recording Form

```
SESSION RECORD
==============
Date:           ___________
Participant ID: ___________
Session #:      ___________
Condition:      [ ] A-Sham  [ ] B-Single  [ ] C-Partial  [ ] D-Full
Condition code: ___________ (blinded, decoded after study)

Pre-Session
-----------
Hours since last caffeine:  ___
Hours since last alcohol:   ___
Hours since last cannabis:  ___
Current mood (0-100):       ___
Current anxiety (0-100):    ___
Hours of sleep last night:  ___
Any medications today:      ___

Stimulation Parameters (Experimenter Only)
------------------------------------------
tDCS:  Site: ___  Current: ___ mA  Duration: ___ min
TENS:  Site: ___  Freq: ___ Hz     Duration: ___ min
taVNS: Site: ___  Current: ___ mA  Duration: ___ min
tACS:  Site: ___  Freq: ___ Hz     Current: ___ mA  Duration: ___ min
LED:   Freq: ___ Hz  Duration: ___ min
Audio: Freq: ___ Hz  Duration: ___ min
Vibro: Freq: ___ Hz  Duration: ___ min

Post-Session VAS (0-100)
------------------------
Euphoria:            ___
Body sensation:      ___
Sensory enhancement: ___
Time distortion:     ___
Altered thinking:    ___
Relaxation:          ___
Anxiety:             ___
THC similarity:      ___ (experienced users only)

Sham Detection
--------------
"Did you receive real stimulation?" [ ] Yes  [ ] No
Confidence (0-100): ___

Free Text
---------
"Describe your experience:"
__________________________________________________
__________________________________________________
__________________________________________________
```

### Adverse Event Form

```
ADVERSE EVENT REPORT
====================
Date:           ___________
Participant ID: ___________
Session #:      ___________

Event:          ___________________________________________
Onset:          During session / ___ min post-session
Duration:       ___________
Severity (0-10): ___
Relationship:   [ ] Definite  [ ] Probable  [ ] Possible  [ ] Unlikely
Action taken:   ___________________________________________
Resolved:       [ ] Yes, at ___  [ ] No, ongoing
Follow-up:      ___________________________________________
```

### Weekly Summary Template

```
WEEKLY SUMMARY — Week ___
=========================
Sessions completed: ___
Adverse events:     ___

Key observations:
- ________________________________________________
- ________________________________________________

VAS trend (average across sessions this week):
  Euphoria:        ___
  Body sensation:  ___
  Relaxation:      ___
  THC similarity:  ___

Hardware issues:
- ________________________________________________

Protocol adjustments for next week:
- ________________________________________________

Go / No-Go for next phase: [ ] Go  [ ] No-Go
Rationale: ________________________________________
```

---

## 8. Ethical Considerations

### Self-Experimentation (Phase 0)

Self-experimentation with non-invasive brain stimulation is legal in most jurisdictions and has a long history in neuroscience research. However:
- There is no external safety monitor. If something goes wrong, you are both the subject and the responder.
- Cognitive impairment during stimulation could affect judgment about whether to stop.
- **Mitigation:** Have another person present during all stimulation sessions. Brief them on the emergency stop procedure.
- Do not self-experiment while alone.

### Participant Studies (Phase 1+)

- **Informed consent:** Written consent describing all known risks of each stimulation modality, the experimental nature of the protocol, and the right to withdraw at any time without penalty.
- **No medical claims:** Nothing in recruitment or consent materials should suggest therapeutic benefit. This is an exploratory study of subjective experience, not a treatment.
- **IRB/Ethics review:** If affiliated with an institution, submit for ethics review before Phase 1. If independent, document the protocol publicly (e.g., OSF) and follow the Declaration of Helsinki principles.
- **Compensation:** If participants are compensated, ensure it does not constitute coercion (especially for students or low-income participants).

### Data Privacy

- No real names in data files. Participant ID codes only.
- Mapping file (ID to name) stored separately, encrypted.
- Raw EEG data stripped of identifying metadata.
- Free-text responses reviewed for identifying information before any sharing.

### Stimulation Safety Limits

All parameters must fall within ranges published in peer-reviewed safety guidelines:
- tDCS: Bikson et al. (2016), "Safety of Transcranial Direct Current Stimulation"
- TMS: Rossi et al. (2021), "Safety and recommendations for TMS use in healthy subjects and patient populations"
- taVNS: Farmer et al. (2021), "International consensus on auricular vagus nerve stimulation"

---

## 9. What Happens If It Doesn't Work

This section is required reading before starting the experiment.

### Scenario: Level 0 Fails (Can't Beat Sham)

**Meaning:** Participants cannot tell real stimulation from sham. The model produces no detectable subjective effect.

**Possible causes:**
- Stimulation intensities too low to affect subjective experience
- Current spread too diffuse for targeted neural circuit modulation
- The 12 variables are correlates, not causes, of the THC state
- Expectation effects dominate, and brain stimulation adds nothing beyond placebo

**What to do:**
- Increase intensities (within safety limits) and retest
- If still nothing, the surface-stimulation approach may be fundamentally limited
- This is a real possibility and an honest scientific outcome

### Scenario: Level 1 But Not Level 3 (Altered State, But Not THC)

**Meaning:** Stimulation produces a noticeable altered state, but it does not resemble THC.

**Possible causes:**
- Brain stimulation can alter consciousness, but the specific phenomenology of THC depends on receptor-level pharmacology that cannot be replicated electrically
- The 12-variable model captures some dimensions of the THC experience but misses the critical ones
- The relative weighting or timing of variables is wrong

**What to do:**
- Analyze which VAS dimensions score highest — what IS the stimulation doing?
- The altered state itself may be valuable, even if not THC-like
- Pivot: instead of "reproduce THC," pursue "novel electrical altered state"
- Revise the model based on what dimensions are and are not achievable

### Scenario: Level 3 But Not Level 5 (Partial Reproduction)

**Meaning:** Experienced users report some THC resemblance, but it is clearly distinguishable.

**Possible causes:**
- Some aspects of THC phenomenology are electrically reproducible, others are not
- Non-invasive stimulation lacks the spatial precision needed for full reproduction
- The model needs more variables or different targets

**What to do:**
- This is actually a strong result. Partial reproduction with $85-$600 of hardware is remarkable.
- Identify which dimensions match well and which do not
- Focus R&D on the gap dimensions
- Consider whether invasive approaches (if ever pursued) could close the gap

### Every Outcome Is Data

The purpose of this experiment is not to confirm the model. It is to test the model. Confirmation bias is the enemy. A clean null result, honestly reported, is more valuable than a tortured positive finding.

---

## 10. Timeline and Budget

### Summary

| Phase | Duration | Cumulative Cost | Primary Question |
|---|---|---|---|
| Phase 0: Self-pilot | 2 weeks | $85 | "Anything at all?" |
| Phase 1: Controlled pilot | 4 weeks | $335 (with EEG) | "Real vs sham?" |
| Phase 2: Escalation | 8 weeks | $515 | "More hardware = better?" |
| Phase 3: Full stack | 12 weeks | $5.5K - $8.5K | "How close to THC?" |

### Cost Breakdown

| Item | Cost | Phase |
|---|---|---|
| tDCS unit | $40-60 | 0 |
| TENS unit | $25 | 0 |
| Arduino + components | $20 | 0 |
| OpenBCI Ganglion (4ch EEG) | $250 | 1 |
| taVNS unit | $100 | 2 |
| tACS unit/modification | $80 | 2 |
| OpenBCI Cyton (16ch EEG) | $1,000 | 3 |
| TMS (used/shared) | $4,000-7,000 | 3 |
| Electrodes, gel, supplies | $50/phase | all |

### Decision Gates

Phase transitions are not automatic. Each requires a deliberate go/no-go decision.

```
Phase 0 ──> Phase 1
  Gate: No serious adverse events
        Subjective: "something feels different" in at least some sessions
        If nothing at all: revisit parameters before spending $250 on EEG

Phase 1 ──> Phase 2
  Gate: Sham detection above chance (>60% accuracy)
        At least one VAS dimension >30/100 above sham
        If sham beats real: stop and reassess model

Phase 2 ──> Phase 3
  Gate: Adding variables improves VAS scores (C > B, D > C trend)
        At least Level 2 achieved
        If more hardware does NOT help: stop. The approach has a ceiling.
```

---

## Appendix A: Known Limitations of This Protocol

In the interest of honesty, these limitations are acknowledged upfront.

1. **No direct neurochemical measurement.** We assume tDCS at F3 increases dopamine based on published literature, but we are not measuring dopamine. We are measuring EEG and behavior and assuming the pharmacological link holds.

2. **VAS scales are noisy.** Subjective reports are influenced by expectation, mood, context, and demand characteristics. Single-blind design partially mitigates this but does not eliminate it.

3. **Small N.** With 5-20 participants, we can detect large effects. If the effect is subtle, we will miss it.

4. **Self-experimentation bias.** The Phase 0 experimenter knows the hypothesis. Their reports are maximally biased. Phase 0 exists only for safety and feasibility, not for drawing conclusions.

5. **No pharmacological comparison arm.** The ideal study would have a THC condition for direct within-session comparison. This is not possible without regulatory approval and is outside our scope.

6. **Stimulation specificity.** tDCS at F3 does not exclusively affect dopamine. It affects everything in the current path. Our "single variable" conditions are not truly single-variable at the neural level.

7. **The hard problem.** Even if we match all 12 measurable variables perfectly, there is no guarantee the subjective experience will match. Consciousness may depend on factors not captured in our model. This is an empirical question, and this protocol is designed to answer it as honestly as we can.

---

## Appendix B: References

- Bikson, M. et al. (2016). Safety of transcranial direct current stimulation. *Brain Stimulation*, 9(5), 641-661.
- Rossi, S. et al. (2021). Safety and recommendations for TMS use in healthy subjects and patient populations. *Clinical Neurophysiology*, 132(1), 269-306.
- Farmer, A.D. et al. (2021). International consensus based review and recommendations for minimum reporting standards in research on transcutaneous vagus nerve stimulation. *Frontiers in Human Neuroscience*, 14, 568051.
- Antal, A. et al. (2017). Low intensity transcranial electric stimulation: safety, ethical, legal regulatory and application guidelines. *Clinical Neurophysiology*, 128(9), 1774-1809.
- Iacono, M.I. et al. (2015). MIDA: A multimodal imaging-based detailed anatomical model of the human head and neck. *PLoS ONE*, 10(4), e0124126.
- Marty, B. et al. (2019). Sensory gamma band entrainment: a systematic review. *Neuroscience & Biobehavioral Reviews*, 107, 385-399.
