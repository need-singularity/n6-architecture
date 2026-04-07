#!/usr/bin/env python3
"""Auto-Hypothesis Generator — Convert discovery engine results to hypothesis .md files.

Reads JSON output from discovery-engine, formula-miner, or signal-analyzer,
filters to high-quality discoveries (Tier A/B or STRONG FALSIFY),
and generates hypothesis markdown files.

Usage:
  python3 auto_hypothesis.py --input data/discoveries/engine_*.json [--dry-run] [--min-tier B]
  python3 auto_hypothesis.py --from-engine  # run engine and pipe output
"""
import json, re, os, sys, glob
from pathlib import Path
from datetime import date

SEDI_ROOT = Path(__file__).parent.parent
HYPOTHESES_DIR = SEDI_ROOT / "docs" / "hypotheses"


def get_next_id(prefix="CX"):
    """Find the next available hypothesis number for given prefix."""
    existing = sorted(HYPOTHESES_DIR.glob(f"H-{prefix}-*.md"))
    if not existing:
        return 1
    # Extract numbers from filenames
    numbers = []
    for f in existing:
        match = re.search(r'H-\w+-(\d+)', f.stem)
        if match:
            numbers.append(int(match.group(1)))
    return max(numbers) + 1 if numbers else 1


def classify_domain(discovery):
    """Classify discovery into hypothesis prefix based on domains/targets."""
    desc = discovery.get("description", "").lower()
    domains = [d.lower() for d in discovery.get("domains", [])]

    # Check for consciousness-related
    if any(kw in desc for kw in ["consciousness", "phi", "awareness", "neural"]):
        return "CS"
    # Check for anima-related
    if any(kw in desc for kw in ["anima", "hexad", "embodiment"]):
        return "CA"
    # Default: physics/constants
    return "CX"


def assign_tier(discovery):
    """Assign tier (A-E) from score and precision since engine JSON has no tier field.

    Tier A: precision == 1.0 and score >= 0.12  (exact matches)
    Tier B: precision >= 0.99 or score >= 0.10
    Tier C: precision >= 0.95 or score >= 0.05
    Tier D: precision >= 0.90
    Tier E: everything else
    """
    precision = discovery.get("precision", 0)
    score = discovery.get("score", 0)

    if precision >= 1.0 and score >= 0.12:
        return "A"
    if precision >= 0.99 or score >= 0.10:
        return "B"
    if precision >= 0.95 or score >= 0.05:
        return "C"
    if precision >= 0.90:
        return "D"
    return "E"


def generate_hypothesis_md(discovery, hyp_id, prefix):
    """Generate hypothesis markdown content matching existing hypothesis format."""
    formula = discovery.get("formula", "unknown")
    score = discovery.get("score", 0)
    operator = discovery.get("operator", "unknown")
    domains = discovery.get("domains", [])
    description = discovery.get("description", "")
    precision = discovery.get("precision", 0)
    novelty = discovery.get("novelty", 0)
    diversity = discovery.get("diversity", 0)

    # Extract error from description
    # Typical: "Water hole upper (MHz) = 1720 <-- n6: σ³-n-φ (err=0.0000%)"
    # or: "Value 0.400000 matches n_e (habitable planets) via 10 n6 expressions: ..."
    error_match = re.search(r'err=(\d+\.?\d*)%', description)
    if not error_match:
        error_match = re.search(r'(\d+\.?\d*)%', description)
    error_pct = error_match.group(1) if error_match else "N/A"

    # Extract a short title from description
    # For INVERSE: "Water hole upper (MHz) = 1720 <-- n6: σ³-n-φ (err=0.0000%)"
    title_match = re.match(r'(.+?)\s*(?:<--|via\s+\d+)', description)
    short_title = title_match.group(1).strip() if title_match else description[:80]
    # Clean up "Value X matches Y" form
    val_match = re.match(r'Value\s+[\d.]+\s+matches\s+(.+)', short_title)
    if val_match:
        short_title = val_match.group(1).strip()

    # Determine grade
    try:
        err_val = float(error_pct)
        if err_val == 0:
            grade = "🟩 EXACT"
        elif err_val < 0.01:
            grade = "🟩 CONFIRMED"
        elif err_val < 0.5:
            grade = "🟧★ APPROXIMATE-PLUS"
        elif err_val < 2.0:
            grade = "🟧 APPROXIMATE"
        else:
            grade = "⬜ UNTESTED"
    except (ValueError, TypeError):
        grade = "⬜ UNTESTED"

    title = f"H-{prefix}-{hyp_id:03d}"

    # Pick the first (simplest) formula expression
    first_formula = formula.split(",")[0].strip()

    content = f"""# {title}: {short_title}

> **Hypothesis**: n=6 arithmetic expression `{first_formula}` reproduces {short_title}.

## Grade: {grade}

## Results

| Observable | n=6 Formula | Error |
|---|---|---|
| {short_title} | `{first_formula}` | {error_pct}% |

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, R(6) = 1
```

### All Matching Expressions

```
{formula}
```

### Discovery Metadata

- **Operator**: {operator}
- **Score**: {score:.4f}
- **Precision**: {precision:.4f}
- **Novelty**: {novelty:.4f}
- **Diversity**: {diversity:.4f}
- **Domains**: {', '.join(domains) if domains else 'N/A'}
- **Auto-generated**: {date.today().isoformat()}
- **Source**: SEDI Discovery Engine v3

## Status

- [ ] Independent verification
- [ ] Cross-reference with existing hypotheses
- [ ] Bayesian tier assignment
- [ ] Literature check
"""
    slug = re.sub(r'[^a-z0-9]+', '-', short_title[:50].lower()).strip('-')
    return title, slug, content


def main():
    import argparse
    parser = argparse.ArgumentParser(description="Auto-generate hypothesis files from discoveries")
    parser.add_argument("--input", nargs="+", help="JSON files from discovery engine (supports glob)")
    parser.add_argument("--from-engine", action="store_true", help="Run discovery engine and use output")
    parser.add_argument("--dry-run", action="store_true", help="Preview without creating files")
    parser.add_argument("--min-tier", default="B", choices=["A", "B", "C", "D", "E"],
                        help="Minimum tier to include (default: B)")
    parser.add_argument("--max-count", type=int, default=20, help="Max hypotheses to generate")
    args = parser.parse_args()

    # Load discoveries
    discoveries = []
    if args.from_engine:
        import subprocess
        engine = SEDI_ROOT / "tools" / "discovery-engine" / "target" / "release" / "sedi-discovery-engine"
        if not engine.exists():
            print(f"Engine binary not found at {engine}")
            print("Build with: cd tools/discovery-engine && cargo build --release")
            sys.exit(1)
        result = subprocess.run([str(engine), "--json"], capture_output=True, text=True)
        if result.returncode != 0:
            print(f"Engine failed: {result.stderr}")
            sys.exit(1)
        data = json.loads(result.stdout)
        discoveries = data.get("discoveries", [])
    elif args.input:
        for pattern in args.input:
            for f in glob.glob(pattern):
                data = json.loads(Path(f).read_text())
                discoveries.extend(data.get("discoveries", []))
    else:
        parser.print_help()
        sys.exit(1)

    if not discoveries:
        print("No discoveries found.")
        return

    # Assign tiers (engine JSON doesn't include them)
    for d in discoveries:
        d["_tier"] = assign_tier(d)

    # Filter by tier
    tier_order = {"A": 0, "B": 1, "C": 2, "D": 3, "E": 4}
    min_tier_idx = tier_order.get(args.min_tier, 1)

    filtered = [d for d in discoveries
                if tier_order.get(d["_tier"], 4) <= min_tier_idx]

    # Sort by score descending
    filtered.sort(key=lambda d: d.get("score", 0), reverse=True)
    filtered = filtered[:args.max_count]

    print(f"Generating {len(filtered)} hypotheses (min tier: {args.min_tier})\n")

    # Track IDs per prefix so we increment correctly within a single run
    next_ids = {}
    created = []

    for disc in filtered:
        prefix = classify_domain(disc)
        if prefix not in next_ids:
            next_ids[prefix] = get_next_id(prefix)
        hyp_num = next_ids[prefix]
        next_ids[prefix] += 1

        title, slug, content = generate_hypothesis_md(disc, hyp_num, prefix)

        filename = f"{title}-{slug[:40]}.md"
        filepath = HYPOTHESES_DIR / filename

        if args.dry_run:
            print(f"  [DRY RUN] {filename}")
            print(f"    Tier {disc['_tier']} | Score {disc.get('score', 0):.4f} | {disc.get('description', '')[:70]}")
        else:
            filepath.write_text(content)
            created.append(filename)
            print(f"  Created: {filename}")

    if not args.dry_run and created:
        print(f"\n{len(created)} hypotheses created in {HYPOTHESES_DIR}")
        print("Run `python3 scripts/auto_grade_n6.py` to assign Bayesian tiers.")
    elif args.dry_run:
        print(f"\n{len(filtered)} hypotheses would be created (dry run)")


if __name__ == "__main__":
    main()
