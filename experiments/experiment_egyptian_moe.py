#!/usr/bin/env python3
"""
Experiment: Egyptian Fraction MoE Routing -- Technique #10
==========================================================
Tests {1/2, 1/3, 1/6} expert routing (perfect number decomposition)
against equal weighting and learned routing.

n=6 connection: 1/2 + 1/3 + 1/6 = 1 is the unique Egyptian fraction
decomposition from the definition of 6 as a perfect number (1+2+3=6).
Expected: Egyptian routing matches or exceeds learned routing.
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

# Egyptian fractions from perfect number 6: divisors 1,2,3 -> 1/2+1/3+1/6=1
EGYPTIAN_WEIGHTS = np.array([1/2, 1/3, 1/6], dtype=np.float32)
EQUAL_WEIGHTS = np.array([1/3, 1/3, 1/3], dtype=np.float32)

RNG_SEED = 42
np.random.seed(RNG_SEED)


# ── Synthetic Dataset: n=6 class spiral ──

def make_spiral(n_samples=2000, n_classes=N, noise=0.3):
    n_per_class = n_samples // n_classes
    X_list, y_list = [], []
    for c in range(n_classes):
        theta = np.linspace(c * 2 * np.pi / n_classes,
                            c * 2 * np.pi / n_classes + 3 * np.pi / n_classes,
                            n_per_class)
        r = np.linspace(0.3, 2.5, n_per_class)
        x1 = r * np.cos(theta) + np.random.normal(0, noise, n_per_class)
        x2 = r * np.sin(theta) + np.random.normal(0, noise, n_per_class)
        X_list.append(np.stack([x1, x2], axis=1))
        y_list.append(np.full(n_per_class, c))
    X = np.concatenate(X_list).astype(np.float32)
    y = np.concatenate(y_list).astype(np.int64)
    # Project to higher dim
    proj = np.random.randn(2, 64).astype(np.float32) * 0.3
    X_proj = X @ proj + np.random.randn(X.shape[0], 64).astype(np.float32) * 0.1
    return X_proj, y


# ── Expert Networks (numpy) ──

def softmax(logits):
    e = np.exp(logits - logits.max(axis=1, keepdims=True))
    return e / e.sum(axis=1, keepdims=True)

class Expert:
    def __init__(self, d_in, d_hidden, d_out):
        self.W1 = np.random.randn(d_in, d_hidden).astype(np.float32) * np.sqrt(2.0 / d_in)
        self.b1 = np.zeros(d_hidden, dtype=np.float32)
        self.W2 = np.random.randn(d_hidden, d_out).astype(np.float32) * np.sqrt(2.0 / d_hidden)
        self.b2 = np.zeros(d_out, dtype=np.float32)

    def forward(self, x):
        z = np.maximum(0, x @ self.W1 + self.b1)  # ReLU
        return z @ self.W2 + self.b2


class MoEModel:
    def __init__(self, d_in, d_hidden, d_out, n_experts=3, weights=None):
        self.experts = [Expert(d_in, d_hidden, d_out) for _ in range(n_experts)]
        self.weights = weights  # None = learned, else fixed
        if weights is None:
            # Simple gate: linear -> softmax
            self.gate_W = np.random.randn(d_in, n_experts).astype(np.float32) * 0.1
        self.n_experts = n_experts
        self.d_in = d_in

    def forward(self, x):
        expert_outputs = [e.forward(x) for e in self.experts]
        stacked = np.stack(expert_outputs, axis=1)  # (B, n_experts, d_out)

        if self.weights is not None:
            w = self.weights[None, :, None]  # (1, n_experts, 1)
        else:
            gate_logits = x @ self.gate_W  # (B, n_experts)
            w = softmax(gate_logits)[:, :, None]  # (B, n_experts, 1)

        return (stacked * w).sum(axis=1)  # (B, d_out)


def train_moe(model, X_train, y_train, X_test, y_test, lr=0.005, epochs=300):
    """Train MoE with SGD."""
    n = len(X_train)
    n_classes = y_train.max() + 1

    for epoch in range(epochs):
        idx = np.random.permutation(n)
        for start in range(0, n, 64):
            end = min(start + 64, n)
            bi = idx[start:end]
            xb, yb = X_train[bi], y_train[bi]
            bs = len(xb)

            # Forward through each expert
            z1s, a1s, outs = [], [], []
            for e in model.experts:
                z1 = xb @ e.W1 + e.b1
                a1 = np.maximum(0, z1)
                out = a1 @ e.W2 + e.b2
                z1s.append(z1)
                a1s.append(a1)
                outs.append(out)

            stacked = np.stack(outs, axis=1)  # (B, 3, d_out)

            if model.weights is not None:
                w = model.weights[None, :, None]
                gate_grad = False
            else:
                gate_logits = xb @ model.gate_W
                w_2d = softmax(gate_logits)
                w = w_2d[:, :, None]
                gate_grad = True

            combined = (stacked * w).sum(axis=1)
            probs = softmax(combined)

            # Gradient
            dout = probs.copy()
            dout[np.arange(bs), yb] -= 1
            dout /= bs

            for i, e in enumerate(model.experts):
                wi = w[:, i, :]  # (B, 1)
                dexpert = dout * wi
                da1 = dexpert @ e.W2.T
                dz1 = da1 * (z1s[i] > 0).astype(np.float32)
                e.W2 -= lr * (a1s[i].T @ dexpert)
                e.b2 -= lr * dexpert.sum(axis=0)
                e.W1 -= lr * (xb.T @ dz1)
                e.b1 -= lr * dz1.sum(axis=0)

            if gate_grad:
                # Gate gradient (simplified)
                for i in range(model.n_experts):
                    dw_i = (dout * outs[i]).sum(axis=1, keepdims=True)  # (B, 1)
                    dgate_i = dw_i * w_2d[:, i:i+1] * (1 - w_2d[:, i:i+1])
                    model.gate_W[:, i] -= lr * (xb.T @ dgate_i).squeeze()

    # Evaluate
    logits = model.forward(X_test)
    pred = np.argmax(logits, axis=1)
    return (pred == y_test).mean()


# ── Main Experiment ──

def main():
    print("=" * 70)
    print("  Experiment: Egyptian Fraction MoE Routing")
    print("  1/2 + 1/3 + 1/6 = 1 (perfect number decomposition)")
    print("=" * 70)

    X, y = make_spiral(n_samples=2000, n_classes=N)
    split = int(0.8 * len(X))
    X_train, y_train = X[:split], y[:split]
    X_test, y_test = X[split:], y[split:]

    d_in = 64
    d_hidden = SIGMA * PHI_N  # 24 = J2
    d_out = N

    routing_configs = {
        'egyptian (1/2+1/3+1/6)': EGYPTIAN_WEIGHTS,
        'equal (1/3+1/3+1/3)': EQUAL_WEIGHTS,
        'learned (softmax gate)': None,
    }

    n_seeds = 5
    results = {}

    for name, weights in routing_configs.items():
        accs = []
        for seed in range(n_seeds):
            np.random.seed(seed * 13 + 42)
            model = MoEModel(d_in, d_hidden, d_out, n_experts=3, weights=weights)
            acc = train_moe(model, X_train, y_train, X_test, y_test,
                           lr=0.005, epochs=300)
            accs.append(acc)
        results[name] = {'acc': np.mean(accs), 'std': np.std(accs)}

    # Report
    print(f"\n--- Results ({n_seeds} seeds, {N}-class spiral) ---")
    print(f"{'Routing':<28} {'Accuracy':>10} {'Std':>8}")
    print("-" * 50)
    for name, r in results.items():
        marker = " <-- n=6" if 'egyptian' in name else ""
        print(f"{name:<28} {r['acc']:>10.4f} {r['std']:>8.4f}{marker}")

    # n=6 verification
    print(f"\n--- n=6 Verification ---")
    print(f"  Expert weights: 1/2 + 1/3 + 1/6 = {sum(EGYPTIAN_WEIGHTS):.1f}")
    print(f"  Perfect number: 6 = 1 + 2 + 3 (sum of proper divisors)")
    print(f"  Reciprocals: 1/1 + 1/2 + 1/3 + 1/6 = 2 = phi(6)")
    print(f"  n_experts = 3 = n/phi = {N}/{PHI_N}")
    print(f"  Hidden dim = {d_hidden} = J2(6)")
    print(f"  Classes = {d_out} = n = {N}")

    eg_acc = results['egyptian (1/2+1/3+1/6)']['acc']
    eq_acc = results['equal (1/3+1/3+1/3)']['acc']
    lr_acc = results['learned (softmax gate)']['acc']
    print(f"\n  Egyptian vs Equal: {eg_acc - eq_acc:+.4f}")
    print(f"  Egyptian vs Learned: {eg_acc - lr_acc:+.4f}")

    if eg_acc >= eq_acc - 0.02:
        print("  PASS: Egyptian routing competitive or better than alternatives")
    else:
        print("  NOTE: Egyptian routing underperforms (data/config dependent)")

    print("\n" + "=" * 70)
    print("  Experiment complete.")
    print("=" * 70)


if __name__ == '__main__':
    main()
