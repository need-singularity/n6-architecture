# N6 Oceanography -- Perfect Number Arithmetic in Ocean Systems

## Overview

Ocean circulation, wave dynamics, marine chemistry, and ecosystem structure
analyzed through n=6 arithmetic. The ocean's discrete structures (circulation cells,
ion species, depth layers) provide testable n=6 connections.

> **Honesty principle**: Oceanographic counts can vary by classification scheme.
> EXACT only when the number is fixed by physics/chemistry and not convention-dependent.

## Core Constants

```
  n = 6          (perfect number)
  sigma(6) = 12  (sum of divisors)
  tau(6) = 4     (number of divisors)
  phi(6) = 2     (Euler totient)
  sopfr(6) = 5   (sum of prime factors)
  J_2(6) = 24    (Jordan totient)
  R(6) = 1       (sigma*phi/(n*tau))
```

## BT Cross-References

```
  BT-119: 지구 6권역 — 수권=해양 핵심
  BT-122: 산호 육방정계 n=6 기하학
  BT-118: CO₂ 해양 흡수, 탄소순환
  BT-120: 해수 정화, CN=6 촉매
  BT-62:  Grid frequency 60Hz=sigma*sopfr — 파동 주기 연결
```

---

## Category A: Ocean Circulation

---

### H-OCN-01: Thermohaline 6-Cell Circulation = n=6

> Global ocean thermohaline circulation consists of 6 major convection cells.

```
  Evidence:
    - 6 major ocean gyres: N.Atlantic, S.Atlantic, N.Pacific, S.Pacific, Indian, Antarctic
    - Each gyre = 1 circulation cell
    - Count = n = 6

  Grade: CLOSE (gyre count is convention-dependent; Antarctic circumpolar
         is sometimes split further)
  Lenses: network, boundary, topology
```

---

### H-OCN-02: Major Seawater Ions = n=6

> The 6 dominant dissolved ions in seawater form the "big six."

```
  Evidence:
    - Cl⁻, Na⁺, SO₄²⁻, Mg²⁺, Ca²⁺, K⁺ = 6 species
    - These account for 99.3% of dissolved salts
    - Count = n = 6 EXACT

  Grade: EXACT (universally accepted in chemical oceanography)
  Lenses: info, boundary, stability
```

---

### H-OCN-03: Beaufort Scale 12 Grades = sigma(6)

> The Beaufort wind scale has 12 force levels (0-12).

```
  Evidence:
    - Beaufort 0 (calm) to Beaufort 12 (hurricane)
    - 13 levels → 12 grade transitions = sigma = 12
    - Originally defined 1805, internationally adopted

  Grade: CLOSE (13 levels = sigma+mu, or 12 transitions = sigma)
  Lenses: scale, wave, boundary
```

---

### H-OCN-04: Average Ocean Depth ~6 km = n

> Mean depth of the world ocean is approximately 3.7 km, but maximum ~11 km,
> and ocean trench zones average ~6 km.

```
  Evidence:
    - Mean ocean depth = 3.688 km (not 6)
    - Abyssal zone: 4-6 km (boundary at 6 km)
    - Hadal zone boundary = 6 km
    - 6 km = n = 6 as depth boundary

  Grade: CLOSE (6 km is abyssal-hadal boundary, not mean depth)
  Lenses: boundary, scale, gravity
```

---

### H-OCN-05: Ocean Depth Zones = n=6

> The ocean is divided into 6 depth zones (layers).

```
  Evidence:
    - Epipelagic (0-200m)
    - Mesopelagic (200-1000m)
    - Bathypelagic (1000-4000m)
    - Abyssopelagic (4000-6000m)
    - Hadopelagic (6000-11000m)
    - Some classifications add "surface" → total 5 or 6

  Grade: CLOSE (standard = 5 zones; with surface zone = 6)
  Lenses: boundary, scale, topology
```

---

### H-OCN-06: Wave Period ~12s = sigma

> Dominant ocean swell period is approximately 12 seconds.

```
  Evidence:
    - Typical ocean swell T = 10-14 s, peak at ~12 s
    - Tsunami deep-water T = 600-3600 s (different regime)
    - Wind wave peak = 12 s = sigma = 12

  Grade: CLOSE (12s is typical peak but varies by region/season)
  Lenses: wave, scale, multiscale
```

---

### H-OCN-07: Ekman Spiral 45-degree Surface Deflection = sigma*tau/φ - n

> Surface current deflects ~45 degrees from wind; Ekman depth contains
> sigma-related layer structure.

```
  Evidence:
    - Ekman theory: surface current 45° from wind direction
    - Net transport 90° = sigma*sopfr/n/phi = 15 (no)
    - Actually 45 is not directly n=6 arithmetic
    - But Ekman layer count in discrete models often uses 12 layers = sigma

  Grade: WEAK (45° is from Coriolis physics, not n=6; layer count is model choice)
  Lenses: wave, gravity, recursion
```

---

### H-OCN-08: Coral Hexagonal Symmetry = n=6

> Hexacorals (order Hexacorallia) have 6-fold radial symmetry.

```
  Evidence:
    - Hexacorallia: tentacles in multiples of 6
    - Most reef-building corals are hexacorals
    - 6-fold symmetry = n = 6 EXACT
    - BT-122: hexagonal geometry universality

  Grade: EXACT (biological fact, not convention)
  Lenses: topology, evolution, boundary
```

---

### H-OCN-09: Seawater Salinity ~35 ppt, Major Cation/Anion Ratio = sigma+phi

> Total dissolved solids ~35 g/kg; Na/Cl mass ratio relationship.

```
  Evidence:
    - Mean salinity = 35 ppt
    - 35 = sopfr * (sigma - sopfr) = 5 * 7 = 35? Yes!
    - Or 35 = n * sopfr + sopfr = 6*5 + 5 = 35
    - But simplest: 35 = sopfr * (sigma - sopfr)

  Grade: CLOSE (35 ppt is well-established but not fundamental constant)
  Lenses: info, stability, scale
```

---

### H-OCN-10: Pacific Ring of Fire ~24 Tectonic Segments = J₂

> The Pacific Ring of Fire consists of approximately 24 major volcanic/seismic segments.

```
  Evidence:
    - Various classifications give 20-30 segments
    - 24 is within range = J₂ = 24
    - 12 major plates bordering Pacific = sigma = 12
    - Total Earth major plates = 12~15 (sigma range)

  Grade: WEAK (segment count is classification-dependent)
  Lenses: network, boundary, topology
```

---

## Summary Table

| ID | Hypothesis | n=6 Link | Grade |
|----|-----------|----------|-------|
| H-OCN-01 | Thermohaline 6-cell | n=6 | CLOSE |
| H-OCN-02 | Seawater 6 major ions | n=6 | EXACT |
| H-OCN-03 | Beaufort 12 grades | sigma=12 | CLOSE |
| H-OCN-04 | Ocean depth boundary 6km | n=6 | CLOSE |
| H-OCN-05 | Ocean 6 depth zones | n=6 | CLOSE |
| H-OCN-06 | Wave period ~12s | sigma=12 | CLOSE |
| H-OCN-07 | Ekman spiral layers | sigma=12 | WEAK |
| H-OCN-08 | Coral hexagonal symmetry | n=6 | EXACT |
| H-OCN-09 | Salinity 35 ppt | sopfr*(sigma-sopfr) | CLOSE |
| H-OCN-10 | Ring of Fire ~24 segments | J₂=24 | WEAK |

**EXACT: 2/10, CLOSE: 6/10, WEAK: 2/10**
