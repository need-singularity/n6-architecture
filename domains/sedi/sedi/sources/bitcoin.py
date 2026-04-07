"""Bitcoin block nonce source.

Monitors Bitcoin blockchain for block nonces and checks n=6 patterns.
Uses public blockchain.info API.
"""
import json
import logging
import time
import urllib.request
from typing import Generator

from ._connection import fetch_json, logger

LATEST_BLOCK_URL = "https://blockchain.info/latestblock"
BLOCK_URL = "https://blockchain.info/rawblock/{hash}"


def fetch_latest_block():
    """Fetch latest Bitcoin block info."""
    return fetch_json(LATEST_BLOCK_URL, source_tag='bitcoin', timeout=10)


def fetch_block(block_hash):
    """Fetch full block data."""
    url = BLOCK_URL.format(hash=block_hash)
    return fetch_json(url, source_tag='bitcoin', timeout=15)


def stream(interval=60) -> Generator:
    """Monitor new Bitcoin blocks for nonce patterns.

    Checks every `interval` seconds (default: 60s, avg block time ~600s).
    """
    last_height = 0

    while True:
        latest = fetch_latest_block()
        if latest and latest.get('height', 0) > last_height:
            last_height = latest['height']
            block = fetch_block(latest['hash'])
            if block:
                nonce = block.get('nonce', 0)
                n_tx = block.get('n_tx', 0)
                size = block.get('size', 0)

                # Extract nonce digits for n=6 analysis
                nonce_digits = [int(d) for d in str(nonce)]

                yield {
                    'source': 'bitcoin',
                    'timestamp': time.time(),
                    'data': nonce_digits,
                    'n': len(nonce_digits),
                    'block_height': last_height,
                    'nonce': nonce,
                    'n_tx': n_tx,
                    'size': size,
                }

        time.sleep(interval)
