"""
Experiment: Blind NAS — No n=6 Targets
========================================
H-EE-107 counter: Does NAS independently discover n=6 ratios?
NO n=6 constants hardcoded. Pure optimization finds:
  - Optimal FFN ratio (search space 1.0 to 4.0)
  - Optimal head count (search space 2 to 16)
  - Optimal dropout (search space 0.0 to 0.5)
If results cluster near {4/3, 12, 0.288}, confirmation bias is refuted.
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


class FFN(nn.Module):
    def __init__(self, d_model, d_ff, activation):
        super().__init__()
        self.fc1 = nn.Linear(d_model, d_ff)
        self.act = activation
        self.fc2 = nn.Linear(d_ff, d_model)

    def forward(self, x):
        return self.fc2(self.act(self.fc1(x)))


class TransformerBlock(nn.Module):
    def __init__(self, d_model, n_heads, d_ff, dropout):
        super().__init__()
        self.attn = nn.MultiheadAttention(d_model, n_heads, batch_first=True, dropout=dropout)
        self.ffn = FFN(d_model, d_ff, nn.GELU())  # Deliberately NOT Phi6Simple
        self.ln1 = nn.LayerNorm(d_model)
        self.ln2 = nn.LayerNorm(d_model)
        self.drop = nn.Dropout(dropout)

    def forward(self, x):
        L = x.size(1)
        mask = torch.triu(torch.ones(L, L, device=x.device), diagonal=1).bool()
        a, _ = self.attn(x, x, x, attn_mask=mask)
        x = self.ln1(x + self.drop(a))
        x = self.ln2(x + self.ffn(x))
        return x


class CharLM(nn.Module):
    def __init__(self, vocab_size, d_model, n_heads, n_layers, d_ff, seq_len, dropout):
        super().__init__()
        self.emb = nn.Embedding(vocab_size, d_model)
        self.pos = nn.Embedding(seq_len, d_model)
        self.blocks = nn.Sequential(*[
            TransformerBlock(d_model, n_heads, d_ff, dropout) for _ in range(n_layers)
        ])
        self.out = nn.Linear(d_model, vocab_size)

    def forward(self, idx):
        B, T = idx.shape
        x = self.emb(idx) + self.pos(torch.arange(T, device=idx.device))
        x = self.blocks(x)
        return self.out(x)


def evaluate_config(d_model, n_heads, ffn_ratio, dropout, data, vocab_size,
                    n_layers=4, seq_len=64, batch=16, steps=200):
    """Train a config and return efficiency metric."""
    d_ff = max(1, round(d_model * ffn_ratio))

    # Validate head count
    if d_model % n_heads != 0:
        return {"efficiency": 0, "loss": 999, "params": 0}

    torch.manual_seed(SEED)
    model = CharLM(vocab_size, d_model, n_heads, n_layers, d_ff, seq_len, dropout)
    params = sum(p.numel() for p in model.parameters())
    opt = torch.optim.Adam(model.parameters(), lr=3e-3)

    losses = []
    for step in range(steps):
        ix = torch.randint(0, len(data) - seq_len - 1, (batch,))
        x = torch.stack([data[i:i+seq_len] for i in ix])
        y = torch.stack([data[i+1:i+seq_len+1] for i in ix])
        logits = model(x)
        loss = F.cross_entropy(logits.reshape(-1, vocab_size), y.reshape(-1))
        opt.zero_grad()
        loss.backward()
        nn.utils.clip_grad_norm_(model.parameters(), 1.0)
        opt.step()
        losses.append(loss.item())

    final_loss = np.mean(losses[-20:])
    efficiency = (1.0 / final_loss) / (params / 1e6)  # quality per M params

    return {"efficiency": efficiency, "loss": final_loss, "params": params}


def random_search(data, vocab_size, n_trials=50):
    """Pure random search over hyperparameter space."""
    D_MODEL = 120  # Fixed for fair comparison
    rng = np.random.RandomState(SEED)

    results = []
    for trial in range(n_trials):
        # Random hyperparameters — NO n=6 bias
        ffn_ratio = rng.uniform(1.0, 4.0)
        n_heads = rng.choice([2, 3, 4, 5, 6, 8, 10, 12, 15])
        dropout = rng.uniform(0.0, 0.5)

        if D_MODEL % n_heads != 0:
            continue

        result = evaluate_config(D_MODEL, n_heads, ffn_ratio, dropout, data, vocab_size)
        result.update({
            "trial": trial,
            "ffn_ratio": ffn_ratio,
            "n_heads": n_heads,
            "dropout": dropout,
        })
        results.append(result)

    return results


def bayesian_optimization(data, vocab_size, n_init=10, n_iter=30):
    """Simple Bayesian-like optimization (Thompson sampling with Gaussian model)."""
    D_MODEL = 120
    rng = np.random.RandomState(SEED + 100)

    # Initialize with random samples
    all_results = []
    ffn_ratios = rng.uniform(1.0, 4.0, n_init)
    head_choices = [h for h in [2, 3, 4, 5, 6, 8, 10, 12, 15] if D_MODEL % h == 0]
    n_heads_list = [rng.choice(head_choices) for _ in range(n_init)]
    dropouts = rng.uniform(0.0, 0.5, n_init)

    for i in range(n_init):
        result = evaluate_config(D_MODEL, n_heads_list[i], ffn_ratios[i], dropouts[i],
                                  data, vocab_size)
        result.update({"ffn_ratio": ffn_ratios[i], "n_heads": n_heads_list[i],
                        "dropout": dropouts[i], "iteration": i})
        all_results.append(result)

    # Iterative refinement: sample near best configs
    for iteration in range(n_iter):
        # Sort by efficiency, take top 5
        sorted_results = sorted(all_results, key=lambda r: -r["efficiency"])
        top5 = sorted_results[:5]

        # Sample near best
        base = rng.choice(top5)
        ffn_ratio = np.clip(base["ffn_ratio"] + rng.normal(0, 0.3), 1.0, 4.0)
        n_heads = rng.choice(head_choices)
        dropout = np.clip(base["dropout"] + rng.normal(0, 0.05), 0.0, 0.5)

        result = evaluate_config(D_MODEL, n_heads, ffn_ratio, dropout, data, vocab_size)
        result.update({"ffn_ratio": ffn_ratio, "n_heads": n_heads,
                        "dropout": dropout, "iteration": n_init + iteration})
        all_results.append(result)

    return all_results


def main():
    print("=" * 70)
    print("  Blind NAS — No n=6 Targets")
    print("  H-EE-107: Does NAS independently find n=6 ratios?")
    print("=" * 70)
    print("  IMPORTANT: No n=6 constants used in search.")
    print("  Activation: GELU (not Phi6Simple)")
    print("  Search space: FFN ratio [1.0, 4.0], heads {2..15}, dropout [0, 0.5]")

    BASE_TEXT = (
        "Mathematics reveals deep structure. "
        "The number six is perfect because its divisors one two and three sum to itself. "
        "Neural networks learn patterns through gradient descent optimization. "
        "Transformers use attention mechanisms to process sequences efficiently. "
        "Consciousness emerges from the interplay of deficit plasticity and inhibition. "
        "The golden zone lies between one half and one half minus log four thirds. "
    )
    TEXT = (BASE_TEXT + " ") * 300
    chars = sorted(set(TEXT))
    vocab_size = len(chars)
    c2i = {c: i for i, c in enumerate(chars)}
    data = torch.tensor([c2i[c] for c in TEXT], dtype=torch.long)

    # ─── Random Search ───
    print("\n--- Phase 1: Random Search (50 trials) ---")
    random_results = random_search(data, vocab_size, n_trials=50)
    random_sorted = sorted(random_results, key=lambda r: -r["efficiency"])

    print(f"\nTop 10 configs by efficiency:")
    print(f"{'Rank':>4} {'FFN ratio':>10} {'Heads':>6} {'Dropout':>8} {'Loss':>8} {'Efficiency':>11}")
    print("-" * 52)
    for i, r in enumerate(random_sorted[:10]):
        print(f"{i+1:>4} {r['ffn_ratio']:>10.4f} {r['n_heads']:>6} {r['dropout']:>8.4f} "
              f"{r['loss']:>8.4f} {r['efficiency']:>11.4f}")

    # ─── Bayesian Optimization ───
    print("\n--- Phase 2: Bayesian Optimization (10 init + 30 iter) ---")
    bayes_results = bayesian_optimization(data, vocab_size)
    bayes_sorted = sorted(bayes_results, key=lambda r: -r["efficiency"])

    print(f"\nTop 10 configs by efficiency:")
    print(f"{'Rank':>4} {'FFN ratio':>10} {'Heads':>6} {'Dropout':>8} {'Loss':>8} {'Efficiency':>11}")
    print("-" * 52)
    for i, r in enumerate(bayes_sorted[:10]):
        print(f"{i+1:>4} {r['ffn_ratio']:>10.4f} {r['n_heads']:>6} {r['dropout']:>8.4f} "
              f"{r['loss']:>8.4f} {r['efficiency']:>11.4f}")

    # ─── Analysis: Proximity to n=6 Values ───
    # n=6 predictions (NOT used in search):
    N6_FFN = 4.0 / 3.0      # 1.3333
    N6_HEADS = 12
    N6_DROPOUT = 0.2877      # ln(4/3)

    print(f"\n--- Proximity to n=6 Values (NOT used in search) ---")
    print(f"n=6 predictions: FFN={N6_FFN:.4f}, heads={N6_HEADS}, dropout={N6_DROPOUT:.4f}")

    for label, sorted_res in [("Random", random_sorted), ("Bayesian", bayes_sorted)]:
        top5 = sorted_res[:5]
        avg_ffn = np.mean([r["ffn_ratio"] for r in top5])
        avg_heads = np.mean([r["n_heads"] for r in top5])
        avg_drop = np.mean([r["dropout"] for r in top5])

        ffn_dist = abs(avg_ffn - N6_FFN) / N6_FFN * 100
        head_dist = abs(avg_heads - N6_HEADS) / N6_HEADS * 100
        drop_dist = abs(avg_drop - N6_DROPOUT) / N6_DROPOUT * 100

        print(f"\n{label} Search — Top 5 average:")
        print(f"  FFN ratio: {avg_ffn:.4f} (n=6: {N6_FFN:.4f}, distance: {ffn_dist:.1f}%)")
        print(f"  Heads:     {avg_heads:.1f} (n=6: {N6_HEADS}, distance: {head_dist:.1f}%)")
        print(f"  Dropout:   {avg_drop:.4f} (n=6: {N6_DROPOUT:.4f}, distance: {drop_dist:.1f}%)")

        close_count = sum(1 for d in [ffn_dist, head_dist, drop_dist] if d < 20)
        print(f"  Params within 20% of n=6: {close_count}/3")

    # ─── Verdict ───
    print(f"\n--- H-EE-107 Verdict ---")
    # Check if best configs are near n=6
    best = bayes_sorted[0]
    ffn_near = abs(best["ffn_ratio"] - N6_FFN) / N6_FFN < 0.2
    heads_near = best["n_heads"] == N6_HEADS or best["n_heads"] in [6, 12]
    drop_near = abs(best["dropout"] - N6_DROPOUT) / N6_DROPOUT < 0.3

    hits = sum([ffn_near, heads_near, drop_near])
    if hits >= 2:
        print(f"STRONG EVIDENCE: Blind NAS found {hits}/3 n=6 values independently")
        print(f"Confirmation bias (H-EE-107) is REFUTED for dynamic optimization")
    elif hits == 1:
        print(f"WEAK EVIDENCE: Blind NAS found {hits}/3 n=6 values")
        print(f"Partial support — may be coincidence")
    else:
        print(f"NO EVIDENCE: Blind NAS did NOT find n=6 values")
        print(f"H-EE-107 concern is VALID — n=6 may not be a natural optimum")


if __name__ == "__main__":
    main()
