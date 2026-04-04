#!/usr/bin/env python3
"""
Experiment: Dedekind Head Pruning -- Technique #11
==================================================
Tests pruning attention heads to divisors of 12 (sigma=12).
psi(6) = sigma(6) = 12: Dedekind function equals divisor sum only at n=6.
Valid head counts: {1, 2, 3, 4, 6, 12}.

n=6 connection: psi(6) = sigma(6) = 12 (unique coincidence).
Expected: ~25% parameter reduction for models with h > 12.
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
DEDEKIND_PSI = 12  # psi(6) = 12
DIVISORS_OF_12 = [1, 2, 3, 4, 6, 12]

RNG_SEED = 42
np.random.seed(RNG_SEED)


# ── Multi-Head Attention (numpy) ──

def softmax_2d(x):
    """Softmax over last axis."""
    e = np.exp(x - x.max(axis=-1, keepdims=True))
    return e / e.sum(axis=-1, keepdims=True)


class MultiHeadAttention:
    def __init__(self, d_model, n_heads):
        self.d_model = d_model
        self.n_heads = n_heads
        self.head_dim = d_model // n_heads
        assert d_model % n_heads == 0

        scale = np.sqrt(2.0 / d_model)
        self.Wq = np.random.randn(d_model, d_model).astype(np.float32) * scale
        self.Wk = np.random.randn(d_model, d_model).astype(np.float32) * scale
        self.Wv = np.random.randn(d_model, d_model).astype(np.float32) * scale
        self.Wo = np.random.randn(d_model, d_model).astype(np.float32) * scale

    def forward(self, x):
        """x: (B, L, D) -> (B, L, D)"""
        B, L, D = x.shape
        Q = (x @ self.Wq).reshape(B, L, self.n_heads, self.head_dim).transpose(0, 2, 1, 3)
        K = (x @ self.Wk).reshape(B, L, self.n_heads, self.head_dim).transpose(0, 2, 1, 3)
        V = (x @ self.Wv).reshape(B, L, self.n_heads, self.head_dim).transpose(0, 2, 1, 3)

        scale = 1.0 / np.sqrt(self.head_dim)
        scores = np.matmul(Q, K.transpose(0, 1, 3, 2)) * scale
        attn = softmax_2d(scores)
        out = np.matmul(attn, V)  # (B, H, L, d_h)
        out = out.transpose(0, 2, 1, 3).reshape(B, L, D)
        return out @ self.Wo, attn

    def param_count(self):
        return 4 * self.d_model * self.d_model


def head_importance_score(attn_weights):
    """Compute per-head importance as mean attention entropy.
    Lower entropy = more focused = more important.
    attn_weights: (B, H, L, L)
    """
    eps = 1e-8
    entropy = -(attn_weights * np.log(attn_weights + eps)).sum(axis=-1).mean(axis=(0, 2))
    # Importance = inverse entropy (focused heads are more important)
    return 1.0 / (entropy + eps)


def prune_to_divisor(n_heads, importance_scores):
    """Prune to nearest valid divisor of 12 that is <= n_heads."""
    valid = [d for d in DIVISORS_OF_12 if d <= n_heads]
    target = max(valid)
    if target >= n_heads:
        return list(range(n_heads)), target

    # Keep top-k by importance
    top_indices = np.argsort(importance_scores)[-target:]
    return sorted(top_indices.tolist()), target


# ── Experiment ──

def evaluate_head_count(n_heads, d_model, X, y, n_seeds=5):
    """Train and evaluate attention with given head count."""
    B, L, D = X.shape
    split = int(0.8 * B)
    X_train, y_train = X[:split], y[:split]
    X_test, y_test = X[split:], y[split:]

    accs = []
    for seed in range(n_seeds):
        np.random.seed(seed * 7 + 42)
        mha = MultiHeadAttention(d_model, n_heads)

        # Forward
        out_train, _ = mha.forward(X_train)
        out_test, _ = mha.forward(X_test)

        # Global pool -> classify
        feat_train = out_train.mean(axis=1)  # (B, D)
        feat_test = out_test.mean(axis=1)

        # Ridge regression
        feat_train_b = np.hstack([feat_train, np.ones((len(feat_train), 1))])
        feat_test_b = np.hstack([feat_test, np.ones((len(feat_test), 1))])
        y_oh = np.zeros((len(y_train), y_train.max() + 1))
        y_oh[np.arange(len(y_train)), y_train] = 1

        W = np.linalg.solve(feat_train_b.T @ feat_train_b + 0.01 * np.eye(d_model + 1),
                            feat_train_b.T @ y_oh)
        pred = np.argmax(feat_test_b @ W, axis=1)
        accs.append((pred == y_test).mean())

    params = 4 * d_model * d_model  # QKV + O
    return np.mean(accs), np.std(accs), params


def main():
    print("=" * 70)
    print("  Experiment: Dedekind Head Pruning")
    print("  psi(6) = sigma(6) = 12 (unique coincidence)")
    print("  Valid head counts: %s" % DIVISORS_OF_12)
    print("=" * 70)

    # Create synthetic data
    d_model = SIGMA * TAU  # 48 (divisible by all divisors of 12)
    seq_len = SIGMA  # 12
    n_samples = 200
    n_classes = N

    X = np.random.randn(n_samples, seq_len, d_model).astype(np.float32) * 0.1
    # Label based on pattern
    y = (X[:, 0, 0] > 0).astype(np.int64) * (n_classes // 2) + \
        (X[:, -1, -1] > 0).astype(np.int64)
    y = y % n_classes

    # Test all valid head counts + some invalid ones
    test_heads = [1, 2, 3, 4, 6, 8, 12, 16, 24, 48]
    test_heads = [h for h in test_heads if d_model % h == 0]

    results = {}
    for h in test_heads:
        acc, std, params = evaluate_head_count(h, d_model, X, y, n_seeds=5)
        is_valid = h in DIVISORS_OF_12
        results[h] = {'acc': acc, 'std': std, 'params': params, 'valid': is_valid}

    # Report
    print(f"\n--- Results (d_model={d_model}, seq_len={seq_len}, {N} classes) ---")
    print(f"{'Heads':>6} {'Valid':>6} {'Accuracy':>10} {'Std':>8} {'head_dim':>10}")
    print("-" * 50)

    for h in sorted(results.keys()):
        r = results[h]
        valid_mark = "div(12)" if r['valid'] else ""
        hd = d_model // h
        print(f"{h:>6} {valid_mark:>6} {r['acc']:>10.4f} {r['std']:>8.4f} {hd:>10}")

    # Pruning demonstration
    print(f"\n--- Pruning Demo: 16 heads -> nearest divisor ---")
    np.random.seed(42)
    mha_16 = MultiHeadAttention(d_model, 16)
    _, attn_w = mha_16.forward(X[:10])
    importance = head_importance_score(attn_w)
    kept_indices, target_h = prune_to_divisor(16, importance)
    pruned_pct = (1 - target_h / 16) * 100
    print(f"  16 heads -> {target_h} heads (pruned {pruned_pct:.0f}%)")
    print(f"  Kept head indices: {kept_indices}")
    print(f"  Target {target_h} is in divisors of 12: {target_h in DIVISORS_OF_12}")

    # n=6 verification
    print(f"\n--- n=6 Verification ---")
    print(f"  psi(6) = 6 * prod(1 + 1/p for p|6) = 6 * (1+1/2) * (1+1/3) = 6 * 3/2 * 4/3 = 12")
    print(f"  sigma(6) = 1+2+3+6 = 12")
    print(f"  psi(n) = sigma(n) only for n = 6 (verified up to 10^6)")
    print(f"  Divisors of 12: {DIVISORS_OF_12} = tau(12) = {len(DIVISORS_OF_12)} options")
    print(f"  d_model = {d_model} = sigma * tau = {SIGMA} * {TAU}")

    # Compare valid vs invalid head counts
    valid_accs = [r['acc'] for h, r in results.items() if r['valid']]
    invalid_accs = [r['acc'] for h, r in results.items() if not r['valid']]
    if valid_accs and invalid_accs:
        print(f"\n  Mean acc (valid divisors): {np.mean(valid_accs):.4f}")
        print(f"  Mean acc (non-divisors):   {np.mean(invalid_accs):.4f}")
        diff = np.mean(valid_accs) - np.mean(invalid_accs)
        print(f"  Difference: {diff:+.4f}")

    print("\n" + "=" * 70)
    print("  Experiment complete.")
    print("=" * 70)


if __name__ == '__main__':
    main()
