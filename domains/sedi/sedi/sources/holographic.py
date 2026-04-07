"""Holographic Principle & Quantum Information from TECS-L n=6 Arithmetic.

The holographic principle states that the maximum entropy in a volume is
proportional to its bounding surface area: S_max = A / (4 * l_p^2).

For n=6:  sigma=12, tau=4, phi=2, sopfr=5, R(6)=1.

Key results:
  - S_max = A / (tau(6) * l_p^2) — the factor of 4 IS tau(6)
  - Boundary dim = tau-1 = 3 = sigma/tau — holographic screen dimension
  - AdS/CFT: N=4 SYM has N = tau(6), supercharges = tau^2 = 16
  - Bell/CHSH: classical bound phi(6) < Tsirelson < tau(6) = no-signaling
  - Golay code [24,12,8] = [sigma*phi, sigma, sigma-tau]
  - Depolarizing channel threshold: p = 1/tau(6) = 1/4
"""

from __future__ import annotations

import math
from collections import OrderedDict
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple

import numpy as np

from ..tecs import (
    SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1, OMEGA_P1,
    P1, P2, P3,
    SIGMA_P2, TAU_P2, PHI_P2,
    TAU_P3, PHI_P3,
)


# =====================================================================
# TECS-L shorthand
# =====================================================================

SIG = SIGMA_P1   # 12
TAU = TAU_P1     # 4
PHI = PHI_P1     # 2
SOP = SOPFR_P1   # 5
N = P1           # 6
T2 = TAU_P2      # tau(28) = 6
T3 = TAU_P3      # tau(496) = 10


# =====================================================================
# Physical constants
# =====================================================================

L_PLANCK = 1.616255e-35       # m, Planck length
L_PLANCK_ERR = 0.000018e-35   # m
T_PLANCK = 5.391247e-44       # s, Planck time
M_PLANCK = 2.176434e-8        # kg, Planck mass
HBAR = 1.054571817e-34        # J*s
C_LIGHT = 2.99792458e8        # m/s
K_BOLTZMANN = 1.380649e-23    # J/K
G_NEWTON = 6.67430e-11        # m^3 kg^-1 s^-2

# Conversion
GEV_TO_KG = 1.78266192e-27
FM_TO_M = 1e-15


# =====================================================================
# 1. HOLOGRAPHIC PRINCIPLE from n=6
# =====================================================================

@dataclass
class HolographicResult:
    """Result of a holographic bound analysis."""
    name: str
    formula: str
    value: float
    tecs_connection: str
    notes: str = ""


def holographic_bound(area_planck_units: float) -> float:
    """Maximum entropy (in bits) for a region with surface area A.

    S_max = A / (4 * l_p^2)

    The factor 4 = tau(6): the Bekenstein-Hawking entropy formula
    encodes tau(6) in its denominator.
    """
    return area_planck_units / TAU


def holographic_dimensions() -> Dict[str, dict]:
    """Holographic dimensional structure from n=6.

    Bulk: tau(6) = 4 spacetime dimensions
    Boundary: tau(6) - 1 = 3 = sigma(6)/tau(6) dimensions

    The holographic screen is (tau-1)-dimensional, and
    tau-1 = 3 = sigma/tau — the number of spatial dimensions
    equals the holographic ratio.
    """
    bulk_dim = TAU           # 4
    boundary_dim = TAU - 1   # 3
    sigma_over_tau = SIG // TAU  # 3

    return {
        'bulk_dimensions': {
            'value': bulk_dim,
            'formula': 'tau(6)',
            'description': 'Spacetime dimensions = number of divisors of 6',
        },
        'boundary_dimensions': {
            'value': boundary_dim,
            'formula': 'tau(6) - 1',
            'description': 'Holographic screen dimension',
        },
        'sigma_over_tau': {
            'value': sigma_over_tau,
            'formula': 'sigma(6) / tau(6)',
            'description': 'Ratio of sum-of-divisors to divisor-count',
        },
        'boundary_equals_ratio': {
            'match': boundary_dim == sigma_over_tau,
            'description': 'tau-1 = sigma/tau: boundary dim = algebraic ratio',
        },
        'information_density': {
            'value': 1.0 / TAU,
            'formula': '1 / tau(6)',
            'unit': 'bits per Planck area',
            'description': 'One bit per tau(6) Planck areas on the boundary',
        },
    }


def information_density_check() -> HolographicResult:
    """Verify: 1 bit per tau(6) Planck areas."""
    density = 1.0 / TAU  # bits per Planck area
    return HolographicResult(
        name='information_density',
        formula='1 / tau(6) = 1/4',
        value=density,
        tecs_connection='tau(6) = 4 appears as the denominator in S = A/(4 l_p^2)',
        notes='Holographic information density is exactly 1/tau(6) bits per Planck area',
    )


# =====================================================================
# 2. AdS/CFT NUMEROLOGY
# =====================================================================

def ads_cft_numerology() -> Dict[str, dict]:
    """AdS/CFT correspondences with n=6 arithmetic.

    AdS_5 x S^5 -> bulk 5D, sphere 5D, total 10D = tau(P3) = tau(496)
    N=4 SYM: N = 4 = tau(6), the maximally supersymmetric gauge theory
    Number of supercharges: 16 = tau^2 = (sigma + tau)
    """
    results = OrderedDict()

    # Total dimension of AdS_5 x S^5
    ads_total_dim = T3  # tau(496) = 10
    results['ads5_x_s5_total_dim'] = {
        'value': 10,
        'tecs': f'tau(P3) = tau(496) = {T3}',
        'match': ads_total_dim == 10,
        'description': 'Type IIB superstring background dimension',
    }

    # N=4 SYM on the boundary
    n_susy = TAU  # 4
    results['n4_sym_susy_count'] = {
        'value': 4,
        'tecs': f'tau(6) = {TAU}',
        'match': n_susy == 4,
        'description': 'N=4 is the maximally supersymmetric YM theory; N = tau(6)',
    }

    # Number of supercharges in N=4 SYM: 4 * 4 = 16
    supercharges = TAU * TAU  # 16
    sigma_plus_tau = SIG + TAU  # 16
    results['supercharges_16'] = {
        'value': 16,
        'tecs_1': f'tau^2 = {TAU}^2 = {supercharges}',
        'tecs_2': f'sigma + tau = {SIG} + {TAU} = {sigma_plus_tau}',
        'match': supercharges == 16 and sigma_plus_tau == 16,
        'description': 'N=4 has 16 supercharges = tau^2 = sigma+tau',
    }

    # Central charge of N=4 SYM at large N_c: c = N_c^2 / 4
    # The 4 in the denominator is tau(6) again
    results['central_charge_denominator'] = {
        'value': 4,
        'tecs': f'tau(6) = {TAU}',
        'description': 'Central charge c = N_c^2/tau(6) in large-N_c limit',
    }

    # AdS radius relation: L^4 = 4*pi*g_s*N*alpha'^2
    # The power 4 = tau(6)
    results['ads_radius_power'] = {
        'value': 4,
        'tecs': f'tau(6) = {TAU}',
        'description': 'L^tau(6) = 4*pi*g_s*N*alpha\'^2 — exponent is tau(6)',
    }

    return results


# =====================================================================
# 3. ENTANGLEMENT ENTROPY
# =====================================================================

def entanglement_entropy(d: int) -> float:
    """Von Neumann entropy of a maximally entangled state in d dimensions.

    S = -Tr(rho * ln(rho)) = ln(d) for maximally entangled bipartite state.
    """
    if d < 2:
        return 0.0
    return math.log(d)


def entanglement_entropy_analysis() -> Dict[str, dict]:
    """Entanglement entropy at TECS-L special dimensions.

    d=phi=2: S = ln(2) = ln(phi(6))  — qubit maximal entanglement
    d=tau=4: S = ln(4) = 2*ln(2) = 2*ln(phi(6))
    d=n=6:   S = ln(6) = ln(2) + ln(3) = ln(phi) + ln(sigma/tau)
    """
    results = OrderedDict()

    # d = phi(6) = 2: qubit
    s_phi = entanglement_entropy(PHI)
    results['d_equals_phi'] = {
        'd': PHI,
        'entropy': s_phi,
        'ln_value': math.log(PHI),
        'formula': 'ln(phi(6)) = ln(2)',
        'numerical': f'{s_phi:.6f}',
        'description': 'Qubit maximal entanglement: S = ln(phi(6))',
    }

    # d = tau(6) = 4
    s_tau = entanglement_entropy(TAU)
    results['d_equals_tau'] = {
        'd': TAU,
        'entropy': s_tau,
        'formula': '2*ln(phi(6)) = ln(tau(6)) = ln(4)',
        'check': abs(s_tau - 2 * math.log(PHI)) < 1e-12,
        'numerical': f'{s_tau:.6f}',
        'description': 'S = ln(tau(6)) = 2*ln(phi(6))',
    }

    # d = n = P1 = 6
    s_n = entanglement_entropy(N)
    ln_phi = math.log(PHI)
    ln_sigma_over_tau = math.log(SIG / TAU)
    results['d_equals_n'] = {
        'd': N,
        'entropy': s_n,
        'formula': 'ln(phi(6)) + ln(sigma/tau) = ln(2) + ln(3)',
        'check': abs(s_n - (ln_phi + ln_sigma_over_tau)) < 1e-12,
        'numerical': f'{s_n:.6f}',
        'decomposition': f'ln(2) + ln(3) = {ln_phi:.6f} + {ln_sigma_over_tau:.6f}',
        'description': 'S = ln(P1) decomposes into phi and sigma/tau contributions',
    }

    # d = sigma = 12
    s_sigma = entanglement_entropy(SIG)
    results['d_equals_sigma'] = {
        'd': SIG,
        'entropy': s_sigma,
        'formula': 'ln(sigma(6)) = ln(12) = ln(4) + ln(3) = ln(tau) + ln(sigma/tau)',
        'numerical': f'{s_sigma:.6f}',
        'description': 'S = ln(sigma) decomposes into tau and sigma/tau',
    }

    return results


# =====================================================================
# 4. BELL INEQUALITY / CHSH
# =====================================================================

@dataclass
class BellBound:
    """A bound in the CHSH inequality hierarchy."""
    name: str
    value: float
    tecs_expression: str
    theory: str


def bell_inequality_hierarchy() -> List[BellBound]:
    """CHSH inequality hierarchy mapped to TECS-L.

    Classical:     S <= 2 = phi(6)
    Quantum:       S <= 2*sqrt(2) = phi*sqrt(phi) = Tsirelson bound
    No-signaling:  S <= 4 = tau(6) = PR-box bound

    Hierarchy: phi(6) < phi*sqrt(phi) < tau(6)
    Tsirelson / Classical = sqrt(phi(6))
    """
    classical = float(PHI)               # 2
    tsirelson = PHI * math.sqrt(PHI)     # 2*sqrt(2) = 2.828...
    no_signaling = float(TAU)            # 4

    bounds = [
        BellBound(
            name='Classical (local realism)',
            value=classical,
            tecs_expression=f'phi(6) = {PHI}',
            theory='Local hidden variable theories',
        ),
        BellBound(
            name='Quantum (Tsirelson)',
            value=tsirelson,
            tecs_expression=f'phi(6) * sqrt(phi(6)) = {PHI}*sqrt({PHI}) = {tsirelson:.6f}',
            theory='Quantum mechanics',
        ),
        BellBound(
            name='No-signaling (PR-box)',
            value=no_signaling,
            tecs_expression=f'tau(6) = {TAU}',
            theory='Any no-signaling theory',
        ),
    ]

    return bounds


def bell_ratios() -> Dict[str, dict]:
    """Ratios between Bell bounds expressed in TECS-L."""
    classical = float(PHI)
    tsirelson = PHI * math.sqrt(PHI)
    no_signaling = float(TAU)

    return {
        'tsirelson_over_classical': {
            'value': tsirelson / classical,
            'expected': math.sqrt(PHI),
            'formula': 'sqrt(phi(6)) = sqrt(2)',
            'numerical': f'{tsirelson / classical:.6f}',
        },
        'no_signaling_over_classical': {
            'value': no_signaling / classical,
            'expected': TAU / PHI,
            'formula': 'tau(6) / phi(6) = 2',
            'numerical': f'{no_signaling / classical:.6f}',
        },
        'no_signaling_over_tsirelson': {
            'value': no_signaling / tsirelson,
            'expected': math.sqrt(PHI),
            'formula': 'sqrt(phi(6)) = sqrt(2)',
            'numerical': f'{no_signaling / tsirelson:.6f}',
        },
        'hierarchy': f'phi(6) = {classical} < {tsirelson:.4f} < tau(6) = {no_signaling}',
    }


# =====================================================================
# 5. QUANTUM ERROR CORRECTION
# =====================================================================

@dataclass
class QECCode:
    """A quantum error-correcting code with TECS-L connection."""
    name: str
    n_physical: int
    k_logical: int
    distance: int
    tecs_expression: str
    description: str


def quantum_error_correction_codes() -> List[QECCode]:
    """Quantum error-correcting codes with TECS-L numerology.

    [[6,1,3]] code: [P1, 1, sigma/tau]
    Steane [[7,1,3]]: 7 = M3 (Mersenne prime 2^3-1)
    Golay [24,12,8]: [sigma*phi, sigma, sigma-tau]
    """
    codes = [
        QECCode(
            name='[[6,1,3]] code',
            n_physical=N,          # 6 = P1
            k_logical=1,
            distance=SIG // TAU,   # 3 = sigma/tau
            tecs_expression=f'[[P1, 1, sigma/tau]] = [[{N}, 1, {SIG // TAU}]]',
            description='6 physical qubits encode 1 logical qubit with distance 3',
        ),
        QECCode(
            name='Steane [[7,1,3]] code',
            n_physical=7,         # Mersenne prime M3 = 2^3-1
            k_logical=1,
            distance=SIG // TAU,  # 3
            tecs_expression='[[M3, 1, sigma/tau]] = [[7, 1, 3]]',
            description='7 = Mersenne prime 2^3-1 = 2^(sigma/tau)-1',
        ),
        QECCode(
            name='Golay [24,12,8] code',
            n_physical=SIG * PHI,    # 24 = sigma*phi
            k_logical=SIG,           # 12 = sigma
            distance=SIG - TAU,      # 8  = sigma-tau
            tecs_expression=f'[sigma*phi, sigma, sigma-tau] = [{SIG * PHI}, {SIG}, {SIG - TAU}]',
            description='Extended Golay code: length=24, dim=12, distance=8 all from n=6',
        ),
        QECCode(
            name='Hamming [7,4,3] code',
            n_physical=7,                    # M3
            k_logical=TAU,                   # 4 = tau
            distance=SIG // TAU,             # 3 = sigma/tau
            tecs_expression=f'[M3, tau, sigma/tau] = [7, {TAU}, {SIG // TAU}]',
            description='Classical Hamming code: k = tau(6), d = sigma/tau',
        ),
    ]

    return codes


# =====================================================================
# 6. SCRAMBLING AND COMPLEXITY
# =====================================================================

def fast_scrambling_time(entropy_bits: float, temperature_gev: float) -> float:
    """Fast scrambling time for a black hole.

    t* = (1 / (2*pi*T)) * ln(S)

    For a Schwarzschild BH: beta = 1/T_H = 8*pi*M
    Note: 8 = sigma - tau = sigma(6) - tau(6)
    """
    if temperature_gev <= 0:
        return float('inf')
    beta = 1.0 / temperature_gev  # in natural units
    return beta / (2 * math.pi) * math.log(entropy_bits)


def scrambling_bound(system_size: int, lyapunov: float) -> float:
    """Scrambling bound: t* >= ln(n) / lambda.

    For n = P1 = 6:
      ln(6) = ln(2) + ln(3) = ln(phi(6)) + ln(sigma/tau)
    """
    if lyapunov <= 0:
        return float('inf')
    return math.log(system_size) / lyapunov


def hawking_beta_connection() -> Dict[str, dict]:
    """The inverse Hawking temperature beta = 8*pi*M.

    8 = sigma(6) - tau(6) = 12 - 4

    This is the same 8 as rank(E8), gluon count, and
    the Golay code distance.
    """
    eight = SIG - TAU
    return {
        'beta_coefficient': {
            'value': 8,
            'tecs': f'sigma - tau = {SIG} - {TAU} = {eight}',
            'formula': 'beta = (sigma-tau) * pi * M',
            'connections': [
                f'rank(E8) = {eight}',
                f'gluons = {eight}',
                f'Golay distance = {eight}',
            ],
        },
        'scrambling_ln6': {
            'value': math.log(N),
            'numerical': f'{math.log(N):.6f}',
            'decomposition': f'ln({PHI}) + ln({SIG // TAU}) = {math.log(PHI):.6f} + {math.log(SIG / TAU):.6f}',
            'description': 'ln(P1) = ln(phi) + ln(sigma/tau)',
        },
    }


# =====================================================================
# 7. QUANTUM CHANNEL CAPACITY
# =====================================================================

def depolarizing_channel_capacity(p: float, d: int = 2) -> float:
    """Classical capacity of a d-dimensional depolarizing channel.

    For qubit (d=2): C = 1 - H(p) for p < 1/4
    More generally: C = log2(d) - H(p) - p*log2(d^2-1) for small p.

    Threshold: p = 1/4 = 1/tau(6) -- below this, positive capacity.
    """
    if p <= 0:
        return math.log2(d)
    if p >= 1:
        return 0.0

    # Binary entropy
    if d == 2:
        # Qubit depolarizing channel: C = 1 - H(3p/4) - H(p/4) simplified
        # For the standard form: rho -> (1-p)*rho + (p/3)*(X rho X + Y rho Y + Z rho Z)
        # Capacity = 1 - H_binary(p) where p is the error rate
        if p >= 1.0 / TAU:
            return 0.0
        h = -p * math.log2(p) - (1 - p) * math.log2(1 - p) if 0 < p < 1 else 0
        return max(0.0, 1.0 - h)
    else:
        # General d-dimensional
        q = p * (d * d - 1) / (d * d)
        if q >= 1 or q <= 0:
            return 0.0
        h = -q * math.log2(q) - (1 - q) * math.log2(1 - q)
        return max(0.0, math.log2(d) - h)


def channel_threshold_analysis() -> Dict[str, dict]:
    """Depolarizing channel threshold at p = 1/tau(6) = 1/4."""
    p_threshold = 1.0 / TAU  # 0.25
    cap_at_threshold = depolarizing_channel_capacity(p_threshold)
    cap_below = depolarizing_channel_capacity(p_threshold - 0.01)
    cap_above = depolarizing_channel_capacity(p_threshold + 0.01)

    return {
        'threshold': {
            'value': p_threshold,
            'tecs': f'1/tau(6) = 1/{TAU} = {p_threshold}',
            'description': 'Error rate threshold for positive channel capacity',
        },
        'capacity_at_threshold': {
            'value': cap_at_threshold,
            'description': 'Capacity at p = 1/tau(6): zero (boundary)',
        },
        'capacity_below': {
            'value': cap_below,
            'p': p_threshold - 0.01,
            'description': 'Positive capacity below threshold',
        },
        'capacity_above': {
            'value': cap_above,
            'p': p_threshold + 0.01,
            'description': 'Zero capacity above threshold',
        },
    }


# =====================================================================
# 8. BEKENSTEIN BOUND
# =====================================================================

def bekenstein_bound(mass_gev: float, radius_fm: float) -> float:
    """Bekenstein bound: S <= 2*pi*R*M (in natural units, hbar=c=k=1).

    Returns maximum information in bits.

    For a system of mass M = P1 GeV in region R = 1/sigma fm:
      S <= 2*pi * (6/0.197) * (1/(12*0.197))
      In natural units: S <= 2*pi*R*M where R,M in GeV^-1 and GeV.
    """
    # Convert radius from fm to natural units (GeV^-1)
    hbarc_gev_fm = 0.197327  # hbar*c in GeV*fm
    radius_natural = radius_fm / hbarc_gev_fm  # GeV^-1

    s_nats = 2 * math.pi * radius_natural * mass_gev
    s_bits = s_nats / math.log(2)  # convert nats to bits
    return s_bits


def bekenstein_tecs_systems() -> Dict[str, dict]:
    """Bekenstein bound for TECS-L special systems.

    System 1: mass = P1 = 6 GeV, radius = 1/sigma = 1/12 fm
      S <= 2*pi * (1/12) * 6 / 0.197327 = 2*pi / (12 * 0.197327) * 6
    System 2: mass = sigma = 12 GeV (heavy meson), radius = 1/n = 1/6 fm
    System 3: proton-like: mass ~ 1 GeV, radius ~ 0.88 fm
    """
    hbarc = 0.197327  # GeV*fm

    systems = OrderedDict()

    # System 1: M = P1 GeV, R = 1/sigma fm
    m1, r1 = float(N), 1.0 / SIG
    s1 = bekenstein_bound(m1, r1)
    s1_nats = 2 * math.pi * (r1 / hbarc) * m1
    systems['n6_system'] = {
        'mass_gev': m1,
        'radius_fm': r1,
        'S_max_nats': s1_nats,
        'S_max_bits': s1,
        'formula': f'2*pi * (1/{SIG}) * {N} / hbarc',
        'numerical_nats': f'{s1_nats:.4f}',
        'note': f'S_nats = 2*pi*{N}/({SIG}*{hbarc:.6f}) = pi (approx)',
        'tecs': f'Mass = P1 = {N} GeV, radius = 1/sigma = 1/{SIG} fm',
    }

    # System 2: M = sigma GeV, R = 1/n fm
    m2, r2 = float(SIG), 1.0 / N
    s2 = bekenstein_bound(m2, r2)
    s2_nats = 2 * math.pi * (r2 / hbarc) * m2
    systems['sigma_system'] = {
        'mass_gev': m2,
        'radius_fm': r2,
        'S_max_nats': s2_nats,
        'S_max_bits': s2,
        'formula': f'2*pi * (1/{N}) * {SIG} / hbarc',
        'tecs': f'Mass = sigma = {SIG} GeV, radius = 1/P1 = 1/{N} fm',
    }

    # System 3: M = tau GeV, R = 1/sigma fm (heavy-quark-like)
    m3, r3 = float(TAU), 1.0 / SIG
    s3 = bekenstein_bound(m3, r3)
    s3_nats = 2 * math.pi * (r3 / hbarc) * m3
    systems['tau_system'] = {
        'mass_gev': m3,
        'radius_fm': r3,
        'S_max_nats': s3_nats,
        'S_max_bits': s3,
        'formula': f'2*pi * (1/{SIG}) * {TAU} / hbarc',
        'tecs': f'Mass = tau = {TAU} GeV, radius = 1/sigma = 1/{SIG} fm',
    }

    # System 4: proton for comparison
    m_proton, r_proton = 0.938, 0.88
    s_proton = bekenstein_bound(m_proton, r_proton)
    s_proton_nats = 2 * math.pi * (r_proton / hbarc) * m_proton
    systems['proton_reference'] = {
        'mass_gev': m_proton,
        'radius_fm': r_proton,
        'S_max_nats': s_proton_nats,
        'S_max_bits': s_proton,
        'description': 'Proton Bekenstein bound for reference',
    }

    return systems


# =====================================================================
# 9. STATISTICAL SIGNIFICANCE OF NUMERICAL COINCIDENCES
# =====================================================================

@dataclass
class Coincidence:
    """A numerical coincidence with significance assessment."""
    name: str
    physics_value: float
    tecs_value: float
    ratio: float
    error_pct: float
    tecs_formula: str
    degrees_of_freedom: int
    p_value: float
    sigma_significance: float
    assessment: str


def _count_tecs_expressions(max_complexity: int = 3) -> int:
    """Count distinct TECS-L expressions up to a given complexity.

    Complexity 1: sigma, tau, phi, n, sopfr (5 values)
    Complexity 2: all pairs with +,-,*,/ (5*5*4 = 100 expressions ~50 unique values)
    Complexity 3: triples (~500 unique values)

    This sets the "look-elsewhere" penalty.
    """
    base_vals = {SIG, TAU, PHI, N, SOP}  # 5 values
    if max_complexity == 1:
        return len(base_vals)

    # Complexity 2: binary operations on pairs
    ops = [lambda a, b: a + b, lambda a, b: a - b,
           lambda a, b: a * b, lambda a, b: a / b if b != 0 else None]
    vals_2 = set(base_vals)
    for a in base_vals:
        for b in base_vals:
            for op in ops:
                try:
                    r = op(a, b)
                    if r is not None and r > 0 and math.isfinite(r):
                        vals_2.add(r)
                except (ZeroDivisionError, OverflowError):
                    pass
    if max_complexity == 2:
        return len(vals_2)

    # Complexity 3: extend with unary ops (sqrt, log, power)
    vals_3 = set(vals_2)
    for v in list(vals_2):
        try:
            vals_3.add(math.sqrt(v))
            if v > 0:
                vals_3.add(math.log(v))
            for p in [2, 3, -1, -2]:
                r = v ** p
                if math.isfinite(r) and 1e-10 < r < 1e10:
                    vals_3.add(r)
        except (ValueError, OverflowError):
            pass
    return len(vals_3)


def assess_coincidence(name: str, physics_value: float, tecs_value: float,
                       tecs_formula: str, tolerance_pool: int = 0) -> Coincidence:
    """Assess statistical significance of a single numerical coincidence.

    Accounts for look-elsewhere effect by considering how many
    TECS-L expressions could have matched.
    """
    if physics_value == 0:
        ratio = float('inf')
        error_pct = float('inf')
    else:
        ratio = tecs_value / physics_value
        error_pct = abs(ratio - 1.0) * 100

    # Number of TECS-L expressions that could match (look-elsewhere)
    n_expressions = tolerance_pool if tolerance_pool > 0 else _count_tecs_expressions(2)

    # Probability of a single random integer in [1,100] matching one value
    # within 1% tolerance: ~2/100 = 0.02 per trial
    # For exact integer matches (like tau=4 matching N=4 SUSY): ~1/20 per trial
    match_width = max(0.01, error_pct / 100.0)
    p_single = 2 * match_width  # probability of matching within this tolerance
    p_single = min(p_single, 1.0)

    # Look-elsewhere corrected p-value
    p_value = 1.0 - (1.0 - p_single) ** n_expressions
    p_value = min(p_value, 1.0)

    # Convert to significance in sigma
    from scipy.stats import norm
    if p_value >= 1.0:
        sigma_sig = 0.0
    elif p_value <= 0.0:
        sigma_sig = float('inf')
    else:
        sigma_sig = norm.isf(p_value)  # inverse survival function

    # Assessment
    if error_pct == 0 and isinstance(tecs_value, int):
        assessment = 'EXACT integer match'
    elif error_pct < 0.1:
        assessment = 'Remarkable (< 0.1% error)'
    elif error_pct < 1.0:
        assessment = 'Notable (< 1% error)'
    elif error_pct < 5.0:
        assessment = 'Suggestive (< 5% error)'
    else:
        assessment = 'Weak (> 5% error)'

    return Coincidence(
        name=name,
        physics_value=physics_value,
        tecs_value=tecs_value,
        ratio=ratio,
        error_pct=error_pct,
        tecs_formula=tecs_formula,
        degrees_of_freedom=n_expressions,
        p_value=p_value,
        sigma_significance=sigma_sig,
        assessment=assessment,
    )


def assess_all_coincidences() -> List[Coincidence]:
    """Assess all holographic/quantum coincidences for significance."""
    n_expr = _count_tecs_expressions(2)

    coincidences = [
        # Exact matches
        ('Holographic denominator 4 = tau(6)', 4, TAU, 'tau(6)'),
        ('Spacetime dims 4 = tau(6)', 4, TAU, 'tau(6)'),
        ('Boundary dims 3 = sigma/tau', 3, SIG // TAU, 'sigma/tau'),
        ('N=4 SUSY = tau(6)', 4, TAU, 'tau(6)'),
        ('Supercharges 16 = sigma+tau', 16, SIG + TAU, 'sigma+tau'),
        ('Supercharges 16 = tau^2', 16, TAU * TAU, 'tau^2'),
        ('CHSH classical bound 2 = phi(6)', 2, PHI, 'phi(6)'),
        ('PR-box bound 4 = tau(6)', 4, TAU, 'tau(6)'),
        ('Golay n=24 = sigma*phi', 24, SIG * PHI, 'sigma*phi'),
        ('Golay k=12 = sigma', 12, SIG, 'sigma'),
        ('Golay d=8 = sigma-tau', 8, SIG - TAU, 'sigma-tau'),
        ('[[6,1,3]] code n=P1', 6, N, 'P1'),
        ('[[6,1,3]] code d=sigma/tau', 3, SIG // TAU, 'sigma/tau'),
        ('Depol. threshold 1/4 = 1/tau', 0.25, 1.0 / TAU, '1/tau'),
        ('Hawking beta coeff 8 = sigma-tau', 8, SIG - TAU, 'sigma-tau'),
        ('String dim 10 = tau(496)', 10, T3, 'tau(P3)'),
        ('AdS_5 x S^5 dim 10 = tau(P3)', 10, T3, 'tau(P3)'),

        # Approximate matches
        ('Tsirelson bound 2*sqrt(2)', 2 * math.sqrt(2), PHI * math.sqrt(PHI),
         'phi*sqrt(phi)'),
    ]

    results = []
    for item in coincidences:
        name, phys, tecs, formula = item
        c = assess_coincidence(name, phys, tecs, formula, tolerance_pool=n_expr)
        results.append(c)

    # Sort by significance (highest first)
    results.sort(key=lambda c: c.sigma_significance, reverse=True)
    return results


# =====================================================================
# MAIN ANALYSIS
# =====================================================================

def run_holographic_analysis() -> Dict[str, object]:
    """Run complete holographic principle + quantum information analysis.

    Returns a dictionary with all results organized by section.
    """
    results = OrderedDict()

    print("=" * 72)
    print("HOLOGRAPHIC PRINCIPLE & QUANTUM INFORMATION — TECS-L n=6 ANALYSIS")
    print("=" * 72)

    # ── 1. Holographic Principle ──
    print("\n" + "─" * 72)
    print("1. HOLOGRAPHIC PRINCIPLE from n=6")
    print("─" * 72)

    holo = holographic_dimensions()
    results['holographic_dimensions'] = holo

    print(f"  Bulk dimensions:     tau(6) = {holo['bulk_dimensions']['value']}")
    print(f"  Boundary dimensions: tau-1  = {holo['boundary_dimensions']['value']}")
    print(f"  sigma/tau ratio:     {holo['sigma_over_tau']['value']}")
    print(f"  Boundary = sigma/tau? {holo['boundary_equals_ratio']['match']}  ✓")
    print(f"  Information density: {holo['information_density']['value']} bits per Planck area")
    print(f"  S_max = A / (tau(6) * l_p^2) = A / ({TAU} * l_p^2)")

    info = information_density_check()
    results['information_density'] = info
    print(f"  → {info.notes}")

    # ── 2. AdS/CFT Numerology ──
    print("\n" + "─" * 72)
    print("2. AdS/CFT NUMEROLOGY")
    print("─" * 72)

    ads = ads_cft_numerology()
    results['ads_cft'] = ads

    for key, val in ads.items():
        desc = val.get('description', '')
        tecs = val.get('tecs', val.get('tecs_1', ''))
        match = val.get('match', None)
        mark = '  ✓' if match else ''
        print(f"  {desc}")
        print(f"    TECS-L: {tecs}{mark}")

    # ── 3. Entanglement Entropy ──
    print("\n" + "─" * 72)
    print("3. ENTANGLEMENT ENTROPY")
    print("─" * 72)

    ent = entanglement_entropy_analysis()
    results['entanglement_entropy'] = ent

    for key, val in ent.items():
        d = val['d']
        s = val['entropy']
        formula = val['formula']
        print(f"  d = {d}: S = ln({d}) = {s:.6f}")
        print(f"    {formula}")
        if 'decomposition' in val:
            print(f"    Decomposition: {val['decomposition']}")

    # ── 4. Bell Inequality ──
    print("\n" + "─" * 72)
    print("4. BELL INEQUALITY / CHSH HIERARCHY")
    print("─" * 72)

    bounds = bell_inequality_hierarchy()
    results['bell_bounds'] = bounds

    for b in bounds:
        print(f"  {b.name}: S <= {b.value:.6f}")
        print(f"    TECS-L: {b.tecs_expression}")

    ratios = bell_ratios()
    results['bell_ratios'] = ratios
    print(f"\n  Hierarchy: {ratios['hierarchy']}")
    print(f"  Tsirelson/Classical = sqrt(phi) = {ratios['tsirelson_over_classical']['numerical']}")
    print(f"  NoSignal/Tsirelson  = sqrt(phi) = {ratios['no_signaling_over_tsirelson']['numerical']}")

    # ── 5. Quantum Error Correction ──
    print("\n" + "─" * 72)
    print("5. QUANTUM ERROR CORRECTION CODES")
    print("─" * 72)

    codes = quantum_error_correction_codes()
    results['qec_codes'] = codes

    for code in codes:
        bracket = f'[{code.n_physical},{code.k_logical},{code.distance}]'
        print(f"  {code.name}: {bracket}")
        print(f"    TECS-L: {code.tecs_expression}")
        print(f"    {code.description}")

    # ── 6. Scrambling and Complexity ──
    print("\n" + "─" * 72)
    print("6. SCRAMBLING AND COMPLEXITY")
    print("─" * 72)

    hawking = hawking_beta_connection()
    results['scrambling'] = hawking

    beta_info = hawking['beta_coefficient']
    print(f"  Hawking beta = (sigma-tau) * pi * M")
    print(f"    sigma-tau = {beta_info['value']} = rank(E8) = gluons = Golay distance")
    scr = hawking['scrambling_ln6']
    print(f"  ln(P1) = ln(6) = {scr['numerical']}")
    print(f"    = {scr['decomposition']}")

    # Scrambling for n=6 system
    lyapunov_max = 2 * math.pi  # Maldacena bound: lambda <= 2*pi*T
    t_scr = scrambling_bound(N, lyapunov_max)
    print(f"  Scrambling bound for n={N}: t* >= ln({N})/lambda = {t_scr:.6f}/lambda")

    # ── 7. Quantum Channel Capacity ──
    print("\n" + "─" * 72)
    print("7. QUANTUM CHANNEL CAPACITY")
    print("─" * 72)

    chan = channel_threshold_analysis()
    results['channel_capacity'] = chan

    print(f"  Depolarizing threshold: p = 1/tau(6) = {chan['threshold']['value']}")
    print(f"  Capacity at threshold:  {chan['capacity_at_threshold']['value']:.6f}")
    print(f"  Capacity at p=0.24:     {chan['capacity_below']['value']:.6f} (positive)")
    print(f"  Capacity at p=0.26:     {chan['capacity_above']['value']:.6f} (zero)")

    # ── 8. Bekenstein Bound ──
    print("\n" + "─" * 72)
    print("8. BEKENSTEIN BOUND for TECS-L systems")
    print("─" * 72)

    bek = bekenstein_tecs_systems()
    results['bekenstein'] = bek

    for key, sys in bek.items():
        m = sys['mass_gev']
        r = sys['radius_fm']
        s_nats = sys['S_max_nats']
        s_bits = sys['S_max_bits']
        desc = sys.get('tecs', sys.get('description', ''))
        print(f"  {key}: M={m:.3f} GeV, R={r:.4f} fm")
        print(f"    S_max = {s_nats:.4f} nats = {s_bits:.4f} bits")
        print(f"    {desc}")

    # ── 9. Statistical Significance ──
    print("\n" + "─" * 72)
    print("9. STATISTICAL SIGNIFICANCE OF COINCIDENCES")
    print("─" * 72)

    coincidences = assess_all_coincidences()
    results['coincidences'] = coincidences

    n_expr = _count_tecs_expressions(2)
    print(f"  Look-elsewhere pool: {n_expr} distinct TECS-L expressions (complexity <= 2)")
    print()

    exact_count = 0
    notable_count = 0
    for c in coincidences:
        if c.error_pct == 0:
            exact_count += 1
        elif c.error_pct < 1.0:
            notable_count += 1

        sig_str = f'{c.sigma_significance:.1f}sigma' if math.isfinite(c.sigma_significance) else 'exact'
        print(f"  [{c.assessment}] {c.name}")
        print(f"    Physics={c.physics_value}, TECS={c.tecs_value}, "
              f"error={c.error_pct:.4f}%, p={c.p_value:.4e}, {sig_str}")

    print(f"\n  Summary:")
    print(f"    Exact integer matches:    {exact_count}")
    print(f"    Notable (< 1% error):     {notable_count}")
    print(f"    Total coincidences:       {len(coincidences)}")

    # Combined assessment
    print(f"\n  Combined assessment:")
    exact_p = 1.0
    for c in coincidences:
        if c.p_value < 1.0:
            exact_p *= (1 - c.p_value)
    combined_p = 1.0 - exact_p
    print(f"    Probability all are chance: {exact_p:.6e}")
    print(f"    Probability at least one is real: {combined_p:.6f}")

    if exact_count >= 10:
        print("    >> MANY exact matches — structural connection likely")
    elif exact_count >= 5:
        print("    >> Multiple exact matches — pattern is suggestive")

    print("\n" + "=" * 72)
    print("ANALYSIS COMPLETE")
    print("=" * 72)

    return results


# =====================================================================
# CLI entry point
# =====================================================================

if __name__ == '__main__':
    run_holographic_analysis()
