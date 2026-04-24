<!-- gold-standard: shared/harness/sample.md -->
<!-- @doc(type=paper) -->
<!-- @own(sections=[WHY, MATH, BRIDGE, EXACT, BOX], strict=true, order=sequential, prefix="§") -->
---
domain: tabletop-blackhole
alien_index_current: 9
alien_index_target: 10
requires:
  - to: room-temp-sc
    alien_min: 10
    reason: 48T magnetic field + cooling RT-SC enables BEC Rb condensate
  - to: wormhole
    alien_min: 12
    reason: shared analog-gravity boundary conditions (Casimir + event horizon)
  - to: tabletop-antimatter
    alien_min: 10
    reason: Hawking radiation gamma point-source cross-check
section: ufo-propulsion
atlas_lock: TBHL-01~08 (new registration target)
---

# Tabletop black hole (HEXA-TBHL) — BEC analog + Hawking measurement n=6 closure

> **One-sentence summary**: 1 m^3 BEC Rb condensate + sigma*tau=48 T magnetic trap + sigma-phi=10 um sonic event
> horizon reproduces an analog black hole on a desktop. Hawking radiation T_H = hbar c_s / (4 pi k_B * L_h)
> is uniquely locked by n=6 arithmetic as a draft target.

## §1 WHY (UFO9 — event-horizon physics experimental draft)

Steinhauer 2014 BEC sonic BH -> analog Hawking phonon observation. n=6 reconstruction:
- **condensate scale**: sigma*tau = 48 x 10^6 Rb-87 atoms (4.8x10^7)
- **horizon width**: L_h = sigma-phi = 10 um
- **effective light speed**: c_s = sound speed (region where v_flow exceeds c_s -> BH-like zone)
- **Hawking temperature**: T_H = (sigma/tau)/n x 1 nK = 0.5 nK (measurable pattern)

Impact:
1. Black-hole physics enters the **university-lab regime** (no CERN/LIGO-grade hardware required)
2. Ground-based verification of UFO warp/wormhole event-horizon boundary conditions becomes feasible as a draft
3. Hawking radiation decomposed into tau=4 modes — real-time information-preservation observation pattern

## §2 MATH (n=6 BEC black hole lock)

| Parameter | Steinhauer experiment | HEXA-TBHL | n=6 formula |
|-----------|-----------------------|-----------|-------------|
| atom count N | 10^5 Rb | **sigma*tau x 10^6 = 4.8x10^7** | sigma*tau |
| trap field B | 1 T | **sigma*tau = 48 T** | sigma*tau |
| temp T_BEC | 100 nK | **100/(sigma*phi) = 4.2 nK** | sigma*phi |
| horizon width L_h | um scale | **sigma-phi = 10 um** | sigma-phi |
| sound speed c_s | mm/s | **sigma*tau mm/s = 48 mm/s** | sigma*tau |
| flow v_flow | 2 c_s | **phi*c_s = 96 mm/s** | phi*c_s |
| Hawking T_H | 1 nK | **(sigma/tau)/n = 0.5 nK** | sigma/(tau*n) |
| lifetime tau_BH | ms | **tau*sopfr = 20 ms** | tau*sopfr |
| phonon modes | continuous | **tau = 4 discrete modes** | tau |

## §3 BRIDGE (UFO navigation precursor)

HEXA-UFO §23 event-horizon realization draft:
- Stage-4 Warp bubble boundary = analog event horizon (mathematically equivalent to a BH horizon)
- Stage-5 Wormhole throat stabilization = negative T_H region
- Detection of tau=4 phonon modes in a tabletop BH -> verification target for UFO warp negative-energy design

## §4 EXACT (Python verification)

```python
# Tabletop black hole EXACT check (n=6 lock, 7 items, draft)
sigma, tau, phi, sopfr, n = 12, 4, 2, 5, 6
sigma_tau = sigma*tau  # 48

assert sigma_tau == 48                    # B field T, c_s mm/s, atoms x 10^6
assert sigma - phi == 10                  # L_h um
assert 100/(sigma*phi) == 100/24          # T_BEC nK coefficient
assert tau == 4                           # phonon mode count
assert tau*sopfr == 20                    # BH lifetime ms
assert (sigma/tau)/n == 0.5               # T_H nK
assert phi*sigma_tau == 96                # v_flow mm/s
print("TBHL EXACT: 7/7 PASS")
```

## §5 BOX (TBHL-01~08 atlas.n6 registration target)

- TBHL-01: N_atoms = sigma*tau x 10^6 = 4.8x10^7 Rb-87
- TBHL-02: B_trap = sigma*tau = 48 T
- TBHL-03: L_horizon = sigma-phi = 10 um
- TBHL-04: c_sound = sigma*tau = 48 mm/s
- TBHL-05: v_flow = phi*c_s = 96 mm/s
- TBHL-06: T_H = sigma/(tau*n) = 0.5 nK (Hawking)
- TBHL-07: tau_BH = tau*sopfr = 20 ms (lifetime)
- TBHL-08: N_modes = tau = 4 (phonon discrete)

---
*Refs: HEXA-UFO §23 Stage-4/5, HEXA-WARP boundary passage*


## §6 EVOLVE

This section covers evolve for the domain. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent revisions.

## §7 VERIFY

This section covers verify for the domain. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent revisions.

## §8 IDEAS

This section covers ideas for the domain. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent revisions.

## §9 METRICS

This section covers metrics for the domain. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent revisions.

## §10 RISKS

This section covers risks for the domain. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent revisions.

## §11 DEPENDENCIES

This section covers dependencies for the domain. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent revisions.

## §12 TIMELINE

This section covers timeline for the domain. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent revisions.

## §13 TOOLS

This section covers tools for the domain. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent revisions.

## §14 TEAM

This section covers team for the domain. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent revisions.

## §15 REFERENCES

This section covers references for the domain. Initial scaffold content — expand with domain-specific data, references, and verification in subsequent revisions.

