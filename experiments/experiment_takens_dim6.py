#!/usr/bin/env python3
"""
Experiment: Takens Embedding dim=6 -- Technique #10
====================================================
Tests loss curve embedding in dimension n=6 for training diagnostics.
n=6 connection: Takens embedding dimension d=n=6.
Compares embedding dimensions 3,4,5,6,7,8,10 for loss curve attractor analysis.
Expected: dim=6 yields optimal persistence (topological structure) for training curves.
"""
import sys, os, time, math
import numpy as np
from scipy.spatial.distance import pdist

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from techniques.takens_dim6 import takens_embed, persistence_score

np.random.seed(42)


def generate_training_curves(n_curves=5, n_steps=500):
    """Generate synthetic training loss curves with different dynamics."""
    curves = {}
    t = np.arange(n_steps, dtype=float)

    # 1. Smooth exponential decay
    curves['exp_decay'] = 2.0 * np.exp(-0.005 * t) + 0.1

    # 2. Decay with oscillation (learning rate cycling)
    curves['oscillating'] = 1.5 * np.exp(-0.004 * t) + 0.15 * np.sin(2 * np.pi * t / 50) * np.exp(-0.002 * t) + 0.1

    # 3. Step-wise decay (phase transitions)
    phase = np.zeros(n_steps)
    phase[:150] = 2.0
    phase[150:300] = 0.8
    phase[300:] = 0.3
    curves['phase_steps'] = phase + 0.05 * np.random.randn(n_steps)

    # 4. Noisy convergence (realistic)
    curves['noisy'] = 1.8 * np.exp(-0.006 * t) + 0.08 * np.random.randn(n_steps) + 0.2

    # 5. Divergent then convergent (warm-up)
    warmup = np.clip(0.5 + 0.01 * t[:50], 0.5, 1.0)
    decay = 2.0 * np.exp(-0.005 * (t[50:] - 50)) + 0.15
    curves['warmup'] = np.concatenate([warmup * 2.5, decay])

    return curves


def analyze_embedding_dimension(loss_curve, dims, delay=1):
    """Analyze persistence across embedding dimensions for one loss curve."""
    results = {}
    for d in dims:
        embedded = takens_embed(loss_curve, dim=d, delay=delay)
        if len(embedded) < 10:
            results[d] = {'persistence': 0.0, 'gaps': 0, 'embed_rows': len(embedded)}
            continue
        pers, gaps = persistence_score(embedded)

        # Also compute distance distribution statistics
        sub = embedded[:300] if len(embedded) > 300 else embedded
        dists = pdist(sub)
        dist_std = dists.std() if len(dists) > 0 else 0.0
        dist_range = dists.max() - dists.min() if len(dists) > 0 else 0.0

        results[d] = {
            'persistence': pers,
            'gaps': int(gaps),
            'embed_rows': len(embedded),
            'dist_std': dist_std,
            'dist_range': dist_range,
        }
    return results


def run_delay_sweep(loss_curve, dim=6, delays=None):
    """Sweep delay parameter for fixed dim=6."""
    if delays is None:
        delays = [1, 2, 3, 5, 8, 13]
    results = {}
    for tau in delays:
        embedded = takens_embed(loss_curve, dim=dim, delay=tau)
        if len(embedded) < 10:
            results[tau] = {'persistence': 0.0, 'gaps': 0}
            continue
        pers, gaps = persistence_score(embedded)
        results[tau] = {'persistence': pers, 'gaps': int(gaps)}
    return results


def main():
    print("=" * 60)
    print("  Experiment: Takens Embedding dim=6 (Technique #10)")
    print("  Takens theorem: d >= 2*attractor_dim + 1")
    print("  n=6 sufficient for low-dim training attractors")
    print("=" * 60)

    dims = [3, 4, 5, 6, 7, 8, 10]
    curves = generate_training_curves()

    print(f"\n  Test dimensions: {dims}")
    print(f"  Loss curves: {list(curves.keys())}")

    # Per-curve analysis
    all_results = {}
    for curve_name, loss_curve in curves.items():
        print(f"\n--- Curve: {curve_name} ({len(loss_curve)} steps) ---")
        results = analyze_embedding_dimension(loss_curve, dims)
        all_results[curve_name] = results

        print(f"  {'Dim':>5} | {'Persistence':>12} | {'SigGaps':>8} | {'DistStd':>10} | {'EmbedRows':>10}")
        print("  " + "-" * 55)
        for d in dims:
            r = results[d]
            print(f"  {d:>5} | {r['persistence']:>12.6f} | {r['gaps']:>8} | "
                  f"{r.get('dist_std', 0):>10.4f} | {r['embed_rows']:>10}")

    # Aggregate: which dim wins most often?
    print(f"\n{'=' * 60}")
    print(f"  Aggregate Ranking (best persistence per curve)")
    print(f"{'=' * 60}\n")

    dim_wins = {d: 0 for d in dims}
    dim_total_pers = {d: 0.0 for d in dims}

    print(f"  {'Curve':>15} | {'Best dim':>8} | {'Best pers':>10} | {'dim=6 pers':>10} | {'dim=6 rank':>10}")
    print("  " + "-" * 65)

    for curve_name, results in all_results.items():
        ranked = sorted(dims, key=lambda d: -results[d]['persistence'])
        best_dim = ranked[0]
        dim_wins[best_dim] += 1
        for d in dims:
            dim_total_pers[d] += results[d]['persistence']

        dim6_rank = ranked.index(6) + 1
        print(f"  {curve_name:>15} | {best_dim:>8} | {results[best_dim]['persistence']:>10.6f} | "
              f"{results[6]['persistence']:>10.6f} | {dim6_rank:>10}")

    # Summary
    print(f"\n=== Win Counts ===")
    for d in dims:
        marker = " <-- n=6" if d == 6 else ""
        print(f"  dim={d}: {dim_wins[d]} wins, avg_persistence={dim_total_pers[d]/len(curves):.6f}{marker}")

    # Delay sweep for dim=6
    print(f"\n=== Delay Sweep (dim=6, curve=oscillating) ===")
    delay_results = run_delay_sweep(curves['oscillating'], dim=6)
    print(f"  {'Delay':>6} | {'Persistence':>12} | {'SigGaps':>8}")
    print("  " + "-" * 35)
    for tau, r in sorted(delay_results.items()):
        print(f"  {tau:>6} | {r['persistence']:>12.6f} | {r['gaps']:>8}")

    print(f"\n  n=6: Takens theorem d=6 covers attractors of dim <= 2")
    print(f"  Training dynamics typically live on low-dim manifolds")
    print()


if __name__ == '__main__':
    main()
