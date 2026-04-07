"""SETI archival data — Allen Telescope Array, SETI@home, VizieR catalogs.

Multiple data sources for historical SETI observations:
1. SETI@home archived workunit data (Berkeley)
2. Allen Telescope Array (ATA) public releases
3. VizieR astronomical catalogs (TAP API)
4. MAST archive (Kepler/TESS for transit anomalies)
"""
import json
import logging
import urllib.request
import urllib.parse
from typing import Optional, List, Dict

from ._connection import fetch_json as _conn_fetch_json, logger

# VizieR TAP service for astronomical catalogs
VIZIER_TAP = "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync"

# MAST archive for Kepler/TESS
MAST_API = "https://mast.stsci.edu/api/v0/invoke"

# Known SETI-relevant catalogs in VizieR
SETI_CATALOGS = {
    # Turnbull & Tarter (2003) — HabCat: Habitable Star Catalog
    'habcat': 'J/ApJS/145/181',
    # Nearby star catalog
    'nearby_stars': 'V/70A',
    # Gaia DR3 nearby
    'gaia_nearby': 'I/355/gaiadr3',
}


def query_vizier(adql, fmt="json", timeout=30):
    """Execute ADQL query against VizieR TAP service."""
    params = urllib.parse.urlencode({
        'request': 'doQuery',
        'lang': 'ADQL',
        'format': fmt,
        'query': adql,
    })
    url = f"{VIZIER_TAP}?{params}"
    try:
        req = urllib.request.Request(url, headers={'User-Agent': 'SEDI/0.1'})
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            return json.loads(resp.read())
    except Exception as e:
        print(f"  [seti-archive] VizieR error: {e}")
    return None


def fetch_habcat(limit=500):
    """Fetch HabCat — Habitable Star Systems catalog.

    Turnbull & Tarter (2003): ~17,000 potentially habitable systems.
    """
    adql = f"""
        SELECT TOP {limit} *
        FROM "{SETI_CATALOGS['habcat']}/table1"
        ORDER BY "Dist"
    """
    return query_vizier(adql.strip())


def fetch_nearby_stars(max_dist_pc=20, limit=200):
    """Fetch nearby stars within distance limit (parsecs)."""
    adql = f"""
        SELECT TOP {limit}
            "Name", "RAJ2000", "DEJ2000", "Dist", "SpType", "Vmag"
        FROM "{SETI_CATALOGS['nearby_stars']}/catalog"
        WHERE "Dist" < {max_dist_pc}
        ORDER BY "Dist"
    """
    return query_vizier(adql.strip())


def query_mast_kepler(target_name, timeout=30):
    """Query MAST for Kepler light curves of a target.

    Returns observation metadata — actual data requires file download.
    """
    params = {
        'service': 'Mast.Caom.Filtered',
        'format': 'json',
        'params': json.dumps({
            'columns': 'dataproduct_type,obs_collection,target_name,t_min,t_max,dataURL',
            'filters': [
                {'paramName': 'obs_collection', 'values': ['Kepler', 'TESS']},
                {'paramName': 'target_name', 'values': [target_name]},
                {'paramName': 'dataproduct_type', 'values': ['timeseries']},
            ],
        }),
    }
    data = urllib.parse.urlencode(params).encode()
    try:
        req = urllib.request.Request(MAST_API, data=data,
                                     headers={'User-Agent': 'SEDI/0.1',
                                              'Content-Type': 'application/x-www-form-urlencoded'})
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            result = json.loads(resp.read())
            return result.get('data', [])
    except Exception as e:
        print(f"  [seti-archive] MAST error: {e}")
    return []


def download_kepler_lightcurve(data_url, output_dir="data/kepler"):
    """Download a Kepler/TESS light curve FITS file."""
    import os
    os.makedirs(output_dir, exist_ok=True)
    filename = os.path.basename(data_url.split('?')[0])
    if '..' in filename or '/' in filename:
        print(f"  [seti-archive] Invalid filename: {filename}")
        return None
    local_path = os.path.join(output_dir, filename)

    if os.path.exists(local_path):
        print(f"  [seti-archive] Already downloaded: {local_path}")
        return local_path

    try:
        print(f"  [seti-archive] Downloading {filename}...")
        urllib.request.urlretrieve(data_url, local_path)
        return local_path
    except Exception as e:
        print(f"  [seti-archive] Download error: {e}")
    return None


def load_lightcurve_fits(filepath):
    """Load a Kepler/TESS light curve from FITS file.

    Requires: pip install astropy
    Returns dict with time and flux arrays for R-spectrum analysis.
    """
    try:
        from astropy.io import fits
    except ImportError:
        print("  [seti-archive] astropy required: pip install astropy")
        return None

    try:
        with fits.open(filepath) as hdul:
            # Kepler/TESS light curves: extension 1 has TIME, PDCSAP_FLUX
            data = hdul[1].data
            time = data['TIME']
            flux = data['PDCSAP_FLUX']

            # Remove NaN
            import numpy as np
            mask = np.isfinite(time) & np.isfinite(flux)
            time = time[mask]
            flux = flux[mask]

            return {
                'source': 'kepler-lightcurve',
                'filepath': filepath,
                'time': time.tolist(),
                'data': flux.tolist(),
                'n': len(flux),
            }
    except Exception as e:
        print(f"  [seti-archive] FITS load error: {e}")
    return None


def scan_transit_anomalies(flux_data, expected_period=None):
    """Scan light curve for anomalous transit patterns.

    Looks for:
    - Transit timing variations (TTV) with n=6 periodicity
    - Depth variations matching SEDI ratios
    - Non-periodic dimming events (like Tabby's Star)
    """
    import numpy as np
    from ..constants import RATIOS, N, SIGMA, TAU

    flux = np.array(flux_data, dtype=float)
    flux_norm = flux / np.median(flux)

    # Find dips (potential transits)
    threshold = 1 - 3 * np.std(flux_norm)
    dip_indices = np.where(flux_norm < threshold)[0]

    if len(dip_indices) < 2:
        return {'anomalies': [], 'n_dips': len(dip_indices)}

    # Group consecutive indices into dip events
    events = []
    current = [dip_indices[0]]
    for idx in dip_indices[1:]:
        if idx - current[-1] <= 2:
            current.append(idx)
        else:
            events.append(current)
            current = [idx]
    events.append(current)

    # Analyze inter-event spacings
    centers = [np.mean(e) for e in events]
    spacings = np.diff(centers)

    anomalies = []
    if len(spacings) >= 2:
        # Check spacing ratios for n=6 patterns
        for i in range(len(spacings)):
            for j in range(i + 1, len(spacings)):
                if spacings[j] == 0:
                    continue
                ratio = spacings[i] / spacings[j]
                for name, target in RATIOS.items():
                    if target == 0:
                        continue
                    if abs(ratio - target) / max(target, 1e-10) < 0.05:
                        anomalies.append({
                            'type': 'spacing_ratio',
                            'pattern': name,
                            'ratio': float(ratio),
                            'target': target,
                            'event_indices': (i, j),
                        })

        # Check if number of events is n=6 related
        n_events = len(events)
        if n_events in [N, SIGMA, TAU, N**2]:
            anomalies.append({
                'type': 'event_count',
                'count': n_events,
                'n6_relation': f"n={N}" if n_events == N else f"related",
            })

    return {
        'anomalies': anomalies,
        'n_dips': len(dip_indices),
        'n_events': len(events),
        'spacings': spacings.tolist() if len(spacings) > 0 else [],
    }
