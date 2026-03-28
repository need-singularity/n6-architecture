"""
Thermodynamic Frame
====================
R(n) = sigma(n)*phi(n) / (n*tau(n))
R(6) = 1 — the unique reversibility condition.

Decomposes any architecture into {sigma, phi, n, tau} subsystems
and computes per-subsystem efficiency.
"""

import math
import torch
import torch.nn as nn
import numpy as np


def sigma(n):
    """Sum of divisors."""
    return sum(d for d in range(1, n + 1) if n % d == 0)


def tau(n):
    """Count of divisors."""
    return sum(1 for d in range(1, n + 1) if n % d == 0)


def euler_phi(n):
    """Euler's totient."""
    return sum(1 for k in range(1, n + 1) if math.gcd(k, n) == 1)


def R(n):
    """Balance ratio. R(n)=1 iff n in {1, 6}."""
    if n < 1:
        return float('inf')
    s, t, p = sigma(n), tau(n), euler_phi(n)
    return (s * p) / (n * t)


def scan_R_spectrum(max_n=100):
    """Compute R(n) for n=1..max_n. Only n=1 and n=6 give R=1."""
    return {n: R(n) for n in range(1, max_n + 1)}


class ArchitectureConfig:
    """Represents a neural architecture's n=6 alignment."""

    def __init__(self, d_model, d_ff, n_heads, n_experts=1,
                 routing_weights=None, dropout=0.0, activation="phi6"):
        self.d_model = d_model
        self.d_ff = d_ff
        self.n_heads = n_heads
        self.n_experts = n_experts
        self.routing_weights = routing_weights or []
        self.dropout = dropout
        self.activation = activation

    def sigma_subsystem_score(self):
        if self.d_model % 12 == 0:
            return 1.0
        remainder = min(self.d_model % 12, 12 - self.d_model % 12)
        return 1.0 - remainder / 12.0

    def phi_subsystem_score(self):
        if self.n_experts <= 1:
            return 1.0
        if self.routing_weights:
            target = [1/2, 1/3, 1/6]
            if len(self.routing_weights) == 3:
                sorted_w = sorted(self.routing_weights, reverse=True)
                error = sum(abs(a - b) for a, b in zip(sorted_w, target))
                return max(0, 1.0 - error)
        return 0.5

    def n_subsystem_score(self):
        if self.activation in ("phi6", "phi6simple", "cyclotomic"):
            return 1.0
        elif self.activation in ("zetaln2", "gz"):
            return 0.8
        elif self.activation in ("gelu", "relu"):
            return 0.5
        return 0.3

    def tau_subsystem_score(self):
        if self.d_model == 0:
            return 0.0
        ratio = self.d_ff / self.d_model
        target = 4.0 / 3.0
        error = abs(ratio - target) / target
        return max(0, 1.0 - error)

    def R_score(self):
        s = self.sigma_subsystem_score()
        p = self.phi_subsystem_score()
        n = self.n_subsystem_score()
        t = self.tau_subsystem_score()
        return s * p * n * t

    def decomposition(self):
        return {
            "sigma_score": self.sigma_subsystem_score(),
            "phi_score": self.phi_subsystem_score(),
            "n_score": self.n_subsystem_score(),
            "tau_score": self.tau_subsystem_score(),
            "R_score": self.R_score(),
            "config": {
                "d_model": self.d_model,
                "d_ff": self.d_ff,
                "n_heads": self.n_heads,
                "n_experts": self.n_experts,
                "activation": self.activation,
            }
        }


def entropy_of_distribution(probs):
    probs = np.array(probs)
    probs = probs[probs > 0]
    return -np.sum(probs * np.log(probs))


def clausius_check(grad_entropy_delta, data_entropy_delta):
    total = grad_entropy_delta + data_entropy_delta
    return total >= 0, total


def main():
    print("=" * 70)
    print("  Thermodynamic Frame: R(n) = sigma*phi / (n*tau)")
    print("=" * 70)

    print("\n--- R-Spectrum (n=1..30) ---")
    print(f"{'n':>4} {'sigma':>6} {'tau':>4} {'phi':>4} {'R(n)':>10}")
    print("-" * 32)
    for n in range(1, 31):
        r = R(n)
        marker = " <-- R=1!" if abs(r - 1.0) < 1e-10 else ""
        print(f"{n:>4} {sigma(n):>6} {tau(n):>4} {euler_phi(n):>4} {r:>10.6f}{marker}")

    print("\n--- Architecture Decomposition ---")
    configs = [
        ("Standard (d=128, GELU, 4x FFN)",
         ArchitectureConfig(128, 512, 8, activation="gelu")),
        ("N6-Optimal (d=120, Phi6, 4/3x FFN, 12 heads)",
         ArchitectureConfig(120, 160, 12, activation="phi6")),
        ("N6-MoE (d=120, Phi6, 24 experts, Egyptian)",
         ArchitectureConfig(120, 160, 12, 24, [1/2, 1/3, 1/6], activation="phi6")),
        ("Partial (d=120, GELU, 4x FFN)",
         ArchitectureConfig(120, 480, 12, activation="gelu")),
    ]

    for label, cfg in configs:
        d = cfg.decomposition()
        print(f"\n{label}")
        print(f"  sigma={d['sigma_score']:.3f}  phi={d['phi_score']:.3f}  "
              f"n={d['n_score']:.3f}  tau={d['tau_score']:.3f}")
        print(f"  R_score = {d['R_score']:.4f}")

    print("\n--- Conclusion ---")
    print("R(6) = 1.000000 — unique among n >= 2")
    print("Architecture R-score measures distance from thermodynamic optimum")
    print("R=1 architectures achieve reversible (zero-waste) information processing")


if __name__ == "__main__":
    main()
