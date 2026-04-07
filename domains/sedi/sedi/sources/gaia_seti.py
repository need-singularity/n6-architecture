"""Gaia DR3 — ESA Gaia mission data for SETI target selection.

TAP: https://gea.esac.esa.int/tap-server/tap/sync
Free, no key needed. ADQL queries against Gaia DR3 catalog.

We query nearby stars with radial velocity data — these are prime
SETI targets because distance and velocity are well-characterized.
"""
import json
import urllib.request
import urllib.parse
from typing import List, Dict, Optional

GAIA_TAP = "https://gea.esac.esa.int/tap-server/tap/sync"


def query_tap(adql: str, fmt: str = 'json', timeout: int = 60) -> Optional[List]:
    """Execute ADQL query against Gaia TAP service."""
    params = urllib.parse.urlencode({
        'REQUEST': 'doQuery',
        'LANG': 'ADQL',
        'FORMAT': fmt,
        'QUERY': adql,
    })
    url = f"{GAIA_TAP}?{params}"
    try:
        req = urllib.request.Request(url, headers={'User-Agent': 'SEDI/0.1'})
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            data = json.loads(resp.read())
            # Gaia TAP returns data in various formats
            if isinstance(data, list):
                return data
            if isinstance(data, dict):
                # Handle VOTable-like JSON response
                if 'data' in data:
                    return data['data']
                if 'metadata' in data and 'data' in data:
                    return data['data']
            return data
    except Exception as e:
        print(f"  [gaia] TAP error: {e}")
    return None


def fetch_nearby_stars(distance_pc: float = 100, limit: int = 500) -> List[float]:
    """Fetch nearby stars with radial velocity from Gaia DR3.

    Args:
        distance_pc: Maximum distance in parsecs.
        limit: Maximum number of results.

    Returns:
        List of float values (parallax, RV, magnitudes) for scanning.
    """
    adql = f"""
        SELECT TOP {limit}
            source_id, parallax, radial_velocity,
            phot_g_mean_mag, phot_bp_mean_mag, phot_rp_mean_mag,
            teff_gspphot, logg_gspphot
        FROM gaiadr3.gaia_source
        WHERE parallax > {1000.0 / distance_pc}
          AND radial_velocity IS NOT NULL
          AND phot_g_mean_mag < 12
        ORDER BY parallax DESC
    """
    result = query_tap(adql.strip())

    if result and isinstance(result, list) and len(result) > 0:
        values = []
        for row in result:
            if isinstance(row, dict):
                for key in ['parallax', 'radial_velocity', 'phot_g_mean_mag',
                            'phot_bp_mean_mag', 'phot_rp_mean_mag',
                            'teff_gspphot', 'logg_gspphot']:
                    val = row.get(key)
                    if val is not None:
                        try:
                            values.append(float(val))
                        except (ValueError, TypeError):
                            pass
            elif isinstance(row, (list, tuple)):
                for val in row:
                    if val is not None:
                        try:
                            values.append(float(val))
                        except (ValueError, TypeError):
                            pass
        if values:
            print(f"  [gaia] Fetched {len(values)} values from {len(result)} nearby stars")
            return values

    # Fallback: hardcoded nearby star data (well-known SETI targets)
    print("  [gaia] TAP unavailable, using reference nearby star data")
    nearby_stars = [
        # parallax(mas), RV(km/s), G_mag, BP_mag, RP_mag, Teff(K), logg
        [768.07, -21.7, 11.13, 12.95, 9.45, 3042, 5.20],   # Proxima Cen
        [747.17, -22.4, 0.00, 0.24, -0.47, 5790, 4.30],    # Alpha Cen A
        [747.17, -18.6, 1.33, 1.81, 0.69, 5260, 4.37],     # Alpha Cen B
        [546.98, 245.2, 9.54, 11.18, 8.16, 3222, 5.10],    # Barnard's Star
        [379.21, -110.6, 7.49, 8.74, 6.32, 3500, 4.96],    # Wolf 359
        [373.70, -16.7, 5.22, 5.81, 4.42, 3828, 4.83],     # Lalande 21185
        [286.99, 5.2, 3.73, 4.20, 3.08, 4015, 4.69],       # Sirius A
        [255.66, 19.7, 7.65, 9.08, 6.35, 3348, 4.99],      # Ross 154
        [316.96, -60.2, 8.09, 9.68, 6.66, 3163, 5.08],     # Ross 248
        [284.56, -7.6, 3.72, 4.19, 3.04, 5084, 4.53],      # Epsilon Eridani
    ]
    values = []
    for star in nearby_stars:
        values.extend([float(v) for v in star])
    return values


def fetch_seti_candidates() -> List[float]:
    """Fetch Gaia data for known SETI candidate stars.

    Queries Gaia for stars near known SETI targets.
    Returns numeric data for n=6 pattern scanning.
    """
    # Known SETI targets with approximate Gaia source IDs or coordinates
    seti_targets = [
        ('KIC_8462852', "ra BETWEEN 301.56 AND 301.57 AND dec BETWEEN 44.45 AND 44.46"),  # Tabby's Star
        ('Ross_128', "ra BETWEEN 176.93 AND 176.94 AND dec BETWEEN 0.79 AND 0.80"),
        ('TRAPPIST_1', "ra BETWEEN 346.62 AND 346.63 AND dec BETWEEN -5.04 AND -5.03"),
        ('Kepler_160', "ra BETWEEN 293.08 AND 293.10 AND dec BETWEEN 42.26 AND 42.28"),
    ]

    all_values = []
    for name, constraint in seti_targets:
        adql = f"""
            SELECT TOP 5
                parallax, radial_velocity, phot_g_mean_mag,
                phot_bp_mean_mag, phot_rp_mean_mag
            FROM gaiadr3.gaia_source
            WHERE {constraint}
              AND parallax > 0
            ORDER BY phot_g_mean_mag
        """
        result = query_tap(adql.strip())
        if result:
            for row in result:
                if isinstance(row, dict):
                    for v in row.values():
                        if v is not None:
                            try:
                                all_values.append(float(v))
                            except (ValueError, TypeError):
                                pass
                elif isinstance(row, (list, tuple)):
                    for v in row:
                        if v is not None:
                            try:
                                all_values.append(float(v))
                            except (ValueError, TypeError):
                                pass

    if not all_values:
        # Fallback: known photometric values for SETI targets
        print("  [gaia] Using reference SETI candidate data")
        all_values = [
            # Tabby's Star: parallax, RV, G, BP, RP
            2.50, -4.6, 11.71, 11.96, 11.30,
            # Ross 128
            296.30, -31.2, 11.13, 12.72, 9.72,
            # TRAPPIST-1
            80.45, -56.3, 17.35, 19.79, 15.27,
            # Kepler-160
            1.87, -27.8, 13.00, 13.40, 12.40,
        ]

    print(f"  [gaia] Loaded {len(all_values)} values for SETI candidates")
    return all_values
