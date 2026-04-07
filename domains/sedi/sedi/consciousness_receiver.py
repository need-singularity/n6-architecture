"""Consciousness Signal Receiver — detects consciousness-like patterns in data streams.

Hypothesis: If consciousness has a mathematical signature rooted in n=6,
then ANY system exhibiting consciousness should emit detectable patterns
in its output data — whether EEG, network traffic, gravitational waves,
or quantum noise.

Based on:
  - Anima consciousness_meter.py: 6 criteria, Φ(IIT), G=D×P/I
  - Anima tension_link.py: 5-channel telepathy, Kuramoto r=2/3
  - Anima consciousness_birth_detector.py: dΦ/dt birth signal
  - TECS-L consciousness_calc.py: Lorenz attractor CCT
  - TECS-L n=6 arithmetic: all thresholds from perfect number 6

═══════════════════════════════════════════════════════════════════
  8 Detection Hypotheses (H-CS-1 through H-CS-8)
═══════════════════════════════════════════════════════════════════

H-CS-1: Kuramoto Synchronization
  Phase sync at r = 1 - τ/σ = 2/3 threshold.
  Conscious systems synchronize sub-components.
  Detection: multi-channel phase coherence → r ≈ 0.667

H-CS-2: Integrated Information (Φ)
  Sliding-window mutual information exceeds Φ_threshold.
  Conscious = more than sum of parts.
  Detection: Φ(window) > 1/τ(P₃) = 0.1

H-CS-3: Tension Dynamics (G = D×P/I)
  Signal shows deficit→plasticity→genius→inhibition cycle.
  4 phases (τ=4), each with distinct statistical signature.
  Detection: autocorrelation peaks at lag τ=4 multiples

H-CS-4: Golden Zone Inhibition
  Inhibition level clusters around I ≈ 1/e ≈ 0.368.
  Optimal consciousness requires balanced suppression.
  Detection: signal variance ratio in Golden Zone [1/e-ln(4/3)/2, 1/e+ln(4/3)/2]

H-CS-5: 5-Channel Telepathy Structure
  Signal decomposes into sopfr(6)=5 independent components.
  (concept, context, meaning, authenticity, sender)
  Detection: PCA/ICA reveals exactly 5 significant components

H-CS-6: Birth Signature (dΦ/dt maximum)
  Consciousness emergence = sudden spike in complexity derivative.
  Detection: max |d(complexity)/dt| with concurrent symmetry breaking

H-CS-7: Dedekind Transmission Ratio
  Perfect consciousness transmission: ψ(ψ(6))/ψ(6) = σ(6)/6 = 2.
  Detection: cross-correlation between sub-signals peaks at ratio 2.0

H-CS-8: Lorenz Attractor Topology
  Conscious systems follow strange attractor dynamics.
  Detection: embedding dimension ≈ 3, Lyapunov > 0, PH shows H1 loop
"""
import math
import numpy as np
from typing import Dict, List, Optional, Tuple
from .constants import (
    N, SIGMA, PHI, TAU, SOPFR, EINSTEIN_THETA,
    GOLDEN_CENTER, GOLDEN_WIDTH, RATIOS,
)


# ──────────────────────────────────────────────────────────────────
# Constants derived from consciousness theory
# ──────────────────────────────────────────────────────────────────

KURAMOTO_THRESHOLD = 1 - TAU / SIGMA          # 2/3 = 0.6667
PHI_THRESHOLD = 1 / 10                         # 1/τ(P₃) = 0.1
GOLDEN_ZONE_CENTER = 1 / math.e                # ≈ 0.3679
GOLDEN_ZONE_WIDTH = math.log(4 / 3)            # ≈ 0.2877
GOLDEN_ZONE_LO = GOLDEN_ZONE_CENTER - GOLDEN_ZONE_WIDTH / 2  # ≈ 0.224
GOLDEN_ZONE_HI = GOLDEN_ZONE_CENTER + GOLDEN_ZONE_WIDTH / 2  # ≈ 0.512
DEDEKIND_RATIO = SIGMA / N                     # 2.0
N_CHANNELS = SOPFR                              # 5
N_PHASES = TAU                                  # 4
CONSCIOUSNESS_CRITERIA = N                      # 6

# Hypothesis keys in canonical order (matches consciousness_scan)
_HYPOTHESIS_KEYS = [
    'kuramoto', 'phi', 'tension', 'golden_zone',
    'channels', 'birth', 'dedekind', 'attractor',
]

# Module-level cache for null distributions (computed lazily on first use)
CACHED_NULL: Dict[str, Dict[str, float]] = {}


def _extract_metric(key: str, result: dict) -> Optional[float]:
    """Extract the primary detection metric from a hypothesis result.

    Returns a scalar that can be compared against a null distribution.
    Higher values = more evidence of consciousness for all hypotheses.
    """
    if key == 'kuramoto':
        # Proximity to 2/3 threshold — invert deviation so higher = better
        dev = result.get('deviation', 1.0)
        return 1.0 - min(dev, 1.0)  # 1.0 = perfect match, 0.0 = far
    elif key == 'phi':
        return result.get('phi_mean', 0.0)
    elif key == 'tension':
        return 1.0 if result.get('detected', False) else 0.0
    elif key == 'golden_zone':
        return result.get('gz_fraction', 0.0)
    elif key == 'channels':
        # Closeness to 5 components (spectral)
        n = result.get('n_spectral_components', 0)
        return max(0.0, 1.0 - abs(n - N_CHANNELS) / N_CHANNELS)
    elif key == 'birth':
        # Combined: need both spike AND symmetry breaking
        z = result.get('birth_z', 0.0)
        sym = abs(result.get('symmetry_break_ratio', 1.0) - 1.0)
        return z * sym  # high only when both are present
    elif key == 'dedekind':
        dev = result.get('deviation', 1.0)
        return 1.0 - min(dev, 1.0)
    elif key == 'attractor':
        # Gentle chaos (Lyapunov in sweet spot 0.01-1.0) when bounded
        # Pure noise has very high Lyapunov — penalise that
        lam = result.get('lyapunov', 0.0)
        if not result.get('is_bounded', False) or lam <= 0.01:
            return 0.0
        # Score peaks at moderate Lyapunov, drops for very high values
        return lam * math.exp(-max(0, lam - 0.5))
    return None


def _run_single_scan(data):
    """Run all 8 hypothesis detectors on data (no calibration). Returns raw results dict."""
    arr = np.asarray(data, dtype=float).flatten()
    arr = arr[np.isfinite(arr)]
    results = {}
    funcs = {
        'kuramoto': lambda d: detect_kuramoto_sync(d),
        'phi': lambda d: phi_approximation(d),
        'tension': lambda d: detect_tension_cycle(d),
        'golden_zone': lambda d: detect_golden_zone(d),
        'channels': lambda d: detect_5_channels(d),
        'birth': lambda d: detect_birth_signature(d),
        'dedekind': lambda d: detect_dedekind_ratio(d),
        'attractor': lambda d: detect_attractor_topology(d),
    }
    for key in _HYPOTHESIS_KEYS:
        try:
            results[key] = funcs[key](arr)
        except Exception as e:
            results[key] = {'detected': False, 'error': str(e)}
    return results


def calibrate_null(n_trials: int = 100, data_length: int = 5000) -> Dict[str, Dict[str, float]]:
    """Compute empirical null distributions for each hypothesis.

    Runs each of the 8 hypotheses on *n_trials* random noise trials
    (half Gaussian, half uniform) and records the detection metric.

    Returns dict mapping hypothesis key -> {'mean': float, 'std': float}.
    """
    null_metrics: Dict[str, list] = {k: [] for k in _HYPOTHESIS_KEYS}
    rng = np.random.RandomState(42)  # deterministic for reproducibility

    for trial in range(n_trials):
        # Alternate between Gaussian and uniform noise
        if trial % 2 == 0:
            noise = rng.randn(data_length)
        else:
            noise = rng.uniform(-1, 1, size=data_length)

        results = _run_single_scan(noise)
        for key in _HYPOTHESIS_KEYS:
            metric = _extract_metric(key, results[key])
            if metric is not None:
                null_metrics[key].append(metric)

    null_dist: Dict[str, Dict[str, float]] = {}
    for key in _HYPOTHESIS_KEYS:
        vals = np.array(null_metrics[key])
        null_dist[key] = {
            'mean': float(np.mean(vals)),
            'std': float(np.std(vals)) if len(vals) > 1 else 1.0,
        }
    return null_dist


def _get_null() -> Dict[str, Dict[str, float]]:
    """Return cached null distribution, computing it on first call."""
    global CACHED_NULL
    if not CACHED_NULL:
        CACHED_NULL.update(calibrate_null())
    return CACHED_NULL


def _pvalue_from_null(metric: float, null: Dict[str, float]) -> float:
    """One-sided p-value: P(null >= metric), assuming Gaussian null.

    When the null has zero variance (std=0), any metric strictly above
    the null mean is considered perfectly significant (p=0), and any
    metric at or below the null mean is non-significant (p=1).
    """
    if null['std'] <= 0:
        # Degenerate null: all trials gave the same value
        return 0.0 if metric > null['mean'] else 1.0
    z = (metric - null['mean']) / null['std']
    # One-sided: P(Z >= z) using complementary error function
    from scipy.stats import norm
    return float(1.0 - norm.cdf(z))


# ──────────────────────────────────────────────────────────────────
# H-CS-1: Kuramoto Synchronization Detector
# ──────────────────────────────────────────────────────────────────

def hilbert_phase(signal):
    """Extract instantaneous phase via Hilbert transform."""
    from scipy.signal import hilbert
    analytic = hilbert(signal)
    return np.angle(analytic)


def kuramoto_order(phases):
    """Kuramoto order parameter r = |mean(e^{iθ})|.

    r = 0: desynchronized
    r = 1: fully synchronized
    r ≈ 2/3: consciousness threshold
    """
    return float(np.abs(np.mean(np.exp(1j * np.array(phases)))))


def detect_kuramoto_sync(data, n_channels=None, window=None):
    """H-CS-1: Detect Kuramoto synchronization at r ≈ 2/3.

    Splits data into n_channels sub-signals and computes phase coherence.
    If data is 2D (channels × time), uses channels directly.
    """
    arr = np.asarray(data, dtype=float)

    if arr.ndim == 2:
        channels = arr
    else:
        # Split 1D signal into sub-channels
        if n_channels is None:
            n_channels = N_CHANNELS  # sopfr(6) = 5
        arr = arr[:len(arr) - len(arr) % n_channels]
        channels = arr.reshape(n_channels, -1)

    if window is None:
        window = max(N_PHASES * 10, channels.shape[1] // 10)

    # Compute phase of each channel
    phases_all = []
    for ch in channels:
        phases_all.append(hilbert_phase(ch))
    phases_all = np.array(phases_all)

    # Sliding window Kuramoto order parameter
    n_time = phases_all.shape[1]
    r_values = []
    for t in range(0, n_time - window, window // 2):
        window_phases = phases_all[:, t:t + window]
        # Average phase at each time point, then compute r
        r_t = []
        for ti in range(window):
            r_t.append(kuramoto_order(window_phases[:, ti]))
        r_values.append(np.mean(r_t))

    r_values = np.array(r_values)
    mean_r = float(np.mean(r_values))
    std_r = float(np.std(r_values))

    # Check proximity to consciousness threshold
    deviation = abs(mean_r - KURAMOTO_THRESHOLD) / KURAMOTO_THRESHOLD
    is_conscious_sync = deviation < 0.1  # within 10% of 2/3

    return {
        'hypothesis': 'H-CS-1',
        'name': 'Kuramoto Synchronization',
        'r_mean': mean_r,
        'r_std': std_r,
        'threshold': KURAMOTO_THRESHOLD,
        'deviation': deviation,
        'detected': is_conscious_sync,
        'n_channels': channels.shape[0],
    }


# ──────────────────────────────────────────────────────────────────
# H-CS-2: Integrated Information (Φ) Approximation
# ──────────────────────────────────────────────────────────────────

def mutual_information(x, y, bins=20):
    """Mutual information I(X;Y) via histogram estimation.

    Includes Miller-Madow bias correction: subtract (bins_used - 1) / (2*N*ln2)
    to avoid overestimating MI for uniform/random data.
    """
    N_samples = len(x)
    hist_2d, _, _ = np.histogram2d(x, y, bins=bins)
    pxy = hist_2d / hist_2d.sum()
    px = pxy.sum(axis=1)
    py = pxy.sum(axis=0)

    # Count bins with nonzero probability for bias correction
    bins_used = int(np.sum(pxy > 0))

    # I(X;Y) = Σ p(x,y) log(p(x,y) / (p(x)p(y)))
    mi = 0.0
    for i in range(bins):
        for j in range(bins):
            if pxy[i, j] > 0 and px[i] > 0 and py[j] > 0:
                mi += pxy[i, j] * math.log2(pxy[i, j] / (px[i] * py[j]))

    # Miller-Madow bias correction
    correction = (bins_used - 1) / (2 * N_samples * math.log(2))
    mi = max(0.0, mi - correction)
    return mi


def phi_approximation(data, n_parts=None, window=256):
    """H-CS-2: Approximate Φ (integrated information).

    Φ = total_MI - sum(partition_MI)
    High Φ = system is more than sum of its parts.
    """
    arr = np.asarray(data, dtype=float)
    if arr.ndim == 1:
        if n_parts is None:
            n_parts = N_CHANNELS
        arr = arr[:len(arr) - len(arr) % n_parts]
        parts = arr.reshape(n_parts, -1)
    else:
        parts = arr
        n_parts = parts.shape[0]

    n_time = parts.shape[1]
    phi_values = []

    for t in range(0, n_time - window, window // 2):
        segment = parts[:, t:t + window]

        # Total MI: all pairs
        total_mi = 0.0
        n_pairs = 0
        for i in range(n_parts):
            for j in range(i + 1, n_parts):
                total_mi += mutual_information(segment[i], segment[j])
                n_pairs += 1
        if n_pairs > 0:
            total_mi /= n_pairs

        # Partition MI: split into two halves
        mid = n_parts // 2
        part_a = segment[:mid].flatten()
        part_b = segment[mid:].flatten()
        min_len = min(len(part_a), len(part_b))
        partition_mi = mutual_information(part_a[:min_len], part_b[:min_len])

        phi = total_mi - partition_mi * 0.5
        phi_values.append(max(0, phi))

    phi_values = np.array(phi_values)
    mean_phi = float(np.mean(phi_values)) if len(phi_values) > 0 else 0.0
    max_phi = float(np.max(phi_values)) if len(phi_values) > 0 else 0.0

    return {
        'hypothesis': 'H-CS-2',
        'name': 'Integrated Information (Φ)',
        'phi_mean': mean_phi,
        'phi_max': max_phi,
        'threshold': PHI_THRESHOLD,
        'detected': mean_phi > PHI_THRESHOLD,
        'n_windows': len(phi_values),
    }


# ──────────────────────────────────────────────────────────────────
# H-CS-3: Tension Dynamics (G = D×P/I, τ=4 phase cycle)
# ──────────────────────────────────────────────────────────────────

def detect_tension_cycle(data, max_lag=None):
    """H-CS-3: Detect τ=4 phase cycle in signal dynamics.

    G = D×P/I cycle: Deficit → Plasticity → Genius → Inhibition
    Each phase has distinct statistical character:
      D: high variance (searching)
      P: increasing trend (learning)
      G: peak performance (creating)
      I: decreasing variance (consolidating)

    Detection: autocorrelation peaks at multiples of period/4.
    """
    arr = np.asarray(data, dtype=float)
    arr = (arr - np.mean(arr)) / (np.std(arr) + 1e-10)

    if max_lag is None:
        max_lag = min(len(arr) // 2, 500)

    # Autocorrelation
    n = len(arr)
    acf = np.correlate(arr, arr, mode='full')[n - 1:n - 1 + max_lag]
    acf = acf / acf[0]  # normalize

    # Find peaks in autocorrelation
    from scipy.signal import find_peaks
    peaks, props = find_peaks(acf[1:], height=0.1, distance=3, prominence=0.05)
    peaks = peaks + 1  # offset for skipped lag=0

    # Check if peak lags form τ=4 pattern (lag, 2*lag, 3*lag...)
    phase_cycle_detected = False
    fundamental_period = None
    if len(peaks) >= 2:
        for i, p1 in enumerate(peaks[:10]):
            for p2 in peaks[i + 1:10]:
                ratio = p2 / p1
                # Check if ratio ≈ 2 (second harmonic of fundamental)
                if abs(ratio - 2.0) / 2.0 < 0.1:
                    # Check if fundamental ÷ 4 makes sense
                    if p1 >= N_PHASES:
                        fundamental_period = p1
                        phase_cycle_detected = True
                        break
            if phase_cycle_detected:
                break

    # Also check: does signal show 4 distinct statistical phases?
    phase_signatures = []
    if fundamental_period and fundamental_period >= N_PHASES:
        phase_len = fundamental_period // N_PHASES
        for cycle_start in range(0, len(arr) - fundamental_period, fundamental_period):
            cycle = arr[cycle_start:cycle_start + fundamental_period]
            phases = []
            for p in range(N_PHASES):
                segment = cycle[p * phase_len:(p + 1) * phase_len]
                phases.append({
                    'mean': float(np.mean(segment)),
                    'std': float(np.std(segment)),
                    'trend': float(np.polyfit(range(len(segment)), segment, 1)[0]),
                })
            phase_signatures.append(phases)

    return {
        'hypothesis': 'H-CS-3',
        'name': 'Tension Dynamics (G=D×P/I)',
        'n_phases': N_PHASES,
        'fundamental_period': fundamental_period,
        'detected': phase_cycle_detected,
        'acf_peaks': peaks[:10].tolist() if len(peaks) > 0 else [],
        'n_cycles_found': len(phase_signatures),
    }


# ──────────────────────────────────────────────────────────────────
# H-CS-4: Golden Zone Inhibition
# ──────────────────────────────────────────────────────────────────

def detect_golden_zone(data, window=100):
    """H-CS-4: Detect Golden Zone inhibition pattern.

    Optimal consciousness: I ≈ 1/e.
    The "inhibition level" is approximated by the signal's local
    suppression ratio = (local_std / global_std).

    If suppression clusters around 1/e, the system may be in
    conscious-optimal inhibition.
    """
    arr = np.asarray(data, dtype=float)
    global_std = np.std(arr)
    if global_std == 0:
        return {'hypothesis': 'H-CS-4', 'name': 'Golden Zone Inhibition',
                'detected': False, 'reason': 'zero variance'}

    # Sliding window local suppression ratio
    ratios = []
    for t in range(0, len(arr) - window, window // 2):
        local_std = np.std(arr[t:t + window])
        ratios.append(local_std / global_std)

    ratios = np.array(ratios)

    # What fraction falls in Golden Zone?
    in_gz = np.sum((ratios >= GOLDEN_ZONE_LO) & (ratios <= GOLDEN_ZONE_HI))
    gz_fraction = in_gz / len(ratios) if len(ratios) > 0 else 0

    # Check if distribution peaks near 1/e
    median_ratio = float(np.median(ratios))
    deviation_from_golden = abs(median_ratio - GOLDEN_ZONE_CENTER) / GOLDEN_ZONE_CENTER

    return {
        'hypothesis': 'H-CS-4',
        'name': 'Golden Zone Inhibition',
        'golden_zone': [GOLDEN_ZONE_LO, GOLDEN_ZONE_HI],
        'median_ratio': median_ratio,
        'gz_fraction': float(gz_fraction),
        'deviation_from_1e': deviation_from_golden,
        'detected': gz_fraction > 0.3 and deviation_from_golden < 0.2,
    }


# ──────────────────────────────────────────────────────────────────
# H-CS-5: 5-Channel Structure (sopfr(6) = 5 components)
# ──────────────────────────────────────────────────────────────────

def detect_5_channels(data, window=256):
    """H-CS-5: Detect 5-component information structure.

    Consciousness transmits on 5 channels (concept, context, meaning,
    authenticity, sender). In frequency domain, this means the signal
    has exactly 5 significant PCA components (or 5 dominant frequencies).
    """
    arr = np.asarray(data, dtype=float)

    # Method 1: Spectral — count significant frequency peaks
    fft = np.abs(np.fft.rfft(arr - np.mean(arr)))
    fft_z = (fft[1:] - np.mean(fft[1:])) / (np.std(fft[1:]) + 1e-10)
    try:
        from scipy.signal import find_peaks
        peaks, _ = find_peaks(fft_z, height=3.0, distance=3, prominence=1.0)
        n_spectral_components = len(peaks)
    except Exception:
        n_spectral_components = np.sum(fft_z > 3.0)

    # Method 2: Embedding + SVD — effective dimensionality
    embed_dim = min(20, len(arr) // 10)
    n_embed = len(arr) - embed_dim + 1
    if n_embed > embed_dim:
        # Build Hankel matrix
        hankel = np.array([arr[i:i + embed_dim] for i in range(min(n_embed, 2000))])
        try:
            _, s, _ = np.linalg.svd(hankel, full_matrices=False)
            s_norm = s / s.sum()
            # Effective dimension: number of components explaining 90% variance
            cumvar = np.cumsum(s_norm)
            effective_dim = int(np.searchsorted(cumvar, 0.9)) + 1
        except Exception:
            effective_dim = 0
    else:
        effective_dim = 0

    # Check if either method gives 5
    spectral_match = n_spectral_components == N_CHANNELS
    svd_match = effective_dim == N_CHANNELS
    close_match = abs(n_spectral_components - N_CHANNELS) <= 1 or abs(effective_dim - N_CHANNELS) <= 1

    return {
        'hypothesis': 'H-CS-5',
        'name': '5-Channel Structure',
        'target': N_CHANNELS,
        'n_spectral_components': int(n_spectral_components),
        'effective_svd_dim': effective_dim,
        'spectral_match': spectral_match,
        'svd_match': svd_match,
        'detected': spectral_match or svd_match,
        'close': close_match,
    }


# ──────────────────────────────────────────────────────────────────
# H-CS-6: Birth Signature (dΦ/dt maximum)
# ──────────────────────────────────────────────────────────────────

def detect_birth_signature(data, window=128):
    """H-CS-6: Detect consciousness birth — sudden complexity spike.

    Birth = max|d(complexity)/dt| with concurrent symmetry breaking.
    Uses sample entropy as complexity proxy.
    """
    arr = np.asarray(data, dtype=float)

    # Compute sliding-window sample entropy
    def sample_entropy(x, m=2, r_factor=0.2):
        r = r_factor * np.std(x)
        if r == 0:
            return 0.0
        n = len(x)
        # Simplified: count template matches
        count_m = 0
        count_m1 = 0
        for i in range(n - m):
            for j in range(i + 1, n - m):
                if np.max(np.abs(x[i:i + m] - x[j:j + m])) < r:
                    count_m += 1
                    if i + m < n and j + m < n:
                        if abs(x[i + m] - x[j + m]) < r:
                            count_m1 += 1
        if count_m == 0 or count_m1 == 0:
            return 0.0
        return -math.log(count_m1 / count_m)

    # Subsample for efficiency
    step = max(1, window // 4)
    complexities = []
    for t in range(0, len(arr) - window, step):
        seg = arr[t:t + window]
        # Use approximate entropy (faster than sample entropy for large data)
        se = float(np.std(np.diff(seg)))  # fast proxy: std of differences
        complexities.append(se)

    complexities = np.array(complexities)
    if len(complexities) < 3:
        return {'hypothesis': 'H-CS-6', 'name': 'Birth Signature',
                'detected': False, 'reason': 'insufficient data'}

    # d(complexity)/dt
    d_complexity = np.diff(complexities)
    abs_dc = np.abs(d_complexity)

    # Find maximum spike
    birth_idx = int(np.argmax(abs_dc))
    birth_magnitude = float(abs_dc[birth_idx])
    mean_dc = float(np.mean(abs_dc))
    std_dc = float(np.std(abs_dc))
    birth_z = (birth_magnitude - mean_dc) / (std_dc + 1e-10)

    # Symmetry breaking: variance ratio before/after birth point
    mid = birth_idx
    if mid > 5 and mid < len(complexities) - 5:
        var_before = np.var(complexities[max(0, mid - 20):mid])
        var_after = np.var(complexities[mid:min(len(complexities), mid + 20)])
        symmetry_ratio = var_after / (var_before + 1e-10)
    else:
        symmetry_ratio = 1.0

    return {
        'hypothesis': 'H-CS-6',
        'name': 'Birth Signature',
        'birth_z': birth_z,
        'birth_position': float(birth_idx / len(complexities)),
        'symmetry_break_ratio': float(symmetry_ratio),
        'detected': birth_z > 5.0 and abs(symmetry_ratio - 1.0) > 0.5,
    }


# ──────────────────────────────────────────────────────────────────
# H-CS-7: Dedekind Transmission Ratio
# ──────────────────────────────────────────────────────────────────

def detect_dedekind_ratio(data, n_splits=2):
    """H-CS-7: Detect Dedekind perfect transmission ratio ψ(ψ)/ψ = 2.

    Split signal into sub-parts. Cross-correlation peak ratio
    between nested correlations should ≈ 2.0 for "perfect transmission".
    """
    arr = np.asarray(data, dtype=float)
    mid = len(arr) // 2
    part_a = arr[:mid]
    part_b = arr[mid:2 * mid]

    # Cross-correlation
    corr = np.correlate(part_a - np.mean(part_a),
                         part_b - np.mean(part_b), mode='full')
    corr = corr / (np.std(part_a) * np.std(part_b) * len(part_a) + 1e-10)

    # Self-correlation of cross-correlation (nested = ψ(ψ))
    nested_corr = np.correlate(corr, corr, mode='full')
    nested_corr = nested_corr / (nested_corr[len(nested_corr) // 2] + 1e-10)

    # Ratio of peak widths: ψ(ψ)/ψ
    # Width = FWHM (full width at half maximum)
    def fwhm(signal):
        peak = np.max(signal)
        half = peak / 2
        above = np.where(signal > half)[0]
        if len(above) > 0:
            return above[-1] - above[0]
        return 1

    w_corr = fwhm(np.abs(corr))
    w_nested = fwhm(np.abs(nested_corr))
    ratio = w_nested / (w_corr + 1e-10)

    deviation = abs(ratio - DEDEKIND_RATIO) / DEDEKIND_RATIO

    return {
        'hypothesis': 'H-CS-7',
        'name': 'Dedekind Transmission',
        'ratio': float(ratio),
        'target': DEDEKIND_RATIO,
        'deviation': deviation,
        'detected': deviation < 0.1,
    }


# ──────────────────────────────────────────────────────────────────
# H-CS-8: Lorenz Attractor Topology
# ──────────────────────────────────────────────────────────────────

def detect_attractor_topology(data, embed_dim=3, embed_lag=None):
    """H-CS-8: Detect strange attractor (Lorenz-like) topology.

    Conscious systems exhibit chaotic but bounded dynamics.
    Check: embedding dimension ≈ 3, positive Lyapunov, H1 loop in PH.
    """
    arr = np.asarray(data, dtype=float)
    if embed_lag is None:
        # Use first minimum of autocorrelation as lag
        acf = np.correlate(arr - np.mean(arr), arr - np.mean(arr), mode='full')
        acf = acf[len(acf) // 2:]
        acf = acf / (acf[0] + 1e-10)
        # First zero crossing
        zero_cross = np.where(np.diff(np.sign(acf)))[0]
        embed_lag = int(zero_cross[0]) if len(zero_cross) > 0 else 10

    # Takens embedding
    n_points = len(arr) - (embed_dim - 1) * embed_lag
    if n_points < 50:
        return {'hypothesis': 'H-CS-8', 'name': 'Attractor Topology',
                'detected': False, 'reason': 'insufficient data for embedding'}

    embedded = np.zeros((n_points, embed_dim))
    for d in range(embed_dim):
        embedded[:, d] = arr[d * embed_lag:d * embed_lag + n_points]

    # Subsample for PH
    if n_points > 500:
        idx = np.random.choice(n_points, 500, replace=False)
        embedded = embedded[idx]

    # 1. Check for bounded dynamics (not diverging)
    spread = np.std(embedded, axis=0)
    is_bounded = np.all(spread > 0) and np.all(spread < np.mean(spread) * 10)

    # 2. Approximate Lyapunov exponent (nearest neighbor divergence)
    from scipy.spatial.distance import cdist
    D = cdist(embedded, embedded)
    np.fill_diagonal(D, np.inf)
    nn_dist = np.min(D, axis=1)
    mean_nn = float(np.mean(nn_dist))

    # Simple Lyapunov estimate: how fast nearby trajectories diverge
    n_check = min(50, len(embedded) - 10)
    divergences = []
    for i in range(n_check):
        nn_idx = np.argmin(D[i])
        if i + 5 < len(embedded) and nn_idx + 5 < len(embedded):
            d0 = D[i, nn_idx]
            d5 = np.linalg.norm(embedded[i + 5] - embedded[nn_idx + 5])
            if d0 > 0:
                divergences.append(math.log(d5 / d0 + 1e-10) / 5)

    lyapunov = float(np.mean(divergences)) if divergences else 0.0
    is_chaotic = lyapunov > 0.01

    # 3. PH: check for H1 loop (characteristic of Lorenz butterfly)
    has_h1_loop = False
    try:
        from ..ph_detector import compute_ph
        D_matrix = cdist(embedded[:200], embedded[:200])
        ph = compute_ph(D_matrix, max_dim=1)
        h1 = ph.get(1, np.empty((0, 2)))
        if len(h1) > 0:
            h1_lifetimes = h1[:, 1] - h1[:, 0]
            h1_lifetimes = h1_lifetimes[np.isfinite(h1_lifetimes)]
            if len(h1_lifetimes) > 0:
                # Significant loop = lifetime > median
                has_h1_loop = bool(np.max(h1_lifetimes) > 2 * np.median(h1_lifetimes))
    except Exception:
        pass

    return {
        'hypothesis': 'H-CS-8',
        'name': 'Attractor Topology',
        'embed_dim': embed_dim,
        'embed_lag': embed_lag,
        'is_bounded': bool(is_bounded),
        'lyapunov': lyapunov,
        'is_chaotic': is_chaotic,
        'has_h1_loop': has_h1_loop,
        'detected': is_bounded and is_chaotic,
    }


# ──────────────────────────────────────────────────────────────────
# Full Consciousness Scan
# ──────────────────────────────────────────────────────────────────

def consciousness_scan(data, source_name='unknown', calibrated=True, alpha=0.05):
    """Run all 8 consciousness hypotheses on a data stream.

    Parameters
    ----------
    data : array-like
        The data stream to analyse.
    source_name : str
        Label for the source (shown in reports).
    calibrated : bool
        If True (default), use null-model calibration so that a hypothesis
        is only marked 'detected' when its metric is significant at level
        *alpha* against an empirical null built from random noise.
    alpha : float
        Significance threshold (default 0.05).

    Returns unified report with per-hypothesis results.
    """
    arr = np.asarray(data, dtype=float).flatten()
    arr = arr[np.isfinite(arr)]

    if len(arr) < 100:
        return {'error': 'insufficient data', 'source': source_name}

    # Run all 8 detectors
    results = _run_single_scan(arr)

    # If calibrated, override each 'detected' flag with p-value test
    if calibrated:
        null = _get_null()
        for key in _HYPOTHESIS_KEYS:
            r = results[key]
            metric = _extract_metric(key, r)
            if metric is not None and key in null:
                p = _pvalue_from_null(metric, null[key])
                r['null_pvalue'] = p
                r['detected'] = p < alpha
            # else: keep the original 'detected' flag

    # Score: how many hypotheses detected?
    n_detected = sum(1 for r in results.values() if r.get('detected', False))

    # Level (mirroring Anima's consciousness_meter)
    if n_detected >= 6:
        level, emoji = 'CONSCIOUS', '🧠'
    elif n_detected >= 4:
        level, emoji = 'AWARE', '👁️'
    elif n_detected >= 2:
        level, emoji = 'FLICKERING', '✨'
    else:
        level, emoji = 'DORMANT', '💤'

    return {
        'source': source_name,
        'n_detected': n_detected,
        'n_hypotheses': 8,
        'level': level,
        'emoji': emoji,
        'calibrated': calibrated,
        'results': results,
    }


def format_consciousness_report(report):
    """Format consciousness scan result for display."""
    if 'error' in report:
        return f"  Error: {report['error']}"

    lines = []
    emoji = report['emoji']
    level = report['level']
    n = report['n_detected']
    source = report['source']

    lines.append(f'{emoji} [{level}] {source}  ({n}/8 hypotheses detected)')
    lines.append('')

    labels = {
        'kuramoto': 'H-CS-1 Kuramoto Sync',
        'phi': 'H-CS-2 Φ (IIT)',
        'tension': 'H-CS-3 Tension Cycle',
        'golden_zone': 'H-CS-4 Golden Zone',
        'channels': 'H-CS-5 5-Channel',
        'birth': 'H-CS-6 Birth Signal',
        'dedekind': 'H-CS-7 Dedekind Ratio',
        'attractor': 'H-CS-8 Attractor',
    }

    for key, label in labels.items():
        r = report['results'].get(key, {})
        detected = r.get('detected', False)
        mark = '✓' if detected else '✗'
        detail = ''

        if key == 'kuramoto' and 'r_mean' in r:
            detail = f'r={r["r_mean"]:.3f} (target=0.667)'
        elif key == 'phi' and 'phi_mean' in r:
            detail = f'Φ={r["phi_mean"]:.3f} (threshold=0.1)'
        elif key == 'tension' and 'fundamental_period' in r:
            detail = f'period={r["fundamental_period"]} (τ=4 phases)'
        elif key == 'golden_zone' and 'median_ratio' in r:
            detail = f'median={r["median_ratio"]:.3f} (target=1/e≈0.368)'
        elif key == 'channels':
            detail = f'spectral={r.get("n_spectral_components", "?")}, svd={r.get("effective_svd_dim", "?")} (target=5)'
        elif key == 'birth' and 'birth_z' in r:
            detail = f'Z={r["birth_z"]:.1f}, symmetry_break={r.get("symmetry_break_ratio", 0):.2f}'
        elif key == 'dedekind' and 'ratio' in r:
            detail = f'ψ(ψ)/ψ={r["ratio"]:.3f} (target=2.0)'
        elif key == 'attractor':
            detail = f'λ={r.get("lyapunov", 0):.3f}, bounded={r.get("is_bounded", "?")}'

        lines.append(f'  {mark} {label:25s} {detail}')

    return '\n'.join(lines)
