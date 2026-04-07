"""LIGO Open Science Center gravitational wave data source.

Data: https://gwosc.org/
Formats: HDF5 strain data, event catalogs
"""
import os
import json
import logging
import urllib.request
from typing import Optional

from ._connection import fetch_json, logger

GWOSC_CATALOG_URL = "https://gwosc.org/eventapi/json/GWTC/"
GWOSC_STRAIN_URL = "https://gwosc.org/eventapi/json/GWTC-1/events/{event}/v3"


def fetch_event_catalog():
    """Fetch GWTC event catalog."""
    return fetch_json(GWOSC_CATALOG_URL, source_tag='ligo', timeout=15)


def fetch_event_details(event_name):
    """Fetch details for a specific GW event."""
    url = GWOSC_STRAIN_URL.format(event=event_name)
    return fetch_json(url, source_tag='ligo', timeout=15)


def load_strain_file(filepath):
    """Load HDF5 strain data file.

    Requires h5py: pip install h5py
    Returns: (strain_data, sample_rate, gps_start)
    """
    try:
        import h5py
    except ImportError:
        logger.error("[ligo] h5py required: pip install h5py")
        return None

    try:
        f = h5py.File(filepath, 'r')
    except Exception as e:
        logger.error("[ligo] Failed to open strain file %s: %s", filepath, e)
        return None

    with f:
        # Standard GWOSC format
        detector = list(f['strain'].keys())[0]
        strain = f['strain'][detector]['Strain'][:]
        dt = f['strain'][detector]['Strain'].attrs.get('Xspacing', 1/4096)
        gps = f['strain'][detector]['Strain'].attrs.get('Xstart', 0)
        return {
            'source': 'ligo',
            'detector': detector,
            'data': strain.tolist(),
            'sample_rate': 1/dt,
            'gps_start': gps,
            'n': len(strain),
        }


def scan_catalog_masses():
    """Scan GW catalog for masses near n=6 constants.

    Looks for: m1 or m2 near 6, 12, 28, 36, etc.
    """
    catalog = fetch_event_catalog()
    if not catalog or 'events' not in catalog:
        return []

    from ..constants import N, SIGMA, TAU, SOPFR

    targets = {
        'n=6': N, 'sigma=12': SIGMA, 'tau=4': TAU,
        'n^2=36': N**2, 'P2=28': 28, 'sigma*phi=24': 24,
    }

    hits = []
    for event_name, event_data in catalog['events'].items():
        for param in ['mass_1_source', 'mass_2_source', 'chirp_mass_source']:
            val = event_data.get(param, {})
            if isinstance(val, dict):
                median = val.get('median', val.get('best', None))
            else:
                median = val
            if median is None:
                continue
            for target_name, target_val in targets.items():
                if abs(median - target_val) / target_val < 0.05:  # within 5%
                    hits.append({
                        'event': event_name,
                        'param': param,
                        'value': median,
                        'target': target_name,
                        'error_pct': abs(median - target_val) / target_val * 100,
                    })
    return hits
