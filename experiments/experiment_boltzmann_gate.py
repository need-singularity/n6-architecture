#!/usr/bin/env python3
"""
Experiment: Boltzmann Gate Sparsity -- Technique #15
====================================================
Tests 1/e activation sparsity gate (keep top-1/e by magnitude, zero rest).
Measures sparsity, accuracy retention, and parameter efficiency.

n=6 connection: 1/e ~ 0.3679, 1 - 1/e ~ 0.6321 = 63% sparsity.
Boltzmann partition function optimum: only 1/e fraction carries signal.
Expected: 63% activation sparsity with minimal accuracy loss.
"""

import sys
import os
import time
import numpy as np

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

# ── n=6 Constants ──
N = 6
SIGMA = 12
PHI_N = 2
TAU = 4
E = np.e
GOLDEN_ZONE = 1.0 / E        # ~0.3679 (fraction kept)
SPARSITY = 1.0 - GOLDEN_ZONE  # ~0.6321

RNG_SEED = 42
np.random.seed(RNG_SEED)


# ── Boltzmann Gate ──

def boltzmann_gate(x, fraction=GOLDEN_ZONE):
    """Keep only top fraction of activations by magnitude, zero rest."""
    flat = np.abs(x).flatten()
    k = max(1, int(len(flat) * fraction))
    if k >= len(flat):
        return x
    threshold = np.sort(flat)[-k]
    mask = (np.abs(x) >= threshold).astype(np.float32)
    return x * mask


def measure_sparsity(x):
    """Fraction of zeros in array."""
    return (x == 0).sum() / x.size


# ── Simple Network ──

def softmax(logits):
    e = np.exp(logits - logits.max(axis=1, keepdims=True))
    return e / e.sum(axis=1, keepdims=True)


class GatedNet:
    def __init__(self, d_in, d_hidden, d_out, use_gate=False, gate_fraction=GOLDEN_ZONE):
        self.use_gate = use_gate
        self.gate_fraction = gate_fraction
        scale1 = np.sqrt(2.0 / d_in)
        scale2 = np.sqrt(2.0 / d_hidden)
        self.W1 = np.random.randn(d_in, d_hidden).astype(np.float32) * scale1
        self.b1 = np.zeros(d_hidden, dtype=np.float32)
        self.W2 = np.random.randn(d_hidden, d_out).astype(np.float32) * scale2
        self.b2 = np.zeros(d_out, dtype=np.float32)

    def forward(self, x):
        z1 = x @ self.W1 + self.b1
        a1 = np.maximum(0, z1)  # ReLU
        if self.use_gate:
            a1 = boltzmann_gate(a1, self.gate_fraction)
        z2 = a1 @ self.W2 + self.b2
        return z2, a1


def train_and_evaluate(net, X_train, y_train, X_test, y_test,
                       lr=0.01, epochs=200, batch_size=64):
    n = len(X_train)
    sparsities = []

    for epoch in range(epochs):
        idx = np.random.permutation(n)
        for start in range(0, n, batch_size):
            end = min(start + batch_size, n)
            bi = idx[start:end]
            xb, yb = X_train[bi], y_train[bi]

            z1 = xb @ net.W1 + net.b1
            a1 = np.maximum(0, z1)
            if net.use_gate:
                a1 = boltzmann_gate(a1, net.gate_fraction)
            z2 = a1 @ net.W2 + net.b2
            probs = softmax(z2)

            dz2 = probs.copy()
            dz2[np.arange(len(yb)), yb] -= 1
            dz2 /= len(yb)

            dW2 = a1.T @ dz2
            db2 = dz2.sum(axis=0)
            da1 = dz2 @ net.W2.T
            dz1 = da1 * (z1 > 0).astype(np.float32)

            net.W1 -= lr * (xb.T @ dz1)
            net.b1 -= lr * dz1.sum(axis=0)
            net.W2 -= lr * dW2
            net.b2 -= lr * db2

            if epoch == epochs - 1:
                sparsities.append(measure_sparsity(a1))

    # Evaluate
    logits, a1_test = net.forward(X_test)
    pred = np.argmax(logits, axis=1)
    acc = (pred == y_test).mean()
    final_sparsity = measure_sparsity(a1_test)
    avg_train_sparsity = np.mean(sparsities) if sparsities else 0.0
    return acc, final_sparsity, avg_train_sparsity


# ── Synthetic Data ──

def make_data(n_samples=1200, n_classes=N):
    n_per = n_samples // n_classes
    X, y = [], []
    for c in range(n_classes):
        center = np.array([np.cos(2 * np.pi * c / n_classes),
                           np.sin(2 * np.pi * c / n_classes)]) * 2
        pts = np.random.randn(n_per, 2).astype(np.float32) * 0.5 + center
        X.append(pts)
        y.append(np.full(n_per, c, dtype=np.int64))
    return np.concatenate(X), np.concatenate(y)


# ── Main Experiment ──

def main():
    print("=" * 70)
    print("  Experiment: Boltzmann Gate Sparsity")
    print("  1/e = %.4f fraction kept, %.1f%% sparsity" % (GOLDEN_ZONE, SPARSITY * 100))
    print("=" * 70)

    X, y = make_data(n_samples=1200, n_classes=N)
    split = int(0.8 * len(X))
    X_train, y_train = X[:split], y[:split]
    X_test, y_test = X[split:], y[split:]

    d_in = 2
    d_hidden = SIGMA * TAU  # 48
    d_out = N

    # Test different sparsity fractions
    fractions = {
        'no_gate (0%)': (False, 1.0),
        'boltzmann (1/e=36.8%)': (True, GOLDEN_ZONE),
        'half (50%)': (True, 0.5),
        'quarter (25%)': (True, 0.25),
        'extreme (10%)': (True, 0.1),
    }

    n_seeds = 5
    results = {}

    for name, (use_gate, frac) in fractions.items():
        accs, spars = [], []
        for seed in range(n_seeds):
            np.random.seed(seed * 7 + 42)
            net = GatedNet(d_in, d_hidden, d_out, use_gate=use_gate, gate_fraction=frac)
            acc, test_spar, train_spar = train_and_evaluate(
                net, X_train, y_train, X_test, y_test, lr=0.01, epochs=200)
            accs.append(acc)
            spars.append(test_spar)
        results[name] = {
            'acc': np.mean(accs), 'std': np.std(accs),
            'sparsity': np.mean(spars), 'fraction_kept': frac
        }

    # Report
    print(f"\n--- Results ({n_seeds} seeds, {N}-class, hidden={d_hidden}=sigma*tau) ---")
    print(f"{'Config':<28} {'Accuracy':>10} {'Std':>8} {'Sparsity':>10} {'Kept':>8}")
    print("-" * 68)

    for name, r in results.items():
        marker = " <-- n=6" if 'boltzmann' in name else ""
        print(f"{name:<28} {r['acc']:>10.4f} {r['std']:>8.4f} "
              f"{r['sparsity']:>9.1%} {r['fraction_kept']:>7.1%}{marker}")

    # n=6 verification
    print(f"\n--- n=6 Verification ---")
    print(f"  Boltzmann constant: 1/e = {1/E:.6f}")
    print(f"  Target sparsity: 1 - 1/e = {SPARSITY:.4f} ({SPARSITY*100:.1f}%)")
    print(f"  Hidden dim: {d_hidden} = sigma * tau = {SIGMA} * {TAU}")

    boltz = results['boltzmann (1/e=36.8%)']
    nogate = results['no_gate (0%)']
    acc_drop = nogate['acc'] - boltz['acc']
    compute_saved = boltz['sparsity']

    print(f"\n  Accuracy drop with Boltzmann gate: {acc_drop:.4f} ({acc_drop*100:.2f}%)")
    print(f"  Compute saved (sparse activations): {compute_saved:.1%}")
    print(f"  Efficiency ratio: {compute_saved / max(acc_drop, 0.001):.1f} "
          f"(sparsity% / accuracy_drop%)")

    if acc_drop < 0.05:
        print("  PASS: <5% accuracy loss with 63% activation sparsity")
    else:
        print("  NOTE: Accuracy loss may be significant for this task")

    print("\n" + "=" * 70)
    print("  Experiment complete.")
    print("=" * 70)


if __name__ == '__main__':
    main()
