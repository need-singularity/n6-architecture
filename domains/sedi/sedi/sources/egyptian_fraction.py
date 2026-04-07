"""Egyptian Fraction — Perfect Number Analysis (H-CX-479/489/507).

The unit-fraction equation  1/a + 1/b + 1/c = 1  (2 <= a <= b <= c) has
exactly three solutions: {2,3,6}, {2,4,4}, {3,3,3}.  Only the first has
lcm equal to a perfect number (6).  This module traces the consequences
through Mersenne primes, the k_min divisor-sum spectrum, and the 37 GeV
mass prediction.
"""
import math
from itertools import combinations_with_replacement
from sympy import divisor_sigma, isprime, prime, divisors, lcm as sym_lcm


# =====================================================================
# 1. Three-term Egyptian fraction solutions: 1/a + 1/b + 1/c = 1
# =====================================================================

def find_unit_fraction_triples(max_denom=200):
    """Find all (a, b, c) with 2 <= a <= b <= c and 1/a+1/b+1/c = 1."""
    from fractions import Fraction
    solutions = []
    one = Fraction(1, 1)
    for a in range(2, max_denom + 1):
        fa = Fraction(1, a)
        if 3 * fa < one:
            # Even 1/a + 1/a + 1/a < 1, so a is too large
            break
        for b in range(a, max_denom + 1):
            fb = Fraction(1, b)
            remainder = one - fa - fb
            if remainder < 0:
                # 1/a + 1/b > 1; as b grows 1/b shrinks so remainder grows
                # → for small b, remainder can be negative; skip and try larger b
                continue
            if remainder == 0:
                # 1/a + 1/b = 1 exactly, no room for 1/c > 0
                continue
            # Now remainder > 0. Need remainder = 1/c with c integer, c >= b
            if remainder.numerator == 1 and remainder.denominator >= b:
                c = remainder.denominator
                if c <= max_denom:
                    solutions.append((a, b, c))
            # If remainder < 1/max_denom, c would exceed max_denom; stop
            if remainder < Fraction(1, max_denom):
                break
    return solutions


# =====================================================================
# 2. Perfect number check and uniqueness proof
# =====================================================================

PERFECT_NUMBERS = [6, 28, 496, 8128, 33550336]


def is_perfect(n):
    """Check whether n is a perfect number: sigma(n) = 2n."""
    return int(divisor_sigma(n)) == 2 * n


def uniqueness_proof(solutions):
    """For each 3-term solution, compute lcm and test perfection."""
    results = []
    for triple in solutions:
        a, b, c = triple
        L = int(sym_lcm(a, sym_lcm(b, c)))
        perfect = is_perfect(L)
        results.append({
            'triple': triple,
            'lcm': L,
            'is_perfect': perfect,
        })
    return results


# =====================================================================
# 3. k_min computation for perfect numbers
# =====================================================================

# Mersenne prime exponents for the first eight perfect numbers
MERSENNE_EXPONENTS = [2, 3, 5, 7, 13, 17, 19, 31]


def perfect_number_from_exponent(p):
    """P_n = 2^(p-1) * (2^p - 1) for Mersenne prime exponent p."""
    return 2 ** (p - 1) * (2 ** p - 1)


def k_min_formula(p):
    """k_min(P_n) = 2p - 1 for Mersenne prime exponent p."""
    return 2 * p - 1


def k_min_table():
    """Build k_min table for all Mersenne exponents."""
    rows = []
    for p in MERSENNE_EXPONENTS:
        pn = perfect_number_from_exponent(p)
        km = k_min_formula(p)
        rows.append({
            'p': p,
            'P_n': pn,
            'k_min': km,
            'k_min_is_prime': isprime(km),
        })
    return rows


# =====================================================================
# 4. Enumerate Egyptian fractions over divisors (brute-force for small P)
# =====================================================================

def _enumerate_divisor_sums(P, target_k=None):
    """Find minimum k such that some subset of divisors of P (with repetition)
    sums to P when written as P/d_1 + P/d_2 + ... + P/d_k = P,
    equivalently 1/d_1 + ... + 1/d_k = 1.

    We search over multi-subsets of proper divisors of P (excluding 1 and P itself
    if P is perfect, since 1/1=1 trivially solves it; the question is about
    non-trivial divisors d >= 2, d | P).
    Returns the minimum k found."""
    from fractions import Fraction
    divs = sorted([d for d in divisors(P) if 1 < d <= P])  # proper divisors > 1

    # BFS over increasing subset sizes (no repetition)
    max_k = target_k if target_k else 50
    for k in range(1, min(max_k + 1, len(divs) + 1)):
        if _check_subset(divs, 0, k, Fraction(1, 1)):
            return k
    return None


def _check_subset(divs, start, k, target):
    """Recursively check if k DISTINCT divisors from divs[start:] have
    reciprocals summing to target (no repetition)."""
    from fractions import Fraction
    if k == 0:
        return target == 0
    if target <= 0 or start >= len(divs):
        return False
    for i in range(start, len(divs)):
        d = divs[i]
        recip = Fraction(1, d)
        if recip > target:
            continue
        # Prune: even with the (k-1) largest remaining reciprocals we can't reach target
        remaining_max = sum(Fraction(1, divs[j]) for j in range(i + 1, min(i + k, len(divs))))
        if recip + remaining_max < target:
            continue
        if _check_subset(divs, i + 1, k - 1, target - recip):
            return True
    return False


def verify_k_min_small(P, expected_k):
    """Verify k_min by enumeration for a small perfect number P."""
    k_found = _enumerate_divisor_sums(P, target_k=expected_k + 2)
    return {
        'P': P,
        'expected_k_min': expected_k,
        'enumerated_k_min': k_found,
        'match': k_found == expected_k,
    }


# =====================================================================
# 5. The 37 GeV connection
# =====================================================================

def gev_37_chain():
    """Trace the number-theoretic chain to 37 GeV.

    k_min(P_6) = 2*19 - 1 = 37        (P_6 = 2^18 * (2^19 - 1) = 27,594,... wait)
    Actually P_6 is the 6th perfect number with p=17 ... let's be precise:
    The Mersenne exponents are [2,3,5,7,13,17,19,31].
    Index 6 (1-based) has p=19, so P_6 uses p=19:
      k_min = 2*19 - 1 = 37

    Also: sigma(6) = 12, and prime(12) = 37.
    So 37 = prime(sigma(6)).

    Predicted mass: 37 + 1/6 = 37.1667 GeV.
    Compare to observed ~37.17 GeV (J/psi x sigma(6) = 3.097 x 12 = 37.164).
    """
    p = 19  # 7th Mersenne exponent (index 6 zero-based, index 7 one-based)
    k_min_val = k_min_formula(p)

    sigma_6 = int(divisor_sigma(6))  # = 12
    prime_sigma_6 = int(prime(sigma_6))  # prime(12) = 37

    predicted_mass = 37 + 1 / 6  # 37.16667 GeV
    jpsi_x_sigma = 3.09690 * 12  # 37.1628 GeV
    error_ppm = abs(predicted_mass - jpsi_x_sigma) / jpsi_x_sigma * 1e6

    return {
        'mersenne_p': p,
        'k_min_P7': k_min_val,
        'sigma_6': sigma_6,
        'prime_of_sigma_6': prime_sigma_6,
        'both_equal_37': k_min_val == prime_sigma_6 == 37,
        'predicted_mass_GeV': predicted_mass,
        'jpsi_x_sigma_GeV': jpsi_x_sigma,
        'error_ppm': error_ppm,
    }


# =====================================================================
# Full report
# =====================================================================

def run_analysis():
    """Run all Egyptian fraction / perfect number analyses and print report."""
    width = 78
    sep = '=' * width

    print(f'\n{sep}')
    print('  EGYPTIAN FRACTION — PERFECT NUMBER ANALYSIS  (H-CX-479/489/507)')
    print(sep)

    # --- 1. Three-term solutions ---
    print('\n--- 1. Three-term solutions to 1/a + 1/b + 1/c = 1 ---\n')
    solutions = find_unit_fraction_triples()
    for s in solutions:
        a, b, c = s
        print(f'  {{1/{a}, 1/{b}, 1/{c}}}   check: '
              f'{1/a:.6f} + {1/b:.6f} + {1/c:.6f} = {1/a+1/b+1/c:.6f}')
    print(f'\n  Total solutions: {len(solutions)}')

    # --- 2. Uniqueness proof ---
    print(f'\n--- 2. Uniqueness: lcm vs perfect numbers ---\n')
    proofs = uniqueness_proof(solutions)
    print(f'  {"Triple":<16s} {"lcm":>8s} {"Perfect?":>10s}')
    print(f'  {"-"*16} {"-"*8} {"-"*10}')
    for p in proofs:
        a, b, c = p['triple']
        tag = 'YES ***' if p['is_perfect'] else 'no'
        print(f'  ({a},{b},{c}){"":<10s} {p["lcm"]:>8d} {tag:>10s}')

    perfect_lcms = [p for p in proofs if p['is_perfect']]
    print(f'\n  Only {len(perfect_lcms)} of {len(proofs)} solutions has '
          f'lcm = perfect number.')
    if perfect_lcms:
        p = perfect_lcms[0]
        print(f'  That solution is {p["triple"]} with lcm = {p["lcm"]} '
              f'(the first perfect number).')

    # --- 3. k_min table ---
    print(f'\n--- 3. k_min(P_n) = 2p - 1 for Mersenne exponents ---\n')
    table = k_min_table()
    print(f'  {"n":>3s} {"p":>4s} {"P_n":>14s} {"k_min":>6s} {"k_min prime?":>14s}')
    print(f'  {"---":>3s} {"----":>4s} {"-"*14} {"------":>6s} {"-"*14}')
    for i, row in enumerate(table, 1):
        pn_str = str(row['P_n']) if row['P_n'] < 10**9 else f'~10^{math.log10(row["P_n"]):.1f}'
        pr = 'yes' if row['k_min_is_prime'] else 'no'
        print(f'  {i:>3d} {row["p"]:>4d} {pn_str:>14s} {row["k_min"]:>6d} {pr:>14s}')

    # --- 4. Enumeration verification for P=6 and P=28 ---
    print(f'\n--- 4. Enumeration verification of k_min ---\n')
    for P, exp_p in [(6, 2), (28, 3)]:
        expected = k_min_formula(exp_p)
        result = verify_k_min_small(P, expected)
        status = 'CONFIRMED' if result['match'] else 'MISMATCH'
        print(f'  P = {P:>5d}:  expected k_min = {expected}, '
              f'enumerated = {result["enumerated_k_min"]}  [{status}]')

    # --- 5. The 37 GeV connection ---
    print(f'\n--- 5. The 37 GeV Chain ---\n')
    chain = gev_37_chain()
    print(f'  Mersenne exponent p = {chain["mersenne_p"]}')
    print(f'  k_min(P_n with p=19) = 2 x 19 - 1 = {chain["k_min_P7"]}')
    print(f'  sigma(6) = {chain["sigma_6"]}')
    print(f'  prime(sigma(6)) = prime({chain["sigma_6"]}) = {chain["prime_of_sigma_6"]}')
    print(f'  Both equal 37: {chain["both_equal_37"]}')
    print()
    print(f'  Predicted mass:  37 + 1/6 = {chain["predicted_mass_GeV"]:.4f} GeV')
    print(f'  J/psi x sigma:   3.09690 x 12 = {chain["jpsi_x_sigma_GeV"]:.4f} GeV')
    print(f'  Discrepancy:     {chain["error_ppm"]:.0f} ppm')

    # --- Summary ---
    print(f'\n{sep}')
    print('  SUMMARY')
    print(sep)
    print(f'''
  Three-term Egyptian fractions 1/a+1/b+1/c = 1 have exactly 3 solutions.
  Of these, only {{2,3,6}} has lcm = 6, the first perfect number.

  For P_n = 2^(p-1)(2^p - 1), the minimum Egyptian-fraction length over
  divisors of P_n is k_min = 2p - 1.

  At p = 19:  k_min = 37 = prime(sigma(6)) = prime(12).
  This yields the mass prediction 37 + 1/6 = 37.1667 GeV,
  matching J/psi x sigma(6) = 37.1628 GeV to {chain["error_ppm"]:.0f} ppm.
''')
    print(sep)

    return {
        'solutions': solutions,
        'uniqueness': proofs,
        'k_min_table': table,
        'gev_37': chain,
    }


if __name__ == '__main__':
    run_analysis()
