"""ANU Quantum Random Number Generator source.

API: https://qrng.anu.edu.au/API/jsonI.php
Free: 1024 numbers per request, no key needed.
"""
import json
import logging
import time
import urllib.request
from typing import Generator

from ._connection import fetch_json, logger

API_URL = "https://qrng.anu.edu.au/API/jsonI.php?length={length}&type=uint16"


def fetch_quantum_random(length=1024):
    """Fetch quantum random numbers from ANU."""
    url = API_URL.format(length=min(length, 1024))
    data = fetch_json(url, source_tag='quantum-rng', timeout=10)
    if data and isinstance(data, dict) and data.get('success'):
        return data['data']
    return None


def stream(interval=5.0, batch=1024) -> Generator:
    """Continuous stream of quantum random numbers.

    Yields batches of `batch` uint16 values every `interval` seconds.
    """
    while True:
        data = fetch_quantum_random(batch)
        if data:
            yield {
                'source': 'quantum-rng',
                'timestamp': time.time(),
                'data': data,
                'n': len(data),
            }
        time.sleep(interval)
