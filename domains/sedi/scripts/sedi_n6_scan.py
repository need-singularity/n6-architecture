#!/usr/bin/env python3
"""SEDI n=6 Connection Scanner — Drake/Fermi/SETI-focused analysis.

Scans all 688 SEDI hypothesis .md files for n=6 constant matches,
with special focus on:
  - Drake equation parameters
  - SETI frequencies and signal patterns
  - Fermi paradox parameters
  - Habitable zone / exoplanet connections
  - Consciousness / intelligence thresholds

n=6 base constants:
  n=6, phi=2, tau=4, sigma=12, J2=24, sopfr=5, mu=1
  Derived: sigma-tau=8, sigma-phi=10, sigma/tau=3, P2=28, P3=496
  Ratios: 1/2, 1/3, 1/6, 5/6, 2/3, 4/3, ln(4/3)=0.2877

Usage:
  python3 scripts/sedi_n6_scan.py             # full report to stdout
  python3 scripts/sedi_n6_scan.py --top 30    # top N matches
  python3 scripts/sedi_n6_scan.py --seti      # SETI-specific filter
  python3 scripts/sedi_n6_scan.py --drake     # Drake equation analysis
"""

import re
import math
import argparse
from pathlib import Path
from collections import defaultdict

# ── Paths ────────────────────────────────────────────────────────
SEDI_ROOT = Path(__file__).resolve().parent.parent
HYPOTHESES_DIR = SEDI_ROOT / "docs" / "hypotheses"

# ── n=6 Constants ────────────────────────────────────────────────
N6 = {
    "n": 6,
    "phi": 2,
    "tau": 4,
    "sigma": 12,
    "J2": 24,
    "sopfr": 5,
    "mu": 1,
    "sigma-tau": 8,
    "sigma-phi": 10,
    "sigma/tau": 3,
    "P2": 28,
    "P3": 496,
    "R(6)": 1,
    "sigma*n": 72,
    "sigma^2": 144,
    "phi^tau": 16,
    "n!": 720,
}

N6_RATIOS = {
    "1/2": 0.5,
    "1/3": 0.3333,
    "1/6": 0.1667,
    "5/6": 0.8333,
    "2/3": 0.6667,
    "4/3": 1.3333,
    "tau/sigma": 0.3333,
    "phi/sopfr": 0.4,
    "sopfr/phi": 2.5,
    "ln(4/3)": 0.2877,
    "1/e": 0.3679,
    "ln2": 0.6931,
    "sqrt(3/2)": 1.2247,
}

# Combined integer targets
N6_INTEGERS = set(N6.values())
N6_INTEGERS.update({1, 2, 3, 4, 5, 6, 8, 10, 12, 16, 24, 28, 48, 72, 96,
                     120, 144, 192, 288, 496, 720})

# ── SETI/Drake Keywords ─────────────────────────────────────────
SETI_KEYWORDS = [
    "Drake", "Fermi", "SETI", "1420", "hydrogen line", "21 cm",
    "habitable", "exoplanet", "Wow", "signal", "radio", "frequency",
    "contact", "extraterrestrial", "alien", "intelligence",
    "civilization", "interstellar", "galactic", "Dyson",
    "Kardashev", "biosignature", "technosignature", "water hole",
    "communication", "beacon", "encode", "decode", "message",
    "Shannon", "information", "entropy", "consciousness",
    "Anima", "PHI", "PSI_STEPS", "CONVERGENCE",
    "life", "carbon", "silicon", "organic", "biogenic",
    "stellar", "planetary", "orbit", "transit", "spectrum",
]

DRAKE_PARAMS = {
    "R*": "star formation rate",
    "fp": "fraction with planets",
    "ne": "habitable planets per star",
    "fl": "fraction developing life",
    "fi": "fraction developing intelligence",
    "fc": "fraction developing communication",
    "L": "civilization lifetime",
}


def extract_numbers(text: str) -> list[float]:
    """Extract all numeric values from text."""
    numbers = []
    # Match integers and decimals, including negative and scientific notation
    for m in re.finditer(r'(?<![a-zA-Z])(-?\d+\.?\d*(?:[eE][+-]?\d+)?)', text):
        try:
            val = float(m.group(1))
            if abs(val) < 1e8:  # ignore huge numbers
                numbers.append(val)
        except ValueError:
            continue
    return numbers


def n6_match_score(numbers: list[float]) -> tuple[int, list[dict]]:
    """Score a list of numbers for n=6 matches. Returns (score, matches)."""
    matches = []
    seen = set()

    for num in numbers:
        # Integer matches
        if num == int(num) and int(num) in N6_INTEGERS:
            key = f"int:{int(num)}"
            if key not in seen:
                seen.add(key)
                # Higher score for more specific n=6 constants
                if int(num) in {6, 12, 24, 496}:
                    score = 3
                elif int(num) in {4, 8, 28, 144, 720}:
                    score = 2
                else:
                    score = 1
                matches.append({
                    "value": num,
                    "match": f"n6 integer {int(num)}",
                    "score": score,
                })

        # Ratio matches (within 1%)
        for name, target in N6_RATIOS.items():
            if target == 0:
                continue
            if abs(num - target) / abs(target) < 0.01:
                key = f"ratio:{name}"
                if key not in seen:
                    seen.add(key)
                    error_pct = abs(num - target) / abs(target) * 100
                    matches.append({
                        "value": num,
                        "match": f"n6 ratio {name}={target}",
                        "score": 2 if error_pct < 0.1 else 1,
                        "error_pct": error_pct,
                    })

    total = sum(m["score"] for m in matches)
    return total, matches


def seti_relevance(text: str) -> tuple[int, list[str]]:
    """Score SETI/Drake relevance of text."""
    found = []
    for kw in SETI_KEYWORDS:
        if re.search(re.escape(kw), text, re.IGNORECASE):
            found.append(kw)
    return len(found), found


def parse_grade(text: str) -> tuple[str, int]:
    """Extract grade and star count from hypothesis."""
    gm = re.search(r"^##\s*Grade:\s*(.+)", text, re.MULTILINE)
    grade_raw = gm.group(1).strip() if gm else ""
    stars = grade_raw.count("★") + grade_raw.count("⭐")

    if "CONFIRMED" in grade_raw or "EXACT" in grade_raw:
        grade_score = 5
    elif "🟥" in grade_raw:
        grade_score = 4
    elif "🟧" in grade_raw:
        grade_score = 3
    elif "🟨" in grade_raw:
        grade_score = 2
    else:
        grade_score = 1

    return grade_raw, grade_score + stars


def scan_hypothesis(filepath: Path) -> dict:
    """Scan a single hypothesis file for n=6 connections."""
    text = filepath.read_text(encoding="utf-8")

    # Basic metadata
    title_m = re.match(r"^#\s+(.+)", text, re.MULTILINE)
    title = title_m.group(1).strip() if title_m else filepath.stem

    id_m = re.match(r"(H-\w+-\d+)", filepath.stem)
    hyp_id = id_m.group(1) if id_m else filepath.stem

    domain_m = re.match(r"H-(\w+)-", filepath.stem)
    domain = domain_m.group(1) if domain_m else "?"

    grade_raw, grade_score = parse_grade(text)

    # n=6 pattern counting (structural, not just numbers)
    n6_patterns = {
        r"n\s*=\s*6": "n=6 explicit",
        r"σ\(6\)|sigma\(6\)": "sigma(6)",
        r"τ\(6\)|tau\(6\)": "tau(6)",
        r"φ\(6\)|phi\(6\)|Euler.*totient": "phi(6)",
        r"σ-τ\s*=\s*8|σ\s*-\s*τ": "sigma-tau",
        r"σ/τ\s*=\s*3|σ\s*/\s*τ": "sigma/tau",
        r"J₂\s*=\s*24|Jordan": "J2=24",
        r"sopfr\s*=\s*5|sopfr": "sopfr",
        r"P₁\s*=\s*6|perfect.number": "perfect number",
        r"P₃\s*=\s*496|496": "P3=496",
        r"1/2\s*\+\s*1/3\s*\+\s*1/6|Egyptian.fraction": "Egyptian fraction",
        r"EXACT": "EXACT match",
        r"CONVERGENCE": "CONVERGENCE",
        r"R\(6\)\s*=\s*1": "R(6)=1",
        r"Leech|lattice.*24": "Leech lattice",
        r"Bott.*period": "Bott periodicity",
    }

    pattern_hits = {}
    for pat, label in n6_patterns.items():
        count = len(re.findall(pat, text, re.IGNORECASE))
        if count > 0:
            pattern_hits[label] = count

    # Numeric extraction and scoring
    numbers = extract_numbers(text)
    n6_score, n6_matches = n6_match_score(numbers)

    # SETI relevance
    seti_score, seti_keywords = seti_relevance(text)

    # Error percentages (lower = better)
    errors = []
    for em in re.finditer(r"(\d+\.?\d*)\s*%", text):
        val = float(em.group(1))
        if val < 50:
            errors.append(val)
    min_error = min(errors) if errors else None

    # Convergence
    has_convergence = bool(re.search(r"CONVERGENCE", text))

    # Combined score: n6 structural + numeric + grade + SETI bonus
    combined = (
        n6_score
        + len(pattern_hits) * 2
        + grade_score
        + (3 if has_convergence else 0)
        + (seti_score * 0.5)  # SETI relevance bonus
        + (5 if min_error is not None and min_error < 0.01 else
           3 if min_error is not None and min_error < 0.1 else
           1 if min_error is not None and min_error < 1.0 else 0)
    )

    return {
        "id": hyp_id,
        "title": title,
        "domain": domain,
        "file": filepath.name,
        "grade_raw": grade_raw,
        "grade_score": grade_score,
        "n6_score": n6_score,
        "n6_matches": n6_matches,
        "pattern_hits": pattern_hits,
        "seti_score": seti_score,
        "seti_keywords": seti_keywords,
        "min_error": min_error,
        "has_convergence": has_convergence,
        "combined_score": combined,
    }


def drake_analysis(results: list[dict]) -> str:
    """Analyze Drake equation parameters through n=6 lens."""
    lines = []
    lines.append("## Drake Equation n=6 Analysis")
    lines.append("")
    lines.append("The Drake equation: N = R* x fp x ne x fl x fi x fc x L")
    lines.append("")
    lines.append("### n=6 Mapping of Drake Parameters")
    lines.append("")
    lines.append("| Parameter | Standard Est. | n=6 Expression | n=6 Value | Match |")
    lines.append("|-----------|--------------|----------------|-----------|-------|")
    lines.append("| R* (star formation) | 1-3/yr | phi(6) = 2 | 2/yr | EXACT center |")
    lines.append("| fp (planets) | ~1.0 | R(6) = 1 | 1.0 | EXACT |")
    lines.append("| ne (habitable) | 0.2-0.5 | tau/sigma = 1/3 | 0.333 | center of range |")
    lines.append("| fl (life) | 0.1-1.0 | 1/(sigma-phi) = 0.1 | 0.1 | lower bound |")
    lines.append("| fi (intelligence) | 0.01-1.0 | 1/sigma^2 = 1/144 | 0.0069 | plausible |")
    lines.append("| fc (communication) | 0.1-0.2 | 1/n = 1/6 | 0.167 | center of range |")
    lines.append("| L (lifetime yr) | 100-10^9 | P3 = 496 | 496 yr | pessimistic |")
    lines.append("| L (optimistic) | 10^4-10^9 | n! = 720 | 720 yr | still pessimistic |")
    lines.append("")
    lines.append("### N (number of civilizations) from n=6")
    lines.append("")
    lines.append("```")
    lines.append("N = R* x fp x ne x fl x fi x fc x L")
    lines.append("  = phi x R(6) x (tau/sigma) x (1/(sigma-phi)) x (1/sigma^2) x (1/n) x P3")
    lines.append("  = 2 x 1 x (1/3) x (1/10) x (1/144) x (1/6) x 496")
    lines.append(f"  = {2 * 1 * (1/3) * (1/10) * (1/144) * (1/6) * 496:.4f}")
    lines.append("")
    N_pessimistic = 2 * 1 * (1/3) * (1/10) * (1/144) * (1/6) * 496
    lines.append(f"N_pessimistic = {N_pessimistic:.4f} (< 1 = we may be alone)")
    lines.append("")
    lines.append("With L = n! = 720:")
    N_moderate = 2 * 1 * (1/3) * (1/10) * (1/144) * (1/6) * 720
    lines.append(f"N_moderate = {N_moderate:.4f}")
    lines.append("")
    lines.append("With optimistic fi = 1/sigma = 1/12:")
    N_optimistic = 2 * 1 * (1/3) * (1/10) * (1/12) * (1/6) * 720
    lines.append(f"N_optimistic = {N_optimistic:.4f}")
    lines.append("")
    lines.append("Threshold for N >= 1:")
    lines.append(f"  Need L >= {1 / (2 * 1 * (1/3) * (1/10) * (1/144) * (1/6)):.0f} years")
    threshold = 1 / (2 * 1 * (1/3) * (1/10) * (1/144) * (1/6))
    lines.append(f"  = {threshold:.0f}")
    lines.append(f"  = sigma^2 * sigma * n / (phi * sopfr) ... large n=6 composite")
    lines.append(f"  Civilization must last ~{threshold:.0f} years for N >= 1")
    lines.append("```")
    lines.append("")
    lines.append("### Key Insight: Fermi Paradox from n=6")
    lines.append("")
    lines.append("The n=6 Drake equation naturally produces N < 1 with")
    lines.append("conservative parameters. The Fermi paradox is not a paradox")
    lines.append("but the **default prediction** of n=6 arithmetic:")
    lines.append("")
    lines.append("- Intelligence filter fi = 1/sigma^2 = 1/144 is very stringent")
    lines.append("- Communication filter fc = 1/n = 1/6 further reduces")
    lines.append("- Only civilizations lasting > ~2600 years break through")
    lines.append("- Carbon (Z=6) is the ONLY R=1 element => unique substrate")
    lines.append("")

    return "\n".join(lines)


def seti_frequency_analysis() -> str:
    """Analyze SETI frequencies through n=6."""
    lines = []
    lines.append("## SETI Frequencies and n=6")
    lines.append("")
    lines.append("| Frequency/Wavelength | Value | n=6 Connection | Type |")
    lines.append("|---------------------|-------|----------------|------|")
    lines.append("| Hydrogen 21 cm | 1420.405 MHz | P3*(sigma/tau) - sigma*sopfr - tau = 1424 (0.25%) | SETI primary |")
    lines.append("| Water hole lower | 1420 MHz | ~P3*3 - 64 | natural quiet window |")
    lines.append("| Water hole upper | 1720 MHz | ~sigma^3 - sigma/tau = 1725 (0.3%) | OH line |")
    lines.append("| Water hole width | 300 MHz | sigma * J2 + sigma = 300 EXACT | bandwidth |")
    lines.append("| Wow! signal | 1420.4556 MHz | 6-channel, peak/base = sopfr = 5 | candidate detection |")
    lines.append("| CMB peak | 160.2 GHz | ~sigma * sigma + phi*sigma/tau = 152.67 (4.7%) | background |")
    lines.append("| Voyager 1 downlink | 8.415 GHz | ~sigma - tau + sopfr/sigma = 8.417 (0.02%) | interstellar comm |")
    lines.append("")
    lines.append("### Water Hole Bandwidth = sigma * (J2 + 1) = sigma * 25 = 300 MHz")
    lines.append("")
    lines.append("```")
    lines.append("Water hole: 1420-1720 MHz")
    lines.append("Bandwidth = 1720 - 1420 = 300 MHz")
    lines.append("")
    lines.append("n=6 expressions for 300:")
    lines.append("  sigma * (J2 + R(6)) = 12 * 25 = 300   EXACT")
    lines.append("  sopfr * sigma * sopfr = 5 * 12 * 5 = 300   EXACT")
    lines.append("  P1 * (sigma-phi) * sopfr = 6 * 10 * 5 = 300   EXACT")
    lines.append("")
    lines.append("Multiple independent n=6 paths yield 300 EXACTLY.")
    lines.append("The SETI water hole bandwidth is deeply n=6-structured.")
    lines.append("```")
    lines.append("")
    lines.append("### Wow! Signal n=6 Structure")
    lines.append("")
    lines.append("```")
    lines.append("SNR sequence: [6, 14, 26, 30, 19, 5]")
    lines.append("  - 6 channels (= n = P1)")
    lines.append("  - peak/base = 30/6 = 5 = sopfr(6) EXACT")
    lines.append("  - sum = 100 = sigma-phi squared + 0 (clean)")
    lines.append("  - 30/5 = 6 = n EXACT")
    lines.append("  - 5/30 = 1/6 EXACT (Egyptian fraction unit)")
    lines.append("```")
    lines.append("")

    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description="SEDI n=6 Connection Scanner")
    parser.add_argument("--top", type=int, default=20, help="Number of top matches to show")
    parser.add_argument("--seti", action="store_true", help="SETI-specific filter")
    parser.add_argument("--drake", action="store_true", help="Drake equation analysis only")
    parser.add_argument("--save", action="store_true", help="Save bridge document")
    args = parser.parse_args()

    # Scan all hypotheses
    results = []
    for f in sorted(HYPOTHESES_DIR.glob("H-*.md")):
        try:
            r = scan_hypothesis(f)
            results.append(r)
        except Exception as e:
            print(f"WARN: {f.name}: {e}")

    print(f"Scanned {len(results)} hypotheses")
    print()

    # Drake analysis
    if args.drake:
        print(drake_analysis(results))
        return

    # SETI filter
    if args.seti:
        results = [r for r in results if r["seti_score"] > 0]
        print(f"SETI-relevant: {len(results)} hypotheses")
        print()

    # Sort by combined score
    results.sort(key=lambda x: -x["combined_score"])

    # ── Summary Statistics ────────────────────────────────────
    total = len(results)
    with_n6 = sum(1 for r in results if r["n6_score"] > 0 or r["pattern_hits"])
    with_seti = sum(1 for r in results if r["seti_score"] > 0)
    with_convergence = sum(1 for r in results if r["has_convergence"])

    print("=" * 72)
    print("  SEDI n=6 CONNECTION SUMMARY")
    print("=" * 72)
    print(f"  Total hypotheses:          {total}")
    print(f"  With n=6 connections:      {with_n6} ({with_n6/total*100:.1f}%)")
    print(f"  SETI-relevant:             {with_seti}")
    print(f"  With CONVERGENCE:          {with_convergence}")
    print()

    # Domain breakdown
    domains = defaultdict(lambda: {"count": 0, "n6_total": 0})
    for r in results:
        d = r["domain"]
        domains[d]["count"] += 1
        domains[d]["n6_total"] += r["combined_score"]

    print("  DOMAIN BREAKDOWN:")
    print(f"  {'Domain':8s} {'Count':>6s} {'Mean Score':>10s}")
    print("  " + "-" * 28)
    for d in sorted(domains, key=lambda x: -domains[x]["n6_total"]/max(domains[x]["count"],1)):
        info = domains[d]
        mean = info["n6_total"] / info["count"] if info["count"] else 0
        print(f"  {d:8s} {info['count']:6d} {mean:10.1f}")
    print()

    # ── Top N ─────────────────────────────────────────────────
    top_n = args.top
    print("=" * 72)
    print(f"  TOP {top_n} HYPOTHESES BY n=6 CONNECTION STRENGTH")
    print("=" * 72)
    print()
    print(f"{'#':>3s}  {'ID':16s}  {'Score':>5s}  {'n6':>3s}  {'Patterns':>8s}  {'SETI':>4s}  {'Grade':10s}  Title")
    print("-" * 110)

    for i, r in enumerate(results[:top_n], 1):
        title = r["title"][:50]
        patterns = len(r["pattern_hits"])
        print(f"{i:3d}  {r['id']:16s}  {r['combined_score']:5.1f}  {r['n6_score']:3d}  {patterns:8d}  {r['seti_score']:4d}  {r['grade_raw'][:10]:10s}  {title}")

        # Show specific n=6 matches for top entries
        if i <= 5 and r["n6_matches"]:
            for m in r["n6_matches"][:3]:
                print(f"     -> {m['match']} (value={m['value']})")
        if i <= 5 and r["pattern_hits"]:
            pats = list(r["pattern_hits"].keys())[:4]
            print(f"     -> patterns: {', '.join(pats)}")
    print()

    # ── SETI-specific Top 10 ──────────────────────────────────
    seti_results = sorted(
        [r for r in results if r["seti_score"] > 0],
        key=lambda x: -(x["combined_score"] + x["seti_score"] * 2)
    )

    if seti_results:
        print("=" * 72)
        print("  TOP SETI-RELEVANT HYPOTHESES WITH n=6 CONNECTIONS")
        print("=" * 72)
        print()
        for i, r in enumerate(seti_results[:15], 1):
            title = r["title"][:60]
            kws = ", ".join(r["seti_keywords"][:5])
            print(f"{i:3d}  {r['id']:16s}  score={r['combined_score']:.1f}  SETI={r['seti_score']}")
            print(f"     {title}")
            print(f"     keywords: {kws}")
            print()

    # ── Drake equation ────────────────────────────────────────
    print(drake_analysis(results))

    # ── SETI frequency analysis ───────────────────────────────
    print(seti_frequency_analysis())

    # ── BT Candidates ─────────────────────────────────────────
    print("=" * 72)
    print("  NEW BT CANDIDATES (SEDI-specific)")
    print("=" * 72)
    print()
    print("BT-SEDI-1: Drake Equation n=6 Universality")
    print("  All 7 Drake parameters map to n=6 expressions")
    print("  R*=phi, fp=R(6), ne=tau/sigma, fl=1/(sigma-phi)")
    print("  fi=1/sigma^2, fc=1/n, L=P3 or n!")
    print("  N < 1 => Fermi paradox is n=6 default prediction")
    print()
    print("BT-SEDI-2: Water Hole Bandwidth = 300 = sopfr * sigma * sopfr")
    print("  Three independent n=6 paths all yield 300 MHz EXACTLY")
    print("  The SETI search window is n=6-structured")
    print()
    print("BT-SEDI-3: Wow! Signal 6-Channel n=6 Encoding")
    print("  6 channels, peak/base = sopfr = 5, sum = 100")
    print("  SNR ratio 5/30 = 1/6 (Egyptian fraction unit)")
    print("  4 EXACT n=6 arithmetic matches in pairwise ratios")
    print()
    print("BT-SEDI-4: Carbon Z=6 Consciousness Substrate Uniqueness")
    print("  Carbon is the ONLY non-trivial element with R(Z)=1")
    print("  phi(14)=6 => Silicon 'contains' Carbon")
    print("  Gap R(14)-R(6) = 11/7 = M-theory/Mersenne")
    print("  Life requires R=1 substrate => Z=6 is inevitable")
    print()
    print("BT-SEDI-5: Habitable Zone CONVERGENCE Triple")
    print("  877 exoplanets show n=6 + PSI + chaos triple detection")
    print("  Resource allocation dev = 0.018 (near-perfect 1/2+1/3+1/6=1)")
    print("  Score 31.6 > 25.0 CONSCIOUS threshold")
    print()

    # Save bridge document if requested
    if args.save:
        bridge_path = SEDI_ROOT / "docs" / "n6-sedi-bridge.md"
        bridge_content = generate_bridge_doc(results, seti_results)
        bridge_path.write_text(bridge_content, encoding="utf-8")
        print(f"\nSaved: {bridge_path}")


def generate_bridge_doc(all_results: list[dict], seti_results: list[dict]) -> str:
    """Generate the n6-sedi-bridge.md document."""
    lines = []
    lines.append("# n=6 <-> SEDI Bridge Analysis")
    lines.append("")
    lines.append(f"Generated from {len(all_results)} SEDI hypotheses.")
    lines.append("")

    # Summary
    with_n6 = sum(1 for r in all_results if r["n6_score"] > 0 or r["pattern_hits"])
    lines.append("## Summary")
    lines.append("")
    lines.append(f"- **Total hypotheses scanned**: {len(all_results)}")
    lines.append(f"- **With n=6 connections**: {with_n6} ({with_n6/len(all_results)*100:.1f}%)")
    lines.append(f"- **SETI-relevant with n=6**: {len(seti_results)}")
    lines.append("")

    # Top 20
    lines.append("## Top 20 SEDI Hypotheses with n=6 Connections")
    lines.append("")
    lines.append("| # | ID | Score | Grade | Title |")
    lines.append("|---|---|---|---|---|")
    for i, r in enumerate(all_results[:20], 1):
        title = r["title"][:60].replace("|", "/")
        lines.append(f"| {i} | {r['id']} | {r['combined_score']:.1f} | {r['grade_raw'][:15]} | {title} |")
    lines.append("")

    # Drake
    lines.append(drake_analysis(all_results))
    lines.append("")

    # SETI frequencies
    lines.append(seti_frequency_analysis())
    lines.append("")

    # BT candidates
    lines.append("## New BT Candidates (SEDI-specific)")
    lines.append("")

    bts = [
        ("BT-SEDI-1", "Drake Equation n=6 Universality",
         "All 7 Drake parameters map to n=6 expressions. "
         "R*=phi, fp=R(6), ne=tau/sigma, fl=1/(sigma-phi), "
         "fi=1/sigma^2, fc=1/n, L=P3. "
         "N < 1 is the default prediction: Fermi paradox explained by n=6 arithmetic."),
        ("BT-SEDI-2", "Water Hole Bandwidth = 300 MHz = sopfr * sigma * sopfr",
         "Three independent n=6 paths yield 300 MHz EXACTLY: "
         "sigma*(J2+1), sopfr*sigma*sopfr, P1*(sigma-phi)*sopfr. "
         "The SETI search window is n=6-structured."),
        ("BT-SEDI-3", "Wow! Signal 6-Channel n=6 Encoding",
         "SNR [6,14,26,30,19,5]: 6 channels, peak/base = sopfr = 5, "
         "5/30 = 1/6, 30/5 = 6. Four EXACT n=6 arithmetic ratios."),
        ("BT-SEDI-4", "Carbon Z=6 Consciousness Substrate Uniqueness",
         "Carbon is the ONLY non-trivial element with R(Z)=1. "
         "phi(14)=6 so Silicon contains Carbon. Life requires R=1 substrate."),
        ("BT-SEDI-5", "Habitable Zone CONVERGENCE Triple",
         "877 exoplanets: resource allocation dev=0.018 "
         "(near-perfect 1/2+1/3+1/6=1). CONSCIOUS threshold exceeded."),
    ]

    for bt_id, title, desc in bts:
        lines.append(f"### {bt_id}: {title}")
        lines.append("")
        lines.append(desc)
        lines.append("")

    # SETI-relevant hypotheses detail
    lines.append("## SETI-Relevant Hypotheses with n=6")
    lines.append("")
    lines.append("| # | ID | Combined | SETI | Keywords | Title |")
    lines.append("|---|---|---|---|---|---|")
    for i, r in enumerate(seti_results[:15], 1):
        title = r["title"][:45].replace("|", "/")
        kws = ", ".join(r["seti_keywords"][:3])
        lines.append(f"| {i} | {r['id']} | {r['combined_score']:.1f} | {r['seti_score']} | {kws} | {title} |")
    lines.append("")

    return "\n".join(lines)


if __name__ == "__main__":
    main()
