#!/usr/bin/env python3
"""
Experiment: Egyptian Fraction Attention (EFA) -- Technique #17
=============================================================
Partitions sigma=12 heads into 3 groups: 6 full + 4 local + 2 global.
1/2 + 1/3 + 1/6 = 1 (perfect number Egyptian fraction).

n=6 connection: 6 = 1+2+3 -> 1/2+1/3+1/6 = 1 defines the partition.
Expected: ~40% FLOPs saved with comparable accuracy.
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
J2 = 24

N_FULL = 6    # 1/2 of heads: full O(L^2) attention
N_LOCAL = 4   # 1/3 of heads: local window attention
N_GLOBAL = 2  # 1/6 of heads: global summary attention
WINDOW = 8    # local window size

RNG_SEED = 42
np.random.seed(RNG_SEED)


# ── Attention Implementations ──

def softmax_last(x):
    e = np.exp(x - x.max(axis=-1, keepdims=True))
    return e / e.sum(axis=-1, keepdims=True)


def full_attention(Q, K, V):
    """Standard O(L^2) attention."""
    d_k = Q.shape[-1]
    scores = np.matmul(Q, K.transpose(0, 1, 3, 2)) / np.sqrt(d_k)
    attn = softmax_last(scores)
    return np.matmul(attn, V)


def local_attention(Q, K, V, window=WINDOW):
    """Sliding window attention, O(L*W)."""
    B, H, L, d_k = Q.shape
    out = np.zeros_like(V)
    for i in range(L):
        start = max(0, i - window // 2)
        end = min(L, i + window // 2 + 1)
        q_i = Q[:, :, i:i+1, :]           # (B, H, 1, d_k)
        k_w = K[:, :, start:end, :]        # (B, H, W, d_k)
        v_w = V[:, :, start:end, :]        # (B, H, W, d_k)
        scores = np.matmul(q_i, k_w.transpose(0, 1, 3, 2)) / np.sqrt(d_k)
        attn = softmax_last(scores)
        out[:, :, i:i+1, :] = np.matmul(attn, v_w)
    return out


def global_attention(Q, K, V):
    """Global summary: attend only to first and last tokens."""
    B, H, L, d_k = Q.shape
    # Summary tokens: first + last
    k_g = np.concatenate([K[:, :, :1, :], K[:, :, -1:, :]], axis=2)  # (B, H, 2, d_k)
    v_g = np.concatenate([V[:, :, :1, :], V[:, :, -1:, :]], axis=2)
    scores = np.matmul(Q, k_g.transpose(0, 1, 3, 2)) / np.sqrt(d_k)
    attn = softmax_last(scores)
    return np.matmul(attn, v_g)


# ── EFA Multi-Head Attention ──

class EFAAttention:
    def __init__(self, d_model, n_full=N_FULL, n_local=N_LOCAL, n_global=N_GLOBAL):
        self.d_model = d_model
        self.n_heads = n_full + n_local + n_global
        self.n_full = n_full
        self.n_local = n_local
        self.n_global = n_global
        self.head_dim = d_model // self.n_heads

        scale = np.sqrt(2.0 / d_model)
        self.Wq = np.random.randn(d_model, d_model).astype(np.float32) * scale
        self.Wk = np.random.randn(d_model, d_model).astype(np.float32) * scale
        self.Wv = np.random.randn(d_model, d_model).astype(np.float32) * scale
        self.Wo = np.random.randn(d_model, d_model).astype(np.float32) * scale

    def forward(self, x):
        B, L, D = x.shape
        Q = (x @ self.Wq).reshape(B, L, self.n_heads, self.head_dim).transpose(0, 2, 1, 3)
        K = (x @ self.Wk).reshape(B, L, self.n_heads, self.head_dim).transpose(0, 2, 1, 3)
        V = (x @ self.Wv).reshape(B, L, self.n_heads, self.head_dim).transpose(0, 2, 1, 3)

        # Split heads into 3 groups
        h1 = self.n_full
        h2 = h1 + self.n_local

        out_full = full_attention(Q[:, :h1], K[:, :h1], V[:, :h1])
        out_local = local_attention(Q[:, h1:h2], K[:, h1:h2], V[:, h1:h2])
        out_global = global_attention(Q[:, h2:], K[:, h2:], V[:, h2:])

        out = np.concatenate([out_full, out_local, out_global], axis=1)
        out = out.transpose(0, 2, 1, 3).reshape(B, L, D)
        return out @ self.Wo


class FullAttention:
    def __init__(self, d_model, n_heads=SIGMA):
        self.d_model = d_model
        self.n_heads = n_heads
        self.head_dim = d_model // n_heads

        scale = np.sqrt(2.0 / d_model)
        self.Wq = np.random.randn(d_model, d_model).astype(np.float32) * scale
        self.Wk = np.random.randn(d_model, d_model).astype(np.float32) * scale
        self.Wv = np.random.randn(d_model, d_model).astype(np.float32) * scale
        self.Wo = np.random.randn(d_model, d_model).astype(np.float32) * scale

    def forward(self, x):
        B, L, D = x.shape
        Q = (x @ self.Wq).reshape(B, L, self.n_heads, self.head_dim).transpose(0, 2, 1, 3)
        K = (x @ self.Wk).reshape(B, L, self.n_heads, self.head_dim).transpose(0, 2, 1, 3)
        V = (x @ self.Wv).reshape(B, L, self.n_heads, self.head_dim).transpose(0, 2, 1, 3)
        out = full_attention(Q, K, V)
        out = out.transpose(0, 2, 1, 3).reshape(B, L, D)
        return out @ self.Wo


# ── FLOPs Estimation ──

def estimate_efa_flops(L, d_model, n_full, n_local, n_global, window=WINDOW):
    head_dim = d_model // (n_full + n_local + n_global)
    full_flops = n_full * (2 * L * L * head_dim)       # O(L^2)
    local_flops = n_local * (2 * L * window * head_dim)  # O(L*W)
    global_flops = n_global * (2 * L * 2 * head_dim)     # O(L*2)
    return full_flops + local_flops + global_flops

def estimate_full_flops(L, d_model, n_heads):
    head_dim = d_model // n_heads
    return n_heads * (2 * L * L * head_dim)


# ── Classify with Attention ──

def classify(attn_model, X, y, n_seeds=5):
    B, L, D = X.shape
    split = int(0.8 * B)

    accs, times = [], []
    for seed in range(n_seeds):
        np.random.seed(seed * 7 + 42)
        # Re-init weights
        scale = np.sqrt(2.0 / D)
        attn_model.Wq = np.random.randn(D, D).astype(np.float32) * scale
        attn_model.Wk = np.random.randn(D, D).astype(np.float32) * scale
        attn_model.Wv = np.random.randn(D, D).astype(np.float32) * scale
        attn_model.Wo = np.random.randn(D, D).astype(np.float32) * scale

        t0 = time.time()
        out = attn_model.forward(X)
        t1 = time.time()
        times.append(t1 - t0)

        feat = out.mean(axis=1)
        feat_b = np.hstack([feat, np.ones((B, 1))])
        y_oh = np.zeros((split, y.max() + 1))
        y_oh[np.arange(split), y[:split]] = 1

        W = np.linalg.solve(feat_b[:split].T @ feat_b[:split] + 0.01 * np.eye(D + 1),
                            feat_b[:split].T @ y_oh)
        pred = np.argmax(feat_b[split:] @ W, axis=1)
        accs.append((pred == y[split:]).mean())

    return np.mean(accs), np.std(accs), np.mean(times)


# ── Main ──

def main():
    print("=" * 70)
    print("  Experiment: Egyptian Fraction Attention (EFA)")
    print("  1/2 + 1/3 + 1/6 = 1 -> %d full + %d local + %d global = %d heads"
          % (N_FULL, N_LOCAL, N_GLOBAL, SIGMA))
    print("=" * 70)

    d_model = SIGMA * TAU  # 48 (divisible by 12)
    seq_len = J2            # 24
    n_samples = 200

    X = np.random.randn(n_samples, seq_len, d_model).astype(np.float32) * 0.1
    y = np.zeros(n_samples, dtype=np.int64)
    for i in range(n_samples):
        y[i] = int(X[i, :seq_len//2].mean() > X[i, seq_len//2:].mean())

    # Models
    efa = EFAAttention(d_model, N_FULL, N_LOCAL, N_GLOBAL)
    full = FullAttention(d_model, SIGMA)

    # Evaluate
    efa_acc, efa_std, efa_time = classify(efa, X, y)
    full_acc, full_std, full_time = classify(full, X, y)

    # FLOPs
    efa_flops = estimate_efa_flops(seq_len, d_model, N_FULL, N_LOCAL, N_GLOBAL)
    full_flops = estimate_full_flops(seq_len, d_model, SIGMA)
    flops_saved = (1 - efa_flops / full_flops) * 100

    # Report
    print(f"\n--- Results ---")
    print(f"{'Method':<20} {'Accuracy':>10} {'Std':>8} {'Time(s)':>10} {'FLOPs':>12}")
    print("-" * 65)
    print(f"{'Full (12 heads)':<20} {full_acc:>10.4f} {full_std:>8.4f} {full_time:>10.4f} {full_flops:>12,d}")
    print(f"{'EFA (6+4+2)':<20} {efa_acc:>10.4f} {efa_std:>8.4f} {efa_time:>10.4f} {efa_flops:>12,d}")
    print(f"\n  FLOPs saved: {flops_saved:.1f}%")
    print(f"  Speedup: {full_time / efa_time:.2f}x" if efa_time > 0 else "  Speedup: N/A")

    # n=6 verification
    print(f"\n--- n=6 Verification ---")
    print(f"  Head partition: {N_FULL} + {N_LOCAL} + {N_GLOBAL} = {SIGMA} = sigma(6)")
    print(f"  Fractions: {N_FULL}/{SIGMA} + {N_LOCAL}/{SIGMA} + {N_GLOBAL}/{SIGMA} "
          f"= 1/2 + 1/3 + 1/6 = 1")
    print(f"  6 = 1 + 2 + 3 (perfect number, sum of proper divisors)")
    print(f"  d_model = {d_model} = sigma * tau = {SIGMA} * {TAU}")
    print(f"  seq_len = {seq_len} = J2(6) = {J2}")
    print(f"  FLOPs saved ~ 40% target: {'PASS' if 30 < flops_saved < 60 else 'CHECK'}")

    acc_delta = efa_acc - full_acc
    print(f"\n  Accuracy delta (EFA - Full): {acc_delta:+.4f}")
    if abs(acc_delta) < 0.05:
        print("  PASS: Comparable accuracy with significant FLOPs reduction")

    print("\n" + "=" * 70)
    print("  Experiment complete.")
    print("=" * 70)


if __name__ == '__main__':
    main()
