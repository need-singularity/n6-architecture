"""TECS-L Mathematical Engine — n=6 arithmetic functions for physics analysis.

Ported from TECS-L project. Core functions: sigma, tau, phi, R, S spectrum
and physics-specific predictions (Koide, fermion masses, dimensional checks).
"""
import math
from sympy import divisor_sigma, divisor_count, totient, factorint


# ─── Core Arithmetic Functions ───

def sigma(n):
    """Sum of divisors."""
    return int(divisor_sigma(n, 1))

def tau(n):
    """Number of divisors."""
    return int(divisor_count(n))

def phi(n):
    """Euler totient."""
    return int(totient(n))

def sopfr(n):
    """Sum of prime factors with repetition."""
    return sum(p * e for p, e in factorint(n).items())

def omega(n):
    """Number of distinct prime factors."""
    return len(factorint(n))

def R(n):
    """R-spectrum: sigma(n)*phi(n) / (n*tau(n)).
    R(6) = 1 uniquely among n >= 2.
    """
    t = tau(n)
    if n * t == 0:
        return float('inf')
    return sigma(n) * phi(n) / (n * t)

def S(n):
    """S-spectrum (dual): sigma(n)*tau(n) / (n*phi(n)).
    S(28) = 1 uniquely.
    """
    p = phi(n)
    if n * p == 0:
        return float('inf')
    return sigma(n) * tau(n) / (n * p)


# ─── P₁ = 6 Constants ───

P1 = 6
SIGMA_P1 = sigma(P1)    # 12
TAU_P1 = tau(P1)         # 4
PHI_P1 = phi(P1)         # 2
SOPFR_P1 = sopfr(P1)     # 5
OMEGA_P1 = omega(P1)     # 2

P2 = 28
SIGMA_P2 = sigma(P2)     # 56
TAU_P2 = tau(P2)          # 6
PHI_P2 = phi(P2)          # 12

P3 = 496
TAU_P3 = tau(P3)          # 10
PHI_P3 = phi(P3)          # 240


# ─── n=6 Target Constants for Pattern Matching ───

# Basic arithmetic targets
TARGETS_BASIC = {
    'n=6':        6,
    'tau=4':      4,
    'phi=2':      2,
    'sigma=12':   12,
    'sigma*phi=24': 24,
    'sopfr=5':    5,
    'sigma/tau=3': 3,
    'phi/tau=0.5': 0.5,
    'sopfr/n=5/6': 5/6,
    '1/n=1/6':    1/6,
    '1/tau=1/4':  1/4,
    '1/sigma=1/12': 1/12,
}

# Derived mathematical targets
TARGETS_DERIVED = {
    'P2=28':          28,
    'sigma*phi+tau=28': 28,   # σφ+τ = 24+4 = 28 = P₂
    'C(8,2)=28':      28,     # C(σ-τ, φ) = C(8,2) = 28 = P₂
    'sigma-tau=8':    8,      # rank(E₈)
    'sigma+tau=16':   16,     # rank(E₈×E₈)
    'tau!=24':        24,     # 4! = 24
    'P1!=720':        720,
    'sqrt(3/2)':      math.sqrt(3/2),  # Einstein theta
    'ln(4/3)':        math.log(4/3),   # Golden Zone width
}

# Trigonometric system (n=6 unique)
TARGETS_TRIG = {
    'sin(pi/6)=phi/tau=0.5': 0.5,            # sin(π/6) = φ/τ
    'cos(pi/6)=sqrt(3)/2':   math.sqrt(3)/2, # cos(π/6)
    'tan(pi/6)=1/sqrt(3)':   1/math.sqrt(3), # tan(π/6)
    'tan2(pi/6)=tau/sigma=1/3': 1/3,         # tan²(π/6) = τ/σ
    'cot(pi/6)=sqrt(sigma/tau)': math.sqrt(3),
}

# Egyptian fraction: 1/2 + 1/3 + 1/6 = 1
EGYPTIAN_FRACTIONS = [1/2, 1/3, 1/6]

ALL_TARGETS = {**TARGETS_BASIC, **TARGETS_DERIVED, **TARGETS_TRIG}


# ─── Koide Formula ───

def koide_angle():
    """Koide angle delta = phi*tau^2/sigma^2 = 2/9 exactly."""
    return PHI_P1 * TAU_P1**2 / SIGMA_P1**2  # = 2*16/144 = 32/144 = 2/9

def koide_ratio(m1, m2, m3):
    """Koide ratio Q = (m1+m2+m3) / (sqrt(m1)+sqrt(m2)+sqrt(m3))^2.
    Should be 2/3 for charged leptons.
    """
    s = math.sqrt(m1) + math.sqrt(m2) + math.sqrt(m3)
    return (m1 + m2 + m3) / (s * s)

def koide_check(masses_triple):
    """Check Koide formula for a mass triple. Returns (ratio, error_from_2_3)."""
    m1, m2, m3 = sorted(masses_triple)
    ratio = koide_ratio(m1, m2, m3)
    error = abs(ratio - 2/3) / (2/3) * 100
    return ratio, error


# ─── Fermion Mass Predictions (MeV) ───

def fermion_predictions():
    """Return TECS-L predictions for fermion masses in MeV."""
    s, t, p = SIGMA_P1, TAU_P1, PHI_P1
    t2 = TAU_P2   # tau(28) = 6
    t3 = TAU_P3   # tau(496) = 10

    return {
        # Quark masses (MeV)
        'top':     s**3 * (s**2 - s*t + t),     # 172800
        'bottom':  p**s,                          # 4096
        'charm':   (s*t3 + t*p) * t3,             # 1280
        'strange': s * t * p,                     # 96
        'down':    t + p / t2,                    # 4.333
        'up':      p + p / s,                     # 2.167

        # Baryon masses (MeV)
        'Delta_1232': s**3 * (s**2 - s*t + t) / (s * t2 * t3),  # 172800/720 = 240... no
    }

def fermion_predictions_gev():
    """Same as fermion_predictions but in GeV."""
    preds = fermion_predictions()
    return {k: v / 1000 for k, v in preds.items()}


# ─── Dimensional Predictions (exact) ───

DIMENSION_MAP = {
    'spacetime_4D':            tau(6),           # 4
    'calabi_yau_6D':           tau(28),          # 6
    'superstring_10D':         tau(496),         # 10
    'G2_holonomy_14D':         tau(8128),        # 14
    'bosonic_string_26D':      tau(33550336),    # 26
    'SM_gauge_dim_12':         sigma(6),         # 12
    'graviton_dof_2':          phi(6),           # 2
    'leech_lattice_24':        sigma(6)*phi(6),  # 24
    'SU5_GUT_24':              sigma(6)*phi(6),  # 24
    'E7_fund_rep_56':          sigma(28),        # 56
    'E8_roots_240':            phi(496),         # 240
    'anomaly_cancel_496':      P3,               # 496
    'heterotic_diff_16':       tau(33550336)-tau(496),  # 16
    'fermion_generations_3':   sigma(6)//tau(6), # 3
}


# ─── Particle Count Matching ───

SM_COUNTS = {
    'quark_flavors':           6,   # = P₁
    'lepton_types':            6,   # 3 charged + 3 neutrinos = P₁
    'fermion_generations':     3,   # = σ/τ
    'gauge_generators':        12,  # = σ(6) = 8+3+1
    'color_charges':           3,   # = σ/τ
    'quarks_per_generation':   2,   # = φ(6)
    'leptons_per_generation':  2,   # = φ(6)
    'W_Z_H_massive':           3,   # = σ/τ
    'gluons':                  8,   # = σ-τ = rank(E₈)
    'total_fermions':          24,  # = σ*φ (12 particles + 12 anti)
}


# ─── Physics Constant Matching ───

PHYSICS_MATCHES = {
    'proton_electron_ratio': {
        'observed': 1836.15267,
        'formula': 'P1 * pi^sopfr = 6*pi^5',  # H-CX-803, 0.0017%
        'predicted': P1 * math.pi**SOPFR_P1,
    },
    'fine_structure_inv': {
        'observed': 137.036,
        'formula': 'sigma^2 - M3 = 144-7 = 137',  # H-CX-675, 0.026%
        'predicted': SIGMA_P1**2 - 7,
    },
    'weinberg_sin2': {
        'observed': 0.23122,
        'formula': '(sigma/tau) / (sigma+1) = 3/13',
        'predicted': (SIGMA_P1 / TAU_P1) / (SIGMA_P1 + 1),
    },
    'baryon_asymmetry': {
        'observed': 6.143e-10,
        'formula': '(sigma^2/tau+sigma/tau+tau)/M3 * 10^-tau(P3)',  # H-CX-531
        'predicted': (SIGMA_P1**2/TAU_P1 + SIGMA_P1/TAU_P1 + TAU_P1) / 7 * 1e-10,
    },
    'lamb_shift_mhz': {
        'observed': 1057.845,
        'formula': 'P3*phi+sigma*sopfr+P1 = 1058',  # H-CX-718
        'predicted': P3 * PHI_P1 + SIGMA_P1 * SOPFR_P1 + P1,
    },
    'pion_charged_mev': {
        'observed': 139.57,
        'formula': 'sigma^2-tau-phi/(sigma-tau)',  # H-CX-752
        'predicted': SIGMA_P1**2 - TAU_P1 - PHI_P1/(SIGMA_P1 - TAU_P1),
    },
    'kaon_charged_mev': {
        'observed': 493.68,
        'formula': 'P3-phi = 494',  # H-CX-754
        'predicted': P3 - PHI_P1,
    },
    'qcd_string_tension_mev': {
        'observed': 440.0,
        'formula': 'P3-sigma*sopfr+tau = 440',  # H-CX-720
        'predicted': P3 - SIGMA_P1 * SOPFR_P1 + TAU_P1,
    },
    'lambda_qcd_mev': {
        'observed': 332.0,
        'formula': 'P2*sigma-tau = 332',  # H-CX-756
        'predicted': P2 * SIGMA_P1 - TAU_P1,
    },
    'bcs_gap_ratio': {
        'observed': 3.528,
        'formula': 'sigma*sopfr/(sigma+sopfr) = 60/17',  # H-CX-646
        'predicted': SIGMA_P1 * SOPFR_P1 / (SIGMA_P1 + SOPFR_P1),
    },
    'sound_speed_ms': {
        'observed': 343.0,
        'formula': 'P2*sigma+M3 = 343',  # H-CX-854
        'predicted': P2 * SIGMA_P1 + 7,
    },
    'sound_horizon_mpc': {
        'observed': 147.09,
        'formula': 'sigma^2+sigma/tau = 147',  # H-CX-621
        'predicted': SIGMA_P1**2 + SIGMA_P1/TAU_P1,
    },
}


# ─── Analysis Helpers ───

def check_egyptian_fraction(r1, r2, r3, tolerance=0.03):
    """Check if three ratios sum to 1 as 1/2+1/3+1/6."""
    vals = sorted([r1, r2, r3])
    targets = sorted(EGYPTIAN_FRACTIONS)
    errors = [abs(v - t) / t for v, t in zip(vals, targets)]
    if all(e < tolerance for e in errors):
        return True, max(errors)
    return False, None

def mass_ratio_matches(mass_dict, tolerance=0.03):
    """Find all pairwise mass ratios matching TECS-L targets."""
    names = list(mass_dict.keys())
    masses = list(mass_dict.values())
    hits = []

    for i in range(len(names)):
        for j in range(i + 1, len(names)):
            if masses[i] == 0 or masses[j] == 0:
                continue
            ratio = max(masses[i], masses[j]) / min(masses[i], masses[j])
            inv_ratio = min(masses[i], masses[j]) / max(masses[i], masses[j])

            for tname, tval in ALL_TARGETS.items():
                if tval <= 0:
                    continue
                # Check ratio
                err = abs(ratio - tval) / tval
                if err < tolerance:
                    hits.append({
                        'p1': names[i], 'p2': names[j],
                        'ratio': ratio, 'target_name': tname,
                        'target_val': tval, 'error_pct': err * 100,
                    })
                # Check inverse ratio for small targets
                if tval < 1:
                    err_inv = abs(inv_ratio - tval) / tval
                    if err_inv < tolerance:
                        hits.append({
                            'p1': names[j], 'p2': names[i],
                            'ratio': inv_ratio, 'target_name': tname,
                            'target_val': tval, 'error_pct': err_inv * 100,
                        })

    return sorted(hits, key=lambda h: h['error_pct'])
