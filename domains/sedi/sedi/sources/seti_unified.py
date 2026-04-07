"""Unified SETI Scanner — aggregate all SETI data sources and scan for n=6.

Imports all SETI-related source modules, fetches available data from each,
runs full_scan() on each dataset, and ranks results by anomaly score.
"""
import time
import numpy as np
from typing import List, Dict, Any


def _safe_fetch(name, fetch_fn):
    """Safely call a fetch function, catching all exceptions."""
    try:
        data = fetch_fn()
        if data and len(data) > 0:
            return data
    except Exception as e:
        print(f"  [unified] {name}: error — {e}")
    return None


def scan_all_seti_sources(verbose: bool = True) -> List[Dict[str, Any]]:
    """Fetch data from all SETI sources and scan each for n=6 patterns.

    Returns:
        List of scan results sorted by anomaly score (highest first).
        Each entry contains source name, data size, scan result, and score.
    """
    from ..seti_scanner import full_scan

    sources = {}

    # 1. SETI Signal Databases (hardcoded — always available)
    try:
        from . import seti_databases
        sources['seti-candidates'] = lambda: seti_databases.get_candidate_signals()
        sources['wow-signal'] = lambda: seti_databases.get_wow_signal()
    except ImportError as e:
        print(f"  [unified] seti_databases import failed: {e}")

    # 2. Breakthrough Listen
    try:
        from . import breakthrough_listen as bl
        sources['breakthrough-listen-targets'] = lambda: [
            t.get('ra_deg', t.get('ra', 0)) for t in bl.fetch_target_catalog()
            if isinstance(t, dict)
        ] + [
            t.get('dec_deg', t.get('dec', 0)) for t in bl.fetch_target_catalog()
            if isinstance(t, dict)
        ]
    except ImportError as e:
        print(f"  [unified] breakthrough_listen import failed: {e}")

    # 3. Allen Telescope Array (ATATS catalog via Socrata API)
    try:
        from . import ata
        sources['ata-atats'] = lambda: ata.scan_catalog()
    except ImportError as e:
        print(f"  [unified] ata import failed: {e}")

    # 4. PICTOR Radio Telescope
    try:
        from . import pictor
        sources['pictor-h1'] = lambda: pictor.fetch_observation()
    except ImportError as e:
        print(f"  [unified] pictor import failed: {e}")

    # 5. NRAO VLA calibrators
    try:
        from . import nrao
        sources['nrao-calibrators'] = lambda: nrao.fetch_calibrator_data()
    except ImportError as e:
        print(f"  [unified] nrao import failed: {e}")

    # 6. MeerKAT system parameters
    try:
        from . import meerkat
        sources['meerkat-params'] = lambda: meerkat.get_system_parameters()
    except ImportError as e:
        print(f"  [unified] meerkat import failed: {e}")

    # 7. Gaia DR3 nearby stars
    try:
        from . import gaia_seti
        sources['gaia-nearby'] = lambda: gaia_seti.fetch_nearby_stars(distance_pc=50)
        sources['gaia-seti-candidates'] = lambda: gaia_seti.fetch_seti_candidates()
    except ImportError as e:
        print(f"  [unified] gaia_seti import failed: {e}")

    # 8. MWA survey data
    try:
        from . import mwa
        sources['mwa-survey'] = lambda: mwa.get_survey_data()
    except ImportError as e:
        print(f"  [unified] mwa import failed: {e}")

    # Run scans
    results = []
    n_sources = len(sources)

    if verbose:
        print(f"\n{'='*60}")
        print(f"  SEDI Unified SETI Scanner — {n_sources} sources")
        print(f"{'='*60}\n")

    for i, (name, fetch_fn) in enumerate(sources.items(), 1):
        if verbose:
            print(f"  [{i}/{n_sources}] Scanning: {name}")

        t0 = time.time()
        data = _safe_fetch(name, fetch_fn)

        if data is None or len(data) == 0:
            if verbose:
                print(f"         -> No data available\n")
            continue

        # Convert to float array
        try:
            arr = np.array([float(v) for v in data if v is not None], dtype=float)
            arr = arr[np.isfinite(arr)]
        except (ValueError, TypeError) as e:
            if verbose:
                print(f"         -> Data conversion error: {e}\n")
            continue

        if len(arr) < 6:
            if verbose:
                print(f"         -> Too few values ({len(arr)})\n")
            continue

        # Run full_scan
        try:
            scan_result = full_scan(arr, source_name=name)
            elapsed = time.time() - t0
            score = scan_result.get('anomaly_score', 0.0) if isinstance(scan_result, dict) else 0.0

            result_entry = {
                'source': name,
                'n_values': len(arr),
                'score': score,
                'elapsed_s': round(elapsed, 2),
                'scan': scan_result,
            }
            results.append(result_entry)

            if verbose:
                print(f"         -> {len(arr)} values, score={score:.4f}, {elapsed:.1f}s\n")

        except Exception as e:
            if verbose:
                print(f"         -> Scan error: {e}\n")

    # Sort by anomaly score (highest first)
    results.sort(key=lambda r: r['score'], reverse=True)

    if verbose:
        print(f"{'='*60}")
        print(f"  Results: {len(results)}/{n_sources} sources scanned")
        print(f"{'='*60}")
        if results:
            print(f"\n  Top sources by anomaly score:")
            for i, r in enumerate(results[:10], 1):
                print(f"    {i}. {r['source']:30s}  score={r['score']:.4f}  n={r['n_values']}")
        print()

    return results


if __name__ == '__main__':
    scan_all_seti_sources()
