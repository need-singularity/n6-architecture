#!/usr/bin/env python3
"""Common utilities — Components shared by 7 models

Mathematical constants (derived from perfect number 6):
  sigma(6) = 12  (sum of divisors)
  tau(6)   = 4   (number of divisors)
  phi(6)   = 2   (Euler's totient)
  sigma_{-1}(6) = 2  (sum of divisor reciprocals)
  {1/2, 1/3, 1/6} = divisor reciprocals (sum=1, probability distribution)
"""

import torch
import torch.nn as nn
import torch.nn.functional as F
from torch.utils.data import DataLoader
from torchvision import datasets, transforms
import numpy as np
import math
import time
from fractions import Fraction

# ─────────────────────────────────────────
# Mathematical constants
# ─────────────────────────────────────────
SIGMA = 12       # sigma(6)
TAU = 4          # tau(6)
PHI = 2          # phi(6)
SIGMA_INV = 2    # sigma_{-1}(6)
DIVISOR_RECIPROCALS = [1/2, 1/3, 1/6]  # sum=1, probability distribution
H_TARGET = sum(-p * math.log(p) for p in DIVISOR_RECIPROCALS)  # Shannon entropy

# ─── Extended n=6 arithmetic (techniques 11-16) ───
DEDEKIND_PSI = 12       # psi(6) = 6 * prod(1 + 1/p for p|6) = 6*(3/2)*(4/3) = 12
JORDAN_J2 = 24          # J_2(6) = 6^2 * prod(1 - 1/p^2 for p|6) = 36*(3/4)*(8/9) = 24
MOBIUS_MU = 1            # mu(6) = (-1)^2 = 1 (squarefree, 2 prime factors)
CARMICHAEL_LAMBDA = 2    # lambda(6) = lcm(lambda(2), lambda(3)) = lcm(1,2) = 2
GOLDEN_ZONE_CENTER = 1.0 / math.e   # 1/e ~ 0.3679
GOLDEN_ZONE_WIDTH = math.log(4/3)    # ln(4/3) ~ 0.2877
SOPFR = 5               # sum of prime factors with repetition: 2+3 = 5
RADICAL = 6              # rad(6) = 2*3 = 6
LEECH_DIM = SIGMA * PHI  # 12*2 = 24

# ─── Atlas Derived Constants (from σφ = nτ theorem) ───
# Ratios
FFN_RATIO = Fraction(TAU**2, SIGMA)          # 4/3 — FFN expansion ratio
MoE_TOP_K = PHI // 1                          # 2 — MoE expert selection
EGYPTIAN = (Fraction(1,2), Fraction(1,3), Fraction(1,6))  # sum = 1

# Powers of 2 from n=6 differences
BYTE = 2 ** (SIGMA - TAU)      # 2^8 = 256 (SHA-256, 8-bit)
AES_BITS = 2 ** (SIGMA - SOPFR) # 2^7 = 128 (AES-128, IPv6)
RSA_BITS = 2 ** (SIGMA - MOBIUS_MU)  # 2^11 = 2048 (RSA-2048)
CHACHA_ROUNDS = JORDAN_J2 - TAU # 24-4 = 20 (ChaCha20)

# Standard Model structure
SM_QUARKS = 6                    # n = 6
SM_LEPTONS = 6                   # n = 6
SM_GAUGE_BOSONS = TAU            # 4
SM_HIGGS = MOBIUS_MU             # 1
SM_TOTAL = SM_QUARKS + SM_LEPTONS + SM_GAUGE_BOSONS + SM_HIGGS  # 17
SM_GAUGE_GENERATORS = SIGMA      # SU(3)+SU(2)+U(1) = 8+3+1 = 12

# Thermodynamic
R_BALANCE = Fraction(SIGMA * PHI, 6 * TAU)  # = 1 (unique at n=6)
MERTENS_DROPOUT = math.log(4/3)              # ≈ 0.288
BOLTZMANN_SPARSITY = 1.0 / math.e            # ≈ 0.368

# Physical predictions
MP_ME_RATIO = 6 * math.pi ** 5               # ≈ 1836.118 (0.002% error)
HUBBLE_H0 = SIGMA * 6 + MOBIUS_MU            # = 73 (SH0ES: 73.04)
WEINBERG_ANGLE = Fraction(3, SIGMA + MOBIUS_MU)  # 3/13 ≈ 0.2308 (0.19% error)


# ─────────────────────────────────────────
# Expert
# ─────────────────────────────────────────
class Expert(nn.Module):
    def __init__(self, input_dim, hidden_dim, output_dim, dropout=0.5):
        super().__init__()
        self.net = nn.Sequential(
            nn.Linear(input_dim, hidden_dim),
            nn.ReLU(),
            nn.Dropout(dropout),
            nn.Linear(hidden_dim, output_dim),
        )

    def forward(self, x):
        return self.net(x)


# ─────────────────────────────────────────
# Basic gates
# ─────────────────────────────────────────
class TopKGate(nn.Module):
    def __init__(self, input_dim, n_experts, k=2):
        super().__init__()
        self.gate = nn.Linear(input_dim, n_experts)
        self.k = k

    def forward(self, x):
        scores = self.gate(x)
        topk_vals, topk_idx = scores.topk(self.k, dim=-1)
        mask = torch.zeros_like(scores)
        mask.scatter_(-1, topk_idx, 1.0)
        weights = F.softmax(scores, dim=-1) * mask
        return weights / (weights.sum(dim=-1, keepdim=True) + 1e-8)


class BoltzmannGate(nn.Module):
    def __init__(self, input_dim, n_experts, temperature=np.e, active_ratio=0.7):
        super().__init__()
        self.gate = nn.Linear(input_dim, n_experts)
        self.temperature = temperature
        self.n_active = max(1, int(n_experts * active_ratio))

    def forward(self, x):
        scores = self.gate(x) / self.temperature
        probs = F.softmax(scores, dim=-1)
        topk_vals, topk_idx = probs.topk(self.n_active, dim=-1)
        mask = torch.zeros_like(probs)
        mask.scatter_(-1, topk_idx, 1.0)
        weights = probs * mask
        return weights / (weights.sum(dim=-1, keepdim=True) + 1e-8)


# ─────────────────────────────────────────
# Basic MoE
# ─────────────────────────────────────────
class BaseMoE(nn.Module):
    def __init__(self, input_dim, hidden_dim, output_dim, n_experts, gate):
        super().__init__()
        self.experts = nn.ModuleList([
            Expert(input_dim, hidden_dim, output_dim)
            for _ in range(n_experts)
        ])
        self.gate = gate
        self.n_experts = n_experts
        self.expert_usage = torch.zeros(n_experts)
        self.active_counts = []

    def forward(self, x):
        weights = self.gate(x)
        outputs = torch.stack([e(x) for e in self.experts], dim=1)
        result = (weights.unsqueeze(-1) * outputs).sum(dim=1)

        with torch.no_grad():
            active = (weights > 0.01).float().sum(dim=-1).mean().item()
            self.active_counts.append(active)
            self.expert_usage += (weights > 0.01).float().sum(dim=0).mean(dim=0).cpu()

        return result

    def get_metrics(self):
        usage = self.expert_usage / max(self.expert_usage.sum().item(), 1)
        avg_active = np.mean(self.active_counts) if self.active_counts else 0
        return {
            'avg_active': avg_active,
            'active_ratio': avg_active / self.n_experts if self.n_experts > 0 else 0,
            'I_effective': 1 - avg_active / self.n_experts if self.n_experts > 0 else 0,
            'usage_std': usage.std().item(),
        }

    def reset_metrics(self):
        self.expert_usage = torch.zeros(self.n_experts)
        self.active_counts = []


# ─────────────────────────────────────────
# Dense model (baseline)
# ─────────────────────────────────────────
class DenseModel(nn.Module):
    def __init__(self, input_dim, hidden_dim, output_dim, dropout=0.5):
        super().__init__()
        self.net = nn.Sequential(
            nn.Linear(input_dim, hidden_dim),
            nn.ReLU(),
            nn.Dropout(dropout),
            nn.Linear(hidden_dim, output_dim),
        )

    def forward(self, x):
        return self.net(x)


# ─────────────────────────────────────────
# Data loader
# ─────────────────────────────────────────
def load_cifar10(batch_size=128, data_dir='data'):
    transform = transforms.Compose([
        transforms.ToTensor(),
        transforms.Normalize((0.5, 0.5, 0.5), (0.5, 0.5, 0.5))
    ])
    train_ds = datasets.CIFAR10(data_dir, train=True, download=True, transform=transform)
    test_ds = datasets.CIFAR10(data_dir, train=False, transform=transform)
    train_loader = DataLoader(train_ds, batch_size=batch_size, shuffle=True, num_workers=0)
    test_loader = DataLoader(test_ds, batch_size=batch_size, shuffle=False, num_workers=0)
    return train_loader, test_loader


def load_mnist(batch_size=128, data_dir='data'):
    transform = transforms.Compose([
        transforms.ToTensor(),
        transforms.Normalize((0.1307,), (0.3081,))
    ])
    train_ds = datasets.MNIST(data_dir, train=True, download=True, transform=transform)
    test_ds = datasets.MNIST(data_dir, train=False, transform=transform)
    train_loader = DataLoader(train_ds, batch_size=batch_size, shuffle=True, num_workers=0)
    test_loader = DataLoader(test_ds, batch_size=batch_size, shuffle=False, num_workers=0)
    return train_loader, test_loader


# ─────────────────────────────────────────
# Training + Evaluation
# ─────────────────────────────────────────
def train_and_evaluate(model, train_loader, test_loader, epochs=10, lr=0.001,
                       optimizer=None, aux_loss_fn=None, aux_lambda=0.1,
                       flatten=True, verbose=True):
    """Common training loop.

    Args:
        optimizer: If None, uses Adam. Can inject custom optimizer (for model C).
        aux_loss_fn: Auxiliary loss function (for model G). Extracted from model output.
        aux_lambda: Auxiliary loss weight.
        flatten: If True, flatten input to 1D (for MNIST).
    """
    if optimizer is None:
        optimizer = torch.optim.Adam(model.parameters(), lr=lr)
    criterion = nn.CrossEntropyLoss()

    train_losses = []
    test_accs = []

    for epoch in range(epochs):
        model.train()
        total_loss = 0
        for X, y in train_loader:
            if flatten:
                X = X.view(X.size(0), -1)
            optimizer.zero_grad()
            out = model(X)

            if isinstance(out, tuple):
                logits, aux = out
                loss = criterion(logits, y) + aux_lambda * aux
            else:
                logits = out
                loss = criterion(logits, y)

            loss.backward()
            optimizer.step()
            total_loss += loss.item()

        avg_loss = total_loss / len(train_loader)
        train_losses.append(avg_loss)

        model.eval()
        correct = total = 0
        with torch.no_grad():
            for X, y in test_loader:
                if flatten:
                    X = X.view(X.size(0), -1)
                out = model(X)
                if isinstance(out, tuple):
                    out = out[0]
                correct += (out.argmax(1) == y).sum().item()
                total += y.size(0)
        acc = correct / total
        test_accs.append(acc)

        if verbose and ((epoch + 1) % 2 == 0 or epoch == 0):
            print(f"    Epoch {epoch+1:>2}/{epochs}: Loss={avg_loss:.4f}, Acc={acc*100:.1f}%")

    return train_losses, test_accs


# ─────────────────────────────────────────
# Compare results
# ─────────────────────────────────────────
def compare_results(results):
    """results: dict of {name: {'acc': float, 'loss': float, 'params': int, ...}}"""
    print("\n" + "=" * 70)
    print(f"  {'Model':<30} {'Accuracy':>8} {'Loss':>8} {'Parameters':>10}")
    print("-" * 70)
    for name, r in sorted(results.items(), key=lambda x: -x[1].get('acc', 0)):
        acc = r.get('acc', 0)
        loss = r.get('loss', 0)
        params = r.get('params', 0)
        marker = ' <-- best' if acc == max(v.get('acc', 0) for v in results.values()) else ''
        print(f"  {name:<30} {acc*100:>7.2f}% {loss:>8.4f} {params:>10,}{marker}")
    print("=" * 70)


def count_params(model):
    return sum(p.numel() for p in model.parameters())