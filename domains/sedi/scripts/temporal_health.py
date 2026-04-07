#!/usr/bin/env python3
"""SEDI TEMPORAL Operator — Track n=6 hypothesis health over time.

Analyzes:
1. Match rate trend: are we finding more or fewer n=6 matches over time?
2. Per-domain health: which physics domains are CONVERGING vs DIVERGING?
3. New data impact: when new measurements arrive, do they confirm or refute n=6?
4. Overall project health score
"""
import json, os, glob, re
from datetime import datetime
from pathlib import Path

SEDI_ROOT = Path(__file__).parent.parent
HYPOTHESES_DIR = SEDI_ROOT / "docs" / "hypotheses"
DATA_DIR = SEDI_ROOT / "data"
GRADES_FILE = DATA_DIR / "sedi-grades.json"

def load_grades():
    """Load current hypothesis grades."""
    if GRADES_FILE.exists():
        return json.loads(GRADES_FILE.read_text())
    return {}

def load_hypothesis_timestamps():
    """Get creation/modification times of hypothesis files as proxy for discovery timeline."""
    files = sorted(HYPOTHESES_DIR.glob("H-*.md"))
    timeline = []
    for f in files:
        stat = f.stat()
        timeline.append({
            "id": f.stem,
            "created": datetime.fromtimestamp(stat.st_ctime).isoformat(),
            "modified": datetime.fromtimestamp(stat.st_mtime).isoformat(),
            "size": stat.st_size,
        })
    return timeline

def compute_health_indicators(grades, timeline):
    """Compute health indicators."""
    # 1. Tier distribution
    tier_dist = grades.get("tier_distribution", {})
    total = sum(tier_dist.values()) if tier_dist else 0

    # 2. Positive evidence ratio (Tier A+B / total)
    positive = tier_dist.get("A", 0) + tier_dist.get("B", 0)
    positive_ratio = positive / total if total > 0 else 0

    # 3. Mean bits
    mean_bits = grades.get("mean_bits", 0)

    # 4. Domain health (from hypothesis categories)
    # Parse hypothesis IDs: H-CX=physics, H-CS=consciousness, H-CA=anima
    categories = {"CX": 0, "CS": 0, "CA": 0, "other": 0}
    for h in timeline:
        cat = h["id"].split("-")[1] if "-" in h["id"] else "other"
        categories[cat] = categories.get(cat, 0) + 1

    # 5. Health score (0-100)
    health = 0
    health += min(30, positive_ratio * 30)  # Up to 30 points for positive ratio
    health += min(30, mean_bits * 30 / 20)  # Up to 30 points for mean bits
    health += min(20, total * 20 / 700)     # Up to 20 points for coverage
    health += min(20, len(categories) * 5)  # Up to 20 points for domain diversity

    return {
        "total_hypotheses": total,
        "tier_distribution": tier_dist,
        "positive_ratio": round(positive_ratio, 3),
        "mean_bits": round(mean_bits, 1),
        "health_score": round(min(100, health), 1),
        "status": "CONVERGING" if positive_ratio > 0.8 else "STABLE" if positive_ratio > 0.5 else "DIVERGING",
        "categories": categories,
        "timestamp": datetime.now().isoformat(),
    }

def generate_report(health):
    """Generate markdown health report."""
    status_emoji = {"CONVERGING": "🟢", "STABLE": "🟡", "DIVERGING": "🔴"}

    report = f"""# SEDI Temporal Health Report
**Date**: {health['timestamp'][:10]}
**Status**: {status_emoji.get(health['status'], '⚪')} {health['status']}
**Health Score**: {health['health_score']}/100

## Tier Distribution
| Tier | Count | Evidence (bits) |
|------|-------|-----------------|
| A | {health['tier_distribution'].get('A', 0)} | >20 |
| B | {health['tier_distribution'].get('B', 0)} | 10-20 |
| C | {health['tier_distribution'].get('C', 0)} | 3-10 |
| D | {health['tier_distribution'].get('D', 0)} | 0-3 |
| E | {health['tier_distribution'].get('E', 0)} | <0 |

## Key Metrics
- **Positive ratio** (A+B/total): {health['positive_ratio']}
- **Mean evidence**: {health['mean_bits']} bits
- **Total hypotheses**: {health['total_hypotheses']}

## Recommendations
"""
    if health['status'] == 'CONVERGING':
        report += "- Project is healthy. Focus on PREDICT operator to extend patterns.\n"
        report += "- Priority: increase Tier A count through precision improvements.\n"
    elif health['status'] == 'STABLE':
        report += "- Consider adding new data sources to break plateau.\n"
        report += "- Run ANOMALY operator to find missing matches.\n"
    else:
        report += "- ⚠️ Review recent hypotheses for quality issues.\n"
        report += "- Run FALSIFY on Tier C/D hypotheses.\n"

    return report

def main():
    import argparse
    parser = argparse.ArgumentParser(description="SEDI Temporal Health Monitor")
    parser.add_argument("--json", action="store_true", help="JSON output")
    parser.add_argument("--save", action="store_true", help="Save report to data/")
    args = parser.parse_args()

    grades = load_grades()
    timeline = load_hypothesis_timestamps()
    health = compute_health_indicators(grades, timeline)

    if args.json:
        print(json.dumps(health, indent=2))
    else:
        report = generate_report(health)
        print(report)

        if args.save:
            out = DATA_DIR / f"temporal_health_{datetime.now().strftime('%Y%m%d')}.md"
            out.write_text(report)
            print(f"\nSaved to {out}")

            # Also append to health history
            history_file = DATA_DIR / "temporal_history.json"
            history = json.loads(history_file.read_text()) if history_file.exists() else []
            history.append(health)
            history_file.write_text(json.dumps(history, indent=2))

if __name__ == "__main__":
    main()
