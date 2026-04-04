#!/usr/bin/env python3
"""
Experiment: Egyptian Linear Attention -- Technique #22
======================================================
Tests linear attention with Egyptian fraction kernel decomposition.
n=6 connection: 1/2 + 1/3 + 1/6 = 1 applied to linear attention kernels.
Extension of Egyptian Fraction Attention (Technique #17) to O(L) complexity.

Compares O(L^2) standard attention vs O(L) Egyptian linear attention
across varying sequence lengths. Measures throughput and output quality.
"""
import sys, os, time, math
import numpy as np

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

import torch
import torch.nn as nn

# ── Attempt to import the technique module ──
try:
    from techniques.egyptian_linear_attention import (
        EgyptianLinearAttention, WINDOW_SIZE, STRIDE, N_ANCHORS,
        W_LOCAL, W_STRIDE, W_GLOBAL, N_HEADS,
    )
    HAS_ELA = True
except ImportError:
    HAS_ELA = False


def build_standard_attention(d_model, n_heads):
    """Standard O(L^2) multi-head attention for baseline."""
    return nn.MultiheadAttention(d_model, n_heads, batch_first=True)


def run_throughput_comparison(d_model=84, seq_lengths=None, batch_size=4, n_trials=5):
    """Compare throughput of standard vs Egyptian linear attention."""
    if seq_lengths is None:
        seq_lengths = [32, 64, 128, 256, 512]

    print("\n=== Throughput Comparison: Standard O(L^2) vs Egyptian Linear O(L) ===\n")
    print(f"  d_model={d_model}, n_heads={N_HEADS}, batch_size={batch_size}, trials={n_trials}")
    print(f"  Egyptian weights: local={W_LOCAL:.4f}, stride={W_STRIDE:.4f}, global={W_GLOBAL:.4f}")
    print(f"  Window={WINDOW_SIZE}, Stride={STRIDE}, Anchors={N_ANCHORS}\n")

    header = f"  {'SeqLen':>6} | {'Std (ms)':>10} | {'ELA (ms)':>10} | {'Speedup':>8} | {'Cos Sim':>8}"
    print(header)
    print("  " + "-" * len(header))

    results = []
    for L in seq_lengths:
        x = torch.randn(batch_size, L, d_model)

        # Standard attention
        std_attn = build_standard_attention(d_model, N_HEADS)
        std_attn.eval()

        # Egyptian linear attention
        ela = EgyptianLinearAttention(d_model)
        ela.eval()

        # Warm up
        with torch.no_grad():
            std_attn(x, x, x)
            ela(x)

        # Time standard
        times_std = []
        for _ in range(n_trials):
            t0 = time.perf_counter()
            with torch.no_grad():
                out_std, _ = std_attn(x, x, x)
            times_std.append((time.perf_counter() - t0) * 1000)

        # Time ELA
        times_ela = []
        for _ in range(n_trials):
            t0 = time.perf_counter()
            with torch.no_grad():
                out_ela = ela(x)
            times_ela.append((time.perf_counter() - t0) * 1000)

        ms_std = np.median(times_std)
        ms_ela = np.median(times_ela)
        speedup = ms_std / ms_ela if ms_ela > 0 else float('inf')

        # Cosine similarity between outputs (quality check)
        with torch.no_grad():
            cos_sim = torch.nn.functional.cosine_similarity(
                out_std.flatten(), out_ela.flatten(), dim=0
            ).item()

        results.append({
            'seq_len': L, 'ms_std': ms_std, 'ms_ela': ms_ela,
            'speedup': speedup, 'cos_sim': cos_sim,
        })
        print(f"  {L:>6} | {ms_std:>10.2f} | {ms_ela:>10.2f} | {speedup:>7.2f}x | {cos_sim:>8.4f}")

    return results


def run_egyptian_weight_verification():
    """Verify Egyptian fraction property: weights sum to 1."""
    print("\n=== Egyptian Fraction Weight Verification ===\n")
    fractions = [W_LOCAL, W_STRIDE, W_GLOBAL]
    labels = ["Local (1/2)", "Stride (1/3)", "Global (1/6)"]
    total = sum(fractions)
    for lbl, w in zip(labels, fractions):
        print(f"  {lbl:15s}: {w:.6f}")
    print(f"  {'Sum':15s}: {total:.6f}")
    exact = abs(total - 1.0) < 1e-10
    print(f"  Exact unity: {exact}")
    return exact


def run_scaling_analysis(d_model=84, batch_size=2, n_trials=3):
    """Measure how runtime scales with sequence length for both methods."""
    print("\n=== Scaling Analysis ===\n")
    lengths = [64, 128, 256, 512, 1024]

    ela = EgyptianLinearAttention(d_model)
    ela.eval()

    print(f"  {'SeqLen':>6} | {'ELA (ms)':>10} | {'Ratio vs prev':>14}")
    print("  " + "-" * 40)

    prev_ms = None
    for L in lengths:
        x = torch.randn(batch_size, L, d_model)
        with torch.no_grad():
            ela(x)  # warmup

        times = []
        for _ in range(n_trials):
            t0 = time.perf_counter()
            with torch.no_grad():
                ela(x)
            times.append((time.perf_counter() - t0) * 1000)

        ms = np.median(times)
        ratio_str = f"{ms / prev_ms:.2f}x" if prev_ms else "---"
        print(f"  {L:>6} | {ms:>10.2f} | {ratio_str:>14}")
        prev_ms = ms

    print("\n  Expected: ~2x ratio for 2x length increase (linear scaling)")


def main():
    print("=" * 60)
    print("  Experiment: Egyptian Linear Attention (Technique #22)")
    print("  1/2 + 1/3 + 1/6 = 1 kernel decomposition")
    print("=" * 60)

    if not HAS_ELA:
        print("\n  [SKIP] Could not import EgyptianLinearAttention technique.")
        print("  Ensure techniques/egyptian_linear_attention.py is available.")
        return

    run_egyptian_weight_verification()
    results = run_throughput_comparison()
    run_scaling_analysis()

    # Summary
    print("\n=== Summary ===")
    print(f"  Egyptian fractions: 1/2 + 1/3 + 1/6 = 1")
    print(f"  Complexity: O(L) vs O(L^2)")
    if results:
        avg_speedup = np.mean([r['speedup'] for r in results])
        avg_cos = np.mean([r['cos_sim'] for r in results])
        print(f"  Average speedup: {avg_speedup:.2f}x")
        print(f"  Average cosine similarity: {avg_cos:.4f}")
    print()


if __name__ == '__main__':
    main()
