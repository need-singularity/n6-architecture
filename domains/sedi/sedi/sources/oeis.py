"""OEIS (Online Encyclopedia of Integer Sequences) monitor.

Monitors OEIS for new sequences and checks against n=6 patterns.
"""
import json
import logging
import time
import urllib.parse
import urllib.request
from typing import Generator

from ._connection import fetch_json, logger

OEIS_SEARCH_URL = "https://oeis.org/search?fmt=json&q={query}&start=0"
OEIS_RECENT_URL = "https://oeis.org/search?fmt=json&q=keyword:new&start=0"


def search_oeis(query, max_results=5):
    """Search OEIS for sequences matching a query."""
    url = OEIS_SEARCH_URL.format(query=urllib.parse.quote(query))
    data = fetch_json(url, source_tag='oeis', timeout=15)
    if not data or not isinstance(data, dict):
        return []
    results = data.get('results', None)
    if isinstance(results, list):
        return results[:max_results]
    return []


def check_sequence_for_n6(seq_values):
    """Check if a sequence contains n=6 constants at significant positions."""
    from ..constants import N, SIGMA, PHI, TAU, SOPFR

    targets = {
        'n=6': N, 'sigma=12': SIGMA, 'phi=2': PHI,
        'tau=4': TAU, 'sopfr=5': SOPFR,
        'sigma*phi=24': SIGMA * PHI, 'P2=28': 28,
    }

    hits = []
    for i, val in enumerate(seq_values[:50]):
        for name, target in targets.items():
            if val == target and i > 0:
                hits.append({
                    'position': i,
                    'value': val,
                    'match': name,
                })
    return hits


def monitor_recent(interval=3600) -> Generator:
    """Monitor OEIS for recently added sequences.

    Checks every `interval` seconds (default: 1 hour).
    """
    seen = set()
    while True:
        results = search_oeis('keyword:new')
        for seq in results:
            seq_id = seq.get('number', 0)
            if seq_id in seen:
                continue
            seen.add(seq_id)

            values = seq.get('data', '').split(',')
            try:
                int_values = [int(v.strip()) for v in values if v.strip()]
            except ValueError:
                continue

            hits = check_sequence_for_n6(int_values)
            if hits:
                yield {
                    'source': 'oeis',
                    'timestamp': time.time(),
                    'seq_id': f'A{seq_id:06d}',
                    'name': seq.get('name', ''),
                    'data': int_values[:20],
                    'n6_hits': hits,
                }

        time.sleep(interval)
