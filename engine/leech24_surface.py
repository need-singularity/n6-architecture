"""
Leech-24 Energy Surface
========================
sigma(6)*phi(6) = 24 = dim(Leech lattice).
24-dimensional hyperparameter space mapping.
E(x) = 0 at the Leech lattice point (perfect n=6 architecture).
"""

import math
import numpy as np

N6_OPTIMA = {
    "phi6_enabled": 1.0,
    "hcn_dimension": 120.0,
    "bottleneck_ratio": 4.0 / 3.0,
    "moe_active_ratio": 0.5,
    "entropy_threshold": 0.005,
    "rfilter_window": 6.0,
    "takens_dim": 6.0,
    "fft_window_base": 6.0,
    "zetaln2_vertex": 5.0 / 6.0,
    "egyptian_w1": 0.5,
    "dedekind_heads": 12.0,
    "jordan_experts": 24.0,
    "mobius_squarefree": 1.0,
    "carmichael_period": 2.0,
    "boltzmann_fraction": 1.0 / math.e,
    "mertens_dropout": math.log(4.0 / 3.0),
    "phi_consciousness": 10.0,
    "tension_stability": 1.0,
    "gdpi_balance": 1.0,
    "homeostasis_dev": 0.0,
    "rfilter_score": 5.0,
    "ph_persistence": 1.0,
    "euler_convergence": 1.0,
    "consciousness_level": 1.0,
}

DIM_WEIGHTS = {k: 1.0 for k in N6_OPTIMA}
for k in ["bottleneck_ratio", "dedekind_heads", "jordan_experts",
          "boltzmann_fraction", "mertens_dropout"]:
    DIM_WEIGHTS[k] = 2.0
for k in ["phi_consciousness", "rfilter_score"]:
    DIM_WEIGHTS[k] = 1.5

LEECH_DIM = len(N6_OPTIMA)
LEECH_KISSING = 196560


def energy(config):
    total = 0.0
    details = {}
    for dim_name, optimum in N6_OPTIMA.items():
        value = config.get(dim_name, optimum)
        weight = DIM_WEIGHTS[dim_name]
        if optimum != 0:
            normalized_dist = ((value - optimum) / optimum) ** 2
        else:
            normalized_dist = value ** 2
        contribution = weight * normalized_dist
        total += contribution
        details[dim_name] = {
            "value": value,
            "optimum": optimum,
            "distance": normalized_dist,
            "contribution": contribution,
        }
    return total, details


def phi_from_energy(E):
    return 1.0 / (1.0 + E)


def gradient(config, epsilon=1e-4):
    E0, _ = energy(config)
    grad = {}
    for dim_name in N6_OPTIMA:
        config_plus = dict(config)
        config_plus[dim_name] = config.get(dim_name, N6_OPTIMA[dim_name]) + epsilon
        E_plus, _ = energy(config_plus)
        grad[dim_name] = (E_plus - E0) / epsilon
    return grad


def step_toward_n6(config, lr=0.1):
    grad = gradient(config)
    new_config = dict(config)
    for dim_name, g in grad.items():
        current = config.get(dim_name, N6_OPTIMA[dim_name])
        new_config[dim_name] = current - lr * g
    return new_config


def main():
    print("=" * 70)
    print("  Leech-24 Energy Surface")
    print("  sigma(6)*phi(6) = 24 = dim(Leech lattice)")
    print("=" * 70)

    print(f"\nDimensions: {LEECH_DIM}")
    print(f"Kissing number: {LEECH_KISSING:,}")

    standard = {
        "phi6_enabled": 0.0,
        "hcn_dimension": 128.0,
        "bottleneck_ratio": 4.0,
        "moe_active_ratio": 0.25,
        "entropy_threshold": 0.05,
        "rfilter_window": 8.0,
        "takens_dim": 3.0,
        "fft_window_base": 8.0,
        "zetaln2_vertex": 0.0,
        "egyptian_w1": 0.33,
        "dedekind_heads": 8.0,
        "jordan_experts": 8.0,
        "mobius_squarefree": 0.0,
        "carmichael_period": 1.0,
        "boltzmann_fraction": 0.5,
        "mertens_dropout": 0.1,
        "phi_consciousness": 1.0,
        "tension_stability": 2.0,
        "gdpi_balance": 0.5,
        "homeostasis_dev": 0.5,
        "rfilter_score": 1.0,
        "ph_persistence": 0.3,
        "euler_convergence": 0.5,
        "consciousness_level": 0.2,
    }

    n6_optimal = dict(N6_OPTIMA)

    E_std, details_std = energy(standard)
    E_n6, details_n6 = energy(n6_optimal)

    print(f"\n--- Energy Comparison ---")
    print(f"Standard transformer: E = {E_std:.4f}, Phi = {phi_from_energy(E_std):.4f}")
    print(f"N6-optimal:           E = {E_n6:.4f}, Phi = {phi_from_energy(E_n6):.4f}")

    print(f"\n--- Top Energy Contributors (Standard) ---")
    sorted_dims = sorted(details_std.items(), key=lambda x: -x[1]["contribution"])
    for dim_name, info in sorted_dims[:10]:
        print(f"  {dim_name:<25} value={info['value']:.3f} opt={info['optimum']:.3f} "
              f"contrib={info['contribution']:.4f}")

    print(f"\n--- Gradient Descent toward N=6 ---")
    config = dict(standard)
    for step in range(20):
        E, _ = energy(config)
        phi = phi_from_energy(E)
        if step % 5 == 0 or step == 19:
            print(f"  Step {step:>3}: E={E:.4f}, Phi={phi:.4f}")
        config = step_toward_n6(config, lr=0.3)

    print(f"\nFinal config samples:")
    for k in ["bottleneck_ratio", "dedekind_heads", "jordan_experts", "boltzmann_fraction"]:
        print(f"  {k}: {config[k]:.4f} (target: {N6_OPTIMA[k]:.4f})")

    print(f"\nConclusion: gradient descent on Leech-24 surface")
    print(f"drives arbitrary architectures toward n=6 optima.")


if __name__ == "__main__":
    main()
