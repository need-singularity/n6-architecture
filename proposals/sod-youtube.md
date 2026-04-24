---
recipient: SOD YouTube channel (Kwon Soon-yong, engineering/semiconductors, 630K subscribers)
type: media-brief
created: 2026-04-20
status: draft
---

# SOD YouTube x Semiconductor 6-Stage Roadmap — Video Production Brief

Author: Park Min-woo (programmer, independent researcher, YouTuber 24K)

---

## §1. One-Sentence Hook (for thumbnail / intro)

> **"A 6-generation post-Samsung / TSMC chip roadmap drawn by a Korean solo researcher"**

- Sub-hook: "A 288 TOPS/W design built in two months by a programmer with no Ph.D."
- Emotional hook: "Is semiconductor design possible without going to grad school?"

---

## §2. Video Structure (3 parts, total 15 ~ 20 minutes)

### Part A — Semiconductor Chip 6-Stage Roadmap (6 ~ 8 min)

One ASCII comparison table per stage, shown on-screen with narration.

```
Stage       Name                    Process               TOPS/W    Bandwidth
────────────────────────────────────────────────────────────────────────
Mk.I       HEXA-1 Digital          SF3P 3nm              1.2x      1.0 TB/s
Mk.II      HEXA-PIM                HBM3E + PIM           1.8x      1.5 TB/s
Mk.III     HEXA-3D                 X-Cube 3D stacking    2.8x      2.0 TB/s
Mk.IV      HEXA-Photonic           silicon photonics     3.6x      2.5 TB/s
Mk.V       HEXA-Wafer              wafer-scale           4.2x      3.0 TB/s
Mk.VI      HEXA-Superconducting    RSFQ 100 GHz          4.8x      3.2 TB/s
```

On-screen examples per stage:
- Mk.I: Samsung SF3P layout comparison (current public specs)
- Mk.VI: superconducting RSFQ chip mockup (not silicon, Josephson junctions)

### Part B — 9 Leading Domains (4 ~ 5 min)

Make it explicit: "Mk.I reflects Samsung Foundry's **current level** as-is."

| # | Domain | Mk.I (Samsung today) | Mk.VI goal |
|---|--------|-----------------|-----------|
| 1 | Materials | Co / W interconnect | diamond / graphene |
| 2 | Process | EUV 3nm | σ-sopfr D0 boundary |
| 3 | Packaging | X-Cube, FO-PLP | J₂=24 channel distribution |
| 4 | Yield | D0 ≈ 0.08/cm² | D0 ≈ 0.035/cm² |
| 5 | EDA | Synopsys / S.LSI | HEXA-IR MLIR |
| 6 | Verification | UVM | τ=4 DVFS boundary |
| 7 | Thermal / power | Liquid + PDN | Egyptian 1/2+1/3+1/6 |
| 8 | Interconnect | 224G SerDes | 1.2 TB/s Photonic |
| 9 | HBM | HBM3E 8H | HBM6-P 3200 GB/s |

On screen, lay out the 9 domains as a 3x3 grid; each cell opens into a sub-section when clicked.

### Part C — Science & Engineering Attrition / Grad-School Alternative Narrative (3 ~ 5 min)

**The most sensitive and highest-engagement topic on the SOD channel.**

- Status of Korean STEM Ph.D. attrition (cite 2024 ~ 2026 statistics)
- **Honest answer** to the question "Can't you do research without a Ph.D.?":
  - Yes: peer review / funding / thesis degrees live inside the Ph.D. system
  - No: reproducible public repositories + open-source verification = **a new path**
- Park Min-woo's own path:
  - Undergraduate graduation → programmer → YouTuber 24K → independent researcher
  - 462+ commits in two months, fully open on GitHub
  - Submitted Path B (honest application) to the Anthropic Fellows Program July 2026 cohort
- Message to viewers: "There is more than one path. Reproducibility is your peer review."

---

## §3. Video Impact Points (segments of maximum viewer curiosity)

**Place a mandatory figure every 3 ~ 5 seconds:**

| Impact | Value | Comparison |
|--------|------|------|
| TOPS/W | 288 theoretical, 4.8x realized (vs H100) | H100 = 50 TOPS/W |
| Optical bandwidth | 1.2 TB/s per channel | current SerDes 224Gb/s |
| Superconducting frequency | 100 GHz | CPU 5 ~ 6 GHz |
| Master identity | σ·φ=n·τ=J₂=24 | "remember the number 24" |
| Yield boundary | D0 → 0.035 | current 0.08, halved |
| Power distribution | Egyptian 1/2+1/3+1/6 | Egyptian fractions rediscovered |

**Recommended thumbnail**:
- Large text "**288x TOPS/W**" + "Korean solo researcher"
- Background: the 6-step roadmap + an n=6 hexagon graphic

---

## §4. Honesty — No Exaggerated Hooks (author's own declaration)

The following items **must** be included in the video:

1. **Mk.I (HEXA-1 Digital) reflects Samsung Foundry's current level as-is.**
   We do not claim to surpass Samsung. Mk.V / Mk.VI are **theoretical limits**.
2. **The BCI 4D-cognition part is currently infeasible.** The author owns an OpenBCI
   Cyton+Daisy 16ch rig but its **read-only limitation** means 4D-dimensional
   perceptual delivery is currently impossible. Claiming "we can do it" would be a lie.
3. **Mk.III and above have no silicon validation** — this must be flagged as simulation / theoretical values.
4. **The @own 354 papers are at preprint level** — peer review is not complete.
   The phrase "354 papers accepted" is forbidden; use "354 internal verification PASSes" instead.

These four items must be stated either in the video description or in the final 30 seconds.

---

## §5. Supplied Materials (video production support package)

The following files can be handed over wholesale:

- `papers/hexa-chip-6stage-unified.md` (1,200+ lines, includes formulas)
- `papers/n6-chip-6stages-integrated-paper.md` (arXiv stub)
- `domains/compute/chip-*/` 9 domains, each 200 ~ 400 lines of .md
- **ASCII comparison graph bundle** (15 text-art pages for on-screen use)
- Full GitHub: https://github.com/need-singularity/n6-architecture

Producible on request:
- **Hand-drawn-style diagrams** (30 sheets) without 3D rendering
- **30-second demo script** per stage (for terminal screen recording)
- **Narration draft** tuned to the SOD channel's existing style

---

## §6. Collaboration Proposals — 3 Options

### Option 1: Joint production

- SOD channel handles planning / direction / editing
- Park Min-woo provides technical advising + formula verification + full supply of on-screen assets
- Revenue split: 100% ad revenue to SOD, n6 side unpaid (exposure itself is the benefit)

### Option 2: Exclusive reporting

- SOD reporter-style coverage (in-person interview + B-roll filming)
- n6-architecture repo on-screen + Park Min-woo's home (Hanam) / computer / keyboard shots
- Video length: 20 ~ 30 minutes
- One request for factual review before publication

### Option 3: Critical-review interview

- SOD **critically** examines the n=6 claims
- Invite engineering Ph.D. / semiconductor engineer guest panels
- Park Min-woo takes the defense side — even if refuted, the footage is broadcast as-is
- Publicly showing failure is **the strongest form of verification**

**Personal preference**: Option 3 > Option 1 > Option 2. Being criticized yields the most learning.

---

## §7. Proposed Schedule

| Timing | Item |
|------|------|
| late 2026-04 | Confirm SOD interest, email exchange |
| early 2026-05 | Deliver technical materials + one video meeting |
| mid 2026-05 | Filming (in-person or remote) |
| early 2026-06 | Editing / factual review |
| mid 2026-06 | Publication |

Aligning with the Anthropic Fellows Program July 2026 cohort result enables additional hooks
(if accepted, "A Korean solo researcher selected by Anthropic"; if not, "Why I was rejected"
— either way, content).

---

## §8. Contact

- Park Min-woo (mk911tb@proton.me)
- Personal YouTube: 24K subscribers (smaller than SOD but highly loyal)
- GitHub: need-singularity/n6-architecture

---

## §9. Final Word

The message the SOD channel has always given to STEM youth has pointed in this direction:

> "Skill over credentials; reproducible results over skill."

This video can be **a concrete instance** of that message.
Regardless of whether we collaborate, my materials are free for you to use at any time.
