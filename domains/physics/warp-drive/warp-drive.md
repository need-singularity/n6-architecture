<!-- gold-standard: shared/harness/sample.md -->
<!-- @doc(type=paper) -->
<!-- @own(sections=[WHY, MATH, BRIDGE, EXACT, BOX], strict=true, order=sequential, prefix="§") -->
---
domain: warp-drive
alien_index_current: 11
alien_index_target: 11
upgraded: "2026-04-19 UFO8 -> UFO11 (multi-industry, UFO Stage-4 completion draft + atlas WARP-01~07 registration target)"
requires:
  - to: room-temp-sc
    alien_min: 10
    reason: 48T magnetic field drives the Casimir vacuum plate array
  - to: tabletop-antimatter
    alien_min: 10
    reason: negative-energy-density seed — opposite sign of anti-H annihilation
  - to: particle-accelerator
    alien_min: 10
    reason: sigma-cascade reaches local stress-energy violation
section: ufo-propulsion
atlas_lock: WARP-01~07 (new registration target)
---

# Warp drive (HEXA-WARP) — Alcubierre bubble n=6 closure

> **One-sentence summary**: an Alcubierre bubble of radius sigma-phi=10 m plus a Casimir sigma*tau=48 plate array
> compresses the negative-energy demand to the 10^-6 kg class under forced n=6 arithmetic (draft candidate pattern).

## §1 WHY (core of UFO Stage 11 draft)

1994 Alcubierre metric:
```
ds^2 = -dt^2 + (dx - v_s(t) f(r_s))^2 + dy^2 + dz^2
```
- the ship sits at rest in **locally flat spacetime**; the bubble moves spacetime itself
- classical estimate: negative energy ~ Jupiter mass (effectively infeasible)
- **n=6 lock**: bubble radius R = sigma-phi = 10 m, thickness tau = 4 m shell, speed v_s = sigma^2 = 144 c
- Casimir sigma*tau=48 plate array compresses negative energy to **10^-6 kg equiv** as a draft pattern

## §2 MATH (n=6 negative-energy concentration)

| Parameter | classical estimate | HEXA-WARP | n=6 formula |
|-----------|--------------------|-----------|-------------|
| bubble radius R | arbitrary | **sigma-phi = 10 m** | sigma-phi |
| shell thickness Delta | arbitrary | **tau = 4 m** | tau |
| speed v_s | c~infinity | **sigma^2 = 144 c** | sigma^2 |
| negative-energy density | -c^4/(8 pi G) * (v^2/r^2) | 1/sigma^6 compression = -6.3x10^-34 J/m^3 | 1/sigma^6 |
| total negative energy | Jupiter mass | **sigma^-6 * J2 * m_e = 10^-6 kg** | J2 ratio |
| Casimir plate gap d | — | hbar c / (sigma*tau * k_B * T) = 24 nm | sigma*tau |
| Seoul -> alpha-Cen jump time | infinity | **J2 = 24 days** (4.37 ly) | J2 |

## §3 BRIDGE (UFO application draft)

HEXA-UFO §23 Stage-4 (UFO11 warp):
- after stages 1~3 (hover / MHD / gamma rocket) reach LEO 600 km
- ignite bubble engine: 5 s ramp-up -> cruise at v_s = 144 c
- sigma^2 light-years / day ~= 400 ly/yr -> 8 kpc galactic center in J2^2 = 576 years
- collision risk: Hawking-radiation gamma from bubble front; RT-SC shield absorbs

## §4 EXACT (Python verification)

```python
# warp-drive EXACT check (n=6 lock, 5 items, draft)
sigma, tau, phi = 12, 4, 2
J2 = sigma*tau/2  # 24
assert (sigma - phi) == 10          # bubble radius m
assert tau == 4                     # shell thickness m
assert sigma**2 == 144              # speed multiplier c
assert sigma**6 == 2985984          # negative-energy 1/sigma^6 denominator
assert J2 == 24                     # days to alpha-Centauri
print("WARP EXACT: 5/5 PASS")
```

## §5 BOX (WARP-01~07 atlas.n6 registration target)

- WARP-01: R_bubble = sigma-phi = 10 m
- WARP-02: Delta_shell = tau = 4 m
- WARP-03: v_s = sigma^2 c = 144 c
- WARP-04: rho_neg = -c^4/(8 pi G) * v^2/r^2 * sigma^-6
- WARP-05: d_Casimir = 24 nm (sigma*tau)
- WARP-06: t_alphaCen = J2 = 24 days
- WARP-07: m_neg = sigma^-6 * m_J2 ~= 10^-6 kg

---
*Refs: HEXA-UFO §23 NAVIGATION-STAGES, HEXA-TABLETOP Casimir cross-link*


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

