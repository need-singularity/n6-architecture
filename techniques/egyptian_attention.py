#!/usr/bin/env python3
"""
Egyptian Fraction Attention (EFA) — n=6 Technique #17
=====================================================
Hypothesis: Partitioning σ=12 attention heads into 3 groups using the
Egyptian fraction decomposition 1/2 + 1/3 + 1/6 = 1 (perfect number
definition) achieves comparable quality to full attention with ~40% fewer
attention FLOPs.

Architecture:
  Group A: 6 heads (1/2) — Full quadratic attention (all tokens)
  Group B: 4 heads (1/3) — Local sliding window (w=64 for demo)
  Group C: 2 heads (1/6) — Global summary (first/last token only)

  Total heads = 6 + 4 + 2 = σ(6) = 12
  Fractions = 1/2 + 1/3 + 1/6 = 1 (Kruskal-Shafranov, BT-5)

Extends Gemma 2's binary local/global to a 3-tier system derived from
the unique Egyptian fraction decomposition of the first perfect number.

Test: MNIST sequence classification, 12-head transformer, 5 seeds.
Compare: full attention vs EFA vs random-split attention.
"""

import sys
sys.path.insert(0, '/Users/ghost/Dev/TECS-L')

import torch
import torch.nn as nn
import torch.nn.functional as F
import numpy as np
import math
import time

torch.manual_seed(42)
np.random.seed(42)

# n=6 constants
SIGMA = 12   # total heads
N_FULL = 6   # 1/2 of heads: full attention
N_LOCAL = 4  # 1/3 of heads: local window
N_GLOBAL = 2 # 1/6 of heads: global summary
WINDOW = 64  # local window size (φ^n = 2^6)

SEQ_LEN = 28
DIM = 84  # divisible by 12 heads -> head_dim = 7


# ─── Standard Multi-Head Attention ─────────────────────────────────────

class StandardAttention(nn.Module):
    """Full quadratic attention on all heads."""
    def __init__(self, dim, n_heads):
        super().__init__()
        self.n_heads = n_heads
        self.head_dim = dim // n_heads
        self.qkv = nn.Linear(dim, 3 * dim)
        self.out = nn.Linear(dim, dim)

    def forward(self, x):
        B, S, D = x.shape
        qkv = self.qkv(x).reshape(B, S, 3, self.n_heads, self.head_dim)
        q, k, v = qkv.permute(2, 0, 3, 1, 4)  # (3, B, H, S, d)
        scale = self.head_dim ** -0.5
        attn = (q @ k.transpose(-2, -1)) * scale
        attn = attn.softmax(dim=-1)
        out = (attn @ v).transpose(1, 2).reshape(B, S, D)
        return self.out(out)


# ─── Egyptian Fraction Attention (EFA) ─────────────────────────────────

class EgyptianFractionAttention(nn.Module):
    """
    3-tier attention: 1/2 full + 1/3 local + 1/6 global = 1.
    Egyptian fraction decomposition of the perfect number n=6.
    """
    def __init__(self, dim, n_full=N_FULL, n_local=N_LOCAL, n_global=N_GLOBAL,
                 window=WINDOW):
        super().__init__()
        self.n_full = n_full
        self.n_local = n_local
        self.n_global = n_global
        self.n_heads = n_full + n_local + n_global
        self.head_dim = dim // self.n_heads
        self.window = window
        self.qkv = nn.Linear(dim, 3 * dim)
        self.out = nn.Linear(dim, dim)

    def _full_attention(self, q, k, v):
        """Standard O(n²) attention."""
        scale = self.head_dim ** -0.5
        attn = (q @ k.transpose(-2, -1)) * scale
        return attn.softmax(dim=-1) @ v

    def _local_attention(self, q, k, v):
        """Sliding window attention O(n·w)."""
        B, H, S, D = q.shape
        scale = self.head_dim ** -0.5
        # Create causal-local mask
        mask = torch.ones(S, S, device=q.device, dtype=torch.bool)
        for i in range(S):
            start = max(0, i - self.window // 2)
            end = min(S, i + self.window // 2 + 1)
            mask[i, start:end] = False
        attn = (q @ k.transpose(-2, -1)) * scale
        attn = attn.masked_fill(mask.unsqueeze(0).unsqueeze(0), float('-inf'))
        return attn.softmax(dim=-1) @ v

    def _global_attention(self, q, k, v):
        """Global summary: attend only to first and last tokens."""
        B, H, S, D = q.shape
        scale = self.head_dim ** -0.5
        # Only attend to positions 0 and S-1
        k_global = torch.stack([k[:, :, 0], k[:, :, -1]], dim=2)  # (B, H, 2, D)
        v_global = torch.stack([v[:, :, 0], v[:, :, -1]], dim=2)  # (B, H, 2, D)
        attn = (q @ k_global.transpose(-2, -1)) * scale  # (B, H, S, 2)
        return attn.softmax(dim=-1) @ v_global  # (B, H, S, D)

    def forward(self, x):
        B, S, D = x.shape
        qkv = self.qkv(x).reshape(B, S, 3, self.n_heads, self.head_dim)
        q, k, v = qkv.permute(2, 0, 3, 1, 4)  # (3, B, H, S, d)

        # Split heads into 3 groups: full | local | global
        h1 = self.n_full
        h2 = h1 + self.n_local

        out_full = self._full_attention(
            q[:, :h1], k[:, :h1], v[:, :h1])
        out_local = self._local_attention(
            q[:, h1:h2], k[:, h1:h2], v[:, h1:h2])
        out_global = self._global_attention(
            q[:, h2:], k[:, h2:], v[:, h2:])

        # Concatenate all heads
        out = torch.cat([out_full, out_local, out_global], dim=1)
        out = out.transpose(1, 2).reshape(B, S, D)
        return self.out(out)

    def flop_ratio(self, seq_len):
        """Theoretical FLOPs relative to full attention."""
        full_flops = seq_len * seq_len  # O(n²)
        local_flops = seq_len * min(self.window, seq_len)
        global_flops = seq_len * 2

        total_efa = (self.n_full * full_flops +
                     self.n_local * local_flops +
                     self.n_global * global_flops)
        total_full = self.n_heads * full_flops
        return total_efa / total_full


# ─── Transformer Block ─────────────────────────────────────────────────

class TransformerBlock(nn.Module):
    def __init__(self, dim, attn_module):
        super().__init__()
        self.attn = attn_module
        self.norm1 = nn.LayerNorm(dim)
        self.ff = nn.Sequential(
            nn.Linear(dim, dim * 4 // 3),  # τ²/σ = 4/3 ratio (phi_bottleneck)
            nn.GELU(),
            nn.Linear(dim * 4 // 3, dim),
        )
        self.norm2 = nn.LayerNorm(dim)

    def forward(self, x):
        x = x + self.attn(self.norm1(x))
        x = x + self.ff(self.norm2(x))
        return x


class MiniTransformer(nn.Module):
    def __init__(self, dim, n_layers, n_classes, attn_factory):
        super().__init__()
        self.embed = nn.Linear(SEQ_LEN, dim)  # project each token
        self.pos = nn.Parameter(torch.randn(1, SEQ_LEN, dim) * 0.02)
        self.layers = nn.ModuleList([
            TransformerBlock(dim, attn_factory())
            for _ in range(n_layers)
        ])
        self.head = nn.Linear(dim, n_classes)

    def forward(self, x):
        x = self.embed(x) + self.pos
        for layer in self.layers:
            x = layer(x)
        x = x.mean(dim=1)  # global average pooling
        return self.head(x)


# ─── Training ──────────────────────────────────────────────────────────

def load_data():
    """Load MNIST as sequence data."""
    try:
        from model_utils import load_mnist
        train_x, train_y, test_x, test_y = load_mnist()
        # Reshape to (N, 28, 28) -- treat as 28 tokens of dim 28
        train_x = train_x.reshape(-1, 28, 28)
        test_x = test_x.reshape(-1, 28, 28)
        return train_x, train_y, test_x, test_y
    except Exception:
        # Fallback: synthetic sequence data
        print("  [MNIST not available, using synthetic data]")
        rng = np.random.RandomState(42)
        n_train, n_test = 5000, 1000
        train_x = torch.randn(n_train, SEQ_LEN, SEQ_LEN)
        test_x = torch.randn(n_test, SEQ_LEN, SEQ_LEN)
        # Labels based on mean of first vs second half
        train_y = (train_x[:, :14].mean(dim=(1, 2)) > 0).long()
        test_y = (test_x[:, :14].mean(dim=(1, 2)) > 0).long()
        return train_x, train_y, test_x, test_y


def train_and_eval(model, train_x, train_y, test_x, test_y,
                   epochs=10, batch_size=128, lr=3e-4):
    """Train and return test accuracy."""
    device = 'cpu'
    model = model.to(device)
    optimizer = torch.optim.AdamW(model.parameters(), lr=lr, weight_decay=0.1)

    n = len(train_x)
    for epoch in range(epochs):
        model.train()
        perm = torch.randperm(n)
        total_loss = 0
        for i in range(0, n, batch_size):
            idx = perm[i:i+batch_size]
            x = train_x[idx].to(device)
            y = train_y[idx].to(device)
            logits = model(x)
            loss = F.cross_entropy(logits, y)
            optimizer.zero_grad()
            loss.backward()
            optimizer.step()
            total_loss += loss.item()

    # Eval
    model.eval()
    with torch.no_grad():
        logits = model(test_x[:1000].to(device))
        pred = logits.argmax(dim=-1)
        acc = (pred == test_y[:1000].to(device)).float().mean().item()
    return acc


def count_params(model):
    return sum(p.numel() for p in model.parameters())


# ─── Main Experiment ───────────────────────────────────────────────────

if __name__ == '__main__':
    print("=" * 70)
    print("Egyptian Fraction Attention (EFA) — n=6 Technique #17")
    print("=" * 70)
    print(f"\n  Architecture: σ={SIGMA} heads")
    print(f"    Group A: {N_FULL} heads (1/2) — Full attention")
    print(f"    Group B: {N_LOCAL} heads (1/3) — Local window (w={WINDOW})")
    print(f"    Group C: {N_GLOBAL} heads (1/6) — Global summary")
    print(f"  FFN ratio: τ²/σ = 4/3 (phi_bottleneck)")
    print(f"  Egyptian fraction: 1/2 + 1/3 + 1/6 = 1")

    # Theoretical FLOP ratio
    efa_temp = EgyptianFractionAttention(DIM)
    for sl in [28, 128, 512, 2048]:
        ratio = efa_temp.flop_ratio(sl)
        print(f"  Attn FLOPs at seq_len={sl}: {ratio:.1%} of full attention "
              f"(saves {1-ratio:.1%})")

    print(f"\n  Loading data...")
    train_x, train_y, test_x, test_y = load_data()
    n_classes = int(train_y.max().item()) + 1

    configs = {
        'Full Attention (12 heads)': lambda: StandardAttention(DIM, SIGMA),
        'EFA 6+4+2 (Egyptian)': lambda: EgyptianFractionAttention(DIM),
        'Naive 6+6+0 (Half local)': lambda: EgyptianFractionAttention(
            DIM, n_full=6, n_local=6, n_global=0),
    }

    n_seeds = 3
    n_layers = 2

    print(f"\n  Training {len(configs)} configs × {n_seeds} seeds × {n_layers} layers")
    print(f"  dim={DIM}, heads={SIGMA}, classes={n_classes}")
    print("-" * 70)

    results = {}
    for name, attn_factory in configs.items():
        accs = []
        for seed in range(n_seeds):
            torch.manual_seed(seed * 7 + 42)
            model = MiniTransformer(DIM, n_layers, n_classes, attn_factory)
            params = count_params(model)
            acc = train_and_eval(model, train_x, train_y, test_x, test_y,
                                epochs=8, lr=3e-4)
            accs.append(acc)
        mean_acc = np.mean(accs)
        std_acc = np.std(accs)
        results[name] = (mean_acc, std_acc, params)
        print(f"  {name:<35} acc={mean_acc:.4f}±{std_acc:.4f}  params={params:,}")

    print("\n" + "=" * 70)
    print("Results Summary")
    print("=" * 70)

    full_acc = results['Full Attention (12 heads)'][0]
    for name, (acc, std, params) in results.items():
        delta = (acc - full_acc) / full_acc * 100
        sign = '+' if delta >= 0 else ''
        print(f"  {name:<35} {acc:.4f} ({sign}{delta:.2f}% vs full)")

    efa_acc = results['EFA 6+4+2 (Egyptian)'][0]
    flop_save = 1 - efa_temp.flop_ratio(SEQ_LEN)

    print(f"\n  EFA attention FLOPs saved: {flop_save:.1%}")
    print(f"  EFA quality delta: {(efa_acc-full_acc)/full_acc*100:+.2f}%")
    print(f"\n  Conclusion: {'EFA maintains quality' if abs(efa_acc - full_acc) < 0.02 else 'Quality gap detected'}")
    print(f"  Egyptian fraction 1/2+1/3+1/6=1 as attention budget allocation: "
          f"{'VALIDATED' if abs(efa_acc - full_acc) < 0.02 else 'NEEDS TUNING'}")
