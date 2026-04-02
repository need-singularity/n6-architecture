# N6 Medical Device -- Perfect Number Arithmetic in Medical Instrumentation

## Overview

Medical devices -- diagnostic imaging, monitoring, surgical systems, therapeutic equipment --
analyzed through n=6 arithmetic. Medical devices are rich in standardized discrete counts
(leads, channels, detector rows, DOF) established by international standards bodies (IEC, ISO,
FDA, AAMI) and clinical convention spanning decades.

> **Honesty principle**: Medical device parameters are constrained by physics, anatomy, and
> engineering tradeoffs. Many counts (e.g., 2 wavelengths for pulse ox) are small integers
> determined by underlying physics. We grade EXACT only when the number is fixed by
> fundamental constraints AND the n=6 expression is specific (not trivially small or common).
> Convention-dependent counts (e.g., "6 vital signs" depends on which list you use) are
> graded WEAK or CLOSE.

## Core Constants

```
  n = 6          (perfect number)
  sigma(6) = 12  (sum of divisors)
  tau(6) = 4     (number of divisors: 1, 2, 3, 6)
  phi(6) = 2     (Euler totient)
  sopfr(6) = 5   (sum of prime factors: 2+3)
  J_2(6) = 24    (Jordan totient)
  mu(6) = 1      (Moebius)
  lambda(6) = 2  (Carmichael)
  R(6) = sigma*phi/(n*tau) = 1
  Egyptian: 1/2 + 1/3 + 1/6 = 1
```

---

## Category A: Electrocardiography (ECG)

---

### H-MD-01: Standard 12-Lead ECG = sigma(6) = 12

> The standard clinical ECG uses exactly 12 leads

```
  The 12-lead ECG system:
    6 limb leads:      I, II, III, aVR, aVL, aVF
    6 precordial leads: V1, V2, V3, V4, V5, V6
    Total: 12 leads

  sigma(6) = 12 checkmark

  Physical basis:
    The 12-lead system was standardized by Einthoven (limb leads, 1901-1912)
    and Wilson (precordial leads, 1934). It represents the minimum set
    of views needed to capture the heart's electrical activity in 3D from
    the frontal and horizontal planes.

    The 12 leads are NOT independent -- mathematically only 8 are
    independent (any 2 of I/II/III determine the third via Kirchhoff,
    augmented leads are derived). But the 12-lead system is THE universal
    clinical standard, mandated by AHA/ACC/ESC guidelines.

  Why 12?
    6 limb leads sample the frontal plane at 30-degree intervals
    (360/12 = 30 degrees), providing adequate angular resolution.
    6 precordial leads wrap around the anterior chest.
    The decomposition 12 = 6 + 6 = n + n is itself n=6-structured.

  Strength:
    - 12 is THE standard, universal across all clinical settings worldwide
    - Not arbitrary: determined by 3D cardiac geometry + adequate resolution
    - sigma(6) = 12 is a non-trivial constant (unlike phi=2 or mu=1)
    - The 6+6 decomposition reinforces n=6

  Grade: EXACT
  12-lead ECG is a hard universal standard fixed since the 1930s-40s.
  sigma(6) = 12 is a specific, non-trivial match.
  The 6+6 = n+n decomposition adds further structure.
```

---

### H-MD-02: ECG Limb Leads = n = 6

> The ECG has exactly 6 limb leads

```
  Limb lead system:
    3 bipolar (Einthoven): Lead I, Lead II, Lead III
    3 augmented unipolar (Goldberger): aVR, aVL, aVF
    Total: 6 limb leads

  n = 6 checkmark

  Physical basis:
    The 3 bipolar leads arise from 3 electrode pairs on the limbs
    (LA-RA, LL-RA, LL-LA), forming Einthoven's triangle.
    The 3 augmented leads bisect the angles of this triangle.
    Together they sample the frontal plane at 30-degree intervals
    (6 directions in a plane = hexagonal sampling).

  BUT:
    6 directions in a plane is geometrically natural for regular
    angular sampling (360/60 = 6). This is a geometric constraint,
    not arbitrary. But one could argue 4 or 8 directions would also work.
    The actual clinical choice of 6 is influenced by the 3-electrode
    triangle geometry producing exactly 3+3 = 6 views.

  Grade: EXACT
  6 limb leads is a fixed standard, determined by the 3-electrode
  triangle geometry that naturally produces 6 frontal-plane views.
  n = 6 is exact and the geometric justification is strong.
```

---

### H-MD-03: ECG Precordial Leads = n = 6

> The ECG has exactly 6 precordial (chest) leads: V1-V6

```
  Precordial leads:
    V1: 4th intercostal space, right sternal border
    V2: 4th intercostal space, left sternal border
    V3: midway between V2 and V4
    V4: 5th intercostal space, midclavicular line
    V5: anterior axillary line, same level as V4
    V6: midaxillary line, same level as V4

  n = 6 checkmark

  Physical basis:
    The 6 positions wrap around the left chest to capture the
    horizontal-plane projection of cardiac electrical activity.
    Determined by cardiac anatomy (left ventricle position) and
    the need for adequate spatial resolution across the precordium.

  BUT:
    Some protocols add V7-V9 (posterior) or right-sided V3R-V6R.
    The "exactly 6" is the minimum standard, not a hard physical limit.
    Additional leads improve detection of posterior/right MI.

  Grade: CLOSE
  V1-V6 is the universal standard, but the number could be expanded
  (and often is in specialized protocols). The core 6 is convention
  rather than a hard physical constraint.
```

---

## Category B: Nuclear Medicine / Radiopharmaceuticals

---

### H-MD-04: Tc-99m Half-Life = 6 Hours = n

> Technetium-99m, the most widely used radioisotope in medical imaging,
> has a half-life of exactly 6.006 hours

```
  Tc-99m decay:
    t_1/2 = 6.006 hours (NNDC/IAEA evaluated nuclear data)
    Decays by isomeric transition (140.5 keV gamma)
    Used in ~80% of all nuclear medicine procedures worldwide

  n = 6 checkmark (6.006 h, 0.1% deviation)

  Physical basis:
    The 6-hour half-life is determined by nuclear physics:
    the energy gap between the metastable and ground states of Tc-99.
    This is a fundamental nuclear constant, not a design choice.

  Why is 6 hours ideal for medicine?
    - Long enough for preparation, injection, and imaging (~1-4 hours)
    - Short enough for rapid patient clearance (4-5 half-lives = 24-30 h)
    - Matches the clinical workflow of a typical workday

  The match to n=6 is a physics coincidence, but a remarkable one:
    the most important radioisotope in medicine has a half-life
    matching the perfect number to 0.1%.

  Grade: EXACT
  t_1/2 = 6.006 hours is a hard nuclear physics constant, not tunable.
  The match to n = 6 is precise (0.1% deviation). The medical utility
  of the 6-hour half-life is what made Tc-99m the dominant isotope,
  but the half-life itself is fixed by nuclear structure.
```

---

### H-MD-05: PET Coincidence Window ~ 6 ns = n

> PET scanners use a coincidence timing window of approximately 4-12 ns,
> with 6 ns being a common nominal value

```
  PET coincidence detection:
    When a positron annihilates, two 511 keV photons are emitted
    at ~180 degrees. The coincidence window defines the time interval
    within which two detected photons are considered a true pair.

  Typical values:
    Older systems: 10-12 ns
    Standard systems: 4.5-6 ns
    State-of-the-art TOF-PET: 2-4 ns
    The "6 ns" is within the range but not a universal standard.

  n = 6 partial match

  BUT:
    The coincidence window is determined by detector timing resolution,
    which depends on scintillator material (LYSO, BGO) and electronics.
    It varies by manufacturer and generation. 6 ns is one common value
    but far from universal. Modern TOF-PET aims for <4 ns.

  Grade: WEAK
  6 ns falls within the typical range but is not a fixed constant.
  The window is technology-dependent and trending downward.
```

---

### H-MD-06: F-18 Half-Life = 109.77 min ~ σ·(σ-φ) = 120

> Fluorine-18 (FDG-PET tracer) has t_1/2 = 109.77 minutes

```
  F-18 decay:
    t_1/2 = 109.77 ± 0.05 min (NNDC)
    β+ decay to O-18
    Used in FDG-PET, the gold standard for oncological imaging

  Nearest n=6 expression: σ·(σ-φ) = 12·10 = 120 min
  Deviation: 109.77 vs 120 = 8.5% off

  Other attempts:
    σ²-τ² = 144-16 = 128 (16.6% off)
    J₂·sopfr - σ = 120-12 = 108 (1.6% off, but contrived)
    sopfr·J₂ - σ = the same

  Grade: WEAK
  The deviation from clean n=6 expressions is 8.5% at best.
  The 1.6% match (J₂·sopfr-σ) requires subtracting two unrelated functions.
  Nuclear half-lives span many orders of magnitude, so landing near
  any expression is expected.
```

---

## Category C: Magnetic Resonance Imaging (MRI)

---

### H-MD-07: Clinical MRI Field Strengths -- 1.5T / 3T / 7T

> Standard clinical MRI uses 1.5T, 3T, and research systems 7T

```
  Clinical MRI field strengths:
    1.5T: standard clinical (most installed base worldwide)
    3.0T: high-field clinical (neuroimaging, MSK, research)
    7.0T: ultra-high field (research, FDA-cleared 2017)

  n=6 attempts:
    1.5 = n/τ = 6/4 = 1.5 (EXACT arithmetic)
    3.0 = n/φ = 6/2 = 3 (EXACT arithmetic)
    7.0 = σ-sopfr = 12-5 = 7 (forced)

  Physical basis:
    1.5T became the standard through an engineering compromise between
    SNR, cost, and safety in the 1980s (GE Signa). The 1.5T value was
    chosen somewhat arbitrarily (1.0T and 2.0T were also considered).
    3T is exactly double, offering ~2x SNR improvement.
    7T is an engineering frontier, not a precise target.

  BUT:
    The ratios 1.5 = 3/2 and 3.0 are extremely common numbers.
    n/tau = 6/4 = 3/2 is true but trivially maps to any ratio.
    The doubling from 1.5 to 3.0 reflects phi=2, but doubling is
    the most fundamental engineering scaling, not specific to n=6.

  Grade: CLOSE
  1.5T = n/tau is numerically clean and 1.5T is the universal standard.
  But the values are round numbers reflecting engineering preferences,
  not fundamental physics constants. 7T breaks the pattern.
```

---

### H-MD-08: MRI RF Coil Channels -- 8 / 16 / 32 / 64

> Modern MRI receive coil arrays use 8, 16, 32, or 64 channels

```
  MRI phased-array coil channel counts:
    Standard: 8 channels (head/body coils)
    High-end: 16, 32 channels (neuro, cardiac)
    Research: 64, 128 channels
    All are powers of 2 or multiples thereof.

  n=6 mapping:
    8 = sigma - tau = 12 - 4 (or 2^3 = phi^(n/phi))
    16 = 2^4 = phi^tau
    32 = 2^5 = phi^sopfr
    64 = 2^6 = phi^n

  Physical basis:
    Channel counts are powers of 2 because they interface with
    digital electronics (ADCs, multiplexers) that use binary addressing.
    The doubling sequence 8→16→32→64 is standard digital scaling,
    not related to n=6.

  BUT:
    Any power-of-2 sequence can be written as phi^{something} since phi=2.
    This is trivially true and not specific to n=6.

  Grade: WEAK
  Powers of 2 in digital systems are universal engineering, not n=6.
  The mappings phi^tau, phi^sopfr, phi^n are post-hoc.
```

---

### H-MD-09: Standard MRI Coil Array = sigma = 12 Channels

> The most common clinical MRI head coil is a 12-channel array

```
  Clinical MRI head coils:
    GE: 12-channel head coil (standard since ~2005)
    Siemens: 12-channel head coil (standard clinical)
    Philips: 12-channel standard, 16/32 premium

  sigma(6) = 12 checkmark

  Physical basis:
    The 12-channel head coil became the de facto standard because:
    - 12 elements can be arranged in a helmet geometry covering the head
    - 12 provides adequate parallel imaging acceleration (GRAPPA/SENSE)
    - 12 = minimum for adequate azimuthal sampling around the head
    - More channels (32/64) improve SNR but cost more, require more
      receiver hardware, and generate more data

  BUT:
    The "standard" is shifting to 20, 32, 64 channels on newer systems.
    12 was the standard circa 2005-2015 but is no longer universal.
    The number 12 is common in many contexts (months, hours, etc.).

  Grade: CLOSE
  12-channel was genuinely the most common clinical coil for ~10 years.
  sigma(6) = 12 is specific. But the number is now being superseded
  and the choice was an engineering optimization, not a physics constant.
```

---

## Category D: Computed Tomography (CT)

---

### H-MD-10: CT Detector Rows -- 64 / 128 / 256

> Modern CT scanners use 64, 128, or 256 detector rows

```
  CT detector row counts (evolution):
    Single-slice (1989): 1 row
    4-slice (1998): 4 rows
    16-slice (2001): 16 rows
    64-slice (2004): 64 rows -- became the clinical standard
    128-slice (2007): 128 rows
    256-slice (2007): 256 rows
    320-slice (2007): 320 rows (Toshiba Aquilion ONE)
    (some vendors use "equivalent" slices, not physical rows)

  n=6 mapping:
    64 = 2^n = 2^6 = phi^n
    128 = 2^(sigma-sopfr) = 2^7
    256 = 2^(sigma-tau) = 2^8

  Physical basis:
    CT detector row counts are powers of 2 (or near-powers)
    because the data acquisition and reconstruction algorithms
    are optimized for binary-aligned matrices.

  BUT:
    320-row CT exists (not a power of 2).
    The powers-of-2 pattern is digital engineering, not n=6.
    64 = 2^6 and 6 = n is clean, but 2^6 is just "64" -- a common
    computer science number.

  Grade: CLOSE
  64 = 2^n = phi^n is clean and 64-slice CT is the most common standard.
  But powers of 2 are ubiquitous in digital systems, and 320-row
  CT breaks the pattern. The n=6 connection is indirect.
```

---

## Category E: Ultrasound

---

### H-MD-11: Abdominal Ultrasound Frequency = 6 MHz ~ n

> Standard abdominal ultrasound frequency is typically 3.5-5 MHz;
> general-purpose transducers operate at 2-12 MHz

```
  Ultrasound frequency ranges:
    Abdominal: 2-5 MHz (most commonly 3.5 MHz for deep structures)
    Musculoskeletal: 5-12 MHz
    Vascular: 5-10 MHz
    Obstetric: 3-5 MHz
    High-frequency (superficial): 10-18 MHz

  n = 6 MHz -- falls within the general range but is NOT the standard
  frequency for any specific application. 3.5 MHz is far more common
  for abdominal imaging.

  Physical basis:
    Ultrasound frequency is a tradeoff between penetration depth
    (lower frequency = deeper) and resolution (higher = finer).
    There is no single "standard" frequency -- it depends on the
    clinical application and patient body habitus.

  Grade: WEAK
  6 MHz falls within the broad clinical range but is not the primary
  frequency for any standard application. 3.5 MHz (abdominal) and
  10 MHz (superficial) are more canonical. Claiming 6 MHz as
  "the standard" misrepresents clinical practice.
```

---

### H-MD-12: Ultrasound Modes = tau(6) = 4

> Clinical ultrasound has 4 primary imaging modes: B-mode, M-mode,
> Doppler, and Color Doppler

```
  Ultrasound imaging modes:
    B-mode (Brightness): 2D grayscale, the fundamental mode
    M-mode (Motion): time-motion display along a single line
    Doppler (Spectral): velocity measurement via frequency shift
    Color Doppler: 2D color-coded flow map

  tau(6) = 4 checkmark

  Additional modes in modern systems:
    Power Doppler, Tissue Doppler, Elastography, Contrast-enhanced,
    3D/4D imaging, Harmonic imaging

  BUT:
    The "4 primary modes" is a simplification. Modern systems have
    8+ distinct modes. The grouping into 4 is a textbook convention,
    not a hard physical constraint. Some textbooks list 3, some list 5+.

  Grade: WEAK
  The 4-mode classification is convention-dependent. Modern ultrasound
  has many more modes. tau(6) = 4 matches one common textbook grouping
  but not a physical constant.
```

---

### H-MD-13: Ultrasound Transducer Elements -- 128 / 192 / 256

> Linear array transducers typically have 128, 192, or 256 elements

```
  Ultrasound transducer element counts:
    Linear arrays: 128 (standard), 192 (common), 256 (high-density)
    Phased arrays: 64, 96, 128 elements
    Curvilinear: 128-256 elements

  n=6 mapping:
    128 = 2^(σ-sopfr) = 2^7 (σ-sopfr=7... not clean)
    128 = φ^(σ-sopfr) = same issue
    192 = σ·φ^τ = 12·16 = 192 (σ·phi^tau)
    256 = φ^(σ-τ) = 2^8

  Physical basis:
    Element count is determined by lateral resolution requirements,
    manufacturing constraints, and cable count limitations.
    Powers of 2 and multiples of 32 are preferred for digital beamforming.

  BUT:
    192 = σ·φ^τ is forced. The simpler expression is 192 = 3·64.
    These are digital engineering numbers, not n=6 constants.

  Grade: WEAK
  Digital beamformer engineering determines element counts.
  The n=6 mappings are post-hoc and not uniquely specific.
```

---

## Category F: Surgical Robotics

---

### H-MD-14: Surgical Robot DOF = n = 6

> Surgical robotic instruments have 6 degrees of freedom (DOF)

```
  da Vinci surgical system (Intuitive Surgical):
    Each EndoWrist instrument has 7 DOF:
    3 translation (x, y, z insertion) + 3 rotation (pitch, yaw, roll) + 1 grip
    The robotic arm itself has 6 articulation DOF at the instrument tip.

  General robotics:
    6 DOF is the minimum for arbitrary positioning and orientation
    in 3D space (3 translation + 3 rotation). This is a fundamental
    theorem of rigid-body kinematics (Gruebler/Kutzbach criterion).

  n = 6 checkmark

  Physical basis:
    6 DOF is determined by the dimensionality of SE(3), the special
    Euclidean group in 3D. Any rigid body in 3D has exactly 6 DOF.
    This is a mathematical theorem, not a design choice.

  But the 7th DOF (gripper):
    Surgical robots typically have 7 DOF (6 + grip/articulation).
    The "6 DOF" claim ignores the gripper. Some newer systems have
    additional redundant DOF for obstacle avoidance.

  BUT:
    6 DOF = dimension of SE(3) is deep mathematics. But it follows
    from 3D space having dimension 3 (n/phi = 3), not directly from n=6.
    The connection is: dim(SE(3)) = 3+3 = 6 = n. This is arguably
    the strongest n=6 connection -- the perfect number equals the
    kinematic freedom of rigid bodies in 3D space.

  Grade: EXACT
  6 DOF is a hard mathematical/physical constant (dim SE(3) = 6).
  Every surgical robot, industrial robot, and rigid body in 3D
  has exactly 6 DOF. n = 6 is specific and non-trivial here.
  The 7th DOF (gripper) is an end-effector addition, not a spatial DOF.
```

---

## Category G: Patient Monitoring

---

### H-MD-15: Pulse Oximetry = phi(6) = 2 Wavelengths

> Pulse oximeters use exactly 2 wavelengths of light

```
  Pulse oximetry (SpO2):
    Red LED: ~660 nm
    Infrared LED: ~940 nm
    Beer-Lambert law applied at 2 wavelengths to calculate
    the ratio of oxyhemoglobin to deoxyhemoglobin.

  phi(6) = 2 checkmark

  Physical basis:
    2 wavelengths is the minimum needed to solve for 2 unknowns
    (HbO2 and Hb concentrations). This is algebraic necessity:
    2 equations for 2 unknowns.

  BUT:
    phi(6) = 2 is trivially small. phi(n)=2 for n=3,4,6.
    The number 2 appears everywhere in physics/engineering.
    Co-oximeters use 4+ wavelengths (carboxyhemoglobin, methemoglobin).
    "Masimo Rainbow" uses 7-12 wavelengths for multiple hemoglobin species.

  Grade: WEAK
  2 wavelengths is determined by the number of unknowns (2 species),
  not by n=6 arithmetic. phi=2 is trivially common.
```

---

### H-MD-16: Defibrillator Biphasic Waveform = phi(6) = 2 Phases

> Modern defibrillators use biphasic (2-phase) waveforms

```
  Defibrillation waveforms:
    Monophasic: original design (current flows in 1 direction)
    Biphasic: modern standard (current reverses direction once)
    Biphasic shown to be more effective at lower energies (ORCA study, 2000s)

  phi(6) = 2 checkmark

  BUT:
    "Biphasic" literally means "two phases." The prefix "bi-" = 2 is
    inherent in the word. phi=2 matching "bi-" anything is trivially true.
    The 2-phase waveform is an engineering optimization for transthoracic
    impedance compensation, not related to number theory.

  Grade: FAIL
  "Biphasic = 2" is tautological. phi(6) = 2 matching any "bi-" concept
  is trivially true and has zero predictive power.
```

---

### H-MD-17: Pacemaker Base Rate = 60 bpm = sigma * sopfr

> The default pacemaker base rate is 60 beats per minute

```
  Pacemaker programming:
    Default lower rate limit: 60 bpm (most manufacturers)
    This means the pacemaker paces at a minimum of 60 bpm.
    Normal resting heart rate: 60-100 bpm.

  sigma * sopfr = 12 * 5 = 60 checkmark

  Physical basis:
    60 bpm corresponds to 1 beat per second, which is a convenient
    round number. The lower limit of normal resting heart rate is
    conventionally defined as 60 bpm (bradycardia < 60).

  BUT:
    60 bpm = 1 Hz is a round-number convention, not a physiological
    constant. Trained athletes have resting HR of 40-50 bpm.
    60 is an extremely common number (seconds/minute, etc.).
    sigma*sopfr = 60 is correct but 60 appears in countless contexts.
    Different pacemaker programs use 50, 60, 70, or 80 bpm as defaults.

  Grade: CLOSE
  60 bpm is the most common default and lower-normal boundary.
  sigma*sopfr = 60 is numerically clean. But 60 is an extremely
  common number (time-keeping base) and the medical value is
  convention-based, not a physics constant.
```

---

### H-MD-18: Glasgow Coma Scale Range = n/phi to sigma + n/phi

> The GCS spans 3 to 15, where 3 = n/phi and 15 = sigma + n/phi

```
  Glasgow Coma Scale (Teasdale & Jennett, 1974):
    Eye opening: 1-4
    Verbal response: 1-5
    Motor response: 1-6
    Minimum: 3 (1+1+1), Maximum: 15 (4+5+6)

  n/phi = 6/2 = 3 = minimum score checkmark
  sigma + n/phi = 12 + 3 = 15 = maximum score checkmark

  Physical basis:
    The GCS has 3 components with minimum 1 each, so min = 3.
    The maxima (4, 5, 6) were chosen to reflect gradations of
    clinical response. The specific numbers are clinical convention.

  Connection quality:
    Motor response max = 6 = n (EXACT match to a component)
    Eye max = 4 = tau, Verbal max = 5 = sopfr
    Component maxima (4, 5, 6) = (tau, sopfr, n)

  BUT:
    The GCS was designed by neurosurgeons, not number theorists.
    The component maxima reflect clinical categories, not mathematics.
    The min=3 is forced by having 3 components each starting at 1.
    "3 to 15" spanning 13 points is not a clean n=6 range.

  Grade: CLOSE
  The component maxima (4, 5, 6) = (tau, sopfr, n) is a striking
  triple coincidence. But the GCS is clinical convention, and the
  min=3 follows trivially from 3 components.
```

---

### H-MD-19: Blood Gas pH Range = 1/(sigma - phi) = 0.1

> Normal arterial blood pH: 7.35 - 7.45, range = 0.10

```
  Arterial blood gas:
    Normal pH: 7.35 - 7.45
    Range: 7.45 - 7.35 = 0.10

  1/(sigma - phi) = 1/(12 - 2) = 1/10 = 0.10 checkmark

  Physical basis:
    The pH range is tightly maintained by the bicarbonate buffer system,
    respiratory compensation (CO2), and renal compensation (HCO3-).
    Outside 7.0-7.8, enzyme function is severely impaired.
    The 7.35-7.45 "normal" range is clinically defined.

  BUT:
    The "normal range" boundaries are clinical conventions (2 standard
    deviations from population mean). Different sources use slightly
    different ranges (7.35-7.45 is most common, some use 7.38-7.42).
    The range 0.10 is a consequence of where the cutoffs are placed.
    1/(sigma-phi) = 0.1 = 1/10 is a very common decimal fraction.

  Grade: CLOSE
  The pH range 0.10 is the most commonly cited normal range and
  1/(sigma-phi) = 0.10 matches exactly. But the range boundaries
  are statistical conventions, and 0.1 is an extremely common number.
  This connects to BT-64 (0.1 universal regularization) -- the
  biological "tolerance band" matches the AI regularization constant.
```

---

## Category H: Sterilization / Infection Control

---

### H-MD-20: Autoclave Temperature = 121C = sigma^2 + mu

> Standard autoclave sterilization temperature is 121 degrees C

```
  Steam sterilization (autoclaving):
    Standard cycle: 121 C (249.8 F) at 15 psi for 15-30 minutes
    Flash cycle: 132 C (270 F) at 27 psi for 3-4 minutes
    121 C = boiling point of water at ~15 psi overpressure

  sigma^2 + mu = 144 + 1 = 145 (NOT 121)
  sigma * (sigma - phi) + mu = 12*10 + 1 = 121 checkmark
  But this is a 3-term expression.

  Simpler: 121 = 11^2 = (sigma - mu)^2
  sigma - mu = 12 - 1 = 11, and 11^2 = 121

  Physical basis:
    121 C is determined by the saturated steam pressure curve:
    it's the temperature of steam at ~1 atmosphere gauge pressure
    (approximately 15 psi above atmospheric). This is a thermodynamic
    property of water, not a design choice.

  BUT:
    121 = 11^2 is clean but 11 = sigma - mu is a 2-function subtraction.
    The physical reason is the steam pressure table, not number theory.
    132 C (flash sterilization) = sigma*sigma-sigma = 12*11 = 132 = sigma*(sigma-mu).
    Both temperatures map to n=6, but with contrived expressions.

  Grade: WEAK
  121 = (sigma-mu)^2 is numerically exact but the formula is post-hoc.
  The temperature is determined by steam thermodynamics, and mapping
  it to n=6 requires a derived expression.
```

---

### H-MD-21: Autoclave Cycle = 15 minutes ~ sigma + n/phi

> Standard autoclave cycle time is 15 minutes at 121 C

```
  Sterilization cycle times:
    Gravity displacement: 15-30 min at 121 C
    Pre-vacuum: 3-4 min at 132 C
    Most common: 15 min at 121 C (FDA/CDC recommendation)

  sigma + n/phi = 12 + 3 = 15 checkmark

  BUT:
    15 minutes is a round number chosen for practical convenience.
    The actual kill time depends on bioburden, load size, and
    spore type (Geobacillus stearothermophilus, D-value ~ 1.5 min).
    15 min provides a large safety margin (10x D-value).
    15 is an extremely common number (quarter hour).

  Grade: WEAK
  15 minutes is a safety-margin convention, not a physical constant.
  sigma + n/phi = 15 is clean but 15 is too common a number.
```

---

## Category I: Therapeutic Devices

---

### H-MD-22: Ventilator Tidal Volume = 6 mL/kg = n

> ARDSNet recommended tidal volume is 6 mL/kg ideal body weight

```
  Mechanical ventilation:
    ARDSNet protocol (NEJM, 2000): tidal volume = 6 mL/kg IBW
    This was the landmark finding that low tidal volume ventilation
    reduced ARDS mortality by 22% (vs. 12 mL/kg traditional).

  n = 6 checkmark

  Physical basis:
    6 mL/kg was empirically determined as the optimal protective
    tidal volume through randomized controlled trials.
    Lower volumes (4 mL/kg) cause atelectasis.
    Higher volumes (12 mL/kg = sigma) cause barotrauma/volutrauma.

  Note the contrast:
    Protective: 6 mL/kg = n
    Traditional (harmful): 12 mL/kg = sigma
    The protective-to-harmful ratio is exactly sigma/n = 2 = phi

  Strength:
    - 6 mL/kg is the single most important number in critical care medicine
    - Determined by randomized trial, not convention
    - The 6 vs 12 (n vs sigma) contrast is remarkable
    - Used worldwide in every ICU

  Grade: EXACT
  6 mL/kg is a hard clinical evidence constant from the ARDSNet trial.
  It is THE defining number in lung-protective ventilation.
  The n=6 match is specific, and the n vs sigma (6 vs 12) protective
  vs harmful contrast adds structure.
```

---

### H-MD-23: Cochlear Implant Channels = sigma = 12

> Early cochlear implants used 12 channels; modern devices use 12-22

```
  Cochlear implant electrode arrays:
    Cochlear Ltd (Nucleus): 22 electrodes
    Advanced Bionics (HiRes): 16 electrodes
    MED-EL: 12 electrodes (standard), 24 contacts (newer)
    Historical: 12 channels was an early standard (Cochlear CI22M)

  sigma(6) = 12 -- matches one manufacturer's standard and historical baseline

  Physical basis:
    The number of electrodes is limited by:
    - Cochlea length (~35 mm)
    - Current spread between electrodes
    - Neural survival patterns
    - Manufacturing constraints
    12-22 electrodes is the practical range.

  BUT:
    There is no single "standard" channel count. Different manufacturers
    use different numbers. 22 (Cochlear) is the most common worldwide.
    12 = sigma(6) matches MED-EL but is not universal.

  Grade: CLOSE
  12 channels is a real clinical value (MED-EL, historical systems)
  and sigma(6) = 12 is specific. But 22 channels (Cochlear) is more
  common globally, and channel count varies by manufacturer.
```

---

### H-MD-24: X-ray Tube kVp Settings -- 80 / 120 / 140

> Standard X-ray/CT tube voltages: 80, 100, 120, 140 kVp

```
  X-ray tube peak kilovoltage:
    Chest X-ray: 120 kVp (standard adult PA)
    Abdomen CT: 120 kVp (standard)
    Pediatric/low-dose: 80 kVp
    Large patient/bone: 140 kVp
    Mammography: 25-35 kVp (entirely different range)

  n=6 mapping:
    80 = phi^tau * sopfr = 16*5 = 80 (forced)
    120 = sigma * (sigma-phi) = 12*10 = 120
    120 = J₂ * sopfr = 24*5 = 120
    140 = σ² - τ = 144 - 4 = 140

  Physical basis:
    kVp settings are determined by X-ray tube physics (Bremsstrahlung
    spectrum) and tissue attenuation coefficients. The specific values
    are engineering conventions influenced by tissue contrast optimization.

  BUT:
    120 = σ·(σ-φ) is a compound expression.
    The specific kVp values are engineering choices, not constants.
    Different manufacturers offer different kVp options.

  Grade: WEAK
  The kVp values are engineering conventions. The n=6 mappings
  require compound expressions and are not uniquely determined.
```

---

## Category J: Neurological Monitoring

---

### H-MD-25: EEG 10-20 System = 21 Electrodes ~ J₂ - n/phi

> The international 10-20 EEG system uses 21 electrode positions

```
  EEG 10-20 system (Jasper, 1958):
    19 recording electrodes + 2 reference = 21 positions
    Named by anatomical location: Fp1/2, F3/4/7/8/z, C3/4/z,
    T3/4/5/6, P3/4/z, O1/2

  J₂ - n/phi = 24 - 3 = 21 checkmark

  Extended systems:
    10-10 system: 75 electrodes
    10-5 system: 345 electrodes
    High-density EEG: 128, 256 electrodes

  BUT:
    "21" is debatable: some count 19 recording + 2 reference = 21,
    others count only 19 recording electrodes.
    J₂ - n/phi = 21 requires subtracting two derived quantities.
    The 10-20 system is a clinical convention, not a physics constraint.

  Grade: WEAK
  The electrode count is convention-dependent (19 or 21 depending
  on counting method). J₂ - n/phi is a contrived expression.
  Extended systems use completely different numbers.
```

---

### H-MD-26: EEG Frequency Bands = sopfr = 5

> Clinical EEG recognizes 5 primary frequency bands

```
  EEG frequency bands:
    Delta: 0.5-4 Hz (deep sleep)
    Theta: 4-8 Hz (drowsiness)
    Alpha: 8-13 Hz (relaxed wakefulness)
    Beta: 13-30 Hz (active thinking)
    Gamma: 30-100+ Hz (high-level processing)

  sopfr(6) = 5 checkmark

  BUT:
    The 5-band classification is a convention. Some systems include
    additional bands (mu, sigma, high-gamma). The boundaries between
    bands are not sharp and vary between sources.
    sopfr(6) = 5 is also just "5" -- a common small integer.

  Grade: WEAK
  The 5-band system is a textbook convention, not a physical constant.
  Some EEG systems recognize 4, 6, or 7 bands.
```

---

## Category K: Clinical Scoring Systems

---

### H-MD-27: APGAR Score at 1 and 5 Minutes = mu and sopfr

> The APGAR score is assessed at 1 minute and 5 minutes after birth

```
  APGAR score (Virginia Apgar, 1952):
    5 criteria: Appearance, Pulse, Grimace, Activity, Respiration
    Each scored 0-2, total 0-10
    Assessed at 1 minute and 5 minutes after birth

  mu = 1 (1 minute), sopfr = 5 (5 minutes) checkmark
  5 criteria = sopfr(6) = 5 checkmark
  Max score per criterion = 2 = phi checkmark

  BUT:
    1 and 5 minutes are extremely common small integers.
    mu(6) = 1 matches trivially (mu is 1 for any squarefree number).
    The 5 criteria were chosen by Dr. Apgar as a mnemonic.
    Score 0-2 per criterion is the simplest ordinal scale.

  Grade: WEAK
  Multiple small-integer matches (1, 2, 5) but all trivially small.
  The APGAR system was designed by a clinician for simplicity,
  and the small numbers reflect practical clinical assessment.
```

---

### H-MD-28: APGAR Maximum Score = sigma - phi = 10

> The APGAR score maximum is 10

```
  APGAR: 5 criteria x 2 max each = 10 maximum

  sigma - phi = 12 - 2 = 10 checkmark

  BUT:
    10 is the most common maximum score in virtually all rating systems
    (satisfaction surveys, pain scales, wine ratings, etc.).
    10 = our decimal base. sigma - phi = 10 is true but 10 is
    perhaps the most over-used number in human scoring systems.

  Grade: WEAK
  10 is the default maximum for virtually any human-designed scale.
  sigma - phi = 10 is correct but meaningless as a discriminator.
```

---

## Category L: Vital Signs / Patient Assessment

---

### H-MD-29: Vital Signs Count = n = 6

> There are 6 vital signs: HR, BP, RR, SpO2, Temperature, Pain

```
  Traditional vital signs:
    4 classic: Heart Rate, Blood Pressure, Respiratory Rate, Temperature
    5th: SpO2 (added ~1990s, "fifth vital sign")
    6th: Pain (added ~2001, Joint Commission, "sixth vital sign" -- later
         controversial and partially rescinded due to opioid crisis)

  n = 6 partial match

  Current practice:
    Most institutions recognize 5 vital signs (HR, BP, RR, Temp, SpO2).
    "Pain as 6th vital sign" was introduced by the VA/Joint Commission
    but has been controversial and many institutions have moved away
    from calling pain a vital sign.
    Some add glucose, level of consciousness, or weight.

  Grade: WEAK
  "6 vital signs" is contested and depends on which institutional
  guidelines you follow. The classic count is 4, the modern standard
  is 5, and "6" is controversial. This is a convention, not a
  physiological constant.
```

---

### H-MD-30: Normal Body Temperature = 98.6 F = ?

> Normal body temperature is 37 C (98.6 F)

```
  Body temperature:
    37 C (98.6 F) -- Wunderlich, 1868
    Modern studies suggest mean is closer to 36.6 C (Protsiv et al., 2020)
    Normal range: 36.1-37.2 C

  n=6 attempts:
    37 = σ*n/φ + μ = 12*3 + 1 = 37 (forced)
    37 is a prime number, not cleanly expressible in n=6 terms
    36.6 C (modern mean) is closer to n*(n+μ/σ) -- extremely forced

  Grade: FAIL
  37 C is not cleanly expressible in n=6 arithmetic.
  The temperature is a physiological constant determined by
  metabolic rate and thermoregulation, not number theory.
```

---

## Category M: Imaging Standards / Regulatory

---

### H-MD-31: DICOM Image Depth = sigma - tau = 8 bits

> Standard medical imaging uses 8-bit (256 level) display

```
  DICOM display:
    Standard grayscale display: 8 bits (256 levels)
    Acquisition: 12-16 bits (4096-65536 levels)
    Window/level adjustment maps 12-16 bit to 8-bit display

  sigma - tau = 12 - 4 = 8 checkmark

  Physical basis:
    8-bit display is the standard because:
    - Human visual system can discriminate ~700-900 gray levels max
    - 256 levels (8-bit) exceeds the JND (just-noticeable difference)
      for most viewing conditions
    - 8-bit = 1 byte aligns with computer architecture

  BUT:
    8-bit is a universal digital standard, not specific to medical imaging.
    All consumer displays, cameras, etc. use 8-bit per channel.
    sigma - tau = 8 maps to the same constant as BT-58 (universal 8).
    Modern medical displays use 10-bit or 12-bit grayscale (GSDF).

  Grade: CLOSE
  8-bit display is genuinely standard in medical imaging (DICOM PS 3.14).
  sigma - tau = 8 is a well-established n=6 constant (BT-58).
  But 8-bit is universal in computing, not specific to medicine.
  Connects to BT-58 (sigma-tau=8 universal AI constant).
```

---

### H-MD-32: Medical Image Acquisition = sigma = 12 bits

> Medical imaging ADC resolution is typically 12 bits

```
  Medical image acquisition bit depth:
    X-ray/DR: 12-14 bits
    CT: 12-16 bits
    MRI: 12-16 bits
    Ultrasound: 8-12 bits
    Nuclear medicine: 8-10 bits

  sigma(6) = 12 checkmark

  Physical basis:
    12-bit ADCs provide 4096 gray levels, which exceeds the
    dynamic range needed for most medical applications.
    12-bit became standard because it balances precision,
    cost, and data volume.

  BUT:
    12-bit is common but not universal -- CT and MRI increasingly
    use 14-16 bit acquisition. The "12-bit standard" is an era-specific
    engineering choice, not a physical constant.
    12 is also the number of bits in many other ADC standards.

  Grade: CLOSE
  12-bit ADC is genuinely common in medical imaging equipment.
  sigma(6) = 12 is specific and matches. But it's an engineering
  choice that is being superseded by higher bit depths.
```

---

### H-MD-33: ISO 13485 Sections / FDA Device Classes = n/phi = 3

> FDA classifies medical devices into 3 classes (I, II, III)

```
  FDA device classification:
    Class I: Low risk (tongue depressors, bandages)
    Class II: Moderate risk (powered wheelchairs, pregnancy tests)
    Class III: High risk (pacemakers, heart valves)

  n/phi = 6/2 = 3 checkmark

  EU MDR also uses 4 classes: I, IIa, IIb, III
  Japan PMDA: 4 classes

  BUT:
    3-tier classification is the simplest hierarchical system.
    Low/Medium/High is the most basic risk stratification.
    FDA uses 3 classes, but EU uses 4. n/phi = 3 = any 3-tier system.
    This is not specific to medical devices.

  Grade: WEAK
  3 device classes is the simplest possible risk hierarchy.
  n/phi = 3 matches trivially. EU uses 4 classes, breaking the pattern.
```

---

### H-MD-34: Tc-99m Gamma Energy = 140.5 keV ~ sigma * (sigma - mu/phi)

> Tc-99m emits a 140.5 keV gamma ray

```
  Tc-99m decay:
    Gamma energy: 140.511 keV (NNDC)
    This energy is ideal for gamma cameras (NaI(Tl) scintillators)
    because it is:
    - High enough to penetrate tissue (>30 keV)
    - Low enough for efficient collimation and detection
    - Near the optimal sensitivity range of NaI(Tl)

  n=6 attempts:
    140 ~ σ² - τ = 144 - 4 = 140 (0.36% deviation)
    140.5 ~ σ² - τ + μ/φ = 140.5 (exact but contrived)

  σ² - τ = 140 is the clearest expression.

  Physical basis:
    The gamma energy is determined by nuclear level structure
    of the Tc-99 nucleus (isomeric transition from 142 keV level
    with internal conversion corrections).

  Grade: CLOSE
  140 keV is close to sigma^2 - tau = 140 (0.36% off from 140.511).
  The expression is relatively clean (only 2 functions).
  But nuclear energies span a wide range, and landing near
  any n=6 expression is expected with enough trial.
```

---

### H-MD-35: Medical Monitor Alarm Limits = Multiples of 6

> ICU monitor alarm defaults often use multiples of 6: HR 60-120,
> SpO2 < 90%, BP 90/60

```
  Common ICU alarm defaults:
    Heart rate: low 60 (= σ*sopfr), high 120 (= σ*(σ-φ))
    SpO2: low 90% (= σ*sopfr + σ*sopfr/2)... forced
    Systolic BP: low 90 mmHg
    Diastolic BP: low 60 mmHg

  n=6 mapping:
    60 = σ·sopfr = 12*5
    120 = σ·(σ-φ) = 12*10 (or sigma*J₂/sigma -- trivial)
    90 = σ·sopfr + σ·sopfr/2 (forced)

  BUT:
    60 and 120 are round numbers (factors of 60-base time system).
    90 mmHg and 60 mmHg are clinical conventions.
    Medical alarm limits are set by clinical practice guidelines
    and are not physical constants. They are adjustable per patient.

  Grade: WEAK
  Clinical alarm limits are convention-based and adjustable.
  60 and 120 are common numbers in time-keeping. The mappings
  to n=6 are forced for most values.
```

---

## Category N: Advanced Imaging / Therapy

---

### H-MD-36: Radiation Therapy Fractions = 5 per Week = sopfr

> Standard radiation therapy delivers 5 fractions per week (weekdays)

```
  Radiation therapy fractionation:
    Conventional: 5 fractions per week (Monday-Friday)
    Typical total: 25-35 fractions
    Daily dose: 1.8-2.0 Gy per fraction

  sopfr(6) = 5 checkmark

  BUT:
    5 fractions/week is simply the 5-day work week.
    Hypofractionation (3/week), hyperfractionation (2x/day),
    and weekend treatments exist. The 5-day schedule is a
    logistical convenience, not a radiobiological optimization.

  Grade: FAIL
  5 fractions/week = 5 working days. This is calendar convention,
  not radiobiology. sopfr = 5 matching the work week is trivial.
```

---

### H-MD-37: Gamma Knife Sources = J₂·(σ-φ) = 240 ~ 192

> The Leksell Gamma Knife uses 192 Co-60 sources (older models used 201)

```
  Gamma Knife models:
    Model B/C: 201 Co-60 sources
    Perfexion: 192 Co-60 sources
    ICON (current): 192 Co-60 sources

  192 = σ · φ^τ = 12 * 16 = 192 (EXACT arithmetic)
  Also: 192 = J₂ * (σ-τ) = 24 * 8 = 192

  Physical basis:
    The number of sources is determined by:
    - Geometric arrangement (sectors and rings)
    - Focal point convergence requirements
    - Manufacturing constraints
    Perfexion arranges 192 sources in 8 sectors of 24 = J₂

  Sector structure:
    192 = 8 sectors * 24 sources/sector = (sigma-tau) * J₂

  Grade: CLOSE
  192 = sigma * phi^tau is numerically exact and the factorization
  into 8 * 24 = (sigma-tau) * J₂ matches the physical arrangement.
  But 192 is also 3 * 64 = 3 * 2^6 and the older model used 201,
  showing the number is design-dependent, not a physics constant.
```

---

## Summary

| ID | Hypothesis | Claimed Match | Grade |
|----|-----------|---------------|-------|
| H-MD-01 | 12-lead ECG | sigma = 12 | **EXACT** |
| H-MD-02 | 6 limb leads | n = 6 | **EXACT** |
| H-MD-03 | 6 precordial leads | n = 6 | **CLOSE** |
| H-MD-04 | Tc-99m t_1/2 = 6 hours | n = 6 | **EXACT** |
| H-MD-05 | PET coincidence ~6 ns | n = 6 | **WEAK** |
| H-MD-06 | F-18 t_1/2 = 110 min | sigma*(sigma-phi) = 120 | **WEAK** |
| H-MD-07 | MRI 1.5T / 3T | n/tau, n/phi | **CLOSE** |
| H-MD-08 | MRI RF 8/16/32/64 ch | phi^{tau,sopfr,n} | **WEAK** |
| H-MD-09 | MRI 12-channel coil | sigma = 12 | **CLOSE** |
| H-MD-10 | CT 64/128/256 rows | 2^n / 2^7 / 2^8 | **CLOSE** |
| H-MD-11 | Ultrasound 6 MHz | n = 6 | **WEAK** |
| H-MD-12 | Ultrasound 4 modes | tau = 4 | **WEAK** |
| H-MD-13 | US transducer 128/192/256 | phi powers | **WEAK** |
| H-MD-14 | Surgical robot 6 DOF | n = 6 | **EXACT** |
| H-MD-15 | Pulse ox 2 wavelengths | phi = 2 | **WEAK** |
| H-MD-16 | Biphasic defibrillator | phi = 2 | **FAIL** |
| H-MD-17 | Pacemaker 60 bpm | sigma*sopfr = 60 | **CLOSE** |
| H-MD-18 | GCS 3-15 range | n/phi to sigma+n/phi | **CLOSE** |
| H-MD-19 | Blood pH range 0.10 | 1/(sigma-phi) = 0.1 | **CLOSE** |
| H-MD-20 | Autoclave 121 C | (sigma-mu)^2 = 121 | **WEAK** |
| H-MD-21 | Autoclave 15 min | sigma+n/phi = 15 | **WEAK** |
| H-MD-22 | Tidal volume 6 mL/kg | n = 6 | **EXACT** |
| H-MD-23 | Cochlear implant 12 ch | sigma = 12 | **CLOSE** |
| H-MD-24 | X-ray 80/120/140 kVp | n=6 expressions | **WEAK** |
| H-MD-25 | EEG 10-20 = 21 electrodes | J₂-n/phi = 21 | **WEAK** |
| H-MD-26 | EEG 5 freq bands | sopfr = 5 | **WEAK** |
| H-MD-27 | APGAR at 1 and 5 min | mu, sopfr | **WEAK** |
| H-MD-28 | APGAR max 10 | sigma-phi = 10 | **WEAK** |
| H-MD-29 | 6 vital signs | n = 6 | **WEAK** |
| H-MD-30 | Body temp 37 C | -- | **FAIL** |
| H-MD-31 | DICOM 8-bit display | sigma-tau = 8 | **CLOSE** |
| H-MD-32 | Medical ADC 12-bit | sigma = 12 | **CLOSE** |
| H-MD-33 | FDA 3 device classes | n/phi = 3 | **WEAK** |
| H-MD-34 | Tc-99m 140.5 keV | sigma^2-tau = 140 | **CLOSE** |
| H-MD-35 | ICU alarm defaults | multiples of 6 | **WEAK** |
| H-MD-36 | RT 5 fractions/week | sopfr = 5 | **FAIL** |
| H-MD-37 | Gamma Knife 192 sources | sigma*phi^tau = 192 | **CLOSE** |

## Grade Distribution

| Grade | Count | Pct | Hypotheses |
|-------|-------|-----|------------|
| EXACT | 5 | 13.5% | H-MD-01, H-MD-02, H-MD-04, H-MD-14, H-MD-22 |
| CLOSE | 12 | 32.4% | H-MD-03, H-MD-07, H-MD-09, H-MD-10, H-MD-17, H-MD-18, H-MD-19, H-MD-23, H-MD-31, H-MD-32, H-MD-34, H-MD-37 |
| WEAK | 17 | 45.9% | H-MD-05, H-MD-06, H-MD-08, H-MD-11, H-MD-12, H-MD-13, H-MD-15, H-MD-20, H-MD-21, H-MD-24, H-MD-25, H-MD-26, H-MD-27, H-MD-28, H-MD-29, H-MD-33, H-MD-35 |
| FAIL | 3 | 8.1% | H-MD-16, H-MD-30, H-MD-36 |

**Non-failing total: 34/37 (91.9%)**
**Honestly strong (EXACT+CLOSE): 17/37 (45.9%)**

Note: The high non-failing rate is inflated by medical devices using many small integers
(2, 3, 4, 5, 6, 8, 12) that trivially match n=6 functions. The 5 EXACT matches
(12-lead ECG, 6 limb leads, Tc-99m 6h, 6-DOF robot, 6 mL/kg tidal volume) are
genuinely strong because they involve non-trivial constants fixed by physics,
anatomy, or clinical evidence rather than convention.
