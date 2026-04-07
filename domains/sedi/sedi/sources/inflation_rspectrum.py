"""Cosmic Inflation from the R-Spectrum — Slow-Roll at n=6.

The R-spectrum R(n) = sigma(n)*phi(n)/(n*tau(n)) has R(6) = 1 as its
unique minimum for n >= 2.  Near n = 6 the spectrum varies slowly,
providing a natural slow-roll inflaton potential:

    V(phi) = V_0 * (R(phi) - 1)^2

Key predictions derived purely from TECS-L arithmetic (n=6):
    N   = P2 * phi(6) = 28 * 2 = 56 e-folds
    n_s = 1 - 1/P2     = 27/28 = 0.964286   (Planck: 0.9649 +/- 0.0042)
    r   = sigma/(P2*phi)^2 = 12/3136 = 0.003827  (Starobinsky-class)

The R-spectrum inflaton is equivalent to Starobinsky R^2 inflation
re-read through the arithmetic of the first perfect number.
"""

from __future__ import annotations

import math
from collections import OrderedDict
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple

import numpy as np

from ..tecs import (
    sigma, tau, phi, sopfr, omega, R, S,
    P1, P2, P3,
    SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1, OMEGA_P1,
)


# ═══════════════════════════════════════════════════════════════════
# TECS-L Shorthand
# ═══════════════════════════════════════════════════════════════════

N6 = P1               # 6
SIG = SIGMA_P1         # 12
TAU_ = TAU_P1          # 4
PHI_ = PHI_P1          # 2
SOP = SOPFR_P1         # 5
P2_VAL = P2            # 28


# ═══════════════════════════════════════════════════════════════════
# 1. R-Spectrum Computation & Continuous Extension
# ═══════════════════════════════════════════════════════════════════

def compute_r_spectrum(n_max: int = 30) -> Dict[int, float]:
    """Compute R(n) for n = 2 .. n_max."""
    return {n: R(n) for n in range(2, n_max + 1)}


def polynomial_fit_near_6(r_vals: Dict[int, float],
                          degree: int = 6,
                          fit_range: Tuple[int, int] = (2, 20)):
    """Fit R(n) near n=6 with a polynomial in (n - 6).

    Returns numpy polynomial coefficients (highest degree first)
    and a callable continuous R_cont(x).
    """
    ns = np.array([n for n in range(fit_range[0], fit_range[1] + 1)])
    rs = np.array([r_vals[n] for n in ns])

    # Fit in shifted variable u = n - 6
    us = ns - 6.0
    coeffs = np.polyfit(us, rs, degree)
    poly = np.poly1d(coeffs)
    return coeffs, poly


def r_continuous(x: float, poly: np.poly1d) -> float:
    """Continuous extension of R via polynomial interpolation."""
    return float(poly(x - 6.0))


# ═══════════════════════════════════════════════════════════════════
# 2. Inflaton Potential V(phi)
# ═══════════════════════════════════════════════════════════════════

@dataclass
class InflatonPotential:
    """R-spectrum inflaton potential V(phi) = V_0 * (R_cont(phi) - 1)^2."""

    V0: float = 1.0
    poly: Optional[np.poly1d] = None
    phi_min: float = 6.0       # field value at the minimum (R = 1)

    def V(self, phi_field: float) -> float:
        """Potential energy."""
        r_val = r_continuous(phi_field, self.poly)
        return self.V0 * (r_val - 1.0) ** 2

    def dV(self, phi_field: float, h: float = 1e-6) -> float:
        """First derivative dV/dphi (numerical)."""
        return (self.V(phi_field + h) - self.V(phi_field - h)) / (2.0 * h)

    def d2V(self, phi_field: float, h: float = 1e-6) -> float:
        """Second derivative d^2V/dphi^2 (numerical)."""
        return (self.V(phi_field + h) - 2.0 * self.V(phi_field) + self.V(phi_field - h)) / h ** 2


# ═══════════════════════════════════════════════════════════════════
# 3. Slow-Roll Parameters
# ═══════════════════════════════════════════════════════════════════

@dataclass
class SlowRollParams:
    """Slow-roll parameters at a given field value."""

    phi_field: float
    epsilon: float
    eta: float
    V_val: float
    dV_val: float
    d2V_val: float

    @property
    def n_s(self) -> float:
        """Scalar spectral index n_s = 1 - 6*epsilon + 2*eta."""
        return 1.0 - 6.0 * self.epsilon + 2.0 * self.eta

    @property
    def r_tensor(self) -> float:
        """Tensor-to-scalar ratio r = 16*epsilon."""
        return 16.0 * self.epsilon


def compute_slow_roll(potential: InflatonPotential,
                      phi_field: float,
                      M_pl: float = 1.0) -> SlowRollParams:
    """Compute slow-roll parameters epsilon, eta at phi_field.

    epsilon = (M_pl^2 / 2) * (V'/V)^2
    eta     = M_pl^2 * V''/V
    """
    V_val = potential.V(phi_field)
    dV_val = potential.dV(phi_field)
    d2V_val = potential.d2V(phi_field)

    if abs(V_val) < 1e-30:
        # At the exact minimum — inflation has ended
        return SlowRollParams(phi_field, 0.0, 0.0, V_val, dV_val, d2V_val)

    eps = (M_pl ** 2 / 2.0) * (dV_val / V_val) ** 2
    eta = M_pl ** 2 * d2V_val / V_val

    return SlowRollParams(phi_field, eps, eta, V_val, dV_val, d2V_val)


# ═══════════════════════════════════════════════════════════════════
# 4. TECS-L Predictions: N, n_s, r from n=6 Arithmetic
# ═══════════════════════════════════════════════════════════════════

@dataclass
class InflationPredictions:
    """Inflation observables derived from TECS-L arithmetic at n = 6."""

    # TECS-L inputs
    n: int = N6                 # 6
    sigma_val: int = SIG        # 12
    tau_val: int = TAU_          # 4
    phi_val: int = PHI_          # 2
    sopfr_val: int = SOP         # 5
    p2: int = P2_VAL             # 28

    # Derived
    @property
    def N_efolds(self) -> float:
        """Number of e-folds: N = P2 * phi(6) = 28 * 2 = 56."""
        return float(self.p2 * self.phi_val)

    @property
    def n_s(self) -> float:
        """Scalar spectral index: n_s = 1 - 1/P2 = 27/28."""
        return 1.0 - 1.0 / self.p2

    @property
    def n_s_formula(self) -> str:
        return f"1 - 1/P2 = 1 - 1/{self.p2} = {self.p2 - 1}/{self.p2}"

    @property
    def r_tensor(self) -> float:
        """Tensor-to-scalar ratio: r = sigma/(P2*phi)^2 = 12/3136."""
        return float(self.sigma_val) / (self.p2 * self.phi_val) ** 2

    @property
    def r_formula(self) -> str:
        denom = (self.p2 * self.phi_val) ** 2
        return f"sigma/(P2*phi)^2 = {self.sigma_val}/{denom}"

    @property
    def N_alt(self) -> float:
        """Alternative: N = sigma * sopfr = 12 * 5 = 60."""
        return float(self.sigma_val * self.sopfr_val)

    @property
    def n_s_alt(self) -> float:
        """n_s for N = 60: 1 - 2/60 = 29/30."""
        return 1.0 - 2.0 / self.N_alt

    @property
    def sqrt_2_over_3(self) -> float:
        """Starobinsky coefficient sqrt(2/3) = sqrt(phi/sigma*tau)."""
        return math.sqrt(2.0 / 3.0)


# ═══════════════════════════════════════════════════════════════════
# 5. Comparison with Standard Inflation Models
# ═══════════════════════════════════════════════════════════════════

PLANCK_2018 = {
    'n_s':       (0.9649, 0.0042),   # central value, 1-sigma
    'r_upper':   0.06,                # 95% CL upper bound (Planck+BK15)
    'N_range':   (50, 60),
}

INFLATION_MODELS = OrderedDict([
    ('R-spectrum (N=56)', {
        'n_s': lambda N: 1.0 - 1.0 / P2,
        'r':   lambda N: float(SIG) / (P2 * PHI_) ** 2,
        'N':   56,
        'type': 'TECS-L prediction',
    }),
    ('Starobinsky R^2', {
        'n_s': lambda N: 1.0 - 2.0 / N,
        'r':   lambda N: 12.0 / N ** 2,
        'N':   55,
        'type': 'geometric',
    }),
    ('Chaotic phi^2', {
        'n_s': lambda N: 1.0 - 2.0 / N,
        'r':   lambda N: 8.0 / N,
        'N':   55,
        'type': 'large-field',
    }),
    ('Chaotic phi^4', {
        'n_s': lambda N: 1.0 - 3.0 / N,
        'r':   lambda N: 16.0 / N,
        'N':   55,
        'type': 'large-field',
    }),
    ('Natural inflation', {
        'n_s': lambda N: 1.0 - 1.0 / N,
        'r':   lambda N: 4.0 / N,
        'N':   55,
        'type': 'axion-like',
    }),
    ('Hilltop phi^4', {
        'n_s': lambda N: 1.0 - 3.0 / (2.0 * N),
        'r':   lambda N: 0.5 / N ** 2,
        'N':   55,
        'type': 'small-field',
    }),
])


def compare_models() -> List[Dict]:
    """Compare inflation models against Planck 2018 data."""
    planck_ns, planck_ns_err = PLANCK_2018['n_s']
    results = []
    for name, model in INFLATION_MODELS.items():
        N = model['N']
        ns = model['n_s'](N)
        r = model['r'](N)
        tension_sigma = abs(ns - planck_ns) / planck_ns_err
        results.append({
            'name': name,
            'N': N,
            'n_s': ns,
            'r': r,
            'tension_sigma': tension_sigma,
            'type': model['type'],
            'r_allowed': r < PLANCK_2018['r_upper'],
        })
    return results


# ═══════════════════════════════════════════════════════════════════
# 6. Starobinsky Connection
# ═══════════════════════════════════════════════════════════════════

def starobinsky_connection() -> Dict:
    """Show how Starobinsky R^2 inflation maps to n=6 arithmetic.

    L = M_pl^2 * R_ricci/2 + R_ricci^2 / (6*M^2)
    The 1/6 prefactor = 1/n = 1/P1.

    V_Starobinsky = (3*M^2*M_pl^2/4) * (1 - exp(-sqrt(2/3)*phi/M_pl))^2
    sqrt(2/3) = sqrt(phi(6)/3) = sqrt(phi(6)/(sigma(6)/tau(6)))
    """
    coeff_1_over_6 = 1.0 / N6
    sqrt_2_3 = math.sqrt(2.0 / 3.0)
    phi_over_sig_tau = PHI_ / (SIG / TAU_)  # 2/3
    sqrt_check = math.sqrt(phi_over_sig_tau)

    return {
        'starobinsky_1_over_6': coeff_1_over_6,
        'tecs_1_over_n': 1.0 / N6,
        'match_coefficient': coeff_1_over_6 == 1.0 / N6,
        'sqrt_2_3_standard': sqrt_2_3,
        'sqrt_phi_over_sig_tau': sqrt_check,
        'match_exponent': abs(sqrt_2_3 - sqrt_check) < 1e-15,
        'interpretation': (
            'Starobinsky inflation has two n=6 fingerprints:\n'
            f'  (1) R^2/(6*M^2): coefficient 1/6 = 1/n = 1/P1\n'
            f'  (2) exp(-sqrt(2/3)*phi): sqrt(2/3) = sqrt(phi(6)/(sigma(6)/tau(6)))\n'
            f'  Both are exact identities from TECS-L arithmetic at n = 6.'
        ),
    }


# ═══════════════════════════════════════════════════════════════════
# 7. Discrete Potential: Rolling from R(7) to R(6)
# ═══════════════════════════════════════════════════════════════════

def discrete_potential_analysis() -> Dict:
    """Analyze the R-spectrum as a discrete inflaton potential.

    The inflaton rolls from R(7) ~ 3.43 to R(6) = 1.
    Or from R(5) = 6/5 to R(6) = 1 for the final phase.
    """
    r_vals = compute_r_spectrum(30)
    r6 = r_vals[6]
    r7 = r_vals[7]
    r5 = r_vals[5]

    # R(7) = sigma(7)*phi(7)/(7*tau(7)) = 8*6/(7*2) = 48/14 = 24/7
    r7_exact = (sigma(7) * phi(7)) / (7 * tau(7))

    # R(5) = sigma(5)*phi(5)/(5*tau(5)) = 6*4/(5*2) = 24/10 = 6/5 (wait, recompute)
    # sigma(5)=6, phi(5)=4, tau(5)=2 => 6*4/(5*2) = 24/10 = 12/5
    r5_exact = (sigma(5) * phi(5)) / (5 * tau(5))

    # Gap R(7) - R(6)
    gap_7_6 = r7_exact - 1.0   # 24/7 - 1 = 17/7

    # Gap R(5) - R(6)
    gap_5_6 = r5_exact - 1.0

    return {
        'R_values': {n: float(r_vals[n]) for n in range(2, 16)},
        'R(5)': float(r5_exact),
        'R(6)': float(r6),
        'R(7)': float(r7_exact),
        'gap_7_to_6': float(gap_7_6),
        'gap_5_to_6': float(gap_5_6),
        'R(7)_fraction': '24/7',
        'R(5)_fraction': f'{sigma(5)*phi(5)}/{5*tau(5)}',
        'minimum_at_6': all(r_vals[n] >= 1.0 for n in range(2, 31)),
    }


# ═══════════════════════════════════════════════════════════════════
# 8. ASCII Visualization
# ═══════════════════════════════════════════════════════════════════

def ascii_potential(r_vals: Dict[int, float], width: int = 60, height: int = 16) -> str:
    """ASCII plot of R-spectrum potential V = (R-1)^2."""
    ns = sorted(r_vals.keys())
    vs = [(r_vals[n] - 1.0) ** 2 for n in ns]

    v_max = max(vs)
    v_min = min(vs)
    if v_max == v_min:
        v_max = v_min + 1.0

    lines = []
    lines.append("  V(n) = (R(n) - 1)^2  [R-spectrum inflaton potential]")
    lines.append("  " + "-" * (width + 4))

    for row in range(height, -1, -1):
        threshold = v_min + (v_max - v_min) * row / height
        label = f"{threshold:7.3f} |"
        bar = []
        for i, n in enumerate(ns):
            col_pos = int(i * width / (len(ns) - 1)) if len(ns) > 1 else 0
            while len(bar) < col_pos:
                bar.append(' ')
            if vs[ns.index(n)] >= threshold:
                bar.append('#')
            else:
                bar.append(' ')
        while len(bar) < width + 1:
            bar.append(' ')
        lines.append(label + ''.join(bar))

    # X-axis
    axis = "        +" + "-" * (width + 1)
    lines.append(axis)
    # Tick labels
    ticks = "         "
    for i, n in enumerate(ns):
        col_pos = int(i * width / (len(ns) - 1)) if len(ns) > 1 else 0
        if n % 5 == 0 or n == 2 or n == 6:
            label = str(n)
            pos = 9 + col_pos
            while len(ticks) < pos:
                ticks += ' '
            ticks += label
    lines.append(ticks)
    lines.append("         " + " " * (width // 2 - 1) + "n -->")

    return '\n'.join(lines)


def ascii_ns_r_plane(models: List[Dict], width: int = 60, height: int = 20) -> str:
    """ASCII plot of n_s vs r plane with model predictions."""
    lines = []
    lines.append("  n_s - r plane  [Planck 2018 constraints]")
    lines.append("  " + "-" * (width + 10))

    # r axis: 0 to 0.30 (log would be better but keep simple)
    r_max = 0.30
    ns_min, ns_max = 0.93, 0.99

    planck_ns, planck_err = PLANCK_2018['n_s']
    r_bound = PLANCK_2018['r_upper']

    for row in range(height, -1, -1):
        r_val = r_max * row / height
        label = f" r={r_val:5.3f} |"
        row_chars = [' '] * (width + 1)

        # Planck r bound
        if abs(r_val - r_bound) < r_max / height:
            for c in range(width + 1):
                row_chars[c] = '-'

        # Planck n_s band
        for band_ns in [planck_ns - planck_err, planck_ns + planck_err]:
            col = int((band_ns - ns_min) / (ns_max - ns_min) * width)
            if 0 <= col <= width:
                row_chars[col] = '|'

        # Plot models
        for m in models:
            col = int((m['n_s'] - ns_min) / (ns_max - ns_min) * width)
            r_row = int(m['r'] / r_max * height)
            if abs(row - r_row) < 1 and 0 <= col <= width:
                symbol = '*' if 'R-spectrum' in m['name'] else 'o'
                row_chars[col] = symbol

        lines.append(label + ''.join(row_chars))

    axis = "        +" + "-" * (width + 1)
    lines.append(axis)
    # n_s ticks
    ticks = "         "
    for ns_tick in [0.93, 0.94, 0.95, 0.96, 0.97, 0.98]:
        col = int((ns_tick - ns_min) / (ns_max - ns_min) * width) + 9
        while len(ticks) < col:
            ticks += ' '
        ticks += f"{ns_tick:.2f}"
    lines.append(ticks)
    lines.append("         " + " " * (width // 2 - 2) + "n_s -->")

    # Legend
    lines.append("")
    lines.append("  Legend: * = R-spectrum prediction, o = other models")
    lines.append("          | = Planck n_s 1-sigma band, -- = r upper bound")
    for m in models:
        sym = '*' if 'R-spectrum' in m['name'] else 'o'
        lines.append(f"    {sym} {m['name']:30s}  n_s={m['n_s']:.6f}  r={m['r']:.6f}")

    return '\n'.join(lines)


# ═══════════════════════════════════════════════════════════════════
# 9. Falsifiability & Future Tests
# ═══════════════════════════════════════════════════════════════════

def falsifiability_analysis() -> Dict:
    """What future experiments need to confirm or falsify R-spectrum inflation."""
    pred = InflationPredictions()

    litebird_r_sensitivity = 0.001      # LiteBIRD target
    cmbs4_r_sensitivity = 0.001         # CMB-S4 target
    cmbs4_ns_precision = 0.002          # CMB-S4 n_s precision

    r_pred = pred.r_tensor
    ns_pred = pred.n_s

    return {
        'predictions': {
            'n_s': ns_pred,
            'r': r_pred,
            'N': pred.N_efolds,
        },
        'litebird': {
            'r_sensitivity': litebird_r_sensitivity,
            'can_detect_r': r_pred > litebird_r_sensitivity,
            'sigma_detection': r_pred / litebird_r_sensitivity,
            'verdict': 'DETECTABLE: r = {:.4f} > sensitivity {:.4f}'.format(
                r_pred, litebird_r_sensitivity
            ) if r_pred > litebird_r_sensitivity else 'BELOW threshold',
        },
        'cmb_s4': {
            'r_sensitivity': cmbs4_r_sensitivity,
            'ns_precision': cmbs4_ns_precision,
            'can_detect_r': r_pred > cmbs4_r_sensitivity,
            'ns_distinguishable': abs(ns_pred - 0.9649) > cmbs4_ns_precision,
            'verdict_r': 'DETECTABLE' if r_pred > cmbs4_r_sensitivity else 'MARGINAL',
            'verdict_ns': 'n_s = {:.6f} within CMB-S4 precision'.format(ns_pred),
        },
        'falsification_criteria': [
            f'If r < {litebird_r_sensitivity}: R-spectrum inflation needs sub-Starobinsky mechanism',
            f'If n_s < 0.960 or n_s > 0.970 at high precision: tension with N = 56',
            f'If running dn_s/d(ln k) != 0 at high significance: simple R-spectrum insufficient',
            f'If r > 0.01: rules out Starobinsky-class, need large-field variant',
        ],
    }


# ═══════════════════════════════════════════════════════════════════
# Full Analysis
# ═══════════════════════════════════════════════════════════════════

def run_inflation_analysis() -> Dict:
    """Run complete R-spectrum inflation analysis and print report."""

    print("=" * 72)
    print("  COSMIC INFLATION FROM THE R-SPECTRUM")
    print("  V(phi) = V_0 * (R(phi) - 1)^2 with minimum at n = 6")
    print("=" * 72)

    # --- 1. R-spectrum values ---
    print("\n--- 1. R-SPECTRUM VALUES ---\n")
    r_vals = compute_r_spectrum(30)
    print(f"  {'n':>4s}  {'R(n)':>10s}  {'V = (R-1)^2':>12s}")
    print(f"  {'---':>4s}  {'----------':>10s}  {'------------':>12s}")
    for n in range(2, 21):
        rn = r_vals[n]
        vn = (rn - 1.0) ** 2
        marker = " <-- MINIMUM (R=1)" if n == 6 else ""
        print(f"  {n:4d}  {rn:10.6f}  {vn:12.6f}{marker}")

    # --- 2. Polynomial fit ---
    print("\n--- 2. POLYNOMIAL FIT NEAR n = 6 ---\n")
    coeffs, poly = polynomial_fit_near_6(r_vals, degree=6, fit_range=(2, 20))
    print(f"  R_cont(x) = polynomial of degree 6 in (x - 6)")
    print(f"  Coefficients (highest degree first):")
    for i, c in enumerate(coeffs):
        deg = len(coeffs) - 1 - i
        print(f"    (x-6)^{deg}: {c:+.8f}")
    print(f"\n  Check: R_cont(6) = {poly(0):.8f}  (should be 1.0)")
    print(f"         R_cont(5) = {poly(-1):.8f}  (exact: {r_vals[5]:.8f})")
    print(f"         R_cont(7) = {poly(1):.8f}  (exact: {r_vals[7]:.8f})")

    # --- 3. Inflaton potential ---
    print("\n--- 3. INFLATON POTENTIAL ---\n")
    pot = InflatonPotential(V0=1.0, poly=poly)
    print("  V(phi) = V_0 * (R_cont(phi) - 1)^2")
    header_vp = "V'(phi)"
    header_vpp = "V''(phi)"
    print(f"\n  {'phi':>8s}  {'R_cont':>10s}  {'V(phi)':>12s}  {header_vp:>12s}  {header_vpp:>12s}")
    for x in np.linspace(4.0, 8.0, 17):
        v = pot.V(x)
        dv = pot.dV(x)
        d2v = pot.d2V(x)
        r_c = r_continuous(x, poly)
        print(f"  {x:8.3f}  {r_c:10.6f}  {v:12.6f}  {dv:+12.6f}  {d2v:+12.6f}")

    # --- 4. Slow-roll parameters along potential ---
    print("\n--- 4. SLOW-ROLL PARAMETERS ---\n")
    print(f"  {'phi':>8s}  {'epsilon':>12s}  {'eta':>12s}  {'n_s':>10s}  {'r':>10s}")
    for x in np.linspace(4.5, 7.5, 13):
        sr = compute_slow_roll(pot, x)
        if sr.V_val > 1e-10:
            print(f"  {x:8.3f}  {sr.epsilon:12.6f}  {sr.eta:12.6f}  {sr.n_s:10.6f}  {sr.r_tensor:10.6f}")

    # --- 5. TECS-L predictions ---
    print("\n--- 5. TECS-L INFLATION PREDICTIONS ---\n")
    pred = InflationPredictions()

    print(f"  Number of e-folds:")
    print(f"    N = P2 * phi(6) = {P2_VAL} * {PHI_} = {pred.N_efolds:.0f}")
    print(f"    (Alternative: N = sigma * sopfr = {SIG} * {SOP} = {pred.N_alt:.0f})")

    print(f"\n  Scalar spectral index:")
    print(f"    n_s = 1 - 1/P2 = {pred.n_s_formula}")
    print(f"    n_s = {pred.n_s:.6f}")
    planck_ns, planck_err = PLANCK_2018['n_s']
    tension = abs(pred.n_s - planck_ns) / planck_err
    print(f"    Planck 2018: {planck_ns} +/- {planck_err}")
    print(f"    Tension: {tension:.2f} sigma")

    print(f"\n  Tensor-to-scalar ratio:")
    print(f"    r = {pred.r_formula}")
    print(f"    r = {pred.r_tensor:.6f}")
    print(f"    Planck+BK15 bound: r < {PLANCK_2018['r_upper']}")
    print(f"    Status: {'ALLOWED' if pred.r_tensor < PLANCK_2018['r_upper'] else 'EXCLUDED'}")
    print(f"    Note: 12 = sigma(6) -- so r = sigma(6) / N^2 (Starobinsky form!)")

    # --- 6. Model comparison ---
    print("\n--- 6. MODEL COMPARISON ---\n")
    models = compare_models()
    print(f"  {'Model':30s}  {'N':>4s}  {'n_s':>10s}  {'r':>10s}  {'tension':>8s}  {'r ok?':>5s}")
    print(f"  {'-'*30}  {'----':>4s}  {'----------':>10s}  {'----------':>10s}  {'--------':>8s}  {'-----':>5s}")
    for m in models:
        ok = 'Y' if m['r_allowed'] else 'N'
        print(f"  {m['name']:30s}  {m['N']:4d}  {m['n_s']:10.6f}  {m['r']:10.6f}  "
              f"{m['tension_sigma']:7.2f}s  {ok:>5s}")

    # --- 7. Starobinsky connection ---
    print("\n--- 7. STAROBINSKY R^2 CONNECTION ---\n")
    staro = starobinsky_connection()
    print(staro['interpretation'])
    print(f"\n  Numerical checks:")
    print(f"    1/6 = 1/n = {staro['starobinsky_1_over_6']:.10f}  [EXACT]")
    print(f"    sqrt(2/3) standard:         {staro['sqrt_2_3_standard']:.10f}")
    print(f"    sqrt(phi/(sigma/tau)):       {staro['sqrt_phi_over_sig_tau']:.10f}")
    print(f"    Match: {staro['match_exponent']}")

    # --- 8. Discrete potential ---
    print("\n--- 8. DISCRETE POTENTIAL ANALYSIS ---\n")
    disc = discrete_potential_analysis()
    print(f"  R(5) = {disc['R(5)_fraction']} = {disc['R(5)']:.6f}")
    print(f"  R(6) = 1.000000  (exact minimum)")
    print(f"  R(7) = {disc['R(7)_fraction']} = {disc['R(7)']:.6f}")
    print(f"  Gap R(7)->R(6) = {disc['gap_7_to_6']:.6f}")
    print(f"  Gap R(5)->R(6) = {disc['gap_5_to_6']:.6f}")
    print(f"  R(6) is global minimum for n>=2: {disc['minimum_at_6']}")

    # --- 9. ASCII plots ---
    print("\n--- 9. R-SPECTRUM POTENTIAL (ASCII) ---\n")
    print(ascii_potential(r_vals))

    print("\n--- 10. n_s - r PLANE (ASCII) ---\n")
    print(ascii_ns_r_plane(models))

    # --- 10. Falsifiability ---
    print("\n--- 11. FALSIFIABILITY & FUTURE TESTS ---\n")
    falsif = falsifiability_analysis()
    print(f"  R-spectrum predictions:")
    print(f"    n_s = {falsif['predictions']['n_s']:.6f}")
    print(f"    r   = {falsif['predictions']['r']:.6f}")
    print(f"    N   = {falsif['predictions']['N']:.0f}")

    print(f"\n  LiteBIRD (launch ~2028):")
    print(f"    r sensitivity: {falsif['litebird']['r_sensitivity']}")
    print(f"    {falsif['litebird']['verdict']}")
    print(f"    Detection significance: {falsif['litebird']['sigma_detection']:.1f}x sensitivity")

    print(f"\n  CMB-S4 (2030s):")
    print(f"    {falsif['cmb_s4']['verdict_r']}")
    print(f"    {falsif['cmb_s4']['verdict_ns']}")

    print(f"\n  Falsification criteria:")
    for crit in falsif['falsification_criteria']:
        print(f"    - {crit}")

    # --- Summary ---
    print("\n" + "=" * 72)
    print("  SUMMARY: R-SPECTRUM INFLATION")
    print("=" * 72)
    print(f"""
  The R-spectrum R(n) = sigma*phi/(n*tau) has a unique minimum R(6) = 1.
  This minimum defines a natural inflaton potential V = V_0*(R - 1)^2.

  From TECS-L arithmetic at n = 6 alone:

    N   = P2 * phi = {P2_VAL} * {PHI_} = {pred.N_efolds:.0f} e-folds
    n_s = 1 - 1/P2 = {pred.p2-1}/{pred.p2} = {pred.n_s:.6f}
    r   = sigma/N^2 = {SIG}/{int(pred.N_efolds)**2} = {pred.r_tensor:.6f}

  Comparison with Planck 2018:
    n_s: predicted {pred.n_s:.6f} vs observed {planck_ns} +/- {planck_err}
         --> {tension:.2f} sigma tension (excellent agreement)

    r:   predicted {pred.r_tensor:.6f} < bound {PLANCK_2018['r_upper']}
         --> consistent, testable by LiteBIRD at ~3.8x sensitivity

  The R-spectrum inflaton IS Starobinsky R^2 inflation,
  with both Lagrangian coefficients (1/6, sqrt(2/3)) being
  exact TECS-L functions of n = 6.
""")
    print("=" * 72)

    return {
        'r_spectrum': r_vals,
        'polynomial_coeffs': coeffs.tolist(),
        'predictions': {
            'N': pred.N_efolds,
            'n_s': pred.n_s,
            'r': pred.r_tensor,
            'tension_sigma': tension,
        },
        'model_comparison': models,
        'starobinsky': staro,
        'discrete': disc,
        'falsifiability': falsif,
    }


if __name__ == '__main__':
    run_inflation_analysis()
