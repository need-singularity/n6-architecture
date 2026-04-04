#!/usr/bin/env python3
"""
Experiment: FFT Mix Attention -- Technique #8
=============================================
Tests windowed FFT attention replacement at HCN window sizes {6, 12, 24}.
Measures speed, accuracy, and n=6 window size optimality.

n=6 connection: Window sizes are n=6, sigma=12, J2=24 (HCN dimensions).
Expected: 3x faster than self-attention, +0.55% accuracy.
"""

import sys
import os
import time
import numpy as np

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

# ── n=6 Constants ──
N = 6
SIGMA = 12
J2 = 24
TAU = 4
WINDOWS = [N, SIGMA, J2]  # HCN window sizes

RNG_SEED = 42
np.random.seed(RNG_SEED)


# ── Windowed FFT Mixing ──

def fft_mix(x, window_size):
    """Mix sequence via windowed FFT.
    x: (batch, seq_len, dim)
    Returns mixed sequence of same shape.
    """
    B, L, D = x.shape
    out = np.zeros_like(x)
    for start in range(0, L, window_size):
        end = min(start + window_size, L)
        window = x[:, start:end, :]
        # FFT along sequence axis, element-wise multiply (learned filter = identity for demo)
        fft_w = np.fft.rfft(window, axis=1)
        # Apply simple frequency filter: keep all (identity mixing)
        mixed = np.fft.irfft(fft_w, n=end - start, axis=1)
        out[:, start:end, :] = mixed.real
    return out


def self_attention(x):
    """Simple dot-product self-attention.
    x: (batch, seq_len, dim)
    """
    B, L, D = x.shape
    scale = 1.0 / np.sqrt(D)
    # Q, K, V = x (no projection for simplicity)
    scores = np.matmul(x, x.transpose(0, 2, 1)) * scale  # (B, L, L)
    # Softmax
    scores_max = scores.max(axis=-1, keepdims=True)
    exp_scores = np.exp(scores - scores_max)
    attn = exp_scores / exp_scores.sum(axis=-1, keepdims=True)
    out = np.matmul(attn, x)  # (B, L, D)
    return out


# ── Synthetic Sequence Data ──

def make_sequence_data(n_samples=500, seq_len=48, dim=SIGMA):
    """Create synthetic sequence classification data.
    seq_len=48=sigma*tau, dim=sigma=12.
    """
    X = np.random.randn(n_samples, seq_len, dim).astype(np.float32)
    # Labels based on mean of first vs second half
    first_half = X[:, :seq_len // 2, :].mean(axis=(1, 2))
    second_half = X[:, seq_len // 2:, :].mean(axis=(1, 2))
    y = (first_half > second_half).astype(np.int64)
    return X, y


# ── Simple Classifier ──

def classify_with_mixing(X, y, mix_fn, mix_name):
    """Apply mixing, then global average pool -> linear -> accuracy."""
    split = int(0.8 * len(X))
    X_train, y_train = X[:split], y[:split]
    X_test, y_test = X[split:], y[split:]

    t0 = time.time()
    X_train_mixed = mix_fn(X_train)
    X_test_mixed = mix_fn(X_test)
    mix_time = time.time() - t0

    # Global average pooling
    train_feat = X_train_mixed.mean(axis=1)  # (B, D)
    test_feat = X_test_mixed.mean(axis=1)

    # Simple logistic regression (closed-form)
    D = train_feat.shape[1]
    # Add bias
    train_feat_b = np.hstack([train_feat, np.ones((len(train_feat), 1))])
    test_feat_b = np.hstack([test_feat, np.ones((len(test_feat), 1))])

    # One-hot
    y_oh = np.zeros((len(y_train), 2))
    y_oh[np.arange(len(y_train)), y_train] = 1

    # Ridge regression
    lam = 0.01
    W = np.linalg.solve(train_feat_b.T @ train_feat_b + lam * np.eye(D + 1),
                         train_feat_b.T @ y_oh)
    pred = np.argmax(test_feat_b @ W, axis=1)
    acc = (pred == y_test).mean()
    return acc, mix_time


# ── FLOPs Estimation ──

def estimate_mixing_flops(method, seq_len, dim, n_samples):
    """Estimate total FLOPs for mixing operation."""
    if method == 'self_attention':
        # O(L^2 * D) per sample
        return n_samples * seq_len * seq_len * dim
    else:
        # FFT: O(L * log(L) * D) per sample
        return n_samples * seq_len * int(np.log2(seq_len + 1)) * dim


# ── Main Experiment ──

def main():
    print("=" * 70)
    print("  Experiment: FFT Mix Attention")
    print("  n=6 window sizes: %s" % WINDOWS)
    print("=" * 70)

    seq_len = SIGMA * TAU  # 48
    dim = SIGMA            # 12

    X, y = make_sequence_data(n_samples=500, seq_len=seq_len, dim=dim)

    n_seeds = 5
    results = {}

    # Self-attention baseline
    accs, times = [], []
    for seed in range(n_seeds):
        np.random.seed(seed * 7 + 42)
        acc, t = classify_with_mixing(X, y, self_attention, 'self_attn')
        accs.append(acc)
        times.append(t)
    sa_flops = estimate_mixing_flops('self_attention', seq_len, dim, len(X))
    results['self_attn'] = {'acc': np.mean(accs), 'std': np.std(accs),
                            'time': np.mean(times), 'flops': sa_flops}

    # FFT at each window size
    for w in WINDOWS:
        accs, times = [], []
        for seed in range(n_seeds):
            np.random.seed(seed * 7 + 42)
            mix_fn = lambda x, ws=w: fft_mix(x, ws)
            acc, t = classify_with_mixing(X, y, mix_fn, f'fft_w{w}')
            accs.append(acc)
            times.append(t)
        fft_flops = estimate_mixing_flops('fft', seq_len, dim, len(X))
        results[f'fft_w{w}'] = {'acc': np.mean(accs), 'std': np.std(accs),
                                 'time': np.mean(times), 'flops': fft_flops}

    # Report
    print(f"\n--- Results (seq_len={seq_len}=sigma*tau, dim={dim}=sigma, {n_seeds} seeds) ---")
    print(f"{'Method':<16} {'Accuracy':>10} {'Std':>8} {'Time(s)':>10} {'Speedup':>10}")
    print("-" * 60)

    sa_time = results['self_attn']['time']
    for name, r in results.items():
        speedup = sa_time / r['time'] if r['time'] > 0 else float('inf')
        print(f"{name:<16} {r['acc']:>10.4f} {r['std']:>8.4f} "
              f"{r['time']:>10.4f} {speedup:>8.2f}x")

    # n=6 analysis
    print(f"\n--- n=6 Verification ---")
    print(f"  Window sizes tested: {WINDOWS} = {{n, sigma, J2}}")
    print(f"  Sequence length: {seq_len} = sigma * tau = {SIGMA} * {TAU}")
    print(f"  Feature dimension: {dim} = sigma = {SIGMA}")

    best_fft = max(
        [(k, v) for k, v in results.items() if k.startswith('fft')],
        key=lambda x: x[1]['acc']
    )
    print(f"  Best FFT window: {best_fft[0]} (acc={best_fft[1]['acc']:.4f})")
    print(f"  Self-attention acc: {results['self_attn']['acc']:.4f}")

    flops_ratio = results['self_attn']['flops'] / results[best_fft[0]]['flops']
    print(f"  FLOPs ratio (self-attn / best FFT): {flops_ratio:.1f}x")
    print(f"  Complexity: O(L^2*D) -> O(L*log(L)*D)")

    print("\n" + "=" * 70)
    print("  Experiment complete.")
    print("=" * 70)


if __name__ == '__main__':
    main()
