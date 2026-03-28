"""
Technique 12: Jordan-Leech MoE Capacity Bound
===============================================
J_2(6) = 24 = dim(Leech lattice).
24 experts maximize specialization packing with minimum overlap.
Combined with Egyptian routing {1/2, 1/3, 1/6} and phi-bottleneck FFN.

Expected: routing overhead elimination via fixed 24-expert topology.
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

# ─── Constants ───
JORDAN_J2 = 24  # J_2(6) = 24
SIGMA = 12
TAU = 4
PHI = 2
EGYPTIAN = [1/2, 1/3, 1/6]  # divisor reciprocals of 6
LEECH_KISSING = 196560       # kissing number of Leech lattice


class Expert(nn.Module):
    def __init__(self, d_model, d_ff):
        super().__init__()
        self.fc1 = nn.Linear(d_model, d_ff)
        self.fc2 = nn.Linear(d_ff, d_model)

    def forward(self, x):
        return self.fc2(F.relu(self.fc1(x)))


class JordanLeechMoE(nn.Module):
    """MoE with J_2(6)=24 experts and Egyptian top-3 routing."""

    def __init__(self, d_model, d_ff_per_expert, n_experts=24, top_k=3):
        super().__init__()
        self.n_experts = n_experts
        self.top_k = top_k
        self.experts = nn.ModuleList([
            Expert(d_model, d_ff_per_expert) for _ in range(n_experts)
        ])
        self.gate = nn.Linear(d_model, n_experts, bias=False)
        self.egyptian_weights = torch.tensor(EGYPTIAN)  # {1/2, 1/3, 1/6}

        # Tracking
        self.expert_usage = torch.zeros(n_experts)
        self.active_counts = []

    def forward(self, x):
        B, L, D = x.shape
        x_flat = x.reshape(-1, D)  # (B*L, D)

        logits = self.gate(x_flat)  # (B*L, n_experts)
        top_vals, top_idx = logits.topk(self.top_k, dim=-1)  # (B*L, 3)

        # Egyptian fraction weighting (fixed, not learned)
        eg = self.egyptian_weights.to(x.device)  # (3,)

        # Compute weighted expert outputs
        output = torch.zeros_like(x_flat)
        for k in range(self.top_k):
            expert_idx = top_idx[:, k]  # (B*L,)
            weight = eg[k]              # scalar: 1/2, 1/3, or 1/6

            for e in range(self.n_experts):
                mask = (expert_idx == e)
                if mask.any():
                    expert_input = x_flat[mask]
                    expert_output = self.experts[e](expert_input)
                    output[mask] += weight * expert_output

        # Track usage
        with torch.no_grad():
            for k in range(self.top_k):
                for e in range(self.n_experts):
                    count = (top_idx[:, k] == e).sum().item()
                    self.expert_usage[e] += count
            self.active_counts.append(self.top_k)

        return output.reshape(B, L, D)

    def get_metrics(self):
        usage = self.expert_usage / max(self.expert_usage.sum().item(), 1)
        return {
            "usage_entropy": -(usage * (usage + 1e-10).log()).sum().item(),
            "max_usage": usage.max().item(),
            "min_usage": usage.min().item(),
            "usage_std": usage.std().item(),
        }

    def reset_metrics(self):
        self.expert_usage.zero_()
        self.active_counts.clear()


class TransformerBlock(nn.Module):
    def __init__(self, d_model, n_heads, moe_layer):
        super().__init__()
        self.attn = nn.MultiheadAttention(d_model, n_heads, batch_first=True)
        self.moe = moe_layer
        self.ln1 = nn.LayerNorm(d_model)
        self.ln2 = nn.LayerNorm(d_model)

    def forward(self, x):
        L = x.size(1)
        mask = torch.triu(torch.ones(L, L, device=x.device), diagonal=1).bool()
        a, _ = self.attn(x, x, x, attn_mask=mask)
        x = self.ln1(x + a)
        x = self.ln2(x + self.moe(x))
        return x


class MoECharLM(nn.Module):
    def __init__(self, vocab_size, d_model, n_heads, n_layers, moe_factory, seq_len):
        super().__init__()
        self.emb = nn.Embedding(vocab_size, d_model)
        self.pos = nn.Embedding(seq_len, d_model)
        self.blocks = nn.ModuleList([
            TransformerBlock(d_model, n_heads, moe_factory())
            for _ in range(n_layers)
        ])
        self.out = nn.Linear(d_model, vocab_size)

    def forward(self, idx):
        B, T = idx.shape
        x = self.emb(idx) + self.pos(torch.arange(T, device=idx.device))
        for block in self.blocks:
            x = block(x)
        return self.out(x)


def count_params(m):
    return sum(p.numel() for p in m.parameters())


def main():
    print("=" * 70)
    print("  Technique 12: Jordan-Leech MoE Capacity Bound")
    print("  J_2(6) = 24 = dim(Leech lattice)")
    print("=" * 70)

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
    D_MODEL = 120
    N_HEADS = 12  # Dedekind optimal
    N_LAYERS = 2  # reduced for MoE overhead

    def get_batch():
        ix = torch.randint(0, len(data) - SEQ_LEN - 1, (BATCH,))
        x = torch.stack([data[i:i+SEQ_LEN] for i in ix])
        y = torch.stack([data[i+1:i+SEQ_LEN+1] for i in ix])
        return x, y

    configs = [
        ("8 experts, 4x FFN, top-2",    8,  4 * D_MODEL, 2),
        ("24 experts, 4/3x FFN, top-3",  24, round(4 * D_MODEL / 3), 3),
        ("32 experts, 1x FFN, top-3",   32, D_MODEL, 3),
        ("48 experts, 2/3x FFN, top-3", 48, round(2 * D_MODEL / 3), 3),
    ]

    results = []
    for label, n_exp, d_ff, top_k in configs:
        print(f"\n--- {label} ---")
        moe_factory = lambda ne=n_exp, df=d_ff, tk=top_k: JordanLeechMoE(
            D_MODEL, df, ne, tk
        )
        model = MoECharLM(vocab_size, D_MODEL, N_HEADS, N_LAYERS, moe_factory, SEQ_LEN)
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

        moe_metrics = model.blocks[0].moe.get_metrics()
        active_per_token = n_exp * top_k
        active_params = total_p

        results.append({
            "label": label,
            "n_experts": n_exp,
            "d_ff": d_ff,
            "top_k": top_k,
            "total_params": total_p,
            "final_loss": np.mean(losses[-20:]),
            "train_time": elapsed,
            "usage_entropy": moe_metrics["usage_entropy"],
            "usage_std": moe_metrics["usage_std"],
        })

    print("\n" + "=" * 70)
    print("  Jordan-Leech MoE Results")
    print("=" * 70)
    print(f"{'Config':<35} {'Experts':>7} {'Params':>10} {'Loss':>8} {'UsageH':>7} {'Time':>7}")
    print("-" * 78)
    for r in results:
        print(f"{r['label']:<35} {r['n_experts']:>7} {r['total_params']:>10,} "
              f"{r['final_loss']:>8.4f} {r['usage_entropy']:>7.3f} {r['train_time']:>6.1f}s")

    print("\n--- Leech Lattice Connection ---")
    print(f"J_2(6) = {JORDAN_J2}")
    print(f"sigma(6) * phi(6) = {SIGMA} * {PHI} = {SIGMA * PHI}")
    print(f"Leech lattice dimension = 24")
    print(f"Leech kissing number = {LEECH_KISSING:,}")
    print(f"Tokens per expert capacity ~ {LEECH_KISSING // JORDAN_J2:,}")
    print(f"\n24 experts with 4/3x FFN each:")
    print(f"  Active params per token: top-3 * d_ff = 3 * {round(4*D_MODEL/3)} = {3*round(4*D_MODEL/3)}")
    print(f"  vs 8 experts top-2 * 4x: 2 * {4*D_MODEL} = {2*4*D_MODEL}")
    print(f"  Ratio: {3*round(4*D_MODEL/3) / (2*4*D_MODEL):.2f}")


if __name__ == "__main__":
    main()
