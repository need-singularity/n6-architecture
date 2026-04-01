#!/usr/bin/env python3
"""Cross-Repo Constant Mining: scan 305 TOML domains for n=6 patterns."""

import os, re, sys
from collections import Counter, defaultdict

DOMAIN_DIR = os.path.expanduser("~/Dev/n6-architecture/tools/universal-dse/domains")

# n=6 constants
N6 = {
    1: "mu", 2: "phi", 3: "n/phi", 4: "tau", 5: "sopfr", 6: "n",
    8: "sigma-tau", 10: "sigma-phi", 11: "sigma-mu", 12: "sigma",
    13: "sigma+mu", 14: "sigma+phi", 20: "J2-tau", 24: "J2",
    48: "sigma*tau", 72: "sigma*n", 144: "sigma^2",
    120: "sigma*(sigma-phi)", 288: "sigma*J2",
    7: "sigma-sopfr", 9: "sigma-n/phi", 16: "2^tau",
    32: "2^sopfr", 64: "2^n", 128: "2^(sigma-sopfr)", 256: "2^(sigma-tau)",
    0.1: "1/(sigma-phi)", 0.288: "ln(4/3)", 0.95: "1-1/(J2-tau)",
}

def parse_toml_light(path):
    """Lightweight TOML parser for DSE domain files."""
    levels, candidates, rules, notes_nums = [], [], [], []
    current_section = None
    current_item = {}

    with open(path) as f:
        for line in f:
            line = line.strip()
            if line.startswith("[[level]]"):
                if current_section == "candidate" and current_item:
                    candidates.append(dict(current_item))
                elif current_section == "rule" and current_item:
                    rules.append(dict(current_item))
                current_section = "level"
                current_item = {}
                continue
            if line.startswith("[[candidate]]"):
                if current_section == "candidate" and current_item:
                    candidates.append(dict(current_item))
                elif current_section == "level" and current_item:
                    levels.append(dict(current_item))
                elif current_section == "rule" and current_item:
                    rules.append(dict(current_item))
                current_section = "candidate"
                current_item = {}
                continue
            if line.startswith("[[rule]]"):
                if current_section == "candidate" and current_item:
                    candidates.append(dict(current_item))
                elif current_section == "level" and current_item:
                    levels.append(dict(current_item))
                current_section = "rule"
                current_item = {}
                continue
            if line.startswith("[") and not line.startswith("[["):
                if current_section == "candidate" and current_item:
                    candidates.append(dict(current_item))
                elif current_section == "level" and current_item:
                    levels.append(dict(current_item))
                elif current_section == "rule" and current_item:
                    rules.append(dict(current_item))
                current_section = line.strip("[]")
                current_item = {}
                continue
            if "=" in line and current_section:
                key, _, val = line.partition("=")
                key = key.strip()
                val = val.strip().strip('"')
                current_item[key] = val
                # Extract numbers from notes
                if key == "notes":
                    nums = re.findall(r'(?<!=)\b(\d+(?:\.\d+)?)\b', val)
                    for n in nums:
                        try:
                            v = float(n) if '.' in n else int(n)
                            notes_nums.append(v)
                        except: pass

    # flush last item
    if current_section == "candidate" and current_item:
        candidates.append(dict(current_item))
    elif current_section == "level" and current_item:
        levels.append(dict(current_item))
    elif current_section == "rule" and current_item:
        rules.append(dict(current_item))

    return levels, candidates, rules, notes_nums


def main():
    files = sorted(f for f in os.listdir(DOMAIN_DIR) if f.endswith(".toml"))
    print(f"=== Cross-Repo Constant Mining: {len(files)} TOML domains ===\n")

    level_counts = Counter()       # how many levels per domain
    cand_per_level = Counter()     # candidates per level across all domains
    all_notes_nums = Counter()     # all numbers found in notes
    domain_stats = []
    level_name_counts = Counter()
    n6_score_dist = Counter()      # distribution of n6 scores
    total_candidates = 0
    total_rules = 0
    scoring_weights = defaultdict(list)

    for fname in files:
        path = os.path.join(DOMAIN_DIR, fname)
        levels, candidates, rules, notes_nums = parse_toml_light(path)

        n_levels = len(levels)
        level_counts[n_levels] += 1

        # Count candidates per level
        lvl_cands = Counter()
        for c in candidates:
            lvl = c.get("level", "?")
            lvl_cands[lvl] += 1
            # n6 score
            try:
                n6v = float(c.get("n6", 0))
                n6_score_dist[round(n6v, 2)] += 1
            except: pass

        for lv in lvl_cands.values():
            cand_per_level[lv] += 1

        for l in levels:
            level_name_counts[l.get("name", "?")] += 1

        for n in notes_nums:
            all_notes_nums[n] += 1

        total_candidates += len(candidates)
        total_rules += len(rules)
        domain_stats.append((fname.replace(".toml",""), n_levels, len(candidates), len(rules)))

    # === OUTPUT ===
    print("=" * 70)
    print("1. LEVEL COUNT DISTRIBUTION (how many levels per domain)")
    print("=" * 70)
    for k in sorted(level_counts):
        pct = level_counts[k] / len(files) * 100
        bar = "#" * int(pct / 2)
        print(f"  {k} levels: {level_counts[k]:4d} domains ({pct:5.1f}%) {bar}")
    print(f"\n  Total: {len(files)} domains, {total_candidates} candidates, {total_rules} rules")

    print(f"\n{'=' * 70}")
    print("2. CANDIDATES PER LEVEL DISTRIBUTION")
    print("=" * 70)
    for k in sorted(cand_per_level):
        pct = cand_per_level[k] / sum(cand_per_level.values()) * 100
        bar = "#" * int(pct / 2)
        print(f"  {k} candidates/level: {cand_per_level[k]:4d} occurrences ({pct:5.1f}%) {bar}")

    print(f"\n{'=' * 70}")
    print("3. TOP 15 LEVEL NAMES")
    print("=" * 70)
    for name, cnt in level_name_counts.most_common(15):
        print(f"  {name:30s} {cnt:4d}")

    print(f"\n{'=' * 70}")
    print("4. NUMBERS IN NOTES FIELDS (top 30, cross-referenced with n=6)")
    print("=" * 70)
    for num, cnt in all_notes_nums.most_common(30):
        n6_match = N6.get(num, "")
        marker = f" *** n=6: {n6_match}" if n6_match else ""
        print(f"  {num:>8} appears {cnt:4d} times{marker}")

    print(f"\n{'=' * 70}")
    print("5. n=6 CONSTANT FREQUENCY IN NOTES (only n=6 matches)")
    print("=" * 70)
    n6_hits = []
    for num, cnt in all_notes_nums.most_common():
        if num in N6:
            n6_hits.append((num, cnt, N6[num]))
    n6_hits.sort(key=lambda x: -x[1])
    for num, cnt, name in n6_hits:
        print(f"  {name:20s} = {num:>8} : {cnt:4d} appearances")

    # n6 score distribution
    print(f"\n{'=' * 70}")
    print("6. n6 SCORE DISTRIBUTION (candidate n6 field values)")
    print("=" * 70)
    for score in sorted(n6_score_dist, reverse=True):
        cnt = n6_score_dist[score]
        bar = "#" * min(int(cnt / 10), 60)
        print(f"  n6={score:.2f}: {cnt:5d} candidates {bar}")

    # Pattern analysis
    print(f"\n{'=' * 70}")
    print("7. NEW PATTERN DISCOVERIES")
    print("=" * 70)

    # Most common level count
    mc_levels = level_counts.most_common(1)[0]
    print(f"\n  [A] Dominant level count: {mc_levels[0]} ({mc_levels[1]}/{len(files)} = {mc_levels[1]/len(files)*100:.1f}%)")
    if mc_levels[0] == 5:
        print(f"      >>> MATCH: 5 = sopfr(6) — most domains have sopfr levels")
    elif mc_levels[0] == 6:
        print(f"      >>> MATCH: 6 = n — most domains have n levels")

    # Most common candidates per level
    mc_cands = cand_per_level.most_common(1)[0]
    print(f"\n  [B] Dominant candidates/level: {mc_cands[0]} ({mc_cands[1]} occurrences)")
    if mc_cands[0] == 6:
        print(f"      >>> MATCH: 6 = n — candidates per level = n")

    # Numbers appearing in 3+ domains that match n=6
    print(f"\n  [C] n=6 constants appearing in notes (3+ times):")
    for num, cnt, name in n6_hits:
        if cnt >= 3:
            print(f"      {name:20s} = {num:>8} : {cnt:4d} times")

    # Non-n6 numbers appearing frequently (potential new constants)
    print(f"\n  [D] Frequent non-n6 numbers (potential NEW constants, 10+ appearances):")
    for num, cnt in all_notes_nums.most_common(50):
        if num not in N6 and cnt >= 10:
            # Try to express as n=6 expression
            exprs = []
            if isinstance(num, int):
                for a, an in N6.items():
                    if isinstance(a, int) and a > 0:
                        for b, bn in N6.items():
                            if isinstance(b, int) and b > 0:
                                if a + b == num: exprs.append(f"{an}+{bn}")
                                if a * b == num: exprs.append(f"{an}*{bn}")
                                if a - b == num and a > b: exprs.append(f"{an}-{bn}")
                                if b > 0 and a / b == num: exprs.append(f"{an}/{bn}")
            expr_str = f" (possible: {', '.join(exprs[:3])})" if exprs else ""
            print(f"      {num:>8} : {cnt:4d} times{expr_str}")

    # Top 10 BT candidates
    print(f"\n{'=' * 70}")
    print("8. TOP 10 POTENTIAL NEW BT CANDIDATES")
    print("=" * 70)

    bt_candidates = []

    # Pattern: level count universality
    if mc_levels[0] in (5, 6):
        bt_candidates.append((
            mc_levels[1]/len(files)*100,
            f"DSE Level Count Universality: {mc_levels[1]}/{len(files)} domains ({mc_levels[1]/len(files)*100:.0f}%) "
            f"use {mc_levels[0]} levels = {'sopfr' if mc_levels[0]==5 else 'n'}"
        ))

    # Pattern: candidate count universality
    if mc_cands[0] == 6:
        bt_candidates.append((
            mc_cands[1]/sum(cand_per_level.values())*100,
            f"DSE Candidate Count = n=6: {mc_cands[1]}/{sum(cand_per_level.values())} level-slots "
            f"({mc_cands[1]/sum(cand_per_level.values())*100:.0f}%) have exactly 6 candidates"
        ))

    # Pattern: n6 score distribution
    perfect = n6_score_dist.get(1.0, 0)
    total_c = sum(n6_score_dist.values())
    if perfect > 0:
        bt_candidates.append((
            perfect/total_c*100,
            f"n6=1.00 Universality: {perfect}/{total_c} candidates ({perfect/total_c*100:.1f}%) achieve perfect n=6 score"
        ))

    # Pattern: each n6 constant frequency
    for num, cnt, name in n6_hits[:10]:
        if cnt >= 20:
            bt_candidates.append((
                cnt,
                f"Notes constant {name}={num}: appears {cnt} times across domains (cross-domain resonance)"
            ))

    bt_candidates.sort(key=lambda x: -x[0])
    for i, (score, desc) in enumerate(bt_candidates[:10], 1):
        print(f"\n  BT-NEW-{i}: {desc}")

    print(f"\n{'=' * 70}")
    print("SUMMARY")
    print("=" * 70)
    print(f"  Domains scanned:    {len(files)}")
    print(f"  Total candidates:   {total_candidates}")
    print(f"  Total rules:        {total_rules}")
    print(f"  n=6 hits in notes:  {sum(c for _,c,_ in n6_hits)}")
    print(f"  Unique numbers:     {len(all_notes_nums)}")


if __name__ == "__main__":
    main()
