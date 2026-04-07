"""NASA Exoplanet Archive — confirmed exoplanets with orbital data.

API: https://exoplanetarchive.ipac.caltech.edu/TAP/
Free, no key needed. TAP (Table Access Protocol) service.

We look for n=6 patterns in orbital parameters:
- period ratios between planets in multi-planet systems
- semi-major axis ratios matching σ, φ, τ constants
- systems with exactly 6 planets
"""
import json
import logging
import urllib.request
import urllib.parse
from typing import List, Dict, Optional

from ._connection import fetch_json as _conn_fetch_json, logger

TAP_BASE = "https://exoplanetarchive.ipac.caltech.edu/TAP/sync"

# Columns of interest for n=6 analysis
PLANET_COLS = (
    "pl_name,hostname,pl_orbper,pl_orbsmax,pl_rade,pl_bmasse,"
    "pl_eqt,sy_snum,sy_pnum,disc_facility"
)


def query_tap(adql, fmt="json", timeout=30):
    """Execute an ADQL query against the Exoplanet Archive TAP service."""
    params = urllib.parse.urlencode({
        'query': adql,
        'format': fmt,
    })
    url = f"{TAP_BASE}?{params}"
    return _conn_fetch_json(url, source_tag='exoplanet', timeout=timeout)


def fetch_confirmed_planets(limit=1000):
    """Fetch confirmed exoplanets with orbital data."""
    adql = f"""
        SELECT {PLANET_COLS}
        FROM ps
        WHERE pl_orbper IS NOT NULL
          AND pl_orbsmax IS NOT NULL
          AND default_flag = 1
        ORDER BY hostname, pl_orbper
    """
    if limit:
        adql += f" FETCH FIRST {limit} ROWS ONLY"
    return query_tap(adql.strip())


def fetch_multiplanet_systems(min_planets=3):
    """Fetch systems with multiple planets — best for ratio analysis."""
    adql = f"""
        SELECT {PLANET_COLS}
        FROM ps
        WHERE sy_pnum >= {min_planets}
          AND pl_orbper IS NOT NULL
          AND pl_orbsmax IS NOT NULL
          AND default_flag = 1
        ORDER BY hostname, pl_orbper
    """
    return query_tap(adql.strip())


def fetch_six_planet_systems():
    """Fetch systems with exactly 6 planets — direct n=6 connection."""
    adql = f"""
        SELECT {PLANET_COLS}
        FROM ps
        WHERE sy_pnum = 6
          AND pl_orbper IS NOT NULL
          AND default_flag = 1
        ORDER BY hostname, pl_orbper
    """
    return query_tap(adql.strip())


def fetch_habitable_zone(t_min=200, t_max=350):
    """Fetch planets in the habitable zone (by equilibrium temperature)."""
    adql = f"""
        SELECT {PLANET_COLS}
        FROM ps
        WHERE pl_eqt BETWEEN {t_min} AND {t_max}
          AND default_flag = 1
        ORDER BY pl_eqt
    """
    return query_tap(adql.strip())


def group_by_system(planets):
    """Group planet list by host star system."""
    systems = {}
    if not planets:
        return systems
    for p in planets:
        host = p.get('hostname', 'unknown')
        if host not in systems:
            systems[host] = []
        systems[host].append(p)
    return systems


def analyze_period_ratios(system_planets):
    """Analyze period ratios in a multi-planet system for n=6 patterns.

    Checks consecutive and all-pairs period ratios against SEDI constants.
    """
    from ..constants import RATIOS, SIGMA, PHI, TAU, N

    sorted_planets = sorted(system_planets, key=lambda p: p.get('pl_orbper', 0))
    periods = [p['pl_orbper'] for p in sorted_planets if p.get('pl_orbper')]

    if len(periods) < 2:
        return {'n6_matches': [], 'n_planets': len(periods)}

    # Extended ratios including integer multiples
    targets = dict(RATIOS)
    targets['n'] = N                    # 6
    targets['sigma'] = SIGMA            # 12
    targets['phi'] = PHI                # 2
    targets['tau'] = TAU                # 4
    targets['sigma_phi'] = SIGMA * PHI  # 24

    matches = []
    tolerance = 0.02  # 2% for orbital mechanics

    for i in range(len(periods)):
        for j in range(i + 1, len(periods)):
            ratio = periods[j] / periods[i]
            for name, target in targets.items():
                if target == 0:
                    continue
                # Check ratio and its reciprocal
                for r, label in [(ratio, f"{name}"), (1/ratio, f"1/{name}")]:
                    if abs(r - target) / target < tolerance:
                        matches.append({
                            'planets': (
                                sorted_planets[i].get('pl_name', '?'),
                                sorted_planets[j].get('pl_name', '?'),
                            ),
                            'periods': (periods[i], periods[j]),
                            'ratio': r,
                            'pattern': label,
                            'target': target,
                            'deviation_pct': abs(r - target) / target * 100,
                        })

    return {
        'n6_matches': matches,
        'n_planets': len(periods),
        'host': sorted_planets[0].get('hostname', '?'),
    }


def scan_all_systems(min_planets=3):
    """Scan all multi-planet systems for n=6 orbital patterns.

    Returns systems ranked by number of n=6 matches.
    """
    planets = fetch_multiplanet_systems(min_planets)
    if not planets:
        return []

    systems = group_by_system(planets)
    results = []

    for host, plist in systems.items():
        analysis = analyze_period_ratios(plist)
        if analysis['n6_matches']:
            results.append(analysis)

    results.sort(key=lambda r: len(r['n6_matches']), reverse=True)
    return results
