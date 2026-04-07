"""Universal Signal Receiver — the PRIMARY detection engine of SEDI.

Detects ANY non-random signal in data streams using:
1. Persistent Homology (PH) — topological signal detection
2. Gravitational Lens — signal focusing and amplification
3. Spectral Analysis — frequency domain anomalies
4. Statistical Tests — deviation from null hypothesis

Philosophy: We don't need to know WHAT the signal is.
We only need to know it's NOT random noise.
PH barcode: noise = short bars (ephemeral), signal = long bars (persistent).
"""
import numpy as np
from scipy import stats
from typing import Dict, List, Optional


def _detect_distribution(data) -> str:
    """Detect likely distribution from data.

    Returns 'uniform' if kurtosis < -1 (flat), 'normal' otherwise.
    A true uniform distribution has kurtosis = -1.2 (excess kurtosis).
    """
    k = stats.kurtosis(data)
    if k < -1.0:
        return 'uniform'
    return 'normal'


class SignalReceiver:
    """Universal anomaly detector for arbitrary data streams."""

    def __init__(self, sensitivity=3.0, window=64, ph_enabled=True,
                 distribution='auto'):
        """
        Args:
            sensitivity: Z-score threshold for anomaly (default 3σ)
            window: sliding window size for analysis
            ph_enabled: use PH detection (requires gudhi/ripser)
            distribution: 'auto', 'normal', or 'uniform'.
                'auto'    — detect from calibration data (kurtosis < -1 → uniform)
                'normal'  — expect Gaussian noise; use Shapiro-Wilk
                'uniform' — expect uniform noise (e.g. quantum RNG uint16);
                            skip Shapiro-Wilk, use KS-uniform instead,
                            LOW entropy = signal, spectral PEAKS = signal
        """
        self.sensitivity = sensitivity
        self.window = window
        self.ph_enabled = ph_enabled
        self.distribution = distribution  # 'auto', 'normal', 'uniform'
        self._resolved_dist = None        # set after calibration or first receive
        self.baseline = None
        self.history = []

    def calibrate(self, noise_data):
        """Calibrate baseline from known noise/background data.

        Run this first with data you KNOW is just noise.
        """
        data = np.asarray(noise_data, dtype=float)
        self.baseline = {
            'mean': float(np.mean(data)),
            'std': float(np.std(data)),
            'median': float(np.median(data)),
            'skew': float(stats.skew(data)),
            'kurtosis': float(stats.kurtosis(data)),
            'n': len(data),
        }

        # Resolve distribution from calibration data if 'auto'
        if self.distribution == 'auto':
            self._resolved_dist = _detect_distribution(data)
        else:
            self._resolved_dist = self.distribution

        # PH baseline: compute barcode of noise
        if self.ph_enabled:
            self.baseline['ph'] = self._compute_ph_baseline(data)

        return self.baseline

    def _resolved_distribution(self, data) -> str:
        """Return resolved distribution, detecting from data if needed."""
        if self._resolved_dist is not None:
            return self._resolved_dist
        if self.distribution == 'auto':
            self._resolved_dist = _detect_distribution(data)
            return self._resolved_dist
        return self.distribution

    def receive(self, data) -> Dict:
        """Receive and analyze a data chunk.

        Returns dict with:
          - anomalies: list of detected anomalies
          - signal_strength: overall signal-to-noise ratio
          - ph_persistent: number of topologically persistent features
          - spectral_peaks: unusual frequency peaks
          - verdict: 'SIGNAL', 'WEAK', or 'NOISE'
        """
        data = np.asarray(data, dtype=float)
        dist = self._resolved_distribution(data)

        result = {
            'n': len(data),
            'anomalies': [],
            'signal_strength': 0.0,
            'verdict': 'NOISE',
            'distribution': dist,
        }

        # 1. Statistical anomaly detection
        stat_anomalies = self._statistical_test(data, dist)
        result['anomalies'].extend(stat_anomalies)

        # 2. Spectral anomaly detection
        spectral = self._spectral_test(data)
        result['spectral_peaks'] = spectral
        result['anomalies'].extend(spectral.get('anomalies', []))

        # 3. PH signal detection (the most powerful)
        if self.ph_enabled and len(data) >= self.window * 3:
            ph = self._ph_test(data)
            result['ph'] = ph
            result['anomalies'].extend(ph.get('anomalies', []))

        # 4. Autocorrelation (non-randomness)
        auto = self._autocorrelation_test(data)
        result['anomalies'].extend(auto)

        # 5. Entropy test
        entropy = self._entropy_test(data, dist)
        result['anomalies'].extend(entropy)

        # Compute overall signal strength
        if result['anomalies']:
            z_scores = [a['z_score'] for a in result['anomalies']]
            result['signal_strength'] = max(z_scores)
            if result['signal_strength'] >= self.sensitivity * 2:
                result['verdict'] = 'SIGNAL'
            elif result['signal_strength'] >= self.sensitivity:
                result['verdict'] = 'WEAK'

        self.history.append({
            'n': len(data),
            'signal_strength': result['signal_strength'],
            'n_anomalies': len(result['anomalies']),
            'verdict': result['verdict'],
        })

        return result

    def _statistical_test(self, data, dist: str) -> List[Dict]:
        """Test for statistical deviation from baseline or expected distribution."""
        anomalies = []

        if dist == 'uniform':
            # Kolmogorov-Smirnov test against U(lo, hi) where lo/hi come from
            # the calibration baseline (or data extremes if no baseline).
            # This catches signals that occupy only part of the expected range.
            if len(data) >= 8:
                if self.baseline:
                    # Reconstruct expected range from calibration mean ± 3*std
                    # (works reliably for uniform: mean = (lo+hi)/2, std = (hi-lo)/sqrt(12))
                    b_mean = self.baseline['mean']
                    b_std = self.baseline['std']
                    # For U(a,b): std = (b-a)/sqrt(12) => b-a = std*sqrt(12)
                    half_range = b_std * np.sqrt(12) / 2
                    lo = b_mean - half_range
                    hi = b_mean + half_range
                else:
                    lo, hi = float(np.min(data)), float(np.max(data))

                if hi > lo:
                    # Normalise data to [0,1] relative to expected full range
                    norm = (data - lo) / (hi - lo)
                    ks_stat, ks_p = stats.kstest(norm, 'uniform')
                    if ks_p < 0.001:
                        z = float(stats.norm.isf(max(ks_p, 1e-300)))
                        anomalies.append({
                            'type': 'non_uniform',
                            'test': 'ks-uniform',
                            'p_value': float(ks_p),
                            'z_score': z,
                            'detail': (f'Data deviates from uniform distribution '
                                       f'(KS p={ks_p:.2e})'),
                        })
        else:
            # Normal baseline: Shapiro-Wilk
            if 8 <= len(data) <= 5000:
                stat, p = stats.shapiro(data[:5000])
                if p < 0.001:
                    anomalies.append({
                        'type': 'non_normal',
                        'test': 'shapiro-wilk',
                        'p_value': float(p),
                        'z_score': float(stats.norm.isf(p)),
                        'detail': f'Data is not normally distributed (p={p:.2e})',
                    })

        # Runs test for randomness (distribution-agnostic)
        median = np.median(data)
        runs = self._count_runs(data > median)
        n = len(data)
        expected_runs = 1 + 2 * (n // 2) * ((n + 1) // 2) / n
        std_runs = np.sqrt(2 * (n // 2) * ((n + 1) // 2) * (2 * (n // 2) * ((n + 1) // 2) - n) /
                           (n ** 2 * (n - 1))) if n > 1 else 1
        if std_runs > 0:
            z_runs = abs(runs - expected_runs) / std_runs
            if z_runs > self.sensitivity:
                anomalies.append({
                    'type': 'non_random',
                    'test': 'runs',
                    'z_score': float(z_runs),
                    'detail': f'Non-random pattern detected (runs={runs}, expected={expected_runs:.0f})',
                })

        # Deviation from baseline
        if self.baseline:
            z_mean = abs(np.mean(data) - self.baseline['mean']) / max(self.baseline['std'], 1e-10)
            if z_mean > self.sensitivity:
                anomalies.append({
                    'type': 'mean_shift',
                    'test': 'baseline',
                    'z_score': float(z_mean),
                    'detail': f'Mean shifted from baseline by {z_mean:.1f}σ',
                })

            # Kurtosis change
            k = stats.kurtosis(data)
            k_diff = abs(k - self.baseline['kurtosis'])
            if k_diff > 2:
                anomalies.append({
                    'type': 'kurtosis_change',
                    'test': 'baseline',
                    'z_score': float(k_diff),
                    'detail': f'Kurtosis changed by {k_diff:.1f} from baseline',
                })

        return anomalies

    def _spectral_test(self, data) -> Dict:
        """FFT-based spectral anomaly detection.

        Under the null hypothesis of white noise, each |FFT[k]|^2 follows an
        exponential distribution with mean = overall mean power.  A simple
        Z-score against the Gaussian approximation produces a very high false-
        positive rate because the *maximum* of N exponential variables is
        expected to be O(log N) standard deviations above the mean.

        Fix: use per-bin exponential p-values with Bonferroni correction
        (alpha=0.001 / N_bins).  This gives a near-zero false-positive rate
        for pure noise while still detecting narrow spectral peaks easily.
        """
        result = {'anomalies': [], 'peaks': []}

        if len(data) < 16:
            return result

        fft = np.fft.rfft(data)
        power = np.abs(fft) ** 2
        freqs = np.fft.rfftfreq(len(data))

        # Skip DC component
        power = power[1:]
        freqs = freqs[1:]

        if len(power) == 0:
            return result

        # Under null, power[k] ~ Exp(mean_power)
        # p-value per bin: P(X >= x) = exp(-x / mean_power)
        mean_power = float(np.mean(power))
        if mean_power <= 0:
            return result

        N_bins = len(power)
        # Bonferroni threshold: reject if p < alpha/N (alpha=0.001)
        bonferroni_threshold = 0.001 / N_bins

        for i in range(N_bins):
            p_val = float(np.exp(-power[i] / mean_power))
            if p_val < bonferroni_threshold:
                # Convert to a Z-score for consistent reporting
                z = float(stats.norm.isf(max(p_val, 1e-300)))
                result['peaks'].append({
                    'freq': float(freqs[i]),
                    'power': float(power[i]),
                    'p_value': p_val,
                    'z_score': z,
                })

        if result['peaks']:
            best = max(result['peaks'], key=lambda p: p['z_score'])
            result['anomalies'].append({
                'type': 'spectral_peak',
                'test': 'fft',
                'z_score': best['z_score'],
                'detail': (f'Spectral peak at f={best["freq"]:.4f} '
                           f'(p={best["p_value"]:.2e}, Bonferroni-corrected)'),
            })

        return result

    def _ph_test(self, data) -> Dict:
        """Persistent Homology signal detection.

        KEY INSIGHT: Random noise produces only short-lived (ephemeral)
        PH features. A real signal creates PERSISTENT features — long
        bars in the barcode that survive across scales.

        False-positive hardening:
          - PH threshold raised from 2σ to 3σ (stricter persistence gate)
          - H1 loops require at least 2 significant features (not just 1)
        """
        result = {'anomalies': [], 'n_persistent': 0, 'max_lifetime': 0}

        try:
            from .ph_detector import sliding_window_embedding, compute_ph
            from scipy.spatial.distance import pdist, squareform
        except ImportError:
            return result

        # Takens embedding
        points = sliding_window_embedding(data, window=min(self.window, len(data) // 3))
        if len(points) < 10:
            return result

        # Subsample for speed
        max_pts = 150
        if len(points) > max_pts:
            idx = np.random.choice(len(points), max_pts, replace=False)
            points = points[idx]

        D = squareform(pdist(points))
        ph = compute_ph(D, max_dim=1)

        if not ph:
            return result

        # Analyze H0 (connected components)
        h0 = ph.get(0, np.empty((0, 2)))
        h0_finite = h0[h0[:, 1] < np.inf] if len(h0) > 0 else np.empty((0, 2))
        h0_lifetimes = h0_finite[:, 1] - h0_finite[:, 0] if len(h0_finite) > 0 else np.array([])

        # Analyze H1 (loops/holes)
        h1 = ph.get(1, np.empty((0, 2)))
        h1_lifetimes = h1[:, 1] - h1[:, 0] if len(h1) > 0 else np.array([])

        # Signal detection: persistent features >> noise features
        all_lifetimes = np.concatenate([h0_lifetimes, h1_lifetimes]) if len(h0_lifetimes) + len(h1_lifetimes) > 0 else np.array([])

        if len(all_lifetimes) > 3:
            median_lt = np.median(all_lifetimes)
            std_lt = np.std(all_lifetimes)
            # Raised threshold: 3σ (was 2σ) — reduces false positives for noise
            threshold = median_lt + 3 * std_lt
            persistent = np.sum(all_lifetimes > threshold)
            result['n_persistent'] = int(persistent)
            result['max_lifetime'] = float(np.max(all_lifetimes))

            if persistent > 0:
                # How unusual is this?
                z = persistent / max(np.sqrt(len(all_lifetimes) * 0.05), 1)  # expect ~5% above 2σ
                result['anomalies'].append({
                    'type': 'ph_persistent',
                    'test': 'topology',
                    'z_score': float(z),
                    'detail': f'{persistent} persistent features (H0+H1), max lifetime={result["max_lifetime"]:.3f}',
                })

            # H1 loops: require at least 2 significant features (was 1)
            if len(h1_lifetimes) > 0:
                significant_h1 = np.sum(h1_lifetimes > median_lt + std_lt)
                if significant_h1 >= 2:
                    result['anomalies'].append({
                        'type': 'ph_loops',
                        'test': 'topology',
                        'z_score': float(significant_h1 * 2),
                        'detail': f'{significant_h1} topological loops detected (strong signal indicator)',
                    })

        return result

    def _autocorrelation_test(self, data) -> List[Dict]:
        """Test for autocorrelation (non-independence)."""
        anomalies = []
        if len(data) < 20:
            return anomalies

        # Lag-1 autocorrelation
        n = len(data)
        mean = np.mean(data)
        denom = np.sum((data - mean) ** 2)
        if denom == 0:
            return anomalies
        lag1 = np.sum((data[:-1] - mean) * (data[1:] - mean)) / denom
        # Under null: lag1 ~ N(-1/n, 1/n)
        z_lag = abs(lag1 + 1/n) / (1/np.sqrt(n))
        if z_lag > self.sensitivity:
            anomalies.append({
                'type': 'autocorrelation',
                'test': 'lag1',
                'z_score': float(z_lag),
                'detail': f'Significant lag-1 autocorrelation r={lag1:.4f} (Z={z_lag:.1f})',
            })

        return anomalies

    def _entropy_test(self, data, dist: str) -> List[Dict]:
        """Test for entropy deviation.

        For uniform sources (dist='uniform'):
            A uniform distribution HAS maximum entropy — that is the null.
            LOW entropy means the data has become structured → signal.

        For normal sources:
            Very low entropy also indicates structure → signal.
        """
        anomalies = []
        if len(data) < 20:
            return anomalies

        # Binned entropy.
        # For uniform sources, bin over the EXPECTED full range (from calibration)
        # so that a signal occupying only a sub-range shows up as low entropy.
        n_bins = min(int(np.sqrt(len(data))), 50)

        if dist == 'uniform' and self.baseline:
            b_mean = self.baseline['mean']
            b_std = self.baseline['std']
            half_range = b_std * np.sqrt(12) / 2
            hist_range = (b_mean - half_range, b_mean + half_range)
            hist, _ = np.histogram(data, bins=n_bins, range=hist_range)
        else:
            hist, _ = np.histogram(data, bins=n_bins)

        hist_nonzero = hist[hist > 0]
        probs = hist_nonzero / hist_nonzero.sum()
        entropy = -np.sum(probs * np.log(probs))
        max_entropy = np.log(n_bins)

        # Normalized entropy (0=deterministic, 1=uniform/max)
        norm_entropy = entropy / max_entropy if max_entropy > 0 else 1.0

        if dist == 'uniform':
            # Uniform noise ≈ max entropy (norm_entropy ~ 1.0).
            # Deviation BELOW a threshold flags a signal.
            if norm_entropy < 0.85:
                z = (0.95 - norm_entropy) / 0.05  # rough Z-score
                anomalies.append({
                    'type': 'low_entropy',
                    'test': 'entropy',
                    'z_score': float(z),
                    'detail': (f'Entropy below uniform baseline '
                               f'H={norm_entropy:.3f} (expected ~1.0 for uniform)'),
                })
        else:
            # Normal baseline: very low entropy = highly structured signal
            if norm_entropy < 0.5:
                z = (0.8 - norm_entropy) / 0.1  # rough Z-score
                anomalies.append({
                    'type': 'low_entropy',
                    'test': 'entropy',
                    'z_score': float(z),
                    'detail': f'Low entropy H={norm_entropy:.3f} (structured signal)',
                })

        return anomalies

    @staticmethod
    def _count_runs(binary_seq):
        """Count number of runs in a binary sequence."""
        runs = 1
        for i in range(1, len(binary_seq)):
            if binary_seq[i] != binary_seq[i-1]:
                runs += 1
        return runs

    def _compute_ph_baseline(self, data):
        """Compute PH baseline statistics from noise."""
        try:
            from .ph_detector import sliding_window_embedding, compute_ph
            from scipy.spatial.distance import pdist, squareform

            points = sliding_window_embedding(data[:500], window=min(self.window, len(data) // 3))
            if len(points) > 100:
                points = points[np.random.choice(len(points), 100, replace=False)]
            D = squareform(pdist(points))
            ph = compute_ph(D, max_dim=0)
            h0 = ph.get(0, np.empty((0, 2)))
            h0_finite = h0[h0[:, 1] < np.inf] if len(h0) > 0 else np.empty((0, 2))
            lifetimes = h0_finite[:, 1] - h0_finite[:, 0] if len(h0_finite) > 0 else np.array([0])
            return {
                'mean_lifetime': float(np.mean(lifetimes)),
                'std_lifetime': float(np.std(lifetimes)),
                'n_bars': len(lifetimes),
            }
        except Exception:
            return {}
