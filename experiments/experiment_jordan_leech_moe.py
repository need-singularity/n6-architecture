#!/usr/bin/env python3
"""
Experiment: Jordan-Leech MoE Scaling -- Technique #12
=====================================================
Tests J2(6)=24 expert capacity bound.
n=6 connection: J2(6) = 24, Leech lattice dimension = 24.
Compares MoE performance across expert counts: 4, 8, 12, 16, 24, 32, 48.
Expected: 24 experts is optimal MoE capacity (diminishing returns beyond).
"""
import sys, os, time, math
import numpy as np

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

import torch
import torch.nn as nn
import torch.nn.functional as F

from techniques.jordan_leech_moe import (
    JordanLeechMoE, Expert, JORDAN_J2, SIGMA, TAU, PHI, EGYPTIAN,
)

torch.manual_seed(42)
np.random.seed(42)

D_MODEL = 64
D_FF = 32
TOP_K = 3
SEQ_LEN = 16
BATCH_SIZE = 16
N_BATCHES = 60
N_EVAL_BATCHES = 20
N_CLASSES = 10


class SimpleMoEClassifier(nn.Module):
    """Classifier using JordanLeechMoE as the core layer."""
    def __init__(self, d_model, d_ff, n_experts, top_k, n_classes):
        super().__init__()
        self.embed = nn.Linear(d_model, d_model)
        self.moe = JordanLeechMoE(d_model, d_ff, n_experts=n_experts, top_k=top_k)
        self.head = nn.Linear(d_model, n_classes)

    def forward(self, x):
        # x: (B, L, D)
        x = F.relu(self.embed(x))
        x = self.moe(x)
        x = x.mean(dim=1)  # pool over sequence
        return self.head(x)


def make_synthetic_data(n_batches, batch_size, seq_len, d_model, n_classes):
    """Generate synthetic classification data."""
    data = []
    for _ in range(n_batches):
        x = torch.randn(batch_size, seq_len, d_model)
        # Create labels based on mean of input features (simple pattern)
        y = (x.mean(dim=(1, 2)) * n_classes / 2).long().clamp(0, n_classes - 1)
        data.append((x, y))
    return data


def train_and_eval(n_experts, train_data, eval_data):
    """Train a MoE model with given expert count and return metrics."""
    model = SimpleMoEClassifier(D_MODEL, D_FF, n_experts, TOP_K, N_CLASSES)
    optimizer = torch.optim.Adam(model.parameters(), lr=1e-3)
    criterion = nn.CrossEntropyLoss()
    n_params = sum(p.numel() for p in model.parameters())

    t0 = time.perf_counter()

    # Train
    model.train()
    train_losses = []
    for x, y in train_data:
        optimizer.zero_grad()
        logits = model(x)
        loss = criterion(logits, y)
        loss.backward()
        optimizer.step()
        train_losses.append(loss.item())

    train_time = time.perf_counter() - t0

    # Eval
    model.eval()
    eval_losses = []
    correct = 0
    total = 0
    with torch.no_grad():
        for x, y in eval_data:
            logits = model(x)
            loss = criterion(logits, y)
            eval_losses.append(loss.item())
            preds = logits.argmax(dim=-1)
            correct += (preds == y).sum().item()
            total += y.numel()

    # Get expert usage metrics
    model.moe.reset_metrics()
    with torch.no_grad():
        for x, y in eval_data[:5]:
            _ = model(x)
    metrics = model.moe.get_metrics()

    return {
        'n_experts': n_experts,
        'params': n_params,
        'train_loss': np.mean(train_losses[-10:]),
        'eval_loss': np.mean(eval_losses),
        'accuracy': correct / total if total > 0 else 0.0,
        'train_time': train_time,
        'usage_entropy': metrics.get('usage_entropy', 0.0),
        'max_usage': metrics.get('max_usage', 0.0),
    }


def main():
    print("=" * 70)
    print("  Experiment: Jordan-Leech MoE Scaling (Technique #12)")
    print(f"  J_2(6) = {JORDAN_J2}, Leech lattice dim = 24")
    print(f"  Egyptian routing: {EGYPTIAN}")
    print("=" * 70)

    print(f"\n  Config: d_model={D_MODEL}, d_ff={D_FF}, top_k={TOP_K}")
    print(f"          seq_len={SEQ_LEN}, batch_size={BATCH_SIZE}")
    print(f"          train_batches={N_BATCHES}, eval_batches={N_EVAL_BATCHES}")

    train_data = make_synthetic_data(N_BATCHES, BATCH_SIZE, SEQ_LEN, D_MODEL, N_CLASSES)
    eval_data = make_synthetic_data(N_EVAL_BATCHES, BATCH_SIZE, SEQ_LEN, D_MODEL, N_CLASSES)

    expert_counts = [4, 8, 12, 16, 24, 32, 48]

    print(f"\n  {'Experts':>7} | {'Params':>8} | {'TrainLoss':>10} | {'EvalLoss':>9} | {'Acc':>6} | {'Time(s)':>8} | {'Entropy':>8}")
    print("  " + "-" * 75)

    results = []
    for n_exp in expert_counts:
        r = train_and_eval(n_exp, train_data, eval_data)
        results.append(r)
        print(f"  {r['n_experts']:>7} | {r['params']:>8} | {r['train_loss']:>10.4f} | {r['eval_loss']:>9.4f} | "
              f"{r['accuracy']:>5.1%} | {r['train_time']:>8.2f} | {r['usage_entropy']:>8.3f}")

    # Analysis
    print(f"\n=== Scaling Analysis ===")

    # Find best eval loss
    best = min(results, key=lambda r: r['eval_loss'])
    j2_result = [r for r in results if r['n_experts'] == 24][0]

    print(f"  Best eval loss: {best['n_experts']} experts (loss={best['eval_loss']:.4f})")
    print(f"  J_2=24 eval loss: {j2_result['eval_loss']:.4f}")

    # Efficiency: eval_loss / params
    print(f"\n  {'Experts':>7} | {'Loss/1kParam':>13} | {'Note':>20}")
    print("  " + "-" * 50)
    for r in results:
        efficiency = r['eval_loss'] / (r['params'] / 1000)
        note = "<-- J_2(6)=24" if r['n_experts'] == 24 else ""
        print(f"  {r['n_experts']:>7} | {efficiency:>13.6f} | {note:>20}")

    # Diminishing returns analysis
    print(f"\n=== Diminishing Returns ===")
    for i in range(1, len(results)):
        prev = results[i - 1]
        curr = results[i]
        loss_delta = prev['eval_loss'] - curr['eval_loss']
        param_delta = curr['params'] - prev['params']
        efficiency = loss_delta / param_delta * 1000 if param_delta > 0 else 0
        marker = " <-- J_2 boundary" if curr['n_experts'] == 24 else ""
        if curr['n_experts'] > 24 and prev['n_experts'] <= 24:
            marker = " <-- beyond J_2"
        print(f"  {prev['n_experts']:>2}->{curr['n_experts']:>2}: "
              f"loss_delta={loss_delta:+.4f}, param_delta=+{param_delta}, "
              f"eff={efficiency:.4f}{marker}")

    print(f"\n  n=6: J_2(6) = 6^2 * prod(1-1/p^2) = 36*(3/4)*(8/9) = 24")
    print(f"  Leech lattice: 24-dim, kissing number = 196560")
    print()


if __name__ == '__main__':
    main()
