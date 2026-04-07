"""Three-Coupling Unification & TECS-L Crossing Analysis.

Runs ALL THREE Standard Model gauge couplings (alpha_1, alpha_2, alpha_3)
from 1 GeV to 10^18 GeV using 1-loop RGE, then systematically checks where
each coupling equals a TECS-L special fraction and whether any two couplings
hit TECS-L values at the SAME energy scale.

Also tracks the Weinberg angle sin^2(theta_W) running and identifies scales
where it equals 1/4, 1/3, and 3/8 (SU(5) prediction).

Physics:
  1-loop RGE:  alpha_i^{-1}(mu) = alpha_i^{-1}(M_Z) + (b_i / 2pi) ln(mu/M_Z)
  SM beta coefficients: b1 = 41/10, b2 = -19/6, b3 = -7
  M_Z boundary: alpha_1^{-1} ~ 59.0, alpha_2^{-1} ~ 29.6, alpha_3^{-1} ~ 8.48

TECS-L connection:
  The n=6 arithmetic (sigma=12, tau=4, phi=2) produces fractions 1/2, 1/3,
  1/4, 1/6, 1/12, 2/9, 5/6, ln(4/3), etc. If gauge couplings equal these
  at physically meaningful scales, TECS encodes the RG structure.
"""

import math
import numpy as np
from ..tecs import (
    SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1,
    P1, P2, P3, TAU_P2, TAU_P3,
    ALL_TARGETS,
)


# ═══════════════════════════════════════════════════════════════════
# Physical Constants
# ═══════════════════════════════════════════════════════════════════

M_Z = 91.1876              # Z boson mass (GeV)
ALPHA_S_MZ = 0.1179        # alpha_s(M_Z), PDG 2024
ALPHA_EM_MZ = 1.0 / 127.95 # alpha_EM at M_Z
SIN2_THETA_W = 0.23122     # sin^2(theta_W) at M_Z

S = SIGMA_P1   # 12
T = TAU_P1     # 4
P = PHI_P1     # 2
F = SOPFR_P1   # 5
N = P1         # 6

# Known particle/energy scales (GeV)
ENERGY_SCALES = {
    'pion':        0.135,
    'kaon':        0.494,
    'proton':      0.938,
    'charm':       1.27,
    'tau lepton':  1.777,
    'J/psi':       3.097,
    'bottom':      4.18,
    'Upsilon':     9.460,
    'W boson':     80.379,
    'Z boson':     91.188,
    'Higgs':       125.25,
    'top':         172.76,
    'LHC 14 TeV':  14000.0,
    'Planck':      1.22e19,
}


# ═══════════════════════════════════════════════════════════════════
# TECS-L Target Fractions for Coupling Values
# ═══════════════════════════════════════════════════════════════════

TECSL_ALPHA_TARGETS = {
    # Fractions for alpha itself
    '1/2 = phi/tau':              1.0 / 2,
    '1/3 = tau/sigma':            1.0 / 3,
    '1/4 = 1/tau':                1.0 / 4,
    '1/5 = 1/sopfr':              1.0 / 5,
    '1/6 = 1/n':                  1.0 / 6,
    '1/8 = 1/(sigma-tau)':        1.0 / 8,
    '1/12 = 1/sigma':             1.0 / 12,
    '1/24 = 1/(sigma*phi)':       1.0 / 24,
    '2/9 = Koide delta':          2.0 / 9,
    '5/6 = sopfr/n':              5.0 / 6,
    'ln(4/3) Golden Zone':        math.log(4.0 / 3.0),
    '1/144 = 1/sigma^2':          1.0 / 144,
}

# Integer targets for alpha^{-1}
TECSL_INV_TARGETS = {
    '4 = tau':          4,
    '6 = n':            6,
    '8 = sigma-tau':    8,
    '12 = sigma':       12,
    '16 = sigma+tau':   16,
    '24 = sigma*phi':   24,
    '28 = P2':          28,
    '42 = sigma*tau-n': 42,
    '144 = sigma^2':    144,
}


# ═══════════════════════════════════════════════════════════════════
# 1-Loop Gauge Coupling Running
# ═══════════════════════════════════════════════════════════════════

# SM 1-loop beta coefficients
B1 = 41.0 / 10.0    # U(1)_Y
B2 = -19.0 / 6.0    # SU(2)_L
B3 = -7.0            # SU(3)_c

# Boundary conditions at M_Z
# GUT-normalized: alpha_1 = (5/3) alpha_Y = (5/3) alpha_EM / cos^2(theta_W)
ALPHA_2_MZ = ALPHA_EM_MZ / SIN2_THETA_W
ALPHA_1_MZ = ALPHA_EM_MZ / (1.0 - SIN2_THETA_W) * (5.0 / 3.0)
ALPHA_3_MZ = ALPHA_S_MZ

INV_A1_MZ = 1.0 / ALPHA_1_MZ   # ~59.0
INV_A2_MZ = 1.0 / ALPHA_2_MZ   # ~29.6
INV_A3_MZ = 1.0 / ALPHA_3_MZ   # ~8.48


def inv_alpha_1(mu):
    """1/alpha_1 at scale mu (GeV), 1-loop.

    Convention: 1/alpha_i(mu) = 1/alpha_i(M_Z) - b_i/(2pi) * ln(mu/M_Z)
    With b1 = +41/10 > 0, the minus sign means 1/alpha_1 DECREASES with energy,
    so alpha_1 increases (U(1) is not asymptotically free). Correct.
    """
    return INV_A1_MZ - B1 / (2 * math.pi) * math.log(mu / M_Z)


def inv_alpha_2(mu):
    """1/alpha_2 at scale mu (GeV), 1-loop.

    With b2 = -19/6 < 0, minus sign means -(-19/6)*positive = positive,
    so 1/alpha_2 INCREASES with energy slightly, then... actually b2 < 0 means
    SU(2) IS asymptotically free: 1/alpha_2 increases, alpha_2 decreases. Correct.
    """
    return INV_A2_MZ - B2 / (2 * math.pi) * math.log(mu / M_Z)


def inv_alpha_3(mu):
    """1/alpha_3 at scale mu (GeV), 1-loop.

    With b3 = -7 < 0: -(-7)*positive = positive, so 1/alpha_3 INCREASES
    with energy. alpha_3 decreases. Asymptotic freedom. Correct.
    """
    return INV_A3_MZ - B3 / (2 * math.pi) * math.log(mu / M_Z)


def sin2_weinberg(mu):
    """Running sin^2(theta_W)(mu) = alpha_1(mu) / (alpha_1(mu) + alpha_2(mu)).

    Using GUT normalization: sin^2(theta_W) = (3/5) * alpha'_1 / (alpha'_1 + alpha_2)
    where alpha'_1 = (5/3)*alpha_1_Y. So:
    sin^2 = (3/5) / (1 + alpha_2/alpha'_1) = (3/5) * inv_a1 / ...
    Actually: sin^2 = alpha_EM / alpha_2 = 1/(1 + (5/3)*alpha_2/alpha_1_Y)
    With GUT normalization alpha_1 = (5/3) alpha_Y:
      sin^2(theta_W) = (3/8) * [1 - (alpha_1-alpha_2)/(alpha_1+alpha_2) * ...]

    Simpler: sin^2 = g'^2/(g^2 + g'^2) where g' = sqrt(5/3)*g_1.
    In terms of alphas: sin^2 = (3/5)*alpha_1 / ((3/5)*alpha_1 + alpha_2)
    """
    ia1 = inv_alpha_1(mu)
    ia2 = inv_alpha_2(mu)
    if ia1 <= 0 or ia2 <= 0:
        return float('nan')
    a1 = 1.0 / ia1
    a2 = 1.0 / ia2
    # sin^2(theta_W) = (3/5)*alpha_1 / ((3/5)*alpha_1 + alpha_2)
    a1_prime = (3.0 / 5.0) * a1
    return a1_prime / (a1_prime + a2)


# ═══════════════════════════════════════════════════════════════════
# TECS-L Crossing Finder
# ═══════════════════════════════════════════════════════════════════

def find_crossing(inv_alpha_func, target_inv_alpha, mu_low=1.0, mu_high=1e18):
    """Find mu where 1/alpha_i(mu) = target using bisection.

    Since 1/alpha_i is monotonic in ln(mu), bisection works perfectly.
    """
    val_low = inv_alpha_func(mu_low)
    val_high = inv_alpha_func(mu_high)

    if (val_low - target_inv_alpha) * (val_high - target_inv_alpha) > 0:
        return None  # target not in range

    for _ in range(200):
        mu_mid = math.exp(0.5 * (math.log(mu_low) + math.log(mu_high)))
        val_mid = inv_alpha_func(mu_mid)
        if abs(val_mid - target_inv_alpha) < 1e-10:
            return mu_mid
        if (val_low - target_inv_alpha) * (val_mid - target_inv_alpha) < 0:
            mu_high = mu_mid
            val_high = val_mid
        else:
            mu_low = mu_mid
            val_low = val_mid
    return math.exp(0.5 * (math.log(mu_low) + math.log(mu_high)))


def find_sin2_crossing(target, mu_low=1.0, mu_high=1e18):
    """Find mu where sin^2(theta_W)(mu) = target."""
    val_low = sin2_weinberg(mu_low)
    val_high = sin2_weinberg(mu_high)

    if math.isnan(val_low) or math.isnan(val_high):
        return None
    if (val_low - target) * (val_high - target) > 0:
        return None

    for _ in range(200):
        mu_mid = math.exp(0.5 * (math.log(mu_low) + math.log(mu_high)))
        val_mid = sin2_weinberg(mu_mid)
        if math.isnan(val_mid):
            return None
        if abs(val_mid - target) < 1e-12:
            return mu_mid
        if (val_low - target) * (val_mid - target) < 0:
            mu_high = mu_mid
        else:
            mu_low = mu_mid
            val_low = val_mid
    return math.exp(0.5 * (math.log(mu_low) + math.log(mu_high)))


def check_energy_proximity(mu_gev, threshold=0.15):
    """Check if mu is within threshold of a known energy scale."""
    matches = []
    for name, mass in ENERGY_SCALES.items():
        if mass <= 0:
            continue
        ratio = mu_gev / mass
        if abs(ratio - 1.0) < threshold:
            pct = (ratio - 1.0) * 100
            matches.append((name, mass, pct))
    return matches


# ═══════════════════════════════════════════════════════════════════
# ASCII Plot of All Three Couplings
# ═══════════════════════════════════════════════════════════════════

def ascii_unification_plot(log_mus, inv_a1s, inv_a2s, inv_a3s, crossings=None):
    """ASCII plot of 1/alpha_1, 1/alpha_2, 1/alpha_3 vs log10(mu)."""
    width = 78
    height = 35

    x_min, x_max = log_mus[0], log_mus[-1]
    all_vals = np.concatenate([inv_a1s, inv_a2s, inv_a3s])
    y_min = max(0, np.nanmin(all_vals) - 2)
    y_max = np.nanmax(all_vals) + 2

    grid = [[' '] * width for _ in range(height)]

    def plot_series(vals, char):
        for i in range(len(log_mus)):
            v = vals[i]
            if np.isnan(v):
                continue
            xi = int((log_mus[i] - x_min) / (x_max - x_min) * (width - 1))
            yi = int((v - y_min) / (y_max - y_min) * (height - 1))
            yi = height - 1 - yi
            if 0 <= xi < width and 0 <= yi < height:
                grid[yi][xi] = char

    plot_series(inv_a1s, '1')
    plot_series(inv_a2s, '2')
    plot_series(inv_a3s, '3')

    # Mark crossings
    if crossings:
        for label, mu_val, inv_val in crossings:
            if mu_val is None:
                continue
            lm = math.log10(mu_val)
            if x_min <= lm <= x_max and y_min <= inv_val <= y_max:
                xi = int((lm - x_min) / (x_max - x_min) * (width - 1))
                yi = int((inv_val - y_min) / (y_max - y_min) * (height - 1))
                yi = height - 1 - yi
                if 0 <= xi < width and 0 <= yi < height:
                    grid[yi][xi] = 'X'

    lines = []
    lines.append("  1/alpha_i vs log10(mu/GeV)  [1=alpha_1, 2=alpha_2, 3=alpha_3, X=TECS-L crossing]")
    lines.append(f"  {'':>{6}}|{'':^{width}}|")

    for row in range(height):
        y_val = y_max - row * (y_max - y_min) / (height - 1)
        label = f"{y_val:6.1f}"
        row_str = ''.join(grid[row])
        lines.append(f"  {label}|{row_str}|")

    lines.append(f"  {'':>{6}}+{'=' * width}+")
    # x-axis ticks
    tick_line = f"  {'':>{7}}"
    n_ticks = 8
    for i in range(n_ticks + 1):
        val = x_min + i * (x_max - x_min) / n_ticks
        pos = int(i * width / n_ticks)
        tick = f"{val:.0f}"
        if i < n_ticks:
            tick_line += tick.ljust(width // n_ticks)
        else:
            tick_line += tick
    lines.append(tick_line)
    lines.append(f"  {'':>{6}}  {'log10(mu / GeV)':^{width}}")

    return '\n'.join(lines)


# ═══════════════════════════════════════════════════════════════════
# Monte Carlo Validation
# ═══════════════════════════════════════════════════════════════════

def mc_validation(n_trials=10000, n_targets=12, n_scales=14, log_range=(0, 18)):
    """Test how often 3 random monotonic functions cross TECS-L values
    near known particle masses.

    For each trial:
      - Generate 3 random linear functions in log-space (random slope + intercept)
      - Check if they cross any of n_targets TECS-L inverse values
      - Check if any crossing is within 15% of one of n_scales known masses
    Count fraction of trials with at least 1 mass-proximate crossing.
    """
    rng = np.random.default_rng(42)

    # Known mass log-positions
    mass_logs = np.array([math.log10(m) for m in ENERGY_SCALES.values() if m > 0])
    log_lo, log_hi = log_range

    # TECS-L inverse targets
    inv_targets = list(TECSL_INV_TARGETS.values())
    # Also add inverses of alpha targets that are > 0
    for v in TECSL_ALPHA_TARGETS.values():
        if v > 0:
            inv_targets.append(1.0 / v)
    inv_targets = np.array(sorted(set(inv_targets)))

    count_any_cross = 0
    count_mass_match = 0
    count_dual_match = 0  # two couplings hit TECS-L at same scale

    for _ in range(n_trials):
        # 3 random monotonic linear functions: y = a + b*x where x = log10(mu)
        # Mimic coupling running: intercepts ~ 5-60, slopes ~ -1 to +1
        intercepts = rng.uniform(5, 60, size=3)
        slopes = rng.uniform(-1.5, 1.5, size=3)

        all_cross_logs = []  # log10(mu) of all crossings this trial
        has_mass_match = False

        for c in range(3):
            for tgt in inv_targets:
                # a + b*x = tgt  =>  x = (tgt - a) / b
                if abs(slopes[c]) < 1e-10:
                    continue
                x_cross = (tgt - intercepts[c]) / slopes[c]
                if log_lo <= x_cross <= log_hi:
                    all_cross_logs.append(x_cross)
                    # Check if near a known mass
                    for ml in mass_logs:
                        if abs(x_cross - ml) < 0.06:  # ~15% in log space
                            has_mass_match = True

        if len(all_cross_logs) > 0:
            count_any_cross += 1
        if has_mass_match:
            count_mass_match += 1

        # Check dual: two crossings at nearly same scale
        cross_arr = np.array(sorted(all_cross_logs))
        if len(cross_arr) >= 2:
            diffs = np.diff(cross_arr)
            if np.any(diffs < 0.1):
                count_dual_match += 1

    return {
        'n_trials': n_trials,
        'any_crossing': count_any_cross,
        'mass_match': count_mass_match,
        'dual_match': count_dual_match,
        'p_any': count_any_cross / n_trials,
        'p_mass': count_mass_match / n_trials,
        'p_dual': count_dual_match / n_trials,
    }


# ═══════════════════════════════════════════════════════════════════
# Main Analysis
# ═══════════════════════════════════════════════════════════════════

def run_analysis():
    """Run complete three-coupling unification & TECS-L analysis."""

    print("=" * 85)
    print("  THREE-COUPLING UNIFICATION & TECS-L CROSSING ANALYSIS")
    print("  1-Loop RGE | All SM Gauge Couplings | Weinberg Angle | MC Validation")
    print("=" * 85)
    print()

    # ─── Verify boundary conditions ───
    print("0. BOUNDARY CONDITIONS AT M_Z = {:.4f} GeV".format(M_Z))
    print("-" * 85)
    print(f"   1/alpha_1(M_Z) = {INV_A1_MZ:.3f}   (expected ~59.0)")
    print(f"   1/alpha_2(M_Z) = {INV_A2_MZ:.3f}   (expected ~29.6)")
    print(f"   1/alpha_3(M_Z) = {INV_A3_MZ:.3f}   (expected ~8.48)")
    print(f"   sin^2(theta_W) = {SIN2_THETA_W:.5f}")
    print()

    # ─── 1. Compute all three at 1000 log-spaced points ───
    log_mus = np.linspace(0, 18, 1000)  # 1 GeV to 10^18 GeV
    mus = 10**log_mus

    inv_a1 = np.array([inv_alpha_1(mu) for mu in mus])
    inv_a2 = np.array([inv_alpha_2(mu) for mu in mus])
    inv_a3 = np.array([inv_alpha_3(mu) for mu in mus])

    print("1. COUPLING EVOLUTION (selected scales)")
    print("-" * 85)
    print(f"   {'log10(mu)':>10} {'mu (GeV)':>14} {'1/alpha_1':>10} {'1/alpha_2':>10} {'1/alpha_3':>10} {'sin2_W':>10}")
    print(f"   {'='*10} {'='*14} {'='*10} {'='*10} {'='*10} {'='*10}")

    sample_logs = [0, 1, 2, 3, 4, 5, 6, 8, 10, 12, 14, 16, 17, 18]
    for lm in sample_logs:
        mu = 10**lm
        ia1 = inv_alpha_1(mu)
        ia2 = inv_alpha_2(mu)
        ia3 = inv_alpha_3(mu)
        sw2 = sin2_weinberg(mu)
        sw_str = f"{sw2:.5f}" if not math.isnan(sw2) else "N/A"
        print(f"   {lm:10.1f} {mu:14.3e} {ia1:10.3f} {ia2:10.3f} {ia3:10.3f} {sw_str:>10}")
    print()

    # ─── 2 & 3. TECS-L crossing scan for each coupling ───
    print("2. TECS-L CROSSINGS FOR EACH COUPLING")
    print("-" * 85)

    coupling_funcs = {
        'alpha_1': inv_alpha_1,
        'alpha_2': inv_alpha_2,
        'alpha_3': inv_alpha_3,
    }

    all_crossings = []  # (coupling_name, label, mu, inv_alpha_val)

    for coup_name, inv_func in coupling_funcs.items():
        print(f"\n   --- {coup_name} ---")
        print(f"   {'Target':.<40} {'1/alpha':>8} {'mu (GeV)':>14} {'log10(mu)':>10} {'Near scale':>20}")

        # Check alpha = TECS-L fraction => 1/alpha = 1/fraction
        for label, frac in TECSL_ALPHA_TARGETS.items():
            inv_target = 1.0 / frac
            mu_cross = find_crossing(inv_func, inv_target)
            if mu_cross:
                prox = check_energy_proximity(mu_cross)
                prox_str = ', '.join(f"{p[0]}({p[2]:+.1f}%)" for p in prox) if prox else '-'
                print(f"   {label:.<40} {inv_target:8.2f} {mu_cross:14.4e} {math.log10(mu_cross):10.3f} {prox_str:>20}")
                all_crossings.append((coup_name, f"alpha={label}", mu_cross, inv_target))

        # Check 1/alpha = TECS-L integer
        for label, intval in TECSL_INV_TARGETS.items():
            mu_cross = find_crossing(inv_func, float(intval))
            if mu_cross:
                prox = check_energy_proximity(mu_cross)
                prox_str = ', '.join(f"{p[0]}({p[2]:+.1f}%)" for p in prox) if prox else '-'
                print(f"   1/alpha = {label:.<28} {intval:8.0f} {mu_cross:14.4e} {math.log10(mu_cross):10.3f} {prox_str:>20}")
                all_crossings.append((coup_name, f"1/alpha={label}", mu_cross, float(intval)))

    print()

    # ─── 4. Proximity to known masses ───
    print("3. CROSSINGS NEAR KNOWN PARTICLE MASSES")
    print("-" * 85)
    near_mass = [(c, l, m, v) for c, l, m, v in all_crossings if check_energy_proximity(m)]
    if near_mass:
        for coup, label, mu, val in near_mass:
            prox = check_energy_proximity(mu)
            for p_name, p_mass, p_pct in prox:
                print(f"   {coup:8s}: {label:40s} at {mu:.4e} GeV  <->  {p_name} ({p_mass:.3f} GeV, {p_pct:+.1f}%)")
    else:
        print("   No crossings within 15% of known particle masses.")
    print()

    # ─── 5. Dual crossings: two couplings at TECS-L at same scale ───
    print("4. DUAL CROSSINGS: Two couplings at TECS-L values at SAME energy")
    print("-" * 85)

    dual_found = []
    for i in range(len(all_crossings)):
        for j in range(i + 1, len(all_crossings)):
            c1, l1, m1, v1 = all_crossings[i]
            c2, l2, m2, v2 = all_crossings[j]
            if c1 == c2:
                continue  # same coupling, skip
            # Check if scales are within 0.5 in log10 space
            if abs(math.log10(m1) - math.log10(m2)) < 0.5:
                dual_found.append((c1, l1, m1, v1, c2, l2, m2, v2))

    if dual_found:
        for c1, l1, m1, v1, c2, l2, m2, v2 in dual_found:
            sep = abs(math.log10(m1) - math.log10(m2))
            print(f"   {c1}: {l1}")
            print(f"     at mu = {m1:.4e} GeV (log10 = {math.log10(m1):.3f})")
            print(f"   {c2}: {l2}")
            print(f"     at mu = {m2:.4e} GeV (log10 = {math.log10(m2):.3f})")
            print(f"     separation: {sep:.3f} decades")
            print()
    else:
        print("   No dual crossings found within 0.5 decades.")
    print()

    # ─── 6. Unification analysis ───
    print("5. UNIFICATION ANALYSIS")
    print("-" * 85)

    # Pairwise crossing scales (analytical for 1-loop)
    # inv_a_i(M_Z) + b_i/(2pi)*ln = inv_a_j(M_Z) + b_j/(2pi)*ln
    # => ln = (inv_a_i - inv_a_j) / ((b_j - b_i)/(2pi))

    def crossing_scale(inv_i_mz, b_i, inv_j_mz, b_j):
        # 1/alpha_i = inv_i - b_i/(2pi)*ln = 1/alpha_j = inv_j - b_j/(2pi)*ln
        # (inv_i - inv_j) = (b_i - b_j)/(2pi) * ln
        denom = (b_i - b_j) / (2 * math.pi)
        if abs(denom) < 1e-15:
            return None
        ln_ratio = (inv_i_mz - inv_j_mz) / denom
        return M_Z * math.exp(ln_ratio)

    mu_23 = crossing_scale(INV_A2_MZ, B2, INV_A3_MZ, B3)
    mu_12 = crossing_scale(INV_A1_MZ, B1, INV_A2_MZ, B2)
    mu_13 = crossing_scale(INV_A1_MZ, B1, INV_A3_MZ, B3)

    if mu_23:
        inv_at_23 = inv_alpha_2(mu_23)
        log_23 = math.log10(mu_23)
        print(f"   alpha_2 = alpha_3 crossing:  mu = {mu_23:.4e} GeV  (log10 = {log_23:.4f})")
        print(f"     1/alpha at crossing = {inv_at_23:.4f}")
        print(f"     alpha at crossing   = {1/inv_at_23:.6f}")
    if mu_12:
        inv_at_12 = inv_alpha_1(mu_12)
        log_12 = math.log10(mu_12)
        print(f"   alpha_1 = alpha_2 crossing:  mu = {mu_12:.4e} GeV  (log10 = {log_12:.4f})")
        print(f"     1/alpha at crossing = {inv_at_12:.4f}")
    if mu_13:
        inv_at_13 = inv_alpha_1(mu_13)
        log_13 = math.log10(mu_13)
        print(f"   alpha_1 = alpha_3 crossing:  mu = {mu_13:.4e} GeV  (log10 = {log_13:.4f})")
        print(f"     1/alpha at crossing = {inv_at_13:.4f}")
    print()

    print("   SM couplings do NOT unify at a single point (three pairwise crossings differ).")
    print(f"   Triangle formed by crossings spans log10 = {math.log10(mu_23):.2f} to {math.log10(mu_12):.2f}")
    print()

    # TECS-L checks on GUT scale
    print("   TECS-L analysis of unification scales:")
    if mu_23:
        log23 = math.log10(mu_23)
        print(f"     log10(mu_23) = {log23:.4f}")
        print(f"       sigma + tau       = {S} + {T} = {S+T} = 16  (compare: {log23:.2f})")
        print(f"       sigma + tau - 1   = {S+T-1} = 15")
        print(f"       2*n + tau         = {2*N+T} = 16")
    if mu_13:
        log13 = math.log10(mu_13)
        print(f"     log10(mu_13) = {log13:.4f}")
        print(f"       sigma + tau + phi = {S+T+P} = 18")
        print(f"       3*n               = {3*N} = 18")

    if mu_23:
        inv_gut = inv_alpha_2(mu_23)
        alpha_gut = 1.0 / inv_gut if inv_gut > 0 else float('nan')
        print(f"\n     alpha_GUT (at alpha_2=alpha_3 crossing) = {alpha_gut:.6f}")
        print(f"     1/alpha_GUT = {inv_gut:.4f}")
        # Check against TECS-L
        for label, val in TECSL_INV_TARGETS.items():
            if abs(inv_gut - val) / max(val, 1) < 0.10:
                pct = (inv_gut - val) / val * 100
                print(f"       ** NEAR 1/alpha = {label} ({pct:+.2f}%)")
        for label, frac in TECSL_ALPHA_TARGETS.items():
            if abs(alpha_gut - frac) / max(frac, 1e-10) < 0.10:
                pct = (alpha_gut - frac) / frac * 100
                print(f"       ** NEAR alpha = {label} ({pct:+.2f}%)")
    print()

    # ─── 7. Weinberg angle running ───
    print("6. WEINBERG ANGLE RUNNING")
    print("-" * 85)

    sw2_mz = sin2_weinberg(M_Z)
    print(f"   sin^2(theta_W) at M_Z = {sw2_mz:.5f}  (input: {SIN2_THETA_W})")
    print(f"   Compare: 3/13 = {3/13:.5f},  (sigma/tau)/(sigma+1) = {(S/T)/(S+1):.5f}")
    print()

    weinberg_targets = {
        '1/4 = 1/tau':    1.0 / 4,
        '1/3 = tau/sigma': 1.0 / 3,
        '3/8 (SU(5))':    3.0 / 8,
        '2/9 = Koide':    2.0 / 9,
        '1/6 = 1/n':      1.0 / 6,
        '1/2 = phi/tau':  1.0 / 2,
    }

    print(f"   {'Target':.<30} {'Value':>8} {'mu (GeV)':>14} {'log10(mu)':>10} {'TECS-L form':>25}")
    print(f"   {'='*30} {'='*8} {'='*14} {'='*10} {'='*25}")

    for label, target in weinberg_targets.items():
        mu_w = find_sin2_crossing(target)
        if mu_w:
            lm = math.log10(mu_w)
            # Check TECS-L form for log10
            tecs_form = ""
            for tl, tv in [('n', N), ('sigma', S), ('tau', T), ('phi', P),
                           ('sigma+tau', S+T), ('sigma-tau', S-T), ('sigma*phi', S*P),
                           ('3*n', 3*N), ('2*n', 2*N)]:
                if abs(lm - tv) < 0.3:
                    tecs_form = f"log10~{tl}={tv}"
            print(f"   {label:.<30} {target:8.5f} {mu_w:14.4e} {lm:10.4f} {tecs_form:>25}")
        else:
            print(f"   {label:.<30} {target:8.5f} {'not in range':>14}")
    print()

    # Detailed Weinberg angle commentary
    print("   3/8 = SU(5) prediction at GUT scale:")
    print(f"     sigma / (sigma + sigma + sigma - tau) = {S} / ({S}+{S}+{S}-{T}) = {S}/{3*S-T} = {S/(3*S-T):.5f}")
    print(f"     3 / (3 + sopfr) = 3 / (3+{F}) = 3/{3+F} = {3/(3+F):.5f}  {'MATCH 3/8' if 3/(3+F) == 3/8 else ''}")
    print(f"     tau / (sigma - phi) = {T} / ({S}-{P}) = {T}/{S-P} = {T/(S-P):.5f}")
    print()

    # ─── 8. ASCII Plot ───
    print("7. ASCII PLOT: All Three Couplings")
    print("-" * 85)

    crossing_marks = []
    for c, l, m, v in all_crossings:
        crossing_marks.append((f"{c}:{l[:15]}", m, v))

    plot = ascii_unification_plot(log_mus, inv_a1, inv_a2, inv_a3, crossing_marks[:20])
    print(plot)
    print()

    # ─── 9. MC Validation ───
    print("8. MONTE CARLO VALIDATION")
    print("-" * 85)
    print("   Testing: how often do 3 random monotonic functions cross TECS-L")
    print("   values near known particle masses? (10,000 trials)")
    print()

    mc = mc_validation(n_trials=10000)
    print(f"   Trials: {mc['n_trials']}")
    print(f"   Any TECS-L crossing in range:     {mc['any_crossing']:5d}  ({mc['p_any']*100:.1f}%)")
    print(f"   Crossing near known mass (<15%):   {mc['mass_match']:5d}  ({mc['p_mass']*100:.1f}%)")
    print(f"   Dual crossing (two at same scale): {mc['dual_match']:5d}  ({mc['p_dual']*100:.1f}%)")
    print()

    # Compare to actual SM
    actual_near = len([1 for c, l, m, v in all_crossings if check_energy_proximity(m)])
    print(f"   Actual SM crossings near known masses: {actual_near}")
    if mc['p_mass'] > 0:
        print(f"   SM result vs random expectation: {actual_near} actual vs {mc['p_mass']*100:.1f}% random chance")
    print()

    # ─── Summary ───
    print("=" * 85)
    print("  SUMMARY")
    print("=" * 85)
    print()
    print(f"  Total TECS-L crossings found: {len(all_crossings)}")
    print(f"  Crossings near particle masses: {actual_near}")
    print(f"  Dual crossings (two couplings at TECS-L, same scale): {len(dual_found)}")
    print()

    if mu_23:
        print(f"  alpha_2 = alpha_3 unification at 10^{math.log10(mu_23):.1f} GeV")
        print(f"    1/alpha_GUT ~ {inv_alpha_2(mu_23):.1f}")
    print()

    mu_w_quarter = find_sin2_crossing(0.25)
    mu_w_third = find_sin2_crossing(1.0/3)
    mu_w_38 = find_sin2_crossing(3.0/8)
    if mu_w_quarter:
        print(f"  sin^2(theta_W) = 1/4 = 1/tau at 10^{math.log10(mu_w_quarter):.1f} GeV")
    if mu_w_third:
        print(f"  sin^2(theta_W) = 1/3 = tau/sigma at 10^{math.log10(mu_w_third):.1f} GeV")
    if mu_w_38:
        print(f"  sin^2(theta_W) = 3/8 (SU(5)) at 10^{math.log10(mu_w_38):.1f} GeV")
    print()
    print("=" * 85)
    print("  END OF ANALYSIS")
    print("=" * 85)


if __name__ == '__main__':
    run_analysis()
