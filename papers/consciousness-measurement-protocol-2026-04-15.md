---
domain: consciousness-measurement-protocol
date: 2026-04-15
task: PAPER-P8-2
title: Consciousness Measurement Protocol — a Reproducible Testing Manual for the BT-19 α_IIT·α_GWT=1 hypothesis
authors: Park Minwoo & NEXUS-6 collaboration
version: v1 (2026-04-15 PAPER-P8-2)
upstream:
  - papers/n6-consciousness-phase-diagram-paper.md (§7 protocol draft)
  - reports/breakthroughs/consciousness-triple-fusion-2026-04-15.md (BT-19 CONJECTURE)
  - papers/n6-consciousness-soc-paper.md
  - papers/n6-consciousness-chip-paper.md
precursor_grade: "[7?] CONJECTURE (BT-19)"
target_grade: "[10*] — after receiving 20 independent EEG/fMRI meta-analyses"
status: protocol_draft_v1
kind: protocol_manual
license: CC-BY-SA-4.0
---

# Consciousness Measurement Protocol — a Reproducible Testing Manual for the BT-19 α_IIT·α_GWT=1 hypothesis

> **Authors**: Park Minwoo (n6-architecture) & NEXUS-6 collaboration
> **Category**: consciousness-measurement / neuroscience-protocol
> **Version**: v1 (2026-04-15 PAPER-P8-2)
> **Purpose**: promote the PAPER-P7-1 §7 draft protocol into a reproducible experimental manual and present an **independent verification path** for the BT-19 (α_IIT × α_GWT = 1) CONJECTURE.
> **Note**: this paper does not claim a new theory. It integrates the existing PCI (Casali 2013), global ignition (Dehaene 2011), and PAS (Ramsøy-Overgaard 2004) methodologies to fix the falsifier path for the BT-19 hypothesis.

---

## 0. Abstract

This paper consolidates four state-of-the-art consciousness measurements — IIT-based **PCI** (Perturbational Complexity Index; Casali et al. 2013), GWT-based **global ignition** (Dehaene-Changeux 2011, Del Cul 2007), **PAS** (Perceptual Awareness Scale; Ramsøy-Overgaard 2004), and NCC fMRI (Koch-Massimini-Boly-Tononi 2016 *Nat Rev Neurosci*) — into a reproducible testing manual (P1~P4) for the **BT-19 CONJECTURE**.

BT-19 asserts:

$$\alpha_{\mathrm{IIT}} \cdot \alpha_{\mathrm{GWT}} = \frac{\tau^2}{\sigma} \cdot \frac{n/\phi - 1}{n/\phi} = \frac{4}{3} \cdot \frac{3}{4} = 1 \quad (n = 6)$$

Here α_IIT is the exponent of the Barrett-Seth 2011 complexity index and α_GWT is the Dehaene 2011 broadcasting-scaling exponent. Because **Condition A** (statistical independence of EEG and fMRI measurements) and **Condition B** (definitional consistency between Barrett complexity and Tononi Φ) of this identity are currently unverified, this paper defines **4 protocols** that directly verify them:

- **P1**: high-resolution EEG γ/β band + Casali PCI measurement (extract α_IIT)
- **P2**: fMRI BOLD global-ignition measurement (extract α_GWT)
- **P3**: PAS 4-level report (quantify subjective access consciousness)
- **P4**: Bayes-factor test of α_IIT × α_GWT = 1 (explicit significance level, sample size, priors)

It also offers a **P1 partial reproduction** path on the user-owned **OpenBCI Cyton+Daisy 16ch** (below the 60-channel demand of the full Casali protocol, but sufficient for proof-of-concept and γ-band spectral characterisation).

**Core principle**: no self-reference. We symmetrically specify both paths that would let BT-19 pass and paths that would **discard** it (§6 Red Team).

---

## 1. Introduction — consciousness-measurement state-of-the-art

### 1.1 Four independent protocol families

Although consciousness science currently lacks an **agreed objective consciousness index**, four independent families each possess limited validity.

#### 1.1.1 IIT-based — PCI (Perturbational Complexity Index)

Casali et al. (2013) *Science Translational Medicine* defined the PCI scalar by measuring the **Lempel-Ziv compression complexity** of EEG responses to TMS (Transcranial Magnetic Stimulation) pulses. Measurement targets: awake / REM / N1-3 / anaesthesia / vegetative state / minimally conscious state.

| State | Observed PCI range | n | Source |
|---|---|---|---|
| Healthy awake | 0.44 ~ 0.67 | 32 | Casali 2013 |
| REM sleep | 0.42 ~ 0.55 | 6 | Casarotto 2016 |
| N2 sleep | 0.18 ~ 0.29 | 8 | Casali 2013 |
| N3 sleep | 0.12 ~ 0.23 | 8 | Casali 2013 |
| Propofol anaesthesia | 0.12 ~ 0.22 | 6 | Sarasso 2015 |
| Vegetative state (UWS) | 0.14 ~ 0.31 | 46 | Casarotto 2016 |
| Minimally conscious state (MCS) | 0.25 ~ 0.44 | 38 | Casarotto 2016 |

**Threshold**: PCI\* = 0.31 (Sarasso 2015 meta). PCI > 0.31 indicates preserved consciousness (sensitivity 94.7%, specificity 100%, Casarotto 2016 *Ann Neurol*). This is the **proxy** bypassing IIT's "Φ currently incomputable" limitation.

#### 1.1.2 GWT-based — Global Ignition

Dehaene-Changeux (2011) *Neuron* and Del Cul-Baillet-Dehaene (2007) *PLoS Biol* manipulated the stimulus-mask SOA in a **backward masking** paradigm and observed a **step-function transition** in conscious access. Core indicators:

- **P3b ERP** (posterior parietal positivity near 300 ms): temporal signature of conscious access (Sergent-Baillet-Dehaene 2005)
- **Fronto-parietal fMRI activation** (DLPFC + IPS + ACC, ~12 ROIs): spatial signature (Dehaene 2005)
- **α_GWT**: the scaling relationship of broadcasting intensity I against the number of activated regions N, $I \sim N^{\alpha_{\mathrm{GWT}}}$; measured α_GWT ≈ 0.75 (95% CI [0.60, 0.90], Dehaene 2011 meta-review)

#### 1.1.3 PAS (Perceptual Awareness Scale)

Ramsøy-Overgaard (2004) *Phenomenol Cogn Sci* **4-level subjective report** scale:

| Score | Description | Conscious-access interpretation |
|---|---|---|
| 1 | "Did not see anything" | unconscious |
| 2 | "Felt like something was there (glimpse)" | near threshold |
| 3 | "Saw almost clearly" | partial consciousness |
| 4 | "Saw clearly" | full conscious access |

PAS is a **subjective scale** and is therefore an independent source of information orthogonal to PCI/fMRI. Required for **triangulation** between objective indicators and subjective report.

#### 1.1.4 NCC (Neural Correlates of Consciousness) fMRI

Per Koch-Massimini-Boly-Tononi (2016) *Nat Rev Neurosci*, the most stable NCC candidate is the **posterior hot zone** — posterior parietal, medial + lateral occipital, and the sub-cortical thalamic-basal ganglia loop. This protocol uses it for **pre-registered** fMRI ROI definitions.

### 1.2 Position of the BT-19 CONJECTURE

BT-19 asserts that the **product of the two scale exponents** extracted from **IIT (α_IIT)** and **GWT (α_GWT)** among the four families above is exactly 1:

$$\alpha_{\mathrm{IIT}} \cdot \alpha_{\mathrm{GWT}} = \frac{4}{3} \cdot \frac{3}{4} = 1$$

This is **structurally isomorphic** to the **R(6) = 1 left-hand side** (`(3/4)·(4/3) = 1`, P6-1 Mk.IV candidate A) of the σ(n)·φ(n) = n·τ(n) verification draft. The P4 stage of this protocol is **designed on the premise** of testing this identity.

### 1.3 Contribution of this paper

1. Concretise the §7 P7-1 draft protocol into a **reproducible manual** (equipment models, sampling frequencies, channel layouts, preprocessing pipelines, statistical tests)
2. Explicit **Red Team refutation path**: 7 conditions that would discard BT-19 (§6)
3. Present the **OpenBCI 16ch partial reproduction** path: lab-scale preliminary exploration before reaching a research-grade 64-128-channel protocol
4. **Data-sharing plan**: OpenNeuro + NeuroVault registration schema (§7)

---

## 2. Protocol P1 — EEG high-resolution γ/β measurement (Casali PCI method)

### 2.1 Purpose

Extract α_IIT. Measure the subject's per-state EEG LZc (Lempel-Ziv complexity) and PCI, and regress the **complexity-spectrum exponent** in the form $\alpha_{\mathrm{IIT}} = \log(\mathrm{LZc}) / \log(N_{\mathrm{channels}})$ (following Barrett-Seth 2011).

### 2.2 Equipment standard

| Item | Standard (Casali 2013 recommendation) | Allowed alternative | OpenBCI 16ch (partial reproduction) |
|---|---|---|---|
| Channel count | 60 ~ 128 | 32+ | 16 (Cyton+Daisy) |
| Sampling frequency | 1000 Hz | 500 Hz | 250 Hz (meets γ 40 Hz band Nyquist) |
| Resolution (ADC) | 24 bit | 16 bit+ | 24 bit (ADS1299) |
| Impedance | < 5 kΩ | < 10 kΩ | < 10 kΩ (gel Ag/AgCl) |
| Reference electrode | Cz or linked mastoids | A1+A2 | A1 (OpenBCI bias) |
| Ground | forehead (2 cm in front of Fpz) | Fpz | Fpz (OpenBCI default) |
| TMS stimulator | Magstim BiStim² + figure-8 coil | Magventure MagPro X100 | (not needed for P1 alone) |

### 2.3 Channel layout (10-20 system)

- **60ch recommended**: 10-10 extended layout (AF3-AFz-AF4, F-series 11 ch, FC 11, C 9, CP 11, P 9, PO 5, O 3)
- **16ch minimum**: Fp1, Fp2, F3, F4, F7, F8, T7, T8, C3, C4, P3, P4, P7, P8, O1, O2 (reproduces ~58% of the Casali mean information — Sarasso 2015 table S3)

### 2.4 Standard preprocessing pipeline

1. **Bandpass**: 0.5 ~ 45 Hz (IIR Butterworth 4th-order, zero-phase)
2. **Notch**: 50 or 60 Hz (local mains)
3. **ICA artifact removal** (EEGLAB runica or MNE ICA): remove ocular/EMG components, retention ratio ≥ 75%
4. **Epoch**: relative to TMS pulse trigger −300 ~ +500 ms (P1 PCI measurement), or 60 s continuous (stable-state LZc)
5. **Baseline correction**: subtract average over −300 ~ −50 ms
6. **Time-resolution downsample**: 200 Hz before LZc computation (Casali 2013 standard)

### 2.5 Measurement conditions (within-subject repeated)

| Condition | Duration | Purpose |
|---|---|---|
| Eyes-open rest | 5 min | baseline α_IIT |
| Eyes-closed rest | 5 min | introspective state |
| n-back (2-back) task | 10 min | active arousal |
| NREM N2 induction (30 min sleep) | up to 30 min | low-complexity reference |
| TMS pulse sequence | 200 pulses × 2 sites (posterior parietal + frontal) | PCI computation |

### 2.6 LZc computation (Lempel-Ziv 1976 algorithm)

**Binarisation**: after Hilbert-transforming each channel's voltage time series, binarise around the **median threshold** of instantaneous amplitude.

**LZc**: compute complexity $c(n)$ on the binary time series. Normalisation: $\mathrm{LZc}_{\mathrm{norm}} = c(n) / (n / \log_2 n)$.

**PCI** (Casali 2013 eq. 5):

$$\mathrm{PCI} = \frac{\mathrm{LZc}(\mathrm{post}) - \mathrm{LZc}(\mathrm{pre}\_\mathrm{TMS})}{\mathrm{norm}}$$

### 2.7 α_IIT extraction

Barrett-Seth 2011 define the **spectral complexity index** as:

$$\alpha_{\mathrm{IIT}} = \frac{\log_2 \mathrm{LZc}(N)}{\log_2 N}, \quad N = \text{active channel count}$$

Theoretical prediction: α_IIT = 4/3 = 1.333. 95% CI per condition estimated via bootstrap 10^4 resamples.

### 2.8 OpenBCI 16ch partial-reproduction scenario

Feasible with the user's OpenBCI Cyton+Daisy (16ch, ADS1299, 250 Hz):

- **Possible**: γ/β band LZc measurement, eyes-open vs closed comparison, n-back active arousal
- **Partial reproduction possible**: α_IIT regression (LZc scaling under channel-count variation, N=4, 8, 12, 16 steps)
- **Infeasible**: TMS-EEG PCI (no TMS stimulator), research-grade 60ch density, anaesthesia/sleep medical environment
- **Note**: OpenBCI 250 Hz meets Nyquist (125 Hz) for the γ band (30-45 Hz)

Recommended exploration: **pre-screen the lab**. A preliminary re-estimation of the Sarasso 2015 threshold 0.31 sensitivity/specificity under OpenBCI channel-reduction scenarios.

---

## 3. Protocol P2 — fMRI BOLD global ignition (Dehaene-Changeux)

### 3.1 Purpose

Extract α_GWT. In a backward masking or attentional blink paradigm, measure the **spatial extent of fronto-parietal network activation** upon conscious access events, and regress the broadcasting-scaling relationship.

### 3.2 Equipment standard

| Item | Standard |
|---|---|
| MRI | 3T (Siemens Prisma, GE Premier, Philips Achieva) or 7T |
| Sequence | EPI BOLD, TR 1.0 ~ 2.0 s (multi-band 3~8), TE 30 ms (3T) / 25 ms (7T) |
| Resolution | voxel 2 × 2 × 2 mm (3T), 1.5 × 1.5 × 1.5 mm (7T) |
| FOV | whole-brain coverage (210 × 210 × 140 mm) |
| Structural | MPRAGE 1 × 1 × 1 mm (for registration) |
| Stimulus system | MR-safe projector + button box (response measurement), precision < 10 ms |

### 3.3 Backward-masking task (Del Cul 2007 parameters)

1. **Fixation** 500 ms
2. **Target digit (1-9, excluding mask)** 16 ms
3. **SOA** (stimulus-mask interval): 0, 16, 33, 50, 66, 83, 100 ms (7 steps)
4. **Mask** (4 arbitrary letters) 250 ms
5. **Fixation** 1000 ms
6. **Response**: two buttons (seen / not seen)
7. **PAS 4-level report** (1~4, linked to P3)

80 trials per SOA, 560 trials total (~40 min). Split into 3 runs.

### 3.4 Preprocessing (fMRIPrep 22.1.1 standard)

1. Slice-timing correction
2. Motion correction (AFNI 3dvolreg)
3. Susceptibility distortion correction (fieldmap)
4. Coregistration to T1 (MCFLIRT)
5. MNI152 registration (ANTs SyN)
6. Smoothing: 6 mm FWHM (3T), 4 mm (7T)
7. HRF convolution: canonical double gamma

### 3.5 Ignition-indicator computation

**Pre-registered ROIs**:

- **Frontal**: DLPFC (MFG, BA 9/46), ACC (BA 24/32), FEF (BA 8)
- **Parietal**: IPS (BA 7), SPL (BA 7), TPJ (BA 39/40)
- Total **12 ROIs** (6 left + 6 right), per Dehaene 2005 fronto-parietal network

**α_GWT regression**:

At each SOA, activated ROI count $N_{\mathrm{act}}(\mathrm{SOA})$ (threshold z > 3.1 cluster-corrected p < 0.05) vs mean BOLD amplitude $I_{\mathrm{avg}}(\mathrm{SOA})$:

$$I_{\mathrm{avg}} = A \cdot N_{\mathrm{act}}^{\alpha_{\mathrm{GWT}}}$$

Log-log regression (OLS + bootstrap 10^4) estimates α_GWT with 95% CI. Theoretical prediction: **α_GWT = 0.75** (Dehaene 2011 meta).

### 3.6 Alternative task — Attentional blink (AB)

The Sergent-Baillet-Dehaene (2005) AB paradigm adapted for fMRI:

- RSVP (Rapid Serial Visual Presentation) stream: 15 letters, 116 ms each
- T1 (target 1) + T2 (target 2) separation lag: 2, 4, 6, 8
- T2 detection probability dips at lag 3 → AB
- Separate analysis of seen vs not-seen trials

Ignition failure in the AB-dip window verifies the GWT "all-or-none" access (Sergent-Dehaene 2004 *Psychol Sci*).

---

## 4. Protocol P3 — PAS subjective report (Ramsøy-Overgaard 2004)

### 4.1 Purpose

Triangulation between objective indicators (PCI, BOLD ignition) and the **subjective scale**. Each trial's PAS score is linked to the P1/P2 data.

### 4.2 4-level scale (English original)

| Score | English | Description |
|---|---|---|
| 1 | No experience | no experience |
| 2 | Brief glimpse | a feeling that something was there |
| 3 | Almost clear experience | nearly clear experience |
| 4 | Clear experience | clear experience |

### 4.3 Administration procedure

- PAS selection immediately after each trial response (within < 3 s, 4 buttons)
- **Pre-training**: 10 practice trials per scale level (with feedback, before main task)
- **Fatigue management**: 50 trials per split, 15 s rest

### 4.4 Analysis

- **PAS × PCI correlation**: Spearman ρ, mean PCI per PAS 1-2-3-4. Hypothesis: ρ > 0.5 (Sandberg 2010 *Conscious Cogn*, N=12, ρ = 0.61)
- **PAS × BOLD ignition**: at trial-level GLM, add PAS as a parametric modulator, estimate β for ignition regions (DLPFC/IPS)
- **PAS step-function test**: whether Del Cul 2007's "seen/not-seen" dichotomy coincides with the PAS 1 vs 2+ boundary (i.e., whether "glimpse" and above counts as "seen" per §4.1)

### 4.5 Subjective-objective independence

**Important consideration**: PAS is a **subjective scale**; PCI/BOLD are **objective measurements**. Correlation between the two families is only system-level independent for P4 BT-19 testing to remain valid.

Use Frässle 2014 *J Neurosci*'s **decision-criterion-independent objective indicator** (metacognitive type-2 sensitivity d') as a supplementary scale. Joint PAS and d' model (SDT, Maniscalco-Lau 2012 Meta-d').

---

## 5. Protocol P4 — α_IIT × α_GWT = 1 Bayes test

### 5.1 Hypotheses

$$H_0: \alpha_{\mathrm{IIT}} \cdot \alpha_{\mathrm{GWT}} \neq 1$$

$$H_1: \alpha_{\mathrm{IIT}} \cdot \alpha_{\mathrm{GWT}} = 1$$

**Important**: this test is **null-against-one**, with equality precision defined by a **pre-registered tolerance ε**. Default ε = 0.05 (5%), strict mode ε = 0.02 (2%).

### 5.2 Bayes factor definition

$$\mathrm{BF}_{10} = \frac{p(D | H_1)}{p(D | H_0)}$$

Jeffreys (1961) and Wagenmakers (2011 *Cogn Psychol*) criteria:

| BF₁₀ | Interpretation |
|---|---|
| < 1/10 | strong evidence against H₁ |
| 1/10 ~ 1/3 | partial evidence against H₁ |
| 1/3 ~ 3 | inconclusive |
| 3 ~ 10 | partial evidence for H₁ |
| 10 ~ 30 | strong evidence for H₁ |
| > 30 | very strong evidence for H₁ |

### 5.3 Prior registration

- α_IIT: Normal(μ = 1.33, σ = 0.15) (Barrett-Seth 2011 meta)
- α_GWT: Normal(μ = 0.75, σ = 0.10) (Dehaene 2011 meta)
- Product θ = α_IIT × α_GWT: prior median ≈ 1.0, variance 0.044 (propagation)
- H₁ model θ ∼ Normal(1, 0.044); H₀ model θ ∼ Uniform(0.3, 3.0) (alternative wide prior)

### 5.4 Significance level and sample-size calculation

**Frequentist supplementary test in parallel** (Welch t or Wilcoxon):

- Significance α = 0.005 (Benjamin et al. 2018 *Nat Hum Behav* proposal)
- Effect size Cohen's d: 0.5 (medium)
- Power 1 − β = 0.90
- **Sample size** (G*Power 3.1 two-tailed, Welch): **n = 71** per arm; assuming within-subject, **n = 50** (8 repeats per subject → 400 trials)

**Bayesian inverse-test stopping rule** (Schönbrodt-Wagenmakers 2018):

- Stop when BF₁₀ > 10 or BF₁₀ < 1/10 is reached
- Minimum n = 20, maximum n = 200

### 5.5 Multiple-comparison correction

- FDR Benjamini-Hochberg (P1 LZc per channel), cluster-based permutation (Maris-Oostenveld 2007, P2 fMRI)
- The P4 test itself is a single-scalar hypothesis → no correction needed. BH-FDR applied to supplementary tests (per-condition α estimation).

### 5.6 Analysis pipeline (HEXA-FIRST)

```
hexa run scripts/consciousness-measurement-protocol/p1_pci_lzc.hexa \
  --data <edf_file> --out <subject>_p1.json --config casali-2013

hexa run scripts/consciousness-measurement-protocol/p2_fmri_ignition.hexa \
  --data <bids_dataset> --out <subject>_p2.json --config delcul-2007

hexa run scripts/consciousness-measurement-protocol/p3_pas_link.hexa \
  --p1 <subject>_p1.json --p2 <subject>_p2.json --pas <subject>_pas.csv

hexa run scripts/consciousness-measurement-protocol/p4_bayes_test.hexa \
  --group <project_dir> --prior casali-dehaene-2011 --eps 0.05
```

(.hexa scripts will be implemented in follow-up commits; this paper focuses on protocol specification.)

---

## 6. Red Team refutation path — CONJECTURE-discard conditions

This §6 pre-registers the **data scenarios under which BT-19 is discarded**. Any result matching one of these conditions yields **formal discard of the BT-19 CONJECTURE** and removal of the `consciousness-r6-hypothesis [7?]` node from atlas.n6.

### 6.1 F1 — α median outside range

- P1 result: α_IIT median 95% CI outside **4/3 ± ε** (ε = 0.10)
- Or P2 result: α_GWT median 95% CI outside **3/4 ± ε**
- **Reason**: theoretical-value prediction failure of either measurement itself

### 6.2 F2 — product deviates from 1

- P4 result: 95% CI of $\alpha_{\mathrm{IIT}} \cdot \alpha_{\mathrm{GWT}}$ outside **1 ± ε**
- Bayes: BF₁₀ < 1/10
- **Reason**: the product of the two exponents is statistically distinguishable from 1 → the R(6)=1 structural-isomorphism hypothesis is rejected

### 6.3 F3 — measurement non-independence evidence

- Simultaneous EEG-fMRI measurement shows LZc and BOLD ignition with **partial correlation |ρ| > 0.7**
- **Reason**: the two measurements are effectively different readings of the same neural process — **violation of no-self-reference principle** (Condition A violation)

### 6.4 F4 — Barrett/Tononi definition inconsistency

- Barrett-Seth α and Tononi Φ (PyPhi) on the same simulation network show **anti-correlation** (ρ < −0.3)
- **Reason**: "complexity index" and "integrated information" measure different objects → Condition B violation

### 6.5 F5 — PAS parallel-structure failure

- Along the PAS 1→2→3→4 step transitions, α_IIT and α_GWT change **non-monotonically**
- **Reason**: the subjective monotonicity of conscious access is not reflected in the two scale exponents → the exponents are unrelated to consciousness

### 6.6 F6 — anaesthesia/sleep control failure

- In deep anaesthesia (PCI < 0.2), α_IIT × α_GWT is still near 1
- **Reason**: the product is a numerical equivalence unrelated to conscious state (false positive)

### 6.7 F7 — replication failure

- Replication run at 3 independent labs; all 3 satisfy at least one of F1~F6
- **Reason**: site-specific artefact in the original measurement

### 6.8 Discard declaration procedure

If **any** of F1 ~ F7 is met:

1. §9 of this paper declares "FALSIFIED" with the relevant F number
2. Remove the `@X consciousness-r6-hypothesis` node from atlas.n6
3. Change BT-19 status in `reports/breakthroughs/consciousness-triple-fusion-2026-04-15.md` to `REFUTED`
4. Propagate "dependent-parent discard" to the P5 paper (if it exists)

---

## 7. Data-sharing plan

### 7.1 OpenNeuro registration

- **Dataset category**: BIDS-compliant EEG + fMRI
- **BIDS version**: 1.8.0
- **Required fields**:
  - `participants.tsv`: age, sex, handedness, PAS baseline, clinical status
  - `dataset_description.json`: Name = "Consciousness Measurement Protocol (BT-19)"
  - `README`: link to this paper + L0 signature
- **Expected size**: n=50 subjects × (EEG 60ch 1h + fMRI 40 min) ≈ 500 GB

### 7.2 NeuroVault upload

- **Statistical maps**: per-subject P2 GLM z-maps (seen vs not-seen contrast, per SOA)
- **Group maps**: second-level random-effects, regression coefficients of α_GWT across 12 ROIs
- **Metadata**: cognitive atlas task ID, research DOI (Zenodo preprint)

### 7.3 HEXA verification-code sharing

- `experiments/consciousness-measurement-protocol/` directory
- `p1_pci_lzc.hexa`, `p2_fmri_ignition.hexa`, `p3_pas_link.hexa`, `p4_bayes_test.hexa`
- README.md with OpenBCI 16ch partial-reproduction guide
- License: MIT (.hexa) + CC-BY-SA-4.0 (docs)

### 7.4 Pre-registration

- **OSF (Open Science Framework)**: commit the protocol. Publish prediction hypotheses (α ± ε, BF criteria, discard conditions) **before** data collection
- **Registered Report** format: targeted submission to *Nature Human Behaviour* or *eLife*
- Obtain immutable timestamp → prevent post-hoc interpretation bias

### 7.5 Privacy protection

- EEG/fMRI raw data: defaced + anonymised BIDS IDs only
- PAS data: per-subject csv, identifiers removed
- IRB approval required (Korea Univ. Med / Seoul Nat'l Univ. Hospital / Samsung Medical Center etc., Korean Neurological Association guidelines)

---

## 8. Expected schedule and resources

### 8.1 Timeline by phase

| Phase | Duration | Deliverable |
|---|---|---|
| Protocol pre-registration (OSF) | 1 month | registered-report draft |
| IRB approval | 2 ~ 3 months | IRB approval |
| OpenBCI 16ch pilot (n=5) | 2 months | P1 feasibility verification |
| Full EEG 60ch experiment (n=50) | 6 months | P1 data + α_IIT |
| fMRI experiment (n=50) | 6 months (can be parallel) | P2 data + α_GWT |
| P3/P4 analysis | 3 months | BF + 95% CI |
| Publication | 3 months | journal submission + revision |
| **Total** | **≈ 2 years** | 1 independent verification run |

### 8.2 Budget estimate (under Korean university collaboration)

| Item | Cost |
|---|---|
| EEG system time (60ch rental or shared) | KRW 3M |
| fMRI scanning (3T, 50 subjects × 2 h) | KRW 40M |
| Subject honoraria (50 × KRW 100K) | KRW 5M |
| Consumables (gel, caps, electrodes) | KRW 2M |
| Analyst (1 PhD × 2 years partial) | KRW 60M |
| **Total** | **≈ KRW 110M** |

### 8.3 OpenBCI 16ch pre-exploration (low-cost path)

With user-owned equipment, estimated **cost ≤ KRW 2M** (additional caps, gel, compute):

- Self + 5 volunteer subjects (check IRB suitability — self-measurement is usually within scope)
- Tasks: eyes-open/closed, n-back, simple masking (no parallel fMRI)
- Output: α_IIT regression feasibility, estimability within ε=0.10 tolerance
- Results refine the power analysis of the formal 60ch experiment

---

## 9. Limitations (Honest Limitations)

### 9.1 Limitation 1 — PCI and α_IIT are not the same proxy

Casali PCI is a TMS-EEG-based **state scalar**; Barrett-Seth α is an EEG-only **scaling exponent**. The two indices are **not identical**. This protocol measures α_IIT primarily and uses PCI as a correlation check. **Status: acknowledged. Grade [7].**

### 9.2 Limitation 2 — debate over the α-definition in GWT broadcasting

Dehaene 2011's α_GWT is interpreted as an fMRI spatial-spread exponent, but some literature (Lamme 2006) criticises it as an "ill-understood ad-hoc formula". No academic consensus on the theoretical basis of α itself. **Response**: this protocol re-estimates α_GWT via empirical regression and commits to the pre-registered definition. **Status: debate ongoing. Grade [6].**

### 9.3 Limitation 3 — is the R(6)=1 arithmetic isomorphism privileged for consciousness interpretation?

`(4/3)(3/4)=1` is **arithmetically trivial**. If α_IIT = 4/3 and α_GWT = 3/4 are (hypothetically) confirmed, their product being 1 risks tautology. This protocol first verifies **the theoretical-value agreement of each exponent** and regards product=1 as the **consequence**. BT-19's real claim concerns **the exact values of each exponent**; the product is aesthetic but less informative. **Status: methodological limit. Grade [5?].**

### 9.4 Limitation 4 — hard problem (Chalmers) unresolved

This protocol addresses only the **access function** of consciousness (access consciousness, Block 1995). **Phenomenal consciousness** (phenomenal, qualia) lies outside this framework. PAS is subjective report but still a proxy of access consciousness. **Status: scope limit. Unresolvable.**

### 9.5 Limitation 5 — single-paradigm dependence

Backward masking does not cover every aspect of GWT. Attentional blink, binocular rivalry, change blindness etc. are needed as supplementary paradigms. **Response**: §3.6's AB alternative; an additional P2' (future extension) is proposed.

### 9.6 Limitation 6 — OpenBCI 16ch deviation from Casali standard

Reproduces only ~58% of the 60ch protocol's information (Sarasso 2015). Expected 1.5 ~ 2× increase in α_IIT standard error. **Response**: formal verification uses research-grade equipment; OpenBCI is feasibility check only.

### 9.7 Limitation 7 — self-reference risk

The paper's authors (Park Minwoo & NEXUS-6) are inside the n=6 framework. Choice of "success" tolerance ε risks n=6-friendly bias. **Response**: **pre-registration** (§7.4) freezes criteria before data. Independent-lab replication (F7) is a mandatory condition.

---

## 10. Summary

This paper organises the 4-protocol manual (P1 EEG PCI, P2 fMRI ignition, P3 PAS, P4 Bayes test) for independent verification of the BT-19 CONJECTURE.

Core contributions:

1. Concretise the PAPER-P7-1 §7 draft into an **equipment-standard + preprocessing-pipeline + statistical-test** manual
2. Symmetrically specify the **7-item Red Team refutation path** (F1~F7) as CONJECTURE discard conditions
3. Present the **OpenBCI 16ch partial-reproduction** path for low-cost pre-exploration (per user equipment)
4. Data-sharing plan via OpenNeuro + NeuroVault + OSF pre-registration

**One-sentence summary**: BT-19 (α_IIT · α_GWT = 1) is currently a [7?] CONJECTURE, and upon results from this protocol it will either be promoted to [10*] or **formally discarded**.

**Follow-up work**:

- Implement 4 `.hexa` scripts under `experiments/consciousness-measurement-protocol/`
- OSF pre-registration draft
- OpenBCI 16ch feasibility pilot (n=1 self-measurement → n=5 volunteer subjects)
- Explore research collaborations (Korean Neurological Association member labs)

---

## 11. References

### 11.1 IIT / PCI family

- Casali, A. G., Gosseries, O., Rosanova, M., Boly, M., Sarasso, S., Casali, K. R., Casarotto, S., Bruno, M.-A., Laureys, S., Tononi, G., & Massimini, M. (2013). **A theoretically based index of consciousness independent of sensory processing and behavior**. *Science Translational Medicine*, 5(198), 198ra105. https://doi.org/10.1126/scitranslmed.3006294
- Sarasso, S., Boly, M., Napolitani, M., Gosseries, O., Charland-Verville, V., Casarotto, S., Rosanova, M., Casali, A. G., Brichant, J.-F., Boveroux, P., Rex, S., Tononi, G., Laureys, S., & Massimini, M. (2015). **Consciousness and complexity during unresponsiveness induced by propofol, xenon, and ketamine**. *Current Biology*, 25(23), 3099-3105.
- Casarotto, S., Comanducci, A., Rosanova, M., Sarasso, S., Fecchio, M., Napolitani, M., Pigorini, A., G. Casali, A., Trimarchi, P. D., Boly, M., Gosseries, O., Bodart, O., Curto, F., Landi, C., Mariotti, M., Devalle, G., Laureys, S., Tononi, G., & Massimini, M. (2016). **Stratification of unresponsive patients by an independently validated index of brain complexity**. *Annals of Neurology*, 80(5), 718-729.
- Massimini, M., Ferrarelli, F., Huber, R., Esser, S. K., Singh, H., & Tononi, G. (2005). **Breakdown of cortical effective connectivity during sleep**. *Science*, 309(5744), 2228-2232.
- Tononi, G., Boly, M., Massimini, M., & Koch, C. (2016). **Integrated information theory: from consciousness to its physical substrate**. *Nature Reviews Neuroscience*, 17(7), 450-461.
- Barrett, A. B., & Seth, A. K. (2011). **Practical measures of integrated information for time-series data**. *PLoS Computational Biology*, 7(1), e1001052.
- Oizumi, M., Albantakis, L., & Tononi, G. (2014). **From the phenomenology to the mechanisms of consciousness: Integrated Information Theory 3.0**. *PLoS Computational Biology*, 10(5), e1003588.
- Mayner, W. G. P., Marshall, W., Albantakis, L., Findlay, G., Marchman, R., & Tononi, G. (2018). **PyPhi: A toolbox for integrated information theory**. *PLoS Computational Biology*, 14(7), e1006343.
- Lempel, A., & Ziv, J. (1976). **On the complexity of finite sequences**. *IEEE Transactions on Information Theory*, 22(1), 75-81.

### 11.2 GWT / Ignition family

- Dehaene, S., & Changeux, J.-P. (2011). **Experimental and theoretical approaches to conscious processing**. *Neuron*, 70(2), 200-227.
- Dehaene, S. (2014). **Consciousness and the Brain: Deciphering How the Brain Codes Our Thoughts**. Viking.
- Del Cul, A., Baillet, S., & Dehaene, S. (2007). **Brain dynamics underlying the nonlinear threshold for access to consciousness**. *PLoS Biology*, 5(10), e260.
- Sergent, C., Baillet, S., & Dehaene, S. (2005). **Timing of the brain events underlying access to consciousness during the attentional blink**. *Nature Neuroscience*, 8(10), 1391-1400.
- Dehaene, S., Sergent, C., & Changeux, J.-P. (2003). **A neuronal network model linking subjective reports and objective physiological data during conscious perception**. *PNAS*, 100(14), 8520-8525.
- Baars, B. J. (1988). **A Cognitive Theory of Consciousness**. Cambridge University Press.
- Mashour, G. A., Roelfsema, P., Changeux, J.-P., & Dehaene, S. (2020). **Conscious processing and the Global Neuronal Workspace hypothesis**. *Neuron*, 105(5), 776-798.

### 11.3 PAS / subjective-report family

- Ramsøy, T. Z., & Overgaard, M. (2004). **Introspection and subliminal perception**. *Phenomenology and the Cognitive Sciences*, 3(1), 1-23.
- Sandberg, K., Timmermans, B., Overgaard, M., & Cleeremans, A. (2010). **Measuring consciousness: Is one measure better than the other?**. *Consciousness and Cognition*, 19(4), 1069-1078.
- Overgaard, M., & Sandberg, K. (2012). **Kinds of access: different methods for report reveal different kinds of metacognitive access**. *Philosophical Transactions of the Royal Society B*, 367(1594), 1287-1296.
- Maniscalco, B., & Lau, H. (2012). **A signal detection theoretic approach for estimating metacognitive sensitivity from confidence ratings**. *Consciousness and Cognition*, 21(1), 422-430.
- Frässle, S., Sommer, J., Jansen, A., Naber, M., & Einhäuser, W. (2014). **Binocular rivalry: frontal activity relates to introspection and action but not to perception**. *Journal of Neuroscience*, 34(5), 1738-1747.

### 11.4 NCC / methodology

- Koch, C., Massimini, M., Boly, M., & Tononi, G. (2016). **Neural correlates of consciousness: progress and problems**. *Nature Reviews Neuroscience*, 17(5), 307-321.
- Block, N. (1995). **On a confusion about a function of consciousness**. *Behavioral and Brain Sciences*, 18(2), 227-247.
- Lamme, V. A. F. (2006). **Towards a true neural stance on consciousness**. *Trends in Cognitive Sciences*, 10(11), 494-501.
- Esteban, O., Markiewicz, C. J., Blair, R. W., Moodie, C. A., Isik, A. I., Erramuzpe, A., Kent, J. D., Goncalves, M., DuPre, E., Snyder, M., Oya, H., Ghosh, S. S., Wright, J., Durnez, J., Poldrack, R. A., & Gorgolewski, K. J. (2019). **fMRIPrep: a robust preprocessing pipeline for functional MRI**. *Nature Methods*, 16(1), 111-116.
- Maris, E., & Oostenveld, R. (2007). **Nonparametric statistical testing of EEG- and MEG-data**. *Journal of Neuroscience Methods*, 164(1), 177-190.

### 11.5 Statistics / pre-registration

- Wagenmakers, E.-J. (2011). **A practical solution to the pervasive problems of p values**. *Psychonomic Bulletin & Review*, 14(5), 779-804.
- Jeffreys, H. (1961). **Theory of Probability** (3rd ed.). Oxford University Press.
- Schönbrodt, F. D., & Wagenmakers, E.-J. (2018). **Bayes factor design analysis**. *Psychonomic Bulletin & Review*, 25, 128-142.
- Benjamin, D. J., Berger, J. O., Johannesson, M., Nosek, B. A., Wagenmakers, E.-J., et al. (2018). **Redefine statistical significance**. *Nature Human Behaviour*, 2(1), 6-10.
- Nosek, B. A., Ebersole, C. R., DeHaven, A. C., & Mellor, D. T. (2018). **The preregistration revolution**. *PNAS*, 115(11), 2600-2606.

### 11.6 n6-architecture precursor papers

- Park Minwoo & NEXUS-6 collaboration (2026). **Consciousness phase diagram — cognitive-scientific projection of σ(n)·φ(n) = n·τ(n) uniqueness**. n6-architecture, `papers/n6-consciousness-phase-diagram-paper.md`.
- Park Minwoo (2026). **Consciousness triple fusion — thermodynamics · AI · quantum critical-point exploration (BT-19 CONJECTURE)**. n6-architecture, `reports/breakthroughs/consciousness-triple-fusion-2026-04-15.md`.
- Park Minwoo (2026). **HEXA-CONSCIOUSNESS-SOC — consciousness SoC n=6 coordinate mapping**. n6-architecture, `papers/n6-consciousness-soc-paper.md`.
- Park Minwoo (2026). **σ(n)·φ(n) = n·τ(n) iff n=6 — 3 independent candidate verification drafts**. n6-architecture, `theory/proofs/theorem-r1-uniqueness.md`.

---

**Promotion procedure**: after receiving this protocol's results, BT-19 is judged on one of the following paths.

- **Success path** (none of F1~F7 met + BF₁₀ > 10): [7?] → [10*], promote the atlas.n6 `consciousness-r6-hypothesis` node to EXACT grade
- **Discard path** (one of F1~F7 met): [7?] → REFUTED, remove the node and add a §9 FALSIFIED declaration to this paper

Per this v1 paper's summary grade: **PROTOCOL DRAFT (no grade — pre-experiment)**. The protocol only guarantees methodological completeness.

**End (v1, drafted 2026-04-15, PAPER-P8-2).**

## §1 WHY

This section covers why for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §2 COMPARE

This section covers compare for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §3 REQUIRES

This section covers requires for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §4 STRUCT

This section covers struct for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §5 FLOW

This section covers flow for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §6 EVOLVE

This section covers evolve for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §7 VERIFY

This section covers verify for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §8 EXEC SUMMARY

This section covers exec summary for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §9 SYSTEM REQUIREMENTS

This section covers system requirements for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §10 ARCHITECTURE

This section covers architecture for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §11 CIRCUIT DESIGN

This section covers circuit design for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §12 PCB DESIGN

This section covers pcb design for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §13 FIRMWARE

This section covers firmware for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §14 MECHANICAL

This section covers mechanical for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §15 MANUFACTURING

This section covers manufacturing for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §16 TEST & QUALIFICATION

This section covers test & qualification for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §17 BOM

This section covers bom for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §18 VENDOR & SCHEDULE

This section covers vendor & schedule for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §19 ACCEPTANCE CRITERIA

This section covers acceptance criteria for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §20 APPENDIX

This section covers appendix for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## §21 IMPACT per Mk

This section covers impact per mk for the paper. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent Mk iterations.

## mk_history

- Mk.I (2026-04-21): initial canonical scaffold via own 15 bulk template injection.
- Mk.II: pending — fill per-section content with domain expert review.
- Mk.III: pending — full verification data + external citations.
