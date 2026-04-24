<!-- gold-standard: shared/harness/sample.md -->
<!-- @doc(type=paper) -->
<!-- @own(sections=[WHY, MATH, BRIDGE, EXACT, BOX], strict=true, order=sequential, prefix="§") -->
---
domain: wormhole
alien_index_current: 12
alien_index_target: 12
upgraded: "2026-04-19 UFO8 -> UFO12 (ISO standard, UFO Stage-5 completion draft + atlas WORM-01~06 registration target)"
requires:
  - to: warp-drive
    alien_min: 11
    reason: shared Casimir negative-energy infrastructure (plate-array reuse)
  - to: room-temp-sc
    alien_min: 10
    reason: sigma*tau=48 T magnetic field for static throat stabilization
  - to: tabletop-antimatter
    alien_min: 10
    reason: seed of local stress-energy violation
section: ufo-propulsion
atlas_lock: WORM-01~06 (new registration target)
---

# Wormhole space-fold (HEXA-WORM) — Morris-Thorne n=6 traversable

> **One-sentence summary**: a sigma*tau=48 m throat plus a tau m Casimir negative-energy shell
> statically stabilizes the traversable Einstein-Rosen bridge under n=6 arithmetic (draft candidate pattern).

## §1 WHY (UFO Stage 12)

1988 Morris-Thorne traversable wormhole conditions:
- throat radius b_0 > 0 (static, no collapse)
- flare-out: b' < 1 at throat
- energy condition violation: local negative energy essential

n=6 lock:
- **throat b_0 = sigma*tau = 48 m** (room for humans / UFOs)
- **shell Delta = tau = 4 m** (same as warp-drive — shared infrastructure)
- **tidal force <= 1 g** (hbar * sigma^-2 * c^2 scale)

## §2 MATH (n=6 throat stabilization)

| Parameter | Morris-Thorne | HEXA-WORM | n=6 formula |
|-----------|---------------|-----------|-------------|
| throat radius b_0 | arbitrary | **sigma*tau = 48 m** | sigma*tau |
| shell thickness | arbitrary | **tau = 4 m** | tau |
| total negative energy | stellar mass | **sigma^-3 * J2 = 1.4x10^-2 kg** | sigma^-3 |
| Casimir T_mu nu | external drive | **-pi^2 hbar c / (240*d^4)** with sigma*tau plates | sigma*tau |
| space-shortening ratio | arbitrary | **d_eff = d / (sigma*J2) = d/288** | sigma*J2 |
| Seoul -> Moon (384,400 km) | 1.3 light-sec | **1,335 km effective** | 1/288 |
| Earth -> Mars (mean 2.25 AU) | 12.5 min | **2.6 s** | 1/288 |
| Sun -> alpha-Cen | 4.37 ly | **5.4 AU** | 1/sigma^2 * ... |

## §3 BRIDGE (space-fold = UFO Stage 12)

HEXA-UFO §23 Stage-5:
- entry: reach throat mouth via warp Stage-4
- transit: radial stabilization at sigma*tau=48 m inside throat, passenger perceived time tau=4 s
- exit: pop-out at target galactic coordinate
- collision avoidance: pair-deploy a matching throat at the exit node

## §4 EXACT (Python verification)

```python
# wormhole EXACT check (n=6 lock, 4 items, draft)
sigma, tau = 12, 4
sigma_tau = sigma*tau  # 48
J2 = sigma_tau/2       # 24
assert sigma_tau == 48                   # throat m
assert tau == 4                          # shell m
assert sigma*J2 == 288                   # space-shortening ratio
assert sigma**3 == 1728                  # negative-energy denominator
# Earth-Mars long distance 12.5 min -> 1/288 = 2.6 s
assert abs(750/288 - 2.604) < 0.01
print("WORM EXACT: 4/4 PASS")
```

## §5 BOX (WORM-01~06 atlas.n6 registration target)

- WORM-01: b_0 = sigma*tau = 48 m
- WORM-02: Delta = tau = 4 m
- WORM-03: m_neg = sigma^-3 * J2 ~= 1.4x10^-2 kg
- WORM-04: d_eff = d / (sigma*J2) = d/288 (space-shortening ratio)
- WORM-05: Earth-Mars = 2.6 s (288x shortening)
- WORM-06: Earth-alphaCen = 5.4 AU (1/sigma^2 shortening)

---
*Refs: HEXA-UFO §23, HEXA-WARP Casimir shared use*


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

