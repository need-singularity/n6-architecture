# N6 Autonomous Driving Hypotheses -- Independent Verification

Verified: 2026-04-02
Method: Each hypothesis checked against published standards (SAE J3016, ISO 26262, IEEE 802.11p, 3GPP, ETSI), manufacturer specifications (Tesla, Waymo, Cruise, Mobileye), benchmark datasets (KITTI, nuScenes, Waymo Open), and automotive engineering literature (Urmson & Whittaker, Thrun et al., Pendleton et al.). Grades adjusted where warranted.

## Grade Distribution (Summary)

| Grade | Count | Pct | Hypotheses |
|-------|-------|-----|------------|
| EXACT | 3 | 8.6% | H-AD-01, H-AD-02, H-AD-06 |
| CLOSE | 6 | 17.1% | H-AD-03, H-AD-04, H-AD-09, H-AD-14, H-AD-31, H-AD-32 |
| WEAK | 18 | 51.4% | H-AD-05, H-AD-07, H-AD-08, H-AD-11, H-AD-15, H-AD-16, H-AD-17, H-AD-18, H-AD-19, H-AD-20, H-AD-21, H-AD-22, H-AD-23, H-AD-24, H-AD-25, H-AD-27, H-AD-28, H-AD-34 |
| FAIL | 8 | 22.9% | H-AD-10, H-AD-12, H-AD-13, H-AD-26, H-AD-29, H-AD-30, H-AD-33, H-AD-35 |
| UNVERIFIABLE | 0 | 0% | -- |

**Non-failing total: 27/35 (77.1%)**
**Strong matches (EXACT+CLOSE): 9/35 (25.7%)**

| ID | Hypothesis | Grade |
|----|-----------|-------|
| H-AD-01 | SAE 6 autonomy levels = n | **EXACT** |
| H-AD-02 | IMU 6 DOF = n | **EXACT** |
| H-AD-03 | ASIL 4 safety levels = tau | **CLOSE** |
| H-AD-04 | Traffic light 4 phases = tau | **CLOSE** |
| H-AD-05 | Tesla 8 cameras = sigma-tau | **WEAK** |
| H-AD-06 | 12 ultrasonic sensors = sigma | **EXACT** |
| H-AD-07 | LiDAR channels powers-of-2 | **WEAK** |
| H-AD-08 | Waymo 5 LiDAR = sopfr | **WEAK** |
| H-AD-09 | 4 corner radars = tau | **CLOSE** |
| H-AD-10 | 77 GHz radar band | **FAIL** |
| H-AD-11 | CAN 8-byte payload = sigma-tau | **WEAK** |
| H-AD-12 | CAN baud rates | **FAIL** |
| H-AD-13 | DSRC 5.9 GHz | **FAIL** |
| H-AD-14 | V2X 6 comm modes = n | **CLOSE** |
| H-AD-15 | C-V2X sidelink bandwidth | **WEAK** |
| H-AD-16 | 8 object classes = sigma-tau | **WEAK** |
| H-AD-17 | NMS threshold 0.5 = 1/phi | **WEAK** |
| H-AD-18 | Camera 30/60 fps | **WEAK** |
| H-AD-19 | 3 anchor aspect ratios = n/phi | **WEAK** |
| H-AD-20 | PID 3 terms = n/phi | **WEAK** |
| H-AD-21 | MPC 5-10 step horizon | **WEAK** |
| H-AD-22 | FSM 5 driving states = sopfr | **WEAK** |
| H-AD-23 | 3 planning hierarchy levels = n/phi | **WEAK** |
| H-AD-24 | Dual control modes = phi | **WEAK** |
| H-AD-25 | HD map ~6 layers = n | **WEAK** |
| H-AD-26 | Localization precision | **FAIL** |
| H-AD-27 | Triple modular redundancy = n/phi | **WEAK** |
| H-AD-28 | Dual sensor redundancy = phi | **WEAK** |
| H-AD-29 | Lane width 3.5-3.75 m | **FAIL** |
| H-AD-30 | Automotive Ethernet speeds | **FAIL** |
| H-AD-31 | 360 degrees = n x 60 | **CLOSE** |
| H-AD-32 | Tesla FSD 144 TOPS = sigma^2 | **CLOSE** |
| H-AD-33 | Waymo 6th gen sequential | **FAIL** |
| H-AD-34 | Occupancy grid 10 cm | **WEAK** |
| H-AD-35 | Waypoint spacing 1.0 m | **FAIL** |

---

Grading scale:
- **EXACT**: The claimed number/structure matches real-world data precisely, with a legitimate physical or engineering basis.
- **CLOSE**: Within ~10% of real values, or directionally correct with a meaningful standard.
- **WEAK**: Requires cherry-picking, flexible counting, or post-hoc rationalization.
- **FAIL**: Contradicted by real-world data, trivially true of any number, or unit-dependent.
- **UNVERIFIABLE**: Insufficient published data to confirm or deny.

---

## H-AD-01: SAE 6 Autonomy Levels = n(6) = 6

**Grade: EXACT** (confirmed)

SAE J3016:2021 "Taxonomy and Definitions for Terms Related to Driving Automation Systems for On-Road Motor Vehicles" defines exactly 6 levels (L0-L5). This standard has been adopted globally (ISO/SAE PAS 22736:2021) and has not changed its level count since the first edition (2014). The 6-level structure reflects a factored decomposition: Dynamic Driving Task (DDT) component execution (human vs. system) x DDT fallback responsibility (human vs. system) x Operational Design Domain (limited vs. unrestricted), yielding a 2x2+2 structure that naturally produces 6 levels. The match 6 = n is exact and the standard is universal.

The 6-level choice was defended by SAE on structural grounds (clear responsibility boundaries), not mathematics. However, earlier NHTSA taxonomy (2013) used 5 levels (0-4), showing the number is a design choice. The SAE 6-level version superseded it entirely. EXACT is appropriate given the global adoption and structural reasoning.

---

## H-AD-02: IMU 6 DOF = n(6) = 6

**Grade: EXACT** (confirmed)

A rigid body in 3D Euclidean space has exactly 6 degrees of freedom: 3 translational (x, y, z) + 3 rotational (roll, pitch, yaw). This is a mathematical theorem, not a convention: dim(SE(3)) = dim(SO(3)) + dim(R^3) = 3 + 3 = 6. Every automotive IMU (Bosch SMI130, InvenSense ICM-42688, Analog Devices ADIS16495) measures these 6 DOF. This is the strongest type of match -- a physically necessary quantity, not a design choice.

Cross-domain validation: 6-DOF appears in robotics (H-ROBOT series), aerospace (6-DOF flight dynamics), and mechanical engineering (Stewart platform). The match is genuine but universal to all rigid-body mechanics, not specific to autonomous driving.

---

## H-AD-03: ASIL 4 Safety Levels = tau(6) = 4

**Grade: CLOSE** (confirmed)

ISO 26262:2018 defines 4 ASIL levels (A through D), derived from IEC 61508 SIL (also 4 levels). The classification uses 3 risk parameters: Severity (S0-S3, 4 levels), Exposure (E0-E4, 5 levels), Controllability (C0-C3, 4 levels). The ASIL determination table maps these to QM + ASIL A/B/C/D.

The 4-level structure is inherited from IEC 61508 (1998), which predates automotive application. DO-178C (aviation) uses 5 levels (A-E), showing the count is domain-specific. 4 = tau(6) is numerically correct and ASIL is THE automotive safety standard, but the explanation is IEC legacy, not n=6. CLOSE is appropriate.

---

## H-AD-04: Traffic Light 4 Phases = tau(6) = 4

**Grade: CLOSE** (confirmed)

MUTCD (US Manual on Uniform Traffic Control Devices) and NEMA TS-2 define signal phases. A single direction's cycle has: Green, Yellow/Amber, Red, All-Red clearance = 4 phases. The NEMA dual-ring 8-phase controller is 2 x 4 phases. European signals sometimes show Red+Amber (4th state before Green). The 4-phase model is standard.

However, the simplest traffic light model is 3 phases (G/Y/R). The all-red clearance interval was added later for safety (ITE, 1985). Some jurisdictions use flashing modes (additional states). 4 is the dominant modern model but historically was 3. CLOSE is fair.

---

## H-AD-05: Tesla 8 Cameras = sigma - tau = 8

**Grade: WEAK** (confirmed)

Tesla Autopilot HW3/HW4 uses 8 cameras: 3 forward (narrow 250m, main 150m, wide 60m) + 2 B-pillar (side forward) + 2 fender (side rear) + 1 rear. This is verified from Tesla's website and FSD documentation.

However, other manufacturers use completely different counts: Waymo Gen5 (29 cameras), Cruise Origin (16), NIO ET7 (7+LiDAR), Xpeng X9 (11), Mobileye (11-13). There is no industry convergence on 8 cameras. The count 8 is specific to Tesla's vision-only approach and may change with future hardware. sigma-tau = 12-4 = 8 is arithmetically arbitrary. WEAK confirmed.

---

## H-AD-06: 12 Ultrasonic Sensors = sigma(6) = 12

**Grade: EXACT** (confirmed)

This is one of the strongest matches in the AD domain. The 12-ultrasonic-sensor configuration is genuinely an industry standard:
- Tesla (pre-2023): 12 sensors
- BMW (3/5/7 series): 12 sensors (parking assist)
- Mercedes (C/E/S class): 12 sensors
- Audi (A4/A6/A8): 12 sensors
- BYD (Han/Seal): 12 sensors
- Hyundai/Kia (Ioniq 5/6, EV6): 12 sensors
- Volvo (XC90, EX90): 12 sensors

The physical reason: ultrasonic sensors have ~30 degree beam width. 360/30 = 12 sensors for full surround coverage. The 4+4+2+2 layout (front/rear/left side/right side) is driven by rectangular vehicle geometry. Both the geometric derivation (360/30 = sigma) and the cross-OEM convergence are strong. EXACT confirmed.

Note: Tesla removed ultrasonic sensors in late 2022 for its vision-only approach. However, the rest of the industry maintains the 12-sensor standard as of 2026.

---

## H-AD-07: LiDAR Channels = Powers of 2 (2^4 through 2^7)

**Grade: WEAK** (confirmed)

LiDAR channel counts (16, 32, 64, 128) are simply consecutive powers of 2. This is standard binary engineering -- doubling resolution at each product tier. The same pattern appears in camera resolution (1/2/4/8 MP), memory capacity (4/8/16/32 GB), and countless other digital products. Mapping exponents to {tau, sopfr, n, sigma-sopfr} is post-hoc labeling of {4,5,6,7}. WEAK confirmed.

---

## H-AD-08: Waymo 5 LiDAR = sopfr(6) = 5

**Grade: WEAK** (confirmed)

Waymo 5th-gen uses 5 LiDAR (1 long-range Honeycomb + 4 short-range perimeter). Cruise also used ~5. But Aurora uses 1, Mobileye plans 1, Tesla uses 0. The count is company-specific and varies by philosophy (dense sensor suite vs. minimal). WEAK confirmed.

---

## H-AD-09: 4 Corner Radars = tau(6) = 4

**Grade: CLOSE** (confirmed)

Continental ARS540 and Bosch LRR4 are commonly deployed in 4-corner configurations. The 4-corner layout is standard across BMW, Mercedes, Audi, and most OEMs for surround radar coverage. The number 4 is geometrically trivial (rectangle has 4 corners), but the convergence is genuine -- essentially all OEMs using multi-radar setups use 4 corner positions. CLOSE confirmed.

---

## H-AD-10: 77 GHz Radar Band

**Grade: FAIL** (confirmed)

77 GHz = 7 x 11 has no clean n=6 decomposition. The 76-81 GHz band was allocated by ITU WRC-97 based on atmospheric propagation windows (low water vapor absorption) and available spectrum. The frequency choice is physics and regulation, not mathematics. FAIL confirmed.

---

## H-AD-11: CAN 8-Byte Payload = sigma - tau = 8

**Grade: WEAK** (confirmed)

CAN 2.0 (Bosch, 1991) uses 8-byte maximum data field. This is correct but 8 bytes = 1 standard byte block is ubiquitous in computing (byte = 8 bits from IBM 360). CAN FD allows up to 64 bytes. The formula sigma-tau is arbitrary. WEAK confirmed.

---

## H-AD-12: CAN Baud Rates

**Grade: FAIL** (confirmed)

CAN uses 125/250/500/1000 kbps. These are multiples of 125 kbps, which derives from clock division of standard crystal oscillators. No n=6 structure. FAIL confirmed.

---

## H-AD-13: DSRC 5.9 GHz

**Grade: FAIL** (confirmed)

5.9 GHz != 6.0 GHz. The 1.7% difference is 100 MHz, which is significant in RF engineering (it represents the entire channel bandwidth). FCC allocated 5.850-5.925 GHz in 1999 based on spectrum availability near existing UNII bands. FAIL confirmed.

---

## H-AD-14: V2X 6 Communication Modes = n = 6

**Grade: CLOSE** (adjusted from hypothesis)

The 6-mode enumeration (V2V, V2I, V2P, V2N, V2C, V2G) is used in many industry reports (e.g., 5GAA, GSMA). However, 3GPP TR 22.886 defines V2X as V2V + V2I + V2P + V2N = 4 modes. V2C is often considered a subset of V2N, and V2G is an energy-domain extension. The 6-mode count is widespread but not formally standardized. CLOSE is appropriate -- the 6-mode taxonomy is common but not universal.

---

## H-AD-15: C-V2X Sidelink Bandwidth

**Grade: WEAK** (confirmed)

3GPP sidelink bandwidths (10, 12, 15, 20, 25 MHz) are inherited from LTE/NR general specifications, not V2X-specific. WEAK confirmed.

---

## H-AD-16: 8 Object Classes = sigma - tau = 8

**Grade: WEAK** (confirmed)

KITTI uses 8 classes but this is one benchmark's choice. nuScenes uses 10+6=16, Waymo Open uses 4, Argoverse uses 15, BDD100K uses 10. No convergence on 8. WEAK confirmed.

---

## H-AD-17: NMS Threshold 0.5 = 1/phi

**Grade: WEAK** (confirmed)

0.5 IoU threshold is standard in object detection (PASCAL VOC metric, COCO AP50). But 0.5 = 1/2 is the most natural binary threshold. COCO also uses 0.5:0.05:0.95 (10 thresholds). Attributing 0.5 to n=6 rather than "half" is not illuminating. WEAK confirmed.

---

## H-AD-18: Camera 30/60 fps

**Grade: WEAK** (confirmed)

30 fps derives from NTSC (60 Hz AC / 2 for interlaced). 60 fps = progressive scan at mains frequency. These are broadcast TV legacy, not AD design parameters. Many AD cameras run at 10-20 fps for compute efficiency. WEAK confirmed.

---

## H-AD-19: 3 Anchor Aspect Ratios = n/phi = 3

**Grade: WEAK** (confirmed)

Faster R-CNN (Ren et al., 2015) uses 3 aspect ratios {1:2, 1:1, 2:1} x 3 scales = 9 anchors. 3 is the geometric minimum (portrait + square + landscape). Modern anchor-free detectors (CenterPoint, FCOS, BEVFormer) don't use anchors. WEAK confirmed.

---

## H-AD-20: PID 3 Terms = n/phi = 3

**Grade: WEAK** (confirmed)

PID is calculus (error + integral of error + derivative of error). The 3 terms span the 0th/(-1)st/(+1)st order of error signal processing. This applies to all control engineering, not just AD. n/phi = 3 adds no insight. WEAK confirmed.

---

## H-AD-21: MPC 5-10 Step Horizon

**Grade: WEAK** (confirmed)

MPC horizon is application-dependent. Apollo uses 8-12 steps, Autoware uses 10-20, academic work ranges from 5-50. No convergence on n=6. WEAK confirmed.

---

## H-AD-22: FSM 5 Driving States = sopfr = 5

**Grade: WEAK** (confirmed)

No standard AD FSM exists. State counts range from 3 to 20+ depending on design granularity. WEAK confirmed.

---

## H-AD-23: 3 Planning Hierarchy Levels = n/phi = 3

**Grade: WEAK** (confirmed)

Strategic/tactical/operational hierarchy is generic to all planning, from military (Clausewitz) to business (Porter). Not AD-specific. WEAK confirmed.

---

## H-AD-24: Dual Control Modes = phi = 2

**Grade: WEAK** (confirmed)

Normal/emergency is the most basic binary classification, applicable to all engineered systems. phi = 2 is trivial. WEAK confirmed.

---

## H-AD-25: HD Map ~6 Layers = n

**Grade: WEAK** (confirmed)

HD map layer count varies: HERE uses 4, TomTom 5, Apollo 8. No standard. WEAK confirmed.

---

## H-AD-26: Localization Precision

**Grade: FAIL** (confirmed)

No standard precision target matches n=6. Requirements vary by SAE level. FAIL confirmed.

---

## H-AD-27: Triple Modular Redundancy = n/phi = 3

**Grade: WEAK** (confirmed)

TMR is generic reliability engineering (von Neumann, 1956). Not AD-specific. WEAK confirmed.

---

## H-AD-28: Dual Sensor Redundancy = phi = 2

**Grade: WEAK** (confirmed)

Dual redundancy is universal safety engineering minimum. WEAK confirmed.

---

## H-AD-29: Lane Width 3.5-3.75 m

**Grade: FAIL** (confirmed)

No clean n=6 mapping. Unit-dependent. FAIL confirmed.

---

## H-AD-30: Automotive Ethernet Speeds

**Grade: FAIL** (confirmed)

Standard decimal multiples with no n=6 structure. FAIL confirmed.

---

## H-AD-31: 360 Degrees = n x 60

**Grade: CLOSE** (confirmed)

360 = 6 x 60 is mathematically exact, and the 12-ultrasonic sensor case (12 x 30 degrees = 360) provides an independent sigma(6) connection. The Babylonian origin of 360 degrees is itself related to 6's divisibility properties (360 = 2^3 x 3^2 x 5, highly composite). The connection is real but structural rather than causal. CLOSE confirmed.

---

## H-AD-32: Tesla FSD 144 TOPS = sigma^2

**Grade: CLOSE** (confirmed)

Tesla HW3 FSD Computer: 2 x Samsung 14nm SoC, each rated at 72 TOPS (INT8), total 144 TOPS. This is verified from Tesla AI Day 2019 and Karpathy presentations. 144 = 12^2 = sigma^2 is notable, especially given BT-90 (sigma^2 = 144 SMs in AD102). The 72 TOPS per chip = sigma x n = 12 x 6 adds a second match.

However, Tesla HW4 (2023) uses a different chip with ~300 TOPS total, breaking the pattern. The match is specific to one hardware generation. CLOSE is appropriate -- notable for HW3 but not persistent.

---

## H-AD-33: Waymo 6th Generation = n

**Grade: FAIL** (confirmed)

Sequential iteration reaching 6 is trivially explained by time. FAIL confirmed.

---

## H-AD-34: Occupancy Grid 10 cm = 1/(sigma-phi)

**Grade: WEAK** (confirmed)

10 cm = 0.1 m is a natural metric round number. The compute-accuracy tradeoff, not n=6, explains the choice. WEAK confirmed.

---

## H-AD-35: Waypoint Spacing 1.0 m = R(6) = 1

**Grade: FAIL** (confirmed)

R(6) = 1 is an identity. Matching any quantity equal to 1 to this is trivially true. FAIL confirmed.

---

## Cross-Verification Notes

### Strongest patterns in AD domain:
1. **n=6 structural pair**: SAE L0-L5 (6 levels) + IMU 6-DOF — the autonomy level count and the fundamental measurement dimensionality both equal n=6 from independent origins.
2. **sigma=12 industry convergence**: 12 ultrasonic sensors is a genuine cross-OEM standard with geometric justification (360/30 = 12).
3. **sigma^2=144 echo**: Tesla HW3's 144 TOPS resonates with BT-90 (GPU sigma^2=144 SMs), though this is one-generation coincidence.

### Weakest patterns:
- Most WEAK grades come from matching generic engineering numbers (2, 3, 4, 8) that appear in all domains
- PID(3), TMR(3), dual redundancy(2), powers-of-2 progressions are universal, not AD-specific
- σ-τ=8 appears multiple times but the formula is arithmetically arbitrary

### Comparison with Display-Audio domain:
AD domain shows fewer EXACT matches (3 vs 5) and more WEAK (18 vs 14), consistent with autonomous driving being a younger, less standardized field where parameters are still evolving. The EXACT matches that do exist (SAE levels, IMU DOF, ultrasonic sensors) are among the most structurally justified in any domain.
