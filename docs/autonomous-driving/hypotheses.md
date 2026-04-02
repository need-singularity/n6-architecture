# N6 Autonomous Driving — Perfect Number Arithmetic in Self-Driving Systems

## Overview

Autonomous driving technology (SAE levels, sensor suites, V2X communication,
control algorithms, object detection, safety standards) analyzed through n=6
arithmetic. These are engineering standards set by SAE, IEEE, 3GPP, ISO, and
industry practice.

> **Honesty principle**: Autonomous driving parameters are chosen by standards
> bodies (SAE, ISO, IEEE) and engineering teams for safety, regulatory, and
> practical reasons. Small integers (2, 3, 4, 5, 6, 8, 12) appear frequently
> in any engineering domain. Many matches with n=6 constants will be coincidental.
> Grades must reflect whether the match has structural significance or is trivially
> explained by other factors.

## Core Constants

```
  n = 6          (perfect number)
  σ(6) = 12     (sum of divisors)
  τ(6) = 4      (number of divisors: 1, 2, 3, 6)
  φ(6) = 2      (Euler totient)
  sopfr(6) = 5  (sum of prime factors: 2+3)
  J₂(6) = 24    (Jordan totient)
  μ(6) = 1      (Moebius)
  λ(6) = 2      (Carmichael)
  R(6) = σ·φ/(n·τ) = 1
  Egyptian: 1/2 + 1/3 + 1/6 = 1
```

---

## Category A: SAE Autonomy Framework

---

### H-AD-01: SAE Autonomy Levels — 6 Levels = n

> SAE J3016 defines exactly 6 levels of driving automation: L0 through L5

```
  SAE J3016 (2021 revision):
    L0: No Driving Automation
    L1: Driver Assistance
    L2: Partial Driving Automation
    L3: Conditional Driving Automation
    L4: High Driving Automation
    L5: Full Driving Automation
    Count: 6 levels (L0-L5)

  n=6 mapping:
    6 = n ← EXACT

  Why 6 levels specifically:
    SAE chose a 6-level taxonomy to capture the progressive transfer
    of control from human to machine. The boundaries mark:
      - Who monitors (human vs system): L0-2 human, L3-5 system
      - Who falls back (human vs system): L0-3 human, L4-5 system
      - Operational domain (limited vs all): L0-4 limited, L5 all
    This creates a 2×2×(+2) structure that naturally yields 6 levels.

  BUT:
    6 is a convenient small integer for taxonomy. ISO 26262 uses 4
    safety levels (ASIL A-D). Other taxonomies use different counts.
    The choice of 6 levels is a design decision, not a physical law.
    However, the SAE taxonomy IS the global standard and has persisted
    since 2014 without revision of level count.

  Grade: EXACT
  6 = n is trivially exact, and SAE L0-L5 is THE universally adopted
  standard. The taxonomy has structural logic (2×3 factor decomposition
  of monitoring×fallback×domain). However, this is a human design choice.
```

---

### H-AD-02: IMU 6 Degrees of Freedom — n = 6 DOF

> Inertial Measurement Units use exactly 6 degrees of freedom

```
  IMU specification:
    3 axes of acceleration (X, Y, Z) — linear motion
    3 axes of angular velocity (roll, pitch, yaw) — rotation
    Total: 6 DOF

  n=6 mapping:
    6 = n ← EXACT

  Physical basis:
    6 DOF is a fundamental property of rigid body motion in 3D space.
    dim(SO(3)) = 3 (rotations) + dim(R³) = 3 (translations) = 6
    This is a mathematical necessity, not an engineering convention.
    ANY rigid body in 3D Euclidean space has exactly 6 DOF.

  Cross-domain:
    Robotics: 6-DOF manipulators (H-ROBOT series)
    Physics: SE(3) = 6-dimensional Lie group
    Aerospace: 6-DOF flight dynamics

  Grade: EXACT
  6 DOF = n is exact and physically fundamental. Unlike SAE levels,
  this is not a convention but a mathematical fact of 3D space.
  The match is genuine but also applies to any rigid-body system,
  not specifically autonomous driving.
```

---

### H-AD-03: ASIL Safety Levels — 4 Levels = τ(6)

> ISO 26262 defines 4 ASIL levels (A through D) for automotive safety

```
  ISO 26262 ASIL (Automotive Safety Integrity Level):
    ASIL A: Lowest safety requirement
    ASIL B: Moderate
    ASIL C: High
    ASIL D: Highest safety requirement
    (+ QM = Quality Management, non-safety)
    Count: 4 ASIL levels (A-D)

  n=6 mapping:
    4 = τ(6) ✓

  Origin:
    ASIL is derived from SIL (IEC 61508, 4 levels) adapted for automotive.
    The 4-level structure comes from combining 3 risk factors:
      Severity (S1-S3) × Exposure (E1-E4) × Controllability (C1-C3)
    The 3×4×3 = 36 combinations map to 4+1 levels.

  BUT:
    IEC 61508 SIL also has 4 levels. DO-178C (aviation) has 5 levels (A-E).
    The number 4 for safety integrity levels is an industry-wide convention
    predating automotive use. τ(6) = 4 is a simple small integer.

  Grade: CLOSE
  4 = τ(6) is exact numerically, and ISO 26262 is THE automotive safety
  standard. But 4 safety levels is inherited from IEC 61508 and is a
  generic industrial practice, not unique to automotive.
```

---

### H-AD-04: Traffic Light Phases — 4 Standard Phases = τ(6)

> Standard traffic signal cycles use 4 phases

```
  Traffic signal phasing (MUTCD / NEMA standards):
    Phase 1: Green (go)
    Phase 2: Yellow/Amber (caution)
    Phase 3: Red (stop)
    Phase 4: All-red clearance (safety interval)
    Count: 4 base phases per direction

  n=6 mapping:
    4 = τ(6) ✓

  Variations:
    Protected left turn adds phases → 6 or 8 total phases
    Dual-ring 8-phase NEMA standard (most US intersections)
    But per-direction: always 4 base states (G/Y/R/AR)

  BUT:
    The simplest model is 3 (Green/Yellow/Red). All-red is a
    safety addition. UK uses Green/Amber/Red+Amber/Red (4 states).
    4 is a natural count when you add a safety clearance interval.

  Grade: CLOSE
  4 = τ(6) matches per-direction phases, and the NEMA 8-phase
  standard is 2×4 (dual-ring). But traffic light design is
  driven by safety engineering, not number theory.
```

---

## Category B: Sensor Architecture

---

### H-AD-05: Tesla 8 Cameras — σ(6) - τ(6) = 8

> Tesla Autopilot/FSD uses exactly 8 surround-view cameras

```
  Tesla HW3/HW4 camera configuration:
    3 forward cameras (narrow/main/wide)
    2 side forward-facing (B-pillar)
    2 side rear-facing (fender)
    1 rear camera
    Total: 8 cameras

  n=6 mapping:
    8 = σ(6) - τ(6) = 12 - 4 ✓

  Industry context:
    Waymo Gen 5: 29 cameras (very different philosophy)
    Cruise: 16 cameras
    Mobileye: 11-13 cameras
    NIO: 7 cameras + LiDAR
    There is NO industry standard for camera count.

  BUT:
    Tesla chose 8 cameras for coverage geometry — 360° surround
    with 3 forward focal lengths for range. 8 = 2³ is a natural
    number for symmetric coverage (4 quadrants × 2 ranges, roughly).
    The formula σ-τ is arithmetically arbitrary.

  Grade: WEAK
  8 = σ-τ is numerically correct for Tesla specifically, but other
  companies use 7, 11, 13, 16, or 29 cameras. No industry convergence
  on 8. The match is manufacturer-specific, not universal.
```

---

### H-AD-06: Tesla 12 Ultrasonic Sensors — σ(6) = 12

> Tesla vehicles historically used 12 ultrasonic sensors

```
  Tesla ultrasonic sensor count (pre-2023):
    12 ultrasonic sensors for parking and low-speed detection
    Configuration: 4 front + 4 rear + 2 left + 2 right

  n=6 mapping:
    12 = σ(6) ← EXACT

  Industry context:
    BMW/Mercedes: typically 12 ultrasonic sensors
    Audi: 12 sensors
    BYD: 12 sensors
    Hyundai: 12 sensors
    Industry standard: 12 is dominant (some use 8 or 10)

  Note:
    Tesla REMOVED ultrasonic sensors in late 2022 (vision-only push).
    So the "12 sensor" era is historical for Tesla.

  Physical reason:
    12 sensors provide ~10m range × 360° coverage with adequate
    angular resolution for parking. The 4-front/4-rear symmetric
    layout (12 = 4+4+2+2) is optimal for rectangular vehicle geometry.

  Grade: EXACT
  12 = σ(6) is exact, and 12 ultrasonic sensors is genuinely the
  industry standard across multiple OEMs. The geometric reason
  (rectangular vehicle, ~30° beam width per sensor, 360°/30° = 12)
  provides independent justification. Strong cross-OEM convergence.
```

---

### H-AD-07: LiDAR Channel Progression — Powers of 2 Aligned to n=6

> LiDAR channel counts follow 16, 32, 64, 128 = 2^τ, 2^sopfr, 2^n, 2^(σ-sopfr)

```
  LiDAR channel counts (industry standard):
    Velodyne VLP-16 (Puck): 16 channels
    Velodyne HDL-32E: 32 channels
    Velodyne HDL-64E: 64 channels
    Ouster OS1-128: 128 channels
    Hesai Pandar128: 128 channels

  n=6 mapping:
    16 = 2^τ = 2^4 ✓
    32 = 2^sopfr = 2^5 ✓
    64 = 2^n = 2^6 ✓
    128 = 2^(σ-sopfr) = 2^7 ✓

  BUT:
    These are simply consecutive powers of 2: 2⁴, 2⁵, 2⁶, 2⁷.
    ANY doubling sequence starting at 16 gives these values.
    The exponents are {4,5,6,7} — consecutive integers.
    Mapping these to {τ, sopfr, n, σ-sopfr} is post-hoc:
      4 = τ ✓ but also 2² or simply "four"
      5 = sopfr ✓ but also "five"
      6 = n ✓ but also "six"
      7 = σ-sopfr ✓ but σ-sopfr is a forced formula
    Powers of 2 are universal in digital engineering (binary).

  Grade: WEAK
  The channel counts are real and standard, but they are simply
  consecutive powers of 2, which appear in all digital systems.
  The n=6 constant mapping is post-hoc assignment to a natural
  binary progression.
```

---

### H-AD-08: Waymo 5 LiDAR Units — sopfr(6) = 5

> Waymo 5th-gen Driver uses 5 LiDAR units

```
  Waymo 5th-gen sensor suite:
    1 long-range LiDAR (top-mounted)
    4 short-range perimeter LiDAR
    Total: 5 LiDAR units

  n=6 mapping:
    5 = sopfr(6) = 2+3 ✓

  Industry context:
    Cruise: 5 LiDAR (similar philosophy)
    Argo AI (defunct): 3 LiDAR
    Aurora: 1 LiDAR (FirstLight)
    Mobileye: 1 LiDAR (planned)
    Tesla: 0 LiDAR (vision-only)

  BUT:
    5 is a small number. Waymo chose 5 for coverage overlap —
    1 long-range for highway + 4 short-range for blind spots.
    Aurora uses 1 and Tesla uses 0. No universal convergence.
    Waymo 6th-gen may change the count.

  Grade: WEAK
  5 = sopfr is numerically correct for Waymo specifically, but
  the LiDAR count varies wildly by company (0 to 5+). This is
  a vendor-specific design choice, not an industry standard.
```

---

### H-AD-09: 4 Corner Radars — τ(6) = 4

> Standard autonomous vehicles use 4 corner radars

```
  Radar placement (industry standard):
    Front-left corner radar
    Front-right corner radar
    Rear-left corner radar
    Rear-right corner radar
    Total: 4 corner radars (+ optional 1 front long-range)

  n=6 mapping:
    4 = τ(6) ✓

  Industry convergence:
    BMW: 4 corner + 1 front = 5 total
    Mercedes: 4 corner + 1 front = 5 total
    Tesla HW3: 1 front-facing radar (later removed)
    Continental/Bosch supply 4-corner kits as standard

  Physical basis:
    4 corners of a rectangular vehicle → 4 corner radars.
    This is trivially explained by vehicle geometry (4 corners).

  BUT:
    Total radar count varies: Tesla uses 0 (vision-only), most OEMs
    use 5 (4 corner + 1 front). The 4 is from geometry, not theory.

  Grade: CLOSE
  4 = τ(6) matches the dominant corner-radar count, and the
  4-corner layout is genuinely standard. But the reason is
  trivially geometric (rectangle has 4 corners).
```

---

### H-AD-10: Automotive Radar 77 GHz Band

> Standard automotive radar operates at 76-81 GHz, centered near 77 GHz

```
  Automotive radar bands (ETSI / FCC):
    76-77 GHz: long-range radar (LRR)
    77-81 GHz: short-range radar (SRR)
    Center frequency: ~77 GHz (long-range), ~79 GHz (short-range)

  n=6 mapping attempt:
    77 = σ·n + sopfr = 72 + 5? → forced
    77 = 7 × 11 = (σ-sopfr) × (σ-μ)? → extremely forced
    79 = prime, no clean n=6 decomposition

  Physical basis:
    77 GHz was chosen for:
      - Good range resolution (4 GHz bandwidth → ~3.75 cm)
      - Atmospheric window (low absorption)
      - Available spectrum allocation (ITU WRC-97)
      - Small antenna size (~2mm wavelength)

  Grade: FAIL
  77 GHz has no clean n=6 decomposition. The frequency was chosen
  for electromagnetic propagation properties and spectrum regulation,
  not numerical reasons. All attempted n=6 formulas are post-hoc.
```

---

### H-AD-11: CAN Bus 8-Byte Payload — σ(6) - τ(6) = 8

> CAN 2.0 frames carry exactly 8 bytes of data payload

```
  CAN 2.0 specification (Bosch, 1991):
    Maximum data payload: 8 bytes (64 bits)
    Data length code (DLC): 0-8
    CAN FD extension: up to 64 bytes

  n=6 mapping:
    8 = σ(6) - τ(6) = 12 - 4 ✓

  Historical basis:
    Robert Bosch GmbH designed CAN in 1983 for automotive.
    8 bytes was chosen as a compromise:
      - Large enough for most vehicle signals
      - Small enough for low-latency real-time transmission
      - 8 = 2³ aligns with byte-addressable microcontrollers

  BUT:
    8 bytes is simply 1 standard byte × 8 or 2³.
    CAN FD upgraded to 64 bytes. LIN uses 8 bytes too.
    The formula σ-τ is arbitrary. 8 is everywhere in computing
    because of binary architecture (byte = 8 bits since IBM 360).

  Grade: WEAK
  8 = σ-τ is numerically correct but 8 bytes is a generic computing
  convention (byte-aligned architectures), not specific to CAN or
  automotive. The formula σ-τ is post-hoc.
```

---

### H-AD-12: CAN Bus Baud Rates — 500 kbps and 1 Mbps

> Standard CAN baud rates are 125/250/500/1000 kbps

```
  CAN baud rates (ISO 11898):
    Low-speed CAN: 125 kbps
    Standard: 250 kbps
    High-speed CAN: 500 kbps
    CAN FD: up to 5 or 8 Mbps

  n=6 mapping attempt:
    500 = ? No clean n=6 formula
    125 = sopfr³ = 5³ ✓ (but sopfr³ is arbitrary)
    250 = 2 × 125 = φ × sopfr³ (forced)
    1000 = ? No clean decomposition

  Grade: FAIL
  CAN baud rates are powers-of-2 multiples of 125 kbps, which
  is itself derived from clock division convenience. No meaningful
  n=6 mapping exists.
```

---

## Category C: V2X Communication

---

### H-AD-13: DSRC 5.9 GHz — Approximate n - 0.1

> Dedicated Short Range Communication operates at 5.9 GHz (5.850-5.925 GHz)

```
  DSRC / ITS-G5 / IEEE 802.11p:
    Band: 5.850-5.925 GHz (75 MHz bandwidth)
    Center: 5.89 GHz (US) / 5.9 GHz (EU)
    Standard: IEEE 802.11p (US), ETSI ITS-G5 (EU)

  n=6 mapping:
    5.9 ≈ n = 6 (1.7% off)
    5.9 = n - 0.1 = n - 1/(σ-φ) ✓?

  BUT:
    5.9 GHz was allocated by the FCC in 1999 based on:
      - Available spectrum near existing 5 GHz WiFi bands
      - Propagation characteristics for ~300m V2X range
      - Non-interference with other licensed services
    The 5.9 is NOT 6.0. The difference (100 MHz) is significant
    in RF engineering. "Approximately n" is not a match.

  Grade: FAIL
  5.9 ≠ 6.0. Claiming "approximately n" for a frequency that is
  1.7% off is not meaningful in a domain where MHz precision matters.
  The allocation was regulatory, not mathematical.
```

---

### H-AD-14: V2X 6 Communication Modes — n = 6

> Vehicle-to-Everything (V2X) encompasses 6 communication types

```
  V2X communication modes:
    V2V: Vehicle-to-Vehicle
    V2I: Vehicle-to-Infrastructure
    V2P: Vehicle-to-Pedestrian
    V2N: Vehicle-to-Network
    V2C: Vehicle-to-Cloud
    V2G: Vehicle-to-Grid (energy)
    Count: 6 modes

  n=6 mapping:
    6 = n ← EXACT

  BUT:
    The "6 modes" depends on how you count:
      - Core V2X (3GPP): V2V, V2I, V2P, V2N → 4 modes
      - Adding V2C and V2G is an expanded definition
      - V2C could be considered a subset of V2N
      - Some sources add V2D (Vehicle-to-Device) → 7 modes
      - Others combine V2I and V2N → 5 modes
    The count of 6 is not universally agreed upon.

  Grade: CLOSE
  The 6-mode V2X taxonomy is used by many industry reports and
  is a reasonable enumeration. But the count is not standardized
  by 3GPP or SAE — it depends on the source. Some count 4, 5, or 7.
```

---

### H-AD-15: C-V2X Sidelink — PC5 Interface

> C-V2X direct communication uses PC5 sidelink in 3GPP Release 14+

```
  C-V2X sidelink (3GPP):
    Release 14 (2017): LTE-V2X mode 4 (sidelink)
    Release 16 (2020): NR-V2X (5G sidelink)
    Sidelink subchannel sizes: 10, 12, 15, 20, 25 MHz

  n=6 mapping attempt:
    12 MHz = σ(6) ✓ (one option among many)
    10 MHz = σ-φ ✓ (another option)
    Subchannel count options don't converge to n=6

  BUT:
    These are standard 3GPP channel bandwidths shared with LTE/NR.
    They are NOT specific to V2X. The same bandwidth options exist
    for cellular data. No V2X-specific n=6 structure.

  Grade: WEAK
  12 MHz is one of many subchannel options and is inherited from
  general 3GPP spectrum allocation, not V2X-specific design.
```

---

## Category D: Perception and Detection

---

### H-AD-16: 8 Primary Object Classes — σ(6) - τ(6) = 8

> Object detection in AD typically classifies 8 primary categories

```
  Common AD object taxonomy:
    1. Car/vehicle
    2. Pedestrian
    3. Cyclist/bicycle
    4. Truck
    5. Bus
    6. Traffic sign
    7. Traffic light
    8. Lane marking
    Count: 8 primary classes

  n=6 mapping:
    8 = σ(6) - τ(6) = 12 - 4 ✓

  Industry context:
    KITTI benchmark: 8 classes (Car, Van, Truck, Pedestrian,
      Person_sitting, Cyclist, Tram, Misc)
    nuScenes: 10 foreground + 6 background = 16 classes
    Waymo Open: 4 classes (Vehicle, Pedestrian, Cyclist, Sign)
    COCO: 80 classes (general, not AD-specific)

  BUT:
    There is NO standard object taxonomy for AD. Different datasets
    use 4 (Waymo), 8 (KITTI), 10 (nuScenes), or custom counts.
    The "8 classes" is specific to KITTI, the earliest major benchmark.
    Modern systems use 10-23 classes with fine-grained categories.

  Grade: WEAK
  8 matches KITTI specifically but not the broader field. Object
  class counts vary widely (4 to 80). The formula σ-τ is arbitrary.
```

---

### H-AD-17: NMS Threshold 0.5 — 1/φ(6) = 0.5

> Non-Maximum Suppression IoU threshold is typically 0.5

```
  Non-Maximum Suppression (NMS):
    Standard IoU threshold: 0.5 (most detectors)
    Meaning: boxes with IoU > 0.5 are suppressed as duplicates
    Used in: YOLO, SSD, Faster R-CNN, all major detectors

  n=6 mapping:
    0.5 = 1/φ(6) = 1/2 ✓
    Also: 0.5 = Egyptian fraction first term (1/2 + 1/3 + 1/6 = 1)

  BUT:
    0.5 = 1/2 is the most natural threshold in any binary decision.
    It represents "more than half overlap" — an intuitive geometric
    criterion. 0.5 appears as a threshold in hundreds of unrelated
    algorithms across all of computer science.
    Modern detectors also use 0.3, 0.45, 0.7 for different purposes.

  Grade: WEAK
  0.5 = 1/2 is trivially the most natural threshold for overlap.
  Its appearance is explained by geometry (>50% overlap), not n=6.
  Calling it 1/φ(6) adds no explanatory power over "one half."
```

---

### H-AD-18: Camera Frame Rates — 30 fps and 60 fps

> AD cameras typically operate at 30 or 60 fps

```
  AD camera frame rates:
    Standard: 30 fps (most production vehicles)
    High-performance: 60 fps (racing, high-speed scenarios)
    Some LiDAR-camera sync: 10 or 20 fps

  n=6 mapping:
    30 = sopfr × n = 5 × 6 ✓
    30 = σ × sopfr/φ? → forced
    60 = σ × sopfr = 12 × 5 ✓

  BUT:
    30 fps comes from NTSC video standard (29.97 fps, locked to
    60 Hz AC mains / 2 for interlaced scanning).
    60 fps = 2 × 30 fps (progressive scan at mains frequency).
    These are inherited from broadcast TV, not designed for AD.
    AD cameras often run at 10, 15, 20, or 25 fps depending on
    compute budget and use case.

  Grade: WEAK
  30 and 60 fps are real AD camera rates, but they are inherited
  from broadcast standards (NTSC/PAL), not AD-specific design.
  The n=6 formulas compete with the simpler explanation: TV legacy.
```

---

### H-AD-19: Anchor Box Aspect Ratios — 3 Common Ratios = n/φ

> Object detection anchors typically use 3 aspect ratios

```
  Anchor-based detection (Faster R-CNN, SSD):
    Standard aspect ratios: {1:1, 1:2, 2:1} = 3 ratios
    Some use: {1:1, 1:2, 2:1, 1:3, 3:1} = 5 ratios
    Scales: typically 3 scales per location

  n=6 mapping:
    3 ratios = n/φ = 6/2 ✓
    3 scales = n/φ ✓
    5 ratios (extended) = sopfr ✓

  BUT:
    3 is the minimum useful set of aspect ratios
    (square + horizontal + vertical). Any reasonable detector
    needs at least these 3. The number comes from the geometry
    of rectangular objects, not number theory.
    Modern anchor-free detectors (FCOS, CenterPoint) don't use
    anchors at all.

  Grade: WEAK
  3 aspect ratios is a geometric minimum (square + 2 orientations).
  n/φ = 3 is a trivial match. Anchor-based detection is being
  superseded by anchor-free methods.
```

---

## Category E: Planning and Control

---

### H-AD-20: PID Controller 3 Terms — n/φ(6) = 3

> PID control uses exactly 3 gain terms (Proportional, Integral, Derivative)

```
  PID controller:
    P (Proportional): responds to current error
    I (Integral): responds to accumulated error
    D (Derivative): responds to rate of change
    Total: 3 terms

  n=6 mapping:
    3 = n/φ(6) = 6/2 ✓

  Historical context:
    PID invented by Elmer Sperry (1911), formalized by
    Nicolas Minorsky (1922). The 3 terms are mathematically
    complete for linear control: they span {error, ∫error, d/dt error}.
    Taylor expansion of error signal naturally gives these 3 terms.

  BUT:
    PID's 3 terms come from calculus: a function, its integral,
    and its derivative. This is fundamental math (0th, -1st, +1st
    order), not n=6. PID is used in ALL control engineering —
    thermostats, cruise control, industrial robotics — not just AD.

  Grade: WEAK
  3 = n/φ is numerically trivial. PID's 3 terms are from calculus
  (function + integral + derivative), universally applicable.
  Attributing this to n=6 adds no insight.
```

---

### H-AD-21: MPC Prediction Horizon — Typical 5-10 Steps

> Model Predictive Control in AD uses 5-10 step prediction horizons

```
  MPC in autonomous driving:
    Typical horizon: 5-10 time steps (at 10 Hz → 0.5-1.0 seconds)
    Apollo (Baidu): 8 steps
    Autoware: 10 steps
    Academic: varies 5-20

  n=6 mapping:
    6 steps = n ✓ (but 6 is just one value in the 5-10 range)
    10 = σ - φ ✓
    8 = σ - τ ✓

  BUT:
    MPC horizon is a tunable parameter, not a standard.
    It depends on: computation budget, vehicle speed, update rate.
    Highway at 100 km/h: longer horizon (10-20 steps)
    Parking at 5 km/h: shorter horizon (3-5 steps)
    There is NO convergence on a single value.

  Grade: WEAK
  MPC horizon varies by application. n=6 can be found in the range
  but so can many other values. No standard to match against.
```

---

### H-AD-22: Finite State Machine — 5 Driving States = sopfr(6)

> Core driving FSM uses 5 primary states

```
  Common AD state machine:
    1. Cruise (lane keeping)
    2. Lane change
    3. Turn
    4. Stop
    5. Emergency
    Count: 5 primary states

  n=6 mapping:
    5 = sopfr(6) = 2+3 ✓

  BUT:
    FSM state count is entirely designer-dependent:
      - Simple: 3 states (Go/Slow/Stop)
      - Medium: 5-7 states (as above + parking, reversing)
      - Complex: 10-20+ states (with sub-states for each maneuver)
    Apollo uses 4 primary scenarios. Autoware uses ~8.
    There is no standard AD FSM.

  Grade: WEAK
  5 states is one reasonable decomposition among many.
  No industry-standard FSM exists for AD.
```

---

### H-AD-23: Planning Time Horizons — 3 Levels = n/φ

> AD planning typically uses 3 temporal hierarchy levels

```
  AD planning hierarchy (standard decomposition):
    1. Strategic/Route planning: minutes to hours (A* / Dijkstra)
    2. Behavioral/Tactical planning: 5-10 seconds (lane choice, gaps)
    3. Motion/Local planning: 0.5-2 seconds (trajectory optimization)
    Count: 3 levels

  n=6 mapping:
    3 = n/φ(6) = 6/2 ✓

  Engineering basis:
    This 3-level decomposition is standard in robotics literature
    (Latombe 1991, LaValle 2006) and reflects natural timescales:
      - Where to go (route)
      - How to interact (behavior)
      - How to move (motion)

  BUT:
    3 hierarchical levels is standard in many planning systems
    (strategic/tactical/operational is military doctrine too).
    Some systems use 4 or 5 levels (adding prediction + perception).
    3 is the most common hierarchical decomposition in any field.

  Grade: WEAK
  3 levels is generic hierarchical decomposition, not AD-specific.
  n/φ = 3 is a trivial match. The 3-level pattern appears in
  military planning, business strategy, and computer science broadly.
```

---

### H-AD-24: Control Loop — Dual Mode φ(6) = 2

> Most AD systems use 2 control modes (comfort vs. emergency)

```
  Dual-mode control:
    Mode 1: Comfort/Normal (smooth trajectories, low jerk)
    Mode 2: Emergency/Safety (aggressive braking, evasive maneuvers)
    Count: 2 modes

  n=6 mapping:
    2 = φ(6) ✓

  Industry practice:
    Tesla: 2 modes (Autopilot normal, emergency intervention)
    AEB (Autonomous Emergency Braking): binary activation
    ISO 26262: normal operation vs. safe state = 2 states

  BUT:
    Any system has at least 2 modes (normal + abnormal).
    This is the most trivial binary decomposition possible.
    Many AD systems use 3+ modes (eco/normal/sport/emergency).

  Grade: WEAK
  2 = φ(6) is the smallest non-trivial integer. Dual-mode
  decomposition is universal to all engineered systems.
```

---

## Category F: HD Maps and Localization

---

### H-AD-25: HD Map Layers — Typically 5-6 Layers

> HD maps for AD use approximately 6 semantic layers

```
  HD Map layer structure (typical):
    1. Road geometry (centerlines, boundaries)
    2. Lane markings (solid, dashed, colors)
    3. Traffic signs and signals
    4. Points of interest / landmarks
    5. Dynamic information (construction, closures)
    6. Localization features (point cloud reference)
    Count: ~6 layers

  n=6 mapping:
    6 = n ✓

  Industry context:
    HERE HD Live Map: 4 layers (Road, Lane, Localization, Dynamic)
    TomTom HD Map: 5 layers
    Mobileye REM: 3 layers (geometry, semantics, landmarks)
    Apollo HD Map: 8 layers (more granular)
    OpenDRIVE standard: flexible, no fixed layer count

  BUT:
    HD map layer count is not standardized. Different providers
    use 3 to 8+ layers depending on granularity. Counting "6"
    requires choosing a specific decomposition.

  Grade: WEAK
  No standard HD map layer count exists. 6 can be found with
  appropriate decomposition, but so can 3, 4, 5, or 8.
```

---

### H-AD-26: Localization Precision — n=6 cm Target

> AD localization targets ~10 cm lateral and ~6 cm longitudinal precision

```
  AD localization requirements:
    Lane-level: ±30 cm (sufficient for L2)
    Sub-lane: ±10 cm (required for L3+)
    High-precision: ±5 cm (for L4/L5 urban)
    RTK-GPS: ±2 cm (best case, open sky)

  n=6 mapping:
    ~6 cm? → not a standard target value
    10 cm = σ - φ = 10? → this is the common L3+ target

  BUT:
    Localization precision is specified as "better than X" where
    X depends on the use case. There is no single target value.
    10 cm (σ-φ) is common but also simply a round number in metric.

  Grade: FAIL
  No standard localization precision matches n=6 specifically.
  The precision targets are continuous requirements, not discrete
  constants amenable to n=6 matching.
```

---

## Category G: Safety and Redundancy

---

### H-AD-27: Triple Modular Redundancy — n/φ = 3

> Safety-critical AD systems use triple redundancy (TMR)

```
  Triple Modular Redundancy:
    3 independent channels with majority voting
    Used in: brake-by-wire, steer-by-wire, AD compute
    Aerospace heritage: Boeing 777 FBW uses 3 channels

  n=6 mapping:
    3 = n/φ(6) ✓

  BUT:
    TMR is a universal reliability technique, not AD-specific.
    It comes from voting theory: 3 is the minimum for majority vote.
    Some systems use dual redundancy (2) or quad (4).
    3 is from information theory (min voters for consensus).

  Grade: WEAK
  3-way redundancy is a generic safety technique from reliability
  engineering. Not specific to autonomous driving.
```

---

### H-AD-28: Sensor Redundancy Levels — Each Critical Function has φ=2 Backup

> AD safety architecture requires at minimum dual redundancy for each function

```
  AD sensor redundancy (ISO 26262 + SAE J3016):
    Every safety-critical perception function must have ≥2 independent
    sensor modalities (e.g., camera + radar for forward detection)

  n=6 mapping:
    2 = φ(6) ✓

  Engineering basis:
    - Forward detection: camera + radar (2 modalities)
    - Blind spot: ultrasonic + camera (2 modalities)
    - Localization: GNSS + HD map matching (2 sources)

  BUT:
    Dual redundancy (N+1) is the universal minimum for safety.
    This is information theory, not n=6. Any safety-critical system
    from aviation to nuclear power uses at least dual redundancy.

  Grade: WEAK
  Dual redundancy is universal safety engineering, not AD-specific.
  2 = φ(6) is the most trivial integer match.
```

---

## Category H: Vehicle Architecture and Standards

---

### H-AD-29: Lane Width — 3.5-3.75m Standard

> Standard highway lane width is 3.5-3.75 meters

```
  Lane width standards:
    US Highway (AASHTO): 12 feet = 3.658 m
    EU Motorway: 3.5-3.75 m
    Urban: 3.0-3.5 m
    Typical: 3.5 m

  n=6 mapping:
    3.5 = 7/2 = (σ-sopfr)/φ? → forced
    3.658 m (12 ft) = σ in feet? → unit-dependent
    No clean n=6 match

  Grade: FAIL
  Lane width has no clean n=6 mapping. The values (3.5-3.75m)
  are determined by vehicle width (~1.8m) + safety margin,
  and are unit-dependent. Claiming σ=12 feet is trivially
  unit-dependent manipulation.
```

---

### H-AD-30: Automotive Ethernet — 100 Mbps / 1 Gbps / 10 Gbps

> In-vehicle Ethernet uses 100BASE-T1, 1000BASE-T1, 10GBASE-T1

```
  Automotive Ethernet (IEEE 802.3):
    100BASE-T1 (BroadR-Reach): 100 Mbps
    1000BASE-T1: 1 Gbps
    10GBASE-T1: 10 Gbps (emerging)
    Multigigabit: 2.5G, 5G planned

  n=6 mapping attempt:
    100 = ? No clean decomposition
    1000 = ? No clean decomposition
    10 = σ - φ ✓ (10 Gbps only)

  BUT:
    Ethernet speeds are powers of 10, which is the base of our
    number system. 100/1000/10000 Mbps is standard Ethernet
    progression across ALL domains, not automotive-specific.

  Grade: FAIL
  Ethernet speeds are decimal multiples with no n=6 structure.
  10 = σ-φ matching 10 Gbps is coincidental decimal round number.
```

---

### H-AD-31: Surround View Coverage — 360° = 60° × n = 30° × σ

> Full surround sensing requires 360° coverage

```
  360° coverage:
    360° = full azimuth circle
    360 = 6 × 60 = n × 60
    360 = 12 × 30 = σ × 30
    Each sensor covers ~60° → n=6 sensors for full circle?

  n=6 mapping:
    360 = n × 60 ✓
    If each sensor covers 60° → need n=6 sensors ✓
    12 ultrasonic sensors × 30° beam = 360° ✓ → σ(6) sensors

  Physical basis:
    360 degrees in a circle is a Babylonian convention (base-60).
    360 = 2³ × 3² × 5 = highly composite, chosen for divisibility.
    The divisibility by 6, 12, 24 is because 360 was designed for it.

  BUT:
    360° = n × 60 is true for ANY n=6, but 360 was chosen by
    Babylonians ~2000 BCE for its rich divisibility. The n=6
    connection is circular (pun intended) — 360 divides by 6
    because that's WHY 360 was chosen.
    Sensor FOV varies: cameras 50-120°, radar 120-150°, LiDAR 360°.

  Grade: CLOSE
  360 = n × 60 is exact, and the ultrasonic case (12 sensors × 30°)
  genuinely matches σ(6). The Babylonian origin of 360 is itself
  related to 6's high divisibility, creating a genuine structural echo.
  But individual sensor FOVs vary widely.
```

---

### H-AD-32: Tesla FSD Computer — Dual Redundant SoC

> Tesla HW3 FSD computer uses 2 redundant chips

```
  Tesla HW3 (2019):
    2 × custom SoC (Samsung 14nm)
    Each: 6 billion transistors, 72 TOPS
    Total: 144 TOPS

  n=6 mapping:
    2 chips = φ(6) ✓ (dual redundancy)
    72 TOPS = σ × n = 12 × 6 ✓
    144 TOPS = σ² = 12² ✓ ← notable!
    6 billion transistors = n billion ✓

  Cross-reference:
    BT-90: SM = φ × K₆ contact number → σ² = 144 SMs (GPU parallel)
    144 TOPS on Tesla FSD echoes 144 SMs on AD102

  BUT:
    72 TOPS per chip was an engineering target for FSD neural nets.
    6B transistors is approximate. TOPS = operations/second depends
    on precision (INT8 vs FP16). HW4 uses different numbers entirely.

  Grade: CLOSE
  144 = σ² total TOPS is a notable match and resonates with BT-90
  (σ² = 144 in GPU architecture). The 6B transistor count and
  72 = σ×n per chip add supporting matches. But HW4 breaks the
  pattern, suggesting this is coincidental for one hardware generation.
```

---

### H-AD-33: Waymo 6th Generation System

> Waymo is developing its 6th generation autonomous driving system

```
  Waymo Driver generations:
    Gen 1 (2009): Google Self-Driving Car Project
    Gen 2 (2012): Lexus RX450h platform
    Gen 3 (2015): Firefly prototype
    Gen 4 (2017): Chrysler Pacifica
    Gen 5 (2020): Jaguar I-PACE
    Gen 6 (2024+): Next generation
    Current generation count: 6

  n=6 mapping:
    6 generations = n ✓

  BUT:
    Generation count is simply sequential numbering.
    Any company that iterates 6 times reaches "gen 6."
    The number has no structural significance — it means
    Waymo has been iterating since 2009. By 2026 they may
    be at Gen 7.

  Grade: FAIL
  Sequential generation numbering reaching 6 is trivially
  explained by time passing. This is not a structural match.
```

---

### H-AD-34: Occupancy Grid Resolution — Typical 10 cm = 1/(σ-φ) m

> Occupancy grids in AD commonly use 10 cm cell resolution

```
  Occupancy grid parameters:
    Common resolution: 0.1 m = 10 cm (most planners)
    Range: 0.05 m to 0.2 m depending on use case
    Grid dimensions: typically 100×100 to 200×200 cells

  n=6 mapping:
    0.1 m = 1/(σ-φ) = 1/10 meters ✓
    0.1 = 1/(σ-φ) echoes BT-64 (0.1 universal regularization)

  BUT:
    10 cm = 0.1 m is a round metric number. It is the obvious
    choice for "good enough resolution for a ~2m wide car in a
    ~20m planning range" — 200 cells per axis is computationally
    manageable. The choice is driven by compute/accuracy tradeoff.

  Grade: WEAK
  0.1m is a natural metric round number, not an n=6-derived
  parameter. The connection to BT-64 is thematic but the
  explanations (regularization vs. grid resolution) are unrelated.
```

---

### H-AD-35: Waypoint Spacing — Typically 0.5-1.0 m

> AD trajectory waypoints are typically spaced 0.5-1.0 meters apart

```
  Waypoint spacing:
    Low-speed (parking): 0.1-0.3 m
    Urban driving: 0.5-1.0 m
    Highway: 1.0-2.0 m
    Common default: 1.0 m

  n=6 mapping:
    1.0 m = R(6) = σφ/(nτ) = 1 ← identity/trivial
    0.5 m = 1/φ ✓

  Grade: FAIL
  Waypoint spacing depends on speed and planning resolution.
  1.0 m is a round number. Matching it to R(6) = 1 is trivially
  true of any identity. No structural content.
```

---

## Grade Summary

| ID | Hypothesis | n=6 Formula | Grade |
|----|-----------|-------------|-------|
| H-AD-01 | SAE 6 autonomy levels | n=6 | **EXACT** |
| H-AD-02 | IMU 6 DOF | n=6 | **EXACT** |
| H-AD-03 | ASIL 4 safety levels | τ=4 | **CLOSE** |
| H-AD-04 | Traffic light 4 phases | τ=4 | **CLOSE** |
| H-AD-05 | Tesla 8 cameras | σ-τ=8 | **WEAK** |
| H-AD-06 | 12 ultrasonic sensors | σ=12 | **EXACT** |
| H-AD-07 | LiDAR channels 16/32/64/128 | 2^{τ,sopfr,n,σ-sopfr} | **WEAK** |
| H-AD-08 | Waymo 5 LiDAR units | sopfr=5 | **WEAK** |
| H-AD-09 | 4 corner radars | τ=4 | **CLOSE** |
| H-AD-10 | 77 GHz radar frequency | — | **FAIL** |
| H-AD-11 | CAN 8-byte payload | σ-τ=8 | **WEAK** |
| H-AD-12 | CAN baud rates | — | **FAIL** |
| H-AD-13 | DSRC 5.9 GHz | ≈n | **FAIL** |
| H-AD-14 | V2X 6 comm modes | n=6 | **CLOSE** |
| H-AD-15 | C-V2X sidelink bandwidth | σ=12 MHz | **WEAK** |
| H-AD-16 | 8 object classes | σ-τ=8 | **WEAK** |
| H-AD-17 | NMS threshold 0.5 | 1/φ=0.5 | **WEAK** |
| H-AD-18 | Camera 30/60 fps | sopfr·n, σ·sopfr | **WEAK** |
| H-AD-19 | 3 anchor aspect ratios | n/φ=3 | **WEAK** |
| H-AD-20 | PID 3 terms | n/φ=3 | **WEAK** |
| H-AD-21 | MPC 5-10 step horizon | n range | **WEAK** |
| H-AD-22 | FSM 5 driving states | sopfr=5 | **WEAK** |
| H-AD-23 | 3 planning hierarchy levels | n/φ=3 | **WEAK** |
| H-AD-24 | Dual control modes | φ=2 | **WEAK** |
| H-AD-25 | HD map ~6 layers | n=6 | **WEAK** |
| H-AD-26 | Localization ~6 cm precision | — | **FAIL** |
| H-AD-27 | Triple modular redundancy | n/φ=3 | **WEAK** |
| H-AD-28 | Dual sensor redundancy | φ=2 | **WEAK** |
| H-AD-29 | Lane width 3.5-3.75 m | — | **FAIL** |
| H-AD-30 | Automotive Ethernet speeds | — | **FAIL** |
| H-AD-31 | 360° = n×60 surround view | n×60 | **CLOSE** |
| H-AD-32 | Tesla FSD 144 TOPS = σ² | σ²=144 | **CLOSE** |
| H-AD-33 | Waymo 6th gen system | n=6 | **FAIL** |
| H-AD-34 | Occupancy grid 10 cm | 1/(σ-φ) | **WEAK** |
| H-AD-35 | Waypoint spacing 1.0 m | R(6)=1 | **FAIL** |

## Grade Distribution

| Grade | Count | Pct | Hypotheses |
|-------|-------|-----|------------|
| EXACT | 3 | 8.6% | H-AD-01, H-AD-02, H-AD-06 |
| CLOSE | 6 | 17.1% | H-AD-03, H-AD-04, H-AD-09, H-AD-14, H-AD-31, H-AD-32 |
| WEAK | 18 | 51.4% | H-AD-05, H-AD-07, H-AD-08, H-AD-11, H-AD-15, H-AD-16, H-AD-17, H-AD-18, H-AD-19, H-AD-20, H-AD-21, H-AD-22, H-AD-23, H-AD-24, H-AD-25, H-AD-27, H-AD-28, H-AD-34 |
| FAIL | 8 | 22.9% | H-AD-10, H-AD-12, H-AD-13, H-AD-26, H-AD-29, H-AD-30, H-AD-33, H-AD-35 |

**Non-failing total: 27/35 (77.1%)**
**Strong matches (EXACT+CLOSE): 9/35 (25.7%)**
