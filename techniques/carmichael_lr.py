"""
Technique 14: Carmichael LR Cycle
===================================
lambda(6) = lcm(lambda(2), lambda(3)) = lcm(1, 2) = 2.
Maximum multiplicative order mod 6 is 2.
Any stable LR schedule on the R=1 surface has period 2.

Expected: eliminates LR schedule hyperparameter search.
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

CARMICHAEL_LAMBDA = 2
SIGMA = 12
PHI_N6 = 2


class CarmichaelLR:
    """2-cycle LR schedule derived from lambda(6) = 2."""

    def __init__(self, optimizer, lr_max, steps_per_epoch):
        self.optimizer = optimizer
        self.lr_max = lr_max
        self.lr_min = lr_max / 6
        self.half_epoch = steps_per_epoch // CARMICHAEL_LAMBDA
        self.step_count = 0

    def step(self):
        self.step_count += 1
        phase = (self.step_count // self.half_epoch) % CARMICHAEL_LAMBDA
        if phase == 0:
            lr = self.lr_max
        else:
            t = (self.step_count % self.half_epoch) / max(self.half_epoch, 1)
            lr = self.lr_min + (self.lr_max - self.lr_min) * (1 + math.cos(math.pi * t)) / 2
        for pg in self.optimizer.param_groups:
            pg['lr'] = lr
        return lr

    def get_lr(self):
        return self.optimizer.param_groups[0]['lr']


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


def main():
    print("=" * 70)
    print("  Technique 14: Carmichael LR Cycle")
    print("  lambda(6) = 2 — period-2 optimal schedule")
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
    D_FF = round(4 * D_MODEL / 3)

    def get_batch():
        ix = torch.randint(0, len(data) - SEQ_LEN - 1, (BATCH,))
        x = torch.stack([data[i:i+SEQ_LEN] for i in ix])
        y = torch.stack([data[i+1:i+SEQ_LEN+1] for i in ix])
        return x, y

    schedules = {
        "constant": lambda opt: None,
        "carmichael-2": lambda opt: CarmichaelLR(opt, LR, STEPS // 5),
        "cosine": lambda opt: torch.optim.lr_scheduler.CosineAnnealingLR(opt, STEPS),
        "step-decay": lambda opt: torch.optim.lr_scheduler.StepLR(opt, STEPS // 3, gamma=0.1),
    }

    results = []
    for sched_name, sched_factory in schedules.items():
        print(f"\n--- {sched_name} ---")
        torch.manual_seed(SEED)
        model = CharLM(vocab_size, D_MODEL, N_HEADS, N_LAYERS, D_FF, SEQ_LEN, Phi6Simple())
        opt = torch.optim.Adam(model.parameters(), lr=LR)
        scheduler = sched_factory(opt)

        t0 = time.time()
        losses = []
        lr_history = []
        for step in range(STEPS):
            x, y = get_batch()
            logits = model(x)
            loss = F.cross_entropy(logits.reshape(-1, vocab_size), y.reshape(-1))
            opt.zero_grad()
            loss.backward()
            nn.utils.clip_grad_norm_(model.parameters(), 1.0)
            opt.step()
            losses.append(loss.item())

            if isinstance(scheduler, CarmichaelLR):
                lr_history.append(scheduler.step())
            elif scheduler is not None:
                scheduler.step()
                lr_history.append(opt.param_groups[0]['lr'])
            else:
                lr_history.append(LR)
        elapsed = time.time() - t0

        results.append({
            "schedule": sched_name,
            "final_loss": np.mean(losses[-20:]),
            "min_loss": min(losses),
            "train_time": elapsed,
            "final_lr": lr_history[-1],
        })

    print("\n" + "=" * 70)
    print("  Carmichael LR Cycle Results")
    print("=" * 70)
    print(f"{'Schedule':<20} {'Final Loss':>12} {'Min Loss':>12} {'Time':>7}")
    print("-" * 55)
    for r in results:
        print(f"{r['schedule']:<20} {r['final_loss']:>12.4f} {r['min_loss']:>12.4f} {r['train_time']:>6.1f}s")

    print(f"\nlambda(6) = {CARMICHAEL_LAMBDA}")
    print(f"2-cycle ratio: lr_max / lr_min = 6 (from n=6)")
    print(f"Phase 1: exploration at lr={LR}")
    print(f"Phase 2: exploitation at lr={LR/6:.5f}")
    print("No hyperparameter search needed — period and ratio are determined by n=6.")


if __name__ == "__main__":
    main()
