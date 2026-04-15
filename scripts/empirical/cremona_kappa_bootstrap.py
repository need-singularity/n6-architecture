#!/usr/bin/env python3
# v3 loop 19: κ(B) bootstrap uncertainty 정량화
#
# v3 E5 에서 κ(B) ~ A·B^α, α=0.1752 측정
# 7 bin 에 대한 log-linear fit — 표준오차 미계산
# 본 스크립트: bootstrap resample 로 α 표준오차 + log(2)/4 match 신뢰구간

import json
import math
import random
from pathlib import Path

ROOT = Path("/Users/ghost/Dev/n6-architecture")
DATA = ROOT / "data/cremona/kappa_10bin_results.json"
REPORT = ROOT / "reports/v3/kappa_bootstrap_2026-04-16.json"


def linear_fit(xs: list[float], ys: list[float]) -> tuple[float, float]:
    """least squares y = a + b·x → (a, b)"""
    n = len(xs)
    xbar = sum(xs) / n
    ybar = sum(ys) / n
    num = sum((x - xbar) * (y - ybar) for x, y in zip(xs, ys))
    den = sum((x - xbar) ** 2 for x in xs)
    b = num / den
    a = ybar - b * xbar
    return a, b


def bootstrap_alpha(bins_data: list[dict], n_trials: int = 10000, seed: int = 42) -> dict:
    """7-bin log-log fit 에 대한 bootstrap (sample w/ replacement, 7 개씩)"""
    rng = random.Random(seed)
    alphas: list[float] = []
    intercepts: list[float] = []

    # 원 데이터: (log B, log κ) — κ > 0 인 bin 만 (κ<0 은 2 bins, 초기)
    valid_bins = [b for b in bins_data if b["kappa"] > 0]
    xs_all = [math.log(b["B_mid"]) for b in valid_bins]
    ys_all = [math.log(b["kappa"]) for b in valid_bins]

    n = len(xs_all)
    print(f"[bootstrap] {n} valid bins (κ>0), {n_trials} trials, seed={seed}")

    for _ in range(n_trials):
        idxs = [rng.randrange(n) for _ in range(n)]
        xs_b = [xs_all[i] for i in idxs]
        ys_b = [ys_all[i] for i in idxs]
        try:
            a_b, b_b = linear_fit(xs_b, ys_b)
            alphas.append(b_b)
            intercepts.append(a_b)
        except ZeroDivisionError:
            continue

    alphas.sort()
    intercepts.sort()

    def pct(vals: list[float], p: float) -> float:
        k = int(round(p / 100 * (len(vals) - 1)))
        return vals[k]

    alpha_mean = sum(alphas) / len(alphas)
    alpha_var = sum((x - alpha_mean) ** 2 for x in alphas) / len(alphas)
    alpha_std = math.sqrt(alpha_var)

    return {
        "n_trials": n_trials,
        "n_bins_resampled": n,
        "seed": seed,
        "alpha_mean": alpha_mean,
        "alpha_std": alpha_std,
        "alpha_ci95": [pct(alphas, 2.5), pct(alphas, 97.5)],
        "alpha_ci68": [pct(alphas, 16), pct(alphas, 84)],
        "alpha_median": pct(alphas, 50),
        "intercept_mean": sum(intercepts) / len(intercepts),
    }


def compare_against_log2_quarter(result: dict) -> dict:
    """α=log(2)/4 에 대한 매칭 평가"""
    target = math.log(2) / 4  # 0.17329
    alpha_mean = result["alpha_mean"]
    alpha_std = result["alpha_std"]
    ci95 = result["alpha_ci95"]
    ci68 = result["alpha_ci68"]

    z = (alpha_mean - target) / alpha_std if alpha_std > 0 else float("inf")
    in_ci68 = ci68[0] <= target <= ci68[1]
    in_ci95 = ci95[0] <= target <= ci95[1]

    return {
        "target_value": target,
        "target_name": "log(2)/4",
        "alpha_mean_observed": alpha_mean,
        "alpha_std": alpha_std,
        "z_score": z,
        "rel_err_pct": 100 * (alpha_mean - target) / target,
        "target_in_68ci": in_ci68,
        "target_in_95ci": in_ci95,
        "verdict": (
            "CONSISTENT (in 68% CI)"
            if in_ci68
            else (
                "COMPATIBLE (in 95% CI)"
                if in_ci95
                else "TENSION (outside 95% CI)"
            )
        ),
    }


def main() -> None:
    data = json.loads(DATA.read_text())
    print(f"[kappa_bootstrap] source: {DATA}")
    print(f"  원 slope (v3 E5): α = {data['power_law_fit']['slope']:.4f}")
    print()

    result = bootstrap_alpha(data["bins"], n_trials=10000)
    match = compare_against_log2_quarter(result)

    print("=== bootstrap α 분포 ===")
    print(f"  평균:    α = {result['alpha_mean']:.4f}")
    print(f"  중앙값:  α = {result['alpha_median']:.4f}")
    print(f"  표준편차 σ = {result['alpha_std']:.4f}")
    print(f"  68% CI: [{result['alpha_ci68'][0]:.4f}, {result['alpha_ci68'][1]:.4f}]")
    print(f"  95% CI: [{result['alpha_ci95'][0]:.4f}, {result['alpha_ci95'][1]:.4f}]")
    print()

    print("=== log(2)/4 = 0.1733 매칭 ===")
    print(f"  z-score:    {match['z_score']:+.3f}")
    print(f"  rel err:    {match['rel_err_pct']:+.2f}%")
    print(f"  in 68% CI:  {'✓' if match['target_in_68ci'] else '✗'}")
    print(f"  in 95% CI:  {'✓' if match['target_in_95ci'] else '✗'}")
    print(f"  평가:       {match['verdict']}")
    print()

    combined = {
        "source": str(DATA.relative_to(ROOT)),
        "bootstrap": result,
        "match_log2_quarter": match,
        "honest_note": (
            "5 valid bins (κ>0), 10000 trials — empirical uncertainty only. "
            "log(2)/4 match 는 여전히 suggestive 이며 증명 아님. "
            "표본 bin 수 5 로 인한 큰 CI 는 정직하게 보고."
        ),
        "date": "2026-04-16",
        "task": "v3 loop 19 empirical robustness",
    }
    REPORT.parent.mkdir(parents=True, exist_ok=True)
    REPORT.write_text(json.dumps(combined, indent=2, ensure_ascii=False))
    print(f"[save] {REPORT}")


if __name__ == "__main__":
    main()
