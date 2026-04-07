"""NRAO VLA/VLBA Archive — National Radio Astronomy Observatory.

Archive: https://data.nrao.edu/portal/
TAP: https://data.nrao.edu/portal/tap (if available)

The NRAO archive contains radio observations from VLA, VLBA, GBT, and ALMA.
We query for publicly available calibrator data and observation metadata.
"""
import json
import urllib.request
import urllib.parse
from typing import List, Dict, Optional

NRAO_ARCHIVE = "https://data.nrao.edu/portal"
NRAO_TAP = "https://data.nrao.edu/portal/tap/sync"

# Alternative: NRAO provides a VO-compatible query interface
NRAO_QUERY_URL = "https://data.nrao.edu/portal/api/query"

# Known VLA calibrator sources with flux densities (Jy at various bands)
# These are well-characterized and good for n=6 ratio analysis
VLA_CALIBRATORS = {
    '3C286': {'ra': 202.784, 'dec': 30.509, 'flux_1.4GHz': 14.65, 'flux_5GHz': 7.46,
              'flux_8.4GHz': 5.21, 'flux_15GHz': 3.44, 'flux_22GHz': 2.52, 'flux_43GHz': 1.45},
    '3C48':  {'ra': 24.422, 'dec': 33.160, 'flux_1.4GHz': 15.87, 'flux_5GHz': 5.48,
              'flux_8.4GHz': 3.25, 'flux_15GHz': 1.78, 'flux_22GHz': 1.13, 'flux_43GHz': 0.55},
    '3C147': {'ra': 85.651, 'dec': 49.852, 'flux_1.4GHz': 22.02, 'flux_5GHz': 7.93,
              'flux_8.4GHz': 4.80, 'flux_15GHz': 2.72, 'flux_22GHz': 1.77, 'flux_43GHz': 0.90},
    '3C138': {'ra': 80.291, 'dec': 16.639, 'flux_1.4GHz': 8.47, 'flux_5GHz': 3.79,
              'flux_8.4GHz': 2.47, 'flux_15GHz': 1.46, 'flux_22GHz': 1.00, 'flux_43GHz': 0.54},
}


def _fetch_json(url, timeout=30):
    """Fetch JSON from URL."""
    try:
        req = urllib.request.Request(url, headers={'User-Agent': 'SEDI/0.1'})
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            return json.loads(resp.read())
    except Exception as e:
        print(f"  [nrao] Fetch error: {e}")
    return None


def search_observations(target: str, radius_deg: float = 1.0) -> List[Dict]:
    """Search NRAO archive for observations of a target.

    Args:
        target: Source name (e.g., '3C286') or coordinates.
        radius_deg: Search radius in degrees.

    Returns:
        List of observation metadata dicts.
    """
    # Try the NRAO archive query API
    params = urllib.parse.urlencode({
        'target': target,
        'radius': radius_deg,
        'format': 'json',
    })
    url = f"{NRAO_QUERY_URL}?{params}"
    data = _fetch_json(url)
    if data and isinstance(data, list):
        return data

    # Fallback: return info about the target if it's a known calibrator
    print(f"  [nrao] Archive query unavailable, checking local calibrator data for '{target}'")
    if target in VLA_CALIBRATORS:
        cal = VLA_CALIBRATORS[target]
        return [{
            'target': target,
            'ra': cal['ra'],
            'dec': cal['dec'],
            'telescope': 'VLA',
            'status': 'calibrator — flux data available locally',
        }]

    return []


def fetch_calibrator_data() -> List[float]:
    """Fetch VLA calibrator flux density data for n=6 scanning.

    Returns flux densities across frequency bands for all primary calibrators.
    This provides a well-characterized dataset of spectral indices.
    """
    values = []
    for name, cal in VLA_CALIBRATORS.items():
        fluxes = [v for k, v in sorted(cal.items()) if k.startswith('flux_')]
        values.extend(fluxes)

    if not values:
        return []

    print(f"  [nrao] Loaded {len(values)} flux measurements from {len(VLA_CALIBRATORS)} calibrators")

    # Also compute spectral index ratios (flux ratios between bands)
    ratios = []
    for name, cal in VLA_CALIBRATORS.items():
        fluxes = [v for k, v in sorted(cal.items()) if k.startswith('flux_')]
        for i in range(len(fluxes) - 1):
            if fluxes[i + 1] > 0:
                ratios.append(fluxes[i] / fluxes[i + 1])

    values.extend(ratios)
    return values
