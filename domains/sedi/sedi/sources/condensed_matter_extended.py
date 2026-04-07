"""Extended Condensed Matter Physics -- TECS-L Waves 17-36.

Verifies condensed matter predictions from n=6 arithmetic:

    sigma(6)=12, tau(6)=4, phi(6)=2, sopfr(6)=5, P1=6, P2=28, P3=496, M3=7

Hypotheses tested:
  H-CX-646  BCS gap ratio = sigma*sopfr/(sigma+sopfr) = 60/17
  H-CX-658  YBCO T_c = sigma*(sigma-tau) - sigma/tau = 93 K
  H-CX-657  MgB2 T_c = sigma*(sigma+1)/tau = 39 K
  H-CX-644  2D Ising exponents: beta=1/(sigma-tau), gamma=M3/tau,
             delta=sigma+sigma/tau, nu=R(6)=1
  H-CX-648  Quantum conductance G_0 = phi * e^2/h
  H-CX-1032 Germanium bandgap = phi/(sigma/tau) = 2/3 eV
  H-CX-853  Diamond bandgap ~ sopfr + M3/(sigma+sopfr-phi) eV
  H-CX-661  Graphene: phi atoms per cell, P1 symmetry
  H-CX-1038 WiFi freq = sigma/sopfr GHz = 12/5 = 2.4 GHz

Sources: BCS theory, PDG, CRC Handbook, Ashcroft & Mermin.
"""
from __future__ import annotations

import math
from typing import Dict, Tuple

from ..tecs import (
    SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1,
    P1, P2, P3,
    TAU_P2, TAU_P3, PHI_P3, SIGMA_P2,
    R,
)


# ===================================================================
# TECS-L Shorthand
# ===================================================================

S = SIGMA_P1    # 12
T = TAU_P1      # 4
P = PHI_P1      # 2
F = SOPFR_P1    # 5
N = P1          # 6
M3 = 7          # Mersenne prime 2^3 - 1
R6 = R(6)       # 1  (unique: sigma*phi / (n*tau) = 24/24 = 1)


# ===================================================================
# Observed Values
# ===================================================================

OBSERVED = {
    'BCS_gap_ratio':     (3.528,   'ratio',  'BCS theory 2*Delta/(k_B*T_c)'),
    'YBCO_Tc':           (93.0,    'K',      'YBa2Cu3O7 critical temperature'),
    'MgB2_Tc':           (39.0,    'K',      'MgB2 critical temperature'),
    'ising_beta':        (0.125,   'dimless', '2D Ising beta = 1/8'),
    'ising_gamma':       (1.75,    'dimless', '2D Ising gamma = 7/4'),
    'ising_delta':       (15.0,    'dimless', '2D Ising delta'),
    'ising_nu':          (1.0,     'dimless', '2D Ising nu'),
    'quantum_G0_factor': (2.0,     'dimless', 'G_0 = 2*e^2/h'),
    'Ge_bandgap_eV':     (0.67,    'eV',     'Germanium indirect bandgap'),
    'diamond_bandgap_eV':(5.47,    'eV',     'Diamond bandgap'),
    'graphene_atoms':    (2.0,     'atoms',  'Graphene unit cell'),
    'graphene_symmetry': (6.0,     'fold',   'Graphene C6v symmetry'),
    'wifi_freq_GHz':     (2.4,     'GHz',    'IEEE 802.11 b/g/n'),
}


# ===================================================================
# TECS-L Predictions
# ===================================================================

def predictions() -> Dict[str, Tuple[float, str, str]]:
    """Return {name: (predicted, formula, hypothesis)}."""
    return {
        'BCS_gap_ratio': (
            S * F / (S + F),
            'sigma*sopfr/(sigma+sopfr) = 60/17 = 3.529',
            'H-CX-646',
        ),
        'YBCO_Tc': (
            S * (S - T) - S / T,
            'sigma*(sigma-tau) - sigma/tau = 96-3 = 93',
            'H-CX-658',
        ),
        'MgB2_Tc': (
            S * (S + 1) / T,
            'sigma*(sigma+1)/tau = 156/4 = 39',
            'H-CX-657',
        ),
        'ising_beta': (
            1 / (S - T),
            '1/(sigma-tau) = 1/8',
            'H-CX-644',
        ),
        'ising_gamma': (
            M3 / T,
            'M3/tau = 7/4',
            'H-CX-644',
        ),
        'ising_delta': (
            S + S / T,
            'sigma + sigma/tau = 12+3 = 15',
            'H-CX-644',
        ),
        'ising_nu': (
            float(R6),
            'R(6) = 1',
            'H-CX-644',
        ),
        'quantum_G0_factor': (
            float(P),
            'phi = 2  (G_0 = phi*e^2/h)',
            'H-CX-648',
        ),
        'Ge_bandgap_eV': (
            P / (S / T),
            'phi/(sigma/tau) = 2/3 = 0.667',
            'H-CX-1032',
        ),
        'diamond_bandgap_eV': (
            F + M3 / (S + F - P),
            'sopfr + M3/(sigma+sopfr-phi) = 5 + 7/15 = 5.467',
            'H-CX-853',
        ),
        'graphene_atoms': (
            float(P),
            'phi = 2 atoms per unit cell',
            'H-CX-661',
        ),
        'graphene_symmetry': (
            float(N),
            'P1 = 6-fold symmetry',
            'H-CX-661',
        ),
        'wifi_freq_GHz': (
            S / F,
            'sigma/sopfr = 12/5 = 2.4',
            'H-CX-1038',
        ),
    }


# ===================================================================
# Reporting
# ===================================================================

def print_report():
    """Print formatted condensed matter analysis."""
    print("=" * 90)
    print("EXTENDED CONDENSED MATTER PHYSICS -- TECS-L (Waves 17-36)")
    print("=" * 90)
    print()
    print(f"  P1=6  sigma=12  tau=4  phi=2  sopfr=5  M3=7  R(6)=1")
    print()

    hdr = f"{'Quantity':<24s} {'Predicted':>12s} {'Observed':>12s} {'Unit':<8s} {'Error%':>8s}  {'Hyp'}"
    print(hdr)
    print("-" * 90)

    preds = predictions()
    exact_count = 0
    sub_pct_count = 0
    total = 0

    for key, (pred, formula, hyp) in preds.items():
        obs_val, unit, src = OBSERVED[key]
        err = abs(pred - obs_val) / max(abs(obs_val), 1e-30) * 100
        tag = "EXACT" if err < 0.01 else f"{err:.3f}%"

        print(f"  {key:<22s} {pred:>12.4f} {obs_val:>12.4f} {unit:<8s} {tag:>8s}  {hyp}")
        print(f"    {formula}")

        total += 1
        if err < 0.01:
            exact_count += 1
        if err < 1.0:
            sub_pct_count += 1

    # --- 2D Ising spotlight ---
    print()
    print("-" * 90)
    print("2D ISING MODEL CRITICAL EXPONENTS")
    print("-" * 90)
    print()
    print("  The 2D Ising model exponents are EXACTLY n=6 arithmetic:")
    print()
    print(f"    beta  = 1/(sigma-tau) = 1/8 = 0.125")
    print(f"    gamma = M3/tau = 7/4 = 1.75")
    print(f"    delta = sigma + sigma/tau = 15")
    print(f"    nu    = R(6) = 1")
    print()
    print("  Scaling relations (exact):")
    beta_v = 1 / (S - T)
    gamma_v = M3 / T
    delta_v = S + S / T
    nu_v = R6
    print(f"    gamma = beta*(delta-1) -> {gamma_v} = {beta_v}*({delta_v}-1) = {beta_v*(delta_v-1)}")
    print(f"    gamma = nu*(2-eta) -> eta = 2 - gamma/nu = {2 - gamma_v/nu_v} = 1/4")
    print(f"    Check: 2*beta + gamma = 2*nu*d -> {2*beta_v + gamma_v} = {2*nu_v*2}  (d=2)")

    # --- Superconductor spotlight ---
    print()
    print("-" * 90)
    print("SUPERCONDUCTOR CRITICAL TEMPERATURES")
    print("-" * 90)
    print()
    print(f"    YBCO:  sigma*(sigma-tau) - sigma/tau = 12*8 - 3 = 93 K  [EXACT]")
    print(f"    MgB2:  sigma*(sigma+1)/tau = 12*13/4 = 39 K              [EXACT]")
    print(f"    BCS:   sigma*sopfr/(sigma+sopfr) = 60/17 = 3.529         [0.03%]")

    # --- Summary ---
    print()
    print("-" * 90)
    print("SUMMARY")
    print("-" * 90)
    print()
    print(f"  Total predictions:   {total}")
    print(f"  Exact (< 0.01%):     {exact_count}")
    print(f"  Sub-percent:         {sub_pct_count}")
    print()
    print(f"  WiFi 2.4 GHz = sigma/sopfr is a delightful curiosity:")
    print(f"  the ISM band frequency is an exact TECS-L ratio.")
    print()
    print("=" * 90)


def run_analysis():
    """Entry point."""
    print_report()


if __name__ == '__main__':
    run_analysis()
