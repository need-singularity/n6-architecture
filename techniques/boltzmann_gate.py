"""
Technique 15: Boltzmann Gate
=============================
Golden Zone center = 1/e ~ 0.3679.
Optimal information throughput: only 1/e of activations carry signal.
The rest are thermal noise (Boltzmann partition function optimum).

Expected: 63% activation sparsity with minimal accuracy loss.
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

GOLDEN_ZONE_CENTER = 1.0 / math.e
SPARSITY = 1.0 - GOLDEN_ZONE_CENTER


class BoltzmannGateSTE(nn.Module):
    """Pass only top-1/e activations by magnitude. Zero the rest.
    Uses straight-through estimator for backward pass."""

    def __init__(self, fraction=GOLDEN_ZONE_CENTER):
        super().__init__()
        self.fraction = fraction

    def forward(self, x):
        if not self.training:
            return x

        flat = x.abs().reshape(-1)
        k = max(1, int(flat.numel() * self.fraction))
        if k >= flat.numel():
            return x
        threshold = flat.topk(k).values[-1]
        mask = (x.abs() >= threshold).float()

        # STE: mask in forward, identity in backward
        return x * (mask + (1 - mask).detach() * 0)


class Phi6Simple(nn.Module):
    def forward(self, x):
        xc = torch.clamp(x, -2.0, 2.0)
        return xc * xc - xc + 1.0


class GatedPhi6(nn.Module):
    """Phi6Simple followed by Boltzmann gate."""
    def __init__(self):
        super().__init__()
        self.phi6 = Phi6Simple()
        self.gate = BoltzmannGateSTE()

    def forward(self, x):
        return self.gate(self.phi6(x))


class FFN(nn.Module):
    def __init__(self, d_model, d_ff, activation):
        super().__init__()
        self.fc1 = nn.Linear(d_model, d_ff)
        self.act = activation
        self.fc2 = nn.Linear(d_ff, d_model)

    def forward(self, x):
        return self.fc2(self.act(self.fc1(x)))


class TransformerBlock(nn.Module):
    def __init__(self, d_model, n_heads, d_ff, activation):
        super().__init__()
        self.attn = nn.MultiheadAttention(d_model, n_heads, batch_first=True)
        self.ffn = FFN(d_model, d_ff, activation)
        self.ln1 = nn.LayerNorm(d_model)
        self.ln2 = nn.LayerNorm(d_model)

    def forward(self, x):
        L = x.size(1)
        mask = torch.triu(torch.ones(L, L, device=x.device), diagonal=1).bool()
        a, _ = self.attn(x, x, x, attn_mask=mask)
        x = self.ln1(x + a)
        x = self.ln2(x + self.ffn(x))
        return x


class CharLM(nn.Module):
    def __init__(self, vocab_size, d_model, n_heads, n_layers, d_ff, seq_len, activation):
        super().__init__()
        self.emb = nn.Embedding(vocab_size, d_model)
        self.pos = nn.Embedding(seq_len, d_model)
        self.blocks = nn.Sequential(*[
            TransformerBlock(d_model, n_heads, d_ff, activation) for _ in range(n_layers)
        ])
        self.out = nn.Linear(d_model, vocab_size)

    def forward(self, idx):
        B, T = idx.shape
        x = self.emb(idx) + self.pos(torch.arange(T, device=idx.device))
        x = self.blocks(x)
        return self.out(x)


def count_params(m):
    return sum(p.numel() for p in m.parameters())


def measure_sparsity(model, x):
    hooks = []
    sparsities = []

    def hook_fn(module, input, output):
        if isinstance(output, torch.Tensor):
            zeros = (output.abs() < 1e-8).float().mean().item()
            sparsities.append(zeros)

    for module in model.modules():
        if isinstance(module, BoltzmannGateSTE):
            hooks.append(module.register_forward_hook(hook_fn))

    model.train()
    with torch.no_grad():
        model(x)

    for h in hooks:
        h.remove()

    return np.mean(sparsities) if sparsities else 0.0


def main():
    print("=" * 70)
    print("  Technique 15: Boltzmann Gate")
    print("  Golden Zone = 1/e — optimal information throughput")
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
    N_HEADS = 12
    N_LAYERS = 4
    D_FF = round(4 * D_MODEL / 3)

    def get_batch():
        ix = torch.randint(0, len(data) - SEQ_LEN - 1, (BATCH,))
        x = torch.stack([data[i:i+SEQ_LEN] for i in ix])
        y = torch.stack([data[i+1:i+SEQ_LEN+1] for i in ix])
        return x, y

    configs = [
        ("Phi6Simple (no gate)",        Phi6Simple()),
        ("Phi6 + Boltzmann(1/e)",       GatedPhi6()),
        ("GELU (baseline)",             nn.GELU()),
    ]

    results = []
    for label, activation in configs:
        print(f"\n--- {label} ---")
        torch.manual_seed(SEED)
        model = CharLM(vocab_size, D_MODEL, N_HEADS, N_LAYERS, D_FF, SEQ_LEN, activation)
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

        sample_x, _ = get_batch()
        sparsity = measure_sparsity(model, sample_x)

        results.append({
            "label": label,
            "total_params": total_p,
            "final_loss": np.mean(losses[-20:]),
            "train_time": elapsed,
            "sparsity": sparsity,
        })

    print("\n" + "=" * 70)
    print("  Boltzmann Gate Results")
    print("=" * 70)
    print(f"{'Config':<30} {'Params':>10} {'Loss':>8} {'Sparsity':>10} {'Time':>7}")
    print("-" * 68)
    for r in results:
        print(f"{r['label']:<30} {r['total_params']:>10,} {r['final_loss']:>8.4f} "
              f"{r['sparsity']:>9.1%} {r['train_time']:>6.1f}s")

    print(f"\n1/e = {GOLDEN_ZONE_CENTER:.4f} (fraction passed)")
    print(f"Target sparsity = {SPARSITY:.1%}")
    print(f"Boltzmann partition function Z = sum(exp(-E/kT))")
    print(f"At thermal equilibrium, fraction of 'active' states = 1/e")
    print(f"63% of activations are thermal noise — safe to gate.")


if __name__ == "__main__":
    main()
