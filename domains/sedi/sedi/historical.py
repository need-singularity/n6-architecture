"""Historical data scanner — search past data for n=6 patterns.

Usage:
  sedi history --source earthquake --start 2000 --end 2025
  sedi history --source solar --start 2010
  sedi history --source ligo-catalog
  sedi history --source oeis --query "perfect"
  sedi history --source cern-masses
  sedi history --source bitcoin --start-block 0 --end-block 100
  sedi history --all
"""
import time
import numpy as np
from .filter import r_filter
from .detector import analyze, format_alert
from .constants import N, SIGMA


def scan_earthquake_history(start_year=2000, end_year=2025, minmag=5.0):
    """Scan 25 years of earthquake data."""
    from .sources.earthquake import scan_historical
    print(f"🌍 Scanning earthquakes {start_year}-{end_year} (M>={minmag})...")

    all_alerts = []
    for batch in scan_historical(start_year, end_year, minmag):
        data = np.array(batch['data'])
        result = r_filter(data)
        alerts = analyze(result, source_name=f"earthquake-{batch['year']}")
        if alerts:
            for a in alerts:
                print(f"   {format_alert(a)}")
                all_alerts.append(a)
        else:
            print(f"   ⚪ {batch['year']}: {batch['n_events']} events, no anomaly")
    return all_alerts


def scan_solar_history(start_year=2010, end_year=2025):
    """Scan solar flare data."""
    from .sources.nasa import scan_solar_historical
    print(f"☀️ Scanning solar flares {start_year}-{end_year}...")

    all_alerts = []
    for batch in scan_solar_historical(start_year, end_year):
        data = np.array(batch['data'])
        if len(data) < 6:
            continue
        result = r_filter(data)
        alerts = analyze(result, source_name=f"solar-{batch['year']}")
        if alerts:
            for a in alerts:
                print(f"   {format_alert(a)}")
                all_alerts.append(a)
        else:
            print(f"   ⚪ {batch['year']}: {batch['n']} flares, no anomaly")
    return all_alerts


def scan_ligo_catalog():
    """Scan all LIGO gravitational wave event masses."""
    from .sources.ligo import scan_catalog_masses
    print("🌊 Scanning LIGO event catalog for n=6 mass patterns...")

    hits = scan_catalog_masses()
    if hits:
        for h in hits:
            print(f"   🟡 {h['event']}: {h['param']}={h['value']:.1f} "
                  f"≈ {h['target']} (err {h['error_pct']:.1f}%)")
    else:
        print("   ⚪ No n=6 mass patterns found")
    return hits


def scan_cern_masses():
    """Check particle mass ratios for n=6 patterns."""
    from .sources.cern import check_mass_ratios
    print("⚛️ Scanning particle mass ratios for n=6 patterns...")

    hits = check_mass_ratios()
    for h in hits:
        p1, p2 = h['particles']
        print(f"   {'🟡' if h['error_pct'] < 1 else '⚪'} "
              f"{p1}/{p2} = {h['ratio']:.4f} ≈ {h['target']}={h['target_val']} "
              f"(err {h['error_pct']:.2f}%)")
    return hits


def scan_oeis(query='sigma(n)*phi(n)', max_results=20):
    """Search OEIS for sequences related to n=6."""
    from .sources.oeis import search_oeis, check_sequence_for_n6
    print(f"📊 Searching OEIS for '{query}'...")

    results = search_oeis(query, max_results)
    hits = []
    for seq in results:
        values = seq.get('data', '').split(',')
        try:
            int_values = [int(v.strip()) for v in values if v.strip()]
        except ValueError:
            continue
        n6_hits = check_sequence_for_n6(int_values)
        if n6_hits:
            seq_id = f"A{seq.get('number', 0):06d}"
            name = seq.get('name', '')[:60]
            print(f"   🟡 {seq_id}: {name}")
            for h in n6_hits:
                print(f"      pos {h['position']}: {h['value']} = {h['match']}")
            hits.append({'seq_id': seq_id, 'name': name, 'hits': n6_hits})
    if not hits:
        print("   ⚪ No n=6 patterns found")
    return hits


def scan_all():
    """Run ALL historical scans."""
    print("🛸 SEDI Historical Scan — ALL sources")
    print("=" * 60)
    print()

    results = {}

    results['cern'] = scan_cern_masses()
    print()

    results['ligo'] = scan_ligo_catalog()
    print()

    results['earthquake'] = scan_earthquake_history(start_year=2020, end_year=2025)
    print()

    results['solar'] = scan_solar_history(start_year=2020, end_year=2025)
    print()

    results['oeis'] = scan_oeis('perfect number')
    print()

    # Summary
    print("=" * 60)
    print("   HISTORICAL SCAN SUMMARY")
    print("=" * 60)
    for source, data in results.items():
        count = len(data) if isinstance(data, list) else 0
        print(f"   {source}: {count} hits")
    total = sum(len(d) for d in results.values() if isinstance(d, list))
    print(f"\n   Total: {total} potential n=6 patterns across all historical data")

    return results
