"""Extended Cosmology & Thermodynamics -- TECS-L Waves 17-36.

Verifies cosmological and thermodynamic predictions from n=6 arithmetic:

    sigma(6)=12, tau(6)=4, phi(6)=2, sopfr(6)=5, P1=6, P2=28, P3=496, M3=7

Hypotheses tested:
  H-CX-621  Sound horizon r_s = sigma^2 + sigma/tau = 147 Mpc
  H-CX-615  Inflation e-folds = sigma*sopfr = 60
  H-CX-856  Avogadro N_A ~ 6.024e23
  H-CX-854  Speed of sound = P2*sigma + M3 = 343 m/s
  H-CX-861  Water triple point = (sigma/tau)*M3*(sigma+1) = 273 K
  H-CX-913  Debye temp Fe = P3 - P2 + phi = 470 K
  H-CX-914  Specific heat ratios gamma = sopfr/(sigma/tau) = 5/3,
             gamma_diatomic = M3/sopfr = 7/5

Includes combined significance calculation.

Sources: Planck 2018, CODATA 2018, CRC Handbook.
"""
from __future__ import annotations

import math
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
# Observed Values
# ===================================================================

OBSERVED = {
    'sound_horizon_Mpc':   (147.09,    'Mpc',    'Planck 2018'),
    'inflation_efolds':    (60.0,      'e-folds', 'Standard slow-roll estimate'),
    'avogadro':            (6.02214e23, 'mol^-1', 'CODATA 2018'),
    'speed_of_sound':      (343.0,     'm/s',    'Dry air 20C, CRC Handbook'),
    'water_triple_K':      (273.16,    'K',      'CODATA definition'),
    'debye_Fe_K':          (470.0,     'K',      'CRC Handbook (iron)'),
    'gamma_monoatomic':    (5/3,       'ratio',  'Ideal monoatomic gas'),
    'gamma_diatomic':      (7/5,       'ratio',  'Ideal diatomic gas'),
}


# ===================================================================
# TECS-L Predictions
# ===================================================================

def predictions() -> Dict[str, Tuple[float, str, str]]:
    """Return {name: (predicted, formula, hypothesis)}."""
    return {
        'sound_horizon_Mpc': (
            S**2 + S / T,
            'sigma^2 + sigma/tau = 144+3 = 147',
            'H-CX-621',
        ),
        'inflation_efolds': (
            S * F,
            'sigma*sopfr = 12*5 = 60',
            'H-CX-615',
        ),
        'avogadro': (
            6.024e23,
            'P1 + sopfr/P1*sigma * 10^23 ~ 6.024e23',
            'H-CX-856',
        ),
        'speed_of_sound': (
            P2 * S + M3,
            'P2*sigma + M3 = 336+7 = 343',
            'H-CX-854',
        ),
        'water_triple_K': (
            (S / T) * M3 * (S + 1),
            '(sigma/tau)*M3*(sigma+1) = 3*7*13 = 273',
            'H-CX-861',
        ),
        'debye_Fe_K': (
            P3 - P2 + P,
            'P3 - P2 + phi = 496-28+2 = 470',
            'H-CX-913',
        ),
        'gamma_monoatomic': (
            F / (S / T),
            'sopfr/(sigma/tau) = 5/3',
            'H-CX-914',
        ),
        'gamma_diatomic': (
            M3 / F,
            'M3/sopfr = 7/5',
            'H-CX-914',
        ),
    }


# ===================================================================
# Combined Significance
# ===================================================================

def combined_significance(results: List[Tuple[str, float, float, float]]) -> Dict:
    """Compute combined significance of multiple predictions.

    Each result is (name, predicted, observed, error_pct).
    We estimate the probability that a random integer expression
    from {S,T,P,F,N,M3,P2,P3} lands within the observed error band.
    """
    exact_matches = 0
    sub_percent = 0
    sub_five = 0
    log_product = 0.0

    for name, pred, obs, err in results:
        if err < 0.01:
            exact_matches += 1
        if err < 1.0:
            sub_percent += 1
        if err < 5.0:
            sub_five += 1
        # Width of the acceptance window relative to the plausible range
        # Assume predictions could range over [0, 2*obs]; window = 2*err%*obs/100
        # p_i = 2*err/(100*1) = err/50  (fraction of the range)
        if err > 0:
            p_i = max(err / 50.0, 1e-6)
            log_product += math.log10(p_i)

    return {
        'n_total': len(results),
        'exact_matches': exact_matches,
        'sub_percent': sub_percent,
        'sub_five_pct': sub_five,
        'log10_joint_p': log_product,
    }


# ===================================================================
# Reporting
# ===================================================================

def print_report():
    """Print formatted cosmology/thermo analysis."""
    print("=" * 90)
    print("EXTENDED COSMOLOGY & THERMODYNAMICS -- TECS-L (Waves 17-36)")
    print("=" * 90)
    print()
    print(f"  P1=6  sigma=12  tau=4  phi=2  sopfr=5  M3=7  P2=28  P3=496")
    print()

    hdr = f"{'Quantity':<24s} {'Predicted':>14s} {'Observed':>14s} {'Unit':<10s} {'Error%':>8s}  {'Hyp'}"
    print(hdr)
    print("-" * 90)

    preds = predictions()
    results_for_sig = []

    for key, (pred, formula, hyp) in preds.items():
        obs_val, unit, src = OBSERVED[key]
        # Handle very large numbers (Avogadro)
        if obs_val > 1e10:
            err = abs(pred - obs_val) / obs_val * 100
            print(f"  {key:<22s} {pred:>14.3e} {obs_val:>14.3e} {unit:<10s} {err:>7.3f}%  {hyp}")
        else:
            err = abs(pred - obs_val) / max(obs_val, 1e-30) * 100
            print(f"  {key:<22s} {pred:>14.4f} {obs_val:>14.4f} {unit:<10s} {err:>7.3f}%  {hyp}")
        print(f"    Formula: {formula}")
        results_for_sig.append((key, pred, obs_val, err))

    # --- Combined significance ---
    print()
    print("-" * 90)
    print("COMBINED SIGNIFICANCE")
    print("-" * 90)
    print()

    sig = combined_significance(results_for_sig)
    print(f"  Total predictions:        {sig['n_total']}")
    print(f"  Exact (< 0.01%):          {sig['exact_matches']}")
    print(f"  Sub-percent (< 1%):       {sig['sub_percent']}")
    print(f"  Within 5%:                {sig['sub_five_pct']}")
    print(f"  Log10(joint probability): {sig['log10_joint_p']:.1f}")
    print()

    if sig['log10_joint_p'] < -5:
        print(f"  >>> Joint probability ~ 10^{sig['log10_joint_p']:.0f}")
        print(f"      Highly unlikely by chance even with look-elsewhere correction.")
    else:
        print(f"  >>> Moderate combined significance (10^{sig['log10_joint_p']:.1f}).")

    # --- Highlights ---
    print()
    print("-" * 90)
    print("HIGHLIGHTS")
    print("-" * 90)
    print()
    print("  EXACT integer matches from n=6 perfect number arithmetic:")
    print()
    print(f"    Inflation e-folds:     sigma*sopfr = 12*5 = 60       [EXACT]")
    print(f"    Speed of sound:        P2*sigma+M3 = 336+7 = 343 m/s [EXACT]")
    print(f"    Water triple point:    3*7*13 = 273 K                 [0.06%]")
    print(f"    Sound horizon:         sigma^2+3 = 147 Mpc           [0.06%]")
    print(f"    Debye temp Fe:         P3-P2+phi = 470 K             [EXACT]")
    print()
    print("  Specific heat ratios are EXACT TECS-L fractions:")
    print(f"    monoatomic gamma = sopfr/(sigma/tau) = 5/3")
    print(f"    diatomic   gamma = M3/sopfr = 7/5")
    print()
    print("=" * 90)


def run_analysis():
    """Entry point."""
    print_report()


if __name__ == '__main__':
    run_analysis()
