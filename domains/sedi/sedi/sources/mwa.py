"""Murchison Widefield Array (MWA) — low-frequency radio telescope.

Data: https://www.mwatelescope.org/telescope/data/
Archive: https://asvo.mwatelescope.org/

MWA operates at 70-300 MHz from Western Australia. Low-frequency radio
observations are interesting for SETI — less explored frequency range
and could contain broadband technosignatures.
"""
import json
import urllib.request
from typing import List, Dict, Optional

MWA_BASE = "https://www.mwatelescope.org"
MWA_ASVO = "https://asvo.mwatelescope.org/api"
MWA_METADATA = "http://ws.mwatelescope.org/metadata"

# MWA system parameters and known survey data
MWA_SPECS = {
    'n_tiles': 128,          # 128 aperture array tiles (Phase II)
    'n_dipoles_per_tile': 16,  # 4x4 crossed dipoles
    'freq_range_mhz': (70, 300),
    'bandwidth_mhz': 30.72,
    'n_channels': 3072,
    'channel_width_khz': 10,
    'field_of_view_deg': 25,   # at 150 MHz
    'angular_res_arcmin': 2,   # at 150 MHz
    'baseline_max_m': 5300,    # Phase II extended
}

# MWA survey programs
MWA_SURVEYS = {
    'GLEAM': {
        'name': 'GaLactic and Extragalactic All-sky MWA',
        'freq_mhz': (72, 231),
        'n_sources': 307455,  # GLEAM catalog
        'description': 'All-sky survey at 72-231 MHz',
    },
    'GLEAM-X': {
        'name': 'GLEAM eXtended',
        'freq_mhz': (72, 231),
        'description': 'Extended GLEAM with Phase II',
    },
    'MWA-SMART': {
        'name': 'Survey for MWA Radio Transients',
        'freq_mhz': (140, 170),
        'description': 'Transient survey — relevant for technosignature detection',
    },
    'EoR': {
        'name': 'Epoch of Reionization',
        'freq_mhz': (138, 197),
        'description': '21cm cosmology — deep field observations',
    },
}


def _fetch_json(url, timeout=30):
    """Fetch JSON from URL."""
    try:
        req = urllib.request.Request(url, headers={'User-Agent': 'SEDI/0.1'})
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            return json.loads(resp.read())
    except Exception as e:
        print(f"  [mwa] Fetch error: {e}")
    return None


def search_observations(target: str = None, freq_mhz: float = 150.0) -> List[Dict]:
    """Search MWA archive for observations.

    Args:
        target: Source name or coordinates.
        freq_mhz: Center frequency in MHz.

    Returns:
        List of observation metadata.
    """
    # Try MWA metadata web service
    url = f"{MWA_METADATA}/find"
    if target:
        url += f"?source={target}&freq={freq_mhz}"
    data = _fetch_json(url)
    if data and isinstance(data, list):
        return data

    # Fallback: return survey information
    print("  [mwa] Archive API unavailable, returning survey metadata")
    results = []
    for key, info in MWA_SURVEYS.items():
        results.append({'survey': key, **info})
    return results


def list_public_data() -> List[Dict]:
    """List publicly available MWA data products.

    Returns:
        List of available data products and catalogs.
    """
    data = _fetch_json(f"{MWA_ASVO}/public")
    if data and isinstance(data, list):
        return data

    print("  [mwa] Using known public data catalog info")
    return [
        {
            'catalog': 'GLEAM',
            'n_sources': 307455,
            'url': 'https://vizier.cds.unistra.fr/viz-bin/VizieR?-source=VIII/100',
            'description': 'GLEAM extragalactic catalog — 307,455 radio sources',
        },
        {
            'catalog': 'GLEAM_4yr',
            'url': 'https://vizier.cds.unistra.fr/viz-bin/VizieR?-source=VIII/106',
            'description': 'GLEAM 4-year data release',
        },
    ]


def get_survey_data() -> List[float]:
    """Return MWA survey and system data as numeric array for scanning.

    Combines system specs, survey frequencies, and GLEAM catalog statistics.
    """
    values = [
        float(MWA_SPECS['n_tiles']),             # 128
        float(MWA_SPECS['n_dipoles_per_tile']),   # 16
        float(MWA_SPECS['bandwidth_mhz']),        # 30.72
        float(MWA_SPECS['n_channels']),            # 3072
        float(MWA_SPECS['channel_width_khz']),     # 10
        float(MWA_SPECS['field_of_view_deg']),     # 25
        float(MWA_SPECS['angular_res_arcmin']),    # 2
        float(MWA_SPECS['baseline_max_m']),        # 5300
    ]
    # Add frequency boundaries
    values.extend([float(v) for v in MWA_SPECS['freq_range_mhz']])

    # Add survey frequency ranges
    for key, info in MWA_SURVEYS.items():
        lo, hi = info['freq_mhz']
        values.extend([float(lo), float(hi)])

    # GLEAM catalog source count
    values.append(307455.0)

    print(f"  [mwa] Loaded {len(values)} system/survey parameter values")
    return values
