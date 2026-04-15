#!/usr/bin/env python3
# v4 E4_v4: κ(B) 13-bin refinement on 27 shard Cremona data
# v3 E5 (7 bin) → v4 E4 (13 bin) 로 bin width 축소 — α uncertainty 재측정

from __future__ import annotations
import json
import math
import re
from pathlib import Path
from typing import Optional

ROOT = Path("/Users/ghost/Dev/n6-architecture")
ALLBSD_DIR = ROOT / "data/cremona/allbsd"
OUT_REPORT = ROOT / "reports/v4/kappa_13bin_2026-04-16.json"


def parse_allbsd_line(line: str) -> Optional[tuple]:
    """allbsd shard line → (conductor, analytic_rank, sha) or None
    Format: N class curve_num [a_invs] rank torsion gens regulator L1' sha tamagawa
    """
    parts = line.strip().split()
    if len(parts) < 11:
        return None
    try:
        N = int(parts[0])
        rank = int(parts[4])
        sha = int(round(float(parts[9])))
        return (N, rank, sha)
    except (ValueError, IndexError):
        return None


def sel_n_estimate(n: int, rank: int, sha: int) -> int:
    """approximate |Sel_n| ≈ n^rank × |Sha[n]|. |Sha[n]| ≈ gcd(sha, n^∞)."""
    sha_n_part = 1
    s = sha
    for p in [2, 3, 5, 7, 11, 13, 17]:
        if n % p == 0:
            while s % p == 0:
                sha_n_part *= p
                s //= p
    return (n ** rank) * sha_n_part


def analyze_shard(path: Path) -> list[dict]:
    out = []
    for line in path.read_text().splitlines():
        parsed = parse_allbsd_line(line)
        if parsed is None:
            continue
        N, rank, sha = parsed
        s2 = sel_n_estimate(2, rank, sha)
        s3 = sel_n_estimate(3, rank, sha)
        s6 = sel_n_estimate(6, rank, sha)
        out.append({"N": N, "rank": rank, "sha": sha, "s2": s2, "s3": s3, "s6": s6})
    return out


def bin_analysis(all_curves: list[dict], bin_edges: list[int]) -> list[dict]:
    """conductor bin edges 에 따라 per-bin E[|Sel_2|], E[|Sel_3|], E[|Sel_6|] 계산"""
    bins = []
    for i in range(len(bin_edges) - 1):
        lo, hi = bin_edges[i], bin_edges[i + 1]
        curves = [c for c in all_curves if lo <= c["N"] < hi]
        N = len(curves)
        if N == 0:
            continue
        E_s2 = sum(c["s2"] for c in curves) / N
        E_s3 = sum(c["s3"] for c in curves) / N
        E_s6 = sum(c["s6"] for c in curves) / N
        kappa = E_s6 - E_s2 * E_s3
        ratio6 = E_s6 / (E_s2 * E_s3) if E_s2 * E_s3 > 0 else 0
        bins.append({
            "bin_lo": lo,
            "bin_hi": hi,
            "B_mid": (lo + hi) // 2,
            "N": N,
            "E_s2": E_s2,
            "E_s3": E_s3,
            "E_s6": E_s6,
            "kappa": kappa,
            "ratio6": ratio6,
        })
    return bins


def power_law_fit(bins: list[dict]) -> dict:
    """log-log linear regression on kappa(B) — κ > 0 bins only"""
    valid = [b for b in bins if b["kappa"] > 0]
    if len(valid) < 2:
        return {"error": "insufficient valid bins"}
    xs = [math.log(b["B_mid"]) for b in valid]
    ys = [math.log(b["kappa"]) for b in valid]
    n = len(xs)
    xbar = sum(xs) / n
    ybar = sum(ys) / n
    num = sum((x - xbar) * (y - ybar) for x, y in zip(xs, ys))
    den = sum((x - xbar) ** 2 for x in xs)
    slope = num / den
    intercept = ybar - slope * xbar
    return {
        "slope": slope,
        "intercept": intercept,
        "A": math.exp(intercept),
        "n_valid_bins": n,
        "log2_over_4": math.log(2) / 4,
        "alpha_minus_log2_over_4": slope - math.log(2) / 4,
        "rel_err_pct": 100 * (slope - math.log(2) / 4) / (math.log(2) / 4),
    }


def main():
    shards = sorted(ALLBSD_DIR.glob("allbsd.*"))
    all_curves: list[dict] = []
    for shard in shards:
        all_curves.extend(analyze_shard(shard))
    print(f"[E4_v4] 총 {len(all_curves)} curves from {len(shards)} shards")

    # bin edges: 13 bins, 0-410k, bin width ~32k
    # 7-bin (v3 E5) 대비 약 1.85x fine
    bin_edges = [0, 30000, 60000, 90000, 120000, 150000, 180000, 210000, 240000, 270000, 300000, 330000, 360000, 410000]
    print(f"[E4_v4] bin edges: {bin_edges}")

    bins = bin_analysis(all_curves, bin_edges)
    print(f"[E4_v4] {len(bins)} valid bins")
    print()

    for b in bins:
        print(f"  B=[{b['bin_lo']:>6}, {b['bin_hi']:>6}]  N={b['N']:>6}  "
              f"E[S2]={b['E_s2']:.3f}  E[S3]={b['E_s3']:.3f}  E[S6]={b['E_s6']:.3f}  "
              f"κ={b['kappa']:+.3f}  ratio6={b['ratio6']:.4f}")

    fit = power_law_fit(bins)
    print()
    print(f"=== Power law fit (κ > 0 valid bins = {fit['n_valid_bins']}) ===")
    print(f"  κ(B) ≈ {fit['A']:.4f} · B^{fit['slope']:.4f}")
    print(f"  log(2)/4 = {fit['log2_over_4']:.4f}")
    print(f"  α - log(2)/4 = {fit['alpha_minus_log2_over_4']:+.4f}  ({fit['rel_err_pct']:+.2f}%)")

    result = {
        "source": "27 Cremona shards (conductor 0-410k)",
        "total_curves": len(all_curves),
        "n_bins": len(bins),
        "bins": bins,
        "power_law_fit": fit,
        "comparison_v3_E5": {
            "v3_alpha": 0.1752,
            "v4_alpha": fit["slope"],
            "diff": fit["slope"] - 0.1752,
            "v3_bins": 7,
            "v4_bins": len(bins),
        },
    }
    OUT_REPORT.parent.mkdir(parents=True, exist_ok=True)
    OUT_REPORT.write_text(json.dumps(result, indent=2, ensure_ascii=False))
    print(f"\n[save] {OUT_REPORT}")


if __name__ == "__main__":
    main()
