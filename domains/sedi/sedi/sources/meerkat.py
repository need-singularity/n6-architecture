"""MeerKAT Radio Telescope Archive — SARAO.

Archive: https://archive.sarao.ac.za/
Python library: katdal (optional, may not be installed)

MeerKAT is a 64-dish radio interferometer in South Africa, part of
the Square Kilometre Array (SKA) precursor program. It produces
high-sensitivity radio continuum and spectral line data.
"""
import json
import urllib.request
from typing import List, Dict, Optional

SARAO_ARCHIVE = "https://archive.sarao.ac.za"
SARAO_API = "https://archive.sarao.ac.za/api"

# Known MeerKAT survey programs with public data
MEERKAT_SURVEYS = {
    'MIGHTEE': {
        'name': 'MeerKAT International GHz Tiered Extragalactic Exploration',
        'description': 'Deep continuum and spectral line survey',
        'freq_range_mhz': (900, 1670),
        'fields': ['COSMOS', 'XMM-LSS', 'CDFS', 'ELAIS-S1'],
        'status': 'ongoing',
    },
    'LADUMA': {
        'name': 'Looking at the Distant Universe with the MeerKAT Array',
        'description': 'Deep HI survey',
        'freq_range_mhz': (900, 1420),
        'fields': ['ECDF-S'],
        'status': 'ongoing',
    },
    'MHONGOOSE': {
        'name': 'MeerKAT HI Observations of Nearby Galactic Objects',
        'description': 'Nearby galaxy HI survey',
        'freq_range_mhz': (900, 1420),
        'fields': ['30 nearby galaxies'],
        'status': 'ongoing',
    },
    'MeerKLASS': {
        'name': 'MeerKAT Large Area Synoptic Survey',
        'description': 'HI intensity mapping',
        'freq_range_mhz': (900, 1420),
        'fields': ['Wide area'],
        'status': 'ongoing',
    },
    'ThunderKAT': {
        'name': 'The Hunt for Dynamic and Explosive Radio Transients',
        'description': 'Radio transient survey',
        'freq_range_mhz': (900, 1670),
        'fields': ['Various'],
        'status': 'ongoing',
    },
}

# Reference data: MeerKAT system parameters
MEERKAT_SPECS = {
    'n_dishes': 64,
    'dish_diameter_m': 13.5,
    'baseline_max_m': 7700,
    'freq_range_mhz': (580, 3500),
    'bands': {
        'UHF': (580, 1015),
        'L': (900, 1670),
        'S': (1750, 3500),
    },
    'system_temp_K': 18,  # at L-band
    'sensitivity_m2_K': 220,
}


def _fetch_json(url, timeout=30):
    """Fetch JSON from URL."""
    try:
        req = urllib.request.Request(url, headers={'User-Agent': 'SEDI/0.1'})
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            return json.loads(resp.read())
    except Exception as e:
        print(f"  [meerkat] Fetch error: {e}")
    return None


def search_public_data(survey: str = None) -> List[Dict]:
    """Search MeerKAT archive for public data.

    Args:
        survey: Optional survey name filter (e.g., 'MIGHTEE').

    Returns:
        List of available data products.
    """
    # Try SARAO archive API
    url = f"{SARAO_API}/public"
    if survey:
        url += f"?survey={survey}"
    data = _fetch_json(url)
    if data and isinstance(data, list):
        return data

    # Fallback: return survey catalog info
    print("  [meerkat] Archive API unavailable, returning survey metadata")
    results = []
    for key, info in MEERKAT_SURVEYS.items():
        if survey and key != survey:
            continue
        results.append({
            'survey': key,
            **info,
        })
    return results


def list_survey_releases() -> List[Dict]:
    """List publicly released MeerKAT survey data products.

    Returns:
        List of data release metadata.
    """
    data = _fetch_json(f"{SARAO_API}/releases")
    if data and isinstance(data, list):
        return data

    print("  [meerkat] Using known survey release info")
    return [
        {'survey': k, 'name': v['name'], 'status': v['status']}
        for k, v in MEERKAT_SURVEYS.items()
    ]


def get_system_parameters() -> List[float]:
    """Return MeerKAT system parameters as numeric array for scanning.

    Includes dish count, frequencies, baselines, sensitivity values.
    """
    values = [
        float(MEERKAT_SPECS['n_dishes']),            # 64
        float(MEERKAT_SPECS['dish_diameter_m']),      # 13.5
        float(MEERKAT_SPECS['baseline_max_m']),       # 7700
        float(MEERKAT_SPECS['system_temp_K']),        # 18
        float(MEERKAT_SPECS['sensitivity_m2_K']),     # 220
    ]
    # Add band frequency boundaries
    for band, (lo, hi) in MEERKAT_SPECS['bands'].items():
        values.extend([float(lo), float(hi)])

    # Add survey frequency ranges
    for key, info in MEERKAT_SURVEYS.items():
        lo, hi = info['freq_range_mhz']
        values.extend([float(lo), float(hi)])

    print(f"  [meerkat] Loaded {len(values)} system parameter values")
    return values
