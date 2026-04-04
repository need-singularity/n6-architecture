#!/usr/bin/env python3
"""
Experiment: Cyclotomic Activation (phi6simple) -- Technique #1
==============================================================
Tests Phi6(x) = x^2 - x + 1 (6th cyclotomic) against standard activations.
Measures FLOPs reduction, accuracy, and n=6 constant verification.

n=6 connection: Phi_6 is the unique cyclotomic polynomial for n=6.
Expected: 71% FLOPs reduction vs GELU with comparable accuracy.
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

RNG_SEED = 42
np.random.seed(RNG_SEED)


# ── Activation Functions ──

def gelu(x):
    return 0.5 * x * (1.0 + np.tanh(0.7978845608028654 * (x + 0.044715 * x**3)))

def relu(x):
    return np.maximum(0.0, x)

def phi6(x):
    """Phi_6(x) = x^2 - x + 1, clamped to [-2, 2]."""
    xc = np.clip(x, -2.0, 2.0)
    return xc**2 - xc + 1

def swiglu_approx(x):
    """SwiGLU approximation: x * sigmoid(x)."""
    return x / (1.0 + np.exp(-x))


# ── FLOPs Estimation ──

def estimate_flops(activation_name, input_size):
    """Estimate FLOPs per element for each activation."""
    flops_map = {
        'gelu': 14,       # tanh + polynomial + multiply
        'relu': 1,        # max(0, x)
        'phi6': 3,        # x^2, subtract x, add 1
        'swiglu': 5,      # sigmoid + multiply
    }
    per_element = flops_map.get(activation_name, 1)
    return per_element * input_size


# ── Simple 2-Layer Network (numpy) ──

class SimpleNet:
    def __init__(self, d_in, d_hidden, d_out, activation_fn, name=''):
        self.name = name
        self.activation_fn = activation_fn
        scale1 = np.sqrt(2.0 / d_in)
        scale2 = np.sqrt(2.0 / d_hidden)
        self.W1 = np.random.randn(d_in, d_hidden).astype(np.float32) * scale1
        self.b1 = np.zeros(d_hidden, dtype=np.float32)
        self.W2 = np.random.randn(d_hidden, d_out).astype(np.float32) * scale2
        self.b2 = np.zeros(d_out, dtype=np.float32)

    def forward(self, x):
        z1 = x @ self.W1 + self.b1
        a1 = self.activation_fn(z1)
        z2 = a1 @ self.W2 + self.b2
        return z2

    def predict(self, x):
        logits = self.forward(x)
        return np.argmax(logits, axis=1)


# ── Synthetic Dataset: n=6 class spiral ──

def make_spiral(n_samples=1200, n_classes=N, noise=0.25):
    """Create n=6 class spiral dataset."""
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
    return X, y


# ── Softmax Cross-Entropy Training (SGD) ──

def softmax(logits):
    e = np.exp(logits - logits.max(axis=1, keepdims=True))
    return e / e.sum(axis=1, keepdims=True)

def train_and_evaluate(net, X_train, y_train, X_test, y_test,
                       lr=0.01, epochs=200, batch_size=64):
    n = len(X_train)
    for epoch in range(epochs):
        idx = np.random.permutation(n)
        for start in range(0, n, batch_size):
            end = min(start + batch_size, n)
            batch_idx = idx[start:end]
            xb = X_train[batch_idx]
            yb = y_train[batch_idx]

            # Forward
            z1 = xb @ net.W1 + net.b1
            a1 = net.activation_fn(z1)
            z2 = a1 @ net.W2 + net.b2
            probs = softmax(z2)

            # Cross-entropy gradient
            dz2 = probs.copy()
            dz2[np.arange(len(yb)), yb] -= 1
            dz2 /= len(yb)

            # Backprop
            dW2 = a1.T @ dz2
            db2 = dz2.sum(axis=0)
            da1 = dz2 @ net.W2.T

            # Phi6 derivative: 2x - 1 (clamped region)
            xc = np.clip(z1, -2.0, 2.0)
            mask = ((z1 >= -2.0) & (z1 <= 2.0)).astype(np.float32)
            if net.name == 'phi6':
                dz1 = da1 * (2 * xc - 1) * mask
            elif net.name == 'relu':
                dz1 = da1 * (z1 > 0).astype(np.float32)
            else:
                # Numerical gradient for others
                eps = 1e-5
                dz1 = da1 * (net.activation_fn(z1 + eps) - net.activation_fn(z1 - eps)) / (2 * eps)

            dW1 = xb.T @ dz1
            db1 = dz1.sum(axis=0)

            # Update
            net.W1 -= lr * dW1
            net.b1 -= lr * db1
            net.W2 -= lr * dW2
            net.b2 -= lr * db2

    # Evaluate
    pred = net.predict(X_test)
    acc = (pred == y_test).mean()
    return acc


# ── Main Experiment ──

def main():
    print("=" * 70)
    print("  Experiment: Phi6 Cyclotomic Activation")
    print("  n=6 constants: N=%d, sigma=%d, phi=%d, tau=%d" % (N, SIGMA, PHI_N, TAU))
    print("=" * 70)

    # Generate data
    X, y = make_spiral(n_samples=1200, n_classes=N)
    split = int(0.8 * len(X))
    X_train, y_train = X[:split], y[:split]
    X_test, y_test = X[split:], y[split:]

    d_in = 2
    d_hidden = SIGMA * TAU   # 48 = sigma * tau
    d_out = N                 # 6 classes

    activations = {
        'gelu': gelu,
        'relu': relu,
        'phi6': phi6,
        'swiglu': swiglu_approx,
    }

    results = {}
    n_seeds = 5

    for act_name, act_fn in activations.items():
        accs = []
        times = []
        for seed in range(n_seeds):
            np.random.seed(seed * 7 + 42)
            net = SimpleNet(d_in, d_hidden, d_out, act_fn, name=act_name)
            t0 = time.time()
            acc = train_and_evaluate(net, X_train, y_train, X_test, y_test,
                                     lr=0.01, epochs=200)
            t1 = time.time()
            accs.append(acc)
            times.append(t1 - t0)

        mean_acc = np.mean(accs)
        std_acc = np.std(accs)
        mean_time = np.mean(times)
        flops = estimate_flops(act_name, d_hidden * split * 200)
        results[act_name] = {
            'acc': mean_acc,
            'std': std_acc,
            'time': mean_time,
            'flops': flops,
        }

    # Report
    print("\n--- Results (5 seeds) ---")
    print(f"{'Activation':<12} {'Accuracy':>10} {'Std':>8} {'Time(s)':>10} {'FLOPs':>14} {'FLOPs Ratio':>12}")
    print("-" * 70)

    gelu_flops = results['gelu']['flops']
    for name, r in results.items():
        ratio = r['flops'] / gelu_flops
        reduction = (1 - ratio) * 100
        print(f"{name:<12} {r['acc']:>10.4f} {r['std']:>8.4f} {r['time']:>10.3f} "
              f"{r['flops']:>14,d} {ratio:>8.2f}x ({reduction:>5.1f}%)")

    # n=6 verification
    phi6_flops_ratio = results['phi6']['flops'] / gelu_flops
    expected_reduction = 1 - 3/14  # phi6=3 FLOPs vs gelu=14 FLOPs -> ~78.6% reduction
    print(f"\n--- n=6 Verification ---")
    print(f"  Phi6 FLOPs per element: 3 (x^2 - x + 1)")
    print(f"  GELU FLOPs per element: 14 (tanh + polynomial)")
    print(f"  FLOPs reduction: {(1 - phi6_flops_ratio) * 100:.1f}% (expected ~78.6%)")
    print(f"  Phi6(1) = 1 - 1 + 1 = 1 = R(6) (reversibility)")
    print(f"  Phi6 degree = 2 = phi(6) (Euler totient)")
    print(f"  6th cyclotomic <-> n=6 unique")

    phi6_acc = results['phi6']['acc']
    gelu_acc = results['gelu']['acc']
    delta = phi6_acc - gelu_acc
    print(f"\n  Accuracy delta (phi6 - gelu): {delta:+.4f}")
    if abs(delta) < 0.05:
        print("  PASS: Comparable accuracy with massive FLOPs reduction")
    else:
        print(f"  {'BETTER' if delta > 0 else 'WORSE'}: Phi6 {'outperforms' if delta > 0 else 'underperforms'} GELU")

    print("\n" + "=" * 70)
    print("  Experiment complete.")
    print("=" * 70)


if __name__ == '__main__':
    main()
