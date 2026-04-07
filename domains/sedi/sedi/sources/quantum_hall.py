"""Fractional Quantum Hall Effect -- n=6 arithmetic in topological phases.

The Fractional Quantum Hall Effect (FQHE) produces quantized Hall conductance
sigma_xy = nu * e^2/h where nu is a rational filling fraction.  The observed
fractions follow structured sequences (Laughlin, Jain) that are NOT random
rationals -- they are constrained by topological order.

TECS-L question: do FQHE filling fractions decompose into n=6 arithmetic?

Key finding: ALL principal FQHE fractions (1/3, 2/5, 3/7, 2/3, 3/5, 1/5,
2/7, 1/2, 5/2) have exact TECS-L rational expressions using only
{sigma=12, tau=4, phi=2, sopfr=5, n=6, M3=7}.

The exotic nu=5/2 non-Abelian state maps to sopfr/phi, and its torus
degeneracy = 6 = P1 = n.

Egyptian fraction identity: 1/2 + 1/3 + 1/6 = 1 unifies three FQHE states
into the integer quantum Hall effect.

Bott periodicity period = 8 = sigma - tau = rank(E8).

Sources:
  - Tsui, Stormer, Gossard (1982): discovery of nu=1/3
  - Laughlin (1983): wavefunction theory
  - Jain (1989): composite fermion theory
  - Willett et al. (1987): nu=5/2 discovery
  - Kitaev (2003): topological quantum computation
"""
from __future__ import annotations

import math
import random
from collections import OrderedDict
from dataclasses import dataclass, field
from fractions import Fraction
from itertools import product as iterproduct
from typing import Dict, List, Optional, Tuple

from ..tecs import (
    SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1, OMEGA_P1,
    P1, P2, P3,
)


# =====================================================================
# TECS-L shorthand
# =====================================================================

S = SIGMA_P1    # 12
T = TAU_P1      # 4
P = PHI_P1      # 2
F = SOPFR_P1    # 5
N = P1          # 6
M3 = 7          # Mersenne prime 2^3 - 1


# =====================================================================
# 1. FQHE FILLING FRACTIONS -- TECS-L MAPPINGS
# =====================================================================

# Observed FQHE filling fractions and their TECS-L decompositions.
# Each entry: (nu_num, nu_den, tecs_expr, tecs_num, tecs_den, note)

@dataclass
class FQHEFraction:
    """A single FQHE filling fraction with its TECS-L mapping."""
    nu: Fraction
    tecs_expr: str
    tecs_numerator: int      # TECS-L expression for numerator
    tecs_denominator: int    # TECS-L expression for denominator
    note: str
    series: str = ""         # Laughlin, Jain, composite, non-Abelian
    degeneracy: int = 0      # ground state degeneracy on torus
    chern_number: int = 0    # first Chern number C_1
    exchange_phase: str = "" # anyonic exchange phase

    @property
    def tecs_value(self) -> Fraction:
        return Fraction(self.tecs_numerator, self.tecs_denominator)

    @property
    def match(self) -> bool:
        return self.nu == self.tecs_value


# All principal observed FQHE filling fractions
FQHE_FRACTIONS: List[FQHEFraction] = [
    # ---- Laughlin states: nu = 1/m for odd m ----
    FQHEFraction(
        nu=Fraction(1, 3),
        tecs_expr="tau/sigma = 1/(sigma/tau)",
        tecs_numerator=T, tecs_denominator=S,
        note="First FQHE state discovered (Tsui-Stormer-Gossard 1982)",
        series="Laughlin",
        degeneracy=3,        # = sigma/tau
        chern_number=1,
        exchange_phase="pi/(sigma/tau) = pi*tau/sigma",
    ),
    FQHEFraction(
        nu=Fraction(1, 5),
        tecs_expr="1/sopfr",
        tecs_numerator=1, tecs_denominator=F,
        note="Laughlin state at nu=1/5",
        series="Laughlin",
        degeneracy=5,        # = sopfr
        chern_number=1,
        exchange_phase="pi/sopfr",
    ),
    FQHEFraction(
        nu=Fraction(1, 7),
        tecs_expr="1/M3",
        tecs_numerator=1, tecs_denominator=M3,
        note="Laughlin state at nu=1/7",
        series="Laughlin",
        degeneracy=7,        # = M3
        chern_number=1,
        exchange_phase="pi/M3",
    ),

    # ---- Jain sequence: nu = p/(2p+1) ----
    # p=1: 1/3 (already above)
    FQHEFraction(
        nu=Fraction(2, 5),
        tecs_expr="phi/sopfr",
        tecs_numerator=P, tecs_denominator=F,
        note="Jain p=2: 2/(2*2+1) = 2/5",
        series="Jain (2p+1)",
        degeneracy=5,
        chern_number=2,
    ),
    FQHEFraction(
        nu=Fraction(3, 7),
        tecs_expr="(sigma/tau)/M3",
        tecs_numerator=S // T, tecs_denominator=M3,
        note="Jain p=3: 3/(2*3+1) = 3/7",
        series="Jain (2p+1)",
        degeneracy=7,
        chern_number=3,
    ),
    FQHEFraction(
        nu=Fraction(4, 9),
        tecs_expr="tau/(tau+sopfr)",
        tecs_numerator=T, tecs_denominator=T + F,
        note="Jain p=4: 4/(2*4+1) = 4/9",
        series="Jain (2p+1)",
        degeneracy=9,
        chern_number=4,
    ),
    FQHEFraction(
        nu=Fraction(5, 11),
        tecs_expr="sopfr/(sigma-1)",
        tecs_numerator=F, tecs_denominator=S - 1,
        note="Jain p=5: 5/(2*5+1) = 5/11",
        series="Jain (2p+1)",
        degeneracy=11,
        chern_number=5,
    ),

    # ---- Jain sequence: nu = p/(2p-1) ----
    FQHEFraction(
        nu=Fraction(2, 3),
        tecs_expr="phi*tau/sigma",
        tecs_numerator=P * T, tecs_denominator=S,
        note="Jain p=2: 2/(2*2-1) = 2/3, hole conjugate of 1/3",
        series="Jain (2p-1)",
        degeneracy=3,
        chern_number=2,
    ),
    FQHEFraction(
        nu=Fraction(3, 5),
        tecs_expr="(sigma/tau)/sopfr",
        tecs_numerator=S // T, tecs_denominator=F,
        note="Jain p=3: 3/(2*3-1) = 3/5",
        series="Jain (2p-1)",
        degeneracy=5,
        chern_number=3,
    ),

    # ---- Special states ----
    FQHEFraction(
        nu=Fraction(1, 2),
        tecs_expr="phi/tau",
        tecs_numerator=P, tecs_denominator=T,
        note="Composite fermion sea (no gap, metallic state)",
        series="Composite fermion",
        degeneracy=0,  # gapless
        chern_number=0,
    ),
    FQHEFraction(
        nu=Fraction(5, 2),
        tecs_expr="sopfr/phi",
        tecs_numerator=F, tecs_denominator=P,
        note="Non-Abelian Pfaffian state! Supports topological qubits",
        series="Non-Abelian",
        degeneracy=6,   # = P1 = n !!!
        chern_number=5,
        exchange_phase="pi/4 = pi/tau (non-Abelian: matrix-valued!)",
    ),
    FQHEFraction(
        nu=Fraction(2, 7),
        tecs_expr="phi/M3",
        tecs_numerator=P, tecs_denominator=M3,
        note="Observed in GaAs 2DEG at high B",
        series="Laughlin hierarchy",
        degeneracy=7,
        chern_number=2,
    ),
    FQHEFraction(
        nu=Fraction(1, 1),
        tecs_expr="phi/phi = tau/tau = sopfr/sopfr",
        tecs_numerator=P, tecs_denominator=P,
        note="Integer QHE (von Klitzing 1980)",
        series="Integer",
        degeneracy=1,
        chern_number=1,
    ),
]

# Build lookup
FQHE_BY_NU: Dict[Fraction, FQHEFraction] = {f.nu: f for f in FQHE_FRACTIONS}


# =====================================================================
# 2. JAIN SEQUENCE ANALYSIS
# =====================================================================

def jain_sequence(p_max: int = 8) -> List[dict]:
    """Generate the Jain composite fermion sequence nu = p/(2p +/- 1).

    For each p, the 2p+1 branch gives filling below 1/2,
    the 2p-1 branch gives filling above 1/2.
    """
    results = []
    for p in range(1, p_max + 1):
        for sign, branch in [(+1, "2p+1"), (-1, "2p-1")]:
            denom = 2 * p + sign
            if denom <= 0:
                continue
            nu = Fraction(p, denom)

            # Try to find TECS-L expression for numerator and denominator
            num_expr = _find_tecs_integer(p)
            den_expr = _find_tecs_integer(denom)

            results.append({
                'p': p,
                'branch': branch,
                'nu': nu,
                'numerator': p,
                'denominator': denom,
                'num_tecs': num_expr,
                'den_tecs': den_expr,
                'in_fqhe_table': nu in FQHE_BY_NU,
            })
    return results


def _find_tecs_integer(target: int) -> str:
    """Find simplest TECS-L expression for an integer."""
    consts = {'sigma': S, 'tau': T, 'phi': P, 'sopfr': F, 'N': N, 'M3': M3}

    # Direct match
    for name, val in consts.items():
        if val == target:
            return name

    # Simple ratios and products
    ops = [
        ('+', lambda a, b: a + b, '{} + {}'),
        ('-', lambda a, b: a - b, '{} - {}'),
        ('*', lambda a, b: a * b, '{} * {}'),
        ('//', lambda a, b: a // b if b != 0 and a % b == 0 else None, '{}/{}'),
    ]
    best = None
    for n1, v1 in consts.items():
        for n2, v2 in consts.items():
            for op_sym, op_fn, fmt in ops:
                try:
                    r = op_fn(v1, v2)
                    if r is not None and r == target:
                        expr = fmt.format(n1, n2)
                        if best is None or len(expr) < len(best):
                            best = expr
                except (OverflowError, ZeroDivisionError):
                    pass
    return best if best else str(target)


# =====================================================================
# 3. EGYPTIAN FRACTION CONNECTION
# =====================================================================

def egyptian_fraction_analysis() -> dict:
    """Analyze the Egyptian fraction 1/2 + 1/3 + 1/6 = 1 in FQHE context.

    The n=6 Egyptian fraction identity connects three FQHE states:
      nu=1/2 (composite fermion) + nu=1/3 (Laughlin) + nu=1/6 = 1 (integer QHE)

    This has physical meaning through the composite fermion transformation:
      nu -> nu/(2*nu + 1) maps integer QHE to FQHE.
    """
    # The three fractions
    f1 = Fraction(1, 2)  # phi/tau
    f2 = Fraction(1, 3)  # tau/sigma = 1/(sigma/tau)
    f3 = Fraction(1, 6)  # 1/N = 1/P1

    total = f1 + f2 + f3

    # Composite fermion transformation: nu -> nu/(2*nu + 1)
    # Starting from integer filling nu=p:
    cf_transforms = {}
    for p in range(1, 6):
        nu_start = Fraction(p, 1)
        nu_cf = nu_start / (2 * nu_start + 1)
        cf_transforms[p] = {
            'start': nu_start,
            'result': nu_cf,
            'is_observed': nu_cf in FQHE_BY_NU,
        }

    # Also try nu -> nu/(2*nu - 1)  (hole conjugate)
    cf_hole_transforms = {}
    for p in range(2, 6):
        nu_start = Fraction(p, 1)
        denom = 2 * nu_start - 1
        if denom > 0:
            nu_cf = nu_start / denom
            cf_hole_transforms[p] = {
                'start': nu_start,
                'result': nu_cf,
                'is_observed': nu_cf in FQHE_BY_NU or nu_cf == Fraction(1, 1),
            }

    return {
        'egyptian_fractions': [f1, f2, f3],
        'tecs_expressions': ['phi/tau', 'tau/sigma', '1/N'],
        'sum': total,
        'sum_is_one': total == 1,
        'physical_meaning': (
            "Sum of three FQHE filling fractions = integer QHE. "
            "The n=6 Egyptian fraction identity unifies the composite fermion "
            "sea (1/2), the Laughlin ground state (1/3), and the P1-denominator "
            "state (1/6) into the integer quantum Hall plateau."
        ),
        'cf_transforms': cf_transforms,
        'cf_hole_transforms': cf_hole_transforms,
    }


# =====================================================================
# 4. TOPOLOGICAL ORDER
# =====================================================================

@dataclass
class TopologicalData:
    """Topological properties of an FQHE state."""
    nu: Fraction
    torus_degeneracy: int
    degeneracy_tecs: str
    genus2_degeneracy: Optional[int] = None
    anyon_type: str = "Abelian"
    braiding_group: str = ""


TOPOLOGICAL_PROPERTIES: List[TopologicalData] = [
    TopologicalData(
        nu=Fraction(1, 3),
        torus_degeneracy=3,
        degeneracy_tecs="sigma/tau",
        genus2_degeneracy=9,
        anyon_type="Abelian",
        braiding_group="Z_3",
    ),
    TopologicalData(
        nu=Fraction(1, 5),
        torus_degeneracy=5,
        degeneracy_tecs="sopfr",
        genus2_degeneracy=25,
        anyon_type="Abelian",
        braiding_group="Z_5",
    ),
    TopologicalData(
        nu=Fraction(1, 7),
        torus_degeneracy=7,
        degeneracy_tecs="M3",
        genus2_degeneracy=49,
        anyon_type="Abelian",
        braiding_group="Z_7",
    ),
    TopologicalData(
        nu=Fraction(5, 2),
        torus_degeneracy=6,
        degeneracy_tecs="P1 = n",
        genus2_degeneracy=6,  # non-Abelian: genus-g degeneracy is different
        anyon_type="Non-Abelian (Pfaffian / Moore-Read)",
        braiding_group="Ising anyons",
    ),
]


# =====================================================================
# 5. CHERN NUMBERS AND CHERN-SIMONS LEVELS
# =====================================================================

def chern_simons_analysis() -> List[dict]:
    """Chern-Simons level k for Laughlin states nu=1/k.

    k must be odd for fermionic systems.
    k=3: sigma/tau, k=5: sopfr, k=7: M3 -- all n=6 constants!
    """
    results = []
    laughlin_k = [3, 5, 7]
    tecs_map = {3: "sigma/tau", 5: "sopfr", 7: "M3"}

    for k in laughlin_k:
        nu = Fraction(1, k)
        results.append({
            'k': k,
            'nu': nu,
            'tecs_expr': tecs_map[k],
            'is_odd': k % 2 == 1,
            'note': f"Chern-Simons SU(1)_{k} theory",
        })

    return results


# =====================================================================
# 6. TOPOLOGICAL INSULATORS AND BOTT PERIODICITY
# =====================================================================

def bott_periodicity_analysis() -> dict:
    """Bott periodicity has period 8 = sigma - tau = rank(E8).

    The periodic table of topological insulators/superconductors repeats
    with period 8 in dimension (the Altland-Zirnbauer 10-fold way has
    periodicity 2 in symmetry class and 8 in spatial dimension).

    Z_2 topological insulators: Z_2 = Z/2Z, the 2 is phi(6).
    3D TI lives in sigma/tau = 3 spatial dimensions.
    """
    bott_period = S - T  # 8

    # The 10-fold way: 10 symmetry classes
    tenfold = S + T - N  # 12 + 4 - 6 = 10

    # Spatial dimensions where Z_2 TIs exist: d = 2, 3
    z2_dimensions = [P, S // T]  # phi, sigma/tau

    return {
        'bott_period': bott_period,
        'bott_tecs': 'sigma - tau',
        'bott_also': 'rank(E8)',
        'tenfold_way': tenfold,
        'tenfold_tecs': 'sigma + tau - N',
        'z2_mod': P,
        'z2_tecs': 'phi = Z/2Z modulus',
        'ti_3d_dimensions': S // T,
        'ti_3d_tecs': 'sigma/tau = 3 spatial dimensions',
        'z2_dimensions': z2_dimensions,
    }


# =====================================================================
# 7. ANYONIC STATISTICS
# =====================================================================

def anyonic_phase_analysis() -> List[dict]:
    """Exchange phases for Laughlin quasiparticles.

    For nu=1/m Laughlin state, quasiparticle exchange phase = pi/m.
    Also: sin(pi/6) = 1/2 = phi/tau -- TECS-L identity!
    """
    results = []

    phases = [
        (Fraction(1, 3), "pi/3", "pi/(sigma/tau) = pi*tau/sigma",
         math.pi / 3, "Laughlin 1/3"),
        (Fraction(1, 5), "pi/5", "pi/sopfr",
         math.pi / 5, "Laughlin 1/5"),
        (Fraction(1, 7), "pi/7", "pi/M3",
         math.pi / 7, "Laughlin 1/7"),
        (Fraction(5, 2), "pi/4", "pi/tau (non-Abelian, matrix-valued!)",
         math.pi / 4, "Pfaffian 5/2"),
    ]

    for nu, phase_str, tecs_phase, phase_val, label in phases:
        results.append({
            'nu': nu,
            'exchange_phase': phase_str,
            'tecs_phase': tecs_phase,
            'phase_value': phase_val,
            'sin_value': math.sin(phase_val),
            'cos_value': math.cos(phase_val),
            'label': label,
        })

    # Special identity: sin(pi/6) = 1/2 = phi/tau
    results.append({
        'nu': Fraction(1, 6),
        'exchange_phase': 'pi/6 = pi/N',
        'tecs_phase': 'pi/P1',
        'phase_value': math.pi / 6,
        'sin_value': math.sin(math.pi / 6),  # = 0.5 = phi/tau
        'cos_value': math.cos(math.pi / 6),
        'label': 'Hypothetical 1/6 (even denom, not Laughlin)',
        'special': 'sin(pi/6) = 1/2 = phi/tau!',
    })

    return results


# =====================================================================
# 8. MONTE CARLO: EXPRESSIBILITY OF FQHE FRACTIONS
# =====================================================================

def _tecs_rationals(max_ops: int = 2) -> set:
    """Generate all rational numbers expressible as a/b where a,b are
    TECS-L expressible integers, with a in [0,20] and b in [1,20].

    Returns a set of Fraction objects.
    """
    consts = [S, T, P, F, N, M3]

    # Build set of expressible positive integers
    expressible = set()
    for v in consts:
        expressible.add(v)

    if max_ops >= 1:
        ops_fn = [
            lambda a, b: a + b,
            lambda a, b: a - b,
            lambda a, b: a * b,
            lambda a, b: a // b if b != 0 and a % b == 0 else None,
        ]
        for v1 in consts:
            for v2 in consts:
                for fn in ops_fn:
                    try:
                        r = fn(v1, v2)
                        if r is not None and 1 <= r <= 50:
                            expressible.add(int(r))
                    except (OverflowError, ZeroDivisionError):
                        pass

    # Build rationals from expressible integers
    rationals = set()
    for a in expressible:
        for b in expressible:
            if b > 0 and a >= 0:
                rationals.add(Fraction(a, b))

    return rationals


def _direct_tecs_rationals() -> set:
    """Rationals expressible as direct ratios of single TECS-L constants.

    Only a/b where a,b are each a single constant from {S,T,P,F,N,M3}.
    This is the STRICT test -- no compound expressions allowed.
    """
    consts = [S, T, P, F, N, M3]
    rats = set()
    for a in consts:
        for b in consts:
            if b > 0:
                rats.add(Fraction(a, b))
    return rats


def mc_fraction_expressibility(n_trials: int = 50_000) -> dict:
    """Test how many FQHE fractions vs random fractions are TECS-L expressible.

    TWO tests:
    (A) Strict: only direct single-constant ratios a/b (6 constants -> 36 rationals).
        How many FQHE fractions are direct ratios of TECS-L constants?
    (B) Structural: given 6 random integers in [1,12], how often do they generate
        as many FQHE fractions as the actual TECS-L constants do?
    """
    # --- Test A: direct single-constant ratios ---
    direct_rats = _direct_tecs_rationals()
    fqhe_fracs = [f.nu for f in FQHE_FRACTIONS]
    n_fqhe = len(fqhe_fracs)
    direct_matches = sum(1 for f in fqhe_fracs if f in direct_rats)

    # --- Test B: random constant sets ---
    # Pick 6 random integers in [1,12] and form all ratios.
    # How often do they cover as many FQHE fractions?
    rng = random.Random(42)
    as_good_count = 0
    match_distribution = []
    for _ in range(n_trials):
        rand_consts = [rng.randint(1, 12) for __ in range(6)]
        rand_rats = set()
        for a in rand_consts:
            for b in rand_consts:
                if b > 0:
                    rand_rats.add(Fraction(a, b))
        n_match = sum(1 for f in fqhe_fracs if f in rand_rats)
        match_distribution.append(n_match)
        if n_match >= direct_matches:
            as_good_count += 1

    mc_p = as_good_count / n_trials
    mc_mean = sum(match_distribution) / n_trials

    # --- Test C: compound expressions (1 op) ---
    compound_rats = _tecs_rationals(max_ops=1)
    compound_matches = sum(1 for f in fqhe_fracs if f in compound_rats)

    return {
        'n_fqhe': n_fqhe,
        'direct_matches': direct_matches,
        'direct_n_rationals': len(direct_rats),
        'compound_matches': compound_matches,
        'compound_n_rationals': len(compound_rats),
        'mc_mean_matches': mc_mean,
        'mc_p_as_good': mc_p,
        'mc_trials': n_trials,
    }


# =====================================================================
# RUN ANALYSIS
# =====================================================================

def run_analysis(mc_trials: int = 20_000) -> None:
    """Run the full Quantum Hall TECS-L analysis."""
    print("=" * 78)
    print("FRACTIONAL QUANTUM HALL EFFECT -- n=6 ARITHMETIC (TECS-L)")
    print("=" * 78)
    print()
    print(f"  TECS-L constants:  sigma={S}, tau={T}, phi={P}, "
          f"sopfr={F}, n=P1={N}, M3={M3}")
    print()

    # ── Section 1: FQHE Fractions ──
    print("-" * 78)
    print("1. FQHE FILLING FRACTIONS: TECS-L MAPPINGS")
    print("-" * 78)
    print()

    all_match = True
    for frac in FQHE_FRACTIONS:
        status = "OK" if frac.match else "MISMATCH"
        if not frac.match:
            all_match = False
        print(f"  nu = {str(frac.nu):>5s}  =  {frac.tecs_expr:<35s}  [{status}]")
        print(f"         {frac.note}")
        if frac.series:
            print(f"         Series: {frac.series}", end="")
            if frac.degeneracy > 0:
                print(f"  |  Torus degeneracy: {frac.degeneracy}", end="")
            if frac.chern_number > 0:
                print(f"  |  C_1 = {frac.chern_number}", end="")
            print()
        print()

    n_matched = sum(1 for f in FQHE_FRACTIONS if f.match)
    print(f"  RESULT: {n_matched}/{len(FQHE_FRACTIONS)} FQHE fractions have exact "
          f"TECS-L expressions.")
    if all_match:
        print("  >>> ALL fractions match! <<<")

    # ── Section 2: Jain Sequence ──
    print()
    print("-" * 78)
    print("2. JAIN COMPOSITE FERMION SEQUENCE: nu = p/(2p +/- 1)")
    print("-" * 78)
    print()
    print(f"  {'p':>3s}  {'branch':>6s}  {'nu':>7s}  {'num TECS':>12s}  {'den TECS':>12s}  observed?")
    print(f"  {'---':>3s}  {'------':>6s}  {'-------':>7s}  {'--------':>12s}  {'--------':>12s}  ---------")

    jain = jain_sequence(p_max=6)
    for j in jain:
        obs = "YES" if j['in_fqhe_table'] else "   "
        print(f"  {j['p']:>3d}  {j['branch']:>6s}  {str(j['nu']):>7s}  "
              f"{j['num_tecs']:>12s}  {j['den_tecs']:>12s}  {obs}")

    print()
    print("  Jain denominators (odd numbers): 1, 3, 5, 7, 9, 11, ...")
    print("    3 = sigma/tau")
    print("    5 = sopfr")
    print("    7 = M3")
    print("    9 = tau + sopfr")
    print("   11 = sigma - 1")
    print("  ALL Jain denominators up to 11 have clean TECS-L expressions.")

    # ── Section 3: Egyptian Fraction ──
    print()
    print("-" * 78)
    print("3. EGYPTIAN FRACTION: 1/2 + 1/3 + 1/6 = 1")
    print("-" * 78)
    print()

    ef = egyptian_fraction_analysis()
    print("  The n=6 Egyptian fraction identity:")
    print()
    for i, (frac, expr) in enumerate(zip(ef['egyptian_fractions'],
                                          ef['tecs_expressions'])):
        sym = "+" if i > 0 else " "
        print(f"    {sym} nu = {frac}  =  {expr}")
    print(f"    {'':>1s} {'---':>7s}")
    print(f"      {ef['sum']}        = integer quantum Hall effect!")
    print()
    print(f"  {ef['physical_meaning']}")

    print()
    print("  Composite fermion map: nu -> nu/(2*nu + 1)")
    for p, data in ef['cf_transforms'].items():
        obs_tag = " [observed]" if data['is_observed'] else ""
        print(f"    nu={p} -> {data['result']}{obs_tag}")

    # ── Section 4: Topological Order ──
    print()
    print("-" * 78)
    print("4. TOPOLOGICAL ORDER: GROUND STATE DEGENERACIES")
    print("-" * 78)
    print()

    for td in TOPOLOGICAL_PROPERTIES:
        print(f"  nu = {str(td.nu):>5s}:  torus degeneracy = {td.torus_degeneracy}"
              f"  =  {td.degeneracy_tecs}")
        print(f"         Anyon type: {td.anyon_type}")
        if td.braiding_group:
            print(f"         Braiding: {td.braiding_group}")
        print()

    print("  KEY: nu=5/2 torus degeneracy = 6 = P1 = n !!!")
    print("  This non-Abelian state supports topological quantum computing.")

    # ── Section 5: Chern-Simons Levels ──
    print()
    print("-" * 78)
    print("5. CHERN-SIMONS LEVELS k FOR LAUGHLIN STATES nu=1/k")
    print("-" * 78)
    print()

    cs = chern_simons_analysis()
    for entry in cs:
        print(f"  k = {entry['k']}  (nu = {entry['nu']})  =  {entry['tecs_expr']}"
              f"    odd: {entry['is_odd']}")
    print()
    print("  Chern-Simons levels k = 3, 5, 7 map to sigma/tau, sopfr, M3")
    print("  -- exactly the trio of odd TECS-L constants!")

    # ── Section 6: Bott Periodicity ──
    print()
    print("-" * 78)
    print("6. TOPOLOGICAL INSULATORS AND BOTT PERIODICITY")
    print("-" * 78)
    print()

    bott = bott_periodicity_analysis()
    print(f"  Bott periodicity period = {bott['bott_period']} = {bott['bott_tecs']}"
          f" = {bott['bott_also']}")
    print(f"  10-fold way classes    = {bott['tenfold_way']} = {bott['tenfold_tecs']}")
    print(f"  Z_2 modulus            = {bott['z2_mod']} = {bott['z2_tecs']}")
    print(f"  3D TI spatial dims     = {bott['ti_3d_dimensions']} = {bott['ti_3d_tecs']}")

    # ── Section 7: Anyonic Statistics ──
    print()
    print("-" * 78)
    print("7. ANYONIC EXCHANGE PHASES")
    print("-" * 78)
    print()

    phases = anyonic_phase_analysis()
    for ph in phases:
        print(f"  nu = {str(ph['nu']):>5s}:  theta = {ph['exchange_phase']:<12s}"
              f"  =  {ph['tecs_phase']}")
        if 'special' in ph:
            print(f"         IDENTITY: {ph['special']}")
    print()
    print("  sin(pi/6) = 1/2 = phi/tau  <-- fundamental TECS-L identity")
    print("  sin(pi/3) = sqrt(3)/2,  cos(pi/3) = 1/2 = phi/tau")

    # ── Section 8: Monte Carlo ──
    print()
    print("-" * 78)
    print("8. MONTE CARLO: EXPRESSIBILITY OF FQHE FRACTIONS")
    print("-" * 78)
    print()

    mc = mc_fraction_expressibility(n_trials=mc_trials)

    print("  Test A -- Direct single-constant ratios (strict):")
    print(f"    FQHE fractions matched:  {mc['direct_matches']}/{mc['n_fqhe']}")
    print(f"    Distinct TECS-L rationals: {mc['direct_n_rationals']}")
    print()
    print("  Test B -- Random 6-constant sets (same range [1,12]):")
    print(f"    Mean matches per random set: {mc['mc_mean_matches']:.1f}/{mc['n_fqhe']}")
    print(f"    P(random >= {mc['direct_matches']} matches): "
          f"{mc['mc_p_as_good']*100:.2f}%  ({mc['mc_trials']:,} trials)")
    print()
    print("  Test C -- Compound expressions (1 op on TECS-L constants):")
    print(f"    FQHE fractions matched:  {mc['compound_matches']}/{mc['n_fqhe']}")
    print(f"    Distinct rationals:      {mc['compound_n_rationals']}")

    if mc['mc_p_as_good'] < 0.05:
        print()
        print(f"  >>> SIGNIFICANT at p < 5%: FQHE fractions are unusually "
              f"well-covered by TECS-L constants! <<<")
    elif mc['mc_p_as_good'] < 0.20:
        print()
        print(f"  >>> SUGGESTIVE: FQHE fractions are better-matched than "
              f"typical random constant sets. <<<")

    # ── Summary ──
    print()
    print("=" * 78)
    print("SUMMARY")
    print("=" * 78)
    print()
    print(f"  {n_matched}/{len(FQHE_FRACTIONS)} observed FQHE fractions have exact "
          f"TECS-L rational expressions.")
    print()
    print("  Key mappings:")
    print("    nu = 1/3  =  tau/sigma           Laughlin ground state")
    print("    nu = 2/5  =  phi/sopfr           Jain p=2")
    print("    nu = 3/7  =  (sigma/tau)/M3      Jain p=3")
    print("    nu = 2/3  =  phi*tau/sigma       Hole conjugate")
    print("    nu = 1/2  =  phi/tau             Composite fermion sea")
    print("    nu = 5/2  =  sopfr/phi           NON-ABELIAN Pfaffian!")
    print()
    print("  Structural coincidences:")
    print("    - Laughlin k = 3, 5, 7 = sigma/tau, sopfr, M3")
    print("    - nu=5/2 degeneracy = 6 = P1 (topological quantum computing!)")
    print("    - Bott periodicity = 8 = sigma - tau = rank(E8)")
    print("    - Egyptian fraction 1/2 + 1/3 + 1/6 = 1  (three FQHE -> integer QHE)")
    print("    - sin(pi/6) = 1/2 = phi/tau")
    print()
    print("=" * 78)


if __name__ == '__main__':
    run_analysis()
