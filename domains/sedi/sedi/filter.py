"""R-filter: core signal processing tuned to n=6."""
import numpy as np
from .constants import WINDOWS, RATIOS, DELTA_PLUS, DELTA_MINUS, FOCAL


def windowed_fft(data, window_size):
    """FFT at n=6 window sizes."""
    n = len(data)
    if n < window_size:
        return np.array([])
    # Sliding window FFT
    num_windows = n - window_size + 1
    magnitudes = []
    for i in range(0, num_windows, max(1, window_size // 2)):
        segment = data[i:i + window_size]
        fft = np.fft.rfft(segment)
        magnitudes.append(np.abs(fft))
    return np.mean(magnitudes, axis=0) if magnitudes else np.array([])


def detect_ratios(data, tolerance=0.02):
    """Detect n=6 ratios in data stream."""
    hits = []
    n = len(data)
    if n < 2:
        return hits

    # Consecutive ratios
    ratios = data[1:] / np.where(data[:-1] != 0, data[:-1], 1)

    for name, target in RATIOS.items():
        if target == 0:
            continue
        matches = np.where(np.abs(ratios - target) / target < tolerance)[0]
        if len(matches) > 0:
            expected = n * tolerance * 2  # rough expected by chance
            z_score = (len(matches) - expected) / max(np.sqrt(expected), 1)
            hits.append({
                'pattern': name,
                'target': target,
                'count': len(matches),
                'expected': expected,
                'z_score': z_score,
                'positions': matches[:10].tolist(),
            })

    return hits


def spectral_peaks(data):
    """Find peaks in FFT at n=6 frequencies."""
    results = {}
    for ws in WINDOWS:
        spectrum = windowed_fft(data, ws)
        if len(spectrum) == 0:
            continue
        # Normalize
        spectrum = spectrum / (np.max(spectrum) + 1e-10)
        # Check for peaks at 1/6, 1/4, 1/3, 1/2 of Nyquist
        nyquist = len(spectrum)
        for name, ratio in [('1/6', 1/6), ('1/4', 1/4), ('1/3', 1/3), ('1/2', 1/2)]:
            idx = int(ratio * nyquist)
            if 0 < idx < nyquist:
                peak_val = spectrum[idx]
                # Compare to neighborhood
                lo = max(0, idx - 2)
                hi = min(nyquist, idx + 3)
                neighborhood = np.mean(spectrum[lo:hi])
                if peak_val > 2 * neighborhood:
                    results[f'w{ws}_{name}'] = {
                        'window': ws,
                        'freq_ratio': ratio,
                        'peak': float(peak_val),
                        'background': float(neighborhood),
                        'snr': float(peak_val / max(neighborhood, 1e-10)),
                    }
    return results


def r_filter(data):
    """Complete R-filter: ratios + spectral peaks."""
    data = np.asarray(data, dtype=float)
    return {
        'ratio_hits': detect_ratios(data),
        'spectral_peaks': spectral_peaks(data),
        'stats': {
            'n': len(data),
            'mean': float(np.mean(data)),
            'std': float(np.std(data)),
            'focal_check': float(np.std(data) * FOCAL),
        }
    }
