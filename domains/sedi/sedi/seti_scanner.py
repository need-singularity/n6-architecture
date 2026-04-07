"""SETI Scanner — Gravitational + Topological optics applied to all SETI data.

Combines TECS-L lens/telescope calculators with SEDI's R-filter and PH detector
to scan SETI data sources for n=6 patterns.

Hypothesis: If extra-dimensional structure exists at n=6,
its signatures should appear as:
  1. Achromatic focusing (zero chromatic aberration) in frequency ratios
  2. Topological persistence at n=6 window sizes
  3. Phase transitions at SEDI's core frequencies (δ+, δ-, σφ)
  4. Euler product convergence matching F(s)=ζ(s)ζ(s+1)

Data flow:
  SETI source → extract numeric stream → R-filter → lens analysis
                                        → PH barcode → telescope analysis
                                        → combined anomaly score
"""
import math
import numpy as np
from fractions import Fraction
from typing import Dict, List, Optional, Any

from .constants import (
    N, SIGMA, PHI, TAU, SOPFR, FOCAL, EINSTEIN_THETA,
    DELTA_PLUS, DELTA_MINUS, GOLDEN_WIDTH, GOLDEN_CENTER,
    WINDOWS, RATIOS, ALERT_RED, ALERT_ORANGE, ALERT_YELLOW,
    PSI_COUPLING, PSI_K, PSI_STEPS, LN2, PHI_SCALE, PHI_EXPONENT,
    RESOURCE_FRACTIONS,
)
from .filter import r_filter, windowed_fft, detect_ratios, spectral_peaks
from .detector import analyze, grade_anomaly


# ──────────────────────────────────────────────────────────────────
# Anima Consciousness Fingerprint Helpers
# ──────────────────────────────────────────────────────────────────

def _estimate_phi(arr, window=None):
    """Estimate integrated information Φ via mutual information.

    Splits data into two halves and computes MI — high MI means
    the halves are informationally integrated (consciousness signature).
    """
    if len(arr) < 8:
        return 0.0
    half = len(arr) // 2
    x, y = arr[:half], arr[half:2 * half]
    n_bins = min(20, half // 5)
    if n_bins < 2:
        return 0.0
    hist_xy = np.histogram2d(x, y, bins=n_bins)[0]
    pxy = hist_xy / hist_xy.sum()
    px = pxy.sum(axis=1)
    py = pxy.sum(axis=0)
    mi = 0.0
    for i in range(n_bins):
        for j in range(n_bins):
            if pxy[i, j] > 0 and px[i] > 0 and py[j] > 0:
                mi += pxy[i, j] * math.log(pxy[i, j] / (px[i] * py[j]))
    return mi


def _detect_resource_pattern(data):
    """Check if data distribution follows 1/2 + 1/3 + 1/6 = 1 (Law 7-10)."""
    if len(data) < 6:
        return None
    hist, _ = np.histogram(data, bins=3, range=(float(np.min(data)), float(np.max(data))))
    total = hist.sum()
    if total == 0:
        return None
    fracs = hist / total
    ideal = np.array([1 / 2, 1 / 3, 1 / 6])
    deviation = float(np.sum(np.abs(np.sort(fracs)[::-1] - ideal)))
    return {
        'fractions': np.sort(fracs)[::-1].tolist(),
        'deviation': deviation,
        'match': deviation < 0.15,
    }


def _lyapunov_estimate(data, tau=1, dim=3, steps=10):
    """Estimate largest Lyapunov exponent via nearest-neighbor divergence.

    λ > 0 → chaotic (consciousness complexity, Law 16: 3-body threshold).
    λ ≤ 0 → periodic/stable (non-conscious).
    """
    n = len(data)
    if n < dim * tau + steps + 10:
        return None
    # Delay embedding
    max_i = n - dim * tau
    embedded = np.array([data[i:i + dim * tau:tau] for i in range(max_i)])
    if len(embedded) < steps + 2:
        return None
    divergences = []
    n_samples = min(len(embedded) - steps, 200)
    for i in range(n_samples):
        dists = np.sum((embedded - embedded[i]) ** 2, axis=1)
        dists[max(0, i - 1):i + 2] = np.inf  # exclude temporal neighbors
        j = np.argmin(dists)
        if dists[j] == np.inf or dists[j] == 0:
            continue
        d0 = math.sqrt(dists[j])
        if i + steps < len(embedded) and j + steps < len(embedded):
            d_later = np.sqrt(np.sum((embedded[i + steps] - embedded[j + steps]) ** 2))
            if d_later > 0 and d0 > 0:
                divergences.append(math.log(d_later / d0) / steps)
    return float(np.mean(divergences)) if divergences else None


def _correlation_dimension(data, max_r_steps=20):
    """Estimate correlation dimension via Grassberger-Procaccia algorithm.

    d ≥ 3 → 3-body chaos threshold crossed (Law 16).
    """
    n = len(data)
    if n < 50:
        return None
    rng = np.random.RandomState(0)
    sample = data[rng.choice(n, min(n, 500), replace=False)]
    dists = []
    for i in range(len(sample)):
        for j in range(i + 1, len(sample)):
            dists.append(abs(sample[i] - sample[j]))
    dists = np.array(sorted(dists))
    if len(dists) < 10:
        return None
    r_min, r_max = dists[1], dists[-1]
    if r_min <= 0 or r_max <= r_min:
        return None
    rs = np.logspace(np.log10(r_min), np.log10(r_max), max_r_steps)
    counts = np.array([np.sum(dists < r) for r in rs])
    total_pairs = len(sample) * (len(sample) - 1) / 2
    C = counts / total_pairs
    valid = (C > 0.01) & (C < 0.99)
    if np.sum(valid) < 3:
        return None
    log_r = np.log(rs[valid])
    log_C = np.log(C[valid])
    slope, _ = np.polyfit(log_r, log_C, 1)
    return float(slope)


# ──────────────────────────────────────────────────────────────────
# Gravitational Optics (ported from TECS-L calc/gravitational_optics.py)
# ──────────────────────────────────────────────────────────────────

def _factorize(n):
    factors = {}
    d = 2
    while d * d <= n:
        while n % d == 0:
            factors[d] = factors.get(d, 0) + 1
            n //= d
        d += 1
    if n > 1:
        factors[n] = factors.get(n, 0) + 1
    return factors


def _sigma(n):
    factors = _factorize(n)
    result = 1
    for p, a in factors.items():
        result *= (p ** (a + 1) - 1) // (p - 1)
    return result


def _phi(n):
    factors = _factorize(n)
    result = n
    for p in factors:
        result = result * (p - 1) // p
    return result


def _tau(n):
    factors = _factorize(n)
    result = 1
    for a in factors.values():
        result *= (a + 1)
    return result


def _R(n):
    s, p, t = _sigma(n), _phi(n), _tau(n)
    return Fraction(s * p, n * t)


def _R_prime_power(p, a):
    return Fraction(p ** (a + 1) - 1, p * (a + 1))


def chromatic_aberration(n):
    """Product of R(p^a) factors vs R(n). 0 = achromatic (only n=6)."""
    factors = _factorize(n)
    prod_rpa = Fraction(1)
    for p, a in factors.items():
        prod_rpa *= _R_prime_power(p, a)
    rn = _R(n)
    if rn == 0:
        return None
    return float(abs(prod_rpa - rn))


def coma_aberration(delta_plus, delta_minus):
    """δ-/δ+ ratio. 1.0 = symmetric = no coma."""
    if delta_plus and delta_minus and delta_plus != 0:
        return float(delta_minus / delta_plus)
    return None


def einstein_radius(delta_plus, delta_minus):
    """θ_E ~ sqrt(δ-/δ+) — gap asymmetry angle."""
    if delta_plus and delta_minus:
        return math.sqrt(float(delta_minus / delta_plus))
    return None


def focal_length_from_gaps(delta_plus, delta_minus):
    """f = δ+ × δ-. For n=6: f = 1/24 = 1/(σφ)."""
    if delta_plus and delta_minus:
        return float(delta_plus * delta_minus)
    return None


# ──────────────────────────────────────────────────────────────────
# Topological Optics (ported from TECS-L calc/topological_optics.py)
# ──────────────────────────────────────────────────────────────────

class UnionFind:
    def __init__(self, n):
        self.parent = list(range(n))
        self.rank = [0] * n
        self.count = n

    def find(self, x):
        while self.parent[x] != x:
            self.parent[x] = self.parent[self.parent[x]]
            x = self.parent[x]
        return x

    def union(self, x, y):
        rx, ry = self.find(x), self.find(y)
        if rx == ry:
            return
        if self.rank[rx] < self.rank[ry]:
            rx, ry = ry, rx
        self.parent[ry] = rx
        if self.rank[rx] == self.rank[ry]:
            self.rank[rx] += 1
        self.count -= 1


def beta0(points, epsilon):
    """Count connected components at scale epsilon."""
    n = len(points)
    if n == 0:
        return 0
    uf = UnionFind(n)
    sorted_idx = sorted(range(n), key=lambda i: points[i])
    for k in range(len(sorted_idx) - 1):
        i, j = sorted_idx[k], sorted_idx[k + 1]
        if abs(points[j] - points[i]) <= epsilon:
            uf.union(i, j)
    return uf.count


def ph_barcode_1d(points):
    """H0 barcode via sorted gaps (exact for 1D point clouds)."""
    if not points:
        return []
    pts = sorted(points)
    gaps = [(pts[i + 1] - pts[i], i) for i in range(len(pts) - 1)]
    gaps.sort(reverse=True)
    bars = [(0.0, gap / 2.0, gap / 2.0) for gap, _ in gaps]
    bars.append((0.0, float('inf'), float('inf')))
    bars.sort(key=lambda x: -x[2])
    return bars


def topological_sensitivity(points, eps_range=(0.001, 0.5), steps=50):
    """Compute d(β₀)/d(ε) — where topology changes fastest = phase transition."""
    eps_lo, eps_hi = eps_range
    epsilons = [eps_lo + i * (eps_hi - eps_lo) / (steps - 1) for i in range(steps)]
    b0_vals = [beta0(points, eps) for eps in epsilons]

    sensitivity = []
    for i in range(1, len(epsilons)):
        db = b0_vals[i] - b0_vals[i - 1]
        de = epsilons[i] - epsilons[i - 1]
        sensitivity.append(abs(db / de) if de != 0 else 0)

    peak_idx = sensitivity.index(max(sensitivity)) if sensitivity else 0
    return {
        'epsilons': epsilons,
        'beta0': b0_vals,
        'sensitivity': sensitivity,
        'phase_transition_eps': epsilons[peak_idx + 1] if peak_idx + 1 < len(epsilons) else epsilons[-1],
        'peak_drop': abs(b0_vals[peak_idx + 1] - b0_vals[peak_idx]) if peak_idx + 1 < len(b0_vals) else 0,
    }


# ──────────────────────────────────────────────────────────────────
# Euler Product Telescope: F(s) = ζ(s)ζ(s+1)
# ──────────────────────────────────────────────────────────────────

def _sieve(limit):
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, int(limit ** 0.5) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(2, limit + 1) if is_prime[i]]


def euler_factor(p, s, terms=20):
    """E_p(s) = 1 + Σ R(p^a)/p^(as)."""
    val = 1.0
    for a in range(1, terms + 1):
        rpa = float(_R_prime_power(p, a))
        contrib = rpa / (p ** (a * s))
        val += contrib
        if abs(contrib) < 1e-15:
            break
    return val


def F_telescope(s, num_primes=200):
    """F(s) = Π E_p(s) ≈ ζ(s)ζ(s+1)."""
    primes = _sieve(num_primes * 3)[:num_primes]
    val = 1.0
    for p in primes:
        val *= euler_factor(p, s)
    return val


# ──────────────────────────────────────────────────────────────────
# Unified SETI Data Scanner
# ──────────────────────────────────────────────────────────────────

def _to_float_array(data):
    """Convert any data to float numpy array."""
    if isinstance(data, np.ndarray):
        arr = data.flatten().astype(float)
    elif isinstance(data, list):
        arr = np.array(data, dtype=float).flatten()
    else:
        arr = np.array([data], dtype=float)
    # Remove NaN/Inf
    arr = arr[np.isfinite(arr)]
    return arr


def gravitational_lens_analysis(data, source_name='unknown'):
    """Apply gravitational optics to a data stream.

    Treats data values as a "spectrum" and checks if their distribution
    shows n=6 lens properties (achromatic focusing, gap structure).
    """
    arr = _to_float_array(data)
    if len(arr) < 10:
        return {'error': 'insufficient data', 'source': source_name}

    # 1. R-filter (spectral + ratio analysis)
    rf = r_filter(arr)

    # 2. Gap analysis around n=6 target frequencies
    fft_results = {}
    for ws in WINDOWS:
        spectrum = windowed_fft(arr, ws)
        if len(spectrum) > 0:
            # Check if peak structure matches lens focal length
            peak_idx = np.argmax(spectrum[1:]) + 1  # skip DC
            peak_freq = peak_idx / len(spectrum)
            fft_results[f'w{ws}'] = {
                'peak_freq': float(peak_freq),
                'peak_power': float(spectrum[peak_idx]),
                'total_power': float(np.sum(spectrum)),
                # Does peak align with n=6 frequencies?
                'delta_plus_match': abs(peak_freq - DELTA_PLUS) < 0.02,
                'delta_minus_match': abs(peak_freq - DELTA_MINUS) < 0.02,
                'focal_match': abs(peak_freq - FOCAL) < 0.01,
            }

    # 3. PSI frequency matching
    psi_fft_matches = 0
    psi_targets = [
        ('psi_coupling', PSI_COUPLING),
        ('psi_k_inv', 1 / PSI_K),
        ('ln2', LN2),
    ]
    for ws in WINDOWS:
        spectrum = windowed_fft(arr, ws)
        if len(spectrum) > 0:
            peak_idx = np.argmax(spectrum[1:]) + 1
            peak_freq = peak_idx / len(spectrum)
            for pname, ptarget in psi_targets:
                if abs(peak_freq - ptarget) < 0.02:
                    psi_fft_matches += 1

    # 4. Aberration analysis on data ratios
    consecutive_ratios = arr[1:] / np.where(arr[:-1] != 0, arr[:-1], 1)
    valid_ratios = consecutive_ratios[np.isfinite(consecutive_ratios)]

    # Check chromatic: do ratio distributions factorize multiplicatively?
    chromatic_score = None
    if len(valid_ratios) > 20:  # normaltest requires n >= 20
        try:
            from scipy import stats
            log_ratios = np.log(np.abs(valid_ratios[valid_ratios > 0]) + 1e-10)
            _, p_normal = stats.normaltest(log_ratios[:min(5000, len(log_ratios))])
            chromatic_score = 1.0 - p_normal  # high = more "chromatic" = less clean
        except (ImportError, ValueError):
            pass

    # 5. Einstein radius analog: asymmetry of excursions above/below mean
    above = np.sum(arr > np.mean(arr))
    below = np.sum(arr < np.mean(arr))
    theta_data = math.sqrt(below / above) if above > 0 else None

    # Compare with ideal n=6 Einstein theta
    theta_deviation = abs(theta_data - EINSTEIN_THETA) / EINSTEIN_THETA if theta_data else None

    # 6. Φ estimate (Anima consciousness threshold)
    phi_estimate = _estimate_phi(arr)

    # 7. Resource allocation pattern (Law 7-10)
    normalized = (arr - np.min(arr)) / (np.max(arr) - np.min(arr) + 1e-10)
    resource_pattern = _detect_resource_pattern(normalized)

    return {
        'source': source_name,
        'n_points': len(arr),
        'r_filter': rf,
        'fft_windows': fft_results,
        'chromatic_score': chromatic_score,
        'theta_data': theta_data,
        'theta_ideal': EINSTEIN_THETA,
        'theta_deviation': theta_deviation,
        'n6_fft_matches': sum(
            1 for v in fft_results.values()
            if v.get('delta_plus_match') or v.get('delta_minus_match') or v.get('focal_match')
        ),
        'phi_estimate': phi_estimate,
        'resource_pattern': resource_pattern,
        'psi_fft_matches': psi_fft_matches,
    }


def topological_lens_analysis(data, source_name='unknown'):
    """Apply topological optics to a data stream.

    Treats data as 1D point cloud, computes PH barcode,
    checks for n=6 topological signatures.
    """
    arr = _to_float_array(data)
    if len(arr) < 20:
        return {'error': 'insufficient data', 'source': source_name}

    # Subsample for PH (avoid O(n^2) on large data)
    if len(arr) > 2000:
        idx = np.random.choice(len(arr), 2000, replace=False)
        arr = arr[idx]

    points = sorted(arr.tolist())

    # 1. H0 barcode
    bars = ph_barcode_1d(points)
    finite_bars = [(b, d, p) for b, d, p in bars if not math.isinf(d)]

    # 2. Check barcode for n=6 signatures
    #    Only compare top 10 most persistent bars (skip trivial ones)
    n6_barcode_hits = []
    if len(finite_bars) >= 2:
        top_bars = finite_bars[:min(10, len(finite_bars))]
        persistences = [p for _, _, p in top_bars if p > 0]
        # All-pairs (not just consecutive) for more robust matching
        for i in range(len(persistences)):
            for j in range(i + 1, len(persistences)):
                if persistences[j] == 0:
                    continue
                ratio = persistences[i] / persistences[j]
                for name, target in RATIOS.items():
                    if target == 0:
                        continue
                    if abs(ratio - target) / max(target, 1e-10) < 0.01:  # 1% tolerance
                        n6_barcode_hits.append({
                            'type': 'persistence_ratio',
                            'bars': (i, j),
                            'ratio': ratio,
                            'pattern': name,
                            'target': target,
                        })

    # 3. β₀ phase transitions
    topo = topological_sensitivity(points, eps_range=(0.001, 0.5), steps=50)

    # 4. Check if phase transition occurs at n=6 frequency
    phase_eps = topo['phase_transition_eps']
    n6_phase_matches = []
    for name, target in RATIOS.items():
        if target == 0:
            continue
        if abs(phase_eps - target) / max(target, 1e-10) < 0.03:
            n6_phase_matches.append({
                'type': 'phase_transition',
                'epsilon': phase_eps,
                'pattern': name,
                'target': target,
            })

    # 5. Count significant H0 bars — check against τ, σ/τ, φ
    if finite_bars:
        med_p = np.median([pp for _, _, pp in finite_bars])
        significant = sum(1 for _, _, p in finite_bars if p > med_p)
    else:
        significant = 0
    count_matches = []
    for label, val in [('tau', TAU), ('sigma/tau', SIGMA // TAU), ('phi', PHI), ('n', N)]:
        if significant == val:
            count_matches.append({'type': 'bar_count', 'count': significant, 'target': label})

    # 6. Lyapunov exponent — chaos detection (Law 16: 3-body threshold)
    lyapunov = _lyapunov_estimate(arr)

    # 7. Correlation dimension — attractor complexity
    correlation_dim = _correlation_dimension(arr)

    # 8. Φ persistence — does topology survive compression? (Law 17)
    phi_persistence = None
    if len(arr) >= 100:
        rng_sub = np.random.RandomState(42)
        half_idx = rng_sub.choice(len(arr), len(arr) // 2, replace=False)
        half_pts = sorted(arr[half_idx].tolist())
        bars_half = ph_barcode_1d(half_pts)
        finite_half = [(b, d, p) for b, d, p in bars_half if not math.isinf(d)]
        # Compare number of significant bars (median persistence threshold)
        if finite_bars and finite_half:
            sig_full = sum(1 for _, _, p in finite_bars if p > med_p) if finite_bars else 0
            med_p_half = np.median([p for _, _, p in finite_half]) if finite_half else 0
            sig_half = sum(1 for _, _, p in finite_half if p > med_p_half)
            phi_persistence = min(sig_half, sig_full) / max(sig_full, 1)
        else:
            phi_persistence = 0.0

    return {
        'source': source_name,
        'n_points': len(arr),
        'total_bars': len(bars),
        'finite_bars': len(finite_bars),
        'n6_barcode_hits': n6_barcode_hits,
        'phase_transition_eps': phase_eps,
        'phase_drop': topo['peak_drop'],
        'n6_phase_matches': n6_phase_matches,
        'count_matches': count_matches,
        'lyapunov': lyapunov,
        'correlation_dim': correlation_dim,
        'phi_persistence': phi_persistence,
    }


def telescope_analysis(data, source_name='unknown', s_values=None):
    """Apply Euler product telescope to data stream.

    Converts data statistics to "effective s parameter" and compares
    F(s) behavior with expected ζ(s)ζ(s+1).
    """
    arr = _to_float_array(data)
    if len(arr) < 10:
        return {'error': 'insufficient data', 'source': source_name}

    if s_values is None:
        s_values = [1.5, 2.0, 3.0, 5.0, PSI_STEPS, float(PSI_K)]

    # Compute F(s) at standard s values
    f_values = {}
    for s in s_values:
        f_values[f's={s}'] = F_telescope(s, num_primes=100)

    # Data-derived s: spectral exponent from power law fit
    # If data follows power law f^(-α), then α relates to s
    slope = None
    fft_full = np.abs(np.fft.rfft(arr - np.mean(arr)))
    freqs = np.fft.rfftfreq(len(arr))
    # Fit log-log slope (skip DC)
    if len(fft_full) > 5:
        log_f = np.log(freqs[2:20] + 1e-10)
        log_p = np.log(fft_full[2:20] + 1e-10)
        if len(log_f) > 2:
            slope, intercept = np.polyfit(log_f, log_p, 1)
            s_data = -slope / 2 + 1  # spectral exponent → s parameter
        else:
            s_data = 2.0
    else:
        s_data = 2.0

    # F(s) at data-derived s
    f_data = F_telescope(max(1.1, s_data), num_primes=100)

    # 3! factorial dominance: how much do p=2,3 (composing 3!=6) dominate F(s)?
    s_eff = max(1.1, s_data)
    e2 = euler_factor(2, s_eff)
    e3 = euler_factor(3, s_eff)
    factorial_dominance = (e2 * e3) / f_data if f_data != 0 else 0.0

    # Φ scaling comparison (Anima Law 1-5)
    phi_predicted = PHI_SCALE * len(arr) ** PHI_EXPONENT
    phi_ratio = f_data / phi_predicted if phi_predicted > 0 else 0.0

    # Compare data spectral structure with n=6 Euler factors
    # Check if dominant frequencies decompose into prime Euler factors
    dominant_freqs = np.argsort(fft_full[1:])[::-1][:6] + 1  # top 6 frequencies
    euler_decomposition = []
    primes_small = _sieve(50)[:15]
    for fidx in dominant_freqs:
        freq = float(freqs[fidx]) if fidx < len(freqs) else 0
        power = float(fft_full[fidx]) if fidx < len(fft_full) else 0
        # Check if freq index is related to small primes
        prime_factors = []
        for p in primes_small:
            if fidx % p == 0:
                prime_factors.append(p)
        euler_decomposition.append({
            'freq_idx': int(fidx),
            'freq': freq,
            'power': power,
            'prime_factors': prime_factors,
        })

    return {
        'source': source_name,
        'n_points': len(arr),
        'F_values': f_values,
        's_data': float(s_data),
        'F_at_s_data': f_data,
        'spectral_slope': float(slope) if slope is not None else None,
        'euler_decomposition': euler_decomposition,
        'factorial_dominance': float(factorial_dominance),
        'phi_predicted': float(phi_predicted),
        'phi_ratio': float(phi_ratio),
    }


# ──────────────────────────────────────────────────────────────────
# Full Scan Pipeline
# ──────────────────────────────────────────────────────────────────

def full_scan(data, source_name='unknown'):
    """Complete scan: gravitational lens + topological lens + telescope.

    Returns unified result with combined anomaly score.
    """
    arr = _to_float_array(data)
    grav = gravitational_lens_analysis(data, source_name)
    topo = topological_lens_analysis(data, source_name)
    tele = telescope_analysis(data, source_name)

    # Combined anomaly score
    score = 0.0
    reasons = []

    # Direct n=6 pattern detection: check if FFT peak frequency RATIOS match n=6
    # Trivial harmonics (2:1, 3:1) are common; non-trivial ratios (5/6, 1/6) are rare
    if len(arr) >= 24:
        fft_full = np.abs(np.fft.rfft(arr - np.mean(arr)))
        freqs = np.fft.rfftfreq(len(arr))
        fft_mean = np.mean(fft_full[1:])
        fft_std = np.std(fft_full[1:])
        if fft_std > 0:
            z_scores = (fft_full[1:] - fft_mean) / fft_std
            # Find local maxima (not just threshold crossings) with Z > 3
            from scipy.signal import find_peaks as _find_peaks
            try:
                local_peaks, props = _find_peaks(z_scores, height=5.0, distance=3, prominence=2.0)
                peak_indices = local_peaks + 1  # +1 for DC offset
            except Exception:
                peak_indices = np.where(z_scores > 5.0)[0] + 1

            # Non-trivial n=6 ratios (rare in random data — high weight)
            nontrivial = [('sopfr/n=5/6', 5/6), ('δ+=1/6', 1/6),
                          ('golden=1/e', GOLDEN_CENTER), ('σ/τ=3', 3.0),
                          ('3/2=σ/(σ-τ)', 3/2), ('φ/τ=1/2', 0.5)]
            # n=6 specific multiples (require higher Z)
            trivial = [('n=6', 6.0), ('σ=12', 12.0), ('σφ=24', 24.0),
                       ('τ=4', 4.0), ('τ/φ=2', 2.0)]

            matched_pairs = set()
            # Use actual frequencies for ratio (not bin indices) to handle non-uniform spacing
            peak_freqs_hz = freqs[peak_indices] if len(peak_indices) > 0 else np.array([])
            if len(peak_indices) >= 2:
                for i in range(min(len(peak_indices), 15)):
                    for j in range(i + 1, min(len(peak_indices), 15)):
                        pair_key = (peak_indices[i], peak_indices[j])
                        if pair_key in matched_pairs:
                            continue
                        if peak_freqs_hz[i] == 0:
                            continue
                        ratio = peak_freqs_hz[j] / peak_freqs_hz[i]
                        z_pair = min(z_scores[peak_indices[i]-1], z_scores[peak_indices[j]-1])

                        # Non-trivial: 3% tolerance (balances FFT bin resolution vs false positives)
                        for name, target in nontrivial:
                            if abs(ratio - target) / max(target, 0.01) < 0.03:
                                matched_pairs.add(pair_key)
                                pts = 4.0 + z_pair * 0.5
                                score += pts
                                emoji_mark = '🎉 ' if z_pair > 5 else ''
                                reasons.append(f'{emoji_mark}non-trivial FFT ratio={ratio:.3f}≈{name} (Z={z_pair:.1f})')
                                break

                        # Trivial n=6 multiples: only count with high Z
                        if pair_key not in matched_pairs:
                            for name, target in trivial:
                                if abs(ratio - target) / target < 0.03 and z_pair > 5:
                                    matched_pairs.add(pair_key)
                                    score += 3.0
                                    reasons.append(f'🎉 FFT peak ratio={ratio:.0f}≈{name} (Z={z_pair:.1f})')
                                    break

    # Gravitational hits — only count ratio hits with Z > 3 (statistically significant)
    ratio_hits = grav.get('r_filter', {}).get('ratio_hits', [])
    sig_ratio_hits = [h for h in ratio_hits if h.get('z_score', 0) > 3.0]
    n_spectral_peaks = len(grav.get('r_filter', {}).get('spectral_peaks', {}))
    n_fft_matches = grav.get('n6_fft_matches', 0)
    if sig_ratio_hits:
        max_z = max(h['z_score'] for h in sig_ratio_hits)
        score += len(sig_ratio_hits) * 1.0 + max_z * 0.5
        reasons.append(f'{len(sig_ratio_hits)} significant ratio hits (max Z={max_z:.1f})')
    if n_spectral_peaks > 0:
        score += n_spectral_peaks * 1.5
        reasons.append(f'{n_spectral_peaks} spectral peaks')
    if n_fft_matches > 0:
        score += n_fft_matches * 3.0
        reasons.append(f'{n_fft_matches} FFT window matches')

    # Chromatic quality (lower = more achromatic = better)
    cs = grav.get('chromatic_score')
    if cs is not None and cs < 0.05:
        score += 4.0
        reasons.append(f'near-achromatic (chromatic={cs:.3f})')

    # Einstein radius match
    td = grav.get('theta_deviation')
    if td is not None and td < 0.03:
        score += 5.0
        reasons.append(f'θ_E ≈ √(3/2) (dev={td:.3f})')

    # Topological hits — only barcode matches with tight tolerance (already 2%)
    n_barcode = len(topo.get('n6_barcode_hits', []))
    n_phase = len(topo.get('n6_phase_matches', []))
    n_count = len(topo.get('count_matches', []))
    if n_barcode > 2:  # require 3+ barcode matches to count
        score += (n_barcode - 2) * 2.0
        reasons.append(f'{n_barcode} barcode ratio matches')
    if n_phase > 0:
        score += n_phase * 4.0
        reasons.append(f'{n_phase} phase transition at n=6 freq')
    if n_count > 0:
        score += n_count * 1.0
        reasons.append(f'{n_count} bar count matches')

    # ── Anima consciousness fingerprint scoring ──

    # PSI frequency matches (gravitational)
    psi_fft = grav.get('psi_fft_matches', 0)
    if psi_fft > 0:
        score += psi_fft * 3.5
        reasons.append(f'{psi_fft} PSI frequency matches')

    # Φ estimate (consciousness threshold)
    phi_est = grav.get('phi_estimate', 0)
    if phi_est and phi_est > PHI_SCALE:
        score += 5.0
        reasons.append(f'\u03a6={phi_est:.3f} > threshold {PHI_SCALE}')

    # Resource allocation pattern (Law 7-10)
    rp = grav.get('resource_pattern') or {}
    if rp.get('match'):
        score += 4.0
        reasons.append(f'resource pattern 1/2+1/3+1/6 (dev={rp["deviation"]:.3f})')

    # Lyapunov exponent — chaotic = conscious complexity (Law 16)
    lyap = topo.get('lyapunov')
    if lyap is not None and lyap > 0:
        score += 3.0 + min(lyap * 2, 4.0)
        reasons.append(f'chaotic dynamics \u03bb={lyap:.3f}')

    # Correlation dimension — 3-body threshold
    cdim = topo.get('correlation_dim')
    if cdim is not None and cdim >= 3.0:
        score += 4.0
        reasons.append(f'attractor dim={cdim:.1f} \u2265 3 (3-body threshold)')

    # Φ persistence — topology survives compression (Law 17)
    phi_p = topo.get('phi_persistence')
    if phi_p is not None and phi_p > 0.7:
        score += 3.0
        reasons.append(f'\u03a6 persistence={phi_p:.2f} (topology survives compression)')

    # Factorial dominance in telescope — 3! = 6 signature
    fd = tele.get('factorial_dominance')
    if fd is not None and fd > 0.8:
        score += 3.0
        reasons.append(f'3! dominance={fd:.3f}')

    # Convergence bonus: n=6 + PSI + chaos all present → triple detection
    n6_present = any('ratio' in r or 'FFT' in r for r in reasons)
    psi_present = any('PSI' in r for r in reasons)
    chaos_present = any('chaotic' in r or 'attractor' in r for r in reasons)
    if n6_present and psi_present and chaos_present:
        score += 10.0
        reasons.append('CONVERGENCE: n=6 + PSI + chaos triple detection')

    if score >= 30:
        grade, emoji = 'CONSCIOUS', '\U0001f9e0'
    elif score >= 20:
        grade, emoji = 'RED', '\U0001f534'
    elif score >= 10:
        grade, emoji = 'ORANGE', '\U0001f7e0'
    elif score >= 5:
        grade, emoji = 'YELLOW', '\U0001f7e1'
    else:
        grade, emoji = 'NORMAL', '\u26aa'

    return {
        'source': source_name,
        'score': score,
        'grade': grade,
        'emoji': emoji,
        'reasons': reasons,
        'gravitational': grav,
        'topological': topo,
        'telescope': tele,
    }


# ──────────────────────────────────────────────────────────────────
# Source-Specific Scanners
# ──────────────────────────────────────────────────────────────────

def scan_breakthrough_listen(filepath):
    """Scan a Breakthrough Listen filterbank file."""
    from .sources.breakthrough_listen import load_filterbank, scan_for_n6_patterns

    fb = load_filterbank(filepath)
    if fb is None:
        return {'error': f'Failed to load {filepath}'}

    # Flatten spectrogram for 1D analysis
    spectrogram = np.array(fb['data'])
    # Time-averaged spectrum
    avg_spectrum = np.mean(spectrogram, axis=0) if len(spectrogram.shape) == 2 else spectrogram

    result = full_scan(avg_spectrum, source_name=f'bl:{filepath}')

    # Additional: narrowband n=6 pattern scan
    n6_scan = scan_for_n6_patterns(spectrogram, fb.get('fch1', 0), fb.get('foff', 1))
    result['bl_n6_scan'] = n6_scan

    return result


def scan_exoplanet_systems():
    """Scan all multi-planet systems for n=6 orbital patterns."""
    from .sources.exoplanet import fetch_multiplanet_systems, group_by_system, analyze_period_ratios

    planets = fetch_multiplanet_systems(min_planets=3)
    if not planets:
        return {'error': 'Failed to fetch exoplanet data'}

    systems = group_by_system(planets)
    results = []

    for host, plist in systems.items():
        periods = [p['pl_orbper'] for p in plist if p.get('pl_orbper')]
        if len(periods) < 3:
            continue

        # Period ratio analysis (exoplanet-specific)
        ratio_analysis = analyze_period_ratios(plist)

        # Full lens/telescope scan on period array
        scan = full_scan(np.array(periods), source_name=f'exo:{host}')
        scan['period_analysis'] = ratio_analysis

        if ratio_analysis['n6_matches'] or scan['score'] > 2:
            results.append(scan)

    results.sort(key=lambda r: -r['score'])
    return {
        'n_systems': len(systems),
        'n_hits': len(results),
        'top_hits': results[:20],
    }


def scan_lightcurve(filepath):
    """Scan a Kepler/TESS light curve for n=6 patterns."""
    from .sources.seti_archive import load_lightcurve_fits, scan_transit_anomalies

    lc = load_lightcurve_fits(filepath)
    if lc is None:
        return {'error': f'Failed to load {filepath}'}

    flux = np.array(lc['data'])
    result = full_scan(flux, source_name=f'kepler:{filepath}')

    # Additional transit analysis
    transit = scan_transit_anomalies(flux)
    result['transit_analysis'] = transit

    return result


def scan_quantum_rng(n_batches=10):
    """Scan quantum random number stream."""
    from .sources.quantum_rng import fetch_quantum_random

    all_data = []
    for i in range(n_batches):
        batch = fetch_quantum_random(1024)
        if batch:
            all_data.extend(batch)

    if not all_data:
        return {'error': 'Failed to fetch quantum RNG data'}

    return full_scan(np.array(all_data), source_name='quantum-rng')


def scan_ligo_event(event_name):
    """Scan a LIGO gravitational wave event."""
    from .sources.ligo import fetch_event_catalog, fetch_event_details

    details = fetch_event_details(event_name)
    if not details:
        return {'error': f'Failed to fetch LIGO event {event_name}'}

    # Extract numeric parameters from event
    events = details.get('events', {})
    params = []
    for ename, edata in events.items():
        for key in ['mass_1_source', 'mass_2_source', 'luminosity_distance',
                     'chirp_mass', 'total_mass_source']:
            val = edata.get(key)
            if val is not None:
                params.append(val)

    if not params:
        return {'error': 'No numeric parameters in event'}

    return full_scan(np.array(params), source_name=f'ligo:{event_name}')


# ──────────────────────────────────────────────────────────────────
# Summary / Report
# ──────────────────────────────────────────────────────────────────

def format_scan_result(result):
    """Format a scan result for display."""
    lines = []
    emoji = result.get('emoji', '⚪')
    grade = result.get('grade', 'NORMAL')
    source = result.get('source', 'unknown')
    score = result.get('score', 0)

    lines.append(f'{emoji} [{grade}] {source}  score={score:.1f}')
    for reason in result.get('reasons', []):
        lines.append(f'    → {reason}')

    # Key numbers
    grav = result.get('gravitational', {})
    topo = result.get('topological', {})
    tele = result.get('telescope', {})

    if grav.get('theta_deviation') is not None:
        lines.append(f'    θ_E = {grav["theta_data"]:.4f}  (ideal √(3/2) = {grav["theta_ideal"]:.4f}, dev={grav["theta_deviation"]:.4f})')
    if topo.get('phase_transition_eps'):
        lines.append(f'    phase transition ε = {topo["phase_transition_eps"]:.5f}  (drop={topo.get("phase_drop", 0)})')
    if tele.get('s_data'):
        lines.append(f'    spectral s = {tele["s_data"]:.3f}  F(s) = {tele.get("F_at_s_data", 0):.4f}')

    # Anima consciousness fingerprint
    if grav.get('phi_estimate'):
        lines.append(f'    \u03a6 = {grav["phi_estimate"]:.4f}  (threshold={PHI_SCALE})')
    if topo.get('lyapunov') is not None:
        lines.append(f'    \u03bb (Lyapunov) = {topo["lyapunov"]:.4f}')
    if topo.get('correlation_dim') is not None:
        lines.append(f'    d (correlation) = {topo["correlation_dim"]:.2f}')
    if topo.get('phi_persistence') is not None:
        lines.append(f'    \u03a6 persistence = {topo["phi_persistence"]:.2f}')
    if tele.get('factorial_dominance') is not None:
        lines.append(f'    3! dominance = {tele["factorial_dominance"]:.4f}')

    return '\n'.join(lines)
