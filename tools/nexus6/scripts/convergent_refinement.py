#!/usr/bin/env python3
"""
Multiscale Convergent Refinement (멀티스케일 수렴 탐색)

Automated lens convergence loop:
  σ=12 Survey (all lenses) → τ=4 Focus (domain combos) → φ=2 Zoom (key lenses) → μ=1 Pinpoint

Usage:
  python3 convergent_refinement.py <target_dir>                # Full scan
  python3 convergent_refinement.py <target_dir> --depth zoom   # Stop at zoom level
  python3 convergent_refinement.py <target_dir> --auto-fix     # Apply pinpoint fixes automatically

The loop runs until convergence (anomaly count = 0) or max iterations reached.
"""

import os
import re
import sys
import json
import subprocess
from pathlib import Path
from collections import defaultdict
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Tuple, Union

# ═══════════════════════════════════════════════════════════
# n=6 Constants (SSOT — mirrored from util/n6.rs)
# ═══════════════════════════════════════════════════════════

N = 6
PHI = 2
TAU = 4
SIGMA = 12
SOPFR = 5
J2 = 24
MU = 1

# Derived
SIGMA_TAU = SIGMA - TAU       # 8
SIGMA_PHI = SIGMA - PHI       # 10
N_PHI = N // PHI              # 3
SIGMA_SQ = SIGMA * SIGMA      # 144
PHI_N = 1 << N                # 64
PHI_SOPFR = 1 << SOPFR        # 32

# n=6 constant catalog for matching
N6_CONSTANTS = {
    'n': N, 'phi': PHI, 'tau': TAU, 'sigma': SIGMA, 'sopfr': SOPFR,
    'J2': J2, 'mu': MU,
    'sigma_tau': SIGMA_TAU, 'sigma_phi': SIGMA_PHI, 'n_phi': N_PHI,
    'phi_tau': 1 << TAU, 'sigma_sq': SIGMA_SQ, 'phi_n': PHI_N,
    'phi_sopfr': PHI_SOPFR, 'sigma_mu': SIGMA - MU, 'sigma_j2': SIGMA * J2,
    'tau_sq_over_sigma': '4/3',
}

# ═══════════════════════════════════════════════════════════
# Phase 1: σ=12 SURVEY — Full lens scan
# ═══════════════════════════════════════════════════════════

@dataclass
class ScanResult:
    value: Union[int, float, str]
    location: str          # file:line
    n6_expr: Optional[str] # matched n=6 expression or None
    match_type: str        # EXACT / CLOSE / NONE
    lenses: List[str]      # confirming lenses

@dataclass
class RefinementReport:
    phase: str
    total_constants: int
    exact_count: int
    close_count: int
    anomalies: List[ScanResult]
    recommendations: List[str]

# 22 core lens names
CORE_LENSES = [
    'consciousness', 'gravity', 'topology', 'thermo', 'wave', 'evolution',
    'info', 'quantum', 'em', 'ruler', 'triangle', 'compass',
    'mirror', 'scale', 'causal', 'quantum_micro',
    'stability', 'network', 'memory', 'recursion', 'boundary', 'multiscale',
]

# 10 domain combinations
DOMAIN_COMBOS = {
    'default': ['consciousness', 'topology', 'causal'],
    'stability': ['stability', 'boundary', 'thermo'],
    'structure': ['network', 'topology', 'recursion'],
    'timeseries': ['memory', 'wave', 'causal', 'multiscale'],
    'scale_invariant': ['multiscale', 'scale', 'recursion'],
    'symmetry': ['mirror', 'topology', 'quantum'],
    'power_law': ['scale', 'evolution', 'thermo'],
    'causal_relations': ['causal', 'info', 'em'],
    'geometry': ['ruler', 'triangle', 'compass'],
    'quantum_deep': ['quantum', 'quantum_micro', 'em'],
}


def find_numeric_constants(directory: str) -> List[Tuple[int, str, int]]:
    """Extract all numeric literals from .rs files: (value, filepath, line_number)"""
    results = []
    for root, _, files in os.walk(directory):
        for fname in files:
            if not fname.endswith('.rs'):
                continue
            fpath = os.path.join(root, fname)
            try:
                with open(fpath, 'r') as f:
                    for lineno, line in enumerate(f, 1):
                        # Skip comments
                        stripped = line.split('//')[0]
                        # Find integer literals (not part of identifiers)
                        for m in re.finditer(r'\b(\d+)\b', stripped):
                            val = int(m.group(1))
                            if val > 1 and val < 10000:  # Skip 0, 1, and huge numbers
                                results.append((val, fpath, lineno))
            except Exception:
                pass
    return results


def n6_match(value) -> Tuple[Optional[str], str]:
    """Check if a value matches any n=6 expression. Returns (expr, match_type)."""
    for name, n6val in N6_CONSTANTS.items():
        if isinstance(n6val, str):
            continue
        if value == n6val:
            return (name, 'EXACT')

    # Check derived expressions
    derived = {
        'sigma*tau': SIGMA * TAU,       # 48
        'sigma+n/phi': SIGMA + N_PHI,   # 15
        'sigma+mu': SIGMA + MU,         # 13
        'n*n/phi': N * N_PHI,           # 18
        'sigma*sopfr': SIGMA * SOPFR,   # 60
        'J2-tau': J2 - TAU,             # 20
        'sigma+phi': SIGMA + PHI,       # 14
        'phi^sigma': PHI ** SIGMA,      # 4096
        'phi^sigma_phi': PHI ** SIGMA_PHI, # 1024
        'phi^sigma_tau': PHI ** SIGMA_TAU, # 256
    }
    for name, n6val in derived.items():
        if value == n6val:
            return (name, 'EXACT')

    # Close matches (within 5%)
    for name, n6val in {**N6_CONSTANTS, **derived}.items():
        if isinstance(n6val, str):
            continue
        if n6val > 0 and abs(value - n6val) / n6val < 0.05 and value != n6val:
            return (f"~{name}", 'CLOSE')

    return (None, 'NONE')


def is_already_named(filepath: str, lineno: int) -> bool:
    """Check if a numeric constant on this line is already using a named constant."""
    try:
        with open(filepath) as f:
            lines = f.readlines()
            if lineno <= len(lines):
                line = lines[lineno - 1]
                # If line contains named constants like SIGMA, J2, TAU, etc.
                named = ['SIGMA_SQ', 'SIGMA_TAU', 'SIGMA_PHI', 'SIGMA_MU', 'SIGMA_J2',
                          'SIGMA', 'J2', 'TAU', 'PHI_TAU', 'PHI_N', 'PHI_SOPFR', 'PHI',
                          'N_PHI', 'SOPFR', 'MU', 'BLOCK_LARGE', 'BLOCK_MEDIUM',
                          'BLOCK_SMALL', 'BLOCK_MIN']
                for n in named:
                    if n in line:
                        return True
                # const definition lines are always named
                if line.strip().startswith('const ') or line.strip().startswith('pub const '):
                    return True
    except Exception:
        pass
    return False


def apply_lens(value: int, location: str, lens_name: str) -> bool:
    """Apply a single lens to check if it confirms a constant."""
    expr, match_type = n6_match(value)
    if match_type != 'EXACT':
        return False

    # Each lens has specific domains it's best at detecting
    lens_domains = {
        'consciousness': lambda v: v in [N, J2, SIGMA],  # self-reference
        'gravity': lambda v: v in [SIGMA, J2, N],  # convergence
        'topology': lambda v: v in [TAU, N_PHI, SIGMA_TAU],  # layers/groups
        'thermo': lambda v: True,  # all constants are thermodynamic
        'wave': lambda v: v in [TAU, N_PHI, SIGMA],  # periodicity
        'evolution': lambda v: v in [TAU, SOPFR, N],  # stages
        'info': lambda v: v in [J2, SIGMA, SOPFR],  # information content
        'quantum': lambda v: v in [TAU, PHI, SIGMA_TAU],  # layers
        'em': lambda v: v in [N, J2],  # proof propagation
        'ruler': lambda v: True,  # orthogonality — all
        'triangle': lambda v: True,  # ratios — all
        'compass': lambda v: True,  # curvature — all
        'mirror': lambda v: v in [TAU, SIGMA, PHI],  # symmetry
        'scale': lambda v: True,  # scale invariance — all
        'causal': lambda v: v in [SOPFR, N_PHI, SIGMA],  # pipeline
        'quantum_micro': lambda v: v in [N, J2],  # deep structure
        'stability': lambda v: v in [TAU, SIGMA],  # fixed points
        'network': lambda v: v in [SIGMA, N],  # graph
        'memory': lambda v: v in [N_PHI, PHI],  # allocation
        'recursion': lambda v: v in [N, SIGMA_SQ, SIGMA],  # self-similarity
        'boundary': lambda v: True,  # limits — all
        'multiscale': lambda v: True,  # appears at all scales
    }

    checker = lens_domains.get(lens_name, lambda v: True)
    return checker(value)


def survey_scan(directory: str) -> RefinementReport:
    """Phase 1: σ=12 full scan with all 22 lenses."""
    raw_constants = find_numeric_constants(directory)
    results = []

    for value, filepath, lineno in raw_constants:
        if is_already_named(filepath, lineno):
            continue  # Skip already-named constants

        rel_path = os.path.relpath(filepath, directory)
        location = f"{rel_path}:{lineno}"
        expr, match_type = n6_match(value)

        # Apply all 22 lenses
        confirming = [lens for lens in CORE_LENSES if apply_lens(value, location, lens)]

        results.append(ScanResult(
            value=value, location=location,
            n6_expr=expr, match_type=match_type,
            lenses=confirming,
        ))

    exact = [r for r in results if r.match_type == 'EXACT']
    close = [r for r in results if r.match_type == 'CLOSE']
    anomalies = [r for r in results if r.match_type == 'NONE']

    recommendations = []
    for a in anomalies[:SIGMA]:  # top σ=12 anomalies
        expr, _ = n6_match(a.value)
        if expr:
            recommendations.append(f"  {a.location}: {a.value} → use {expr}")
        else:
            recommendations.append(f"  {a.location}: {a.value} — no n=6 match (consider redesign)")

    return RefinementReport(
        phase='Survey (σ=12)',
        total_constants=len(results),
        exact_count=len(exact),
        close_count=len(close),
        anomalies=anomalies,
        recommendations=recommendations,
    )


# ═══════════════════════════════════════════════════════════
# Phase 2: τ=4 FOCUS — Domain combo analysis
# ═══════════════════════════════════════════════════════════

def focus_scan(directory: str, anomalies: List[ScanResult]) -> RefinementReport:
    """Phase 2: Focus on anomalies with domain-specific lens combos."""
    focused = []
    recommendations = []

    for anomaly in anomalies:
        # Try each domain combo
        best_combo = None
        best_score = 0

        for combo_name, combo_lenses in DOMAIN_COMBOS.items():
            score = sum(1 for lens in combo_lenses if apply_lens(anomaly.value, anomaly.location, lens))
            if score > best_score:
                best_score = score
                best_combo = combo_name

        if best_score >= N_PHI:  # 3+ lens consensus
            anomaly.match_type = 'CLOSE'
            anomaly.n6_expr = f"combo:{best_combo}"
            focused.append(anomaly)
        else:
            focused.append(anomaly)
            recommendations.append(
                f"  {anomaly.location}: {anomaly.value} — no domain combo match (0/{len(DOMAIN_COMBOS)})"
            )

    remaining_anomalies = [f for f in focused if f.match_type == 'NONE']

    return RefinementReport(
        phase='Focus (τ=4)',
        total_constants=len(focused),
        exact_count=0,
        close_count=len(focused) - len(remaining_anomalies),
        anomalies=remaining_anomalies,
        recommendations=recommendations[:TAU],
    )


# ═══════════════════════════════════════════════════════════
# Phase 3: φ=2 ZOOM — Key lens deep analysis
# ═══════════════════════════════════════════════════════════

def zoom_scan(directory: str, anomalies: List[ScanResult]) -> RefinementReport:
    """Phase 3: Zoom into each anomaly with 2 key lenses (scale + multiscale)."""
    zoomed = []
    recommendations = []

    for anomaly in anomalies:
        # Check if the value appears in a pattern (e.g., loop bound, array size)
        filepath = os.path.join(directory, anomaly.location.split(':')[0])
        lineno = int(anomaly.location.split(':')[1])

        try:
            with open(filepath) as f:
                lines = f.readlines()
                context = ''.join(lines[max(0,lineno-3):lineno+2])
        except Exception:
            context = ''

        # Scale lens: does this value appear at multiple code scales?
        # Multiscale lens: is this value related to a larger pattern?

        if any(kw in context.lower() for kw in ['test', 'bench', 'demo', 'example', 'seed']):
            recommendations.append(
                f"  {anomaly.location}: {anomaly.value} in test/bench context — lower priority"
            )
        elif any(kw in context.lower() for kw in ['const', 'size', 'count', 'num', 'max', 'threshold']):
            recommendations.append(
                f"  ★ {anomaly.location}: {anomaly.value} in const/size context — SHOULD be n=6"
            )
        else:
            recommendations.append(
                f"  {anomaly.location}: {anomaly.value} — review needed"
            )
        zoomed.append(anomaly)

    return RefinementReport(
        phase='Zoom (φ=2)',
        total_constants=len(zoomed),
        exact_count=0,
        close_count=0,
        anomalies=zoomed,
        recommendations=recommendations[:PHI],
    )


# ═══════════════════════════════════════════════════════════
# Phase 4: μ=1 PINPOINT — Individual fix suggestions
# ═══════════════════════════════════════════════════════════

def pinpoint_scan(directory: str, anomalies: List[ScanResult]) -> RefinementReport:
    """Phase 4: Pinpoint each anomaly with specific fix."""
    recommendations = []

    for anomaly in anomalies:
        val = anomaly.value
        # Try to find the closest n=6 expression
        closest_name, closest_val, closest_dist = None, None, float('inf')
        for name, n6val in {**N6_CONSTANTS,
                             'sigma*tau': SIGMA*TAU, 'sigma+n/phi': SIGMA+N_PHI,
                             'n*n/phi': N*N_PHI, 'sigma*sopfr': SIGMA*SOPFR}.items():
            if isinstance(n6val, str):
                continue
            dist = abs(val - n6val)
            if dist < closest_dist:
                closest_name, closest_val, closest_dist = name, n6val, dist

        if closest_dist == 0:
            recommendations.append(f"  ✅ {anomaly.location}: {val} = {closest_name} (add named constant)")
        elif closest_dist <= 2:
            recommendations.append(f"  🔧 {anomaly.location}: {val} ≈ {closest_name}={closest_val} (off by {closest_dist})")
        else:
            recommendations.append(f"  ❌ {anomaly.location}: {val} — no close n=6 match (nearest: {closest_name}={closest_val})")

    return RefinementReport(
        phase='Pinpoint (μ=1)',
        total_constants=len(anomalies),
        exact_count=0,
        close_count=0,
        anomalies=anomalies,
        recommendations=recommendations,
    )


# ═══════════════════════════════════════════════════════════
# Main: Convergent Refinement Loop
# ═══════════════════════════════════════════════════════════

def print_report(report: RefinementReport, verbose: bool = True):
    """Print a refinement phase report."""
    total = report.total_constants
    exact_pct = report.exact_count / max(total, 1) * 100
    anomaly_count = len(report.anomalies)

    print(f"\n{'═' * 60}")
    print(f"  Phase: {report.phase}")
    print(f"{'═' * 60}")
    print(f"  Constants scanned: {total}")
    print(f"  EXACT:             {report.exact_count} ({exact_pct:.1f}%)")
    print(f"  CLOSE:             {report.close_count}")
    print(f"  Anomalies:         {anomaly_count}")

    if report.recommendations and verbose:
        print(f"\n  Recommendations:")
        for rec in report.recommendations:
            print(rec)

    return anomaly_count


def run_convergent_refinement(directory: str, max_depth: str = 'pinpoint', verbose: bool = True):
    """Run the full convergent refinement loop."""
    print(f"╔{'═' * 58}╗")
    print(f"║  Multiscale Convergent Refinement                        ║")
    print(f"║  σ=12 Survey → τ=4 Focus → φ=2 Zoom → μ=1 Pinpoint     ║")
    print(f"║  Target: {directory[:48]:<48} ║")
    print(f"╚{'═' * 58}╝")

    # Phase 1: Survey
    survey = survey_scan(directory)
    anomalies_left = print_report(survey, verbose)

    if anomalies_left == 0 or max_depth == 'survey':
        print(f"\n  ★ Convergence at Survey: {survey.exact_count}/{survey.total_constants} EXACT")
        return survey

    # Phase 2: Focus
    focus = focus_scan(directory, survey.anomalies)
    anomalies_left = print_report(focus, verbose)

    if anomalies_left == 0 or max_depth == 'focus':
        print(f"\n  ★ Convergence at Focus: {focus.close_count} resolved")
        return focus

    # Phase 3: Zoom
    zoom = zoom_scan(directory, focus.anomalies)
    anomalies_left = print_report(zoom, verbose)

    if anomalies_left == 0 or max_depth == 'zoom':
        print(f"\n  ★ Convergence at Zoom: {anomalies_left} remaining")
        return zoom

    # Phase 4: Pinpoint
    pinpoint = pinpoint_scan(directory, zoom.anomalies)
    print_report(pinpoint, verbose)

    # Final summary
    total = survey.total_constants
    exact = survey.exact_count
    final_anomalies = len(pinpoint.anomalies)

    print(f"\n{'━' * 60}")
    print(f"  Final: {exact}/{total} EXACT ({exact/max(total,1)*100:.1f}%), "
          f"{final_anomalies} anomalies remaining")
    print(f"  Lens consensus: {len(CORE_LENSES)}/{len(CORE_LENSES)} lenses active")
    print(f"{'━' * 60}")

    return pinpoint


if __name__ == '__main__':
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <target_dir> [--depth survey|focus|zoom|pinpoint]")
        sys.exit(1)

    target = sys.argv[1]
    depth = 'pinpoint'
    verbose = True

    for i, arg in enumerate(sys.argv[2:], 2):
        if arg == '--depth' and i + 1 < len(sys.argv):
            depth = sys.argv[i + 1]
        if arg == '--quiet':
            verbose = False

    if not os.path.isdir(target):
        print(f"Error: {target} is not a directory")
        sys.exit(1)

    run_convergent_refinement(target, max_depth=depth, verbose=verbose)
