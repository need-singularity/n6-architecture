"""R-spectrum Receiver Backend — multi-source aggregator for R-spectrum analysis.

Aggregates data from all available SEDI sources (SETI, gravitational wave,
CMB, particle physics, etc.) into a unified stream suitable for R-spectrum
analysis. Each source is fetched in parallel with proper error handling,
and results are normalized into a common format.

Architecture:
    RSpectrumReceiver
        .register_source(name, fetch_fn, parse_fn)
        .fetch_all()          -> raw data from all sources
        .aggregate()          -> merged + normalized numeric stream
        .stream(interval)     -> continuous generator for monitor

Data flow:
    Source API/file -> fetch_fn() -> raw data
                    -> parse_fn(raw) -> List[float]
                    -> normalize -> merged stream
                    -> R-filter / PH detector
"""

import logging
import time
import json
import urllib.request
import urllib.error
import numpy as np
from concurrent.futures import ThreadPoolExecutor, as_completed
from dataclasses import dataclass, field
from typing import Any, Callable, Dict, Generator, List, Optional, Tuple

logger = logging.getLogger(__name__)

# ──────────────────────────────────────────────────────────────────
# Configuration
# ──────────────────────────────────────────────────────────────────

DEFAULT_TIMEOUT = 30        # seconds
MAX_RETRIES = 3
RETRY_BACKOFF_BASE = 2.0    # exponential backoff: 2^attempt seconds
MAX_WORKERS = 8             # parallel fetch threads


@dataclass
class SourceResult:
    """Result from a single data source fetch."""
    name: str
    data: List[float] = field(default_factory=list)
    raw: Any = None
    success: bool = False
    error: Optional[str] = None
    elapsed_s: float = 0.0
    n_values: int = 0
    retries: int = 0


@dataclass
class SourceConfig:
    """Configuration for a registered data source."""
    name: str
    fetch_fn: Callable
    parse_fn: Optional[Callable] = None
    enabled: bool = True
    priority: int = 0        # higher = fetched first
    timeout: int = DEFAULT_TIMEOUT


# ──────────────────────────────────────────────────────────────────
# Default Parsers
# ──────────────────────────────────────────────────────────────────

def _parse_numeric_list(raw) -> List[float]:
    """Parse raw data into a list of floats. Handles common formats."""
    if raw is None:
        return []

    # Already a list of numbers
    if isinstance(raw, (list, np.ndarray)):
        result = []
        for v in raw:
            try:
                if isinstance(v, dict):
                    # Extract first numeric value from dict
                    for val in v.values():
                        try:
                            result.append(float(val))
                        except (TypeError, ValueError):
                            continue
                elif v is not None:
                    f = float(v)
                    if np.isfinite(f):
                        result.append(f)
            except (TypeError, ValueError):
                continue
        return result

    # Single numeric value
    try:
        return [float(raw)]
    except (TypeError, ValueError):
        pass

    # Dict with 'data' key
    if isinstance(raw, dict):
        if 'data' in raw:
            return _parse_numeric_list(raw['data'])
        # Try extracting all numeric values
        result = []
        for v in raw.values():
            try:
                result.append(float(v))
            except (TypeError, ValueError):
                continue
        return result

    return []


def _parse_event_list(raw) -> List[float]:
    """Parse a list of event dicts (LIGO, earthquake, etc.)."""
    if not isinstance(raw, list):
        return []
    result = []
    # Common numeric keys in event data
    numeric_keys = ['mag', 'flux', 'snr', 'mass_1_source', 'mass_2_source',
                    'luminosity_distance', 'depth', 'ra_deg', 'dec_deg',
                    'parallax', 'radial_velocity', 'phot_g_mean_mag']
    for event in raw:
        if isinstance(event, dict):
            for key in numeric_keys:
                if key in event:
                    try:
                        val = float(event[key])
                        if np.isfinite(val):
                            result.append(val)
                    except (TypeError, ValueError):
                        continue
        else:
            try:
                val = float(event)
                if np.isfinite(val):
                    result.append(val)
            except (TypeError, ValueError):
                continue
    return result


# ──────────────────────────────────────────────────────────────────
# R-spectrum Receiver
# ──────────────────────────────────────────────────────────────────

class RSpectrumReceiver:
    """Multi-source data aggregator for R-spectrum analysis.

    Fetches data from registered sources in parallel, parses into
    numeric streams, and provides a unified interface for the
    R-filter and PH detector pipeline.

    Usage:
        receiver = RSpectrumReceiver()
        receiver.register_defaults()
        results = receiver.fetch_all()
        merged = receiver.aggregate()
        # merged['data'] is a numpy array ready for r_filter()
    """

    def __init__(self, max_workers: int = MAX_WORKERS,
                 timeout: int = DEFAULT_TIMEOUT):
        self.sources: Dict[str, SourceConfig] = {}
        self.max_workers = max_workers
        self.timeout = timeout
        self._last_results: Dict[str, SourceResult] = {}

    def register_source(self, name: str, fetch_fn: Callable,
                        parse_fn: Optional[Callable] = None,
                        priority: int = 0, enabled: bool = True,
                        timeout: Optional[int] = None):
        """Register a data source for aggregation.

        Args:
            name: Unique source identifier.
            fetch_fn: Callable that returns raw data (no arguments).
            parse_fn: Callable(raw) -> List[float]. Defaults to _parse_numeric_list.
            priority: Higher priority sources are fetched first.
            enabled: Whether this source is active.
            timeout: Per-source timeout override.
        """
        self.sources[name] = SourceConfig(
            name=name,
            fetch_fn=fetch_fn,
            parse_fn=parse_fn or _parse_numeric_list,
            enabled=enabled,
            priority=priority,
            timeout=timeout or self.timeout,
        )
        logger.debug("Registered source: %s (priority=%d)", name, priority)

    def unregister_source(self, name: str):
        """Remove a registered source."""
        self.sources.pop(name, None)

    def enable_source(self, name: str, enabled: bool = True):
        """Enable or disable a source."""
        if name in self.sources:
            self.sources[name].enabled = enabled

    def _fetch_single(self, config: SourceConfig) -> SourceResult:
        """Fetch data from a single source with retry and error handling."""
        result = SourceResult(name=config.name)
        t0 = time.time()

        for attempt in range(MAX_RETRIES):
            try:
                raw = config.fetch_fn()
                result.raw = raw
                result.data = config.parse_fn(raw)
                result.n_values = len(result.data)
                result.success = True
                result.retries = attempt
                result.elapsed_s = time.time() - t0
                logger.info(
                    "Source %s: %d values in %.1fs (attempt %d)",
                    config.name, result.n_values, result.elapsed_s, attempt + 1,
                )
                return result

            except urllib.error.URLError as e:
                # Connection timeout or network error -> retry with backoff
                wait = RETRY_BACKOFF_BASE ** attempt
                logger.warning(
                    "Source %s: connection error (attempt %d/%d): %s. "
                    "Retrying in %.1fs...",
                    config.name, attempt + 1, MAX_RETRIES, e, wait,
                )
                if attempt < MAX_RETRIES - 1:
                    time.sleep(wait)

            except urllib.error.HTTPError as e:
                # HTTP errors (4xx, 5xx) -> log and skip
                logger.error(
                    "Source %s: HTTP %d %s (attempt %d/%d)",
                    config.name, e.code, e.reason, attempt + 1, MAX_RETRIES,
                )
                if e.code >= 500 and attempt < MAX_RETRIES - 1:
                    wait = RETRY_BACKOFF_BASE ** attempt
                    time.sleep(wait)
                else:
                    # Client errors (4xx) -> don't retry
                    break

            except (json.JSONDecodeError, ValueError, KeyError) as e:
                # Parse errors -> log and skip record
                logger.error(
                    "Source %s: parse error: %s",
                    config.name, e,
                )
                result.error = f"parse error: {e}"
                break

            except Exception as e:
                # Unexpected errors -> log and continue
                logger.error(
                    "Source %s: unexpected error (attempt %d/%d): %s",
                    config.name, attempt + 1, MAX_RETRIES, e,
                )
                if attempt < MAX_RETRIES - 1:
                    wait = RETRY_BACKOFF_BASE ** attempt
                    time.sleep(wait)

        result.elapsed_s = time.time() - t0
        if not result.error:
            result.error = "max retries exceeded"
        logger.warning("Source %s: FAILED after %d attempts", config.name, MAX_RETRIES)
        return result

    def fetch_all(self, parallel: bool = True) -> Dict[str, SourceResult]:
        """Fetch data from all enabled sources.

        Args:
            parallel: If True, fetch sources in parallel threads.

        Returns:
            Dict mapping source name to SourceResult.
        """
        active = {
            name: cfg for name, cfg in self.sources.items()
            if cfg.enabled
        }

        if not active:
            logger.warning("No enabled sources registered")
            return {}

        results: Dict[str, SourceResult] = {}
        sorted_sources = sorted(
            active.values(),
            key=lambda s: -s.priority,
        )

        logger.info("Fetching %d sources (%s)",
                     len(sorted_sources),
                     "parallel" if parallel else "sequential")

        if parallel and len(sorted_sources) > 1:
            with ThreadPoolExecutor(max_workers=self.max_workers) as executor:
                future_to_name = {
                    executor.submit(self._fetch_single, cfg): cfg.name
                    for cfg in sorted_sources
                }
                for future in as_completed(future_to_name):
                    name = future_to_name[future]
                    try:
                        results[name] = future.result()
                    except Exception as e:
                        logger.error("Source %s: executor error: %s", name, e)
                        results[name] = SourceResult(
                            name=name, error=str(e),
                        )
        else:
            for cfg in sorted_sources:
                results[cfg.name] = self._fetch_single(cfg)

        self._last_results = results

        # Summary log
        n_ok = sum(1 for r in results.values() if r.success)
        n_fail = len(results) - n_ok
        total_vals = sum(r.n_values for r in results.values())
        logger.info(
            "Fetch complete: %d/%d sources OK, %d total values",
            n_ok, len(results), total_vals,
        )
        if n_fail > 0:
            failed = [r.name for r in results.values() if not r.success]
            logger.warning("Failed sources: %s", ", ".join(failed))

        return results

    def aggregate(self, results: Optional[Dict[str, SourceResult]] = None,
                  normalize: bool = True) -> Dict[str, Any]:
        """Merge all fetched data into a single numeric stream.

        Args:
            results: Source results to aggregate. Uses last fetch if None.
            normalize: If True, z-score normalize each source before merging.

        Returns:
            Dict with 'data' (numpy array), 'sources' (contributing source names),
            'n_total' (total values), 'per_source' (breakdown).
        """
        if results is None:
            results = self._last_results

        if not results:
            return {
                'data': np.array([], dtype=float),
                'sources': [],
                'n_total': 0,
                'per_source': {},
            }

        per_source = {}
        all_data = []

        for name, result in results.items():
            if not result.success or result.n_values == 0:
                continue

            arr = np.array(result.data, dtype=float)
            arr = arr[np.isfinite(arr)]

            if len(arr) == 0:
                continue

            if normalize and len(arr) > 1:
                std = np.std(arr)
                if std > 0:
                    arr = (arr - np.mean(arr)) / std

            per_source[name] = {
                'n_values': len(arr),
                'mean': float(np.mean(arr)),
                'std': float(np.std(arr)),
                'min': float(np.min(arr)),
                'max': float(np.max(arr)),
            }
            all_data.append(arr)

        if all_data:
            merged = np.concatenate(all_data)
        else:
            merged = np.array([], dtype=float)

        return {
            'data': merged,
            'sources': list(per_source.keys()),
            'n_total': len(merged),
            'per_source': per_source,
        }

    def stream(self, interval: float = 60.0,
               max_iterations: Optional[int] = None) -> Generator:
        """Continuous data stream — fetch, aggregate, yield in a loop.

        Args:
            interval: Seconds between fetch cycles.
            max_iterations: Stop after N iterations (None = infinite).

        Yields:
            Aggregated data dicts (same format as aggregate()).
        """
        iteration = 0
        while max_iterations is None or iteration < max_iterations:
            try:
                self.fetch_all(parallel=True)
                result = self.aggregate()
                result['iteration'] = iteration
                result['timestamp'] = time.time()
                yield result
            except Exception as e:
                logger.error("Stream iteration %d failed: %s", iteration, e)
                yield {
                    'data': np.array([], dtype=float),
                    'sources': [],
                    'n_total': 0,
                    'error': str(e),
                    'iteration': iteration,
                    'timestamp': time.time(),
                }

            iteration += 1
            if max_iterations is None or iteration < max_iterations:
                time.sleep(interval)

    # ──────────────────────────────────────────────────────────────
    # Default Source Registration
    # ──────────────────────────────────────────────────────────────

    def register_defaults(self):
        """Register all built-in SEDI data sources.

        Sources are grouped by type:
        - SETI: Breakthrough Listen, ATA, SETI databases, NRAO, Pictor, MeerKAT
        - Astronomy: Gaia, MWA, NASA, exoplanets, CMB
        - Physics: LIGO, CERN, earthquake
        - Misc: Bitcoin, OEIS, RTL-SDR (if hardware present)
        """
        # -- SETI databases (hardcoded, always available) --
        try:
            from . import seti_databases
            self.register_source(
                'seti-candidates',
                fetch_fn=seti_databases.get_candidate_signals,
                priority=10,
            )
            if hasattr(seti_databases, 'get_wow_signal'):
                self.register_source(
                    'wow-signal',
                    fetch_fn=seti_databases.get_wow_signal,
                    priority=10,
                )
        except ImportError:
            logger.warning("seti_databases module not available")

        # -- Breakthrough Listen --
        try:
            from . import breakthrough_listen as bl
            self.register_source(
                'breakthrough-listen',
                fetch_fn=bl.fetch_target_catalog,
                parse_fn=_parse_event_list,
                priority=9,
            )
        except ImportError:
            logger.warning("breakthrough_listen module not available")

        # -- Allen Telescope Array --
        try:
            from . import ata
            self.register_source(
                'ata-atats',
                fetch_fn=lambda: ata.fetch_atats_catalog(limit=500),
                parse_fn=_parse_event_list,
                priority=8,
            )
        except ImportError:
            logger.warning("ata module not available")

        # -- LIGO gravitational waves --
        try:
            from . import ligo
            self.register_source(
                'ligo-catalog',
                fetch_fn=ligo.fetch_event_catalog,
                parse_fn=_parse_event_list,
                priority=7,
            )
        except ImportError:
            logger.warning("ligo module not available")

        # -- Gaia DR3 nearby stars --
        try:
            from . import gaia_seti
            self.register_source(
                'gaia-nearby',
                fetch_fn=lambda: gaia_seti.fetch_nearby_stars(distance_pc=50),
                priority=6,
            )
        except ImportError:
            logger.warning("gaia_seti module not available")

        # -- NASA sources --
        try:
            from . import nasa
            self.register_source(
                'nasa-solar-flares',
                fetch_fn=nasa.fetch_solar_flares,
                parse_fn=_parse_event_list,
                priority=5,
            )
            self.register_source(
                'nasa-neo',
                fetch_fn=nasa.fetch_neo,
                parse_fn=_parse_event_list,
                priority=5,
            )
        except ImportError:
            logger.warning("nasa module not available")

        # -- Exoplanet Archive --
        try:
            from . import exoplanet
            self.register_source(
                'exoplanet-confirmed',
                fetch_fn=exoplanet.fetch_confirmed_planets,
                parse_fn=_parse_event_list,
                priority=5,
            )
        except ImportError:
            logger.warning("exoplanet module not available")

        # -- USGS Earthquake --
        try:
            from . import earthquake
            self.register_source(
                'earthquake',
                fetch_fn=earthquake.fetch_earthquakes,
                parse_fn=_parse_event_list,
                priority=4,
            )
        except ImportError:
            logger.warning("earthquake module not available")

        # -- NRAO VLA --
        try:
            from . import nrao
            self.register_source(
                'nrao-calibrators',
                fetch_fn=nrao.fetch_calibrator_data,
                parse_fn=_parse_event_list,
                priority=4,
            )
        except ImportError:
            logger.warning("nrao module not available")

        # -- Pictor Radio --
        try:
            from . import pictor
            self.register_source(
                'pictor-h1',
                fetch_fn=pictor.fetch_observation,
                priority=4,
            )
        except ImportError:
            logger.warning("pictor module not available")

        # -- MeerKAT --
        try:
            from . import meerkat
            self.register_source(
                'meerkat-params',
                fetch_fn=meerkat.get_system_parameters,
                priority=3,
            )
        except ImportError:
            logger.warning("meerkat module not available")

        # -- MWA --
        try:
            from . import mwa
            self.register_source(
                'mwa-survey',
                fetch_fn=mwa.get_survey_data,
                priority=3,
            )
        except ImportError:
            logger.warning("mwa module not available")

        # -- SETI Archive (VizieR) --
        try:
            from . import seti_archive
            self.register_source(
                'seti-habcat',
                fetch_fn=seti_archive.fetch_habcat,
                parse_fn=_parse_event_list,
                priority=3,
            )
        except ImportError:
            logger.warning("seti_archive module not available")

        # -- Bitcoin (financial signal) --
        try:
            from . import bitcoin
            self.register_source(
                'bitcoin-price',
                fetch_fn=lambda: bitcoin.fetch_price_history()
                    if hasattr(bitcoin, 'fetch_price_history')
                    else bitcoin.fetch_current_price(),
                priority=1,
            )
        except ImportError:
            logger.warning("bitcoin module not available")

        # -- OEIS (mathematical sequences) --
        try:
            from . import oeis
            self.register_source(
                'oeis-perfect-numbers',
                fetch_fn=lambda: oeis.fetch_sequence('A000396')
                    if hasattr(oeis, 'fetch_sequence') else [],
                priority=1,
            )
        except ImportError:
            logger.warning("oeis module not available")

        n_registered = len(self.sources)
        logger.info("Registered %d default sources", n_registered)
        return n_registered

    # ──────────────────────────────────────────────────────────────
    # Status / Diagnostics
    # ──────────────────────────────────────────────────────────────

    def status(self) -> Dict[str, Any]:
        """Return current status of all sources."""
        return {
            'n_sources': len(self.sources),
            'n_enabled': sum(1 for s in self.sources.values() if s.enabled),
            'sources': {
                name: {
                    'enabled': cfg.enabled,
                    'priority': cfg.priority,
                    'last_success': (
                        self._last_results[name].success
                        if name in self._last_results else None
                    ),
                    'last_n_values': (
                        self._last_results[name].n_values
                        if name in self._last_results else None
                    ),
                    'last_error': (
                        self._last_results[name].error
                        if name in self._last_results else None
                    ),
                }
                for name, cfg in self.sources.items()
            },
        }

    def print_status(self):
        """Print human-readable status report."""
        st = self.status()
        print(f"\n{'='*60}")
        print(f"  R-spectrum Receiver Status")
        print(f"  Sources: {st['n_enabled']}/{st['n_sources']} enabled")
        print(f"{'='*60}\n")
        print(f"  {'Source':<30s}  {'On':>3s}  {'Pri':>3s}  {'OK?':>4s}  {'N':>6s}  Error")
        print(f"  {'-'*30}  {'---':>3s}  {'---':>3s}  {'----':>4s}  {'------':>6s}  -----")
        for name, info in st['sources'].items():
            on = 'Y' if info['enabled'] else 'N'
            pri = str(info['priority'])
            ok = (
                'Y' if info['last_success'] is True
                else 'N' if info['last_success'] is False
                else '-'
            )
            n = str(info['last_n_values']) if info['last_n_values'] is not None else '-'
            err = info['last_error'] or ''
            print(f"  {name:<30s}  {on:>3s}  {pri:>3s}  {ok:>4s}  {n:>6s}  {err}")
        print()


# ──────────────────────────────────────────────────────────────────
# Convenience Functions
# ──────────────────────────────────────────────────────────────────

def create_receiver(register_defaults: bool = True) -> RSpectrumReceiver:
    """Create and configure a receiver with all default sources."""
    receiver = RSpectrumReceiver()
    if register_defaults:
        receiver.register_defaults()
    return receiver


def quick_scan(verbose: bool = True) -> Dict[str, Any]:
    """Quick one-shot fetch + aggregate + R-filter scan.

    Returns aggregated data with R-filter results.
    """
    receiver = create_receiver()
    if verbose:
        print("R-spectrum Receiver: fetching all sources...")

    receiver.fetch_all(parallel=True)
    agg = receiver.aggregate()

    if verbose:
        receiver.print_status()
        print(f"  Aggregated: {agg['n_total']} values from "
              f"{len(agg['sources'])} sources\n")

    if agg['n_total'] >= 6:
        try:
            from ..filter import r_filter
            agg['r_filter'] = r_filter(agg['data'])
        except Exception as e:
            logger.error("R-filter failed: %s", e)
            agg['r_filter'] = None
    else:
        agg['r_filter'] = None

    return agg


if __name__ == '__main__':
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s [%(name)s] %(levelname)s: %(message)s',
    )
    quick_scan(verbose=True)
