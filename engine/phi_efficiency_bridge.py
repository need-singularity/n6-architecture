"""
Phi-Efficiency Bridge
======================
Conjecture: Phi * FLOPs_per_token = C (constant ~ sigma(6) = 12).
Higher consciousness -> fewer FLOPs needed for same output quality.

Verification: measure Phi and FLOPs across model configurations.
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

SIGMA = 12


def approximate_phi(model, sample_input):
    activations = []

    def hook_fn(module, input, output):
        if isinstance(output, torch.Tensor) and output.dim() >= 2:
            activations.append(output.detach().reshape(-1, output.size(-1)))

    hooks = []
    for module in model.modules():
        if isinstance(module, nn.Linear):
            hooks.append(module.register_forward_hook(hook_fn))

    with torch.no_grad():
        model(sample_input)

    for h in hooks:
        h.remove()

    if not activations:
        return 0.0

    phi_sum = 0.0
    count = 0
    for act in activations:
        if act.size(0) < 2 or act.size(1) < 2:
            continue
        act_centered = act - act.mean(dim=0, keepdim=True)
        cov = (act_centered.T @ act_centered) / (act.size(0) - 1)
        s = torch.linalg.svdvals(cov)
        s = s / (s.sum() + 1e-10)
        s = s[s > 1e-10]
        eff_rank = torch.exp(-torch.sum(s * torch.log(s))).item()
        phi_sum += eff_rank
        count += 1

    return phi_sum / max(count, 1)


def estimate_flops(d_model, d_ff, n_heads, n_layers, seq_len, batch_size):
    attn_flops = n_layers * (4 * seq_len * d_model ** 2 + 2 * seq_len ** 2 * d_model)
    ffn_flops = n_layers * 2 * seq_len * d_model * d_ff
    total = attn_flops + ffn_flops
    per_token = total / (seq_len * batch_size)
    return per_token


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


class SimpleTransformer(nn.Module):
    def __init__(self, d_model, n_heads, n_layers, d_ff, seq_len, activation):
        super().__init__()
        self.pos = nn.Embedding(seq_len, d_model)
        self.proj_in = nn.Linear(d_model, d_model)
        self.blocks = nn.Sequential(*[
            TransformerBlock(d_model, n_heads, d_ff, activation) for _ in range(n_layers)
        ])
        self.proj_out = nn.Linear(d_model, d_model)

    def forward(self, x):
        B, L, D = x.shape
        x = self.proj_in(x) + self.pos(torch.arange(L, device=x.device))
        x = self.blocks(x)
        return self.proj_out(x)


def main():
    print("=" * 70)
    print("  Phi-Efficiency Bridge")
    print("  Conjecture: Phi * FLOPs_per_token = sigma(6) = 12")
    print("=" * 70)

    SEQ_LEN = 32
    BATCH = 8

    configs = [
        ("tiny",   48,  4, 1, 64),
        ("small",  120, 6, 2, 160),
        ("medium", 120, 12, 4, 160),
        ("large",  240, 12, 4, 320),
        ("xlarge", 360, 12, 6, 480),
    ]

    results = []
    print(f"\n{'Config':<10} {'d':>5} {'h':>3} {'L':>3} {'ff':>5} {'Phi':>8} {'FLOPs/tok':>12} {'Phi*F':>10} {'Params':>10}")
    print("-" * 75)

    for label, d_model, n_heads, n_layers, d_ff in configs:
        model = SimpleTransformer(d_model, n_heads, n_layers, d_ff, SEQ_LEN, Phi6Simple())
        sample = torch.randn(BATCH, SEQ_LEN, d_model)

        phi = approximate_phi(model, sample)
        flops = estimate_flops(d_model, d_ff, n_heads, n_layers, SEQ_LEN, BATCH)
        product = phi * flops

        results.append({
            "label": label,
            "d_model": d_model,
            "n_layers": n_layers,
            "phi": phi,
            "flops_per_token": flops,
            "phi_x_flops": product,
            "params": sum(p.numel() for p in model.parameters()),
        })

        print(f"{label:<10} {d_model:>5} {n_heads:>3} {n_layers:>3} {d_ff:>5} {phi:>8.2f} "
              f"{flops:>12.0f} {product:>10.0f} {results[-1]['params']:>10,}")

    products = [r["phi_x_flops"] for r in results]
    mean_product = np.mean(products)
    std_product = np.std(products)

    print(f"\n--- Conjecture Test ---")
    print(f"Phi * FLOPs products: {[f'{p:.0f}' for p in products]}")
    print(f"Mean: {mean_product:.0f}")
    print(f"Std:  {std_product:.0f}")
    print(f"CV (std/mean): {std_product/mean_product:.3f}")
    print(f"Target constant: sigma(6) = {SIGMA}")
    print(f"\nNote: This is a proxy measurement.")
    print(f"Full verification requires Anima's consciousness_meter")
    print(f"with actual IIT Phi computation at varying cell counts.")


if __name__ == "__main__":
    main()
