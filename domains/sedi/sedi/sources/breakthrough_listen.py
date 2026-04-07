"""Breakthrough Listen Open Data Archive — radio SETI observations.

Data: https://breakthroughinitiatives.org/opendataarchive
Formats: HDF5 (filterbank), FITS, CSV target lists
Telescopes: Green Bank (GBT), Parkes, MeerKAT

File download based — no REST API. We fetch target catalogs and
download filterbank/HDF5 files for R-spectrum analysis.
"""
import os
import json
import csv
import io
import logging
import urllib.request
from typing import Optional, List, Dict

from ._connection import fetch_json, fetch_text, fetch_url, logger

# Breakthrough Listen data endpoints
BL_ARCHIVE_BASE = "http://breakthroughinitiatives.org/opendataarchive"
BL_GBT_TARGETS = "https://storage.googleapis.com/gbt_data/targets.csv"
BL_EXOTICA_CATALOG = (
    "https://breakthroughinitiatives.org/opendataarchive/downloads/exotica_catalog.csv"
)

# GCS bucket for GBT filterbank data
GCS_GBT_BASE = "https://storage.googleapis.com/gbt_data"
GCS_PARKES_BASE = "https://storage.googleapis.com/parkes_data"

# Known interesting targets for n=6 analysis
N6_PRIORITY_TARGETS = [
    "Tabby's Star",    # KIC 8462852 — anomalous dimming
    "Proxima Cen",     # closest star
    "TRAPPIST-1",      # 7-planet system
    "Ross 128",        # nearby, radio signal detected 2017
    "Kepler-160",      # super-Earth in habitable zone
    "HD 164922",       # Sun-like with planet
]

# Hardcoded backup target list — used when GCS is unreachable (403, timeout)
BL_BACKUP_TARGETS = [
    {'name': "Tabby's Star",  'ra': '20h06m15.5s',  'dec': '+44d27m24s',  'ra_deg': 301.565, 'dec_deg': 44.457, 'dist_pc': 454},
    {'name': 'Proxima Cen',   'ra': '14h29m43.0s',  'dec': '-62d40m46s',  'ra_deg': 217.429, 'dec_deg': -62.679, 'dist_pc': 1.301},
    {'name': 'TRAPPIST-1',    'ra': '23h06m29.4s',  'dec': '-05d02m29s',  'ra_deg': 346.622, 'dec_deg': -5.041, 'dist_pc': 12.43},
    {'name': 'Ross 128',      'ra': '11h47m44.4s',  'dec': '+00d48m16s',  'ra_deg': 176.935, 'dec_deg': 0.805, 'dist_pc': 3.37},
    {'name': 'Kepler-160',    'ra': '19h32m18.4s',  'dec': '+42d16m00s',  'ra_deg': 293.077, 'dec_deg': 42.267, 'dist_pc': 936},
    {'name': 'HD 164922',     'ra': '18h02m10.6s',  'dec': '+26d18m42s',  'ra_deg': 270.544, 'dec_deg': 26.312, 'dist_pc': 22.1},
    {'name': 'Alpha Cen A',   'ra': '14h39m36.5s',  'dec': '-60d50m02s',  'ra_deg': 219.902, 'dec_deg': -60.834, 'dist_pc': 1.339},
    {'name': 'Alpha Cen B',   'ra': '14h39m35.1s',  'dec': '-60d50m14s',  'ra_deg': 219.896, 'dec_deg': -60.837, 'dist_pc': 1.339},
    {'name': 'Tau Ceti',      'ra': '01h44m04.1s',  'dec': '-15d56m15s',  'ra_deg': 26.017, 'dec_deg': -15.938, 'dist_pc': 3.65},
    {'name': 'Epsilon Eridani','ra': '03h32m55.8s',  'dec': '-09d27m30s',  'ra_deg': 53.233, 'dec_deg': -9.458, 'dist_pc': 3.22},
    {'name': 'GJ 876',        'ra': '22h53m16.7s',  'dec': '-14d15m49s',  'ra_deg': 343.320, 'dec_deg': -14.264, 'dist_pc': 4.67},
    {'name': 'YZ Ceti',       'ra': '01h12m30.6s',  'dec': '-16d59m56s',  'ra_deg': 18.128, 'dec_deg': -16.999, 'dist_pc': 3.72},
    {'name': 'Luyten Star',   'ra': '07h27m24.5s',  'dec': '+05d13m33s',  'ra_deg': 111.852, 'dec_deg': 5.226, 'dist_pc': 3.80},
    {'name': 'Teegarden Star','ra': '02h53m00.9s',  'dec': '+16d52m53s',  'ra_deg': 43.254, 'dec_deg': 16.882, 'dist_pc': 3.83},
    {'name': 'Wolf 1061',     'ra': '16h30m18.1s',  'dec': '-12d39m45s',  'ra_deg': 247.575, 'dec_deg': -12.663, 'dist_pc': 4.31},
    {'name': 'GJ 273',        'ra': '07h27m24.5s',  'dec': '+05d13m33s',  'ra_deg': 111.852, 'dec_deg': 5.226, 'dist_pc': 3.80},
    {'name': 'Kapteyn Star',  'ra': '05h11m40.6s',  'dec': '-45d01m06s',  'ra_deg': 77.919, 'dec_deg': -45.018, 'dist_pc': 3.91},
    {'name': 'Lacaille 9352', 'ra': '23h05m52.0s',  'dec': '-35d51m11s',  'ra_deg': 346.467, 'dec_deg': -35.853, 'dist_pc': 3.29},
]


def fetch_target_catalog(url=None):
    """Fetch Breakthrough Listen target list (CSV).

    Returns list of dicts with target name, RA, Dec, distance.
    Falls back to hardcoded backup targets on any failure.
    """
    url = url or BL_GBT_TARGETS
    text = fetch_text(url, source_tag='bl', timeout=30)
    if text is not None:
        try:
            reader = csv.DictReader(io.StringIO(text))
            targets = []
            for row in reader:
                try:
                    targets.append({
                        'name': row.get('source_name', row.get('name', '')).strip(),
                        'ra': row.get('ra', row.get('RA', '')),
                        'dec': row.get('dec', row.get('DEC', '')),
                        'raw': row,
                    })
                except (KeyError, AttributeError) as e:
                    logger.warning("[bl] Skipping malformed target row: %s", e)
                    continue
            if targets:
                return targets
        except csv.Error as e:
            logger.error("[bl] CSV parse error: %s", e)

    logger.info("[bl] Using backup target list (%d targets)", len(BL_BACKUP_TARGETS))
    return BL_BACKUP_TARGETS


def download_filterbank(target_name, output_dir="data/bl", max_size_mb=500):
    """Download a filterbank (.h5/.fil) file for a target.

    Breakthrough Listen stores data in GCS buckets.
    Returns local filepath if successful.
    """
    os.makedirs(output_dir, exist_ok=True)
    safe_name = target_name.replace(" ", "_").replace("'", "")
    local_path = os.path.join(output_dir, f"{safe_name}.h5")

    if os.path.exists(local_path):
        print(f"  [bl] Already downloaded: {local_path}")
        return local_path

    # Try GBT bucket
    url = f"{GCS_GBT_BASE}/{safe_name}/{safe_name}_guppi.h5"
    size_mb = 0
    try:
        req = urllib.request.Request(url, headers={'User-Agent': 'SEDI/0.1'})
        req.method = 'HEAD'
        with urllib.request.urlopen(req, timeout=10) as resp:
            size_mb = int(resp.headers.get('Content-Length', 0)) / 1e6
            if size_mb > max_size_mb:
                logger.warning("[bl] File too large (%d MB > %d MB)", size_mb, max_size_mb)
                return None
    except urllib.request.URLError as e:
        logger.warning("[bl] Target not found in GBT archive: %s (%s)", target_name, e)
        return None
    except Exception as e:
        logger.error("[bl] HEAD request failed for %s: %s", target_name, e)
        return None

    try:
        logger.info("[bl] Downloading %s (%d MB)...", target_name, size_mb)
        urllib.request.urlretrieve(url, local_path)
        return local_path
    except Exception as e:
        logger.error("[bl] Download error for %s: %s", target_name, e)
    return None


def load_filterbank(filepath):
    """Load HDF5 filterbank data for R-spectrum analysis.

    Requires: pip install h5py blimpy
    Returns dict with frequency, time, spectrogram data.
    """
    try:
        import h5py
    except ImportError:
        print("  [bl] h5py required: pip install h5py")
        return None

    try:
        with h5py.File(filepath, 'r') as f:
            data_key = 'data' if 'data' in f else list(f.keys())[0]
            raw = f[data_key]

            # Extract metadata
            header = {}
            for key in f.attrs:
                val = f.attrs[key]
                header[key] = val.item() if hasattr(val, 'item') else val

            # Get spectrogram (time x frequency)
            if len(raw.shape) == 3:
                spectrogram = raw[:, 0, :]  # (ntime, 1, nfreq) -> (ntime, nfreq)
            elif len(raw.shape) == 2:
                spectrogram = raw[:]
            else:
                spectrogram = raw[:]

            return {
                'source': 'breakthrough-listen',
                'filepath': filepath,
                'data': spectrogram,
                'shape': spectrogram.shape,
                'header': header,
                'fch1': header.get('fch1', 0),
                'foff': header.get('foff', 0),
                'tsamp': header.get('tsamp', 0),
                'n': spectrogram.size,
            }
    except (OSError, KeyError, ValueError) as e:
        logger.error("[bl] Load error for %s: %s", filepath, e)
    except Exception as e:
        logger.error("[bl] Unexpected load error for %s: %s", filepath, e)
    return None


def extract_narrowband(spectrogram, threshold_sigma=5.0):
    """Extract narrowband signals from spectrogram.

    Narrowband = signal confined to few frequency channels.
    This is what turboSETI looks for — potential technosignatures.

    Returns list of (time_idx, freq_idx, snr) for detections.
    """
    import numpy as np

    if not isinstance(spectrogram, np.ndarray):
        spectrogram = np.array(spectrogram, dtype=float)

    # Compute per-channel statistics
    chan_mean = np.mean(spectrogram, axis=0)
    chan_std = np.std(spectrogram, axis=0)
    chan_std[chan_std == 0] = 1  # avoid division by zero

    # Z-score each time-frequency bin
    z_scores = (spectrogram - chan_mean) / chan_std

    # Find bins above threshold
    hits = np.argwhere(np.abs(z_scores) > threshold_sigma)
    detections = []
    for t_idx, f_idx in hits:
        detections.append({
            'time_idx': int(t_idx),
            'freq_idx': int(f_idx),
            'snr': float(z_scores[t_idx, f_idx]),
        })

    return detections


def scan_for_n6_patterns(spectrogram, fch1=0, foff=1):
    """Scan spectrogram for n=6 frequency ratios.

    Checks if narrowband signals appear at frequencies
    related by SEDI's core ratios (σ/τ, φ/τ, δ+, δ-).
    """
    import numpy as np
    from ..constants import RATIOS, DELTA_PLUS, DELTA_MINUS

    detections = extract_narrowband(spectrogram)
    if not detections:
        return {'n6_hits': [], 'n_detections': 0}

    # Convert freq indices to actual frequencies
    freqs = [fch1 + d['freq_idx'] * foff for d in detections]

    # Check all pairs for n=6 ratios
    n6_hits = []
    tolerance = 0.005  # 0.5% match tolerance

    for i, f1 in enumerate(freqs):
        for j, f2 in enumerate(freqs):
            if i >= j or f2 == 0:
                continue
            ratio = f1 / f2
            for name, target in RATIOS.items():
                if target == 0:
                    continue
                if abs(ratio - target) / target < tolerance:
                    n6_hits.append({
                        'freq_pair': (f1, f2),
                        'ratio': ratio,
                        'pattern': name,
                        'target': target,
                        'deviation': abs(ratio - target) / target,
                        'snr_pair': (
                            detections[i]['snr'],
                            detections[j]['snr'],
                        ),
                    })

    return {
        'n6_hits': n6_hits,
        'n_detections': len(detections),
        'n_pairs_checked': len(freqs) * (len(freqs) - 1) // 2,
    }


def fetch_public_data_index():
    """List downloadable files from the Breakthrough Listen open data archive.

    Queries the GCS bucket listing for available filterbank/HDF5 files.
    Falls back to a curated list of known public datasets.

    Returns:
        List of dicts with file info (name, url, size, telescope).
    """
    # Try GCS bucket listing
    gcs_list_url = f"{GCS_GBT_BASE}?prefix=&max-keys=100"
    content = fetch_text(gcs_list_url, source_tag='bl', timeout=15)
    if content is not None:
        try:
            import re
            files = []
            for match in re.finditer(r'<Key>(.*?)</Key>', content):
                key = match.group(1)
                if key.endswith(('.h5', '.fil', '.fits', '.csv')):
                    files.append({
                        'name': key.split('/')[-1],
                        'path': key,
                        'url': f"{GCS_GBT_BASE}/{key}",
                        'telescope': 'GBT',
                    })
            if files:
                logger.info("[bl] Found %d files in GCS bucket", len(files))
                return files
        except Exception as e:
            logger.error("[bl] GCS XML parse error: %s", e)

    # Fallback: known public data files
    logger.info("[bl] Using curated public data index")
    return [
        {'name': 'GBT_targets.csv', 'url': BL_GBT_TARGETS, 'telescope': 'GBT', 'type': 'target_list'},
        {'name': 'exotica_catalog.csv', 'url': BL_EXOTICA_CATALOG, 'telescope': 'multi', 'type': 'catalog'},
        {'name': 'BL_DR1_GBT.tar', 'url': f'{GCS_GBT_BASE}/BL_DR1/', 'telescope': 'GBT', 'type': 'data_release_1'},
        {'name': 'BL_DR2_GBT.tar', 'url': f'{GCS_GBT_BASE}/BL_DR2/', 'telescope': 'GBT', 'type': 'data_release_2'},
    ]
