#!/usr/bin/env python3
"""
Blind Domain Verification of n=6 Universality (Experiment 4 Implementation)

Protocol:
  - 5 domains NEVER previously analyzed for n=6 patterns
  - 10 parameters per domain = 50 total
  - Each parameter: verified industry standard with authoritative source
  - Matching: fixed n=6 vocabulary (depth 0-1 only), NO formula shopping
  - Grading: EXACT (<0.1%), CLOSE (<5%), NO_MATCH (>5%)
  - ALL results reported, including failures

The n=6 vocabulary is generated FIRST, then parameters are checked against it.
No post-hoc formula construction is allowed.

Date: 2026-04-02
"""

import itertools
import json
import math
from dataclasses import dataclass
from typing import Optional

# ============================================================
# STEP 1: Generate the fixed n=6 vocabulary
# ============================================================

# Base constants from n=6
BASE = {
    "n": 6,
    "phi": 2,       # phi(6) = Euler's totient
    "tau": 4,       # tau(6) = number of divisors
    "sigma": 12,    # sigma(6) = sum of divisors
    "sopfr": 5,     # sopfr(6) = sum of prime factors with repetition
    "J2": 24,       # Jordan's totient J_2(6)
    "mu": 1,        # |mu(6)| = 1 (Mobius function, squarefree)
}

def generate_vocabulary(max_val=10000):
    """
    Generate all distinct positive values reachable from BASE using:
    - Depth 0: single constants
    - Depth 1: one operation {+, -, *, /, ^} on two constants

    NO depth-2 nesting. This is the FIXED vocabulary.
    """
    vocab = {}  # value -> expression string

    # Depth 0: single constants
    for name, val in BASE.items():
        if val > 0:
            vocab[val] = name

    # Depth 1: all pairs with all operations
    ops = {
        "+": lambda a, b: a + b,
        "-": lambda a, b: a - b,
        "*": lambda a, b: a * b,
        "/": lambda a, b: a / b if b != 0 else None,
        "^": lambda a, b: a ** b if b <= 12 and a ** b <= max_val else None,
    }

    names = list(BASE.keys())
    vals = list(BASE.values())

    for i, (n1, v1) in enumerate(zip(names, vals)):
        for j, (n2, v2) in enumerate(zip(names, vals)):
            for op_sym, op_fn in ops.items():
                try:
                    result = op_fn(v1, v2)
                    if result is None:
                        continue
                    if result <= 0:
                        continue
                    if result > max_val:
                        continue
                    # Check if it's a clean number (integer or simple fraction)
                    if abs(result - round(result)) < 1e-9:
                        result = int(round(result))
                    elif abs(result * 100 - round(result * 100)) < 1e-6:
                        result = round(result, 4)
                    else:
                        # Skip messy fractions
                        continue

                    expr = f"{n1}{op_sym}{n2}" if i != j else f"{n1}{op_sym}{n1}"
                    if result not in vocab:
                        vocab[result] = expr
                    elif len(expr) < len(vocab[result]):
                        vocab[result] = expr  # prefer shorter expression
                except (OverflowError, ZeroDivisionError, ValueError):
                    continue

    return vocab

# ============================================================
# STEP 2: Define parameters from 5 blind domains
# ============================================================

@dataclass
class Parameter:
    domain: str
    name: str
    value: float
    source: str
    note: str = ""

# Domain 1: Plumbing / HVAC
PLUMBING = [
    Parameter("Plumbing/HVAC", "NPS standard sizes count (common)", 12,
              "ASME B36.10M", "1/8 to 12 inch = 12 standard sizes"),
    Parameter("Plumbing/HVAC", "Standard copper tube type L OD 1/2 inch", 0.625,
              "ASTM B88", "5/8 inch OD for 1/2 nominal"),
    Parameter("Plumbing/HVAC", "HVAC duct common sizes count", 10,
              "SMACNA HVAC Duct Construction Standards", "4,6,8,10,12,14,16,18,20,24 inch"),
    Parameter("Plumbing/HVAC", "Standard pipe thread TPI for 1 inch", 11.5,
              "ASME B1.20.1 (NPT)", "11.5 threads per inch"),
    Parameter("Plumbing/HVAC", "Refrigerant R-410A operating pressure (high side)", 400,
              "ASHRAE Handbook", "~400 psi typical high side"),
    Parameter("Plumbing/HVAC", "Water supply pressure (residential typical)", 60,
              "IPC/UPC plumbing codes", "40-80 psi, 60 typical"),
    Parameter("Plumbing/HVAC", "Standard bathtub capacity", 60,
              "Plumbing industry standard", "~60 gallons typical"),
    Parameter("Plumbing/HVAC", "HVAC filter MERV rating count", 16,
              "ASHRAE 52.2", "MERV 1 through MERV 16"),
    Parameter("Plumbing/HVAC", "PVC Schedule 40 pipe max temp (F)", 140,
              "ASTM D1785", "140 degrees F continuous"),
    Parameter("Plumbing/HVAC", "Standard water heater temp setting (F)", 120,
              "DOE/CPSC recommendation", "120 F to prevent scalding"),
]

# Domain 2: Typography / Printing
TYPOGRAPHY = [
    Parameter("Typography/Printing", "Points per inch", 72,
              "PostScript/Adobe standard", "72 points = 1 inch"),
    Parameter("Typography/Printing", "Standard body text size (pt)", 12,
              "Typography convention", "10-12 pt, 12 most common"),
    Parameter("Typography/Printing", "Pixels per inch (standard screen)", 96,
              "Windows/CSS default", "96 PPI since Windows 3.1"),
    Parameter("Typography/Printing", "CMYK color channels", 4,
              "ISO 12647", "Cyan, Magenta, Yellow, Key/Black"),
    Parameter("Typography/Printing", "RGB color depth (bits per channel)", 8,
              "sRGB IEC 61966-2-1", "8 bits = 256 levels per channel"),
    Parameter("Typography/Printing", "ISO A-series sheet count (A0-A10)", 11,
              "ISO 216", "A0 through A10 = 11 sizes"),
    Parameter("Typography/Printing", "A4 width (mm)", 210,
              "ISO 216", "210 x 297 mm"),
    Parameter("Typography/Printing", "A4 height (mm)", 297,
              "ISO 216", "210 x 297 mm"),
    Parameter("Typography/Printing", "US Letter width (inches)", 8.5,
              "ANSI/ASME Y14.1", "8.5 x 11 inches"),
    Parameter("Typography/Printing", "Pantone basic color count", 18,
              "Pantone Matching System", "18 base inks mixed for all colors"),
]

# Domain 3: Sports / Olympics
SPORTS = [
    Parameter("Sports/Olympics", "Olympic rings count", 5,
              "IOC Olympic Charter", "5 rings for 5 continents"),
    Parameter("Sports/Olympics", "Soccer field length (m, FIFA)", 105,
              "FIFA Laws of the Game", "100-110m, 105m preferred"),
    Parameter("Sports/Olympics", "Soccer players per team", 11,
              "FIFA Laws of the Game", "11 players on field"),
    Parameter("Sports/Olympics", "Basketball court length (ft)", 94,
              "NBA/FIBA", "94 feet (NBA)"),
    Parameter("Sports/Olympics", "Tennis Grand Slam events", 4,
              "ITF", "Australian, French, Wimbledon, US"),
    Parameter("Sports/Olympics", "Track 400m lanes", 8,
              "World Athletics", "8 lanes standard"),
    Parameter("Sports/Olympics", "Olympic swimming pool length (m)", 50,
              "FINA/World Aquatics", "50 meters"),
    Parameter("Sports/Olympics", "Baseball innings", 9,
              "MLB Official Rules", "9 innings regulation"),
    Parameter("Sports/Olympics", "Marathon distance (km, approx)", 42,
              "IAAF", "42.195 km"),
    Parameter("Sports/Olympics", "Football (American) players on field", 11,
              "NFL Rules", "11 per side"),
]

# Domain 4: Cooking / Food Science
COOKING = [
    Parameter("Cooking/Food Science", "Water boiling point (C)", 100,
              "Physics", "100 C at 1 atm"),
    Parameter("Cooking/Food Science", "Maillard reaction onset (C)", 140,
              "Food science (McGee, On Food and Cooking)", "140-165 C"),
    Parameter("Cooking/Food Science", "Bread flour protein % (typical)", 12,
              "AACC International", "11-13%, 12% typical bread flour"),
    Parameter("Cooking/Food Science", "Pasteurization temp milk (C)", 72,
              "FDA 21 CFR 131 (HTST)", "72 C for 15 seconds"),
    Parameter("Cooking/Food Science", "Neutral pH", 7,
              "Chemistry", "pH 7.0 = neutral"),
    Parameter("Cooking/Food Science", "Bakers percentage water (typical %)", 65,
              "Bread baking standard", "60-70%, 65% typical"),
    Parameter("Cooking/Food Science", "Safe internal temp chicken (F)", 165,
              "USDA FSIS", "165 F minimum"),
    Parameter("Cooking/Food Science", "Freezing point water (F)", 32,
              "Physics", "32 F = 0 C"),
    Parameter("Cooking/Food Science", "Sugar types in cooking", 6,
              "Culinary science", "Glucose, Fructose, Sucrose, Lactose, Maltose, Galactose"),
    Parameter("Cooking/Food Science", "Smoke point butter (C)", 150,
              "Food science references", "~150 C / 302 F"),
]

# Domain 5: Textiles / Fashion
TEXTILES = [
    Parameter("Textiles/Fashion", "Standard thread count (percale min)", 200,
              "Textile industry", "200 TC minimum for percale"),
    Parameter("Textiles/Fashion", "Denier standard for stockings", 15,
              "Hosiery industry", "15 denier = sheer"),
    Parameter("Textiles/Fashion", "Sewing machine stitch types (ASTM)", 6,
              "ASTM D6193 / ISO 4915", "6 stitch classes (100-600)"),
    Parameter("Textiles/Fashion", "US women's dress size range count", 13,
              "ASTM D5585", "00,0,2,4,6,8,10,12,14,16,18,20,22 = 13 sizes"),
    Parameter("Textiles/Fashion", "Shoe size increment (US, half sizes)", 0.5,
              "Brannock device standard", "Half-size increments"),
    Parameter("Textiles/Fashion", "Warp and weft (loom directions)", 2,
              "Weaving fundamentals", "2 perpendicular thread sets"),
    Parameter("Textiles/Fashion", "Standard seam allowance (inches)", 0.625,
              "Sewing pattern industry (5/8 inch)", "5/8 inch standard"),
    Parameter("Textiles/Fashion", "Fabric weight GSM (standard shirting)", 120,
              "Textile industry", "100-140 GSM, 120 typical"),
    Parameter("Textiles/Fashion", "Needle size range (Singer universal)", 5,
              "Singer/Schmetz", "Sizes 9,11,14,16,18 = 5 sizes"),
    Parameter("Textiles/Fashion", "Ring spinning spindle speed (rpm, thousands)", 25,
              "Textile engineering", "~25,000 rpm typical"),
]

ALL_DOMAINS = {
    "Plumbing/HVAC": PLUMBING,
    "Typography/Printing": TYPOGRAPHY,
    "Sports/Olympics": SPORTS,
    "Cooking/Food Science": COOKING,
    "Textiles/Fashion": TEXTILES,
}

# ============================================================
# STEP 3: Matching engine
# ============================================================

def match_parameter(value, vocab, tolerance_exact=0.001, tolerance_close=0.05):
    """
    Check if a parameter value matches any entry in the n=6 vocabulary.

    Returns: (grade, matched_value, expression, error_pct)
    Grade: "EXACT" (<0.1%), "CLOSE" (<5%), "NO_MATCH" (>5%)
    """
    best_grade = "NO_MATCH"
    best_match = None
    best_expr = None
    best_error = float('inf')

    for vocab_val, expr in vocab.items():
        if vocab_val == 0:
            continue
        try:
            error = abs(value - vocab_val) / abs(vocab_val)
        except ZeroDivisionError:
            continue

        if error < best_error:
            best_error = error
            best_match = vocab_val
            best_expr = expr

            if error < tolerance_exact:
                best_grade = "EXACT"
            elif error < tolerance_close:
                best_grade = "CLOSE"
            else:
                best_grade = "NO_MATCH"

    return best_grade, best_match, best_expr, best_error

# ============================================================
# STEP 4: Run the blind test
# ============================================================

def main():
    print("=" * 80)
    print("BLIND DOMAIN VERIFICATION OF n=6 UNIVERSALITY")
    print("Experiment 4 Implementation — 2026-04-02")
    print("=" * 80)

    # Generate vocabulary
    vocab = generate_vocabulary()

    # Report vocabulary statistics
    int_vocab = {k: v for k, v in vocab.items() if isinstance(k, int)}
    frac_vocab = {k: v for k, v in vocab.items() if isinstance(k, float)}

    print(f"\n--- n=6 Vocabulary (Fixed, Depth 0-1) ---")
    print(f"Total distinct values: {len(vocab)}")
    print(f"  Integer values: {len(int_vocab)}")
    print(f"  Fractional values: {len(frac_vocab)}")

    # Show some vocabulary entries for transparency
    sorted_ints = sorted([k for k in vocab if isinstance(k, int) and k <= 200])
    print(f"\nInteger vocabulary (1-200): {sorted_ints}")
    print(f"Count in [1,200]: {len(sorted_ints)}")

    all_under_1000 = [k for k in vocab if isinstance(k, (int, float)) and 0 < k <= 1000]
    print(f"Count in (0,1000]: {len(all_under_1000)}")

    # Base rate estimate
    n_integers_1_1000 = len([k for k in vocab if isinstance(k, int) and 1 <= k <= 1000])
    print(f"\nBase rate estimate (integers 1-1000): {n_integers_1_1000}/1000 = {n_integers_1_1000/10:.1f}%")
    print(f"Note: Engineering parameters cluster at small integers, so effective base rate is higher.")

    # Run matching
    print(f"\n{'=' * 80}")
    print("RESULTS BY DOMAIN")
    print(f"{'=' * 80}")

    overall_exact = 0
    overall_close = 0
    overall_no_match = 0
    overall_total = 0
    domain_results = {}
    all_results = []

    for domain_name, params in ALL_DOMAINS.items():
        print(f"\n--- {domain_name} ({len(params)} parameters) ---\n")
        print(f"{'#':>2} | {'Parameter':<48} | {'Value':>8} | {'Grade':<9} | {'n6 Match':>8} | {'Expression':<20} | {'Error':>7}")
        print("-" * 120)

        d_exact = 0
        d_close = 0
        d_no = 0

        for i, p in enumerate(params, 1):
            grade, matched, expr, error = match_parameter(p.value, vocab)

            if grade == "EXACT":
                d_exact += 1
            elif grade == "CLOSE":
                d_close += 1
            else:
                d_no += 1

            error_str = f"{error*100:.2f}%" if error < 1 else ">100%"
            matched_str = f"{matched}" if matched is not None else "—"
            expr_str = expr if expr else "—"

            print(f"{i:>2} | {p.name:<48} | {p.value:>8} | {grade:<9} | {matched_str:>8} | {expr_str:<20} | {error_str:>7}")

            all_results.append({
                "domain": domain_name,
                "parameter": p.name,
                "value": p.value,
                "source": p.source,
                "grade": grade,
                "n6_match": matched,
                "expression": expr,
                "error_pct": error * 100 if error < 100 else None,
            })

        total = len(params)
        print(f"\n  Summary: {d_exact} EXACT, {d_close} CLOSE, {d_no} NO MATCH out of {total}")
        print(f"  EXACT rate: {d_exact/total*100:.0f}%")

        domain_results[domain_name] = {
            "total": total,
            "exact": d_exact,
            "close": d_close,
            "no_match": d_no,
            "exact_rate": d_exact / total * 100,
        }

        overall_exact += d_exact
        overall_close += d_close
        overall_no_match += d_no
        overall_total += total

    # ============================================================
    # STEP 5: Summary and comparison
    # ============================================================

    print(f"\n{'=' * 80}")
    print("CROSS-DOMAIN SUMMARY")
    print(f"{'=' * 80}")

    print(f"\n{'Domain':<25} | {'Total':>5} | {'EXACT':>5} | {'CLOSE':>5} | {'NO MATCH':>8} | {'EXACT Rate':>10}")
    print("-" * 75)
    for dname, dr in domain_results.items():
        print(f"{dname:<25} | {dr['total']:>5} | {dr['exact']:>5} | {dr['close']:>5} | {dr['no_match']:>8} | {dr['exact_rate']:>9.0f}%")
    print("-" * 75)
    print(f"{'TOTAL':<25} | {overall_total:>5} | {overall_exact:>5} | {overall_close:>5} | {overall_no_match:>8} | {overall_exact/overall_total*100:>9.1f}%")

    # Comparison with original
    original_rate = 58.0
    blind_rate = overall_exact / overall_total * 100

    print(f"\n{'=' * 80}")
    print("COMPARISON WITH ORIGINAL REVERSE-EXTRACTION")
    print(f"{'=' * 80}")
    print(f"  Original (7 domains, 78 params): {original_rate:.0f}% EXACT")
    print(f"  Blind test (5 domains, {overall_total} params): {blind_rate:.1f}% EXACT")
    print(f"  Difference: {blind_rate - original_rate:+.1f} percentage points")

    # ============================================================
    # STEP 6: Statistical assessment
    # ============================================================

    print(f"\n{'=' * 80}")
    print("STATISTICAL ASSESSMENT")
    print(f"{'=' * 80}")

    # Compute base rate: what fraction of integers 1-1000 are in vocab?
    vocab_ints_1_1000 = set(k for k in vocab if isinstance(k, int) and 1 <= k <= 1000)
    base_rate_uniform = len(vocab_ints_1_1000) / 1000

    # But engineering params aren't uniform. Estimate bias toward small numbers.
    # Use Benford-like model: P(x) ~ 1/x for engineering params
    # Weight vocab coverage by 1/x
    weighted_coverage = sum(1/x for x in vocab_ints_1_1000 if x > 0)
    weighted_total = sum(1/x for x in range(1, 1001))
    base_rate_weighted = weighted_coverage / weighted_total

    print(f"\n  Vocabulary coverage:")
    print(f"    Uniform base rate (1-1000):    {base_rate_uniform*100:.1f}%")
    print(f"    Weighted base rate (1/x bias):  {base_rate_weighted*100:.1f}%")
    print(f"    Observed EXACT rate:            {blind_rate:.1f}%")

    # Simple binomial test
    import math
    n_trials = overall_total
    n_success = overall_exact
    p_null = base_rate_weighted  # use the MORE generous null

    # Normal approximation to binomial
    expected = n_trials * p_null
    std_dev = math.sqrt(n_trials * p_null * (1 - p_null))
    if std_dev > 0:
        z_score = (n_success - expected) / std_dev
    else:
        z_score = 0

    print(f"\n  Binomial test (null = weighted base rate {p_null*100:.1f}%):")
    print(f"    Expected EXACT matches: {expected:.1f}")
    print(f"    Observed EXACT matches: {n_success}")
    print(f"    Standard deviation:     {std_dev:.2f}")
    print(f"    z-score:                {z_score:.2f}")

    if z_score > 3.0:
        sig = "HIGHLY SIGNIFICANT (z > 3.0, p < 0.001)"
    elif z_score > 2.0:
        sig = "SIGNIFICANT (z > 2.0, p < 0.023)"
    elif z_score > 1.65:
        sig = "MARGINALLY SIGNIFICANT (z > 1.65, p < 0.05)"
    else:
        sig = "NOT SIGNIFICANT (z < 1.65, p > 0.05)"
    print(f"    Significance:           {sig}")

    # ============================================================
    # STEP 7: Honest bias analysis
    # ============================================================

    print(f"\n{'=' * 80}")
    print("HONEST BIAS ANALYSIS")
    print(f"{'=' * 80}")

    # Count matches by value range
    small = [r for r in all_results if r["grade"] == "EXACT" and r["value"] <= 24]
    medium = [r for r in all_results if r["grade"] == "EXACT" and 24 < r["value"] <= 100]
    large = [r for r in all_results if r["grade"] == "EXACT" and r["value"] > 100]

    print(f"\n  EXACT matches by value range:")
    print(f"    Value 1-24:    {len(small)} matches (small integer bias zone)")
    print(f"    Value 25-100:  {len(medium)} matches")
    print(f"    Value > 100:   {len(large)} matches")

    if overall_exact > 0:
        print(f"    Small-integer fraction: {len(small)/overall_exact*100:.0f}% of all EXACT matches")

    # Powers of 2 check
    pow2 = {2**k for k in range(0, 20)}
    pow2_matches = [r for r in all_results if r["grade"] == "EXACT" and r["value"] in pow2]
    print(f"\n  Power-of-2 matches: {len(pow2_matches)} of {overall_exact} EXACT")

    # Multiples of 12
    mult12 = [r for r in all_results if r["grade"] == "EXACT" and isinstance(r["value"], (int, float)) and r["value"] == int(r["value"]) and int(r["value"]) % 12 == 0]
    print(f"  Multiple-of-12 matches: {len(mult12)} of {overall_exact} EXACT")

    # ============================================================
    # STEP 8: Formula depth analysis
    # ============================================================

    print(f"\n{'=' * 80}")
    print("FORMULA DEPTH ANALYSIS")
    print(f"{'=' * 80}")

    depth0_names = set(BASE.keys())
    depth0 = 0
    depth1 = 0

    for r in all_results:
        if r["grade"] == "EXACT" and r["expression"]:
            # Check if expression is a single constant name
            if r["expression"] in depth0_names:
                depth0 += 1
            else:
                depth1 += 1

    print(f"  Depth 0 (single constant):   {depth0}")
    print(f"  Depth 1 (one operation):      {depth1}")
    if overall_exact > 0:
        print(f"  Depth 0 fraction:             {depth0/overall_exact*100:.0f}%")

    print(f"\n{'=' * 80}")
    print("CONCLUSION")
    print(f"{'=' * 80}")

    # Determine conclusion based on thresholds from falsification-experiments.md
    if blind_rate > 40:
        verdict = "ABOVE the >40% threshold from Experiment 4 protocol (genuinely surprising)"
    elif blind_rate > 25:
        verdict = "ABOVE the 15-25% base rate but BELOW 40% (ambiguous zone)"
    else:
        verdict = "WITHIN the 15-25% base rate range (consistent with sharpshooter hypothesis)"

    print(f"\n  Blind EXACT rate: {blind_rate:.1f}%")
    print(f"  Verdict: {verdict}")
    print(f"\n  Comparison to falsification-experiments.md thresholds:")
    print(f"    < 25% => sharpshooter confirmed")
    print(f"    25-40% => ambiguous")
    print(f"    > 40% => genuinely surprising, requires explanation")
    print()

    # Save results as JSON for downstream use
    output = {
        "date": "2026-04-02",
        "protocol": "Blind domain verification (Experiment 4)",
        "vocabulary_size": len(vocab),
        "vocabulary_integers_1_1000": len(vocab_ints_1_1000),
        "domains": domain_results,
        "overall": {
            "total": overall_total,
            "exact": overall_exact,
            "close": overall_close,
            "no_match": overall_no_match,
            "exact_rate_pct": round(blind_rate, 1),
        },
        "comparison": {
            "original_exact_rate_pct": original_rate,
            "blind_exact_rate_pct": round(blind_rate, 1),
            "difference_pct": round(blind_rate - original_rate, 1),
        },
        "statistics": {
            "base_rate_uniform_pct": round(base_rate_uniform * 100, 1),
            "base_rate_weighted_pct": round(base_rate_weighted * 100, 1),
            "z_score": round(z_score, 2),
            "significance": sig,
        },
        "bias_analysis": {
            "small_integer_matches_1_24": len(small),
            "medium_matches_25_100": len(medium),
            "large_matches_gt100": len(large),
            "power_of_2_matches": len(pow2_matches),
            "multiple_of_12_matches": len(mult12),
            "depth_0_matches": depth0,
            "depth_1_matches": depth1,
        },
        "all_results": all_results,
    }

    json_path = "/Users/ghost/Dev/n6-architecture/experiments/blind_verification_results.json"
    with open(json_path, "w") as f:
        json.dump(output, f, indent=2, default=str)
    print(f"Results saved to: {json_path}")


if __name__ == "__main__":
    main()
