"""n=6 exoplanet tracker — dedicated monitoring of top n=6 candidate systems.

Tracks period ratio convergence toward exact n=6 arithmetic constants
across priority multi-planet systems. HD 110067 (6 planets, b↔g ratio
= 6.0096 ≈ n=6) and TRAPPIST-1 (7 planets, 12 n=6 matches) are the
strongest candidates. TOI-1136 has the most precise single match
(b↔d = 3.0004, deviation 0.01%).
"""
import json
import os
import urllib.request
import urllib.parse
from datetime import datetime, timezone
from itertools import combinations
from typing import Dict, List, Optional, Tuple

from .constants import (
    N, SIGMA, PHI, TAU, SOPFR, OMEGA,
    RATIOS, DELTA_PLUS, DELTA_MINUS,
    GOLDEN_CENTER, EINSTEIN_THETA,
)
from .sources.exoplanet import query_tap, group_by_system, PLANET_COLS

# ---------------------------------------------------------------------------
# Data directory
# ---------------------------------------------------------------------------
DATA_DIR = os.path.join(os.path.dirname(os.path.dirname(__file__)), "data", "n6_tracker")
os.makedirs(DATA_DIR, exist_ok=True)

# ---------------------------------------------------------------------------
# N6_TARGETS — priority systems with coordinates
# ---------------------------------------------------------------------------
N6_TARGETS: Dict[str, dict] = {
    "HD 110067": {
        "ra": 189.84, "dec": 20.03, "dist_pc": 32.2,
        "note": "6 planets, b↔g period ratio = 6.0096 ≈ n=6",
        "priority": 1,
    },
    "TRAPPIST-1": {
        "ra": 346.63, "dec": -5.04, "dist_pc": 12.4,
        "note": "7 planets, 12 n=6 matches across all pairs",
        "priority": 1,
    },
    "TOI-1136": {
        "ra": 192.18, "dec": 64.86, "dist_pc": 84.5,
        "note": "Most precise single match: b↔d = 3.0004 (0.01% dev)",
        "priority": 1,
    },
    "GJ 876": {
        "ra": 343.32, "dec": -14.27, "dist_pc": 4.7,
        "note": "Nearby M-dwarf, 4 planets, strong mean-motion resonances",
        "priority": 2,
    },
    "V1298 Tau": {
        "ra": 60.96, "dec": 20.16, "dist_pc": 108.5,
        "note": "Young system, 4 planets, evolving resonance chain",
        "priority": 2,
    },
    "Kepler-9": {
        "ra": 284.70, "dec": 38.90, "dist_pc": 699.0,
        "note": "First confirmed TTV system, 3 planets near 2:1",
        "priority": 3,
    },
    "Kepler-235": {
        "ra": 291.45, "dec": 41.54, "dist_pc": 770.0,
        "note": "4 planets, compact multi-planet system",
        "priority": 3,
    },
    "Kepler-79": {
        "ra": 293.09, "dec": 41.09, "dist_pc": 1066.0,
        "note": "4 planets in near-resonance chain",
        "priority": 3,
    },
    "Kepler-32": {
        "ra": 287.72, "dec": 48.77, "dist_pc": 305.0,
        "note": "5 planets around M-dwarf, compact system",
        "priority": 3,
    },
    "HD 219134": {
        "ra": 348.33, "dec": 57.17, "dist_pc": 6.5,
        "note": "6 planets, very nearby bright star",
        "priority": 2,
    },
    "HD 10180": {
        "ra": 24.45, "dec": -60.47, "dist_pc": 39.0,
        "note": "Up to 9 planets claimed, rich resonance structure",
        "priority": 2,
    },
    # --- New candidates added 2026-03-28 via n6_hunt scan ---
    "Kepler-223": {
        "ra": 298.32, "dec": 47.28, "dist_pc": 1859.7,
        "note": "4 planets, score=11, b↔c 4/3 (0.0035%), resonance chain",
        "priority": 1,
    },
    "HIP 41378": {
        "ra": 126.62, "dec": 10.08, "dist_pc": 106.3,
        "note": "6 planets, score=9, b↔g tau=4 (0.37%), complete ladder candidate",
        "priority": 1,
    },
    "TOI-178": {
        "ra": 7.30, "dec": -30.45, "dist_pc": 62.7,
        "note": "6 planets, score=9, Laplace resonance chain",
        "priority": 1,
    },
    "K2-138": {
        "ra": 348.95, "dec": -10.85, "dist_pc": 202.6,
        "note": "6 planets, score=8, unbroken resonance chain",
        "priority": 2,
    },
    "Kepler-176": {
        "ra": 294.67, "dec": 43.85, "dist_pc": 527.3,
        "note": "4 planets, score=8, delta_minus 1/4 match (0.25%)",
        "priority": 2,
    },
    "Kepler-305": {
        "ra": 299.22, "dec": 40.34, "dist_pc": 868.5,
        "note": "4 planets, score=7, n/tau 3/2 match",
        "priority": 2,
    },
    "Kepler-326": {
        "ra": 294.33, "dec": 46.00, "dist_pc": 487.7,
        "note": "3 planets, score=7, sigma/tau=3 match (0.32%)",
        "priority": 3,
    },
    "Kepler-85": {
        "ra": 290.97, "dec": 45.29, "dist_pc": 765.0,
        "note": "4 planets, score=7, n/tau 3/2 match (0.48%)",
        "priority": 2,
    },
    "HD 158259": {
        "ra": 261.35, "dec": 52.79, "dist_pc": 27.0,
        "note": "5 planets, score=6, very nearby, n/tau 3/2 matches",
        "priority": 2,
    },
    "HD 23472": {
        "ra": 55.46, "dec": -62.77, "dist_pc": 39.0,
        "note": "5 planets, score=6, phi=2 match (0.58%)",
        "priority": 2,
    },
    "K2-384": {
        "ra": 20.50, "dec": 0.75, "dist_pc": 82.7,
        "note": "5 planets, score=5, COMPLETE LADDER (sigma/tau + n=6 + phi)",
        "priority": 1,
    },
    "KOI-351": {
        "ra": 284.43, "dec": 49.31, "dist_pc": 848.3,
        "note": "8 planets (Kepler-90), score=5, most planets known",
        "priority": 2,
    },
    "Kepler-80": {
        "ra": 296.11, "dec": 39.98, "dist_pc": 369.5,
        "note": "6 planets, score=5, resonance chain",
        "priority": 2,
    },
    # --- Iteration-2 candidates: non-trivial ratio matches (2026-03-28) ---
    "Kepler-50": {
        "ra": 291.57, "dec": 41.81, "dist_pc": 849.0,
        "note": "2 planets, sopfr/n=5/6 match at 0.015% — most precise non-trivial ratio",
        "priority": 2,
    },
    "TOI-270": {
        "ra": 69.99, "dec": -51.95, "dist_pc": 22.5,
        "note": "3 planets, score=4, nearby M-dwarf, phi/tau (1/2) match",
        "priority": 2,
    },
    "Kepler-292": {
        "ra": 292.50, "dec": 42.10, "dist_pc": 500.0,
        "note": "5 planets, score=4, golden_center (1/e) match (0.57%)",
        "priority": 3,
    },
    "Kepler-444": {
        "ra": 282.09, "dec": 41.63, "dist_pc": 35.7,
        "note": "5 planets, score=4, oldest known (~11 Gyr), 5/4 match (0.09%)",
        "priority": 2,
    },
}

# ---------------------------------------------------------------------------
# n=6 target ratios — expanded set for exoplanet period matching
# ---------------------------------------------------------------------------
N6_RATIOS: Dict[str, float] = {
    # Core SEDI constants
    "delta_plus (1/6)": DELTA_PLUS,
    "delta_minus (1/4)": DELTA_MINUS,
    "phi/tau (1/2)": PHI / TAU,
    "sopfr/n (5/6)": SOPFR / N,
    "golden_center (1/e)": GOLDEN_CENTER,
    "sigma/tau (3)": SIGMA / TAU,
    # Direct n=6 family
    "n (6)": float(N),
    "sigma (12)": float(SIGMA),
    "phi (2)": float(PHI),
    "tau (4)": float(TAU),
    "sopfr (5)": float(SOPFR),
    "sigma*phi (24)": float(SIGMA * PHI),
    "n/phi (3)": float(N / PHI),
    "n/tau (3/2)": float(N / TAU),
    "sigma/n (2)": float(SIGMA / N),
    # Einstein theta — sqrt(sigma/(sigma-tau)) = sqrt(3/2)
    "einstein_theta sqrt(3/2)": EINSTEIN_THETA,
    # Small integer ratios common in mean-motion resonance
    "3/2": 1.5,
    "4/3": 4.0 / 3.0,
    "5/3": 5.0 / 3.0,
    "5/4": 1.25,
    "7/6": 7.0 / 6.0,
}

# Tolerance for matching (fraction)
MATCH_TOLERANCE = 0.02  # 2%


# ---------------------------------------------------------------------------
# Core functions
# ---------------------------------------------------------------------------

def fetch_latest_periods(hostname: str) -> Optional[List[dict]]:
    """Query NASA Exoplanet Archive for current best periods of a system.

    Returns list of dicts with planet name, period, and uncertainties,
    sorted by orbital period.
    """
    adql = f"""
        SELECT pl_name, pl_orbper, pl_orbpererr1, pl_orbpererr2,
               pl_orbsmax, pl_rade, pl_bmasse, sy_pnum, disc_facility
        FROM ps
        WHERE hostname = '{hostname}'
          AND pl_orbper IS NOT NULL
          AND default_flag = 1
        ORDER BY pl_orbper
    """
    rows = query_tap(adql.strip())
    if not rows:
        return None
    return rows


def compute_n6_matrix(periods: List[float],
                      names: Optional[List[str]] = None,
                      tolerance: float = MATCH_TOLERANCE
                      ) -> dict:
    """Compute full period-ratio matrix and identify all n=6 constant matches.

    Parameters
    ----------
    periods : sorted list of orbital periods (days)
    names   : optional planet letter labels
    tolerance : fractional tolerance for matching

    Returns
    -------
    dict with keys:
        ratio_matrix : list of dicts {i, j, name_i, name_j, ratio}
        matches      : list of dicts for each n=6 match found
        n6_score     : total number of matches (primary ranking metric)
        best_match   : the single match with lowest deviation
    """
    if names is None:
        names = [chr(ord('b') + i) for i in range(len(periods))]

    ratio_matrix = []
    matches = []

    for idx_i, idx_j in combinations(range(len(periods)), 2):
        ratio = periods[idx_j] / periods[idx_i]
        entry = {
            "i": idx_i, "j": idx_j,
            "name_i": names[idx_i], "name_j": names[idx_j],
            "period_i": periods[idx_i], "period_j": periods[idx_j],
            "ratio": ratio,
        }
        ratio_matrix.append(entry)

        # Check against every target ratio (and reciprocal)
        for label, target in N6_RATIOS.items():
            if target == 0:
                continue
            for r, direction in [(ratio, "direct"), (1.0 / ratio, "inverse")]:
                dev = abs(r - target) / target
                if dev < tolerance:
                    matches.append({
                        "pair": f"{names[idx_i]}↔{names[idx_j]}",
                        "periods": (periods[idx_i], periods[idx_j]),
                        "ratio": ratio if direction == "direct" else 1.0 / ratio,
                        "pattern": label,
                        "target": target,
                        "direction": direction,
                        "deviation_pct": dev * 100,
                    })

    matches.sort(key=lambda m: m["deviation_pct"])
    best = matches[0] if matches else None

    return {
        "ratio_matrix": ratio_matrix,
        "matches": matches,
        "n6_score": len(matches),
        "best_match": best,
    }


def _load_history(hostname: str) -> dict:
    """Load stored tracking history for a system."""
    path = os.path.join(DATA_DIR, f"{hostname.replace(' ', '_')}.json")
    if os.path.exists(path):
        with open(path) as f:
            return json.load(f)
    return {"hostname": hostname, "snapshots": []}


def _save_history(hostname: str, data: dict):
    """Save tracking history for a system."""
    path = os.path.join(DATA_DIR, f"{hostname.replace(' ', '_')}.json")
    with open(path, "w") as f:
        json.dump(data, f, indent=2)


def track_precision_history(hostname: str) -> dict:
    """Compare current periods vs stored history, track ratio convergence.

    Fetches latest periods, computes n=6 matrix, stores a snapshot, and
    compares deviation trends across snapshots to see if ratios converge
    toward exact n=6 values over time.

    Returns
    -------
    dict with:
        current   : current n6 matrix result
        snapshots : number of stored snapshots
        trends    : per-match deviation trend (list of {pattern, pair, devs})
    """
    planets = fetch_latest_periods(hostname)
    if not planets:
        return {"error": f"No data for {hostname}"}

    periods = [p["pl_orbper"] for p in planets]
    names = [p["pl_name"].split()[-1] for p in planets]  # letter only

    matrix = compute_n6_matrix(periods, names)

    # Build snapshot
    snapshot = {
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "periods": {n: p for n, p in zip(names, periods)},
        "n6_score": matrix["n6_score"],
        "matches": matrix["matches"],
    }

    history = _load_history(hostname)
    history["snapshots"].append(snapshot)
    _save_history(hostname, history)

    # Compute deviation trends across snapshots
    trends = {}
    for snap in history["snapshots"]:
        for m in snap.get("matches", []):
            key = (m["pattern"], m["pair"])
            if key not in trends:
                trends[key] = {"pattern": m["pattern"], "pair": m["pair"], "devs": []}
            trends[key]["devs"].append(m["deviation_pct"])

    return {
        "current": matrix,
        "snapshots": len(history["snapshots"]),
        "trends": list(trends.values()),
    }


def generate_report(hostname: str) -> str:
    """Full report: periods, ratio matrix, n=6 matches, precision trend.

    Returns a formatted string report.
    """
    planets = fetch_latest_periods(hostname)
    if not planets:
        return f"[n6_tracker] No data found for {hostname}"

    periods = [p["pl_orbper"] for p in planets]
    names = [p["pl_name"].split()[-1] for p in planets]

    matrix = compute_n6_matrix(periods, names)
    history = _load_history(hostname)

    lines = []
    lines.append("=" * 70)
    lines.append(f"  n=6 TRACKER REPORT: {hostname}")
    lines.append(f"  Generated: {datetime.now(timezone.utc).strftime('%Y-%m-%d %H:%M UTC')}")
    if hostname in N6_TARGETS:
        t = N6_TARGETS[hostname]
        lines.append(f"  Coordinates: RA={t['ra']:.2f}, Dec={t['dec']:+.2f}, d={t['dist_pc']}pc")
        lines.append(f"  Note: {t['note']}")
    lines.append("=" * 70)

    # Periods
    lines.append("\n  ORBITAL PERIODS (days):")
    lines.append("  " + "-" * 40)
    for name, per, pl in zip(names, periods, planets):
        err1 = pl.get("pl_orbpererr1", "?")
        err2 = pl.get("pl_orbpererr2", "?")
        lines.append(f"    {name:>4s}  {per:12.6f}  (+{err1} / {err2})")

    # Ratio matrix
    lines.append("\n  PERIOD RATIO MATRIX:")
    lines.append("  " + "-" * 40)
    for entry in matrix["ratio_matrix"]:
        lines.append(
            f"    {entry['name_i']:>4s} / {entry['name_j']:<4s}  "
            f"ratio = {entry['ratio']:.6f}"
        )

    # n=6 matches
    lines.append(f"\n  n=6 MATCHES ({matrix['n6_score']} total):")
    lines.append("  " + "-" * 40)
    if matrix["matches"]:
        for m in matrix["matches"]:
            lines.append(
                f"    {m['pair']:>8s}  {m['pattern']:<22s}  "
                f"ratio={m['ratio']:.6f}  target={m['target']:.6f}  "
                f"dev={m['deviation_pct']:.4f}%"
            )
    else:
        lines.append("    (none found at 2% tolerance)")

    # Best match
    if matrix["best_match"]:
        b = matrix["best_match"]
        lines.append(f"\n  BEST MATCH: {b['pair']} → {b['pattern']} "
                      f"(dev={b['deviation_pct']:.4f}%)")

    # Precision trend
    n_snap = len(history.get("snapshots", []))
    lines.append(f"\n  PRECISION HISTORY: {n_snap} snapshot(s) stored")
    if n_snap >= 2:
        lines.append("  " + "-" * 40)
        # Show trend for top matches
        trends = {}
        for snap in history["snapshots"]:
            for m in snap.get("matches", []):
                key = (m["pattern"], m["pair"])
                if key not in trends:
                    trends[key] = []
                trends[key].append(m["deviation_pct"])
        for (pat, pair), devs in sorted(trends.items(), key=lambda x: x[1][-1]):
            arrow = "↓" if len(devs) > 1 and devs[-1] < devs[0] else "→"
            lines.append(f"    {pair:>8s}  {pat:<22s}  {arrow} "
                          f"{devs[0]:.4f}% → {devs[-1]:.4f}%")

    lines.append("\n" + "=" * 70)
    return "\n".join(lines)


def scan_all_targets() -> List[dict]:
    """Scan all N6_TARGETS, rank by total n=6 score.

    Returns list of dicts sorted by n6_score descending.
    """
    results = []
    for hostname, meta in N6_TARGETS.items():
        print(f"  [n6_tracker] Scanning {hostname}...")
        planets = fetch_latest_periods(hostname)
        if not planets:
            print(f"    → No data returned")
            results.append({
                "hostname": hostname,
                "priority": meta.get("priority", 9),
                "n6_score": 0,
                "n_planets": 0,
                "best_match": None,
                "note": meta.get("note", ""),
                "error": "no data",
            })
            continue

        periods = [p["pl_orbper"] for p in planets]
        names = [p["pl_name"].split()[-1] for p in planets]

        matrix = compute_n6_matrix(periods, names)
        best = matrix["best_match"]
        print(f"    → {len(periods)} planets, {matrix['n6_score']} n=6 matches"
              + (f", best: {best['pair']} {best['pattern']} "
                 f"({best['deviation_pct']:.4f}%)" if best else ""))

        results.append({
            "hostname": hostname,
            "priority": meta.get("priority", 9),
            "n6_score": matrix["n6_score"],
            "n_planets": len(periods),
            "best_match": best,
            "n_matches_by_pattern": _count_by_pattern(matrix["matches"]),
            "note": meta.get("note", ""),
        })

    results.sort(key=lambda r: r["n6_score"], reverse=True)
    return results


def _count_by_pattern(matches: list) -> Dict[str, int]:
    """Count matches grouped by pattern name."""
    counts: Dict[str, int] = {}
    for m in matches:
        p = m["pattern"]
        counts[p] = counts.get(p, 0) + 1
    return counts


def search_new_candidates(min_planets: int = 3,
                          tolerance: float = MATCH_TOLERANCE,
                          top_n: int = 20) -> List[dict]:
    """Find NEW systems not in our target list that have n=6 patterns.

    Queries all multi-planet systems from the archive, computes n=6
    matrices, and returns top scorers not already in N6_TARGETS.

    Parameters
    ----------
    min_planets : minimum planets in system
    tolerance   : fractional tolerance for matching
    top_n       : return this many top candidates

    Returns
    -------
    List of dicts with hostname, n6_score, n_planets, best_match, etc.
    """
    print(f"  [n6_tracker] Fetching all systems with >= {min_planets} planets...")
    adql = f"""
        SELECT {PLANET_COLS}
        FROM ps
        WHERE sy_pnum >= {min_planets}
          AND pl_orbper IS NOT NULL
          AND default_flag = 1
        ORDER BY hostname, pl_orbper
    """
    rows = query_tap(adql.strip())
    if not rows:
        print("  [n6_tracker] No data returned from archive")
        return []

    systems = group_by_system(rows)
    known = set(N6_TARGETS.keys())

    candidates = []
    for host, plist in systems.items():
        if host in known:
            continue

        periods = sorted([p["pl_orbper"] for p in plist if p.get("pl_orbper")])
        if len(periods) < 2:
            continue
        names = [p["pl_name"].split()[-1]
                 for p in sorted(plist, key=lambda x: x.get("pl_orbper", 0))]

        matrix = compute_n6_matrix(periods, names, tolerance=tolerance)
        if matrix["n6_score"] > 0:
            candidates.append({
                "hostname": host,
                "n6_score": matrix["n6_score"],
                "n_planets": len(periods),
                "best_match": matrix["best_match"],
                "n_matches_by_pattern": _count_by_pattern(matrix["matches"]),
            })

    candidates.sort(key=lambda c: c["n6_score"], reverse=True)
    top = candidates[:top_n]

    print(f"  [n6_tracker] Found {len(candidates)} systems with n=6 matches, "
          f"returning top {len(top)}")
    for c in top[:5]:
        b = c["best_match"]
        print(f"    {c['hostname']:20s}  score={c['n6_score']:3d}  "
              f"planets={c['n_planets']}  "
              f"best: {b['pair']} {b['pattern']} ({b['deviation_pct']:.4f}%)")
    return top


def cross_source_check(hostname: str) -> dict:
    """Check if target has Breakthrough Listen observations, TESS data, etc.

    Queries multiple catalogs and flags for cross-source observations.
    """
    result = {
        "hostname": hostname,
        "breakthrough_listen": False,
        "tess": False,
        "kepler": False,
        "radial_velocity": False,
        "facilities": [],
    }

    # Check discovery facility and observation methods from archive
    adql = f"""
        SELECT DISTINCT disc_facility, discoverymethod
        FROM ps
        WHERE hostname = '{hostname}'
          AND default_flag = 1
    """
    rows = query_tap(adql.strip())
    if rows:
        facilities = set()
        methods = set()
        for r in rows:
            fac = r.get("disc_facility", "")
            meth = r.get("discoverymethod", "")
            if fac:
                facilities.add(fac)
            if meth:
                methods.add(meth)

        result["facilities"] = sorted(facilities)

        # Flag known sources
        fac_lower = " ".join(facilities).lower()
        if "tess" in fac_lower or "transiting exoplanet survey" in fac_lower:
            result["tess"] = True
        if "kepler" in fac_lower or "k2" in fac_lower:
            result["kepler"] = True
        if "radial" in " ".join(methods).lower():
            result["radial_velocity"] = True

    # Check Breakthrough Listen target list via SEDI seti_archive if available
    try:
        from .sources.breakthrough_listen import check_target
        bl = check_target(hostname)
        if bl:
            result["breakthrough_listen"] = True
            result["bl_data"] = bl
    except (ImportError, Exception):
        # Check by proximity to known BL targets (GJ 876 is on the list)
        bl_targets = {"GJ 876", "TRAPPIST-1", "HD 219134"}
        if hostname in bl_targets:
            result["breakthrough_listen"] = True
            result["bl_note"] = "Known Breakthrough Listen target"

    # Check TESS via TIC if not already flagged
    if not result["tess"]:
        adql_tess = f"""
            SELECT pl_name
            FROM ps
            WHERE hostname = '{hostname}'
              AND disc_facility LIKE '%TESS%'
        """
        tess_rows = query_tap(adql_tess.strip())
        if tess_rows:
            result["tess"] = True

    return result


# ---------------------------------------------------------------------------
# CLI entry point
# ---------------------------------------------------------------------------
def main():
    """Run full n=6 tracker scan and generate reports."""
    import sys

    print("=" * 70)
    print("  SEDI n=6 Exoplanet Tracker")
    print("=" * 70)

    # Scan all targets
    print("\n[1/3] Scanning all priority targets...\n")
    ranked = scan_all_targets()

    print("\n" + "=" * 70)
    print("  RANKED RESULTS")
    print("=" * 70)
    for i, r in enumerate(ranked, 1):
        best = r.get("best_match")
        best_str = (f"best: {best['pair']} {best['pattern']} "
                    f"({best['deviation_pct']:.4f}%)" if best else "no matches")
        print(f"  {i:2d}. {r['hostname']:20s}  "
              f"score={r['n6_score']:3d}  planets={r['n_planets']}  {best_str}")

    # Generate report for top system
    if ranked and ranked[0]["n6_score"] > 0:
        top_host = ranked[0]["hostname"]
        print(f"\n[2/3] Generating detailed report for {top_host}...\n")
        report = generate_report(top_host)
        print(report)

    # Cross-source check for top 3
    print("\n[3/3] Cross-source checks...\n")
    for r in ranked[:3]:
        host = r["hostname"]
        xref = cross_source_check(host)
        flags = []
        if xref["tess"]:
            flags.append("TESS")
        if xref["kepler"]:
            flags.append("Kepler")
        if xref["breakthrough_listen"]:
            flags.append("BL")
        if xref["radial_velocity"]:
            flags.append("RV")
        print(f"  {host:20s}  sources: {', '.join(flags) if flags else 'none found'}  "
              f"facilities: {', '.join(xref['facilities'])}")

    print("\n" + "=" * 70)
    print("  Tracker complete. Data stored in data/n6_tracker/")
    print("=" * 70)


if __name__ == "__main__":
    main()
