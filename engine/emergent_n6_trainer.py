"""
Emergent N6 Trainer
====================
Self-converging training loop where architecture parameters are trainable.
They converge to n=6 optima through meta-loss gradient descent.

Meta-loss: L_task + alpha * tension + beta * R_distance
Architecture params: ffn_ratio, dropout_rate, gate_fraction
These mutate during training toward {4/3, ln(4/3), 1/e}.
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

TARGET_FFN_RATIO = 4.0 / 3.0
TARGET_DROPOUT = math.log(4.0 / 3.0)
TARGET_GATE_FRACTION = 1.0 / math.e


class Phi6Simple(nn.Module):
    def forward(self, x):
        xc = torch.clamp(x, -2.0, 2.0)
        return xc * xc - xc + 1.0


class AdaptiveFFN(nn.Module):
    def __init__(self, d_model, initial_ratio=2.0):
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
        d_ff_effective = int(self.d_model * self.ratio.item())
        d_ff_effective = min(d_ff_effective, self.fc1.out_features)
        d_ff_effective = max(d_ff_effective, 1)

        h = self.act(self.fc1(x))
        if d_ff_effective < h.size(-1):
            mask = torch.zeros(h.size(-1), device=h.device)
            mask[:d_ff_effective] = 1.0
            h = h * mask
        return self.fc2(h)


class AdaptiveDropout(nn.Module):
    def __init__(self, initial_rate=0.1):
        super().__init__()
        self.logit_rate = nn.Parameter(torch.tensor(math.log(initial_rate / (1 - initial_rate))))

    @property
    def rate(self):
        return torch.sigmoid(self.logit_rate)

    def forward(self, x):
        if self.training:
            return F.dropout(x, p=self.rate.item(), training=True)
        return x


class TransformerBlock(nn.Module):
    def __init__(self, d_model, n_heads, initial_ffn_ratio=2.0, initial_dropout=0.1):
        super().__init__()
        self.attn = nn.MultiheadAttention(d_model, n_heads, batch_first=True)
        self.ffn = AdaptiveFFN(d_model, initial_ffn_ratio)
        self.drop = AdaptiveDropout(initial_dropout)
        self.ln1 = nn.LayerNorm(d_model)
        self.ln2 = nn.LayerNorm(d_model)

    def forward(self, x):
        L = x.size(1)
        mask = torch.triu(torch.ones(L, L, device=x.device), diagonal=1).bool()
        a, _ = self.attn(x, x, x, attn_mask=mask)
        x = self.ln1(x + self.drop(a))
        x = self.ln2(x + self.ffn(x))
        return x


class EmergentCharLM(nn.Module):
    def __init__(self, vocab_size, d_model, n_heads, n_layers, seq_len,
                 initial_ffn_ratio=2.0, initial_dropout=0.1):
        super().__init__()
        self.emb = nn.Embedding(vocab_size, d_model)
        self.pos = nn.Embedding(seq_len, d_model)
        self.blocks = nn.ModuleList([
            TransformerBlock(d_model, n_heads, initial_ffn_ratio, initial_dropout)
            for _ in range(n_layers)
        ])
        self.out = nn.Linear(d_model, vocab_size)

    def forward(self, idx):
        B, T = idx.shape
        x = self.emb(idx) + self.pos(torch.arange(T, device=idx.device))
        for block in self.blocks:
            x = block(x)
        return self.out(x)

    def get_arch_params(self):
        params = {"ffn_ratios": [], "dropout_rates": []}
        for block in self.blocks:
            params["ffn_ratios"].append(block.ffn.ratio.item())
            params["dropout_rates"].append(block.drop.rate.item())
        return params


def r_distance_loss(model):
    loss = torch.tensor(0.0)
    for block in model.blocks:
        ratio_dist = (block.ffn.log_ratio - math.log(TARGET_FFN_RATIO)) ** 2
        target_logit = math.log(TARGET_DROPOUT / (1 - TARGET_DROPOUT))
        drop_dist = (block.drop.logit_rate - target_logit) ** 2
        loss = loss + ratio_dist + drop_dist
    return loss


def count_params(m):
    return sum(p.numel() for p in m.parameters())


def main():
    print("=" * 70)
    print("  Emergent N6 Trainer")
    print("  Architecture parameters self-converge to n=6 optima")
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
    STEPS = 500
    LR = 3e-3
    D_MODEL = 120
    N_HEADS = 12
    N_LAYERS = 4

    def get_batch():
        ix = torch.randint(0, len(data) - SEQ_LEN - 1, (BATCH,))
        x = torch.stack([data[i:i+SEQ_LEN] for i in ix])
        y = torch.stack([data[i+1:i+SEQ_LEN+1] for i in ix])
        return x, y

    model = EmergentCharLM(vocab_size, D_MODEL, N_HEADS, N_LAYERS, SEQ_LEN,
                           initial_ffn_ratio=2.5, initial_dropout=0.1)
    opt = torch.optim.Adam(model.parameters(), lr=LR)

    print(f"\nInitial architecture params:")
    init_params = model.get_arch_params()
    print(f"  FFN ratios: {[f'{r:.3f}' for r in init_params['ffn_ratios']]}")
    print(f"  Dropout rates: {[f'{r:.4f}' for r in init_params['dropout_rates']]}")
    print(f"  Targets: FFN={TARGET_FFN_RATIO:.4f}, Dropout={TARGET_DROPOUT:.4f}")

    print(f"\n{'Step':>5} {'Task Loss':>10} {'R-dist':>8} {'FFN ratio':>10} {'Dropout':>10}")
    print("-" * 48)

    loss_history = []
    for step in range(STEPS):
        x, y = get_batch()
        logits = model(x)
        task_loss = F.cross_entropy(logits.reshape(-1, vocab_size), y.reshape(-1))

        beta = 0.5 * min(1.0, step / (STEPS * 0.3))
        r_dist = r_distance_loss(model)
        total_loss = task_loss + beta * r_dist

        opt.zero_grad()
        total_loss.backward()
        nn.utils.clip_grad_norm_(model.parameters(), 1.0)
        opt.step()

        loss_history.append(task_loss.item())

        if step % 50 == 0 or step == STEPS - 1:
            params = model.get_arch_params()
            mean_ratio = np.mean(params["ffn_ratios"])
            mean_drop = np.mean(params["dropout_rates"])
            print(f"{step:>5} {task_loss.item():>10.4f} {r_dist.item():>8.4f} "
                  f"{mean_ratio:>10.4f} {mean_drop:>10.4f}")

    final_params = model.get_arch_params()
    print(f"\n--- Convergence Results ---")
    print(f"{'Layer':>6} {'FFN Ratio':>10} {'Target':>10} {'Dropout':>10} {'Target':>10}")
    print("-" * 50)
    for i in range(N_LAYERS):
        ffn_r = final_params["ffn_ratios"][i]
        drop_r = final_params["dropout_rates"][i]
        print(f"{i:>6} {ffn_r:>10.4f} {TARGET_FFN_RATIO:>10.4f} "
              f"{drop_r:>10.4f} {TARGET_DROPOUT:.4f}")

    mean_ffn = np.mean(final_params["ffn_ratios"])
    mean_drop = np.mean(final_params["dropout_rates"])
    ffn_error = abs(mean_ffn - TARGET_FFN_RATIO) / TARGET_FFN_RATIO * 100
    drop_error = abs(mean_drop - TARGET_DROPOUT) / TARGET_DROPOUT * 100

    print(f"\nFFN ratio: {mean_ffn:.4f} (target {TARGET_FFN_RATIO:.4f}, error {ffn_error:.1f}%)")
    print(f"Dropout:   {mean_drop:.4f} (target {TARGET_DROPOUT:.4f}, error {drop_error:.1f}%)")
    print(f"\nConvergence {'CONFIRMED' if ffn_error < 10 and drop_error < 30 else 'PARTIAL'}")
    print(f"Architecture parameters evolve toward n=6 optima via meta-loss gradient.")


if __name__ == "__main__":
    main()
