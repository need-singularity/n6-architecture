#!/usr/bin/env python3
"""
Experiment: Constant Time Stride -- Technique #21
=================================================
Tests constant-time strided access patterns for cache efficiency.
n=6 connection: Stride by divisors of 6 for cache-aligned access.
Compares sigma=12 fixed attention positions vs standard O(L^2) attention.
Measures throughput and quality across sequence lengths.
"""
import sys, os, time, math
import numpy as np

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

import torch
import torch.nn as nn
import torch.nn.functional as F

try:
    from techniques.constant_time_stride import (
        ConstantTimeStrideAttention, SIGMA, N_LOCAL, N_STRIDE, N_GLOBAL,
        STRIDE_STEP, W_LOCAL, W_STRIDE, W_GLOBAL, N, TAU, PHI, SOPFR,
    )
    HAS_CTSA = True
except ImportError:
    HAS_CTSA = False

torch.manual_seed(42)
np.random.seed(42)


def measure_memory_access_patterns():
    """Benchmark cache-aligned vs misaligned access in numpy."""
    print("\n=== Memory Access Pattern Benchmark ===\n")

    sizes = [1024, 4096, 16384, 65536, 262144]
    divisors_of_6 = [1, 2, 3, 6]
    non_divisors = [5, 7, 11]
    all_strides = divisors_of_6 + non_divisors

    print(f"  Divisors of 6: {divisors_of_6}")
    print(f"  Non-divisors:  {non_divisors}")
    print()

    header = f"  {'Size':>8} |"
    for s in all_strides:
        header += f" s={s:>2} (ms) |"
    print(header)
    print("  " + "-" * (10 + 12 * len(all_strides)))

    for size in sizes:
        arr = np.random.randn(size).astype(np.float32)
        row = f"  {size:>8} |"
        for stride in all_strides:
            # Strided sum access pattern
            n_trials = 200
            t0 = time.perf_counter()
            for _ in range(n_trials):
                _ = arr[::stride].sum()
            elapsed = (time.perf_counter() - t0) / n_trials * 1000
            row += f" {elapsed:>9.4f} |"
        print(row)

    print("\n  Note: Divisor-aligned strides may show better cache performance")


def measure_attention_throughput():
    """Compare constant-time stride attention vs standard attention."""
    if not HAS_CTSA:
        print("\n  [SKIP] ConstantTimeStrideAttention not available.")
        return []

    print(f"\n=== Attention Throughput: O(1)/query vs O(L)/query ===\n")
    print(f"  CTSA config: sigma={SIGMA} positions per query")
    print(f"    Local:  {N_LOCAL} positions (weight={W_LOCAL:.4f})")
    print(f"    Stride: {N_STRIDE} positions, step={STRIDE_STEP} (weight={W_STRIDE:.4f})")
    print(f"    Global: {N_GLOBAL} positions (weight={W_GLOBAL:.4f})")
    print(f"    Total:  {N_LOCAL}+{N_STRIDE}+{N_GLOBAL} = {SIGMA}")
    print()

    d_model = 64
    n_heads = 4
    batch_size = 4
    n_trials = 5
    seq_lengths = [32, 64, 128, 256, 512, 1024]

    std_attn = nn.MultiheadAttention(d_model, n_heads, batch_first=True)
    std_attn.eval()

    ctsa = ConstantTimeStrideAttention(d_model, n_heads)
    ctsa.eval()

    print(f"  {'SeqLen':>6} | {'Std (ms)':>10} | {'CTSA (ms)':>10} | {'Speedup':>8} | {'Positions':>10}")
    print("  " + "-" * 60)

    results = []
    for L in seq_lengths:
        x = torch.randn(batch_size, L, d_model)

        # Warmup
        with torch.no_grad():
            std_attn(x, x, x)
            ctsa(x)

        # Standard
        times_std = []
        for _ in range(n_trials):
            t0 = time.perf_counter()
            with torch.no_grad():
                std_attn(x, x, x)
            times_std.append((time.perf_counter() - t0) * 1000)

        # CTSA
        times_ctsa = []
        for _ in range(n_trials):
            t0 = time.perf_counter()
            with torch.no_grad():
                ctsa(x)
            times_ctsa.append((time.perf_counter() - t0) * 1000)

        ms_std = np.median(times_std)
        ms_ctsa = np.median(times_ctsa)
        speedup = ms_std / ms_ctsa if ms_ctsa > 0 else 0

        # Standard attends to L positions per query; CTSA attends to sigma=12
        positions_std = L
        positions_ctsa = SIGMA

        results.append({
            'seq_len': L, 'ms_std': ms_std, 'ms_ctsa': ms_ctsa,
            'speedup': speedup, 'pos_std': positions_std, 'pos_ctsa': positions_ctsa,
        })
        print(f"  {L:>6} | {ms_std:>10.2f} | {ms_ctsa:>10.2f} | {speedup:>7.2f}x | {positions_ctsa:>4}/{positions_std:>4}")

    return results


def analyze_complexity_scaling(results):
    """Analyze how time scales with sequence length."""
    if not results:
        return

    print(f"\n=== Complexity Scaling Analysis ===\n")
    print(f"  Standard attention: O(L^2) expected")
    print(f"  CTSA attention:     O(L)   expected (sigma=12 fixed positions)")
    print()

    # Check ratio of times when doubling L
    print(f"  {'L1->L2':>10} | {'Std ratio':>10} | {'CTSA ratio':>11} | {'Std theory':>11} | {'CTSA theory':>12}")
    print("  " + "-" * 60)

    for i in range(1, len(results)):
        r1 = results[i - 1]
        r2 = results[i]
        L_ratio = r2['seq_len'] / r1['seq_len']
        std_ratio = r2['ms_std'] / r1['ms_std'] if r1['ms_std'] > 0 else 0
        ctsa_ratio = r2['ms_ctsa'] / r1['ms_ctsa'] if r1['ms_ctsa'] > 0 else 0
        std_theory = L_ratio ** 2  # O(L^2)
        ctsa_theory = L_ratio  # O(L)

        print(f"  {r1['seq_len']:>4}->{r2['seq_len']:>4} | {std_ratio:>10.2f} | {ctsa_ratio:>11.2f} | "
              f"{std_theory:>11.2f} | {ctsa_theory:>12.2f}")


def main():
    print("=" * 60)
    print("  Experiment: Constant Time Stride (Technique #21)")
    print("  O(1) per query via sigma=12 fixed attention positions")
    print("  Divisors of 6: {1, 2, 3, 6} -> cache-aligned strides")
    print("=" * 60)

    # Part 1: Memory access patterns with divisor strides
    measure_memory_access_patterns()

    # Part 2: Attention throughput comparison
    results = measure_attention_throughput()

    # Part 3: Complexity scaling
    analyze_complexity_scaling(results)

    # Summary
    print(f"\n=== Summary ===")
    print(f"  Egyptian partition: {N_LOCAL}+{N_STRIDE}+{N_GLOBAL} = sigma = 12" if HAS_CTSA else "  CTSA module not loaded")
    print(f"  n=6 divisors {{1,2,3,6}} for cache-aligned stride patterns")
    print(f"  Constant sigma=12 positions per query regardless of L")
    print()


if __name__ == '__main__':
    main()
