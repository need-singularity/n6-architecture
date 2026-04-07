"""PICTOR Radio Telescope — open-source hydrogen line radio telescope.

URL: https://pictortelescope.com/
Data: PICTOR provides radio astronomy observations, primarily 21cm hydrogen line.
No formal REST API — data may be available via direct download or web scraping.

We attempt to fetch any publicly available observation data for n=6 scanning.
"""
import json
import urllib.request
from typing import List, Dict, Optional

PICTOR_BASE = "https://pictortelescope.com"
# Known data endpoints (may change as the project evolves)
PICTOR_DATA_URL = "https://pictortelescope.com/obs_data"
PICTOR_API_URL = "https://pictortelescope.com/api"


def _fetch_json(url, timeout=15):
    """Fetch JSON from a URL with error handling."""
    try:
        req = urllib.request.Request(url, headers={'User-Agent': 'SEDI/0.1'})
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            return json.loads(resp.read())
    except Exception as e:
        print(f"  [pictor] Fetch error ({url}): {e}")
    return None


def list_observations() -> List[Dict]:
    """List available observations from PICTOR.

    PICTOR is a community telescope; data availability varies.
    Returns list of observation metadata dicts, or hardcoded fallback.
    """
    # Try API endpoint first
    data = _fetch_json(f"{PICTOR_API_URL}/observations")
    if data and isinstance(data, list):
        return data

    # Fallback: return documented observation parameters
    # PICTOR observes the hydrogen 21cm line at 1420.405 MHz
    print("  [pictor] No live API available, using reference observation parameters")
    return [
        {
            'id': 'h1_ref',
            'description': 'Hydrogen 21cm line reference observation',
            'freq_mhz': 1420.405,
            'bandwidth_mhz': 2.4,
            'integration_s': 60,
            'source': 'pictor-reference',
        }
    ]


def fetch_observation(obs_id: str = 'h1_ref') -> Optional[List[float]]:
    """Fetch observation data from PICTOR.

    Args:
        obs_id: Observation identifier.

    Returns:
        List of float values (spectrum amplitudes) suitable for np.array(),
        or reference hydrogen line data if API is unavailable.
    """
    # Try live data fetch
    data = _fetch_json(f"{PICTOR_API_URL}/observation/{obs_id}")
    if data and 'spectrum' in data:
        return [float(v) for v in data['spectrum']]

    # Fallback: generate reference 21cm hydrogen line profile
    # Based on typical PICTOR observation characteristics:
    # - Center frequency: 1420.405 MHz
    # - Bandwidth: ~2.4 MHz
    # - Channels: 256
    # - Gaussian emission profile with typical parameters
    import numpy as np
    print("  [pictor] Using synthetic hydrogen line profile for scanning")
    n_channels = 256
    freqs = np.linspace(1419.2, 1421.6, n_channels)
    center = 1420.405  # MHz — hydrogen 21cm rest frequency
    # Typical HI emission line (Gaussian + noise baseline)
    profile = 10.0 * np.exp(-0.5 * ((freqs - center) / 0.15) ** 2)
    noise = np.random.RandomState(42).normal(0, 0.3, n_channels)
    spectrum = profile + noise + 1.0  # baseline offset
    return spectrum.tolist()
