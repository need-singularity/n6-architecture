"""Persistent Homology anomaly detector.

Uses gudhi (fast) or ripser (fallback) for topological signal analysis.
Detects n=6 patterns in PH barcodes of data streams.
"""
import numpy as np
from .constants import N, SIGMA, TAU, PHI, SOPFR, DELTA_PLUS, DELTA_MINUS

try:
    import gudhi
    HAS_GUDHI = True
except ImportError:
    HAS_GUDHI = False

try:
    from ripser import ripser
    HAS_RIPSER = True
except ImportError:
    HAS_RIPSER = False


def compute_ph(distance_matrix, max_dim=1):
    """Compute persistent homology from distance matrix.

    Uses gudhi.SimplexTree.create_from_array (fastest) or ripser fallback.
    """
    D = np.asarray(distance_matrix, dtype=np.float64)
    np.fill_diagonal(D, 0)

    if HAS_GUDHI:
        st = gudhi.SimplexTree.create_from_array(D)
        st.persistence()
        result = {}
        for dim in range(max_dim + 1):
            intervals = np.array(st.persistence_intervals_in_dimension(dim))
            result[dim] = intervals if len(intervals) > 0 else np.empty((0, 2))
        return result

    elif HAS_RIPSER:
        rips = ripser(D, maxdim=max_dim, distance_matrix=True)
        return {dim: rips['dgms'][dim] for dim in range(max_dim + 1)}

    else:
        print("  [ph] gudhi or ripser required: pip install gudhi")
        return {}


def sliding_window_embedding(data, window=6, stride=1):
    """Takens embedding: convert 1D time series to point cloud.

    Default window = n = 6.
    """
    data = np.asarray(data)
    n = len(data)
    points = []
    for i in range(0, n - window + 1, stride):
        points.append(data[i:i + window])
    return np.array(points)


def ph_barcode_anomaly(data, window=None):
    """Detect n=6 patterns in PH barcode of time series.

    1. Embed data using Takens window (default n=6)
    2. Compute distance matrix
    3. Run PH
    4. Check barcode for n=6 signatures
    """
    if window is None:
        window = N  # default to n=6

    data = np.asarray(data, dtype=float)
    if len(data) < window * 3:
        return {'error': 'insufficient data', 'n': len(data)}

    # Subsample if too large (PH is O(n^3))
    max_points = 200
    points = sliding_window_embedding(data, window=window)
    if len(points) > max_points:
        idx = np.random.choice(len(points), max_points, replace=False)
        points = points[idx]

    # Distance matrix (Euclidean)
    from scipy.spatial.distance import pdist, squareform
    D = squareform(pdist(points))

    # Compute PH
    ph = compute_ph(D, max_dim=1)
    if not ph:
        return {'error': 'PH computation failed'}

    # Analyze H0
    h0 = ph.get(0, np.empty((0, 2)))
    h0_finite = h0[h0[:, 1] < np.inf] if len(h0) > 0 else np.empty((0, 2))
    h0_lifetimes = h0_finite[:, 1] - h0_finite[:, 0] if len(h0_finite) > 0 else np.array([])

    # Analyze H1
    h1 = ph.get(1, np.empty((0, 2)))
    h1_lifetimes = h1[:, 1] - h1[:, 0] if len(h1) > 0 else np.array([])

    # Check for n=6 patterns in lifetimes
    anomalies = []

    # Pattern 1: lifetime ratios matching delta+/delta-
    if len(h0_lifetimes) >= 2:
        sorted_lt = np.sort(h0_lifetimes)[::-1]
        for i in range(len(sorted_lt) - 1):
            ratio = sorted_lt[i + 1] / sorted_lt[i] if sorted_lt[i] > 0 else 0
            if abs(ratio - DELTA_PLUS / DELTA_MINUS) < 0.05:  # 1/6 / 1/4 = 2/3
                anomalies.append({
                    'type': 'lifetime_ratio',
                    'value': ratio,
                    'target': DELTA_PLUS / DELTA_MINUS,
                    'bars': (i, i + 1),
                })

    # Pattern 2: number of significant H0 bars = sigma/tau or tau
    significant = np.sum(h0_lifetimes > np.median(h0_lifetimes)) if len(h0_lifetimes) > 0 else 0
    if significant == SIGMA // TAU:
        anomalies.append({'type': 'h0_count', 'value': significant, 'target': 'sigma/tau'})
    elif significant == TAU:
        anomalies.append({'type': 'h0_count', 'value': significant, 'target': 'tau'})

    # Pattern 3: H1 loop count = phi
    if len(h1_lifetimes) > 0:
        significant_h1 = np.sum(h1_lifetimes > np.median(h1_lifetimes))
        if significant_h1 == PHI:
            anomalies.append({'type': 'h1_count', 'value': significant_h1, 'target': 'phi'})

    return {
        'h0_bars': len(h0_lifetimes),
        'h1_bars': len(h1_lifetimes),
        'h0_total_lifetime': float(np.sum(h0_lifetimes)) if len(h0_lifetimes) > 0 else 0,
        'h1_total_lifetime': float(np.sum(h1_lifetimes)) if len(h1_lifetimes) > 0 else 0,
        'anomalies': anomalies,
        'window': window,
        'n_points': len(points),
    }
