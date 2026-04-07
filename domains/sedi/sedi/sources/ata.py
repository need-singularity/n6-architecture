"""Allen Telescope Array (ATA) — SETI Institute's radio telescope.

ATATS catalog: https://data.nasa.gov/resource/wqn5-jz4i.json (Socrata API)
The ATA Transient Survey (ATATS) catalog contains radio sources detected
during ATA's survey of the northern sky.

ATA is a 42-dish array at Hat Creek Observatory, California,
designed specifically for SETI and radio astronomy.
"""
import json
import logging
import urllib.request
from typing import List, Dict, Optional

from ._connection import fetch_json, logger

# NASA Open Data Socrata API endpoint for ATATS catalog
ATATS_URL = "https://data.nasa.gov/resource/wqn5-jz4i.json"

# ATA system parameters
ATA_SPECS = {
    'n_dishes': 42,
    'dish_diameter_m': 6.1,
    'freq_range_ghz': (0.5, 11.2),
    'max_baseline_m': 300,
    'location': 'Hat Creek, CA',
    'latitude': 40.8178,
    'longitude': -121.4733,
}


def fetch_atats_catalog(limit: int = 1000, offset: int = 0) -> List[Dict]:
    """Fetch ATATS radio source catalog from NASA Open Data.

    Args:
        limit: Max number of records (Socrata default is 1000).
        offset: Pagination offset.

    Returns:
        List of radio source records with positions and flux densities.
    """
    url = f"{ATATS_URL}?$limit={limit}&$offset={offset}"
    data = fetch_json(url, source_tag='ata', timeout=30,
                      headers={'Accept': 'application/json'})
    if data and isinstance(data, list):
        logger.info("[ata] Fetched %d ATATS catalog entries", len(data))
        return data

    logger.warning("[ata] ATATS catalog unavailable")
    return []


def scan_catalog() -> List[float]:
    """Fetch ATATS catalog and extract numeric values for n=6 scanning.

    Extracts flux densities, positions, and other numeric fields.
    Returns flat list of floats suitable for np.array().
    """
    catalog = fetch_atats_catalog(limit=500)
    values = []

    if catalog:
        for entry in catalog:
            # Extract all numeric fields
            for key, val in entry.items():
                if val is not None:
                    try:
                        values.append(float(val))
                    except (ValueError, TypeError):
                        pass

    if values:
        print(f"  [ata] Extracted {len(values)} numeric values from ATATS catalog")
        return values

    # Fallback: ATA system parameters
    print("  [ata] Using ATA system parameters as fallback data")
    fallback = [
        float(ATA_SPECS['n_dishes']),           # 42
        float(ATA_SPECS['dish_diameter_m']),     # 6.1
        float(ATA_SPECS['max_baseline_m']),      # 300
        float(ATA_SPECS['latitude']),            # 40.8178
        float(ATA_SPECS['longitude']),           # -121.4733
        ATA_SPECS['freq_range_ghz'][0],          # 0.5
        ATA_SPECS['freq_range_ghz'][1],          # 11.2
    ]
    return fallback
