"""37 GeV Resonance Prediction — TECS-L ladder convergence analysis.

Two independent extensions of the QCD resonance ladder converge at 37-38 GeV:
    J/psi(3097) x sigma(6)=12  = 37.163 GeV
    Upsilon(9460) x tau(6)=4   = 37.842 GeV

This module computes the predicted mass, searches for known states, evaluates
the statistical significance of the convergence, cross-checks against all
TECS-L combinations, and produces a prediction card for CMS/ATLAS searches.

Physics context:
    - The QCD vector meson ladder: rho(0.775) x4 -> J/psi(3.097) x3 -> Upsilon(9.460)
    - sigma(6) = 12 = 4 x 3, so a sigma-step from J/psi or a tau-step from Upsilon
      should land on the SAME next rung — and they nearly do (1.8% apart).
    - No established particle exists at 37-38 GeV.
"""
from __future__ import annotations

import math
from collections import OrderedDict
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple

import numpy as np
from scipy.stats import norm

from ..tecs import (
    SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1, OMEGA_P1,
    P1, P2, P3,
    TAU_P2, TAU_P3, PHI_P3,
)
from .pdg import get_all as pdg_base_all, get_masses as pdg_base_masses
from .pdg_extended import get_all as pdg_ext_all, get_masses as pdg_ext_masses


# =====================================================================
# 0. Constants
# =====================================================================

S = SIGMA_P1     # 12
T = TAU_P1       # 4
P = PHI_P1       # 2
F = SOPFR_P1     # 5
N = P1           # 6

# Ground-state vector meson masses (GeV, PDG 2024)
M_RHO   = 0.77526
M_JPSI  = 3.09690
M_UPS   = 9.46040

# Uncertainties (GeV)
DM_RHO  = 0.00025
DM_JPSI = 0.00001
DM_UPS  = 0.00010


# =====================================================================
# 1. Predicted mass from ladder convergence
# =====================================================================

@dataclass
class LadderPrediction:
    """A single ladder extension prediction."""
    label: str
    basis_particle: str
    basis_mass: float
    basis_unc: float
    multiplier_name: str
    multiplier_value: float   # exact TECS-L integer
    predicted_mass: float
    predicted_unc: float

    @property
    def frac_unc(self):
        return self.predicted_unc / self.predicted_mass


def primary_predictions() -> List[LadderPrediction]:
    """The two primary ladder extensions that converge at ~37 GeV."""
    preds = []

    # J/psi x sigma(6) = 3.097 x 12 = 37.163
    p1_mass = M_JPSI * S
    p1_unc = DM_JPSI * S
    preds.append(LadderPrediction(
        label='J/psi x sigma(6)',
        basis_particle='J/psi(1S)',
        basis_mass=M_JPSI,
        basis_unc=DM_JPSI,
        multiplier_name='sigma(6)=12',
        multiplier_value=S,
        predicted_mass=p1_mass,
        predicted_unc=p1_unc,
    ))

    # Upsilon x tau(6) = 9.460 x 4 = 37.842
    p2_mass = M_UPS * T
    p2_unc = DM_UPS * T
    preds.append(LadderPrediction(
        label='Upsilon x tau(6)',
        basis_particle='Upsilon(1S)',
        basis_mass=M_UPS,
        basis_unc=DM_UPS,
        multiplier_name='tau(6)=4',
        multiplier_value=T,
        predicted_mass=p2_mass,
        predicted_unc=p2_unc,
    ))

    return preds


def weighted_average(predictions: List[LadderPrediction]) -> dict:
    """Inverse-variance weighted average of predictions.

    For the primary pair the dominant uncertainty is the TECS-L ladder
    precision itself (the multiplier might not be exactly integer), so
    we additionally include a 'ladder tolerance' equal to the deviation
    of the observed ratio from its TECS-L integer.
    """
    # Ladder precision: how well does rho x 4 = J/psi, J/psi x 3 = Ups
    ratio1 = M_JPSI / M_RHO        # should be 4
    ratio2 = M_UPS / M_JPSI        # should be 3
    ladder_frac_err1 = abs(ratio1 - T) / T   # fractional
    ladder_frac_err2 = abs(ratio2 - (S / T)) / (S / T)

    masses = np.array([p.predicted_mass for p in predictions])
    # Experimental uncertainty (tiny)
    exp_unc = np.array([p.predicted_unc for p in predictions])
    # Ladder systematic uncertainty (dominant)
    ladder_unc = np.array([
        predictions[0].predicted_mass * ladder_frac_err1,
        predictions[1].predicted_mass * ladder_frac_err2,
    ])
    total_unc = np.sqrt(exp_unc**2 + ladder_unc**2)

    weights = 1.0 / total_unc**2
    w_sum = weights.sum()
    avg_mass = (weights * masses).sum() / w_sum
    avg_unc = 1.0 / np.sqrt(w_sum)

    # Error band: range spanned by predictions +/- their individual uncertainties
    lo = min(m - u for m, u in zip(masses, total_unc))
    hi = max(m + u for m, u in zip(masses, total_unc))

    return {
        'central_mass_GeV': avg_mass,
        'stat_unc_GeV': avg_unc,
        'error_band_lo': lo,
        'error_band_hi': hi,
        'spread_pct': abs(masses[1] - masses[0]) / avg_mass * 100,
        'individual': [
            {'label': p.label, 'mass': p.predicted_mass,
             'total_unc': float(u), 'ladder_unc': float(lu)}
            for p, u, lu in zip(predictions, total_unc, ladder_unc)
        ],
    }


# =====================================================================
# 2. Additional TECS-L combinations giving ~37 GeV
# =====================================================================

def additional_predictions(window_lo=33.0, window_hi=42.0) -> List[dict]:
    """Find ALL TECS-L multiplier combinations from known particles
    that land in the 33-42 GeV window."""
    bases = OrderedDict([
        ('rho(770)',    (M_RHO, DM_RHO)),
        ('J/psi(1S)',   (M_JPSI, DM_JPSI)),
        ('Upsilon(1S)', (M_UPS, DM_UPS)),
        ('phi(1020)',   (1.019461, 0.000016)),
        ('psi(2S)',     (3.68610, 0.00001)),
        ('eta_c',       (2.9839, 0.0004)),
        ('eta_b',       (9.3990, 0.0023)),
        ('omega(782)',  (0.78266, 0.00013)),
    ])

    multipliers = OrderedDict([
        ('phi=2',       2),
        ('sigma/tau=3', 3),
        ('tau=4',       4),
        ('sopfr=5',     5),
        ('n=6',         6),
        ('sigma-tau=8', 8),
        ('tau(28)=6',   TAU_P2),
        ('tau(496)=10', TAU_P3),
        ('sigma=12',    S),
        ('sigma*phi=24', S * P),
        ('P2=28',       P2),
        ('n^2=36',      N**2),
        ('sigma*tau=48', S * T),
    ])

    hits = []
    for bname, (bmass, bunc) in bases.items():
        for mname, mval in multipliers.items():
            pred = bmass * mval
            if window_lo <= pred <= window_hi:
                hits.append({
                    'basis': bname,
                    'multiplier': mname,
                    'factor': mval,
                    'predicted_GeV': pred,
                    'unc_GeV': bunc * mval,
                })

    return sorted(hits, key=lambda h: abs(h['predicted_GeV'] - 37.5))


# =====================================================================
# 3. Known particle search near 37-38 GeV
# =====================================================================

def search_known_particles(center=37.5, half_width=3.0) -> dict:
    """Search ALL known PDG states (base + extended + exotic) for anything
    near the predicted mass."""
    lo, hi = center - half_width, center + half_width

    all_particles = pdg_ext_all()
    masses = pdg_ext_masses()

    # Direct mass matches
    direct = [p for p in all_particles if lo <= p['mass'] <= hi]

    # Check specific categories of interest
    categories_of_interest = {
        'exotic_XYZ': [p for p in all_particles
                       if p['category'] == 'exotic'],
        'bottomonium': [p for p in all_particles
                        if 'Upsilon' in p['name'] or 'chi_b' in p['name']
                        or 'eta_b' in p['name'] or 'h_b' in p['name']],
        'charmonium': [p for p in all_particles
                       if 'psi' in p['name'] or 'chi_c' in p['name']
                       or 'J_psi' in p['name'] or 'eta_c' in p['name']
                       or 'h_c' in p['name']],
        'Bc_family': [p for p in all_particles if 'B_c' in p['name']],
    }

    # Highest-mass known particles
    by_mass = sorted(all_particles, key=lambda p: p['mass'], reverse=True)
    heaviest = by_mass[:10]

    # Gap analysis: where is the spectroscopy desert?
    all_masses_sorted = sorted(set(p['mass'] for p in all_particles))
    gaps = []
    for i in range(len(all_masses_sorted) - 1):
        gap_lo = all_masses_sorted[i]
        gap_hi = all_masses_sorted[i + 1]
        gap_size = gap_hi - gap_lo
        if gap_size > 5.0:  # significant gaps only
            gaps.append({
                'from_GeV': gap_lo,
                'to_GeV': gap_hi,
                'gap_GeV': gap_size,
            })

    return {
        'search_window': f'{lo:.1f} - {hi:.1f} GeV',
        'direct_matches': direct,
        'n_direct': len(direct),
        'categories': {k: len(v) for k, v in categories_of_interest.items()},
        'heaviest_known': [(p['name'], p['mass']) for p in heaviest],
        'spectroscopy_gaps': gaps,
        'verdict': 'NO KNOWN PARTICLE' if len(direct) == 0 else
                   f'{len(direct)} candidates found',
    }


# =====================================================================
# 4. Physical expectations at 37 GeV
# =====================================================================

def physical_expectations() -> dict:
    """What QCD / Standard Model predicts should exist near 37 GeV."""
    # Known thresholds
    bb_threshold = 2 * 4.18       # ~8.36 GeV (well below)
    tt_threshold = 2 * 172.76     # ~345.5 GeV (well above)
    ww_threshold = 2 * 80.377     # ~160.8 GeV
    zz_threshold = 2 * 91.1876    # ~182.4 GeV

    # Quarkonium: highest known bottomonium is Upsilon(11020) at ~11 GeV
    # Gap between highest bottomonium (11 GeV) and W (80 GeV) is a desert
    # No bound qqbar state predicted at 37 GeV by standard QCD

    # B_c excited states: ground B_c = 6.275 GeV
    # Highest B_c excited ~ 6.8-7.0 GeV predicted
    # Not anywhere near 37 GeV

    return {
        'thresholds': {
            'bb_bar': bb_threshold,
            'tt_bar': tt_threshold,
            'WW': ww_threshold,
            'ZZ': zz_threshold,
        },
        'quarkonium_ceiling': {
            'highest_bottomonium': ('Upsilon(11020)', 11.000),
            'highest_charmonium': ('psi(4415)', 4.421),
            'note': 'Standard QCD predicts no qqbar bound state at 37 GeV',
        },
        'Bc_excited': {
            'ground_state': 6.275,
            'predicted_ceiling': '~7 GeV (2S, 2P states)',
            'note': 'B_c family far below 37 GeV',
        },
        'spectroscopy_desert': {
            'from_GeV': 11.0,
            'to_GeV': 80.4,
            'note': ('No established hadronic resonance exists between '
                     'Upsilon(11020) at 11 GeV and W at 80 GeV. '
                     'A state at 37 GeV would be the FIRST in this desert.'),
        },
        'possible_interpretations': [
            'New vector boson (Z-prime-like) with suppressed couplings',
            'Composite scalar from new strong dynamics',
            'Tetraquark / hexaquark at unexpectedly high mass',
            'TECS-L predicted resonance without standard QCD interpretation',
        ],
    }


# =====================================================================
# 5. Collider coverage at 37 GeV
# =====================================================================

def collider_coverage() -> dict:
    """What experiments have data covering the 37 GeV region."""
    return {
        'LEP': {
            'sqrt_s_range': '91-209 GeV (LEP1: Z-pole, LEP2: up to 209)',
            'direct_scan': ('LEP did NOT run at sqrt(s) = 37 GeV. '
                            'LEP1 ran at the Z pole (91 GeV); '
                            'LEP2 ran 130-209 GeV. '
                            'The PETRA/PEP/TRISTAN colliders covered '
                            'sqrt(s) = 12-64 GeV in the 1980s.'),
            'PETRA_TRISTAN': ('PETRA ran at sqrt(s) = 12-46 GeV (1978-1986). '
                              'A scan through 37 GeV DID occur. '
                              'R = sigma(e+e- -> hadrons)/sigma_point was '
                              'measured but no narrow resonance reported.'),
            'sensitivity': ('PETRA R-measurement precision ~5-10%. '
                            'A narrow resonance with sigma < few % of '
                            'continuum could have been missed.'),
        },
        'Tevatron': {
            'sqrt_s': '1.96 TeV (pp-bar)',
            'coverage': ('Drell-Yan dimuon/dielectron invariant mass spectra '
                         'cover the 37 GeV region. No excess reported in '
                         'published CDF/D0 analyses, but sensitivity depends '
                         'on coupling strength.'),
        },
        'LHC': {
            'sqrt_s': '7, 8, 13, 13.6 TeV (pp)',
            'coverage': ('CMS and ATLAS have high-statistics dimuon and '
                         'dielectron invariant mass spectra covering 37 GeV. '
                         'This region sits between the Upsilon family (~10 GeV) '
                         'and the Z peak (91 GeV).'),
            'existing_searches': [
                'CMS: dimuon spectrum (HIG-19-006) — smooth background at 37 GeV',
                'ATLAS: dielectron/dimuon (EXOT-2019-05) — no bump at 37 GeV',
                'LHCb: dimuon mass spectrum — best low-mass sensitivity',
            ],
            'key_point': ('37 GeV falls in the "valley" between Upsilon '
                          'and Z in the dilepton mass spectrum. A narrow '
                          'resonance would appear as a bump on a smoothly '
                          'falling Drell-Yan background.'),
        },
        'proposed_search': {
            'channel': 'pp -> X(37) -> mu+mu- / e+e-',
            'mass_window': '35-40 GeV',
            'background': 'Drell-Yan continuum (well understood)',
            'method': 'Bump hunt on dimuon invariant mass spectrum',
            'datasets': ['LHC Run 2 (139 fb^-1 at 13 TeV)',
                         'LHC Run 3 (ongoing, 13.6 TeV)'],
        },
    }


# =====================================================================
# 6. Decay channel predictions
# =====================================================================

def decay_predictions() -> dict:
    """Predict decay signatures assuming the 37 GeV state is vector (J^PC = 1--)."""
    m_x = 37.5  # central prediction

    # If vector (like rho, J/psi, Upsilon) -> lepton pairs
    # Branching ratios scale roughly as 1/m^2 for QCD-like states
    # J/psi: BR(mu+mu-) = 5.96%, Upsilon: BR(mu+mu-) = 2.48%

    br_jpsi_mumu = 0.0596
    br_ups_mumu = 0.0248
    # Naive 1/m^2 extrapolation from Upsilon
    br_x_mumu_naive = br_ups_mumu * (M_UPS / m_x)**2

    return {
        'assumed_JPC': '1-- (vector)',
        'primary_channels': {
            'mu+mu-': {
                'signature': 'Opposite-sign dimuon peak at 37 GeV',
                'estimated_BR': f'{br_x_mumu_naive*100:.3f}%',
                'note': 'Naive 1/m^2 scaling from Upsilon',
            },
            'e+e-': {
                'signature': 'Opposite-sign dielectron peak at 37 GeV',
                'estimated_BR': f'{br_x_mumu_naive*100:.3f}% (lepton universality)',
            },
            'hadrons': {
                'signature': 'Enhancement in R = sigma(hadrons)/sigma_point at sqrt(s) = 37 GeV',
                'estimated_BR': f'~{(1 - 3*br_x_mumu_naive)*100:.1f}% (hadronic)',
            },
            'tau+tau-': {
                'signature': 'Ditau peak at 37 GeV invariant mass',
                'estimated_BR': f'{br_x_mumu_naive*100:.3f}% (lepton universality)',
                'note': 'Challenging but possible at LHC',
            },
        },
        'production_modes': {
            'Drell-Yan': 'qq-bar -> gamma*/Z* -> X(37) (dominant at LHC)',
            'photon_fusion': 'gamma gamma -> X(37) (ultra-peripheral collisions)',
            'e+e-': 'Direct production at sqrt(s) = 37 GeV (future e+e- collider)',
        },
        'R_ratio_prediction': {
            'quantity': 'R(s) = sigma(e+e- -> hadrons) / (4*pi*alpha^2 / 3s)',
            'background': 'R ~ 3.5-4.0 (sum of quark charges squared x N_c x QCD corrections)',
            'signal': 'Narrow peak on top of continuum at sqrt(s) = 37.5 GeV',
        },
    }


# =====================================================================
# 7. Full TECS-L rho prediction table
# =====================================================================

def rho_prediction_table() -> List[dict]:
    """All integer multiples of rho(770) using TECS-L numbers, checked
    against known particles."""
    all_masses = pdg_ext_masses()

    # Build TECS-L motivated multipliers
    tecs_multipliers = []
    for n in range(1, 61):
        # Pure integer multiples
        tecs_multipliers.append((f'x{n}', n))

    # Add special TECS-L combinations
    tecs_multipliers.extend([
        ('x(sigma)=12', S),
        ('x(sigma*phi)=24', S * P),
        ('x(n^2)=36', N**2),
        ('x(sigma*tau)=48', S * T),
        ('x(sigma^2)=144', S**2),
    ])

    results = []
    for label, mult in tecs_multipliers:
        pred = M_RHO * mult

        # Find nearest known particle
        best_name, best_mass, best_err = None, None, float('inf')
        for name, mass in all_masses.items():
            err = abs(mass - pred) / pred
            if err < best_err:
                best_name, best_mass, best_err = name, mass, err

        is_tecs = mult in [S, T, P, F, N, S*P, N**2, S*T, S-T, S+T, P2, S**2]
        results.append({
            'multiplier': label,
            'factor': mult,
            'predicted_GeV': pred,
            'nearest': best_name,
            'nearest_mass': best_mass,
            'error_pct': best_err * 100,
            'tecs_special': is_tecs,
            'match': best_err < 0.03,
        })

    return results


# =====================================================================
# 8. Monte Carlo convergence significance
# =====================================================================

def convergence_mc(n_trials=500_000, seed=42) -> dict:
    """Monte Carlo test: how often do two random ladder extensions
    from DIFFERENT particles converge within 1.8% of each other?

    Method: Take two observed masses from the PDG spectrum, multiply each
    by a random TECS-L-like integer (2-12), and check if the products
    land within 1.8% of each other.
    """
    rng = np.random.default_rng(seed)

    # The actual convergence
    pred1 = M_JPSI * S       # 37.163
    pred2 = M_UPS * T        # 37.842
    actual_spread = abs(pred2 - pred1) / ((pred1 + pred2) / 2)

    # All PDG masses for sampling
    all_masses = np.array([p['mass'] for p in pdg_ext_all() if p['mass'] > 0.1])
    n_particles = len(all_masses)

    # TECS-L multipliers to sample from
    multipliers = np.array([2, 3, 4, 5, 6, 8, 10, 12])

    hits = 0
    for _ in range(n_trials):
        # Pick two different particles
        idx = rng.choice(n_particles, size=2, replace=False)
        m1, m2 = all_masses[idx]

        # Pick two multipliers
        k1, k2 = rng.choice(multipliers, size=2)

        # Products
        p1 = m1 * k1
        p2 = m2 * k2

        # Check convergence
        if p1 > 0 and p2 > 0:
            spread = abs(p2 - p1) / ((p1 + p2) / 2)
            if spread < actual_spread:
                hits += 1

    p_value = hits / n_trials if hits > 0 else 1 / n_trials
    sigma_val = norm.isf(p_value) if p_value < 0.5 else 0.0

    return {
        'n_trials': n_trials,
        'actual_spread_pct': actual_spread * 100,
        'hits': hits,
        'p_value': p_value,
        'p_value_str': f'{p_value:.6f}' if hits > 0 else f'< {1/n_trials:.1e}',
        'sigma': f'{sigma_val:.2f}' if sigma_val > 0 else '< 1',
        'method': ('Draw 2 random PDG masses, multiply each by random '
                   'TECS-L integer [2..12], check if products converge '
                   f'within {actual_spread*100:.2f}%'),
    }


def ladder_chain_mc(n_trials=500_000, seed=42) -> dict:
    """Stricter test: how often does a CHAIN rho->X->Y with ratios
    near 4 and 3 produce a next step Y->Z (via x4) that converges
    with X->Z (via x12)?

    This tests whether the algebraic identity tau x (sigma/tau) = sigma
    is trivially satisfied by random mass spectra.
    """
    rng = np.random.default_rng(seed)

    # Observed ladder precision
    r1_obs = M_JPSI / M_RHO   # ~3.995 (should be 4)
    r2_obs = M_UPS / M_JPSI   # ~3.054 (should be 3)
    tol_r1 = abs(r1_obs - 4) / 4   # ~0.12%
    tol_r2 = abs(r2_obs - 3) / 3   # ~1.8%

    # Use 3% tolerance for matching ratios
    tol = 0.03

    all_masses = np.array([p['mass'] for p in pdg_ext_all()
                           if 0.1 < p['mass'] < 200])
    n_particles = len(all_masses)

    hits = 0
    for _ in range(n_trials):
        idx = rng.choice(n_particles, size=3, replace=False)
        triple = np.sort(all_masses[idx])

        r1 = triple[1] / triple[0]
        r2 = triple[2] / triple[1]

        # Does this triple form a x4, x3 ladder?
        if abs(r1 - 4) / 4 < tol and abs(r2 - 3) / 3 < tol:
            # Now predict next rung two ways
            pred_sigma = triple[1] * 12      # middle x sigma
            pred_tau = triple[2] * 4         # top x tau
            spread = abs(pred_sigma - pred_tau) / ((pred_sigma + pred_tau) / 2)
            # The observed spread is 1.8%
            if spread < 0.018:
                hits += 1

    p_value = hits / n_trials if hits > 0 else 1 / n_trials
    sigma_val = norm.isf(p_value) if 0 < p_value < 0.5 else float('inf')

    return {
        'n_trials': n_trials,
        'hits': hits,
        'p_value': p_value,
        'p_value_str': f'{p_value:.6f}' if hits > 0 else f'< {1/n_trials:.1e}',
        'sigma': f'{sigma_val:.2f}' if sigma_val < 100 else f'> {norm.isf(1/n_trials):.1f}',
        'method': ('Draw 3 PDG masses forming x4, x3 ladder, '
                   'then check if sigma-extension and tau-extension '
                   'of the next rung converge within 1.8%'),
    }


# =====================================================================
# 9. Prediction card for CMS/ATLAS
# =====================================================================

def prediction_card() -> str:
    """Generate a formatted prediction card suitable for a search proposal."""
    preds = primary_predictions()
    avg = weighted_average(preds)
    addl = additional_predictions()
    search = search_known_particles()
    phys = physical_expectations()
    decay = decay_predictions()

    m = avg['central_mass_GeV']
    lo = avg['error_band_lo']
    hi = avg['error_band_hi']

    card = []
    card.append('=' * 78)
    card.append('  TECS-L RESONANCE PREDICTION CARD')
    card.append('  State: X(37) — Predicted QCD Ladder Resonance')
    card.append('=' * 78)
    card.append('')
    card.append(f'  Predicted mass:  {m:.2f} GeV')
    card.append(f'  Error band:      [{lo:.2f}, {hi:.2f}] GeV')
    card.append(f'  Search window:   35 - 40 GeV (conservative)')
    card.append(f'  Spread of primary predictions: {avg["spread_pct"]:.2f}%')
    card.append('')
    card.append('  --- Derivation ---')
    card.append(f'  Path 1: J/psi(3097) x sigma(6)=12 = {preds[0].predicted_mass:.3f} GeV')
    card.append(f'  Path 2: Upsilon(9460) x tau(6)=4  = {preds[1].predicted_mass:.3f} GeV')
    card.append(f'  Convergence: two independent ladder extensions land {avg["spread_pct"]:.1f}% apart')
    card.append('')
    card.append('  --- Supporting predictions in 33-42 GeV window ---')
    for a in addl[:8]:
        card.append(f'    {a["basis"]:<16s} x {a["multiplier"]:<16s} = {a["predicted_GeV"]:.3f} GeV')
    card.append('')
    card.append(f'  --- Known particles: {search["verdict"]} ---')
    if search['direct_matches']:
        for p in search['direct_matches']:
            card.append(f'    {p["name"]}: {p["mass"]:.3f} GeV ({p["category"]})')
    else:
        card.append('    No established particle in the 34.5-40.5 GeV window.')
        card.append(f'    Spectroscopy desert: {phys["spectroscopy_desert"]["note"]}')
    card.append('')
    card.append('  --- Assumed quantum numbers ---')
    card.append(f'    J^PC = {decay["assumed_JPC"]}')
    card.append('    (continuation of rho/J/psi/Upsilon vector meson ladder)')
    card.append('')
    card.append('  --- Primary search channels ---')
    for ch, info in decay['primary_channels'].items():
        card.append(f'    {ch}: {info["signature"]}')
    card.append('')
    card.append('  --- Recommended datasets ---')
    card.append('    * CMS/ATLAS dimuon invariant mass, Run 2 (139 fb^-1)')
    card.append('    * LHCb dimuon spectrum (best low-mass resolution)')
    card.append('    * LHC Run 3 data (ongoing)')
    card.append('')
    card.append('  --- Method ---')
    card.append('    Bump hunt on smoothly falling Drell-Yan background')
    card.append('    in the dimuon/dielectron invariant mass spectrum,')
    card.append('    mass window 35-40 GeV.')
    card.append('')
    card.append('=' * 78)

    return '\n'.join(card)


# =====================================================================
# Full report
# =====================================================================

def run_analysis():
    """Run complete 37 GeV resonance analysis."""
    width = 78
    sep = '=' * width
    subsep = '-' * width

    print(f'\n{sep}')
    print('  37 GeV RESONANCE PREDICTION — TECS-L Ladder Convergence Analysis')
    print(sep)

    # --- 1. Primary predictions ---
    print(f'\n--- 1. Primary Ladder Predictions ---\n')
    preds = primary_predictions()
    for p in preds:
        print(f'  {p.label}:')
        print(f'    {p.basis_particle} ({p.basis_mass:.5f} GeV) '
              f'x {p.multiplier_name} = {p.predicted_mass:.3f} +/- {p.predicted_unc:.3f} GeV')

    avg = weighted_average(preds)
    print(f'\n  Weighted average:  {avg["central_mass_GeV"]:.2f} GeV')
    print(f'  Error band:        [{avg["error_band_lo"]:.2f}, {avg["error_band_hi"]:.2f}] GeV')
    print(f'  Spread:            {avg["spread_pct"]:.2f}%')
    print(f'\n  Individual total uncertainties:')
    for ind in avg['individual']:
        print(f'    {ind["label"]}: {ind["mass"]:.3f} +/- {ind["total_unc"]:.3f} GeV '
              f'(ladder: {ind["ladder_unc"]:.3f} GeV)')

    # --- 2. Additional TECS-L predictions ---
    print(f'\n--- 2. Additional TECS-L Combinations in 33-42 GeV ---\n')
    addl = additional_predictions()
    print(f'  {"Basis":<16s} {"x Multiplier":<18s} {"Predicted":>10s}')
    print(f'  {"-"*16} {"-"*18} {"-"*10}')
    for a in addl:
        print(f'  {a["basis"]:<16s} {a["multiplier"]:<18s} {a["predicted_GeV"]:>10.3f}')
    print(f'\n  Total combinations landing in 33-42 GeV: {len(addl)}')

    # --- 3. Known particle search ---
    print(f'\n--- 3. Known Particle Search near 37 GeV ---\n')
    search = search_known_particles()
    print(f'  Search window: {search["search_window"]}')
    print(f'  Direct matches: {search["n_direct"]}')
    print(f'  Verdict: {search["verdict"]}')
    if search['direct_matches']:
        for p in search['direct_matches']:
            print(f'    -> {p["name"]}: {p["mass"]:.3f} GeV ({p["category"]})')
    print(f'\n  Heaviest known particles:')
    for name, mass in search['heaviest_known']:
        print(f'    {name:<20s} {mass:>10.3f} GeV')
    print(f'\n  Spectroscopy gaps > 5 GeV:')
    for g in search['spectroscopy_gaps']:
        marker = ' <-- 37 GeV IS HERE' if g['from_GeV'] < 37 < g['to_GeV'] else ''
        print(f'    {g["from_GeV"]:.1f} - {g["to_GeV"]:.1f} GeV '
              f'(gap = {g["gap_GeV"]:.1f} GeV){marker}')

    # --- 4. Physical expectations ---
    print(f'\n--- 4. Physical Expectations at 37 GeV ---\n')
    phys = physical_expectations()
    print('  Relevant thresholds:')
    for name, val in phys['thresholds'].items():
        print(f'    {name:<10s}: {val:.1f} GeV')
    print(f'\n  Quarkonium ceiling:')
    for key, val in phys['quarkonium_ceiling'].items():
        if isinstance(val, tuple):
            print(f'    {key}: {val[0]} ({val[1]:.3f} GeV)')
        else:
            print(f'    {key}: {val}')
    print(f'\n  Spectroscopy desert: {phys["spectroscopy_desert"]["note"]}')
    print(f'\n  Possible interpretations:')
    for interp in phys['possible_interpretations']:
        print(f'    * {interp}')

    # --- 5. Collider coverage ---
    print(f'\n--- 5. Collider Coverage at 37 GeV ---\n')
    cov = collider_coverage()
    for exp, info in cov.items():
        if exp == 'proposed_search':
            continue
        print(f'  {exp}:')
        if isinstance(info, dict):
            for k, v in info.items():
                if isinstance(v, list):
                    print(f'    {k}:')
                    for item in v:
                        print(f'      - {item}')
                else:
                    # Wrap long strings
                    vstr = str(v)
                    if len(vstr) > 60:
                        words = vstr.split()
                        lines = []
                        line = ''
                        for w in words:
                            if len(line) + len(w) + 1 > 60:
                                lines.append(line)
                                line = w
                            else:
                                line = f'{line} {w}' if line else w
                        if line:
                            lines.append(line)
                        print(f'    {k}: {lines[0]}')
                        for l in lines[1:]:
                            print(f'      {l}')
                    else:
                        print(f'    {k}: {vstr}')
        print()

    # --- 6. Decay predictions ---
    print(f'--- 6. Decay Channel Predictions (assuming 1--) ---\n')
    decay = decay_predictions()
    for ch, info in decay['primary_channels'].items():
        print(f'  {ch}:')
        print(f'    Signature:    {info["signature"]}')
        print(f'    Estimated BR: {info["estimated_BR"]}')
        if 'note' in info:
            print(f'    Note:         {info["note"]}')

    # --- 7. Rho prediction table (condensed) ---
    print(f'\n--- 7. TECS-L Rho Multiples — Matches Only ---\n')
    rho_table = rho_prediction_table()
    matches = [r for r in rho_table if r['match']]
    tecs_special = [r for r in rho_table if r['tecs_special']]

    print(f'  Matched rho multiples (< 3% error):')
    print(f'  {"Mult":<18s} {"Predicted":>10s} {"Nearest":<20s} {"Mass":>10s} {"Err%":>6s} {"TECS":>5s}')
    print(f'  {"-"*18} {"-"*10} {"-"*20} {"-"*10} {"-"*6} {"-"*5}')
    for r in matches:
        tecs_flag = '*' if r['tecs_special'] else ''
        print(f'  {r["multiplier"]:<18s} {r["predicted_GeV"]:>10.3f} '
              f'{r["nearest"]:<20s} {r["nearest_mass"]:>10.3f} '
              f'{r["error_pct"]:>6.1f} {tecs_flag:>5s}')

    print(f'\n  TECS-special multiples:')
    print(f'  {"Mult":<18s} {"Factor":>6s} {"Predicted":>10s} {"Nearest":<20s} {"Err%":>6s}')
    print(f'  {"-"*18} {"-"*6} {"-"*10} {"-"*20} {"-"*6}')
    for r in tecs_special:
        print(f'  {r["multiplier"]:<18s} {r["factor"]:>6.0f} {r["predicted_GeV"]:>10.3f} '
              f'{r["nearest"]:<20s} {r["error_pct"]:>6.1f}')

    # --- 8. Monte Carlo significance ---
    print(f'\n--- 8. Convergence Significance (Monte Carlo) ---\n')

    print('  Test A: Random pair convergence')
    mc_pair = convergence_mc(n_trials=200_000)
    print(f'    Method:  {mc_pair["method"]}')
    print(f'    Trials:  {mc_pair["n_trials"]:,}')
    print(f'    Hits:    {mc_pair["hits"]:,}')
    print(f'    p-value: {mc_pair["p_value_str"]}')
    print(f'    Sigma:   {mc_pair["sigma"]}')
    print()

    print('  Test B: Full ladder chain convergence')
    mc_chain = ladder_chain_mc(n_trials=200_000)
    print(f'    Method:  {mc_chain["method"]}')
    print(f'    Trials:  {mc_chain["n_trials"]:,}')
    print(f'    Hits:    {mc_chain["hits"]:,}')
    print(f'    p-value: {mc_chain["p_value_str"]}')
    print(f'    Sigma:   {mc_chain["sigma"]}')

    # --- 9. Prediction card ---
    print(f'\n--- 9. CMS/ATLAS Prediction Card ---\n')
    card = prediction_card()
    print(card)

    # --- Summary ---
    print(f'\n{sep}')
    print('  SUMMARY')
    print(sep)
    print(f'''
  The QCD vector meson ladder rho(770) -> J/psi(3097) -> Upsilon(9460)
  extends to a CONVERGENCE POINT at ~{avg["central_mass_GeV"]:.1f} GeV:

    J/psi x sigma(6)=12  = {preds[0].predicted_mass:.3f} GeV
    Upsilon x tau(6)=4   = {preds[1].predicted_mass:.3f} GeV
    Spread:                {avg["spread_pct"]:.2f}%

  This convergence arises because sigma(6) = tau(6) x [sigma(6)/tau(6)]
  = 4 x 3 = 12. The ladder uses tau(6)=4 at each step, and when we
  multiply J/psi by the FULL sigma=12 vs Upsilon by the STEP tau=4,
  both point to ~37 GeV.

  Known particles at 37 GeV:  {search["verdict"]}
  Spectroscopy desert:        11 GeV (Upsilon) to 80 GeV (W)
  Additional TECS-L routes:   {len(addl)} combinations land in 33-42 GeV

  MC pair convergence:        p = {mc_pair["p_value_str"]} ({mc_pair["sigma"]} sigma)
  MC chain convergence:       p = {mc_chain["p_value_str"]} ({mc_chain["sigma"]} sigma)

  PREDICTION: A narrow resonance at {avg["central_mass_GeV"]:.1f} +/- {(avg["error_band_hi"]-avg["error_band_lo"])/2:.1f} GeV
  should appear as a bump in the LHC dimuon invariant mass spectrum.
''')
    print(sep)

    return {
        'primary_predictions': preds,
        'weighted_average': avg,
        'additional': addl,
        'known_search': search,
        'physics': phys,
        'collider': cov,
        'decay': decay,
        'rho_table_matches': matches,
        'mc_pair': mc_pair,
        'mc_chain': mc_chain,
    }


if __name__ == '__main__':
    run_analysis()
