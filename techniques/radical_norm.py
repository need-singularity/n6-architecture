"""
Technique 18: Radical Normalization
====================================
rad(n) = product of distinct prime factors of n.
rad(6) = 2 × 3 = 6 = n.

6 is squarefree (μ(6)=1), so rad(6)=6=n — the radical equals the number
itself. This is a self-referential fixed point: the "skeleton" of 6 IS 6.

Insight: In standard LayerNorm, the normalization denominator (std dev) is
data-dependent and varies wildly. Radical Norm adds a "radical constraint":
group the hidden dimension into rad(n)=n=6 equal groups, normalize each
group independently, then rescale by the group's radical weight (1/2, 1/3,
or 1/6 — the unit fractions of 6's proper divisors).

This creates a structured normalization where:
  - 6 groups mirror the 6 divisors of 6: {1, 2, 3, 6}
  - Group sizes follow σ=12 alignment (dim must be divisible by 6)
  - The radical self-reference (rad(6)=6) means the group count = n itself
  - Squarefree property ensures no redundant sub-grouping

Expected: Faster convergence due to structured norm groups, slight accuracy
improvement from divisor-weighted rescaling.
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
RAD_6 = 6          # rad(6) = 2×3 = 6 = n (squarefree fixed point)
SIGMA = 12          # σ(6) = 12
DIVISORS = [1, 2, 3, 6]  # divisors of 6
PROPER_DIVISORS = [1, 2, 3]  # proper divisors (sum = 6 = perfect)


class RadicalNorm(nn.Module):
    """Radical Normalization: rad(6)=6 group-wise normalization.

    Splits the feature dimension into rad(6)=6 groups and normalizes each
    independently, then applies learnable scale/bias per group. The groups
    are weighted by the divisor structure of 6.

    Key property: since 6 is squarefree, rad(6)=n=6, meaning the grouping
    is maximally fine-grained for a single-level decomposition — no nested
    sub-groups exist. This is the "radical skeleton" of the normalization.
    """

    def __init__(self, dim, n_groups=RAD_6, eps=1e-5):
        super().__init__()
        assert dim % n_groups == 0, f"dim={dim} must be divisible by n_groups={n_groups}"
        self.n_groups = n_groups
        self.group_size = dim // n_groups
        self.eps = eps

        # Learnable per-group scale and bias
        self.gamma = nn.Parameter(torch.ones(dim))
        self.beta = nn.Parameter(torch.zeros(dim))

        # Radical weights: based on divisor structure of 6
        # Each group pair (i, n_groups-1-i) gets weight from divisor fraction
        # This encodes the self-referential rad(6)=6 structure
        radical_weights = self._compute_radical_weights(n_groups)
        self.register_buffer('radical_weights', radical_weights)

    def _compute_radical_weights(self, n_groups):
        """Compute weights from the divisor structure of n=6.

        The proper divisors of 6 are {1, 2, 3} with unit fractions
        1/1 + 1/2 + 1/3 + 1/6 = 2 = σ(6)/6 = abundancy.
        We use the normalized divisor fractions as group importance weights.
        """
        # Assign each group a divisor-based weight
        # 6 groups, weights cycle through divisor reciprocals
        weights = torch.ones(n_groups)
        for i in range(n_groups):
            # Map group index to a divisor of 6
            div = DIVISORS[i % len(DIVISORS)]
            weights[i] = div / N  # normalized: 1/6, 2/6, 3/6, 6/6
        # Normalize so mean weight = 1.0 (preserve scale)
        weights = weights / weights.mean()
        return weights

    def forward(self, x):
        # x: (..., dim)
        shape = x.shape
        dim = shape[-1]

        # Reshape to (..., n_groups, group_size)
        x_grouped = x.reshape(*shape[:-1], self.n_groups, self.group_size)

        # Per-group normalization
        mean = x_grouped.mean(dim=-1, keepdim=True)
        var = x_grouped.var(dim=-1, keepdim=True, unbiased=False)
        x_normed = (x_grouped - mean) / (var + self.eps).sqrt()

        # Apply radical weights per group
        # radical_weights: (n_groups,) -> broadcast to (..., n_groups, 1)
        rw = self.radical_weights.reshape(1, self.n_groups, 1)
        while rw.dim() < x_normed.dim():
            rw = rw.unsqueeze(0)
        x_normed = x_normed * rw

        # Reshape back and apply learnable params
        x_normed = x_normed.reshape(shape)
        return x_normed * self.gamma + self.beta


class Phi6Simple(nn.Module):
    def forward(self, x):
        xc = torch.clamp(x, -2.0, 2.0)
        return xc * xc - xc + 1.0


class FFN(nn.Module):
    def __init__(self, d_model, d_ff, activation):
        super().__init__()
        self.fc1 = nn.Linear(d_model, d_ff)
        self.act = activation
        self.fc2 = nn.Linear(d_ff, d_model)

    def forward(self, x):
        return self.fc2(self.act(self.fc1(x)))


class TransformerBlock(nn.Module):
    def __init__(self, d_model, n_heads, d_ff, activation, norm_cls):
        super().__init__()
        self.attn = nn.MultiheadAttention(d_model, n_heads, batch_first=True)
        self.ffn = FFN(d_model, d_ff, activation)
        self.ln1 = norm_cls(d_model)
        self.ln2 = norm_cls(d_model)

    def forward(self, x):
        L = x.size(1)
        mask = torch.triu(torch.ones(L, L, device=x.device), diagonal=1).bool()
        a, _ = self.attn(x, x, x, attn_mask=mask)
        x = self.ln1(x + a)
        x = self.ln2(x + self.ffn(x))
        return x


class CharLM(nn.Module):
    def __init__(self, vocab_size, d_model, n_heads, n_layers, d_ff, seq_len,
                 activation, norm_cls):
        super().__init__()
        self.emb = nn.Embedding(vocab_size, d_model)
        self.pos = nn.Embedding(seq_len, d_model)
        self.blocks = nn.Sequential(*[
            TransformerBlock(d_model, n_heads, d_ff, activation, norm_cls)
            for _ in range(n_layers)
        ])
        self.out = nn.Linear(d_model, vocab_size)

    def forward(self, idx):
        B, T = idx.shape
        x = self.emb(idx) + self.pos(torch.arange(T, device=idx.device))
        x = self.blocks(x)
        return self.out(x)


def count_params(m):
    return sum(p.numel() for p in m.parameters())


def main():
    print("=" * 70)
    print("  Technique 18: Radical Normalization")
    print("  rad(6) = 6 = n — squarefree self-referential fixed point")
    print("=" * 70)
    print(f"\n  rad(6) = 2 × 3 = 6 = n (squarefree: μ(6) = 1)")
    print(f"  Groups = rad(6) = {RAD_6}")
    print(f"  Divisors of 6: {DIVISORS}")
    print(f"  Proper divisors: {PROPER_DIVISORS} (sum = {sum(PROPER_DIVISORS)} = n = perfect)")
    print(f"  Radical weights: divisor-normalized importance per group")

    BASE_TEXT = (
        "Mathematics reveals deep structure. "
        "The number six is perfect because its divisors one two and three sum to itself. "
        "Neural networks learn patterns through gradient descent optimization. "
        "Transformers use attention mechanisms to process sequences efficiently. "
    )
    TEXT = (BASE_TEXT + " ") * 200
    chars = sorted(set(TEXT))
    vocab_size = len(chars)
    c2i = {c: i for i, c in enumerate(chars)}
    data = torch.tensor([c2i[c] for c in TEXT], dtype=torch.long)

    SEQ_LEN = 64
    BATCH = 16
    STEPS = 300
    LR = 3e-3
    D_MODEL = 120  # divisible by 6 (rad(6))
    N_HEADS = 12   # σ(6) = 12
    N_LAYERS = 4   # τ(6) = 4
    D_FF = round(4 * D_MODEL / 3)  # τ²/σ = 4/3

    def get_batch():
        ix = torch.randint(0, len(data) - SEQ_LEN - 1, (BATCH,))
        x = torch.stack([data[i:i+SEQ_LEN] for i in ix])
        y = torch.stack([data[i+1:i+SEQ_LEN+1] for i in ix])
        return x, y

    configs = [
        ("LayerNorm (baseline)",    lambda d: nn.LayerNorm(d)),
        ("RadicalNorm (6 groups)",  lambda d: RadicalNorm(d, n_groups=6)),
        ("GroupNorm (6 groups)",    lambda d: nn.GroupNorm(6, d)),
        ("RadicalNorm (3 groups)",  lambda d: RadicalNorm(d, n_groups=3)),
    ]

    results = []
    for label, norm_factory in configs:
        print(f"\n--- {label} ---")
        torch.manual_seed(SEED)
        norm_cls = norm_factory
        model = CharLM(vocab_size, D_MODEL, N_HEADS, N_LAYERS, D_FF, SEQ_LEN,
                        Phi6Simple(), norm_cls)
        total_p = count_params(model)
        opt = torch.optim.Adam(model.parameters(), lr=LR)

        t0 = time.time()
        losses = []
        for step in range(STEPS):
            x, y = get_batch()
            logits = model(x)
            loss = F.cross_entropy(logits.reshape(-1, vocab_size), y.reshape(-1))
            opt.zero_grad()
            loss.backward()
            nn.utils.clip_grad_norm_(model.parameters(), 1.0)
            opt.step()
            losses.append(loss.item())
        elapsed = time.time() - t0

        # Eval
        model.eval()
        eval_losses = []
        with torch.no_grad():
            for _ in range(20):
                x, y = get_batch()
                logits = model(x)
                eloss = F.cross_entropy(logits.reshape(-1, vocab_size), y.reshape(-1))
                eval_losses.append(eloss.item())

        results.append({
            "label": label,
            "total_params": total_p,
            "train_loss": np.mean(losses[-20:]),
            "eval_loss": np.mean(eval_losses),
            "gap": np.mean(eval_losses) - np.mean(losses[-20:]),
            "train_time": elapsed,
        })

    print("\n" + "=" * 70)
    print("  Radical Normalization Results")
    print("=" * 70)
    print(f"{'Config':<30} {'Params':>10} {'Train':>8} {'Eval':>8} {'Gap':>8} {'Time':>7}")
    print("-" * 75)
    for r in results:
        marker = " <--" if "RadicalNorm (6" in r["label"] else ""
        print(f"{r['label']:<30} {r['total_params']:>10,} {r['train_loss']:>8.4f} "
              f"{r['eval_loss']:>8.4f} {r['gap']:>8.4f} {r['train_time']:>6.1f}s{marker}")

    print(f"\n  rad(6) = {RAD_6} = n (self-referential)")
    print(f"  The radical of 6 equals 6 itself — the normalization group count")
    print(f"  is determined by the number's own prime skeleton.")
    print(f"  Divisor-weighted rescaling encodes σ(6)/6 = 2 abundancy structure.")
    print(f"  No hyperparameter search needed — groups = rad(n) = {RAD_6}.")


if __name__ == "__main__":
    main()
