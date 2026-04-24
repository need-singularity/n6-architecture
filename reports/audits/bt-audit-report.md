# BT Audit Report — BT-1~541 full audit (ID range BT-6~540)

- Audit target: `docs/breakthrough-theorems.md`

- Audit script: `scripts/audit_bt.py`

- Total BT count: **541**


## Summary

| Category | Count | Ratio |
|---|---|---|
| EXACT | 481 | 88.9% |
| PARTIAL | 54 | 10.0% |
| MISS | 6 | 1.1% |
| UNKNOWN | 0 | 0.0% |

**BT-unit match rates (dual metric)**

- EXACT/(total) = 481/541 = 88.9% (including UNKNOWN 0)
- EXACT/(decidable) = 481/541 = 88.9% (excluding UNKNOWN)
- (EXACT+PARTIAL)/(total) = 535/541 = 98.9%
- (EXACT+PARTIAL)/(decidable) = 535/541 = 98.9%

**Row-unit**: EXACT 3498 / CLOSE 95 / MISS 71 (sum 3664) — MISS rate 1.94%

**mismatch total: 71** (target <50)


## MISS-classified BT

| BT | Title | EXACT | MISS |
|---|---|---|---|
| BT-475 | Planck 2018 Ω n=6 | 2 | 3 |
| BT-476 | CMB — ℓ ≈ 220 = σ(6)·... | 1 | 3 |
| BT-478 | τ=4 | 3 | 2 |
| BT-483 | CODATA n=6 | 3 | 2 |
| BT-487 | 13.8 Gyr / Hubble τ_H — n=6 | 2 | 3 |
| BT-503 | 6 — Brayton τ=4 | 3 | 2 |


## PARTIAL-classified BT (includes some MISS)

| BT | Title | EXACT | MISS |
|---|---|---|---|
| BT-463 | LTR σ(6)=12 / 6bp | 4 | 1 |
| BT-464 | Tat-TAR 6 | 4 | 1 |
| BT-465 | Rev-RRE 4→6 | 4 | 1 |
| BT-466 | HIV C₂ / φ=2 | 4 | 1 |
| BT-467 | HIV 6 | 4 | 1 |
| BT-468 | (bNAb) σ(6)=1+2+3+6 | 4 | 1 |
| BT-469 | CCR5 6nt PAM / CRISPR | 4 | 1 |
| BT-470 | HEXA-ART — HIV 6 n=6 | 4 | 1 |
| BT-471 | 6 = n — PDG | 4 | 1 |
| BT-472 | σ-sopfr=7 — + | 3 | 1 |
| BT-474 | 3 = n/φ | 3 | 1 |
| BT-477 | τ=3 — M,Q,J | 3 | 1 |
| BT-481 | BCS 2Δ/k_BT_c ≈ 3.53 → n/φ | 3 | 1 |
| BT-482 | σ_xy=νe²/h — n=6 plateau | 3 | 1 |
| BT-484 | Bell CHSH 2 = φ — 2√2 | 4 | 1 |
| BT-488 | = φ=2 — | 4 | 1 |
| BT-491 | ADD n=6 — Arkani-Hamed/Dimopoulos/Dvali | 4 | 1 |
| BT-493 | M- 11 → σ-sopfr=7 | 4 | 1 |
| BT-494 | critical dimension 26/10 = φ=2 | 4 | 1 |
| BT-498 | (RO) 6 = n | 4 | 2 |
| BT-499 | (MED/MSF) σ=12 | 4 | 1 |
| BT-500 | 3D 6 = n — FDM/SLA/SLS | 5 | 1 |
| BT-501 | 3D — n=6 | 4 | 1 |
| BT-502 | 6 = n — / | 4 | 2 |
| BT-504 | 12 HP + 24 LP — σ/J₂ | 4 | 1 |
| BT-505 | 6 = n — | 4 | 1 |
| BT-506 | HACCP σ-sopfr=7 | 4 | 1 |
| BT-507 | 6 = n | 4 | 1 |
| BT-508 | CSS Box Model τ=4 — | 5 | 1 |
| BT-509 | σ=12 — WDM | 4 | 1 |
| BT-510 | SiO₂ — 4 CN=τ / n=6 | 4 | 1 |
| BT-511 | — / n=6 | 4 | 1 |
| BT-512 | 6 = n — FAA | 4 | 1 |
| BT-513 | 1435 mm — σ-sopfr=7 × 205 | 4 | 1 |
| BT-514 | 6 GMP — ICH Q7 | 4 | 1 |
| BT-515 | — 300 mm = σ·(J₂+μ) | 4 | 1 |
| BT-516 | NAND — 2^n=64 / n/φ=3 bit TLC | 5 | 1 |
| BT-517 | — 6 / Ekman / σ=12 | 4 | 1 |
| BT-518 | — 6 / n=6 | 4 | 1 |
| BT-519 | 6 — / φ=2 | 4 | 1 |
| BT-520 | DNA — 6 / 64=φⁿ | 4 | 1 |
| BT-521 | — 6 = n | 4 | 1 |
| BT-522 | — 6 / σ-μ=11 | 5 | 1 |
| BT-523 | - — 12 σ = AES-128 (σ-sopfr) | 4 | 1 |
| BT-524 | — VEI 8 / n=6 | 4 | 1 |
| BT-525 | — 6 / 28 P₂ | 4 | 1 |
| BT-526 | — Fe-C / FCC CN=σ=12 | 4 | 1 |
| BT-527 | — 6 / σ-τ=8 | 5 | 1 |
| BT-528 | - — 12TET = σ, A440 = σ·sopfr·(σ-sopfr)+σ·φ | 4 | 1 |
| BT-529 | - — 6 / n=6 | 4 | 1 |
| BT-530 | GPS-- — σ=12 | 4 | 1 |
| BT-535 | A = n/φ = 3 → φ = 2 — | 2 | 1 |
| BT-536 | DC — 1 = σ·sopfr = 60 MWe, σ·n = 72 | 4 | 1 |
| BT-539 | REBCO — σ T , τ , J₂-τ K | 5 | 1 |


## UNKNOWN  BT (0) —  ,   

| BT | Title |
|---|---|


## mismatch/MISS  ( 100)

Total 71

```
MISS tag in BT-463: MISS
MISS tag in BT-464: MISS
MISS tag in BT-465: MISS
MISS tag in BT-466: MISS
MISS tag in BT-467: MISS
MISS tag in BT-468: MISS
MISS tag in BT-469: MISS
MISS tag in BT-470: MISS
MISS tag in BT-471: MISS
MISS tag in BT-472:   
MISS tag in BT-474: sin²θ₁₃
MISS tag in BT-475: Ω_Λ
MISS tag in BT-475: Ω_m
MISS tag in BT-475: H₀ (km/s/Mpc)
MISS tag in BT-476:    ℓ₁
MISS tag in BT-476: 2/1  
MISS tag in BT-476:     (Mpc/h)
MISS tag in BT-477: Kerr a  a/M
MISS tag in BT-478: 2D  β 
MISS tag in BT-478: 3D  ν
MISS tag in BT-481: 2Δ/k_B T_c (BCS)
MISS tag in BT-482: von Klitzing  R_K (Ω)
MISS tag in BT-483: h (J·s)
MISS tag in BT-483: e (C)
MISS tag in BT-484: Tsirelson 
MISS tag in BT-487: t₀ (Gyr)
MISS tag in BT-487: H₀ (km/s/Mpc)
MISS tag in BT-487: CMB  (K)
MISS tag in BT-488: 4D   
MISS tag in BT-491: n_extra=2   (m)
MISS tag in BT-493:   
MISS tag in BT-494:  
MISS tag in BT-498:  TDS (mg/L)
MISS tag in BT-498: MISS
MISS tag in BT-499: MISS
MISS tag in BT-500: MISS
MISS tag in BT-501: MISS
MISS tag in BT-502:  30S/50S  
MISS tag in BT-502: MISS
MISS tag in BT-503:    (K)
MISS tag in BT-503: MISS
MISS tag in BT-504: MISS
MISS tag in BT-505:   ()
MISS tag in BT-506: MISS
MISS tag in BT-507: MISS
MISS tag in BT-508: MISS
MISS tag in BT-509: MISS
MISS tag in BT-510: MISS
MISS tag in BT-511: MISS
MISS tag in BT-512: MISS
MISS tag in BT-513: MISS
MISS tag in BT-514: MISS
MISS tag in BT-515: MISS
MISS tag in BT-516: MISS
MISS tag in BT-517:    (‰)
MISS tag in BT-518: MISS
MISS tag in BT-519: MISS
MISS tag in BT-520: MISS
MISS tag in BT-521: MISS
MISS tag in BT-522: MISS
MISS tag in BT-523: MISS
MISS tag in BT-524: MISS
MISS tag in BT-525: MISS
MISS tag in BT-526: MISS
MISS tag in BT-527: MISS
MISS tag in BT-528: MISS
MISS tag in BT-529: MISS
MISS tag in BT-530: 12-TET  = AES-128  
MISS tag in BT-535: MAST-U 
MISS tag in BT-536: Helion Polaris 
MISS tag in BT-539: REBCO  (K)
```


## Full result table

| BT | Status | EXACT | CLOSE | MISS | Title |
|---|---|---|---|---|---|
| BT-6 | EXACT | 3 | 0 | 0 | Golay-Leech Unification — [J_2, sigma, sigma-tau] = [24 |
| BT-7 | EXACT | 3 | 2 | 0 | Egyptian Fraction Power Theorem — 1/2 + 1/3 + 1/6 = 1 |
| BT-8 | EXACT | 5 | 0 | 0 | Pulse Rectifier Chain — n --> sigma --> J_2 (6 --> 12 - |
| BT-9 | EXACT | 1 | 0 | 0 | Bott Periodicity Bridge — sigma - tau = 8 |
| BT-10 | EXACT | 2 | 0 | 0 | Landauer-WHH Information-Thermodynamic Bridge — ln(phi) |
| BT-11 | EXACT | 14 | 0 | 0 | Software-Physics Isomorphism — tau=4, n/phi=3, sigma=12 |
| BT-12 | EXACT | 5 | 0 | 0 | Hamming-OSI-ECC Triple Bridge — [sigma-sopfr, tau, n/ph |
| BT-13 | EXACT | 4 | 0 | 0 | σ±μ Internet Infrastructure Duality — TCP(11) + DNS(13) |
| BT-19 | EXACT | 3 | 0 | 0 | GUT Hierarchy = n=6 Arithmetic — 11/11 Parameter Match |
| BT-17 | EXACT | 1 | 0 | 0 | SM Fermion-Boson σ-Balance — Core Theorem in Particle P |
| BT-15 | EXACT | 1 | 0 | 0 | Kissing Number Quadruple — K₁..₄ = (φ, n, σ, J₂) |
| BT-16 | EXACT | 4 | 2 | 0 | Riemann Zeta Trident — ζ(s) generates n=6 at three inde |
| BT-14 | EXACT | 10 | 0 | 0 | Carbon-Silicon Tetrahedral Bridge — Life and Computing  |
| BT-18 | EXACT | 2 | 0 | 0 | The Vacuum Energy Chain — From R(n)=1 to the Monster Gr |
| BT-20 | EXACT | 2 | 0 | 0 | Gauge Coupling Trinity — Three SM Couplings = n=6 Arith |
| BT-21 | EXACT | 3 | 0 | 0 | Neutrino Mixing Trident — PMNS Angles from n=6 Fraction |
| BT-22 | EXACT | 2 | 0 | 0 | Inflation from Perfect Numbers — n_s = 1 - 1/P₂ = 27/28 |
| BT-23 | EXACT | 1 | 0 | 0 | CKM Quark Mixing Hierarchy — |V_ub| = r = 3/784 |
| BT-24 | EXACT | 2 | 0 | 0 | Koide Pole Residue — φ²/n = 2/3 |
| BT-25 | EXACT | 8 | 1 | 0 | Genetic Code Arithmetic — Life's Information System fro |
| BT-26 | EXACT | 1 | 0 | 0 | Chinchilla Scaling Law Constants from n=6 Arithmetic |
| BT-27 | EXACT | 4 | 0 | 0 | Carbon-6 Universal Energy Chain — LiC₆ + C₆H₁₂O₆ + C₆H₆ |
| BT-28 | EXACT | 5 | 0 | 0 | Computing Architecture Ladder — Exponents Trace n=6 Con |
| BT-29 | EXACT | 3 | 0 | 0 | IEEE 519 Power Quality = sopfr + n/φ + (σ-τ) |
| BT-30 | EXACT | 4 | 0 | 0 | Shockley-Queisser Bridge — Solar Bandgap + Thermal Volt |
| BT-31 | EXACT | 6 | 0 | 0 | MoE Expert Routing Vocabulary = {μ, φ, n, σ-τ} |
| BT-32 | EXACT | 1 | 0 | 0 | Nuclear Fission Scaffold — 6 Delayed Neutron Groups |
| BT-33 | EXACT | 11 | 0 | 0 | Transformer Dimension Ladder — σ(6)=12 as the Architect |
| BT-34 | EXACT | 9 | 0 | 0 | RoPE Base Frequency Family — (σ-φ)^{τ, sopfr, n} = {10⁴ |
| BT-35 | EXACT | 7 | 0 | 0 | Battery Voltage Periodic Table — Cell Potentials from n |
| BT-36 | EXACT | 5 | 0 | 0 | Grand Energy-Information-Hardware-Physics Chain |
| BT-37 | EXACT | 8 | 0 | 0 | Semiconductor Lithography Pitch — P₂ = 28nm at TSMC N5 |
| BT-38 | EXACT | 4 | 0 | 0 | Hydrogen Energy Density Quadruplet — 120/142/113/118 MJ |
| BT-39 | EXACT | 12 | 0 | 0 | KV-Head Universality + Mistral Large 2 as n=6 Archetype |
| BT-40 | EXACT | 10 | 0 | 0 | Computing Power Ecosystem — ATX 12V + ACPI Triple-τ |
| BT-41 | EXACT | 1 | 0 | 0 | Quantum Error Correction at J₂ — Surface Code d=5 Syndr |
| BT-42 | EXACT | 2 | 0 | 0 | Inference Scaling Law — Test-Time Compute from n=6 |
| BT-43 | EXACT | 9 | 0 | 0 | Battery Cathode Universality — All Li-ion = CN6 |
| BT-44 | EXACT | 6 | 0 | 0 | LLM Context Window Ladder — Exponents Trace σ±{φ,μ} |
| BT-45 | EXACT | 3 | 0 | 0 | AI Chip FP8/FP16 Ratio = φ(6) = 2 Universal |
| BT-46 | EXACT | 3 | 0 | 0 | ln(4/3) RLHF Constant Family — Information Bandwidth |
| BT-47 | EXACT | 10 | 0 | 0 | Interconnect Generation Counts = {σ-sopfr, sopfr, n} |
| BT-48 | EXACT | 4 | 0 | 0 | Display-Audio Universal Constants — σ=12, J₂=24, σ·τ=48 |
| BT-49 | EXACT | 6 | 0 | 0 | Pure Math Bridge — Bernoulli + Kissing + S₆ + Perfect C |
| BT-50 | EXACT | 4 | 0 | 0 | Programming Language Constants — IEEE 754 Exponent Ladd |
| BT-51 | EXACT | 5 | 0 | 0 | Genetic Code Information Chain — τ→(n/φ)→2^n→(J₂-τ) |
| BT-52 | EXACT | 4 | 0 | 0 | Compiler + OS Kernel Constants |
| BT-53 | EXACT | 4 | 0 | 0 | Cryptocurrency Consensus Constants |
| BT-54 | EXACT | 5 | 0 | 0 | AdamW Optimizer Universals — The Training Quintuplet |
| BT-55 | EXACT | 9 | 0 | 0 | GPU HBM Capacity Ladder — n=6 Memory Hierarchy |
| BT-56 | EXACT | 14 | 0 | 0 | Complete n=6 LLM Architecture — The Canonical Design Th |
| BT-57 | EXACT | 7 | 0 | 0 | Battery Cell Count Ladder — Electrochemistry Meets n=6 |
| BT-58 | EXACT | 1 | 0 | 0 | σ-τ=8 — The Universal AI Engineering Constant |
| BT-59 | EXACT | 13 | 0 | 0 | The Complete AI Stack — 8-Layer Silicon-to-Inference Ch |
| BT-60 | EXACT | 2 | 0 | 0 | Datacenter Power-to-Inference Chain — 6 Voltage Steps,  |
| BT-61 | EXACT | 9 | 0 | 0 | Diffusion Model n=6 Universality — Complete Parameteriz |
| BT-62 | EXACT | 3 | 0 | 0 | Grid Frequency Pair — 60/50Hz from n=6 with PUE Bridge |
| BT-63 | EXACT | 2 | 0 | 0 | Solar Panel Cell Ladder — σ·{sopfr, n, σ-φ, σ} = {60, 7 |
| BT-64 | EXACT | 6 | 0 | 0 | 1/(σ-φ) = 0.1 Universal Regularization Constant |
| BT-65 | EXACT | 6 | 0 | 0 | Mamba SSM Complete n=6 Parameterization |
| BT-66 | EXACT | 2 | 0 | 0 | Vision AI Complete n=6 Universality |
| BT-67 | EXACT | 8 | 0 | 0 | MoE Activation Fraction Universal Law |
| BT-68 | EXACT | 10 | 0 | 0 | HVDC Voltage Ladder = (σ-μ, σ-φ, sopfr) · (σ-φ)² |
| BT-69 | EXACT | 5 | 0 | 0 | Chiplet Architecture n=6 Convergence |
| BT-70 | EXACT | 6 | 0 | 0 | 1/(σ-φ)=0.1 Universal Convergence — 8th Algorithm (SimC |
| BT-71 | EXACT | 1 | 0 | 0 | NeRF/3DGS Complete n=6 Parameterization |
| BT-72 | EXACT | 1 | 0 | 0 | Neural Audio Codec n=6 Universality |
| BT-73 | EXACT | 6 | 0 | 0 | Tokenizer Vocabulary n=6 Law |
| BT-74 | EXACT | 5 | 0 | 0 | The 95/5 Cross-Domain Resonance |
| BT-75 | EXACT | 2 | 0 | 0 | HBM Interface Width Exponent Ladder |
| BT-76 | EXACT | 5 | 0 | 0 | σ·τ = 48 Triple Attractor |
| BT-77 | EXACT | 7 | 0 | 0 | Cross-Vendor HBM Capacity Convergence to n=6 |
| BT-78 | EXACT | 6 | 0 | 0 | Interconnect Speed Ladder — PCIe/UCIe/CXL Follow n=6 Ex |
| BT-79 | EXACT | 6 | 0 | 0 | sigma^2 = 144 Cross-Domain Attractor |
| BT-80 | EXACT | 6 | 0 | 0 | Solid-State Electrolyte CN=6 Universality |
| BT-81 | EXACT | 0 | 2 | 0 | Anode Capacity Ladder sigma-phi = 10x |
| BT-82 | EXACT | 7 | 3 | 0 | Complete Battery Pack n=6 Parameter Map |
| BT-83 | EXACT | 5 | 1 | 0 | Li-S Polysulfide n=6 Decomposition Ladder |
| BT-84 | EXACT | 5 | 0 | 0 | 96/192 Energy-Computing-AI Triple Convergence |
| BT-85 | EXACT | 16 | 2 | 0 | Carbon Z=6 Material Synthesis Universality |
| BT-86 | EXACT | 21 | 1 | 0 | Crystal Coordination Number CN=6 Law |
| BT-87 | EXACT | 11 | 3 | 0 | Atomic Manipulation Precision n=6 Ladder |
| BT-88 | EXACT | 18 | 0 | 0 | Self-Assembly n=6 Hexagonal Universality |
| BT-89 | EXACT | 12 | 3 | 0 | Photonic-Energy n=6 Bridge |
| BT-90 | EXACT | 6 | 0 | 0 | SM = phi * K6 Contact Number Theorem |
| BT-91 | EXACT | 3 | 0 | 0 | Z2 Topological ECC -- J2 GB Savings Theorem |
| BT-92 | EXACT | 9 | 0 | 0 | Bott Periodicity Active Channels = sopfr |
| BT-93 | EXACT | 8 | 1 | 0 | Carbon Z=6 Chip Material Universality |
| BT-94 | EXACT | 5 | 0 | 0 | CO2   n=6  |
| BT-95 | EXACT | 6 | 0 | 0 | Carbon Cycle  n=6  |
| BT-96 | EXACT | 6 | 0 | 0 | DAC-MOF   |
| BT-97 | EXACT | 3 | 0 | 0 | Weinberg Angle n=6 Bridge — sin^2(theta_W) = (n/phi)/(s |
| BT-98 | EXACT | 6 | 0 | 0 | D-T   = sopfr(6) —      |
| BT-99 | EXACT | 5 | 0 | 0 | Tokamak q=1   —     |
| BT-100 | EXACT | 5 | 1 | 0 | CNO   = sigma + div(6) —    |
| BT-101 | EXACT | 9 | 1 | 0 | -  —  24 = J_2 |
| BT-102 | EXACT | 6 | 0 | 0 |    0.1 = 1/(sigma-phi) — -AI   |
| BT-103 | EXACT | 8 | 0 | 0 |   n=6  |
| BT-104 | EXACT | 8 | 2 | 0 | CO2   n=6  |
| BT-105 | EXACT | 3 | 0 | 0 | SLE_6 Critical Exponent Universality |
| BT-106 | EXACT | 2 | 0 | 0 | S_3 Algebraic Bootstrap |
| BT-107 | EXACT | 3 | 0 | 0 | Ramanujan Tau Divisor Purity |
| BT-108 | EXACT | 4 | 0 | 0 | Music-Audio Consonance Universality |
| BT-109 | EXACT | 3 | 0 | 0 | Zeta-Bernoulli n=6 Trident |
| BT-110 | EXACT | 3 | 0 | 0 | sigma-mu = 11 Dimensional Stack |
| BT-111 | EXACT | 3 | 0 | 0 | tau^2/sigma = 4/3 Solar-AI-Math Trident |
| BT-112 | EXACT | 1 | 0 | 0 | phi^2/n = 2/3 Byzantine-Koide Resonance |
| BT-113 | EXACT | 18 | 0 | 0 | SW Engineering Constant Stack |
| BT-114 | EXACT | 6 | 0 | 0 | Cryptography Parameter Ladder |
| BT-115 | EXACT | 8 | 0 | 0 | OS-Network Layer Count Universality |
| BT-116 | EXACT | 7 | 0 | 0 | ACID-BASE-CAP Database Trinity |
| BT-117 | EXACT | 6 | 0 | 0 | Software-Physics Isomorphism |
| BT-119 | EXACT | 10 | 0 | 0 | Earth 6 Spheres + Troposphere sigma=12km Universality |
| BT-120 | EXACT | 8 | 0 | 0 | Water Treatment pH=6 + CN=6 Catalyst Universality |
| BT-121 | EXACT | 8 | 0 | 0 | 6 Major Plastics + C6 Backbone Universality |
| BT-122 | EXACT | 7 | 0 | 0 | Honeycomb-Snowflake-Coral n=6 Hexagonal Geometry Univer |
| BT-118 | EXACT | 8 | 0 | 0 | Kyoto 6 Greenhouse Gases = n + Carbon Z=6 Universality |
| BT-123 | EXACT | 4 | 0 | 0 | SE(3) dim=n=6 Robot Universality |
| BT-124 | EXACT | 3 | 0 | 0 | phi=2 Bilateral Symmetry + sigma=12 Joint Universality |
| BT-125 | EXACT | 3 | 0 | 0 | tau=4 Locomotion/Flight Minimum Stability Principle |
| BT-126 | EXACT | 3 | 0 | 0 | sopfr=5 Fingers + 2^sopfr=32 Grasp Space Universality |
| BT-127 | EXACT | 3 | 0 | 0 | 3D Kissing Number sigma=12 + Hexacopter n=6 Fault Toler |
| BT-128 | EXACT | 8 | 0 | 0 | Medical Imaging n=6 Parameter Stack |
| BT-129 | EXACT | 6 | 1 | 0 | Civil Engineering n=6 Structural Constants |
| BT-130 | EXACT | 6 | 0 | 0 | Space Orbital Mechanics n=6 Ladder |
| BT-131 | EXACT | 8 | 0 | 0 | Manufacturing Quality n=6 Standard Stack |
| BT-132 | EXACT | 6 | 0 | 0 | Neuroscience Cortical Layer n=6 Universality |
| BT-133 | EXACT | 7 | 0 | 0 | Transportation Infrastructure n=6 Stack |
| BT-134 | EXACT | 6 | 0 | 0 | Periodic Table Period Lengths = n=6 Arithmetic |
| BT-135 | EXACT | 10 | 0 | 0 | Musical Scale n=6 Universality |
| BT-136 | EXACT | 9 | 0 | 0 | Human Anatomy n=6 Structural Constants |
| BT-137 | EXACT | 8 | 0 | 0 | Standard Model Particle Count n=6 Complete Map |
| BT-138 | EXACT | 7 | 0 | 0 | Calendar and Timekeeping n=6 Universality |
| BT-139 | EXACT | 7 | 0 | 0 | Crystallography Space Group n=6 Arithmetic |
| BT-140 | EXACT | 5 | 0 | 0 | TCP/IP Protocol Port n=6 Archaeology |
| BT-141 | EXACT | 5 | 0 | 0 | Amino Acid n=6 Biochemistry |
| BT-142 | EXACT | 5 | 0 | 0 | Semiconductor Memory Hierarchy n=6 |
| BT-143 | EXACT | 3 | 1 | 0 | Cosmological Constant n=6 Ladder |
| BT-144 | EXACT | 8 | 0 | 0 | Chess and Game Theory n=6 Constants |
| BT-145 | EXACT | 6 | 0 | 0 | Electromagnetic Spectrum Band n=6 Partition |
| BT-146 | EXACT | 6 | 0 | 0 | DNA/RNA Molecular Constants n=6 |
| BT-147 | EXACT | 6 | 0 | 0 | Financial Market n=6 Constants |
| BT-148 | EXACT | 9 | 0 | 0 | Olympic and Sports n=6 Structure |
| BT-149 | EXACT | 7 | 0 | 0 | Thermodynamic Laws and Constants n=6 |
| BT-150 | EXACT | 8 | 0 | 0 | Agriculture and Food n=6 Constants |
| BT-151 | EXACT | 7 | 0 | 0 | Graph Theory n=6 Structural Theorems |
| BT-152 | EXACT | 8 | 0 | 0 | Sensory and Perception n=6 Constants |
| BT-153 | EXACT | 6 | 0 | 0 | Electric Vehicle n=6 Architecture |
| BT-154 | EXACT | 8 | 0 | 0 | Map and Geography n=6 Constants |
| BT-155 | EXACT | 8 | 0 | 0 | Immune System n=6 Architecture |
| BT-156 | EXACT | 7 | 0 | 0 | Volcanic and Seismic n=6 Scale Constants |
| BT-157 | EXACT | 7 | 0 | 0 | Color Theory n=6 Framework |
| BT-158 | EXACT | 8 | 0 | 0 | Martial Arts and Combat n=6 Constants |
| BT-159 | EXACT | 7 | 0 | 0 | Cloud Computing n=6 Architecture |
| BT-160 | EXACT | 20 | 0 | 0 | Safety Engineering n=6 Universality |
| BT-161 | EXACT | 9 | 0 | 0 | Solar System Architecture Structural Universality — Row |
| BT-162 | EXACT | 11 | 0 | 0 | Compiler-OS-CPU Architecture Constant Stack |
| BT-163 | EXACT | 10 | 0 | 0 | RL/Alignment Training Parameter Stack — PPO, DPO, GRPO  |
| BT-164 | EXACT | 8 | 0 | 0 | LLM Training Schedule n=6 Universality — LR, Warmup, Co |
| BT-165 | EXACT | 6 | 0 | 0 | SM Gauge Generator Partition σ = (σ-τ) + (n/φ) + μ |
| BT-166 | EXACT | 3 | 0 | 0 | Proton-Electron Mass Ratio = n * pi^5 |
| BT-167 | EXACT | 4 | 0 | 0 | CMB Spectral Index n_s = (n/phi)^3 / ((n/phi)^3 + mu) = |
| BT-168 | EXACT | 5 | 0 | 0 | SU(5) GUT Generator Count = J2 and J2 -> sigma + sigma  |
| BT-169 | EXACT | 3 | 0 | 0 | Neutrino Mixing Angle n=6 Triple |
| BT-170 | EXACT | 7 | 0 | 0 | String/M-Theory Dimension Ladder tau -> n -> sigma-phi  |
| BT-171 | EXACT | 3 | 1 | 0 | SM Coupling Constant n=6 Fraction Pair |
| BT-172 | EXACT | 5 | 0 | 0 | Baryon-to-Photon Ratio eta = n * 10^{-(sigma-phi)} |
| BT-173 | EXACT | 10 | 2 | 0 | Medical Clinical Standards n=6 Convergence — ECG, Nucle |
| BT-174 | EXACT | 10 | 0 | 0 | Space Systems Hardware n=6 Complete Map — GNSS J₂=24 Fo |
| BT-175 | EXACT | 8 | 0 | 0 | Crystallographic Classification n=6 Complete Chain |
| BT-176 | EXACT | 30 | 0 | 0 | Crystal Prototype Unit Cell n=6 Atlas |
| BT-177 | EXACT | 14 | 0 | 0 | Crystal Stacking Periods = div(6) Completeness + FCC Sl |
| BT-178 | EXACT | 9 | 0 | 0 | Digital Media J₂=24 Encoding Universality |
| BT-179 | EXACT | 9 | 0 | 0 | Consensus Protocol n=6 Byzantine Stack |
| BT-180 | EXACT | 10 | 0 | 0 | OS Memory Hierarchy τ=4 Universality + 2^σ Page Law |
| BT-181 | EXACT | 9 | 0 | 0 | Telecommunications n=6 Spectral Stack |
| BT-182 | EXACT | 10 | 0 | 0 | Calendar & Timekeeping n=6 Temporal Stack |
| BT-183 | EXACT | 9 | 0 | 0 | Financial Engineering n=6 Risk Architecture |
| BT-184 | EXACT | 10 | 0 | 0 | Education & Cognitive Science n=6 Learning Stack |
| BT-185 | EXACT | 10 | 0 | 0 | Pharmacology & Clinical Medicine n=6 Drug Stack |
| BT-186 | EXACT | 10 | 0 | 0 | Crystallography & Mineralogy n=6 Crystal Stack |
| BT-187 | EXACT | 9 | 0 | 0 | Control Theory & Automation n=6 Feedback Stack |
| BT-188 | EXACT | 11 | 0 | 0 | Genomics n=6 Information Architecture |
| BT-189 | EXACT | 9 | 0 | 0 | Optics & Photonics n=6 Spectral Stack |
| BT-190 | EXACT | 9 | 0 | 0 | Acoustic Instrument n=6 Resonance Architecture |
| BT-191 | EXACT | 9 | 0 | 0 | Cartography & Geodesy n=6 Coordinate Universality |
| BT-192 | EXACT | 8 | 0 | 0 | Culinary Science & Food Chemistry n=6 Structural Stack |
| BT-193 | EXACT | 10 | 0 | 0 | Classical Thermodynamics n=6 Complete Stack |
| BT-194 | EXACT | 10 | 0 | 0 | Immunology & Immune System n=6 Biological Architecture |
| BT-195 | EXACT | 11 | 0 | 0 | Quantum Computing Hardware n=6 Complete Architecture |
| BT-196 | EXACT | 10 | 0 | 0 | Aviation & Aeronautics n=6 Flight Architecture |
| BT-197 | EXACT | 10 | 0 | 0 | Linguistics & Communication Systems n=6 Information Sta |
| BT-198 | EXACT | 10 | 0 | 0 | Agriculture & Botany n=6 Growth Architecture |
| BT-199 | EXACT | 10 | 0 | 0 | Fluid Dynamics & Turbulence n=6 Complete Architecture |
| BT-200 | EXACT | 10 | 0 | 0 | Game Theory & Social Choice n=6 Decision Architecture |
| BT-201 | EXACT | 10 | 0 | 0 | Classical Mechanics n=6 Phase Space Architecture |
| BT-202 | EXACT | 10 | 0 | 0 | Competitive Sports & Games n=6 Universal Architecture |
| BT-203 | EXACT | 10 | 0 | 0 | Seismology & Geophysics n=6 Earth Dynamics Architecture |
| BT-204 | EXACT | 10 | 0 | 0 | Epidemiology & Public Health n=6 Disease Control Archit |
| BT-205 | EXACT | 7 | 0 | 0 | E₆ Exceptional Lie Algebra n=6 Universality |
| BT-206 | EXACT | 9 | 0 | 0 | Electric Vehicle n=6 Voltage-Connector Stack |
| BT-207 | EXACT | 11 | 0 | 0 | Modular Forms Weight Hierarchy n=6 Purity |
| BT-208 | EXACT | 10 | 0 | 0 | Standard Model Particle Census n=6 Complete Architectur |
| BT-209 | EXACT | 2 | 2 | 0 | Proton-Electron Mass Ratio nπ⁵ Fundamental Bridge |
| BT-210 | EXACT | 10 | 0 | 0 | GNSS J₂=24 Four-Nation Constellation Convergence |
| BT-211 | EXACT | 10 | 0 | 0 | Cybersecurity & Information Security n=6 Defense Archit |
| BT-212 | EXACT | 10 | 0 | 0 | Classical Games & Combinatorial Strategy n=6 Board Arch |
| BT-213 | EXACT | 10 | 0 | 0 | Oceanography & Marine Science n=6 Hydrosphere Architect |
| BT-214 | EXACT | 10 | 0 | 0 | Periodic Table Quantum Shell n=6 Electron Architecture |
| BT-215 | EXACT | 10 | 0 | 0 | Biochemical Pathway n=6 Metabolic Architecture |
| BT-216 | EXACT | 10 | 0 | 0 | Cryptographic Round Count n=6 Complete Architecture |
| BT-217 | EXACT | 10 | 0 | 0 | Color Science & Visual Perception n=6 Chromatic Archite |
| BT-218 | EXACT | 10 | 0 | 0 | Meteorology & Climate Science n=6 Atmospheric Architect |
| BT-219 | EXACT | 10 | 0 | 0 | Formal Language & Computation Theory n=6 Logic Architec |
| BT-220 | EXACT | 10 | 0 | 0 | Protein Structure & Folding n=6 Structural Biology Arch |
| BT-221 | EXACT | 9 | 1 | 0 | Circadian & Sleep Physiology n=6 Chronobiology Architec |
| BT-222 | EXACT | 10 | 0 | 0 | Photography & Imaging Sensor n=6 Optical Capture Archit |
| BT-223 | EXACT | 10 | 0 | 0 | Psychology & Cognitive Science n=6 Mind Architecture |
| BT-224 | EXACT | 10 | 0 | 0 | Human Anatomy & Physiology n=6 Body Architecture |
| BT-225 | EXACT | 10 | 0 | 0 | Ecology & Biodiversity n=6 Life Classification Architec |
| BT-226 | EXACT | 10 | 0 | 0 | Typography & Typesetting n=6 Print Architecture |
| BT-227 | EXACT | 10 | 0 | 0 | Global Identification Code n=6 Encoding Architecture |
| BT-228 | EXACT | 10 | 0 | 0 | International Governance n=6 Institutional Architecture |
| BT-229 | EXACT | 18 | 0 | 0 | Algebraic Blowup–Emergence E₆ Bridge — n=6 Determines S |
| BT-230 | EXACT | 10 | 0 | 0 | Blockchain & Distributed Ledger n=6 Consensus Architect |
| BT-231 | EXACT | 10 | 0 | 0 | Solar System & Celestial Mechanics n=6 Orbital Architec |
| BT-232 | EXACT | 10 | 0 | 0 | Graph Theory & Combinatorial Topology n=6 Structural Ar |
| BT-233 | EXACT | 10 | 0 | 0 | Sexagesimal Time-Angle n=6 Temporal-Spatial Architectur |
| BT-234 | EXACT | 8 | 2 | 0 | Hardy-Ramanujan σ³+μ = 1729 Taxicab–Modular–Computing B |
| BT-235 | EXACT | 9 | 0 | 0 | Icosahedral Capsid–Fullerene–Quasicrystal n=6 Symmetry  |
| BT-236 | EXACT | 10 | 0 | 0 | Quality & Operations Management n=6 Process Architectur |
| BT-237 | EXACT | 8 | 2 | 0 | DNA Double Helix n=6 Structural Geometry Architecture |
| BT-238 | EXACT | 7 | 3 | 0 | Particle Accelerator n=6 Engineering Architecture |
| BT-239 | EXACT | 10 | 0 | 0 | Crystallography & Mineral Science n=6 Lattice Architect |
| BT-240 | EXACT | 10 | 0 | 0 | Combinatorial Design Theory n=6 Steiner Architecture |
| BT-241 | EXACT | 10 | 0 | 0 | Aviation & Aerospace n=6 Flight Architecture |
| BT-242 | EXACT | 8 | 0 | 0 | SLE₆ Percolation–Plasma Transport Topological Equivalen |
| BT-243 | EXACT | 8 | 0 | 0 | Tokamak Topology–Quantum Error Correction n=6 Isomorphi |
| BT-244 | EXACT | 8 | 0 | 0 | ATP Synthase–Tokamak Rotational Energy Conversion n=6 U |
| BT-245 | EXACT | 7 | 0 | 0 | MHD q-Surface = Musical Consonance div(6) Resonance |
| BT-246 | EXACT | 8 | 0 | 0 | Fusion–Carbon Cycle Complete n=6 Loop |
| BT-247 | EXACT | 7 | 0 | 0 | SE(3) Plasma Confinement–Robot Manipulation Duality |
| BT-248 | EXACT | 4 | 0 | 0 | ACID–Tokamak τ=4 Stability Isomorphism |
| BT-249 | EXACT | 6 | 0 | 0 | Disruption = Algebraic Blowup Physical Realization |
| BT-250 | EXACT | 7 | 0 | 0 | Honeycomb–Snowflake–Plasma Crystal n=6 Hexagonal Univer |
| BT-251 | EXACT | 7 | 0 | 0 | Tokamak Remote Maintenance Robot SE(3) n=6 Necessity |
| BT-252 | EXACT | 7 | 0 | 0 | D-T Baryon–Codon Dual Life Code |
| BT-253 | EXACT | 7 | 0 | 0 | Plasma Confinement = Information Security n=6 Parameter |
| BT-254 | EXACT | 10 | 0 | 0 | Cerebral Cortex n=6 Layer Universality — Neocortex = Pe |
| BT-255 | EXACT | 3 | 0 | 0 | Grid Cell Hexagonal = Perfect Number Space Filling — Co |
| BT-256 | EXACT | 5 | 0 | 0 | Sexagesimal 60 = σ·sopfr Universal Time Unit — Temporal |
| BT-257 | EXACT | 4 | 0 | 0 | GPS Orbital Plane n=6 Optimal Configuration — Temporal- |
| BT-258 | EXACT | 4 | 0 | 0 | Six Degrees of Separation = n Social Topology Theorem |
| BT-259 | EXACT | 4 | 0 | 0 | Dunbar σ²+n = 150 Cognitive Limit — Social-Cognitive Br |
| BT-260 | EXACT | 10 | 0 | 0 | Cellular Automata Boolean Emergence Architecture — 2^(σ |
| BT-261 | EXACT | 10 | 0 | 0 | Universal Measurement Scale n=6 Architecture — 200-Year |
| BT-262 | EXACT | 10 | 0 | 0 | 2^n=64 Universal Information Encoding — Braille-Codon-H |
| BT-263 | EXACT | 10 | 0 | 0 | Working Memory τ±μ = 4±1 Cognitive Channel Capacity — M |
| BT-264 | EXACT | 9 | 1 | 0 | Moral Foundation n=6 Universal Ethics — Haidt-Schwartz- |
| BT-265 | EXACT | 9 | 0 | 0 | Circaseptan-Circadian-Circannual τ·(σ-sopfr)·σ Biologic |
| BT-266 | EXACT | 10 | 0 | 0 | Compiler-Cortex Isomorphism — τ=4 Processing Stages Uni |
| BT-267 | EXACT | 8 | 0 | 0 | Hexagonal City Planning n=6 Spatial Optimization — Chri |
| BT-268 | EXACT | 6 | 0 | 0 | Atomic Clock Caesium-133 Hyperfine = 9,192,631,770 Hz — |
| BT-269 | EXACT | 8 | 0 | 0 | Cognitive-Social-Temporal Triple Bridge — Dunbar×Circad |
| BT-270 | EXACT | 8 | 0 | 0 | Multirotor Blade Count Ladder τ→n→(σ-τ) — Rotorcraft St |
| BT-271 | EXACT | 7 | 0 | 0 | Ti-6Al-4V Dual n=6 Aerospace Alloy — World's Most Used  |
| BT-272 | EXACT | 7 | 0 | 0 | Airport Runway Heading n²=36 Compass Division — Aviatio |
| BT-273 | EXACT | 8 | 0 | 0 | Space Crew Size Divisor Cascade — Mercury→Gemini→Apollo |
| BT-274 | EXACT | 8 | 0 | 0 | Aircraft Wing Aspect Ratio n~σ Optimal Band — Aerodynam |
| BT-275 | EXACT | 7 | 0 | 0 | Rocket Stage Count φ~n/φ — Tsiolkovsky Optimal Staging  |
| BT-276 | EXACT | 10 | 0 | 0 | Aerospace n/φ=3 Triple Redundancy Universal Architectur |
| BT-277 | EXACT | 11 | 0 | 0 | Transportation n=6 Universal Architecture — Vehicle Eng |
| BT-278 | EXACT | 10 | 0 | 0 | Railway Signaling & Track n=6 Safety Architecture — Glo |
| BT-279 | EXACT | 10 | 0 | 0 | Maritime IMO Safety & Navigation n=6 Architecture — SOL |
| BT-280 | EXACT | 10 | 0 | 0 | Automotive Safety Rating & Crashworthiness n=6 Architec |
| BT-281 | EXACT | 10 | 0 | 0 | Logistics & Supply Chain n=6 Container-Warehouse Archit |
| BT-282 | EXACT | 10 | 0 | 0 | Surgical Safety & Operating Room n=6 Architecture — WHO |
| BT-283 | EXACT | 10 | 0 | 0 | Neonatal & Critical Care Scoring n=6 Architecture — Apg |
| BT-284 | EXACT | 10 | 0 | 0 | Cardiac & Cardiovascular System n=6 Architecture — ECG  |
| BT-285 | EXACT | 10 | 0 | 0 | WHO Social Determinants & Global Health Framework n=6 A |
| BT-286 | EXACT | 10 | 0 | 0 | Dental & Oral Medicine n=6 Architecture — FDI Tooth Num |
| BT-287 | EXACT | 8 | 0 | 0 | Inline-6 Engine n=6 Perfect Balance Universality — 120- |
| BT-288 | EXACT | 6 | 0 | 0 | Automotive Voltage Ladder 6→12→24→48 — 80-Year phi=2 Do |
| BT-289 | EXACT | 7 | 0 | 0 | Transmission Gear Count n=6 Convergence — 130-Year Mech |
| BT-290 | EXACT | 10 | 0 | 0 | Formula 1 Racing Parameter n=6 Architecture — FIA-Pirel |
| BT-291 | EXACT | 5 | 0 | 0 | D-T Energy Partition = 1/sopfr(6) — Two-Body Kinematics |
| BT-292 | EXACT | 6 | 0 | 0 | Aneutronic Fusion n=6 Complete Map — D-He3 + p-B11 Bary |
| BT-293 | EXACT | 6 | 0 | 0 | Triple-Alpha Carbon Synthesis = (n/phi) x tau = sigma — |
| BT-294 | EXACT | 7 | 0 | 0 | Stellar Nucleosynthesis Ladder P1 -> P2 -> sigma(P2) —  |
| BT-295 | EXACT | 13 | 0 | 0 | Alpha Process Even-Z Selection Rule = phi(6) Multiples  |
| BT-296 | EXACT | 5 | 0 | 0 | D-T-Li6 Fuel Cycle Complete n=6 Closure — Mass Numbers  |
| BT-297 | EXACT | 5 | 0 | 0 | Nuclear Magic Numbers First Five = n=6 Function Ladder  |
| BT-298 | EXACT | 4 | 0 | 0 | Fusion Ignition Triple Product n=6 Encoding — Exponent, |
| BT-299 | EXACT | 8 | 0 | 0 | A15 Superconductor Triple Integer Theorem — Nb=n, Sn=ph |
| BT-300 | EXACT | 9 | 0 | 0 | YBCO Perfect Number Stoichiometry — Y:Ba:Cu = div(6) =  |
| BT-301 | EXACT | 7 | 0 | 0 | MgB2 Dual Atomic Number Theorem — Mg Z=sigma, B Z=sopfr |
| BT-302 | EXACT | 10 | 0 | 0 | ITER Magnet n=6 Architecture — PF=n, CS=n, TF=3n, REBCO |
| BT-303 | EXACT | 9 | 0 | 0 | BCS Analytical Constants Complete Map — sigma, 1/phi, p |
| BT-304 | EXACT | 8 | 0 | 0 | d-wave + BdG Topological Classification — tau, phi, sig |
| BT-305 | EXACT | 9 | 0 | 0 | Elemental + Molecular SC n=6 Atlas — Nb CN=sigma-tau, K |
| BT-306 | EXACT | 8 | 0 | 0 | Superconducting Quantum Device Junction Architecture —  |
| BT-307 | EXACT | 10 | 0 | 0 | CO2 Capture/Utilization Reaction Stoichiometry n=6 Univ |
| BT-308 | EXACT | 8 | 0 | 0 | DAC Thermodynamic n=6 Triple -- Carnot 1/n, Gap sigma-p |
| BT-309 | EXACT | 12 | 0 | 0 | Carbon Allotrope/Material Complete n=6 Structural Encod |
| BT-310 | EXACT | 7 | 0 | 0 | Stellarator Field Period n=6 Family — W7-X=sopfr, LHD=s |
| BT-311 | EXACT | 6 | 0 | 0 | Kruskal-Shafranov Stability Bound q > phi(6) = 2 — Exte |
| BT-312 | EXACT | 7 | 0 | 0 | MHD Instability Quartet tau(6) = 4 — Four Fundamental M |
| BT-313 | EXACT | 5 | 1 | 0 | Tokamak Triangularity Optimal delta = 1/3 = phi/n — Sha |
| BT-314 | EXACT | 6 | 0 | 0 | Plasma Confinement Mode Triad n/phi = 3 — L-H-I Mode Tr |
| BT-315 | EXACT | 7 | 0 | 0 | Plasma Heating Method Quartet tau(6) = 4 — Ohmic+NBI+IC |
| BT-316 | EXACT | 7 | 0 | 0 | Matter Phase Quartet tau(6) = 4 — Solid-Liquid-Gas-Plas |
| BT-317 | EXACT | 9 | 0 | 0 | Tokamak Operational Parameter Complete n=6 Map — q/delt |
| BT-318 | EXACT | 7 | 1 | 0 | Thermal Conductivity Materials Ladder — Cu=(σ-φ)²·τ=400 |
| BT-319 | EXACT | 9 | 0 | 0 | Chip Temperature Boundary Architecture — Tjmax=(σ-φ)^φ, |
| BT-320 | EXACT | 8 | 0 | 0 | Server Rack Power Density Ladder — n->sigma->sigma·tau  |
| BT-321 | EXACT | 8 | 0 | 0 | Thermoelectric Complete n=6 Map — ZT=R(6)=1, Seebeck=(σ |
| BT-322 | EXACT | 8 | 0 | 0 | Water/Air Heat Capacity tau=4 — Cooling Medium Foundati |
| BT-323 | EXACT | 7 | 1 | 0 | PUE Convergence Ladder σ/(σ-μ)->σ/(σ-φ)->R(6) = 1.09->1 |
| BT-324 | EXACT | 8 | 0 | 0 | (σ-φ)^φ = 100 Thermal Boundary Universality |
| BT-325 | EXACT | 8 | 0 | 0 | Thermal-Electrical sigma·tau=48 Dual Convergence — 48V  |
| BT-326 | EXACT | 8 | 0 | 0 | Power Grid Operations Complete n=6 Map — Stability/Mark |
| BT-327 | EXACT | 8 | 0 | 0 | Autonomous Driving Sensor-Compute Complete n=6 Map — SE |
| BT-328 | EXACT | 9 | 1 | 0 | Autonomous Driving tau=4 Subsystem Universality — Wheel |
| BT-329 | EXACT | 20 | 0 | 0 | Programming Language Complete n=6 Map — Type/Paradigm/K |
| BT-330 | EXACT | 6 | 0 | 0 | Quantization Precision Ladder Complete n=6 — FP32→FP16→ |
| BT-331 | EXACT | 8 | 0 | 0 | Speculative Decoding + Inference Acceleration Complete  |
| BT-332 | EXACT | 12 | 0 | 0 | DeepSeek MLA KV Cache Architecture Complete n=6 — Compr |
| BT-333 | EXACT | 10 | 0 | 0 | Post-Transformer Hybrid Architecture n=6 Convergence —  |
| BT-334 | EXACT | 8 | 0 | 0 | AI FLOPs Reduction Technique Stack — MAE/MoD/Egyptian/F |
| BT-335 | EXACT | 14 | 1 | 0 | DeepSeek-V3 Complete n=6 Architecture — 14/15 Parameter |
| BT-336 | EXACT | 10 | 0 | 0 | GQA/MQA/MHA Attention Compression Hierarchy — Head Coun |
| BT-337 | EXACT | 8 | 0 | 0 | Whisper Audio Model Layer Ladder — Complete n=6 Family  |
| BT-338 | EXACT | 10 | 0 | 0 | Financial Temporal-Governance Complete n=6 Map — Fiscal |
| BT-339 | EXACT | 10 | 0 | 0 | Financial Engineering Parameter n=6 Map — Black-Scholes |
| BT-340 | EXACT | 16 | 0 | 0 | Linguistics Complete n=6 Architecture — Phonology/Gramm |
| BT-341 | EXACT | 9 | 0 | 0 | Food Science Complete n=6 Nutrient-Safety-Chemistry Map |
| BT-342 | EXACT | 9 | 0 | 0 | Aviation Engineering Complete n=6 Map — 6-DOF/12km/META |
| BT-343 | EXACT | 9 | 0 | 0 | Oceanography Hydrosphere Complete n=6 Map — 6 Ions/5 Oc |
| BT-344 | EXACT | 8 | 0 | 0 | HEXA-GATE Axis Necessity — tau+phi=n=6 Singularity Brea |
| BT-345 | EXACT | 6 | 0 | 0 | Breakthrough Perturbation Constant 2401 = 7^tau = (sigm |
| BT-346 | EXACT | 5 | 0 | 0 | Orthogonal Filter FP Inverse Bound 288 = sigma·J2 (Fals |
| BT-347 | EXACT | 12 | 2 | 0 | Nuclear Safety Defense-in-Depth Complete n=6 (sopfr=5 B |
| BT-348 | EXACT | 13 | 0 | 0 | SMR Thermal-Load-Waste Triple Convergence σ·τ=48 (Effic |
| BT-349 | EXACT | 13 | 0 | 0 | SMR Modularization-Lifetime-Waste Triple Convergence σ· |
| BT-350 | EXACT | 20 | 0 | 0 | ()  n=6   — /// 4  (19/2 |
| BT-356 | EXACT | 10 | 0 | 0 |  - n=6  —   =  -   (30/30 |
| BT-357 | EXACT | 20 | 0 | 0 |     n=6  — Delta~High-γ 6 + ERP 4 + EEG  |
| BT-358 | EXACT | 10 | 2 | 0 | Alcubierre   n=6  —  1/σ, York  σ,   c |
| BT-359 | EXACT | 10 | 2 | 0 |    n=6  — BT-170   τ→n→σ-φ→σ-μ→J₂→J₂+φ  |
| BT-360 | EXACT | 8 | 2 | 0 | -   n=6  — τ=4  (→→→),  (σ-φ |
| BT-351 | EXACT | 1 | 0 | 0 | / n=6    |
| BT-352 | EXACT | 1 | 0 | 0 | / n=6   |
| BT-353 | EXACT | 1 | 0 | 0 | / n=6  |
| BT-354 | EXACT | 1 | 0 | 0 |   n=6 --  (54/57 EXACT=94.7% ⭐⭐⭐). HBM |
| BT-355 | EXACT | 0 | 1 | 0 |  n=6   |
| BT-361 | EXACT | 1 | 0 | 0 |   n=6  |
| BT-362 | EXACT | 1 | 0 | 0 |  Hexapoda  n=6 |
| BT-363 | EXACT | 1 | 0 | 0 |  - n=6 |
| BT-364 | EXACT | 1 | 0 | 0 |   n=6   |
| BT-365 | EXACT | 1 | 0 | 0 |  n=6  |
| BT-366 | EXACT | 1 | 0 | 0 | / n=6  |
| BT-367 | EXACT | 1 | 0 | 0 | / n=6 - |
| BT-368 | EXACT | 1 | 0 | 0 |   n=6   |
| BT-369 | EXACT | 1 | 0 | 0 |  n=6   |
| BT-370 | EXACT | 1 | 0 | 0 | / n=6  |
| BT-371 | EXACT | 1 | 0 | 0 |    n=6  |
| BT-372 | EXACT | 1 | 0 | 0 |   n=6    |
| BT-373 | EXACT | 1 | 0 | 0 |   n=6   |
| BT-374 | EXACT | 1 | 0 | 0 | / n=6 |
| BT-375 | EXACT | 1 | 0 | 0 |   n=6 |
| BT-376 | EXACT | 1 | 0 | 0 |  n=6 |
| BT-377 | EXACT | 1 | 0 | 0 |   n=6 |
| BT-378 | EXACT | 0 | 1 | 0 |    n=6 |
| BT-379 | EXACT | 1 | 0 | 0 |    n=6 |
| BT-380 | EXACT | 3 | 0 | 0 | -  n=6  |
| BT-381 | EXACT | 0 | 1 | 0 |   n=6   |
| BT-382 | EXACT | 0 | 1 | 0 |  X-bar τ=4  |
| BT-383 | EXACT | 0 | 1 | 0 |  Zipf  n=6  |
| BT-384 | EXACT | 0 | 1 | 0 | 12  = σ²=144 / σ-φ=10  |
| BT-385 | EXACT | 0 | 1 | 0 |   τ=4 / n=6   |
| BT-386 | EXACT | 0 | 1 | 0 |   sopfr  |
| BT-387 | EXACT | 0 | 1 | 0 | Kondratiev  = n·sopfr=30  |
| BT-388 | EXACT | 0 | 1 | 0 | Pareto 80/20 = (σ-φ)²/(σ²+n)  |
| BT-389 | EXACT | 0 | 1 | 0 |  = σ/φ=4 /  1/τ-φ=20% |
| BT-390 | EXACT | 0 | 1 | 0 |   = sopfr(6)+1=6 |
| BT-391 | EXACT | 0 | 1 | 0 |  r/K  = τ/σ-τ  |
| BT-392 | EXACT | 0 | 1 | 0 |  Shannon H' = log(σ-φ)=log(10) |
| BT-393 | EXACT | 0 | 1 | 0 |  n=6  (Brodmann ) |
| BT-394 | EXACT | 0 | 1 | 0 |   Dunbar  = (σ-φ)·σ·sopfr=...150 |
| BT-395 | EXACT | 0 | 1 | 0 |    = τ-φ  |
| BT-396 | EXACT | 0 | 1 | 0 | MHC  ↔ τ-φ=2 /  n=6 |
| BT-397 | EXACT | 0 | 1 | 0 |    = σ-φ²·τ  |
| BT-398 | EXACT | 0 | 1 | 0 |   sopfr  |
| BT-399 | EXACT | 0 | 1 | 0 | 6  n=6   |
| BT-400 | EXACT | 0 | 1 | 0 | 6  : σ·φ=n·τ    |
| BT-401 | EXACT | 12 | 0 | 0 |    - n=6   |
| BT-402 | EXACT | 10 | 0 | 0 |    n=6   |
| BT-403 | EXACT | 11 | 0 | 0 | [[6,4,2]]       —   |
| BT-404 | EXACT | 11 | 0 | 0 | Boltzmann   →   n=6   |
| BT-405 | EXACT | 12 | 0 | 0 | Shannon-Landauer-Boltzmann -  n=6  |
| BT-406 | EXACT | 14 | 0 | 0 | BCS-Josephson-   n=6   — σφ=nτ=24   |
| BT-407 | EXACT | 20 | 0 | 0 | τ=4   — 4  4-  n=6   |
| BT-408 | EXACT | 12 | 0 | 0 |  --  n=6   —   |
| BT-409 | EXACT | 9 | 1 | 0 |    n=6   — /// σ·τ=48  |
| BT-410 | EXACT | 8 | 0 | 0 |  -- n=6   |
| BT-411 | EXACT | 10 | 0 | 0 |   -- n=6   |
| BT-412 | EXACT | 12 | 0 | 0 |    n=6 — Miller 7, Big Five, Piaget τ=4, Erikson |
| BT-413 | EXACT | 12 | 0 | 0 | -   n=6 —  σ=12,  τ=4,  φ=2 |
| BT-414 | EXACT | 6 | 0 | 0 |  -- σ·(σ-φ)=120   |
| BT-415 | EXACT | 8 | 0 | 0 |    n=6  —  σ-τ,  σ,  σ-τ,  σ-sopfr |
| BT-416 | EXACT | 10 | 0 | 0 |   -- n=6   |
| BT-417 | EXACT | 12 | 0 | 0 |  Beaufort σ=12 — -- n=6   |
| BT-418 | EXACT | 10 | 0 | 0 |   n=6  —  σ+n=18,  n=6,  n/φ=3 |
| BT-419 | EXACT | 8 | 0 | 0 |  -  n=6  —  σ-sopfr=7,  sopfr=5,  35‰ |
| BT-420 | EXACT | 8 | 0 | 0 |    n=6 —  n/φ=3,  σ=12, CMYK τ=4,  φ |
| BT-421 | EXACT | 10 | 0 | 0 | -  n=6  —  n=6 ,  σ+φ=14,  σ+s |
| BT-422 | EXACT | 8 | 0 | 0 |  Bloom-ISCED n=6 —   n,   σ-τ=8,  σ=12 |
| BT-423 | EXACT | 8 | 0 | 0 |  n=6 --  |
| BT-424 | EXACT | 8 | 0 | 0 | -  J₂=24  —    |
| BT-425 | EXACT | 8 | 0 | 0 |  -UN n=6  |
| BT-426 | EXACT | 8 | 0 | 0 | - τ=4   —  τ=4, Kohlberg n=6, UDHR 30 |
| BT-427 | EXACT | 8 | 0 | 0 |  Dunbar σ²+n=150 — -- n=6  |
| BT-428 | EXACT | 8 | 0 | 0 | -   n=6 —  τ=4, 3 n/φ=3,  n·sopfr=30 |
| BT-429 | EXACT | 8 | 0 | 0 |  --  n=6   |
| BT-430 | EXACT | 8 | 0 | 0 |  -- n=6   |
| BT-431 | EXACT | 10 | 0 | 0 |  -- n=6  |
| BT-432 | EXACT | 8 | 0 | 0 |  - n=6   |
| BT-433 | EXACT | 8 | 0 | 0 |  -  n=6  |
| BT-434 | EXACT | 10 | 0 | 0 |   --ASCII n=6   |
| BT-435 | EXACT | 8 | 0 | 0 | AES   σ-φ=10 / σ=12   |
| BT-436 | EXACT | 8 | 0 | 0 |   OSI σ-sopfr=7 / TCP τ=4   |
| BT-437 | EXACT | 8 | 0 | 0 |  Shannon-Nyquist φ=2   |
| BT-438 | EXACT | 8 | 0 | 0 |  Bravais σ+φ=14 —  σ-sopfr=7,  φ^sopfr=32,   |
| BT-439 | EXACT | 8 | 0 | 0 |    τ+μ=5 /  τ=4 / Maxwell τ=4   |
| BT-440 | EXACT | 8 | 0 | 0 |  Maxwell τ=4  — 4- τ=4, 4- τ=4 |
| BT-441 | EXACT | 8 | 0 | 0 |   Si  14=σ+φ — Ge 32=φ^sopfr, GaAs 31+33 |
| BT-442 | EXACT | 8 | 0 | 0 | -   n=6 —  24MPa=J₂,  3m=n/φ,  400MPa |
| BT-443 | EXACT | 10 | 0 | 0 | σ-sopfr=7    — //Baltimore/OSI/Miller// |
| BT-444 | EXACT | 10 | 0 | 0 | sopfr=5    — Big Five///// 6  |
| BT-445 | EXACT | 10 | 0 | 0 | φ=2    — /-//φ-Nyquist/Gram  10  |
| BT-446 | EXACT | 2 | 0 | 0 | τ=4     — 30    (BT-407 ) |
| BT-447 | EXACT | 2 | 0 | 0 | σ=12     — //Beaufort////AES-192 7 |
| BT-448 | EXACT | 2 | 0 | 0 | J₂=24  -  — ////Golay/ 6  |
| BT-449 | EXACT | 2 | 0 | 0 | n=6  36    — BT-399   |
| BT-450 | EXACT | 3 | 0 | 0 | σφ=nτ=24     —   |
| BT-451 | EXACT | 4 | 1 | 0 |  (TME)   n=6 |
| BT-452 | EXACT | 4 | 1 | 0 |  τ=4  |
| BT-453 | EXACT | 4 | 1 | 0 | CAR-T  sopfr=5 /  n=6 |
| BT-454 | EXACT | 5 | 0 | 0 | Warburg   σ-φ=10 |
| BT-455 | EXACT | 4 | 1 | 0 |  VEGF  sopfr=5 / VEGFR n/φ=3 |
| BT-456 | EXACT | 4 | 1 | 0 |   n=6  |
| BT-457 | EXACT | 4 | 1 | 0 |    n/φ=3 /  τ=4 |
| BT-458 | EXACT | 4 | 1 | 0 |  DNA  τ=4 /  σ=12 |
| BT-459 | EXACT | 4 | 1 | 0 | ADC  σ-sopfr=7 /  n/φ=3 |
| BT-460 | EXACT | 4 | 2 | 0 |   n=6 |
| BT-451 | EXACT | 0 | 1 | 0 | ~460  |
| BT-461 | EXACT | 4 | 0 | 0 | gp120-CD4  6 |
| BT-462 | EXACT | 4 | 0 | 0 | (RT) τ=4   |
| BT-463 | PARTIAL | 4 | 0 | 1 |  LTR  σ(6)=12 /  6bp  |
| BT-464 | PARTIAL | 4 | 0 | 1 | Tat-TAR 6   |
| BT-465 | PARTIAL | 4 | 0 | 1 | Rev-RRE  4→6  |
| BT-466 | PARTIAL | 4 | 0 | 1 | HIV  C₂  /  φ=2 |
| BT-467 | PARTIAL | 4 | 0 | 1 | HIV   6 |
| BT-468 | PARTIAL | 4 | 0 | 1 | (bNAb) σ(6)=1+2+3+6   |
| BT-469 | PARTIAL | 4 | 0 | 1 | CCR5  6nt PAM / CRISPR   |
| BT-470 | PARTIAL | 4 | 0 | 1 | HEXA-ART — HIV 6  n=6   |
| BT-461 | EXACT | 0 | 1 | 0 | ~470  |
| BT-471 | PARTIAL | 4 | 0 | 1 |   6 = n — PDG   |
| BT-472 | PARTIAL | 3 | 0 | 1 |   σ-sopfr=7 — +  |
| BT-473 | EXACT | 3 | 0 | 0 | CKM  τ=4   |
| BT-474 | PARTIAL | 3 | 0 | 1 |  3  = n/φ |
| BT-475 | MISS | 2 | 0 | 3 | Planck 2018 Ω  n=6  |
| BT-476 | MISS | 1 | 0 | 3 | CMB   —   ℓ ≈ 220 = σ(6)·...  |
| BT-477 | PARTIAL | 3 | 0 | 1 |    τ=3 — M,Q,J   |
| BT-478 | MISS | 3 | 0 | 2 |    τ=4   |
| BT-479 | EXACT | 4 | 0 | 0 |   n/φ=3 — // |
| BT-480 | EXACT | 4 | 0 | 0 | Pauli  n/φ=3 / Dirac  τ=4 |
| BT-481 | PARTIAL | 3 | 0 | 1 | BCS   2Δ/k_BT_c ≈ 3.53 → n/φ  |
| BT-482 | PARTIAL | 3 | 0 | 1 |   σ_xy=νe²/h —  n=6 plateau  |
| BT-483 | MISS | 3 | 0 | 2 | CODATA   n=6   |
| BT-484 | PARTIAL | 4 | 0 | 1 | Bell  CHSH  2 = φ —   2√2 |
| BT-485 | EXACT | 5 | 0 | 0 |  Bloch   φ=2 —   |
| BT-486 | EXACT | 5 | 0 | 0 |   [[7,1,3]] Steane / Shor [[9,1,3]] |
| BT-487 | MISS | 2 | 0 | 3 |    13.8 Gyr / Hubble  τ_H — n=6  |
| BT-471 | EXACT | 0 | 1 | 0 | ~487  (17 ) |
| BT-488 | PARTIAL | 4 | 0 | 1 |      = φ=2 —   |
| BT-489 | EXACT | 4 | 0 | 0 | - 5 → 4+1  σ=n —   |
| BT-490 | EXACT | 5 | 0 | 0 | -    = φ=2 — RS1  |
| BT-491 | PARTIAL | 4 | 0 | 1 | ADD     n=6 — Arkani-Hamed/Dimopoulos/Dvali |
| BT-492 | EXACT | 4 | 0 | 0 |  Morris-Thorne    = n=6 |
| BT-493 | PARTIAL | 4 | 0 | 1 | M- 11 → σ-sopfr=7   |
| BT-494 | PARTIAL | 4 | 0 | 1 |  critical dimension 26/10  = φ=2 |
| BT-495 | EXACT | 5 | 0 | 0 |    = n/φ=3 — Kerr-Newman |
| BT-496 | EXACT | 4 | 0 | 0 | Casimir   = φ=2 —     |
| BT-497 | EXACT | 5 | 0 | 0 | Gödel      = n=6 |
| BT-488 | EXACT | 0 | 1 | 0 | ~497  (10  — / ) |
| BT-498 | PARTIAL | 4 | 0 | 2 | (RO)  6  = n |
| BT-499 | PARTIAL | 4 | 0 | 1 | (MED/MSF)  σ=12  |
| BT-500 | PARTIAL | 5 | 0 | 1 | 3D  6  = n — FDM/SLA/SLS  |
| BT-501 | PARTIAL | 4 | 0 | 1 | 3D    —   n=6  |
| BT-502 | PARTIAL | 4 | 0 | 2 |  6  = n — /  |
| BT-503 | MISS | 3 | 1 | 2 |  6  — Brayton  τ=4 |
| BT-504 | PARTIAL | 4 | 0 | 1 |  12 HP + 24 LP — σ/J₂  |
| BT-505 | PARTIAL | 4 | 0 | 1 |  6  = n —   |
| BT-506 | PARTIAL | 4 | 0 | 1 | HACCP  σ-sopfr=7  |
| BT-507 | PARTIAL | 4 | 0 | 1 |    6  = n |
| BT-508 | PARTIAL | 5 | 0 | 1 | CSS Box Model τ=4  —    |
| BT-509 | PARTIAL | 4 | 1 | 1 |   σ=12  — WDM  |
| BT-510 | PARTIAL | 4 | 0 | 1 |  SiO₂ —  4 CN=τ /  n=6  |
| BT-511 | PARTIAL | 4 | 0 | 1 |   — /  n=6  |
| BT-512 | PARTIAL | 4 | 0 | 1 |  6  = n — FAA  |
| BT-513 | PARTIAL | 4 | 0 | 1 |   1435 mm — σ-sopfr=7 × 205  |
| BT-514 | PARTIAL | 4 | 0 | 1 |  6 GMP — ICH Q7  |
| BT-515 | PARTIAL | 4 | 0 | 1 |   — 300 mm = σ·(J₂+μ)   |
| BT-516 | PARTIAL | 5 | 0 | 1 | NAND  — 2^n=64   / n/φ=3 bit TLC |
| BT-517 | PARTIAL | 4 | 0 | 1 |  — 6 / Ekman  /  σ=12 |
| BT-518 | PARTIAL | 4 | 0 | 1 |  — 6  /  n=6  |
| BT-519 | PARTIAL | 4 | 0 | 1 |  6 — /   φ=2 |
| BT-520 | PARTIAL | 4 | 0 | 1 | DNA  — 6  /  64=φⁿ |
| BT-521 | PARTIAL | 4 | 0 | 1 |   — 6   = n |
| BT-522 | PARTIAL | 5 | 0 | 1 |  — 6  /  σ-μ=11 |
| BT-523 | PARTIAL | 4 | 0 | 1 | -  — 12 σ = AES-128  (σ-sopfr) |
| BT-524 | PARTIAL | 4 | 0 | 1 |  — VEI 8 /   n=6 |
| BT-525 | PARTIAL | 4 | 0 | 1 |  —  6 / 28  P₂ |
| BT-526 | PARTIAL | 4 | 0 | 1 |  — Fe-C  /  FCC CN=σ=12 |
| BT-527 | PARTIAL | 5 | 0 | 1 |  — 6  /  σ-τ=8 |
| BT-528 | PARTIAL | 4 | 0 | 1 | -  — 12TET = σ, A440 = σ·sopfr·(σ-sopfr)+σ·φ |
| BT-529 | PARTIAL | 4 | 0 | 1 | -  — 6  /  n=6  |
| BT-530 | PARTIAL | 4 | 0 | 1 | GPS--   — σ=12   |
| BT-498 | EXACT | 0 | 1 | 0 | ~530  (33  — ///) |
| BT-531 | EXACT | 4 | 0 | 0 | SPARC   B₀ ≈ 12 T = σ —     |
| BT-532 | EXACT | 6 | 0 | 0 | TF   σ+n = 18 —     |
| BT-533 | EXACT | 3 | 0 | 0 | D-T    E_n/E_α = τ = 4 —    |
| BT-534 | EXACT | 5 | 0 | 0 | Lawson  q ≥ n/φ = 3 —  MHD   |
| BT-535 | PARTIAL | 2 | 0 | 1 |   A = n/φ = 3 → φ = 2 —     |
| BT-536 | PARTIAL | 4 | 0 | 1 | DC  —   1 = σ·sopfr = 60 MWe, σ·n = 72  |
| BT-537 | EXACT | 4 | 0 | 0 | Tritium Breeding — Li-6 , TBR = 1+1/n = 7/6 |
| BT-538 | EXACT | 5 | 0 | 0 | Divertor  — τ=4  , σ-φ = 10 MW/m² |
| BT-539 | PARTIAL | 5 | 0 | 1 | REBCO   — σ T , τ , J₂-τ K  |
| BT-540 | EXACT | 6 | 0 | 0 |     — σ 720 MWe → Mk.V J₂ 1.44 GWe |
| BT-531 | EXACT | 0 | 1 | 0 | ~540  (10  —   ) |
