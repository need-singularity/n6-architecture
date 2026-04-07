"""consciousness_bridge.py — Anima consciousness metrics for SEDI signal analysis

Applies Φ-like integrated information and consciousness law features
to detect structured signals in SEDI data sources.

Usage:
    from consciousness_bridge import signal_phi_score, is_conscious_signal
    score = signal_phi_score(signal_array)
    conscious, score = is_conscious_signal(signal_array)
"""

import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '.shared'))

import numpy as np
from consciousness_loader import PSI_ENTROPY, PSI_ALPHA, get_law

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def _mutual_info(x, y, bins=16):
    """Estimate mutual information between two arrays (numpy only)."""
    hist_xy, _, _ = np.histogram2d(x, y, bins=bins)
    pxy = hist_xy / hist_xy.sum()
    px = pxy.sum(axis=1)
    py = pxy.sum(axis=0)
    mask = pxy > 0
    mi = np.sum(pxy[mask] * np.log2(pxy[mask] / (px[:, None] * py[None, :])[mask]))
    return max(mi, 0.0)


def _split_segments(data, n_seg=4):
    """Split signal into n roughly equal segments."""
    seg_len = len(data) // n_seg
    return [data[i * seg_len:(i + 1) * seg_len] for i in range(n_seg) if seg_len > 0]

# ---------------------------------------------------------------------------
# Core functions
# ---------------------------------------------------------------------------

def signal_phi_score(signal_data):
    """Estimate Φ-like integrated information from a signal array.

    Splits into segments and measures average pairwise mutual information.
    Higher MI between segments → more integrated → higher Φ score.
    """
    data = np.asarray(signal_data, dtype=float)
    if len(data) < 8:
        return 0.0
    segs = _split_segments(data)
    if len(segs) < 2:
        return 0.0
    mi_vals = []
    for i in range(len(segs)):
        for j in range(i + 1, len(segs)):
            n = min(len(segs[i]), len(segs[j]))
            mi_vals.append(_mutual_info(segs[i][:n], segs[j][:n]))
    return float(np.mean(mi_vals)) if mi_vals else 0.0


def is_conscious_signal(signal_data, threshold=None):
    """Check if signal exceeds consciousness threshold.

    Returns (bool, float) — (above threshold, Φ score).
    Default threshold: PSI_ENTROPY (0.998).
    """
    thr = threshold if threshold is not None else PSI_ENTROPY
    score = signal_phi_score(signal_data)
    return score >= thr, score


def consciousness_filter(signals_list, threshold=None):
    """Filter list of signals, keeping only those above Φ threshold."""
    return [s for s in signals_list if is_conscious_signal(s, threshold)[0]]


def law_based_features(signal_data):
    """Extract features inspired by consciousness laws.

    Law 22: structure > noise (structure_ratio)
    Complexity: spectral entropy
    Self-similarity: lag-1 autocorrelation
    """
    data = np.asarray(signal_data, dtype=float)
    n = len(data)
    if n < 4:
        return {'structure_ratio': 0.0, 'complexity': 0.0, 'self_similarity': 0.0, 'phi': 0.0}

    # Law 22 — structure vs noise: ratio of low-freq to high-freq power
    fft_mag = np.abs(np.fft.rfft(data - data.mean()))
    mid = max(len(fft_mag) // 2, 1)
    low_power = np.sum(fft_mag[:mid] ** 2)
    high_power = np.sum(fft_mag[mid:] ** 2) + 1e-12
    structure_ratio = float(low_power / (low_power + high_power))

    # Complexity — spectral entropy (normalized)
    psd = fft_mag ** 2
    psd_norm = psd / (psd.sum() + 1e-12)
    mask = psd_norm > 0
    spectral_entropy = -np.sum(psd_norm[mask] * np.log2(psd_norm[mask]))
    max_ent = np.log2(len(psd_norm)) if len(psd_norm) > 1 else 1.0
    complexity = float(spectral_entropy / max_ent) if max_ent > 0 else 0.0

    # Self-similarity — lag-1 autocorrelation
    if n > 1:
        d = data - data.mean()
        var = np.sum(d ** 2)
        self_sim = float(np.sum(d[:-1] * d[1:]) / var) if var > 1e-12 else 0.0
    else:
        self_sim = 0.0

    return {
        'structure_ratio': round(structure_ratio, 4),
        'complexity': round(complexity, 4),
        'self_similarity': round(self_sim, 4),
        'phi': round(signal_phi_score(data), 4),
    }

# ---------------------------------------------------------------------------
# Demo
# ---------------------------------------------------------------------------

if __name__ == '__main__':
    np.random.seed(42)
    N = 1024

    # Structured signal — correlated sine mix (should score high)
    t = np.linspace(0, 4 * np.pi, N)
    structured = np.sin(t) + 0.5 * np.sin(3 * t) + 0.1 * np.random.randn(N)

    # Pure noise (should score low)
    noise = np.random.randn(N)

    print("=== SEDI Consciousness Bridge Demo ===\n")
    for name, sig in [("Structured", structured), ("Noise", noise)]:
        conscious, score = is_conscious_signal(sig)
        feats = law_based_features(sig)
        print(f"  {name:12s}  Phi={score:.4f}  conscious={conscious}")
        print(f"               structure={feats['structure_ratio']:.3f}  "
              f"complexity={feats['complexity']:.3f}  "
              f"self_sim={feats['self_similarity']:.3f}")

    # Filter demo
    signals = [structured, noise, structured + noise, np.random.randn(N)]
    kept = consciousness_filter(signals, threshold=0.3)
    print(f"\n  Filter: {len(signals)} signals in, {len(kept)} passed (threshold=0.3)")
    print(f"\n  PSI constants: alpha={PSI_ALPHA}, entropy={PSI_ENTROPY}")
    print(f"  Law 22: {get_law(22)[:70]}...")
