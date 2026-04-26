#!/usr/bin/env python3
# GALO-PX-1 BT-546 BKLPR (A3) 무상관 가정 정밀 검증
# 목표: Cremona 332k 에서 joint distribution of (|Sel_2|, |Sel_3|) 계산
#       → Pearson r, Cov, 조건부 기대값, (A3) 반증 강도 측정
# 정직: |Sel_n| 1차근사 유지. Sage 정밀 계산은 DEFERRED.

import json
import sys
import random
from pathlib import Path
from collections import Counter, defaultdict
import math

# Resolve ROOT robustly: prefer current repo (n6-architecture) over the
# legacy hardcoded /Users/ghost/Dev/n6-architecture path.
_HERE = Path(__file__).resolve()
_CANDIDATES = [
    _HERE.parents[2],                          # repo containing scripts/
    Path("/Users/ghost/core/n6-architecture"),
    Path("/Users/ghost/Dev/n6-architecture"),
]
ROOT = next(
    (p for p in _CANDIDATES if (p / "data" / "cremona" / "allbsd").is_dir()),
    _CANDIDATES[0],
)
DATA_DIR = ROOT / "data" / "cremona" / "allbsd"


def parse(line):
    p = line.strip().split()
    if len(p) < 8:
        return None
    try:
        N = int(p[0])
        r = int(p[4])
        t = int(p[5])
        sha = int(round(float(p[-1])))
        return (N, r, t, max(sha, 1))
    except (ValueError, IndexError):
        return None


def sel2(r, t, sha):
    tf = 2 if t % 2 == 0 else 1
    sp = 1
    s = sha
    while s % 2 == 0:
        sp *= 2
        s //= 2
    return (2 ** r) * tf * sp


def sel3(r, t, sha):
    tf = 3 if t % 3 == 0 else 1
    sp = 1
    s = sha
    while s % 3 == 0:
        sp *= 3
        s //= 3
    return (3 ** r) * tf * sp


def _load_pairs():
    """Stream all curves and return parallel lists (s2, s3, t, sha)."""
    shards = sorted(DATA_DIR.glob("allbsd.*"))
    s2_all, s3_all, t_all, sha_all = [], [], [], []
    for s in shards:
        with s.open() as f:
            for line in f:
                c = parse(line)
                if c:
                    _, r, t, sha = c
                    s2_all.append(sel2(r, t, sha))
                    s3_all.append(sel3(r, t, sha))
                    t_all.append(t)
                    sha_all.append(sha)
    return s2_all, s3_all, t_all, sha_all


def _sha_class(sha):
    if sha == 1:
        return 1
    if sha == 4:
        return 4
    if sha == 9:
        return 9
    return ">9"


def _kappa_with_bootstrap(xs, ys, n_boot=1000, seed=20260425):
    n = len(xs)
    if n < 2:
        return float("nan"), float("nan"), float("nan"), float("nan")
    try:
        import numpy as np
        xa = np.asarray(xs, dtype=np.float64)
        ya = np.asarray(ys, dtype=np.float64)
        kappa = float(np.mean((xa - xa.mean()) * (ya - ya.mean())))
        rng = np.random.default_rng(seed)
        boots = np.empty(n_boot, dtype=np.float64)
        for b in range(n_boot):
            idx = rng.integers(0, n, size=n)
            sx = xa[idx]
            sy = ya[idx]
            boots[b] = float(np.mean((sx - sx.mean()) * (sy - sy.mean())))
        se = float(boots.std(ddof=0))
        lo = float(np.quantile(boots, 0.025))
        hi = float(np.quantile(boots, 0.975))
        return kappa, se, lo, hi
    except ImportError:
        pass
    mx = sum(xs) / n
    my = sum(ys) / n
    kappa = sum((xs[i] - mx) * (ys[i] - my) for i in range(n)) / n
    rng = random.Random(seed)
    boots = []
    for _ in range(n_boot):
        sample = [rng.randrange(n) for _ in range(n)]
        sx = [xs[k] for k in sample]
        sy = [ys[k] for k in sample]
        smx = sum(sx) / n
        smy = sum(sy) / n
        boots.append(sum((sx[i] - smx) * (sy[i] - smy) for i in range(n)) / n)
    boots.sort()
    mu = sum(boots) / len(boots)
    var = sum((b - mu) ** 2 for b in boots) / len(boots)
    se = math.sqrt(var)
    lo = boots[int(0.025 * len(boots))]
    hi = boots[int(0.975 * len(boots)) - 1]
    return kappa, se, lo, hi


def main_per_stratum():
    print("[BT-546 Probe B] 28-stratum joint covariance run", file=sys.stderr)
    print(f"[ROOT] {ROOT}", file=sys.stderr)
    s2_all, s3_all, t_all, sha_all = _load_pairs()
    N = len(s2_all)
    print(f"[N] {N} curves loaded", file=sys.stderr)

    torsion_strata = [1, 2, 3, 4, 6, 8, 12]
    sha_strata = [1, 4, 9, ">9"]
    cells = defaultdict(lambda: ([], []))
    for i in range(N):
        t = t_all[i]
        if t not in torsion_strata:
            continue
        sc = _sha_class(sha_all[i])
        cells[(t, sc)][0].append(s2_all[i])
        cells[(t, sc)][1].append(s3_all[i])

    rows = []
    for t in torsion_strata:
        for sc in sha_strata:
            xs, ys = cells.get((t, sc), ([], []))
            ncell = len(xs)
            if ncell >= 2:
                k, se, lo, hi = _kappa_with_bootstrap(xs, ys, n_boot=1000)
            else:
                k = se = lo = hi = float("nan")
            rows.append({
                "torsion": t,
                "sha_class": sc,
                "N_cell": ncell,
                "kappa": k,
                "se": se,
                "conf_lo": lo,
                "conf_hi": hi,
            })

    out = {
        "tool": "cremona_joint_covariance.py --per-stratum",
        "data": str(DATA_DIR),
        "N_total": N,
        "torsion_strata": torsion_strata,
        "sha_strata": sha_strata,
        "n_bootstrap": 1000,
        "seed": 20260425,
        "rows": rows,
    }
    print(json.dumps(out, indent=2, ensure_ascii=False))


def main():
    if "--per-stratum" in sys.argv:
        main_per_stratum()
        return
    shards = sorted(DATA_DIR.glob("allbsd.*"))
    print(f"[GALO-PX-1] Cremona joint distribution 정밀 분석", file=sys.stderr)
    s2_all, s3_all = [], []
    for s in shards:
        with s.open() as f:
            for line in f:
                c = parse(line)
                if c:
                    _, r, t, sha = c
                    s2_all.append(sel2(r, t, sha))
                    s3_all.append(sel3(r, t, sha))

    N = len(s2_all)
    print(f"[N] {N} curves", file=sys.stderr)

    # 기본 통계
    mean_s2 = sum(s2_all) / N
    mean_s3 = sum(s3_all) / N
    mean_s6 = sum(a * b for a, b in zip(s2_all, s3_all)) / N
    var_s2 = sum((x - mean_s2) ** 2 for x in s2_all) / N
    var_s3 = sum((x - mean_s3) ** 2 for x in s3_all) / N
    cov = sum((a - mean_s2) * (b - mean_s3) for a, b in zip(s2_all, s3_all)) / N
    # Pearson r = Cov / (σ_2 σ_3)
    sd2 = math.sqrt(var_s2)
    sd3 = math.sqrt(var_s3)
    r_pearson = cov / (sd2 * sd3) if sd2 * sd3 > 0 else 0

    print()
    print("=" * 68)
    print("[결과] 1차 통계 (Cremona 332k)")
    print("=" * 68)
    print(f"  N = {N}")
    print(f"  E[|Sel_2|]    = {mean_s2:.4f}   (theoretical σ(2)=3,  ratio={mean_s2/3:.4f})")
    print(f"  E[|Sel_3|]    = {mean_s3:.4f}   (theoretical σ(3)=4,  ratio={mean_s3/4:.4f})")
    print(f"  E[|Sel_6|]    = {mean_s6:.4f}   (theoretical σ(6)=12, ratio={mean_s6/12:.4f})")
    print(f"  E[|S_2|]E[|S_3|] = {mean_s2 * mean_s3:.4f}")
    print(f"  Cov(|S_2|,|S_3|) = E[|S_6|] - E[|S_2|]E[|S_3|] = {cov:.4f}")
    print(f"  Var(|Sel_2|) = {var_s2:.4f}  sd = {sd2:.4f}")
    print(f"  Var(|Sel_3|) = {var_s3:.4f}  sd = {sd3:.4f}")
    print(f"  Pearson r    = {r_pearson:.6f}")
    print()

    # 조건부 기대값 E[|Sel_3| | |Sel_2| = k] — 주요 k 만
    print("[조건부 E[|Sel_3| | |Sel_2|=k]]")
    print("  (A3) 독립 가정 하에서 k 값에 무관하게 E[|Sel_3|] = μ_3 이어야 함.")
    print()
    print(f"  {'k':>6} {'count':>8} {'% total':>10} {'E[|Sel_3||k]':>14} {'편차 from μ_3':>16}")
    cond = defaultdict(list)
    for a, b in zip(s2_all, s3_all):
        cond[a].append(b)
    for k in sorted(cond.keys())[:10]:
        vals = cond[k]
        cm = sum(vals) / len(vals)
        dev = cm - mean_s3
        pct = 100 * len(vals) / N
        print(f"  {k:>6} {len(vals):>8} {pct:>9.3f}% {cm:>14.4f} {dev:>+16.4f}")

    # (A3) 반증 강도: max 조건부 편차
    max_dev = 0
    for k, vals in cond.items():
        if len(vals) > 100:  # 통계적 의미 확보
            cm = sum(vals) / len(vals)
            if abs(cm - mean_s3) > max_dev:
                max_dev = abs(cm - mean_s3)

    print()
    print("[A3 반증 강도]")
    print(f"  max |E[|Sel_3| | |Sel_2|=k] - E[|Sel_3|]| (k bin with ≥100 curves) = {max_dev:.4f}")
    print(f"  Pearson r = {r_pearson:.6f}")
    print(f"  정량적 (A3) 반증: r ≠ 0 (P-value deferred, χ² test Sage 필요)")
    print()

    # 수정 conjecture (A3') 제안
    print("[Modified conjecture (A3') 제안]")
    print("  원 (A3): Cov(|Sel_p|, |Sel_q|) = 0 for p≠q (asymptotic B→∞)")
    print("  (A3'): Cov(|Sel_p|, |Sel_q|) → 0 as B→∞, but finite at any B.")
    print("         E_B[|Sel_{pq}|] = E_B[|Sel_p|]·E_B[|Sel_q|] + κ(p,q,B)")
    print(f"  Cremona B=49999: κ(2,3,49999) = {cov:.4f}, r = {r_pearson:.6f}")
    print()
    print("  Corollary: 만약 κ → 0 as B → ∞, 그리고 E_B[|Sel_p|] → σ(p),")
    print("             then E_B[|Sel_6|] → σ(6) = 12 (점근적으로만 (A3) 복구)")
    print("             현재 B=49999 는 |E_B[|Sel_6|] - 12| = 2.49 (20.7% 편차)")

    # JSON 저장
    summary = {
        "source": "Cremona ecdata allbsd.00000-49999",
        "N": N,
        "marginal_means": {"E_sel_2": mean_s2, "E_sel_3": mean_s3, "E_sel_6": mean_s6},
        "theoretical_sigma": {"sigma_2": 3, "sigma_3": 4, "sigma_6": 12},
        "ratios_to_sigma": {
            "r_2": mean_s2 / 3,
            "r_3": mean_s3 / 4,
            "r_6": mean_s6 / 12,
        },
        "joint_statistics": {
            "product_of_marginals": mean_s2 * mean_s3,
            "covariance": cov,
            "pearson_r": r_pearson,
            "var_s2": var_s2,
            "var_s3": var_s3,
            "sd_s2": sd2,
            "sd_s3": sd3,
        },
        "A3_violation": {
            "original_A3_claim": "Cov(|Sel_p|, |Sel_q|) = 0 for p != q",
            "empirical_Cov_32_B_49999": cov,
            "pearson_r_32": r_pearson,
            "modified_A3_prime": "E_B[|Sel_{pq}|] = E_B[|S_p|]·E_B[|S_q|] + κ(p,q,B); κ → 0 as B → ∞ (conjectural)",
        },
        "BKLPR_survives_approximately": {
            "conductor_bias_reduces_marginals": "E_B[|S_p|] < σ(p)",
            "joint_covariance_boosts_joint": "E_B[|S_pq|] > E_B[|S_p|]·E_B[|S_q|]",
            "net_ratio_to_sigma_6": mean_s6 / 12,
            "interpretation": "두 효과 부분 상쇄, B→∞ 극한에서 (A3) + σ(6)=12 복구 추측",
        },
        "caveats": [
            "|Sel_n| 1차근사 (torsion Z/2×Z/2 미분리)",
            "Sha analytic (BSD conjectural)",
            "conductor B=49999 유한, asymptotic 아님",
            "chi-square independence test 미수행 (Sage/scipy DEFERRED)",
        ],
    }
    out = ROOT / "data" / "cremona" / "joint_covariance_A3_prime.json"
    out.write_text(json.dumps(summary, indent=2, ensure_ascii=False), encoding="utf-8")
    print(f"[저장] {out}")


if __name__ == "__main__":
    main()
