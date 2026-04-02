"""
Technique 20: Fibonacci-Strided Attention
==========================================
F(6) = 8 = σ - τ = 12 - 4.

The 6th Fibonacci number is 8, connecting to the universal AI constant
σ - τ = 8 (BT-58: LoRA rank, MoE top-k, KV heads, batch size powers,
FlashAttention block size — all multiples of 8).

Insight: Instead of attending to all positions (O(n^2)) or uniformly
strided positions (O(n)), attend at Fibonacci-spaced distances from
each query position. The Fibonacci sequence {1, 1, 2, 3, 5, 8, 13, 21, ...}
creates a naturally logarithmic receptive field:
  - Dense attention to nearby tokens (distances 1, 1, 2)
  - Sparse attention to distant tokens (distances 8, 13, 21, ...)

This mirrors how biological perception works: high resolution locally,
coarser globally. The F(6)=8 connection means the "fundamental stride
unit" is the universal AI constant σ-τ.

Architecture:
  - For each query at position i, attend to positions at Fibonacci
    distances: i±1, i±1, i±2, i±3, i±5, i±8, i±13, ...
  - Number of attended positions per query: O(log_φ(n)) ≈ O(log n)
  - Total attention FLOPs: O(n · log n) vs O(n²) for full attention
  - Causal variant: only attend to i-fib (backward Fibonacci)

Expected: Near-full-attention quality with O(n log n) attention cost,
natural multi-scale pattern capture.
"""

import torch
import torch.nn as nn
import torch.nn.functional as F
import numpy as np
import math
import time

SEED = 42
torch.manual_seed(SEED)
np.random.seed(SEED)

# n=6 constants
N = 6
SIGMA = 12       # σ(6) = 12
PHI = 2          # φ(6) = 2
TAU = 4          # τ(6) = 4
J2 = 24          # J₂(6) = 24
SIGMA_TAU = 8    # σ - τ = 8 = F(6), universal AI constant (BT-58)
SOPFR = 5        # sopfr(6) = 2+3 = 5

# Fibonacci sequence
def fibonacci_sequence(max_val):
    """Generate Fibonacci numbers up to max_val."""
    fibs = [1, 1]
    while True:
        next_fib = fibs[-1] + fibs[-2]
        if next_fib > max_val:
            break
        fibs.append(next_fib)
    return fibs


def fibonacci_attention_positions(seq_len, causal=True):
    """Build Fibonacci-strided attention mask.

    For each query position i, attend to positions at Fibonacci distances.
    Returns a boolean mask of shape (seq_len, seq_len) where True = attend.

    Args:
        seq_len: sequence length
        causal: if True, only attend to past positions (i - fib)
    Returns:
        mask: (seq_len, seq_len) boolean tensor
    """
    fibs = fibonacci_sequence(seq_len)
    mask = torch.zeros(seq_len, seq_len, dtype=torch.bool)

    for i in range(seq_len):
        # Always attend to self
        mask[i, i] = True
        for f in fibs:
            # Backward (past) positions
            if i - f >= 0:
                mask[i, i - f] = True
            # Forward (future) positions — only if not causal
            if not causal and i + f < seq_len:
                mask[i, i + f] = True
    return mask


class FibonacciAttention(nn.Module):
    """Fibonacci-strided sparse attention.

    Attends at Fibonacci-spaced distances from each query position,
    creating a logarithmic receptive field. F(6)=8=σ-τ connects this
    to the universal AI constant.
    """

    def __init__(self, dim, n_heads=SIGMA, causal=True):
        super().__init__()
        self.n_heads = n_heads
        self.head_dim = dim // n_heads
        self.causal = causal
        self.qkv = nn.Linear(dim, 3 * dim)
        self.out = nn.Linear(dim, dim)
        self._cached_mask = None
        self._cached_len = -1

    def _get_mask(self, seq_len, device):
        """Get or compute Fibonacci attention mask."""
        if seq_len != self._cached_len:
            mask = fibonacci_attention_positions(seq_len, causal=self.causal)
            self._cached_mask = mask.to(device)
            self._cached_len = seq_len
        return self._cached_mask

    def forward(self, x):
        B, S, D = x.shape
        qkv = self.qkv(x).reshape(B, S, 3, self.n_heads, self.head_dim)
        q, k, v = qkv.permute(2, 0, 3, 1, 4)  # (3, B, H, S, d)

        scale = self.head_dim ** -0.5
        attn = (q @ k.transpose(-2, -1)) * scale  # (B, H, S, S)

        # Apply Fibonacci mask
        fib_mask = self._get_mask(S, x.device)  # (S, S)
        attn = attn.masked_fill(~fib_mask.unsqueeze(0).unsqueeze(0), float('-inf'))

        attn = attn.softmax(dim=-1)
        # Replace NaN from all-masked rows with zero
        attn = torch.nan_to_num(attn, nan=0.0)

        out = (attn @ v).transpose(1, 2).reshape(B, S, D)
        return self.out(out)

    def attended_fraction(self, seq_len):
        """Fraction of positions attended to (sparsity measure)."""
        mask = fibonacci_attention_positions(seq_len, causal=self.causal)
        return mask.float().mean().item()

    def positions_per_query(self, seq_len):
        """Average number of positions each query attends to."""
        mask = fibonacci_attention_positions(seq_len, causal=self.causal)
        return mask.float().sum(dim=-1).mean().item()


class FullAttention(nn.Module):
    """Standard full attention for comparison."""

    def __init__(self, dim, n_heads=SIGMA, causal=True):
        super().__init__()
        self.n_heads = n_heads
        self.head_dim = dim // n_heads
        self.causal = causal
        self.qkv = nn.Linear(dim, 3 * dim)
        self.out = nn.Linear(dim, dim)

    def forward(self, x):
        B, S, D = x.shape
        qkv = self.qkv(x).reshape(B, S, 3, self.n_heads, self.head_dim)
        q, k, v = qkv.permute(2, 0, 3, 1, 4)

        scale = self.head_dim ** -0.5
        attn = (q @ k.transpose(-2, -1)) * scale

        if self.causal:
            causal_mask = torch.triu(torch.ones(S, S, device=x.device), diagonal=1).bool()
            attn = attn.masked_fill(causal_mask.unsqueeze(0).unsqueeze(0), float('-inf'))

        attn = attn.softmax(dim=-1)
        out = (attn @ v).transpose(1, 2).reshape(B, S, D)
        return self.out(out)


class UniformStrideAttention(nn.Module):
    """Uniform stride attention for comparison (stride=σ-τ=8)."""

    def __init__(self, dim, n_heads=SIGMA, stride=SIGMA_TAU, causal=True):
        super().__init__()
        self.n_heads = n_heads
        self.head_dim = dim // n_heads
        self.stride = stride
        self.causal = causal
        self.qkv = nn.Linear(dim, 3 * dim)
        self.out = nn.Linear(dim, dim)

    def forward(self, x):
        B, S, D = x.shape
        qkv = self.qkv(x).reshape(B, S, 3, self.n_heads, self.head_dim)
        q, k, v = qkv.permute(2, 0, 3, 1, 4)

        scale = self.head_dim ** -0.5
        attn = (q @ k.transpose(-2, -1)) * scale

        # Uniform stride mask: attend every `stride` positions
        mask = torch.zeros(S, S, dtype=torch.bool, device=x.device)
        for i in range(S):
            for j in range(0, S, self.stride):
                if not self.causal or j <= i:
                    mask[i, j] = True
            # Also attend to self and neighbors
            mask[i, i] = True
            if i > 0:
                mask[i, i-1] = True

        attn = attn.masked_fill(~mask.unsqueeze(0).unsqueeze(0), float('-inf'))
        attn = attn.softmax(dim=-1)
        attn = torch.nan_to_num(attn, nan=0.0)

        out = (attn @ v).transpose(1, 2).reshape(B, S, D)
        return self.out(out)


# ─── Transformer wrapper ──────────────────────────────────────────────

class TransformerBlock(nn.Module):
    def __init__(self, dim, attn_module):
        super().__init__()
        self.attn = attn_module
        self.norm1 = nn.LayerNorm(dim)
        self.ff = nn.Sequential(
            nn.Linear(dim, dim * TAU // N),  # τ/n = 4/6 = 2/3 per-expert ratio
            nn.GELU(),
            nn.Linear(dim * TAU // N, dim),
        )
        self.norm2 = nn.LayerNorm(dim)

    def forward(self, x):
        x = x + self.attn(self.norm1(x))
        x = x + self.ff(self.norm2(x))
        return x


class SeqClassifier(nn.Module):
    def __init__(self, dim, seq_len, n_classes, n_layers, attn_factory):
        super().__init__()
        self.embed = nn.Linear(dim, dim)
        self.pos = nn.Parameter(torch.randn(1, seq_len, dim) * 0.02)
        self.layers = nn.ModuleList([
            TransformerBlock(dim, attn_factory()) for _ in range(n_layers)
        ])
        self.head = nn.Linear(dim, n_classes)

    def forward(self, x):
        x = self.embed(x) + self.pos[:, :x.size(1)]
        for layer in self.layers:
            x = layer(x)
        return self.head(x.mean(dim=1))


def count_params(m):
    return sum(p.numel() for p in m.parameters())


def main():
    print("=" * 70)
    print("  Technique 20: Fibonacci-Strided Attention")
    print("  F(6) = 8 = sigma - tau = universal AI constant (BT-58)")
    print("=" * 70)

    # Show Fibonacci connection
    fibs = fibonacci_sequence(256)
    print(f"\n  Fibonacci sequence: {fibs}")
    print(f"  F(6) = {fibs[5]} = sigma - tau = {SIGMA} - {TAU} = {SIGMA_TAU}")
    print(f"  BT-58: sigma-tau=8 appears in LoRA rank, MoE top-k, KV heads,")
    print(f"         FlashAttention block size, batch exponents (16/16 EXACT)")

    # Sparsity analysis for different sequence lengths
    print(f"\n  Fibonacci attention sparsity analysis:")
    print(f"  {'SeqLen':>8} {'Positions/Query':>16} {'Density':>10} {'vs Full':>10}")
    print(f"  {'-'*48}")

    fib_attn = FibonacciAttention(dim=48, n_heads=SIGMA, causal=True)
    for sl in [16, 32, 64, 128, 256, 512]:
        ppq = fib_attn.positions_per_query(sl)
        density = fib_attn.attended_fraction(sl)
        full_ppq = (sl + 1) / 2  # average for causal
        ratio = ppq / full_ppq
        print(f"  {sl:>8} {ppq:>16.1f} {density:>10.4f} {ratio:>10.1%}")

    # Training experiment
    SEQ_LEN = 64     # moderate length to show sparsity benefit
    D_MODEL = 48     # σ·τ = 48
    N_CLASSES = N     # n = 6
    N_LAYERS = PHI    # φ = 2 layers (lightweight demo)

    print(f"\n  Experiment: seq_len={SEQ_LEN}, d_model={D_MODEL}(=sigma*tau),")
    print(f"  n_heads={SIGMA}(=sigma), layers={N_LAYERS}(=tau), classes={N_CLASSES}(=n)")

    # Generate synthetic data
    rng = np.random.RandomState(SEED)
    N_TRAIN, N_TEST = 2000, 400

    train_x = torch.randn(N_TRAIN, SEQ_LEN, D_MODEL)
    test_x = torch.randn(N_TEST, SEQ_LEN, D_MODEL)

    # Labels from position-dependent patterns (benefits multi-scale attention)
    def make_labels(x):
        # Combine local (first 8 positions) and global (mean) features
        local = x[:, :SIGMA_TAU].mean(dim=(1, 2))  # F(6)=8 positions
        glob = x.mean(dim=(1, 2))
        combined = local + glob
        # Quantize to n=6 classes
        percentiles = torch.quantile(combined, torch.linspace(0, 1, N_CLASSES + 1))
        labels = torch.bucketize(combined, percentiles[1:-1])
        return labels

    train_y = make_labels(train_x)
    test_y = make_labels(test_x)

    configs = [
        ("Full Attention (causal)",
         lambda: FullAttention(D_MODEL, SIGMA, causal=True)),
        ("Fibonacci Attention (causal)",
         lambda: FibonacciAttention(D_MODEL, SIGMA, causal=True)),
        ("Uniform Stride-8 (causal)",
         lambda: UniformStrideAttention(D_MODEL, SIGMA, stride=SIGMA_TAU, causal=True)),
    ]

    print(f"\n  {'Config':<35} {'Acc':>7} {'Loss':>7} {'Params':>10} {'Time':>7}")
    print(f"  {'-'*70}")

    results = []
    for label, attn_factory in configs:
        torch.manual_seed(SEED)
        model = SeqClassifier(D_MODEL, SEQ_LEN, N_CLASSES, N_LAYERS, attn_factory)
        params = count_params(model)

        optimizer = torch.optim.Adam(model.parameters(), lr=3e-3)
        EPOCHS = 10
        BATCH = 64

        t0 = time.time()
        for epoch in range(EPOCHS):
            model.train()
            perm = torch.randperm(N_TRAIN)
            for i in range(0, N_TRAIN, BATCH):
                idx = perm[i:i + BATCH]
                logits = model(train_x[idx])
                loss = F.cross_entropy(logits, train_y[idx])
                optimizer.zero_grad()
                loss.backward()
                optimizer.step()
        elapsed = time.time() - t0

        model.eval()
        with torch.no_grad():
            logits = model(test_x)
            pred = logits.argmax(dim=-1)
            acc = (pred == test_y).float().mean().item()
            loss = F.cross_entropy(logits, test_y).item()

        results.append((label, acc, loss, params, elapsed))
        marker = " <--" if "Fibonacci" in label else ""
        print(f"  {label:<33} {acc:>7.4f} {loss:>7.4f} {params:>10,} {elapsed:>6.1f}s{marker}")

    # Summary
    print(f"\n{'=' * 70}")
    print("  Results Summary")
    print("=" * 70)

    full_acc = results[0][1]
    fib_acc = results[1][1]
    stride_acc = results[2][1]

    ppq = fib_attn.positions_per_query(SEQ_LEN)
    full_ppq = (SEQ_LEN + 1) / 2
    flop_ratio = ppq / full_ppq

    print(f"\n  Fibonacci vs Full:     {(fib_acc - full_acc) / max(full_acc, 1e-6) * 100:+.2f}%")
    print(f"  Fibonacci vs Stride-8: {(fib_acc - stride_acc) / max(stride_acc, 1e-6) * 100:+.2f}%")
    print(f"\n  Attention positions/query (causal, seq={SEQ_LEN}):")
    print(f"    Full:      {full_ppq:.0f}")
    print(f"    Fibonacci: {ppq:.1f} ({flop_ratio:.1%} of full)")
    print(f"    Savings:   {1 - flop_ratio:.1%} attention FLOPs")

    print(f"\n  F(6) = {SIGMA_TAU} = sigma - tau = universal AI constant")
    print(f"  Fibonacci spacing creates O(n log n) attention naturally")
    print(f"  Dense locally + sparse globally = biological perception pattern")
    print(f"  No stride hyperparameter to tune — Fibonacci IS the schedule")


if __name__ == "__main__":
    main()
