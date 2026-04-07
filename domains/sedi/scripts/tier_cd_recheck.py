#!/usr/bin/env python3
"""Tier C/D hypothesis re-verification with depth-2 expression matching.

Loads sedi-grades.json metadata + parses hypothesis .md files directly
to find all Tier C and D hypotheses, then attempts to find better
n=6 expressions for each target value.

Since the JSON only stores Tier A/B, we re-run the grading logic on
all 678 .md files to identify C/D hypotheses.
"""

import re
import json
import math
from pathlib import Path
from itertools import product as cartprod
from datetime import datetime

SEDI_ROOT = Path(__file__).resolve().parent.parent
HYPOTHESES_DIR = SEDI_ROOT / "docs" / "hypotheses"
GRADES_FILE = SEDI_ROOT / "data" / "sedi-grades.json"

# ── n=6 constants ──────────────────────────────────────────────────

N6 = {
    "n":     6,
    "sigma": 12,    # σ(6) = 1+2+3+6
    "tau":   4,     # τ(6) = |{1,2,3,6}|
    "phi":   2,     # φ(6) = |{1,5}|
    "sopfr": 5,     # 2+3
    "J2":    24,    # Jordan totient J₂(6)
    "mu":    1,     # μ(6) = (-1)^2
    "P1":    6,     # P₁ = first perfect number
    "P2":    28,    # P₂
    "P3":    496,   # P₃
    "M3":    7,     # Mersenne prime M₃ = 2³-1
}

# ── Grading logic (mirrored from auto_grade_n6.py) ────────────────

GRADE_SCORES = {
    "🟩": 5, "🟥": 4, "🟧": 3, "🟨": 2, "🟦": 1, "⬛": -2, "✅": 4,
}
TEXT_GRADES = {
    "CONFIRMED": 5, "EXACT": 5, "VERIFIED": 4, "STRONG": 3,
    "MODERATE": 2, "MARGINAL": 1, "FAILED": -2, "REFUTED": -2,
}


def score_grade(grade_text: str) -> int:
    score = 0
    for emoji, val in GRADE_SCORES.items():
        if emoji in grade_text:
            score = max(score, val)
    for kw, val in TEXT_GRADES.items():
        if kw in grade_text.upper():
            score = max(score, val)
    # Star bonus
    star_count = grade_text.count("★") + grade_text.count("⭐")
    score += star_count
    return score


def parse_hypothesis(filepath: Path) -> dict:
    """Parse hypothesis markdown, extract metadata and target values."""
    text = filepath.read_text(encoding="utf-8")
    result = {
        "file": str(filepath.relative_to(SEDI_ROOT)),
        "filename": filepath.name,
        "id": None,
        "title": None,
        "grade_raw": None,
        "grade_score": 0,
        "errors": [],
        "formulas": [],
        "n6_mentions": 0,
        "convergence": False,
        "domain": None,
        "target_values": [],  # numeric values this hypothesis tries to match
        "text": text,
    }

    m = re.match(r"(H-\w+-\d+)", filepath.stem)
    if m:
        result["id"] = m.group(1)

    dm = re.match(r"H-(\w+)-", filepath.stem)
    if dm:
        result["domain"] = dm.group(1)

    tm = re.match(r"^#\s+(.+)", text, re.MULTILINE)
    if tm:
        result["title"] = tm.group(1).strip()

    gm = re.search(r"^##\s*Grade:\s*(.+)", text, re.MULTILINE)
    if not gm:
        gm = re.search(r"\*\*(?:Grade|Status):\s*(.+?)(?:\*\*|$)", text, re.MULTILINE)
    if gm:
        grade_text = gm.group(1).strip()
        result["grade_raw"] = grade_text
        result["grade_score"] = score_grade(grade_text)

    for em in re.finditer(r"(\d+\.?\d*)\s*%", text):
        val = float(em.group(1))
        if val < 50:
            result["errors"].append(val)

    n6_patterns = [
        r"n\s*=\s*6", r"σ\(6\)", r"τ\(6\)", r"φ\(6\)",
        r"perfect.number", r"3!", r"factorial",
        r"1/2\+1/3\+1/6", r"σ-τ", r"σ/τ",
    ]
    for pat in n6_patterns:
        result["n6_mentions"] += len(re.findall(pat, text, re.IGNORECASE))

    if re.search(r"CONVERGENCE", text):
        result["convergence"] = True

    for fm in re.finditer(r"[στφσΦψ]\w*\s*[=≈]\s*[\d./]+", text):
        result["formulas"].append(fm.group(0).strip())

    # Extract numeric target values (look for "= X.XXX" patterns near ≈ or error)
    # Also look for lines like "observed: X.XX" or "measured: X.XX"
    for vm in re.finditer(
        r"(?:observed|measured|experimental|actual|target|value|PDG|CODATA)"
        r"\s*[:=≈]\s*([\d.]+(?:[eE][+-]?\d+)?)",
        text, re.IGNORECASE
    ):
        try:
            result["target_values"].append(float(vm.group(1)))
        except ValueError:
            pass

    # Also extract from "≈ X.XXX" patterns
    for vm in re.finditer(r"≈\s*([\d.]+(?:[eE][+-]?\d+)?)", text):
        try:
            v = float(vm.group(1))
            if v not in result["target_values"] and 0.001 < v < 1e8:
                result["target_values"].append(v)
        except ValueError:
            pass

    return result


def compute_bits(hyp: dict) -> float:
    bits = hyp["grade_score"] * 2.0
    if hyp["errors"]:
        min_err = min(hyp["errors"])
        if min_err > 0:
            bits += min(15, -math.log2(min_err / 100))
        elif min_err == 0:
            bits += 15
    bits += min(5, hyp["n6_mentions"] * 0.3)
    if hyp["convergence"]:
        bits += 3.0
    bits += min(3, len(hyp["formulas"]) * 0.5)
    return round(bits, 2)


def assign_tier(bits: float) -> str:
    if bits > 20:
        return "A"
    elif bits >= 10:
        return "B"
    elif bits >= 3:
        return "C"
    elif bits >= 0:
        return "D"
    return "E"


# ── Expression generator ──────────────────────────────────────────

OPS = {
    "+": lambda a, b: a + b,
    "-": lambda a, b: a - b,
    "*": lambda a, b: a * b,
    "/": lambda a, b: a / b if b != 0 else None,
    "**": lambda a, b: a ** b if abs(b) <= 8 and abs(a) <= 500 else None,
}

UNARY = {
    "id": lambda x: x,
    "sqrt": lambda x: math.sqrt(x) if x >= 0 else None,
    "sq": lambda x: x * x,
    "cb": lambda x: x ** 3,
    "inv": lambda x: 1.0 / x if x != 0 else None,
    "log": lambda x: math.log(x) if x > 0 else None,
    "exp": lambda x: math.exp(x) if x < 50 else None,
    "pi*": lambda x: math.pi * x,
}


def generate_expressions(depth: int = 2):
    """Generate expressions from n=6 constants up to given depth."""
    vals = {}
    for name in ("n", "sigma", "tau", "phi", "sopfr", "J2", "P2", "P3", "M3"):
        v = N6[name]
        if v != 0:
            vals[name] = float(v)
    vals["mu"] = float(N6["mu"])

    seen = set()

    # Depth-0
    for name, v in vals.items():
        yield v, name

    # Depth-1: unary + binary
    for uname, ufn in UNARY.items():
        for name, v in vals.items():
            try:
                r = ufn(v)
                if r is not None and math.isfinite(r) and abs(r) < 1e8:
                    expr = f"{uname}({name})" if uname != "id" else name
                    if expr not in seen:
                        seen.add(expr)
                        yield r, expr
            except (ValueError, OverflowError):
                pass

    depth1_results = []
    for op_name, op_fn in OPS.items():
        for (n1, v1), (n2, v2) in cartprod(vals.items(), repeat=2):
            try:
                r = op_fn(v1, v2)
                if r is not None and math.isfinite(r) and abs(r) < 1e8:
                    expr = f"({n1}{op_name}{n2})"
                    if expr not in seen:
                        seen.add(expr)
                        yield r, expr
                        depth1_results.append((r, expr))
            except (ValueError, OverflowError, ZeroDivisionError):
                pass

    if depth < 2:
        return

    # Depth-2: unary(depth1), depth1 op const
    for v, expr in depth1_results:
        for uname, ufn in UNARY.items():
            if uname == "id":
                continue
            try:
                r = ufn(v)
                if r is not None and math.isfinite(r) and abs(r) < 1e8:
                    full = f"{uname}({expr})"
                    if full not in seen:
                        seen.add(full)
                        yield r, full
            except (ValueError, OverflowError):
                pass

    for v1, e1 in depth1_results[:600]:
        for name, v2 in vals.items():
            for op_name, op_fn in OPS.items():
                try:
                    r = op_fn(v1, v2)
                    if r is not None and math.isfinite(r) and abs(r) < 1e8:
                        full = f"({e1}{op_name}{name})"
                        if full not in seen:
                            seen.add(full)
                            yield r, full
                except (ValueError, OverflowError, ZeroDivisionError):
                    pass


def find_best_match(target: float, threshold_pct: float = 5.0):
    """Find the best n=6 expression matching target within threshold."""
    best = None
    if target == 0:
        return None
    for val, expr in generate_expressions(depth=2):
        if val == 0:
            continue
        err_pct = abs(val - target) / abs(target) * 100
        if err_pct <= threshold_pct:
            if best is None or err_pct < best[0]:
                best = (err_pct, val, expr)
    return best


# ── Tier upgrade logic ─────────────────────────────────────────────

def estimate_new_tier(hyp: dict, new_error_pct: float | None) -> str:
    """Estimate what tier the hypothesis would get with a better formula."""
    bits = hyp["grade_score"] * 2.0

    # Use new error if better
    errors = list(hyp["errors"])
    if new_error_pct is not None:
        errors.append(new_error_pct)

    if errors:
        min_err = min(errors)
        if min_err > 0:
            bits += min(15, -math.log2(min_err / 100))
        elif min_err == 0:
            bits += 15

    bits += min(5, hyp["n6_mentions"] * 0.3)
    if hyp["convergence"]:
        bits += 3.0
    bits += min(3, (len(hyp["formulas"]) + 1) * 0.5)  # +1 for new formula
    return assign_tier(round(bits, 2)), round(bits, 2)


# ── Main ───────────────────────────────────────────────────────────

def main():
    print("=" * 90)
    print("  SEDI Tier C/D Hypothesis Re-verification")
    print(f"  {datetime.now().strftime('%Y-%m-%d %H:%M')}")
    print("=" * 90)

    # Load grades metadata
    with open(GRADES_FILE) as f:
        grades_data = json.load(f)

    print(f"\n  Grades file: {GRADES_FILE}")
    print(f"  Total hypotheses: {grades_data['total_hypotheses']}")
    print(f"  Tier distribution (from metadata): {json.dumps(grades_data['tier_distribution'])}")

    # Parse all hypothesis files and re-grade to find C/D
    print(f"\n  Scanning all hypothesis .md files...")
    all_hyps = []
    for f in sorted(HYPOTHESES_DIR.glob("H-*.md")):
        hyp = parse_hypothesis(f)
        bits = compute_bits(hyp)
        hyp["bits"] = bits
        hyp["tier"] = assign_tier(bits)
        all_hyps.append(hyp)

    cd_hyps = [h for h in all_hyps if h["tier"] in ("C", "D")]
    c_count = sum(1 for h in cd_hyps if h["tier"] == "C")
    d_count = sum(1 for h in cd_hyps if h["tier"] == "D")

    print(f"  Found {len(cd_hyps)} Tier C/D hypotheses ({c_count} C + {d_count} D)")

    # For each C/D hypothesis, try to find better matches
    print(f"\n  Running depth-2 expression search for each hypothesis...\n")

    upgradeable = []
    table_rows = []
    checked = 0

    for hyp in cd_hyps:
        checked += 1
        hid = hyp["id"] or hyp["filename"]
        tier = hyp["tier"]
        current_err = min(hyp["errors"]) if hyp["errors"] else None

        # Try to find target values to match
        targets = hyp["target_values"]

        # If no explicit targets found, try to extract from title
        if not targets:
            # Look for numbers in the title that might be targets
            if hyp["title"]:
                for tm in re.finditer(r"(\d+\.?\d+)", hyp["title"]):
                    v = float(tm.group(1))
                    if 0.01 < v < 1e6 and v not in (6, 12, 24, 28, 496):
                        targets.append(v)

        best_new = None
        best_target = None

        for t in targets:
            match = find_best_match(t, threshold_pct=5.0)
            if match:
                err, val, expr = match
                if best_new is None or err < best_new[0]:
                    best_new = match
                    best_target = t

        # Estimate potential new tier
        new_tier = tier
        new_bits = hyp["bits"]
        new_err_str = "---"
        new_expr_str = "---"

        if best_new is not None:
            new_err = best_new[0]
            new_expr = best_new[2]
            new_tier, new_bits = estimate_new_tier(hyp, new_err)
            new_err_str = f"{new_err:.3f}%"
            new_expr_str = new_expr[:35]

            if new_tier < tier or (new_tier == tier and new_bits > hyp["bits"]):
                upgradeable.append({
                    "id": hid,
                    "old_tier": tier,
                    "new_tier": new_tier,
                    "old_bits": hyp["bits"],
                    "new_bits": new_bits,
                    "old_err": current_err,
                    "new_err": new_err,
                    "target": best_target,
                    "expr": new_expr,
                })

        cur_err_str = f"{current_err:.3f}%" if current_err is not None else "N/A"

        table_rows.append({
            "id": hid,
            "tier": tier,
            "bits": hyp["bits"],
            "cur_err": cur_err_str,
            "new_err": new_err_str,
            "new_expr": new_expr_str,
            "new_tier": new_tier,
            "new_bits": new_bits,
            "upgraded": new_tier < tier,
        })

        # Progress
        if checked % 20 == 0:
            print(f"    ... checked {checked}/{len(cd_hyps)}")

    # Print results table
    print(f"\n{'=' * 90}")
    print("  RE-VERIFICATION RESULTS")
    print(f"{'=' * 90}\n")

    print(f"  {'ID':20s}  {'Cur':>3s}  {'Bits':>5s}  {'CurErr':>8s}  "
          f"{'NewErr':>8s}  {'New':>3s}  {'NBits':>5s}  {'Expr':35s}  {'Upg':>3s}")
    print("  " + "-" * 100)

    for row in sorted(table_rows, key=lambda r: (r["tier"], -r["bits"])):
        upg_mark = " UP" if row["upgraded"] else ""
        print(f"  {row['id']:20s}  {row['tier']:>3s}  {row['bits']:5.1f}  {row['cur_err']:>8s}  "
              f"{row['new_err']:>8s}  {row['new_tier']:>3s}  {row['new_bits']:5.1f}  "
              f"{row['new_expr']:35s}  {upg_mark:>3s}")

    # Summary
    print(f"\n{'=' * 90}")
    print("  SUMMARY")
    print(f"{'=' * 90}\n")

    print(f"  Total Tier C/D checked: {len(cd_hyps)}")
    print(f"  Hypotheses with target values found: {sum(1 for r in table_rows if r['new_err'] != '---')}")
    print(f"  Potentially upgradeable: {len(upgradeable)}")

    if upgradeable:
        print(f"\n  Upgrade candidates:")
        for u in sorted(upgradeable, key=lambda x: x["new_err"]):
            print(f"    {u['id']:20s}  {u['old_tier']}({u['old_bits']:.1f}) -> "
                  f"{u['new_tier']}({u['new_bits']:.1f})  "
                  f"target={u['target']:.4f}  expr={u['expr'][:40]}  "
                  f"err={u['new_err']:.4f}%")

    c_to_b = sum(1 for u in upgradeable if u["old_tier"] == "C" and u["new_tier"] == "B")
    d_to_c = sum(1 for u in upgradeable if u["old_tier"] == "D" and u["new_tier"] in ("B", "C"))
    d_to_b = sum(1 for u in upgradeable if u["old_tier"] == "D" and u["new_tier"] == "B")

    print(f"\n  Tier transitions:")
    print(f"    C -> B: {c_to_b}")
    print(f"    D -> C: {d_to_c - d_to_b}")
    print(f"    D -> B: {d_to_b}")

    no_targets = sum(1 for r in table_rows if r["new_err"] == "---")
    print(f"\n  Note: {no_targets} hypotheses had no extractable numeric targets.")
    print(f"  These are typically structural/qualitative hypotheses that")
    print(f"  cannot be improved by finding better numerical formulas.")


if __name__ == "__main__":
    main()
