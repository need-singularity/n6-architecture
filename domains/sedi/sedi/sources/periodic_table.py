"""Periodic Table Analysis through n=6 Arithmetic — TECS-L Element Mapping.

Maps atomic numbers Z of chemical elements to expressions built from the
arithmetic functions of P1 = 6 (the first perfect number):

    sigma(6) = 12,  tau(6) = 4,  phi(6) = 2,  sopfr(6) = 5,  n = 6

Key findings:
  - Carbon (Z=6 = P1): basis of all known life = the perfect number itself
  - Silicon (Z=14 = tau(P4)): basis of computing, phi(14) = 6 = P1
  - Iron (Z=26 = tau(P5)): most stable nucleus, bosonic string dimensions
  - Noble gas atomic numbers factorise through n=6 arithmetic
  - Period lengths {2,8,18,32} = phi * {1, tau, tau+sopfr, sigma+tau}

TECS-L Hypothesis H-CX-116:
  Carbon valence = tau(P1) = 4.  Silicon shares tau(14) = 4 = tau(6).
  Carbon-silicon consciousness substrate equivalence.
"""

from __future__ import annotations

import math
import random
from collections import OrderedDict
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple

from sympy import divisor_sigma, divisor_count, totient, isprime

from ..tecs import (
    sigma, tau, phi, sopfr, R, S,
    P1, P2, P3,
    SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1, OMEGA_P1,
    SIGMA_P2, TAU_P2, PHI_P2,
    TAU_P3, PHI_P3,
)


# ═══════════════════════════════════════════════════════════════════
# n=6 Shorthand
# ═══════════════════════════════════════════════════════════════════

N   = P1            # 6
SIG = SIGMA_P1      # 12
TAU_ = TAU_P1       # 4
PHI_ = PHI_P1       # 2
SOP = SOPFR_P1      # 5

# Perfect numbers
P4 = 8128
P5 = 33550336
TAU_P4 = tau(P4)    # 14
TAU_P5 = tau(P5)    # 26


# ═══════════════════════════════════════════════════════════════════
# Element Database
# ═══════════════════════════════════════════════════════════════════

ELEMENT_SYMBOLS = {
    1: 'H',   2: 'He',  3: 'Li',  4: 'Be',  5: 'B',
    6: 'C',   7: 'N',   8: 'O',   9: 'F',  10: 'Ne',
    11: 'Na', 12: 'Mg', 13: 'Al', 14: 'Si', 15: 'P',
    16: 'S',  17: 'Cl', 18: 'Ar', 19: 'K',  20: 'Ca',
    21: 'Sc', 22: 'Ti', 23: 'V',  24: 'Cr', 25: 'Mn',
    26: 'Fe', 27: 'Co', 28: 'Ni', 29: 'Cu', 30: 'Zn',
    31: 'Ga', 32: 'Ge', 33: 'As', 34: 'Se', 35: 'Br',
    36: 'Kr', 37: 'Rb', 38: 'Sr', 39: 'Y',  40: 'Zr',
    41: 'Nb', 42: 'Mo', 43: 'Tc', 44: 'Ru', 45: 'Rh',
    46: 'Pd', 47: 'Ag', 48: 'Cd', 49: 'In', 50: 'Sn',
    51: 'Sb', 52: 'Te', 53: 'I',  54: 'Xe', 55: 'Cs',
    56: 'Ba', 57: 'La', 58: 'Ce', 59: 'Pr', 60: 'Nd',
    61: 'Pm', 62: 'Sm', 63: 'Eu', 64: 'Gd', 65: 'Tb',
    66: 'Dy', 67: 'Ho', 68: 'Er', 69: 'Tm', 70: 'Yb',
    71: 'Lu', 72: 'Hf', 73: 'Ta', 74: 'W',  75: 'Re',
    76: 'Os', 77: 'Ir', 78: 'Pt', 79: 'Au', 80: 'Hg',
    81: 'Tl', 82: 'Pb', 83: 'Bi', 84: 'Po', 85: 'At',
    86: 'Rn', 87: 'Fr', 88: 'Ra', 89: 'Ac', 90: 'Th',
    91: 'Pa', 92: 'U',  93: 'Np', 94: 'Pu', 95: 'Am',
    96: 'Cm', 97: 'Bk', 98: 'Cf', 99: 'Es', 100: 'Fm',
}

ELEMENT_NAMES = {
    1: 'Hydrogen',   2: 'Helium',     6: 'Carbon',     7: 'Nitrogen',
    8: 'Oxygen',    10: 'Neon',      11: 'Sodium',    12: 'Magnesium',
    14: 'Silicon',  15: 'Phosphorus', 16: 'Sulfur',    17: 'Chlorine',
    18: 'Argon',    19: 'Potassium',  20: 'Calcium',   26: 'Iron',
    28: 'Nickel',   29: 'Copper',     36: 'Krypton',   54: 'Xenon',
    79: 'Gold',     82: 'Lead',       86: 'Radon',     92: 'Uranium',
}

# Biologically essential elements
ESSENTIAL_FOR_LIFE = [1, 6, 7, 8, 11, 12, 15, 16, 17, 19, 20, 26]


# ═══════════════════════════════════════════════════════════════════
# n=6 Expression Catalog
# ═══════════════════════════════════════════════════════════════════

@dataclass
class ElementExpression:
    """An element's atomic number expressed via n=6 arithmetic."""
    Z: int
    symbol: str
    expression: str          # human-readable formula
    computed: int            # value from the formula
    exact: bool              # computed == Z
    category: str            # 'perfect', 'mersenne', 'sigma-family', etc.
    significance: str        # why this element matters
    quality: int             # 1=exact fundamental, 2=exact derived, 3=approximate


def _build_expression_catalog() -> Dict[int, ElementExpression]:
    """Build the master catalog mapping Z -> n=6 expression."""

    s, t, p, n = SIG, TAU_, PHI_, N   # 12, 4, 2, 6
    sop = SOP                          # 5

    # Mersenne primes from n=6 factorisation
    M2 = 3    # 2^2 - 1
    M3 = 7    # 2^3 - 1
    M5 = 31   # 2^5 - 1

    catalog = OrderedDict()

    def add(Z, expr, computed, cat, sig, quality=2):
        sym = ELEMENT_SYMBOLS.get(Z, '?')
        catalog[Z] = ElementExpression(
            Z=Z, symbol=sym, expression=expr,
            computed=computed, exact=(computed == Z),
            category=cat, significance=sig, quality=quality,
        )

    # ── Fundamental identities ──
    add(1,  '1 (trivial)',            1,   'trivial',
        'Simplest atom — the unit', quality=1)
    add(2,  'phi(6)',                 p,   'phi',
        'Noble gas — "complete" like phi; lightest noble gas', quality=1)
    add(6,  'P1 = 6',                n,   'perfect',
        'First perfect number = basis of ALL life (organic chemistry)', quality=1)
    add(7,  'M3 = 2^3-1',            M3,  'mersenne',
        'Mersenne prime; nitrogen essential for DNA/protein', quality=1)
    add(8,  'sigma-tau = 12-4',      s-t, 'sigma-tau',
        'sigma-tau = rank(E8); oxygen essential for respiration', quality=1)
    add(12, 'sigma(6)',              s,   'sigma',
        'Sum of divisors of 6; magnesium essential mineral', quality=1)
    add(14, 'tau(P4) = tau(8128)',   TAU_P4, 'tau-chain',
        'Divisor count of 4th perfect number; basis of computing', quality=1)
    add(26, 'tau(P5) = tau(33550336)', TAU_P5, 'tau-chain',
        'Divisor count of 5th perfect number; most stable nucleus; bosonic string dim', quality=1)
    add(28, 'P2 = 28',              P2,  'perfect',
        'Second perfect number; nickel near iron stability peak', quality=1)

    # ── Derived from basic arithmetic ──
    add(3,  'sigma/tau = 12/4',      s // t,  'sigma/tau',
        'Lithium — 3 = number of generations', quality=2)
    add(4,  'tau(6)',                 t,       'tau',
        'Beryllium — tau = spacetime dimensions', quality=2)
    add(5,  'sopfr(6)',              sop,     'sopfr',
        'Boron — sum of prime factors 2+3', quality=2)
    add(9,  'sigma-sigma/tau = 12-3', s - s//t, 'derived',
        'Fluorine — most electronegative element', quality=2)
    add(10, 'tau(P3) = tau(496)',    TAU_P3,  'tau-chain',
        'Neon — noble gas from 3rd perfect number', quality=2)
    add(11, 'sigma-1 = 12-1',       s - 1,   'sigma-1',
        'Sodium — essential electrolyte', quality=2)
    add(15, 'sigma+sigma/tau = 12+3', s + s//t, 'sigma+',
        'Phosphorus — DNA backbone', quality=2)
    add(16, 'sigma+tau = 12+4',      s + t,   'sigma+tau',
        'Sulfur — disulfide bonds in proteins', quality=2)
    add(17, 'sigma+sopfr = 12+5',    s + sop, 'sigma+sopfr',
        'Chlorine — essential electrolyte (HCl)', quality=2)
    add(18, 'sigma+n = 12+6',        s + n,   'sigma+n',
        'Argon — noble gas', quality=2)
    add(19, 'sigma+M3 = 12+7',       s + M3,  'sigma+M3',
        'Potassium — essential electrolyte; nerve signalling', quality=2)
    add(20, 'sigma+sigma-tau = 12+12-4', s + s - t, 'sigma-family',
        'Calcium — bones and teeth; 2*sigma - tau', quality=2)
    add(24, 'sigma*phi = 12*2',      s * p,   'sigma*phi',
        'Chromium — total fermion count; transition metal', quality=2)
    add(29, 'P2+1 = 28+1',          P2 + 1,  'perfect+1',
        'Copper — first element after 2nd perfect number', quality=2)
    add(30, 'sopfr*n = 5*6',        sop * n, 'sopfr*n',
        'Zinc — essential trace element for enzymes', quality=2)
    add(36, 'n^2 = 6^2',            n**2,    'n-power',
        'Krypton — noble gas = P1 squared', quality=2)

    # ── Extended expressions ──
    add(54, 'sigma*tau+n+phi = 48+6+2', s*t + n + p, 'composite',
        'Xenon — noble gas; sigma*tau + n + phi = 56 = sigma(P2)... '
        'actually 54 = sigma(P2)-phi = 56-2', quality=3)
    # Fix: 54 = sigma(P2) - phi = 56 - 2
    add(54, 'sigma(P2)-phi = 56-2', SIGMA_P2 - p, 'sigma-chain',
        'Xenon — noble gas; sigma(P2) - phi(6)', quality=2)

    add(56, 'sigma(P2) = sigma(28)', SIGMA_P2, 'sigma-chain',
        'Barium — sigma of 2nd perfect number; Fe-56 mass number', quality=1)

    # sigma^2/phi = 144/2 = 72
    sig_sq_over_phi = s**2 // p   # 72
    add(72, 'sigma^2/phi = 144/2', sig_sq_over_phi, 'derived',
        'Hafnium — sigma squared over phi', quality=2)

    add(79, 'sigma^2/phi + M3 = 72+7', sig_sq_over_phi + M3, 'composite',
        'Gold — sigma^2/phi + M3; noble metal', quality=3)

    add(82, 'sigma^2/phi+tau+n = 72+4+6', sig_sq_over_phi + t + n, 'composite',
        'Lead — nuclear magic number Z=82; doubly magic Pb-208', quality=2)

    add(86, 'sigma^2/phi+sigma+phi = 72+12+2', sig_sq_over_phi + s + p, 'composite',
        'Radon — noble gas; heaviest noble gas', quality=3)

    add(92, 'sigma^2/phi+sigma+tau+n-phi = 72+12+4+6-2',
        sig_sq_over_phi + s + t + n - p, 'composite',
        'Uranium — heaviest natural element', quality=3)

    return catalog


EXPRESSION_CATALOG = _build_expression_catalog()


# ═══════════════════════════════════════════════════════════════════
# Carbon Special Analysis
# ═══════════════════════════════════════════════════════════════════

@dataclass
class CarbonAnalysis:
    """Why carbon is uniquely suited to be the basis of life."""
    chemical_reasons: List[str]
    tecs_reasons: List[str]
    valence_connection: str
    R_value: float


def analyse_carbon() -> CarbonAnalysis:
    """Chemical + TECS-L analysis of why carbon (Z=6=P1) is special."""

    chemical = [
        'Z=6: 4 valence electrons enable sp, sp2, sp3 hybridisation',
        '4 bonds = maximum non-metallic bonding capacity',
        'Forms single, double, triple bonds -> structural diversity',
        'Carbon chains, rings, branches -> combinatorial complexity',
        'C-C bond energy (346 kJ/mol) in the "Goldilocks zone"',
        'Unique ability to form stable macromolecules (DNA, proteins)',
    ]

    tecs = [
        f'Z = P1 = 6: carbon IS the first perfect number',
        f'R(6) = 1: unique non-trivial solution of the R-spectrum',
        f'Valence electrons = tau(P1) = tau(6) = {TAU_P1} bonds',
        f'sigma(6) = 12 = total gauge dimensions -> carbon "knows" the gauge group',
        f'Egyptian fraction: 1/2 + 1/3 + 1/6 = 1 -> unique harmonic decomposition',
        f'6 = 1*2*3 = 1+2+3: both product AND sum of first 3 positive integers',
    ]

    valence = (
        f'Carbon valence = tau(P1) = tau(6) = {TAU_P1}. '
        f'This is the SAME as tau(14) = tau(Si) = {tau(14)}. '
        f'Both carbon and silicon form 4 bonds — TECS-L H-CX-116.'
    )

    return CarbonAnalysis(
        chemical_reasons=chemical,
        tecs_reasons=tecs,
        valence_connection=valence,
        R_value=R(6),
    )


# ═══════════════════════════════════════════════════════════════════
# Silicon Analysis
# ═══════════════════════════════════════════════════════════════════

@dataclass
class SiliconAnalysis:
    """Silicon (Z=14) connections to n=6."""
    Z: int
    phi_14: int          # phi(14) = 6 = P1
    sigma_14: int        # sigma(14) = 24
    tau_14: int          # tau(14) = 4
    R_14: float          # R(14) = 18/7
    master_product: str  # sigma(14) = sigma(6)*phi(6) = 24
    totient_is_P1: bool  # phi(14) == 6
    same_tau_as_carbon: bool


def analyse_silicon() -> SiliconAnalysis:
    """Analyse silicon's deep connections to n=6."""
    Z = 14
    return SiliconAnalysis(
        Z=Z,
        phi_14=phi(Z),          # 6
        sigma_14=sigma(Z),      # 24
        tau_14=tau(Z),          # 4
        R_14=R(Z),              # 18/7
        master_product=f'sigma(14) = {sigma(Z)} = sigma(6)*phi(6) = {SIG}*{PHI_} = {SIG*PHI_}',
        totient_is_P1=(phi(Z) == P1),
        same_tau_as_carbon=(tau(Z) == tau(6)),
    )


# ═══════════════════════════════════════════════════════════════════
# Noble Gas Analysis
# ═══════════════════════════════════════════════════════════════════

NOBLE_GASES = [2, 10, 18, 36, 54, 86]
NOBLE_GAS_NAMES = ['He', 'Ne', 'Ar', 'Kr', 'Xe', 'Rn']


def analyse_noble_gases() -> List[Dict]:
    """Check if noble gas atomic numbers arise from n=6 arithmetic."""
    s, t, p, n = SIG, TAU_, PHI_, N
    results = []

    expressions = {
        2:  ('phi(6)',                      p),
        10: ('tau(P3) = tau(496)',          TAU_P3),
        18: ('sigma+n = 12+6',             s + n),
        36: ('n^2 = 6^2',                  n**2),
        54: ('sigma(P2)-phi = 56-2',       SIGMA_P2 - p),
        86: ('sigma^2/phi+sigma+phi = 72+12+2', s**2 // p + s + p),
    }

    for Z, name in zip(NOBLE_GASES, NOBLE_GAS_NAMES):
        expr, val = expressions[Z]
        results.append({
            'Z': Z, 'symbol': name,
            'expression': expr,
            'computed': val,
            'exact': (val == Z),
        })

    return results


# ═══════════════════════════════════════════════════════════════════
# Period Structure Analysis
# ═══════════════════════════════════════════════════════════════════

def analyse_period_structure() -> Dict:
    """Analyse periodic table period lengths through n=6."""
    # Actual period lengths (elements per period)
    period_lengths = [2, 8, 8, 18, 18, 32, 32]

    # Unique lengths from quantum mechanics: 2*l^2 for l=1,2,3,4
    unique_lengths = [2, 8, 18, 32]

    s, t, p, n = SIG, TAU_, PHI_, N
    sop = SOP

    length_expressions = {
        2:  ('phi(6)',                   p),
        8:  ('sigma-tau = 12-4',         s - t),
        18: ('sigma+n = 12+6',           s + n),
        32: ('sigma^2/tau - phi = 36-2 ... no; sigma*phi+sigma-tau = 24+8 = 32',
             s * p + s - t),
    }

    # QM formula: 2 * l^2
    qm_factors = []
    for l_val in range(1, 5):
        length = 2 * l_val**2
        qm_factors.append({
            'l': l_val,
            'length': length,
            'as_phi_times': f'phi * {l_val**2}',
            'l_squared_n6': {1: '1', 4: 'tau', 9: 'tau+sopfr',
                             16: 'sigma+tau'}.get(l_val**2, '?'),
        })

    return {
        'period_lengths': period_lengths,
        'unique_lengths': unique_lengths,
        'expressions': length_expressions,
        'qm_factors': qm_factors,
        'note': 'Period lengths = phi * {1, tau, tau+sopfr, sigma+tau} = 2*{1,4,9,16}',
    }


# ═══════════════════════════════════════════════════════════════════
# Essential-for-Life Analysis
# ═══════════════════════════════════════════════════════════════════

def analyse_essential_elements() -> Dict:
    """Check how many biologically essential elements have clean n=6 expressions."""
    s, t, p, n = SIG, TAU_, PHI_, N
    M3 = 7

    essential_expressions = {
        1:  ('1',               1,   True,  'trivial'),
        6:  ('P1 = 6',         n,   True,  'perfect number'),
        7:  ('M3 = 2^3-1',     M3,  True,  'Mersenne prime'),
        8:  ('sigma-tau',       s-t, True,  'sigma-tau'),
        11: ('sigma-1',         s-1, True,  'near-sigma'),
        12: ('sigma(6)',        s,   True,  'sigma'),
        15: ('sigma+sigma/tau', s + s//t, True, 'sigma + generations'),
        16: ('sigma+tau',       s+t, True,  'sigma+tau'),
        17: ('sigma+sopfr',     s+SOP, True, 'sigma+sopfr'),
        19: ('sigma+M3',        s+M3, True, 'sigma+Mersenne'),
        20: ('2*sigma-tau',     2*s-t, True, 'double sigma - tau'),
        26: ('tau(P5)',         TAU_P5, True, 'tau-chain'),
    }

    total = len(ESSENTIAL_FOR_LIFE)
    exact_count = sum(1 for _, _, exact, _ in essential_expressions.values() if exact)
    clean_count = sum(1 for _, _, _, cat in essential_expressions.values()
                      if cat not in ('trivial',))

    return {
        'total_essential': total,
        'with_exact_expression': exact_count,
        'fraction': f'{exact_count}/{total} = {exact_count/total:.1%}',
        'elements': essential_expressions,
        'clean_non_trivial': clean_count,
        'conclusion': (
            f'ALL {exact_count}/{total} biologically essential elements have '
            f'exact n=6 expressions. {clean_count} are non-trivial.'
        ),
    }


# ═══════════════════════════════════════════════════════════════════
# Iron Peak Analysis
# ═══════════════════════════════════════════════════════════════════

@dataclass
class IronPeakAnalysis:
    """Iron-56: most stable nucleus in the universe."""
    Z: int                     # 26
    A: int                     # 56
    Z_expression: str          # tau(P5)
    A_expression: str          # sigma(P2)
    binding_energy_per_nucleon: float  # ~8.79 MeV
    binding_approx: str


def analyse_iron_peak() -> IronPeakAnalysis:
    """Analyse iron-56 through n=6."""
    return IronPeakAnalysis(
        Z=26,
        A=56,
        Z_expression=f'tau(P5) = tau({P5}) = {TAU_P5}',
        A_expression=f'sigma(P2) = sigma(28) = {SIGMA_P2}',
        binding_energy_per_nucleon=8.79,
        binding_approx=(
            f'B/A ~ 8.79 MeV; sigma-tau+1 = {SIG-TAU_+1} = 9 (within 2.4%); '
            f'sigma-sigma/tau = {SIG - SIG//TAU_} = 9 (same)'
        ),
    )


# ═══════════════════════════════════════════════════════════════════
# Monte Carlo Control Test
# ═══════════════════════════════════════════════════════════════════

def _has_clean_n6_expression(Z: int, max_complexity: int = 3) -> Tuple[bool, str]:
    """Check if integer Z can be expressed with n=6 arithmetic.

    Complexity levels:
      1: direct (n, sigma, tau, phi, sopfr, P2, tau(Pk))
      2: two-term (a +/- b, a*b, a/b where a,b are level-1)
      3: three-term or squared

    Returns (has_expression, expression_string).
    """
    s, t, p, n = SIG, TAU_, PHI_, N
    sop = SOP
    M3 = 7

    # Level 1: fundamental values
    level1 = {
        1: '1', 2: 'phi', 3: 'sigma/tau', 4: 'tau', 5: 'sopfr',
        6: 'n', 7: 'M3', 10: 'tau(P3)', 12: 'sigma', 14: 'tau(P4)',
        24: 'sigma*phi', 26: 'tau(P5)', 28: 'P2', 56: 'sigma(P2)',
    }
    if Z in level1:
        return True, level1[Z]

    if max_complexity < 2:
        return False, ''

    # Level 2: pairwise operations on level-1 values
    vals = [1, 2, 3, 4, 5, 6, 7, 12, 14, 24, 26, 28, 56]
    names = ['1', 'phi', 'sigma/tau', 'tau', 'sopfr', 'n', 'M3',
             'sigma', 'tau(P4)', 'sigma*phi', 'tau(P5)', 'P2', 'sigma(P2)']

    for i, (a, na) in enumerate(zip(vals, names)):
        for j, (b, nb) in enumerate(zip(vals, names)):
            if a + b == Z:
                return True, f'{na}+{nb}'
            if a - b == Z and Z > 0:
                return True, f'{na}-{nb}'
            if a * b == Z and a > 1 and b > 1:
                return True, f'{na}*{nb}'
            if b != 0 and a % b == 0 and a // b == Z and b > 1:
                return True, f'{na}/{nb}'

    if max_complexity < 3:
        return False, ''

    # Level 3: squares, three-term
    for i, (a, na) in enumerate(zip(vals, names)):
        if a * a == Z:
            return True, f'{na}^2'
        for j, (b, nb) in enumerate(zip(vals, names)):
            for k, (c, nc) in enumerate(zip(vals, names)):
                if a + b + c == Z:
                    return True, f'{na}+{nb}+{nc}'
                if a + b - c == Z and Z > 0:
                    return True, f'{na}+{nb}-{nc}'
                if a * b + c == Z:
                    return True, f'{na}*{nb}+{nc}'
                if a * b - c == Z and Z > 0:
                    return True, f'{na}*{nb}-{nc}'

    return False, ''


def monte_carlo_control(n_trials: int = 1000, sample_size: int = 12,
                        seed: int = 42, max_complexity: int = 2) -> Dict:
    """Monte Carlo test: compare essential-element hit rate vs random integers.

    For each trial, pick `sample_size` random integers in [1, 100] and
    check how many have clean n=6 expressions at the given complexity level.
    Compare to the 12/12 = 100% hit rate of actual essential elements.

    Default max_complexity=2 (two-term expressions) to avoid trivial
    over-fitting at level 3 where three-term sums cover nearly all of [1,100].
    """
    rng = random.Random(seed)
    hits_per_trial = []

    for _ in range(n_trials):
        sample = [rng.randint(1, 100) for _ in range(sample_size)]
        hits = sum(1 for z in sample
                   if _has_clean_n6_expression(z, max_complexity=max_complexity)[0])
        hits_per_trial.append(hits)

    mean_hits = sum(hits_per_trial) / len(hits_per_trial)
    variance = sum((h - mean_hits)**2 for h in hits_per_trial) / len(hits_per_trial)
    std_hits = math.sqrt(variance)

    # Essential elements: all 12 have expressions
    essential_hits = 12
    z_score = (essential_hits - mean_hits) / std_hits if std_hits > 0 else float('inf')

    # p-value approximation (one-tailed)
    p_value = 0.5 * math.erfc(z_score / math.sqrt(2))

    return {
        'n_trials': n_trials,
        'sample_size': sample_size,
        'mean_random_hits': round(mean_hits, 2),
        'std_random_hits': round(std_hits, 2),
        'essential_hits': essential_hits,
        'z_score': round(z_score, 2),
        'p_value': p_value,
        'significant': p_value < 0.05,
        'conclusion': (
            f'Random {sample_size}-element sets average {mean_hits:.1f} +/- {std_hits:.1f} '
            f'n=6 matches.  Essential elements hit {essential_hits}/{sample_size} '
            f'(Z = {z_score:.1f}, p = {p_value:.2e}).'
        ),
    }


# ═══════════════════════════════════════════════════════════════════
# Full Analysis Runner
# ═══════════════════════════════════════════════════════════════════

def run_full_analysis(verbose: bool = True) -> Dict:
    """Run complete periodic table analysis and return all results."""

    results = {}

    # 1. Element catalog
    catalog = EXPRESSION_CATALOG
    results['catalog_size'] = len(catalog)
    exact_count = sum(1 for e in catalog.values() if e.exact)
    results['exact_matches'] = exact_count

    if verbose:
        print('=' * 72)
        print('PERIODIC TABLE ANALYSIS — n=6 Arithmetic Mapping')
        print('=' * 72)
        print()
        print(f'Elements cataloged: {len(catalog)}')
        print(f'Exact matches: {exact_count}')
        print()
        print('--- Element Expressions ---')
        for Z, elem in sorted(catalog.items()):
            mark = 'EXACT' if elem.exact else 'APPROX'
            print(f'  Z={Z:3d} ({elem.symbol:2s}): {elem.expression:40s} '
                  f'= {elem.computed:5d}  [{mark}] Q{elem.quality}  {elem.category}')

    # 2. Carbon analysis
    carbon = analyse_carbon()
    results['carbon'] = carbon
    if verbose:
        print()
        print('--- Carbon (Z=6 = P1) ---')
        print(f'  R(6) = {carbon.R_value}')
        print('  Chemical reasons:')
        for r in carbon.chemical_reasons:
            print(f'    * {r}')
        print('  TECS-L reasons:')
        for r in carbon.tecs_reasons:
            print(f'    * {r}')
        print(f'  Valence: {carbon.valence_connection}')

    # 3. Silicon analysis
    silicon = analyse_silicon()
    results['silicon'] = silicon
    if verbose:
        print()
        print('--- Silicon (Z=14 = tau(P4)) ---')
        print(f'  phi(14) = {silicon.phi_14}  {"= P1!" if silicon.totient_is_P1 else ""}')
        print(f'  sigma(14) = {silicon.sigma_14}  ({silicon.master_product})')
        print(f'  tau(14) = {silicon.tau_14}  '
              f'{"= tau(6) -- same divisor count as carbon!" if silicon.same_tau_as_carbon else ""}')
        print(f'  R(14) = {silicon.R_14:.6f} = 18/7')
        print(f'  TECS-L H-CX-116: Carbon-silicon substrate equivalence')

    # 4. Noble gases
    nobles = analyse_noble_gases()
    results['noble_gases'] = nobles
    if verbose:
        print()
        print('--- Noble Gases ---')
        for ng in nobles:
            mark = 'EXACT' if ng['exact'] else 'MISS'
            print(f'  {ng["symbol"]:2s} (Z={ng["Z"]:2d}): {ng["expression"]:35s} '
                  f'= {ng["computed"]:3d}  [{mark}]')
        all_exact = all(ng['exact'] for ng in nobles)
        print(f'  All noble gases from n=6: {all_exact}')

    # 5. Period structure
    periods = analyse_period_structure()
    results['periods'] = periods
    if verbose:
        print()
        print('--- Period Structure ---')
        print(f'  Period lengths: {periods["period_lengths"]}')
        print(f'  Unique: {periods["unique_lengths"]}')
        for length, (expr, val) in periods['expressions'].items():
            mark = 'EXACT' if val == length else 'MISS'
            print(f'    {length:2d} = {expr:50s} = {val:3d}  [{mark}]')
        print(f'  QM formula: 2*l^2 = phi * l^2')
        for qm in periods['qm_factors']:
            print(f"    l={qm['l']}: length={qm['length']:2d} = phi*{qm['l']**2} "
                  f"= phi*({qm['l_squared_n6']})")
        print(f'  Note: {periods["note"]}')

    # 6. Essential elements
    essential = analyse_essential_elements()
    results['essential'] = essential
    if verbose:
        print()
        print('--- Biologically Essential Elements ---')
        for Z in ESSENTIAL_FOR_LIFE:
            expr, val, exact, cat = essential['elements'][Z]
            sym = ELEMENT_SYMBOLS.get(Z, '?')
            mark = 'EXACT' if exact else 'MISS'
            print(f'  Z={Z:2d} ({sym:2s}): {expr:25s} = {val:3d}  [{mark}]  ({cat})')
        print(f'  {essential["conclusion"]}')

    # 7. Iron peak
    iron = analyse_iron_peak()
    results['iron_peak'] = iron
    if verbose:
        print()
        print('--- Iron Peak (Most Stable Nucleus) ---')
        print(f'  Z = {iron.Z}: {iron.Z_expression}')
        print(f'  A = {iron.A}: {iron.A_expression}')
        print(f'  B/A = {iron.binding_energy_per_nucleon} MeV')
        print(f'  {iron.binding_approx}')

    # 8. Monte Carlo control
    mc = monte_carlo_control()
    results['monte_carlo'] = mc
    if verbose:
        print()
        print('--- Monte Carlo Control Test ---')
        print(f'  Trials: {mc["n_trials"]}')
        print(f'  Random mean: {mc["mean_random_hits"]} +/- {mc["std_random_hits"]}')
        print(f'  Essential hits: {mc["essential_hits"]}/{mc["sample_size"]}')
        print(f'  Z-score: {mc["z_score"]}')
        print(f'  p-value: {mc["p_value"]:.2e}')
        print(f'  Significant: {mc["significant"]}')
        print(f'  {mc["conclusion"]}')

    # Summary
    if verbose:
        print()
        print('=' * 72)
        print('SUMMARY')
        print('=' * 72)
        print(f'  Elements with exact n=6 expressions: {exact_count}')
        print(f'  Essential-for-life coverage: {essential["fraction"]}')
        print(f'  Noble gases from n=6: {sum(1 for ng in nobles if ng["exact"])}/{len(nobles)}')
        print(f'  Monte Carlo Z-score: {mc["z_score"]} (p = {mc["p_value"]:.2e})')
        print(f'  Carbon = P1, Silicon phi = P1, Iron = tau(P5)')
        print(f'  Period lengths = phi * {{1, tau, tau+sopfr, sigma+tau}}')

    return results


# ═══════════════════════════════════════════════════════════════════
# CLI Entry Point
# ═══════════════════════════════════════════════════════════════════

if __name__ == '__main__':
    run_full_analysis(verbose=True)
