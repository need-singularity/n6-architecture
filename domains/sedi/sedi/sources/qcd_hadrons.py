"""QCD & Hadron Spectroscopy -- TECS-L Waves 17-36.

Verifies QCD and hadron mass predictions from n=6 arithmetic:

    sigma(6)=12, tau(6)=4, phi(6)=2, sopfr(6)=5, P1=6, P2=28, P3=496, M3=7

Hypotheses tested:
  H-CX-720  sqrt(sigma_QCD) = P3 - sigma*sopfr + tau = 440 MeV
  H-CX-756  Lambda_QCD = P2*sigma - tau = 332 MeV
  H-CX-757  QCD condensate = phi(P3) + tau(P3) = 250 MeV
  H-CX-752  pi+- mass = sigma^2 - tau - phi/(sigma-tau) = 139.75 MeV
  H-CX-887  pi0 mass = sigma^2 - sigma + sigma/tau = 135 MeV
  H-CX-754  K+- mass = P3 - phi = 494 MeV
  H-CX-751  Proton mass ~ P3*phi - sigma*tau - P1 = 938 MeV
  H-CX-761  Glueball mass = sqrt(sigma/tau) GeV = sqrt(3) GeV
  H-CX-721  Color factors: C_F=4/3, C_A=3, T_F=1/2
  H-CX-722  beta_0(n_f=6) = M3 = 7

Includes Monte Carlo Texas Sharpshooter test.

Sources: PDG 2022, FLAG 2021, lattice QCD.
"""
from __future__ import annotations

import math
import random
from typing import Dict, List, Tuple

from ..tecs import (
    SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1,
    P1, P2, P3,
    TAU_P2, TAU_P3, PHI_P3, SIGMA_P2,
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


# ===================================================================
# Observed Values (PDG 2022 / lattice QCD)
# ===================================================================

OBSERVED_MASSES = {
    'sqrt_sigma_QCD':  (440.0,    'MeV', 'Lattice QCD string tension'),
    'Lambda_QCD':      (332.0,    'MeV', 'FLAG 2021 MS-bar Nf=3'),
    'QCD_condensate':  (250.0,    'MeV', '<qq>^{1/3} FLAG 2021'),
    'pi_pm_mass':      (139.570,  'MeV', 'PDG 2022'),
    'pi_0_mass':       (134.977,  'MeV', 'PDG 2022'),
    'K_pm_mass':       (493.677,  'MeV', 'PDG 2022'),
    'proton_mass':     (938.272,  'MeV', 'PDG 2022'),
    'glueball_mass':   (1730.0,   'MeV', 'Lattice: 0++ scalar glueball'),
}

OBSERVED_QCD = {
    'C_F':        (4/3,   'dimless', 'SU(3) Casimir'),
    'C_A':        (3.0,   'dimless', 'SU(3) adjoint Casimir'),
    'T_F':        (0.5,   'dimless', 'SU(3) fundamental index'),
    'beta_0_nf6': (7.0,   'dimless', '(11*C_A - 4*T_F*n_f)/(4*pi)... leading coeff'),
}


# ===================================================================
# TECS-L Predictions
# ===================================================================

def mass_predictions() -> Dict[str, Tuple[float, str, str]]:
    """Return {name: (predicted, formula, hypothesis)}."""
    return {
        'sqrt_sigma_QCD': (
            P3 - S * F + T,
            'P3 - sigma*sopfr + tau = 496-60+4',
            'H-CX-720',
        ),
        'Lambda_QCD': (
            P2 * S - T,
            'P2*sigma - tau = 336-4',
            'H-CX-756',
        ),
        'QCD_condensate': (
            PHI_P3 + TAU_P3,
            'phi(P3) + tau(P3) = 240+10',
            'H-CX-757',
        ),
        'pi_pm_mass': (
            S**2 - T - P / (S - T),
            'sigma^2 - tau - phi/(sigma-tau) = 144-4-0.25',
            'H-CX-752',
        ),
        'pi_0_mass': (
            S**2 - S + S / T,
            'sigma^2 - sigma + sigma/tau = 144-12+3',
            'H-CX-887',
        ),
        'K_pm_mass': (
            P3 - P,
            'P3 - phi = 496-2',
            'H-CX-754',
        ),
        'proton_mass': (
            P3 * P - S * T - N,
            'P3*phi - sigma*tau - P1 = 992-48-6',
            'H-CX-751',
        ),
        'glueball_mass': (
            math.sqrt(S / T) * 1000,
            'sqrt(sigma/tau) GeV = sqrt(3) GeV',
            'H-CX-761',
        ),
    }


def qcd_predictions() -> Dict[str, Tuple[float, str, str]]:
    """Return QCD structure predictions."""
    return {
        'C_F': (
            T / (S / T),
            'tau/(sigma/tau) = 4/3',
            'H-CX-721',
        ),
        'C_A': (
            S / T,
            'sigma/tau = 3',
            'H-CX-721',
        ),
        'T_F': (
            P / T,
            'phi/tau = 1/2',
            'H-CX-721',
        ),
        'beta_0_nf6': (
            float(M3),
            'M3 = 7',
            'H-CX-722',
        ),
    }


# ===================================================================
# Monte Carlo Texas Sharpshooter Test
# ===================================================================

def texas_sharpshooter(n_trials: int = 100_000, seed: int = 42) -> Dict:
    """Test whether TECS-L matches to hadron masses could arise by chance.

    For each trial, generate random 'predictions' from the same set of
    constants {S,T,P,F,N,M3,P2,P3} using random 2-3 term arithmetic,
    and count how many land within 1% of an observed mass.
    """
    rng = random.Random(seed)

    consts = [S, T, P, F, N, M3, P2, P3]
    ops = [
        lambda a, b: a + b,
        lambda a, b: a - b,
        lambda a, b: a * b,
        lambda a, b: a / b if b != 0 else None,
    ]

    obs_vals = [v for v, _, _ in OBSERVED_MASSES.values()]

    def random_expr():
        """Build a random 2-3 term expression from TECS constants."""
        a = rng.choice(consts)
        b = rng.choice(consts)
        op = rng.choice(ops)
        r = op(a, b)
        if r is None or abs(r) > 1e6:
            return None
        # 50% chance of a third operation
        if rng.random() < 0.5:
            c = rng.choice(consts)
            op2 = rng.choice(ops)
            r = op2(r, c)
            if r is None or abs(r) > 1e6:
                return None
        return r

    # Our actual match count: predictions within 1% of observed
    actual_preds = mass_predictions()
    actual_hits = 0
    for key, (pred, _, _) in actual_preds.items():
        obs_val = OBSERVED_MASSES[key][0]
        if abs(pred - obs_val) / obs_val < 0.01:
            actual_hits += 1

    # Monte Carlo: for each trial, generate len(actual_preds) random expressions
    # and count how many land within 1% of ANY observed mass
    n_preds = len(actual_preds)
    random_hit_counts = []

    for _ in range(n_trials):
        hits = 0
        for _ in range(n_preds):
            val = random_expr()
            if val is not None and val > 0:
                for obs in obs_vals:
                    if abs(val - obs) / obs < 0.01:
                        hits += 1
                        break
        random_hit_counts.append(hits)

    mean_random = sum(random_hit_counts) / n_trials
    p_geq = sum(1 for c in random_hit_counts if c >= actual_hits) / n_trials

    return {
        'actual_hits': actual_hits,
        'n_predictions': n_preds,
        'mc_mean': mean_random,
        'p_geq_observed': p_geq,
        'n_trials': n_trials,
    }


# ===================================================================
# Reporting
# ===================================================================

def print_report():
    """Print formatted QCD/hadron analysis."""
    print("=" * 90)
    print("QCD & HADRON SPECTROSCOPY -- TECS-L PREDICTIONS (Waves 17-36)")
    print("=" * 90)
    print()
    print(f"  P1=6  sigma=12  tau=4  phi=2  sopfr=5  M3=7  P2=28  P3=496")
    print(f"  phi(P3)=240  tau(P3)=10")
    print()

    # --- Mass predictions ---
    print("-" * 90)
    print("HADRON & QCD SCALE PREDICTIONS")
    print("-" * 90)
    hdr = f"{'Quantity':<22s} {'Predicted':>12s} {'Observed':>12s} {'Unit':<6s} {'Error%':>8s}  {'Hyp'}"
    print(hdr)
    print("-" * 90)

    mpreds = mass_predictions()
    for key, (pred, formula, hyp) in mpreds.items():
        obs_val, unit, src = OBSERVED_MASSES[key]
        err = abs(pred - obs_val) / obs_val * 100
        print(f"  {key:<20s} {pred:>12.3f} {obs_val:>12.3f} {unit:<6s} {err:>7.3f}%  {hyp}")
        print(f"    {formula}")

    # --- QCD structure ---
    print()
    print("-" * 90)
    print("QCD COLOR ALGEBRA")
    print("-" * 90)
    hdr = f"{'Quantity':<22s} {'Predicted':>12s} {'Observed':>12s} {'Unit':<8s} {'Error%':>8s}  {'Hyp'}"
    print(hdr)
    print("-" * 90)

    qpreds = qcd_predictions()
    for key, (pred, formula, hyp) in qpreds.items():
        obs_val, unit, src = OBSERVED_QCD[key]
        err = abs(pred - obs_val) / max(obs_val, 1e-30) * 100
        tag = "EXACT" if err < 0.001 else f"{err:.3f}%"
        print(f"  {key:<20s} {pred:>12.6f} {obs_val:>12.6f} {unit:<8s} {tag:>8s}  {hyp}")
        print(f"    {formula}")

    # --- Texas Sharpshooter ---
    print()
    print("-" * 90)
    print("MONTE CARLO TEXAS SHARPSHOOTER TEST")
    print("-" * 90)
    print()
    print("  Null hypothesis: random 2-3 term arithmetic from the same")
    print("  constants {S,T,P,F,N,M3,P2,P3} matches observed masses equally well.")
    print()

    mc = texas_sharpshooter(n_trials=50_000)
    print(f"  Actual TECS-L hits (within 1%): {mc['actual_hits']}/{mc['n_predictions']}")
    print(f"  Random expression mean hits:    {mc['mc_mean']:.2f}/{mc['n_predictions']}")
    print(f"  P(random >= observed):          {mc['p_geq_observed']:.4f}")
    print()

    if mc['p_geq_observed'] < 0.05:
        print(f"  >>> SIGNIFICANT at p = {mc['p_geq_observed']:.4f}")
        print(f"      TECS-L expressions match QCD observables better than")
        print(f"      random arithmetic from the same constants.")
    else:
        print(f"  >>> Not significant (p = {mc['p_geq_observed']:.4f}).")
        print(f"      Consider tighter tolerance or more predictions.")

    # --- Summary ---
    print()
    print("-" * 90)
    print("HIGHLIGHTS")
    print("-" * 90)
    print()
    print("  Pion mass (139.75 MeV):  sigma^2 - tau - phi/(sigma-tau)")
    print("    = 144 - 4 - 1/4 = 139.75  vs  139.570 observed  (0.13%)")
    print()
    print("  Proton mass (938 MeV):   P3*phi - sigma*tau - P1")
    print("    = 992 - 48 - 6 = 938  vs  938.272 observed  (0.03%)")
    print()
    print("  Color factors C_F=4/3, C_A=3, T_F=1/2 are EXACT TECS-L ratios:")
    print("    tau/(sigma/tau), sigma/tau, phi/tau")
    print()
    print("=" * 90)


def run_analysis():
    """Entry point."""
    print_report()


if __name__ == '__main__':
    run_analysis()
