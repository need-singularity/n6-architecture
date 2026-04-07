"""Shared connection utilities for SEDI data sources.

Provides robust HTTP fetching with:
- Retry with exponential backoff for connection timeouts
- HTTP error logging and graceful continuation
- JSON parse error handling with malformed data logging
- Python logging module for all error reporting

Usage in source modules:
    from ._connection import fetch_json, fetch_text, logger

    data = fetch_json(url, source_tag='ligo', timeout=15)
"""

import json
import logging
import time
import urllib.error
import urllib.request
from typing import Any, Optional

logger = logging.getLogger('sedi.sources')

# ──────────────────────────────────────────────────────────────────
# Configuration
# ──────────────────────────────────────────────────────────────────

DEFAULT_TIMEOUT = 30
MAX_RETRIES = 3
BACKOFF_BASE = 2.0          # exponential backoff: base^attempt seconds
USER_AGENT = 'SEDI/0.1'


# ──────────────────────────────────────────────────────────────────
# Core Fetch Functions
# ──────────────────────────────────────────────────────────────────

def fetch_url(url: str, source_tag: str = 'http',
              timeout: int = DEFAULT_TIMEOUT,
              max_retries: int = MAX_RETRIES,
              headers: Optional[dict] = None) -> Optional[bytes]:
    """Fetch raw bytes from a URL with retry and error handling.

    Args:
        url: Target URL.
        source_tag: Source name for log messages (e.g. 'ligo', 'cern').
        timeout: Request timeout in seconds.
        max_retries: Maximum number of retry attempts.
        headers: Additional HTTP headers.

    Returns:
        Response bytes on success, None on failure.
    """
    req_headers = {'User-Agent': USER_AGENT}
    if headers:
        req_headers.update(headers)

    for attempt in range(max_retries):
        try:
            req = urllib.request.Request(url, headers=req_headers)
            with urllib.request.urlopen(req, timeout=timeout) as resp:
                return resp.read()

        except urllib.error.HTTPError as e:
            logger.error(
                "[%s] HTTP %d %s: %s (attempt %d/%d)",
                source_tag, e.code, e.reason, url, attempt + 1, max_retries,
            )
            if e.code >= 500 and attempt < max_retries - 1:
                # Server errors -> retry with backoff
                wait = BACKOFF_BASE ** attempt
                logger.info("[%s] Retrying in %.1fs...", source_tag, wait)
                time.sleep(wait)
            elif e.code == 429:
                # Rate limited -> longer backoff
                wait = BACKOFF_BASE ** (attempt + 2)
                logger.warning("[%s] Rate limited, waiting %.1fs", source_tag, wait)
                if attempt < max_retries - 1:
                    time.sleep(wait)
            else:
                # Client errors (4xx except 429) -> don't retry
                break

        except urllib.error.URLError as e:
            # Connection refused, DNS failure, timeout, etc.
            logger.warning(
                "[%s] Connection error: %s (attempt %d/%d)",
                source_tag, e.reason, attempt + 1, max_retries,
            )
            if attempt < max_retries - 1:
                wait = BACKOFF_BASE ** attempt
                logger.info("[%s] Retrying in %.1fs...", source_tag, wait)
                time.sleep(wait)

        except OSError as e:
            # Low-level network errors (socket timeout, etc.)
            logger.warning(
                "[%s] OS error: %s (attempt %d/%d)",
                source_tag, e, attempt + 1, max_retries,
            )
            if attempt < max_retries - 1:
                wait = BACKOFF_BASE ** attempt
                time.sleep(wait)

        except Exception as e:
            # Unexpected errors
            logger.error(
                "[%s] Unexpected error fetching %s: %s",
                source_tag, url, e,
            )
            break

    logger.warning("[%s] Failed to fetch %s after %d attempts",
                   source_tag, url, max_retries)
    return None


def fetch_json(url: str, source_tag: str = 'http',
               timeout: int = DEFAULT_TIMEOUT,
               max_retries: int = MAX_RETRIES,
               headers: Optional[dict] = None) -> Optional[Any]:
    """Fetch and parse JSON from a URL with full error handling.

    Args:
        url: Target URL.
        source_tag: Source name for log messages.
        timeout: Request timeout in seconds.
        max_retries: Maximum number of retry attempts.
        headers: Additional HTTP headers.

    Returns:
        Parsed JSON (dict/list) on success, None on failure.
    """
    raw = fetch_url(url, source_tag=source_tag, timeout=timeout,
                    max_retries=max_retries, headers=headers)
    if raw is None:
        return None

    try:
        return json.loads(raw)
    except json.JSONDecodeError as e:
        logger.error(
            "[%s] JSON parse error from %s: %s (first 200 chars: %r)",
            source_tag, url, e, raw[:200],
        )
        return None
    except (UnicodeDecodeError, ValueError) as e:
        logger.error(
            "[%s] Data decode error from %s: %s",
            source_tag, url, e,
        )
        return None


def fetch_text(url: str, source_tag: str = 'http',
               timeout: int = DEFAULT_TIMEOUT,
               max_retries: int = MAX_RETRIES,
               encoding: str = 'utf-8',
               headers: Optional[dict] = None) -> Optional[str]:
    """Fetch text content from a URL with full error handling.

    Args:
        url: Target URL.
        source_tag: Source name for log messages.
        timeout: Request timeout in seconds.
        max_retries: Maximum number of retry attempts.
        encoding: Text encoding.
        headers: Additional HTTP headers.

    Returns:
        Response text on success, None on failure.
    """
    raw = fetch_url(url, source_tag=source_tag, timeout=timeout,
                    max_retries=max_retries, headers=headers)
    if raw is None:
        return None

    try:
        return raw.decode(encoding)
    except (UnicodeDecodeError, ValueError) as e:
        logger.error(
            "[%s] Text decode error from %s: %s",
            source_tag, url, e,
        )
        return None
