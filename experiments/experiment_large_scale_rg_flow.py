"""
Experiment: Multi-Scale RG Flow Extrapolation
===============================================
Phase 4 prep: Does n=6 convergence persist at scale?
Train at {1K, 10K, 100K, 1M} params, measure:
  - FFN ratio convergence speed
  - Final convergence error
  - R-score trajectory
Extrapolate to 1B+ scale.
"""

import sys
sys.path.insert(0, '.')

import torch
import torch.nn as nn
import torch.nn.functional as F
import numpy as np
import math
import time

from engine.thermodynamic_frame import ArchitectureConfig

SEED = 42
torch.manual_seed(SEED)
np.random.seed(SEED)

TARGET_FFN_RATIO = 4.0 / 3.0


class Phi6Simple(nn.Module):
    def forward(self, x):
        xc = torch.clamp(x, -2.0, 2.0)
        return xc * xc - xc + 1.0


class AdaptiveFFN(nn.Module):
    def __init__(self, d_model, initial_ratio):
        super().__init__()
        self.d_model = d_model
        self.log_ratio = nn.Parameter(torch.tensor(math.log(initial_ratio)))
        self.act = Phi6Simple()
        max_d_ff = d_model * 4
        self.fc1 = nn.Linear(d_model, max_d_ff)
        self.fc2 = nn.Linear(max_d_ff, d_model)

    @property
    def ratio(self):
        return self.log_ratio.exp()

    def forward(self, x):
        d_ff = int(min(self.d_model * self.ratio.item(), self.fc1.out_features))
        d_ff = max(d_ff, 1)
        h = self.act(self.fc1(x))
        if d_ff < h.size(-1):
            mask = torch.zeros(h.size(-1), device=h.device)
            mask[:d_ff] = 1.0
            h = h * mask
        return self.fc2(h)


class Block(nn.Module):
    def __init__(self, d_model, n_heads, init_ratio):
        super().__init__()
        self.attn = nn.MultiheadAttention(d_model, n_heads, batch_first=True)
        self.ffn = AdaptiveFFN(d_model, init_ratio)
        self.ln1 = nn.LayerNorm(d_model)
        self.ln2 = nn.LayerNorm(d_model)

    def forward(self, x):
        L = x.size(1)
        mask = torch.triu(torch.ones(L, L, device=x.device), diagonal=1).bool()
        a, _ = self.attn(x, x, x, attn_mask=mask)
        x = self.ln1(x + a)
        x = self.ln2(x + self.ffn(x))
        return x


class Model(nn.Module):
    def __init__(self, vocab, d, heads, layers, seq, init_ratio):
        super().__init__()
        self.emb = nn.Embedding(vocab, d)
        self.pos = nn.Embedding(seq, d)
        self.blocks = nn.ModuleList([Block(d, heads, init_ratio) for _ in range(layers)])
        self.out = nn.Linear(d, vocab)

    def forward(self, idx):
        B, T = idx.shape
        x = self.emb(idx) + self.pos(torch.arange(T, device=idx.device))
        for b in self.blocks:
            x = b(x)
        return self.out(x)

    def get_mean_ratio(self):
        return np.mean([b.ffn.ratio.item() for b in self.blocks])

    def param_count(self):
        return sum(p.numel() for p in self.parameters())


def r_distance_loss(model):
    loss = torch.tensor(0.0)
    for b in model.blocks:
        loss = loss + (b.ffn.log_ratio - math.log(TARGET_FFN_RATIO)) ** 2
    return loss


def run_scale(d_model, n_heads, n_layers, data, vocab_size, steps=500):
    """Run one scale point."""
    SEQ, BATCH = 64, 16
    model = Model(vocab_size, d_model, n_heads, n_layers, SEQ, init_ratio=3.0)
    params = model.param_count()
    opt = torch.optim.Adam(model.parameters(), lr=3e-3)

    ratio_trajectory = []
    t0 = time.time()

    for step in range(steps):
        ix = torch.randint(0, len(data) - SEQ - 1, (BATCH,))
        x = torch.stack([data[i:i+SEQ] for i in ix])
        y = torch.stack([data[i+1:i+SEQ+1] for i in ix])

        logits = model(x)
        task_loss = F.cross_entropy(logits.reshape(-1, vocab_size), y.reshape(-1))
        beta = 0.5 * min(1.0, step / (steps * 0.2))
        total = task_loss + beta * r_distance_loss(model)

        opt.zero_grad()
        total.backward()
        nn.utils.clip_grad_norm_(model.parameters(), 1.0)
        opt.step()

        if step % 10 == 0:
            ratio_trajectory.append(model.get_mean_ratio())

    elapsed = time.time() - t0
    final_ratio = model.get_mean_ratio()
    error = abs(final_ratio - TARGET_FFN_RATIO) / TARGET_FFN_RATIO * 100

    # Convergence speed: steps to reach within 5% of target
    conv_step = steps
    for i, r in enumerate(ratio_trajectory):
        if abs(r - TARGET_FFN_RATIO) / TARGET_FFN_RATIO < 0.05:
            conv_step = i * 10
            break

    return {
        "d_model": d_model,
        "n_layers": n_layers,
        "params": params,
        "final_ratio": final_ratio,
        "error_pct": error,
        "conv_step": conv_step,
        "time": elapsed,
        "trajectory": ratio_trajectory,
    }


def main():
    print("=" * 70)
    print("  Multi-Scale RG Flow Extrapolation")
    print("  Does n=6 convergence persist at scale?")
    print("=" * 70)

    BASE_TEXT = (
        "Mathematics reveals deep structure. "
        "The number six is perfect because its divisors one two and three sum to itself. "
        "Neural networks learn patterns through gradient descent optimization. "
        "Transformers use attention mechanisms to process sequences efficiently. "
        "Consciousness emerges from the interplay of deficit plasticity and inhibition. "
    )
    TEXT = (BASE_TEXT + " ") * 500
    chars = sorted(set(TEXT))
    vocab_size = len(chars)
    c2i = {c: i for i, c in enumerate(chars)}
    data = torch.tensor([c2i[c] for c in TEXT], dtype=torch.long)

    # Scale configs: (d_model, n_heads, n_layers) → ~target param count
    scales = [
        ("~1K",   12,  2, 1),
        ("~10K",  24,  4, 2),
        ("~100K", 60,  6, 3),
        ("~500K", 120, 12, 4),
        ("~1M",   180, 12, 6),
    ]

    results = []
    print(f"\n{'Scale':<8} {'d':>5} {'L':>3} {'Params':>10} {'Final Ratio':>12} {'Error':>8} {'Conv Step':>10} {'Time':>7}")
    print("-" * 70)

    for label, d, h, layers in scales:
        torch.manual_seed(SEED)
        r = run_scale(d, h, layers, data, vocab_size, steps=500)
        results.append(r)
        print(f"{label:<8} {d:>5} {layers:>3} {r['params']:>10,} {r['final_ratio']:>12.4f} "
              f"{r['error_pct']:>7.2f}% {r['conv_step']:>10} {r['time']:>6.1f}s")

    # ─── Extrapolation ───
    print(f"\n--- Scale Extrapolation ---")
    log_params = [np.log10(r["params"]) for r in results]
    errors = [r["error_pct"] for r in results]
    conv_steps = [r["conv_step"] for r in results]

    # Linear fit: error vs log(params)
    if len(log_params) >= 3:
        slope_err, intercept_err = np.polyfit(log_params, errors, 1)
        # Extrapolate to 1B (log10(1e9) = 9)
        error_1B = slope_err * 9 + intercept_err

        slope_conv, intercept_conv = np.polyfit(log_params, conv_steps, 1)
        conv_1B = slope_conv * 9 + intercept_conv

        print(f"Error vs log(params): slope={slope_err:.2f}, intercept={intercept_err:.2f}")
        print(f"Extrapolated error at 1B params: {error_1B:.2f}%")
        print(f"Extrapolated convergence step at 1B: {conv_1B:.0f}")

        if slope_err > 0:
            print(f"\nWARNING: Error INCREASES with scale (slope={slope_err:.2f})")
            print(f"n=6 convergence may weaken at large scale (H-EE-98 risk)")
        elif slope_err < -0.5:
            print(f"\nSTRONG: Error DECREASES with scale (slope={slope_err:.2f})")
            print(f"n=6 convergence strengthens at scale — RG flow is robust")
        else:
            print(f"\nNEUTRAL: Error roughly constant across scales")
            print(f"n=6 convergence persists but doesn't strengthen")

    # ─── Verdict ───
    print(f"\n--- Verdict ---")
    all_converged = all(r["error_pct"] < 10 for r in results)
    print(f"All scales converged (<10% error): {all_converged}")
    print(f"Scale range tested: {results[0]['params']:,} to {results[-1]['params']:,}")
    print(f"Target: FFN ratio = {TARGET_FFN_RATIO:.4f} (4/3)")

    if all_converged and slope_err <= 0:
        print(f"\nPhase 4 READY: n=6 convergence persists across 3 orders of magnitude")
        print(f"Extrapolation suggests 1B+ will also converge")
    else:
        print(f"\nPhase 4 CAUTION: scale dependence detected, proceed with care")


if __name__ == "__main__":
    main()
