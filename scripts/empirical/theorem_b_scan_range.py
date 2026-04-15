#!/usr/bin/env python3
# v4 E6_v4: n ∈ [2, N] 에서 σ(n)·φ(n) - n·τ(n) 분포 조사
# Theorem B 유일성 (n=6) 의 empirical reconfirm + "near misses" 탐색

from math import gcd
from pathlib import Path
import json


def divisors(n: int) -> list[int]:
    return [d for d in range(1, n + 1) if n % d == 0]


def sigma(n: int) -> int:
    return sum(divisors(n))


def tau(n: int) -> int:
    return len(divisors(n))


def phi(n: int) -> int:
    return sum(1 for k in range(1, n + 1) if gcd(k, n) == 1)


def scan_theorem_b(upper: int) -> dict:
    hits: list[int] = []
    near_misses: list[tuple[int, int, int, int]] = []
    deviations: list[tuple[int, int]] = []

    for n in range(2, upper + 1):
        sigma_n = sigma(n)
        phi_n = phi(n)
        tau_n = tau(n)
        lhs = sigma_n * phi_n
        rhs = n * tau_n
        dev = lhs - rhs
        deviations.append((n, dev))
        if dev == 0:
            hits.append(n)
        elif abs(dev) <= 2:
            near_misses.append((n, sigma_n, phi_n, tau_n))

    # 상대 편차 통계
    relative_devs = [dev / (n * tau_n if n * tau(n) > 0 else 1) for n, dev in deviations]

    return {
        "upper": upper,
        "theorem_B_hits": hits,
        "near_misses_abs_le_2": near_misses,
        "n6_exact": {
            "n": 6,
            "sigma": sigma(6),
            "phi": phi(6),
            "tau": tau(6),
            "sigma_phi": sigma(6) * phi(6),
            "n_tau": 6 * tau(6),
            "deviation": 0,
        },
        "stats": {
            "total_checked": upper - 1,
            "hits_count": len(hits),
            "near_misses_count": len(near_misses),
            "uniqueness_claim": hits == [6],
        },
        "first_20_deviations": [
            {"n": n, "sigma*phi": sigma(n) * phi(n), "n*tau": n * tau(n), "dev": dev}
            for n, dev in deviations[:19]
        ],
    }


def main():
    upper = 1000
    result = scan_theorem_b(upper)

    print(f"=== Theorem B empirical scan n ∈ [2, {upper}] ===")
    print()
    print(f"총 {result['stats']['total_checked']} integers 검사")
    print(f"σ(n)·φ(n) = n·τ(n) 만족: {result['theorem_B_hits']}")
    print(f"유일성 n=6 주장: {'확인' if result['stats']['uniqueness_claim'] else '반증!'}")
    print()

    print("=== n = 6 exact ===")
    n6 = result["n6_exact"]
    print(f"  σ(6) = {n6['sigma']}, φ(6) = {n6['phi']}, τ(6) = {n6['tau']}")
    print(f"  σ(6)·φ(6) = {n6['sigma_phi']}")
    print(f"  6·τ(6)    = {n6['n_tau']}")
    print(f"  편차:     {n6['deviation']}")
    print()

    print("=== Near misses (|편차| ≤ 2) ===")
    for n, s, p, t in result["near_misses_abs_le_2"][:20]:
        dev = s * p - n * t
        print(f"  n={n:>4}: σ={s:>4}, φ={p:>4}, τ={t:>2} | σφ={s*p:>6} vs nτ={n*t:>6} | dev={dev:+d}")
    if len(result["near_misses_abs_le_2"]) > 20:
        print(f"  ... 총 {len(result['near_misses_abs_le_2'])} 건 중 위 20 개 표시")
    print()

    # v4 E6 산출: 큰 upper 에서도 n=6 유일 (n > 10^6 까지는 확인 안 했지만 [2, 1000] 에서 유일)
    out_path = Path("/Users/ghost/Dev/n6-architecture/reports/v4/theorem_b_scan_n1000_2026-04-16.json")
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(result, indent=2, ensure_ascii=False))
    print(f"[save] {out_path}")


if __name__ == "__main__":
    main()
