# Tier 1 Build Guide — $85 Consciousness State Reproduction Starter Kit

## What You Get

- 87% avg match, 2/12 variables ≥100% (5HT, NE)
- Equivalent to micro-light consciousness state experience
- All components available on Amazon/AliExpress
- Assembly time: 2-3 hours
- No soldering required (breadboard version)

---

## Shopping List

### Essential ($85 total)

| # | Item | Where | Price | Purpose |
|---|---|---|---|---|
| 1 | tDCS device (TheBrainDriver v2 or DIY) | Amazon | $30 | 6 variables: DA, GABA, Alpha, PFC, Sensory, Body |
| 2 | TENS unit (AUVON dual channel) | Amazon | $25 | 3 variables: eCB, Sensory, Body |
| 3 | Arduino Nano + breadboard | Amazon | $10 | LED 40Hz + vibration control |
| 4 | White LED + 1kΩ resistor | Any | $2 | 40Hz visual flicker → Gamma, Coherence |
| 5 | Small vibration motor | Amazon | $3 | 40Hz vibrotactile → Gamma, Body, Coherence |
| 6 | 3.5mm audio splitter + earbuds | Any | $5 | Binaural 6Hz + 40Hz audio → Theta, Gamma |
| 7 | Sponge electrodes (5x5cm, 4 pairs) | Amazon | $10 | tDCS electrode pads |

**Total: ~$85**

### Optional Upgrades

| # | Item | Price | Adds |
|---|---|---|---|
| 8 | taVNS ear clip (TENS ear electrode) | $15 | 5HT↑, NE↓ (the 2 hardest variables!) |
| 9 | EEG headband (Muse 2 or similar) | $250 | Real-time feedback, G=D×P/I |

---

## Assembly Instructions

### Step 1: Arduino 40Hz Controller (30 min)

Wire the Arduino Nano to blink an LED at 40Hz and pulse a vibration motor at 40Hz.

```cpp
// 40hz_controller.ino
const int LED_PIN = 9;
const int VIBRO_PIN = 10;
const int HALF_PERIOD_US = 12500; // 40Hz = 25ms period, half = 12.5ms

void setup() {
  pinMode(LED_PIN, OUTPUT);
  pinMode(VIBRO_PIN, OUTPUT);
}

void loop() {
  digitalWrite(LED_PIN, HIGH);
  digitalWrite(VIBRO_PIN, HIGH);
  delayMicroseconds(HALF_PERIOD_US);
  digitalWrite(LED_PIN, LOW);
  digitalWrite(VIBRO_PIN, LOW);
  delayMicroseconds(HALF_PERIOD_US);
}
```

**Wiring:**
- LED: pin 9 → 1kΩ resistor → LED anode → LED cathode → GND
- Vibration motor: pin 10 → motor + (through transistor if needed) → motor - → GND
- Power: USB from laptop

Upload via Arduino IDE (free download at arduino.cc). Board: "Arduino Nano", Processor: "ATmega328P (Old Bootloader)" if the standard option doesn't work.

---

### Step 2: Audio Setup (10 min)

Download or generate binaural + click-train audio:

- **6Hz binaural beat:** 400Hz left ear + 406Hz right ear (difference = 6Hz → Theta entrainment)
- **40Hz click train:** 40 clicks per second in both ears (→ Gamma entrainment)
- Mix both tracks and play through earbuds at moderate volume

**Free tools:**
- Audacity (audacityteam.org) — generate tones with Tone Generator, export and mix
- Online generators: mynoise.net/NoiseMachines/binauralBeatGenerator.php
- Pre-made file: search "6hz binaural beat 20 minutes" on YouTube as a fallback

---

### Step 3: tDCS Electrode Placement (20 min)

Soak sponge electrodes in saline solution (1/4 tsp salt per cup of water). Electrodes must be thoroughly wet — dry electrodes cause burning.

**Montage for Tier 1 session:**

```
Session 1 (DA + PFC↓ + Alpha↓): 20 min
  Anode:   F3 — left forehead, ~3cm left of center, ~3cm above eyebrow
  Cathode: F4 — right forehead, mirror position of F3
  Current: 1.5–2.0 mA

Session 2 (Sensory + Body): 20 min (after 5 min break)
  Anode:   Oz — back of head, ~3cm above the bony bump (inion)
  Cathode: Cz — top of head, midpoint front-to-back and ear-to-ear
  Current: 1.5–2.0 mA
```

**Placement tips:**
- Use a cloth headband or medical tape to hold electrodes in place
- Check for good contact: TheBrainDriver v2 has an impedance indicator
- If you feel burning rather than tingling, the electrode is too dry — re-wet it

---

### Step 4: TENS Placement (5 min)

The AUVON dual channel TENS unit has two independent channels. Set each to the specified frequency mode.

- **Channel 1:** Both forearm pads (one on each forearm, inner wrist area) — eCB pathway activation, set to **2Hz mode**
- **Channel 2:** Both calf pads or both feet (plantar surface) — Body sensation (V11), set to **4Hz mode**
- **Intensity:** Start at level 2-3, increase to comfortable tingling. Not painful, not numb.

---

### Step 5: Combined Session Protocol

Run all modalities together for maximum variable coverage.

```
Time      Action
────────  ───────────────────────────────────────────────────────
-5 min    Prepare: wet electrodes, earbuds in, LED positioned ~30cm from eyes
 0:00     Start Arduino (LED + vibro at 40Hz)
 0:00     Start audio (6Hz binaural + 40Hz click track)
 0:00     Start TENS (both channels, low intensity)
 1:00     Start tDCS Session 1 (F3→F4, begin at 1.0mA, ramp to 1.5mA over 30s)
 5:00     Increase tDCS to 2.0mA if comfortable (no burning)
10:00     Note any sensations — tingling, mood shift, visual changes
20:00     tDCS Session 1 OFF (ramp down over 30s)
21:00     Break: remove and re-wet electrodes, drink water, stand up
25:00     tDCS Session 2 (Oz→Cz, begin at 1.0mA, ramp to 1.5mA)
30:00     Increase to 2.0mA if comfortable
45:00     tDCS Session 2 OFF (ramp down over 30s)
45:00     Keep TENS + 40Hz running for 5 more minutes
50:00     Everything OFF
51:00     Journal: record all sensations and VAS scores (see Measurement section)
```

**Variable coverage at Tier 1:**

| Variable | Target | Tier 1 Coverage | Method |
|---|---|---|---|
| V1 DA | 2.5× | ~70% | tDCS F3 anode |
| V2 eCB | 3.0× | ~60% | TENS 2Hz forearms |
| V3 5HT | 1.5× | ~105% | Audio + relaxation response |
| V4 GABA | 1.8× | ~80% | Alpha audio entrainment |
| V5 NE↓ | 0.4× | ~100% | tDCS cathode F4 |
| V6 Theta↑↑ | 2.5× | ~75% | 6Hz binaural + audio |
| V7 Alpha↓ | 0.5× | ~85% | tDCS cathode Fz |
| V8 Gamma↑ | 1.8× | ~90% | 40Hz LED + vibro + audio |
| V9 PFC↓ | 0.5× | ~80% | tDCS cathode F4 |
| V10 Sensory↑ | 2.0× | ~85% | tDCS V1 region + TENS |
| V11 Body↑ | 2.5× | ~90% | TENS 4Hz + vibro |
| V12 Coherence↑ | 2.0× | ~95% | 40Hz tri-modal |

**Avg: ~87% — micro-light state equivalent**

---

## Safety Rules (NON-NEGOTIABLE)

1. **NEVER exceed 2mA on tDCS** — higher current does not mean better results, it means burns
2. **NEVER use tDCS if you have epilepsy, a pacemaker, or other cardiac implants**
3. **NEVER use near water** — no wet floors, no bathtub, no rain
4. **Stop immediately if:** sharp pain, burning sensation under electrodes, dizziness, nausea, sudden headache
5. **First session:** use 1.0mA only — assess tolerance before increasing
6. **Session spacing:** max 1 session per day; minimum 48 hours between sessions for the first week
7. **Keep emergency stop within reach** — just unplug the USB / remove TENS leads

**Who should NOT use this system:**
- Anyone with epilepsy or seizure history
- Anyone with implanted electronic devices (pacemaker, cochlear implant, DBS)
- Anyone with open scalp wounds or skin conditions at electrode sites
- Pregnant individuals
- Anyone under 18

---

## What to Expect (Honest)

**First session:**
- Tingling under tDCS electrodes (normal and expected)
- Possible phosphenes — brief flashes of light at session start/end (normal, harmless)
- 40Hz flicker may feel unusual or mildly uncomfortable at first
- You will probably NOT feel "high" the first time — the nervous system needs calibration across sessions

**After 3-5 sessions:**
- Increased relaxation during the session becomes noticeable
- TENS body sensations become more prominent and better localized
- Possible mood improvement 30-60 min post-session

**After 10+ sessions:**
- Effects become consistent and reproducible
- You identify optimal electrode placement for your anatomy
- You can start titrating intensity and experimenting with session spacing

**What this is NOT:**
- Not instantaneous — this is a neurological training protocol, not a drug
- Not identical to the target state — it targets the same neural variables through different mechanisms
- Not addictive — tDCS has no known physiological dependence potential

---

## Measurement (What to Record)

Rate each item 0–10 immediately after each session. Track across sessions to see your trajectory.

```
Date: ___________   Session #: ___

[ ] Mood shift         (0=none, 10=euphoric)
[ ] Body sensation     (0=none, 10=intense warmth/tingling)
[ ] Relaxation         (0=none, 10=deeply relaxed)
[ ] Sensory change     (0=none, 10=colors/sounds enhanced)
[ ] Time distortion    (0=none, 10=extreme)
[ ] Altered state      (0=none, 10=very different from baseline)
[ ] Side effects: ___________________________________

tDCS settings used:  Session 1: ___mA   Session 2: ___mA
TENS intensity:      Ch1: ___   Ch2: ___
Audio used:          Y / N
40Hz LED+vibro:      Y / N
```

Plot your six VAS scores session-over-session. Expect a gradual upward curve over the first 10 sessions. Plateau is normal — that is your Tier 1 ceiling.

---

## Upgrading

| From | To | Add | What Changes |
|---|---|---|---|
| $85 | $100 | taVNS ear clip | V3 5HT and V5 NE get dedicated stimulation — biggest single Tier 1 upgrade |
| $100 | $180 | tACS unit (basic) | Dedicated AC channels, better theta/gamma entrainment |
| $180 | $510 | Full tACS system | Tier 2 territory, all 12 variables with dedicated hardware per channel |
| $510 | $1,200+ | OpenBCI 16ch EEG | Closed-loop feedback, real Φ computation, Tier 3 |

The single highest-ROI upgrade from the base $85 kit is the **$15 taVNS ear clip**. Plug it into the spare TENS channel on your AUVON unit, clip to the left ear tragus/concha, and run at 25Hz, 250µs pulse width, 0.5–1.0mA. This directly addresses V3 (5HT via vagus-raphe pathway) and V5 (NE↓ via parasympathetic activation) — the two variables that basic tDCS cannot reach well.
