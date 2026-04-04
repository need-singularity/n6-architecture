#!/usr/bin/env python3
"""
Experiment: Mertens Dropout -- Technique #16
============================================
Tests p=ln(4/3)=0.2877 dropout rate vs standard rates {0.0, 0.1, 0.2, 0.3, 0.5}.
n=6 connection: ln(4/3) = ln(tau^2/sigma) from Mertens theorem.
Expected: Competitive with searched dropout rates on a char-level LM task.
"""
import sys, os, time, math
import numpy as np

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

import torch
import torch.nn as nn

from techniques.mertens_dropout import (
    MERTENS_DROPOUT, CharLM, Phi6Simple, count_params,
)

torch.manual_seed(42)
np.random.seed(42)

# ── Synthetic char-level data ──
VOCAB_SIZE = 30
SEQ_LEN = 32
D_MODEL = 48
N_HEADS = 6
N_LAYERS = 2
D_FF = 64
BATCH_SIZE = 32
N_BATCHES = 80
N_EVAL_BATCHES = 20


def make_data(n_batches, batch_size, seq_len, vocab_size):
    """Generate synthetic sequence data with simple patterns."""
    data = []
    for _ in range(n_batches):
        # Create sequences with repeating pattern + noise
        base = torch.randint(0, vocab_size, (batch_size, 1)).expand(-1, seq_len)
        noise = torch.randint(0, 3, (batch_size, seq_len))
        idx = (base + noise) % vocab_size
        # Target is next-token (shifted)
        x = idx[:, :-1]
        y = idx[:, 1:]
        data.append((x, y))
    return data


def train_and_eval(dropout_rate, train_data, eval_data, label=""):
    """Train a CharLM with given dropout and return eval loss + train loss."""
    model = CharLM(
        vocab_size=VOCAB_SIZE, d_model=D_MODEL, n_heads=N_HEADS,
        n_layers=N_LAYERS, d_ff=D_FF, seq_len=SEQ_LEN - 1,
        activation=Phi6Simple(), dropout=dropout_rate,
    )
    optimizer = torch.optim.Adam(model.parameters(), lr=1e-3)
    criterion = nn.CrossEntropyLoss()

    # Train
    model.train()
    train_losses = []
    for x, y in train_data:
        optimizer.zero_grad()
        logits = model(x)  # (B, T, V)
        loss = criterion(logits.reshape(-1, VOCAB_SIZE), y.reshape(-1))
        loss.backward()
        optimizer.step()
        train_losses.append(loss.item())

    # Eval
    model.eval()
    eval_losses = []
    with torch.no_grad():
        for x, y in eval_data:
            logits = model(x)
            loss = criterion(logits.reshape(-1, VOCAB_SIZE), y.reshape(-1))
            eval_losses.append(loss.item())

    return {
        'dropout': dropout_rate,
        'label': label,
        'train_loss': np.mean(train_losses[-20:]),
        'eval_loss': np.mean(eval_losses),
        'params': count_params(model),
    }


def main():
    print("=" * 60)
    print("  Experiment: Mertens Dropout Sweep (Technique #16)")
    print(f"  Mertens rate: p = ln(4/3) = {MERTENS_DROPOUT:.6f}")
    print("=" * 60)

    print(f"\n  Config: vocab={VOCAB_SIZE}, d_model={D_MODEL}, heads={N_HEADS},")
    print(f"          layers={N_LAYERS}, d_ff={D_FF}, seq_len={SEQ_LEN}")
    print(f"          batches={N_BATCHES} train, {N_EVAL_BATCHES} eval, bs={BATCH_SIZE}")

    train_data = make_data(N_BATCHES, BATCH_SIZE, SEQ_LEN, VOCAB_SIZE)
    eval_data = make_data(N_EVAL_BATCHES, BATCH_SIZE, SEQ_LEN, VOCAB_SIZE)

    # Dropout rates to compare
    rates = [
        (0.0, "no dropout"),
        (0.1, "standard 0.1"),
        (0.2, "standard 0.2"),
        (MERTENS_DROPOUT, "Mertens ln(4/3)"),
        (0.3, "standard 0.3"),
        (0.5, "standard 0.5"),
    ]

    print(f"\n  {'Rate':>8} | {'Label':>16} | {'Train Loss':>11} | {'Eval Loss':>10} | {'Gap':>8}")
    print("  " + "-" * 65)

    results = []
    for rate, label in rates:
        r = train_and_eval(rate, train_data, eval_data, label)
        gap = r['eval_loss'] - r['train_loss']
        results.append(r)
        print(f"  {rate:>8.4f} | {label:>16} | {r['train_loss']:>11.4f} | {r['eval_loss']:>10.4f} | {gap:>8.4f}")

    # Find best eval loss
    best = min(results, key=lambda r: r['eval_loss'])
    mertens_result = [r for r in results if abs(r['dropout'] - MERTENS_DROPOUT) < 1e-6][0]

    print(f"\n=== Results ===")
    print(f"  Best eval loss:    {best['label']} (rate={best['dropout']:.4f}, loss={best['eval_loss']:.4f})")
    print(f"  Mertens eval loss: {mertens_result['eval_loss']:.4f}")
    print(f"  Mertens rank:      {sorted(results, key=lambda r: r['eval_loss']).index(mertens_result) + 1}/{len(results)}")

    # Generalization gap analysis
    print(f"\n=== Generalization Gap (eval - train) ===")
    for r in results:
        gap = r['eval_loss'] - r['train_loss']
        bar = "+" * max(0, int(gap * 50))
        print(f"  {r['label']:>16}: gap={gap:.4f} {bar}")

    print(f"\n  n=6 connection: ln(4/3) = ln(tau^2/sigma) = ln(16/12) = {MERTENS_DROPOUT:.6f}")
    print(f"  4/3 = SwiGLU ratio = tau(6)^2/sigma(6)")
    print()


if __name__ == '__main__':
    main()
