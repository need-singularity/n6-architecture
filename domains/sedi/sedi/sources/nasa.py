"""NASA data sources — solar, NEO, cosmic rays.

APIs: api.nasa.gov (free key), SDO, DONKI
"""
import json
import logging
import time
import urllib.request
from typing import Generator

from ._connection import fetch_json, logger

# Default demo key (rate limited). Get your own at api.nasa.gov
NASA_API_KEY = "DEMO_KEY"

DONKI_URL = "https://api.nasa.gov/DONKI"
NEO_URL = "https://api.nasa.gov/neo/rest/v1/feed"


def fetch_solar_flares(start_date='2024-01-01', end_date='2024-12-31'):
    """Fetch solar flare events from DONKI.

    Returns list of flare events with class (A/B/C/M/X) and peak flux.
    """
    url = f"{DONKI_URL}/FLR?startDate={start_date}&endDate={end_date}&api_key={NASA_API_KEY}"
    flares = fetch_json(url, source_tag='nasa', timeout=30)
    if not flares or not isinstance(flares, list):
        return []

    events = []
    for f in flares:
        cls = f.get('classType', '')
        try:
            prefix = cls[0]
            num = float(cls[1:])
            scale = {'A': 0.001, 'B': 0.01, 'C': 0.1, 'M': 1.0, 'X': 10.0}
            flux = scale.get(prefix, 0) * num
        except (IndexError, ValueError):
            logger.debug("[nasa] Skipping malformed flare class: %r", cls)
            flux = 0
        events.append({
            'time': f.get('peakTime', ''),
            'class': cls,
            'flux': flux,
        })
    return events


def fetch_neo(start_date='2024-01-01', end_date='2024-01-07'):
    """Fetch Near-Earth Objects for a date range (max 7 days)."""
    url = f"{NEO_URL}?start_date={start_date}&end_date={end_date}&api_key={NASA_API_KEY}"
    data = fetch_json(url, source_tag='nasa', timeout=30)
    if not data or not isinstance(data, dict):
        return []

    objects = []
    for date, neos in data.get('near_earth_objects', {}).items():
        for neo in neos:
            try:
                diameter = neo.get('estimated_diameter', {}).get('meters', {})
                objects.append({
                    'name': neo.get('name', ''),
                    'date': date,
                    'diameter_min': diameter.get('estimated_diameter_min', 0),
                    'diameter_max': diameter.get('estimated_diameter_max', 0),
                    'hazardous': neo.get('is_potentially_hazardous_asteroid', False),
                    'miss_distance_km': float(
                        neo.get('close_approach_data', [{}])[0]
                        .get('miss_distance', {}).get('kilometers', 0)
                    ),
                })
            except (KeyError, IndexError, TypeError, ValueError) as e:
                logger.warning("[nasa] Skipping malformed NEO record: %s", e)
                continue
    return objects


def fetch_cme(start_date='2024-01-01', end_date='2024-12-31'):
    """Fetch Coronal Mass Ejections from DONKI."""
    url = f"{DONKI_URL}/CME?startDate={start_date}&endDate={end_date}&api_key={NASA_API_KEY}"
    cmes = fetch_json(url, source_tag='nasa', timeout=30)
    if not cmes or not isinstance(cmes, list):
        return []

    events = []
    for c in cmes:
        try:
            speed = 0
            for a in c.get('cmeAnalyses', []):
                s = a.get('speed', 0)
                if s and s > speed:
                    speed = s
            events.append({
                'time': c.get('startTime', ''),
                'speed': speed,  # km/s
            })
        except (KeyError, TypeError, ValueError) as e:
            logger.warning("[nasa] Skipping malformed CME record: %s", e)
            continue
    return events


def scan_solar_historical(start_year=2010, end_year=2025):
    """Scan solar flare data year by year."""
    for year in range(start_year, end_year + 1):
        flares = fetch_solar_flares(f'{year}-01-01', f'{year}-12-31')
        if flares:
            fluxes = [f['flux'] for f in flares if f['flux'] > 0]
            yield {
                'source': 'solar-flares',
                'timestamp': time.time(),
                'year': year,
                'data': fluxes,
                'n': len(fluxes),
                'n_events': len(flares),
            }
        time.sleep(2)  # rate limit for DEMO_KEY
