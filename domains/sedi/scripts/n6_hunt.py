#!/usr/bin/env python3
"""n=6 orbital pattern hunter — comprehensive scan and discovery."""
import sys
import os
import json
from datetime import datetime, timezone

sys.path.insert(0, os.path.dirname(os.path.dirname(__file__)))

from sedi.n6_tracker import (
    scan_all_targets, search_new_candidates, compute_n6_matrix,
    N6_TARGETS, N6_RATIOS, MATCH_TOLERANCE, track_precision_history,
    DATA_DIR,
)
from sedi.sources.exoplanet import (
    fetch_six_planet_systems, group_by_system, query_tap, PLANET_COLS,
)


def main():
    print("=" * 72)
    print("  SEDI n=6 EXOPLANET ORBITAL PATTERN HUNTER")
    print(f"  {datetime.now(timezone.utc).strftime('%Y-%m-%d %H:%M UTC')}")
    print("=" * 72)

    # ------------------------------------------------------------------
    # STEP 1: Refresh all 11 priority systems
    # ------------------------------------------------------------------
    print("\n" + "=" * 72)
    print("  STEP 1: Scanning all 11 priority systems")
    print("=" * 72 + "\n")

    ranked = scan_all_targets()

    print("\n  RANKING TABLE:")
    print("  " + "-" * 68)
    print(f"  {'#':>3s}  {'System':20s}  {'Score':>5s}  {'Planets':>7s}  {'Pri':>3s}  {'Best Match'}")
    print("  " + "-" * 68)
    for i, r in enumerate(ranked, 1):
        best = r.get("best_match")
        best_str = (f"{best['pair']} {best['pattern']} ({best['deviation_pct']:.4f}%)"
                    if best else "no matches")
        print(f"  {i:3d}  {r['hostname']:20s}  {r['n6_score']:5d}  {r['n_planets']:7d}  "
              f"{r['priority']:3d}  {best_str}")

    # ------------------------------------------------------------------
    # STEP 2: Search for NEW candidate systems
    # ------------------------------------------------------------------
    print("\n" + "=" * 72)
    print("  STEP 2: Searching for NEW candidate systems (min_planets=3)")
    print("=" * 72 + "\n")

    new_candidates = search_new_candidates(min_planets=3, top_n=50)

    if new_candidates:
        print("\n  NEW CANDIDATES WITH 3+ n=6 MATCHES:")
        print("  " + "-" * 68)
        three_plus = [c for c in new_candidates if c["n6_score"] >= 3]
        for c in three_plus:
            b = c["best_match"]
            print(f"    {c['hostname']:25s}  score={c['n6_score']:3d}  "
                  f"planets={c['n_planets']}  "
                  f"best: {b['pair']} {b['pattern']} ({b['deviation_pct']:.4f}%)")
        if not three_plus:
            print("    (none with 3+ matches)")

    # ------------------------------------------------------------------
    # STEP 3: Focus on 6-planet systems
    # ------------------------------------------------------------------
    print("\n" + "=" * 72)
    print("  STEP 3: Querying 6-planet systems from archive")
    print("=" * 72 + "\n")

    six_planet_rows = fetch_six_planet_systems()
    new_six_planet_results = []

    if six_planet_rows:
        six_systems = group_by_system(six_planet_rows)
        known = set(N6_TARGETS.keys())
        print(f"  Found {len(six_systems)} systems with 6 planets total")
        print(f"  Already tracked: {known & set(six_systems.keys())}")

        for host, plist in six_systems.items():
            if host in known:
                continue
            periods = sorted([p["pl_orbper"] for p in plist if p.get("pl_orbper")])
            if len(periods) < 2:
                continue
            names = [p["pl_name"].split()[-1]
                     for p in sorted(plist, key=lambda x: x.get("pl_orbper", 0))]
            matrix = compute_n6_matrix(periods, names)

            new_six_planet_results.append({
                "hostname": host,
                "n6_score": matrix["n6_score"],
                "n_planets": len(periods),
                "best_match": matrix["best_match"],
                "matches": matrix["matches"],
                "periods": periods,
                "names": names,
            })

            b = matrix["best_match"]
            print(f"    {host:25s}  score={matrix['n6_score']:3d}  "
                  f"best: {b['pair']} {b['pattern']} ({b['deviation_pct']:.4f}%)"
                  if b else f"    {host:25s}  score=0  no matches")

        new_six_planet_results.sort(key=lambda x: x["n6_score"], reverse=True)
    else:
        print("  No 6-planet systems returned from archive")

    # ------------------------------------------------------------------
    # STEP 4: Check for complete arithmetic ladders
    # ------------------------------------------------------------------
    print("\n" + "=" * 72)
    print("  STEP 4: Checking for complete arithmetic ladders")
    print("  (systems containing phi=2 AND sigma/tau=3 AND tau=4 or n=6)")
    print("=" * 72 + "\n")

    # Combine all systems with their match data for ladder check
    all_systems_for_ladder = []

    # Add priority systems
    for r in ranked:
        if r.get("n_matches_by_pattern"):
            all_systems_for_ladder.append({
                "hostname": r["hostname"],
                "n6_score": r["n6_score"],
                "pattern_counts": r["n_matches_by_pattern"],
                "source": "priority",
            })

    # Add new 6-planet systems
    for r in new_six_planet_results:
        patterns = {}
        for m in r.get("matches", []):
            p = m["pattern"]
            patterns[p] = patterns.get(p, 0) + 1
        all_systems_for_ladder.append({
            "hostname": r["hostname"],
            "n6_score": r["n6_score"],
            "pattern_counts": patterns,
            "source": "6-planet-new",
        })

    # Add new candidates
    for c in new_candidates:
        if c.get("n_matches_by_pattern"):
            all_systems_for_ladder.append({
                "hostname": c["hostname"],
                "n6_score": c["n6_score"],
                "pattern_counts": c["n_matches_by_pattern"],
                "source": "new-candidate",
            })

    ladder_systems = []
    for sys_info in all_systems_for_ladder:
        pc = sys_info["pattern_counts"]
        # Check for complete ladder: phi=2, sigma/tau=3, and tau=4 or n=6
        has_phi = any("phi" in k.lower() and "2" in k for k in pc) or pc.get("phi (2)", 0) > 0 or pc.get("sigma/n (2)", 0) > 0
        has_3 = pc.get("sigma/tau (3)", 0) > 0 or pc.get("n/phi (3)", 0) > 0
        has_4_or_6 = pc.get("tau (4)", 0) > 0 or pc.get("n (6)", 0) > 0

        if has_phi and has_3 and has_4_or_6:
            ladder_systems.append(sys_info)
            print(f"  *** COMPLETE LADDER: {sys_info['hostname']} "
                  f"(score={sys_info['n6_score']}, source={sys_info['source']})")
            print(f"      patterns: {sys_info['pattern_counts']}")

    if not ladder_systems:
        print("  No complete arithmetic ladders found (phi=2 + sigma/tau=3 + tau/n=4/6)")
        print("  (Partial ladders may exist — checking...)")
        for sys_info in all_systems_for_ladder:
            pc = sys_info["pattern_counts"]
            has_phi = any("phi" in k.lower() for k in pc)
            has_3 = any("3" in k for k in pc)
            has_4_or_6 = any("4" in k or "6" in k for k in pc)
            count = sum([has_phi, has_3, has_4_or_6])
            if count >= 2:
                print(f"    Partial (2/3): {sys_info['hostname']} "
                      f"(score={sys_info['n6_score']})  patterns: {dict(pc)}")

    # ------------------------------------------------------------------
    # STEP 5: Add new high-scoring candidates to N6_TARGETS
    # ------------------------------------------------------------------
    print("\n" + "=" * 72)
    print("  STEP 5: Identifying new systems to add to N6_TARGETS")
    print("=" * 72 + "\n")

    min_score = min(r["n6_score"] for r in ranked) if ranked else 0
    print(f"  Current minimum score in N6_TARGETS: {min_score}")

    new_to_add = []

    # From new candidates
    for c in new_candidates:
        if c["n6_score"] >= max(min_score, 3) and c["hostname"] not in N6_TARGETS:
            new_to_add.append(c)

    # From new 6-planet systems
    for r in new_six_planet_results:
        if r["n6_score"] >= max(min_score, 3) and r["hostname"] not in N6_TARGETS:
            already = any(x["hostname"] == r["hostname"] for x in new_to_add)
            if not already:
                new_to_add.append(r)

    if new_to_add:
        print(f"  Found {len(new_to_add)} new systems to add:")
        for c in new_to_add[:15]:
            b = c.get("best_match")
            print(f"    {c['hostname']:25s}  score={c['n6_score']:3d}  planets={c['n_planets']}")
    else:
        print("  No new systems meet the threshold for adding")

    # ------------------------------------------------------------------
    # STEP 6: Track precision history for all systems
    # ------------------------------------------------------------------
    print("\n" + "=" * 72)
    print("  STEP 6: Tracking precision history")
    print("=" * 72 + "\n")

    all_hostnames = list(N6_TARGETS.keys())
    # Also add top new candidates
    for c in new_to_add[:10]:
        if c["hostname"] not in all_hostnames:
            all_hostnames.append(c["hostname"])

    for host in all_hostnames:
        try:
            result = track_precision_history(host)
            if "error" in result:
                print(f"  {host:25s}  ERROR: {result['error']}")
            else:
                cur = result["current"]
                print(f"  {host:25s}  score={cur['n6_score']:3d}  "
                      f"snapshots={result['snapshots']}")
        except Exception as e:
            print(f"  {host:25s}  EXCEPTION: {e}")

    # ------------------------------------------------------------------
    # STEP 7: Final report
    # ------------------------------------------------------------------
    print("\n" + "=" * 72)
    print("  STEP 7: FINAL COMPREHENSIVE REPORT")
    print("=" * 72)

    total_scanned = len(ranked) + len(new_candidates)
    for r in new_six_planet_results:
        if r["hostname"] not in [c["hostname"] for c in new_candidates]:
            total_scanned += 1

    print(f"\n  Total systems scanned:         {total_scanned}")
    print(f"  Priority systems:              {len(ranked)}")
    print(f"  New candidates found:          {len(new_candidates)}")
    print(f"  New 6-planet systems found:    {len(new_six_planet_results)}")
    print(f"  Complete arithmetic ladders:   {len(ladder_systems)}")
    print(f"  Systems added to N6_TARGETS:   {len(new_to_add)}")

    # Check if any exceed HD 110067
    hd_score = next((r["n6_score"] for r in ranked if r["hostname"] == "HD 110067"), 0)
    print(f"\n  HD 110067 score:               {hd_score}")
    exceeding = [r for r in ranked if r["n6_score"] > hd_score and r["hostname"] != "HD 110067"]
    for c in new_candidates:
        if c["n6_score"] > hd_score:
            exceeding.append(c)
    if exceeding:
        print(f"  Systems EXCEEDING HD 110067:")
        for e in exceeding:
            print(f"    {e['hostname']:25s}  score={e['n6_score']}")
    else:
        print(f"  No system exceeds HD 110067's score")

    # Top 20 overall
    all_ranked = []
    for r in ranked:
        all_ranked.append({"hostname": r["hostname"], "n6_score": r["n6_score"],
                           "n_planets": r["n_planets"], "source": "priority",
                           "best_match": r.get("best_match")})
    for c in new_candidates:
        if c["hostname"] not in [a["hostname"] for a in all_ranked]:
            all_ranked.append({"hostname": c["hostname"], "n6_score": c["n6_score"],
                               "n_planets": c["n_planets"], "source": "new",
                               "best_match": c.get("best_match")})
    for r in new_six_planet_results:
        if r["hostname"] not in [a["hostname"] for a in all_ranked]:
            all_ranked.append({"hostname": r["hostname"], "n6_score": r["n6_score"],
                               "n_planets": r["n_planets"], "source": "6-planet",
                               "best_match": r.get("best_match")})

    all_ranked.sort(key=lambda x: x["n6_score"], reverse=True)

    print(f"\n  TOP 20 SYSTEMS BY n=6 SCORE:")
    print("  " + "-" * 68)
    print(f"  {'#':>3s}  {'System':25s}  {'Score':>5s}  {'Planets':>7s}  {'Source':10s}  {'Best Match'}")
    print("  " + "-" * 68)
    for i, r in enumerate(all_ranked[:20], 1):
        b = r.get("best_match")
        best_str = (f"{b['pair']} {b['pattern']} ({b['deviation_pct']:.4f}%)"
                    if b else "—")
        print(f"  {i:3d}  {r['hostname']:25s}  {r['n6_score']:5d}  "
              f"{r['n_planets']:7d}  {r['source']:10s}  {best_str}")

    # Return new_to_add for step 5 file update
    return new_to_add, all_ranked


if __name__ == "__main__":
    new_to_add, all_ranked = main()
    print("\n" + "=" * 72)
    print("  HUNT COMPLETE")
    print("=" * 72)
