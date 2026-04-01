#!/usr/bin/env python3
"""
N6 Architecture — Anomaly Detection
Find candidates with n6 < 0.50 across all TOML domain files.
For weak matches, attempt inverse formula discovery using depth-2 n6 expressions.
"""

import os
import re
import math
from collections import defaultdict
from pathlib import Path
from datetime import datetime

# ─── N6 base constants ───
SIGMA = 12
TAU = 4
PHI = 2
SOPFR = 5
J2 = 24
MU = 1
N = 6

BASE = {
    'n': N, 'sigma': SIGMA, 'tau': TAU, 'phi': PHI,
    'sopfr': SOPFR, 'J2': J2, 'mu': MU,
}

# ─── Generate ALL depth-2 n6 expressions ───
def generate_n6_expressions():
    """Generate n=6 derived expressions up to depth 2."""
    exprs = {}

    # Level 0: base constants
    for name, val in BASE.items():
        exprs[name] = val

    # Level 1: unary ops on bases
    names = list(BASE.keys())
    vals = list(BASE.values())

    for name, val in BASE.items():
        if val > 0:
            exprs[f'1/{name}'] = 1.0 / val
            if val <= 12:
                exprs[f'{name}^2'] = val ** 2
                exprs[f'{name}^3'] = val ** 3
                exprs[f'{name}^4'] = val ** 4
            if val >= 2:
                exprs[f'2^{name}'] = 2 ** val
            exprs[f'ln({name})'] = math.log(val) if val > 0 else None

    # Level 1: binary ops on pairs
    for i, (n1, v1) in enumerate(zip(names, vals)):
        for j, (n2, v2) in enumerate(zip(names, vals)):
            if i == j:
                continue
            exprs[f'{n1}+{n2}'] = v1 + v2
            exprs[f'{n1}-{n2}'] = v1 - v2
            exprs[f'{n1}*{n2}'] = v1 * v2
            if v2 != 0:
                exprs[f'{n1}/{n2}'] = v1 / v2
            if v1 > 0 and v2 > 0 and v2 <= 8:
                exprs[f'{n1}^{n2}'] = v1 ** v2

    # Level 2: combine level-1 results with bases
    level1 = {}
    for n1, v1 in zip(names, vals):
        for n2, v2 in zip(names, vals):
            if n1 == n2:
                continue
            key_add = f'({n1}+{n2})'
            key_sub = f'({n1}-{n2})'
            key_mul = f'({n1}*{n2})'
            key_div = f'({n1}/{n2})' if v2 != 0 else None
            level1[key_add] = v1 + v2
            level1[key_sub] = v1 - v2
            level1[key_mul] = v1 * v2
            if key_div:
                level1[key_div] = v1 / v2

    for l1name, l1val in level1.items():
        for bname, bval in BASE.items():
            exprs[f'{l1name}+{bname}'] = l1val + bval
            exprs[f'{l1name}-{bname}'] = l1val - bval
            exprs[f'{l1name}*{bname}'] = l1val * bval
            if bval != 0:
                exprs[f'{l1name}/{bname}'] = l1val / bval
            if l1val > 0 and 0 < bval <= 6:
                try:
                    v = l1val ** bval
                    if abs(v) < 1e15:
                        exprs[f'{l1name}^{bname}'] = v
                except:
                    pass
            if bval > 0 and 0 < l1val <= 6:
                try:
                    v = bval ** l1val
                    if abs(v) < 1e15:
                        exprs[f'{bname}^{l1name}'] = v
                except:
                    pass

    # Special well-known n6 expressions
    exprs['1/e'] = 1.0 / math.e
    exprs['ln(4/3)'] = math.log(4.0/3)
    exprs['1-1/e'] = 1 - 1.0/math.e
    exprs['e^(-1)'] = math.exp(-1)
    exprs['sigma*phi/(n*tau)'] = (SIGMA * PHI) / (N * TAU)  # R(6) = 1
    exprs['tau^2/sigma'] = TAU**2 / SIGMA  # 4/3
    exprs['phi/tau'] = PHI / TAU  # 1/2
    exprs['(sigma-phi)^tau'] = (SIGMA - PHI) ** TAU  # 10000
    exprs['sigma*(sigma-phi)'] = SIGMA * (SIGMA - PHI)  # 120
    exprs['sigma^2-phi'] = SIGMA**2 - PHI  # 142
    exprs['sigma*(sigma-tau)'] = SIGMA * (SIGMA - TAU)  # 96
    exprs['J2-tau'] = J2 - TAU  # 20
    exprs['tau/(n/phi)'] = TAU / (N/PHI)  # 4/3
    exprs['tau^2/(n/phi)^3'] = TAU**2 / (N/PHI)**3  # 16/27 Betz
    exprs['sigma/(sigma-phi)'] = SIGMA / (SIGMA - PHI)  # 1.2
    exprs['1/(sigma-phi)'] = 1.0 / (SIGMA - PHI)  # 0.1
    exprs['1-1/(sigma-phi)'] = 1 - 1.0/(SIGMA - PHI)  # 0.9
    exprs['1-1/(J2-tau)'] = 1 - 1.0/(J2 - TAU)  # 0.95
    exprs['sigma*sopfr'] = SIGMA * SOPFR  # 60
    exprs['sigma*tau'] = SIGMA * TAU  # 48
    exprs['sigma*n'] = SIGMA * N  # 72
    exprs['sigma^2'] = SIGMA**2  # 144
    exprs['sigma*J2'] = SIGMA * J2  # 288
    exprs['phi^tau*sopfr'] = PHI**TAU * SOPFR  # 80
    exprs['sopfr*(sigma-phi)'] = SOPFR * (SIGMA - PHI)  # 50
    exprs['sigma-sopfr'] = SIGMA - SOPFR  # 7
    exprs['sigma+mu'] = SIGMA + MU  # 13
    exprs['sigma-mu'] = SIGMA - MU  # 11
    exprs['sigma-tau'] = SIGMA - TAU  # 8
    exprs['sigma-phi'] = SIGMA - PHI  # 10
    exprs['n/phi'] = N / PHI  # 3
    exprs['J2+phi'] = J2 + PHI  # 26
    exprs['sigma*phi'] = SIGMA * PHI  # 24

    # Filter out None, inf, nan
    clean = {}
    for k, v in exprs.items():
        if v is not None and math.isfinite(v) and abs(v) < 1e12:
            clean[k] = v
    return clean


def extract_number_from_notes(label):
    """Try to extract a numeric value from candidate label/notes."""
    numbers = []
    # Match patterns like "123.45", "1.2V", "48kHz", "0.95", etc.
    for m in re.finditer(r'(?<![A-Za-z])(\d+\.?\d*)\s*(?:nm|mm|um|μm|eV|V|A|W|Hz|kHz|MHz|GHz|THz|K|°C|°F|GPa|MPa|Pa|ppm|%|GB|TB|MB|KB|Gbps|Mbps|mA|mV|mW|μW|nJ|pJ|fJ|ns|ps|fs|ms|s|g|kg|mg|μg|mol|J|cal|kcal|kWh|MWh|cm|m|km|Å|bar|atm|torr|lux|cd|lm|dB|pH|rpm|N|lb|in|ft|Ω|ohm|S|F|H|T|Wb|Sv|Bq|Gy|rem|rad|sr)?(?![A-Za-z0-9])', label):
        try:
            v = float(m.group(1))
            if v > 0:
                numbers.append(v)
        except:
            pass
    return numbers


def find_inverse_matches(value, n6_exprs, threshold=0.05):
    """Find n6 expressions that match the given value within threshold."""
    matches = []
    if value == 0:
        return matches
    for expr_name, expr_val in n6_exprs.items():
        if expr_val == 0:
            continue
        # Check relative error
        rel_err = abs(value - expr_val) / max(abs(value), abs(expr_val))
        if rel_err <= threshold:
            matches.append((expr_name, expr_val, rel_err))
    # Sort by error
    matches.sort(key=lambda x: x[2])
    return matches[:5]  # top 5


def parse_toml_simple(filepath):
    """Simple TOML parser for DSE domain files.
    Tracks section context: [meta], [scoring], [[level]], [[candidate]], [[rule]]
    Only extracts n6 from [[candidate]] sections.
    """
    candidates = []
    meta_name = os.path.basename(filepath).replace('.toml', '')

    current_level_idx = -1
    level_names = {}

    with open(filepath, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    # Section state machine
    section = None  # 'meta', 'scoring', 'level', 'candidate', 'rule', 'other'
    cand = {}

    for line in lines:
        raw = line.strip()
        if not raw or raw.startswith('#'):
            continue

        # Detect section headers
        if raw == '[meta]':
            if section == 'candidate' and cand.get('id'):
                candidates.append(cand)
                cand = {}
            section = 'meta'
            continue
        elif raw == '[scoring]':
            if section == 'candidate' and cand.get('id'):
                candidates.append(cand)
                cand = {}
            section = 'scoring'
            continue
        elif raw == '[[level]]':
            if section == 'candidate' and cand.get('id'):
                candidates.append(cand)
                cand = {}
            section = 'level'
            current_level_idx += 1
            continue
        elif raw == '[[candidate]]':
            if section == 'candidate' and cand.get('id'):
                candidates.append(cand)
            section = 'candidate'
            cand = {'domain': meta_name}
            continue
        elif raw == '[[rule]]':
            if section == 'candidate' and cand.get('id'):
                candidates.append(cand)
                cand = {}
            section = 'rule'
            continue
        elif raw.startswith('['):
            if section == 'candidate' and cand.get('id'):
                candidates.append(cand)
                cand = {}
            section = 'other'
            continue

        # Parse key = value
        if '=' not in raw:
            continue
        key, _, val = raw.partition('=')
        key = key.strip()
        val = val.strip().strip('"')

        if section == 'meta':
            if key == 'name':
                meta_name = val

        elif section == 'level':
            if key == 'name':
                level_names[current_level_idx] = val

        elif section == 'candidate':
            if key == 'level':
                try:
                    lv = int(val)
                    cand['level'] = lv
                    cand['level_name'] = level_names.get(lv, f'L{lv}')
                except:
                    pass
            elif key == 'id':
                cand['id'] = val
            elif key == 'label':
                cand['label'] = val
            elif key == 'n6':
                try:
                    cand['n6'] = float(val)
                except:
                    pass
            elif key == 'notes':
                cand['notes'] = val

    # Last candidate
    if section == 'candidate' and cand.get('id'):
        candidates.append(cand)

    return meta_name, candidates


def main():
    domains_dir = Path(os.path.expanduser('~/Dev/n6-architecture/tools/universal-dse/domains'))
    toml_files = sorted(domains_dir.glob('*.toml'))

    print(f"Found {len(toml_files)} TOML domain files")
    print("Generating n6 expression dictionary...")
    n6_exprs = generate_n6_expressions()
    print(f"  {len(n6_exprs)} expressions generated")

    # Parse all TOMLs
    all_candidates = []
    domain_stats = defaultdict(lambda: {'total': 0, 'anomalies': 0})

    for tf in toml_files:
        dname, cands = parse_toml_simple(tf)
        for c in cands:
            c.setdefault('domain', dname)
            all_candidates.append(c)
            domain_stats[dname]['total'] += 1
            if c.get('n6', 1.0) < 0.50:
                domain_stats[dname]['anomalies'] += 1

    print(f"Total candidates parsed: {len(all_candidates)}")

    # Filter anomalies (n6 < 0.50)
    anomalies = [c for c in all_candidates if c.get('n6', 1.0) < 0.50]
    print(f"Anomalies (n6 < 0.50): {len(anomalies)}")

    # Categorize anomalies
    genuinely_non_n6 = []
    undiscovered = []
    misclassified = []

    for a in anomalies:
        label = a.get('label', '') + ' ' + a.get('notes', '')
        numbers = extract_number_from_notes(label)

        found_match = False
        best_matches = []
        for num in numbers:
            matches = find_inverse_matches(num, n6_exprs, threshold=0.05)
            if matches:
                found_match = True
                best_matches.extend([(num, m) for m in matches])

        if found_match:
            a['inverse_matches'] = best_matches
            # If n6 is very low but we found matches -> potential misclassified or undiscovered
            if a.get('n6', 0) <= 0.20:
                undiscovered.append(a)
            else:
                undiscovered.append(a)
        else:
            genuinely_non_n6.append(a)

    print(f"\nCategorization:")
    print(f"  Genuinely non-n6: {len(genuinely_non_n6)}")
    print(f"  Undiscovered formula candidates: {len(undiscovered)}")

    # Domain anomaly rates
    domain_rates = []
    for dname, stats in domain_stats.items():
        if stats['total'] > 0:
            rate = stats['anomalies'] / stats['total']
            domain_rates.append((dname, stats['anomalies'], stats['total'], rate))
    domain_rates.sort(key=lambda x: -x[3])

    # Score distribution
    n6_buckets = defaultdict(int)
    for c in all_candidates:
        score = c.get('n6', 1.0)
        bucket = int(score * 10) / 10
        n6_buckets[bucket] += 1

    # ─── Generate output report ───
    output_path = Path(os.path.expanduser('~/Dev/n6-architecture/docs/anomaly-detection-results.md'))

    with open(output_path, 'w') as f:
        f.write(f"# N6 Anomaly Detection Results\n\n")
        f.write(f"> Generated: {datetime.now().strftime('%Y-%m-%d %H:%M')}\n")
        f.write(f"> Threshold: n6 < 0.50\n\n")

        f.write(f"## Summary Statistics\n\n")
        f.write(f"| Metric | Value |\n")
        f.write(f"|--------|-------|\n")
        f.write(f"| TOML files parsed | {len(toml_files)} |\n")
        f.write(f"| Total candidates | {len(all_candidates)} |\n")
        f.write(f"| Anomalies (n6 < 0.50) | {len(anomalies)} |\n")
        f.write(f"| Anomaly rate | {100*len(anomalies)/len(all_candidates):.1f}% |\n")
        f.write(f"| Genuinely non-n6 | {len(genuinely_non_n6)} |\n")
        f.write(f"| Undiscovered formula candidates | {len(undiscovered)} |\n")
        f.write(f"| N6 expressions checked | {len(n6_exprs)} |\n\n")

        # Score distribution
        f.write(f"## N6 Score Distribution (all candidates)\n\n")
        f.write(f"| Score Range | Count | Pct |\n")
        f.write(f"|-------------|-------|-----|\n")
        for bucket in sorted(n6_buckets.keys()):
            cnt = n6_buckets[bucket]
            pct = 100 * cnt / len(all_candidates)
            bar = '#' * int(pct)
            f.write(f"| {bucket:.1f}-{bucket+0.1:.1f} | {cnt} | {pct:.1f}% {bar} |\n")

        # Top anomaly domains
        f.write(f"\n## Domains with Highest Anomaly Rates\n\n")
        f.write(f"| Domain | Anomalies | Total | Rate |\n")
        f.write(f"|--------|-----------|-------|------|\n")
        for dname, anom, total, rate in domain_rates[:30]:
            if anom > 0:
                f.write(f"| {dname} | {anom} | {total} | {100*rate:.0f}% |\n")

        # Top 20 undiscovered formula candidates
        f.write(f"\n## Top 20 Undiscovered Formula Candidates\n\n")
        f.write(f"These anomalies have numeric values in their labels that match n6 expressions within 5%.\n\n")

        # Sort by lowest error
        scored = []
        for a in undiscovered:
            if 'inverse_matches' in a:
                best_err = min(m[1][2] for m in a['inverse_matches'])
                scored.append((best_err, a))
        scored.sort(key=lambda x: x[0])

        f.write(f"| # | Domain | Level | ID | n6 | Value | Matching Expression | Expr Value | Error |\n")
        f.write(f"|---|--------|-------|----|----|-------|---------------------|------------|-------|\n")
        for i, (err, a) in enumerate(scored[:20]):
            best = min(a['inverse_matches'], key=lambda x: x[1][2])
            num_val = best[0]
            expr_name = best[1][0]
            expr_val = best[1][1]
            rel_err = best[1][2]
            f.write(f"| {i+1} | {a.get('domain','')} | {a.get('level_name','')} "
                    f"| {a.get('id','')} | {a.get('n6',0):.2f} "
                    f"| {num_val} | {expr_name} | {expr_val:.4f} | {100*rel_err:.2f}% |\n")

        # All undiscovered formula matches (detailed)
        f.write(f"\n## All New Formula Candidates (n6 < 0.50 with inverse match)\n\n")
        f.write(f"Total: {len(undiscovered)} candidates with potential n6 connections\n\n")

        for i, (err, a) in enumerate(scored[:50]):
            f.write(f"### {i+1}. {a.get('domain','')} / {a.get('id','')}\n")
            f.write(f"- Level: {a.get('level_name','')} | n6 score: {a.get('n6',0):.2f}\n")
            f.write(f"- Label: {a.get('label','')}\n")
            f.write(f"- Matches:\n")
            seen = set()
            for num_val, (expr_name, expr_val, rel_err) in a['inverse_matches'][:5]:
                key = f"{num_val}-{expr_name}"
                if key not in seen:
                    seen.add(key)
                    f.write(f"  - {num_val} ~ {expr_name} = {expr_val:.4f} (error: {100*rel_err:.2f}%)\n")
            f.write(f"\n")

        # Genuinely non-n6 (sample)
        f.write(f"\n## Genuinely Non-N6 Candidates (no formula match found)\n\n")
        f.write(f"Total: {len(genuinely_non_n6)} candidates with no plausible n6 connection.\n\n")
        f.write(f"| # | Domain | Level | ID | n6 | Label (truncated) |\n")
        f.write(f"|---|--------|-------|----|----|-------------------|\n")
        for i, a in enumerate(genuinely_non_n6[:40]):
            label = a.get('label', '')[:60]
            f.write(f"| {i+1} | {a.get('domain','')} | {a.get('level_name','')} "
                    f"| {a.get('id','')} | {a.get('n6',0):.2f} | {label} |\n")

        # Recommendations
        f.write(f"\n## Recommendations\n\n")
        f.write(f"### 1. Upgrade Candidates (undiscovered formulas found)\n")
        f.write(f"The {len(undiscovered)} candidates with inverse matches should be reviewed.\n")
        f.write(f"If the n6 expression is genuine, upgrade their n6 score in the TOML file.\n\n")

        f.write(f"### 2. High-Anomaly Domains\n")
        high_anom = [d for d in domain_rates if d[3] > 0.15 and d[1] >= 2]
        if high_anom:
            f.write(f"These domains have >15% anomaly rate and may need TOML revision:\n")
            for dname, anom, total, rate in high_anom[:10]:
                f.write(f"- **{dname}**: {anom}/{total} ({100*rate:.0f}%)\n")
        else:
            f.write(f"No domains exceed 15% anomaly rate with 2+ anomalies.\n")

        f.write(f"\n### 3. Genuinely Non-N6\n")
        f.write(f"The {len(genuinely_non_n6)} candidates with no match are either:\n")
        f.write(f"- Material/physical constants with no n6 connection (expected)\n")
        f.write(f"- Parameters needing deeper (depth-3+) expression search\n")
        f.write(f"- Candidates that should be removed or replaced with n6-aligned alternatives\n\n")

        f.write(f"### 4. Average n6 Score\n")
        avg_n6 = sum(c.get('n6', 0) for c in all_candidates) / len(all_candidates)
        f.write(f"Overall average: {avg_n6:.3f}\n")
        f.write(f"Non-anomaly average: {sum(c.get('n6', 0) for c in all_candidates if c.get('n6', 1.0) >= 0.50) / max(1, len([c for c in all_candidates if c.get('n6', 1.0) >= 0.50])):.3f}\n")

    print(f"\nReport written to: {output_path}")
    print("Done.")


if __name__ == '__main__':
    main()
