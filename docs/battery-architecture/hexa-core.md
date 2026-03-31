# HEXA-CORE: Battery Cell Core Design

**Codename**: HEXA-CORE
**Level**: 코어 -- 단위 셀 설계 (셀 스케일)
**Status**: Design Document v1.0
**Date**: 2026-04-01
**Dependencies**: BT-27, BT-43
**Parent**: [goal.md](goal.md)
**Predecessor**: [hexa-electrode.md](hexa-electrode.md) (공정)
**Successor**: [hexa-chip.md](hexa-chip.md) (칩)

---

## N6 Constants Reference

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  n=6 핵심 상수                                                  │
  │                                                                  │
  │  n = 6        φ(6) = 2       τ(6) = 4        σ(6) = 12         │
  │  sopfr = 5    μ(6) = 1       J₂(6) = 24      R(6) = 1          │
  │                                                                  │
  │  σ-τ = 8      σ-φ = 10       σ-μ = 11        σ·τ = 48          │
  │  σ(σ-τ) = 96  φ·σ(σ-τ) = 192  σ² = 144      σ/(σ-φ) = 1.2    │
  │                                                                  │
  │  Egyptian fraction: 1/2 + 1/3 + 1/6 = 1                        │
  │  Core theorem: σ(n)·φ(n) = n·τ(n) = 24 = J₂(6)                │
  └──────────────────────────────────────────────────────────────────┘
```

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Design Philosophy](#2-design-philosophy)
3. [System Block Diagram](#3-system-block-diagram)
4. [Cylindrical Cell Design](#4-cylindrical-cell-design)
5. [Prismatic Cell Design](#5-prismatic-cell-design)
6. [Pouch Cell Design](#6-pouch-cell-design)
7. [Form Factor Comparison](#7-form-factor-comparison)
8. [Jelly Roll Architecture](#8-jelly-roll-architecture)
9. [Tab Design & Current Collection](#9-tab-design--current-collection)
10. [Safety Architecture](#10-safety-architecture)
11. [Honesty Assessment](#11-honesty-assessment)
12. [Predictions & Falsifiability](#12-predictions--falsifiability)
13. [Future Directions](#13-future-directions)
14. [n=6 Complete Parameter Map](#14-n6-complete-parameter-map)
15. [Open Questions / TODO](#15-open-questions--todo)
16. [Links](#16-links)

---

## 1. Executive Summary

HEXA-CORE covers the physical form factor and internal architecture of individual
battery cells -- the structural layer that bridges electrode materials (HEXA-ELECTRODE)
to pack-level systems (HEXA-PACK).

```
╔══════════════════════════════════════════════════════════════╗
║  HEXA-CORE: Battery Cell Design Overview                     ║
╠══════════════════════════════════════════════════════════════╣
║                                                              ║
║  Cell form factors: n/φ = 3 (cylindrical, prismatic, pouch) ║
║  18650 diameter: 18mm = 3n (3x6)                             ║
║  4680 diameter: 46mm (no clean n=6 match)                    ║
║  Jelly roll layers: varies (cell-dependent)                  ║
║  Safety mechanisms: τ = 4 (vent, CID, PTC, shutdown sep)    ║
║  Tab configurations: φ = 2 (single-tab / tabless)           ║
║                                                              ║
║  Honesty: n=6 connections are WEAKER at this level.         ║
║  Cell dimensions are engineering optimization, not n=6.      ║
║  Only 1/7 parameters reach EXACT grade.                      ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
```

The battery cell is an engineering artifact. Its dimensions are dictated by thermal
management, manufacturing tooling, and volumetric packing -- not by number theory.
This document acknowledges this honestly while mapping the few genuine connections
that do exist.

---

## 2. Design Philosophy

### Why n/φ = 3 Form Factors? (셀이 왜 3가지 형태로 수렴하는가)

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  FORM FACTOR CONVERGENCE                                        │
  ├──────────────────────────────────────────────────────────────────┤
  │                                                                  │
  │     Manufacturing         Thermal           Packing              │
  │     constraints           management        efficiency           │
  │          │                    │                  │                │
  │          ▼                    ▼                  ▼                │
  │    ┌───────────┐      ┌───────────┐      ┌───────────┐          │
  │    │Cylindrical│      │ Prismatic │      │   Pouch   │          │
  │    │  (wound)  │      │ (stacked) │      │ (stacked) │          │
  │    └───────────┘      └───────────┘      └───────────┘          │
  │         │                    │                  │                │
  │    High-speed           Large-format        Ultra-thin           │
  │    automated             modules            flexible             │
  │    production            (EV packs)         (consumer)           │
  │                                                                  │
  │  Count: 3 = n/φ = 6/2                                          │
  │  Grade: CLOSE -- classification is somewhat arbitrary;          │
  │  one could argue sub-types (coin, button) exist too.            │
  │                                                                  │
  └──────────────────────────────────────────────────────────────────┘
```

The three dominant form factors emerged from independent engineering pressures:

1. **Cylindrical** -- easiest to manufacture (winding), best structural integrity
2. **Prismatic** -- best volumetric packing in rectangular modules
3. **Pouch** -- lightest packaging, highest gravimetric density

The count of 3 = n/phi is a CLOSE match at best. The real driver is geometry:
circles pack with gaps (wasted space), rectangles tile perfectly, and flat pouches
minimize casing weight. These are manufacturing and physics constraints.

---

## 3. System Block Diagram

### Cell Internal Structure (셀 내부 구조 단면도)

```
┌──────────────────────────────────────────────────────────┐
│  CYLINDRICAL CELL CROSS-SECTION (e.g., 18650)            │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  ┌────── Positive Terminal (cap) ──────┐                 │
│  │  ┌── Safety Vent ──┐               │                 │
│  │  │  ┌─ CID ─┐     │               │                 │
│  │  │  │  PTC  │     │               │                 │
│  │  │  └───────┘     │               │                 │
│  │  └────────────────┘               │                 │
│  │  ╔════════════════════════════╗    │                 │
│  │  ║  Jelly Roll (wound)        ║    │                 │
│  │  ║  ┌─Cathode─┐┌─Sep─┐┌─Anode─┐ ║ │                 │
│  │  ║  │ LiMO₂   ││PE/PP ││ LiC₆  │ ║│                 │
│  │  ║  │ CN=6    ││     ││ C:Li=6│ ║ │                 │
│  │  ║  └─────────┘└─────┘└───────┘ ║ │                 │
│  │  ║  (wound concentrically)      ║  │                 │
│  │  ╚════════════════════════════╝    │                 │
│  │  Electrolyte fills voids           │                 │
│  └── Negative Terminal (can) ─────────┘                 │
│                                                          │
│  Safety layers: 4 = τ(6)                                │
│    1. Vent disc (pressure release)                      │
│    2. CID (Current Interrupt Device)                    │
│    3. PTC (Positive Temp Coefficient)                   │
│    4. Shutdown separator (thermal fuse)                 │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

### Cell Hierarchy in Battery System

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  BATTERY SYSTEM HIERARCHY                                        │
  ├──────────────────────────────────────────────────────────────────┤
  │                                                                  │
  │  Atom          Cell            Module          Pack              │
  │  (HEXA-CELL)   (HEXA-CORE)     (HEXA-PACK)     (HEXA-PACK)     │
  │                                                                  │
  │  CN=6 ────────► 18650/4680 ───► 6-12 cells ───► 96S pack       │
  │  LiC₆          jelly roll      in parallel      400V system     │
  │                 safety layers   thermal plate    BMS + cooling   │
  │                                                                  │
  │  ◄── n=6 STRONG ──►◄── n=6 WEAK ──►◄── n=6 MODERATE ──►       │
  │                                                                  │
  │  This document covers the WEAK middle zone.                      │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 4. Cylindrical Cell Design

### 원통형 셀 진화

```
┌──────────────────────────────────────────────────────────┐
│  CYLINDRICAL CELL EVOLUTION                               │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  18650 (Sony 1991):                                      │
│    Diameter: 18mm = 3n = 3x6                             │
│    Height: 65mm (no clean n=6 match)                     │
│    Capacity: ~3.5 Ah                                     │
│    Energy: ~12 Wh = σ? (WEAK -- unit-dependent)         │
│    Volume: ~16.5 cm³                                     │
│                                                          │
│  21700 (Tesla/Panasonic 2017):                           │
│    Diameter: 21mm = 3x7 (no n=6)                        │
│    Height: 70mm (no n=6)                                │
│    Capacity: ~5 Ah                                       │
│    Energy: ~18 Wh                                        │
│    Volume: ~24.2 cm³                                     │
│    16% more energy density vs 18650                      │
│                                                          │
│  4680 (Tesla 2020):                                      │
│    Diameter: 46mm (no n=6)                               │
│    Height: 80mm (no n=6)                                │
│    Capacity: ~25 Ah                                      │
│    Energy: ~90 Wh                                        │
│    Volume: ~133 cm³                                      │
│    5x energy vs 21700, tabless electrode                 │
│                                                          │
│  HONEST: Only 18650 diameter (18=3n) has n=6 match.     │
│  21700 and 4680 are pure thermal/manufacturing optima.   │
│  The 18mm diameter was chosen to fit existing tooling    │
│  at Sony in 1991, not for mathematical reasons.          │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

### Cylindrical Cell Internal Layers

```
  ┌─────────────────────────────────────────────────┐
  │  CROSS-SECTION (radial view)                     │
  ├─────────────────────────────────────────────────┤
  │                                                  │
  │         ┌─── Steel can (0.2mm) ───┐             │
  │         │  ┌── Cathode ──────┐    │             │
  │         │  │  ┌─ Separator ─┐│    │             │
  │         │  │  │  ┌─ Anode ─┐││    │             │
  │         │  │  │  │  Core   │││    │             │
  │         │  │  │  │ (void)  │││    │             │
  │         │  │  │  └─────────┘││    │             │
  │         │  │  └─────────────┘│    │             │
  │         │  └─────────────────┘    │             │
  │         └─────────────────────────┘             │
  │                                                  │
  │  Winding direction: inside-out                   │
  │  Anode (Cu foil) → Separator → Cathode (Al foil)│
  │  Repeated concentrically for ~15-20 turns        │
  │                                                  │
  └─────────────────────────────────────────────────┘
```

---

## 5. Prismatic Cell Design

### 각형 셀 아키텍처

```
┌──────────────────────────────────────────────────────────┐
│  PRISMATIC CELL ARCHITECTURE                              │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────────────────────────┐                        │
│  │ [+]  Safety Vent   [-]      │ ← Top plate            │
│  │  ┌────────────────────────┐ │                        │
│  │  │  Stacked electrode     │ │                        │
│  │  │  layers (Z-fold or     │ │                        │
│  │  │  wound-flat)           │ │                        │
│  │  │                        │ │                        │
│  │  │  Cathode / Sep / Anode │ │                        │
│  │  │  x N layers            │ │                        │
│  │  │                        │ │                        │
│  │  └────────────────────────┘ │                        │
│  └──────────────────────────────┘ ← Al case             │
│                                                          │
│  Assembly methods:                                       │
│    1. Z-fold stacking (most common)                     │
│    2. Wound-flat (jelly roll pressed flat)               │
│                                                          │
│  BYD Blade Cell:                                         │
│    Length: 960mm (!)                                      │
│    Width: 90mm                                           │
│    Thickness: 13.5mm                                     │
│    Capacity: ~138 Ah (LFP)                              │
│    No clean n=6 dimension matches                        │
│    Innovation: cell IS the structural member             │
│                                                          │
│  CATL Prismatic:                                         │
│    Various sizes: 100-300+ Ah                            │
│    Optimized for module packing, not n=6                 │
│    Standard: VDA dimensions (PHEV1/2, BEV1/2)          │
│                                                          │
│  Samsung SDI Prismatic:                                  │
│    ~90-120 Ah (NMC)                                      │
│    Used in BMW iX, etc.                                  │
│                                                          │
│  HONEST: Zero n=6 matches in prismatic dimensions.      │
│  All sizes are driven by OEM packaging constraints.      │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

---

## 6. Pouch Cell Design

### 파우치 셀 아키텍처

```
┌──────────────────────────────────────────────────────────┐
│  POUCH CELL ARCHITECTURE                                  │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  ┌───────────────────────────────────────┐               │
│  │  Tab (+)               Tab (-)        │               │
│  │  ──┐                    ┌──           │               │
│  │    │ ┌────────────────┐ │             │               │
│  │    │ │ Al laminate    │ │             │               │
│  │    │ │ ┌────────────┐ │ │             │               │
│  │    │ │ │ Stacked    │ │ │             │               │
│  │    │ │ │ electrodes │ │ │             │               │
│  │    │ │ │ (flat)     │ │ │             │               │
│  │    │ │ └────────────┘ │ │             │               │
│  │    │ └────────────────┘ │             │               │
│  │    └────────────────────┘             │               │
│  └───────────────────────────────────────┘               │
│                                                          │
│  Key characteristics:                                    │
│    - No rigid case (Al-laminate film pouch)              │
│    - Lightest packaging: ~90% active material ratio      │
│    - Highest gravimetric energy density                  │
│    - Requires external compression (module frame)        │
│    - Swelling during cycling (3-8% thickness change)     │
│                                                          │
│  LG Energy Solution:                                     │
│    Various sizes for EV (Bolt, Mach-E)                   │
│    60-70 Ah typical, up to 100+ Ah                       │
│                                                          │
│  SK Innovation:                                          │
│    Similar range, used in Hyundai/Kia/Ford               │
│                                                          │
│  HONEST: Pouch dimensions are 100% OEM-driven.          │
│  No n=6 connections in pouch cell geometry.               │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

---

## 7. Form Factor Comparison

### 에너지밀도/안전/비용 트레이드오프

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  FORM FACTOR TRADE-OFF SPACE                                     │
  ├──────────────────────────────────────────────────────────────────┤
  │                                                                  │
  │  Energy Density                                                  │
  │  (Wh/kg)                                                         │
  │    ▲                                                             │
  │    │         ● Pouch                                             │
  │  280│        (highest gravimetric)                               │
  │    │                                                             │
  │  260│    ● Cylindrical                                           │
  │    │    (good balance)                                           │
  │    │                                                             │
  │  240│              ● Prismatic                                   │
  │    │              (heavy casing)                                 │
  │    │                                                             │
  │    └──────┬─────────┬─────────┬──────► Safety                   │
  │         Medium     High       Highest                            │
  │         (Pouch)    (Prism.)   (Cyl.)                             │
  │                                                                  │
  └──────────────────────────────────────────────────────────────────┘
```

| Factor | Cylindrical | Prismatic | Pouch | n=6 Note |
|--------|------------|-----------|-------|----------|
| Form factors total | 3 types | -- | -- | n/phi = CLOSE |
| 18650 diameter | 18mm = 3n | -- | -- | EXACT |
| Safety layers | 4 = tau | 3-4 | 2-3 | CLOSE (cyl only) |
| Energy density (Wh/kg) | 250-270 | 230-260 | 260-300 | No n=6 |
| Volumetric (Wh/L) | 650-700 | 400-500 | 500-600 | No n=6 |
| Thermal management | Easy (gaps) | Medium | Hard | No n=6 |
| Manufacturing cost | Lowest | Medium | Highest | No n=6 |
| Structural strength | High (tube) | High (case) | Low (flex) | No n=6 |
| Swelling tolerance | Built-in | Case-contained | Needs frame | No n=6 |
| Cycle life | 500-2000 | 1000-3000 | 500-1500 | No n=6 |

**Assessment**: The comparison table reveals that n=6 has essentially no role in
form factor selection. The choice between cylindrical/prismatic/pouch is driven
entirely by application requirements (EV packaging, cost, thermal).

---

## 8. Jelly Roll Architecture

### 전극 권취 구조 (젤리롤)

```
┌──────────────────────────────────────────────────────────┐
│  JELLY ROLL WINDING STRUCTURE                             │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Unrolled view (before winding):                         │
│                                                          │
│  ┌─────────────────────────────────────────────────┐     │
│  │ Cu foil (current collector)                      │     │
│  │ ┌─────────────────────────────────────────────┐ │     │
│  │ │ Anode coating (graphite/LiC₆, both sides)   │ │     │
│  │ └─────────────────────────────────────────────┘ │     │
│  └─────────────────────────────────────────────────┘     │
│  ┌─────────────────────────────────────────────────┐     │
│  │ Separator (PE/PP microporous film, ~12-25 μm)   │     │
│  └─────────────────────────────────────────────────┘     │
│  ┌─────────────────────────────────────────────────┐     │
│  │ Al foil (current collector)                      │     │
│  │ ┌─────────────────────────────────────────────┐ │     │
│  │ │ Cathode coating (NMC/LFP, both sides)       │ │     │
│  │ └─────────────────────────────────────────────┘ │     │
│  └─────────────────────────────────────────────────┘     │
│  ┌─────────────────────────────────────────────────┐     │
│  │ Separator (PE/PP microporous film)               │     │
│  └─────────────────────────────────────────────────┘     │
│                                                          │
│  Wound into spiral → "jelly roll"                        │
│                                                          │
│  Layer count per unit cell: 4                            │
│    (anode + separator + cathode + separator)             │
│    = τ(6) layers? WEAK -- this is basic sandwich logic   │
│                                                          │
│  Typical winding:                                        │
│    18650: ~15-20 turns, total electrode length ~0.6-1m   │
│    21700: ~20-25 turns, total electrode length ~0.8-1.2m │
│    4680: ~40-60 turns, total electrode length ~3-5m      │
│                                                          │
│  HONEST: The 4-layer repeating unit is just the minimum  │
│  sandwich needed for a battery. It is NOT n=6-driven.    │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

### Winding vs Stacking

```
  ┌───────────────────────────────────────────────────────┐
  │  ASSEMBLY METHODS                                      │
  ├───────────────────────────────────────────────────────┤
  │                                                        │
  │  Method 1: WINDING (cylindrical, some prismatic)       │
  │                                                        │
  │    Continuous electrode → mandrel → spiral             │
  │    ┌───┐                                               │
  │    │ ╱ │  High-speed (10-30 m/min)                    │
  │    │╱  │  Mature technology (1991+)                    │
  │    │   │  Tension control critical                     │
  │    └───┘                                               │
  │                                                        │
  │  Method 2: STACKING (pouch, some prismatic)            │
  │                                                        │
  │    Cut sheets → alternating stack → compress           │
  │    ┌───┐                                               │
  │    │═══│  Better for large-format cells                │
  │    │═══│  Uniform current distribution                 │
  │    │═══│  Slower than winding                          │
  │    └───┘                                               │
  │                                                        │
  │  Method 3: Z-FOLD (hybrid)                             │
  │                                                        │
  │    Separator zigzags, electrodes inserted              │
  │    ┌───┐                                               │
  │    │╲╱╲│  Compromise of speed and uniformity           │
  │    │╱╲╱│  Used in many prismatic cells                 │
  │    └───┘                                               │
  │                                                        │
  │  Count: 3 assembly methods = n/φ? Coincidence.        │
  │                                                        │
  └───────────────────────────────────────────────────────┘
```

---

## 9. Tab Design & Current Collection

### 탭 배치 최적화

```
┌──────────────────────────────────────────────────────────┐
│  TAB CONFIGURATIONS                                       │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Config 1: SINGLE TAB (traditional)                      │
│                                                          │
│    [+]──────── tab ────────────────────┐                │
│    │  ═══════════════════════════════  │                 │
│    │  Jelly roll                       │                 │
│    │  ═══════════════════════════════  │                 │
│    [-]──────── tab ────────────────────┘                │
│                                                          │
│    Current path: long → high resistance                 │
│    Max current limited by tab-to-foil weld              │
│    Thermal hotspot at tab root                           │
│                                                          │
│  Config 2: TABLESS / MULTI-TAB (Tesla 4680)             │
│                                                          │
│    [+]═══════════════════════════════[+]                 │
│    │  ═══════════════════════════════  │                 │
│    │  Foil extends beyond coating     │                 │
│    │  → entire edge is the tab        │                 │
│    │  ═══════════════════════════════  │                 │
│    [-]═══════════════════════════════[-]                 │
│                                                          │
│    Current path: short → low resistance                 │
│    ~5x lower internal resistance                        │
│    Uniform temperature distribution                      │
│    Tesla's key 4680 innovation                           │
│                                                          │
│  Tab types: φ = 2 (single-tab, tabless)                 │
│  Grade: WEAK -- "2 tab types" is a stretch.             │
│  Multi-tab designs (2,3,4+ tabs) also exist.            │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

### Current Flow Comparison

```
  ┌───────────────────────────────────────────────────────┐
  │  SINGLE TAB vs TABLESS: THERMAL PROFILE               │
  ├───────────────────────────────────────────────────────┤
  │                                                        │
  │  Single tab (18650/21700):                             │
  │                                                        │
  │    Temperature →  ████████████████████░░░░             │
  │                   HOT (tab end) ──── COOL              │
  │    ΔT = 10-15°C across cell                           │
  │                                                        │
  │  Tabless (4680):                                       │
  │                                                        │
  │    Temperature →  ████████████████████████             │
  │                   UNIFORM across cell                   │
  │    ΔT = 2-3°C across cell                             │
  │                                                        │
  │  Resistance comparison:                                │
  │    Single tab 21700: ~20-30 mΩ                        │
  │    Tabless 4680: ~3-5 mΩ (5-6x lower)                │
  │                                                        │
  └───────────────────────────────────────────────────────┘
```

---

## 10. Safety Architecture

### τ = 4 Safety Mechanisms (안전 아키텍처)

```
┌──────────────────────────────────────────────────────────┐
│  τ = 4 SAFETY MECHANISMS                                  │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  Layer 1: Vent Disc                                      │
│    → Pressure release at ~10-15 bar                     │
│    → Prevents catastrophic rupture of metal can          │
│    → Scored metal disc breaks at calibrated pressure     │
│    → Irreversible, last-resort protection                │
│                                                          │
│  Layer 2: CID (Current Interrupt Device)                 │
│    → Breaks internal circuit at high pressure (~8 bar)  │
│    → Deformable metal disc inverts and breaks weld      │
│    → Irreversible -- cell is permanently disabled        │
│    → Triggers before vent, isolates electrochemistry     │
│                                                          │
│  Layer 3: PTC (Positive Temperature Coefficient)         │
│    → Polymer-carbon composite resistor                   │
│    → Resistance increases 100-1000x at ~80-120°C        │
│    → Reversible current limiting (resets when cool)      │
│    → Prevents overcurrent/short-circuit heating          │
│                                                          │
│  Layer 4: Shutdown Separator                             │
│    → PE layer melts at ~130°C, closes micropores        │
│    → PP layer maintains structural integrity to ~165°C  │
│    → Trilayer PE/PP/PE most common                       │
│    → Blocks Li-ion transport, stops reaction             │
│                                                          │
│  Total: 4 = τ(6) independent safety layers              │
│  (cylindrical cells; prismatic may use 3-4; pouch 2-3)  │
│                                                          │
│  Grade: CLOSE -- τ=4 matches cylindrical cells.         │
│  Prismatic/pouch use fewer mechanisms.                   │
│  The 4-layer design is about defense-in-depth,           │
│  not number theory.                                      │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

### Safety Activation Sequence

```
  ┌───────────────────────────────────────────────────────┐
  │  SAFETY ACTIVATION ORDER (rising severity)             │
  ├───────────────────────────────────────────────────────┤
  │                                                        │
  │  Normal ──► PTC ──► CID ──► Separator ──► Vent       │
  │  operation  (~80°C) (~8bar) (~130°C)   (~10-15bar)   │
  │             ▲       ▲       ▲           ▲             │
  │             │       │       │           │             │
  │          Reversible │    Semi-         Last            │
  │                  Irreversible reversible resort        │
  │                                                        │
  │  Each layer is independent -- if one fails, the next  │
  │  catches the fault. True defense-in-depth.             │
  │                                                        │
  │  Additional safety (not counted in τ=4):              │
  │    - External fuse (pack level)                        │
  │    - BMS monitoring (pack level)                       │
  │    - Ceramic coating on separator (~3 μm Al₂O₃)      │
  │    - Electrolyte additives (flame retardant)           │
  │                                                        │
  └───────────────────────────────────────────────────────┘
```

---

## 11. Honesty Assessment

### 정직한 평가 -- 이 레벨은 n=6 연결이 가장 약하다

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  HONESTY ASSESSMENT: HEXA-CORE                                   │
  ├──────────────────────────────────────────────────────────────────┤
  │                                                                  │
  │  EXACT (1):                                                      │
  │    - 18650 diameter 18mm = 3n (but only ONE cell format)        │
  │      Even this is likely coincidental -- Sony chose 18mm        │
  │      based on available winding mandrel sizes in 1991           │
  │                                                                  │
  │  CLOSE (2):                                                      │
  │    - 3 form factors = n/φ (classification is somewhat           │
  │      arbitrary; coin cells, button cells, and other             │
  │      niche formats also exist)                                  │
  │    - 4 safety layers = τ (cylindrical only; prismatic           │
  │      and pouch use 2-3 layers)                                  │
  │                                                                  │
  │  WEAK (2):                                                       │
  │    - Cell energy ~12 Wh for 18650 = σ (unit-dependent)         │
  │    - Tab configurations = φ = 2 (oversimplification;            │
  │      multi-tab variants exist)                                  │
  │                                                                  │
  │  FAIL (5+):                                                      │
  │    - 21700 dimensions: 21mm / 70mm (no match)                  │
  │    - 4680 dimensions: 46mm / 80mm (no match)                   │
  │    - BYD blade: 960 / 90 / 13.5 mm (no match)                 │
  │    - CATL prismatic: all sizes OEM-driven (no match)           │
  │    - Pouch cell dimensions: 100% application-specific          │
  │    - Cycle life counts: varies wildly (no pattern)             │
  │    - Voltage per cell: 3.2-4.2V (chemistry, not n=6)          │
  │                                                                  │
  │  ═══════════════════════════════════════════════════════        │
  │  VERDICT: Cell form factor design is driven by manufacturing   │
  │  constraints, thermal management, and cost optimization.       │
  │  n=6 connections at this level are sparse and largely          │
  │  coincidental. This is the WEAKEST level in the                │
  │  HEXA-BATTERY hierarchy.                                       │
  │                                                                  │
  │  Why is this level weak?                                        │
  │    - Crystal chemistry (Level 1): CN=6 is physics.             │
  │    - Electrode chemistry (Level 2): stoichiometry matters.     │
  │    - Cell geometry (THIS LEVEL): pure engineering.              │
  │    - Pack systems (Level 3): 96S etc. recover n=6 matches.    │
  │                                                                  │
  │  Cell dimensions are optimized for: cost, thermal, safety,    │
  │  manufacturing speed, and OEM packaging. Mathematics does      │
  │  not constrain physical form factor at this scale.             │
  │  ═══════════════════════════════════════════════════════        │
  │                                                                  │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 12. Predictions & Falsifiability

### 검증 가능한 예측

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  PREDICTIONS (with honesty)                                      │
  ├──────────────────────────────────────────────────────────────────┤
  │                                                                  │
  │  P1: Future cylindrical formats will NOT follow n=6             │
  │      → 4695, 46120 are rumored -- neither has n=6 dims          │
  │      → ANTI-PREDICTION: we predict n=6 will NOT appear          │
  │      → This is honest -- cell geometry is engineering           │
  │      → Status: LIKELY CORRECT                                   │
  │                                                                  │
  │  P2: Safety layer count will remain ~4 for cylindrical          │
  │      → Defense-in-depth has engineering optimum near 3-5        │
  │      → Too few = unsafe; too many = expensive                   │
  │      → τ=4 match is coincidental but stable                    │
  │      → Status: TESTABLE (track new cell designs)               │
  │                                                                  │
  │  P3: 3 form factors will persist as dominant                    │
  │      → Solid-state may introduce 4th (bipolar stack)           │
  │      → If 4th form factor becomes dominant, n/φ=3 breaks       │
  │      → Status: TESTABLE (watch SSB commercialization)          │
  │                                                                  │
  │  P4: Tabless design will dominate large-format cylindrical     │
  │      → Not an n=6 prediction -- pure engineering trend         │
  │      → But reduces tab types to 1, breaking φ=2 match          │
  │      → Status: LIKELY (Tesla, others adopting)                 │
  │                                                                  │
  │  NOTE: Most predictions at this level are ANTI-n=6.            │
  │  This is the correct honest stance for engineering-driven      │
  │  parameters.                                                    │
  │                                                                  │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 13. Future Directions

### 블레이드, 테이블탑, CTC/CTB

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  CELL DESIGN EVOLUTION (2025-2035)                               │
  ├──────────────────────────────────────────────────────────────────┤
  │                                                                  │
  │  Trend 1: CELL-TO-PACK (CTP) / CELL-TO-BODY (CTB)             │
  │                                                                  │
  │    Traditional:  Cell → Module → Pack → Vehicle                │
  │    CTP (CATL):   Cell ──────────► Pack → Vehicle               │
  │    CTB (BYD):    Cell ──────────────────► Vehicle               │
  │                                                                  │
  │    Eliminates module = fewer parts, more energy density         │
  │    BYD Blade cell acts as structural member                     │
  │                                                                  │
  │  Trend 2: LARGER CELLS                                          │
  │                                                                  │
  │    18650 (3.5Ah) → 21700 (5Ah) → 4680 (25Ah) → ???            │
  │    Trend is toward fewer, bigger cells per pack                 │
  │    Reduces number of welds, interconnects, failure points       │
  │                                                                  │
  │  Trend 3: SOLID-STATE CELLS                                     │
  │                                                                  │
  │    Solid electrolyte → no liquid → bipolar stacking            │
  │    Cell voltage stacking inside single package                  │
  │    Could redefine what "cell" means                             │
  │    Toyota, QuantumScape, Samsung SDI targets: 2027-2030        │
  │                                                                  │
  │  Trend 4: DRY ELECTRODE PROCESS                                │
  │                                                                  │
  │    No solvent → 50% less capex, 30% less energy                │
  │    Tesla (Maxwell acquisition), others pursuing                 │
  │    Changes manufacturing but not form factor                    │
  │                                                                  │
  │  Trend 5: SILICON ANODE INTEGRATION                             │
  │                                                                  │
  │    Si expands 300% during lithiation                            │
  │    Cell design must accommodate swelling                        │
  │    Pouch cells may benefit (natural flex)                       │
  │    Cylindrical harder (rigid can)                               │
  │                                                                  │
  │  n=6 impact on future trends: NONE.                            │
  │  All trends are driven by cost, density, safety, speed.        │
  │                                                                  │
  └──────────────────────────────────────────────────────────────────┘
```

---

## 14. n=6 Complete Parameter Map

### 전체 파라미터 맵

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  HEXA-CORE PARAMETER MAP                                         │
  ├──────────────────────────────────────────────────────────────────┤
  │                                                                  │
  │  This is intentionally short. We do not inflate matches.        │
  │                                                                  │
  └──────────────────────────────────────────────────────────────────┘
```

| # | Parameter | Value | n=6 Expression | Grade |
|---|-----------|-------|----------------|-------|
| 1 | Cell form factors | 3 | n/phi = 6/2 | CLOSE |
| 2 | 18650 diameter | 18mm | 3n = 3x6 | EXACT |
| 3 | Safety layers (cylindrical) | 4 | tau(6) | CLOSE |
| 4 | 18650 energy | ~12 Wh | sigma? | WEAK |
| 5 | Tab configurations | 2 | phi(6) | WEAK |
| 6 | 21700 diameter | 21mm | -- | FAIL |
| 7 | 4680 diameter | 46mm | -- | FAIL |
| 8 | 4680 height | 80mm | -- | FAIL |
| 9 | BYD blade length | 960mm | -- | FAIL |
| 10 | Pouch cell dims | varies | -- | FAIL |

**Summary**:

```
  ┌────────────────────────────────────────┐
  │  GRADE DISTRIBUTION                     │
  ├────────────────────────────────────────┤
  │                                         │
  │  EXACT:  1 / 10   (10%)               │
  │  CLOSE:  2 / 10   (20%)               │
  │  WEAK:   2 / 10   (20%)               │
  │  FAIL:   5 / 10   (50%)               │
  │                                         │
  │  EXACT rate: 10% -- weakest level.     │
  │                                         │
  │  For comparison:                        │
  │    HEXA-CELL (chemistry): ~80% EXACT   │
  │    HEXA-ELECTRODE: ~60% EXACT          │
  │    HEXA-CORE (this): ~10% EXACT        │
  │    HEXA-PACK (systems): ~50% EXACT     │
  │                                         │
  │  Cell design is engineering,            │
  │  not number theory.                     │
  │                                         │
  └────────────────────────────────────────┘
```

---

## 15. Open Questions / TODO

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  OPEN QUESTIONS                                                  │
  ├──────────────────────────────────────────────────────────────────┤
  │                                                                  │
  │  Q1: Does the 18650 → 21700 → 4680 size evolution follow      │
  │      any mathematical pattern? (Probably not -- each was        │
  │      optimized for specific thermal/manufacturing constraints)  │
  │                                                                  │
  │  Q2: Will solid-state cells create a 4th form factor that      │
  │      breaks the n/phi=3 count?                                  │
  │                                                                  │
  │  Q3: Is there an optimal cell aspect ratio derivable from      │
  │      first principles? (Thermal modeling suggests height/dia   │
  │      = 3-4 is optimal for cylindrical, but this is physics,    │
  │      not n=6)                                                    │
  │                                                                  │
  │  Q4: Separator thickness evolution: 25→20→15→12 μm.           │
  │      Is σ=12 μm a natural limit? Probably coincidence --       │
  │      limited by puncture resistance and coating uniformity.     │
  │                                                                  │
  │  Q5: Can jelly roll turn count be optimized via n=6?           │
  │      Almost certainly not -- it is determined by electrode      │
  │      thickness, cell diameter, and separator thickness.         │
  │                                                                  │
  └──────────────────────────────────────────────────────────────────┘
```

### TODO

- [ ] Verify 18650 manufacturing history (was 18mm truly mandrel-driven?)
- [ ] Track solid-state cell form factors as they commercialize (2027+)
- [ ] Analyze whether safety layer count changes with new chemistries
- [ ] Cross-reference with HEXA-PACK for cell-to-pack n=6 connections
- [ ] Investigate if any cell thermal models produce n=6 optima

---

## 16. Links

### Internal Links (HEXA-BATTERY 계층)

```
  ┌──────────────────────────────────────────────────────────────────┐
  │  HEXA-BATTERY HIERARCHY                                          │
  ├──────────────────────────────────────────────────────────────────┤
  │                                                                  │
  │  Level 1: hexa-cell.md .......... Crystal chemistry (CN=6)     │
  │  Level 2: hexa-electrode.md ..... Electrode architecture        │
  │  Level 3: hexa-core.md .......... Cell core design (THIS)   ◄  │
  │  Level 4: hexa-pack.md .......... Pack & BMS systems           │
  │  Level 5: hexa-grid.md .......... Grid integration             │
  │                                                                  │
  │  Roadmap: goal.md                                               │
  │                                                                  │
  └──────────────────────────────────────────────────────────────────┘
```

### Breakthrough Theorems

- **BT-27**: Carbon-6 chain (LiC6 + C6H12O6 + C6H6 -> 24e = J2)
- **BT-43**: Battery cathode CN=6 universality (ALL Li-ion = octahedral)
- **BT-57**: Battery cell ladder (6->12->24 cells = n->sigma->J2)

### External References

- Sony US patent 4,668,595 (1987) -- original 18650 specification
- Tesla Battery Day 2020 -- 4680 cell announcement
- BYD Blade Battery whitepaper (2020)
- CATL CTP 3.0 technical documentation (2023)
- Dahn et al., "Understanding the Degradation of Li-ion Batteries" (2020)

---

*HEXA-CORE: 1/10 EXACT (10%) -- the weakest level in the hierarchy, honestly assessed.
Cell design is engineering optimization. n=6 lives in chemistry (Level 1) and systems (Level 4), not in physical form factors.*
