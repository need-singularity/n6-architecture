"""USGS Earthquake data source — historical + real-time.

API: https://earthquake.usgs.gov/fdsnws/event/1/
Data from 1900 to present. Free, no key needed.
"""
import json
import logging
import time
import urllib.request
import numpy as np
from typing import Generator, Optional

from ._connection import fetch_json, logger

BASE_URL = "https://earthquake.usgs.gov/fdsnws/event/1/query"


def fetch_earthquakes(starttime='2024-01-01', endtime='2024-12-31',
                      minmagnitude=4.0, limit=1000):
    """Fetch earthquake events in date range.

    Args:
        starttime/endtime: ISO date strings (YYYY-MM-DD)
        minmagnitude: minimum magnitude
        limit: max events to return
    """
    params = (
        f"?format=geojson&starttime={starttime}&endtime={endtime}"
        f"&minmagnitude={minmagnitude}&limit={limit}&orderby=time"
    )
    url = BASE_URL + params
    data = fetch_json(url, source_tag='earthquake', timeout=30)
    if not data or not isinstance(data, dict):
        return []

    features = data.get('features', [])
    events = []
    for f in features:
        try:
            props = f['properties']
            coords = f['geometry']['coordinates']
            events.append({
                'time': props.get('time', 0) / 1000,  # ms -> s
                'mag': props.get('mag', 0),
                'depth': coords[2] if len(coords) > 2 else 0,
                'lat': coords[1],
                'lon': coords[0],
                'place': props.get('place', ''),
            })
        except (KeyError, IndexError, TypeError) as e:
            logger.warning("[earthquake] Skipping malformed event: %s", e)
            continue
    return events


def extract_magnitudes(events):
    """Extract magnitude sequence for R-filter analysis."""
    return [e['mag'] for e in events if e['mag'] is not None]


def extract_depths(events):
    """Extract depth sequence for R-filter analysis."""
    return [e['depth'] for e in events if e['depth'] is not None]


def extract_intervals(events):
    """Extract time intervals between consecutive events."""
    times = sorted([e['time'] for e in events])
    if len(times) < 2:
        return []
    return [times[i+1] - times[i] for i in range(len(times)-1)]


def scan_historical(start_year=2000, end_year=2025, minmag=5.0):
    """Scan historical earthquake data year by year.

    Yields yearly batches for R-filter analysis.
    """
    for year in range(start_year, end_year + 1):
        events = fetch_earthquakes(
            starttime=f'{year}-01-01',
            endtime=f'{year}-12-31',
            minmagnitude=minmag,
        )
        if events:
            mags = extract_magnitudes(events)
            intervals = extract_intervals(events)
            yield {
                'source': 'earthquake',
                'timestamp': time.time(),
                'year': year,
                'n_events': len(events),
                'data': mags,
                'intervals': intervals,
                'n': len(mags),
            }
        time.sleep(1)  # rate limit


def stream_recent(interval=300, minmag=4.0) -> Generator:
    """Monitor recent earthquakes (last hour, every 5 min)."""
    while True:
        from datetime import datetime, timedelta
        now = datetime.utcnow()
        start = (now - timedelta(hours=1)).strftime('%Y-%m-%dT%H:%M:%S')
        end = now.strftime('%Y-%m-%dT%H:%M:%S')
        events = fetch_earthquakes(starttime=start, endtime=end, minmagnitude=minmag)
        if events:
            yield {
                'source': 'earthquake-live',
                'timestamp': time.time(),
                'data': extract_magnitudes(events),
                'n': len(events),
            }
        time.sleep(interval)
