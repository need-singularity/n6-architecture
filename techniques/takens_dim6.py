#!/usr/bin/env python3
"""H-SEDI-7: Takens embedding dim=6 optimal for training dynamics.
Embed loss curves at dims 4,5,6,7,8,10, compute persistence via distance matrices.
Measure: which embedding dimension produces most persistent topological features?
"""
import sys, os; sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__)))); sys.path.insert(0, '/Users/ghost/Dev/TECS-L')
import torch, torch.nn as nn, numpy as np
from model_pure_field import PureFieldEngine
from model_utils import load_mnist
from scipy.spatial.distance import pdist, squareform

def takens_embed(series, dim, delay=1):
    """Takens time-delay embedding."""
    n = len(series) - (dim - 1) * delay
    return np.array([series[i:i + dim * delay:delay] for i in range(n)])

def persistence_score(embedded):
    """Approximate persistence: count significant gaps in distance distribution.
    More gaps = more topological structure = better embedding.
    """
    # Subsample for speed
    if len(embedded) > 500:
        idx = np.random.choice(len(embedded), 500, replace=False)
        embedded = embedded[idx]
    dists = pdist(embedded)
    if len(dists) == 0:
        return 0.0, 0
    sorted_d = np.sort(dists)
    # Compute gaps between consecutive sorted distances (normalized)
    gaps = np.diff(sorted_d)
    if gaps.std() < 1e-10:
        return 0.0, 0
    significant_gaps = np.sum(gaps > gaps.mean() + 2 * gaps.std())
    # Persistence = range of significant gaps / total range
    persistence = significant_gaps / len(gaps) if len(gaps) > 0 else 0
    return persistence, significant_gaps

def main():
    print("=" * 60)
    print("H-SEDI-7: Takens Embedding dim=6 Optimality Test")
    print("=" * 60)
    train_loader, _ = load_mnist(batch_size=64)
    model = PureFieldEngine(784, 128, 10)
    optimizer = torch.optim.Adam(model.parameters(), lr=0.001)
    criterion = nn.CrossEntropyLoss()

    # Train 3 epochs, collect per-batch loss
    batch_losses = []
    for epoch in range(3):
        model.train()
        for X, y in train_loader:
            X = X.view(X.size(0), -1)
            optimizer.zero_grad()
            out, tension = model(X)
            loss = criterion(out, y)
            loss.backward()
            optimizer.step()
            batch_losses.append(loss.item())
        print(f"  Epoch {epoch+1}: last_loss={batch_losses[-1]:.4f}")

    losses = np.array(batch_losses)
    print(f"\nTotal batches: {len(losses)}")

    # Test different embedding dimensions
    dims = [4, 5, 6, 7, 8, 10]
    print(f"\n{'Dim':>5} {'Persistence':>12} {'SigGaps':>8} {'EmbedSize':>10} {'DistStd':>10}")
    print("-" * 50)
    results = {}
    for d in dims:
        embedded = takens_embed(losses, d)
        pers, gaps = persistence_score(embedded)
        # Also compute distance matrix spread
        sub = embedded[:500] if len(embedded) > 500 else embedded
        dist_std = pdist(sub).std()
        results[d] = {'persistence': pers, 'gaps': gaps, 'embed_size': len(embedded), 'dist_std': dist_std}
        print(f"{d:>5} {pers:>12.6f} {gaps:>8} {len(embedded):>10} {dist_std:>10.4f}")

    # Rank by persistence
    ranked = sorted(results.items(), key=lambda x: -x[1]['persistence'])
    print(f"\n--- Ranking by Persistence ---")
    for i, (d, r) in enumerate(ranked):
        marker = " <-- BEST" if i == 0 else ""
        print(f"  #{i+1}: dim={d}, persistence={r['persistence']:.6f}{marker}")

    best_dim = ranked[0][0]
    dim6_rank = [i+1 for i, (d, _) in enumerate(ranked) if d == 6][0]

    # Also test with tension signal
    print(f"\n--- Tension Signal Embedding ---")
    batch_tensions = []
    model.eval()
    with torch.no_grad():
        for X, y in train_loader:
            X = X.view(X.size(0), -1)
            _, tension = model(X)
            batch_tensions.append(tension.mean().item())
    tensions = np.array(batch_tensions)

    print(f"{'Dim':>5} {'Persistence':>12} {'SigGaps':>8}")
    print("-" * 30)
    t_results = {}
    for d in dims:
        embedded = takens_embed(tensions, d)
        pers, gaps = persistence_score(embedded)
        t_results[d] = pers
        print(f"{d:>5} {pers:>12.6f} {gaps:>8}")
    t_ranked = sorted(t_results.items(), key=lambda x: -x[1])
    t_best = t_ranked[0][0]

    print(f"\n{'='*60}")
    print(f"VERDICT:")
    print(f"  Loss curve: best dim={best_dim}, dim=6 rank={dim6_rank}/6")
    print(f"  Tension curve: best dim={t_best}")
    print(f"  dim=6 optimal = {best_dim == 6 or t_best == 6}")
    status = "SUPPORTED" if best_dim == 6 or t_best == 6 else "PARTIAL" if dim6_rank <= 3 else "REFUTED"
    print(f"  Status: {status}")

if __name__ == '__main__':
    main()
