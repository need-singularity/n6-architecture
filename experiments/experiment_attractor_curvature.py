"""
Experiment: Attractor Dimension + Energy Surface Curvature
===========================================================
H-EE-29: Loss attractor has fractal dimension ~6.
H-EE-35: Phi = -K (Gaussian curvature of energy surface).
"""

import sys
sys.path.insert(0, '.')

import numpy as np
import math

from engine.leech24_surface import energy, phi_from_energy, N6_OPTIMA

SEED = 42
np.random.seed(SEED)


# ─── H-EE-29: Correlation Dimension via Grassberger-Procaccia ───

def embed_time_series(series, dim, delay=1):
    """Takens time-delay embedding."""
    n = len(series) - (dim - 1) * delay
    if n <= 0:
        return np.array([])
    embedded = np.zeros((n, dim))
    for i in range(dim):
        embedded[:, i] = series[i * delay: i * delay + n]
    return embedded


def correlation_dimension(series, dims=(4, 5, 6, 7, 8, 10), delay=1, n_radii=20):
    """Estimate correlation dimension D_2 via Grassberger-Procaccia algorithm."""
    results = {}
    for dim in dims:
        points = embed_time_series(series, dim, delay)
        if len(points) < 50:
            continue

        # Compute pairwise distances
        n = len(points)
        # Sample for efficiency
        sample_size = min(n, 300)
        idx = np.random.choice(n, sample_size, replace=False)
        sampled = points[idx]

        dists = []
        for i in range(sample_size):
            for j in range(i + 1, sample_size):
                d = np.linalg.norm(sampled[i] - sampled[j])
                if d > 0:
                    dists.append(d)

        if not dists:
            continue

        dists = np.array(dists)
        r_min = np.percentile(dists, 5)
        r_max = np.percentile(dists, 95)
        radii = np.logspace(np.log10(r_min), np.log10(r_max), n_radii)

        # Correlation integral C(r) = fraction of pairs with distance < r
        C_r = np.array([np.mean(dists < r) for r in radii])
        C_r = C_r[C_r > 0]
        radii_valid = radii[:len(C_r)]

        if len(C_r) < 5:
            continue

        # Linear fit in log-log space for the scaling region
        log_r = np.log(radii_valid)
        log_C = np.log(C_r)

        # Use middle 60% for fit (avoid boundary effects)
        n_pts = len(log_r)
        start = n_pts // 5
        end = 4 * n_pts // 5
        if end - start < 3:
            continue

        slope, intercept = np.polyfit(log_r[start:end], log_C[start:end], 1)
        results[dim] = {
            "D2_estimate": slope,
            "n_points": len(points),
        }

    return results


def generate_training_loss_curve(n_steps=2000, n6_architecture=True):
    """Simulate a training loss curve."""
    t = np.arange(n_steps, dtype=float)

    if n6_architecture:
        # N6: structured descent with 4 phase transitions
        base = 3.0 * np.exp(-t / 300)
        # 4 phase transitions at 1/6, 2/6, 3/6, 6/6
        for frac in [1/6, 2/6, 3/6, 1.0]:
            idx = int(frac * n_steps)
            if idx < n_steps:
                base[idx:] -= 0.1 * np.exp(-(t[idx:] - t[idx]) / 50)
        # Periodic component at frequency 1/6
        periodic = 0.05 * np.sin(2 * np.pi * t / 6)
        noise = np.random.randn(n_steps) * 0.02
        loss = np.maximum(base + periodic + noise, 0.01)
    else:
        # Standard: less structured
        base = 3.0 * np.exp(-t / 400)
        noise = np.random.randn(n_steps) * 0.05
        loss = np.maximum(base + noise, 0.01)

    return loss


# ─── H-EE-35: Gaussian Curvature of Leech-24 Surface ───

def numerical_hessian(func, point, epsilon=1e-3):
    """Compute Hessian matrix numerically."""
    dims = list(point.keys())
    n = len(dims)
    H = np.zeros((n, n))

    f0 = func(point)

    for i in range(n):
        for j in range(i, n):
            # f(x+ei+ej)
            p_pp = dict(point)
            p_pp[dims[i]] = p_pp[dims[i]] + epsilon
            p_pp[dims[j]] = p_pp[dims[j]] + epsilon
            f_pp = func(p_pp)

            # f(x+ei-ej)
            p_pm = dict(point)
            p_pm[dims[i]] = p_pm[dims[i]] + epsilon
            p_pm[dims[j]] = p_pm[dims[j]] - epsilon
            f_pm = func(p_pm)

            # f(x-ei+ej)
            p_mp = dict(point)
            p_mp[dims[i]] = p_mp[dims[i]] - epsilon
            p_mp[dims[j]] = p_mp[dims[j]] + epsilon
            f_mp = func(p_mp)

            # f(x-ei-ej)
            p_mm = dict(point)
            p_mm[dims[i]] = p_mm[dims[i]] - epsilon
            p_mm[dims[j]] = p_mm[dims[j]] - epsilon
            f_mm = func(p_mm)

            H[i, j] = (f_pp - f_pm - f_mp + f_mm) / (4 * epsilon ** 2)
            H[j, i] = H[i, j]

    return H, dims


def gaussian_curvature_proxy(hessian):
    """Approximate Gaussian curvature from Hessian eigenvalues."""
    eigenvalues = np.linalg.eigvalsh(hessian)
    # Gaussian curvature = product of principal curvatures
    # For high-dim, use determinant of Hessian as proxy
    # K ~ det(H) / (1 + |grad|^2)^((n+2)/2)
    # At critical points (grad=0): K ~ det(H)
    sign = np.sign(np.prod(np.sign(eigenvalues)))
    log_abs_det = np.sum(np.log(np.abs(eigenvalues) + 1e-30))
    return sign * np.exp(log_abs_det / len(eigenvalues))  # geometric mean


def main():
    print("=" * 70)
    print("  Experiment: Attractor Dimension + Energy Surface Curvature")
    print("  H-EE-29: Loss attractor dim ~6")
    print("  H-EE-35: Phi = -K (Gaussian curvature)")
    print("=" * 70)

    # ─── H-EE-29: Correlation Dimension ───
    print("\n" + "=" * 50)
    print("  Part 1: Correlation Dimension (H-EE-29)")
    print("=" * 50)

    for label, is_n6 in [("N6 architecture", True), ("Standard architecture", False)]:
        print(f"\n--- {label} ---")
        loss_curve = generate_training_loss_curve(2000, is_n6)
        results = correlation_dimension(loss_curve, dims=(3, 4, 5, 6, 7, 8, 10, 12))

        print(f"{'Embed dim':>10} {'D2 estimate':>12}")
        print("-" * 25)
        for dim, info in sorted(results.items()):
            marker = " <--" if abs(info["D2_estimate"] - 6.0) < 1.5 else ""
            print(f"{dim:>10} {info['D2_estimate']:>12.3f}{marker}")

        if results:
            d2_values = [info["D2_estimate"] for info in results.values()]
            # D2 should saturate around the true attractor dimension
            saturation = np.mean(d2_values[-3:]) if len(d2_values) >= 3 else np.mean(d2_values)
            print(f"\nSaturation D2 ~ {saturation:.2f}")
            print(f"Expected: ~6.0 for N6, variable for standard")

    # ─── H-EE-35: Gaussian Curvature ───
    print("\n" + "=" * 50)
    print("  Part 2: Gaussian Curvature (H-EE-35)")
    print("=" * 50)

    def energy_scalar(config):
        E, _ = energy(config)
        return E

    # Sample points at varying distances from N6 optimum
    test_points = []
    for scale in [0.0, 0.1, 0.3, 0.5, 1.0, 2.0]:
        point = {}
        for k, v in N6_OPTIMA.items():
            if v != 0:
                point[k] = v * (1.0 + scale * (np.random.random() - 0.5))
            else:
                point[k] = scale * np.random.random()
        test_points.append((scale, point))

    print(f"\n{'Scale':>7} {'E(x)':>10} {'Phi':>8} {'K (proxy)':>12} {'Phi + K':>10}")
    print("-" * 52)

    for scale, point in test_points:
        E, _ = energy(point)
        phi = phi_from_energy(E)

        # Compute Hessian and curvature (use subset of dims for efficiency)
        # Use only 6 key dimensions to make Hessian computation feasible
        key_dims = ["bottleneck_ratio", "dedekind_heads", "jordan_experts",
                     "boltzmann_fraction", "mertens_dropout", "hcn_dimension"]
        sub_point = {k: point[k] for k in key_dims}
        sub_optima = {k: N6_OPTIMA[k] for k in key_dims}

        def sub_energy(p):
            full = dict(point)
            full.update(p)
            E, _ = energy(full)
            return E

        H, dims = numerical_hessian(sub_energy, sub_point, epsilon=1e-2)
        K = gaussian_curvature_proxy(H)

        print(f"{scale:>7.1f} {E:>10.4f} {phi:>8.4f} {K:>12.6f} {phi + K:>10.4f}")

    # ─── Analysis ───
    print(f"\n--- Analysis ---")
    print(f"H-EE-29: If D2 saturates near 6, the loss attractor is 6-dimensional")
    print(f"H-EE-35: If Phi + K ~ 0 (or constant), then Phi = -K + const")
    print(f"Both connect n=6 to dynamical systems and differential geometry")


if __name__ == "__main__":
    main()
